//! The clean-sheet display: the copper list read once per frame as a table, and the frame composed from the screen the
//! game set up (bitplane bases, strides, palette, sprites). No beam, no copper execution, no cycle timing.
//!
//! Ported from `kit/amigo/port/runtime/intent.c` (M5, 28 Oct 2026), which is the oracle: `tests/display_diff.rs` compiles
//! intent.c with gcc and compares the table and every pixel. The C's environment switches are fixed at their defaults
//! (decision 28 Oct 2026): COPJMP and COPTIME on, mid-line colour changes on, 6 planes (HAM6, EHB) on, sprites on, hires
//! reduced by taking the left pixel. Only the whole-frame compose (`intent_compose`) is ported, not the progressive one.

use std::io::{self, Write};
use std::path::Path;

/// Output window in lores pixels, and where it sits (amiga.h SCR_W, SCR_H, SCR_HSTART, SCR_VSTART).
pub const SCR_W: usize = 352;
pub const SCR_H: usize = 272;
pub const SCR_HSTART: i32 = 0x70;
pub const SCR_VSTART: i32 = 0x24;

const MAX_CHG: usize = 4096;
const MAX_CB: usize = 256;

/// Chip RAM as the display reads it: word addresses wrap at its (power of two) size, as Agnus does (amiga.h CHIP_WMASK).
#[derive(Clone, Copy)]
pub struct Chip<'a> {
    ram: &'a [u8],
    wmask: u32,
}

impl<'a> Chip<'a> {
    /// Panics unless the size is a power of two of at least 4 bytes (the C requires the same of CHIP_SIZE).
    pub fn new(ram: &'a [u8]) -> Self {
        assert!(
            ram.len().is_power_of_two() && ram.len() >= 4,
            "chip RAM size must be a power of two"
        );
        Chip {
            ram,
            wmask: (ram.len() as u32 - 1) & !1,
        }
    }
    pub fn wmask(&self) -> u32 {
        self.wmask
    }
    /// intent.c in_rw.
    pub fn r16(&self, a: u32) -> u16 {
        let a = (a & self.wmask) as usize;
        u16::from_be_bytes([self.ram[a], self.ram[a + 1]])
    }
}

/// One register change in the table: "from line N (at copper position hpos), register R = V" (intent.c ichg_t).
/// hpos -1 marks a CPU write, whose position in the line is unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    pub line: i32,
    pub hpos: i32,
    pub seq: i32,
    pub reg: u16,
    pub val: u16,
}

/// The copper list as a table (intent.c itable_t), plus the COP1LC/COP2LC values the list itself moved (G2E9 coplc).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CopperTable {
    pub changes: Vec<Change>,
    /// "At line N raise these interrupt request bits" (copper MOVEs to INTREQ with the set bit).
    pub callbacks: Vec<(i32, u16)>,
    pub skips: i32,
    pub cop1: u32,
    pub cop2: u32,
    pub cop1_written: bool,
    pub cop2_written: bool,
}

