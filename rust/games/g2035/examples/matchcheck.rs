//! Verification helper for the match parts (used by the checks outside the repo against reference recordings).
//! `matchcheck PACK view PITCH_TYPE CX CY OUT` writes the pitch view as 352 x 272 RGBA;
//! `matchcheck PACK cam < LINES` steps the camera once per JSON line and prints the result per line
//! (line: {"ball":[x,y],"v":[vx,vy],"fixed":0|1,"focus":null|[x,y],"aim":"ball"|0..7|null,"event":n,"cam":[x,y,lx,ly]});
//! `matchcheck PACK pitch MONTH SEED CALLS FRAME` prints the pitch type chosen and the random state after.

use std::io::BufRead;

use g2035::assets::{Assets, H, W};
use g2035::camera::{Aim, CamInput, Camera};
use g2035::pitch;
use g2035::rng::Rng;
use native_engine::gfx::Frame;
use native_engine::pack::Pack;
use serde_json::Value;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let pack = Pack::from_dir(std::path::Path::new(&a[1])).expect("pack");
    let assets = Assets::load(&pack).expect("assets");
    let md = assets.matchd.as_ref().expect("match tables in the pack");
    let n = |i: usize| a[i].parse::<i64>().expect("number");
    match a[2].as_str() {
        "view" => {
            let mut f = Frame::new(W, H);
            pitch::render(md, n(3) as u16, n(4) as i32, n(5) as i32, &mut f);
            let mut out = Vec::with_capacity(W * H * 4);
            for &c in &f.image.px {
                let v = f.palette.colours[c as usize];
                out.extend_from_slice(&[(v >> 16) as u8, (v >> 8) as u8, v as u8, 255]);
            }
            std::fs::write(&a[6], out).expect("write");
        }
        "cam" => {
            for l in std::io::stdin().lock().lines() {
                let v: Value = serde_json::from_str(&l.expect("line")).expect("json");
                let g = |k: &str, i: usize| v[k][i].as_i64().unwrap_or(0);
                let mut c = Camera {
                    x: g("cam", 0) as i32,
                    y: g("cam", 1) as i32,
                    look_x: g("cam", 2) as i16,
                    look_y: g("cam", 3) as i16,
                };
                let i = CamInput {
                    ball: (g("ball", 0) as i16, g("ball", 1) as i16),
                    ball_v: (g("v", 0) as i32, g("v", 1) as i32),
                    fixed: v["fixed"].as_i64().unwrap_or(0) != 0,
                    focus: if v["focus"].is_array() {
                        Some((g("focus", 0) as i16, g("focus", 1) as i16))
                    } else {
                        None
                    },
                    aim: match &v["aim"] {
                        Value::String(_) => Aim::Ball,
                        Value::Number(d) => Aim::Facing(d.as_i64().unwrap_or(0) as u8),
                        _ => Aim::None,
                    },
                    event: v["event"].as_i64().unwrap_or(0) as u16,
                };
                c.step(&i, &md.aim);
                println!("{} {} {} {}", c.x, c.y, c.look_x, c.look_y);
            }
        }
        "pitch" => {
            let mut r = Rng {
                seed: n(4) as u8,
                calls: n(5) as u16,
            };
            let t = md.choose_pitch_type(n(3) as i16, &mut r, n(6) as u16);
            println!("{} {} {}", t, r.seed, r.calls);
        }
        _ => eprintln!("matchcheck PACK view|cam|pitch ..."),
    }
}
