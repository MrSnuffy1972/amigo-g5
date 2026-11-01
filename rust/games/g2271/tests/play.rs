//! Runs the game headless on the real asset pack when one is given (G2271_PACK, outside the repo), and checks the
//! pieces that need no pack on synthetic data.

use g2271::assets::{ScreenFile, Tables};
use g2271::castle::wrap;
use g2271::screen::{fade, Screen};
use g2271::stage::{room_range, Simon};

/// A 40 by 25 map with a solid floor in attribute row `floor` (synthetic, no game data).
fn floor_map(floor: usize) -> ScreenFile {
    let (w, h) = (40, 25);
    let mut attrs = vec![0i8; w * h];
    for c in 0..w {
        attrs[floor * w + c] = 1;
    }
    ScreenFile {
        w,
        h,
        tiles: vec![0; w * h],
        attrs,
        tile_px: Vec::new(),
    }
}

fn tables() -> Tables {
    Tables {
        anim: vec![0; 700],
        jump: vec![-384, -256, -208, -192, 0, 192, 208, 256, 384, 1000],
        knock: vec![-256, -192, -64, 64, 192, 256, 1000],
        time: vec![3; 32],
        ..Tables::default()
    }
}

fn simon() -> Simon {
    Simon {
        map_w: 40,
        x: 80,
        y: 10 << 8,
        hp: 16,
        facing: 4,
        last_dir: 4,
        centre: true,
        ..Simon::default()
    }
}

#[test]
fn simon_falls_up_to_three_rows_a_frame_and_lands_on_the_floor() {
    let sf = floor_map(20);
    let t = tables();
    let mut s = simon();
    s.update(&t, &[0], &sf);
    assert_eq!(s.y >> 8, 13);
    assert_eq!(s.falling, 1);
    for _ in 0..10 {
        s.update(&t, &[0], &sf);
    }
    assert_eq!(s.y >> 8, 20);
    assert_eq!(s.falling, 0);
}

#[test]
fn simon_walks_eight_pixels_a_frame_and_stops_at_a_wall() {
    let mut sf = floor_map(20);
    for r in 15..20 {
        sf.attrs[r * 40 + 14] = 1;
    }
    let t = tables();
    let mut s = simon();
    s.y = 20 << 8;
    s.right = true;
    s.centre = false;
    s.update(&t, &[0], &sf);
    assert_eq!(s.x, 88);
    assert_eq!(s.mv, 4);
    for _ in 0..10 {
        s.update(&t, &[0], &sf);
    }
    // the wall is in column 14: the player's right column (x / 8 + 2) stops just before it
    assert_eq!(s.x, 96);
    assert_eq!(s.mv, 0);
}

#[test]
fn simon_jumps_and_comes_back_down() {
    let sf = floor_map(20);
    let t = tables();
    let mut s = simon();
    s.y = 20 << 8;
    s.update(&t, &[0], &sf);
    s.jump_btn = true;
    s.update(&t, &[0], &sf);
    s.jump_btn = false;
    assert!(s.jump > 0);
    let mut top = s.y;
    for _ in 0..12 {
        s.update(&t, &[0], &sf);
        top = top.min(s.y);
    }
    assert!(top < (19 << 8), "rose to {top:#x}");
    assert_eq!(s.jump, 0);
    assert_eq!(s.y >> 8, 20);
}

#[test]
fn time_out_starts_the_death() {
    let mut s = simon();
    s.time = 1;
    s.time_on = true;
    for _ in 0..10 {
        s.timers();
    }
    assert_eq!(s.time, 0);
    assert_eq!(s.dying, 0x10);
}

#[test]
fn room_blocks() {
    assert_eq!(room_range(0), Some(0));
    assert_eq!(room_range(16), Some(1));
    assert_eq!(room_range(45), Some(6));
    assert_eq!(room_range(48), None);
}

#[test]
fn wrap_keeps_words_and_width() {
    let l = vec![
        "  one two three four five six seven".to_string(),
        String::new(),
    ];
    let w = wrap(&l, 10);
    assert!(w.iter().all(|s| s.len() <= 10));
    assert_eq!(w.join(" "), "one two three four five six seven");
}

#[test]
fn fade_ends() {
    assert_eq!(fade(0xFA5, 15), 0xFA5);
    assert_eq!(fade(0xFA5, 0), 0);
}

#[test]
fn tile_goes_into_all_pictures() {
    let mut s = Screen::new(400);
    let t = [7u8; 64];
    s.tile(360, 30, &t);
    assert_eq!(s.clean.get(367, 37), 7);
    assert_eq!(s.disp[0].get(360, 30), 7);
    assert_eq!(s.disp[1].get(363, 33), 7);
}

#[test]
fn headless_on_real_pack() {
    let Ok(p) = std::env::var("G2271_PACK") else {
        eprintln!("G2271_PACK not set: skipped");
        return;
    };
    let pack = native_engine::pack::Pack::from_dir(std::path::Path::new(&p)).unwrap();
    let r = native_engine::headless::Run {
        ticks: 400,
        out: None,
        every: 100,
        script: Vec::new(),
    };
    let o = native_engine::game::Options { seed: Some(1) };
    let n = native_engine::headless::run(&g2271::spec(), &pack, &o, &r).unwrap();
    assert_eq!(n, 400);
}

/// The scripted walk (tests/scripts/p2-walk.txt) on the real pack: the game starts, Simon walks right and the map
/// scrolls, jumps, whips and crouches. The numbers are this port's own (a regression check), not the reference's.
#[test]
fn stage_walk_on_real_pack() {
    let Ok(p) = std::env::var("G2271_PACK") else {
        eprintln!("G2271_PACK not set: skipped");
        return;
    };
    let pack = native_engine::pack::Pack::from_dir(std::path::Path::new(&p)).unwrap();
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/scripts/p2-walk.txt"
    ))
    .unwrap();
    let out = std::env::temp_dir().join(format!("g2271-walk-{}", std::process::id()));
    let r = native_engine::headless::Run {
        ticks: 920,
        out: Some(out.clone()),
        every: 1000,
        script: native_engine::script::parse(&text).unwrap(),
    };
    let o = native_engine::game::Options { seed: Some(1) };
    native_engine::headless::run(&g2271::spec(), &pack, &o, &r).unwrap();
    let states = std::fs::read_to_string(out.join("state.jsonl")).unwrap();
    let _ = std::fs::remove_dir_all(&out);
    let at = |t: i64| -> serde_json::Value {
        states
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap())
            .find(|v| v["t"].as_i64() == Some(t))
            .unwrap()
    };
    let v = at(487);
    assert_eq!(v["phase"], 2, "in the stage by tick 487");
    assert_eq!((v["x"].as_i64(), v["map_col"].as_i64()), (Some(8), Some(0)));
    let v = at(607);
    assert_eq!(
        (v["x"].as_i64(), v["map_col"].as_i64()),
        (Some(152), Some(7))
    );
    assert!(at(627)["jump"].as_i64().unwrap() > 0);
    assert_eq!(at(647)["y"], at(607)["y"]);
    assert!(at(695)["whip"].as_i64().unwrap() > 0);
    assert_eq!(at(767)["crouch"], 1);
    assert_eq!(at(799)["crouch"], 0);
}
