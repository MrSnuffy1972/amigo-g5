//! Game 1907, International Karate + (System 3, 1988), rewritten natively on native_engine from its recovered code
//! (Ami-Go native, batch 1). The main program is in `ik` (with `flow`, `fight`, `speech`, `hiscore`, `bonus`,
//! `attract`, `ambient`), the music player in `music`, the asset loading in `assets`, and `co` runs the main program
//! in lock-step with the engine's ticks.

pub mod ambient;
pub mod assets;
pub mod attract;
pub mod bonus;
pub mod co;
pub mod fight;
pub mod flow;
pub mod hiscore;
pub mod ik;
pub mod music;
pub mod skew;
pub mod speech;

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

/// The controls as the front door shows them. IK+ is a joystick game: the eight directions with and without fire
/// give its sixteen moves (shown for a fighter facing right; they mirror when facing left).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move: walk, jump, crouch, somersault")),
            ("left_stick", s("move (8 directions)")),
            ("a", s("fire: with a direction, the kicks and punches")),
            ("b", s("two-player game (F2)")),
            ("x", s("start a one-player game (F1)")),
            ("y", s("music on / off (F3)")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrows: the joystick; Right Ctrl or Space: fire"),
            s("Up: jump; Down: crouch; away: walk back; towards: walk"),
            s("Fire + up: high kick; fire + down: low punch/sweep"),
            s("Fire + towards: punches and kicks; fire + away: back kick"),
            s("Fire + diagonals: flying kick, round kick, head butt..."),
            s("Return or F1: one-player game; F2: two players"),
            s("Player 2: W A S D and Left Shift (fire); F10: pause menu"),
        ],
        start_hint: Some(s("Press fire to play")),
        touch_buttons: vec![
            (Action::Fire, s("Fire")),
            (Action::Start, s("1P")),
            (Action::Fire2, s("2P")),
        ],
        touch_pointer: false,
    }
}

pub fn bindings() -> Bindings {
    use native_engine::input::Bind;
    let mut b = Bindings::joystick_game();
    b.pad.retain(|(c, _)| *c != "y");
    b.pad.push(("y", Bind::Act(Action::Secondary)));
    b.keys.push(("F1".to_string(), Bind::Act(Action::Start)));
    b.keys.push(("F2".to_string(), Bind::Act(Action::Fire2)));
    b.keys
        .push(("F3".to_string(), Bind::Act(Action::Secondary)));
    // the second joystick (port 1) on the keyboard: W A S D and Left Shift
    for (k, a) in [
        ("W", Action::Up),
        ("S", Action::Down),
        ("A", Action::Left),
        ("D", Action::Right),
        ("Left Shift", Action::Fire),
    ] {
        b.keys.push((k.to_string(), Bind::P2(a)));
    }
    b
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "International Karate +".into(),
            id: "g1907".into(),
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
