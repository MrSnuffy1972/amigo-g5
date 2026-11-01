//! PGA Tour Golf's integer arithmetic, kept exactly: 16-bit values, 1.15 fractions and 32-bit products, with the
//! original's own rounding and saturation (the author's `mul_frac`, `div_int2frac`, `magxy`, `mul_div`), so a native
//! frame lands on the same pixels as the original's.

/// `mul_frac`: a times a 1.15 fraction (the high word of twice the 32-bit product).
pub fn mul_frac(a: i16, b: i16) -> i16 {
    hi2(a as i32 * b as i32)
}

/// The high word of twice a 32-bit value (how the original drops 15 fraction bits).
pub fn hi2(v: i32) -> i16 {
    (v.wrapping_mul(2) >> 16) as i16
}

/// `div_int2frac`: a / b as a 1.15 fraction, saturated to +-1 when |a| >= |b|.
pub fn div_int2frac(a: i16, b: i16) -> i16 {
    let (ua, ub) = ((a as i32).abs(), (b as i32).abs());
    if ua < ub {
        (((a as i32) << 15) / b as i32) as i16
    } else if (a ^ b) < 0 {
        i16::MIN
    } else {
        0x7fff
    }
}

/// `magxy`: the fast hypotenuse (two Newton-style correction terms), 0x7fff on overflow.
pub fn magxy(a: i16, b: i16) -> i16 {
    let mut d0 = a.unsigned_abs();
    let mut d1 = b.unsigned_abs();
    if (d0 as i16) < (d1 as i16) {
        std::mem::swap(&mut d0, &mut d1);
    }
    if d1 == 0 {
        return d0 as i16;
    }
    let d2 = d0;
    let mut q = ((d1 as u32 * d1 as u32) / d0 as u32) as u16;
    q >>= 1;
    d0 = d0.wrapping_add(q);
    if (d0 as i16) < 0 {
        return 0x7fff;
    }
    let r = ((q as u32 * q as u32) / d2 as u32) as u16;
    let t = r.wrapping_mul(3) >> 3;
    d0.wrapping_sub(t) as i16
}

/// `mul_div`: a * b / c in 16 bits; when the quotient does not fit, the original keeps the product's low word.
pub fn mul_div(a: i16, b: i16, c: i16) -> i16 {
    let p = a as i32 * b as i32;
    if c == 0 {
        return p as i16;
    }
    let q = p.wrapping_div(c as i32);
    if (-32768..=32767).contains(&q) {
        q as i16
    } else {
        p as i16
    }
}

/// A 16-bit signed division of a 32-bit product with the original's overflow result (`perspect`): on overflow it
/// gives about +-k with the quotient's sign.
pub fn div_sat(p: i32, z: i16, k: u16) -> i16 {
    let q = p.wrapping_div(z as i32);
    if (-32768..=32767).contains(&q) {
        return q as i16;
    }
    let hi = (p as u32 >> 16) as u16;
    let mut d1: u32 = ((k as u32) << 16) | hi as u32;
    let low = (d1 as u16) ^ (z as u16);
    d1 = (d1 & 0xffff_0000) | low as u32;
    if (low as i16) < 0 {
        d1 = d1.wrapping_neg();
    }
    (d1 >> 16) as i16
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fractions() {
        assert_eq!(mul_frac(380, 0x7fff), 379);
        assert_eq!(div_int2frac(1, 2), 0x4000);
        assert_eq!(div_int2frac(-5, 5), i16::MIN);
        assert_eq!(div_int2frac(5, 5), 0x7fff);
        assert_eq!(magxy(3, 4), 5);
        assert_eq!(magxy(0, -7), 7);
        assert_eq!(mul_div(100, 200, 50), 400);
        assert_eq!(div_sat(1 << 30, 1, 0x7f69), 0x7f69);
        assert!(div_sat(-(1 << 30), 1, 0x7f69) < -32000);
    }
}
