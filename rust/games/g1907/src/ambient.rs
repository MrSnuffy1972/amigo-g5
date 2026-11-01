//! The background's own life: title_logo_frame ($18642), which the frame interrupt runs every frame outside the
//! title. The sea's 21 running values and the shimmer strip they choose (decrypt_global_bytes,
//! initialize_sprite_tables_2), the sun's reflection on the water (write_masked_bytes_to_table) and its sparkles
//! (self_7), and the background extras (rotate_and_copy_sprite_words: a state machine of eight states, one creature
//! at a time, chosen at random or by the code words). Everything is drawn into the shown screen or into both, as the
//! original drew it; the pictures come from the three sheets, and the tables and the scenery mask from the pack table
//! `extras` (the original's image from unit $18240 on). The running values are also read by the bonus round.

use crate::assets::{Assets, Pic, H, W};
use crate::ik::{Bob, Ik, Src};

/// Unit offset of the first byte of the pack table `extras`.
const BASE: usize = 0x18240;
/// conditionally_recurse_and_clear_flag+36: the scenery mask, one bit a pixel, 40 bytes a row, from screen row 16;
/// mask pixel = screen pixel + 32 (the original's 32-pixel window starts 4 bytes before the creature's word).
const MASK: usize = 0x19D24;

/// The table reader (big-endian, as the original's words and longs).
struct T<'a>(&'a [u8]);

impl T<'_> {
    fn b(&self, u: usize) -> u8 {
        u.checked_sub(BASE)
            .and_then(|i| self.0.get(i))
            .copied()
            .unwrap_or(0)
    }
    fn w(&self, u: usize) -> u16 {
        (self.b(u) as u16) << 8 | self.b(u + 1) as u16
    }
    fn l(&self, u: usize) -> u32 {
        (self.w(u) as u32) << 16 | self.w(u + 2) as u32
    }
    /// The scenery mask at a screen pixel: the creature shows there (behind the rocks and the trees it does not).
    fn mask(&self, px: i32, py: i32) -> bool {
        if py < 16 || px < -32 {
            return false;
        }
        let m = px + 32;
        let o = MASK + (py as usize - 16) * 40 + m as usize / 8;
        self.b(o) >> (7 - (m & 7)) & 1 != 0
    }
}

/// Where to put the background back next time: 32 pixels (the original's two words) by `h` rows of screen `buf`
/// (2: both screens), at the window a creature was drawn in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Patch {
    x: i32,
    y: i32,
    h: i32,
    buf: usize,
}

/// The state of title_logo_frame's routines (the original's variables at d0_1824D..d0_18334 and d0_195CC..d0_19618).
#[derive(Clone, Debug)]
pub struct Ambient {
    /// d0_18269: the 21 running values (stepped every other frame; also read by the bonus round).
    pub v: [u8; 21],
    /// d0_1827E: each value's step (31, that is one down, from clear_table_entries).
    step: [u8; 21],
    /// d0_18293 and d0_182A8: two bit patterns a value chose (read as one table of 42).
    tone: [u8; 42],
    /// d0_182E8: each value's row of the shimmer strip (16 bytes a row of d0_185B2).
    row: [u16; 21],
    /// d0_182BE..d0_182CC: where the eight sparkles are (byte offsets into the screen).
    spark: [u16; 8],
    /// The code words' extras: d0_18312 (state 6), d0_18314 (state 3), d0_18316 (state 1), d0_18318 (state 7).
    pub cheat: [bool; 4],
    /// d0_1824D, d0_1824E, d0_1824F: the slow counters that nudge the running values every 256 steps.
    c4d: u8,
    c4e: u8,
    c4f: u8,
    /// d0_18250: the extras' state (0 choose, 1..7 a creature).
    pub state: u8,
    /// d0_18254: a frame count (most creatures move every fourth frame).
    tick: u8,
    /// d0_18251 and d0_18252: the creature's row and its x in two-pixel units.
    y: u8,
    x: u8,
    /// d0_18253: its animation step; d0_18258: its direction; d0_18259: its colour (0 black, else 1).
    step_n: u8,
    dir: u8,
    colour: u8,
    /// d0_18260, d0_18261, d0_18262: the rotating choice of state 3, the kind (0..7) and the number drawn.
    rot: u8,
    kind: u8,
    pick: u8,
    /// d0_18263: repeats left; d0_18264: creatures so far; d0_18265: the shape set of state 1.
    repeat: u8,
    count: u8,
    shape: u8,
    /// d0_18266, d0_18267: state 4's frame timer and rate; d0_18268: a creature just chosen.
    sub: u8,
    rate: u8,
    fresh: u8,
    /// d0_1825A, d0_1825D: set when a creature is chosen (kept as the original kept them).
    c5a: u8,
    c5d: u8,
    /// d0_1831A: state 6's route (0..3); d0_1831C..d0_18326: state 7's script, place in it, pause, frame, legs.
    route: u16,
    script: u16,
    pos: u16,
    pause: u16,
    frame7: u16,
    legs: u16,
    legs_t: i16,
    /// d0_18328, d0_18330, d0_1832C, d0_195CC: the patches of states 1, 3, 4 and 5 to 7.
    r1: Option<Patch>,
    r3: Option<Patch>,
    r4: Option<Patch>,
    r5: Option<Patch>,
    /// d0_195D0, d0_195D2: the big creatures' position; d0_195D4: just started; d0_195D6: their animation.
    bx: i16,
    by: i16,
    init: bool,
    anim: u16,
    /// d0_195D8 and d0_19618: the last 16 landed drops of state 5 and the next slot.
    trail: [Option<Patch>; 16],
    trail_i: u16,
}

