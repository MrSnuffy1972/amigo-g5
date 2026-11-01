//! Blocks and what comes out of them: free diamonds, diamond blocks, power blocks and the item that rises out of
//! them, breakable bricks, the hidden bricks that drop a block, multi-diamond blocks and the warp blocks; and their
//! animations. Each method is named after the recovered routine.

use crate::giana::{Enemy, Giana, NONE};

/// Object lists by name: bonus (diamond blocks) and bonus1 (power blocks).
const BONUS: usize = 2;
const BONUS1: usize = 3;

impl Giana {
    fn wd(&self) -> isize {
        self.st.w as isize
    }

    /// The first of the three tiles above her head that is `t`.
    fn above_head(&self, t: u8) -> Option<isize> {
        let c = self.player_cell? - self.wd();
        (0..3).map(|k| c + k).find(|&p| self.map_get(p) == t)
    }

    fn award_diamond(&mut self) {
        self.add_diamond();
        self.pending = 0x25;
        self.add_points();
        self.play_sound_effect(3);
    }

    /// `update_block_hits`.
    pub fn update_block_hits(&mut self) {
        self.hit_diamond_block();
        self.break_brick();
        self.break_hidden_brick();
        self.hit_multi_diamond_block();
        self.hit_power_block();
        self.hit_warp_blocks();
        self.update_block_animations();
    }

    /// `collect_diamond_tile`: a free diamond she touches.
    pub fn collect_diamond_tile(&mut self) {
        let Some(k) = (0..9).rev().find(|&k| self.tiles[k] == 2) else {
            return;
        };
        let Some(c) = self.player_cell else {
            return;
        };
        let mut a = c + (k % 3) as isize + (k / 3) as isize * self.wd();
        let i = (self.map_get(a) as i32 - 0xA6) * 2;
        let get = |j: i32| {
            if j < 0 {
                0
            } else {
                self.a.diamond_off.get(j as usize).copied().unwrap_or(0)
            }
        };
        let (dx, dy) = (get(i), get(i + 1));
        a += dx as isize + dy as isize * self.wd();
        let w = self.wd();
        for p in [a, a + 1, a + w, a + w + 1] {
            self.map_set(p, 0);
        }
        let (x, y) = self.cell_xy(a);
        let img = self.a.diamond_gone;
        self.draw_bob(img, x, y, false);
        self.award_diamond();
    }

    /// `hit_diamond_block`: her head hits a diamond block; it empties, bumps and gives a diamond.
    pub fn hit_diamond_block(&mut self) {
        if self.head_free {
            return;
        }
        let Some(p) = self.above_head(0x47) else {
            return;
        };
        self.map_set(p, 0x46);
        let (x, y) = self.cell_xy(p - 1);
        self.add_collected_diamond(x, y);
        self.start_block_bump(x, y);
        self.mark_block_used(BONUS, x, 0x9F - y);
        self.award_diamond();
    }

    /// `start_block_bump`.
    pub fn start_block_bump(&mut self, x: i32, y: i32) {
        let k = if self.bump[0].0 == NONE { 0 } else { 1 };
        self.bump[k] = (x, y);
    }

    /// `hit_power_block`: the item rises out of it.
    pub fn hit_power_block(&mut self) {
        if self.head_free {
            return;
        }
        let Some(p) = self.above_head(0xF2) else {
            return;
        };
        self.map_set(p, 0x46);
        let (x, y) = self.cell_xy(p - 1);
        self.start_block_bump(x, y);
        self.set_power_item_start(x, y);
        self.mark_block_used(BONUS1, x, 0x9F - y);
        self.add_diamond();
    }

    /// `set_power_item_start`.
    pub fn set_power_item_start(&mut self, x: i32, y: i32) {
        self.item_pos = (x, y);
    }

    /// `hit_warp_blocks`: rising into the hidden warp blocks skips stages.
    pub fn hit_warp_blocks(&mut self) {
        let Some(c) = self.player_cell else {
            return;
        };
        if self.py < 0x30 {
            return;
        }
        let w = self.wd();
        let a = c - w + 1;
        if self.map_get(a) != 0xF7 || self.jumping != 0x80 {
            return;
        }
        let mut p = a;
        while self.map_get(p - 1) == 0xF7 {
            p -= 1;
        }
        p -= w;
        if self.map_get(p) != 0xF7 {
            p += w;
        }
        for r in 0..2 {
            for k in 0..3 {
                self.map_set(p + r * w + k, 0x46);
            }
        }
        let (x, y) = self.cell_xy(p);
        self.start_block_bump(x, y);
        self.warp = true;
        self.cleared = true;
        self.play_sound_effect(6);
    }

    /// `break_brick`: powered up, her head breaks a brick.
    pub fn break_brick(&mut self) {
        if self.powered == 0 || self.head_free {
            return;
        }
        let Some(p) = self.above_head(0xEB) else {
            return;
        };
        self.map_set(p, 0x4C);
        let (x, y) = self.cell_xy(p - 1);
        self.add_breaking_brick(x, y);
        self.play_sound_effect(9);
    }

