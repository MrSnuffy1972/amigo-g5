//! The clean-sheet blitter: each blit the game starts is classified by what it achieves (clear, copy, shifted copy,
//! bob draw, area fill, line draw) and done at once on chip RAM. No cycle model, no DMA slots, no busy time.
//!
//! Ported from `kit/amigo/port/runtime/intentrt.c` (the native blit classes, `op_blit_*` and `intent_blit_dispatch`;
//! M5 part 2, 28 Oct 2026), which is the oracle: `tests/blit_diff.rs` compiles that part of intentrt.c with gcc and
//! compares chip RAM, the blitter registers and the class of every blit. Address arithmetic follows the C exactly:
//! 32-bit wrapping sums, masked to a chip word address where the C masks (CHIP_WMASK), so pointers left in the
//! registers after a blit are the C's.

/// What a blit achieves (intentrt.c `blit_class_counts` index in brackets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlitKind {
    /// Any blit the classes below do not cover: the general minterm data path (class 0, G2E4 genblit).
    General = 0,
    /// Clear or constant fill (class 1).
    Clear = 1,
    /// Plain copy from A or B (class 2).
    Copy = 2,
    /// Copy from A through the shifter (class 3).
    Shift = 3,
    /// Masked bob or font draw, D = A ? B : C (class 4).
    DrawBob = 4,
    /// Area fill (class 5), done by the general path.
    Fill = 5,
    /// Line draw (class 6, G2E6 lineblit).
    Line = 6,
    /// No destination: writes nothing (class 7).
    NoDest = 7,
}

/// Chip RAM as the blitter writes it: word addresses wrap at its (power of two) size (amiga.h CHIP_WMASK).
pub struct ChipMut<'a> {
    ram: &'a mut [u8],
    wmask: u32,
}

impl<'a> ChipMut<'a> {
    /// Panics unless the size is a power of two of at least 4 bytes (the C requires the same of CHIP_SIZE).
    pub fn new(ram: &'a mut [u8]) -> Self {
        assert!(
            ram.len().is_power_of_two() && ram.len() >= 4,
            "chip RAM size must be a power of two"
        );
        let wmask = (ram.len() as u32 - 1) & !1;
        ChipMut { ram, wmask }
    }
    /// intentrt.c ib_r16.
    fn r16(&self, a: u32) -> u16 {
        let a = (a & self.wmask) as usize;
        u16::from_be_bytes([self.ram[a], self.ram[a + 1]])
    }
    /// intentrt.c ib_w16.
    fn w16(&mut self, a: u32, v: u16) {
        let a = (a & self.wmask) as usize;
        self.ram[a..a + 2].copy_from_slice(&v.to_be_bytes());
    }
}

const FIRST: u16 = 0x040;
const LAST: u16 = 0x074;

/// The blitter's registers ($040 BLTCON0 to $074 BLTADAT) as the game last wrote them. No global state: the runtime
/// owns one of these.
#[derive(Debug, Clone, Default)]
pub struct Blitter {
    regs: [u16; ((LAST - FIRST) / 2 + 1) as usize],
}

/// C's `(int32_t)p + m` stored back into a uint32_t.
fn add(p: u32, m: i32) -> u32 {
    p.wrapping_add(m as u32)
}

/// intentrt.c ib_minterm: LF bit 7 = ABC ... bit 0 = ~A~B~C.
fn minterm(lf: u16, a: u16, b: u16, c: u16) -> u16 {
    (0..8).filter(|k| lf & (1 << k) != 0).fold(0, |d, k| {
        let ta = if k & 4 != 0 { a } else { !a };
        let tb = if k & 2 != 0 { b } else { !b };
        let tc = if k & 1 != 0 { c } else { !c };
        d | (ta & tb & tc)
    })
}

/// First and last word masks for word `x` of a row `w` words wide.
fn edge(x: u16, w: u16, fwm: u16, lwm: u16) -> u16 {
    let mut m = 0xffff;
    if x == 0 {
        m &= fwm;
    }
    if x == w - 1 {
        m &= lwm;
    }
    m
}

/// One blit's set-up, read from the registers the way intent_blit_dispatch reads them.
struct Setup {
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

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;

impl Setup {
    fn ash(&self) -> u32 {
        (self.con0 >> 12) as u32
    }
    fn bsh(&self) -> u32 {
        (self.con1 >> 12) as u32
    }
    fn uses(&self) -> u16 {
        (self.con0 >> 8) & 15
    }
    fn lf(&self) -> u16 {
        self.con0 & 0xff
    }
    fn desc(&self) -> bool {
        self.con1 & 2 != 0
    }