impl Default for Ambient {
    /// reset_game_state_2 ($1834C) with clear_table_entries ($1841E).
    fn default() -> Self {
        let init = [
            0x00, 0x09, 0x14, 0x1e, 0x06, 0x0f, 0x16, 0x1d, 0x04, 0x0a, 0x10, 0x16, 0x1b, 0x00,
            0x04, 0x09, 0x0d, 0x11, 0x15, 0x19, 0x1c,
        ];
        let mut spark = [0; 8];
        spark[6] = 0x20;
        spark[7] = 0x20;
        Ambient {
            v: init,
            step: [0x1f; 21],
            tone: [0; 42],
            row: [0; 21],
            spark,
            cheat: [false; 4],
            c4d: 0,
            c4e: 0,
            c4f: 0,
            state: 0,
            tick: 0,
            y: 0,
            x: 0,
            step_n: 0,
            dir: 0,
            colour: 0,
            rot: 0,
            kind: 0,
            pick: 0,
            repeat: 0,
            count: 0,
            shape: 0,
            sub: 0,
            rate: 0,
            fresh: 0,
            c5a: 0,
            c5d: 0,
            route: 0,
            script: 0,
            pos: 0,
            pause: 0,
            frame7: 0,
            legs: 0,
            legs_t: 0,
            r1: None,
            r3: None,
            r4: None,
            r5: None,
            bx: 0,
            by: 0,
            init: false,
            anim: 0,
            trail: [None; 16],
            trail_i: 0,
        }
    }
}

// ---- drawing: the original's plane operations as colour-index bits

fn bufs(b: usize) -> std::ops::Range<usize> {
    if b >= 2 {
        0..2
    } else {
        b..b + 1
    }
}

/// copy_struct_array ($19592) and copy_sprite_blocks ($18EEA): the background back over a patch.
fn restore(g: &mut Ik, p: Patch) {
    restore_rect(g, p.buf, p.x, p.y, 32, p.h);
}

fn restore_rect(g: &mut Ik, buf: usize, x: i32, y: i32, w: i32, h: i32) {
    for b in bufs(buf) {
        for yy in y.max(0)..(y + h).min(H as i32) {
            for xx in x.max(0)..(x + w).min(W as i32) {
                let o = yy as usize * W + xx as usize;
                g.scr[b][o] = g.a.background.px[o];
            }
        }
    }
}

/// One plane's byte at a screen byte offset (the original's interleaved rows: 160 bytes, 40 a plane) set by `f`.
fn plane_byte(g: &mut Ik, buf: usize, off: i32, f: impl Fn(u8) -> u8) {
    if off < 0 {
        return;
    }
    let off = off as usize;
    let (row, b) = (off / 160, off % 160);
    if row >= H {
        return;
    }
    let (plane, x0) = (b / 40, (b % 40) * 8);
    let px = &mut g.scr[buf][row * W + x0..row * W + x0 + 8];
    let old = px.iter().fold(0u8, |a, &p| a << 1 | (p >> plane & 1));
    let new = f(old);
    for (k, p) in px.iter_mut().enumerate() {
        let bit = new >> (7 - k) & 1;
        *p = (*p & !(1 << plane)) | bit << plane;
    }
}

