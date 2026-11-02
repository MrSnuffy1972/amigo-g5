//! Enemy motion shared by the per-round enemy handlers, translated from the helpers at +F182 .. +F7F2 of the
//! recovered code: the 32-way step patterns (+F0E6, +F122, +F162), the speed table (+F182, +F1AC, +F1EC), one move
//! (+F22C, +F250, +F686), the heading towards a point (+F430, +F496), the turn towards it (+F30C), the reflection
//! from the wall flags (+F296, +F2EA, +F2F2) and the contact box against the player (+F4D6).
//!
//! Directions are 0..31 clockwise on screen: 0 right, 8 down, 16 left, 24 up. Wall flags (object word +28): bit 0
//! left, bit 1 bottom, bit 2 top, bit 3 right.

/// +F122: for the angle within an octant (direction & 7) two 16-bit patterns (high word, low word). A move takes
/// the next n bits of each (n = pixels a step) from the object's bit phase; the one bits counted are the pixels moved
/// on that axis.
pub const PATTERNS: [u32; 8] = [
    0x0000_FFFF,
    0x1084_FFFF,
    0x2529_FEFF,
    0x365B_FF7F,
    0xEDDB_EDDB,
    0xFF7F_365B,
    0xFFF7_2529,
    0xFFFF_1084,
];

/// +F1AC: per speed index (object word +8, & 15) the frames until the next move and the bits taken a move.
const SPEED_A: [(u16, u16); 16] = [
    (255, 1),
    (7, 1),
    (6, 1),
    (5, 1),
    (4, 1),
    (3, 1),
    (2, 1),
    (1, 1),
    (1, 1),
    (1, 2),
    (1, 2),
    (1, 3),
    (1, 3),
    (1, 4),
    (1, 4),
    (1, 5),
];
/// +F1EC: used instead when +F1AC's bits equal the current ones, so indexes 8, 10, 12 and 14 alternate (1.5, 2.5,
/// 3.5 and 4.5 pixels a move on average).
const SPEED_B: [(u16, u16); 16] = [
    (255, 1),
    (7, 1),
    (6, 1),
    (5, 1),
    (4, 1),
    (3, 1),
    (2, 1),
    (1, 1),
    (1, 2),
    (1, 2),
    (1, 3),
    (1, 3),
    (1, 4),
    (1, 4),
    (1, 5),
    (1, 5),
];

/// +F2EA / +F714: per quadrant (direction & $18) the two sides the object faces; both hit at once is a corner.
pub const FACING: [u16; 4] = [0x0A, 0x03, 0x05, 0x0C];
/// +F2F2: per quadrant of (direction - 4) the flags that make the reflection vertical (direction negated).
const VERTICAL: [u16; 4] = [0x02, 0x06, 0x04, 0x06];
/// +F496: heading towards a point, by the signs of dy and dx and the ratio of |dy| to |dx|.
const TOWARDS: [u16; 16] = [0, 2, 4, 6, 14, 12, 10, 8, 30, 28, 26, 24, 16, 18, 20, 22];

/// The motion fields of an object record: direction (+6), speed index (+8), frames to the next move (+10), bits a
/// move (+12) and the bit phase (+18). +F5EA starts them at speed 7, delay 1, one bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Motion {
    pub dir: u16,
    pub speed: u16,
    pub delay: u16,
    pub bits: u16,
    pub phase: u16,
}

impl Default for Motion {
    fn default() -> Self {
        Motion {
            dir: 0,
            speed: 7,
            delay: 1,
            bits: 1,
            phase: 0,
        }
    }
}

/// +F250: the one bits among the next `n` bits of `pattern` rotated left by `phase`.
fn take(pattern: u16, phase: u16, n: u16) -> i32 {
    let p = pattern.rotate_left(u32::from(phase & 15));
    (0..n.min(16)).filter(|i| p & (0x8000 >> i) != 0).count() as i32
}

/// +F0E6 and +F22C: the (dx, dy) of one move of `n` bits in direction `dir` from bit phase `phase`.
pub fn delta(dir: u16, n: u16, phase: u16) -> (i32, i32) {
    let d = (dir & 31) as usize;
    let (h, l) = ((PATTERNS[d & 7] >> 16) as u16, PATTERNS[d & 7] as u16);
    let q = (d >> 3) & 3;
    // +F162: quadrants 0 and 2 swap the halves; the next two bits give the signs of y and x
    let (py, px) = if q == 0 || q == 2 { (h, l) } else { (l, h) };
    let (sy, sx) = match q {
        0 => (1, 1),
        1 => (1, -1),
        2 => (-1, -1),
        _ => (-1, 1),
    };
    (sx * take(px, phase, n), sy * take(py, phase, n))
}

