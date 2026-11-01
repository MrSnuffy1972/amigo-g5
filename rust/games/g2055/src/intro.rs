//! The front end of Speedball 2, written from the recovered intro module (the routines from
//! `set_game_status_flags` to `zero_fill_memory_block`, plus the palette fades and the screen swap they call). The
//! routine names are the recovery's automatic names (several describe the routine badly; the comments say what each
//! really does), so the function map reads straight across.
//!
//! Timing is the original's: a frame wait sets the low byte of the frame counter and waits for it to wrap to zero
//! (5 frames between the steps of the typing, 2 between fade steps, 255 for the title), and fire skips only the
//! waits of the intro proper (not the opening pages, not the fades).

use std::sync::{Arc, Mutex, MutexGuard};

use native_engine::game::{Ctx, Flow, Game};
use native_engine::gfx::{rgb12, Bitmap, Frame, Rect};
use native_engine::input::Action;

use crate::assets::{Assets, Page};
use crate::co::{Co, Link};
use crate::world::{Stage, World};

/// The cursor glyph and the save slot the typing uses (the original keeps the background under the cursor in glyph
/// slot 43 of the font).
const CURSOR: usize = 41;
/// Sound effects the front end asks for.
const SFX_TYPE: u8 = 54;
const SFX_BLINK: u8 = 55;
const SFX_WHOOSH: u8 = 44;
const SFX_AGAIN: u8 = 45;

/// The game as the engine runs it: the program thread and the state it shares with `render`.
pub struct Speedball {
    world: Arc<Mutex<World>>,
    co: Co,
}

impl Speedball {
    pub fn new(a: Assets) -> Speedball {
        let world = Arc::new(Mutex::new(World::default()));
        let w = world.clone();
        let co = Co::start(move |l| {
            Intro {
                w,
                a,
                l,
                saved: Bitmap::new(16, 16),
            }
            .run()
        });
        Speedball { world, co }
    }

    fn world(&self) -> MutexGuard<'_, World> {
        self.world.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Game for Speedball {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        {
            let i = cx.input;
            let mut w = self.world();
            w.vertical_blank();
            w.fire = [Action::Fire, Action::Start]
                .iter()
                .any(|&a| i.held(a) || i.pressed(a));
        }
        self.co.tick();
        Flow::Continue
    }

    fn render(&self, f: &mut Frame) {
        let w = self.world();
        let s = &w.screens[w.display];
        if f.image.px.len() == s.px.len() {
            f.image.px.copy_from_slice(&s.px);
        }
        for (i, c) in w.colours.iter().enumerate() {
            f.palette.colours[i] = rgb12(*c);
        }
    }

    fn state(&self) -> Vec<(String, i64)> {
        let w = self.world();
        let pal: i64 = w.colours.iter().map(|&c| c as i64).sum();
        vec![
            ("stage".into(), w.stage as i64),
            ("page".into(), w.page as i64),
            ("cursor_x".into(), w.cursor.0 as i64),
            ("cursor_y".into(), w.cursor.1 as i64),
            ("sound".into(), w.sound as i64),
            ("intro".into(), w.intro as i64),
            ("colour_sum".into(), pal),
            ("sfx_last".into(), w.sfx_last as i64),
            ("sfx_count".into(), w.sfx_count as i64),
        ]
    }
}

/// The front-end program.
struct Intro<'l> {
    w: Arc<Mutex<World>>,
    a: Assets,
    l: &'l mut Link,
    /// The background under the cursor.
    saved: Bitmap,
}

