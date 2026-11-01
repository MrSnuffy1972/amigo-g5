//! Loads game 2179's asset pack into typed tables once. Everything comes from the pack (extracted outside the repo
//! from the original's own disk); a missing or malformed entry fails loudly.

use std::collections::HashMap;
use std::sync::Arc;

use native_engine::gfx::Bitmap;
use native_engine::pack::Pack;
use serde_json::Value;

use crate::music::Song;

/// The logical picture: a PAL low-resolution screen of 320 by 200 (the display window the game uses).
pub const W: usize = 320;
pub const H: usize = 200;
/// The colour index the pack uses for a bob's transparent pixels (the original's mask bit clear).
pub const CLEAR: u8 = 16;

/// A bob, scenery block or background pattern: palette indices (16 = transparent) and the width its header gives
/// (the enemies' collision boxes use it).
pub struct Img {
    pub bmp: Bitmap,
    pub pw: i32,
}

/// One frame of an enemy or item animation: the picture and where it sits against the object's position.
#[derive(Clone, Copy)]
pub struct AnimFrame {
    pub img: usize,
    pub dx: i32,
    pub dy: i32,
}

/// An enemy or item type (the original's 48-byte template).
#[derive(Clone)]
pub struct Template {
    pub dx: i32,
    pub dy: i32,
    pub shift: u32,
    /// Walking right and left, killed right and left, squashed right and left (indices into `anims`).
    pub anim: [usize; 6],
    pub kind: i32,
    pub hp: i32,
    /// Points as packed decimal (the low three bytes are added to the score).
    pub points: u32,
    pub movement: i32,
    pub path: Vec<i32>,
    pub xoff: i32,
    pub yoff: i32,
}

/// A stage: the map of 8 by 8 tiles, its palette, its background pattern, scenery blocks and tiles to draw, and the
/// named object lists (bridge, thorns, diamond blocks, power blocks, flames, hint).
#[derive(Clone)]
pub struct Stage {
    pub w: usize,
    pub h: usize,
    pub map: Vec<u8>,
    pub palette: [u16; 16],
    pub tiles: Vec<(usize, i32, i32)>,
    pub scenery: Vec<(usize, i32, i32)>,
    pub background: usize,
    /// bruecke2, dornen, bonus, bonus1, flamme, hinweis.
    pub objects: [Option<Vec<(u16, u16)>>; 6],
}

pub struct Sfx {
    pub data: Arc<Vec<i8>>,
    pub period: u16,
    pub volume: u8,
}

