//! AmigaDOS hunk loader, ported from `kit/amigo/hunkload.py` (`load`). The Python tool is the oracle: the
//! differential test in `tests/hunk_diff.rs` compares image, BASE and ENTRY byte for byte.
//!
//! Known divergences from the Python (it indexes and slices loosely; this port reports an error instead):
//! a reloc before the first hunk, a reloc target or offset out of range, and a hunk body longer than the
//! image the header declared.

use crate::mem::{GuestMemory, MemError};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HunkError {
    /// Python `ValueError` / `EOFError`: the file is not loadable (exit 1 in hunkload.py).
    Invalid(String),
    /// Python `NotImplementedError`: overlays, HUNK_EXT references (exit 2 in hunkload.py).
    Unsupported(String),
}

impl fmt::Display for HunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HunkError::Invalid(s) => write!(f, "invalid hunk file: {s}"),
            HunkError::Unsupported(s) => write!(f, "unsupported: {s}"),
        }
    }
}

impl std::error::Error for HunkError {}

fn bad<T>(s: impl Into<String>) -> Result<T, HunkError> {
    Err(HunkError::Invalid(s.into()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkKind {
    Code,
    Data,
    Bss,
    /// Declared in the header but no body seen.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemKind {
    Any,
    Chip,
    Fast,
    Ext,
}

#[derive(Debug, Clone)]
pub struct LoadOptions {
    /// Where non-chip hunks go. `None` = chip RAM if any hunk is chip, else `$C00000`.
    pub base: Option<u32>,
    pub align: u32,
    pub chip_base: u32,
    /// Put the 8-byte LoadSeg header (size, next BPTR) in front of every hunk.
    pub seglist: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions {
            base: None,
            align: 8,
            chip_base: 0x20000,
            seglist: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Loaded {
    /// Flat image starting at `base`; gaps between chip and non-chip hunks are zero.
    pub image: Vec<u8>,
    pub base: u32,
    pub entry: u32,
    pub addr: Vec<u32>,
    pub sizes: Vec<u32>,
    pub kinds: Vec<HunkKind>,
    pub mem: Vec<MemKind>,
    pub relocs: u32,
    /// BPTR of the first segment when `seglist` was requested.
    pub seglist: Option<u32>,
}

impl Loaded {
    /// Copy each hunk (and its LoadSeg header, when present) into guest memory. Hunks are written one at a time
    /// so a chip/slow layout does not need the zero-filled gap between them to be mapped.
    pub fn write_to(&self, mem: &mut GuestMemory, seglist: bool) -> Result<(), MemError> {
        for (&a, &s) in self.addr.iter().zip(&self.sizes) {
            let start = if seglist { a - 8 } else { a };
            let o = (start - self.base) as usize;
            let end = (a - self.base) as usize + s as usize;
            mem.write_bytes(start, &self.image[o..end])?;
        }
        Ok(())
    }
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}

impl Reader<'_> {
    fn eof<T>(&self) -> Result<T, HunkError> {
        bad(format!("unexpected end of file at offset {}", self.p))
    }

    fn l(&mut self) -> Result<u32, HunkError> {
        match self.b.get(self.p..self.p + 4) {
            Some(s) => {
                self.p += 4;
                Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
            }
            None => self.eof(),
        }
    }

    fn w(&mut self) -> Result<u16, HunkError> {
        match self.b.get(self.p..self.p + 2) {
            Some(s) => {
                self.p += 2;
                Ok(u16::from_be_bytes([s[0], s[1]]))
            }
            None => self.eof(),
        }
    }

    fn skip_longs(&mut self, n: u32) -> Result<(), HunkError> {
        let end = self.p as u64 + 4 * n as u64;
        if end > self.b.len() as u64 {
            return bad("hunk runs past end of file");
        }
        self.p = end as usize;
        Ok(())
    }
}

fn be32(image: &[u8], o: usize) -> Result<u32, HunkError> {
    match image.get(o..o + 4) {
        Some(s) => Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]])),
        None => bad(format!("relocation offset {o} outside the image")),
    }
}

fn put(image: &mut [u8], o: usize, bytes: &[u8]) -> Result<(), HunkError> {
    match image.get_mut(o..o + bytes.len()) {
        Some(d) => {
            d.copy_from_slice(bytes);
            Ok(())
        }
        None => bad(format!("relocation offset {o} outside the image")),
    }
}

