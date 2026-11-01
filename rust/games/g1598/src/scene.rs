//! The 3D hole view (the author's `draw_hole` and everything under it): a view (`_view1`: position, eye height and
//! heading), `transform` (heading rotation and the fixed 11-degree pitch), `perspect` (480 and 545 focal lengths onto
//! the 320 x 200 picture), `clip_3d` and `clip_ocean` (clipping against the floor plane y = z), the edge-table
//! scan fill (`compute_edge`, `draw_poly`, `hline`, two-colour dither), the sky and ground (`draw_bkg`), the
//! mountain strips by view angle (`draw_horizon`, `xy_to_angle960`) and the depth-sorted trees and flag
//! (`draw_trees`). Integer arithmetic throughout, as the original's.

use crate::fixed::{div_int2frac, div_sat, hi2, magxy, mul_div};
use native_engine::gfx::Bitmap;

pub const W: i32 = 320;
pub const H: i32 = 200;
/// The sky's last line + 1 (`draw_bkg`).
pub const HORIZON: i32 = 44;
/// sin and cos of the view's fixed downward pitch (1.15).
const PITCH_SIN: i16 = 0x186c;
const PITCH_COS_NEG: i16 = -32166; // 0x825a

/// The view (`_view1`): where the eye is, its height (z, negative = above the ground) and its heading as a 1.15
/// sine and cosine pair (offsets 6 and 8 of the record).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct View {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub s: i16,
    pub c: i16,
}

impl View {
    /// `create_matrix(target, view)`: aims the view at a point.
    pub fn aim(&mut self, tx: i16, ty: i16) {
        let dy = ty.wrapping_sub(self.y);
        let dx = tx.wrapping_sub(self.x);
        let d = magxy(dx, dy);
        self.c = if dx.wrapping_abs() == d {
            if dx < 0 {
                -0x7fff
            } else {
                0x7fff
            }
        } else {
            div_int2frac(dx, d)
        };
        self.s = if dy.wrapping_abs() == d {
            if dy < 0 {
                0x7fff
            } else if d != 0 {
                -0x7fff
            } else {
                0
            }
        } else {
            div_int2frac(dy.wrapping_neg(), d)
        };
    }

    /// `transform`: a ground point into view space (x across, y down the picture, z into it).
    pub fn transform(&self, x: i16, y: i16, z: i16) -> [i16; 3] {
        let dx = x.wrapping_sub(self.x) as i32;
        let dy = y.wrapping_sub(self.y) as i32;
        let o0 = hi2((dx * self.s as i32).wrapping_add(dy * self.c as i32));
        let t = hi2((dy * self.s as i32).wrapping_sub(dx * self.c as i32)) as i32;
        let dz = z.wrapping_sub(self.z) as i32;
        let o1 = hi2((t * PITCH_SIN as i32).wrapping_sub(dz * PITCH_COS_NEG as i32));
        let o2 = hi2((dz * PITCH_SIN as i32).wrapping_add(t * PITCH_COS_NEG as i32));
        [o0, o1, o2]
    }

    /// `transform_ocean`: the 32-bit transform used for far water points.
    pub fn transform_ocean(&self, x: i16, y: i16, z: i16) -> [i32; 3] {
        let dx = x as i32 - self.x as i32;
        let dy = y as i32 - self.y as i32;
        let dz = z as i32 - self.z as i32;
        let o0 = (self.s as i32)
            .wrapping_mul(dx)
            .wrapping_add((self.c as i32).wrapping_mul(dy))
            >> 15;
        let t = (self.s as i32)
            .wrapping_mul(dy)
            .wrapping_sub((self.c as i32).wrapping_mul(dx))
            >> 15;
        let o1 = t
            .wrapping_mul(PITCH_SIN as i32)
            .wrapping_sub(dz.wrapping_mul(PITCH_COS_NEG as i32))
            >> 15;
        let o2 = dz
            .wrapping_mul(PITCH_SIN as i32)
            .wrapping_add(t.wrapping_mul(PITCH_COS_NEG as i32))
            >> 15;
        [o0, o1, o2]
    }
}

