//! Drawing the playfield (indexed, 320 x 196): the dithered gradient background, the bars, balls and ships (which
//! wrap round the edges), the tethers, the 16-line font with its shadow, the rounded corners and the status line.
//! Colours: 0 to 7, 8 to 15 and 16 to 23 are the three ball colours' ramps, 24 the text, 30 and 31 the two colours of
//! the gradient band a line is in.

use crate::assets::{Img, PH, PW};
use crate::emo::Emo;

/// Which screen a routine draws on.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum On {
    Back,
    Clean,
}

/// The colour bank a ball, bar or tile colour word selects ($00, $10, $20 -> ramps 0, 8, 16).
pub fn bank(colour: u16) -> u8 {
    match (colour >> 2) & 0xC {
        0 => 0,
        4 => 8,
        _ => 16,
    }
}

/// generate_bit_permutation_table (+$2DBC): row k of 14 has k/14 of its pixels set, spread evenly.
pub fn dither_row(k: usize) -> [u8; PW] {
    let mut row = [0u8; PW];
    let d1 = (k as u16).wrapping_mul(0x1249);
    let mut acc: u16 = 0x8000;
    for p in row.iter_mut() {
        let (v, c) = acc.overflowing_add(d1);
        acc = v;
        *p = c as u8;
    }
    row
}

impl Emo {
    pub fn buf(&mut self, on: On) -> &mut Vec<u8> {
        match on {
            On::Back => &mut self.screens[self.drawbuf],
            On::Clean => &mut self.bg,
        }
    }

    /// clear_object_flags (+$2D88): the clean screen becomes the gradient (colours 30 and 31, dithered).
    pub fn clear_clean(&mut self) {
        for k in 0..14 {
            let row = dither_row(k);
            for y in (k..PH).step_by(14) {
                for (d, r) in self.bg[y * PW..(y + 1) * PW].iter_mut().zip(row.iter()) {
                    *d = 30 | r;
                }
            }
        }
    }

    /// copy_struct_block (+$2D36): the clean screen to the screen being drawn.
    pub fn copy_clean(&mut self) {
        let b = self.bg.clone();
        self.screens[self.drawbuf].copy_from_slice(&b);
    }

    /// copy_struct_array (+$2D26): the clean screen to both screens.
    pub fn copy_clean_both(&mut self) {
        let b = self.bg.clone();
        self.screens[0].copy_from_slice(&b);
        self.screens[1].copy_from_slice(&b);
    }

    fn put(&mut self, on: On, x: i32, y: i32, c: u8) {
        if (0..PW as i32).contains(&x) && (0..PH as i32).contains(&y) {
            self.buf(on)[y as usize * PW + x as usize] = c;
        }
    }

    /// self_6 (+$24AC): a 16 x 16 tile centred on (x, y), drawn through its mask in a colour bank.
    pub fn tile(&mut self, on: On, t: usize, x: i32, y: i32, bank: u8) {
        let a = self.a.clone();
        let img = &a.tiles[t];
        for r in 0..16 {
            for c in 0..16 {
                let p = img.at(c, r);
                if p & 0x80 != 0 {
                    self.put(on, x - 8 + c as i32, y - 8 + r as i32, (p & 7) | bank);
                }
            }
        }
    }

    /// build_screen_from_layout (+$435E): the bars of a level (or a panel) on the clean screen. Horizontal bars are a
    /// shaded body between two end caps; vertical bars' caps join the horizontal bars they meet.
    pub fn build_layout(&mut self, hb: &[[i16; 4]], vb: &[[i16; 4]]) {
        let a = self.a.clone();
        self.hbars = hb.iter().map(|b| [b[0], b[1], b[2]]).collect();
        self.vbars = vb.iter().map(|b| [b[0], b[1], b[2]]).collect();
        for b in hb {
            let (x, y, len, k) = (b[0] as i32, b[1] as i32, b[2] as i32, bank(b[3] as u16));
            // extract_image_from_table (+$229C): the body's 16 rows repeat the pattern tile's columns
            for r in 0..16 {
                for sx in x + 8..x + len - 8 {
                    let p = a.tiles[3].at((sx & 15) as usize, r) & 7;
                    self.put(On::Clean, sx, y - 8 + r as i32, p | k);
                }
            }
            self.tile(On::Clean, 4, x, y, k);
            self.tile(On::Clean, 5, x + len, y, k);
        }
        for b in vb {
            let (x, y, len, k) = (b[0] as i32, b[1] as i32, b[2] as i32, bank(b[3] as u16));
            // write_planes_from_source (+$23DE): the body repeats tile 0's rows
            for r in 0..(len - 16).max(0) {
                for c in 0..16 {
                    let p = a.tiles[0].at(c, (r & 15) as usize) & 7;
                    self.put(On::Clean, x - 8 + c as i32, y + 8 + r, p | k);
                }
            }
            let top = self.join_kind(x as i16, y as i16);
            self.tile(On::Clean, top, x, y, k);
            let bot = self.join_kind(x as i16, (y + len) as i16) + 1;
            self.tile(On::Clean, bot, x, y + len, k);
        }
    }

