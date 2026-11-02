//! Draws the match arena's view at a camera position from the asset pack and writes it as raw RGB (320 x 184, three
//! bytes a pixel), for checking against the reference recording.
//!   cargo run --example arena_view -- PACK CAMX CAMY OUT.rgb

use g2055::arena::{Arena, Camera, VIEW_H, VIEW_W};
use native_engine::gfx::Bitmap;
use native_engine::pack::Pack;

fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage: arena_view PACK CAMX CAMY OUT.rgb".into());
    }
    let path = std::path::Path::new(&a[1]);
    let p = if path.is_dir() {
        Pack::from_dir(path)?
    } else {
        Pack::from_bytes(&std::fs::read(path).map_err(|e| e.to_string())?)?
    };
    let arena = Arena::load(&p)?;
    let n = |s: &str| s.parse::<i16>().map_err(|_| format!("{s}: not a number"));
    let cam = Camera {
        x: n(&a[2])?,
        y: n(&a[3])?,
    };
    let mut b = Bitmap::new(VIEW_W, VIEW_H);
    arena.draw_view(&mut b, cam, 0);
    let mut out = Vec::with_capacity(VIEW_W * VIEW_H * 3);
    for c in &b.px {
        let v = arena.pal[*c as usize & 31];
        out.extend([
            ((v >> 8) & 15) as u8 * 17,
            ((v >> 4) & 15) as u8 * 17,
            (v & 15) as u8 * 17,
        ]);
    }
    std::fs::write(&a[4], out).map_err(|e| e.to_string())
}
