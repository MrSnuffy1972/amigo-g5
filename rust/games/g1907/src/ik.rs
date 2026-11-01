//! International Karate + (System 3, 1988), the main program, rewritten from the recovered code as typed Rust.
//!
//! Each original routine keeps its name (from the recovery's function list, many of them automatic and misleading)
//! as a method, and its doc comment says what it really does. Game state is typed: per-fighter arrays of three
//! (the white, red and blue fighters), the judge's speech, the bonus objects, the high-score table. The original's
//! byte variables are named after what they hold; the address the original kept each one at is given once in its
//! comment, for reading the two side by side.
//!
//! Frame waits: the original's main program spins on counters the frame interrupt changes; here every such wait is
//! `frame()`, which hands the picture to the engine, takes the next tick's input and runs the frame interrupt's work
//! (`vblank`), exactly once a tick (50 a second, PAL).

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::audio::Block;
use native_engine::input::{Action, InputState};

use crate::assets::{Assets, Pic, H, W};
use crate::co::{FrameIn, FrameOut, Snd, Stop, View};
use crate::music::Music;

/// Raised inside the main program to restart it at `fault_restart` (a game start from the title or demo).
struct Restart;

/// Which picture a bob comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Src {
    Sheet1,
    Sheet2,
    Back,
}

/// A picture source: a 16-pixel-wide column of a picture, from (x, y).
#[derive(Clone, Copy, Debug)]
pub struct Bob {
    pub src: Src,
    pub x: i32,
    pub y: i32,
}

impl Bob {
    /// The source of a bob the original named by its address in one of the three pictures.
    pub fn at(addr: u32) -> Bob {
        let u = addr.wrapping_sub(0x600) as i32;
        let (src, base) = if (0x1C45C..0x2415C).contains(&u) {
            (Src::Sheet1, 0x1C45C)
        } else if (0x242CA..0x2BFCA).contains(&u) {
            (Src::Sheet2, 0x242CA)
        } else {
            (Src::Back, 0x2BFCA)
        };
        let o = u - base;
        Bob {
            src,
            x: (o % 160) * 8,
            y: o / 160,
        }
    }
    pub fn right(self, words: i32) -> Bob {
        Bob {
            x: self.x + 16 * words,
            ..self
        }
    }
    pub fn down(self, rows: i32) -> Bob {
        Bob {
            y: self.y + rows,
            ..self
        }
    }
}

