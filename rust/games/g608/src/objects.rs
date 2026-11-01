//! E-Motion's objects (update_and_draw_objects, +$5344): each pass every object runs its routine for its kind (the
//! jump table at +$530C), then each is drawn (the table at +$53CA). The physics (+$49F8 to +$5118) is kept exactly:
//! 16.16 positions and speeds, the original's 16-bit multiplies and divides, its square root and its rounding.

use crate::emo::{Emo, Obj, FIRE, OBJS};
use crate::game::Go;
use crate::ops::{divs, divu, isqrt, lo, muls, mulu, random_below, random_next};

impl Emo {
    /// update_and_draw_objects (+$5344).
    pub fn update_and_draw_objects(&mut self) -> Go {
        self.alive = 0;
        self.hit_list.clear();
        let mut i = 0;
        while i < OBJS {
            let k = self.objs[i].kind;
            if k == 0xFFFF {
                break;
            }
            if k != 0 {
                let g = self.update(i, k);
                if g != Go::On {
                    return g;
                }
            }
            i += 1;
        }
        for i in 0..OBJS {
            let k = self.objs[i].kind;
            if k == 0xFFFF {
                break;
            }
            if k != 0 {
                self.draw_obj(i, k);
            }
        }
        if self.alive == 1 && self.mode != 2 && !self.kill_touch && self.bonus == 0 {
            let j = self.last_obj;
            if self.objs[j].kind == 0x34 {
                if self.objs[j].w24 == 0 {
                    return Go::On;
                }
                self.objs[j].w24 = 1;
            }
            if self.objs[j].kind == 0x14 {
                self.ball_explodes(j);
            }
        }
        Go::On
    }

    fn update(&mut self, i: usize, k: u16) -> Go {
        match k {
            0x04 => self.update_ship(i),
            0x08 => return self.update_exploding(i),
            0x0C => {
                let o = &mut self.objs[i];
                o.w22 = (o.w22 + 1) & 3;
            }
            0x10 => self.update_tether(i),
            0x14 => self.update_ball(i),
            0x18 => self.update_fragment(i),
            0x1C => {
                let o = &mut self.objs[i];
                o.size -= 1;
                if o.size < 0 {
                    o.kind = 0;
                }
            }
            0x20 => {
                let o = &mut self.objs[i];
                o.w22 = o.w22.wrapping_sub(1);
                if o.w22 == 0 && self.bonus == 0 {
                    self.pod_count = 0;
                    let o = &mut self.objs[i];
                    o.kind = 0x2C;
                    o.max = 0xC;
                    self.effect_named("pod_hatches");
                }
                self.drift(i);
            }
            0x24 => {}
            0x28 => {
                let o = &mut self.objs[i];
                o.size += 1;
                if (o.max as i16) < o.size {
                    o.kind = 0x20;
                    o.w22 = 0x4B;
                }
                self.drift(i);
            }
            0x2C => {
                let pt = self.pod_time;
                let o = &mut self.objs[i];
                o.size += 1;
                if (o.max as i16) < o.size {
                    o.kind = 0x14;
                    o.w22 = pt;
                    o.w24 = 5;
                }
                self.drift(i);
            }
            0x30 => {
                let o = &mut self.objs[i];
                o.size += 1;
                if o.size >= o.max as i16 {
                    self.burst(i);
                    self.initialize_extra_output_block();
                }
            }
            0x34 => {
                let o = &mut self.objs[i];
                if o.size == o.max as i16 {
                    o.w24 = o.w24.wrapping_sub(1);
                    if o.w24 != 0 {
                        self.drift(i);
                        return Go::On;
                    }
                }
                let o = &mut self.objs[i];
                if o.w24 == 0 {
                    o.size -= 1;
                    if o.size == 0 {
                        o.kind = 0;
                        return Go::On;
                    }
                } else {
                    o.size += 1;
                }
                self.drift(i);
            }
            _ => {}
        }
        Go::On
    }

