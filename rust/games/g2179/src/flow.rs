//! The game's outer flow: the title, a life, turns and lives for one or two players, the message screens, the stage
//! tally and the status bar. Each method is named after the recovered routine it was written from.

use crate::giana::{bcd_add, bcd_add3, bcd_sub, bcd_text, slots, Disp, Giana, Turn};

/// Copies `b` into `t` at `at` (the original patched names and digits into its texts in place).
fn patch(t: &mut [u8], at: usize, b: &[u8]) {
    for (i, &c) in b.iter().enumerate() {
        if let Some(x) = t.get_mut(at + i) {
            *x = c;
        }
    }
}

fn name5(v: &[u8]) -> [u8; 5] {
    let mut n = [b' '; 5];
    for (i, c) in v.iter().take(5).enumerate() {
        n[i] = *c;
    }
    n
}

impl Giana {
    /// The endless title-and-game loop after the start-up (`supervisor_start`).
    pub fn main_loop(&mut self) -> ! {
        loop {
            self.title_screen();
            self.play_one_life();
        }
    }

    /// `title_screen`: the title with the attract stage scrolling back and forth until the joystick picks one player
    /// (left) or two (right); then a new game is set up.
    pub fn title_screen(&mut self) {
        self.stage = 0x21;
        self.scroll = 0;
        self.display_off();
        self.set_start_position();
        self.load_stage();
        self.parse_stage_header();
        self.resolve_object_names();
        self.fill_stage_background();
        self.draw_stage_scenery();
        self.draw_stage_tiles();
        self.print_title_credits();
        self.set_stage_palette();
        self.display_off();
        self.disp = Disp::Title;
        self.display_on();
        self.start_title_tune();
        let two = loop {
            self.wait_line(0x64);
            self.move_cloud_sprites();
            self.animate_map_tiles();
            self.attract_scroll_step();
            self.set_playfield_scroll();
            self.wait_line(0xC8);
            self.restore_bob_backgrounds();
            self.draw_queued_bobs();
            self.read_joystick();
            if self.left {
                break false;
            }
            if self.right {
                break true;
            }
        };
        self.turns[0].active = true;
        self.turns[1].active = two;
        self.player = -1;
        self.player_colour = 0xA0F;
        self.set_player_name(name5(&self.a.texts.giana));
        self.lives = 4;
        self.turns[0].lives = 3;
        self.turns[1].lives = 3;
        self.score = 0;
        self.turns[0].score = 0;
        self.turns[1].score = 0;
        self.stage_bcd = 1;
        self.turns[0].stage_bcd = 1;
        self.turns[1].stage_bcd = 1;
        self.stage = 0;
        self.turns[0].stage = 0;
        self.turns[1].stage = 0;
        self.bonus_idx = 0;
        self.turns[0].bonus_idx = 0;
        self.turns[1].bonus_idx = 0;
        self.item_idx = 0;
        self.scroll = 0;
        self.turns[0].scroll = 0;
        self.turns[1].scroll = 0;
        self.game_ended = false;
        self.game_over = false;
        self.dream_over = false;
        self.quit = false;
    }

    /// `print_title_credits`.
    pub fn print_title_credits(&mut self) {
        Giana::clear_text_picture(&mut self.status);
        self.cursor = (0, 0);
        let t = self.a.texts.credits.clone();
        self.print_text(0, &t);
    }

    /// `attract_scroll_step`.
    pub fn attract_scroll_step(&mut self) {
        self.prev_scroll = self.scroll;
        self.scroll += self.attract_dir;
        if self.scroll <= 0 {
            self.attract_dir = 1;
            return;
        }
        if self.scroll > 0x6BE {
            self.scroll = 0x6BE;
            self.attract_dir = -1;
        }
    }

