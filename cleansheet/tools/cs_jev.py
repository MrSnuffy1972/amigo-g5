"""cs_jev.py WORKSPACE (Cowork 16 Oct 2026, Karl: side-by-side picture, original left, port right, JEV asked "same?").
Runs on the HOST (JEV needs the network; the clean-sheet container has none). Only for a result whose class starts with
"pass": each out/cs_shots pair f/p is joined left/right and JEV answers a noul. Dissent gate (doctrine): code and JEV must
agree; mean noul < 0.5 or any frame < 0.2 turns the class into inconclusive:jev-dissent (held for review, class_before kept).
JEV unreachable -> class unchanged, jev_error recorded (code alone decides, as before)."""
import sys, os, glob, json
sys.path.insert(0, os.path.expanduser("~/bin"))
from PIL import Image
w = sys.argv[1]; rp = os.path.join(w, "gates", "CS", "result.json")
r = json.load(open(rp))
if not str(r.get("class", "")).startswith("pass"):
    print(r.get("class")); sys.exit(0)
Q = {"same": {"type": "noul", "instructions": "`pair` shows two Amiga game screenshots side by side, separated by a magenta bar. LEFT half: the original game in an emulator (ground truth). RIGHT half: our converted program at the same moment. Is the right half the same picture as the left half (same screen, graphics, colours and positions; a few stray pixels do not matter)?",
              "criteria": {"true": "the right half shows the same picture as the left half", "false": "the right half differs visibly from the left half (different screen, missing or wrong graphics, wrong colours, shifted, blank or garbled)"}},
     # Karl 16 Oct: "is this a blank image?" asked of the PORT half; a code measure of the same thing is recorded beside it
     "port_blank": {"type": "noul", "instructions": "In `pair`, is the RIGHT half (right of the magenta bar) blank: one flat colour with nothing drawn on it?",
                    "criteria": {"true": "the right half is blank, a single flat colour with no text, graphics or sprites", "false": "the right half shows something: text, graphics, sprites or a picture"}}}
import numpy as np
def code_blank(im):
    a = np.asarray(im).reshape(-1, 3); v, c = np.unique(a, axis=0, return_counts=True); return float(c.max()) / len(a) > 0.998
# fix-jevcal (Claude sub-agent, 26 Oct 2026; measured in /srv/fast/amigo/scratch/g2/fix-jevcal, results.json):
# the per-frame floor was 0.2 and let known-different pairs through. Fitted on our own labelled pairs (144 labelled, split by
# game): highest known-different score in the FIT half 0.40 (a System 3 logo with wrong colours), largest call-to-call drift
# 0.08 over 22 repeated pairs, so the floor is 0.40 + 0.08 + 0.01 = 0.49. HOLDOUT, JEV-judged pairs (11 same, 40 different):
# floor 0.2 -> 1/40 false passes, 3/11 false fails, accuracy 0.922; floor 0.49 -> 0/40 false passes, 6/11 false fails,
# accuracy 0.882 (the extra fails are near-identical timing frames). The mean rule stays at 0.5. A false pass is worse.
JEV_MEAN_MIN = 0.5     # mean "same" noul over the sampled frames (identical frames count 1.0), unchanged
JEV_FRAME_MIN = 0.49   # was 0.2 (fix-jevcal, 26 Oct 2026)
# Karl's ruling 26 Oct 2026: a NEAR-identical pair is decided by code like a byte-identical one (counts 1.0, JEV not asked).
# Measure: pixels whose RGB differs, counted over the whole frame, on the same decoded shots (same size only). 0.2% of a
# 352x272 frame is 191 pixels. The code blank checks still apply to these frames. A difference confined to a small area
# (one score digit, a cursor, one letter of text) is within 0.2% and PASSES without JEV: deliberate, see fix-jevcal NOTES.md.
JEV_NEAR_MAX = 0.002
d = os.path.join(w, "out", "cs_shots"); res = {}; blank = {}; ident = []; near = {}
# jev-error ruling (Karl 26 Oct 2026): every frame with a port shot, listed BEFORE anything can fail, so a frame never
# reached because JEV failed on an earlier one still counts as undecided.
allk = [os.path.basename(f)[1:6] for f in sorted(glob.glob(d + "/f*.ppm") + glob.glob(d + "/f*.png")) if os.path.exists(os.path.join(d, "p" + os.path.basename(f)[1:]))]   # G2E5 jevident: frames code proves identical
try:
    # jev-error ruling (Karl 26 Oct 2026): jev is imported only when a frame needs it, so a run whose frames are all decided
    # by code never depends on JEV being reachable.
    for f in sorted(glob.glob(d + "/f*.ppm") + glob.glob(d + "/f*.png")):   # Cowork 18 Oct: shots are PNG now, PPM in old runs
        p = os.path.join(d, "p" + os.path.basename(f)[1:])
        if not os.path.exists(p): continue
        A, B = Image.open(f).convert("RGB"), Image.open(p).convert("RGB")
        if A.size == B.size and A.tobytes() == B.tobytes():   # G2E5 jevident (Claude sub-agent 25 Oct 2026): byte-identical pair: code decides
            k = os.path.basename(f)[1:6]; ident.append(k)
            blank[k] = {"jev_port_blank": None, "code_port_blank": code_blank(B), "code_ref_blank": code_blank(A), "identical": True}
            continue
        if A.size == B.size:   # fix-jevcal near-identical rule (Karl 26 Oct 2026): code decides
            nd = int(np.any(np.asarray(A) != np.asarray(B), axis=2).sum())
            if nd <= JEV_NEAR_MAX * A.width * A.height:
                k = os.path.basename(f)[1:6]; near[k] = nd
                blank[k] = {"jev_port_blank": None, "code_port_blank": code_blank(B), "code_ref_blank": code_blank(A), "near": True, "diff_px": nd}
                continue
        im = Image.new("RGB", (A.width * 2 + 8, A.height), (255, 0, 255)); im.paste(A, (0, 0)); im.paste(B, (A.width + 8, 0))
        fn = os.path.join(d, "sxs" + os.path.basename(f)[1:6] + ".png")
        try: im.save(fn)
        except OSError:   # G2E10 jevfix sxsfallback (27 Oct 2026): out/cs_shots is root-owned after a Ryzen-local run (11, 730 jev-error)
            fn = os.path.join(w, "gates", "CS", os.path.basename(fn)); im.save(fn)
        import jev
        ref, asset = jev.image_asset(fn)
        ans = jev.ask({"pair": ref}, Q, caller="amigo:cs-sxs", assets={ref: asset}); k = os.path.basename(f)[1:6]
        res[k] = float(ans["same"]["noul"])
        blank[k] = {"jev_port_blank": float(ans["port_blank"]["noul"]), "code_port_blank": code_blank(B), "code_ref_blank": code_blank(A)}
