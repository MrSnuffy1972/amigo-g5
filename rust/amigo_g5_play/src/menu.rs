//! The in-game pause menu (M7, 29 Oct 2026; Controls entry and the theme module M8): Resume, Restart, Controls, Quit
//! over the frozen game, in the pre-boot's look (theme.rs). Controls shows the pre-boot's full-screen diagram.

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::controls::Host;
use crate::theme::{self, Fonts, CYAN, R, TEXT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Resume,
    Restart,
    Controls,
    Quit,
}

const ITEMS: [(MenuItem, &str); 4] = [
    (MenuItem::Resume, "Resume"),
    (MenuItem::Restart, "Restart"),
    (MenuItem::Controls, "Controls"),
    (MenuItem::Quit, "Quit"),
];
const W: usize = 480;
const H: usize = 400;
const ROW_Y: f32 = 90.0;
const ROW_H: f32 = 70.0;

pub struct Menu<'a> {
    sel: usize,
    fonts: Fonts,
    tex: Texture<'a>,
    dirty: bool,
}

impl<'a> Menu<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut tex = tc
            .create_texture_streaming(PixelFormatEnum::ARGB8888, W as u32, H as u32)
            .map_err(|e| e.to_string())?;
        tex.set_blend_mode(BlendMode::Blend);
        Ok(Menu {
            sel: 0,
            fonts: Fonts::load()?,
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
}

/// The panel with item `sel` focused.
fn render(sel: usize, fonts: &Fonts) -> theme::Canvas {
    let mut cv = theme::Canvas::plain(W, H);
    let all = R::new(0.0, 0.0, W as f32, H as f32);
    cv.metal(all, 0x04);
    cv.bevel(all, true);
    cv.text(&fonts.title, 32.0, 32.0, 56.0, "Paused", CYAN);
    for (i, (_, label)) in ITEMS.iter().enumerate() {
        let r = row(i);
        if i == sel {
            cv.outline(r, CYAN, 2.0);
        } else {
            cv.bevel(r, false);
        }
        cv.text(&fonts.body, 28.0, 48.0, r.y + 38.0, label, TEXT);
    }
    cv
}

impl Menu<'_> {
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.dirty {
            let b: Vec<u8> = render(self.sel, &self.fonts)
                .px
                .iter()
                .flat_map(|p| p.to_ne_bytes())
                .collect();
            self.tex
                .update(None, &b, W * 4)
                .map_err(|e| e.to_string())?;
            self.dirty = false;
        }
        let (ww, wh) = canvas.output_size()?;
        canvas.copy(&self.tex, None, Some(place(ww, wh)))
    }

    /// The item under output pixel (x, y), for a click or a tap (M9 part 2: a phone has no keys to move the focus).
    pub fn hit(&mut self, ww: u32, wh: u32, x: f32, y: f32) -> Option<MenuItem> {
        let r = place(ww, wh);
        let s = r.height() as f32 / H as f32;
        let (px, py) = ((x - r.x() as f32) / s, (y - r.y() as f32) / s);
        let i = (0..ITEMS.len()).find(|&i| row(i).contains(px, py))?;
        self.sel = i;
        self.dirty = true;
        Some(ITEMS[i].0)
    }
}

/// Where the panel goes in a `ww` x `wh` window: sized for 1080 lines, scaled with the window and centred.
fn place(ww: u32, wh: u32) -> Rect {
    let s = wh as f32 / 1080.0 * 1.6;
    let (w, h) = ((W as f32 * s) as u32, (H as f32 * s) as u32);
    Rect::new(
        (ww.saturating_sub(w) / 2) as i32,
        (wh.saturating_sub(h) / 2) as i32,
        w.max(1),
        h.max(1),
    )
}

/// Item `i`'s row in panel pixels.
fn row(i: usize) -> R {
    R::new(
        24.0,
        ROW_Y + i as f32 * ROW_H,
        W as f32 - 48.0,
        ROW_H - 14.0,
    )
}

/// One host event in the menu: up and down move the selection, Enter / A choose, pause again resumes (a click or tap
/// goes to `Menu::hit`).
fn nav(sel: &mut usize, h: &Host) -> Option<MenuItem> {
    let (name, down) = match h {
        Host::Key(n, d) | Host::Pad(n, d) => (n.as_str(), *d),
        _ => return None,
    };
    if !down {
        return None;
    }
    let n = ITEMS.len();
    match name {
        "Up" | "dpad_up" => *sel = (*sel + n - 1) % n,
        "Down" | "dpad_down" => *sel = (*sel + 1) % n,
        "Return" | "Keypad Enter" | "Space" | "Select" | "a" => return Some(ITEMS[*sel].0),
        "Escape" | "AC Back" | "start" | "back" | "b" => return Some(MenuItem::Resume),
        _ => {}
    }
    None
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
        nav(&mut sel, &key("Down"));
        assert_eq!(nav(&mut sel, &key("Space")), Some(MenuItem::Controls));
        assert_eq!(nav(&mut sel, &key("AC Back")), Some(MenuItem::Resume));
    }

    #[test]
    fn panel_renders_with_labels() {
        let px = render(1, &Fonts::load().unwrap()).px;
        let plain = theme::Canvas::plain(W, H).px[0];
        assert!(px.iter().filter(|&&p| p == CYAN).count() > 200);
        assert!(px.iter().filter(|&&p| p != plain).count() > 200);
    }
}
