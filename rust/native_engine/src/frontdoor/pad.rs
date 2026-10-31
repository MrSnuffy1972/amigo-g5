//! The controls panels of the pre-boot: a pad drawn in code (vector shapes only, no copied artwork, no logos) with a
//! leader line and arrow from each mapped control to a label naming what it does in this game, and the keyboard and
//! mouse list. Both come from the game's `ControlSheet`; a control with no action is drawn dimmed and has no label.

use super::theme::{fit, measure, Canvas, Fonts, CYAN, DIM, MAGENTA, R, TEXT};
use super::ControlSheet;

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
const LABEL_H: f32 = 34.0;
const GAP: f32 = 6.0;

/// One label: its text box (design units) and the control it points at.
#[derive(Debug, Clone)]
pub struct Label {
    pub control: &'static str,
    pub text: String,
    pub r: R,
    pub anchor: (f32, f32),
    pub left: bool,
}

/// The pad's rectangle in `panel` and its labels, measured at the canvas scale `s`. Labels on each side keep their
/// anchors' order, are pushed apart so none overlap, and stay inside the panel.
pub fn layout(panel: R, c: &ControlSheet, f: &Fonts, s: f32) -> (R, Vec<Label>) {
    let pw = (panel.w * 0.44).min(panel.h * 0.9 * 1000.0 / 640.0);
    let pad = R::new(
        panel.x + (panel.w - pw) / 2.0,
        panel.y + (panel.h - pw * 0.64) / 2.0,
        pw,
        pw * 0.64,
    );
    let k = pad.w / 1000.0;
    let mut out = Vec::new();
    for left in [true, false] {
        let col_w = pad.x - panel.x - 50.0;
        let mut side: Vec<Label> = PARTS
            .iter()
            .filter(|p| p.4 == left)
            .filter_map(|&(ctl, name, u, v, _)| {
                let what = c.pad.iter().find(|p| p.0 == ctl)?.1.clone();
                let text = fit(&f.body, LABEL * s, &format!("{name}: {what}"), col_w * s);
                let w = measure(&f.body, LABEL * s, &text) / s;
                let anchor = (pad.x + u * k, pad.y + v * k);
                let x = if left {
                    pad.x - 50.0 - w
                } else {
                    pad.right() + 50.0
                };
                Some(Label {
                    control: ctl,
                    text,
                    r: R::new(x, anchor.1 - LABEL_H / 2.0, w, LABEL_H),
                    anchor,
                    left,
                })
            })
            .collect();
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

/// Draws the controller panel's contents in `panel`.
pub fn draw_pad_panel(cv: &mut Canvas, panel: R, c: &ControlSheet, f: &Fonts) {
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
    }
}

/// The pad itself: body, bumpers, triggers, sticks, D-pad, face buttons, Back and Start.
fn draw_pad(cv: &mut Canvas, pad: R, c: &ControlSheet, f: &Fonts) {
    let k = pad.w / 1000.0;
    let p = |u: f32, v: f32| (pad.x + u * k, pad.y + v * k);
    let mapped = |ctl: &str| c.pad.iter().any(|q| q.0 == ctl);
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
    let (arm, t) = (120.0 * k, 42.0 * k);
    let dcol = if mapped("dpad") {
        0xFF4A505A
    } else {
        0xFF262A30
    };
    cv.fill(R::new(dx - arm / 2.0, dy - t / 2.0, arm, t), dcol);
    cv.fill(R::new(dx - t / 2.0, dy - arm / 2.0, t, arm), dcol);
    for (ctl, u, v, tint) in [
        ("y", 720.0, 175.0, 0xFF8A7A20),
        ("x", 655.0, 240.0, 0xFF20508A),
        ("b", 785.0, 240.0, 0xFF8A2828),
        ("a", 720.0, 305.0, 0xFF287A30),
    ] {
        let (x, y) = p(u, v);
        cv.circle(x, y, 31.0 * k, if mapped(ctl) { tint } else { 0xFF2A2D33 });
        let size = 34.0 * k;
        cv.text_mid(
            &f.body,
            size,
            x,
            y + size * 0.36,
            &ctl.to_ascii_uppercase(),
            ink(ctl),
        );
    }
    for (ctl, u) in [("back", 430.0), ("start", 570.0)] {
        let (x, y) = p(u, 240.0);
        cv.circle(x, y, 17.0 * k, ink(ctl));
        cv.circle(x, y, 12.0 * k, 0xFF1A1C20);
    }
}

/// Draws the keyboard and mouse lines in `panel`, as many as fit.
pub fn draw_keyboard_panel(cv: &mut Canvas, panel: R, c: &ControlSheet, f: &Fonts) {
    let mut y = panel.y;
    for text in &c.keys {
        if y + LABEL_H > panel.bottom() {
            break;
        }
        let t = fit(&f.body, LABEL * cv.s, text, panel.w * cv.s);
        cv.text(&f.body, LABEL, panel.x, y + 26.0, &t, TEXT);
        y += LABEL_H + GAP * 2.0;
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
    use super::super::preboot::{CONTROLS_PANELS, MENU_PANELS};
    use super::*;

    fn crowded() -> ControlSheet {
        ControlSheet {
            pad: PARTS
                .iter()
                .map(|p| (p.0, "a long description of what it does".to_string()))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn labels_never_overlap_at_1080p_and_720p() {
        let f = Fonts::load().unwrap();
        for c in [ControlSheet::default(), crowded()] {
            for (w, h) in [(1920, 1080), (1280, 720)] {
                let s = Canvas::new(w, h).s;
                for panels in [MENU_PANELS, CONTROLS_PANELS] {
                    let inner = inner(panels.0);
                    let (pad, ls) = layout(inner, &c, &f, s);
                    assert_eq!(ls.len(), c.pad.len());
                    for (i, a) in ls.iter().enumerate() {
                        assert!(!a.r.overlaps(&pad), "{} over the pad", a.control);
                        assert!(a.r.y >= inner.y && a.r.bottom() <= inner.bottom() + 0.01);
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
}
