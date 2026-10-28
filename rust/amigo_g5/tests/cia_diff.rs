//! M5 part 4 differential test: the Rust CIAs and interrupt level (`cia.rs`) against `runtime/intentrt.c`.
//!
//! The C side is the whole of intentrt.c, compiled with gcc and linked with a small driver written here (it defines
//! only the scheduler clock and the PC ring intentrt.c expects from cpu_native.c). Each scenario is synthetic: a
//! stream of CIA writes and reads, PAL lines (the real `tick_line_step`), key presses, fire buttons and INTENA/INTREQ
//! writes through `chip_poke`, starting from `chip_reset`. Every CIA read value, and INTREQ and `chip_ipl` after
//! every line, must be identical. Chip RAM stays zero and DMA off, so the copper and Paula raise nothing; the Rust
//! side adds VERTB at the end of each tick as `tick_line_step` does. Skips cleanly without gcc.

use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::process::{Command, Stdio};

use amigo_g5::cia::{ipl, Buttons, Cias};

mod common;
use common::{have, repo_root, run_ok};

const LINES_PER_FRAME: u32 = 313;

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
    /// CIA n, register, value.
    Write(u8, u8, u8),
    /// CIA n, register.
    Read(u8, u8),
    /// This many lines.
    Lines(u32),
    /// Raw key code, pressed.
    Key(u8, bool),
    /// Port 2 fire, left mouse button.
    Buttons(bool, bool),
    /// INTENA ($09A) or INTREQ ($09C) write.
    Custom(u16, u16),
}

fn text(ev: &[Ev]) -> String {
    let mut s = String::new();
    for e in ev {
        let _ = match *e {
            Ev::Write(n, r, v) => writeln!(s, "w {n} {r} {v}"),
            Ev::Read(n, r) => writeln!(s, "r {n} {r}"),
            Ev::Lines(k) => writeln!(s, "l {k}"),
            Ev::Key(c, d) => writeln!(s, "k {c} {}", d as u8),
            Ev::Buttons(f, l) => writeln!(s, "b {} {}", f as u8, l as u8),
            Ev::Custom(r, v) => writeln!(s, "c {r} {v}"),
        };
    }
    s
}

