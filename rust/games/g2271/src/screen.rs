//! The game's three pictures, as the original kept them, but as indexed pixels in game coordinates rather than
//! bitplanes: the clean background (tiles and anything drawn into it for good), and two display pictures that the
//! game draws its moving shapes into and shows in turn. Display pictures are restored from the clean one, box by
//! box, as the original's restore list did (+BE2A, +BEAE, +BFD2).
//!
//! Coordinates: the status panel is lines 0 to 23 at x 0 to 351 (never scrolled); the play area is lines 24 to 199
//! at world x (map column times 8), shown from the scroll position.

use crate::assets::{Shape, H, PANEL_H, W};

/// Width of the original's screen rows in pixels (44 bytes); the panel uses the first 320.
pub const ROW_W: usize = 352;

#[derive(Clone)]
pub struct Pic {
    pub w: usize,
    pub px: Vec<u8>,
}

impl Pic {
    pub fn new(w: usize) -> Pic {
        Pic {
            w,
            px: vec![0; w * H],
        }
    }
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: u8) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < H {
            self.px[y as usize * self.w + x as usize] = c;
        }
    }
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < H {
            self.px[y as usize * self.w + x as usize]
        } else {
            0
        }
    }
}

/// Where a shape is drawn (the original's mode argument of +C29A).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Into {
    /// Straight into the picture being drawn (modes 0, 4 and 5).
    Work,
    /// Into the clean background (mode 3).
    Clean,
    /// Into the clean background, and restored into both display pictures so it shows (modes 1 and 2).
    CleanShow,
}

pub struct Screen {
    pub clean: Pic,
    pub disp: [Pic; 2],
    /// The display picture being drawn (the other one is on screen).
    pub work: usize,
    /// Pixel scroll of the play area (the original's display position).
    pub scroll: i32,
    /// Boxes to restore from the clean picture before the next frame is drawn, per display picture.
    pub restore: [Vec<(i32, i32, i32, i32)>; 2],
    /// Text colours: ink and paper (+BB6C, five bits each).
    pub ink: u8,
    pub paper: u8,
    /// The palette as set (+CC3A) and the brightness of the fade, 0 to 15 (+CD08 out, +CCC6 in).
    pub palette: [u16; 32],
    pub level: u8,
    /// Shapes drawn into a display picture stop at the top of the play area (on during a stage).
    pub clip_top: bool,
    /// The display scroll's control (+C60A, +C7A4, the vertical blank's +CA78): the position it moves to (-$2BD8),
    /// the direction it is moving (-$2BCA: 4 left, 8 right), the direction the game asked for (-$2BCC, taken when
    /// a frame is shown with the scroll on a column boundary), and the move last started (-$2BC2).
    pub scroll_to: i32,
    pub scroll_dir: i32,
    pub scroll_want: i32,
    pub scroll_started: i32,
}

impl Screen {
    /// +BC9A initialize_object_state: the display scroll and its target at a position, no move asked or running.
    pub fn set_scroll(&mut self, x: i32) {
        self.scroll = x;
        self.scroll_to = x;
        self.scroll_want = 0;
        self.scroll_dir = 0;
    }

    pub fn new(world_w: usize) -> Screen {
        let w = world_w.max(ROW_W);
        Screen {
            clean: Pic::new(w),
            disp: [Pic::new(w), Pic::new(w)],
            work: 0,
            scroll: 0,
            restore: [Vec::new(), Vec::new()],
            ink: 1,
            paper: 0,
            palette: [0; 32],
            level: 0,
            clip_top: false,
            scroll_to: 0,
            scroll_dir: 0,
            scroll_want: 0,
            scroll_started: 0,
        }
    }

    /// A new world width (a new screen file): all three pictures start black.
    pub fn resize(&mut self, world_w: usize) {
        let w = world_w.max(ROW_W);
        self.clean = Pic::new(w);
        self.disp = [Pic::new(w), Pic::new(w)];
        self.restore = [Vec::new(), Vec::new()];
    }

    /// Writes one 8 by 8 tile into the clean picture and both display pictures (+C98C, +C8DC).
    pub fn tile(&mut self, x: i32, y: i32, t: &[u8; 64]) {
        for r in 0..8 {
            for c in 0..8 {
                let v = t[r * 8 + c];
                self.clean.set(x + c as i32, y + r as i32, v);
                self.disp[0].set(x + c as i32, y + r as i32, v);
                self.disp[1].set(x + c as i32, y + r as i32, v);
            }
        }
    }

