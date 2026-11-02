//! The match pitch: the tile map, the pitch types (grass colours and the monthly weather chances) and the scrolled
//! view. Recovered from the main program: the pitch file loader ($C0323C, which file by the high byte of the pitch
//! type), the map fix-up ($C03298), the ring-buffer tile blitter ($C0343A, $C0371C) and the match copper list
//! ($C028B4, palette $C025C4 with entries 0, 7 and 9 from the pitch type at $C09BAC).
//!
//! A pitch file is a map of 38 x 46 big-endian longs (byte offsets of 128-byte tiles) followed by the tiles: 16 x 16
//! pixels, four bitplanes interleaved by row (row y, plane p at y * 8 + p * 2). Every marking is in the tiles; nothing
//! is drawn over them. The original scrolled a 352 x 272 ring buffer and blitted a new tile row or column as the
//! camera crossed a tile edge; natively the view is read straight from the map.

use native_engine::gfx::{rgb12, Frame};
use serde_json::Value;

use crate::rng::Rng;

pub const MAP_W: usize = 38;
pub const MAP_H: usize = 46;
pub const PITCH_W: i32 = (MAP_W * 16) as i32;
pub const PITCH_H: i32 = (MAP_H * 16) as i32;
/// Screen columns 1 to 336 show the pitch (the display window is 336 wide); column 0 and 337 to 351 show colour 0.
pub const VIEW_X0: i32 = 1;
pub const VIEW_W: i32 = 336;

/// One pitch file, decoded: the map as tile numbers and the tiles as colour indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pitch {
    pub map: Vec<u16>,
    pub tiles: Vec<[u8; 256]>,
}

impl Pitch {
    pub fn from_file(b: &[u8]) -> Result<Pitch, String> {
        let n = MAP_W * MAP_H;
        if b.len() < n * 4 {
            return Err("pitch map: short file".into());
        }
        let td = &b[n * 4..];
        let tiles: Vec<[u8; 256]> = td
            .as_chunks::<128>()
            .0
            .iter()
            .map(|t| {
                let mut o = [0u8; 256];
                for y in 0..16 {
                    for p in 0..4 {
                        let w = u16::from_be_bytes([t[y * 8 + p * 2], t[y * 8 + p * 2 + 1]]);
                        for x in 0..16 {
                            o[y * 16 + x] |= (((w >> (15 - x)) & 1) as u8) << p;
                        }
                    }
                }
                o
            })
            .collect();
        let mut map = Vec::with_capacity(n);
        for i in 0..n {
            let off = u32::from_be_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
            if !off.is_multiple_of(128) || (off / 128) as usize >= tiles.len() {
                return Err(format!("pitch map: entry {i} is not a tile offset"));
            }
            map.push((off / 128) as u16);
        }
        Ok(Pitch { map, tiles })
    }

    /// The colour index at pitch pixel (x, y). Columns past the right edge run on into the next map row, as the
    /// original's blitter read the map linearly; outside the map is colour 0.
    pub fn at(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }
        let (tx, ty) = ((x / 16) as usize, (y / 16) as usize);
        match self.map.get(ty * MAP_W + tx) {
            Some(&t) => self.tiles[t as usize][(y as usize % 16) * 16 + x as usize % 16],
            None => 0,
        }
    }
}

/// The match's tables (asset `match_tables`, read out of the main program by the extractor) and the two pitches.
#[derive(Debug, Clone)]
pub struct MatchData {
    pub palette: [u16; 16],
    /// Colours 0, 7 and 9 per pitch type (low byte of the pitch type word).
    pub grass: Vec<[u16; 3]>,
    /// Chances per month: 12 rows of 7 percentages, then the default row (8 bytes) for "no month".
    pub weather: Vec<u8>,
    pub weather_default: Vec<u8>,
    pub rng_table: Vec<u8>,
    /// The eight facing directions as (dx, dy) signs.
    pub aim: [(i32, i32); 8],
    pub pitches: Vec<Pitch>,
}

fn ints(v: &Value, k: &str) -> Result<Vec<i64>, String> {
    v[k].as_array()
        .ok_or(format!("match_tables: no {k}"))?
        .iter()
        .map(|x| x.as_i64().ok_or(format!("match_tables: bad {k}")))
        .collect()
}

