//! Enemies and items (spawning as they scroll into view, collisions with Giana and her shot, movement against the
//! map or along a path, animation), the animated map tiles, and building a stage. Each method is named after the
//! recovered routine.

use std::sync::Arc;

use crate::assets::Template;
use crate::giana::{bcd_add3, Enemy, Giana, Tgt, NONE};

impl Enemy {
    /// A record filled from a type template (`spawn_enemy`, `raise_power_item`).
    pub fn from_template(t: &Template, x: i32, y: i32) -> Enemy {
        Enemy {
            state: t.kind,
            x,
            y,
            dx: t.dx,
            anim: t.anim,
            points: t.points,
            movement: t.movement,
            path: Arc::new(t.path.clone()),
            w: 0,
            h: 0,
            tick: 0,
            shift: t.shift,
            xoff: t.xoff,
            yoff: t.yoff,
            step: 1,
            idx: 0,
            dy: t.dy,
            hp: t.hp,
        }
    }
}

fn path_at(p: &[i32], i: i32) -> i32 {
    if i < 0 {
        return 0;
    }
    p.get(i as usize).copied().unwrap_or(-1)
}

impl Giana {
    /// `update_enemies`.
    pub fn update_enemies(&mut self) {
        if self.in_bonus {
            return;
        }
        self.spawn_enemies_in_view();
        self.update_enemy_list();
    }

    /// `spawn_enemies_in_view`: each unused enemy whose column has just scrolled into view.
    pub fn spawn_enemies_in_view(&mut self) {
        let s = self.stage as usize;
        let Some(list) = self.a.enemy_lists.get(s).cloned() else {
            return;
        };
        let col = ((self.scroll + 0x148) as u16) >> 3;
        for (i, &(c, r, t)) in list.iter().enumerate() {
            if self.enemy_spawned.get(i).copied().unwrap_or(true) || c != col {
                continue;
            }
            self.spawn_enemy(i, c, r, t);
        }
    }

    /// `spawn_enemy`.
    pub fn spawn_enemy(&mut self, i: usize, c: u16, r: u16, t: u16) {
        self.enemy_spawned[i] = true;
        let Some(slot) = self.enemies.iter().position(|e| e.state == 0) else {
            return;
        };
        let Some(tp) = self.a.templates.get(t as usize) else {
            return;
        };
        let mut e = Enemy::from_template(tp, c as i32 * 8, r as i32 * 8 - 0x38);
        if e.movement == 6 {
            e.step = path_at(&e.path, 0);
        }
        self.enemies[slot] = e;
    }

    /// `update_enemy_list`.
    pub fn update_enemy_list(&mut self) {
        for i in 0..self.enemies.len() {
            if self.enemies[i].state == 0 {
                continue;
            }
            self.enemy_collisions(i);
            self.enemy_map_movement(i);
            self.animate_enemy(i);
        }
    }

    /// `enemy_collisions`.
    pub fn enemy_collisions(&mut self, i: usize) {
        self.enemy_player_collision(i);
        self.enemy_shot_collision(i);
    }

