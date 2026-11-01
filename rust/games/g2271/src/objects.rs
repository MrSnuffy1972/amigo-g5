//! The objects of Castlevania (part 3 of the port): enemies, bosses, candles, items, sub-weapons, shots, score
//! pop-ups, moving platforms and hidden treasures. Each routine keeps the original's name and offset (`+XXXX`) in
//! its doc comment.
//!
//! The original keeps its object state in globals and tables of its data area (sixteen 0x70-byte object records
//! from -$2136(a4), per-type counters from -$4474, item marks from -$13D4, platform, wall and treasure tables), and
//! most of the behaviour is driven by its type table (-$7956, 0x2C bytes a type). The port keeps that state the way
//! the original laid it out, in a [`Mem`] model of the data area built from the program's own initial data (from the
//! asset pack, never the repo), so the routines below read like the recovered code: `g(k, F)` is field F of object
//! k, `ty(t, n)` byte n of type t. The player's own record (object 0) is [`Simon`], read and written directly.

use crate::stage::Simon;

/// The program's data area: initial data from the program, then zeroed globals, addressed as the original did,
/// relative to its base register (`-off(a4)`): byte `0x7FFE - off`.
#[derive(Clone)]
pub struct Mem {
    pub m: Vec<u8>,
}

impl Mem {
    pub fn new(init: &[u8]) -> Mem {
        let mut m = vec![0u8; 0x8000];
        let n = init.len().min(m.len());
        m[..n].copy_from_slice(&init[..n]);
        Mem { m }
    }
    fn ix(g: i32) -> Option<usize> {
        let i = 0x7FFE - g;
        (0..0x7FFF).contains(&i).then_some(i as usize)
    }
    /// The byte, signed and unsigned, and the word of the global at -g(a4) (0 outside the area).
    pub fn b(&self, g: i32) -> i8 {
        Mem::ix(g).map(|i| self.m[i] as i8).unwrap_or(0)
    }
    pub fn ub(&self, g: i32) -> u8 {
        self.b(g) as u8
    }
    pub fn w(&self, g: i32) -> i16 {
        Mem::ix(g)
            .map(|i| i16::from_be_bytes([self.m[i], self.m[i + 1]]))
            .unwrap_or(0)
    }
    pub fn sw(&mut self, g: i32, v: i16) {
        if let Some(i) = Mem::ix(g) {
            self.m[i..i + 2].copy_from_slice(&v.to_be_bytes());
        }
    }
    pub fn sb(&mut self, g: i32, v: i8) {
        if let Some(i) = Mem::ix(g) {
            self.m[i] = v as u8;
        }
    }
}

// ---- object record fields (byte offsets in the 0x70-byte record at -$2136 + 0x70 k)
pub const X: i32 = 0x00; // map x in pixels (the player: x on screen)
pub const Y: i32 = 0x02; // y in rows, 8.8
pub const F04: i32 = 0x04; // from the type table +0x0A
pub const HX: i32 = 0x08; // where it was made (column << 3) and row (8.8)
pub const HY: i32 = 0x0A;
pub const SEEN_PY: i32 = 0x0C; // the player's y and its own y last frame (types 3, 7, 8)
pub const SEEN_Y: i32 = 0x0E;
pub const FR: i32 = 0x12; // animation frame (-1: start)
pub const AN: i32 = 0x16; // moving direction (1 up, 2 down, 3 left, 4 right)
pub const FC: i32 = 0x18; // facing (3 left, 4 right)
pub const VD: i32 = 0x1A; // vertical direction (1 up, 2 down)
pub const NOFLOOR: i32 = 0x22; // nothing solid under it (+3F32)
pub const KIND: i32 = 0x32; // 0x1B/0x1C candles, 0x1D items, 0x33 the player's weapons, 0x34 effects,
                            // 0x35 and 0x36 platforms, 0x37 shots, 0x38 enemies, 0x39 bosses
pub const ST: i32 = 0x34; // 0 free, 1 alive, 2 hit, 4 gone, 8 new
pub const PST: i32 = 0x36; // the state last frame
pub const STEP: i32 = 0x3C; // step in its jump table
pub const STEP2: i32 = 0x3E; // step in its wave table
pub const TYPE: i32 = 0x40;
pub const PH: i32 = 0x42; // phase (per type)
pub const A44: i32 = 0x44;
pub const A46: i32 = 0x46;
pub const A48: i32 = 0x48;
pub const FALL: i32 = 0x4A; // falling (frames)
pub const JMP: i32 = 0x4C; // in a jump
pub const WAIT: i32 = 0x4E; // waiting for the player (distance in columns)
pub const VX: i32 = 0x50; // x speed while falling
pub const MODE: i32 = 0x52; // 2: hurts the player
pub const LIFE: i32 = 0x54; // frames left (0: for ever)
pub const OWN: i32 = 0x56; // made by the player (weapons, pop-ups)
pub const CD: i32 = 0x58; // shot cooldown
pub const SHOOT: i32 = 0x5A; // shooting (frames left)
pub const ITEM: i32 = 0x5C; // item mark it came from (+1)
pub const VULN: i32 = 0x5E; // can be hit
pub const HP: i32 = 0x60;
pub const PHP: i32 = 0x62;
pub const HITBY: i32 = 0x68; // the object a weapon hit last
pub const STUN: i32 = 0x6A; // frames it cannot be hit
pub const PSTUN: i32 = 0x6C;
pub const HITS: i32 = 0x6E; // a weapon's hits (combo score)

/// Sound effects and tunes the game asks for (the original's numbers: +AC24 effects, +ACB0 tunes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Snd {
    Fx(i16),
    Tune(i16),
}

// ---- globals the object code uses (offsets below a4)
pub const G_LIVE: i32 = 0x13D6; // objects alive
pub const G_ITEMS: i32 = 0x13D4; // 10 item marks of 8 bytes: state, mark, column, row
pub const G_TREC: i32 = 0x4474; // per-type records of 8 bytes: spawner on, count, timer, timer reset
pub const G_TYPE: i32 = 0x7956; // the type table, 0x2C bytes a type
pub const G_SHAPEIX: i32 = 0x630A; // shape entry by frame, direction and type
pub const G_ITEMTYPE: i32 = 0x7A88; // spawn and item codes: 6 a screen file
pub const G_WALLS: i32 = 0x3D06; // breakable walls: 3 a screen file of 12 bytes
pub const G_SECRETS: i32 = 0x384C; // hidden treasures: 10 bytes each
pub const G_PLAT: i32 = 0x3892; // moving platforms: 10 bytes each
pub const G_ELEV: i32 = 0x38A4; // the lifts of screen file 9: 8 bytes each
pub const G_CHAIN: i32 = 0x12A8; // the segments of type 0x0E (words)

/// The object state (part 3): the data area, the stage's enemy shapes and what the frame asks for.
#[derive(Clone)]
pub struct World {
    pub m: Mem,
    /// The enemy shape table (-$185E, 0xD9 entries): the art file and frame of each entry, loaded per block of
    /// rooms by +0C22 (enemies) and +0D26 (the goodies file from entry 0x7C).
    pub shp: Vec<Option<(String, usize)>>,
    /// The random generator's state (-$2580): +DBC2.
    pub rng: u32,
    pub snd: Vec<Snd>,
    /// The scroll in pixels (-$19A4) and the first map column (-$19AE), copied in each frame.
    pub sx: i16,
    pub scol: i16,
    /// The room's attribute map (a working copy: platforms, walls and candles change it) and its width.
    pub attrs: Vec<i8>,
    pub mw: i16,
    /// The room's screen file number (room table +8) and the block of rooms (-$19A8).
    pub van: i16,
    pub block: i16,
    /// The game's loop count (-$2174): some walls only break on later loops.
    pub lap: i16,
    /// Map cells cleared by a broken wall (+700C), to redraw as background: (column, row).
    pub cleared: Vec<(i16, i16)>,
    /// A wall piece to draw over the background (-$2162, +1A40) and the rosary's flash (-$131E).
    pub wall_draw: i16,
    pub flash: i16,
    /// The boss: its object (-$1370), health (-$1352, shown -$1350), full health (-$134E), the bar's animation
    /// (-$2158, direction -$1320), the defeated boss's object (-$131C) and the bar as shown (-$2184 in Simon).
    pub boss: i16,
    pub boss_hp: i16,
    pub boss_full: i16,
    pub bar_anim: i16,
    pub bar_dir: i16,
    pub boss_dead: i16,
    /// The orb was taken (-$1314) and the frames to the stage's end (-$1312).
    pub orb: i16,
    pub orb_wait: i16,
    /// -$12AE/-$12AC (a thrown axe-knight's weapon came back), -$12B0/-$12B2/-$12B4 (the twin bosses).
    pub g12ae: i16,
    pub g12ac: i16,
    pub g12b0: i16,
    pub g12b2: i16,
    pub g12b4: i16,
    /// -$12A6 (the last type 0x0E made), -$4212 and -$41EA (counts read through the type records).
    pub g12a6: i16,
    /// -$12AA (rows to the floor found by +5FE8).
    pub g12aa: i16,
    /// The shape headers of the shape table's entries and of the castshps frames (the player, -$14F6).
    pub hdrs: Vec<Hdr>,
    pub phdrs: Vec<Hdr>,
    /// The damage a hit does to the player in this block (-$1382).
    pub hit_dmg: i16,
    /// The boss as it was last frame (+16C6: -$1372, -$1350, -$134C).
    pub prev_boss: i16,
    pub prev_hp: i16,
    pub prev_full: i16,
    /// Frames of the rosary's flash left (+7784), and the frame counter stunned objects blink by (-$1324).
    pub rosary: i16,
    pub blink: i16,
}

/// Random below `n` as the original computed it (+B76E random_below): |r| mod (n - 1), 0 when n < 2.
pub fn below(r: i16, n: i16) -> i16 {
    let d = n - 1;
    if d < 1 {
        return 0;
    }
    ((r as i32).unsigned_abs() as u16 % d as u16) as i16
}

impl World {
    pub fn new(init: &[u8]) -> World {
        World {
            m: Mem::new(init),
            shp: vec![None; 0xD9],
            rng: 0,
            snd: Vec::new(),
            sx: 0,
            scol: 0,
            attrs: Vec::new(),
            mw: 0,
            van: 0,
            block: 0,
            lap: 1,
            cleared: Vec::new(),
            wall_draw: 0,
            flash: 0,
            boss: 0,
            boss_hp: 0,
            boss_full: 0,
            bar_anim: 0,
            bar_dir: 0,
            boss_dead: 0,
            orb: 0,
            orb_wait: 0,
            g12ae: 0,
            g12ac: 0,
            g12b0: 0,
            g12b2: 0,
            g12b4: 0,
            g12a6: 0,
            g12aa: 0,
            hdrs: vec![Hdr::default(); 0xD9],
            phdrs: Vec::new(),
            hit_dmg: 2,
            prev_boss: 0,
            prev_hp: 0,
            prev_full: 0,
            rosary: 0,
            blink: 0,
        }
    }

    // ---- the data area as the original addressed it
    /// Field `f` of object `k`.
    pub fn g(&self, k: i16, f: i32) -> i16 {
        self.m.w(0x2136 - 0x70 * k as i32 - f)
    }
    pub fn s(&mut self, k: i16, f: i32, v: i16) {
        self.m.sw(0x2136 - 0x70 * k as i32 - f, v)
    }
    pub fn add(&mut self, k: i16, f: i32, d: i16) {
        let v = self.g(k, f).wrapping_add(d);
        self.s(k, f, v)
    }
    /// Byte `n` (signed) and word `n` of type `t`'s entry in the type table.
    pub fn ty(&self, t: i16, n: i32) -> i8 {
        self.m.b(G_TYPE - 0x2C * t as i32 - n)
    }
    pub fn tyw(&self, t: i16, n: i32) -> i16 {
        self.m.w(G_TYPE - 0x2C * t as i32 - n)
    }
    /// Type `t`'s record: 0 spawner on, 1 count alive, 2 spawn timer, 3 timer reset.
    pub fn trec(&self, t: i16, k: i32) -> i16 {
        self.m.w(G_TREC - 8 * t as i32 - 2 * k)
    }
    pub fn set_trec(&mut self, t: i16, k: i32, v: i16) {
        self.m.sw(G_TREC - 8 * t as i32 - 2 * k, v)
    }
    /// Object `k`'s type table byte.
    fn oty(&self, k: i16, n: i32) -> i8 {
        self.ty(self.g(k, TYPE), n)
    }
    /// Object `k`'s speed for direction `d` (type table words +2, +4, +6, +8: up, down, left, right).
    fn speed(&self, k: i16, d: i16) -> i16 {
        self.m
            .w(G_TYPE - 0x2C * self.g(k, TYPE) as i32 - 2 * d as i32)
    }
    /// The class of type `t` (type table +0x22): 2 candles, 4 items, 5 not hittable, 6 sub-weapons on the floor.
    pub fn class(&self, t: i16) -> i8 {
        self.ty(t, 0x22)
    }
    /// The shape entry object `k` shows (-$630A[frame, direction, type]); -1 ends an animation.
    pub fn shape_ix(&self, k: i16) -> i16 {
        let f = self.g(k, FR).max(0);
        self.m.w(G_SHAPEIX
            - 2 * f as i32
            - 0x12 * self.g(k, AN) as i32
            - 0x5A * self.g(k, TYPE) as i32)
    }
    /// The attribute byte at flat map offset `i` (0 outside the map).
    pub fn at(&self, i: i32) -> i8 {
        if i < 0 {
            return 0;
        }
        self.attrs.get(i as usize).copied().unwrap_or(0)
    }
    pub fn set_at(&mut self, i: i32, v: i8) {
        if i >= 0 {
            if let Some(c) = self.attrs.get_mut(i as usize) {
                *c = v;
            }
        }
    }
    fn room_van_ix(&self) -> i32 {
        self.van as i32
    }

