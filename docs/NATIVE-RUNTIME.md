# Ami-Go native runtime: clean-sheet design (decided 14 Oct 2026)

Karl's decision, 14 Oct 2026: shipped ports are clean-sheet. No Amiga custom chip (copper, blitter, Denise
bitplanes and sprites, Paula, CIA) is emulated in a shipped .exe or .apk. The game's 68000 code is translated to C
ahead of time; everything it asks the custom chips to do is done by modern, multi-core, GPU code (Direct3D on
Windows, Vulkan or OpenGL ES on Android). Take advantage of multiple cores where they are present.

Sources: Karl's three design documents (14 Oct: clean sheet, command-stream recording with asynchronous GPU replay,
shared Windows/Android core) and the research report "WHDLoad and clean sheet ports" (reports/, 14 Oct), which
covers N64Recomp/RT64, XenonRecomp, the snesrev zelda3/sm ports, sm64-port, retro-foundry/amiga-recomp, SWIV-Amiga,
Speedball 2 Decomp, swos-port and the WHDLoad 20.1 developer package.

## How the existing kit maps onto this

The current kit (kit/amigo/port, k13) has a functional chipset model in runtime/chipset.c. It is a build-time tool
from now on: the analysis harness, G0/G1 discovery and the reference for verification. It is not linked into a
shipped port once the native runtime covers that game. During the change-over a port may still use it; the gate
contract says which runtime a build used, and G4 requires the native runtime.

## Architecture (one source tree, Windows and Android)

```
core/ (100% shared, C, no platform calls)
  game/            translated 68000 code (m68k2c), one C function per block, no interpreter in release builds
  hw_intercept     every $DFF000-$DFF1FE and $BFxxxx access goes to a shadow register file; writes that start or
                   change something emit a command packet stamped with an estimated beam position
  cmdstream        per-frame lock-free ring buffer of packets (copper pointer, blit start, palette, bitplane
                   pointers, BPLCONx, DIW/DDF, sprite pointers, audio channel start/stop/period/volume)
  blit_exec        exact C implementation of each blit whose destination the CPU reads back (collision masks,
                   off-screen buffers, fonts): SIMD (AVX2 / NEON through one portable header), run synchronously
  frame_builder    turns the frame's packets and memory into a display list: per-line register tables from the copper
                   list (WAIT/MOVE analysed, not stepped), playfield layers, scroll, sprites, priorities
  paula_dsp        sample playback from the audio packets, BLEP steps and the A500/A1200 filters (pt2-clone code)
  timing           fixed 50 Hz (PAL) or 60 Hz (NTSC) logic tick, accumulator, previous/current state for interpolation
  resload          WHDLoad resload API reimplemented in C (below)
platform/windows   Win32/SDL2 window, Direct3D 11/12 renderer, WASAPI/XAudio2, XInput, menu.c front end
platform/android   NativeActivity/SDL2, Vulkan or GLES 3.x renderer, AAudio/Oboe, touch overlay, Choreographer pacing
shaders/           written once (HLSL), compiled with DXC to DXIL (Windows) and SPIR-V (Vulkan); spirv-cross for GLES
```

Threads: logic thread (translated game code, one 50 Hz tick at a time) -> packet ring -> worker pool
(planar-to-chunky in bands, copper table build, blit batches) -> render thread at the display's refresh rate.

## The rules that keep it correct (from the research)

1. A blit runs exactly, in C, at the moment BLTSIZE is written, when anything later reads its destination: the CPU
   (collision, fonts, decompression), another blit, or the copper. Only blits whose destination is purely displayed
   may become GPU quads. A build-time memory-access trace (harness) decides per blit routine; the result is recorded
   in game.json, not guessed by a model.
2. Copper lists are analysed per frame into per-line tables (colour, scroll, bitplane pointers, sprite pointers).
   Mid-line changes, copper writes to blitter registers, copper interrupts and lists the CPU edits while they run are
   detected at build time and get their own handling; they are not silently approximated.
3. Reads of VPOSR/VHPOSR, INTREQR, DMACONR (blitter busy) and CIA timers must return values consistent with the
   beam estimate, so the translated code needs a cycle estimate per block (m68k2c already counts cycles).
