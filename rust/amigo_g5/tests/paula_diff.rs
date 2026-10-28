//! M5 part 3 differential test: the Rust Paula (`paula.rs`) against the C audio code in `runtime/intentrt.c`.
//!
//! Each scenario is synthetic (no game data): 64 KB of chip RAM holding noise and sample blocks generated here
//! (square, saw and sine waves), then a stream of events: AUDx and DMACON writes, PAL lines played with the LED filter
//! in or out, and output-stage model changes. The C side takes everything from intentrt.c's "audio" section (channel
//! start and stop, chan_step, audio_line with the pt2-clone BLEP and RC filters, paula_setup) from its own text at test
//! time, and the AUDx and DMACON cases of write_custom copied below. Every output sample, the INTREQ bits each event
//! raises, the channel and filter state (bit for bit) every few lines, the BLEP table and the WAV file main.c `--wav`
//! writes must be identical. Skips cleanly without gcc.

use std::fs;
use std::process::Command;

use amigo_g5::display::Chip;
use amigo_g5::paula::{led_filter_on, write_wav, Model, Paula};

mod common;
use common::{have, repo_root, run_ok};

const CHIP: usize = 64 * 1024;

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

#[derive(Clone, Copy)]
enum Ev {
    /// A custom register write ($096 DMACON or $0A0-$0DF).
    Write(u16, u16),
    /// One line, with CIA-A PRA and DDRA as the game set them, and whether to dump the state after it.
    Line { pra: u8, ddra: u8, dump: bool },
    /// The output stage changed (0 raw, 1 A500, 2 A1200), as paula_set_model.
    Model(u8),
}

struct Scenario {
    name: String,
    model: u8,
    defer: bool,
    chip: Vec<u8>,
    ev: Vec<Ev>,
}

fn model(m: u8) -> Model {
    [Model::Raw, Model::A500, Model::A1200][m as usize]
}