pub struct Picture {
    pub bmp: Bitmap,
    pub depth: u32,
    pub colours: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HsEntry {
    pub name: [u8; 10],
    /// Packed decimal score.
    pub score: u32,
    pub stage: u32,
}

/// Frame-set indices of the player (the original's eight frame-table pointers).
pub const WALK_R: usize = 0;
pub const WALK_L: usize = 1;
pub const SKID_R: usize = 2;
pub const SKID_L: usize = 3;
pub const JUMP_R: usize = 4;
pub const JUMP_L: usize = 5;
pub const STAND_R: usize = 6;
pub const STAND_L: usize = 7;

/// The texts the game prints, as the original holds them.
#[derive(Clone, Default)]
pub struct Texts {
    pub get_ready: Vec<u8>,
    pub out_of_time: Vec<u8>,
    pub warp: Vec<u8>,
    pub credits: Vec<u8>,
    pub tally1: Vec<u8>,
    pub tally2: Vec<u8>,
    pub wake: Vec<u8>,
    pub game_over: Vec<u8>,
    pub stage: Vec<u8>,
    pub lives: Vec<u8>,
    pub bonus: Vec<u8>,
    pub time: Vec<u8>,
    pub giana: Vec<u8>,
    pub maria: Vec<u8>,
    pub hs_tables: Vec<u8>,
    pub enter_name: Vec<u8>,
    pub save_hint: Vec<u8>,
}

pub struct Assets {
    pub texts: Texts,
    /// The title's and high-score screen's colour bar, one colour a line.
    pub title_bar: Vec<u16>,
    /// The high-score screen's sideways wobble, one value a line.
    pub hs_wobble: Vec<i32>,
    pub imgs: Vec<Img>,
    /// Player frames: [normal, powered][frame table] -> pictures.
    pub frame_sets: [[Vec<usize>; 8]; 2],
    pub block_shimmer: Vec<usize>,
    pub bump: Vec<usize>,
    pub multi: Vec<usize>,
    pub debris: Vec<usize>,
    pub breaking: Vec<usize>,
    pub bridge: Vec<usize>,
    pub dornen: Vec<usize>,
    pub flamme: Vec<usize>,
    pub transform: Vec<usize>,
    pub death: Vec<usize>,
    pub shot: Vec<usize>,
    pub explosion: Vec<usize>,
    pub diamond_gone: usize,
    pub multi_done: usize,
    pub lift: usize,
    pub flying_diamond: usize,
    pub dropped_block: usize,
    pub dropped_gone: usize,
    pub bridge_piece: usize,
    pub anims: Vec<Vec<AnimFrame>>,
    pub templates: Vec<Template>,
    pub items: Vec<Template>,
    /// Which power counter each item raises (0 power-up, 1 shot level, 2 freeze, 3 kill all, 4 protection, 5 life).
    pub item_flag: Vec<usize>,
    pub enemy_lists: Vec<Vec<(u16, u16, u16)>>,
    pub enemy_pool: usize,
    /// Horizontal movement per speed -35..35.
    pub hspeed: Vec<i32>,
    pub anim_rate: Vec<i32>,
    pub jump: Vec<i32>,
    /// The words stored before the jump table, nearest last (the fall reads below index 0).
    pub jump_before: Vec<i32>,
    pub arc_before: Vec<i32>,
    pub fall: Vec<i32>,
    pub shot_arc: Vec<i32>,
    pub death_bounce: Vec<i32>,
    pub diamond_path: Vec<i32>,
    pub drop_fall: Vec<i32>,
    pub tile_class: [u8; 256],
    pub diamond_off: Vec<i32>,
    pub start_pos: Vec<i32>,
    pub sprite_colours: [u16; 4],
    pub homing_fallback: (i32, i32),
    pub stages: Vec<Stage>,
    pub bonus: Vec<Stage>,
    pub font: Bitmap,
    pub clouds: Vec<Bitmap>,
    pub sfx: Vec<Sfx>,
    pub music: Song,
    pub boot_music: Song,
    pub logo: Picture,
    pub title_picture: Picture,
    pub hs_all_time: Vec<HsEntry>,
    pub hs_today: Vec<HsEntry>,
}

fn arr<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>, String> {
    v.get(k)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("tables: {k} missing"))
}

fn ints(v: &Value, k: &str) -> Result<Vec<i32>, String> {
    arr(v, k)?
        .iter()
        .map(|x| {
            x.as_i64()
                .map(|i| i as i32)
                .ok_or(format!("{k}: not a number"))
        })
        .collect()
}

fn int(v: &Value, k: &str) -> Result<i64, String> {
    v.get(k)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("{k} missing"))
}

struct Loader<'a> {
    pack: &'a Pack,
    imgs: Vec<Img>,
    names: HashMap<String, usize>,
    pw: HashMap<String, i32>,
}

