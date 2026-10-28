//! The two CIAs as the game uses them: interval timers that raise interrupts, the time-of-day counters, the
//! interrupt control registers, the fire buttons on CIA-A's port A and the keyboard's serial link. No cycle model:
//! the timers advance once per PAL line by the E clock ticks that line holds, as the C runtime does.
//!
//! Ported from `kit/amigo/port/runtime/intentrt.c` (the "CIA" section, the CIA part of `tick_line_step`,
//! `update_cia_intreq`, `chip_ipl` and the CIA lines of `chip_reset`; M5 part 4, 28 Oct 2026), which is the oracle:
//! `tests/cia_diff.rs` links the whole of intentrt.c and compares every CIA read, INTREQ and the interrupt level
//! after every line.

use std::collections::VecDeque;

/// E clock ticks per PAL line, as the C accumulates them (`CYC_PER_LINE / 10.0`).
const E_PER_LINE: f64 = 454.0 / 10.0;
/// Lines between two keyboard bytes (intentrt.c KBD_GAP_LINES).
const KBD_GAP_LINES: u32 = 8;
/// Lines a keyboard byte waits for the game's handshake before the link gives up (KBD_TIMEOUT_LINES, 8 x 143 ms).
const KBD_TIMEOUT_LINES: u32 = 17872;
/// Keys the link holds before it drops new ones (the C's 64-entry ring keeps one slot empty).
const KBD_QUEUE: usize = 63;

/// Interrupt control register bits (ICR and its mask).
const ICR_TA: u8 = 1;
const ICR_TB: u8 = 2;
const ICR_ALARM: u8 = 4;
const ICR_SP: u8 = 8;

/// The fire buttons the game reads on CIA-A port A, set by the front end.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Buttons {
    /// Port 2 fire (the joystick), bit 7.
    pub fire: bool,
    /// Port 1 fire (the left mouse button), bit 6.
    pub lmb: bool,
}

/// One CIA (intentrt.c `cia_t`).
#[derive(Debug, Clone, Default)]
struct Cia {
    pra: u8,
    prb: u8,
    ddra: u8,
    ddrb: u8,
    sdr: u8,
    icr: u8,
    icrmask: u8,
    cra: u8,
    crb: u8,
    ta: u16,
    tb: u16,
    ta_latch: u16,
    tb_latch: u16,
    tod: u32,
    tod_latch: u32,
    tod_latched: bool,
    tod_stopped: bool,
    tod_alarm: u32,
    eacc: f64,
}

/// Counts one timer down by `ticks`; returns the new count and whether it underflowed (intentrt.c `cia_tick`).
/// `cr` is the control register: bit 0 start, bit 3 one-shot (which stops the timer and reloads the latch).
fn count_down(count: u16, latch: u16, cr: &mut u8, ticks: i32) -> (u16, bool) {
    let mut t = count as i32 - ticks;
    let mut fired = false;
    while t < 0 {
        fired = true;
        if *cr & 8 != 0 {
            *cr &= !1;
            t = latch as i32;
            break;
        }
        t += latch as i32 + 1;
        if latch == 0 {
            t = 0;
            break;
        }
    }
    (t as u16, fired)
}

impl Cia {
    /// The reset state (intentrt.c `chip_reset`): timers and latches all ones, everything else zero.
    fn reset() -> Self {
        Cia {
            ta: 0xffff,
            tb: 0xffff,
            ta_latch: 0xffff,
            tb_latch: 0xffff,
            ..Default::default()
        }
    }

    /// One line's E clock ticks on both timers. Timer B counting timer A underflows (CRB bit 6) is not counted,
    /// as in the C.
    fn tick(&mut self, ticks: i32) {
        if self.cra & 1 != 0 {
            let (t, fired) = count_down(self.ta, self.ta_latch, &mut self.cra, ticks);
            self.ta = t;
            if fired {
                self.icr |= ICR_TA;
            }
        }
        if self.crb & 1 != 0 && self.crb & 0x40 == 0 {
            let (t, fired) = count_down(self.tb, self.tb_latch, &mut self.crb, ticks);
            self.tb = t;
            if fired {
                self.icr |= ICR_TB;
            }
        }
    }

