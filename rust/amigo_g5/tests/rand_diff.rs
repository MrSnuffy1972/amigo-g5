//! M3 randomised differential test: the Rust emitter against the C translation, instruction by instruction.
//!
//! `support/rand_slots.py` sorts every opcode the decoder accepts into classes by the C it decodes to (registers
//! and constants blanked out) and lays out two opcodes per class with random extension words, one per slot.
//! One translator run writes both the `--single` C and the Rust. Each side then runs every slot TRIALS times from
//! random D0-D7, A0-A7, other stack pointer, CCR, supervisor bit and interrupt mask over a noise-filled memory
//! (the pilot's Musashi difftest did the same with 128 trials per instruction), and prints D0-D7, A0-A7, SR, PC,
//! the cycle count, OSP, the stopped flag and every byte the step changed. The two outputs must be identical.
//!
//! A word or long access at an odd address is a typed error in Rust (the 68000 address error) and silent in the
//! C driver, so the C driver flags it and both sides report `ADDRERR` for that trial without the state; such a
//! trial does not count towards TRIALS. A trial where the C dies with SIGFPE (`divs.w` of $80000000 by -1, undefined
//! behaviour in the decoder's C, found in M2) is not compared: the Rust must show the 68000 result there, V set.
//! Skips cleanly when python3 or gcc is missing.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

mod common;
use common::{have, repo_root, run_ok};

const TRIALS: u32 = 128;
const MAX_ATTEMPTS: u32 = 4 * TRIALS;
const SEED: u32 = 1;

/// The trial state generator, written out identically in C and Rust: xorshift64 (the pilot difftest's), and per
/// attempt k: D0-D7 (random, small values near 0 and $8000, edge values, random even), A0-A7 and OSP (even, inside
/// the noise window), then CCR, S (3 in 4 supervisor) and the interrupt mask from one draw.
const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <setjmp.h>
#include "cpu.h"
#include "boot.h"
#include "run.h"
typedef struct { uint32_t pc; int (*fn)(void); const char *name; int abs_hw; } single_t;
extern const single_t singles[]; extern const int n_singles;
cpu_t cpu; uint64_t CYC, EVT;
static uint8_t ram[1u << 24];
static uint32_t la[4096]; static uint8_t lo[4096]; static int nl, addrerr;
/* odd word/long, or one that runs off the top of the 24-bit space: the Rust GuestMemory refuses both */
static void chk(uint32_t a, uint32_t n) { a &= 0xffffff; if ((a & 1) || a + n > 0x1000000) addrerr = 1; }
uint32_t mem_rb(uint32_t a) { return ram[a & 0xffffff]; }
uint32_t mem_rw(uint32_t a) { chk(a, 2); return (mem_rb(a) << 8) | mem_rb(a + 1); }
uint32_t mem_rl(uint32_t a) { chk(a, 4); return (mem_rw(a) << 16) | mem_rw(a + 2); }
void mem_wb(uint32_t a, uint32_t v) { a &= 0xffffff; if (nl == 4096) abort(); la[nl] = a; lo[nl] = ram[a]; nl++; ram[a] = (uint8_t)v; }
void mem_ww(uint32_t a, uint32_t v) { chk(a, 2); mem_wb(a, v >> 8); mem_wb(a + 1, v); }
void mem_wl(uint32_t a, uint32_t v) { chk(a, 4); mem_ww(a, v >> 16); mem_ww(a + 2, v); }
/* the same as cpu_exception in runtime/cpu_native.c (that file needs the whole runtime, so it is not linked here) */
uint32_t cpu_exception(int vec, uint32_t pc) {
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc);
    A[7] -= 2; mem_ww(A[7], sr);
    CYC += 34;
    return mem_rl((uint32_t)vec * 4);
}
static uint64_t rng = 88172645463325252ull;
static uint32_t rnd(void) { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; return (uint32_t)rng; }
static const uint32_t edge[8] = {0, 0x80, 0xff, 0x8000, 0xffff, 0x7fffffff, 0x80000000u, 0xffffffffu};
/* divs.w of $80000000 by -1 divides before the overflow check in the decoder's C: undefined, SIGFPE on x86-64
   (found in M2, 28 Oct 2026, a translator change for Karl). The trial is reported, not compared. */
