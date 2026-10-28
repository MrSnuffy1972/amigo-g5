//! M5 part 2 differential test: the Rust blitter (`blit.rs`) against the C native blit classes in `runtime/intentrt.c`.
//!
//! Each scenario is synthetic (no game data): 128 KB of chip RAM filled with noise, then a stream of blitter register
//! writes made here (clears, copies, shifted copies, bob draws, area fills, lines in all eight octants, blits with no
//! destination, ECS BLTSIZV/BLTSIZH, pointers that wrap, chained blits that start where the last one left its pointers,
//! and random register streams). The C side takes the blit code from intentrt.c's own text at test time and starts each
//! blit the way intentrt.c write_custom does. Chip RAM, every blitter register and the class of every blit must be
//! identical. Skips cleanly without gcc.

use std::fmt::Write as _;
use std::fs;
use std::process::Command;

use amigo_g5::blit::{Blitter, ChipMut};

mod common;
use common::{have, repo_root, run_ok};

const CHIP: usize = 128 * 1024;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn u16(&mut self) -> u16 {
        self.next() as u16
    }
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[self.below(v.len() as u64) as usize]
    }
}

struct Scenario {
    name: String,
    chip: Vec<u8>,
    writes: Vec<(u16, u16)>,
}

impl Scenario {
    fn new(name: &str, seed: u64) -> Self {
        let mut r = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
        Scenario {
            name: name.into(),
            chip: (0..CHIP).map(|_| r.next() as u8).collect(),
            writes: Vec::new(),
        }
    }
    fn serialise(&self, b: &mut Vec<u8>) {
        b.extend_from_slice(&self.chip);
        b.extend_from_slice(&(self.writes.len() as u32).to_le_bytes());
        for &(r, v) in &self.writes {
            b.extend_from_slice(&r.to_le_bytes());
            b.extend_from_slice(&v.to_le_bytes());
        }
    }
}

/// One blit's register set-up. Pointers and modulos are A, B, C, D; data registers A, B, C.
#[derive(Clone, Copy)]
struct Spec {
    con0: u16,
    con1: u16,
    fwm: u16,
    lwm: u16,
    ptr: [u32; 4],
    modulo: [i16; 4],
    dat: [u16; 3],
    h: u16,
    w: u16,
}

impl Default for Spec {
    fn default() -> Self {
        Spec {
            con0: 0,
            con1: 0,
            fwm: 0xffff,
            lwm: 0xffff,
            ptr: [0x2000, 0x6000, 0xa000, 0xe000],
            modulo: [0; 4],
            dat: [0; 3],
            h: 1,
            w: 1,
        }
    }
}

impl Spec {
    /// The register writes, then BLTSIZE (h and w 1024 and 64 are written as 0, as the hardware takes them).
    fn push(&self, s: &mut Scenario) {
        let w = &mut s.writes;
        w.extend([(0x040, self.con0), (0x042, self.con1)]);
        w.extend([(0x044, self.fwm), (0x046, self.lwm)]);
        for (hi, p) in [0x050, 0x04c, 0x048, 0x054].into_iter().zip(self.ptr) {
            w.extend([(hi, (p >> 16) as u16), (hi + 2, p as u16)]);
        }
        for (r, m) in [0x064, 0x062, 0x060, 0x066].into_iter().zip(self.modulo) {
            w.push((r, m as u16));
        }
        for (r, d) in [0x074, 0x072, 0x070].into_iter().zip(self.dat) {
            w.push((r, d));
        }
        w.push((0x058, ((self.h & 0x3ff) << 6) | (self.w & 0x3f)));
    }
}

/// A line from (x0, y0) to (x1, y1) in a 40-byte-wide plane at `base`, set up as the Hardware Reference Manual
/// describes (octant from the signs and the major axis, error and increments from dx and dy).
fn line(
    s: &mut Scenario,
    base: u32,
    (x0, y0): (i32, i32),
    (x1, y1): (i32, i32),
    sing: bool,
    lf: u16,
) {
    let (dx, dy) = ((x1 - x0).abs(), (y1 - y0).abs());
    let (dmax, dmin) = (dx.max(dy), dx.min(dy));
    // the C's octant bits: 4 = y is the major axis, 2 = step back along the major axis, 1 = back along the minor one
    let oct = if dy >= dx {
        4 | if y1 < y0 { 2 } else { 0 } | if x1 < x0 { 1 } else { 0 }
    } else {
        (if x1 < x0 { 2 } else { 0 }) | if y1 < y0 { 1 } else { 0 }
    };
    let err = 4 * dmin - 2 * dmax;
    let start = base + (y0 * 40 + (x0 >> 4) * 2) as u32;
    Spec {
        con0: (((x0 & 15) as u16) << 12) | 0x0b00 | lf,
        con1: ((x0 as u16 * 7) & 15) << 12
            | (oct << 2)
            | if err < 0 { 0x40 } else { 0 }
            | if sing { 2 } else { 0 }
            | 1,
        ptr: [err as u32, 0, start, start],
        modulo: [(4 * (dmin - dmax)) as i16, (4 * dmin) as i16, 40, 40],
        dat: [0x8000, if sing { 0xffff } else { 0xf0f3 }, 0],
        h: (dmax + 1) as u16,
        w: 2,
        ..Spec::default()
    }
    .push(s);
}

