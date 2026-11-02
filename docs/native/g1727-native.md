# Game 1727 native: Volfied (Empire / Arcade Masters, 1991)

Started 2 Nov 2026 by the batch 2 agent (p1727). Stage 1 (code recovery) is in [g1727-recovery.md](g1727-recovery.md).
This page holds stage 2 (understanding) as far as it has gone and the state of stages 3 and 4. No game bytes, assets or
disassembly are in the repository; the extractor and pack live on Ryzen (`/srv/fast/native-work/1727/tools`,
`/srv/fast/native-work/1727/pack`).

## Status

| stage | state |
|---|---|
| 1 recovery | done: proofs 1-4 pass, installed over the shared recovery folder (see the recovery page) |
| 2 understanding | main loop, round set-up, playfield model, player record, file formats, font; per-round enemy logic mapped but not yet read in detail |
| 3 native rewrite | started: crate `rust/games/g1727` (field, player and cut, claim fill, percentage, shield, RNG, HUD, title, round card, clear, game over); enemies are a provisional stand-in; not released |
| 4 verification | reference harness copy with state, frame and PC logs ready; nine reference recordings made |

No release yet: the crate runs and plays a round, but its enemies are not yet translated, so it is not the game.

## Stage 3 so far (crate `rust/games/g1727`)

Typed Rust on native_engine; no guest memory, registers, chips or Kickstart. Coordinates are the original's map
coordinates, which are also its text coordinates (measured: frame pixel = map + (17, 8) in the harness frames; the
round picture sits at map (0, 1); the border runs along x 0 and 286, y 0 and 198, so the inside is 285 x 197 =
56,145 pixels). The logical screen is 336 x 200 because the overscanned display's HUD reaches x 335.

| module | translated from | state |
|---|---|---|
| `field` | claim map (data hunk 1 +0028), claim close +6608, fill +68DE, percentage +7D54, end test +7D90 | typed cells (free, line, claimed, cut); the fill keeps the boss's side free and claims the rest (same outcome as the boundary-walking fill; checked by tests, not yet against the harness); percentage formula exact |
| `play` | player routine +643E, direction steps +66C4..+66CA, round set-up +5BB2 | start (142, 198), shield 900 counting down one step every 5 calls (measured: every ~10 frames with two calls a frame), one pixel a move, every other call without the speed power-up, cut start needs fire over free ground, backing up erases, closing claims; shield off while cutting |
| `rng` | +5442, state +5458 | exact step (six-byte state, overlapping longs, swap, plus the frame counter) |
| `volfied` | main program +2C04, HUD +7DC8, +6484, +7D4C | title with start level and difficulty, ROUND card, play, clear with the full picture, game over; HUD positions from the code where known |
| enemies | +E8FA, +999A .. +14658 | **provisional stand-in** (a bouncing boss and three guards that kill on touching the cut, or the player when the shield is off); to be replaced by the 16 translated handlers |

Not yet translated: the 16 per-round enemy handlers and their shared helpers (+EA30 .. +F8E0), the power-up boxes
(+82F6, +8546, +8516; including the speed flag +8078), the bonus objects (+7672), the score for a claim, the
shield-out handler (+64A2), the 8-frame turn delay after backing up to a corner (+6694), sound, story, high-score
table, two-player turns, and the per-difficulty shield step table (+5746; 5 is NORMAL as measured). The function
map is therefore not complete, and stage 4 (frames, named state and RNG against the harness) has not been run.

Seven unit tests pass (`cargo test -p g1727 --no-default-features`); a scripted headless run (`tests/scripts/p1-claim.txt`)
renders the playfield, the cut and the HUD.

## The program (stage 2 so far)

- Start-up (+0000): opens dos, graphics and intuition, calls +0972, +0516 and +0334 (audio.device is opened there),
  seeds from the beam position ($DFF006 into +5458) and runs the main program +2C04.
