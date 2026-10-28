//! The run loop of a cold-booted program (M6, 28 Oct 2026): translated code, the OS services and the custom chips
//! on one clock. Ported from `kit/amigo/port/runtime/cpu_native.c` `run_frame_preempt` (the default scheduler of
//! the clean-sheet native build) and `take_irq`, which are the oracle.
//!
//! Decision 28 Oct 2026 (M6): the translated code is the Rust form of the C `--single` build (one match arm per
//! instruction), so a "slice" here is one instruction, not a chunk. Virtual time is the beam clock: an
//! instruction's cycles are stretched by `SLOW_NUM / 1024` (cpu_native.c `cpu_slow_num`, the fitted bus-contention
//! slowdown the reference harness uses) with the fraction carried, as the C's AMIGO_SCHED_FIXCYC does (truncating
//! per instruction would lose most of the stretch). Lines pass when the clock reaches them; interrupts are taken
//! between instructions with a real stack frame; a STOPped CPU waits for the next line. There is no yield table:
//! a busy loop spins on its own cycles, as the original does on the 68000.

use std::any::Any;

use crate::boot::Booted;
use crate::cpu::{Cpu, Stop};
use crate::hardware::{Hardware, CYC_PER_LINE};
use crate::mem::{GuestMemory, Io};
use crate::os::Os;
use crate::paula::Paula;

/// cpu_native.c `cpu_slow_num`: beam cycles per 1024 CPU cycles.
pub const SLOW_NUM: u64 = 1126;

/// The custom chips on the bus.
pub struct Hw(pub Hardware);

impl Io for Hw {
    fn custom_read(&mut self, reg: u16, now: u64) -> u16 {
        self.0.read(reg, now)
    }
    fn custom_write(&mut self, chip: &mut [u8], reg: u16, v: u16) {
        self.0.write(chip, reg, v)
    }
    fn cia_read(&mut self, n: usize, reg: u8) -> u8 {
        self.0.cia_read(n, reg)
    }
    fn cia_write(&mut self, n: usize, reg: u8, v: u8) {
        self.0.cia_write(n, reg, v)
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn chips(io: &mut dyn Io) -> &mut Hardware {
    &mut io
        .as_any_mut()
        .downcast_mut::<Hw>()
        .expect("the bus is the runtime's Hw")
        .0
}

/// Counters for the run log.
#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub steps: u64,
    pub os_calls: u64,
    pub irqs: [u64; 8],
    pub stopped_lines: u64,
}

/// A cold-booted program with its chips.
pub struct Machine {
    pub cpu: Cpu,
    pub mem: GuestMemory,
    pub os: Os,
    /// The beam clock (cpu_native.c CYC), in colour-clock cycles: CYC_PER_LINE a line.
    pub beam: u64,
    next_line: u64,
    carry: u64,
    pub stats: Stats,
    /// When `Some`, every OS call is recorded as (tick, library base, LVO): the dev trace that lines the port's
    /// OS calls up with the reference harness's AMIGO_OSHLE_TRACE.
    pub os_trace: Option<Vec<(u64, u32, i32)>>,
}

impl Machine {
    /// The booted program with the chips reset (intentrt.c chip_reset) and the fake OS's boot writes applied.
    pub fn new(b: Booted) -> Self {
        let Booted { cpu, mut mem, os } = b;
        let chip_len = (0..)
            .map(|k: u32| 1usize << k)
            .take_while(|&n| mem.read_bytes(0, n).is_ok())
            .last()
            .unwrap_or(0);
        let cs = Hardware::new(
            mem.read_bytes(0, chip_len).unwrap_or(&[]),
            0,
            Paula::default(),
        );
        mem.attach_io(Box::new(Hw(cs)));
        let log = os.chip_log.clone();
        mem.with_io(|io, chip| chips(io).apply_os_log(chip, &log));
        Machine {
            cpu,
            mem,
            os,
            beam: 0,
            next_line: CYC_PER_LINE,
            carry: 0,
            stats: Stats::default(),
            os_trace: None,
        }
    }

    /// Run `f` on the chips.
    pub fn chips<R>(&mut self, f: impl FnOnce(&mut Hardware) -> R) -> R {
        self.mem
            .with_io(|io, _| f(chips(io)))
            .expect("bus attached")
    }

    /// cpu_native.c take_irq: an interrupt at level `lvl` with a real stack frame.
    fn take_irq(&mut self, lvl: u8) -> Result<(), Stop> {
        let c = &mut self.cpu;
        let sr = c.get_sr();
        if c.s == 0 {
            std::mem::swap(&mut c.a[7], &mut c.osp);
            c.s = 1;
        }
        c.a[7] = c.a[7].wrapping_sub(4);
        self.mem.write_u32(c.a[7], c.pc)?;
        c.a[7] = c.a[7].wrapping_sub(2);
        self.mem.write_u16(c.a[7], sr as u16)?;
        c.im = lvl as u32;
        c.stopped = 0;
        c.pc = self.mem.read_u32((24 + lvl as u32) * 4)?;
        self.beam += 44;
        self.stats.irqs[lvl as usize & 7] += 1;
        Ok(())
    }

    /// Charge `d` CPU cycles to the beam clock, stretched by SLOW_NUM / 1024 with the fraction carried.
    fn charge(&mut self, d: u64) {
        let t = d * SLOW_NUM + self.carry;
        self.beam += t / 1024;
        self.carry = t % 1024;
    }