fn hand_written() -> Vec<Scenario> {
    let mut v = Vec::new();
    let mut add = |name: &str, specs: &[Spec]| {
        let mut s = Scenario::new(name, v.len() as u64 + 1);
        specs.iter().for_each(|b| b.push(&mut s));
        v.push(s);
    };
    let base = Spec {
        h: 20,
        w: 10,
        modulo: [6, 4, 2, 8],
        ..Spec::default()
    };
    add(
        "clear_zero",
        &[Spec {
            con0: 0x0100,
            ..base
        }],
    );
    add(
        "fill_ones",
        &[Spec {
            con0: 0x01ff,
            ..base
        }],
    );
    add(
        "fill_adat",
        &[Spec {
            con0: 0x01f0,
            dat: [0xa5c3, 0, 0],
            ..base
        }],
    );
    add(
        "copy_a_masked",
        &[Spec {
            con0: 0x09f0,
            fwm: 0x0fff,
            lwm: 0xfff0,
            ..base
        }],
    );
    add(
        "copy_b",
        &[Spec {
            con0: 0x05cc,
            ..base
        }],
    );
    add(
        "shift_up",
        &[Spec {
            con0: 0x49f0,
            ..base
        }],
    );
    add(
        "shift_down",
        &[Spec {
            con0: 0x79f0,
            con1: 2,
            ptr: [0x4000, 0, 0, 0xf000],
            fwm: 0xff00,
            ..base
        }],
    );
    add(
        "shift_down_zero",
        &[Spec {
            con0: 0x09f0,
            con1: 2,
            ..base
        }],
    );
    add(
        "bob",
        &[Spec {
            con0: 0x8fca,
            con1: 0x8000,
            lwm: 0,
            ..base
        }],
    );
    add(
        "bob_mask_from_adat",
        &[Spec {
            con0: 0x37ca,
            con1: 0x3000,
            dat: [0xf00f, 0, 0],
            ..base
        }],
    );
    add(
        "fill_inclusive_down",
        &[Spec {
            con0: 0x09f0,
            con1: 0x000a,
            ptr: [0x3000, 0, 0, 0x3000],
            ..base
        }],
    );
    add(
        "fill_exclusive_fci",
        &[Spec {
            con0: 0x09f0,
            con1: 0x0016,
            ..base
        }],
    );
    add(
        "fill_exclusive_up",
        &[Spec {
            con0: 0x0bfa,
            con1: 0x0010,
            ..base
        }],
    );
    add(
        "no_destination",
        &[Spec {
            con0: 0x0cfc,
            ..base
        }],
    );
    add(
        "general_minterms",
        &[Spec {
            con0: 0x3b5a,
            con1: 0x5000,
            ..base
        }],
    );
    add(
        "wrapping_pointers",
        &[Spec {
            con0: 0x0fe2,
            ptr: [0x00ff_fff0, 0x7fff_0000, 0x1_ffe0, 0xfffe],
            modulo: [-100, 31, -3, 7],
            ..base
        }],
    );
    add(
        "bltsize_zero_is_1024_by_64",
        &[Spec {
            con0: 0x01f0,
            dat: [0x1234, 0, 0],
            h: 0,
            w: 0,
            ..Spec::default()
        }],
    );
    // chained general blits: the second writes only BLTCON0/1 and BLTSIZE, so it starts at the first's end pointers
    let mut s = Scenario::new("chained_general", 99);
    Spec {
        con0: 0x0f96,
        con1: 0x2000,
        ..base
    }
    .push(&mut s);
    s.writes
        .extend([(0x040, 0x0d3c), (0x042, 0x0002), (0x058, (7 << 6) | 3)]);
    v.push(s);
    // ECS: BLTSIZV holds the height, BLTSIZH starts the blit
    let mut s = Scenario::new("ecs_bltsizh", 100);
    Spec {
        con0: 0x2b6c,
        ..base
    }
    .push(&mut s);
    // (a zero height is 32768 rows and a zero width 2048 words)
    s.writes.extend([
        (0x05c, 37),
        (0x05e, 5),
        (0x05c, 3),
        (0x05e, 0x7c3),
        (0x05c, 2),
        (0x05e, 0),
        (0x05c, 0x8000),
        (0x05e, 0xf801),
    ]);
    v.push(s);
    // lines: every octant, both SING settings, short and long
    let mut s = Scenario::new("lines", 101);
    let ends = [
        (40, 3),
        (3, 40),
        (-40, 9),
        (-9, 40),
        (40, -11),
        (11, -40),
        (-40, -13),
        (-13, -40),
        (0, 0),
        (25, 25),
        (-1, 60),
    ];
    for (k, &(dx, dy)) in ends.iter().enumerate() {
        for sing in [false, true] {
            let (x0, y0) = (100 + k as i32 * 3, 120);
            line(&mut s, 0x8000, (x0, y0), (x0 + dx, y0 + dy), sing, 0xca);
            line(&mut s, 0x8000, (x0 + 1, y0), (x0 + dx, y0 + dy), sing, 0x4a);
        }
    }
    v.push(s);
    v
}