fn pic(a: &Assets, s: Src) -> &Pic {
    match s {
        Src::Sheet1 => &a.sheet1,
        Src::Sheet2 => &a.sheet2,
        Src::Back => &a.background,
    }
}

/// self_8 ($1B512, x in two-pixel units, `shift` = twice its low three bits) and compute_sprite_tile_offset
/// ($1B4FE, x in pixels): a 16-pixel column of a sheet, `rows` rows, colour 0 clear, into screen `buf`; `masked`
/// = behind the scenery (a5 = 40), else only the screen's edges clip. Returns the 32-pixel window drawn in (the
/// original's d0_18334), or None when nothing was drawn.
#[allow(clippy::too_many_arguments)]
fn bob_at(
    g: &mut Ik,
    t: &T,
    buf: usize,
    src: Bob,
    x2: u32,
    shift: i32,
    y: i32,
    rows: i32,
    masked: bool,
) -> Option<(i32, i32)> {
    let y = y & 0xfff;
    if y >= 200 {
        return None;
    }
    let rows = if y + rows >= 201 { 200 - y } else { rows };
    if !masked && !(8..0xb0).contains(&x2) {
        return None;
    }
    let w0 = (x2 & 0xf8) as i32 * 2 - 32;
    let sx = w0 + shift;
    let a = g.a.clone();
    let p = pic(&a, src.src);
    let lw = g.lw_o;
    for r in 0..rows {
        let (sy, py) = (src.y + r, y + r);
        if sy < 0 || sy as usize >= p.h {
            continue;
        }
        for c in 0..16 {
            let px = sx + c;
            let qx = src.x + c;
            if !(0..W as i32).contains(&px) || qx < 0 || qx as usize >= p.w {
                continue;
            }
            let v = p.at(qx as usize, sy as usize);
            if v != 0 && (!masked || t.mask(px, py)) {
                let o = py as usize * W + px as usize;
                g.scr[buf][o] = v;
                if g.obj_on {
                    g.layer[lw][o] = 1;
                }
            }
        }
    }
    Some((w0, y))
}

#[allow(clippy::too_many_arguments)]
fn bob8(
    g: &mut Ik,
    t: &T,
    buf: usize,
    src: Bob,
    x: u16,
    y: i16,
    rows: i32,
    masked: bool,
) -> Option<(i32, i32)> {
    let x2 = (x & 0xff) as u32;
    bob_at(
        g,
        t,
        buf,
        src,
        x2,
        ((x2 & 7) * 2) as i32,
        y as i32,
        rows,
        masked,
    )
}

#[allow(clippy::too_many_arguments)]
fn bobp(
    g: &mut Ik,
    t: &T,
    buf: usize,
    src: Bob,
    x: i16,
    y: i16,
    rows: i32,
    masked: bool,
) -> Option<(i32, i32)> {
    let x = (x as u16 & 0x1ff) as u32;
    bob_at(
        g,
        t,
        buf,
        src,
        x >> 1,
        (x & 15) as i32,
        y as i32,
        rows,
        masked,
    )
}

fn patch(w: Option<(i32, i32)>, h: i32, buf: usize) -> Option<Patch> {
    w.map(|(x, y)| Patch { x, y, h, buf })
}

// ---- the frame's routines

/// title_logo_frame ($18642): the extras' patches put back (reset_flag_and_recurse, reset_one_shot_flag,
/// conditionally_recurse_and_clear_flag), the running values stepped every other frame, the reflection, the
/// sparkles, the shimmer strip, then one step of the extras.
pub fn title_logo_frame(g: &mut Ik) {
    let a = g.a.clone();
    let Some(tab) = a.tables.get("extras") else {
        decrypt_values(g, None);
        return;
    };
    let t = T(tab);
    let s = g.ambient.state;
    if s == 4 {
        if let Some(p) = g.ambient.r4.take() {
            restore(g, p);
        }
    }
    if s == 3 {
        if let Some(p) = g.ambient.r3.take() {
            restore(g, p);
        }
    }
    if s == 7 {
        if let Some(p) = g.ambient.r5.take() {
            restore(g, p);
        }
    }
    if g.vbl & 1 == 0 {
        decrypt_values(g, Some(&t));
    }
    write_masked_bytes_to_table(g, &t);
    self_7(g, &t);
    initialize_sprite_tables_2(g, &t);
    rotate_and_copy_sprite_words(g, &t);
}

