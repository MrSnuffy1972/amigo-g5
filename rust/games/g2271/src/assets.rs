//! Game 2271's assets and data, typed, from its asset pack (built by the extractor from the original disk and the
//! recovered program; the pack never enters the repo). Nothing here holds game bytes.

use std::collections::BTreeMap;
use std::sync::Arc;

use native_engine::gfx::{Argb, Bitmap};
use native_engine::pack::Pack;
use serde_json::Value;

/// The picture the game shows: 320 by 200 (the top 24 lines are the status panel).
pub const W: usize = 320;
pub const H: usize = 200;
/// Lines of the status panel at the top of the screen (screen rows 0 to 2).
pub const PANEL_H: usize = 24;

/// One entry of the room table: where the player starts when a room is entered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    /// First map column on screen.
    pub scroll: i16,
    /// Player column and row.
    pub x: i16,
    pub y: i16,
    /// 1: facing right.
    pub facing: i16,
    /// Screen file number.
    pub van: i16,
}

/// A screen file: tile map, attribute map (collision, stairs, spawn marks) and its 8 by 8 tiles (32 colours).
#[derive(Debug, Clone, Default)]
pub struct ScreenFile {
    pub w: usize,
    pub h: usize,
    pub tiles: Vec<u8>,
    pub attrs: Vec<i8>,
    pub tile_px: Vec<[u8; 64]>,
}

impl ScreenFile {
    pub fn tile(&self, col: usize, row: usize) -> u8 {
        self.tiles.get(row * self.w + col).copied().unwrap_or(0)
    }
    /// The attribute byte at flat offset `i` (row times width plus column), as the original's pointer arithmetic
    /// read it; 0 outside the map.
    pub fn at(&self, i: i32) -> i8 {
        at(&self.attrs, i)
    }
    pub fn attr(&self, col: i32, row: i32) -> i8 {
        if col < 0 || row < 0 {
            return 0;
        }
        self.attrs
            .get(row as usize * self.w + col as usize)
            .copied()
            .unwrap_or(0)
    }
}

/// One frame of a shape file: the picture (255 = not drawn) and its header words.
#[derive(Debug, Clone)]
pub struct Shape {
    pub w: usize,
    pub h: usize,
    /// Subtracted from x and y when drawn (the frame's anchor).
    pub ax: i16,
    pub ay: i16,
    /// Header words +8 and +A: the width and height the game uses for clipping and restore boxes.
    pub cw: i16,
    pub ch: i16,
    pub px: Vec<u8>,
}

/// The title menu's numbers.
#[derive(Debug, Clone)]
pub struct Title {
    pub ease: [i16; 16],
    pub palette: usize,
    pub cursor_x: i16,
    pub top: i16,
    pub bottom: i16,
    pub split: i16,
}

/// The stage engine's tables, from the program's data (see the extractor for where each one lives).
#[derive(Debug, Clone, Default)]
pub struct Tables {
    /// Animations: row r's frames at 9 + 20r (99 ends a row), its next-animation map by move state at 20r.
    pub anim: Vec<u8>,
    /// Jump steps (1/256 rows a frame; 1000 ends) and the knock-back arc when hit.
    pub jump: Vec<i16>,
    pub knock: Vec<i16>,
    /// Exits: 12 words a screen file, 4 exits of (room, door kind, mode).
    pub exits: Vec<i16>,
    /// Hundreds of seconds by stage, whip length by level, first room of a block, restart room by stage, stage
    /// by screen file, death animation by facing, the animation set when hit.
    pub time: Vec<u8>,
    pub whip: Vec<u8>,
    pub block_room: Vec<u8>,
    pub stage_room: Vec<u8>,
    pub van_stage: Vec<u8>,
    pub death_anim: Vec<u8>,
    pub hurt_anim: u8,
}

/// The value at `i` of a table, 0 past its end (the original read on into the next table).
pub fn at<T: Copy + Default>(v: &[T], i: i32) -> T {
    if i < 0 {
        return T::default();
    }
    v.get(i as usize).copied().unwrap_or_default()
}

impl Tables {
    /// Frame `i` of animation `row` (a castshps frame number; 99 ends the animation).
    pub fn frame(&self, row: i16, i: i16) -> u8 {
        at(&self.anim, 9 + 20 * row as i32 + i as i32)
    }
    /// The animation that follows `row` in move state `s` (+90EC lookup_tile_byte, +3E78).
    pub fn next(&self, row: i16, s: i16) -> i16 {
        at(&self.anim, 20 * row as i32 + s as i32) as i16
    }
}

