//! Global Thermonuclear Warfare, rewritten natively from its recovered code (game 296). Two players take turns, America
//! and Russia, each picking a launcher, setting elevation and heading and firing one missile across a scrolling world
//! map. The rules, timings and screens follow the original routine by routine (the names in the comments are the
//! author's own routine names, mapped in docs/native/g296-native.md); the state is ordinary typed Rust and the picture,
//! sound and input go through native_engine.
//!
//! Timing: one `tick` is one 50 Hz frame. It runs what the original did in its vertical-blank handler (latch the
//! scroll and pointer for this frame's picture, the background flash, the mouse, the music or the end of finished
//! sound effects), then the main program until it next waited for a frame. The main program's loops are the `Phase`
//! states; where the original waited N frames (palette fades, typed text, the mushroom cloud) the state counts them.
//!
//! Two pieces of the original's timing come from how long its work took, and are kept as measured on the reference
//! (docs/native/g296-native.md): clearing or redrawing the whole world picture (clearmap, printmap) took 6 frames, and
//! a new war's set-up took 6 frames plus the time to place the objects; and while the title music plays, a frame that
//! starts a new row of the tune left the main program no time that frame (its waits for the top of the picture miss it),
//! so typed text and the options screen pause on those frames.

use std::collections::VecDeque;

use native_engine::audio::{Block, Mixer, Pan};
use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{rgb12, Bitmap, Frame, Rect, TextMode};
use native_engine::input::Action;
use native_engine::tracker::Tracker;

use crate::assets::{Assets, Button, Count, Handler};

pub const SCREEN_W: usize = 320;
pub const SCREEN_H: usize = 256;
pub const HUD_H: i32 = 50;
pub const WORLD_W: usize = 576;
pub const WORLD_H: usize = 206;
/// Objects a side can have (70 records a side).
pub const RECORDS: usize = 70;
/// Frames the original spent clearing or redrawing the whole world picture (clearmap, printmap), measured.
pub const WHOLE_MAP_FRAMES: u32 = 6;
/// The war-over screen after a missile won the war: the hit test and the win test, then clearing the map (measured).
pub const WAROVER_AFTER_HIT_FRAMES: u32 = 7;
/// Frames a new war's set-up took besides drawing the map: placing the objects (per attempt, from the reference's
/// set-up time) and the rest (the object tables cleared, the markers drawn).
pub const SETUP_FRAMES_PER_ATTEMPT: f32 = 0.013;
pub const SETUP_FRAMES_BASE: f32 = 1.0;
/// The first effect voice (the music player has voices 0-3).
const SFX_VOICE: usize = 4;
const PANS: [Pan; 4] = [-1.0, 1.0, 1.0, -1.0];
/// Text is written into colour bit 0 (one bit-layer of the picture).
const TEXT_LAYER: TextMode = TextMode::Layer(1);
/// The missile trail sets every colour bit (colour 31).
const TRAIL_BITS: u8 = 0x1F;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Usa = 0,
    Ussr = 1,
}

impl Side {
    fn other(self) -> Side {
        match self {
            Side::Usa => Side::Ussr,
            Side::Ussr => Side::Usa,
        }
    }
}

/// One object on the map (a record of usapics / ussrpics).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Obj {
    /// 0 destroyed; America 1 silo, 2 submarine, 3 factory, 4 city; Russia 5-8 in the same order.
    pub kind: u8,
    pub x: i32,
    pub y: i32,
    /// Screen x (world x less the scroll), kept up to date as the map scrolls.
    pub sx: i32,
    pub missiles: i32,
    /// Factory: turns left to the next delivery.
    pub countdown: i32,
    /// Factory: the two silos it supplies (indices in its side's table).
    pub supplies: (usize, usize),
    pub elev: i32,
    pub head: i32,
    pub name: String,
}

impl Obj {
    fn alive(&self) -> bool {
        self.kind != 0
    }
    fn launcher(&self) -> bool {
        matches!(self.kind, 1 | 2 | 5 | 6)
    }
}

/// The game options (oncevars, the options screen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opts {
    pub silos: i32,
    pub subs: i32,
    pub factories: i32,
    pub cities: i32,
    /// Factory rate: turns between deliveries.
    pub rate: i32,
    pub map: usize,
    pub trails: bool,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            silos: 10,
            subs: 3,
            factories: 5,
            cities: 14,
            rate: 2,
            map: 0,
            trails: true,
        }
    }
}

/// The map scroll: whole 16-pixel steps and the fine delay (mapaddr and scwait).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct View {
    pub coarse: i32,
    pub fine: i32,
}

impl View {
    /// The world x at the left edge of the map area.
    pub fn left(&self) -> i32 {
        self.coarse * 16 - self.fine
    }
}

/// The missile in flight, in pixels x 1024 (icbmx .. gravfall).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub alt: i32,
    pub alt2: i32,
    pub dist: i32,
    pub fall: i32,
}

