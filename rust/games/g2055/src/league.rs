//! The league's tables as typed values (part 2 of the port, second slice): the 17 team records the original keeps
//! at `$7F50` (a pointer each; record 1 is the player's team, BRUTAL DELUXE, and 2 to 16 the computer teams in the
//! order the knockout meets them), the player's squad and the opponent's players (the pointer lists at `$594E` and
//! `$597E`), and the text rows of the NEXT FIXTURE screen (`$32BE`). Loaded from the asset pack's `fixture` data,
//! which `fixture2055.py` takes from the program image.

use serde_json::Value;

/// A line of text at a pixel column (the row is given when it is drawn): the original's entries are an x byte, a
/// row byte and a zero-ended string in the 8 x 8 text font (`byte - 'A'`; digits are the bytes after `Z`, the box
/// pieces the bytes after the digits).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub x: u8,
    pub text: Vec<u8>,
}

/// One team's record (the original's 114-byte team structures, the fields the screens read).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: Vec<u8>,
    /// Played, won, drawn, lost, points for, points against, points (the words at offsets 8 to 20).
    pub record: [u16; 7],
    /// The league placing (offset 22; the low three bits, 0 shown as 8).
    pub placing: u8,
    /// Defence, midfield, attack and substitutes: eight skill bytes each (offsets 24 to 55).
    pub skills: [[u8; 8]; 4],
    /// The four groups' average (offset 56).
    pub average: [u8; 8],
}

/// The record's fields by name.
pub const PLAYED: usize = 0;
pub const POINTS: usize = 6;

/// The NEXT FIXTURE screen's data and the starting tables.
#[derive(Debug, Clone)]
pub struct League {
    /// The screen's heading and footing (in the 8 x 8 heading fonts, as the menus).
    pub top: Vec<u8>,
    pub bottom: Vec<u8>,
    /// The 21 rows in drawing order, and each row's address in the original (the stats are written into the rows
    /// by address).
    pub rows: Vec<Entry>,
    pub row_ids: Vec<u16>,
    /// PLAY between the two names.
    pub play: Entry,
    pub teams: Vec<Team>,
    /// Twelve players each: skill bytes 3 to 10 of the player structures.
    pub squad: Vec<[u8; 8]>,
    pub rivals: Vec<[u8; 8]>,
}

fn bytes(v: &Value, what: &str) -> Result<Vec<u8>, String> {
    v.as_array()
        .ok_or(format!("fixture: {what} is not a list"))?
        .iter()
        .map(|x| {
            x.as_u64()
                .filter(|&n| n < 256)
                .map(|n| n as u8)
                .ok_or(format!("fixture: {what}: not a byte"))
        })
        .collect()
}

fn eight(v: &Value, what: &str) -> Result<[u8; 8], String> {
    bytes(v, what)?
        .try_into()
        .map_err(|_| format!("fixture: {what}: want 8 bytes"))
}

fn entry(v: &Value, what: &str) -> Result<Entry, String> {
    Ok(Entry {
        x: v["x"].as_u64().ok_or(format!("fixture: {what}: no x"))? as u8,
        text: bytes(&v["text"], what)?,
    })
}

fn list<T>(
    v: &Value,
    what: &str,
    f: impl Fn(&Value) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    v.as_array()
        .ok_or(format!("fixture: no {what}"))?
        .iter()
        .map(f)
        .collect()
}

fn team(v: &Value) -> Result<Team, String> {
    let rec = list(&v["record"], "record", |x| {
        x.as_u64()
            .map(|n| n as u16)
            .ok_or("fixture: record".to_string())
    })?;
    let sk = list(&v["skills"], "skills", |x| eight(x, "skills"))?;
    Ok(Team {
        name: bytes(&v["name"], "name")?,
        record: rec.try_into().map_err(|_| "fixture: want 7 record words")?,
        placing: v["placing"].as_u64().unwrap_or(0) as u8,
        skills: sk.try_into().map_err(|_| "fixture: want 4 skill groups")?,
        average: [0; 8],
    })
}

