//! The controls panels of the pre-boot (M8, 29 Oct 2026, docs/G5-PLAYABLE.md "The pre-boot menu"): an Xbox-style
//! pad drawn in code (vector shapes only, no copied artwork, no logos) with a leader line and arrow from each mapped
//! control to a label naming what it does in this game, and the keyboard and mouse list. Both are read from the
//! controls data; a control with no action is drawn dimmed and gets no label.

use crate::controls::{describe, Action, Controls};
use crate::theme::{fit, measure, Canvas, Fonts, CHIP, CYAN, DIM, MAGENTA, R, TEXT};

/// Each control's place on the pad, in pad units (the pad is 1000 x 640), and the side its label goes.
const PARTS: [(&str, &str, f32, f32, bool); 15] = [
    ("lt", "LT", 265.0, 28.0, true),
    ("lb", "LB", 270.0, 85.0, true),
    ("left_stick", "Left stick", 300.0, 240.0, true),
    ("ls", "Left stick click", 300.0, 280.0, true),
    ("back", "Back", 430.0, 240.0, true),
    ("dpad", "D-pad", 400.0, 380.0, true),
    ("rt", "RT", 735.0, 28.0, false),
    ("rb", "RB", 730.0, 85.0, false),
    ("y", "Y", 720.0, 175.0, false),
    ("start", "Start", 570.0, 240.0, false),
    ("x", "X", 655.0, 215.0, false),
    ("b", "B", 785.0, 240.0, false),
    ("a", "A", 720.0, 305.0, false),
    ("right_stick", "Right stick", 600.0, 380.0, false),
    ("rs", "Right stick click", 600.0, 415.0, false),
];

const LABEL: f32 = 28.0;
const CHIP_SIZE: f32 = 24.0;
const LABEL_H: f32 = 34.0;
const CHIP_H: f32 = 22.0;
const GAP: f32 = 6.0;

/// One label: its text box (design units), the chip under it, and the control it points at.
#[derive(Debug, Clone)]
pub struct Label {
    pub control: &'static str,
    pub text: String,
    pub chip: Option<&'static str>,
    pub r: R,
    pub anchor: (f32, f32),
    pub left: bool,
    /// The chip goes under the text, or (when a side is too crowded for that) after it on the same line.
    pub chip_below: bool,
}

/// The chip for a label's status: none when confirmed (or when nothing says otherwise for a default mapping).
pub fn chip(status: Option<&str>) -> Option<&'static str> {
    Some(match status? {
        "confirmed" => return None,
        "manual" => "from manual",
        "web" => "from web",
        "code" => "from game code",
        _ => "unconfirmed",
    })
}

fn find<'a>(v: &'a [(String, String)], k: &str) -> Option<&'a str> {
    v.iter().find(|p| p.0 == k).map(|p| p.1.as_str())
}

/// The pad's rectangle in `panel` and its labels, measured at the canvas scale `s`. Labels on each side are kept in
/// their anchors' order, pushed apart so none overlap, and kept inside the panel.
pub fn layout(panel: R, c: &Controls, f: &Fonts, s: f32) -> (R, Vec<Label>) {
    let pw = (panel.w * 0.44).min(panel.h * 0.9 * 1000.0 / 640.0);
    let pad = R::new(
        panel.x + (panel.w - pw) / 2.0,
        panel.y + (panel.h - pw * 0.64) / 2.0,
        pw,
        pw * 0.64,
    );
    let mut out = Vec::new();
    for left in [true, false] {
        let mut side = Vec::new();
        for chip_below in [true, false] {
            side = side_labels(c, f, s, pad, pad.x - panel.x - 50.0, left, chip_below);
            let need: f32 = side.iter().map(|l| l.r.h + GAP).sum();
            if need <= panel.h {
                break;
            }
        }
        side.sort_by(|a, b| a.anchor.1.total_cmp(&b.anchor.1));
        let mut top = panel.y;
        for l in side.iter_mut() {
            l.r.y = l.r.y.max(top);
            top = l.r.bottom() + GAP;
        }
        let mut bottom = panel.bottom();
        for l in side.iter_mut().rev() {
            l.r.y = l.r.y.min(bottom - l.r.h);
            bottom = l.r.y - GAP;
        }
        out.extend(side);
    }
    (pad, out)
}