/// A rectangle of a screen to put the background back over next time that screen is drawn.
#[derive(Clone, Copy, Debug)]
pub struct Dirty {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

pub const NF: usize = 3;

pub struct Ik {
    pub a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    input: InputState,
    sounds: Vec<Snd>,
    pub music: Music,
    // ---- the two screens and what was drawn on them
    pub scr: [Vec<u8>; 2],
    /// Which screen is being drawn ($638) and which is shown ($634).
    pub back: usize,
    pub shown: usize,
    pub dirty: [Vec<Dirty>; 2],
    /// The four one-bit collision layers ($66e20, $68e20, $6ae20, $6ce20), 320 by 200.
    pub layer: [Vec<u8>; 4],
    /// Fighters' silhouettes go into layer `lw_f`, bonus objects into `lw_o`; collisions read `lr_f` and `lr_o`.
    pub lw_f: usize,
    pub lw_o: usize,
    pub lr_f: usize,
    pub lr_o: usize,
    /// Write silhouettes ($90DC) / object outlines ($90DE) into the collision layers.
    pub sil_on: bool,
    pub obj_on: bool,
    /// The title screen's own colours are shown (the original's second display list, $618).
    pub title_colours: bool,
    /// Colour registers the program writes itself (title screen).
    pub regs: [u16; 32],
    // ---- global state
    pub phase: u8,              // $678
    pub cur: usize,             // $67a, the fighter being worked on
    pub vbl: u8,                // $608 frame counter
    pub vbl_hi: u8,             // $862
    pub step_wait: u8,          // $82e frames since the last logic step
    pub steps: u8,              // $930 logic steps
    pub human: [u8; 3],         // $609.. 1 = a player controls this fighter
    pub humans: u8,             // $60c
    pub key_count: u8,          // $60e
    pub speed: u8,              // $60f
    pub music_on: u8,           // $610
    pub sfx_on: u8,             // $611
    pub ntsc: u8,               // $612
    pub sea: u8,                // $613 (0, 2, 4, 6)
    pub flip: u8,               // $615
    pub trousers: u8,           // $617 (0 or 12)
    pub shadow_col: u16,        // $61c
    pub col_p1: u16,            // $61e
    pub col_p2: u16,            // $620
    pub col_c9: u16,            // $622
    pub col_c1: u16,            // $624
    pub volume: u16,            // $626
    pub cheats_used: u16,       // $628
    pub outline: bool,          // $62e
    pub german: bool,           // $632
    pub restart: bool,          // $939
    pub frame_x: [u8; 3],       // $67d current frame (pose) of each fighter
    pub x: [u8; 3],             // $680 position (2-pixel units)
    pub facing: [u8; 3],        // $683 1 = facing left (mirrored)
    pub target: usize,          // $686
    pub cur_ai: usize,          // $67b the fighter whose choice or attack is being worked out
    pub dist4: u8,              // $68a distance / 4
    pub dist: u8,               // $68b distance
    pub sides: u8,              // $68c side/facing combination
    pub msg_req: u8,            // $691 judge message wanted ($ff = none)
    pub msg_cur: u8,            // $692
    pub hum_mask: u8,           // $693
    pub result_msg: u8,         // $694
    pub order: [u8; 3],         // $695 fighters by points, best last
    pub stun: [u8; 3],          // $698
    pub snd_t: [u8; 2],         // $6a2, $6a3
    pub bonus_kind: u8,         // $6a4
    pub pose_t: [u8; 3],        // $7cc
    pub round: u8,              // $7cf bouts won (BCD): the belt
    pub atk: u8,                // $7d0
    pub wait: u8,               // $7d1 frames to wait (counted down by the frame interrupt)
    pub points: [u8; 3],        // $7d2 half points this bout
    pub scored: u8,             // $7d5
    pub score: [[u8; 3]; 3],    // $7d6 scores (BCD, 2 digits a byte)
    pub timer: u8,              // $7df bout clock (BCD seconds)
    pub turned: [u8; 3],        // $7ed
    pub want: [u8; 3],          // $7f3 move asked for
    pub cursor: [u8; 3],        // $7f6 animation cursor
    pub next_cursor: [u8; 3],   // $7f9
    pub mv: [u8; 3],            // $7fc current move
    pub same: [u8; 3],          // $7ff
    pub hold: [u8; 3],          // $802
    pub rev: [u8; 3],           // $805
    pub reversible: [u8; 3],    // $808
    pub hit: [u8; 3],           // $80b 1 = hit and falling, 2 = hit
    pub got: [u8; 3],           // $80e
    pub idle: [u8; 3],          // $811
    pub frozen: [u8; 3],        // $814
    pub attacking: [u8; 3],     // $817
    pub flags: [u8; 3],         // $81a
    pub prev_flags: [u8; 3],    // $81d
    pub last_move: [u8; 3],     // $820
    pub front: u8,              // $829
    pub demo_first: u8,         // $82a
    pub points_to_add: u8,      // $83c
    pub level: u8,              // $83d (2..5)
    pub sec_t: u8,              // $83f
    pub half_add: u8,           // $842
    pub bout_pts: [[u8; 2]; 3], // $843/$846
    pub joy_tmp: u8,            // $84a
    pub shown_secs: u8,         // $84c
    pub bout_in_round: u8,      // $84d
    pub first_hit: u8,          // $84e
    pub joy: [u8; 2],           // $850 port 2 (player 1), $851 port 1 (player 2): active-low UDLRF
    pub entry: u8,              // $85a name entry: the fighter entering
    pub rank: u8,               // $85b
    pub prev_frame: [u8; 3],    // $864
    pub sfx_req: u8,            // $867
    pub delay_snd: (u8, u8),    // $86e, $86f
    pub hit_t: [u8; 3],         // $8bb
    pub hit_t2: [u8; 3],        // $8be
    pub hit_anim: [u8; 3],      // $8c1
    pub hit_gfx: [u8; 3],       // $8c4
    pub hit_by: [u8; 3],        // $8c7
    pub hit_x: [u8; 3],         // $8ca
    pub hit_y: [u8; 3],         // $8cd
    pub extra: [u8; 3],         // $8e0
    pub extra_x: [u8; 3],       // $8e5
    pub bonus_lv: [u8; 2],      // $8e8, $8e9
    pub result: u8,             // $8eb
    pub saved_xf: [u8; 6],      // $8ec
    pub key_delay: u8,          // $8f5
    pub flash: [u8; 2],         // $8f6
    pub demo: u8,               // $8f8
    pub entry_pos: (u8, u8),    // $8f9, $8fa
    pub nfighters: u8,          // $8fb
    pub held_copy: [u8; 3],     // $8fc
    pub skill: [u8; 3],         // $8ff, $900, $901
    pub music_div: u8,          // $902
    pub rng_n: u16,             // $92e
    pub rng_v: [u16; 3],        // $7856..
    pub msg_t: u16,             // $952 code-word message timer
    pub msg_i: u16,             // $954
    pub keys: [u8; 5],          // $986 last keys (newest first)
    pub last_raw: u16,          // $97a
    pub sfx_busy: u16,          // $94e
    pub sfx_left: u32,
    pub pause: bool,     // $94c
    pub shield_alt: u16, // $948
    pub toggle_94a: u16, // $94a
    pub no_wait: u16,    // $958
    /// The high-score table: 51 rows of score (2 BCD bytes), name (3 letters), flag.
    pub hs_score: [[u8; 2]; 51],
    pub hs_name: [[u8; 3]; 51],
    pub hs_flag: [u8; 51],
    pub speech: crate::speech::Speech,
    pub bonus: crate::bonus::Bonus,
    pub attract: crate::attract::Attract,
    pub ambient: crate::ambient::Ambient,
    pub sign: (u16, u16, u16), // $942, $944, $946 the top-scores sign
    pub tick: u64,
    snap: Option<View>,
}

/// BCD helpers (the original counts seconds, points and scores in BCD).
pub fn bcd_add(a: u8, b: u8) -> (u8, bool) {
    let (mut lo, mut hi) = ((a & 15) + (b & 15), (a >> 4) + (b >> 4));
    if lo > 9 {
        lo -= 10;
        hi += 1;
    }
    let c = hi > 9;
    if c {
        hi -= 10;
    }
    ((hi << 4) | lo, c)
}
pub fn bcd_sub1(a: u8) -> u8 {
    if a == 0 {
        0x99
    } else if a & 15 == 0 {
        a - 7
    } else {
        a - 1
    }
}

impl Ik {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Ik {
        let music = Music::new(a.clone());
        let z = || vec![0u8; W * H];
        let mut g = Ik {
            a,
            rx,
            tx,
            input: InputState::default(),
            sounds: Vec::new(),
            music,
            scr: [z(), z()],
            back: 1,
            shown: 0,
            dirty: [Vec::new(), Vec::new()],
            layer: [z(), z(), z(), z()],
            lw_f: 2,
            lw_o: 3,
            lr_f: 0,
            lr_o: 1,
            sil_on: false,
            obj_on: false,
            title_colours: false,
            regs: [0; 32],
            phase: 8,
            cur: 0,
            vbl: 0,
            vbl_hi: 0,
            step_wait: 0,
            steps: 0,
            human: [0; 3],
            humans: 0,
            key_count: 0,
            speed: 2,
            music_on: 1,
            sfx_on: 1,
            ntsc: 0,
            sea: 0,
            flip: 4,
            trousers: 0,
            shadow_col: 0x333,
            col_p1: 0xc00,
            col_p2: 0xe6e,
            col_c9: 0xcc0,
            col_c1: 0xf,
            volume: 15,
            cheats_used: 0,
            outline: true,
            german: false,
            restart: false,
            frame_x: [0x57; 3],
            x: [0; 3],
            facing: [0; 3],
            target: 0,
            cur_ai: 0,
            dist4: 0,
            dist: 0,
            sides: 0,
            msg_req: 0xff,
            msg_cur: 0,
            hum_mask: 0,
            result_msg: 0,
            order: [0, 1, 2],
            stun: [0; 3],
            snd_t: [0; 2],
            bonus_kind: 0,
            pose_t: [0; 3],
            round: 0,
            atk: 0,
            wait: 0,
            points: [0; 3],
            scored: 0,
            score: [[0; 3]; 3],
            timer: 0x30,
            turned: [0; 3],
            want: [0; 3],
            cursor: [0; 3],
            next_cursor: [0; 3],
            mv: [0; 3],
            same: [0; 3],
            hold: [0; 3],
            rev: [0; 3],
            reversible: [0; 3],
            hit: [0; 3],
            got: [0; 3],
            idle: [0; 3],
            frozen: [0; 3],
            attacking: [0; 3],
            flags: [0; 3],
            prev_flags: [0; 3],
            last_move: [0; 3],
            front: 0,
            demo_first: 0,
            points_to_add: 0,
            level: 0,
            sec_t: 0,
            half_add: 0,
            bout_pts: [[0; 2]; 3],
            joy_tmp: 0,
            shown_secs: 0,
            bout_in_round: 0,
            first_hit: 0xff,
            joy: [0x0f; 2],
            entry: 0,
            rank: 0,
            prev_frame: [0; 3],
            sfx_req: 0xff,
            delay_snd: (0, 0),
            hit_t: [0; 3],
            hit_t2: [0; 3],
            hit_anim: [0; 3],
            hit_gfx: [0; 3],
            hit_by: [0; 3],
            hit_x: [0; 3],
            hit_y: [0; 3],
            extra: [0; 3],
            extra_x: [0; 3],
            bonus_lv: [0; 2],
            result: 0,
            saved_xf: [0; 6],
            key_delay: 0,
            flash: [0; 2],
            demo: 0,
            entry_pos: (0, 0),
            nfighters: 3,
            held_copy: [0; 3],
            skill: [0; 3],
            music_div: 0,
            rng_n: 0,
            rng_v: [0; 3],
            msg_t: 0,
            msg_i: 0,
            keys: [0; 5],
            last_raw: 0,
            sfx_busy: 0,
            sfx_left: 0,
            pause: false,
            shield_alt: 0,
            toggle_94a: 0,
            no_wait: 0,
            hs_score: [[0; 2]; 51],
            hs_name: [[0; 3]; 51],
            hs_flag: [0; 51],
            speech: Default::default(),
            bonus: Default::default(),
            attract: Default::default(),
            ambient: Default::default(),
            sign: (0, 0, 0),
            tick: 0,
            snap: None,
        };
        g.initialize_input_state();
        // --seed: the draws made before this point (the original's cracktro and title draw too; the count varies with
        // when the player clicks and presses fire, so a seed reproduces one particular session)
        if let Some(c) = seed {
            g.set_draws(c as u16);
        }
        g
    }

