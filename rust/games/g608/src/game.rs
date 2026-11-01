//! E-Motion's main program (mainbit): the attract loop (title picture, high scores, the instruction demo), the
//! options screen, a game (levels, bonus levels, secret bonuses, the status line, game over), the high-score table
//! and the offer to carry on from level 25. Each routine keeps its name from the recovered code (offsets in +$XXXX);
//! the objects are in `objects.rs`.

use crate::draw::On;
use crate::emo::{
    Display, Emo, Obj, DOWN, FIRE, KEY_ESC, KEY_P, KEY_RETURN, LEFT, OBJS, RIGHT, UP,
};
use crate::ops::random_next;

/// Where a game goes after one of its routines (the original jumped straight to these labels).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Go {
    /// Carry on.
    On,
    /// Title (l0_35DC, Escape or the end of practice).
    Title,
    /// The title picture again (l0_35E4, the end of the demo).
    TitlePicture,
    /// Start a game from the options screen (l0_3E68: fire during the attract loop or the demo).
    Options,
    /// The level again with full energy (l0_3EB4: a ship lost, lives left).
    Restart,
    /// The level (l0_3EBA: next level, or a level chosen with the cheat keys).
    Level,
    /// The game is over (l0_3850: game over or the last level done).
    Over,
    /// Carry on from level 25 (l0_3E96).
    Continue,
}

impl Emo {
    /// program_entry (+$0) and the start-up at +$3594.
    pub fn game_main(&mut self) {
        self.display = Display::Game;
        self.pal_sel = crate::emo::Pal::Boot;
        self.all32 = true;
        self.select_draw_buffer(0);
        self.load_high_scores();
        let mut go = Go::Title;
        loop {
            go = match go {
                Go::Title => self.attract(true),
                Go::TitlePicture => self.attract(false),
                Go::Options => self.start_from_options(),
                Go::Continue => {
                    self.level = 24;
                    self.start_sound_effects();
                    self.new_game()
                }
                Go::Over => self.game_over(),
                other => other,
            };
            if go == Go::Restart || go == Go::Level || go == Go::On {
                go = Go::Title;
            }
        }
    }

    /// The attract loop (l0_35DC / l0_35E4): the title picture, then the high scores, then the demo.
    fn attract(&mut self, fade_first: bool) -> Go {
        self.start_music();
        if fade_first {
            self.fade_out();
        }
        self.start_music();
        // the title picture, decoded straight onto the first screen
        let a = self.a.clone();
        self.display = Display::Picture;
        self.screens[0].copy_from_slice(&a.pic_title.px);
        self.keys.clear();
        self.all32 = true;
        self.pic_cmap = a.cmap_title.clone();
        self.pal_sel = crate::emo::Pal::Picture;
        self.fade_dir = 1;
        self.clr_frame16();
        loop {
            self.wait_frame();
            self.read_key_check_cheat();
            if (self.joy1 | self.joy2) & FIRE != 0 {
                return Go::Options;
            }
            if self.f16() >= 250 {
                break;
            }
        }
        self.fade_out();
        self.display = Display::Game;
        self.build_title_screen();
        self.high_score_screen()
    }

    /// l0_364C: the high-score table for five seconds, then the demo.
    fn high_score_screen(&mut self) -> Go {
        let a = self.a.clone();
        self.heading(&a.heading_scores);
        self.draw_high_score_table();
        self.all32 = false;
        self.fade_in();
        self.clr_frame16();
        loop {
            self.wait_frame();
            self.read_key_check_cheat();
            if (self.joy1 | self.joy2) & FIRE != 0 {
                return Go::Options;
            }
            if self.f16() >= 250 {
                break;
            }
        }
        self.mode = 2;
        self.level = 0x33;
        self.new_game()
    }

    /// build_title_screen / draw_title_layout (+$3800, +$3810).
    fn build_title_screen(&mut self) {
        self.build_gradient(0x408);
        let r = self.a.tables.ramps_title;
        self.build_palette_ramps(r);
        self.draw_title_layout();
    }