    /// +DBC2: the generator (r = r * 0x41C64E6D + 0x3039; bits 16 to 30).
    pub fn rand(&mut self) -> i16 {
        self.rng = self.rng.wrapping_mul(0x41C6_4E6D).wrapping_add(0x3039);
        ((self.rng >> 16) & 0x7FFF) as i16
    }
    /// +B76E random_below (|r| mod (n - 1)).
    pub fn rnd(&mut self, n: i16) -> i16 {
        let r = self.rand();
        below(r, n)
    }
    /// +AC24: a sound effect.
    pub fn fx(&mut self, n: i16) {
        self.snd.push(Snd::Fx(n));
    }
    /// +ACB0: a tune.
    pub fn tune(&mut self, n: i16) {
        self.snd.push(Snd::Tune(n));
    }

    /// +787C find_or_create_enemy_entry: a new object in the first free slot (1 to 15), set up from the type table.
    /// None when a candle, item or enemy of the same type already stands there (classes below 3), or no slot is free.
    #[allow(clippy::too_many_arguments)]
    pub fn spawn(
        &mut self,
        mode: i16,
        kind: i16,
        t: i16,
        x: i16,
        y: i16,
        face: i16,
        vd: i16,
    ) -> i16 {
        let mut slot = 0;
        let mut k = 1;
        while slot == 0 && k <= 15 {
            if self.class(t) < 3
                && self.g(k, HX) == x
                && self.g(k, HY) == y
                && t != 0x18
                && t != 0x1A
            {
                return 0;
            }
            let st = self.g(k, ST);
            if st == 0 || st == 4 {
                slot = k;
            } else {
                k += 1;
            }
        }
        if slot == 0 {
            return 0;
        }
        let k = slot;
        let v = self.tyw(t, 0x0A);
        self.s(k, F04, v);
        for f in [HITS, ITEM, SHOOT, JMP, FALL, VX, STEP2, STEP, NOFLOOR, 0x14] {
            self.s(k, f, 0);
        }
        self.s(k, HITBY, -1);
        self.s(k, FR, -1);
        self.s(k, KIND, kind);
        self.s(k, TYPE, t);
        self.s(k, HX, x);
        self.s(k, X, x);
        self.s(k, HY, y);
        self.s(k, Y, y);
        self.s(k, FC, face);
        self.s(k, AN, face);
        self.s(k, VD, vd);
        self.s(k, ST, 8);
        self.s(k, PH, 0);
        self.s(k, WAIT, self.ty(t, 0x14) as i16);
        self.s(k, A48, 0);
        self.s(k, LIFE, self.ty(t, 0x21) as i16);
        self.s(k, MODE, mode);
        self.s(k, CD, self.ty(t, 0x25) as i16);
        self.s(k, VULN, 1);
        self.s(k, STUN, 0);
        let hp = if self.block < 2 || t != 0x11 {
            self.tyw(t, 0x26)
        } else {
            self.tyw(t, 0x26) >> 1
        };
        self.s(k, HP, hp);
        self.s(k, 0x64, 0);
        let live = self.m.w(G_LIVE) + 1;
        self.m.sw(G_LIVE, live);
        self.set_trec(t, 1, self.trec(t, 1) + 1);
        if mode == 2 {
            self.set_trec(t, 2, self.trec(t, 3));
        }
        k
    }

    /// +8924 clear_enemy_slot: the object goes (its item mark and its type's spawn timer are reset).
    pub fn remove(&mut self, k: i16) {
        let st = self.g(k, ST);
        if st == 4 || st == 0 {
            return;
        }
        self.s(k, ST, 4);
        self.s(k, HY, 0);
        self.s(k, HX, 0);
        let live = self.m.w(G_LIVE) - 1;
        self.m.sw(G_LIVE, live);
        let t = self.g(k, TYPE);
        self.set_trec(t, 1, self.trec(t, 1) - 1);
        let it = self.g(k, ITEM);
        if it != 0 {
            self.m.sw(G_ITEMS - 8 * (it as i32 - 1), 0);
        }
        if self.g(k, MODE) == 2 {
            self.set_trec(t, 2, self.trec(t, 3));
        }
    }

    /// +89F6 iterate_table_and_call: every enemy on the screen goes (not the platforms).
    pub fn remove_enemies(&mut self) {
        for k in 1..0x11 {
            let p = self.g(k, PST);
            if (p == 2 || p == 1) && self.g(k, MODE) == 2 && self.g(k, KIND) != 0x35 {
                self.remove(k);
            }
        }
    }

    /// +89C6 check_table_for_value: everything but the platforms goes.
    pub fn remove_all(&mut self) {
        for k in 1..0x11 {
            if self.g(k, KIND) != 0x35 {
                self.remove(k);
            }
        }
    }

    /// +16C6 save_previous_positions (the objects' part): health, state and stun as they were.
    pub fn save_prev(&mut self) {
        for k in 1..0x10 {
            let v = self.g(k, HP);
            self.s(k, PHP, v);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            let v = self.g(k, STUN);
            self.s(k, PSTUN, v);
        }
    }

    /// +171E clear_flags_for_entries: objects that went last frame are freed.
    pub fn free_gone(&mut self) {
        for k in (1..=0x10).rev() {
            if self.g(k, PST) == 4 {
                self.s(k, ST, 0);
                self.s(k, PST, 0);
            }
        }
    }

    /// +01B2 clear_two_word_tables: no spawner on, no counts, no timers.
    pub fn clear_counts(&mut self) {
        for i in 0..0x20 {
            self.set_trec(i, 0, 0);
            self.set_trec(i + 1, 1, 0);
            self.set_trec(i + 1, 2, 0);
        }
    }

    /// +01F4 clear_tile_table and +0230 clear_table_entries: the walls and the hidden treasures are back.
    pub fn reset_walls(&mut self) {
        for v in 1..0x1E {
            for k in 0..3 {
                self.m.sw(0x3CFC - 0xC * k - 0x24 * v, 0);
            }
        }
        for i in 0..0x12 {
            self.m.sw(G_SECRETS - 10 * i, 0);
        }
    }
}

// ---- making objects: from the map, from candles and enemies, the player's weapons and the pop-ups
impl World {
    /// +7A4E update_object_entry: a candle (mark 0x20: kind 0x1B, type 0x1E; mark 0x40: kind 0x1C, type 0x1F) at
    /// map column `col`, row `row`.
    pub fn candle(&mut self, col: i16, row: i16, kind: i16, t: i16) {
        let k = self.spawn(1, kind, t, col << 3, (row + 1) << 8, 0, 0);
        if k != 0 {
            self.s(k, HX, col << 3);
            self.s(k, HY, (row + 1) * 0x100);
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
        }
    }

    /// +8A46 process_object_type: an enemy from a spawn mark (`code` is the mark's low four bits, looked up in the
    /// screen file's codes, or 0x20 as it is) at map column `col` and row `row`.
    pub fn map_enemy(&mut self, s: &Simon, code: i16, col: i16, row: i16) -> i16 {
        let mut code = code;
        if code != 0x20 {
            code = self.m.ub(G_ITEMTYPE - code as i32 - 6 * self.room_van_ix()) as i16;
        }
        let row = row + 1;
        let x = col << 3;
        let y = row.wrapping_mul(0x100);
        let pl = s.x < (col - self.scol) * 8;
        let face = if pl { 3 } else { 4 };
        let mut k = 0;
        match code {
            7 => {
                k = self.spawn(2, 0x38, 7, x, y, if pl { 1 } else { 2 }, 0);
                if k != 0 {
                    self.s(k, VX, if pl { -0x11 } else { 0x11 });
                    self.s(k, AN, 0);
                }
            }
            8 => {
                let f = if self.rnd(2) == 0 { 4 } else { 3 };
                k = self.spawn(2, 0x38, 8, x, y, f, 0);
            }
            0xA | 0xB | 0xC | 0xD | 0xF => k = self.spawn(2, 0x38, code, x, y, face, 0),
            0xE => {
                k = self.spawn(2, 0x38, 0xE, x, y, 0, 0);
                if k != 0 {
                    self.s(k, PH, 7);
                    self.s(k, A44, y);
                    self.s(k, A46, x);
                    self.m.sw(G_CHAIN - 2, k);
                }
            }
            0x10 => {
                let k2 = self.spawn(2, 0x38, 0x10, x, y, face, 0);
                if k2 != 0 {
                    self.s(k2, HX, x);
                    self.s(k2, HY, y);
                    self.s(k2, FR, 5);
                    self.s(k2, VULN, 0);
                    self.s(k2, PH, 9);
                    let v = self.m.b(0x7671) as i16;
                    self.s(k2, SHOOT, v);
                    let v = self.g(k2, ST);
                    self.s(k2, PST, v);
                }
                return 0;
            }
            0x11 => {
                k = self.spawn(2, 0x38, 0x11, x, y, face, 0);
                if k != 0 {
                    self.s(k, PH, 5);
                }
            }
            0x13 => k = self.spawn(2, 0x39, 0x13, x, y, face, 0),
            0x14 => {
                if self.trec(0x13, 1) < 1 {
                    k = self.spawn(2, 0x38, 0x14, x, y, face, 0);
                    if k != 0 {
                        let v = self.g(k, ST);
                        self.s(k, PST, v);
                        let k2 = self.spawn(2, 0x38, 0x52, (col - 2) * 8, y - 0x800, face, 0);
                        if k2 != 0 {
                            self.s(k2, FR, 0);
                        }
                    }
                }
            }
            0x16 => k = self.spawn(2, 0x38, 0x16, x, y, 2, 0),
            0x41..=0x43 => {
                let py = self.m.w(G_ELEV - 8 * (code as i32 - 0x41));
                k = self.spawn(2, 0x35, code, x, py, 0, 0);
                if k != 0 {
                    self.s(k, PH, code - 0x41);
                }
            }
            0x53 => k = self.spawn(2, 0x38, 0x53, x, y, 0, 0),
            _ => {}
        }
        if k != 0 {
            self.s(k, HX, x);
            self.s(k, HY, row << 8);
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            return row << 8;
        }
        0
    }

    /// +6068 find_free_slot_in_table: remembers an item mark (0x40 or 0x60 marks) at column `col`, row `row`, so
    /// the player can collect it by walking into it (+60E6).
    pub fn item_mark(&mut self, mark: i16, col: i16, row: i16) {
        let mut free = 10;
        for i in (0..10).rev() {
            let e = G_ITEMS - 8 * i;
            let st = self.m.w(e);
            if st == 0 {
                free = i;
            } else if st == 1 && self.m.w(e - 4) == col && self.m.w(e - 6) == row + 1 {
                return;
            }
        }
        if free == 10 {
            return;
        }
        let e = G_ITEMS - 8 * free;
        self.m.sw(e, 1);
        self.m.sw(e - 2, mark);
        self.m.sw(e - 4, col);
        self.m.sw(e - 6, row + 1);
    }

    /// +60E6 collect_item: the player walked into item mark `mark`: the hidden item (or a boss, or the gate of
    /// the twin bosses) appears.
    pub fn collect_mark(&mut self, s: &mut Simon, mark: i16) {
        for i in 0..10 {
            let e = G_ITEMS - 8 * i;
            if self.m.w(e) != 1 || self.m.w(e - 2) != mark {
                continue;
            }
            self.m.sw(e, 2);
            let code = self.m.ub(G_ITEMTYPE - mark as i32 - 6 * self.room_van_ix()) as i16;
            if code == 0x13 {
                for k in 1..0x10 {
                    if self.g(k, TYPE) == 0x13 {
                        self.s(k, PH, 1);
                        self.boss = k;
                        self.boss_hp = self.g(k, HP);
                        self.boss_full = self.tyw(0x13, 0x26);
                        s.scroll_lock = 1;
                    }
                }
                return;
            }
            let kind = if self.ty(code, 0x0C) == 1 {
                self.remove_enemies();
                0x39
            } else {
                0x38
            };
            let col = self.m.w(e - 4);
            let row = self.m.w(e - 6);
            let face = if (col - self.scol) * 8 > s.x { 3 } else { 4 };
            let k = self.spawn(2, kind, code, col << 3, row << 8, face, 0);
            if k == 0 {
                continue;
            }
            self.s(k, ITEM, i as i16 + 1);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            if kind == 0x39 {
                self.boss = k;
                self.boss_hp = self.g(k, HP);
                self.boss_full = self.tyw(code, 0x26);
                s.scroll_lock = 1;
            }
            match code {
                0x11 => self.s(k, PH, 5),
                0x12 => self.flash = 1,
                0x14 => {
                    self.s(k, PH, 1);
                    let k2 = self.spawn(2, 0x38, 0x52, (col - 2) * 8, (row << 8) - 0x800, face, 0);
                    if k2 != 0 {
                        self.s(k2, FR, 0);
                        self.s(k2, PH, 1);
                    }
                }
                0x15 => self.s(k, A48, 5),
                0x1A => {
                    self.s(k, LIFE, 0);
                    self.s(k, PH, 10);
                    self.s(k, A48, 0x14);
                }
                _ => {}
            }
        }
    }