    /// The main program: the cold start, then the phase loop; a game start from the title restarts it.
    pub fn run(&mut self) {
        let mut first = true;
        loop {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if first {
                    self.attract_then_main_loop();
                } else {
                    self.fault_restart();
                }
                self.main_loop_phase_dispatch();
            }));
            match r {
                Ok(()) => return,
                Err(e) if e.downcast_ref::<Restart>().is_some() => first = false,
                Err(e) => std::panic::resume_unwind(e),
            }
        }
    }

    fn out(&mut self) -> FrameOut {
        let mut rows = Vec::with_capacity(H);
        let mut pal = [0u16; 16];
        if self.title_colours {
            pal.copy_from_slice(&self.regs[..16]);
            rows.resize(H, pal);
        } else {
            pal.copy_from_slice(&self.regs[..16]);
            let cr = &self.a.colour_rows;
            let slot = |n: &str| self.a.slots.get(n).copied().unwrap_or(usize::MAX);
            let (s_sea, s_p1, s_p2, s_c9, s_c1, s_sh) = (
                slot("sea"),
                slot("p1"),
                slot("p2"),
                slot("c9"),
                slot("c1"),
                slot("shadow"),
            );
            let mut k = 0;
            for y in 0..H as i32 {
                while k < cr.len() && cr[k].0 <= y {
                    let (_, c, mut v) = cr[k];
                    if k == s_sea {
                        v = self.a.sea_choices[(self.sea as usize / 2) & 3];
                    } else if k == s_p1 {
                        v = self.col_p1;
                    } else if k == s_p2 {
                        v = self.col_p2;
                    } else if k == s_c9 {
                        v = self.col_c9;
                    } else if k == s_c1 {
                        v = self.col_c1;
                    } else if k == s_sh {
                        v = self.shadow_col;
                    }
                    if c < 16 {
                        pal[c] = v;
                    }
                    k += 1;
                }
                rows.push(pal);
            }
        }
        FrameOut {
            view: View {
                screen: self.scr[self.shown].clone(),
                rows,
            },
            sounds: std::mem::take(&mut self.sounds),
            state: self.state(),
        }
    }

    /// One frame: the picture goes out, the next tick's input comes in, and the frame interrupt's work runs.
    pub fn frame(&mut self) {
        // the picture of this frame is the shown screen as the frame interrupt left it (the original's display
        // list took the screen at the frame's start; the interrupt draws the bonus balls straight onto it)
        let mut o = self.out();
        if let Some(v) = self.snap.take() {
            o.view = v;
        }
        if self.tx.send(o).is_err() {
            std::panic::resume_unwind(Box::new(Stop));
        }
        match self.rx.recv() {
            Ok(i) => self.input = i.input,
            Err(_) => std::panic::resume_unwind(Box::new(Stop)),
        }
        self.tick += 1;
        self.vblank();
        let s = std::mem::take(&mut self.sounds);
        self.snap = Some(self.out().view);
        self.sounds = s;
        if self.restart {
            self.restart = false;
            std::panic::resume_unwind(Box::new(Restart));
        }
    }

    /// infinite_loop_when_flag_set ($713C): waits for `n` frames (one if n is 0); returns at once in phase 0.
    pub fn wait_frames(&mut self, n: u16) {
        if self.phase == 0 {
            return;
        }
        let mut n = n;
        loop {
            self.frame();
            if n == 0 {
                return;
            }
            n -= 1;
            if n == 0 {
                return;
            }
        }
    }

    pub fn state(&self) -> Vec<(String, i64)> {
        let mut v = vec![
            ("phase".to_string(), self.phase as i64),
            ("timer".to_string(), self.timer as i64),
            ("round".to_string(), self.round as i64),
            ("level".to_string(), self.level as i64),
            ("msg".to_string(), self.msg_cur as i64),
        ];
        for i in 0..NF {
            v.push((format!("x{i}"), self.x[i] as i64));
            v.push((format!("frame{i}"), self.frame_x[i] as i64));
            v.push((format!("facing{i}"), self.facing[i] as i64));
            v.push((format!("move{i}"), self.mv[i] as i64));
            v.push((format!("points{i}"), self.points[i] as i64));
            v.push((format!("human{i}"), self.human[i] as i64));
        }
        for i in 0..2 {
            let s = &self.score[i];
            v.push((format!("score{i}"), ((s[0] as i64) << 8) | s[1] as i64));
        }
        v.push(("rng_n".to_string(), self.rng_n as i64));
        v.push(("rng_v0".to_string(), self.rng_v[0] as i64));
        v.push(("vbl".to_string(), self.vbl as i64));
        v.push(("extra".to_string(), self.ambient.state as i64));
        v
    }
}

