#!/usr/bin/env python3
"""irqdiff.py PORTLOG REFLOG: first difference between two IRQLOG files (frame line level interrupted-PC).
The interrupted PC shows WHERE in the game code each interrupt landed. If the sequences agree on frame, level and
line but not on the interrupted PC, the two CPUs see the interrupt at different instruction boundaries (interrupt
latency); if lines or levels differ first, the chipset timing seen by the code differs."""
import sys
a = [l.split() for l in open(sys.argv[1]) if l.strip()]
b = [l.split() for l in open(sys.argv[2]) if l.strip()]
n = min(len(a), len(b))
for i in range(n):
    if a[i] != b[i]:
        print("IRQDIFF first difference at entry %d of %d/%d" % (i, len(a), len(b)))
        for j in range(max(0, i - 3), min(n, i + 4)):
            print("  %s port %-28s ref %s" % ("->" if j == i else "  ", " ".join(a[j]), " ".join(b[j])))
        kinds = ["frame" if a[i][0] != b[i][0] else "", "line" if a[i][1] != b[i][1] else "", "level" if a[i][2] != b[i][2] else "", "pc" if a[i][3] != b[i][3] else ""]
        print("  differs in:", ",".join(k for k in kinds if k))
        sys.exit(1)
print("IRQDIFF identical for %d entries (port %d, ref %d)" % (n, len(a), len(b)))