/// Sample blocks at fixed places: a square at $1000, a saw at $2000, a sine at $3000 (256 bytes each), full-scale
/// steps at $4000; the rest noise.
fn chip_with_waves(seed: u64) -> Vec<u8> {
    let mut r = Rng(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
    let mut c: Vec<u8> = (0..CHIP).map(|_| r.next() as u8).collect();
    for k in 0..256 {
        c[0x1000 + k] = if k & 16 != 0 { 0x60 } else { 0xa0 };
        c[0x2000 + k] = k as u8;
        c[0x3000 + k] =
            ((k as f64 * std::f64::consts::TAU / 64.0).sin() * 120.0).round() as i8 as u8;
        c[0x4000 + k] = if k & 1 != 0 { 0x80 } else { 0x7f };
    }
    c
}

impl Scenario {
    fn new(name: &str, model: u8, defer: bool, seed: u64) -> Self {
        Scenario {
            name: name.into(),
            model,
            defer,
            chip: chip_with_waves(seed),
            ev: Vec::new(),
        }
    }
    /// Channel `n` set to play `len` words from `lc` at `per`, volume `vol`.
    fn voice(&mut self, n: u16, lc: u32, len: u16, per: u16, vol: u16) {
        let b = 0x0a0 + n * 16;
        self.ev.extend([
            Ev::Write(b, (lc >> 16) as u16),
            Ev::Write(b + 2, lc as u16),
            Ev::Write(b + 4, len),
            Ev::Write(b + 6, per),
            Ev::Write(b + 8, vol),
        ]);
    }
    fn dma(&mut self, v: u16) {
        self.ev.push(Ev::Write(0x096, v));
    }
    /// `n` lines with the LED filter in or out, dumping the state every 4th.
    fn lines(&mut self, n: usize, led: bool) {
        for k in 0..n {
            self.ev.push(Ev::Line {
                pra: if led { 0xfc } else { 0xfe },
                ddra: 0x03,
                dump: k % 4 == 3,
            });
        }
    }
    fn serialise(&self, b: &mut Vec<u8>) {
        b.extend([self.model, self.defer as u8]);
        b.extend_from_slice(&self.chip);
        b.extend_from_slice(&(self.ev.len() as u32).to_le_bytes());
        for e in &self.ev {
            match *e {
                Ev::Write(r, v) => {
                    b.push(0);
                    b.extend_from_slice(&r.to_le_bytes());
                    b.extend_from_slice(&v.to_le_bytes());
                }
                Ev::Line { pra, ddra, dump } => b.extend([1, pra, ddra, dump as u8]),
                Ev::Model(m) => b.extend([2, m]),
            }
        }
    }
}

fn hand_written() -> Vec<Scenario> {
    let mut v = Vec::new();
    for m in 0..3u8 {
        // one square wave, LED out then in
        let mut s = Scenario::new(&format!("square_m{m}"), m, false, v.len() as u64 + 1);
        s.voice(0, 0x1000, 128, 200, 64);
        s.dma(0x8201);
        s.lines(200, false);
        s.lines(200, true);
        v.push(s);
        // all four channels: square, saw, sine and full-scale steps (clipping), left and right
        let mut s = Scenario::new(&format!("four_channels_m{m}"), m, false, v.len() as u64 + 1);
        s.voice(0, 0x1000, 128, 254, 64);
        s.voice(1, 0x2000, 128, 428, 48);
        s.voice(2, 0x3000, 128, 160, 64);
        s.voice(3, 0x4000, 128, 124, 64);
        s.dma(0x820f);
        s.lines(312, true);
        s.lines(312, false);
        v.push(s);
    }
    // period below 124 plays at 124; volume above 64 at 64; LEN 0 is 65536 words
    let mut s = Scenario::new("clamps", 1, false, 10);
    s.voice(0, 0x2000, 0, 50, 99);
    s.voice(1, 0x3000, 1, 0, 64);
    s.dma(0x8203);
    s.lines(400, false);
    v.push(s);
    // LEN 1: a block ends and restarts (with its interrupt) several times a line
    let mut s = Scenario::new("short_blocks", 2, false, 11);
    s.voice(2, 0x4000, 1, 130, 40);
    s.dma(0x8204);
    s.lines(100, false);
    v.push(s);
    // DMACON before LC/LEN: without the g47 deferral the channel plays the old pointer, with it the new one
    for defer in [false, true] {
        let mut s = Scenario::new(&format!("dmacon_first_defer{}", defer as u8), 1, defer, 12);
        s.voice(0, 0x1000, 64, 300, 64);
        s.dma(0x8201);
        s.voice(0, 0x3000, 64, 300, 64);
        s.lines(150, true);
        // switched off and on again within a line, and off before a deferred start
        s.dma(0x0001);
        s.dma(0x8001);
        s.dma(0x0001);
        s.lines(10, true);
        s.dma(0x8001);
        s.lines(150, true);
        v.push(s);
    }
    // the master audio bit (DMACON bit 9) gates every channel
    let mut s = Scenario::new("master_bit", 1, false, 13);
    s.voice(0, 0x1000, 128, 200, 64);
    s.voice(1, 0x2000, 128, 300, 64);
    s.dma(0x8003);
    s.lines(20, false);
    s.dma(0x8200);
    s.lines(100, false);
    s.dma(0x0200);
    s.lines(50, false);
    s.dma(0x8200);
    s.lines(50, false);
    v.push(s);
    // AUDxDAT with DMA off asks for the next word; with DMA on it does not
    let mut s = Scenario::new("dat_writes", 1, false, 14);
    s.ev.extend([Ev::Write(0x0aa, 0x1234), Ev::Write(0x0da, 0x5678)]);
    s.dma(0x8208);
    s.ev.extend([Ev::Write(0x0da, 1), Ev::Write(0x0ca, 2)]);
    s.lines(4, false);
    v.push(s);
    // volume and period changed mid-play, every line
    let mut s = Scenario::new("sweeps", 1, false, 15);
    s.voice(0, 0x3000, 128, 400, 0);
    s.voice(1, 0x3000, 128, 400, 64);
    s.dma(0x8203);
    for k in 0..300u16 {
        s.ev.extend([
            Ev::Write(0x0a8, k % 70),
            Ev::Write(0x0b6, 124 + (k * 7) % 600),
        ]);
        s.lines(1, k % 50 < 25);
    }
    v.push(s);
    // the output stage changed while playing (the game's menu)
    let mut s = Scenario::new("model_changes", 0, false, 16);
    s.voice(0, 0x1000, 128, 180, 64);
    s.voice(1, 0x4000, 128, 140, 64);
    s.dma(0x8203);
    for m in [0u8, 1, 2, 0, 2, 1] {
        s.ev.push(Ev::Model(m));
        s.lines(80, m != 2);
    }
    v.push(s);
    v
}

fn random_scenario(seed: u64) -> Scenario {
    let mut r = Rng(seed * 7919 + 17);
    let mut s = Scenario::new(
        &format!("random_{seed}"),
        r.below(3) as u8,
        r.chance(30),
        seed + 1000,
    );
    let lines = 300 + r.below(700) as usize;
    let (mut pra, ddra) = (
        r.next() as u8,
        if r.chance(80) { 3 } else { r.next() as u8 },
    );
    for k in 0..lines {
        for _ in 0..r.below(4) {
            let reg = 0x0a0 + (r.below(4) as u16) * 16 + (r.below(6) as u16) * 2;
            let val = match reg & 0xf {
                0x0 => {
                    if r.chance(85) {
                        0
                    } else {
                        r.u16()
                    }
                }
                0x2 => r.u16(),
                0x4 => {
                    if r.chance(80) {
                        1 + r.below(300) as u16
                    } else {
                        r.u16()
                    }
                }
                0x6 => {
                    if r.chance(85) {
                        100 + r.below(800) as u16
                    } else {
                        r.u16()
                    }
                }
                0x8 => r.below(80) as u16,
                _ => r.u16(),
            };
            s.ev.push(Ev::Write(reg, val));
        }
        if r.chance(4) {
            let bits = (r.below(16) as u16) | if r.chance(60) { 0x200 } else { 0 };
            s.dma(bits | if r.chance(65) { 0x8000 } else { 0 });
        }
        if r.chance(1) {
            s.ev.push(Ev::Model(r.below(3) as u8));
        }
        if r.chance(2) {
            pra ^= 2;
        }
        s.ev.push(Ev::Line {
            pra,
            ddra,
            dump: k % 16 == 15 || k + 1 == lines,
        });
    }
    s
}

const C_DRIVER: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include "amiga.h"
uint8_t chipram[CHIP_SIZE]; uint16_t intena, intreq, dmacon; aud_t aud[4];
static struct { uint8_t pra, ddra; } cia[2];
static void prof_aud(int n) { (void)n; }
static void no_ipl(void) {} void (*ipl_changed)(void) = no_ipl;
@CRW@
@AUDIO@
/* intentrt.c write_custom: the DMACON case and the AUDx cases */
static void wr(int reg, uint16_t v) {
    reg &= 0x1fe;
    if (reg == 0x096) { uint16_t old = dmacon; if (v & 0x8000) dmacon |= v & 0x7fff; else dmacon &= ~v; audio_dmacon(old, dmacon); return; }
    if (reg >= 0x0a0 && reg < 0x0e0) {
        int n = (reg - 0xa0) >> 4, r = reg & 0xf;
        switch (r) {
        case 0x0: aud[n].lc = (aud[n].lc & 0xffff) | ((uint32_t)v << 16); break;
        case 0x2: aud[n].lc = (aud[n].lc & 0xffff0000) | (v & 0xfffe); break;
        case 0x4: aud[n].len = v; break;
        case 0x6: aud[n].per = v; break;
        case 0x8: aud[n].vol = v; break;
        case 0xa: aud[n].dat = v;
            if (!((dmacon & 0x200) && (dmacon & (1 << n)))) { intreq |= (uint16_t)(0x80 << n); ipl_changed(); }
            break;
        }
    }
}
static FILE *in, *out;
static void rd(void *p, size_t n) { if (fread(p, 1, n, in) != n) { fprintf(stderr, "short input\n"); exit(2); } }
static uint32_t u32(void) { uint8_t b[4]; rd(b, 4); return b[0] | b[1] << 8 | b[2] << 16 | (uint32_t)b[3] << 24; }
static uint16_t u16(void) { uint8_t b[2]; rd(b, 2); return (uint16_t)(b[0] | b[1] << 8); }
static uint8_t u8(void) { uint8_t b; rd(&b, 1); return b; }
static void o16(uint16_t v) { fwrite(&v, 2, 1, out); }
static void o32(uint32_t v) { fwrite(&v, 4, 1, out); }
static void o64(uint64_t v) { fwrite(&v, 8, 1, out); }
static uint64_t fb(float f) { uint32_t u; memcpy(&u, &f, 4); return u; }
static uint64_t db(double d) { uint64_t u; memcpy(&u, &d, 8); return u; }
/* the order of paula.rs state_bits */
static void dump_state(void) {
    o32(4 * 40 + 2 * 4 + 12 + 1);
    for (int n = 0; n < 4; n++) {
        o64(db(ch[n].phase)); o64(fb(plast[n]));
        o64((uint8_t)ch[n].cur); o64(ch[n].left); o64(ch[n].word); o64((uint64_t)ch[n].bytei);
        o64((uint64_t)pblep[n].index); o64((uint64_t)(int64_t)pblep[n].samplesLeft);
        for (int k = 0; k <= BLEP_RNS; k++) o64(fb(pblep[n].fBuffer[k]));
    }
    onePoleFilter_t *f[2] = { &pf_lo, &pf_hi };
    for (int k = 0; k < 2; k++) { o64(fb(f[k]->tmpL)); o64(fb(f[k]->tmpR)); o64(fb(f[k]->a0)); o64(fb(f[k]->b1)); }
    for (int k = 0; k < 4; k++) o64(fb(pf_led.tmpL[k]));
    for (int k = 0; k < 4; k++) o64(fb(pf_led.tmpR[k]));
    o64(fb(pf_led.a1)); o64(fb(pf_led.a2)); o64(fb(pf_led.b1)); o64(fb(pf_led.b2));
    o64(db(out_frac));
}
int main(int argc, char **argv) {
    unsetenv("AMIGO_PAULA"); unsetenv("AMIGO_AUD_DEFER");
    char p[4096]; snprintf(p, sizeof p, "%s/blep.bin", argv[2]); out = fopen(p, "wb");
    for (int k = 0; k < 257; k++) o32((uint32_t)fb(fMinBlepData[k]));
    fclose(out);
    in = fopen(argv[1], "rb"); if (!in) return 2;
    uint32_t ns = u32();
    for (uint32_t sc = 0; sc < ns; sc++) {
        memset(aud, 0, sizeof aud); memset(ch, 0, sizeof ch); memset(pblep, 0, sizeof pblep); memset(plast, 0, sizeof plast);
        memset(&pf_lo, 0, sizeof pf_lo); memset(&pf_hi, 0, sizeof pf_hi); memset(&pf_led, 0, sizeof pf_led);
        memset(aud_on, 0, sizeof aud_on); aud_irq_pending = aud_start_pending = 0; out_frac = 0; intreq = dmacon = 0; audio_len = 0;
        uint8_t m = u8(); aud_defer = u8(); paula_set_model(m);
        rd(chipram, CHIP_SIZE);
        snprintf(p, sizeof p, "%s/s%03u.bin", argv[2], sc); out = fopen(p, "wb");
        snprintf(p, sizeof p, "%s/s%03u.wav", argv[2], sc); FILE *wav = fopen(p, "wb");
        uint32_t wavbytes = 0; { uint8_t z[44] = {0}; fwrite(z, 1, 44, wav); }
        uint32_t ne = u32();
        for (uint32_t e = 0; e < ne; e++) {
            uint8_t k = u8();
            if (k == 0) { uint16_t reg = u16(), val = u16(); wr(reg, val); o16(intreq); intreq = 0; }
            else if (k == 1) {
                cia[0].pra = u8(); cia[0].ddra = u8(); uint8_t dump = u8();
                audio_line(); o16(intreq); intreq = 0;
                o32((uint32_t)audio_len); fwrite(audio_buf, 4, (size_t)audio_len, out);
                fwrite(audio_buf, 4, (size_t)audio_len, wav); wavbytes += (uint32_t)audio_len * 4; audio_len = 0;
                if (dump) dump_state();
            } else paula_set_model(u8());
        }
        fclose(out);
        { @WAVHDR@
          fseek(wav, 0, SEEK_SET); fwrite(h, 1, 44, wav); fclose(wav); }
    }
    return 0;
}
"#;

/// The text of `src` from the line holding `from` to the end of the line holding `to` (after `from`).
fn cut<'a>(src: &'a str, from: &str, to: &str) -> &'a str {
    let i = src
        .find(from)
        .unwrap_or_else(|| panic!("{from:?} not found"));
    let i = src[..i].rfind('\n').map_or(0, |k| k + 1);
    let j = i + src[i..]
        .find(to)
        .unwrap_or_else(|| panic!("{to:?} not found"));
    let j = j + src[j..].find('\n').unwrap() + 1;
    &src[i..j]
}

