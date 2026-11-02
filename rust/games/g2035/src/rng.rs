//! The game's random number routine ($C0DFAC), exactly: an 8-bit seed stepped as seed * 5 + $81, XORed with a byte
//! of a 1024-byte table indexed by (frame counter + number of calls) & $3FF. The original's table is its own code
//! at $C0F2B6; the port carries those bytes in the asset pack (`match_tables.rng`), not in the repo.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rng {
    /// The seed byte ($C0DFF6).
    pub seed: u8,
    /// Calls so far ($C0DFF4).
    pub calls: u16,
}

impl Rng {
    /// The next number, 0 to 255. `frame` is the frame counter ($C0DE6A).
    pub fn next(&mut self, frame: u16, table: &[u8]) -> u8 {
        let s = self.seed;
        self.seed = (s << 2).wrapping_add(0x81).wrapping_add(s);
        let i = (frame.wrapping_add(self.calls) & 0x3FF) as usize;
        self.calls = self.calls.wrapping_add(1);
        self.seed ^ table.get(i).copied().unwrap_or(0)
    }
}
