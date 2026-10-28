# Clean-sheet G4: design (Cowork, 25 Oct 2026) - NOT wired into campaign.py

Karl's definition (25 Oct): **G4 = a Windows .exe of the native (clean-sheet) code, with NO emulator code inside, that passes the
same picture and sound check as G2/G3 when run under wine.** Polish (start menu, controller, saves) is later, not G4.

Everything here lives in /srv/fast/amigo/scratch/g4 (a copy). The live kit, kit-intent, /srv/fast/cleansheet and campaign 37 were
not changed. Results of the prototype run: see "Prototype results" at the end (filled from logs/run_all.log).

## 1. The G4 rule (decided by code only)
A game passes CS-G4 when, for ONE build at the clean-sheet kit_rev it passed CS-G3 at:
1. `amigo_port.sh native_windows` builds `out/native_win/game.exe` from the SAME sources as `out/native` (gen/*.c, boot/*.c,
   intentrt.c, boot.c, nowhd.c, nativefs.c, cpu_native.c, main.c, game hooks), -O1, mingw-w64, SDL2 static (video, audio, input).
2. `symcheck_g4.sh` passes on `game.unstripped.exe` (5 checks: kit symcheck.sh patterns; Musashi/softfloat/harness/resload names;
   object proof - no global symbol that only the reference build's harness.c/chipset.c/whd.c/Musashi/cpu.c objects define; no
   emulator source-file names in strings; PE imports are Windows system DLLs only). Negative control: the same script FAILS
   out/nativeprobe (Musashi linked), proving it can see emulator code.
3. Under wine + xvfb, `game.exe --frames 600 --dump --wav --script` (AMIGO_NOMENU=1, AMIGO_SAVE_PROFILE=0, SDL_AUDIODRIVER=dummy,
   AMIGO_IMG=rgba) runs every tick (rc 0, 600 frames), and judge.py against the harness reference (same ref2 + cs_align.py as
   cs_game.sh, same snapshot frame, same input script) gives the CS-G3 result: picture SSIM >= 0.90 on >= 95% of ticks with a
   moving reference (active >= max(5, ticks/20)), sound mean spectral correlation >= 0.85.
4. Recorded alongside (not a pass condition yet): byte-identity of wine frames and audio with out/native's.

## 2. How a `--gate 4` epoch would run
- **Scope**: active games that are CS-G3 passed now (gate_results gate 3 pass at the current clean-sheet rev, the sticky G3 rule)
  and not G4 passed now. G4 Epoch 1 = `--all-in-scope` baseline.
- **Item** (`run_g4(pid)` beside run_g2 in tools/campaign.py, called from amigo_preport.loop() like G2):
  1. Find the game's clean-sheet workspace at the current kit_rev (/srv/fast/cleansheet/ws/NNNNN with cs_kit_rev == rev and a
     CS-G3 pass in result.json). If absent or stale, call cs_queue.one() first (a fresh G2/G3 run; its rows land as usual).
  2. Copy the workspace to /srv/fast/cleansheet/ws_g4/NNNNN (never build in the G2 workspace), run `g4_inner.sh` in amigo-tools:3
     through amigo_cpu (same CPU worker pool, same 5400 s cap; prototype: about 2-3 min a game on ryzen at nice 15, 2 CPUs).
  3. Parse gates/G4/: build_rc, 2-symcheck.log, wine_rc, 5-judge-wine.log, 6-wine-vs-linux.txt -> gates/G4/result.json.
  4. Write ONE gate_results row (gate 4, source 'cleansheet-g4', kit_hash = clean-sheet kit_rev, cause = first failing step:
     build / symcheck / wine-run / picture / sound), then run cs_gates.py so cs_gates gets a **CS-G4** mirror row (history kept,
     'lost' on a later fail, as CS-G2/CS-G3).
- **CPU share**: settings `campaign_g4_every` = 6 (G4 items take at most 1 slot in 6; unused slots go to the other streams),
  added to the same candidates_hook() interleave as campaign_g2_every / campaign_g3_every.
- **Regression set**: every 4th G4 epoch (G4_CYCLE_START = 1, so Epochs 4, 8, 12 ...) adds the REG_N = 25 G4-passing games with
  the fewest G4 passes (gate_passes gate 4), role 'regression', as the G1/G3 rule.
- **Epoch rules** as Karl's G0-G3 policy: targets first, diagnosis starts when the targets have run, next epoch prepared early,
  excluded games stay excluded, show_gates_max raised to 4 only at the first numbered G4 epoch.
- **Later (not G4)**: delivery of passing game.exe (stripped) to F:/Game Source/Amiga/Windows (amigo_worker.py:905 path), then
  polish (start menu, controller, saves) as G5 work.

## 3. Exact install steps (for when Karl says go; nothing below has been applied)
1. Backups (hash-checked, outside the served tree): kit-intent amigo_port.sh, /srv/fast/cleansheet/cs_gates.py, tools/campaign.py.
2. kit-intent: add `do_native_windows()` (scratch/g4/src/native_windows.fn) before `do_nativeprobe()` in amigo/port/amigo_port.sh
   (the `native_windows) do_native_windows ;;` case already exists at line 196 with no function behind it), add
   amigo/port/symcheck_g4.sh, and copy the live kit's amigo/port/sdl2-mingw (SDL2 2.30.x mingw static libs; not in kit-intent).
   Commit and tag `cs-g4-1`. NOTE: this changes the dev-kit content hash, so CURRENT_KIT_REV changes and the low-gate watcher
   would start a refresh run: mark the new rev handled in reports/lowgate-state.json at the same moment (as the s2 cleanup did),
   or exclude sdl2-mingw/symcheck_g4.sh/the new function from the dev hash so G2/G3 results stay valid.
3. /srv/fast/cleansheet/tools/: add g4_inner.sh (scratch/g4/src/g4_inner.sh) and a cs_g4_summary.py that writes result.json.
4. cs_gates.py: add CS-G4 from the latest cs_g4_runs row (new table cs_g4_runs, same shape as cleansheet_runs).
5. campaign.py: G4_EVERY = 6, G4_CYCLE_START = 1, run_g4(), `--gate 4` in start() scope and is_g4_regression_epoch(),
   candidates_hook() share, settings default campaign_g4_every = 6.
6. Smoke: `campaign.py start --smoke --gate 4 --targets 241,234,266 --change "G4 smoke"`, compare with this prototype, then G4 Epoch 1.
7. Restart amigo-web only if the UI shows G4 (show_gates_max).

## 4. Prototype results (25 Oct 2026, scratch/g4, amigo-tools:3, --cpus 2, nice 15)
Games: CS-G3 passes in G2 Epoch 1 (campaign 37 gate_results gate 3 pass): 241, 234, 266. Workspaces copied from
/srv/fast/cleansheet/ws (kit_rev 54e95d9+dev-23cb2889cf69+cs-e0b7ef25374e). Log: logs/run_all.log; per game ws/NNNNN/gates/G4/.

| Game | build | symcheck_g4 | wine run | picture (% ticks >= 0.90, mean SSIM) | active | sound | wine vs out/native |
|---|---|---|---|---|---|---|---|
| 241 | ok, 5.75 MB | PASS | rc 0, 600 frames | 100.0%, 1.0000 | 138/600 | 0.969 | 600/600 frames and audio byte-identical |
| 234 | ok, 4.36 MB | PASS | rc 0, 600 frames | 100.0%, 1.0000 | 49/600 | 0.997 | 600/600 frames and audio byte-identical |
| 266 | ok, 5.01 MB | PASS | rc 0, 600 frames | 100.0%, 1.0000 | 570/600 | 0.998 | 600/600 frames and audio byte-identical |
Control: out/native rerun in the same container reproduced each game's recorded CS-G3 numbers exactly; symcheck_g4 on
out/nativeprobe (Musashi linked) FAILS on checks 2, 3 and 4, so the check can see emulator code. PE imports of every game.exe:
advapi32 gdi32 imm32 kernel32 msvcrt ole32 oleaut32 setupapi shell32 user32 version winmm (no SDL2.dll: static).
By eye: 241 frame 400 wine vs reference = same starfield and ship, ship a few pixels apart while the judge scores 1.0000 (the
same observation CLEANUP.md made for out/native; a G2 judge or alignment question, not a G4 one, since wine equals out/native
byte for byte). 266 frame 400 = identical "HeroQuest protection / please press the space bar" screen.

## 5. Findings and what is left before G4 epochs
- `native_windows` in amigo_port.sh had NO function behind it (case at line 196 only). Written: src/native_windows.fn.
- kit-intent has no mingw SDL2; the live kit's amigo/port/sdl2-mingw was copied (read only). Install step 2 must bring it.
- First symcheck_g4 draft had two false positives (interp_pcs = --stats counter in cpu_native.c; "whd.c" inside "nowhd.c");
  patterns tightened, recorded here so the reason is not lost.
- Wine: the prefix needs an owned parent dir (WINEPREFIX=/tmp/wp failed until mkdir). wineboot costs about 2 min a game; a
  prebuilt prefix (tar in the image or a cached volume) would cut a G4 item to about 3 min.
- Under xvfb the exe runs unpaced (600 ticks in 2 s): game logic is per tick, so the measure is unaffected; real-time pacing,
  audio device output and the start menu are polish, not G4.
- Not yet done: install steps 2-7, cs_g4_runs table + CS-G4 in cs_gates.py, run_g4() in campaign.py, kit_rev handling so the
  kit change does not trigger a low-gate refresh or invalidate G2/G3, and a smoke epoch on more games (only 3 tested; all three
  are attract-only, none with an input script, so the scripted-input path under wine is unmeasured).

## 6. Decisions recorded 25 Oct 2026 (via coordinator, Karl approved the install)
- **G4 = a Windows .exe of the native (clean-sheet) code, with no emulator code inside, that passes the G2/G3 picture and sound
  check under wine.** Pass class: `pass:native-windows`; failures `fail:<step>` (build, symcheck, wine-run, picture,
  static-reference, sound).
- **G5 (planned)**: Rust cold boot (the Copilot design), piloted later on one small G3 game, likely 234. Gemini's raw-pointer
  Rust design was rejected as CPU-shaped.
- Install design changes since section 3 (prepared, NOT applied - see section 7):
  - amigo_port.sh stays byte-unchanged: do_native_windows lives in kit/amigo/port/native_windows.fn and is loaded with
    `BASH_ENV=$K/native_windows.fn bash amigo_port.sh native_windows`. The old Musashi `windows` target is untouched.
  - Kit-rev guard: cs_content_id.G4_EXEMPT = {port/native_windows.fn, port/symcheck_g4.sh} (dev tree) and cs_queue.kit_rev uses the
    newest commit touching anything but those two files (`git log -1 --format=%h -- . ':(exclude)...'`), ignoring their status
    lines, so the cs-g4-1 commit leaves kit_rev = 54e95d9+dev-23cb2889cf69+cs-e0b7ef25374e. campaign.py reloads cs_content_id and
    cs_queue when amigo_preport reloads it, so no service restart is needed. reports/lowgate.stop is present (watcher stopped)
    and lowgate-state last_rev equals that rev, so it would be idle either way.
  - G4 scope: latest clean-sheet G3 result is a pass (amigo_g0now.g3_now pass OR once) and G2 passed now; strict sticky G3
    (two passes) would leave G4 nearly empty after one G3 epoch. Karl to confirm.
  - G4 runs on Ryzen by local docker (2 CPUs, nice 15) from ws_g4/NNNNN, not amigo_cpu yet; tools in /srv/fast/cleansheet/g4
    (outside the hashed tools tree): g4_inner.sh, sdl2-mingw, wineprefix.tgz.
  - G4 items ride the G2 dispatch (G2_IDS; run_g2 hands G4_IDS to run_g4), so amigo_preport.py is unchanged.

## 7. Install status (25 Oct 2026, ~13:05)
BLOCKED before any live change: the first install command (backups, cs_content_id.py / cs_queue.py patch, g4 tool dir) was
refused by the Claude Code auto-mode safety classifier ("Modify Shared Resources"). Nothing ran: no backup, no patch, no kit
commit, no campaign started. Prepared and waiting in /srv/fast/amigo/scratch/g4/src on approval (copied from WinPC
F:\openclaw\tmp\g4b-*): campaign.py (G4 edits), g4_inner2.sh (installed name g4_inner.sh), native_windows.fn, symcheck_g4.sh,
and the install script g4b-inst1.sh.

## G4 Epoch 1 result (campaign 51, 25 Oct 2026)
29 of 32 pass (Windows .exe under wine matches G3): 5, 20, 23, 32, 37, 50, 52, 58, 77, 83, 99, 101, 106, 116, 117, 119, 122, 131, 155, 157, 188, 234, 241, 254, 265, 266, 268, 280, 318. Fail: 48, 51, 196 (to diagnose).

## Kickemu games: Karl's G5 release-rule exception and the Kickstart mount (27 Oct 2026)
Karl's rulings, 27 Oct 2026 (fix-g2e10-kick installed with KICKINTERP=1; companion fix-g4-kick):
1. **Exception to the G5 release proof (no 68000 interpreter in a shipped exe), for WHDLoad kickemu games only.** Their native and
   Windows exes link Musashi ONLY to run the player's own Kickstart ROM, read at run time (Kickstarts folder next to the exe, or
   AMIGO_KICKSTART). The interpreter stops before the first instruction outside the ROM, so every game instruction stays translated,
   and an untranslated game address is still a finding. No Kickstart byte ships: the exe carries no ROM, no ROM-derived code and no
   RTB table (kcheck byte search on every built exe and every G4 output). symcheck.sh / symcheck_g4.sh accept Musashi only in an exe
   that carries the marker kick_interp_only; interp_run and every chip-model symbol stay forbidden for all games.
2. **G4 for kickemu games mounts the Kickstart ROMs read-only, on Ryzen only.** campaign.run_g4 (local docker on Ryzen) adds
   tools/kickroms and tools/whdload/rtb read-only at the paths bin/gt uses, outside /w, for kickemu games only (same G0-evidence test
   as the G2 pin); g4_inner.sh hands the wine exe those folders (AMIGO_KICKSTART / WHD_RTBDIR, Z: paths) for that one command. Nothing
   is copied into the G4 workspace or any output. A kickemu G4 run on a host without the ROMs refuses.