impl Loader<'_> {
    fn img(&mut self, v: &Value) -> Result<usize, String> {
        let n = v.as_str().ok_or("image name expected")?;
        if let Some(&i) = self.names.get(n) {
            return Ok(i);
        }
        let bmp = self.pack.bitmap(n)?;
        let pw = self.pw.get(n).copied().unwrap_or(bmp.w as i32);
        self.imgs.push(Img { bmp, pw });
        self.names.insert(n.to_string(), self.imgs.len() - 1);
        Ok(self.imgs.len() - 1)
    }
    fn list(&mut self, v: &Value) -> Result<Vec<usize>, String> {
        v.as_array()
            .ok_or("image list expected")?
            .iter()
            .map(|x| self.img(x))
            .collect()
    }
    fn placed(&mut self, v: &Value) -> Result<Vec<(usize, i32, i32)>, String> {
        let mut out = Vec::new();
        for e in v.as_array().ok_or("placement list expected")? {
            let e = e.as_array().ok_or("placement expected")?;
            let n = self.img(&e[0])?;
            let x = e[1].as_i64().ok_or("x")? as i32;
            let y = e[2].as_i64().ok_or("y")? as i32;
            out.push((n, x, y));
        }
        Ok(out)
    }
    fn stage(&mut self, name: &str) -> Result<Stage, String> {
        let s = self.pack.data(name)?;
        let w = int(&s, "w")? as usize;
        let h = int(&s, "h")? as usize;
        let hex = s.get("map").and_then(Value::as_str).ok_or("map")?;
        let map: Vec<u8> = (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap_or(0))
            .collect();
        let p = ints(&s, "palette")?;
        let mut palette = [0u16; 16];
        for (i, c) in p.iter().take(16).enumerate() {
            palette[i] = *c as u16;
        }
        let tiles = self.placed(s.get("tiles").ok_or("tiles")?)?;
        let scenery = self.placed(s.get("scenery").ok_or("scenery")?)?;
        let background = self.img(s.get("background").ok_or("background")?)?;
        let o = s.get("objects").ok_or("objects")?;
        let mut objects: [Option<Vec<(u16, u16)>>; 6] = Default::default();
        for (i, k) in ["bruecke2", "dornen", "bonus", "bonus1", "flamme", "hinweis"]
            .iter()
            .enumerate()
        {
            if let Some(l) = o.get(*k).and_then(Value::as_array) {
                objects[i] = Some(
                    l.iter()
                        .filter_map(|e| {
                            let e = e.as_array()?;
                            Some((e[0].as_u64()? as u16, e[1].as_u64()? as u16))
                        })
                        .collect(),
                );
            }
        }
        if map.len() < w * h {
            return Err(format!("{name}: map too short"));
        }
        Ok(Stage {
            w,
            h,
            map,
            palette,
            tiles,
            scenery,
            background,
            objects,
        })
    }
}

fn template(
    l: &mut Loader,
    v: &Value,
    anim_ids: &HashMap<String, usize>,
) -> Result<Template, String> {
    let a = |k: &str| -> usize {
        v.get(k)
            .and_then(Value::as_str)
            .and_then(|s| anim_ids.get(s).copied())
            .unwrap_or(usize::MAX)
    };
    let _ = l;
    Ok(Template {
        dx: int(v, "dx")? as i32,
        dy: int(v, "dy")? as i32,
        shift: int(v, "shift")? as u32,
        anim: [
            a("anim_r"),
            a("anim_l"),
            a("dead_r"),
            a("dead_l"),
            a("squash_r"),
            a("squash_l"),
        ],
        kind: int(v, "kind")? as i32,
        hp: int(v, "hp")? as i32,
        points: int(v, "points")? as u32,
        movement: int(v, "move")? as i32,
        path: ints(v, "path")?,
        xoff: int(v, "xoff")? as i32,
        yoff: int(v, "yoff")? as i32,
    })
}

fn hs_table(v: &Value) -> Result<Vec<HsEntry>, String> {
    v.as_array()
        .ok_or("high-score table")?
        .iter()
        .map(|e| {
            let mut name = [b' '; 10];
            let s = e.get("name").and_then(Value::as_str).unwrap_or("");
            for (i, c) in s.chars().take(10).enumerate() {
                name[i] = c as u32 as u8;
            }
            Ok(HsEntry {
                name,
                score: int(e, "score")? as u32,
                stage: int(e, "stage")? as u32,
            })
        })
        .collect()
}

