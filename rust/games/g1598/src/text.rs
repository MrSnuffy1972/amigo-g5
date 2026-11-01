//! Text for the native build. The original printed with the Amiga's ROM font, which is not shipped; text here is
//! drawn with the engine's bundled VT323 face (SIL OFL), rasterised once into fixed cells of palette ink.

use native_engine::gfx::Bitmap;
use std::collections::HashMap;

pub struct Text {
    font: fontdue::Font,
    size: f32,
    pub cell_w: i32,
    pub cell_h: i32,
    base: i32,
    cache: HashMap<char, (i32, i32, usize, usize, Vec<u8>)>,
}

impl Text {
    pub fn new(font: fontdue::Font, cell_w: i32, cell_h: i32) -> Text {
        let size = cell_h as f32 * 1.0;
        let lm = font.horizontal_line_metrics(size);
        let base = lm
            .map(|m| m.ascent.round() as i32)
            .unwrap_or(cell_h * 3 / 4)
            .min(cell_h - 1);
        Text {
            font,
            size,
            cell_w,
            cell_h,
            base,
            cache: HashMap::new(),
        }
    }

    fn glyph(&mut self, c: char) -> &(i32, i32, usize, usize, Vec<u8>) {
        let (font, size) = (&self.font, self.size);
        self.cache.entry(c).or_insert_with(|| {
            let (m, cov) = font.rasterize(c, size);
            (m.xmin, m.ymin + m.height as i32, m.width, m.height, cov)
        })
    }

    /// Draws `s` with its top-left at (x, y); `bg` fills each cell first when given. Returns the x after it.
    pub fn draw(
        &mut self,
        scr: &mut Bitmap,
        x: i32,
        y: i32,
        s: &str,
        ink: u8,
        bg: Option<u8>,
    ) -> i32 {
        let (cw, ch, base) = (self.cell_w, self.cell_h, self.base);
        let mut cx = x;
        for c in s.chars() {
            if let Some(b) = bg {
                for yy in 0..ch {
                    for xx in 0..cw {
                        scr.set(cx + xx, y + yy, b);
                    }
                }
            }
            let (xmin, top, w, h, cov) = self.glyph(c).clone();
            let ox = cx + xmin.max(0);
            let oy = y + base - top;
            for gy in 0..h {
                for gx in 0..w {
                    if cov[gy * w + gx] >= 110 && (gx as i32) < cw {
                        scr.set(ox + gx as i32, oy + gy as i32, ink);
                    }
                }
            }
            cx += cw;
        }
        cx
    }

    /// Centred on `cx`.
    pub fn centred(&mut self, scr: &mut Bitmap, cx: i32, y: i32, s: &str, ink: u8, bg: Option<u8>) {
        let w = s.chars().count() as i32 * self.cell_w;
        self.draw(scr, cx - w / 2, y, s, ink, bg);
    }
}
