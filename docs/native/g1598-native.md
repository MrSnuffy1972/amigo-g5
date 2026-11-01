# Game 1598 (PGA Tour Golf): native rewrite notes

Native batch 2, 2 Nov 2026, p1598 agent. Status: **stage 1 (recovery) nearly complete (naming pass running), stage 2 under way,
stage 3 parts 1, 2 and 3a done (front door to pro shop, the 3D hole view, the ball in motion); not yet playable.** No game bytes here; the asset pack is built outside the repo by
`/srv/fast/native-work/1598/tools/mkpack.py` (its first version already decodes the pictures, palettes, golfer, pro
shop, tree and mountain graphics, checked by eye against the expected look).

## Program structure (from the recovered code, author names)
- `_main`: `amiga_init`, `new_init` (default player record, view window 320x200), `graphics` (title music on,
  `title_screen`, `stp` start menu; loads sound effects, fly-by music, GR3), `load_setup`, sprites for ball, big ball
  and shadow, then `pro_shop` until quit.
- The vertical-blank server `SxHan` runs every frame: the music player (`music_code`, the loaded GMUSIC's entry 0),
  then, while a shot is live, `add_event` (input queue) and `ball_interrupt` (ball flight and roll are integrated in
  the interrupt, so ball motion is tied to the 50 Hz PAL frame).
- `play`: per hole `load_hole` (the hole's record list from the CRS file), `play_hole`, `update_scores`; after 18
  holes a tournament round runs `update_tourn_stats`, `show_tourn_results`, `init_round`, `announce_tourney` for four
  rounds, then `show_tourn_stats`; practice modes leave after the hole.
- `play_hole`: fly-by on the first hole of a round when the option is on, `init_new_scene`, the play menu bar, then a
  loop of input events: the space bar or a click on the view starts the shot (`init_shot`, then `putt` on the green
  (club 16) or `shoot`, then `end_of_shot`); L/R keys (or clicks) change club or putt strength (5 to 120 in steps of
  5), aim moves the cursor along the view's heading; menu items give instant replay, mulligan, show lie, putting grid,
  overhead view, scorecard, special shots, drop/pick up ball.
- Scene: `draw_hole` fills the sky and ground (`draw_bkg`), then every polygon record (types 0 to 9; colours from
  `_poly2_colors`, water through `clip_ocean`) is transformed, near-clipped (`clip_3d`), projected (`perspect`) and
  scan-filled (`compute_edge`, `draw_poly`, `hline`); then the mountains (`draw_horizon`, three strips by
  `xy_to_angle960`), the balls and the trees (`draw_trees`, sized by distance from `_tree_dist`).
- Swing: `draw_snap_meter`/`snap_event`/`draw_snap_line` (the power snap meter), `swing_fracs` and `over_swing_fracs`,
  club table `_clubs` (16 clubs), lies and surfaces tables, backspin table, wind (`do_wind`, `draw_wind`).
- Putting: `draw_grid` (the contour grid, `perspect_grid`) and `get_contour` (slope vectors from the hole's contour
  lines; near the cup the ball is pulled in and the rim sound plays).
- Computer pros: `generate_score` with `_score_probs`; leader board and results screens.
- Arithmetic: 16-bit fixed point (`mul_frac`, `div_int2frac`, `magxy` fast hypotenuse) plus IEEE doubles through
  mathieeedoub libraries in the physics.

## What remains (in order)
1. Finish proof 3 (the model pass is running): merge the two description shards (`tools-g1598/b1-merge.py`), check
   the model's names by hand, re-run `pipeline.py 1598 --stages report`; proof 4 then follows.
2. Stage 2: decode the hole record list fully (tip text, polygon heights if any, tee and pin records), the swing and
   ball-flight maths (`shoot`, `ball_interrupt`, `end_of_shot`), the music and sound-effect player formats, and the
   menus and scorecard layout; record state names and timing.
3. Stage 3: `rust/games/g1598` on `native_engine` (pointer game bindings); course renderer, golfer animation, swing
   meter, ball physics, wind, putting grid, scorecard, tournament, the AGL text page as the intro.
4. Stage 4: reference runs on the project harness (`prekit/out/port/harness` in the archive) with scripted mouse
   input, frame and state comparison, a play test of holes to a scorecard, then `release.sh 1598`.

## Native build: parts 1 and 2 (2 Nov 2026)
`rust/games/g1598` on `native_engine` (pointer-game bindings, 50 Hz). No emulation: typed Rust with the game's own
integer arithmetic; assets from the pack (built outside the repo by `tools/mkpack.py` + `mkpack_more.py`).
- **Part 1, front door to pro shop.** The engine's pre-boot menu, then the intro: the AGL text page executed as the
  Amiga console did (CSI raster offsets `y`, left offset `x`, forward, position, clear, inverse video), so its large
  lettering, built one text line a raster line apart from the top pixel rows of a few characters, comes out; the ROM
  font is not shipped, so those top rows are this build's own (`intro::top_row`) and other glyphs are VT323. Then the
  title picture and the pro shop picture with their palettes. The logical screen is 640 x 400 (the game's 320 x 200
  doubled) so the 80-column page and text overlays are sharp. The pro shop's course choice is an interim menu.
- **Part 2, the 3D hole view**, ported routine for routine: `create_matrix` (aim), `transform` (heading and the fixed
  pitch, 1.15 fractions, the original's high-word rounding), `perspect` (480 and 545 focal lengths, centre 150, 149,
  the divide-overflow results), `clip_3d` (clip to y <= z, `mul_div` with its overflow rule; the first vertex only
  opens the first edge), `clip_ocean` / `transform_ocean` / `perspect_ocean` (32-bit path for far water),
  `compute_edge` / `draw_poly` / `reset_poly` (edge table, active list, insertion sort, removal at y max),
  `clipped_hline` / `hline` / `pattern_on` (two-colour checkerboard), `draw_bkg`, `draw_hole` (records in file order,
  colours from `_poly1_colors` / `_poly2_colors`), `draw_horizon` / `xy_to_angle960` (three mountain strips by the
  bearing to the pin), `draw_trees` (shell sort far to near, 12 sizes by `_tree_dist`, flag from GR3), `new_hole`, and
  `init_scene`'s camera (eye 88 up, 380 behind the ball, aimed at the aim point from a par 4 or 5 tee, else the pin).
  Every hole of both courses can be viewed from its tee (click left or right of the picture to step).
- Checked so far: unit tests on synthetic data (fixed-point helpers, projection, clip and fill, bearings, the console
  parser) and a scripted headless run on the real pack (intro, title, pro shop, hole 2). Frames were looked at by eye;
  they have **not yet been compared with reference frames** (stage 4).
- Function map: `g1598-function-map.md` (41 routines ported, 13 partial, 355 game routines pending, the rest platform,
  AmigaOS or C runtime).

## Part 3a: the ball in motion (2 Nov 2026)
`ball`: `ball_interrupt` ported from the assembly (the decompiler's version differs in the landing code): the step
gate (odd frames and every sixth even frame), tree deflection, flight with wind and spin after ten steps, gravity
0x275c, landing by surface (sounds 2 to 5, plugging in sand, stopping in water or hazards, the green keeping
backspin), the roll with contour vectors and friction (halved on fairway and green), the stop test on the velocity's
high words, `ball_32_to_16` rounding, `inside_poly` (even-odd with the original's vertex rule, 6 as 3, 9 as none) and
`check_cup` (within 2 units: drop below speed 0x276, else the lip hop). Unit tests on synthetic holes. Not yet wired
to a swing (part 3b), and `get_contour` (green slopes) is still to port, so rolls on greens have no slope yet.

## Next parts
3. Swing and shot: `init_shot`, the snap meter (`draw_snap_meter`, `snap_event`, `draw_snap_line`), `shoot` (club
   table, swing and over-swing fractions, hook and slice from the snap line, spin and backspin), `ball_interrupt`
   (flight with wind and spin, bounces by surface, roll with contour vectors), `end_of_shot`, `check_cup`, the golfer
   animation (GR2), the ball and shadow, the camera switch for long shots.
4. Putting (`putt`, `draw_grid`, `get_contour`), wind (`do_wind`, `draw_wind`), the menus and play menu bar, lie,
   overhead view, scorecard, players and tournaments (`generate_score`, leader boards), sound and music players.
