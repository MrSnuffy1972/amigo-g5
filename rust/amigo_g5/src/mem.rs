//! Guest memory: owned, bounds-checked storage with explicit big-endian accessors.
//!
//! Decision 28 Oct 2026: addresses are masked to 24 bits, as on the 68000 address bus. An odd address on a word
//! or long access is a typed error (the 68000 address error). A single access must lie inside one region.
//!
//! Decision 28 Oct 2026 (M3): an optional write journal records the old value of every byte written, so a
//! differential test (and later the per-tick oracle check) can list the bytes a step touched and roll them back
//! without copying the whole of guest memory. It is off unless asked for and changes no access result.

use std::fmt;

/// The 68000 drives 24 address lines.
pub const ADDR_MASK: u32 = 0x00FF_FFFF;
/// Where slow (ranger) RAM sits on an A500 with a trapdoor expansion.
pub const SLOW_BASE: u32 = 0x00C0_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemError {
    /// Word or long access at an odd address (68000 address error).
    OddAddress { addr: u32, size: u8 },
    /// The access is not wholly inside one mapped region.
    Unmapped { addr: u32, len: usize },
}

impl fmt::Display for MemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemError::OddAddress { addr, size } => {
                write!(
                    f,
                    "address error: {size}-byte access at odd address ${addr:08X}"
                )
            }
            MemError::Unmapped { addr, len } => {
                write!(f, "unmapped access: {len} bytes at ${addr:08X}")
            }
        }
    }
}

impl std::error::Error for MemError {}

/// A bad memory map given to [`GuestMemory::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapError {
    Empty,
    /// A region is empty, or runs past the 24-bit address space.
    BadRegion {
        base: u32,
        size: usize,
    },
    Overlap {
        a: u32,
        b: u32,
    },
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapError::Empty => write!(f, "memory map has no regions"),
            MapError::BadRegion { base, size } => {
                write!(f, "bad region: base ${base:08X}, size {size}")
            }
            MapError::Overlap { a, b } => write!(f, "regions at ${a:08X} and ${b:08X} overlap"),
        }
    }
}

impl std::error::Error for MapError {}

#[derive(Debug, Clone)]
struct Region {
    base: u32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct GuestMemory {
    regions: Vec<Region>,
    /// `(masked address, old byte)` for every byte written while journalling, oldest first.
    journal: Option<Vec<(u32, u8)>>,
}

impl GuestMemory {
    /// Build from a map of `(base, size)` pairs. All bytes start as zero.
    pub fn new(map: &[(u32, usize)]) -> Result<Self, MapError> {
        if map.is_empty() {
            return Err(MapError::Empty);
        }
        let mut regions: Vec<Region> = Vec::new();
        for &(base, size) in map {
            if size == 0 || base > ADDR_MASK || base as u64 + size as u64 > ADDR_MASK as u64 + 1 {
                return Err(MapError::BadRegion { base, size });
            }
            regions.push(Region {
                base,
                data: vec![0; size],
            });
        }
        regions.sort_by_key(|r| r.base);
        for w in regions.windows(2) {
            if w[0].base as u64 + w[0].data.len() as u64 > w[1].base as u64 {
                return Err(MapError::Overlap {
                    a: w[0].base,
                    b: w[1].base,
                });
            }
        }
        Ok(GuestMemory {
            regions,
            journal: None,
        })
    }

    /// Chip RAM at 0, plus slow RAM at `SLOW_BASE` when `slow_size` is non-zero.
    pub fn a500(chip_size: usize, slow_size: usize) -> Result<Self, MapError> {
        let mut map = vec![(0, chip_size)];
        if slow_size > 0 {
            map.push((SLOW_BASE, slow_size));
        }
        Self::new(&map)
    }

    fn locate(&self, addr: u32, len: usize) -> Result<(usize, usize), MemError> {
        let a = (addr & ADDR_MASK) as u64;
        let err = MemError::Unmapped { addr, len };
        for (i, r) in self.regions.iter().enumerate() {
            let base = r.base as u64;
            if a >= base && a < base + r.data.len() as u64 {
                let off = (a - base) as usize;
                return if off + len <= r.data.len() {
                    Ok((i, off))
                } else {
                    Err(err)
                };
            }
        }
        Err(err)
    }

