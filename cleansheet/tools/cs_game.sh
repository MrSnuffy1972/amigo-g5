#!/bin/bash
# cs_game.sh TRIAGE [TICKS]: clean-sheet v0 for ONE game (Cowork, 15 Oct 2026). Runs in the tools container at /w, which is a
# scratch copy of the game's prekit with Claude Code's DEV kit (kit-intent) in kit/. Nothing here touches the live kit or the
# game's real workspace. Stages, each logged to gates/CS/: ref (harness + first-execution order), pick (snapshot PC),
# snap (harness snapshot), snapcfg (game.json that boots the snapshot), discover (native_discover), build (out/native +
# symcheck), run (out/native, HWSTAT+DECSTAT, frames + sound), judge (judge.py against the harness from the snapshot frame).
# v0 limits: no input script (attract only); WHDLoad and AmigaDOS (filed) games; resload/OS calls during play are findings.
set -u
T="${1:-whdload}"; N="${2:-600}"
K=/w/kit/amigo/port; S=/w/cs_tools; CS=/w/gates/CS; H=/w/out/port/harness; IN=/w/gates/INPUT; IM="$S/cs_inputmine.py"   # input intelligence (observation only, 19 Oct)
# Cowork 18 Oct: frame format of the judge path (native dump + reference dump): rgba (default) | png | ppm. Rollback: put ppm in
# /w/cs_imgfmt (cs_queue --preset) or change the default here. The G0/G1 kit steps keep the kit default (AMIGO_IMG unset = ppm).
IMG=$(cat /w/cs_imgfmt 2>/dev/null || echo rgba)
# W1 beamfix (21 Oct, Karl approved ON by default): harness beam clock fix (cs_kitfix item 3, AMIGO_BEAMFIX gate). Off for one
# game: /w/cs_beamfix containing 0, or CS_BEAMFIX=0 in the environment. Off everywhere: deploy_w1_beamfix.sh --rollback.
BF=$(cat /w/cs_beamfix 2>/dev/null); [ "${CS_BEAMFIX:-${BF:-1}}" != 0 ] && export AMIGO_BEAMFIX=1
mkdir -p "$CS" "$IN"; : > "$CS/stages.txt"; echo "$IMG" > "$CS/img_format"
st() { echo "$1 $(date +%s)" >> "$CS/stages.txt"; }
# preflight (Component 1, 19 Oct 2026): run evidence in gates/CS/preflight.json + run_manifest.json, log preflight.log. Never
# changes rc or class (every call || true). CS_PREFLIGHT=off (env) or /w/cs_preflight_mode = off: nothing runs, no files.
pf() { local m="${CS_PREFLIGHT:-$(cat /w/cs_preflight_mode 2>/dev/null)}"; [ "$m" = off ] && return 0; [ -f "$S/cs_preflight.py" ] || return 0
  timeout 600 python3 "$S/cs_preflight.py" "$1" --triage "$T" --ticks "$N" >> "$CS/preflight.log" 2>&1 || true; return 0; }
fin() {
  python3 "$S/cs_summary.py" "$T" "$N" > "$CS/summary.log" 2>&1
  python3 "$S/cs_behaviour_attest.py" > "$CS/10-behaviour-attest.log" 2>&1 || true
  python3 "$S/cs_summary.py" "$T" "$N" > "$CS/summary.log" 2>&1
  pf final
  python3 "$S/cs_promotion_gate.py" > "$CS/11-promotion-gate.log" 2>&1 || true
  python3 "$S/cs_summary.py" "$T" "$N" > "$CS/summary.log" 2>&1
}
trap fin EXIT
: > "$S/noinput.txt"
pf start