// ---- the frame interrupt, input, sound, random numbers
impl Ik {
    /// The joystick byte the game reads ($850/$851): bits 0-3 up, down, left, right (0 = held), bit 4 fire (1 = held).
    fn joy_byte(i: &InputState) -> u8 {
        let mut b = 0x0f;
        for (k, a) in [Action::Up, Action::Down, Action::Left, Action::Right]
            .iter()
            .enumerate()
        {
            if i.held(*a) {
                b &= !(1 << k);
            }
        }
        if i.held(Action::Fire) || i.held(Action::Primary) {
            b |= 0x10;
        }
        b
    }

    /// The keyboard as the original's key handler (self_3, $1448) saw it: the newest key first in `keys`.
    fn key(&mut self, code: u8) {
        self.keys.rotate_right(1);
        self.keys[0] = code;
    }

    /// The level-3 frame interrupt (l0_098E).
    fn vblank(&mut self) {
        self.vbl = self.vbl.wrapping_add(1);
        if self.vbl == 0 {
            self.vbl_hi = self.vbl_hi.wrapping_add(1);
        }
        if self.wait > 0 {
            self.wait -= 1;
        }
        self.restore_interrupt_dma_state();
        self.music_tick_divider();
        // input: the joystick on port 2 is player 1; the engine's second player is port 1
        self.joy[0] = Ik::joy_byte(&self.input);
        let p2 = InputState {
            held: self.input.held2,
            ..Default::default()
        };
        self.joy[1] = Ik::joy_byte(&p2);
        let i = self.input.clone();
        if i.pressed(Action::Start) {
            self.key(if self.human == [0; 3] { 0x3b } else { 0x19 });
        }
        if i.pressed(Action::Fire2) {
            self.key(0x3c);
        }
        if i.pressed(Action::Secondary) {
            self.key(0x3d);
        }
        // $618 set (the title's colours, or a game starting from the title until its first flip): the original's
        // interrupt only loads the title's colours (l0_0BFE)
        if self.title_colours && self.phase != 8 {
            self.step_wait = self.step_wait.wrapping_add(1);
            return;
        }
        if self.title_colours && self.phase == 8 {
            self.handle_input_byte();
            if self.restart {
                return;
            }
            crate::attract::vbl_step(self);
            return;
        }
        self.handle_input_byte();
        if self.restart {
            return;
        }
        if self.phase == 8 {
            crate::attract::vbl_step(self);
            return;
        }
        if self.key_delay > 0 {
            self.key_delay -= 1;
        }
        self.step_wait = self.step_wait.wrapping_add(1);
        self.speech.vbl_count = self.speech.vbl_count.wrapping_add(1);
        for i in 0..NF {
            if self.hit_t[i] > 0 {
                self.hit_t[i] -= 1;
            }
            if self.hit_t2[i] > 0 {
                self.hit_t2[i] -= 1;
            } else if self.stun[i] > 0 {
                self.stun[i] -= 1;
            }
        }
        for t in self.snd_t.iter_mut() {
            if *t > 0 {
                *t -= 1;
            }
        }
        if self.delay_snd.0 > 0 {
            self.delay_snd.0 -= 1;
            if self.delay_snd.0 == 0 {
                self.set_state_flags(self.delay_snd.1);
            }
        }
        crate::ambient::title_logo_frame(self);
        self.advance_scrolling_object();
        if self.vbl & 3 == 0 {
            self.decrement_flag_table_bytes();
        }
        crate::bonus::process_state_table(self);
        crate::bonus::update_bonus_objects(self);
        if self.phase == 1 && self.mv[0] != 0x1c {
            if self.msg_t > 0 {
                self.msg_t -= 1;
            }
            self.sec_t = self.sec_t.wrapping_sub(1);
            if self.sec_t == 0 && self.timer != 0 {
                self.timer = bcd_sub1(self.timer);
                self.sec_t = 50;
            }
        }
    }

