//! The front door's look (the LGP metallic-and-neon theme of the G5 pre-boot, M8, carried over unchanged by native
//! stage 1): colours, the three bundled fonts and the drawing helpers every screen uses (brushed metal, bevels, neon
//! glow, text, leader lines and arrows).
//! Screens are laid out in a 1920x1080 design space and drawn on the CPU at the real size, scaled and centred, so a
//! screenshot needs no display and the pre-boot looks the same on a 720p phone and a 4K TV.

use fontdue::{Font, FontSettings};

pub const CYAN: u32 = 0xFF00E5FF;
pub const MAGENTA: u32 = 0xFFFF2BD6;
pub const AMIGA_BLUE: u32 = 0xFF0055AA;
pub const ORANGE: u32 = 0xFFFF8800;
pub const TEXT: u32 = 0xFFE8ECF0;
pub const DIM: u32 = 0xFF5C626C;
/// Secondary text (hints).
pub const SOFT: u32 = 0xFFA0A6B0;
pub const LIGHT: u32 = 0xFF5A606A;
pub const DARK: u32 = 0xFF050608;
/// The chip beside a mapping that is not confirmed: dim magenta.
pub const CHIP: u32 = 0xFFB0409A;

pub const DESIGN_W: f32 = 1920.0;
pub const DESIGN_H: f32 = 1080.0;

/// The fonts, all SIL Open Font Licence 1.1, licence text beside each in rust/assets/fonts. Bundled so the exe and
/// the APK need no files.
const CINZEL: &[u8] = include_bytes!("../../../assets/fonts/Cinzel-Variable.ttf");
const VT323: &[u8] = include_bytes!("../../../assets/fonts/VT323-Regular.ttf");
const INTER: &[u8] = include_bytes!("../../../assets/fonts/Inter-Variable.ttf");

pub struct Fonts {
    /// Cinzel: the game title and headings.
    pub title: Font,
    /// VT323: the Amiga touch (countdown, version line, chips).
    pub mono: Font,
    /// Inter: body text and mapping labels.
    pub body: Font,
}

impl Fonts {
    pub fn load() -> Result<Fonts, String> {
        let f = |b| Font::from_bytes(b, FontSettings::default()).map_err(String::from);
        Ok(Fonts {
            title: f(CINZEL)?,
            mono: f(VT323)?,
            body: f(INTER)?,
        })
    }
}

