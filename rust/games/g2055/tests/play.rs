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
    assert!(r.get("stage") >= 6, "the intro is over");
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

fn tap(r: &mut Run, key: &str) {
    for d in [true, false] {
        r.step(&[Host::Key(key.into(), d)]);
    }
    r.run(60);
}

#[test]
fn the_menus_lead_to_the_league_and_esc_goes_back() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    let mut n = 0;
    while r.get("stage") != 7 && n < 4000 {
        r.step(&[Host::Key("Space".into(), n % 20 < 10)]);
        n += 1;
    }
    r.run(120);
    assert_eq!(r.get("menu"), 1, "the first menu");
    assert!(r.lit() > 30000, "the panel is shown");
    tap(&mut r, "Space");
    assert_eq!(r.get("menu"), 2, "1 PLAYER GAME leads to the match menu");
    tap(&mut r, "Down");
    assert_eq!(r.get("choice"), 1, "LEAGUE");
    tap(&mut r, "Up");
    tap(&mut r, "Up");
    assert_eq!(r.get("choice"), 3, "the arrows wrap to PRACTICE");
    tap(&mut r, "Down");
    tap(&mut r, "Down");
    tap(&mut r, "Space");
    assert_eq!(r.get("menu"), 3, "the league's game menu");
    assert_eq!((r.get("mode"), r.get("money")), (1, 550));
    tap(&mut r, "Escape");
    assert_eq!(r.get("menu"), 1, "Esc goes back to the first menu");
    assert_eq!(r.get("picks"), 0);
    tap(&mut r, "Space");
    tap(&mut r, "Down");
    tap(&mut r, "Down");
    tap(&mut r, "Space");
    assert_eq!(r.get("menu"), 6, "the cup's game menu");
    tap(&mut r, "Space");
    assert_eq!(r.get("next"), 5, "CUP, NEW GAME");
    assert_eq!(r.get("menu"), 1, "back to the first menu");
}

#[test]
fn the_first_menu_starts_the_demo_after_1500_idle_frames() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    let mut n = 0;
    while r.get("stage") != 7 && n < 4000 {
        r.step(&[Host::Key("Space".into(), n % 20 < 10)]);
        n += 1;
    }
    r.run(1400);
    assert_eq!(r.get("picks"), 0);
    r.run(200);
    assert_eq!(r.get("next"), 9, "DEMO GAME");
}

#[test]
fn knockout_shows_the_next_fixture_until_fire() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    let mut n = 0;
    while r.get("stage") != 7 && n < 4000 {
        r.step(&[Host::Key("Space".into(), n % 20 < 10)]);
        n += 1;
    }
    r.run(120);
    tap(&mut r, "Space");
    assert_eq!(r.get("menu"), 2, "the match menu");
    tap(&mut r, "Space");
    r.run(60);
    assert_eq!(r.get("stage"), 8, "KNOCKOUT shows NEXT FIXTURE");
    assert_eq!(
        (r.get("mode"), r.get("money"), r.get("opponent")),
        (0, 1100, 1)
    );
    assert!(r.lit() > 30000, "the screen is shown");
    r.run(200);
    assert_eq!(r.get("stage"), 8, "it waits for fire");
    tap(&mut r, "Space");
    assert_eq!(r.get("stage"), 7, "back to the menus");
    assert_eq!(
        r.get("next"),
        1,
        "the knockout's gym and match come next (a later part)"
    );
}
