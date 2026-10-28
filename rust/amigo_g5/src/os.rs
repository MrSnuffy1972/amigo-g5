//! OS services for a cold-booted filed program: the part of exec and dos a simple program calls (memory, libraries,
//! files, locks). Ported from `kit/amigo/port/oshle/oshle.c` (`hle_lib_body`, `exec_call`, `dos_call`), which is the
//! oracle: `tests/os_diff.rs` links that file and runs the same synthetic programs on both.
//!
//! Decision 28 Oct 2026 (M4): a call the C serves but this port does not yet is a typed stop (`Stop::OsCall`), never
//! a silent "return 0", so a program that needs more of the OS says so at once instead of drifting from the C.
//! The library bases are the same fake ones the C kit lays out (oshle.py `LIBS`): the program sees the same
//! addresses, and the jump-table slot it calls through names the service.

use crate::cpu::{Cpu, Stop};
use crate::mem::{GuestMemory, MemError};

/// oshle.py `LIBS`: (name, base, jump-table slots).
pub const LIBS: [(&str, u32, u32); 11] = [
    ("exec.library", 0xC00, 140),
    ("dos.library", 0x1400, 170),
    ("graphics.library", 0x1C00, 180),
    ("intuition.library", 0x2400, 140),
    ("icon.library", 0x2C00, 28),
    ("trackdisk.device", 0x3400, 16),
    ("disk.resource", 0x3800, 16),
    ("ciaa.resource", 0x3C00, 16),
    ("mathffp.library", 0x3000, 16),
    ("dummy.device", 0x3200, 16),
    ("layers.library", 0x3600, 40),
];
pub const EXEC: u32 = 0xC00;
pub const DOS: u32 = 0x1400;
pub const GFX: u32 = 0x1C00;
pub const TASK: u32 = 0xF00;
pub const CLI: u32 = 0x3F00;
/// Where a program's final RTS lands (a `bra.s *`).
pub const EXIT_PC: u32 = 0x3FF0;
const MEMF_CHIP: u32 = 2;
const NFH: usize = 8;
const FH0: u32 = 0x100;
const NLOCK: usize = 16;
const LK0: u32 = 0x300;
const ROOT_LOCK: u32 = 0x200;
const NFREE: usize = 512;
/// oshle.c `osp_out`: the allocator and boot state mirrored into guest RAM, so a later boot can carry on.
const OSP_M: u32 = 0x3FC8;
const OSP_MAGIC: u32 = 0x4F53_484C;
/// oshle.c `VBL_SERVERS`: the AddIntServer VERTB list (is_Code, is_Data; 0 ends), run by the level 3 handler at $3E40.
const VBL_SERVERS: u32 = 0x3EC0;

/// The program's files, as the C kit embeds them (looked up by the last part of the path).
#[derive(Debug, Clone, Default)]
pub struct GameFiles {
    files: Vec<(String, Vec<u8>)>,
}

impl GameFiles {
    pub fn add(&mut self, name: &str, data: Vec<u8>) {
        self.files.push((name.to_string(), data));
    }

    /// Mirrors `gamefile` (boot.h): the exact name first, then ignoring case (`amigo_streq_ci`).
    pub fn find(&self, name: &[u8]) -> Option<usize> {
        let exact = self.files.iter().position(|(n, _)| n.as_bytes() == name);
        exact.or_else(|| {
            self.files
                .iter()
                .position(|(n, _)| n.as_bytes().eq_ignore_ascii_case(name))
        })
    }

    pub fn data(&self, i: usize) -> &[u8] {
        &self.files[i].1
    }
}

#[derive(Debug, Clone, Copy)]
struct Handle {
    file: usize,
    pos: usize,
}

#[derive(Debug, Clone)]
struct Lock {
    file: usize,
    name: Vec<u8>,
}

/// The free RAM the C kit gives AllocMem (oshle.py `main`: `oshle_chip_lo` .. `oshle_slow_hi`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pools {
    pub chip_lo: u32,
    pub chip_hi: u32,
    pub slow_lo: u32,
    pub slow_hi: u32,
}

