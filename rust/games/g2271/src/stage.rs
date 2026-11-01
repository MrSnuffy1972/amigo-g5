//! The stage engine of Castlevania (part 2 of the port): room set-up, the per-frame game loop, scrolling, the
//! attribute map (floors, walls, stairs, exits), Simon (walk, crouch, stairs, jump, fall, whip, knock-back,
//! death) and the status panel. Each routine keeps the original's name and offset (`+XXXX`) in its doc comment;
//! fields keep the original's global in theirs (`-$XXXX(a4)`).
//!
//! The player logic lives on [`Simon`], which holds no picture and no assets, so it can be run on its own; the
//! drawing and the frame loop are on [`Castle`]. Enemies, items, bosses, sub-weapons and sound follow in part 3:
//! the places that call them are marked "part 3".

use std::sync::Arc;

use crate::assets::{at, Assets, ScreenFile, Tables};

/// The 3 by 6 window of attribute bytes around the player (+31E8): rows from the head (0) to the floor under the
/// feet (5), columns left, middle and right of the player's 8-pixel column.
pub type Grid = [[i8; 3]; 6];

/// The ground and stairs flags +350C works out from the grid (the original's globals in brackets).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Ground {
    pub nothing_under: bool, // -$12C6
    pub under_d: bool,       // -$12BE
    pub can_left: bool,      // -$12F6
    pub can_right: bool,     // -$1300
    pub head_free: bool,     // -$12C4
    pub step_l: bool,        // -$12FC
    pub step_r: bool,        // -$1306
    pub up_l_free: bool,     // -$12C0
    pub up_r_free: bool,     // -$12C2
    pub down_l_free: bool,   // -$12F8
    pub down_r_free: bool,   // -$1302
    pub stair_ul: bool,      // -$12FE
    pub stair_ur: bool,      // -$1308
    pub on_stair: bool,      // -$130C
    pub stair_dr: bool,      // -$1304
    pub stair_dl: bool,      // -$12FA
    pub stair_flat: bool,    // -$130A
    pub stair_top_up: bool,  // -$12F2
    pub stair_top: bool,     // -$12F4
}

/// Frames the original spent loading and depacking a screen file of `bytes` packed bytes (see `enter_room`).
pub fn load_frames(bytes: usize) -> usize {
    ((bytes * 1943 + 50_000) / 100_000).saturating_sub(9)
}

/// The pause message as this disk has it (+B00C: the crack's banner).
pub const PAUSE_BANNER: &str = "     Q U A R T E X      ";

/// Sound effects the stage engine asks for (played by part 3; kept as numbers of the original's calls).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sfx {
    Land,
    Hurt,
    Death,
    TimeWarning,
    Fall,
}

/// The player and the stage state the player logic reads and writes.
#[derive(Debug, Clone, Default)]
pub struct Simon {
    // ---- the player's object record (-$2136(a4), 0x70 bytes)
    /// x on screen in pixels (-$2136), y in rows (8.8, -$2134).
    pub x: i16,
    pub y: i16,
    /// Animation frame (-$2124) and animation row (-$2122).
    pub frame: i16,
    pub anim: i16,
    /// Move state this frame (-$2120: 0 still, 1 straight up a stair top, 3 left, 4 right, 5 up-left,
    /// 6 down-left, 7 up-right, 8 down-right), facing (-$211E, 3 left, 4 right), last move (-$211A).
    pub mv: i16,
    pub facing: i16,
    pub last_dir: i16,
    /// Horizontal drift while jumping (-$20E6, sign only).
    pub dx: i16,
    /// Health 0 to 16 (-$20D6), the health the panel shows (-$2138), the boss's health (-$2184).
    pub hp: i16,
    pub hp_shown: i16,
    pub enemy_hp: i16,
    /// Health bar animation: frames left (-$2156) and direction (-$131A).
    pub hp_anim: i16,
    pub hp_dir: i16,
    // ---- player states
    pub hurt: i16, // -$2176: 1 to 6 knock-back, then counts on to 30 (no damage meanwhile)
    pub dying: i16, // -$2178: 16 starts the death, counts down to 1
    pub death_done: i16, // -$1310
    pub crouch: i16, // -$214C
    pub jump: i16, // -$214A: jump step
    pub jump_req: i16, // -$12F0
    pub jump_phase: i16, // -$12E6
    pub whip: i16, // -$2148: frames into the whip
    pub whip_anim: i16, // -$188E: the whip's own animation row (0: none)
    pub falling: i16, // -$188C
    pub hurt_facing: i16, // -$1358
    /// What the player was doing, for +2952 (jump -$1376, crouch -$137A, whip -$1374, hurt -$1378,
    /// fall -$137C).
    pub was: [bool; 6],
    /// The whip reached its last frame this frame (what it hits is checked by +6D4C), or the throw's (+7CE4).
    pub whip_strike: bool,
    pub throw_strike: bool,
    /// At the strike: the whip's frame, and whether the body's animation had ended (+6D4C reads both).
    pub strike: (u8, bool),
    /// Up was held when the whip started (-$12B8), and the sub-weapon may be thrown (no more of it on the
    /// screen than the shot multiplier allows; set by the objects each frame).
    pub throw_up: bool,
    pub throw_ok: bool,
    /// The whip's damage (-$20D2: 0x10, 0x14 from the second whip) and the extra lives scored (-$2172).
    pub dmg: i16,
    pub extra: i16,
    /// The negative attribute byte in the body row this frame (spawner and item marks; exits are handled here).
    pub mark: i8,
    // ---- the stage
    pub room: i16,                 // -$19B0
    pub van: i16,                  // the room's screen file (room table +8)
    pub map_w: i16,                // -$13DE
    pub scroll_col: i16,           // -$19AE
    pub prev_col: i16,             // -$19AC
    pub saved_col: i16,            // -$1380
    pub scroll_lock: i16,          // -$199A
    pub block: i16,                // -$19A8
    pub prev_room: i16,            // -$199E
    pub stage: i16,                // -$19C0
    pub lives: i16,                // -$19AA
    pub score: i32,                // -$2170
    pub time: i16,                 // -$19BE
    pub hearts: i16,               // -$19C6
    pub whip_level: i16,           // -$19C4
    pub whip_len: i16,             // -$19BC
    pub sub: i16,                  // -$19C2
    pub shot: i16,                 // -$214E
    pub entry_mode: i16,           // -$19B2
    pub door: i16,                 // -$1890
    pub new_life: i16,             // -$213A
    pub room_change: i16,          // -$1892
    pub changing: i16,             // -$1894
    pub room_change_prev: i16,     // -$137E
    pub fade_point: i16,           // -$12E4
    pub tick10: i16,               // -$19A0
    pub stopwatch: i16,            // -$2166
    pub inv: i16,                  // -$2168
    pub inv_count: i16,            // -$216A
    pub cross: i16,                // -$216C
    pub still: i16,                // -$20D0
    pub last_pos: (i16, i16, i16), // -$2126, -$2128, -$12B6
    pub game_over: i16,            // -$217E
    pub stage_clear: i16,          // -$2180
    pub quit: i16,                 // -$2182
    /// The cheats (-$2188 collision off, -$218C game time on, -$218A life count on).
    pub no_collision: bool,
    pub time_on: bool,
    pub lives_on: bool,
    // ---- this frame's attribute window and flags
    pub g: Grid,
    pub gr: Ground,
    /// The last attribute byte +2FB2 saw that was an event or empty (-$130E).
    pub attr_event: i16,
    // ---- controls (+15AA)
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub centre: bool,
    pub fire: bool,
    pub jump_btn: bool,
    pub sfx: Vec<Sfx>,
}

const ST_JUMP: usize = 1;
const ST_CROUCH: usize = 2;
const ST_WHIP: usize = 3;
const ST_HURT: usize = 4;
const ST_FALL: usize = 5;

