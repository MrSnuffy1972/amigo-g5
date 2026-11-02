//! Part 3a checks on synthetic data (no game bytes in the repo): the random number routine, pitch file decoding,
//! the pitch-type choice, the camera and the view mapping.

use g2035::camera::{Aim, CamInput, Camera};
use g2035::pitch::{self, MatchData, Pitch, MAP_H, MAP_W};
use g2035::rng::Rng;
use native_engine::gfx::Frame;

fn pitch_file() -> Vec<u8> {
    let mut b = vec![0u8; MAP_W * MAP_H * 4];
    b[4..8].copy_from_slice(&128u32.to_be_bytes()); // map entry 1 = tile 1
    let mut t0 = vec![0u8; 128];
    t0[6] = 0xFF; // tile 0, row 0, plane 3: all set
    t0[7] = 0xFF;
    let mut t1 = vec![0u8; 128];
    t1[0] = 0x80; // tile 1, row 0, plane 0: first pixel
    t1[8 + 2] = 0x40; // row 1, plane 1: second pixel
    b.extend(t0);
    b.extend(t1);
    b
}

fn data() -> MatchData {
    let p = Pitch::from_file(&pitch_file()).unwrap();
    MatchData {
        palette: [0x111; 16],
        grass: (0..7).map(|t| [t, 0x100 + t, 0x200 + t]).collect(),
        weather: [30u8, 20, 30, 20, 0, 0, 0].repeat(12),
        weather_default: vec![5, 5, 10, 20, 30, 20, 10, 0],
        rng_table: vec![0; 1024],
        aim: [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ],
        pitches: vec![p.clone(), p],
    }
}

#[test]
fn rng_steps_seed_and_xors_table() {
    let mut r = Rng::default();
    let z = vec![0u8; 1024];
    assert_eq!(r.next(0, &z), 0x81);
    assert_eq!(r.next(0, &z), 0x06); // (0x81 << 2) & 0xFF + 0x81 + 0x81
    assert_eq!(r.calls, 2);
    let t: Vec<u8> = (0..1024).map(|i| i as u8).collect();
    let mut r = Rng { seed: 0, calls: 1 };
    assert_eq!(r.next(0x3FF, &t), 0x81); // index (0x3FF + 1) & 0x3FF = 0
    let mut r = Rng { seed: 0, calls: 0 };
    assert_eq!(r.next(5, &t), 0x81 ^ 5);
}

#[test]
fn pitch_file_decodes_interleaved_planes() {
    let p = Pitch::from_file(&pitch_file()).unwrap();
    assert_eq!(p.at(0, 0), 8);
    assert_eq!(p.at(0, 1), 0);
    assert_eq!(p.at(16, 0), 1);
    assert_eq!(p.at(17, 1), 2);
    assert_eq!(p.at(-1, 0), 0);
    assert_eq!(p.at(0, 16 * MAP_H as i32), 0);
    let mut bad = pitch_file();
    bad[3] = 1;
    assert!(Pitch::from_file(&bad).is_err());
}

#[test]
fn pitch_type_walks_the_month_row() {
    let d = data();
    // first number 0x81 -> 129 * 100 / 256 = 50: past 30 and 20, stops in the third (type 2)
    assert_eq!(d.choose_pitch_type(3, &mut Rng::default(), 0), 2);
    assert_eq!(d.choose_pitch_type(3, &mut Rng::default(), 0x10), 0x102);
    // default row 5, 5, 10, 20, 30: 50 - 5 - 5 - 10 - 20 = 10 < 30 -> type 4
    assert_eq!(d.choose_pitch_type(-1, &mut Rng::default(), 0), 4);
    // direct types do not draw a number
    let mut r = Rng::default();
    assert_eq!(d.choose_pitch_type(-3, &mut r, 0), 5);
    assert_eq!(r.calls, 0);
    assert_eq!(d.palette_for(0x104)[7], 0x104);
}

#[test]
fn camera_eases_leans_and_clamps() {
    let aim = data().aim;
    let mut c = Camera::place(100, 100);
    let i = CamInput {
        ball: (500, 236),
        ball_v: (1, 0),
        fixed: false,
        focus: None,
        aim: Aim::Ball,
        event: 0,
    };
    c.step(&i, &aim);
    assert_eq!((c.px(), c.py(), c.look_x, c.look_y), (105, 100, 2, 0)); // at most 5 pixels a frame
    for _ in 0..200 {
        c.step(&i, &aim);
    }
    assert_eq!(c.look_x, 40);
    assert_eq!(c.px(), 0x110 - 0x10 - 3); // target 256; the last 3 pixels are under a quarter-pixel step
    let mut c = Camera::place(0, 0);
    c.step(
        &CamInput {
            focus: Some((0, 0)),
            ..i
        },
        &aim,
    );
    assert_eq!((c.px(), c.py()), (0, 9)); // y never above 9
    let mut c = Camera::place(200, 200);
    c.step(
        &CamInput {
            aim: Aim::Facing(6),
            ..i
        },
        &aim,
    );
    assert_eq!(c.look_x, -2);
    c.step(&CamInput { event: 0x16, ..i }, &aim);
    assert_eq!((c.look_x, c.look_y), (0, 0));
}

#[test]
fn view_shows_pitch_in_columns_1_to_336() {
    let d = data();
    let mut f = Frame::new(352, 272);
    pitch::render(&d, 0, 1, 0, &mut f);
    let px = |x: usize, y: usize| f.image.px[y * 352 + x];
    assert_eq!(px(0, 0), 0); // border
    assert_eq!(px(1, 0), 8); // pitch (1 - 1 + 1, 0) = tile 0 plane 3
    assert_eq!(px(16, 0), 1); // pitch (16, 0) = tile 1
    assert_eq!(px(337, 0), 0);
    assert_eq!(f.palette.colours[7], native_engine::gfx::rgb12(0x100));
}
