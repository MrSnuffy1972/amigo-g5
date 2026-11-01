//! The main program of Castlevania (Konami 1990, Amiga conversion by Novotrade), rewritten from the recovered code.
//! Each routine keeps the original's name (from the recovered function list) in its doc comment, with its offset in
//! the program (`+XXXX`), so the function map reads straight across.
//!
//! Port status (part 1): the crack's boot text page, the title menu and the room drawing. The stage engine, the
//! player, enemies, bosses, sub-weapons, the panel and the sound follow in the next parts (see
//! docs/native/g2271-native.md).

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::input::{Action, InputState};

use crate::assets::{Assets, H, W};
use crate::co::{FrameIn, FrameOut, Stop};
use crate::screen::{Into, Screen};

/// Workbench 1.3's CLI colours, for the boot text page: blue paper, white ink.
const WB_PAPER: u16 = 0x05A;
const WB_INK: u16 = 0xFFF;

/// The joystick as the game reads it each frame (+15AA read_player_controls).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Controls {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    /// No direction held.
    pub centre: bool,
    /// Fire button: held, and pressed this frame (-$1364, -$1920).
    pub fire_held: bool,
    pub fire: bool,
    /// Second button (the original's other port bit): held, and pressed this frame (-$1366, -$191E).
    pub fire2_held: bool,
    pub fire2: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Intro,
    Title,
    Room,
}

pub struct Castle {
    a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    pub scr: Screen,
    input: InputState,
    pub ctl: Controls,
    pub frame: u32,
    pub phase: Phase,
    /// Title menu cursor line.
    pub title_y: i16,
    /// Room number (-$19B0) and its first map column on screen (-$19AE).
    pub room: i16,
    pub scroll_col: i16,
    #[allow(dead_code)]
    seed: Option<u32>,
}

impl Castle {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Castle {
        Castle {
            a,
            rx,
            tx,
            scr: Screen::new(W),
            input: InputState::default(),
            ctl: Controls::default(),
            frame: 0,
            phase: Phase::Intro,
            title_y: 0,
            room: 0,
            scroll_col: 0,
            seed,
        }
    }

    pub fn state(&self) -> Vec<(String, i64)> {
        vec![
            ("frame".into(), self.frame as i64),
            ("phase".into(), self.phase as i64),
            ("title_y".into(), self.title_y as i64),
            ("room".into(), self.room as i64),
            ("scroll_col".into(), self.scroll_col as i64),
        ]
    }

    /// The frame wait (+CBB8 wait_for_vblank_and_clear_flags): shows the picture drawn, takes the next frame's
    /// input, and restores the boxes of the shapes drawn into the next picture.
    pub fn wait_frame(&mut self) {
        self.scr.flip();
        let (screen, palette) = self.scr.view();
        let out = FrameOut {
            screen,
            palette,
            quit: false,
            state: self.state(),
        };
        if self.tx.send(out).is_err() {
            std::panic::panic_any(Stop);
        }
        match self.rx.recv() {
            Ok(f) => self.input = f.input,
            Err(_) => std::panic::panic_any(Stop),
        }
        self.frame += 1;
        self.scr.restore_boxes();
    }

    /// +CC16 decrement_until_negative: waits n frames.
    pub fn wait_frames(&mut self, n: u32) {
        for _ in 0..n {
            self.wait_frame();
        }
    }

    /// +15AA read_player_controls: directions, and the two buttons with their press edges.
    pub fn read_controls(&mut self) {
        let i = &self.input;
        let mut c = Controls {
            up: i.held(Action::Up),
            down: i.held(Action::Down),
            right: i.held(Action::Right),
            left: i.held(Action::Left),
            ..Controls::default()
        };
        if c.up && c.down {
            c.down = false;
        }
        if c.left && c.right {
            c.left = false;
        }
        c.centre = !(c.up || c.down || c.left || c.right);
        let f1 = i.held(Action::Fire) || i.pressed(Action::Fire);
        let f2 = i.held(Action::Fire2) || i.pressed(Action::Fire2);
        c.fire = f1 && !self.ctl.fire_held;
        c.fire2 = f2 && !self.ctl.fire2_held;
        c.fire_held = f1;
        c.fire2_held = f2;
        self.ctl = c;
    }

