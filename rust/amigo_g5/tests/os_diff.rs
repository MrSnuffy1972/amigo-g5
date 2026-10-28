//! M4 differential test: cold boot of a filed program and the OS services it calls, Rust against the C kit.
//!
//! Each synthetic program (assembled here, no game data) is an AmigaDOS hunk file with its own data files. The C
//! side boots it the way the kit boots a filed game: hunkload.py with LoadSeg headers, oshle.py's `execlib()` and
//! library table, oshle.c's `game_boot_extra`, and `cpu.c run_boot`'s start registers; it runs the `--single` C
//! translation with every jump-table call served by the real `oshle.c` (linked whole). The Rust side boots the same
//! hunk file with `boot::cold_boot` and runs the generated Rust with `os::run`. Both print the state after the boot
//! (registers and a memory hash) and at the stop (registers, cycles, the custom-chip writes, every non-zero byte of
//! chip and slow RAM), and the outputs must be identical. Skips cleanly without python3 or gcc.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;
use common::{have, repo_root, run_ok};

const MAX_STEPS: u64 = 20000;

/// A tiny assembler for the handful of 68000 forms the programs use. Encodings follow the 68000 manual.
#[derive(Default)]
struct Asm {
    w: Vec<u16>,
    labels: HashMap<&'static str, usize>,
    pcrel: Vec<(usize, &'static str)>,
    /// Byte offsets of long words that point into the BSS hunk (RELOC32 against hunk 1).
    relocs: Vec<u32>,
}

impl Asm {
    fn op(&mut self, ws: &[u16]) -> &mut Self {
        self.w.extend_from_slice(ws);
        self
    }
    fn moveq(&mut self, n: i8, d: u16) -> &mut Self {
        self.op(&[0x7000 | d << 9 | n as u8 as u16])
    }
    fn move_l(&mut self, src_d: u16, dst_d: u16) -> &mut Self {
        self.op(&[0x2000 | dst_d << 9 | src_d])
    }
    fn move_a_d(&mut self, src_a: u16, dst_d: u16) -> &mut Self {
        self.op(&[0x2008 | dst_d << 9 | src_a])
    }
    fn movea_d(&mut self, src_d: u16, dst_a: u16) -> &mut Self {
        self.op(&[0x2040 | dst_a << 9 | src_d])
    }
    fn imm_l(&mut self, v: u32, d: u16) -> &mut Self {
        self.op(&[0x203C | d << 9, (v >> 16) as u16, v as u16])
    }
    /// move.l Dn,(A2)+ : the program's results go to its BSS, so the memory compare sees them.
    fn keep(&mut self, d: u16) -> &mut Self {
        self.op(&[0x24C0 | d])
    }
    fn lea_d16(&mut self, disp: i16, src_a: u16, dst_a: u16) -> &mut Self {
        self.op(&[0x41E8 | dst_a << 9 | src_a, disp as u16])
    }
    fn lea_pc(&mut self, label: &'static str, a: u16) -> &mut Self {
        self.op(&[0x41FA | a << 9]);
        self.pcrel.push((self.w.len(), label));
        self.op(&[0])
    }
    fn lea_bss(&mut self, off: u32, a: u16) -> &mut Self {
        self.op(&[0x41F9 | a << 9]);
        self.relocs.push(2 * self.w.len() as u32);
        self.op(&[(off >> 16) as u16, off as u16])
    }
    /// jsr lvo(a6)
    fn call(&mut self, lvo: i16) -> &mut Self {
        self.op(&[0x4EAE, lvo as u16])
    }
    fn exec_base(&mut self) -> &mut Self {
        self.op(&[0x2C78, 0x0004]) // movea.l 4.w,a6
    }
    fn string(&mut self, label: &'static str, s: &[u8]) -> &mut Self {
        self.labels.insert(label, 2 * self.w.len());
        let mut b = s.to_vec();
        b.push(0);
        if b.len() % 2 == 1 {
            b.push(0);
        }
        for p in b.chunks(2) {
            self.w.push(u16::from_be_bytes([p[0], p[1]]));
        }
        self
    }
    fn bytes(&self) -> Vec<u8> {
        let mut w = self.w.clone();
        for &(at, label) in &self.pcrel {
            w[at] = (self.labels[label] as i32 - 2 * at as i32) as u16;
        }
        w.iter().flat_map(|x| x.to_be_bytes()).collect()
    }
}

/// HUNK_HEADER, a CODE hunk with RELOC32 into a BSS hunk, HUNK_END each. `chip` flags both hunks MEMF_CHIP.
fn hunk_file(a: &Asm, bss_longs: u32, chip: bool) -> Vec<u8> {
    let mut code = a.bytes();
    while !code.len().is_multiple_of(4) {
        code.push(0);
    }
    let flag = if chip { 0x4000_0000 } else { 0 };
    let mut l = vec![
        0x3F3,
        0,
        2,
        0,
        1,
        (code.len() / 4) as u32 | flag,
        bss_longs | flag,
    ];
    l.extend([0x3E9, (code.len() / 4) as u32]);
    let mut out: Vec<u8> = l.iter().flat_map(|x| x.to_be_bytes()).collect();
    out.extend(&code);
    let mut tail = vec![0x3EC, a.relocs.len() as u32, 1];
    tail.extend(&a.relocs);
    tail.extend([0, 0x3F2, 0x3EB, bss_longs, 0x3F2]);
    out.extend(tail.iter().flat_map(|x| x.to_be_bytes()));
    out
}

/// Program A: loaded in slow RAM. Libraries, AllocMem/FreeMem/AvailMem, Disable/Enable, FindTask, a file read in
/// parts with Seek, a missing file, Lock/Examine/UnLock of a file and of a directory, IoErr, console Write; ends
/// with RTS to the CLI.
fn program_a() -> Vec<u8> {
    let mut a = Asm::default();
    a.move_a_d(0, 6).move_l(0, 7); // the argument line and its length
    a.lea_bss(0, 2).lea_d16(256, 2, 3); // a2 = results, a3 = buffers
    a.exec_base().lea_pc("dos", 1).moveq(0, 0).call(-552);
    a.movea_d(0, 5).keep(0);
    a.lea_pc("nolib", 1).moveq(0, 0).call(-552).keep(0);
    a.moveq(100, 0)
        .imm_l(0x10001, 1)
        .call(-198)
        .move_l(0, 5)
        .keep(0); // AllocMem(100, PUBLIC|CLEAR)
    a.op(&[0x2245]).moveq(100, 0).call(-210); // movea.l d5,a1 / FreeMem
    a.op(&[0x2245]).moveq(100, 0).call(-210); // a second FreeMem of the same block is not listed twice
    a.moveq(104, 0).moveq(2, 1).call(-198).keep(0); // AllocMem(104, CHIP): the freed block is slow, not taken
    a.moveq(101, 0).moveq(0, 1).call(-198).keep(0); // AllocMem(101, 0): the freed block again
    a.moveq(100, 0).moveq(0, 1).call(-198).keep(0); // and once only: this one is new
    a.moveq(2, 1)
        .call(-216)
        .keep(0)
        .moveq(0, 1)
        .call(-216)
        .keep(0); // AvailMem(CHIP), AvailMem(0)
    a.imm_l(0x20000, 1).call(-216).keep(0); // AvailMem(LARGEST)
    a.call(-132)
        .call(-138)
        .call(-120)
        .call(-120)
        .call(-126)
        .call(-126); // Forbid Permit Disable x2 Enable x2
    a.op(&[0x93C9]).call(-294).keep(0); // suba.l a1,a1 / FindTask(0)
    a.op(&[0x2C4D]); // movea.l a5,a6: dos
    a.lea_pc("file", 0)
        .move_a_d(0, 1)
        .imm_l(1005, 2)
        .call(-30)
        .move_l(0, 4)
        .keep(0)
        .keep(1);
    a.move_l(4, 1).move_a_d(3, 2).moveq(40, 3).call(-42).keep(0); // Read 40
    a.move_l(4, 1).moveq(-4, 2).moveq(1, 3).call(-66).keep(0); // Seek(-4, OFFSET_END)
    a.move_l(4, 1)
        .lea_d16(64, 3, 0)
        .move_a_d(0, 2)
        .moveq(16, 3)
        .call(-42)
        .keep(0); // Read 16: 4 left
    a.move_l(4, 1).move_a_d(0, 2).moveq(16, 3).call(-42).keep(0); // Read at the end: 0
    a.move_l(4, 1).moveq(-1, 2).moveq(0, 3).call(-66).keep(0); // Seek(-1, OFFSET_CURRENT)
    a.move_l(4, 1).moveq(50, 2).moveq(1, 3).call(-66).keep(0); // Seek(50, OFFSET_END): held at the end
    a.move_l(4, 1).moveq(0, 2).moveq(0, 3).call(-66).keep(0); // Seek(0, OFFSET_CURRENT): 100
    a.move_l(4, 1).call(-36).keep(0); // Close
    a.move_l(4, 1).move_a_d(0, 2).moveq(16, 3).call(-42).keep(0); // Read after Close: -1
    a.lea_pc("missing", 0)
        .move_a_d(0, 1)
        .imm_l(1005, 2)
        .call(-30)
        .keep(0);
    a.lea_pc("upper", 0)
        .move_a_d(0, 1)
        .moveq(-2, 2)
        .call(-84)
        .move_l(0, 4)
        .keep(0); // Lock, case ignored
    a.move_l(4, 1)
        .lea_d16(128, 3, 0)
        .move_a_d(0, 2)
        .call(-102)
        .keep(0); // Examine the file
    a.move_l(4, 1).call(-90).call(-132).keep(0); // UnLock, IoErr
    a.lea_pc("dir", 0)
        .move_a_d(0, 1)
        .moveq(-2, 2)
        .call(-84)
        .move_l(0, 4)
        .keep(0); // Lock a directory
    a.move_l(4, 1)
        .lea_d16(400, 3, 0)
        .move_a_d(0, 2)
        .call(-102)
        .keep(0); // Examine it
    a.move_l(4, 1)
        .lea_d16(400, 3, 0)
        .move_a_d(0, 2)
        .call(-108)
        .keep(0); // ExNext: none
    a.call(-132).keep(0); // IoErr: 232
    a.call(-60)
        .move_l(0, 1)
        .lea_pc("msg", 0)
        .move_a_d(0, 2)
        .moveq(10, 3)
        .call(-48)
        .keep(0); // Write
    a.keep(6).keep(7).op(&[0x2046, 0x24D0]); // the argument pointer, length and first four bytes
    a.moveq(0, 0).op(&[0x4E75]);
    a.string("dos", b"dos.library")
        .string("nolib", b"nosuch.library");
    a.string("file", b"Data/Level1.dat")
        .string("missing", b"df0:missing.dat");
    a.string("upper", b"LEVEL1.DAT")
        .string("dir", b"Data")
        .string("msg", b"hello G5!\n");
    hunk_file(&a, 256, false)
}

/// Program B: chip hunks. A file read whole, Seek modes, DateStamp, Info, Delay, the console's Input and Read,
/// DupLock, ParentDir, OpenResource, CopyMem; ends with dos Exit.
fn program_b() -> Vec<u8> {
    let mut a = Asm::default();
    a.lea_bss(0, 2).lea_bss(0, 3);
    a.exec_base()
        .lea_pc("dos", 1)
        .moveq(0, 0)
        .call(-408)
        .move_l(0, 5)
        .keep(0); // OldOpenLibrary
    a.lea_pc("ciab", 1).call(-498).keep(0); // OpenResource("ciab.resource")
    a.op(&[0x2C45]); // movea.l d5,a6
    a.lea_pc("file", 0)
        .move_a_d(0, 1)
        .imm_l(1005, 2)
        .call(-30)
        .move_l(0, 4)
        .keep(0);
    a.move_l(4, 1)
        .lea_d16(192, 3, 0)
        .move_a_d(0, 2)
        .imm_l(200, 3)
        .call(-42)
        .keep(0); // Read all (60)
    a.move_l(4, 1).moveq(3, 2).moveq(-1, 3).call(-66).keep(0); // Seek(3, BEGINNING): old position 60
    a.move_l(4, 1).moveq(2, 2).moveq(0, 3).call(-66).keep(0); // Seek(2, CURRENT): 3
    a.move_l(4, 1)
        .lea_d16(96, 3, 0)
        .move_a_d(0, 2)
        .moveq(4, 3)
        .call(-42)
        .keep(0); // Read 4 from 5
    a.move_l(4, 1).call(-36);
    a.lea_d16(112, 3, 0).move_a_d(0, 1).call(-192).keep(0); // DateStamp
    a.moveq(0, 1)
        .lea_d16(128, 3, 0)
        .move_a_d(0, 2)
        .call(-114)
        .keep(0); // Info
    a.moveq(1, 1).call(-198).keep(1); // Delay(1): D1 = D0 after a dos call
    a.call(-54)
        .move_l(0, 1)
        .lea_d16(160, 3, 0)
        .move_a_d(0, 2)
        .moveq(2, 3)
        .call(-42)
        .keep(0); // Input, Read
    a.lea_pc("file", 0)
        .move_a_d(0, 1)
        .moveq(-2, 2)
        .call(-84)
        .move_l(0, 4)
        .keep(0);
    a.move_l(4, 1)
        .call(-96)
        .keep(0)
        .move_l(4, 1)
        .call(-210)
        .keep(0); // DupLock, ParentDir
    a.op(&[0x2C78, 0x0004]); // exec again
    a.lea_d16(192, 3, 0)
        .lea_d16(176, 3, 1)
        .moveq(8, 0)
        .call(-624); // CopyMem 8 bytes
    a.op(&[0x2C45]).moveq(5, 1).call(-144); // dos Exit(5)
    a.op(&[0x4E71, 0x60FE]); // never reached
    a.string("dos", b"dos.library")
        .string("ciab", b"ciab.resource")
        .string("file", b"readme");
    hunk_file(&a, 64, true)
}

/// Prepares one program the way the C kit does (hunkload.py, oshle.py `execlib()`, the pools from oshle.py `main`
/// and its oshle_cfg.c format). Prints "base entry code_lo code_hi".
const PREP_PY: &str = r##"
import json, os, sys
root, exe, out, args = sys.argv[1:5]
sys.path.insert(0, os.path.join(root, "kit/amigo")); sys.path.insert(0, os.path.join(root, "kit/amigo/port/oshle"))
import hunkload, oshle
res = hunkload.load(open(exe, "rb").read(), seglist=True, chip_base=0x4000)
base, size = res["base"], len(res["image"])
end = (base + size + 0xFFF) & ~0xFFF
in_chip = base < 0x80000
chip_lo, chip_hi = max(0x80000, end if in_chip else 0x80000), 0x100000
slow_lo, slow_hi = (max(0xC00000, end) if not in_chip else 0xC00000), 0xC00000 + 1536 * 1024
open(os.path.join(out, "image.bin"), "wb").write(res["image"])
open(os.path.join(out, "execlib.bin"), "wb").write(oshle.execlib())
open(os.path.join(out, "oshle_cfg.c"), "w").write(
    "#include <stdint.h>\nconst char *oshle_adf = 0;\nconst uint32_t oshle_chip_lo = 0x%X, oshle_chip_hi = 0x%X;\n"
    "const uint32_t oshle_slow_lo = 0x%X, oshle_slow_hi = 0x%X;\nconst uint32_t oshle_seglist = 0x%X;\n"
    "const char *oshle_args = %s;\nconst int oshle_wb = 0;\nconst char *oshle_exe = \"prog\";\n"
    % (chip_lo, chip_hi, slow_lo, slow_hi, res["seglist"], json.dumps(args)) + oshle.libs_c())
code = [(a, a + s) for a, s, k in zip(res["addr"], res["sizes"], res["kinds"]) if k == "code"]
print("%X %X %X %X" % (base, res["entry"], code[0][0], code[0][1]))
"##;

const C_DRIVER: &str = r#"
#include <dirent.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cpu.h"
#include "boot.h"
typedef struct { uint32_t pc; int (*fn)(void); const char *name; int abs_hw; } single_t;
extern const single_t singles[]; extern const int n_singles;
typedef struct { const char *name; unsigned base, slots; } oshle_lib_t;
extern const oshle_lib_t oshle_libs[]; extern const int oshle_nlibs;
void game_boot_extra(uint32_t *regs, uint32_t *pc, uint16_t *sr);
void hle_lib(void);
cpu_t cpu; uint64_t CYC, EVT, chip_frame_count, hle_last_load_frame; uint16_t dmacon;
static void logf_(const char *f, ...) { if (getenv("OS_DIFF_LOG")) { va_list ap; va_start(ap, f); vfprintf(stderr, f, ap); va_end(ap); } }
void (*hle_log)(const char *, ...) = logf_;
static uint8_t chip[0x100000], slow[0x180000];
static uint32_t wlog[64][2]; static int nw;
static uint8_t *at(uint32_t a) {
    a &= 0xffffff;
    if (a < sizeof chip) return &chip[a];
    if (a >= 0xC00000 && a < 0xC00000 + sizeof slow) return &slow[a - 0xC00000];
    printf("BAD %x\n", a); exit(3);
}
uint32_t mem_rb(uint32_t a) { return *at(a); }
uint32_t mem_rw(uint32_t a) { return (mem_rb(a) << 8) | mem_rb(a + 1); }
uint32_t mem_rl(uint32_t a) { return (mem_rw(a) << 16) | mem_rw(a + 2); }
void mem_wb(uint32_t a, uint32_t v) { *at(a) = (uint8_t)v; }
void mem_ww(uint32_t a, uint32_t v) {
    if ((a & 0xfff000) == 0xdff000) { if (nw < 64) { wlog[nw][0] = a & 0xffffff; wlog[nw][1] = v & 0xffff; nw++; } return; }
    mem_wb(a, v >> 8); mem_wb(a + 1, v);
}
void mem_wl(uint32_t a, uint32_t v) { mem_ww(a, v >> 16); mem_ww(a + 2, v); }
uint32_t cpu_exception(int vec, uint32_t pc) {   /* as runtime/cpu_native.c */
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc); A[7] -= 2; mem_ww(A[7], sr); CYC += 34;
    return mem_rl((uint32_t)vec * 4);
}
static char fname[16][64]; static uint8_t *fdata[16]; static size_t flen[16]; static int nfiles;
const uint8_t *gamefile(const char *name, size_t *len) {
    for (int i = 0; i < nfiles; i++) if (!strcmp(fname[i], name)) { *len = flen[i]; return fdata[i]; }
    for (int i = 0; i < nfiles; i++) if (amigo_streq_ci(fname[i], name)) { *len = flen[i]; return fdata[i]; }
    return 0;
}
static uint8_t *slurp(const char *p, size_t *n) {
    FILE *f = fopen(p, "rb"); if (!f) { printf("NOFILE %s\n", p); exit(2); }
    fseek(f, 0, SEEK_END); long l = ftell(f); fseek(f, 0, SEEK_SET);
    uint8_t *b = malloc(l ? l : 1); *n = fread(b, 1, l, f); fclose(f); return b;
}
static int slot(uint32_t pc) {
    for (int i = 0; i < oshle_nlibs; i++) { uint32_t b = oshle_libs[i].base, lo = b - 6 * oshle_libs[i].slots;
        if (pc >= lo && pc < b && (b - pc) % 6 == 0) return 1; }
    return 0;
}
static void dump(const char *tag) {
    uint64_t h = 1469598103934665603ull;
    for (size_t i = 0; i < sizeof chip; i++) { h ^= chip[i]; h *= 1099511628211ull; }
    for (size_t i = 0; i < sizeof slow; i++) { h ^= slow[i]; h *= 1099511628211ull; }
    printf("%s", tag); for (int i = 0; i < 8; i++) printf(" %x", cpu.dr[i]); for (int i = 0; i < 8; i++) printf(" %x", cpu.ar[i]);
    printf(" SR %x PC %x CYC %llu OSP %x STOPPED %d HASH %016llx\n", get_sr(), cpu.PC, (unsigned long long)CYC, cpu.OSP, cpu.stopped, (unsigned long long)h);
}
int main(int argc, char **argv) {
    (void)argc; size_t n;
    uint8_t *x = slurp(argv[1], &n); memcpy(chip, x, n);
    uint32_t base = strtoul(argv[3], 0, 16), entry = strtoul(argv[4], 0, 16);
    x = slurp(argv[2], &n); for (size_t i = 0; i < n; i++) mem_wb(base + (uint32_t)i, x[i]);
    DIR *d = opendir(argv[5]); struct dirent *e;
    while (d && (e = readdir(d)) && nfiles < 16) if (e->d_name[0] != '.') {
        char p[512]; snprintf(p, sizeof p, "%s/%s", argv[5], e->d_name);
        snprintf(fname[nfiles], 64, "%s", e->d_name); fdata[nfiles] = slurp(p, &flen[nfiles]); nfiles++; }
    if (d) closedir(d);
    uint32_t regs[16] = {0}, pc = entry; uint16_t sr = 0x2700; regs[15] = 0x7FFF0;
    game_boot_extra(regs, &pc, &sr);   /* then cpu.c run_boot */
    memcpy(cpu.dr, regs, 32); memcpy(cpu.ar, regs + 8, 32);
    cpu.S = 1; cpu.IM = (sr >> 8) & 7; set_ccr(sr); cpu.OSP = 0; cpu.PC = pc;
    if (!(sr & 0x2000)) { cpu.S = 0; cpu.OSP = mem_rl(0); }
    dump("BOOT");
    unsigned long long steps = 0, maxs = strtoull(argv[6], 0, 10); const char *why = "ok";
    while (cpu.PC != 0x3FF0 && steps < maxs) {
        if (slot(cpu.PC)) hle_lib();
        else { int k = -1; for (int i = 0; i < n_singles; i++) if (singles[i].pc == cpu.PC) { k = i; break; }
               if (k < 0) { why = "untranslated"; break; } singles[k].fn(); }
        steps++;
    }
    printf("STOP %s STEPS %llu\n", why, steps);
    dump("END");
    for (int i = 0; i < nw; i++) printf("W %x %x\n", wlog[i][0], wlog[i][1]);
    for (uint32_t a = 0; a < sizeof chip; a++) if (chip[a]) printf("M %x %x\n", a, chip[a]);
    for (uint32_t a = 0; a < sizeof slow; a++) if (slow[a]) printf("M %x %x\n", a + 0xC00000, slow[a]);
    return 0;
}
"#;

