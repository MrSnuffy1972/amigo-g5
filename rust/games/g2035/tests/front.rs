//! Part 2 checks on the synthetic pack: the menu loop's timing (set-up frames, a move every five frames, fire only
//! after a release), the fade between screens, the grid walk, the attract time-outs, the language text blocks and
//! the item drawing.

mod common;

use common::{hold, pack, state, step};
use g2035::menu::{fade, navigate, SETUP};
use native_engine::game::{Game, Options};
use native_engine::gfx::Frame;
use native_engine::input::Action;

/// At the language screen with its loop about to make its first pass.
fn at_language() -> Box<dyn Game> {
    let p = pack();
    let mut g = g2035::make(&p, &Options::default()).unwrap();
    hold(&mut *g, &[], 70);
    assert_eq!((state(&*g)["phase"], state(&*g)["menu"]), (1, 0));
    hold(&mut *g, &[], SETUP - 1);
    g
}

/// Ticks (holding nothing) until menu `m` is open and its loop is live; panics after `max`.
fn until_menu(g: &mut dyn Game, m: i64, max: u32) -> u32 {
    for n in 1..=max {
        hold(g, &[], 1);
        let s = state(g);
        if s["phase"] == 1 && s["menu"] == m && s["phase_ticks"] == SETUP as i64 - 1 {
            return n;
        }
    }
    panic!("menu {m} not reached in {max} ticks: {:?}", state(g));
}

fn render(g: &dyn Game) -> Frame {
    let mut fr = Frame::new(g2035::assets::W, g2035::assets::H);
    g.render(&mut fr);
    fr
}

#[test]
fn the_cursor_moves_after_four_frames() {
    let mut g = at_language();
    // the loop waits 4 frames on entry
    hold(&mut *g, &[Action::Down], 4);
    assert_eq!(state(&*g)["cursor"], 0);
    hold(&mut *g, &[Action::Down], 1);
    assert_eq!(state(&*g)["cursor"], 2);
    // the frame it moves on shows no cursor (erased; drawn at the new item next frame)
    assert_eq!(render(&*g).image.get(12, 60), 6);
    hold(&mut *g, &[], 5);
    assert_eq!(render(&*g).image.get(12, 160), 3);
    hold(&mut *g, &[Action::Right], 1);
    assert_eq!(state(&*g)["cursor"], 3);
    hold(&mut *g, &[], 5);
    hold(&mut *g, &[Action::Up], 1);
    assert_eq!(state(&*g)["cursor"], 1);
}

#[test]
fn a_held_stick_repeats_every_six_frames() {
    let mut g = at_language();
    hold(&mut *g, &[], 5);
    hold(&mut *g, &[Action::Fire], 1);
    until_menu(&mut *g, 1, 100);
    hold(&mut *g, &[], 5);
    // main menu column 1: items 0, 1, 3 from the top; a move, a skipped pass, four waiting passes, a move
    let mut moves = Vec::new();
    for t in 0..14 {
        let before = state(&*g)["cursor"];
        let prev: &[Action] = if t == 0 { &[] } else { &[Action::Down] };
        step(&mut *g, prev, &[Action::Down]);
        if state(&*g)["cursor"] != before {
            moves.push((t, state(&*g)["cursor"]));
        }
    }
    assert_eq!(moves, vec![(0, 1), (6, 3)]);
}

#[test]
fn fire_must_be_released_before_it_activates() {
    let mut g = at_language();
    // (held for 255 frames the wait runs out and fire activates, as in the original)
    hold(&mut *g, &[Action::Fire], 200);
    assert_eq!(state(&*g)["phase"], 1);
    step(&mut *g, &[Action::Fire], &[]);
    hold(&mut *g, &[Action::Fire], 1);
    assert_eq!(state(&*g)["phase"], 3);
    // the language screen fades out, then the main menu opens black and fades in
    assert_eq!(render(&*g).palette.colours[6], 0xFF66_0000);
    hold(&mut *g, &[], 2);
    assert_eq!(render(&*g).palette.colours[6], 0xFF33_0000);
    let n = until_menu(&mut *g, 1, 100);
    assert_eq!(n, 7 + SETUP - 1);
    let s = state(&*g);
    assert_eq!((s["menu"], s["language"], s["cursor"]), (1, 0, 0));
}

#[test]
fn fades_truncate_each_amiga_channel() {
    assert_eq!(fade(0xFFFF_7711, 8), 0xFF77_3300);
    assert_eq!(fade(0xFFFF_7711, 4), 0xFF33_1100);
    assert_eq!(fade(0xFFAA_4400, 2), 0xFF11_0000);
    assert_eq!(fade(0xFFAA_4400, 16), 0xFFAA_4400);
}

