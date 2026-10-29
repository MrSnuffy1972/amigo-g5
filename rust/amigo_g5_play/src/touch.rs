//! The on-screen touch controls for phones (M9 part 2, 29 Oct 2026, docs/G5-PLAYABLE.md "On-screen touch controls"):
//! a thumb stick on the left feeding the joystick, four round buttons in a diamond on the right, a pause button and,
//! when the game uses keys, a key button that pops up an Amiga-layout keyboard. Every control turns a finger into the
//! same actions a pad gives (`Host::Touch`), so the game's input goes through `Mapper` as a pad's does. Drawn on the
//! CPU in the theme's look, 40% opaque at rest and 80% while touched, and never over the middle third of the picture.

use std::f32::consts::FRAC_PI_4;
use std::path::Path;

use crate::controls::{describe, key_name, rawkey, Action, Controls, Host, Phase};
use crate::theme::{fit, Canvas, Fonts, CYAN, DIM, LIGHT, R, TEXT};

/// Opacity at rest and while touched.
const REST: u32 = 102;
const TOUCHED: u32 = 204;
/// The stick's dead zone, as a share of its radius.
const DEAD: f32 = 0.3;

/// The four buttons: `touch.buttons` when the game gives them, else fire, fire 2 and the first keys of the pad layout
/// (X, Y, B, RB, LB, RT, LT, as the pre-boot orders them), with the key `start_hint` names always among them. Fewer
/// than four when the game has fewer: an empty place is not drawn (29 Oct 2026: a made-up button would do nothing).
pub fn buttons(c: &Controls) -> Vec<(Action, String)> {
    if !c.touch_buttons.is_empty() {
        return c.touch_buttons.clone();
    }
    let mut v: Vec<Action> = vec![];
    let pad = |k: &str| c.pad.iter().find(|p| p.0 == k).map(|p| p.1);
    for a in [pad("a"), pad("b")].into_iter().flatten() {
        if matches!(a, Action::Fire | Action::Fire2) && !v.contains(&a) {
            v.push(a);
        }
    }
    for k in ["x", "y", "b", "rb", "lb", "rt", "lt"] {
        if let Some(a @ Action::Key(_)) = pad(k) {
            if !v.contains(&a) {
                v.push(a);
            }
        }
    }
    let start = c.start_hint.as_ref().and_then(|h| start_key(&h.0));
    if let Some(s) = start.filter(|s| !v[..v.len().min(4)].contains(s)) {
        v.truncate(3);
        v.push(s);
    }
    v.truncate(4);
    v.into_iter().map(|a| (a, label(a))).collect()
}

fn label(a: Action) -> String {
    match a {
        Action::Key(rk) => key_name(rk),
        a => {
            let d = describe(a);
            d[..1].to_uppercase() + &d[1..]
        }
    }
}

/// The key a start hint names ("F1 or fire: start" names F1): a function key, a named key, or one capital letter or
/// digit (so the "a" of "press a key" is not taken for the A key).
pub fn start_key(hint: &str) -> Option<Action> {
    hint.split(|c: char| c.is_whitespace() || ",:;/()".contains(c))
        .find_map(|w| {
            let n = match w {
                "Esc" => "Escape",
                "Enter" => "Return",
                "Del" => "Delete",
                "Help" => "F11",
                w => w,
            };
            let one = w.chars().count() == 1;
            let ok = !one
                || w.chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
            ok.then(|| rawkey(n)).flatten().map(Action::Key)
        })
}

/// The key button and pop-up keyboard: when `touch.keyboard` says so, or the game's controls use keys beyond the
/// four buttons.
pub fn wants_keyboard(c: &Controls) -> bool {
    let b: Vec<Action> = buttons(c).into_iter().map(|b| b.0).collect();
    let used = c.pad.iter().map(|p| p.1).chain(c.keys.iter().map(|k| k.1));
    c.touch_keyboard
        || used
            .into_iter()
            .any(|a| matches!(a, Action::Key(_)) && !b.contains(&a))
}

