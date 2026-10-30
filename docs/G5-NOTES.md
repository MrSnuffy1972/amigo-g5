# G5: what is decided, what is assumed, and the session log

## Decided by Karl (dated)
- **29 Oct 2026**: Android versions of every game that passes G5 (296, 336, 608 today), with an updated pre-boot
  menu in the Laya Game Playa theme (metallic plus neon, Cinzel, VT323 and Inter), the same pre-boot on Windows.
  Built as milestones M7 to M9; Ryzen packages and installs (M10). Details: docs/G5-PLAYABLE.md.
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

### 2026-10-28: M3, full emitter coverage with randomised differential tests against the C translation (DONE)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 20 unit tests (new: the guest memory write journal), the hunk
  differential test, `emit_diff` and the new `rand_diff` pass (`rand_diff` about 26 s); `python3 -m unittest discover
  -s tests/synthetic`: 9 tests pass; leakcheck clean. `rand_diff` sorts all 46088 decodable opcodes into 1100 classes
  by the C each decodes to (register numbers and constants blanked), draws 2 opcodes per class with random extension
  words (2170 slots, 1 for single-opcode classes), translates them in one `m68k2c.py --emit rust --single` run and runs
  every slot from random D0-D7, A0-A7, OSP, CCR, supervisor bit and interrupt mask over noise-filled memory until 128
  trials complete (at most 512 attempts), on both the `--single` C (gcc -O1) and the generated Rust. Result: 267006
  trials compared, all identical (D0-D7, A0-A7, SR, PC, cycles, OSP, stopped flag, every changed byte); 56633 attempts
  were address errors on both sides; every one of the 1100 classes has a slot with the full 128 trials; 84 slots
  (second draws with odd extension words) have fewer. Mutation checks: the `abcd` V flag tested on bit 6 instead of
  bit 7 passes `emit_diff` but fails 4 slots of `rand_diff`; the emitter zero-extending instead of sign-extending
  `SX16` fails 273 slots.
- **Decision (28 Oct 2026)**: guest memory gets an optional write journal (`start_journal`, `journal`, `rollback`), off
  by default, so a test can list the bytes a step changed and undo them without copying 16 MB. It is meant for the
  per-tick oracle check later too. The pilot's "128 trials per instruction" is read as 128 per decoder class (two
  opcodes each, as above), since a translated test of all 46088 opcodes x 128 is too slow for CI.
- **Found (measured, not fixed)**: the `divs.w` $80000000 / -1 case recorded under M2 turns up in random trials (2 of
  them with the fixed seed): the C dies with SIGFPE. The C test driver catches it and the trial is not compared; the
  test checks the Rust took the 68000 overflow path (V set) there. The C fix is still Karl's call.
- **Assumed**: a word or long access at an odd address (or one that runs off the top of the 24-bit space) is
  reported as an address error by both sides and not compared further; neither side models the 68000 address error
  exception. The C model is the reference for cycles and for the undefined flags (abcd/sbcd N and V, div overflow,
  chk), so nothing is masked; Musashi is not in this loop (the pilot's difftest.c already checked the C against it).
  Extension words are random per slot but fixed across its trials; branch targets land where they fall in the image.
- **Not done**: the chunk C build as oracle (still the `--single` build); HLE arms; no game-level check.
- **For Ryzen**: nothing for M3.

### 2026-10-28: M4, cold boot of a filed program and the OS services it calls (DONE)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 22 unit tests (new: the fake OS tables and slot lookup, a typed stop
  for an unserved call), the hunk differential test, `emit_diff`, `rand_diff` and the new `os_diff` pass; `python3 -m
  unittest discover -s tests/synthetic`: 9 tests pass; leakcheck clean. `os_diff` assembles two synthetic AmigaDOS hunk
  programs in the test (a code hunk with RELOC32 into a BSS hunk; one in slow RAM ending with RTS to the CLI, one in chip
  RAM ending with dos Exit) with their own data files. The C side boots each the way the kit boots a filed game:
  hunkload.py with LoadSeg headers, oshle.py `execlib()` and its library table, oshle.c `game_boot_extra` (linked whole,
  with the real `hle_lib` serving every jump-table call) and cpu.c `run_boot`'s start registers, then runs the `--single`
  C translation. The Rust side boots the same files with `boot::cold_boot` and runs the generated Rust with `os::run`.
  Boot state (registers, a hash of chip and slow RAM) and end state (registers, cycles, custom-chip writes, every non-zero
  byte of chip and slow RAM) are identical: 197 and 99 steps. Calls covered: exec OpenLibrary, OldOpenLibrary (a fake
  base and an unknown name), OpenResource (ciab), AllocMem (slow, chip, the freed-block list), FreeMem (twice), AvailMem
  (chip, any, largest), Forbid/Permit, nested Disable/Enable, FindTask, CopyMem; dos Open (a path, a missing file),
  Read (in parts, past the end, after Close, the console), Seek (all three modes, past the end), Close, Lock (case
  ignored, a directory), UnLock, DupLock, ParentDir, Examine (file and root), ExNext, IoErr, Input, Output, Write,
  DateStamp, Info, Delay, Exit. Mutation checks, each fails the test: dropping dos's D1 = D0, a changed stack fill word,
  a changed Workbench copper list word, listing a block freed twice twice, Seek not held at the end, a changed root name.
- **Decision (28 Oct 2026)**: a call the C fake OS serves but the Rust does not yet is a typed stop
  (`Stop::OsCall { base, lvo }`), not "D0 = 0 and return", so a program that needs more of the OS stops at the call
  rather than drifting from the C. The Rust keeps the C's fake library bases and jump-table addresses, its allocator
  state mirror at $3FC8-$3FEF (so the memory compare covers it) and its boot display set-up; the custom-chip writes the
  C makes (COP1LC, DMACON, INTENA) are kept as an ordered list (`Os::chip_log`) for the presentation code (M5) to act on.
- **Assumed**: the C side's start (`game_boot_extra` plus cpu.c `run_boot`) and the AllocMem pools copied from
  oshle.py `main` into the test stand for the kit's full path (portg0.py, portgen.py `boot_gen.c`), which needs a game
  workspace. The C test driver's memory is chip 1 MB plus slow 1.5 MB (oshle.py's `chip_kb`/`slow_kb`) with custom
  registers logged, not the chipset. File names are looked up by their last path part, exact then ignoring case, as
  `gamefile` does for one disk.
- **Not done**: Workbench start (WBStartup message), the startup-sequence lines before and after the program, Assign,
  libraries from the game's disk, LoadSeg, overlays, CreateProc/AddTask, multi-disk volumes, save files (nativefs.c),
  and every other exec, graphics and intuition call the C serves (each is a `Stop::OsCall` today). No game-level check.
  Non-test code is about 700 lines (os.rs, boot.rs), a little over the ~600 aim, because the boot and the services are
  only testable together.
- **For Ryzen**: nothing for M4. When the pilot is chosen, its `AMIGO_OSHLE_TRACE` log from the C native build lists
  the OS calls it makes: any not in the list above is work before M6.