/// decrypt_global_bytes ($1867E) and initialize_global_table ($18BA0). Without the pack table (an older pack) only
/// the running values step.
fn decrypt_values(g: &mut Ik, t: Option<&T>) {
    let a = &mut g.ambient;
    a.c4e = a.c4e.wrapping_add(1);
    for k in 0..21 {
        let v = a.v[k].wrapping_add(a.step[k]) & 0x1f;
        a.v[k] = v;
        if let Some(t) = t {
            let d2 = t.b(0x18460 + v as usize).wrapping_add(t.b(0x18492 + k)) as usize;
            let d0 = t.b(0x18460 + 74 + d2) as usize;
            a.tone[k] = t.b(0x18460 + 32 + d0);
            a.tone[21 + k] = t.b(0x18460 + 41 + d0);
            a.row[k] = (d0 as u16) << 4;
        }
    }
    if a.c4e == 0 {
        if let Some(t) = t {
            a.c4f = a.c4f.wrapping_add(1);
            a.c4d = a.c4f >> 4;
            for d1 in (0..21).rev() {
                let i = ((d1 & 3) as u8).wrapping_add(a.c4d) & 0xf;
                a.v[d1] = t.b(0x18B90 + i as usize).wrapping_add(a.v[d1]) & 0x1f;
            }
        }
    }
}

/// initialize_sprite_tables_2 ($186E2): the shimmer strip on the sea, 64 pixels by 21 rows from (128, 78) of the
/// shown screen, each row the strip pattern its running value chose; then the notch at (144, 98).
fn initialize_sprite_tables_2(g: &mut Ik, t: &T) {
    let b = g.shown;
    for k in 0..21 {
        let e = 0x185B2 + g.ambient.row[k] as usize;
        let o = (78 + k) * W + 128;
        for px in 0..64 {
            let bit = 7 - (px & 7);
            let p2 = t.b(e + px / 8) >> bit & 1;
            let p3 = t.b(e + 8 + px / 8) >> bit & 1;
            g.scr[b][o + px] = 2 | p2 << 2 | p3 << 3;
        }
    }
    let m: u32 = 0xc00f_ffc3;
    for k in 0..32 {
        if m >> (31 - k) & 1 == 0 {
            g.scr[b][98 * W + 144 + k] = 0;
        }
    }
}

/// write_masked_bytes_to_table ($187C2): the sun's reflection, 18 bytes of the shown screen set from the patterns.
fn write_masked_bytes_to_table(g: &mut Ik, t: &T) {
    let b = g.shown;
    for i in 0..18 {
        let off = t.l(0x18756 + 4 * i) as i32;
        let d0 = g.ambient.tone[t.w(0x1879E + 2 * i) as usize % 42];
        plane_byte(g, b, off, |p| p & !d0);
        plane_byte(g, b, off + 40, |_| d0);
        plane_byte(g, b, off + 80, |_| d0);
        plane_byte(g, b, off + 120, |p| p & !d0);
    }
}

/// self_7 ($18944): eight sparkles on the water, each a byte of one plane in both screens (the first four mirrored
/// about a point); a sparkle whose pattern comes out 0 moves to a new place from its table.
fn self_7(g: &mut Ik, t: &T) {
    let tn = g.ambient.tone;
    let v = g.ambient.v;
    let vbl = g.vbl;
    let c4e = g.ambient.c4e;
    let mirrored = [
        (19, 41, 0u8, 0),
        (17, 39, 0xc3, 1),
        (38, 40, 0, 2),
        (6, 32, 0, 3),
    ];
    for (k, &(i, j, e, vi)) in mirrored.iter().enumerate() {
        let d0 = g.ambient.spark[k] as i16 as i32;
        let d2 = tn[i] ^ e ^ tn[j];
        for b in 0..2 {
            plane_byte(g, b, 0x32BA + d0, |_| d2);
            plane_byte(g, b, 0x3DED - d0, |_| d2);
        }
        if d2 == 0 {
            let n = (vbl.wrapping_add(v[vi]) & 0x3f) as usize;
            g.ambient.spark[k] = t.w(0x18804 + 2 * n);
        }
    }
    let rest: [(usize, u8, i32, usize, usize); 4] = [
        (4, tn[15] ^ tn[16], 0x30D6, 0x18884, {
            ((vbl.rotate_left(3) ^ v[5]).wrapping_add(v[6]) & 0x1f) as usize
        }),
        (5, tn[16] ^ tn[17], 0x30D6, 0x18884, {
            ((vbl.rotate_left(6) ^ v[3]).wrapping_add(v[5]) & 0x1f) as usize
        }),
        (6, (tn[16] ^ tn[37]) & 0xdf, 0x30C0, 0x188C4, {
            (vbl.wrapping_sub(c4e).wrapping_add(v[4]) & 0x3f) as usize
        }),
        (7, tn[17] ^ tn[40], 0x30C0, 0x188C4, {
            (vbl.wrapping_add(c4e).wrapping_add(v[2]) & 0x3f) as usize
        }),
    ];
    for (k, d2, at, table, n) in rest {
        let d0 = g.ambient.spark[k] as i16 as i32;
        for b in 0..2 {
            plane_byte(g, b, at + d0, |_| d2);
        }
        if d2 == 0 {
            g.ambient.spark[k] = t.w(table + 2 * n);
        }
    }
}

