//! The ball in motion (the author's vertical-blank `ball_interrupt` and what it calls): flight with wind and spin,
//! landing by surface (bounce, plug in sand, stop in water), the roll with green contours, and the cup. Positions are
//! 16.16 (`_ball_x` .. `_ball_z`, z negative above the ground), velocities likewise; `ball_32_to_16` rounds to the
//! 16-bit position the rest of the game uses. The step runs on odd frames and on every sixth even frame, as the
//! original's counter test does.

use crate::fixed::{magxy, mul_div};
use crate::hole::Poly;

/// Gravity per step.
pub const GRAVITY: i32 = 0x275c;

/// Surfaces as `inside_poly` reports them (polygon types; 6 water becomes 3, 9 becomes none).
pub const OFF: i16 = -1;

/// The ball's state; `sounds` collects the effects it asks for (`sound(n)`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub vx: i32,
    pub vy: i32,
    pub vz: i32,
    pub wind_vx: i32,
    pub wind_vy: i32,
    pub spin_vx: i32,
    pub spin_vy: i32,
    pub backspin_vx: i32,
    pub backspin_vy: i32,
    pub contour_vx: i32,
    pub contour_vy: i32,
    pub tree_vx: i32,
    pub tree_vy: i32,
    pub tree_vz: i32,
    /// The 16-bit position (`_ball`).
    pub pos: (i16, i16, i16),
    pub moving: u8,
    pub rolling: bool,
    pub hop: bool,
    pub wind_zapped: bool,
    pub long_shot: bool,
    pub hit_tree: i8,
    pub off_dead_center: u8,
    pub tree_cnt: i16,
    pub save_tree_cnt: i16,
    pub num_bounces: i8,
    pub ball_cnt: i16,
    pub intcnt: u16,
    /// `_plyr+2`: the surface under the ball.
    pub surface: i16,
    /// `_plyr+6`: the club class (2 = putter shots roll on landing on the green).
    pub club_class: i16,
    pub flag: bool,
    pub sounds: Vec<i32>,
}

/// `ball_32_to_16`: a 16.16 value rounded to 16 bits.
pub fn round16(v: i32) -> i16 {
    ((v >> 16) as i16).wrapping_add(((v & 0x8000) != 0) as i16)
}

/// `inside_poly`: the type of the last polygon whose outline contains the point (even-odd rule, the original's
/// vertex test), 6 reported as 3 and 9 as none (-1).
pub fn inside_poly(polys: &[Poly], bx: i16, by: i16) -> i16 {
    let mut r = OFF;
    for p in polys {
        if !(p.min.0 <= bx && bx <= p.max.0 && p.min.1 <= by && by <= p.max.1) {
            continue;
        }
        let w = &p.pts;
        let n = w.len();
        if n < 4 {
            continue;
        }
        let mut inside = false;
        let mut prev_y = w[n - 3];
        let mut e = 0;
        while e + 1 < n / 2 {
            let (x0, y0, x1, y1) = (w[2 * e], w[2 * e + 1], w[2 * e + 2], w[2 * e + 3]);
            if (y0 <= by || y1 <= by) && (by <= y0 || by <= y1) && (x0 <= bx || x1 <= bx) {
                if by == y0 {
                    let a = if y1.wrapping_sub(y0) < 0 { -1 } else { 1 };
                    let b = if y0.wrapping_sub(prev_y) < 0 { -1 } else { 1 };
                    if a != b {
                        inside = !inside;
                    }
                } else {
                    let dx = mul_div(
                        by.wrapping_sub(y0),
                        x1.wrapping_sub(x0),
                        y1.wrapping_sub(y0),
                    );
                    if x0.wrapping_add(dx) <= bx {
                        inside = !inside;
                    }
                }
            }
            prev_y = y0;
            e += 1;
        }
        if inside {
            r = p.kind;
        }
    }
    match r {
        6 => 3,
        9 => OFF,
        v => v,
    }
}

