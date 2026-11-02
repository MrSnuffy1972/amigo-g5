//! One round being played: the field, the player (the player record at BSS +0044 and its routine +643E), the cut
//! (the corner list at BSS +009C) and the enemies.
//!
//! Translated from the recovered code: the player's start, movement, cutting, backing up along the cut, closing a
//! claim, the shield and its count-down, the percentage and the round's end. NOT yet translated (stage 2 has not
//! read them): the 16 per-round enemy handlers (+999A .. +14658), the power-up boxes (+82F6) and the score for a
//! claim. The enemies follow round 1's handler (+999A: the guards, the boss cycle and its sparks) on the shared motion
//! of `motion`, in every round until the other fifteen handlers are translated; see docs/native/g1727-native.md for
//! what is exact and what is still approximate.

use crate::field::{Cell, Field};
use crate::motion::{reflect, touches_player, towards, turn_towards, Motion, FACING};
use crate::rng::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    /// The step of the original's direction routines (+66C4: right, down, left, up).
    pub fn delta(self) -> (i32, i32) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }
    pub fn opposite(self) -> Dir {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
}

/// Player start: the middle of the bottom edge (+5BC0: x $8E, y $C6).
pub const START: (i32, i32) = (0x8E, 0xC6);
/// The shield a round starts with (+5BCA: $900 BCD, shown as 900).
pub const SHIELD_START: u16 = 900;
/// Player-routine calls between shield steps (+6458 reloads it from +636E, set per difficulty at +5746). 5 is the
/// value measured on NORMAL: the shield drops by one about every ten frames, and the routine runs twice a frame.
pub const SHIELD_STEP: u8 = 5;

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub shield: u16,
    pub shield_on: bool,
    shield_count: u8,
    /// The every-other-call move toggle (+42) used without the speed power-up.
    half: bool,
    /// The open cut, every pixel in order (the original keeps its corners at BSS +009C).
    pub cut: Vec<(i32, i32)>,
    pub last: Option<Dir>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: START.0,
            y: START.1,
            shield: SHIELD_START,
            shield_on: true,
            shield_count: 0,
            half: false,
            cut: Vec::new(),
            last: None,
        }
    }
    pub fn cutting(&self) -> bool {
        !self.cut.is_empty()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The boss record (BSS +0800).
    Boss,
    /// A guard (44-byte records from BSS +06A0).
    Guard,
    /// A spark launched by the boss (30-byte records at a5+6042).
    Spark,
}

/// Round 1's guard animation (frames 0, 1, 2, 1, one step every 5 calls).
const GUARD_ANIM: [u8; 4] = [0, 1, 2, 1];
/// Score for a guard caught in a claim, by its index (BCD 100 .. 6400 in the original).
pub const GUARD_SCORE: [u32; 7] = [100, 200, 400, 800, 1600, 3200, 6400];
/// Calls in one boss phase (+999A set-up: 300).
pub const BOSS_PHASE: u16 = 300;

/// An enemy object: position (+0, +2), motion (+6 .. +18) and the handler's own fields.
#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    /// Half the size used for walls and the cut (0 = gone).
    pub r: i32,
    pub boss: bool,
    pub kind: Kind,
    pub m: Motion,
    /// Handler state (+4 for a guard): 0 set-up, 1 moving; for the boss 2 is the end of a phase.
    pub state: u8,
    /// Turn step (+30, +2 or -2 for a guard, the rotation step for the boss).
    pub turn_step: i16,
    /// Turn timer (+10/+20 in the original).
    pub turn_timer: u16,
    swings: u8,
    /// Base heading (+34).
    pub base: u16,
    anim: u8,
    anim_timer: u8,
    /// Animation frame shown (+24).
    pub frame: u8,
    /// Wall hits in the boss phase.
    pub hits: u16,
    /// Calls left in the boss phase.
    pub phase_left: u16,
    pause: u16,
}

impl Enemy {
    fn new(kind: Kind, x: i32, y: i32, dir: u16, r: i32) -> Enemy {
        Enemy {
            x,
            y,
            r,
            boss: kind == Kind::Boss,
            kind,
            m: Motion {
                dir: dir & 31,
                ..Motion::default()
            },
            state: 0,
            turn_step: 0,
            turn_timer: 0,
            swings: 0,
            base: dir & 31,
            anim: 0,
            anim_timer: 0,
            frame: 0,
            hits: 0,
            phase_left: 0,
            pause: 0,
        }
    }
    pub fn boss(x: i32, y: i32, dir: u16) -> Enemy {
        Enemy::new(Kind::Boss, x, y, dir, 10)
    }
    pub fn guard(x: i32, y: i32, dir: u16) -> Enemy {
        Enemy::new(Kind::Guard, x, y, dir, 5)
    }
    pub fn spark(x: i32, y: i32, dir: u16) -> Enemy {
        let mut e = Enemy::new(Kind::Spark, x, y, dir, 2);
        e.m.speed = 10;
        e.state = 1;
        e
    }
}