st ref
WHDTIME=1 bash "$K/amigo_port.sh" harness 1500 > "$CS/1-ref.log" 2>&1 || { echo "harness build/run failed" > "$CS/fail"; exit 0; }
[ -s /w/out/port/ref/first_bytes.txt ] || { echo "no first_bytes.txt from the harness" > "$CS/fail"; exit 0; }
[ -f "$IM" ] && timeout 60 python3 "$IM" collect --phase g0-static --root /w --reference-log "$CS/1-ref.log" --first-bytes /w/out/port/ref/first_bytes.txt --output "$IN/g0-static.json" >> "$IN/inputmine.log" 2>&1 || true

st pick
SP=$(python3 "$S/cs_pickpc.py" "$T" "$CS/1-ref.log" 2> "$CS/2-pick.log") || { echo "no snapshot PC: $(tail -1 $CS/2-pick.log)" > "$CS/fail"; exit 0; }
echo "$SP" > "$CS/snap_pc"

st snap
mkdir -p /w/out/snap /tmp/snapref
case "$SP" in frame*) SNAPENV="SNAP_PC=FFFFFFFF SNAP_FRAME=${SP#frame }";; *) SNAPENV="SNAP_PC=$SP";; esac   # v0d
env $SNAPENV SNAP_OUT=/w/out/snap timeout 600 "$H" /w/game 3000 /tmp/snapref > "$CS/3-snap.log" 2>&1
[ -s /w/out/snap/snap_regs.txt ] || { echo "snapshot PC $SP never reached in 3000 frames" > "$CS/fail"; exit 0; }
F=$(awk '$1=="frame"{print $2}' /w/out/snap/snap_regs.txt); echo "$F" > "$CS/snap_frame"
awk '$1=="pc"{print substr($2,3)}' /w/out/snap/snap_regs.txt > "$CS/snap_pc"   # v0d: the PC actually snapshotted
# G2E2 latewin (Claude sub-agent, 25 Oct 2026): when the reference shows no change at all inside [F, F+N] (kept frames of the
# 1500-frame reference), cs_latewin.py picks the later frame where the original starts to move and the snapshot is retaken from
# there. Off for one game: /w/cs_latewin = 0; everywhere: CS_LATEWIN=0. Never when /w/cs_snap_override is set (an explicit choice).
LWO=$(cat /w/cs_latewin 2>/dev/null)
if [ "${CS_LATEWIN:-${LWO:-1}}" != 0 ] && [ -z "$(cat /w/cs_snap_override 2>/dev/null)" ] && [ -f "$S/cs_latewin.py" ]; then
  LX=$(timeout 900 python3 "$S/cs_latewin.py" "$F" "$N" 2>> "$CS/3c-latewin.log")
  if [ -n "$LX" ]; then
    mv /w/out/snap /w/out/snap.first; mkdir -p /w/out/snap /tmp/snapref2
    SNAP_PC=FFFFFFFF SNAP_FRAME=$LX SNAP_OUT=/w/out/snap timeout 600 "$H" /w/game 4000 /tmp/snapref2 > "$CS/3-snap.log" 2>&1
    [ -s /w/out/snap/snap_regs.txt ] || { echo "latewin snapshot from frame $LX never taken" > "$CS/fail"; exit 0; }
    F=$(awk '$1=="frame"{print $2}' /w/out/snap/snap_regs.txt); echo "$F" > "$CS/snap_frame"
    awk '$1=="pc"{print substr($2,3)}' /w/out/snap/snap_regs.txt > "$CS/snap_pc"
    echo "frame $LX -> snapshot frame $F" > "$CS/latewin"
  fi
fi
[ -f "$IM" ] && timeout 30 python3 "$IM" collect --phase snapshot --root /w --output "$IN/snapshot.json" >> "$IN/inputmine.log" 2>&1 || true
[ -f "$IM" ] && timeout 15 python3 "$IM" propose --root /w --static "$IN/g0-static.json" --snapshot "$IN/snapshot.json" --output "$IN/input_candidates.json" >> "$IN/inputmine.log" 2>&1 || true