    /// +0830 clear_state_flags_2: reads the controls once and forgets the presses.
    fn clear_presses(&mut self) {
        self.read_controls();
        self.ctl.fire = false;
        self.ctl.fire2 = false;
        self.ctl.up = false;
        self.ctl.down = false;
    }
}

impl Castle {
    /// The program: the crack's boot page, then the title menu for ever (+0006 main_program). The manual code
    /// lookup that followed the title (+A7C0 manual_password_check) is not ported: the crack made every answer
    /// pass, and the native build behaves as if it had passed.
    pub fn run(&mut self) {
        self.intro();
        loop {
            let choice = self.title_screen();
            if choice == 0 {
                self.room_view(0);
            }
        }
    }

    /// The crack's boot text page: s/quartex_speaks!, which the startup-sequence typed into the CLI window and
    /// left for three seconds (`wait 3`) before starting the game. Lines wider than the screen are wrapped.
    fn intro(&mut self) {
        self.phase = Phase::Intro;
        self.scr.resize(W);
        let mut pal = [WB_PAPER; 32];
        pal[1] = WB_INK;
        self.scr.palette = pal;
        self.scr.level = 15;
        let lines = wrap(&self.a.intro, W / 8);
        let top = (H / 8).saturating_sub(lines.len()) / 2;
        self.scr.ink = 1;
        self.scr.paper = 0;
        for (r, l) in lines.iter().enumerate() {
            let x0 = (W / 8 - l.chars().count().min(W / 8)) / 2;
            for (i, ch) in l.chars().enumerate().take(W / 8) {
                let g = self.glyph(ch);
                self.scr.char(
                    &g,
                    ((x0 + i) * 8) as i32,
                    ((top + r) * 8) as i32,
                    true,
                    true,
                );
            }
        }
        for _ in 0..self.a.intro_seconds * 50 {
            self.wait_frame();
            self.read_controls();
            if self.ctl.fire || self.input.pressed(Action::Start) {
                break;
            }
        }
    }

    fn glyph(&self, ch: char) -> [u8; 64] {
        let i = (ch as u32).wrapping_sub(0x20) as usize;
        self.a.font.get(i).copied().unwrap_or([0; 64])
    }

    /// +B5BC copy_handler_table_entries: the sprite colours and the room's colours, then a fade in (+CCC6).
    fn set_palette(&mut self, n: usize) {
        self.scr.palette = self.a.palettes[n.min(self.a.palettes.len() - 1)];
        self.fade_in();
    }

    /// +CCC6 clear_and_copy_word_table: 33 frames, each colour one step nearer its value.
    fn fade_in(&mut self) {
        for k in 1..=33u8 {
            self.scr.level = k.min(15);
            self.wait_frame();
        }
    }

    /// +CD08 clear_flag_and_poll_loop: 33 frames to black (only when not already black).
    fn fade_out(&mut self) {
        if self.scr.level == 0 {
            return;
        }
        for k in 0..33u8 {
            self.scr.level = 15u8.saturating_sub(k + 1);
            self.wait_frame();
        }
    }

    /// +248A run_callback_grid / +20EC process_level_tilemap: draws screen rows `rows` of a screen file from map
    /// column `col`, into all three pictures.
    fn draw_screen_file(&mut self, van: usize, col: usize, rows: std::ops::Range<usize>) {
        let a = self.a.clone();
        let Ok(sf) = a.screen(van) else {
            return;
        };
        self.scr.resize(sf.w * 8);
        self.scr.scroll = (col * 8) as i32;
        for r in rows {
            for c in 0..40 {
                let mc = col + c;
                let t = sf.tile(mc, r) as usize;
                if let Some(px) = sf.tile_px.get(t) {
                    self.scr.tile((mc * 8) as i32, (r * 8) as i32, px);
                }
            }
        }
    }