/// Mirrors intent.c intent_read_table_ex with AMIGO_INTENT_COPJMP and AMIGO_INTENT_COPTIME on.
pub fn read_table(
    chip: Chip,
    cop: u32,
    mut cop1: u32,
    mut cop2: u32,
    start_line: i32,
) -> CopperTable {
    let w = chip.wmask();
    let mut t = CopperTable {
        cop1,
        cop2,
        ..Default::default()
    };
    let (mut line, mut hpos) = (start_line, 0i32);
    let mut skipnext = false;
    let mut jumps: Vec<(u32, i32)> = Vec::new();
    let mut a = cop & w;
    'copper: for _ in 0..8000 {
        let (w1, w2) = (chip.r16(a), chip.r16(a.wrapping_add(2)));
        if w1 == 0xffff && w2 == 0xfffe {
            break;
        }
        // chipset.c runs the copper to $E4, then the next line from 0
        if hpos >= 0xe4 {
            line += 1;
            hpos = 0;
            if line > 312 {
                break;
            }
        }
        hpos += 4;
        'body: {
            if skipnext {
                skipnext = false;
                break 'body;
            }
            if w1 & 1 == 0 {
                let reg = w1 & 0x1fe;
                let (hi, lo) = ((w2 as u32) << 16, (w2 & 0xfffe) as u32);
                match reg {
                    0x80 => {
                        (t.cop1, t.cop1_written, cop1) =
                            ((t.cop1 & 0xffff) | hi, true, (cop1 & 0xffff) | hi)
                    }
                    0x82 => {
                        (t.cop1, t.cop1_written, cop1) =
                            ((t.cop1 & 0xffff_0000) | lo, true, (cop1 & 0xffff_0000) | lo)
                    }
                    0x84 => {
                        (t.cop2, t.cop2_written, cop2) =
                            ((t.cop2 & 0xffff) | hi, true, (cop2 & 0xffff) | hi)
                    }
                    0x86 => {
                        (t.cop2, t.cop2_written, cop2) =
                            ((t.cop2 & 0xffff_0000) | lo, true, (cop2 & 0xffff_0000) | lo)
                    }
                    0x88 | 0x8a => {
                        // COPJMP1/2: a jump back to a target already taken at this line would loop until a WAIT that is never met
                        let to = if reg == 0x88 { cop1 } else { cop2 } & w;
                        if jumps.contains(&(to, line)) || jumps.len() >= 32 {
                            break 'copper;
                        }
                        jumps.push((to, line));
                        a = to.wrapping_sub(4) & w;
                        break 'body;
                    }
                    _ => {}
                }
                if reg == 0x9c {
                    if w2 & 0x8000 != 0 && t.callbacks.len() < MAX_CB {
                        t.callbacks.push((line, w2 & 0x7fff));
                    }
                } else if reg >= 0x40 && t.changes.len() < MAX_CHG {
                    let seq = t.changes.len() as i32;
                    t.changes.push(Change {
                        line,
                        hpos,
                        seq,
                        reg,
                        val: w2,
                    });
                }
            } else if w2 & 1 != 0 {
                // SKIP against the table's beam, 8-bit line as chipset.c
                t.skips += 1;
                let (v, h) = ((w1 >> 8) as i32, (w1 & 0xfe) as i32);
                let vl = line & 0xff;
                if vl > v || (vl == v && hpos >= h) {
                    skipnext = true;
                }
            } else {
                // WAIT, as chipset.c copper_run_until: 8-bit beam line, a WAIT for h >= $E4 is met at the next line's position 2
                let (v, h) = ((w1 >> 8) as i32, (w1 & 0xfe) as i32);
                if hpos >= 0xe4 {
                    line += 1;
                    hpos = 0;
                }
                let vp = line & 0xff;
                if vp > v || (vp == v && hpos >= h) {
                    hpos += 2;
                } else {
                    if vp < v {
                        line += v - vp;
                    }
                    if h >= 0xe4 {
                        line += 1;
                        hpos = 2;
                    } else {
                        hpos = h + 2;
                    }
                }
                if line > 312 {
                    break 'copper;
                }
            }
        }
        a = a.wrapping_add(4) & w;
    }
    t
}

/// Mirrors intent.c coplc_plausible: a copper list that ends within 4000 instructions and does not start with zeros.
pub fn coplc_plausible(chip: Chip, a: u32) -> bool {
    let mut a = a & chip.wmask();
    if chip.r16(a) == 0 && chip.r16(a.wrapping_add(2)) == 0 {
        return false;
    }
    for _ in 0..4000 {
        if chip.r16(a) == 0xffff && chip.r16(a.wrapping_add(2)) == 0xfffe {
            return true;
        }
        a = a.wrapping_add(4) & chip.wmask();
    }
    false
}

/// What the game has set up: the register shadow, palette, bitplane pointers and effective DMACON (intent.c istate_t,
/// with the running sprite pointers).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenState {
    pub reg: [u16; 256],
    pub pal: [u16; 32],
    pub bpl: [u32; 6],
    pub spt: [u32; 8],
    pub dmacon: u16,
}

impl Default for ScreenState {
    fn default() -> Self {
        ScreenState {
            reg: [0; 256],
            pal: [0; 32],
            bpl: [0; 6],
            spt: [0; 8],
            dmacon: 0,
        }
    }
}

fn set_hi_lo(old: u32, reg: u16, val: u16) -> u32 {
    if reg & 2 != 0 {
        (old & 0xffff_0000) | (val & 0xfffe) as u32
    } else {
        (old & 0xffff) | (val as u32) << 16
    }
}

