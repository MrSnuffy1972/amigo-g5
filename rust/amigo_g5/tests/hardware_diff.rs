//! M5 part 5 differential test: the custom chips as one runtime (`hardware.rs`) against `runtime/intentrt.c`.
//!
//! The C side is the whole of intentrt.c (with intent.c and the blit classes it includes), compiled with gcc and
//! linked with a small driver written here that defines only the scheduler clock and the PC ring intentrt.c expects
//! from cpu_native.c. Each scenario is synthetic: chip RAM made in the test (noise, copper lists, sample data), then a
//! stream of custom register writes through `chip_poke` (write_custom) and reads through `hw_r16` at a given CPU
//! clock, CIA writes and reads, chip RAM writes mid-tick, player input, and PAL lines through the real
//! `tick_line_step` and `chip_ipl`. Every read, INTREQ before and after `chip_ipl` and the level after every line, a
//! hash of every audio sample and of every finished frame, the blit classes and chip RAM at the end must be
//! identical. Skips cleanly without gcc.

use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::process::{Command, Stdio};

use amigo_g5::hardware::{Hardware, Input, CYC_PER_LINE};
use amigo_g5::paula::Paula;

mod common;
use common::{have, repo_root, run_ok};

const CHIP: usize = 65536;

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
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
}

#[derive(Clone, Copy)]
enum Ev {
    /// Custom register write.
    W(u16, u16),
    /// Custom register read, this many CPU cycles after the start of the current line.
    R(u16, u32),
    /// CIA n write, register, value.
    CiaW(u8, u8, u8),
    /// CIA n read, register.
    CiaR(u8, u8),
    /// Chip RAM byte write by the CPU.
    Poke(u16, u8),
    /// Player input.
    In(Input),
    /// This many lines.
    Lines(u32),
}

fn text(ev: &[Ev]) -> String {
    let mut s = String::new();
    for e in ev {
        let _ = match *e {
            Ev::W(r, v) => writeln!(s, "w {r} {v}"),
            Ev::R(r, d) => writeln!(s, "r {r} {d}"),
            Ev::CiaW(n, r, v) => writeln!(s, "W {n} {r} {v}"),
            Ev::CiaR(n, r) => writeln!(s, "R {n} {r}"),
            Ev::Poke(a, v) => writeln!(s, "m {a} {v}"),
            Ev::In(i) => writeln!(
                s,
                "i {} {} {} {} {} {} {} {} {}",
                i.up as u8,
                i.down as u8,
                i.left as u8,
                i.right as u8,
                i.fire as u8,
                i.fire2 as u8,
                i.mouse_joy0dat,
                i.lmb as u8,
                i.potgor_clear
            ),
            Ev::Lines(k) => writeln!(s, "l {k}"),
        };
    }
    s
}

