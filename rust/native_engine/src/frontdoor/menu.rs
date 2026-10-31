//! The in-game pause menu: Resume, Restart, Controls, Quit over the frozen game, in the pre-boot's look. Drawn on the
//! CPU as a panel the platform layer places in the middle of the screen.

use super::theme::{Canvas, Fonts, CYAN, R, TEXT};
use crate::input::Host;

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
pub const W: usize = 480;
pub const H: usize = 400;
const ROW_Y: f32 = 90.0;
const ROW_H: f32 = 70.0;

pub struct PauseMenu {
    pub sel: usize,
    fonts: Fonts,
}

impl PauseMenu {
    pub fn new() -> Result<PauseMenu, String> {
        Ok(PauseMenu {
            sel: 0,
            fonts: Fonts::load()?,
        })
    }

    pub fn open(&mut self) {
        self.sel = 0;
    }

    /// One key or pad press: up and down move, Enter / A choose, pause or back again resumes.
    pub fn handle(&mut self, h: &Host) -> Option<MenuItem> {
        let (name, down) = match h {
            Host::Key(n, d) | Host::Pad(n, d) => (n.as_str(), *d),
            _ => return None,
        };
        if !down {
            return None;
        }
        let n = ITEMS.len();
        match name {
            "Up" | "dpad_up" => self.sel = (self.sel + n - 1) % n,
            "Down" | "dpad_down" => self.sel = (self.sel + 1) % n,
            "Return" | "Keypad Enter" | "Space" | "Select" | "a" => return Some(ITEMS[self.sel].0),
            "Escape" | "AC Back" | "start" | "back" | "b" | "F10" => return Some(MenuItem::Resume),
            _ => {}
        }
        None
    }

    /// Where the panel goes on a `ww` x `wh` screen: (x, y, w, h), sized for 1080 lines and centred.
    pub fn place(ww: u32, wh: u32) -> (i32, i32, u32, u32) {
        let s = wh as f32 / 1080.0 * 1.6;
        let (w, h) = ((W as f32 * s) as u32, (H as f32 * s) as u32);
        (
            (ww.saturating_sub(w) / 2) as i32,
            (wh.saturating_sub(h) / 2) as i32,
            w.max(1),
            h.max(1),
        )
    }

    /// The item under screen pixel (x, y) of a `ww` x `wh` screen, for a click or a tap.
    pub fn hit(&mut self, ww: u32, wh: u32, x: f32, y: f32) -> Option<MenuItem> {
        let (rx, ry, _, rh) = Self::place(ww, wh);
        let s = rh as f32 / H as f32;
        let (px, py) = ((x - rx as f32) / s, (y - ry as f32) / s);
        let i = (0..ITEMS.len()).find(|&i| row(i).contains(px, py))?;
        self.sel = i;
        Some(ITEMS[i].0)
    }

    /// The panel with the selection shown.
    pub fn render(&self) -> Canvas {
        let mut cv = Canvas::plain(W, H);
        let all = R::new(0.0, 0.0, W as f32, H as f32);
        cv.metal(all, 0x04);
        cv.bevel(all, true);
        cv.text(&self.fonts.title, 32.0, 32.0, 56.0, "Paused", CYAN);
        for (i, (_, label)) in ITEMS.iter().enumerate() {
            let r = row(i);
            if i == self.sel {
                cv.outline(r, CYAN, 2.0);
            } else {
                cv.bevel(r, false);
            }
            cv.text(&self.fonts.body, 28.0, 48.0, r.y + 38.0, label, TEXT);
        }
        cv
    }
}

fn row(i: usize) -> R {
    R::new(
        24.0,
        ROW_Y + i as f32 * ROW_H,
        W as f32 - 48.0,
        ROW_H - 14.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_wraps_chooses_and_hits() {
        let mut m = PauseMenu::new().unwrap();
        let key = |n: &str| Host::Key(n.into(), true);
        assert_eq!(m.handle(&key("Up")), None);
        assert_eq!(m.handle(&key("Return")), Some(MenuItem::Quit));
        m.handle(&Host::Pad("dpad_down".into(), true));
        m.handle(&Host::Pad("dpad_down".into(), true));
        assert_eq!(
            m.handle(&Host::Pad("a".into(), true)),
            Some(MenuItem::Restart)
        );
        assert_eq!(m.handle(&key("AC Back")), Some(MenuItem::Resume));
        let (x, y, _, h) = PauseMenu::place(1920, 1080);
        let s = h as f32 / H as f32;
        let r = row(2);
        assert_eq!(
            m.hit(
                1920,
                1080,
                x as f32 + (r.x + 5.0) * s,
                y as f32 + (r.y + 5.0) * s
            ),
            Some(MenuItem::Controls)
        );
        assert!(m.render().px.iter().filter(|&&p| p == CYAN).count() > 200);
    }
}
