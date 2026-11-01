//! A scripted game on the real asset pack, headless. Needs the pack (never in the repo): G608_PACK=path to the pack
//! directory or binary. Without it the test passes without running (Ryzen runs it; see docs/native/g608-native.md).
//! It checks Karl's report on the old build directly: START GAME starts the game, the game reaches play, the levels
//! progress, and nothing freezes or quits after the start-up screens.

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::input::{Host, Mapper};
use native_engine::pack::Pack;
use native_engine::{audio::Mixer, save::Save};

fn pack() -> Option<Pack> {
    let p = std::env::var("G608_PACK").ok()?;
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
    tick: u32,
}

impl Run {
    fn new(p: &Pack) -> Run {
        let s = g608::spec();
        Run {
            g: (s.make)(p, &Options { seed: None }).unwrap(),
            m: Mapper::new(s.info.bindings.clone()),
            mix: Mixer::new(44_100, 50, 4),
            save: Save::memory(),
            tick: 0,
        }
    }
    fn step(&mut self, ev: &[Host]) {
        for h in ev {
            self.m.handle(h);
        }
        let input = self.m.tick();
        self.tick += 1;
        let f = self.g.tick(&mut Ctx {
            input: &input,
            audio: &mut self.mix,
            save: &mut self.save,
            tick: self.tick,
        });
        assert_eq!(f, Flow::Continue, "the game quit at tick {}", self.tick);
    }
    fn run(&mut self, n: u32) {
        for _ in 0..n {
            self.step(&[]);
        }
    }
    fn press(&mut self, b: &str) {
        self.step(&[Host::Pad(b.into(), true)]);
        self.step(&[Host::Pad(b.into(), false)]);
    }
    fn get(&self, k: &str) -> i64 {
        self.g
            .state()
            .into_iter()
            .find(|s| s.0 == k)
            .map(|s| s.1)
            .unwrap_or_else(|| panic!("no state {k}"))
    }
}

#[test]
fn start_game_plays_and_levels_progress() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    r.run(20);
    // the intro's START GAME, then the loader's pictures
    r.press("a");
    r.run(400);
    // fire on the title: the options screen; fire again: level 1
    r.press("a");
    r.run(60);
    r.press("a");
    r.run(100);
    assert_eq!(r.get("level"), 0);
    assert_eq!(r.get("o1k"), 4, "player one's ship is in play");
    assert_eq!(r.get("lives"), 3);
    // a bot: turn now and then and thrust in bursts; the game must keep running (its frame counter moving) and
    // move on from level 1
    let mut last = r.get("frame");
    let mut levels = std::collections::BTreeSet::new();
    for k in 0..30_000u32 {
        let ev = match k % 97 {
            0 => vec![Host::Pad("a".into(), true)],
            30 => vec![Host::Pad("a".into(), false)],
            50 if k % 3 == 0 => vec![Host::Pad("dpad_left".into(), true)],
            56 => vec![Host::Pad("dpad_left".into(), false)],
            _ => vec![],
        };
        r.step(&ev);
        levels.insert(r.get("level"));
        if k % 500 == 499 {
            let f = r.get("frame");
            assert_ne!(f, last, "frozen at tick {}", r.tick);
            last = f;
        }
    }
    assert!(
        levels.iter().any(|&l| l >= 1),
        "the levels progressed: {levels:?}"
    );
}
