//! The crack's intro, kept as the game's intro piece (Karl: part of gaming culture): the Setrox/Alcatraz trainer menu
//! (runtime $050000) with its rippling selected line, its colour-cycling copper lines, the scroller and the SoundMon
//! music, then the crack loader's two pictures (preseuro.lbm, e_motion.lbm) with their fades. The trainer's four
//! options (unlimited lives, energy and time, the start level) are the native build's cheats, all off at the start,
//! and act as the trainer's code patches did.

use native_engine::gfx::{rgb12, Argb};

use crate::assets::{Img, H, TOP_LINE, W};
use crate::emo::{fade_colour, Display, Emo, DOWN, FIRE, UP};
use crate::soundmon::SoundMon;

/// The intro's bitplane rows start on display line $29, the scroller's on line $E0.
const MENU_LINE: i32 = 0x29;
const SCROLL_LINE: i32 = 0xE0;
/// The selected row's copper lines for row 4 (START GAME), as the intro set them up.
const SEL_TOP: i32 = 0xBE;

impl Emo {
    /// The trainer menu (initialize_game_state_two, +$30, and its frame loop, +$236).
    pub fn intro(&mut self) {
        let a = self.a.clone();
        let it = &a.intro;
        self.display = Display::Composed;
        let mut sm = SoundMon::new(&a.soundmon, &it.periods);
        // the text plane (320 x 216, one bit a pixel) and the scroller's (352 x 8)
        let mut plane = vec![0u8; 320 * 216];
        let menu: Vec<(u8, u8, String)> = it.menu.clone();
        let mut values = it.values.clone();
        for (x, y, s) in it.header.iter().chain(menu.iter()) {
            put_text(&a.font8, &mut plane, *x as usize, *y as usize, s.as_bytes());
        }
        let mut scroll = vec![0u8; 352 * 8];
        let (mut sub, mut speed, mut spos, mut pause) = (4u8, 4u8, 0usize, 0u8);
        let mut acc: i32 = 0x40;
        let mut row;
        let mut wob = 0usize;
        let mut cyc = 0usize;
        let mut repeat = 6u8;
        loop {
            self.wait_frame();
            self.sounds.extend(sm.frame());
            // update_hud_digit_display (+$312): the mouse picks the row (here up and down move a row at a time)
            let j = self.joy1;
            let new = j & !self.prev1;
            self.prev1 = j;
            if new & UP != 0 {
                acc = (acc - 16).max(0);
            }
            if new & DOWN != 0 {
                acc = (acc + 16).min(0x40);
            }
            row = (acc >> 4) as usize;
            let left = j & FIRE != 0;
            let right = self.input.held(native_engine::input::Action::Fire2)
                || self.input.held(native_engine::input::Action::Secondary);
            let kind = it.rows[row * 2];
            let mut redraw = None;
            match kind {
                0 => {
                    if left {
                        values[row] = 0xFF;
                        redraw = Some(it.on.clone());
                    } else if right {
                        values[row] = 0;
                        redraw = Some(it.off.clone());
                    }
                }
                1 => {
                    if repeat > 0 {
                        repeat -= 1;
                    } else {
                        let max = it.rows[row * 2 + 1];
                        let v = values[row];
                        let nv = if left && v < max {
                            Some(v + 1)
                        } else if right && !left && v > 1 {
                            Some(v - 1)
                        } else {
                            None
                        };
                        if let Some(nv) = nv {
                            repeat = 6;
                            values[row] = nv;
                            redraw = Some(format!("0{:02}", nv));
                        }
                    }
                }
                _ => {
                    if new & FIRE != 0 {
                        break;
                    }
                }
            }
            if let Some(t) = redraw {
                let y = menu[row + 1].1 as usize;
                put_text(&a.font8, &mut plane, 0x1E, y, t.as_bytes());
            }
            // update_bob_animation (+$4FC): the scroller
            if pause > 0 {
                pause -= 1;
            } else {
                sub = (sub + speed) & 7;
                if sub == 0 {
                    if spos >= it.scroll.len() {
                        spos = 0;
                    }
                    let mut c = it.scroll[spos];
                    spos += 1;
                    if c < 2 {
                        let v = *it.scroll.get(spos).unwrap_or(&0);
                        if c == 0 {
                            pause = v;
                        } else {
                            speed = v;
                        }
                        c = *it.scroll.get(spos + 1).unwrap_or(&b' ');
                        spos += 2;
                    }
                    let g = (c as usize).wrapping_sub(32);
                    for r in 0..8 {
                        for x in 0..8 {
                            scroll[r * 352 + 320 + x] =
                                if g < 80 { a.font8[g][r * 8 + x] } else { 0 };
                        }
                    }
                }
                let s = speed as usize & 15;
                for r in 0..8 {
                    let line = &mut scroll[r * 352..(r + 1) * 352];
                    line.copy_within(s.., 0);
                    for p in line[352 - s..].iter_mut() {
                        *p = 0;
                    }
                }
            }
            // update_animation_frames (+$46C): the ripple and the copper colours move on
            wob = (wob + 1) % 43;
            cyc = cyc % 31 + 1;
            self.composed = compose(&plane, &scroll, it, row, wob, cyc, &it.gradient);
        }
        self.sounds.extend(sm.stop());
        // the options go where the trainer's code patches went (l0_024E)
        self.trainer = [values[0] != 0, values[1] != 0, values[2] != 0];
        self.start_level = values[3].max(1) as u16;
    }