const RUST_MAIN: &str = r#"
use amigo_g5::boot::cold_boot;
use amigo_g5::cpu::{Cpu, Stop};
use amigo_g5::mem::GuestMemory;
use amigo_g5::os::{run, GameFiles, EXIT_PC};
use std::fmt::Write;
type Step = fn(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>;
fn regions(mem: &GuestMemory) -> [(u32, &[u8]); 2] {
    [(0, mem.read_bytes(0, 0x100000).unwrap()), (0xC00000, mem.read_bytes(0xC00000, 0x180000).unwrap())]
}
fn dump(o: &mut String, tag: &str, cpu: &Cpu, mem: &GuestMemory) {
    let mut h: u64 = 1469598103934665603;
    for (_, r) in regions(mem) { for b in r { h ^= *b as u64; h = h.wrapping_mul(1099511628211); } }
    let _ = write!(o, "{tag}");
    for v in cpu.d.iter().chain(cpu.a.iter()) { let _ = write!(o, " {v:x}"); }
    let _ = writeln!(o, " SR {:x} PC {:x} CYC {} OSP {:x} STOPPED {} HASH {h:016x}", cpu.get_sr(), cpu.pc, cpu.cyc, cpu.osp, cpu.stopped);
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let steps_tab: Vec<Step> = vec![__STEP_TABLE__];
    let n: usize = args[1].parse().unwrap();
    let dir = std::path::Path::new(&args[2]);
    let exe = std::fs::read(dir.join("prog.exe")).unwrap();
    let line = std::fs::read(dir.join("args.txt")).unwrap();
    let mut files = GameFiles::default();
    let mut names: Vec<_> = std::fs::read_dir(dir.join("files")).unwrap().map(|e| e.unwrap().path()).collect();
    names.sort();
    for p in names { files.add(p.file_name().unwrap().to_str().unwrap(), std::fs::read(&p).unwrap()); }
    let mut b = cold_boot(&exe, &line, files).unwrap();
    let mut o = String::new();
    dump(&mut o, "BOOT", &b.cpu, &b.mem);
    let r = run(&mut b.cpu, &mut b.mem, &mut b.os, EXIT_PC, args[3].parse().unwrap(), steps_tab[n]);
    match r {
        Ok(s) => { let _ = writeln!(o, "STOP ok STEPS {s}"); }
        Err(e) => { let _ = writeln!(o, "STOP {e:?}"); }
    }
    dump(&mut o, "END", &b.cpu, &b.mem);
    for (a, v) in &b.os.chip_log { let _ = writeln!(o, "W {a:x} {v:x}"); }
    for (base, r) in regions(&b.mem) {
        for (i, v) in r.iter().enumerate() { if *v != 0 { let _ = writeln!(o, "M {:x} {v:x}", base as usize + i); } }
    }
    print!("{o}");
}
"#;

struct Case {
    name: &'static str,
    exe: Vec<u8>,
    args: &'static str,
    files: Vec<(&'static str, Vec<u8>)>,
}

fn cases() -> Vec<Case> {
    let level: Vec<u8> = (0..100u32).map(|i| (i * 7 + 3) as u8).collect();
    let readme: Vec<u8> = (0..60u32).map(|i| b'a' + (i % 26) as u8).collect();
    vec![
        Case {
            name: "slow_cli_rts",
            exe: program_a(),
            args: "-x level1\n",
            files: vec![("Level1.dat", level), ("other.bin", vec![1, 2, 3])],
        },
        Case {
            name: "chip_exit",
            exe: program_b(),
            args: "\n",
            files: vec![("ReadMe", readme)],
        },
    ]
}

/// Prepares the C kit's inputs and translates the program (`--emit rust --single`). Returns (gen dir, base, entry).
fn prepare(dir: &Path, c: &Case) -> (PathBuf, String, String) {
    let root = repo_root();
    fs::create_dir_all(dir.join("files")).unwrap();
    fs::create_dir_all(dir.join("out")).unwrap();
    fs::write(dir.join("prog.exe"), &c.exe).unwrap();
    fs::write(dir.join("args.txt"), c.args).unwrap();
    for (n, d) in &c.files {
        fs::write(dir.join("files").join(n), d).unwrap();
    }
    let prep = dir.join("prep.py");
    fs::write(&prep, PREP_PY).unwrap();
    let line = run_ok(
        Command::new("python3")
            .arg(&prep)
            .arg(&root)
            .arg(dir.join("prog.exe"))
            .arg(dir.join("out"))
            .arg(c.args),
    );
    let v: Vec<String> = line.split_whitespace().map(String::from).collect();
    let cfg = format!(
        "{{\"name\":\"synthetic\",\"segments\":[{{\"file\":\"out/image.bin\",\"addr\":\"0x{}\"}}],\
         \"code\":[\"0x{}-0x{}\"],\"entry\":\"0x{}\",\"ghidra\":false,\"seeds\":[],\"hle\":{{}},\"seed_addrs\":[]}}",
        v[0], v[2], v[3], v[1]
    );
    fs::write(dir.join("game.json"), cfg).unwrap();
    let gen = dir.join("gen");
    fs::create_dir_all(&gen).unwrap();
    run_ok(
        Command::new("python3")
            .arg(root.join("kit/amigo/port/m68k2c.py"))
            .arg(&gen)
            .args(["--config"])
            .arg(dir.join("game.json"))
            .args(["--no-speculative", "--emit", "rust", "--single"])
            .arg(gen.join("single.c"))
            .env("AMIGO_W", dir),
    );
    let stats = fs::read_to_string(gen.join("translate_stats.json")).unwrap();
    assert!(
        !stats.contains("\"rust_untranslated\": {\""),
        "{}: emitter left instructions untranslated",
        c.name
    );
    (gen, v[0].clone(), v[1].clone())
}

#[test]
fn cold_boot_and_os_services_match_c_kit() {
    if !have("python3") || !have("gcc") {
        eprintln!("skipped: needs python3 and gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-os-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).unwrap();
    let work = work.canonicalize().unwrap();
    let cases = cases();
    let driver = work.join("driver.c");
    fs::write(&driver, C_DRIVER).unwrap();
    let mut gens = Vec::new();
    let mut c_out = Vec::new();
    for (n, c) in cases.iter().enumerate() {
        let dir = work.join(format!("p{n}"));
        let (gen, base, entry) = prepare(&dir, c);
        let exe = dir.join("c_run");
        run_ok(
            Command::new("gcc")
                .args(["-O1", "-w", "-DAMIGO_NATIVE", "-I"])
                .arg(root.join("kit/amigo/port/runtime"))
                .arg("-I")
                .arg(root.join("kit/amigo/port"))
                .arg("-o")
                .arg(&exe)
                .arg(&driver)
                .arg(gen.join("single.c"))
                .arg(root.join("kit/amigo/port/oshle/oshle.c"))
                .arg(dir.join("out/oshle_cfg.c"))
                .arg("-lm"),
        );
        c_out.push(run_ok(
            Command::new(&exe)
                .arg(dir.join("out/execlib.bin"))
                .arg(dir.join("out/image.bin"))
                .args([&base, &entry])
                .arg(dir.join("files"))
                .arg(MAX_STEPS.to_string()),
        ));
        fs::write(dir.join("c_out.txt"), c_out.last().unwrap()).unwrap();
        gens.push(gen);
    }

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
        "{mods}\n{}",
        RUST_MAIN.replace("__STEP_TABLE__", &tab.join(", "))
    );
    fs::write(krate.join("src/main.rs"), main_rs).unwrap();
    fs::write(
        krate.join("Cargo.toml"),
        format!(
            "[package]\nname=\"os_check\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\n\
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
    let exe = work.join("target/debug/os_check");
    for (n, c) in cases.iter().enumerate() {
        let r = run_ok(
            Command::new(&exe)
                .arg(n.to_string())
                .arg(work.join(format!("p{n}")))
                .arg(MAX_STEPS.to_string()),
        );
        let lines: Vec<&str> = c_out[n].lines().collect();
        assert!(
            lines[1].starts_with("STOP ok"),
            "{}: C did not reach EXIT_PC:\n{}",
            c.name,
            lines[..3].join("\n")
        );
        eprintln!("{}: {} ({} lines)", c.name, lines[1], lines.len());
        if r != c_out[n] {
            let first = r.lines().zip(c_out[n].lines()).position(|(a, b)| a != b);
            panic!(
                "{}: Rust and C differ at line {:?}\nrust: {:?}\nc:    {:?}",
                c.name,
                first,
                first.and_then(|i| r.lines().nth(i)),
                first.and_then(|i| c_out[n].lines().nth(i))
            );
        }
    }
    if std::env::var_os("G5_KEEP").is_none() {
        let _ = fs::remove_dir_all(&work); // G5_KEEP=1 keeps the inputs and both programs for a look
    }
}
