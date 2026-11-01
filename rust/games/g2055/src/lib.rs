//! Game 2055, Speedball 2 - Brutal Deluxe (The Bitmap Brothers, 1990; the CSL crack), rewritten natively on
//! native_engine from its recovered code (Ami-Go native, batch 1). See docs/native/g2055-native.md.
//!
//! Part 1 (this crate so far): the whole front end the original runs before the game loads: the crack's
//! CRYSTAL CRACKED lines (kept as the opening intro page), the Bitmap Brothers pages, the title, the story and the
//! credits over the arena, and the LOADING GAME page. The program runs on its own thread in lock-step with the
//! engine's ticks (`co`), so the intro module reads as its original routines did (`intro`); `world` holds the typed
//! state it works on (two screens, the colour registers, the frame counters and the flags).

pub mod assets;
pub mod co;
pub mod intro;
pub mod world;

/// The engine, for the Android library (which reaches it through this crate).
pub use native_engine;

use native_engine::frontdoor::ControlSheet;
use native_engine::game::{Game, GameInfo, Options, Spec};
use native_engine::input::{Action, Bindings};
use native_engine::pack::Pack;

// EMBEDDED_PACK: Option<&[u8]>, written by build.rs (NATIVE_PACK at build time)
include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

pub fn make(p: &Pack, _o: &Options) -> Result<Box<dyn Game>, String> {
    Ok(Box::new(intro::Speedball::new(assets::Assets::load(p)?)))
}

/// The controls as the front door shows them (a joystick game).
pub fn sheet() -> ControlSheet {
    let s = |v: &str| v.to_string();
    ControlSheet {
        pad: vec![
            ("dpad", s("move")),
            ("left_stick", s("move")),
            ("a", s("fire: pass, shoot, tackle; skip the intro")),
            ("x", s("fire")),
            ("start", s("pause menu")),
        ],
        keys: vec![
            s("Arrow keys: move"),
            s("Space or Right Ctrl: fire (skips the intro)"),
            s("F10: pause menu"),
        ],
        start_hint: Some(s("Press fire to skip the intro")),
        touch_buttons: vec![(Action::Fire, s("Fire"))],
        touch_pointer: false,
    }
}

pub fn spec() -> Spec {
    Spec {
        info: GameInfo {
            title: "Speedball 2 - Brutal Deluxe".into(),
            id: "g2055".into(),
            width: world::SCREEN_W,
            height: world::SCREEN_H,
            tick_hz: 50,
            bindings: Bindings::joystick_game(),
            sheet: sheet(),
            effect_voices: 4,
        },
        make,
    }
}