    /// `play_one_life`: a life (repeated until the game ends or the mouse button quits).
    pub fn play_one_life(&mut self) {
        loop {
            self.start_turn();
            if self.game_ended {
                return;
            }
            self.set_start_position();
            self.load_stage();
            self.display_on();
            self.show_get_ready();
            self.display_off();
            self.build_stage();
            self.draw_status_bar();
            self.start_stage_music();
            self.disp = Disp::Game;
            self.display_on();
            loop {
                self.wait_line(0x50);
                self.update_status_numbers();
                self.collect_diamond_tile();
                self.move_cloud_sprites();
                self.animate_map_tiles();
                self.update_crumbling_bridges();
                self.move_lift();
                self.update_block_hits();
                self.update_enemies();
                self.update_player();
                self.animate_collected_diamonds();
                self.set_playfield_scroll();
                self.apply_horizontal_speed();
                self.count_down_time();
                self.wait_line(0xC8);
                self.restore_bob_backgrounds();
                self.draw_queued_bobs();
                self.check_quit_button();
                if self.quit {
                    return;
                }
                if self.life_over {
                    break;
                }
            }
        }
    }

    /// `check_quit_button`: the left mouse button ends the game.
    pub fn check_quit_button(&mut self) {
        if self.mouse_button() {
            self.quit = true;
        }
    }

    fn save_turn(&mut self) -> Turn {
        Turn {
            score: self.score,
            lives: self.lives,
            stage_bcd: self.stage_bcd,
            stage: self.stage,
            bonus_idx: self.bonus_idx,
            scroll: self.scroll,
            active: true,
        }
    }

    fn load_turn(&mut self, t: Turn) {
        self.score = t.score;
        self.lives = t.lives;
        self.stage_bcd = t.stage_bcd;
        self.stage = t.stage;
        self.bonus_idx = t.bonus_idx;
        self.scroll = t.scroll;
    }

    /// `start_turn`: between lives. The end message, the stage advance after a cleared stage, the high-score screen
    /// after the last life, the lost life, and in a two-player game the swap to the other player.
    pub fn start_turn(&mut self) {
        if self.in_bonus {
            return;
        }
        self.show_wake_up_message();
        let mut to_hs = false;
        if self.lives == 0 && self.dying {
            self.game_over = true;
            to_hs = true;
        }
        if !to_hs && self.cleared {
            self.cleared = false;
            if self.stage >= 0x1F {
                return;
            }
            self.next_stage();
            self.stage_clear_tally();
            self.time = 0x99;
            return;
        }
        if self.game_over {
            self.highscore_screen();
            self.game_over = false;
            self.dream_over = false;
            if self.player >= 0 {
                self.turns[0].active = false;
            } else {
                self.turns[1].active = false;
            }
        }
        self.lose_life();
        self.diamonds = 0;
        self.item_idx = 0;
        self.use_normal_player_frames();
        self.powered = 0;
        self.shot_level = 0;
        self.freezes = 0;
        self.kill_alls = 0;
        self.protection = 0;
        self.extra_life = 0;
        self.time = 0x99;
        let t = self.save_turn();
        let keep = |o: &Turn| o.active;
        if self.player >= 0 {
            let a = keep(&self.turns[0]);
            self.turns[0] = Turn { active: a, ..t };
        } else {
            let a = keep(&self.turns[1]);
            self.turns[1] = Turn { active: a, ..t };
        }
        self.player = -self.player;
        let (p1, p2) = (self.turns[0].active, self.turns[1].active);
        let pick = if self.player >= 0 {
            if p1 {
                Some(0)
            } else if p2 {
                Some(1)
            } else {
                None
            }
        } else if p2 {
            Some(1)
        } else if p1 {
            Some(0)
        } else {
            None
        };
        match pick {
            None => self.game_ended = true,
            Some(0) => {
                self.player = 1;
                let t = self.turns[0];
                self.load_turn(t);
                self.set_player_name(name5(&self.a.texts.giana));
                self.player_colour = 0xA0F;
            }
            Some(_) => {
                self.player = -1;
                let t = self.turns[1];
                self.load_turn(t);
                self.set_player_name(name5(&self.a.texts.maria));
                self.player_colour = 0x080;
            }
        }
    }

    /// `set_player_name`: the name in the get-ready, status-bar and end texts.
    pub fn set_player_name(&mut self, n: [u8; 5]) {
        self.name = n;
    }