/// `perspect`: a view-space point onto the picture.
pub fn perspect(p: [i16; 3]) -> (i16, i16) {
    if p[2] == 0 {
        let x = if p[0] > 0 { 319 } else { p[0] };
        let y = if p[1] > 0 { 199 } else { p[1] };
        return (x, y);
    }
    let x = div_sat(p[0] as i32 * 480, p[2], 0x7f69).wrapping_add(150);
    let y = div_sat(p[1] as i32 * 545, p[2], 0x7f6a).wrapping_add(149);
    (x, y)
}

/// `perspect_ocean`: the 32-bit projection, saturated to +-0x3fff; a point at depth 0 keeps `keep`.
fn perspect_ocean(p: [i32; 3], keep: (i16, i16)) -> (i16, i16) {
    if p[2] == 0 {
        return keep;
    }
    let f = |v: i32, k: i32, off: i32| {
        let r = v.wrapping_mul(k).wrapping_div(p[2]).wrapping_add(off);
        if r > 0x7fff {
            0x3fff
        } else if r < -0x8000 {
            -0x4000
        } else {
            r as i16
        }
    };
    (f(p[0], 480, 0x9e), f(p[1], 545, 0x94))
}

/// The scan-fill edge table (`_et`, `_linkptr`, `_xmin`, `_invm`, `_ymaxl`, `_ymin`, `_ymax`).
pub struct Fill {
    et: [u16; 200],
    link: Vec<u16>,
    x: Vec<i32>,
    slope: Vec<i32>,
    ymaxl: Vec<i16>,
    ymin: i16,
    ymax: i16,
    /// The scan line's crossings; kept between lines as the original's stack array is.
    xs: Vec<i16>,
    /// The fill window (`d1_031E` .. `d1_0324`).
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl Default for Fill {
    fn default() -> Fill {
        let mut f = Fill {
            et: [0; 200],
            link: vec![0],
            x: vec![0],
            slope: vec![0],
            ymaxl: vec![0],
            ymin: 0,
            ymax: 0,
            xs: vec![0; 64],
            left: 0,
            top: 0,
            right: 319,
            bottom: 199,
        };
        f.reset();
        f
    }
}

impl Fill {
    /// `reset_poly`.
    pub fn reset(&mut self) {
        self.et = [0; 200];
        self.link.truncate(1);
        self.x.truncate(1);
        self.slope.truncate(1);
        self.ymaxl.truncate(1);
        self.ymax = 0;
        self.ymin = 200;
    }

    /// `compute_edge(p1, p2)`.
    pub fn edge(&mut self, p1: (i16, i16), p2: (i16, i16)) {
        let dy = p1.1.wrapping_sub(p2.1);
        if dy == 0 {
            return;
        }
        let slope = ((p1.0.wrapping_sub(p2.0) as i32) << 16).wrapping_div(dy as i32);
        let (mut ymax, mut ys, xs) = if dy < 0 {
            (p2.1, p1.1, p1.0)
        } else {
            (p1.1, p2.1, p2.0)
        };
        if ys >= self.bottom || ymax < self.top {
            return;
        }
        let mut x = ((xs as i32) << 16).wrapping_add(slope);
        if ys < self.top - 1 {
            x = x.wrapping_add(((self.top - 1 - ys) as i32).wrapping_mul(slope));
            ys = self.top - 1;
        }
        if ymax > self.bottom {
            ymax = self.bottom;
        }
        let row = ys + 1;
        if self.ymin > row {
            self.ymin = row;
        }
        if self.ymax < ymax {
            self.ymax = ymax;
        }
        let id = self.x.len() as u16;
        let r = row as usize;
        if self.et[r] == 0 {
            self.et[r] = id;
        } else {
            let mut e = self.et[r];
            while self.link[e as usize] != 0 {
                e = self.link[e as usize];
            }
            self.link[e as usize] = id;
        }
        self.ymaxl.push(ymax);
        self.x.push(x);
        self.slope.push(slope);
        self.link.push(0);
    }

