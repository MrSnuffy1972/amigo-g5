//! Game 1598, PGA Tour Golf (Electronic Arts 1991, the AGL one-disk release with TPC Sawgrass and TPC Avenel),
//! rewritten natively on native_engine from its recovered code (Ami-Go native, batch 2). The 3D hole view is in
//! `scene` and `hole`, the original's integer arithmetic in `fixed`, the main program in `golf` (run in lock-step by
//! `co`), the intro text page in `intro`, the asset loading in `assets`.

pub mod assets;
pub mod co;
pub mod fixed;
pub mod golf;
pub mod hole;
pub mod intro;
pub mod scene;
pub mod text;

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

/// The controls as the front door shows them (a mouse game).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move the pointer")),
            ("left_stick", s("move the pointer")),
            ("right_stick", s("move the pointer")),
            ("a", s("click (swing, choose)")),
            ("b", s("second button (back)")),
            ("back", s("back")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Mouse: point and click, as the original"),
            s("Arrow keys: move the pointer"),
            s("Space or Return: click"),
            s("Escape or right button: back"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Click to begin")),
        touch_buttons: vec![(Action::Secondary, s("Back")), (Action::Pause, s("Pause"))],
        touch_pointer: true,
    }
}

pub fn bindings() -> Bindings {
    Bindings::pointer_game()
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "PGA Tour Golf".into(),
            id: "g1598".into(),
            width: assets::DW,
            height: assets::DH,
            tick_hz: 50,
            bindings: bindings(),
            sheet: sheet(),
            effect_voices: 4,
        },
        make,
    }
}