    /// +7AB8 clear_entity_flags: item `t` falls from (x, y) (a broken wall's item).
    pub fn drop_item(&mut self, t: i16, x: i16, y: i16) {
        let k = self.spawn(1, 0x1D, t, x, y, 0, 0);
        if k != 0 {
            self.s(k, FR, 0);
            self.s(k, FALL, 1);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, OWN, 0);
        }
    }

    /// +8324 map_tile_lookup: the fixed item of a candle at map column `col` of the room's screen file (0: none).
    pub fn fixed_item(&self, col: i16) -> i16 {
        match (self.van, col) {
            (0, 0x47) => 0x21,
            (0, 0xE) => 0x2C,
            (1, 0x8F) => 0x23,
            (1, 0x110) => 0x26,
            (3, 0x34) => 0x20,
            (3, 0x5C) => 0x2C,
            (4, 0x29) => 0x27,
            (4, 0x3F) => 0x22,
            (4, 0x89) => 0x2C,
            (5, 0xB) => 0x24,
            (7, 0x57) => 0x21,
            (8, 0x62) => 0x20,
            (9, 0x56) => 0x23,
            (10, 0x62) => 0x26,
            (0xB, 0x68) => 0x23,
            (0xC, 0x74) => 0x22,
            (0xD, 0x67) => 0x24,
            (0xF, 0x10F) => 0x21,
            (0x10, 0x28) => 0x20,
            (0x10, 0x89) => 0x23,
            (0x12, 0x12) => 0x21,
            (0x13, 0x41) => 0x23,
            (0x15, 5) => 0x22,
            (0x17, 0xE) => 0x24,
            (0x19, 0x96) => 0x20,
            (0x19, 0x119) => 0x22,
            (0x1A, 0x66) => 0x21,
            (0x1D, 0x53) => 0x24,
            (0x1D, 5) => 0x23,
            _ => 0,
        }
    }

    /// +7B14 select_enemy_sprite_and_reset_state: what a broken candle or a beaten enemy (object `src`) leaves at
    /// (x, y): the candle's fixed item, else the whip while it is not at its best, else a random item (a heart
    /// mostly, now and then something better). A sub-weapon the player already holds becomes a small heart.
    pub fn drop_from(&mut self, s: &Simon, x: i16, y: i16, src: i16) {
        let mut t = if self.class(self.g(src, TYPE)) == 2 {
            self.fixed_item(x >> 3)
        } else {
            0
        };
        if t == 0 {
            if s.whip_level < 2 {
                t = 0x28;
            } else {
                let r = self.rnd(6);
                t = self.m.w(0x7E02 - 2 * r as i32);
                if self.rnd(0x3C) == 1 {
                    let r = self.rnd(5);
                    t = self.m.w(0x7DF6 - 2 * r as i32);
                }
            }
        }
        let mut kind = 0x1D;
        if (0x20..=0x24).contains(&t) {
            kind = 0x33;
            if t == s.sub {
                kind = 0x1D;
                t = 0x25;
            }
        }
        let k = self.spawn(1, kind, t, x, y, 0, 0);
        if k != 0 {
            self.s(k, FR, 0);
            self.s(k, FALL, 1);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, PH, 1);
            self.s(k, OWN, 0);
        }
    }

    /// +7C1E set_entity_flags: a pop-up (score, sparks, wall pieces) of type `t` at (x, y).
    pub fn popup(&mut self, t: i16, x: i16, y: i16, face: i16, vd: i16) {
        let k = self.spawn(1, 0x34, t, x, y, face, vd);
        if k != 0 {
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, OWN, 1);
        }
    }

    /// +7C82 set_entity_flags_from_index: moving platform `n`'s object (type 0x44) at x, y.
    pub fn platform_obj(&mut self, x: i16, y: i16, n: i16) {
        let k = self.spawn(1, 0x36, 0x44, x, y, 0, 0);
        if k != 0 {
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, OWN, 1);
            self.s(k, PH, n);
        }
    }

    /// +82C2 clear_enemy_state: the axe comes back to the player as a weapon (type 0x36 at x, y).
    pub fn axe_back(&mut self, x: i16, y: i16) {
        let k = self.spawn(1, 0x33, 0x36, x, y, 0, 0);
        if k != 0 {
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, PH, 2);
            self.s(k, OWN, 1);
            self.s(k, HITS, self.g12ac);
        }
    }

    /// +7CE4 decrement_counter_and_update_struct: the player throws his sub-weapon (one heart).
    pub fn throw(&mut self, s: &mut Simon) {
        if s.hearts == 0 || self.trec(s.sub, 1) > s.shot {
            return;
        }
        let k = self.spawn(1, 0x33, s.sub, self.sx + s.x + 8, s.y - 0x200, s.facing, 0);
        if k != 0 {
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, PH, 2);
            self.s(k, OWN, 1);
            s.hearts -= 1;
        }
    }
}

// ---- enemy shots, the spawners, hidden treasures and the moving platforms
impl World {
    /// +7D78 process_entity_action: an enemy shot of kind `st` from (x, y): 0x4B (thrown, facing `face`), 0x4C, 0x64
    /// and 0x65 (aimed at the player at three speeds), 0x4D to 0x51 (per enemy).
    pub fn shot(&mut self, s: &Simon, st: i16, x: i16, y: i16, face: i16) -> i16 {
        let lr = if s.x < x.wrapping_sub(self.sx) { 3 } else { 4 };
        let mut k = 0;
        match st {
            0x4B => {
                k = self.spawn(2, 0x37, 0x4B, x, y, face, 0);
                if k != 0 {
                    self.s(k, FR, 0);
                }
            }
            0x4C | 0x64 | 0x65 => {
                let sp = match st {
                    0x65 => 0x80,
                    0x64 => 0x400,
                    _ => 0x200,
                };
                let f = if x.wrapping_sub(self.sx) > s.x { 3 } else { 4 };
                let vd = if y > s.y { 1 } else { 2 };
                k = self.spawn(2, 0x37, 0x4C, x, y, f, vd);
                if k != 0 {
                    self.s(k, FR, 0);
                    if !self.aim(s, k, x, y, s.x, s.y.wrapping_sub(sp)) {
                        self.remove(k);
                        self.s(k, ST, 0);
                    }
                }
            }
            0x4D => {
                k = self.spawn(2, 0x37, 0x4D, x, y, lr, 0);
                if k != 0 {
                    self.s(k, FR, 0);
                }
            }
            0x4E => {
                if self.trec(0x4E, 1) < 3 {
                    k = self.spawn(2, 0x37, 0x4E, x, y, lr, 0);
                    if k != 0 {
                        self.s(k, FR, 0);
                        self.s(k, OWN, 1);
                    }
                }
            }
            0x4F => {
                if self.trec(0x4F, 1) < 2 {
                    k = self.spawn(2, 0x37, 0x4F, x, y, face, 0);
                    if k != 0 {
                        self.s(k, FR, 0);
                        self.s(k, OWN, 1);
                    }
                }
            }
            0x50 => {
                if self.trec(0x50, 1) < 2 {
                    k = self.spawn(2, 0x37, 0x50, x, y, face, 0);
                    if k != 0 {
                        self.s(k, FR, 0);
                        self.s(k, A46, 0);
                        let v = self.oty(k, 0x25) as i16;
                        self.s(k, A48, v);
                        let d = self.speed(k, face).wrapping_mul(3);
                        self.add(k, X, d);
                    }
                }
            }
            0x51 => {
                k = self.spawn(2, 0x37, 0x51, x, y, 0, 0);
                if k != 0 {
                    self.s(k, FR, 0);
                    self.s(k, PH, 5);
                    self.s(k, SHOOT, 0x10);
                }
            }
            _ => {}
        }
        if k != 0 {
            let v = self.g(k, ST);
            self.s(k, PST, v);
        }
        k
    }

    /// +8110 update_enemy_movement_state: aims object `k` at (px, py) from (x, y): the direction, the step along
    /// the major axis and the speed along the minor one (A44 the major direction, A46 the minor speed, A48 the
    /// major speed). False when it is already there.
    pub fn aim(&mut self, s: &Simon, k: i16, x: i16, y: i16, px: i16, py: i16) -> bool {
        let dx = x.wrapping_sub(self.sx).wrapping_sub(px);
        let dy = (if s.crouch != 0 { 0x100i16 } else { 0 }).wrapping_add(y.wrapping_sub(py));
        let cx = (dx as i32).abs() >> 3;
        let cy = (dy as i32).abs() >> 8;
        if cx == 0 && cy == 0 {
            self.s(k, A44, 3);
            self.s(k, A46, 0);
            self.s(k, A48, 0);
            return false;
        }
        self.s(k, AN, if dx >= 0 { 3 } else { 4 });
        self.s(k, VD, if dy >= 0 { 1 } else { 2 });
        let ax = (dx as i32).abs();
        let ay = (dy as i32).abs();
        if cx >= cy {
            let d6 = (self.oty_w(k, 8) as i32).abs().max(1);
            let mut d7 = (ax << 4) / d6;
            if d7 as i16 == 0 {
                d7 = d6 << 4;
            }
            let d7w = (d7 as i16).max(1) as i32;
            let a2 = if cy == 0 { 0 } else { (ay << 4) / d7w };
            self.s(k, A48, (d7 as i16) >> 4);
            if dy >= 0 {
                self.s(k, A44, 1);
                self.s(k, A46, -(a2 as i16));
            } else {
                self.s(k, A44, 2);
                self.s(k, A46, a2 as i16);
            }
        } else {
            let d6 = (self.oty_w(k, 4) as i32).abs().max(1);
            let a2 = ay / d6;
            let d7 = (self.oty_w(k, 8) as i32).abs() as i16;
            self.s(k, A48, a2 as i16);
            if dx >= 0 {
                self.s(k, A44, 3);
                self.s(k, A46, -d7);
            } else {
                self.s(k, A44, 4);
                self.s(k, A46, d7);
            }
        }
        true
    }

    fn oty_w(&self, k: i16, n: i32) -> i16 {
        self.tyw(self.g(k, TYPE), n)
    }

    /// The left or right edge column an enemy walks in from (+8538): 2 columns off the screen.
    fn edge_x(&self, face: i16) -> i16 {
        (self.scol + if face == 3 { 0x2A } else { 0 } - 2) * 8
    }

    /// +8538 process_enemy_spawners: the six spawners a room's marks turn on (zombies, bats, fishmen, zombies
    /// from the low passage, eagles, and the skeleton riding a bird), each with its count limit and its timer.
    pub fn spawners(&mut self, s: &Simon) {
        if self.m.w(G_LIVE) >= 0x11 || s.stopwatch != 0 {
            return;
        }
        if self.trec(1, 0) != 0 && self.trec(1, 1) < 3 && self.trec(1, 2) == 0 {
            let f = if self.rnd(10) < 6 { 4 } else { 3 };
            self.spawn(2, 1, 1, self.edge_x(f), 0x1700, f, 0);
            self.set_trec(1, 2, self.trec(1, 3));
        }
        if self.trec(2, 0) != 0 && self.trec(2, 1) < 1 && self.trec(2, 2) == 0 {
            let f = if s.facing & 1 == 0 { 3 } else { 4 };
            self.rnd(4);
            self.spawn(2, 2, 2, self.edge_x(f), s.y.wrapping_sub(0x400), f, 0);
            self.set_trec(2, 2, self.trec(2, 3));
        }
        if self.trec(3, 0) != 0 && self.trec(3, 1) < 2 && self.trec(3, 2) == 0 {
            let f = if self.rnd(10) < 6 { 4 } else { 3 };
            let r = self.rnd(0x80);
            let x = (s.x as i32 + self.sx as i32 + r as i32 - 0x40) as i16;
            if x < 1 || x as i32 >= (self.mw as i32) << 3 {
                self.set_trec(3, 2, self.trec(3, 3));
            } else {
                let k = self.spawn(2, 3, 3, x, 0x1600, f, 0);
                if k != 0 {
                    self.fx(0x12);
                    self.s(k, FR, 0);
                    self.popup(0x3E, x, 0x1600, 4, 0);
                    self.popup(0x3F, x, 0x1700, 4, 0);
                    self.popup(0x40, x, 0x1600, 4, 0);
                    self.set_trec(3, 2, self.trec(3, 3));
                }
            }
        }
        if self.trec(4, 0) != 0 && self.trec(4, 1) < 3 && self.trec(4, 2) == 0 {
            let cell = self.at(self.mw as i32 * 0xF + self.scol as i32 + 0x27);
            let (f, row) = if cell == 1 && self.rnd(2) != 0 {
                (3, 0xF)
            } else {
                (4, 0x17)
            };
            self.spawn(2, 1, 4, self.edge_x(f), row << 8, f, 0);
            self.set_trec(4, 2, self.trec(4, 3));
        }
        if self.trec(5, 0) != 0 && self.trec(5, 1) < 1 && self.trec(5, 2) == 0 {
            let f = if s.facing & 1 == 0 { 3 } else { 4 };
            self.rnd(2);
            self.spawn(2, 6, 5, self.edge_x(f), s.y.wrapping_sub(0x280), f, 0);
            self.set_trec(5, 2, self.trec(5, 3));
        }
        if self.trec(6, 0) != 0 && self.trec(0x17, 1) < 3 && self.trec(6, 2) == 0 {
            let f = if self.rnd(2) == 0 { 4 } else { 3 };
            let x = self.edge_x(f);
            let h = (self.rnd(3) * -3 + 0xF) * 0x100;
            self.spawn(2, 0xC, 6, x, h, f, 0);
            self.set_trec(6, 2, self.trec(6, 3));
            let k = self.spawn(2, 9, 0x17, x, h + 0x300, f, 0);
            if k != 0 {
                self.s(k, VX, if f == 3 { -8 } else { 8 });
                self.s(k, FR, 0);
            }
        }
    }

