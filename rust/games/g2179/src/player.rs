//! Giana herself: reading the map around her, walking, jumping and falling, the hazards, the dream-bubble shot,
//! the power items, the lift and the crumbling bridges. Each method is named after the recovered routine.

use crate::assets::{JUMP_L, JUMP_R, SKID_L, SKID_R, STAND_L, STAND_R, WALK_L, WALK_R};
use crate::giana::{bcd_add3, Giana, NONE};

impl Giana {
    // ------------------------------------------------------------------ the map

    /// A map byte by its offset from the map start (outside the map: 0).
    pub fn map_get(&self, i: isize) -> u8 {
        if i < 0 {
            return 0;
        }
        self.st.map.get(i as usize).copied().unwrap_or(0)
    }

    pub fn map_set(&mut self, i: isize, v: u8) {
        if i >= 0 {
            if let Some(x) = self.st.map.get_mut(i as usize) {
                *x = v;
            }
        }
    }

    pub fn class(&self, i: isize) -> u8 {
        self.a.tile_class[self.map_get(i) as usize]
    }

    /// The map offset of column `col` and the row of pixel line `y` as the original computed it in 16-bit words
    /// (a negative line wraps to the rows above, as its unsigned shift and sign-extending add did).
    pub fn map_offset(&self, col: i32, y: i32) -> isize {
        let row = ((y as u16) >> 3) as u32;
        let p = (row * self.st.w as u32) as u16;
        p.wrapping_add(col as u16) as i16 as isize
    }

    /// The map offset as a pixel position (`divu` by the map width, times 8).
    pub fn cell_xy(&self, i: isize) -> (i32, i32) {
        let w = self.st.w as isize;
        let i = i.max(0);
        (((i % w) * 8) as i32, ((i / w) * 8) as i32)
    }

    fn w(&self) -> isize {
        self.st.w as isize
    }

    /// `update_player`.
    pub fn update_player(&mut self) {
        self.read_joystick();
        self.read_tiles_around_player();
        self.update_player_shot();
        self.update_power_items();
        self.draw_shot_explosion();
        self.player_movement();
        self.lift_collision();
    }

    /// `read_tiles_around_player`: the collision classes of the three-by-four tiles around her; the exit tile clears
    /// the stage.
    pub fn read_tiles_around_player(&mut self) {
        self.player_cell = None;
        self.tiles = [0; 12];
        if self.dying {
            return;
        }
        let col = (((self.px + self.scroll) as u16) >> 3) as i32;
        let y = self.py - 0x28;
        if !(-24..0xA0).contains(&y) {
            return;
        }
        let mut c = self.map_offset(col, y);
        self.player_cell = Some(c);
        let end = 20 * self.w();
        for row in 0..4 {
            if c >= 0 && c <= end {
                self.classify_tile_row(row, c);
            }
            c += self.w();
        }
    }

    /// `classify_tile_row`.
    fn classify_tile_row(&mut self, row: usize, c: isize) {
        for k in 0..3 {
            let t = self.map_get(c + k as isize);
            if t == 0xD9 {
                self.cleared = true;
                self.life_over = true;
            }
            self.tiles[row * 3 + k] = self.a.tile_class[t as usize];
        }
    }

    /// `player_movement`: one frame of her state machine; it stops at the first routine that draws her.
    pub fn player_movement(&mut self) {
        self.drawn = false;
        if self.on_lift {
            self.py = self.lift_y + 0x13;
            self.player_animation_frames();
            return;
        }
        self.power_up_transformation();
        if self.drawn {
            return;
        }
        self.summarise_collision();
        self.player_death_check();
        if self.drawn {
            return;
        }
        self.set_facing();
        self.stop_at_walls();
        self.player_jump_state();
        self.player_air_control();
        self.player_vertical_move();
        if self.drawn {
            return;
        }
        self.walk_right();
        if self.drawn {
            return;
        }
        self.walk_left();
        if self.drawn {
            return;
        }
        self.slow_down_left();
        if self.drawn {
            return;
        }
        self.slow_down_right();
        if self.drawn {
            return;
        }
        self.player_animation_frames();
    }