/// The wall flags of a move (+F6DE .. +F7C8): on each side the object faces, a short segment at `half` pixels from
/// its centre is tested against the claim map and the field's limits (left x 3, right x $11C, top y 1, bottom y
/// $C7). APPROXIMATION: the original steps a four-pixel line cursor along the side, shifted towards the motion; here
/// the seven pixels across the side are tested.
fn walls(f: &Field, x: i32, y: i32, dir: u16, half: i32) -> u16 {
    let face = FACING[((dir & 0x18) >> 3) as usize];
    let blocked = |px: i32, py: i32| matches!(f.get(px, py), Cell::Line | Cell::Claimed);
    let mut flags = 0;
    if face & 1 != 0 {
        if x - half < 3 || (-3..=3).any(|k| blocked(x - half, y + k)) {
            flags |= 1;
        }
    } else if x + half > 0x11C || (-3..=3).any(|k| blocked(x + half, y + k)) {
        flags |= 8;
    }
    if face & 2 != 0 {
        if y + half > 0xC7 || (-3..=3).any(|k| blocked(x + k, y + half)) {
            flags |= 2;
        }
    } else if y - half < 1 || (-3..=3).any(|k| blocked(x + k, y - half)) {
        flags |= 4;
    }
    flags
}

/// Round 1's guard (+999A, states 0 and 1).
fn guard_step(e: &mut Enemy, f: &Field, px: i32, py: i32, shield: bool) {
    if e.state == 0 {
        e.m.speed = 7;
        e.turn_step = 2;
        e.turn_timer = 50;
        e.base = e.m.dir;
        e.state = 1;
        return;
    }
    if e.m.step(&mut e.x, &mut e.y, true) {
        let fl = walls(f, e.x, e.y, e.m.dir, 6);
        if fl != 0 {
            e.m.dir = reflect(e.m.dir, fl);
            e.base = e.m.dir;
            e.turn_step = -e.turn_step;
            e.swings = 0;
        }
    }
    e.turn_timer = e.turn_timer.saturating_sub(1);
    if e.turn_timer == 0 {
        if shield {
            // shield on: swing by the turn step every 7 calls, back again after half a turn from the base heading
            e.turn_timer = 7;
            e.m.dir = e.m.dir.wrapping_add_signed(e.turn_step) & 31;
            e.swings += 1;
            if e.swings >= 8 {
                e.swings = 0;
                e.turn_step = -e.turn_step;
            }
        } else {
            // shield off: one step towards the player every 10 calls (+F430, +F30C)
            e.turn_timer = 10;
            let t = towards(e.x, e.y, px, py);
            if t != e.m.dir {
                e.m.dir = e.m.dir.wrapping_add_signed(turn_towards(e.m.dir, t)) & 31;
            }
        }
    }
    e.anim_timer += 1;
    if e.anim_timer >= 5 {
        e.anim_timer = 0;
        e.anim = (e.anim + 1) % 4;
    }
    e.frame = GUARD_ANIM[e.anim as usize];
}

/// Round 1's boss cycle (+999A): a 300-call phase of moving and turning, then open, flash, three sparks, close.
fn boss_step(e: &mut Enemy, f: &Field, px: i32, py: i32, sparks: &mut Vec<Enemy>) {
    if e.state == 0 {
        e.phase_left = BOSS_PHASE;
        e.turn_step = -1;
        e.m.speed = 9;
        e.hits = 0;
        e.state = 1;
        return;
    }
    if e.state == 1 {
        if e.m.step(&mut e.x, &mut e.y, false) {
            let fl = walls(f, e.x, e.y, e.m.dir, e.r);
            if fl != 0 {
                e.m.dir = reflect(e.m.dir, fl);
                e.hits += 1;
            }
        }
        // turn by the rotation step, skipping the four axis directions
        let mut d = e.m.dir.wrapping_add_signed(e.turn_step) & 31;
        if d.is_multiple_of(8) {
            d = d.wrapping_add_signed(e.turn_step) & 31;
        }
        e.m.dir = d;
        // speed from the calls left, in 32-call bands at the end of the phase
        e.m.speed = match e.phase_left {
            0..=32 => 5,
            33..=64 => 7,
            65..=96 => 9,
            _ => 11,
        };
        e.phase_left -= 1;
        if e.phase_left == 0 {
            e.state = 2;
            e.pause = 0;
        }
        return;
    }
    // the end of a phase: open (3 frames of 3 calls), flash (3 on, 6 off, 3 on, 3 off), sparks after 1, 11 and 21
    // calls, wait 10, close
    e.pause += 1;
    let launch = 9 + 15;
    e.frame = match e.pause {
        0..=9 => (e.pause.saturating_sub(1) / 3) as u8,
        _ => 2,
    };
    if [launch + 1, launch + 11, launch + 21].contains(&e.pause) {
        sparks.push(Enemy::spark(e.x, e.y, towards(e.x, e.y, px, py)));
    }
    if e.pause >= launch + 21 + 10 + 9 {
        e.frame = 0;
        e.state = 1;
        e.phase_left = BOSS_PHASE;
        e.hits = 0;
    }
}