/// One side's labels, unplaced (each beside its anchor).
fn side_labels(
    c: &Controls,
    f: &Fonts,
    s: f32,
    pad: R,
    col_w: f32,
    left: bool,
    below: bool,
) -> Vec<Label> {
    let k = pad.w / 1000.0;
    PARTS
        .iter()
        .filter(|p| p.4 == left)
        .filter_map(|&(ctl, name, u, v, _)| {
            let a = c.pad.iter().find(|p| p.0 == ctl)?.1;
            if a == Action::None {
                return None;
            }
            let what = find(&c.labels, ctl).map_or_else(|| describe(a), String::from);
            let chip = chip(find(&c.status, ctl));
            let cw = match chip {
                Some(ch) if !below => measure(&f.mono, CHIP_SIZE * s, ch) / s + 12.0,
                _ => 0.0,
            };
            let text = fit(
                &f.body,
                LABEL * s,
                &format!("{name}: {what}"),
                (col_w - cw) * s,
            );
            let w = measure(&f.body, LABEL * s, &text) / s + cw;
            let h = LABEL_H + if chip.is_some() && below { CHIP_H } else { 0.0 };
            let anchor = (pad.x + u * k, pad.y + v * k);
            let x = if left {
                pad.x - 50.0 - w
            } else {
                pad.right() + 50.0
            };
            Some(Label {
                control: ctl,
                text,
                chip,
                r: R::new(x, anchor.1 - h / 2.0, w, h),
                anchor,
                left,
                chip_below: below,
            })
        })
        .collect()
}

/// Draws the controller panel's contents in `panel`.
pub fn draw_pad_panel(cv: &mut Canvas, panel: R, c: &Controls, f: &Fonts) {
    let (pad, labels) = layout(panel, c, f, cv.s);
    draw_pad(cv, pad, c, f);
    for l in &labels {
        let y = l.r.y + LABEL_H * 0.5;
        let from = if l.left {
            (l.r.right() + 8.0, y)
        } else {
            (l.r.x - 8.0, y)
        };
        cv.arrow(from, l.anchor, CYAN & 0xC0FF_FFFF);
        cv.text(&f.body, LABEL, l.r.x, l.r.y + 26.0, &l.text, TEXT);
        match l.chip {
            Some(ch) if l.chip_below => {
                cv.text(&f.mono, CHIP_SIZE, l.r.x, l.r.y + LABEL_H + 16.0, ch, CHIP)
            }
            Some(ch) => {
                let cw = measure(&f.mono, CHIP_SIZE * cv.s, ch) / cv.s;
                cv.text(&f.mono, CHIP_SIZE, l.r.right() - cw, l.r.y + 26.0, ch, CHIP)
            }
            None => 0.0,
        };
    }
}