    /// One count of the 24-bit time of day, with its alarm (intentrt.c `cia_tod`).
    fn tod_step(&mut self) {
        if self.tod_stopped {
            return;
        }
        self.tod = (self.tod + 1) & 0xff_ffff;
        if self.tod == self.tod_alarm {
            self.icr |= ICR_ALARM;
        }
    }

    /// The E clock ticks this line holds, carrying the fraction to the next line.
    fn e_ticks(&mut self) -> i32 {
        self.eacc += E_PER_LINE;
        let t = self.eacc as i32;
        self.eacc -= t as f64;
        t
    }
}

/// The keyboard's serial link into CIA-A (intentrt.c kbfix, 17 Oct 2026): a byte stays in SDR until the game's
/// handshake (CRA SPMODE high then low) or the timeout, then a gap of a few lines before the next.
#[derive(Debug, Clone, Default)]
struct Keyboard {
    queue: VecDeque<u8>,
    wait: u32,
    busy: bool,
    handshake: bool,
    lines: u32,
}

/// Both CIAs and the keyboard link. No global state: the runtime owns one of these.
#[derive(Debug, Clone)]
pub struct Cias {
    cia: [Cia; 2],
    kbd: Keyboard,
}

impl Default for Cias {
    fn default() -> Self {
        Self::new()
    }
}

impl Cias {
    /// Both CIAs as `chip_reset` leaves them: CIA-A's serial port interrupt enabled, as the OS keyboard driver
    /// leaves it.
    pub fn new() -> Self {
        let mut a = Cia::reset();
        a.icrmask = ICR_SP;
        Cias {
            cia: [a, Cia::reset()],
            kbd: Keyboard::default(),
        }
    }

    /// Queues a key press or release for the game (intentrt.c `kbd_push`); dropped when the link is full.
    pub fn push_key(&mut self, rawkey: u8, down: bool) {
        if self.kbd.queue.len() < KBD_QUEUE {
            self.kbd
                .queue
                .push_back((rawkey & 0x7f) | if down { 0 } else { 0x80 });
        }
    }

    /// A read of register `reg` (0-15) of CIA `n` (0 = CIA-A at $BFE001, 1 = CIA-B at $BFD000), with its side
    /// effects: an ICR read clears the pending bits, the TOD reads latch and release the count (intentrt.c
    /// `cia_read_raw`).
    pub fn read(&mut self, n: usize, reg: u8, buttons: Buttons) -> u8 {
        let c = &mut self.cia[n];
        match reg & 15 {
            0 => {
                // the disk lines read high and never change; a pressed button pulls its bit low
                let mut v = 0xffu8;
                if n == 0 && buttons.fire {
                    v &= !0x80;
                }
                if n == 0 && buttons.lmb {
                    v &= !0x40;
                }
                (v & !c.ddra) | (c.pra & c.ddra)
            }
            1 => !c.ddrb | (c.prb & c.ddrb),
            2 => c.ddra,
            3 => c.ddrb,
            4 => c.ta as u8,
            5 => (c.ta >> 8) as u8,
            6 => c.tb as u8,
            7 => (c.tb >> 8) as u8,
            8 => {
                let t = if c.tod_latched { c.tod_latch } else { c.tod };
                c.tod_latched = false;
                t as u8
            }
            9 => ((if c.tod_latched { c.tod_latch } else { c.tod }) >> 8) as u8,
            10 => {
                c.tod_latch = c.tod;
                c.tod_latched = true;
                (c.tod >> 16) as u8
            }
            12 => c.sdr,
            13 => {
                let mut v = c.icr;
                if v & c.icrmask != 0 {
                    v |= 0x80;
                }
                c.icr = 0;
                v
            }
            14 => c.cra,
            15 => c.crb,
            _ => 0xff,
        }
    }

