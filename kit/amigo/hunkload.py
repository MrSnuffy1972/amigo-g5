#!/usr/bin/env python3
"""hunkload.py: load an AmigaDOS hunk executable the way LoadSeg() does, and write a flat memory image.

Written 13 Oct 2026 for Ami-Go after Gemma failed gate G0 nine times on 3D Pool: every session found that the
file was a hunk executable that had to be loaded and relocated, and every session ran out of time writing a
hunk parser (one was killed, exit 137). Loading a hunk file is a solved, deterministic step, so it is a tool
(rung 3), not a model task. Models use it and spend their time on what the game does after it is loaded.

Usage:
  python3 hunkload.py FILE --out /w/out/image.bin [--base 0xC00000] [--chip-base 0x20000] [--align 8] [--memmap F]

What it does: reads HUNK_HEADER, lays the hunks out one after another from --base (each aligned), copies
HUNK_CODE / HUNK_DATA contents, zero-fills HUNK_BSS and the unused tail of each hunk, applies HUNK_RELOC32,
HUNK_RELOC32SHORT and HUNK_RELOC16/8 relative relocations, and skips HUNK_SYMBOL, HUNK_DEBUG, HUNK_NAME and
HUNK_EXT definitions. Entry point = start of the first hunk (as LoadSeg + CreateProc). Prints BASE, ENTRY and one
REGION line per hunk in the MEMMAP.md format, so they can be pasted and then refined.
Limits (says so and exits 2): overlay files (HUNK_OVERLAY/BREAK), and external references that need a linker.
It is NOT a decruncher: a packed executable loads fine but its code region holds the packed data and its
depacker; say so in MEMMAP.md (kind packed) and unpack it as a later step.
"""
import argparse, struct, sys

HUNK = {0x3E7: "UNIT", 0x3E8: "NAME", 0x3E9: "CODE", 0x3EA: "DATA", 0x3EB: "BSS", 0x3EC: "RELOC32",
        0x3ED: "RELOC16", 0x3EE: "RELOC8", 0x3EF: "EXT", 0x3F0: "SYMBOL", 0x3F1: "DEBUG", 0x3F2: "END",
        0x3F3: "HEADER", 0x3F5: "OVERLAY", 0x3F6: "BREAK", 0x3F7: "DREL32", 0x3F8: "DREL16", 0x3F9: "DREL8",
        0x3FC: "RELOC32SHORT", 0x3FD: "RELRELOC32"}


class R:
    def __init__(self, b):
        self.b, self.p = b, 0

    def l(self):
        if self.p + 4 > len(self.b):
            raise EOFError("unexpected end of file at offset %d" % self.p)
        v = struct.unpack_from(">I", self.b, self.p)[0]
        self.p += 4
        return v

    def w(self):
        v = struct.unpack_from(">H", self.b, self.p)[0]
        self.p += 2
        return v

    def skip_longs(self, n):
        if self.p + 4 * n > len(self.b):
            raise EOFError("hunk runs past end of file")
        self.p += 4 * n