// ---- the extras (rotate_and_copy_sprite_words, $18C14: a state a frame, through the table at d0_18BF4)

fn rotate_and_copy_sprite_words(g: &mut Ik, t: &T) {
    g.ambient.tick = g.ambient.tick.wrapping_add(1);
    match g.ambient.state & 7 {
        0 => decode_input_byte(g, t),
        1 => increment_global_index(g, t),
        2 => copy_opponent_sprite_data(g, t),
        3 => title_object_grow_step(g, t),
        4 => process_global_state(g, t),
        5 => reset_animation_state_2(g, t),
        6 => reset_and_poll_input_state(g, t),
        _ => reset_and_update_state(g, t),
    }
}

/// decode_input_byte ($18C2C), state 0: choose the next creature (not in the bonus rounds or under the top-scores
/// sign). A random draw every frame; twice in 256 frames a second draw picks: below 128 one of the small creatures
/// (states 1 to 4, kind and place from d0_18D6E), else one of the big ones (l0_1961A).
fn decode_input_byte(g: &mut Ik, t: &T) {
    g.ambient.state = 0;
    if g.phase == 5 || g.phase == 7 || g.sign.2 != 0 {
        return;
    }
    if g.ambient.cheat[0] || g.ambient.cheat[3] {
        return choose_big(g, t);
    }
    let mut d0: u8 = 0x18;
    if !g.ambient.cheat[1] {
        d0 = g.random() & 7;
        if !g.ambient.cheat[2] {
            if g.ambient.tick & 0xfd != 0 {
                return;
            }
            let r = g.random();
            if r >= 0x80 {
                return choose_big(g, t);
            }
            d0 = r & 0x1f;
        }
    }
    let a = &mut g.ambient;
    a.fresh = 1;
    a.pick = d0;
    a.count = a.count.wrapping_add(1);
    a.kind = d0 & 7;
    let i = (d0 >> 3) as usize;
    a.c5d = t.b(0x18D6A + i);
    a.state = t.b(0x18D66 + i);
    let mut d2 = d0;
    if a.state == 3 {
        d2 = a.rot.wrapping_add(0x10);
        a.kind = d2 & 7;
        a.rot = a.rot.wrapping_add(1) & 7;
    }
    let d2 = d2 as usize;
    a.x = t.b(0x18D6A + 12 + d2);
    a.y = t.b(0x18D6A + 44 + d2);
    a.colour = t.b(0x18D6A + 76 + d2);
    a.c5a = 0;
    a.dir = 0;
    a.step_n = 0;
    a.repeat = 1;
    a.shape = 0;
    let r = g.random();
    if r & 0x40 == 0 {
        g.ambient.shape = (r & 3) * 0x28;
    }
}

/// l0_1961A: a big creature: state 6 or 7 by their code words, else at random from d0_19674 (mostly 5).
fn choose_big(g: &mut Ik, t: &T) {
    g.ambient.r5 = None;
    if g.ambient.cheat[0] {
        g.ambient.cheat[0] = false;
        g.ambient.state = 6;
    } else if g.ambient.cheat[3] {
        g.ambient.cheat[3] = false;
        g.ambient.state = 7;
    } else {
        let r = (g.random() & 0xf) as usize;
        g.ambient.state = t.b(0x19674 + r);
    }
    g.ambient.init = true;
}

