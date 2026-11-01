//! Game 2179, The Great Giana Sisters (Rainbow Arts / Time Warp, 1987), rewritten natively on native_engine from its
//! recovered code (Ami-Go native batch 1). The program is in `giana` and the modules beside it (`flow`, `player`,
//! `blocks`, `enemies`, `hiscore`), the music in `music`, the asset loading in `assets`, and `co` runs the program in
//! lock-step with the engine's ticks.

pub mod assets;
pub mod blocks;
pub mod co;
pub mod enemies;
pub mod flow;
pub mod giana;
pub mod hiscore;
pub mod music;
pub mod player;

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

/// The controls as the front door shows them (a joystick game; the WHDLoad install's second button jumps).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("walk; up jumps; down + fire uses items")),
            ("left_stick", s("walk; up jumps")),
            ("a", s("fire (dream bubbles, start, continue)")),
            ("b", s("jump")),
            ("y", s("mouse button: end the game, save high scores")),
            ("back", s("mouse button: end the game, save high scores")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: walk, Up jumps"),
            s("Right Ctrl, Space or Left Ctrl: fire"),
            s("Left Alt or Z: jump"),
            s("Escape: mouse button (end the game; save high scores)"),
            s("] or F9: trainer, life loss on or off (off at start)"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s(
            "Fire past the logo and title, then left for one player, right for two",
        )),
        touch_buttons: vec![
            (Action::Fire, s("Fire")),
            (Action::Fire2, s("Jump")),
            (Action::Cancel, s("Mouse")),
        ],
        touch_pointer: false,
    }
}

pub fn bindings() -> Bindings {
    let mut b = Bindings::joystick_game();
    for (c, bind) in b.pad.iter_mut() {
        if *c == "y" {
            *bind = Bind::Act(Action::Cancel);
        }
    }
    for (k, v) in [
        ("Left Ctrl", Action::Fire),
        ("Left Alt", Action::Fire2),
        ("Z", Action::Fire2),
        ("]", Action::Secondary),
        ("F9", Action::Secondary),
    ] {
        b.keys.push((k.to_string(), Bind::Act(v)));
    }
    b
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "The Great Giana Sisters".into(),
            id: "g2179".into(),
            width: assets::W,
            height: assets::H,
            tick_hz: 50,
            bindings: bindings(),
            sheet: sheet(),
            effect_voices: 0,
        },
        make,
    }
}
