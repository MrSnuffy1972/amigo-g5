//! The knockout and its NEXT FIXTURE screen (part 2 of the port, second slice). Written from the recovered routines
//! `cycle_team_selection_screen`, `render_post_match_summary`, `draw_two_team_names`, `render_team_stats_text`,
//! `render_ui_component`, `update_player_flags`, `initialize_entity_properties`, `parse_entity_message`,
//! `wait_for_input_flag`, `clear_player_stats`, `copy_player_struct_fields` and their helpers.
//!
//! The screen: the menu panel with NEXT FIXTURE and PRESS FIRE, the middle filled with colour 6, then 21 rows of
//! text in the 8 x 8 text font (6 pixels a character, the boxes are box-piece glyphs), the two team names and PLAY.
//! Each team's numbers are written into the shared rows (the left team at character 19, the right one at 49) before
//! they are drawn, as the original writes them into its strings.
//!
//! After the screen the original runs the gym and team screen (`handle_team_selection_logic`) and the match
//! (`initialize_match_state`); those are later parts of the port, so the knockout records where it leads and goes back
//! to the first menu.

use native_engine::gfx::{Bitmap, Sheet};

use super::menu::{parse_and_project_entities, JOY_FIRE, KEY_ESC};
use super::Intro;
use crate::league::{average_squad, digits, rating, POINTS};
use crate::world::{Next, Stage};

/// The colour the middle of the screen is filled with, and the filled rectangle (first and last column, first row,
/// rows).
const FILL: u8 = 6;
const FILL_X: (usize, usize) = (17, 17 + 0x11E - 1);
const FILL_Y: (usize, usize) = (17, 0xA6);
/// The player's team, and the left and right teams' text columns.
const OWN: usize = 1;
const LEFT: usize = 19;
const RIGHT: usize = 49;
/// Each knockout purse (1100).
const KNOCKOUT_PURSE: u16 = 0x44C;

impl Intro<'_> {
    /// Fills the league tables from the pack (the original has them in its program image).
    pub(super) fn load_league(&mut self) {
        let l = self.a.league.clone();
        let mut w = self.w();
        w.rows = l.rows.iter().map(|e| e.text.clone()).collect();
        w.teams = l.teams;
        w.squad = l.squad;
        w.rivals = l.rivals;
    }

    /// KNOCKOUT: every team's record cleared, 1100 in each purse, then the opponents one by one from team 1 of the
    /// table, each shown on the NEXT FIXTURE screen.
    pub(super) fn cycle_team_selection_screen(&mut self) {
        {
            let mut w = self.w();
            // clear_player_stats: the 17 records (offsets 8 to 21)
            for t in w.teams.iter_mut() {
                t.record = [0; 7];
            }
            w.mode = 0;
            w.money = [KNOCKOUT_PURSE; 3];
            w.opponent = 1;
        }
        // copy_player_struct_fields: the opponent's players take the team's skill groups
        {
            let mut w = self.w();
            let t = w.opponent as usize + 1;
            let sk = w.teams[t].skills;
            for (i, p) in w.rivals.iter_mut().enumerate() {
                *p = sk[i / 3];
            }
        }
        let right = self.w().opponent as usize + 1;
        self.render_post_match_summary(OWN, right);
        // the gym and team screen, then the match: later parts of the port
        self.pick(Next::Knockout);
    }

    /// The NEXT FIXTURE screen for two teams (indices in the table), shown until fire or a key.
    pub(super) fn render_post_match_summary(&mut self, left: usize, right: usize) {
        self.w().stage = Stage::Fixture;
        self.render_team_stats_text(left, LEFT);
        self.render_team_stats_text(right, RIGHT);
        {
            let l = &self.a.league;
            let mut w = self.w();
            let s = w.back();
            // process_entity_list: the panel, the heading, the footing
            w.screens[s].px.copy_from_slice(&self.a.panel.px);
            w.picture = self.a.panel_pal;
            parse_and_project_entities(&mut w.screens[s], &self.a.topfont, &l.top, -1);
            parse_and_project_entities(&mut w.screens[s], &self.a.botfont, &l.bottom, 0);
            // render_ui_component
            fill(&mut w.screens[s], FILL_X, FILL_Y, FILL);
            // update_player_flags: the rows, two rows of 4 lines apart from row 4
            let rows = w.rows.clone();
            for (i, (e, t)) in l.rows.iter().zip(rows.iter()).enumerate() {
                parse_entity_message(&mut w.screens[s], &self.a.textfont, e.x, 4 + 2 * i as u8, t);
            }
            // initialize_entity_properties: the names and PLAY
            let (ln, rn) = (w.teams[left].name.clone(), w.teams[right].name.clone());
            parse_entity_message(&mut w.screens[s], &self.a.textfont, 0x10, 6, &ln);
            parse_entity_message(
                &mut w.screens[s],
                &self.a.textfont,
                l.play.x,
                6,
                &l.play.text,
            );
            parse_entity_message(&mut w.screens[s], &self.a.textfont, 0xC4, 6, &rn);
        }
        self.wait_and_update_palette(false);
        self.wait_for_input_flag();
        self.w().stage = Stage::Menu;
    }

    /// Writes a team's numbers into the rows at text column `d4` (the original's string offsets less the entry's two
    /// position bytes). The player's team first takes its skills from its squad.
    fn render_team_stats_text(&mut self, team: usize, d4: usize) {
        let l = &self.a.league;
        let mut w = self.w();
        if team <= OWN {
            let sq = w.squad.clone();
            average_squad(&mut w.teams[team], &sq);
        }
        let t = w.teams[team].clone();
        let mode = w.mode;
        let rows = &mut w.rows;
        let col = d4 - 2;
        // the numbers row: points (or NA in the cup), then lost, drawn, won, played
        let r = &mut rows[l.row(0x2A46)];
        let mut p = col - 1;
        if mode <= 1 {
            p = digits(r, p, t.record[POINTS], 4);
        } else {
            for c in *b" AN " {
                if let Some(b) = r.get_mut(p) {
                    *b = c;
                }
                p -= 1;
            }
        }
        p -= 2;
        for k in (0..4).rev() {
            p = digits(r, p, t.record[k], 2);
            p -= 1;
        }
        // league placing, points for, points against: NA outside the league
        if mode != 1 {
            for (addr, text) in [(0x2ADF, &b"AN"[..]), (0x2B12, b"AN "), (0x2B45, b"AN ")] {
                let r = &mut rows[l.row(addr)];
                for (i, &c) in text.iter().enumerate() {
                    if let Some(b) = r.get_mut(col - i) {
                        *b = c;
                    }
                }
            }
        } else {
            let pl = match t.placing & 7 {
                0 => 8,
                n => n as u16,
            };
            digits(&mut rows[l.row(0x2ADF)], col, pl, 2);
            digits(&mut rows[l.row(0x2B12)], col, t.record[4], 3);
            digits(&mut rows[l.row(0x2B45)], col, t.record[5], 3);
        }
        // the four skill groups' ratings
        for (g, addr) in [0x2B78u16, 0x2BAB, 0x2BDE, 0x2C11].into_iter().enumerate() {
            digits(&mut rows[l.row(addr)], col, rating(&t.skills[g]), 3);
        }
    }

    /// Waits for fire or a key (`wait_for_input_flag`); Esc sets the escape flag.
    pub(super) fn wait_for_input_flag(&mut self) {
        {
            let mut w = self.w();
            w.key = 0;
            w.joy = 0;
        }
        loop {
            {
                let mut w = self.w();
                if w.joy & JOY_FIRE != 0 {
                    return;
                }
                let k = w.key;
                if k != 0 {
                    w.key = 0;
                    if k == KEY_ESC {
                        w.escape = true;
                    }
                    return;
                }
            }
            self.frame();
        }
    }
}