    /// `draw_poly(c1, c2)`: fills the edges collected so far, then `reset_poly`.
    pub fn draw(&mut self, scr: &mut Bitmap, c1: u8, c2: u8) {
        let mut active: Vec<u16> = Vec::new();
        let mut y = self.ymin;
        while y <= self.ymax {
            let mut e = self.et[y as usize];
            while e != 0 {
                active.push(e);
                e = self.link[e as usize];
            }
            let mut n = 0usize;
            let mut k = 0usize;
            while k < active.len() {
                let e = active[k] as usize;
                if self.xs.len() <= n + 1 {
                    self.xs.resize(n + 8, 0);
                }
                self.xs[n] = (self.x[e] >> 16) as i16;
                self.x[e] = self.x[e].wrapping_add(self.slope[e]);
                for j in 0..n {
                    let v = self.xs[j];
                    if self.xs[n] < v {
                        self.xs[j] = self.xs[n];
                        self.xs[n] = v;
                    }
                }
                if self.ymaxl[e] == y {
                    active.remove(k);
                } else {
                    k += 1;
                }
                n += 1;
            }
            let mut j = 0;
            while j < n {
                self.clipped_hline(scr, self.xs[j], y, self.xs[j + 1], c1, c2);
                j += 2;
            }
            y += 1;
        }
        self.reset();
    }