    /// The spawn timers' tick (+1480, every frame).
    pub fn spawn_timers(&mut self) {
        for t in 1..=6 {
            let v = self.trec(t, 2);
            if v != 0 {
                self.set_trec(t, 2, v - 1);
            }
        }
        let v = self.trec(0xB, 2);
        if v != 0 {
            self.set_trec(0xB, 2, v - 1);
        }
    }

    /// +A4D0 set_spawn_flags_by_position: the hidden treasures: standing still (or crouching) at the right place
    /// makes one rise out of the floor, once a life.
    pub fn secrets(&mut self, s: &Simon) {
        let c = ((self.sx.wrapping_add(s.x)) as u16 >> 3) as i32;
        let r = s.y >> 8;
        let still = s.still;
        let crouch = s.crouch != 0;
        let inr = |a: i32, b: i32| c > a && c < b;
        let n = match self.van {
            0 => {
                if (100..=0x66).contains(&c) && r == 0x17 {
                    1
                } else if inr(0x2C, 0x30) && r == 0x17 && still > 0x20 {
                    2
                } else {
                    0
                }
            }
            1 if inr(0x92, 0x96) && r == 0xB && still > 0x20 => 3,
            3 if inr(0x55, 0x59) && r == 0x15 && crouch => 4,
            5 if inr(0x23, 0x27) && r == 0xF => 5,
            7 if inr(0x36, 0x3A) && r == 7 && still > 0x20 => 6,
            8 if inr(0x53, 0x57) && r == 9 => 7,
            9 if inr(0x53, 0x57) && r == 0x13 && still > 0x20 => 8,
            10 if inr(0x58, 0x5C) && r == 0x11 && still > 0x20 => 9,
            0xC => {
                if (0x19..=0x1B).contains(&c) && r == 0xD && crouch {
                    10
                } else if inr(0x60, 100) && r == 0x17 && crouch {
                    0xB
                } else {
                    0
                }
            }
            0xD if inr(0x82, 0x86) && r == 0x11 && crouch => 0xC,
            0xE if inr(0x33, 0x37) && r == 0x11 && crouch => 0xD,
            0x16 | 0x17 => {
                let s5 = if self.van == 0x16 && inr(0x6A, 0x71) && r == 0xD && crouch {
                    0xF
                } else {
                    0
                };
                if (0xC0..=0xC2).contains(&c) && r == 0x15 && still >= 0x21 {
                    0xE
                } else if inr(0x3B, 0x3F) && r == 0x15 && still > 0x20 {
                    0x10
                } else {
                    s5
                }
            }
            0x1A if inr(0x44, 0x48) && r == 0xD && crouch => 0x11,
            _ => 0,
        };
        if n == 0 {
            return;
        }
        let e = G_SECRETS - 10 * n;
        if self.m.w(e) != 0 {
            return;
        }
        self.m.sw(e, 1);
        let t = self.m.w(e - 2);
        let col = self.m.w(e - 4);
        let row = self.m.w(e - 6);
        let k = self.spawn(1, 0x1D, t, col << 3, (row + 2) * 0x100, 0, 0);
        if k != 0 {
            self.s(k, A46, (row - 2) * 8);
            self.s(k, A48, row << 3);
            self.s(k, FR, 0);
            let v = self.g(k, ST);
            self.s(k, PST, v);
            self.s(k, PH, 1);
            self.s(k, OWN, 0);
            self.fx(8);
        }
    }

    /// +33C2 process_enemy_hitbox_table: moving platforms `a` to `b` (screen files 6, 8 and 16) slide between
    /// their two ends, moving their solid cells in the attribute map; one carries the player when he stands on it.
    /// Returns the direction the player was carried (3 left, 4 right; 0 none).
    pub fn platforms(&mut self, s: &Simon, a: i32, b: i32) -> i16 {
        let mut carried = 0;
        for n in a..=b {
            let e = G_PLAT - 10 * n;
            let lo = self.m.w(e);
            let row = self.m.w(e - 2);
            let hi = self.m.w(e - 4);
            let mut x = self.m.w(e - 8);
            let mut dir = self.m.b(e - 6);
            let mut tick = self.m.b(e - 7);
            if x <= lo {
                tick = tick.wrapping_add(1);
                if tick > 3 {
                    tick = 0;
                    dir = 4;
                    x = lo;
                }
            }
            if hi <= x {
                tick = tick.wrapping_add(1);
                if tick > 3 {
                    tick = 0;
                    dir = 3;
                    x = hi;
                }
            }
            let pxm = self.sx.wrapping_add(s.x);
            let on = s.g[4][1] == 8;
            if tick == 0 && dir == 3 {
                x -= 8;
                let i = (row >> 8) as i32 * self.mw as i32 + (x >> 3) as i32;
                self.set_at(i - 2, 8);
                self.set_at(i + 4, 0);
                if on && x - 0x28 <= pxm && pxm <= x + 0x18 {
                    carried = 3;
                }
            } else if tick == 0 && dir == 4 {
                x += 8;
                let i = (row >> 8) as i32 * self.mw as i32 + (x >> 3) as i32;
                self.set_at(i + 3, 8);
                self.set_at(i - 3, 0);
                if on && x - 0x28 <= pxm && pxm <= x + 0x28 {
                    carried = 4;
                }
            }
            self.m.sw(e - 8, x);
            self.m.sb(e - 6, dir);
            self.m.sb(e - 7, tick);
        }
        carried
    }

    /// +20EC's platform part: the platforms of screen files 6, 8 and 16 start at their ends, with their objects.
    pub fn start_platforms(&mut self) {
        let (a, b) = match self.van {
            6 => (0, 0),
            8 => (1, 2),
            0x10 => (3, 6),
            _ => return,
        };
        for n in a..=b {
            let e = G_PLAT - 10 * n;
            let x = if self.m.b(e - 6) == 3 {
                self.m.w(e - 4)
            } else {
                self.m.w(e)
            };
            self.m.sw(e - 8, x);
            self.m.sb(e - 7, 0);
            let lo = self.m.w(e);
            let y = self.m.w(e - 2).wrapping_add(0x100);
            self.platform_obj(lo, y, n as i16);
        }
    }
}

/// The labels of +4528's flow (the original jumps between its parts).
#[derive(Clone, Copy, PartialEq, Eq)]
enum L {
    Main,
    Walk,
    Turn,
    Shoot,
    Move,
    Anim,
    Apply,
    Edge,
}

// ---- moving the objects
impl World {
    /// The word at `n` of object `k`'s direction-indexed flags: free ahead (n = 0xF, set by +3F32 for directions 3
    /// and 4) and solid ahead (n = 0x14).
    fn dflag(&self, k: i16, n: i32) -> i16 {
        let d = self.g(k, AN) as i32;
        self.g(k, 2 * (d + n))
    }

    /// +3F32 set_tile_flags_from_map: what is around object `k` in the attribute map: nothing under it, solid on
    /// its left and right edges (the type's left and right speeds give the edges), and free above them.
    pub fn map_flags(&mut self, k: i16) {
        let t = self.g(k, TYPE);
        let x = self.g(k, X);
        let row = self.g(k, Y) >> 8;
        let base = row as i32 * self.mw as i32 + 1;
        let c5 = base + (x.wrapping_add(self.tyw(t, 8)) >> 3) as i32;
        let c6 = base + (x.wrapping_add(self.tyw(t, 6)) >> 3) as i32;
        let b7 = if row >= 0 {
            self.at(base + (x >> 3) as i32) as u8
        } else {
            0
        };
        self.s(k, NOFLOOR, (b7 & 0xF == 0) as i16);
        self.s(k, 0x30, (self.at(c5) >= 1) as i16);
        self.s(k, 0x2E, (self.at(c6) >= 1) as i16);
        let w = self.mw as i32;
        self.s(k, 0x26, (self.at(c5 - w) != 1) as i16);
        self.s(k, 0x24, (self.at(c6 - w) != 1) as i16);
    }

    /// +407C update_enemy_animation_frame: object `k` falls up to two rows a frame (drifting by its x speed when
    /// its type says so) until something solid is under it; off the screen's edge it bounces back.
    fn fall(&mut self, k: i16) {
        self.add(k, FALL, 1);
        if self.g(k, FALL) == 2 {
            let v = self.g(k, VX) >> 1;
            self.s(k, VX, v);
        }
        let drift = self.oty_w(k, 0x2A) != 0;
        if self.oty(k, 0x24) == 0 || self.g(k, OWN) != 1 {
            let y = self.g(k, Y);
            let mut r = (y >> 8) as i32;
            if y & 0xFF != 0 {
                r += 1;
            }
            let p = r * self.mw as i32 + (self.g(k, X) >> 3) as i32 + 1;
            let b1 = self.at(p);
            let b2 = self.at(p + self.mw as i32);
            let carry = drift && (self.g(k, MODE) == 2 || self.g(k, OWN) == 1);
            let mut landed = true;
            if b1 & 1 == 0 || b1 < 0 {
                self.add(k, Y, 0x100);
                if carry {
                    let v = self.g(k, VX);
                    self.add(k, X, v);
                }
                if b2 & 1 == 0 || b2 < 0 {
                    self.add(k, Y, 0x100);
                    if carry {
                        let v = self.g(k, VX);
                        self.add(k, X, v);
                    }
                    landed = false;
                }
            }
            if landed {
                self.s(k, FALL, 0);
                let y = self.g(k, Y);
                if y & 0xFF != 0 {
                    self.s(k, Y, (y.wrapping_add(0x100)) & !0xFF);
                }
                if self.g(k, TYPE) == 0x23 && self.g(k, OWN) == 1 {
                    self.g12ae = 1;
                    self.g12ac = self.g(k, HITS);
                    self.fx(0x13);
                } else {
                    self.g12ae = 0;
                }
                return;
            }
        } else {
            self.add(k, Y, 0x200);
            if drift {
                let v = self.g(k, VX).wrapping_mul(2);
                self.add(k, X, v);
            }
        }
        if self.oty(k, 0x20) == 0 {
            let sxp = self.g(k, X).wrapping_sub(self.sx);
            if !(0..=0x138).contains(&sxp) {
                let v = self.g(k, VX).wrapping_mul(2);
                self.add(k, X, -v);
                let v = self.g(k, VX);
                self.s(k, VX, -v);
            }
        }
    }

    /// +5FE8 find_tile_property_in_map: the first platform (8) or solid (1) cell under column x from row y down
    /// to y + dy; the rows to it in -$12AA.
    fn find_floor(&mut self, x: i16, y: i16, dy: i16) -> i16 {
        self.g12aa = 0;
        let mut r = (y >> 8) - 1;
        let mut p = r as i32 * self.mw as i32 + (x >> 3) as i32 + 1;
        loop {
            r += 1;
            if (dy.wrapping_add(y) >> 8) <= r {
                return 0;
            }
            p += self.mw as i32;
            let c = self.at(p);
            if c == 8 || c == 1 {
                self.g12aa = r - (y >> 8);
                return c as i16;
            }
        }
    }

    /// +42B2 update_enemy_state: a step of object `k`'s jump (table `mode`: the type's bytes +0x15, +0x16 or
    /// +0x1F choose a row of -$3FDC); landing on a platform or the floor ends it. Returns 1 while jumping.
    fn jump(&mut self, k: i16, mode: i16) -> i16 {
        let x0 = self.g(k, X);
        let y0 = self.g(k, Y);
        if self.dflag(k, 0xF) == 0 && self.oty(k, 0xD) == 0 {
            self.s(k, JMP, 0);
            self.s(k, STEP, 0);
            self.g12ae = (self.g(k, TYPE) == 0x23) as i16;
            return 0;
        }
        let tb = match mode {
            1 => self.oty(k, 0x15),
            2 => self.oty(k, 0x16),
            3 => self.oty(k, 0x1F),
            _ => 0,
        } as i32;
        let v = self.m.w(0x3FDC - tb * 60 - 2 * self.g(k, STEP) as i32);
        if v == 1000 {
            self.s(k, JMP, 0);
            self.s(k, STEP, 0);
            return 0;
        }
        if v < 1 {
            self.add(k, Y, v);
            self.add(k, STEP, 1);
        } else {
            let f = self.find_floor(x0, y0, v);
            if f == 1 && self.oty(k, 0x24) == 0 {
                if self.g12aa != 0 {
                    self.add(k, Y, self.g12aa.wrapping_mul(0x100));
                }
                let y = self.g(k, Y) & !0xFF;
                self.s(k, Y, y);
                self.s(k, STEP, 0);
                self.s(k, VD, 0);
                self.s(k, JMP, 0);
                let p = ((y0 >> 8) as i32 - 1) * self.mw as i32 + (x0 >> 3) as i32 - 1;
                if (self.at(p + 2) | self.at(p + 1) | self.at(p)) & 1 != 0 {
                    self.add(k, Y, -0x100);
                }
                return 0;
            }
            self.add(k, Y, v);
            self.add(k, STEP, 1);
        }
        self.s(k, JMP, 1);
        1
    }