    /// The ship (+$49F8): it glides (losing a sixteenth of its speed a pass), turns, turns round and thrusts.
    fn update_ship(&mut self, i: usize) {
        let o = &mut self.objs[i];
        o.x = o.x.wrapping_add(o.vx);
        o.y = o.y.wrapping_add(o.vy);
        o.vx = o.vx.wrapping_sub(o.vx >> 4);
        o.vy = o.vy.wrapping_sub(o.vy >> 4);
        let one = o.colour == 0xFFFF;
        let (d0, d2) = if self.mode == 2 {
            self.demo_thrust();
            let p = self.demo_prev;
            self.demo_prev = self.demo_in;
            (self.demo_in, p)
        } else if one {
            let p = self.prev1;
            self.prev1 = self.joy1;
            (self.joy1, p)
        } else {
            let p = self.prev2;
            self.prev2 = self.joy2;
            (self.joy2, p)
        };
        let mut d1 = self.objs[i].w22;
        if d0 & 4 != 0 {
            d1 = d1.wrapping_add(self.turn_rate);
            self.effect_named("rotate");
        }
        if d0 & 8 != 0 {
            self.turned_right = true;
            d1 = d1.wrapping_sub(self.turn_rate);
            self.effect_named("rotate");
        }
        // normal control: down turns round, fire thrusts; alternative: fire turns round, up thrusts
        let (flip, thrust) = if self.options[4] != 0 {
            (FIRE, 1)
        } else {
            (2, FIRE)
        };
        if d0 & flip != 0 && d2 & flip == 0 {
            d1 = d1.wrapping_add(0x8000);
            self.effect_named("turn_round");
        }
        self.objs[i].w22 = d1;
        if d0 & thrust != 0 {
            // sixteen directions: (angle >> 9) & $FFF8 indexes eight-byte entries
            let (tx, ty) = self.a.tables.thrust[(d1 >> 12) as usize];
            let o = &mut self.objs[i];
            o.vx = o.vx.wrapping_add(tx);
            o.vy = o.vy.wrapping_add(ty);
        }
        self.collide(i);
    }

    /// set_flag_by_global_range (+$4B3A): the demo's ships thrust at set moments of some instruction levels.
    fn demo_thrust(&mut self) {
        let f = self.f16() as i16;
        let l = self.level;
        self.demo_in = 0;
        if l > 0x33 && l != 0x37 && f >= 0x32 && (f < 0x46 || (l == 0x36 && f <= 0x78)) {
            self.demo_in = 0x80;
        }
    }

    /// The exploding ship (+$4924): its rows spread a pixel further each pass; after 32 a life is lost.
    fn update_exploding(&mut self, i: usize) -> Go {
        let o = &mut self.objs[i];
        o.set_l22(o.l22().wrapping_add(0x1_0000));
        if (o.w22 as i16) < 0x20 {
            return Go::On;
        }
        let mut d0 = self.lives;
        let b = (d0 as u8).wrapping_sub(1);
        d0 = (d0 & 0xFF00) | b as u16;
        if (b as i8) < 0 {
            d0 = d0.wrapping_add(0xFE0A);
        }
        if !self.trainer[0] {
            self.lives = d0;
        }
        if (d0 as i16) >= 0 {
            return Go::Restart;
        }
        self.objs[i].kind = 0;
        self.over_timer = 0x96;
        self.effect_named("game_over");
        Go::On
    }

    /// A tether (+$496C): a spring between two balls (or a ball and a ship) that pulls them to 64 pixels apart.
    fn update_tether(&mut self, i: usize) {
        let a = ((self.objs[i].x >> 16) as u16 / 28) as usize;
        let b = ((self.objs[i].x & 0xFFFF) as u16 / 28) as usize;
        let ok = |k: u16| k == 0x14 || k == 0x2C || k == 4;
        if a >= OBJS || b >= OBJS || !ok(self.objs[a].kind) || !ok(self.objs[b].kind) {
            self.objs[i].kind = 0;
            return;
        }
        let dx = (self.objs[a].xi() as i32) - self.objs[b].xi() as i32;
        let dy = (self.objs[a].yi() as i32) - self.objs[b].yi() as i32;
        let (dx, dy) = (dx as i16 as i32, dy as i16 as i32);
        let d0 = muls(dx, dx).wrapping_add(muls(dy, dy));
        let d2 = isqrt(d0 as u32) as i16;
        let q = lo(divs(dx << 6, d2));
        let d6 = (((dx - q) as i16 as i32) << 16) >> 8;
        self.objs[a].vx = self.objs[a].vx.wrapping_sub(d6);
        self.objs[b].vx = self.objs[b].vx.wrapping_add(d6);
        let q = lo(divs(dy << 6, d2));
        let d7 = (((dy - q) as i16 as i32) << 16) >> 8;
        self.objs[a].vy = self.objs[a].vy.wrapping_sub(d7);
        self.objs[b].vy = self.objs[b].vy.wrapping_add(d7);
    }