pub struct Assets {
    /// 32 colours a palette (12-bit): 0 to 15 the sprite colours, 16 to 31 the room's.
    pub palettes: Vec<[u16; 32]>,
    pub rooms: Vec<Room>,
    pub screens: BTreeMap<usize, ScreenFile>,
    pub shapes: BTreeMap<String, Vec<Shape>>,
    /// The 8 by 8 font from the space character: one value (0 or 1) a pixel.
    pub font: Vec<[u8; 64]>,
    pub intro: Vec<String>,
    pub intro_seconds: u32,
    pub title: Title,
    pub tables: Tables,
    /// The program's initial data area (its data hunk), for the object state (`objects::Mem`).
    pub dat: Vec<u8>,
    /// The disk's sounds/ files by lower-case name (tunes, instruments, the title module, the 8SVX effects), read by
    /// the game's own loaders in `sound`.
    pub sounds: Arc<BTreeMap<String, Arc<Vec<i8>>>>,
    /// Packed bytes of each screen file (VANn), for the time the original took to depack it.
    pub screen_bytes: BTreeMap<usize, usize>,
}

/// Every file of the disk's sounds/ directory (asset `snd_<name>` with the dot made an underscore).
pub const SOUND_FILES: [&str; 34] = [
    "3rdblock.mus",
    "4thblock.mus",
    "5thblock.mus",
    "bonus.eff",
    "snare6",
    "allclearbgm.mus",
    "allcleared.mus",
    "badkill.eff",
    "bgm1.mus",
    "bgm2.mus",
    "bgm6.mus",
    "bosscharacter.mus",
    "btime.eff",
    "cvbass1",
    "cvbassdrum",
    "cvsolo1",
    "cvstart",
    "cvstart.mus",
    "entry.mus",
    "explay.eff",
    "explod.eff",
    "falling.eff",
    "gameover.mus",
    "get.eff",
    "hellojarre.mod",
    "holybur.eff",
    "lwvill2.eff",
    "lwvillam.eff",
    "monhit.eff",
    "sack.eff",
    "splash.eff",
    "stagecleared.mus",
    "timeout.eff",
    "whip.eff",
];

fn num(v: &Value, k: &str) -> Result<i64, String> {
    v.get(k)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing number {k}"))
}

fn rgb_to_12(c: Argb) -> u16 {
    let r = ((c >> 16) & 0xFF) as u16 / 17;
    let g = ((c >> 8) & 0xFF) as u16 / 17;
    let b = (c & 0xFF) as u16 / 17;
    (r << 8) | (g << 4) | b
}

/// The frames of a sheet image (side by side), each as its own row-major pixels.
fn frames(b: &Bitmap, fw: usize, fh: usize, n: usize) -> Vec<Vec<u8>> {
    (0..n)
        .map(|f| {
            let mut v = Vec::with_capacity(fw * fh);
            for y in 0..fh {
                v.extend_from_slice(&b.px[y * b.w + f * fw..y * b.w + f * fw + fw]);
            }
            v
        })
        .collect()
}