    fn draw_title_layout(&mut self) {
        self.clear_clean();
        let a = self.a.clone();
        self.build_layout(&a.layout_title[0], &a.layout_title[1]);
        self.pal_sel = crate::emo::Pal::Title;
    }

    /// l0_3E68: the options screen, then a game.
    fn start_from_options(&mut self) -> Go {
        self.level = 0;
        if self.options_screen() == Go::Title {
            return Go::Title;
        }
        // The trainer's start level replaced the level's clear here.
        if self.start_level > 1 {
            self.level = (self.level & 0xFF00) | (self.start_level - 1);
        }
        self.turn_rate = if self.options[5] != 0 { 0x800 } else { 0x1000 };
        self.mode = self.options[0];
        if self.mode == 2 {
            self.level = 0x33;
        }
        self.start_sound_effects();
        self.new_game()
    }

    /// options_screen (+$3AD4).
    fn options_screen(&mut self) -> Go {
        while (self.joy1 | self.joy2) & FIRE != 0 {
            self.wait_frame();
        }
        self.fade_out();
        self.set_game_display();
        let r = self.a.tables.ramps_title;
        self.build_palette_ramps(r);
        self.build_gradient(0x62f);
        self.clear_clean();
        let a = self.a.clone();
        for (k, l) in a.texts.options.iter().enumerate() {
            self.text(On::Clean, 56, 20 + 26 * k as i32, l.as_bytes(), false);
        }
        let mut row = 0usize;
        loop {
            self.copy_clean();
            for k in 0..6 {
                if k == row {
                    self.text(On::Back, 24, 20 + 26 * k as i32, b"@", false);
                }
                let v = &a.texts.option_values[k][self.options[k] as usize];
                self.text(On::Back, 176, 20 + 26 * k as i32, v.as_bytes(), false);
            }
            self.mask_edges();
            self.swap_screens();
            self.fade_in();
            if self.read_key() == Some(KEY_ESC) {
                return Go::Title;
            }
            let d0 = self.joy1;
            let d1 = self.prev1;
            self.prev1 = d0;
            let new = d0 & !d1;
            if new & DOWN != 0 {
                row = (row + 1) % 6;
            }
            if new & UP != 0 {
                row = (row + 5) % 6;
            }
            let max = a.tables.option_max[row];
            if new & LEFT != 0 {
                if self.options[row] == 0 {
                    self.options[row] = max;
                } else {
                    self.options[row] -= 1;
                }
            }
            if new & RIGHT != 0 {
                self.options[row] += 1;
                if self.options[row] > max {
                    self.options[row] = 0;
                }
            }
            if new & FIRE != 0 {
                return Go::On;
            }
        }
    }

    /// l0_3E9A: a new game (lives, score), then its levels.
    fn new_game(&mut self) -> Go {
        self.energy_due = 0;
        self.lives = 3;
        self.score = [0; 6];
        self.secret_level = 0xFF00 | (self.secret_level & 0xFF);
        let mut go = Go::Restart;
        loop {
            if go == Go::Restart {
                self.energy = 0x68;
            }
            go = self.play_level();
            match go {
                Go::Restart | Go::Level => continue,
                other => return other,
            }
        }
    }

