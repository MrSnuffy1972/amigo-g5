# G5: what is decided, what is assumed, and the session log

## Decided by Karl (dated)
- **25 Oct 2026**: "Copilot plan as G5". G5 is a **Rust cold-boot product** following Copilot's architecture, with the
  existing **C translation kept as the oracle**. Piloted on one small game that already passes G3 (G4-DESIGN.md
  section 6 names game 234, Airball v1.0, a filed game, as the likely pilot). Gemini's alternative Rust design, built
  on raw pointers into guest memory, was **rejected** as "CPU-shaped" (an emulator in Rust clothing).
- **17 Oct 2026**: every custom chip's native code is a clean-sheet rewrite on modern principles; translated code is
  an intermediate step, not the product. A Rust pilot of the runtime on one game that already passes G2.
- **14 Oct 2026**: ports are fully native with **no emulated hardware**: "copy the functionality required to do what
  the game needs to do", not what the blitter (or any chip) does. Modern fixed-timestep timing; CPU frame composition
  with SDL2 GPU presentation (D3D11 on Windows, GLES/Vulkan on Android); draw the screen the way a modern game does,
  not by emulating the beam line by line.
- **26-27 Oct 2026**: Kickstart ROMs never ship. A WHDLoad kickemu game loads the **player's own** Kickstart at run
  time (a Kickstarts folder next to the exe, or AMIGO_KICKSTART). A 68000 interpreter may be linked **only** for
  kickemu games, **only** to run the player's ROM, marked `kick_interp_only`, and it stops before the first
  instruction outside the ROM. Every game instruction stays translated. No ROM byte, ROM-derived code or RTB table
  is ever in a build or in this repo.
- Finished ports are self-contained: one Windows .exe and one Android .apk, no emulator, no separate files
  (except the player's own Kickstart for kickemu games, per the ruling above).

## The Copilot design document
The Copilot G5 design document itself was **not found** on Ryzen or WinPC when this repo was exported
(28 Oct 2026; searched /srv/fast/amigo, /srv/fast/cleansheet, /srv/bulk/amigo, F:\openclaw\copilot,
F:\openclaw\gemini, F:\openclaw\out). Until Karl adds it as docs/G5-COPILOT-DESIGN.md, the decisions above are the
whole of what is known. Where the prompt or CLAUDE.md describes the architecture in more detail than the list
above, that detail is the exporter's reading and is labelled as an assumption.

## Assumptions (the exporter's reading; check against the Copilot document when it arrives)
- "Cold boot" means the port starts the game the way the Amiga would, from the game's own files and entry point,
  with **no snapshot** of a reference emulator run. Today's clean-sheet G2/G3 native builds start from a reference
  snapshot (snapcfg/chip.bin plus restored custom-chip, CIA and Paula state at a snapshot frame; see
  `kit/amigo/port/runtime/intentrt.c` and `cleansheet/tools/cs_snapcfg.py`).
- "C translation as the oracle" means: for the same program and inputs, the Rust port's per-tick state (68000
  registers, guest RAM hash, custom-register writes, frames, audio) is compared with the C native build's, and the
  C build is presumed right where the two differ until a test against the original says otherwise.
- "Filed game" means a game installed as files (an AmigaDOS executable plus data files, served by
  `runtime/nativefs.c` / `nowhd.c`), as opposed to a trackloader disk.

## Session log (newest last; one entry per Claude Code session)
<!-- Each session appends: date, milestone, PR link, Measured, Assumed, Not done, Next. -->

### 2026-10-28: M1, Rust skeleton, guest memory and hunk loader (DONE)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 13 unit tests (mem accessors, boundaries, odd-address error,
  map validation, hunk basics) and 2 integration tests pass. The differential test builds 14 synthetic hunk files in
  the test (several hunks, RELOC32/RELOC32SHORT/RELRELOC32/RELOC16/RELOC8, BSS, chip and fast hunks, NAME/SYMBOL/DEBUG/EXT
  definitions, resident library names, and error cases) and compares the Rust loader with `hunkload.load()` under four
  layouts each (auto base, explicit `$C00000`, seglist, seglist at `$400000`): BASE, ENTRY, RELOCS, every hunk address
  and the full image hex are identical for the 8 cases that load, and the 6 error cases agree on error class.
- **Assumed**: addresses are masked to 24 bits (68000 bus); a single access must sit inside one region; the Rust
  loader reports an error where the Python indexes loosely (reloc before the first hunk, target or offset out of
  range, body longer than the declared image). No real game hunk file was loaded.
- **Not done**: M2 onwards. `HunkError::Unsupported` covers overlays and HUNK_EXT references, as the Python does.
- **For Ryzen**: nothing for M1.