/// The pre-boot checkbox's default: on with a touchscreen and no game controller connected.
pub fn default_on(touchscreen: bool, pads: usize) -> bool {
    touchscreen && pads == 0
}

const STORE: &str = "onscreen-controls";

/// The player's last choice for this game, from its app storage directory.
pub fn load(dir: &Path) -> Option<bool> {
    match std::fs::read_to_string(dir.join(STORE)).ok()?.trim() {
        "on" => Some(true),
        "off" => Some(false),
        _ => None,
    }
}

pub fn save(dir: &Path, on: bool) -> std::io::Result<()> {
    std::fs::write(dir.join(STORE), if on { "on\n" } else { "off\n" })
}

/// Where the controls are on a `w` x `h` pixel screen. Sized from the shorter side so they scale with the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    /// Centre and radius of the stick.
    pub stick: (f32, f32, f32),
    /// Centre and radius of each button: bottom, right, left, top (where A, B, X and Y sit on a pad).
    pub buttons: [(f32, f32, f32); 4],
    pub pause: R,
    pub keys_button: Option<R>,
    /// The pop-up keyboard, along the bottom.
    pub keyboard: R,
}

pub fn layout(w: f32, h: f32, keyboard: bool) -> Layout {
    let u = h.min(w * 9.0 / 16.0) / 1080.0;
    let m = 40.0 * u;
    let sr = 190.0 * u;
    let (br, d) = (78.0 * u, 120.0 * u);
    let (cx, cy) = (w - m - d - br, h - m - d - br);
    let p = 96.0 * u;
    Layout {
        stick: (m + sr, h - m - sr, sr),
        buttons: [
            (cx, cy + d, br),
            (cx + d, cy, br),
            (cx - d, cy, br),
            (cx, cy - d, br),
        ],
        pause: R::new(w - m - p, m, p, p),
        keys_button: keyboard.then(|| R::new(w - m - 2.0 * p - 24.0 * u, m, p, p)),
        keyboard: R::new(0.0, h * 0.45, w, h * 0.55),
    }
}

/// The square around a circle.
pub fn bbox((x, y, r): (f32, f32, f32)) -> R {
    R::new(x - r, y - r, 2.0 * r, 2.0 * r)
}

/// The pop-up keyboard's rows: (label, Amiga raw key, width in key units), 15 units a row, as an Amiga 500 is laid
/// out (Return spans two rows there, so it is here too).
pub fn keyboard_rows() -> Vec<Vec<(String, u8, f32)>> {
    let k = |l: &str, rk: u8, w: f32| (l.to_string(), rk, w);
    let chars = |s: &str| -> Vec<(String, u8, f32)> {
        s.chars()
            .map(|c| (c.to_string(), rawkey(&c.to_string()).unwrap_or(0), 1.0))
            .collect()
    };
    let row = |a: Vec<(String, u8, f32)>, b: Vec<(String, u8, f32)>, c: Vec<(String, u8, f32)>| {
        [a, b, c].concat()
    };
    let fkeys = (1..=10).map(|n| (format!("F{n}"), 0x4F + n, 1.0)).collect();
    vec![
        row(
            vec![k("Esc", 0x45, 1.5)],
            fkeys,
            vec![k("Del", 0x46, 1.5), k("Help", 0x5F, 2.0)],
        ),
        row(vec![], chars("`1234567890-=\\"), vec![k("Bksp", 0x41, 1.0)]),
        row(
            vec![k("Tab", 0x42, 1.5)],
            chars("QWERTYUIOP[]"),
            vec![k("Return", 0x44, 1.5)],
        ),
        row(
            vec![k("Ctrl", 0x63, 2.0)],
            chars("ASDFGHJKL;'"),
            vec![k("Return", 0x44, 2.0)],
        ),
        row(
            vec![k("Shift", 0x60, 1.5)],
            chars("ZXCVBNM,./"),
            vec![
                k("Shift", 0x61, 1.5),
                k("Left", 0x4F, 1.0),
                k("Right", 0x4E, 1.0),
            ],
        ),
        vec![
            k("Alt", 0x64, 1.5),
            k("Amiga", 0x66, 1.5),
            k("Space", 0x40, 7.0),
            k("Amiga", 0x67, 1.5),
            k("Alt", 0x65, 1.5),
            k("Up", 0x4C, 1.0),
            k("Down", 0x4D, 1.0),
        ],
    ]
}

