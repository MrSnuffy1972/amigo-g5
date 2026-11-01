//! The match over: the high-score table (50 places, sorted), entering a name with the joystick, and the "top 50
//! scores" show with its running lights.

use crate::ik::{bcd_add, Bob, Ik, NF};

/// match_over_set_up ($9B10).
pub fn match_over_set_up(g: &mut Ik) {
    if g.entry >= 0x80 {
        self_modifying_loop(g);
        initialize_game_state(g);
        return finish(g);
    }
    g.flash = [0x1e, 0x1e];
    for i in 0..50 {
        g.hs_flag[i] = 0;
    }
    let slot = if g.order[2] != 0 { 1 } else { 0 };
    g.hs_flag[50] = 1;
    g.hs_score[50] = [g.score[slot][0], g.score[slot][1]];
    let qualifies = if g.hs_score[50] == [0, 0] {
        None
    } else {
        g.hs_name[50] = [0xc; 3];
        if g.hs_score[49] < g.hs_score[50] {
            self_modifying_loop(g);
            (0..50).find(|&i| g.hs_flag[i] != 0)
        } else {
            g.msg_req = 0x13;
            g.wait = 200;
            g.loop_with_conditional_calls();
            None
        }
    };
    let Some(rank) = qualifies else {
        g.human[0] = 0;
        g.human[1] = 0;
        g.draw_flash_pair_b(1);
        g.draw_flash_pair_b(0);
        g.msg_req = 0x14;
        g.wait = 150;
        g.loop_with_conditional_calls();
        self_modifying_loop(g);
        initialize_game_state(g);
        return finish(g);
    };
    g.rank = rank as u8;
    let (mut d0, mut d1) = (rank as u8, 0u8);
    if rank >= 6 {
        d0 = 0;
        d1 = rank as u8;
        if rank >= 0x2d {
            d0 = rank as u8 - 0x2d;
            d1 = 0x2d;
        }
    }
    g.entry_pos = (d0, d1);
    g.msg_req = 0x12;
    g.wait = 200;
    g.loop_with_conditional_calls();
    let e = g.order[2] as usize;
    g.facing[e] = 0;
    g.x[e] = 0x14;
    g.frame_x[e] = 0x30;
    for k in [0usize, 1] {
        let o = g.order[k] as usize;
        g.frame_x[o] = 0x57;
    }
    g.sign = (g.sign.0, 0, 1);
    reset_global_flag_and_call(g);
    initialize_display_state(g);
    g.frame_x[e] = 0x31;
    reset_low_nibble_flag(g);
    g.wait_frames(0x28);
    g.frame_x[e] = 0x30;
    reset_low_nibble_flag(g);
    g.wait_frames(0x64);
    g.human[e.min(2)] = 0;
    if e < 2 {
        g.draw_flash_pair_b(e);
    }
    while g.vbl_hi < 2 {
        g.frame();
    }
    g.demo = 0;
    g.sign.2 = 0;
    g.draw_into_both_buffers();
    g.copy_sprite_attribute_blocks();
    finish(g);
}

/// l0_9D04: back to the title.
fn finish(g: &mut Ik) {
    g.phase = 8;
    g.human = [0; 3];
    g.demo = 0;
    g.draw_flash_pair_b(0);
    g.draw_flash_pair_b(1);
}

fn hs_less(g: &Ik, a: usize, b: usize) -> bool {
    g.hs_score[a] < g.hs_score[b]
}

/// self_modifying_loop ($A716): sorts the 51 rows, best first (a bubble sort, as the original).
pub fn self_modifying_loop(g: &mut Ik) {
    let mut n = 50;
    while n > 0 {
        for k in 0..n {
            if hs_less(g, k, k + 1) {
                g.hs_score.swap(k, k + 1);
                g.hs_name.swap(k, k + 1);
                g.hs_flag.swap(k, k + 1);
            }
        }
        n -= 1;
    }
}

