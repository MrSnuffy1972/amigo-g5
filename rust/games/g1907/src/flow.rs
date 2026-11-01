//! The program's phases ($678): 8 the title and demo, 0 setting up a game, 1 a bout, 3 between bouts, 6 the pause
//! (the fighters show their moves), 4 the match over and the high scores, 5 and 7 the two bonus rounds.

use crate::ik::{bcd_add, bcd_sub1, Bob, Ik, NF};

impl Ik {
    fn p3(&self, i: usize) -> u8 {
        self.a.t("p3")[i]
    }

    /// initialize_input_state ($11B8): the cold start: the high-score table and the default options.
    pub fn initialize_input_state(&mut self) {
        let names = self.a.t("hs_names").to_vec();
        let mut lo = 0u8;
        for i in 0..51 {
            for k in 0..3 {
                self.hs_name[i][k] = names[3 * i + k].wrapping_sub(0x30);
            }
            self.hs_score[i] = [0, lo];
            self.hs_flag[i] = 0;
            lo = bcd_sub1(lo);
        }
        self.hs_score[0] = [1, 0];
        self.sfx_on = 1;
        self.music_on = 1;
        self.human = [0; 3];
        self.speed = 2;
        self.sea = 0;
        self.shadow_col = 0x333;
        self.cheats_used = 0;
        self.trousers = 0;
        self.german = false;
        self.outline = true;
        self.clear_memory_and_initialize_globals();
        self.music.start();
        self.phase = 8;
    }

    /// attract_then_main_loop ($04CA): the title and demo first.
    pub fn attract_then_main_loop(&mut self) {
        crate::attract::clear_memory_and_loop(self);
    }

    /// fault_restart ($04D4): a game starts (from the title, the demo, or after a fault).
    pub fn fault_restart(&mut self) {
        if self.phase == 8 {
            self.title_colours = true;
            self.copy_word_block_to_memory();
        }
        self.phase = 0;
        self.key_count = 0;
        self.vbl = 0;
        self.speed = 2;
        // the original's clearing and set-up take two frames of 68000 time before the start's random draws (the
        // reference's first draws of a game see $608 = 2); the frame interrupt does nothing meanwhile ($618 set)
        self.frame();
        self.frame();
        // and the first flip of the game ends two frames after the set-up: the reference's first full frame
        // interrupt of a game sees $608 = 5 (the step count the original carries in from the title is not modelled)
        self.step_wait = 3;
        self.reset_flag_and_call_helpers();
        self.set_state_flags(15);
    }

    /// The main loop ($0538): runs the phase the program is in, for ever.
    pub fn main_loop_phase_dispatch(&mut self) {
        loop {
            match self.phase {
                1 => self.check_and_set_state(),
                3 => self.swap_and_clear_state(),
                6 => self.swap_and_clear_tables(),
                4 => {
                    crate::hiscore::match_over_set_up(self);
                    self.clear_state_and_loop();
                }
                5 => {
                    crate::bonus::reset_game_state(self);
                    self.bonus_lv[0] = (self.bonus_lv[0] + 1) & 7;
                    self.clear_state_and_loop();
                }
                7 => {
                    crate::bonus::reset_input_state_and_wait_for_key(self);
                    self.bonus_lv[1] = (self.bonus_lv[1] + 1) & 7;
                    self.clear_state_and_loop();
                }
                8 => crate::attract::clear_memory_and_loop(self),
                _ => self.frame(),
            }
        }
    }

    /// reset_flag_and_call_helpers ($71AA).
    pub fn reset_flag_and_call_helpers(&mut self) {
        self.phase = 0;
        self.clear_memory_and_initialize_globals();
        self.phase = 1;
        self.flip();
    }

