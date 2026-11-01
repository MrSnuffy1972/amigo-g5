//! E-Motion's main program state and its frame: what the original's vertical-blank handler did each frame (the frame
//! counter, palette fades, the controllers, the bonus-level clock), the two playfield screens and how a frame is shown
//! (the playfield's colour gradient, the pictures), and the four sound channels.

use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::audio::{Block, Sample};
use native_engine::gfx::{rgb12, Argb};
use native_engine::input::{Action, InputState};

use crate::assets::{Assets, H, PH, PW, SAMPLE_RATE, TOP_LINE, W};
use crate::co::{FrameIn, FrameOut, Snd, Stop};

/// Controller bits as the program reads them: up, down, left, right, fire.
pub const UP: u8 = 1;
pub const DOWN: u8 = 2;
pub const LEFT: u8 = 4;
pub const RIGHT: u8 = 8;
pub const FIRE: u8 = 0x80;

/// Key codes the program acts on (the Amiga keyboard's raw codes, as the original compared them).
pub const KEY_ESC: u8 = 0x45;
pub const KEY_P: u8 = 0x19;
pub const KEY_RETURN: u8 = 0x44;

/// How the frame is shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Display {
    /// The playfield: colours 30 and 31 are the two colours of the gradient band each 14 lines (the copper list).
    Game,
    /// A picture in 32 colours (the title picture).
    Picture,
    /// A picture the intro composed itself.
    Composed,
}

/// One object (28 bytes in the original's table of 64).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Obj {
    /// What it is (0 free, 4 ship, 8 exploding ship, $C, $10 tether, $14 ball, $18 fragment, $1C dying, $20 pod,
    /// $24, $28 hatching pod, $2C hatching ball, $30 firework, $34 bonus spark); the table ends with $FFFF.
    pub kind: u16,
    /// Position, 16.16 fixed point (a tether keeps its two ends' offsets in the two halves of `x`).
    pub x: i32,
    pub y: i32,
    pub size: i16,
    /// Colour ($00, $10, $20), or for a ship which player ($FFFF one, $FFFE two).
    pub colour: u16,
    pub vx: i32,
    pub vy: i32,
    /// The ship's angle; a ball's fuse; for an exploding ship the high half of its spread.
    pub w22: u16,
    pub w24: u16,
    pub max: u16,
}

impl Obj {
    pub fn xi(&self) -> i16 {
        (self.x >> 16) as i16
    }
    pub fn yi(&self) -> i16 {
        (self.y >> 16) as i16
    }
    pub fn l22(&self) -> u32 {
        (self.w22 as u32) << 16 | self.w24 as u32
    }
    pub fn set_l22(&mut self, v: u32) {
        self.w22 = (v >> 16) as u16;
        self.w24 = v as u16;
    }
}

pub const OBJS: usize = 64;

/// The colour lists the fades read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pal {
    Game,
    Title,
    Boot,
    Hw,
    Picture,
}

pub struct Emo {
    pub a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    pending: Option<FrameOut>,
    pub sounds: Vec<Snd>,
    pub save_out: Option<Vec<u8>>,
    pub saved: Option<Vec<u8>>,
    pub quit: bool,
    // ---- the vertical-blank handler's state
    pub frame: u32,
    pub input: InputState,
    pub joy1: u8,
    pub joy2: u8,
    pub keys: VecDeque<u8>,
    pub display: Display,
    pub composed: Vec<Argb>,
    pub hw: [u16; 32],
    /// Which colour list the fades use ($442).
    pub pal_sel: Pal,
    /// The playfield's colour list (+$7E38; colours 0 to 23 are rebuilt as ramps for each level).
    pub pal_game: Vec<u16>,
    pub pic_cmap: Vec<u16>,
    pub all32: bool,
    pub fade_level: u16,
    pub fade_dir: i16,
    pub band_base: [u16; 15],
    pub bands: [u16; 15],
    pub diw_top: i32,
    pub clock: [u8; 4],
    pub muted: bool,
    pub music: bool,
    // ---- screens: two playfield buffers and the clean copy (indexed, 320 x 196)
    pub screens: [Vec<u8>; 2],
    pub drawbuf: usize,
    pub shown: usize,
    pub bg: Vec<u8>,
    pub last_swap: u16,
    // ---- sound channels
    pub effects_on: bool,
    pub ch_free_at: [u32; 4],
    pub ch_rr: u16,
    // ---- the game
    pub rng: [u32; 2],
    pub objs: [Obj; OBJS + 1],
    pub level: u16,
    pub lives: u16,
    pub energy: i16,
    pub energy_due: i16,
    pub score: [u8; 6],
    pub bonus_score: [u8; 6],
    pub mode: u8,
    pub options: [u8; 6],
    pub turn_rate: u16,
    pub high: Vec<Vec<u8>>,
    pub high_loaded: bool,
    pub cheat_pos: usize,
    pub cheat_on: bool,
    pub prev1: u8,
    pub prev2: u8,
    pub demo_in: u8,
    pub demo_prev: u8,
    pub f404: bool,
    pub f405: bool,
    pub f406: bool,
    pub kill_touch: bool,
    pub pods4: bool,
    pub wrapped: bool,
    pub turned_right: bool,
    pub pod_count: u8,
    pub secret: u16,
    pub over_timer: u16,
    pub secret_timer: u16,
    pub banner_timer: u16,
    pub secret_level: u16,
    pub hit_list: Vec<usize>,
    pub spark_timer: u16,
    pub exploding: bool,
    pub alive: u16,
    pub pod_time: u16,
    pub bonus: u16,
    pub chain: u16,
    pub pod_frame: u32,
    pub last_obj: usize,
    pub hbars: Vec<[i16; 3]>,
    pub vbars: Vec<[i16; 3]>,
    pub dirty: Vec<(usize, usize, usize, usize)>,
    pub trainer: [bool; 3],
    pub start_level: u16,
    pub start_pressed: bool,
}