/// initialize_game_state ($9D2C): the high-score show: the white fighter holds the sign while the table scrolls
/// (the joystick's up and down keys scroll it by hand).
pub fn initialize_game_state(g: &mut Ik) {
    for d2 in (0..NF).rev() {
        g.want[d2] = 0;
        g.hit[d2] = 0;
        g.frozen[d2] = 0;
        g.initialize_sprite_structure(d2, 0);
        g.frame_x[d2] = 0x57;
    }
    g.demo_first = g.random() & 1;
    g.x[0] = 0x12;
    g.frame_x[0] = 0x3f;
    g.facing[0] = 0;
    g.step_wait = 0xa;
    g.entry_pos.1 = 0x31;
    g.points = [0, 1, 1];
    g.skill = [2, 5, 6];
    reset_global_flag_and_call(g);
    g.step_wait = 6;
    let mut manual = false;
    loop {
        g.set_up_three_fighters_alt();
        self_calling_state_toggle(g);
        if !manual {
            if g.entry_pos.1 == 0 || g.keys[0] == 0x48 || g.keys[0] == 0x50 {
                manual = true;
                if g.vbl_hi >= 5 {
                    break;
                }
                continue;
            }
            g.entry_pos.1 -= 1;
            if g.vbl_hi >= 4 {
                break;
            }
        } else {
            if g.keys[0] == 0x48 && g.entry_pos.1 != 0 {
                g.entry_pos.1 -= 1;
            }
            if g.keys[0] == 0x50 && g.entry_pos.1 < 0x31 {
                g.entry_pos.1 += 1;
            }
            if g.vbl_hi >= 5 {
                break;
            }
        }
    }
    g.demo = 0;
    g.draw_into_both_buffers();
    g.copy_sprite_attribute_blocks();
}

/// reset_global_flag_and_call ($9E40) then reset_low_nibble_flag.
fn reset_global_flag_and_call(g: &mut Ik) {
    g.msg_req = 0xff;
    crate::speech::decode_global_byte_table(g);
    g.draw_into_both_buffers();
    reset_low_nibble_flag(g);
}

/// reset_low_nibble_flag ($9E50).
fn reset_low_nibble_flag(g: &mut Ik) {
    g.demo = 1;
    g.vbl_hi = 0;
    g.vbl &= 0xf;
    self_calling_state_toggle(g);
}

/// self_calling_state_toggle ($9E68): one step of the show: the fighters and the table.
fn self_calling_state_toggle(g: &mut Ik) {
    g.dirty_clear_back();
    g.copy_sprite_attribute_blocks_2();
    g.cur = 1;
    g.dispatch_via_indexed_table();
    g.cur = 0;
    g.dispatch_via_indexed_table();
    if g.demo_first == 0 {
        g.cur = 2;
        g.dispatch_via_indexed_table();
        g.shift_lfsr_state();
        initialize_bcd_counter(g);
    } else {
        initialize_bcd_counter(g);
        g.cur = 2;
        g.dispatch_via_indexed_table();
        g.shift_lfsr_state();
    }
    g.flip();
}

/// print_bcd_byte ($A6BA): two digits, leading zeros as `blank` until a digit has been printed.
fn print_bcd(g: &mut Ik, line: u16, d2: &mut u16, v: u8, blank: &mut u8, col: u8) {
    for nib in [v >> 4, v & 15] {
        let c = if nib == 0 { *blank } else { nib };
        if nib != 0 {
            *blank = 0;
        }
        g.char5(line + *d2, c, col, 0, false);
        *d2 += 1;
    }
}

/// initialize_bcd_counter ($A594): six rows of the table: place, name, belt, score.
fn initialize_bcd_counter(g: &mut Ik) {
    g.outline = true;
    let head =
        g.a.t(if g.german { "hs_head_de" } else { "hs_head" })
            .to_vec();
    for (k, &c) in head.iter().enumerate().take(0x1b) {
        if c != 0x40 {
            g.char5(0x28a + k as u16, c.wrapping_sub(0x30), 4, 0, false);
        }
    }
    let mut line = 0x2b1u16;
    let mut idx = g.entry_pos.1;
    let mut place = 0u8;
    for _ in 0..idx {
        place = bcd_add(place, 1).0;
    }
    for _row in 0..6 {
        let p1 = bcd_add(place, 1).0;
        let mut d2 = 1u16;
        let mut blank = 0x10u8;
        print_bcd(g, line, &mut d2, p1, &mut blank, 9);
        let i = (idx as usize).min(50);
        for k in 0..3 {
            g.char5(line + 7 + k as u16, g.hs_name[i][k], 9, 0, false);
        }
        let s = g.hs_score[i];
        let v = ((s[0] as u16) << 8) | s[1] as u16;
        g.belt_text_row(line + 0xc, v, 9);
        let mut d2 = 0x15u16;
        let mut blank = 0x10u8;
        print_bcd(g, line, &mut d2, s[0], &mut blank, 9);
        print_bcd(g, line, &mut d2, s[1], &mut blank, 9);
        print_bcd(g, line, &mut d2, 0, &mut blank, 9);
        d2 -= 1;
        g.char5(line + d2, 0, 9, 0, false);
        line += 0x28;
        idx += 1;
        place = p1;
        if place >= 0x50 {
            break;
        }
    }
}

