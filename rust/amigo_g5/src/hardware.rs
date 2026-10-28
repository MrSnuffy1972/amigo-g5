//! The custom chips as one runtime: the register shadow the game writes, the interrupt request and enable bits, and
//! one PAL line of time at a call. It owns the display, blitter, Paula and the CIAs and does what intentrt.c's
//! `write_custom`, `read_custom`, `tick_line_step` and `chip_ipl` do with them. Still no beam and no copper
//! execution: the copper list is the display's table, read once a tick, and the picture is composed line by line as
//! the tick's lines pass.
//!
//! Ported from `kit/amigo/port/runtime/intentrt.c` (M5 part 5, 28 Oct 2026), which is the oracle:
//! `tests/hardware_diff.rs` compiles the whole of intentrt.c with gcc and compares every read, INTREQ and the level
//! after every line, each tick's frame, every audio sample and chip RAM. Chip RAM is passed in by the caller (the
//! guest memory owns it); its size must be a power of two, as the C's CHIP_SIZE.

use crate::blit::{BlitKind, Blitter, ChipMut};
use crate::cia::{self, Buttons, Cias};
use crate::display::{Chip, CpuWrites, Display, Frame, Progress, Shadow, SCR_H, SCR_W};
use crate::paula::Paula;

/// CPU clock cycles in one PAL line, and lines in one tick (amiga.h CYC_PER_LINE, LINES_PER_FRAME).
pub const CYC_PER_LINE: u64 = 454;
pub const LINES_PER_FRAME: i32 = 313;

/// The player's controls as the chips report them (the C's front-end globals joy_*, mouse_joy0dat, mouse_lmb,
/// amigo_potgor_clear).
#[derive(Debug, Clone, Copy, Default)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    /// Port 2 fire (CIA-A PRA bit 7) and the second button (POTGOR bit 14).
    pub fire: bool,
    pub fire2: bool,
    /// Port 1: the mouse counters (JOY0DAT), the left button (CIA-A PRA bit 6), POTGOR bits pulled low.
    pub mouse_joy0dat: u16,
    pub lmb: bool,
    pub potgor_clear: u16,
}

/// The custom chips. No global state: the runtime owns one of these.
pub struct Hardware {
    custom: [u16; 256],
    pub intena: u16,
    pub intreq: u16,
    pub dmacon: u16,
    line: i32,
    /// The CPU clock at the start of the current line (intentrt.c frame_base_cyc), for beam position reads.
    line_cyc: u64,
    cop1lc: u32,
    cop2lc: u32,
    bplpt: [u32; 6],
    palette: [u16; 32],
    display: Display,
    frame: Frame,
    progress: Progress,
    cpuw: CpuWrites,
    blitter: Blitter,
    pub paula: Paula,
    pub cias: Cias,
    pub input: Input,
    /// Set when a tick ends (the frame in `framebuf` is complete); the caller clears it.
    pub frame_done: bool,
    pub frame_count: u64,
    /// The composed picture, SCR_W x SCR_H, 0xAARRGGBB.
    pub framebuf: Vec<u32>,
    /// 44.1 kHz stereo samples (left, right) since the caller last drained it.
    pub audio: Vec<i16>,
    /// Blits done, by `BlitKind` (intentrt.c blit_class_counts).
    pub blits: [u64; 8],
}

impl Hardware {
    /// The chips as intentrt.c `chip_reset` leaves them, with the CPU clock at `now` and the given Paula (its output
    /// model and start deferral are the C's environment switches).
    pub fn new(chip: &[u8], now: u64, paula: Paula) -> Self {
        let display = Display::default();
        let custom = [0u16; 256];
        let frame = display.frame_begin(
            Chip::new(chip),
            &Shadow {
                custom,
                palette: [0; 32],
                bplpt: [0; 6],
                dmacon: 0,
            },
            0,
        );
        Hardware {
            custom,
            intena: 0,
            intreq: 0,
            dmacon: 0,
            line: 0,
            line_cyc: now,
            cop1lc: 0,
            cop2lc: 0,
            bplpt: [0; 6],
            palette: [0; 32],
            display,
            progress: Progress::begin(&frame),
            frame,
            cpuw: CpuWrites::default(),
            blitter: Blitter::default(),
            paula,
            cias: Cias::new(),
            input: Input::default(),
            frame_done: false,
            frame_count: 0,
            framebuf: vec![0; SCR_W * SCR_H],
            audio: Vec::new(),
            blits: [0; 8],
        }
    }

    /// The line within the tick (0 to 312).
    pub fn line(&self) -> i32 {
        self.line
    }

    /// The value last written to custom register `reg` ($000-$1FE).
    pub fn shadow(&self, reg: u16) -> u16 {
        self.custom[((reg & 0x1fe) >> 1) as usize]
    }