    /// `player_jump_state`: jumping (up held on the ground, as high as the jump table allows), the turn at the top
    /// and the fall, landing when the feet find ground.
    pub fn player_jump_state(&mut self) {
        if !self.up {
            self.jump_latch = false;
        }
        if !self.head_free {
            self.jumping = 0xFF;
            self.falling = 0;
            self.fall_i = 0;
        }
        if (self.falling == 0xFF || self.jumping == 0xFF) && !self.airborne {
            self.jumping = 0;
            self.falling = 0;
            self.jump_i = 0;
            self.fall_i = 0;
            self.py = (self.py & !7) + 3;
        }
        if self.falling != 0 {
            if self.falling == 0x80 {
                if self.fall_i != 7 {
                    self.fall_i += 1;
                } else {
                    self.falling = 0xFF;
                }
                return;
            }
            if self.fall_i != 0 {
                self.fall_i -= 1;
                return;
            }
            self.falling = 0;
            self.jumping = 0xFF;
            return;
        }
        if self.jumping == 0x80 {
            if self.up && self.jump_i != 0x1A {
                self.jump_i += 1;
                return;
            }
            self.falling = 0x80;
            self.fall_i = 0;
            return;
        }
        if self.jumping != 0 {
            self.jump_i -= 1;
            return;
        }
        if !self.up || self.jump_latch {
            return;
        }
        self.jumping = 0x80;
        self.jump_latch = true;
        self.play_sound_effect(0);
    }

    /// `player_air_control`.
    pub fn player_air_control(&mut self) {
        if self.jumping == 0 {
            return;
        }
        if self.right {
            if self.speed <= 0 {
                self.speed += 2;
            } else if self.speed != 0x23 {
                self.speed += 1;
            }
            return;
        }
        if self.left {
            if self.speed >= 0 {
                self.speed -= 2;
            } else if self.speed != -0x23 {
                self.speed -= 1;
            }
            return;
        }
        if self.speed <= 0 {
            self.speed += 1;
        } else {
            self.speed -= 1;
        }
    }

    fn table(v: &[i32], i: i32) -> i32 {
        if i < 0 {
            return 0;
        }
        v.get(i as usize).copied().unwrap_or(0)
    }

    /// A word of a table that the original also read below its start (`before` holds the words stored there).
    fn table_ext(v: &[i32], before: &[i32], i: i32) -> i32 {
        if i >= 0 {
            return Self::table(v, i);
        }
        let k = before.len() as i32 + i;
        if k < 0 {
            0
        } else {
            before[k as usize]
        }
    }

    /// `player_vertical_move`: up or down by the jump or fall table, with the jump frame.
    pub fn player_vertical_move(&mut self) {
        if self.jumping == 0 {
            return;
        }
        let d = if self.falling == 0 {
            let d = Self::table_ext(&self.a.jump, &self.a.jump_before, self.jump_i);
            if self.jumping != 0xFF {
                -d
            } else {
                d
            }
        } else {
            let d = Self::table(&self.a.fall, self.fall_i);
            if self.falling != 0xFF {
                -d
            } else {
                d
            }
        };
        self.py += d;
        self.player_frame(if self.facing_left { JUMP_L } else { JUMP_R }, 0);
    }

    /// `player_death_check`: the hazards; a death plays its sound and starts the death animation.
    pub fn player_death_check(&mut self) {
        if self.death_t == 0 {
            self.check_hazards();
            if !self.dying {
                return;
            }
            self.play_sound_effect(4);
        }
        self.death_animation();
    }

    /// The death animation: she bounces up and falls off the bottom of the screen.
    fn death_animation(&mut self) {
        self.py -= Self::table(&self.a.death_bounce, self.death_t);
        self.speed = 0;
        let img = self.a.death[0];
        self.draw_player_frame(img);
        self.death_t += 1;
        if self.py < 0xD2 {
            return;
        }
        self.death_t = 0;
        self.life_over = true;
    }