/// increment_global_index ($18DE6), state 1: a small bird crossing (every fourth frame a step left or right by its
/// kind, until the end place of its kind), drawn by mask_sprite_planes.
fn increment_global_index(g: &mut Ik, t: &T) {
    if g.ambient.tick & 3 != 0 {
        return;
    }
    let k = g.ambient.kind as usize;
    let a = &mut g.ambient;
    a.x = if t.b(0x18DD6 + k) != 0 {
        a.x.wrapping_sub(1)
    } else {
        a.x.wrapping_add(1)
    };
    if a.x == t.b(0x18DD6 + 8 + k) {
        a.state = 0;
        a.cheat[2] = false;
        if let Some(p) = a.r1 {
            restore(g, p);
        }
        return;
    }
    mask_sprite_planes(g, t);
}

/// mask_sprite_planes ($18F4E): last frame's place put back in both screens, then the bird's five rows (one of four
/// frames by its x, from the shape set) in colour 0 or 1, behind the scenery, into both screens.
fn mask_sprite_planes(g: &mut Ik, t: &T) {
    if let Some(p) = g.ambient.r1 {
        restore(g, p);
    }
    let (x, y) = (g.ambient.x, g.ambient.y as i32);
    let w0 = (x & 0xf8) as i32 * 2 - 32;
    g.ambient.r1 = Some(Patch {
        x: w0,
        y,
        h: 5,
        buf: 2,
    });
    if !(8..0xb0).contains(&x) {
        return;
    }
    let colour = (g.ambient.colour != 0) as u8;
    let base = 0x18E4A + (x & 3) as usize * 10 + g.ambient.shape as usize;
    let sx = 2 * x as i32 - 32;
    for r in 0..5 {
        let s = t.w(base + 2 * r);
        let py = y + r as i32;
        if !(0..H as i32).contains(&py) {
            continue;
        }
        for c in 0..16 {
            let px = sx + c;
            if s >> (15 - c) & 1 != 0 && (0..W as i32).contains(&px) && t.mask(px, py) {
                let o = py as usize * W + px as usize;
                g.scr[0][o] = colour;
                g.scr[1][o] = colour;
            }
        }
    }
}

/// copy_opponent_sprite_data ($190A2), state 2: a fish jumping out of the sea and falling back (every fourth frame
/// a step up or down; it turns when the running value 17 is low, or at the height of its kind), seven rows a word
/// wide drawn over the background into both screens.
fn copy_opponent_sprite_data(g: &mut Ik, t: &T) {
    if g.ambient.tick & 3 != 0 {
        return;
    }
    if g.vbl & 0xc0 == 0 {
        if (g.ambient.v[17] as i8) < 0x0f {
            g.ambient.dir ^= 1;
        }
    } else if g.ambient.dir & 1 != 0 {
        g.ambient.step_n = g.ambient.step_n.wrapping_sub(1);
        if (g.ambient.step_n as i8) < 0 {
            g.ambient.state = 0;
            return;
        }
    } else {
        g.ambient.step_n = g.ambient.step_n.wrapping_add(1);
        let top = t.b(0x18D6E + g.ambient.kind as usize) as i8;
        if g.ambient.step_n as i8 >= top {
            g.ambient.dir = 1;
        }
    }
    let n = g.ambient.step_n as usize;
    let d0 = ((((n & 1) ^ 1) << 5) as u16).wrapping_add(t.w(0x1925A + 82 + 2 * n)) as usize;
    let rows_down = t.w(0x1925A + 2 * n) as i32 / 160;
    let x = g.ambient.x;
    let w0 = (x & 0xf8) as i32 * 2 - 16;
    let s = ((x & 7) * 2) as u32;
    let y0 = g.ambient.y as i32 + rows_down;
    let colour = (g.ambient.colour != 0) as u8;
    for r in 0..7 {
        let sh = t.w(0x1921A + d0 + 2 * r);
        let py = y0 + r as i32;
        if sh == 0 {
            restore_rect(g, 2, w0, py, 16, 1);
            continue;
        }
        if !(0..H as i32).contains(&py) {
            continue;
        }
        let m = sh >> s;
        for c in 0..16 {
            let px = w0 + c;
            if !(0..W as i32).contains(&px) {
                continue;
            }
            let o = py as usize * W + px as usize;
            let v = if m >> (15 - c) & 1 != 0 {
                colour
            } else {
                g.a.background.px[o]
            };
            g.scr[0][o] = v;
            g.scr[1][o] = v;
        }
    }
}

