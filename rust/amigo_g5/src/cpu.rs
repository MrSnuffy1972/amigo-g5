//! Register state and flag helpers for translated code. Not a CPU: there is no fetch, decode or opcode table here.
//! Translated code (from `m68k2c.py --emit rust`) calls these helpers the way translated C calls `runtime/cpu.h`.
//!
//! Every helper names the C function it is checked against. The C code is the oracle (Karl, 25 Oct 2026).
//! Flags are kept as `u32` 0/1 like the C `cpu_t`, so differential tests compare like with like.

use crate::mem::{GuestMemory, MemError};

/// Why translated code stopped before reaching its stop address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    Mem(MemError),
    /// No translated code at this address (the C path hands this to its fallback interpreter; G5 has none).
    Untranslated(u32),
    /// A library call the OS services (`os.rs`) do not serve yet: the library base and the LVO (negative).
    OsCall {
        base: u32,
        lvo: i32,
    },
}

impl From<MemError> for Stop {
    fn from(e: MemError) -> Self {
        Stop::Mem(e)
    }
}

/// Mirrors `cpu_t` in runtime/cpu.h, plus the cycle count (`CYC`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cpu {
    pub d: [u32; 8],
    pub a: [u32; 8],
    pub pc: u32,
    pub osp: u32,
    pub s: u32,
    pub im: u32,
    pub x: u32,
    pub n: u32,
    pub z: u32,
    pub v: u32,
    pub c: u32,
    pub stopped: u32,
    pub cyc: u64,
}

/// `SX8` in cpu.h.
pub fn sx8(v: u32) -> u32 {
    v as u8 as i8 as i32 as u32
}

/// `SX16` in cpu.h.
pub fn sx16(v: u32) -> u32 {
    v as u16 as i16 as i32 as u32
}

fn b(v: bool) -> u32 {
    v as u32
}

macro_rules! arith {
    ($add:ident, $sub:ident, $cmp:ident, $addx:ident, $subx:ident, $logic:ident, $bits:expr, $mask:expr, $sign:expr) => {
        /// `add<SZ>` in cpu.h.
        pub fn $add(&mut self, s: u32, d: u32) -> u32 {
            let (s, d) = (s & $mask, d & $mask);
            let r64 = s as u64 + d as u64;
            let r = (r64 as u32) & $mask;
            self.c = ((r64 >> $bits) & 1) as u32;
            self.x = self.c;
            self.v = b(((s ^ r) & (d ^ r) & $sign) != 0);
            self.n = b(r & $sign != 0);
            self.z = b(r == 0);
            r
        }
        /// `sub<SZ>` in cpu.h.
        pub fn $sub(&mut self, s: u32, d: u32) -> u32 {
            let (s, d) = (s & $mask, d & $mask);
            let r = d.wrapping_sub(s) & $mask;
            self.c = b(s > d);
            self.x = self.c;
            self.v = b(((s ^ d) & (r ^ d) & $sign) != 0);
            self.n = b(r & $sign != 0);
            self.z = b(r == 0);
            r
        }
        /// `cmp<SZ>` in cpu.h.
        pub fn $cmp(&mut self, s: u32, d: u32) {
            let (s, d) = (s & $mask, d & $mask);
            let r = d.wrapping_sub(s) & $mask;
            self.c = b(s > d);
            self.v = b(((s ^ d) & (r ^ d) & $sign) != 0);
            self.n = b(r & $sign != 0);
            self.z = b(r == 0);
        }
        /// `addx<SZ>` in cpu.h.
        pub fn $addx(&mut self, s: u32, d: u32) -> u32 {
            let (s, d) = (s & $mask, d & $mask);
            let r64 = s as u64 + d as u64 + self.x as u64;
            let r = (r64 as u32) & $mask;
            self.c = ((r64 >> $bits) & 1) as u32;
            self.x = self.c;
            self.v = b(((s ^ r) & (d ^ r) & $sign) != 0);
            self.n = b(r & $sign != 0);
            if r != 0 {
                self.z = 0;
            }
            r
        }
        /// `subx<SZ>` in cpu.h.
        pub fn $subx(&mut self, s: u32, d: u32) -> u32 {
            let (s, d) = (s & $mask, d & $mask);
            let r64 = (d as u64)
                .wrapping_sub(s as u64)
                .wrapping_sub(self.x as u64);
            let r = (r64 as u32) & $mask;
            self.c = b(s as u64 + self.x as u64 > d as u64);
            self.x = self.c;
            self.v = b(((s ^ d) & (r ^ d) & $sign) != 0);
            self.n = b(r & $sign != 0);
            if r != 0 {
                self.z = 0;
            }
            r
        }
        /// `logic<SZ>` in cpu.h.
        pub fn $logic(&mut self, r: u32) -> u32 {
            let r = r & $mask;
            self.n = b(r & $sign != 0);
            self.z = b(r == 0);
            self.v = 0;
            self.c = 0;
            r
        }
    };
}

