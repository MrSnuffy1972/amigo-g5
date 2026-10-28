"""cs_summary.py TRIAGE N: read gates/CS/* and write gates/CS/result.json (one record per game: stage reached, outcome class,
the first finding, counters, judge lines). The outcome class is decided by code only, so the queue can be clustered."""
import json, os, re, sys
CS = "/w/gates/CS"
def rd(n, d=""):
    p = os.path.join(CS, n)
    return open(p, errors="replace").read() if os.path.exists(p) else d
st = [l.split()[0] for l in rd("stages.txt").splitlines() if l.strip()]
r = {"triage": sys.argv[1], "ticks": int(sys.argv[2]), "stages": st, "last_stage": st[-1] if st else None,
     "fail": rd("fail").strip() or None, "snap_pc": rd("snap_pc").strip() or None, "snap_frame": rd("snap_frame").strip() or None}
try:
    sa = json.load(open(os.path.join(CS, "snapshot_assessment.json")))
    r["snapshot_assessment"] = {"status": sa.get("status"), "warnings": len(sa.get("warnings", [])), "errors": len(sa.get("errors", []))}
    r["g0_provenance"] = sa.get("provenance", {}).get("g0_files", {})
except Exception:
    r["snapshot_assessment"] = None
_snaplog = rd("3-snap.log")
r["snap_rule"] = ("B" if re.search(r"SNAPSHOT rule B \(depth 0, IPL mask \d+\)", _snaplog) else "A" if re.search(r"SNAPSHOT written to .* at PC [0-9A-Fa-f]{6} \(frame \d+\)", _snaplog) else None)
d = rd("5-discover.log")
rounds = re.findall(r"native_discover round (\d+): (\d+) interpreted addresses", d)
r["discover"] = [[int(a), int(b)] for a, b in rounds]
r["interp_left"] = int(rounds[-1][1]) if rounds else None
try:
    dl = json.load(open(os.path.join(CS, "discovery_ledger.json")))
    r["discovery_ledger"] = {"status": dl.get("status"), "fixed_point": dl.get("fixed_point"), "terminal_reason": dl.get("terminal_reason"), "iterations": len(dl.get("iterations", []))}
except Exception:
    r["discovery_ledger"] = None
try:
    ta = json.load(open(os.path.join(CS, "translation_attestation.json")))
    r["translation_attestation"] = {"status": ta.get("status"), "identity_sha256": ta.get("translation_identity_sha256"), "translation_tree_sha256": ta.get("translation", {}).get("tree_sha256"), "native_sha256": ta.get("build", {}).get("native_sha256")}
except Exception:
    r["translation_attestation"] = None
b = rd("6-build.log")
r["symcheck"] = "PASS" if re.search(r"symcheck.*PASS|PASS.*symcheck", b, re.I) else ("FAIL" if "symcheck" in b.lower() else None)
rc = rd("run_rc").strip()
r["run_rc"] = int(rc) if rc.lstrip("-").isdigit() else None
err = rd("7-run.err")
fl = [l for l in err.splitlines() if l.startswith("FINDING")]
r["findings_n"] = len(fl)
r["first_finding"] = fl[0][:240] if fl else None
kinds = {}
for l in fl:
    k = re.sub(r"[0-9A-F]{6}|\d+", "#", l)[:90]; kinds[k] = kinds.get(k, 0) + 1
r["finding_kinds"] = sorted(kinds.items(), key=lambda x: -x[1])[:8]
m = re.search(r"^FINDING: wild_jump .*$", err, re.M); r["wild_jump"] = m.group(0)[:240] if m else None   # wj2 19 Oct: the PC left the memory map (runaway); kit cpu_native.c wild-jump detection
m = re.search(r"^FINDING: rom_call .*$", err, re.M); r["rom_call"] = m.group(0)[:240] if m else None   # wj2 19 Oct: the PC entered the Kickstart ROM window F80000-FFFFFF; class stays run:untranslated-address
m = re.search(r"HWSTAT custom register accesses: (\d+) reads, (\d+) writes in (\d+) ticks; of all accesses \(custom \+ CIA\) (\d+) came through named calls", err)
if m: r["hw"] = {"reads": int(m.group(1)), "writes": int(m.group(2)), "ticks": int(m.group(3)), "named": int(m.group(4))}
st_out = rd("7-run.out") + err
for k in ("native_blits", "native_sprite_uses", "native_watchdog_trips", "native_unhandled", "interpreter_share"):
    m = re.search(k + r"\D{0,4}([\d.]+)", st_out)
    if m: r[k] = float(m.group(1))
nf = rd("native_frames").strip(); r["native_frames"] = int(nf) if nf.isdigit() else 0
r["input"] = rd("input_mode").strip() or None   # Cowork 18 Oct: fire / seq / seq-unsupported / None (attract only)
try:   # input intelligence (observation only, 19 Oct): gates/INPUT/g2-transitions.json from cs_inputmine.py correlate
    q = json.load(open("/w/gates/INPUT/g2-transitions.json")); r["input_structure"] = q.get("classification"); r["input_controls_proven"] = q.get("proven_controls"); r["input_transitions"] = q.get("transitions"); r["input_manifest_sha256"] = q.get("manifest_sha256")
except Exception: r["input_structure"] = None


