//! The menus of Speedball 2 (part 2 of the port, first slice): the menu screen and the menu runner the whole game
//! uses, the first menu (1 PLAYER GAME, 2 PLAYER GAME, DEMO GAME, REPLAY GOALS, with the demo after 1500 idle frames),
//! the match menu (KNOCKOUT, LEAGUE, CUP, PRACTICE) and the league's and the cup's game menus. Written from the
//! recovered routines `initialize_post_match_screen`, `process_player_selection_menu`, `process_entity_list`,
//! `parse_player_name_string`, `parse_and_project_entities` and their helpers; the names are the recovery's automatic
//! ones (several describe the routine badly; the comments say what each does).
//!
//! What a selection leads to (team selection, the team manager, the matches, loading a saved game) is a later part
//! of the port: the selection is recorded in `World::next` and the game goes back to the first menu, as the original
//! does when that part returns.

use native_engine::gfx::{Bitmap, Sheet};

use super::Intro;
use crate::assets::Menu;
use crate::world::{Next, Stage};

/// The menu font's two arrows (left, right).
const ARROW_L: usize = 36;
const ARROW_R: usize = 37;
/// Sound effects: the arrows move, an item is chosen.
const SFX_MOVE: u8 = 20;
const SFX_PICK: u8 = 22;
/// The key code of Esc (the keyboard handler's byte).
pub(crate) const KEY_ESC: u8 = 0x45;
/// Joystick events (the interrupt's event byte): up, down, fire.
pub(crate) const JOY_UP: u8 = 1;
pub(crate) const JOY_DOWN: u8 = 2;
pub(crate) const JOY_FIRE: u8 = 0x80;
/// Idle frames before the first menu starts the demo game.
const DEMO_WAIT: u16 = 1500;

/// Which menu is on screen (the `menu` the verification reads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuId {
    Players = 1,
    Matches = 2,
    League = 3,
    /// (4 and 5 are the series and disk-error menus, which come with the parts that use them.)
    Cup = 6,
}

