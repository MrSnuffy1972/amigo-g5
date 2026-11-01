//! The judge and his speech bubble: where the bubble goes (in the widest gap between the fighters), its frame and
//! tail, the text with the fighters' names, the judge talking, and the sparkles round him.

use crate::ik::{Bob, Ik, NF};

/// The speech state ($872 to $888, the gap map at $904, the sparkles at $8a1).
#[derive(Default, Clone)]
pub struct Speech {
    pub width: u8,      // $872 bubble width in cells
    pub speaking: bool, // $873
    pub judge: u8,      // $874 the judge's cell
    pub left: u8,       // $875 the bubble's first cell
    pub cl: [u8; 3],    // $876 each fighter's first cell
    pub cr: [u8; 3],    // $879 each fighter's last cell
    pub cw: [u8; 3],    // $87c each fighter's width in cells
    pub gap_at: u8,     // $87f
    pub gap: u8,        // $880
    pub ch: u8,         // $881
    pub name_ch: u8,    // $882
    pub big: bool,      // $884
    pub line: u8,       // $885
    pub cursor: u8,     // $886
    pub saved_line: u8, // $888
    pub colour: u8,     // $92c
    pub map: Vec<u8>,   // $904 (40 cells)
    pub sp_t: [u8; 8],  // $8a1
    pub sp_x: [u8; 8],  // $8a9
    pub sp_y: [u8; 8],  // $8b1
    pub sparkles: u8,   // $8ea
    pub vbl_count: u8,
}

fn msgs(g: &Ik, o: usize) -> u8 {
    g.a.t("msgs")[o]
}
const M_COUNT: usize = 0x00; // d0_7D24
const M_START: usize = 0x03; // d0_7D27
const M_NAMES: usize = 0x06; // d0_7D2A
const M_NCOL: usize = 0x15; // d0_7D39
const M_LINES: usize = 0x18; // d0_7D3C
const M_WIDTH: usize = 0x5D9; // d0_82FD
const M_COL: usize = 0x6AA; // d0_83CE
const M_TALK: usize = 0x6DA; // d0_83FE

/// decode_global_byte_table ($7832): shows the judge's message `msg_req` ($ff = none).
pub fn decode_global_byte_table(g: &mut Ik) {
    let d0 = g.msg_req;
    if d0 >= 0x80 {
        if g.msg_cur != d0 {
            g.msg_cur = d0;
            draw_panel_into_both_buffers(g);
        }
        g.speech.speaking = false;
        return;
    }
    if d0 != g.msg_cur {
        g.msg_cur = d0;
        if d0 >= 0x28 {
            draw_panel_into_both_buffers(g);
            g.speech.speaking = false;
            return;
        }
        let w = if g.german {
            g.a.t("msg_de")[d0 as usize]
        } else {
            msgs(g, M_WIDTH + d0 as usize)
        };
        g.speech.width = w.wrapping_add(2);
        if g.speech.speaking {
            normalize_global_byte(g);
        } else {
            initialize_player_state_tables(g);
            g.speech.sparkles = 0x28;
        }
        draw_panel_into_both_buffers(g);
        g.speech.speaking = true;
    }
    g.speech.colour = 4;
    self_5(g);
    decode_script_bytes(g);
    g.speech.speaking = true;
    restore_global_variable_and_call(g);
}

/// draw_panel_into_both_buffers ($7CFA): clears the speech rows on both screens and the judge's spot.
pub fn draw_panel_into_both_buffers(g: &mut Ik) {
    let keep = g.back;
    for b in [1, 0] {
        g.back = b;
        g.copy_sprite_attribute_blocks();
    }
    g.back = keep;
    // draw_masked_pair ($78E4): the background back under the judge's speech tail
    let x = g.speech.judge as i32 * 4 + 0x13;
    let s = Bob::at(0x2287E + 0x600);
    for b in [g.shown, g.back] {
        restore_through(g, b, s, 2 * x - 32, 0x8a, 7);
    }
}