    /// Copies the clean picture to the picture being drawn (+BDAE).
    pub fn copy_clean(&mut self) {
        let w = self.work;
        self.disp[w].px.copy_from_slice(&self.clean.px);
        self.restore[w].clear();
    }

    /// Restores this picture's boxes from the clean one (+BFD2).
    pub fn restore_boxes(&mut self) {
        let w = self.work;
        let boxes = std::mem::take(&mut self.restore[w]);
        for (x0, y0, x1, y1) in boxes {
            for y in y0.max(0)..y1.min(H as i32) {
                for x in x0.max(0)..x1.min(self.clean.w as i32) {
                    let v = self.clean.get(x, y);
                    self.disp[w].set(x, y, v);
                }
            }
        }
    }

    /// Draws a shape frame with its mask at (x, y), less its anchor (+C29A). Shapes use colours 0 to 15 (the
    /// original cleared the fifth plane under the mask).
    pub fn shape(&mut self, s: &Shape, x: i32, y: i32, into: Into, keep: bool) {
        let x0 = x - s.ax as i32;
        let y0 = y - s.ch as i32 - s.ay as i32;
        let w = self.work;
        let pic = match into {
            Into::Work => &mut self.disp[w],
            Into::Clean | Into::CleanShow => &mut self.clean,
        };
        for r in 0..s.h {
            let yy = y0 + r as i32;
            // the play area's shapes are clipped at the top of the play area (+C29A, -$2BE2)
            if !(0..H as i32).contains(&yy)
                || (into == Into::Work && self.clip_top && yy < PANEL_H as i32)
            {
                continue;
            }
            for c in 0..s.w {
                let v = s.px[r * s.w + c];
                if v != 255 {
                    pic.set(x0 + c as i32, yy, v & 15);
                }
            }
        }
        let b = (x0, y0, x0 + s.w as i32 + 16, y0 + s.h as i32 + 1);
        if into == Into::Work && !keep {
            self.restore[w].push(b);
        }
        if into == Into::CleanShow {
            self.restore[0].push(b);
            self.restore[1].push(b);
        }
    }

    /// Draws one character of the 8 by 8 font in the text colours (+B9E2). `panel`: unscrolled panel position.
    pub fn char(&mut self, glyph: &[u8; 64], x: i32, y: i32, panel: bool, both: bool) {
        let dx = if panel { 0 } else { self.scroll & !15 };
        for r in 0..8 {
            for c in 0..8 {
                let v = if glyph[r * 8 + c] != 0 {
                    self.ink
                } else {
                    self.paper
                };
                let (px, py) = (dx + (x & !7) + c as i32, y + r as i32);
                self.clean.set(px, py, v);
                if both {
                    self.disp[0].set(px, py, v);
                    self.disp[1].set(px, py, v);
                } else {
                    let w = self.work;
                    self.disp[w].set(px, py, v);
                }
            }
        }
    }

    /// Ends a frame: the picture drawn becomes the one shown (+CAB4).
    pub fn flip(&mut self) {
        self.work ^= 1;
    }

    /// The picture on screen (320 by 200 indexed, 32 colours) and its palette at the fade's brightness.
    pub fn view(&self) -> (Vec<u8>, [u16; 32]) {
        let shown = &self.disp[self.work ^ 1];
        let mut out = vec![0u8; W * H];
        for y in 0..H {
            let sx = if y < PANEL_H { 0 } else { self.scroll };
            for x in 0..W {
                out[y * W + x] = shown.get(sx + x as i32, y as i32);
            }
        }
        let mut pal = self.palette;
        for c in pal.iter_mut() {
            *c = fade(*c, self.level);
        }
        (out, pal)
    }
}

/// A 12-bit colour at brightness `level` (0 to 15).
pub fn fade(c: u16, level: u8) -> u16 {
    let l = level.min(15) as u16;
    let f = |v: u16| (v * l + 7) / 15;
    (f((c >> 8) & 15) << 8) | (f((c >> 4) & 15) << 4) | f(c & 15)
}
