//! Runs the game headless on the real asset pack when one is given (G2271_PACK, outside the repo), and checks the
//! pieces that need no pack on synthetic data.

use g2271::castle::wrap;
use g2271::screen::{fade, Screen};

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
