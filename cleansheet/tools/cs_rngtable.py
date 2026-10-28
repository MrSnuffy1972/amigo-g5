"""cs_rngtable.py RNGREC N OUT: G2E10 rngseed (Claude sub-agent, 27 Oct 2026; Karl 27 Oct, TEST-ONLY). From the reference harness's record of
entropy reads ("frame pc key value", from the snapshot moment), keep the read SITES (pc, key) that are not wait loops and write the native's
replay table "PC KEY n v1 .. vn" (hex). A wait loop (polling the beam or a timer until it reaches a value) reads a different number of
times in the port, so replaying it in order would desynchronise it; a random draw reads a few times per tick at most. Kept: sites read
at most 8 times in any one tick and at most 2 x N times in all. Reads from the snapshot tick onward (the native starts there)."""
import sys, collections
src, N, out = sys.argv[1], int(sys.argv[2]), sys.argv[3]
seq = collections.OrderedDict(); per = collections.Counter(); start = None
try:
    for l in open(src):
        if l.startswith("#"):
            if start is None and "start frame" in l: start = l.strip()
            continue
        p = l.split()
        if len(p) != 4: continue
        fr, pc, key, v = int(p[0]), int(p[1], 16), int(p[2], 16), int(p[3], 16)
        seq.setdefault((pc, key), []).append(v); per[(pc, key, fr)] += 1
except OSError:
    print("no reference record (%s): no replay table" % src); open(out, "w").close(); sys.exit(0)
mx = collections.Counter()
for (pc, key, fr), c in per.items(): mx[(pc, key)] = max(mx[(pc, key)], c)
keep = [(s, v) for s, v in seq.items() if mx[s] <= 8 and len(v) <= 2 * N]
with open(out, "w") as f:
    for (pc, key), v in keep: f.write("%06X %03X %X %s\n" % (pc, key, len(v), " ".join("%X" % x for x in v)))
print("record: %s; %d read sites, %d reads; kept %d sites (%d reads) for replay; dropped %d wait-loop sites (%d reads)" % (
    start, len(seq), sum(len(v) for v in seq.values()), len(keep), sum(len(v) for _, v in keep), len(seq) - len(keep), sum(len(v) for s, v in seq.items() if (s, v) not in keep)))
for (pc, key), v in keep[:40]: print("  keep %06X %03X x%d max/tick %d first %s" % (pc, key, len(v), mx[(pc, key)], " ".join("%X" % x for x in v[:6])))