static sigjmp_buf fpe_jb;
static void on_fpe(int sig) { (void)sig; siglongjmp(fpe_jb, 1); }
static int cmpu(const void *x, const void *y) { uint32_t a = *(const uint32_t *)x, b = *(const uint32_t *)y; return a < b ? -1 : a > b; }
int main(int argc, char **argv) {
    (void)argc;
    FILE *f = fopen(argv[1], "rb"); size_t n = fread(ram, 1, sizeof ram, f); (void)n; fclose(f);
    unsigned trials = (unsigned)atoi(argv[3]), maxatt = (unsigned)atoi(argv[4]);
    signal(SIGFPE, on_fpe);
    f = fopen(argv[2], "r");
    unsigned pc, op, len, cls; int slot = 0;
    static uint32_t ws[4096];
    while (fscanf(f, "%x %x %u %u", &pc, &op, &len, &cls) == 4) {
        int k = -1; for (int i = 0; i < n_singles; i++) if (singles[i].pc == pc) { k = i; break; }
        if (k < 0) { printf("%d untranslated\n", slot); return 3; }
        unsigned ok = 0, att;
        for (att = 0; att < maxatt && ok < trials; att++) {
            for (int j = 0; j < 8; j++) {
                uint32_t v = rnd(), w = rnd(), d;
                switch (att % 4) {
                case 0: d = v; break;
                case 1: d = (w % 7) - 3 + ((v & 1) ? 0x8000 : 0); break;
                case 2: d = edge[w % 8]; break;
                default: d = v & ~1u;
                }
                cpu.dr[j] = d;
            }
            for (int j = 0; j < 8; j++) cpu.ar[j] = (0x14000 + rnd() % 0x18000) & ~1u;
            cpu.OSP = (0x14000 + rnd() % 0x18000) & ~1u;
            uint32_t x = rnd();
            set_ccr(x & 0x1f); cpu.S = ((x >> 5) & 3) != 0; cpu.IM = (x >> 8) & 7;
            cpu.PC = pc; cpu.stopped = 0; CYC = 0; nl = 0; addrerr = 0;
            if (sigsetjmp(fpe_jb, 1)) { printf("%d %u SIGFPE\n", slot, att); ok++; goto undo; }
            singles[k].fn();
            if (addrerr) printf("%d %u ADDRERR\n", slot, att);
            else {
                ok++;
                printf("%d %u", slot, att);
                for (int j = 0; j < 8; j++) printf(" %x", cpu.dr[j]);
                for (int j = 0; j < 8; j++) printf(" %x", cpu.ar[j]);
                printf(" sr %x pc %x cyc %llu osp %x st %d", get_sr(), cpu.PC, (unsigned long long)CYC, cpu.OSP, cpu.stopped);
                /* every byte whose value changed, by address */
                int nw = 0;
                for (int i = 0; i < nl; i++) { int seen = 0; for (int q = 0; q < nw; q++) if (ws[q] == la[i]) seen = 1; if (!seen) ws[nw++] = la[i]; }
                qsort(ws, nw, sizeof ws[0], cmpu);
                for (int q = 0; q < nw; q++) {
                    uint8_t old = 0; for (int i = 0; i < nl; i++) if (la[i] == ws[q]) { old = lo[i]; break; }
                    if (ram[ws[q]] != old) printf(" %x=%x", ws[q], ram[ws[q]]);
                }
                printf("\n");
            }
        undo:
            while (nl > 0) { nl--; ram[la[nl]] = lo[nl]; }
        }
        printf("%d ok %u of %u\n", slot, ok, att);
        slot++;
    }
    return 0;
}
"#;