    /// `enemy_player_collision`: landing on an enemy squashes it (points and a bounce); touching it otherwise kills
    /// her; touching an item collects it.
    pub fn enemy_player_collision(&mut self, i: usize) {
        let e = self.enemies[i].clone();
        if self.dying || e.state == 3 || e.state == 4 {
            return;
        }
        let d0 = self.px + self.scroll;
        let d2 = d0 + 16;
        let d1 = self.py;
        let d3 = d1 + 0x13;
        let a0 = e.x + e.xoff;
        let a2 = a0 + e.w;
        let a1 = e.y + 40 + e.yoff;
        let a3 = a1 + e.h;
        if a0 >= d2 || d0 >= a2 || a1 >= d3 || d1 >= a3 {
            return;
        }
        if e.state == 2 {
            self.play_sound_effect(7);
            self.enemies[i].state = 0;
            let k = (self.item_idx.max(0) as usize).min(self.a.item_flag.len() - 1);
            match self.a.item_flag[k] {
                0 => self.powered = self.powered.wrapping_add(1),
                1 => self.shot_level = self.shot_level.wrapping_add(1),
                2 => self.freezes = self.freezes.wrapping_add(1),
                3 => self.kill_alls = self.kill_alls.wrapping_add(1),
                4 => self.protection = self.protection.wrapping_add(1),
                _ => self.extra_life = self.extra_life.wrapping_add(1),
            }
            self.item_idx += 1;
            if self.item_idx == 8 {
                self.item_idx = 4;
            }
            return;
        }
        let stomp = d3 < a3
            && self.airborne
            && self.jumping != 0x80
            && self.falling != 0x80
            && e.state != 5
            && e.hp <= 1;
        if !stomp {
            self.dying = true;
            self.play_sound_effect(4);
            return;
        }
        self.jump_i = 0;
        self.fall_i = 0;
        self.falling = 0;
        self.jumping = 0x80;
        self.enemies[i].state = 4;
        self.play_sound_effect(1);
        self.score = bcd_add3(self.score, e.points);
        if e.movement != 6 {
            self.enemies[i].movement = 0;
        }
    }

    /// `enemy_shot_collision`: the shot bursts and the enemy loses a hit point, dying with points at zero.
    pub fn enemy_shot_collision(&mut self, i: usize) {
        let e = self.enemies[i].clone();
        if e.state > 1 {
            return;
        }
        let (d0, d1) = self.shot;
        let a0 = e.x + e.xoff;
        let a1 = e.y + e.yoff;
        if a0 >= d0 + 4 || d0 >= a0 + e.w || a1 >= d1 + 4 || d1 >= a1 + e.h {
            return;
        }
        self.play_sound_effect(1);
        self.explosion = (self.shot.0 - 2, self.shot.1 - 2);
        self.shot.0 = NONE;
        self.enemies[i].hp -= 1;
        if self.enemies[i].hp != 0 {
            return;
        }
        self.enemies[i].state = 3;
        self.score = bcd_add3(self.score, e.points);
        if e.movement != 6 {
            self.enemies[i].movement = 0;
        }
    }

    /// `enemy_map_movement`: turning at walls and edges and falling (movement types 0 to 2), or along a path table
    /// (3 to 6).
    pub fn enemy_map_movement(&mut self, i: usize) {
        let mut e = self.enemies[i].clone();
        if e.movement >= 3 {
            self.path_movement(&mut e);
            self.enemies[i] = e;
            return;
        }
        let w = self.st.w as isize;
        let end = 20 * w;
        let row = ((e.y >> 3) + 1) as u32;
        let mut a0 = (((e.x as u16) >> 3) as isize) + ((row * w as u32) as u16 as i16) as isize;
        let mut t = [0u8; 6];
        if a0 <= end {
            if a0 >= 0 {
                for (k, x) in t.iter_mut().take(3).enumerate() {
                    *x = self.class(a0 + k as isize);
                }
            }
            a0 += w;
            if a0 <= end && a0 >= 0 {
                for k in 0..3 {
                    t[3 + k] = self.class(a0 + k as isize);
                }
            }
        }
        self.enemy_tiles = t;
        if e.state == 3 || e.state == 4 {
            if t[4] == 1 {
                e.y &= !7;
            } else {
                e.y += 4;
            }
            self.enemies[i] = e;
            return;
        }
        let walls = |e: &mut Enemy| {
            if t[0] == 1 && t[1] != 1 {
                if e.dx < 0 {
                    e.dx = -e.dx;
                }
            } else if t[2] == 1 && t[1] != 1 && e.dx >= 0 {
                e.dx = -e.dx;
            }
        };
        let gravity = |e: &mut Enemy| {
            if t[3..6].contains(&1) {
                e.y &= !7;
            } else {
                e.y += 4;
            }
        };
        match e.movement {
            0 => {
                walls(&mut e);
                gravity(&mut e);
            }
            2 => walls(&mut e),
            _ => {
                walls(&mut e);
                if t[3] != 1 {
                    if e.dx < 0 {
                        e.dx = -e.dx;
                    }
                } else if t[5] != 1 && e.dx >= 0 {
                    e.dx = -e.dx;
                }
                gravity(&mut e);
            }
        }
        self.enemies[i] = e;
    }