### 2026-10-28: M5 part 1, the clean-sheet display in Rust, checked against intent.c (PARTIAL)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 22 unit tests, `hunk_diff`, `emit_diff`, `rand_diff`, `os_diff` and the
  new `display_diff` (2 tests, about 3 s) pass; `python3 -m unittest discover -s tests/synthetic`: 9 tests pass; leakcheck
  clean. `display_diff` compiles a C driver that `#include`s the real `runtime/intent.c` (with `rgb12` taken from
  intentrt.c's text at test time and frames written by the kit's own `amigo_img_save`), feeds it and `display.rs` the same
  59 synthetic scenarios (11 written by hand: lores 5 planes, hires 4, hires 6 clamped, HAM6, EHB, BPU 7, DDFSTOP before
  DDFSTRT, all 8 sprites with reuse after VSTOP, attached pairs and dual-playfield priority, two copper-flow lists with
  COPJMP2, SKIP, the 8-bit WAIT line, WAIT h >= $E4, a COPJMP loop, a COP1LC move to a plausible and to an all-zero list,
  CPU writes during the frame and COPTIME deferral; 48 random: noise chip RAM, random shadow, copper lists of 10 to 309
  random MOVE, WAIT, SKIP, INTREQ, COPxLC, COPJMP and raw words, random CPU writes) for 2 or 3 frames each. Result: 121
  frames, 15485 table changes, 11,492,894 non-black pixels, every copper table (changes, callbacks, skips, the list's COP1LC/COP2LC), every
  next-frame copper start and every raw RGBA dump byte-identical. Mutation checks, each fails the test: HAM green and blue
  swapped, the EHB mask, the mid-line `+1`, the COPTIME cut-off moved by 2 either way, WAIT h >= $E6, SKIP with a 9-bit
  line, attached sprite bits swapped, sprite priority `<=`, odd/even plane scroll swapped, DIWSTOP bit 8 forced, no COPJMP
  loop guard, CPU palette writes not re-applied after a mid-line change, no line-312 sprite fetch.
- **Decision (28 Oct 2026)**: the Rust display runs intent.c's defaults with no switches (COPJMP, COPTIME, mid-line colour,
  6 planes and sprites on, hires by the left pixel). It reads chip RAM through `display::Chip`, a bounds-checked slice
  whose word addresses wrap at its power-of-two size as CHIP_WMASK does; no global state (the sprite pointers carried
  from frame to frame live in `display::Display`). Frames are dumped as `pNNNNN.rgba`, the kit's raw RGBA that judge.py
  reads.
- **Assumed**: intent.c's whole-frame compose is the right oracle for this step; the product runtime (intentrt.c) composes
  progressively, line by line as its virtual beam passes, so chip RAM written during the frame shows from that line. The
  random scenarios use noise bitplanes and sprite data, not pictures a game would draw.
- **Not done**: the progressive compose (intent_compose_to) and the copper re-read on a mid-frame CPU write to COP2LC or
  COPJMP1/2 (intent_copper_cpu; the test makes no such write); the blitter intents (intentrt.c op_blit_* and
  intent_blit_dispatch, including line mode); Paula channels and the 44.1 kHz mix with its BLEP and RC filters; the
  headless audio dump (main.c `--wav`); the hires AVERAGE policy and the other environment switches. Non-test code is
  about 600 lines (display.rs, without comments and blank lines), so the rest of M5 is the next session's.
- **For Ryzen**: nothing for this step.

### 2026-10-28: M5 part 2, the clean-sheet blitter in Rust, checked against intentrt.c (PARTIAL)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.13, gcc): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 24 unit tests (new: the minterm table, a BLTSIZE clear that wraps at the
  chip size), `hunk_diff`, `emit_diff`, `rand_diff`, `os_diff`, `display_diff` and the new `blit_diff` (about 2.5 s) pass;
  `python3 -m unittest discover -s tests/synthetic`: 9 tests pass; leakcheck clean. `blit_diff` takes intentrt.c's blit
  classes from its own text at test time (`ib_r16` to `intent_blit_dispatch_ext`), compiles them with gcc at a 128 KB
  CHIP_SIZE, starts each blit the way write_custom does (BLTSIZE, ECS BLTSIZV/BLTSIZH) and feeds it and `blit.rs` the same
  180 synthetic register streams over noise chip RAM (20 written by hand: clear 0/1/BLTADAT, copy A with masks, copy B,
  shifted copy up, down and down by 0, bob draw and bob with the mask from BLTADAT, inclusive, exclusive and FCI area fill,
  no destination, general minterms, pointers with high bits and odd modulos that wrap, BLTSIZE 0 (1024 x 64), chained blits
  from the pointers the last one left, ECS sizes including the zero defaults, 44 lines across all eight octants with and
  without SING; 160 random streams of 1 to 6 blits). Result: 763 blits (by class 0-7: 282 general, 11 clear, 2 copy, 8 shift,
  10 bob, 183 fill, 189 line, 78 no destination), 1,257,183 chip bytes changed, chip RAM, every blitter register after the
  stream and the class of every blit identical. Mutation checks, each fails the test: inclusive fill as exclusive, an octant
  swap, the descending B shift by one, no last word mask, the clear stride, copy taking shifted blits, SING ignored, the line
  pattern rotated the wrong way, no D pointer write-back, the ECS zero height, the line increments swapped, the bob cut
  without ~A, the fill carry not reset per row, the descending shift modulo sign.
- **Decision (28 Oct 2026)**: blits are done at once on the BLTSIZE or BLTSIZH write and return what they were
  (`blit::BlitKind`, named for what the game wants: Clear, Copy, Shift, DrawBob, Fill, Line, General, NoDest); the caller
  raises INTREQ bit 6 as write_custom does. Chip RAM is written through `blit::ChipMut`, bounds-checked, wrapping at its
  power-of-two size; the blitter's registers live in `blit::Blitter`, owned by the runtime, no global state.
- **Assumed**: the two BLTSIZE cases of write_custom, copied into the C test driver by hand, stand for the product's write
  path (they are the only lines of it a blit depends on). The random streams use noise data, not shapes a game would blit.
- **Not done**: wiring the blitter, display and OS chip log into one runtime tick; the copper writing blitter registers;
  the BLITLOG trace and class counters; Paula channels and the 44.1 kHz mix with its BLEP and RC filters; the headless
  audio dump (main.c `--wav`); the progressive compose and copper re-read on mid-frame COPxLC writes (from part 1). Non-test
  code is about 375 lines (blit.rs, without comments, blank lines and its unit tests).
- **For Ryzen**: nothing for this step.

