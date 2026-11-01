//! The Crack Inc intro (unit `cracktro` of the recovered code): one static page of text, background and ink colours
//! set once, each line centred on the 352-pixel screen, eight-pixel glyphs on a ten-line pitch. In the original it
//! stayed up while the boot block read the start-up program (about 70 frames in the reference boot); here it stays up
//! for the same time, and fire skips it. Kept as the native intro at Karl's request; it reads no keys and no disks.

use native_engine::gfx::{Frame, TextMode};

use crate::assets::{IntroPage, W};

/// Colour indices the intro draws with.
const PAPER: u8 = 0;
const INK: u8 = 1;

pub struct Intro {
    pub ticks: u32,
}

impl Intro {
    pub fn new() -> Intro {
        Intro { ticks: 0 }
    }

    /// One tick; true when the intro is over.
    pub fn tick(&mut self, page: &IntroPage, skip: bool) -> bool {
        self.ticks += 1;
        skip || self.ticks >= page.ticks
    }

    pub fn render(&self, page: &IntroPage, frame: &mut Frame) {
        frame.palette.colours[PAPER as usize] = page.background;
        frame.palette.colours[INK as usize] = page.ink;
        frame.image.fill(PAPER);
        let (gw, _) = page.font.cell();
        for (i, line) in page.lines.iter().enumerate() {
            let x = (W as i32 - line.chars().count() as i32 * gw as i32) / 2;
            let y = page.top + i as i32 * page.step;
            page.font
                .draw(&mut frame.image, x, y, line, TextMode::Ink(INK));
        }
    }
}

impl Default for Intro {
    fn default() -> Self {
        Intro::new()
    }
}
