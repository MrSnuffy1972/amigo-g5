//! The front end's menu system, rewritten from the recovered main program: a menu is a list of items drawn from a
//! descriptor (`open_menu_descriptor` $C1E656, item drawer $C1EC62) and a navigation grid walked by the stick
//! ($C1EA4E); the menu loop ($C1E2EE) flashes a one-pixel cursor round the current item, moves it at most once
//! every six frames while the stick is held, and activates the item on fire (then ignores fire for 8 frames).

use native_engine::gfx::{Argb, Bitmap, Frame, Rect};

use crate::assets::{FrontEnd, Item, Label, MenuDef};

/// Frames from opening a menu to its loop's first pass: the original clears its frame counter, draws the menu while
/// fading the palette in (counter 3 to 6) and only then starts the loop, which draws the cursor (measured on the
/// reference main menu: first cursor at counter 7).
pub const SETUP: u32 = 7;

/// Fade levels (sixteenths) by frame counter during the set-up, from 0.
const FADE_IN: [u32; 7] = [0, 0, 0, 2, 4, 8, 16];

/// Menu numbers (order of the `menus` asset).
pub const LANGUAGE: usize = 0;
pub const MAIN: usize = 1;

/// The joystick as the menus read it (the original's held direction and fire bytes, $C0DE43-$C0DE47).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stick {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fire: bool,
}

impl Stick {
    pub fn any_direction(&self) -> bool {
        self.left || self.right || self.up || self.down
    }
}

/// What a menu tick ended with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    None,
    /// Fire on item n (the menu's activate handler returns it to the caller).
    Activated(usize),
    /// The attract time-out ran out (the original returned $ABCD).
    Idle,
    /// Fire on the main menu while attract mode was on: back to the language screen.
    AttractExit,
}

/// A menu being shown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuRun {
    pub menu: usize,
    pub cur: usize,
    pub col: i32,
    pub row: i32,
    /// The original's frame counter ($C0DE6A): it drives the cursor flash and the attract time-out. Some menu
    /// openings clear it, others carry it on from the screen before (`MenuRun::open`).
    pub ticks: u32,
    /// Frames since this menu opened (set-up and fade-in).
    pub age: u32,
    pub move_wait: u32,
    pub fire_wait: u32,
    /// Attract mode ($C1D732) and its time-out ($C1D372), in frames since the menu opened.
    pub attract: bool,
    pub idle_limit: u32,
    /// The cursor moved this frame: the original erased it and drew it at the new item the next frame.
    pub moved: bool,
    /// The move's redraw waited for the next frame, so the loop skips a pass after a move (a held stick repeats
    /// every six frames, as on the reference main menu).
    pub skip: bool,
}

impl MenuRun {
    /// `counter`: the frame counter's value at opening (0 where the original clears it).
    pub fn open(
        fe: &FrontEnd,
        menu: usize,
        attract: bool,
        idle_limit: u32,
        counter: u32,
    ) -> MenuRun {
        let m = &fe.menus[menu];
        MenuRun {
            menu,
            cur: m.first,
            col: m.col,
            row: m.row,
            ticks: counter,
            age: 0,
            // on entry the loop waits 4 frames before a move and sets the low byte of the fire wait ($C1E2EE),
            // so fire must be let go before it can activate anything
            move_wait: fe.ui.move_delay,
            fire_wait: 0xFF,
            attract,
            idle_limit,
            moved: false,
            skip: false,
        }
    }

    /// Opens a menu on a remembered cursor (the main menu keeps its place, $C1D31A).
    pub fn reopen(fe: &FrontEnd, menu: usize, at: (usize, i32, i32), counter: u32) -> MenuRun {
        let mut r = MenuRun::open(fe, menu, false, 0, counter);
        (r.cur, r.col, r.row) = at;
        r
    }

    /// One frame of the menu loop.
    pub fn tick(&mut self, fe: &FrontEnd, s: Stick) -> Event {
        self.moved = false;
        self.ticks += 1;
        self.age += 1;
        if self.attract && self.ticks >= self.idle_limit {
            return Event::Idle;
        }
        if self.age < SETUP {
            return Event::None;
        }
        if self.skip {
            self.skip = false;
            return Event::None;
        }
        if s.fire && self.attract {
            self.attract = false;
            if self.menu == MAIN {
                return Event::AttractExit;
            }
        }
        let m = &fe.menus[self.menu];
        if self.move_wait > 0 {
            self.move_wait -= 1;
            if s.any_direction() {
                return Event::None;
            }
            self.move_wait = 0;
        } else if s.any_direction() {
            // the original tests left, right, down, up in that order
            let (dx, dy) = if s.left {
                (-1, 0)
            } else if s.right {
                (1, 0)
            } else if s.down {
                (0, 1)
            } else {
                (0, -1)
            };
            if let Some((item, col, row)) = navigate(m, self.col, self.row, dx, dy) {
                (self.cur, self.col, self.row) = (item, col, row);
                self.move_wait = fe.ui.move_delay;
                self.moved = true;
                self.skip = true;
            }
            return Event::None;
        }
        if self.fire_wait > 0 {
            self.fire_wait -= 1;
            if !s.fire {
                self.fire_wait = 0;
            }
            return Event::None;
        }
        if s.fire {
            self.fire_wait = fe.ui.fire_delay;
            return Event::Activated(self.cur);
        }
        Event::None
    }
}

