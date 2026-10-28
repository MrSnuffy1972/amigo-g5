//! M2 differential test: the Rust emitter (`m68k2c.py --emit rust`) against the C translation.
//!
//! Each synthetic program (hand-encoded here, no game data) goes through one translator run that writes both the
//! Rust (`translated.rs`) and the one-function-per-instruction C (`--single`). Both are run from the same start
//! state to the same stop address, and D0-D7, A0-A7, SR, PC, the cycle count and every changed RAM byte are
//! compared. Skips cleanly when python3 or gcc is missing.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BASE: u32 = 0x20000;
const DATA: u32 = 0x21000; // data window, filled with a fixed pattern
const IMAGE_LEN: usize = 0x2000;
const MAX_STEPS: u64 = 5000;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn have(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Programs: (name, 16-bit words). The last word is always `bra.s *` ($60FE), the stop address.
fn programs() -> Vec<(&'static str, Vec<u16>)> {
    let end = 0x60FE;
    vec![
        // moveq/add/subq/bne loop from tests/synthetic: d0 = 11, d1 = 0
        ("loop", vec![0x7005, 0x7203, 0xD081, 0x5381, 0x66FA, end]),
        (
            "move_modes",
            vec![
                0x203C, 0x1234, 0x5678, // move.l #$12345678,d0
                0x41F9, 0x0002, 0x1000, // lea $21000,a0
                0x43F9, 0x0002, 0x1100, // lea $21100,a1
                0x2340, 0x0010, // move.l d0,16(a1)
                0x3029, 0x0010, // move.w 16(a1),d0 ... reads the high word just written
                0x1A18,
                0x1A18, // move.b (a0)+,d5 twice (keeps a0 even for the word access below)
                0x2F00, // move.l d0,-(a7)
                0x221F, // move.l (a7)+,d1
                0x7204, // moveq #4,d1
                0x3030, 0x1002, // move.w 2(a0,d1.w),d0
                0x4BE9, 0x0004, // lea 4(a1),a5
                0x2A7C, 0x0002, 0x1010, // movea.l #$21010,a5
                0x4850, // pea (a0)
                0x4840, 0x4880, 0x48C0, // swap d0 / ext.w d0 / ext.l d0
                0xC141, // exg d0,d1
                0x4281, 0x4A40, 0x4640, 0x4480, // clr.l d1 / tst.w d0 / not.w d0 / neg.l d0
                0x1D40, 0xFFFF, // move.b d0,-1(a6): a6 is a data pointer in the start state
                end,
            ],
        ),
        (
            "arith",
            vec![
                0xD081, 0x9640, 0xD0A9, 0x0004, 0xD199, 0xD6FC, 0x0010,
                0x95C8, // add/sub/adda/suba forms
                0x0640, 0x0123, 0x0481, 0x0001, 0x0000, 0x0C40,
                0x7FFF, // addi.w / subi.l / cmpi.w
                0xB081, 0xB0FC, 0xFFFF, 0xB308,
                0xDD41, // cmp.l / cmpa.w #-1 / cmpm.b / addx.w
                0xB340, 0xC240, 0x8280, 0x0240, 0x00FF, 0x0001,
                0x0080, // eor.w / and.w / or.l / andi.w / ori.b
                0x5E80, 0x5788, 0x5319, // addq.l #7,d0 / subq.l #3,a0 / subq.b #1,(a1)+
                end,
            ],
        ),
        (
            "shifts",
            vec![
                0xE348, 0xE258, 0xE0A9, 0xE5A1, 0xE988, 0xE2D0,
                0xE191, // lsl/ror/lsr/asl/lsl/mem lsr/roxl
                0xE840, 0xE641, 0xE4A2,
                0xE7B3, // asr.w #4,d0 / roxr.w #3,d1 / roxr.l d2,d2 / rol.l d3,d3
                end,
            ],
        ),
        (
            "bits_bcd_ccr",
            vec![
                0x0801, 0x0003, 0x08C2, 0x0005, 0x0101, 0x0890, 0x0001,
                0x0143, // btst/bset/btst/bclr/bchg
                0xC101, 0x8101, 0x4800, 0x4AD0, // abcd / sbcd / nbcd d0 / tas (a0)
                0x44C0, 0x40C1, 0x003C, 0x0010, // move d0,ccr / move sr,d1 / ori #$10,ccr
                0x57C2, 0x56C3, // seq d2 / sne d3
                end,
            ],
        ),
        (
            "movem_link",
            vec![
                0x4BF9, 0x0002, 0x1200, // lea $21200,a5
                0x2E4D, // movea.l a5,a7
                0x48E7, 0xC0C0, // movem.l d0-d1/a0-a1,-(a7)
                0x4CDF, 0x0303, // movem.l (a7)+,d0-d1/a0-a1
                0x4E56, 0xFFF8, // link a6,#-8
                0x2D40, 0xFFFC, // move.l d0,-4(a6)
                0x4E5E, // unlk a6
                end,
            ],
        ),
        (
            "calls",
            vec![
                0x6100, 0x0010, // bsr.w sub1 (+16 from $2)
                0x4EB9, 0x0002, 0x0016, // jsr $20016 (sub2)
                0x60FE, // stop is at $20000+10; see below
                0x4E71, 0x4E71, 0x4E71, 0x4E71, // padding
                0x5280, 0x4E75, // sub1 at $20012: addq.l #1,d0 / rts
                0x5281, 0x4E75, // sub2 at $20016
            ],
        ),
        (
            "branches",
            vec![
                0x7005, 0x7200, // moveq #5,d0 / moveq #0,d1
                0x5241, 0x51C8, 0xFFFC, // loop: addq.w #1,d1 / dbra d0,loop
                0xB27C, 0x0006, 0x6706, // cmp.w #6,d1 / beq +6
                0x7E63, 0x7C63, // (skipped when equal) moveq #99,d7 / d6
                0x6A02, 0x7A01, // bpl +2 / moveq #1,d5 (skipped when N=0)
                0x6B02, 0x7C09, // bmi +2 / moveq #9,d6
                0x6202, 0x7801, // bhi +2 / moveq #1,d4
                0x6D02, 0x7601, // blt +2 / moveq #1,d3
                0x6000, 0x0002, // bra.w +2
                0x7401, // moveq #1,d2 (skipped)
                end,
            ],
        ),
        (
            "mul_div",
            vec![
                0xC0C1, 0xC7C2, // mulu.w d1,d0 / muls.w d2,d3 (-32768 * -1)
                0xC9FC, 0xFFFB, 0xCED0, // muls.w #-5,d4 / mulu.w (a0),d7
                0x80FC, 0x0007, // divu.w #7,d0
                0x87FC,
                0xFFFD, // divs.w #-3,d3 (negative quotient, remainder takes the dividend's sign)
                0x8AFC, 0x0001, // divu.w #1,d5: quotient overflows, V set, d5 unchanged
                0x89C6, // divs.w d6,d4
                0x83FC,
                0x0002, // divs.w #2,d1: overflows (never $80000000 / -1, which is undefined in the C)
                0x8BFC, 0xFED4, // divs.w #-300,d5: -1 / -300 = 0 remainder -1
                end,
            ],
        ),
        (
            // One handler at $20002 (addq.w #1,d6 / rte) for every vector, so d6 counts the exceptions taken.
            "exceptions",
            vec![
                0x6004, 0x5246, 0x4E73, // bra.s over / handler: addq.w #1,d6 / rte
                0x4BF9, 0x0002, 0x0002, // lea $20002,a5
                0x21CD, 0x0014, 0x21CD,
                0x0018, // move.l a5,$14.w (divide by zero) / $18.w (chk)
                0x21CD, 0x001C, 0x21CD, 0x008C, // move.l a5,$1C.w (trapv) / $8C.w (trap #3)
                0x4E60, 0x4E69, // move a0,usp / move usp,a1
                0x8EFC, 0x0000, // divu.w #0,d7: vector 5
                0x41BC, 0x000A, // chk.w #10,d0: in bounds, no exception
                0x41BC, 0x0002, // chk.w #2,d0: above the bound, vector 6 with N clear
                0x45BC, 0x0002, // chk.w #2,d2: negative, vector 6 with N set
                0x44FC, 0x0002, 0x4E76, // move #2,ccr / trapv: vector 7
                0x44FC, 0x0000, 0x4E76, // move #0,ccr / trapv: no exception
                0x027C,
                0xDFFF, // andi.w #$DFFF,sr: user mode, A7 and the other stack pointer swap
                0x4E43,
                0x4E43, // trap #3 twice from user mode: supervisor stack, rte back to user mode
                0x487A, 0x0008, // pea 10(pc): the address after rtr
                0x3F3C, 0x001F, 0x4E77, // move.w #$1F,-(a7) / rtr
                end,
            ],
        ),
        (
            "movep_stop",
            vec![
                0x05C9, 0x0000, // movep.l d2,0(a1)
                0x0F89, 0x0009, // movep.w d7,9(a1)
                0x0909, 0x0001, // movep.w 1(a1),d4
                0x0B49, 0x0000, // movep.l 0(a1),d5
                0x4E72,
                0x2700, // stop #$2700: sets the stopped flag; the single build carries on
                0x4E70, 0x4E71, // reset / nop
                end,
            ],
        ),
    ]
}

/// A fixed, readable data pattern (xorshift) so memory operands see non-zero bytes.
fn image_for(words: &[u16]) -> Vec<u8> {
    let mut img = vec![0u8; IMAGE_LEN];
    for (i, w) in words.iter().enumerate() {
        img[i * 2..i * 2 + 2].copy_from_slice(&w.to_be_bytes());
    }
    let mut x: u32 = 0x1234_5678;
    for b in &mut img[0x1000..0x1400] {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *b = (x >> 8) as u8;
    }
    img
}

fn start_regs() -> ([u32; 8], [u32; 8]) {
    let d = [
        0x0000_0003,
        0x8000_00FF,
        0x1234_FFFF,
        0x0000_8000,
        0x7FFF_FFFF,
        0xFFFF_FFFF,
        0x0000_0001,
        0x00FF_00FF,
    ];
    let a = [
        DATA,
        DATA + 0x100,
        DATA + 0x20,
        DATA + 0x30,
        DATA + 0x40,
        DATA + 0x50,
        DATA + 0x80,
        0x30000,
    ];
    (d, a)
}

fn case_text(stop: u32) -> String {
    let (d, a) = start_regs();
    let mut s = format!("{stop:x} {MAX_STEPS}\n");
    for v in d.iter().chain(a.iter()) {
        let _ = writeln!(s, "{v:x}");
    }
    s
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cpu.h"
#include "boot.h"
#include "run.h"
typedef struct { uint32_t pc; int (*fn)(void); const char *name; int abs_hw; } single_t;
extern const single_t singles[]; extern const int n_singles;
cpu_t cpu; uint64_t CYC, EVT;
static uint8_t ram[0x80000], init[0x80000];
uint32_t mem_rb(uint32_t a) { return ram[a & 0x7ffff]; }
uint32_t mem_rw(uint32_t a) { return (ram[a & 0x7ffff] << 8) | ram[(a + 1) & 0x7ffff]; }
uint32_t mem_rl(uint32_t a) { return (mem_rw(a) << 16) | mem_rw(a + 2); }
void mem_wb(uint32_t a, uint32_t v) { ram[a & 0x7ffff] = (uint8_t)v; }
void mem_ww(uint32_t a, uint32_t v) { mem_wb(a, v >> 8); mem_wb(a + 1, v); }
void mem_wl(uint32_t a, uint32_t v) { mem_ww(a, v >> 16); mem_ww(a + 2, v); }
/* the same as cpu_exception in runtime/cpu_native.c (that file needs the whole runtime, so it is not linked here) */
uint32_t cpu_exception(int vec, uint32_t pc) {
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc);
    A[7] -= 2; mem_ww(A[7], sr);
    CYC += 34;
    return mem_rl((uint32_t)vec * 4);
}
int main(int argc, char **argv) {
    (void)argc;
    FILE *f = fopen(argv[1], "rb"); size_t n = fread(ram + BASE_ADDR, 1, 0x2000, f); (void)n; fclose(f);
    memcpy(init, ram, sizeof ram);
    unsigned stop; unsigned long long maxs; unsigned v[16];
    f = fopen(argv[2], "r"); if (fscanf(f, "%x %llu", &stop, &maxs) != 2) return 2;
    for (int i = 0; i < 16; i++) if (fscanf(f, "%x", &v[i]) != 1) return 2;
    fclose(f);
    for (int i = 0; i < 8; i++) { cpu.dr[i] = v[i]; cpu.ar[i] = v[8 + i]; }
    cpu.PC = BASE_ADDR; cpu.S = 1; cpu.IM = 7; cpu.OSP = 0x28000;
    unsigned long long steps = 0; const char *why = "ok";
    while (cpu.PC != stop && steps < maxs) {
        int k = -1; for (int i = 0; i < n_singles; i++) if (singles[i].pc == cpu.PC) { k = i; break; }
        if (k < 0) { why = "untranslated"; break; }
        singles[k].fn(); steps++;
    }
    printf("STOP %s STEPS %llu\n", why, steps);
    for (int i = 0; i < 8; i++) printf("D%d %x\n", i, cpu.dr[i]);
    for (int i = 0; i < 8; i++) printf("A%d %x\n", i, cpu.ar[i]);
    printf("SR %x PC %x CYC %llu OSP %x STOPPED %d\n", get_sr(), cpu.PC, (unsigned long long)CYC, cpu.OSP, cpu.stopped);
    for (uint32_t a = 0; a < sizeof ram; a++) if (ram[a] != init[a]) printf("M %x %x\n", a, ram[a]);
    return 0;
}
"#;

const RUST_MAIN: &str = r#"
use amigo_g5::cpu::{run_translated, Cpu, Stop};
use amigo_g5::mem::GuestMemory;
use std::fmt::Write;
type Step = fn(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let steps_tab: Vec<Step> = vec![__STEP_TABLE__];
    let n: usize = args[1].parse().unwrap();
    let dir = &args[2];
    let bin = std::fs::read(format!("{dir}/c{n}.bin")).unwrap();
    let txt = std::fs::read_to_string(format!("{dir}/c{n}.txt")).unwrap();
    let mut it = txt.split_whitespace();
    let stop = u32::from_str_radix(it.next().unwrap(), 16).unwrap();
    let maxs: u64 = it.next().unwrap().parse().unwrap();
    let mut v = [0u32; 16];
    for x in v.iter_mut() { *x = u32::from_str_radix(it.next().unwrap(), 16).unwrap(); }
    let mut mem = GuestMemory::new(&[(0, 0x80000)]).unwrap();
    mem.write_bytes(BASE_ADDR, &bin).unwrap();
    let init = mem.read_bytes(0, 0x80000).unwrap().to_vec();
    let mut cpu = Cpu::default();
    cpu.d.copy_from_slice(&v[..8]);
    cpu.a.copy_from_slice(&v[8..]);
    cpu.pc = BASE_ADDR; cpu.s = 1; cpu.im = 7; cpu.osp = 0x28000;
    let step = steps_tab[n];
    let (why, steps) = match run_translated(&mut cpu, &mut mem, stop, maxs, step) {
        Ok(s) => ("ok", s),
        Err(Stop::Untranslated(_)) => ("untranslated", 0),
        Err(e) => panic!("{e:?}"),
    };
    let mut o = String::new();
    let _ = writeln!(o, "STOP {why} STEPS {}", if why == "ok" { steps } else { 0 });
    for i in 0..8 { let _ = writeln!(o, "D{i} {:x}", cpu.d[i]); }
    for i in 0..8 { let _ = writeln!(o, "A{i} {:x}", cpu.a[i]); }
    let _ = writeln!(o, "SR {:x} PC {:x} CYC {} OSP {:x} STOPPED {}", cpu.get_sr(), cpu.pc, cpu.cyc, cpu.osp, cpu.stopped);
    let now = mem.read_bytes(0, 0x80000).unwrap();
    for (a, (x, y)) in now.iter().zip(init.iter()).enumerate() {
        if x != y { let _ = writeln!(o, "M {:x} {:x}", a, x); }
    }
    print!("{o}");
}
"#;

fn run_ok(cmd: &mut Command) -> String {
    let out = cmd.output().expect("spawn");
    assert!(
        out.status.success(),
        "{:?} failed:\n{}\n{}",
        cmd,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// One translator run over a synthetic image: writes `translated.rs` and `single.c` into `work/gen`.
/// `seeds` are extra discovery start points (game.json `seed_addrs`), e.g. a handler reached only through a vector.
fn translate(work: &Path, image: &[u8], seeds: &[u32]) -> PathBuf {
    let root = repo_root();
    fs::create_dir_all(work.join("out")).unwrap();
    fs::write(work.join("out/image.bin"), image).unwrap();
    let cfg = format!(
        "{{\"name\":\"synthetic\",\"segments\":[{{\"file\":\"out/image.bin\",\"addr\":\"0x{BASE:X}\"}}],\
         \"code\":[\"0x{BASE:X}-0x{:X}\"],\"entry\":\"0x{BASE:X}\",\"ghidra\":false,\"seeds\":[],\"hle\":{{}},\
         \"seed_addrs\":[{}]}}",
        BASE as usize + 0x400,
        seeds
            .iter()
            .map(|a| format!("\"0x{a:X}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    fs::write(work.join("game.json"), cfg).unwrap();
    let gen = work.join("gen");
    fs::create_dir_all(&gen).unwrap();
    run_ok(
        Command::new("python3")
            .arg(root.join("kit/amigo/port/m68k2c.py"))
            .arg(&gen)
            .args(["--config"])
            .arg(work.join("game.json"))
            .args(["--no-speculative", "--emit", "rust", "--single"])
            .arg(gen.join("single.c"))
            .env("AMIGO_W", work),
    );
    gen
}

fn stats(gen: &Path) -> serde_free::Stats {
    serde_free::read(&gen.join("translate_stats.json"))
}

/// Tiny reader for the two stats fields used here (no JSON dependency in this crate).
mod serde_free {
    use std::path::Path;
    pub struct Stats {
        pub rust_translated: usize,
        pub rust_untranslated: usize,
        pub undecodable: usize,
    }
    fn num_after(text: &str, key: &str) -> usize {
        let i = text.find(key).unwrap_or_else(|| panic!("no {key}")) + key.len();
        let rest = text[i..].trim_start_matches([' ', ':']);
        let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap();
        rest[..end].parse().unwrap()
    }
    pub fn read(p: &Path) -> Stats {
        let t = std::fs::read_to_string(p).unwrap();
        let un = t.find("\"rust_untranslated\"").map(|i| {
            let rest = &t[i..];
            let open = rest.find('{').unwrap();
            let close = rest.find('}').unwrap();
            rest[open + 1..close].matches("\": \"").count()
        });
        Stats {
            rust_translated: num_after(&t, "\"rust_instructions_translated\""),
            rust_untranslated: un.unwrap_or(0),
            undecodable: num_after(&t, "\"undecodable_addresses_reached\""),
        }
    }
}

#[test]
fn rust_emitter_matches_c_translation() {
    if !have("python3") || !have("gcc") {
        eprintln!("skipped: needs python3 and gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-emit-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();
    let progs = programs();
    let mut gens = Vec::new();
    let mut cdir = work.join("cases");
    fs::create_dir_all(&cdir).unwrap();
    for (n, (name, words)) in progs.iter().enumerate() {
        let img = image_for(words);
        let stop = BASE + 2 * (words.len() as u32 - 1);
        // "calls" has its stop word in the middle; the others end on it
        let stop = if *name == "calls" { BASE + 10 } else { stop };
        fs::write(cdir.join(format!("c{n}.bin")), &img).unwrap();
        fs::write(cdir.join(format!("c{n}.txt")), case_text(stop)).unwrap();
        // "exceptions" has its handler at $20002, reached only through the vectors it installs, and code after
        // each trap (the decoder ends a block at a trap, as at rts: the return address is not static)
        let seeds: &[u32] = if *name == "exceptions" {
            &[BASE + 2, BASE + 0x42, BASE + 0x44]
        } else {
            &[]
        };
        let gen = translate(&work.join(format!("p{n}")), &img, seeds);
        let st = stats(&gen);
        assert_eq!(st.undecodable, 0, "{name}: undecodable addresses reached");
        assert_eq!(
            st.rust_untranslated, 0,
            "{name}: emitter left instructions untranslated"
        );
        assert!(st.rust_translated >= 5, "{name}: too few instructions");
        gens.push(gen);
    }
    cdir = cdir.canonicalize().unwrap();

    // C side: one binary per program (each has its own `singles` table).
    let driver = work.join("driver.c");
    fs::write(&driver, format!("#define BASE_ADDR 0x{BASE:X}\n{C_DRIVER}")).unwrap();
    let mut c_out = Vec::new();
    for (n, gen) in gens.iter().enumerate() {
        let exe = work.join(format!("c_{n}"));
        run_ok(
            Command::new("gcc")
                .args(["-O1", "-w", "-I"])
                .arg(root.join("kit/amigo/port/runtime"))
                .arg("-I")
                .arg(root.join("kit/amigo/port"))
                .arg("-o")
                .arg(&exe)
                .arg(&driver)
                .arg(gen.join("single.c")),
        );
        c_out.push(run_ok(
            Command::new(&exe)
                .arg(cdir.join(format!("c{n}.bin")))
                .arg(cdir.join(format!("c{n}.txt"))),
        ));
    }

    // Rust side: one throwaway crate with every program's generated step function.
    let krate = work.join("rustside");
    fs::create_dir_all(krate.join("src")).unwrap();
    let mut mods = String::new();
    let mut tab = Vec::new();
    for (n, gen) in gens.iter().enumerate() {
        fs::copy(
            gen.join("translated.rs"),
            krate.join(format!("src/gen_{n}.rs")),
        )
        .unwrap();
        let _ = writeln!(mods, "mod gen_{n};");
        tab.push(format!("gen_{n}::step"));
    }
    let main_rs = format!(
        "{mods}\nconst BASE_ADDR: u32 = 0x{BASE:X};\n{}",
        RUST_MAIN.replace("__STEP_TABLE__", &tab.join(", "))
    );
    fs::write(krate.join("src/main.rs"), main_rs).unwrap();
    fs::write(
        krate.join("Cargo.toml"),
        format!(
            "[package]\nname=\"emit_check\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n\
             amigo_g5={{path=\"{}\"}}\n[workspace]\n",
            env!("CARGO_MANIFEST_DIR")
        ),
    )
    .unwrap();
    run_ok(
        Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--offline"])
            .current_dir(&krate)
            .env("CARGO_TARGET_DIR", work.join("target")),
    );
    let exe = work.join("target/debug/emit_check");
    for (n, (name, _)) in progs.iter().enumerate() {
        let r = run_ok(Command::new(&exe).arg(n.to_string()).arg(&cdir));
        assert!(
            c_out[n].starts_with("STOP ok"),
            "{name}: C did not reach the stop address:\n{}",
            c_out[n]
        );
        eprintln!("{name}: {}", c_out[n].lines().next().unwrap());
        assert_eq!(r, c_out[n], "{name}: Rust and C results differ");
    }
    let _ = fs::remove_dir_all(&work);
}
