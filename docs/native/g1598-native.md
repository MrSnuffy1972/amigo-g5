# Game 1598 (PGA Tour Golf): native rewrite notes

Native batch 2, 2 Nov 2026, p1598 agent. Status: **stage 1 (recovery) nearly complete, stage 2 (understanding) begun,
stage 3 (rewrite) not started.** No game bytes here; the asset pack is built outside the repo by
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
