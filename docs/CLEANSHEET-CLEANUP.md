# Clean-sheet G2 cleanup (Cowork s2, 25 Oct 2026)

Karl, step 2 of 5: "clean up the G2 and commit the changes". No G2 runs or campaigns were started; wiring into Campaign Epochs is step 3.
Backups: /srv/fast/cleansheet/backups/s2-cleanup-20261025-083846/ (kit-intent .git copy, status and full diff before, CURRENT_KIT_REV, lowgate-state.json).

## Live components (read only, no schedule changed)
| Component | What runs | How started |
|---|---|---|
| cs_night_runner.py | hourly observe cycle (DB snapshot, C13/C15/C18/C19/C22, digest) | nohup, pid 1134509 since 19 Oct 20:53, --interval 3600 |
| cs_night_watchdog.py | stall watchdog, pages via fleet-alert | nohup, pid 2691188 since 20 Oct 01:51 |
| cs_lowgate_watch.sh -> cs_lowgate_refresh.py --watch-step | every 10 min; starts cs_queue.py when CURRENT_KIT_REV changes | nohup, pid 485212 since 21 Oct 22:05 |
| cs_queue.py (+ cs_fallback.py, cs_postrun_finalize.py, cs_run_identity_stamp.py, cs_kitfix.py, tools/cs_game.sh, judge.py) | one clean-sheet run per game, writes cleansheet_runs | on demand (queue, lowgate watcher); none running at cleanup |
| cs_gates.py | records CS-G2/CS-G3 into amigo.db cs_gates | on demand |
| amigo-golden.timer -> amigo-golden.service | hourly golden replays when the port kit changed | systemd (system) |
| amigo-web, amigo-worker, amigo-precook | Ami-Go app, G0/G1 worker, pre-analysis | systemd --user / system |
No cron entries. The dev kit the clean-sheet path builds from is /srv/fast/amigo/claude-run/intent/kit-intent (CS_DEV = kit-intent/amigo).

## What was dirty
kit_rev = <kit-intent HEAD>+dirty-<sha of git status>+dev-<content hash of the dev kit + cs_kitfix.py>+cs-<tools tree + judge.py>.
"dirty-11d4fd" was 13 modified tracked files and 66 untracked files in kit-intent (last commit 2009582, 15 Oct). "dev-" is not a
dirt marker: it is always present (content hash). All 37 CS-G2 passes were recorded at 2009582+dirty-11d4fd+dev-23cb2889cf69+cs-e0b7ef25374e.

## Kept (committed as live; each is a deliberate, deployed fix recorded in LESSONS.md / C19-INSTALL-NOTES.txt)
amigo_port.sh, harness.c, m68k2c.py, main.c, menu.c, oshle/oshle.c, runtime/{boot.c, chipset.c, cpu_native.c, intent.c, intentrt.c, run.h},
and the new files they include: amigo_img.h, amigo_ui.h, amigo_font.h, reccap.c, runtime/intentblit.inc, stb_image_write.h, stb_truetype.h,
NOTICE-frontend.txt, plus oshle/trace_analyze.py (helper, not built).
Untracked from the index only (files kept on disk): 4 __pycache__/*.pyc. Excluded in kit-intent/.git/info/exclude: __pycache__, *.pyc,
*.bak*, *.old-*, port.old-*, out/, build/ (the 52 backup files stay where they are, so every rollback path in the notes still works).

## Moved aside (never deleted): /srv/fast/cleansheet/aside/20261025/ (sha256 in MOVED.sha256)
- cs_night_runner.py.c19r-merged-unverified-1849 (staged merge, never verified; live runner unchanged)
- cs_night_runner.py.copilot-48e7e90f, cs_night_watchdog.py.gemini, cs_evidence_verdict.py.copilot-c872a247,
  cs_run_identity.py.copilot-01b9aef5, cs_run_identity_integrate.py.copilot-7a776af3 (verbatim model originals, not live;
  cs_night_watchdog.py's header still names the .gemini copy: it is now in aside/20261025/)
Other *.bak-* files were left in place (rollback commands in C19-INSTALL-NOTES.txt and the deploy scripts point at them).

## Commits and tags
- kit-intent: 54e95d9 "Clean-sheet G2 kit: commit the live dev-kit fixes deployed 16-21 Oct", annotated tag cs-g2-clean-1. Status clean.
- New pipeline repo /srv/fast/cleansheet.git (git dir outside the tree, like kit.git; exclude for ws/, reports/, locks/, backups/, aside/,
  logs, *.bak*, __pycache__): 17be230 first commit (84 files), e8ae8c5 CURRENT_KIT_REV, tag cs-g2-clean-1.
- CURRENT_KIT_REV: 54e95d9+dev-23cb2889cf69+cs-e0b7ef25374e (was 2009582+dirty-11d4fd+...). The dev and cs hashes are UNCHANGED,
  so the new rev is content-identical to the rev all 37 passes were recorded at. The low-gate watcher was marked handled for the new
  rev at the same moment (reports/lowgate-state.json), so no refresh campaign starts. The watchdog will count games without a row at
  the new rev; with the last queue log ending "done" that is a status line, not a page (its rule F12).

## Smoke test (scratch, not recorded: /srv/fast/tmp/s2-smoke, CS_WS override, local container, no cleansheet_runs or cs_gates write)
| Game | class | picture (% ticks >= 0.90, mean SSIM) | active | sound | CS-G2 |
|---|---|---|---|---|---|
| 241 Awesome | pass:picture+sound | 100.0, 1.0000 | 138/600 | 0.969 | PASS |
| 77 Blood Money | pass:picture+sound | 100.0, 1.0000 | 178/600 | 0.997 | PASS |
| 58 Xenon 2 (2 disk) | pass:picture+sound | 100.0, 1.0000 | 137/600 | 0.962 | PASS |
All three equal their recorded passes number for number, at kit_rev 54e95d9+dev-23cb2889cf69+cs-e0b7ef25374e.
Checked by eye: 241 reference vs port at frames 50/200/400/600 (out/cs_shots, sheet /srv/fast/tmp/s2-smoke/sheet-241.png): same starfield
intro and ship. Small difference at frame 400: the ship is a few pixels apart between the two shots, while the judge scores every tick 1.0000.

## UI (display only, same switch settings.show_gates_max, absent = 1)
amigo_app.py (backup amigo_app.py.bak-cowork-s2hide-20261025-083846): /metrics drops amigo_games_at_gate and
amigo_current_gate_passed/failed for gates above N, and recounts games_with_pass when it can; /cleansheet shows a "results hidden" page
when N < 2; /api/launcher returns no previews (G2/G3) when N < 2 and no finished games (G4/G5) when N < 4, with hidden_above.
static/launcher.html (backup .bak-cowork-s2hide-...) shows a one-line notice. amigo-web (user unit) restarted 08:40:46.
Measured: /metrics shows G0/G1 only; /cleansheet the notice; /api/launcher games 0, previews 0, hidden_above 1.
Found, not caused by this change: /api/launcher took 38.7 s while the smoke runs were busy (the per-project _built() loop).