impl Intro<'_> {
    /// The main program's loop (`l_008050`): the first menu, again and again.
    pub(super) fn main_loop(&mut self) {
        self.w().stage = Stage::Menu;
        loop {
            self.initialize_post_match_screen();
        }
    }

    /// The first menu. Fire chooses; with no joystick event for 1500 frames the demo game starts.
    fn initialize_post_match_screen(&mut self) {
        {
            let mut w = self.w();
            w.escape = false;
            w.choice = 0;
            w.menu = MenuId::Players as u8;
        }
        let backdrop = self.menu_backdrop(MenuMenu::Players);
        let last = self.a.menus.players.len() as u16 - 1;
        loop {
            self.show_menu(&backdrop, MenuMenu::Players);
            {
                let mut w = self.w();
                w.joy = 0;
                w.idle = 0;
            }
            // the original calls its random number generator on every pass of this wait (see the function map)
            let joy = loop {
                let (joy, idle) = {
                    let w = self.w();
                    (w.joy, w.idle)
                };
                if idle == DEMO_WAIT {
                    self.pick(Next::Demo);
                    return;
                }
                if joy != 0 {
                    break joy;
                }
                self.frame();
            };
            let mut w = self.w();
            if joy & JOY_UP != 0 {
                w.choice = if w.choice == 0 { last } else { w.choice - 1 };
            } else if joy & JOY_DOWN != 0 {
                w.choice = if w.choice == last { 0 } else { w.choice + 1 };
            } else if joy & JOY_FIRE != 0 {
                w.sfx(SFX_PICK);
                let c = w.choice;
                drop(w);
                match c {
                    0 => self.toggle_display_mode_flags(),
                    1 => self.pick(Next::TwoPlayers),
                    2 => self.pick(Next::Demo),
                    _ => self.pick(Next::ReplayGoals),
                }
                return;
            } else {
                continue;
            }
            w.sfx(SFX_MOVE);
        }
    }

    /// The match menu, then the chosen competition's own start.
    fn toggle_display_mode_flags(&mut self) {
        let c = self.process_player_selection_menu(MenuMenu::Matches);
        if self.w().escape {
            return;
        }
        match c {
            0 => self.pick(Next::Knockout),
            1 => self.enter_league_menu_mode(),
            2 => self.initialize_league_menu_state(),
            _ => self.toggle_display_flags(),
        }
    }

    /// The league: mode 1, 550 for each team's purse, then NEW GAME, TEAM MANAGER or LOAD GAME.
    fn enter_league_menu_mode(&mut self) {
        {
            let mut w = self.w();
            w.mode = 1;
            w.money = [550; 3];
        }
        let c = self.process_player_selection_menu(MenuMenu::League);
        if self.w().escape {
            return;
        }
        self.pick(match c {
            0 => Next::LeagueNew,
            1 => Next::LeagueManager,
            _ => Next::LeagueLoad,
        });
    }

    /// The cup: mode 2, 2000 for each purse, then NEW GAME or LOAD GAME.
    fn initialize_league_menu_state(&mut self) {
        {
            let mut w = self.w();
            w.mode = 2;
            w.money = [2000; 3];
        }
        let c = self.process_player_selection_menu(MenuMenu::Cup);
        if self.w().escape {
            return;
        }
        self.pick(if c == 0 { Next::CupNew } else { Next::CupLoad });
    }

    /// Practice: 10000 for each purse.
    fn toggle_display_flags(&mut self) {
        self.w().money = [10000; 3];
        self.pick(Next::Practice);
    }

    /// Records where a selection leads (a later part of the port).
    fn pick(&mut self, n: Next) {
        let mut w = self.w();
        w.next = n;
        w.picks += 1;
    }

    /// The menu runner: draws the menu, moves the arrows with the joystick (wrapping), returns the item fire chose.
    /// Esc leaves with the escape flag set.
    fn process_player_selection_menu(&mut self, m: MenuMenu) -> u16 {
        {
            let mut w = self.w();
            w.choice = 0;
            w.menu = m.id() as u8;
        }
        let backdrop = self.menu_backdrop(m);
        let n = self.menu(m).len() as u16;
        loop {
            self.show_menu(&backdrop, m);
            self.w().joy = 0;
            let joy = loop {
                let (key, joy) = {
                    let mut w = self.w();
                    let k = w.key;
                    w.key = 0;
                    (k, w.joy)
                };
                if key == KEY_ESC {
                    self.w().escape = true;
                    return self.w().choice;
                }
                if joy & (JOY_UP | JOY_DOWN | JOY_FIRE) != 0 {
                    break joy;
                }
                self.frame();
            };
            let mut w = self.w();
            if joy & JOY_UP != 0 {
                w.choice = if w.choice == 0 { n - 1 } else { w.choice - 1 };
            } else if joy & JOY_DOWN != 0 {
                w.choice += 1;
                if w.choice == n {
                    w.choice = 0;
                }
            } else {
                w.sfx(SFX_PICK);
                return w.choice;
            }
            w.sfx(SFX_MOVE);
        }
    }

    fn menu(&self, m: MenuMenu) -> &Menu {
        let s = &self.a.menus;
        match m {
            MenuMenu::Players => &s.players,
            MenuMenu::Matches => &s.matches,
            MenuMenu::League => &s.league,
            MenuMenu::Cup => &s.cup,
        }
    }

    /// The menu's still picture, drawn once into the backdrop buffer: the panel with the heading and footing
    /// (`process_entity_list`) and the items (`parse_player_name_string`). The panel's colours become the colours the
    /// screen fades to.
    fn menu_backdrop(&mut self, m: MenuMenu) -> Bitmap {
        let mut b = Bitmap::new(self.a.panel.w, self.a.panel.h);
        let clip = b.rect();
        b.blit(&self.a.panel, self.a.panel.rect(), 0, 0, clip);
        self.w().picture = self.a.panel_pal;
        let menu = self.menu(m);
        parse_and_project_entities(&mut b, &self.a.topfont, &menu.top, -1);
        parse_and_project_entities(&mut b, &self.a.botfont, &menu.bottom, 0);
        parse_player_name_string(&mut b, &self.a.menufont, &menu.items);
        b
    }

    /// One redraw: the backdrop copied to the drawing screen (`setup_copper_blit_sequence`), the arrows beside the
    /// chosen item (`process_player_coordinates`), and the screen shown (fading if the colours differ).
    fn show_menu(&mut self, backdrop: &Bitmap, m: MenuMenu) {
        {
            let choice = self.w().choice as usize;
            let [lx, ly, rx, ry] = self.menu(m).arrows[choice];
            let mut w = self.w();
            let s = w.back();
            let clip = w.screens[s].rect();
            w.screens[s].blit(backdrop, backdrop.rect(), 0, 0, clip);
            calculate_player_screen_position(&mut w.screens[s], &self.a.menufont, ARROW_L, lx, ly);
            calculate_player_screen_position(&mut w.screens[s], &self.a.menufont, ARROW_R, rx, ry);
        }
        self.wait_and_update_palette(false);
    }
}