    /// layout_join_kind (+$43D2): the cap a vertical bar end needs where it meets a horizontal bar.
    fn join_kind(&self, x: i16, y: i16) -> usize {
        for b in &self.hbars {
            if b[1] == y {
                if b[0] == x {
                    return 6;
                }
                if b[0].wrapping_add(b[2]) == x {
                    return 8;
                }
            }
        }
        1
    }

    /// An image whose top-left corner is (x, y), wrapping round the playfield's edges, through its mask; `map` turns
    /// the image's colour into the screen's.
    pub fn sprite(&mut self, img: &Img, x: i32, y: i32, map: impl Fn(u8) -> u8) {
        let b = &mut self.screens[self.drawbuf];
        for r in 0..img.h {
            let sy = (y + r as i32).rem_euclid(PH as i32) as usize;
            for c in 0..img.w {
                let p = img.at(c, r);
                if p & 0x80 != 0 {
                    let sx = (x + c as i32).rem_euclid(PW as i32) as usize;
                    b[sy * PW + sx] = map(p & 0x7F);
                }
            }
        }
    }

    /// draw_sprite_frame (+$1D6E): a ball of `size` centred on (x, y) in its colour.
    pub fn draw_ball(&mut self, x: i32, y: i32, size: i16, colour: u16) {
        if !(0..24).contains(&size) {
            return;
        }
        let a = self.a.clone();
        let k = bank(colour);
        self.sprite(
            &a.balls[size as usize],
            x - size as i32,
            y - size as i32,
            |p| (p & 7) | k,
        );
    }

    /// The ship (+$1B98): frame 0 to 15 of its rotation, centred on (x, y); player two's has its colours swapped.
    /// Drawing it across an edge is what the "never wrapped" secret bonus notices.
    pub fn draw_ship(&mut self, x: i32, y: i32, frame: usize, two: bool) {
        let a = self.a.clone();
        let mut dx = x - 16;
        if dx < 0 {
            dx += 320;
        }
        let mut dy = y - 16;
        if dy < 0 {
            dy += 196;
        }
        if dx & !15 >= 288 || dy + 32 > 196 {
            self.wrapped = true;
        }
        self.sprite(
            &a.ship[frame & 15],
            dx,
            dy,
            |p| if two { swap34(p) } else { p },
        );
    }

    /// The exploding ship (+$1A06): the ship's rows fly apart from its middle line, `spread` (16.16) apart.
    pub fn draw_explosion(&mut self, x: i32, y: i32, spread: u32, two: bool) {
        let a = self.a.clone();
        let img = &a.ship[0];
        let mut dx = x - 16;
        if dx < 0 {
            dx += 320;
        }
        let b = &mut self.screens[self.drawbuf];
        let mut row = |r: usize, yy: i32| {
            for c in 0..32 {
                let p = img.at(c, r);
                if p & 0x80 != 0 {
                    let sx = (dx + c as i32).rem_euclid(PW as i32) as usize;
                    let v = p & 0x7F;
                    b[yy as usize * PW + sx] = if two { swap34(v) } else { v };
                }
            }
        };
        let y0 = (y as u32) << 16;
        let mut d1 = y0 as i32;
        for k in 0..16 {
            row(16 + k, d1 >> 16);
            d1 = d1.wrapping_add(spread as i32);
            if d1 >= 0xC4_0000 {
                break;
            }
        }
        let mut d1 = (y0 as i32).wrapping_sub(spread as i32);
        for k in 0..16 {
            if d1 < 0 {
                break;
            }
            row(15 - k, d1 >> 16);
            d1 = d1.wrapping_sub(spread as i32);
        }
    }

    /// The tether (+$206C): a one-pixel line in colour 1 (keeping what is under it in the top bitplane).
    pub fn draw_tether(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (mut x, mut y, ex, ey) = if x1 < x0 {
            (x1, y1, x0, y0)
        } else {
            (x0, y0, x1, y1)
        };
        let dx = ex - x;
        let dy = (ey - y).abs();
        let sy = if ey >= y { 1 } else { -1 };
        let b = &mut self.screens[self.drawbuf];
        let mut plot = |x: i32, y: i32| {
            if (0..PW as i32).contains(&x) && (0..PH as i32).contains(&y) {
                let p = &mut b[y as usize * PW + x as usize];
                *p = (*p & 16) | 1;
            }
        };
        if dx > dy {
            let mut err = dx >> 1;
            plot(x, y);
            for _ in 0..dx {
                err -= dy;
                if err < 0 {
                    err += dx;
                    y += sy;
                }
                x += 1;
                plot(x, y);
            }
        } else {
            let mut err = dy >> 1;
            for _ in 0..=dy {
                plot(x, y);
                err -= dx;
                if err < 0 {
                    err += dy;
                    x += 1;
                }
                y += sy;
            }
        }
    }

