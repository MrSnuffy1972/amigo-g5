# CLAUDE.md: Ami-Go G5 toolkit repo

## What Ami-Go is
Ami-Go is Karl DeWet's process for turning Amiga games he owns into **native** programs: idiomatic Rust on a shared,
system-neutral engine (`rust/native_engine`) with a per-system front end, released as a Windows .exe and an Android
.apk, so the same process can later cover other consoles. It is **not an emulator**. Real games, ROMs, recovered source
and the judging harness live on Karl's home server (called Ryzen in the docs). **This repo holds toolkit and native
source only.** You will never see a game's bytes here, and you must never add them.

## The process (Karl, 31 Oct to 3 Nov 2026)
1. **Recover 100% of the code**, never skipped or partial, tracked by the **R gate**. R is green only when all four
   proofs hold: (1) every byte classified; (2) byte-identical reassembly with vasm; (3) every function named and
   described; (4) every executed PC covered. Ryzen stores the result in `amigo.db` table `recovery_gate` (script
   `/srv/fast/amigo/amigo_rgate.py`). Method: `docs/native/recovery-method.md`.
2. Understand the game from the recovered code: loop, rules, objects, timing, state and assets.
3. Rewrite it natively on the shared engine.
4. Verify against the game's golden set, then release.

**R first:** a G gate counts only results produced after the game's R pass, and Ryzen's schedulers
(`tools/campaign.py` `rfirst_tick` and the rest) only run G work on R-green games.

## The gates (what "passing" means)
A gate passes only when a script says so, never on a model's claim.
| Gate | Passes when |
|---|---|
| R | 100% code recovery: all four proofs above pass (`amigo_rgate.py`). |
| G0 | Boot / first-screen match in the clean-sheet harness. |
| G1 | Second-stage screen match (CPU). |
| G2 | Picture. |
| G3 | Sound. |
| G4 | The released native Rust Windows .exe passes picture and sound against the game's golden set (run under Wine on Ryzen, `native/tools/g4_native.sh`). |
| G5 | The released native Rust Android .apk passes picture and sound against the golden set (Android emulator on IntelPC, being built: `native/tools/g5_native.sh`). |
| G6 | Shipped: Karl has confirmed the release on his phone (`amigo_ngate.py ship`). |

**Legacy, history only:** the old translated-code G4 (a Windows .exe of the C translation judged under wine: `g4/`,
`symcheck_g4.sh`, docs/G4-DESIGN.md), the Rust cold-boot G5 (the frozen translated build under `rust/`, docs/G5-NOTES.md,
docs/G5-PLAYABLE.md) and the old Android check once called G5 or G6 (`g5check.py` / `g6check.py`). Their results no
longer count. Do not extend the translated build's runtime, translator, ports or chips; it is kept only as a reference
recorder.

## Golden sets, sound rule, retention and release (Ryzen)
- Golden sets: `/srv/fast/amigo/native/golden/<id>/` with `manifest.json` and `replay.sh`, proven by replay; a game
  needs one before anything of it is pruned. Sound passes on envelope correlation >= 0.9, except games whose manifest
  says `"sound_class": "continuous"`, which pass on spectrum correlation >= 0.98 and level within 5%.
- Retention "Lean + regression": `/srv/fast/amigo/docs/retention-policy.md`. `tools/retention_prune.py` is a dry run by
  default; `--apply` moves files to `_to_prune` and never deletes; it refuses any game that is not fully green.
- Releases: file names `<id>-<slug>-r<N>` (docs/G5-NATIVE.md, "Release builds and revisions"). `native/tools/release.sh`
  also publishes to WinPC `F:\Game Source\Amiga\Native Ports` (append-only) and runs the native checks.

## What every native port must do
- Copy protection is removed in every port (recovery still documents it, so the 100% proof holds). Cracktros are kept
  as native intros. Trainers become a cheats menu, off by default.
- Not an emulator: no guest memory, no 68000 register file, no custom-chip registers and no joystick or mouse port reads
  in the native engine or a native game. Input is abstract actions.
- `unsafe` only inside a module named `ffi`, each block with a `// SAFETY:` comment.