    /// l0_3EBA: build a level and run it (the main loop at +$40E0).
    fn play_level(&mut self) -> Go {
        self.fade_out();
        self.pods4 = false;
        self.wrapped = false;
        self.turned_right = false;
        self.pod_count = 0;
        self.clr_frame16();
        self.set_game_display();
        self.clear_clean();
        // measured on the reference: clearing the clean screen and setting up took the original two frames
        self.busy(2);
        let mut d0 = self.level;
        if self.mode == 1 && (d0 == 0x18 || d0 == 0x26 || d0 == 0x2a) {
            self.level += 1;
            d0 += 1;
        }
        if d0 == 7 || d0 == 0x14 || d0 == 0x21 {
            loop {
                let r = (random_next(&mut self.rng) as u16 & 7).wrapping_add(self.level);
                if r & 3 != 3 {
                    self.secret_level = r;
                    break;
                }
            }
        }
        self.spark_timer = 0;
        if self.level == self.secret_level {
            self.spark_timer = random_next(&mut self.rng) as u16 & 0xFF;
            self.secret_level = 0xFF00 | (self.secret_level & 0xFF);
        }
        let a = self.a.clone();
        let Some(lv) = a.levels.get(self.level as usize) else {
            return Go::TitlePicture;
        };
        self.effect_named("level_start");
        self.build_gradient(lv.gradient);
        self.build_palette_ramps(lv.ramps);
        self.clock = [0; 4];
        self.bonus = lv.bonus;
        if self.bonus != 0 {
            self.bonus_level_screen();
            self.pod_frame = self.frame;
            if self.mode != 1 {
                let t = if self.options[1] != 0 {
                    lv.times[1]
                } else {
                    lv.times[0]
                };
                self.clock = t.to_be_bytes();
            }
        }
        self.pod_time = lv.pod_time;
        // the objects: player two's ship, player one's, then the level's. Only the fields the level sets are written:
        // the rest (a position's fraction, a tether's unused fields) keep what the slot held before, as they did.
        self.objs[0].kind = 0;
        let ship = |o: &mut Obj, p: (i32, i32), colour: u16, angle: u16| {
            o.kind = 4;
            o.x = p.0;
            o.y = p.1;
            o.vx = 0;
            o.vy = 0;
            o.w22 = angle;
            o.size = 0x10;
            o.colour = colour;
        };
        if self.options[1] != 0 && self.level < 0x33 {
            ship(&mut self.objs[0], lv.ship2, 0xFFFE, 0);
        }
        ship(&mut self.objs[1], lv.ship1, 0xFFFF, 0xC000);
        for (i, o) in lv.objects.iter().enumerate() {
            let s = &mut self.objs[2 + i];
            match *o {
                crate::assets::LevelObj::Tether(p, q) => {
                    s.kind = 0x10;
                    s.x = (p as i32) << 16 | q as i32;
                }
                crate::assets::LevelObj::Thing { kind, x, y, colour } => {
                    s.kind = kind;
                    s.x = (s.x & 0xFFFF) | (x as i32) << 16;
                    s.y = (s.y & 0xFFFF) | (y as i32) << 16;
                    s.size = 0;
                    s.colour = colour;
                    s.vx = 0;
                    s.vy = 0;
                    s.max = if kind == 0x2C { 0xC } else { 4 };
                }
            }
        }
        for s in self.objs[2 + lv.objects.len()..OBJS].iter_mut() {
            s.kind = 0;
        }
        self.objs[OBJS].kind = 0xFFFF;
        self.build_layout(&lv.hbars, &lv.vbars);
        if self.level >= 0x33 {
            let page =
                &a.texts.instructions[(self.level - 0x33) as usize % a.texts.instructions.len()];
            self.text(On::Clean, 56, 130, page[0].as_bytes(), false);
            self.text(On::Clean, 56, 150, page[1].as_bytes(), false);
            self.text(
                On::Clean,
                112,
                90,
                a.texts.instructions_title.as_bytes(),
                false,
            );
        }
        // measured: the bars took the original a frame, and copying the clean screen to both screens three
        self.busy(1);
        self.select_draw_buffer(0);
        self.copy_clean_both();
        self.busy(3);
        self.fade_dir = 1;
        self.secret_timer = 0;
        self.banner_timer = if self.level == 0x32 { 0xFA } else { 0x32 };
        self.f404 = false;
        self.f405 = false;
        self.f406 = false;
        self.kill_touch = false;
        self.exploding = false;
        self.over_timer = 0;
        self.chain = 0;
        loop {
            let go = self.level_frame();
            if go != Go::On {
                return go;
            }
        }
    }

