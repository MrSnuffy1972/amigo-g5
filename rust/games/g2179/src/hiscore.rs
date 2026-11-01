//! The high-score screen (the all-time table, saved like the original's disk track, and today's table) and the boot
//! program (the Time Warp logo with its colour cycling, the title picture with its music). Each method is named after
//! the recovered routine.

use crate::assets::HsEntry;
use crate::giana::{bcd_text, Disp, Giana};

/// Where `format_highscore_tables` writes into the table text (the original's offsets into its text buffer).
const ROW: usize = 0x23;

impl Giana {
    /// `highscore_screen`.
    pub fn highscore_screen(&mut self) {
        self.build_highscore_copper_list();
        self.start_highscore_tune();
        self.clear_highscore_screen();
        self.load_highscore_track();
        self.scroll_in_highscores();
        self.enter_highscore_name();
        self.insert_highscore();
    }

    /// `build_highscore_copper_list`: each line's sideways wobble and its colour bar; the picture starts 200 lines
    /// down so it scrolls in.
    pub fn build_highscore_copper_list(&mut self) {
        let w = self.a.hs_wobble.clone();
        for y in 0..200 {
            self.hs_shift[y] = w[y % w.len()];
        }
        self.hs_row = 200;
    }

    /// `clear_highscore_screen`.
    pub fn clear_highscore_screen(&mut self) {
        self.dma_display = false;
        self.dma_sprites = false;
        Giana::clear_text_picture(&mut self.hs);
        // 8400 passes of four word stores and a loop step (about 353,000 processor cycles by count); measured
        // against the reference run's name-entry blink, the screen change costs one frame more.
        self.busy(777 + 313);
        self.col[12] = 0;
        self.disp = Disp::HighScore;
        self.dma_display = true;
    }

    /// `load_highscore_track`: the all-time table, once a session (the saved one, or the one on the original disk).
    pub fn load_highscore_track(&mut self) {
        if !self.hs_loaded {
            self.hs_loaded = true;
            if let Some(s) = self.saved_hs.clone() {
                self.hs_all = s;
            }
        }
        self.format_highscore_tables();
    }

    /// `format_highscore_tables`: names, scores and stages into the table text, which is printed.
    pub fn format_highscore_tables(&mut self) {
        let mut t = self.a.texts.hs_tables.clone();
        let put = |t: &mut Vec<u8>, at: usize, b: &[u8]| {
            for (i, &c) in b.iter().enumerate() {
                if let Some(x) = t.get_mut(at + i) {
                    *x = c;
                }
            }
        };
        for (first, table) in [(1usize, self.hs_all.clone()), (7, self.hs_today.clone())] {
            for (k, e) in table.iter().enumerate().take(5) {
                let r = (first + k) * ROW;
                put(&mut t, r + 18, &e.name);
                let sc = bcd_text(&[(e.score >> 16) as u8, (e.score >> 8) as u8, e.score as u8]);
                put(&mut t, r + 11, &sc);
                let st = bcd_text(&[e.stage as u8]);
                put(&mut t, r + 30, &st);
            }
        }
        self.wait_line(0xC8);
        self.cursor = (0, 0);
        self.print_text(2, &t);
        self.hs_count = 0xC8;
    }

    /// `scroll_in_highscores`: the tables scroll up into view, then the wobble settles.
    pub fn scroll_in_highscores(&mut self) {
        loop {
            self.hs_row = self.hs_count;
            self.wait_line(0xC8);
            self.hs_count -= 1;
            if self.hs_count < 0 {
                break;
            }
        }
        for _ in 0..15 {
            self.wait_line(0xC8);
            for s in self.hs_shift.iter_mut() {
                if *s != 0 {
                    *s -= 0x11;
                }
            }
        }
        // The counter word ends at -1; the name entry's blink counts in its upper byte.
        self.hs_blink = 0xFF;
    }

    fn hs_print(&mut self, x: i32, t: &[u8]) {
        self.cursor = (x, 0xC0);
        self.print_text(2, t);
    }

    /// `enter_highscore_name`: if the score makes either table, the name is chosen letter by letter with the
    /// joystick, the letter being chosen blinking.
    pub fn enter_highscore_name(&mut self) {
        if self.score < self.hs_all[4].score && self.score < self.hs_today[4].score {
            return;
        }
        self.hs_cursor = 0;
        self.wait_line(0xC8);
        let t = self.a.texts.enter_name.clone();
        self.hs_print(0, &t);
        self.hs_touched = [false; 10];
        self.hs_name = [b' '; 10];
        loop {
            let i = self.hs_cursor;
            let saved = self.hs_name[i];
            for _ in 0..6 {
                self.wait_line(0xC8);
                self.hs_blink = self.hs_blink.wrapping_add(1);
                self.hs_name[i] = if self.hs_blink & 4 != 0 { b' ' } else { saved };
                let n = self.hs_name;
                self.hs_print(0x17, &n);
            }
            self.hs_name[i] = saved;
            self.read_joystick();
            if self.left && self.hs_cursor > 0 {
                self.hs_cursor -= 1;
            }
            if self.right && self.hs_cursor != 9 {
                self.hs_cursor += 1;
            }
            let i = self.hs_cursor;
            let mut c = self.hs_name[i];
            if !self.hs_touched[i] {
                c = b'A';
                self.hs_touched[i] = true;
            }
            if self.up && (c as i8) < b'Z' as i8 {
                c += 1;
            }
            if self.down && c != b' ' {
                c = c.wrapping_sub(1);
            }
            self.hs_name[i] = c;
            let n = self.hs_name;
            self.hs_print(0x17, &n);
            if self.fire {
                break;
            }
        }
    }