    fn check_even(addr: u32, size: u8) -> Result<(), MemError> {
        if addr & 1 != 0 {
            Err(MemError::OddAddress { addr, size })
        } else {
            Ok(())
        }
    }

    pub fn read_bytes(&self, addr: u32, len: usize) -> Result<&[u8], MemError> {
        let (i, off) = self.locate(addr, len)?;
        Ok(&self.regions[i].data[off..off + len])
    }

    /// Start (or restart) recording writes. Any earlier record is dropped, not rolled back.
    pub fn start_journal(&mut self) {
        self.journal = Some(Vec::new());
    }

    /// The bytes written since [`start_journal`](Self::start_journal), as `(address, old byte)`, oldest first.
    /// An address appears once per write, so a byte written twice is listed twice.
    pub fn journal(&self) -> &[(u32, u8)] {
        self.journal.as_deref().unwrap_or(&[])
    }

    /// Put back every journalled byte (newest first) and clear the record. Journalling stays on.
    pub fn rollback(&mut self) {
        let Some(log) = self.journal.take() else {
            return;
        };
        for &(a, old) in log.iter().rev() {
            if let Ok((i, off)) = self.locate(a, 1) {
                self.regions[i].data[off] = old;
            }
        }
        self.journal = Some(Vec::new());
    }