# v0e (Cowork 16 Oct 19:45): scripted input. 70 games judged "static reference" because the attract-only run never leaves a
# title screen (47 of them already matched the picture). With /w/cs_input_mode = "fire", the SAME fire presses go to the
# native port (frames from the snapshot) and to the reference harness (frames + snapshot frame F), so the game moves and the
# judge sees active ticks. Joystick fire only: the harness pushes 'S' (space) twice per line, the port once.
NSCRIPT=cs_tools/noinput.txt; RSCRIPT=""
if [ "$(cat /w/cs_input_mode 2>/dev/null)" = "fire" ]; then
  : > "$S/in_nat.txt"; : > "$S/in_ref.txt"
  for t in 60 180 300 420; do
    printf "%d F\n%d -\n" $t $((t + 8)) >> "$S/in_nat.txt"
    printf "%d F\n%d -\n" $((F + t)) $((F + t + 8)) >> "$S/in_ref.txt"
  done
  NSCRIPT=cs_tools/in_nat.txt; RSCRIPT="$S/in_ref.txt"; echo fire > "$CS/input_mode"
fi
# v1 input (Cowork 18 Oct): winners of input discovery (/srv/fast/tmp/inputdisc/inputdisc.py), per game id in
# cs_tools/cs_inputs.json. The SAME sequence goes to the native port and the reference at the same frame offset from the
# snapshot (cs_inputseq.py). Used when /w/cs_input_mode is "seq", or absent and this game has a winner; "none" = attract only.
# Keys other than fire, directions, space and F1-F8 need the raw-key tokens in the kit's harness.c and main.c; without
# them the run stays attract-only and input_mode says seq-unsupported.
MODE=$(cat /w/cs_input_mode 2>/dev/null)
if [ "$MODE" = seq ] || [ -z "$MODE" ]; then
  python3 "$S/cs_inputseq.py" "$F" "$S/in_nat.txt" "$S/in_ref.txt" > "$CS/input_seq.log" 2>&1; IRC=$?
  if [ $IRC = 0 ]; then NSCRIPT=cs_tools/in_nat.txt; RSCRIPT="$S/in_ref.txt"; echo seq > "$CS/input_mode"
  elif [ $IRC = 1 ]; then echo seq-unsupported > "$CS/input_mode"; fi
fi

# v1b keyed reference coverage (Cowork 18 Oct): with an input script, run the reference harness once more WITH the script and add
# the first-execution bytes of every PC it reaches that the no-input reference never ran (code loaded or unpacked after a key
# or fire press) to out/port/ref/first_bytes.txt. The no-input lines stay first and unchanged (a union: nothing found before is
# lost); the no-input file is kept as first_bytes.noinput.txt. snapcfg then seeds those PCs and the translator takes their bytes.
if [ -n "$RSCRIPT" ]; then
  st refkeyed
  mkdir -p /tmp/refkeyed; FB=/w/out/port/ref/first_bytes.txt; cp -n "$FB" /w/out/port/ref/first_bytes.noinput.txt
  FIRSTBYTES=/tmp/fb_keyed.txt timeout 900 "$H" /w/game $((F + N + 60)) /tmp/refkeyed "$RSCRIPT" > "$CS/3b-refkeyed.log" 2>&1
  python3 - "$FB" /tmp/fb_keyed.txt >> "$CS/3b-refkeyed.log" 2>&1 <<'PYEOF'
import sys
fb, kf = sys.argv[1], sys.argv[2]
have = {l.split()[0] for l in open(fb) if l.strip()}
add = [l for l in open(kf) if l.strip() and l.split()[0] not in have] if __import__("os").path.exists(kf) else []
open(fb, "a").writelines(add)
print("keyed reference: %d first-executed PCs not in the no-input reference, added (no-input had %d)" % (len(add), len(have)))
PYEOF
fi

st snapcfg
cp -n /w/game.json /w/game-orig.json
python3 "$S/cs_snapcfg.py" "$T" > "$CS/4-snapcfg.log" 2>&1 || { echo "snapcfg failed" > "$CS/fail"; exit 0; }
pf snapshot
python3 "$S/cs_snapshot_assess.py" > "$CS/4b-snapshot-assess.log" 2>&1 || true