/// The pad itself: body, bumpers, triggers, sticks, D-pad, face buttons, Back and Start.
fn draw_pad(cv: &mut Canvas, pad: R, c: &Controls, f: &Fonts) {
    let k = pad.w / 1000.0;
    let p = |u: f32, v: f32| (pad.x + u * k, pad.y + v * k);
    let mapped = |ctl: &str| c.pad.iter().any(|q| q.0 == ctl && q.1 != Action::None);
    let ink = |ctl: &str| if mapped(ctl) { TEXT } else { DIM };
    for (ctl, u, v, w, h) in [
        ("lt", 200.0, 0.0, 130.0, 56.0),
        ("rt", 670.0, 0.0, 130.0, 56.0),
        ("lb", 170.0, 60.0, 200.0, 50.0),
        ("rb", 630.0, 60.0, 200.0, 50.0),
    ] {
        let (x, y) = p(u, v);
        let r = R::new(x, y, w * k, h * k);
        cv.metal(r, if mapped(ctl) { 0x18 } else { 0x04 });
        cv.bevel(r, true);
    }
    // the body: a wide oval with two grips, outlined by drawing it slightly larger first
    let body = |grow: f32| {
        move |x: f32, y: f32| {
            let (u, v) = ((x - pad.x) / k, (y - pad.y) / k);
            let e = |cx: f32, cy: f32, rx: f32, ry: f32| {
                ((u - cx) / (rx + grow)).powi(2) + ((v - cy) / (ry + grow)).powi(2) <= 1.0
            };
            e(500.0, 230.0, 400.0, 160.0)
                || e(255.0, 400.0, 150.0, 210.0)
                || e(745.0, 400.0, 150.0, 210.0)
        }
    };
    let whole = R::new(pad.x, pad.y + 60.0 * k, pad.w, pad.h);
    cv.shape(whole, body(5.0), |_| CYAN & 0x90FF_FFFF);
    cv.shape(whole, body(0.0), |t| {
        let v = 0x40 - (t * 40.0) as u32;
        0xFF00_0000 | v << 16 | v << 8 | (v + 6)
    });
    for (ctl, u, v) in [("left_stick", 300.0, 240.0), ("right_stick", 600.0, 380.0)] {
        let (x, y) = p(u, v);
        cv.circle(x, y, 62.0 * k, 0xFF0C0D10);
        cv.circle(
            x,
            y,
            44.0 * k,
            if mapped(ctl) { 0xFF4A505A } else { 0xFF262A30 },
        );
        cv.circle(x, y, 30.0 * k, ink(ctl) & 0x40FF_FFFF);
    }
    let (dx, dy) = p(400.0, 380.0);
    let arm = 120.0 * k;
    let t = 42.0 * k;
    let dcol = if mapped("dpad") {
        0xFF4A505A
    } else {
        0xFF262A30
    };
    cv.fill(R::new(dx - arm / 2.0, dy - t / 2.0, arm, t), dcol);
    cv.fill(R::new(dx - t / 2.0, dy - arm / 2.0, t, arm), dcol);
    // face buttons with their letters, softly tinted as on the common pads
    for (ctl, u, v, tint) in [
        ("y", 720.0, 175.0, 0xFF8A7A20),
        ("x", 655.0, 240.0, 0xFF20508A),
        ("b", 785.0, 240.0, 0xFF8A2828),
        ("a", 720.0, 305.0, 0xFF287A30),
    ] {
        let (x, y) = p(u, v);
        cv.circle(x, y, 31.0 * k, if mapped(ctl) { tint } else { 0xFF2A2D33 });
        let size = 34.0 * k;
        let name = ctl.to_ascii_uppercase();
        cv.text_mid(&f.body, size, x, y + size * 0.36, &name, ink(ctl));
    }
    for (ctl, u) in [("back", 430.0), ("start", 570.0)] {
        let (x, y) = p(u, 240.0);
        cv.circle(x, y, 17.0 * k, ink(ctl));
        cv.circle(x, y, 12.0 * k, 0xFF1A1C20);
    }
}

/// The keyboard and mouse rows: (text, chip).
pub fn keyboard_rows(c: &Controls) -> Vec<(String, Option<&'static str>)> {
    let mut rows = Vec::new();
    let arrows = ["Up", "Down", "Left", "Right"];
    let dirs = [Action::Up, Action::Down, Action::Left, Action::Right];
    let is_arrows = arrows
        .iter()
        .zip(dirs)
        .all(|(k, d)| c.keys.iter().any(|x| x.0 == *k && x.1 == d));
    if is_arrows {
        rows.push(("Arrow keys: joystick".to_string(), None));
    }
    for (k, a) in &c.keys {
        // the TV remote's Back is not a key on the keyboard
        if *a == Action::None || k == "AC Back" || (is_arrows && arrows.contains(&k.as_str())) {
            continue;
        }
        let what = find(&c.labels, k).map_or_else(|| describe(*a), String::from);
        rows.push((format!("{k}: {what}"), chip(find(&c.status, k))));
    }
    if c.passthrough {
        rows.push(("Other keys: to the game".into(), None));
    }
    if c.mouse {
        rows.push(("Mouse: the Amiga mouse".into(), None));
    }
    rows
}