    /// One pass of the main loop (+$40E0 to +$42BE).
    fn level_frame(&mut self) -> Go {
        // wait_for_vblank_and_copy_planes (+$2602): what was drawn on this screen two passes ago is cleaned away
        self.copy_clean();
        let g = self.update_and_draw_objects();
        if g != Go::On {
            return g;
        }
        if self.spark_timer != 0 {
            self.spark_timer -= 1;
            if self.spark_timer == 0 {
                self.clear_object_state();
            }
        }
        if self.banner_timer != 0 {
            self.banner_timer -= 1;
            let g = self.draw_level_banner();
            if g != Go::On {
                return g;
            }
        }
        if self.secret_timer != 0 {
            self.secret_timer -= 1;
            if self.secret_timer == 0 {
                if self.bonus != 0 {
                    self.bonus_points_screen();
                }
                self.level += 1;
                return Go::Level;
            }
            self.draw_secret_bonus();
        }
        if self.over_timer != 0 {
            self.over_timer -= 1;
            if self.over_timer == 0 {
                return Go::Over;
            }
            self.draw_game_over();
        }
        self.mask_edges();
        self.draw_score();
        self.draw_lives();
        if self.bonus != 0 {
            if self.mode != 1 {
                self.draw_clock();
            }
        } else {
            self.update_energy_bar();
            self.draw_energy();
        }
        self.swap_screens();
        let key = self.read_key();
        if key == Some(KEY_ESC) {
            return Go::Title;
        }
        if key == Some(KEY_P) {
            self.wait_key();
        }
        if self.cheat_on || self.mode == 1 {
            if let Some(k @ 0x50..=0x54) = key {
                if k == 0x54 {
                    self.explode_ships();
                } else {
                    let d = [1i16, -1, 10, -10][(k - 0x50) as usize];
                    let l = self.level as i16 + d;
                    if (0..0x33).contains(&l) && (self.mode != 1 || l < 5) {
                        self.level = l as u16;
                        return Go::Level;
                    }
                }
            }
        }
        if self.mode == 2 {
            if self.f16() >= 0xFA {
                return self.level_end_secrets();
            }
            if (self.joy1 | self.joy2) & FIRE != 0 {
                return Go::Options;
            }
            return Go::On;
        }
        if self.secret_timer != 0 {
            return Go::On;
        }
        if !self.f406 {
            if self.alive != 0 || self.energy_due != 0 || self.exploding || self.over_timer != 0 {
                return Go::On;
            }
            if !self.f405 {
                self.add_score(0x50000);
            }
        }
        // l0_4214
        if self.bonus != 0 {
            let mut d0 = 1;
            if self.clock[2] == 3 {
                return self.award(d0);
            }
            if self.bonus == 4 {
                d0 = 3;
                self.pod_count = self.pod_count.wrapping_sub(1);
                if self.pod_count == 0 {
                    return self.award(d0);
                }
            }
            if self.mode == 1 {
                return self.level_end_secrets();
            }
            self.bonus_points_screen();
            return self.next_level();
        }
        if !self.f404 {
            self.add_score(0x100_0000);
        }
        self.level_end_secrets()
    }

    /// l0_4250: the secret bonuses at the end of a level.
    fn level_end_secrets(&mut self) -> Go {
        if self.mode == 2 {
            return self.next_level();
        }
        if !self.f404 {
            if !self.wrapped && self.level == 0 {
                return self.award(4);
            }
            if !self.turned_right {
                return self.award(5);
            }
        }
        if self.pods4 {
            return self.award(2);
        }
        self.next_level()
    }

    /// l0_427C.
    fn next_level(&mut self) -> Go {
        if self.mode == 1 && self.level == 4 {
            return Go::Title;
        }
        // measured on the reference (the demo's levels): the move to the next level shows a frame later than the
        // pass that decided it
        self.busy(1);
        self.level += 1;
        Go::Level
    }

    /// l0_4296: a secret bonus: 8000 points, four fireworks and five seconds of its screen.
    fn award(&mut self, n: u16) -> Go {
        self.secret = n;
        self.add_score(0x800_0000);
        for _ in 0..4 {
            self.initialize_extra_output_block();
        }
        self.pods4 = false;
        self.wrapped = false;
        self.turned_right = false;
        self.pod_count = 0;
        self.secret_timer = 0xFA;
        Go::On
    }

