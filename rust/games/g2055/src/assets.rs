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
    /// Part 2: the menu panel (the picture of disk file 1, which the game loads to `$361D6`) and its colours, the menu
    /// font (38 glyphs of 16 x 16: the letters, the digits and punctuation after `Z`, the arrows 36 and 37), the
    /// heading and footing fonts (28 glyphs of 8 x 8 each) and the menu tables.
    pub panel: Bitmap,
    pub panel_pal: [u16; 32],
    pub menufont: Sheet,
    pub topfont: Sheet,
    pub botfont: Sheet,
    pub menus: Menus,
    /// The 8 x 8 text font of the league screens (letters, digits, box pieces) and the league tables (part 2).
    pub textfont: Sheet,
    pub league: crate::league::League,
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
            panel: p.bitmap("panel")?,
            panel_pal: pal12(&p.palette("panel_pal")?),
            menufont: p.sheet("menufont")?,
            topfont: p.sheet("topfont")?,
            botfont: p.sheet("botfont")?,
            menus: Menus::load(&p.data("menus")?)?,
            textfont: p.sheet("textfont")?,
            league: crate::league::League::load(&p.data("fixture")?)?,
        };
        for (n, b) in [
            ("title", &a.title),
            ("arena", &a.arena),
            ("panel", &a.panel),
        ] {
            if b.w != 320 || b.h != 200 {
                return Err(format!("{n}: want 320 x 200"));
            }
        }
        if a.start_pages.len() != 2 || a.story_pages.len() != 9 || a.credit_pages.len() != 8 {
            return Err("intro: want 2 start, 9 story and 8 credit pages".into());
        }
        if a.menufont.frame_w != 16 || a.menufont.frame_h != 16 || a.menufont.frames < 38 {
            return Err("menufont: want 38 glyphs of 16 x 16".into());
        }
        for (n, f) in [("topfont", &a.topfont), ("botfont", &a.botfont)] {
            if f.frame_w != 8 || f.frame_h != 8 {
                return Err(format!("{n}: want glyphs of 8 x 8"));
            }
        }
        if a.menus.players.len() != 4 || a.menus.matches.len() != 4 || a.menus.cup.len() != 2 {
            return Err("menus: want 4 players, 4 match and 2 cup items".into());
        }
        if a.textfont.frame_w != 8 || a.textfont.frame_h != 8 || a.textfont.frames < 45 {
            return Err("textfont: want 45 glyphs of 8 x 8".into());
        }
        Ok(a)
    }
}

/// One menu as the original's menu tables give it: the heading and the footing (in the 8 x 8 heading fonts), the
/// items (in the 16 x 16 menu font) and, for each item, where its two arrows go (column byte and row in 4-line units,
/// left arrow then right arrow). Text bytes below `$20` are a position (column byte, then row), `$20` is a space and
/// any other byte is a glyph (`byte - 'A'`; the digits are stored as the bytes after `Z`).
pub struct Menu {
    pub top: Vec<u8>,
    pub bottom: Vec<u8>,
    pub items: Vec<u8>,
    pub arrows: Vec<[u16; 4]>,
}

impl Menu {
    pub fn len(&self) -> usize {
        self.arrows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.arrows.is_empty()
    }
}

/// The menus of the front end (part 2).
pub struct Menus {
    /// 1 PLAYER GAME, 2 PLAYER GAME, DEMO GAME, REPLAY GOALS (the first menu, `initialize_post_match_screen`).
    pub players: Menu,
    /// KNOCKOUT, LEAGUE, CUP, PRACTICE.
    pub matches: Menu,
    /// The league's NEW GAME, TEAM MANAGER, LOAD GAME.
    pub league: Menu,
    /// ONE MATCH, BEST OF THREE, BEST OF FIVE.
    pub series: Menu,
    /// RETRY, CANCEL.
    pub disk_error: Menu,
    /// The cup's NEW GAME, LOAD GAME.
    pub cup: Menu,
}

fn bytes(v: &Value, what: &str) -> Result<Vec<u8>, String> {
    v.as_array()
        .ok_or(format!("menus: no {what}"))?
        .iter()
        .map(|b| {
            b.as_u64()
                .map(|b| b as u8)
                .ok_or(format!("menus: bad byte in {what}"))
        })
        .collect()
}

fn menu(v: &Value, k: &str) -> Result<Menu, String> {
    let m = &v["menus"][k];
    let arrows = m["arrows"]
        .as_array()
        .ok_or(format!("menus: no {k} arrows"))?
        .iter()
        .map(|a| {
            let a = bytes(a, k)?;
            if a.len() != 4 {
                return Err(format!("menus: bad {k} arrows"));
            }
            Ok([a[0] as u16, a[1] as u16, a[2] as u16, a[3] as u16])
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Menu {
        top: bytes(&m["top"], k)?,
        bottom: bytes(&m["bottom"], k)?,
        items: bytes(&m["items"], k)?,
        arrows,
    })
}

impl Menus {
    pub fn load(v: &Value) -> Result<Menus, String> {
        Ok(Menus {
            players: menu(v, "players")?,
            matches: menu(v, "match")?,
            league: menu(v, "league")?,
            series: menu(v, "series")?,
            disk_error: menu(v, "disk_error")?,
            cup: menu(v, "cup")?,
        })
    }
}
