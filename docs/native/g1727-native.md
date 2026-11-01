# Game 1727 native: Volfied (Empire / Arcade Masters, 1991)

Started 2 Nov 2026 by the batch 2 agent (p1727). Stage 1 (code recovery) is in [g1727-recovery.md](g1727-recovery.md).
This page holds stage 2 (understanding) as far as it has gone and the state of stages 3 and 4. No game bytes, assets or
disassembly are in the repository; the extractor and pack live on Ryzen (`/srv/fast/native-work/1727/tools`,
`/srv/fast/native-work/1727/pack`).

## Status

| stage | state |
|---|---|
| 1 recovery | proofs 1 and 2 pass; 3 and 4 wait for the naming pass (see the recovery page) |
| 2 understanding | main loop, round set-up, playfield model, player record, file formats, font; per-round enemy logic mapped but not yet read in detail |
| 3 native rewrite | asset extractor and pack done (pictures, bob sets, font, texts); no crate yet (`rust/games/g1727` not started) |
| 4 verification | reference harness copy with state, frame and PC logs ready; nine reference recordings made |

No release yet: nothing native is playable.

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

## Assets (extractor `extract1727.py`, pack `native-asset-pack/1`, 74 assets)

`scr_at`, `scr_a`..`scr_q` with `pal_*` (pictures and their 16-colour palettes), `bobs_at`, `bobs_a`..`bobs_q`
(each round's bob frames in one atlas, colour 15 transparent) with `bobs_*_rects` (frame rectangles in table order),
`font8` (128 glyphs of 8x8 from code 0), `texts`. The decoders (`cnp.py`, `lzhuf.py`, `sprites.py`) check that every
file is consumed to its last byte.

## What remains (exact)

1. Stage 1: finish the naming pass (1,134 routines; about 400 named so far), check the main-loop, loader, decoder,
   playfield and player names by hand, re-run the report until proofs 3 and 4 pass, and install the result over
   `/srv/fast/amigo/recovery/01727` once the shared queue's own run of 1727 has ended (its lock holder then keeps the
   queue away).
2. Stage 2: read the claim fill (+68DE), the percentage and round target, the power-up boxes (+82F6, +8546, +8516),
   the bonus items (+7672), the 16 per-round enemy handlers (+999A .. +14658, about 45 KB of the 85 KB code hunk,
   built from shared helpers at +EA30 .. +F8E0), the sound (+0D7C, +0D9C, +24FC), the HUD, title, story, high-score
   table and two-player turns; take the game's random number generator from the code and log it from the harness.
3. Stage 3: `rust/games/g1727` on native_engine (typed playfield, player, cut and claim, boss and enemies per round,
   power-ups, percentage, levels, the reveal picture, HUD, title and options, story, high scores), the function map,
   cheats none (no trainer on this disk; TRAINER stays a difficulty).
4. Stage 4: scripted comparisons against the harness (state log addresses above), the RNG match, a play test through
   round 1 into round 2, death and game over; then `release.sh 1727` and the Windows and Shield checks.