/// What the C driver writes for one scenario, made by the Rust side, with the byte offset each event's record
/// starts at (for the report). Also returns the samples, for the WAV file, and the number of interrupt bits raised.
fn run_rust(s: &Scenario) -> (Vec<u8>, Vec<usize>, Vec<i16>, usize) {
    let chip = Chip::new(&s.chip);
    let mut p = Paula::new(model(s.model), s.defer);
    let (mut b, mut at, mut all) = (Vec::new(), Vec::new(), Vec::new());
    let (mut dmacon, mut irqs) = (0u16, 0usize);
    for e in &s.ev {
        at.push(b.len());
        match *e {
            Ev::Write(reg, v) => {
                let irq = if reg == 0x096 {
                    let old = dmacon;
                    if v & 0x8000 != 0 {
                        dmacon |= v & 0x7fff;
                    } else {
                        dmacon &= !v;
                    }
                    p.dma_changed(old, dmacon);
                    0
                } else {
                    p.write(reg, v, dmacon)
                };
                irqs += irq.count_ones() as usize;
                b.extend_from_slice(&irq.to_le_bytes());
            }
            Ev::Line { pra, ddra, dump } => {
                let mut out = Vec::new();
                let irq = p.play_line(chip, led_filter_on(pra, ddra), &mut out);
                irqs += irq.count_ones() as usize;
                b.extend_from_slice(&irq.to_le_bytes());
                b.extend_from_slice(&((out.len() / 2) as u32).to_le_bytes());
                out.iter()
                    .for_each(|x| b.extend_from_slice(&x.to_le_bytes()));
                all.extend(out);
                if dump {
                    let st = p.state_bits();
                    b.extend_from_slice(&(st.len() as u32).to_le_bytes());
                    st.iter()
                        .for_each(|x| b.extend_from_slice(&x.to_le_bytes()));
                }
            }
            Ev::Model(m) => p.set_model(model(m)),
        }
    }
    (b, at, all, irqs)
}