    /// +5E7E update_enemy_state_table: the lifts of screen file 9, the spawners, then every object's frame.
    pub fn update(&mut self, s: &mut Simon) {
        if self.van == 9 {
            for n in 0..3 {
                let e = G_ELEV - 8 * n;
                let y = self.m.w(e);
                let lo = self.m.w(e - 2);
                let hi = self.m.w(e - 4);
                if lo < y {
                    if hi <= y {
                        self.m.sw(e - 6, 1);
                        self.m.sw(e, hi);
                    }
                } else {
                    self.m.sw(e - 6, 2);
                    self.m.sw(e, lo);
                }
                let d = if self.m.w(e - 6) == 1 { -0x80 } else { 0x100 };
                let y = self.m.w(e).wrapping_add(d);
                self.m.sw(e, y);
            }
        }
        if self.m.w(G_LIVE) < 0x10 {
            self.spawners(s);
        }
        for k in 1..0x10 {
            let st = self.g(k, ST);
            if st != 0 && st != 4 {
                self.map_flags(k);
                self.step(s, k);
                if self.g(k, ST) == 8 {
                    self.s(k, ST, 2);
                }
            }
        }
        if self.g12b0 != 0 {
            self.g12b0 = 0;
            self.s(self.g12b2, A48, 0);
            self.s(self.g12b4, A48, 0);
        }
    }
}

impl World {
    fn flip(d: i16) -> i16 {
        if d == 3 {
            4
        } else {
            3
        }
    }

    /// +4528 update_enemy_animation_frame_2: one frame of object `k`. Most types are driven by their type table
    /// (waiting for the player, jumping, falling, walking and turning at walls and ledges, keeping a distance,
    /// shooting, flying up and down or on a wave); the bosses and a few others have their own code.
    pub fn step(&mut self, s: &mut Simon, k: i16) {
        let px = s.x as u16;
        let py = s.y as u16;
        let t = self.g(k, TYPE);
        let ty = |w: &World, n: i32| w.ty(t, n);
        let x0 = self.g(k, X);
        let dcol = ((px as i16).wrapping_sub(x0.wrapping_sub(self.sx)) as i32).abs() as i16 >> 3;
        let drow = ((py as i16).wrapping_sub(self.g(k, Y)) as i32).abs() as i16 >> 8;
        let lr = |w: &World, x: i16| {
            if px < x.wrapping_sub(w.sx) as u16 {
                3
            } else {
                4
            }
        };
        let mut dx: i16 = 0;
        let mut dy: i16 = 0;
        let mut l;
        if s.stopwatch != 0 && self.g(k, MODE) == 2 && (self.block < 2 || s.scroll_lock == 0) {
            if self.g(k, FR) < 0 {
                self.s(k, FR, 0);
            }
            l = L::Apply;
        } else {
            if self.g(k, LIFE) != 0 {
                self.add(k, LIFE, -1);
                if self.g(k, LIFE) == 0 {
                    self.remove(k);
                    return;
                }
            }
            if self.g(k, STUN) != 0 {
                self.add(k, STUN, -1);
                if self.g(k, STUN) != 0 {
                    self.s(k, VULN, 0);
                    return;
                }
                self.s(k, VULN, 1);
            }
            l = if ty(self, 0x1A) == 0 {
                L::Main
            } else {
                self.special(s, k, &mut dx, &mut dy, dcol, drow)
            };
        }
        loop {
            match l {
                L::Main => {
                    let w = self.g(k, WAIT);
                    if w != 0 {
                        let c1c = ty(self, 0x1C) as i16;
                        if w <= dcol || (c1c != 0 && c1c <= drow) {
                            l = L::Apply;
                            continue;
                        }
                        self.s(k, WAIT, 0);
                        let f = lr(self, self.g(k, X));
                        self.s(k, FC, f);
                        self.s(k, AN, f);
                    }
                    let jumping = self.g(k, FALL) == 0
                        && (self.g(k, JMP) != 0
                            || (ty(self, 0x15) != 0
                                && self.dflag(k, 0x14) == 0
                                && self.dflag(k, 0xF) != 0
                                && ty(self, 0x1F) == 0)
                            || ty(self, 0x16) != 0
                            || (ty(self, 0x1F) != 0 && self.g(k, ST) == 8));
                    if !jumping {
                        l = L::Walk;
                        continue;
                    }
                    let r = if t == 0xB || t == 0x17 {
                        if self.g(k, JMP) == 0 {
                            let xs = self.g(k, X).wrapping_sub(self.sx) as u16;
                            let near = dcol < 6
                                && drow < 6
                                && ((s.facing == 4 && px < xs && self.g(k, FC) == 3)
                                    || (s.facing == 3 && xs < px && self.g(k, FC) == 4));
                            self.s(k, PH, if near { 1 } else { 2 });
                        }
                        let m = self.g(k, PH);
                        self.jump(k, m)
                    } else {
                        self.jump(k, 1)
                    };
                    if r == 0 {
                        l = if self.g12ae != 0 { L::Edge } else { L::Walk };
                        continue;
                    }
                    if ty(self, 0x1E) == 0 {
                        dx = if t == 0xB || t == 0x17 || t == 0x52 {
                            if self.g(k, AN) == 3 {
                                -7
                            } else {
                                7
                            }
                        } else {
                            0
                        };
                    } else {
                        dx = self.speed(k, self.g(k, AN));
                    }
                    let x = self.g(k, X);
                    if ty(self, 0x20) == 0
                        && (dx.wrapping_add(x).wrapping_sub(self.sx) < 0
                            || dx.wrapping_add(x).wrapping_sub(self.scol.wrapping_mul(8)) > 0x130)
                    {
                        dx = -dx;
                        let f = if self.g(k, AN) == 4 { 3 } else { 4 };
                        self.s(k, FC, f);
                        self.s(k, AN, f);
                    }
                    l = if ty(self, 0x23) == 0 {
                        L::Apply
                    } else {
                        L::Anim
                    };
                }
                L::Walk => {
                    if (ty(self, 0x18) != 0 && self.g(k, NOFLOOR) != 0)
                        || self.g(k, FALL) != 0
                        || t == 0x23
                        || ty(self, 0x24) != 0
                    {
                        if self.g(k, FALL) == 0 {
                            let v = self.speed(k, self.g(k, AN));
                            self.s(k, VX, v);
                        }
                        self.fall(k);
                        l = if ty(self, 0x23) == 0 {
                            L::Apply
                        } else {
                            L::Anim
                        };
                        continue;
                    }
                    let x = self.g(k, X);
                    let y = self.g(k, Y);
                    if t == 3 || t == 7 || t == 8 {
                        if (y as u16 == py && y != self.g(k, SEEN_PY)) || y != self.g(k, SEEN_Y) {
                            let f = lr(self, x);
                            self.s(k, FC, f);
                            self.s(k, AN, f);
                        }
                        self.s(k, SEEN_PY, py as i16);
                        self.s(k, SEEN_Y, y);
                    }
                    if ty(self, 0xE) != 0 {
                        let c = ty(self, 0x1D) as i16;
                        if c == 0 || c < dcol {
                            let f = lr(self, x);
                            self.s(k, FC, f);
                        }
                    }
                    let cf = ty(self, 0xF) as u8;
                    if cf & 1 == 0 || (self.dflag(k, 0x14) != 0 && self.dflag(k, 0xF) != 0) {
                        if cf & 0xFE != 0 && self.rnd(cf as i8 as i16) == 1 && self.g(k, SHOOT) == 0
                        {
                            let f = World::flip(self.g(k, AN));
                            self.s(k, AN, f);
                        }
                        let mut turn = true;
                        if ty(self, 0x20) == 0 {
                            if self.sx.wrapping_add(0x130) < x {
                                self.s(k, AN, 3);
                                self.s(k, FC, 3);
                                self.s(k, HITBY, -1);
                                turn = false;
                            } else if self.scol + 1 > (x >> 3) {
                                self.s(k, AN, 4);
                                self.s(k, FC, 4);
                                self.s(k, HITBY, -1);
                                turn = false;
                            }
                        }
                        if turn {
                            l = L::Turn;
                            continue;
                        }
                    } else {
                        let f = World::flip(self.g(k, AN));
                        self.s(k, AN, f);
                        if ty(self, 0xE) == 0 {
                            self.s(k, FC, f);
                        }
                    }
                    l = L::Shoot;
                }
                L::Turn => {
                    let c19 = ty(self, 0x19) as i16;
                    if c19 == 0 || 3 < drow {
                        if t != 0x14 {
                            let f = self.g(k, FC);
                            self.s(k, AN, f);
                        }
                    } else if dcol < c19 {
                        let f = World::flip(self.g(k, FC));
                        self.s(k, AN, f);
                        if self.dflag(k, 0xF) == 0 {
                            let f = World::flip(self.g(k, AN));
                            self.s(k, AN, f);
                        }
                    } else if c19 + 4 < dcol {
                        let f = self.g(k, FC);
                        self.s(k, AN, f);
                    }
                    l = L::Shoot;
                }
                L::Shoot => {
                    self.shooting(s, k);
                    l = L::Move;
                }
                L::Move => {
                    if self.g(k, SHOOT) == 0
                        && (self.dflag(k, 0xF) != 0
                            || ty(self, 0xD) != 0
                            || self.g(k, JMP) != 0
                            || ty(self, 0x10) != 0)
                    {
                        dx = self.speed(k, self.g(k, AN));
                    }
                    let c17 = ty(self, 0x17) as i32;
                    if c17 == 0 {
                        let c1b = ty(self, 0x1B) as i16;
                        let y = self.g(k, Y) as u16;
                        let target = py.wrapping_sub(0x200);
                        if c1b != 0 && y != target {
                            dy = if y < target { c1b } else { -c1b };
                        }
                    } else {
                        let mut v = self.m.w(0x41BC - c17 * 80 - 2 * self.g(k, STEP2) as i32);
                        if v == 1000 {
                            self.s(k, STEP2, 1);
                            v = self.m.w(0x41BC - c17 * 80);
                        } else {
                            self.add(k, STEP2, 1);
                        }
                        dy = v;
                    }
                    l = L::Anim;
                }
                L::Anim => {
                    self.add(k, FR, 1);
                    l = L::Apply;
                }
                L::Apply => {
                    if self.g(k, FR) < 0 {
                        self.s(k, FR, 0);
                    }
                    if self.shape_ix(k) == -1 {
                        self.s(k, FR, 0);
                    }
                    self.add(k, X, dx);
                    self.add(k, Y, dy);
                    l = L::Edge;
                }
                L::Edge => {
                    self.edges(k);
                    return;
                }
            }
        }
    }

    /// The end of +4528 (l0_5CEA): objects far off the screen go (a fishman falling back into the water
    /// splashes), the axe that landed comes back, and a hit object is alive again.
    fn edges(&mut self, k: i16) {
        let t = self.g(k, TYPE);
        let x = self.g(k, X);
        let y = self.g(k, Y);
        if (self.sx.wrapping_add(0x178) < x
            || x < self.sx.wrapping_sub(0x40)
            || (0x1800 < y && t != 5))
            && self.g(k, KIND) != 0x36
            && t != 0x19
            && t != 0x1A
            && t != 0x1B
            && self.class(t) != 6
        {
            if t == 0xE || t == 0x18 {
                self.remove_enemies();
            } else {
                if t == 3 && 0x1800 < y {
                    self.fx(0x12);
                    self.remove(k);
                    return;
                }
                self.remove(k);
            }
        }
        if t == 3 && 0x15FF < y {
            self.popup(0x3E, x, y, 4, 0);
            self.popup(0x3F, x, y.wrapping_add(0x100), 4, 0);
            self.popup(0x40, x, y, 4, 0);
        }
        if self.g12ae != 0 {
            self.remove(k);
            self.axe_back(x, y);
            self.g12ae = 0;
            self.g12ac = 0;
        }
        if self.g(k, ST) == 2 {
            self.s(k, ST, 1);
        }
    }