impl Assets {
    pub fn load(p: &Pack) -> Result<Assets, String> {
        let t = p.data("tables")?;
        let mut pw = HashMap::new();
        if let Some(m) = t.get("images").and_then(Value::as_object) {
            for (k, v) in m {
                if let Some(x) = v.get(2).and_then(Value::as_i64) {
                    pw.insert(k.clone(), x as i32);
                }
            }
        }
        let mut l = Loader {
            pack: p,
            imgs: Vec::new(),
            names: HashMap::new(),
            pw,
        };
        let fs = t.get("frame_sets").ok_or("frame_sets")?;
        let mut frame_sets: [[Vec<usize>; 8]; 2] = Default::default();
        for (si, k) in ["normal", "powered"].iter().enumerate() {
            let sets = fs.get(*k).and_then(Value::as_array).ok_or("frame set")?;
            for (i, f) in sets.iter().enumerate().take(8) {
                frame_sets[si][i] = l.list(f)?;
            }
        }
        let an = t.get("anim").ok_or("anim")?;
        let mut g = |k: &str| -> Result<Vec<usize>, String> {
            l.list(an.get(k).ok_or(format!("anim {k}"))?)
        };
        let block_shimmer = g("block_shimmer")?;
        let bump = g("bump")?;
        let multi = g("multi")?;
        let debris = g("debris")?;
        let breaking = g("breaking")?;
        let bridge = g("bridge")?;
        let dornen = g("dornen")?;
        let flamme = g("flamme")?;
        let transform = g("transform")?;
        let death = g("death")?;
        let shot = g("shot")?;
        let explosion = g("explosion")?;
        let lit = t.get("lit").ok_or("lit")?;
        let mut li =
            |k: &str| -> Result<usize, String> { l.img(lit.get(k).ok_or(format!("lit {k}"))?) };
        let diamond_gone = li("diamond_gone")?;
        let multi_done = li("multi_done")?;
        let lift = li("lift")?;
        let flying_diamond = li("flying_diamond")?;
        let dropped_block = li("dropped_block")?;
        let dropped_gone = li("dropped_gone")?;
        let bridge_piece = li("bridge_piece")?;
        let mut anims = Vec::new();
        let mut anim_ids = HashMap::new();
        if let Some(m) = t.get("anim_tables").and_then(Value::as_object) {
            for (k, v) in m {
                let mut fr = Vec::new();
                for e in v.as_array().ok_or("anim table")? {
                    let e = e.as_array().ok_or("anim frame")?;
                    fr.push(AnimFrame {
                        img: l.img(&e[0])?,
                        dx: e[1].as_i64().unwrap_or(0) as i32,
                        dy: e[2].as_i64().unwrap_or(0) as i32,
                    });
                }
                anim_ids.insert(k.clone(), anims.len());
                anims.push(fr);
            }
        }
        let mut templates = Vec::new();
        for v in arr(&t, "templates")? {
            templates.push(template(&mut l, v, &anim_ids)?);
        }
        let mut items = Vec::new();
        for v in arr(&t, "items")? {
            items.push(template(&mut l, v, &anim_ids)?);
        }
        let item_flag = ints(&t, "item_flag")?.iter().map(|&x| x as usize).collect();
        let mut enemy_lists = Vec::new();
        for s in arr(&t, "enemy_lists")? {
            let mut v = Vec::new();
            for e in s.as_array().ok_or("enemy list")? {
                let e = e.as_array().ok_or("enemy entry")?;
                v.push((
                    e[0].as_u64().unwrap_or(0) as u16,
                    e[1].as_u64().unwrap_or(0) as u16,
                    e[2].as_u64().unwrap_or(0) as u16,
                ));
            }
            enemy_lists.push(v);
        }
        let hs = t.get("hspeed").ok_or("hspeed")?;
        let hspeed = ints(hs, "v")?;
        let mut tile_class = [0u8; 256];
        for (i, v) in ints(&t, "tile_class")?.iter().enumerate().take(256) {
            tile_class[i] = *v as u8;
        }
        let sc = ints(&t, "sprite_colours")?;
        let hf = ints(&t, "homing_fallback")?;
        let mut stages = Vec::new();
        for s in 0..34 {
            stages.push(l.stage(&format!("stage{s:02}"))?);
        }
        let mut bonus = Vec::new();
        for s in 0..5 {
            bonus.push(l.stage(&format!("bonus{s}"))?);
        }
        let font = p.bitmap("font")?;
        let clouds = (0..6)
            .map(|k| p.bitmap(&format!("cloud{k}")))
            .collect::<Result<Vec<_>, _>>()?;
        let mut sfx = Vec::new();
        for e in arr(&t, "sfx")? {
            let n = e
                .get("sample")
                .and_then(Value::as_str)
                .ok_or("sfx sample")?;
            sfx.push(Sfx {
                data: Arc::new(p.bytes(n)?.iter().map(|&b| b as i8).collect()),
                period: int(e, "period")? as u16,
                volume: int(e, "volume")?.min(64) as u8,
            });
        }
        let m = p.data("music")?;
        let music = Song::load(p, m.get("game").ok_or("music game")?, false)?;
        let boot_music = Song::load(p, m.get("boot").ok_or("music boot")?, true)?;
        let pic = |k: &str| -> Result<Picture, String> {
            let i = t.get(k).ok_or(k.to_string())?;
            Ok(Picture {
                bmp: p.bitmap(k)?,
                depth: int(i, "depth")? as u32,
                colours: ints(i, "colours")?.iter().map(|&c| c as u16).collect(),
            })
        };
        let logo = pic("logo")?;
        let title_picture = pic("title_picture")?;
        let tx = t.get("texts").ok_or("texts")?;
        let tb = |k: &str| -> Result<Vec<u8>, String> {
            Ok(ints(tx, k)?.iter().map(|&c| c as u8).collect())
        };
        let texts = Texts {
            get_ready: tb("get_ready")?,
            out_of_time: tb("out_of_time")?,
            warp: tb("warp")?,
            credits: tb("credits")?,
            tally1: tb("tally1")?,
            tally2: tb("tally2")?,
            wake: tb("wake")?,
            game_over: tb("game_over")?,
            stage: tb("stage")?,
            lives: tb("lives")?,
            bonus: tb("bonus")?,
            time: tb("time")?,
            giana: tb("giana")?,
            maria: tb("maria")?,
            hs_tables: tb("hs_tables")?,
            enter_name: tb("enter_name")?,
            save_hint: tb("save_hint")?,
        };
        Ok(Assets {
            texts,
            title_bar: ints(&t, "title_bar")?.iter().map(|&c| c as u16).collect(),
            hs_wobble: ints(&t, "hs_wobble")?,
            imgs: l.imgs,
            frame_sets,
            block_shimmer,
            bump,
            multi,
            debris,
            breaking,
            bridge,
            dornen,
            flamme,
            transform,
            death,
            shot,
            explosion,
            diamond_gone,
            multi_done,
            lift,
            flying_diamond,
            dropped_block,
            dropped_gone,
            bridge_piece,
            anims,
            templates,
            items,
            item_flag,
            enemy_lists,
            enemy_pool: int(&t, "enemy_pool")? as usize,
            hspeed,
            anim_rate: ints(&t, "anim_rate")?,
            jump: ints(&t, "jump")?,
            jump_before: ints(&t, "jump_before")?,
            arc_before: ints(&t, "arc_before")?,
            fall: ints(&t, "fall")?,
            shot_arc: ints(&t, "shot_arc")?,
            death_bounce: ints(&t, "death_bounce")?,
            diamond_path: ints(&t, "diamond_path")?,
            drop_fall: ints(&t, "drop_fall")?,
            tile_class,
            diamond_off: ints(&t, "diamond_off")?,
            start_pos: ints(&t, "start_pos")?,
            sprite_colours: [sc[0] as u16, sc[1] as u16, sc[2] as u16, sc[3] as u16],
            homing_fallback: (hf[0], hf[1]),
            stages,
            bonus,
            font,
            clouds,
            sfx,
            music,
            boot_music,
            logo,
            title_picture,
            hs_all_time: hs_table(t.get("highscores_all_time").ok_or("hs")?)?,
            hs_today: hs_table(t.get("highscores_today").ok_or("hs")?)?,
        })
    }
}
