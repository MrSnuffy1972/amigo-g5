//! The main program (the author's `_main`): the intro text page, the title screen, the pro shop and, so far, the
//! hole view of every hole of both courses. Each routine waits for the next frame with `frame()`, which hands the
//! picture to the engine and takes that tick's input, so the program keeps the original's nested-loop shape.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::gfx::{Argb, Bitmap, Palette};
use native_engine::input::{Action, InputState};

use crate::assets::{Assets, DH, DW, H, W};
use crate::co::{FrameIn, FrameOut, Stop};
use crate::hole;
use crate::intro;
use crate::text::Text;

/// Which screen the program is on (named state for verification).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Intro = 0,
    Title = 1,
    ProShop = 2,
    Hole = 3,
}

pub struct Golf {
    pub a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    pub input: InputState,
    /// The pointer in game pixels (320 x 200).
    pub ptr: (i32, i32),
    fine: (i32, i32),
    pub scr: Bitmap,
    hires: Option<Bitmap>,
    pal: Vec<Argb>,
    /// Text drawn over the doubled picture at full resolution: (x, y, text, ink, background) in DW x DH pixels.
    pub labels: Vec<(i32, i32, String, u8, Option<u8>)>,
    small: Text,
    console_font: Text,
    pub screen: Screen,
    pub course: usize,
    pub hole: usize,
    pub tick: u32,
    seed: u32,
}

fn colours(p: &Palette) -> Vec<Argb> {
    p.colours.clone()
}

impl Golf {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Golf {
        let small = Text::new(a.mono.clone(), 8, 16);
        let console_font = Text::new(a.mono.clone(), 8, 8);
        Golf {
            pal: colours(&a.pal_title),
            a,
            rx,
            tx,
            input: InputState::default(),
            ptr: (160, 100),
            fine: (320, 200),
            scr: Bitmap::new(W, H),
            hires: None,
            labels: Vec::new(),
            small,
            console_font,
            screen: Screen::Intro,
            course: 0,
            hole: 0,
            tick: 0,
            seed: seed.unwrap_or(0x1598),
        }
    }

    fn state(&self) -> Vec<(String, i64)> {
        vec![
            ("screen".into(), self.screen as i64),
            ("course".into(), self.course as i64),
            ("hole".into(), self.hole as i64 + 1),
            ("pointer_x".into(), self.ptr.0 as i64),
            ("pointer_y".into(), self.ptr.1 as i64),
            ("seed".into(), self.seed as i64),
        ]
    }

    /// Shows the picture and waits for the next tick's input.
    pub fn frame(&mut self) {
        let mut screen = vec![0u8; DW * DH];
        if let Some(h) = &self.hires {
            screen.copy_from_slice(&h.px[..DW * DH]);
        } else {
            for y in 0..DH {
                let src = &self.scr.px[(y / 2) * W..(y / 2) * W + W];
                let row = &mut screen[y * DW..(y + 1) * DW];
                for (x, c) in src.iter().enumerate() {
                    row[2 * x] = *c;
                    row[2 * x + 1] = *c;
                }
            }
            let mut b = Bitmap {
                w: DW,
                h: DH,
                px: screen,
            };
            for (x, y, t, ink, bg) in &self.labels {
                self.small.draw(&mut b, *x, *y, t, *ink, *bg);
            }
            screen = b.px;
            self.draw_pointer(&mut screen);
        }
        let out = FrameOut {
            screen,
            palette: self.pal.clone(),
            quit: false,
            state: self.state(),
        };
        if self.tx.send(out).is_err() {
            std::panic::panic_any(Stop);
        }
        match self.rx.recv() {
            Ok(i) => self.input = i.input,
            Err(_) => std::panic::panic_any(Stop),
        }
        self.tick += 1;
        if let Some((x, y)) = self.input.absolute {
            self.fine = (x, y);
        } else {
            self.fine.0 += self.input.motion.0;
            self.fine.1 += self.input.motion.1;
        }
        self.fine.0 = self.fine.0.clamp(0, DW as i32 - 1);
        self.fine.1 = self.fine.1.clamp(0, DH as i32 - 1);
        self.ptr = (self.fine.0 / 2, self.fine.1 / 2);
    }

