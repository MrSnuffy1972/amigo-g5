#!/usr/bin/env python3
"""g3check.py --standard fsuae|interpreter --need 0.99
(interpreter: reads out/port/g3/result.json from `bash kit/amigo/port/amigo_port.sh g3`, 13 Oct 2026)
G3 check. Runs the pilot's framematch.py on the port's frame hashes against
the reference grabs and measures the fraction of grabs matched exactly.
Inputs the model provides: out/g3/hashes.txt, gates/G3/ref/ (FS-UAE grabs)
or gates/G3/ref_interp/ (reference-interpreter grabs), and
gates/G3/framematch.json {"x0":..,"y0":..,"scale":..}."""
import json, subprocess, sys
a = sys.argv
std = a[a.index("--standard") + 1]; need = float(a[a.index("--need") + 1])
if std == "interpreter":
    # Karl, 13 Oct: G3 for library games is the reference-interpreter match, made by code (amigo_port.sh g3)
    try:
        r = json.load(open("out/port/g3/result.json"))
    except Exception as ex:
        print("G3_MATCH 0/0 = 0 against interpreter (need %.3f): no out/port/g3/result.json (%s)" % (need, ex)); sys.exit(1)
    errs = []
    if not r["conclusive"]:
        errs.append("not conclusive: the original showed %d different pictures in %d frames (change gates/G3/input.txt "
                    "so the play reaches the game)" % (r["ref_distinct_pictures"], r["ref_frames"]))
    if r["fraction"] < need:
        errs.append("pictures: %d/%d frames match, first miss at frame %s" % (r["matched"], r["frames"], r["first_miss"]))
    if r["sound_ref_peak_rms"] >= 50 and (r["sound_corr"] is None or r["sound_corr"] < 0.8):
        errs.append("SOUND FAIL: correlation %s with the original (need 0.8)" % r["sound_corr"])
    print("G3_MATCH %d/%d = %.4f against interpreter (need %.3f); first miss: %s; sound corr %s (original peak RMS %s)%s" % (
        r["matched"], r["frames"], r["fraction"], need, r["first_miss"], r["sound_corr"], r["sound_ref_peak_rms"],
        ("; " + "; ".join(errs)) if errs else "; PASS"))
    sys.exit(1 if errs else 0)
refdir = "gates/G3/ref"
fm = json.load(open("gates/G3/framematch.json"))
r = subprocess.run(["python3", "kit/tools/framematch.py", "out/g3/hashes.txt", refdir,
                    str(fm["x0"]), str(fm["y0"]), str(fm.get("scale", 2))], capture_output=True, text=True)
lines = [l for l in r.stdout.splitlines() if l.strip()]
if r.returncode != 0 or not lines:
    print("framematch failed rc=%d %s" % (r.returncode, r.stderr[-400:])); sys.exit(1)
matched = sum(1 for l in lines if not l.rstrip().endswith(" -"))
f = matched / len(lines)
first_miss = next((l for l in lines if l.rstrip().endswith(" -")), None)
print("G3_MATCH %d/%d = %.4f against %s (need %.3f); first miss: %s" % (matched, len(lines), f, std, need, first_miss))
sys.exit(0 if f >= need else 1)