    /// A ball (+$4BA6): its fuse burns down (pulsing faster near the end) and it explodes, costing energy.
    fn update_ball(&mut self, i: usize) {
        if self.mode != 1 {
            let o = &mut self.objs[i];
            o.w22 = o.w22.wrapping_sub(1);
            if o.w22 == 0 {
                self.ball_explodes(i);
                return;
            }
            o.w24 = o.w24.wrapping_sub(1);
            if o.w24 == 0 {
                o.w24 = ((o.w22 >> 6) + 1).min(5);
                let mut d0 = o.max;
                if d0 as i16 == o.size {
                    d0 += 2;
                }
                o.size = d0 as i16;
            }
        }
        self.drift(i);
    }

    /// A fragment (+$4B78): flies straight until it leaves the screen.
    fn update_fragment(&mut self, i: usize) {
        let o = &mut self.objs[i];
        let x = o.vx.wrapping_add(o.x);
        let y = o.vy.wrapping_add(o.y);
        if x < 0 || y < 0 {
            o.kind = 0;
            return;
        }
        o.x = x;
        o.y = y;
        if (x >> 16) >= 0x140 || (y >> 16) >= 0xC4 {
            o.kind = 0;
        }
    }

    /// l0_4BE0: glide (losing a thirty-second of the speed a pass), then the bars and the other objects.
    fn drift(&mut self, i: usize) {
        let o = &mut self.objs[i];
        o.x = o.x.wrapping_add(o.vx);
        o.y = o.y.wrapping_add(o.vy);
        o.vx = o.vx.wrapping_sub(o.vx >> 5);
        o.vy = o.vy.wrapping_sub(o.vy >> 5);
        self.collide(i);
    }

    /// l0_4BFA: bounce off the bars, then meet the objects already moved this pass.
    fn collide(&mut self, i: usize) {
        self.walls_vertical(i);
        self.walls_horizontal(i);
        let (d0, d1, d2) = (self.objs[i].xi(), self.objs[i].yi(), self.objs[i].size);
        let mut n = 0;
        while n < self.hit_list.len() {
            let j = self.hit_list[n];
            n += 1;
            let d5 = self.objs[j].size.wrapping_add(d2);
            let d3 = loop {
                let mut d = self.objs[j].xi().wrapping_sub(d0);
                if d < 0 {
                    d = -d;
                    if d >= 0x120 {
                        self.objs[j].x = self.objs[j].x.wrapping_add(0x140 << 16);
                        continue;
                    }
                } else if d >= 0x120 {
                    self.objs[j].x = self.objs[j].x.wrapping_sub(0x140 << 16);
                    continue;
                }
                break d;
            };
            if d3 > d5 {
                continue;
            }
            let d4 = loop {
                let mut d = self.objs[j].yi().wrapping_sub(d1);
                if d < 0 {
                    d = -d;
                    if d >= 0xA4 {
                        self.objs[j].y = self.objs[j].y.wrapping_add(0xC4 << 16);
                        continue;
                    }
                } else if d >= 0xA4 {
                    self.objs[j].y = self.objs[j].y.wrapping_sub(0xC4 << 16);
                    continue;
                }
                break d;
            };
            if d4 > d5 {
                continue;
            }
            let dist = mulu(d3 as i32, d3 as i32).wrapping_add(mulu(d4 as i32, d4 as i32));
            if dist > mulu(d5 as i32, d5 as i32) {
                continue;
            }
            self.meet(i, j, dist);
            return;
        }
        self.hit_list.push(i);
    }

