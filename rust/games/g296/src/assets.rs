//! Game 296's assets and data, typed, from its asset pack (built from the recovered code and data by the extractor;
//! the pack never enters the repo). Nothing here holds game bytes: every picture, sound, table and text is loaded.

use std::collections::BTreeMap;

use native_engine::audio::Sample;
use native_engine::gfx::{Bitmap, Font, Sheet};
use native_engine::pack::Pack;
use native_engine::tracker::TrackerTables;
use serde_json::Value;

/// What a button on the HUD or the options screen does (the handler named in the game's button tables).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handler {
    IncElev,
    DecElev,
    IncHead,
    DecHead,
    SetElev,
    SetHead,
    Peace,
    Fire,
    ClearTrails,
    AddSub,
    SubSub,
    AddSilo,
    SubSilo,
    AddCity,
    SubCity,
    AddRate,
    SubRate,
    Trails,
    AddMap,
    SubMap,
    LoadMap,
    Defaults,
    Instructions,
    Play,
}

impl Handler {
    /// The handler a table entry names (the original routine's own name).
    pub fn named(s: &str) -> Option<Handler> {
        use Handler::*;
        Some(match s {
            "incelev" => IncElev,
            "decelev" => DecElev,
            "inchead" => IncHead,
            "dechead" => DecHead,
            "setelev" => SetElev,
            "sethead" => SetHead,
            "peace" => Peace,
            "fired" => Fire,
            "clearsc" => ClearTrails,
            "addsub" => AddSub,
            "subsub" => SubSub,
            "addsilo" => AddSilo,
            "subsilo" => SubSilo,
            "addcity" => AddCity,
            "subcity" => SubCity,
            "addship" => AddRate,
            "subship" => SubRate,
            "trails" => Trails,
            "addmap" => AddMap,
            "submap" => SubMap,
            "loadmap" => LoadMap,
            "defaults" => Defaults,
            "instructs" => Instructions,
            "exitopt" => Play,
            _ => return None,
        })
    }
}

/// A clickable box in screen coordinates (inclusive), and what it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub handler: Handler,
}

impl Button {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
}

/// Which option sets how many of a kind each side gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Count {
    Silos,
    Subs,
    Factories,
    Cities,
}

/// One kind of object in the placement order: the map colour it must stand on, its starting missiles, its names.
#[derive(Debug, Clone, PartialEq)]
pub struct Kind {
    pub colour: u8,
    pub missiles: i32,
    pub names: Vec<String>,
    pub count: Count,
}

pub struct Sfx {
    pub sample: Sample,
    pub volume: u8,
}

pub struct Assets {
    /// The game's 32-colour table as 12-bit colours.
    pub palette: [u16; 32],
    pub hud: Bitmap,
    pub photos: Sheet,
    pub ammo: Sheet,
    pub side: Sheet,
    pub title: Bitmap,
    pub options: Bitmap,
    pub frame: Bitmap,
    pub pieces: Sheet,
    pub pieces_mask: Sheet,
    pub mushroom: Sheet,
    pub mushroom_mask: Sheet,
    pub pointer: Bitmap,
    pub big: Font,
    pub mini: Font,
    pub maps: Vec<Bitmap>,
    pub sfx: Vec<Sfx>,
    pub music: Vec<u8>,
    pub tracker: TrackerTables,
    pub sine: Vec<i32>,
    pub cosine: Vec<i32>,
    pub buttons_repeat: Vec<Button>,
    pub buttons_click: Vec<Button>,
    pub buttons_options: Vec<Button>,
    pub head_presets: Vec<i32>,
    pub elev_presets: Vec<i32>,
    pub marker_frames: Vec<usize>,
    pub flash_frames_a: Vec<usize>,
    pub flash_frames_b: Vec<usize>,
    pub hit_frames: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub type_names: Vec<String>,
    pub text: BTreeMap<String, String>,
    /// The four win reasons (headquarters, out of ammunition, offences destroyed, peace), two lines each.
    pub reasons: Vec<(String, String)>,
    pub map_names: Vec<String>,
    pub instructions: Vec<Vec<String>>,
}

fn ints(v: &Value, k: &str) -> Result<Vec<i32>, String> {
    v[k].as_array()
        .ok_or(format!("game data: no {k}"))?
        .iter()
        .map(|x| {
            x.as_i64()
                .map(|n| n as i32)
                .ok_or(format!("game data: {k}: not a number"))
        })
        .collect()
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str().map(String::from))
        .collect()
}

