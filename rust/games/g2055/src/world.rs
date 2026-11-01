//! The state the front end works on, as typed values: the two screens the original draws into and shows in turn,
//! the colour registers, the frame counters its waits use, and the intro flags.

use native_engine::gfx::Bitmap;

pub const SCREEN_W: usize = 320;
pub const SCREEN_H: usize = 200;

/// Where the front end is (the `stage` the verification reads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Before the first page.
    Boot = 0,
    /// The two opening pages on black (the crack's CRYSTAL CRACKED, then A BITMAP BROTHERS GAME), with sound.
    Opening = 1,
    /// The title picture.
    Title = 2,
    /// The story pages over the arena.
    Story = 3,
    /// The credit pages over the arena.
    Credits = 4,
    /// LOADING GAME over the arena.
    Loading = 5,
    /// The intro is over (the main program starts).
    GameStart = 6,
    /// The menus (part 2).
    Menu = 7,
}

/// Where a menu selection leads: the routine the original runs next, which is a later part of the port (the `next`
/// the verification reads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Next {
    None = 0,
    /// KNOCKOUT: team selection (`cycle_team_selection_screen`).
    Knockout = 1,
    /// LEAGUE, NEW GAME.
    LeagueNew = 2,
    /// LEAGUE, TEAM MANAGER.
    LeagueManager = 3,
    /// LEAGUE, LOAD GAME (`process_league_menu_input`).
    LeagueLoad = 4,
    /// CUP, NEW GAME (`reset_league_tables_and_teams`).
    CupNew = 5,
    /// CUP, LOAD GAME.
    CupLoad = 6,
    /// PRACTICE.
    Practice = 7,
    /// 2 PLAYER GAME (`run_player_management_loop`).
    TwoPlayers = 8,
    /// DEMO GAME, or 1500 idle frames on the first menu.
    Demo = 9,
    /// REPLAY GOALS (`initialize_game_session_data`).
    ReplayGoals = 10,
}

pub struct World {
    /// The two screens (320 x 200, colour indices 0-31).
    pub screens: [Bitmap; 2],
    /// The screen the program has handed to the display (the original's shown-screen pointer); the other one is
    /// drawn into.
    pub shown: usize,
    /// The screen the display shows this frame: the vertical-blank handler takes `shown` at the next frame.
    pub display: usize,
    /// The 32 colour registers (12-bit).
    pub colours: [u16; 32],
    /// The colours of the picture decoded last (the original's picture palette).
    pub picture: [u16; 32],
    /// The frame counter the vertical blank increments; its low byte is the countdown the frame waits use.
    pub frames: u32,
    /// The second frame counter (waits for "the next frame").
    pub vbl: u16,
    /// Sound on (the opening pages type with sound; cleared at the title).
    pub sound: bool,
    /// In the intro (fire skips; cleared for LOADING GAME).
    pub intro: bool,
    /// Fire (joystick fire or Space) held this frame.
    pub fire: bool,
    pub stage: Stage,
    /// The page being typed (index in its group), the typing position (column byte, row in 4-line units).
    pub page: usize,
    pub cursor: (u16, u16),
    /// Sound effects asked for: the last one and how many (the sound driver is a later part of the port).
    pub sfx_last: u8,
    pub sfx_count: u32,
    /// Joystick events since the menu last cleared them (the interrupt's event byte: up 1, down 2, fire `$80`).
    pub joy: u8,
    /// The last key pressed (the keyboard handler's byte; Esc is `$45`).
    pub key: u8,
    /// The chosen menu item (the original's `$34A2`), the menu on screen, and Esc pressed in a menu (`$172` bit 6).
    pub choice: u16,
    pub menu: u8,
    pub escape: bool,
    /// Frames since the first menu last redrew (`$EA`; the demo starts at 1500).
    pub idle: u16,
    /// The game mode (`$EF`: 1 league, 2 cup) and the three purses (`$1EC`-`$1F0`).
    pub mode: u8,
    pub money: [u16; 3],
    /// Where the last selection leads, and how many selections were made.
    pub next: Next,
    pub picks: u32,
}

impl Default for World {
    fn default() -> Self {
        World {
            screens: [
                Bitmap::new(SCREEN_W, SCREEN_H),
                Bitmap::new(SCREEN_W, SCREEN_H),
            ],
            shown: 0,
            display: 0,
            colours: [0; 32],
            picture: [0; 32],
            frames: 0,
            vbl: 0,
            sound: false,
            intro: false,
            fire: false,
            stage: Stage::Boot,
            page: 0,
            cursor: (0, 0),
            sfx_last: 0,
            sfx_count: 0,
            joy: 0,
            key: 0,
            choice: 0,
            menu: 0,
            escape: false,
            idle: 0,
            mode: 0,
            money: [0; 3],
            next: Next::None,
            picks: 0,
        }
    }
}

impl World {
    /// The screen being drawn into.
    pub fn back(&self) -> usize {
        1 - self.shown
    }

    /// The low byte of the frame counter.
    pub fn countdown(&self) -> u8 {
        self.frames as u8
    }

    pub fn set_countdown(&mut self, v: u8) {
        self.frames = (self.frames & !0xFF) | v as u32;
    }

    /// What the vertical-blank handler does each frame.
    pub fn vertical_blank(&mut self) {
        self.frames = self.frames.wrapping_add(1);
        self.vbl = self.vbl.wrapping_add(1);
        self.idle = self.idle.wrapping_add(1);
        self.display = self.shown;
    }

    /// Hands the drawn screen to the display and draws into the other one from now on.
    pub fn swap_screens(&mut self) {
        self.shown = self.back();
    }

    pub fn sfx(&mut self, n: u8) {
        self.sfx_last = n;
        self.sfx_count += 1;
    }
}