impl Ball {
    fn to16(&mut self) {
        self.pos = (round16(self.x), round16(self.y), round16(self.z));
    }

    /// One vertical blank of `ball_interrupt`. `contour` is `get_contour_vector` (the green's slope at the ball).
    pub fn interrupt(
        &mut self,
        polys: &[Poly],
        hole: (i16, i16),
        contour: &mut dyn FnMut(i16, i16) -> (i32, i32),
    ) {
        self.intcnt = self.intcnt.wrapping_add(1);
        if self.intcnt & 1 == 0 && (self.intcnt.wrapping_add(6) as i16 as i32) % 6 != 0 {
            return;
        }
        if self.moving == 0 {
            return;
        }
        self.ball_cnt = self.ball_cnt.wrapping_add(1);
        if self.hit_tree <= 1 {
            if self.hit_tree == 1 || self.tree_cnt == self.save_tree_cnt {
                self.vx = self.tree_vx;
                self.vy = self.tree_vy;
                self.vz = self.tree_vz;
                self.wind_vx = 0;
                self.wind_vy = 0;
                self.spin_vx = 0;
                self.spin_vy = 0;
                self.backspin_vx = 0;
                self.backspin_vy = 0;
                let s = match self.off_dead_center {
                    0 => 0x18,
                    0xff => 0x12,
                    _ => 0x10,
                };
                self.sounds.push(s);
                self.hit_tree = 2;
            } else {
                self.tree_cnt = self.tree_cnt.wrapping_add(1);
            }
        }
        if self.rolling {
            self.roll(polys, contour);
        } else if self.fly(polys) {
            return self.cup(hole);
        } else {
            self.to16();
        }
        self.cup(hole);
    }

    /// The flight step; true when the ball has just landed (its landing already settled).
    fn fly(&mut self, polys: &[Poly]) -> bool {
        if !self.wind_zapped && self.vz >= 0 {
            self.wind_zapped = true;
            self.wind_vx = 0;
            self.wind_vy = 0;
        }
        self.x = self.x.wrapping_add(self.vx);
        self.y = self.y.wrapping_add(self.vy);
        self.vx = self.vx.wrapping_add(self.wind_vx);
        self.wind_vx =
            self.wind_vx
                .wrapping_add(if self.ball_cnt >= 10 { self.spin_vx } else { 0 });
        self.vy = self.vy.wrapping_add(self.wind_vy);
        self.wind_vy =
            self.wind_vy
                .wrapping_add(if self.ball_cnt >= 10 { self.spin_vy } else { 0 });
        self.z = self.z.wrapping_add(self.vz);
        if self.z < 0 {
            self.vz = self.vz.wrapping_add(GRAVITY);
            return false;
        }
        self.land(polys);
        true
    }

    fn land(&mut self, polys: &[Poly]) {
        self.z = 0;
        self.wind_vx = 0;
        self.wind_vy = 0;
        self.spin_vx = 0;
        self.spin_vy = 0;
        self.long_shot = false;
        self.to16();
        let mut t = inside_poly(polys, self.pos.0, self.pos.1);
        if t == 4 {
            t = 0;
        }
        self.surface = t;
        let mut s = -1;
        if t == OFF || t == 5 {
            s = if self.num_bounces > 2 { 5 } else { 4 };
        } else if t == 1 || t == 0 || t == 7 {
            s = if self.num_bounces > 2 { 3 } else { 2 };
        }
        self.num_bounces = self.num_bounces.wrapping_add(1);
        if s != -1 {
            self.sounds.push(s);
        }
        let mut d4 = self.vz;
        if self.vz as u32 & 0xffff_0000 == 0 {
            self.rolling = true;
            return;
        }
        match self.surface {
            OFF => d4 >>= 1,
            5 => {}
            1 => {
                if self.club_class == 2 {
                    self.vx >>= 1;
                    self.vy >>= 1;
                    self.rolling = true;
                    return;
                }
            }
            _ => {
                if t != 0 && t != 7 {
                    self.moving = 0;
                    return;
                }
            }
        }
        self.vz = !(d4 >> 2);
        let (mut a, mut b) = (self.vx, self.vy);
        match self.surface {
            0 => {
                a >>= 1;
                b >>= 1;
            }
            1 => {
                self.vx = a.wrapping_add(self.backspin_vx) >> 1;
                self.vy = b.wrapping_add(self.backspin_vy) >> 1;
                self.backspin_vx -= self.backspin_vx >> 2;
                self.backspin_vy -= self.backspin_vy >> 2;
                return;
            }
            _ => {
                a >>= 2;
                b >>= 2;
            }
        }
        self.vx = a >> 1;
        self.vy = b >> 1;
        self.backspin_vx = 0;
        self.backspin_vy = 0;
    }

