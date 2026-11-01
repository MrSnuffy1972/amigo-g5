# Game 2271 native: Castlevania (Konami 1990, Amiga conversion by Novotrade)

Started 1 Nov 2026 by the batch 1 port agent (p2271). Crate `rust/games/g2271` on `native_engine`; the asset pack,
extractor, tools and function map live outside the repo (Ryzen `/srv/bulk/amigo-native/02271/`). No game bytes,
assets or disassembly are in the repo.

## Status: part 1 of 4 committed (formats, extractor, pack, program frame)

| Part | Content | State |
|------|---------|-------|
| 1 | File formats, extractor, asset pack, crate, picture model, boot text page, title menu | done |
| 2 | Stage engine (room set-up, scrolling, collision map, stairs, doors) and Simon (walk, jump, crouch, whip, stairs, damage, death) with the status panel | to do |
| 3 | Enemies, bosses, items, sub-weapons, timer, score, continue and game over, sound (.mus replayer, samples, title module) | to do |
| 4 | Verification against the reference harness, play test, releases (Windows, Android) | to do |

Not released yet: a release needs parts 2 to 4 (the build would not be the game).

## The program (from the recovered code)

- One AmigaDOS executable (compiled C, small-data model): code, an initialised data area that holds every table,
  and zeroed globals. The globals are addressed relative to one base register; the port names them by meaning.
- Main program: start-up, title and attract, the manual code lookup (crack: always passes), then per game: stage
  set-up, the 50 Hz frame loop (timers, controls or demo, player, enemies and objects, previous positions, stage
  clear test, panel, scroll and redraw, frame wait), and on its end game over, continue or the stage-clear sequence.
- Frame waits sit deep in nested routines (fades, messages, title, stage clear), so the port runs the main program on
  its own thread in lock-step with the engine (`co.rs`, the pattern of game 336).

## Formats (all decoded by the extractor, checked on every file)

- Packed files (art/, screens/): the game's Huffman coder (16-bit checksum, 32-bit length, pre-order code tree, MSB
  first bit stream). Screen files then pass a run-length stage (a marker byte, then "marker, count, value" runs, a
  zero count ends).
- Screen files (38): three header words: the tile graphics (word offset), the tile map (width, height, then one byte
  a cell) and the attribute map (same size; collision, stairs, platforms and spawn marks). Tiles are 8 by 8 in five
  bitplanes (32 colours), 40 bytes each. Map rows 0 to 2 are under the status panel; the play area is rows 3 to 24.
- Shape files (art/, 36): frames of a 24-byte header (width in words, height, x and y anchor, clip width and height,
  two pointers filled at run time, bytes a plane), then four bitplanes and a mask. Shapes use colours 0 to 15; the
  original clears the fifth plane under the mask.
- Palettes: colours 0 to 15 are fixed sprite colours; colours 16 to 31 come from a table indexed by the room number
  (53 entries; the title menu uses entry 51). Fades step every colour one unit a frame for 33 frames.
- Room table (48 rooms): start scroll column, player column and row, facing, screen file.
- Font: 96 glyphs of 8 by 8 from the space character; drawn with an ink and a paper colour (five bits each).
- Display: a 352-pixel-wide picture (44 bytes a row), the 24-line panel unscrolled above a play area scrolled by
  moving the picture start; three pictures (a clean background and two shown in turn, restored box by box).
  The native model keeps the three pictures as indexed pixels in game coordinates (`screen.rs`).
- Sound (part 3): `.mus` tunes (chunked, "SONG"/"SAMP"), `.eff` IFF 8SVX samples, and `hellojarre.mod`, a
  ProTracker module with a "MODU" header (title music) for the engine's tracker.

## Copy protection and crack

- Manual code lookup ("ENTER PASSWORD FOR MISSION": a letter and a row from the manual's table, four tries). The
  crack made every answer pass. Native: not ported; the game goes on as if it had passed (no screen).
- No disk protection: the game reads its files through AmigaDOS.
- Crack intro: the Quartex text page that the startup-sequence typed for three seconds. Native: the boot text page
  in the CLI colours, wrapped to 40 columns, three seconds or until fire. The crack's name in the pause banner stays.
- Developer cheat (pause screen code): to become a cheats menu, off by default (part 3).

## Controls

Joystick game: directions walk, crouch and climb stairs; fire whips; the second button jumps (the original's second
port bit); Space or P pauses; Escape returns to the title.

## Verification so far

- Extractor: every packed file depacks with a matching checksum; screen and shape files parse to their last frame;
  title menu and room 0 rendered from the pack match the reference harness's frames by eye.
- Headless: boot page, title menu (cursor eases down and up under scripted input, fire starts) checked from frames.

## What remains (exact)

Part 2: +0ED2 stage set-up, +20EC/+25C6/+273A drawing and scroll columns, the VBlank scroll (+CA0A, +C60A, +C7A4,
+CB10), +350C/+31E8/+2FB2 collision probes, +386A player state machine with +2B36..+2DC8 movement and jumps,
+90EC/+9076 animation tables, +306C doors, +1A40/+24E4 panel. Part 3: +5E7E and +4528 objects and enemies, +63DE
stage clear, +6976 bosses, +6D4C/+7CE4 sub-weapons, +8A46/+6068 spawns, +1480 timers, +9CA6/+99E6 game over and
stage clear, +EEB8/+ED3A music, +F364 samples, the attract sequence (+9158), high scores (+B2BA), pause and
cheats (+AFCA). Part 4: reference comparison, play test, releases.