    /// `break_hidden_brick`: the second brick type, which lets a block drop.
    pub fn break_hidden_brick(&mut self) {
        if self.powered == 0 || self.head_free {
            return;
        }
        let Some(p) = self.above_head(0x44) else {
            return;
        };
        self.map_set(p, 0x4C);
        let (x, y) = self.cell_xy(p - 1);
        let k = if self.debris[0].0 == NONE { 0 } else { 1 };
        self.debris[k] = (x, y);
    }

    /// `hit_multi_diamond_block`: a diamond each hit until its count runs out.
    pub fn hit_multi_diamond_block(&mut self) {
        if self.powered == 0 || self.head_free {
            return;
        }
        let Some(q) = self.above_head(0xEA) else {
            return;
        };
        let p = q - 1;
        let (x, y) = self.cell_xy(p);
        let v = self.map_get(p).wrapping_sub(1);
        self.map_set(p, v);
        if (v as i8) > 0x3E {
            self.start_multi_block_animation(x, y);
            self.add_collected_diamond(x, y);
            self.award_diamond();
            return;
        }
        self.multi = [(NONE, 0); 2];
        self.multi_t = [0; 2];
        self.map_set(p, 0x43);
        self.map_set(p + 1, 0x43);
        let img = self.a.multi_done;
        self.draw_bob(img, x, y, false);
        self.add_collected_diamond(x, y);
        self.award_diamond();
    }

    /// `mark_block_used`: a block in an animated-tile list stops shimmering.
    pub fn mark_block_used(&mut self, list: usize, x: i32, y: i32) {
        let Some(l) = self.objects[list].as_mut() else {
            return;
        };
        for e in l.iter_mut() {
            if (e.1 & 0x7FFF) as i32 == (y as u16) as i32 && e.0 as i32 == x {
                if e.1 & 0x8000 == 0 {
                    e.1 |= 0x8000;
                }
                return;
            }
        }
    }

    /// `update_block_animations`.
    pub fn update_block_animations(&mut self) {
        self.raise_power_item();
        self.bump_animation(0);
        self.bump_animation(1);
        self.multi_block_animation(0);
        self.multi_block_animation(1);
        self.animate_breaking_bricks();
        self.animate_brick_debris();
        self.update_dropped_blocks();
    }

    /// `raise_power_item`: the item rises out of its block for 24 frames and then moves as an object.
    pub fn raise_power_item(&mut self) {
        if self.item_pos.0 == NONE {
            return;
        }
        let x = self.item_pos.0;
        let y = self.item_pos.1 - self.item_rise;
        let i = (self.item_idx.max(0) as usize).min(self.a.items.len() - 1);
        let t = self.a.items[i].clone();
        let img = self.a.anims[t.anim[0]][0].img;
        self.draw_bob(img, x, y, true);
        self.item_rise += 1;
        if self.item_rise < 0x18 {
            return;
        }
        self.item_rise = 0;
        self.item_pos.0 = NONE;
        if let Some(e) = self.enemies.iter_mut().find(|e| e.state == 0) {
            *e = Enemy::from_template(&t, x, y);
        }
    }

    /// `bump_animation_a` and `bump_animation_b`; slot a ends a warp by moving on two stages.
    pub fn bump_animation(&mut self, k: usize) {
        if self.bump[k].0 == NONE {
            return;
        }
        let (x, y) = self.bump[k];
        let img = self.a.bump[((self.bump_t[k] >> 1) as usize).min(7)];
        self.draw_bob(img, x, y, false);
        self.bump_t[k] += 1;
        if self.bump_t[k] < 16 {
            return;
        }
        self.bump_t[k] = 0;
        self.bump[k].0 = NONE;
        if k == 0 && self.warp {
            self.next_stage();
            self.next_stage();
            self.life_over = true;
            self.cleared = true;
        }
    }

    /// `multi_block_animation_a` and `multi_block_animation_b`.
    pub fn multi_block_animation(&mut self, k: usize) {
        if self.multi[k].0 == NONE {
            return;
        }
        let (x, y) = self.multi[k];
        let img = self.a.multi[((self.multi_t[k] >> 1) as usize).min(7)];
        self.draw_bob(img, x, y, false);
        self.multi_t[k] += 1;
        if self.multi_t[k] < 16 {
            return;
        }
        self.multi_t[k] = 0;
        self.multi[k].0 = NONE;
    }

    /// `start_multi_block_animation`.
    pub fn start_multi_block_animation(&mut self, x: i32, y: i32) {
        let k = if self.multi[0].0 == NONE { 0 } else { 1 };
        self.multi[k] = (x, y);
    }

    /// `animate_collected_diamonds`: each diamond flies up its path and vanishes.
    pub fn animate_collected_diamonds(&mut self) {
        for i in 0..self.flying.len() {
            let [x, y, t] = self.flying[i];
            if x == 0 {
                continue;
            }
            let p = &self.a.diamond_path;
            let dx = p.get(2 * t as usize).copied().unwrap_or(0);
            let dy = p.get(2 * t as usize + 1).copied().unwrap_or(0);
            self.flying[i][0] += dx;
            self.flying[i][1] -= dy;
            let img = self.a.flying_diamond;
            self.draw_bob(img, x, y, true);
            self.flying[i][2] += 1;
            if self.flying[i][2] >= 0x4C {
                self.flying[i][0] = 0;
            }
        }
    }

