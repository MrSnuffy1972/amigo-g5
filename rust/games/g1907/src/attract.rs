//! The title (phase 8, clear_memory_and_loop at $1EAC). The original runs a 33-step script of colour fades,
//! a starfield, the System 3 and IK+ logos, the credits and the key help while the fighters dance behind the
//! title; then the demo bout. This version shows the same pictures from the title sheet as a sequence of cards with
//! the music, then starts the demo bout as the original does. Fire, F1 (Start) or F2 starts a game at any time.

use crate::ik::{Bob, Ik, Src};

#[derive(Default, Clone)]
pub struct Attract {
    pub card: usize,
    pub t: u32,
}

/// The cards: (sheet x, y, w, h) shown at (screen x, y).
const CARDS: [(i32, i32, i32, i32, i32, i32); 3] = [
    (0, 0, 160, 121, 80, 40),
    (160, 0, 160, 116, 80, 42),
    (160, 120, 160, 80, 80, 60),
];

/// The frame interrupt's title work (set_palette_and_dispatch, $2498): nothing to step in this version.
pub fn vbl_step(_g: &mut Ik) {}

fn show(g: &mut Ik, n: usize) {
    let (sx, sy, w, h, dx, dy) = CARDS[n];
    for s in 0..2 {
        g.scr[s].fill(0);
        for y in 0..h {
            for x in (0..w).step_by(16) {
                let src = Bob {
                    src: Src::Sheet2,
                    x: sx + x,
                    y: sy + y,
                };
                g.bob(s, src, dx + x, dy + y, 1, false);
            }
        }
    }
}

/// clear_memory_and_loop ($1EAC): the title cards, then the demo bout.
pub fn clear_memory_and_loop(g: &mut Ik) {
    g.human = [0; 3];
    g.points = [0; 3];
    g.title_colours = false;
    for c in 0..CARDS.len() {
        g.attract.card = c;
        show(g, c);
        for _ in 0..250 {
            g.frame();
        }
    }
    // l0_20D0: the demo bout
    g.copy_word_block_to_memory();
    g.demo = 0;
    g.sil_on = false;
    g.reset_flag_and_call_helpers();
}