/// Port of `hunkload.load` (checked against it by `tests/hunk_diff.rs`).
pub fn load(data: &[u8], opts: &LoadOptions) -> Result<Loaded, HunkError> {
    let mut r = Reader { b: data, p: 0 };
    if r.l()? != 0x3F3 {
        return bad("not a hunk executable (no HUNK_HEADER $3F3 at offset 0)");
    }
    loop {
        let n = r.l()?; // resident library names (normally none)
        if n == 0 {
            break;
        }
        r.skip_longs(n)?;
    }
    let _table = r.l()?;
    let first = r.l()? as i64;
    let last = r.l()? as i64;
    let count = last - first + 1;
    if count <= 0 || count > 4096 {
        return bad(format!("implausible hunk count {count}"));
    }
    let count = count as usize;
    let (mut sizes, mut mem) = (Vec::new(), Vec::new());
    for _ in 0..count {
        let v = r.l()?;
        let flags = v >> 30;
        if flags == 3 {
            r.l()?; // extended memory attributes
        }
        sizes.push((v & 0x3FFF_FFFF) * 4);
        mem.push(match flags {
            0 => MemKind::Any,
            1 => MemKind::Chip,
            2 => MemKind::Fast,
            _ => MemKind::Ext,
        });
    }
    if opts.align == 0 {
        return bad("align must be non-zero");
    }
    let chip_base = opts.chip_base as u64;
    let base = match opts.base {
        Some(b) => b as u64,
        None if mem.contains(&MemKind::Chip) => chip_base,
        None => 0xC0_0000,
    };
    let align = opts.align as u64;
    let hdr: u64 = if opts.seglist { 8 } else { 0 };
    let (mut nxt_chip, mut nxt_other) = (chip_base, base);
    let mut addr: Vec<u64> = Vec::new();
    for (&s, &m) in sizes.iter().zip(&mem) {
        let chip = m == MemKind::Chip || base == chip_base;
        let nxt = if chip { &mut nxt_chip } else { &mut nxt_other };
        let a = nxt.div_ceil(align) * align + hdr;
        addr.push(a);
        *nxt = a + s as u64;
    }
    let lo = addr.iter().min().copied().unwrap() - hdr;
    let hi = addr
        .iter()
        .zip(&sizes)
        .map(|(&a, &s)| a + s as u64)
        .max()
        .unwrap();
    if hi - lo > 16 << 20 {
        return bad(format!(
            "image would span {} MB; use --base close to --chip-base",
            (hi - lo) >> 20
        ));
    }
    if hi > u32::MAX as u64 {
        return bad("image runs past 32-bit address space");
    }
    let base = lo;
    let mut image = vec![0u8; (hi - lo) as usize];
    let mut kinds = vec![HunkKind::Unknown; count];
    let mut relocs = 0u32;
    let mut h: i64 = -1;
    // Start of the current hunk inside `image`.
    let cur = |h: i64| -> Result<usize, HunkError> {
        if h < 0 {
            return bad("relocation before the first hunk");
        }
        Ok((addr[h as usize] - base) as usize)
    };
    let target_addr = |t: u32| -> Result<u64, HunkError> {
        match addr.get(t as usize) {
            Some(&a) => Ok(a),
            None => bad(format!("relocation target hunk {t} out of range")),
        }
    };
    while r.p < data.len() {
        if data.len() - r.p < 4 {
            break;
        }
        let t = r.l()? & 0x3FFF_FFFF;
        match t {
            0x3E9..=0x3EB => {
                h += 1;
                if h as usize >= count {
                    return bad("more hunks in the file than the header declares");
                }
                let n = r.l()? & 0x3FFF_FFFF;
                kinds[h as usize] = match t {
                    0x3E9 => HunkKind::Code,
                    0x3EA => HunkKind::Data,
                    _ => HunkKind::Bss,
                };
                if t != 0x3EB {
                    let end = (r.p + 4 * n as usize).min(data.len());
                    let body = &data[r.p..end];
                    r.skip_longs(n)?;
                    put(&mut image, cur(h)?, body)?;
                }
            }
            0x3EC => loop {
                let n = r.l()?;
                if n == 0 {
                    break;
                }
                let ta = target_addr(r.l()?)?;
                for _ in 0..n {
                    let o = cur(h)? + r.l()? as usize;
                    let v = be32(&image, o)?;
                    put(&mut image, o, &v.wrapping_add(ta as u32).to_be_bytes())?;
                    relocs += 1;
                }
            },
            0x3FC | 0x3FD => {
                let start = r.p;
                loop {
                    let n = r.w()?;
                    if n == 0 {
                        break;
                    }
                    let ta = target_addr(r.w()? as u32)?;
                    for _ in 0..n {
                        let off = r.w()? as usize;
                        let o = cur(h)? + off;
                        let mut v = be32(&image, o)?;
                        if t == 0x3FC {
                            v = v.wrapping_add(ta as u32);
                        } else {
                            // PC-relative between hunks
                            let here = addr[h as usize] + off as u64;
                            v = v.wrapping_add(ta.wrapping_sub(here) as u32);
                        }
                        put(&mut image, o, &v.to_be_bytes())?;
                        relocs += 1;
                    }
                }
                if !(r.p - start).is_multiple_of(4) {
                    r.p += 2; // padded to a long
                }
            }
            0x3ED | 0x3EE => {
                let wide = t == 0x3ED;
                loop {
                    let n = r.l()?;
                    if n == 0 {
                        break;
                    }
                    let ta = target_addr(r.l()?)?;
                    for _ in 0..n {
                        let off = r.l()? as u64;
                        let o = cur(h)? + off as usize;
                        let here = addr[h as usize] + off;
                        let delta = ta.wrapping_sub(here) as u32;
                        if wide {
                            let s = match image.get(o..o + 2) {
                                Some(s) => i16::from_be_bytes([s[0], s[1]]) as i32,
                                None => return bad("relocation offset outside the image"),
                            };
                            let v = s.wrapping_add(delta as i32) as u16;
                            put(&mut image, o, &v.to_be_bytes())?;
                        } else {
                            let s = match image.get(o) {
                                Some(&s) => s as i8 as i32,
                                None => return bad("relocation offset outside the image"),
                            };
                            let v = s.wrapping_add(delta as i32) as u8;
                            put(&mut image, o, &[v])?;
                        }
                        relocs += 1;
                    }
                }
            }
            0x3F0 => loop {
                let n = r.l()?;
                if n == 0 {
                    break;
                }
                r.skip_longs(n.saturating_add(1))?;
            },
            0x3F1 | 0x3E8 | 0x3E7 => {
                let n = r.l()?;
                r.skip_longs(n)?;
            }
            0x3EF => loop {
                let v = r.l()?;
                if v == 0 {
                    break;
                }
                let (typ, n) = (v >> 24, v & 0xFF_FFFF);
                r.skip_longs(n)?;
                match typ {
                    1..=3 => {
                        r.l()?; // definitions: one value
                    }
                    130 => {
                        r.l()?; // common: size then refs
                        let k = r.l()?;
                        r.skip_longs(k)?;
                    }
                    _ => {
                        return Err(HunkError::Unsupported(format!(
                            "HUNK_EXT reference type {typ} needs a linker"
                        )))
                    }
                }
            },
            0x3F2 => {}
            0x3F5 | 0x3F6 => {
                return Err(HunkError::Unsupported(
                    "overlay executables are not supported".into(),
                ))
            }
            0 if data[r.p..].iter().all(|&b| b == 0) => break, // zero padding at end of file
            _ => {
                return bad(format!(
                    "unknown hunk type ${:X} at file offset {}",
                    t,
                    r.p - 4
                ))
            }
        }
    }
    let mut seglist = None;
    if opts.seglist {
        // The LoadSeg headers: allocation size, then the BPTR to the next segment (0 for the last).
        for i in 0..count {
            let nxt_hdr = if i + 1 < count {
                ((addr[i + 1] - 4) >> 2) as u32
            } else {
                0
            };
            let o = (addr[i] - 8 - base) as usize;
            put(&mut image, o, &(sizes[i] + 8).to_be_bytes())?;
            put(&mut image, o + 4, &nxt_hdr.to_be_bytes())?;
        }
        seglist = Some(((addr[0] - 4) >> 2) as u32);
    }
    Ok(Loaded {
        image,
        base: base as u32,
        entry: addr[0] as u32,
        addr: addr.iter().map(|&a| a as u32).collect(),
        sizes,
        kinds,
        mem,
        relocs,
        seglist,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn long(v: &mut Vec<u8>, x: u32) {
        v.extend_from_slice(&x.to_be_bytes());
    }

    /// One code hunk: moveq #0,d0 / rts.
    fn tiny() -> Vec<u8> {
        let mut v = Vec::new();
        for x in [0x3F3, 0, 1, 0, 0, 1, 0x3E9, 1, 0x7000_4E75, 0x3F2] {
            long(&mut v, x);
        }
        v
    }

    #[test]
    fn loads_one_code_hunk_at_default_base() {
        let l = load(&tiny(), &LoadOptions::default()).unwrap();
        assert_eq!((l.base, l.entry), (0xC0_0000, 0xC0_0000));
        assert_eq!(l.image, [0x70, 0x00, 0x4E, 0x75]);
        assert_eq!(l.kinds, [HunkKind::Code]);
    }

    #[test]
    fn writes_into_guest_memory() {
        let l = load(&tiny(), &LoadOptions::default()).unwrap();
        let mut m = GuestMemory::a500(0x1000, 0x1000).unwrap();
        l.write_to(&mut m, false).unwrap();
        assert_eq!(m.read_u32(l.entry), Ok(0x7000_4E75));
    }

    #[test]
    fn rejects_bad_magic_and_truncation() {
        assert!(matches!(
            load(&[0, 0, 0, 1], &LoadOptions::default()),
            Err(HunkError::Invalid(_))
        ));
        let t = tiny();
        assert!(load(&t[..30], &LoadOptions::default()).is_err());
    }

    #[test]
    fn overlay_is_unsupported() {
        let mut v = tiny();
        v.truncate(v.len() - 4);
        long(&mut v, 0x3F5);
        assert!(matches!(
            load(&v, &LoadOptions::default()),
            Err(HunkError::Unsupported(_))
        ));
    }
}