impl MatchData {
    pub fn from_json(
        v: &Value,
        map_file: &dyn Fn(&str) -> Result<Vec<u8>, String>,
    ) -> Result<MatchData, String> {
        let pal = ints(v, "palette")?;
        if pal.len() != 16 {
            return Err("match_tables: palette is not 16 colours".into());
        }
        let mut palette = [0u16; 16];
        for (d, s) in palette.iter_mut().zip(&pal) {
            *d = *s as u16;
        }
        let grass = v["grass"]
            .as_array()
            .ok_or("match_tables: no grass")?
            .iter()
            .map(|g| {
                let a: Vec<u16> = g
                    .as_array()
                    .map(|a| a.iter().map(|x| x.as_i64().unwrap_or(0) as u16).collect())
                    .unwrap_or_default();
                if a.len() == 3 {
                    Ok([a[0], a[1], a[2]])
                } else {
                    Err("match_tables: bad grass".to_string())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let bytes = |k: &str| -> Result<Vec<u8>, String> {
            Ok(ints(v, k)?.iter().map(|&x| x as u8).collect())
        };
        let rs = v["rng"].as_str().ok_or("match_tables: no rng")?;
        let rng_table: Vec<u8> = (0..rs.len() / 2)
            .map(|i| {
                u8::from_str_radix(&rs[i * 2..i * 2 + 2], 16)
                    .map_err(|_| "match_tables: bad rng".to_string())
            })
            .collect::<Result<_, _>>()?;
        if rng_table.len() != 1024 {
            return Err("match_tables: rng table is not 1024 bytes".into());
        }
        let mut aim = [(0, 0); 8];
        for (d, a) in aim
            .iter_mut()
            .zip(v["aim"].as_array().ok_or("match_tables: no aim")?)
        {
            *d = (
                a[0].as_i64().unwrap_or(0) as i32,
                a[1].as_i64().unwrap_or(0) as i32,
            );
        }
        let pitches = v["maps"]
            .as_array()
            .ok_or("match_tables: no maps")?
            .iter()
            .map(|n| Pitch::from_file(&map_file(n.as_str().unwrap_or(""))?))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MatchData {
            palette,
            grass,
            weather: bytes("weather")?,
            weather_default: bytes("weather_default")?,
            rng_table,
            aim,
            pitches,
        })
    }

    /// The match palette for a pitch type ($C09BAC): the program palette with colours 0, 7 and 9 from the type.
    pub fn palette_for(&self, pitch_type: u16) -> [u16; 16] {
        let mut p = self.palette;
        if let Some(g) = self.grass.get((pitch_type & 0xFF) as usize) {
            p[0] = g[0];
            p[7] = g[1];
            p[9] = g[2];
        }
        p
    }

    /// The pitch file for a pitch type: its high byte ($C09A00).
    pub fn pitch_for(&self, pitch_type: u16) -> &Pitch {
        let i = (pitch_type >> 8) as usize;
        &self.pitches[i.min(self.pitches.len() - 1)]
    }

    /// The pitch type chosen before a match ($C0B3FA). `month` 0 to 11 picks a row of chances, -1 the default row,
    /// -2 to -8 a type directly (month + 8). The random number scaled to 0..99 walks the row; bit 4 of the frame
    /// counter picks the pitch file (the high byte).
    pub fn choose_pitch_type(&self, month: i16, rng: &mut Rng, frame: u16) -> u16 {
        let mut kind: u16 = match month {
            m if m >= 0 || m == -1 => {
                let (row, at) = if m >= 0 {
                    (&self.weather, m as usize * 7)
                } else {
                    (&self.weather_default, 0)
                };
                let mut r = ((rng.next(frame, &self.rng_table) as u32 * 100) >> 8) as i8;
                let mut k = 0u16;
                while let Some(&c) = row.get(at + k as usize) {
                    if r < c as i8 {
                        break;
                    }
                    r = r.wrapping_sub(c as i8);
                    k += 1;
                }
                k
            }
            m => (m + 8) as u16,
        };
        kind = kind.wrapping_add((frame & 0x10) << 4);
        kind
    }
}

/// Draws the pitch seen from camera (`cx`, `cy`) with the palette of pitch type `pitch_type`: screen column `sx`
/// shows pitch column `cx - 1 + sx`. The display shows the camera of the frame before (the original drew into the
/// back buffer and swapped at the next vertical blank), measured on the reference: pass the previous frame's camera.
pub fn render(d: &MatchData, pitch_type: u16, cx: i32, cy: i32, frame: &mut Frame) {
    let pal = d.palette_for(pitch_type);
    for (i, c) in pal.iter().enumerate() {
        frame.palette.colours[i] = rgb12(*c);
    }
    let p = d.pitch_for(pitch_type);
    let (w, h) = (frame.image.w, frame.image.h);
    for sy in 0..h {
        let row = &mut frame.image.px[sy * w..(sy + 1) * w];
        for (sx, o) in row.iter_mut().enumerate() {
            let sx = sx as i32;
            *o = if (VIEW_X0..VIEW_X0 + VIEW_W).contains(&sx) {
                p.at(cx - 1 + sx, cy + sy as i32)
            } else {
                0
            };
        }
    }
}