    /// A game write to custom register `reg` (intentrt.c write_custom). A blit is done at once; DMACON starts and
    /// stops Paula's channels; a write to the copper's pointers mid-tick re-reads the copper table.
    pub fn write(&mut self, chip: &mut [u8], reg: u16, v: u16) {
        let reg = reg & 0x1fe;
        // the game's own writes are stamped with the line they happen at
        self.frame
            .copper_cpu(Chip::new(chip), &mut self.progress, self.line, reg, v);
        self.cpuw.push(self.line, reg, v);
        self.custom[(reg >> 1) as usize] = v;
        let (hi, lo) = ((v as u32) << 16, (v & 0xfffe) as u32);
        match reg {
            0x080 => self.cop1lc = (self.cop1lc & 0xffff) | hi,
            0x082 => self.cop1lc = (self.cop1lc & 0xffff_0000) | lo,
            0x084 => self.cop2lc = (self.cop2lc & 0xffff) | hi,
            0x086 => self.cop2lc = (self.cop2lc & 0xffff_0000) | lo,
            0x040..=0x074 => {
                if let Some(kind) = self.blitter.write(&mut ChipMut::new(chip), reg, v) {
                    self.blit_done(kind);
                }
            }
            0x096 => {
                let old = self.dmacon;
                set_clr(&mut self.dmacon, v);
                self.paula.dma_changed(old, self.dmacon);
            }
            0x09a => set_clr(&mut self.intena, v),
            0x09c => set_clr(&mut self.intreq, v),
            0x0e0..=0x0f6 => {
                let p = ((reg - 0xe0) >> 2) as usize;
                self.bplpt[p] = if reg & 2 != 0 {
                    (self.bplpt[p] & 0xffff_0000) | lo
                } else {
                    (self.bplpt[p] & 0xffff) | hi
                };
            }
            0x180..=0x1be => self.palette[((reg - 0x180) >> 1) as usize] = v & 0xfff,
            0x0a0..=0x0de => self.intreq |= self.paula.write(reg, v, self.dmacon),
            _ => {}
        }
    }

    /// After a blit: the pointers it left are in the shadow too (the C's blitter works on its shadow), and the
    /// blit-done interrupt is raised.
    fn blit_done(&mut self, kind: BlitKind) {
        for r in (0x040..=0x074u16).step_by(2) {
            self.custom[(r >> 1) as usize] = self.blitter.reg(r);
        }
        self.blits[kind as usize] += 1;
        self.intreq |= 0x40;
    }

    /// The line the CPU has reached at clock `now`, and the position in it (intentrt.c beam_line, beam_h: the
    /// current line plus the whole lines of the running slice).
    fn beam(&self, now: u64) -> (i32, u16) {
        let d = now.saturating_sub(self.line_cyc);
        let l = (self.line as u64 + d / CYC_PER_LINE) % LINES_PER_FRAME as u64;
        let h = ((d % CYC_PER_LINE) / 2).min(0xe2);
        (l as i32, h as u16)
    }

    /// A game read of custom register `reg` at CPU clock `now` (intentrt.c read_custom). The blitter is never busy:
    /// blits are done when they are started.
    pub fn read(&self, reg: u16, now: u64) -> u16 {
        let i = &self.input;
        match reg & 0x1fe {
            0x002 => self.dmacon & 0x3ff,
            0x004 => 0x8000 | ((self.beam(now).0 >> 8) & 1) as u16,
            0x006 => {
                let (l, h) = self.beam(now);
                ((l as u16 & 0xff) << 8) | h
            }
            0x00a => i.mouse_joy0dat,
            0x00c => {
                let (r, l, u, d) = (i.right as u16, i.left as u16, i.up as u16, i.down as u16);
                (r << 1) | (l << 9) | (r ^ d) | ((l ^ u) << 8)
            }
            0x010 => self.custom[0x9e >> 1],
            // POTGOR: bit 14 is the second fire button
            0x016 => 0xff00 & !(if i.fire2 { 0x4000 } else { 0 }) & !i.potgor_clear,
            0x018 => 0x3000,
            0x01c => self.intena,
            0x01e => self.intreq,
            _ => 0,
        }
    }

    /// A game read of register `reg` of CIA `n` (0 = CIA-A, 1 = CIA-B), with the fire buttons from `input`.
    pub fn cia_read(&mut self, n: usize, reg: u8) -> u8 {
        let b = Buttons {
            fire: self.input.fire,
            lmb: self.input.lmb,
        };
        self.cias.read(n, reg, b)
    }

    pub fn cia_write(&mut self, n: usize, reg: u8, v: u8) {
        self.cias.write(n, reg, v);
    }

    /// The custom-chip writes the fake OS made while booting (`Os::chip_log`), in order.
    pub fn apply_os_log(&mut self, chip: &mut [u8], log: &[(u32, u16)]) {
        for &(addr, v) in log {
            if addr & 0xff_f000 == 0xdf_f000 {
                self.write(chip, (addr & 0x1fe) as u16, v);
            }
        }
    }