    /// +0674 set_title_screen_art: the menu ("Start a New Game", "Continue a Saved Game") with its easing cursor.
    /// Returns 0 for a new game, 1 for a saved one.
    fn title_screen(&mut self) -> i32 {
        self.phase = Phase::Title;
        let t = self.a.title.clone();
        self.fade_out();
        self.draw_screen_file(34, 0, 0..25);
        self.scr.scroll = 0;
        self.set_palette(t.palette);
        let mut y: i16 = 0;
        let mut dir = 0u8;
        self.clear_presses();
        let choice;
        loop {
            loop {
                self.read_controls();
                let d = (self.ctl.up as u8) | ((self.ctl.down as u8) << 1);
                if dir != d && d != 0 {
                    dir = d;
                }
                let mut step = 0i16;
                if dir == 1 {
                    let i = (16 - ((y - t.top) * 15) / 0x40) & 15;
                    step = -t.ease[i as usize];
                }
                if dir == 2 {
                    let i = (16 - ((t.bottom - y) * 15) / 0x40) & 15;
                    step = t.ease[i as usize];
                }
                y += step;
                if y < t.top {
                    y = t.top;
                    dir = 0;
                }
                if y > t.bottom {
                    y = t.bottom;
                    dir = 0;
                }
                self.title_y = y;
                if let Some(s) = self.a.shape("titshape", 0).cloned() {
                    self.scr
                        .shape(&s, t.cursor_x as i32, y as i32, Into::Work, false);
                }
                self.wait_frame();
                if self.ctl.fire || self.input.pressed(Action::Start) {
                    break;
                }
            }
            if y < t.split {
                choice = 0;
                break;
            }
            // +08FA load_castle_data: no saved game in this part of the port.
            self.message("No Game Saved");
        }
        self.clear_presses();
        self.fade_out();
        choice
    }

    /// +C044 handle_game_state: a one-line message in the middle of the play area for 100 frames.
    fn message(&mut self, text: &str) {
        self.scr.ink = 2;
        self.scr.paper = 0;
        let x0 = (W as i32 - 8 * text.len() as i32) / 2;
        for (i, ch) in text.chars().enumerate() {
            let g = self.glyph(ch);
            self.scr.char(&g, x0 + 8 * i as i32, 96, true, false);
        }
        self.wait_frames(100);
    }

    /// Part 1 stand-in for a stage: room `room` as the stage set-up draws it (+0ED2, +20EC), until Escape.
    fn room_view(&mut self, room: usize) {
        self.phase = Phase::Room;
        let r = self.a.rooms[room];
        self.room = room as i16;
        self.scroll_col = r.scroll;
        self.draw_screen_file(r.van as usize, r.scroll.max(0) as usize, 3..25);
        self.set_palette(room);
        loop {
            self.wait_frame();
            self.read_controls();
            if self.input.pressed(Action::Cancel) {
                break;
            }
        }
    }
}

/// Wraps text at word boundaries to `width` columns, dropping the common left margin.
pub fn wrap(lines: &[String], width: usize) -> Vec<String> {
    let mut out = Vec::new();
    for l in lines {
        let words: Vec<&str> = l.split_whitespace().collect();
        if words.is_empty() {
            out.push(String::new());
            continue;
        }
        let mut cur = String::new();
        for w in words {
            if !cur.is_empty() && cur.len() + 1 + w.len() > width {
                out.push(std::mem::take(&mut cur));
            }
            if !cur.is_empty() {
                cur.push(' ');
            }
            cur.push_str(w);
        }
        out.push(cur);
    }
    while out.last().is_some_and(|s| s.is_empty()) {
        out.pop();
    }
    out
}