#[test]
fn paula_matches_c_intentrt() {
    if !have("gcc") {
        eprintln!("skipped: needs gcc");
        return;
    }
    let root = repo_root();
    let work = std::env::temp_dir().join(format!("g5-paula-{}", std::process::id()));
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(work.join("out")).unwrap();
    let rt = root.join("kit/amigo/port/runtime");
    // the oracle: intentrt.c's audio section and its chip word read, and main.c's WAV header, taken from their text
    let intentrt = fs::read_to_string(rt.join("intentrt.c")).unwrap();
    let audio = cut(
        &intentrt,
        "/* ---------------- audio (state + interrupt timing)",
        "    aud_irq_pending = 0;",
    );
    let main_c = fs::read_to_string(root.join("kit/amigo/port/main.c")).unwrap();
    let driver = C_DRIVER
        .replace(
            "@CRW@",
            cut(&intentrt, "static inline uint16_t crw(", "crw("),
        )
        .replace("@AUDIO@", &format!("{audio}}}\n"))
        .replace(
            "@WAVHDR@",
            cut(&main_c, "uint8_t h[44];", "memcpy(h + 40, &wavbytes, 4);"),
        );
    fs::write(work.join("driver.c"), driver).unwrap();
    let exe = work.join("c_paula");
    // -ffp-contract=off: no fused multiply-adds, so the C rounds each float operation as the Rust does
    run_ok(
        Command::new("gcc")
            .args([
                "-O1",
                "-w",
                "-ffp-contract=off",
                &format!("-DCHIP_SIZE={CHIP}"),
                "-I",
            ])
            .arg(&rt)
            .arg("-o")
            .arg(&exe)
            .arg(work.join("driver.c"))
            .arg("-lm"),
    );
    let mut sc = hand_written();
    sc.extend((0..120).map(random_scenario));
    let mut input = (sc.len() as u32).to_le_bytes().to_vec();
    sc.iter().for_each(|s| s.serialise(&mut input));
    fs::write(work.join("in.bin"), &input).unwrap();
    run_ok(
        Command::new(&exe)
            .arg(work.join("in.bin"))
            .arg(work.join("out")),
    );

    let blep: Vec<u32> = fs::read(work.join("out/blep.bin"))
        .unwrap()
        .chunks(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    assert_eq!(blep, Paula::blep_table_bits(), "BLEP table");

    let (mut lines, mut samples, mut loud, mut irqs, mut clipped) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for (n, s) in sc.iter().enumerate() {
        let (rust, at, all, irq) = run_rust(s);
        irqs += irq;
        let bpath = work.join(format!("out/s{n:03}.bin"));
        let wpath = work.join(format!("out/s{n:03}.wav"));
        let c = fs::read(&bpath).unwrap();
        if rust != c {
            let k = (0..rust.len().min(c.len()))
                .find(|&k| rust[k] != c[k])
                .unwrap_or(rust.len().min(c.len()));
            let e = at.iter().rposition(|&o| o <= k).unwrap_or(0);
            panic!(
                "{}: output differs at byte {k} (event {e} of {}, record at byte {}); lengths rust {} c {}",
                s.name,
                s.ev.len(),
                at[e],
                rust.len(),
                c.len()
            );
        }
        let mut w = Vec::new();
        write_wav(&mut w, &all).unwrap();
        assert!(
            w == fs::read(&wpath).unwrap(),
            "{}: WAV file differs",
            s.name
        );
        lines += s.ev.iter().filter(|e| matches!(e, Ev::Line { .. })).count();
        samples += all.len() / 2;
        loud += all.iter().filter(|&&x| x != 0).count();
        clipped += all
            .iter()
            .filter(|&&x| x == i16::MAX || x == i16::MIN)
            .count();
        let _ = fs::remove_file(bpath);
        let _ = fs::remove_file(wpath);
    }
    eprintln!(
        "{} scenarios, {lines} lines, {samples} stereo samples ({loud} values not zero, {clipped} clipped), \
         {irqs} audio interrupts: identical, WAV files identical",
        sc.len()
    );
    // the scenarios must have made sound, clipped somewhere and raised interrupts
    assert!(loud > samples && clipped > 0 && irqs > 1000);
    let _ = fs::remove_dir_all(&work);
}