    fn insert(table: &mut [HsEntry], e: HsEntry) {
        let old = table.to_vec();
        let mut done = false;
        for k in 0..5 {
            table[k] = if done {
                old[k - 1].clone()
            } else if e.score >= old[k].score {
                done = true;
                e.clone()
            } else {
                old[k].clone()
            };
        }
    }

    /// `insert_highscore`: the score into both tables. The original then waited for the mouse button to write the
    /// all-time table back to its disk (fire went on without saving); the WHDLoad install this port is made from
    /// replaced that wait with an immediate save (none once its trainer key had been used), and so does the port.
    pub fn insert_highscore(&mut self) {
        let e = HsEntry {
            name: self.hs_name,
            score: self.score,
            stage: self.stage_bcd as u32,
        };
        if self.score >= self.hs_all[4].score {
            Self::insert(&mut self.hs_all, e.clone());
        }
        if self.score >= self.hs_today[4].score {
            Self::insert(&mut self.hs_today, e);
            self.format_highscore_tables();
        }
        self.wait_line(0xC8);
        let t = self.a.texts.save_hint.clone();
        self.hs_print(0, &t);
        if !self.trainer_used {
            self.save_out = Some(self.hs_all.clone());
        }
    }

    // ------------------------------------------------------------------ the boot program

    /// `boot_main` (entered at `boot_start`): the logo and the title picture, then the game.
    pub fn boot_main(&mut self) {
        self.col[0] = 0;
        self.boot_music_on = true;
        self.show_logo();
        self.show_title_picture();
        // Loading the game program (nine tracks).
        self.busy(313 * 2);
        self.boot_music_on = false;
        self.machine_setup();
    }

    /// `machine_setup`: the buffers, the game data (40 tracks) and the vertical-blank handler are ready.
    pub fn machine_setup(&mut self) {
        self.busy(313 * 2);
    }

    /// `show_logo` and `logo_loop`: the Time Warp logo, its colours cycling until fire.
    pub fn show_logo(&mut self) {
        // `unpack_iff_picture`.
        self.busy(313 * 8);
        self.load_picture_palette(true);
        self.disp = Disp::Logo;
        self.dma_display = true;
        loop {
            self.wait_line(0xC8);
            self.cycle_logo_colours();
            self.read_joystick();
            if self.fire {
                break;
            }
        }
        // `boot_display_off`.
        self.dma_display = false;
    }

    /// `load_picture_palette`: the picture's colour map into the colours (and the logo's cycling table).
    pub fn load_picture_palette(&mut self, logo: bool) {
        let c = if logo {
            self.a.logo.colours.clone()
        } else {
            self.a.title_picture.colours.clone()
        };
        for (i, v) in c.iter().enumerate().take(32) {
            self.col[i] = *v;
        }
        if logo {
            let mut t = [0u16; 16];
            for (k, x) in t.iter_mut().enumerate() {
                *x = c.get(16 + k).copied().unwrap_or(0);
            }
            self.logo_cycle = t;
        }
    }

    /// `cycle_logo_colours`: every second frame the logo's upper colours turn by one.
    pub fn cycle_logo_colours(&mut self) {
        self.logo_cycle_flip = !self.logo_cycle_flip;
        if !self.logo_cycle_flip {
            return;
        }
        self.logo_cycle.rotate_left(1);
        for k in 0..16 {
            self.col[16 + k] = self.logo_cycle[k];
        }
    }

    /// `show_title_picture` and `title_picture_loop`: the title picture with its music until fire.
    pub fn show_title_picture(&mut self) {
        // `unpack_iff_picture`: the 320 by 400 picture in six planes.
        self.busy(313 * 20);
        self.load_picture_palette(false);
        self.disp = Disp::TitlePicture;
        self.wait_line(-0x2C);
        self.dma_display = true;
        let a = self.a.clone();
        self.boot_music
            .start(&a.boot_music, &mut self.snd, 1, 2, false);
        loop {
            self.wait_line(0xC8);
            self.boot_music.boot_tick(&a.boot_music, &mut self.snd);
            self.read_joystick();
            if self.fire {
                break;
            }
        }
        // `boot_stop_music` (`boot_silence_channels`, `boot_silence_one_channel`).
        for c in 0..4 {
            self.snd.volume(c, 0);
            self.snd.off(c);
        }
        self.dma_display = false;
    }
}
