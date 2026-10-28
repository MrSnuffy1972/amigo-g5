#!/bin/bash
# g4_inner.sh [TICKS] (G4 install, Cowork 25 Oct 2026; runs in amigo-tools:3 at /w = ws_g4/NNNNN, a COPY of the game's clean-sheet
# workspace with a CS-G3 pass). G4 = a Windows .exe of the clean-sheet native code with NO emulator code inside, passing the same
# picture + sound check as G2/G3 under wine. Steps: build (amigo_port.sh native_windows; do_native_windows comes from
# kit/amigo/port/native_windows.fn via BASH_ENV, so amigo_port.sh itself is unchanged), symcheck_g4 (inside the build), harness
# reference (same ref2 + cs_align.py as cs_game.sh, same snapshot frame and input script), out/native control, game.exe under wine +
# xvfb, judge.py on both, byte comparison, gates/G4/result.json (class pass:native-windows or fail:<step>).
set -u
cd /w; K=/w/kit/amigo/port; L=/w/gates/G4; mkdir -p "$L"; : > "$L/stages.txt"
st() { echo "$1 $(date +%s)" >> "$L/stages.txt"; }
export AMIGO_BEAMFIX=1
N="${1:-600}"; F=$(cat /w/gates/CS/snap_frame); H=/w/out/port/harness
[ -f /w/cs_tools/noinput.txt ] || : > /w/cs_tools/noinput.txt
NS=/w/cs_tools/noinput.txt; RS=""; NSW='Z:\w\cs_tools\noinput.txt'
case "$(cat /w/gates/CS/input_mode 2>/dev/null)" in seq|fire)
  if [ -s /w/cs_tools/in_nat.txt ] && [ -s /w/cs_tools/in_ref.txt ]; then NS=/w/cs_tools/in_nat.txt; RS=/w/cs_tools/in_ref.txt; NSW='Z:\w\cs_tools\in_nat.txt'; fi ;; esac
echo "input: native $NS reference ${RS:-none}" > "$L/input.txt"
st build
SDL_MINGW=/w/third_party/sdl2-mingw BASH_ENV="$K/native_windows.fn" bash "$K/amigo_port.sh" native_windows > "$L/1-build.log" 2>&1; echo $? > "$L/build_rc"
grep -A40 "^\[1\]" "$L/1-build.log" > "$L/2-symcheck.log"
SYMCHECK_EXE=/w/out/nativeprobe bash "$K/symcheck_g4.sh" /w > "$L/2b-symcheck-control-nativeprobe.log" 2>&1; echo $? > "$L/symcheck_control_rc"
if [ -f /w/out/native_win/game.exe ]; then
  st ref
  rm -rf /tmp/refall /tmp/ref; mkdir -p /tmp/refall /tmp/ref
  AMIGO_IMG=rgba DUMPALL=1 WAVOUT=/tmp/refall.raw timeout 900 "$H" /w/game $((F + N)) /tmp/refall $RS > "$L/3-ref.log" 2>&1
  python3 /w/cs_tools/cs_align.py "$F" "$N" > "$L/3-align.log" 2>&1
  st linux
  rm -rf /tmp/nat; mkdir -p /tmp/nat
  AMIGO_IMG=rgba timeout 900 /w/out/native --frames "$N" --dump /tmp/nat --wav /tmp/nat.raw --script "$NS" > "$L/4-linux.out" 2> "$L/4-linux.err"; echo $? > "$L/linux_rc"
  M=$(ls /tmp/nat | grep -cE '^p[0-9]+\.rgba$')
  python3 /w/cs_tools/judge.py /tmp/ref /tmp/nat --frames "$M" --wav-ref /tmp/ref.raw --wav-new /tmp/nat.raw > "$L/4-judge-linux.log" 2>&1
  st wine
  mkdir -p /tmp/wp /tmp/home; export WINEPREFIX=/tmp/wp WINEDEBUG=-all HOME=/tmp/home   # wine refuses a prefix whose parent it does not own
  WP=/w/g4/wineprefix.tgz; [ -f /g4ro/wineprefix.tgz ] && WP=/g4ro/wineprefix.tgz
  if [ -f "$WP" ]; then tar xzf "$WP" -C /tmp && echo "prefix from $WP" > "$L/5-wineboot.log"
  else timeout 300 wineboot -i > "$L/5-wineboot.log" 2>&1; fi
  # fix-g4-kick (27 Oct 2026, Karl): a kickemu game's container has the player's Kickstart ROMs + RTB tables mounted read-only (G4_KICK=1,
  # campaign._g4_kick_args); the Windows exe gets those folders as Z: paths for this one command (the harness and Linux control use the default path)
  KENV=''; [ "${G4_KICK:-0}" = 1 ] && KENV='AMIGO_KICKSTART=Z:\srv\fast\amigo\tools\kickroms WHD_RTBDIR=Z:\srv\fast\amigo\tools\whdload\rtb'
  wrun() { rm -rf /tmp/natw; mkdir -p /tmp/natw
    env $KENV AMIGO_IMG=rgba AMIGO_NOMENU=1 AMIGO_SAVE_PROFILE=0 SDL_AUDIODRIVER=dummy timeout 900 xvfb-run -a -s "-screen 0 1280x1024x24" \
      wine /w/out/native_win/game.exe --frames "$N" --dump 'Z:\tmp\natw' --wav 'Z:\tmp\natw.raw' --script "$NSW" > "$L/5-wine.out" 2> "$L/5-wine.err"; echo $? > "$L/wine_rc"; }
  wrun
  if [ "$(cat $L/wine_rc)" != 0 ] && [ -f "$WP" ]; then   # stale prefix: build a fresh one and retry once
    rm -rf /tmp/wp; mkdir -p /tmp/wp; timeout 300 wineboot -i >> "$L/5-wineboot.log" 2>&1; echo "retry after wineboot" >> "$L/5-wineboot.log"; wrun; fi
  MW=$(ls /tmp/natw | grep -cE '^p[0-9]+\.rgba$'); echo "$MW" > "$L/wine_frames"
  [ "$MW" -gt 10 ] && python3 /w/cs_tools/judge.py /tmp/ref /tmp/natw --frames "$MW" --wav-ref /tmp/ref.raw --wav-new /tmp/natw.raw > "$L/5-judge-wine.log" 2>&1
  st compare
  python3 - "$N" > "$L/6-wine-vs-linux.txt" <<'PYEOF'