except Exception as ex:
    r["jev_error"] = str(ex)[:200]
# jev-error ruling (Karl 26 Oct 2026): a frame code did not decide and JEV did not answer (unreachable, refused, error)
# means no verdict, so the class becomes inconclusive:jev-error (re-run) instead of keeping the code pass.
undecided = [k for k in allk if k not in res and k not in ident and k not in near]
if undecided:
    r["class_before"] = r["class"]; r["class"] = "inconclusive:jev-error"; r["jev_undecided"] = undecided
    r["jev"] = res; r["blank"] = blank
    if ident: r["jev_identical"] = ident
    if near: r["jev_near"] = near
    r["first_finding"] = "no JEV verdict on frame(s) %s not decided by code: %s" % (",".join(undecided), r.get("jev_error") or "JEV did not answer")
elif res or ident or near:
    r["jev"] = res; r["blank"] = blank; vals = list(res.values()) + [1.0] * (len(ident) + len(near)); mean = sum(vals) / len(vals)
    if near: r["jev_near"] = near   # fix-jevcal: near-identical frames (differing pixel count), code agreement, counted as 1.0
    if ident: r["jev_identical"] = ident   # G2E5 jevident: these frames were not asked (pixel-identical: code agreement, counted as 1.0)
    # a blank port where the original shows something is a failure whichever decider sees it (JEV >= 0.7 or the code measure)
    blank_bad = [k for k, b in blank.items() if not b["code_ref_blank"] and (b["code_port_blank"] or (b["jev_port_blank"] or 0) >= 0.7)]
    if blank and all(b["code_ref_blank"] for b in blank.values()):
        # Karl's blank question found it (16 Oct): game 180 "passed" with the original AND the port black in every sampled
        # frame. A blank screen matching a blank screen proves nothing.
        r["class_before"] = r["class"]; r["class"] = "inconclusive:blank-reference"
        r["first_finding"] = "the original shows a blank screen in every sampled frame; a blank port matching it proves nothing"
    elif mean < JEV_MEAN_MIN or min(vals) < JEV_FRAME_MIN or blank_bad:
        r["class_before"] = r["class"]; r["class"] = "inconclusive:jev-dissent"
        r["first_finding"] = "JEV dissent: code check passed but JEV says the port picture differs (noul per frame %s)" % res
json.dump(r, open(rp, "w"), indent=1)
print(r["class"])