impl League {
    pub fn load(v: &Value) -> Result<League, String> {
        let l = League {
            top: bytes(&v["top"], "top")?,
            bottom: bytes(&v["bottom"], "bottom")?,
            rows: list(&v["rows"], "rows", |x| entry(x, "row"))?,
            row_ids: list(&v["row_ids"], "row_ids", |x| {
                x.as_str()
                    .and_then(|s| u16::from_str_radix(s, 16).ok())
                    .ok_or("fixture: row id".to_string())
            })?,
            play: entry(&v["play"], "play")?,
            teams: list(&v["teams"], "teams", team)?,
            squad: list(&v["squad"], "squad", |x| eight(x, "squad"))?,
            rivals: list(&v["rivals"], "rivals", |x| eight(x, "rivals"))?,
        };
        if l.rows.len() != 21 || l.row_ids.len() != 21 || l.teams.len() != 17 {
            return Err("fixture: want 21 rows and 17 teams".into());
        }
        if l.squad.len() != 12 || l.rivals.len() != 12 {
            return Err("fixture: want 12 players a team".into());
        }
        Ok(l)
    }

    /// The row the original keeps at `addr`.
    pub fn row(&self, addr: u16) -> usize {
        self.row_ids.iter().position(|&a| a == addr).unwrap_or(0)
    }
}

/// Writes `v` as `n` decimal digits leftwards from `pos` (the original's `write_divided_value_to_buffer`, called
/// once a digit; a digit is the byte `$5B + d`). Returns the position left of the last digit.
pub fn digits(s: &mut [u8], pos: usize, v: u16, n: usize) -> usize {
    let mut v = v;
    let mut p = pos;
    for _ in 0..n {
        if let Some(c) = s.get_mut(p) {
            *c = 0x5B + (v % 10) as u8;
        }
        v /= 10;
        p = p.wrapping_sub(1);
    }
    p
}

/// The squad's groups (three players each: defence, midfield, attack, substitutes) averaged into a team's skills
/// (`average_attribute_bytes`), then the groups averaged into `average` (`average_player_attributes`).
pub fn average_squad(t: &mut Team, squad: &[[u8; 8]]) {
    for (g, sk) in t.skills.iter_mut().enumerate() {
        for (k, b) in sk.iter_mut().enumerate() {
            let sum: u16 = (0..3).map(|p| squad[g * 3 + p][k] as u16).sum();
            *b = (sum / 3) as u8;
        }
    }
    for k in 0..8 {
        let sum: u16 = t.skills.iter().map(|g| g[k] as u16).sum();
        t.average[k] = (sum >> 2) as u8;
    }
}

/// A skill group's rating: its eight bytes summed and divided by eight (`sum_byte_array_divide_eight`).
pub fn rating(g: &[u8; 8]) -> u16 {
    g.iter().map(|&b| b as u16).sum::<u16>() >> 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_go_leftwards_as_the_bytes_after_z() {
        let mut s = *b"[[[[";
        assert_eq!(digits(&mut s, 3, 127, 3), 0);
        assert_eq!(&s, b"[\\]b");
    }

    #[test]
    fn a_rating_is_the_eight_bytes_over_eight() {
        assert_eq!(rating(&[130, 120, 130, 120, 140, 120, 120, 140]), 127);
    }

    #[test]
    fn the_squad_averages_three_players_a_group() {
        let mut t = Team {
            name: vec![],
            record: [0; 7],
            placing: 0,
            skills: [[0; 8]; 4],
            average: [0; 8],
        };
        let mut sq = vec![[100u8; 8]; 12];
        sq[0][0] = 103;
        average_squad(&mut t, &sq);
        assert_eq!(t.skills[0][0], 101);
        assert_eq!(t.skills[1][0], 100);
        assert_eq!(t.average[0], 100);
    }
}