import os, sys
n = int(sys.argv[1]); same = diff = miss = 0
for i in range(1, n + 1):
    a, b = "/tmp/nat/p%05d.rgba" % i, "/tmp/natw/p%05d.rgba" % i
    if not (os.path.exists(a) and os.path.exists(b)): miss += 1; continue
    if open(a, "rb").read() == open(b, "rb").read(): same += 1
    else: diff += 1
rd = lambda p: open(p, "rb").read() if os.path.exists(p) else b""
wa, wb = rd("/tmp/nat.raw"), rd("/tmp/natw.raw")
print("frames byte-identical wine vs linux: %d, different: %d, missing: %d" % (same, diff, miss))
print("audio bytes linux %d wine %d identical %s" % (len(wa), len(wb), wa == wb))
PYEOF
  mkdir -p "$L/shots"; python3 - <<'PYEOF'
import os, numpy as np
from PIL import Image
for t in (50, 200, 400, 600):
    for d, x, tag in (("/tmp/natw", "p", "wine"), ("/tmp/ref", "f", "ref")):
        s = "%s/%s%05d.rgba" % (d, x, t)
        if not os.path.exists(s): continue
        b = open(s, "rb").read(); w, h = int.from_bytes(b[4:8], "little"), int.from_bytes(b[8:12], "little")
        Image.fromarray(np.frombuffer(b, np.uint8, w * h * 4, 12).reshape(h, w, 4)[..., :3].copy()).save("/w/gates/G4/shots/%s-%05d.png" % (tag, t))
PYEOF
fi
st result
python3 - "$N" <<'PYEOF'
import json, os, re, sys
N = int(sys.argv[1]); L = "/w/gates/G4"
rd = lambda f: open(os.path.join(L, f)).read() if os.path.exists(os.path.join(L, f)) else ""
def num(f):
    try: return int(rd(f).strip())
    except ValueError: return None
def judge(f):
    t = rd(f); r = {}
    m = re.search(r"^PICTURE: (\d+) ticks, SSIM >= 0\.90 on ([\d.]+)% of ticks .*?mean ([\d.]+), min ([\d.]+) -> (\w+)", t, re.M)
    if m: r["picture"] = [float(m.group(2)), float(m.group(3)), m.group(5)]
    m = re.search(r"^ACTIVE ticks .*?: (\d+) of (\d+)", t, re.M)
    if m: r["active"] = [int(m.group(1)), int(m.group(2))]
    m = re.search(r"^SOUND: .*?mean spectral correlation ([\d.]+).*-> (\w+)", t, re.M)
    if m: r["sound"] = [float(m.group(1)), m.group(2)]
    return r
w, l = judge("5-judge-wine.log"), judge("4-judge-linux.log")
res = {"gate": "G4", "build_rc": num("build_rc"), "symcheck": "PASS" if "SYMCHECK_G4 PASS" in rd("1-build.log") else "FAIL",
       "symcheck_control_rc": num("symcheck_control_rc"), "run_rc": num("wine_rc"), "native_frames": num("wine_frames"), "ticks": N,
       "picture": w.get("picture"), "active": w.get("active"), "sound": w.get("sound"), "linux_control": l,
       "wine_vs_linux": rd("6-wine-vs-linux.txt").strip().splitlines(), "input": rd("input.txt").strip(),
       "exe_bytes": os.path.getsize("/w/out/native_win/game.exe") if os.path.exists("/w/out/native_win/game.exe") else None}
act, pic, snd = res["active"], res["picture"], res["sound"]
steps = [("build", res["build_rc"] == 0 and res["exe_bytes"]), ("symcheck", res["symcheck"] == "PASS"),
         ("wine-run", res["run_rc"] == 0 and res["native_frames"] == N), ("picture", bool(pic) and pic[2] == "PASS"),
         ("static-reference", bool(act) and act[0] >= max(5, N // 20)), ("sound", bool(snd) and snd[1] == "PASS")]
bad = next((s for s, ok in steps if not ok), None)
res["class"] = "pass:native-windows" if bad is None else "fail:" + bad
res["last_stage"] = (rd("stages.txt").split() or ["?"])[-2] if rd("stages.txt") else None
json.dump(res, open(os.path.join(L, "result.json"), "w"), indent=1)
print("G4 RESULT", res["class"], "picture", pic, "active", act, "sound", snd, "|", " / ".join(res["wine_vs_linux"]))
PYEOF
st done