    /// Two objects touch (+$4C82): pods and sparks are taken by a ship, same colours destroy each other (scoring),
    /// anything else bounces, and two balls of different colours make a pod of the third.
    fn meet(&mut self, i: usize, j: usize, dist: u32) {
        let (si, sj) = (self.objs[i].kind, self.objs[j].kind);
        if si == 0x28 || si == 0x20 {
            if sj != 4 {
                return self.bounce(i, j, dist);
            }
            self.objs[i].kind = 0;
            let c = self.objs[i].colour;
            return self.pod_taken(i, c);
        }
        if si == 0x34 {
            if sj != 4 {
                return self.bounce(i, j, dist);
            }
            self.objs[i].kind = 0;
            self.kill_touch = true;
            return;
        }
        if sj == 0x28 || sj == 0x20 || sj == 0x34 {
            return self.bounce(i, j, dist);
        }
        let same = if self.kill_touch && sj == 4 && si != 4 {
            None
        } else if self.objs[i].colour == self.objs[j].colour {
            Some(true)
        } else {
            Some(false)
        };
        match same {
            None => {
                self.objs[i].kind = 0x1C;
                self.effect_named("same_colour");
            }
            Some(true) => {
                let sc =
                    [0x2_0000u32, 0x2_0500, 0x3_0000][((self.objs[i].colour >> 4) as usize).min(2)];
                self.add_score(sc);
                self.objs[j].kind = 0x1C;
                self.objs[i].kind = 0x1C;
                self.effect_named("same_colour");
            }
            Some(false) => self.bounce(i, j, dist),
        }
    }

    /// A pod taken by a ship: what it gives depends on the level's bonus kind (+$4D0E).
    fn pod_taken(&mut self, i: usize, c: u16) {
        match self.bonus {
            0 => {
                self.energy_due = self.energy_due.wrapping_sub(((c >> 4) + 8) as i16);
                self.effect_named("pod_taken");
                self.pod_count = self.pod_count.wrapping_add(1);
                if self.pod_count == 4 {
                    self.pods4 = true;
                }
            }
            4 => {
                if c == 0x20 {
                    self.pod_count = self.pod_count.wrapping_add(1);
                    let mut d0 = self.chain + 1;
                    if d0 >= 10 {
                        d0 = 0x100;
                    }
                    self.chain = d0;
                    self.add_bonus_and_score((d0 as u32) << 16);
                    self.effect_named("pod_taken");
                    for k in i + 1..OBJS {
                        let kd = self.objs[k].kind;
                        if kd == 0xFFFF {
                            break;
                        }
                        if kd == 0x20 {
                            self.objs[k].colour = 0x20;
                            break;
                        }
                    }
                } else {
                    self.chain = 0;
                    self.effect_named("pod_taken");
                }
            }
            8 => {
                if c == 0x20 {
                    self.effect_named("pod_taken");
                    self.add_bonus_and_score(0x5_0000);
                } else {
                    self.f406 = true;
                }
            }
            _ => {
                self.effect_named("pod_taken");
                let d2 = self.frame.wrapping_sub(self.pod_frame);
                self.pod_frame = self.frame;
                let q = divu(d2, 50) as u16;
                let d0 = if q == 0 {
                    Some(0x100_0000u32)
                } else {
                    let v = 10i16 - q as i16;
                    if v < 0 {
                        None
                    } else {
                        Some((v as u32) << 16)
                    }
                };
                if let Some(d0) = d0 {
                    self.add_bonus_and_score(d0);
                }
                for k in i + 1..OBJS {
                    let kd = self.objs[k].kind;
                    if kd == 0xFFFF {
                        break;
                    }
                    if kd == 0x24 {
                        self.objs[k].kind = 0x28;
                        break;
                    }
                }
            }
        }
    }