/// Draws the keyboard and mouse rows in `panel`, as many as fit.
pub fn draw_keyboard_panel(cv: &mut Canvas, panel: R, c: &Controls, f: &Fonts) {
    let mut y = panel.y;
    for (text, ch) in keyboard_rows(c) {
        let h = LABEL_H + if ch.is_some() { CHIP_H } else { 0.0 };
        if y + h > panel.bottom() {
            break;
        }
        let t = fit(&f.body, LABEL * cv.s, &text, panel.w * cv.s);
        cv.text(&f.body, LABEL, panel.x, y + 26.0, &t, TEXT);
        if let Some(ch) = ch {
            cv.text(&f.mono, CHIP_SIZE, panel.x, y + LABEL_H + 16.0, ch, CHIP);
        }
        y += h + GAP * 2.0;
    }
}

/// A panel: recessed metal with a heading in neon magenta; returns the inside for the contents.
pub fn panel(cv: &mut Canvas, r: R, heading: &str, f: &Fonts) -> R {
    cv.metal(r, 0x02);
    cv.bevel(r, false);
    cv.text(&f.title, 34.0, r.x + 24.0, r.y + 46.0, heading, MAGENTA);
    inner(r)
}

/// The inside of a panel, below its heading.
pub fn inner(r: R) -> R {
    R::new(r.x + 24.0, r.y + 70.0, r.w - 48.0, r.h - 90.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preboot::{CONTROLS_PANELS, MENU_PANELS};

    /// Every pad control mapped, with long labels and a chip each: the most crowded the panel can get.
    fn crowded() -> Controls {
        let mut c = Controls::default();
        for p in c.pad.iter_mut() {
            if p.1 == Action::None {
                p.1 = Action::Key(0x50);
            }
            c.labels
                .push((p.0.into(), "a long description of what it does".into()));
            c.status.push((p.0.into(), "unconfirmed".into()));
        }
        c
    }

    #[test]
    fn labels_never_overlap_at_1080p_and_720p() {
        let f = Fonts::load().unwrap();
        for c in [Controls::default(), crowded()] {
            for (w, h) in [(1920, 1080), (1280, 720)] {
                let s = Canvas::new(w, h).s;
                for panels in [MENU_PANELS, CONTROLS_PANELS] {
                    let inner = inner(panels.0);
                    let (pad, ls) = layout(inner, &c, &f, s);
                    let n = c.pad.iter().filter(|p| p.1 != Action::None).count();
                    assert_eq!(ls.len(), n);
                    for (i, a) in ls.iter().enumerate() {
                        assert!(!a.r.overlaps(&pad), "{} over the pad", a.control);
                        assert!(a.r.y >= inner.y && a.r.bottom() <= inner.bottom() + 0.01);
                        assert!(a.r.x >= inner.x && a.r.right() <= inner.right());
                        for b in &ls[i + 1..] {
                            assert!(
                                !a.r.overlaps(&b.r),
                                "{} {} at {w}x{h}",
                                a.control,
                                b.control
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn labels_come_from_the_controls_data() {
        let f = Fonts::load().unwrap();
        let j = br#"{"controls":{"pad":{"x":"key:F1"},"labels":{"a":"Jump"},"label_status":{"a":"manual"}}}"#;
        let c = Controls::from_game_json(Some(j)).unwrap();
        let (_, ls) = layout(MENU_PANELS.0, &c, &f, 1.0);
        let get = |k: &str| ls.iter().find(|l| l.control == k).unwrap();
        assert_eq!(get("a").text, "A: Jump");
        assert_eq!(get("a").chip, Some("from manual"));
        assert_eq!(get("x").text, "X: key F1");
        assert_eq!(get("start").chip, None);
        assert!(ls.iter().all(|l| l.control != "y"));
        let rows = keyboard_rows(&Controls::default());
        assert_eq!(rows[0].0, "Arrow keys: joystick");
        assert!(rows.iter().any(|r| r.0 == "Right Ctrl: fire"));
        assert!(rows.iter().all(|r| !r.0.starts_with("AC Back")));
    }
}