impl Intro<'_> {
    fn w(&self) -> MutexGuard<'_, World> {
        self.w.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn frame(&mut self) {
        self.l.frame();
    }

    fn run(mut self) {
        self.set_game_status_flags();
        self.load_and_apply_palette_data();
        self.w().stage = Stage::GameStart;
        loop {
            self.frame();
        }
    }

    // ---- waits ----

    /// The intro's frame wait (5 frames from when it was last set). It returns at once when fire is pressed during
    /// the intro proper (sound off, intro on); the typing and blinking loops then end at their next check.
    fn check_game_state_flags(&mut self) {
        loop {
            {
                let mut w = self.w();
                if !w.sound && w.intro && w.fire {
                    return;
                }
                if w.countdown() == 0 {
                    w.set_countdown(0xFB);
                    return;
                }
            }
            self.frame();
        }
    }

    /// The fades' frame wait (2 frames), not skippable.
    fn set_player_dead_flag(&mut self) {
        while self.w().countdown() != 0 {
            self.frame();
        }
        self.w().set_countdown(0xFE);
    }

    /// Waits for the next frame (the original compares its second frame counter).
    fn wait_next_frame(&mut self) {
        self.frame();
    }

    fn skipping(&self) -> bool {
        let w = self.w();
        !w.sound && w.intro && w.fire
    }

    // ---- the intro module ----

    /// Start-up: sound and intro on, then the two opening pages.
    fn set_game_status_flags(&mut self) {
        {
            let mut w = self.w();
            w.sound = true;
            w.intro = true;
            w.stage = Stage::Opening;
        }
        self.parse_game_status_block();
    }

    /// Decodes the arena (only its colours are used: the screen is cleared) and types the two opening pages.
    fn parse_game_status_block(&mut self) {
        let s = self.w().back();
        self.select_and_parse_block(false, s);
        for i in 0..2 {
            self.w().page = i;
            let p = self.a.start_pages[i].clone();
            self.load_game_status_block(&p);
        }
    }

    /// An opening page: clear the drawing screen, show it (fading to the picture's colours), type the page with
    /// sound, then blink the cursor 13 times.
    fn load_game_status_block(&mut self, page: &Page) {
        let s = self.w().back();
        self.zero_fill_memory_block(s);
        self.wait_and_update_palette(false);
        let at = self.parse_player_string_table(page, s);
        self.blink(s, at, 13, true);
    }

    /// The intro proper, repeated until fire: the title, the story, the title again, the credits; then LOADING GAME.
    fn load_and_apply_palette_data(&mut self) {
        loop {
            {
                let mut w = self.w();
                w.sound = false;
                w.stage = Stage::Title;
            }
            let s = self.w().back();
            self.select_and_parse_block(true, s);
            self.wait_and_update_palette(true);
            self.w().set_countdown(1);
            self.check_game_state_flags();
            self.initialize_team_data_tables();
            self.w().stage = Stage::Title;
            let s = self.w().back();
            self.select_and_parse_block(true, s);
            self.wait_and_update_palette(false);
            self.w().set_countdown(1);
            self.check_game_state_flags();
            self.initialize_string_table_entries();
            if self.w().fire {
                break;
            }
            self.w().sfx(SFX_AGAIN);
        }
        // the original spins until fire is let go
        while self.w().fire {
            self.frame();
        }
        let s = self.load_and_parse_disk_block();
        {
            let mut w = self.w();
            w.intro = false;
            w.stage = Stage::Loading;
            w.page = 0;
        }
        let p = self.a.loading_page.clone();
        self.parse_player_string_table(&p, s);
        self.copy_to_drawing_screen();
    }

    /// The nine story pages (named by the recovery for the tables it thought they filled).
    fn initialize_team_data_tables(&mut self) {
        self.w().stage = Stage::Story;
        for i in 0..self.a.story_pages.len() {
            self.w().page = i;
            let p = self.a.story_pages[i].clone();
            self.initialize_team_statistics(&p, 17);
        }
    }

    /// The eight credit pages.
    fn initialize_string_table_entries(&mut self) {
        self.w().stage = Stage::Credits;
        for i in 0..self.a.credit_pages.len() {
            self.w().page = i;
            let p = self.a.credit_pages[i].clone();
            self.initialize_team_statistics(&p, 16);
        }
    }

    /// A story or credit page over the arena (`initialize_player_string_table` is the same with 16 blinks):
    /// nothing when fire is held; otherwise show the arena, type the page and blink the cursor.
    fn initialize_team_statistics(&mut self, page: &Page, blinks: usize) {
        if self.w().fire {
            return;
        }
        let s = self.load_and_parse_disk_block();
        let at = self.parse_player_string_table(page, s);
        self.blink(s, at, blinks, false);
    }

    /// Decodes the arena into the drawing screen and shows it; returns that screen (the pages are typed on it).
    fn load_and_parse_disk_block(&mut self) -> usize {
        let s = self.w().back();
        self.select_and_parse_block(false, s);
        self.wait_and_update_palette(false);
        s
    }

    /// Types a page on screen `s`: each glyph shows the cursor for 5 frames first; a position byte moves the
    /// typing. Returns where the typing stopped (the cursor blinks there). With sound on each glyph clicks.
    fn parse_player_string_table(&mut self, page: &Page, s: usize) -> (u16, u16) {
        self.w().set_countdown(0xFB);
        let (mut x, mut y) = (0u16, 0u16);
        let mut i = 0;
        while i < page.len() && !self.skipping() {
            let c = page[i];
            if c <= 0x40 {
                x = c as u16;
                i += 1;
                y = page.get(i).copied().unwrap_or(0) as u16;
            } else {
                self.w().cursor = (x, y);
                self.copy_player_stat_blocks(s, x, y);
                self.check_game_state_flags();
                self.process_player_collision_data(s, x, y, CURSOR);
                self.check_game_state_flags();
                self.scatter_load_words(s, x, y);
                if self.w().sound {
                    self.w().sfx(SFX_TYPE);
                }
                self.process_player_collision_data(s, x, y, (c - 0x41) as usize);
                x += 2;
                if x >= 0x28 {
                    x = 2;
                    y += 5;
                }
            }
            i += 1;
        }
        self.w().cursor = (x, y);
        (x, y)
    }

    /// The cursor blinking where the typing stopped (`n` times; with a click each time on the opening pages).
    fn blink(&mut self, s: usize, (x, y): (u16, u16), n: usize, click: bool) {
        self.copy_player_stat_blocks(s, x, y);
        for _ in 0..n {
            if self.skipping() {
                return;
            }
            self.check_game_state_flags();
            self.process_player_collision_data(s, x, y, CURSOR);
            self.check_game_state_flags();
            self.scatter_load_words(s, x, y);
            if click {
                self.w().sfx(SFX_BLINK);
            }
        }
    }

    // ---- drawing (named `call_indirect_with_offset` + helper in the recovery) ----

    fn cell(x: u16, y: u16) -> (i32, i32) {
        (((x & !1) * 8) as i32, (y * 4) as i32)
    }

    /// Draws glyph `g` at a typing position, colour 0 showing the background (the original's masked draw).
    fn process_player_collision_data(&mut self, s: usize, x: u16, y: u16, g: usize) {
        let (px, py) = Self::cell(x, y);
        let r = self.a.font.frame(g);
        let mut w = self.w();
        let clip = w.screens[s].rect();
        w.screens[s].blit_keyed(&self.a.font.image, 0, r, px, py, clip);
    }

    /// Saves the 16 x 16 background at a typing position.
    fn copy_player_stat_blocks(&mut self, s: usize, x: u16, y: u16) {
        let (px, py) = Self::cell(x, y);
        let w = self.w();
        let clip = self.saved.rect();
        let src = &w.screens[s];
        let mut b = Bitmap::new(16, 16);
        b.blit(
            src,
            Rect {
                x: px,
                y: py,
                w: 16,
                h: 16,
            },
            0,
            0,
            clip,
        );
        drop(w);
        self.saved = b;
    }

    /// Puts the saved background back.
    fn scatter_load_words(&mut self, s: usize, x: u16, y: u16) {
        let (px, py) = Self::cell(x, y);
        let mut w = self.w();
        let clip = w.screens[s].rect();
        w.screens[s].blit(&self.saved, self.saved.rect(), px, py, clip);
    }

    fn zero_fill_memory_block(&mut self, s: usize) {
        self.w().screens[s].fill(0);
    }

    /// Decodes a picture (the title or the arena) into screen `s` and keeps its colours as the picture palette. The
    /// decoding itself was done when the asset pack was built.
    fn select_and_parse_block(&mut self, title: bool, s: usize) {
        let (b, p) = if title {
            (&self.a.title, self.a.title_pal)
        } else {
            (&self.a.arena, self.a.arena_pal)
        };
        let mut w = self.w();
        w.screens[s].px.copy_from_slice(&b.px);
        w.picture = p;
    }

    /// After LOADING GAME the original copies the first 32000 bytes of the shown screen's planes to the drawing
    /// screen: planes 0 to 2 whole, plane 3 down to row 134, plane 4 not at all.
    fn copy_to_drawing_screen(&mut self) {
        let mut w = self.w();
        let (sh, bk) = (w.shown, w.back());
        let src = w.screens[sh].px.clone();
        let dst = &mut w.screens[bk].px;
        for (i, (d, s)) in dst.iter_mut().zip(src).enumerate() {
            let row = i / crate::world::SCREEN_W;
            let mask: u8 = if row < 134 { 0x0F } else { 0x07 };
            *d = (*d & !mask) | (s & mask);
        }
    }

    // ---- colours and the screen swap ----

    /// Shows the drawn screen with the picture's colours: if the colour registers already hold them, only swap;
    /// otherwise fade to black, swap, and fade up to the picture's colours. `whoosh` is the title's version
    /// (`verify_and_apply_palette`), which also plays the whoosh between the fades.
    fn wait_and_update_palette(&mut self, whoosh: bool) {
        self.wait_next_frame();
        let same = {
            let w = self.w();
            w.colours == w.picture
        };
        if same {
            self.w().swap_screens();
            return;
        }
        self.update_palette_colors();
        self.w().swap_screens();
        self.wait_next_frame();
        if whoosh {
            self.w().sfx(SFX_WHOOSH);
        }
        self.process_player_color_palette();
    }

    /// Fade to black: 15 steps, each taking one off every non-zero red, green and blue.
    fn update_palette_colors(&mut self) {
        self.w().set_countdown(0xFE);
        for _ in 0..15 {
            {
                let mut w = self.w();
                for c in w.colours.iter_mut() {
                    let old = *c;
                    *c = [0x100u16, 0x010, 0x001]
                        .iter()
                        .fold(0, |v, &u| v | (((old / u) & 15).saturating_sub(1) * u));
                }
            }
            self.set_player_dead_flag();
        }
    }

    /// Fade up: 15 steps, each component at its target less the steps still to go.
    fn process_player_color_palette(&mut self) {
        for k in (0..15u16).rev() {
            {
                let mut w = self.w();
                let t = w.picture;
                for (c, t) in w.colours.iter_mut().zip(t) {
                    *c = [0x100u16, 0x010, 0x001]
                        .iter()
                        .fold(0, |v, &u| v | (((t / u) & 15).saturating_sub(k) * u));
                }
            }
            self.set_player_dead_flag();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_typing_position_is_eight_pixels_a_column_byte_and_four_lines_a_row() {
        assert_eq!(Intro::cell(10, 20), (80, 80));
        assert_eq!(Intro::cell(3, 5), (16, 20));
    }
}
