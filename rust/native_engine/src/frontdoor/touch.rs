//! The on-screen touch controls for phones (the G5 M9 controls, made system-neutral): a thumb stick on the left giving
//! Up, Down, Left and Right, up to four round buttons in a diamond on the right, and a pause button. In a pointer game
//! a touch anywhere else is the pointer: it places the pointer under the finger and holds the primary button. Every
//! control turns a finger into the same `Host` events a pad gives, so the game's input goes through the `Mapper`. Drawn
//! on the CPU in the theme's look, 40% opaque at rest and 80% while touched.

use std::f32::consts::FRAC_PI_4;
use std::path::Path;

use super::theme::{fit, Canvas, Fonts, CYAN, DIM, LIGHT, R, TEXT};
use super::ControlSheet;
use crate::input::{Action, Host};

const REST: u32 = 102;
const TOUCHED: u32 = 204;
const DEAD: f32 = 0.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Down,
    Move,
    Up,
}

/// The pre-boot checkbox's default: on with a touchscreen and no game controller connected.
pub fn default_on(touchscreen: bool, pads: usize) -> bool {
    touchscreen && pads == 0
}

const STORE: &str = "onscreen-controls";

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

/// Where the controls are on a `w` x `h` pixel screen, sized from the shorter side.
#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    /// Centre and radius of the stick (None in a pointer game, where the left of the screen is the picture).
    pub stick: Option<(f32, f32, f32)>,
    /// Centre and radius of each button: bottom, right, left, top.
    pub buttons: [(f32, f32, f32); 4],
    pub pause: R,
}

pub fn layout(w: f32, h: f32, stick: bool) -> Layout {
    let u = h.min(w * 9.0 / 16.0) / 1080.0;
    let m = 40.0 * u;
    let sr = 190.0 * u;
    let (br, d) = (78.0 * u, 120.0 * u);
    let (cx, cy) = (w - m - d - br, h - m - d - br);
    let p = 96.0 * u;
    Layout {
        stick: stick.then_some((m + sr, h - m - sr, sr)),
        buttons: [
            (cx, cy + d, br),
            (cx + d, cy, br),
            (cx - d, cy, br),
            (cx, cy - d, br),
        ],
        pause: R::new(w - m - p, m, p, p),
    }
}