    /// handle_input_byte ($607A): the option keys, the code words, and starting a game.
    fn handle_input_byte(&mut self) {
        if (self.phase == 5 || self.phase == 7) && self.vbl & 1 == 0 {
            return;
        }
        let k4 = u32::from_le_bytes([self.keys[0], self.keys[1], self.keys[2], self.keys[3]]);
        if self.key_count < 3 {
            self.key_count += 1;
            return;
        }
        let k = self.keys[0];
        let mut done = false;
        if !matches!(self.phase, 5 | 7 | 8) {
            let sk = &self.a.t("speed_keys")[..5];
            if let Some(i) = (0..5).rev().find(|&i| sk[i] == k && k != 0) {
                self.speed = i as u8;
                self.call_sequence_routines();
                done = true;
            }
        }
        if !done {
            done = true;
            if k == 0x66 || k == 0x09 {
                self.sea = (self.sea + 2) & 6;
            } else if k4 == 0xa19e9f14 {
                for _ in 0..3 {
                    self.music_tick_divider();
                }
                done = false;
            } else if k4 == 0xa197a614 {
                // the sound filter switch: no effect on the native mixer
            } else if k4 & 0xffffff == 0x999e2e {
                self.ambient.cheat[0] = true;
            } else if k4 == 0xa1979f23 {
                self.ambient.cheat[1] = true;
            } else if k4 == 0xb0979320 {
                self.ambient.cheat[2] = true;
            } else if k4 == 0x99929397 {
                self.ambient.cheat[3] = true;
            } else if k == 0x1f {
                self.shadow_col = if self.shadow_col == 0x333 {
                    0x444
                } else {
                    0x333
                };
            } else if (k == 0x0d || k == 0x4e) && self.volume != 15 {
                self.volume += 1;
            } else if (k == 0x0c || k == 0x4a) && self.volume != 0 {
                self.volume -= 1;
            } else if k4 == 0xa2929332 {
                self.german = true;
            } else if k4 & 0xffff == 0x9625 {
                self.german = false;
            } else if k == 0x30 && self.phase == 5 {
                self.shield_alt ^= 1;
            } else if k == 0x19 && self.phase == 6 {
                self.toggle_94a ^= 1;
            } else if k == 0x24 {
                self.trousers = if self.trousers == 0 { 12 } else { 0 };
            } else if self.msg_t == 0 && (self.phase == 1 || self.phase == 3) && {
                let codes = self.a.t("cheats");
                k == 0x62
                    || (0..20).any(|n| {
                        u32::from_be_bytes([
                            codes[4 * n],
                            codes[4 * n + 1],
                            codes[4 * n + 2],
                            codes[4 * n + 3],
                        ]) == k4
                    })
            } {
                let codes = self.a.t("cheats").to_vec();
                let d0 = if k == 0x62 {
                    (self.humans + self.human[1]) as u16
                } else {
                    4 + (0..20)
                        .find(|&n| {
                            u32::from_be_bytes([
                                codes[4 * n],
                                codes[4 * n + 1],
                                codes[4 * n + 2],
                                codes[4 * n + 3],
                            ]) == k4
                        })
                        .unwrap_or(0) as u16
                };
                self.msg_i = d0;
                self.cheats_used += *codes.get(84 + d0 as usize).unwrap_or(&0) as u16;
                if self.cheats_used > 1 {
                    self.restart = true;
                    return;
                }
                self.msg_t = 250;
                self.speed = 2;
                self.call_sequence_routines();
            } else if k == 0x3d {
                if self.music_on != 0 {
                    self.music.stop(&mut self.sounds);
                    self.music_on = 0;
                } else {
                    self.music_on = 1;
                    self.music.start();
                }
            } else if k == 0x3e {
                self.sfx_on ^= 1;
            } else {
                done = false;
                if self.vbl >= 0x19 && (k == 0x3c || k == 0x3b) {
                    self.human = [1, (k == 0x3c) as u8, 0];
                    self.restart = true;
                    self.key_count = 0;
                    self.keys[0] = 0;
                    self.keys[1] = 0;
                    return;
                }
            }
        }
        if done {
            self.key_count = 0;
            self.keys[0] = 0;
        }
        if self.vbl < 0x19 && !done {
            return;
        }
        if self.human[0] | self.human[1] == 0 {
            if self.joy[0] & 0x10 != 0 {
                self.human = [1, 0, 0];
                self.restart = true;
            } else if self.joy[1] & 0x10 != 0 {
                self.human = [1, 1, 0];
                self.restart = true;
            }
            if self.restart {
                self.key_count = 0;
                self.keys = [0; 5];
            }
        }
    }

    /// set_state_flags ($0F9A): asks for sound effect `n` (when effects are on).
    pub fn set_state_flags(&mut self, n: u8) {
        let n = n & 0x1f;
        if n < 0x12 && self.sfx_on != 0 {
            self.sfx_req = n;
        }
    }

