//! The background's own life (title_logo_frame, $18642, every frame): the 21 running values the original keeps at
//! d0_18269 (also a source of randomness for the bonus round) and the code-word extras. The sea shimmer and the
//! background creatures are not drawn in this version.

use crate::ik::Ik;

#[derive(Clone)]
pub struct Ambient {
    /// d0_18269: 21 values stepped every other frame.
    pub v: [u8; 21],
    /// The code words' extras (d0_18312, d0_18314, d0_18316, d0_18318).
    pub cheat: [bool; 4],
    step: [u8; 21],
}

impl Default for Ambient {
    fn default() -> Self {
        // clear_table_entries ($1841E): the values start from d0_18404
        let init = [
            0x00, 0x09, 0x14, 0x1e, 0x06, 0x0f, 0x16, 0x1d, 0x04, 0x0a, 0x10, 0x16, 0x1b, 0x00,
            0x04, 0x09, 0x0d, 0x11, 0x15, 0x19, 0x1c,
        ];
        Ambient {
            v: init,
            cheat: [false; 4],
            step: [1; 21],
        }
    }
}

/// title_logo_frame ($18642): steps the running values (decrypt_global_bytes on every other frame).
pub fn title_logo_frame(g: &mut Ik) {
    if g.vbl & 1 == 0 {
        let a = &mut g.ambient;
        for k in 0..21 {
            a.v[k] = (a.v[k].wrapping_add(a.step[k])) & 0x1f;
        }
    }
}
