//! The intro: the cracking group's text page (`s/text`, typed by the disk's startup-sequence before the game). It is
//! Amiga console output, and its picture is built with the console's raster offsets: CSI (0x9B) `n y` moves the text
//! window's top down n pixels, so each line is drawn one pixel below the last and only the top rows of the earlier
//! lines stay visible; `n x` sets the left offset, `n C` moves forward, `r;c H` positions, `K` clears to the end of the
//! line, `7m` and `0m` switch inverse video, a form feed clears the window. Executed here op by op onto a 640 x 256
//! console in 8 x 8 cells (the ROM font is not shipped; `Text` stands in for it).

use crate::text::Text;
use native_engine::gfx::Bitmap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Put(char),
    Cr,
    Lf,
    Clear,
    Forward(usize),
    Pos(usize, usize),
    ClearEol,
    Inverse(bool),
    Top(i32),
    Left(i32),
}

pub fn parse(s: &str) -> Vec<Op> {
    let cs: Vec<char> = s.chars().collect();
    let mut ops = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let ch = cs[i];
        i += 1;
        match ch {
            '\u{9b}' => {
                let mut arg = String::new();
                while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == ';' || cs[i] == ' ') {
                    arg.push(cs[i]);
                    i += 1;
                }
                let Some(&cmd) = cs.get(i) else { break };
                i += 1;
                let n: Vec<usize> = arg
                    .split(';')
                    .filter_map(|x| x.trim().parse().ok())
                    .collect();
                let n0 = n.first().copied();
                match cmd {
                    'C' => ops.push(Op::Forward(n0.unwrap_or(1))),
                    'H' => ops.push(Op::Pos(n0.unwrap_or(1), n.get(1).copied().unwrap_or(1))),
                    'K' => ops.push(Op::ClearEol),
                    'm' => ops.push(Op::Inverse(n.contains(&7))),
                    'y' => ops.push(Op::Top(n0.unwrap_or(0) as i32)),
                    'x' => ops.push(Op::Left(n0.unwrap_or(0) as i32)),
                    _ => {}
                }
            }
            '\r' => ops.push(Op::Cr),
            '\n' => ops.push(Op::Lf),
            '\x0c' => ops.push(Op::Clear),
            ch if (ch as u32) >= 32 => ops.push(Op::Put(ch)),
            _ => {}
        }
    }
    ops
}

/// The page draws its large lettering from the top pixel rows of a few characters, one text line a raster line
/// apart. These are this build's own top rows for those characters (drawn to make the lettering), standing in for
/// the ROM font's.
pub fn top_row(c: char) -> Option<u8> {
    Some(match c {
        'L' => 0xf0,
        'B' => 0xfc,
        '0' => 0x7c,
        '\\' => 0xc0,
        '/' => 0x03,
        '(' => 0x0c,
        ')' => 0x30,
        'V' => 0xc6,
        '\u{7f}' => 0xff,
        '\u{e4}' => 0x6c,
        ' ' => 0x00,
        _ => return None,
    })
}

pub const CW: usize = 640;
pub const CH: usize = 256;

/// The console the page is typed onto: pen 1 on pen 0.
pub struct Console {
    pub scr: Bitmap,
    row: usize,
    col: usize,
    inv: bool,
    top: i32,
    left: i32,
}

impl Default for Console {
    fn default() -> Console {
        Console {
            scr: Bitmap::new(CW, CH),
            row: 0,
            col: 0,
            inv: false,
            top: 0,
            left: 0,
        }
    }
}

impl Console {
    pub fn run(&mut self, op: &Op, t: &mut Text) {
        match op {
            Op::Put(c) => {
                let x = self.left + self.col as i32 * 8;
                let y = self.top + self.row as i32 * 8;
                let (ink, bg) = if self.inv { (0, 1) } else { (1, 0) };
                t.draw(&mut self.scr, x, y, &c.to_string(), ink, Some(bg));
                if let Some(bits) = top_row(*c) {
                    for k in 0..8 {
                        self.scr
                            .set(x + k, y, if bits & (0x80 >> k) != 0 { ink } else { bg });
                    }
                }
                self.col += 1;
            }
            Op::Cr => self.col = 0,
            Op::Lf => {
                self.row += 1;
                self.col = 0;
            }
            Op::Clear => {
                self.scr.fill(0);
                self.row = 0;
                self.col = 0;
            }
            Op::Forward(n) => self.col += n,
            Op::Pos(r, c) => {
                self.row = r.saturating_sub(1);
                self.col = c.saturating_sub(1);
            }
            Op::ClearEol => {
                let x = self.left + self.col as i32 * 8;
                let y = self.top + self.row as i32 * 8;
                for yy in y..y + 8 {
                    for xx in x..CW as i32 {
                        self.scr.set(xx, yy, 0);
                    }
                }
            }
            Op::Inverse(v) => self.inv = *v,
            Op::Top(v) => self.top = *v,
            Op::Left(v) => self.left = *v,
        }
    }
}