    /// restore_interrupt_dma_state ($0FE6) and set_audio_channel_3 ($102C): plays the requested effect once on
    /// channel 3, which the music then leaves alone until the sample has played.
    fn restore_interrupt_dma_state(&mut self) {
        if self.sfx_left > 0 {
            self.sfx_left -= 1;
            if self.sfx_left == 0 {
                self.sfx_busy = 0;
            }
        }
        if self.sfx_req == 0xff {
            return;
        }
        let n = self.sfx_req as usize;
        self.sfx_req = 0xff;
        let period: u16 = if self.phase == 7 {
            0x166
        } else if self.phase == 5 {
            0x1a4 - 2 * self.bonus.speed_step as u16 + (self.vbl & 7) as u16
        } else {
            let a = (self.random() & 0x3f) as u16;
            let b = (self.random() & 0xf) as u16;
            a + b + 0x13a
        };
        let s = self.a.sfx[n].clone();
        let len = s.len();
        let b = Block {
            data: s,
            start: 0,
            len,
        };
        self.sounds.push(Snd::Start {
            ch: 3,
            block: b,
            next: Block::silence(),
            period,
            volume: 64,
        });
        self.sfx_busy = 0xffff;
        self.sfx_left = (len as f32 * period as f32 / crate::co::PAULA * 50.0) as u32 + 1;
    }

    /// music_tick_divider ($0D3C): the music steps every frame while it is on.
    fn music_tick_divider(&mut self) {
        if self.ntsc != 0 {
            self.music_div += 1;
            if self.music_div >= 6 {
                self.music_div = 0;
                return;
            }
        }
        if self.music_on != 0 {
            let busy = self.sfx_busy != 0;
            self.music.frame(self.volume, busy, &mut self.sounds);
        }
    }

    /// call_and_return_result ($725C) / compute_obfuscation_checksum ($726A): the game's random byte. It mixes three
    /// running sums with bytes read from the program's own image (the table `rngwin`).
    pub fn random(&mut self) -> u8 {
        (self.random16() & 0xff) as u8
    }
    /// compute_obfuscation_checksum ($726A), exactly: d0 is cleared and loaded with the call count as a word, so the
    /// first byte read moves only its low byte and the count's high byte stays in d0's low word. That word (sign
    /// extended, as a 68000 index register) indexes the second read and is the multiplicand of `mulu.w`. `sub.b
    /// ($92E)` takes the count's high byte (the word is big-endian), and the closing `andi.l` keeps the low word (its
    /// operand is the label at $FFFF). Checked against 197 of 197 draws of a reference bout.
    pub fn random16(&mut self) -> u16 {
        self.rng_n = self.rng_n.wrapping_add(1);
        self.rng_v[0] = self.rng_v[0].wrapping_add(0x1b1);
        self.rng_v[1] = self.rng_v[1].wrapping_add(0x23b);
        self.rng_v[2] = self.rng_v[2].wrapping_add(0x2b3);
        let n = self.rng_n;
        let mut b = self.rng_byte(n as i16 as i32);
        b = b.wrapping_add(self.vbl);
        let d0w = (n & 0xff00) | b as u16;
        let d3 = self.rng_byte(100 + d0w as i16 as i32);
        b ^= d3;
        b = b.wrapping_sub((n >> 8) as u8);
        let d0w = (n & 0xff00) | b as u16;
        let mut r = (d0w as u32).wrapping_mul(self.rng_v[0] as u32) as u16;
        r = r.wrapping_add(self.rng_v[1]);
        let r = r ^ self.rng_v[2];
        rng_log(self.tick, n, r);
        r
    }
    /// A byte of the original's memory at $785C + `i` (`i` from -$8000 to $80FF). `rngwin` holds the game image from
    /// $600; below that the original reads the vectors and system area or (below 0) the Kickstart ROM, which a native
    /// game has not got: those read as 0, and only a game running past 32768 draws (about seven minutes) reaches them.
    fn rng_byte(&self, i: i32) -> u8 {
        let t = self.a.t("rngwin");
        let o = RNGWIN_BASE + i;
        if o >= 0 && (o as usize) < t.len() {
            t[o as usize]
        } else {
            0
        }
    }
    /// The power-on draw count as the original leaves it in the three running sums ($7856..): each draw adds a fixed
    /// step, so `c` draws give `c` times the steps. The call count itself ($92E) restarts with every game.
    pub fn set_draws(&mut self, c: u16) {
        self.rng_v = [
            c.wrapping_mul(0x1b1),
            c.wrapping_mul(0x23b),
            c.wrapping_mul(0x2b3),
        ];
    }
}

/// Verification aid: with `G1907_RNGLOG=file` every draw is written as "tick count value" (compared with the reference
/// harness's RNGLOG of the original).
fn rng_log(tick: u64, n: u16, r: u16) {
    use std::io::Write;
    static LOG: std::sync::OnceLock<Option<std::sync::Mutex<std::fs::File>>> =
        std::sync::OnceLock::new();
    let f = LOG.get_or_init(|| {
        std::env::var_os("G1907_RNGLOG")
            .and_then(|p| std::fs::File::create(p).ok())
            .map(std::sync::Mutex::new)
    });
    if let Some(m) = f {
        if let Ok(mut f) = m.lock() {
            let _ = writeln!(f, "{tick} {n} {r:04X}");
        }
    }
}

/// How a fighter's three planes become colours: the same frames drawn three ways give the white, red and blue
/// fighters (draw_fighter_frame, decode_sprite_pattern, xor_rotate_sprite_block).
/// Where $785C (the window the random numbers read) lies in `rngwin`, which starts at the game image's $600.
const RNGWIN_BASE: i32 = 0x785C - 0x600;

fn fighter_colour(i: usize, v: u8) -> u8 {
    let (b0, b1, b2) = (v & 1, (v >> 1) & 1, (v >> 2) & 1);
    match i {
        0 => (b0 << 1) | (b1 << 2) | (b2 << 3),
        1 => b1 | (b0 << 1) | (b1 << 2) | (b2 << 3),
        _ => b1 | (b0 << 1) | (b2 << 3),
    }
}