fn ints(v: &Value, k: &str) -> Result<Vec<i64>, String> {
    v.get(k)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing list {k}"))?
        .iter()
        .map(|x| x.as_i64().ok_or_else(|| format!("bad number in {k}")))
        .collect()
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let mut palettes = Vec::new();
        while let Ok(pal) = p.palette(&format!("pal.{}", palettes.len())) {
            let mut c = [0u16; 32];
            for (i, v) in pal.colours.iter().take(32).enumerate() {
                c[i] = rgb_to_12(*v);
            }
            palettes.push(c);
        }
        if palettes.is_empty() {
            return Err("no palettes in the pack".into());
        }
        let rooms = p
            .data("rooms")?
            .as_array()
            .ok_or("rooms: not a list")?
            .iter()
            .map(|r| {
                Ok(Room {
                    scroll: num(r, "scroll")? as i16,
                    x: num(r, "x")? as i16,
                    y: num(r, "y")? as i16,
                    facing: num(r, "facing")? as i16,
                    van: num(r, "van")? as i16,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut screens = BTreeMap::new();
        for n in 0..38 {
            let Ok(d) = p.data(&format!("van.{n}")) else {
                continue;
            };
            let sheet = p.sheet(&format!("tiles.{n}"))?;
            let tile_px = frames(&sheet.image, 8, 8, sheet.frames)
                .into_iter()
                .map(|v| {
                    let mut t = [0u8; 64];
                    t.copy_from_slice(&v);
                    t
                })
                .collect();
            screens.insert(
                n,
                ScreenFile {
                    w: num(&d, "w")? as usize,
                    h: num(&d, "h")? as usize,
                    tiles: ints(&d, "tiles")?.into_iter().map(|v| v as u8).collect(),
                    attrs: ints(&d, "attrs")?.into_iter().map(|v| v as i8).collect(),
                    tile_px,
                },
            );
        }
        let mut shapes = BTreeMap::new();
        let sd = p.data("shapes")?;
        for (name, list) in sd.as_object().ok_or("shapes: not an object")? {
            let mut v = Vec::new();
            for (i, f) in list
                .as_array()
                .ok_or("shapes: bad entry")?
                .iter()
                .enumerate()
            {
                let h = f.as_array().ok_or("shapes: bad frame")?;
                let g = |k: usize| h.get(k).and_then(Value::as_i64).unwrap_or(0);
                let b = p.bitmap(&format!("art.{name}.{i}"))?;
                v.push(Shape {
                    w: g(0) as usize,
                    h: g(1) as usize,
                    ax: g(2) as i16,
                    ay: g(3) as i16,
                    cw: g(4) as i16,
                    ch: g(5) as i16,
                    px: b.px,
                });
            }
            shapes.insert(name.clone(), v);
        }
        let fs = p.sheet("font")?;
        let font = frames(&fs.image, 8, 8, fs.frames)
            .into_iter()
            .map(|v| {
                let mut t = [0u8; 64];
                t.copy_from_slice(&v);
                t
            })
            .collect();
        let it = p.data("intro")?;
        let intro = it
            .get("lines")
            .and_then(Value::as_array)
            .ok_or("intro: no lines")?
            .iter()
            .map(|l| l.as_str().unwrap_or("").to_string())
            .collect();
        let tt = p.data("tables")?;
        let bytes = |k: &str| -> Result<Vec<u8>, String> {
            Ok(ints(&tt, k)?.into_iter().map(|v| v as u8).collect())
        };
        let words = |k: &str| -> Result<Vec<i16>, String> {
            Ok(ints(&tt, k)?.into_iter().map(|v| v as i16).collect())
        };
        let tables = Tables {
            anim: bytes("anim")?,
            jump: words("jump")?,
            knock: words("knock")?,
            exits: words("exits")?,
            time: bytes("time")?,
            whip: bytes("whip")?,
            block_room: bytes("block_room")?,
            stage_room: bytes("stage_room")?,
            van_stage: bytes("van_stage")?,
            death_anim: bytes("death_anim")?,
            hurt_anim: bytes("hurt_anim")?.first().copied().unwrap_or(0),
        };
        let td = p.data("title")?;
        let e = ints(&td, "ease")?;
        let mut ease = [0i16; 16];
        for (i, v) in e.iter().take(16).enumerate() {
            ease[i] = *v as i16;
        }
        let mut sounds = BTreeMap::new();
        for n in SOUND_FILES {
            let s = p.sample(&format!("snd_{}", n.replace('.', "_")), 8000.0)?;
            sounds.insert(n.to_string(), s.data.clone());
        }
        let mut screen_bytes = BTreeMap::new();
        if let Ok(v) = p.data("screen_bytes") {
            if let Some(o) = v.as_object() {
                for (k, n) in o {
                    if let (Ok(k), Some(n)) = (k.parse::<usize>(), n.as_u64()) {
                        screen_bytes.insert(k, n as usize);
                    }
                }
            }
        }
        Ok(Assets {
            screen_bytes,
            sounds: Arc::new(sounds),
            palettes,
            rooms,
            screens,
            shapes,
            font,
            intro,
            intro_seconds: num(&it, "seconds")? as u32,
            title: Title {
                ease,
                palette: num(&td, "palette")? as usize,
                cursor_x: num(&td, "cursor_x")? as i16,
                top: num(&td, "top")? as i16,
                bottom: num(&td, "bottom")? as i16,
                split: num(&td, "split")? as i16,
            },
            tables,
            dat: ints(&p.data("datahunk")?, "bytes")?
                .into_iter()
                .map(|v| v as u8)
                .collect(),
        })
    }

    pub fn shape(&self, file: &str, frame: usize) -> Option<&Shape> {
        self.shapes.get(file).and_then(|v| v.get(frame))
    }

    pub fn screen(&self, n: usize) -> Result<&ScreenFile, String> {
        self.screens
            .get(&n)
            .ok_or_else(|| format!("screen file {n} is not in the pack"))
    }
}