/// Each key of the keyboard in `r`: its rectangle, label and raw key.
pub fn keyboard_keys(r: R) -> Vec<(R, String, u8)> {
    let rows = keyboard_rows();
    let pad = r.h * 0.04;
    let (uw, rh) = (
        (r.w - 2.0 * pad) / 15.0,
        (r.h - 2.0 * pad) / rows.len() as f32,
    );
    let gap = rh * 0.08;
    let mut out = vec![];
    for (i, row) in rows.into_iter().enumerate() {
        let mut x = r.x + pad;
        for (l, rk, w) in row {
            let y = r.y + pad + i as f32 * rh;
            out.push((
                R::new(x + gap / 2.0, y + gap / 2.0, w * uw - gap, rh - gap),
                l,
                rk,
            ));
            x += w * uw;
        }
    }
    out
}

/// What a finger holds.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Grip {
    Stick,
    Button(usize),
    Key(usize),
    Nothing,
}

/// The on-screen controls: their layout, the fingers on them, and what those fingers hold.
pub struct TouchPad {
    pub buttons: Vec<(Action, String)>,
    pub keyboard_open: bool,
    pub lay: Layout,
    w: f32,
    h: f32,
    keys: Vec<(R, String, u8)>,
    fingers: Vec<(i64, Grip)>,
    /// Up, down, left, right, as the stick holds them.
    dirs: [bool; 4],
    knob: (f32, f32),
    fonts: Fonts,
    /// Changed since it was last drawn.
    pub dirty: bool,
}

