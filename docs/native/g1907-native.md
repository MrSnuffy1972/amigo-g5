# Game 1907 native: International Karate +

Added by native batch 1 (1 Nov 2026). International Karate + (System 3, 1988; Ami-Go game 1907, the Oracle crack
with the Invisible Crime cracktro) is a three-fighter karate game: the white fighter (player 1), red (player 2 or the
computer) and blue (always the computer) fight bouts of 30 seconds judged by a referee who speaks in a bubble; six
half points win a bout; a fighter who comes last is out; every third bout is a bonus round; the belt colour follows
the score; a 50-place high-score table with name entry.

The native version is `rust/games/g1907` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md)). It is
written from the recovered source (`docs/native/g1907-recovery.md`) as typed Rust: no guest memory, no 68000
registers, no Kickstart, no custom-chip registers, no port reads, no copper, blitter or bitplane emulation. All
pictures, frames, fonts, colours, sounds, the music and the game's tables come from the asset pack at run time
(extracted outside the repository; nothing of the game is in git).

## How it is built

- `ik.rs`: the state (per-fighter arrays of three, named after what they hold, each with the original's address in
  its comment), the frame interrupt's work (`vblank`: clocks, hit timers, delayed sounds, the effects channel, the
  music), the option keys and code words (`handle_input_byte`), drawing (the two screens and the background put back
  under what moved, as the original kept them, indexed 320 by 200), and the random numbers (the original's mixer of
  three running sums with bytes of its own image, a table in the pack).
- `flow.rs`: the phases of the main loop: 0 set-up, 1 a bout, 3 between bouts (ranking, the verdict, the time
  bonus, who is out), 6 the pause (the fighters show their moves), 4 the match over, 5 the bonus round; the panel
  (points markers, scores, clock, round, speed meter) and the belt colours.
- `fight.rs`: moves are table driven as in the original (each move a run of animation cursors; each cursor a frame,
  a step and flags: hold, turn, interruptible, knocked down, attacking); the joystick tables (with the special table
  while an opponent attacks from behind); the computer fighters' choices (target, counters, attacks by distance and
  sides, skill by level and bout); hit detection (attack frames against body zones, reach, half and full points, the
  score pop-up, the reaction move); the stars over a fallen fighter.
- `speech.rs`: the referee: the bubble placed in the widest gap between the fighters, its frame and tail, the text
  (fighters' names by rank, big letters, the seconds left), the referee talking, the sparkles.
- `hiscore.rs`: the high-score table, sorting, name entry with the joystick, the "top 50 scores" show and its lights.
- `bonus.rs`: the ball round: the shield's seven places from the joystick, the balls' physics (in the frame
  interrupt), pixel collisions against the fighter's and the shield's outlines, the bonus for all balls turned away.
- `music.rs`: the title music player rewritten on typed channel records (note streams, instruments with attack and
  loop, volume envelopes, slides, vibrato), feeding the engine mixer as Paula channels; channel 3 is shared with the
  effects as on the original.
- `co.rs`: the main program on its own thread in lock-step with the engine's ticks (the g336 pattern), the per-row
  colours of the original's display list given as one colour a pixel (`Frame::direct`: the sky and the sea are
  gradients down the screen).
- `skew.rs`: the shadows' lean, generated from the recovered routine as a list of word operations.

Controls: the joystick's eight directions with and without fire give the sixteen moves (pad D-pad or stick and A,
arrows and Right Ctrl or Space, the touch stick and Fire button); Start / Return = F1 (one player), B / F2 = two
players, Y / F3 = music; the pre-boot menu is the front door.

## Verification (1 Nov 2026)

Reference: the b1 copy of the 68000 reference harness running the original from power-on with the recovery's
one-player recording (`s_p1a`), state logged every tick (phase, clock, positions, frames, facing, moves, points,
scores). Native: the same joystick script through `examples/headless.rs`, aligned at the game start.

| Check | Result | Tolerance |
|---|---|---|
| Phase sequence, first bout and the break after it | bout 1580 vs 1585 ticks, break 350 vs 350 | 2 frames a step |
| First picture of the bout (fighters bowing), whole screen | 98.5% of pixels identical | sea shimmer, 1 random step |
| Named state over bout 1 (1580 ticks) | phase 100%, clock 94% within 2 ticks, positions and frames identical until the first computer decision (+70 ticks) | see note |

