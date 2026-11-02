//! Part 3, first slice: the match arena and its scrolling.
//!
//! The original keeps the arena as a map of 40 by 72 cells of 16 x 16 pixels (640 by 1,152) at `$27A`, one word a
//! cell: 0 is the plain blank cell (colour 17), a positive number is a tile of disk file 26 (five planes of 16 rows,
//! 160 bytes a tile; tile 1 is the file's first) and a negative number the same tile turned upside down (the blitter
//! reads it bottom row first). At start-up every cell holding `$A2` is cleared to 0 (the start-up routine's loop over
//! `$27A`). `configure_team_sprite_layout` draws the 21 by 13 cells under the view into the screen buffer it is about
//! to show (only the cells its dirty map at `$19EA` marks for that buffer), `toggle_copper_blitter_wait` filling a blank
//! cell, `execute_conditional_blit_loop` copying a tile and `wait_and_update_sprite_state` copying one upside down; the
//! display list then starts the planes at the camera's row and pixel. Natively the view is drawn straight from the map
//! each frame: no buffers, no dirty map, no copper.
//!
//! The camera (`$18E`, `$190`: the arena pixel at the view's top left) starts at (160, 484) (`initialize_game_state`
//! and the match set-up), and each frame `clamp_player_position` moves it towards the followed object: the target is
//! the object less (160, 92), the step an eighth of the distance plus one, at most 16 and never past the target, and
//! the camera stays inside 0..=320 across and 0..=968 down. The view is 320 by 184 (`$192` = `$B8`); the 16-row status
//! bar under it is a later slice.

use native_engine::gfx::{Bitmap, Sheet};
use native_engine::pack::Pack;

pub const MAP_W: usize = 40;
pub const MAP_H: usize = 72;
pub const CELL: usize = 16;
pub const ARENA_W: i32 = (MAP_W * CELL) as i32;
pub const ARENA_H: i32 = (MAP_H * CELL) as i32;
pub const VIEW_W: usize = 320;
pub const VIEW_H: usize = 184;
/// The colour a blank cell is filled with (planes 0 and 4 set).
pub const BLANK: u8 = 17;
/// The cell value the start-up clears to blank.
const CLEARED_AT_START: i16 = 0xA2;

pub struct Arena {
    cells: Vec<i16>,
    tiles: Sheet,
    /// The match's 32 colours (12-bit, as the colour registers take them), from the program's table at `$F2`.
    pub pal: [u16; 32],
}

impl Arena {
    pub fn new(mut cells: Vec<i16>, tiles: Sheet, pal: [u16; 32]) -> Result<Arena, String> {
        if cells.len() != MAP_W * MAP_H {
            return Err(format!("arena_map: want {} cells", MAP_W * MAP_H));
        }
        if tiles.frame_w != CELL || tiles.frame_h != CELL {
            return Err("arena_tiles: want tiles of 16 x 16".into());
        }
        for c in cells.iter_mut().filter(|c| **c == CLEARED_AT_START) {
            *c = 0;
        }
        let top = cells
            .iter()
            .map(|c| c.unsigned_abs() as usize)
            .max()
            .unwrap_or(0);
        if top > tiles.frames {
            return Err(format!(
                "arena_map: tile {top} but only {} tiles",
                tiles.frames
            ));
        }
        Ok(Arena { cells, tiles, pal })
    }