impl Ik {
    /// reset_and_run_eight_items ($A2EC): the belt name in the table (no row offset).
    pub fn belt_text_row(&mut self, pos: u16, score: u16, col: u8) {
        let n = self.compare_d7_to_table(score) as usize;
        let t = self
            .a
            .t(if self.german { "belts_de" } else { "belts" })
            .to_vec();
        for k in 0..8 {
            let c = t.get(n * 8 + k).copied().unwrap_or(0x40);
            self.char5(pos + k as u16, c.wrapping_sub(0x30), col, 0, false);
        }
    }
}

/// initialize_display_state ($A0EA): entering a name: left and right pick the letter, fire takes it, three letters.
fn initialize_display_state(g: &mut Ik) {
    g.vbl_hi = 0;
    let mut tick = 0u8; // $85c
    let mut k = 0usize; // $861
    let mut name = [0xcu8; 3];
    let sx = g.a.t("sign_x").to_vec();
    let p = (g.entry_pos.0 as usize).min(5);
    let line = u16::from_be_bytes([sx[2 * p], sx[2 * p + 1]]);
    let e = (g.entry as usize).min(1);
    let read = |g: &Ik| (g.joy[e] & 0xf, ((g.joy[e] >> 4) & 1) ^ 1);
    let (_, mut prev) = read(g);
    let mut letter = 0x11u8;
    loop {
        if g.vbl_hi >= 0xa {
            break;
        }
        let (dir, up) = read(g);
        let mut take = false;
        if up != 0 {
            prev = 1;
            if tick >= 5 {
                tick = 0;
                if dir & 4 == 0 {
                    g.char5(line + k as u16, letter, 0xf, 0, true);
                    letter = letter.wrapping_sub(1);
                    if letter < 0x10 {
                        letter = 0x2a;
                    }
                } else if dir & 8 == 0 {
                    g.char5(line + k as u16, letter, 0xf, 0, true);
                    letter += 1;
                    if letter >= 0x2b {
                        letter = 0x10;
                    }
                }
            }
        } else if prev != 0 {
            take = true;
        }
        if take {
            g.char5(line + k as u16, letter, 4, 0, true);
            prev = 0;
            name[k] = letter;
            letter = 0x11;
            tick = 0;
            g.vbl &= 7;
            if k >= 2 {
                break;
            }
            k += 1;
            g.vbl_hi &= 3;
        }
        let col = if g.vbl & 0x1f >= 0x18 { 8 } else { 4 };
        g.char5(line + k as u16, letter, col, 0, true);
        g.wait_frames(1);
        tick = tick.wrapping_add(1);
    }
    g.char5(line + k as u16, letter, 9, 0, true);
    let r = (g.rank as usize).min(50);
    g.hs_name[r] = name;
}

/// advance_scrolling_object ($9ED0) and self_14: the twelve lights running round the "top 50 scores" sign.
pub fn sign_lights(g: &mut Ik) {
    let b = g.shown;
    lights(g, b, false);
    g.sign.1 = (g.sign.1 + 2) & 0x3f;
    if g.sign.1 == 0 {
        g.sign.0 = g.sign.0.wrapping_sub(1);
    }
    g.sign.0 &= 7;
    lights(g, b, true);
}

fn lights(g: &mut Ik, b: usize, draw: bool) {
    let s = g.sign.1 as i32;
    let mut pts = Vec::new();
    for k in 0..4 {
        pts.push((0x50 + s + 0x40 * k, 0x78));
    }
    for k in 0..2 {
        pts.push((0x150, 0x78 + s / 2 + 0x20 * k));
    }
    for k in 0..4 {
        pts.push((0x150 - s - 0x40 * k, 0xb8));
    }
    for k in 0..2 {
        pts.push((0x50, 0xb8 - s / 2 - 0x20 * k));
    }
    for (n, (x, y)) in pts.into_iter().enumerate() {
        let src = Bob::at(0x2249C + 0x600 + 2 * ((g.sign.0 as u32 + n as u32) & 7));
        if draw {
            g.bob(b, src, (x & 0x1ff) - 32, y, 3, false);
        } else {
            for r in 0..3 {
                for c in 0..16 {
                    let (xx, yy) = ((x & 0x1ff) - 32 + c, y + r);
                    if (0..320).contains(&xx) && (0..200).contains(&yy) {
                        let o = yy as usize * 320 + xx as usize;
                        g.scr[b][o] = g.a.background.px[o];
                    }
                }
            }
        }
    }
}