/// FNV-1a over bytes, as the driver computes it.
fn fnv(b: impl IntoIterator<Item = u8>) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for x in b {
        h = (h ^ x as u64).wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// The Rust side: the same output the C driver prints.
fn run_rust(chip0: &[u8], ev: &[Ev]) -> String {
    let mut chip = chip0.to_vec();
    let mut cs = Hardware::new(&chip, 0, Paula::default());
    let mut lines = 0u64;
    let mut out = String::new();
    for e in ev {
        match *e {
            Ev::W(r, v) => cs.write(&mut chip, r, v),
            Ev::R(r, d) => {
                let v = cs.read(r, lines * CYC_PER_LINE + d as u64);
                let _ = writeln!(out, "r {v:04x}");
            }
            Ev::CiaW(n, r, v) => cs.cia_write(n as usize, r, v),
            Ev::CiaR(n, r) => {
                let _ = writeln!(out, "R {:02x}", cs.cia_read(n as usize, r));
            }
            Ev::Poke(a, v) => chip[a as usize] = v,
            Ev::In(i) => cs.input = i,
            Ev::Lines(k) => {
                for _ in 0..k {
                    cs.pass_line(&chip);
                    lines += 1;
                    let before = cs.intreq;
                    let level = cs.ipl();
                    let _ = write!(out, "l {before:04x} {:04x} {level}", cs.intreq);
                    if !cs.audio.is_empty() {
                        let h = fnv(cs.audio.iter().flat_map(|s| s.to_le_bytes()));
                        let _ = write!(out, " a {} {h:016x}", cs.audio.len() / 2);
                        cs.audio.clear();
                    }
                    if cs.frame_done {
                        cs.frame_done = false;
                        let h = fnv(cs.framebuf.iter().flat_map(|p| p.to_le_bytes()));
                        let _ = write!(out, " f {h:016x}");
                    }
                    out.push('\n');
                }
            }
        }
    }
    let b = cs.blits;
    let _ = writeln!(
        out,
        "end {:016x} {} {} {} {} {} {} {} {}",
        fnv(chip.iter().copied()),
        b[0],
        b[1],
        b[2],
        b[3],
        b[4],
        b[5],
        b[6],
        b[7]
    );
    out
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdint.h>
#include "amiga.h"
uint64_t CYC; uint32_t pc_ring[16]; int pc_ring_n;
extern int mouse_lmb; extern uint16_t mouse_joy0dat, amigo_potgor_clear; extern uint64_t blit_class_counts[8];
uint16_t hw_r16(int reg); uint8_t hw_ciar(int n, int reg); void hw_ciaw(int n, int reg, uint8_t v);
static uint64_t fnv(const void *p, size_t n) {
    const uint8_t *b = p; uint64_t h = 0xcbf29ce484222325ull;
    for (size_t i = 0; i < n; i++) h = (h ^ b[i]) * 0x100000001b3ull;
    return h;
}
int main(void) {
    FILE *f = fopen("chip.bin", "rb"); if (!f || fread(chipram, 1, CHIP_SIZE, f) != CHIP_SIZE) return 2; fclose(f);
    chip_reset();
    uint64_t lines = 0; char op;
    while (scanf(" %c", &op) == 1) {
        unsigned a, b, c, d, e, g, h, k, m;
        switch (op) {
        case 'w': scanf("%u %u", &a, &b); chip_poke(a, (uint16_t)b); break;
        case 'r': scanf("%u %u", &a, &b); CYC = lines * CYC_PER_LINE + b; printf("r %04x\n", hw_r16(a)); CYC = lines * CYC_PER_LINE; break;
        case 'W': scanf("%u %u %u", &a, &b, &c); hw_ciaw(a, b, (uint8_t)c); break;
        case 'R': scanf("%u %u", &a, &b); printf("R %02x\n", hw_ciar(a, b)); break;
        case 'm': scanf("%u %u", &a, &b); chipram[a % CHIP_SIZE] = (uint8_t)b; break;
        case 'i': scanf("%u %u %u %u %u %u %u %u %u", &a, &b, &c, &d, &e, &g, &h, &k, &m);
            joy_up = a; joy_down = b; joy_left = c; joy_right = d; joy_fire = e; joy_fire2 = g;
            mouse_joy0dat = (uint16_t)h; mouse_lmb = k; amigo_potgor_clear = (uint16_t)m; break;
        case 'l': scanf("%u", &a);
            for (unsigned i = 0; i < a; i++) {
                tick_line_step(); lines++; CYC = lines * CYC_PER_LINE;
                uint16_t before = intreq; int l = chip_ipl(); printf("l %04x %04x %d", before, intreq, l);
                if (audio_len) { printf(" a %d %016llx", audio_len, (unsigned long long)fnv(audio_buf, (size_t)audio_len * 4)); audio_len = 0; }
                if (chip_frame_done) { chip_frame_done = 0; printf(" f %016llx", (unsigned long long)fnv(framebuf, sizeof framebuf)); }
                printf("\n");
            }
            break;
        }
    }
    printf("end %016llx", (unsigned long long)fnv(chipram, CHIP_SIZE));
    for (int i = 0; i < 8; i++) printf(" %llu", (unsigned long long)blit_class_counts[i]);
    printf("\n");
    return 0;
}
"#;

fn put16(chip: &mut [u8], a: usize, v: u16) {
    chip[a..a + 2].copy_from_slice(&v.to_be_bytes());
}

/// A copper list of (word, word) pairs at `a`, ended by $FFFF,$FFFE.
fn put_list(chip: &mut [u8], a: usize, l: &[(u16, u16)]) {
    for (i, &(w1, w2)) in l.iter().chain(&[(0xffff, 0xfffe)]).enumerate() {
        put16(chip, a + i * 4, w1);
        put16(chip, a + i * 4 + 2, w2);
    }
}

fn noise(chip: &mut [u8], seed: u64, from: usize, to: usize) {
    let mut r = Rng(seed | 1);
    for b in &mut chip[from..to] {
        *b = r.next() as u8;
    }
}

/// A lores 4-plane screen at $8000 with a palette, as a game's copper list sets it up.
fn screen_moves() -> Vec<(u16, u16)> {
    let mut l = vec![
        (0x100, 0x4200),
        (0x08e, 0x2c81),
        (0x090, 0x2cc1),
        (0x092, 0x0038),
        (0x094, 0x00d0),
        (0x108, 0),
        (0x10a, 0),
    ];
    for p in 0..4u16 {
        l.push((0x0e0 + p * 4, 0));
        l.push((0x0e2 + p * 4, 0x8000 + p * 0x1f40));
    }
    for c in 0..16u16 {
        l.push((0x180 + c * 2, (c * 0x111) ^ 0x0f3));
    }
    l
}

fn cop1(a: u16) -> [Ev; 2] {
    [Ev::W(0x080, 0), Ev::W(0x082, a)]
}

fn hand_written() -> Vec<(String, Vec<u8>, Vec<Ev>)> {
    use Ev::*;
    let mut v = Vec::new();

    // a screen with copper interrupts at lines, CPU palette writes and drawing during the tick (progressive compose)
    let mut chip = vec![0u8; CHIP];
    noise(&mut chip, 1, 0x8000, 0x10000);
    let mut l = screen_moves();
    l.extend([
        (0x4001, 0xfffe),
        (0x09c, 0x8010),
        (0x8007, 0xfffe),
        (0x180, 0x0f00),
        (0x09c, 0x8004),
        (0xffdf, 0xfffe),
        (0x2001, 0xfffe),
        (0x180, 0x00f0),
    ]);
    put_list(&mut chip, 0x1000, &l);
    let mut e = vec![W(0x09a, 0xc034)];
    e.extend(cop1(0x1000));
    e.extend([W(0x096, 0x8380), Lines(313)]);
    for k in 0..3 {
        e.extend([Lines(60), W(0x182, 0x0abc + k), W(0x09c, 0x0014)]);
        for i in 0..200u16 {
            e.push(Poke(0x8000 + 40 * (90 + k) + i, 0xff));
        }
        e.extend([Lines(120), W(0x100, 0x3200 + k * 0x1000), Lines(133)]);
    }
    v.push((
        "screen, copper interrupts, drawing during the tick".into(),
        chip,
        e,
    ));

    // COPJMP2 and COP2LC written by the CPU mid-tick (the copper table re-read), and a list that moves COP1LC to a
    // plausible list and, later, to all-zero memory
    let mut chip = vec![0u8; CHIP];
    noise(&mut chip, 2, 0x8000, 0x10000);
    let mut l = screen_moves();
    l.extend([
        (0x084, 0),
        (0x086, 0x1800),
        (0x6001, 0xfffe),
        (0x180, 0x0444),
    ]);
    put_list(&mut chip, 0x1000, &l);
    put_list(
        &mut chip,
        0x1800,
        &[
            (0x180, 0x0f0f),
            (0x9001, 0xfffe),
            (0x09c, 0x8010),
            (0x182, 0x0fff),
            (0xa001, 0xfffe),
            (0x08a, 0),
        ],
    );
    put_list(
        &mut chip,
        0x2000,
        &[
            (0x180, 0x0123),
            (0x080, 0),
            (0x082, 0x3000),
            (0x5001, 0xfffe),
        ],
    );
    let mut e = vec![W(0x09a, 0xc010)];
    e.extend(cop1(0x1000));
    e.extend([W(0x096, 0x8380), Lines(313 + 100), W(0x08a, 0), Lines(50)]);
    e.extend([W(0x086, 0x1810), Lines(30), W(0x088, 0), Lines(133)]);
    e.extend([Lines(80), W(0x084, 0), W(0x086, 0x1800), W(0x08a, 0)]);
    e.extend([Lines(233), W(0x080, 0), W(0x082, 0x2000), Lines(313 * 3)]);
    v.push(("copper jumps and pointer writes".into(), chip, e));

    // blits through the register writes: clear, copy, line, fill; INTREQ bit 6 and level 3
    let mut chip = vec![0u8; CHIP];
    noise(&mut chip, 3, 0, 0x10000);
    let mut e = vec![W(0x09a, 0xc040)];
    let blits: [&[(u16, u16)]; 4] = [
        &[
            (0x040, 0x0100),
            (0x042, 0),
            (0x054, 0),
            (0x056, 0x5000),
            (0x066, 4),
        ],
        &[
            (0x040, 0x09f0),
            (0x042, 0),
            (0x044, 0xffff),
            (0x046, 0xff00),
            (0x050, 0),
            (0x052, 0x9000),
            (0x064, 2),
        ],
        &[
            (0x040, 0x0bca),
            (0x042, 0x0011),
            (0x044, 0xffff),
            (0x046, 0xffff),
            (0x048, 0),
            (0x04a, 0x6000),
            (0x054, 0),
            (0x056, 0x6000),
            (0x052, 0xffe0),
            (0x062, 0xffc0),
            (0x064, 0xff80),
            (0x060, 40),
            (0x066, 40),
            (0x074, 0x8000),
            (0x072, 0xffff),
        ],
        &[(0x040, 0x09f0), (0x042, 0x000a), (0x064, 0), (0x066, 0)],
    ];
    for (i, b) in blits.iter().enumerate() {
        e.extend(b.iter().map(|&(r, x)| W(r, x)));
        e.extend([W(0x058, [0x0a14, 0x0c0a, 0x0802, 0x1014][i]), R(0x01e, 0)]);
        e.extend([Lines(2), W(0x09c, 0x0040), Lines(1)]);
    }
    e.extend([W(0x05c, 3), W(0x05e, 5), R(0x01e, 0), R(0x002, 0), Lines(2)]);
    v.push(("blits".into(), chip, e));

    // Paula: a square wave on channel 0, a saw on 1, AUD2DAT with DMA off, the LED filter switched by CIA-A
    let mut chip = vec![0u8; CHIP];
    for i in 0..64 {
        chip[0x4000 + i] = if i < 32 { 0x60 } else { 0xa0 };
        chip[0x4100 + i] = (i as u8).wrapping_mul(4).wrapping_sub(128);
    }
    let mut e = vec![W(0x09a, 0xc780), CiaW(0, 2, 0x03), CiaW(0, 0, 0x00)];
    for (n, a, per) in [(0u16, 0x4000u16, 300u16), (1, 0x4100, 200)] {
        let b = 0x0a0 + n * 16;
        e.extend([
            W(b, 0),
            W(b + 2, a),
            W(b + 4, 32),
            W(b + 6, per),
            W(b + 8, 64),
        ]);
    }
    e.extend([W(0x096, 0x8203), Lines(150), W(0x09c, 0x0780)]);
    e.extend([
        W(0x0ca, 0x1234),
        CiaW(0, 0, 0x02),
        Lines(313),
        W(0x096, 0x0001),
    ]);
    e.extend([W(0x0b6, 100), W(0x0b8, 40), Lines(200)]);
    v.push(("audio".into(), chip, e));

    // beam position reads at the start of a line, inside it, across lines and across the end of the tick
    let mut e = Vec::new();
    for at in [0u32, 1, 2, 453, 454, 455, 908, 4000, 454 * 313 + 7] {
        e.extend([R(0x004, at), R(0x006, at)]);
    }
    for k in [1u32, 100, 154, 1, 56] {
        e.push(Lines(k));
        for at in [0u32, 200, 453, 455, 454 * 3 + 100] {
            e.extend([R(0x004, at), R(0x006, at)]);
        }
    }
    v.push(("beam reads".into(), vec![0u8; CHIP], e));

    // player input and the other readable registers
    let mut e = vec![
        W(0x09e, 0x9234),
        W(0x096, 0x87ff),
        W(0x09a, 0xc0ff),
        W(0x09c, 0x8123),
    ];
    for k in 0..16u16 {
        let b = |i: u16| k & (1 << i) != 0;
        e.push(In(Input {
            up: b(0),
            down: b(1),
            left: b(2),
            right: b(3),
            fire: b(1),
            fire2: b(2),
            mouse_joy0dat: k * 0x1111,
            lmb: b(0),
            potgor_clear: if b(3) { 0x0400 } else { 0 },
        }));
        for r in (0x000..0x020).step_by(2) {
            e.push(R(r, 0));
        }
        e.extend([CiaR(0, 0), CiaR(1, 0)]);
    }
    v.push(("input and readable registers".into(), vec![0u8; CHIP], e));
    v
}

/// Registers a random CPU write goes to, weighted by what games write.
const POOL: &[u16] = &[
    0x040, 0x042, 0x044, 0x046, 0x048, 0x04a, 0x04c, 0x04e, 0x050, 0x052, 0x054, 0x056, 0x058,
    0x058, 0x05c, 0x05e, 0x060, 0x062, 0x064, 0x066, 0x070, 0x072, 0x074, 0x080, 0x082, 0x084,
    0x086, 0x088, 0x08a, 0x08e, 0x090, 0x092, 0x094, 0x096, 0x096, 0x09a, 0x09c, 0x09c, 0x09e,
    0x0a0, 0x0a2, 0x0a4, 0x0a6, 0x0a8, 0x0aa, 0x0b6, 0x0c8, 0x0da, 0x0e0, 0x0e2, 0x0e6, 0x0f6,
    0x100, 0x102, 0x104, 0x108, 0x10a, 0x120, 0x122, 0x140, 0x180, 0x182, 0x19e, 0x1be,
];

fn random_list(r: &mut Rng, lists: &[u16]) -> Vec<(u16, u16)> {
    let mut l = Vec::new();
    let mut line = 0x20u16;
    for _ in 0..5 + r.below(80) {
        match r.below(100) {
            0..=54 => {
                let reg = POOL[r.below(POOL.len() as u64) as usize];
                let v = match reg {
                    0x082 | 0x086 => lists[r.below(lists.len() as u64) as usize],
                    0x080 | 0x084 => 0,
                    0x088 | 0x08a if r.chance(70) => continue,
                    0x09c => 0x8000 | (r.next() as u16 & 0x07f4),
                    // keep blits out of the copper (the table does not run them) but let the move be read
                    _ => r.next() as u16,
                };
                l.push((reg, v));
            }
            55..=64 => l.push((0x09c, 0x8000 | (1 << r.below(14)) as u16)),
            65..=89 => {
                line = line.wrapping_add(r.below(40) as u16) & 0xff;
                l.push(((line << 8) | (r.next() as u16 & 0xfe) | 1, 0xfffe));
            }
            _ => l.push((((r.next() as u16) & 0xfffe) | 1, (r.next() as u16) | 1)),
        }
    }
    l
}

fn random(seed: u64) -> (Vec<u8>, Vec<Ev>) {
    let mut r = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
    let mut chip = vec![0u8; CHIP];
    noise(&mut chip, r.next(), 0x4000, CHIP);
    let lists = [0x1000u16, 0x1800, 0x2000, 0x3800];
    for (i, &a) in lists.iter().enumerate() {
        let mut l = if i == 0 { screen_moves() } else { Vec::new() };
        l.extend(random_list(&mut r, &lists));
        put_list(&mut chip, a as usize, &l[..l.len().min(500)]);
    }
    let mut e = vec![Ev::W(0x09a, 0xc000 | (r.next() as u16 & 0x3fff))];
    e.extend(cop1(lists[0]));
    e.push(Ev::W(0x096, 0x8000 | (r.next() as u16 & 0x03ff) | 0x0300));
    let total = 300 + r.below(900) as u32;
    let mut lines = 0;
    while lines < total {
        match r.below(100) {
            0..=39 => {
                let reg = if r.chance(5) {
                    r.below(0x100) as u16 * 2
                } else {
                    POOL[r.below(POOL.len() as u64) as usize]
                };
                let v = match reg {
                    // small blits mostly, so a run stays quick (an ECS blit can be 32768 x 2048 words)
                    0x058 if r.chance(80) => (r.below(64) as u16) << 6 | r.below(8) as u16,
                    0x05c => r.below(512) as u16,
                    0x05e => (r.next() as u16 & 0x3f) >> if r.chance(80) { 2 } else { 0 },
                    0x082 | 0x086 if r.chance(70) => lists[r.below(4) as usize],
                    0x080 | 0x084 if r.chance(70) => 0,
                    _ => r.next() as u16,
                };
                e.push(Ev::W(reg, v));
            }
            40..=54 => {
                let reg = if r.chance(50) {
                    [0x004, 0x006, 0x01e, 0x01c, 0x002][r.below(5) as usize]
                } else {
                    r.below(0x100) as u16 * 2
                };
                e.push(Ev::R(reg, r.below(1500) as u32));
            }
            55..=62 => e.push(Ev::CiaW(
                r.below(2) as u8,
                r.below(16) as u8,
                r.next() as u8,
            )),
            63..=68 => e.push(Ev::CiaR(r.below(2) as u8, r.below(16) as u8)),
            69..=78 => {
                let a = 0x4000 + r.below(CHIP as u64 - 0x4000) as u16;
                let n = 1 + r.below(64) as u16;
                for k in 0..n {
                    e.push(Ev::Poke(a.saturating_add(k), r.next() as u8));
                }
            }
            79..=81 => {
                let x = r.next();
                let b = |i: u32| x & (1 << i) != 0;
                e.push(Ev::In(Input {
                    up: b(0),
                    down: b(1),
                    left: b(2),
                    right: b(3),
                    fire: b(4),
                    fire2: b(5),
                    mouse_joy0dat: (x >> 16) as u16,
                    lmb: b(6),
                    potgor_clear: (x >> 32) as u16 & 0x5500,
                }));
            }
            _ => {
                let span = if r.chance(15) { 313 } else { 40 };
                let k = 1 + r.below(span) as u32;
                lines += k;
                e.push(Ev::Lines(k));
            }
        }
    }
    (chip, e)
}

#[test]
fn hardware_matches_c_intentrt() {
    if !have("gcc") {
        eprintln!("skipped: needs gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-chipset-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();
    let rt = root.join("kit/amigo/port/runtime");
    fs::write(work.join("driver.c"), C_DRIVER).unwrap();
    let exe = work.join("c_chipset");
    // no float contraction, as paula_diff: the mix must round as the Rust does
    run_ok(
        Command::new("gcc")
            .args(["-O1", "-w", "-ffp-contract=off"])
            .arg(format!("-DCHIP_SIZE={CHIP}"))
            .arg("-I")
            .arg(&rt)
            .arg("-o")
            .arg(&exe)
            .arg(work.join("driver.c"))
            .arg(rt.join("intentrt.c"))
            .arg("-lm"),
    );

    let mut sc = hand_written();
    sc.extend((0..80).map(|i| {
        let (chip, e) = random(i);
        (format!("random {i}"), chip, e)
    }));
    let (mut lines, mut frames, mut samples, mut reads, mut levels) = (0, 0, 0, 0, [0usize; 7]);
    let mut blits = [0u64; 8];
    for (name, chip, ev) in &sc {
        fs::write(work.join("chip.bin"), chip).unwrap();
        // the C's environment switches stay at their defaults, as the Rust has them
        let mut child = Command::new(&exe)
            .current_dir(&work)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        // Feed stdin from a thread while wait_with_output drains stdout. Writing all of stdin first deadlocks as
        // soon as the C driver fills its stdout pipe: measured on Ryzen 29 Oct 2026, where a user over
        // fs.pipe-user-pages-soft gets 8 KiB pipes instead of 64 KiB and this test hung for the full gate timeout.
        let mut stdin = child.stdin.take().unwrap();
        let input = text(ev);
        let feeder = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
        let out = child.wait_with_output().unwrap();
        feeder.join().unwrap().unwrap();
        assert!(out.status.success(), "{name}: C driver failed");
        let c = String::from_utf8(out.stdout).unwrap();
        let rust = run_rust(chip, ev);
        if rust != c {
            let k = rust
                .lines()
                .zip(c.lines())
                .position(|(a, b)| a != b)
                .unwrap_or(rust.lines().count().min(c.lines().count()));
            panic!(
                "{name}: output line {k} differs: rust {:?} c {:?}",
                rust.lines().nth(k),
                c.lines().nth(k)
            );
        }
        for l in c.lines() {
            let w: Vec<&str> = l.split(' ').collect();
            match w[0] {
                "l" => {
                    lines += 1;
                    levels[w[3].parse::<usize>().unwrap()] += 1;
                    if let Some(i) = w.iter().position(|&x| x == "a") {
                        samples += w[i + 1].parse::<usize>().unwrap();
                    }
                    frames += w.contains(&"f") as usize;
                }
                "end" => {
                    for (b, x) in blits.iter_mut().zip(&w[2..]) {
                        *b += x.parse::<u64>().unwrap();
                    }
                }
                _ => reads += 1,
            }
        }
    }
    eprintln!(
        "hardware_diff: {} scenarios, {lines} lines, {frames} frames, {samples} stereo samples, {reads} reads, \
         blits by class 0-7 {blits:?}, lines by level 0-6 {levels:?}: identical",
        sc.len()
    );
}