4. Mapping a register address to a packet type is a table lookup in code (ladder rung 3). Models are used only for
   genuine judgement: classifying an unusual copper effect, choosing a touch layout, naming a routine's intent when
   a trace is ambiguous. Only local models (Laya, or the fleet's local LLMs) see game code; JEV is remote and gets
   shared-tool questions only.
5. No per-game workaround inside the renderer. Game-specific facts live in game.json and hooks.c.
6. A finished port builds and runs with the interpreter fallback compiled out.

## Presentation

Logic runs at exactly 50 Hz. The render thread presents at the display's rate. Scroll registers, sprite positions
and blits converted to quads are known per tick, so they are interpolated between the previous and current tick;
pixels drawn into bitplanes by exact blits cannot be, so those games show 50-on-60 judder on fixed-60 Hz phones
(Android phones generally have no 50 Hz mode; TVs and some PC monitors do, and VRR displays avoid it). This is
measured per game, not assumed.

## WHDLoad (89 of the queued games, parsed 14 Oct)

Measured on Karl's own queue, 14 Oct: 89 slaves parsed, 65 direct (no Kickstart), 19 kickemu (16 Kickstart 1.3,
3 Kickstart 3.1), 5 with a multi-Kickstart table (kickemu). Direct slaves run with a clean-room C implementation of
the resload API: the 36-entry jump table (about 18 calls cover most games: Patch, Control, DiskLoad, SaveFile,
GetFileSize, Abort, LoadFileOffset, CRC16, LoadFileDecrunch, Examine/ExNext, SetCPU, PatchSeg, LoadFile, Relocate),
a patch-list interpreter (encoding defined by the documented macros), and decompression through temisu/ancient
(BSD 2-Clause). Our own constants are defined from the documented values; WHDLoad's include files are not copied
(all rights reserved). At build time the slave and the game's loader run on the harness to produce the memory
image; in the shipped port the same code is translated and calls the same C resload functions.
Kickemu slaves boot Kickstart 1.3 or 3.1. Karl's decision, 14 Oct: zero ported ROM code, all clean sheet. The
exec, dos, graphics and other library calls those games make are reimplemented in C (only the calls the games
actually use, found by tracing them at build time); no Kickstart code is translated into any port.
Reference emulator: Karl approved, 14 Oct, an Amiga emulator at build time only, as the oracle for verification;
never inside a shipped game.

## Verification (G3 and every later gate)

The snesrev zelda3/sm method, generalised: at build time the port and a reference emulator (FS-UAE, WinUAE or
vAmiga; build tool only, never shipped) run the same input frame by frame; compare chip RAM, fast RAM and the
command stream; learn which differences are harmless; re-sync after a mismatch and save a replayable snapshot.
Game-state equivalence (positions, scores, collision arrays) is what must match; framebuffer bytes need not when
the display is produced by the GPU.

## Additions from the review of 14 Oct 2026 (agreed parts)

### Game tiers, in build order
1. Tier 1, direct WHDLoad slaves (65 of 89 parsed): C resload + ancient. No ROM, no OS. Built first.
2. Tier 2, kickemu slaves (24): no Kickstart code in any port (Karl, 14 Oct). At build time the reference emulator
   runs the slave with Karl's own Kickstart and logs every library call (library, LVO offset, arguments, caller).
   Only those calls get a C implementation. Most such games use the OS only as a disk loader before taking over the
   hardware: their calls are memory and file calls (AllocMem, Open, Read, Seek, Close, Lock, LoadSeg), which map to
   the same C file layer as resload. Graphics and Intuition calls, where a game keeps the OS, are listed per game
   and built in order of how many games need them.
3. Tier 3, custom trackloaders (8): the Speedball 2 Decomp pattern. The reference emulator boots the original disk;
   at each point where the loader finishes a read (detected by the disk DMA going idle and the CPU jumping into the
   loaded area), chip and fast RAM are captured. Each capture becomes a data asset of the port, and the loader's
   call sites are replaced by C that installs the matching capture. Games that load levels later get one capture
   per load point, found by playing the recorded input through the reference run.

### Thread boundary (fixed)
- Thread 1, logic: translated 68000 code, one 50 Hz tick at a time, single-threaded and deterministic (interrupt
  order and register timing make it unsafe to split). Register writes become timestamped packets in a lock-free
  ring. Exact CPU-side blits run here, synchronously, because the next instruction may read their result.