/// The block of rooms a room belongs to, as +0ED2 tests it (7 ranges; the last loads its own art but stays
/// block 5).
pub fn room_range(room: i16) -> Option<usize> {
    match room {
        0..=7 => Some(0),
        8..=16 => Some(1),
        17..=23 => Some(2),
        24..=26 => Some(3),
        27..=35 => Some(4),
        36..=44 => Some(5),
        45..=47 => Some(6),
        _ => None,
    }
}

impl Simon {
    /// A new game's state (+0006 main_program's flags, +051C reset_game_state_flags, +04D6, +042A).
    pub fn new_game(t: &Tables, vans: &[i16]) -> Simon {
        let mut s = Simon {
            time_on: true,
            lives_on: true,
            ..Simon::default()
        };
        // +051C
        s.prev_room = -2;
        s.stage = 1;
        s.block = 0;
        // +04D6 initialize_global_state
        s.lives = 3;
        s.score = 0;
        s.room = at(&t.block_room, s.block as i32) as i16;
        s.stage = at(&t.van_stage, at(vans, s.room as i32) as i32) as i16;
        s.reset_life(t);
        s
    }

    /// +042A initialize_game_state_2: full health, five hearts, the plain whip and the stage's time.
    pub fn reset_life(&mut self, t: &Tables) {
        self.hp_shown = 16;
        self.hp = 16;
        self.hearts = 5;
        self.whip_level = 0;
        self.sub = 0;
        self.shot = 0;
        self.time = at(&t.time, self.stage as i32 - 1) as i16 * 100;
        self.whip_len = at(&t.whip, self.whip_level as i32) as i16;
        self.dmg = 0x10;
        self.stopwatch = 0;
        self.inv = 0;
        self.inv_count = 0;
        self.cross = 0;
    }

    /// +03F0 clear_state_flags: after a death, the player starts again.
    pub fn reset_player(&mut self, t: &Tables) {
        self.hurt = 0;
        self.crouch = 0;
        self.jump = 0;
        self.whip = 0;
        self.reset_life(t);
        self.new_life = 1;
    }

    /// +28F6 set_flag_by_index: remembers what the player was doing.
    fn set_was(&mut self, k: usize) {
        self.was[k] = true;
    }

    /// +90EC lookup_tile_byte: the animation that follows `row` for the last move (-2), the facing (-1) or move
    /// state `d`.
    fn sel(&self, t: &Tables, row: i16, d: i16) -> i16 {
        match d {
            -2 => t.next(row, self.last_dir),
            -1 => t.next(row, self.facing),
            _ => t.next(row, d),
        }
    }

    /// +2952 clear_flags_and_call: the animation to go back to when state `k` ends.
    fn back_from(&mut self, t: &Tables, k: usize) -> i16 {
        self.was[k] = false;
        let row = match k {
            ST_CROUCH => 0,
            ST_WHIP => {
                if self.was[ST_CROUCH] {
                    6
                } else if self.was[ST_JUMP] {
                    0xA
                } else {
                    0
                }
            }
            ST_JUMP => {
                if self.was[ST_WHIP] {
                    0x1C
                } else {
                    2
                }
            }
            ST_FALL if self.whip != 0 => 0x1C,
            _ => 0,
        };
        self.sel(t, row, -1)
    }

    /// +9076 check_collision_flag (for the player): the animation has run to its end marker.
    pub fn anim_ended(&self, t: &Tables) -> bool {
        t.frame(self.anim, self.frame) == 99
    }

    /// +2A5E check_enemy_type_flag: animations that keep their pose when the player starts to fall.
    fn keeps_pose(&self) -> bool {
        matches!(self.anim, 4 | 5 | 0x12 | 0x13 | 0x16 | 0x17 | 0x1C | 0x1D)
    }

    /// The player's 8-pixel column in the map.
    fn col(&self) -> i32 {
        self.scroll_col as i32 + (self.x >> 3) as i32
    }

    /// +31E8 load_palette_from_table: reads the attribute window around the player.
    pub fn read_grid(&mut self, sf: &ScreenFile) {
        let w = self.map_w as i32;
        let mut r1 = (self.y >> 8) as i32;
        if self.y & 0xFF != 0 {
            r1 += 1;
        }
        let top = r1 - 5;
        self.g = [[0; 3]; 6];
        let row = |p: i32| [sf.at(p), sf.at(p + 1), sf.at(p + 2)];
        let mut p = top * w + self.col();
        let mut skip = if top < 0 { -top } else { 0 };
        if self.jump != 0 && skip < 3 {
            skip = 3;
        }
        if skip != 0 {
            p += (skip - 1) * w;
            self.g[0] = row(p);
            p += w;
        }
        if skip <= 5 {
            for k in skip..=5 {
                if k == 5 && r1 >= 0x1A {
                    break;
                }
                self.g[k as usize] = row(p);
                p += w;
            }
        }
    }

    /// +2FB2 decode_tile_attribute: the solid bits of the row under (x, y + dy); event bytes go to `attr_event`.
    pub fn floor_bits(&mut self, sf: &ScreenFile, x: i16, y: i16, dy: i16) -> u16 {
        self.attr_event = 0;
        let v = y.wrapping_add(dy);
        let mut r = (v >> 8) as i32;
        if v & 0xFF != 0 {
            r += 1;
        }
        let mut bits = 0u16;
        if r < 0x1A && r > 3 {
            let p = (r - 1) * self.map_w as i32 + self.scroll_col as i32 + (x >> 3) as i32;
            let a0 = sf.at(p);
            if a0 < 1 || !self.gr.can_left {
                self.attr_event = a0 as i16;
            } else {
                bits |= a0 as u16;
            }
            let a1 = sf.at(p + 1);
            if a1 < 1 {
                self.attr_event = a1 as i16;
            } else {
                bits |= a1 as u16;
            }
            let a2 = sf.at(p + 2);
            if a2 < 1 || !self.gr.can_right {
                self.attr_event = a2 as i16;
            } else {
                bits |= a2 as u16;
            }
        }
        bits
    }

    /// +2AA2 increment_counter_and_update_offset: one step right, scrolling the map once past column 18.
    pub(crate) fn step_right(&mut self) {
        if self.scroll_col < self.map_w - 0x28 && (self.x >> 3) > 0x12 && self.scroll_lock == 0 {
            self.scroll_col += 1;
        } else if (self.x >> 3) < 0x27 {
            self.x += 8;
        } else {
            self.dx = 0;
        }
    }

    /// +2AF2 decrement_timer_or_update_counter: one step left.
    fn step_left(&mut self) {
        if self.scroll_col < 1 || (self.x >> 3) > 0x12 || self.scroll_lock != 0 {
            if (self.x >> 3) < 1 {
                self.dx = 0;
            } else {
                self.x -= 8;
            }
        } else {
            self.scroll_col -= 1;
        }
    }

    fn blocked_move(&self) -> bool {
        !self.gr.on_stair
            || self.gr.stair_flat
            || self.jump != 0
            || self.falling != 0
            || self.hurt != 0
    }

    /// +2B36 check_and_update_state: right (4), up the stairs right (7) or down them right (8); 0 if blocked.
    pub(crate) fn move_right(&mut self) -> i16 {
        if self.blocked_move() {
            if self.gr.can_right {
                self.step_right();
                4
            } else {
                0
            }
        } else if self.gr.stair_ur && self.gr.step_r {
            self.y -= 0x100;
            self.step_right();
            7
        } else if self.gr.stair_dr && self.gr.down_r_free {
            self.y += 0x100;
            self.step_right();
            8
        } else {
            0
        }
    }

    /// +2BA4 check_and_update_state_flags: left (3), up the stairs left (5) or down them left (6).
    pub(crate) fn move_left(&mut self) -> i16 {
        if self.blocked_move() {
            if self.gr.can_left {
                self.step_left();
                3
            } else {
                0
            }
        } else if self.gr.stair_ul && self.gr.step_l {
            self.y -= 0x100;
            self.step_left();
            5
        } else if self.gr.stair_dl && self.gr.down_l_free {
            self.y += 0x100;
            self.step_left();
            6
        } else {
            0
        }
    }

