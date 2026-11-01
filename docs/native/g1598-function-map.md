# Game 1598 (PGA Tour Golf): function map

Every routine of the game program (unit 1, the decrunched Golf, author names from the 1593 symbols), with where it
lives in the native build. Status: **ported** (typed Rust, same arithmetic), **partial** (part of its behaviour so far),
**pending** (game logic still to port), **platform** (Amiga hardware, memory and file plumbing that native_engine and
the asset pack replace), **os** (AmigaOS calls), **runtime** (the Aztec C library). Generated from the recovered names;
counts at the end. No code or bytes here.

| routine | status | native |
|---|---|---|
| `random_init` | pending | game logic, to port |
| `stp` | pending | game logic, to port |
| `check_on` | pending | game logic, to port |
| `check_off` | pending | game logic, to port |
| `set_proshop` | pending | game logic, to port |
| `auto_center_string` | pending | game logic, to port |
| `set_player_name` | pending | game logic, to port |
| `set_pro_menu_items` | pending | game logic, to port |
| `set_game_menu_items` | pending | game logic, to port |
| `pro_disable_choice` | pending | game logic, to port |
| `game_disable_choice` | pending | game logic, to port |
| `disable_choice` | pending | game logic, to port |
| `option_state` | pending | game logic, to port |
| `put_check` | pending | game logic, to port |
| `remove_check` | pending | game logic, to port |
| `option_toggle` | pending | game logic, to port |
| `load_setup` | pending | game logic, to port |
| `save_setup` | pending | game logic, to port |
| `toggle_overhead` | pending | game logic, to port |
| `toggle_ball_lie` | pending | game logic, to port |
| `toggle_greens` | pending | game logic, to port |
| `toggle_fly_by` | pending | game logic, to port |
| `toggle_music` | pending | game logic, to port |
| `toggle_sound` | pending | game logic, to port |
| `get_offset` | pending | game logic, to port |
| `set_planes` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_new_object` | pending | game logic, to port |
| `free_card` | pending | game logic, to port |
| `load_card` | pending | game logic, to port |
| `free_board` | pending | game logic, to port |
| `load_board` | pending | game logic, to port |
| `free_pro_golfer` | pending | game logic, to port |
| `load_pro_golfer` | pending | game logic, to port |
| `create_offsets` | pending | game logic, to port |
| `set_player_slot` | pending | game logic, to port |
| `player_active` | pending | game logic, to port |
| `remove_active_player` | pending | game logic, to port |
| `total_players` | pending | game logic, to port |
| `get_next_player_slot` | pending | game logic, to port |
| `save_players_status` | pending | game logic, to port |
| `load_players_status` | pending | game logic, to port |
| `get_open_slot` | pending | game logic, to port |
| `get_player_file` | pending | game logic, to port |
| `get_player_names` | pending | game logic, to port |
| `key_convert` | pending | game logic, to port |
| `get_string` | pending | game logic, to port |
| `get_object` | pending | game logic, to port |
| `free_mountains` | pending | game logic, to port |
| `load_mountains` | ported | assets (mountain strips) |
| `write_pixel` | pending | game logic, to port |
| `moreinit` | pending | game logic, to port |
| `blit_fig` | pending | game logic, to port |
| `new_blit` | pending | game logic, to port |
| `draw_box` | pending | game logic, to port |
| `draw_frame` | pending | game logic, to port |
| `draw_frame2` | pending | game logic, to port |
| `path_attach` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `graf_read` | ported | pack builder (object banks) |
| `return_from_error` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `error_exit` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `file_read` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `file_write` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `file_search` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `delete_file` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `alloc_file_info` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `free_file_info` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `get_files` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `get_file_size` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `byte_transfer` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `set_buffer_address` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `get_buffer` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_obj` | partial | scene::blit_bottom_centre (placement only) |
| `_clip_blit` | pending | game logic, to port |
| `draw_pobj` | pending | game logic, to port |
| `init_anims` | pending | game logic, to port |
| `animate` | pending | game logic, to port |
| `mouse_init` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `mouse_end` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `move_words` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `clip_blit` | partial | scene::blit_bottom_centre |
| `generate_sound` | pending | game logic, to port |
| `hide_mouse` | partial | golf (pointer drawn) |
| `log_scr` | partial | golf::Golf::frame |
| `phys_scr` | partial | golf::Golf::frame |
| `random` | pending | game logic, to port |
| `rnd` | pending | game logic, to port |
| `curr_font` | pending | game logic, to port |
| `show_mouse` | partial | golf (pointer drawn) |
| `swap_bytes` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `swap_longs` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `page_flip` | partial | golf::Golf::frame |
| `key_wait` | pending | game logic, to port |
| `cleanup_exit` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `memory_error_exit` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `read_error_exit` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `abs` | ported | Rust i16::wrapping_abs |
| `max` | ported | Rust max |
| `min` | ported | Rust min |
| `mul_div` | ported | fixed::mul_div |
| `mul_ints` | pending | game logic, to port |
| `mul_frac` | ported | fixed::mul_frac |
| `div_int2frac` | ported | fixed::div_int2frac |
| `magxy` | ported | fixed::magxy |
| `percent` | pending | game logic, to port |
| `mul32_frac` | pending | game logic, to port |
| `proshop_pallette` | ported | golf (pal.proshop) |
| `title_pallette` | ported | golf (pal.title) |
| `new_pallette` | ported | golf (pal.new) |
| `old_pallette` | pending | game logic, to port |
| `credits_pallette` | pending | game logic, to port |
| `clr_screen` | pending | game logic, to port |
| `set_color` | ported | scene::hline (c, c) |
| `clipped_xy_hline` | pending | game logic, to port |
| `line` | pending | game logic, to port |
| `vline` | pending | game logic, to port |
| `set_hilite` | pending | game logic, to port |
| `set_string_hilite` | pending | game logic, to port |
| `print_character` | pending | game logic, to port |
| `print_string` | pending | game logic, to port |
| `font_color` | pending | game logic, to port |
| `b_print_font_at` | pending | game logic, to port |
| `b_print_font_centered` | pending | game logic, to port |
| `s_print_font_at` | pending | game logic, to port |
| `s_print_font_centered` | pending | game logic, to port |
| `print_font_at` | pending | game logic, to port |
| `print_font_centered` | pending | game logic, to port |
| `_draw_object` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `blitter_object` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `page_copy` | pending | game logic, to port |
| `Save_Object_Init` | pending | game code (display and object management): native_engine frame and pack |
| `Free_Object_Planes` | pending | game code (display and object management): native_engine frame and pack |
| `Free_Background` | pending | game code (display and object management): native_engine frame and pack |
| `save_bkg` | pending | game logic, to port |
| `rest_bkg` | pending | game logic, to port |
| `free_bkg` | pending | game logic, to port |
| `menu_init` | pending | game logic, to port |
| `save_menu_bkg` | pending | game logic, to port |
| `free_menu_bkg` | pending | game logic, to port |
| `rest_menu_bkg` | pending | game logic, to port |
| `amiga_init` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `sprite_off` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `sprite_ball` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `end_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_shadow_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `shadow_sprite_off` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `sprite_shadow` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `end_shadow_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_ball_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `bigball_sprite_off` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `sprite_bigball` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `end_ball_sprite` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `window_front` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `SxHan` | pending | game code (display and object management): native_engine frame and pack |
| `swcolor` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `mouse_position` | partial | golf::Golf::frame (pointer) |
| `mouse_and_keyboard` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_hw` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `wait_bovp` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `DisplayOpen` | pending | game code (display and object management): native_engine frame and pack |
| `hdswap` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `hidden_scr` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `hidden_scr_off` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `term` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `DisplayClose` | pending | game code (display and object management): native_engine frame and pack |
| `SIntOff` | pending | game code (display and object management): native_engine frame and pack |
| `decode` | ported | pack builder (pictures) |
| `lzw_picture` | ported | pack builder (pictures) |
| `box_colors` | pending | game logic, to port |
| `set_title_box_colors` | pending | game logic, to port |
| `set_box_colors` | pending | game logic, to port |
| `please_wait1` | pending | game logic, to port |
| `please_wait2` | pending | game logic, to port |
| `hot_key_colors` | pending | game logic, to port |
| `title_box_colors` | pending | game logic, to port |
| `default_box_colors` | pending | game logic, to port |
| `memory_detect` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `memory_size` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `find_buffer` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `load_sound_effects` | pending | game logic, to port |
| `sound_size` | pending | game logic, to port |
| `sound_init` | pending | game logic, to port |
| `sound_end` | pending | game logic, to port |
| `get_memory` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_fly_by_music` | pending | game logic, to port |
| `fly_by_music_on` | pending | game logic, to port |
| `fly_by_music_off` | pending | game logic, to port |
| `fly_by_end` | pending | game logic, to port |
| `music_init` | pending | game logic, to port |
| `music_load` | pending | game logic, to port |
| `music_end` | pending | game logic, to port |
| `sound` | pending | game logic, to port |
| `fly_by_song` | pending | game logic, to port |
| `_sound` | pending | game logic, to port |
| `music_on` | pending | game logic, to port |
| `music_off` | pending | game logic, to port |
| `music_code` | pending | game logic, to port |
| `load_pro_shop` | pending | game logic, to port |
| `load_tent_shop` | pending | game logic, to port |
| `alloc_mask` | pending | game logic, to port |
| `mask_object` | pending | game logic, to port |
| `draw_object_one` | pending | game logic, to port |
| `draw_object_two` | pending | game logic, to port |
| `draw_object` | pending | game logic, to port |
| `hline` | ported | scene::hline |
| `set_blitter_nasty` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `transform` | ported | scene::View::transform |
| `perspect` | ported | scene::perspect |
| `new_init` | pending | game logic, to port |
| `player_init` | pending | game logic, to port |
| `pattern_on` | ported | scene::hline (c1, c2 checkerboard) |
| `draw_bkg` | ported | scene::draw_bkg |
| `draw_hole` | ported | hole::draw |
| `reload_trees` | pending | game logic, to port |
| `load_course` | ported | assets::Assets::load |
| `load_hole` | ported | assets (courses in the pack) + hole::load |
| `new_hole` | ported | hole::load |
| `create_matrix` | ported | scene::View::aim |
| `clip_3d` | ported | scene::clip_3d |
| `clipped_hline` | ported | scene::Fill::clipped_hline |
| `draw_poly` | ported | scene::Fill::draw |
| `compute_edge` | ported | scene::Fill::edge |
| `reset_poly` | ported | scene::Fill::reset |
| `alloc_poly_arrays` | ported | scene::Fill (growable buffers) |
| `inside_poly` | ported | ball::inside_poly |
| `clip_ocean` | ported | scene::clip_ocean |
| `perspect_ocean` | ported | scene (perspect_ocean) |
| `transform_ocean` | ported | scene::View::transform_ocean |
| `clipped_line` | pending | game logic, to port |
| `cliptest` | pending | game logic, to port |
| `delay` | pending | game logic, to port |
| `main` | pending | game logic, to port |
| `special_effects` | pending | game logic, to port |
| `_copy_hline` | pending | game logic, to port |
| `_move_line_down` | pending | game logic, to port |
| `_copy_vline` | pending | game logic, to port |
| `_transform` | pending | game logic, to port |
| `_transform_hair` | pending | game logic, to port |
| `transform_shoes` | pending | game logic, to port |
| `transform_golfer` | pending | game logic, to port |
| `transform_putter` | pending | game logic, to port |
| `XlateColor` | pending | game code (display and object management): native_engine frame and pack |
| `new_init_obj` | pending | game logic, to port |
| `get_byte_width` | pending | game logic, to port |
| `transform_object` | pending | game logic, to port |
| `object_free` | pending | game logic, to port |
| `Free_All_Planes` | pending | game code (display and object management): native_engine frame and pack |
| `Free_Planes` | pending | game code (display and object management): native_engine frame and pack |
| `Free_Object` | pending | game code (display and object management): native_engine frame and pack |
| `alloc_plane` | platform | Amiga hardware, memory or file plumbing: native_engine and the asset pack |
| `init_mask_object` | pending | game logic, to port |
| `init_draw_object` | pending | game logic, to port |
| `alert_disk_full` | pending | game logic, to port |
| `choose_filename` | pending | game logic, to port |
| `new_player` | pending | game logic, to port |
| `load_player` | pending | game logic, to port |
| `delete_player` | pending | game logic, to port |
| `save_player` | pending | game logic, to port |
| `save_all_players` | pending | game logic, to port |
| `get_program_disk` | pending | game logic, to port |
| `show_plyr_stats` | pending | game logic, to port |
| `update_tourn_stats` | pending | game logic, to port |
| `show_tourn_stats` | pending | game logic, to port |
| `new_stats` | pending | game logic, to port |
| `reset_pro_stats` | pending | game logic, to port |
| `get_avg_score` | pending | game logic, to port |
| `saved_plyr_stats` | pending | game logic, to port |
| `draw_trees` | ported | scene::draw_trees |
| `check_trees` | pending | game logic, to port |
| `draw_horizon` | ported | hole::draw_horizon |
| `xy_to_angle960` | ported | scene::xy_to_angle960 |
| `get_dist` | pending | game logic, to port |
| `draw_grid` | pending | game logic, to port |
| `get_contour` | pending | game logic, to port |
| `get_contour_vector` | pending | game logic, to port |
| `perspect_grid` | pending | game logic, to port |
| `Free_Graphics` | pending | game code (display and object management): native_engine frame and pack |
| `blit_obj` | partial | scene::blit_bottom_centre |
| `free_gr1` | pending | game logic, to port |
| `load_gr1` | pending | game logic, to port |
| `free_gr2` | pending | game logic, to port |
| `load_gr2` | pending | game logic, to port |
| `load_gr3` | pending | game logic, to port |
| `free_gr3` | pending | game logic, to port |
| `title_screen_start` | pending | game logic, to port |
| `gr3_mode_switch` | pending | game logic, to port |
| `gr4_mode_switch` | pending | game logic, to port |
| `please_wait` | pending | game logic, to port |
| `graphics` | pending | game logic, to port |
| `get_pro_name` | pending | game logic, to port |
| `fly_by` | pending | game logic, to port |
| `print_hole_title` | pending | game logic, to port |
| `init_round` | pending | game logic, to port |
| `reset_players` | pending | game logic, to port |
| `who_is_up` | pending | game logic, to port |
| `player_up` | pending | game logic, to port |
| `player_down` | pending | game logic, to port |
| `set_default_players` | pending | game logic, to port |
| `init_new_scene` | pending | game logic, to port |
| `draw_name` | pending | game logic, to port |
| `init_scene` | partial | hole::camera (the view only) |
| `draw_snap_meter` | pending | game logic, to port |
| `snap_event` | pending | game logic, to port |
| `draw_snap_line` | pending | game logic, to port |
| `draw_club` | pending | game logic, to port |
| `display_club` | pending | game logic, to port |
| `do_splash` | pending | game logic, to port |
| `auto_club_select` | pending | game logic, to port |
| `ball_on_tee` | pending | game logic, to port |
| `all_sprite_off` | pending | game logic, to port |
| `end_of_shot` | pending | game logic, to port |
| `do_instant_replay` | pending | game logic, to port |
| `special_shots` | pending | game logic, to port |
| `set_clip` | pending | game logic, to port |
| `reset_clip` | pending | game logic, to port |
| `save_ball_pos` | pending | game logic, to port |
| `overhead` | pending | game logic, to port |
| `convert_pt` | pending | game logic, to port |
| `draw_course_overhead` | pending | game logic, to port |
| `sync_to_int` | pending | game logic, to port |
| `ball_interrupt` | ported | ball::Ball::interrupt (fly, land, roll) |
| `ball_32_to_16` | ported | ball::round16 |
| `get_ball_pos` | pending | game logic, to port |
| `new_draw_ball` | pending | game logic, to port |
| `draw_ball` | pending | game logic, to port |
| `erase_ball` | pending | game logic, to port |
| `draw_cursor` | pending | game logic, to port |
| `erase_cursor` | pending | game logic, to port |
| `check_cup` | ported | ball::Ball::cup |
| `check_ball_in_cup` | pending | game logic, to port |
| `check_priority` | pending | game logic, to port |
| `draw_inset_objs` | pending | game logic, to port |
| `set_inset_clip` | pending | game logic, to port |
| `draw_inset` | pending | game logic, to port |
| `clipped_xy_pixel` | pending | game logic, to port |
| `draw_inset_box` | pending | game logic, to port |
| `draw_all_balls` | pending | game logic, to port |
| `obj_overlap` | pending | game logic, to port |
| `events_off` | pending | game logic, to port |
| `events_on` | pending | game logic, to port |
| `get_event` | pending | game logic, to port |
| `timer_set` | pending | game logic, to port |
| `timer_off` | pending | game logic, to port |
| `timer_on` | pending | game logic, to port |
| `add_event` | pending | game logic, to port |
| `any_input` | pending | game logic, to port |
| `clear_inputs` | pending | game logic, to port |
| `wait_for_input` | pending | game logic, to port |
| `delay_until_input` | pending | game logic, to port |
| `flush_event_q` | pending | game logic, to port |
| `Queue_Add_Update` | pending | game code (display and object management): native_engine frame and pack |
| `Queue_Remove_Update` | pending | game code (display and object management): native_engine frame and pack |
| `Queue_Element` | pending | game code (display and object management): native_engine frame and pack |
| `update_scores` | pending | game logic, to port |
| `draw_scorecard` | pending | game logic, to port |
| `game_scorecard` | pending | game logic, to port |
| `pro_scorecard` | pending | game logic, to port |
| `do_birds` | pending | game logic, to port |
| `mulligan` | pending | game logic, to port |
| `show_lie` | pending | game logic, to port |
| `leave_game` | pending | game logic, to port |
| `start_game` | pending | game logic, to port |
| `play` | pending | game logic, to port |
| `play_hole` | pending | game logic, to port |
| `free_ball_and_shadow` | pending | game logic, to port |
| `shoot` | pending | game logic, to port |
| `putt` | pending | game logic, to port |
| `check_lip` | pending | game logic, to port |
| `check_splash` | pending | game logic, to port |
| `init_shot` | pending | game logic, to port |
| `pro_shop` | partial | golf::Golf::pro_shop (interim course menu) |
| `draw_pro_shop` | pending | game logic, to port |
| `get_courses` | pending | game logic, to port |
| `pick_course` | pending | game logic, to port |
| `driving_range` | pending | game logic, to port |
| `putting_green` | pending | game logic, to port |
| `practice` | pending | game logic, to port |
| `tournament` | pending | game logic, to port |
| `prepare_game` | pending | game logic, to port |
| `_prepare_game` | pending | game logic, to port |
| `announce_tourney` | pending | game logic, to port |
| `print_font_drop_shadow` | pending | game logic, to port |
| `add_commas` | pending | game logic, to port |
| `load_pros` | pending | game logic, to port |
| `do_wind` | pending | game logic, to port |
| `force_draw_wind` | pending | game logic, to port |
| `draw_wind` | pending | game logic, to port |
| `xy_to_angle` | pending | game logic, to port |
| `generate_score` | pending | game logic, to port |
| `sort_t_players` | pending | game logic, to port |
| `show_leader_board` | pending | game logic, to port |
| `show_tourn_results` | pending | game logic, to port |
| `game_t_results` | pending | game logic, to port |
| `pro_t_results` | pending | game logic, to port |
| `game_show_t_stats` | pending | game logic, to port |
| `pro_show_t_stats` | pending | game logic, to port |
| `game_lboard` | pending | game logic, to port |
| `pro_lboard` | pending | game logic, to port |
| `wait_for_next_page` | pending | game logic, to port |
| `new_title` | pending | game logic, to port |
| `title_screen` | partial | golf::Golf::title_screen (picture; credits, music to come) |
| `credits_screen` | pending | game logic, to port |
| `sparkle_sequence` | pending | game logic, to port |
| `showmouse` | pending | game logic, to port |
| `hidemouse` | pending | game logic, to port |
| `text_box` | pending | game logic, to port |
| `_text_box` | pending | game logic, to port |
| `pro_menu` | pending | game logic, to port |
| `play_menu` | pending | game logic, to port |
| `_restore_video` | pending | game logic, to port |
| `restore_video2` | pending | game logic, to port |
| `_save_video` | pending | game logic, to port |
| `save_video2` | pending | game logic, to port |
| `hilite_bar_letter` | pending | game logic, to port |
| `hilite_menu_letter` | pending | game logic, to port |
| `menu_word` | pending | game logic, to port |
| `event` | pending | game logic, to port |
| `get_option_number` | pending | game logic, to port |
| `draw_down_menu` | pending | game logic, to port |
| `erase_down_menu` | pending | game logic, to port |
| `check_xlimits` | pending | game logic, to port |
| `invert_option_text` | pending | game logic, to port |
| `restore_option_text` | pending | game logic, to port |
| `get_menu_option` | pending | game logic, to port |
| `draw_menu` | pending | game logic, to port |
| `erase_menu` | pending | game logic, to port |
| `newmenu` | pending | game logic, to port |
| `get_menu_word` | pending | game logic, to port |
| `get_keyborad_menu_option` | pending | game logic, to port |
| `get_adjacent_boxes` | pending | game logic, to port |
| `adjacent_boxes` | pending | game logic, to port |
| `center_2d` | pending | game logic, to port |
| `message_window` | pending | game logic, to port |
| `box_initialize` | pending | game logic, to port |
| `_box` | pending | game logic, to port |
| `in_box` | pending | game logic, to port |
| `which_box` | pending | game logic, to port |
| `box_exit` | pending | game logic, to port |
| `mouse_press` | pending | game logic, to port |
| `hilite_box` | pending | game logic, to port |
| `restore_box` | pending | game logic, to port |
| `get_box_states` | pending | game logic, to port |
| `update_box` | pending | game logic, to port |
| `box_interface` | pending | game logic, to port |
| `not_implemented` | pending | game logic, to port |
| `alert_box` | pending | game logic, to port |
| `_alert_box` | pending | game logic, to port |
| `drop_ball` | pending | game logic, to port |
| `message` | pending | game logic, to port |
| `pick_up_ball` | pending | game logic, to port |
| `show_hot_keys` | pending | game logic, to port |
| `special_shot_menu` | pending | game logic, to port |
| `bye` | pending | game logic, to port |
| `save_game` | pending | game logic, to port |
| `ok_to_abort_game` | pending | game logic, to port |
| `restore_game` | pending | game logic, to port |
| `goto_hole` | pending | game logic, to port |
| `choose_player` | pending | game logic, to port |
| `pro_show_plyr_stats` | pending | game logic, to port |
| `pick_clubs` | pending | game logic, to port |
| `new_course` | pending | game logic, to port |
| `sprintf` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `printf` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `format` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `ftoa` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fadd` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fcmp` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fneg` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Ftst` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Ffix` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fsub` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `ran` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `randl` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `exp` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fdiv` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fflt` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Fmul` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_abort` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_Output` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_Write` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_OpenLibrary` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strcpy` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strlen` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `toupper` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `tolower` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strcmp` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strncmp` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `int_start` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `int_end` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `lseek` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `creat` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `open` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `read` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `begin` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `geta4` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_main` | partial | golf::Golf::run (intro, title, pro shop, holes so far) |
| `_cli_parse` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strcat` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strncat` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `strncpy` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `mulu` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_wb_parse` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `divs` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `mods` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `modu` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `divu` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `putchar` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `aputc` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `putc` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `fclose` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `flsh_` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `newstream` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `getbuff` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `lmalloc` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `malloc` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `free` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `isatty` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `unlink` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `write` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Chk_Abort` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `exit` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `_exit` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `close` | runtime | Aztec C runtime: Rust std (formatting, strings, allocation, files) |
| `Close` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Close` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_CurrentDir` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `DeleteFile` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_DeleteFile` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `DupLock` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Examine` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Examine` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `ExNext` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Input` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_IoErr` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_IsInteractive` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Lock` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Lock` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Open` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Open` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `ParentDir` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Read` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Read` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Seek` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Seek` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `UnLock` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_UnLock` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Write` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `AddIntServer` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Alert` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `AvailMem` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `CloseLibrary` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_CloseLibrary` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `AllocMem` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_AllocMem` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FindTask` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_FindTask` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_Forbid` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeMem` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_FreeMem` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `GetMsg` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_GetMsg` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `OpenLibrary` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `RemIntServer` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `ReplyMsg` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_ReplyMsg` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_SetSignal` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `SetTaskPri` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `_WaitPort` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `AllocRaster` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `BltBitMap` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `BltBitMapRastPort` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `BltClear` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `BltMaskBitMapRastPort` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `ChangeSprite` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `DisownBlitter` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Draw` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeCprList` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeRaster` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeSprite` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeVPortCopLists` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `GetSprite` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `InitBitMap` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `InitView` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `LoadView` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `MakeVPort` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Move` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `MoveSprite` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `MrgCop` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `OwnBlitter` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `SetAPen` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `SetDrMd` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `SetRGB4` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `Text` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `WaitBlit` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `WaitBOVP` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `WaitTOF` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FindToolType` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `FreeDiskObject` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `GetDiskObject` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `CloseScreen` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `CloseWindow` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `MakeScreen` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `OpenScreen` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `OpenWindow` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `RethinkDisplay` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `WBenchToBack` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |
| `WindowToFront` | os | AmigaOS library call: replaced by native_engine (display, input, files, memory) |

Counts: os 84, partial 13, pending 355, platform 53, ported 41, runtime 61 (of 607).
