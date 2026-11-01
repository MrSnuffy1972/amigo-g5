# Game 608 native: E-Motion

Added by native batch 1 (1 Nov 2026). E-Motion (US Gold / The Assembly Line, 1990) is a physics puzzle game: a ship
pushes coloured balls round a screen that wraps at every edge. Balls of the same colour destroy each other (scoring),
balls of different colours bounce and make a pod of the third colour, pods give energy when the ship runs over them
and hatch into new balls if left, tethers pull pairs of balls together, and every ball's fuse burns down until it
explodes and costs energy. 50 levels, bonus levels (blue, yellow and red pods, a clock), five secret bonuses, a
secret level's spark that lets the ship destroy balls by touching them, one or two players, practice and demo modes,
and the high-score table.

The native version is `rust/games/g608` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md)). It is written
from the recovered code (`/srv/fast/amigo/recovery/00608`, the real game is the raw file `mainbit` loaded at $400) as
typed Rust: no guest memory, no 68000 registers, no Kickstart, no chip registers, no port reads, no copper, blitter or
bitplane emulation. Pictures, sprites, font, samples, music block lists, levels, texts and tables come from the asset
pack at run time (`/srv/bulk/amigo-native/00608/extract608.py`, outside the repo; nothing of the game is in git).

## How it is built

- `game.rs`: the main program (attract loop, options, a game, the levels' main loop, bonus and secret screens, game
  over, high-score entry, carry on from level 25). `objects.rs`: the 64-slot object table and its per-kind routines
  (ship, exploding ship, tether spring, ball fuse, fragments, pods, hatching, fireworks, the spark), the collisions
  with the bars (flat sides and round ends) and between objects, exactly as the original computed them: 16.16
  positions and speeds, 16-bit multiplies and divides with the original's overflow and divide-by-zero results
  (`ops.rs`), its integer square root and its random numbers.
- `emo.rs`: the frame (what the vertical-blank handler did: frame counter, palette fades, controller bits, the bonus
  clock), the two screens and how a frame is shown: the playfield's colours 30 and 31 are two colours of a gradient
  band that changes every 14 lines (the copper list), over a dithered background. The four sound channels: effects
  play once on the next free channel; the title music is four channels of sample blocks played round and round.
- `draw.rs`: the drawing (tiles, bars, balls and ships that wrap round the edges, the tether lines, the 16-line font
  with its shadow, rounded corners, status line), into indexed 320 x 196 screens.
- `co.rs`: the main program runs on its own thread in lock-step with the engine's ticks (game 336's pattern), so the
  original's nested frame waits keep their shape.
- `intro.rs` and `soundmon.rs`: the crack's intro, kept as the game's intro piece (part of gaming culture): the
  Setrox/Alcatraz trainer menu with its rippling selected line, colour-cycling copper lines and scroller, its SoundMon 2
  music (a rewritten replayer: patterns, sample and synth instruments with ADSR, LFO and EG), then the crack loader's
  two pictures with their fades.

50 Hz timing is kept: the main loop's passes take two frames (swap_screens waits so a pass takes at least two), and
the work the original took frames to do at a level's start (clearing and copying the screens) is a measured cost.

## Protection removed

The only protection is a track-79 check (+$3D1C: two raw reads with the non-standard sync $4124, the second sync at
least 200 words after the first); the crack had already removed its call. Not ported: the native build behaves as
passed. The high-score load and save (own MFM read/write of track 78) are replaced by the engine's save store.

## Cracktro and cheats

The intro is the trainer menu: up and down pick a row (the original used the mouse), fire switches an option on or
adds one to the start level (repeating every seven frames while held), Fire 2 switches it off or takes one away, fire
on START GAME starts. Its options are the native build's cheats, all off at the start: unlimited lives, energy and
time and the start level act exactly as the trainer's code patches did (the lives store, the energy decrements and
the clock's seconds skipped; the start level replaces the game's level clear).

## Function map summary

232 recovered routines, 232/232 accounted for. mainbit: 186 routines, 140 ported, 10 protection removed (the track-79
check), 36 not ported because the engine does their job: the high-score disk routines (27, the save store), the
hardware set-up, copper and bitplane hand-over, the keyboard interrupt and its key-code routines, the
divide-by-zero vector and an error trap that a corrupt channel counter would reach. Crack: 35 routines of the trainer
menu, its music player, the trainer's start-level hook and the loader's pictures ported as the intro; 11 not ported
(the depacker, boot block, file loading and memory copies: the pack holds the files unpacked; the menu's OS copper
restore). The full map (routine, unit, range, native code, reason) is kept outside the repo with the asset pack.

## Verification

Reference: the nv608 68000 reference harness (`/srv/fast2/native608/hw`, with HLE_OFF_PC=400) running the original
from power-on with the recording `pa` (level 1 played to the energy running out, a ship lost and the level started
again), logging named state each frame (frame counter, level, lives, energy, energy owed, score, the first seven
object slots' kind and position). The native headless runner got the same joystick input, placed relative to the
level's start (the two intros differ in length, so the runs are aligned at the frame player one's ship appears).
Tools outside the repo: `conv608.py` (the script), `cmp608.py` (the comparison).

| Check | Result | Tolerance |
|---|---|---|
| Named state (27 values) exact, same frame, from the level's start | 2063 of 2063 frames, then 2040 of 4881 | exact |
| Covers | one life to its end: balls hatching, tethers, fuses and explosions, energy running out, the ship's explosion, the lost life, the level restarted | |
| After frame 2063 | positions drift by a pixel and the runs part (sub-pixel fractions, which the log cannot show, differ after the restart) | |
| Frames by eye | title, high scores, options, play, the exploding ship and the restart side by side with the reference | |

Fixed on the way by the comparison: object slots keep the fields a level does not set (their fractions show
through, as on the original, and slots start with the file's bytes there); the measured costs of a level's start.

Play test: `cargo test -p g608` with `G608_PACK` set: START GAME starts, fire on the title and the options reaches
level 1 with the ship in play and three lives, then a bot plays 30000 frames: the frame counter keeps moving (no
freeze), nothing quits, and the levels move on from level 1 (Karl reported exactly that bug for the old build).

## Known differences

- The second player's controller has no input of its own (the engine has one player's controls); two-player mode
  runs, with the second ship idle.
- The keyboard's letters do not reach the native build: the high-score name is entered with the controller (up and
  down choose a letter, right takes it, left rubs out, fire ends); the developer cheat word and its F-keys cannot be
  typed.
- The random numbers start from the file's seed (the original mixed in the display's beam position at start-up).