/// title_object_grow_step ($1932C), state 3: a slow thing on the horizon (every 48 frames 6 pixels on, to the end of
/// its kind), four rows behind the scenery in the shown screen; reset_one_shot_flag puts it back next frame.
fn title_object_grow_step(g: &mut Ik, t: &T) {
    if g.ambient.tick & 3 == 0 {
        let a = &mut g.ambient;
        a.step_n = a.step_n.wrapping_add(1);
        if a.step_n as i8 >= 0x0c {
            a.x = a.x.wrapping_add(3);
            a.step_n = 0;
        }
        if a.x >= t.b(0x193CC + a.kind as usize) {
            a.state = 0;
            return;
        }
    }
    let src = Bob::at(0x1D7BC + 0x600 + 2 * g.ambient.step_n as u32);
    let b = g.shown;
    let (x, y) = (g.ambient.x as u16, g.ambient.y as i16);
    let w = bob8(g, t, b, src, x, y, 4, true);
    g.ambient.r3 = patch(w, 5, b);
}

/// process_global_state ($19404), state 4: a creature that walks along in hops (place, direction and repeats at
/// random from d0_193D4), fourteen rows behind the scenery in the shown screen.
fn process_global_state(g: &mut Ik, t: &T) {
    if g.ambient.fresh != 0 {
        g.ambient.fresh = 0;
        g.ambient.cheat[1] = false;
        let r = g.random();
        g.ambient.rate = (r & 1) + 3;
        let r = g.random();
        let i = (r & 0xf) as usize;
        let a = &mut g.ambient;
        a.x = t.b(0x193D4 + i);
        a.y = t.b(0x193D4 + 16 + i);
        a.dir = (r & 0x20) >> 5;
        a.repeat = 0;
        if r & 0xc0 != 0 {
            a.repeat = t.b(0x193D4 + 32 + i);
        }
    }
    let a = &mut g.ambient;
    a.sub = a.sub.wrapping_add(1);
    if a.sub as i8 >= a.rate as i8 {
        a.step_n = a.step_n.wrapping_add(1);
        a.sub = 0;
    }
    if a.step_n as i8 >= 0x14 {
        a.repeat = a.repeat.wrapping_sub(1);
        if (a.repeat as i8) < 0 {
            a.state = 0;
            return;
        }
        a.step_n = 0;
        a.sub = 0;
        a.x = a.x.wrapping_sub(8);
        if a.dir != 0 {
            a.x = a.x.wrapping_add(16);
        }
        return;
    }
    if a.step_n == 6 && a.sub == 0 {
        a.x = a.x.wrapping_sub(4);
        if a.dir != 0 {
            a.x = a.x.wrapping_add(8);
        }
    }
    let mut addr = 0x1C45C + 0x600 + 2 * a.step_n as u32;
    if a.dir != 0 {
        addr += 0x8C0;
    }
    let b = g.shown;
    let (x, y) = (g.ambient.x as u16, g.ambient.y as i16);
    let w = bob8(g, t, b, Bob::at(addr), x, y, 14, true);
    g.ambient.r4 = patch(w, 15, b);
}

/// reset_animation_state_2 ($196E4), state 5: a drop falling with a random drift until a random height; one still
/// falling is drawn in the shown screen, one landed is drawn into both screens and kept (the last 16 stay).
fn reset_animation_state_2(g: &mut Ik, t: &T) {
    if g.ambient.init {
        g.ambient.init = false;
        let r = (g.random() & 0x1e) as usize;
        g.ambient.bx = (t.w(0x196A4 + r) as i16).wrapping_add(0x20);
        g.ambient.by = t.w(0x196A4 + 32 + r) as i16;
        g.ambient.r5 = None;
    }
    if let Some(p) = g.ambient.r5.take() {
        restore(g, Patch { buf: 2, ..p });
    }
    let r = (g.random() & 0x1e) as usize;
    let a = &mut g.ambient;
    a.bx = a.bx.wrapping_add(t.w(0x19684 + r) as i16);
    if a.bx < 0xf2 {
        a.bx += 3;
    }
    if g.vbl & 1 != 0 {
        g.ambient.by = g.ambient.by.wrapping_add(1);
        let mut r = g.random() & 7;
        if r >= 5 {
            r &= 3;
        }
        if g.ambient.by >= r as i16 + 0x6c {
            g.ambient.state = 0;
        }
    }
    let src = Bob::at(0x229AC + 0x600);
    let (x, y) = (g.ambient.bx, g.ambient.by);
    if g.ambient.state == 0 {
        bobp(g, t, 0, src, x, y, 2, false);
        let w = bobp(g, t, 1, src, x, y, 2, false);
        let i = (g.ambient.trail_i & 0xf) as usize;
        if let Some(p) = g.ambient.trail[i] {
            restore(g, p);
        }
        g.ambient.trail[i] = patch(w, 3, 2);
        g.ambient.trail_i = (g.ambient.trail_i + 1) & 0xf;
        g.ambient.r5 = None;
    } else {
        let b = g.shown;
        let w = bobp(g, t, b, src, x, y, 2, false);
        g.ambient.r5 = patch(w, 3, 2);
    }
}

