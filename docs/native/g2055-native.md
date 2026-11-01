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

668 recovered routines, 668 accounted for: 24 ported (part 1), 16 ported (part 2, first slice: the menus), 17
protection removed (all Copylock units and the crack's bypass), 44 not ported because the engine does their job (the
boot block, the crack's track loader, the program depacker run when the pack is built, the program entry), 3 started
(the first menu, the league's and the cup's starts: their menu parts are ported, the rest is pending) and 564 pending
(the rest of parts 2 to 4). The full map (routine,
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

## Part 2, first slice: the menus

`src/intro/menu.rs`, written from `initialize_post_match_screen`, `process_player_selection_menu`,
`process_entity_list`, `parse_player_name_string`, `parse_and_project_entities` and their helpers.

- The menu screen: the panel picture (disk file 1, which the game loads to `$361D6`: an IFF picture decoded by the
  same decoder as part 1's pictures), the heading and footing in two 8 x 8 fonts (one column byte a glyph, the heading
  one line up) and the items in the 16 x 16 menu font (two column bytes a glyph); the arrows (glyphs 36 and 37) at the
  item's two positions. The still parts are drawn once into a backdrop; each move copies the backdrop to the drawing
  screen, draws the arrows and shows the screen (with the panel's colours: a fade only when the colours differ).
- The menu runner: up and down move the arrows (wrapping), fire chooses (sound 22, moves sound 20), Esc leaves with
  the escape flag. Joystick events are the new pushes since the menu last cleared them, as the original's interrupt
  sets them.
- The first menu (1 PLAYER GAME, 2 PLAYER GAME, DEMO GAME, REPLAY GOALS) and its demo timeout (1500 frames without a
  joystick event; the counter is the one the vertical blank increments). 1 PLAYER GAME leads to the match menu
  (KNOCKOUT, LEAGUE, CUP, PRACTICE); LEAGUE sets mode 1 and 550 in each purse and shows NEW GAME, TEAM MANAGER, LOAD
  GAME; CUP sets mode 2 and 2000 and shows NEW GAME, LOAD GAME; PRACTICE sets 10000.
- Where a selection leads (team selection, the team manager, a match, the demo, loading a game) is recorded in the
  named state (`next`, `picks`) and the game goes back to the first menu: those screens are the rest of part 2 and
  parts 3 and 4.
- Not reproduced: the original calls its random number generator on every pass of the first menu's busy wait, so its
  generator state depends on how long the player waits; it is to be seeded where the match engine (part 3) needs it.
- Timing: the native game reaches the first menu at tick 1185 with the intro skipped; the original takes longer
  because it loads the main program from disk.

### Verification (part 2, first slice)

Reference: the harness copy with the league script (`l1`, and a new run `m55a` with RAM dumps at frames 2250, 2700,
3100 that located the panel, the fonts and the menu tables). Native: `examples/headless.rs` with scripts that make
the same choices. The playfield is cropped at x 17, y 8 as in part 1.

| Screen (same choice on both) | Differing pixels |
|---|---|
| First menu, 1 PLAYER GAME (reference frame 2150) | 0 of 64,000 (two native runs) |
| Match menu, KNOCKOUT (reference frame 2250) | 0 (two native runs) |
| League game menu, NEW GAME (reference frame 2700) | 0 |
| Back on the first menu after Esc | 0 |

Play tests: the menus lead to the league (mode 1, 550) and the cup (NEW GAME recorded), the arrows wrap, Esc goes back
to the first menu, and the demo is chosen after 1500 idle frames.

## Part 2, second slice: the knockout's NEXT FIXTURE screen

`src/intro/fixture.rs` and `src/league.rs`, written from `cycle_team_selection_screen`, `render_post_match_summary`,
`draw_two_team_names`, `render_team_stats_text`, `render_ui_component`, `update_player_flags`,
`initialize_entity_properties`, `parse_entity_message`, `wait_for_input_flag`, `clear_player_stats`,
`copy_player_struct_fields`, `average_attribute_bytes`, `average_player_attributes` and their helpers.

- The league tables as typed values (`league::League`, `league::Team`): the 17 team records of `$7F50` (names,
  played/won/drawn/lost, points for and against, points, placing, the four skill groups), the squad (`$594E`) and the
  opponent's players (`$597E`), and the 21 text rows of the screen (`$32BE`). The pack's `fixture` data and the 8 x 8
  text font (`textfont`, disk file 1 at `$44364`, 51 glyphs: letters, digits, box pieces) are added by
  `fixture2055.py` (outside the repo, beside `extract2055.py`).
- KNOCKOUT: every record cleared, mode 0, 1100 in each purse, the first opponent (REVOLVER, team 1 of the knockout
  table); the opponent's players take the team's skill groups.
- The screen: the menu panel with NEXT FIXTURE and PRESS FIRE, the middle (x 17 to 302, rows 17 to 182) filled with
  colour 6, the rows drawn 6 pixels a character and 4 lines a row (rows 4, 6, ... 44; the boxes are box-piece glyphs),
  the names and PLAY on row 6. Each team's numbers are written into the shared rows first (left team at character
  19, right at 49): points as four digits, played/won/drawn/lost as two, NA for placing and points for/against outside
  the league (their values in the league), and the four groups' ratings (the eight skill bytes over eight; the
  player's team takes its groups from its squad first, three players a group).
- Fire (or a key) leaves the screen; Esc sets the escape flag.
- Not yet: the gym and team screen (`handle_team_selection_logic`) and the match (`initialize_match_state`) that the
  knockout runs next, the knockout's progression through the 15 opponents and its win sequence; the knockout
  records `next` = 1 and goes back to the first menu.

### Verification (part 2, second slice)

Reference: the harness copy's knockout run (`k1`: fire at 2200, 2350, then every 120 to 150 frames). Native:
`examples/headless.rs` with the same choices (fire at 1000, 1300, 1450, 1700).

| Screen | Differing pixels |
|---|---|
| NEXT FIXTURE, BRUTAL DELUXE v REVOLVER (reference frame 2450; native ticks 1500 to 1700, five frames) | 0 of 64,000 each |

Play test: KNOCKOUT shows NEXT FIXTURE (mode 0, 1100, opponent 1), waits for fire, then goes back to the menus.

## Part 3 groundwork: the match (measured, nothing ported yet)

Started 1 Nov 2026 (second run). No match code is ported yet, so there is still no release: the match engine is
roughly 250 recovered routines (from `initialize_match_state` to the sprite update before `initialize_game_state`),
about a quarter of the program, and it did not fit in this run. What was measured, for the port to build on:

- **Reference recording**: the DEMO GAME (computer against computer, so no input decides it and the random numbers
  alone drive the match) is a whole short match in the harness: the darkened arena at frame 2600, the line-up with
  the team box (FATAL JUSTICE v TURBO HAMMERS, shortened as the game prints them) at 2700, play from 2750, MATCH OVER
  at 3800, the result screen at 3900 and the manager and gym screens after it. A new run `dmr` (the demo script, 3300
  frames, frames every 50) keeps RAM snapshots at frames 2560, 2600, 2650, 2700, 2750, 2800 and 3000 (outside the
  repo, with the other runs). This is the recording the match engine is to be verified against: frames and named
  state (each object's position, speed and flags), with tolerances written down when the comparison is made.
- **The screen**: three screen buffers (triple buffering), each five planes of 42 bytes by 209 rows (336 pixels wide,
  16 more than shown, for the sideways fine scroll); the display list restarts the plane pointers part-way down, so
  each buffer is a circular strip that scrolls up and down by moving the start row. A separate 16-row, five-plane
  strip is the status bar under the arena.
- **The arena graphics**: the background under the centre launcher is drawn from blocks of 96 by 64 pixels, five
  planes of 12 bytes a row, 3,840 bytes a block, kept in chip memory; the plain floor repeats one block. How the rest
  (the star, the walls, the multipliers) is laid out is still to be read from the drawing routines.
- **The objects**: the match works on 40-byte object records: the ball first, then the two squads (nine on the
  pitch each, reached through two tables of pointers) and further records not yet identified. Words 20
  and 22 of a record are its arena position, word 32 its speed, byte 34 its flags. The arena is 640 by about 1,150
  units with the centre spot at (320, 576), where the ball sits before the launch. The squads walk on in a tight
  block either side of the centre (frame 2560) and line up at thirds and sixths of the width (213, 426, 106, 533) by
  frame 2700; the two words the projection reads as the view position are (160, 484) at the line-up.
- **The frame**: each frame the match runs, in order, the goal and state checks, the timers, the goal collisions and
  the goal animation, the camera clamp, the players' input and decisions, the team bookkeeping, the ball, the
  distances between players, the event timers and the position checks, then waits for the frame count, lays out the
  sprites, projects every object to the screen (in a fixed order, the ball before or after the players depending on
  who holds it) and swaps the buffers.

Next for part 3, in order: the arena (the block map and the scroll, checked against frames 2600 and 2700 with the
objects masked out), the objects drawn at their recorded positions (checked against the same frames), then the
match logic routine by routine with the random numbers, checked as named state against `dmr` frame by frame.
