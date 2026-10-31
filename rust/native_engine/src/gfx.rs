//! Indexed-colour graphics: palettes, bitmaps, sprite sheets, bitmap fonts and the frame a game draws into. Everything
//! is drawn on the CPU into a logical-resolution picture of palette indices; the platform layer converts it to RGB with
//! the frame's palette and scales it to the screen. Nothing here knows about any old machine's display hardware.

/// A 0xAARRGGBB colour.
pub type Argb = u32;

/// The colour of a 12-bit RGB value (4 bits a component, `0x0RGB`), as many 1980s and 1990s systems stored them.
pub fn rgb12(v: u16) -> Argb {
    let c = |s: u16| ((v >> s) & 15) as u32 * 17;
    0xFF00_0000 | c(8) << 16 | c(4) << 8 | c(0)
}

/// A palette of up to 256 colours.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    pub colours: Vec<Argb>,
}

impl Palette {
    pub fn new(n: usize) -> Palette {
        Palette {
            colours: vec![0xFF00_0000; n],
        }
    }

    /// From `#RRGGBB` strings.
    pub fn from_hex(v: &[&str]) -> Result<Palette, String> {
        let mut colours = Vec::new();
        for s in v {
            let h = s.trim_start_matches('#');
            let n = u32::from_str_radix(h, 16).map_err(|_| format!("bad colour {s:?}"))?;
            if h.len() != 6 {
                return Err(format!("bad colour {s:?}"));
            }
            colours.push(0xFF00_0000 | n);
        }
        Ok(Palette { colours })
    }

    pub fn get(&self, i: u8) -> Argb {
        self.colours.get(i as usize).copied().unwrap_or(0xFF00_0000)
    }
}

/// A rectangle in pixels; `w` and `h` may be zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }
    pub fn intersect(&self, o: &Rect) -> Rect {
        let x0 = self.x.max(o.x);
        let y0 = self.y.max(o.y);
        let x1 = (self.x + self.w).min(o.x + o.w);
        let y1 = (self.y + self.h).min(o.y + o.h);
        Rect::new(x0, y0, (x1 - x0).max(0), (y1 - y0).max(0))
    }
}

/// A picture of palette indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

/// How a font's pixels go onto a bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextMode {
    /// Ink pixels take this colour; the rest of the cell is left alone.
    Ink(u8),
    /// The whole cell is drawn: ink in the first colour, paper in the second.
    Opaque(u8, u8),
    /// Indexed-colour layering: in the whole cell, ink pixels set the `mask` bits of the colour index and paper
    /// pixels clear them, the other bits are kept (text written into one bit-layer of a picture).
    Layer(u8),
}

impl Bitmap {
    pub fn new(w: usize, h: usize) -> Bitmap {
        Bitmap {
            w,
            h,
            px: vec![0; w * h],
        }
    }

    pub fn from_indices(w: usize, h: usize, px: Vec<u8>) -> Result<Bitmap, String> {
        if px.len() != w * h {
            return Err(format!(
                "bitmap {w}x{h} needs {} bytes, got {}",
                w * h,
                px.len()
            ));
        }
        Ok(Bitmap { w, h, px })
    }

    pub fn rect(&self) -> Rect {
        Rect::new(0, 0, self.w as i32, self.h as i32)
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return 0;
        }
        self.px[y as usize * self.w + x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, c: u8) {
        if x >= 0 && y >= 0 && x < self.w as i32 && y < self.h as i32 {
            self.px[y as usize * self.w + x as usize] = c;
        }
    }

    pub fn fill(&mut self, c: u8) {
        self.px.fill(c);
    }

    pub fn fill_rect(&mut self, r: Rect, c: u8) {
        let r = r.intersect(&self.rect());
        for y in r.y..r.y + r.h {
            let o = y as usize * self.w;
            self.px[o + r.x as usize..o + (r.x + r.w) as usize].fill(c);
        }
    }

    /// Copies `src` area `s` to (`dx`, `dy`), clipped to `clip` and to this bitmap.
    pub fn blit(&mut self, src: &Bitmap, s: Rect, dx: i32, dy: i32, clip: Rect) {
        self.blit_with(src, s, dx, dy, clip, |_, sp, _| Some(sp));
    }

    /// Copies the pixels of `src` area `s` whose `mask` pixel (same coordinates in `mask`) is not zero.
    pub fn blit_masked(
        &mut self,
        src: &Bitmap,
        mask: &Bitmap,
        s: Rect,
        dx: i32,
        dy: i32,
        clip: Rect,
    ) {
        self.blit_with(src, s, dx, dy, clip, |(x, y), sp, _| {
            (mask.get(x, y) != 0).then_some(sp)
        });
    }

    /// Copies the pixels of `src` area `s` that are not colour `key`.
    pub fn blit_keyed(&mut self, src: &Bitmap, key: u8, s: Rect, dx: i32, dy: i32, clip: Rect) {
        self.blit_with(src, s, dx, dy, clip, |_, sp, _| (sp != key).then_some(sp));
    }

