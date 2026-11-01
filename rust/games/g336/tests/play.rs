//! A scripted game on the real asset pack, headless. Needs the pack (never in the repo): G336_PACK=path to the pack
//! directory or binary. Without it the test passes without running (Ryzen runs it; see docs/native/g336-native.md).
//! It checks Karl's earlier complaints directly: the title reacts to the mouse, START GAME starts a game (no freeze, no
//! quit after the start-up screens), the ball serves and the game scores; and the editor saves its levels.

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::input::{Host, Mapper};
use native_engine::pack::Pack;
use native_engine::{audio::Mixer, save::Save};

fn pack() -> Option<Pack> {
    let p = std::env::var("G336_PACK").ok()?;
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
        let s = g336::spec();
        Run {
            g: (s.make)(p, &Options { seed: Some(1) }).unwrap(),
            m: Mapper::new(s.info.bindings.clone()),
            mix: Mixer::new(44_100, 50, 4),
            save: Save::memory(),
            tick: 0,
        }
    }
    fn step(&mut self, ev: &[Host]) -> Flow {
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
        })
    }
    fn run(&mut self, n: u32) {
        for _ in 0..n {
            assert_eq!(
                self.step(&[]),
                Flow::Continue,
                "the game quit at tick {}",
                self.tick
            );
        }
    }
    fn get(&self, k: &str) -> i64 {
        self.g
            .state()
            .into_iter()
            .find(|s| s.0 == k)
            .map(|s| s.1)
            .unwrap_or_else(|| panic!("no state {k}"))
    }
    /// Moves the pointer's hot spot to (x, y) and clicks.
    fn click(&mut self, x: i64, y: i64) {
        loop {
            let (px, py) = (self.get("ptr_x") + 7, self.get("ptr_y") + 7);
            if (px, py) == (x, y) {
                break;
            }
            let dx = (x - px).clamp(-50, 50) as i32;
            let dy = (y - py).clamp(-50, 50) as i32;
            self.step(&[Host::MouseMove(dx, dy)]);
        }
        self.step(&[Host::MouseButton(true, true)]);
        self.run(3);
        self.step(&[Host::MouseButton(true, false)]);
        self.run(3);
    }
    fn wait_for(&mut self, k: &str, v: i64, max: u32) {
        for _ in 0..max {
            if self.get(k) == v {
                return;
            }
            self.run(1);
        }
        panic!("{k} never became {v} (tick {})", self.tick);
    }
}

#[test]
fn start_game_plays_and_scores() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    // Intro, credits and the title (the copy check is gone).
    r.run(800);
    assert_eq!(r.get("mode"), 0);
    r.click(0x98, 0xa8);
    r.wait_for("mode", 2, 50);
    r.wait_for("ball1_img", 0x4c, 1200);
    assert_eq!(r.get("p1_lives"), 3);
    // The ball serves by itself after a while or on a click; keep the bat under it and the game goes on.
    r.step(&[Host::MouseButton(true, true)]);
    r.run(2);
    r.step(&[Host::MouseButton(true, false)]);
    let mut scored = false;
    for _ in 0..3000 {
        let d = (r.get("ball1_x") - r.get("bat_x") - 8).clamp(-20, 20) as i32;
        assert_eq!(r.step(&[Host::MouseMove(d, 0)]), Flow::Continue);
        if r.get("p1_score") > 0 {
            scored = true;
            break;
        }
    }
    assert!(scored, "no brick hit in 3000 ticks");
}

#[test]
fn editor_saves_custom_levels() {
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    r.run(800);
    r.click(0x98, 0x98);
    r.wait_for("mode", 1, 50);
    r.run(200);
    r.click(0x34, 0x64);
    r.click(0x80, 0x50);
    // SAVE, then EXIT.
    r.click(0x30, 0xa6);
    r.run(100);
    assert!(
        r.save.get("custom_bricks").is_some(),
        "the editor's save wrote nothing"
    );
    r.click(0x30, 0xb2);
    r.wait_for("mode", 0, 100);
}

#[test]
fn a_touch_presses_the_button_under_it() {
    // Found on the Shield (1 Nov 2026): a tap far from the pointer landed on the wrong button, because the jump was
    // passed through the mouse counters, which carry at most 127 a frame. A touch now places the pointer directly.
    let Some(p) = pack() else { return };
    let mut r = Run::new(&p);
    r.run(800);
    assert_eq!((r.get("ptr_x"), r.get("ptr_y")), (0, 0));
    r.step(&[
        Host::PointerAt(0x98, 0xa8),
        Host::Touch("f".into(), native_engine::input::Action::Primary, true),
    ]);
    r.step(&[
        Host::PointerAt(0x98, 0xa8),
        Host::Touch("f".into(), native_engine::input::Action::Primary, false),
    ]);
    r.wait_for("mode", 2, 20);
}