/// Fills a rectangle with one colour (`render_ui_component` through `loop_add_and_branch`: one line at a time, all
/// five planes written).
fn fill(b: &mut Bitmap, (x0, x1): (usize, usize), (y0, n): (usize, usize), c: u8) {
    for y in y0..(y0 + n).min(b.h) {
        for x in x0..=x1.min(b.w - 1) {
            b.px[y * b.w + x] = c;
        }
    }
}

/// Draws a text entry (`parse_entity_message`): from pixel column `x` at row `row` (4 lines a row), 6 pixels a
/// character, spaces skipped, colour 0 transparent.
pub(crate) fn parse_entity_message(b: &mut Bitmap, f: &Sheet, x: u8, row: u8, text: &[u8]) {
    let clip = b.rect();
    let mut px = x as i32;
    for &c in text.iter().take_while(|&&c| c != 0) {
        if c != 0x20 {
            let g = c.wrapping_sub(b'A') as usize;
            if g < f.frames {
                b.blit_keyed(&f.image, 0, f.frame(g), px, row as i32 * 4, clip);
            }
        }
        px += 6;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_six_pixels_a_character_and_four_lines_a_row() {
        let mut img = Bitmap::new(8 * 51, 8);
        img.fill(3);
        let f = Sheet::new(img, 8, 51).unwrap();
        let mut b = Bitmap::new(320, 200);
        parse_entity_message(&mut b, &f, 16, 6, b"A B");
        assert_eq!(b.get(16, 24), 3);
        assert_eq!(b.get(28, 24), 3, "B two characters on");
        assert_eq!(b.get(24, 32), 0);
    }

    #[test]
    fn the_fill_covers_its_rectangle_only() {
        let mut b = Bitmap::new(320, 200);
        fill(&mut b, FILL_X, FILL_Y, FILL);
        assert_eq!(b.get(17, 17), FILL);
        assert_eq!(b.get(302, 182), FILL);
        assert_eq!(b.get(303, 100), 0);
        assert_eq!(b.get(100, 183), 0);
    }
}