# G2E10 rngseed (Claude sub-agent, 27 Oct 2026; Karl 27 Oct, TEST-ONLY): the reference run comes BEFORE discovery and the native runs, and
# records the hardware reads a game can draw randomness from (RNGREC); cs_rngtable.py keeps the non-wait-loop read sites and the
# native runs below replay the reference values there (AMIGO_RNG_REPLAY), so a random screen draws the reference's numbers.
# Off for one game: /w/cs_rngseed = 0; everywhere: CS_RNGSEED=0.
st ref2
# G2E10 tickoff (Claude sub-agent, 27 Oct 2026; Karl 27 Oct): 3 reference frames past the window and the 2 before it (cs_align.py
# EXTRA -> /tmp/refx) so the judge can apply one constant per-game tick offset of up to +-2 (--auto-offset 2) with real reference shots.
mkdir -p /tmp/refall /tmp/ref
AMIGO_IMG=$IMG DUMPALL=1 RNGREC=/tmp/rngrec.txt RNGREC_PC=$(cat "$CS/snap_pc") RNGREC_FRAME=$F WAVOUT=/tmp/refall.raw AMIGO_INPUT_TRACE="$IN/g2-reference-events.jsonl" timeout 900 "$H" /w/game $((F + N + 3)) /tmp/refall $RSCRIPT > "$CS/8-ref2.log" 2>&1
python3 "$S/cs_align.py" "$F" "$N" 3 > "$CS/8-align.log" 2>&1
RNGT=""; RS=$(cat /w/cs_rngseed 2>/dev/null); if [ "${CS_RNGSEED:-${RS:-1}}" != 0 ] && [ -f "$S/cs_rngtable.py" ]; then
  python3 "$S/cs_rngtable.py" /tmp/rngrec.txt "$N" "$CS/rng_replay.txt" > "$CS/8b-rngtable.log" 2>&1 && RNGT="$CS/rng_replay.txt"; fi

st discover
# v1b keyed discovery (Cowork 18 Oct): when this run has an input script, discovery runs TWICE into the same seed file, first with
# no input (exactly the old coverage) and then with the script, so the seeds are the UNION of both and nothing found before is lost.
# Without a script it is the single no-input pass it always was. KD_ROUNDS (default 4) sets the rounds per pass.
KR=$(cat /w/cs_discover_rounds 2>/dev/null || echo 4)
if [ "$NSCRIPT" != cs_tools/noinput.txt ]; then
  NATIVE_DISCOVER_STATUS="$CS/5a-discover-noinput.status" timeout 1800 bash "$K/amigo_port.sh" native_discover "$KR" "$N" cs_tools/noinput.txt > "$CS/5a-discover-noinput.log" 2>&1 || echo "native_discover rc $?" >> "$CS/5a-discover-noinput.log"
  echo keyed+noinput > "$CS/discover_mode"
else echo noinput > "$CS/discover_mode"; fi
DRC=0; NATIVE_DISCOVER_STATUS="$CS/5-discover.status" timeout 1800 bash "$K/amigo_port.sh" native_discover "$KR" "$N" "$NSCRIPT" > "$CS/5-discover.log" 2>&1 || DRC=$?
[ "$DRC" = 0 ] || echo "native_discover rc $DRC" >> "$CS/5-discover.log"
# fix3b 19 Oct (disc2 guard): interpreted addresses left by a discovery log. "rc 1" with no line for the last round is grep -c
# finding 0 under set -e (= nothing left), so it counts as 0; a timeout (124) or no round line at all counts as unknown (-1).
discn() { local n; n=$(grep "native_discover round" "$1" 2>/dev/null | tail -1 | awk '{print $4}'); case "$2" in 1) echo 0;; 124) echo -1;; *) echo "${n:--1}";; esac; }
DN=$(discn "$CS/5-discover.log" "$DRC"); DREDO=1
[ "$DRC" = 124 ] && { DREDO=0; echo "rediscovery off: main discovery timed out (rc 124)" >> "$CS/7b-yields.log"; }
[ -f "$IM" ] && timeout 60 python3 "$IM" collect --phase g1-generated --root /w --discover-log "$CS/5-discover.log" --output "$IN/g1-generated.json" >> "$IN/inputmine.log" 2>&1 || true
[ -f "$IM" ] && timeout 15 python3 "$IM" merge --root /w --output "$IN/input_manifest.json" >> "$IN/inputmine.log" 2>&1 || true
pf generated

