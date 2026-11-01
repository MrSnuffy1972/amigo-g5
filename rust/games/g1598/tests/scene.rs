//! The hole view's pieces on synthetic data (no game data), and a headless run on the real asset pack when one is
//! given (G1598_PACK, outside the repo).

use g1598::hole::camera;
use g1598::intro;
use g1598::scene::{clip_3d, draw_bkg, perspect, xy_to_angle960, Fill, View};
use native_engine::gfx::Bitmap;

#[test]
fn a_view_aimed_north_sees_a_point_ahead_in_the_middle_of_the_picture() {
    let v = camera((1000, 1000), (1000, 5000));
    // pulled back 380 units behind the ball, away from the target
    assert_eq!((v.x, v.y), (1000, 620));
    let p = v.transform(1000, 3000, 0);
    assert_eq!(p[0], 0);
    assert!(p[2] > 0);
    let (x, y) = perspect(p);
    assert_eq!(x, 150);
    assert!(y > 44 && y < 200);
}

#[test]
fn perspect_at_depth_zero_pins_to_the_far_edges() {
    assert_eq!(perspect([5, 7, 0]), (319, 199));
    assert_eq!(perspect([-5, -7, 0]), (-5, -7));
}

#[test]
fn a_ground_square_ahead_fills_a_band_between_horizon_and_bottom() {
    let v = View {
        x: 0,
        y: 0,
        z: -88,
        s: 0,
        c: 0x7fff,
    };
    // facing +x: a square from 200 to 2000 ahead, 300 either side; the first vertex is repeated at the end
    let sq = [200, -300, 2000, -300, 2000, 300, 200, 300, 200, -300];
    let mut f = Fill::default();
    let mut scr = Bitmap::new(320, 200);
    draw_bkg(&mut scr, &f, true);
    assert_eq!(scr.get(10, 10), 9);
    assert_eq!(scr.get(10, 150), 3);
    clip_3d(&v, &sq, &mut f);
    f.draw(&mut scr, 5, 5);
    let filled = scr.px.iter().filter(|&&c| c == 5).count();
    assert!(filled > 1000, "{filled}");
    assert_eq!(scr.get(150, 10), 9, "nothing drawn in the sky");
}

#[test]
fn a_two_colour_fill_is_a_checkerboard() {
    let mut f = Fill::default();
    let mut scr = Bitmap::new(320, 200);
    f.edge((10, 10), (10, 20));
    f.edge((30, 20), (30, 10));
    f.draw(&mut scr, 1, 2);
    assert_eq!(scr.get(12, 12), 1);
    assert_eq!(scr.get(13, 12), 2);
    assert_eq!(scr.get(13, 13), 1);
}

#[test]
fn bearings_cover_the_turn_in_960ths() {
    // a tangent table of 16 steps to 45 degrees, made here
    let t: Vec<i16> = (0..16)
        .map(|i| ((i as f64 * 45.0 / 15.0).to_radians().tan() * 32767.0) as i16)
        .collect();
    assert_eq!(xy_to_angle960(&t, 100, 0), 0);
    assert_eq!(xy_to_angle960(&t, 0, 100), 240);
    assert_eq!(xy_to_angle960(&t, -100, 0), 480);
    assert_eq!(xy_to_angle960(&t, 0, -100), 720);
    assert_eq!(xy_to_angle960(&t, 100, 100), 120);
}

#[test]
fn the_console_page_parses_offsets_and_inverse_video() {
    let ops = intro::parse("\u{9b}0x\u{9b}12y\r\u{9b}10C\u{9b}7m A\u{9b}0m");
    assert_eq!(
        ops,
        vec![
            intro::Op::Left(0),
            intro::Op::Top(12),
            intro::Op::Cr,
            intro::Op::Forward(10),
            intro::Op::Inverse(true),
            intro::Op::Put(' '),
            intro::Op::Put('A'),
            intro::Op::Inverse(false),
        ]
    );
}

#[test]
fn headless_run_on_the_real_pack() {
    let Ok(p) = std::env::var("G1598_PACK") else {
        return;
    };
    let pack = native_engine::pack::Pack::from_dir(std::path::Path::new(&p)).unwrap();
    let script = native_engine::script::parse(include_str!("scripts/p2-holes.txt")).unwrap();
    let out = std::env::temp_dir().join("g1598-test-run");
    let r = native_engine::headless::Run {
        ticks: 170,
        out: Some(out.clone()),
        every: 170,
        script,
    };
    let n = native_engine::headless::run(
        &g1598::spec(),
        &pack,
        &native_engine::game::Options { seed: Some(1) },
        &r,
    )
    .unwrap();
    assert_eq!(n, 170);
    let st = std::fs::read_to_string(out.join("state.jsonl")).unwrap();
    let last = st.lines().last().unwrap();
    assert!(
        last.contains("\"screen\":3") && last.contains("\"hole\":2"),
        "{last}"
    );
}