    /// `walk_right`.
    pub fn walk_right(&mut self) {
        self.jumping = if self.airborne { 0xFF } else { 0 };
        if !self.right {
            return;
        }
        if self.speed < 0 {
            self.speed += 2;
            self.player_frame(SKID_L, 0);
            return;
        }
        if self.speed < 0x23 {
            self.speed += 1;
        }
        self.walk_right_frames();
    }

    fn walk_right_frames(&mut self) {
        self.player_frame(WALK_R, self.walk_r as usize);
        let rate = Self::table(&self.a.anim_rate, self.speed);
        self.anim_tick += 1;
        if rate > self.anim_tick {
            return;
        }
        self.anim_tick = 0;
        self.walk_r = (self.walk_r + 1) & 3;
    }

    fn walk_left_frames(&mut self) {
        self.player_frame(WALK_L, self.walk_l as usize);
        let rate = Self::table(&self.a.anim_rate, -self.speed);
        self.anim_tick += 1;
        if rate > self.anim_tick {
            return;
        }
        self.anim_tick = 0;
        self.walk_l = (self.walk_l + 1) & 3;
    }

    /// `walk_left`.
    pub fn walk_left(&mut self) {
        if !self.left {
            return;
        }
        if self.speed > 0 {
            self.speed -= 2;
            self.player_frame(SKID_R, 0);
            return;
        }
        if self.speed > -0x23 {
            self.speed -= 1;
        }
        self.walk_left_frames();
    }

    /// `slow_down_right`.
    pub fn slow_down_right(&mut self) {
        if self.speed <= 0 {
            return;
        }
        self.speed -= 1;
        self.walk_right_frames();
    }

    /// `slow_down_left`.
    pub fn slow_down_left(&mut self) {
        if self.speed >= 0 {
            return;
        }
        self.speed += 1;
        self.walk_left_frames();
    }

    /// `player_animation_frames`: standing.
    pub fn player_animation_frames(&mut self) {
        self.player_frame(if self.facing_left { STAND_L } else { STAND_R }, 0);
    }

    fn player_frame(&mut self, set: usize, i: usize) {
        let f = &self.a.frame_sets[self.frame_set][set];
        let img = f[i.min(f.len() - 1)];
        self.draw_player_frame(img);
    }

    /// `draw_player_frame`.
    pub fn draw_player_frame(&mut self, img: usize) {
        self.drawn = true;
        let x = self.px + self.scroll - 4;
        let y = self.py - 0x28;
        self.draw_bob(img, x, y, true);
    }

    /// `apply_horizontal_speed`: right of the middle the stage scrolls under her while there is more to show;
    /// otherwise she moves on the screen between its edges.
    pub fn apply_horizontal_speed(&mut self) {
        self.prev_scroll = self.scroll;
        let i = (self.speed + 0x23).clamp(0, self.a.hspeed.len() as i32 - 1) as usize;
        let d = self.a.hspeed[i];
        if !self.in_bonus && self.px >= 0xA0 && d >= 0 && self.scroll < 0x6BF {
            self.scroll += d;
            if self.scroll >= 0x6BF {
                self.scroll = 0x6BF;
            }
            return;
        }
        if self.px <= 4 {
            if d >= 0 {
                self.px += d;
            }
            return;
        }
        if self.px > 0x12C {
            if d < 0 {
                self.px += d;
            }
            return;
        }
        self.px += d;
    }

    /// `power_up_transformation`: after the power-up she changes for 32 frames, frozen, into the punk.
    pub fn power_up_transformation(&mut self) {
        if self.powered == 0 || self.frame_set == 1 {
            return;
        }
        if self.speed != 0 {
            self.saved_speed = self.speed;
            self.speed = 0;
        }
        let img = self.a.transform[((self.transform_t >> 2) as usize).min(7)];
        self.draw_player_frame(img);
        self.transform_t += 1;
        if self.transform_t < 0x20 {
            return;
        }
        self.transform_t = 0;
        self.speed = self.saved_speed;
        self.use_powered_player_frames();
    }

