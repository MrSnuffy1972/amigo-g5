//! The main program of Castlevania (Konami 1990, Amiga conversion by Novotrade), rewritten from the recovered code.
//! Each routine keeps the original's name (from the recovered function list) in its doc comment, with its offset in
//! the program (`+XXXX`), so the function map reads straight across.
//!
//! Port status (part 2): the crack's boot text page, the title menu, and the stage engine with Simon and the
//! panel (in `stage`). Enemies, bosses, sub-weapons, items and the sound follow in part 3 (see
//! docs/native/g2271-native.md).

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::input::{Action, InputState};

use crate::assets::{Assets, ScreenFile, H, W};
use crate::co::{FrameIn, FrameOut, Stop};
use crate::objects::{Snd, World};
use crate::screen::{Into, Screen};
use crate::sound::Sound;
use crate::stage::{Sfx, Simon};

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
    Stage,
    /// The high-score table (+9CA6).
    Hall,
}

pub struct Castle {
    pub(crate) a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    pub scr: Screen,
    pub(crate) input: InputState,
    /// The player and the stage (part 2).
    pub st: Simon,
    /// The panel's numbers as last drawn (score, time, hearts, lives, stage).
    pub(crate) panel_cache: Option<(i32, i16, i16, i16, i16)>,
    pub ctl: Controls,
    /// -$2674: reads the fire button has been held.
    pub fire_count: i16,
    pub frame: u32,
    pub phase: Phase,
    /// Title menu cursor line.
    pub title_y: i16,
    /// Room number (-$19B0) and its first map column on screen (-$19AE).
    pub room: i16,
    pub scroll_col: i16,
    #[allow(dead_code)]
    seed: Option<u32>,
    /// The objects (part 3) and the room's screen file as the player reads it (platforms change it).
    pub w: World,
    pub sf: ScreenFile,
    /// Columns the last scroll moved (-$199C), and the sub-weapon box as last drawn.
    pub(crate) scan_n: i16,
    pub(crate) sub_cache: Option<(i16, i16)>,
    /// The sound: the replayer, the effect slots and the commands for the mixer (part 3b).
    pub snd: Sound,
    /// The high-score table (from the save store) and the table to save at the next frame.
    pub scores: Vec<(i32, String)>,
    pub(crate) save_out: Option<serde_json::Value>,
}

impl Castle {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Castle {
        let snd = Sound::new(&a);
        Castle {
            rx,
            tx,
            scr: Screen::new(W),
            input: InputState::default(),
            st: Simon::default(),
            panel_cache: None,
            ctl: Controls::default(),
            fire_count: 0,
            frame: 0,
            phase: Phase::Intro,
            title_y: 0,
            room: 0,
            scroll_col: 0,
            seed,
            w: {
                let mut w = World::new(&a.dat);
                // +B798 seed_random seeds from the CIA timers at start; a fixed seed makes runs repeatable
                w.rng = seed.unwrap_or(0x2271);
                w
            },
            a,
            sf: ScreenFile::default(),
            scan_n: 1,
            sub_cache: None,
            snd,
            scores: crate::hall::scores_from(None),
            save_out: None,
        }
    }

    /// The player's sound requests, as the original's numbers (+AC24), in the order they were made.
    pub fn forward_sfx(&mut self) {
        let sfx: Vec<Sfx> = std::mem::take(&mut self.st.sfx);
        for s in sfx {
            // +2D98 landing (6, no sample), +1502 the time running out (1), +3EEE and +39FA falling and dying (7)
            let n = match s {
                Sfx::Land => 6,
                Sfx::TimeWarning => 1,
                Sfx::Fall | Sfx::Death => 7,
                Sfx::Hurt => continue,
            };
            self.w.fx(n);
        }
    }

    /// The frame's sound requests: effects at once (+AC24), tunes through +ACB0 (which can wait for the music to
    /// fade out first).
    pub fn flush_sounds(&mut self) {
        self.forward_sfx();
        let snd: Vec<Snd> = std::mem::take(&mut self.w.snd);
        for s in snd {
            match s {
                Snd::Fx(n) => self.snd.effect(n),
                Snd::Tune(n) => self.music(n),
            }
        }
    }

    /// +ACB0 handle_music_event: fade the music out at 4 a step, wait (the main program spins; the picture stays)
    /// until it has stopped, then tune `n` (1000, the title module, starts at once without the wait).
    pub fn music(&mut self, n: i16) {
        self.snd.fade_out(4);
        let n = if n > 12 && n != 1000 { 12 } else { n };
        if n == 1000 {
            self.snd.play_title();
            return;
        }
        while !self.snd.check_ended() {
            self.hold_frame();
        }
        self.snd.play_tune(n);
    }

    /// One vertical blank's sound: the requests made, the interrupts, and the commands for the mixer.
    fn sound_vblank(&mut self) -> Vec<crate::sound::Mix> {
        self.flush_sounds();
        self.snd.vblank();
        std::mem::take(&mut self.snd.out)
    }

    pub fn state(&self) -> Vec<(String, i64)> {
        let mut v = self.state_main();
        v.extend(self.snd.state());
        v
    }