/// A spark: straight on at speed 10 until a wall. APPROXIMATION: aimed at the player and gone at the first wall;
/// the spark routine itself is not read yet.
fn spark_step(e: &mut Enemy, f: &Field) {
    e.m.step(&mut e.x, &mut e.y, true);
    if walls(f, e.x, e.y, e.m.dir, 2) != 0 {
        e.r = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Playing,
    LifeLost,
    Cleared,
}

pub struct Round {
    pub number: usize,
    pub field: Field,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub score_gain: u32,
}

impl Round {
    pub fn new(number: usize, rng: &mut Rng) -> Round {
        // positions as before (the per-round start tables read by +F60C and +F63E are not read yet); headings from
        // the game's random numbers
        let mut enemies = vec![Enemy::boss(143, 80, rng.step() & 31)];
        for i in 0..3 {
            let v = rng.step();
            enemies.push(Enemy::guard(
                40 + 100 * i,
                30 + (v & 0x3F) as i32,
                (v >> 8) & 31,
            ));
        }
        Round {
            number,
            field: Field::new(),
            player: Player::new(),
            enemies,
            score_gain: 0,
        }
    }

    /// One call of the player routine (+643E). `dir` is the joystick, `fire` the button, `fast` the speed power-up.
    pub fn player_step(&mut self, dir: Option<Dir>, fire: bool, fast: bool) -> Outcome {
        let p = &mut self.player;
        if p.shield_on {
            if p.shield_count <= 1 {
                p.shield_count = SHIELD_STEP;
                p.shield = p.shield.saturating_sub(1);
                if p.shield == 0 {
                    p.shield_on = false;
                }
            } else {
                p.shield_count -= 1;
            }
        }
        let Some(d) = dir else {
            return Outcome::Playing;
        };
        if !fast {
            p.half = !p.half;
            if p.half {
                return Outcome::Playing;
            }
        }
        let (dx, dy) = d.delta();
        let (nx, ny) = (p.x + dx, p.y + dy);
        if !p.cutting() {
            if self.field.walkable(nx, ny) {
                p.x = nx;
                p.y = ny;
                p.last = Some(d);
            } else if fire && self.field.get(nx, ny) == Cell::Free {
                // a cut starts: the shield is off while cutting (+658A), each new pixel scores 1 (+65A4)
                p.shield_on = false;
                p.cut.push((p.x, p.y));
                p.x = nx;
                p.y = ny;
                p.last = Some(d);
                self.field.set(nx, ny, Cell::Cut);
                self.score_gain += 1;
            }
            return Outcome::Playing;
        }
        if p.last == Some(d.opposite()) {
            // backing up along the cut erases it (+6654)
            self.field.set(p.x, p.y, Cell::Free);
            let (bx, by) = p.cut.pop().unwrap_or((p.x, p.y));
            p.x = bx;
            p.y = by;
            if !p.cut.is_empty() {
                let (px, py) = *p.cut.last().unwrap();
                p.last = dir_of(bx - px, by - py);
            } else {
                p.shield_on = p.shield > 0;
            }
            return Outcome::Playing;
        }
        match self.field.get(nx, ny) {
            Cell::Free => {
                p.cut.push((p.x, p.y));
                p.x = nx;
                p.y = ny;
                p.last = Some(d);
                self.field.set(nx, ny, Cell::Cut);
                self.score_gain += 1;
            }
            Cell::Line => {
                // the claim closes (+6608): the side without the boss is filled (+68DE)
                p.x = nx;
                p.y = ny;
                p.last = Some(d);
                p.cut.clear();
                p.shield_on = p.shield > 0;
                let boss = self.enemies.iter().find(|e| e.boss).map(|e| (e.x, e.y));
                self.field.close(boss.unwrap_or((143, 99)));
                let mut caught = 0;
                for e in self.enemies.iter_mut() {
                    if !e.boss && self.field.get(e.x, e.y) != Cell::Free {
                        // caught in the claim: gone; a guard scores by its index
                        if e.kind == Kind::Guard {
                            self.score_gain += GUARD_SCORE[caught.min(GUARD_SCORE.len() - 1)];
                            caught += 1;
                        }
                        e.r = 0;
                    }
                }
                self.enemies.retain(|e| e.r > 0 || e.boss);
                if self.field.cleared() {
                    return Outcome::Cleared;
                }
            }
            _ => {}
        }
        Outcome::Playing
    }

    /// One call of the round's enemy logic (+E8FA; round 1's handler +999A for every round so far), then contact:
    /// an enemy on the open cut, or touching the player while the shield is off (+F4D6), costs a life.
    pub fn enemies_step(&mut self) -> Outcome {
        let p = &self.player;
        let (px, py, shield, cutting) = (p.x, p.y, p.shield_on, p.cutting());
        let mut sparks = Vec::new();
        for e in self.enemies.iter_mut() {
            match e.kind {
                Kind::Guard => guard_step(e, &self.field, px, py, shield),
                Kind::Boss => boss_step(e, &self.field, px, py, &mut sparks),
                Kind::Spark => spark_step(e, &self.field),
            }
        }
        self.enemies.retain(|e| e.r > 0 || e.boss);
        self.enemies.extend(sparks);
        let f = &self.field;
        for e in &self.enemies {
            let r = e.r.max(1);
            let hit_cut = cutting
                && (-r..=r).any(|dy| (-r..=r).any(|dx| f.get(e.x + dx, e.y + dy) == Cell::Cut));
            if hit_cut || touches_player(e.x, e.y, px, py, shield) {
                return Outcome::LifeLost;
            }
        }
        Outcome::Playing
    }

    /// A life lost: the cut goes and the player is back where it started.
    pub fn lose_life(&mut self) {
        if let Some(&(x, y)) = self.player.cut.first() {
            self.player.x = x;
            self.player.y = y;
        }
        self.player.cut.clear();
        self.field.clear_cut();
        self.player.shield_on = self.player.shield > 0;
    }
}

fn dir_of(dx: i32, dy: i32) -> Option<Dir> {
    match (dx.signum(), dy.signum()) {
        (1, 0) => Some(Dir::Right),
        (-1, 0) => Some(Dir::Left),
        (0, 1) => Some(Dir::Down),
        (0, -1) => Some(Dir::Up),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_the_edge_and_cuts_a_claim() {
        let mut rng = Rng::new(1);
        let mut r = Round::new(0, &mut rng);
        r.enemies.retain(|e| e.boss);
        r.enemies[0].x = 250;
        // along the bottom edge to x = 40 (two calls per pixel without the speed power-up)
        while r.player.x > 40 {
            r.player_step(Some(Dir::Left), false, true);
        }
        assert_eq!(r.player.y, 198);
        // up through the field to the top edge with fire held
        let mut out = Outcome::Playing;
        for _ in 0..200 {
            out = r.player_step(Some(Dir::Up), true, true);
            if !r.player.cutting() {
                break;
            }
        }
        assert_eq!(out, Outcome::Playing);
        assert_eq!(r.player.y, 0);
        assert_eq!(r.field.get(20, 100), Cell::Claimed);
        assert_eq!(r.field.count, 40 * 197);
        assert!(r.player.shield_on);
    }

    #[test]
    fn backing_up_erases_the_cut() {
        let mut rng = Rng::new(1);
        let mut r = Round::new(0, &mut rng);
        for _ in 0..10 {
            r.player_step(Some(Dir::Up), true, true);
        }
        assert_eq!(r.player.cut.len(), 10);
        for _ in 0..10 {
            r.player_step(Some(Dir::Down), false, true);
        }
        assert!(!r.player.cutting());
        assert_eq!((r.player.x, r.player.y), START);
        assert_eq!(r.field.count, 0);
    }

    #[test]
    fn shield_counts_down_every_five_calls() {
        let mut rng = Rng::new(1);
        let mut r = Round::new(0, &mut rng);
        for _ in 0..50 {
            r.player_step(None, false, false);
        }
        assert_eq!(r.player.shield, 890);
    }
    #[test]
    fn enemies_stay_inside_the_field() {
        use crate::field::{FH, FW};
        let mut rng = Rng::new(7);
        let mut r = Round::new(0, &mut rng);
        let mut sparks = 0;
        for _ in 0..20_000 {
            assert_eq!(r.enemies_step(), Outcome::Playing);
            sparks = sparks.max(r.enemies.iter().filter(|e| e.kind == Kind::Spark).count());
            for e in &r.enemies {
                assert!(
                    (0..=FW).contains(&e.x) && (0..=FH).contains(&e.y),
                    "{:?} at {},{}",
                    e.kind,
                    e.x,
                    e.y
                );
            }
        }
        assert!(sparks > 0, "the boss launches sparks at the end of a phase");
    }
}
