//! The in-game pause menu (M7, 29 Oct 2026): Resume, Restart, Quit over the frozen game. A first cut of the LGP look
//! (near-black metal panel, Workbench bevels, cyan focus outline, Inter labels) drawn on the CPU; M8 moves the
//! colours, bevels and glow into the one theme module and adds the Controls screen.

use fontdue::{Font, FontSettings};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::controls::Host;

/// Inter (SIL Open Font Licence 1.1, rust/assets/fonts/Inter-OFL.txt), bundled so the exe needs no files.
const INTER: &[u8] = include_bytes!("../../assets/fonts/Inter-Variable.ttf");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Resume,
    Restart,
    Quit,
}

const ITEMS: [(MenuItem, &str); 3] = [
    (MenuItem::Resume, "Resume"),
    (MenuItem::Restart, "Restart"),
    (MenuItem::Quit, "Quit"),
];
const W: usize = 480;
const H: usize = 330;
const ROW_Y: usize = 90;
const ROW_H: usize = 70;

const METAL: u32 = 0xF01A1D22;
const LIGHT: u32 = 0xFF5A606A;
const DARK: u32 = 0xFF050608;
const CYAN: u32 = 0xFF00E5FF;
const TEXT: u32 = 0xFFE8ECF0;

pub struct Menu<'a> {
    sel: usize,
    font: Font,
    tex: Texture<'a>,
    dirty: bool,
}

impl<'a> Menu<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let font = Font::from_bytes(INTER, FontSettings::default())?;
        let mut tex = tc
            .create_texture_streaming(PixelFormatEnum::ARGB8888, W as u32, H as u32)
            .map_err(|e| e.to_string())?;
        tex.set_blend_mode(BlendMode::Blend);
        Ok(Menu {
            sel: 0,
            font,
            tex,
            dirty: true,
        })
    }

    pub fn open(&mut self) {
        self.sel = 0;
        self.dirty = true;
    }

    pub fn handle(&mut self, h: &Host) -> Option<MenuItem> {
        self.dirty = true;
        nav(&mut self.sel, h)
    }

    fn render(&self) -> Vec<u32> {
        let mut px = vec![METAL; W * H];
        bevel(&mut px, 0, 0, W, H, LIGHT, DARK);
        text(&self.font, &mut px, "Paused", 30.0, 32, 52, CYAN);
        for (i, (_, label)) in ITEMS.iter().enumerate() {
            let y = ROW_Y + i * ROW_H;
            let (hi, lo) = if i == self.sel {
                (CYAN, CYAN)
            } else {
                (DARK, LIGHT)
            };
            bevel(&mut px, 24, y, W - 48, ROW_H - 14, hi, lo);
            text(&self.font, &mut px, label, 28.0, 48, y + 40, TEXT);
        }
        px
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.dirty {
            let b: Vec<u8> = self.render().iter().flat_map(|p| p.to_ne_bytes()).collect();
            self.tex
                .update(None, &b, W * 4)
                .map_err(|e| e.to_string())?;
            self.dirty = false;
        }
        let (ww, wh) = canvas.output_size()?;
        // the panel is sized for 1080 lines and scaled with the window
        let s = wh as f32 / 1080.0 * 1.6;
        let (w, h) = ((W as f32 * s) as u32, (H as f32 * s) as u32);
        let r = Rect::new(
            (ww.saturating_sub(w) / 2) as i32,
            (wh.saturating_sub(h) / 2) as i32,
            w,
            h,
        );
        canvas.copy(&self.tex, None, Some(r))
    }
}

/// One host event in the menu: up and down move the selection, fire / Enter / A choose, pause again resumes.
fn nav(sel: &mut usize, h: &Host) -> Option<MenuItem> {
    let (name, down) = match h {
        Host::Key(n, d) | Host::Pad(n, d) => (n.as_str(), *d),
        Host::MouseButton(true, d) => ("mouse_left", *d),
        _ => return None,
    };
    if !down {
        return None;
    }
    let n = ITEMS.len();
    match name {
        "Up" | "dpad_up" => *sel = (*sel + n - 1) % n,
        "Down" | "dpad_down" => *sel = (*sel + 1) % n,
        "Return" | "Keypad Enter" | "Space" | "Select" | "a" | "mouse_left" => {
            return Some(ITEMS[*sel].0)
        }
        "Escape" | "AC Back" | "start" | "back" | "b" => return Some(MenuItem::Resume),
        _ => {}
    }
    None
}

/// A Workbench-style bevel: `hi` on the top and left edges, `lo` on the bottom and right, 2 px.
fn bevel(px: &mut [u32], x: usize, y: usize, w: usize, h: usize, hi: u32, lo: u32) {
    for d in 0..2 {
        for i in x..x + w {
            px[(y + d) * W + i] = hi;
            px[(y + h - 1 - d) * W + i] = lo;
        }
        for j in y..y + h {
            px[j * W + x + d] = hi;
            px[j * W + x + w - 1 - d] = lo;
        }
    }
}

/// `s` in Inter at `size` px with its baseline at `base`, blended over `px`.
fn text(font: &Font, px: &mut [u32], s: &str, size: f32, x: usize, base: usize, col: u32) {
    let mut pen = x as f32;
    for c in s.chars() {
        let (m, cov) = font.rasterize(c, size);
        let top = base as i32 - m.height as i32 - m.ymin;
        for (k, &a) in cov.iter().enumerate() {
            let (gx, gy) = (
                pen as i32 + m.xmin + (k % m.width.max(1)) as i32,
                top + (k / m.width.max(1)) as i32,
            );
            if a == 0 || gx < 0 || gy < 0 || gx as usize >= W || gy as usize >= H {
                continue;
            }
            let d = &mut px[gy as usize * W + gx as usize];
            let mix = |sh: u32| {
                let (f, b) = ((col >> sh) & 0xFF, (*d >> sh) & 0xFF);
                ((f * a as u32 + b * (255 - a as u32)) / 255) << sh
            };
            *d = (*d & 0xFF00_0000) | mix(16) | mix(8) | mix(0);
        }
        pen += m.advance_width;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_wraps_and_chooses() {
        let key = |n: &str| Host::Key(n.into(), true);
        let mut sel = 0;
        assert_eq!(nav(&mut sel, &key("Up")), None);
        assert_eq!(nav(&mut sel, &Host::Key("Return".into(), false)), None);
        assert_eq!(nav(&mut sel, &key("Return")), Some(MenuItem::Quit));
        nav(&mut sel, &Host::Pad("dpad_down".into(), true));
        nav(&mut sel, &Host::Pad("dpad_down".into(), true));
        assert_eq!(
            nav(&mut sel, &Host::Pad("a".into(), true)),
            Some(MenuItem::Restart)
        );
        assert_eq!(nav(&mut sel, &key("AC Back")), Some(MenuItem::Resume));
    }

    #[test]
    fn panel_renders_with_labels() {
        let font = Font::from_bytes(INTER, FontSettings::default()).unwrap();
        let mut px = vec![METAL; W * H];
        text(&font, &mut px, "Restart", 28.0, 48, 200, TEXT);
        assert!(px.iter().filter(|&&p| p != METAL).count() > 200);
    }
}