    /// clear_memory_and_initialize_globals ($71C6): every game variable from $650 on back to zero, the screens and
    /// layers to their start, then the panel drawn.
    pub fn clear_memory_and_initialize_globals(&mut self) {
        self.frame_x = [0; 3];
        self.x = [0; 3];
        self.facing = [0; 3];
        self.stun = [0; 3];
        self.points = [0; 3];
        self.score = [[0; 3]; 3];
        self.round = 0;
        self.timer = 0;
        self.sec_t = 0;
        self.level = 0;
        self.bout_in_round = 0;
        self.bout_pts = [[0; 2]; 3];
        for a in [
            &mut self.turned,
            &mut self.want,
            &mut self.cursor,
            &mut self.next_cursor,
            &mut self.mv,
            &mut self.same,
            &mut self.hold,
            &mut self.rev,
            &mut self.reversible,
            &mut self.hit,
            &mut self.got,
            &mut self.idle,
            &mut self.frozen,
            &mut self.attacking,
            &mut self.flags,
            &mut self.prev_flags,
            &mut self.last_move,
            &mut self.hit_t,
            &mut self.hit_t2,
            &mut self.hit_anim,
            &mut self.extra,
            &mut self.pose_t,
            &mut self.prev_frame,
        ] {
            *a = [0; 3];
        }
        self.front = 0;
        self.result = 0;
        self.key_delay = 0;
        self.flash = [0; 2];
        self.demo = 0;
        self.msg_t = 0;
        self.msg_i = 0;
        self.keys = [0; 5];
        self.rng_n = 0;
        self.pause = false;
        self.sfx_busy = 0;
        self.bonus_kind = 0;
        self.speech = Default::default();
        self.bonus = Default::default();
        self.snd_t = [0; 2];
        self.wait = 0;
        self.flip = 4;
        self.back = 1;
        self.shown = 0;
        (self.lr_f, self.lr_o, self.lw_f, self.lw_o) = (0, 2, 1, 3);
        self.sil_on = false;
        self.obj_on = false;
        self.dirty_clear();
        self.col_c1 = 0xf;
        self.msg_req = 0xff;
        self.clear_state_and_loop();
        self.draw_four_bobs();
        self.call_sequence_routines();
        self.call_three_routines();
        self.choose_background_colours();
    }

    /// check_and_set_state ($184A): one logic step of a bout.
    fn check_and_set_state(&mut self) {
        if !self.check_global_range() {
            self.phase = 3;
            return;
        }
        self.set_up_three_fighters();
        if self.msg_t != 0 {
            self.bout_frame_body();
        } else {
            self.erase_bobs_and_reset_list();
            self.update_three_fighters();
            self.shift_lfsr_state();
            self.flip();
        }
        self.clear_byte_table_and_search();
        self.set_byte_from_table();
        if !self.clear_global_flag() {
            self.phase = 6;
        }
    }

    /// bout_frame_body ($18A2): a step with a code-word message on the screen.
    pub fn bout_frame_body(&mut self) {
        self.dirty_clear_back();
        self.copy_sprite_attribute_blocks_2();
        self.update_three_fighters();
        if self.phase == 3 || self.phase == 4 {
            crate::speech::decode_global_byte_table(self);
        }
        self.outline = false;
        if self.msg_t > 0x14 {
            let c = self.a.t("code_cols")[(self.msg_i & 0x1f) as usize % 20];
            if c != 0xf {
                self.outline = true;
                self.shift_lfsr_state();
                self.parse_byte_script_stream();
            } else {
                self.parse_byte_script_stream();
                self.shift_lfsr_state();
            }
        } else {
            self.shift_lfsr_state();
        }
        self.flip();
    }

    /// parse_byte_script_stream ($191C): the code-word message: text in the 5-row font, $9C = new line, $9B = end,
    /// $9F/$A0 = the players' scores.
    pub fn parse_byte_script_stream(&mut self) {
        let i = (self.msg_i & 0x1f) as usize;
        let col = self.a.t("code_cols")[i % 20];
        let texts = if self.german {
            &self.a.codes_de
        } else {
            &self.a.codes
        };
        let Some(t) = texts.get(i).cloned() else {
            return;
        };
        if t.is_empty() {
            self.msg_t = 0;
            self.msg_i = 0;
            return;
        }
        let mut line = 0x258u16;
        let mut pos = 0u16;
        for &b in t.iter() {
            match b {
                0x9c => {
                    line += 0x28;
                    pos = 0;
                }
                0x9b => break,
                0..=0x14 => pos += b as u16,
                0x9f | 0xa0 => {
                    let s = &self.score[(b - 0x9f) as usize];
                    let v = ((s[0] as u16) << 8) | s[1] as u16;
                    self.belt_text(line + pos, v, col);
                    pos += 8;
                }
                0x20 => pos += 1,
                _ => {
                    self.char5(line + pos, b.wrapping_sub(0x30), col, 1, false);
                    pos += 1;
                }
            }
        }
    }