    pub fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), MemError> {
        let (i, off) = self.locate(addr, bytes.len())?;
        if let Some(log) = self.journal.as_mut() {
            let data = &self.regions[i].data;
            let first = addr & ADDR_MASK;
            for k in 0..bytes.len() {
                log.push((first + k as u32, data[off + k]));
            }
        }
        self.regions[i].data[off..off + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8, MemError> {
        Ok(self.read_bytes(addr, 1)?[0])
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, MemError> {
        Self::check_even(addr, 2)?;
        let b = self.read_bytes(addr, 2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, MemError> {
        Self::check_even(addr, 4)?;
        let b = self.read_bytes(addr, 4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn write_u8(&mut self, addr: u32, v: u8) -> Result<(), MemError> {
        self.write_bytes(addr, &[v])
    }

    pub fn write_u16(&mut self, addr: u32, v: u16) -> Result<(), MemError> {
        Self::check_even(addr, 2)?;
        self.write_bytes(addr, &v.to_be_bytes())
    }

    pub fn write_u32(&mut self, addr: u32, v: u32) -> Result<(), MemError> {
        Self::check_even(addr, 4)?;
        self.write_bytes(addr, &v.to_be_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> GuestMemory {
        GuestMemory::a500(0x1000, 0x100).unwrap()
    }

    #[test]
    fn byte_roundtrip_and_zero_init() {
        let mut m = mem();
        assert_eq!(m.read_u8(5), Ok(0));
        m.write_u8(5, 0xAB).unwrap();
        assert_eq!(m.read_u8(5), Ok(0xAB));
    }

    #[test]
    fn word_and_long_are_big_endian() {
        let mut m = mem();
        m.write_u32(0x10, 0x1122_3344).unwrap();
        assert_eq!(m.read_bytes(0x10, 4).unwrap(), &[0x11, 0x22, 0x33, 0x44]);
        assert_eq!(m.read_u16(0x10), Ok(0x1122));
        assert_eq!(m.read_u16(0x12), Ok(0x3344));
        m.write_u16(0x20, 0xBEEF).unwrap();
        assert_eq!(m.read_u8(0x20), Ok(0xBE));
        assert_eq!(m.read_u8(0x21), Ok(0xEF));
        assert_eq!(m.read_u32(0x10), Ok(0x1122_3344));
    }

    #[test]
    fn odd_word_and_long_are_address_errors() {
        let mut m = mem();
        assert_eq!(
            m.read_u16(1),
            Err(MemError::OddAddress { addr: 1, size: 2 })
        );
        assert_eq!(
            m.read_u32(3),
            Err(MemError::OddAddress { addr: 3, size: 4 })
        );
        assert_eq!(
            m.write_u16(7, 1),
            Err(MemError::OddAddress { addr: 7, size: 2 })
        );
        assert_eq!(
            m.write_u32(9, 1),
            Err(MemError::OddAddress { addr: 9, size: 4 })
        );
        // Byte access at an odd address is fine.
        assert!(m.write_u8(9, 1).is_ok());
    }

    #[test]
    fn boundaries_of_a_region() {
        let mut m = mem();
        assert!(m.write_u8(0xFFF, 1).is_ok());
        assert!(m.read_u8(0x1000).is_err());
        assert!(m.write_u32(0xFFC, 7).is_ok());
        assert!(m.write_u32(0xFFE, 7).is_err());
        assert!(m.write_u16(0xFFE, 7).is_ok());
        assert_eq!(
            m.read_u16(0x1000),
            Err(MemError::Unmapped {
                addr: 0x1000,
                len: 2
            })
        );
        // An access that starts inside and runs out does not write anything.
        let mut m2 = mem();
        assert!(m2.write_bytes(0xFFE, &[1, 2, 3, 4]).is_err());
        assert_eq!(m2.read_u16(0xFFE), Ok(0));
    }

    #[test]
    fn slow_ram_and_gap() {
        let mut m = mem();
        m.write_u32(SLOW_BASE, 0xCAFE_F00D).unwrap();
        assert_eq!(m.read_u32(SLOW_BASE), Ok(0xCAFE_F00D));
        assert!(m.write_u8(SLOW_BASE + 0x100, 1).is_err());
        assert!(m.read_u8(0x80_0000).is_err());
        assert!(m.read_u8(SLOW_BASE - 1).is_err());
    }

    #[test]
    fn address_is_masked_to_24_bits() {
        let mut m = mem();
        m.write_u16(0x100, 0x5555).unwrap();
        assert_eq!(m.read_u16(0xFF00_0100), Ok(0x5555));
    }

    #[test]
    fn no_slow_ram_by_default() {
        let m = GuestMemory::a500(0x1000, 0).unwrap();
        assert!(m.read_u8(SLOW_BASE).is_err());
    }

    #[test]
    fn map_validation() {
        assert_eq!(GuestMemory::new(&[]).unwrap_err(), MapError::Empty);
        assert!(matches!(
            GuestMemory::new(&[(0, 0)]),
            Err(MapError::BadRegion { .. })
        ));
        assert!(matches!(
            GuestMemory::new(&[(0xFF_FFF0, 0x100)]),
            Err(MapError::BadRegion { .. })
        ));
        assert!(matches!(
            GuestMemory::new(&[(0, 0x100), (0x80, 0x100)]),
            Err(MapError::Overlap { .. })
        ));
        // Order of the map does not matter; adjacent regions are fine.
        let m = GuestMemory::new(&[(0x100, 0x100), (0, 0x100)]).unwrap();
        assert!(m.read_u8(0xFF).is_ok() && m.read_u8(0x100).is_ok());
        // A single access cannot straddle two adjacent regions.
        assert!(m.read_u16(0xFE).is_ok());
        assert!(m.read_bytes(0xFF, 2).is_err());
    }

    #[test]
    fn journal_lists_old_bytes_and_rolls_back() {
        let mut m = mem();
        m.write_u16(0x10, 0x1234).unwrap();
        assert!(m.journal().is_empty(), "off by default");
        m.start_journal();
        m.write_u16(0xFF00_0010, 0xABCD).unwrap();
        m.write_u8(0x11, 0xEE).unwrap();
        assert!(
            m.write_u32(0x13, 1).is_err(),
            "a failed access writes nothing"
        );
        assert_eq!(m.journal(), &[(0x10, 0x12), (0x11, 0x34), (0x11, 0xCD)]);
        assert_eq!(m.read_u16(0x10), Ok(0xABEE));
        m.rollback();
        assert_eq!(m.read_u16(0x10), Ok(0x1234));
        assert!(m.journal().is_empty());
        m.write_u8(SLOW_BASE, 7).unwrap();
        assert_eq!(m.journal(), &[(SLOW_BASE, 0)]);
    }

    #[test]
    fn memory_is_cloneable_for_snapshots() {
        let mut a = mem();
        a.write_u8(0, 9).unwrap();
        let b = a.clone();
        a.write_u8(0, 1).unwrap();
        assert_eq!(b.read_u8(0), Ok(9));
    }
}
