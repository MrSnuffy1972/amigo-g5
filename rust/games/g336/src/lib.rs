//! Game 336, Jail Break (Odyssey, 1990), rewritten natively on native_engine from its recovered code (Ami-Go native,
//! second game). The game is in `jail`, the asset loading in `assets`, and `co` runs the main program in lock-step
//! with the engine's ticks.

pub mod assets;
pub mod co;
pub mod jail;

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

/// The controls as the front door shows them (a mouse game: the mouse moves the bat, and the pointer on the title
/// and in the editor).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move the bat / pointer")),
            ("left_stick", s("move the bat / pointer")),
            ("right_stick", s("move the bat / pointer")),
            ("a", s("launch, fire, click")),
            ("b", s("right button (editor: clear)")),
            ("x", s("pause the game")),
            ("back", s("end the game (title: quit)")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Mouse: bat, pointer and both buttons"),
            s("Arrow keys: move the bat or pointer"),
            s("Return: left button"),
            s("Right Ctrl: right button"),
            s("Space or P: pause the game"),
            s("Escape: end the game (on the title: quit)"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Click START GAME on the title")),
        touch_buttons: vec![
            (Action::Primary, s("Fire")),
            (Action::Start, s("Pause")),
            (Action::Cancel, s("Esc")),
        ],
        touch_pointer: true,
    }
}

/// A pointer game's controls, with Space as the original's pause key (the original's left button is Return and pad A).
pub fn bindings() -> Bindings {
    let mut b = Bindings::pointer_game();
    for (k, bind) in b.keys.iter_mut() {
        if k == "Space" {
            *bind = Bind::Act(Action::Start);
        }
    }
    b.keys.push(("P".to_string(), Bind::Act(Action::Start)));
    for (c, bind) in b.pad.iter_mut() {
        if *c == "x" {
            *bind = Bind::Act(Action::Start);
        }
    }
    b
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Jail Break".into(),
            id: "g336".into(),
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