/// self_17: the background back wherever the source picture has a pixel.
fn restore_through(g: &mut Ik, b: usize, s: Bob, sx: i32, y: i32, h: i32) {
    for r in 0..h {
        for c in 0..16 {
            let x = sx + c;
            let yy = y + r;
            if !(0..320).contains(&x) || !(0..200).contains(&yy) {
                continue;
            }
            if g.pic(s.src).at((s.x + c) as usize, (s.y + r) as usize) != 0 {
                let o = yy as usize * 320 + x as usize;
                g.scr[b][o] = g.a.background.px[o];
            }
        }
    }
}

/// initialize_player_state_tables ($7944): finds the widest gap between the fighters for the bubble.
fn initialize_player_state_tables(g: &mut Ik) {
    if g.phase == 6 {
        g.speech.judge = 0x22;
        normalize_global_byte(g);
        return;
    }
    let s = &mut g.speech;
    s.map = vec![0; 40];
    for d2 in (0..6).rev() {
        s.sp_t[d2] = (d2 & 3) as u8;
    }
    let widths = g.a.t("anim").to_vec();
    for d2 in (0..NF).rev() {
        let l = (g.x[d2] >> 2).saturating_sub(4);
        let w = (widths[g.frame_x[d2] as usize] >> 2) + 1;
        let s = &mut g.speech;
        s.cl[d2] = l;
        s.cw[d2] = w;
        if g.facing[d2] != 0 {
            s.cr[d2] = l + 0xa;
            s.cl[d2] = s.cr[d2].wrapping_sub(w);
        } else {
            s.cr[d2] = l + w;
        }
    }
    let s = &mut g.speech;
    s.cw[2] = 0xa;
    if s.cr[2] >= 0x1d {
        s.cr[2] = 0x27;
    }
    if s.cl[2] < 9 {
        s.cl[2] = 0;
    }
    for d2 in 0..3 {
        let mut d1 = s.cl[d2];
        loop {
            if d1 < 0x28 {
                s.map[d1 as usize] = d2 as u8 + 1;
            }
            d1 = d1.wrapping_add(1);
            if d1 >= s.cr[d2] {
                break;
            }
        }
    }
    let (mut run, mut best, mut at) = (0u8, 0u8, 0u8);
    for d1 in 0..=0x28u8 {
        if d1 == 0x28 || s.map[d1 as usize] != 0 {
            if run > best {
                best = run;
                at = d1 - run;
            }
            run = 0;
        } else {
            run += 1;
        }
    }
    s.gap = best;
    s.gap_at = at;
    if s.gap < 6 {
        let mut guard = 0;
        loop {
            guard += 1;
            if guard > 200 {
                break;
            }
            let d2 = s.gap_at as i32 - 1;
            if d2 >= 0 {
                if s.map[d2 as usize] >= 3 {
                    s.gap_at += 1;
                    continue;
                }
                let e = (s.gap + s.gap_at) as usize;
                if s.map[e.min(39)] >= 3 {
                    s.gap_at = s.gap_at.wrapping_sub(1);
                }
            }
            if s.gap_at >= 0x22 {
                s.gap_at -= 1;
                continue;
            }
            break;
        }
        s.gap = 0;
    }
    let mut d0 = s.gap;
    if d0 != 0 {
        d0 = d0.wrapping_sub(6) >> 1;
    }
    s.judge = d0.wrapping_add(s.gap_at);
    normalize_global_byte(g);
}

/// normalize_global_byte ($7B48): the bubble's first cell from the judge's cell and the bubble's width.
fn normalize_global_byte(g: &mut Ik) {
    let s = &mut g.speech;
    let d0 = s.judge;
    s.left = if d0 < 0xa {
        d0
    } else if d0 >= 0x1b {
        d0.wrapping_sub(s.width).wrapping_add(4)
    } else {
        d0.wrapping_add(3).wrapping_sub(s.width >> 1)
    };
}