    /// A write of `v` to register `reg` of CIA `n` (intentrt.c `cia_write`).
    pub fn write(&mut self, n: usize, reg: u8, v: u8) {
        let c = &mut self.cia[n];
        match reg & 15 {
            0 => c.pra = v,
            1 => c.prb = v,
            2 => c.ddra = v,
            3 => c.ddrb = v,
            4 => c.ta_latch = (c.ta_latch & 0xff00) | v as u16,
            5 => {
                c.ta_latch = (c.ta_latch & 0xff) | (v as u16) << 8;
                if c.cra & 1 == 0 {
                    c.ta = c.ta_latch;
                }
                // a one-shot timer starts on the high byte
                if c.cra & 8 != 0 {
                    c.ta = c.ta_latch;
                    c.cra |= 1;
                }
            }
            6 => c.tb_latch = (c.tb_latch & 0xff00) | v as u16,
            7 => {
                c.tb_latch = (c.tb_latch & 0xff) | (v as u16) << 8;
                if c.crb & 1 == 0 {
                    c.tb = c.tb_latch;
                }
                if c.crb & 8 != 0 {
                    c.tb = c.tb_latch;
                    c.crb |= 1;
                }
            }
            // CRB bit 7 points the TOD writes at the alarm; a write of the high byte stops the count until the low
            // byte is written
            8 => {
                if c.crb & 0x80 != 0 {
                    c.tod_alarm = (c.tod_alarm & 0xff_ff00) | v as u32;
                } else {
                    c.tod = (c.tod & 0xff_ff00) | v as u32;
                    c.tod_stopped = false;
                }
            }
            9 => {
                if c.crb & 0x80 != 0 {
                    c.tod_alarm = (c.tod_alarm & 0xff_00ff) | (v as u32) << 8;
                } else {
                    c.tod = (c.tod & 0xff_00ff) | (v as u32) << 8;
                }
            }
            10 => {
                if c.crb & 0x80 != 0 {
                    c.tod_alarm = (c.tod_alarm & 0x00_ffff) | (v as u32) << 16;
                } else {
                    c.tod = (c.tod & 0x00_ffff) | (v as u32) << 16;
                    c.tod_stopped = true;
                }
            }
            // serial output: the byte counts as sent at once
            12 => {
                c.sdr = v;
                if c.cra & 0x40 != 0 {
                    c.icr |= ICR_SP;
                }
            }
            13 => {
                if v & 0x80 != 0 {
                    c.icrmask |= v & 0x1f;
                } else {
                    c.icrmask &= !(v & 0x1f);
                }
            }
            14 => {
                if n == 0 && self.kbd.busy {
                    if v & 0x40 != 0 {
                        self.kbd.handshake = true;
                    } else if self.kbd.handshake {
                        self.kbd.busy = false;
                        self.kbd.wait = KBD_GAP_LINES;
                    }
                }
                // bit 4 (force load) is a strobe, never stored
                c.cra = v & !0x10;
                if v & 0x10 != 0 {
                    c.ta = c.ta_latch;
                }
            }
            15 => {
                c.crb = v & !0x10;
                if v & 0x10 != 0 {
                    c.tb = c.tb_latch;
                }
            }
            _ => {}
        }
    }

    /// What one PAL line does to the CIAs (intentrt.c `tick_line_step`): both timers count the line's E clock
    /// ticks, CIA-B's TOD counts the line, and the keyboard link moves on.
    pub fn pass_line(&mut self) {
        for c in &mut self.cia {
            let t = c.e_ticks();
            c.tick(t);
        }
        self.cia[1].tod_step();
        self.keyboard_step();
    }

    /// The end of a tick: CIA-A's TOD counts frames (the C's vertical blank count).
    pub fn frame_end(&mut self) {
        self.cia[0].tod_step();
    }

    /// The INTREQ bits the CIAs hold raised: PORTS (bit 3) for CIA-A, EXTER (bit 13) for CIA-B (intentrt.c
    /// `update_cia_intreq`). The caller ORs them into INTREQ; they stay there until the game clears them.
    pub fn intreq_bits(&self) -> u16 {
        let mut bits = 0;
        if self.cia[0].icr & self.cia[0].icrmask & 0x1f != 0 {
            bits |= 0x0008;
        }
        if self.cia[1].icr & self.cia[1].icrmask & 0x1f != 0 {
            bits |= 0x2000;
        }
        bits
    }

    /// True when the game has the audio LED filter in (CIA-A PRA bit 1 driven low), for `Paula::play_line`.
    pub fn led_filter_on(&self) -> bool {
        crate::paula::led_filter_on(self.cia[0].pra, self.cia[0].ddra)
    }