/// Text typed one character a frame (write12 with its wait flag set): jobs in order, each (text, x, y).
#[derive(Debug, Clone, PartialEq)]
struct Typing {
    jobs: VecDeque<(String, i32, i32)>,
    pos: usize,
    then: AfterTyping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AfterTyping {
    /// The options screen: print the values, then handle its buttons.
    Options,
    /// The instructions: show page 0 and wait for a click.
    Instructions,
    /// The war-over screen: wait for a click.
    WarOver,
}

#[derive(Debug, Clone, PartialEq)]
enum Phase {
    /// The first frame: start-up, then the intro.
    Boot,
    /// intro, fading to white: frames left in this step's delay.
    FadeWhite(u8),
    /// intro, fading back to the colours: the first step (after the map is cleared), then frames left in a step.
    FadeBackStart,
    FadeBack(u8),
    /// titlescreen: rows of the logo shown so far.
    Title(i32),
    TitleClick,
    /// The options, instructions and war-over screens: the frame drawn (after the map is cleared), then typing.
    OptionsDraw,
    InstructionsDraw,
    WarOverDraw,
    Typing(Box<Typing>),
    Options,
    /// instructs: the page shown, waiting for a click.
    Instructions(usize),
    /// initstuff: the clean map drawn, then the objects and the HUD (when the original's set-up had finished).
    WarMap(u32),
    WarReady,
    /// usaloop / ussrloop.
    Turn,
    /// icbmloop.
    Flight,
    /// mushroom: the explosion at (x, y), frames drawn so far.
    Explosion {
        x: i32,
        y: i32,
        drawn: usize,
    },
    /// After the map was redrawn at the end of a flight (trails off): the win test and the next turn.
    AfterFlight,
    WarOverClick,
}

pub struct Gtw {
    a: Assets,
    phase: Phase,
    /// The vertical-blank handler is running (installed at start-up).
    started: bool,
    // ---- the picture
    hud: Bitmap,
    world: Bitmap,
    /// The game's colour table (coltable), faded in place by the intro.
    colours: [u16; 32],
    /// The colours on screen (copycols copies the table; the flash writes colour 0).
    shown_colours: [u16; 32],
    view: View,
    /// The scroll and pointer this frame shows (latched at the frame's start).
    shown_view: View,
    shown_pointer: (i32, i32),
    flash_col: i32,
    flash_dec: i32,
    flash_count: i32,
    /// The intro's fade step: entries already at the target when the step began (d6).
    fade_done: usize,
    // ---- the pointer and buttons (screen coordinates)
    px: i32,
    py: i32,
    lmb: bool,
    rmb: bool,
    cancel: bool,
    waitclik_latch: bool,
    clix2_latch: bool,
    siloclick_latch: bool,
    clix_repeat: i32,
    // ---- options
    pub opts: Opts,
    /// The map currently loaded (LOAD loads the one chosen).
    loaded_map: usize,
    seed: u16,
    // ---- a war
    pub usa: Vec<Obj>,
    pub ussr: Vec<Obj>,
    hq: [usize; 2],
    side: Side,
    cur: (Side, usize),
    last: [usize; 2],
    pub elev: i32,
    pub head: i32,
    pub missiles: i32,
    flasher: i32,
    animspeed: i32,
    pub score: [i64; 2],
    winflag: bool,
    exitflag: bool,
    win_text: String,
    win_reason: usize,
    pub icbm: Missile,
    gravrate: i32,
    musicflag: bool,
    sfx_chan: i32,
    /// The map under the mushroom cloud (mushbg), saved when the missile lands.
    mushbg: Bitmap,
    /// Frames the main program is still busy with work started earlier (see the module notes).
    busy: u32,
    /// This frame's music started a new row, so the main program's wait for the top of the picture missed it.
    overrun: bool,
    /// Placement attempts of the last set-up.
    pub attempts: u32,
    /// The war is being ended by a missile (the explosion's hit test ran this frame).
    phase_is_explosion: bool,
    /// A war has been set up (the selected object exists).
    war_started: bool,
}

/// The original's integer square root (squroot): a first guess from the top bit, then three Newton steps with
/// 16-bit signed division (a division that overflows leaves the dividend, as the processor did).
pub fn squroot(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }
    let top = 31 - n.leading_zeros();
    let half = top >> 1;
    let mut g: u16 = (half as u16) | (1u16 << half);
    for _ in 0..3 {
        let q = (n as i32).checked_div(g as i16 as i32).unwrap_or(0);
        let low = if (-32768..=32767).contains(&q) {
            q as u16
        } else {
            n as u16
        };
        g = g.wrapping_add(low) >> 1;
    }
    g as i32
}

/// convert: `n` as `digits` decimal digits by repeated subtraction (a first digit past 9 runs on in the font).
pub fn convert(mut n: i64, digits: u32) -> String {
    let mut p = 10i64.pow(digits.saturating_sub(1));
    let mut s = String::new();
    for _ in 0..digits {
        let mut c = b'0';
        while n >= p {
            n -= p;
            c = c.wrapping_add(1);
        }
        s.push(c as char);
        p = (p / 10).max(1);
    }
    s
}

impl Gtw {
    pub fn new(a: Assets, o: &Options) -> Gtw {
        let seed = o.seed.map(|s| s as u16).unwrap_or_else(|| {
            // oncevars seeds the generator from a free-running hardware counter; the clock does the same job here
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            (t.subsec_nanos() >> 10) as u16
        });
        let colours = a.palette;
        Gtw {
            hud: a.hud.clone(),
            world: Bitmap::new(WORLD_W, WORLD_H),
            colours,
            shown_colours: colours,
            a,
            phase: Phase::Boot,
            started: false,
            view: View::default(),
            shown_view: View::default(),
            shown_pointer: (0, 0),
            flash_col: 0,
            flash_dec: 0,
            flash_count: 2,
            fade_done: 0,
            px: -128,
            py: -44,
            lmb: false,
            rmb: false,
            cancel: false,
            waitclik_latch: false,
            clix2_latch: false,
            siloclick_latch: false,
            clix_repeat: 2,
            opts: Opts::default(),
            loaded_map: 0,
            seed,
            usa: vec![Obj::default(); RECORDS],
            ussr: vec![Obj::default(); RECORDS],
            hq: [0, 0],
            side: Side::Usa,
            cur: (Side::Usa, 0),
            last: [0, 0],
            elev: 0,
            head: 0,
            missiles: 0,
            flasher: 0,
            animspeed: 0,
            score: [0, 0],
            winflag: false,
            exitflag: false,
            win_text: String::new(),
            win_reason: 3,
            icbm: Missile::default(),
            gravrate: 17,
            musicflag: false,
            sfx_chan: 3,
            mushbg: Bitmap::new(32, 9),
            busy: 0,
            overrun: false,
            attempts: 0,
            phase_is_explosion: false,
            war_started: false,
        }
    }

    // ================================================================ the frame start (vbgint)
    fn vertical_blank(&mut self, cx: &mut Ctx) {
        // pointmap, pointwait, printpoint: this frame shows the scroll and pointer as they are now
        self.shown_view = self.view;
        self.shown_pointer = (self.px, self.py);
        self.flash();
        self.mousemove(cx);
        self.overrun = false;
        if self.musicflag {
            self.overrun = cx.audio.tick();
        } else {
            self.soundoff(cx.audio);
        }
    }

    /// The rest of the current routine runs `frames` frames from now (the original was busy that long); 0 = at once.
    fn spend(&mut self, frames: u32, then: Phase) {
        self.phase = then;
        self.busy = frames.saturating_sub(1);
    }

    /// flash: after an explosion the background colour steps down from white every second frame.
    fn flash(&mut self) {
        if self.flash_col <= 0 {
            return;
        }
        self.flash_count -= 1;
        if self.flash_count != 0 {
            return;
        }
        self.flash_count = 2;
        let c = self.flash_col - self.flash_dec;
        if c < 0 {
            return;
        }
        self.shown_colours[0] = c as u16;
        self.flash_col = c;
    }

    /// mousemove: the pointer follows the mouse (at most 128 pixels a frame each way) and stays on the screen; a touch
    /// places it directly.
    fn mousemove(&mut self, cx: &Ctx) {
        let (dx, dy) = cx.input.motion;
        if let Some((x, y)) = cx.input.absolute {
            self.px = x;
            self.py = y;
        }
        self.px += dx.clamp(-127, 128);
        self.py += dy.clamp(-127, 128);
        if self.py > 255 && self.py <= 512 {
            self.py = 255;
        }
        if self.py < 0 && self.py > -256 {
            self.py = 0;
        }
        if self.px > 319 && self.px <= 576 {
            self.px = 319;
        }
        if self.px < 0 && self.px > -257 {
            self.px = 0;
        }
    }