/// State of the OS services. Everything the C keeps in `static` variables lives here (no global state).
#[derive(Debug, Clone)]
pub struct Os {
    pub files: GameFiles,
    pub pools: Pools,
    chip_next: u32,
    slow_next: u32,
    /// The display's copper list and planes sit here, below `chip_hi` (oshle.c `dsp_top`); 0 = none.
    pub(crate) dsp_top: u32,
    freed: Vec<(u32, u32)>,
    handles: [Option<Handle>; NFH],
    locks: [Option<Lock>; NLOCK],
    io_err: u32,
    /// oshle.c `vbk_nest`: Disable/Enable nesting, -1 = enabled.
    nest: i32,
    pub(crate) boot_top: u32,
    pub(crate) boot_ret: u32,
    pub(crate) ramlib_sp: u32,
    pub(crate) lof0: u32,
    /// Custom-chip register writes the C fake OS makes (boot display, INTENA), in order. A record for the
    /// presentation code (M5) to act on; nothing here models a chip.
    pub chip_log: Vec<(u32, u16)>,
    /// What the program wrote to its console (dos Write), for tests and logs.
    pub console: Vec<u8>,
}

impl Os {
    pub fn new(files: GameFiles, pools: Pools) -> Self {
        Os {
            files,
            pools,
            chip_next: pools.chip_lo,
            slow_next: pools.slow_lo,
            dsp_top: 0,
            freed: Vec::new(),
            handles: [None; NFH],
            locks: Default::default(),
            io_err: 0,
            nest: -1,
            boot_top: 0,
            boot_ret: 0,
            ramlib_sp: 0x7A800,
            lof0: 0,
            chip_log: Vec::new(),
            console: Vec::new(),
        }
    }

    /// The library whose jump table holds `pc`, when `pc` is one of its slots.
    pub fn slot(pc: u32) -> Option<(u32, i32)> {
        LIBS.iter().find_map(|&(_, base, n)| {
            let lo = base - 6 * n;
            (pc >= lo && pc < base && (base - pc).is_multiple_of(6))
                .then(|| (base, pc as i32 - base as i32))
        })
    }

    pub(crate) fn chip_top(&self) -> u32 {
        if self.dsp_top != 0 {
            self.dsp_top
        } else {
            self.pools.chip_hi
        }
    }

    pub(crate) fn chip_next(&self) -> u32 {
        self.chip_next
    }

    /// Mirrors oshle.c `alloc` (no chip gap: the C kit's `oshle_gap_lo` is optional and not set for this layout).
    fn alloc(&mut self, mem: &mut GuestMemory, size: u32, flags: u32) -> Result<u32, Stop> {
        let size = size.wrapping_add(7) & !7;
        let p = &self.pools;
        let chip = flags & MEMF_CHIP != 0
            || p.slow_hi == 0
            || self.slow_next as u64 + size as u64 > p.slow_hi as u64;
        let (next, end) = if chip {
            (self.chip_next, self.chip_top())
        } else {
            (self.slow_next, p.slow_hi)
        };
        if next as u64 + size as u64 > end as u64 {
            return Ok(0);
        }
        if chip {
            self.chip_next += size;
        } else {
            self.slow_next += size;
        }
        mem.write_bytes(next, &vec![0; size as usize])?;
        Ok(next)
    }

    /// oshle.c `osp_out` (the display part writes nothing until a screen exists, which M4 never makes).
    pub(crate) fn mirror(&self, mem: &mut GuestMemory) -> Result<(), MemError> {
        let words = [
            self.boot_top,
            self.boot_ret,
            self.ramlib_sp,
            self.dsp_top,
            self.lof0,
            8,
        ];
        for (i, w) in words.iter().enumerate() {
            mem.write_u32(OSP_M + 4 * i as u32, *w)?;
        }
        mem.write_u32(0x3FE0, self.chip_next)?;
        mem.write_u32(0x3FE4, self.slow_next)?;
        mem.write_u32(0x3FE8, OSP_MAGIC)?;
        mem.write_u32(0x3FEC, 0)?;
        Ok(())
    }

    fn intena(&mut self, v: u16) {
        self.chip_log.push((0xDFF09A, v));
    }