const RUST_MAIN: &str = r#"
mod gen;
use amigo_g5::cpu::{Cpu, Stop};
use amigo_g5::mem::GuestMemory;
use std::fmt::Write as _;
use std::io::Write as _;
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 as u32
    }
}
const EDGE: [u32; 8] = [0, 0x80, 0xff, 0x8000, 0xffff, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff];
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let img = std::fs::read(&args[1]).unwrap();
    let slots = std::fs::read_to_string(&args[2]).unwrap();
    let trials: u32 = args[3].parse().unwrap();
    let maxatt: u32 = args[4].parse().unwrap();
    let mut mem = GuestMemory::new(&[(0, 1 << 24)]).unwrap();
    mem.write_bytes(0, &img).unwrap();
    mem.start_journal();
    let mut rng = Rng(88172645463325252);
    let mut o = String::new();
    for (slot, line) in slots.lines().enumerate() {
        let pc = u32::from_str_radix(line.split_whitespace().next().unwrap(), 16).unwrap();
        let (mut ok, mut att) = (0, 0);
        while att < maxatt && ok < trials {
            let mut cpu = Cpu::default();
            for j in 0..8 {
                let (v, w) = (rng.next(), rng.next());
                cpu.d[j] = match att % 4 {
                    0 => v,
                    1 => (w % 7).wrapping_sub(3).wrapping_add(if v & 1 != 0 { 0x8000 } else { 0 }),
                    2 => EDGE[(w % 8) as usize],
                    _ => v & !1,
                };
            }
            for j in 0..8 {
                cpu.a[j] = (0x14000 + rng.next() % 0x18000) & !1;
            }
            cpu.osp = (0x14000 + rng.next() % 0x18000) & !1;
            let x = rng.next();
            cpu.set_ccr(x & 0x1f);
            cpu.s = (((x >> 5) & 3) != 0) as u32;
            cpu.im = (x >> 8) & 7;
            cpu.pc = pc;
            match gen::step(&mut cpu, &mut mem) {
                Ok(()) => {
                    ok += 1;
                    let _ = write!(o, "{slot} {att}");
                    for v in cpu.d.iter().chain(cpu.a.iter()) {
                        let _ = write!(o, " {v:x}");
                    }
                    let _ = write!(o, " sr {:x} pc {:x} cyc {} osp {:x} st {}", cpu.get_sr(), cpu.pc, cpu.cyc, cpu.osp, cpu.stopped);
                    let mut first: Vec<(u32, u8)> = Vec::new();
                    for &(a, old) in mem.journal() {
                        if !first.iter().any(|&(b, _)| b == a) {
                            first.push((a, old));
                        }
                    }
                    first.sort();
                    for (a, old) in first {
                        let now = mem.read_u8(a).unwrap();
                        if now != old {
                            let _ = write!(o, " {a:x}={now:x}");
                        }
                    }
                    o.push('\n');
                }
                Err(Stop::Mem(_)) => {
                    let _ = writeln!(o, "{slot} {att} ADDRERR");
                }
                Err(e) => panic!("slot {slot} at {pc:x}: {e:?}"),
            }
            mem.rollback();
            att += 1;
        }
        let _ = writeln!(o, "{slot} ok {ok} of {att}");
    }
    std::io::stdout().write_all(o.as_bytes()).unwrap();
}
"#;