    /// One PAL line of time (intentrt.c tick_line_step): Paula plays the line, the CIAs count it, the picture is
    /// composed down to the line just passed and the copper table's interrupts for it are raised. At the end of the
    /// tick the frame is finished, the next tick's copper table is read from COP1LC (or where this tick's list moved
    /// it), and VERTB is raised.
    pub fn pass_line(&mut self, chip: &[u8]) {
        let c = Chip::new(chip);
        let led = self.cias.led_filter_on();
        self.intreq |= self.paula.play_line(c, led, &mut self.audio);
        self.cias.pass_line();
        self.line_cyc += CYC_PER_LINE;
        self.line += 1;
        self.compose_to(c, self.line);
        self.raise_callbacks();
        // at the end of the tick the call above has composed the last lines (the C's second compose_to 312 does nothing)
        if self.line >= LINES_PER_FRAME {
            self.line = 0;
            self.frame_done = true;
            self.frame_count += 1;
            let shadow = Shadow {
                custom: self.custom,
                palette: self.palette,
                bplpt: self.bplpt,
                dmacon: self.dmacon,
            };
            self.frame = self.display.frame_begin(c, &shadow, self.cop1lc);
            self.cpuw.clear();
            // G2E9 coplc: a copper MOVE to COP1LC/COP2LC in the list sets where the next tick's copper starts, when it
            // leads to a list that ends and is not zero memory
            let t = &self.frame.table;
            let w = c.wmask();
            if t.cop1_written && crate::display::coplc_plausible(c, t.cop1 & w) {
                self.cop1lc = t.cop1 & w;
            }
            if t.cop2_written && crate::display::coplc_plausible(c, t.cop2 & w) {
                self.cop2lc = t.cop2 & w;
            }
            self.progress = Progress::begin(&self.frame);
            self.intreq |= 0x0020; // VERTB
                                   // a copper INTREQ move before the list's first WAIT fires with VERTB
            self.raise_callbacks();
            self.cias.frame_end();
        }
        self.intreq |= self.cias.intreq_bits();
    }

    fn compose_to(&mut self, c: Chip, lend: i32) {
        let (f, w, p) = (&self.frame, &self.cpuw, &mut self.progress);
        self.display
            .compose_to(c, f, w, p, lend, &mut self.framebuf);
    }

    /// The copper list's line-timed interrupts for the current line, from the table.
    fn raise_callbacks(&mut self) {
        for &(l, bits) in &self.frame.table.callbacks {
            if l == self.line {
                self.intreq |= bits;
            }
        }
    }

    /// The interrupt level the 68000 sees (intentrt.c chip_ipl).
    pub fn ipl(&mut self) -> u8 {
        self.intreq |= self.cias.intreq_bits();
        cia::ipl(self.intena, self.intreq)
    }
}

fn set_clr(x: &mut u16, v: u16) {
    if v & 0x8000 != 0 {
        *x |= v & 0x7fff;
    } else {
        *x &= !v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_log_and_a_tick() {
        let mut chip = vec![0u8; 4096];
        // a copper list at $100 that raises SOFT (bit 2) at line $40
        for (i, w) in [0x4001u16, 0xfffe, 0x009c, 0x8004, 0xffff, 0xfffe]
            .iter()
            .enumerate()
        {
            chip[0x100 + i * 2..0x102 + i * 2].copy_from_slice(&w.to_be_bytes());
        }
        // the reset table is read from $0: end it there, or its zero words (MOVE $000) would run on into the list
        chip[..4].copy_from_slice(&[0xff, 0xff, 0xff, 0xfe]);
        let mut cs = Hardware::new(&chip, 0, Paula::default());
        // a CIA address in the log is not a custom register and is skipped
        let log = [
            (0xdff080, 0),
            (0xdff082, 0x100),
            (0xbfe001, 0x1234),
            (0xdff09a, 0xc024),
        ];
        cs.apply_os_log(&mut chip, &log);
        assert_eq!((cs.shadow(0x082), cs.intena), (0x100, 0x4024));
        // the list is read at the end of the first tick; VERTB (level 3) then, SOFT (level 1) at line $40 after it
        for _ in 0..LINES_PER_FRAME {
            cs.pass_line(&chip);
        }
        assert!(cs.frame_done);
        assert_eq!((cs.intreq, cs.ipl()), (0x0020, 3));
        cs.write(&mut chip, 0x09c, 0x0020);
        for _ in 0..0x40 {
            cs.pass_line(&chip);
        }
        assert_eq!((cs.intreq, cs.ipl()), (0x0004, 1));
        // the beam: line $40, half way along it
        assert_eq!(
            cs.read(0x006, (LINES_PER_FRAME as u64 + 0x40) * CYC_PER_LINE + 200),
            0x4064
        );
    }
}