    /// The tests of intentrt.c intent_blit_dispatch, in its order.
    fn classify(&self) -> BlitKind {
        let (uses, lf, desc) = (self.uses(), self.lf(), self.desc());
        if self.con1 & 1 != 0 {
            BlitKind::Line
        } else if self.con1 & 0x18 != 0 {
            BlitKind::Fill
        } else if uses & 1 == 0 {
            BlitKind::NoDest
        } else if uses == 1 && matches!(lf, 0x00 | 0xff | 0xf0) {
            BlitKind::Clear
        } else if (uses == 9 && lf == 0xf0 && self.ash() == 0
            || uses == 5 && lf == 0xcc && self.bsh() == 0)
            && !desc
        {
            BlitKind::Copy
        } else if uses == 9 && lf == 0xf0 {
            BlitKind::Shift
        } else if uses & 3 == 3 && lf == 0xca && !desc {
            BlitKind::DrawBob
        } else {
            BlitKind::General
        }
    }

    /// intentrt.c op_blit_clear.
    fn clear(&self, chip: &mut ChipMut, fill: u16) {
        let stride = self.w as i32 * 2 + self.modulo[D] as i32;
        let mut d = self.ptr[D];
        for _ in 0..self.h {
            let mut p = d;
            for _ in 0..self.w {
                chip.w16(p, fill);
                p = p.wrapping_add(2);
            }
            d = add(d, stride) & chip.wmask;
        }
    }

    /// intentrt.c op_blit_copy, from channel `src` (A with its masks, or B with none).
    fn copy(&self, chip: &mut ChipMut, src: usize, fwm: u16, lwm: u16) {
        let (mut s, mut d) = (self.ptr[src], self.ptr[D]);
        for _ in 0..self.h {
            for x in 0..self.w {
                let v = chip.r16(s) & edge(x, self.w, fwm, lwm);
                chip.w16(d, v);
                s = s.wrapping_add(2);
                d = d.wrapping_add(2);
            }
            s = add(s, self.modulo[src] as i32) & chip.wmask;
            d = add(d, self.modulo[D] as i32) & chip.wmask;
        }
    }

    /// intentrt.c op_blit_shift (with `orc` 0, as the dispatch calls it): D = A through the shifter, the previous word
    /// carried across rows.
    fn shift(&self, chip: &mut ChipMut) {
        let (ash, desc) = (self.ash(), self.desc());
        let (mut a, mut d) = (self.ptr[A], self.ptr[D]);
        let mut prev = 0u16;
        for _ in 0..self.h {
            for x in 0..self.w {
                let cur = chip.r16(a) & edge(x, self.w, self.fwm, self.lwm);
                let out = if !desc {
                    ((((prev as u32) << 16) | cur as u32) >> ash) as u16
                } else if ash == 0 {
                    cur
                } else {
                    ((((cur as u32) << 16) | prev as u32) >> (16 - ash)) as u16
                };
                prev = cur;
                chip.w16(d, out);
                let step = if desc { -2 } else { 2 };
                a = add(a, step);
                d = add(d, step);
            }
            let sign = if desc { -1 } else { 1 };
            a = add(a, sign * self.modulo[A] as i32) & chip.wmask;
            d = add(d, sign * self.modulo[D] as i32) & chip.wmask;
        }
    }

    /// intentrt.c op_blit_cookie: D = (A & B) | (~A & C) with A (the mask) and B (the image) shifted; a channel that is
    /// off reads its data register.
    fn draw_bob(&self, chip: &mut ChipMut) {
        let (ash, bsh) = (self.ash(), self.bsh());
        let (usea, useb) = (self.uses() & 8 != 0, self.uses() & 4 != 0);
        let mut p = self.ptr;
        let (mut pa, mut pb) = (0u16, 0u16);
        for _ in 0..self.h {
            for x in 0..self.w {
                let m = edge(x, self.w, self.fwm, self.lwm);
                let ca = (if usea { chip.r16(p[A]) } else { self.dat[A] }) & m;
                let cb = if useb { chip.r16(p[B]) } else { self.dat[B] };
                let cc = chip.r16(p[C]);
                let sa = ((((pa as u32) << 16) | ca as u32) >> ash) as u16;
                let sb = ((((pb as u32) << 16) | cb as u32) >> bsh) as u16;
                pa = ca;
                pb = cb;
                chip.w16(p[D], (sa & sb) | (!sa & cc));
                for (ch, on) in [(A, usea), (B, useb), (C, true), (D, true)] {
                    if on {
                        p[ch] = p[ch].wrapping_add(2);
                    }
                }
            }
            for (ch, on) in [(A, usea), (B, useb), (C, true), (D, true)] {
                if on {
                    p[ch] = add(p[ch], self.modulo[ch] as i32) & chip.wmask;
                }
            }
        }
    }