#[test]
fn rust_emitter_matches_c_on_random_trials() {
    if !have("python3") || !have("gcc") {
        eprintln!("skipped: needs python3 and gcc");
        return;
    }
    let root = repo_root();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let work = std::env::temp_dir().join(format!("g5-rand-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(work.join("out")).unwrap();
    let made = run_ok(
        Command::new("python3")
            .arg(here.join("tests/support/rand_slots.py"))
            .arg(work.join("out"))
            .arg(SEED.to_string()),
    );
    eprintln!("{}", made.trim());
    let slots_txt = fs::read_to_string(work.join("out/slots.txt")).unwrap();
    let slots: Vec<(u32, String, u32)> = slots_txt
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split_whitespace().collect();
            (
                u32::from_str_radix(f[0], 16).unwrap(),
                f[1].to_string(),
                f[3].parse().unwrap(),
            )
        })
        .collect();
    let end = slots.last().unwrap().0 + 16;
    let seeds: Vec<String> = slots.iter().map(|s| format!("\"0x{:X}\"", s.0)).collect();
    fs::write(
        work.join("game.json"),
        format!(
            "{{\"name\":\"synthetic\",\"segments\":[{{\"file\":\"out/image.bin\",\"addr\":\"0x0\"}}],\
             \"code\":[\"0x{:X}-0x{end:X}\"],\"entry\":\"0x{:X}\",\"ghidra\":false,\"seeds\":[],\"hle\":{{}},\
             \"seed_addrs\":[{}]}}",
            slots[0].0,
            slots[0].0,
            seeds.join(",")
        ),
    )
    .unwrap();
    let gen = work.join("gen");
    fs::create_dir_all(&gen).unwrap();
    run_ok(
        Command::new("python3")
            .arg(root.join("kit/amigo/port/m68k2c.py"))
            .arg(&gen)
            .arg("--config")
            .arg(work.join("game.json"))
            .args(["--no-speculative", "--emit", "rust", "--single"])
            .arg(gen.join("single.c"))
            .env("AMIGO_W", &work),
    );
    let stats = fs::read_to_string(gen.join("translate_stats.json")).unwrap();
    assert!(
        stats.contains("\"rust_untranslated\": {}"),
        "emitter left instructions untranslated"
    );

    let driver = work.join("driver.c");
    fs::write(&driver, C_DRIVER).unwrap();
    let c_exe = work.join("c_side");
    run_ok(
        Command::new("gcc")
            .args(["-O1", "-w", "-I"])
            .arg(root.join("kit/amigo/port/runtime"))
            .arg("-I")
            .arg(root.join("kit/amigo/port"))
            .arg("-o")
            .arg(&c_exe)
            .arg(&driver)
            .arg(gen.join("single.c")),
    );

    let krate = work.join("rustside");
    fs::create_dir_all(krate.join("src")).unwrap();
    fs::copy(gen.join("translated.rs"), krate.join("src/gen.rs")).unwrap();
    fs::write(krate.join("src/main.rs"), RUST_MAIN).unwrap();
    // opt-level 1: the generated step function has one arm per instruction, and a debug build of thousands of
    // trials per slot is slow; the C side is built with -O1 too
    fs::write(
        krate.join("Cargo.toml"),
        format!(
            "[package]\nname=\"rand_check\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n\
             amigo_g5={{path=\"{}\"}}\n[profile.dev]\nopt-level=1\n[workspace]\n",
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
    let r_exe = work.join("target/debug/rand_check");

    let args = |c: &mut Command| {
        c.arg(work.join("out/image.bin"))
            .arg(work.join("out/slots.txt"))
            .arg(TRIALS.to_string())
            .arg(MAX_ATTEMPTS.to_string());
    };
    let mut cc = Command::new(&c_exe);
    args(&mut cc);
    let c_out = run_ok(&mut cc);
    let mut rc = Command::new(&r_exe);
    args(&mut rc);
    let r_out = run_ok(&mut rc);

    // Compare line by line; report the first differing trial of up to 10 slots, with the slot's opcode.
    let mut report = String::new();
    let mut bad_slots = Vec::new();
    let (mut compared, mut addrerr, mut c_fpe) = (0usize, 0usize, 0usize);
    let mut short = Vec::new();
    let mut c_lines = c_out.lines();
    let mut r_lines = r_out.lines();
    loop {
        let (c, r) = (c_lines.next(), r_lines.next());
        if c.is_none() && r.is_none() {
            break;
        }
        let (c, r) = (c.unwrap_or("<end>"), r.unwrap_or("<end>"));
        let slot: usize = c.split(' ').next().unwrap().parse().unwrap_or(usize::MAX);
        if let Some(head) = c.strip_suffix(" SIGFPE") {
            let sr = r.split_once(" sr ").map(|x| x.1.split(' ').next().unwrap());
            let v_set = sr.is_some_and(|x| u32::from_str_radix(x, 16).unwrap() & 2 != 0);
            if r.starts_with(&format!("{head} ")) && v_set {
                c_fpe += 1;
                continue;
            }
        }
        if c != r {
            if !bad_slots.contains(&slot) {
                bad_slots.push(slot);
                if bad_slots.len() <= 10 {
                    let (pc, op, _) = slots.get(slot).cloned().unwrap_or_default();
                    let _ = writeln!(
                        report,
                        "slot {slot} ${pc:X} op {op}:\n  C    {c}\n  Rust {r}"
                    );
                }
            }
            continue;
        }
        if c.ends_with("ADDRERR") {
            addrerr += 1;
        } else if let Some(rest) = c.split_once(" ok ") {
            let ok: u32 = rest.1.split(' ').next().unwrap().parse().unwrap();
            if ok < TRIALS {
                short.push((slot, ok));
            }
        } else {
            compared += 1;
        }
    }
    eprintln!(
        "slots {}, trials compared {compared}, address-error trials {addrerr}, C SIGFPE trials (Rust V set) {c_fpe}, \
         slots short of {TRIALS} trials {}",
        slots.len(),
        short.len()
    );
    for (slot, ok) in short.iter().take(10) {
        let (pc, op, cls) = &slots[*slot];
        eprintln!("  short: slot {slot} ${pc:X} op {op} class {cls}: {ok} trials (the rest are address errors)");
    }
    assert!(
        bad_slots.is_empty(),
        "{} slots differ between Rust and C:\n{report}",
        bad_slots.len()
    );
    // Every class must have at least one slot with the full TRIALS.
    let mut full = std::collections::HashSet::new();
    for (n, s) in slots.iter().enumerate() {
        if !short.iter().any(|x| x.0 == n) {
            full.insert(s.2);
        }
    }
    let classes: std::collections::HashSet<u32> = slots.iter().map(|s| s.2).collect();
    let missing: Vec<_> = classes.difference(&full).collect();
    assert!(
        missing.is_empty(),
        "classes with no slot that reached {TRIALS} trials: {missing:?}"
    );
    let _ = fs::remove_dir_all(&work);
}