    /// l0_4D30: push the two apart along the line between their centres and exchange their speeds along it.
    fn bounce(&mut self, i: usize, j: usize, dist: u32) {
        let d2 = isqrt(dist) as i32;
        let ov = (self.objs[i].size as i32 + self.objs[j].size as i32 - d2) as u16 as u32;
        let q = lo(divu(ov << 16, (d2 << 4) as u16) as i32);
        let push = |a: i16, b: i16| -> i16 {
            let d0 = muls((a.wrapping_sub(b)) as i32, q);
            ((d0 << 4).wrapping_add(0x8000) >> 16) as i16
        };
        let px = push(self.objs[i].xi(), self.objs[j].xi());
        self.objs[i].x = ((self.objs[i].xi().wrapping_add(px) as i32) << 16) & !0xFFFF;
        let py = push(self.objs[i].yi(), self.objs[j].yi());
        self.objs[i].y = ((self.objs[i].yi().wrapping_add(py) as i32) << 16) & !0xFFFF;
        let d4 = (self.objs[i].size as i32 + self.objs[j].size as i32) as i16;
        let ux = lo(divs(self.objs[i].x.wrapping_sub(self.objs[j].x) >> 2, d4));
        let uy = lo(divs(self.objs[i].y.wrapping_sub(self.objs[j].y) >> 2, d4));
        let rvx = self.objs[i].vx.wrapping_sub(self.objs[j].vx) >> 5;
        let rvy = self.objs[i].vy.wrapping_sub(self.objs[j].vy) >> 5;
        let mut d0 = (muls(rvx, ux).wrapping_add(muls(rvy, uy)).wrapping_mul(2) >> 16) as i16;
        if self.objs[j].kind >= 0x28 {
            d0 = d0.wrapping_sub(0x400);
        }
        if self.objs[i].kind >= 0x28 {
            d0 = d0.wrapping_sub(0x400);
        }
        let fy = ((muls(uy, d0 as i32) >> 16) as i16 as i32) << 8;
        let fx = ((muls(ux, d0 as i32) >> 16) as i16 as i32) << 8;
        self.objs[i].vx = self.objs[i].vx.wrapping_sub(fx);
        self.objs[i].vy = self.objs[i].vy.wrapping_sub(fy);
        self.objs[j].vx = self.objs[j].vx.wrapping_add(fx);
        self.objs[j].vy = self.objs[j].vy.wrapping_add(fy);
        if self.objs[j].kind == 4 {
            return self.bump();
        }
        if self.objs[j].kind != 0x14 || self.objs[i].kind != 0x14 || self.mode == 1 {
            return;
        }
        let Some(n) = self.find_free_object() else {
            return;
        };
        self.pod_count = 0;
        self.effect_named("pod_made");
        self.f405 = true;
        let cj = self.objs[j].colour;
        let ci = self.objs[i].colour;
        let sj = self.objs[j].size as i32;
        let x = muls(ux, sj).wrapping_shl(2).wrapping_add(self.objs[j].x);
        let y = muls(uy, sj).wrapping_shl(2).wrapping_add(self.objs[j].y);
        let vx = self.objs[i].vx.wrapping_add(self.objs[j].vx) >> 1;
        let vy = self.objs[i].vy.wrapping_add(self.objs[j].vy) >> 1;
        self.objs[n] = Obj {
            kind: 0x28,
            x,
            y,
            size: -4,
            colour: 0x30u16.wrapping_sub(cj).wrapping_sub(ci),
            vx,
            vy,
            w22: self.objs[n].w22,
            w24: self.objs[n].w24,
            max: 4,
        };
    }

    /// l0_5108: one of eight bump sounds at random.
    fn bump(&mut self) {
        let r = (random_next(&mut self.rng) & 0x1C) as usize >> 2;
        let a = self.a.clone();
        self.effect(&a.bump[r]);
    }

    /// update_object_motion (+$4F7E): the vertical bars (a flat side, or a round end).
    fn walls_vertical(&mut self, i: usize) {
        let (d0, d1) = (self.objs[i].xi(), self.objs[i].yi());
        let d2 = self.objs[i].size.wrapping_add(8);
        for b in self.vbars.clone() {
            let (bx, by, len) = (b[0], b[1], b[2]);
            let d3 = bx.wrapping_sub(d0).abs();
            if d3 >= d2 {
                continue;
            }
            let mut d4 = by.wrapping_sub(d1);
            if d4 <= 0 {
                d4 = d4.wrapping_add(len);
                if d4 >= 0 {
                    // the side: put it beside the bar and reflect its speed
                    let off = if bx.wrapping_sub(d0) >= 0 { -d2 } else { d2 };
                    let o = &mut self.objs[i];
                    o.x = ((bx.wrapping_add(off) as i32) << 16) & !0xFFFF;
                    o.vx = o.vx.wrapping_neg();
                    return self.bump();
                }
                d4 = -d4;
            }
            if d4 >= d2 {
                continue;
            }
            let dist = mulu(d3 as i32, d3 as i32).wrapping_add(mulu(d4 as i32, d4 as i32));
            if dist > mulu(d2 as i32, d2 as i32) {
                continue;
            }
            let ey = if by > d1 { by } else { by.wrapping_add(len) };
            return self.bounce_point(i, dist, bx, ey);
        }
    }

