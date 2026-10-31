//! The front door every native game opens on: the pre-boot menu of the G5 milestones M8 to M11a (a 30 second
//! countdown that starts the game by itself, the controller and keyboard panels, Start game, Controls and Quit by pad,
//! keys, mouse, touch and TV remote), the in-game pause menu and the on-screen touch controls. All of it is drawn on
//! the CPU in the metallic-and-neon theme, so it needs no display to test or screenshot.

pub mod menu;
pub mod pad;
pub mod preboot;
pub mod theme;
pub mod touch;

use crate::input::Action;

/// What the front door shows about a game's controls, written by the game from its bindings.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ControlSheet {
    /// Each mapped pad control (names as in input::PAD_CONTROLS) and what it does in this game.
    pub pad: Vec<(&'static str, String)>,
    /// The keyboard and mouse panel, one line each ("Escape: back to the title").
    pub keys: Vec<String>,
    /// How the game is started, shown under the title.
    pub start_hint: Option<String>,
    /// The on-screen buttons for phones (at most four).
    pub touch_buttons: Vec<(Action, String)>,
    /// Touches on the picture move the pointer and press the primary button (a pointer game).
    pub touch_pointer: bool,
}
