//! E-Motion's asset pack, loaded once into typed tables. Everything of the game (pictures, sprites, font, sounds,
//! levels, texts and tables) comes from the pack at run time; nothing of it is in the source.

use std::collections::HashMap;

use native_engine::audio::Sample;
use native_engine::pack::Pack;
use serde_json::Value;

/// The output picture: 320 lores pixels by 216 lines, display lines 42 to 257 (the playfield starts on line 58).
pub const W: usize = 320;
pub const H: usize = 216;
/// The display line of output row 0.
pub const TOP_LINE: i32 = 42;
/// The playfield (the game's own screens): 320 by 196.
pub const PW: usize = 320;
pub const PH: usize = 196;
/// The sound chip's rate for the game's sounds: PAL clock / period 449 (init_audio_and_sign_samples).
pub const SAMPLE_RATE: f32 = 3_546_895.0 / 449.0;

/// An image whose pixels are colour numbers in the low bits and bit 7 set where it is drawn (its mask).
#[derive(Clone)]
pub struct Img {
    pub w: usize,
    pub h: usize,
    pub px: Vec<u8>,
}

impl Img {
    pub fn at(&self, x: usize, y: usize) -> u8 {
        self.px[y * self.w + x]
    }
}

#[derive(Clone, Debug)]
pub enum LevelObj {
    /// A tether between two objects (their byte offsets in the object table, as the level data gives them).
    Tether(u16, u16),
    Thing {
        kind: u16,
        x: i16,
        y: i16,
        colour: u16,
    },
}

#[derive(Clone, Debug)]
pub struct Level {
    pub gradient: u16,
    pub ramps: [u16; 3],
    pub bonus: u16,
    pub times: [u32; 2],
    pub pod_time: u16,
    pub ship1: (i32, i32),
    pub ship2: (i32, i32),
    pub objects: Vec<LevelObj>,
    pub hbars: Vec<[i16; 4]>,
    pub vbars: Vec<[i16; 4]>,
}

pub struct Music {
    pub blocks: Vec<Sample>,
    pub looped: usize,
}

pub struct Intro {
    pub header: Vec<(u8, u8, String)>,
    pub menu: Vec<(u8, u8, String)>,
    pub rows: Vec<u8>,
    pub scroll: Vec<u8>,
    pub wobble: Vec<u16>,
    pub gradient: Vec<u16>,
    pub colours: [u16; 3],
    pub on: String,
    pub off: String,
    pub values: Vec<u8>,
    pub periods: Vec<u16>,
}

pub struct Texts {
    pub instructions_title: String,
    pub instructions: Vec<[String; 2]>,
    pub options: Vec<String>,
    pub option_values: Vec<Vec<String>>,
    pub cont: [String; 2],
    pub bonus_awarded: String,
    pub bonus_level: Vec<String>,
    pub secret: Vec<String>,
    pub congratulations: String,
    pub game_over: String,
    pub cheat: String,
    pub high_scores: Vec<Vec<u8>>,
}

pub struct Tables {
    pub thrust: Vec<(i32, i32)>,
    pub fragments: Vec<(i32, i32)>,
    pub pal_game: Vec<u16>,
    pub pal_title: Vec<u16>,
    pub pal_boot: Vec<u16>,
    pub pal_hw: Vec<u16>,
    pub ramps_title: [u16; 3],
    pub option_max: Vec<u8>,
    pub option_default: Vec<u8>,
    pub edge_left: Vec<u16>,
    pub edge_right: Vec<u16>,
    pub seed: [u32; 2],
    pub objects_init: Vec<u16>,
}

pub struct Assets {
    pub font16: Vec<[u8; 128]>,
    pub font8: Vec<[u8; 64]>,
    pub tiles: Vec<Img>,
    pub balls: Vec<Img>,
    pub ship: Vec<Img>,
    pub heading_scores: Img,
    pub heading_name: Img,
    pub pic_title: Img,
    pub pic_preseuro: Img,
    pub pic_emotion: Img,
    pub cmap_title: Vec<u16>,
    pub cmap_preseuro: Vec<u16>,
    pub cmap_emotion: Vec<u16>,
    pub levels: Vec<Level>,
    pub layout_title: [Vec<[i16; 4]>; 2],
    pub layout_panel: [Vec<[i16; 4]>; 2],
    pub texts: Texts,
    pub tables: Tables,
    pub sfx: HashMap<String, Sample>,
    pub bump: Vec<Sample>,
    pub music: Vec<Music>,
    pub intro: Intro,
    pub soundmon: Vec<u8>,
}