    /// update_sprite_motion_state (+$4FF0): the horizontal bars.
    fn walls_horizontal(&mut self, i: usize) {
        let (d0, d1) = (self.objs[i].xi(), self.objs[i].yi());
        let d2 = self.objs[i].size.wrapping_add(8);
        for b in self.hbars.clone() {
            let (bx, by, len) = (b[0], b[1], b[2]);
            let d4 = by.wrapping_sub(d1).abs();
            if d4 >= d2 {
                continue;
            }
            let mut d3 = bx.wrapping_sub(d0);
            if d3 <= 0 {
                d3 = d3.wrapping_add(len);
                if d3 >= 0 {
                    let off = if by.wrapping_sub(d1) >= 0 { -d2 } else { d2 };
                    let o = &mut self.objs[i];
                    o.y = ((by.wrapping_add(off) as i32) << 16) & !0xFFFF;
                    o.vy = o.vy.wrapping_neg();
                    return self.bump();
                }
                d3 = -d3;
            }
            if d3 >= d2 {
                continue;
            }
            let dist = mulu(d3 as i32, d3 as i32).wrapping_add(mulu(d4 as i32, d4 as i32));
            if dist > mulu(d2 as i32, d2 as i32) {
                continue;
            }
            let ex = if bx > d0 { bx } else { bx.wrapping_add(len) };
            return self.bounce_point(i, dist, ex, by);
        }
    }

    /// l0_505E: bounce off a bar's round end at (px, py) (a fixed point: the whole exchange goes to the object).
    fn bounce_point(&mut self, i: usize, dist: u32, px: i16, py: i16) {
        let d2 = isqrt(dist) as i32;
        let ov = (self.objs[i].size as i32 + 8 - d2) as u16 as u32;
        let q = lo(divu(ov << 16, (d2 << 4) as u16) as i32);
        let push = |a: i16, b: i16| -> i16 {
            let d0 = muls((a.wrapping_sub(b)) as i32, q);
            ((d0 << 4).wrapping_add(0x8000) >> 16) as i16
        };
        let ax = push(self.objs[i].xi(), px);
        self.objs[i].x = ((self.objs[i].xi().wrapping_add(ax) as i32) << 16) & !0xFFFF;
        let ay = push(self.objs[i].yi(), py);
        self.objs[i].y = ((self.objs[i].yi().wrapping_add(ay) as i32) << 16) & !0xFFFF;
        let d4 = (self.objs[i].size as i32 + 8) as i16;
        let ux = lo(divs(
            self.objs[i].x.wrapping_sub((px as i32) << 16) >> 2,
            d4,
        ));
        let uy = lo(divs(
            self.objs[i].y.wrapping_sub((py as i32) << 16) >> 2,
            d4,
        ));
        let rvx = self.objs[i].vx >> 5;
        let rvy = self.objs[i].vy >> 5;
        let mut d0 = (muls(rvx, ux).wrapping_add(muls(rvy, uy)).wrapping_mul(2) >> 16) as i16;
        if self.objs[i].kind >= 0x28 {
            d0 = d0.wrapping_sub(0x400);
        }
        let fy = (((muls(uy, d0 as i32) >> 16) as i16 as i32) << 8).wrapping_mul(2);
        let fx = (((muls(ux, d0 as i32) >> 16) as i16 as i32) << 8).wrapping_mul(2);
        self.objs[i].vx = self.objs[i].vx.wrapping_sub(fx);
        self.objs[i].vy = self.objs[i].vy.wrapping_sub(fy);
        self.bump();
    }

    /// find_free_object (+$43FC): a free slot from the third on (none if the table is full).
    pub fn find_free_object(&self) -> Option<usize> {
        (2..OBJS)
            .find(|&k| self.objs[k].kind == 0)
            .filter(|&k| self.objs[k].kind != 0xFFFF)
    }

    /// self_11 (+$517A): a ball explodes and costs energy (a quarter of its colour value and 32).
    fn ball_explodes(&mut self, i: usize) {
        let c = self.objs[i].colour;
        self.energy_due = self.energy_due.wrapping_add(((c >> 2) + 0x20) as i16);
        self.burst(i);
    }

    /// self_12 (+$5188): an explosion: eight fragments of half the size fly out.
    fn burst(&mut self, i: usize) {
        self.effect_named("ball_explodes");
        self.f404 = true;
        let o = self.objs[i];
        self.objs[i].kind = 0;
        let fr = self.a.tables.fragments.clone();
        for (vx, vy) in fr {
            let Some(n) = self.find_free_object() else {
                return;
            };
            let s = &mut self.objs[n];
            s.kind = 0x18;
            s.x = o.x;
            s.y = o.y;
            s.size = ((o.size as u16) >> 1) as i16;
            s.colour = o.colour;
            s.vx = vx;
            s.vy = vy;
        }
    }

