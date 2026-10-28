//! M5 differential test: the Rust display (`display.rs`) against the C clean-sheet display (`runtime/intent.c`).
//!
//! Each scenario is synthetic (no game data): chip RAM filled with patterns or noise, a register shadow, a copper list
//! written here, and the CPU's own register writes during each frame. The C side `#include`s the real intent.c, with
//! `rgb12` taken from intentrt.c's text and frames written by the kit's own `amigo_img_save` (amigo_img.h), and runs
//! frame_begin, the CPU writes, compose and the next frame's COP1LC choice (intentrt.c G2E9 coplc) for 2 or 3 frames.
//! The Rust does the same. The copper table (every change, callback, skip count and the list's COP1LC/COP2LC moves),
//! the next frame's copper start and the raw RGBA dump must be byte-identical. Skips cleanly without gcc.

use std::fmt::Write as _;
use std::fs;
use std::process::Command;

use amigo_g5::display::{self, Chip, CopperTable, CpuWrites, Display, Shadow, SCR_H, SCR_W};

mod common;
use common::{have, repo_root, run_ok};

const CHIP: usize = 512 * 1024;

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
}

struct Scenario {
    name: String,
    chip: Vec<u8>,
    custom: [u16; 256],
    palette: [u16; 32],
    bplpt: [u32; 6],
    dmacon: u16,
    cop: u32,
    /// Per frame: the CPU's register writes (line, register, value), in the order they happen.
    frames: Vec<Vec<(i32, u16, u16)>>,
}

impl Scenario {
    fn new(name: &str) -> Self {
        let mut s = Scenario {
            name: name.into(),
            chip: vec![0; CHIP],
            custom: [0; 256],
            palette: [0; 32],
            bplpt: [0; 6],
            dmacon: 0x8380,
            cop: 0x1000,
            frames: vec![vec![], vec![]],
        };
        s.set(0x8e, 0x2c81);
        s.set(0x90, 0x2cc1);
        s.set(0x92, 0x38);
        s.set(0x94, 0xd0);
        for (i, p) in s.palette.iter_mut().enumerate() {
            *p = (i as u16 * 0x135) & 0xfff;
        }
        s
    }
    fn set(&mut self, reg: u16, v: u16) {
        self.custom[(reg >> 1) as usize] = v;
    }
    fn w16(&mut self, a: u32, v: u16) {
        self.chip[a as usize..a as usize + 2].copy_from_slice(&v.to_be_bytes());
    }
    fn noise(&mut self, r: &mut Rng, from: u32, len: u32) {
        for a in from..from + len {
            self.chip[a as usize] = r.next() as u8;
        }
    }
    /// Writes a copper list at `at` (instruction pairs; the caller adds the end marker).
    fn list(&mut self, at: u32, ins: &[(u16, u16)]) {
        for (i, &(a, b)) in ins.iter().enumerate() {
            self.w16(at + i as u32 * 4, a);
            self.w16(at + i as u32 * 4 + 2, b);
        }
    }
    fn serialise(&self, b: &mut Vec<u8>) {
        b.extend_from_slice(&self.chip);
        self.custom
            .iter()
            .for_each(|v| b.extend_from_slice(&v.to_le_bytes()));
        self.palette
            .iter()
            .for_each(|v| b.extend_from_slice(&v.to_le_bytes()));
        self.bplpt
            .iter()
            .for_each(|v| b.extend_from_slice(&v.to_le_bytes()));
        b.extend_from_slice(&self.dmacon.to_le_bytes());
        b.extend_from_slice(&self.cop.to_le_bytes());
        b.extend_from_slice(&(self.frames.len() as u32).to_le_bytes());
        for f in &self.frames {
            b.extend_from_slice(&(f.len() as u32).to_le_bytes());
            for &(l, r, v) in f {
                b.extend_from_slice(&l.to_le_bytes());
                b.extend_from_slice(&r.to_le_bytes());
                b.extend_from_slice(&v.to_le_bytes());
            }
        }
    }
}

fn mv(reg: u16, v: u16) -> (u16, u16) {
    (reg, v)
}
fn wait(v: u16, h: u16) -> (u16, u16) {
    ((v << 8) | (h & 0xfe) | 1, 0xfffe)
}
fn skip(v: u16, h: u16) -> (u16, u16) {
    ((v << 8) | (h & 0xfe) | 1, 0xffff)
}
const END: (u16, u16) = (0xffff, 0xfffe);