    /// draw_char / draw_two_chars (+$1328, +$1382): text at (x, y) (x a multiple of 8) in colour 24 with a shadow one
    /// pixel right and down that clears the colour under it (the shadow's last pixel wraps round within the
    /// character, or the pair for two-character drawing, as the original's rotate did).
    pub fn text(&mut self, on: On, x: i32, y: i32, s: &[u8], pairs: bool) {
        let a = self.a.clone();
        let cell = if pairs { 16 } else { 8 };
        let mut cx = x;
        for ch in s.chunks(if pairs { 2 } else { 1 }) {
            // the glyph rows, `cell` pixels wide
            let mut rows = [0u32; 16];
            for (k, c) in ch.iter().enumerate() {
                let g = (*c as usize).wrapping_sub(32);
                if g < 96 {
                    for (r, row) in rows.iter_mut().enumerate() {
                        for px in 0..8 {
                            if a.font16[g][r * 8 + px] != 0 {
                                *row |= 1 << (cell - 1 - (k * 8 + px));
                            }
                        }
                    }
                }
            }
            let full = (1u32 << cell) - 1;
            let ror = |v: u32| ((v >> 1) | ((v & 1) << (cell - 1))) & full;
            let mut shadow = 0u32; // pixels the previous row's shadow clears
            for (r, &g) in rows.iter().enumerate() {
                for px in 0..cell {
                    let bit = 1 << (cell - 1 - px);
                    let (sx, sy) = (cx + px as i32, y + r as i32);
                    if !(0..PW as i32).contains(&sx) || !(0..PH as i32).contains(&sy) {
                        continue;
                    }
                    let i = sy as usize * PW + sx as usize;
                    let old = self.buf(on)[i];
                    let v = if g & bit != 0 {
                        24
                    } else if shadow & bit != 0 {
                        old & 16
                    } else {
                        old
                    };
                    self.buf(on)[i] = v;
                }
                shadow = ror(g);
            }
            if shadow != 0 {
                let sy = y + 16;
                for px in 0..cell {
                    if shadow & (1 << (cell - 1 - px)) != 0 {
                        self.put(on, cx + px as i32, sy, 0);
                    }
                }
            }
            cx += cell as i32;
        }
    }

    /// A number's digits (0 to 9 each) as characters.
    pub fn digits(&mut self, on: On, x: i32, y: i32, d: &[u8], pairs: bool) {
        let s: Vec<u8> = d.iter().map(|v| v.wrapping_add(b'0')).collect();
        self.text(on, x, y, &s, pairs);
    }

    /// mask_screen_edges (+$12C0): the screen's rounded corners (the top four bitplanes cleared outside them).
    pub fn mask_edges(&mut self) {
        let a = self.a.clone();
        let b = &mut self.screens[self.drawbuf];
        for r in 0..8 {
            let (l, rt) = (a.tables.edge_left[r], a.tables.edge_right[r]);
            for &row in &[r, PH - 1 - r] {
                for c in 0..16 {
                    if l & (0x8000 >> c) == 0 {
                        b[row * PW + c] &= 16;
                    }
                    if rt & (0x8000 >> c) == 0 {
                        b[row * PW + 304 + c] &= 16;
                    }
                }
            }
        }
    }

    /// The energy bar (+$256C): two pixels a unit from (64, 6), four lines of alternate text-colour pixels and a
    /// shadow line under them.
    pub fn draw_energy(&mut self) {
        let e = self.energy.clamp(0, 0x68) as i32;
        let b = &mut self.screens[self.drawbuf];
        for r in 0..4 {
            for c in 0..2 * e {
                b[(6 + r) * PW + 64 + c as usize] = if c & 1 == 0 { 24 } else { 0 };
            }
        }
        for c in 0..2 * e {
            b[10 * PW + 64 + c as usize] = 0;
        }
    }

    /// The score (+$4862), lives (+$4848) and the bonus levels' clock (+$4828) on the status line.
    pub fn draw_score(&mut self) {
        let s = self.score;
        self.digits(On::Back, 8, 2, &s[0..1], false);
        self.digits(On::Back, 16, 2, &s[1..3], true);
        self.digits(On::Back, 32, 2, &s[3..5], true);
        self.digits(On::Back, 48, 2, &s[5..6], false);
    }

    pub fn draw_lives(&mut self) {
        let (mut t, mut u) = ((self.lives >> 8) as u8, self.lives as u8);
        if t & 0x80 != 0 {
            t = 0;
            u = 0;
        }
        self.digits(On::Back, 288, 2, &[t, u], true);
    }

    pub fn draw_clock(&mut self) {
        let c = self.clock;
        self.digits(On::Back, 144, 2, &[c[0], 10], true);
        self.digits(On::Back, 160, 2, &[c[1], c[2]], true);
    }

    /// apply_bitmask_to_table (+$1126): a 320 x 32 heading over the top of the clean screen.
    pub fn heading(&mut self, img: &Img) {
        for r in 0..32 {
            for c in 0..PW {
                let p = img.at(c, r);
                if p & 0x80 != 0 {
                    self.bg[r * PW + c] = p & 15;
                }
            }
        }
    }
}

/// Player two's ship: bitplanes 4 and 5 swapped.
pub fn swap34(p: u8) -> u8 {
    (p & 0x07) | ((p & 8) << 1) | ((p & 16) >> 1)
}