    /// `summarise_collision`.
    pub fn summarise_collision(&mut self) {
        let t = self.tiles;
        self.head = t[0..3].iter().filter(|&&c| c == 1).count() as u8;
        self.feet = t[9..12].iter().filter(|&&c| c == 1).count() as u8;
        self.right_free = !(t[8] == 1 || t[5] == 1);
        self.left_free = !(t[6] == 1 || t[3] == 1);
        self.check_head_blocked();
        self.check_ground();
    }

    /// `check_head_blocked`.
    pub fn check_head_blocked(&mut self) {
        self.head_free = true;
        if self.head == 0 {
            return;
        }
        self.head_free = self.head <= 1;
        if self.head > 1 {
            return;
        }
        if self.right_free && self.tiles[2] == 1 {
            self.head_free = false;
            return;
        }
        if self.left_free && self.tiles[0] == 1 {
            self.head_free = false;
        }
    }

    /// `check_ground`.
    pub fn check_ground(&mut self) {
        self.airborne = false;
        let t = self.tiles;
        if self.feet == 0 {
            self.airborne = true;
            return;
        }
        if self.feet <= 1 {
            if !self.right_free {
                if t[9] != 1 {
                    self.airborne = true;
                    return;
                }
            } else if !self.left_free && t[11] != 1 {
                self.airborne = true;
                return;
            }
        }
        if self.head > 1 {
            self.airborne = true;
            return;
        }
        if t[0] == 1 {
            self.left_free = false;
            return;
        }
        if t[2] == 1 {
            self.right_free = false;
        }
    }

    /// `check_hazards`: falling off the screen, water, thorns (unless protected), flames; the dream's end tile, and
    /// the entrance to a bonus stage.
    pub fn check_hazards(&mut self) {
        if self.dying {
            return;
        }
        self.dying = self.py > 0xBE;
        if self.dying {
            return;
        }
        let w = self.w();
        if self.dream_over {
            if let Some(c) = self.player_cell {
                if (0x30..=0xA0).contains(&self.py)
                    && (self.map_get(c + 1) == 0xFF || self.map_get(c + w + 1) == 0xFF)
                {
                    self.game_over = true;
                    self.life_over = true;
                }
            }
        }
        if self.tiles[..9].contains(&3) {
            self.dying = true;
            return;
        }
        self.dying = false;
        if self.protection == 0 {
            if self.tiles[..9].contains(&4) {
                self.dying = true;
                return;
            }
            self.dying = false;
        }
        if self.in_bonus || self.py >= 0xB4 {
            return;
        }
        let Some(c) = self.player_cell else {
            return;
        };
        let a = c + 3 * w;
        for k in 0..3 {
            if self.map_get(a + k) == 0x8F {
                self.in_bonus = true;
                self.life_over = true;
                let mut p = a + k;
                while self.map_get(p - 1) == 0x8F {
                    p -= 1;
                }
                self.bonus_entry = Some(p);
                return;
            }
        }
        if self.jumping == 0x80 {
            return;
        }
        self.dying = self.map_get(a + 2) == 0xA1
            || self.map_get(a) == 0xA4
            || self.map_get(a + 1) == 0xA1
            || self.map_get(a + 1) == 0xA4;
    }

    /// `set_facing`.
    pub fn set_facing(&mut self) {
        if self.right {
            self.facing_left = false;
        } else if self.left {
            self.facing_left = true;
        }
    }

    /// `stop_at_walls`.
    pub fn stop_at_walls(&mut self) {
        if !self.right_free && self.speed >= 0 {
            self.speed = 0;
        }
        if !self.left_free && self.speed < 0 {
            self.speed = 0;
        }
    }

    // ------------------------------------------------------------------ the shot