    /// soundoff: with no music, a finished sound effect is followed by silence on every voice (its first voice's test
    /// reads a bit that is always clear, which happens to silence that voice too), so every effect plays once:
    /// `playsfx` starts them that way and nothing is left to do each frame.
    fn soundoff(&mut self, _m: &mut Mixer) {}

    /// playsfx: sound effect `id` on the next voice, round robin 3, 2, 1, 0.
    fn playsfx(&mut self, id: usize, m: &mut Mixer) {
        self.sfx_chan -= 1;
        if self.sfx_chan < 0 {
            self.sfx_chan = 3;
        }
        let ch = self.sfx_chan as usize;
        let s = &self.a.sfx[id];
        m.start(
            SFX_VOICE + ch,
            Block::whole(&s.sample),
            Block::silence(),
            s.sample.rate,
            s.volume,
            PANS[ch],
        );
    }

    fn silence_effects(m: &mut Mixer) {
        for v in SFX_VOICE..SFX_VOICE + 4 {
            m.stop(v);
        }
    }

    // ================================================================ drawing
    /// The map area of the world: drawn from (0, 0), clipped.
    fn world_rect(&self) -> Rect {
        self.world.rect()
    }

    /// printslab: a picture (or one frame of a sheet) copied whole.
    fn slab(dst: &mut Bitmap, src: &Bitmap, s: Rect, x: i32, y: i32) {
        let clip = dst.rect();
        dst.blit(src, s, x, y, clip);
    }

    /// printblock: map marker `frame` at an object, cut out by its mask.
    fn marker(&mut self, frame: usize, o_x: i32, o_y: i32) {
        let r = self.a.pieces.frame(frame);
        let mask = &self.a.pieces_mask;
        let clip = self.world_rect();
        let (img, msk) = (&self.a.pieces.image, &mask.image);
        self.world
            .blit_with(img, r, o_x - 3, o_y - 3, clip, |(sx, sy), p, _| {
                (msk.get(sx, sy) != 0).then_some(p)
            });
    }

    fn obj(&self, s: Side, i: usize) -> &Obj {
        match s {
            Side::Usa => &self.usa[i],
            Side::Ussr => &self.ussr[i],
        }
    }

    fn obj_mut(&mut self, s: Side, i: usize) -> &mut Obj {
        match s {
            Side::Usa => &mut self.usa[i],
            Side::Ussr => &mut self.ussr[i],
        }
    }

    fn table(&self, s: Side) -> &Vec<Obj> {
        match s {
            Side::Usa => &self.usa,
            Side::Ussr => &self.ussr,
        }
    }

    /// printmap: the clean map into the world picture.
    fn printmap(&mut self) {
        let m = &self.a.maps[self.loaded_map];
        self.world.px.copy_from_slice(&m.px);
    }

    /// clearmap: the world picture blank and the scroll back to the left.
    fn clearmap(&mut self) {
        self.world.fill(0);
        self.view = View::default();
    }

    /// prtsilo: every live object's marker.
    fn prtsilo(&mut self) {
        for s in [Side::Usa, Side::Ussr] {
            for i in 0..RECORDS {
                let o = self.obj(s, i).clone();
                if o.alive() {
                    let f = self.a.marker_frames[o.kind as usize - 1];
                    self.marker(f, o.x, o.y);
                }
            }
        }
    }

    /// One character of the big font into the world picture (write12) or the HUD (write).
    fn big_char(dst: &mut Bitmap, a: &Assets, c: char, x: i32, y: i32) {
        a.big.draw_cell(dst, x, y, a.big.glyph(c), TEXT_LAYER);
    }

    fn big_text(dst: &mut Bitmap, a: &Assets, s: &str, x: i32, y: i32) {
        a.big.draw(dst, x, y, s, TEXT_LAYER);
    }

    fn mini_text(dst: &mut Bitmap, a: &Assets, s: &str, x: i32, y: i32) {
        a.mini.draw(dst, x, y, s, TEXT_LAYER);
    }

    // ================================================================ the HUD
    /// flashsilo: the selected marker blinks, 10 frames lit, 10 plain.
    fn flashsilo(&mut self) {
        self.flasher -= 1;
        if self.flasher == 0 {
            self.flasher = 20;
        }
        let frames = if self.flasher > 10 {
            &self.a.flash_frames_b
        } else {
            &self.a.flash_frames_a
        };
        let o = self.obj(self.cur.0, self.cur.1).clone();
        if o.alive() {
            let f = frames[o.kind as usize - 1];
            self.marker(f, o.x, o.y);
        }
    }

    /// showinfo: select an object: its aim is kept in its record; its name, kind, photo, missiles and aim are shown.
    fn showinfo(&mut self, s: Side, i: usize) {
        self.flasher = 2;
        self.flashsilo();
        let (cs, ci) = self.cur;
        let (e, h) = (self.elev, self.head);
        let old = self.obj_mut(cs, ci);
        old.elev = e;
        old.head = h;
        self.flasher = 1;
        self.cur = (s, i);
        let o = self.obj(s, i).clone();
        self.missiles = o.missiles;
        self.elev = o.elev;
        self.head = o.head;
        Self::mini_text(&mut self.hud, &self.a, &o.name, 80, 12);
        let kind = (o.kind as usize).max(1) - 1;
        let caption = self.a.type_names.get(kind).cloned().unwrap_or_default();
        Self::mini_text(&mut self.hud, &self.a, &caption, 80, 6);
        let photo = if o.kind > 4 { o.kind - 4 } else { o.kind } as usize;
        let r = self.a.photos.frame(photo.max(1) - 1);
        Self::slab(&mut self.hud, &self.a.photos.image, r, 0, 3);
        self.prtammo();
        self.prtelev();
    }

    /// prtammo: the missile count picture.
    fn prtammo(&mut self) {
        let r = self.a.ammo.frame(self.missiles.clamp(0, 3) as usize);
        Self::slab(&mut self.hud, &self.a.ammo.image, r, 80, 25);
    }

    /// prtelev: elevation and heading as numbers.
    fn prtelev(&mut self) {
        Self::big_text(
            &mut self.hud,
            &self.a,
            &convert(self.elev as i64, 2),
            184,
            7,
        );
        Self::big_text(
            &mut self.hud,
            &self.a,
            &convert(self.head as i64, 3),
            184,
            29,
        );
    }

    /// showturn: whose turn it is.
    fn showturn(&mut self) {
        let r = self.a.side.frame(self.side as usize);
        Self::slab(&mut self.hud, &self.a.side.image, r, 256, 3);
    }