    /// intentrt.c op_blit_generic: the general data path (minterm, shifter, masks, area fill). Returns the pointers
    /// where the blit leaves them.
    fn general(&self, chip: &mut ChipMut) -> [u32; 4] {
        let (ash, bsh, lf, desc) = (self.ash(), self.bsh(), self.lf(), self.desc());
        let used = [8, 4, 2, 1].map(|bit| self.uses() & bit != 0);
        let fill = self.con1 & 0x18 != 0;
        let (ife, fci) = (self.con1 & 8 != 0, self.con1 & 4 != 0);
        let (step, sign) = if desc { (-2, -1) } else { (2, 1) };
        let mut p = self.ptr;
        let (mut pa, mut pb) = (0u16, 0u16);
        for _ in 0..self.h {
            let mut fc = fci;
            for x in 0..self.w {
                let m = edge(x, self.w, self.fwm, self.lwm);
                let ca = (if used[A] { chip.r16(p[A]) } else { self.dat[A] }) & m;
                let cb = if used[B] { chip.r16(p[B]) } else { self.dat[B] };
                let cc = if used[C] { chip.r16(p[C]) } else { self.dat[C] };
                let (sa, sb) = if !desc {
                    (
                        ((((pa as u32) << 16) | ca as u32) >> ash) as u16,
                        ((((pb as u32) << 16) | cb as u32) >> bsh) as u16,
                    )
                } else {
                    (
                        ((((ca as u32) << 16) | pa as u32) >> (16 - ash)) as u16,
                        ((((cb as u32) << 16) | pb as u32) >> (16 - bsh)) as u16,
                    )
                };
                pa = ca;
                pb = cb;
                let mut out = minterm(lf, sa, sb, cc);
                if fill {
                    let input = out;
                    for i in 0..16 {
                        let bit = 1u16 << i;
                        if fc {
                            out = if ife { out | bit } else { out ^ bit };
                        }
                        if input & bit != 0 {
                            fc = !fc;
                        }
                    }
                }
                if used[D] {
                    chip.w16(p[D], out);
                }
                for ch in [A, B, C, D] {
                    if used[ch] {
                        p[ch] = add(p[ch], step) & chip.wmask;
                    }
                }
            }
            for ch in [A, B, C, D] {
                if used[ch] {
                    p[ch] = add(p[ch], sign * self.modulo[ch] as i32) & chip.wmask;
                }
            }
        }
        p
    }
}

impl Blitter {
    /// The value last written to blitter register `reg` ($040-$074), or 0 for any other register.
    pub fn reg(&self, reg: u16) -> u16 {
        match reg & 0x1fe {
            r @ FIRST..=LAST => self.regs[((r - FIRST) / 2) as usize],
            _ => 0,
        }
    }

    fn set(&mut self, reg: u16, v: u16) {
        self.regs[((reg - FIRST) / 2) as usize] = v;
    }

    fn ptr(&self, hi: u16, wmask: u32) -> u32 {
        (((self.reg(hi) as u32) << 16) | self.reg(hi + 2) as u32) & wmask
    }

    fn set_ptr(&mut self, hi: u16, p: u32) {
        self.set(hi, (p >> 16) as u16);
        self.set(hi + 2, p as u16);
    }

    /// A write to a blitter register. A write to BLTSIZE ($058) or ECS BLTSIZH ($05E) does the blit at once and returns
    /// what it was; the caller then raises the blit-done interrupt (INTREQ bit 6), as intentrt.c write_custom does.
    /// Writes to registers outside $040-$074 are ignored (they are not the blitter's).
    pub fn write(&mut self, chip: &mut ChipMut, reg: u16, v: u16) -> Option<BlitKind> {
        let reg = reg & 0x1fe;
        if !(FIRST..=LAST).contains(&reg) {
            return None;
        }
        self.set(reg, v);
        let (h, w) = match reg {
            0x058 => ((v >> 6) & 0x3ff, v & 0x3f),
            0x05e => (self.reg(0x05c) & 0x7fff, v & 0x7ff),
            _ => return None,
        };
        let (h0, w0) = if reg == 0x058 {
            (1024, 64)
        } else {
            (32768, 2048)
        };
        Some(self.blit(
            chip,
            if h != 0 { h } else { h0 },
            if w != 0 { w } else { w0 },
        ))
    }