    pub fn load(p: &Pack) -> Result<Arena, String> {
        let m = p.data("arena_map")?;
        let cells = m["cells"]
            .as_array()
            .ok_or("arena_map: no cells")?
            .iter()
            .map(|v| {
                v.as_i64()
                    .map(|v| v as i16)
                    .ok_or("arena_map: bad cell".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut pal = [0u16; 32];
        for (o, c) in pal.iter_mut().zip(&p.palette("match_pal")?.colours) {
            let n = |s: u32| ((c >> s) & 0xFF) as u16 / 17;
            *o = n(16) << 8 | n(8) << 4 | n(0);
        }
        Arena::new(cells, p.sheet("arena_tiles")?, pal)
    }

    /// The cell value at map column `cx`, row `cy` (0 outside the map).
    pub fn cell(&self, cx: i32, cy: i32) -> i16 {
        if cx < 0 || cy < 0 || cx >= MAP_W as i32 || cy >= MAP_H as i32 {
            return 0;
        }
        self.cells[cy as usize * MAP_W + cx as usize]
    }

    /// The colour of arena pixel (`x`, `y`).
    pub fn pixel(&self, x: i32, y: i32) -> u8 {
        let c = self.cell(x.div_euclid(CELL as i32), y.div_euclid(CELL as i32));
        if c == 0 {
            return BLANK;
        }
        let (tx, mut ty) = (x.rem_euclid(CELL as i32), y.rem_euclid(CELL as i32));
        if c < 0 {
            ty = CELL as i32 - 1 - ty;
        }
        let f = (c.unsigned_abs() as i32 - 1) * CELL as i32;
        self.tiles.image.get(f + tx, ty)
    }

    /// Draws the view at `cam` into rows `dy..dy + 184` of `dst`.
    pub fn draw_view(&self, dst: &mut Bitmap, cam: Camera, dy: i32) {
        for y in 0..VIEW_H as i32 {
            for x in 0..VIEW_W as i32 {
                dst.set(x, dy + y, self.pixel(cam.x as i32 + x, cam.y as i32 + y));
            }
        }
    }
}

/// The view's top left in arena pixels (`$18E`, `$190`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Camera {
    pub x: i16,
    pub y: i16,
}

impl Camera {
    pub const MAX_X: i16 = 0x140;
    pub const MAX_Y: i16 = 0x3C8;

    /// Where the match starts the camera (the centre spot under the view).
    pub fn start() -> Camera {
        Camera { x: 0xA0, y: 0x1E4 }
    }

    /// `clamp_player_position`: one frame's move towards the followed object at arena (`ox`, `oy`).
    pub fn follow(&mut self, ox: i16, oy: i16) {
        self.x = step(self.x, ox.wrapping_sub(0xA0), Camera::MAX_X);
        self.y = step(self.y, oy.wrapping_sub(0x5C), Camera::MAX_Y);
    }
}

/// `compute_clamped_difference` and one axis of `clamp_player_position`.
fn step(cur: i16, target: i16, max: i16) -> i16 {
    let d = ((target.wrapping_sub(cur).unsigned_abs() >> 3) + 1).min(16) as i16;
    match target.cmp(&cur) {
        std::cmp::Ordering::Equal => cur,
        std::cmp::Ordering::Greater => cur.wrapping_add(d).min(target).min(max),
        std::cmp::Ordering::Less => cur.wrapping_sub(d).max(target).max(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arena() -> Arena {
        // two tiles side by side: tile 1 has row y in colour y, tile 2 is all colour 3
        let mut px = vec![0u8; 32 * 16];
        for y in 0..16 {
            for x in 0..16 {
                px[y * 32 + x] = y as u8;
                px[y * 32 + 16 + x] = 3;
            }
        }
        let tiles = Sheet::new(Bitmap::from_indices(32, 16, px).unwrap(), 16, 2).unwrap();
        let mut cells = vec![0i16; MAP_W * MAP_H];
        cells[0] = 1;
        cells[1] = -1;
        cells[2] = 0xA2;
        cells[MAP_W] = 2;
        Arena::new(cells, tiles, [0; 32]).unwrap()
    }

    #[test]
    fn cells_tiles_flips_and_blanks() {
        let a = arena();
        assert_eq!(a.pixel(3, 5), 5);
        assert_eq!(a.pixel(16 + 3, 5), 10);
        assert_eq!(a.pixel(32, 0), BLANK);
        assert_eq!(a.pixel(0, 16), 3);
        assert_eq!(a.pixel(-1, 0), BLANK);
    }

    #[test]
    fn camera_steps_an_eighth_plus_one_up_to_16() {
        let mut c = Camera::start();
        c.follow(0xA0 + 0xA0 + 20, 0x1E4 + 0x5C);
        assert_eq!(
            c,
            Camera {
                x: 0xA0 + 3,
                y: 0x1E4
            }
        );
        let mut c = Camera { x: 0, y: 500 };
        c.follow(0xA0 + 300, 0x5C + 100);
        assert_eq!(c, Camera { x: 16, y: 484 });
        let mut c = Camera { x: 315, y: 960 };
        c.follow(2000, 2000);
        assert_eq!(
            c,
            Camera {
                x: Camera::MAX_X,
                y: Camera::MAX_Y
            }
        );
        let mut c = Camera { x: 3, y: 2 };
        c.follow(0, 0);
        assert_eq!(c, Camera { x: 0, y: 0 });
        let mut c = Camera { x: 100, y: 100 };
        c.follow(0xA0 + 101, 0x5C + 99);
        assert_eq!(c, Camera { x: 101, y: 99 });
    }
}
