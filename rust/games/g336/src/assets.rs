//! Game 336's assets and data, typed, from its asset pack (built from the recovered code and data by the extractor;
//! the pack never enters the repo). Nothing here holds game bytes: every picture, sound, table and level is loaded.

use std::collections::BTreeMap;
use std::sync::Arc;

use native_engine::audio::Sample;
use native_engine::gfx::{rgb12, Argb, Bitmap};
use native_engine::pack::Pack;
use serde_json::Value;

/// Width and height of the game's picture (and of every full-screen picture it loads).
pub const W: usize = 320;
pub const H: usize = 208;
/// Tiles in the two tile sheets together (13 rows of 20 in each).
pub const TILES: usize = 520;
/// Bytes of one level's brick map (26 rows of 13) and of one level's background map (13 rows of 13).
pub const BRICK_MAP: usize = 338;
pub const BACK_MAP: usize = 169;
/// Levels in each map file.
pub const LEVELS: usize = 100;

/// One entry of the game's sound table.
#[derive(Debug, Clone)]
pub struct Sfx {
    pub sample: Sample,
    pub period: u16,
    pub volume: u8,
    /// Frames the channel stays on (the sample repeats until then); 65535 is "until stopped".
    pub duration: u16,
    /// Enabled by the music switch (true) or the sound-effects switch (false).
    pub music: bool,
    /// Loaded from its own file on first use (into the one shared buffer).
    pub own_file: bool,
}

/// A clickable box of the title or editor screen, with the picture it shows pressed and released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button {
    /// Source (y, x) in the button picture when pressed and when released.
    pub on: (i16, i16),
    pub off: (i16, i16),
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub id: i16,
}

/// A 16 by 16 tile: palette indices (0 to 31) and the mask of its set pixels.
#[derive(Debug, Clone)]
pub struct Tile {
    pub px: [u8; 256],
}

impl Tile {
    pub fn at(&self, x: usize, y: usize) -> u8 {
        self.px[y * 16 + x]
    }
}

#[derive(Debug, Clone)]
pub struct Assets {
    pub pictures: BTreeMap<String, Bitmap>,
    /// The 32 colour registers each picture sets (12-bit values, as the original copied them from the file).
    pub palettes: BTreeMap<String, Vec<u16>>,
    /// The hold-and-modify title picture as the display showed it.
    pub intro_direct: Arc<Vec<Argb>>,
    pub tiles: Vec<Tile>,
    /// 8 by 8 glyphs, one byte a row (bit 7 leftmost), from character 32.
    pub font: Vec<[u8; 8]>,
    pub sfx: Vec<Sfx>,
    pub menu_buttons: Vec<Button>,
    pub editor_buttons: Vec<Button>,
    pub editor_brick_sfx: Vec<i16>,
    pub border_pattern: Vec<u8>,
    pub editor_names: Vec<String>,
    pub reg_bricks: Vec<u8>,
    pub reg_backs: Vec<u8>,
    pub cust_bricks: Vec<u8>,
    pub cust_backs: Vec<u8>,
}

fn hex(v: &Value, k: &str, n: usize) -> Result<Vec<u8>, String> {
    let s = v[k].as_str().ok_or(format!("maps: no {k}"))?;
    let b: Result<Vec<u8>, _> = (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16))
        .collect();
    let b = b.map_err(|e| format!("maps {k}: {e}"))?;
    if b.len() != n {
        return Err(format!("maps {k}: {} bytes, want {n}", b.len()));
    }
    Ok(b)
}

fn int(v: &Value) -> Result<i64, String> {
    v.as_i64().ok_or(format!("not a number: {v}"))
}

fn buttons(v: &Value) -> Result<Vec<Button>, String> {
    let a = v.as_array().ok_or("buttons: not a list")?;
    a.iter()
        .map(|b| {
            let p = |k: &str, i: usize| int(&b[k][i]).map(|n| n as i16);
            let f = |k: &str| int(&b[k]).map(|n| n as i16);
            Ok(Button {
                on: (p("on", 0)?, p("on", 1)?),
                off: (p("off", 0)?, p("off", 1)?),
                x: f("x")?,
                y: f("y")?,
                w: f("w")?,
                h: f("h")?,
                id: f("id")?,
            })
        })
        .collect()
}

/// The six bit-planes' pixel values of the title picture shown in hold-and-modify: each pixel either takes one of the
/// 16 base colours or keeps the colour to its left with one component replaced; every row starts from colour 0.
pub fn hold_and_modify(px: &[u8], w: usize, base: &[u16]) -> Vec<Argb> {
    let mut out = Vec::with_capacity(px.len());
    for row in px.chunks(w) {
        let mut c = base[0];
        for &v in row {
            let m = (v & 15) as u16;
            c = match v >> 4 {
                0 => base[m as usize],
                1 => (c & 0xFF0) | m,
                2 => (c & 0x0FF) | (m << 8),
                _ => (c & 0xF0F) | (m << 4),
            };
            out.push(rgb12(c));
        }
    }
    out
}