    /// `update_player_shot`: the powered-up player's dream bubble: one at a time; it bounces (level 1), comes back
    /// to be caught (level 2) or homes in on an enemy (level 3).
    pub fn update_player_shot(&mut self) {
        if self.shot_level == 0 {
            return;
        }
        if self.fire && self.shot.0 == NONE {
            if self.dying || self.in_bonus || self.fire_latch {
                return;
            }
            self.fire_latch = true;
            self.play_sound_effect(2);
            let d0 = self.px + self.scroll + 0x14;
            self.shot = (d0, self.py - 0x28);
            self.arc_dir = 1;
            self.arc_i = 0x0E;
            self.shot_dx = 7;
            if self.facing_left {
                self.shot_dx = -7;
                self.shot.0 -= 0x1A;
            }
            if self.shot_level == 3 {
                self.find_homing_target(d0 - 0x14);
            }
        }
        if !self.fire {
            self.fire_latch = false;
        }
        if self.shot.0 == NONE {
            return;
        }
        match self.shot_level {
            2 => self.shot_level2(),
            3 => {
                let Some(h) = self.homing else {
                    self.shot_level2();
                    return;
                };
                let e = self.enemies[h].clone();
                let mut a2 = e.x + e.xoff;
                if a2 > self.scroll + 0x140 || a2 < self.scroll - e.w {
                    self.homing = None;
                    return;
                }
                a2 += e.w >> 1;
                let a3 = e.y + e.yoff + (e.h >> 1) - 3;
                let (sx, sy) = self.shot;
                self.plot_homing_path(sx, sy, a2, a3);
                self.homing_shot_step();
                self.draw_shot();
            }
            _ => {
                self.draw_shot();
                self.shot_map_collision();
                self.shot_bounce_arc();
                self.move_shot();
            }
        }
    }

    fn shot_level2(&mut self) {
        self.draw_shot();
        self.shot_map_collision();
        self.shot_catch_check();
        self.shot_bounce_arc();
        self.move_shot();
    }

    fn move_shot(&mut self) {
        self.shot.0 = (self.shot.0 + self.shot_dx) as i16 as i32;
        let d0 = (self.shot.0 - self.scroll) as i16 as i32;
        if d0 >= 0x140 || d0 + 4 < 0 {
            self.shot.0 = NONE;
        }
    }

    /// `shot_map_collision`: off the ground it bounces; into a wall it bursts (level 1) or turns back.
    pub fn shot_map_collision(&mut self) {
        let col = ((self.shot.0 as u16) >> 3) as isize;
        let y8 = self.shot.1 + 8;
        if y8 < 0 {
            return;
        }
        let row = (y8 >> 3) as u32;
        let a = col + ((row * self.st.w as u32) as u16 as i16) as isize;
        let (c0, c1, c2) = (self.class(a - 1), self.class(a), self.class(a + 1));
        if c1 != 1 {
            return;
        }
        let side = if self.shot_dx >= 0 { c0 } else { c2 };
        if side == 1 {
            self.arc_i = 0;
            self.arc_dir = 1;
            return;
        }
        if self.shot_level < 2 {
            self.play_sound_effect(1);
            self.explosion = (self.shot.0 - 6, self.shot.1 - 3);
            self.shot.0 = NONE;
        } else {
            self.shot_dx = -self.shot_dx;
        }
    }

    /// `shot_catch_check`.
    pub fn shot_catch_check(&mut self) {
        let d0 = self.px + self.scroll;
        let d2 = d0 + 16;
        let d1 = self.py - 0x28;
        let d3 = d1 + 0x13;
        let (a0, a1) = self.shot;
        if a0 >= d2 || d0 >= a0 + 6 || a1 >= d3 || d1 >= a1 + 6 {
            return;
        }
        self.shot.0 = NONE;
    }