    /// call_loop_with_decrement / reset_and_run_eight_items ($A2E4): the belt name for a score (8 letters).
    pub fn belt_text(&mut self, pos: u16, score: u16, col: u8) {
        let n = self.compare_d7_to_table(score) as usize;
        let t = self
            .a
            .t(if self.german { "belts_de" } else { "belts" })
            .to_vec();
        for k in 0..8 {
            let c = t.get(n * 8 + k).copied().unwrap_or(0x40);
            self.char5(pos + k as u16, c.wrapping_sub(0x30), col, 1, false);
        }
    }

    /// compare_d7_to_table ($A3BC): how many of the belt thresholds a score has passed (the belt).
    pub fn compare_d7_to_table(&self, v: u16) -> u16 {
        let t = self.a.t("belt_cut");
        let mut n = 0;
        while n < 23 {
            let th = u16::from_be_bytes([t[2 * n], t[2 * n + 1]]);
            if v < th {
                break;
            }
            n += 1;
        }
        n as u16
    }

    /// check_global_range ($5C00): whether the bout goes on (no one has six half points, and time is left).
    fn check_global_range(&self) -> bool {
        if self.timer == 0 && self.sec_t < 0x14 {
            return false;
        }
        self.points.iter().all(|&p| p < 6)
    }

    /// clear_global_flag ($5B5A): false when Space is pressed (the pause), or fire in the pause (its end).
    pub fn clear_global_flag(&mut self) -> bool {
        if self.key_delay != 0 {
            return true;
        }
        let go =
            self.keys[0] == 0x39 || (self.phase == 6 && (self.joy[0] | self.joy[1]) & 0x10 != 0);
        if go {
            self.keys[0] = 0;
            self.key_delay = 0x19;
            return false;
        }
        true
    }

    fn ranks_equal(&self, a: usize, b: usize) -> bool {
        self.points[a] == self.points[b] && self.bout_pts[a] == self.bout_pts[b]
    }

    fn rank_less(&self, a: usize, b: usize) -> bool {
        (self.points[a], self.bout_pts[a][0], self.bout_pts[a][1])
            < (self.points[b], self.bout_pts[b][0], self.bout_pts[b][1])
    }

