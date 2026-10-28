//! Cold boot of a filed program: the program's own hunk file and data files, its entry point, and the start state
//! AmigaDOS gives a CLI program. No snapshot of a reference run is used.
//!
//! Ported from the C kit's filed-game boot, which is the oracle (`tests/os_diff.rs`): `kit/amigo/port/oshle/oshle.py`
//! (`execlib`, the memory layout in `main`) and `oshle.c` (`game_boot_extra`, CLI start, not Workbench).
//! The low 16 KB hold the fake OS's tables (library bases with one jump-table slot per function, a Process with a
//! CLI); the slots are never run as code, `os::run` serves them.

use crate::cpu::Cpu;
use crate::hunk::{self, HunkError, LoadOptions};
use crate::mem::{GuestMemory, MapError, MemError, SLOW_BASE};
use crate::os::{GameFiles, Os, Pools, CLI, EXEC, EXIT_PC, GFX, LIBS, TASK};

/// Supervisor stack top (the reset vector); $7A000-$7C000 is kept out of AllocMem.
const SSP: u32 = 0x7C000;
/// The CLI program's stack top before the boot pushes its stack size and return address (portcfg.py default A7).
const USER_SP: u32 = 0x7FFF0;
/// oshle.py sets chip_kb = 1024 and slow_kb = 1536 for every filed game it boots.
pub const CHIP_SIZE: usize = 1024 * 1024;
pub const SLOW_SIZE: usize = 1536 * 1024;
/// exec slot past every real LVO that the illegal-instruction vector points at (oshle.c `ILL_PC`).
const ILL_PC: u32 = EXEC - 0x336;

/// The fake OS's own 68000 code from oshle.py (the kit's, written by hand there): the disk-library stub at $3E00,
/// the level 3 server runner at $3E40 and the default autovector handler at $3E70.
const STUB: (u32, &str) = (
    0x3E00,
    "48E77FFE20383EF020783EF422783EF82C7800044E914A8067062C404EAEFFFA4CDF7FFE4E75",
);
const L3: (u32, &str) = (
    0x3E40,
    "48E7FFFE49F83EC0201C6714225C24402A4041F900DFF0002C7800044E9260E833FC007000DFF09C4CDF7FFF4E73",
);
const LX: (u32, &str) = (0x3E70, "4A3900BFED014A3900BFDD0033FC3F8F00DFF09C4E73");

#[derive(Debug)]
pub enum BootError {
    Hunk(HunkError),
    Map(MapError),
    Mem(MemError),
    /// The program fills chip RAM: no room is left for AllocMem.
    NoRoom,
    /// An overlaid program (overlay manager header in its first hunk) is not booted yet.
    Overlay,
}

impl From<HunkError> for BootError {
    fn from(e: HunkError) -> Self {
        BootError::Hunk(e)
    }
}
impl From<MemError> for BootError {
    fn from(e: MemError) -> Self {
        BootError::Mem(e)
    }
}
impl From<MapError> for BootError {
    fn from(e: MapError) -> Self {
        BootError::Map(e)
    }
}

/// A program ready to run from its entry point.
#[derive(Debug)]
pub struct Booted {
    pub cpu: Cpu,
    pub mem: GuestMemory,
    pub os: Os,
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap_or(0))
        .collect()
}

