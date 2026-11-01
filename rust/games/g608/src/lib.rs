//! Game 608, E-Motion (US Gold / The Assembly Line, 1990), rewritten natively on native_engine from its recovered
//! code (Ami-Go native, batch 1). The main program is in `game` and `objects`, its frame and sound channels in `emo`,
//! the drawing in `draw`, the physics arithmetic in `ops`, the crack intro in `intro` and `soundmon`, the asset loading
//! in `assets`, and `co` runs the main program in lock-step with the engine's ticks.

pub mod assets;
pub mod co;
pub mod draw;
pub mod emo;
pub mod game;
pub mod intro;
pub mod objects;
pub mod ops;
pub mod soundmon;

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

impl emo::Emo {
    /// The whole program: the crack intro, the loader's pictures, then the game.
    pub fn run(&mut self) {
        self.intro();
        self.loader();
        self.game_main();
    }
}

/// The controls as the front door shows them (a joystick game).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("left/right turn, down turns round")),
            ("left_stick", s("left/right turn, down turns round")),
            ("a", s("thrust, start, choose")),
            ("b", s("intro menu: switch off")),
            ("x", s("pause")),
            ("back", s("quit to the title (Esc)")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: turn (left, right), turn round (down), up"),
            s("Space or Ctrl: thrust / fire"),
            s("P: pause, Escape: back to the title"),
            s("Intro menu: up/down pick, fire switches on, Fire 2 off"),
            s("High-score name: up/down letter, right next, left rub out, fire done"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Fire on START GAME, then fire on the title")),
        touch_buttons: vec![
            (Action::Fire, s("Thrust")),
            (Action::Fire2, s("Off/Pause")),
            (Action::Cancel, s("Esc")),
        ],
        touch_pointer: false,
    }
}

/// A joystick game's controls, with P as the pause key and Escape back to the title.
pub fn bindings() -> Bindings {
    let mut b = Bindings::joystick_game();
    b.keys.push(("P".to_string(), Bind::Act(Action::Pause)));
    b.keys
        .push(("Escape".to_string(), Bind::Act(Action::Cancel)));
    b
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "E-Motion".into(),
            id: "g608".into(),
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