### 2026-10-28: M5 part 3, Paula channels, the 44.1 kHz mix and the WAV dump in Rust, checked against intentrt.c (PARTIAL)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.11, gcc 13.3): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 27 unit tests (new: a square wave with its start and block-end
  interrupts, AUDxDAT with DMA off, the WAV header), `hunk_diff`, `emit_diff`, `rand_diff`, `os_diff`, `display_diff`,
  `blit_diff` and the new `paula_diff` (under 1 s) pass; `python3 -m unittest discover -s tests/synthetic`: 9 tests pass;
  leakcheck clean. `paula_diff` takes intentrt.c's whole audio section (audio_dmacon, aud_start, chan_step, audio_line,
  paula_setup and the vendored pt2-clone BLEP and RC filters it includes) and its `crw` from its own text at test time, and
  main.c's `--wav` header lines from main.c's text, compiles them with gcc (`-ffp-contract=off`) at a 64 KB CHIP_SIZE and
  feeds it and `paula.rs` the same 134 synthetic scenarios (14 written by hand: one square wave and all four channels
  (square, saw, sine, full-scale steps) in each of the raw, A500 and A1200 models with the LED filter in and out, period
  below 124, volume above 64, LEN 0, LEN 1 blocks, DMACON before LC/LEN with and without the g47 deferral, a channel off
  again before its deferred start, the DMACON master bit, AUDxDAT with DMA on and off, volume and period sweeps, model
  changes while playing; 120 random: noise chip RAM, random AUDx writes, DMACON and LED changes over 300 to 1000 lines).
  Result: 77,776 lines, 219,927 stereo samples (324,071 values not zero, 3 clipped), 12,177 audio interrupt bits, every
  sample, every INTREQ bit, the channel, BLEP and filter state bit for bit every 4th or 16th line, the BLEP table and every
  WAV file byte-identical. Mutation checks, each fails the test: left and right channel pairs swapped, the period floor
  123, the volume cap 63, the LED switch inverted, no g47 deferral, the BLEP step offset ignored, the BLEP offset clamped
  at 0.999 instead of 1, no end-of-line start interrupt, LEN 0 as 65535, the byte order in a word, the A500 low-pass in
  A1200 mode, a 361 ohm R321, the AUDxDAT interrupt ignoring the master bit, DMACON ignoring the master bit, a 227 colour
  clock line, a deferred start not cancelled by DMA off. (The test found one real bug on the way: the C keeps a BLEP
  offset in [0.999, 1) as it is, a Rust `clamp(0.0, 0.999)` did not.)
- **Decision (28 Oct 2026)**: Paula is `paula::Paula`, owned by the runtime, no global state. The output model (raw, A500,
  A1200) and the g47 start deferral are constructor arguments (the C reads AMIGO_PAULA and AMIGO_AUD_DEFER); their
  defaults are the C's (A500, no deferral). `play_line` takes the LED switch (`led_filter_on(pra, ddra)` from CIA-A) and
  returns the INTREQ bits instead of writing a global. Samples are appended to a caller's Vec, so there is no 16384-sample
  cap as in `audio_buf` (the C frontend drains it every frame, well under the cap). The BLEP table is a copy of
  pt2_blep.c's literals, with pt2-clone's BSD 3-Clause notice in the module header; the test checks its bits against
  the compiled C table. `write_wav` writes the file main.c `--wav` writes, which judge.py `wav()` reads.
- **Assumed**: the DMACON and AUDx cases of write_custom, copied into the C test driver by hand, stand for the product's
  write path. Rust's `f64::exp`, `tan` and `sqrt` give glibc's results (they call the platform libm), so the filter
  coefficients match; on another platform's libm they may differ in the last bit, which the test cannot see. The product
  C build may contract float multiply-adds where the test forbids it; that would change low bits of samples, not sound.
- **Not done**: the snapshot restore of channel state (aud_restore_shadow: a cold boot has no snapshot); the audio
  profile hooks; wiring the blitter, display, Paula and the OS chip log into one runtime tick (with CIA timers, the
  copper writing blitter registers and INTREQ/INTENA); the BLITLOG trace and class counters; the progressive compose and
  copper re-read on mid-frame COPxLC writes (from part 1). Non-test code is about 490 lines (paula.rs without comments,
  blank lines and its unit tests, of which 65 are the BLEP table).
- **For Ryzen**: nothing for this step.

### 2026-10-28: M5 part 4, the CIAs and the interrupt level in Rust, checked against intentrt.c (PARTIAL)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.11, gcc 13.3): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 31 unit tests (new: a continuous timer with its interrupt and ICR
  clear, a one-shot started by the high byte, a keyboard byte waiting for its handshake, the interrupt priorities),
  `hunk_diff`, `emit_diff`, `rand_diff`, `os_diff`, `display_diff`, `blit_diff`, `paula_diff` and the new `cia_diff` (about
  1.5 s) pass; `python3 -m unittest discover -s tests/synthetic`: 9 tests pass; leakcheck clean. `cia_diff` compiles the
  **whole** of intentrt.c with gcc (a 30-line driver defines only `CYC` and the PC ring it expects from cpu_native.c) and
  drives the real `chip_reset`, `hw_ciaw`, `hw_ciar`, `kbd_push`, `chip_poke` (INTENA/INTREQ), `tick_line_step` and
  `chip_ipl`; the Rust side runs `cia::Cias` and `cia::ipl` with VERTB added at each tick's end. 134 synthetic scenarios
  (14 written by hand: every register after reset, CIA-B timer A continuous at latches 0, 1, 44, 45, 46 and 300, one-shots
  started by the high byte and restarted, timer B in CRB bit 6 mode, the force-load strobes and a high-byte write while
  running, both TODs with alarms, the latch on a high read and the stop on a high write, the keyboard link with 70 keys
  (handshake, gap, the 17,872-line timeout probed line by line, the full queue dropping the 64th key), serial output and
  the ICR mask, fire buttons and ports through DDRA/DDRB, interrupt priorities with INTREQ cleared while a CIA still holds
  it; 120 random streams of 300 to 1500 lines). Result: 131,331 lines, 4,983 CIA reads, 74,878 lines with a CIA interrupt
  raised, lines by level 0-6 [60567, 1540, 25763, 11525, 11271, 12009, 8656]: every read value, INTREQ and level after
  every line identical. Mutation checks, each fails the test: the timer reload without its +1, timer B counting in CRB
  bit 6 mode, the force-load bit stored, 45 E ticks a line instead of 45.4, the two TODs swapped, no TOD stop on a high
  write, the TOD low read keeping its latch, the one-shot not started by the high byte, the alarm one count late, the
  key code not rotated, a 7-line key gap, a 62- or 64-key queue, the timeout one line late, a receive while SPMODE is an
  output, an ICR read keeping the SP bit, PORTS raised without the mask, level 5 without bit 12, the left mouse button
  on CIA-B, PRB ignoring DDRB.