def load(data, base=None, align=8, chip_base=0x20000, seglist=False):
    """seglist=True lays the hunks out as AmigaDOS LoadSeg really does: 8 bytes in front of every hunk hold its
    allocation size and a BPTR to the next hunk's header (k17, 14 Oct 2026: a self-decrunching stub in 4th & Inches
    finds its packed data with lea -10(pc),a0 / move.l (a0),a0; without the header it read CIA space and unpacked
    garbage). Off by default so images that already pass keep their addresses."""
    r = R(data)
    if r.l() != 0x3F3:
        raise ValueError("not a hunk executable (no HUNK_HEADER $3F3 at offset 0)")
    while True:                                   # resident library names (normally none)
        n = r.l()
        if n == 0:
            break
        r.skip_longs(n)
    table, first, last = r.l(), r.l(), r.l()
    count = last - first + 1
    if count <= 0 or count > 4096:
        raise ValueError("implausible hunk count %d" % count)
    sizes, mem = [], []
    for _ in range(count):
        v = r.l()
        flags = v >> 30
        if flags == 3:
            r.l()                                  # extended memory attributes
        sizes.append((v & 0x3FFFFFFF) * 4)
        mem.append({0: "any", 1: "chip", 2: "fast"}.get(flags, "ext"))
    # hunks flagged CHIP must sit in chip RAM (below $80000 on an A500): they go from chip_base, the rest from
    # base. Measured 13 Oct on 3D Pool: its code hunk is flagged chip. The image is written from the lowest address,
    # so with both kinds present it spans the gap (zero-filled) and BASE is the lower start.
    if base is None:
        # auto: a file with any CHIP hunk is laid out entirely in chip RAM (what LoadSeg does on a chip-only A500,
        # and it keeps the image compact: measured 13 Oct, 3D Pool's 4-byte BSS at $C00000 made a 12 MB image)
        base = chip_base if "chip" in mem else 0xC00000
    addr, nxt = [], {"chip": chip_base, "other": base}
    hdr = 8 if seglist else 0
    for s_, m_ in zip(sizes, mem):
        k = "chip" if (m_ == "chip" or base == chip_base) else "other"
        a = (nxt[k] + align - 1) // align * align + hdr
        addr.append(a)
        nxt[k] = a + s_
    lo = min(addr) - hdr
    hi = max(a + s_ for a, s_ in zip(addr, sizes))
    if hi - lo > 16 << 20:
        raise ValueError("image would span %d MB; use --base close to --chip-base" % ((hi - lo) >> 20))
    base = lo
    image = bytearray(hi - lo)
    kinds = ["?"] * count
    relocs = 0
    h = -1
    while r.p < len(data):
        if len(data) - r.p < 4:
            break
        t = r.l() & 0x3FFFFFFF
        name = HUNK.get(t)
        if name in ("CODE", "DATA", "BSS"):
            h += 1
            if h >= count:
                raise ValueError("more hunks in the file than the header declares")
            n = r.l() & 0x3FFFFFFF
            kinds[h] = name.lower()
            if name != "BSS":
                body = data[r.p:r.p + 4 * n]
                r.skip_longs(n)
                o = addr[h] - base
                image[o:o + len(body)] = body
        elif name == "RELOC32":
            while True:
                n = r.l()
                if n == 0:
                    break
                target = r.l()
                for _ in range(n):
                    off = r.l()
                    o = addr[h] - base + off
                    v = struct.unpack_from(">I", image, o)[0]
                    struct.pack_into(">I", image, o, (v + addr[target]) & 0xFFFFFFFF)
                    relocs += 1
        elif name in ("RELOC32SHORT", "RELRELOC32"):
            start = r.p
            while True:
                n = r.w()
                if n == 0:
                    break
                target = r.w()
                for _ in range(n):
                    off = r.w()
                    o = addr[h] - base + off
                    v = struct.unpack_from(">I", image, o)[0]
                    if name == "RELOC32SHORT":
                        v = (v + addr[target]) & 0xFFFFFFFF
                    else:                          # PC-relative between hunks
                        v = (v + addr[target] - (addr[h] + off)) & 0xFFFFFFFF
                    struct.pack_into(">I", image, o, v)
                    relocs += 1
            if (r.p - start) % 4:
                r.p += 2                           # padded to a long
        elif name in ("RELOC16", "RELOC8"):
            size = 2 if name == "RELOC16" else 1
            while True:
                n = r.l()
                if n == 0:
                    break
                target = r.l()
                for _ in range(n):
                    off = r.l()
                    o = addr[h] - base + off
                    fmt = ">h" if size == 2 else ">b"
                    v = struct.unpack_from(fmt, image, o)[0] + addr[target] - (addr[h] + off)
                    struct.pack_into(">H" if size == 2 else ">B", image, o, v & (0xFFFF if size == 2 else 0xFF))
                    relocs += 1
        elif name in ("SYMBOL",):
            while True:
                n = r.l()
                if n == 0:
                    break
                r.skip_longs(n + 1)
        elif name in ("DEBUG", "NAME", "UNIT"):
            r.skip_longs(r.l())
        elif name == "EXT":
            while True:
                v = r.l()
                if v == 0:
                    break
                typ, n = v >> 24, v & 0xFFFFFF
                r.skip_longs(n)
                if typ in (1, 2, 3):              # definitions: one value
                    r.l()
                elif typ in (130,):               # common: size then refs
                    r.l(); r.skip_longs(r.l())
                else:
                    raise NotImplementedError("HUNK_EXT reference type %d needs a linker" % typ)
        elif name == "END":
            continue
        elif name in ("OVERLAY", "BREAK"):
            raise NotImplementedError("overlay executables are not supported")
        elif t == 0 and not any(data[r.p:]):
            break                                 # zero padding at end of file
        else:
            raise ValueError("unknown hunk type $%X at file offset %d" % (t, r.p - 4))
    if seglist:   # the LoadSeg headers: allocation size, then the BPTR to the next segment (0 for the last). A segment
        # BPTR points at the segment's own "next" longword (header + 4), so its data starts at BPTR * 4 + 4
        for i, a in enumerate(addr):
            nxt_hdr = ((addr[i + 1] - 4) >> 2) if i + 1 < len(addr) else 0
            struct.pack_into(">II", image, a - 8 - base, sizes[i] + 8, nxt_hdr)
    return {"image": bytes(image), "base": base, "entry": addr[0], "addr": addr, "sizes": sizes,
            "kinds": kinds, "mem": mem, "relocs": relocs, "seglist": ((addr[0] - 4) >> 2) if seglist else None}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("file")
    ap.add_argument("--out", required=True)
    ap.add_argument("--base", default="auto", help="where non-CHIP hunks go; auto = chip RAM if any hunk is CHIP, else $C00000")
    ap.add_argument("--align", type=int, default=8)
    ap.add_argument("--chip-base", default="0x20000", help="where CHIP hunks go (default $20000)")
    ap.add_argument("--memmap")
    a = ap.parse_args()
    try:
        res = load(open(a.file, "rb").read(), None if a.base == "auto" else int(a.base, 0), a.align, int(a.chip_base, 0))
    except NotImplementedError as ex:
        print("UNSUPPORTED:", ex); sys.exit(2)
    except (ValueError, EOFError) as ex:
        print("ERROR:", ex); sys.exit(1)
    open(a.out, "wb").write(res["image"])
    lines = ["BASE: $%06X" % res["base"], "ENTRY: $%06X" % res["entry"]]
    for i, (ad, s, k, m) in enumerate(zip(res["addr"], res["sizes"], res["kinds"], res["mem"])):
        lines.append("REGION: $%06X-$%06X %s hunk %d (%s memory, %d bytes)" % (ad, ad + s, k, i, m, s))
    txt = "\n".join(lines)
    print(txt)
    print("# %d hunks, %d relocations applied, image %d bytes written to %s" % (
        len(res["addr"]), res["relocs"], len(res["image"]), a.out))
    if a.memmap:
        open(a.memmap, "w").write(txt + "\n")


if __name__ == "__main__":
    main()
