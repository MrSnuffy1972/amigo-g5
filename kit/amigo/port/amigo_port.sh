#!/bin/bash
# amigo_port.sh: the pilot's proven port pipeline as fixed steps (Ami-Go port kit, 13 Oct 2026).
#
# Karl asked whether the proven toolchain from the Ghosts 'n Goblins pilot could take load off the
# models once Ghidra has done its analysis. It can: everything below is code. A model's job is reduced
# to game.json (what is loaded where, which routines need a C replacement) and hooks.c.
#
# usage: bash /w/kit/amigo/port/amigo_port.sh STEP [args]     (work folder /w, or AMIGO_W)
#   g0                   G0 by code for hunk-executable games: rebuild_image.sh + MEMMAP.md from the pre-analysis
#   init                 write /w/game.json from gates/G0/MEMMAP.md (once G0 has passed)
#   show                 print the resolved game.json (code ranges, seeds, Ghidra files used)
#   translate            68000 -> C into out/port/gen; out/port/coverage.json (G1 format)
#   headless             translate + build out/headless (same options as the pilot's gng_headless)
#   harness [N] [SCRIPT] run the ORIGINAL code on Musashi for N frames (default 600) against the same
#                        chipset runtime: out/port/ref/f*.rgba (or .png/.ppm) and out/port/ref/executed_pcs.txt
#   discover [ROUNDS]    harness then translate, repeated until every executed PC is translated
#   difftest [TRIALS]    every translated instruction vs Musashi with random state (default 32 trials)
#   compare [N]          frame N (default 400) of the port vs frame N of the harness: pixels different
#   g1                   init + discover + headless + gates/G1/coverage.json + compare (the whole of G1)
#   g3 [FRAMES]          G3 by code (g3_standard interpreter): the port and the ORIGINAL code on the harness play the
#                        same input (gates/G3/input.txt; a default one is written if missing) for FRAMES frames
#                        (default 1500 = 30 s PAL); every frame's picture hash and the sound are compared.
#                        Writes out/port/g3/result.json and prints G3_MATCH. Needs no FS-UAE run and no model.
#   whdrun SLAVE [FRAMES] run a WHDLoad slave on the harness through the kit's C resload (runtime/whd.c), with no
#                        WHDLoad and no Kickstart: out/port/whd/ (frames, executed PCs, RAM, whd.log with every call)
#   windows              out/game.exe (mingw-w64, static SDL2, game data embedded)
#   sources              the C files of the port, one per line (for G5's build_apk.sh)
# Everything game-specific: game.json + the hooks_c files it names. See portcfg.py for the format.
set -euo pipefail
K="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
W="${AMIGO_W:-/w}"
export AMIGO_W="$W"
O="$W/out/port"
M="$K/musashi"
RT="$K/runtime"
mkdir -p "$O"
cd "$W"
step="${1:-}"; shift || true