/// A rectangle in design units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl R {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> R {
        R { x, y, w, h }
    }
    pub fn right(&self) -> f32 {
        self.x + self.w
    }
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }
    pub fn overlaps(&self, o: &R) -> bool {
        self.x < o.right() && o.x < self.right() && self.y < o.bottom() && o.y < self.bottom()
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

/// Width of `s` at `size` px.
pub fn measure(f: &Font, size: f32, s: &str) -> f32 {
    s.chars().map(|c| f.metrics(c, size).advance_width).sum()
}

/// `s`, cut with an ellipsis to fit `w` at `size`.
pub fn fit(f: &Font, size: f32, s: &str, w: f32) -> String {
    if measure(f, size, s) <= w {
        return s.to_string();
    }
    let mut t: String = s.into();
    while !t.is_empty() && measure(f, size, &format!("{t}...")) > w {
        t.pop();
    }
    format!("{}...", t.trim_end())
}

/// An ARGB picture with the design-to-device transform.
pub struct Canvas {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u32>,
    pub s: f32,
    ox: f32,
    oy: f32,
}

impl Canvas {
    /// A `w` x `h` picture showing the 1920x1080 design space scaled to fit and centred.
    pub fn new(w: usize, h: usize) -> Canvas {
        let s = (w as f32 / DESIGN_W).min(h as f32 / DESIGN_H);
        let (ox, oy) = (
            (w as f32 - DESIGN_W * s) / 2.0,
            (h as f32 - DESIGN_H * s) / 2.0,
        );
        Canvas {
            w,
            h,
            px: vec![DARK; w * h],
            s,
            ox,
            oy,
        }
    }

    /// A picture whose design units are its pixels (the pause panel).
    pub fn plain(w: usize, h: usize) -> Canvas {
        Canvas {
            s: 1.0,
            ox: 0.0,
            oy: 0.0,
            ..Canvas::new(w, h)
        }
    }

    /// A see-through picture whose design units are its pixels (the on-screen touch controls, drawn over the game).
    pub fn overlay(w: usize, h: usize) -> Canvas {
        Canvas {
            px: vec![0; w * h],
            ..Canvas::plain(w, h)
        }
    }

    /// Device pixel to design units (for the mouse).
    pub fn to_design(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.ox) / self.s, (y - self.oy) / self.s)
    }

    fn dev(&self, x: f32, y: f32) -> (f32, f32) {
        (x * self.s + self.ox, y * self.s + self.oy)
    }

    /// The device pixels covering `r`, clipped.
    fn span(&self, r: R) -> (usize, usize, usize, usize) {
        let (x0, y0) = self.dev(r.x, r.y);
        let (x1, y1) = self.dev(r.right(), r.bottom());
        let c = |v: f32, m: usize| (v.round().max(0.0) as usize).min(m);
        (c(x0, self.w), c(y0, self.h), c(x1, self.w), c(y1, self.h))
    }

    /// Blends `col` (its alpha times `a`/255) over one device pixel.
    fn put(&mut self, x: usize, y: usize, col: u32, a: u32) {
        let a = a * (col >> 24) / 255;
        let d = &mut self.px[y * self.w + x];
        let da = *d >> 24;
        if da == 255 {
            let mix = |sh: u32| {
                ((((col >> sh) & 0xFF) * a + ((*d >> sh) & 0xFF) * (255 - a)) / 255) << sh
            };
            *d = 0xFF00_0000 | mix(16) | mix(8) | mix(0);
            return;
        }
        // over a see-through pixel (an overlay): the usual "over" with straight alpha
        let keep = da * (255 - a) / 255;
        let oa = a + keep;
        if oa > 0 {
            let mix =
                |sh: u32| ((((col >> sh) & 0xFF) * a + ((*d >> sh) & 0xFF) * keep) / oa) << sh;
            *d = oa << 24 | mix(16) | mix(8) | mix(0);
        }
    }

    /// Scales the opacity of everything in `r` by `a`/255 (an on-screen control at rest or touched).
    pub fn fade(&mut self, r: R, a: u32) {
        let (x0, y0, x1, y1) = self.span(r);
        for y in y0..y1 {
            for p in &mut self.px[y * self.w + x0..y * self.w + x1] {
                *p = ((*p >> 24) * a / 255) << 24 | (*p & 0xFF_FFFF);
            }
        }
    }

    pub fn fill(&mut self, r: R, col: u32) {
        let (x0, y0, x1, y1) = self.span(r);
        for y in y0..y1 {
            for x in x0..x1 {
                self.put(x, y, col, 255);
            }
        }
    }

    /// Fills the pixels of `r` whose centre (in design units) is `inside`, coloured by how far down `r` they are.
    pub fn shape(&mut self, r: R, inside: impl Fn(f32, f32) -> bool, col: impl Fn(f32) -> u32) {
        let (x0, y0, x1, y1) = self.span(r);
        for y in y0..y1 {
            for x in x0..x1 {
                let (dx, dy) = self.to_design(x as f32 + 0.5, y as f32 + 0.5);
                if inside(dx, dy) {
                    self.put(x, y, col((dy - r.y) / r.h), 255);
                }
            }
        }
    }

    pub fn circle(&mut self, cx: f32, cy: f32, rad: f32, col: u32) {
        let r = R::new(cx - rad, cy - rad, rad * 2.0, rad * 2.0);
        self.shape(
            r,
            |x, y| (x - cx).powi(2) + (y - cy).powi(2) <= rad * rad,
            |_| col,
        );
    }

    /// `fade` for the disc of radius `rad` only (round controls whose squares overlap).
    pub fn fade_disc(&mut self, cx: f32, cy: f32, rad: f32, a: u32) {
        let (x0, y0, x1, y1) = self.span(R::new(cx - rad, cy - rad, 2.0 * rad, 2.0 * rad));
        for y in y0..y1 {
            for x in x0..x1 {
                let (dx, dy) = self.to_design(x as f32 + 0.5, y as f32 + 0.5);
                if (dx - cx).hypot(dy - cy) <= rad {
                    let p = &mut self.px[y * self.w + x];
                    *p = ((*p >> 24) * a / 255) << 24 | (*p & 0xFF_FFFF);
                }
            }
        }
    }

    /// A ring between radii `r0` and `r1`.
    pub fn ring(&mut self, cx: f32, cy: f32, r0: f32, r1: f32, col: u32) {
        let r = R::new(cx - r1, cy - r1, r1 * 2.0, r1 * 2.0);
        self.shape(
            r,
            |x, y| (r0 * r0..=r1 * r1).contains(&((x - cx).powi(2) + (y - cy).powi(2))),
            |_| col,
        );
    }

    /// Brushed metal: near-black, a subtle vertical gradient and fine horizontal striping. `lift` brightens it
    /// (raised parts, the controller body).
    pub fn metal(&mut self, r: R, lift: u32) {
        let (x0, y0, x1, y1) = self.span(r);
        let span = (y1 - y0).max(1) as u32;
        for y in y0..y1 {
            // a fixed hash of the row, so the striping is the same every frame
            let n = (y as u32).wrapping_mul(2_654_435_761) >> 29;
            let v = lift + 0x26 - 0x10 * (y - y0) as u32 / span + n;
            let c = 0xFF00_0000 | v << 16 | v << 8 | (v + 5);
            self.px[y * self.w + x0..y * self.w + x1].fill(c);
        }
    }

    /// A Workbench bevel: light top and left, dark bottom and right when `raised`, the other way when recessed.
    pub fn bevel(&mut self, r: R, raised: bool) {
        let (hi, lo) = if raised { (LIGHT, DARK) } else { (DARK, LIGHT) };
        self.edges(r, hi, lo, 2.0);
    }

    pub fn outline(&mut self, r: R, col: u32, t: f32) {
        self.edges(r, col, col, t);
    }

    fn edges(&mut self, r: R, hi: u32, lo: u32, t: f32) {
        let t = (t * self.s).max(1.0) / self.s;
        self.fill(R::new(r.x, r.y, r.w, t), hi);
        self.fill(R::new(r.x, r.y + t, t, r.h - t), hi);
        self.fill(R::new(r.x + t, r.bottom() - t, r.w - t, t), lo);
        self.fill(R::new(r.right() - t, r.y + t, t, r.h - 2.0 * t), lo);
    }

    /// A soft neon glow outside `r`, fading over `rad` design units; the inside is left alone so text stays crisp.
    pub fn glow(&mut self, r: R, col: u32, rad: f32) {
        let g = R::new(r.x - rad, r.y - rad, r.w + 2.0 * rad, r.h + 2.0 * rad);
        let (x0, y0, x1, y1) = self.span(g);
        for y in y0..y1 {
            for x in x0..x1 {
                let (dx, dy) = self.to_design(x as f32 + 0.5, y as f32 + 0.5);
                let ex = (r.x - dx).max(dx - r.right()).max(0.0);
                let ey = (r.y - dy).max(dy - r.bottom()).max(0.0);
                let d = (ex * ex + ey * ey).sqrt() / rad;
                if d > 0.0 && d < 1.0 {
                    self.put(x, y, col, (150.0 * (1.0 - d).powi(2)) as u32);
                }
            }
        }
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32), t: f32, col: u32) {
        let n = ((b.0 - a.0).hypot(b.1 - a.1) * self.s).ceil().max(1.0) as usize;
        for i in 0..=n {
            let f = i as f32 / n as f32;
            let (x, y) = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            self.fill(R::new(x - t / 2.0, y - t / 2.0, t, t), col);
        }
    }

    /// A leader line from `a` with an arrow head at `b`.
    pub fn arrow(&mut self, a: (f32, f32), b: (f32, f32), col: u32) {
        let len = (b.0 - a.0).hypot(b.1 - a.1).max(1.0);
        let (ux, uy) = ((b.0 - a.0) / len, (b.1 - a.1) / len);
        let (hl, hw) = (16.0, 7.0);
        let base = (b.0 - ux * hl, b.1 - uy * hl);
        self.line(a, base, 2.0, col);
        let p = [
            b,
            (base.0 - uy * hw, base.1 + ux * hw),
            (base.0 + uy * hw, base.1 - ux * hw),
        ];
        let side = |p: (f32, f32), q: (f32, f32), x: f32, y: f32| {
            (q.0 - p.0) * (y - p.1) - (q.1 - p.1) * (x - p.0)
        };
        let bx = R::new(
            b.0 - hl - hw,
            b.1 - hl - hw,
            2.0 * (hl + hw),
            2.0 * (hl + hw),
        );
        self.shape(
            bx,
            |x, y| {
                let s = [
                    side(p[0], p[1], x, y),
                    side(p[1], p[2], x, y),
                    side(p[2], p[0], x, y),
                ];
                s.iter().all(|&v| v >= 0.0) || s.iter().all(|&v| v <= 0.0)
            },
            |_| col,
        );
    }

    /// `s` at `size` with its baseline at `base`, starting at `x`; returns its width.
    pub fn text(&mut self, f: &Font, size: f32, x: f32, base: f32, s: &str, col: u32) -> f32 {
        let (mut pen, by) = self.dev(x, base);
        for c in s.chars() {
            let (m, cov) = f.rasterize(c, size * self.s);
            let top = by.round() as i32 - m.height as i32 - m.ymin;
            for (k, &a) in cov.iter().enumerate() {
                let gx = pen.round() as i32 + m.xmin + (k % m.width.max(1)) as i32;
                let gy = top + (k / m.width.max(1)) as i32;
                if a > 0 && gx >= 0 && gy >= 0 && (gx as usize) < self.w && (gy as usize) < self.h {
                    self.put(gx as usize, gy as usize, col, a as u32);
                }
            }
            pen += m.advance_width;
        }
        (pen - self.dev(x, base).0) / self.s
    }

    /// `s` centred on `cx`.
    pub fn text_mid(&mut self, f: &Font, size: f32, cx: f32, base: f32, s: &str, col: u32) {
        let w = measure(f, size * self.s, s) / self.s;
        self.text(f, size, cx - w / 2.0, base, s, col);
    }
}

