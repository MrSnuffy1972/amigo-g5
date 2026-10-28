"""cs_latewin.py F N (G2 Epoch 2 "latewin", Claude sub-agent 25 Oct 2026): move a STILL reference window to where the original moves.
Read in the container at /w after the snap stage. F = snapshot frame, N = ticks. The kept frames of the 1500-frame no-input reference
(out/port/ref/fNNNNN, every 50th) inside [F, F+N] are compared pixel for pixel. Only when EVERY kept frame in the window is identical
to the one before it (the window shows no change at all) does it run the reference harness once more, 3000 frames, no input, and
print X = 50 frames before the change that opens the 600-tick window holding the most picture changes (ties: earliest; at least 3
changes, each to a frame that is not one colour). cs_game.sh then re-snapshots from frame X (SNAP_FRAME, as v0d does). Prints nothing
when the window already moves or no later window qualifies, so those games run exactly as before. The judge, the 0.90 / 95% rule and
the active-tick rule are untouched: a loader screen that does not change stays inconclusive."""
import glob, os, subprocess, sys
import numpy as np
F, N = int(sys.argv[1]), int(sys.argv[2])
H = "/w/out/port/harness"
def load(p):
    b = open(p, "rb").read()
    if b[:4] == b"RGBA":
        w, h = int.from_bytes(b[4:8], "little"), int.from_bytes(b[8:12], "little")
        return np.frombuffer(b, np.uint8, w * h * 4, 12).reshape(h, w, 4)[..., :3]
    from PIL import Image
    return np.asarray(Image.open(p).convert("RGB"))
def frames(d):
    out = {}
    for p in glob.glob(d + "/f[0-9]*.*"):
        if p.rsplit(".", 1)[-1] in ("rgba", "png", "ppm"):
            out[int(os.path.basename(p)[1:6])] = p
    return out
def same(a, b): return a.shape == b.shape and not (a != b).any()
def onecol(a):
    _, c = np.unique(a.reshape(-1, 3), axis=0, return_counts=True); return c.max() / float(c.sum()) > 0.998
ref = frames("/w/out/port/ref")
win = sorted(f for f in ref if F <= f <= F + N)
if len(win) < 2:
    sys.stderr.write("latewin: %d kept reference frames in [%d, %d]: not judged\n" % (len(win), F, F + N)); sys.exit(0)
prev = load(ref[win[0]])
for f in win[1:]:
    a = load(ref[f])
    if not same(prev, a):
        sys.stderr.write("latewin: window [%d, %d] moves (kept frame %d changed): unchanged\n" % (F, F + N, f)); sys.exit(0)
    prev = a
sys.stderr.write("latewin: window [%d, %d] is still on all %d kept frames; scanning 3000 frames\n" % (F, F + N, len(win)))
os.makedirs("/tmp/latescan", exist_ok=True)
subprocess.run([H, "/w/game", "3000", "/tmp/latescan"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=600)
sc = frames("/tmp/latescan"); ks = sorted(sc); ch = []
prev = None
for f in ks:
    a = load(sc[f])
    if prev is not None and not same(prev, a) and not onecol(a): ch.append(f)
    prev = a
sys.stderr.write("latewin: picture changes at kept frames %s\n" % ch[:40])
best = None
for b in ch:
    X = b - 50
    if X <= F or X + N > 3000: continue
    n = sum(1 for c in ch if X < c <= X + N)
    if n >= 3 and (best is None or n > best[1]): best = (X, n)
if best:
    sys.stderr.write("latewin: re-snapshot from frame %d (%d changes in [%d, %d])\n" % (best[0], best[1], best[0], best[0] + N)); print(best[0])
else:
    sys.stderr.write("latewin: no later 600-tick window with 3 or more changes: unchanged\n")