    /// +2C12 check_flags_and_increment_counter: down on to or along the stairs.
    fn move_down(&mut self) -> i16 {
        if !self.gr.stair_top {
            if self.gr.stair_dr {
                self.y += 0x100;
                self.step_right();
                8
            } else if self.gr.stair_dl {
                self.y += 0x100;
                self.step_left();
                6
            } else {
                0
            }
        } else {
            self.y += 0x100;
            if self.gr.stair_dr {
                8
            } else {
                6
            }
        }
    }

    /// +2C66 check_and_decrement_counter: up the stairs.
    fn move_up(&mut self) -> i16 {
        if !self.gr.stair_top_up {
            if self.gr.stair_ur && self.gr.step_r {
                self.y -= 0x100;
                self.step_right();
                7
            } else if self.gr.stair_ul && self.gr.step_l {
                self.y -= 0x100;
                self.step_left();
                5
            } else {
                0
            }
        } else {
            self.y -= 0x100;
            1
        }
    }

    /// +350C reset_input_state_flags: reads the window and works out the ground and stairs flags; an exit mark in
    /// the body row starts a room change (+306C). The moving platforms of screen files 6, 8 and 16 (+33C2) and
    /// the spawn and item marks are part 3.
    pub fn ground(&mut self, t: &Tables, rooms_van: &[i16], sf: &ScreenFile) {
        self.read_grid(sf);
        let e0 = self.g[4][1];
        self.mark = 0;
        if e0 < 0 {
            let kind = (e0 as u8) & 0x70;
            if kind == 0x30 {
                self.exit(t, rooms_van, sf, (e0 & 0xF) as i16);
            } else {
                self.mark = e0;
            }
        }
        let g = |r: usize, c: usize| self.g[r][c] as u8;
        let s = |r: usize, c: usize| self.g[r][c];
        let not_solid = |v: u8| v & 0x81 != 1;
        let under = g(5, 0) | g(5, 1) | g(5, 2);
        let none = under & 0xF == 0;
        let mut f = Ground {
            nothing_under: none,
            under_d: under & 0xD != 0,
            can_left: not_solid(g(4, 0) | g(3, 0) | g(2, 0) | g(1, 0)),
            can_right: not_solid(g(4, 2) | g(3, 2) | g(2, 2) | g(1, 2)),
            head_free: s(0, 1) != 1,
            step_l: g(4, 0) & 7 != 0,
            step_r: g(4, 2) & 7 != 0,
            up_l_free: not_solid(g(3, 0) | g(2, 0) | g(1, 0) | g(0, 0)),
            up_r_free: not_solid(g(3, 2) | g(2, 2) | g(1, 2) | g(0, 2)),
            down_l_free: not_solid(g(5, 0) | g(4, 0) | g(3, 0) | g(2, 0)) && !none,
            down_r_free: not_solid(g(5, 2) | g(4, 2) | g(3, 2) | g(2, 2)) && !none,
            ..Ground::default()
        };
        let stairish = |v: i8| v == 0 || v == 4 || v == 2 || v < 0;
        f.stair_ul = s(4, 0) == 2 && stairish(s(3, 0));
        f.stair_ur = s(4, 2) == 2 && stairish(s(3, 2));
        if s(5, 1) == 2 {
            f.on_stair = true;
            if s(5, 2) <= 0 && s(4, 0) == 2 {
                f.stair_dr = true;
            }
            if s(5, 0) <= 0 && s(4, 2) == 2 {
                f.stair_dl = true;
            }
            if s(4, 0) < 1 && s(4, 2) < 1 {
                f.stair_flat = true;
                if s(5, 2) <= 0 || s(5, 2) == 4 {
                    f.stair_dr = true;
                }
                if s(5, 0) <= 0 || s(5, 2) == 4 {
                    f.stair_dl = true;
                }
            }
        }
        if s(4, 1) == 2 && s(3, 1) == 0 {
            f.stair_top_up = true;
        }
        if s(5, 1) == 4 {
            f.stair_top = true;
            if s(5, 2) == 2 {
                f.stair_dl = true;
            }
            if s(5, 0) == 2 {
                f.stair_dr = true;
            }
        }
        self.gr = f;
    }

    /// +306C set_tile_collision_flags: leaves the room by exit `k` of its screen file (-1: the stage's restart
    /// room). Mode 3 exits look for the fade point mark (0x7F) near the player.
    pub fn exit(&mut self, t: &Tables, rooms_van: &[i16], sf: &ScreenFile, k: i16) {
        self.room_change = 1;
        self.changing = 1;
        if k == -1 {
            self.room = at(&t.stage_room, self.stage as i32 - 1) as i16;
        } else {
            let van = at(rooms_van, self.room as i32) as i32;
            let e = van * 12 + k as i32 * 3;
            self.room = at(&t.exits, e);
            self.door = at(&t.exits, e + 1);
            self.entry_mode = at(&t.exits, e + 2);
            if self.entry_mode == 3 {
                let row = self.scroll_col as i32 + ((self.y >> 8) as i32 - 2) * self.map_w as i32;
                let c0 = (self.x >> 3) as i32;
                self.fade_point = 100;
                for c in c0..c0 + 10 {
                    if sf.at(c + row) == 0x7F {
                        self.fade_point = (c - c0) as i16;
                        break;
                    }
                }
                if self.fade_point == 100 {
                    let mut c = c0;
                    while c > c0 - 10 {
                        if sf.at(c + row) == 0x7F {
                            self.fade_point = (c - c0) as i16;
                            break;
                        }
                        c -= 1;
                    }
                }
            }
        }
        self.crouch = 0;
        self.whip_anim = 0;
        self.whip = 0;
        self.jump = 0;
        self.falling = 0;
        self.dx = 0;
    }

    /// +2CB4 player_walk_and_jump: gravity. With nothing solid under the feet the player falls up to three rows a
    /// frame; landing ends the fall.
    fn gravity(&mut self, t: &Tables, sf: &ScreenFile) {
        let b = self.floor_bits(sf, self.x, self.y, 0x100);
        if b & 0xF == 0 && self.changing == 0 {
            if self.falling == 0 {
                self.falling += 1;
                self.dx = 0;
                if !self.keeps_pose() {
                    self.set_was(ST_FALL);
                    self.anim = self.sel(t, 0, -2);
                    self.frame = 0;
                }
            }
            self.y += 0x100;
            let b = self.floor_bits(sf, self.x, self.y, 0x100);
            if b & 1 == 0 && b & 8 == 0 {
                self.y += 0x100;
                let b = self.floor_bits(sf, self.x, self.y, 0x100);
                if b & 1 == 0 && b & 8 == 0 {
                    self.y += 0x100;
                    self.read_grid(sf);
                    return;
                }
            }
        }
        self.read_grid(sf);
        if self.falling != 0 {
            self.sfx.push(Sfx::Land);
            self.falling = 0;
            self.anim = self.back_from(t, ST_FALL);
            if self.y & 0xFF != 0 {
                self.y = (self.y.wrapping_add(0x100)) & !0xFF;
            }
        }
    }

    /// +2F46 reset_game_state_on_condition: a jump starts when the button was pressed with ground under the feet
    /// and nothing solid over the head.
    fn jump_start(&mut self, t: &Tables, head: i8, under: u16) {
        if self.jump_req != 0
            && head != 1
            && (under & 1 != 0 || under & 4 != 0 || under & 8 != 0 || self.gr.stair_flat)
        {
            self.jump = 1;
            self.set_was(ST_JUMP);
            self.jump_phase = 1;
            self.anim = self.sel(t, 0xA, -2);
            self.frame = 0;
        }
    }