/// MOVEs that point the bitplanes at `base`, `stride` bytes apart.
fn planes(n: usize, base: u32, stride: u32) -> Vec<(u16, u16)> {
    let mut v = Vec::new();
    for p in 0..n {
        let a = base + p as u32 * stride;
        v.push(mv(0xe0 + p as u16 * 4, (a >> 16) as u16));
        v.push(mv(0xe2 + p as u16 * 4, a as u16));
    }
    v
}

/// A playfield of `con0` over noise, colour bars every 8 lines, mid-line colour changes and a COPER callback.
fn playfield(name: &str, con0: u16, ddf: (u16, u16), seed: u64) -> Scenario {
    let mut r = Rng(seed);
    let mut s = Scenario::new(name);
    s.set(0x92, ddf.0);
    s.set(0x94, ddf.1);
    s.noise(&mut r, 0x10000, 0x40000);
    let mut l = vec![
        mv(0x100, con0),
        mv(0x102, 0x0035),
        mv(0x108, 8),
        mv(0x10a, 0xfff8),
    ];
    l.extend(planes(6, 0x10000, 0xa000));
    for i in 0..32u16 {
        l.push(mv(0x180 + i * 2, r.u16() & 0xfff));
    }
    let mut v = 0x2c;
    while v < 0xff {
        l.push(wait(v, 0x07));
        l.push(mv(0x180, r.u16() & 0xfff));
        l.push(wait(v, 0x50));
        l.push(mv(0x182, r.u16() & 0xfff));
        l.push(wait(v, 0x9a));
        l.push(mv(0x184, r.u16() & 0xfff));
        v += 8;
    }
    l.push(mv(0x9c, 0x8010));
    l.push(wait(0xff, 0xde));
    l.push(wait(0x20, 0x07));
    l.push(mv(0x180, 0x0f00));
    l.push(END);
    s.list(0x1000, &l);
    s
}

fn sprite(
    s: &mut Scenario,
    at: u32,
    vstart: u16,
    vstop: u16,
    h: u16,
    attach: bool,
    rows: &[(u16, u16)],
) -> u32 {
    let pos = ((vstart & 0xff) << 8) | ((h >> 1) & 0xff);
    let ctl = ((vstop & 0xff) << 8)
        | if attach { 0x80 } else { 0 }
        | ((vstart >> 8) & 1) << 2
        | ((vstop >> 8) & 1) << 1
        | (h & 1);
    s.w16(at, pos);
    s.w16(at + 2, ctl);
    let mut a = at + 4;
    for &(da, db) in rows {
        s.w16(a, da);
        s.w16(a + 2, db);
        a += 4;
    }
    a
}

fn sprites(seed: u64) -> Scenario {
    let mut s = playfield("sprites_dpf_attached", 0x6600, (0x38, 0xd0), seed);
    let mut r = Rng(seed ^ 0x55);
    s.dmacon = 0x83e0;
    s.frames.push(vec![]);
    s.set(0x104, 0x0024);
    let mut l = vec![mv(0x100, 0x6600), mv(0x104, 0x0012)];
    l.extend(planes(6, 0x10000, 0xa000));
    for n in 0..8u32 {
        let base = 0x60000 + n * 0x800;
        l.push(mv(0x120 + n as u16 * 4, (base >> 16) as u16));
        l.push(mv(0x122 + n as u16 * 4, base as u16));
        let rows: Vec<(u16, u16)> = (0..20).map(|_| (r.u16(), r.u16())).collect();
        let vs = 0x30 + n as u16 * 20;
        // two uses of each sprite in a frame (the second after VSTOP), the odd ones of pairs 1 and 3 attached
        let a = sprite(
            &mut s,
            base,
            vs,
            vs + 20,
            0x80 + n as u16 * 13,
            n == 3 || n == 7,
            &rows,
        );
        let a = sprite(
            &mut s,
            a,
            vs + 40 + 0xc0 * (n as u16 & 1),
            vs + 50 + 0xc0 * (n as u16 & 1),
            0x90 + n as u16 * 7,
            n == 7,
            &rows[..10],
        );
        s.w16(a, 0);
        s.w16(a + 2, 0);
    }
    l.push(wait(0x80, 0x07));
    l.push(mv(0x104, 0x0009));
    l.push(wait(0xa0, 0x07));
    l.push(mv(0x100, 0x5200));
    l.push(END);
    s.list(0x1000, &l);
    s.frames[1] = vec![(0x90, 0x104, 0x003f), (0xc0, 0x96, 0x0020)];
    s
}