## Who does the work
Recovery runs on Ryzen plus remote stage-A workers (IntelPC with 3 slots now; Unraid, WinPC and MiniPC being added).
Naming models: Ryzen Glimmer, IntelPC qwen3.6, M4 gemma (Leroy first); shared signature library
`/srv/fast/amigo/siglib` (never in a repository); paid fallback Haiku, capped at A$5, off-peak only. Fleet tasks go
through OpenClaw as the worker, with Claude as XO verifying.

## Karl's rules (non-negotiable)
- Australian English spelling (behaviour, colour, initialise, analyse, licence as a noun). **No em dashes** in any
  file, comment, commit or PR text: use a hyphen, a colon or a new sentence.
- **Never add ROM or game data**: no Kickstart ROM, WHDLoad RTB, game file, disk image (.adf .ipf .dms .hdf .lha),
  screenshot, frame dump or audio capture, and no byte tables copied out of any of them. Tests use synthetic
  68000 written by hand or generated in the test.
- **Kickstart ROMs never ship.** A port that needs one loads the player's own ROM at run time.
- No secrets, keys, tokens, host names or LAN addresses in code or docs.
- Measure before claiming. Say plainly what was measured and what is assumed.

## Layout
- `kit/amigo/port/`: translator `m68k2c.py`, `portcfg.py` (game.json fields), `portgen.py`, `portg0.py`, the C
  runtime in `runtime/` (`intentrt.c` and `intent.c` are the clean-sheet chip functions, `cpu_native.c` the
  translated-code dispatcher, `nativefs.c` / `nowhd.c` / `oshle/` the OS services for filed games, `kickrt.c` the
  run-time Kickstart loader), `main.c`, `harness.c` (the **reference** harness, which links Musashi; never part of a
  product), `symcheck.sh`, `symcheck_g4.sh`, `native_windows.fn`, `amigo_port.sh` (build steps).
  `musashi/` is the reference 68000 core used only by the harness and difftest. The prebuilt SDL2 mingw libraries
  are not exported (21 MB of binaries); the Windows build runs on Ryzen.
- `kit/amigo/`: `hunkload.py` (AmigaDOS hunk loader), gate checks (`g0check.py`, `g3check.py`, `g5check.py`, which is
  the legacy Android check; none of these is the current G4-G6).
- `cleansheet/tools/`: the clean-sheet G2/G3 job (`cs_game.sh`), `judge.py`, alignment, snapshot config, bisect.
- `g4/`: the legacy translated-code G4 Windows build and its wine judging script (history only). `docs/`: G4 design, native-runtime design, lessons.
- `tools/leakcheck.py`, `tools/leak-hashes.txt`, `tools/leak-allow.txt`, `tools/export.sh`. `tests/synthetic/`.

## Build and test in the cloud (no games needed)
```
python3 tools/leakcheck.py --hashes tools/leak-hashes.txt --allow tools/leak-allow.txt .
python3 -m unittest discover -s tests/synthetic -v
cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets
cd rust/amigo_g5_play && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets
```
Needs python3, gcc and a stable Rust toolchain. Anything that needs a real game, the reference harness run, wine or
the judge on real frames is **Ryzen-only**: write the test so it skips cleanly without its inputs, and say in the PR
what Ryzen should run.

## Coding conventions
- Rust: stable, edition 2021, `cargo fmt`, `clippy -D warnings`, no new dependency without a line in the PR saying
  why (expect `sdl2` later; avoid heavy frameworks). No global mutable state; deterministic fixed-timestep ticks.
  Name things after what the game needs (`draw_bob`, `play_sample`), not after chip registers, once the intent is
  known.
- Python tools: python3 standard library first; numpy and pillow exist on Ryzen. Keep the existing style.
- Every function that mirrors C behaviour names the C function it is checked against.
- Comments say why, with a date when a decision is recorded.

## Pull requests
- Small and reviewable: one milestone step per PR, ideally under ~600 changed lines of non-test code.
- Tests with every PR, passing in CI. No PR that only adds code with no test.
- The PR description has four headings: **Measured** (commands run and their real output), **Assumed** (what you
  believed but did not verify), **Not done**, **For Ryzen to run** (exact commands for the game-level check).
- Append the same summary as a dated entry to the session log in `docs/G5-NOTES.md`.
- Run `tools/leakcheck.py` before every commit. A hit is a stop: remove the file and say so in the PR.