fn arr<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>, String> {
    v[k].as_array().ok_or(format!("no {k}"))
}
fn nums<T: TryFrom<i64>>(v: &Value) -> Result<Vec<T>, String> {
    v.as_array()
        .ok_or("not a list".to_string())?
        .iter()
        .map(|x| {
            x.as_i64()
                .and_then(|n| T::try_from(n).ok())
                .ok_or("bad number".to_string())
        })
        .collect()
}
fn s(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}
fn strs(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().map(s).collect())
        .unwrap_or_default()
}
fn bars(v: &Value) -> Result<Vec<[i16; 4]>, String> {
    v.as_array()
        .ok_or("bars")?
        .iter()
        .map(|b| {
            let n: Vec<i16> = nums(b)?;
            Ok([n[0], n[1], n[2], n[3]])
        })
        .collect()
}
fn lines(v: &Value) -> Vec<(u8, u8, String)> {
    v.as_array()
        .map(|a| {
            a.iter()
                .map(|l| {
                    (
                        l[0].as_u64().unwrap_or(0) as u8,
                        l[1].as_u64().unwrap_or(0) as u8,
                        s(&l[2]),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn frames(p: &Pack, name: &str) -> Result<Vec<Img>, String> {
    let sh = p.sheet(name)?;
    Ok((0..sh.frames)
        .map(|f| {
            let r = sh.frame(f);
            let mut px = Vec::with_capacity(sh.frame_w * sh.frame_h);
            for y in 0..r.h {
                for x in 0..r.w {
                    px.push(sh.image.get(r.x + x, r.y + y));
                }
            }
            Img {
                w: sh.frame_w,
                h: sh.frame_h,
                px,
            }
        })
        .collect())
}

fn one(p: &Pack, name: &str) -> Result<Img, String> {
    Ok(frames(p, name)?.remove(0))
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let font16 = frames(p, "font16")?
            .into_iter()
            .map(|g| {
                let mut a = [0u8; 128];
                a.copy_from_slice(&g.px);
                a
            })
            .collect();
        let font8 = frames(p, "font8")?
            .into_iter()
            .map(|g| {
                let mut a = [0u8; 64];
                a.copy_from_slice(&g.px);
                a
            })
            .collect();
        let balls = (0..24)
            .map(|i| one(p, &format!("ball{i:02}")))
            .collect::<Result<Vec<_>, _>>()?;
        let pics = p.data("pictures")?;
        let lv = p.data("levels")?;
        let mut levels = Vec::new();
        for l in lv.as_array().ok_or("levels")? {
            let ramps: Vec<u16> = nums(&l["ramps"])?;
            let times: Vec<u32> = if l["times"].is_array() {
                nums(&l["times"])?
            } else {
                vec![0, 0]
            };
            let s1: Vec<i32> = nums(&l["ship1"])?;
            let s2: Vec<i32> = nums(&l["ship2"])?;
            let mut objects = Vec::new();
            for o in arr(l, "objects")? {
                let n: Vec<i64> = nums(o)?;
                objects.push(if n[0] == 0x10 {
                    LevelObj::Tether(n[1] as u16, n[2] as u16)
                } else {
                    LevelObj::Thing {
                        kind: n[0] as u16,
                        x: n[1] as i16,
                        y: n[2] as i16,
                        colour: n[3] as u16,
                    }
                });
            }
            levels.push(Level {
                gradient: l["gradient"].as_u64().ok_or("gradient")? as u16,
                ramps: [ramps[0], ramps[1], ramps[2]],
                bonus: l["bonus"].as_u64().ok_or("bonus")? as u16,
                times: [times[0], times[1]],
                pod_time: l["pod_time"].as_u64().ok_or("pod_time")? as u16,
                ship1: (s1[0], s1[1]),
                ship2: (s2[0], s2[1]),
                objects,
                hbars: bars(&l["hbars"])?,
                vbars: bars(&l["vbars"])?,
            });
        }
        let lay = p.data("layouts")?;
        let t = p.data("texts")?;
        let tb = p.data("tables")?;
        let pairs = |v: &Value| -> Result<Vec<(i32, i32)>, String> {
            v.as_array()
                .ok_or("pairs")?
                .iter()
                .map(|q| {
                    Ok((
                        q[0].as_i64().ok_or("p")? as i32,
                        q[1].as_i64().ok_or("p")? as i32,
                    ))
                })
                .collect()
        };
        let sf = p.data("sfx")?;
        let mut sfx = HashMap::new();
        for (k, v) in sf["named"].as_object().ok_or("sfx")? {
            sfx.insert(k.clone(), p.sample(v.as_str().ok_or("sfx")?, SAMPLE_RATE)?);
        }
        let bump = strs(&sf["bump"])
            .iter()
            .map(|n| p.sample(n, SAMPLE_RATE))
            .collect::<Result<Vec<_>, _>>()?;
        let mut music = Vec::new();
        for c in p.data("music")?.as_array().ok_or("music")? {
            music.push(Music {
                blocks: strs(&c["blocks"])
                    .iter()
                    .map(|n| p.sample(n, SAMPLE_RATE))
                    .collect::<Result<Vec<_>, _>>()?,
                looped: c["loop"].as_u64().unwrap_or(0) as usize,
            });
        }
        let it = p.data("intro")?;
        let ic: Vec<u16> = nums(&it["colours"])?;
        let ramps_title: Vec<u16> = nums(&tb["ramps_title"])?;
        let seed: Vec<u32> = nums(&tb["seed"])?;
        Ok(Assets {
            font16,
            font8,
            tiles: frames(p, "tiles")?,
            balls,
            ship: frames(p, "ship")?,
            heading_scores: one(p, "heading_scores")?,
            heading_name: one(p, "heading_name")?,
            pic_title: one(p, "pic_title")?,
            pic_preseuro: one(p, "pic_preseuro")?,
            pic_emotion: one(p, "pic_emotion")?,
            cmap_title: nums(&pics["title"])?,
            cmap_preseuro: nums(&pics["preseuro"])?,
            cmap_emotion: nums(&pics["emotion"])?,
            levels,
            layout_title: [bars(&lay["title"][0])?, bars(&lay["title"][1])?],
            layout_panel: [bars(&lay["panel"][0])?, bars(&lay["panel"][1])?],
            texts: Texts {
                instructions_title: s(&t["instructions_title"]),
                instructions: arr(&t, "instructions")?
                    .iter()
                    .map(|q| [s(&q[0]), s(&q[1])])
                    .collect(),
                options: strs(&t["options"]),
                option_values: arr(&t, "option_values")?.iter().map(strs).collect(),
                cont: [s(&t["continue"][0]), s(&t["continue"][1])],
                bonus_awarded: s(&t["bonus_awarded"]),
                bonus_level: strs(&t["bonus_level"]),
                secret: strs(&t["secret"]),
                congratulations: s(&t["congratulations"]),
                game_over: s(&t["game_over"]),
                cheat: s(&t["cheat"]),
                high_scores: arr(&t, "high_scores")?
                    .iter()
                    .map(nums)
                    .collect::<Result<Vec<_>, _>>()?,
            },
            tables: Tables {
                thrust: pairs(&tb["thrust"])?,
                fragments: pairs(&tb["fragments"])?,
                pal_game: nums(&tb["pal_game"])?,
                pal_title: nums(&tb["pal_title"])?,
                pal_boot: nums(&tb["pal_boot"])?,
                pal_hw: nums(&tb["pal_hw"])?,
                ramps_title: [ramps_title[0], ramps_title[1], ramps_title[2]],
                option_max: nums(&tb["option_max"])?,
                option_default: nums(&tb["option_default"])?,
                edge_left: nums(&tb["edge_left"])?,
                edge_right: nums(&tb["edge_right"])?,
                seed: [seed[0], seed[1]],
                objects_init: nums(&tb["objects_init"])?,
            },
            sfx,
            bump,
            music,
            intro: Intro {
                header: lines(&it["header"]),
                menu: lines(&it["menu"]),
                rows: nums(&it["rows"])?,
                scroll: nums(&it["scroll"])?,
                wobble: nums(&it["wobble"])?,
                gradient: nums(&it["gradient"])?,
                colours: [ic[0], ic[1], ic[2]],
                on: s(&it["on"]),
                off: s(&it["off"]),
                values: nums(&it["values"])?,
                periods: nums(&it["periods"])?,
            },
            soundmon: p.music("soundmon")?.to_vec(),
        })
    }
}
