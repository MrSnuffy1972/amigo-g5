//! Game 2271, Castlevania (Konami 1990, Amiga conversion by Novotrade), rewritten natively on native_engine from its
//! recovered code (Ami-Go native, batch 1). The main program is in `castle`, its pictures in `screen`, the asset
//! loading in `assets`, and `co` runs the main program in lock-step with the engine's ticks.

pub mod assets;
pub mod castle;
pub mod co;
pub mod screen;

/// The engine, for the Android library (which reaches it through this crate).
pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bindings};
use native_engine::pack::Pack;

// EMBEDDED_PACK: Option<&[u8]>, written by build.rs (NATIVE_PACK at build time)
include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn make(p: &Pack, o: &Options) -> Result<Box<dyn Game>, String> {
    Ok(Box::new(co::Native::new(assets::Assets::load(p)?, o)))
}

/// The controls as the front door shows them (a joystick game).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("walk, crouch, stairs, menu")),
            ("left_stick", s("walk, crouch, stairs, menu")),
            ("a", s("whip (fire)")),
            ("b", s("jump")),
            ("x", s("pause")),
            ("back", s("back to the title")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: walk, crouch, stairs, menu"),
            s("Ctrl or Return: whip (fire)"),
            s("Alt or Shift: jump"),
            s("Space or P: pause"),
            s("Escape: back to the title"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Press fire on Start a New Game")),
        touch_buttons: vec![
            (Action::Fire, s("Whip")),
            (Action::Fire2, s("Jump")),
            (Action::Pause, s("Pause")),
        ],
        touch_pointer: false,
    }
}

pub fn bindings() -> Bindings {
    Bindings::joystick_game()
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Castlevania".into(),
            id: "g2271".into(),
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
