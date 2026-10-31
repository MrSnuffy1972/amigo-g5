//! native_engine: the shared, system-neutral engine for Ami-Go's native game rewrites (native stage 1, 31 Oct 2026,
//! docs/G5-NATIVE.md "Added by native stage 1"). A game written natively (its own typed state and rules) plugs in
//! through `game::Game` and gets:
//!
//! - a fixed-rate game loop (the game's own tick rate, 50 Hz for a PAL game) drawn at the display's rate (`timing`,
//!   `platform`);
//! - an indexed-colour renderer for game-level draw calls: bitmaps, sprite sheets, masks, tile-free maps, lines, bitmap
//!   fonts, palettes and palette effects, at a logical resolution scaled to the screen (`gfx`);
//! - a mixer for samples with pitch, volume, pan and loops, and a native tracker module player (`audio`, `tracker`);
//! - input as abstract actions and a pointer, from pad, keyboard, mouse, touch and TV remote (`input`, on-screen
//!   controls in `frontdoor::touch`);
//! - save data (`save`), the versioned asset-pack format (`pack`), scripted headless runs (`script`, `headless`);
//! - the front door every game opens on: the pre-boot menu, the pause menu (`frontdoor`);
//! - desktop and Android through SDL2 (`platform`, feature `sdl`).
//!
//! Nothing in this crate is specific to any old machine: no emulated memory, processor or chips.

pub mod audio;
pub mod frontdoor;
pub mod game;
pub mod gfx;
pub mod headless;
pub mod input;
pub mod pack;
#[cfg(feature = "sdl")]
pub mod platform;
pub mod save;
pub mod script;
pub mod timing;
pub mod tracker;
