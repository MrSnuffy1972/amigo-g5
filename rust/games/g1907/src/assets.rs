//! The asset pack of game 1907 loaded into typed tables once. Everything here comes from the pack, which the
//! extractor (outside the repository) builds from the original game's files: pictures, the fighters' frames and
//! shadows, fonts, colours, sounds, the music and the game's tables.

use std::collections::HashMap;
use std::sync::Arc;

use native_engine::pack::Pack;

/// The picture is 320 by 200, as the original's playfield.
pub const W: usize = 320;
pub const H: usize = 200;

/// A picture of 4-bit colour numbers (the original's four bitplanes), row by row.
#[derive(Clone)]
pub struct Pic {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

impl Pic {
    pub fn at(&self, x: usize, y: usize) -> u8 {
        self.px[y * self.w + x]
    }
}

/// One fighter frame: its pixels (1..7 = plane 1, 2 and 3 bits; 0 = clear) and where its corner sits relative to the
/// fighter's anchor (x in pixels from twice the fighter's position minus 32, y in screen rows).
#[derive(Clone)]
pub struct FighterFrame {
    pub dx: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

pub struct Assets {
    pub background: Pic,
    pub sheet1: Pic,
    pub sheet2: Pic,
    /// 96 frames facing right, then the same 96 mirrored.
    pub fighters: Vec<FighterFrame>,
    /// 8 rows of 80 pixels under each frame (same order), drawn into the shadow plane.
    pub shadows: Vec<[u128; 8]>,
    pub font8: Vec<[u8; 8]>,
    pub font5: Vec<[u8; 5]>,
    /// The game screen's colour changes down the picture: (row, colour, 12-bit value).
    pub colour_rows: Vec<(i32, usize, u16)>,
    /// The colour changes the program rewrites: sea, p1, p2, c1, c9, shadow (index into colour_rows).
    pub slots: HashMap<String, usize>,
    pub sea_choices: [u16; 4],
    pub sfx: Vec<Arc<Vec<i8>>>,
    pub music: Arc<Vec<u8>>,
    pub music_base: u32,
    pub music_channels: [u32; 4],
    pub tables: HashMap<String, Vec<u8>>,
    /// The effects' sample block (the music's instruments may point into it too).
    pub sfx_bytes: Vec<u8>,
    pub sfx_base: u32,
    /// The judge's messages and the code-word messages (English and German), each ending with $9B.
    pub speech: Vec<Vec<u8>>,
    pub speech_de: Vec<Vec<u8>>,
    pub codes: Vec<Vec<u8>>,
    pub codes_de: Vec<Vec<u8>>,
}

fn hex(s: &str) -> Result<Vec<u8>, String> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn pic(p: &Pack, n: &str) -> Result<Pic, String> {
    let b = p.bitmap(n)?;
    Ok(Pic {
        w: b.w,
        h: b.h,
        px: b.px.clone(),
    })
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let fs = p.sheet("fighters")?;
        let boxes = p.data("fighter_boxes")?;
        let boxes = boxes.as_array().ok_or("fighter_boxes")?;
        let mut fighters = Vec::new();
        for (n, b) in boxes.iter().enumerate() {
            let v: Vec<i64> = b
                .as_array()
                .ok_or("box")?
                .iter()
                .map(|x| x.as_i64().unwrap_or(0))
                .collect();
            let r = fs.frame(n);
            let (w, h) = (v[2] as usize, v[3] as usize);
            let mut px = vec![0u8; w * h];
            for y in 0..h {
                for x in 0..w {
                    px[y * w + x] = fs.image.get(r.x + x as i32, r.y + y as i32);
                }
            }
            fighters.push(FighterFrame {
                dx: v[0] as i32,
                y: v[1] as i32,
                w,
                h,
                px,
            });
        }
        let ss = p.sheet("shadows")?;
        let mut shadows = Vec::new();
        for n in 0..192 {
            let r = ss.frame(n);
            let mut s = [0u128; 8];
            for (y, row) in s.iter_mut().enumerate() {
                for x in 0..80 {
                    if ss.image.get(r.x + x, r.y + y as i32) != 0 {
                        *row |= 1u128 << (79 - x);
                    }
                }
            }
            shadows.push(s);
        }
        let font = |n: &str, rows: usize| -> Result<Vec<Vec<u8>>, String> {
            let s = p.sheet(n)?;
            Ok((0..s.frames)
                .map(|g| {
                    let r = s.frame(g);
                    (0..rows)
                        .map(|y| {
                            (0..8).fold(0u8, |a, x| {
                                a | ((s.image.get(r.x + x, r.y + y as i32) != 0) as u8) << (7 - x)
                            })
                        })
                        .collect()
                })
                .collect())
        };
        let font8 = font("font8", 8)?
            .into_iter()
            .map(|v| v.try_into().unwrap_or([0; 8]))
            .collect();
        let font5 = font("font5", 5)?
            .into_iter()
            .map(|v| v.try_into().unwrap_or([0; 5]))
            .collect();
        let gc = p.data("game_colours")?;
        let colour_rows = gc["rows"]
            .as_array()
            .ok_or("rows")?
            .iter()
            .map(|r| {
                (
                    r[0].as_i64().unwrap_or(0) as i32,
                    r[1].as_u64().unwrap_or(0) as usize,
                    r[2].as_u64().unwrap_or(0) as u16,
                )
            })
            .collect();
        let slots = gc["slots"]
            .as_object()
            .ok_or("slots")?
            .iter()
            .map(|(k, v)| (k.clone(), v.as_u64().unwrap_or(0) as usize))
            .collect();
        let mut sea_choices = [0u16; 4];
        for (i, c) in sea_choices.iter_mut().enumerate() {
            *c = gc["sea_choices"][i].as_u64().ok_or("sea")? as u16;
        }
        let mut sfx = Vec::new();
        for i in 0..18 {
            sfx.push(p.sample(&format!("sfx{i:02}"), 8000.0)?.data);
        }
        let m = p.data("music")?;
        let music = Arc::new(hex(m["hex"].as_str().ok_or("music")?)?);
        let mut music_channels = [0u32; 4];
        for (i, c) in music_channels.iter_mut().enumerate() {
            *c = m["channels"][i].as_u64().ok_or("channels")? as u32;
        }
        let t = p.data("tables")?;
        let tables = t
            .as_object()
            .ok_or("tables")?
            .iter()
            .map(|(k, v)| Ok((k.clone(), hex(v.as_str().unwrap_or(""))?)))
            .collect::<Result<_, String>>()?;
        let sx = p.data("sfx_samples")?;
        let sfx_bytes = hex(sx["hex"].as_str().ok_or("sfx_samples")?)?;
        let sfx_base = sx["base"].as_u64().ok_or("sfx base")? as u32;
        let tx = p.data("texts")?;
        let list = |k: &str| -> Result<Vec<Vec<u8>>, String> {
            tx[k]
                .as_array()
                .ok_or("texts")?
                .iter()
                .map(|v| hex(v.as_str().unwrap_or("")))
                .collect()
        };
        Ok(Assets {
            sfx_bytes,
            sfx_base,
            speech: list("speech")?,
            speech_de: list("speech_de")?,
            codes: list("codes")?,
            codes_de: list("codes_de")?,
            background: pic(p, "background")?,
            sheet1: pic(p, "sheet1")?,
            sheet2: pic(p, "sheet2")?,
            fighters,
            shadows,
            font8,
            font5,
            colour_rows,
            slots,
            sea_choices,
            sfx,
            music,
            music_base: m["base"].as_u64().ok_or("base")? as u32,
            music_channels,
            tables,
        })
    }

    /// A table of the game by its name (see the extractor's list).
    pub fn t(&self, n: &str) -> &[u8] {
        &self.tables[n]
    }
}