fn buttons(v: &Value, k: &str) -> Result<Vec<Button>, String> {
    v[k].as_array()
        .ok_or(format!("game data: no {k}"))?
        .iter()
        .map(|b| {
            let n = |f: &str| {
                b[f].as_i64()
                    .map(|x| x as i32)
                    .ok_or(format!("{k}: no {f}"))
            };
            let h = b["handler"].as_str().unwrap_or("");
            Ok(Button {
                x1: n("x1")?,
                y1: n("y1")?,
                x2: n("x2")?,
                y2: n("y2")?,
                handler: Handler::named(h).ok_or(format!("{k}: unknown handler {h:?}"))?,
            })
        })
        .collect()
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let pal = p.data_or_palette()?;
        let d = p.data("game")?;
        let frames = |k: &str| -> Result<Vec<usize>, String> {
            Ok(ints(&d, k)?.into_iter().map(|v| v as usize).collect())
        };
        let sfx = d["sfx"]
            .as_array()
            .ok_or("game data: no sfx")?
            .iter()
            .map(|s| {
                let rate = s["rate"].as_f64().ok_or("sfx: no rate")? as f32;
                Ok(Sfx {
                    sample: p.sample(s["sample"].as_str().unwrap_or(""), rate)?,
                    volume: s["volume"].as_u64().unwrap_or(64) as u8,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let names = &d["names"];
        let kinds = d["setup_types"]
            .as_array()
            .ok_or("game data: no setup_types")?
            .iter()
            .map(|k| {
                Ok(Kind {
                    colour: k["colour"].as_u64().ok_or("setup: no colour")? as u8,
                    missiles: k["ammo"].as_i64().unwrap_or(0) as i32,
                    names: strings(&names[k["names"].as_str().unwrap_or("")]),
                    count: match k["count"].as_str() {
                        Some("numsilos") => Count::Silos,
                        Some("numsubs") => Count::Subs,
                        Some("numfacts") => Count::Factories,
                        Some("numcities") => Count::Cities,
                        c => return Err(format!("setup: unknown count {c:?}")),
                    },
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let t = &d["tracker"];
        let tracker = TrackerTables {
            periods: ints(t, "periods")?.into_iter().map(|v| v as u16).collect(),
            vibrato: ints(t, "vibrato")?.into_iter().map(|v| v as u8).collect(),
            funk: ints(t, "funk")?.into_iter().map(|v| v as u8).collect(),
        };
        let reasons = ["reason1", "reason2", "reason3", "reason4"]
            .iter()
            .map(|k| {
                let l = strings(&d["reasons"][*k]);
                (
                    l.first().cloned().unwrap_or_default(),
                    l.get(1).cloned().unwrap_or_default(),
                )
            })
            .collect();
        let a = Assets {
            palette: pal,
            hud: p.bitmap("hud")?,
            photos: p.sheet("photos")?,
            ammo: p.sheet("ammo")?,
            side: p.sheet("side")?,
            title: p.bitmap("title")?,
            options: p.bitmap("options")?,
            frame: p.bitmap("frame")?,
            pieces: p.sheet("pieces")?,
            pieces_mask: p.sheet("pieces_mask")?,
            mushroom: p.sheet("mushroom")?,
            mushroom_mask: p.sheet("mushroom_mask")?,
            pointer: p.bitmap("pointer")?,
            big: p.font("big")?,
            mini: p.font("mini")?,
            maps: (1..=3)
                .map(|i| p.bitmap(&format!("map{i}")))
                .collect::<Result<_, _>>()?,
            sfx,
            music: p.music("music")?.to_vec(),
            tracker,
            sine: ints(&d, "sine")?,
            cosine: ints(&d, "cosine")?,
            buttons_repeat: buttons(&d, "buttons_repeat")?,
            buttons_click: buttons(&d, "buttons_click")?,
            buttons_options: buttons(&d, "buttons_options")?,
            head_presets: ints(&d, "head_presets")?,
            elev_presets: ints(&d, "elev_presets")?,
            marker_frames: frames("marker_frames")?,
            flash_frames_a: frames("flash_frames_a")?,
            flash_frames_b: frames("flash_frames_b")?,
            hit_frames: frames("hit_frames")?,
            kinds,
            type_names: strings(&d["type_names"]),
            text: d["text"]
                .as_object()
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            reasons,
            map_names: strings(&d["map_names"]),
            instructions: d["instructions"]
                .as_array()
                .into_iter()
                .flatten()
                .map(strings)
                .collect(),
        };
        a.check()?;
        Ok(a)
    }

    fn check(&self) -> Result<(), String> {
        let ok = self.sine.len() >= 360
            && self.cosine.len() >= 360
            && self.kinds.len() == 8
            && self.type_names.len() == 8
            && self.sfx.len() == 5
            && self.marker_frames.len() == 8
            && self.flash_frames_a.len() == 8
            && self.flash_frames_b.len() == 8
            && self.hit_frames.len() == 8
            && self.head_presets.len() == 4
            && self.elev_presets.len() == 4
            && self.maps.len() == 3
            && self
                .maps
                .iter()
                .all(|m| (m.w, m.h) == (crate::gtw::WORLD_W, crate::gtw::WORLD_H))
            && (self.hud.w, self.hud.h) == (320, 50)
            && self.reasons.len() == 4
            && self.map_names.len() == 3;
        if !ok {
            return Err("game 296 asset pack: tables or pictures have unexpected sizes".into());
        }
        Ok(())
    }

    pub fn text(&self, k: &str) -> &str {
        self.text.get(k).map(String::as_str).unwrap_or("")
    }
}

/// The palette as 12-bit colours (the game fades its colours one 12-bit step at a time).
trait Rgb12Palette {
    fn data_or_palette(&self) -> Result<[u16; 32], String>;
}

impl Rgb12Palette for Pack {
    fn data_or_palette(&self) -> Result<[u16; 32], String> {
        self.entry("main", "palette")?;
        let v: Value =
            serde_json::from_slice(self.bytes("main")?).map_err(|e| format!("palette: {e}"))?;
        let mut out = [0u16; 32];
        let list = v["rgb12"].as_array().ok_or("palette: no rgb12")?;
        if list.len() != 32 {
            return Err("palette: need 32 colours".into());
        }
        for (o, c) in out.iter_mut().zip(list) {
            *o = u16::from_str_radix(c.as_str().unwrap_or(""), 16)
                .map_err(|_| "palette: bad colour")?;
        }
        Ok(out)
    }
}
