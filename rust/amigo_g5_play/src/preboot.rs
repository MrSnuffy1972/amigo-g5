//! The pre-boot menu (M8, 29 Oct 2026, docs/G5-PLAYABLE.md "The pre-boot menu"; Karl's standing rule of 14 Oct): shown
//! at every start, a 30 second countdown that starts the game by itself, paused by any input other than start and
//! resumed after 10 s idle, the controller and keyboard panels from the controls data, and Start game, Controls and
//! Quit by pad, keys, mouse and TV remote. The same Controls screen serves the in-game pause menu.

use crate::controls::{Controls, Host};
use crate::pad::{chip, draw_keyboard_panel, draw_pad_panel, panel};
use crate::theme::{
    measure, png, Canvas, Fonts, AMIGA_BLUE, CHIP, CYAN, DARK, DESIGN_W, LIGHT, MAGENTA, ORANGE, R,
    SOFT, TEXT,
};

pub const COUNTDOWN_MS: u32 = 30_000;
pub const RESUME_IDLE_MS: u32 = 10_000;

/// The countdown: runs down unless paused; a pause ends after RESUME_IDLE_MS with no input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Countdown {
    pub left_ms: u32,
    /// Milliseconds idle since the last input, while paused.
    pub paused: Option<u32>,
}

impl Default for Countdown {
    fn default() -> Self {
        Countdown {
            left_ms: COUNTDOWN_MS,
            paused: None,
        }
    }
}

impl Countdown {
    pub fn touch(&mut self) {
        self.paused = Some(0);
    }

    /// Advances by `ms`; true when the countdown has run out.
    pub fn advance(&mut self, ms: u32) -> bool {
        match self.paused {
            Some(idle) if idle + ms < RESUME_IDLE_MS => self.paused = Some(idle + ms),
            Some(_) => self.paused = None,
            None => self.left_ms = self.left_ms.saturating_sub(ms),
        }
        self.left_ms == 0
    }

    /// Whole seconds left, rounded up (30 at the start, 1 in the last second).
    pub fn seconds(&self) -> u32 {
        self.left_ms.div_ceil(1000)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Start,
    Controls,
    Quit,
}

const ITEMS: [(Item, &str); 3] = [
    (Item::Start, "Start game"),
    (Item::Controls, "Controls"),
    (Item::Quit, "Quit"),
];

/// What the player did, in the menu's terms. Mouse positions are in design units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Nav {
    Prev,
    Next,
    Choose,
    Start,
    Back,
    Point(f32, f32),
    Click(f32, f32),
    Other,
}