memdefs() {   # k18: the memory model: WHD_CHIP_KB/WHD_SLOW_KB (whdrun) or gates/G0/g0.json chip_kb/slow_kb; default 512/512
  python3 - "$W/gates/G0/g0.json" <<'PYEOF'
import json, os, sys
try: c = json.load(open(sys.argv[1]))
except Exception: c = {}
ck = int(os.environ.get("WHD_CHIP_KB") or c.get("chip_kb") or 512); sk = int(os.environ.get("WHD_SLOW_KB") or c.get("slow_kb") or 512)
print(("-DCHIP_SIZE=%d " % (ck * 1024) if ck != 512 else "") + ("-DSLOW_SIZE=%d" % (sk * 1024) if sk != 512 else ""))
PYEOF
}
hooks() { python3 -c "import sys; sys.path.insert(0,'$K'); import portcfg as P; c=P.load(); print(' '.join(P.path(h) for h in c['hooks_c']))"; }
cc_all() {   # cc_all CC FLAGS OBJDIR files...
  local cc="$1" cf="$2" od="$3"; shift 3; mkdir -p "$od"; rm -f "$od"/*.o
  local f n=0 J="${AMIGO_J:-2}"      # 15 Oct (Karl, Ryzen unusable): at most AMIGO_J compiles at once (was one per source file, 411 cc1 at a time)
  for f in "$@"; do
    $cc $cf -c "$f" -o "$od/$(basename "${f%.c}")_$(echo "$f" | md5sum | cut -c1-6).o" &
    n=$((n+1)); if [ "$n" -ge "$J" ]; then wait -n; n=$((n-1)); fi
  done
  wait
}
MUS="$M/m68kcpu.c $M/m68kops.c $M/m68kdasm.c $M/softfloat/softfloat.c"
# G2E10 kickinterp: a kickemu game (game.json "kick") links Musashi ONLY to run the player's own Kickstart ROM (cpu_native.c); every other
# native exe stays interpreter-free. KICKINTERP_DEFAULT=0 (install HOLD=kickinterp) or AMIGO_KICKINTERP=0 = no interpreter.
KICKINTERP_DEFAULT=1
kickinterp_on() { [ "${AMIGO_KICKINTERP:-$KICKINTERP_DEFAULT}" != 0 ] && [ -f "$W/game.json" ] && python3 -c "import json,sys;k=json.load(open(sys.argv[1])).get('kick') or {};sys.exit(0 if int(str(k.get('size') or '0'),16) else 1)" "$W/game.json" 2>/dev/null; }
kif() { kickinterp_on && echo "-I$M -DAMIGO_KICKINTERP"; }
kis() { kickinterp_on && echo "$MUS $RT/rtkickstop.c"; }

do_translate() {
  rm -rf "$O/gen"; mkdir -p "$O/gen"
  python3 "$K/m68k2c.py" "$O/gen" --config "$W/game.json" --coverage "$O/coverage.json" "$@" | tail -2
}
do_boot() { rm -rf "$O/boot"; python3 "$K/portgen.py" gen "$O/boot"; }
do_headless() {
  do_translate; do_boot
  cc_all gcc "-O1 -w $(memdefs) -I$RT -I$M -DNO_SDL" "$O/obj_h" "$O"/gen/*.c "$O"/boot/*.c $RT/chipset.c $RT/boot.c $RT/rtkick.c $RT/whd.c $RT/cpu.c "$K/main.c" $MUS $(hooks)
  ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_h"
  g++ "$O"/obj_h/*.o "$O/ancient/libancient.a" -lm -o "$W/out/headless"
  echo "built out/headless"
}
do_native() {   # the clean-sheet exe (vertical slice): no Musashi, no chipset.c, no whd.c, no fake libraries; symcheck proves it
  do_translate; do_boot
  cc_all gcc "-O1 -w $(memdefs) -I$RT -I$O/gen -DNO_SDL -DAMIGO_NATIVE ${AMIGO_TRACE:+-DAMIGO_TRACE} $(kif)" "$O/obj_n" "$O"/gen/*.c "$O"/boot/*.c $RT/intentrt.c $RT/boot.c $RT/rtkick.c $RT/nowhd.c $RT/nativefs.c $RT/cpu_native.c "$K/main.c" $(hooks) $(kis)
  ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_n"   # G2E9 ndecrunch: nativefs.c resload Decrunch uses the harness's ancient glue
  g++ "$O"/obj_n/*.o "$O/ancient/libancient.a" -lm -o "$W/out/native"
  echo "built out/native"; bash "$K/symcheck.sh" "$W/out/native"
}
do_nativeprobe() {   # DEV ONLY: the native runtime with Musashi as a fallback, to find addresses the translator has not covered yet (never shipped)
  do_translate; do_boot
  cc_all gcc "-O1 -w $(memdefs) -I$RT -I$M -I$O/gen -DNO_SDL -DAMIGO_PROBE" "$O/obj_np" "$O"/gen/*.c "$O"/boot/*.c $RT/intentrt.c $RT/boot.c $RT/rtkick.c $RT/nowhd.c $RT/nativefs.c $RT/cpu_native.c "$K/main.c" $MUS $(hooks)
  ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_np"   # G2E9 ndecrunch (as native)
  g++ "$O"/obj_np/*.o "$O/ancient/libancient.a" -lm -o "$W/out/nativeprobe"
}
build_harness() {
  grep -qs boot_whd_slave "$O/boot/boot_gen.c" || do_boot   # tables from before 14 Oct lack boot_whd_slave
  cc_all gcc "-O2 -w $(memdefs) -I$RT -I$M" "$O/obj_r" "$K/harness.c" "$O"/boot/*.c $RT/chipset.c $RT/boot.c $RT/rtkick.c $RT/whd.c $MUS $(hooks)
  ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_r"
  g++ "$O"/obj_r/*.o "$O/ancient/libancient.a" -lm -o "$O/harness"
}
do_harness() {
  local n="${1:-600}" script="${2:-}"
  build_harness
  rm -rf "$O/ref"; mkdir -p "$O/ref"
  FIRSTBYTES="$O/ref/first_bytes.txt" "$O/harness" "$W/game" "$n" "$O/ref" $script 2>&1 | tail -3
}
do_compare() {
  local n="${1:-400}"
  [ -x "$W/out/headless" ] || do_headless
  build_harness
  rm -rf "$O/cmp"; mkdir -p "$O/cmp"
  "$W/out/headless" --frames "$n" --shot "$O/cmp/port.png" --stats > "$O/cmp/stats.txt"
  "$O/harness" "$W/game" "$n" "$O/cmp" >/dev/null 2>&1 || true
  REF=$(ls "$O"/cmp/$(printf f%05d "$n").* 2>/dev/null | head -1)   # Cowork 18 Oct: .rgba (default), .png or legacy .ppm
  python3 - "$O/cmp/port.png" "$REF" "$O/cmp/stats.txt" <<'EOF'
import sys, os
def ppm(p):   # Cowork 18 Oct: sniffs raw RGBA / PNG / legacy PPM; returns (w, h, RGB bytes)
    d = open(p, 'rb').read()
    if d[:4] == b'RGBA':
        w, h = int.from_bytes(d[4:8], 'little'), int.from_bytes(d[8:12], 'little'); a = d[12:]; o = bytearray(w * h * 3)
        o[0::3] = a[0::4]; o[1::3] = a[1::4]; o[2::3] = a[2::4]; return w, h, bytes(o)
    if d[:8] == b'\x89PNG\r\n\x1a\n':
        from PIL import Image
        im = Image.open(p).convert('RGB'); return im.width, im.height, im.tobytes()
    parts = d.split(b'\n', 3); w, h = map(int, parts[1].split()); return w, h, parts[3]
a = ppm(sys.argv[1]); b = ppm(sys.argv[2])
diff = sum(1 for i in range(0, len(a[2]), 3) if a[2][i:i+3] != b[2][i:i+3])
lit = sum(1 for i in range(0, len(a[2]), 3) if a[2][i:i+3] != b'\0\0\0')
st = dict(l.split(None, 1) for l in open(sys.argv[3]) if ' ' in l)
print("COMPARE frame %s: %d of %d pixels differ (port vs original code on Musashi); %d pixels not black; interpreter share %s" % (
    os.path.basename(sys.argv[2])[1:6], diff, a[0] * a[1], lit, st.get('interpreter_share', '?').strip()))
EOF
}

do_g3() {
  local n="${1:-1500}" G="$O/g3"
  mkdir -p "$W/gates/G3"; rm -rf "$G"; mkdir -p "$G"
  if [ ! -s "$W/gates/G3/input.txt" ]; then
    # default play: wait for the title, press fire to start, then move and fire in turn (same input both sides,
    # so the comparison holds whatever the game does with it)
    { echo "0 -"; for f in 250 300 350 400; do echo "$f F"; echo "$((f+6)) -"; done
      t=450; while [ $t -lt "$n" ]; do echo "$t R"; echo "$((t+40)) RF"; echo "$((t+46)) L"; echo "$((t+90)) U";
        echo "$((t+110)) D"; echo "$((t+130)) F"; echo "$((t+136)) -"; t=$((t+150)); done; } > "$W/gates/G3/input.txt"
    echo "wrote default gates/G3/input.txt"
  fi
  if [ -f "$W/gates/G1/build.sh" ]; then bash "$W/gates/G1/build.sh" > "$G/build.log" 2>&1 || { tail -5 "$G/build.log"; echo "G3_MATCH 0/0 = 0 (port build failed)"; return 1; }; else do_headless > "$G/build.log" 2>&1; fi
  build_harness
  # the port must take the kit interface (--frames, --script, --wav, HASHLOG; kit/amigo/port/main.c). Measured 13 Oct:
  # 3D Pool's own G1 build took only --frames/--script/--ppm, so it ran 0 frames; say so instead of scoring 0
  local wavopt=(--wav "$G/port.wav")
  if ! "$W/out/headless" --frames 1 --wav "$G/probe.wav" > /dev/null 2>"$G/probe.err"; then
    wavopt=()
    echo "NOTE: out/headless does not take --wav ($(head -c 160 "$G/probe.err" | tr '\n' ' ')): picture-only comparison; sound cannot be compared until it writes the mixed Paula output with --wav FILE like kit/amigo/port/main.c"
  fi
  HASHLOG="$G/port.hash" "$W/out/headless" --frames "$n" --script "$W/gates/G3/input.txt" "${wavopt[@]}" > /dev/null 2>"$G/port.err" || true
  [ -s "$G/port.hash" ] || echo "NOTE: out/headless wrote no frame hashes: it must write one line per frame to the file named by HASHLOG (frame, hash_frame(), hash_ram()), as kit/amigo/port/main.c does"
  HASHLOG="$G/ref.hash" WAVOUT="$G/ref.raw" "$O/harness" "$W/game" "$n" "$G" "$W/gates/G3/input.txt" > /dev/null 2>"$G/ref.err" || true
  python3 "$K/g3cmp.py" "$G" "$n"
}

ancient_lib() {   # ancient_lib CXX OUTDIR: temisu/ancient (BSD 2-Clause, Karl approved 14 Oct) as a static library, built once
  local cxx="$1" od="$2" A="$K/third_party/ancient"
  [ -s "$od/libancient.a" ] && return 0
  mkdir -p "$od"
  for f in $(cat "$A/OBJS.txt"); do
    if [ -f "$A/src/$f.cpp" ]; then echo "$A/src/$f.cpp"; else echo "$A/src/common/$f.cpp"; fi
  done | xargs -P "${AMIGO_J:-2}" -I{} sh -c '"$0" -Os -w -std=c++17 -fno-rtti -I"$1/src" -I"$1/api" -I"$1/api/ancient" -c "{}" -o "$2/$(basename "{}" .cpp).o"' "$cxx" "$A" "$od"
  ar rcs "$od/libancient.a" "$od"/*.o
}
ancient_obj() {   # ancient_obj CXX OBJDIR: the whd_decrunch glue (runtime/whd_ancient.cpp)
  "$1" -O2 -w -std=c++17 -I"$K/third_party/ancient/api/ancient" -c "$RT/whd_ancient.cpp" -o "$2/whd_ancient.o"
}

do_whdrun() {
  local slave="$1" n="${2:-1500}" G="$O/whd"
  [ -f "$slave" ] || { echo "whdrun: no slave $slave"; return 1; }
  rm -rf "$G"; mkdir -p "$G"
  cc_all gcc "-O2 -w $(memdefs) -I$RT -I$M" "$O/obj_whd" "$K/harness.c" $RT/boot_whdstub.c $RT/chipset.c $RT/boot.c $RT/rtkick.c $RT/whd.c $MUS
  ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_whd"
  g++ "$O"/obj_whd/*.o "$O/ancient/libancient.a" -lm -o "$O/harness_whd"
  WHD_SLAVE="$slave" WHD_DIR="$(dirname "$slave")" WHD_LOG="$G/whd.log" WHD_SAVE="$G/save" HASHLOG="$G/hash.txt" \
    "$O/harness_whd" "$(dirname "$slave")" "$n" "$G" 2>&1 | tail -4
  grep -E "^WHD (CALLS|EXIT|ABORT|NOT|NEED|EXCEPTION)" "$G/whd.log" | head -8
  python3 - "$G" <<'PYEOF'
import sys, os
g = sys.argv[1]
ops = [l.split() for l in open(os.path.join(g, "executed_ops.txt"))] if os.path.exists(os.path.join(g, "executed_ops.txt")) else []
z = sum(1 for o in ops if len(o) > 1 and o[1] == "0000")
h = [l.split()[1] for l in open(os.path.join(g, "hash.txt"))] if os.path.exists(os.path.join(g, "hash.txt")) else []
print("WHDRUN: %d executed addresses (%.1f%% zero words), %d frames, %d different pictures" % (len(ops), 100.0 * z / max(len(ops), 1), len(h), len(set(h))))
PYEOF
}

case "$step" in
  g0) python3 "$K/portg0.py" make ;;
  init) python3 "$K/portgen.py" init ;;
  show) python3 "$K/portgen.py" show ;;
  translate) do_translate ;;
  headless) do_headless ;;
  native) do_native ;;
  native_apk)   # Cowork 17 Oct: the clean-sheet port as an Android APK (SDL2 frontend in main.c, no Musashi, no chip model)
    do_translate; do_boot
    bash "$K/../android/build_apk.sh" --name "${1:-Ami-Go game}" --id "${2:-game}" --out "$W/out/game.apk" \
      --cflags "-O2 -w $(memdefs) -I$RT -I$O/gen -DAMIGO_NATIVE" "$O"/gen/*.c "$O"/boot/*.c $RT/intentrt.c $RT/boot.c $RT/rtkick.c $RT/nowhd.c \
      $RT/nativefs.c $RT/cpu_native.c "$K/main.c" $(hooks) ;;
  nativeprobe) do_nativeprobe ;;
  native_windows) do_native_windows ;;
  native_discover)   # native_discover [ROUNDS] [FRAMES] [SCRIPT]: probe, add the interpreted addresses as seeds, translate again, until none are left
    # Component 3 (Copilot design, rebased onto fix3b wild-jump 19 Oct): explicit stop reasons in $NATIVE_DISCOVER_STATUS -
    # converged-zero-interpreted, converged-no-new-seeds, interpreter-pc-cap, wild-jump (the probe left the memory map and stopped
    # recording, so another round would replay it), probe-failed (exit 3), maximum-rounds.
    rounds="${1:-6}"; fr="${2:-1500}"; sc="${3:-gates/G3/input.txt}"
    ndst="${NATIVE_DISCOVER_STATUS:-$W/out/native_discover.status}"; ndcap="${NATIVE_DISCOVER_PC_CAP:-4096}"
    seedf=$(python3 -c "import sys; sys.path.insert(0,'$K'); import portcfg as P; c=P.load(); print([s for s in c['seeds'] if 'snapcfg' in s][0] if any('snapcfg' in s for s in c['seeds']) else '')")
    [ -n "$seedf" ] || { echo "no snapcfg seed file in game.json"; exit 1; }
    nd_canon() { [ -f "$1" ] || : > "$1"; tr 'a-f' 'A-F' < "$1" | sed '/^[[:space:]]*$/d' | LC_ALL=C sort -u > "$1.canon"; mv "$1.canon" "$1"; }
    nd_canon "$W/$seedf"
    ndreason=maximum-rounds; ndr=0; n=0; ndadded=0; wj=""
    for r in $(seq 1 "$rounds"); do
      ndr=$r
      do_nativeprobe; prc=0; "$W/out/nativeprobe" --frames "$fr" --script "$W/$sc" --stats > "$W/out/probe.txt" 2>/dev/null || prc=$?
      wj=$(grep -m1 -E '^(wild_jump|rom_call)' "$W/out/probe.txt" || true)
      [ -n "$wj" ] && echo "native_discover round-$r $wj"   # fix3b: the probe left the memory map
      n=$(grep -c '^interp_pc' "$W/out/probe.txt" || true); n=${n:-0}; sh=$(grep '^interpreter_share' "$W/out/probe.txt" || true)
      echo "native_discover round $r: $n interpreted addresses, $sh"
      if [ "$prc" != 0 ] && [ -z "$wj" ]; then ndreason=probe-failed; break; fi
      if [ "$n" = 0 ]; then ndreason=converged-zero-interpreted; [ -n "$wj" ] && ndreason=wild-jump; break; fi
      ndb=$(wc -l < "$W/$seedf")
      # G2E10 kick: never seed a PC inside the player's Kickstart (game.json "kick"): its code is not translated into a port
      kr=$(python3 -c "import json;k=json.load(open('$W/game.json')).get('kick') or {};b=int(str(k.get('base','0')),16);print('%d %d'%(b,b+int(str(k.get('size','0')),16)))" 2>/dev/null || echo '0 0')
      grep '^interp_pc' "$W/out/probe.txt" | python3 -c "import sys;lo,hi=map(int,sys.argv[1:3]);[print(l.split()[1]) for l in sys.stdin if len(l.split())>1 and not lo<=int(l.split()[1],16)<hi]" $kr >> "$W/$seedf" || true; nd_canon "$W/$seedf"   # G2E10 kick
      ndadded=$(( $(wc -l < "$W/$seedf") - ndb ))
      if [ -n "$wj" ]; then ndreason=wild-jump; break; fi
      if [ "$ndadded" = 0 ]; then if [ "$n" -ge "$ndcap" ]; then ndreason=interpreter-pc-cap; else ndreason=converged-no-new-seeds; fi; break; fi
    done
    printf 'reason=%s\nrounds=%s\nmaximum_rounds=%s\ninterpreted=%s\nnew_seeds=%s\nseed_file=%s\nwild=%s\n' "$ndreason" "$ndr" "$rounds" "$n" "$ndadded" "$seedf" "$wj" > "$ndst" || true
    echo "native_discover stop: reason=$ndreason rounds=$ndr interpreted=$n new_seeds=$ndadded"
    [ "$ndreason" = probe-failed ] && exit 3
    true ;;
  harness) do_harness "${1:-600}" "${2:-}" ;;
  discover)
    rounds="${1:-3}"; do_boot
    for r in $(seq 1 "$rounds"); do
      do_harness "${2:-600}"
      do_translate
      left=$(python3 -c "import json;print(json.load(open('$O/coverage.json'))['executed_pcs_not_translated'])")
      echo "discover round $r: executed PCs not translated = $left"
      [ "$left" = 0 ] && break
    done ;;
  difftest)
    do_boot; mkdir -p "$O/test"
    python3 "$K/m68k2c.py" "$O/gen" --config "$W/game.json" --single "$O/test/singles.c" > /dev/null
    cc_all gcc "-O1 -w -DMEMLOG $(memdefs) -I$RT -I$M" "$O/obj_t" "$K/difftest.c" "$O/test/singles.c" "$O"/boot/*.c $RT/chipset.c $RT/boot.c $RT/rtkick.c $RT/whd.c $MUS $(hooks)
    ancient_lib g++ "$O/ancient"; ancient_obj g++ "$O/obj_t"
    g++ "$O"/obj_t/*.o "$O/ancient/libancient.a" -lm -o "$O/test/difftest"
    "$O/test/difftest" "${1:-32}" > "$O/test/difftest.txt"
    grep '^CYCLES' "$O/test/difftest.txt" | awk '{print $3, $5, $6, $7, $8}' | sort | uniq -c | sort -rn | head -40
    grep -v '^CYCLES' "$O/test/difftest.txt" | tail -6 ;;
  compare) do_compare "${1:-400}" ;;
  g3) do_g3 "${1:-1500}" ;;
  whdrun) do_whdrun "${1:-}" "${2:-1500}" ;;
  g1)
    python3 "$K/portgen.py" init
    bash "$0" discover 3
    do_headless
    mkdir -p "$W/gates/G1"; cp "$O/coverage.json" "$W/gates/G1/coverage.json"
    python3 -c "import json;c=json.load(open('$W/gates/G1/coverage.json'));print('COVERAGE reachable=%d translated=%d interpreted=%d executed-not-translated=%d'%(c['reachable'],c['translated'],c['interpreted'],c['executed_pcs_not_translated']))"
    do_compare 400 ;;
  windows)
    SDL="${SDL_MINGW:-}"
    for d in "$K/sdl2-mingw" /opt/SDL2-2.30.9/x86_64-w64-mingw32 "$W/third_party/SDL2-2.30.9/x86_64-w64-mingw32"; do [ -z "$SDL" ] && [ -f "$d/lib/libSDL2.a" ] && SDL="$d"; done
    [ -n "$SDL" ] || { echo "no mingw SDL2 found (set SDL_MINGW)"; exit 1; }
    do_translate; do_boot
    REL=""
    if [ "${AMIGO_RELEASE:-0}" = 1 ]; then   # release build (Karl 23:58, Gemini review items): diagnostics compiled out; untranslated code = build fails
      REL="-DAMIGO_RELEASE"
      python3 -c "import json,sys;c=json.load(open('$W/gates/G1/coverage.json'));n=c['executed_pcs_not_translated'];sys.exit('RELEASE BUILD REFUSED: %d executed PCs are not translated (no interpreter fallback in a release)'%n if n else 0)" || exit 1
    fi
    cc_all x86_64-w64-mingw32-gcc "-O2 -w $REL $(memdefs) -I$RT -I$M -I$SDL/include/SDL2" "$O/obj_w" "$O"/gen/*.c "$O"/boot/*.c $RT/chipset.c $RT/boot.c $RT/rtkick.c $RT/whd.c $RT/cpu.c "$K/main.c" $MUS $(hooks)
    ancient_lib x86_64-w64-mingw32-g++ "$O/ancient_w"; ancient_obj x86_64-w64-mingw32-g++ "$O/obj_w"
    x86_64-w64-mingw32-g++ -static -static-libgcc -static-libstdc++ -mwindows "$O"/obj_w/*.o "$O/ancient_w/libancient.a" "$SDL/lib/libSDL2.a" \
      -lsetupapi -lwinmm -limm32 -lversion -lole32 -loleaut32 -lgdi32 -luser32 -lshell32 -luuid -ladvapi32 -lcfgmgr32 -lm \
      -o "$W/out/game.exe"
    cp "$W/out/game.exe" "$W/out/game.unstripped.exe"      # G5 symcheck.sh needs the symbols (Karl 23:58)
    if [ "${AMIGO_RELEASE:-0}" = 1 ]; then bash "$K/symcheck.sh" "$W/out/game.unstripped.exe" || { echo "RELEASE BUILD REFUSED: chip-model or interpreter symbols are linked in (needs the native-intent runtime)"; mkdir -p "$W/out/_to_delete"; mv "$W/out/game.exe" "$W/out/_to_delete/game-refused-$(date +%s).exe"; exit 1; }; fi
    x86_64-w64-mingw32-strip "$W/out/game.exe"; ls -la "$W/out/game.exe" ;;
  sources)
    ls "$O"/gen/*.c "$O"/boot/*.c; echo "$RT/chipset.c"; echo "$RT/boot.c"; echo "$RT/rtkick.c"; echo "$RT/whd.c"; echo "$RT/cpu.c"; echo "$K/main.c"
    for f in $MUS $(hooks); do echo "$f"; done ;;
  *) sed -n '2,24p' "$0"; exit 2 ;;
esac
