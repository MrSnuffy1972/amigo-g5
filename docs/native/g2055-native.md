# Game 2055 native: Speedball 2 - Brutal Deluxe

Started 1 Nov 2026 (batch 1). Speedball 2 (The Bitmap Brothers, 1990; this is the CSL crack, Ami-Go game 2055) is
the futuristic armoured ball game: knockout, league and cup competitions for Brutal Deluxe, a gym and transfer market
between matches, and the scrolling metal arena with its stars, bounce domes, score multipliers and electro-bounce.
The code recovery (all four proofs) is summarised in [g2055-recovery.md](g2055-recovery.md).

The native version is `rust/games/g2055` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md)). It is
written from the recovered source as ordinary typed Rust: no guest memory, no 68000 registers, no Kickstart, no chip
registers, no port reads, no copper, blitter or bitplane emulation. Pictures, the font and the intro pages come from
the asset pack at run time (built outside the repo; nothing of the game is in git).

## Status: part 1 of 4 (the front end)

The port is done in parts, each committed when it passes its checks:

1. **Front end (done)**: everything the original shows before the game loads.
2. Team management and data: the select-match menu (knockout, league, cup, practice), the fixture and result panels,
   the gym and the transfer market, the save disk.
3. The match engine: the arena, players, ball, goalkeepers, scoring, the multipliers, injuries and substitutes.
4. League, cup and knockout progression, sound, then the release builds (Windows and Android) and the Shield test.

No release has been made yet: a release waits until a full match can be played (part 3).

## What part 1 ports

The intro module, routine by routine (the recovery's automatic names are kept so the function map reads across;
several describe the routine badly, and the doc comments say what each one really does):

- The two opening pages typed on black with a click a letter: the crack's `CRYSTAL CRACKED` lines, kept as the first
  intro page (the crack has no separate cracktro program), then `A BITMAP BROTHERS GAME`. Nothing skips these.
- The title picture with its fade and whoosh, held for 255 frames; the nine story pages typed over the arena
  (`YEAR: 2095 ...` to `THE NEW TEAM IS BRUTAL DELUXE`), the title again, the eight credit pages; then the whole
  intro again unless fire is held, and `LOADING GAME` once it is.
- The typing as the original does it: each letter shows the cursor block for 5 frames over the saved background, then
  the letter; a column byte and a row in 4-line units move the typing; a line wraps at the right edge. The cursor
  blinks 13 times (opening pages, with a click), 17 (story) or 16 (credits) where the typing stopped.
- The palette fades: 15 steps of one off each colour component to black, the screen swap, 15 steps up to the
  picture's colours, 2 frames a step; only a swap when the colours already match.
- Fire skips exactly as on the original: only the waits of the intro proper (sound off, intro on), and only while it
  is held, so a short tap during the typing skips the rest of the story and the title then shows again; a tap
  during the title's wait or the credits goes to `LOADING GAME`.

Structure: the original's routines wait for the next frame deep inside nested loops, so the program runs on its own
thread in lock-step with the engine's ticks (`co.rs`, the pattern of game 336). The state it works on is typed
(`world.rs`): the two screens it draws into and shows in turn, the colour registers, the frame counter whose low byte
its waits count down, the sound and intro flags. The vertical blank shows the screen handed over during the previous
frame, as on the original.

The pictures are IFF ILBM files inside the program; `extract2055.py` decodes them as the game's own decoder does
(320 wide, always 200 rows, the left half of the 640-wide arena, CMAP to 12-bit colours as the game converts them).
The font is 43 glyphs of 16 by 16 in 32 colours, plane by plane.

Not yet in part 1: the sound effects. The front end asks for them (typing click, blink, whoosh); the native build
records each request (`sfx_last`, `sfx_count` in the named state) and plays nothing until the sound driver is ported
(part 4).

## Protection removed

- Rob Northen Copylock (entered through an F-line trap planted at run time, a longword XOR layer, then
  trace-vector decoding; it patches two game routines): not ported. The native game starts as if Copylock passed;
  the two routines it patches will be written in their patched form (return at once, no-op) when their parts are
  ported.
- The crack's own name check (the start-up compares two letters of `CRYSTAL` and hangs flashing the screen if they
  changed): not ported; the credit text is kept, so it would always pass.
- The crack's Copylock bypass in the boot block: not needed.
- The depacker's checksum that is computed and thrown away: not ported.

## Function map summary

668 recovered routines, 668 accounted for: 24 ported (part 1), 17 protection removed (all Copylock units and the
crack's bypass), 44 not ported because the engine does their job (the boot block, the crack's track loader, the
program depacker run when the pack is built, the program entry), 583 pending (parts 2 to 4). The full map (routine,
unit, address, native code, status) is kept outside the repo with the asset pack.

## Verification (part 1)

Reference: the project's 68000 reference harness copy for this game (the disk attached after the boot snapshot),
run from the G0 snapshot with no input for 2700 frames, frames every 50. Native: `examples/headless.rs` for 2700
ticks with no input. The playfield is cropped from the reference frame (it sits at x 17, y 8 of the 352 by 272 frame).

| Check | Result | Tolerance |
|---|---|---|
| Frame offset (one constant for the whole run) | reference frame = native tick - 110 (the G0 snapshot starts 110 frames into the intro) | constant |
| Playfield pixels within 24 of 765 RGB, same offset | 51 frames: 12 identical, 41 at 99% or more, 49 at 95% or more | |
| Frames under 95% | tick 1060 (80%: inside a fade, one fade step apart) and tick 2210 (93%: one typing step apart) | one fade or typing step |

The title and arena pictures match the reference exactly (100% of pixels on a held title frame), and the pages, the
typing positions, the cursor and the fade timing line up over the whole run, including the repeat of the intro.

Play tests (`cargo test -p g2055` with `G2055_PACK` set): the CRYSTAL CRACKED page is typed first with clicks asked
for, the title and the story follow; nothing skips the opening pages; fire held during the title's wait reaches
`LOADING GAME` and ends the intro.