// ---- drawing
impl Ik {
    pub fn pic(&self, s: Src) -> &Pic {
        match s {
            Src::Sheet1 => &self.a.sheet1,
            Src::Sheet2 => &self.a.sheet2,
            Src::Back => &self.a.background,
        }
    }

    /// Puts the background back over a rectangle of screen `b`.
    fn restore(&mut self, b: usize, d: Dirty) {
        let x0 = d.x.max(0);
        let x1 = (d.x + d.w).min(W as i32);
        for y in d.y.max(0)..(d.y + d.h).min(H as i32) {
            let r = y as usize * W;
            for x in x0..x1 {
                self.scr[b][r + x as usize] = self.a.background.px[r + x as usize];
            }
        }
    }

    fn note(&mut self, d: Dirty) {
        let l = &mut self.dirty[self.back];
        if l.len() < 59 {
            l.push(d);
        }
    }

    /// self_8 ($1B514) and compute_sprite_tile_offset: draws a 16-pixel column of a picture, `h` rows, at screen
    /// pixel (`sx`, `y`), colour 0 clear; `keep` = the original's bob list entry (put the background back next time).
    pub fn bob(&mut self, buf: usize, src: Bob, sx: i32, y: i32, h: i32, keep: bool) {
        if !(0..200).contains(&y) || !(-32..W as i32).contains(&sx) {
            return;
        }
        let h = h.min(200 - y);
        if h <= 0 {
            return;
        }
        if keep && buf == self.back {
            self.note(Dirty {
                x: (sx + 32) / 16 * 16 - 32,
                y,
                w: 32,
                h: h + 1,
            });
        }
        let obj = self.obj_on;
        let lw = self.lw_o;
        for r in 0..h {
            let sy = src.y + r;
            if !(0..200).contains(&sy) {
                continue;
            }
            for c in 0..16 {
                let x = sx + c;
                if !(0..W as i32).contains(&x) {
                    continue;
                }
                let v = self.pic(src.src).at((src.x + c) as usize, sy as usize);
                if v != 0 {
                    let o = (y + r) as usize * W + x as usize;
                    self.scr[buf][o] = v;
                    if obj {
                        self.layer[lw][o] = 1;
                    }
                }
            }
        }
    }

    /// The same in the original's 2-pixel position units (self_8: screen x = 2 * x - 32).
    pub fn bob2(&mut self, buf: usize, src: Bob, x: i32, y: i32, h: i32, keep: bool) {
        if !(8..0xb0).contains(&x) {
            return;
        }
        self.bob(buf, src, 2 * x - 32, y, h, keep);
    }

    /// Draws fighter `i` (its frame, position and facing) into the drawn screen, with its silhouette and shadow.
    pub fn draw_fighter(&mut self, i: usize) {
        let n = self.frame_x[i] as usize + if self.facing[i] != 0 { 96 } else { 0 };
        if n >= self.a.fighters.len() || self.a.fighters[n].w == 0 {
            return;
        }
        let a = self.a.clone();
        let f = &a.fighters[n];
        let bx = 2 * self.x[i] as i32 + f.dx;
        let b = self.back;
        let x0 = (bx + 32).div_euclid(16) * 16 - 32;
        self.note(Dirty {
            x: x0,
            y: f.y,
            w: ((bx + f.w as i32 - x0 + 15) / 16 + 1) * 16,
            h: f.h as i32 + 1,
        });
        let sil = self.sil_on;
        let lw = self.lw_f;
        for r in 0..f.h {
            let y = f.y + r as i32;
            if !(0..H as i32).contains(&y) {
                continue;
            }
            for c in 0..f.w {
                let v = f.px[r * f.w + c];
                let x = bx + c as i32;
                if v == 0 || !(0..W as i32).contains(&x) {
                    continue;
                }
                let o = y as usize * W + x as usize;
                self.scr[b][o] = fighter_colour(i, v);
                if sil {
                    self.layer[lw][o] = 1;
                }
            }
        }
        if self.phase != 8 {
            // rotate_word_table ($A97E): the shadow, 80 pixels by 8 rows into the shadow plane (bit 0)
            let s = a.shadows[n];
            let sx = 2 * (self.x[i] as i32) - 32;
            for (r, row) in s.iter().enumerate() {
                let y = 192 + r;
                for c in 0..80 {
                    if row >> (79 - c) & 1 != 0 {
                        let x = sx + c;
                        if (0..W as i32).contains(&x) {
                            self.scr[b][y * W + x as usize] |= 1;
                        }
                    }
                }
            }
        }
    }

    /// erase_bobs_and_reset_list ($A84A): puts the background back under what was drawn on this screen last time
    /// (and clears those parts of the collision layers), then clears the shadow rows' shadow plane.
    pub fn erase_bobs_and_reset_list(&mut self) {
        let b = self.back;
        let l = std::mem::take(&mut self.dirty[b]);
        for d in &l {
            self.restore(b, *d);
            for y in d.y.max(0)..(d.y + d.h).min(H as i32) {
                for x in d.x.max(0)..(d.x + d.w).min(W as i32) {
                    let o = y as usize * W + x as usize;
                    if self.sil_on {
                        self.layer[self.lw_f][o] = 0;
                    }
                    if self.obj_on {
                        self.layer[self.lw_o][o] = 0;
                    }
                }
            }
        }
        for v in self.scr[b][192 * W..].iter_mut() {
            *v &= !1;
        }
    }