    /// The mouse pointer (the original's sprite: a small arrow in colour 15 outlined in 0).
    fn draw_pointer(&self, screen: &mut [u8]) {
        const ARROW: [u8; 8] = [0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xe0, 0xb0, 0x18];
        let (px, py) = (self.ptr.0 * 2, self.ptr.1 * 2);
        for (r, bits) in ARROW.iter().enumerate() {
            for c in 0..8 {
                if bits & (0x80 >> c) != 0 {
                    for (dy, dx) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                        let x = px + c * 2 + dx;
                        let y = py + r as i32 * 2 + dy;
                        if (0..DW as i32).contains(&x) && (0..DH as i32).contains(&y) {
                            screen[y as usize * DW + x as usize] = 15;
                        }
                    }
                }
            }
        }
    }

    fn clicked(&self) -> bool {
        self.input.pressed(Action::Primary) || self.input.pressed(Action::Fire)
    }

    fn cancelled(&self) -> bool {
        self.input.pressed(Action::Cancel) || self.input.pressed(Action::Secondary)
    }

    fn quit(&mut self) -> ! {
        let out = FrameOut {
            screen: vec![0; DW * DH],
            palette: self.pal.clone(),
            quit: true,
            state: self.state(),
        };
        let _ = self.tx.send(out);
        let _ = self.rx.recv();
        std::panic::panic_any(Stop);
    }

    pub fn run(&mut self) {
        self.intro();
        self.title_screen();
        loop {
            match self.pro_shop() {
                Some(c) => {
                    self.course = c;
                    self.play();
                }
                None => self.quit(),
            }
        }
    }

    /// The cracktro page, typed onto the console as the startup-sequence's `type s:text` did, then held until a
    /// click or five seconds.
    fn intro(&mut self) {
        self.screen = Screen::Intro;
        let ops = intro::parse(&self.a.intro);
        let mut con = intro::Console::default();
        let mut hi = Bitmap::new(DW, DH);
        self.pal = vec![0xFF00_0000, 0xFFFF_FFFF, 0xFF66_88BB, 0xFF00_55AA];
        self.pal.resize(16, 0xFF00_0000);
        let per_tick = 40;
        let mut i = 0;
        let mut hold = 250;
        loop {
            for _ in 0..per_tick {
                if i < ops.len() {
                    con.run(&ops[i], &mut self.console_font);
                    i += 1;
                }
            }
            for y in 0..intro::CH {
                let o = (y + (DH - intro::CH) / 2) * DW;
                hi.px[o..o + DW].copy_from_slice(&con.scr.px[y * DW..(y + 1) * DW]);
            }
            self.hires = Some(hi.clone());
            self.frame();
            if self.clicked() || self.input.pressed(Action::Start) {
                break;
            }
            if i >= ops.len() {
                hold -= 1;
                if hold == 0 {
                    break;
                }
            }
        }
        self.hires = None;
    }

    /// `title_screen`: the title picture until a click.
    fn title_screen(&mut self) {
        self.screen = Screen::Title;
        self.labels.clear();
        self.pal = colours(&self.a.pal_title);
        self.scr.px.copy_from_slice(&self.a.title.px);
        loop {
            self.frame();
            if self.clicked() || self.input.pressed(Action::Start) {
                return;
            }
        }
    }

    /// The pro shop with its course choice (an interim menu: the original's menu bar is ported in a later part).
    fn pro_shop(&mut self) -> Option<usize> {
        self.screen = Screen::ProShop;
        self.pal = colours(&self.a.pal_proshop);
        let names: Vec<String> = self.a.courses.iter().map(|c| c.name.clone()).collect();
        loop {
            self.scr.px.copy_from_slice(&self.a.proshop.px);
            self.labels.clear();
            let mut items: Vec<(i32, i32, i32, i32)> = Vec::new();
            for (i, n) in names
                .iter()
                .chain(std::iter::once(&"Quit".to_string()))
                .enumerate()
            {
                let y = 150 + i as i32 * 12;
                let w = (n.chars().count() as i32 * self.small.cell_w + 16) / 2;
                let x = 160 - w / 2;
                let hot = (x..x + w).contains(&self.ptr.0) && (y - 2..y + 10).contains(&self.ptr.1);
                for yy in y - 2..y + 10 {
                    for xx in x..x + w {
                        self.scr.set(xx, yy, if hot { 15 } else { 0 });
                    }
                }
                let tw = n.chars().count() as i32 * self.small.cell_w;
                self.labels.push((
                    320 - tw / 2,
                    2 * y - 2,
                    n.clone(),
                    if hot { 0 } else { 15 },
                    None,
                ));
                items.push((x, y - 2, w, 12));
            }
            self.frame();
            if self.clicked() {
                for (i, &(x, y, w, h)) in items.iter().enumerate() {
                    if (x..x + w).contains(&self.ptr.0) && (y..y + h).contains(&self.ptr.1) {
                        return if i < names.len() { Some(i) } else { None };
                    }
                }
            }
            if self.cancelled() {
                return None;
            }
        }
    }

    /// The holes of the chosen course, each drawn from its tee as `init_scene` sets the view; a click on the left
    /// or right of the picture goes back or on a hole, cancel returns to the pro shop.
    fn play(&mut self) {
        self.screen = Screen::Hole;
        self.pal = colours(&self.a.pal_new);
        self.hole = 0;
        let mut dirty = true;
        loop {
            if dirty {
                self.draw_hole_from_tee();
                dirty = false;
            }
            self.frame();
            if self.cancelled() {
                return;
            }
            if self.clicked() {
                let n = self.a.courses[self.course].holes.len();
                self.hole = if self.ptr.0 < 160 {
                    (self.hole + n - 1) % n
                } else {
                    (self.hole + 1) % n
                };
                dirty = true;
            }
        }
    }

    pub fn draw_hole_from_tee(&mut self) {
        let a = self.a.clone();
        let c = &a.courses[self.course];
        let h = &c.holes[self.hole];
        let l = hole::load(h, 0);
        let target = if h.par != 3 { l.aim } else { l.pin };
        let v = hole::camera(l.tee, target);
        hole::draw(&mut self.scr, &v, h, &l, c, &a.gr3, &a.tables);
        let label = format!(
            "{}  Hole {}  Par {}  {} yds",
            c.name,
            self.hole + 1,
            h.par,
            h.yards[0]
        );
        self.labels.clear();
        self.labels.push((8, 380, label, 15, Some(0)));
    }
}