impl ScreenState {
    /// Mirrors intent.c intent_apply.
    pub fn apply(&mut self, reg: u16, val: u16) {
        if (0x180..0x1c0).contains(&reg) {
            self.pal[((reg - 0x180) >> 1) as usize] = val & 0xfff;
        } else if (0xe0..0xf8).contains(&reg) {
            let p = ((reg - 0xe0) >> 2) as usize;
            if p < 6 {
                self.bpl[p] = set_hi_lo(self.bpl[p], reg, val);
            }
        } else {
            self.reg[(reg >> 1) as usize] = val;
            if (0x120..0x140).contains(&reg) {
                let n = ((reg - 0x120) >> 2) as usize;
                self.spt[n] = set_hi_lo(self.spt[n], reg, val);
            } else if reg == 0x96 {
                if val & 0x8000 != 0 {
                    self.dmacon |= val & 0x7fff;
                } else {
                    self.dmacon &= !val;
                }
            }
        }
    }
}

/// 12-bit Amiga colour to the frame buffer's 0xAARRGGBB (intentrt.c rgb12).
pub fn rgb12(c: u16) -> u32 {
    let (r, g, b) = (
        ((c >> 8) & 15) as u32,
        ((c >> 4) & 15) as u32,
        (c & 15) as u32,
    );
    0xff00_0000 | (r * 17) << 16 | (g * 17) << 8 | (b * 17)
}

/// EHB: colours 32-63 are 0-31 at half brightness (intent.c in_col).
fn colour(pal: &[u16; 32], v: u8) -> u16 {
    if v >= 32 {
        (pal[(v & 31) as usize] >> 1) & 0x777
    } else {
        pal[v as usize]
    }
}

/// One frame to compose: the state at its start and its copper table.
#[derive(Debug, Clone)]
pub struct Frame {
    pub start: ScreenState,
    pub table: CopperTable,
}

#[derive(Clone, Copy, Default)]
struct Sprite {
    state: u8,
    pos: u16,
    ctl: u16,
    da: u16,
    db: u16,
    vstart: i32,
    vstop: i32,
    hstart: i32,
}

/// Sprite fetch for one frame, replayed per composed line (intent.c in_spr, in_sprpix, in_sprpair).
struct Sprites {
    spr: [Sprite; 8],
    pix: [u8; 1024],
    pair: [u8; 1024],
}

impl Sprites {
    fn poslc(&mut self, chip: Chip, s: &mut ScreenState, n: usize) {
        let a = s.spt[n] & chip.wmask();
        let p = &mut self.spr[n];
        p.pos = chip.r16(a);
        p.ctl = chip.r16(a + 2);
        s.spt[n] = a + 4;
        p.vstart = (p.pos >> 8) as i32 | ((p.ctl & 4) as i32) << 6;
        p.vstop = (p.ctl >> 8) as i32 | ((p.ctl & 2) as i32) << 7;
        p.hstart = ((p.pos & 0xff) as i32) << 1 | (p.ctl & 1) as i32;
    }

