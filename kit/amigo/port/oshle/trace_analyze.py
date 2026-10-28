#!/usr/bin/env python3
"""trace_analyze.py TRACE : summarise an AMIGO_OSHLE_TRACE log (g52, Cowork 19 Oct 2026, from Gemini's sketch).
Prints the bitplane-DMA-on frame, every UNIMPLEMENTED call (first frame, count), the first 15 calls that returned 0/-1 before
the screen came on (many are benign: IoErr, CurrentDir, Delay return 0 by design) and the last 8 calls of the run."""
import collections, re, sys
if len(sys.argv) < 2: sys.exit("usage: trace_analyze.py TRACE")
lines = open(sys.argv[1], errors="replace").read().splitlines()
BENIGN = {"IoErr", "CurrentDir", "Delay", "WaitForChar", "Forbid", "Permit", "Disable", "Enable", "SetSignal", "Close", "UnLock", "CloseLibrary"}
dma = next((l for l in lines if "BPL DMA ON" in l), None)
print("calls: %d   %s" % (sum(1 for l in lines if " CALL " in l), dma or "bitplane DMA never seen on at a library call"))
un = collections.OrderedDict()
for l in lines:
    if l.endswith("UNIMPLEMENTED"):
        m = re.match(r"FRAME (\d+) CALL (\S+) (\S+)", l); k = (m.group(2), m.group(3)) if m else (l, "")
        if k not in un: un[k] = [int(m.group(1)) if m else -1, 0]
        un[k][1] += 1
print("UNIMPLEMENTED (first frame, count):")
for (lib, fn), (f, c) in un.items(): print("  %-22s %-24s frame %-6d x%d" % (lib, fn, f, c))
print("first FAIL? before the screen came on (benign names skipped):")
n = 0
for l in lines:
    if "BPL DMA ON" in l: break
    if "(FAIL?)" in l:
        m = re.match(r"FRAME \d+ CALL \S+ (\S+)", l)
        if m and m.group(1) in BENIGN: continue
        print("  " + l); n += 1
        if n >= 15: break
print("last calls:")
for l in [l for l in lines if " CALL " in l][-8:]: print("  " + l)