impl Emo {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Emo {
        let mut rng = a.tables.seed;
        if let Some(s) = seed {
            rng[0] ^= s;
        }
        let high = a.texts.high_scores.clone();
        let mut objs = [Obj::default(); OBJS + 1];
        let w = &a.tables.objects_init;
        for (i, o) in objs.iter_mut().enumerate() {
            let g = |k: usize| *w.get(i * 14 + k).unwrap_or(&0);
            let l = |k: usize| ((g(k) as u32) << 16 | g(k + 1) as u32) as i32;
            *o = Obj {
                kind: g(0),
                x: l(1),
                y: l(3),
                size: g(5) as i16,
                colour: g(6),
                vx: l(7),
                vy: l(9),
                w22: g(11),
                w24: g(12),
                max: g(13),
            };
        }
        let options = {
            let mut o = [0u8; 6];
            o.copy_from_slice(&a.tables.option_default[..6]);
            o
        };
        Emo {
            rx,
            tx,
            pending: None,
            sounds: Vec::new(),
            save_out: None,
            saved: None,
            quit: false,
            frame: 0,
            input: InputState::default(),
            joy1: 0,
            joy2: 0,
            keys: VecDeque::new(),
            display: Display::Composed,
            composed: vec![0xFF00_0000; W * H],
            hw: [0; 32],
            pal_sel: Pal::Hw,
            pal_game: a.tables.pal_game.clone(),
            pic_cmap: Vec::new(),
            all32: true,
            fade_level: 0,
            fade_dir: 0,
            band_base: [0; 15],
            bands: [0; 15],
            diw_top: 0x3a,
            clock: [0; 4],
            muted: false,
            music: false,
            screens: [vec![0; PW * PH], vec![0; PW * PH]],
            drawbuf: 0,
            shown: 0,
            bg: vec![0; PW * PH],
            last_swap: 0,
            effects_on: false,
            ch_free_at: [0; 4],
            ch_rr: 0,
            rng,
            objs,
            level: 0,
            lives: 3,
            energy: 0,
            energy_due: 0,
            score: [0; 6],
            bonus_score: [0; 6],
            mode: 0,
            options,
            turn_rate: 0x1000,
            high,
            high_loaded: false,
            cheat_pos: 0,
            cheat_on: false,
            prev1: 0,
            prev2: 0,
            demo_in: 0,
            demo_prev: 0,
            f404: false,
            f405: false,
            f406: false,
            kill_touch: false,
            pods4: false,
            wrapped: false,
            turned_right: false,
            pod_count: 0,
            secret: 0,
            over_timer: 0,
            secret_timer: 0,
            banner_timer: 0,
            secret_level: 0,
            hit_list: Vec::with_capacity(OBJS),
            spark_timer: 0,
            exploding: false,
            alive: 0,
            pod_time: 0,
            bonus: 0,
            chain: 0,
            pod_frame: 0,
            last_obj: 0,
            hbars: Vec::new(),
            vbars: Vec::new(),
            dirty: Vec::new(),
            trainer: [false; 3],
            start_level: 1,
            start_pressed: false,
            a,
        }
    }