    /// `set_start_position`: back from a bonus stage to where the player left, into a bonus stage, or the stage start
    /// (or the halfway point after a death past it); then `reset_life_state`.
    pub fn set_start_position(&mut self) {
        if self.in_bonus && self.bonus_return_scroll >= 0 {
            self.in_bonus = false;
            self.life_over = false;
            self.on_lift = false;
            self.clouds_on = true;
            self.scroll = self.bonus_return_scroll;
            self.prev_scroll = self.scroll;
            self.px = self.saved_pos.0;
            self.py = self.saved_pos.1;
            self.forced_jump = 0x23;
            self.facing_left = self.saved_facing;
            self.saves.clear();
            self.queue.clear();
            self.bonus_idx += 1;
            if self.bonus_idx >= 5 {
                self.bonus_idx = 0;
            }
            return;
        }
        let mut d1 = 0;
        let mut d0 = self.stage;
        if self.in_bonus {
            d0 = 1;
            self.bonus_return_scroll = self.scroll;
            self.scroll = 0;
            self.saved_pos = (self.px, self.py);
            self.saved_facing = self.facing_left;
            self.clouds_on = false;
            if self.stage == 0x1F {
                self.stage += 1;
                self.in_bonus = false;
                self.dream_over = true;
            }
        }
        let mut idx = (d0 * 4) as usize;
        if self.dying {
            self.dying = false;
            if self.scroll >= 0x2C8 {
                d1 = 0x2C8;
                idx += 2;
            }
        }
        let sp = |i: usize| self.a.start_pos.get(i).copied().unwrap_or(0);
        self.px = sp(idx) - 0x14;
        self.py = sp(idx + 1) - 0x30;
        self.scroll = d1;
        self.prev_scroll = d1;
        self.saves.clear();
        self.queue.clear();
        self.jump_i = 0;
        self.fall_i = 0;
        self.speed = 0;
        self.anim_tick = 0;
        self.falling = 0;
        self.jumping = 0;
        self.walk_r = 0;
        self.walk_l = 0;
        self.reset_life_state();
    }

    /// `reset_life_state` (its lists emptied as `clear_object_list` did).
    pub fn reset_life_state(&mut self) {
        self.death_t = 0;
        self.life_over = false;
        self.facing_left = false;
        self.on_lift = false;
        self.use_normal_player_frames();
        if self.powered != 0 {
            self.use_powered_player_frames();
        }
        self.breaking = slots(5);
        self.dropped = slots(15);
        self.flying = slots(15);
        self.falling_bridges = slots(5);
        if self.in_bonus {
            return;
        }
        for e in self.enemies.iter_mut() {
            *e = Default::default();
        }
    }

    /// `use_normal_player_frames`.
    pub fn use_normal_player_frames(&mut self) {
        self.frame_set = 0;
    }
    /// `use_powered_player_frames`.
    pub fn use_powered_player_frames(&mut self) {
        self.frame_set = 1;
    }

    /// `clear_message_screen`: clears the message picture and shows it, with its text colours.
    pub fn clear_message_screen(&mut self) {
        Giana::clear_text_picture(&mut self.msg);
        self.busy(270);
        self.disp = Disp::Message;
        self.col[12] = 0;
        self.col[4] = 0xFF6;
    }

    fn message(&mut self, x: i32, y: i32, text: &[u8]) {
        self.cursor = (x, y);
        self.print_text(1, text);
    }

    /// `show_get_ready`: the player's name and the stage before a life (not in bonus stages, after a bonus stage or
    /// before the last stage).
    pub fn show_get_ready(&mut self) {
        if self.in_bonus || self.forced_jump == 0x23 || self.stage == 0x20 {
            return;
        }
        self.start_get_ready_tune();
        self.display_off();
        self.clear_message_screen();
        self.display_on();
        let st = bcd_text(&[self.stage_bcd]);
        self.wait_line(0xC8);
        let name = self.name;
        let mut t = self.a.texts.get_ready.clone();
        patch(&mut t, 0x29, &name);
        patch(&mut t, 0x44, &st);
        self.message(0, 0x5C, &t);
        self.wait_fire();
    }

    /// `show_out_of_time`.
    pub fn show_out_of_time(&mut self) {
        self.display_off();
        self.clear_message_screen();
        self.display_on();
        let t = self.a.texts.out_of_time.clone();
        self.message(0, 0x5C, &t);
        self.wait_fire();
        self.display_off();
        self.time = 0x99;
    }

    /// `show_game_over`: never called by the original (kept for the function map).
    #[allow(dead_code)]
    pub fn show_game_over(&mut self) {
        self.display_off();
        self.clear_message_screen();
        self.display_on();
        let t = self.a.texts.game_over.clone();
        self.message(0x0F, 0x5C, &t);
        self.wait_fire();
        self.display_off();
    }