    /// The shooting part of +4528 (l0_5784): types with a shot (type table +0x11) fire every so often (+0x13
    /// frames of shooting, +0x25 frames between), each in its own way.
    fn shooting(&mut self, s: &mut Simon, k: i16) {
        let t = self.g(k, TYPE);
        if self.ty(t, 0x11) == 0 {
            return;
        }
        let x = self.g(k, X);
        let y = self.g(k, Y);
        let px = s.x as u16;
        let c13 = self.ty(t, 0x13) as i16;
        if c13 != 0 {
            if self.g(k, CD) == 0 {
                let xs = x.wrapping_sub(self.sx) as u16;
                if xs <= px || self.g(k, FC) != 4 || t == 0x11 || t == 0x12 {
                    if self.g(k, SHOOT) == 0 {
                        self.s(k, SHOOT, c13);
                        match t {
                            3 => {
                                let f = self.g(k, FC);
                                self.shot(s, 0x4B, x, y.wrapping_sub(0x2A0), f);
                            }
                            0xC => {
                                self.shot(s, 0x4E, x, y.wrapping_sub(0x200), 0);
                            }
                            0xE => {
                                if self.trec(0x4C, 1) < 1 && xs > px {
                                    self.shot(s, 0x4C, x, y.wrapping_sub(0x80), 0);
                                }
                            }
                            0x12 => {
                                self.shot(s, 0x4D, x, y.wrapping_sub(0x200), 0);
                            }
                            _ => {}
                        }
                    } else {
                        self.add(k, SHOOT, -1);
                        let fr = match t {
                            3 => 6,
                            10 | 0xC | 0xE => 1,
                            0x13 => 3,
                            _ => -1,
                        };
                        self.s(k, FR, fr);
                        let n = c13 - self.g(k, SHOOT);
                        if t == 0xC && (n == 3 || n == 6) && self.rnd(2) == 1 {
                            self.shot(s, 0x4E, x, y.wrapping_sub(0x200), 0);
                        }
                        if t == 10 && (n == 7 || n == 0x10) {
                            let f = self.g(k, FC);
                            self.shot(s, 0x4B, x, y.wrapping_sub(0x200), f);
                        }
                        if t == 0x13 && n == 3 {
                            let f = self.g(k, FC);
                            self.shot(s, 0x4F, x, y.wrapping_sub(0x300), f);
                        }
                        if self.g(k, SHOOT) == 0 {
                            let v = self.ty(t, 0x25) as i16;
                            self.s(k, CD, v);
                            self.s(k, FR, -1);
                        }
                    }
                }
            } else {
                self.add(k, CD, -1);
            }
        }
        if t == 0x52 && self.trec(0x4C, 1) < 1 {
            self.shot(s, 0x4C, x, y.wrapping_sub(0x100), 0);
        }
        if t == 0xF && self.rnd(0x19) == 1 {
            let f = self.g(k, FC);
            self.shot(s, 0x50, x, y.wrapping_sub(0x200), f);
        }
        if t == 0x15 && self.trec(0x51, 1) < 4 {
            if self.g(k, A48) == 0 {
                let v = self.ty(t, 0x25) as i16;
                self.s(k, A48, v);
            } else {
                self.add(k, A48, -1);
                if self.g(k, A48) == 0 {
                    self.shot(s, 0x51, x, y, 0);
                }
            }
        }
    }
}

impl World {
    /// Chain segment `i` of type 0x0E (-$12A8 words; segment 1 is the head).
    fn chain(&self, i: i32) -> i16 {
        self.m.w(G_CHAIN - 2 * i)
    }

    /// The types with their own code in +4528 (type table +0x1A set): returns where the common code goes on.
    #[allow(clippy::too_many_arguments)]
    fn special(
        &mut self,
        s: &mut Simon,
        k: i16,
        dx: &mut i16,
        dy: &mut i16,
        dcol: i16,
        drow: i16,
    ) -> L {
        let t = self.g(k, TYPE);
        let px = s.x as u16;
        let py = s.y as u16;
        let x = self.g(k, X);
        let y = self.g(k, Y);
        let lr = |w: &World, x: i16| {
            if px < x.wrapping_sub(w.sx) as u16 {
                3
            } else {
                4
            }
        };
        if self.class(t) == 6 {
            if self.g(k, A48) << 5 < y {
                *dy = -0x20;
            }
            return L::Anim;
        }
        match t {
            0xE => {
                if self.g(k, PH) == 7 {
                    if self.g(k, A48) < 7 {
                        self.add(k, A48, 1);
                        let n = self.g(k, A48);
                        if n < 7 {
                            let k2 =
                                self.spawn(2, 0x15, 0x18, self.g(k, A46), self.g(k, A44), 0, 1);
                            self.s(k2, FR, 0);
                            self.m.sw(G_CHAIN - 2 * (n as i32 + 1), k2);
                        }
                        for i in 1..n.max(1) {
                            let c = self.chain(i as i32);
                            self.add(c, X, -0xC);
                        }
                    } else {
                        self.s(k, PH, 8);
                    }
                }
                if self.g(k, PH) != 8 {
                    return L::Anim;
                }
                if self.rnd(0x1E) == 1 {
                    let v = if self.g(k, VD) == 1 { 2 } else { 1 };
                    self.s(k, VD, v);
                }
                let home = self.g(k, A44);
                if y < home.wrapping_sub(0x500) {
                    self.s(k, VD, 2);
                } else if home.wrapping_add(0x500) < y || 0x16FF < y {
                    self.s(k, VD, 1);
                }
                if self.g(k, SHOOT) == 0 {
                    *dy = if self.g(k, VD) == 1 { -0x100 } else { 0x100 };
                }
                for i in 2..8 {
                    let a = self.chain(i - 1);
                    let b = self.chain(i);
                    let d = self.g(a, Y).wrapping_sub(self.g(b, Y));
                    if d < 0x101 {
                        if d < -0x100 {
                            self.add(b, Y, -0x100);
                        }
                    } else {
                        self.add(b, Y, 0x100);
                    }
                }
                L::Shoot
            }
            0x10 => {
                if self.g(k, SHOOT) != 0 {
                    self.add(k, SHOOT, -1);
                    if self.g(k, SHOOT) == 0 {
                        self.s(k, FR, 6);
                        self.s(k, VULN, 1);
                    } else if self.g(k, SHOOT) < self.ty(t, 0x25) as i16 - 2 || self.g(k, ST) == 2 {
                        self.s(k, FR, 5);
                    } else {
                        self.s(k, FR, 6);
                    }
                    return L::Apply;
                }
                L::Main
            }
            0x11 => {
                if self.g(k, PH) == 6 {
                    if self.sx.wrapping_add(0x138) < x {
                        self.s(k, AN, 3);
                        self.s(k, FC, 3);
                    } else if x < self.sx {
                        self.s(k, AN, 4);
                        self.s(k, FC, 4);
                    }
                    *dx = self.speed(k, self.g(k, AN));
                    *dy = self.m.w(0x40CC - 2 * self.g(k, STEP) as i32);
                    self.add(k, STEP, 1);
                    if *dy == 1000 {
                        self.s(k, STEP, 0);
                        self.s(k, PH, 5);
                        self.s(k, Y, y & !0xFF);
                        *dy = 0;
                    } else if 0x1800 < y && 0 < *dy {
                        *dy = 0;
                    }
                }
                if self.g(k, PH) == 5 {
                    let target = py.wrapping_sub(0x600);
                    let yy = self.g(k, Y) as u16;
                    if yy == target {
                        self.s(k, PH, 6);
                        self.s(k, STEP, 0);
                        let f = lr(self, x);
                        self.s(k, FC, f);
                        self.s(k, AN, f);
                    } else {
                        *dy = if yy < target { 0x80 } else { -0x80 };
                        *dx = if px < x.wrapping_sub(self.sx) as u16 {
                            -4
                        } else {
                            4
                        };
                    }
                }
                if py < 0xF01 && self.trec(0x4C, 1) < 1 {
                    self.shot(s, 0x4C, x, y.wrapping_sub(0x200), 0);
                }
                L::Anim
            }
            0x12 => {
                if self.sx.wrapping_add(0x138) < x || x < self.sx {
                    let f = World::flip(self.g(k, AN));
                    self.s(k, AN, f);
                    self.s(k, FC, f);
                }
                L::Shoot
            }
            0x13 | 0x14 | 0x52 => {
                if self.g(k, PH) != 0 {
                    L::Main
                } else {
                    L::Apply
                }
            }
            0x19 | 0x1A => self.twins(s, k, dy),
            0x1B => {
                if self.g(k, PH) < 8 {
                    let f = lr(self, x);
                    self.s(k, FC, f);
                    if self.sx.wrapping_add(0x128) < x {
                        self.s(k, AN, 3);
                        self.s(k, FC, 3);
                    } else if x < self.sx.wrapping_add(0x10) {
                        self.s(k, AN, 4);
                        self.s(k, FC, 4);
                    }
                    let st = self.g(k, STEP);
                    self.add(k, STEP, 1);
                    let mut v = self.m.w(0x3D48 - 2 * st as i32);
                    if v == 1000 {
                        self.s(k, STEP, 1);
                        v = self.m.w(0x3D48);
                    } else if v != 0 {
                        *dx = self.speed(k, self.g(k, AN));
                    }
                    *dy = v;
                    self.add(k, A48, 1);
                    if self.g(k, A48) < 5 {
                        return L::Apply;
                    }
                    self.s(k, A48, 0);
                    self.add(k, PH, 1);
                } else {
                    let f = self.g(k, FC);
                    self.s(k, AN, f);
                    self.s(k, FR, -1);
                    self.s(k, PH, 1);
                    self.s(k, STEP, 0);
                }
                L::Anim
            }
            0x35 => {
                if self.g(k, CD) == 0 {
                    return L::Main;
                }
                self.add(k, CD, -1);
                if self.g(k, CD) == 0 {
                    self.s(k, FALL, 1);
                }
                L::Anim
            }
            0x4C => {
                match self.g(k, A44) {
                    3 | 4 => {
                        *dx = self.g(k, A46);
                        *dy = self.speed(k, self.g(k, VD));
                    }
                    1 | 2 => {
                        *dy = self.g(k, A46);
                        *dx = self.speed(k, self.g(k, AN));
                    }
                    _ => {}
                }
                L::Anim
            }
            0x50 => {
                self.add(k, A48, -1);
                if self.g(k, A48) == 0 {
                    let f = World::flip(self.g(k, AN));
                    self.s(k, AN, f);
                    self.s(k, FC, f);
                }
                L::Move
            }
            0x51 => {
                if self.g(k, PH) == 5 {
                    if drow < 8 && dcol < 8 {
                        self.s(k, PH, 0);
                    } else {
                        *dy = if (y as u16) < py { 0x80 } else { -0x80 };
                        *dx = if px < x.wrapping_sub(self.sx) as u16 {
                            -4
                        } else {
                            4
                        };
                    }
                } else {
                    if self.g(k, SHOOT) == 0x10 {
                        self.aim(s, k, x, y, s.x, s.y.wrapping_sub(0x200));
                        self.s(k, A48, 0xE);
                    }
                    if self.g(k, SHOOT) != 0 {
                        self.add(k, SHOOT, -1);
                    }
                    if self.g(k, SHOOT) == 0 {
                        let moved = match self.g(k, A44) {
                            3 | 4 => {
                                *dx = self.g(k, A46);
                                *dy = self.speed(k, self.g(k, VD));
                                true
                            }
                            1 | 2 => {
                                *dy = self.g(k, A46);
                                *dx = self.speed(k, self.g(k, AN));
                                true
                            }
                            _ => false,
                        };
                        if moved {
                            self.add(k, A48, -1);
                            if self.g(k, A48) == 0 {
                                self.s(k, SHOOT, 0x10);
                                self.s(k, PH, 5);
                            }
                        }
                    }
                }
                L::Anim
            }
            _ => {
                let kind = self.g(k, KIND);
                let n = self.g(k, PH) as i32;
                if kind == 0x35 {
                    let v = self.m.w(G_ELEV - 8 * n);
                    self.s(k, Y, v);
                } else if kind == 0x36 {
                    let v = self.m.w(G_PLAT - 8 - 10 * n);
                    self.s(k, X, v);
                }
                L::Anim
            }
        }
    }

    /// Types 0x19 and 0x1A in +4528: the twin bosses. One rises from the floor and drops its partner; then they
    /// walk, vanish and come back together at a random place, the first one firing three shots.
    fn twins(&mut self, s: &mut Simon, k: i16, dy: &mut i16) -> L {
        let t = self.g(k, TYPE);
        let x = self.g(k, X);
        let y = self.g(k, Y);
        if t == 0x1A {
            self.g12b2 = k;
        } else {
            self.g12b4 = k;
        }
        if t == 0x1A && self.g(k, PH) == 10 && self.g(k, A48) != 0 {
            *dy = -0x40;
            self.add(k, A48, -1);
            if self.g(k, A48) == 0 {
                self.s(k, PH, 0);
                self.g12b0 = 1;
                let fc = self.g(k, FC);
                let d = if fc == 4 { -8 } else { 8 };
                let k2 = self.spawn(
                    2,
                    0x38,
                    0x19,
                    x.wrapping_add(d),
                    y.wrapping_add(0x500),
                    fc,
                    0,
                );
                if k2 != 0 {
                    self.s(k2, LIFE, 0);
                    self.s(k2, FR, 0);
                    let v = self.g(k2, ST);
                    self.s(k2, PST, v);
                }
            }
            return L::Anim;
        }
        self.add(k, A48, 1);
        let n = self.g(k, A48);
        if 0x18 < n {
            if n == 0x19 {
                self.s(k, X, 400);
            } else if 0x2D < n {
                if t == 0x19 {
                    let r = self.rnd(0x28);
                    let nx = self.sx.wrapping_add(r * 8);
                    self.s(k, X, nx);
                    let f = if (s.x as u16) < nx.wrapping_sub(self.sx) as u16 {
                        3
                    } else {
                        4
                    };
                    self.s(k, AN, f);
                    self.s(k, FC, f);
                    let (b2, b4) = (self.g12b2, self.g12b4);
                    self.s(b2, AN, f);
                    self.s(b2, FC, f);
                    let bx = self.g(b4, X);
                    let d = if self.g(b4, FC) == 4 { 8 } else { -8 };
                    self.s(b2, X, bx.wrapping_add(d));
                    self.s(b4, STUN, 0xE);
                    self.s(b2, STUN, 0xE);
                }
                self.s(k, FR, 0);
                self.s(k, A48, 0);
                return L::Apply;
            }
            return L::Anim;
        }
        if t == 0x19 {
            if n == 0xC {
                return L::Anim;
            }
            if n == 0xF {
                self.shot(s, 0x4C, x, y.wrapping_sub(0x280), 0);
                self.shot(s, 0x64, x, y.wrapping_sub(0x280), 0);
                self.shot(s, 0x65, x, y.wrapping_sub(0x280), 0);
            }
        }
        L::Apply
    }
}

