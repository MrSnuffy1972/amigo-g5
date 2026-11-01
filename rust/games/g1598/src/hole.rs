//! A hole as the game holds it once loaded (`new_hole`): the ground polygons in file order with their bounding
//! boxes, the tee, pro tee and aim point, the pin for the tournament day, the standing objects (trees and flag, then
//! the player marker last), and the scene drawing (`draw_hole`) and camera set-up (`init_scene`).

use crate::assets::{Course, Hole, Tables};
use crate::scene::{self, Fill, Standing, TreeArt, View};
use native_engine::gfx::Bitmap;

#[derive(Debug, Clone, Default)]
pub struct Poly {
    /// The polygon's type (8 is kept as 0, as `new_hole` does for its table).
    pub kind: i16,
    pub min: (i16, i16),
    pub max: (i16, i16),
    pub pts: Vec<i16>,
}

#[derive(Debug, Clone, Default)]
pub struct Loaded {
    pub polys: Vec<Poly>,
    pub tee: (i16, i16),
    pub pro_tee: (i16, i16),
    pub aim: (i16, i16),
    pub pin: (i16, i16),
    pub objs: Vec<Standing>,
}

/// `new_hole`: walks the record list. `day` picks the pin (records 0x13 to 0x16).
pub fn load(h: &Hole, day: i16) -> Loaded {
    let mut l = Loaded::default();
    for (t, w) in &h.records {
        let t = *t;
        let p2 = || {
            (
                w.first().copied().unwrap_or(0),
                w.get(1).copied().unwrap_or(0),
            )
        };
        match t {
            0..=9 => {
                let mut p = Poly {
                    kind: if t == 8 { 0 } else { t },
                    min: (0x7fff, 0x7fff),
                    max: (-0x7fff, -0x7fff),
                    pts: w.clone(),
                };
                for c in w.chunks(2) {
                    if c.len() == 2 {
                        p.max.0 = p.max.0.max(c[0]);
                        p.min.0 = p.min.0.min(c[0]);
                        p.max.1 = p.max.1.max(c[1]);
                        p.min.1 = p.min.1.min(c[1]);
                    }
                }
                l.polys.push(p);
            }
            0x10 => l.tee = p2(),
            0x11 => l.pro_tee = p2(),
            0x12 => l.aim = p2(),
            0x13..=0x16 => {
                if t - 0x13 == day {
                    l.pin = p2();
                    l.objs.push(Standing {
                        kind: 0x1f,
                        x: l.pin.0,
                        y: l.pin.1,
                    });
                }
            }
            0x20..=0x23 => {
                let (x, y) = p2();
                l.objs.push(Standing { kind: t, x, y });
            }
            _ => {}
        }
    }
    l.objs.push(Standing {
        kind: 0x1e,
        x: 0,
        y: 0,
    });
    l
}

/// The eye height above the ground (`_view1+4`).
pub const EYE_Z: i16 = -88;
/// How far behind the ball the eye stands (`init_scene`).
pub const PULL_BACK: i16 = 0x17c;

/// `init_scene`'s camera: at the ball, aimed at `target`, pulled back behind it.
pub fn camera(ball: (i16, i16), target: (i16, i16)) -> View {
    let mut v = View {
        x: ball.0,
        y: ball.1,
        z: EYE_Z,
        s: 0,
        c: 0,
    };
    v.aim(target.0, target.1);
    v.x = v.x.wrapping_sub(crate::fixed::mul_frac(PULL_BACK, v.c));
    v.y = v.y.wrapping_add(crate::fixed::mul_frac(PULL_BACK, v.s));
    v
}

/// `draw_hole`: background, every ground record in file order, the mountains toward the pin, then the trees.
pub fn draw(
    scr: &mut Bitmap,
    v: &View,
    h: &Hole,
    l: &Loaded,
    c: &Course,
    gr3: &[Bitmap],
    t: &Tables,
) {
    let mut f = Fill::default();
    scene::draw_bkg(scr, &f, true);
    for (kind, w) in &h.records {
        let k = *kind;
        if !(0..0x10).contains(&k) {
            continue;
        }
        if k == 6 {
            scene::clip_ocean(v, w, &mut f);
        } else {
            scene::clip_3d(v, w, &mut f);
        }
        let i = (k as usize).min(9);
        f.draw(scr, t.poly1[i], t.poly2[i]);
    }
    draw_horizon(scr, v, l.pin, c, t, &f);
    let art = TreeArt {
        tree_dist: &t.tree_dist,
        trees: &c.tree_bank,
        flags: gr3,
        too_close: 0,
    };
    scene::draw_trees(scr, v, &l.objs, &art, &f);
}

/// `draw_horizon`: the three mountain strips, placed by the bearing from the eye to the pin.
pub fn draw_horizon(scr: &mut Bitmap, v: &View, pin: (i16, i16), c: &Course, t: &Tables, f: &Fill) {
    let a = scene::xy_to_angle960(&t.angles, pin.0.wrapping_sub(v.x), pin.1.wrapping_sub(v.y));
    let mut d = 80 - a;
    if d <= -480 {
        d += 480;
    }
    for m in &c.mountains {
        let x = if d < -80 { d + 480 } else { d };
        scene::blit_bottom_centre(scr, m, x as i32, 0x2b, f);
        d += 160;
    }
}
