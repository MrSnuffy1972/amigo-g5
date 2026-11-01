# Game 2271 native: Castlevania (Konami 1990, Amiga conversion by Novotrade)

Started 1 Nov 2026 by the batch 1 port agent (p2271). Crate `rust/games/g2271` on `native_engine`; the asset pack,
extractor, tools and function map live outside the repo (Ryzen `/srv/bulk/amigo-native/02271/`). No game bytes,
assets or disassembly are in the repo.

## Status: part 2 of 4 committed (stage engine, Simon, panel)

| Part | Content | State |
|------|---------|-------|
| 1 | File formats, extractor, asset pack, crate, picture model, boot text page, title menu | done |
| 2 | Stage engine (room set-up, scrolling, collision map, stairs, exits) and Simon (walk, jump, crouch, whip, stairs, knock-back, death) with the status panel | done (1 Nov 2026) |
| 3 | Enemies, bosses, items, sub-weapons, timer, score, continue and game over, sound (.mus replayer, samples, title module) | to do |
| 4 | Verification against the reference harness, play test, releases (Windows, Android) | to do |

Not released yet: a release needs parts 3 and 4 (without enemies the build would not be the game).

## The program (from the recovered code)

- One AmigaDOS executable (compiled C, small-data model): code, an initialised data area that holds every table,
  and zeroed globals. The globals are addressed relative to one base register; the port names them by meaning.
- Main program: start-up, title and attract, the manual code lookup (crack: always passes), then per game: stage
  set-up, the frame loop (12.5 frames a second) (timers, controls or demo, player, enemies and objects, previous positions, stage
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

## The stage engine (part 2, `stage.rs`)

- Timing: during a stage the game draws a frame every four vertical blanks (the frame-rate word is set to 4 after
  the title), so the stage runs at 12.5 frames a second; the title and fades run every blank. The display's scroll
  eases towards the map position two pixels a blank, so a column step (8 pixels) takes one game frame.
- Frame loop, in the original's order: timers (every 10 frames the time counts down, below 30 a warning, at 0 the
  death), controls (none while dying), the player, objects (part 3), previous positions, then the frame: shapes, panel
  changes, the four-blank wait, the end of a death (a life lost, the stage's restart room, or the game over) and a
  room change.
- Attribute map: the player reads a 3 by 6 window (head to the floor under the feet). Bit 0 solid, bits 1 to 3 stairs
  and platforms, 2 and 4 the stair cells; negative bytes are marks: 0x30 + n is exit n of the screen file (a table
  of 4 exits a screen file: room, door kind, mode), 0x00/0x10 spawn triggers and 0x50 items (part 3), 0x7F the fade
  point a stair exit looks for.
- Simon: walking moves 8 pixels a frame, scrolling the map once he is past column 18; stairs move a row and a column
  a step; the jump follows a 10-step table (up 3, 2, 0.8, 0.75 rows, the top, then down); falling drops up to three
  rows a frame; the whip runs its animation and marks its last frame for part 3's hit test; knock-back follows a
  6-step arc away from the hit; death plays the facing's death animation, then a life is lost and the stage's
  restart room is entered (game over when the lives run out: back to the title until part 3's continue screen).
- Animations come from the program's table: 9 frames a row (99 ends a row) and, for each row, the next row by move
  state (still, up, left, right, the four stair directions).
- Panel: the panel picture, score (7 digits), time (4), hearts, lives and stage (2 each) drawn when they change, and
  the two health bars (a segment every second frame when health changes).

## Controls

Joystick game: directions walk, crouch and climb stairs; fire (Space, Ctrl, pad A) whips; the second button (Alt,
Shift, Z, pad B; the original's second port bit) jumps; Return or pad X start on the title; Escape returns to the
title. The pause screen and the cheats menu come with part 3.

## Verification so far

- Extractor: every packed file depacks with a matching checksum; screen and shape files parse to their last frame;
  title menu and room 0 rendered from the pack match the reference harness's frames by eye.
- Headless: boot page, title menu (cursor eases down and up under scripted input, fire starts) checked from frames.
- Part 2 (this port's own behaviour, not yet compared with the reference): unit tests on synthetic maps (falling,
  landing, walking into a wall, a jump, time out); scripted runs on the real pack (tests/scripts/p2-walk.txt and
  p2-exit.txt): the courtyard scrolls, Simon walks, jumps, whips and crouches, and the courtyard's exit leads into
  the castle hall (room 1). Frames checked by eye.

## What remains (exact)

Part 2 simplifications to finish with part 3: the door and stair-exit pictures (+A162, the fade point use), the
castle map between blocks (+A21A, +A2FA), the moving platforms of screen files 6, 8 and 16 (+33C2), the column
spawn marks while scrolling (+25C6, +273A). Part 3: +5E7E and +4528 objects and enemies, +63DE stage clear,
+6976 bosses, +6D4C/+7CE4 whip hits and sub-weapons, +8A46/+6068 spawns, +60E6 items, +9CA6/+99E6 game over and
stage clear, continue, +EEB8/+ED3A music, +F364 samples, the attract sequence (+9158), high scores (+B2BA), pause
and the cheats menu (+AFCA). Part 4: reference comparison, play test, releases. The function map
(/srv/bulk/amigo-native/02271/function-map.md) lists every routine's state: 107 ported, 192 not ported with
reasons, 295 to do (all part 3).