    /// `show_wake_up_message`: the end of a game.
    pub fn show_wake_up_message(&mut self) {
        if !self.game_over {
            return;
        }
        self.display_off();
        self.clear_message_screen();
        self.display_on();
        let name = self.name;
        let mut t = self.a.texts.wake.clone();
        patch(&mut t, 13, &name);
        self.message(0, 0x5C, &t);
        self.wait_fire();
        self.display_off();
    }

    /// `stage_clear_tally`: the time left counts down second by second, ten points each.
    pub fn stage_clear_tally(&mut self) {
        self.show_warp_message();
        if self.diamonds == 0 {
            return;
        }
        let t = self.a.texts.tally1.clone();
        self.message(5, 0x5C, &t);
        let t = self.a.texts.tally2.clone();
        self.message(5, 0x6C, &t);
        loop {
            let mut x = false;
            self.time = bcd_sub(self.time, 1, &mut x);
            self.delay_loop(0x1D4C);
            let t = bcd_text(&[self.time]);
            self.t_time = [t[0], t[1]];
            self.pending = 0x10;
            self.add_points();
            self.refresh_score_text();
            self.wait_line(0xC8);
            let tt = self.t_time;
            self.message(7, 0x6C, &tt);
            let ts = self.t_score;
            self.message(0x18, 0x6C, &ts);
            self.tally_t -= 1;
            if self.tally_t < 0 {
                self.tally_t = 9;
                self.play_sound_effect(8);
            }
            if self.time == 0 {
                break;
            }
        }
        self.time = 0x99;
    }

    fn refresh_score_text(&mut self) {
        let t = bcd_text(&[
            (self.score >> 16) as u8,
            (self.score >> 8) as u8,
            self.score as u8,
        ]);
        self.t_score.copy_from_slice(&t[..6]);
    }

    /// `show_warp_message`: after a warp, the stage it leads to.
    pub fn show_warp_message(&mut self) {
        if self.warp {
            self.delay_loop(0x30D40);
            self.display_off();
            self.clear_message_screen();
            self.display_on();
            self.warp = false;
            let st = bcd_text(&[self.stage_bcd]);
            let mut t = self.a.texts.warp.clone();
            patch(&mut t, 0x2C, &st);
            self.message(0, 0x5C, &t);
            self.wait_fire();
        } else {
            self.display_off();
            self.clear_message_screen();
            self.display_on();
        }
    }

    /// `count_down_time`: a second off the stage clock every fifty frames; at zero the life is lost.
    pub fn count_down_time(&mut self) {
        self.second_t += 1;
        if self.second_t < 50 {
            return;
        }
        self.second_t = 0;
        if self.time != 0 {
            let mut x = false;
            self.time = bcd_sub(self.time, 1, &mut x);
            if self.time != 0 {
                return;
            }
        }
        if self.in_bonus {
            return;
        }
        self.dying = true;
        self.life_over = true;
        self.show_out_of_time();
    }

    fn status_text(&mut self, x: i32, y: i32, t: &[u8]) {
        self.cursor = (x, y);
        self.print_text(0, t);
    }

    /// `draw_status_bar`.
    pub fn draw_status_bar(&mut self) {
        Giana::clear_text_picture(&mut self.status);
        // Measured against the reference run (with the music start and display set-up after it).
        self.busy(40 + 313 * 2);
        self.col[5] = self.player_colour;
        let name = self.name;
        self.status_text(2, 0, &name);
        let tx = self.a.texts.clone();
        self.status_text(0x0A, 0, &tx.bonus);
        self.status_text(0x12, 0, &tx.lives);
        self.status_text(0x1A, 0, &tx.stage);
        self.status_text(0x23, 0, &tx.time);
        let t = self.t_score;
        self.status_text(1, 8, &t);
        let t = self.t_dia;
        self.status_text(0x0D, 8, &t);
        let t = self.t_lives;
        self.status_text(0x14, 8, &t);
        let s = bcd_text(&[self.stage_bcd]);
        self.t_stage = [s[0], s[1]];
        let t = self.t_stage;
        self.status_text(0x1C, 8, &t);
        let t = self.t_time;
        self.status_text(0x24, 8, &t);
    }