    /// +2DC8 process_animation_frame: a jump step from the jump table (up, the top, down until it lands).
    fn jump_step(&mut self, t: &Tables, sf: &ScreenFile) {
        if self.jump_req != 0 && self.jump == 0 {
            let u = (self.g[5][0] as u8 | self.g[5][1] as u8 | self.g[5][2] as u8) as u16;
            self.jump_start(t, self.g[0][1], u);
        }
        if self.jump == 0 {
            return;
        }
        let step = at(&t.jump, self.jump as i32 - 1);
        if self.jump == 3 {
            self.y -= 0x100;
            self.read_grid(sf);
        }
        if step < 1 {
            self.jump_phase = 1;
            if self.g[0][1] == 1 {
                // head against a ceiling: on to the matching falling step
                while -step != 0 && at(&t.jump, self.jump as i32) != -step {
                    self.jump += 1;
                }
                self.jump_phase = 0;
            } else {
                self.y = self.y.wrapping_add(step);
                self.read_grid(sf);
            }
            self.jump += 1;
        } else {
            self.jump_phase = 2;
            let b = self.floor_bits(sf, self.x, self.y, step);
            if b & 0x70 == 0 && (b & 1 != 0 || b & 8 != 0 || b & 2 != 0) {
                if self.y & 0xFF != 0 {
                    self.y = (self.y & !0xFF).wrapping_add(0x100);
                }
                self.anim = self.back_from(t, ST_JUMP);
                self.frame = 1;
                self.dx = 0;
                self.jump = 0;
                self.jump_phase = 0;
                self.read_grid(sf);
            } else {
                self.y = self.y.wrapping_add(step);
                self.read_grid(sf);
                self.jump += 1;
            }
        }
        if step == 0 {
            self.jump_phase = 0;
        }
        if self.jump > 9 {
            self.falling = 1;
            self.anim = self.back_from(t, ST_JUMP);
            self.frame = 1;
            self.dx = 0;
            self.jump = 0;
            self.jump_phase = 0;
        }
    }

    /// +1480 tick_animation_counters: every 10 frames the stopwatch, the invulnerability and the cross count down,
    /// and the stage time; time out starts the death.
    pub fn timers(&mut self) {
        self.tick10 += 1;
        if self.tick10 == 10 {
            self.tick10 = 0;
            if self.stopwatch != 0 {
                self.stopwatch -= 1;
            }
            if self.inv != 0 {
                self.inv -= 1;
                if self.inv == 0 {
                    self.inv_count = 0;
                }
            }
            if self.cross != 0 {
                self.cross -= 1;
                if self.cross == 0 {
                    self.inv_count = 0;
                }
            }
            if self.time > 0 && self.stopwatch == 0 {
                if self.time_on {
                    self.time -= 1;
                }
                if self.time < 0x1E {
                    self.sfx.push(Sfx::TimeWarning);
                }
            }
            if self.time == 0 && self.dying == 0 {
                self.hurt_facing = self.facing;
                self.dying = 0x10;
            }
        }
    }

    /// +386A update_game_state_flags: the player's frame. Order as the original: ground flags (and exits), the
    /// knock-back when hit, else death, whip, jump, walking, stairs and crouching, then the jump step and gravity,
    /// and last the animation for the move state.
    pub fn update(&mut self, t: &Tables, rooms_van: &[i16], sf: &ScreenFile) {
        self.whip_strike = false;
        self.throw_strike = false;
        // -$12BC is never set: the frame counter runs every frame
        self.frame += 1;
        if self.changing == 0 {
            self.ground(t, rooms_van, sf);
            if self.room_change != 0 {
                return;
            }
            let mut normal = self.hurt == 0 || self.hurt > 6;
            if !normal {
                // knock-back (hurt 1 to 6)
                if self.hurt == 1 {
                    self.dx = if self.hurt_facing == 4 { 3 } else { 4 };
                    self.set_was(ST_HURT);
                    self.frame = 0;
                    self.whip_anim = 0;
                    self.falling = 0;
                    self.jump = 0;
                    self.whip = 0;
                    self.was[ST_FALL] = false;
                    self.was[ST_CROUCH] = false;
                    self.was[ST_JUMP] = false;
                    self.was[ST_WHIP] = false;
                    self.anim = t.hurt_anim as i16;
                    self.inv += 5;
                }
                self.mv = self.facing;
                self.y = self.y.wrapping_add(at(&t.knock, self.frame as i32));
                if self.hurt_facing == 3 {
                    self.move_left();
                } else {
                    self.move_right();
                }
                self.hurt += 1;
                if self.hurt == 7 {
                    self.frame = 0;
                    self.dx = 0;
                    if self.hp < 1 {
                        self.hurt = 0;
                        self.dying = 0x10;
                        normal = true;
                    } else {
                        self.anim = self.back_from(t, ST_HURT);
                        if self.y & 0xFF != 0 {
                            self.y = (self.y.wrapping_add(0x100)) & !0xFF;
                        }
                        self.read_grid(sf);
                        if self.g[4][1] == 1 {
                            self.y -= 0x100;
                        }
                    }
                }
            }
            if normal {
                self.normal(t, sf);
            }
            let s = self.mv;
            if s != 0 {
                self.last_dir = s;
            }
            if s == 3 || s == 5 || s == 6 {
                self.facing = 3;
            }
            if s == 4 || s == 7 || s == 8 {
                self.facing = 4;
            }
        }
        // +3E78: the animation for the move state
        let n = t.next(self.anim, self.mv);
        if n != self.anim {
            self.frame = 0;
            self.anim = n;
        }
        if self.anim_ended(t) {
            self.frame = 0;
        }
        if self.y > 0x1E00 {
            // fell off the bottom of the screen
            self.hp = 0;
            self.dying = 1;
            self.death_done = 1;
            self.sfx.push(Sfx::Fall);
        }
        let pos = (self.x, self.y, self.scroll_col);
        if pos == self.last_pos {
            self.still += 1;
        } else {
            self.still = 0;
        }
        self.last_pos = pos;
    }

