//! The playfield: the claim map of the original (a one-bit map, 40 bytes a row, data hunk 1 +0028) written as typed
//! cells. Coordinates are the original's map coordinates, which are also its screen coordinates: the border runs
//! along x = 0 and x = 286, y = 0 and y = 198 (measured from the reference state log and frames), so the inside is
//! 285 x 197 = 56,145 pixels. The round picture sits at (0, 1).

/// Width and height of the map, border included.
pub const FW: i32 = 287;
pub const FH: i32 = 199;
/// Pixels inside the border: what the percentage counts against.
pub const INSIDE: u32 = 285 * 197;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Not claimed: the round's surface shows here and the enemies move here.
    Free,
    /// The edge of the claimed area (the border and the lines of closed cuts): the player walks on it.
    Line,
    /// Claimed and enclosed: the picture shows here.
    Claimed,
    /// The open cut (the red line, colour 15 in the original).
    Cut,
}

#[derive(Debug, Clone)]
pub struct Field {
    cells: Vec<Cell>,
    /// The original's claimed-pixel count (player record +30): inside pixels that are not free.
    pub count: u32,
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl Field {
    /// A round's empty field: the border is the only line (round set-up +5BB2).
    pub fn new() -> Field {
        let mut cells = vec![Cell::Free; (FW * FH) as usize];
        for y in 0..FH {
            for x in 0..FW {
                if x == 0 || y == 0 || x == FW - 1 || y == FH - 1 {
                    cells[(y * FW + x) as usize] = Cell::Line;
                }
            }
        }
        Field { cells, count: 0 }
    }

    pub fn inside(x: i32, y: i32) -> bool {
        (0..FW).contains(&x) && (0..FH).contains(&y)
    }

    /// The cell at (x, y); outside the map counts as claimed.
    pub fn get(&self, x: i32, y: i32) -> Cell {
        if Self::inside(x, y) {
            self.cells[(y * FW + x) as usize]
        } else {
            Cell::Claimed
        }
    }

    pub fn set(&mut self, x: i32, y: i32, c: Cell) {
        if Self::inside(x, y) {
            self.cells[(y * FW + x) as usize] = c;
        }
    }

    /// A line cell the player may walk on: it still touches free ground (the original tests the neighbouring map
    /// bits and the display planes for this, +6508 .. +6518).
    pub fn walkable(&self, x: i32, y: i32) -> bool {
        self.get(x, y) == Cell::Line && self.touches_free(x, y)
    }

    fn touches_free(&self, x: i32, y: i32) -> bool {
        (-1..=1).any(|dy| (-1..=1).any(|dx| self.get(x + dx, y + dy) == Cell::Free))
    }

    /// Closes a cut (+6608 then the fill +68DE): the cut becomes line, every free pixel that the boss cannot reach
    /// is claimed, and lines left with no free ground beside them become claimed. Returns the pixels gained.
    pub fn close(&mut self, keep: (i32, i32)) -> u32 {
        for c in self.cells.iter_mut() {
            if *c == Cell::Cut {
                *c = Cell::Line;
            }
        }
        let mut reach = vec![false; self.cells.len()];
        if let Some(seed) = self.nearest_free(keep) {
            let mut stack = vec![seed];
            reach[(seed.1 * FW + seed.0) as usize] = true;
            while let Some((x, y)) = stack.pop() {
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if self.get(nx, ny) == Cell::Free && !reach[(ny * FW + nx) as usize] {
                        reach[(ny * FW + nx) as usize] = true;
                        stack.push((nx, ny));
                    }
                }
            }
        }
        for (i, c) in self.cells.iter_mut().enumerate() {
            if *c == Cell::Free && !reach[i] {
                *c = Cell::Claimed;
            }
        }
        let lines: Vec<(i32, i32)> = (0..FH)
            .flat_map(|y| (0..FW).map(move |x| (x, y)))
            .filter(|&(x, y)| self.get(x, y) == Cell::Line && !self.touches_free(x, y))
            .collect();
        for (x, y) in lines {
            self.set(x, y, Cell::Claimed);
        }
        let before = self.count;
        self.recount();
        self.count - before
    }

    /// Removes the open cut (a life lost while cutting).
    pub fn clear_cut(&mut self) {
        for c in self.cells.iter_mut() {
            if *c == Cell::Cut {
                *c = Cell::Free;
            }
        }
        self.recount();
    }

    fn nearest_free(&self, (kx, ky): (i32, i32)) -> Option<(i32, i32)> {
        for r in 0..FW.max(FH) {
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx.abs() != r && dy.abs() != r {
                        continue;
                    }
                    if self.get(kx + dx, ky + dy) == Cell::Free {
                        return Some((kx + dx, ky + dy));
                    }
                }
            }
        }
        None
    }

    pub fn recount(&mut self) {
        let mut n = 0;
        for y in 1..FH - 1 {
            for x in 1..FW - 1 {
                if self.cells[(y * FW + x) as usize] != Cell::Free {
                    n += 1;
                }
            }
        }
        self.count = n;
    }

    /// The percentage as the HUD shows it, three digits "TU.t" (+7D54): ((count + 32) >> 6) * 747, capped at
    /// $9FD70, plus $100, read as a 16.16 fraction of ten. The tens digit reaching 8 ends the round (+7D90).
    pub fn percent_digits(&self) -> [u8; 3] {
        let mut v: u32 = ((self.count + 32) >> 6) * 0x2EB;
        v = v.min(0x9FD70) + 0x100;
        let t = (v >> 16) as u8;
        v = (v & 0xFFFF) * 10;
        let u = (v >> 16) as u8;
        v = (v & 0xFFFF) * 10;
        [t, u, (v >> 16) as u8]
    }

    pub fn cleared(&self) -> bool {
        self.percent_digits()[0] >= 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_field_is_zero_percent() {
        let f = Field::new();
        assert_eq!(f.count, 0);
        assert_eq!(f.percent_digits(), [0, 0, 0]);
        assert!(f.walkable(142, 198));
        assert!(!f.walkable(142, 100));
    }

    #[test]
    fn closing_a_cut_claims_the_side_without_the_boss() {
        let mut f = Field::new();
        for y in 1..198 {
            f.set(100, y, Cell::Cut);
        }
        f.close((200, 100));
        assert_eq!(f.get(50, 50), Cell::Claimed);
        assert_eq!(f.get(200, 100), Cell::Free);
        assert!(f.walkable(100, 50));
        // 99 columns claimed plus the line itself
        assert_eq!(f.count, 100 * 197);
        assert_eq!(f.percent_digits(), [3, 5, 1]);
    }

    #[test]
    fn eighty_percent_clears() {
        let mut f = Field::new();
        for y in 1..198 {
            f.set(229, y, Cell::Cut);
        }
        f.close((250, 100));
        assert!(f.cleared());
    }
}
