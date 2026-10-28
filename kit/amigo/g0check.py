#!/usr/bin/env python3
"""g0check.py - G0 'Code recovered' check (tightened 13 Oct 2026).

The first G0 pass (3D Pool, Gemma) met the old check by copying one file to
$C00000 with dd and writing a three-line MEMMAP. That is not a recovered memory
map, so the check now requires, in gates/G0/MEMMAP.md:
  BASE: $xxxxxx        address of byte 0 of out/image.bin
  ENTRY: $xxxxxx       entry point
  REGION: $start-$end kind  description   (kind: code|data|packed|unpacked|bss), at least 2 lines
and verifies: the image rebuilds, ENTRY lies in a code region inside the image,
and the 68000 code at ENTRY disassembles cleanly for at least 24 instructions.
If the disk holds AmigaDOS hunk executables (magic $000003F3), the image must
not contain a raw hunk header at ENTRY (hunks must be relocated, not copied)."""
import re, subprocess, sys
r = subprocess.run(["bash", "gates/G0/rebuild_image.sh"], capture_output=True, text=True)
if r.returncode != 0:
    print("rebuild_image.sh failed rc=%d %s" % (r.returncode, r.stderr[-300:])); sys.exit(1)
img = open("out/image.bin", "rb").read()
mm = open("gates/G0/MEMMAP.md").read()
def addr(tag):
    m = re.search(r"^%s:\s*\$([0-9A-Fa-f]+)" % tag, mm, re.M)
    return int(m.group(1), 16) if m else None
base, entry = addr("BASE"), addr("ENTRY")
regions = [(int(a, 16), int(b, 16), k.lower()) for a, b, k in
           re.findall(r"^REGION:\s*\$([0-9A-Fa-f]+)\s*-\s*\$([0-9A-Fa-f]+)\s+(\w+)", mm, re.M)]
errs = []
if base is None: errs.append("no BASE: line")
if entry is None: errs.append("no ENTRY: line")
if len(regions) < 2: errs.append("fewer than 2 REGION: lines (%d)" % len(regions))
# the contract asks for prose on where the loader puts each part (13 Oct: hunkload.py now prints the BASE/ENTRY/
# REGION lines, so the lines alone no longer show that anyone looked at what the loaded code does)
prose = re.sub(r"^(BASE|ENTRY|REGION):.*$", "", mm, flags=re.M)
if len(re.findall(r"[A-Za-z]{2,}", prose)) < 60: errs.append("MEMMAP.md needs at least 60 words of prose on how the loader lays the game out")
if not errs:
    if not any(a <= entry < b and k == "code" for a, b, k in regions): errs.append("ENTRY is not inside a code REGION")
    off = entry - base
    if not 0 <= off < len(img) - 64: errs.append("ENTRY $%X is outside the image (base $%X, %d bytes)" % (entry, base, len(img)))
    else:
        if img[off:off + 4] == b"\x00\x00\x03\xf3": errs.append("raw hunk header at ENTRY: the executable was copied, not loaded and relocated")
        import capstone
        md = capstone.Cs(capstone.CS_ARCH_M68K, capstone.CS_MODE_BIG_ENDIAN | capstone.CS_MODE_M68K_000)
        n = sum(1 for _ in md.disasm(img[off:off + 400], entry))
        import json, os
        whd = (json.load(open("gates/G0/g0.json")).get("whdload") if os.path.exists("gates/G0/g0.json") else None)
        if whd:
            # a WHDLoad slave (14 Oct 2026): its game loader is often a few instructions that call resload, so the
            # proof is structural plus the measured run: WHDLOADS id, ENTRY = ws_GameLoader, and the slave ran on
            # the kit's C resload showing changing pictures
            gl = int.from_bytes(img[0x18:0x1A], "big")   # ws_GameLoader
            if img[4:12] != b"WHDLOADS": errs.append("out/image.bin is not a WHDLoad slave (no WHDLOADS id at +4)")
            elif entry != base + gl: errs.append("ENTRY $%X is not the slave's ws_GameLoader $%X" % (entry, base + gl))
            # many slaves keep data inline right after a short BRA at ws_GameLoader (resload pointer, tag list), so
            # a linear sweep stops at the data: follow unconditional BRAs within the image (14 Oct 2026, Altered Beast)
            pc, n, seen = entry, 0, set()
            while n < 64 and pc not in seen and 0 <= pc - base < len(img):
                seen.add(pc)
                ins = list(md.disasm(img[pc - base:pc - base + 400], pc))
                if not ins: break
                for i in ins:
                    n += 1
                    if i.mnemonic.startswith("bra"):
                        pc = int(i.op_str.lstrip("$"), 16); break
                else: break
            if n < 6: errs.append("only %d valid 68000 instructions at ENTRY (need 6 for a slave loader)" % n)
            hp = "out/port/whd/hash.txt"
            pics = len({l.split()[1] for l in open(hp) if len(l.split()) > 1}) if os.path.exists(hp) else 0
            if pics < 2: errs.append("the slave's harness run (amigo_port.sh whdrun) showed %d different pictures (need 2)" % pics)
        elif n < 24: errs.append("only %d valid 68000 instructions at ENTRY (need 24)" % n)
if errs:
    print("G0 FAIL: " + "; ".join(errs)); sys.exit(1)
print("G0 OK: base $%X entry $%X, %d regions, image %d bytes" % (base, entry, len(regions), len(img)))
