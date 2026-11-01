//! The front end on the real asset pack, headless. Needs the pack (never in the repo): G2055_PACK=path to the pack
//! directory or binary. Without it the tests pass without running (Ryzen runs them; see docs/native/g2055-native.md).

use native_engine::game::{Ctx, Game, Options};
use native_engine::gfx::Frame;
use native_engine::input::{Host, Mapper};
use native_engine::pack::Pack;
use native_engine::{audio::Mixer, save::Save};

fn pack() -> Option<Pack> {
    let p = std::env::var("G2055_PACK").ok()?;
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
        let s = g2055::spec();
        Run {
            g: (s.make)(p, &Options { seed: Some(1) }).unwrap(),
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
        self.g.tick(&mut Ctx {
            input: &input,
            audio: &mut self.mix,
            save: &mut self.save,
            tick: self.tick,
        });
    }
    fn run(&mut self, n: u32) {
        for _ in 0..n {
            self.step(&[]);
        }
    }
    fn get(&self, k: &str) -> i64 {
        self.g.state().into_iter().find(|s| s.0 == k).unwrap().1
    }
    fn lit(&self) -> usize {
        let mut f = Frame::new(320, 200);
        self.g.render(&mut f);
        f.image.px.iter().filter(|&&p| p != 0).count()
    }
}

#[test]
fn the_intro_types_the_cracktro_then_the_title_and_the_story() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    r.run(150);
    assert_eq!(r.get("stage"), 1, "opening pages");
    assert_eq!(r.get("page"), 0, "the CRYSTAL CRACKED page first");
    assert!(r.lit() > 0, "letters typed");
    assert!(r.get("sfx_count") > 0, "typing clicks asked for");
    r.run(1150);
    assert!(r.get("stage") >= 2, "past the opening pages");
    r.run(400);
    assert_eq!(r.get("stage"), 3, "story");
}

#[test]
fn fire_skips_to_loading_game_and_the_intro_ends() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    // a tap skips only while it is held, as on the original: tap during the title's wait
    while r.get("stage") != 2 {
        r.step(&[]);
    }
    r.run(150);
    let fire = |d| [Host::Key("Space".into(), d)];
    for _ in 0..5 {
        r.step(&fire(true));
    }
    r.step(&fire(false));
    r.run(600);
    assert_eq!(r.get("stage"), 6, "the intro is over");
    assert_eq!(r.get("intro"), 0);
    assert!(r.lit() > 1000, "the arena with LOADING GAME is shown");
}

#[test]
fn nothing_skips_the_opening_pages() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    for _ in 0..300 {
        r.step(&[Host::Key("Space".into(), true)]);
    }
    assert_eq!(r.get("stage"), 1);
}