macro_rules! shifts {
    ($asl:ident, $asr:ident, $lsl:ident, $lsr:ident, $rol:ident, $ror:ident, $roxl:ident, $roxr:ident, $mask:expr, $sign:expr) => {
        /// `asl<SZ>` in cpu.h.
        pub fn $asl(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
                self.v = 0;
            } else {
                let (mut v, mut r, mut c) = (0, d, 0);
                for _ in 0..cnt {
                    c = b(r & $sign != 0);
                    r = (r << 1) & $mask;
                    if b(r & $sign != 0) != c {
                        v = 1;
                    }
                }
                d = r;
                self.c = c;
                self.x = c;
                self.v = v;
            }
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `asr<SZ>` in cpu.h.
        pub fn $asr(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
            } else {
                let mut c = 0;
                for _ in 0..cnt {
                    c = d & 1;
                    d = (d >> 1) | (d & $sign);
                }
                self.c = c;
                self.x = c;
            }
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `lsl<SZ>` in cpu.h.
        pub fn $lsl(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
            } else {
                let mut c = 0;
                for _ in 0..cnt {
                    c = b(d & $sign != 0);
                    d = (d << 1) & $mask;
                }
                self.c = c;
                self.x = c;
            }
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `lsr<SZ>` in cpu.h.
        pub fn $lsr(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
            } else {
                let mut c = 0;
                for _ in 0..cnt {
                    c = d & 1;
                    d >>= 1;
                }
                self.c = c;
                self.x = c;
            }
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `rol<SZ>` in cpu.h.
        pub fn $rol(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
            } else {
                let mut c = 0;
                for _ in 0..cnt {
                    c = b(d & $sign != 0);
                    d = ((d << 1) | c) & $mask;
                }
                self.c = c;
            }
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `ror<SZ>` in cpu.h.
        pub fn $ror(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            if cnt == 0 {
                self.c = 0;
            } else {
                let mut c = 0;
                for _ in 0..cnt {
                    c = d & 1;
                    d = (d >> 1) | if c != 0 { $sign } else { 0 };
                }
                self.c = c;
            }
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `roxl<SZ>` in cpu.h.
        pub fn $roxl(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            for _ in 0..cnt {
                let c = b(d & $sign != 0);
                d = ((d << 1) | self.x) & $mask;
                self.x = c;
            }
            self.c = self.x;
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
        /// `roxr<SZ>` in cpu.h.
        pub fn $roxr(&mut self, d: u32, cnt: u32) -> u32 {
            let mut d = d & $mask;
            for _ in 0..cnt {
                let c = d & 1;
                d = (d >> 1) | if self.x != 0 { $sign } else { 0 };
                self.x = c;
            }
            self.c = self.x;
            self.v = 0;
            self.n = b(d & $sign != 0);
            self.z = b(d == 0);
            d
        }
    };
}

impl Cpu {
    /// `get_sr` in cpu.h.
    pub fn get_sr(&self) -> u32 {
        (self.s << 13)
            | (self.im << 8)
            | (self.x << 4)
            | (self.n << 3)
            | (self.z << 2)
            | (self.v << 1)
            | self.c
    }

    /// `set_ccr` in cpu.h.
    pub fn set_ccr(&mut self, v: u32) {
        self.x = (v >> 4) & 1;
        self.n = (v >> 3) & 1;
        self.z = (v >> 2) & 1;
        self.v = (v >> 1) & 1;
        self.c = v & 1;
    }

    /// `set_sr` in cpu.h: swaps A7 with the other stack pointer when the supervisor bit changes.
    pub fn set_sr(&mut self, v: u32) {
        let s = (v >> 13) & 1;
        if s != self.s {
            std::mem::swap(&mut self.a[7], &mut self.osp);
            self.s = s;
        }
        self.im = (v >> 8) & 7;
        self.set_ccr(v);
    }

    /// `cpu_exception` in runtime/cpu_native.c: enter supervisor mode, push the return PC and the old SR, and
    /// return the handler address from the vector table. Used by translated trap, trapv, chk and divide by zero.
    /// The cycle charge is the C function's own (34), on top of the instruction's.
    pub fn exception(&mut self, mem: &mut GuestMemory, vec: u32, pc: u32) -> Result<u32, Stop> {
        let sr = self.get_sr();
        if self.s == 0 {
            std::mem::swap(&mut self.a[7], &mut self.osp);
            self.s = 1;
        }
        self.a[7] = self.a[7].wrapping_sub(4);
        mem.write_u32(self.a[7], pc)?;
        self.a[7] = self.a[7].wrapping_sub(2);
        mem.write_u16(self.a[7], sr as u16)?;
        self.cyc = self.cyc.wrapping_add(34);
        Ok(mem.read_u32(vec.wrapping_mul(4))?)
    }

    /// `cond` in cpu.h.
    pub fn cond(&self, cc: u32) -> bool {
        let (n, z, v, c) = (self.n != 0, self.z != 0, self.v != 0, self.c != 0);
        match cc {
            0 => true,
            1 => false,
            2 => !c && !z,
            3 => c || z,
            4 => !c,
            5 => c,
            6 => !z,
            7 => z,
            8 => !v,
            9 => v,
            10 => !n,
            11 => n,
            12 => n == v,
            13 => n != v,
            14 => n == v && !z,
            _ => z || n != v,
        }
    }

    arith!(add1, sub1, cmp1, addx1, subx1, logic1, 8, 0xff, 0x80);
    arith!(add2, sub2, cmp2, addx2, subx2, logic2, 16, 0xffff, 0x8000);
    arith!(
        add4,
        sub4,
        cmp4,
        addx4,
        subx4,
        logic4,
        32,
        0xffff_ffff,
        0x8000_0000
    );
    shifts!(asl1, asr1, lsl1, lsr1, rol1, ror1, roxl1, roxr1, 0xff, 0x80);
    shifts!(asl2, asr2, lsl2, lsr2, rol2, ror2, roxl2, roxr2, 0xffff, 0x8000);
    shifts!(
        asl4,
        asr4,
        lsl4,
        lsr4,
        rol4,
        ror4,
        roxl4,
        roxr4,
        0xffff_ffff,
        0x8000_0000
    );

    /// `abcd` in cpu.h.
    pub fn abcd(&mut self, s: u32, d: u32) -> u32 {
        let mut res = (s & 15) + (d & 15) + self.x;
        let pre = res;
        if res > 9 {
            res += 6;
        }
        res += (s & 0xf0) + (d & 0xf0);
        self.c = b(res > 0x99);
        self.x = self.c;
        if self.c != 0 {
            res = res.wrapping_sub(0xa0);
        }
        self.v = b((!pre & res) & 0x80 != 0);
        self.n = b(res & 0x80 != 0);
        res &= 0xff;
        if res != 0 {
            self.z = 0;
        }
        res
    }

    /// `sbcd` in cpu.h.
    pub fn sbcd(&mut self, s: u32, d: u32) -> u32 {
        let mut res = (d & 15).wrapping_sub(s & 15).wrapping_sub(self.x);
        let pre = res;
        if res > 9 {
            res = res.wrapping_sub(6);
        }
        res = res.wrapping_add((d & 0xf0).wrapping_sub(s & 0xf0));
        self.c = b(res > 0x99);
        self.x = self.c;
        if self.c != 0 {
            res = res.wrapping_add(0xa0);
        }
        res &= 0x1ff;
        self.v = b((pre & !res) & 0x80 != 0);
        self.n = b(res & 0x80 != 0);
        res &= 0xff;
        if res != 0 {
            self.z = 0;
        }
        res
    }
}

/// Runs a translated-code step function until `pc == stop_pc` or `max_steps` steps have run.
/// Returns the number of steps taken. The step function is generated code: one match arm per translated
/// instruction, selected by address, as in the C dispatcher. Nothing here decodes an opcode.
pub fn run_translated<F>(
    cpu: &mut Cpu,
    mem: &mut GuestMemory,
    stop_pc: u32,
    max_steps: u64,
    mut step: F,
) -> Result<u64, Stop>
where
    F: FnMut(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>,
{
    let mut steps = 0;
    while cpu.pc != stop_pc && steps < max_steps {
        step(cpu, mem)?;
        steps += 1;
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sets_carry_overflow_and_zero() {
        let mut c = Cpu::default();
        assert_eq!(c.add1(0x80, 0x80), 0);
        assert_eq!((c.c, c.x, c.v, c.n, c.z), (1, 1, 1, 0, 1));
        assert_eq!(c.add4(1, 0x7FFF_FFFF), 0x8000_0000);
        assert_eq!((c.c, c.v, c.n, c.z), (0, 1, 1, 0));
    }

    #[test]
    fn sub_and_cmp_borrow() {
        let mut c = Cpu::default();
        assert_eq!(c.sub2(2, 1), 0xFFFF);
        assert_eq!((c.c, c.x, c.n), (1, 1, 1));
        c.x = 0;
        c.cmp4(5, 5);
        assert_eq!((c.z, c.c, c.x), (1, 0, 0));
    }

    #[test]
    fn set_sr_swaps_stack_pointers_when_mode_changes() {
        let mut c = Cpu {
            s: 1,
            ..Cpu::default()
        };
        c.a[7] = 0x100;
        c.osp = 0x200;
        c.set_sr(0x0015);
        assert_eq!((c.s, c.a[7], c.osp, c.get_sr()), (0, 0x200, 0x100, 0x15));
    }

    #[test]
    fn conditions_follow_flags() {
        let mut c = Cpu::default();
        c.set_ccr(0b01000);
        assert!(c.cond(11) && c.cond(13) && !c.cond(12) && c.cond(15));
        assert!(!c.cond(1) && c.cond(0));
    }

    #[test]
    fn exception_from_user_mode_switches_stack_and_pushes_frame() {
        let mut mem = GuestMemory::new(&[(0, 0x1000)]).unwrap();
        mem.write_u32(0x8C, 0x0456).unwrap();
        let mut c = Cpu::default();
        c.a[7] = 0x800;
        c.osp = 0xC00;
        c.set_ccr(0x1F);
        assert_eq!(c.exception(&mut mem, 35, 0x0123_4566).unwrap(), 0x0456);
        assert_eq!((c.s, c.a[7], c.osp, c.cyc), (1, 0xC00 - 6, 0x800, 34));
        assert_eq!(mem.read_u16(0xC00 - 6).unwrap(), 0x001F);
        assert_eq!(mem.read_u32(0xC00 - 4).unwrap(), 0x0123_4566);
    }

    #[test]
    fn shifts_set_x_and_c_from_last_bit_out() {
        let mut c = Cpu::default();
        assert_eq!(c.lsl1(0x81, 1), 0x02);
        assert_eq!((c.c, c.x), (1, 1));
        assert_eq!(c.asr2(0x8001, 1), 0xC000);
        assert_eq!((c.c, c.n), (1, 1));
        assert_eq!(c.rol4(0x8000_0000, 0), 0x8000_0000);
        assert_eq!(c.c, 0);
    }
}