/// The Rust side: the same output the C driver prints.
fn run_rust(ev: &[Ev]) -> String {
    let mut c = Cias::new();
    let (mut intena, mut intreq, mut line, mut buttons) = (0u16, 0u16, 0u32, Buttons::default());
    let mut out = String::new();
    for e in ev {
        match *e {
            Ev::Write(n, r, v) => c.write(n as usize, r, v),
            Ev::Read(n, r) => {
                let _ = writeln!(out, "r {:02x}", c.read(n as usize, r, buttons));
            }
            Ev::Lines(k) => {
                for _ in 0..k {
                    c.pass_line();
                    line += 1;
                    if line >= LINES_PER_FRAME {
                        line = 0;
                        intreq |= 0x0020;
                        c.frame_end();
                    }
                    intreq |= c.intreq_bits();
                    let level = ipl(intena, intreq);
                    let _ = writeln!(out, "l {intreq:04x} {level}");
                }
            }
            Ev::Key(code, down) => c.push_key(code, down),
            Ev::Buttons(f, l) => buttons = Buttons { fire: f, lmb: l },
            Ev::Custom(r, v) => {
                let x = if r == 0x9a { &mut intena } else { &mut intreq };
                if v & 0x8000 != 0 {
                    *x |= v & 0x7fff;
                } else {
                    *x &= !v;
                }
            }
        }
    }
    out
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdint.h>
#include "amiga.h"
uint64_t CYC; uint32_t pc_ring[16]; int pc_ring_n;
extern int mouse_lmb;
uint8_t hw_ciar(int n, int reg); void hw_ciaw(int n, int reg, uint8_t v);
int main(void) {
    char op; chip_reset();
    while (scanf(" %c", &op) == 1) {
        unsigned a, b, c;
        switch (op) {
        case 'w': scanf("%u %u %u", &a, &b, &c); hw_ciaw(a, b, (uint8_t)c); break;
        case 'r': scanf("%u %u", &a, &b); printf("r %02x\n", hw_ciar(a, b)); break;
        case 'l': scanf("%u", &a);
            for (unsigned i = 0; i < a; i++) { tick_line_step(); int l = chip_ipl(); printf("l %04x %d\n", intreq, l); }
            break;
        case 'k': scanf("%u %u", &a, &b); kbd_push((uint8_t)a, b); break;
        case 'b': scanf("%u %u", &a, &b); joy_fire = a; mouse_lmb = b; break;
        case 'c': scanf("%u %u", &a, &b); chip_poke(a, (uint16_t)b); break;
        }
    }
    return 0;
}
"#;

fn hand_written() -> Vec<(String, Vec<Ev>)> {
    use Ev::*;
    let rd = |n: u8| -> Vec<Ev> { (0..16).map(|r| Read(n, r)).collect() };
    let mut v: Vec<(String, Vec<Ev>)> = Vec::new();
    // reset state of every register, both CIAs
    let mut e = rd(0);
    e.extend(rd(1));
    v.push(("reset reads".into(), e));
    // CIA-B timer A continuous at latches 0, 1, 44, 45, 46, 300: the level 6 music interrupt
    for latch in [0u16, 1, 44, 45, 46, 300] {
        let mut e = vec![
            Custom(0x9a, 0xe000),
            Write(1, 13, 0x81),
            Write(1, 4, latch as u8),
            Write(1, 5, (latch >> 8) as u8),
            Write(1, 14, 0x01),
        ];
        for _ in 0..12 {
            e.extend([Lines(7), Read(1, 4), Read(1, 5), Read(1, 13)]);
            e.push(Custom(0x9c, 0x2000));
        }
        v.push((format!("timer A continuous latch {latch}"), e));
    }
    // one-shot started by the high byte, restarted, and CRA start with a one-shot already counting
    v.push((
        "one-shot".into(),
        vec![
            Write(1, 13, 0x83),
            Write(1, 14, 0x08),
            Write(1, 4, 0x20),
            Write(1, 5, 0x01),
            Read(1, 14),
            Lines(5),
            Read(1, 14),
            Read(1, 4),
            Read(1, 5),
            Read(1, 13),
            Write(1, 15, 0x08),
            Write(1, 6, 0x10),
            Write(1, 7, 0x00),
            Lines(1),
            Read(1, 13),
            Read(1, 15),
            Write(1, 5, 0x00),
            Lines(3),
            Read(1, 13),
        ],
    ));
    // timer B counting timer A underflows (CRB bit 6) is not counted; force load strobes; a high-byte write while
    // running does not reload
    v.push((
        "timer B and strobes".into(),
        vec![
            Write(0, 13, 0x82),
            Write(0, 6, 0x40),
            Write(0, 7, 0x00),
            Write(0, 15, 0x41),
            Lines(10),
            Read(0, 6),
            Read(0, 13),
            Write(0, 15, 0x01),
            Lines(2),
            Read(0, 6),
            Write(0, 7, 0x10),
            Read(0, 6),
            Read(0, 7),
            Write(0, 15, 0x11),
            Read(0, 15),
            Read(0, 6),
            Read(0, 7),
            Write(0, 4, 0x34),
            Write(0, 5, 0x12),
            Write(0, 14, 0x10),
            Read(0, 4),
            Read(0, 5),
            Read(0, 14),
            Lines(3),
            Read(0, 13),
        ],
    ));
    // TOD: CIA-B counts lines, CIA-A counts ticks; alarm on both; the latch on a high read; stopped by a high write
    v.push((
        "time of day".into(),
        vec![
            Custom(0x9a, 0xe008),
            Write(1, 13, 0x84),
            Write(1, 15, 0x80),
            Write(1, 10, 0),
            Write(1, 9, 0),
            Write(1, 8, 100),
            Write(1, 15, 0x00),
            Write(1, 10, 0),
            Write(1, 9, 0),
            Write(1, 8, 0),
            Lines(99),
            Read(1, 13),
            Lines(2),
            Read(1, 13),
            Read(1, 10),
            Lines(5),
            Read(1, 9),
            Read(1, 8),
            Read(1, 8),
            Write(1, 10, 0x12),
            Lines(4),
            Read(1, 8),
            Write(1, 8, 0x05),
            Lines(4),
            Read(1, 8),
            Write(0, 13, 0x84),
            Write(0, 15, 0x80),
            Write(0, 8, 2),
            Write(0, 15, 0x00),
            Lines(700),
            Read(0, 13),
            Read(0, 8),
            Read(0, 9),
            Read(0, 10),
        ],
    ));
    // keyboard: bytes, handshake, the gap, SPMODE output blocking receive, overflow, and the timeout
    let mut e = vec![Custom(0x9a, 0xc008)];
    for k in 0..70u8 {
        e.push(Key(k, k % 3 != 0));
    }
    for i in 0..30 {
        e.extend([Lines(1 + i % 4), Read(0, 12), Read(0, 13)]);
        if i % 5 == 4 {
            e.push(Write(0, 14, 0x40));
            e.extend([Lines(2), Read(0, 12)]);
        }
        e.push(Write(0, 14, 0x40));
        e.push(Write(0, 14, 0x00));
        e.push(Custom(0x9c, 0x0008));
    }
    // the timeout, probed line by line around its end so a byte arriving one line early or late shows
    e.push(Lines(17870));
    for _ in 0..20 {
        e.extend([Lines(1), Read(0, 12)]);
    }
    // drain the rest: the 64th key must have been dropped by the full link
    for _ in 0..70 {
        e.extend([
            Write(0, 14, 0x40),
            Write(0, 14, 0x00),
            Lines(9),
            Read(0, 12),
        ]);
    }
    v.push(("keyboard".into(), e));
    // serial output and the ICR mask
    v.push((
        "serial output and ICR mask".into(),
        vec![
            Write(0, 14, 0x40),
            Write(0, 12, 0x5a),
            Read(0, 13),
            Write(0, 12, 0xa5),
            Write(0, 13, 0x08),
            Lines(1),
            Read(0, 13),
            Write(0, 13, 0x9f),
            Write(0, 13, 0x05),
            Write(0, 12, 0x11),
            Lines(1),
            Read(0, 13),
            Read(0, 12),
        ],
    ));
    // fire buttons and port reads through the data direction registers
    let mut e = Vec::new();
    for (f, l) in [(false, false), (true, false), (false, true), (true, true)] {
        e.push(Buttons(f, l));
        for (ddra, pra) in [(0x00u8, 0x00u8), (0x03, 0xfc), (0xc3, 0x00), (0xff, 0x5a)] {
            e.extend([
                Write(0, 2, ddra),
                Write(0, 0, pra),
                Write(1, 2, ddra),
                Write(1, 0, pra),
                Write(0, 3, !ddra),
                Write(0, 1, pra),
                Read(0, 0),
                Read(1, 0),
                Read(0, 1),
            ]);
        }
    }
    v.push(("buttons and ports".into(), e));
    // the interrupt level: priorities, the master bit, INTREQ cleared while the CIA still holds it
    v.push((
        "interrupt level".into(),
        vec![
            Write(0, 13, 0x81),
            Write(0, 4, 10),
            Write(0, 5, 0),
            Write(0, 14, 0x01),
            Write(1, 13, 0x82),
            Write(1, 6, 50),
            Write(1, 7, 0),
            Write(1, 15, 0x01),
            Lines(3),
            Custom(0x9a, 0x8008),
            Lines(1),
            Custom(0x9a, 0xc000),
            Lines(1),
            Custom(0x9a, 0x8020),
            Lines(1),
            Custom(0x9c, 0x0008),
            Lines(1),
            Read(0, 13),
            Custom(0x9c, 0x0008),
            Lines(1),
            Custom(0x9a, 0xa000),
            Lines(2),
            Custom(0x9c, 0x8004),
            Custom(0x9a, 0x0028),
            Lines(1),
            Custom(0x9a, 0x8784),
            Custom(0x9c, 0x8580),
            Lines(1),
            Custom(0x9c, 0x7fff),
            Read(0, 13),
            Read(1, 13),
            Lines(320),
        ],
    ));
    v
}

fn random(seed: u64) -> Vec<Ev> {
    let mut r = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
    let mut e = vec![Ev::Custom(0x9a, 0xc000 | (r.next() as u16 & 0x3fff))];
    let total = 300 + r.below(1200) as u32;
    let mut lines = 0;
    while lines < total {
        match r.below(100) {
            0..=34 => {
                let n = r.below(2) as u8;
                let reg = r.below(16) as u8;
                let mut v = r.next() as u8;
                // keep timers busy: small latches often, and CRA/CRB start bits mostly on
                if (4..=7).contains(&reg) && r.chance(60) {
                    v = if reg & 1 == 1 { 0 } else { r.below(120) as u8 };
                }
                if (reg == 14 || reg == 15) && r.chance(70) {
                    v |= 1;
                }
                if reg == 13 && r.chance(60) {
                    v |= 0x80;
                }
                e.push(Ev::Write(n, reg, v));
            }
            35..=59 => {
                // the game's interrupt handlers read ICR most of all
                let reg = if r.chance(40) { 13 } else { r.below(16) as u8 };
                e.push(Ev::Read(r.below(2) as u8, reg));
            }
            60..=79 => {
                let span = if r.chance(10) { 400 } else { 30 };
                let k = 1 + r.below(span) as u32;
                lines += k;
                e.push(Ev::Lines(k));
            }
            80..=86 => e.push(Ev::Key(r.next() as u8, r.chance(50))),
            87..=90 => e.push(Ev::Buttons(r.chance(50), r.chance(50))),
            _ => {
                let reg = if r.chance(70) { 0x9c } else { 0x9a };
                let v = if reg == 0x9c && r.chance(60) {
                    0x2008
                } else {
                    r.next() as u16
                };
                e.push(Ev::Custom(reg, v));
            }
        }
    }
    e
}

#[test]
fn cia_matches_c_intentrt() {
    if !have("gcc") {
        eprintln!("skipped: needs gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-cia-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();
    let rt = root.join("kit/amigo/port/runtime");
    fs::write(work.join("driver.c"), C_DRIVER).unwrap();
    let exe = work.join("c_cia");
    run_ok(
        Command::new("gcc")
            .args(["-O1", "-w", "-DCHIP_SIZE=65536", "-I"])
            .arg(&rt)
            .arg("-o")
            .arg(&exe)
            .arg(work.join("driver.c"))
            .arg(rt.join("intentrt.c"))
            .arg("-lm"),
    );

    let mut sc = hand_written();
    sc.extend((0..120).map(|i| (format!("random {i}"), random(i))));
    let (mut lines, mut reads, mut irq_lines, mut levels) = (0usize, 0usize, 0usize, [0usize; 7]);
    for (name, ev) in &sc {
        let mut child = Command::new(&exe)
            .current_dir(&work)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(text(ev).as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success(), "{name}: C driver failed");
        let c = String::from_utf8(out.stdout).unwrap();
        let rust = run_rust(ev);
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
            if let Some(rest) = l.strip_prefix("l ") {
                lines += 1;
                let (bits, level) = rest.split_once(' ').unwrap();
                if u16::from_str_radix(bits, 16).unwrap() & 0x2008 != 0 {
                    irq_lines += 1;
                }
                levels[level.parse::<usize>().unwrap()] += 1;
            } else {
                reads += 1;
            }
        }
    }
    eprintln!(
        "cia_diff: {} scenarios, {lines} lines, {reads} CIA reads, {irq_lines} lines with a CIA interrupt raised, \
         lines by level 0-6 {levels:?}: identical",
        sc.len()
    );
}
