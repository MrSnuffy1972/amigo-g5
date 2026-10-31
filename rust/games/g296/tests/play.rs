//! A scripted war on the real asset pack, headless. Needs the pack (never in the repo): G296_PACK=path to the pack
//! directory or binary. Without it the test passes without running (Ryzen runs it; see docs/native/g296-native.md).

use native_engine::game::{Ctx, Game, Options};
use native_engine::input::{Host, Mapper};
use native_engine::pack::Pack;
use native_engine::{audio::Mixer, save::Save};

fn pack() -> Option<Pack> {
    let p = std::env::var("G296_PACK").ok()?;
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
        let s = g296::spec();
        Run {
            g: (s.make)(p, &Options { seed: Some(1234) }).unwrap(),
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
        self.g
            .state()
            .into_iter()
            .find(|s| s.0 == k)
            .map(|s| s.1)
            .unwrap_or(i64::MIN)
    }
    fn click_at(&mut self, x: i32, y: i32) {
        self.step(&[Host::PointerAt(x, y)]);
        self.step(&[Host::MouseButton(true, true)]);
        self.run(2);
        self.step(&[Host::MouseButton(true, false)]);
        self.run(2);
    }
}

#[test]
fn title_options_war_and_peace() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    r.run(200);
    assert_eq!(r.get("musicflag"), 1, "the title music plays");
    assert_eq!(r.get("phase"), 4, "the title waits for a click");
    r.click_at(100, 100);
    r.run(40);
    assert_eq!(r.get("phase"), 6, "the options screen");
    // one more submarine, then PLAY
    r.click_at(230, 92);
    assert_eq!(r.get("numsubs"), 4);
    r.click_at(230, 217);
    assert_eq!(r.get("musicflag"), 0, "PLAY stops the music");
    // the set-up takes the original's time (the map after 6 frames, the objects when placing them was done)
    r.run(40);
    assert_eq!(r.get("phase"), 8, "America's turn");
    assert_eq!(r.get("usa_alive"), 10 + 4 + 5 + 14);
    // elevation preset 60, heading preset 90, LAUNCH
    r.click_at(161, 18);
    r.click_at(151, 40);
    assert_eq!((r.get("curelev"), r.get("curhead")), (60, 90));
    r.click_at(280, 35);
    let t0 = r.tick;
    let mut saw_flight = false;
    while r.get("phase") != 8 && r.tick < t0 + 400 {
        saw_flight |= r.get("phase") == 9;
        r.run(1);
    }
    assert!(saw_flight, "the missile flew");
    assert_eq!(r.get("side"), 1, "then it is Russia's turn");
    assert_eq!(r.get("curammo"), 3);
    // PEACE ends the war at once
    r.click_at(127, 35);
    r.run(200);
    assert_eq!(r.get("phase"), 11, "war over, waiting for a click");
    r.click_at(10, 100);
    r.run(5);
    assert_eq!(r.get("phase"), 3, "back to the title");
}