st build
bash "$K/amigo_port.sh" native > "$CS/6-build.log" 2>&1 || { echo "native build failed" > "$CS/fail"; exit 0; }
[ -x /w/out/native ] || { echo "no out/native" > "$CS/fail"; exit 0; }
pf build

st run
mkdir -p /tmp/nat
AMIGO_RNG_REPLAY="$RNGT" AMIGO_IMG=$IMG HWSTAT=1 DECSTAT=1 AMIGO_INPUT_TRACE="$IN/g2-native-events.jsonl" timeout 900 /w/out/native --frames "$N" --stats --dump /tmp/nat --wav /tmp/nat.raw --script "/w/$NSCRIPT" > "$CS/7-run.out" 2> "$CS/7-run.err"
echo $? > "$CS/run_rc"

st yields   # Component 3 (Copilot design, rebased onto fix3b 19 Oct): outer fixed point - add yields, rediscover, rebuild, rerun,
# until cs_addyields finds nothing new after the latest run. Keeps the fix3b guard (no rediscovery after a main-discovery timeout,
# a count that did not fall, or 0 -> 0) and caps the whole loop by time so cs_game.sh stays inside cs_queue's 5400 s per-game cap.
FP_MAX=$(cat /w/cs_fixedpoint_rounds 2>/dev/null || echo 4); FPLOG="$CS/7b-fixedpoint.log"
FP_DEADLINE=$(cat /w/cs_fixedpoint_deadline 2>/dev/null || echo 4200)   # bash SECONDS since cs_game.sh start; leaves ~1200 s for ref2 + judge
fp_hashes() { python3 -c '
import hashlib, json, os
c = json.load(open("/w/game.json")); sf = [s for s in c.get("seeds", []) if "snapcfg" in s]
p = "/w/" + sf[0] if sf else ""
seeds = sorted(set(l.strip().upper() for l in open(p) if l.strip())) if p and os.path.exists(p) else []
ys = sorted(set((int(v, 0) if isinstance(v, str) else int(v)) & 0xFFFFFF for v in c.get("yields", [])))
h = lambda xs: hashlib.sha256("\n".join(xs).encode()).hexdigest()[:12]
print("seeds=%s/%d yields=%s/%d" % (h(seeds), len(seeds), h(["%06X" % y for y in ys]), len(ys)))' 2>/dev/null || echo unreadable; }
FP_REASON=maximum-rounds; FP_R=0; FP_LAST=300; LASTD="$CS/5-discover.status"
echo "fixed point: max $FP_MAX rounds, deadline ${FP_DEADLINE}s, now ${SECONDS}s, start $(fp_hashes)" >> "$FPLOG"
for r in $(seq 1 "$FP_MAX"); do
  if [ $(( FP_DEADLINE - SECONDS )) -lt "$FP_LAST" ]; then FP_REASON=time-budget; break; fi
  t0=$SECONDS; H0=$(fp_hashes)
  if python3 "$S/cs_addyields.py" >> "$CS/7b-yields.log" 2>&1; then AY=changed; else AY=unchanged; fi
  if [ "$AY" = unchanged ]; then
    FP_REASON=converged-no-seed-or-yield-change
    case "$(sed -n 's/^reason=//p' "$LASTD" 2>/dev/null)" in interpreter-pc-cap) FP_REASON=interpreter-pc-cap;; wild-jump) FP_REASON=wild-jump;; esac
    echo "round $r: addyields unchanged -> stop $FP_REASON ($H0)" >> "$FPLOG"; break
  fi
  FP_R=$r; DREASON=off
  if [ "$DREDO" = 1 ]; then   # fix3b (dsfix G4 + disc2 guard): code behind a new yield is only seen by a fresh discovery
    DT=$(( FP_DEADLINE - SECONDS - 1200 )); [ "$DT" -gt 1800 ] && DT=1800
    if [ "$DT" -lt 120 ]; then DREASON=skipped-time-budget; DREDO=0; else
      rm -f "$CS/5c-discover-$r.status"; RRC=0
      NATIVE_DISCOVER_STATUS="$CS/5c-discover-$r.status" timeout "$DT" bash "$K/amigo_port.sh" native_discover "$KR" "$N" "$NSCRIPT" > "$CS/5c-discover-$r.log" 2>&1 || RRC=$?
      DREASON=$(sed -n 's/^reason=//p' "$CS/5c-discover-$r.status" 2>/dev/null); [ "$RRC" = 124 ] && DREASON=timeout; DREASON=${DREASON:-discovery-failed}
      LASTD="$CS/5c-discover-$r.status"
      RN=$(discn "$CS/5c-discover-$r.log" "$RRC"); echo "rediscovery $r: rc $RRC, interpreted $DN -> $RN, stop $DREASON" >> "$CS/7b-yields.log"
      # stop rediscovering on a timeout, an unknown count, a count that did not fall, or 0 -> 0 (nothing new behind the yields)
      if [ "$RRC" = 124 ] || [ "$RN" -lt 0 ] || { [ "$DN" -gt 0 ] && [ "$RN" -ge "$DN" ]; } || { [ "$DN" = 0 ] && [ "$RN" = 0 ]; }; then DREDO=0; echo "rediscovery off after round $r" >> "$CS/7b-yields.log"; fi
      DN=$RN
    fi
  fi
  bash "$K/amigo_port.sh" native > "$CS/6b-build-$r.log" 2>&1 || { echo "rebuild $r failed" >> "$CS/7b-yields.log"; FP_REASON=build-failed; echo "round $r: build failed" >> "$FPLOG"; break; }
  rm -rf /tmp/nat; mkdir -p /tmp/nat
  AMIGO_RNG_REPLAY="$RNGT" AMIGO_IMG=$IMG HWSTAT=1 DECSTAT=1 AMIGO_INPUT_TRACE="$IN/g2-native-events.jsonl" timeout 900 /w/out/native --frames "$N" --stats --dump /tmp/nat --wav /tmp/nat.raw --script "/w/$NSCRIPT" > "$CS/7-run.out" 2> "$CS/7-run.err"
  echo $? > "$CS/run_rc"; echo "round $r rc $(cat $CS/run_rc)" >> "$CS/7b-yields.log"
  FP_LAST=$(( SECONDS - t0 )); echo "round $r: addyields $AY, discover $DREASON, run rc $(cat $CS/run_rc), $H0 -> $(fp_hashes), ${FP_LAST}s" >> "$FPLOG"
  if [ "$DREASON" = wild-jump ]; then FP_REASON=wild-jump; break; fi
