#!/usr/bin/env python3
"""bestdiff.py PORT_DIR REF_DIR [--max 0.02]
G2 check: the closest pair between the port's screen grabs and the reference
game frames (those Laya/vision judged as 'game' in reference.json, else all
picture frames). Both are cropped to their non-black bounding box and scaled
to 320x256, then a pixel counts as different when any channel differs by more
than 24. Prints the measured fraction; exit 0 if it is at most --max."""
import json, os, sys
from PIL import Image, ImageChops

def norm(p):
    im = Image.open(p).convert("RGB")
    box = im.point(lambda v: 255 if v > 8 else 0).getbbox()
    if box: im = im.crop(box)
    return im.resize((320, 256), Image.NEAREST)

def frac(a, b):
    d = ImageChops.difference(a, b).point(lambda v: 255 if v > 24 else 0).convert("L")
    return sum(1 for v in d.getdata() if v) / (320 * 256)

port, ref = sys.argv[1], sys.argv[2]
mx = float(sys.argv[sys.argv.index("--max") + 1]) if "--max" in sys.argv else 0.02
refs = []
rj = os.path.join(ref, "reference.json")
if os.path.exists(rj):
    r = json.load(open(rj))
    refs = [os.path.join(ref, os.path.basename(r["frames_dir"]), v["frame"]) for v in r["verdicts"] if v.get("laya") in ("game", None)]
if not refs:
    for dp, _, fn in os.walk(ref):
        refs += [os.path.join(dp, f) for f in fn if f.endswith(".png")]
pf = [os.path.join(port, f) for f in sorted(os.listdir(port)) if f.endswith(".png")]
if not pf or not refs:
    print("NO_FRAMES port=%d ref=%d" % (len(pf), len(refs))); sys.exit(1)
R = [(r, norm(r)) for r in refs]
best = min((frac(norm(p), rn), p, r) for p in pf for r, rn in R)
print("BEST_DIFF %.4f port=%s ref=%s (pass if <= %.3f)" % (best[0], os.path.basename(best[1]), os.path.basename(best[2]), mx))
sys.exit(0 if best[0] <= mx else 1)