    /// update_energy_bar (+$42C2): the energy moves one unit a pass towards what it owes; at nothing the ships blow up.
    fn update_energy_bar(&mut self) {
        if self.energy_due < 0 {
            self.energy_due += 1;
            self.energy += 1;
            if self.energy >= 0x68 {
                self.energy_due = 0;
                if !self.trainer[1] {
                    self.energy -= 1;
                }
            }
        } else if self.energy_due > 0 {
            self.energy_due -= 1;
            if self.trainer[1] {
                return;
            }
            self.energy -= 1;
            if self.energy < 0 {
                self.energy = 0;
                self.energy_due = 0;
                self.explode_ships();
            }
        }
    }

    /// set_game_display (+$4306).
    pub fn set_game_display(&mut self) {
        self.display = Display::Game;
        self.all32 = false;
        self.pal_sel = crate::emo::Pal::Game;
    }

    /// build_gradient_colours (+$127C): the 15 gradient band colours from one colour.
    pub fn build_gradient(&mut self, c: u16) {
        let mut d0 = c;
        let mut d1 = d0 & 0x888;
        d0 = d0.wrapping_sub(d1);
        d1 >>= 3;
        let d2 = d1 & d0;
        d1 = d1.wrapping_sub(d2).wrapping_sub(d2);
        d0 = d0.wrapping_add(d0);
        for k in 0..15 {
            self.band_base[k] = d0;
            d0 = d0.wrapping_add(d1);
        }
    }

    /// build_palette_ramps (+$4344): colours 0 to 23 are three ramps of eight.
    pub fn build_palette_ramps(&mut self, r: [u16; 3]) {
        for (k, step) in r.iter().enumerate() {
            let mut v: u16 = 0;
            for i in 0..8 {
                self.pal_game[k * 8 + i] = v;
                v = v.wrapping_add(*step);
            }
        }
    }

    /// swap_screens (+$4410): show the screen just drawn (waiting so a pass takes at least two frames) and draw on the
    /// other.
    pub fn swap_screens(&mut self) {
        let d0 = self.f16().wrapping_sub(self.last_swap).wrapping_sub(2) as i16;
        if d0 < 0 {
            self.wait_frame();
        }
        self.shown = self.drawbuf;
        self.select_draw_buffer(1 - self.drawbuf);
    }

    /// select_draw_buffer (+$4430).
    pub fn select_draw_buffer(&mut self, b: usize) {
        self.drawbuf = b;
        self.last_swap = self.f16();
        self.wait_frame();
    }

    /// wait_key (+$1164): the pause, until a key.
    fn wait_key(&mut self) -> u8 {
        loop {
            if let Some(k) = self.read_key() {
                return k;
            }
            self.wait_frame();
        }
    }

    /// Work the original took `n` frames to do (measured on the reference run): the frame interrupt's work goes on.
    pub fn busy(&mut self, n: u32) {
        for _ in 0..n {
            self.wait_frame();
        }
    }

    pub fn clr_frame16(&mut self) {
        self.frame &= 0xFFFF_0000;
    }

    /// read_key_check_cheat (+$37C2): typing MOONUNIT turns on the level keys. (The keyboard's letters do not reach
    /// the native build, so only a front end with text input could type it; the routine is kept as it was.)
    fn read_key_check_cheat(&mut self) {
        let Some(k) = self.read_key() else { return };
        let word = self.a.texts.cheat.clone().into_bytes();
        let typed = match k {
            0x45 => 0,
            _ => k,
        };
        if word.get(self.cheat_pos) == Some(&typed) {
            self.cheat_pos += 1;
            if self.cheat_pos == word.len() {
                self.cheat_on = true;
            }
        } else {
            self.cheat_pos = 0;
        }
    }

    // ---- scores

    /// add_score (+$4478): BCD digits; an extra life when the ten-thousands digit leaves an odd number.
    pub fn add_score(&mut self, d0: u32) {
        if self.mode == 1 {
            return;
        }
        let old = self.score[1];
        add_digits(&mut self.score, d0);
        if self.score[1] != old && old & 1 != 0 {
            self.effect_named("extra_life");
            let mut d = self.lives.wrapping_add(1);
            if (d as u8 as i8) >= 10 {
                d = d.wrapping_sub(0xFF0A);
            }
            if !self.trainer[0] || true {
                self.lives = d;
            }
        }
    }