#[test]
fn choosing_deutsch_uses_the_german_text_block() {
    let mut g = at_language();
    hold(&mut *g, &[], 5);
    hold(&mut *g, &[Action::Right], 1);
    hold(&mut *g, &[], 1);
    hold(&mut *g, &[Action::Fire], 1);
    until_menu(&mut *g, 1, 100);
    assert_eq!(state(&*g)["language"], 1);
    let fr = render(&*g);
    // button 0 says "AA" in German: glyphs advance 3, total 6, so x = 29 + 65 - 3 = 91 (+1 on screen);
    // y = 56 + (36 - 5 + 1) / 2 = 72; colour 6 replaces the face colour, the shadow stays colour 8
    assert_eq!(fr.image.get(92, 72), 6);
    assert_eq!(fr.image.get(95, 72), 6);
    assert_eq!(fr.image.get(98, 72), 8);
    assert_eq!(fr.image.get(91, 72), 5);
}

#[test]
fn boxes_have_a_ring_a_top_edge_and_a_shaded_bottom() {
    let mut g = at_language();
    hold(&mut *g, &[], 1);
    let fr = render(&*g);
    // (the menu display starts one pixel right: screen x = item x + 1)
    let px = |x, y| fr.image.get(x, y);
    // box 1 (247, 39, 80 x 79), attribute $8F06: fill 6, frame 15, bottom shade[6] = 5
    assert_eq!(px(251, 60), 6);
    assert_eq!(px(250, 60), 15);
    assert_eq!(px(249, 60), 6);
    assert_eq!(px(261, 39), 15);
    assert_eq!(px(261, 117), 5);
    // the flag sprite on its own fill
    assert_eq!(px(34, 52), 3);
    // the cursor on box 0, at counter 7 in the flash colour of pass 3 (colour 1)
    assert_eq!(px(12, 60), 1);
    assert_eq!(px(31, 39), 1);
}

#[test]
fn attract_mode_times_out_to_the_main_menu_and_back() {
    let mut g = at_language();
    // the language screen's counter is at SETUP - 1: 1500 frames from opening
    hold(&mut *g, &[], 1500 - SETUP);
    assert_eq!(state(&*g)["menu"], 0);
    hold(&mut *g, &[], 1);
    // no fade on the attract changes
    assert_eq!((state(&*g)["phase"], state(&*g)["menu"]), (1, 1));
    hold(&mut *g, &[], 374);
    assert_eq!(state(&*g)["menu"], 1);
    hold(&mut *g, &[], 1);
    assert_eq!(state(&*g)["menu"], 0);
    // the second language screen times out after 375 frames
    hold(&mut *g, &[], 374);
    assert_eq!(state(&*g)["menu"], 0);
    hold(&mut *g, &[], 1);
    assert_eq!(state(&*g)["menu"], 1);
    // fire on the attract main menu (read once its loop runs): the language screen again, now without a time-out
    hold(&mut *g, &[Action::Fire], SETUP);
    assert_eq!(state(&*g)["menu"], 0);
    hold(&mut *g, &[], 3000);
    assert_eq!((state(&*g)["phase"], state(&*g)["menu"]), (1, 0));
}

#[test]
fn main_menu_choice_is_pending_and_the_cursor_is_kept() {
    let mut g = at_language();
    hold(&mut *g, &[], 5);
    hold(&mut *g, &[Action::Fire], 1);
    until_menu(&mut *g, 1, 100);
    hold(&mut *g, &[], 5);
    hold(&mut *g, &[Action::Right], 1);
    assert_eq!(state(&*g)["cursor"], 2);
    hold(&mut *g, &[], 1);
    hold(&mut *g, &[Action::Fire], 1);
    hold(&mut *g, &[], 10);
    assert_eq!((state(&*g)["phase"], state(&*g)["cursor"]), (2, 2));
    hold(&mut *g, &[Action::Cancel], 1);
    assert_eq!((state(&*g)["phase"], state(&*g)["cursor"]), (1, 2));
}

#[test]
fn grid_walk_codes() {
    let p = pack();
    let fe = g2035::assets::FrontEnd::load(&p).unwrap();
    let m = &fe.menus[1];
    // right from (1, 1) is item 2; down from (2, 1) meets -3, walks to row 3 and finds -1
    assert_eq!(navigate(m, 1, 1, 1, 0), Some((2, 2, 1)));
    assert_eq!(navigate(m, 2, 1, 0, 1), None);
    // left of the grid clamps to column 0, which is -1
    assert_eq!(navigate(m, 0, 1, -1, 0), None);
}