Note: the computer fighters' choices use the original's random numbers, which mix a count of calls since power-on
with bytes of the program's own memory; the native run starts from the title (no cracktro, a shorter title) and the
window it reads holds a few run-time variables as stored in the image, so the numbers part after the first random
choice and the bouts then differ in detail (same rules, different fights). Scoring, verdicts and phases follow the
same code.

Random numbers (revision 2, 1 Nov 2026). `Ik::random16` is now compute_obfuscation_checksum ($726A) exactly: the
call count is loaded into d0 as a word, so the first byte read replaces only its low byte; that word (sign extended,
as a 68000 index) indexes the second read and is the multiplicand of `mulu.w`; `sub.b ($92E)` subtracts the count's
high byte (the word is big-endian). The first revision used the byte alone and subtracted the low byte, which is the same only while the count is below 256 (the first 100 or so
ticks of a game). `rngwin` now holds the image from $600, so the reads reach from $600 up to $785C+$80FF as the
original's do. `--seed N` now means what the original carries into a game: N draws made before it (the three running
sums at $7856; the count at $92E restarts with every game), where it used to set the count, which every game clears.
Measured against the reference (`STATELOG` with $92E, $7856-$785B and $608 every tick, recording `s_p1a`):

| Check | Result | Tolerance |
|---|---|---|
| Running sums at the first draw of the game, `--seed 1495` | equal to the reference (1495 draws made by the cracktro and title) | exact |
| Generator output, every draw of bout 1 up to tick 3120 (`RNGLOG`, `RNG_PC=786A`: the routine's absolute address) | 197 of 197 equal | exact |
| Draws per tick in bout 1 | reference 1 to 3 a tick from the bow onwards (1645 by the end of the bout); native one every 5 ticks (330) | not met |
| First difference in named state, aligned at the bow | player position +76 ticks, computer fighter 2 +81, fighter 1 +171: the same with and without the seed | see note |

Note: the sequences part at the second tick of the bout because the original draws far more often, from a caller
the native does not yet make: about one a tick enters with D0 = $C00F0014 or $C00F0018 and D1 pointing into chip
RAM at $71xxx, which no ported routine does (the call sites in the recovered source are unit offsets; add $600 for
the harness). Until that caller is found and ported the computer fighters' choices differ in detail;
the first state difference (+76) is the player's own position, so it is not the random numbers that end the match.
Run-time variables in the window ($96CD-$96DB, $D5D7-$D6B9) are read from the image as stored: only a game past about
7700 draws (some 100 s) reaches them. Draws past 32768 in one game would read the Kickstart ROM and read 0 here.

Play test: `cargo test -p g1907` with `G1907_PACK` set: fire starts a one-player game from the title, the clock runs,
the player's fighter moves, fighters score, the bout ends. Headless 6000-tick sessions show the title cards, bouts
with the referee's verdicts, the ball round and the match over; screenshots checked by eye.

Release tests (1 Nov 2026, release r2 = fc5c3e2). Windows (WinPC, keys posted to the window): pre-boot, title card,
F1 starts a one-player game, the bout runs (scores and clock move), closed cleanly. Shield (only from an idle,
asleep Shield, put back to sleep after): pre-boot, the title's options card, then the computer's demo bouts and the
high-score table drawn correctly, 58.0 to 62.5 fps (SurfaceFlinger timestats, the game layer); the injected remote
centre presses (a press of a few milliseconds) did not start a player game, so play under control on the Shield is
still to be checked with a held press or a pad.

## Not in this revision (remaining)

- The cracktro (Invisible Crime intro) as the native intro piece.
- The title's 33-step script (fades, starfield, logos, scrolling credits, fighters dancing behind the title): shown
  as three cards from the title sheet.
- The background's extras (sea shimmer and sun reflection, the creatures and code-word extras).
- The bomb round (bonus phase 7): played as the ball round.
- The random numbers' per-tick caller (about one draw a tick in a bout, see Verification), so the computer fighters'
  choices match the original only while no draw is missed.
- A second pad and touch for player 2: the engine's input has no pad index yet (a change to the SDL and Android
  layers for every game), so player 2 stays on the keyboard (W A S D and Left Shift).
- A second player's controls: the engine has one set of controls; two-player games start but player 2 has no input.

The function map (routine, unit, range, status, where or why) is kept outside the repo with the asset pack:
166 routines ported, 41 done by the engine or the extractor, 143 not yet (the items above).