/// reset_and_poll_input_state ($198D8), state 6: a creature crossing the whole beach along one of four routes in
/// turn, ten frames of animation, behind the scenery once inside it, in the shown screen.
fn reset_and_poll_input_state(g: &mut Ik, t: &T) {
    let d5 = ((g.ambient.route << 1) & 6) as usize;
    if g.ambient.init {
        g.ambient.init = false;
        g.ambient.anim = 0;
        g.ambient.bx = t.w(0x19868 + 8 + d5) as i16;
        g.ambient.by = t.w(0x19868 + 24 + d5) as i16;
        g.ambient.r5 = None;
    }
    if let Some(p) = g.ambient.r5.take() {
        restore(g, p);
    }
    if g.vbl & 1 == 0 {
        g.ambient.anim += 1;
        if g.ambient.anim >= 10 {
            g.ambient.anim = 0;
        }
    }
    let d4 = t.w(0x19868 + d5) as i16;
    if g.vbl & 1 == 0 {
        g.ambient.bx = g.ambient.bx.wrapping_add(d4);
        if g.ambient.bx == t.w(0x19868 + 16 + d5) as i16 {
            g.ambient.route = (g.ambient.route + 1) & 3;
            g.ambient.state = 0;
            return;
        }
    }
    let tab = if d4 < 0 { 0x19888 } else { 0x198B0 };
    let src = Bob::at(t.l(tab + 4 * g.ambient.anim as usize));
    let (x, y) = (g.ambient.bx, g.ambient.by);
    let masked = (0x22..0x154).contains(&x);
    let b = g.shown;
    let w = bobp(g, t, b, src, x, y, 8, masked);
    g.ambient.r5 = patch(w, 9, b);
}

/// reset_and_update_state ($19B82), state 7: a figure acting out one of eight scripts (d0_19B42: a body frame a
/// step, pauses, the end), its legs cycling under it, behind the scenery in the shown screen.
fn reset_and_update_state(g: &mut Ik, t: &T) {
    if g.ambient.init {
        g.ambient.init = false;
        let a = &mut g.ambient;
        (a.pos, a.pause, a.frame7, a.legs, a.legs_t) = (0, 0, 0, 0, 0);
        g.ambient.script = (g.random() & 7) as u16;
        let r = (g.random() & 0xe) as usize;
        g.ambient.bx = ((t.w(0x19B62 + r) >> 1) + 0x10) as i16;
        g.ambient.by = t.w(0x19B62 + 16 + r) as i16;
        g.ambient.r5 = None;
    }
    if g.ambient.pause != 0 {
        g.ambient.pause -= 1;
    } else {
        let p = (t.l(0x19B42 + 4 * g.ambient.script as usize) as usize).wrapping_sub(0x600);
        let mut d7 = t.b(p.wrapping_add(g.ambient.pos as usize)) as u16;
        if d7 == 0xff {
            g.ambient.state = 0;
            return;
        }
        if d7 >= 0x80 {
            g.ambient.pause = 0xa;
        }
        if d7 != 0xfe {
            d7 &= 0x1f;
        }
        g.ambient.frame7 = d7;
        if g.vbl & 3 == 0 {
            g.ambient.pos += 1;
        }
    }
    let a = &mut g.ambient;
    a.legs_t -= 1;
    if a.legs_t < 0 {
        a.legs_t = 5;
        a.legs += 1;
        if a.legs >= 7 {
            a.legs = 0;
        }
    }
    let (mut rows, mut f) = (9, a.frame7);
    if f == 0xfe {
        rows = 1;
        f = 0;
    }
    let (x, y) = (a.bx as u16, a.by);
    let b = g.shown;
    let body = Bob::at(t.l(0x19A0C + 4 * f as usize));
    let w = bob8(g, t, b, body, x, y, rows, true);
    g.ambient.r5 = patch(w, 12, b);
    let legs = Bob::at(t.l(0x19A64 + 4 * g.ambient.legs as usize));
    bob8(g, t, b, legs, x, y.wrapping_add(7), 4, true);
}