    /// `add_collected_diamond`.
    pub fn add_collected_diamond(&mut self, x: i32, y: i32) {
        if let Some(e) = self.flying.iter_mut().find(|e| e[0] == 0) {
            *e = [x + 8, y - 8, 0];
        }
    }

    /// `animate_breaking_bricks`: the brick's tiles leave the map partway through.
    pub fn animate_breaking_bricks(&mut self) {
        for i in 0..self.breaking.len() {
            let [x, y, t] = self.breaking[i];
            if x == 0 {
                continue;
            }
            let d0 = x - 2;
            let img = self.a.breaking[((t >> 2) as usize).min(7)];
            self.draw_bob(img, d0, y, false);
            self.breaking[i][2] += 1;
            let t = self.breaking[i][2];
            if t == 8 {
                let w = self.wd();
                let o = (d0 >> 3) as isize + (y >> 3) as isize * w;
                for r in 0..2 {
                    for k in 0..3 {
                        self.map_set(o + r * w + k, 0);
                    }
                }
            } else if t >= 0x20 {
                self.breaking[i][0] = 0;
            }
        }
    }

    /// `add_breaking_brick`.
    pub fn add_breaking_brick(&mut self, x: i32, y: i32) {
        if let Some(e) = self.breaking.iter_mut().find(|e| e[0] == 0) {
            *e = [x + 2, y, 0];
        }
    }

    /// `animate_brick_debris`: slot a, then slot b. Slot b's end test reads slot a's counter and clears slot a,
    /// as in the original; its frame is held at the last debris frame rather than reading past the table.
    pub fn animate_brick_debris(&mut self) {
        let n = self.a.debris.len() - 1;
        if self.debris[0].0 != NONE {
            let (x, y) = self.debris[0];
            let img = self.a.debris[((self.debris_t[0] >> 1) as usize).min(n)];
            self.draw_bob(img, x, y, false);
            self.debris_t[0] += 1;
            if self.debris_t[0] < 8 {
                return;
            }
            self.debris_t[0] = 0;
            self.debris[0].0 = NONE;
            self.add_dropped_block(x, y);
            return;
        }
        if self.debris[1].0 == NONE {
            return;
        }
        let (x, y) = self.debris[1];
        let img = self.a.debris[((self.debris_t[1] >> 1) as usize).min(n)];
        self.draw_bob(img, x, y, false);
        self.debris_t[1] += 1;
        if self.debris_t[0] < 8 {
            return;
        }
        self.debris_t[1] = 0;
        self.debris[0].0 = NONE;
        self.add_dropped_block(x, y);
    }

    /// `update_dropped_blocks`: a dropped block falls until it lands on solid ground, then becomes block tiles.
    pub fn update_dropped_blocks(&mut self) {
        let w = self.wd();
        for i in 0..self.dropped.len() {
            let [mut d0, d1, t] = self.dropped[i];
            if d0 == 0 {
                continue;
            }
            let d4 = ((d1 >> 3) + 2) as isize * w + (d0 >> 3) as isize;
            let landed = self.class(d4 + 1) == 1;
            if !landed {
                if self.class(d4) == 1 {
                    d0 += 8;
                    self.dropped[i][0] += 8;
                } else if self.class(d4 + 2) == 1 {
                    d0 -= 8;
                    self.dropped[i][0] -= 8;
                }
            } else {
                if self.scroll - 0x1E >= d0 {
                    self.dropped[i][0] = 0;
                    continue;
                }
                if t == 0 {
                    continue;
                }
                let y = d1 & !7;
                self.dropped[i][1] = y;
                for r in 1..3 {
                    for k in 0..3 {
                        self.map_set(d4 - r * w + k, 0x43);
                    }
                }
                let img = self.a.dropped_block;
                self.draw_bob(img, d0, y, false);
                self.dropped[i][2] = 0;
                self.play_sound_effect(5);
                continue;
            }
            if t == 0 {
                for r in 1..3 {
                    for k in 0..3 {
                        self.map_set(d4 - r * w + k, 0);
                    }
                }
                let img = self.a.dropped_gone;
                self.draw_bob(img, d0, d1, false);
            }
            let f = self.a.drop_fall.get(2 * t as usize).copied().unwrap_or(0);
            self.dropped[i][1] += f;
            let img = self.a.dropped_block;
            self.draw_bob(img, d0, d1, true);
            self.dropped[i][2] += 1;
            if self.dropped[i][1] >= 0xA0 {
                self.dropped[i][0] = 0;
            }
        }
    }

    /// `add_dropped_block`.
    pub fn add_dropped_block(&mut self, x: i32, y: i32) {
        if let Some(e) = self.dropped.iter_mut().find(|e| e[0] == 0) {
            *e = [x, y, 0];
        }
    }
}