    // ================================================================ buttons
    /// clix2: a new press of the left button on a box runs its handler.
    fn clix2(&mut self, table: &[Button], m: &mut Mixer) -> Option<Handler> {
        if !self.lmb {
            self.clix2_latch = false;
            return None;
        }
        if self.clix2_latch {
            return None;
        }
        self.clix2_latch = true;
        let b = table.iter().find(|b| b.contains(self.px, self.py))?;
        self.handle(b.handler, m);
        Some(b.handler)
    }

    /// clix: while the right button is held, the box under the pointer repeats every second frame.
    fn clix(&mut self, table: &[Button], m: &mut Mixer) {
        if !self.rmb {
            self.clix_repeat = 2;
            return;
        }
        self.clix_repeat -= 1;
        if self.clix_repeat != 0 {
            return;
        }
        self.clix_repeat = 2;
        if let Some(b) = table.iter().find(|b| b.contains(self.px, self.py)) {
            self.handle(b.handler, m);
        }
    }

    /// waitclik: true on a new press of the left button.
    fn waitclik(&mut self) -> bool {
        if !self.lmb {
            self.waitclik_latch = false;
            return false;
        }
        if self.waitclik_latch {
            return false;
        }
        self.waitclik_latch = true;
        true
    }

    fn handle(&mut self, h: Handler, m: &mut Mixer) {
        use Handler::*;
        let o = &mut self.opts;
        match h {
            IncElev => {
                if self.elev != 90 {
                    self.elev += 1;
                    self.prtelev();
                }
            }
            DecElev => {
                if self.elev != 0 {
                    self.elev -= 1;
                    self.prtelev();
                }
            }
            IncHead => {
                self.head += 1;
                if self.head == 360 {
                    self.head = 0;
                }
                self.prtelev();
            }
            DecHead => {
                self.head -= 1;
                if self.head < 0 {
                    self.head = 359;
                }
                self.prtelev();
            }
            SetElev | SetHead => {
                // the four 10-pixel cells from x 140 (divu of a value below 140 never happens: the box starts there)
                let cell = ((self.px - 140).max(0) / 10).min(3) as usize;
                if h == SetElev {
                    self.elev = self.a.elev_presets[cell];
                } else {
                    self.head = self.a.head_presets[cell];
                }
                self.prtelev();
            }
            Peace => {
                self.exitflag = true;
                self.winflag = true;
                self.win_text = self.a.text("adraw").to_string();
                self.win_reason = 3;
            }
            Fire => self.fired(m),
            ClearTrails => {
                self.printmap();
                self.prtsilo();
                self.busy = WHOLE_MAP_FRAMES - 1;
            }
            AddSub => {
                if o.subs != 10 {
                    o.subs += 1;
                    self.prtvals();
                }
            }
            SubSub => {
                if o.subs != 1 {
                    o.subs -= 1;
                    self.prtvals();
                }
            }
            AddSilo => {
                if o.silos != 20 {
                    o.silos += 2;
                    o.factories += 1;
                    self.prtvals();
                }
            }
            SubSilo => {
                if o.silos != 2 {
                    o.silos -= 2;
                    o.factories -= 1;
                    self.prtvals();
                }
            }
            AddCity => {
                if o.cities != 30 {
                    o.cities += 1;
                    self.prtvals();
                }
            }
            SubCity => {
                if o.cities != 1 {
                    o.cities -= 1;
                    self.prtvals();
                }
            }
            AddRate => {
                if o.rate != 8 {
                    o.rate += 1;
                    self.prtvals();
                }
            }
            SubRate => {
                if o.rate != 1 {
                    o.rate -= 1;
                    self.prtvals();
                }
            }
            Trails => {
                o.trails = !o.trails;
                self.prtvals();
            }
            AddMap => {
                if o.map != 2 {
                    o.map += 1;
                    self.prtvals();
                }
            }
            SubMap => {
                if o.map != 0 {
                    o.map -= 1;
                    self.prtvals();
                }
            }
            LoadMap => self.loaded_map = self.opts.map,
            Defaults => {
                *o = Opts {
                    map: o.map,
                    trails: o.trails,
                    ..Opts::default()
                };
                self.prtvals();
            }
            Instructions => self.start_instructions(),
            Play => self.exitflag = true,
        }
    }

    // ================================================================ the screens
    /// fadetowhite / fadetocols: one step of every colour towards the target; returns the entries already there.
    fn fade_step(&mut self, to_white: bool) -> usize {
        let mut done = 0;
        for i in 0..32 {
            let c = self.colours[i];
            let t = if to_white { 0x0FFF } else { self.a.palette[i] };
            if c == t {
                done += 1;
                continue;
            }
            let step = |sh: u16| {
                let (v, tv) = ((c >> sh) & 15, (t >> sh) & 15);
                let nv = if v == tv {
                    v
                } else if to_white {
                    v + 1
                } else {
                    v.wrapping_sub(1) & 15
                };
                nv << sh
            };
            self.colours[i] = step(8) | step(4) | step(0);
        }
        self.shown_colours = self.colours;
        done
    }

    /// titlescreen, start: the music, a blank map.
    fn start_title(&mut self, m: &mut Mixer) {
        if let Ok(t) = Tracker::new(&self.a.music, self.a.tracker.clone()) {
            Self::silence_effects(m);
            m.music_play(t);
        }
        self.musicflag = true;
        self.clearmap();
        self.spend(WHOLE_MAP_FRAMES, Phase::Title(0));
    }

    /// titlescreen: the logo grows up from its bottom line, one more row a frame.
    fn title_row(&mut self) {
        let Phase::Title(n) = self.phase else { return };
        let n = n + 1;
        let r = Rect::new(0, 0, self.a.title.w as i32, n);
        Self::slab(&mut self.world, &self.a.title, r, 32, 132 - n);
        self.phase = if n >= self.a.title.h as i32 {
            Phase::TitleClick
        } else {
            Phase::Title(n)
        };
    }

    /// optsscrn: the frame, then the title typed.
    fn start_options(&mut self) {
        self.exitflag = false;
        self.clearmap();
        self.spend(WHOLE_MAP_FRAMES, Phase::OptionsDraw);
    }

    fn draw_options(&mut self) {
        Self::slab(
            &mut self.world,
            &self.a.options,
            self.a.options.rect(),
            32,
            30,
        );
        self.type_then(
            vec![(self.a.text("opttitle").to_string(), 104, 10)],
            AfterTyping::Options,
        );
    }

    /// prtvals: every option value.
    fn prtvals(&mut self) {
        let o = self.opts;
        for (v, y) in [(o.subs, 36), (o.silos, 56), (o.cities, 76), (o.rate, 96)] {
            Self::big_text(&mut self.world, &self.a, &convert(v as i64, 2), 192, y);
        }
        let t = if o.trails {
            self.a.text("tron")
        } else {
            self.a.text("troff")
        }
        .to_string();
        Self::big_text(&mut self.world, &self.a, &t, 192, 116);
        let name = self.a.map_names[o.map].clone();
        Self::big_text(&mut self.world, &self.a, &name, 80, 136);
    }