    fn state_main(&self) -> Vec<(String, i64)> {
        vec![
            ("frame".into(), self.frame as i64),
            ("phase".into(), self.phase as i64),
            ("title_y".into(), self.title_y as i64),
            ("room".into(), self.room as i64),
            ("scroll_col".into(), self.scroll_col as i64),
            ("map_col".into(), self.st.scroll_col as i64),
            ("x".into(), self.st.x as i64),
            ("y".into(), self.st.y as i64),
            ("anim".into(), self.st.anim as i64),
            ("anim_frame".into(), self.st.frame as i64),
            ("mv".into(), self.st.mv as i64),
            ("facing".into(), self.st.facing as i64),
            ("hp".into(), self.st.hp as i64),
            ("lives".into(), self.st.lives as i64),
            ("time".into(), self.st.time as i64),
            ("stage".into(), self.st.stage as i64),
            ("game_room".into(), self.st.room as i64),
            ("dying".into(), self.st.dying as i64),
            ("jump".into(), self.st.jump as i64),
            ("whip".into(), self.st.whip as i64),
            ("crouch".into(), self.st.crouch as i64),
            ("falling".into(), self.st.falling as i64),
            ("score".into(), self.st.score as i64),
            ("hearts".into(), self.st.hearts as i64),
            ("sub".into(), self.st.sub as i64),
            ("whip_level".into(), self.st.whip_level as i64),
            ("hurt".into(), self.st.hurt as i64),
            ("block".into(), self.st.block as i64),
            ("objects".into(), self.w.m.w(crate::objects::G_LIVE) as i64),
            ("boss_hp".into(), self.w.boss_hp as i64),
            ("enemy_bar".into(), self.st.enemy_hp as i64),
            ("game_over".into(), self.st.game_over as i64),
            ("stage_clear".into(), self.st.stage_clear as i64),
            ("rng".into(), self.w.rng as i64),
        ]
    }

    /// One more vertical blank of the frame already on screen (no flip): the stage's frames last four.
    pub fn hold_frame(&mut self) {
        let mix = self.sound_vblank();
        let (screen, palette) = self.scr.view();
        let out = FrameOut {
            save: self.save_out.take(),
            mix,
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
    }

    /// The frame wait (+CBB8 wait_for_vblank_and_clear_flags): shows the picture drawn, takes the next frame's
    /// input, and restores the boxes of the shapes drawn into the next picture.
    pub fn wait_frame(&mut self) {
        let mix = self.sound_vblank();
        self.scr.flip();
        let (screen, palette) = self.scr.view();
        let out = FrameOut {
            save: self.save_out.take(),
            mix,
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

    /// +15AA read_player_controls with +D14E read_joystick_and_update_state: the joystick's directions (up is
    /// also the jump bit, 0x21), and the one fire button counted over the reads (-$2674): a tap fires on its
    /// release (the whip), a hold fires with up on its fourth read (the sub-weapon). The second button (jump key
    /// or pad B) is a second way to push the stick up. Edges as +15AA keeps them (-$1920 fire, -$191E jump).
    pub fn read_controls(&mut self) {
        let i = &self.input;
        let hold = |a: Action| i.held(a) || i.pressed(a);
        let (up, down) = (hold(Action::Up) || hold(Action::Fire2), hold(Action::Down));
        let mut bits = 0u16;
        if hold(Action::Right) {
            bits |= 4;
        } else if hold(Action::Left) {
            bits |= 8;
        }
        if up {
            bits |= 0x21;
        } else if down {
            bits |= 2;
        }
        if hold(Action::Fire) {
            self.fire_count += 1;
            if self.fire_count == 4 {
                bits |= 0x11;
            }
        } else if self.fire_count != 0 {
            if self.fire_count <= 4 {
                bits |= 0x10;
                bits &= !1;
            }
            self.fire_count = 0;
        }
        let mut c = Controls {
            up: bits & 1 != 0,
            down: bits & 2 != 0,
            right: bits & 4 != 0,
            left: bits & 8 != 0,
            fire_held: bits & 0x10 != 0,
            fire2_held: bits & 0x20 != 0,
            ..Controls::default()
        };
        c.centre = !(c.up || c.down || c.left || c.right);
        c.fire = c.fire_held && !self.ctl.fire_held;
        c.fire2 = c.fire2_held && !self.ctl.fire2_held;
        self.ctl = c;
    }

    /// +0830 clear_state_flags_2: reads the controls once and forgets the presses.
    pub(crate) fn clear_presses(&mut self) {
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
        // +0006: the title module (tune 1000) and the villain's laugh (+A96E 0) for the opening; then the module
        // fades at 2 a step and the rest of the sound loads once it has stopped (+A96E 1)
        self.music(1000);
        self.snd.load_sounds(0);
        self.snd.fade_out(2);
        while !self.snd.check_ended() {
            self.hold_frame();
        }
        self.snd.load_sounds(1);
        loop {
            // l0_0076: the title's tune
            self.music(3);
            let choice = self.title_screen();
            if choice == 0 {
                self.game();
                // +015E: unless the game was quit, the high-score table
                if self.st.quit == 0 {
                    self.hall_of_blame();
                }
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

    pub(crate) fn glyph(&self, ch: char) -> [u8; 64] {
        let i = (ch as u32).wrapping_sub(0x20) as usize;
        self.a.font.get(i).copied().unwrap_or([0; 64])
    }

    /// +B5BC copy_handler_table_entries: the sprite colours and the room's colours, then a fade in (+CCC6).
    pub(crate) fn set_palette(&mut self, n: usize) {
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
    pub(crate) fn fade_out(&mut self) {
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
    pub(crate) fn draw_screen_file(
        &mut self,
        van: usize,
        col: usize,
        rows: std::ops::Range<usize>,
    ) {
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