/// The picture as a PNG: 8-bit RGB, deflate's stored blocks (no compression, so no dependency for a review shot).
pub fn png(w: usize, h: usize, px: &[u32]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(h * (w * 3 + 1));
    for row in px.chunks(w) {
        raw.push(0);
        raw.extend(
            row.iter()
                .flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8]),
        );
    }
    let mut z = vec![0x78, 0x01];
    let blocks: Vec<&[u8]> = raw.chunks(65_535).collect();
    for (i, b) in blocks.iter().enumerate() {
        z.push((i + 1 == blocks.len()) as u8);
        z.extend((b.len() as u16).to_le_bytes());
        z.extend((!(b.len() as u16)).to_le_bytes());
        z.extend_from_slice(b);
    }
    z.extend(adler32(&raw).to_be_bytes());
    let mut ihdr = (w as u32).to_be_bytes().to_vec();
    ihdr.extend((h as u32).to_be_bytes());
    ihdr.extend([8, 2, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    for (kind, data) in [(&b"IHDR"[..], ihdr), (b"IDAT", z), (b"IEND", vec![])] {
        out.extend((data.len() as u32).to_be_bytes());
        let start = out.len();
        out.extend_from_slice(kind);
        out.extend(&data);
        let c = crc32(&out[start..]);
        out.extend(c.to_be_bytes());
    }
    out
}

pub fn crc32(b: &[u8]) -> u32 {
    let mut c = !0u32;
    for &x in b {
        c ^= x as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xEDB8_8320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
    }
    !c
}

pub fn adler32(b: &[u8]) -> u32 {
    let (mut a, mut s) = (1u32, 0u32);
    for &x in b {
        a = (a + x as u32) % 65_521;
        s = (s + a) % 65_521;
    }
    s << 16 | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums_match_their_standard_check_values() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
    }

    #[test]
    fn design_space_scales_and_centres() {
        let c = Canvas::new(1280, 720);
        assert_eq!(c.to_design(640.0, 360.0), (960.0, 540.0));
        // a tall window: the design is centred with bars above and below
        let c = Canvas::new(1920, 1440);
        assert_eq!(c.to_design(0.0, 180.0), (0.0, 0.0));
    }

    #[test]
    fn text_is_cut_to_fit() {
        let f = Fonts::load().unwrap();
        let s = fit(
            &f.body,
            28.0,
            "A very long label that cannot fit here",
            200.0,
        );
        assert!(s.ends_with("...") && measure(&f.body, 28.0, &s) <= 200.0);
        assert_eq!(fit(&f.body, 28.0, "Fire", 200.0), "Fire");
    }
}
