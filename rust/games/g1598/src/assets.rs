//! The asset pack, typed: pictures and palettes, the object banks (golfer, flags, trees), the courses (their record
//! lists as the CRS files hold them), the mountain strips, the game's small tables and the intro text page.

use native_engine::gfx::{Bitmap, Palette};
use native_engine::pack::Pack;
use serde_json::Value;

/// The game's picture size; the program draws at twice this (`DW` x `DH`) so the intro's 80-column text fits.
pub const W: usize = 320;
pub const H: usize = 200;
pub const DW: usize = 640;
pub const DH: usize = 400;

/// One hole of a course file.
#[derive(Debug, Clone, Default)]
pub struct Hole {
    pub par: i16,
    /// Back and front tee yardages.
    pub yards: [i16; 2],
    pub tip: String,
    /// The record list: (type, words). Types 0..9 ground polygons, 0x10..0x12 tee, pro tee, aim point, 0x13..0x16
    /// the pin for each tournament day, 0x20..0x23 trees, 0x30 on contour lines.
    pub records: Vec<(i16, Vec<i16>)>,
}

#[derive(Debug, Clone, Default)]
pub struct Course {
    pub file: String,
    pub name: String,
    pub trees: String,
    pub tournament: String,
    pub holes: Vec<Hole>,
    /// The three mountain strips.
    pub mountains: Vec<Bitmap>,
    /// The course's tree bank (TR1 to TR4).
    pub tree_bank: Vec<Bitmap>,
}

#[derive(Debug, Clone, Default)]
pub struct Tables {
    pub angles: Vec<i16>,
    pub tree_dist: Vec<i16>,
    pub poly1: Vec<u8>,
    pub poly2: Vec<u8>,
}

pub struct Assets {
    pub pal_title: Palette,
    pub pal_proshop: Palette,
    pub pal_new: Palette,
    pub pal_old: Palette,
    pub title: Bitmap,
    pub proshop: Bitmap,
    pub tent: Bitmap,
    pub gr2: Vec<Bitmap>,
    pub gr3: Vec<Bitmap>,
    pub gr4: Vec<Bitmap>,
    pub courses: Vec<Course>,
    pub tables: Tables,
    /// The cracktro's console text (Amiga CSI sequences kept).
    pub intro: String,
    pub mono: fontdue::Font,
}

fn bank(p: &Pack, name: &str) -> Vec<Bitmap> {
    (0..)
        .map_while(|i| p.bitmap(&format!("obj.{name}.{i}")).ok())
        .collect()
}

fn words(v: &Value) -> Vec<i16> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_i64())
                .map(|x| x as i16)
                .collect()
        })
        .unwrap_or_default()
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let t = p.data("data.tables")?;
        let tables = Tables {
            angles: words(&t["angles"]),
            tree_dist: words(&t["tree_dist"]),
            poly1: words(&t["poly1_colors"]).iter().map(|&x| x as u8).collect(),
            poly2: words(&t["poly2_colors"]).iter().map(|&x| x as u8).collect(),
        };
        if tables.angles.len() != 16 || tables.tree_dist.len() != 11 || tables.poly1.len() != 10 {
            return Err("data.tables: wrong table sizes".into());
        }
        let mut courses = Vec::new();
        for c in p
            .data("data.courses")?
            .as_array()
            .ok_or("data.courses: not a list")?
        {
            let s = |k: &str| c[k].as_str().unwrap_or("").to_string();
            let file = s("file");
            let holes = c["holes"]
                .as_array()
                .ok_or("course without holes")?
                .iter()
                .map(|h| {
                    let y = words(&h["yards"]);
                    Hole {
                        par: h["par"].as_i64().unwrap_or(4) as i16,
                        yards: [
                            y.first().copied().unwrap_or(0),
                            y.get(1).copied().unwrap_or(0),
                        ],
                        tip: h["tip"].as_str().unwrap_or("").to_string(),
                        records: h["records"]
                            .as_array()
                            .map(|r| {
                                r.iter()
                                    .map(|e| (e[0].as_i64().unwrap_or(-1) as i16, words(&e[1])))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    }
                })
                .collect();
            let lower = file.to_lowercase();
            let mountains = (0..3)
                .map(|i| p.bitmap(&format!("img.mtn.{lower}.{i}")))
                .collect::<Result<Vec<_>, _>>()?;
            let trees = s("trees");
            courses.push(Course {
                name: s("name"),
                tournament: s("tournament"),
                tree_bank: bank(p, &trees.to_lowercase()),
                trees,
                holes,
                mountains,
                file,
            });
        }
        if courses.is_empty() {
            return Err("asset pack has no courses".into());
        }
        let intro = p.data("data.intro")?["console"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let fonts = native_engine::frontdoor::theme::Fonts::load()?;
        Ok(Assets {
            pal_title: p.palette("pal.title")?,
            pal_proshop: p.palette("pal.proshop")?,
            pal_new: p.palette("pal.new")?,
            pal_old: p.palette("pal.old")?,
            title: p.bitmap("img.title")?,
            proshop: p.bitmap("img.proshop")?,
            tent: p.bitmap("img.tent")?,
            gr2: bank(p, "gr2"),
            gr3: bank(p, "gr3"),
            gr4: bank(p, "gr4"),
            courses,
            tables,
            intro,
            mono: fonts.mono,
        })
    }
}