/// COPJMP2 to a second list, SKIP, the 8-bit WAIT line ($FF,$DE then $1A), a WAIT for h >= $E4, a jump loop at one
/// line, and a list that moves COP1LC (the next frame starts there) or points it at zeros (ignored).
fn copper_flow(plausible: bool) -> Scenario {
    let mut s = Scenario::new(if plausible {
        "copper_flow_coplc"
    } else {
        "copper_flow_zero_coplc"
    });
    let mut r = Rng(77);
    s.noise(&mut r, 0x10000, 0x10000);
    s.set(0x100, 0x2200);
    s.bplpt = [0x10000, 0x14000, 0, 0, 0, 0];
    let next = if plausible { 0x3000 } else { 0x7000 };
    s.list(
        0x1000,
        &[
            mv(0x84, 0),
            mv(0x86, 0x2000),
            mv(0x180, 0x111),
            wait(0x40, 0x20),
            mv(0x8a, 0),
            mv(0x180, 0xbad),
            END,
        ],
    );
    s.list(
        0x2000,
        &[
            mv(0x182, 0x0f0),
            skip(0x50, 0x00),
            mv(0x180, 0xbad),
            mv(0x180, 0x222),
            skip(0xf0, 0x00),
            mv(0x180, 0x333),
            wait(0x60, 0xe6),
            mv(0x102, 0x0011),
            wait(0x70, 0x40),
            mv(0x102, 0x0022),
            mv(0x80, (next >> 16) as u16),
            mv(0x82, next as u16),
            mv(0x9c, 0x8004),
            mv(0x9c, 0x0004),
            wait(0xff, 0xde),
            wait(0x1a, 0x07),
            mv(0x180, 0x444),
            END,
        ],
    );
    s.list(
        0x3000,
        &[
            mv(0x180, 0x555),
            wait(0x30, 0x07),
            mv(0x80, 0),
            mv(0x82, 0x3008),
            mv(0x88, 0),
            mv(0x88, 0),
            END,
        ],
    );
    s.frames.push(vec![]);
    s
}

/// The CPU's own writes during the frame (palette, plane pointer, BPLCON0) and copper MOVEs that land after the
/// line's fetch (COPTIME: they show from the next line).
fn cpu_writes() -> Scenario {
    let mut s = playfield("cpu_writes_coptime", 0x4200, (0x30, 0xd8), 9);
    s.list(
        0x1000,
        &[
            mv(0x100, 0x4200),
            wait(0x50, 0x30),
            mv(0x102, 0x00ff),
            wait(0x51, 0x10),
            mv(0x102, 0x0000),
            mv(0x184, 0x0f0f),
            wait(0x52, 0x7c),
            mv(0x92, 0x0040),
            // DDFSTRT $40 gives the cut-off $30: a MOVE at position $34 shows from the next line, one at $32 at once
            wait(0x60, 0x2e),
            mv(0x102, 0x0077),
            wait(0x61, 0x2c),
            mv(0x102, 0x0033),
            // a mid-line COLOR01 change on the line where the CPU also writes COLOR01: the CPU value holds after the line
            wait(0x90, 0x60),
            mv(0x182, 0x0ff0),
            END,
        ],
    );
    s.bplpt = [0x10000, 0x18000, 0x20000, 0x28000, 0, 0];
    s.frames[0] = vec![
        (0x40, 0x180, 0x0f00),
        (0x60, 0x0e2, 0x8000),
        (0x70, 0x100, 0x3200),
        (0x90, 0x182, 0x00f0),
        (0x90, 0x108, 0x0028),
    ];
    s.frames[1] = vec![(0x00, 0x100, 0x6a00), (0x50, 0x180, 0x0abc)];
    s
}

