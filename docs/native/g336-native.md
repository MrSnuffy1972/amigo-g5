# Game 336 native: Jail Break

Added by native stage 2 (1 Nov 2026). Jail Break (Odyssey, 1990) is a bat-and-ball brick game for one or two players
taking turns: 100 built-in levels of bricks over a background map, pills that fall from bricks (cannon, slow, fast,
mine ball, next level, hold, extra life, kill, three balls, wide, giant ball, thin, double bat, reversed controls,
random), gold bricks that never break, silver bricks that take up to five hits, moving bricks, an extra life every
1000 points, credits and a countdown to carry on from the level reached, and a level editor with its own 100 custom
levels (saved). The title screen holds the switches: one or two players, each player's skill (ball speed), the
starting level (0, 25, 50), music and effects.

The native version is `rust/games/g336` on `rust/native_engine` (see [G5-NATIVE.md](../G5-NATIVE.md)). It is
written from the recovered source as ordinary typed Rust: no guest memory, no 68000 registers, no Kickstart, no chip
registers, no mouse or joystick port reads, no copper, blitter or bitplane emulation. All pictures, tiles, sounds,
tables and levels come from the asset pack at run time (extracted outside the repo; nothing of the game is in git).

## What was ported

Every screen and code path: the publisher's hold-and-modify title picture and the credits with their sounds; the
title with all its switches; one- and two-player games (the "player one / player two" screens, turns passing on a
lost life, lives, scores, extra lives); the level build-up (backgrounds, the border in one of seven styles, bricks
drawn one a frame with their sound); the ball physics pixel by pixel (eight bat segments and their angles, walls,
brick edges, moving bricks, the speed-up every 40 bat hits, the random speed nudges of hard bricks); every pill;
the cannon (and the double bat's downward cannon); the brick hit animations; the level-clear sequence; losing a life
(including the kill pill's exploding bat); pause (Space); Escape; the game-over screens with the final scores; the
editor (bricks, pills and background layers, palettes, 100 levels, clear all, reload, save); and quitting from the
title. 50 Hz PAL timing is kept: the frames the original spent decoding pictures, masking, cutting tiles and copying
large blocks are measured costs, and a frame interrupt that came while the original was busy lets its next wait
return at once, as on the original.

How the program is structured: the original is nested routines that wait for the next frame wherever they need to
(often deep inside other routines). The native main program keeps that shape: it runs on its own thread in strict
lock-step with the engine's ticks (`co.rs`), so each routine reads as its original did and the result is exactly as
deterministic as a single thread. The picture is kept as the original kept it: a shown screen and a clean copy that
moving things are drawn over and erased from (indexed 320 by 208), plus up to four sprites drawn on top.

The original's own quirks are kept: the bat-hit "pitch change" writes to a register that does nothing (so the music
never changes pitch); the double bat's second shot also moves the first shot down 12 pixels; the font comes from the
second tile sheet because the font file the program asks for is not on the disk.

Engine addition (general, additive): `Frame::direct`, an optional one-colour-a-pixel picture for screens with more
colours than a palette (the hold-and-modify title has 544).

## Protection removed

The recovery reported no protection; the code has one. After the intro the original showed a "copy check": the word
at a page and line of the manual, typed in, three tries, and the program quit after three wrong answers (a developers'
word also turned on letter keys that give pills). The native version skips the copy check as if the first answer
had been right (the letter keys stay off, as for any player who passed). The one random draw the check made is kept,
so every random number after it matches the original's. Cracktro: none.

## Function map summary

156 recovered routines, 156/156 accounted for: 141 ported (100% of the game's own logic), 7 removed with the copy
check, 8 not ported because the engine does their job. The full map (routine, unit, range, native code, reason) is
kept outside the repo with the asset pack.

Ported: initialize_game_state_and_loop wait_vblank_and_refresh wait_for_vblank_and_update_game run_game_loop_3 run_game_loop wait_vblank_and_refresh_loop initialize_game_state initialize_level_state init_jail_custom_level run_jail_over_sequence process_prisoner_cells_2 process_prisoner_cells decrement_prisoner_counters reset_player_state decrement_timer_and_trigger_event decrement_timer_and_trigger_event_2 update_player_position update_prisoner_positions check_level_two_flags_and_update reset_player_timers initialize_game_state_2 reset_and_run_game_loop decrement_counter_and_check update_prisoner_flags init_enemy_sprite_data choose_game_mode handle_input_command handle_custody_state draw_boss_health_bar decrement_animation_timer process_player_movement handle_player_escape update_prisoner_state clear_game_state update_enemy_ai update_prisoner_timer wait_for_vblank_twice write_score_digit sync_player_state set_blitter_copy_params draw_player_sprite draw_background_tile reset_level_tiles handle_player_input check_player_input wait_for_player_input wait_for_input_and_update_state update_map_tile wait_for_vblank update_map_tile_2 clear_tile_and_update clear_map_cell update_game_state trigger_level_events fill_level_tilemap draw_background_tiles initialize_player_object trigger_game_over_and_reset init_player_state play_prisoner_escape_jingle check_game_over_2 check_game_over wait_for_player_input_3 init_player_state_2 clear_pill trigger_game_over reset_player_state_2 reset_game_state reset_player_state_2_2 call_init_player_state_2 init_game_state handle_jail_customer set_interrupt_mask clear_game_buffers increment_timer_and_update decrement_timer_and_update increment_level_counter decrement_timer_and_trigger_event_3 wait_one_second clear_player_flags clear_player_flags_2 process_player_input clear_tile_buffer store_player_state process_tile_interaction reflect_player_bounce check_object_collision update_player_inventory_count set_guard_patrol_direction set_tile_collision_flags update_screen_scroll animate_prisoner_cells init_game_state_3 wait_frames wait_for_input wait_frames_2 copy_tile_row clear_tile_buffer_2 decode_level_map set_sprite_dma_and_wait_vblank wait_three_frames parse_script_command write_byte_to_buffer init_player_state_4 draw_bob_sprite process_input_string copy_tilemap_row animate_prisoner_sprite configure_and_run_bob_blitter copy_sprite_frames wait_for_game_over_flag decode_map_tile_row wait_for_input_frames init_player_state_5 accumulate_tile_checksums transpose_tilemap_rows clear_input_buffer wait_for_input_2 decrement_timer_counters decode_map_data wait_frames_3 clear_entity_slots get_digit_tile set_player_sprite clear_sprite_buffers set_sprite_dma clear_player_collision_flags update_prisoner_state_2 handle_input_state poll_cia_timer initialize_jail_state clear_enemy_targets process_sfx_jail_general wait_for_timer_tick initialize_prisoner_positions load_jail_tile_graphics copy_character_graphics show_jail_intro_sequence close_game_libraries update_score_display handle_vblank_interrupt.

(Many names come from the automatic naming pass and do not describe the routine; the native doc comments say what
each one really does, for example `wait_one_second` draws a two-digit number and `set_sprite_dma` plays a sound.
The picture decoder and tile cutting run when the pack is built or loaded; their frame costs are kept.)

Removed with the copy check: handle_jail_title_screen (kept only as the skipped check) trigger_level_event compare_strings wait_for_vblank_by_copying_tilemap draw_prisoner_sprite (its random draw kept) scramble_level_data scramble_level_data_2.

Not ported, because the engine does their job (OS start-up and shutdown, memory, the display hand-over, sprite
channel set-up, the disk): validate_disk_image (AmigaDOS's own Disk-Validator file) boot_entry run_level_script process_game_objects iterate_entity_callbacks open_dos_library close_graphics_library fill_bullet_positions.

## Verification

Reference: the 68000 reference harness running the original from power-on (the game's own reference harness build,
with mouse and button script tokens, a call log and the named state), driven by the same scripted input as the
native headless runner with `--features trace,ref-harness`. The reference passes the copy check by typing the
developers' word (no letter keys are pressed afterwards); the native build has no copy check, so the two runs are
aligned at the title screen. Two runs:

- cal: one player, start game, a scripted mouse, a pause, three lives lost, the game-over screens, the title (4721
  ticks compared).
- v2: two players with player one on SKILLED, a long game with turns passing (eleven lives lost), a pause, Escape out
  of the game, the editor (pick a brick, place two, clear one with the right button, next level, place, background
  layer, pick and place a tile, exit) and Escape on the title, which quits (10001 ticks compared).

| Check | cal | v2 | Tolerance |
|---|---|---|---|
| Call sequence (screen loads, life losses, level starts, game over) | every event within 0 to 2 frames | same | 2 frames |
| Named state (35 values: mode, pointer, bat, balls, pill, shot, scores, lives, levels, speed, serve, bricks left, credits, countdown, random seed, power-ups) exact, same tick | 802 / 4721 | 5364 / 10001 | |
| Named state exact within 2 ticks | 4657 / 4721 (98.6%) | 9618 / 10001 (96.2%) | 2 ticks |
| Playfield picture identical within 2 ticks | 4620 / 4721 (97.9%) | 9137 / 10001 (91.4%) | 2 ticks |
| Playfield picture within 64 pixels within 2 ticks | 4648 / 4721 (98.5%) | 9396 / 10001 (94.0%) | |
| Sound envelope, 250-frame windows, best lag | | median correlation 0.89 (20 of 40 windows above 0.9) | see note |

Notes: the original sometimes took two frames for one pass of its loop and caught up later (it depends on where in
the frame its work fell); the native build models the measured costs of the long work, not every routine, so it
runs up to two frames (three around the two-player screens) from the original and comes back. The remaining
picture differences are those offsets during screen wipes and screen changes. The ball trajectories, brick hits and
life losses happen in the same order on the same scripted input in both runs. The reference harness runs 313-line
frames (20.04 ms) while the native build runs exactly 50 Hz, so the sound drifts by about 3 frames a thousand; the
envelope comparison searches the lag per window.

Two differences are the reference's, not the game's, and the verification build reproduces them so like is compared
with like: the harness's stand-in OS returns from the one-second OS delay at once (the game itself waits the second,
as on an Amiga), and it never turns sprites on, so its frames have no balls, pill, shots or pointer (a real Workbench
has sprites on, and the game needs them).

Play tests: `cargo test -p g336` with `G336_PACK` set: START GAME starts a game from the title, the ball serves and
the game scores while a bot follows the ball; the editor's SAVE writes the custom levels. Desktop (Linux, the
product build, sprites on): a 4000-tick scripted session through the intro, title, two-player game, both player
screens and play; screenshots checked by eye. Start does something, nothing freezes and nothing quits after the
start-up screens (the only exits are Escape on the title and the pause menu, as designed).

Android: `tools/native_package.sh --game g336 ... --package org.amigo.g5.g00336.native`, arm64-v8a and armeabi-v7a,
g6check passes (signature, phone and TV launchers, libraries). On the Shield (installed beside the translated
build, 1 Nov 2026): pre-boot menu, title, a game started by touch, the remote's D-pad moving the bat both ways and its
centre button, then three lives lost and the game-over screens back to the title; screenshots checked by eye. Frame
rate over 140 s of play: 59.9 to 60.3 fps drawn, the game at 50 ticks a second. Fixed on the Shield: a touch far from
the pointer pressed the wrong title button (the jump went through the mouse counters, which carry at most 127 a
frame); a touch now places the pointer directly (test `a_touch_presses_the_button_under_it`).