/// The grid walk ($C1EA4E): from cell (col, row) one step (dx, dy), clamped to the grid; then follow the cell's
/// code until it names an item. Returns the item and the cell it was found in, or None (-1, or a dead end).
pub fn navigate(m: &MenuDef, col: i32, row: i32, dx: i32, dy: i32) -> Option<(usize, i32, i32)> {
    let (cols, rows) = (m.cols, m.rows);
    let (mut c, mut r) = (col + dx, row + dy);
    let clamp = |c: &mut i32, r: &mut i32| {
        if *c < 0 {
            *c = 0;
        }
        if *c == cols {
            *c -= 1;
        }
        if *r < 0 {
            *r = 0;
        }
        if *r == rows {
            *r -= 1;
        }
    };
    clamp(&mut c, &mut r);
    let mut i = r * cols + c;
    // a bound on the walk: the original has none, but a well-formed grid ends within a few cells
    for _ in 0..(cols * rows * 4).max(16) {
        let v = *m.grid.get(usize::try_from(i).ok()?)?;
        match v {
            0.. => return Some((v as usize, c, r)),
            -1 => return None,
            -2 => {
                i -= cols;
                r -= 1;
            }
            -3 => {
                i += cols;
                r += 1;
            }
            -4 => {
                i -= 1;
                c -= 1;
            }
            -5 => {
                i += 1;
                c += 1;
            }
            -8..=-6 => {
                // -6 only for a sideways step, -7 only for an up/down step, -8 always: carry on in the same direction
                if (v == -6 && dx == 0) || (v == -7 && dy == 0) {
                    return None;
                }
                c += dx;
                r += dy;
                clamp(&mut c, &mut r);
                i = r * cols + c;
            }
            -9 => {
                i += 1 + cols;
                c += 1;
                r += 1;
            }
            -10 => {
                i += 1 - cols;
                c += 1;
                r -= 1;
            }
            _ => return None,
        }
    }
    None
}

/// Draws sprite `n` with its top left at (x, y): colour 0 is transparent; `ink`, if given, replaces colour 10
/// (the face colour of the font glyphs, which the original recoloured once per text colour).
pub fn draw_sprite(
    fe: &FrontEnd,
    dst: &mut Bitmap,
    n: usize,
    x: i32,
    y: i32,
    ink: Option<u8>,
) -> Option<i32> {
    let s = (*fe.sprites.get(n)?)?;
    let src = &fe.pics[s.pic];
    // the original cut and blitted whole 16-pixel words, so a sprite shows its picture up to the next word edge
    let w = ((s.w + 15) & !15).min(src.w as i32 - s.x);
    let clip = dst.rect();
    dst.blit_with(
        src,
        Rect::new(s.x, s.y, w, s.h),
        x,
        y,
        clip,
        |_, sp, _| match sp {
            0 => None,
            10 => Some(ink.unwrap_or(10)),
            c => Some(c),
        },
    );
    Some(s.w)
}

/// The item's text in the chosen language (the program's English block replaced by the language's).
pub fn label<'a>(fe: &'a FrontEnd, it: &'a Item, lang: usize) -> &'a [u8] {
    match &it.label {
        Label::None => &[],
        Label::Text(t) => t,
        Label::TextOff(o) => {
            let b = &fe.texts[lang.min(fe.texts.len() - 1)];
            let s = &b[(*o).min(b.len())..];
            &s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())]
        }
    }
}

fn glyph(fe: &FrontEnd, base: usize, c: u8) -> Option<usize> {
    let g = *fe.ui.font_index.get(c.wrapping_sub(32) as usize)?;
    (g != 255).then_some(base + g as usize)
}

fn width(fe: &FrontEnd, n: usize) -> i32 {
    fe.sprites.get(n).copied().flatten().map_or(0, |s| s.w)
}

/// Text centred on x ($C0A496 measuring, $C0A56E drawing): glyphs advance their width less one, a space half the
/// width of the font's `A`; drawing stops at the right limit (x 336).
pub fn draw_text_centred(
    fe: &FrontEnd,
    dst: &mut Bitmap,
    text: &[u8],
    cx: i32,
    y: i32,
    colour: u8,
) {
    let base = fe
        .ui
        .font_base
        .iter()
        .find(|&&(c, _)| c == colour as u16)
        .or_else(|| fe.ui.font_base.iter().find(|&&(c, _)| c == 10))
        .map_or(0, |&(_, b)| b as usize);
    let space = glyph(fe, base, b'A').map_or(0, |g| width(fe, g) / 2);
    let mut w = 0;
    for &c in text {
        w += if c == b' ' {
            space
        } else {
            glyph(fe, base, c).map_or(0, |g| width(fe, g) - 1)
        };
    }
    let mut x = cx - (w + 1) / 2;
    for &c in text {
        if c == b' ' {
            if x + space >= fe.ui.text_right {
                return;
            }
            x += space;
            continue;
        }
        let Some(g) = glyph(fe, base, c) else {
            continue;
        };
        let gw = width(fe, g);
        if x + gw > fe.ui.text_right {
            return;
        }
        draw_sprite(fe, dst, g, x, y, Some(colour));
        x += gw - 1;
    }
}