    /// swap_and_clear_state ($5712): the end of a bout: the fighters ranked, the judge's verdict, the time bonus,
    /// and who goes on.
    fn swap_and_clear_state(&mut self) {
        self.order = [0, 1, 2];
        for k in (0..3).rev() {
            let (a, b) = (self.p3(k) as usize, self.p3(3 + k) as usize);
            let (d1, d2) = (self.order[a] as usize, self.order[b] as usize);
            if self.rank_less(d1, d2) {
                self.order.swap(a, b);
            }
        }
        let o = self.order.map(|v| v as usize);
        self.hum_mask = (self.human[o[0]] << 2) | (self.human[o[1]] << 1) | self.human[o[2]];
        let mut r = 0u8;
        if self.ranks_equal(o[0], o[1]) {
            r = if self.ranks_equal(o[0], o[2]) {
                0x15
            } else {
                0x0e
            };
        } else if self.ranks_equal(o[1], o[2]) {
            r = 7;
        }
        r += self.hum_mask;
        self.result = r;
        let mut d0 = self.p3(6 + r as usize) >> 4;
        if self.hum_mask == 0 {
            let k = (self.random() & 0x1f) as usize;
            d0 = self.p3(0x22 + k);
        }
        self.result_msg = d0;
        for i in 0..NF {
            self.same[i] = 0;
            self.want[i] = 0;
            self.idle[i] = 0;
            if self.hit_t2[i] != 0 {
                self.hit_t2[i] &= 0x3f;
            }
            if self.hit_t[i] != 0 {
                self.hit_t[i] &= 0x3f;
            }
            if self.stun[i] != 0 {
                self.stun[i] = 0x78;
            }
        }
        self.wait = 0x64;
        self.no_wait = (self.delay_snd.1 == 6) as u16;
        loop {
            self.set_up_three_fighters();
            if self.no_wait == 0 {
                self.set_byte_from_table();
            }
            self.call_subroutines_conditionally();
            if self.wait == 0 {
                break;
            }
        }
        self.wait = self.p3(0x42 + self.level as usize);
        self.msg_req = self.result_msg;
        self.loop_with_conditional_calls();
        if self.humans != 0 && self.result != 1 {
            if (self.result_msg == 6 || self.result_msg == 9) && self.round >= 2 {
                let idle =
                    ((self.human[0] & self.human[1]) ^ 1) | self.last_move[0] | self.last_move[1];
                let sc = |s: &Ik, i: usize| ((s.score[i][0] as u16) << 8) | s.score[i][1] as u16;
                if idle == 0 {
                    self.human = [0; 3];
                    self.humans = 0;
                    self.draw_flash_pair_b(0);
                    self.draw_flash_pair_b(1);
                    self.msg_req = 0x23;
                    self.wait = 200;
                    self.loop_with_conditional_calls();
                    self.phase = 8;
                    self.demo = 0;
                    return;
                }
                if self.human[0] != 0 && self.last_move[0] == 0 && sc(self, 0) < 0x50 {
                    self.human[0] = 0;
                    self.draw_flash_pair_b(0);
                    self.msg_req = 0x24;
                    self.wait = 200;
                    self.loop_with_conditional_calls();
                } else if self.human[1] != 0 && self.last_move[1] == 0 && sc(self, 1) < 0x50 {
                    self.human[1] = 0;
                    self.draw_flash_pair_b(1);
                    self.msg_req = 0x25;
                    self.wait = 200;
                    self.loop_with_conditional_calls();
                }
            }
            if self.timer != 0 {
                self.shown_secs = self.timer;
                self.erase_bobs_and_reset_list();
                self.update_three_fighters();
                self.msg_req = 0x10;
                crate::speech::decode_global_byte_table(self);
                self.shift_lfsr_state();
                if self.msg_t != 0 {
                    self.parse_byte_script_stream();
                }
                self.flip();
                self.wait = 0x32;
                self.loop_with_conditional_calls();
                self.points_to_add = 1;
                self.scored = 0;
                self.cur = o[0];
                while self.timer != 0 {
                    self.timer = bcd_sub1(self.timer);
                    self.update_bcd_counters_and_recurse();
                    self.set_global_and_recurse();
                    self.choose_background_colours();
                    self.wait = 0;
                    self.loop_with_conditional_calls();
                    self.scored = 0;
                }
            }
        }
        self.wait = 0x32;
        self.loop_with_conditional_calls();
        let r = self.result as usize;
        let mut ph = 1u8;
        if self.p3(6 + r) & 7 == 0 {
            if self.hum_mask != 0 {
                // the players are out: the match is over
                self.entry = self.order[2];
                self.phase = 4;
                self.msg_req = 0x11;
                self.wait = 200;
                self.loop_with_conditional_calls();
                self.msg_t = 0xf;
                self.msg_i = 0;
                self.wait = 10;
                self.loop_with_conditional_calls();
                return;
            }
            ph = 4;
        }
        self.phase = ph;
        let mut m = self.p3(6 + r) & 7;
        for d2 in (0..3).rev() {
            let d1 = o[d2];
            self.human[d1] &= m;
            m >>= 1;
        }
        self.human[2] = 0;
        self.entry = 0x80;
        self.msg_req = 0xff;
        crate::speech::decode_global_byte_table(self);
        self.msg_t = 0xf;
        self.msg_i = 0;
        self.wait = 0x14;
        self.loop_with_conditional_calls();
        if self.phase != 4 {
            self.clear_state_and_loop();
        }
    }

