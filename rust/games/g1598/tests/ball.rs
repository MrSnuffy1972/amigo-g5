//! The ball's flight, landing, roll and cup on synthetic holes (no game data).

use g1598::ball::{inside_poly, round16, Ball, OFF};
use g1598::hole::Poly;

fn square(kind: i16, x0: i16, y0: i16, x1: i16, y1: i16) -> Poly {
    Poly {
        kind,
        min: (x0, y0),
        max: (x1, y1),
        pts: vec![x0, y0, x1, y0, x1, y1, x0, y1, x0, y0],
    }
}

#[test]
fn rounding_to_16_bits_goes_half_up() {
    assert_eq!(round16(0x0001_8000), 2);
    assert_eq!(round16(0x0001_7fff), 1);
    assert_eq!(round16(-0x0000_8000), 0);
}

#[test]
fn the_last_containing_polygon_names_the_surface() {
    let polys = vec![
        square(0, 0, 0, 1000, 1000),
        square(2, 400, 400, 600, 600),
        square(6, 900, 900, 950, 950),
    ];
    assert_eq!(inside_poly(&polys, 100, 100), 0);
    assert_eq!(inside_poly(&polys, 500, 500), 2);
    assert_eq!(inside_poly(&polys, 920, 920), 3, "water reports as 3");
    assert_eq!(inside_poly(&polys, 2000, 2000), OFF);
}

fn no_slope(_: i16, _: i16) -> (i32, i32) {
    (0, 0)
}

#[test]
fn a_lofted_ball_flies_lands_on_the_fairway_and_rolls_to_a_stop() {
    let polys = vec![square(0, -30000, -30000, 30000, 30000)];
    let mut b = Ball {
        vx: 40 << 16,
        vz: -(6 << 16),
        moving: 1,
        hit_tree: 2,
        ..Ball::default()
    };
    let mut steps = 0;
    let mut landed_at = None;
    for t in 0..20000 {
        b.interrupt(&polys, (29000, 29000), &mut no_slope);
        if b.rolling && landed_at.is_none() {
            landed_at = Some(t);
        }
        if b.moving == 0 {
            steps = t;
            break;
        }
    }
    assert!(landed_at.is_some(), "it came down");
    assert!(steps > 0, "it stopped");
    assert!(b.pos.0 > 1000 && b.pos.1 == 0, "{:?}", b.pos);
    assert!(b.sounds.contains(&2), "a fairway bounce sound");
}

#[test]
fn a_slow_ball_at_the_hole_drops_in_and_a_fast_one_hops() {
    let polys = vec![square(1, -1000, -1000, 1000, 1000)];
    let mut b = Ball {
        x: 0,
        vx: 1 << 14,
        moving: 1,
        rolling: true,
        hit_tree: 2,
        ..Ball::default()
    };
    for _ in 0..12 {
        b.interrupt(&polys, (0, 0), &mut no_slope);
    }
    assert_eq!(b.moving, 0);
    assert_eq!((b.pos.0, b.pos.1), (0, 0));
    let mut f = Ball {
        x: -(1 << 16),
        vx: 4 << 16,
        moving: 1,
        rolling: true,
        hit_tree: 2,
        ball_cnt: 20,
        ..Ball::default()
    };
    for _ in 0..3 {
        f.interrupt(&polys, (2, 0), &mut no_slope);
    }
    assert!(f.hop || f.sounds.contains(&0x17), "lipped out: {f:?}");
}