    /// The general copy: `f((sx, sy), source pixel, destination pixel)` gives the new pixel or None to keep it.
    pub fn blit_with(
        &mut self,
        src: &Bitmap,
        s: Rect,
        dx: i32,
        dy: i32,
        clip: Rect,
        f: impl Fn((i32, i32), u8, u8) -> Option<u8>,
    ) {
        let clip = clip.intersect(&self.rect());
        for y in 0..s.h {
            let ty = dy + y;
            if ty < clip.y || ty >= clip.y + clip.h {
                continue;
            }
            for x in 0..s.w {
                let tx = dx + x;
                if tx < clip.x || tx >= clip.x + clip.w {
                    continue;
                }
                let (sx, sy) = (s.x + x, s.y + y);
                if sx < 0 || sy < 0 || sx >= src.w as i32 || sy >= src.h as i32 {
                    continue;
                }
                let o = ty as usize * self.w + tx as usize;
                if let Some(c) = f(
                    (sx, sy),
                    src.px[sy as usize * src.w + sx as usize],
                    self.px[o],
                ) {
                    self.px[o] = c;
                }
            }
        }
    }

    /// A line from (x0, y0) to (x1, y1) inclusive in colour `c` (a run-sliced line stepping one pixel along the
    /// major axis per step), clipped.
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c: u8, clip: Rect) {
        self.line_with(x0, y0, x1, y1, clip, |_| c);
    }

    /// As `line`, with the new colour a function of the old one (e.g. `|old| old | 0x1F` to set bit-layers).
    pub fn line_with(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        clip: Rect,
        f: impl Fn(u8) -> u8,
    ) {
        let clip = clip.intersect(&self.rect());
        let (dx, dy) = ((x1 - x0).abs(), (y1 - y0).abs());
        let (sx, sy) = (if x1 >= x0 { 1 } else { -1 }, if y1 >= y0 { 1 } else { -1 });
        let (mut x, mut y) = (x0, y0);
        let mut put = |x: i32, y: i32| {
            if clip.contains(x, y) {
                let o = y as usize * self.w + x as usize;
                self.px[o] = f(self.px[o]);
            }
        };
        if dx >= dy {
            let mut err = 2 * dy - dx;
            for _ in 0..=dx {
                put(x, y);
                if err > 0 {
                    y += sy;
                    err -= 2 * dx;
                }
                err += 2 * dy;
                x += sx;
            }
        } else {
            let mut err = 2 * dx - dy;
            for _ in 0..=dy {
                put(x, y);
                if err > 0 {
                    x += sx;
                    err -= 2 * dy;
                }
                err += 2 * dx;
                y += sy;
            }
        }
    }
}

/// A strip of equal frames side by side in one bitmap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sheet {
    pub image: Bitmap,
    pub frame_w: usize,
    pub frame_h: usize,
    pub frames: usize,
}

impl Sheet {
    pub fn new(image: Bitmap, frame_w: usize, frames: usize) -> Result<Sheet, String> {
        if frames == 0 || image.w != frame_w * frames {
            return Err(format!(
                "sheet of {frames} frames {frame_w} wide does not fit a {} wide image",
                image.w
            ));
        }
        let frame_h = image.h;
        Ok(Sheet {
            image,
            frame_w,
            frame_h,
            frames,
        })
    }

    pub fn frame(&self, i: usize) -> Rect {
        Rect::new(
            (i % self.frames * self.frame_w) as i32,
            0,
            self.frame_w as i32,
            self.frame_h as i32,
        )
    }
}

/// A fixed-cell bitmap font: glyphs for `count` characters from `first`, one cell each in a strip (1 = ink).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub glyphs: Sheet,
    pub first: u32,
    /// Lower case is drawn with the upper-case glyphs.
    pub fold_lower: bool,
    /// Characters drawn with another character's glyph.
    pub alias: Vec<(char, char)>,
    /// Characters drawn as an empty cell.
    pub blank: Vec<char>,
}

impl Font {
    pub fn cell(&self) -> (usize, usize) {
        (self.glyphs.frame_w, self.glyphs.frame_h)
    }

    /// The glyph for `c`: Some(frame), or None for an empty cell.
    pub fn glyph(&self, c: char) -> Option<usize> {
        if self.blank.contains(&c) {
            return None;
        }
        let mut c = if self.fold_lower {
            c.to_ascii_uppercase()
        } else {
            c
        };
        if let Some(a) = self.alias.iter().find(|a| a.0 == c) {
            c = a.1;
        }
        let i = (c as u32).checked_sub(self.first)? as usize;
        (i < self.glyphs.frames).then_some(i)
    }

    /// Draws `text` with its first cell's top left at (x, y); returns the x after the last cell.
    pub fn draw(&self, dst: &mut Bitmap, x: i32, y: i32, text: &str, mode: TextMode) -> i32 {
        let (cw, ch) = self.cell();
        let mut cx = x;
        for c in text.chars() {
            self.draw_cell(dst, cx, y, self.glyph(c), mode);
            cx += cw as i32;
        }
        let _ = ch;
        cx
    }

