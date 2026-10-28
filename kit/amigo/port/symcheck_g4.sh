#!/bin/bash
# symcheck_g4.sh WORKDIR: G4 proof (Cowork 25 Oct 2026) that out/native_win/game.unstripped.exe links NO emulator code.
# Five checks, any hit = exit 1:
#  1 the kit's symcheck.sh name patterns (m68k_*, interp_run, chipset.c entry points)
#  2 extra name patterns: Musashi internals (m68ki_*, softfloat), harness and resload symbols
#  3 object proof: every global symbol DEFINED by the reference build's forbidden objects (out/port/obj_r: harness.c, chipset.c,
#    whd.c, Musashi; out/port/obj_h: cpu.c) and NOT also defined by the native sources (out/port/obj_nw) must be absent from the exe
#  4 strings: no source-file names of the emulator (chipset.c, whd.c, harness.c, m68kcpu, m68kops, musashi)
#  5 PE imports: Windows system DLLs only (SDL2 is static; any other DLL is a failure)
W="${1:-/w}"; EXE="${SYMCHECK_EXE:-$W/out/native_win/game.unstripped.exe}"; O="$W/out/port"; K="$W/kit/amigo/port"
NM=x86_64-w64-mingw32-nm; OD=x86_64-w64-mingw32-objdump; fail=0
[ -f "$EXE" ] || { echo "SYMCHECK_G4 FAIL: no $EXE"; exit 1; }
echo "[1] kit symcheck.sh"; bash "$K/symcheck.sh" "$EXE" || fail=1
echo "[2] extra patterns"
PAT='^(m68k[a-z_0-9]*|m68ki_[a-z_0-9]*|musashi[a-z_0-9]*|softfloat[a-z_0-9]*|float32_[a-z_0-9]*|floatx80_[a-z_0-9]*|harness_[a-z_0-9]*|resload_[a-z_0-9]*|whd_resload[a-z_0-9]*|cpu_execute)$'   # not interp_*: cpu_native.c's interp_pcs/stat_interp_* are --stats counters (0 in a native build)
$NM "$EXE" | awk '{print $NF}' | sort -u > /tmp/g4_exe_syms.txt
KI=0; grep -qE '^_?kick_interp_only$' /tmp/g4_exe_syms.txt && { KI=1; echo "  G2E10 kickinterp: kick_interp_only: Musashi is linked only to run the player's own Kickstart ROM (its names, objects and strings allowed)"; }
[ $KI = 1 ] && PAT='^(harness_[a-z_0-9]*|resload_[a-z_0-9]*|whd_resload[a-z_0-9]*|cpu_execute)$'
h=$(grep -Ei "$PAT" /tmp/g4_exe_syms.txt); [ $KI = 1 ] && h=$(echo "$h" | grep -vx harness_instr_hook)   # G2E10 kickinterp2
if [ -n "$h" ]; then echo "  FAIL $(echo "$h" | wc -l): $(echo "$h" | head -8 | tr '\n' ' ')"; fail=1; else echo "  PASS ($(wc -l < /tmp/g4_exe_syms.txt) symbols scanned)"; fi
echo "[3] forbidden-object proof"
FOB=$(ls "$O"/obj_r/harness_*.o "$O"/obj_r/chipset_*.o "$O"/obj_r/whd_*.o "$O"/obj_r/m68k*.o "$O"/obj_r/softfloat_*.o "$O"/obj_h/cpu_*.o 2>/dev/null)
[ $KI = 1 ] && FOB=$(ls "$O"/obj_r/harness_*.o "$O"/obj_r/chipset_*.o "$O"/obj_r/whd_*.o "$O"/obj_h/cpu_*.o 2>/dev/null)   # G2E10 kickinterp
if [ -z "$FOB" ]; then echo "  FAIL: no reference objects to prove against (out/port/obj_r, obj_h)"; fail=1; else
  nm --defined-only -g $FOB 2>/dev/null | awk 'NF==3{print $3}' | sort -u > /tmp/g4_forbid_all.txt
  $NM --defined-only -g "$O"/obj_nw/*.o 2>/dev/null | awk 'NF==3{print $3; if ($3 ~ /^\.weak\./) { split($3, a, "."); print a[3] }}' | sort -u > /tmp/g4_allowed.txt   # G2E10 g4weak
  comm -23 /tmp/g4_forbid_all.txt /tmp/g4_allowed.txt > /tmp/g4_forbid.txt
  h=$(comm -12 /tmp/g4_forbid.txt /tmp/g4_exe_syms.txt)
  echo "  forbidden objects: $(echo "$FOB" | wc -l), symbols only they define: $(wc -l < /tmp/g4_forbid.txt), shared with native sources (allowed): $(comm -12 /tmp/g4_forbid_all.txt /tmp/g4_allowed.txt | wc -l)"
  if [ -n "$h" ]; then echo "  FAIL $(echo "$h" | wc -l) in exe: $(echo "$h" | head -8 | tr '\n' ' ')"; fail=1; else echo "  PASS: none of them in the exe"; fi
fi
echo "[4] strings"
SPAT='(^|[^a-z0-9_])(chipset\.c|whd\.c|harness\.c|cpu\.c)|m68kcpu|m68kops|musashi'; [ $KI = 1 ] && SPAT='(^|[^a-z0-9_])(chipset\.c|whd\.c|harness\.c|cpu\.c)'   # G2E10 kickinterp
h=$(strings -a "$EXE" | grep -Eio "$SPAT" | sort | uniq -c)   # bounded: nowhd.c, cpu_native.c are allowed sources
OPAT='^(chipset|whd|harness|cpu|m68k|softfloat)_[0-9a-f]{6}\.o$'; [ $KI = 1 ] && OPAT='^(chipset|whd|harness|cpu)_[0-9a-f]{6}\.o$'   # G2E10 kickinterp
bad=$(ls "$O"/obj_nw/ | grep -E "$OPAT"); [ -n "$bad" ] && { echo "  FAIL forbidden object in obj_nw: $bad"; fail=1; }
if [ -n "$h" ]; then echo "  FAIL: $h"; fail=1; else echo "  PASS"; fi
echo "[5] PE imports"
dl=$($OD -p "$EXE" | awk '/DLL Name:/{print tolower($3)}' | sort -u); echo "  $(echo $dl)"
ALLOW='^(kernel32|user32|gdi32|winmm|imm32|ole32|oleaut32|shell32|version|setupapi|advapi32|msvcrt|cfgmgr32|ucrtbase|uuid|api-ms-win-[a-z0-9-]*)\.dll$'
h=$(echo "$dl" | grep -Eiv "$ALLOW"); if [ -n "$h" ]; then echo "  FAIL non-system DLL: $h"; fail=1; else echo "  PASS"; fi
[ $fail = 0 ] && echo "SYMCHECK_G4 PASS $EXE" || echo "SYMCHECK_G4 FAIL $EXE"
exit $fail