    /// The frame wait: hands out the frame composed at the last wait (with this frame's sounds), then takes the next
    /// frame's input and does the vertical-blank work. Every frame wait of the original is one call.
    pub fn wait_frame(&mut self) {
        if let Some(mut o) = self.pending.take() {
            o.sounds = std::mem::take(&mut self.sounds);
            o.save = self.save_out.take();
            o.quit = self.quit;
            if self.tx.send(o).is_err() {
                std::panic::panic_any(Stop);
            }
        }
        let Ok(fi) = self.rx.recv() else {
            std::panic::panic_any(Stop)
        };
        if fi.saved.is_some() {
            self.saved = fi.saved;
        }
        self.input = fi.input;
        self.vblank();
        let picture = Arc::new(self.compose());
        let state = self.named_state();
        self.pending = Some(FrameOut {
            picture,
            sounds: Vec::new(),
            save: None,
            quit: false,
            state,
        });
    }

    /// What the vertical-blank handler (+$F16) did each frame.
    fn vblank(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        if self.fade_dir != 0 {
            self.fade_step();
        }
        let i = &self.input;
        let mut j = 0;
        let bits = [
            (Action::Up, UP),
            (Action::Down, DOWN),
            (Action::Left, LEFT),
            (Action::Right, RIGHT),
        ];
        for (a, b) in bits {
            if i.held(a) {
                j |= b;
            }
        }
        if i.held(Action::Fire) || i.held(Action::Primary) {
            j |= FIRE;
        }
        // Player one's controller (port 2 or the keyboard); the second player's has no input of its own here.
        self.joy1 = j;
        self.joy2 = 0;
        if i.pressed(Action::Cancel) {
            self.keys.push_back(KEY_ESC);
        }
        if i.pressed(Action::Pause) || i.pressed(Action::Fire2) {
            self.keys.push_back(KEY_P);
        }
        if i.pressed(Action::Start) {
            self.keys.push_back(KEY_RETURN);
        }
        // The bonus levels' clock: minutes, tens, units, frames (counted down while any digit is set).
        if self.clock != [0; 4] {
            let c = &mut self.clock;
            c[3] = c[3].wrapping_sub(1);
            if c[3] == 0 {
                if *c == [0; 4] {
                    self.f406 = true;
                }
            } else if c[3] & 0x80 != 0 {
                c[3] = 0x31;
                if !self.trainer[2] {
                    c[2] = c[2].wrapping_sub(1);
                }
                if c[2] & 0x80 != 0 {
                    c[2] = 9;
                    c[1] = c[1].wrapping_sub(1);
                    if c[1] & 0x80 != 0 {
                        c[1] = 5;
                        c[0] = c[0].wrapping_sub(1);
                    }
                }
            }
        }
    }

    /// One step of a fade (fade_step_palettes, +$10A0): the level moves one step, and the colour registers and the
    /// gradient bands are recomputed from their lists.
    fn fade_step(&mut self) {
        let d7 = (self.fade_level as i16).wrapping_sub(self.fade_dir) as u16;
        self.fade_level = d7;
        if d7 == 0 || d7 == 15 {
            self.fade_dir = 0;
        }
        for k in 0..15 {
            self.bands[k] = fade_colour(self.band_base[k], d7);
        }
        self.hw[30] = self.bands[0];
        self.hw[31] = self.bands[1];
        let n = if self.all32 { 32 } else { 30 };
        let a = self.a.clone();
        let list: Vec<u16> = match self.pal_sel {
            Pal::Game => self.pal_game.clone(),
            Pal::Title => a.tables.pal_title.clone(),
            Pal::Boot => [&a.tables.pal_boot[..], &a.tables.pal_hw[..]].concat(),
            Pal::Hw => a.tables.pal_hw.clone(),
            Pal::Picture => self.pic_cmap.clone(),
        };
        for k in 0..n {
            self.hw[k] = fade_colour(*list.get(k).unwrap_or(&0), d7);
        }
    }

    /// fade_out (+$4324): fade to black and wait until it is done.
    pub fn fade_out(&mut self) {
        self.fade_level = 0;
        self.fade_dir = -1;
        while self.fade_dir != 0 {
            self.wait_frame();
        }
    }

    /// fade_out_if_needed (+$431C).
    pub fn fade_out_if_needed(&mut self) {
        if self.fade_level != 15 {
            self.fade_out();
        }
    }

    /// fade_in (+$4336): start fading up (no wait).
    pub fn fade_in(&mut self) {
        if self.fade_level != 0 {
            self.fade_dir = 1;
        }
    }