    /// clear_state_and_loop ($5C3C): set up the next bout (or a bonus round every third).
    pub fn clear_state_and_loop(&mut self) {
        for c in (0..NF).rev() {
            self.cur = c;
            self.bout_pts[c] = [0; 2];
            self.points[c] = 0;
            self.last_move[c] = 0;
            self.half_add = 0;
            self.points_to_add = 0;
            self.scored = 0;
            self.update_global_and_call_routine();
            self.update_bcd_counters_and_recurse();
        }
        self.hit = [0; 3];
        crate::speech::draw_panel_into_both_buffers(self);
        if self.human[0] == 0 {
            self.draw_flash_pair_b(0);
        }
        if self.human[1] == 0 {
            self.draw_flash_pair_b(1);
        }
        if self.human[0] | self.human[1] == 0 {
            self.round = 0;
            self.bout_in_round = 0;
            self.score = [[0; 3]; 3];
            self.level += 1;
            if self.level >= 4 {
                self.level = 2;
            }
        }
        self.sec_t = 0x19;
        self.timer = 0x30;
        if self.round < 0x99 {
            self.round = bcd_add(self.round, 1).0;
        }
        self.load_karate_character_data();
        self.bout_in_round += 1;
        if self.bout_in_round == 3 {
            self.bout_in_round = 0;
            self.phase = if self.bonus_kind != 0 { 7 } else { 5 };
            self.got = [0; 3];
            self.hit = [0; 3];
            self.bonus_kind = (self.bonus_kind & 1) ^ 1;
            self.level += 1;
            if self.level >= 6 {
                self.level = 5;
            }
        } else {
            self.decode_and_clear_table();
            self.four_steps_per_frame();
            self.keys[0] = 0;
            self.first_hit = 0xff;
        }
    }

    /// decode_and_clear_table ($5DB8): the three fighters to their start places, bowing.
    fn decode_and_clear_table(&mut self) {
        self.humans = self.human.iter().map(|h| (*h != 0) as u8).sum();
        self.flash = [0x1e, 0x1e];
        self.front = 0;
        let t = self.a.t("p_start").to_vec();
        for i in (0..NF).rev() {
            self.cur = i;
            self.frame_x[i] = 0;
            self.mv[i] = 0;
            self.same[i] = 0;
            self.hit[i] = 0;
            self.hit_t[i] = 0;
            self.hit_t2[i] = 0;
            self.prev_flags[i] = 0;
            self.flags[i] = 0;
            self.frozen[i] = 0;
            self.extra[i] = 0;
            self.got[i] = 0;
            self.facing[i] = if self.humans >= 2 { t[6 + i] } else { t[3 + i] };
            self.x[i] = t[i] + (self.random() & 3);
            self.want[i] = 0x1c;
            self.reset_animation_state(i);
            self.want[i] = 0;
        }
        self.msg_req = 0xff;
        crate::speech::decode_global_byte_table(self);
    }

    /// load_karate_character_data ($6904): the computer fighters' skill for this bout.
    fn load_karate_character_data(&mut self) {
        let t = self.a.t("char").to_vec();
        let d2 = (self.round & 7) as usize;
        self.skill = [self.level, self.level, t[16 + d2]];
        if t[d2] != 0 {
            self.skill[0] = t[d2];
        }
        if t[8 + d2] != 0 {
            self.skill[1] = t[8 + d2];
        }
    }