/// apply_byte_mask_to_table ($8772): sets the pixels of `mask` in an 8-pixel cell to the speech colour, `rows` rows.
fn mask_rows(g: &mut Ik, cell: u8, row: usize, mask: u8, rows: usize, col: u8) {
    let b = g.back;
    let x0 = (cell & 0x3f) as usize * 8;
    for r in 0..rows {
        let y = row + r;
        if y >= 200 {
            break;
        }
        for k in 0..8 {
            if mask & (0x80 >> k) != 0 && x0 + k < 320 {
                g.scr[b][y * 320 + x0 + k] = col & 15;
            }
        }
    }
}

/// self_5 ($86A8): the bubble (colour 4, rows 117 to 137) and its tail towards the judge.
fn self_5(g: &mut Ik) {
    let w = g.speech.width.wrapping_sub(2);
    if w == 0 || w >= 0x80 {
        return;
    }
    let c = g.speech.colour;
    let mut cell = g.speech.left;
    mask_rows(g, cell, 117, 0x01, 1, c);
    mask_rows(g, cell, 118, 0x03, 19, c);
    mask_rows(g, cell, 137, 0x01, 1, c);
    for _ in 0..w {
        cell = cell.wrapping_add(1);
        mask_rows(g, cell, 117, 0xff, 21, c);
    }
    cell = cell.wrapping_add(1);
    mask_rows(g, cell, 117, 0x80, 1, c);
    mask_rows(g, cell, 118, 0xc0, 19, c);
    mask_rows(g, cell, 137, 0x80, 1, c);
    // draw_indicator_bob ($7918): the tail
    let x = g.speech.judge as i32 * 4 + 0x13;
    let b = g.back;
    g.bob2(b, Bob::at(0x2287E + 0x600), x, 0x8a, 7, true);
}

/// decode_script_bytes ($842E): the message's text in the bubble: codes $0A-$0C = a fighter's name by rank, $9A
/// big letters, $9C new line, $9D the seconds left, $9B the end.
fn decode_script_bytes(g: &mut Ik) {
    let m = g.msg_cur as usize;
    if m >= 0x28 {
        g.speech.width = 3;
        draw_panel_into_both_buffers(g);
        return;
    }
    {
        let s = &mut g.speech;
        s.big = false;
        s.saved_line = 0;
        s.line = 0;
        s.ch = 0;
        s.cursor = s.left.wrapping_add(1);
    }
    g.speech.colour = msgs(g, M_COL + m);
    let text = if g.german {
        g.a.speech_de[m].clone()
    } else {
        g.a.speech[m].clone()
    };
    loop {
        let i = g.speech.ch as usize;
        let d0 = text.get(i).copied().unwrap_or(0x9b);
        match d0 {
            0..=9 => {
                for _ in 0..d0 {
                    self_13(g, 0x40);
                }
            }
            0x0a..=0x0c => {
                let d2 = g.order[(d0 - 0x0a) as usize] as usize;
                let n = msgs(g, M_COUNT + d2);
                g.speech.name_ch = msgs(g, M_START + d2);
                let keep = g.speech.colour;
                g.speech.colour = msgs(g, M_NCOL + d2);
                for _ in 0..n {
                    let k = g.speech.name_ch as usize;
                    let c = if g.german {
                        g.a.t("names_de").get(k).copied().unwrap_or(0x40)
                    } else {
                        msgs(g, M_NAMES + k)
                    };
                    self_13(g, c);
                    g.speech.name_ch += 1;
                }
                g.speech.colour = keep;
            }
            0x30..=0x5e => self_13(g, d0),
            0x9a => {
                g.speech.big = true;
                if g.speech.ch == 0 {
                    g.speech.saved_line = g.speech.line;
                    g.speech.line = 3;
                }
            }
            0x9b => return,
            0x9c => {
                if g.speech.line < 3 {
                    g.speech.cursor = g.speech.left.wrapping_add(1);
                    g.speech.line += 1;
                }
            }
            0x9d => {
                let s = g.shown_secs;
                self_13(g, if s >> 4 == 0 { 0x40 } else { (s >> 4) + 0x30 });
                self_13(g, (s & 15) + 0x30);
            }
            0x9e => {
                g.speech.big = false;
                if g.speech.ch == 0 {
                    g.speech.line = g.speech.saved_line;
                }
            }
            _ => {}
        }
        g.speech.ch = g.speech.ch.wrapping_add(1);
        if g.speech.ch >= 0x80 {
            return;
        }
    }
}