    /// Mirrors intent.c in_sprite_line.
    fn line(&mut self, chip: Chip, s: &mut ScreenState, l: i32) {
        self.pix = [0; 1024];
        if s.dmacon & 0x220 != 0x220 {
            return;
        }
        let mut act = [false; 8];
        for n in (0..8).rev() {
            if l < 0x19 {
                continue;
            }
            if self.spr[n].state == 0 {
                self.poslc(chip, s, n);
                if self.spr[n].pos == 0 && self.spr[n].ctl == 0 {
                    self.spr[n].state = 3;
                    continue;
                }
                self.spr[n].state = 1;
            }
            if self.spr[n].state == 1 && l == self.spr[n].vstart {
                self.spr[n].state = 2;
            }
            if self.spr[n].state != 2 {
                continue;
            }
            if l >= self.spr[n].vstop {
                self.poslc(chip, s, n);
                let p = &mut self.spr[n];
                if p.pos == 0 && p.ctl == 0 {
                    p.state = 3;
                    continue;
                }
                p.state = if l == p.vstart { 2 } else { 1 };
                if p.state != 2 {
                    continue;
                }
            }
            let a = s.spt[n] & chip.wmask();
            self.spr[n].da = chip.r16(a);
            self.spr[n].db = chip.r16(a + 2);
            s.spt[n] = a + 4;
            act[n] = true;
        }
        if !act.iter().any(|&x| x) {
            return;
        }
        let bits =
            |sp: &Sprite, i: i32| ((sp.da >> (15 - i)) & 1 | ((sp.db >> (15 - i)) & 1) << 1) as u8;
        for q in (0..4).rev() {
            let (e, o) = (self.spr[2 * q], self.spr[2 * q + 1]);
            let (ae, ao) = (act[2 * q], act[2 * q + 1]);
            if !ae && !ao {
                continue;
            }
            let attached = ao && o.ctl & 0x80 != 0;
            for i in 0..16 {
                if ae {
                    let h = (e.hstart + 1 + i) as usize;
                    let ve = bits(&e, i);
                    if attached {
                        let oi = h as i32 - (o.hstart + 1);
                        let vo = if (0..16).contains(&oi) {
                            bits(&o, oi)
                        } else {
                            0
                        };
                        if ve | vo != 0 {
                            self.pix[h] = 16 + ((vo << 2) | ve);
                            self.pair[h] = q as u8;
                        }
                    } else if ve != 0 {
                        self.pix[h] = 16 + q as u8 * 4 + ve;
                        self.pair[h] = q as u8;
                    }
                }
                if ao {
                    let h = (o.hstart + 1 + i) as usize;
                    let vo = bits(&o, i);
                    let ei = h as i32 - (e.hstart + 1);
                    let ve = if ae && (0..16).contains(&ei) {
                        bits(&e, ei)
                    } else {
                        0
                    };
                    if attached {
                        if ve | vo != 0 {
                            self.pix[h] = 16 + ((vo << 2) | ve);
                            self.pair[h] = q as u8;
                        }
                    } else if vo != 0 && ve == 0 {
                        self.pix[h] = 16 + q as u8 * 4 + vo;
                        self.pair[h] = q as u8;
                    }
                }
            }
        }
    }
}

/// The game's own register writes during a frame, stamped with the line they happen at (intent.c cpuw).
#[derive(Debug, Clone, Default)]
pub struct CpuWrites(Vec<Change>);

