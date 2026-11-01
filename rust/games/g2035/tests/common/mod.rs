//! A small synthetic asset pack for the tests (no game bytes in the repo): the intro, and a front end whose two
//! menus have the shapes of the original's (a 4 x 4 language grid round four boxes, a main menu of buttons).
#![allow(dead_code)]

use std::collections::BTreeMap;

use native_engine::audio::Mixer;
use native_engine::game::{Ctx, Game};
use native_engine::input::{Action, InputState};
use native_engine::pack::Pack;
use native_engine::save::Save;
use serde_json::{json, Value};

pub fn pack() -> Pack {
    let m = json!({"format": "native-asset-pack/1", "game": "02035", "title": "Sensible Soccer", "assets": [
        {"kind": "data", "name": "intro_text", "path": "t.json"},
        {"kind": "font", "name": "intro_font", "path": "f.idx", "glyph_w": 8, "glyph_h": 8, "first": 65, "count": 1},
        {"kind": "image", "name": "Marble", "path": "m.idx", "w": 352, "h": 272},
        {"kind": "palette", "name": "Marble.pal", "path": "m.json"},
        {"kind": "image", "name": "glyphs", "path": "g.idx", "w": 32, "h": 8},
        {"kind": "data", "name": "sprites", "path": "s.json"},
        {"kind": "data", "name": "menus", "path": "menus.json"},
        {"kind": "data", "name": "menu_ui", "path": "ui.json"},
        {"kind": "blob", "name": "text0", "path": "t0"}, {"kind": "blob", "name": "text1", "path": "t1"},
        {"kind": "blob", "name": "text2", "path": "t2"}, {"kind": "blob", "name": "text3", "path": "t3"}]});
    let mut f = BTreeMap::new();
    let j = |v: Value| serde_json::to_vec(&v).unwrap();
    f.insert(
        "intro_text".to_string(),
        j(json!({"lines": ["A", "", "AA"], "top": 23, "step": 10,
            "background": "#550044", "ink": "#FF0077", "ticks": 70})),
    );
    f.insert("intro_font".to_string(), vec![1u8; 64]);
    f.insert("Marble".to_string(), vec![0u8; 352 * 272]);
    let cols: Vec<String> = (0..16).map(|i| format!("#{:02X}0000", i * 16)).collect();
    f.insert("Marble.pal".to_string(), j(json!({ "colours": cols })));
    // glyph "A": 4 x 5, face colour 10 with a shadow pixel of colour 8; a flag 4 x 4 of colour 3 at (16, 0)
    let mut g = vec![0u8; 32 * 8];
    for y in 0..5 {
        for x in 0..3 {
            g[y * 32 + x] = 10;
        }
        g[y * 32 + 3] = 8;
    }
    for y in 0..4 {
        for x in 16..20 {
            g[y * 32 + x] = 3;
        }
    }
    f.insert("glyphs".to_string(), g);
    f.insert(
        "sprites".to_string(),
        j(json!({"base": [], "menu": [
            {"n": 5, "pic": "glyphs", "x": 0, "y": 0, "w": 4, "h": 5},
            {"n": 7, "pic": "glyphs", "x": 16, "y": 0, "w": 4, "h": 4},
            {"n": 9, "pic": null, "x": 0, "y": 0, "w": 8, "h": 8}]})),
    );
    let bx = |x: i32, y: i32| json!({"kind": 10, "x": x, "y": y, "w": 80, "h": 79, "attr": 0x8F06});
    let language = json!({"name": "language", "first": 0, "col": 1, "row": 1, "cols": 4, "rows": 4,
        "grid": [-1, -1, -1, -1, -1, 0, 1, -1, -1, 2, 3, -1, -1, -1, -1, -1],
        "items": [bx(11, 39), bx(247, 39), bx(11, 142), bx(247, 142),
            {"kind": 4, "x": 33, "y": 52, "w": 4, "h": 4, "attr": 0x0F06, "sprite": 7, "dx": 0, "dy": 0},
            {"kind": 11, "x": 15, "y": 96, "w": 72, "h": 5, "attr": 0x0F06, "text": "A"}]});
    let button = |x: i32, y: i32, o: i32| json!({"kind": 11, "x": x, "y": y, "w": 130, "h": 36, "attr": 0x8605, "text_off": o});
    let main = json!({"name": "main", "first": 0, "col": 1, "row": 1, "cols": 4, "rows": 4,
        "grid": [-1, -1, -1, -1, -1, 0, 2, -1, -1, 1, -3, -1, -1, 3, -1, -1],
        "items": [button(29, 56, 0), button(29, 96, 2), button(179, 56, 5), button(29, 136, 0)]});
    f.insert("menus".to_string(), j(json!({ "menus": [language, main] })));
    let mut idx = vec![255u8; 96];
    idx[33] = 0;
    f.insert(
        "menu_ui".to_string(),
        j(json!({"font_h": 5, "font_index": idx, "font_base": [[6, 5], [10, 5], [15, 5]],
            "shade": [255, 7, 255, 255, 3, 4, 5, 8, 3, 8, 12, 3, 3, 11, 3, 6], "flash": [3, 3, 3, 1, 1, 2, 1, 1],
            "language_idle": 1500, "main_idle": 375, "move_delay": 4, "fire_delay": 8, "text_right": 336})),
    );
    for (i, t) in [
        &b"A\0AA\0AAA\0"[..],
        b"AA\0A\0A A\0",
        b"A\0A\0A\0",
        b"A\0A\0A\0",
    ]
    .iter()
    .enumerate()
    {
        f.insert(format!("text{i}"), t.to_vec());
    }
    Pack::new(m, f).unwrap()
}

/// The named state as a map.
pub fn state(g: &dyn Game) -> BTreeMap<String, i64> {
    g.state().into_iter().collect()
}

/// One tick with `held` actions held (pressed = newly held since `prev`).
pub fn step(g: &mut dyn Game, prev: &[Action], held: &[Action]) {
    let bits = |v: &[Action]| v.iter().fold(0, |b, a| b | a.bit());
    let input = InputState {
        held: bits(held),
        pressed: bits(held) & !bits(prev),
        released: bits(prev) & !bits(held),
        motion: (0, 0),
        absolute: None,
        held2: 0,
    };
    let mut audio = Mixer::new(44100, 50, 4);
    let mut save = Save::memory();
    let mut cx = Ctx {
        input: &input,
        audio: &mut audio,
        save: &mut save,
        tick: 0,
    };
    g.tick(&mut cx);
}

/// `n` ticks holding `held` throughout (the first one presses it).
pub fn hold(g: &mut dyn Game, held: &[Action], n: u32) {
    for k in 0..n {
        step(g, if k == 0 { &[] } else { held }, held);
    }
}