    fn roll(&mut self, polys: &[Poly], contour: &mut dyn FnMut(i16, i16) -> (i32, i32)) {
        if self.hop {
            self.vz = self.vz.wrapping_add(GRAVITY);
            self.z = self.z.wrapping_add(self.vz);
            if self.z < 0 {
                self.x = self.x.wrapping_add(self.vx);
                self.y = self.y.wrapping_add(self.vy);
                self.to16();
                return;
            }
            self.z = 0;
            self.hop = false;
        }
        let (cx, cy) = contour(self.pos.0, self.pos.1);
        self.contour_vx = cx;
        self.contour_vy = cy;
        let slow = self.surface == 1 || self.surface == 0;
        let mut d4 = self.vx;
        let mut d5 = d4;
        if self.ball_cnt >= 10 {
            d4 = d4.wrapping_add(cx.wrapping_add(self.spin_vx));
            d5 = d4;
            self.spin_vx >>= 1;
        }
        self.x = self.x.wrapping_add(d4);
        if slow {
            d4 >>= 1;
        }
        self.vx = d5.wrapping_sub(d4 >> 5);
        let mut d4 = self.vy;
        let mut d5 = d4;
        if self.ball_cnt >= 10 {
            d4 = d4.wrapping_add(cy.wrapping_add(self.spin_vy));
            d5 = d4;
            self.spin_vy >>= 1;
        }
        self.y = self.y.wrapping_add(d4);
        if slow {
            d4 >>= 1;
        }
        self.vy = d5.wrapping_sub(d4 >> 5);
        let hy = self.vy as u32 & 0xffff_0000;
        if hy == 0 || hy.wrapping_add(0x10000) == 0 {
            let mut hx = self.vx as u32 & 0xffff_0000;
            if hx != 0 {
                hx = hx.wrapping_add(0x10000);
            }
            self.moving = ((hx & 0xff_0000) >> 16) as u8;
        }
        self.to16();
        let t = inside_poly(polys, self.pos.0, self.pos.1);
        self.surface = t;
        if t & 0xff == 3 {
            self.moving = 0;
        }
    }

    /// `check_cup`: on the ground within 2 of the hole, slow enough drops in, faster hops over the lip.
    fn cup(&mut self, hole: (i16, i16)) {
        self.flag = true;
        if self.pos.2 != 0 {
            return;
        }
        if magxy(
            self.pos.0.wrapping_sub(hole.0),
            self.pos.1.wrapping_sub(hole.1),
        ) as u16
            > 2
        {
            return;
        }
        if (magxy((self.vx >> 8) as i16, (self.vy >> 8) as i16) as u16) < 0x276 {
            self.moving = 0;
            self.pos.0 = hole.0;
            self.pos.1 = hole.1;
        } else {
            self.sounds.push(0x17);
            self.vx >>= 1;
            self.vy >>= 1;
            self.vz = 0xfffe_7668u32 as i32;
            self.hop = true;
        }
    }
}
