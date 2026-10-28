#!/usr/bin/env python3
"""g1sane.py: part of the G1 check (Ami-Go, 13 Oct 2026). The port kit seeds translation from the harness run of
the original code (out/port/ref/executed_ops.txt). Measured on 4th & Inches: the original ran off into empty memory
(every 2-byte address from $000000 to $07FFFF executed, opcode $0000 = ORI.B #0,D0, 262,237 PCs), so 'translated'
became 262,000 blocks of zeros and the coverage count passed while nothing real was translated. A run like that
proves nothing, so G1 fails with the reason instead of passing on it. No harness log (a model's own build): passes."""
import os, sys
f = "out/port/ref/executed_ops.txt"
if not os.path.exists(f):
    print("G1 SANE: no harness log (not a port-kit build); not checked"); sys.exit(0)
n = z = 0
exited = False
for l in open(f):
    p = l.split()
    if len(p) >= 2:
        n += 1
        z += p[1] == "0000"
        try: exited |= int(p[0], 16) == 0x3FF0 and p[1].upper() == "60FE"
        except ValueError: pass
share = z / n if n else 0.0
if n > 150000 or share > 0.5:
    print("G1 SANE FAIL: the original code ran astray on the harness (%d executed addresses, %.0f%% of them zero "
          "words = running through empty memory). Usually the game calls an AmigaDOS/Exec routine the harness does "
          "not emulate (loading files, allocating memory). Find the last real code before the run-away with "
          "TRACE=out/port/trace.txt bash kit/amigo/port/amigo_port.sh harness 50, then add the call to game.json 'hle' "
          "or a hooks_c routine." % (n, 100 * share))
    sys.exit(1)
if exited:
    # k17 (14 Oct 2026): oshle (kit/amigo/port/oshle) gives an OS-calling program a CLI to return to, a BRA.S * at
    # $3FF0. Measured on A10 Tank Killer: the run passed the zero-word test with 672 addresses because the program
    # gave up early (a file it wanted was not found) and returned there. A game that exits proved nothing either.
    print("G1 SANE FAIL: the original program exited back to the CLI on the harness after %d executed addresses "
          "(it reached oshle's exit at $3FF0). Usually a file it opens is not found or an OS call it checks returned "
          "0. Run the harness with OSHLE_ALL=1 and read the HLE lines before the exit in its stderr." % n)
    sys.exit(1)
print("G1 SANE: harness run looks real (%d executed addresses, %.1f%% zero words)" % (n, 100 * share))
