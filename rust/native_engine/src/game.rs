//! What a game gives the engine: its description (`GameInfo`) and a `Game` that advances one tick at a time and draws
//! its frame. A game is built from its asset pack by the `Spec::make` function, so the platform layer can start it
//! again (Restart in the pause menu) and the headless runner can run it without a display.

use crate::audio::Mixer;
use crate::frontdoor::ControlSheet;
use crate::gfx::Frame;
use crate::input::{Bindings, InputState};
use crate::pack::Pack;
use crate::save::Save;

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub title: String,
    /// Short id (file names, storage directory).
    pub id: String,
    /// The logical resolution the game draws at.
    pub width: usize,
    pub height: usize,
    /// Logic ticks a second (50 for a PAL game).
    pub tick_hz: u32,
    pub bindings: Bindings,
    pub sheet: ControlSheet,
    /// Sound-effect voices the game uses (beside the music player's four).
    pub effect_voices: usize,
}

/// Start options a game may read.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// A fixed random seed (verification runs); None = from the clock, as the original seeded itself.
    pub seed: Option<u32>,
}

/// What one tick may use.
pub struct Ctx<'a> {
    pub input: &'a InputState,
    pub audio: &'a mut Mixer,
    pub save: &'a mut Save,
    pub tick: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    /// The game asks to end the program.
    Quit,
}

pub trait Game {
    /// One logic tick.
    fn tick(&mut self, cx: &mut Ctx) -> Flow;
    /// The current frame.
    fn render(&self, frame: &mut Frame);
    /// Named state values for verification (compared with the original's).
    fn state(&self) -> Vec<(String, i64)> {
        Vec::new()
    }
}

/// Builds a game from its asset pack.
pub type MakeFn = fn(&Pack, &Options) -> Result<Box<dyn Game>, String>;

/// A game as the platform layer starts it.
pub struct Spec {
    pub info: GameInfo,
    pub make: MakeFn,
}