    /// The part of +386A from l0_39B0: death, whip, jump, walking, stairs and crouching.
    fn normal(&mut self, t: &Tables, sf: &ScreenFile) {
        if self.dying != 0 {
            if self.falling == 0
                && self.jump == 0
                && self.crouch == 0
                && self.whip == 0
                && self.dying == 0x10
            {
                self.hp = 0;
                self.frame = 0;
                self.anim = at(&t.death_anim, self.hurt_facing as i32) as i16;
                self.dying -= 1;
                self.sfx.push(Sfx::Death);
            } else if self.crouch == 0 || self.whip != 0 {
                if self.dying < 0x10 {
                    self.dying -= 1;
                    if self.dying == 1 || self.anim_ended(t) {
                        self.frame -= 1;
                        self.hp = 0;
                        self.inv = 0;
                        self.dying = 1;
                        self.cross = 0;
                        self.death_done = 1;
                    }
                }
            } else {
                self.anim = self.back_from(t, ST_CROUCH);
                self.frame = 0;
                self.crouch = 0;
            }
        }
        if self.hurt != 0 {
            self.hurt += 1;
            if self.hurt > 0x1E {
                self.hurt = 0;
            }
        }
        // the whip (or, with up held and a sub-weapon, a throw: part 3)
        if self.falling == 0 && self.fire && self.whip == 0 && self.dying == 0 {
            let up = self.up;
            if !up || self.sub != 0x20 || self.stopwatch != 0 || self.hearts < 5 {
                self.set_was(ST_WHIP);
                self.whip = 1;
                self.anim = if self.crouch == 0 {
                    self.sel(t, 0x1E, -2)
                } else {
                    self.sel(t, 0x1C, -2)
                };
                // a throw needs no more of the sub-weapon on the screen than the shot multiplier allows
                self.throw_up = up;
                if !up || !self.throw_ok || self.sub == 0x20 || self.sub == 0 || self.hearts < 1 {
                    let row = [4, 0x12, 0x16][self.whip_level.clamp(0, 2) as usize];
                    self.whip_anim = self.sel(t, row, -2);
                } else {
                    self.whip_anim = 0;
                }
                self.frame = 0;
            } else {
                // the stopwatch
                self.stopwatch = 5;
                self.hearts -= 5;
            }
        }
        if self.whip != 0 {
            self.whip += 1;
            if !self.anim_ended(t) {
                if t.frame(self.anim, self.frame + 1) == 99 {
                    self.strike = (
                        t.frame(self.whip_anim, self.frame),
                        t.frame(self.anim, self.frame) == 99,
                    );
                    if !self.throw_up
                        || !self.throw_ok
                        || self.sub == 0x20
                        || self.sub == 0
                        || self.hearts < 1
                    {
                        self.whip_strike = true;
                    } else {
                        self.throw_strike = true;
                    }
                }
            } else {
                self.whip_anim = 0;
                self.whip = 0;
                self.anim = self.back_from(t, ST_WHIP);
                self.frame = 0;
            }
        }
        // a jump is asked for (it starts in the jump step if there is ground under the feet)
        if self.jump == 0
            && self.falling == 0
            && self.room_change == 0
            && self.whip == 0
            && self.dying == 0
            && self.jump_btn
            && self.crouch == 0
            && self.g[4][1] != 2
            && self.g[4][0] != 2
            && self.g[4][2] != 2
        {
            self.jump_req = 1;
            let s = if self.left {
                self.dx = -1;
                3
            } else if self.right {
                self.dx = 1;
                4
            } else {
                self.dx = 0;
                0
            };
            if s != 0 && self.last_dir != s {
                self.facing = s;
                self.last_dir = s;
                self.mv = s;
            }
        }
        if self.hurt == 0 || self.hurt > 6 {
            self.mv = 0;
        }
        let mut to_jump_step = false;
        if self.dying == 0
            && self.whip == 0
            && self.crouch == 0
            && self.falling == 0
            && self.jump == 0
            && self.jump_req == 0
        {
            if !(self.hurt != 0 && self.hurt < 7) {
                let mut walk = true;
                if self.up {
                    let s = self.move_up();
                    self.mv = s;
                    walk = s == 0;
                }
                if walk {
                    let mut done = false;
                    if self.down {
                        let s = self.move_down();
                        self.mv = s;
                        if s != 0 {
                            to_jump_step = true;
                            done = true;
                        }
                    }
                    if !done {
                        if self.left {
                            self.mv = self.move_left();
                        }
                        if self.right {
                            self.mv = self.move_right();
                        }
                        if self.centre {
                            self.mv = 0;
                        }
                        if self.down && self.crouch == 0 && self.mv == 0 {
                            self.set_was(ST_CROUCH);
                            self.anim = self.sel(t, 6, -2);
                            self.mv = self.last_dir;
                            self.frame = 0;
                            self.dx = 0;
                            self.crouch = 1;
                        }
                    }
                }
            }
        } else if self.dx < 0 {
            self.move_left();
        } else if self.dx > 0 {
            self.move_right();
        }
        if !to_jump_step {
            // l0_3D9C: crouching
            if self.crouch != 0 && !self.down && self.whip == 0 {
                self.anim = self.back_from(t, ST_CROUCH);
                self.frame = 0;
                self.crouch = 0;
            } else if self.crouch != 0 && self.whip == 0 && (self.left || self.right) {
                let s = if self.left { 3 } else { 4 };
                if self.last_dir != s {
                    self.set_was(ST_CROUCH);
                    self.frame = 0;
                    self.facing = s;
                    self.last_dir = s;
                    self.mv = s;
                    self.anim = self.sel(t, if self.left { 6 } else { 7 }, -1);
                }
            }
        }
        // l0_3E2A
        self.jump_step(t, sf);
        self.jump_req = 0;
        if self.jump == 0 {
            self.gravity(t, sf);
        }
    }

    /// The castshps frame to draw for the player, and for the whip (+188A call_if_flag_set).
    pub fn shapes(&self, t: &Tables) -> (u8, Option<u8>) {
        let f = t.frame(self.anim, self.frame);
        let w = (self.whip_anim != 0).then(|| t.frame(self.whip_anim, self.frame));
        (f, w)
    }
}

use crate::castle::{Castle, Phase};
use crate::objects::{Hdr, World};
use crate::screen::Into;

/// Vertical blanks a game frame takes during a stage (+BD00 store_word_into_global, called with 4 after the
/// title): the stage runs at 12.5 frames a second.
pub const VBLANKS: u32 = 4;
/// The display's scroll moves this many pixels a vertical blank towards the map position (+C7A4).
pub const SCROLL_STEP: i32 = 2;

/// The goodies file's frames the panel uses (the original's shape pointers -$1646, -$165E, -$1662, -$157E and
/// -$1586, which point at goodies frames 10, 4, 3, 60 and 58), the sub-weapon frame (-$15EE/-$1582: 30 and
/// 59), the shot multipliers (-$15F2, -$15F6: 14 and 13) and the broken wall (-$159A: 53).
const PANEL: usize = 10;
const PANEL_A: usize = 4;
const PANEL_B: usize = 3;
const BAR_EMPTY: usize = 60;
const BAR_FULL: usize = 58;
const WALL: usize = 53;

/// The damage a hit does to the player by block of rooms (+0ED2: -$1382).
const HIT_DMG: [i16; 7] = [2, 2, 3, 3, 4, 4, 4];

impl Castle {
    fn hdr_of(a: &Assets, file: &str, frame: usize) -> Hdr {
        a.shape(file, frame)
            .map(|s| Hdr {
                h: s.h as i16,
                ax: s.ax,
                ay: s.ay,
                cw: s.cw,
            })
            .unwrap_or_default()
    }

    /// +0C22 load_stage_enemy_art: block `b`'s enemy art files into the shape table (the program's table at
    /// -$7C02: file, first and last entry, nine a block; file names at -$7CFE, nine bytes each).
    fn load_block_shapes(&mut self, b: i16) {
        let a = self.a.clone();
        for j in 0..9 {
            let e = 0x7C02 - 6 * j - 0x36 * b as i32;
            let fi = self.w.m.w(e);
            if fi == -1 {
                break;
            }
            let first = self.w.m.w(e - 2);
            let last = self.w.m.w(e - 4);
            let mut name = String::new();
            for i in 0..9 {
                let c = self.w.m.ub(0x7CFE - fi as i32 * 9 - i);
                if c == 0 {
                    break;
                }
                name.push((c as char).to_ascii_lowercase());
            }
            for (n, ix) in (first..=last).enumerate() {
                if (0..0xD9).contains(&ix) {
                    self.w.hdrs[ix as usize] = Castle::hdr_of(&a, &name, n);
                    self.w.shp[ix as usize] = Some((name.clone(), n));
                }
            }
        }
    }

    /// +0D26 load_castle_shapes_and_goodies (after the title): the player's frames and the goodies file (entries
    /// 0x7C to 0xBD of the shape table).
    fn load_fixed_shapes(&mut self) {
        let a = self.a.clone();
        self.w.phdrs = (0..0x41)
            .map(|i| Castle::hdr_of(&a, "castshps", i))
            .collect();
        for (n, ix) in (0x7C..0xBE).enumerate() {
            self.w.hdrs[ix] = Castle::hdr_of(&a, "goodies", n);
            self.w.shp[ix] = Some(("goodies".to_string(), n));
        }
    }