    /// shift_lfsr_state ($AA0A): leans the shadows out from the middle of the screen (bit shifts of the shadow plane
    /// on rows 193 to 199).
    pub fn shift_lfsr_state(&mut self) {
        use crate::skew::{Sk, SHADOW_SKEW};
        let b = self.back;
        let mut x = false;
        let word = |s: &Vec<u8>, row: usize, w: usize| -> u16 {
            (0..16).fold(0u16, |a, k| {
                a | (((s[row * W + w * 16 + k] & 1) as u16) << (15 - k))
            })
        };
        for &(row, w, op) in SHADOW_SKEW.iter() {
            let (row, w) = (row as usize, w as usize);
            let v = word(&self.scr[b], row, w);
            let nv = match op {
                Sk::SetX => {
                    x = v & 1 != 0;
                    v
                }
                Sk::Roxl => {
                    let c = v & 0x8000 != 0;
                    let r = (v << 1) | x as u16;
                    x = c;
                    r
                }
                Sk::Roxr => {
                    let c = v & 1 != 0;
                    let r = (v >> 1) | ((x as u16) << 15);
                    x = c;
                    r
                }
                Sk::Asr => {
                    x = v & 1 != 0;
                    ((v as i16) >> 1) as u16
                }
            };
            for k in 0..16 {
                let p = &mut self.scr[b][row * W + w * 16 + k];
                *p = (*p & !1) | ((nv >> (15 - k)) & 1) as u8;
            }
        }
    }

    /// write_sprite_mask_tile ($7632): an 8x8 character (font8) in colour `col` (opaque) into both screens at
    /// character cell `cell` (40 a row).
    pub fn char8(&mut self, cell: u16, ch: u8, col: u8) {
        let cell = cell & 0x3ff;
        let (cx, cy) = ((cell % 40) as usize * 8, (cell / 40) as usize * 8);
        let g = self.a.font8[(ch & 0x3f) as usize];
        for (r, bits) in g.iter().enumerate() {
            if cy + r >= H {
                break;
            }
            for k in 0..8 {
                let v = if bits & (0x80 >> k) != 0 { col & 15 } else { 0 };
                for s in 0..2 {
                    self.scr[s][(cy + r) * W + cx + k] = v;
                }
            }
        }
    }

    /// apply_sprite_tile_pattern ($9FE8): a 5-row character (font5) in colour `col` at text position `pos`
    /// (character cells, 40 a row, plus `row_off` rows), with a colour-15 outline when `outline` is on. `both` =
    /// decode_screen_tiles (into both screens).
    pub fn char5(&mut self, pos: u16, ch: u8, col: u8, row_off: usize, both: bool) {
        let pos = pos & 0x3ff;
        let (cx, cy) = ((pos % 40) as usize * 8, (pos / 40) as usize * 8 + row_off);
        let ch = (ch & 0x3f) as usize;
        if ch >= self.a.font5.len() {
            return;
        }
        let g = self.a.font5[ch];
        let ol = self.outline;
        for (r, &b) in g.iter().enumerate() {
            let y = cy + r;
            if y >= H {
                break;
            }
            let o = if ol { ((b >> 1) | (b << 1) | b) ^ b } else { 0 };
            for k in 0..8 {
                let m = 0x80 >> k;
                let x = cx + k;
                if x >= W {
                    continue;
                }
                for s in 0..2 {
                    if !(both || s == self.back) {
                        continue;
                    }
                    let p = &mut self.scr[s][y * W + x];
                    if b & m != 0 {
                        *p = col & 15;
                    } else if o & m != 0 {
                        *p = 15;
                    }
                }
            }
        }
    }

    /// load_configuration_table ($5FA6): waits until the logic step's frames have passed (the game's speed), then
    /// shows the screen just drawn and swaps, redraws the panel's numbers and picks the belt colours.
    pub fn flip(&mut self) {
        let n = self.a.t("speed_wait")[((self.speed + self.ntsc) as usize).min(5)];
        // at least one frame: the original's drawing took that long even when the step's frames had passed
        self.frame();
        while n >= self.step_wait {
            self.frame();
        }
        self.steps = self.steps.wrapping_add(1);
        self.step_wait = 0;
        self.title_colours = false;
        self.flip ^= 4;
        if self.flip == 0 {
            self.back = 1;
            self.shown = 0;
            (self.lr_f, self.lr_o, self.lw_f, self.lw_o) = (0, 2, 1, 3);
        } else {
            self.back = 0;
            self.shown = 1;
            (self.lr_f, self.lr_o, self.lw_f, self.lw_o) = (1, 3, 0, 2);
        }
        self.call_three_routines();
        self.choose_background_colours();
    }

    /// copy_word_block_to_memory ($16D6): the background into both screens.
    pub fn copy_word_block_to_memory(&mut self) {
        let bg = self.a.background.px.clone();
        self.scr[0].copy_from_slice(&bg);
        self.scr[1].copy_from_slice(&bg);
    }

    /// copy_sprite_attribute_blocks_2 ($A7F4): rows 117 to 199 of the drawn screen back to the background.
    pub fn copy_sprite_attribute_blocks_2(&mut self) {
        let b = self.back;
        let s = 117 * W;
        let bg = &self.a.background.px[s..];
        self.scr[b][s..].copy_from_slice(bg);
    }

    /// draw_into_both_buffers ($A7CE).
    pub fn draw_into_both_buffers(&mut self) {
        let keep = self.back;
        for b in [1, 0] {
            self.back = b;
            self.copy_sprite_attribute_blocks_2();
        }
        self.back = keep;
    }

    /// copy_sprite_attribute_blocks ($87D0): the speech area (rows 117 to 137) of the drawn screen back to the
    /// background.
    pub fn copy_sprite_attribute_blocks(&mut self) {
        let b = self.back;
        let (s, e) = (117 * W, 138 * W);
        let bg = self.a.background.px[s..e].to_vec();
        self.scr[b][s..e].copy_from_slice(&bg);
    }

    /// four_steps_per_frame ($90E0): clears the four collision layers.
    pub fn four_steps_per_frame(&mut self) {
        for l in self.layer.iter_mut() {
            l.fill(0);
        }
    }
}