    /// add_bonus_and_score (+$446C).
    pub fn add_bonus_and_score(&mut self, d0: u32) {
        add_digits(&mut self.bonus_score, d0);
        self.add_score(d0);
    }

    // ---- screens between levels

    /// bonus_level_screen (+$4530).
    fn bonus_level_screen(&mut self) {
        self.bonus_score = [0; 6];
        let a = self.a.clone();
        self.build_layout(&a.layout_panel[0], &a.layout_panel[1]);
        self.copy_clean();
        self.mask_edges();
        let t = &a.texts.bonus_level;
        self.text(On::Back, 112, 60, t[0].as_bytes(), false);
        let k = ((self.bonus / 4) as usize * 2)
            .saturating_sub(1)
            .min(t.len() - 2);
        self.text(On::Back, 56, 100, t[k].as_bytes(), false);
        self.text(On::Back, 56, 120, t[k + 1].as_bytes(), false);
        self.swap_screens();
        self.fade_dir = 1;
        while (self.joy1 | self.joy2) & FIRE == 0 {
            self.wait_frame();
        }
        self.fade_out();
        self.clear_clean();
    }

    /// bonus_points_screen (+$44CE).
    fn bonus_points_screen(&mut self) {
        if !self.f406 {
            self.effect_named("bonus_points");
        }
        self.fade_out();
        self.clear_clean();
        let a = self.a.clone();
        self.build_layout(&a.layout_panel[0], &a.layout_panel[1]);
        self.copy_clean();
        self.mask_edges();
        self.text(On::Back, 80, 60, a.texts.bonus_awarded.as_bytes(), false);
        let b = self.bonus_score;
        self.digits(On::Back, 136, 100, &b[0..1], false);
        self.digits(On::Back, 144, 100, &b[1..3], true);
        self.digits(On::Back, 160, 100, &b[3..5], true);
        self.digits(On::Back, 176, 100, &b[5..6], false);
        self.swap_screens();
        self.fade_dir = 1;
        while (self.joy1 | self.joy2) & FIRE == 0 {
            self.wait_frame();
        }
    }

    /// secret_bonus_screen (+$468A): "SECRET BONUS #n" and what it was for.
    fn draw_secret_bonus(&mut self) {
        let a = self.a.clone();
        let t = &a.texts.secret;
        self.text(On::Back, 96, 90, t[0].as_bytes(), true);
        let n = self.secret;
        self.digits(On::Back, 208, 90, &[n as u8], false);
        if let Some(s) = t.get(n as usize) {
            self.text(On::Back, 80, 110, s.as_bytes(), true);
        }
    }

    /// draw_level_banner (+$4768): "LEVEL nn" while the banner lasts; on level 51 (the last) its congratulations,
    /// after which the game is over.
    fn draw_level_banner(&mut self) -> Go {
        if self.level > 0x32 {
            return Go::On;
        }
        if self.level == 0x32 {
            if self.banner_timer == 0 {
                return Go::Over;
            }
            let c = self.a.texts.congratulations.clone();
            self.text(On::Back, 96, 90, c.as_bytes(), true);
            return Go::On;
        }
        self.text(On::Back, 128, 90, b"LE", true);
        self.text(On::Back, 144, 90, b"VE", true);
        self.text(On::Back, 160, 90, b"L", false);
        let n = self.level + 1;
        self.digits(On::Back, 176, 90, &[(n / 10) as u8, (n % 10) as u8], true);
        Go::On
    }

    /// draw_game_over (+$47EE): "GAME OVER" a letter every four passes.
    fn draw_game_over(&mut self) {
        let n = ((0x96 - self.over_timer as i32) >> 2).min(9) as usize;
        let t = self.a.texts.game_over.clone().into_bytes();
        let k = (n + 1).min(t.len());
        self.text(On::Back, 120, 90, &t[..k], false);
    }

