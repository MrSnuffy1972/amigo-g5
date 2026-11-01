//! The arithmetic the game's physics is built on, with the original's exact results: 16-bit multiplies and divides
//! (a divide that would overflow, or divide by zero, leaves its operand as it was, as the original's divides did with
//! its divide-by-zero vector pointing at a plain return), the integer square root, and the random numbers.

/// 16 x 16 signed multiply of the low halves.
pub fn muls(a: i32, b: i32) -> i32 {
    (a as i16 as i32) * (b as i16 as i32)
}

/// 16 x 16 unsigned multiply of the low halves.
pub fn mulu(a: i32, b: i32) -> u32 {
    (a as u16 as u32) * (b as u16 as u32)
}

/// Unsigned 32 / 16 divide: remainder in the high half, quotient in the low half, or `n` unchanged on overflow.
pub fn divu(n: u32, d: u16) -> u32 {
    if d == 0 {
        return n;
    }
    let q = n / d as u32;
    if q > 0xFFFF {
        return n;
    }
    (n % d as u32) << 16 | q
}

/// Signed 32 / 16 divide (quotient towards zero), as `divu`.
pub fn divs(n: i32, d: i16) -> i32 {
    if d == 0 {
        return n;
    }
    let q = n as i64 / d as i64;
    if !(-32768..=32767).contains(&q) {
        return n;
    }
    let r = n as i64 % d as i64;
    ((r as u16 as u32) << 16 | q as u16 as u32) as i32
}

/// The low half of a divide's result, as the code after it read it.
pub fn lo(v: i32) -> i32 {
    v as i16 as i32
}

/// The integer square root the collisions use (+$54C0), bit for bit.
pub fn isqrt(v: u32) -> u16 {
    let mut d0 = v;
    let mut d2: u32 = 0x4000_0000;
    let mut d1: u32 = d0 >> 16;
    for _ in 0..16 {
        d2 = d2.rotate_left(16);
        let (r, borrow) = d1.overflowing_sub(d2);
        d2 = d2.rotate_left(16);
        let w = d2 as u16;
        let w = if borrow {
            w.wrapping_add(w)
        } else {
            d1 = r;
            w.wrapping_add(w).wrapping_add(1)
        };
        d2 = (d2 & 0xFFFF_0000) | w as u32;
        for _ in 0..2 {
            let x = (d0 as u16 >> 15) as u32;
            d0 = (d0 & 0xFFFF_0000) | ((d0 as u16) << 1) as u32;
            d1 = d1.wrapping_shl(1) | x;
        }
    }
    d2 as u16
}

/// random_next (+$48AE): two 32-bit words added into each other with carry, half a word at a time.
pub fn random_next(s: &mut [u32; 2]) -> u32 {
    let (mut d0, mut d1) = (s[0], s[1]);
    // add.w d0,d0
    let w0 = d0 as u16;
    let (dbl, mut x) = w0.overflowing_add(w0);
    d0 = (d0 & 0xFFFF_0000) | dbl as u32;
    let d2 = d0 as u16;
    // addx.w d1,d0
    let (t, c1) = (d0 as u16).overflowing_add(d1 as u16);
    let (t, c2) = t.overflowing_add(x as u16);
    x = c1 || c2;
    d0 = (d0 & 0xFFFF_0000) | t as u32;
    d1 = (d1 & 0xFFFF_0000) | d2 as u32;
    d0 = d0.rotate_left(16);
    d1 = d1.rotate_left(16);
    let d2 = d0 as u16;
    let (t, c1) = (d0 as u16).overflowing_add(d1 as u16);
    let (t, _) = t.overflowing_add(x as u16);
    let _ = c1;
    d0 = (d0 & 0xFFFF_0000) | t as u32;
    d1 = (d1 & 0xFFFF_0000) | d2 as u32;
    d0 = d0.rotate_left(16);
    d1 = d1.rotate_left(16);
    s[0] = d0;
    s[1] = d1;
    d0
}

/// random_below (+$489A): a random number below `n` (the whole 32-bit number modulo `n`).
pub fn random_below(s: &mut [u32; 2], n: u16) -> u16 {
    (random_next(s) % n as u32) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_roots() {
        for v in [0u32, 1, 4, 99, 100, 1024, 4096, 65535, 1_000_000] {
            let r = isqrt(v) as u32;
            assert!(r * r <= v && (r + 1) * (r + 1) > v, "{v} -> {r}");
        }
    }

    #[test]
    fn divides_keep_the_operand_on_overflow() {
        assert_eq!(divu(0x0010_0000, 1), 0x0010_0000);
        assert_eq!(divu(100, 7), 2 << 16 | 14);
        assert_eq!(
            divs(-100, 7) as u32,
            ((-2i16 as u16 as u32) << 16) | (-14i16 as u16 as u32)
        );
        assert_eq!(divs(5, 0), 5);
    }
}
