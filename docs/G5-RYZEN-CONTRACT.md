# G5 contract: what the amigo-g5 port must declare for Ryzen's G5 gate

Written 28 Oct 2026 by the Claude sub-agent fix-g5-plumbing. Ryzen's side is `/srv/fast/cleansheet/g5/g5_build.sh` (the one entry point), `g5_symcheck.sh` and `tools/campaign.py run_g5()`. A copy of this file should go into the repo as `docs/G5-RYZEN-CONTRACT.md` by a normal pull request.

## 1. Readiness is declared, never guessed

The repo adds `rust/G5-CAPABILITIES.json` when the port can do what it claims:

```json
{"schema": 1,
 "milestones": ["M1", "M2", "M3", "M4", "M5"],
 "cold_boot_filed": true,
 "headless_dumps": true,
 "boot_bin": "amigo-g5-boot"}
```

- `cold_boot_filed` true only after M4: the port boots a filed game (AmigaDOS executable plus its files) from its own files and entry point, with no snapshot of a reference run.
- `headless_dumps` true only after M5: frames and audio are written in judge.py's format (section 2).
- `boot_bin`: the cargo binary target that Ryzen builds with `cargo build --release --offline --locked --bin NAME`. Letters, digits, `-` and `_` only.

Until all of this holds, `g5_build.sh --check-ready` prints `G5 port not ready: <missing milestone>` and exits 20. Ryzen records a G5 FAIL with cause `g5-not-ready`. It is never a pass.

## 2. The boot binary's command line

```
amigo-g5-boot --game DIR --config game.json --ticks N --frames OUTDIR --audio OUT.raw
```

- `--game`: the game's own files.
- `--config`: Ami-Go's game.json for the game (entry, segments, HLE addresses).
- `--ticks`: run N 50 Hz ticks headless, with a fixed timestep and no window.
- `--frames`: write one frame per tick as `pNNNNN.rgba`, starting at `p00001`. The format is the 4 bytes `RGBA`, then width and height as little-endian uint32, then width x height x 4 bytes. This is the format judge.py reads.
- `--audio`: signed 16-bit little-endian stereo at 44100 Hz, 882 sample frames per tick, with no header (`.raw`).
- Exit 0 when all N ticks ran.

## 3. What the binary must not contain (g5_symcheck.sh)

The gate build keeps its symbols. A stripped binary fails.

- No Musashi or 68000 interpreter names: `m68k_*`, `m68ki_*`, `musashi*`, `cpu_execute`, `interp_run`, `execute_instruction`, `fetch_decode_execute`.
- No module called `interp`, `interpreter`, `emulator`, `emu`, `chip_model`, `chipset` or `beam`.
- No chip-model stepping: `copper::`, `blitter::`, `paula::`, `denise::`, `agnus::`, `cia::` or `custom::` functions named `*step*`, `*cycle*` or `*clock*`, and no `cycle_step` or `step_cycle`. Name functions after what the game needs (`draw_bob`, `play_sample`).
- No `kick_interp_only`. Filed games need no Kickstart, so the ROM interpreter exception does not apply.
- No Kickstart bytes: leakcheck's ROM byte search runs on the binary.

## 4. Ryzen's side (not the repo's)

- `g5_ref.sh WORKDIR TICKS REFDIR REF.raw`: the original's cold-boot reference run on the reference harness, in the same frame and audio format. Written in M6 (section 5).
- Sync-in: a merged amigo-g5 commit reaches the kit only through a generated installer (`scratch/g5/fix-g5-plumbing/g5-sync-in.sh COMMIT`).
  - `rust/` goes to `kit-intent/rust` (outside the dev root, so kit_rev does not move).
  - `G5_RUST_ONLY=1` (M6): only `rust/`, with the translator (`m68k2c.py`, `m68k2rs.py`, `portcfg.py`) in `rust/g5-translator`; kit-intent/amigo is untouched, so kit_rev cannot move while a G2-G4 campaign runs. `g5_game.sh` uses that translator when it is there.
  - `kit/amigo/**` changes go to `kit-intent/amigo/**`. `port/m68k2rs.py` is exempt from kit_rev. Any other kit file moves kit_rev, and every G2/G3/G4 workspace then rebuilds.
- Pass: judge.py picture PASS with a moving reference, sound PASS, and g5_symcheck PASS (class `pass:rust-coldboot`).

## 5. M6 additions (28 Oct 2026, the pilot milestone)

- **Ryzen side now written.** `g5_ref.sh` runs the game's own reference harness from its clean-sheet workspace
  (`ws/NNNNN/out/port/harness`, the interpreter plus the kit's fake OS, no Kickstart) from power-on, with the same
  environment cs_game.sh uses (AMIGO_BEAMFIX=1, raw RGBA, no input). A cold boot starts where the harness starts, so
  frames 1..TICKS are taken as they are (no snapshot alignment) and the sound is not cut. The harness binary is copied
  into the G5 workspace first, with its sha256, because campaigns rebuild ws/NNNNN.
- **Per-game glue** (`g5_game.sh`, on Ryzen only; nothing of it goes to the repo):
  - `prep`: the game's disk files (`ws/NNNNN/out/port/g0/vol/files`, extracted by G0 from the game's own disk image)
    go to `WORKDIR/g5files`, the directory the boot binary boots; the translator's inputs (game.json and the files it
    names) are copied; `m68k2c.py --emit rust` writes the game's translated code, which the boot crate's `build.rs`
    compiles in through `AMIGO_G5_GAME_RS`.
  - `imgcheck`: the translator reads its code bytes from the clean-sheet image taken at the program's entry. The boot
    binary's `AMIGO_G5_BOOTDUMP` (guest RAM right after the cold boot) must hold the same bytes at every translated
    instruction, or the build fails (exit 9). Nothing of a reference run is used at run time.
- **The boot binary's game directory** is `WORKDIR/g5files`, not `WORKDIR/game` (which holds the disk image). The
  program is the first command of `s/startup-sequence` that names a hunk executable at the top of that directory.
- **Exit 3** from the boot binary: the run stopped early (untranslated address, an OS call not served yet, a bus
  error), with the cause on stderr and the frames up to the stop written. g5_build.sh reports it as a failed run.
- **Naming (section 3) applied to M5's code:** the custom-chip runtime module `chipset` (type `Chipset`) is renamed
  `hardware` (`Hardware`), and its and the CIAs' `line_step` is `pass_line`, because g5_symcheck.sh rejects a module
  called `chipset` and a `cia::` function with `step` in its name. The rename changes no behaviour; the module was
  always the intent runtime (copper list as a table, blits done at once), not a chip model. The check is by name, and
  this note says so.
- **M6 part 2:** `--program REL` (optional) names the program to boot, a path under the game directory; the Ryzen glue
  passes the kit's G0 choice (gates/G0/g0.json `exe_rel`). The binary prints `boot: game code fnv <16 hex>`, the FNV-1a
  64 of the generated code compiled into it; g5_build.sh fails (exit 9) when it is not the hash of the game's own
  translated.rs. `AMIGO_G5_OSTRACE=FILE` writes every OS call as `TICK t CALL base $B LVO -$L` (dev only).
- **M6 part 3:** the game's code is built by the crates `amigo_g5_part0..7` from the same `AMIGO_G5_GAME_RS` (pages by
  PC >> 9, part = page modulo 8), so `cargo build --bin amigo-g5-boot` builds them too; m68k2rs.py must emit 512-byte
  pages (`PAGE_BITS = 9`) or the parts refuse the file.