    /// Serves the library call at `cpu.pc` and returns to the caller. Mirrors `hle_lib` / `hle_lib_body`.
    pub fn call(&mut self, cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {
        let (base, lvo) = Os::slot(cpu.pc).ok_or(Stop::Untranslated(cpu.pc))?;
        let done = match base {
            EXEC => self.exec_call(cpu, mem, lvo)?,
            DOS => {
                let r = self.dos_call(cpu, mem, lvo)?;
                // Kickstart 1.3's dos returns its result in D1 as well (oshle.c, measured 14 Oct on 3D World Tennis)
                if r == Done::Return {
                    cpu.d[1] = cpu.d[0];
                }
                r
            }
            _ => Done::No,
        };
        match done {
            Done::No => return Err(Stop::OsCall { base, lvo }),
            Done::Return => {
                cpu.pc = mem.read_u32(cpu.a[7])?;
                cpu.a[7] = cpu.a[7].wrapping_add(4);
            }
            Done::Jumped => {}
        }
        Ok(self.mirror(mem)?)
    }

    /// Mirrors oshle.c `exec_call` for the services listed; anything else is `Done::No`.
    fn exec_call(&mut self, cpu: &mut Cpu, mem: &mut GuestMemory, lvo: i32) -> Result<Done, Stop> {
        let (d, a) = (cpu.d, cpu.a);
        match lvo {
            -0x78 => {
                // Disable: INTENA off, nested
                self.nest += 1;
                if self.nest == 0 {
                    self.intena(0x4000);
                }
            }
            -0x7E => {
                // Enable
                self.nest -= 1;
                if self.nest < 0 {
                    self.nest = -1;
                    self.intena(0xC000);
                }
            }
            -0x84 | -0x8A => {} // Forbid, Permit
            -0xC6 => {
                // AllocMem(D0 size, D1 flags): an exact-size freed block first, then the bump allocator
                let (s, f) = (d[0], d[1]);
                cpu.d[0] = 0;
                let z = s.wrapping_add(7) & !7;
                let chip_hi = self.pools.chip_hi;
                let hit = self.freed.iter().position(|&(fa, fz)| {
                    z != 0 && fz == z && (f & MEMF_CHIP == 0 || fa < chip_hi)
                });
                if let Some(j) = hit {
                    let (fa, _) = self.freed.swap_remove(j);
                    mem.write_bytes(fa, &vec![0; z as usize])?;
                    cpu.d[0] = fa;
                }
                if cpu.d[0] == 0 {
                    cpu.d[0] = self.alloc(mem, s, f)?;
                }
            }
            -0xD2 => {
                // FreeMem(A1 block, D0 size): remembered for an AllocMem of the same size
                let (fa, z) = (a[1], d[0].wrapping_add(7) & !7);
                let p = self.pools;
                let ours = (fa >= p.chip_lo && fa as u64 + z as u64 <= self.chip_next as u64)
                    || (p.slow_hi != 0
                        && fa >= p.slow_lo
                        && fa as u64 + z as u64 <= self.slow_next as u64);
                if fa != 0
                    && z != 0
                    && self.freed.len() < NFREE
                    && ours
                    && !self.freed.iter().any(|b| b.0 == fa)
                {
                    self.freed.push((fa, z));
                }
            }
            -0xD8 => {
                // AvailMem(D1 flags)
                let ch = self.chip_top().wrapping_sub(self.chip_next);
                let sl = self.pools.slow_hi.wrapping_sub(self.slow_next);
                cpu.d[0] = if d[1] & MEMF_CHIP != 0 {
                    ch
                } else if d[1] & 0x20000 != 0 {
                    sl.max(ch)
                } else {
                    sl.wrapping_add(ch)
                };
            }
            -0x126 => cpu.d[0] = if a[1] != 0 { 0 } else { TASK }, // FindTask(A1 name)
            -0x198 | -0x228 => {
                // OldOpenLibrary, OpenLibrary(A1 name): the fake bases; a library from the game's disk is not done yet
                let name = cstr(mem, a[1], 64)?;
                let base = lib_base(&name);
                let tail = last_part(&name);
                if base == 0
                    && (self.files.find(tail).is_some()
                        || self.files.find(&[b"libs/", tail].concat()).is_some())
                {
                    return Ok(Done::No);
                }
                cpu.d[0] = base;
            }
            -0x19E => cpu.d[0] = 0, // CloseLibrary
            -0xA8 => {
                // AddIntServer(D0 intnum, A1 Interrupt): a VERTB server goes in the list the level 3 handler runs, and
                // INTENA gets INTEN|VERTB so it runs (oshle.c; the C writes it only when not yet set, which SET makes
                // the same)
                if d[0] == 5 {
                    let mut e = VBL_SERVERS;
                    while e < VBL_SERVERS + 40 {
                        if mem.read_u32(e)? == 0 {
                            let code = mem.read_u32(a[1].wrapping_add(0x12))?;
                            let data = mem.read_u32(a[1].wrapping_add(0x0E))?;
                            mem.write_u32(e, code)?;
                            mem.write_u32(e + 8, 0)?;
                            mem.write_u32(e + 4, data)?;
                            break;
                        }
                        e += 8;
                    }
                    self.intena(0xC020);
                }
            }
            -0xAE => {
                // RemIntServer(D0 intnum, A1 Interrupt): close the list over the removed entry
                let c = mem.read_u32(a[1].wrapping_add(0x12))?;
                let mut e = VBL_SERVERS;
                while e < VBL_SERVERS + 40 && mem.read_u32(e)? != 0 && mem.read_u32(e)? != c {
                    e += 8;
                }
                while e < VBL_SERVERS + 40 && mem.read_u32(e)? != 0 {
                    let (n0, n1) = (mem.read_u32(e + 8)?, mem.read_u32(e + 12)?);
                    mem.write_u32(e, n0)?;
                    mem.write_u32(e + 4, n1)?;
                    e += 8;
                }
            }
            -0xA2 => cpu.d[0] = 0, // SetIntVector: no old handler
            -0x1F2 => cpu.d[0] = lib_base(&cstr(mem, a[1], 64)?), // OpenResource
            -0x270 | -0x276 => {
                // CopyMem, CopyMemQuick
                for j in 0..d[0] {
                    let b = mem.read_u8(a[0].wrapping_add(j))?;
                    mem.write_u8(a[1].wrapping_add(j), b)?;
                }
            }
            _ => return Ok(Done::No),
        }
        Ok(Done::Return)
    }

    /// oshle.c `dos_path` without assigns (no Assign has run in an M4 boot) and the last-part lookup of `md_file_ci`.
    fn lookup(&self, path: &[u8]) -> Option<usize> {
        let tail = last_part(path);
        if tail.is_empty() {
            return None;
        }
        self.files.find(tail)
    }

    /// Mirrors oshle.c `dos_call` for the services listed; anything else is `Done::No`.
    fn dos_call(&mut self, cpu: &mut Cpu, mem: &mut GuestMemory, lvo: i32) -> Result<Done, Stop> {
        let d = cpu.d;
        let fh = |h: u32| (h.wrapping_sub(FH0) as usize) < NFH;
        match lvo {
            -0x1E => {
                // Open(D1 name, D2 mode): the console names are never files
                let s = cstr(mem, d[1], 128)?;
                let low = s.to_ascii_lowercase();
                let con = s == b"*"
                    || [b"con:", b"raw:", b"nil:"]
                        .iter()
                        .any(|p| low.starts_with(*p));
                cpu.d[0] = if con { FH0 - 1 } else { 0 };
                if let Some(file) = if con { None } else { self.lookup(&s) } {
                    if let Some(i) = self.handles.iter().position(|h| h.is_none()) {
                        self.handles[i] = Some(Handle { file, pos: 0 });
                        cpu.d[0] = FH0 + i as u32;
                    }
                }
            }
            -0x24 => {
                // Close
                if fh(d[1]) {
                    self.handles[(d[1] - FH0) as usize] = None;
                }
                cpu.d[0] = 1;
            }
            -0x2A => {
                // Read(D1 fh, D2 buf, D3 len); the console answers "1" and Return (oshle.c, measured on MiG-29)
                if d[1] == FH0 - 1 && d[3] != 0 {
                    mem.write_u8(d[2], b'1')?;
                    cpu.d[0] = 1;
                    if d[3] > 1 {
                        mem.write_u8(d[2].wrapping_add(1), b'\n')?;
                        cpu.d[0] = 2;
                    }
                    return Ok(Done::Return);
                }
                let Some(h) = self.handle(d[1]) else {
                    cpu.d[0] = u32::MAX;
                    return Ok(Done::Return);
                };
                let data = self.files.data(h.file);
                let k = (data.len() - h.pos).min(d[3] as usize);
                mem.write_bytes(d[2], &data[h.pos..h.pos + k])?;
                if let Some(h) = self.handles[(d[1] - FH0) as usize].as_mut() {
                    h.pos += k;
                }
                cpu.d[0] = k as u32;
            }
            -0x30 => {
                // Write: the C logs it; kept here as console text
                for j in 0..d[3] {
                    self.console.push(mem.read_u8(d[2].wrapping_add(j))?);
                }
                cpu.d[0] = d[3];
            }
            -0x36 | -0x3C => cpu.d[0] = FH0 - 1, // Input, Output
            -0x42 => {
                // Seek(D1 fh, D2 offset, D3 mode): returns the old position
                let Some(h) = self.handle(d[1]) else {
                    cpu.d[0] = u32::MAX;
                    return Ok(Done::Return);
                };
                let n = self.files.data(h.file).len() as i64;
                let from = match d[3] as i32 {
                    -1 => 0,
                    0 => h.pos as i64,
                    _ => n,
                };
                let p = (d[2] as i32 as i64 + from).clamp(0, n);
                cpu.d[0] = h.pos as u32;
                if let Some(h) = self.handles[(d[1] - FH0) as usize].as_mut() {
                    h.pos = p as usize;
                }
            }
            -0x48 => cpu.d[0] = u32::MAX, // DeleteFile: done (nothing is changed)
            -0x54 => {
                // Lock(D1 name): a file's lock remembers it; anything else is the root directory
                let s = cstr(mem, d[1], 128)?;
                cpu.d[0] = ROOT_LOCK;
                if let Some(file) = self.lookup(&s) {
                    if let Some(i) = self.locks.iter().position(|l| l.is_none()) {
                        let tail = last_part(&s);
                        let name = tail[..tail.len().min(39)].to_vec();
                        self.locks[i] = Some(Lock { file, name });
                        cpu.d[0] = LK0 + i as u32;
                    }
                }
            }
            -0x5A => {
                // UnLock
                if let Some(l) = self.locks.get_mut(d[1].wrapping_sub(LK0) as usize) {
                    *l = None;
                }
            }
            -0x60 => {
                // DupLock
                cpu.d[0] = 0;
                if let Some(Some(l)) = self.locks.get(d[1].wrapping_sub(LK0) as usize).cloned() {
                    if let Some(j) = self.locks.iter().position(|l| l.is_none()) {
                        self.locks[j] = Some(l);
                        cpu.d[0] = LK0 + j as u32;
                    }
                }
            }
            -0x66 => self.examine(cpu, mem)?,
            -0x6C => {
                // ExNext: no more entries
                cpu.d[0] = 0;
                self.io_err = 232;
                mem.write_u32(TASK + 0x94, 232)?;
            }
            -0x72 => {
                // Info: a full DF0
                let v = [0, 0, 82, 1760, 1760, 512, 0x444F_5300];
                for (i, x) in v.iter().enumerate() {
                    mem.write_u32(d[2].wrapping_add(4 * i as u32), *x)?;
                }
                cpu.d[0] = u32::MAX;
            }
            -0x7E => cpu.d[0] = 0,           // CurrentDir
            -0x84 => cpu.d[0] = self.io_err, // IoErr
            -0x90 => {
                // Exit: no other process is queued in an M4 boot, so the run ends at EXIT_PC
                cpu.pc = EXIT_PC;
                return Ok(Done::Jumped);
            }
            -0xC0 => {
                // DateStamp: 1 Jan 1978
                mem.write_bytes(d[1], &[0; 12])?;
                cpu.d[0] = d[1];
            }
            -0xC6 => {}            // Delay: nothing else runs
            -0xCC => cpu.d[0] = 0, // WaitForChar: no key typed
            -0xD2 => {
                // ParentDir: a file's parent is the root; the root has none
                self.io_err = 0;
                cpu.d[0] = if (d[1].wrapping_sub(LK0) as usize) < NLOCK {
                    ROOT_LOCK
                } else {
                    0
                };
            }
            -0xD8 => cpu.d[0] = 0, // IsInteractive
            _ => return Ok(Done::No),
        }
        Ok(Done::Return)
    }

    fn handle(&self, h: u32) -> Option<Handle> {
        self.handles
            .get(h.wrapping_sub(FH0) as usize)
            .copied()
            .flatten()
    }

    /// Examine(D1 lock, D2 FileInfoBlock): a file, or the volume's root ("game", as the C names it).
    fn examine(&mut self, cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {
        let (lock, f) = (cpu.d[1], cpu.d[2]);
        let l = self
            .locks
            .get(lock.wrapping_sub(LK0) as usize)
            .cloned()
            .flatten();
        self.io_err = 0;
        mem.write_bytes(f, &[0; 260])?;
        let kind = if l.is_some() { (-3i32) as u32 } else { 2 };
        mem.write_u32(f + 4, kind)?;
        mem.write_u32(f + 0x78, kind)?;
        let name: &[u8] = l.as_ref().map_or(b"game", |l| &l.name);
        let nb = &name[..name.len().min(106)];
        mem.write_u8(f + 8, nb.len() as u8)?;
        mem.write_bytes(f + 9, nb)?;
        if let Some(l) = l {
            let size = self.files.data(l.file).len() as u32;
            mem.write_u32(f + 0x7C, size)?;
            mem.write_u32(f + 0x80, size.div_ceil(488))?;
        }
        cpu.d[0] = u32::MAX;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Done {
    /// Not served here.
    No,
    /// Served; return to the caller (RTS).
    Return,
    /// Served and the call set the PC itself.
    Jumped,
}

/// oshle.c `lib_open`: a fake library base, ciab.resource sharing ciaa's table, else 0.
fn lib_base(name: &[u8]) -> u32 {
    if let Some(&(_, b, _)) = LIBS.iter().find(|l| l.0.as_bytes() == name) {
        return b;
    }
    if name.eq_ignore_ascii_case(b"ciab.resource") {
        return 0x3C00;
    }
    0
}

/// The part of an AmigaDOS path after its last ':' or '/'.
fn last_part(p: &[u8]) -> &[u8] {
    p.rsplit(|&c| c == b':' || c == b'/').next().unwrap_or(&[])
}

/// oshle.c `cstr`: a NUL-terminated guest string of at most `max - 1` bytes.
pub(crate) fn cstr(mem: &GuestMemory, a: u32, max: u32) -> Result<Vec<u8>, Stop> {
    let mut s = Vec::new();
    if a == 0 {
        return Ok(s);
    }
    for i in 0..max - 1 {
        let c = mem.read_u8(a.wrapping_add(i))?;
        if c == 0 {
            break;
        }
        s.push(c);
    }
    Ok(s)
}

/// Runs a cold-booted program: translated code, with every call into a library jump table served by `os`, until
/// `pc == stop_pc` or `max_steps` steps (an OS call counts as one step, as in the C test driver).
pub fn run<F>(
    cpu: &mut Cpu,
    mem: &mut GuestMemory,
    os: &mut Os,
    stop_pc: u32,
    max_steps: u64,
    mut step: F,
) -> Result<u64, Stop>
where
    F: FnMut(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>,
{
    let mut steps = 0;
    while cpu.pc != stop_pc && steps < max_steps {
        if Os::slot(cpu.pc).is_some() {
            os.call(cpu, mem)?;
        } else {
            step(cpu, mem)?;
        }
        steps += 1;
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unserved_call_is_a_typed_stop_and_served_call_returns() {
        let mut mem = GuestMemory::new(&[(0, 0x10000)]).unwrap();
        let mut os = Os::new(
            GameFiles::default(),
            Pools {
                chip_lo: 0x8000,
                chip_hi: 0x10000,
                slow_lo: 0,
                slow_hi: 0,
            },
        );
        let mut cpu = Cpu::default();
        cpu.a[7] = 0x7000;
        mem.write_u32(0x7000, 0x1234).unwrap();
        cpu.pc = GFX - 0xDE; // graphics LoadView
        assert_eq!(
            os.call(&mut cpu, &mut mem),
            Err(Stop::OsCall {
                base: GFX,
                lvo: -0xDE
            })
        );
        cpu.pc = EXEC - 0xC6; // AllocMem(16, CHIP) with no slow RAM
        cpu.d[0] = 16;
        cpu.d[1] = MEMF_CHIP;
        os.call(&mut cpu, &mut mem).unwrap();
        assert_eq!((cpu.d[0], cpu.pc, cpu.a[7]), (0x8000, 0x1234, 0x7004));
        assert_eq!(mem.read_u32(0x3FE0).unwrap(), 0x8010);
    }
}