    /// The crack loader's pictures (initialize_game_state, hunk 2): preseuro.lbm faded up and held three seconds,
    /// faded out, then e_motion.lbm faded up while the game loads.
    pub fn loader(&mut self) {
        let a = self.a.clone();
        self.show_picture(&a.pic_preseuro, &a.cmap_preseuro, true);
        for _ in 0..150 {
            self.wait_frame();
        }
        for lvl in 0..16 {
            self.wait_frame();
            self.composed = picture(&a.pic_preseuro, &a.cmap_preseuro, lvl);
        }
        self.show_picture(&a.pic_emotion, &a.cmap_emotion, true);
        // the display goes off for the game
        self.composed = vec![0xFF00_0000; W * H];
    }

    fn show_picture(&mut self, img: &Img, cmap: &[u16], fade_up: bool) {
        self.display = Display::Composed;
        if fade_up {
            for lvl in (0..16).rev() {
                self.wait_frame();
                self.composed = picture(img, cmap, lvl);
            }
        }
    }
}

/// load_animation_frames (+$1DE): 8 x 8 characters on the text plane at byte column x, pixel row y.
fn put_text(font: &[[u8; 64]], plane: &mut [u8], x: usize, y: usize, s: &[u8]) {
    for (k, c) in s.iter().enumerate() {
        let g = (*c as usize).wrapping_sub(32);
        for r in 0..8 {
            for px in 0..8 {
                let sx = (x + k) * 8 + px;
                let sy = y + r;
                if sx < 320 && sy < 216 {
                    plane[sy * 320 + sx] = if g < 80 { font[g][r * 8 + px] } else { 0 };
                }
            }
        }
    }
}

/// A 320 x 200 picture (display lines 58 on) at a fade level.
fn picture(img: &Img, cmap: &[u16], lvl: u16) -> Vec<Argb> {
    let mut o = vec![0xFF00_0000; W * H];
    let pal: Vec<Argb> = (0..32)
        .map(|i| rgb12(fade_colour(*cmap.get(i).unwrap_or(&0), lvl)))
        .collect();
    let top = (0x3A - TOP_LINE) as usize;
    for y in 0..img.h.min(H - top) {
        for x in 0..W.min(img.w) {
            o[(top + y) * W + x] = pal[(img.at(x, y) & 31) as usize];
        }
    }
    o
}

/// The intro's frame: two bitplanes (the text and itself one line lower and one pixel right, a shadow), its copper
/// lines and the ripple on the selected row.
fn compose(
    plane: &[u8],
    scroll: &[u8],
    it: &crate::assets::Intro,
    row: usize,
    wob: usize,
    cyc: usize,
    grad: &[u16],
) -> Vec<Argb> {
    let mut o = vec![0xFF00_0000u32; W * H];
    let cols = [
        rgb12(0),
        rgb12(it.colours[0]),
        rgb12(it.colours[1]),
        rgb12(it.colours[2]),
    ];
    let sel = SEL_TOP - (4 - row as i32) * 11;
    let gl = |k: usize| rgb12(grad[(cyc + k) % grad.len().max(1)]);
    for oy in 0..H as i32 {
        let line = oy + TOP_LINE;
        let dst = &mut o[oy as usize * W..(oy as usize + 1) * W];
        // the copper's colour lines: the top one, the selected row's pair, the bottom one
        let bar = match line {
            0x30 => Some(false),
            l if l == sel || l == sel + 10 => Some(l != sel),
            0xFD => Some(true),
            _ => None,
        };
        if let Some(rev) = bar {
            for (x, d) in dst.iter_mut().enumerate() {
                let k = x / 8 + 5;
                *d = gl(if rev { 49usize.saturating_sub(k) } else { k });
            }
            continue;
        }
        let (src, stride, r0, shift) = if line >= SCROLL_LINE {
            (scroll, 352usize, SCROLL_LINE, 0i32)
        } else {
            (plane, 320usize, MENU_LINE, 8i32)
        };
        let r = line - r0;
        if r < 0 {
            continue;
        }
        let r = r as usize;
        let rows = src.len() / stride;
        // the ripple: lines sel+2 to sel+9 take the scroll values the table gives them this frame
        let mut s1 = shift;
        let mut s2 = shift + 1;
        if line >= SCROLL_LINE {
            s1 = 0;
            s2 = 1;
        } else if line > sel + 1 && line < sel + 10 {
            let v = it.wobble[(wob + (line - sel - 2) as usize) % 43];
            s1 = (v & 15) as i32;
            s2 = ((v >> 4) & 15) as i32;
        }
        for (x, d) in dst.iter_mut().enumerate() {
            let p1x = x as i32 - s1;
            let p2x = x as i32 - s2;
            let b1 = r < rows
                && p1x >= 0
                && (p1x as usize) < stride
                && src[r * stride + p1x as usize] != 0;
            let b2 = r >= 1
                && r - 1 < rows
                && p2x >= 0
                && (p2x as usize) < stride
                && src[(r - 1) * stride + p2x as usize] != 0;
            *d = cols[b1 as usize | (b2 as usize) << 1];
        }
    }
    o
}
