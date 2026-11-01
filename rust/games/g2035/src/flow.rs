//! The game's top-level flow: the phases of the recovered start-up and main programs in order. Part 1 has the intro;
//! the loading wait that followed it in the original (the screen stays black while the start-up program reads the
//! main program off disk 1 and asks for disk 2) is not ported, as nothing is read from disks.

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::Frame;
use native_engine::input::Action;

use crate::assets::Assets;
use crate::intro::Intro;

pub enum Phase {
    Intro(Intro),
    /// The phases of part 2 onwards (language choice, menus, the match) start here; until then a black screen,
    /// as the original showed after the intro.
    Front {
        ticks: u32,
    },
}

pub struct Native {
    pub a: Assets,
    pub phase: Phase,
    pub seed: u32,
}

impl Native {
    pub fn new(a: Assets, o: &Options) -> Native {
        Native {
            a,
            phase: Phase::Intro(Intro::new()),
            seed: o.seed.unwrap_or(0),
        }
    }
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        let fire = cx.input.pressed(Action::Fire) || cx.input.pressed(Action::Start);
        match &mut self.phase {
            Phase::Intro(i) => {
                if i.tick(&self.a.intro, fire) {
                    self.phase = Phase::Front { ticks: 0 };
                }
            }
            Phase::Front { ticks } => *ticks += 1,
        }
        Flow::Continue
    }

    fn render(&self, frame: &mut Frame) {
        match &self.phase {
            Phase::Intro(i) => i.render(&self.a.intro, frame),
            Phase::Front { .. } => {
                frame.palette.colours[0] = 0xFF00_0000;
                frame.image.fill(0);
            }
        }
    }

    fn state(&self) -> Vec<(String, i64)> {
        let (p, t) = match &self.phase {
            Phase::Intro(i) => (0, i.ticks),
            Phase::Front { ticks } => (1, *ticks),
        };
        vec![("phase".into(), p), ("phase_ticks".into(), t as i64)]
    }
}