    /// `shot_bounce_arc`.
    pub fn shot_bounce_arc(&mut self) {
        let d1 = Self::table_ext(&self.a.shot_arc, &self.a.arc_before, self.arc_i);
        if Self::table_ext(&self.a.shot_arc, &self.a.arc_before, self.arc_i + 1) < 0 {
            self.arc_dir = -self.arc_dir;
        }
        self.shot.1 -= d1 * self.arc_dir;
        self.arc_i += self.arc_dir;
    }

    /// `find_homing_target`: the first live enemy, if it is ahead of her.
    pub fn find_homing_target(&mut self, d0: i32) {
        self.homing = None;
        for i in 0..self.enemies.len() {
            if self.enemies[i].state != 1 {
                continue;
            }
            let ex = self.enemies[i].x;
            if (!self.facing_left && d0 < ex) || (self.facing_left && d0 > ex) {
                self.homing = Some(i);
            }
            return;
        }
    }

    /// `homing_shot_step`.
    pub fn homing_shot_step(&mut self) {
        let col = ((self.shot.0 as u16) >> 3) as isize;
        let y8 = self.shot.1 + 8;
        let open = if (0..0xA0).contains(&y8) {
            let a = col + ((((y8 >> 3) as u32) * self.st.w as u32) as u16 as i16) as isize;
            self.class(a) == 0
        } else {
            true
        };
        self.shot = if open {
            self.homing_path[6]
        } else {
            self.homing_path[1]
        };
    }

    /// `draw_shot`.
    pub fn draw_shot(&mut self) {
        let x = self.shot.0 - 4;
        let y = self.shot.1 + 2;
        if y <= -0x2C || y >= 0xA0 {
            self.homing = None;
            self.shot.0 = NONE;
            return;
        }
        let img = self.a.shot[(self.shot_anim >> 1) as usize & 3];
        self.draw_bob(img, x, y, true);
        self.shot_anim = (self.shot_anim + 1) & 7;
    }

    /// `plot_homing_path`: up to ten points of a line from the shot towards its target.
    pub fn plot_homing_path(&mut self, mut d2: i32, mut d3: i32, a2: i32, a3: i32) {
        let mut d4: i32 = 0;
        let (mut a4, mut a5) = (1, 1);
        let mut d6 = a2 - d2;
        if d6 < 0 {
            d6 = -d6;
            a4 = -1;
        }
        let mut d7 = a3 - d3;
        if d7 == 0 {
            d4 = !d4;
        } else if d7 < 0 {
            d7 = -d7;
            a5 = -1;
        }
        let mut k = 0;
        while k < 10 {
            if a2 == d2 && a3 == d3 {
                for _ in 0..4 {
                    if k < 10 {
                        self.homing_path[k] = (d2, d3);
                    }
                    k += 1;
                }
                return;
            }
            if d4 < 0 {
                d2 += a4;
                d4 += d7;
            } else {
                d3 += a5;
                d4 -= d6;
            }
            self.homing_path[k] = (d2, d3);
            k += 1;
        }
    }

    /// `draw_shot_explosion`.
    pub fn draw_shot_explosion(&mut self) {
        if self.explosion.0 == NONE {
            return;
        }
        let img = self.a.explosion[(self.explosion_t >> 1) as usize & 3];
        let (x, y) = self.explosion;
        self.draw_bob(img, x, y, true);
        self.explosion_t += 1;
        if self.explosion_t < 8 {
            return;
        }
        self.explosion_t = 0;
        self.explosion.0 = NONE;
    }

    /// `update_power_items`: the extra life, freezing the enemies (fire with down-left) and destroying them all
    /// (fire with down-right).
    pub fn update_power_items(&mut self) {
        if self.extra_life != 0 {
            self.extra_life = 0;
            self.add_life();
            self.play_sound_effect(6);
        }
        if self.freeze != 0 {
            self.freeze -= 1;
        }
        if self.freezes != 0 && self.down && self.left && self.fire {
            self.freezes -= 1;
            self.freeze = 300;
            return;
        }
        if self.kill_alls == 0 || !self.down || !self.right || !self.fire {
            return;
        }
        self.kill_alls -= 1;
        for i in 0..self.enemies.len() {
            if self.enemies[i].state == 1 {
                self.enemies[i].state = 3;
                self.score = bcd_add3(self.score, self.enemies[i].points);
            }
        }
    }