    /// intentrt.c intent_blit_dispatch: classify one blit of `h` rows by `w` words and do it.
    pub fn blit(&mut self, chip: &mut ChipMut, h: u16, w: u16) -> BlitKind {
        let s = Setup {
            con0: self.reg(0x040),
            con1: self.reg(0x042),
            fwm: self.reg(0x044),
            lwm: self.reg(0x046),
            ptr: [0x050, 0x04c, 0x048, 0x054].map(|r| self.ptr(r, chip.wmask)),
            modulo: [0x064, 0x062, 0x060, 0x066].map(|r| self.reg(r) as i16),
            dat: [0x074, 0x072, 0x070].map(|r| self.reg(r)),
            h,
            w,
        };
        let kind = s.classify();
        match kind {
            BlitKind::Line => self.line(chip, h),
            BlitKind::NoDest => {}
            BlitKind::Clear => {
                let fill = match s.lf() {
                    0x00 => 0,
                    0xff => 0xffff,
                    _ => s.dat[A],
                };
                s.clear(chip, fill);
            }
            BlitKind::Copy if s.uses() == 9 => s.copy(chip, A, s.fwm, s.lwm),
            BlitKind::Copy => s.copy(chip, B, 0xffff, 0xffff),
            BlitKind::Shift => s.shift(chip),
            BlitKind::DrawBob => s.draw_bob(chip),
            BlitKind::General | BlitKind::Fill => {
                let p = s.general(chip);
                for (hi, ch) in [(0x050, A), (0x04c, B), (0x048, C), (0x054, D)] {
                    self.set_ptr(hi, p[ch]);
                }
            }
        }
        kind
    }

    /// intentrt.c op_blit_line (G2E6 lineblit, Bresenham per the Hardware Reference Manual): a line `len` dots long.
    /// BLTAPTL holds the error term, BLTBMOD and BLTAMOD the two increments, BLTCMOD the row stride.
    fn line(&mut self, chip: &mut ChipMut, len: u16) {
        let (con0, con1) = (self.reg(0x040), self.reg(0x042));
        let mt = con0 & 0xff;
        let mut ash = (con0 >> 12) as i32;
        let (oct, sing) = ((con1 >> 2) & 7, con1 & 2 != 0);
        let mut err = self.reg(0x052) as i16;
        let (inc1, inc2) = (self.reg(0x062) as i16, self.reg(0x064) as i16);
        let stride = self.reg(0x060) as i16 as i32;
        let mut pc = self.ptr(0x048, chip.wmask);
        let mut pd = self.ptr(0x054, chip.wmask);
        let (adat, bdat) = (self.reg(0x074), self.reg(0x072));
        let bsh = (con1 >> 12) as u32;
        let mut pat = (((bdat as u32) << bsh) | ((bdat as u32) >> ((16 - bsh) & 15))) as u16;
        let (sud, sul, aul) = (oct & 4 != 0, oct & 2 != 0, oct & 1 != 0);
        let mut dotdone = false;
        for _ in 0..len {
            let c = chip.r16(pc);
            let a = adat >> ash;
            let b = if pat & 0x8000 != 0 { 0xffff } else { 0 };
            if !(sing && dotdone) {
                chip.w16(pd, minterm(mt, a, b, c));
            }
            dotdone = true;
            pd = pc;
            let back = |neg: bool| if neg { -1 } else { 1 };
            let (xstep, ystep) = match (err >= 0, sud) {
                (true, true) => (back(aul), back(sul)),
                (true, false) => (back(sul), back(aul)),
                (false, true) => (0, back(sul)),
                (false, false) => (back(sul), 0),
            };
            err = err.wrapping_add(if err >= 0 { inc2 } else { inc1 });
            if xstep != 0 {
                ash += xstep;
                if ash > 15 {
                    ash = 0;
                    pc = pc.wrapping_add(2);
                } else if ash < 0 {
                    ash = 15;
                    pc = pc.wrapping_sub(2);
                }
            }
            if ystep != 0 {
                pc = add(pc, ystep * stride);
                dotdone = false;
            }
            pat = pat.rotate_left(1);
        }
        self.set_ptr(0x048, pc);
        self.set_ptr(0x054, pc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minterm_matches_the_lf_table() {
        // LF $CA is "A ? B : C", $F0 is A, $CC is B, $AA is C
        assert_eq!(minterm(0xca, 0xff00, 0x0f0f, 0x3333), 0x0f33);
        assert_eq!(minterm(0xf0, 0x1234, 0, 0xffff), 0x1234);
        assert_eq!(minterm(0xcc, 0, 0x5678, 0), 0x5678);
        assert_eq!(minterm(0xaa, 0, 0, 0x9abc), 0x9abc);
    }

    #[test]
    fn bltsize_clears_and_wraps_at_chip_size() {
        let mut ram = vec![0x55u8; 64];
        let mut chip = ChipMut::new(&mut ram);
        let mut b = Blitter::default();
        // D only, LF $FF, two rows of two words from $3C (the second row wraps to 0)
        for (r, v) in [(0x040, 0x01ff), (0x054, 0), (0x056, 0x3c), (0x066, 0)] {
            assert_eq!(b.write(&mut chip, r, v), None);
        }
        assert_eq!(
            b.write(&mut chip, 0x058, (2 << 6) | 2),
            Some(BlitKind::Clear)
        );
        assert_eq!(&ram[0x3c..], &[0xff; 4]);
        assert_eq!(&ram[..4], &[0xff; 4]);
        assert_eq!(ram[4], 0x55);
    }
}