def blank_ref():
    """Cowork 18 Oct (input discovery v1): the reference frames the judge compared, sampled every 25th: share of the most
    common colour on each. Blank = every sampled frame has one colour on more than 99.8% of its pixels."""
    import glob
    fs = sorted(glob.glob("/tmp/ref/f*.ppm") + glob.glob("/tmp/ref/f*.rgba") + glob.glob("/tmp/ref/f*.png"))[::25]   # Cowork 18 Oct
    if not fs: return None
    try:
        import numpy as np
        from PIL import Image
        sh = []
        for p in fs:
            if open(p, "rb").read(4) == b"RGBA":   # Cowork 18 Oct: raw RGBA frame ("RGBA", w, h little-endian uint32, RGBA bytes)
                a = np.fromfile(p, np.uint8)[12:].reshape(-1, 4)[:, :3]
            else: a = np.asarray(Image.open(p).convert("RGB")).reshape(-1, 3)
            _, c = np.unique(a, axis=0, return_counts=True); sh.append(float(c.max()) / len(a))
        r["ref_one_colour_share_min"] = round(min(sh), 5)
        return min(sh) > 0.998
    except Exception as ex:
        r["blank_check_error"] = str(ex)[:120]; return None


j = rd("9-judge.log")
for key, pat in (("picture", r"PICTURE: .*?on ([\d.]+)% of ticks.*?mean ([\d.]+).*?-> (\w+)"), ("active", r"ACTIVE ticks.*?: (\d+) of (\d+), SSIM >= 0.90 on ([\d.]+)%"),
                 ("sound", r"SOUND: .*?correlation ([\d.]+).*?-> (\w+)"), ("content", r"CONTENT: .*?on ([\d.]+)% of ticks.*?mean ([\d.]+).*?-> (\w+)")):
    m = re.search(pat, j)
    if m: r[key] = list(m.groups())
m = re.search(r"TICK_OFFSET: ([-+]\d+)", j)   # G2E10 tickoff: the per-game tick offset the judge applied (absent = none chosen, old judge)
r["tick_offset"] = int(m.group(1)) if m else None
m = re.search(r"TICK_OFFSET: [-+]\d+ \(auto[^;]*; margin over offset 0 ([-+][\d.]+)", j)   # G2E10 judgemargin: chosen offset's mean content gain over offset 0
r["tick_offset_margin"] = float(m.group(1)) if m else None
# outcome class (deterministic, for clustering)
if r["fail"]:
    c = "fail:" + (r["last_stage"] or "?")
elif r["run_rc"] == 3 and r["wild_jump"]: c = "run:wild-jump"   # wj2 19 Oct: a runaway, not translator coverage; clusters on the loop it left (r["wild_jump"])
elif r["run_rc"] == 3: c = "run:untranslated-address"
elif r["run_rc"] == 4: c = "run:decoder-access"
elif r["run_rc"] not in (0, None): c = "run:rc%s" % r["run_rc"]
elif r.get("active") and int(r["active"][0]) < max(5, r["native_frames"] // 20) and blank_ref(): c = "inconclusive:blank-reference"   # Cowork 18 Oct: the original shows one colour (>99.8% of pixels) all window: nothing to compare, not a title screen to press past; cr 21 Oct (Karl approved 20 Oct): checked BEFORE the static picture-fail rule, so a one-colour reference against a port that lights the screen is inconclusive, not judge:picture-fail
elif r.get("active") and int(r["active"][0]) < max(5, r["native_frames"] // 20) and r.get("picture") and r["picture"][2] != "PASS": c = "judge:picture-fail"   # Cowork 16 Oct 19:40: a static reference whose picture does NOT match is a real picture failure (23 of 70 were hidden as inconclusive)
elif r.get("active") and 5 <= int(r["active"][0]) < max(5, r["native_frames"] // 20) and float(r["active"][2]) >= 100.0: c = "inconclusive:low-activity-match"   # dsfix 18 Oct (G11): the reference changes on only 5-29 ticks and the port matches every one: better than static, still NOT a pass (cs_gates needs class pass*)
elif r.get("active") and int(r["active"][0]) < max(5, r["native_frames"] // 20): c = "inconclusive:static-reference"   # judge passes a black/static screen (0 active ticks): not a pass (Bionic Commando 34, 19:12)
elif r.get("picture") and r["picture"][2] == "PASS": c = "pass:picture" + ("+sound" if r.get("sound") and r["sound"][1] == "PASS" else "")
elif r.get("picture"): c = "judge:picture-fail"
else: c = "incomplete:" + (r["last_stage"] or "?")
r["class"] = c
try:
    ba = json.load(open(os.path.join(CS, "behaviour_attestation.json")))
    r["behaviour_attestation"] = {"status": ba.get("status"), "identity_sha256": ba.get("behaviour_identity_sha256"), "native_frames_sha256": ba.get("native_frames", {}).get("aggregate_sha256"), "reference_frames_sha256": ba.get("reference_frames", {}).get("aggregate_sha256")}
except Exception:
    r["behaviour_attestation"] = None
try:
    pg = json.load(open(os.path.join(CS, "promotion_decision.json")))
    r["promotion"] = {"decision": pg.get("decision"), "policy": pg.get("policy"), "identity_sha256": pg.get("decision_identity_sha256"), "hard_failures": pg.get("hard_failures"), "soft_failures": pg.get("soft_failures")}
except Exception:
    r["promotion"] = None
json.dump(r, open(os.path.join(CS, "result.json"), "w"), indent=1)
print(c)
