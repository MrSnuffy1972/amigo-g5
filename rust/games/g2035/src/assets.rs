//! The game's assets, read once from the asset pack (built outside the repo by the extractor).

use std::collections::BTreeMap;

use native_engine::gfx::{Argb, Bitmap, Font, Palette};
use native_engine::pack::Pack;
use serde_json::Value;

/// The logical screen: the original's 352 x 272 PAL overscan display.
pub const W: usize = 352;
pub const H: usize = 272;

/// The Crack Inc intro: a static text page (lines centred, one every `step` lines from `top`), shown for `ticks`.
pub struct IntroPage {
    pub lines: Vec<String>,
    pub top: i32,
    pub step: i32,
    pub background: Argb,
    pub ink: Argb,
    pub ticks: u32,
    pub font: Font,
}

/// Where a sprite number is cut from: picture `pic` (an index into `FrontEnd::pics`), rectangle x, y, w, h. The
/// original cut these at start-up from its sprite cut lists (main program $C131CC base set, $C16FF0 menu set).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpriteDef {
    pub pic: usize,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// A menu item's text: none, literal (the language names), or an offset into the current language's text block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    None,
    Text(Vec<u8>),
    TextOff(usize),
}

/// One 16-byte menu item of the original: kind ($0A box, $04 sprite, $0B text), place and size, attribute word
/// (bit 15 framed; bits 8-14 text colour; low byte fill colour, bit 7 of it = no fill), text or sprite number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub kind: u16,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub attr: u16,
    pub label: Label,
    pub sprite: u16,
    pub sdx: i32,
    pub sdy: i32,
}

/// A menu descriptor: items, the item and grid cell the cursor starts on, and the navigation grid (item numbers,
/// -1 none, -2 to -11 the original's walk codes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuDef {
    pub name: String,
    pub items: Vec<Item>,
    pub first: usize,
    pub col: i32,
    pub row: i32,
    pub cols: i32,
    pub rows: i32,
    pub grid: Vec<i8>,
}

/// The menu system's tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuUi {
    pub font_h: i32,
    /// Glyph number per character code minus 32 (255 = none).
    pub font_index: Vec<u8>,
    /// Text colour -> first sprite number of the font set in that colour.
    pub font_base: Vec<(u16, u16)>,
    /// Bottom edge colour per fill colour.
    pub shade: Vec<u8>,
    /// Cursor colours, one per two frames.
    pub flash: Vec<u8>,
    pub language_idle: u32,
    pub main_idle: u32,
    pub move_delay: u32,
    pub fire_delay: u32,
    pub text_right: i32,
}

/// Everything the front end (language screen and menus) draws with.
pub struct FrontEnd {
    /// The pictures sprites are cut from (`cjcgrafs`, `cjcbits`, ...).
    pub pics: Vec<Bitmap>,
    /// By sprite number: the base set, overlaid by the menu set (as the original loaded them).
    pub sprites: Vec<Option<SpriteDef>>,
    /// The `Marble` menu background and the shared 16-colour menu palette.
    pub background: Bitmap,
    pub palette: Palette,
    pub menus: Vec<MenuDef>,
    pub ui: MenuUi,
    /// The language text blocks (English, Deutsch, Francais, Italiano), copied over the program's English block by
    /// the original when a language is chosen.
    pub texts: Vec<Vec<u8>>,
}

pub struct Assets {
    pub intro: IntroPage,
    pub front: FrontEnd,
}

fn colour(v: &Value, k: &str) -> Result<Argb, String> {
    let s = v[k].as_str().ok_or(format!("intro_text: no {k}"))?;
    let n = u32::from_str_radix(s.trim_start_matches('#'), 16)
        .map_err(|_| format!("intro_text: bad {k}"))?;
    Ok(0xFF00_0000 | n)
}

fn int(v: &Value, k: &str) -> Result<i64, String> {
    v[k].as_i64().ok_or(format!("no number {k:?}"))
}

fn bytes_of(s: &str) -> Vec<u8> {
    s.chars().map(|c| c as u32 as u8).collect()
}

fn list<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>, String> {
    v[k].as_array().ok_or(format!("no list {k:?}"))
}

fn small(v: &Value, k: &str) -> Result<Vec<u8>, String> {
    Ok(list(v, k)?
        .iter()
        .map(|x| x.as_u64().unwrap_or(0) as u8)
        .collect())
}