    fn path_movement(&self, e: &mut Enemy) {
        let p = e.path.clone();
        if e.movement == 6 {
            if e.state == 3 || e.state == 4 {
                e.y += 2;
                return;
            }
            let mut d0 = e.idx * 3;
            if e.step == 0 {
                e.idx += 1;
                d0 += 3;
                e.step = path_at(&p, d0);
                if e.step < 0 {
                    e.idx = 0;
                    d0 = 0;
                    e.step = path_at(&p, 0);
                }
            }
            e.step -= 1;
            e.dx = -path_at(&p, d0 + 1);
            e.dy = -path_at(&p, d0 + 2);
            return;
        }
        e.idx += e.step;
        if e.idx < 0 {
            e.step = -e.step;
            e.idx += 1;
        }
        let mut d0 = e.idx;
        if path_at(&p, d0) < 0 {
            e.step = -e.step;
            d0 -= 1;
        }
        let v = -(path_at(&p, d0) * e.step);
        if e.movement == 3 {
            e.dx = v;
        } else {
            e.dy = v;
        }
    }

    /// `animate_enemy`: moves (unless frozen), animates and draws one enemy, and drops it once it leaves the screen.
    pub fn animate_enemy(&mut self, i: usize) {
        let mut d7 = self.scroll - 0x20;
        let mut e = self.enemies[i].clone();
        if d7 >= e.x || e.y >= 0xAA {
            self.enemies[i].state = 0;
            return;
        }
        let pick = |k: usize| e.anim.get(k).copied().filter(|&a| a < self.a.anims.len());
        if e.state == 3 || e.state == 4 {
            let base = if e.state == 4 { 4 } else { 2 };
            let k = if e.dx >= 0 { base } else { base + 1 };
            let Some(an) = pick(k) else {
                return;
            };
            let f = self.a.anims[an][0];
            let x = e.x + e.xoff + f.dx;
            let y = e.y + e.yoff + f.dy;
            self.draw_bob(f.img, x, y, true);
            return;
        }
        let Some(an) = pick(if e.dx >= 0 { 0 } else { 1 }) else {
            return;
        };
        if self.freeze == 0 {
            e.x += e.dx;
            e.y += e.dy;
        }
        let d2 = e.tick;
        e.tick += 1;
        let mut fi = (d2 as u32 >> e.shift) as usize;
        let frames = &self.a.anims[an];
        if fi >= frames.len() {
            e.tick = 0;
            fi = 0;
        }
        let f = frames[fi];
        e.w = self.a.imgs[f.img].pw;
        e.h = self.a.imgs[f.img].bmp.h as i32;
        let d0 = e.x + e.xoff;
        d7 += 0x168;
        self.enemies[i] = e.clone();
        if d0 > d7 {
            return;
        }
        let d1 = e.y + e.yoff;
        self.draw_bob(f.img, d0 + f.dx, d1 + f.dy, true);
    }

    // ------------------------------------------------------------------ animated tiles

    /// `animate_map_tiles`.
    pub fn animate_map_tiles(&mut self) {
        self.animate_diamond_blocks();
        self.animate_power_blocks();
        self.animate_flames();
        self.animate_water();
    }

    /// `animate_diamond_blocks` (the bonus list).
    pub fn animate_diamond_blocks(&mut self) {
        if self.objects[2].is_none() {
            return;
        }
        let img = self.a.block_shimmer[((self.shimmer_t >> 3) as usize) & 7];
        self.draw_animated_tile_list(2, img, 0, 0);
        self.shimmer_t = (self.shimmer_t + 1) & 0x3F;
    }