    /// The keyboard link, once a line (intentrt.c `kbd_service`).
    fn keyboard_step(&mut self) {
        let k = &mut self.kbd;
        if k.busy {
            k.lines += 1;
            if k.lines >= KBD_TIMEOUT_LINES {
                k.busy = false;
                k.wait = KBD_GAP_LINES;
            }
            return;
        }
        if k.wait > 0 {
            k.wait -= 1;
            return;
        }
        // nothing is received while the serial port is an output
        if self.cia[0].cra & 0x40 != 0 {
            return;
        }
        let Some(code) = k.queue.pop_front() else {
            return;
        };
        // the keyboard sends the code rotated left one bit and inverted
        self.cia[0].sdr = !code.rotate_left(1);
        self.cia[0].icr |= ICR_SP;
        k.busy = true;
        k.handshake = false;
        k.lines = 0;
    }
}

/// The interrupt level the 68000 sees for INTENA and INTREQ (intentrt.c `chip_ipl`, after the caller has ORed in
/// `Cias::intreq_bits`): 0 when the master enable (bit 14) is off or nothing enabled is pending.
pub fn ipl(intena: u16, intreq: u16) -> u8 {
    if intena & 0x4000 == 0 {
        return 0;
    }
    let p = intena & intreq & 0x3fff;
    match p {
        0 => 0,
        _ if p & 0x2000 != 0 => 6,
        _ if p & 0x1800 != 0 => 5,
        _ if p & 0x0780 != 0 => 4,
        _ if p & 0x0070 != 0 => 3,
        _ if p & 0x0008 != 0 => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuous_timer_underflows_and_reloads() {
        let mut c = Cias::new();
        c.write(1, 13, 0x81); // enable the CIA-B timer A interrupt
        c.write(1, 4, 99);
        c.write(1, 5, 0); // latch 99, loaded while stopped
        c.write(1, 14, 0x01); // start, continuous
        c.pass_line(); // 45 ticks: 54 left
        assert_eq!(c.read(1, 4, Buttons::default()), 54);
        assert_eq!(c.intreq_bits(), 0);
        c.pass_line(); // 45 more (45.4 accumulates): 9 left
        c.pass_line(); // underflow, reload to 99 + 1 - 36
        assert_eq!(c.intreq_bits(), 0x2000);
        assert_eq!(c.read(1, 13, Buttons::default()), 0x81);
        assert_eq!(c.intreq_bits(), 0, "an ICR read clears it");
    }

    #[test]
    fn one_shot_starts_on_high_byte_and_stops() {
        let mut c = Cias::new();
        c.write(0, 14, 0x08);
        c.write(0, 4, 10);
        c.write(0, 5, 0);
        assert_eq!(c.read(0, 14, Buttons::default()) & 1, 1);
        c.pass_line();
        assert_eq!(c.read(0, 14, Buttons::default()) & 1, 0);
        assert_eq!(c.read(0, 4, Buttons::default()), 10);
    }

    #[test]
    fn keyboard_byte_waits_for_handshake() {
        let mut c = Cias::new();
        c.push_key(0x45, true); // Esc down
        c.push_key(0x45, false);
        c.pass_line();
        assert_eq!(c.read(0, 12, Buttons::default()), !0x8au8);
        assert_eq!(c.intreq_bits(), 0x0008);
        assert_eq!(c.read(0, 13, Buttons::default()), 0x88);
        for _ in 0..20 {
            c.pass_line();
        }
        assert_eq!(c.intreq_bits(), 0, "no second byte before the handshake");
        c.write(0, 14, 0x40);
        c.write(0, 14, 0x00);
        for _ in 0..9 {
            c.pass_line();
        }
        assert_eq!(c.read(0, 12, Buttons::default()), !0x8bu8);
    }

    #[test]
    fn ipl_priorities() {
        assert_eq!(ipl(0x0020, 0x0020), 0);
        assert_eq!(ipl(0xc028, 0x0028), 3);
        assert_eq!(ipl(0x6008, 0x2008), 6);
        assert_eq!(ipl(0x4004, 0x0004), 1);
        assert_eq!(ipl(0x4080, 0x0080), 4);
    }
}
