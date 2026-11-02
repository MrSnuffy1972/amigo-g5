//! Ami-Go native: game 2035, Sensible Soccer (Sensible Software / Renegade 1992, the Crack Inc release), rewritten
//! natively on `native_engine` from its recovered code (docs/native/g2035-recovery.md, g2035-native.md). Typed Rust
//! state only: no guest memory, no 68000 registers, no chip registers. The Rob Northen Copylock and the five key-decoded
//! trap jumps are not ported (the port calls what the correct key selects); the Crack Inc intro is kept as the intro.
//! Assets come from the asset pack (built outside the repo by the extractor), never from the repo.

pub mod assets;
pub mod camera;
pub mod flow;
pub mod intro;
pub mod menu;
pub mod pitch;
pub mod rng;

pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bindings};
use native_engine::pack::Pack;

include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn make(p: &Pack, o: &Options) -> Result<Box<dyn Game>, String> {
    Ok(Box::new(flow::Native::new(assets::Assets::load(p)?, o)))
}

pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("run, aim, menus")),
            ("left_stick", s("run, aim, menus")),
            ("a", s("pass, shoot (hold), slide tackle")),
            ("start", s("pause menu")),
            ("back", s("back")),
        ],
        keys: vec![
            s("Arrow keys: run, aim, menus"),
            s("Space or Ctrl: pass, shoot (hold), slide tackle"),
            s("Escape: back"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Press fire to skip the intro")),
        touch_buttons: vec![(Action::Fire, s("Kick")), (Action::Pause, s("Pause"))],
        touch_pointer: false,
    }
}

pub fn bindings() -> Bindings {
    Bindings::joystick_game()
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Sensible Soccer".into(),
            id: "g2035".into(),
            width: assets::W,
            height: assets::H,
            tick_hz: 50,
            bindings: bindings(),
            sheet: sheet(),
            effect_voices: 4,
        },
        make,
    }
}
