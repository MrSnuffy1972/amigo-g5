# CLAUDE.md: Ami-Go G5 toolkit repo

## What Ami-Go is
Ami-Go is Karl DeWet's pipeline that turns Amiga games he owns into **native** Windows (.exe) and Android (.apk)
ports. Games are reverse-engineered: the 68000 code is recovered, translated statically to C (`kit/amigo/port/
m68k2c.py`), and run on a clean-sheet runtime that does what the game needs rather than emulating the custom chips.
Real games, ROMs and the judging harness live on Karl's home server (called Ryzen in the docs). **This repo holds
toolkit source only.** You will never see a game here, and you must never add one.

## The gates (what "passing" means)
A gate passes only when a script says so, never on a model's claim. Results are judged against a reference run of
the original game.
| Gate | Passes when |
|---|---|
| G0 | The game's code is recovered (memory image, entry point, code regions) and the pre-port reaches a real game screen. |
| G1 | The whole reachable program is translated to C and the translated build matches the original on a real picture. |
| G2 | The clean-sheet native build's picture matches the reference: SSIM >= 0.90 on >= 95% of ticks with a moving reference (`cleansheet/tools/judge.py`). |
| G3 | G2 plus sound: mean spectral correlation >= 0.85. |
| G4 | A Windows .exe of the same native code, with **no emulator code inside** (`kit/amigo/port/symcheck_g4.sh`), passes the G3 check under wine. 29 of 32 games passed G4 Epoch 1. See docs/G4-DESIGN.md. |
| G5 | **This repo's job**: the Rust cold-boot port (below). |
Older notes that call G4 "polish" and G5 "Android" (the 13 Oct scoring table, `kit/amigo/g5check.py`) are superseded
by the table above.

## G5: goal and boundaries
Goal (Karl, 25 Oct 2026, "Copilot plan as G5"): a **Rust** port that **cold-boots** a game from its own files and
entry point, with no snapshot from a reference emulator run, using the existing **C translation as the oracle**.
Pilot: one small filed game that already passes G3 (likely game 234). Read docs/G5-NOTES.md first: it separates what
Karl decided from what is assumed, and the Copilot design document is not yet in the repo.

In scope: a Rust crate under `rust/`; Rust code generation from the translator's decoded instructions (a new
emitter beside the C one, sharing the decoder); clean-sheet Rust versions of the runtime pieces (guest memory, OS
services for filed games, copper/blitter/bitplane/Paula *functions*); differential tests against the C path;
CI. Out of scope unless a milestone says so: changing gate thresholds or judge.py, the campaign scheduler, the web
UI, Android packaging, polish (menus, controller mapping, saves).

Hard boundaries:
- **No emulated hardware.** No cycle-stepped chip models, no beam emulation, no general 68000 interpreter in the
  product. Karl: copy "the functionality required to do what the game needs to do", not what the chip does.
- **No raw-pointer guest memory.** Gemini's raw-pointer Rust design was rejected as CPU-shaped. Guest memory is an
  owned, bounds-checked structure with explicit big-endian accessors. `unsafe` only inside a module named `ffi`,
  each block with a `// SAFETY:` comment.
- **ROM interpreter exception (Karl, 27 Oct 2026):** only for WHDLoad kickemu games, only to run the player's own
  Kickstart loaded at run time, marked `kick_interp_only`, stopping before the first instruction outside the ROM.
  Not needed for the filed pilot; do not build it unless a milestone asks.

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
  the old Android check, not this G5).
- `cleansheet/tools/`: the clean-sheet G2/G3 job (`cs_game.sh`), `judge.py`, alignment, snapshot config, bisect.
- `g4/`: the G4 Windows build and wine judging script. `docs/`: G4 design, native-runtime design, lessons.
- `tools/leakcheck.py`, `tools/leak-hashes.txt`, `tools/leak-allow.txt`, `tools/export.sh`. `tests/synthetic/`.

## Build and test in the cloud (no games needed)
```
python3 tools/leakcheck.py --hashes tools/leak-hashes.txt --allow tools/leak-allow.txt .
python3 -m unittest discover -s tests/synthetic -v
cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets
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