/// A shape's header numbers the hit tests use: height (lines), x anchor, y anchor, clip width.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hdr {
    pub h: i16,
    pub ax: i16,
    pub ay: i16,
    pub cw: i16,
}

/// A box in map pixels (x) and rows (8.8, y): left, right, top, bottom.
#[derive(Debug, Clone, Copy, Default)]
struct Bx {
    l: i16,
    r: i16,
    t: i16,
    b: i16,
}

// ---- hits: the player and his weapons against everything else
impl World {
    fn hdr(&self, ix: i16) -> Hdr {
        if ix < 0 {
            return Hdr::default();
        }
        self.hdrs.get(ix as usize).copied().unwrap_or_default()
    }

    /// Object `k`'s box from its shape.
    fn obj_box(&self, k: i16) -> (Bx, Hdr) {
        let h = self.hdr(self.shape_ix(k));
        let l = self.g(k, X).wrapping_sub(h.ax);
        let b = self.g(k, Y);
        (
            Bx {
                l,
                r: l.wrapping_add(h.cw),
                t: b.wrapping_sub(h.h.wrapping_mul(0x20)),
                b,
            },
            h,
        )
    }

    /// +6386 accumulate_and_test_range: score; passing the next threshold (-$7965 byte table, in ten thousands,
    /// by the extra lives given so far) gives an extra life.
    pub fn score(&mut self, s: &mut Simon, n: i32) {
        let old = s.score;
        s.score += n;
        let next = self.m.ub(0x7965 - s.extra as i32) as i32 * 10000;
        if old < next && next <= s.score {
            self.fx(0);
            s.lives += 1;
            s.extra += 1;
        }
    }

    /// +7610 scan_table_for_value: every hurting enemy dies (the boss's minions when it falls).
    pub fn kill_hostiles(&mut self, s: &mut Simon) {
        for k in 1..0x11 {
            if self.g(k, MODE) == 2 {
                self.die(s, k);
            }
        }
    }

    /// +7784 call_other_routine_twenty_times: the rosary: the screen flashes for ten frames, then every enemy
    /// dies. The flashing is done by the frame loop (`flash_frames`).
    fn rosary(&mut self) {
        self.rosary = 10;
    }

    /// +74EC process_entity_state: object `k` dies in a flame; an enemy may leave an item (half the time).
    pub fn die(&mut self, s: &mut Simon, k: i16) {
        let st = self.g(k, ST);
        if st == 4 || st == 0 {
            return;
        }
        let t = self.g(k, TYPE);
        let x = self.g(k, X);
        let y = self.g(k, Y);
        if t == 0x10 {
            self.s(k, VULN, 0);
            let v = self.m.b(0x7671) as i16;
            self.s(k, SHOOT, v);
        } else {
            self.fx(0xD);
            let f = self.spawn(1, 0x1A, 0x1D, x, y.wrapping_sub(0x200), 0, 0);
            if f != 0 {
                self.s(f, FR, 0);
            }
            self.s(k, ST, 4);
            self.s(k, HY, 0);
            self.s(k, HX, 0);
            let live = self.m.w(G_LIVE) - 1;
            self.m.sw(G_LIVE, live);
            self.set_trec(t, 1, self.trec(t, 1) - 1);
        }
        let it = self.g(k, ITEM);
        if it != 0 {
            self.m.sw(G_ITEMS - 8 * (it as i32 - 1), 0);
        }
        if self.g(k, MODE) == 2 {
            self.set_trec(t, 2, self.trec(t, 3));
            if self.g(k, KIND) != 0x39 && t != 0x10 && self.rnd(0x14) < 10 {
                self.drop_from(s, x, y.wrapping_sub(0x300), k);
            }
        }
    }

    /// +700C clear_entity_slot_and_schedule_updates: a hit. `who` above 0 is an object hit at the top of its box
    /// `top` (it loses the whip's or weapon's damage; at 0 it dies and scores, a boss leaves the orb); `who` 0 or
    /// below is wall `-who` of the room, which breaks. Returns 1 when an enemy died.
    pub fn damage(&mut self, s: &mut Simon, who: i16, top: i16) -> i16 {
        if who < 1 {
            self.fx(2);
            let i = -who as i32;
            let e = G_WALLS - 0xC * i - 0x24 * self.van as i32;
            self.wall_draw = -who + 1;
            let col = self.m.w(e - 2);
            let row = self.m.w(e - 4);
            for (dc, dr) in [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)] {
                let p = (row + dr) as i32 * self.mw as i32 + (col + dc) as i32;
                self.set_at(p, 0);
                self.cleared.push((col + dc, row + dr));
            }
            let item = self.m.w(e - 8);
            if item != 0 {
                let t = if (0x20..=0x24).contains(&item) && item == s.sub {
                    0x25
                } else {
                    item
                };
                self.drop_item(t, (col << 3) + 4, (row + 2) << 8);
            }
            self.popup(0x3A, col << 3, row << 8, 4, 0);
            self.popup(0x3B, (col + 2) * 8, row << 8, 4, 0);
            self.popup(0x3C, col << 3, (row + 2) * 0x100, 4, 0);
            self.popup(0x3D, (col + 2) * 8, (row + 2) * 0x100, 4, 0);
            self.m.sw(e - 10, 1);
            return 0;
        }
        let mut v = who;
        if self.g(v, TYPE) == 0x13 {
            v = self.boss;
        }
        let t = self.g(v, TYPE);
        let mut ret = 0;
        if self.g(v, MODE) == 2 {
            if t == 0x52 {
                self.s(v, STUN, 0x14);
                return 0;
            }
            if self.tyw(t, 0x26) != 0 {
                let hp = (self.g(v, HP) - s.dmg).max(0);
                self.s(v, HP, hp);
                if self.g(v, KIND) == 0x39 {
                    self.boss_hp = hp;
                }
                if hp == 0 {
                    if t == 0x1A {
                        self.remove_all();
                        self.boss_dead = v;
                    } else if t == 0xE {
                        self.kill_hostiles(s);
                        self.score(s, 1000);
                    } else {
                        self.die(s, v);
                        ret = 1;
                        let n = self.tyw(t, 0x28) as i32;
                        self.score(s, n);
                    }
                    if self.g(v, KIND) == 0x39 && t != 0x1A {
                        self.kill_hostiles(s);
                        let o = self.spawn(1, 0x32, 0x35, self.sx.wrapping_add(0x90), 0xA00, 0, 0);
                        if o != 0 {
                            self.s(o, FR, 0);
                            let v2 = self.g(o, ST);
                            self.s(o, PST, v2);
                            self.s(o, OWN, 0);
                            let c = self.ty(0x35, 0x25) as i16;
                            self.s(o, CD, c);
                        }
                    }
                } else {
                    let x = if self.g(who, TYPE) == 0x13 {
                        self.g(who, X)
                    } else {
                        self.g(v, X)
                    };
                    let sp = self.spawn(1, 0x19, 0x1C, x - 6, top.wrapping_add(0x200), 0, 0);
                    if sp != 0 {
                        self.s(sp, FR, 0);
                    }
                    self.fx(0x14);
                }
            }
        } else if t == 0x1E || t == 0x1F {
            let p =
                ((self.g(v, HY) - 1) >> 8) as i32 * self.mw as i32 + (self.g(v, HX) >> 3) as i32;
            self.set_at(p, 0);
            self.die(s, v);
            let (x, y) = (self.g(v, X), self.g(v, Y));
            self.drop_from(s, x, y, v);
        }
        ret
    }

    /// The player's box (+63DE, +6D4C): from his frame's shape in the castshps file.
    fn player_box(&self, s: &Simon, frame: u8) -> Bx {
        let h = self.phdrs.get(frame as usize).copied().unwrap_or_default();
        let l = self.sx.wrapping_add(s.x).wrapping_sub(h.ax);
        let b = s.y.wrapping_sub(0x20);
        let t = if s.crouch != 0 {
            s.y.wrapping_sub(0x1E0)
        } else {
            s.y.wrapping_sub(0x3E0)
        };
        Bx {
            l,
            r: l.wrapping_add(h.cw),
            t,
            b,
        }
    }

    /// +6D4C execute_collision_checks: the whip's last frame: what it reaches is hit, and a breakable wall.
    pub fn whip(&mut self, s: &mut Simon, whip_frame: u8, body_ended: bool) {
        self.fx(0x16);
        let f = if body_ended { 0 } else { whip_frame };
        let h = self.phdrs.get(f as usize).copied().unwrap_or_default();
        let left = self.sx.wrapping_add(s.x).wrapping_sub(h.ax);
        let right = s.whip_len.wrapping_add(left);
        let mut top = s.y.wrapping_sub(h.ay.wrapping_mul(0x20));
        if s.crouch != 0 {
            top = top.wrapping_add(0x100);
        }
        for k in 1..0x10 {
            if self.g(k, ST) != 1 {
                continue;
            }
            let (mut b, _) = self.obj_box(k);
            let t = b.t;
            if self.g(k, TYPE) == 0x1B {
                b.l += 0x10;
                b.r -= 0x10;
                b.b = t.wrapping_add(0x300);
            }
            if self.g(k, VULN) == 1
                && b.l <= right
                && left <= b.r
                && top.wrapping_sub(0x180) <= b.b
                && t <= top
            {
                self.damage(s, k, t);
            }
        }
        for i in 0..3 {
            let e = G_WALLS - 0xC * i - 0x24 * self.van as i32;
            let lap = self.m.w(e);
            if lap == 0 {
                break;
            }
            let col = self.m.w(e - 2);
            let row = self.m.w(e - 4);
            if self.m.w(e - 10) != 1
                && lap <= self.lap
                && col << 3 <= right
                && left <= (col + 3) * 8
                && top.wrapping_sub(0x180) <= (row + 2) * 0x100
                && row << 8 <= top
            {
                self.damage(s, -(i as i16), 0);
                return;
            }
        }
    }

    /// +63DE check_enemy_collision: the player and his weapons against everything: a hurting object hurts the
    /// player (the block's damage; a falling platform crushes him); a weapon hits enemies (a combo of hits
    /// scores); the player picks up items. Returns true when the player was hit.
    pub fn collisions(&mut self, s: &mut Simon, frame: u8) -> bool {
        let mut hit = false;
        for a in 0..0x10i16 {
            let ab = if a == 0 {
                self.player_box(s, frame)
            } else {
                if self.g(a, ST) != 1
                    || self.g(a, VULN) != 1
                    || !((self.g(a, KIND) == 0x33 && self.g(a, OWN) == 1)
                        || self.g(a, TYPE) == 0x50)
                {
                    continue;
                }
                self.obj_box(a).0
            };
            let akind = if a == 0 { 0 } else { self.g(a, KIND) };
            for v in 1..0x10i16 {
                if a == v
                    || akind == self.g(v, KIND)
                    || self.g(v, VULN) == 0
                    || self.g(v, ST) != 1
                    || self.class(self.g(v, TYPE)) == 5
                {
                    continue;
                }
                let (vb0, vh) = self.obj_box(v);
                if vb0.l > ab.r || ab.l > vb0.r {
                    continue;
                }
                let platform = self.g(v, KIND) == 0x35;
                let vbot = if platform {
                    vb0.b.wrapping_sub(vh.ay.wrapping_mul(0x20))
                } else {
                    vb0.b
                };
                if ab.t > vbot {
                    continue;
                }
                let vtop = if platform {
                    0xA00
                } else {
                    vbot.wrapping_sub(vh.h.wrapping_mul(0x20))
                };
                if vtop > ab.b {
                    continue;
                }
                if a == 0 && self.g(v, MODE) == 2 && s.hurt == 0 {
                    s.hurt_facing = if self.g(v, FC) == 4 { 4 } else { 3 };
                    if !s.no_collision && s.inv == 0 && s.hp > 0 {
                        s.hp -= self.hit_dmg;
                        if platform {
                            s.hp = 0;
                        }
                        s.hp = s.hp.max(0);
                        hit = true;
                    } else {
                        hit = false;
                    }
                    break;
                }
                if self.g(v, TYPE) == 0x24
                    && s.facing != self.g(v, FC)
                    && self.g(v, OWN) == 1
                    && a == 0
                {
                    self.remove(v);
                    continue;
                }
                if akind == 0x33
                    && self.g(a, OWN) == 1
                    && self.g(a, HITBY) != v
                    && self.g(v, HP) > 0
                {
                    self.s(a, HITBY, v);
                    if self.damage(s, v, vtop) == 1 {
                        self.add(a, HITS, 1);
                        let (n, pt) = match self.g(a, HITS) {
                            2 => (1000, 0x54),
                            3 => (2000, 0x55),
                            4 => (4000, 0x56),
                            _ => (0, 0),
                        };
                        if n != 0 {
                            self.score(s, n);
                            let (vx, vy) = (self.g(v, X), self.g(v, Y));
                            self.popup(pt, vx + 0x10, vy.wrapping_sub(0x200), 0, 0);
                        }
                    }
                    if self.g(a, TYPE) == 0x21 {
                        self.remove(a);
                    }
                } else if a != 0 && self.g(a, TYPE) == 0x50 && self.g(v, TYPE) == 0xF {
                    self.remove(v);
                    continue;
                }
                let c = self.class(self.g(v, TYPE));
                if a == 0 && self.g(v, OWN) == 0 && (c == 4 || c == 6) {
                    self.pick_up(s, v);
                }
            }
        }
        hit
    }

    /// The pick-ups of +63DE (its jump table from type 0x25): hearts, the rosary, the potion, the whip, money,
    /// the shot multipliers, the 1-up, meat and the orb that ends the stage; a sub-weapon replaces the player's.
    fn pick_up(&mut self, s: &mut Simon, v: i16) {
        let t = self.g(v, TYPE);
        let (vx, vy) = (self.g(v, X), self.g(v, Y));
        let pop = |w: &mut World, s: &mut Simon, n: i32, p: i16| {
            w.fx(0x10);
            w.score(s, n);
            w.popup(p, vx + 0x10, vy.wrapping_sub(0x200), 0, 0);
        };
        self.remove(v);
        if self.g(v, KIND) == 0x33 {
            self.fx(0xA);
            s.sub = t;
            s.shot = 0;
            return;
        }
        match t - 0x25 {
            0 => {
                self.fx(9);
                s.hearts = (s.hearts + 1).min(99);
            }
            1 => {
                self.fx(0xF);
                self.rosary();
            }
            2 => {
                self.fx(0xB);
                s.inv = 6;
            }
            3 => {
                self.fx(0xA);
                if s.whip_level < 2 {
                    s.whip_level += 1;
                    s.whip_len = self.m.ub(0x7959 - s.whip_level as i32) as i16;
                    s.cross = 2;
                }
                s.dmg = 0x14;
            }
            4 => pop(self, s, 700, 0x39),
            5 => pop(self, s, 400, 0x38),
            6 => pop(self, s, 100, 0x37),
            7 => {
                self.fx(9);
                s.hearts = (s.hearts + 5).min(99);
            }
            8 | 9 => {
                self.fx(0xA);
                if s.sub == 0 || s.sub == 0x20 {
                    self.score(s, 700);
                } else {
                    s.shot = if t - 0x25 == 8 { 2 } else { 1 };
                }
            }
            0xA | 0xB => pop(self, s, 2000, 0x55),
            0xC => pop(self, s, 4000, 0x56),
            0xD => {
                self.fx(0);
                s.lives += 1;
            }
            0xE => pop(self, s, 1000, 0x54),
            0xF => {
                self.fx(0x10);
                s.hp = (s.hp + 6).min(0x10);
            }
            0x10 => {
                self.tune(0xB);
                self.orb = 1;
                self.orb_wait = if s.time < 0x1F { 0x14 } else { 0x32 };
                s.hp = 0x10;
            }
            _ => {}
        }
    }
}

