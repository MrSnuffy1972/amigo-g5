//! Scripted play on the real asset pack, headless. Needs the pack (never in the repo): G2179_PACK = path to the
//! pack directory or binary. Without it the tests pass without running (Ryzen runs them; see
//! docs/native/g2179-native.md). The scripts are joystick input only.
//!
//! What they check: fire gets past the logo and the title picture, left starts a one-player game, the get-ready
//! screen takes fire, Giana walks, jumps and scrolls the stage, the stage clock runs, a stage is completed (exit,
//! tally, get-ready for stage 2, stage 2 playing); and in a two-player game player one's lives run out, the game-over
//! high-score screen takes a name, the all-time table is saved, and the turn passes to player two. Nothing quits.

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::input::{Host, Mapper};
use native_engine::pack::Pack;
use native_engine::{audio::Mixer, save::Save};

fn pack() -> Option<Pack> {
    let p = std::env::var("G2179_PACK").ok()?;
    let path = std::path::Path::new(&p);
    Some(if path.is_dir() {
        Pack::from_dir(path).unwrap()
    } else {
        Pack::from_bytes(&std::fs::read(path).unwrap()).unwrap()
    })
}

struct Run {
    g: Box<dyn Game>,
    m: Mapper,
    mix: Mixer,
    save: Save,
    script: Vec<(u32, Host)>,
    tick: u32,
}

impl Run {
    fn new(p: &Pack, script: &str) -> Run {
        let s = g2179::spec();
        Run {
            g: (s.make)(p, &Options { seed: Some(1) }).unwrap(),
            m: Mapper::new(s.info.bindings.clone()),
            mix: Mixer::new(44_100, 50, 0),
            save: Save::memory(),
            script: native_engine::script::parse(script).unwrap(),
            tick: 0,
        }
    }
    fn state(&self, k: &str) -> i64 {
        self.g
            .state()
            .iter()
            .find(|(n, _)| n == k)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("no state {k}"))
    }
    /// Runs to `until`, calling `each` after every tick.
    fn run(&mut self, until: u32, mut each: impl FnMut(&Run)) {
        while self.tick < until {
            self.tick += 1;
            for (_, h) in self.script.iter().filter(|(t, _)| *t == self.tick) {
                self.m.handle(h);
            }
            let input = self.m.tick();
            let f = self.g.tick(&mut Ctx {
                input: &input,
                audio: &mut self.mix,
                save: &mut self.save,
                tick: self.tick,
            });
            assert_eq!(f, Flow::Continue, "the game quit at tick {}", self.tick);
            each(self);
        }
    }
}

#[test]
fn a_game_starts_scrolls_and_completes_stage_one() {
    let Some(p) = pack() else {
        eprintln!("G2179_PACK not set: skipped");
        return;
    };
    let mut r = Run::new(&p, include_str!("scripts/stage_clear.txt"));
    let (mut max_scroll, mut min_time, mut cleared_at) = (0, 0x99, 0);
    r.run(7850, |r| {
        if r.state("stage") == 0 && r.state("disp") == 3 {
            max_scroll = max_scroll.max(r.state("scroll"));
            min_time = min_time.min(r.state("time"));
        }
        if cleared_at == 0 && r.state("stage") == 1 {
            cleared_at = r.tick;
        }
    });
    assert!(max_scroll > 1500, "the stage scrolled to {max_scroll}");
    assert!(min_time < 0x90, "the stage clock ran");
    assert!(cleared_at > 0, "stage one was completed");
    // The tally paid ten points a second left, and stage two is being played.
    assert!(r.state("score") >= 0x1000, "score {:x}", r.state("score"));
    assert_eq!((r.state("stage"), r.state("disp")), (1, 3));
}

#[test]
fn game_over_saves_the_high_scores_and_passes_the_turn() {
    let Some(p) = pack() else {
        eprintln!("G2179_PACK not set: skipped");
        return;
    };
    let mut r = Run::new(&p, include_str!("scripts/two_players.txt"));
    let mut saw_hs = false;
    let mut deaths = 0;
    let mut dying = false;
    r.run(7880, |r| {
        saw_hs |= r.state("disp") == 5;
        let d = r.state("dying") != 0;
        if d && !dying {
            deaths += 1;
        }
        dying = d;
    });
    assert!(deaths >= 3, "{deaths} deaths");
    assert!(saw_hs, "the high-score screen showed");
    let hs = r
        .save
        .get("highscores")
        .expect("the all-time table was saved");
    assert_eq!(hs.as_array().map(|a| a.len()), Some(5));
    assert_eq!(r.state("player"), -1, "player two's turn");
}