    /// instructs: the frame and title, then pages of the mini font.
    fn start_instructions(&mut self) {
        self.exitflag = false;
        self.clearmap();
        self.spend(WHOLE_MAP_FRAMES, Phase::InstructionsDraw);
    }

    fn draw_instructions(&mut self) {
        Self::slab(&mut self.world, &self.a.frame, self.a.frame.rect(), 32, 30);
        self.type_then(
            vec![(self.a.text("institle").to_string(), 104, 10)],
            AfterTyping::Instructions,
        );
    }

    fn instructions_page(&mut self, p: usize) {
        let lines = self.a.instructions.get(p).cloned().unwrap_or_default();
        for (k, l) in lines.iter().enumerate() {
            Self::mini_text(&mut self.world, &self.a, l, 48, 40 + 7 * k as i32);
        }
        self.phase = Phase::Instructions(p);
    }

    fn type_then(&mut self, jobs: Vec<(String, i32, i32)>, then: AfterTyping) {
        self.phase = Phase::Typing(Box::new(Typing {
            jobs: jobs.into(),
            pos: 0,
            then,
        }));
    }

    /// warover: the frame, then the result typed.
    fn start_warover(&mut self) {
        let frames = if self.phase_is_explosion {
            WAROVER_AFTER_HIT_FRAMES
        } else {
            WHOLE_MAP_FRAMES
        };
        self.clearmap();
        self.spend(frames, Phase::WarOverDraw);
    }

    fn draw_warover(&mut self) {
        Self::slab(&mut self.world, &self.a.frame, self.a.frame.rect(), 32, 30);
        let (r1, r2) = self.a.reasons[self.win_reason].clone();
        let jobs = vec![
            (self.a.text("endtitle").to_string(), 96, 10),
            (self.win_text.clone(), 48, 35),
            (r1, 48, 65),
            (r2, 48, 80),
            (self.a.text("usascored").to_string(), 48, 110),
            (convert(self.score[0], 5), 176, 110),
            (self.a.text("ussrscored").to_string(), 48, 125),
            (convert(self.score[1], 5), 176, 125),
        ];
        self.type_then(jobs, AfterTyping::WarOver);
    }

    // ================================================================ a war
    /// random: the original generator: seed = seed x $24A1 + $24DF; a number from 0 to `n`.
    fn random(&mut self, n: i32) -> i32 {
        self.seed = self.seed.wrapping_mul(0x24A1).wrapping_add(0x24DF);
        (self.seed & 0x7FFF) as i32 % (n + 1)
    }

    /// initstuff (with initvars, clearsilo, setup, esthq): a new war.
    fn start_war(&mut self) {
        self.war_started = true;
        // initvars (its long write clears the pointer's x too: the pointer jumps to the left edge)
        self.score = [0, 0];
        self.view = View::default();
        self.px = -128;
        self.exitflag = false;
        self.winflag = false;
        self.elev = 0;
        self.head = 0;
        self.flasher = 1;
        self.animspeed = 5;
        self.last = [0, 0];
        self.cur = (Side::Usa, 0);
        self.usa = vec![Obj::default(); RECORDS];
        self.ussr = vec![Obj::default(); RECORDS];
        self.setup();
        // esthq: each side's secret headquarters, one of its cities
        let c = self.opts.cities - 1;
        let u = self.random(c) as usize;
        self.hq[0] += u;
        let r = self.random(c) as usize;
        self.hq[1] += r;
        // the map appears when the original had copied it, the objects and HUD when its set-up was done
        let rest =
            (SETUP_FRAMES_BASE + SETUP_FRAMES_PER_ATTEMPT * self.attempts as f32).round() as u32;
        self.spend(WHOLE_MAP_FRAMES, Phase::WarMap(rest.max(1)));
    }

    fn war_ready(&mut self) {
        self.prtsilo();
        let (s, i) = self.cur;
        self.showinfo(s, i);
        self.prtelev();
        self.side = Side::Usa;
        self.start_turn();
    }

    /// setup: every object placed at random on land of its side's colour (submarines at sea).
    fn setup(&mut self) {
        self.attempts = 0;
        for k in 0..8 {
            let side = if k < 4 { Side::Usa } else { Side::Ussr };
            let base = if k == 0 || k == 4 {
                0
            } else {
                self.table(side)
                    .iter()
                    .position(|o| !o.alive())
                    .unwrap_or(RECORDS)
            };
            if k == 3 {
                self.hq[0] = base;
            }
            if k == 7 {
                self.hq[1] = base;
            }
            let kind = self.a.kinds[k].clone();
            let n = match kind.count {
                Count::Silos => self.opts.silos,
                Count::Subs => self.opts.subs,
                Count::Factories => self.opts.factories,
                Count::Cities => self.opts.cities,
            } as usize;
            let mut supply = 0usize;
            for j in 0..n {
                let (mut x, mut y);
                loop {
                    self.attempts += 1;
                    x = self.random(0x22B) + 8;
                    y = self.random(0xC6) + 3;
                    if self.a.maps[self.loaded_map].get(x, y) == kind.colour {
                        break;
                    }
                }
                let o = Obj {
                    kind: k as u8 + 1,
                    x,
                    y,
                    sx: x,
                    missiles: kind.missiles,
                    countdown: self.opts.rate,
                    supplies: (supply, supply + 1),
                    elev: 0,
                    head: 0,
                    name: kind.names.get(j).cloned().unwrap_or_default(),
                };
                supply += 2;
                if let Some(slot) = (base + j < RECORDS).then_some(base + j) {
                    *self.obj_mut(side, slot) = o;
                }
            }
        }
    }

    /// usaloop / ussrloop, start: the last launcher used (or the first live one), the HUD, whose turn.
    fn start_turn(&mut self) {
        let s = self.side;
        let mut i = self.last[s as usize];
        if !self.obj(s, i).alive() {
            // the first live silo or submarine (there is always one: a side without any has already lost)
            i = self.table(s).iter().position(Obj::launcher).unwrap_or(0);
        }
        self.showinfo(s, i);
        self.showturn();
        self.exitflag = false;
        self.phase = Phase::Turn;
    }

    /// scroll: the pointer held at the right or left edge moves the map 4 pixels a frame (up to 244 pixels).
    fn scroll(&mut self) {
        if self.px == 319 && self.view.coarse != 16 {
            self.onscreen(-4);
            self.view.fine -= 4;
            if self.view.fine < 0 {
                self.view.coarse += 1;
                self.view.fine &= 15;
            }
        }
        if self.px == 0 && self.view.coarse != 0 {
            self.onscreen(4);
            self.view.fine += 4;
            if self.view.fine >= 16 {
                self.view.coarse -= 1;
                self.view.fine &= 15;
            }
        }
    }