/// A shape to draw this frame: art file, frame, x and y in picture coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draw {
    pub file: String,
    pub frame: usize,
    pub x: i32,
    pub y: i32,
}

// ---- rooms, the map's marks, drawing and the boss's bar
impl World {
    /// +0ED2's object part (a room is entered): every object goes, the item marks are forgotten.
    pub fn clear_room(&mut self) {
        self.wall_draw = 0;
        for k in 1..0x11 {
            self.s(k, PST, 0);
            self.s(k, ST, 0);
            self.s(k, HY, 0);
            self.s(k, HX, 0);
            let live = self.m.w(G_LIVE) - 1;
            self.m.sw(G_LIVE, live);
            let t = self.g(k, TYPE);
            self.set_trec(t, 1, 0);
            let it = self.g(k, ITEM);
            if it != 0 {
                self.m.sw(G_ITEMS - 8 * (it as i32 - 1), 0);
            }
            if self.g(k, MODE) == 2 {
                self.set_trec(t, 2, self.trec(t, 3));
            }
        }
        for i in 0..10 {
            self.m.sw(G_ITEMS - 8 * i, 0);
        }
        self.boss = 0;
        self.boss_hp = 0;
        self.prev_boss = 0;
        self.boss_dead = 0;
        self.bar_anim = 0;
        self.rosary = 0;
        self.cleared.clear();
    }

    /// One map mark at (col, row) as the column scans read it (+20EC, +25C6, +273A): candles, enemies and item
    /// marks. `right` is +273A (a column coming in on the right), which skips code 0x16.
    fn mark(&mut self, s: &Simon, b: i8, col: i16, row: i16, right: bool) {
        let u = b as u8;
        if u == 0 || u == 1 || u == 2 {
            return;
        }
        if u == 0x20 {
            self.candle(col, row, 0x1B, 0x1E);
        } else if u == 0x40 {
            self.candle(col, row, 0x1C, 0x1F);
        }
        if b < 0 {
            let code = (b & 0xF) as i16;
            match u & 0x70 {
                0x20 => {
                    let t = self.m.ub(G_ITEMTYPE - code as i32 - 6 * self.room_van_ix());
                    if !right || t != 0x16 {
                        self.map_enemy(s, code, col, row);
                    }
                }
                0x40 => self.item_mark(code, col, row),
                0x60 => {
                    self.item_mark(code, col, row);
                    self.map_enemy(s, code, col, row);
                }
                _ => {}
            }
        }
    }

    /// +20EC's marks: the screen's 40 columns as the room is entered. Returns the decorations (mark 0x10: entry
    /// 0x7F of the shape table) to draw into the background: (x, y).
    pub fn room_marks(&mut self, s: &Simon) -> Vec<(i32, i32)> {
        let mut deco = Vec::new();
        for row in 3..0x19i16 {
            for c in 0..0x28i16 {
                let col = self.scol + c;
                let b = self.at(row as i32 * self.mw as i32 + col as i32);
                if b == 0x10 {
                    deco.push((c as i32 * 8 + 3 + self.sx as i32, (row as i32 + 1) * 8));
                    continue;
                }
                self.mark(s, b, col, row, false);
            }
        }
        self.start_platforms();
        deco
    }

    /// +25C6 parse_script_bytes and +273A scan_tile_map_for_events: the columns the screen scrolled on to (left:
    /// from the new first column; right: from the last), `n` of them.
    pub fn scan(&mut self, s: &Simon, left: bool, n: i16) {
        for u in 0..n {
            let (scan, col) = if left {
                let c = (self.scol - 1).max(0) + u + 1;
                (c, self.scol + u)
            } else {
                let c = self.scol + 0x28 - 1 - u;
                (c, self.scol + 0x27 - u)
            };
            for row in 3..0x19i16 {
                let b = self.at(row as i32 * self.mw as i32 + scan as i32);
                self.mark(s, b, col, row, !left);
            }
        }
    }

    /// +1A40's object part and +18FC compute_entity_score: the objects in depth order (by their feet), each
    /// from the shape table at (x + 8, y / 32); a stunned object blinks.
    pub fn draws(&mut self) -> Vec<Draw> {
        self.blink = self.blink.wrapping_add(1);
        let mut order: Vec<(u16, i16)> = (1..0x10i16)
            .map(|k| {
                let key = (self.g(k, Y).wrapping_add(self.g(k, F04).wrapping_mul(0x20)) as u16)
                    & 0xFFF0
                    | k as u16;
                (key, k)
            })
            .collect();
        order.sort_by_key(|e| e.0);
        let mut out = Vec::new();
        for (_, k) in order {
            let st = self.g(k, ST);
            if st == 4 || st == 0 || st == 8 {
                continue;
            }
            if self.g(k, PSTUN) != 0 && self.blink & 1 != 0 {
                continue;
            }
            let ix = self.shape_ix(k);
            if !(0..0xD9).contains(&ix) {
                continue;
            }
            if let Some((file, frame)) = self.shp[ix as usize].clone() {
                out.push(Draw {
                    file,
                    frame,
                    x: self.g(k, X) as i32 + 8,
                    y: (self.g(k, Y) >> 5) as i32,
                });
            }
        }
        out
    }

    /// +19D8 update_table_entry: the boss bar's target (16 segments of the boss's health, as last frame) and
    /// the start of its animation.
    pub fn boss_bar(&mut self, s: &Simon) {
        if self.prev_boss == 0 || self.prev_full == 0 {
            return;
        }
        let v = ((self.prev_hp as i32) << 12) / self.prev_full as i32;
        let mut seg = (v / 0x100) as i16;
        if v & 0xFF != 0 {
            seg += 1;
        }
        if seg != s.enemy_hp {
            self.bar_anim += 2;
            self.bar_dir = if seg > s.enemy_hp { 1 } else { -1 };
        }
    }

    /// +16C6's boss part: the boss, its health and its full health as they were.
    pub fn save_boss(&mut self) {
        self.prev_boss = self.boss;
        self.prev_hp = self.boss_hp;
        self.prev_full = self.boss_full;
    }

    /// +1A40's boss bar animation, one segment a frame: emptying as it is hurt (when the twin bosses' bar is
    /// empty their third form comes), or filling when a boss appears. Returns the segments to draw: (index,
    /// full).
    pub fn bar_step(&mut self, s: &mut Simon) -> Vec<(i16, bool)> {
        let mut out = Vec::new();
        if self.bar_anim == 0 {
            return out;
        }
        if self.bar_dir < 0 {
            let mut go_on = self.bar_anim < 2;
            if !go_on {
                s.enemy_hp -= 1;
                go_on = s.enemy_hp != 0 || self.boss_dead == 0;
            }
            if go_on {
                out.push((s.enemy_hp, false));
                self.bar_anim -= 1;
            } else {
                let b = self.boss_dead;
                let k = self.spawn(
                    2,
                    0x39,
                    0x1B,
                    self.g(b, X),
                    self.g(b, Y).wrapping_add(0x500),
                    self.g(b, FC),
                    0,
                );
                self.s(k, FR, 0);
                self.s(k, LIFE, 0);
                self.boss = k;
                self.boss_full = self.m.w(0x748C);
                self.boss_hp = self.boss_full;
                self.tune(7);
                self.boss_dead = 0;
                self.bar_anim = 0;
            }
        } else if self.bar_dir > 0 {
            s.enemy_hp = 0;
            while s.enemy_hp < 0x10 {
                out.push((s.enemy_hp, true));
                s.enemy_hp += 1;
            }
            self.bar_anim = 0;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_below_is_the_originals() {
        // +B76E divides by n - 1: random_below(2) is always 0, random_below(1) and (0) too
        for r in [0, 1, 7, 0x7FFF] {
            assert_eq!(below(r, 2), 0);
            assert_eq!(below(r, 1), 0);
        }
        assert_eq!(below(10, 6), 0);
        assert_eq!(below(11, 6), 1);
        let mut w = World::new(&[]);
        w.rng = 1;
        let a = w.rand();
        assert_eq!(
            a as u32,
            (1u32.wrapping_mul(0x41C6_4E6D).wrapping_add(0x3039) >> 16) & 0x7FFF
        );
    }

    #[test]
    fn objects_are_made_and_go() {
        let mut w = World::new(&[]);
        let k = w.spawn(2, 0x38, 7, 0x40, 0x1700, 3, 0);
        assert_eq!(k, 1);
        assert_eq!(w.g(k, ST), 8);
        assert_eq!(w.g(k, TYPE), 7);
        assert_eq!(w.trec(7, 1), 1);
        assert_eq!(w.m.w(G_LIVE), 1);
        // the same place twice: classes below 3 are not made again
        assert_eq!(w.spawn(2, 0x38, 7, 0x40, 0x1700, 3, 0), 0);
        let k2 = w.spawn(2, 0x38, 7, 0x48, 0x1700, 3, 0);
        assert_eq!(k2, 2);
        w.remove(k);
        assert_eq!(w.g(k, ST), 4);
        assert_eq!(w.trec(7, 1), 1);
        w.save_prev();
        w.free_gone();
        assert_eq!(w.g(k, ST), 0);
        assert_eq!(w.spawn(1, 0x34, 0x37, 8, 0x100, 0, 0), 1);
    }

    #[test]
    fn data_area_words_are_big_endian_below_the_base() {
        let mut m = Mem::new(&[]);
        m.sw(0x2136, 0x1234);
        assert_eq!(m.ub(0x2136), 0x12);
        assert_eq!(m.ub(0x2135), 0x34);
        assert_eq!(m.w(0x2136), 0x1234);
        assert_eq!(m.w(0x9000), 0);
    }
}