done
printf 'reason=%s\nrounds=%s\nmaximum_rounds=%s\ndeadline_s=%s\nelapsed_s=%s\n' "$FP_REASON" "$FP_R" "$FP_MAX" "$FP_DEADLINE" "$SECONDS" > "$CS/discovery-fixedpoint.status"
echo "fixed point stop: $FP_REASON after $FP_R rebuild round(s), ${SECONDS}s" >> "$FPLOG"
python3 "$S/cs_discovery_ledger.py" > "$CS/7c-discovery-ledger.log" 2>&1 || true
python3 "$S/cs_translation_attest.py" > "$CS/7d-translation-attest.log" 2>&1 || true


st judge
M=$(ls /tmp/nat | grep -cE '^p[0-9]+\.(rgba|png|ppm)$'); echo "$M" > "$CS/native_frames"
if [ "$M" -gt 10 ]; then
  python3 "$S/judge.py" /tmp/ref /tmp/nat --frames "$M" --wav-ref /tmp/ref.raw --wav-new /tmp/nat.raw --auto-offset 2 --ref-extra /tmp/refx > "$CS/9-judge.log" 2>&1
fi
[ -f "$IM" ] && timeout 30 python3 "$IM" correlate --root /w --manifest "$IN/input_manifest.json" --native-events "$IN/g2-native-events.jsonl" --reference-events "$IN/g2-reference-events.jsonl" --native-frames /tmp/nat --reference-frames /tmp/ref --native-audio /tmp/nat.raw --reference-audio /tmp/ref.raw --judge "$CS/9-judge.log" --output "$IN/g2-transitions.json" >> "$IN/inputmine.log" 2>&1 || true
[ -f "$IM" ] && timeout 15 python3 "$IM" merge --root /w --output "$IN/input_manifest.json" >> "$IN/inputmine.log" 2>&1 || true
# Cowork 18 Oct: the kept shots are PNG (for humans and storage), whatever format the run dumped
TOFF=$(sed -n 's/^TICK_OFFSET: \([-+0-9]*\).*/\1/p' "$CS/9-judge.log" 2>/dev/null | head -1); TOFF=${TOFF:-0}   # G2E10 tickoff: the f shot of port tick t is reference tick t-offset
mkdir -p /w/out/cs_shots; python3 - "$M" "$TOFF" <<'PYEOF'
import os, sys, numpy as np
from PIL import Image
O = int(sys.argv[2]) if len(sys.argv) > 2 else 0
# G2E10 jevfix shotslack (Claude sub-agent, 27 Oct 2026): the f shot is the reference tick among t-O-1, t-O, t-O+1 with the fewest
# differing pixels against the port shot (t-O wins ties): the judge's own +-1 tick slack around its chosen offset. Campaign 74: with
# offset -1 picked on content margins of 0.0000-0.0016, games 171 196 338 688 had shots that were byte-identical at offset 0 in
# campaign 73 and now differed by one animation tick, and JEV dissented. Chosen ticks are recorded in out/cs_shots/ftick.json.
def _load(s):
    b = open(s, "rb").read()
    if b[:4] == b"RGBA":
        w, h = int.from_bytes(b[4:8], "little"), int.from_bytes(b[8:12], "little")
        return Image.fromarray(np.frombuffer(b, np.uint8, w * h * 4, 12).reshape(h, w, 4)[..., :3].copy())
    return Image.open(s).convert("RGB")