impl CpuWrites {
    /// Mirrors the recording half of intent.c intent_cpu_write. Not done (28 Oct 2026): the C also re-reads the copper
    /// list on a CPU write to COP2LC or COPJMP1/2 mid-frame (intent_copper_cpu); here those writes only enter the list.
    pub fn push(&mut self, line: i32, reg: u16, val: u16) {
        if reg >= 0x40 && self.0.len() < MAX_CHG {
            let seq = self.0.len() as i32;
            self.0.push(Change {
                line,
                hpos: -1,
                seq,
                reg,
                val,
            });
        }
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

/// The register shadow a frame starts from (what the game has written to the custom chips).
#[derive(Debug, Clone)]
pub struct Shadow {
    pub custom: [u16; 256],
    pub palette: [u16; 32],
    pub bplpt: [u32; 6],
    pub dmacon: u16,
}

/// Frame to frame display state: where the sprite pointers stand at the end of the last composed frame.
#[derive(Debug, Clone, Default)]
pub struct Display {
    spt_next: Option<[u32; 8]>,
}

impl Display {
    /// Mirrors intent.c intent_frame_begin and intent_read_table: snapshot the shadow and read the copper list.
    pub fn frame_begin(&self, chip: Chip, sh: &Shadow, cop: u32) -> Frame {
        let mut start = ScreenState {
            reg: sh.custom,
            pal: sh.palette,
            bpl: sh.bplpt,
            dmacon: sh.dmacon,
            spt: [0; 8],
        };
        for n in 0..8 {
            start.spt[n] = match self.spt_next {
                Some(s) => s[n],
                None => {
                    (((sh.custom[0x90 + n * 2] as u32) << 16)
                        | (sh.custom[0x91 + n * 2] & 0xfffe) as u32)
                        & chip.wmask()
                }
            };
        }
        let cop2 = ((sh.custom[0x84 >> 1] as u32) << 16) | (sh.custom[0x86 >> 1] & 0xfffe) as u32;
        let table = read_table(chip, cop, cop, cop2, 0);
        Frame { start, table }
    }

    /// Mirrors intent.c intent_compose (intent_compose_begin plus intent_compose_to line 312): the whole frame into
    /// `out` (SCR_W x SCR_H, 0xAARRGGBB), and the sprite pointers carried to the next frame.
    pub fn compose(&mut self, chip: Chip, f: &Frame, cpuw: &CpuWrites, out: &mut [u32]) {
        assert_eq!(out.len(), SCR_W * SCR_H);
        let (t, cpuw) = (&f.table.changes, &cpuw.0);
        let mut s = f.start.clone();
        let mut spr = Sprites {
            spr: [Sprite::default(); 8],
            pix: [0; 1024],
            pair: [0; 1024],
        };
        let (mut ci, mut wi) = (0usize, 0usize);
        let mut mid: Vec<Change> = Vec::new();
        let mut defr: Vec<Change> = Vec::new();
        for l in 0..312 {
            mid.clear();
            defr.clear();
            let wi0 = wi;
            // COPTIME: a non-colour MOVE that executes at or after DDFSTRT-$10 shows from the next line
            let d0 = (s.reg[0x92 >> 1] & 0xfc) as i32;
            let tm1 = if d0 > 0x18 { d0 - 0x10 } else { 0x08 };
            while ci < t.len() && t[ci].line <= l {
                let c = t[ci];
                ci += 1;
                let is_col = (0x180..0x1c0).contains(&c.reg);
                if c.line == l && is_col && 2 * c.hpos + 1 > SCR_HSTART {
                    mid.push(c);
                } else if c.line == l && c.hpos >= 4 && c.hpos - 4 >= tm1 && !is_col {
                    defr.push(c);
                } else {
                    s.apply(c.reg, c.val);
                }
            }
            while wi < cpuw.len() && cpuw[wi].line <= l {
                s.apply(cpuw[wi].reg, cpuw[wi].val);
                wi += 1;
            }
            let y = l - SCR_VSTART;
            let (con0, con1) = (s.reg[0x100 >> 1], s.reg[0x102 >> 1]);
            let mut nplanes = ((con0 >> 12) & 7) as usize;
            let (hires, ham, dpf) = (con0 & 0x8000 != 0, con0 & 0x800 != 0, con0 & 0x400 != 0);
            let (dws, dwp) = (s.reg[0x8e >> 1], s.reg[0x90 >> 1]);
            // DIWSTOP vertical: bit 8 is the complement of bit 7
            let vstart = (dws >> 8) as i32;
            let vstop = (dwp >> 8) as i32 | if dwp & 0x8000 != 0 { 0 } else { 0x100 };
            let (hstart, hstop) = ((dws & 0xff) as i32, (dwp & 0xff) as i32 | 0x100);
            let mut ddfstrt = (s.reg[0x92 >> 1] & 0xfc) as i32;
            let mut ddfstop = (s.reg[0x94 >> 1] & 0xfc) as i32;
            if hires && nplanes > 4 {
                nplanes = 4;
            }
            let ham6 = !hires && ham && !dpf && nplanes == 6;
            let ehb = !hires && !ham && !dpf && nplanes == 6;
            let disp = l >= vstart && l < vstop && nplanes > 0;
            if nplanes > 5 && !ham6 && !ehb {
                nplanes = 5;
            }
            let mut pix = [0u8; 1024];
            spr.line(chip, &mut s, l);
            if disp {
                ddfstrt = ddfstrt.max(0x18);
                ddfstop = ddfstop.min(0xd8);
                let units = (((ddfstop - ddfstrt) >> 3) + 1).max(0);
                let words = if hires { units * 2 } else { units };
                let first = if hires {
                    ddfstrt * 2 + 9
                } else {
                    ddfstrt * 2 + 0x11
                };
                let npix = words * 16 + 16;
                let mut idx = vec![0u8; (npix + 32) as usize];
                for p in 0..nplanes {
                    let a = s.bpl[p] & chip.wmask();
                    let shift = if p & 1 != 0 { 4 } else { 0 };
                    let mut dly = ((con1 >> shift) & 15) as i32;
                    if hires {
                        dly *= 2;
                    }
                    for w in 0..words {
                        let d = chip.r16(a.wrapping_add(w as u32 * 2));
                        for b in 0..16 {
                            let x = w * 16 + b + dly;
                            if d & (0x8000 >> b) != 0 && x < npix + 31 {
                                idx[x as usize] |= 1 << p;
                            }
                        }
                    }
                    let modulo = s.reg[if p & 1 != 0 { 0x10a >> 1 } else { 0x108 >> 1 }] as i16;
                    s.bpl[p] = a
                        .wrapping_add(words as u32 * 2)
                        .wrapping_add(modulo as i32 as u32)
                        & chip.wmask();
                }
                for h in hstart..hstop.min(1024) {
                    let x = h - first;
                    let xi = if hires { x * 2 } else { x };
                    if x >= 0 && xi < npix + 31 {
                        pix[h as usize] = idx[xi as usize];
                    }
                }
            }
            let mut pal = s.pal;
            let mut mi = 0;
            if (0..SCR_H as i32).contains(&y) {
                let mut hamc = pal[0]; // HAM hold: COLOR00 at the start of every line
                let row = &mut out[y as usize * SCR_W..(y as usize + 1) * SCR_W];
                for (x, px) in row.iter_mut().enumerate() {
                    let h = SCR_HSTART + x as i32;
                    // the C tests 2 * hpos + 1 <= h, the same for integers
                    while mi < mid.len() && 2 * mid[mi].hpos < h {
                        pal[((mid[mi].reg - 0x180) >> 1) as usize] = mid[mi].val & 0xfff;
                        mi += 1;
                    }
                    let v = if disp { pix[h as usize] } else { 0 };
                    let mut c = if disp && ham6 {
                        let dat = (v & 15) as u16;
                        hamc = match v >> 4 {
                            0 => pal[dat as usize],
                            1 => (hamc & 0xff0) | dat,
                            2 => (hamc & 0x0ff) | dat << 8,
                            _ => (hamc & 0xf0f) | dat << 4,
                        };
                        rgb12(hamc)
                    } else {
                        rgb12(colour(&pal, v))
                    };
                    let sp = spr.pix[h as usize];
                    if sp != 0 && h >= hstart && h < hstop && l >= vstart && l < vstop {
                        // chipset.c's sprite and playfield priority, inside the display window only
                        let con2 = s.reg[0x104 >> 1];
                        let pair = spr.pair[h as usize] as u16;
                        let front = if dpf {
                            let p1 = (v & 1) | ((v >> 1) & 2) | ((v >> 2) & 4);
                            let p2 = ((v >> 1) & 1) | ((v >> 2) & 2) | ((v >> 3) & 4);
                            !(p1 != 0 && pair >= con2 & 7) && !(p2 != 0 && pair >= (con2 >> 3) & 7)
                        } else {
                            v == 0 || pair < con2 & 7
                        };
                        if front {
                            c = rgb12(pal[(sp & 31) as usize]);
                        }
                    }
                    *px = c;
                }
            }
            if !mid.is_empty() {
                for m in &mid[mi..] {
                    pal[((m.reg - 0x180) >> 1) as usize] = m.val & 0xfff;
                }
                s.pal = pal;
                // CPU writes still win after the line
                for k in &cpuw[wi0..wi] {
                    if (0x180..0x1c0).contains(&k.reg) {
                        s.apply(k.reg, k.val);
                    }
                }
            }
            for c in &defr {
                s.apply(c.reg, c.val);
            }
        }
        spr.line(chip, &mut s, 312); // chipset.c also runs sprite DMA on line 312
        self.spt_next = Some(s.spt);
    }
}

/// One frame as the kit's raw RGBA dump (amigo_img.h amigo_img_save, `.rgba`): "RGBA", width and height as
/// little-endian u32, then R, G, B, 255 per pixel, top row first. judge.py `_img` reads this.
pub fn write_rgba<W: Write>(mut w: W, px: &[u32], width: usize, height: usize) -> io::Result<()> {
    assert_eq!(px.len(), width * height);
    let mut b = Vec::with_capacity(12 + px.len() * 4);
    b.extend_from_slice(b"RGBA");
    b.extend_from_slice(&(width as u32).to_le_bytes());
    b.extend_from_slice(&(height as u32).to_le_bytes());
    for &c in px {
        b.extend_from_slice(&[(c >> 16) as u8, (c >> 8) as u8, c as u8, 255]);
    }
    w.write_all(&b)
}

/// Writes tick `tick`'s frame as `dir/pNNNNN.rgba`, the name judge.py looks for in a port's directory.
pub fn dump_frame(dir: &Path, tick: u32, px: &[u32]) -> io::Result<()> {
    let f = std::fs::File::create(dir.join(format!("p{tick:05}.rgba")))?;
    write_rgba(io::BufWriter::new(f), px, SCR_W, SCR_H)
}