fn random_scenario(seed: u64) -> Scenario {
    let mut r = Rng(seed * 7919 + 13);
    let mut s = Scenario::new(&format!("random_{seed}"), seed + 1000);
    for _ in 0..1 + r.below(6) {
        let uses = if r.chance(70) {
            r.pick(&[1u16, 9, 5, 0xf, 7, 0xb, 0xd, 3, 0xc])
        } else {
            r.below(16) as u16
        };
        let lf = if r.chance(60) {
            r.pick(&[0u16, 0xff, 0xf0, 0xcc, 0xca, 0x0a, 0x5a])
        } else {
            r.below(256) as u16
        };
        let mut con1 = r.u16() & 0xf01e;
        if r.chance(25) {
            con1 |= 1;
        }
        if r.chance(60) {
            con1 &= !0x18;
        }
        let ptr = |r: &mut Rng| {
            if r.chance(80) {
                r.below(CHIP as u64) as u32 & !1
            } else {
                r.next() as u32
            }
        };
        let modulo = |r: &mut Rng| {
            if r.chance(80) {
                (r.below(80) as i16 - 40) & !1
            } else {
                r.u16() as i16
            }
        };
        let (h, w) = if r.chance(90) {
            (1 + r.below(64) as u16, 1 + r.below(24) as u16)
        } else {
            (r.u16() & 0x3ff, r.u16() & 0x3f)
        };
        Spec {
            con0: ((r.below(16) as u16) << 12) | (uses << 8) | lf,
            con1: if r.chance(30) { con1 & 0x0fff } else { con1 },
            fwm: if r.chance(50) { 0xffff } else { r.u16() },
            lwm: if r.chance(50) { 0xffff } else { r.u16() },
            ptr: [ptr(&mut r), ptr(&mut r), ptr(&mut r), ptr(&mut r)],
            modulo: [
                modulo(&mut r),
                modulo(&mut r),
                modulo(&mut r),
                modulo(&mut r),
            ],
            dat: [r.u16(), r.u16(), r.u16()],
            h,
            w,
        }
        .push(&mut s);
        // sometimes a second blit straight after, from the pointers the first left
        if r.chance(20) {
            s.writes
                .push((0x058, (r.u16() & 0x0fc0) | (1 + r.below(8) as u16)));
        }
    }
    s
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "amiga.h"
uint8_t chipram[CHIP_SIZE]; uint16_t custom[256];
@BLIT@
/* intentrt.c write_custom: the register store and the two cases that start a blit */
static void wr(int reg, uint16_t v) {
    reg &= 0x1fe; custom[reg >> 1] = v;
    if (reg == 0x05e) { uint16_t hh = custom[0x05c >> 1] & 0x7fff, ww = v & 0x7ff; intent_blit_dispatch_ext(hh ? hh : 32768, ww ? ww : 2048); }
    if (reg == 0x058) { uint16_t hh = (v >> 6) & 0x3ff, ww = v & 0x3f; intent_blit_dispatch_ext(hh ? hh : 1024, ww ? ww : 64); }
}
static FILE *in;
static void rd(void *p, size_t n) { if (fread(p, 1, n, in) != n) { fprintf(stderr, "short input\n"); exit(2); } }
static uint32_t u32(void) { uint8_t b[4]; rd(b, 4); return b[0] | b[1] << 8 | b[2] << 16 | (uint32_t)b[3] << 24; }
static uint16_t u16(void) { uint8_t b[2]; rd(b, 2); return (uint16_t)(b[0] | b[1] << 8); }
int main(int argc, char **argv) {
    in = fopen(argv[1], "rb"); if (!in) return 2;
    uint32_t ns = u32();
    for (uint32_t sc = 0; sc < ns; sc++) {
        rd(chipram, CHIP_SIZE); memset(custom, 0, sizeof custom);
        char p[4096]; snprintf(p, sizeof p, "%s/s%03u.txt", argv[2], sc); FILE *t = fopen(p, "w");
        uint32_t nw = u32();
        for (uint32_t k = 0; k < nw; k++) {
            uint16_t reg = u16(), val = u16(); uint64_t before[8]; memcpy(before, blit_class_counts, sizeof before);
            wr(reg, val);
            for (int c = 0; c < 8; c++) if (blit_class_counts[c] != before[c]) fprintf(t, "blit %d\n", c);
        }
        for (int r = 0x040; r <= 0x074; r += 2) fprintf(t, "reg %03X %04X\n", r, custom[r >> 1]);
        fclose(t);
        snprintf(p, sizeof p, "%s/s%03u.chip", argv[2], sc); t = fopen(p, "wb");
        if (fwrite(chipram, 1, CHIP_SIZE, t) != CHIP_SIZE) return 3;
        fclose(t);
    }
    return 0;
}
"#;

#[test]
fn blits_match_c_intentrt() {
    if !have("gcc") {
        eprintln!("skipped: needs gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-blit-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(work.join("out")).unwrap();
    let rt = root.join("kit/amigo/port/runtime");
    // the oracle: intentrt.c's blit classes, from ib_r16 to intent_blit_dispatch_ext, taken from its text
    let intentrt = fs::read_to_string(rt.join("intentrt.c")).unwrap();
    let i = intentrt
        .find("static inline uint16_t ib_r16(")
        .expect("ib_r16 in intentrt.c");
    let j = i
        + intentrt[i..]
            .find("\nint intent_blit_dispatch_ext(")
            .expect("intent_blit_dispatch_ext in intentrt.c")
        + 1;
    let j = j + intentrt[j..].find('\n').unwrap() + 1;
    fs::write(
        work.join("driver.c"),
        C_DRIVER.replace("@BLIT@", &intentrt[i..j]),
    )
    .unwrap();
    let exe = work.join("c_blit");
    run_ok(
        Command::new("gcc")
            .args(["-O1", "-w", &format!("-DCHIP_SIZE={CHIP}"), "-I"])
            .arg(&rt)
            .arg("-o")
            .arg(&exe)
            .arg(work.join("driver.c")),
    );
    let mut sc = hand_written();
    sc.extend((0..160).map(random_scenario));
    let mut input = (sc.len() as u32).to_le_bytes().to_vec();
    sc.iter().for_each(|s| s.serialise(&mut input));
    fs::write(work.join("in.bin"), &input).unwrap();
    run_ok(
        Command::new(&exe)
            .arg(work.join("in.bin"))
            .arg(work.join("out")),
    );

    let (mut blits, mut changed) = (0usize, 0usize);
    let mut kinds = [0usize; 8];
    for (n, s) in sc.iter().enumerate() {
        let mut ram = s.chip.clone();
        let mut chip = ChipMut::new(&mut ram);
        let mut b = Blitter::default();
        let mut text = String::new();
        for &(reg, val) in &s.writes {
            if let Some(k) = b.write(&mut chip, reg, val) {
                let _ = writeln!(text, "blit {}", k as usize);
                kinds[k as usize] += 1;
                blits += 1;
            }
        }
        for r in (0x040..=0x074).step_by(2) {
            let _ = writeln!(text, "reg {r:03X} {:04X}", b.reg(r));
        }
        let tpath = work.join(format!("out/s{n:03}.txt"));
        let cpath = work.join(format!("out/s{n:03}.chip"));
        assert_eq!(
            text,
            fs::read_to_string(&tpath).unwrap(),
            "{}: blit classes or registers differ",
            s.name
        );
        let c_chip = fs::read(&cpath).unwrap();
        if let Some(k) = (0..CHIP).find(|&k| ram[k] != c_chip[k]) {
            panic!(
                "{}: chip RAM differs first at ${k:05X}: rust {:02X} c {:02X}",
                s.name, ram[k], c_chip[k]
            );
        }
        changed += (0..CHIP).filter(|&k| ram[k] != s.chip[k]).count();
        let _ = fs::remove_file(tpath);
        let _ = fs::remove_file(cpath);
    }
    eprintln!(
        "{} scenarios, {blits} blits (by class 0-7: {kinds:?}), {changed} chip bytes changed: identical",
        sc.len()
    );
    // every class must be exercised, and the blits must have written something
    assert!(kinds.iter().all(|&k| k > 0), "classes {kinds:?}");
    assert!(changed > 100_000);
    let _ = fs::remove_dir_all(&work);
}