### 2026-10-28: M2, Rust emitter for a first instruction subset, checked against the C translation (PARTIAL, completed below)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy
  --all-targets -- -D warnings` clean; `cargo test --all-targets`: 18 unit tests, hunk differential test, and the new
  `emit_diff` (2 tests) pass; leakcheck clean; `python3 -m unittest discover -s tests/synthetic` 6 tests pass.
  `emit_diff` translates 8 hand-encoded programs (loop from tests/synthetic, move modes, arithmetic and compares,
  shifts and rotates, bit ops/BCD/CCR/Scc, movem/link/unlk, bsr/jsr/rts, branches and dbra) with one
  `m68k2c.py ... --emit rust --single` run each, runs the `--single` C (gcc) and the generated Rust from the same
  start state to the same stop address (5 to 24 steps each), and compares D0-D7, A0-A7, SR, PC, cycle count, OSP and
  every changed RAM byte: identical for all 8. Mutation check: flipping `>` to `>=` in the Rust `sub` carry made the
  test fail on the first program.
- **Decision (28 Oct 2026)**: the Rust emitter transliterates the decoder's own C statements (`Insn.code`) with a small
  expression parser (`kit/amigo/port/m68k2rs.py`), so both outputs come from one decode. Anything it cannot translate
  (goto, loops, `/` and `%`, exceptions: traps, chk, mul/div) becomes an arm that returns `Stop::Untranslated`, and
  is counted in `translate_stats.json` (`rust_untranslated`). A structured form in the decoder is the alternative if
  the subset grows.
- **Assumed**: the `--single` C build (one function per instruction, the build `difftest.c` uses against Musashi) is the
  right oracle for M2; the chunk build with its event checks was not compared. A word or long access at an odd
  address is a typed error in Rust and silent in the C test driver, so no test program does one.
  The C test driver's memory is a flat 512 KB, so nothing here exercises custom-chip addresses.
- **Not done**: mul/div, chk, trap, rte, stop, movep, rts-through-exception paths; the chunk C build as oracle;
  randomised trials (M3); HLE arms; no game-level check.
- **For Ryzen**: nothing for M2.

### 2026-10-28: M2 completed, the emitter covers every opcode the decoder accepts (DONE)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 19 unit tests (new: `Cpu::exception` from user mode), the hunk
  differential test and `emit_diff` pass; `python3 -m unittest discover -s tests/synthetic`: 9 tests pass (3 new);
  leakcheck clean. `emit_diff` now runs 11 hand-encoded programs, all identical between Rust and the `--single` C
  (D0-D7, A0-A7, SR, PC, cycles, OSP, the stopped flag, every changed RAM byte). New programs: `mul_div` (mulu/muls
  including the per-bit cycle loops, divu/divs with negative operands and both overflow cases), `exceptions` (divide
  by zero, chk above and below bounds, trapv taken and not taken, trap #3 twice from user mode with the stack swap,
  move usp, rte, rtr) and `movep_stop` (movep word and long both ways, stop, reset, nop). An opcode sweep (every
  opcode, extension words of 4) decodes 46088 opcodes and all of them translate to Rust; before this change 2193
  became `Stop::Untranslated`. Mutation checks: pushing only the low byte of SR in `Cpu::exception` fails
  `exceptions`; typing decimal literals as unsigned again fails `emit_diff`.
- **Decision (28 Oct 2026)**: the transliterator now handles `else`, block-local `uint32_t`/`int32_t` declarations,
  `for (; c; step)`, `/` and `%` (as `wrapping_div`/`wrapping_rem`, reached only after the decoder's zero check),
  `goto dyn_exit` and `cpu_exception` (new `Cpu::exception`, mirroring cpu_native.c). C literal types are kept
  (a plain decimal is a signed int), so `q < -32768` and `(int16_t)t < 0` stay signed compares. The test C driver's
  `cpu_exception` stub (it returned the PC) is replaced by the cpu_native.c body, so both sides really take the
  exception. The unsupported-text test moved to Python (`RustEmitterCoverage`), since no decodable opcode is
  untranslated any more.
- **Found (measured, not fixed)**: the decoder's C for `divs.w` computes `(int32_t)D / (int16_t)s` before its overflow
  check, so `divs.w #-1,Dn` with Dn = $80000000 is undefined behaviour in C: a 5-line gcc -O1 test of that exact
  expression dies with SIGFPE (exit 136) on x86-64. The Rust takes the overflow path (V set, Dn unchanged, as the
  68000 does). The test avoids that input. Fixing the C (test the overflow before dividing) is a translator change
  outside G5 and is for Karl to decide.
- **Assumed**: the seeds the `exceptions` program needs (its handler and the code after each trap) stand in for what
  the C kit's discovery gets from the reference run; the cycle counts are the C model's, not checked against Musashi
  here. Odd-address word access and custom-chip addresses are still not exercised.
- **Not done**: the chunk C build as oracle; randomised trials (M3); HLE arms; no game-level check.
- **For Ryzen**: nothing for M2.
