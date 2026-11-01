//! The asset pack (made on Ryzen by extract1727.py; nothing of the game is in the repository): the pictures with
//! their palettes, each round's bob set as one atlas with its frame rectangles, the 8x8 font and the texts.

use native_engine::gfx::{Bitmap, Font, Palette, Rect};
use native_engine::pack::Pack;

/// The logical screen: the original's map and text coordinates (the playfield at the left, the HUD at the right).
/// 336 wide: the original's display is overscanned and its HUD text reaches x = 335.
pub const W: usize = 336;
pub const H: usize = 200;
pub const ROUNDS: usize = 16;

pub struct Picture {
    pub image: Bitmap,
    pub palette: Palette,
}

pub struct Bobs {
    pub atlas: Bitmap,
    pub rects: Vec<Rect>,
}

pub struct Assets {
    /// The title picture (Screen@).
    pub title: Picture,
    /// Screen A .. Screen Q: round r's surface is `screens[r]`, its claimed area shows `screens[r + 1]`.
    pub screens: Vec<Picture>,
    /// The bob sets: `bobs[0]` the title set, `bobs[r + 1]` round r's set (Sprite A + round).
    pub bobs: Vec<Bobs>,
    pub font: Font,
    pub texts: serde_json::Value,
}

fn letters() -> impl Iterator<Item = char> {
    ('a'..='q').take(ROUNDS + 1)
}

fn bobs(p: &Pack, name: &str) -> Result<Bobs, String> {
    let atlas = p.bitmap(name)?;
    let v = p.data(&format!("{name}_rects"))?;
    let rects = v["rects"]
        .as_array()
        .ok_or(format!("{name}_rects: no rects"))?
        .iter()
        .map(|r| {
            let n = |i: usize| r[i].as_i64().unwrap_or(0) as i32;
            Rect::new(n(0), n(1), n(2), n(3))
        })
        .collect();
    Ok(Bobs { atlas, rects })
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let pic = |s: &str| -> Result<Picture, String> {
            Ok(Picture {
                image: p.bitmap(&format!("scr_{s}"))?,
                palette: p.palette(&format!("pal_{s}"))?,
            })
        };
        let mut screens = Vec::new();
        let mut sets = vec![bobs(p, "bobs_at")?];
        for c in letters() {
            screens.push(pic(&c.to_string())?);
            if sets.len() <= ROUNDS {
                sets.push(bobs(p, &format!("bobs_{c}"))?);
            }
        }
        Ok(Assets {
            title: pic("at")?,
            screens,
            bobs: sets,
            font: p.font("font8")?,
            texts: p.data("texts")?,
        })
    }

    pub fn text(&self, k: &str) -> String {
        self.texts[k].as_str().unwrap_or("").to_string()
    }
}
