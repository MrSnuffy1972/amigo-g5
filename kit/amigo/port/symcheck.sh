#!/bin/bash
# symcheck.sh BINARY...: G5 release proof (Karl 23:58, Gemini review item adopted): a shipped exe or libmain.so must contain no
# 68000 interpreter and no chip-model symbols. Exit 0 = clean, 1 = forbidden symbols found (listed), 2 = no nm tool.
# Forbidden: m68k_* (Musashi), interp_run, and the chipset.c entry points copper_run_until, render_line, cia_tick, do_blit, chip_end_line, blit_end.
NM=""
for t in $(ls /opt/android/ndk*/toolchains/llvm/prebuilt/*/bin/llvm-nm 2>/dev/null | head -1) x86_64-w64-mingw32-nm llvm-nm nm; do [ -n "$t" ] && command -v $t >/dev/null 2>&1 && { NM=$t; break; }; done      # the NDK llvm-nm reads every ABI (arm64 libmain.so)
[ -n "$NM" ] || { echo "symcheck: no nm tool"; exit 2; }
PAT='(^|[ _])(m68k_[a-z_0-9]*|interp_run|copper_run_until|render_line|cia_tick|do_blit|chip_end_line|blit_end)$'
bad=0
for f in "$@"; do
  if $NM "$f" 2>&1 | grep -q "no symbols"; then echo "SYMCHECK FAIL $f: stripped, nothing to check (run it on the unstripped build, out/game.unstripped.exe)"; bad=1; continue; fi
  hits=$($NM "$f" 2>/dev/null | awk '{print $NF}' | grep -E "$PAT" | sort -u); kmsg=""
  if $NM "$f" 2>/dev/null | awk '{print $NF}' | grep -qE '^_?kick_interp_only$'; then   # G2E10 kickinterp: Musashi only for the player's Kickstart ROM
    hits=$(echo "$hits" | grep -Ev '(^|_)m68k_[a-z_0-9]*$'); kmsg=" (kick_interp_only: Musashi linked only to run the player's own Kickstart ROM; m68k_* allowed)"; fi
  if [ -n "$hits" ]; then echo "SYMCHECK FAIL $f: $(echo "$hits" | wc -l) forbidden symbols, e.g. $(echo "$hits" | head -5 | tr '\n' ' ')"; bad=1
  else echo "SYMCHECK PASS $f: no m68k_*, interp_run or chip-model symbols$kmsg"; fi
done
exit $bad