    /// `animate_power_blocks` (the bonus1 list; it shares the shimmer counter).
    pub fn animate_power_blocks(&mut self) {
        if self.objects[3].is_none() {
            return;
        }
        let img = self.a.block_shimmer[((self.shimmer_t >> 3) as usize) & 7];
        self.draw_animated_tile_list(3, img, 0, 0);
        self.shimmer_t = (self.shimmer_t + 1) & 0x3F;
    }

    /// `animate_flames` (the dornen list, 40 frames).
    pub fn animate_flames(&mut self) {
        if self.objects[1].is_none() {
            return;
        }
        let n = self.a.dornen.len() - 1;
        let img = self.a.dornen[((self.dornen_t >> 3) as usize).min(n)];
        self.draw_animated_tile_list(1, img, 8, -8);
        self.dornen_t += 1;
        if self.dornen_t >= 0x28 {
            self.dornen_t = 0;
        }
    }

    /// `animate_water` (the flamme list, 24 frames).
    pub fn animate_water(&mut self) {
        if self.objects[4].is_none() {
            return;
        }
        let n = self.a.flamme.len() - 1;
        let img = self.a.flamme[((self.flamme_t >> 3) as usize).min(n)];
        self.draw_animated_tile_list(4, img, 0, 0);
        self.flamme_t += 1;
        if self.flamme_t >= 0x18 {
            self.flamme_t = 0;
        }
    }

    /// `draw_animated_tile_list`: the frame at every position of the list in the visible part of the stage.
    pub fn draw_animated_tile_list(&mut self, list: usize, img: usize, dx: i32, dy: i32) {
        let lo = self.scroll - 0x16;
        let hi = self.scroll + 0x13E;
        let l = self.objects[list].clone().unwrap_or_default();
        for (x, y) in l {
            let x = x as i16 as i32;
            if x < lo || x > hi || y & 0x8000 != 0 {
                continue;
            }
            let y = 0x9F - y as i32;
            self.draw_bob(img, x + dx, y + dy, false);
        }
    }

    // ------------------------------------------------------------------ stages

    /// `load_stage`: the stage map fresh from the pack (or a bonus stage); back from a bonus stage, its entrance
    /// becomes a crumbling bridge.
    pub fn load_stage(&mut self) {
        if self.in_bonus {
            let k = (self.bonus_idx.max(0) as usize) % self.a.bonus.len();
            self.st = self.a.bonus[k].clone();
            return;
        }
        let s = (self.stage.max(0) as usize).min(self.a.stages.len() - 1);
        self.st = self.a.stages[s].clone();
        if self.bonus_return_scroll < 0 {
            return;
        }
        self.bonus_return_scroll = -1;
        self.parse_stage_header();
        self.bridge_marks.clear();
        if let Some(mut p) = self.bonus_entry {
            loop {
                let (x, y) = self.cell_xy(p);
                self.bridge_marks.push((x, y));
                self.map_set(p, 0x53);
                self.map_set(p + 1, 0x52);
                p += 2;
                if self.map_get(p) != 0x8F {
                    break;
                }
            }
        }
    }

    /// `parse_stage_header` (`find_tag`): the pack already holds the stage's sections parsed (width, height, map and the tagged
    /// lists), so nothing is left to find at run time.
    pub fn parse_stage_header(&mut self) {}

    /// `build_stage`.
    pub fn build_stage(&mut self) {
        self.parse_stage_header();
        self.resolve_object_names();
        self.find_lift_start();
        self.select_enemy_list();
        self.set_stage_palette();
        self.fill_stage_background();
        self.draw_stage_scenery();
        self.draw_stage_tiles();
        self.draw_crumbling_bridges();
    }