/// Mirrors oshle.py `execlib`: the fake OS tables at $0-$4000.
pub fn execlib(mem: &mut GuestMemory) -> Result<(), MemError> {
    mem.write_u32(0, SSP)?;
    mem.write_u32(4, EXEC)?; // AbsExecBase
    for &(name, base, n) in LIBS.iter() {
        for i in 1..=n {
            mem.write_bytes(base - 6 * i, &[0x4E, 0x75, 0x4E, 0x71, 0x4E, 0x71])?;
            // RTS NOP NOP
        }
        mem.write_u16(base + 0x14, 34)?; // lib_Version (Kickstart 1.3)
        mem.write_bytes(base + 0x80, name.as_bytes())?;
        mem.write_u32(base + 0x0A, base + 0x80)?; // ln_Name
    }
    mem.write_u16(EXEC + 0x212, 50)?; // VBlankFrequency (PAL)
    mem.write_u32(EXEC + 0x114, TASK)?; // ThisTask
    mem.write_u8(TASK + 8, 13)?; // NT_PROCESS
    mem.write_u32(TASK + 0xAC, CLI >> 2)?; // pr_CLI: started from the CLI
    mem.write_u16(EXIT_PC, 0x60FE)?; // BRA.S *
    for (at, code) in [STUB, L3, LX] {
        mem.write_bytes(at, &hex(code))?;
    }
    for v in [0x64, 0x68, 0x70, 0x74, 0x78] {
        mem.write_u32(v, LX.0)?; // autovectors 1, 2, 4, 5, 6
    }
    mem.write_u32(0x6C, L3.0)?; // level 3: the VERTB server runner
    Ok(())
}

