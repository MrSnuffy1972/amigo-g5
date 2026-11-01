# Game 2271 native: Castlevania (Konami 1990, Amiga conversion by Novotrade)

Started 1 Nov 2026 by the batch 1 port agent (p2271). Crate `rust/games/g2271` on `native_engine`; the asset pack,
extractor, tools and function map live outside the repo (Ryzen `/srv/bulk/amigo-native/02271/`). No game bytes,
assets or disassembly are in the repo. From part 3 (Ryzen's /srv/bulk being full) the current extractor and pack
are at `/srv/fast2/native/b1-2271/tools/extract2271.py` and `/srv/fast2/native/b1-2271/pack`.

## Status: parts 1, 2, 3a and most of 3b committed; release r2

| Part | Content | State |
|------|---------|-------|
| 1 | File formats, extractor, asset pack, crate, picture model, boot text page, title menu | done |
| 2 | Stage engine (room set-up, scrolling, collision map, stairs, exits) and Simon (walk, jump, crouch, whip, stairs, knock-back, death) with the status panel | done (1 Nov 2026) |
| 3a | The object system (`objects.rs`): enemies and their behaviours, bosses, candles, items, sub-weapons, shots, spawners, platforms, walls, hidden treasures, hits, score and extra lives, the orb, tally and next block | done (1 Nov 2026) |
| 3b | Sound and music (done), the pause banner (done), the high-score table (done), the controls as +D14E reads them (done), the screen files' load time (done); attract sequence, saved game, door and map pictures, ending | partly done (1 Nov 2026) |
| 4 | Verification against the reference harness, play test, releases (Windows, Android) | stage 1 compared; r2 released |

Released: r1 (silent), r2 (with sound; see below).

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

## The objects (part 3a, `objects.rs`)

- The original keeps its object state in its data area: 16 records of 0x70 bytes (slot 0 the player), per-type
  records (spawner on, count alive, spawn timer and its reset), 10 item marks, the breakable walls (3 a screen file,
  each broken once a life), the hidden treasures, the moving platforms and the lifts. Most behaviour comes from its
  type table (0x2C bytes a type: speeds by direction, flags for waiting, jumping, falling, turning at ledges,
  keeping a distance, shooting, flying on a wave, health, score). The port keeps that state laid out as the
  original did (`Mem`, built at load from the program's initial data in the asset pack, never the repo), so each
  routine reads like the recovered one; the player stays `Simon`.
- Frame order as the original: timers (with the spawn timers), controls, the moving platforms, the player (the whip's
  last frame hits, or throws the sub-weapon with up held), the body-row marks (spawners on and off, hidden items),
  the objects (+5E7E: lifts, the six spawners, then each object's frame +3F32/+4528), previous positions, hits
  (+63DE: the player hurt by the block's damage, weapons and their combo score, pick-ups), the hidden treasures
  (+A4D0), then the frame (+1F8E): the columns scrolled on to are scanned for marks, objects are drawn in depth
  order, the orb's countdown leads to the tally and the next block.
- Random numbers: the original's generator (+DBC2, a linear congruential generator) and its `random_below`, which
  divides by n - 1 (so random_below(2) is always 0, and some "one in two" chances never happen; kept). The seed
  is the run's seed (the original seeded from the CIA timers).
- Not yet: the boxes drawn under platforms and ropes (+18FC's class 6 and kind 0x35 box), the boss bar's
  priority in the panel chain (it moves every frame here, the original only when no number changed).

## Sound and music (part 3b, `sound.rs`)

The game has its own replayer, not ProTracker: a tune ("SONG" .mus files with their samples in an instrument bank,
or the title module, "MODU" with its own samples after "SAMP") is a position list, a pattern table and rows of a
channel mask word and four-byte notes (period, instrument, command C volume, 0 arpeggio, 1 and 2 portamento, E
filter, F speed; a negative F stops the tune). +EEB8 steps it from the CIA-A timer A interrupt (14,640 ticks of
709,379 Hz, about 48.45 a second); the port runs those steps inside each vertical blank by the same clock. Effects
are IFF 8SVX files in 15 slots (+A96E, +F364); +AC24 plays the 15 numbers that have a sample, on channel 0 or 1,
the first one no effect holds; the effect borrows the music's channel until the sample has played (the audio
interrupt, modelled from the sample's length). +ACB0 fades the music out at 4 a step and the main program spins
until it has stopped before the next tune: the port waits the same frames with the picture held. Tunes: the
title module at boot (it fades at 2 a step while the rest of the sound loads), tune 3 on the title, the block's tune
on entering a block (and again on entering a room when a tune has stopped), the boss's (7) and the orb's (11), tune
6 on the high-score table. Quirks kept: the portamento slides a per-channel word no note sets, the effect channel's
commands are compared without masking the instrument bits, the effect periods are computed with the NTSC clock and
played on the PAL one, a tune muted under an effect leaves that channel borrowed until the next tune. The pack
carries the disk's sounds/ files as raw sample assets (`snd_<name>`), read by the game's own loaders.

## Pause and high scores (part 3b)

- Pause (+AFCA): the music is switched off (volumes go to nothing at its next row), the disk's pause message (the
  crack's banner "Q U A R T E X") is shown over the play area until a button, then the picture and the music come
  back. The typed cheat code the original checked on the pause screen (+D394, "661017") is not ported: no typed
  keys; the developer cheats stay in G2271_CHEATS.
- "Hall Of Blame" (+9CA6, after the game ends unless it was quit): screen file 31, ten entries of score and name,
  the new score in its place, the name entered, the table saved, then until the tune ends or a button. The table is
  kept in the engine's save store (`scores`) instead of SCORES.DAT. The original read the name from the keyboard
  (+D27A); the native build takes it from the joystick: up and down choose a letter, fire adds it, the second
  button removes one, start (Return, pad X) or ten letters end it.

## Controls

The original reads the joystick as +D14E does, and the port now does the same: up is also the jump bit (0x21), so
pushing up jumps (and climbs stairs); the one fire button is counted over the reads: a tap fires when it is
released (the whip) and a hold fires with up on its fourth read (the sub-weapon). The second button (Alt, Shift,
Z, pad B) is a second way to push up. Return or pad X start on the title; Escape returns to the title; the pause key
shows the pause banner. (Parts 2 and 3a had jump on the second button only and the whip on the press: the
reference showed the difference, see Verification.)

## Verification so far

- Extractor: every packed file depacks with a matching checksum; screen and shape files parse to their last frame;
  title menu and room 0 rendered from the pack match the reference harness's frames by eye.
- Headless: boot page, title menu (cursor eases down and up under scripted input, fire starts) checked from frames.
- Part 2 (this port's own behaviour, not yet compared with the reference): unit tests on synthetic maps (falling,
  landing, walking into a wall, a jump, time out); scripted runs on the real pack (tests/scripts/p2-walk.txt and
  p2-exit.txt): the courtyard scrolls, Simon walks, jumps, whips and crouches, and the courtyard's exit leads into
  the castle hall (room 1). Frames checked by eye.

- Part 3a (own behaviour, headless on the real pack, frames checked by eye; not yet compared with the reference):
  p3-candle.txt (a fire bowl breaks, its big heart is taken: hearts 5 to 10); p3-hall.txt (the hall's zombies and
  panther spawn, hurt the player with knock-back, die to the whip for 100; the dagger is picked up; with no cheat
  the player dies, a life is lost and the stage restarts in room 0); with G2271_CHEATS=nocollision,room=5 the giant
  bat appears at its mark, its bar drains, it dies for 3000, the orb is taken, the tally runs and stage 2 (room 8,
  block 1, time 400) begins. Unit tests: random_below's quirk, the generator, objects made and freed.

- Part 3b and 4 (1 Nov 2026), against the reference harness (b1-2271 copy, `tools/ref.sh` with STATELOG,
  WAVOUT and RNGLOG; c-tools scripts in /srv/fast2/native/b1-2271/c-tools):
  - Sound: the replayer's position, row and tick match the reference's frame for frame within one frame (the CIA
    timer's phase against the vertical blank is not modelled) for the title module, the title's tune and block 0's
    tune; the fades take the same frames (74 and 32); the mixed audio's spectrum correlates 0.985 to 0.997 with the
    reference's and its level is within 5%.
  - Stage 1 with the reference recording play1 (its joystick input moved to the native timeline, the generator
    seeded with the reference's seed at the stage start, 0xB3D3E515): x, y, map column, room, time, hearts and
    lives are equal frame for frame for 1,200 ticks of the courtyard and, after the room change, for 1,100 ticks
    of the next room, where the reference takes one more frame (a frame the original drops under load, not
    modelled) and the runs part. The score changes one tick earlier than the reference's after a kill (tolerance:
    1 tick). All 45 of the reference's generator draws from the stage start are equal, in order.
  - What the comparison corrected: the controls (+D14E, above) and the screen files' load time. The original spends
    about a fiftieth of a second for each 51 packed bytes depacking a screen file with the screen black (measured:
    VAN0 76 frames, VAN1 203); `stage::load_frames` keeps that cost, so a room change takes the original's time.
  - Not compared: stage 2 (the reference recordings do not reach it; the original's room cheat needs typed keys).
- Play test (native, scripted; p3b-boss.txt, p3b-gameover.txt): with G2271_CHEATS=room=5 only (no collision cheat)
  Simon walks to the giant bat, whips it to death (3000), takes the orb and the tally runs into stage 2 (room 8,
  block 1, tune 1); deaths lose a life and restart the room (3 to 2 to 1 to 0); the high-score table shows on screen
  file 31. Not run: stage 1 from the start to the boss without the room cheat, and a name entered on screen (the
  scripted runs ran out before a game over with points; the table and ranking have unit tests).

## Release r1 (1 Nov 2026, commit 68d9909)

`02271-castlevania-r1.apk` and `02271-castlevania-r1-windows.zip` (release.sh with `--pack-dir
/srv/fast2/native/b1-2271/pack`). Windows (WinPC): the front door's countdown starts the game, the boot page and
the title menu draw correctly, the window closes cleanly (exit code 0); keys posted to the window (PostMessage, and
SendKeys after AppActivate) did not reach it, so play under control was not tested on Windows. Shield: untested
(no adb binary on Ryzen or WinPC to check whether it is idle).

## What remains (exact)

Part 3b: the attract sequence (+9158, with +97F2, +980C: the castle and title pictures with the bats; the title
module now fades straight after boot because of it), the save and load of a game (+0858, +0884, +08FA, +099E: the
title's "Continue a Saved Game" says "No Game Saved"), the door and stair-exit pictures and the castle map between
blocks (+A162, +A21A, +A2FA, +A34C, +A3E0, with their tunes 9 and 10), the ending (+99E6, tune 12) and the next loop
(+05E2), the platform and rope boxes of +18FC, the effect slot release (+F422). Part 4: stage 2 against the
reference; frames the original drops under load. The function map (/srv/fast2/native/b1-2271/function-map.md,
copied to /srv/bulk/amigo-native/02271/) lists all 594 routines: 208 ported, 315 not ported with reasons, 71 to do.