/// The menus this slice runs (the series and disk-error menus come with the parts that use them).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuMenu {
    Players,
    Matches,
    League,
    Cup,
}

impl MenuMenu {
    fn id(self) -> MenuId {
        match self {
            MenuMenu::Players => MenuId::Players,
            MenuMenu::Matches => MenuId::Matches,
            MenuMenu::League => MenuId::League,
            MenuMenu::Cup => MenuId::Cup,
        }
    }
}

/// Draws a 16 x 16 menu glyph at a text position: 8 pixels a column byte, 4 lines a row, colour 0 transparent.
fn calculate_player_screen_position(b: &mut Bitmap, f: &Sheet, g: usize, col: u16, row: u16) {
    if g < f.frames {
        let clip = b.rect();
        b.blit_keyed(
            &f.image,
            0,
            f.frame(g),
            col as i32 * 8,
            row as i32 * 4,
            clip,
        );
    }
}

/// Writes menu items in the 16 x 16 font: a glyph or space moves two column bytes on, wrapping after column `$25`
/// to column 1 five rows down (`wrap_index_counter`).
fn parse_player_name_string(b: &mut Bitmap, f: &Sheet, text: &[u8]) {
    let (mut col, mut row) = (0u16, 0u16);
    let mut i = 0;
    while i < text.len() && text[i] != 0 {
        let c = text[i];
        if c < 0x20 {
            col = c as u16;
            row = text.get(i + 1).copied().unwrap_or(0) as u16;
            i += 2;
            continue;
        }
        if c > 0x20 {
            calculate_player_screen_position(b, f, c.wrapping_sub(b'A') as usize, col, row);
        }
        col += 2;
        if col > 0x25 {
            col = 1;
            row += 5;
        }
        i += 1;
    }
}

/// Writes a heading in an 8 x 8 font: one column byte a glyph, wrapping after 40 (`increment_and_check_limit`); `dy`
/// is the font's line offset (the heading sits one line higher than the footing).
fn parse_and_project_entities(b: &mut Bitmap, f: &Sheet, text: &[u8], dy: i32) {
    let (mut col, mut row) = (0u16, 0u16);
    let mut i = 0;
    while i < text.len() && text[i] != 0 {
        let c = text[i];
        if c < 0x20 {
            col = c as u16;
            row = text.get(i + 1).copied().unwrap_or(0) as u16;
            i += 2;
            continue;
        }
        let g = c.wrapping_sub(b'A') as usize;
        if c > 0x20 && g < f.frames {
            let clip = b.rect();
            b.blit_keyed(
                &f.image,
                0,
                f.frame(g),
                col as i32 * 8,
                row as i32 * 4 + dy,
                clip,
            );
        }
        col += 1;
        if col == 0x28 {
            col = 0;
            row += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet(w: usize, n: usize) -> Sheet {
        let mut img = Bitmap::new(w * n, w);
        img.fill(1);
        Sheet::new(img, w, n).unwrap()
    }

    #[test]
    fn menu_text_is_eight_pixels_a_column_byte_and_four_lines_a_row() {
        let mut b = Bitmap::new(320, 200);
        parse_player_name_string(&mut b, &sheet(16, 38), &[12, 11, b'K', b' ', b'O']);
        assert_eq!(b.get(96, 44), 1, "K at column 12, row 11");
        assert_eq!(b.get(112, 44), 0, "the space");
        assert_eq!(b.get(128, 44), 1, "O two columns on");
    }

    #[test]
    fn headings_move_one_column_a_glyph_and_the_heading_sits_a_line_up() {
        let mut b = Bitmap::new(320, 200);
        parse_and_project_entities(&mut b, &sheet(8, 28), &[14, 1, b'S', b'E'], -1);
        assert_eq!(b.get(112, 3), 1);
        assert_eq!(b.get(120, 3), 1);
        assert_eq!(b.get(112, 11), 0, "eight lines from row 1 less one");
    }
}