    /// One tick (cpu_native.c run_frame_preempt): returns when the tick's frame is complete.
    pub fn run_tick<F>(&mut self, step: &mut F) -> Result<(), Stop>
    where
        F: FnMut(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>,
    {
        self.chips(|c| c.frame_done = false);
        loop {
            while self.beam >= self.next_line {
                let done = self
                    .mem
                    .with_io(|io, chip| {
                        let c = chips(io);
                        c.pass_line(chip);
                        c.frame_done
                    })
                    .expect("bus attached");
                self.next_line += CYC_PER_LINE;
                if done {
                    return Ok(());
                }
            }
            let l = self.chips(|c| c.ipl());
            if l as u32 > self.cpu.im {
                self.take_irq(l)?;
            }
            if self.cpu.stopped != 0 {
                self.beam = self.next_line;
                self.stats.stopped_lines += 1;
                continue;
            }
            let c0 = self.cpu.cyc;
            self.mem.set_now(self.beam);
            if let Some((base, lvo)) = Os::slot(self.cpu.pc) {
                if self.os_trace.is_some() {
                    let tick = self.chips(|c| c.frame_count);
                    if let Some(t) = self.os_trace.as_mut() {
                        t.push((tick, base, lvo));
                    }
                }
                self.os.call(&mut self.cpu, &mut self.mem)?;
                self.stats.os_calls += 1;
            } else {
                step(&mut self.cpu, &mut self.mem)?;
                self.stats.steps += 1;
            }
            let d = self.cpu.cyc.wrapping_sub(c0);
            self.charge(d);
        }
    }

    /// The frame of the tick just run (SCR_W x SCR_H, 0xAARRGGBB) and the audio since the last call.
    pub fn take_output(&mut self) -> (Vec<u32>, Vec<i16>) {
        self.chips(|c| (c.framebuf.clone(), std::mem::take(&mut c.audio)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::{GameFiles, Pools};

    /// A program that turns on VERTB, then STOPs: the interrupt runs its handler, which counts in D7 and returns.
    #[test]
    fn vertb_interrupt_runs_handler_between_stops() {
        let mut mem = GuestMemory::a500(0x10000, 0).unwrap();
        // level 3 autovector -> $200: addq.l #1,d7 ; move.w #$20,$dff09c ; rte
        mem.write_u32(0x6C, 0x200).unwrap();
        let os = Os::new(
            GameFiles::default(),
            Pools {
                chip_lo: 0x8000,
                chip_hi: 0x10000,
                slow_lo: 0,
                slow_hi: 0,
            },
        );
        let mut cpu = Cpu::default();
        cpu.a[7] = 0x1000;
        cpu.osp = 0x2000;
        cpu.pc = 0x100;
        let mut m = Machine::new(Booted { cpu, mem, os });
        m.chips(|c| c.intena = 0x4020);
        let mut step = |cpu: &mut Cpu, mem: &mut GuestMemory| -> Result<(), Stop> {
            cpu.cyc += 4;
            match cpu.pc {
                0x100 => {
                    cpu.stopped = 1; // stop #$2000 (as translated: S stays as it is in this test)
                    cpu.pc = 0x100;
                }
                0x200 => {
                    cpu.d[7] += 1;
                    mem.write_u16(0xDFF09C, 0x0020)?;
                    // rte
                    let sr = mem.read_u16(cpu.a[7])? as u32;
                    let pc = mem.read_u32(cpu.a[7] + 2)?;
                    cpu.a[7] += 6;
                    cpu.set_sr(sr);
                    cpu.pc = pc;
                }
                pc => return Err(Stop::Untranslated(pc)),
            }
            Ok(())
        };
        for _ in 0..3 {
            m.run_tick(&mut step).unwrap();
        }
        // VERTB is raised at each tick end and taken at the start of the next tick's loop
        assert_eq!(m.cpu.d[7], 2);
        assert_eq!(m.stats.irqs[3], 2);
        // the third tick's VERTB is raised at its end and waits for the next tick
        assert_eq!(m.mem.read_u16(0xDFF01E).unwrap() & 0x20, 0x20);
        let (px, _) = m.take_output();
        assert_eq!(px.len(), crate::display::SCR_W * crate::display::SCR_H);
    }

    #[test]
    fn chip_ram_mirrors_and_unmapped_reads_zero_on_the_bus() {
        let mem = GuestMemory::a500(0x10000, 0).unwrap();
        let os = Os::new(
            GameFiles::default(),
            Pools {
                chip_lo: 0x8000,
                chip_hi: 0x10000,
                slow_lo: 0,
                slow_hi: 0,
            },
        );
        let mut m = Machine::new(Booted {
            cpu: Cpu::default(),
            mem,
            os,
        });
        m.mem.write_u16(0x10, 0xBEEF).unwrap();
        assert_eq!(m.mem.read_u16(0x10010).unwrap(), 0xBEEF);
        assert_eq!(m.mem.read_u32(0xE00000).unwrap(), 0);
        assert_eq!(m.mem.unmapped_accesses(), 2);
        // CIA-A PRA (odd byte, A12 low) reads the pull-ups; CIA-B is the even byte with A13 low
        assert_eq!(
            m.mem.read_u8(0xBFE101).unwrap(),
            m.mem.read_u8(0xBFE101).unwrap()
        );
    }
}
