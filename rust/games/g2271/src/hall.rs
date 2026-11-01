//! The high-score table, "Hall Of Blame" (+9CA6 insert_high_score_entry), shown after the game ends. The original
//! kept it in SCORES.DAT (ten entries of a score and a name of up to 11 letters); the native build keeps it in the
//! engine's save store under `scores`. The name was typed on the keyboard (+D27A read_text_line, ten letters); the
//! native build takes it from the joystick as well (up and down choose a letter, fire adds it, the second button
//! takes one back, start or a full name ends), since the TV and phone builds have no keyboard.

use native_engine::input::Action;
use serde_json::{json, Value};

use crate::castle::{Castle, Phase};

/// Ten entries, best first.
pub const ENTRIES: usize = 10;
/// The letters the joystick steps through.
const LETTERS: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.-!";

/// The table from the save store (or ten empty entries, as the original made when SCORES.DAT was missing).
pub fn scores_from(v: Option<&Value>) -> Vec<(i32, String)> {
    let mut t: Vec<(i32, String)> = v
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .map(|e| {
                    let s = e.get("score").and_then(Value::as_i64).unwrap_or(0) as i32;
                    let n = e.get("name").and_then(Value::as_str).unwrap_or("");
                    (s, n.chars().take(11).collect())
                })
                .collect()
        })
        .unwrap_or_default();
    t.resize(ENTRIES, (0, String::new()));
    t.truncate(ENTRIES);
    t
}

pub fn scores_json(t: &[(i32, String)]) -> Value {
    Value::Array(
        t.iter()
            .map(|(s, n)| json!({"score": s, "name": n}))
            .collect(),
    )
}

/// The rank a score takes: before the first entry it beats (10 when none).
pub fn rank(t: &[(i32, String)], score: i32) -> usize {
    t.iter().position(|(s, _)| *s < score).unwrap_or(ENTRIES)
}

impl Castle {
    /// Text at a screen position in the current ink and paper (+B800 with a format already applied).
    fn text_at(&mut self, x: i32, y: i32, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            let g = self.glyph(ch);
            self.scr.char(&g, x + 8 * i as i32, y, true, true);
        }
    }

    /// +9CA6 insert_high_score_entry: the table on screen file 31, the new score in its place with its name, the
    /// table saved, then a wait for the tune to end or a button, and the crack's banner while the picture fades.
    pub fn hall_of_blame(&mut self) {
        self.read_controls();
        self.fade_out();
        self.music(6);
        self.phase = Phase::Hall;
        let score = self.st.score;
        let mut t = self.scores.clone();
        let r = rank(&t, score);
        self.draw_screen_file(31, 0, 0..25);
        self.scr.scroll = 0;
        self.scr.ink = 4;
        self.scr.paper = 0x18;
        self.text_at(0x70, 0x32, "Hall Of Blame");
        self.scr.ink = 3;
        if r < ENTRIES {
            for k in (r + 1..ENTRIES).rev() {
                t[k] = t[k - 1].clone();
            }
            t[r] = (score, String::new());
        }
        for (i, (s, n)) in t.iter().enumerate() {
            if *s == 0 {
                break;
            }
            if i != r {
                let y = 0x3E + 9 * i as i32;
                self.text_at(0xB0, y, &format!("{s:07}"));
                self.text_at(0x58, y, n);
            }
        }
        if r < ENTRIES {
            let y = 0x3E + 9 * r as i32;
            self.text_at(0xB0, y, &format!("{score:07}"));
            self.set_palette(0x34);
            t[r].1 = self.read_name(0x58, y);
            self.scores = t.clone();
            self.save_out = Some(scores_json(&t));
        } else {
            self.set_palette(0x34);
        }
        loop {
            if self.snd.check_ended() {
                break;
            }
            self.wait_frame();
            self.read_controls();
            if self.ctl.fire || self.input.pressed(Action::Start) {
                break;
            }
        }
        self.scr.ink = 2;
        self.text_at(0x58, 0xA8, " QUARTEX  ");
        self.fade_out();
    }

    /// The name, ten letters at most (+D27A read_text_line, from the joystick here).
    fn read_name(&mut self, x: i32, y: i32) -> String {
        let letters: Vec<char> = LETTERS.chars().collect();
        let mut name = String::new();
        let mut cur = 1usize;
        let mut held = 0u32;
        self.clear_presses();
        loop {
            let shown: String = format!("{name}{}", letters[cur]);
            self.text_at(x, y, &format!("{shown:<11}"));
            self.wait_frame();
            self.read_controls();
            let c = self.ctl;
            if c.up || c.down {
                // a step at once, then one every 6 frames while held
                if held == 0 || (held > 12 && held.is_multiple_of(6)) {
                    let n = letters.len();
                    cur = if c.up {
                        (cur + 1) % n
                    } else {
                        (cur + n - 1) % n
                    };
                }
                held += 1;
            } else {
                held = 0;
            }
            if c.fire && name.chars().count() < 10 {
                name.push(letters[cur]);
            }
            if c.fire2 {
                name.pop();
            }
            if self.input.pressed(Action::Start) || name.chars().count() >= 10 {
                break;
            }
        }
        let name = name.trim_end().to_string();
        self.text_at(x, y, &format!("{name:<11}"));
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_score_takes_its_rank_and_the_table_round_trips() {
        let mut t = scores_from(None);
        assert_eq!(t.len(), ENTRIES);
        assert_eq!(rank(&t, 100), 0);
        t[0] = (5000, "SIMON".into());
        t[1] = (300, "DRAC".into());
        assert_eq!(rank(&t, 5000), 1, "a tie goes below");
        assert_eq!(rank(&t, 400), 1);
        let back = scores_from(Some(&scores_json(&t)));
        assert_eq!(back, t);
    }
}