/// A key or pad press as menu navigation: arrows and D-pad move, Enter / Space / A / the TV remote's centre (SDL's
/// Select) choose, Start starts, Escape / B / Back / the remote's Back go back. Releases and small stick moves are
/// nothing; any other press is `Other` (it still pauses the countdown).
pub fn nav_of(h: &Host) -> Option<Nav> {
    match h {
        Host::Key(n, true) | Host::Pad(n, true) => Some(match n.as_str() {
            "Up" | "Left" | "dpad_up" | "dpad_left" => Nav::Prev,
            "Down" | "Right" | "Tab" | "dpad_down" | "dpad_right" => Nav::Next,
            "Return" | "Keypad Enter" | "Space" | "Select" | "a" => Nav::Choose,
            "start" => Nav::Start,
            "Escape" | "AC Back" | "back" | "b" => Nav::Back,
            _ => Nav::Other,
        }),
        Host::Axis(_, x, y) if x.abs() > 16_000 || y.abs() > 16_000 => Some(Nav::Other),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Controls,
}

/// The menu screen's panels (controller, keyboard) and the Controls screen's, in design units.
pub const MENU_PANELS: (R, R) = (
    R::new(60.0, 250.0, 1300.0, 610.0),
    R::new(1400.0, 250.0, 460.0, 610.0),
);
pub const CONTROLS_PANELS: (R, R) = (
    R::new(60.0, 170.0, 1300.0, 830.0),
    R::new(1400.0, 170.0, 460.0, 830.0),
);
const BAR: R = R::new(560.0, 906.0, 800.0, 18.0);
/// The "On-screen controls" checkbox (Android only, M9 part 2), right of the countdown bar: focus item 3.
pub const CHECK: R = R::new(1420.0, 868.0, 440.0, 60.0);

fn button(i: usize) -> R {
    R::new(410.0 + i as f32 * 380.0, 950.0, 340.0, 80.0)
}

pub struct Preboot {
    fonts: Fonts,
    controls: Controls,
    title: String,
    pub sel: usize,
    pub countdown: Countdown,
    pub screen: Screen,
    /// The "On-screen controls" checkbox: None hides it (every build but Android, unless `--touch` is given).
    pub touch: Option<bool>,
    /// The screen's unchanging layer at its size, so a frame redraws only the buttons and the countdown.
    base: Option<(usize, usize, Screen, Vec<u32>)>,
}

impl Preboot {
    pub fn new(title: &str, controls: &Controls) -> Result<Preboot, String> {
        Ok(Preboot {
            fonts: Fonts::load()?,
            controls: controls.clone(),
            title: title.into(),
            sel: 0,
            countdown: Countdown::default(),
            screen: Screen::Menu,
            touch: None,
            base: None,
        })
    }

    /// One input; `Some(Start)` or `Some(Quit)` ends the pre-boot.
    pub fn handle(&mut self, n: Nav) -> Option<Item> {
        if n == Nav::Start {
            return Some(Item::Start);
        }
        // a mouse merely passing over the window is not a player reading the screen: only hovering a button counts
        if !matches!(n, Nav::Point(..)) {
            self.countdown.touch();
        }
        if self.screen == Screen::Controls {
            if matches!(n, Nav::Choose | Nav::Back | Nav::Click(..)) {
                self.screen = Screen::Menu;
            }
            return None;
        }
        let count = ITEMS.len() + self.touch.is_some() as usize;
        let check = self.touch.is_some().then_some(CHECK);
        let hit = |x, y| {
            (0..ITEMS.len())
                .find(|&i| button(i).contains(x, y))
                .or(check.filter(|c| c.contains(x, y)).map(|_| ITEMS.len()))
        };
        // the checkbox (focus item 3) toggles where a button would be chosen
        let pick = |p: &mut Preboot, i: usize| match ITEMS.get(i) {
            Some(it) => Some(it.0),
            None => {
                p.touch = p.touch.map(|t| !t);
                None
            }
        };
        let chosen = match n {
            Nav::Prev => {
                self.sel = (self.sel + count - 1) % count;
                None
            }
            Nav::Next => {
                self.sel = (self.sel + 1) % count;
                None
            }
            Nav::Choose => {
                let i = self.sel;
                pick(self, i)
            }
            Nav::Back => Some(Item::Quit),
            Nav::Point(x, y) => {
                if let Some(i) = hit(x, y) {
                    self.sel = i;
                    self.countdown.touch();
                }
                None
            }
            Nav::Click(x, y) => hit(x, y).and_then(|i| pick(self, i)),
            _ => None,
        };
        if chosen == Some(Item::Controls) {
            self.screen = Screen::Controls;
            return None;
        }
        chosen
    }

    /// Advances the clock by `ms`; `Some(Start)` when the countdown runs out on the menu.
    pub fn advance(&mut self, ms: u32) -> Option<Item> {
        (self.countdown.advance(ms) && self.screen == Screen::Menu).then_some(Item::Start)
    }

    /// The screen at `w` x `h` pixels.
    pub fn render(&mut self, w: usize, h: usize) -> Canvas {
        let mut cv = Canvas::new(w, h);
        match &self.base {
            Some((bw, bh, s, px)) if (*bw, *bh, *s) == (w, h, self.screen) => cv.px.clone_from(px),
            _ => {
                self.draw_base(&mut cv);
                self.base = Some((w, h, self.screen, cv.px.clone()));
            }
        }
        if self.screen == Screen::Menu {
            self.draw_live(&mut cv);
        }
        cv
    }

    /// The design point under device pixel (x, y) of a `w` x `h` screen.
    pub fn to_design(w: usize, h: usize, x: f32, y: f32) -> (f32, f32) {
        Canvas::new(w.max(1), h.max(1)).to_design(x, y)
    }

    fn draw_base(&self, cv: &mut Canvas) {
        let f = &self.fonts;
        cv.metal(R::new(-1e4, -1e4, 2e4, 2e4), 0);
        title_bar(cv, f);
        let (panels, heading_y) = match self.screen {
            Screen::Menu => (MENU_PANELS, 170.0),
            Screen::Controls => (CONTROLS_PANELS, 132.0),
        };
        let heading = match self.screen {
            Screen::Menu => self.title.clone(),
            Screen::Controls => format!("Controls: {}", self.title),
        };
        let size = if self.screen == Screen::Menu {
            76.0
        } else {
            52.0
        };
        cv.text_mid(&f.title, size, DESIGN_W / 2.0, heading_y, &heading, TEXT);
        if self.screen == Screen::Menu {
            if let Some((hint, status)) = &self.controls.start_hint {
                let w = measure(&f.body, 30.0, hint);
                let x = DESIGN_W / 2.0 - w / 2.0;
                cv.text(&f.body, 30.0, x, 218.0, hint, SOFT);
                if let Some(ch) = chip(Some(status)) {
                    cv.text(&f.mono, 24.0, x + w + 14.0, 218.0, ch, CHIP);
                }
            }
        } else {
            let hint = "Back, B or A: return";
            cv.text_mid(&f.mono, 32.0, DESIGN_W / 2.0, 1050.0, hint, SOFT);
        }
        let pad = panel(cv, panels.0, "Controller", f);
        draw_pad_panel(cv, pad, &self.controls, f);
        let kb = panel(cv, panels.1, "Keyboard and mouse", f);
        draw_keyboard_panel(cv, kb, &self.controls, f);
    }

    /// The countdown and the three buttons, which change from frame to frame.
    fn draw_live(&self, cv: &mut Canvas) {
        let f = &self.fonts;
        let c = &self.countdown;
        let line = match c.paused {
            Some(_) => format!("PAUSED  {} s  (resumes when idle)", c.seconds()),
            None => format!("Game starts in {} s", c.seconds()),
        };
        let col = if c.paused.is_some() { MAGENTA } else { ORANGE };
        cv.text_mid(&f.mono, 36.0, DESIGN_W / 2.0, 894.0, &line, col);
        cv.fill(BAR, DARK);
        cv.bevel(BAR, false);
        let frac = c.left_ms as f32 / COUNTDOWN_MS as f32;
        cv.fill(
            R::new(BAR.x + 2.0, BAR.y + 2.0, (BAR.w - 4.0) * frac, BAR.h - 4.0),
            col,
        );
        for (i, (_, name)) in ITEMS.iter().enumerate() {
            let r = button(i);
            let on = i == self.sel;
            if on {
                cv.glow(r, CYAN, 22.0);
            }
            cv.metal(r, if on { 0x1C } else { 0x0C });
            if on {
                cv.outline(r, CYAN, 3.0);
            } else {
                cv.bevel(r, true);
            }
            cv.text_mid(&f.body, 32.0, r.x + r.w / 2.0, r.y + 52.0, name, TEXT);
        }
        if let Some(on) = self.touch {
            let focus = self.sel == ITEMS.len();
            if focus {
                cv.glow(CHECK, CYAN, 16.0);
                cv.outline(CHECK, CYAN, 2.0);
            }
            let b = R::new(CHECK.x + 14.0, CHECK.y + 12.0, 36.0, 36.0);
            cv.fill(b, DARK);
            cv.bevel(b, false);
            if on {
                cv.line((b.x + 8.0, b.y + 19.0), (b.x + 15.0, b.y + 27.0), 5.0, CYAN);
                cv.line((b.x + 15.0, b.y + 27.0), (b.x + 29.0, b.y + 9.0), 5.0, CYAN);
            }
            let label = "On-screen controls";
            cv.text(&f.body, 30.0, b.right() + 16.0, CHECK.y + 41.0, label, TEXT);
        }
    }
}

/// The Workbench-style title bar: the "Ami-Go G5" badge, drag-bar stripes, the build line and two gadgets.
fn title_bar(cv: &mut Canvas, f: &Fonts) {
    let bar = R::new(0.0, 0.0, DESIGN_W, 56.0);
    cv.metal(bar, 0x14);
    cv.bevel(bar, true);
    let badge = R::new(16.0, 9.0, 220.0, 38.0);
    cv.fill(badge, AMIGA_BLUE);
    cv.bevel(badge, true);
    cv.text_mid(
        &f.mono,
        34.0,
        badge.x + badge.w / 2.0,
        38.0,
        "Ami-Go G5",
        TEXT,
    );
    let build = format!(
        "rev {}  {}",
        option_env!("AMIGO_G5_REV").unwrap_or("dev"),
        option_env!("AMIGO_G5_BUILD_DATE").unwrap_or("")
    );
    let bw = measure(&f.mono, 30.0, &build);
    let stripes_end = 1780.0 - bw - 30.0;
    for i in 0..5 {
        let y = 14.0 + i as f32 * 6.0;
        cv.fill(R::new(260.0, y, stripes_end - 260.0, 2.0), LIGHT);
        cv.fill(R::new(260.0, y + 2.0, stripes_end - 260.0, 2.0), DARK);
    }
    cv.text(&f.mono, 30.0, 1780.0 - bw, 38.0, &build, CYAN);
    for x in [1800.0, 1856.0] {
        let g = R::new(x, 9.0, 44.0, 38.0);
        cv.bevel(g, true);
        cv.bevel(R::new(x + 12.0, 18.0, 20.0, 20.0), false);
    }
}

/// `--preboot-shot`: the menu `at_s` seconds into the countdown, as a 1920x1080 PNG. Drawn on the CPU, so it needs
/// no display (and no SDL at all).
pub fn shot(
    title: &str,
    c: &Controls,
    touch: Option<bool>,
    at_s: f32,
    out: &std::path::Path,
) -> Result<(), String> {
    let mut p = Preboot::new(title, c)?;
    p.touch = touch;
    p.countdown.advance((at_s * 1000.0) as u32);
    let cv = p.render(1920, 1080);
    std::fs::write(out, png(cv.w, cv.h, &cv.px)).map_err(|e| format!("{}: {e}", out.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_starts_the_game_after_30_s_untouched() {
        let mut c = Countdown::default();
        assert_eq!(c.seconds(), 30);
        for _ in 0..1499 {
            assert!(!c.advance(20));
        }
        assert_eq!(c.seconds(), 1);
        assert!(c.advance(20));
    }

    #[test]
    fn input_pauses_the_countdown_until_10_s_idle() {
        let mut c = Countdown::default();
        c.advance(5_000);
        c.touch();
        c.advance(9_000);
        assert_eq!((c.left_ms, c.paused), (25_000, Some(9_000)));
        // more input restarts the idle wait
        c.touch();
        c.advance(9_999);
        assert!(c.paused.is_some());
        c.advance(1);
        assert_eq!(c.paused, None);
        c.advance(1_000);
        assert_eq!(c.left_ms, 24_000);
    }

    #[test]
    fn menu_navigation_by_pad_keys_mouse_and_remote() {
        let mut p = Preboot::new("Test", &Controls::default()).unwrap();
        let key = |n: &str| nav_of(&Host::Key(n.into(), true)).unwrap();
        let pad = |n: &str| nav_of(&Host::Pad(n.into(), true)).unwrap();
        assert_eq!(nav_of(&Host::Key("Return".into(), false)), None);
        assert_eq!(nav_of(&Host::Axis("left_stick".into(), 100, 0)), None);
        // any other press pauses the countdown
        assert_eq!(p.handle(key("F5")), None);
        assert!(p.countdown.paused.is_some());
        p.handle(pad("dpad_down"));
        assert_eq!(p.handle(key("Select")), None);
        assert_eq!(p.screen, Screen::Controls);
        // the countdown never starts the game under the Controls screen
        p.countdown.left_ms = 0;
        p.countdown.paused = None;
        assert_eq!(p.advance(20), None);
        p.handle(key("AC Back"));
        assert_eq!(p.screen, Screen::Menu);
        assert_eq!(p.advance(20), Some(Item::Start));
        p.handle(key("Up"));
        assert_eq!(p.sel, 0);
        assert_eq!(p.handle(pad("a")), Some(Item::Start));
        assert_eq!(p.handle(pad("start")), Some(Item::Start));
        assert_eq!(p.handle(key("Escape")), Some(Item::Quit));
        let q = button(2);
        p.handle(Nav::Point(q.x + 5.0, q.y + 5.0));
        assert_eq!(p.sel, 2);
        assert_eq!(p.handle(Nav::Click(q.x + 5.0, q.y + 5.0)), Some(Item::Quit));
        assert_eq!(p.handle(Nav::Click(5.0, 5.0)), None);
    }

    #[test]
    fn touch_checkbox_is_a_fourth_item_only_when_shown() {
        let mut p = Preboot::new("Test", &Controls::default()).unwrap();
        p.handle(Nav::Prev);
        assert_eq!(p.sel, 2);
        assert_eq!(p.handle(Nav::Click(CHECK.x + 5.0, CHECK.y + 5.0)), None);
        assert_eq!(p.touch, None);
        p.touch = Some(true);
        p.handle(Nav::Next);
        assert_eq!(p.sel, 3);
        assert_eq!(p.handle(Nav::Choose), None);
        assert_eq!(p.touch, Some(false));
        p.handle(Nav::Next);
        assert_eq!(p.sel, 0);
        assert_eq!(p.handle(Nav::Click(CHECK.x + 5.0, CHECK.y + 5.0)), None);
        assert_eq!(p.touch, Some(true));
        p.handle(Nav::Point(CHECK.x + 5.0, CHECK.y + 5.0));
        assert_eq!(p.sel, 3);
        // the box is drawn, ticked in cyan, and gone when hidden
        let shown = p.render(1920, 1080);
        let tick = |cv: &Canvas| {
            let r = R::new(CHECK.x + 14.0, CHECK.y + 12.0, 36.0, 36.0);
            (r.y as usize..r.bottom() as usize)
                .flat_map(|y| (r.x as usize..r.right() as usize).map(move |x| (x, y)))
                .filter(|&(x, y)| cv.px[y * cv.w + x] == CYAN)
                .count()
        };
        assert!(tick(&shown) > 50);
        p.touch = None;
        assert_eq!(tick(&p.render(1920, 1080)), 0);
    }

    #[test]
    fn screens_render_with_glow_and_countdown_bar() {
        let j = br#"{"controls":{"start_hint":{"text":"F1 or fire: start","status":"web"}}}"#;
        let c = Controls::from_game_json(Some(j)).unwrap();
        let mut p = Preboot::new("Synthetic", &c).unwrap();
        p.countdown.advance(15_000);
        let cv = p.render(1280, 720);
        let at = |x: f32, y: f32| {
            let (s, ox) = (cv.s, (cv.w as f32 - DESIGN_W * cv.s) / 2.0);
            cv.px[(y * s) as usize * cv.w + (x * s + ox) as usize]
        };
        // half the bar is orange, the rest empty
        assert_eq!(at(BAR.x + BAR.w * 0.25, BAR.y + 9.0), ORANGE);
        assert_ne!(at(BAR.x + BAR.w * 0.75, BAR.y + 9.0), ORANGE);
        // the focused button glows cyan-ish just outside its edge
        let g = at(button(0).x - 4.0, button(0).y + 40.0);
        assert!((g & 0xFF) > ((g >> 16) & 0xFF) + 20, "{g:08X}");
        let distinct: std::collections::HashSet<_> = cv.px.iter().collect();
        assert!(distinct.len() > 500);
        p.screen = Screen::Controls;
        assert_ne!(p.render(1280, 720).px, cv.px);
    }
}