/// One item ($C1EC62): the fill (unless bit 7 of the low byte says the background shows through), for framed
/// items (bit 15) a ring two pixels in, a top edge in the frame colour and a bottom edge in the fill's shade; then
/// the content by kind: $04 a sprite at its offset, $0B the text centred in the box in the frame colour's font.
pub fn draw_item(fe: &FrontEnd, dst: &mut Bitmap, it: &Item, lang: usize) {
    if it.kind == 0 || it.kind & 0x8000 != 0 {
        return;
    }
    let (x, y, w, h) = (it.x, it.y, it.w, it.h);
    let fill = (it.attr & 0x0F) as u8;
    let frame = ((it.attr >> 8) & 0x0F) as u8;
    if it.attr & 0x80 == 0 {
        dst.fill_rect(Rect::new(x, y, w, h), fill);
        if it.attr & 0x8000 != 0 {
            dst.fill_rect(Rect::new(x + 2, y + 2, w - 4, h - 4), frame);
            dst.fill_rect(Rect::new(x + 3, y + 3, w - 6, h - 6), fill);
            dst.fill_rect(Rect::new(x + 1, y, w - 2, 1), frame);
            let shade = fe.ui.shade.get(fill as usize).copied().unwrap_or(fill);
            dst.fill_rect(Rect::new(x + 1, y + h - 1, w - 2, 1), shade);
        }
    }
    let mut colour = ((it.attr >> 8) & 0x7F) as u8;
    if colour == 0 {
        colour = 10;
    }
    match it.kind {
        0x04 => {
            draw_sprite(fe, dst, it.sprite as usize, x + it.sdx, y + it.sdy, None);
        }
        0x0B => {
            let d = h - fe.ui.font_h;
            let ty = y + (d >> 1) + (d & 1);
            draw_text_centred(fe, dst, label(fe, it, lang), x + (w >> 1), ty, colour);
        }
        _ => {}
    }
}

/// The cursor ($C1F078): a one-pixel outline round the item in the flash colour of this frame.
pub fn draw_cursor(dst: &mut Bitmap, it: &Item, c: u8) {
    let (x, y, w, h) = (it.x, it.y, it.w, it.h);
    dst.fill_rect(Rect::new(x, y, 1, h), c);
    dst.fill_rect(Rect::new(x, y, w, 1), c);
    dst.fill_rect(Rect::new(x + w - 1, y, 1, h), c);
    dst.fill_rect(Rect::new(x, y + h - 1, w, 1), c);
}

/// A colour faded to `m` sixteenths, per 4-bit Amiga channel (truncating, as the reference fades show: 15 -> 7, 3, 1).
pub fn fade(c: Argb, m: u32) -> Argb {
    if m >= 16 {
        return c;
    }
    let ch = |sh: u32| (((c >> sh) & 0xFF) / 17 * m / 16) * 17;
    0xFF00_0000 | ch(16) << 16 | ch(8) << 8 | ch(0)
}

/// The menu screen as shown for a menu run: faded in during the set-up, the cursor from the loop's first pass and
/// not on a frame where it moved.
pub fn render(fe: &FrontEnd, run: &MenuRun, lang: usize, frame: &mut Frame) {
    let m = FADE_IN.get(run.age as usize).copied().unwrap_or(16);
    let cursor = run.age >= SETUP && !run.moved;
    render_screen(fe, run, lang, frame, m, cursor);
}

/// The whole menu screen: background, items, cursor. The menu display starts one pixel to the right of the
/// screen's left edge (column 0 shows the background colour 0), as the reference frames show. `m`: fade level in
/// sixteenths.
pub fn render_screen(
    fe: &FrontEnd,
    run: &MenuRun,
    lang: usize,
    frame: &mut Frame,
    m: u32,
    cursor: bool,
) {
    let n = frame.palette.colours.len();
    for (i, &c) in fe.palette.colours.iter().enumerate().take(n) {
        frame.palette.colours[i] = fade(c, m);
    }
    let clip = frame.image.rect();
    frame.image.fill(0);
    frame
        .image
        .blit(&fe.background, fe.background.rect(), 0, 0, clip);
    let menu = &fe.menus[run.menu];
    for it in &menu.items {
        draw_item(fe, &mut frame.image, it, lang);
    }
    if let Some(it) = menu.items.get(run.cur).filter(|_| cursor) {
        let f = &fe.ui.flash;
        if !f.is_empty() {
            let c = f[((run.ticks >> 1) as usize) % f.len()];
            draw_cursor(&mut frame.image, it, c);
        }
    }
    let w = frame.image.w;
    for row in frame.image.px.chunks_mut(w) {
        row.copy_within(0..w - 1, 1);
        row[0] = 0;
    }
}