    /// The game part of +0006 main_program: a new game from the first room, frame after frame until the game is
    /// over or Escape leaves it (+051C, +04BA, then the loop of +1480, +15AA, +386A, +5E7E, +16C6, +63DE, +A4D0
    /// and +1F8E).
    pub fn game(&mut self) {
        let a = self.a.clone();
        let vans: Vec<i16> = a.rooms.iter().map(|r| r.van).collect();
        self.st = Simon::new_game(&a.tables, &vans);
        let rng = self.w.rng;
        self.w = World::new(&a.dat);
        self.w.rng = rng;
        self.load_fixed_shapes();
        self.w.clear_counts();
        self.w.reset_walls();
        self.phase = Phase::Stage;
        self.scr.clip_top = true;
        self.dev_cheats();
        self.enter_room(&a);
        self.panel_cache = None;
        loop {
            self.st.timers();
            if self.st.tick10 == 0 && self.st.stopwatch != 0 {
                self.w.fx(5);
            }
            self.forward_sfx();
            self.w.spawn_timers();
            if self.st.dying == 0 {
                self.player_controls();
            } else {
                self.clear_controls();
            }
            if self.input.pressed(native_engine::input::Action::Cancel) {
                self.st.quit = 1;
            }
            if self.input.pressed(native_engine::input::Action::Pause) {
                self.pause();
            }
            // +350C's platforms (screen files 6, 8 and 16) carry the player
            let carried = match self.st.van {
                6 => self.w.platforms(&self.st, 0, 0),
                8 => self.w.platforms(&self.st, 1, 2),
                0x10 => self.w.platforms(&self.st, 3, 6),
                _ => 0,
            };
            self.sf.attrs.clone_from(&self.w.attrs);
            if carried == 3 {
                self.st.move_left();
                self.st.read_grid(&self.sf);
            } else if carried == 4 {
                self.st.move_right();
                self.st.read_grid(&self.sf);
            }
            let st_sub = self.st.sub;
            self.st.throw_ok = self.w.trec(st_sub, 1) <= self.st.shot;
            self.st.update(&a.tables, &vans, &self.sf);
            self.forward_sfx();
            self.sync_world();
            if self.st.whip_strike {
                let (wf, ended) = self.st.strike;
                self.w.whip(&mut self.st, wf, ended);
            }
            if self.st.throw_strike {
                self.w.throw(&mut self.st);
            }
            let m = self.st.mark;
            if m != 0 {
                let kind = (m as u8) & 0x70;
                let code = (m & 0xF) as i32;
                if kind == 0x10 || kind == 0 {
                    let t = self.w.m.ub(0x7A88 - code - 6 * self.st.van as i32) as i16;
                    self.w.set_trec(t, 0, kind as i16);
                } else if kind == 0x50 {
                    self.w.collect_mark(&mut self.st, code as i16);
                }
            }
            // +5E7E the objects, +16C6 the previous positions, +63DE the hits, +A4D0 the hidden treasures
            self.w.update(&mut self.st);
            self.st.saved_col = self.st.scroll_col;
            self.st.room_change_prev = self.st.room_change;
            self.w.save_prev();
            self.w.save_boss();
            let f = self.st.shapes(&a.tables).0;
            let hit = self.w.collisions(&mut self.st, f);
            if self.st.hurt == 0 && hit {
                self.st.hurt = 1;
            }
            if self.w.rosary != 0 {
                self.rosary(&a);
            }
            self.w.secrets(&self.st);
            self.forward_sfx();
            if !self.frame_end(&a) {
                break;
            }
            if self.st.quit != 0 || self.st.game_over != 0 || self.st.stage_clear != 0 {
                break;
            }
        }
        self.fade_out();
        self.scr.clip_top = false;
        self.scr.scroll = 0;
    }

    /// The developer cheats (off by default). The original had them behind a code typed on its pause screen
    /// (+D394, then keys for collision off +09CC, game time off +0A26, life count off +0A80 and a room number
    /// +0B0C); the native build takes them from the environment variable G2271_CHEATS, a comma list of
    /// `nocollision`, `notime`, `nolives` and `room=N` (N from 0 to 46).
    fn dev_cheats(&mut self) {
        let Ok(v) = std::env::var("G2271_CHEATS") else {
            return;
        };
        for c in v.split(',') {
            let c = c.trim();
            match c {
                "nocollision" => self.st.no_collision = true,
                "notime" => self.st.time_on = false,
                "nolives" => self.st.lives_on = false,
                _ => {
                    if let Some(n) = c.strip_prefix("room=").and_then(|n| n.parse::<i16>().ok()) {
                        if (0..0x2F).contains(&n) {
                            self.st.room = n;
                            self.st.entry_mode = 1;
                        }
                    }
                }
            }
        }
    }

    /// The scroll and the map as the objects see them this frame.
    fn sync_world(&mut self) {
        self.w.scol = self.st.scroll_col;
        self.w.sx = self.st.scroll_col.wrapping_mul(8);
        self.w.block = self.st.block;
    }

    /// The rosary (+7784): ten frames of flashing (the room's colours swapped with white, +B5FE), then every enemy
    /// on the screen dies.
    fn rosary(&mut self, a: &Arc<Assets>) {
        let pal = self.scr.palette;
        for i in 0..10 {
            if i & 1 == 0 {
                for c in self.scr.palette.iter_mut().skip(16) {
                    *c = 0xFFF;
                }
            } else {
                self.scr.palette = pal;
            }
            self.draw_objects(a);
            self.wait_game_frame();
        }
        self.scr.palette = pal;
        self.w.rosary = 0;
        self.w.kill_hostiles(&mut self.st);
    }

    /// The pause (+AFCA pause_screen): the game stops, the picture held, until the pause key again.
    /// +AFCA pause_screen: the music is switched off (its volumes go to nothing at its next row), the banner the
    /// crack put in place of the pause message is shown in the middle of the play area (+C044 with +AE1C: until a
    /// key or the fire button), then the music is switched back as it was and the picture restored. The typed
    /// cheat check that follows (+D394: the keys "661017" in the keyboard buffer) is not ported: the native
    /// build has no typed keys (the developer cheats are G2271_CHEATS).
    fn pause(&mut self) {
        let music = self.snd.music_off;
        self.snd.music_off = true;
        let saved = self.scr.disp.clone();
        let text = PAUSE_BANNER;
        self.scr.ink = 2;
        self.scr.paper = 0;
        let x0 = (crate::assets::W as i32 - 8 * text.len() as i32) / 2;
        for (i, ch) in text.chars().enumerate() {
            let g = self.glyph(ch);
            self.scr.char(&g, x0 + 8 * i as i32, 112, true, true);
        }
        loop {
            self.hold_frame();
            let i = &self.input;
            use native_engine::input::Action;
            if [Action::Pause, Action::Cancel, Action::Fire, Action::Start]
                .iter()
                .any(|a| i.pressed(*a))
            {
                break;
            }
        }
        self.scr.disp = saved;
        self.snd.music_off = music;
    }

    /// +15AA read_player_controls, into the player's flags.
    fn player_controls(&mut self) {
        self.read_controls();
        let c = self.ctl;
        let s = &mut self.st;
        s.up = c.up;
        s.down = c.down;
        s.left = c.left;
        s.right = c.right;
        s.centre = c.centre;
        s.fire = c.fire;
        s.jump_btn = c.fire2;
    }

    /// +156A clear_state_variables: no controls while dying.
    fn clear_controls(&mut self) {
        let s = &mut self.st;
        s.up = false;
        s.down = false;
        s.left = false;
        s.right = false;
        s.centre = true;
        s.fire = false;
        s.jump_btn = false;
    }