/// self_13 ($862A): one letter of the 5-row font at the bubble's cursor (twice as tall in big mode).
fn self_13(g: &mut Ik, ch: u8) {
    let d0 = ch.wrapping_sub(0x30) as usize;
    let t = g.a.t("msgs");
    let li = (g.speech.line as usize).min(3) * 4 + M_LINES;
    let off = u32::from_be_bytes([t[li], t[li + 1], t[li + 2], t[li + 3]]) as usize;
    let row0 = off / 160;
    let cell = (off % 160) as u8 + (g.speech.cursor & 0x3f);
    let glyph = g.a.font5.get(d0).copied().unwrap_or([0; 5]);
    let col = g.speech.colour;
    let mut row = row0;
    for &b in glyph.iter() {
        // the letter's pixels replace the cell's row (the rest of the cell keeps the bubble colour)
        mask_rows(g, cell, row, b, 1, col);
        row += 1;
        if g.speech.big {
            mask_rows(g, cell, row, b, 1, col);
            row += 1;
        }
    }
    g.speech.cursor = g.speech.cursor.wrapping_add(1);
}

/// restore_global_variable_and_call ($7B94): the judge (talking while the message is new) and his sparkles.
fn restore_global_variable_and_call(g: &mut Ik) {
    let (f, fc, x) = (g.frame_x[0], g.facing[0], g.x[0]);
    let mut fr = 0x46 + g.trousers;
    let m = g.msg_cur as usize;
    if g.step_wait < 10 && m < 0x30 && msgs(g, M_TALK + m) != 0 {
        fr = [0x46, 0x47, 0x48, 0x47][(g.steps & 3) as usize] + g.trousers;
    }
    g.frame_x[0] = fr;
    g.facing[0] = 0;
    g.x[0] = g.speech.judge.wrapping_mul(4).wrapping_add(0x14);
    g.draw_fighter(0);
    g.frame_x[0] = f;
    g.facing[0] = fc;
    g.x[0] = x;
    decrement_flag_loop(g);
}

/// decrement_flag_loop ($7C5A): eight sparkles round the judge while a message is new.
fn decrement_flag_loop(g: &mut Ik) {
    let t = g.a.t("sparkle").to_vec();
    for k in (0..8).rev() {
        if g.speech.sp_t[k] == 0 {
            if g.speech.sparkles == 0 {
                continue;
            }
            g.speech.sparkles -= 1;
            let r = (g.random() & 0x1f) as usize;
            g.speech.sp_x[k] = t[r];
            let r = (g.random() & 0x1f) as usize;
            g.speech.sp_y[k] = t[33 + r];
            g.speech.sp_t[k] = 3;
        } else {
            g.speech.sp_t[k] -= 1;
        }
        let x = (g.speech.judge.wrapping_mul(4).wrapping_add(0xe)).wrapping_add(g.speech.sp_x[k])
            as i32;
        let y = g.speech.sp_y[k] as i32 + 0x8c;
        let s = Bob::at(0x227C0 + 0x600 + 2 * g.speech.sp_t[k] as u32);
        let b = g.back;
        g.bob2(b, s, x, y, 5, true);
    }
}