    /// l0_3850: the game is over: the high-score table, the offer to carry on, then the title's high scores.
    fn game_over(&mut self) -> Go {
        self.wait_effects_idle();
        self.fade_out();
        self.build_title_screen();
        let s = self.score;
        self.enter_high_score(s);
        if self.offer_continue() {
            return Go::Continue;
        }
        self.start_music();
        self.draw_title_layout();
        self.high_score_screen()
    }

    /// enter_high_score (+$3870): a place in the table and a name typed into it.
    fn enter_high_score(&mut self, s: [u8; 6]) {
        let mut entry: Vec<u8> = s.iter().map(|d| d + b'0').collect();
        entry.extend_from_slice(b".L");
        let mut lv = self.level;
        if lv != 0x32 {
            lv += 1;
        }
        entry.push(b'0' + (lv / 10) as u8);
        entry.push(b'0' + (lv % 10) as u8);
        let Some(place) = (0..8).find(|&i| !less(&entry, &self.high[i][..10])) else {
            return;
        };
        // the name-entry sound loops on channel 0 while the name is typed
        let a = self.a.clone();
        if let Some(smp) = a.sfx.get("name_entry") {
            let b = native_engine::audio::Block::whole(smp);
            self.sounds.push(crate::co::Snd::Start {
                ch: 0,
                block: b.clone(),
                next: b,
                rate: crate::assets::SAMPLE_RATE,
                vol: self.vol(),
            });
        }
        self.high.insert(place, vec![b' '; 28]);
        self.high.truncate(8);
        self.high[place][..10].copy_from_slice(&entry);
        self.high[place][10] = b'.';
        self.high[place][11] = b'.';
        self.heading(&a.heading_name);
        let mut cur = 0usize;
        let mut letter = b'A';
        let mut prev = self.joy1;
        loop {
            if cur != 16 {
                self.high[place][12 + cur] = b'_';
            }
            self.draw_high_score_table();
            self.fade_in();
            if cur != 16 {
                self.high[place][12 + cur] = b' ';
            }
            // the name is typed with the controller: up and down choose the letter, right takes it, left rubs out,
            // fire (or Return) ends
            let k = loop {
                self.wait_frame();
                if let Some(k) = self.read_key() {
                    if k == KEY_RETURN {
                        break 0x0Du16;
                    }
                }
                let j = self.joy1;
                let new = j & !prev;
                prev = j;
                if new & FIRE != 0 {
                    break 0x0D;
                }
                if new & UP != 0 {
                    letter = next_letter(letter, 1);
                    break letter as u16 | 0x100;
                }
                if new & DOWN != 0 {
                    letter = next_letter(letter, -1);
                    break letter as u16 | 0x100;
                }
                if new & RIGHT != 0 {
                    break letter as u16;
                }
                if new & LEFT != 0 {
                    break 8;
                }
            };
            match k {
                0x0D => break,
                8 => {
                    if cur > 0 {
                        if cur != 16 {
                            self.high[place][12 + cur] = b' ';
                        }
                        cur -= 1;
                    }
                }
                k if k & 0x100 != 0 => {
                    // show the letter being chosen at the cursor
                    if cur != 16 {
                        self.high[place][12 + cur] = k as u8;
                        self.draw_high_score_table();
                        self.high[place][12 + cur] = b' ';
                    }
                }
                k => {
                    if cur != 16 {
                        self.high[place][12 + cur] = k as u8;
                        cur += 1;
                    }
                }
            }
        }
        self.sounds.push(crate::co::Snd::Stop(0));
        self.save_high_scores();
    }

    /// draw_high_score_table (+$3978).
    fn draw_high_score_table(&mut self) {
        self.copy_clean();
        for i in 0..8 {
            let row = self.high[i].clone();
            self.text(On::Back, 48, 52 + 16 * i as i32, &row, true);
        }
        self.mask_edges();
        self.swap_screens();
    }