    /// The picture of this frame.
    fn compose(&self) -> Vec<Argb> {
        if self.display == Display::Composed {
            return self.composed.clone();
        }
        let mut o = vec![rgb12(self.hw[0]); W * H];
        let top = self.diw_top - TOP_LINE;
        let scr = &self.screens[self.shown];
        let pal: Vec<Argb> = self.hw.iter().map(|c| rgb12(*c)).collect();
        let bands: Vec<Argb> = self.bands.iter().map(|c| rgb12(*c)).collect();
        for y in 0..PH {
            let oy = top + y as i32;
            if !(0..H as i32).contains(&oy) {
                continue;
            }
            let row = &scr[y * PW..(y + 1) * PW];
            let dst = &mut o[oy as usize * W..oy as usize * W + W];
            let k = (y / 14).min(13);
            for x in 0..PW {
                let c = row[x] as usize & 31;
                dst[x] = if self.display == Display::Game && c >= 30 {
                    bands[k + c - 30]
                } else {
                    pal[c]
                };
            }
        }
        o
    }

    // ---- sound

    /// play_sound_effect (+$B66): the sample plays once on the next free channel (or the next in turn).
    pub fn effect(&mut self, s: &Sample) {
        if !self.effects_on {
            return;
        }
        let mut ch = None;
        for c in (0..4).rev() {
            if self.ch_free_at[c] <= self.frame {
                ch = Some(c);
                break;
            }
        }
        let c = ch.unwrap_or_else(|| {
            self.ch_rr = (self.ch_rr + 1) & 3;
            self.ch_rr as usize
        });
        // The channel stays taken until the sample has played and its interrupt has turned it off.
        let frames = (s.data.len() as f32 / SAMPLE_RATE * 50.0).ceil() as u32 + 2;
        self.ch_free_at[c] = self.frame + frames;
        let b = Block::whole(s);
        self.sounds.push(Snd::Start {
            ch: c,
            block: b,
            next: Block::silence(),
            rate: SAMPLE_RATE,
            vol: self.vol(),
        });
    }

    pub fn effect_named(&mut self, n: &str) {
        let a = self.a.clone();
        if let Some(s) = a.sfx.get(n) {
            self.effect(s);
        }
    }

    pub fn vol(&self) -> u8 {
        if self.muted {
            0
        } else {
            64
        }
    }

    /// wait_effects_idle (+$C58).
    pub fn wait_effects_idle(&mut self) {
        while self.effects_on && self.ch_free_at.iter().any(|&f| f > self.frame) {
            self.wait_frame();
        }
    }

    /// start_sound_effects (+$B46): the channels go over to the effects.
    pub fn start_sound_effects(&mut self) {
        self.effects_on = true;
        self.music = false;
        self.ch_free_at = [0; 4];
        for c in 0..4 {
            self.sounds.push(Snd::Stop(c));
        }
    }

    /// start_music (+$C60): each channel plays its list of sound blocks round and round.
    pub fn start_music(&mut self) {
        if self.music {
            return;
        }
        self.effects_on = false;
        for (c, m) in self.a.music.iter().enumerate() {
            let mut d = Vec::new();
            for b in &m.blocks[m.looped..] {
                d.extend_from_slice(&b.data);
            }
            let mut pre = Vec::new();
            for b in &m.blocks[..m.looped] {
                pre.extend_from_slice(&b.data);
            }
            let body = Sample {
                data: Arc::new(d),
                rate: SAMPLE_RATE,
            };
            let first = if pre.is_empty() {
                Block::whole(&body)
            } else {
                Block::whole(&Sample {
                    data: Arc::new(pre),
                    rate: SAMPLE_RATE,
                })
            };
            self.sounds.push(Snd::Start {
                ch: c,
                block: first,
                next: Block::whole(&body),
                rate: SAMPLE_RATE,
                vol: self.vol(),
            });
        }
        self.music = true;
    }

    /// The keyboard's key-down codes in order (read_key_ascii, +$116A: one per call, none when there is none).
    pub fn read_key(&mut self) -> Option<u8> {
        self.keys.pop_front()
    }

    /// The frame counter's low word ($42C), which the program's timers compare.
    pub fn f16(&self) -> u16 {
        self.frame as u16
    }
}

/// fade_colour_list (+$10FC): each component less the fade level, not below zero.
pub fn fade_colour(c: u16, lvl: u16) -> u16 {
    let b = (c & 15) as i16 - lvl as i16;
    let g = (c & 0xF0) as i16 - (lvl << 4) as i16;
    let r = (c & 0xF00) as i16 - (lvl << 8) as i16;
    let mut o = if b < 0 { 0 } else { b as u16 };
    if g >= 0 {
        o |= g as u16;
    }
    if r >= 0 {
        o |= r as u16;
    }
    o
}