    /// `clipped_hline`.
    fn clipped_hline(&self, scr: &mut Bitmap, mut x1: i16, y: i16, mut x2: i16, c1: u8, c2: u8) {
        if x1 >= self.left && x2 <= self.right {
            hline(scr, x1, y, x2, c1, c2);
            return;
        }
        if x1 > self.right || x2 < self.left {
            return;
        }
        if x1 < self.left {
            x1 = self.left;
        }
        if x2 > self.right {
            x2 = self.right;
        }
        hline(scr, x1, y, x2, c1, c2);
    }
}

/// `hline` with the current pattern (`pattern_on(c1, c2)`; `set_color(c)` is c, c): a checkerboard of the two
/// colours, c1 where x + y is even.
pub fn hline(scr: &mut Bitmap, x1: i16, y: i16, x2: i16, c1: u8, c2: u8) {
    for x in x1..=x2 {
        scr.set(
            x as i32,
            y as i32,
            if (x as i32 + y as i32) & 1 == 0 {
                c1
            } else {
                c2
            },
        );
    }
}

/// `draw_bkg(flag)`: sky (colour 9) to line 43, then the ground (pattern 3, 3 when the flag is set, else colour 2).
pub fn draw_bkg(scr: &mut Bitmap, f: &Fill, flag: bool) {
    let mut y = f.top;
    while y < HORIZON as i16 {
        hline(scr, f.left, y, f.right, 9, 9);
        y += 1;
    }
    let c = if flag { 3 } else { 2 };
    while y <= f.bottom {
        hline(scr, f.left, y, f.right, c, c);
        y += 1;
    }
}

/// `clip_3d`: a ground polygon (x, y word pairs at z = 0) transformed, clipped to y <= z, projected and given to
/// the edge table. Its first vertex only starts the first edge, as in the original.
pub fn clip_3d(v: &View, pts: &[i16], f: &mut Fill) {
    let n = pts.len();
    if n < 2 {
        return;
    }
    let mut out: Vec<[i16; 3]> = Vec::with_capacity(n);
    let mut p = v.transform(pts[0], pts[1], 0);
    let mut k = 2;
    while k + 1 < n {
        let c = v.transform(pts[k], pts[k + 1], 0);
        if c[1] > c[2] {
            if p[1] <= p[2] {
                let a = c[2].wrapping_sub(c[1]);
                let b = p[2].wrapping_sub(c[2]);
                let d = p[1].wrapping_sub(c[1]).wrapping_sub(b);
                let e = p[0].wrapping_sub(c[0]);
                let zz = mul_div(a, b, d).wrapping_add(c[2]);
                let xx = mul_div(a, e, d).wrapping_add(c[0]);
                out.push([xx, zz, zz]);
            }
        } else {
            if p[1] > p[2] {
                let a = p[2].wrapping_sub(p[1]);
                let b = c[2].wrapping_sub(p[2]);
                let d = c[1].wrapping_sub(p[1]).wrapping_sub(b);
                let e = c[0].wrapping_sub(p[0]);
                let zz = mul_div(a, b, d).wrapping_add(p[2]);
                let xx = mul_div(a, e, d).wrapping_add(p[0]);
                out.push([xx, zz, zz]);
            }
            out.push(c);
        }
        p = c;
        k += 2;
    }
    edges(out.iter().map(|&q| perspect(q)).collect(), f);
}

/// The edges of a projected outline, closing it last to first (both clip routines end this way).
fn edges(s: Vec<(i16, i16)>, f: &mut Fill) {
    if s.is_empty() {
        return;
    }
    for i in 1..s.len() {
        f.edge(s[i - 1], s[i]);
    }
    f.edge(s[s.len() - 1], s[0]);
}

/// `clip_ocean`: water polygons may reach far beyond the 16-bit range, so points outside -25000..32000 go through
/// the 32-bit transform at 25 units below the eye.
pub fn clip_ocean(v: &View, pts: &[i16], f: &mut Fill) {
    let n = pts.len();
    if n < 2 {
        return;
    }
    let tr = |x: i16, y: i16| -> [i32; 3] {
        if (-25000..=32000).contains(&x) && (-25000..=32000).contains(&y) {
            let t = v.transform(x, y, 0);
            [t[0] as i32, t[1] as i32, t[2] as i32]
        } else {
            v.transform_ocean(x, y, v.z.wrapping_add(25))
        }
    };
    let mut out: Vec<[i32; 3]> = Vec::with_capacity(n);
    let mut p = tr(pts[0], pts[1]);
    let mut k = 2;
    while k + 1 < n {
        let c = tr(pts[k], pts[k + 1]);
        if c[1] > c[2] {
            if p[1] <= p[2] {
                let a = c[2].wrapping_sub(c[1]);
                let b = p[2].wrapping_sub(c[2]);
                let d = p[1].wrapping_sub(c[1]).wrapping_sub(b);
                let e = p[0].wrapping_sub(c[0]);
                let zz = div0(b.wrapping_mul(a), d).wrapping_add(c[2]);
                let xx = div0(e.wrapping_mul(a), d).wrapping_add(c[0]);
                out.push([xx, zz, zz]);
            }
        } else {
            if p[1] > p[2] {
                let a = p[2].wrapping_sub(p[1]);
                let b = c[2].wrapping_sub(p[2]);
                let d = c[1].wrapping_sub(p[1]).wrapping_sub(b);
                let e = c[0].wrapping_sub(p[0]);
                let zz = div0(b.wrapping_mul(a), d).wrapping_add(p[2]);
                let xx = div0(e.wrapping_mul(a), d).wrapping_add(p[0]);
                out.push([xx, zz, zz]);
            }
            out.push(c);
        }
        p = c;
        k += 2;
    }
    let mut keep = (0, 0);
    let s = out
        .iter()
        .map(|&q| {
            keep = perspect_ocean(q, keep);
            keep
        })
        .collect();
    edges(s, f);
}

fn div0(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        a.wrapping_div(b)
    }
}