const DISP_REGS: [u16; 12] = [
    0x8e, 0x90, 0x92, 0x94, 0x96, 0x100, 0x102, 0x104, 0x108, 0x10a, 0xe0, 0x120,
];

fn random_reg(r: &mut Rng) -> u16 {
    let reg = DISP_REGS[r.below(DISP_REGS.len() as u64) as usize];
    match reg {
        0xe0 => 0xe0 + r.below(12) as u16 * 2,
        0x120 => 0x120 + r.below(16) as u16 * 2,
        _ => reg,
    }
}

fn random_scenario(seed: u64) -> Scenario {
    let mut r = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
    let mut s = Scenario::new(&format!("random_{seed}"));
    s.noise(&mut r, 0, CHIP as u32);
    for &reg in &[
        0x8e, 0x90, 0x92, 0x94, 0x100, 0x102, 0x104, 0x108, 0x10a, 0x84, 0x86,
    ] {
        if r.chance(40) {
            s.set(reg, r.u16());
        }
    }
    if r.chance(60) {
        let bpu = r.below(8) as u16;
        let mode = [0, 0x8000, 0x800, 0x400, 0][r.below(5) as usize];
        s.set(0x100, bpu << 12 | mode | 0x200);
    }
    for n in 0..16 {
        s.custom[0x90 + n] = r.u16();
    }
    s.palette.iter_mut().for_each(|p| *p = r.u16() & 0xfff);
    s.bplpt
        .iter_mut()
        .for_each(|p| *p = (r.next() as u32) & 0x7fffe);
    s.dmacon = if r.chance(70) {
        0x0220 | r.u16() & 0x7fff
    } else {
        r.u16() & 0x7fff
    };
    s.cop = 0x1000 + r.below(0x100) as u32 * 4;
    let mut l = Vec::new();
    let mut v: u16 = r.below(0x40) as u16;
    for _ in 0..r.below(300) + 10 {
        let k = r.below(100);
        l.push(match k {
            0..=29 => mv(0x180 + r.below(32) as u16 * 2, r.u16()),
            30..=54 => mv(random_reg(&mut r), r.u16()),
            55..=79 => {
                v = if r.chance(5) {
                    r.below(256) as u16
                } else {
                    (v + r.below(16) as u16).min(0xff)
                };
                wait(v, r.u16())
            }
            80..=84 => skip(r.below(256) as u16, r.u16()),
            85..=89 => mv(0x9c, r.u16()),
            90..=92 => mv(0x80 + r.below(6) as u16 * 2, r.u16()),
            93 => mv(0x88 + r.below(2) as u16 * 2, 0),
            _ => (r.u16(), r.u16()),
        });
    }
    if r.chance(85) {
        l.push(END);
    }
    let cop = s.cop;
    s.list(cop, &l);
    for f in 0..2 {
        let mut w: Vec<(i32, u16, u16)> = (0..r.below(30))
            .map(|_| {
                let reg = if r.chance(40) {
                    0x180 + r.below(32) as u16 * 2
                } else {
                    random_reg(&mut r)
                };
                (r.below(313) as i32, reg, r.u16())
            })
            .collect();
        w.sort_by_key(|x| x.0);
        s.frames[f] = w;
    }
    s
}

fn scenarios() -> Vec<Scenario> {
    let mut v = vec![
        playfield("lores_5_planes", 0x5200, (0x38, 0xd0), 1),
        playfield("hires_4_planes", 0xc200, (0x3c, 0xd4), 2),
        playfield("hires_6_clamped", 0xe200, (0x20, 0xe0), 3),
        playfield("ham6", 0x6a00, (0x38, 0xd0), 4),
        playfield("ehb", 0x6200, (0x38, 0xd0), 5),
        playfield("seven_planes", 0x7200, (0x10, 0xd8), 6),
        playfield("no_fetch_ddf_backwards", 0x3200, (0xd0, 0x38), 7),
        sprites(8),
        copper_flow(true),
        copper_flow(false),
        cpu_writes(),
    ];
    v.extend((0..48).map(random_scenario));
    v
}

