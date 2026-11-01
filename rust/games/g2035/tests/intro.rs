//! Part 1 checks on a small synthetic pack (no game bytes in the repo): the intro's timing, skip and drawing.

use std::collections::BTreeMap;

use native_engine::game::Options;
use native_engine::gfx::Frame;
use native_engine::pack::Pack;
use serde_json::json;

fn pack() -> Pack {
    let m = json!({"format": "native-asset-pack/1", "game": "02035", "title": "Sensible Soccer", "assets": [
        {"kind": "data", "name": "intro_text", "path": "t.json"},
        {"kind": "font", "name": "intro_font", "path": "f.idx", "glyph_w": 8, "glyph_h": 8, "first": 65, "count": 1}]});
    let mut f = BTreeMap::new();
    f.insert(
        "intro_text".to_string(),
        serde_json::to_vec(&json!({"lines": ["A", "", "AA"], "top": 23, "step": 10,
            "background": "#550044", "ink": "#FF0077", "ticks": 70}))
        .unwrap(),
    );
    f.insert("intro_font".to_string(), vec![1u8; 64]);
    Pack::new(m, f).unwrap()
}

fn phase(g: &dyn native_engine::game::Game) -> (i64, i64) {
    let s = g.state();
    (s[0].1, s[1].1)
}

#[test]
fn intro_lasts_70_ticks_then_black() {
    let p = pack();
    let mut g = g2035::make(&p, &Options::default()).unwrap();
    let mut fr = Frame::new(g2035::assets::W, g2035::assets::H);
    g.render(&mut fr);
    // line 0 "A" centred: x = (352 - 8) / 2 = 172, y = 23; line 2 "AA" at x 168, y 43
    assert_eq!(fr.image.get(172, 23), 1);
    assert_eq!(fr.image.get(171, 23), 0);
    assert_eq!(fr.image.get(168, 43), 1);
    assert_eq!(fr.palette.colours[0], 0xFF55_0044);
    for _ in 0..69 {
        step(&mut *g, false);
    }
    assert_eq!(phase(&*g), (0, 69));
    step(&mut *g, false);
    assert_eq!(phase(&*g).0, 1);
}

fn step(g: &mut dyn native_engine::game::Game, fire: bool) {
    use native_engine::audio::Mixer;
    use native_engine::input::{Action, InputState};
    use native_engine::save::Save;
    let input = InputState {
        held: 0,
        pressed: if fire { Action::Fire.bit() } else { 0 },
        released: 0,
        motion: (0, 0),
        absolute: None,
        held2: 0,
    };
    let mut audio = Mixer::new(44100, 50, 4);
    let mut save = Save::memory();
    let mut cx = native_engine::game::Ctx {
        input: &input,
        audio: &mut audio,
        save: &mut save,
        tick: 0,
    };
    g.tick(&mut cx);
}

#[test]
fn fire_skips_the_intro() {
    let p = pack();
    let mut g = g2035::make(&p, &Options::default()).unwrap();
    step(&mut *g, false);
    step(&mut *g, true);
    assert_eq!(phase(&*g).0, 1);
}