- Pool, translation: as many workers as the device has cores minus two, at least one. Planar-to-chunky in
  32-line bands (portable SIMD), copper list to per-line tables, display-only blits to quad batches, audio mixing.
- Thread 3, render and present: Direct3D on Windows, Vulkan or GLES on Android, at the display's refresh, with the
  interpolation factor from the 50 Hz accumulator.

### Platform portability
- SIMD through one portable layer: SIMDe (MIT) or our own small header; AVX2/SSE4.1 on x86-64, NEON on ARM64,
  plain C fallback. The same source on both targets.
- Shaders written once in HLSL, compiled with DXC to DXIL (Direct3D 12) and SPIR-V (Vulkan); spirv-cross for GLES.
- Android pacing: the frame loop is driven by the display's vsync (Android Choreographer, through the Android
  Frame Pacing library Swappy where SDL does not already provide it). The 50 Hz tick is decoupled from the 60, 90
  or 120 Hz frame callback in every game; interpolation of scroll, sprites and quad blits is added on top where the
  game allows it.
- Alignment: ARM64 builds are tested with -fsanitize=alignment (and undefined) in the build-time test runs; the
  translator emits byte-safe loads and stores for 68000 word and long accesses that can be odd-aligned.

### Where models help, and where they must not be used
- The memory-access trace decides whether a blit's result is read back. That is a measurement and stays the
  authority (ladder rung 3). Where the trace never exercised a blit routine, Laya (local, on the LAN) gives a typed
  answer, display_surface_only or cpu_readback, the routine is built the safe way (exact CPU blit) until the
  lockstep run confirms otherwise, and Laya's answer is logged against the outcome so its accuracy is measured.
- JEV is a remote service, so it never receives game code, disassembly or game-derived C (standing rule: no
  game-derived code leaves the LAN). JEV can be asked about shared-tool code only.
- Equivalence of translated code is decided by tests, not by a model's score: the instruction-by-instruction
  difference test against the reference 68000 interpreter, the lockstep frame comparison against the reference
  emulator, and the sanitiser runs. A model's opinion of a C block is logged as advice only.
- Ports are private builds for Karl (never shared or distributed), so no commercial-distribution split is needed;
  Tier 2 games need no ROM in the shipped port anyway, by Karl's decision.

## Additions from the second review (14 Oct 2026, agreed parts)

### Guest memory is always exact; GPU promotion is an optimisation
- Every blit runs through the exact C blitter (portable SIMD) into guest chip RAM at the BLTSIZE write, in every
  verification run and by default in release. Amiga games constantly read blit results without the CPU touching
  them: save-under and restore blits, and cookie-cut draws that read the playfield back through channel C.
- A buffer may be promoted out of chip RAM to an unbacked GPU render target only when the build-time blitter
  dependency graph (every blit's sources and destination, every CPU read, every copper and DMA read, measured in
  the reference runs) shows that nothing reads it except the display and self-contained cookie-cut composites onto
  it. This is a measured graph property, decided by code; the lockstep run with the promotion on must still match.
- Release ports may keep exact blits for any game where promotion is not proven; correctness first.

### Hardware collision (CLXDAT $DFF00E, CLXCON $DFF098)
- The trace records whether a game reads CLXDAT. If it does, the runtime computes the collision bits for the
  sprite and bitplane combinations CLXCON selects, from the frame's sprite positions and the bitplane data up to
  the beam position of the read, and returns them. Games that never read it pay nothing.