pub fn bbox((x, y, r): (f32, f32, f32)) -> R {
    R::new(x - r, y - r, 2.0 * r, 2.0 * r)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Grip {
    Stick,
    Button(usize),
    Pointer,
    Nothing,
}

pub struct TouchPad {
    pub buttons: Vec<(Action, String)>,
    pointer: bool,
    pub lay: Layout,
    w: f32,
    h: f32,
    fingers: Vec<(i64, Grip)>,
    dirs: [bool; 4],
    knob: (f32, f32),
    fonts: Fonts,
    pub dirty: bool,
}

impl TouchPad {
    pub fn new(c: &ControlSheet, w: u32, h: u32) -> Result<TouchPad, String> {
        let mut buttons = c.touch_buttons.clone();
        buttons.truncate(4);
        Ok(TouchPad {
            buttons,
            pointer: c.touch_pointer,
            lay: layout(w as f32, h as f32, !c.touch_pointer),
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

    pub fn resize(&mut self, w: u32, h: u32) {
        self.lay = layout(w as f32, h as f32, !self.pointer);
        (self.w, self.h) = (w as f32, h as f32);
        self.release_all();
    }

    pub fn release_all(&mut self) {
        self.fingers.clear();
        self.dirs = [false; 4];
        self.knob = (0.0, 0.0);
        self.dirty = true;
    }

    /// One finger event at screen pixel (x, y); `to_logical` maps a screen pixel to the game's logical pixel (None
    /// outside the picture). Returns the events it makes.
    pub fn finger(
        &mut self,
        id: i64,
        ph: Phase,
        x: f32,
        y: f32,
        to_logical: impl Fn(f32, f32) -> Option<(i32, i32)>,
    ) -> Vec<Host> {
        let mut out = vec![];
        let i = self.fingers.iter().position(|f| f.0 == id);
        let grip = i.map(|i| self.fingers[i].1);
        match (ph, grip) {
            (Phase::Down, g) => {
                if let Some(g) = g {
                    self.let_go(g, &mut out);
                    self.fingers.retain(|f| f.0 != id);
                }
                let g = self.take(x, y, &mut out, &to_logical);
                self.fingers.push((id, g));
            }
            (Phase::Move, Some(Grip::Stick)) => self.aim(x, y, &mut out),
            (Phase::Move, Some(Grip::Pointer)) => {
                if let Some((lx, ly)) = to_logical(x, y) {
                    out.push(Host::PointerAt(lx, ly));
                }
            }
            (Phase::Move, Some(g @ Grip::Button(_))) if !self.over(g, x, y) => {
                self.let_go(g, &mut out);
                if let Some(i) = i {
                    self.fingers[i].1 = Grip::Nothing;
                }
            }
            (Phase::Up, Some(g)) => {
                self.let_go(g, &mut out);
                self.fingers.retain(|f| f.0 != id);
            }
            _ => {}
        }
        out
    }

    fn over(&self, g: Grip, x: f32, y: f32) -> bool {
        let near = |(cx, cy, r): (f32, f32, f32), k: f32| (x - cx).hypot(y - cy) <= r * k;
        match g {
            Grip::Stick => self.lay.stick.is_some_and(|s| near(s, 1.25)),
            Grip::Button(i) => near(self.lay.buttons[i], 1.1),
            _ => false,
        }
    }

    fn take(
        &mut self,
        x: f32,
        y: f32,
        out: &mut Vec<Host>,
        to_logical: &impl Fn(f32, f32) -> Option<(i32, i32)>,
    ) -> Grip {
        if self.lay.pause.contains(x, y) {
            out.push(Host::Touch("touch:pause".into(), Action::Pause, true));
            return Grip::Nothing;
        }
        if self.over(Grip::Stick, x, y) {
            self.aim(x, y, out);
            return Grip::Stick;
        }
        if let Some(i) = (0..self.buttons.len()).find(|&i| self.over(Grip::Button(i), x, y)) {
            out.push(Host::Touch(
                format!("touch:button{i}"),
                self.buttons[i].0,
                true,
            ));
            self.dirty = true;
            return Grip::Button(i);
        }
        if self.pointer {
            if let Some((lx, ly)) = to_logical(x, y) {
                out.push(Host::PointerAt(lx, ly));
                out.push(Host::Touch("touch:pointer".into(), Action::Primary, true));
                return Grip::Pointer;
            }
        }
        Grip::Nothing
    }

    fn aim(&mut self, x: f32, y: f32, out: &mut Vec<Host>) {
        let Some((cx, cy, r)) = self.lay.stick else {
            return;
        };
        let (dx, dy) = (x - cx, y - cy);
        let d = dx.hypot(dy);
        let want = if d < r * DEAD {
            [false; 4]
        } else {
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
        self.dirty = true;
        match g {
            Grip::Stick => {
                self.knob = (0.0, 0.0);
                set_dirs(&mut self.dirs, [false; 4], out);
            }
            Grip::Button(i) => out.push(Host::Touch(
                format!("touch:button{i}"),
                self.buttons[i].0,
                false,
            )),
            Grip::Pointer => out.push(Host::Touch("touch:pointer".into(), Action::Primary, false)),
            Grip::Nothing => {}
        }
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
        if let Some((sx, sy, sr)) = self.lay.stick {
            let on = held(Grip::Stick);
            cv.shape(
                bbox((sx, sy, sr)),
                |x, y| (x - sx).hypot(y - sy) <= sr,
                metal,
            );
            cv.ring(sx, sy, sr - 4.0, sr, if on { CYAN } else { LIGHT });
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
            cv.fade_disc(sx, sy, sr * 1.45, if on { TOUCHED } else { REST });
        }
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

fn set_dirs(dirs: &mut [bool; 4], want: [bool; 4], out: &mut Vec<Host>) {
    let acts = [Action::Up, Action::Down, Action::Left, Action::Right];
    for i in 0..4 {
        if dirs[i] != want[i] {
            out.push(Host::Touch("touch:stick".into(), acts[i], want[i]));
        }
    }
    *dirs = want;
}

fn metal(t: f32) -> u32 {
    let v = 0x48 - (0x20 as f32 * t.clamp(0.0, 1.0)) as u32;
    0xFF00_0000 | v << 16 | v << 8 | (v + 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet(pointer: bool) -> ControlSheet {
        ControlSheet {
            touch_buttons: vec![
                (Action::Secondary, "Right".into()),
                (Action::Cancel, "Esc".into()),
            ],
            touch_pointer: pointer,
            ..Default::default()
        }
    }

    #[test]
    fn pointer_game_touches_place_the_pointer_and_click() {
        let mut t = TouchPad::new(&sheet(true), 1920, 1080).unwrap();
        assert!(t.lay.stick.is_none());
        let map = |x: f32, y: f32| Some((x as i32 / 4, y as i32 / 4));
        let e = t.finger(1, Phase::Down, 400.0, 200.0, map);
        assert_eq!(
            e,
            vec![
                Host::PointerAt(100, 50),
                Host::Touch("touch:pointer".into(), Action::Primary, true)
            ]
        );
        assert_eq!(
            t.finger(1, Phase::Move, 404.0, 208.0, map),
            vec![Host::PointerAt(101, 52)]
        );
        assert_eq!(
            t.finger(1, Phase::Up, 404.0, 208.0, map),
            vec![Host::Touch("touch:pointer".into(), Action::Primary, false)]
        );
        let b = t.lay.buttons[0];
        let e = t.finger(2, Phase::Down, b.0, b.1, map);
        assert_eq!(
            e,
            vec![Host::Touch("touch:button0".into(), Action::Secondary, true)]
        );
        let p = t.lay.pause;
        assert_eq!(
            t.finger(3, Phase::Down, p.x + 2.0, p.y + 2.0, map)[0],
            Host::Touch("touch:pause".into(), Action::Pause, true)
        );
        assert!(t.render().px.iter().any(|&p| p >> 24 != 0));
    }

    #[test]
    fn stick_gives_directions_in_a_joystick_game() {
        let mut t = TouchPad::new(&sheet(false), 1920, 1080).unwrap();
        let (sx, sy, sr) = t.lay.stick.unwrap();
        let e = t.finger(1, Phase::Down, sx + sr, sy, |_, _| None);
        assert_eq!(
            e,
            vec![Host::Touch("touch:stick".into(), Action::Right, true)]
        );
        let e = t.finger(1, Phase::Up, sx + sr, sy, |_, _| None);
        assert_eq!(
            e,
            vec![Host::Touch("touch:stick".into(), Action::Right, false)]
        );
        assert!(default_on(true, 0) && !default_on(true, 1));
    }
}