    /// +1F8E sync_game_state: the columns scrolled on to are scanned for marks, the frame is drawn and waited
    /// for, then the orb's countdown (the tally and the next block, or the end of the game), the death
    /// sequence's end (a life lost, or the game over) and a room change. False when the game is over.
    fn frame_end(&mut self, a: &Arc<Assets>) -> bool {
        let n = (self.st.saved_col - self.st.prev_col).abs();
        if n != 0 {
            self.scan_n = n;
        }
        self.sync_world();
        if self.st.saved_col < self.st.prev_col {
            self.w.scan(&self.st, true, self.scan_n);
        } else if self.st.prev_col < self.st.saved_col {
            self.w.scan(&self.st, false, self.scan_n);
        }
        self.draw_objects(a);
        self.wait_game_frame();
        self.play_sounds();
        self.st.prev_col = self.st.saved_col;
        if self.w.orb != 0 {
            self.w.orb_wait -= 1;
            if self.w.orb_wait < 0x14 {
                self.w.orb_wait = 0;
                self.w.orb = 0;
                self.panel_cache = None;
                self.tally(a);
                if self.st.block > 4 {
                    self.st.stage_clear = 1;
                    return true;
                }
                let k = self.w.m.ub(0x79B2 - self.st.block as i32) as i16;
                let vans: Vec<i16> = a.rooms.iter().map(|r| r.van).collect();
                self.st.exit(&a.tables, &vans, &self.sf, k);
                self.st.room_change_prev = self.st.room_change;
            }
        }
        if self.st.death_done == 1 && self.st.hp < 1 && self.st.dying == 1 {
            self.st.dying = 0;
            self.st.death_done = 0;
            if self.st.lives_on {
                self.st.lives -= 1;
                if self.st.lives < 0 {
                    self.st.game_over = 1;
                    return false;
                }
            }
            let vans: Vec<i16> = a.rooms.iter().map(|r| r.van).collect();
            self.st.exit(&a.tables, &vans, &self.sf, -1);
            self.st.room_change_prev = self.st.room_change;
            self.st.reset_player(&a.tables);
            self.w.clear_counts();
            self.w.reset_walls();
        }
        if self.st.room_change_prev != 0 {
            self.enter_room(a);
            self.st.saved_col = self.st.scroll_col;
            self.st.room_change_prev = self.st.room_change;
        }
        true
    }

    /// +1E68 decrement_counters_and_call_helpers: after the orb, the time left scores 10 a second and the
    /// hearts 100 each, counted down on the panel (a step every two vertical blanks, +CC16 twice).
    fn tally(&mut self, a: &Arc<Assets>) {
        while self.st.time > 0 {
            self.w.score(&mut self.st, 10);
            self.w.fx(4);
            self.st.time -= 1;
            self.tally_step(a);
        }
        while self.st.hearts > 0 {
            self.w.score(&mut self.st, 100);
            self.w.fx(5);
            self.st.hearts -= 1;
            self.tally_step(a);
        }
        self.st.time = 0;
        self.st.hearts = 0;
    }

    fn tally_step(&mut self, a: &Arc<Assets>) {
        self.draw_objects(a);
        self.wait_frame();
        self.hold_frame();
        self.play_sounds();
    }

    /// The frame's sound requests go to the sound part.
    fn play_sounds(&mut self) {
        self.flush_sounds();
    }

    /// The frame wait of a stage (+CBB8 with -$2BB6 = 4): the frame shows for four vertical blanks while the
    /// display's scroll eases towards the map position; presses during the four count.
    fn wait_game_frame(&mut self) {
        let target = self.st.scroll_col as i32 * 8;
        self.ease_scroll(target);
        self.wait_frame();
        let mut pressed = self.input.pressed;
        for _ in 1..VBLANKS {
            self.ease_scroll(target);
            self.hold_frame();
            pressed |= self.input.pressed;
        }
        self.input.pressed = pressed;
    }

    fn ease_scroll(&mut self, target: i32) {
        let d = target - self.scr.scroll;
        self.scr.scroll += if d.abs() > 8 {
            d
        } else {
            d.clamp(-SCROLL_STEP, SCROLL_STEP)
        };
    }

    /// +0ED2 initialize_serial_device (the room set-up) and +20EC process_level_tilemap (the room drawn): the
    /// block of rooms (its enemy art, its damage, its tune and its time), the screen file, the player where the
    /// room table puts him, the objects of the screen's marks, the panel, then a fade in. The door and stair-exit
    /// pictures (+A162) and the castle map between blocks (+A21A, +A2FA) are not drawn yet.
    fn enter_room(&mut self, a: &Arc<Assets>) {
        let t = &a.tables;
        self.w.clear_room();
        let st = &mut self.st;
        st.changing = 0;
        st.scroll_lock = 0;
        st.inv = 0;
        st.cross = 0;
        let room = st.room.clamp(0, a.rooms.len() as i16 - 1);
        st.room = room;
        let r = a.rooms[room as usize];
        st.stage = at(&t.van_stage, r.van as i32) as i16;
        let old_block = st.block;
        let old = if st.prev_room < 0 {
            None
        } else {
            room_range(st.prev_room)
        };
        let mut new_block = None;
        if let Some(n) = room_range(room) {
            if old != Some(n) {
                st.block = [0, 1, 2, 3, 4, 5, 5][n];
                if st.hearts == 0 {
                    st.hearts = 5;
                }
                new_block = Some(n);
            }
        }
        if st.block != old_block {
            st.time = at(&t.time, st.stage as i32 - 1) as i16 * 100;
        }
        if st.new_life == 1 {
            st.entry_mode = 1;
            st.new_life = 0;
        }
        st.van = r.van;
        st.scroll_col = r.scroll;
        st.x = r.x << 3;
        st.y = r.y << 8;
        let f = if r.facing == 1 { 4 } else { 3 };
        st.last_dir = f;
        st.facing = f;
        st.mv = f;
        st.anim = r.facing;
        st.frame = 0;
        st.prev_col = st.scroll_col;
        st.saved_col = st.scroll_col;
        st.room_change_prev = 0;
        st.room_change = 0;
        st.jump = 0;
        st.falling = 0;
        st.dx = 0;
        st.crouch = 0;
        st.whip_anim = 0;
        st.whip = 0;
        st.was = [false; 6];
        st.enemy_hp = 0x10;
        st.changing = 0;
        st.prev_room = room;
        if let Some(n) = new_block {
            self.load_block_shapes(n as i16);
            self.w.hit_dmg = HIT_DMG[n];
            let b = self.st.block;
            self.w.tune(b);
        }
        self.fade_out();
        // +0E14 loads and depacks the screen file (+D418, +D4BE): on the original that took about a fiftieth of a
        // second for each 51 packed bytes, less 9 frames (measured on the reference harness: VAN0, 4,371 bytes, 76
        // frames; VAN1, 10,907 bytes, 203), with the screen black
        let bytes = a.screen_bytes.get(&(r.van as usize)).copied().unwrap_or(0);
        for _ in 0..load_frames(bytes) {
            self.hold_frame();
        }
        // +0E14 and +20EC: the screen file, every column of it into the three pictures
        let Ok(sf) = a.screen(r.van as usize) else {
            return;
        };
        self.sf = sf.clone();
        self.st.map_w = sf.w as i16;
        self.scr.resize(sf.w * 8);
        for row in 3..25 {
            for c in 0..sf.w {
                let tile = sf.tile(c, row) as usize;
                if let Some(px) = sf.tile_px.get(tile) {
                    self.scr.tile((c * 8) as i32, (row * 8) as i32, px);
                }
            }
        }
        self.scr.scroll = self.st.scroll_col as i32 * 8;
        self.w.attrs = sf.attrs.clone();
        self.w.mw = sf.w as i16;
        self.w.van = r.van;
        self.sync_world();
        let deco = self.w.room_marks(&self.st);
        if let Some((file, frame)) = self.w.shp[0x7F].clone() {
            if let Some(s) = a.shape(&file, frame) {
                for (x, y) in deco {
                    self.scr.shape(s, x, y, Into::CleanShow, false);
                }
            }
        }
        self.panel(a);
        // +BDAE twice: both display pictures start as the clean one
        self.scr.copy_clean();
        self.scr.work ^= 1;
        self.scr.copy_clean();
        self.scr.work ^= 1;
        self.room = room;
        self.scroll_col = self.st.scroll_col;
        // l0_1458: a tune that has stopped (the boss's, the orb's, a death's fade) gives way to the block's again
        if self.snd.check_ended() {
            let b = self.st.block;
            self.music(b);
        }
        self.set_palette(room as usize);
    }

