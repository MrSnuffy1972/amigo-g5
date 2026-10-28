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
