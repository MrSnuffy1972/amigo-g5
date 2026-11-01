//! One round being played: the field, the player (the player record at BSS +0044 and its routine +643E), the cut
//! (the corner list at BSS +009C) and the enemies.
//!
//! Translated from the recovered code: the player's start, movement, cutting, backing up along the cut, closing a
//! claim, the shield and its count-down, the percentage and the round's end. NOT yet translated (stage 2 has not
//! read them): the 16 per-round enemy handlers (+999A .. +14658), the power-up boxes (+82F6) and the score for a
//! claim. Until they are, `Enemy` is a provisional stand-in (a bouncing boss and three bouncing guards) so the round
//! can be played; it is marked as such in docs/native/g1727-native.md and is not a verified behaviour.

use crate::field::{Cell, Field, FH, FW};
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

/// PROVISIONAL stand-in for a round's enemies (see the module comment).
#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub r: i32,
    pub boss: bool,
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
        let mut enemies = vec![Enemy {
            x: 143,
            y: 80,
            dx: 1,
            dy: 1,
            r: 10,
            boss: true,
        }];
        for i in 0..3 {
            let v = rng.step();
            enemies.push(Enemy {
                x: 40 + 100 * i,
                y: 30 + (v & 0x3F) as i32,
                dx: if v & 0x100 != 0 { 1 } else { -1 },
                dy: if v & 0x200 != 0 { 1 } else { -1 },
                r: 5,
                boss: false,
            });
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
                for e in self.enemies.iter_mut() {
                    if self.field.get(e.x, e.y) != Cell::Free {
                        e.r = 0; // trapped in the claimed area: gone
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

    /// PROVISIONAL enemy movement and contact (see the module comment).
    pub fn enemies_step(&mut self) -> Outcome {
        let f = &self.field;
        for e in self.enemies.iter_mut() {
            let nx = e.x + e.dx;
            if f.get(nx, e.y) != Cell::Free && f.get(nx, e.y) != Cell::Cut {
                e.dx = -e.dx;
            } else {
                e.x = nx;
            }
            let ny = e.y + e.dy;
            if f.get(e.x, ny) != Cell::Free && f.get(e.x, ny) != Cell::Cut {
                e.dy = -e.dy;
            } else {
                e.y = ny;
            }
            e.x = e.x.clamp(1, FW - 2);
            e.y = e.y.clamp(1, FH - 2);
        }
        let p = &self.player;
        for e in &self.enemies {
            let r = e.r.max(1);
            let hit_cut = p.cutting()
                && (-r..=r).any(|dy| (-r..=r).any(|dx| f.get(e.x + dx, e.y + dy) == Cell::Cut));
            let hit_player =
                (e.x - p.x).abs() <= r && (e.y - p.y).abs() <= r && (p.cutting() || !p.shield_on);
            if hit_cut || hit_player {
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
}