    /// swap_and_clear_tables ($5542): the pause: the three fighters show their moves until Space or fire.
    fn swap_and_clear_tables(&mut self) {
        self.draw_into_both_buffers();
        self.toggle_94a = 0;
        let p6 = self.a.t("p6").to_vec();
        for d2 in (0..NF).rev() {
            self.saved_xf[d2] = self.x[d2];
            self.x[d2] = p6[d2];
            self.saved_xf[3 + d2] = self.facing[d2];
            self.frame_x[d2] = 0;
            self.want[d2] = p6[3];
            self.initialize_sprite_structure(d2, 0);
        }
        self.pose_t = [0; 3];
        self.facing = [0; 3];
        if self.random() & 1 == 0 {
            self.cur = 0;
            self.process_player_state_flags();
            self.process_player_state_flags();
            self.cur = 1;
            self.process_player_state_flags();
        }
        loop {
            for d2 in (0..NF).rev() {
                if self.flags[d2] & 0x10 != 0 {
                    let k = self.pose_t[d2] as usize;
                    let m = p6[3 + k];
                    self.want[d2] = m;
                    self.cur = d2;
                    self.initialize_sprite_structure(d2, m);
                    self.want[d2] = 0;
                    self.pose_t[d2] += 1;
                    if self.pose_t[d2] >= 0xd {
                        self.pose_t[d2] = 0;
                    }
                }
            }
            for c in (0..NF).rev() {
                self.cur = c;
                self.process_player_state_flags();
            }
            self.toggle_global_state_sequence();
            self.flip();
            if self.toggle_94a != 0 {
                self.set_byte_from_table();
            }
            if !self.clear_global_flag() {
                break;
            }
        }
        self.draw_into_both_buffers();
        for d2 in (0..NF).rev() {
            self.initialize_sprite_structure(d2, 0);
            self.want[d2] = 0;
            self.x[d2] = self.saved_xf[d2];
            self.facing[d2] = self.saved_xf[3 + d2];
        }
        self.phase = 1;
    }

    /// toggle_global_state_sequence ($5EC8): each fighter drawn twice, side by side (the pause's show).
    fn toggle_global_state_sequence(&mut self) {
        self.erase_bobs_and_reset_list();
        for (c, dx) in [(2usize, 0x68u8), (1, 0x44), (0, 0x1e)] {
            self.cur = c;
            self.dispatch_via_indexed_table();
            self.x[c] = self.x[c].wrapping_add(dx);
            self.dispatch_via_indexed_table();
            self.x[c] = self.x[c].wrapping_sub(dx);
        }
        self.shift_lfsr_state();
    }

    /// loop_with_conditional_calls ($5F6C): steps with the fighters standing until the wait has run out.
    pub fn loop_with_conditional_calls(&mut self) {
        loop {
            if self.msg_t != 0 {
                self.bout_frame_body();
            } else {
                self.erase_bobs_and_reset_list();
                self.update_three_fighters();
                crate::speech::decode_global_byte_table(self);
                self.shift_lfsr_state();
                self.flip();
            }
            if self.wait == 0 {
                return;
            }
        }
    }

    /// call_subroutines_conditionally ($5F48).
    fn call_subroutines_conditionally(&mut self) {
        self.erase_bobs_and_reset_list();
        self.update_three_fighters();
        self.shift_lfsr_state();
        if self.msg_t != 0 {
            self.parse_byte_script_stream();
        }
        self.flip();
    }

    /// set_byte_from_table ($0DF6) and lookup_byte_from_table ($0EE6): sounds when a fighter's frame changes (the
    /// swish of an attack, the thud of a hit, a cry a little later).
    pub fn set_byte_from_table(&mut self) {
        if self.sfx_on == 0 {
            return;
        }
        let t = self.a.t("snd").to_vec();
        let mut want: i16 = -1;
        for i in (0..NF).rev() {
            let f = self.frame_x[i];
            let p = self.prev_frame[i];
            self.prev_frame[i] = f;
            if f == p {
                continue;
            }
            let Some(d2) = (0..17).find(|&k| t[k] == f) else {
                continue;
            };
            let d0: u8;
            if self.got[i] != 0 {
                d0 = t[0x44 + d2];
                let d3 = t[0x55 + d2];
                if (d3 as i8) > 0 {
                    self.delay_snd = (d3, t[0x66 + d2]);
                }
            } else if self.hit[i] != 0 {
                d0 = t[0x77 + d2];
            } else {
                if self.flags[i] & 0x4d == 0 {
                    continue;
                }
                d0 = t[0x11 + d2];
                let d3 = t[0x22 + d2];
                if (d3 as i8) > 0 {
                    self.delay_snd = (d3, t[0x33 + d2]);
                }
            }
            if (d0 as i8) >= 0 {
                want = d0 as i16;
            }
        }
        if want >= 0 {
            self.set_state_flags(want as u8);
        }
    }

