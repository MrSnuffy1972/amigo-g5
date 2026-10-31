//! Game 296, Global Thermonuclear Warfare (Mark Sheeky, Scorpius Software, 1994), rewritten natively on native_engine
//! from its recovered code (Ami-Go native stage 1). The rules and screens are in `gtw`, the asset loading in `assets`.

pub mod assets;
pub mod gtw;

/// The engine, for the Android library (which reaches it through this crate).
pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bindings};
use native_engine::pack::Pack;

// EMBEDDED_PACK: Option<&[u8]>, written by build.rs (NATIVE_PACK at build time)
include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn make(p: &Pack, o: &Options) -> Result<Box<dyn Game>, String> {
    Ok(Box::new(gtw::Gtw::new(assets::Assets::load(p)?, o)))
}

/// The controls as the front door shows them (a mouse game: everything moves and clicks the pointer).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move the pointer")),
            ("left_stick", s("move the pointer")),
            ("right_stick", s("move the pointer")),
            ("a", s("click (left button)")),
            ("b", s("right button: repeat the arrows")),
            ("lb", s("left button")),
            ("rb", s("right button")),
            ("back", s("leave the war (America's turn)")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Mouse: pointer and both buttons"),
            s("Arrow keys: move the pointer"),
            s("Space or Return: left button"),
            s("Right Ctrl: right button"),
            s("Escape: leave the war"),
            s("F10: pause menu"),
            s("Pointer at the left or right edge: scroll the map"),
        ],
        start_hint: Some(s("Click at the title, set the options, then PLAY")),
        touch_buttons: vec![(Action::Secondary, s("Right")), (Action::Cancel, s("Esc"))],
        touch_pointer: true,
    }
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Global Thermonuclear Warfare".into(),
            id: "g296".into(),
            width: gtw::SCREEN_W,
            height: gtw::SCREEN_H,
            tick_hz: 50,
            bindings: Bindings::pointer_game(),
            sheet: sheet(),
            effect_voices: 4,
        },
        make,
    }
}