/// Boots `exe` (an AmigaDOS hunk executable) as a CLI program with argument line `args` (ending in a newline, as
/// AmigaDOS passes it) and its data `files`. Mirrors the C kit: hunkload with LoadSeg headers and chip hunks from
/// $4000 (portg0.py `rebuild`), the AllocMem pools (oshle.py `main`), then `game_boot_extra` (oshle.c).
pub fn cold_boot(exe: &[u8], args: &[u8], files: GameFiles) -> Result<Booted, BootError> {
    let opts = LoadOptions {
        chip_base: 0x4000,
        seglist: true,
        ..LoadOptions::default()
    };
    let prog = hunk::load(exe, &opts)?;
    let end = (prog.base as u64 + prog.image.len() as u64 + 0xFFF) & !0xFFF;
    let in_chip = prog.base < 0x80000;
    let pools = Pools {
        chip_lo: if in_chip {
            end.max(0x80000) as u32
        } else {
            0x80000
        },
        chip_hi: CHIP_SIZE as u32,
        slow_lo: if in_chip {
            SLOW_BASE
        } else {
            end.max(SLOW_BASE as u64) as u32
        },
        slow_hi: SLOW_BASE + SLOW_SIZE as u32,
    };
    if pools.chip_lo >= pools.chip_hi {
        return Err(BootError::NoRoom);
    }
    let mut mem = GuestMemory::a500(CHIP_SIZE, SLOW_SIZE)?;
    execlib(&mut mem)?;
    prog.write_to(&mut mem, true)?;
    let seglist = prog.seglist.unwrap_or(0);
    let h0 = (seglist << 2) + 4;
    if seglist != 0 && mem.read_u16(h0)? == 0x6000 && mem.read_u32(h0 + 4)? == 0xABCD {
        return Err(BootError::Overlay);
    }
    let mut os = Os::new(files, pools);
    let mut r = [0u32; 16];
    r[15] = USER_SP;

    os.ramlib_sp = SSP - 0x1800;
    // A CLI program starts with the display DMA Kickstart left on and an empty copper list (oshle.c EPOCH16), and
    // GfxBase->LOFlist names a Workbench screen's list: two 640x256 planes at the top of free chip (EPOCH17).
    for (i, w) in [0xFFFFu16, 0xFFFE, 0xFFFF, 0xFFFE].iter().enumerate() {
        mem.write_u16(0x3DF8 + 2 * i as u32, *w)?;
    }
    os.chip_log.extend([
        (0xDFF080, 0),
        (0xDFF082, 0x3DF8),
        (0xDFF088, 0),
        (0xDFF096, 0x83C0),
    ]);
    let top = os.chip_top() & !7;
    let pb = 80 * 256;
    let need = 2 * pb + 32;
    if top > os.chip_next() + need + 0x40000 {
        let pl = top - need;
        let cl = pl + 2 * pb;
        mem.write_bytes(pl, &vec![0; need as usize])?;
        let list = [
            0xE0,
            pl >> 16,
            0xE2,
            pl & 0xFFFF,
            0xE4,
            (pl + pb) >> 16,
            0xE6,
            (pl + pb) & 0xFFFF,
        ];
        for (i, w) in list
            .iter()
            .chain(&[0x100, 0xA200, 0xFFFF, 0xFFFE])
            .enumerate()
        {
            mem.write_u16(cl + 2 * i as u32, *w as u16)?;
        }
        mem.write_u32(GFX + 0x32, cl)?;
        os.dsp_top = pl;
    }
    mem.write_u32(CLI + 0x3C, seglist)?; // cli_Module
    if mem.read_u32(0x10)? == 0 {
        mem.write_u32(0x10, ILL_PC)?;
    }
    // D0 = argument length, A0 = the arguments; cli_CommandName a BSTR
    let na = args.len().min(0xBE);
    mem.write_bytes(0x3D00, &args[..na])?;
    mem.write_u8(0x3D00 + na as u32, 0)?;
    mem.write_bytes(CLI + 0xC0, b"\x04game")?;
    mem.write_u32(CLI + 0x10, (CLI + 0xC0) >> 2)?;
    r[0] = na as u32;
    r[8] = 0x3D00;
    // The stack size at 4(SP) and in the Task, Process and CLI (SAS/C and Lattice startup code checks it)
    let top = r[15];
    let size = if top > 0x7C100 && top - SSP < 0x10000 {
        top - SSP - 0x40
    } else {
        0x4000
    } & !3;
    mem.write_u32(TASK + 0x3A, top - size)?; // tc_SPLower
    mem.write_u32(TASK + 0x3E, top)?; // tc_SPUpper
    mem.write_u32(TASK + 0x84, size)?; // pr_StackSize
    mem.write_u32(CLI + 0x34, size >> 2)?; // cli_DefaultStack
    let mut j = (top - size + 3) & !3;
    while j + 4 <= top - 8 {
        mem.write_u32(j, 0xABAD_CAFE)?; // an uncleared stack, as the C kit leaves it
        j += 4;
    }
    r[15] -= 4;
    mem.write_u32(r[15], size)?;
    r[2] = size; // as RunCommand leaves it
    os.boot_top = top;
    r[15] -= 4;
    mem.write_u32(r[15], EXIT_PC)?; // the program's final RTS lands on BRA.S *
    os.boot_ret = r[15];
    os.chip_log.push((0xDFF09A, 0xC000)); // INTENA master on, as Kickstart leaves it
    os.lof0 = mem.read_u32(GFX + 0x32)?;
    os.mirror(&mut mem)?;

    // User mode, interrupts open; the other stack pointer is the supervisor stack (cpu.c run_boot, SR = 0)
    let mut cpu = Cpu::default();
    cpu.d.copy_from_slice(&r[..8]);
    cpu.a.copy_from_slice(&r[8..]);
    cpu.osp = mem.read_u32(0)?;
    cpu.pc = prog.entry;
    Ok(Booted { cpu, mem, os })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execlib_slots_names_and_vectors() {
        let mut m = GuestMemory::a500(CHIP_SIZE, 0).unwrap();
        execlib(&mut m).unwrap();
        assert_eq!(m.read_u32(4).unwrap(), 0xC00);
        assert_eq!(m.read_u16(0x1400 - 30).unwrap(), 0x4E75);
        assert_eq!(m.read_u32(0x1400 + 0x0A).unwrap(), 0x1480);
        assert_eq!(m.read_bytes(0x1480, 11).unwrap(), b"dos.library");
        assert_eq!(m.read_u32(0x6C).unwrap(), 0x3E40);
        assert_eq!(m.read_u16(0x3E70).unwrap(), 0x4A39);
        assert_eq!(Os::slot(0x1400 - 30), Some((0x1400, -30)));
        assert_eq!(Os::slot(0x1400 - 31), None);
    }
}