    // ------------------------------------------------------------------ lift and bridges

    /// `lift_collision`: standing on the lift she rides it.
    pub fn lift_collision(&mut self) {
        if self.dying || self.lift_x == 0 || self.lift_x > self.scroll + 0x140 {
            return;
        }
        let d0 = self.px + self.scroll;
        let d2 = d0 + 4;
        let d1 = self.py - 0x28;
        let d3 = d1 + 0x13;
        let a0 = self.lift_x + 2;
        let a2 = a0 + 64;
        let a1 = self.lift_y;
        let a3 = a1 + 6;
        if a0 >= d2 || d0 >= a2 || a1 >= d3 || d1 >= a3 || d3 >= a3 {
            return;
        }
        self.on_lift = true;
        self.speed = 0;
        self.lift_dir = if self.in_bonus { 2 } else { -2 };
    }

    /// `move_lift`.
    pub fn move_lift(&mut self) {
        if self.lift_x == 0 || self.lift_x > self.scroll + 0x140 {
            return;
        }
        if self.in_bonus {
            self.lift_y -= self.lift_dir;
            if self.lift_y < 0 {
                self.life_over = true;
            }
        } else if self.on_lift {
            self.lift_y += 2;
            let out = self.lift_y >= 0xB3;
            self.cleared = out;
            self.life_over = out;
        } else {
            self.lift_y += self.lift_dir;
            if self.lift_y <= 0x1E {
                self.lift_dir = 1;
            }
            if self.lift_y >= 0x82 {
                self.lift_dir = -1;
            }
        }
        let (img, x, y) = (self.a.lift, self.lift_x, self.lift_y);
        self.draw_bob(img, x, y, true);
    }

    /// `update_crumbling_bridges`.
    pub fn update_crumbling_bridges(&mut self) {
        self.detect_crumbling_bridge();
        self.animate_crumbling_bridges();
    }

    /// `detect_crumbling_bridge`: a bridge piece under her feet starts falling (the left piece is found one tile
    /// further left than it is, as in the original).
    pub fn detect_crumbling_bridge(&mut self) {
        if self.airborne {
            return;
        }
        let Some(c) = self.player_cell else {
            return;
        };
        let a = c + 3 * self.w();
        for k in 0..3 {
            let t = self.map_get(a + k);
            let p = if t == 0x52 {
                a + k - 1
            } else if t == 0x53 {
                a + k
            } else {
                continue;
            };
            let (x, y) = self.cell_xy(p);
            self.add_crumbling_bridge(x, y);
            return;
        }
    }

    /// `animate_crumbling_bridges`: each piece falls for 24 frames and then leaves the map.
    pub fn animate_crumbling_bridges(&mut self) {
        for i in 0..self.falling_bridges.len() {
            let [x, y, t] = self.falling_bridges[i];
            if x == 0 {
                continue;
            }
            let img = self.a.bridge[((t >> 2) as usize).min(self.a.bridge.len() - 1)];
            self.draw_bob(img, x, y, false);
            self.falling_bridges[i][2] += 1;
            if self.falling_bridges[i][2] >= 0x18 {
                let o = (y >> 3) as isize * self.w() + (x >> 3) as isize;
                self.map_set(o, 0);
                self.map_set(o + 1, 0);
                self.falling_bridges[i][0] = 0;
            }
        }
    }

    /// `add_crumbling_bridge`.
    pub fn add_crumbling_bridge(&mut self, x: i32, y: i32) {
        if self.falling_bridges.iter().any(|e| e[0] == x && e[1] == y) {
            return;
        }
        if let Some(e) = self.falling_bridges.iter_mut().find(|e| e[0] == 0) {
            *e = [x, y, 0];
        }
    }
}