fn item(v: &Value) -> Result<Item, String> {
    let label = if let Some(t) = v["text"].as_str() {
        Label::Text(bytes_of(t))
    } else if let Some(o) = v["text_off"].as_u64() {
        Label::TextOff(o as usize)
    } else {
        Label::None
    };
    Ok(Item {
        kind: int(v, "kind")? as u16,
        x: int(v, "x")? as i32,
        y: int(v, "y")? as i32,
        w: int(v, "w")? as i32,
        h: int(v, "h")? as i32,
        attr: int(v, "attr")? as u16,
        label,
        sprite: v["sprite"].as_u64().unwrap_or(0) as u16,
        sdx: v["dx"].as_i64().unwrap_or(0) as i32,
        sdy: v["dy"].as_i64().unwrap_or(0) as i32,
    })
}

fn menu(v: &Value) -> Result<MenuDef, String> {
    Ok(MenuDef {
        name: v["name"].as_str().unwrap_or("").to_string(),
        items: list(v, "items")?
            .iter()
            .map(item)
            .collect::<Result<_, _>>()?,
        first: int(v, "first")? as usize,
        col: int(v, "col")? as i32,
        row: int(v, "row")? as i32,
        cols: int(v, "cols")? as i32,
        rows: int(v, "rows")? as i32,
        grid: list(v, "grid")?
            .iter()
            .map(|x| x.as_i64().unwrap_or(-1) as i8)
            .collect(),
    })
}

/// The IFF CMAP holds 8-bit values (#101020); the Amiga showed the high four bits of each, so a colour appears
/// with its nibble repeated (#111122), as the reference frames show.
fn amiga_colours(mut p: Palette) -> Palette {
    for c in p.colours.iter_mut() {
        let n = |sh: u32| ((*c >> sh) & 0xF0) >> 4;
        *c = native_engine::gfx::rgb12(((n(16) << 8) | (n(8) << 4) | n(0)) as u16);
    }
    p
}

impl FrontEnd {
    pub fn load(p: &Pack) -> Result<FrontEnd, String> {
        let s = p.data("sprites")?;
        let mut names: BTreeMap<String, usize> = BTreeMap::new();
        let mut pics = Vec::new();
        let mut sprites: Vec<Option<SpriteDef>> = Vec::new();
        for set in ["base", "menu"] {
            for e in list(&s, set)? {
                let Some(pic) = e["pic"].as_str() else {
                    continue; // a blank work sprite, drawn into at run time
                };
                let idx = match names.get(pic) {
                    Some(&i) => i,
                    None => {
                        pics.push(p.bitmap(pic)?);
                        names.insert(pic.to_string(), pics.len() - 1);
                        pics.len() - 1
                    }
                };
                let n = int(e, "n")? as usize;
                if sprites.len() <= n {
                    sprites.resize(n + 1, None);
                }
                sprites[n] = Some(SpriteDef {
                    pic: idx,
                    x: int(e, "x")? as i32,
                    y: int(e, "y")? as i32,
                    w: int(e, "w")? as i32,
                    h: int(e, "h")? as i32,
                });
            }
        }
        let u = p.data("menu_ui")?;
        let ui = MenuUi {
            font_h: int(&u, "font_h")? as i32,
            font_index: small(&u, "font_index")?,
            font_base: list(&u, "font_base")?
                .iter()
                .map(|x| {
                    (
                        x[0].as_u64().unwrap_or(0) as u16,
                        x[1].as_u64().unwrap_or(0) as u16,
                    )
                })
                .collect(),
            shade: small(&u, "shade")?,
            flash: small(&u, "flash")?,
            language_idle: int(&u, "language_idle")? as u32,
            main_idle: int(&u, "main_idle")? as u32,
            move_delay: int(&u, "move_delay")? as u32,
            fire_delay: int(&u, "fire_delay")? as u32,
            text_right: int(&u, "text_right")? as i32,
        };
        let m = p.data("menus")?;
        let menus = list(&m, "menus")?
            .iter()
            .map(menu)
            .collect::<Result<Vec<_>, _>>()?;
        let texts = (0..4)
            .map(|i| p.bytes(&format!("text{i}")).map(<[u8]>::to_vec))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FrontEnd {
            pics,
            sprites,
            background: p.bitmap("Marble")?,
            palette: amiga_colours(p.palette("Marble.pal")?),
            menus,
            ui,
            texts,
        })
    }
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let t = p.data("intro_text")?;
        let num = |k: &str| t[k].as_i64().ok_or(format!("intro_text: no {k}"));
        let intro = IntroPage {
            lines: t["lines"]
                .as_array()
                .ok_or("intro_text: no lines")?
                .iter()
                .map(|l| l.as_str().unwrap_or("").to_string())
                .collect(),
            top: num("top")? as i32,
            step: num("step")? as i32,
            background: colour(&t, "background")?,
            ink: colour(&t, "ink")?,
            ticks: num("ticks")? as u32,
            font: p.font("intro_font")?,
        };
        Ok(Assets {
            intro,
            front: FrontEnd::load(p)?,
        })
    }
}