    // ---- the panel

    /// call_three_routines ($64D8): the round number, the clock's label and the clock.
    pub fn call_three_routines(&mut self) {
        self.char8(0x1c, 0x1c, 0xc);
        self.char8(0x1d, 0x26, 0xc);
        self.char8(0x44, self.round >> 4, 0xc);
        self.char8(0x45, self.round & 15, 0xc);
        self.update_timer_state();
        self.set_global_and_recurse();
    }

    /// update_timer_state ($6542): "TIME=", "DEMO " or a blinking "PAUSE".
    fn update_timer_state(&mut self) {
        let t = self.a.t("timer_text").to_vec();
        let (text, col): (Vec<u8>, u8) = if self.phase == 6 {
            (t[4..9].to_vec(), t[2 + ((self.vbl >> 4) & 1) as usize])
        } else if self.human[0] | self.human[1] == 0 {
            (t[9..14].to_vec(), 3)
        } else if self.german {
            (self.a.t("timer_de")[..5].to_vec(), 6)
        } else {
            (t[14..19].to_vec(), 6)
        };
        for (k, c) in text.iter().enumerate() {
            self.char8(0x20 + k as u16, c.wrapping_sub(0x30), col);
        }
    }

    /// set_global_and_recurse ($65BA): the clock's two digits (blinking in the last five seconds).
    pub fn set_global_and_recurse(&mut self) {
        let mut col = 6;
        if self.phase == 1 && self.timer < 6 {
            col = [1, 7][((self.steps >> 2) & 1) as usize];
        }
        self.char8(0x26, (self.timer >> 4) & 15, col);
        self.char8(0x27, self.timer & 15, col);
    }

    /// call_sequence_routines ($6622): the speed meter (top right).
    pub fn call_sequence_routines(&mut self) {
        let t = self.a.t("speed_bobs").to_vec();
        let d0 = (self.speed.wrapping_sub(self.ntsc) as usize).min(4);
        let a = u32::from_be_bytes([t[4 * d0], t[4 * d0 + 1], t[4 * d0 + 2], t[4 * d0 + 3]]);
        self.four_bobs(Bob::at(a), 0xa0);
    }

    /// draw_four_bobs ($6642): the strip at the top (16 by 5, twice, into both screens).
    pub fn draw_four_bobs(&mut self) {
        self.four_bobs(Bob::at(0x22B7C + 0x600), 0x90);
    }

    fn four_bobs(&mut self, s: Bob, x: i32) {
        for b in [1, 0] {
            self.bob2(b, s, x, 10, 5, false);
        }
        for b in [0, 1] {
            self.bob2(b, s.right(1), x + 8, 10, 5, false);
        }
    }

    /// decrement_flag_table_bytes ($668C): the player markers flash after a bout starts.
    fn decrement_flag_table_bytes_body(&mut self) {
        for d2 in [1usize, 0] {
            if self.human[d2] != 0 && self.flash[d2] != 0 {
                self.flash[d2] -= 1;
                if self.flash[d2] & 4 == 0 {
                    self.draw_flash_pair_a(d2);
                } else {
                    self.draw_flash_pair_b(d2);
                }
            }
        }
    }
    pub fn decrement_flag_table_bytes(&mut self) {
        self.decrement_flag_table_bytes_body();
    }

    fn flash_bob(&mut self, d2: usize, o: usize) {
        let t = self.a.t("flash").to_vec();
        let l = |k: usize| u32::from_be_bytes([t[k], t[k + 1], t[k + 2], t[k + 3]]);
        let x = l(4 * d2) as i32;
        let s = Bob::at(l(o + 4 * d2));
        for b in [1, 0] {
            self.bob2(b, s, x, 0, 15, false);
        }
    }
    /// draw_flash_pair_a ($66E0).
    pub fn draw_flash_pair_a(&mut self, d2: usize) {
        self.flash_bob(d2, 8);
    }
    /// draw_flash_pair_b ($66EE).
    pub fn draw_flash_pair_b(&mut self, d2: usize) {
        self.flash_bob(d2, 16);
    }