    /// One cell: glyph `g` (None = empty) at (x, y).
    pub fn draw_cell(&self, dst: &mut Bitmap, x: i32, y: i32, g: Option<usize>, mode: TextMode) {
        let (cw, ch) = self.cell();
        let r = g.map(|g| self.glyphs.frame(g));
        for yy in 0..ch as i32 {
            for xx in 0..cw as i32 {
                let ink = r.is_some_and(|r| self.glyphs.image.get(r.x + xx, r.y + yy) != 0);
                let (tx, ty) = (x + xx, y + yy);
                let old = dst.get(tx, ty);
                let new = match mode {
                    TextMode::Ink(c) if ink => c,
                    TextMode::Ink(_) => continue,
                    TextMode::Opaque(i, p) => {
                        if ink {
                            i
                        } else {
                            p
                        }
                    }
                    TextMode::Layer(m) => {
                        if ink {
                            old | m
                        } else {
                            old & !m
                        }
                    }
                };
                dst.set(tx, ty, new);
            }
        }
    }
}

/// What a game draws each frame: a logical-resolution picture of palette indices and the palette to show it with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub image: Bitmap,
    pub palette: Palette,
}

impl Frame {
    pub fn new(w: usize, h: usize) -> Frame {
        Frame {
            image: Bitmap::new(w, h),
            palette: Palette::new(256),
        }
    }

    /// The picture in colours, row by row.
    pub fn to_argb(&self, out: &mut Vec<Argb>) {
        out.clear();
        let lut: Vec<Argb> = (0..256).map(|i| self.palette.get(i as u8)).collect();
        out.extend(self.image.px.iter().map(|&i| lut[i as usize]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb12_expands_each_nibble() {
        assert_eq!(rgb12(0x0FFF), 0xFFFF_FFFF);
        assert_eq!(rgb12(0x0000), 0xFF00_0000);
        assert_eq!(rgb12(0x0F80), 0xFFFF_8800);
    }

    #[test]
    fn blits_clip_and_mask() {
        let mut d = Bitmap::new(8, 4);
        let s = Bitmap::from_indices(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let m = Bitmap::from_indices(3, 2, vec![1, 0, 1, 0, 1, 0]).unwrap();
        d.blit(&s, s.rect(), 6, 3, d.rect());
        assert_eq!((d.get(6, 3), d.get(7, 3)), (1, 2));
        d.blit_masked(&s, &m, s.rect(), 0, 0, d.rect());
        assert_eq!(
            (d.get(0, 0), d.get(1, 0), d.get(2, 0), d.get(1, 1)),
            (1, 0, 3, 5)
        );
        d.blit_keyed(&s, 5, s.rect(), 0, 2, Rect::new(0, 0, 2, 4));
        assert_eq!((d.get(0, 3), d.get(1, 3), d.get(2, 2)), (4, 0, 0));
    }

    #[test]
    fn lines_cover_both_ends_and_respect_clip() {
        let mut d = Bitmap::new(10, 10);
        d.line(1, 1, 8, 4, 7, d.rect());
        assert_eq!((d.get(1, 1), d.get(8, 4)), (7, 7));
        assert_eq!(d.px.iter().filter(|&&p| p == 7).count(), 8);
        d.line_with(0, 9, 0, 0, Rect::new(0, 5, 10, 5), |o| o | 0x10);
        assert_eq!((d.get(0, 9), d.get(0, 4)), (0x10, 0));
    }

    #[test]
    fn layer_text_sets_and_clears_one_bit() {
        // a 2x2 font of two glyphs: 'A' = diagonal, 'B' = full
        let g = Bitmap::from_indices(4, 2, vec![1, 0, 1, 1, 0, 1, 1, 1]).unwrap();
        let f = Font {
            glyphs: Sheet::new(g, 2, 2).unwrap(),
            first: 'A' as u32,
            fold_lower: true,
            alias: vec![('.', 'B')],
            blank: vec![' '],
        };
        let mut d = Bitmap::new(6, 2);
        d.fill(6);
        let x = f.draw(&mut d, 0, 0, "a. ", TextMode::Layer(1));
        assert_eq!(x, 6);
        assert_eq!(&d.px[..6], &[7, 6, 7, 7, 6, 6]);
        assert_eq!(&d.px[6..], &[6, 7, 7, 7, 6, 6]);
        assert_eq!(f.glyph('?'), None);
    }

    #[test]
    fn frames_convert_through_the_palette() {
        let mut f = Frame::new(2, 1);
        f.palette.colours[1] = 0xFF11_2233;
        f.image.px[1] = 1;
        let mut out = vec![];
        f.to_argb(&mut out);
        assert_eq!(out, vec![0xFF00_0000, 0xFF11_2233]);
    }
}