    /// initialize_extra_output_block (+$52A0): a firework at a random place in a random colour.
    pub fn initialize_extra_output_block(&mut self) {
        let Some(n) = self.find_free_object() else {
            return;
        };
        let x = random_below(&mut self.rng, 0x140);
        let y = random_below(&mut self.rng, 0xC4);
        let c = loop {
            let r = random_next(&mut self.rng) as u16 & 0x30;
            if r != 0x30 {
                break r;
            }
        };
        let s = &mut self.objs[n];
        s.kind = 0x30;
        s.x = (s.x & 0xFFFF) | (x as i32) << 16;
        s.y = (s.y & 0xFFFF) | (y as i32) << 16;
        s.size = 0;
        s.max = 0xC;
        s.colour = c;
    }

    /// clear_object_state (+$5138): the secret level's spark appears somewhere at random.
    pub fn clear_object_state(&mut self) {
        let Some(n) = self.find_free_object() else {
            return;
        };
        let x = random_below(&mut self.rng, 0x140);
        let y = random_below(&mut self.rng, 0xC4);
        let s = &mut self.objs[n];
        s.kind = 0x34;
        s.x = (s.x & 0xFFFF) | (x as i32) << 16;
        s.y = (s.y & 0xFFFF) | (y as i32) << 16;
        s.vx = 0;
        s.vy = 0;
        s.size = 0;
        s.max = 4;
        s.colour = 0;
        s.w24 = 0x177;
    }

    /// explode_ships (+$48E4).
    pub fn explode_ships(&mut self) {
        if self.exploding {
            return;
        }
        self.effect_named("ship_explodes");
        if self.options[1] != 0 {
            self.objs[0].kind = 8;
            self.objs[0].set_l22(0x1_0000);
        }
        self.objs[1].kind = 8;
        self.objs[1].set_l22(0x1_0000);
        self.exploding = true;
    }

    /// The drawing pass (+$53CA); drawing also brings positions back onto the screen (swap_two_table_entries).
    fn draw_obj(&mut self, i: usize, k: u16) {
        match k {
            0x04 => {
                self.wrap_pos(i);
                let o = self.objs[i];
                self.draw_ship(
                    o.xi() as i32,
                    o.yi() as i32,
                    (o.w22 >> 12) as usize,
                    o.colour != 0xFFFF,
                );
            }
            0x08 => {
                self.wrap_pos(i);
                let o = self.objs[i];
                self.draw_explosion(o.xi() as i32, o.yi() as i32, o.l22(), o.colour != 0xFFFF);
            }
            0x0C | 0x24 => {}
            0x10 => {
                let a = ((self.objs[i].x >> 16) as u16 / 28) as usize;
                let b = ((self.objs[i].x & 0xFFFF) as u16 / 28) as usize;
                if a < OBJS && b < OBJS {
                    let w = |v: i16, m: i16| {
                        if v < 0 {
                            v + m
                        } else if v >= m {
                            v - m
                        } else {
                            v
                        }
                    };
                    let (x0, y0) = (w(self.objs[a].xi(), 320), w(self.objs[a].yi(), 196));
                    let (x1, y1) = (w(self.objs[b].xi(), 320), w(self.objs[b].yi(), 196));
                    self.draw_tether(x0 as i32, y0 as i32, x1 as i32, y1 as i32);
                }
            }
            _ => {
                let count = !(k == 0x20 && self.bonus == 8 && self.objs[i].colour != 0x20);
                if count {
                    self.alive += 1;
                    self.last_obj = i;
                }
                self.wrap_pos(i);
                let o = self.objs[i];
                if o.size >= 0 {
                    self.draw_ball(o.xi() as i32, o.yi() as i32, o.size, o.colour);
                }
            }
        }
    }

    /// swap_two_table_entries (+$5402): the position's whole part brought into the screen (one wrap either way).
    fn wrap_pos(&mut self, i: usize) {
        let w = |v: i16, m: i16| {
            if v < 0 {
                v + m
            } else if v >= m {
                v - m
            } else {
                v
            }
        };
        let o = &mut self.objs[i];
        let (x, y) = (w(o.xi(), 320), w(o.yi(), 196));
        o.x = (x as i32) << 16 | (o.x & 0xFFFF);
        o.y = (y as i32) << 16 | (o.y & 0xFFFF);
    }
}
