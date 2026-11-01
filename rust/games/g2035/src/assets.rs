//! The game's assets, read once from the asset pack.

use native_engine::gfx::{Argb, Font};
use native_engine::pack::Pack;

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

pub struct Assets {
    pub intro: IntroPage,
}

fn colour(v: &serde_json::Value, k: &str) -> Result<Argb, String> {
    let s = v[k].as_str().ok_or(format!("intro_text: no {k}"))?;
    let n = u32::from_str_radix(s.trim_start_matches('#'), 16)
        .map_err(|_| format!("intro_text: bad {k}"))?;
    Ok(0xFF00_0000 | n)
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
        Ok(Assets { intro })
    }
}
