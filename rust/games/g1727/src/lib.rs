//! Game 1727, Volfied (Taito 1989, Amiga conversion by Empire / Arcade Masters 1991), rewritten natively on
//! native_engine from its recovered code (Ami-Go native, batch 2). Typed state only: the playfield in `field`, a
//! round (player, cut, claim, enemies) in `play`, the random number generator in `rng`, the main program and HUD in
//! `volfied`, the asset pack in `assets`. Work in progress: see docs/native/g1727-native.md for what is translated
//! and what is still a stand-in.

pub mod assets;
pub mod field;
pub mod motion;
pub mod play;
pub mod rng;
pub mod volfied;

/// The engine, for the Android library (which reaches it through this crate).
pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bindings};
use native_engine::pack::Pack;

// EMBEDDED_PACK: Option<&[u8]>, written by build.rs (NATIVE_PACK at build time)
include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn make(p: &Pack, o: &Options) -> Result<Box<dyn Game>, String> {
    Ok(Box::new(volfied::Native::new(assets::Assets::load(p)?, o)))
}

/// The controls as the front door shows them (a joystick game).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move along the edge, cut")),
            ("left_stick", s("move along the edge, cut")),
            ("a", s("hold to cut into the field")),
            ("x", s("start (title)")),
            ("back", s("back to the title")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: move; up and down choose on the title"),
            s("Space or Ctrl: hold to cut, start on the title"),
            s("Escape: back to the title"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Press fire on the title to start")),
        touch_buttons: vec![(Action::Fire, s("Cut")), (Action::Pause, s("Pause"))],
        touch_pointer: false,
    }
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Volfied".into(),
            id: "g1727".into(),
            width: assets::W,
            height: assets::H,
            tick_hz: 50,
            bindings: Bindings::joystick_game(),
            sheet: sheet(),
            effect_voices: 4,
        },
        make,
    }
}