    /// onscreen: every live object's screen x moves with the map.
    fn onscreen(&mut self, d: i32) {
        for o in self
            .usa
            .iter_mut()
            .chain(self.ussr.iter_mut())
            .filter(|o| o.alive())
        {
            o.sx += d;
        }
    }

    /// siloclix: a new left press on the map selects one of this side's objects within 3 pixels.
    fn siloclix(&mut self, m: &mut Mixer) {
        if !self.lmb {
            self.siloclick_latch = false;
            return;
        }
        if self.siloclick_latch {
            return;
        }
        self.siloclick_latch = true;
        if self.py < HUD_H {
            return;
        }
        let (x, y) = (self.px, self.py - HUD_H);
        let s = self.side;
        let hit = self.table(s).iter().position(|o| {
            o.alive() && (o.sx - 3..=o.sx + 3).contains(&x) && (o.y - 3..=o.y + 3).contains(&y)
        });
        if let Some(i) = hit {
            self.playsfx(1, m);
            self.showinfo(s, i);
        }
    }

    /// fired: launch from the selected object (or the empty click when it has no missiles).
    fn fired(&mut self, m: &mut Mixer) {
        if self.missiles == 0 {
            self.playsfx(0, m);
            return;
        }
        self.playsfx(3, m);
        self.missiles -= 1;
        let (s, i) = self.cur;
        let (e, h) = (self.elev, self.head);
        let o = self.obj_mut(s, i);
        o.missiles -= 1;
        o.elev = e;
        o.head = h;
        let (x, y) = (o.x << 10, o.y << 10);
        self.icbm = Missile {
            x,
            y,
            x2: x,
            y2: y,
            ..Missile::default()
        };
        self.flasher = 2;
        self.flashsilo();
        self.prtammo();
        self.exitflag = true;
    }

    /// One frame of a turn (the body of usaloop / ussrloop).
    fn turn_frame(&mut self, m: &mut Mixer) {
        self.scroll();
        self.flashsilo();
        let rep = self.a.buttons_repeat.clone();
        self.clix(&rep, m);
        let click = self.a.buttons_click.clone();
        self.clix2(&click, m);
        self.siloclix(m);
        if self.side == Side::Usa && self.cancel {
            // ESC in America's turn leaves the war at once, back to the title (game returns to main)
            self.start_title(m);
            return;
        }
        if !self.exitflag {
            return;
        }
        self.last[self.side as usize] = self.cur.1;
        if self.winflag {
            self.start_warover();
            return;
        }
        // icbmloop: the flight starts next frame
        self.exitflag = false;
        self.phase = Phase::Flight;
    }

    /// One frame of the flight (the body of icbmloop).
    fn flight_frame(&mut self, m: &mut Mixer) {
        self.scroll();
        self.printicbm();
        // moveicbm1: climb by sin(elevation), ground speed cos(elevation) / 2
        let i = (90 - self.elev).clamp(0, 359) as usize;
        self.icbm.dist = self.a.sine[i] >> 1;
        self.icbm.alt2 += self.a.cosine[i];
        // moveicbm2: along the heading
        let h = self.head.clamp(0, 359) as usize;
        let (sn, cs) = (self.a.sine[h] as i16 as i32, self.a.cosine[h] as i16 as i32);
        let d = self.icbm.dist as i16 as i32;
        self.icbm.x2 += (sn * d) >> 7;
        self.icbm.y2 -= (cs * d) >> 7;
        // gravity
        self.icbm.alt2 -= self.icbm.fall;
        if self.icbm.alt2 > 0 {
            self.icbm.fall += self.gravrate;
        } else {
            self.land(m);
            return;
        }
        if self.exitflag {
            if self.opts.trails {
                self.end_flight(m);
            } else {
                // clip1 redrew the map when the missile left it
                self.spend(WHOLE_MAP_FRAMES, Phase::AfterFlight);
            }
        }
    }

    /// printicbm (with clip1 and clip2): the newest trail segment, the missile wrapped round the world, and the
    /// flight ended when it leaves the top or bottom of the map.
    fn printicbm(&mut self) {
        let a0 = self.icbm.alt >> 9;
        let a1 = self.icbm.alt2 >> 9;
        let (mut x0, mut y0) = (self.icbm.x >> 10, self.icbm.y >> 10);
        let (mut x1, mut y1) = (self.icbm.x2 >> 10, self.icbm.y2 >> 10);
        if (x1 as i16) < 0 {
            self.icbm.x2 += 576 << 10;
            x1 += 576;
            (x0, y0) = (x1, y1);
        }
        if (x1 as i16) > 575 {
            self.icbm.x2 -= 576 << 10;
            x1 -= 576;
            (x0, y0) = (x1, y1);
        }
        for out in [(y1 as i16) < 0, (y1 as i16) > 205] {
            if out {
                self.exitflag = true;
                if !self.opts.trails {
                    self.printmap();
                    self.prtsilo();
                    y1 = -1;
                }
            }
        }
        y0 -= a0;
        y1 -= a1;
        if (y1 as i16) >= 0 {
            let clip = self.world_rect();
            self.world
                .line_with(x0, y0, x1, y1, clip, |c| c | TRAIL_BITS);
        }
        self.icbm.x = self.icbm.x2;
        self.icbm.y = self.icbm.y2;
        self.icbm.alt = self.icbm.alt2;
    }

    /// gravity, landing: the last segment, the flash, the explosion sound, then the mushroom cloud.
    fn land(&mut self, m: &mut Mixer) {
        self.exitflag = true;
        self.printicbm();
        self.flash_col = 0x0FFF;
        self.flash_dec = 0x0111;
        let (x, y) = (self.icbm.x2 >> 10, self.icbm.y2 >> 10);
        self.playsfx(2, m);
        self.mushbg = self.save_under(x, y);
        self.phase = Phase::Explosion { x, y, drawn: 0 };
    }

    /// explode / mushroom: every fifth frame the next frame of the cloud over the saved map; the last one stays.
    fn explosion_frame(&mut self, x: i32, y: i32, drawn: usize, bg: &Bitmap, m: &mut Mixer) {
        self.animspeed -= 1;
        if self.animspeed != 0 {
            return;
        }
        self.animspeed = 5;
        let (wx, shift, wy) = (((x - 4) >> 4) << 4, (x - 4) & 15, y - 4);
        let clip = self.world_rect();
        self.world.blit(bg, bg.rect(), wx, wy, clip);
        let r = self.a.mushroom.frame(drawn);
        let (img, msk) = (&self.a.mushroom.image, &self.a.mushroom_mask.image);
        let window = Rect::new(wx, wy, 32, 9).intersect(&clip);
        self.world
            .blit_with(img, r, wx + shift, wy, window, |(sx, sy), p, _| {
                (msk.get(sx, sy) != 0).then_some(p)
            });
        let drawn = drawn + 1;
        if drawn < self.a.mushroom.frames {
            self.phase = Phase::Explosion { x, y, drawn };
            return;
        }
        self.checkhit(x, y, m);
        self.phase_is_explosion = true;
        if !self.opts.trails {
            self.printmap();
            self.prtsilo();
            self.spend(WHOLE_MAP_FRAMES, Phase::AfterFlight);
            return;
        }
        self.end_flight(m);
    }