- Main program (+2C04): title and options (+0EA6, +553C, +634C per player), story (+5850: Screen@, the scrolling text
  and the ship), then per life: round set-up (+5BB2), then the frame loop. The loop runs the logic once or twice per
  frame (+12EA counts frames the logic is behind):
  +7672 (objects moving in straight lines, and a path follower that ends the loop at its last point), +E8FA (the
  round's enemy logic: a table of 16 handlers at +E91A, one per round, +999A for round 1 to +14658 for round 16),
  +82F6 (the power-up boxes), +643E (the player: moving, cutting, closing a claim), then once a frame +13AA (wait for
  the vertical blank), +20A8, +79CC, +80BA, +747C, +E95A, +5470 (drawing, effects and HUD; not read yet), +038C (swaps
  the display buffers), +6274 (keys $45 Escape and $19 P). +5410 non-zero ends the loop: negative = a life lost,
  positive = the round cleared.
- After the loop: life lost (+5D24; a life is taken, +5E02 when they run out, else +85C8, +6370, +7C96 and back to
  the loop), or round clear (+6028, +7A14, then the next round; after round 16 +5E72; the texts include "ROUND ..
  CLEAR", "SPECIAL ROUND CLEAR", "SEPERATE ROUND CLEAR" and "CHAMPIE ROUND CLEAR").
- Round set-up (+5BB2): the player starts at (142, 198) (the middle of the bottom edge), shield 900 (BCD); the round's
  surface is Screen(A + round) in four planes; the claimed area shows Screen(B + round) planes 0-1 (four greys);
  colour 15 is set to red ($F00, the cut line); the bob set Sprite(A + round) is loaded (+7332).
- Playfield: a one-bit claim map, 40 bytes a row (320 pixels), at the pointer in data hunk 1 +0028; the player record
  (the pointer at BSS +0040, player 1 at BSS +0044) holds x, y (words), the bit mask and byte pointer of its map cell,
  the direction (+10), a turn delay (+12), round (+16), lives (+18), shield (BCD, +20), score (BCD, +22), shield on
  (+40) and the shield step counter (+41). The cut path is a list of corner points at BSS +009C (count in bytes, then
  x,y longs). A claim closes when the cut meets the claimed edge again (+6608), which calls +68DE (the fill; not read
  yet).
- Measured from the reference recordings: the player moves one pixel a frame; the shield (+20) counts down only while
  its flag (+40) is on.
- Random numbers: +5442 steps a two-long state (+5458 onwards, seeded from the beam position): the second long is
  added to the first, the sum stored, its halves swapped and a word added; callers mask the result (for example
  `andi.w #$3c` in the story). Stage 4 logs it from the harness with the PC log.
- Text: the 8x8 font at data hunk 1 +277C, drawn by +7024/+7132 in any of 16 colours into the interleaved display
  (four planes of 40 bytes a row, 160 bytes a line).

## Round 1 enemy logic (read 2 Nov 2026, not yet translated)

Read from the recovered code of +999A (1,890 bytes) for the port; the shared helpers it calls (+EA30 .. +F8E0:
collision against the claim map and the cut, 32-way direction steps, the line cursor) are next.

- Objects: the boss record at BSS +0800 and three guards of 44 bytes from BSS +06A0; x and y at +0 and +2, state or
  speed at +4 (state for a guard, step size for the boss), direction 0-31 at +6, a size or speed word at +8, timers at +10, +20 and +32, the collision flags of the last move at
  +28, the turn step at +30 (+2 or -2), the base heading at +34, the animation index and frame at +22 and +24.
- Each frame the boss runs its continuation (pointer at +44, delay in frames at +48; a step returns the next delay in
  d7 and the next step in a6) unless the time-stop counter is running or the check over the eight 24-byte objects
  at a5+5658 (a5 = BSS +3108) reports a hit this frame (what those objects are is not read yet).
- Guards: state 0 sets +8 = 7, turn step 2, turn timer 50; state 1 moves, and if the move touched the cut or the
  player the guard goes to state 2, which checks again and returns to 1 when clear. On a wall hit the direction is
  reflected from the collision flags, the base heading is kept and the turn step reversed. Turning: while the
  player's shield is on, every 7 frames the direction swings by the turn step, reversing 16 steps (half a turn) away
  from the base heading; while it is off, every 10 frames the guard turns one step towards the player. Animation
  frames 0, 1, 2, 1, one step every 5 frames. Under time stop, or when caught in a claim, a guard dies and scores from
  the table 100, 200, 400, 800, 1600, 3200, 6400 (BCD) by its frame index.
- Boss cycle: set-up gives a 300-frame phase, rotation step -1, spark speed 10 and +8 = 9. Each frame of a phase:
  move, reflect on a hit (counting hits), turn the direction by the rotation step (skipping the four axis directions,
  multiples of 8), and set the animation delays and +8 from the frames left in 32-frame bands (delays 3 and +8 = 11
  for most of the phase, rising to 4, 5 and 6 with +8 falling to 9, 7 and 5 in the last 96 frames). At the end of a
  phase: open animation (frames 0, 1, 2), flash (on 3, off 6, on 3, off 3 frames), launch three sparks from the boss
  position (30-byte records at a5+6042, delays 1, 11 and 21 frames), wait 10, close animation, then speed up by one
  (to 8) if it hit a wall 20 or more times in the phase, else slow down by one (to its minimum at +58), and start the
  next phase.

## Assets (extractor `extract1727.py`, pack `native-asset-pack/1`, 74 assets)

`scr_at`, `scr_a`..`scr_q` with `pal_*` (pictures and their 16-colour palettes), `bobs_at`, `bobs_a`..`bobs_q`
(each round's bob frames in one atlas, colour 15 transparent) with `bobs_*_rects` (frame rectangles in table order),
`font8` (128 glyphs of 8x8 from code 0), `texts`. The decoders (`cnp.py`, `lzhuf.py`, `sprites.py`) check that every
file is consumed to its last byte.

## What remains (exact)

1. Stage 1: done (2 Nov 2026).
2. Stage 2: read the claim fill (+68DE), the percentage and round target, the power-up boxes (+82F6, +8546, +8516),
   the bonus items (+7672), the 16 per-round enemy handlers (+999A .. +14658, about 45 KB of the 85 KB code hunk,
   built from shared helpers at +EA30 .. +F8E0), the sound (+0D7C, +0D9C, +24FC), the HUD, title, story, high-score
   table and two-player turns; take the game's random number generator from the code and log it from the harness.
3. Stage 3: finish `rust/games/g1727` on native_engine (typed playfield, player, cut and claim, boss and enemies per round,
   power-ups, percentage, levels, the reveal picture, HUD, title and options, story, high scores), the function map,
   cheats none (no trainer on this disk; TRAINER stays a difficulty).
4. Stage 4: scripted comparisons against the harness (state log addresses above), the RNG match, a play test through
   round 1 into round 2, death and game over; then `release.sh 1727` and the Windows and Shield checks.