fn table_text(t: &CopperTable, next: u32) -> String {
    let mut s = String::new();
    for c in &t.changes {
        let _ = writeln!(
            s,
            "chg {} {} {} {:X} {:X}",
            c.line, c.hpos, c.seq, c.reg, c.val
        );
    }
    for &(l, b) in &t.callbacks {
        let _ = writeln!(s, "cb {l} {b:X}");
    }
    let _ = writeln!(s, "skips {}", t.skips);
    let _ = writeln!(
        s,
        "coplc {:X} {:X} {} {}",
        t.cop1, t.cop2, t.cop1_written as i32, t.cop2_written as i32
    );
    let _ = writeln!(s, "next {next:X}");
    s
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "amiga.h"
#include "amigo_img.h"
uint8_t chipram[CHIP_SIZE]; uint16_t custom[256]; uint16_t dmacon; uint32_t framebuf[SCR_W * SCR_H];
static uint16_t palette[32]; static uint32_t bplpt[6];
@RGB12@
#include "intent.c"
static FILE *in;
static void rd(void *p, size_t n) { if (fread(p, 1, n, in) != n) { fprintf(stderr, "short input\n"); exit(2); } }
static uint32_t u32(void) { uint8_t b[4]; rd(b, 4); return b[0] | b[1] << 8 | b[2] << 16 | (uint32_t)b[3] << 24; }
static uint16_t u16(void) { uint8_t b[2]; rd(b, 2); return (uint16_t)(b[0] | b[1] << 8); }
int main(int argc, char **argv) {
    in = fopen(argv[1], "rb"); if (!in) return 2;
    uint32_t ns = u32();
    for (uint32_t sc = 0; sc < ns; sc++) {
        rd(chipram, CHIP_SIZE);
        for (int i = 0; i < 256; i++) custom[i] = u16();
        for (int i = 0; i < 32; i++) palette[i] = u16();
        for (int i = 0; i < 6; i++) bplpt[i] = u32();
        dmacon = u16(); uint32_t cop = u32(), nf = u32();
        in_spt_valid = 0;
        for (uint32_t f = 0; f < nf; f++) {
            intent_frame_begin(cop);
            uint32_t nw = u32();
            for (uint32_t k = 0; k < nw; k++) { int32_t l = (int32_t)u32(); uint16_t reg = u16(), val = u16(); intent_cpu_write(l, reg, val); }
            char p[4096]; snprintf(p, sizeof p, "%s/s%03u_t%u.txt", argv[2], sc, f); FILE *t = fopen(p, "w");
            for (int i = 0; i < it_start.nchg; i++) { ichg_t *c = &it_start.chg[i]; fprintf(t, "chg %d %d %d %X %X\n", c->line, c->hpos, c->seq, c->reg, c->val); }
            for (int i = 0; i < it_start.ncb; i++) fprintf(t, "cb %d %X\n", it_start.cb[i].line, it_start.cb[i].bits);
            fprintf(t, "skips %d\ncoplc %X %X %d %d\n", it_start.skips, in_frame_c1, in_frame_c2, in_frame_c1w, in_frame_c2w);
            intent_compose(chipram, &is_start, &it_start, framebuf);
            if (in_frame_c1w && coplc_plausible(in_frame_c1 & CHIP_WMASK)) cop = in_frame_c1 & CHIP_WMASK;   /* intentrt.c G2E9 coplc */
            fprintf(t, "next %X\n", cop); fclose(t);
            snprintf(p, sizeof p, "%s/s%03u_p%u.rgba", argv[2], sc, f);
            if (!amigo_img_save(p, framebuf, SCR_W, SCR_H)) return 3;
        }
    }
    return 0;
}
"#;

#[test]
fn display_matches_c_intent() {
    if !have("gcc") {
        eprintln!("skipped: needs gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-display-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(work.join("out")).unwrap();
    let rt = root.join("kit/amigo/port/runtime");
    // the oracle's own colour conversion, taken from intentrt.c rather than copied into this file
    let intentrt = fs::read_to_string(rt.join("intentrt.c")).unwrap();
    let i = intentrt
        .find("static uint32_t rgb12(uint16_t c) {")
        .expect("rgb12 in intentrt.c");
    let j = i + intentrt[i..].find("\n}\n").unwrap() + 3;
    fs::write(
        work.join("driver.c"),
        C_DRIVER.replace("@RGB12@", &intentrt[i..j]),
    )
    .unwrap();
    let exe = work.join("c_display");
    run_ok(
        Command::new("gcc")
            .args(["-O1", "-w", "-I"])
            .arg(&rt)
            .arg("-I")
            .arg(root.join("kit/amigo/port"))
            .arg("-o")
            .arg(&exe)
            .arg(work.join("driver.c"))
            .arg("-lm")
            .env_remove("AMIGO_INTENT_COPJMP")
            .env_remove("AMIGO_INTENT_COPTIME"),
    );
    let sc = scenarios();
    let mut input = (sc.len() as u32).to_le_bytes().to_vec();
    sc.iter().for_each(|s| s.serialise(&mut input));
    fs::write(work.join("in.bin"), &input).unwrap();
    // the C reads its switches from the environment: clear them so it runs at its defaults, as the Rust does
    let mut cmd = Command::new(&exe);
    cmd.arg(work.join("in.bin")).arg(work.join("out"));
    for e in ["COPJMP", "COPTIME", "MIDLINE", "6PL", "SPRITES"] {
        cmd.env_remove(format!("AMIGO_INTENT_{e}"));
    }
    run_ok(cmd.env_remove("AMIGO_HIRES_POLICY"));

    let (mut frames, mut changes, mut nonblack) = (0, 0, 0);
    let mut out = vec![0u32; SCR_W * SCR_H];
    for (n, s) in sc.iter().enumerate() {
        let chip = Chip::new(&s.chip);
        let shadow = Shadow {
            custom: s.custom,
            palette: s.palette,
            bplpt: s.bplpt,
            dmacon: s.dmacon,
        };
        let mut disp = Display::default();
        let mut cop = s.cop;
        for (f, w) in s.frames.iter().enumerate() {
            let frame = disp.frame_begin(chip, &shadow, cop);
            let mut cw = CpuWrites::default();
            w.iter().for_each(|&(l, r, v)| cw.push(l, r, v));
            disp.compose(chip, &frame, &cw, &mut out);
            if frame.table.cop1_written
                && display::coplc_plausible(chip, frame.table.cop1 & chip.wmask())
            {
                cop = frame.table.cop1 & chip.wmask();
            }
            let tpath = work.join(format!("out/s{n:03}_t{f}.txt"));
            let ppath = work.join(format!("out/s{n:03}_p{f}.rgba"));
            let c_table = fs::read_to_string(&tpath).unwrap();
            assert_eq!(
                table_text(&frame.table, cop),
                c_table,
                "{} frame {f}: copper table differs",
                s.name
            );
            let c_px = fs::read(&ppath).unwrap();
            let mut r_px = Vec::new();
            display::write_rgba(&mut r_px, &out, SCR_W, SCR_H).unwrap();
            if r_px != c_px {
                let k = (12..c_px.len().min(r_px.len()))
                    .find(|&k| r_px[k] != c_px[k])
                    .unwrap_or(0);
                let p = (k.max(12) - 12) / 4;
                panic!(
                    "{} frame {f}: frame differs from the C (lengths {} {}), first at x {} y {}: rust {:?} c {:?}",
                    s.name,
                    r_px.len(),
                    c_px.len(),
                    p % SCR_W,
                    p / SCR_W,
                    &r_px[12 + p * 4..16 + p * 4],
                    &c_px[12 + p * 4..16 + p * 4]
                );
            }
            frames += 1;
            changes += frame.table.changes.len();
            nonblack += out.iter().filter(|&&c| c & 0xffffff != 0).count();
            let _ = fs::remove_file(tpath);
            let _ = fs::remove_file(ppath);
        }
    }
    eprintln!("{} scenarios, {frames} frames, {changes} table changes, {nonblack} non-black pixels: identical", sc.len());
    // the scenarios must exercise something: most pixels should not be black
    assert!(nonblack > frames * SCR_W * SCR_H / 2);
    let _ = fs::remove_dir_all(&work);
}

#[test]
fn rgba_dump_is_the_kit_format() {
    let mut b = Vec::new();
    display::write_rgba(&mut b, &[0xff12_3456, 0x00ab_cdef], 2, 1).unwrap();
    assert_eq!(
        b,
        [
            b'R', b'G', b'B', b'A', 2, 0, 0, 0, 1, 0, 0, 0, 0x12, 0x34, 0x56, 255, 0xab, 0xcd,
            0xef, 255
        ]
    );
}