impl TouchPad {
    pub fn new(c: &Controls, w: u32, h: u32) -> Result<TouchPad, String> {
        let kb = wants_keyboard(c);
        let lay = layout(w as f32, h as f32, kb);
        Ok(TouchPad {
            buttons: buttons(c),
            keyboard_open: false,
            keys: keyboard_keys(lay.keyboard),
            lay,
            w: w as f32,
            h: h as f32,
            fingers: vec![],
            dirs: [false; 4],
            knob: (0.0, 0.0),
            fonts: Fonts::load()?,
            dirty: true,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w as u32, self.h as u32)
    }

    /// The screen changed size (the device turned): lay out again with nothing held.
    pub fn resize(&mut self, w: u32, h: u32) {
        self.lay = layout(w as f32, h as f32, self.lay.keys_button.is_some());
        self.keys = keyboard_keys(self.lay.keyboard);
        (self.w, self.h) = (w as f32, h as f32);
        self.release_all();
    }

    /// Forgets every finger (on pause, where the mapping lets go of everything itself).
    pub fn release_all(&mut self) {
        self.fingers.clear();
        self.dirs = [false; 4];
        self.knob = (0.0, 0.0);
        self.dirty = true;
    }

    /// One finger event (position as fractions of the screen); returns the actions it holds or lets go.
    pub fn finger(&mut self, id: i64, ph: Phase, fx: f32, fy: f32) -> Vec<Host> {
        let (x, y) = (fx * self.w, fy * self.h);
        let mut out = vec![];
        let i = self.fingers.iter().position(|f| f.0 == id);
        let grip = i.map(|i| self.fingers[i].1);
        match (ph, i, grip) {
            (Phase::Down, _, _) => {
                if let Some(g) = grip {
                    self.let_go(g, &mut out);
                    self.fingers.retain(|f| f.0 != id);
                }
                let g = self.take(x, y, &mut out);
                self.fingers.push((id, g));
            }
            (Phase::Move, _, Some(Grip::Stick)) => self.aim(x, y, &mut out),
            (Phase::Move, Some(i), Some(g)) if !self.over(g, x, y) => {
                // a finger that slides off a button lets it go
                self.let_go(g, &mut out);
                self.fingers[i].1 = Grip::Nothing;
            }
            (Phase::Up, _, Some(g)) => {
                self.let_go(g, &mut out);
                self.fingers.retain(|f| f.0 != id);
            }
            _ => {}
        }
        out
    }

    fn button_at(&self, x: f32, y: f32) -> Option<usize> {
        (0..self.buttons.len()).find(|&i| self.over(Grip::Button(i), x, y))
    }

    fn over(&self, g: Grip, x: f32, y: f32) -> bool {
        let near = |(cx, cy, r): (f32, f32, f32), k: f32| (x - cx).hypot(y - cy) <= r * k;
        match g {
            Grip::Stick => near(self.lay.stick, 1.25),
            Grip::Button(i) => near(self.lay.buttons[i], 1.1),
            Grip::Key(i) => self.keys[i].0.contains(x, y),
            Grip::Nothing => false,
        }
    }

    /// What a finger put down at (x, y) takes hold of.
    fn take(&mut self, x: f32, y: f32, out: &mut Vec<Host>) -> Grip {
        if self.lay.pause.contains(x, y) {
            out.push(Host::Touch("touch:pause".into(), Action::Pause, true));
            return Grip::Nothing;
        }
        if self.lay.keys_button.is_some_and(|r| r.contains(x, y)) {
            self.keyboard_open = !self.keyboard_open;
            self.dirty = true;
            // the keyboard covers the stick and buttons: let go of whatever they held
            for f in &mut self.fingers {
                let g = std::mem::replace(&mut f.1, Grip::Nothing);
                out.extend(self_let_go(&self.buttons, &self.keys, &mut self.dirs, g));
            }
            self.knob = (0.0, 0.0);
            return Grip::Nothing;
        }
        if self.keyboard_open {
            return match (0..self.keys.len()).find(|&i| self.keys[i].0.contains(x, y)) {
                Some(i) => {
                    out.push(Host::Touch(
                        format!("touch:key{i}"),
                        Action::Key(self.keys[i].2),
                        true,
                    ));
                    self.dirty = true;
                    Grip::Key(i)
                }
                None => Grip::Nothing,
            };
        }
        if self.over(Grip::Stick, x, y) {
            self.aim(x, y, out);
            return Grip::Stick;
        }
        match self.button_at(x, y) {
            Some(i) => {
                out.push(Host::Touch(
                    format!("touch:button{i}"),
                    self.buttons[i].0,
                    true,
                ));
                self.dirty = true;
                Grip::Button(i)
            }
            None => Grip::Nothing,
        }
    }

    /// The stick held at (x, y): one of 8 directions outside the dead zone.
    fn aim(&mut self, x: f32, y: f32, out: &mut Vec<Host>) {
        let (cx, cy, r) = self.lay.stick;
        let (dx, dy) = (x - cx, y - cy);
        let d = dx.hypot(dy);
        let want = if d < r * DEAD {
            [false; 4]
        } else {
            // 0 = right, 2 = down, 4 = left, 6 = up (screen y grows downwards)
            let s = ((dy.atan2(dx) / FRAC_PI_4).round() as i32).rem_euclid(8);
            [
                (5..=7).contains(&s),
                (1..=3).contains(&s),
                (3..=5).contains(&s),
                s == 7 || s <= 1,
            ]
        };
        let k = if d > r { r / d } else { 1.0 };
        self.knob = (dx * k, dy * k);
        self.dirty = true;
        set_dirs(&mut self.dirs, want, out);
    }

    fn let_go(&mut self, g: Grip, out: &mut Vec<Host>) {
        if g == Grip::Stick {
            self.knob = (0.0, 0.0);
        }
        self.dirty = true;
        out.extend(self_let_go(&self.buttons, &self.keys, &mut self.dirs, g));
    }

    /// The controls as an overlay the size of the screen.
    pub fn render(&self) -> Canvas {
        let mut cv = Canvas::overlay(self.w as usize, self.h as usize);
        let f = &self.fonts;
        let held = |g: Grip| self.fingers.iter().any(|x| x.1 == g);
        let p = self.lay.pause;
        cv.shape(p, |_, _| true, metal);
        cv.bevel(p, true);
        for i in [0.3, 0.58] {
            cv.fill(
                R::new(p.x + p.w * i, p.y + p.h * 0.28, p.w * 0.14, p.h * 0.44),
                TEXT,
            );
        }
        cv.fade(p, REST);
        if let Some(k) = self.lay.keys_button {
            cv.shape(k, |_, _| true, metal);
            // a key icon: a keycap with rows of keys
            let cap = R::new(k.x + k.w * 0.16, k.y + k.h * 0.3, k.w * 0.68, k.h * 0.42);
            cv.outline(cap, TEXT, 2.0);
            for (row, n) in [(0.38, 5), (0.5, 5)] {
                for j in 0..n {
                    let kx = cap.x + cap.w * (0.1 + 0.17 * j as f32);
                    cv.fill(R::new(kx, k.y + k.h * row, cap.w * 0.1, k.h * 0.07), TEXT);
                }
            }
            cv.fill(
                R::new(
                    cap.x + cap.w * 0.25,
                    k.y + k.h * 0.61,
                    cap.w * 0.5,
                    k.h * 0.06,
                ),
                TEXT,
            );
            if self.keyboard_open {
                cv.outline(k, CYAN, 3.0);
            } else {
                cv.bevel(k, true);
            }
            cv.fade(k, if self.keyboard_open { TOUCHED } else { REST });
        }
        if self.keyboard_open {
            let kb = self.lay.keyboard;
            cv.shape(kb, |_, _| true, metal);
            cv.bevel(kb, true);
            for (i, (r, l, _)) in self.keys.iter().enumerate() {
                if held(Grip::Key(i)) {
                    cv.outline(*r, CYAN, 3.0);
                } else {
                    cv.bevel(*r, true);
                }
                let size = r.h * 0.42;
                let t = fit(&f.body, size, l, r.w * 0.9);
                cv.text_mid(&f.body, size, r.x + r.w / 2.0, r.y + r.h * 0.65, &t, TEXT);
            }
            cv.fade(kb, TOUCHED);
            return cv;
        }
        let (sx, sy, sr) = self.lay.stick;
        let on = held(Grip::Stick);
        cv.shape(
            bbox(self.lay.stick),
            |x, y| (x - sx).hypot(y - sy) <= sr,
            metal,
        );
        cv.ring(sx, sy, sr - 4.0, sr, if on { CYAN } else { LIGHT });
        // the dead zone, visible
        cv.ring(sx, sy, sr * DEAD - 2.0, sr * DEAD, DIM);
        let (kx, ky) = (sx + self.knob.0, sy + self.knob.1);
        cv.circle(kx, ky, sr * 0.42, 0xFF30343A);
        cv.ring(
            kx,
            ky,
            sr * 0.42 - 3.0,
            sr * 0.42,
            if on { CYAN } else { TEXT },
        );
        // the knob reaches 1.42 radii out when the stick is pushed to its edge
        cv.fade_disc(sx, sy, sr * 1.45, if on { TOUCHED } else { REST });
        for (i, (_, l)) in self.buttons.iter().enumerate() {
            let c = self.lay.buttons[i];
            let on = held(Grip::Button(i));
            cv.shape(bbox(c), |x, y| (x - c.0).hypot(y - c.1) <= c.2, metal);
            cv.ring(c.0, c.1, c.2 - 4.0, c.2, if on { CYAN } else { LIGHT });
            let size = c.2 * 0.38;
            let t = fit(&f.body, size, l, c.2 * 1.6);
            cv.text_mid(&f.body, size, c.0, c.1 + size * 0.35, &t, TEXT);
            cv.fade_disc(c.0, c.1, c.2 + 1.0, if on { TOUCHED } else { REST });
        }
        cv
    }
}

/// The releases for what grip `g` held (free of `self`, so a loop over the fingers can call it).
fn self_let_go(
    buttons: &[(Action, String)],
    keys: &[(R, String, u8)],
    dirs: &mut [bool; 4],
    g: Grip,
) -> Vec<Host> {
    let mut out = vec![];
    match g {
        Grip::Stick => set_dirs(dirs, [false; 4], &mut out),
        Grip::Button(i) => out.push(Host::Touch(format!("touch:button{i}"), buttons[i].0, false)),
        Grip::Key(i) => out.push(Host::Touch(
            format!("touch:key{i}"),
            Action::Key(keys[i].2),
            false,
        )),
        Grip::Nothing => {}
    }
    out
}

/// Holds or lets go of each stick direction that changed.
fn set_dirs(dirs: &mut [bool; 4], want: [bool; 4], out: &mut Vec<Host>) {
    let acts = [Action::Up, Action::Down, Action::Left, Action::Right];
    for i in 0..4 {
        if dirs[i] != want[i] {
            out.push(Host::Touch("touch:stick".into(), acts[i], want[i]));
        }
    }
    *dirs = want;
}

/// The controls' brushed metal, lighter at the top.
fn metal(t: f32) -> u32 {
    let v = 0x48 - (0x20 as f32 * t.clamp(0.0, 1.0)) as u32;
    0xFF00_0000 | v << 16 | v << 8 | (v + 6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::Mapper;

    fn json(s: &str) -> Controls {
        Controls::from_game_json(Some(s.as_bytes())).unwrap()
    }

    #[test]
    fn buttons_default_to_fire_then_pad_keys_with_the_start_key() {
        let b = |c: &Controls| buttons(c).into_iter().map(|b| b.1).collect::<Vec<_>>();
        assert_eq!(b(&Controls::default()), ["Fire", "Fire 2"]);
        assert!(!wants_keyboard(&Controls::default()));
        let c = json(r#"{"controls":{"pad":{"x":"key:Space","y":"key:P","rb":"key:Return"}}}"#);
        assert_eq!(b(&c), ["Fire", "Fire 2", "Space", "P"]);
        // Return is used but has no button: the keyboard is offered
        assert!(wants_keyboard(&c));
        let c = json(
            r#"{"controls":{"pad":{"x":"key:Space","y":"key:P"},
                "start_hint":{"text":"Press F1 or fire: start","status":"web"}}}"#,
        );
        assert_eq!(b(&c), ["Fire", "Fire 2", "Space", "F1"]);
        assert_eq!(start_key("press a key"), None);
        assert_eq!(start_key("Fire or Help"), Some(Action::Key(0x5F)));
        let c = json(
            r#"{"controls":{"touch":{"buttons":[{"action":"fire","label":"Shoot"},
                {"action":"key:F1","label":"Start"},{"action":"up","label":"Jump"},{"action":"pause","label":"II"}],
                "keyboard":true}}}"#,
        );
        assert_eq!(b(&c), ["Shoot", "Start", "Jump", "II"]);
        assert!(wants_keyboard(&c));
        let bad = r#"{"controls":{"touch":{"buttons":[{"action":"fire","label":"F"}]}}}"#;
        assert!(Controls::from_game_json(Some(bad.as_bytes())).is_err());
    }

    #[test]
    fn layout_keeps_clear_of_itself_and_the_middle_third() {
        for (w, h) in [(2400, 1080), (1600, 720), (1920, 1080), (1280, 720)] {
            let l = layout(w as f32, h as f32, true);
            // the diamond's squares overlap at their corners, so its buttons are checked as circles below
            let mut rs = vec![bbox(l.stick), l.pause, l.keys_button.unwrap()];
            rs.push(R::new(
                l.buttons[2].0 - l.buttons[2].2,
                l.buttons[3].1 - l.buttons[3].2,
                l.buttons[1].0 - l.buttons[2].0 + 2.0 * l.buttons[1].2,
                l.buttons[0].1 - l.buttons[3].1 + 2.0 * l.buttons[0].2,
            ));
            let pic = crate::fit_rect(w, h);
            let third = pic.width() as f32 / 3.0;
            let mid = R::new(pic.x() as f32 + third, 0.0, third, h as f32);
            let screen = R::new(0.0, 0.0, w as f32, h as f32);
            for (i, a) in rs.iter().enumerate() {
                assert!(
                    !a.overlaps(&mid),
                    "{w}x{h}: control {i} over the middle third"
                );
                assert!(
                    a.x >= 0.0 && a.y >= 0.0 && a.right() <= screen.w && a.bottom() <= screen.h
                );
                for b in &rs[i + 1..] {
                    assert!(!a.overlaps(b), "{w}x{h}: {a:?} overlaps {b:?}");
                }
            }
            for (i, a) in l.buttons.iter().enumerate() {
                for b in &l.buttons[i + 1..] {
                    assert!((a.0 - b.0).hypot(a.1 - b.1) > a.2 + b.2);
                }
            }
            let keys = keyboard_keys(l.keyboard);
            for (i, k) in keys.iter().enumerate() {
                assert!(k.0.x >= l.keyboard.x && k.0.right() <= l.keyboard.right() + 0.01);
                assert!(k.0.bottom() <= l.keyboard.bottom());
                assert!(
                    keys[i + 1..].iter().all(|o| !o.0.overlaps(&k.0)),
                    "{:?}",
                    k.1
                );
            }
        }
        let names: Vec<String> = keyboard_keys(R::new(0.0, 0.0, 1500.0, 600.0))
            .into_iter()
            .map(|k| k.1)
            .collect();
        for n in [
            "Esc", "F1", "F10", "Help", "Del", "Up", "Down", "Left", "Right", "Q", "M", "0",
            "Space", "Return",
        ] {
            assert!(names.contains(&n.to_string()), "{n}");
        }
        for n in ["Shift", "Ctrl", "Alt", "Amiga"] {
            assert!(names.contains(&n.to_string()), "{n}");
        }
        assert!(keyboard_rows()
            .iter()
            .all(|r| r.iter().map(|k| k.2).sum::<f32>() == 15.0));
    }

    #[test]
    fn fingers_drive_the_same_input_as_the_pad() {
        let c = Controls::default();
        let mut t = TouchPad::new(&c, 1920, 1080).unwrap();
        let (sx, sy, sr) = t.lay.stick;
        let (bx, by, _) = t.lay.buttons[0];
        let at = |x: f32, y: f32| (x / 1920.0, y / 1080.0);
        let mut touch = Mapper::new(c.clone());
        let feed = |t: &mut TouchPad, m: &mut Mapper, id, ph, (x, y): (f32, f32)| {
            t.finger(id, ph, x, y)
                .iter()
                .fold(false, |p, h| m.handle(h) | p)
        };
        // stick right and up (two fingers: stick and fire at once)
        feed(&mut t, &mut touch, 1, Phase::Down, at(sx + sr * 0.8, sy));
        feed(&mut t, &mut touch, 2, Phase::Down, at(bx, by));
        feed(
            &mut t,
            &mut touch,
            1,
            Phase::Move,
            at(sx + sr * 0.6, sy - sr * 0.6),
        );
        let mut pad = Mapper::new(c.clone());
        pad.handle(&Host::Pad("dpad_right".into(), true));
        pad.handle(&Host::Pad("dpad_up".into(), true));
        pad.handle(&Host::Pad("a".into(), true));
        let dbg = |i: amigo_g5::hardware::Input| format!("{i:?}");
        assert_eq!(dbg(touch.tick_input()), dbg(pad.tick_input()));
        // inside the dead zone nothing is held; a finger sliding off the button lets go
        feed(&mut t, &mut touch, 1, Phase::Move, at(sx + sr * 0.1, sy));
        feed(&mut t, &mut touch, 2, Phase::Move, at(bx, by - 300.0));
        let i = touch.tick_input();
        assert!(!i.up && !i.right && !i.fire);
        feed(&mut t, &mut touch, 2, Phase::Up, at(bx, by - 300.0));
        // down-left, then the finger lifts
        feed(&mut t, &mut touch, 1, Phase::Move, at(sx - sr, sy + sr));
        let i = touch.tick_input();
        assert!(i.down && i.left && !i.up && !i.right);
        feed(&mut t, &mut touch, 1, Phase::Up, at(sx - sr, sy + sr));
        assert_eq!(
            dbg(touch.tick_input()),
            dbg(Mapper::new(c.clone()).tick_input())
        );
        // the pause button asks for the pause menu
        let p = t.lay.pause;
        assert!(feed(
            &mut t,
            &mut touch,
            3,
            Phase::Down,
            at(p.x + 5.0, p.y + 5.0)
        ));
    }

    #[test]
    fn the_keyboard_pops_up_and_sends_amiga_keys() {
        let c = json(r#"{"controls":{"touch":{"keyboard":true}}}"#);
        let mut t = TouchPad::new(&c, 1600, 720).unwrap();
        let k = t.lay.keys_button.unwrap();
        let (sx, sy, _) = t.lay.stick;
        let n = |x: f32, y: f32| (x / 1600.0, y / 720.0);
        let s = n(sx, sy);
        assert_eq!(t.finger(1, Phase::Down, s.0, s.1).len(), 0);
        // opening the keyboard lets go of nothing here (dead zone), and hides the stick
        let kb = n(k.x + 2.0, k.y + 2.0);
        t.finger(2, Phase::Down, kb.0, kb.1);
        assert!(t.keyboard_open);
        let (r, _, rk) = t.keys.iter().find(|k| k.1 == "F1").unwrap().clone();
        let f1 = n(r.x + r.w / 2.0, r.y + r.h / 2.0);
        let mut m = Mapper::new(c.clone());
        for h in t.finger(3, Phase::Down, f1.0, f1.1) {
            m.handle(&h);
        }
        for h in t.finger(3, Phase::Up, f1.0, f1.1) {
            m.handle(&h);
        }
        assert_eq!(rk, 0x50);
        assert_eq!(m.take_keys(), vec![(0x50, true), (0x50, false)]);
    }

    #[test]
    fn overlay_is_see_through_with_controls_at_rest_and_touched() {
        let mut t = TouchPad::new(&Controls::default(), 1280, 720).unwrap();
        let cv = t.render();
        let a = |cv: &Canvas, x: f32, y: f32| cv.px[y as usize * cv.w + x as usize] >> 24;
        assert_eq!(a(&cv, 640.0, 360.0), 0);
        let (bx, by, br) = t.lay.buttons[0];
        assert_eq!(a(&cv, bx, by - br * 0.8), REST);
        t.finger(1, Phase::Down, bx / 1280.0, by / 720.0);
        assert_eq!(a(&t.render(), bx, by - br * 0.8), TOUCHED);
    }

    #[test]
    fn checkbox_default_and_storage() {
        assert!(default_on(true, 0));
        assert!(!default_on(true, 1));
        assert!(!default_on(false, 0));
        let d = std::env::temp_dir().join(format!("g5touch-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        assert_eq!(load(&d), None);
        save(&d, false).unwrap();
        assert_eq!(load(&d), Some(false));
        save(&d, true).unwrap();
        assert_eq!(load(&d), Some(true));
        let _ = std::fs::remove_dir_all(&d);
    }
}