/// The 16 by 16 tiles of a sheet picture, row by row (load_jail_tile_graphics, wait_for_input_frames).
fn slice(b: &Bitmap, out: &mut Vec<Tile>) {
    for ty in 0..13 {
        for tx in 0..20 {
            let mut t = Tile { px: [0; 256] };
            for y in 0..16 {
                for x in 0..16 {
                    t.px[y * 16 + x] = b.get((tx * 16 + x) as i32, (ty * 16 + y) as i32) & 31;
                }
            }
            out.push(t);
        }
    }
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let names = [
            "jailtiles1",
            "jailtiles2",
            "jailtitle1",
            "jailtitle2",
            "jailgadgets1",
            "jailgadgets2",
            "jailplayer1",
            "jailplayer2",
            "jailintro1",
            "jailintro2",
            "jailover1",
            "jailover2",
            "jailover3",
        ];
        let pal = p.data("palettes")?;
        let mut pictures = BTreeMap::new();
        let mut palettes = BTreeMap::new();
        for n in names {
            let b = p.bitmap(n)?;
            if (b.w, b.h) != (W, H) {
                return Err(format!("{n}: {}x{}, want {W}x{H}", b.w, b.h));
            }
            let c: Result<Vec<u16>, String> = pal[n]
                .as_array()
                .ok_or(format!("palettes: no {n}"))?
                .iter()
                .map(|v| int(v).map(|x| x as u16))
                .collect();
            let c = c?;
            if c.len() != 32 {
                return Err(format!("palette {n}: {} colours", c.len()));
            }
            palettes.insert(n.to_string(), c);
            pictures.insert(n.to_string(), b);
        }
        let intro_direct = Arc::new(hold_and_modify(
            &pictures["jailintro1"].px,
            W,
            &palettes["jailintro1"],
        ));
        let mut tiles = Vec::with_capacity(TILES);
        slice(&pictures["jailtiles1"], &mut tiles);
        slice(&pictures["jailtiles2"], &mut tiles);
        // copy_character_graphics: the game asks for a character picture its disk does not have, so the open fails and
        // the buffer still holds the last picture read (the second tile sheet). Its first bit-plane, top 16 rows, is the
        // font: 40 glyphs of 8 by 8 a row, two rows.
        let t2 = &pictures["jailtiles2"];
        let mut font = Vec::with_capacity(80);
        for g in 0..80usize {
            let mut gl = [0u8; 8];
            for (r, row) in gl.iter_mut().enumerate() {
                for b in 0..8 {
                    let v = t2.get((g % 40 * 8 + b) as i32, ((g / 40) * 8 + r) as i32);
                    if v & 1 != 0 {
                        *row |= 0x80 >> b;
                    }
                }
            }
            font.push(gl);
        }
        let t = p.data("tables")?;
        let mut sfx = Vec::new();
        for e in t["sfx"].as_array().ok_or("tables: no sfx")? {
            let period = int(&e["period"])? as u16;
            let name = e["name"].as_str().ok_or("sfx: no name")?;
            sfx.push(Sfx {
                sample: p.sample(name, PAULA_CLOCK / period.max(1) as f32)?,
                period,
                volume: int(&e["volume"])?.clamp(0, 64) as u8,
                duration: int(&e["duration"])? as u16,
                music: int(&e["music"])? != 0,
                own_file: e.get("file").is_some(),
            });
        }
        if sfx.len() != 33 {
            return Err(format!("tables: {} sounds, want 33", sfx.len()));
        }
        let list = |k: &str| -> Result<Vec<i64>, String> {
            t[k].as_array()
                .ok_or(format!("tables: no {k}"))?
                .iter()
                .map(int)
                .collect()
        };
        let editor_brick_sfx: Vec<i16> = list("editor_brick_sfx")?
            .iter()
            .map(|&v| v as i16)
            .collect();
        let border_pattern: Vec<u8> = list("border_pattern")?.iter().map(|&v| v as u8).collect();
        if editor_brick_sfx.len() != 32 || border_pattern.len() != 260 {
            return Err("tables: editor sounds or border pattern size".into());
        }
        let editor_names: Vec<String> = t["editor_names"]
            .as_array()
            .ok_or("tables: no editor_names")?
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .collect();
        if editor_names.len() != 32 {
            return Err("tables: editor names".into());
        }
        let m = p.data("maps")?;
        Ok(Assets {
            pictures,
            palettes,
            intro_direct,
            tiles,
            font,
            sfx,
            menu_buttons: buttons(&t["menu_buttons"])?,
            editor_buttons: buttons(&t["editor_buttons"])?,
            editor_brick_sfx,
            border_pattern,
            editor_names,
            reg_bricks: hex(&m, "jailregbrick", BRICK_MAP * LEVELS)?,
            reg_backs: hex(&m, "jailregback", BACK_MAP * LEVELS)?,
            cust_bricks: hex(&m, "jailcustbrick", BRICK_MAP * LEVELS)?,
            cust_backs: hex(&m, "jailcustback", BACK_MAP * LEVELS)?,
        })
    }

    pub fn picture(&self, n: &str) -> &Bitmap {
        &self.pictures[n]
    }
}

/// The PAL sound clock: a sample played at period P runs at this many bytes a second over P.
pub const PAULA_CLOCK: f32 = 3_546_895.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_and_modify_keeps_and_changes_components() {
        let mut base = vec![0u16; 16];
        base[1] = 0x0123;
        // base colour 1, then blue 15, red 9, green 4, then base 0 again
        let px = [1u8, 0x1f, 0x29, 0x34, 0x00];
        let out = hold_and_modify(&px, 5, &base);
        assert_eq!(out[0], rgb12(0x123));
        assert_eq!(out[1], rgb12(0x12f));
        assert_eq!(out[2], rgb12(0x92f));
        assert_eq!(out[3], rgb12(0x94f));
        assert_eq!(out[4], rgb12(0x000));
    }

    #[test]
    fn every_row_starts_from_colour_zero() {
        let mut base = vec![0u16; 16];
        base[0] = 0x0a00;
        let out = hold_and_modify(&[0x1f, 0x1f], 1, &base);
        assert_eq!(out, vec![rgb12(0xa0f), rgb12(0xa0f)]);
    }
}