    /// +24E4 redraw_status_panel: the panel picture, the numbers, the sub-weapon and both health bars, into the
    /// clean picture.
    fn panel(&mut self, a: &Arc<Assets>) {
        for (f, x, y) in [(PANEL, 0, 0), (PANEL_A, 0x8E, 9), (PANEL_B, 0x9C, 9)] {
            if let Some(s) = a.shape("goodies", f) {
                self.scr.shape(s, x, y, Into::CleanShow, false);
            }
        }
        self.panel_cache = None;
        self.sub_cache = None;
        self.panel_numbers(a);
        // +255C and +2584: the lost part of each health bar
        self.st.hp_shown = self.st.hp;
        for i in self.st.hp_shown.max(0)..16 {
            self.bar(a, BAR_EMPTY, i, 0x10);
        }
        let e = self.st.enemy_hp;
        if e - 1 < 0 {
            for i in 0..16 {
                self.bar(a, BAR_FULL, i, 0x17);
            }
        } else {
            for i in e..16 {
                self.bar(a, BAR_EMPTY, i, 0x17);
            }
        }
    }

    /// One segment of a health bar (+77B6, +77E6, +781C, +784C): x = 0x49 + 6i.
    fn bar(&mut self, a: &Arc<Assets>, frame: usize, i: i16, y: i32) {
        if let Some(s) = a.shape("goodies", frame) {
            self.scr
                .shape(s, 0x49 + 6 * i as i32, y, Into::CleanShow, false);
        }
    }

    /// +7640, +7664, +7688, +76AE, +76D4: score, time, hearts, lives and stage, each drawn when it changes
    /// (+1A40 compares them with the values last drawn); +76F8 the sub-weapon and +7736 the shot multiplier.
    fn panel_numbers(&mut self, a: &Arc<Assets>) {
        let s = &self.st;
        let now = (s.score, s.time, s.hearts, s.lives, s.stage);
        let old = self.panel_cache;
        if old != Some(now) {
            self.scr.ink = 3;
            self.scr.paper = 0;
            let items: [(i32, i32, String); 5] = [
                (0x48, 0, format!("{:07}", now.0.clamp(0, 9_999_999))),
                (0xD8, 0, format!("{:04}", now.1.clamp(0, 9999))),
                (0xE8, 8, format!("{:02}", now.2.clamp(0, 99))),
                (0xE8, 0x10, format!("{:02}", now.3.clamp(0, 99))),
                (0x130, 0, format!("{:02}", now.4.clamp(0, 99))),
            ];
            for (k, (x, y, text)) in items.iter().enumerate() {
                let changed = match old {
                    None => true,
                    Some(o) => [
                        o.0 != now.0,
                        o.1 != now.1,
                        o.2 != now.2,
                        o.3 != now.3,
                        o.4 != now.4,
                    ][k],
                };
                if !changed {
                    continue;
                }
                for (i, ch) in text.chars().enumerate() {
                    let g = self.glyph(ch);
                    self.scr.char(&g, x + 8 * i as i32, *y, true, true);
                }
            }
            self.panel_cache = Some(now);
        }
        let sub = (self.st.sub, self.st.shot);
        if self.sub_cache != Some(sub) {
            self.sub_cache = Some(sub);
            // +1A40 clears the two boxes (goodies 59 and 32), +76F8 draws the weapon's first frame at
            // (0xBD, 0x13), +7736 the multiplier (goodies 14 or 13) at (0x11D, 0x18)
            if let Some(s) = a.shape("goodies", 59) {
                self.scr.shape(s, 0xBD, 0x18, Into::CleanShow, false);
            }
            if let Some(s) = a.shape("goodies", 32) {
                self.scr.shape(s, 0x11D, 0x18, Into::CleanShow, false);
            }
            if sub.0 != 0 {
                let ix = self.w.m.w(0x630A - 0x5A * sub.0 as i32);
                if let Some((file, frame)) = (0..0xD9)
                    .contains(&ix)
                    .then(|| self.w.shp[ix as usize].clone())
                    .flatten()
                {
                    if let Some(s) = a.shape(&file, frame) {
                        self.scr.shape(s, 0xBD, 0x13, Into::CleanShow, false);
                    }
                }
            }
            let m = match sub.1 {
                1 => Some(14),
                2 => Some(13),
                _ => None,
            };
            if let Some(f) = m {
                if let Some(s) = a.shape("goodies", f) {
                    self.scr.shape(s, 0x11D, 0x18, Into::CleanShow, false);
                }
            }
        }
    }

    /// +1A40 process_input_events: the frame's shapes (a broken wall, the twin bosses' gate, the objects in depth
    /// order, then the player and his whip, +188A) and the panel's changes: numbers, the player's health bar a
    /// segment every second frame, and the boss's bar.
    fn draw_objects(&mut self, a: &Arc<Assets>) {
        let t = &a.tables;
        self.w.free_gone();
        if self.w.wall_draw != 0 {
            let i = self.w.wall_draw as i32 - 1;
            self.w.wall_draw = 0;
            let e = 0x3D06 - 0xC * i - 0x24 * self.st.van as i32;
            let col = self.w.m.w(e - 2) as i32;
            let row = self.w.m.w(e - 4) as i32;
            if let Some(s) = a.shape("goodies", WALL) {
                self.scr
                    .shape(s, (col + 1) * 8, row * 8 + 7, Into::CleanShow, false);
            }
        }
        if self.w.flash != 0 {
            self.w.flash = 0;
            let sx = self.st.scroll_col as i32 * 8;
            if let Some(s) = a.shape("goodies", WALL) {
                for (x, y) in [
                    (0x68, 0x70),
                    (0x68, 0x7E),
                    (0x78, 0x70),
                    (0x78, 0x7E),
                    (0x7E, 0x78),
                ] {
                    self.scr.shape(s, sx + x, y, Into::CleanShow, false);
                }
            }
        }
        for d in self.w.draws() {
            if let Some(s) = a.shape(&d.file, d.frame) {
                self.scr.shape(s, d.x, d.y, Into::Work, false);
            }
        }
        let (f, w) = self.st.shapes(t);
        let sx = self.st.scroll_col as i32 * 8 + self.st.x as i32;
        let sy = (self.st.y >> 5) as i32 + 1;
        let flicker = self.st.cross + self.st.inv != 0;
        let mut show = true;
        if flicker {
            self.st.inv_count = self.st.inv_count.wrapping_add(1);
            show = self.st.inv_count & 1 == 0;
        }
        if show {
            if let Some(s) = a.shape("castshps", f as usize) {
                self.scr.shape(s, sx, sy, Into::Work, false);
            }
        }
        if let Some(w) = w {
            let wy = sy + if self.st.crouch != 0 { 8 } else { 0 };
            if !flicker || self.st.inv_count & 1 == 0 {
                if let Some(s) = a.shape("castshps", w as usize) {
                    self.scr.shape(s, sx, wy, Into::Work, false);
                }
            }
        }
        self.panel_numbers(a);
        // the boss's bar (+19D8 starts it, +1A40 moves it a segment a frame)
        if self.w.bar_anim == 0 {
            self.w.boss_bar(&self.st);
        }
        for (i, full) in self.w.bar_step(&mut self.st) {
            if (0..16).contains(&i) {
                self.bar(a, if full { BAR_FULL } else { BAR_EMPTY }, i, 0x17);
            }
        }
        // +1A40's health bar: two frames a segment
        let st = &mut self.st;
        if st.hp_anim != 0 {
            if st.hp_dir < 0 {
                if st.hp_anim > 1 {
                    st.hp_shown -= 1;
                }
                let i = st.hp_shown;
                st.hp_anim -= 1;
                if i < 16 {
                    self.bar(a, BAR_EMPTY, i, 0x10);
                }
            } else if st.hp_dir > 0 {
                let i = st.hp_shown;
                st.hp_anim -= 1;
                if st.hp_anim == 0 {
                    st.hp_shown += 1;
                }
                self.bar(a, BAR_FULL, i, 0x10);
            }
        }
        let st = &mut self.st;
        if st.hp != st.hp_shown && st.hp_anim == 0 {
            st.hp_anim += 2;
            st.hp_dir = if st.hp_shown < st.hp { 1 } else { -1 };
        }
    }
}