- **Decision (28 Oct 2026)**: the CIAs are `cia::Cias`, owned by the runtime, no global state. `line_step` and
  `frame_end` are what one line and one tick do to them; `intreq_bits` returns the PORTS/EXTER bits for the caller to OR
  into INTREQ (the C's `update_cia_intreq` writes the global). The fire buttons are passed to `read` (`cia::Buttons`)
  instead of read from front-end globals. `Cias::led_filter_on` feeds `Paula::play_line`.
- **Assumed**: a reset CIA pair is the right cold-boot start (`chip_reset`; a snapshot boot restores the CIAs with
  chipset_restore_cia, which a cold boot does not have). With chip RAM zero and DMA off, the C's copper table and Paula
  raise no interrupt in the test, so INTREQ differences can only come from the CIAs or VERTB.
- **Not done**: wiring the display, blitter, Paula, CIAs and the OS chip log into one runtime tick (write_custom,
  read_custom, the copper's line-timed interrupts, the blitter's INTREQ bit, beam reads); the progressive compose and
  the copper re-read on mid-frame COPxLC writes (from part 1); the snapshot restore paths (not needed cold); the BLITLOG
  trace and the HWSTAT/CREADLOG counters. Non-test code is about 330 lines (cia.rs without comments, blank lines and
  its unit tests).
- **For Ryzen**: nothing for this step.

### 2026-10-28: M5 part 5, the custom chips as one runtime tick in Rust, checked against the whole of intentrt.c (M5 DONE)
- **Measured** (cloud container, rustc/cargo 1.97.0, python 3.11, gcc 13.3): `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings` clean; `cargo test --all-targets`: 32 unit tests (new: an OS boot log applied, a tick with VERTB and a
  copper SOFT interrupt, a beam read), `hunk_diff`, `emit_diff`, `rand_diff`, `os_diff`, `display_diff`, `blit_diff`,
  `paula_diff`, `cia_diff` and the new `chipset_diff` (about 8 s) pass; `python3 -m unittest discover -s tests/synthetic`:
  9 tests pass; leakcheck clean. `chipset_diff` compiles the **whole** of intentrt.c (with intent.c and the blit classes)
  with gcc (`-ffp-contract=off`, CHIP_SIZE 64 KB, environment cleared so every switch is at its default) and drives the
  real `chip_reset`, `chip_poke` (write_custom), `hw_r16` (read_custom) at a given scheduler clock, `hw_ciaw`/`hw_ciar`,
  `tick_line_step` and `chip_ipl`, with chip RAM written directly mid-tick and the front-end input globals set; the Rust
  side runs `chipset::Chipset`. 86 synthetic scenarios (6 written by hand: a 4-plane screen with copper interrupts at
  lines, CPU palette and BPLCON0 writes and drawing during the tick; COPJMP1/2 and COP2LC written by the CPU mid-tick
  and a list moving COP1LC to a plausible list and later to zero memory; clear, copy, line and fill blits started by
  BLTSIZE and ECS BLTSIZH with their interrupt; two Paula channels, AUDxDAT with DMA off and the LED filter switched by
  CIA-A; beam reads at 0 to 142,000 cycles into a line, across lines and the end of the tick; every input combination and
  every readable register; 80 random: noise chip RAM, four random copper lists of up to 85 MOVE, WAIT, SKIP, INTREQ and
  pointer moves, random writes to 64 registers, reads, CIA accesses, chip RAM writes and input over 300 to 1200 lines).
  Result: 69,205 lines, 183 frames, 195,712 stereo samples, 2,462 reads, blits by class 0-7 [11, 1, 1, 0, 0, 21, 43,
  94], lines by level 0-6 [9286, 835, 182, 6259, 34018, 10672, 7953]: every read, INTREQ before and after `chip_ipl`,
  the level after every line, a hash of every audio sample and of every finished frame, the blit classes and chip RAM
  at the end identical. Mutation checks, each fails the test: no progressive compose, no copper re-read on a CPU
  COP2LC/COPJMP write, COPJMP2 restarting at COP1LC, no plausibility guard on the next tick's COP1LC, no COP1LC from the
  table, no line-0 copper interrupts with VERTB, no blit interrupt, no line wrap in the beam read, JOY1DAT up and down
  swapped, CPU writes stamped a line late, DMACON not passed to Paula, no AUDxDAT interrupt, bitplane pointer halves
  swapped, DMACONR unmasked, the LED switch inverted, CIA bits not in INTREQ after the line, no CIA-A TOD count, the
  POTGOR fire bit moved. Four mutations pass and are equivalent, so the code mirrors the C without a test that can see
  them: the beam position cap $E2 (453 / 2 never exceeds it), the palette's 12-bit mask (masked again when shown), the
  compose cursor stepped back on a re-read (in progressive mode it never passes the line of the write), and the C's
  second compose at the end of the tick (removed in the Rust: the call before it already reaches line 312).
- **Decision (28 Oct 2026)**: the custom chips are `chipset::Chipset`, owned by the runtime, no global state. It owns
  the display, blitter, Paula and the CIAs and does what write_custom, read_custom, tick_line_step and chip_ipl do with
  them; chip RAM is passed in by the caller (guest memory owns it). The CPU clock for a beam read is an argument (the
  C reads the scheduler's `CYC`). The player's controls are `chipset::Input` instead of front-end globals.
  `apply_os_log` replays `Os::chip_log` (the fake OS's boot writes) through the same write path. The display gained the
  progressive compose (`Display::compose_to` with a `Progress` cursor) and the mid-tick copper re-read
  (`Frame::copper_cpu`); the whole-frame `compose` is now the progressive one run to line 312. After a blit the
  pointers it left are copied into the register shadow, as the C's blitter works on that shadow. Audio samples are
  appended to `Chipset::audio` for the caller to drain, with no 16384-sample cap.
- **Assumed**: the test driver's clock (the scheduler steps one line per 454 cycles, a read happens at a given offset
  in the line) is how the product's run loop will call the chips. The random copper lists and register streams are
  noise, not what a game sets up.
- **Not done**: the dev traces and counters (BLITLOG, HWSTAT, CREADLOG, DECSTAT, WATCH, the sprite findings); the
  snapshot restore paths (chipset_restore_words, chipset_restore_cia, aud_restore_shadow: a cold boot has none); the
  test-only RNG replay (AMIGO_RNG_REPLAY); the environment switches other than their defaults. **Not part of any
  milestone yet**: routing the translated code's custom-chip and CIA accesses ($DFFxxx, $BFxxxx) from guest memory to
  `Chipset`, the run loop that steps one line per 454 cycles and takes interrupts at `ipl`, and a headless runner that
  writes `pNNNNN.rgba` and the WAV for judge.py. M6 needs them; Karl to say whether they are M6's first step. Non-test
  code is about 375 lines (chipset.rs 270 and display.rs about 105 added, without comments, blank lines and unit tests).
- **For Ryzen**: nothing for this step.
- **M5 status**: done (parts 1 to 5: copper list as a table, bitplane composition, sprites, the blitter intents, Paula
  and its mix, the CIAs, the whole tick, frame and WAV dumps in judge.py's formats). Next is M6, which needs Karl.

### 2026-10-28: M6, the pilot game cold-booted in the Rust port on Ryzen and judged against the reference (DONE; Karl 28 Oct 17:25: "Run it all through")
- **Pilot choice (measured):** game 234 (Airball), named as the likely pilot in G4-DESIGN.md, is NOT in G5 scope: it is a
  WHDLoad game (game-orig.json `whdload`), and the G5 scope is the G4 entry rule plus preport triage `filed` (15 games:
  135 179 296 336 337 344 375 464 472 502 507 549 557 608 713). The pilot is **296, Global Thermonuclear Warfare (1994)**:
  one AmigaDOS program (GTW, 398,208 bytes) started by `s/startup-sequence`, snapshot frame 0 in its clean-sheet run, and
  its C native build's AMIGO_OSHLE_TRACE lists 9 OS calls (Forbid, OpenLibrary x2, AllocMem, dos Open, Seek x2, Read,
  Close), all served by os.rs since M4. Baseline (measured): the C native build against the same cold-boot reference
  passes judge.py (picture mean 1.0000, sound 0.987).
- **Measured, pilot 296** (Ryzen, campaign 81 G5 Epoch 1, gate_results gate 5 `pass` 17:57:39, class `pass:rust-coldboot`):
  - the boot binary cold-boots GTW from the disk's own files (entry $004008, A7 $07FFE8, the C kit's values), runs 600
    ticks headless: 5,330,814 translated instructions (the reference harness: 5,345,769 for 603 frames), 9 OS calls,
    599 VERTB interrupts, 0 unmapped accesses;
  - judge.py: PICTURE PASS, SSIM >= 0.90 on 100% of ticks, mean 1.0000, min 0.9879; 113 of 600 reference ticks moving
    (the gate needs 30); TICK_OFFSET +0; SOUND PASS, mean spectral correlation 0.998, 100% of ticks >= 0.85;
  - 599 of 600 frames byte-identical to the reference (the C native build: 520 of 600);
  - g5_symcheck.sh PASS: 1297 symbols, no interpreter or chip-model names, no emulator strings, Kickstart byte search
    over the binary with 28 ROMs loaded: no ROM bytes;
  - image check: the 3371 translated instructions' first words, 5744 bytes, equal the cold boot's RAM (0 differ; 998
    bytes are code the translator found outside the clean-sheet image, from the reference run's observations).
- **Measured, the rest of the scope** (gate_results gate 5; G5 passed now **3 of 15**: 296, 336, 608):
  - Epoch 1 (campaign 81, amigo-g5 68ef8a5): 296 and 336 pass. 135, 179, 337, 344 and 375 failed the image check:
    the first startup-sequence command was not the program the kit boots (fixed in part 2 for 135 and 337).
    464, 502, 507, 549, 557 and 608 were stopped by this session after rustc had compiled one game's single
    huge function for about 15 minutes (fixed in part 3); 472 and 713 never started (epoch cancelled).
  - Epoch 2 (campaign 82, amigo-g5 895d018, 14 targets): **336 pass** again; **608 pass** (E-Motion, 111,280
    translated instructions: picture mean 0.9995, min 0.9659, 528 of 600 ticks moving, sound 0.990, symcheck PASS).
  - 135 fail:picture, but only by the moving rule: picture PASS (mean 1.0000, min 1.0000) and sound PASS (0.999),
    with 15 of 600 reference ticks moving where the gate needs 30 (no input in G5 runs).
  - 337 fail:picture (SSIM >= 0.90 on 41%, sound 0.655): the reference runs the startup-sequence's crack intro (dos
    Execute at frame 0) and starts the game's program at frame 101; the port boots the program at once. With
    AMIGO_G5_OSTRACE against the reference's AMIGO_OSHLE_TRACE: port tick 1 shows the reference's tick 103 frame.
  - 179 and 472 fail the image check on disk-library stubs (5 and 349 bytes at $0F72, $2E9A-$2EE3): libraries
    from the game's disk are not served by os.rs yet.
  - 464 and 502 stop at tick 1 on intuition.library LVO -$84 (GetPrefs), not served yet.
  - 507 fails the image check at $C1000 (24 bytes): the C kit loads code into slow RAM there and the Rust cold boot
    does not (not measured further).
  - 549: the kit's program `kick` is on disk 2 (multi-disk not served; the glue copies disk 1's files only).
  - 344, 375, 557: trackloader games with `filed` triage (no AmigaDOS program to boot); out of reach of a filed
    cold boot.
  - 713: its gate item waits for G2 campaign 80 (one slot per game). Run by hand through the same g5_build.sh:
    builds, cold-boots `tetris`, and stops at tick 279 on untranslated $004058 after 4.3 million instructions with
    one OS call; most likely the program unpacks itself over its own start, and the translator's image (taken at
    clean-sheet frame 2350) holds the later bytes there (assumed, not measured further).
