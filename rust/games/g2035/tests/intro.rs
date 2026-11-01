//! Part 1 checks on a small synthetic pack (no game bytes in the repo): the intro's timing, skip and drawing.

mod common;

use common::{hold, pack, state};
use native_engine::game::Options;
use native_engine::gfx::Frame;
use native_engine::input::Action;

#[test]
fn intro_lasts_70_ticks_then_the_language_screen() {
    let p = pack();
    let mut g = g2035::make(&p, &Options::default()).unwrap();
    let mut fr = Frame::new(g2035::assets::W, g2035::assets::H);
    g.render(&mut fr);
    // line 0 "A" centred: x = (352 - 8) / 2 = 172, y = 23; line 2 "AA" at x 168, y 43
    assert_eq!(fr.image.get(172, 23), 1);
    assert_eq!(fr.image.get(171, 23), 0);
    assert_eq!(fr.image.get(168, 43), 1);
    assert_eq!(fr.palette.colours[0], 0xFF55_0044);
    hold(&mut *g, &[], 69);
    assert_eq!((state(&*g)["phase"], state(&*g)["phase_ticks"]), (0, 69));
    hold(&mut *g, &[], 1);
    assert_eq!((state(&*g)["phase"], state(&*g)["menu"]), (1, 0));
}

#[test]
fn fire_skips_the_intro() {
    let p = pack();
    let mut g = g2035::make(&p, &Options::default()).unwrap();
    hold(&mut *g, &[], 1);
    hold(&mut *g, &[Action::Fire], 1);
    assert_eq!(state(&*g)["phase"], 1);
}