    /// update_global_and_call_routine ($76CC): adds the hit's half points to fighter `cur` and lights its markers.
    pub fn update_global_and_call_routine(&mut self) {
        let c = self.cur;
        let lc = self.a.t("level_cols").to_vec();
        let (on, off) = (lc[c], lc[3 + c]);
        self.points[c] = (self.points[c] + self.half_add).min(7);
        let n = self.points[c];
        let cell = lc[0x72 + c] as u16;
        for k in 0..6u8 {
            let col = if k < n { on } else { off };
            self.char8(cell + k as u16, 0xc, col);
        }
    }

    /// update_bcd_counters_and_recurse ($7744): adds the hit's points to fighter `cur`'s bout and match score and
    /// shows the score (once per hit).
    pub fn update_bcd_counters_and_recurse(&mut self) {
        if self.scored != 0 {
            return;
        }
        self.scored = self.points_to_add;
        let c = self.cur;
        let p = self.points_to_add;
        let (lo, cy) = bcd_add(self.bout_pts[c][1], p);
        self.bout_pts[c][1] = lo;
        self.bout_pts[c][0] = bcd_add(self.bout_pts[c][0], cy as u8).0;
        let lc = self.a.t("level_cols").to_vec();
        let slot = (lc[0x78 + c] / 3) as usize;
        let s = &mut self.score[slot.min(2)];
        let (lo, cy) = bcd_add(s[1], p);
        s[1] = lo;
        s[0] = bcd_add(s[0], cy as u8).0;
        let mut cell = lc[0x75 + c] as u16;
        let col = lc[0x7b + c];
        let s = self.score[slot.min(2)];
        let mut lit = false;
        let mut left = 6;
        for b in s.iter() {
            for nib in [b >> 4, b & 15] {
                left -= 1;
                if nib != 0 {
                    lit = true;
                }
                let g = if lit || left == 0 { nib } else { 0xa };
                self.char8(cell, g, col);
                cell += 1;
            }
        }
    }

    /// save_and_restore_global_variable ($7818): the score shown again (forced).
    pub fn save_and_restore_global_variable(&mut self) {
        let (p, s) = (self.points_to_add, self.scored);
        self.scored = 0;
        self.update_bcd_counters_and_recurse();
        self.scored = s;
        self.points_to_add = p;
    }

    /// choose_background_colours ($A338): the belt colours of the players' fighters, by their scores.
    pub fn choose_background_colours(&mut self) {
        self.col_c9 = 0xcc0;
        if self.phase == 6 {
            self.col_p1 = 0xc00;
            self.col_p2 = 0xe6e;
            return;
        }
        let bc = self.a.t("belt_cols").to_vec();
        let sc = |s: &Ik, i: usize| ((s.score[i][0] as u16) << 8) | s.score[i][1] as u16;
        self.col_p1 = if self.human[0] == 0 {
            0xc00
        } else {
            let n = 2 * self.compare_d7_to_table(sc(self, 0)) as usize;
            u16::from_be_bytes([bc[n], bc[n + 1]])
        };
        self.col_p2 = if self.human[1] == 0 {
            0xe6e
        } else {
            let n = 0x30 + 2 * self.compare_d7_to_table(sc(self, 1)) as usize;
            u16::from_be_bytes([bc[n], bc[n + 1]])
        };
    }

    /// advance_scrolling_object ($9ED0): the "top scores" sign's lights running round (in the high-score show).
    pub fn advance_scrolling_object(&mut self) {
        if self.sign.2 == 0 {
            return;
        }
        crate::hiscore::sign_lights(self);
    }

    pub fn dirty_clear(&mut self) {
        self.dirty = Default::default();
    }
    pub fn dirty_clear_back(&mut self) {
        let b = self.back;
        self.dirty[b].clear();
    }
}