def _find(d, x, t, alt=None):
    for e in ("rgba", "png", "ppm"):
        for dd in ((d,) if alt is None else (d, alt)):
            s = "%s/%s%05d.%s" % (dd, x, t, e)
            if os.path.exists(s): return s
    return None
FT = {}
for t in (50, 200, 400, int(sys.argv[1])):
    sp = _find("/tmp/nat", "p", t); P = _load(sp) if sp else None
    if P is not None: P.save("/w/out/cs_shots/p%05d.png" % t)
    best = None
    for k, u in enumerate((t - O, t - O - 1, t - O + 1)):
        sf = _find("/tmp/ref", "f", u, "/tmp/refx")
        if not sf: continue
        F_ = _load(sf)
        if P is None or F_.size != P.size:
            if k == 0: best = (0, u, F_)
            continue
        nd = int(np.any(np.asarray(F_) != np.asarray(P), axis=2).sum())
        if best is None or nd < best[0]: best = (nd, u, F_)
    if best is not None:
        best[2].save("/w/out/cs_shots/f%05d.png" % t); FT["%05d" % t] = {"ref_tick": best[1], "offset_tick": t - O, "diff_px": best[0]}
import json; json.dump(FT, open("/w/out/cs_shots/ftick.json", "w"), indent=1)
PYEOF
st done