    /// `resolve_object_names` (`find_object_name`): the stage's named object lists (bridge, thorns, blocks, flames, hint).
    pub fn resolve_object_names(&mut self) {
        self.objects = self.st.objects.clone();
    }

    /// `find_lift_start`.
    pub fn find_lift_start(&mut self) {
        self.lift_x = 0;
        self.lift_y = 0x8C;
        self.lift_dir = 1;
        if self.in_bonus {
            self.lift_y = 0x89;
            self.lift_dir = 0;
        }
        let w = self.st.w;
        for c in 0..w {
            if self.map_get((15 * w + c) as isize) == 0xE7 {
                self.lift_x = c as i32 * 8 - 0x18;
                return;
            }
        }
    }

    /// `select_enemy_list`: the stage's enemies, all unused.
    pub fn select_enemy_list(&mut self) {
        if self.in_bonus {
            return;
        }
        let n = self
            .a
            .enemy_lists
            .get(self.stage as usize)
            .map_or(0, |l| l.len());
        self.enemy_spawned = vec![false; n];
    }

    /// `set_stage_palette`: the stage's colours, the sprite colours, and whether the clouds show (colour 12 set).
    pub fn set_stage_palette(&mut self) {
        self.clouds_on = self.st.palette[12] != 0;
        for k in 0..16 {
            self.col[k] = (self.st.palette[k] << 1) & 0xFFF;
        }
        for k in 0..4 {
            for (j, c) in self.a.sprite_colours.iter().enumerate() {
                self.col[16 + 4 * k + j] = (c << 1) & 0xFFF;
            }
        }
    }

    /// `fill_stage_background`: the stage's 32 by 32 pattern over the whole playfield.
    pub fn fill_stage_background(&mut self) {
        let a = self.a.clone();
        let b = &a.imgs[self.st.background].bmp;
        for y in 0..160 {
            for x in 0..2048 {
                self.pf.px[y * 2048 + x] = b.px[(y % b.h) * b.w + (x % b.w)];
            }
        }
        // About 1.8 million processor cycles of word copies by count; measured against the reference run, a stage
        // build costs 21 frames more than the counted copies, which is kept here.
        self.busy(3960 + 313 * 21);
    }

    /// `draw_stage_scenery`: the large scenery blocks, copied whole at word positions.
    pub fn draw_stage_scenery(&mut self) {
        let a = self.a.clone();
        let mut words = 0i64;
        for &(img, x, y) in &self.st.scenery {
            let b = &a.imgs[img].bmp;
            let x0 = (((x as u16) & 0xFFF0) >> 3) as i64 * 8;
            let row = 0x9F - y;
            let start = row as i64 * 2048 + x0;
            for r in 0..b.h as i64 {
                for c in 0..b.w as i64 {
                    let o = start + r * 2048 + c;
                    if o >= 0 && (o as usize) < self.pf.px.len() {
                        self.pf.px[o as usize] = b.px[r as usize * b.w + c as usize];
                    }
                }
            }
            words += (b.w / 16 * b.h * 4) as i64;
        }
        self.busy((words * 22 / 454) as i32);
    }

    /// `draw_stage_tiles`: every tile object of the stage, cookie-cut.
    pub fn draw_stage_tiles(&mut self) {
        let tiles = self.st.tiles.clone();
        let mut words = 0i64;
        for (img, x, y) in tiles {
            self.blit_bob(Tgt::Playfield, img, x, 0x9F - y, false);
            let b = &self.a.imgs[img].bmp;
            words += (b.w / 16 * b.h * 4) as i64;
        }
        self.busy((words * 8 / 454) as i32);
    }

    /// `draw_crumbling_bridges`: the bridge pieces that replaced a used bonus entrance.
    pub fn draw_crumbling_bridges(&mut self) {
        let marks = std::mem::take(&mut self.bridge_marks);
        let img = self.a.bridge_piece;
        for (x, y) in marks {
            self.blit_bob(Tgt::Playfield, img, x, y, false);
        }
    }
}
