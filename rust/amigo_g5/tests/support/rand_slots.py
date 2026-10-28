#!/usr/bin/env python3
"""Synthetic image for the M3 randomised differential test (rust/amigo_g5/tests/rand_diff.rs). 28 Oct 2026.

Every opcode m68k2c.py's decoder accepts is put in a class by the C it decodes to, with register numbers and
constants blanked out, so two opcodes share a class when they run the same statements on different registers or
immediates (for example every `add.w Dn,Dm`). PER_CLASS opcodes are drawn from each class, each with random
extension words, and laid out one per 16-byte slot, followed by `bra.s *`. Nothing here comes from a game: the image
is generated, the noise is a fixed xorshift stream, and the opcodes are the decoder's own.

usage: rand_slots.py OUTDIR [SEED] [PER_CLASS]
writes OUTDIR/image.bin (loaded at address 0) and OUTDIR/slots.txt (one line per slot: pc opcode length class)
"""
import os
import random
import re
import sys

PORT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '../../../../kit/amigo/port')
sys.path.insert(0, PORT)
import m68k2c  # noqa: E402

NOISE = (0x10000, 0x30000)   # address registers and the other stack pointer point into the middle of this
CODE = 0x40000
SLOT = 16
STOP = 0x60FE                # bra.s *


class Words:
    """The decoder's memory interface over one opcode and its extension words at CODE."""
    snap = None

    def __init__(self, op, ext):
        self.op, self.ext = op, ext

    def w(self, a):
        if a == CODE:
            return self.op
        i = (a - CODE - 2) // 2
        return self.ext[i] if 0 <= i < len(self.ext) else STOP

    def l(self, a):
        return (self.w(a) << 16) | self.w(a + 2)


def decode(op, ext):
    try:
        return m68k2c.Decoder(Words(op, ext)).decode(CODE)
    except m68k2c.Undecodable:
        return None


def key(ins):
    return re.sub(r'\b(0x[0-9A-Fa-f]+|\d+)u?\b', '#', ' '.join(ins.code)) + '|' + ins.kind


def classes():
    out = {}
    for op in range(0x10000):
        ins = decode(op, [4, 4, 4, 4])
        if ins is not None:
            out.setdefault(key(ins), []).append(op)
    return out


def main():
    outdir = sys.argv[1]
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    per = int(sys.argv[3]) if len(sys.argv) > 3 else 2
    rng = random.Random(seed)
    cls = classes()
    slots = []
    for ci, k in enumerate(sorted(cls)):
        ops = cls[k]
        for n, op in enumerate(rng.sample(ops, min(per, len(ops)))):
            for _ in range(64):
                ext = [rng.getrandbits(16) for _ in range(4)]
                # the first opcode of a class gets even extension words (displacements and absolute addresses that
                # do not force an address error on every trial); later ones are left odd half the time
                if n == 0 or rng.random() < 0.5:
                    ext = [e & ~1 for e in ext]
                ins = decode(op, ext)
                if ins is not None:
                    break
            else:
                sys.exit('no extension words decode for %04X' % op)
            slots.append((op, ext[:(ins.length - 2) // 2], ins.length, ci))
    img = bytearray(CODE + SLOT * len(slots))
    x = 0x2545F491
    for lo, hi in ((0, 0x400), NOISE):
        for a in range(lo, hi):
            x ^= (x << 13) & 0xFFFFFFFF
            x ^= x >> 17
            x ^= (x << 5) & 0xFFFFFFFF
            img[a] = (x >> 8) & 0xFF
    lines = []
    for n, (op, ext, length, ci) in enumerate(slots):
        pc = CODE + SLOT * n
        words = [op] + ext
        words += [STOP] * (SLOT // 2 - len(words))
        for i, w in enumerate(words):
            img[pc + 2 * i:pc + 2 * i + 2] = w.to_bytes(2, 'big')
        lines.append('%X %04X %d %d' % (pc, op, length, ci))
    os.makedirs(outdir, exist_ok=True)
    open(os.path.join(outdir, 'image.bin'), 'wb').write(img)
    open(os.path.join(outdir, 'slots.txt'), 'w').write('\n'.join(lines) + '\n')
    print('classes %d slots %d' % (len(cls), len(slots)))


if __name__ == '__main__':
    main()