impl Motion {
    /// +F686 (`wrap`: positions kept to 9 bits, the guards) or +F266 (the boss): count down to the next move, then
    /// reload the delay and bits from the speed table and move. Returns true when it moved.
    pub fn step(&mut self, x: &mut i32, y: &mut i32, wrap: bool) -> bool {
        self.delay = self.delay.wrapping_sub(1);
        if self.delay != 0 {
            return false;
        }
        let i = (self.speed & 15) as usize;
        let (d, b) = if SPEED_A[i].1 == self.bits {
            SPEED_B[i]
        } else {
            SPEED_A[i]
        };
        self.delay = d;
        self.bits = b;
        self.dir &= 31;
        let (dx, dy) = delta(self.dir, self.bits, self.phase);
        self.phase = self.phase.wrapping_add(self.bits);
        *x += dx;
        *y += dy;
        if wrap {
            *x &= 0x1FF;
            *y &= 0x1FF;
        }
        true
    }
}

/// +F430: the heading from (x, y) towards (tx, ty).
pub fn towards(x: i32, y: i32, tx: i32, ty: i32) -> u16 {
    let (dy, dx) = (ty - y, tx - x);
    let (ady, adx) = (dy.unsigned_abs(), dx.unsigned_abs());
    let base = (usize::from(dy < 0) * 2 + usize::from(dx < 0)) * 4;
    let k = if 2 * ady < adx {
        0
    } else if ady < adx {
        1
    } else if ady < 2 * adx {
        2
    } else {
        3
    };
    TOWARDS[base + k]
}

/// +F30C: the turn step (+1 or -1) that brings `dir` round towards `target` the short way.
pub fn turn_towards(dir: u16, target: u16) -> i16 {
    let diff = target.wrapping_sub(dir);
    if diff != 0 && (diff & 0x1C) >= 0x10 {
        -1
    } else {
        1
    }
}

/// +F296: the direction after hitting the walls in `flags`.
pub fn reflect(dir: u16, flags: u16) -> u16 {
    if flags == 0 {
        return dir & 31;
    }
    let q = ((dir & 0x18) >> 3) as usize;
    let d = if flags == FACING[q] {
        dir.wrapping_add(16)
    } else if flags & VERTICAL[((dir.wrapping_sub(4) & 0x18) >> 3) as usize] != 0 {
        dir.wrapping_neg()
    } else {
        16u16.wrapping_sub(dir)
    };
    d & 31
}

/// +F4D6: an object within 6 pixels of the player on both axes touches it; it only counts with the shield off.
pub fn touches_player(x: i32, y: i32, px: i32, py: i32, shield_on: bool) -> bool {
    !shield_on && (py - y).abs() < 6 && (px - x).abs() < 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis_directions_move_one_pixel_a_move() {
        for (dir, want) in [(0, (1, 0)), (8, (0, 1)), (16, (-1, 0)), (24, (0, -1))] {
            assert_eq!(delta(dir, 1, 0), want, "dir {dir}");
        }
    }

    #[test]
    fn diagonal_takes_twelve_of_sixteen_bits() {
        let (mut x, mut y) = (100, 100);
        let mut m = Motion {
            dir: 4,
            speed: 7,
            ..Motion::default()
        };
        for _ in 0..16 {
            assert!(m.step(&mut x, &mut y, true));
        }
        assert_eq!((x, y), (112, 112));
    }

    #[test]
    fn odd_speed_index_alternates_bits() {
        let (mut x, mut y) = (0, 0);
        let mut m = Motion {
            dir: 0,
            speed: 8,
            ..Motion::default()
        };
        let mut seen = Vec::new();
        for _ in 0..4 {
            m.step(&mut x, &mut y, true);
            seen.push(m.bits);
        }
        assert_eq!(seen, vec![2, 1, 2, 1]);
        assert_eq!(x, 6);
    }

    #[test]
    fn slow_speeds_wait_between_moves() {
        let (mut x, mut y) = (0, 0);
        let mut m = Motion {
            dir: 0,
            speed: 3,
            ..Motion::default()
        };
        let moves = (0..21).filter(|_| m.step(&mut x, &mut y, true)).count();
        assert_eq!(moves, 5); // the first after one frame, then one every five
    }

    #[test]
    fn reflections() {
        assert_eq!(reflect(2, 0x08), 14); // right wall: mirrored left-right
        assert_eq!(reflect(2, 0x02), 30); // bottom wall: mirrored up-down
        assert_eq!(reflect(2, 0x0A), 18); // the corner: straight back
        assert_eq!(reflect(20, 0x04), 12);
    }

    #[test]
    fn heading_and_turn() {
        assert_eq!(towards(0, 0, 50, 0), 0);
        assert_eq!(towards(0, 0, 0, 50), 6);
        assert_eq!(towards(0, 0, -50, -50), 20);
        assert_eq!(turn_towards(0, 4), 1);
        assert_eq!(turn_towards(0, 28), -1);
        assert!(touches_player(10, 10, 15, 5, false) && !touches_player(10, 10, 15, 5, true));
        assert!(!touches_player(10, 10, 16, 10, false));
    }
}
