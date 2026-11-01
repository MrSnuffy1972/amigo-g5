//! The asset pack of game 2055 loaded into typed tables once. The pack is built outside the repo by extract2055.py
//! from the recovered program (pictures decoded from the game's own IFF pictures, the 16x16 intro font, the intro
//! pages); nothing of the game is in git.

use native_engine::gfx::{Bitmap, Palette, Sheet};
use native_engine::pack::Pack;
use serde_json::Value;

/// One intro page as the original stores it: a byte at or below `$40` is a position (column byte, then row in
/// 4-line units), any other byte is a glyph (`byte - 'A'`).
pub type Page = Vec<u8>;

pub struct Assets {
    /// The title picture (320 x 200, 32 colours) and its colours as the game converts them (12-bit).
    pub title: Bitmap,
    pub title_pal: [u16; 32],
    /// The arena picture the story and credits are typed over (the left 320 columns of the 640-wide picture, which
    /// is what the game's decoder leaves on its 320-wide screen).
    pub arena: Bitmap,
    pub arena_pal: [u16; 32],
    /// 43 glyphs of 16 x 16: `A`..`Z`, the digits and punctuation the pages use, the cursor (41) and the blank (42).
    pub font: Sheet,
    pub start_pages: Vec<Page>,
    pub story_pages: Vec<Page>,
    pub credit_pages: Vec<Page>,
    pub loading_page: Page,
}

fn pal12(p: &Palette) -> [u16; 32] {
    let mut out = [0u16; 32];
    for (o, c) in out.iter_mut().zip(&p.colours) {
        let n = |s: u32| ((c >> s) & 0xFF) as u16 / 17;
        *o = n(16) << 8 | n(8) << 4 | n(0);
    }
    out
}

fn pages(v: &Value, k: &str) -> Result<Vec<Page>, String> {
    v["pages"][k]
        .as_array()
        .ok_or(format!("intro: no {k} pages"))?
        .iter()
        .map(|p| {
            p.as_array()
                .ok_or(format!("intro: bad {k} page"))?
                .iter()
                .map(|b| {
                    b.as_u64()
                        .map(|b| b as u8)
                        .ok_or("intro: bad byte".to_string())
                })
                .collect()
        })
        .collect()
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let intro = p.data("intro")?;
        let font = p.sheet("introfont")?;
        if font.frame_w != 16 || font.frame_h != 16 || font.frames < 43 {
            return Err("introfont: want 43 glyphs of 16 x 16".into());
        }
        let loading = pages(&intro, "loading")?;
        let a = Assets {
            title: p.bitmap("title")?,
            title_pal: pal12(&p.palette("title_pal")?),
            arena: p.bitmap("arena")?,
            arena_pal: pal12(&p.palette("arena_pal")?),
            font,
            start_pages: pages(&intro, "start")?,
            story_pages: pages(&intro, "story")?,
            credit_pages: pages(&intro, "credits")?,
            loading_page: loading.into_iter().next().ok_or("intro: no loading page")?,
        };
        for (n, b) in [("title", &a.title), ("arena", &a.arena)] {
            if b.w != 320 || b.h != 200 {
                return Err(format!("{n}: want 320 x 200"));
            }
        }
        if a.start_pages.len() != 2 || a.story_pages.len() != 9 || a.credit_pages.len() != 8 {
            return Err("intro: want 2 start, 9 story and 8 credit pages".into());
        }
        Ok(a)
    }
}
