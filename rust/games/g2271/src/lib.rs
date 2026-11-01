//! Game 2271, Castlevania (Konami 1990, Amiga conversion by Novotrade), rewritten natively on native_engine from its
//! recovered code (Ami-Go native, batch 1). The main program is in `castle`, its pictures in `screen`, the asset
//! loading in `assets`, and `co` runs the main program in lock-step with the engine's ticks.

pub mod assets;
pub mod castle;
pub mod co;
pub mod screen;
pub mod stage;

/// The engine, for the Android library (which reaches it through this crate).
pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bind, Bindings};
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
            ("x", s("start (title menu)")),
            ("back", s("back to the title")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: walk, crouch, stairs, menu"),
            s("Space or Ctrl: whip (fire)"),
            s("Alt, Shift or Z: jump"),
            s("Return: start (title menu)"),
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

/// The joystick game's bindings, with keys for the second button (jump).
pub fn bindings() -> Bindings {
    let mut b = Bindings::joystick_game();
    for (k, v) in [
        ("Left Ctrl", Action::Fire),
        ("Left Alt", Action::Fire2),
        ("Right Alt", Action::Fire2),
        ("Left Shift", Action::Fire2),
        ("Right Shift", Action::Fire2),
        ("Z", Action::Fire2),
    ] {
        b.keys.push((k.to_string(), Bind::Act(v)));
    }
    b
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