    /// checkhit: everything within 6 pixels of the impact is destroyed; points to the victim's opponent, and the
    /// war is won by hitting the enemy headquarters (whoever fired).
    fn checkhit(&mut self, x: i32, y: i32, m: &mut Mixer) {
        for s in [Side::Usa, Side::Ussr] {
            for i in 0..RECORDS {
                let o = self.obj(s, i).clone();
                if !o.alive() {
                    continue;
                }
                let dx = ((x - o.x) as i16).unsigned_abs() as u32;
                let dy = ((y - o.y) as i16).unsigned_abs() as u32;
                let d = squroot(dx * dx + dy * dy);
                if d > 6 {
                    continue;
                }
                self.playsfx(4, m);
                let points = 400 - 50 * d as i64;
                let winner = s.other();
                self.score[winner as usize] += points;
                if self.hq[s as usize] == i {
                    self.win_text = self
                        .a
                        .text(if winner == Side::Usa {
                            "usawins"
                        } else {
                            "ussrwins"
                        })
                        .to_string();
                    self.win_reason = 0;
                    self.winflag = true;
                    self.score[winner as usize] += 500;
                }
                let f = self.a.hit_frames[o.kind as usize - 1];
                self.marker(f, o.x, o.y);
                self.obj_mut(s, i).kind = 0;
            }
        }
    }

    /// After a flight: the win test of the side that fired (usawon / ussrwon), the factories, the next turn.
    fn end_flight(&mut self, _m: &mut Mixer) {
        let s = self.side;
        if !self.winflag {
            self.won(s);
        }
        if self.winflag {
            self.start_warover();
            self.phase_is_explosion = false;
            return;
        }
        self.phase_is_explosion = false;
        self.factories(s);
        self.side = s.other();
        self.start_turn();
    }

    /// usawon / ussrwon: the enemy without launchers, or without missiles, loses.
    fn won(&mut self, s: Side) {
        let enemy = self.table(s.other());
        let launchers: Vec<&Obj> = enemy.iter().filter(|o| o.launcher()).collect();
        let reason = if launchers.is_empty() {
            2
        } else if launchers.iter().map(|o| o.missiles).sum::<i32>() == 0 {
            1
        } else {
            return;
        };
        self.win_text = self
            .a
            .text(if s == Side::Usa {
                "usawins"
            } else {
                "ussrwins"
            })
            .to_string();
        self.win_reason = reason;
        self.winflag = true;
        self.score[s as usize] += 500;
    }

    /// factories: every live factory of the side that just fired counts down; at zero its two silos get a missile
    /// each (up to 3).
    fn factories(&mut self, s: Side) {
        let rate = self.opts.rate;
        let t = match s {
            Side::Usa => &mut self.usa,
            Side::Ussr => &mut self.ussr,
        };
        for i in 0..RECORDS {
            if !matches!(t[i].kind, 3 | 7) {
                continue;
            }
            t[i].countdown -= 1;
            if t[i].countdown != 0 {
                continue;
            }
            t[i].countdown = rate;
            let (a, b) = t[i].supplies;
            for k in [a, b] {
                if let Some(o) = t.get_mut(k) {
                    if o.missiles != 3 {
                        o.missiles += 1;
                    }
                }
            }
        }
    }

    // ================================================================ the main program, one frame at a time
    fn main(&mut self, m: &mut Mixer) {
        if self.busy > 0 {
            self.busy -= 1;
            return;
        }
        // the waits for the top of the picture (typed text, the options loop) miss a frame the music overran
        if self.overrun && matches!(self.phase, Phase::Typing(_) | Phase::Options) {
            return;
        }
        match std::mem::replace(&mut self.phase, Phase::Boot) {
            Phase::Boot => {
                // initialise: options (oncevars), the first map (loadmap), the frame-start handler; then intro
                self.opts = Opts::default();
                self.loaded_map = 0;
                self.started = true;
                self.fade_done = self.fade_step(true);
                self.phase = Phase::FadeWhite(2);
            }
            Phase::FadeWhite(n) if n > 1 => self.phase = Phase::FadeWhite(n - 1),
            Phase::FadeWhite(_) => {
                if self.fade_done != 32 {
                    self.fade_done = self.fade_step(true);
                    self.phase = Phase::FadeWhite(2);
                } else {
                    self.clearmap();
                    self.spend(WHOLE_MAP_FRAMES, Phase::FadeBackStart);
                }
            }
            Phase::FadeBackStart => {
                self.fade_done = self.fade_step(false);
                self.phase = Phase::FadeBack(2);
            }
            Phase::OptionsDraw => self.draw_options(),
            Phase::InstructionsDraw => self.draw_instructions(),
            Phase::WarOverDraw => self.draw_warover(),
            Phase::WarMap(rest) => {
                self.printmap();
                self.spend(rest, Phase::WarReady);
            }
            Phase::WarReady => self.war_ready(),
            Phase::AfterFlight => self.end_flight(m),
            Phase::FadeBack(n) if n > 1 => self.phase = Phase::FadeBack(n - 1),
            Phase::FadeBack(_) => {
                if self.fade_done != 32 {
                    self.fade_done = self.fade_step(false);
                    self.phase = Phase::FadeBack(2);
                } else {
                    self.start_title(m);
                }
            }
            p @ Phase::Title(_) => {
                self.phase = p;
                self.title_row();
                self.title_click();
            }
            Phase::TitleClick => {
                self.phase = Phase::TitleClick;
                self.title_click();
            }
            Phase::Typing(mut t) => {
                let job = t.jobs.front().cloned();
                match job {
                    Some((s, x, y)) if t.pos < s.chars().count() => {
                        let c = s.chars().nth(t.pos).unwrap_or(' ');
                        let cw = self.a.big.cell().0 as i32;
                        Self::big_char(&mut self.world, &self.a, c, x + cw * t.pos as i32, y);
                        t.pos += 1;
                        self.phase = Phase::Typing(t);
                    }
                    Some(_) => {
                        t.jobs.pop_front();
                        t.pos = 0;
                        if t.jobs.is_empty() {
                            match t.then {
                                AfterTyping::Options => {
                                    self.prtvals();
                                    self.phase = Phase::Options;
                                }
                                AfterTyping::Instructions => self.instructions_page(0),
                                AfterTyping::WarOver => {
                                    self.phase = Phase::WarOverClick;
                                    self.warover_click(m);
                                }
                            }
                        } else {
                            self.phase = Phase::Typing(t);
                        }
                    }
                    None => self.phase = Phase::Options,
                }
            }
            Phase::Options => {
                self.phase = Phase::Options;
                let table = self.a.buttons_options.clone();
                let h = self.clix2(&table, m);
                if h == Some(Handler::Instructions) {
                    return;
                }
                if self.exitflag {
                    // PLAY: the music stops and the war begins
                    self.musicflag = false;
                    m.music_stop();
                    Self::silence_effects(m);
                    self.start_war();
                }
            }
            Phase::Instructions(p) => {
                if !self.waitclik() {
                    self.phase = Phase::Instructions(p);
                } else if p + 1 < self.a.instructions.len().min(6) {
                    self.instructions_page(p + 1);
                } else {
                    self.start_options();
                }
            }
            Phase::Turn => {
                self.phase = Phase::Turn;
                self.turn_frame(m);
            }
            Phase::Flight => {
                self.phase = Phase::Flight;
                self.flight_frame(m);
            }
            Phase::Explosion { x, y, drawn } => {
                self.phase = Phase::Explosion { x, y, drawn };
                let bg = std::mem::replace(&mut self.mushbg, Bitmap::new(0, 0));
                self.explosion_frame(x, y, drawn, &bg, m);
                self.mushbg = bg;
            }
            Phase::WarOverClick => {
                self.phase = Phase::WarOverClick;
                self.warover_click(m);
            }
        }
    }