    /// load_high_scores (+$39B8): from the save store (the original read its own track on the disk).
    fn load_high_scores(&mut self) {
        self.wait_frame();
        if let Some(b) = self.saved.take() {
            if b.len() == 8 * 28 {
                self.high = b.chunks(28).map(|c| c.to_vec()).collect();
                self.high_loaded = true;
            }
        }
    }

    /// save_high_scores (+$39FA).
    fn save_high_scores(&mut self) {
        self.save_out = Some(self.high.concat());
    }

    /// offer_continue_from_level_25 (+$3A1E): ten seconds to press fire and carry on from level 25.
    fn offer_continue(&mut self) -> bool {
        if self.level < 0x18 || self.level == 0x32 {
            return false;
        }
        self.fade_out_if_needed();
        self.build_gradient(0x408);
        let r = self.a.tables.ramps_title;
        self.build_palette_ramps(r);
        self.clear_clean();
        let a = self.a.clone();
        self.build_layout(&a.layout_panel[0], &a.layout_panel[1]);
        for d7 in (0..10u8).rev() {
            self.copy_clean();
            self.mask_edges();
            self.text(On::Back, 72, 60, a.texts.cont[0].as_bytes(), false);
            self.text(On::Back, 104, 80, a.texts.cont[1].as_bytes(), false);
            self.digits(On::Back, 160, 120, &[d7], false);
            self.swap_screens();
            self.fade_in();
            for _ in 0..50 {
                self.wait_frame();
                if (self.joy1 | self.joy2) & FIRE != 0 {
                    return true;
                }
            }
        }
        self.fade_out();
        false
    }

    /// The values the verification compares with the original's memory.
    pub fn named_state(&self) -> Vec<(String, i64)> {
        let mut v = vec![
            ("frame".to_string(), self.frame as i64),
            ("level".to_string(), self.level as i64),
            ("lives".to_string(), self.lives as i64),
            ("energy".to_string(), self.energy as i64),
            ("energy_due".to_string(), self.energy_due as i64),
            (
                "score".to_string(),
                self.score.iter().fold(0i64, |a, d| a * 10 + *d as i64),
            ),
            ("mode".to_string(), self.mode as i64),
            ("bonus".to_string(), self.bonus as i64),
            ("fade".to_string(), self.fade_level as i64),
            ("alive".to_string(), self.alive as i64),
            ("over".to_string(), self.over_timer as i64),
            ("secret".to_string(), self.secret_timer as i64),
            ("clock".to_string(), u32::from_be_bytes(self.clock) as i64),
            ("rng".to_string(), self.rng[0] as i64),
        ];
        for i in 0..8 {
            let o = &self.objs[i];
            v.push((format!("o{i}k"), o.kind as i64));
            v.push((format!("o{i}x"), o.xi() as i64));
            v.push((format!("o{i}y"), o.yi() as i64));
        }
        v
    }
}

/// cmpm.b over ten characters: is `a` below `b` (signed bytes, the first difference decides)?
fn less(a: &[u8], b: &[u8]) -> bool {
    for (x, y) in a.iter().zip(b) {
        if x != y {
            return (*x as i8) < (*y as i8);
        }
    }
    false
}

fn next_letter(c: u8, d: i32) -> u8 {
    const SET: &[u8] = b" ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.!-";
    let i = SET.iter().position(|&x| x == c).unwrap_or(1) as i32;
    SET[(i + d).rem_euclid(SET.len() as i32) as usize]
}

/// add_digits (+$44B6): adds the bytes of `d0` (lowest first) into six decimal digits from the last.
pub fn add_digits(dg: &mut [u8; 6], mut d0: u32) {
    for i in (0..6).rev() {
        let b = (d0 as u8).wrapping_add(dg[i]);
        d0 = (d0 & !0xFF) | b as u32;
        if (b as i8) >= 10 {
            let w = (d0 as u16).wrapping_sub(0xFF0A);
            d0 = (d0 & 0xFFFF_0000) | w as u32;
        }
        dg[i] = d0 as u8;
        d0 >>= 8;
        if d0 == 0 {
            break;
        }
    }
}
