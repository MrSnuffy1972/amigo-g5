# Game 2179 native: The Great Giana Sisters

Added by native batch 1 (1 Nov 2026). The Great Giana Sisters (Rainbow Arts / Time Warp, 1987) is a scrolling
platform game for one or two players taking turns: 32 stages and the dream's end, free diamonds and diamond blocks
(a life every hundred), power blocks whose items rise out of them (the power-up that turns Giana into the punk who
breaks bricks, three levels of dream-bubble shot: bouncing, returning and homing, freezing and destroying all enemies,
protection from thorns, an extra life), breakable and hidden bricks, multi-diamond blocks, warp blocks that skip two
stages, crumbling bridges, the lift, bonus stages entered through the floor, enemies that walk, fall, follow paths or
fly patterns, the stage clock, the stage-clear tally, and the high-score tables (all-time, saved, and today's).

The native version is `rust/games/g2179` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md)). It was
written from the recovered source ([g2179-recovery.md](g2179-recovery.md)) as typed Rust: no guest memory, no 68000
registers, no Kickstart, no chip registers, no port reads, no copper, blitter or bitplane emulation. All pictures,
stages, tables, texts, sounds and music come from the asset pack at run time (extracted outside the repo from the
WHDLoad install's disk image; nothing of the game is in git, the play-test scripts are joystick timings only).

## How it is built

- `giana.rs`: the program's state in named, typed fields (Giana's position, speed, jump and fall phases, collision
  summary, power counters, the shot, block and bridge slots, enemies, scores, turns), the frame clock and the drawing
  primitives, and the composition of the frame. The program runs on its own thread in lock-step with the engine
  (`co.rs`, the pattern of game 336), because the original waits for the beam deep inside nested routines (title,
  message screens, tally, name entry). `wait_line` is the only place a frame ends; the original's long loops are
  measured costs in display lines (`busy`), calibrated against the reference run.
- `flow.rs` (title, lives, turns, messages, tally, status bar), `player.rs` (movement, jumping, collision, hazards,
  shot, power items, lift, bridges), `blocks.rs` (all blocks and their animations), `enemies.rs` (enemies and items,
  animated tiles, building a stage), `hiscore.rs` (high-score screen and the boot program: logo and title picture),
  `music.rs` (the Beathoven 1.09 sequencer, both its game and boot copies, and the four sound channels the music and
  effects share).
- Every method keeps the name of the recovered routine it was written from, in its doc comment, so the function map
  reads straight across.
- Pictures are kept as the original kept them, as palette-index pictures it draws into: the whole stage's 2048 by 160
  playfield (built from its background pattern, scenery blocks and tiles, then changed by the game as blocks are hit),
  the status bar, the message and high-score screens. Bobs are drawn with their masks and their backgrounds restored
  next frame, in the original's order. The frame shows them as the display lists did (status bar over the scrolled
  playfield, the cloud sprites behind, the title's colour bar, the high-score screen's line wobble that settles after
  the scroll-in, the logo's colour cycling, and the interlaced hold-and-modify title picture).
- Music and effects are commands to the engine's mixer: a channel starts its block when switched on and then repeats
  the block the game last gave it, as the original's sound hardware did; the sequencer runs once a frame from the
  vertical blank. Paula channels 0 and 3 left, 1 and 2 right.

## Protection, cracktro, WHDLoad and the cheat

The recovery found no protection in the code (the original's protection was its custom disk format, which a native
build has no use for) and no cracktro. The WHDLoad install's glue is not ported (it loads tracks, patches the game,
fixes raster waits and delays); three of its behaviours are kept because they are part of the game as installed:

- The second joystick button jumps (pad B, Left Alt or Z), as the slave's joystick routine did.
- After the name entry the all-time table is saved at once (the slave replaced the "mouse button to save" wait), to
  the engine's save file instead of the disk.
- The trainer key (`]` or F9) turns life loss off and on; it is off at the start. As in the install, a session that
  used it no longer saves its high scores.

## Function map summary

302 recovered routines, 302 accounted for: 182 ported (all of the game's own logic and the boot program's
logo, title picture and music), 120 not ported with a reason: the WHDLoad slave and harness stubs, the floppy
track loader and writer and drive control, the display-list builders and display, interrupt and blitter set-up, the
picture decoder (run when the pack is built), memory allocation and pointer relocation, routines the original never
calls, and the byte-identical copies of boot routines and the forms of routines as the slave patched them in memory
(ported once, as the routine itself). The full map (routine, unit, range, native code or reason) is kept outside the
repo with the asset pack.

## Verification

Reference: the reference harness running the WHDLoad install from power-on, with RAM dumps at chosen frames (the
named state is read from the game's own variables at their recovered addresses) and the same scripted input through
the native headless runner (`examples/headless.rs`). Measured 1 Nov 2026:

| Run | Compared | Result |
|---|---|---|
| p1: one player, trainer on, 4000 frames (logo, title picture, title, get-ready, play, a death, the restart at the halfway point) | 29 named values (stage, scroll, position, speed, jump and fall phases, score, diamonds, lives, clock, power, shot, five enemy records) at 240 frames | 239 of 240 frames all exact at the same tick (the other is the title, where the player position is unused) |
| p2: two players, 14000 frames | same, 248 frames | all exact through player one's whole game (deaths, game over, the high-score screen, its scroll-in and wobble, the name entry letter by letter) up to frame 7866; there a 3-frame fire pulse falls 1 to 2 frames from a screen-change wait, the reference takes it and the native build waits for the next press, so the runs part |
| Playfield picture | the reference's own bitplanes and display-list pointers rendered from its RAM dumps, against the native frame at the same tick | 100% of pixels on 5 of 6 gameplay frames, 99.7% on the sixth (the reference dump caught the player bob half-drawn) |

Tolerance: named state exact at the same tick; picture within 24 per colour channel (the two expand 12-bit colours
differently). The reference harness's own frame dumps are used only for the screens that do not scroll (logo, title
text, message and high-score screens, which match); for the scrolling playfield they show neither the bobs nor the
scroll position its RAM holds, so the playfield is judged from its RAM instead.

Play tests (`cargo test -p g2179` with `G2179_PACK` set; Ryzen): fire passes the logo and title picture, left starts a
one-player game, get-ready takes fire, Giana walks, jumps and scrolls the whole stage, the clock runs, stage one is
completed (exit, tally, get-ready for stage 2, stage 2 playing); in a two-player game player one's lives run out, the
high-score screen takes a name, the all-time table is saved and the turn passes to player two; nothing quits.

## Known differences

- Map reads outside the stage map return 0 (the original read whatever followed the map in memory).
- The blitter's shift carries the last pixels of one bob row into the next row's first word; bobs are drawn without
  that carry (the bobs' last words are empty, so no picture changes were seen).
- The interlaced title picture shows both fields at once (each line the mix of the two field lines).
- Slot b of the hidden-brick debris animation tests and clears slot a's counter, as the original does; where the
  original then read frames past the end of its table, the last debris frame is held.
- Packed-decimal additions start with the carry clear.