    /// titlescreen's waitclik, polled from the frame the logo is complete.
    fn title_click(&mut self) {
        if self.phase == Phase::TitleClick && self.waitclik() {
            self.start_options();
        }
    }

    fn warover_click(&mut self, m: &mut Mixer) {
        if self.waitclik() {
            self.start_title(m);
        }
    }

    /// explode: the map under the cloud (two 16-pixel words wide, 9 rows) saved before the first frame.
    fn save_under(&self, x: i32, y: i32) -> Bitmap {
        let mut bg = Bitmap::new(32, 9);
        let wx = ((x - 4) >> 4) << 4;
        bg.blit(&self.world, Rect::new(wx, y - 4, 32, 9), 0, 0, bg.rect());
        bg
    }
}

impl Game for Gtw {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        if self.started {
            self.vertical_blank(cx);
        }
        self.lmb = cx.input.held(Action::Primary);
        self.rmb = cx.input.held(Action::Secondary);
        self.cancel = cx.input.held(Action::Cancel);
        self.main(cx.audio);
        Flow::Continue
    }

    /// The picture: the HUD over the top 50 lines, the world map scrolled below it, the pointer on top.
    fn render(&self, f: &mut Frame) {
        for (i, c) in self.shown_colours.iter().enumerate() {
            f.palette.colours[i] = rgb12(*c);
        }
        let img = &mut f.image;
        let hud = HUD_H as usize * SCREEN_W;
        img.px[..hud].copy_from_slice(&self.hud.px[..hud]);
        let left = self.shown_view.left();
        for y in 0..WORLD_H {
            let row =
                &mut img.px[(HUD_H as usize + y) * SCREEN_W..(HUD_H as usize + y + 1) * SCREEN_W];
            for (x, p) in row.iter_mut().enumerate() {
                *p = self.world.get(left + x as i32, y as i32);
            }
        }
        let (px, py) = self.shown_pointer;
        let clip = img.rect();
        img.blit_keyed(&self.a.pointer, 0, self.a.pointer.rect(), px, py, clip);
    }

    fn state(&self) -> Vec<(String, i64)> {
        let mut v: Vec<(String, i64)> = vec![
            ("phase".into(), self.phase_id()),
            ("pointx".into(), self.px as i64),
            ("pointy".into(), self.py as i64),
            ("coarse".into(), self.view.coarse as i64),
            ("scwait".into(), self.view.fine as i64),
            ("curelev".into(), self.elev as i64),
            ("curhead".into(), self.head as i64),
            ("curammo".into(), self.missiles as i64),
            ("usascore".into(), self.score[0]),
            ("ussrscore".into(), self.score[1]),
            ("musicflag".into(), self.musicflag as i64),
            ("winflag".into(), self.winflag as i64),
            ("trailflag".into(), if self.opts.trails { 255 } else { 0 }),
            ("numsilos".into(), self.opts.silos as i64),
            ("numsubs".into(), self.opts.subs as i64),
            ("numfacts".into(), self.opts.factories as i64),
            ("numcities".into(), self.opts.cities as i64),
            ("shipspeed".into(), self.opts.rate as i64),
            ("mapnum".into(), self.opts.map as i64),
            ("seed".into(), self.seed as i64),
            ("side".into(), self.side as i64),
            (
                "cursilo".into(),
                if self.war_started {
                    (self.cur.0 as i64) * RECORDS as i64 + self.cur.1 as i64
                } else {
                    -1
                },
            ),
            ("icbmx2".into(), self.icbm.x2 as i64),
            ("icbmy2".into(), self.icbm.y2 as i64),
            ("icbmalt2".into(), self.icbm.alt2 as i64),
            ("flashcol".into(), self.flash_col as i64),
            ("col0".into(), self.shown_colours[0] as i64),
        ];
        let alive = |t: &Vec<Obj>| t.iter().filter(|o| o.alive()).count() as i64;
        v.push(("usa_alive".into(), alive(&self.usa)));
        v.push(("ussr_alive".into(), alive(&self.ussr)));
        v.push(("attempts".into(), self.attempts as i64));
        v.push(("busy".into(), self.busy as i64));
        v
    }
}

impl Gtw {
    fn phase_id(&self) -> i64 {
        match self.phase {
            Phase::Boot => 0,
            Phase::FadeWhite(_) => 1,
            Phase::FadeBack(_) | Phase::FadeBackStart => 2,
            Phase::OptionsDraw | Phase::InstructionsDraw | Phase::WarOverDraw => 5,
            Phase::WarMap(_) | Phase::WarReady | Phase::AfterFlight => 12,
            Phase::Title(_) => 3,
            Phase::TitleClick => 4,
            Phase::Typing(_) => 5,
            Phase::Options => 6,
            Phase::Instructions(_) => 7,
            Phase::Turn => 8,
            Phase::Flight => 9,
            Phase::Explosion { .. } => 10,
            Phase::WarOverClick => 11,
        }
    }

    /// The objects of a side (verification and tests).
    pub fn objects(&self, s: Side) -> &[Obj] {
        self.table(s)
    }
}