/// `xy_to_angle960`: the bearing of (x, y) in 960ths of a turn, from the `_angles` table (16 tangents).
pub fn xy_to_angle960(angles: &[i16], x: i16, y: i16) -> i16 {
    let ax = x.wrapping_abs();
    let ay = y.wrapping_abs();
    let mn = ax.min(ay);
    let mx = ax.max(ay);
    if mx == 0 {
        return 14;
    }
    let t = div_int2frac(mn, mx);
    let mut i: i16 = 14;
    while i != 0 && t < angles[i as usize] {
        i -= 1;
    }
    let span = angles[i as usize + 1].wrapping_sub(angles[i as usize]);
    let num = ((t.wrapping_sub(angles[i as usize]) as i32) << 3).wrapping_add((span >> 1) as i32);
    let num = (num as i16) as i32;
    let q = if span == 0 {
        0
    } else {
        (num / span as i32) as i16
    };
    let mut a = q.wrapping_add(i << 3);
    if ay > ax {
        a = 0xf0 - a;
    }
    if (x ^ y) < 0 {
        a = 0xf0 - a;
    }
    if x < 0 {
        a += if y < 0 { 0x1e0 } else { 0xf0 };
    } else if y < 0 {
        a += 0x2d0;
        if a == 0x3c0 {
            a = 0;
        }
    }
    a
}

/// Draws an object with its bottom centre at (x, y), colour 0 clear, clipped to the window (`init_obj` placement).
pub fn blit_bottom_centre(scr: &mut Bitmap, obj: &Bitmap, x: i32, y: i32, f: &Fill) {
    let x0 = x - (obj.w as i32 >> 1);
    let y0 = y - (obj.h as i32 - 1);
    for oy in 0..obj.h as i32 {
        let sy = y0 + oy;
        if sy < f.top as i32 || sy > f.bottom as i32 {
            continue;
        }
        for ox in 0..obj.w as i32 {
            let sx = x0 + ox;
            if sx < f.left as i32 || sx > f.right as i32 {
                continue;
            }
            let c = obj.px[(oy as usize) * obj.w + ox as usize];
            if c != 0 {
                scr.set(sx, sy, c);
            }
        }
    }
}

/// A tree, flag or marker standing on the hole (the `_objs` records from 8 on, `new_hole`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Standing {
    /// 0x20..0x23 a tree kind, 0x1f the pin, 0x1e the player marker.
    pub kind: i16,
    pub x: i16,
    pub y: i16,
}

/// What `draw_trees` needs besides the view.
pub struct TreeArt<'a> {
    pub tree_dist: &'a [i16],
    /// The course's tree bank (4 kinds x 12 sizes).
    pub trees: &'a [Bitmap],
    /// GR3: the flag in 12 sizes (objects 0..11).
    pub flags: &'a [Bitmap],
    /// Objects closer than this are not drawn (`_too_close`).
    pub too_close: i16,
}

/// `draw_trees`: every standing object transformed, shell-sorted far to near, sized by distance and drawn.
pub fn draw_trees(scr: &mut Bitmap, v: &View, objs: &[Standing], art: &TreeArt, f: &Fill) {
    let mut rec: Vec<([i16; 3], usize)> = objs
        .iter()
        .enumerate()
        .map(|(i, o)| (v.transform(o.x, o.y, 0), i))
        .collect();
    let n = rec.len();
    let mut h = 1usize;
    while h <= n {
        h = h * 3 + 1;
    }
    while h > 1 {
        h /= 3;
        for i in h..n {
            let key = rec[i];
            let mut j = i;
            while j >= h && rec[j - h].0[2] < key.0[2] {
                rec[j] = rec[j - h];
                j -= h;
            }
            rec[j] = key;
        }
    }
    for (u, &(p, i)) in rec.iter().enumerate() {
        let z = p[2];
        let kind = objs[i].kind;
        if z <= 0 || !(kind < 0x20 || art.too_close < z) {
            continue;
        }
        let mut s = 0usize;
        while s < 11 && z >= art.tree_dist[s] {
            s += 1;
        }
        let img = match kind {
            0x1f => art.flags.get(s),
            0x20..=0x23 => {
                if s == 11 && (u & 1) != 0 {
                    s = 10;
                }
                art.trees.get(s + (kind as usize - 0x20) * 12)
            }
            _ => None,
        };
        if let Some(img) = img {
            let (x, y) = perspect(p);
            blit_bottom_centre(scr, img, x as i32, y as i32, f);
        }
    }
}