- **Decisions (28 Oct 2026):**
  - Interrupts, lines and OS calls run between single translated instructions (the `--single` build), with the beam
    clock stretched by the C's cpu_slow_num 1126/1024 and the fraction carried. No yield table is needed: a busy loop
    spins on its own cycles, as on the 68000, and the instruction count lands within 0.3% of the reference's.
  - The bus routes $DFFxxx/$BFxxxx exactly as intentrt.c's mem_* do and, when attached, reads unmapped addresses as 0
    (counted) as the C does; M1-M5 code without a bus keeps its typed errors.
  - `chipset` renamed `hardware` (and `line_step` to `pass_line`) because g5_symcheck.sh's name rules reject them; the
    check is by name and nothing else changed (every differential test passes unchanged).
  - The game's translated code is compiled into the boot binary through AMIGO_G5_GAME_RS on Ryzen, in 512-byte
    page functions spread over eight crates (rustc's front end grows much faster than linearly with one crate's size:
    100 pages of a game checked in 5.2 s, 200 in 19 s; 608 now builds in 16 s). The binary prints the code's FNV-1a
    64 and the Ryzen build checks it, because one build directory serves every game.
  - The program is the kit's G0 choice (g0.json `exe_rel`, `--program`), with oshle.py's argument rule.
  - OS custom-chip writes made at run time (Disable, Enable, AddIntServer's INTENA) reach the chips after the call.
- **Assumed:** the translator's inputs are the clean-sheet workspace's (its code image taken at the program's entry, its
  seed list and the reference run's discovery observations); the image check proves the bytes the cold boot loads are
  the ones translated, and nothing of a reference run is used when the port runs. The reference is the game's own
  reference harness from power-on (Musashi plus the kit's fake OS, no Kickstart), with no input, as G2/G3 use it.
- **Not done:** graphics and intuition calls (464, 502); running the startup-sequence's other lines (337's reference runs a crack intro with dos Execute for
  about 100 ticks before the program, so the port runs about 100 ticks ahead); Workbench start (179, `run >nil: hfro`);
  libraries from the game's disk (179's image check: 5 bytes of a disk library stub differ); trackloader games in the
  `filed` triage (344, 375, 557: no AmigaDOS program to boot); reading the ADF in the binary (the Ryzen glue gives it
  the files G0 extracted); a G5 Windows build.
- **Incident (owned):** the first sync-in committed rust/ to kit-intent, and kit_rev's first part is kit-intent's newest
  commit, so kit_rev moved (0cdb614 to 3a06cbe) for about two minutes while G2 campaign 80 ran; restored by moving
  the branch back one commit and listing /rust/ in .git/info/exclude (nothing deleted). One campaign-80 workspace,
  game 404, was stamped with the moved label at 17:48 (same dev and tools content). g5-sync-in.sh's G5_RUST_ONLY mode
  now never commits and compares the full kit_rev before and after.
- **Ryzen side (not in this repo):** /srv/fast/cleansheet/g5/g5_ref.sh (cold-boot reference), g5_game.sh (per-game
  glue: disk files, the kit's program, translation, the image check), g5_build.sh (build lock, --program, fingerprint
  check); the crate is
  synced with `g5-sync-in.sh` in its G5_RUST_ONLY mode (rust/ only, uncommitted in kit-intent, so kit_rev is unchanged
  while G2 campaign 80 runs).

### 2026-10-29: M7, the playable runtime on desktop (DONE, with the pause menu's Controls entry moved to M8)
- **What**: `amigo-g5-play` (crate `rust/amigo_g5_play`): SDL2 window, 50 Hz fixed timestep decoupled from the
  display (the clock is set again after a pause or a stall of more than 5 ticks, so the game never runs fast to catch
  up), 44.1 kHz stereo audio queue (882 frames a tick, capped at 6 ticks queued), SDL GameController, keyboard and
  mouse mapped to the Amiga joystick (port 2), mouse (port 1) and keyboard; the in-game pause menu (Resume, Restart,
  Quit; Start, Back, Esc or the TV remote's Back opens it, never quits); letterboxed scaling. The game pack
  (`amigo_g5::pack`, `tools/g5_pack.py`) and the game.json `controls` block, both documented in docs/G5-PLAYABLE.md.
  The program choice and file collection moved from the boot binary into `amigo_g5::game::GameTree` unchanged, so the
  two binaries boot the same way.
- **Decisions (29 Oct 2026)**:
  - `amigo_g5_play` is its own cargo workspace: in the main workspace its crates enter rust/Cargo.lock, and
    `cargo build --release --offline --locked --bin amigo-g5-boot` then fails on a cache without them (measured:
    "no matching package named `fontdue` found"). The boot binary's lock file is unchanged by this PR.
  - New dependencies (play crate only): `sdl2` 0.37 (`bundled`, `static-link`: window, renderer, audio queue and game
    controllers, with SDL 2.26.4 built from source so the exe needs no DLL), `serde_json` 1 (reads the game.json
    `controls` block), `fontdue` 0.9 (pure-Rust rasteriser for the bundled Inter font in the pause menu; M8 uses it
    for the pre-boot).
  - The pause menu's Controls entry shows the full-screen controller diagram, which is M8's; it is added with M8.
- **Measured** (this container, rustc 1.97 stable, cmake 3.28, gcc 13):
  - `cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets`: clean,
    all pass (amigo_g5 lib 37 tests including the new game.rs and pack.rs ones).
  - `cd rust/amigo_g5_play && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
    --all-targets`: clean; 8 unit tests and `play_frames_match_headless_under_scripted_input` pass. Under SDL's dummy
    video and audio drivers, 14 ticks of a synthetic program that reads JOY1DAT, CIA-A PRA, JOY0DAT and the keyboard
    give byte-identical frames in the play loop (input from a script: key, pad, mouse move, mouse button) and in the
    headless loop amigo-g5-boot runs (input set by hand); 6 or more distinct frames, so every input showed. Made to
    fail on purpose twice (fire held one tick longer, a different key code): it failed at tick 8 and tick 9.
  - The same test packs the synthetic game with `tools/g5_pack.py`, reads it back in Rust equal to the directory,
    and runs `amigo-g5-play --pack` (no game code compiled in): exit 3, "STOP after 0 ticks: Untranslated".
  - `python3 -m unittest discover -s tests/synthetic -v`: 11 tests OK (2 new: pack round trip, damage caught).
  - Boot refactor: the old (main) and new `amigo-g5-boot` on a synthetic game directory with no `--program`,
    `--program Other`, `--program data/f` and `--program nope` give identical stderr, exit codes and audio.
  - With an empty CARGO_HOME, `cargo build --release --offline --locked --bin amigo-g5-boot` in rust/ builds.
- **Assumed**: SDL GameController events on a real Xbox pad, Shield controller and TV remote arrive as SDL documents
  them (no pad here); the audio queue's 6-tick cap keeps latency acceptable on real hardware (not measured with a
  real device); a real game's frames in amigo-g5-play equal amigo-g5-boot's (the core is shared and the synthetic
  test proves the input path, but no game runs here).
- **Not done**: the pause menu's Controls entry (M8, with the diagram); the menu's look is a first cut (metal
  panel, bevels, cyan focus, Inter labels) that M8's theme module takes over; mouse support inside the pause menu
  is a click on the selected item only; Android `SDL_main` and APK asset loading (M9). Size: about 1,100 lines of
  non-test code, of which about 130 moved from the boot binary; over the ~600 aim, because the milestone's pieces
  (runtime, mapping, pack, menu) only prove out together in the frame test.
- **For Ryzen**: optional, before M10: build with a game's AMIGO_G5_GAME_RS and compare frames:
  `cd rust/amigo_g5_play && AMIGO_G5_GAME_RS=... cargo build --release` then
  `amigo-g5-play --game WORKDIR/g5files --program REL --ticks 600 --frames /tmp/play` against
  `amigo-g5-boot ... --ticks 600 --frames /tmp/boot` (`cmp` each pNNNNN.rgba). Needs `cargo fetch` in
  rust/amigo_g5_play first for an offline build, and `CMAKE_POLICY_VERSION_MINIMUM=3.5 CFLAGS=-std=gnu17`.
- **M7 status**: done (Controls entry of the pause menu moved to M8). Next is M8.

### 2026-10-29: M8, the pre-boot menu in the LGP theme (DONE, with the Android on-screen controls checkbox moved to M9)
- **PR**: https://github.com/MrSnuffy1972/amigo-g5/pull/12
- **What**: `amigo-g5-play` opens on the pre-boot at every start (Karl's rule of 14 Oct): the game's name in Cinzel,
  the "Ami-Go G5" badge and build line in VT323 on a Workbench-style title bar, the `start_hint` line, a Controller
  panel with an Xbox-style pad drawn in code (vector shapes, no artwork, no logos) and a leader line with an arrow
  from each mapped control to its label, a Keyboard and mouse panel, Start game / Controls / Quit with a cyan glow on
  focus, and the 30 s orange countdown bar (auto-start; any input pauses; 10 s idle resumes). The theme lives in one
  module (`theme.rs`); the pause menu now uses it and has its Controls entry. `--preboot-shot OUT.png --preboot-at
  SECONDS` writes the menu as a 1920x1080 PNG. Labels and chips come from the `controls` block (`labels`,
  `label_status`, `start_hint`, read and ignored when absent). Details in docs/G5-PLAYABLE.md "Added by M8".
- **Decisions (29 Oct 2026)**:
  - Screens are drawn on the CPU in a 1920x1080 design space scaled to the window, so the screenshot needs no
    display, the layout test is exact, and the look is the same at 720p and 4K.
  - The PNG writer is 40 lines in theme.rs (stored deflate blocks, CRC-32, Adler-32): no new dependency. A shot is
    about 6 MB; fine for review, and the size was not worth a compressor crate.
  - A mouse moving over the window pauses the countdown only when it hovers a button.
  - `--preboot-at` counts seconds since the pre-boot opened (0 shows 30 s left).
  - A crowded side (9 labels with chips) puts each chip after its label instead of under it, so the labels fit.
- **Measured** (this container, rustc 1.97 stable):
  - `cd rust/amigo_g5_play && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
    --all-targets`: clean; 18 unit tests (new: countdown auto-start at 30 s, pause and 10 s idle resume, menu
    navigation by key, pad, remote and mouse, labels from the controls data, no overlapping labels at 1920x1080 and
    1280x720 on both screens for the default mapping and for every control mapped with long labels and chips,
    screens render with the glow and the bar, the pause menu's Controls entry, label_status and start_hint parsing,
    key names, PNG checksums), `play_frames_match_headless_under_scripted_input` (unchanged, passes) and the new
    `preboot_shot_renders_without_a_display` (the binary writes a PNG that decodes to 1920x1080 with every CRC and
    the Adler-32 right, orange to the bar's middle at 15 s and all of it at 0 s; `--preboot-at 31` exits 2).
  - `cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets`: clean,
    48 passed (the boot workspace is untouched by this PR).
  - `python3 -m unittest discover -s tests/synthetic`: 11 tests OK. Leak check: clean.
  - Under SDL's dummy drivers, `amigo-g5-play --game SYNTH --config game.json` with no input waited on the pre-boot
    and started the game by itself after 30.5 s (exit 3, "STOP after 0 ticks: Untranslated": no game code here).
  - Looked at the shot (1920x1080, menu) and the Controls screen at 1280x720 by eye: labels clear of each other and
    of the pad, chips readable.
- **Assumed**: the Shield remote's centre arrives as SDL's `Select` scancode and its Back as `AC Back` (SDL 2's
  Android key mapping; not tried on a device); the pre-boot redraw (a copy of the cached static layer plus the
  buttons and bar each frame) keeps 50 Hz in a release build on the Shield (the debug build here used about 0.8 of a
  core at 1920x1080 under the dummy driver; not measured on a device).
- **Not done**: the Android "On-screen controls" checkbox (Android only, hidden on Windows; its default needs the
  touchscreen and controller queries and per-game app storage that come with M9's Android glue, so it moves to M9
  with the touch controls it switches); left stick navigation in the pre-boot (the D-pad and keys do it); the
  Controls screen reuses the menu's two-panel layout at a larger height rather than a bigger pad. Size: about 1,000
  lines of non-test code (theme 360, pad 310, pre-boot 330, the rest in controls, lib, main and menu), over the ~600
  aim: the screenshot, layout and render tests only prove out with all three modules together.
- **For Ryzen**: optional, before M10: `cd rust/amigo_g5_play && cargo build --release` then for each G5 pass
  `amigo-g5-play --pack PACK.bin --preboot-shot /tmp/preboot-ID.png --preboot-at 5` and look at it, with the game's
  real `controls` block. The game-level frame check from M7 is unchanged.
- **M8 status**: done (the Android checkbox moved to M9). Next is M9.

### 2026-10-29: M9 part 1, the Windows and Android builds (PARTIAL: touch controls are part 2)
- **PR**: https://github.com/MrSnuffy1972/amigo-g5/pull/13
- **What**: the start of `amigo-g5-play` moved into `amigo_g5_play::app::main`, shared by the desktop binary and the
  new Android library `libmain.so` (crate `rust/amigo_g5_play/android`, `SDL_main` in its module `ffi`, no
  `unsafe`). The pack comes from `--pack`/`--game`, else from the pack built in with `AMIGO_G5_PACK` (the one-file
  Windows exe; such a build is a Windows GUI program with no console), else from `gamepack.bin` through SDL_RWops
  (the APK asset). Android Back is trapped so it reaches the pause menu. `tools/g5_package.sh` builds the exe and
  the APK per the packaging contract. CI clippy for aarch64-linux-android and x86_64-pc-windows-gnu. Details in
  docs/G5-PLAYABLE.md "Added by M9 part 1".
- **Decisions (29 Oct 2026)**:
  - The APK stages of `kit/amigo/android/build_apk.sh` are copied into `g5_package.sh`, not changed in place:
    build_apk.sh needs C sources, and editing a kit/ file moves Ryzen's kit_rev and rebuilds every G2 to G4
    workspace (docs/G5-RYZEN-CONTRACT.md section 4).
  - The symbol check is Ryzen's own `g5_symcheck.sh` (`G5_SYMCHECK`), not a copy: a rewrite here could disagree
    with the gate (a debug build here has `amigo_g5::cia::Cia::tod_step`, which a naive reading of contract section
    3 would flag, while Ryzen's check passes the release build).
  - The pack is built into the exe but not into the APK's library (it is the asset there), so it is never twice.
  - The Windows cross check in CI runs without the `desktop` feature: with it, sdl2-sys builds SDL2 with a mingw
    compiler CI lacks. No new dependency.
- **Measured** (this container, rustc 1.97 stable, cmake 3.28, gcc 13):
  - `cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets`: clean,
    48 passed (the boot workspace is untouched).
  - `cd rust/amigo_g5_play && cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo test
    --all-targets`: clean; 20 unit tests (new: the pack source order, a damaged built-in pack exits 2),
    `pack_sources` (new: no pack and no asset exits 2 naming gamepack.bin; `gamepack.bin` in the working directory
    boots with `play: pack gamepack.bin` and the same last stderr line as `--pack` on the same file; a damaged one
    exits 2), `play_frames` and `preboot_shot` pass unchanged.
  - `cargo clippy --target aarch64-linux-android -p amigo_g5_android -- -D warnings` and `cargo clippy --target
    x86_64-pc-windows-gnu --no-default-features --bin amigo-g5-play -- -D warnings`: clean. With `AMIGO_G5_PACK`
    set, the Windows check's rustc line carries `--cfg g5_embedded` (seen with `-v`).
  - The Android crate built for the host (`cargo build -p amigo_g5_android --features amigo_g5_play/desktop`):
    `nm -D --defined-only libmain.so` shows `T SDL_main`.
  - Built in: `AMIGO_G5_PACK=p.bin cargo build --bin amigo-g5-play` with a synthetic pack, run with no arguments
    and `--ticks 1` under the dummy drivers: `STOP after 0 ticks: Untranslated(12582920)`, exit 3 (the pack booted
    to its entry point; no game code here).
  - `python3 -m unittest discover -s tests/synthetic`: 15 tests OK (4 new: `g5_package.sh --dry-run` gives the
    three cargo builds with the contract's targets, the pack built into the exe and unset for Android, the SDL2
    and libc++ copies, symbol checks on the three unstripped binaries in order, the asset, the signed
    `OUTDIR/synth.apk`, a manifest with package `org.amigo.g5.synth`, both launchers, banner, landscape, touchscreen
    not required and gamepad, and an escaped label; x86_64 when asked; `--skip-symcheck` says so; a missing target,
    a bad id, a missing value, a missing icon and a damaged pack exit 2). Leak check: clean.
- **Assumed**: the NDK 26 clang names (`aarch64-linux-android21-clang`, `armv7a-linux-androideabi21-clang`) and
  sysroot paths of `libc++_shared.so` are where `g5_package.sh` looks; sdl2-sys without `bundled` links
  `-lSDL2` against `/opt/sdl2-android/ABI/libSDL2.so` and nothing more is needed on Android; `sdl2` 0.37's `bundled`
  build cross-compiles SDL 2.26.4 with mingw-w64 on Ryzen (cmake-rs sets the Windows system name); `g5_symcheck.sh`
  takes the binary as its one argument and exits 0 on a pass; SDL_RWops reads the APK asset `gamepack.bin` at
  `SDL_main` time (SDL documents it; not run on a device); a Windows GUI exe with SDL2 static needs no SDL2main.
  None of these links ran here: no NDK, no mingw.
- **Not done**: the Android on-screen touch controls and the pre-boot "On-screen controls" checkbox (moved from M8;
  M9 part 2: virtual stick, four buttons, pop-up Amiga keyboard, pause button, the layout tests at 1080x2400 and
  720x1600, the checkbox default rule and per-game storage); the Windows exe icon; stderr to Android logcat (a
  failed start on Android is silent for now); running `kit/amigo/g6check.py`, which is not in this repo. Size: about
  290 lines of new non-test code (app 130 after the 110 moved from main.rs, package script 145, build.rs 35,
  Android 20).
- **For Ryzen**: in the amigo-tools image, after `cd rust/amigo_g5_play && cargo fetch`: for a G5 pass,
  `AMIGO_G5_GAME_RS=... G5_SYMCHECK=/path/to/g5_symcheck.sh tools/g5_package.sh --pack PACK.bin --name NAME --id
  gNNNNN --out /w/out/g5pkg --windows --android`, then `kit/amigo/g6check.py` on `/w/out/g5pkg/gNNNNN.apk`, and
  under wine `gNNNNN.exe` (no arguments: it opens on the pre-boot). Report any link error from either target.
- **M9 status**: partial (part 1 done; part 2 is the touch controls and the checkbox). Next is M9 part 2.

### 2026-10-29: M9 part 2, the on-screen touch controls and the pre-boot checkbox (M9 DONE)
- **PR**: https://github.com/MrSnuffy1972/amigo-g5/pull/14
- **What**: `rust/amigo_g5_play/src/touch.rs`: the virtual stick (8 directions, visible dead zone), four buttons in a
  diamond from `touch.buttons` or the default rule, the pause button, the key button and the pop-up Amiga keyboard,
  multi-touch, slide-off release, the overlay in the theme (40% at rest, 80% touched). Fingers become `Host::Touch`
  actions that go through `Mapper` like a pad's. The pre-boot's "On-screen controls" checkbox (Android only; the
  default rule; remembered per game in SDL's pref path). The pause menu takes clicks and taps by position. `--touch
  on|off` and the script line `TICK touch down|move|up ID X Y`. Details in docs/G5-PLAYABLE.md "Added by M9 part 2".
- **Decisions (29 Oct 2026)**: fewer than four default buttons leaves the empty places undrawn; the open keyboard
  hides and lets go of the stick and buttons; with the controls on, SDL's touch-made mouse events are dropped in
  game; the pointer shows while paused. Also fixed: `rawkey("F11")` (Help) returned nothing.
- **Measured** (this container, rustc 1.97 stable):
  - `cd rust && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --all-targets`: clean,
    48 passed (the boot workspace is untouched).
  - `cd rust/amigo_g5_play && cargo fmt --all --check && cargo clippy --all-targets -- -D warnings && cargo test
    --all-targets`: clean; 27 unit tests (new: button choice and start key, layout at 2400x1080, 1600x720, 1920x1080
    and 1280x720 with no overlap, on screen and clear of the middle third, keyboard keys inside and apart, fingers
    give the same `Input` as the pad with stick and fire at once, slide-off and dead zone, the keyboard sends F1 as
    raw $50, the overlay is see-through with 40% and 80% controls, the checkbox default rule and storage round
    trip, the checkbox as a fourth focus item drawn ticked in cyan and gone when hidden, the touch script line);
    `play_frames` now also runs the scripted test with the stick and fire given by fingers (`--touch on`) and its
    14 frames equal the headless ones byte for byte; `pack_sources` and `preboot_shot` pass unchanged.
  - `cargo clippy --target aarch64-linux-android -p amigo_g5_android -- -D warnings` and the Windows check: clean.
  - `python3 -m unittest discover -s tests/synthetic`: 15 tests OK. Leak check: clean.
  - Looked by eye at the overlay at 2400x1080 (stick held, four buttons, key and pause buttons), the open keyboard
    and the pre-boot shot with `--touch on`: as described, nothing clipped.
- **Assumed**: SDL on Android reports the touchscreen in `SDL_GetNumTouchDevices` before the first touch, and a
  Shield with no touchscreen reports none (so the default is off there); SDL_GetPrefPath on Android is the app's
  writable internal storage; SDL's touch events arrive normalised to the window (SDL documents both; not run on a
  device); redrawing the full-screen overlay when a control changes keeps 50 Hz in a release build on a phone (not
  measured on a device: while the stick is dragged it is redrawn every frame).
- **Not done**: the Windows exe icon and stderr to Android logcat (from part 1, still open); the pause menu is
  unchanged in look (tap support only); no haptics. Size: about 820 lines of non-test code (touch.rs 545, lib.rs
  150, preboot 75, theme 60, menu 35, controls 30), over the ~600 aim: the checkbox, the overlay and its wiring
  only prove out together in the scripted frame test.
- **For Ryzen**: with part 1's `g5_package.sh` command, install a G5 APK on a phone (touch, no pad: the box should be
  ticked) and on the Shield (it should not be); on the phone, play with the stick and buttons, open the keyboard,
  and check the choice survives a restart. Optional desktop review: `amigo-g5-play --pack PACK.bin --touch on`
  (the controls show; a mouse does not drive them, fingers do) and `--preboot-shot /tmp/pb.png --touch on`.
- **M9 status**: done (part 1 and part 2). Next is M10, which is Ryzen's.

## 30 Oct 2026: playability fixes (Karl, Qattro "fix this for all games")

Qattro passed G5 (600 ticks, no input) but the APK closed at once, ignored the pad and froze at tick 1855. Four causes, all generic:

1. **Wrong program.** `amigo-g5-play` booted the first command of `s/startup-sequence` (Qattro's is `type`), while the G5 check boots the kit's G0 choice (`program.txt`). The pack's game.json now carries `"program"`, and play boots it when no `--program` is given (`app.rs config_program`).
2. **Input on the wrong port.** Qattro reads only JOY0DAT and CIA-A PRA (port 1). Play now counts JOY0DAT/JOY1DAT reads (`Hardware::port_reads`); a game that has read only port 1 gets the joystick there: directions move the port-1 counters, Fire is the left button, Fire 2 the right.
3. **Code translated from zeros.** The translator read bytes inside code ranges that no segment loaded (the program unpacks there at run time) as the image's zero fill whenever the reference run had not executed them, so a `beq` into Qattro's music player ran `ori.b #0,d0` words into the next routine's `rts`, left A6 at $DFF020 and spun on a blitter wait at $DFF022. `m68k2c.py Mem.rb` now takes those bytes from the reference run's end-of-run RAM (first-execution bytes and loaded bytes still win). Qattro: 1,891 instructions translated before, 29,532 after; 6 undecodable addresses reached before, 1 after.
4. **Unknown registers.** A write-only or unknown custom register now reads $FFFF in `hardware.rs` and `intentrt.c`, as the reference harness's `chipset.c` does (CLXDAT and DSKBYTR stay 0).

The G5 check itself is now stronger: at least 3000 ticks, and a stall check (the port's picture still for the last 300 frames while the reference's changed 5 or more times there fails as `fail:stall`). Packaging adds a scripted-input play check (Ryzen `playcheck.sh`).