    /// `update_status_numbers`: reprints the numbers that changed since the last frame.
    pub fn update_status_numbers(&mut self) {
        if self.score != self.shown_score {
            self.shown_score = self.score;
            self.refresh_score_text();
            let t = self.t_score;
            self.status_text(1, 8, &t);
        }
        if self.diamonds != self.shown_diamonds {
            self.shown_diamonds = self.diamonds;
            let s = bcd_text(&[self.diamonds]);
            self.t_dia = [s[0], s[1]];
            let t = self.t_dia;
            self.status_text(0x0D, 8, &t);
        }
        if self.lives != self.shown_lives {
            self.shown_lives = self.lives;
            let s = bcd_text(&[self.lives]);
            self.t_lives = [s[0], s[1]];
            let t = self.t_lives;
            self.status_text(0x14, 8, &t);
        }
        if self.time != self.shown_time {
            self.shown_time = self.time;
            let s = bcd_text(&[self.time]);
            self.t_time = [s[0], s[1]];
            let t = self.t_time;
            self.status_text(0x24, 8, &t);
        }
    }

    /// `start_stage_music`: the castle tune every fourth stage, otherwise the stage tune (unless already playing).
    pub fn start_stage_music(&mut self) {
        if self.stage == 0x20 {
            return;
        }
        if (self.stage + 1) % 4 == 0 {
            if self.music.tune != 3 {
                self.start_castle_tune();
            }
        } else if self.music.tune != 2 {
            self.start_stage_tune();
        }
    }

    /// `move_cloud_sprites`: the clouds drift at half the scroll speed and wrap at the screen edges.
    pub fn move_cloud_sprites(&mut self) {
        if !self.clouds_on {
            self.dma_sprites = false;
            return;
        }
        self.dma_sprites = true;
        self.cloud_acc += self.prev_scroll - self.scroll;
        if self.cloud_acc.abs() > 1 {
            let d = self.cloud_acc >> 1;
            self.clouds_x.0 += d;
            self.clouds_x.1 += d;
            self.cloud_acc = 0;
        }
        for c in [&mut self.clouds_x.0, &mut self.clouds_x.1] {
            if *c < -40 {
                *c = 0x14A;
            }
            if *c > 0x14A {
                *c = -40;
            }
        }
        // `set_sprite_position`: three sprites a cloud, side by side.
        let (a, b) = self.clouds_x;
        for k in 0..3 {
            self.cloud_pos[k] = (a + 16 * k as i32 - 6, 0x11);
            self.cloud_pos[3 + k] = (b + 16 * k as i32 - 6, 0x16);
        }
    }

    /// `set_playfield_scroll`: the scroll the display shows from the next frame.
    pub fn set_playfield_scroll(&mut self) {
        self.shown_scroll = self.scroll;
    }

    /// `add_diamond`: one bonus diamond; at a hundred the count wraps and the player gets a life.
    pub fn add_diamond(&mut self) {
        let mut x = false;
        self.diamonds = bcd_add(self.diamonds, 1, &mut x);
        if x {
            self.add_life();
        }
    }

    /// `add_life`.
    pub fn add_life(&mut self) {
        let mut x = false;
        self.lives = bcd_add(self.lives, 1, &mut x);
        self.play_sound_effect(6);
    }

    /// `lose_life` (the trainer option leaves the lives alone, as the WHDLoad install's trainer key did).
    pub fn lose_life(&mut self) {
        if self.trainer {
            return;
        }
        let mut x = false;
        self.lives = bcd_sub(self.lives, 1, &mut x);
    }

    /// `next_stage`.
    pub fn next_stage(&mut self) {
        self.stage += 1;
        let mut x = false;
        self.stage_bcd = bcd_add(self.stage_bcd, 1, &mut x);
    }

    /// `previous_stage`: never called by the original (kept for the function map).
    #[allow(dead_code)]
    pub fn previous_stage(&mut self) {
        self.stage -= 1;
        let mut x = false;
        self.stage_bcd = bcd_sub(self.stage_bcd, 1, &mut x);
    }

    /// `add_points`: the pending points onto the six-digit score.
    pub fn add_points(&mut self) {
        self.score = bcd_add3(self.score, self.pending);
    }
}