### Kickemu games without ROM code
- Library calls are logged in the reference run at the library jump tables (the A6-relative JSR into each
  library's vector table), named by offset from the official function descriptor (.fd) lists: a table lookup,
  not a model question. The kickemu sources (kick13.s, kick31.s, kickfs.s) are public domain and are the
  reference for how WHDLoad maps DOS file calls onto resload; our C shim does the same: exec memory and interrupt
  calls, dos file calls on the resload file layer, LoadSeg through the hunk loader and ancient, and the graphics
  calls games use to shut the OS display down (LoadView(0), WaitTOF, OwnBlitter, DisownBlitter).

### Code discovery beyond the trace
- Coverage traces alone miss levels and routines the test play never reached, and a release build has no
  interpreter fallback. So every payload that resload or the OS shim loads, decrunches or patches in the reference
  run is dumped whole after patching, and recursive-descent disassembly (Ghidra plus the translator's own
  discovery) runs over the entire dump, not just the executed PCs. Indirect jumps are resolved by Ghidra's switch
  analysis and by the translator's table recogniser; what remains goes to a local model (never JEV) as a typed
  question (word-offset table, long-pointer table, callback structure, data), each answer verified by translating
  the targets and checking they decode as valid code. Unresolved targets are listed; a development build logs any
  dispatch miss with the game state so the next build includes it.

### Pipelining safely
- Frame N renders while frame N+1's logic runs. At the vertical-blank barrier the logic thread copies the chip RAM
  the frame's commands point into (bitplanes, copper lists, sprite data, blit sources) into a double-buffered
  render snapshot; the renderer reads only the snapshot and the frame's command stream. Copying 512 KB to 2 MB per
  frame is a small cost on current hardware (an estimate; to be measured on the target phone).

### Interpolation on fixed-refresh displays
- Android pacing through Swappy/Choreographer. Scroll (BPLxPT, BPLCON1), hardware sprite positions (SPRxPOS) and
  promoted quad blits are interpolated automatically when their per-tick deltas are steady; the pre-start menu gets
  an optional frame-blend setting for games whose movement is drawn into bitplanes.

### Lockstep masks
- A differing address is never masked just because it differs every frame. It may be masked only when the trace
  shows that no game code reads it before it is next written (pure scratch, timers, stack slack). An address that
  is read, such as a random-number seed, is a desync to fix, not a mask. A local model's view of the writing code
  is logged as advice only.

## Decisions from the outside architectural review (14 Oct 2026)

### Presentation path (Karl: CPU compose + SDL2 GPU)
- Each frame is composed on the CPU into an RGBA8888 buffer: per-line tables, bitplanes, HAM/EHB, dual playfield and sprites. It uses portable SIMD on the worker pool, in bands.
  - The result is deterministic, so the G3 frame hash compares exactly what the player sees.
- SDL2's renderer presents the frame: Direct3D 11 on Windows, GLES or Vulkan on Android. It does the scaling, CRT look and display-rate interpolation.
- This replaces the raw Direct3D 12 / Vulkan renderer in the earlier sections. Reason: a raw D3D12 device is unlikely to start under the Wine/Xvfb G2 check, and CPU composition keeps every platform's pixels identical.
- Measured cost basis: the whole current kit frame (CPU, chipset and composition) takes about 0.8 ms on one Ryzen core. The reviewer's 0.15 ms figure for composition alone is an estimate, not a measurement.

### Stalled games (Karl: 3 tries or 45 minutes without progress)
- A model session that makes no progress on a gate for 3 sessions, or for 45 minutes, triggers this sequence:
  1. the port kit tries the gate with code;
  2. the free-model fault analysis runs (shared-tool code only);
  3. the game is parked at the end of the queue and its model is freed.
- When the game reaches the front again, it resumes with the analysis hint.
- Progress is currently "the check output changed". It becomes "the first divergence point moved later" once the bisection tool exists.

### Review items, measured before acting
- **Turrican 2 G3 drift:**
  - The review's mechanism (cycle batching at hardware reads) did not hold: the translator already charges per instruction, and there are no hardware reads in the divergent window.
  - Measured causes, all fixed in k16:
    - unequal CPU speed factors between port and reference;
    - DBcc and MULU/MULS cycle costs;
    - the harness not carrying Musashi's per-line overshoot.
  - Result: 99.87% match at equal 1.0x speed; 99.2% at the shared 1.1x default.
- **Turrican 3 interpreter share:** confirmed. 1605 PCs ran different code at different times. Next step: per-version capture in the harness and versioned translation, with a dispatch keyed on the first differing word; immediate-operand SMC reads from guest RAM.
- **ExpMem placement:** must stay inside the 68000's 24-bit address space (the review's $10000000 is not addressable). Use the slow-RAM or Zorro II fast-RAM range.
- **Kickemu ROM-less boot:** the synthetic library bases with trap dispatch match this document's kickemu plan. Two corrections:
  - Dispatch by a table of (base, LVO), not by page arithmetic: negative LVOs fall into the previous page.
  - Size each jump table from the .fd files, not a fixed 1 KB.
- **JEV features:** numeric aggregate features only (ratios, counts, strides). No addresses, opcodes or bytes. This is consistent with the LAN boundary.
