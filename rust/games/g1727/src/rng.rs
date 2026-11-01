//! The game's random number generator (+5442) with its six-byte state (+5458, "Champ)" before seeding; the start-up
//! code seeds it from the display beam position). A step reads the long at state+2, adds the long at state+0, stores
//! the sum at state+0, swaps its halves and adds the game's frame counter word; callers mask the word they get.

#[derive(Debug, Clone)]
pub struct Rng {
    s: [u8; 6],
    /// The frame counter the original adds to each result.
    pub frames: u16,
}

impl Rng {
    /// `seed` goes where the start-up code stores the beam position (the first word of the state).
    pub fn new(seed: u32) -> Rng {
        let mut s = *b"Champ)";
        s[0] = (seed >> 8) as u8;
        s[1] = seed as u8;
        Rng { s, frames: 0 }
    }

    fn long(&self, o: usize) -> u32 {
        u32::from_be_bytes([self.s[o], self.s[o + 1], self.s[o + 2], self.s[o + 3]])
    }

    pub fn step(&mut self) -> u16 {
        let d = self.long(2).wrapping_add(self.long(0));
        self.s[..4].copy_from_slice(&d.to_be_bytes());
        let swapped = d.rotate_left(16);
        (swapped as u16).wrapping_add(self.frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_like_the_original() {
        let mut r = Rng::new(0x4368);
        // state "Champ)": long(2) = "amp)", long(0) = "Cham"
        let d = u32::from_be_bytes(*b"amp)").wrapping_add(u32::from_be_bytes(*b"Cham"));
        assert_eq!(r.step(), (d >> 16) as u16);
        assert_eq!(&r.s[..4], &d.to_be_bytes());
    }
}
