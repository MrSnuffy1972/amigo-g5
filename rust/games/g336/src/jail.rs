//! Jail Break (Odyssey, 1990), rewritten natively from its recovered code. A bat-and-ball brick game for one or two
//! players (taking turns), 100 built-in levels of bricks over a background map, falling pills with power-ups,
//! moving bricks, multi-hit silver bricks, a level editor with its own 100 custom levels, and a title screen of
//! switches (players, skill, start level, music and effects).
//!
//! Structure: one struct with typed state. The main program is written the way the original was: nested routines
//! that wait for the next frame wherever the original waited (`refresh` is the original's "wait for the frame, then
//! move and draw everything" routine; `frame` is a bare wait). `co.rs` runs it in lock-step with the engine. Each
//! method keeps the role of one recovered routine and its doc comment names it, so the function map reads across.
//!
//! The picture is kept as the original kept it: a shown screen (`disp`) and a clean copy of it (`work`) that moving
//! things are drawn over and erased from, both 320 by 208 palette-index pictures, plus up to four sprites drawn on top.
//! Nothing here is a model of the original machine: no memory image, processor, chip registers or display hardware.
//!
//! Copy protection removed (31 Oct 2026, Karl: behave as if every check passed): the title's "copy check" (type the
//! word from the manual at a page and line it names, three tries, then the program quits) is skipped as if the first
//! answer had been right. Its one random draw is kept so the random sequence that follows matches the original's.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::gfx::{Argb, Bitmap};
use native_engine::input::{Action, InputState};

use crate::assets::{Assets, BACK_MAP, BRICK_MAP, H, W};
use crate::co::{FrameIn, FrameOut, Snd, Stop};

/// What the frame shows.
#[derive(Debug, Clone)]
pub struct View {
    pub screen: Vec<u8>,
    pub palette: [u16; 32],
    pub direct: Option<Arc<Vec<Argb>>>,
    /// The display window's first line (the wipes move it); the picture's first row is shown on that line.
    pub window_top: u8,
    /// Each sprite's (first line, horizontal position) when it has been placed.
    pub sprites: [Option<(u8, i16)>; 8],
    pub sprite_tiles: [i16; 8],
}

/// A thing on the screen: the bat's pieces are drawn into the picture, balls, the pill, shots and the pointer are
/// sprites.
#[derive(Debug, Clone, Copy, Default)]
struct Ent {
    y: i16,
    x: i16,
    img: i16,
    /// Which sprite shows it.
    spr: usize,
    sprite: bool,
    last_img: i16,
}

/// A ball's (or a shot's) direction and speed: steps a frame up or down and across.
#[derive(Debug, Clone, Copy, Default)]
struct Vel {
    vy: i16,
    vx: i16,
    ys: i16,
    xs: i16,
}

#[derive(Debug, Clone)]
struct Player {
    /// 1 or 2.
    id: i16,
    skill: i16,
    level: i16,
    lives: i16,
    score: i16,
    next_life: i16,
    /// This player's copy of the current level's bricks (26 rows of 13 cells).
    map: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Mover {
    kind: i16,
    y: i16,
    x: i16,
    dx: i16,
}

/// Which picture the display shows.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shown {
    Display,
    Work,
    /// The intro picture (decoded into the tile-sheet buffer).
    Intro,
}

/// Source of a masked or plain block copy.
#[derive(Debug, Clone, Copy)]
enum Src {
    /// The clean picture (`work`), masked by the set pixels it had when the mask was last taken.
    Work,
    /// A tile, masked by its own set pixels.
    Tile(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dst {
    Display,
    Work,
}

// Entities (the original's table of thirteen).
const BAT_L: usize = 0;
const BAT_R: usize = 1;
const BAT_M: usize = 2;
const TOP_L: usize = 3;
const TOP_R: usize = 4;
const TOP_M: usize = 5;
const BALL1: usize = 6;
const PILL: usize = 7;
const SHOT1: usize = 8;
const SHOT2: usize = 9;
const BALL2: usize = 10;
const BALL3: usize = 11;
const POINTER: usize = 12;
// Direction and speed records.
const V_BALL1: usize = 0;
const V_BALL2: usize = 1;
const V_BALL3: usize = 2;
const V_SHOT1: usize = 3;
const V_SHOT2: usize = 4;

// Power-up flags (the original kept them in a data register).
const P_CANNON: u32 = 1 << 1;
const P_MINE: u32 = 1 << 4;
const P_HOLD: u32 = 1 << 6;
const P_KILL: u32 = 1 << 8;
const P_BALLS: u32 = 1 << 9;
const P_WIDE: u32 = 1 << 10;
const P_GIANT: u32 = 1 << 11;
const P_THIN: u32 = 1 << 12;
const P_DOUBLE: u32 = 1 << 13;
const P_REVERSE: u32 = 1 << 14;

// Modes.
const M_TITLE: i16 = 0;
const M_EDITOR: i16 = 1;
const M_PLAY: i16 = 2;

// Keys as the original's key table gave them.
const K_ESCAPE: i16 = 2;
const K_SPACE: i16 = 0x20;

// Sounds named after their use.
const S_CLICK: i16 = 1;

/// The tile numbers of the background-map and editor tiles start here.
const BACK_TILE: i16 = 0x104;

pub struct Jail {
    a: Arc<Assets>,
    rx: Receiver<FrameIn>,
    tx: Sender<FrameOut>,
    input: InputState,
    sounds: Vec<Snd>,
    save_out: Option<(Vec<u8>, Vec<u8>)>,
    quit: bool,
    frames: u32,
    /// The original's time in frames (frame interrupts at whole numbers), and whether an interrupt that came while it
    /// was busy is still waiting to be seen.
    time: f64,
    pending: bool,
    #[cfg(feature = "trace")]
    trace: Option<std::io::BufWriter<std::fs::File>>,
    #[cfg(feature = "trace")]
    last_trace: (u32, &'static str),

    // pictures
    disp: Bitmap,
    work: Bitmap,
    mask: Vec<bool>,
    shown: Shown,
    /// The picture file last read (its palette and its picture are what the decoders use).
    file: &'static str,
    intro_pic: &'static str,
    regs: [u16; 32],
    ham: bool,
    window_top: u8,
    sprites: [Option<(u8, i16)>; 8],
    sprite_tiles: [i16; 8],

    // sound
    counters: [u16; 4],
    sfx_chan: Vec<i16>,
    sfx_cached: Vec<bool>,
    sound_flags: u8,

    // input
    mouse_x: u8,
    mouse_y: u8,
    mouse_last: (u8, u8),
    keys: Vec<i16>,
    last_key: i16,

    // game state
    ents: [Ent; 13],
    vels: [Vel; 5],
    players: [Player; 2],
    cur: usize,
    movers: Vec<Mover>,
    anim: Vec<u8>,
    cust_bricks: Vec<u8>,
    cust_backs: Vec<u8>,
    /// Which level files are in the level buffers: 1 the built-in levels, 2 the custom levels.
    bricks: Vec<u8>,
    backs: Vec<u8>,
    maps_loaded: i16,
    menu_flags: Vec<u8>,
    editor_flags: Vec<u8>,
    pow: u32,
    mode: i16,
    running: i16,
    serve_wait: i16,
    launch_guard: i16,
    lmb_was_up: i16,
    movers_on: i16,
    normal_death: i16,
    cheats_off: i16,
    pill_effect: i16,
    ed_layer: i16,
    ed_brick: i16,
    ed_back: i16,
    ed_page: i16,
    speed: i16,
    planes: i16,
    bat_w: i16,
    bat_img: i16,
    brick_anim_div: i16,
    ball_anim_div: i16,
    hits_to_speedup: i16,
    serve_timer: i16,
    mine_timer: i16,
    countdown: i16,
    credits: i16,
    pitch: i16,
    n_players: i16,
    cust_level: i16,
    start_level: i16,
    bricks_left: i16,
    playing: i16,
    segment: i16,
    ed_running: i16,
    rng: u16,
    button_hit: i16,
    ptr_y: i16,
    ptr_x: i16,
}

macro_rules! tr {
    ($s:expr, $n:expr) => {
        #[cfg(feature = "trace")]
        $s.trace_call($n);
    };
}

fn ent(y: i16, x: i16, sprite: bool, spr: usize) -> Ent {
    Ent {
        y,
        x,
        img: 0,
        spr,
        sprite,
        last_img: 0,
    }
}

impl Jail {
    pub fn new(
        a: Arc<Assets>,
        seed: Option<u32>,
        rx: Receiver<FrameIn>,
        tx: Sender<FrameOut>,
    ) -> Jail {
        let blank = || Player {
            id: 1,
            skill: 0,
            level: 0,
            lives: 0,
            score: 0,
            next_life: 0,
            map: vec![0; BRICK_MAP],
        };
        let mut p2 = blank();
        p2.id = 2;
        let mut ents = [Ent::default(); 13];
        for e in ents.iter_mut().take(6) {
            *e = ent(0, 0, false, 0);
        }
        ents[BALL1] = ent(0, 0, true, 0);
        ents[PILL] = ent(0, 0, true, 2);
        ents[SHOT1] = ent(0, 0, true, 4);
        ents[SHOT2] = ent(0, 0, true, 6);
        ents[BALL2] = ent(0, 0, true, 4);
        ents[BALL3] = ent(0, 0, true, 6);
        ents[POINTER] = ent(0, 0, true, 0);
        let rng = match seed {
            Some(s) => (s as u16).max(1),
            None => {
                let t = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.subsec_nanos())
                    .unwrap_or(1);
                ((t >> 8) as u16).max(1)
            }
        };
        let n_menu = a.menu_buttons.len();
        let n_ed = a.editor_buttons.len();
        let cb = a.cust_bricks.clone();
        let ck = a.cust_backs.clone();
        Jail {
            rx,
            tx,
            input: InputState::default(),
            sounds: Vec::new(),
            save_out: None,
            quit: false,
            frames: 0,
            time: 0.0,
            pending: false,
            #[cfg(feature = "trace")]
            trace: std::env::var("G336_TRACE")
                .ok()
                .and_then(|p| std::fs::File::create(p).ok())
                .map(std::io::BufWriter::new),
            #[cfg(feature = "trace")]
            last_trace: (u32::MAX, ""),
            disp: Bitmap::new(W, H),
            work: Bitmap::new(W, H),
            mask: vec![false; W * H],
            shown: Shown::Intro,
            file: "",
            intro_pic: "",
            regs: [0; 32],
            ham: false,
            window_top: 0xF4,
            sprites: [None; 8],
            sprite_tiles: [0; 8],
            counters: [0; 4],
            sfx_chan: vec![0; 33],
            sfx_cached: vec![false; 33],
            sound_flags: 0,
            mouse_x: 0,
            mouse_y: 0,
            mouse_last: (0, 0),
            keys: Vec::new(),
            last_key: 0,
            ents,
            vels: [Vel::default(); 5],
            players: [blank(), p2],
            cur: 0,
            movers: Vec::new(),
            anim: vec![0; BRICK_MAP],
            cust_bricks: cb,
            cust_backs: ck,
            bricks: vec![0; BRICK_MAP * 100],
            backs: vec![0; BACK_MAP * 100],
            maps_loaded: 0,
            menu_flags: vec![0; n_menu],
            editor_flags: vec![0; n_ed],
            pow: 0,
            mode: 0,
            running: 0,
            serve_wait: 0,
            launch_guard: 0,
            lmb_was_up: 0,
            movers_on: 0,
            normal_death: 0,
            cheats_off: 0,
            pill_effect: 0,
            ed_layer: 0,
            ed_brick: 0,
            ed_back: 0,
            ed_page: 0,
            speed: 0,
            planes: 5,
            bat_w: 0,
            bat_img: 0,
            brick_anim_div: 0,
            ball_anim_div: 0,
            hits_to_speedup: 0,
            serve_timer: 0,
            mine_timer: 0,
            countdown: 0,
            credits: 0,
            pitch: 0,
            n_players: 1,
            cust_level: 0,
            start_level: 0,
            bricks_left: 0,
            playing: 0,
            segment: 0,
            ed_running: 0,
            rng,
            button_hit: 0,
            ptr_y: 0,
            ptr_x: 0,
            a,
        }
    }

    #[cfg(feature = "trace")]
    fn trace_call(&mut self, n: &'static str) {
        use std::io::Write;
        if self.last_trace == (self.frames, n) {
            return;
        }
        self.last_trace = (self.frames, n);
        let f = self.frames;
        if let Some(t) = self.trace.as_mut() {
            let _ = writeln!(t, "{f} {n}");
        }
    }
}

// ------------------------------------------------------------------------------------------------ frames, input, sound
impl Jail {
    /// Runs the whole program: the original's entry routine (initialize_game_state_and_loop).
    pub fn run(&mut self) {
        match self.rx.recv() {
            Ok(i) => self.take_input(i),
            Err(_) => std::panic::resume_unwind(Box::new(Stop)),
        }
        self.initialize_game_state_and_loop();
        self.quit = true;
        let out = self.out();
        let _ = self.tx.send(out);
    }

    fn out(&mut self) -> FrameOut {
        FrameOut {
            view: self.view(),
            sounds: std::mem::take(&mut self.sounds),
            save: self.save_out.take(),
            quit: self.quit,
            state: self.state(),
        }
    }

    /// The original's wait for the frame interrupt (at the display's line 244). When the interrupt already came while
    /// the program was busy (its request still set), the wait returns at once; otherwise the program waits for it.
    fn frame(&mut self) {
        if self.pending {
            self.pending = false;
            return;
        }
        let next = self.time.floor() + 1.0;
        self.pass_time(next);
    }

    /// Work the original took `frames` of a frame to do (measured on the reference run): time passes, and a frame
    /// interrupt that comes meanwhile is left waiting for the next wait.
    fn busy(&mut self, frames: f32) {
        let end = self.time + frames as f64;
        if end.floor() > self.time.floor() {
            self.pending = true;
        }
        self.pass_time(end);
    }

    /// Time passes to `end` (in frames, the frame interrupts at whole numbers). Each frame is shown when the display
    /// starts it (its line 0, LINE0 after the interrupt): what the program changed before that point is in it.
    fn pass_time(&mut self, end: f64) {
        let mut k = (self.time - LINE0).floor() + 1.0;
        while k + LINE0 <= end {
            self.time = k + LINE0;
            self.next_frame();
            k += 1.0;
        }
        self.time = end;
    }

    /// One frame is shown: the vertical-blank work, the frame handed to the engine and the next frame's input read.
    fn next_frame(&mut self) {
        self.decrement_prisoner_counters();
        self.frames += 1;
        let out = self.out();
        if self.tx.send(out).is_err() {
            std::panic::resume_unwind(Box::new(Stop));
        }
        match self.rx.recv() {
            Ok(i) => self.take_input(i),
            Err(_) => std::panic::resume_unwind(Box::new(Stop)),
        }
    }

    fn take_input(&mut self, i: FrameIn) {
        if let Some((b, k)) = i.saved {
            if b.len() == self.cust_bricks.len() && k.len() == self.cust_backs.len() {
                self.cust_bricks = b;
                self.cust_backs = k;
            }
        }
        self.input = i.input;
        let (mut dx, dy) = self.input.motion;
        if let Some((ax, ay)) = self.input.absolute {
            // A touch (or an uncaptured mouse) puts the pointer's hot spot where it is (directly: a jump across the
            // screen is more than the mouse counters can carry in one frame), or brings the bat's middle to it.
            if self.mode == M_PLAY {
                dx += ax - (self.ents[BAT_L].x + self.bat_w / 2) as i32;
            } else {
                let p = &mut self.ents[POINTER];
                p.x = (ax - 7).clamp(0, 0x130) as i16;
                p.y = (ay - 7).clamp(0, 0xc0) as i16;
                self.ptr_y = p.y + 7;
                self.ptr_x = p.x + 7;
            }
        }
        self.mouse_x = self.mouse_x.wrapping_add(dx.clamp(-127, 127) as u8);
        self.mouse_y = self.mouse_y.wrapping_add(dy.clamp(-127, 127) as u8);
        if self.input.pressed(Action::Start) {
            self.keys.push(K_SPACE);
        }
        if self.input.pressed(Action::Cancel) {
            self.keys.push(K_ESCAPE);
        }
    }

    /// The left mouse button (the original read it from the port each time).
    fn lmb(&self) -> bool {
        self.input.held(Action::Primary)
    }

    fn rmb(&self) -> bool {
        self.input.held(Action::Secondary)
    }

    fn view(&self) -> View {
        let (screen, direct) = match self.shown {
            Shown::Display => (self.disp.px.clone(), None),
            Shown::Work => (self.work.px.clone(), None),
            Shown::Intro => {
                if self.intro_pic.is_empty() {
                    (vec![0; W * H], None)
                } else if self.ham && self.intro_pic == "jailintro1" {
                    (vec![0; W * H], Some(self.a.intro_direct.clone()))
                } else {
                    (self.a.picture(self.intro_pic).px.clone(), None)
                }
            }
        };
        View {
            screen,
            palette: self.regs,
            direct,
            window_top: self.window_top,
            sprites: self.sprites,
            sprite_tiles: self.sprite_tiles,
        }
    }

    /// The named state the verification compares with the original's memory.
    fn state(&self) -> Vec<(String, i64)> {
        let mut v: Vec<(String, i64)> = Vec::new();
        let mut put = |k: &str, x: i64| v.push((k.to_string(), x));
        put("mode", self.mode as i64);
        put("ptr_y", self.ents[POINTER].y as i64);
        put("ptr_x", self.ents[POINTER].x as i64);
        put("bat_x", self.ents[BAT_L].x as i64);
        put("bat_w", self.bat_w as i64);
        for (n, i) in [
            ("ball1", BALL1),
            ("ball2", BALL2),
            ("ball3", BALL3),
            ("pill", PILL),
            ("shot1", SHOT1),
        ] {
            put(&format!("{n}_y"), self.ents[i].y as i64);
            put(&format!("{n}_x"), self.ents[i].x as i64);
            put(&format!("{n}_img"), self.ents[i].img as i64);
        }
        for (i, p) in self.players.iter().enumerate() {
            put(&format!("p{}_score", i + 1), p.score as u16 as i64);
            put(&format!("p{}_lives", i + 1), p.lives as i64);
            put(&format!("p{}_level", i + 1), p.level as i64);
        }
        put("speed", self.speed as i64);
        put("serve_wait", self.serve_wait as i64);
        put("serve_timer", self.serve_timer as i64);
        put("bricks_left", self.bricks_left as i64);
        put("playing", self.playing as i64);
        put("countdown", self.countdown as i64);
        put("credits", self.credits as i64);
        put("pow", self.pow as i64);
        put("rng", self.rng as i64);
        put("window_top", self.window_top as i64);
        put("cur", self.cur as i64);
        v
    }

    /// update_prisoner_state_2: the game's random number generator, a draw below `n`.
    fn random(&mut self, n: u16) -> i16 {
        tr!(self, "update_prisoner_state_2");
        let q = (self.rng as u32 / 0x7c4c) as u16;
        let v = (self.rng ^ q).rotate_right(7) ^ 0x3d7c;
        self.rng = v;
        (v % n.max(1)) as i16
    }

    /// decrement_prisoner_counters (run by the vertical-blank interrupt): each channel's frame count runs down and the
    /// channel stops at zero.
    fn decrement_prisoner_counters(&mut self) {
        tr!(self, "handle_vblank_interrupt");
        tr!(self, "decrement_prisoner_counters");
        for ch in (0..4).rev() {
            if self.counters[ch] == 0 {
                if !self.sounds.contains(&Snd::Stop(ch)) {
                    self.sounds.push(Snd::Stop(ch));
                }
            } else {
                self.counters[ch] -= 1;
            }
        }
    }

    /// set_sprite_dma: plays sound `n` on the first free channel, if its switch (music or effects) is on. A sound in a
    /// file of its own is read into the shared buffer first (clear_player_collision_flags). The channel repeats the
    /// sample until its frame count runs out.
    fn sfx(&mut self, n: i16) {
        tr!(self, "set_sprite_dma");
        let n = n as usize;
        let (music, own, dur) = {
            let e = &self.a.sfx[n];
            (e.music, e.own_file, e.duration)
        };
        if self.sound_flags & (1 << music as u8) == 0 {
            return;
        }
        let Some(ch) = (0..4).find(|&c| self.counters[c] == 0) else {
            return;
        };
        if own && !self.sfx_cached[n] {
            self.clear_player_collision_flags();
            self.sfx_cached[n] = true;
        }
        self.counters[ch] = dur;
        self.sfx_chan[n] = ch as i16;
        self.sounds.retain(|s| *s != Snd::Stop(ch));
        self.sounds.push(Snd::Play { ch, sfx: n });
    }

    /// clear_player_collision_flags: reads a sound file into the shared buffer; no other file sound is in it any more.
    fn clear_player_collision_flags(&mut self) {
        tr!(self, "clear_player_collision_flags");
        self.init_game_state_file();
        for c in self.sfx_cached.iter_mut() {
            *c = false;
        }
    }

    /// clear_sprite_buffers: stops every channel.
    fn clear_sprite_buffers(&mut self) {
        tr!(self, "clear_sprite_buffers");
        self.counters = [0; 4];
        for ch in 0..4 {
            self.sounds
                .retain(|s| !matches!(s, Snd::Play { ch: c, .. } if *c == ch));
            if !self.sounds.contains(&Snd::Stop(ch)) {
                self.sounds.push(Snd::Stop(ch));
            }
        }
        self.refresh();
    }

    /// set_player_sprite: meant to change the pitch of the level's music on every bat hit; the original wrote the new
    /// period to a register next to the channel's period register that does nothing, so nothing happens.
    fn set_player_sprite(&mut self, _period: i16, _sound: i16) {
        tr!(self, "set_player_sprite");
    }

    /// poll_cia_timer: the next key pressed (0 when none), with its click.
    fn poll_cia_timer(&mut self) -> i16 {
        tr!(self, "poll_cia_timer");
        if self.keys.is_empty() {
            return 0;
        }
        let k = self.keys.remove(0);
        self.last_key = k;
        self.sfx(S_CLICK);
        k
    }
}

// ------------------------------------------------------------------------------------------------ pictures and drawing
impl Jail {
    fn dst(&mut self, d: Dst) -> &mut Bitmap {
        match d {
            Dst::Display => &mut self.disp,
            Dst::Work => &mut self.work,
        }
    }

    /// init_game_state: reads a file (Open, Read, Close). A picture file stays in the file buffer for the decoders.
    fn init_game_state(&mut self, name: &'static str) {
        tr!(self, "init_game_state");
        self.file = name;
        self.busy(load_cost(name));
    }

    /// init_game_state for a sound file (it goes to the shared sound buffer, not the picture file buffer).
    fn init_game_state_file(&mut self) {
        tr!(self, "init_game_state");
    }

    /// wait_for_game_over_flag: decodes the picture in the file buffer into a picture buffer.
    fn wait_for_game_over_flag(&mut self, d: Dst) {
        tr!(self, "wait_for_game_over_flag");
        tr!(self, "parse_script_command");
        let src = self.a.picture(self.file).px.clone();
        self.dst(d).px.copy_from_slice(&src);
        self.busy(decode_cost(self.file));
    }

    /// wait_for_game_over_flag into the tile-sheet buffer (the intro pictures).
    fn decode_into_sheet(&mut self) {
        tr!(self, "wait_for_game_over_flag");
        tr!(self, "parse_script_command");
        self.intro_pic = self.file;
        self.busy(decode_cost(self.file));
    }

    /// decode_map_tile_row: the picture's colours become the screen's.
    fn decode_map_tile_row(&mut self) {
        tr!(self, "decode_map_tile_row");
        let p = &self.a.palettes[self.file];
        self.regs.copy_from_slice(&p[..32]);
        self.busy(0.036);
    }

    /// accumulate_tile_checksums: the mask of the clean picture's set pixels, used by the masked copies from it.
    fn accumulate_tile_checksums(&mut self) {
        tr!(self, "accumulate_tile_checksums");
        for (m, p) in self.mask.iter_mut().zip(&self.work.px) {
            *m = *p != 0;
        }
        self.busy(MASK_COST);
    }

    /// Loads a picture file, sets its colours if asked, decodes it into `d` and takes its mask if asked: the sequence
    /// the original repeats for every screen.
    fn load_picture(&mut self, name: &'static str, colours: bool, d: Dst, mask: bool) {
        self.init_game_state(name);
        if colours {
            self.decode_map_tile_row();
        }
        self.wait_for_game_over_flag(d);
        if mask {
            self.accumulate_tile_checksums();
        }
    }

    /// The masked block copy (configure_and_run_bob_blitter): the source's set pixels go onto the destination at any
    /// position; the source's left edge is on a 16-pixel boundary.
    #[allow(clippy::too_many_arguments)]
    fn masked(&mut self, src: Src, sy: i16, sx: i16, d: Dst, dy: i16, dx: i16, h: i16, w: i16) {
        tr!(self, "configure_and_run_bob_blitter");
        let sx = sx as i32 & !15;
        for r in 0..h as i32 {
            for i in 0..w as i32 {
                let (x, y) = (sx + i, sy as i32 + r);
                let v = match src {
                    Src::Work => {
                        if x < 0 || y < 0 || x >= W as i32 || y >= H as i32 {
                            continue;
                        }
                        let k = y as usize * W + x as usize;
                        if !self.mask[k] {
                            continue;
                        }
                        self.work.px[k]
                    }
                    Src::Tile(t) => {
                        if !(0..16).contains(&x) || !(0..16).contains(&y) {
                            continue;
                        }
                        let v = self.a.tiles[t].at(x as usize, y as usize);
                        if v == 0 {
                            continue;
                        }
                        v
                    }
                };
                let (tx, ty) = (dx as i32 + i, dy as i32 + r);
                self.dst(d).set(tx, ty, v);
            }
        }
    }

    /// The plain block copy (animate_prisoner_sprite): a rectangle copied as it is, both edges on 16-pixel boundaries.
    #[allow(clippy::too_many_arguments)]
    fn plain(&mut self, src: Src, sy: i16, sx: i16, d: Dst, dy: i16, dx: i16, h: i16, w: i16) {
        tr!(self, "animate_prisoner_sprite");
        let (sx, dx) = (sx as i32 & !15, dx as i32 & !15);
        for r in 0..h as i32 {
            for i in 0..w as i32 {
                let (x, y) = (sx + i, sy as i32 + r);
                let v = match src {
                    Src::Work => self.work.get(x, y),
                    Src::Tile(t) => {
                        if !(0..16).contains(&x) || !(0..16).contains(&y) {
                            continue;
                        }
                        self.a.tiles[t].at(x as usize, y as usize)
                    }
                };
                self.dst(d).set(dx + i, dy as i32 + r, v);
            }
        }
    }

    /// draw_bob_sprite: a whole tile, opaque, onto the screen (on a 16-pixel boundary).
    fn draw_bob_sprite(&mut self, y: i16, x: i16, tile: i16) {
        tr!(self, "draw_bob_sprite");
        self.plain(
            Src::Tile(tile_index(tile)),
            0,
            0,
            Dst::Display,
            y,
            x,
            16,
            16,
        );
    }

    /// draw_player_sprite: the top half of a tile over the clean picture onto the screen (a brick, or a frame of a
    /// brick's hit animation).
    fn draw_player_sprite(&mut self, y: i16, x: i16, tile: i16) {
        tr!(self, "draw_player_sprite");
        let t = tile_index(tile);
        let x0 = x as i32 & !15;
        for r in 0..8i32 {
            for i in 0..16i32 {
                let v = self.a.tiles[t].at(i as usize, r as usize);
                let (tx, ty) = (x0 + i, y as i32 + r);
                let v = if v != 0 { v } else { self.work.get(tx, ty) };
                self.disp.set(tx, ty, v);
            }
        }
    }

    /// draw_background_tile: a moving brick, at any position: the top half of its tile without its outer columns over
    /// the clean picture, which also fills the rest of the 32 pixels it touches (erasing its old place).
    fn draw_background_tile(&mut self, y: i16, x: i16, tile: i16) {
        tr!(self, "draw_background_tile");
        let t = tile_index(tile);
        let x0 = x as i32 & !15;
        let s = x as i32 & 15;
        for r in 0..8i32 {
            for i in 0..32i32 {
                let j = i - s;
                let (tx, ty) = (x0 + i, y as i32 + r);
                let v = if (1..15).contains(&j) && self.a.tiles[t].at(j as usize, r as usize) != 0 {
                    self.a.tiles[t].at(j as usize, r as usize)
                } else {
                    self.work.get(tx, ty)
                };
                self.disp.set(tx, ty, v);
            }
        }
    }

    /// copy_tilemap_row: one 8 by 8 character in colour `colour` on colour 0, on an 8-pixel boundary.
    fn copy_tilemap_row(&mut self, y: i16, x: i16, ch: i16, colour: i16) {
        tr!(self, "copy_tilemap_row");
        let g = (ch - 0x20) as usize;
        let glyph = self.a.font.get(g).copied().unwrap_or([0; 8]);
        let x0 = x as i32 & !7;
        for (r, row) in glyph.iter().enumerate() {
            for b in 0..8 {
                let on = row & (0x80 >> b) != 0;
                self.disp.set(
                    x0 + b,
                    y as i32 + r as i32,
                    if on { (colour & 31) as u8 } else { 0 },
                );
            }
        }
    }

    /// process_input_string: a text, one character after another.
    fn process_input_string(&mut self, text: &str, y: i16, x: i16, colour: i16) {
        tr!(self, "process_input_string");
        let mut x = x;
        for c in text.bytes() {
            self.copy_tilemap_row(y, x, c as i16, colour);
            x += 8;
        }
    }

    /// init_player_state_2: a rectangle of the clean picture, masked, onto the screen.
    #[allow(clippy::too_many_arguments)]
    fn init_player_state_2(&mut self, sy: i16, sx: i16, dy: i16, dx: i16, h: i16, w: i16) {
        tr!(self, "init_player_state_2");
        self.masked(Src::Work, sy, sx, Dst::Display, dy, dx, h, w);
    }

    /// reset_game_state: the whole clean picture, masked, onto the screen.
    fn reset_game_state(&mut self) {
        tr!(self, "reset_game_state");
        self.masked(Src::Work, 0, 0, Dst::Display, 0, 0, H as i16, W as i16);
        self.busy(FULL_BLIT_COST);
    }

    /// reset_player_state_2_2: the screen becomes the clean picture.
    fn reset_player_state_2_2(&mut self) {
        tr!(self, "reset_player_state_2_2");
        self.work.px.copy_from_slice(&self.disp.px);
        self.busy(SCREEN_COPY_COST);
    }

    /// wait_one_second (misnamed by the naming pass): a two-digit number in the digit tiles.
    fn wait_one_second(&mut self, y: i16, x: i16, n: i16) {
        tr!(self, "wait_one_second");
        let mut v = n as u16 as u32;
        let mut x = x;
        for _ in 0..2 {
            let d = (v % 10) as i16;
            v /= 10;
            self.draw_bob_sprite(y, x, d + 0x28);
            x -= 16;
        }
    }

    /// initialize_prisoner_positions: which picture the display shows.
    fn initialize_prisoner_positions(&mut self, s: Shown) {
        tr!(self, "initialize_prisoner_positions");
        self.shown = s;
    }
}

/// Tile number to tile index (the sheets hold 520).
fn tile_index(t: i16) -> usize {
    (t as u16 as usize).min(crate::assets::TILES - 1)
}

// Timing (measured on the reference run of the original, 1 Nov 2026: frames from a routine's start to the next routine,
// at 142102 processor cycles a frame). File reads cost nothing (the reference's OS reads files at once, and the native
// build has no disk); decoding a picture, masking, cutting tiles and large block copies took the original frames.

/// Frames the original spent reading each file.
fn load_cost(_name: &str) -> f32 {
    0.0
}

/// Frames the original spent decoding each picture (about 27, more for busier pictures, 34 for the six-plane title).
fn decode_cost(name: &str) -> f32 {
    match name {
        "jailintro1" => 34.453,
        "jailintro2" => 28.577,
        "jailtiles1" => 27.057,
        "jailtiles2" => 28.351,
        "jailtitle1" => 29.122,
        "jailtitle2" => 27.66,
        "jailgadgets1" => 26.88,
        "jailgadgets2" => 26.655,
        "jailplayer1" => 26.408,
        // Not reached by the reference run; about the same size as the first player picture.
        "jailplayer2" => 26.45,
        "jailover1" => 28.539,
        "jailover2" => 28.657,
        "jailover3" => 26.704,
        _ => 27.0,
    }
}

/// Where the display starts a frame (its line 0), in frames after the frame interrupt (line 244 of 313).
const LINE0: f64 = 69.0 / 313.0;
const MASK_COST: f32 = 7.305;
const FULL_BLIT_COST: f32 = 1.252;
const SCREEN_COPY_COST: f32 = 0.601;
const BACKGROUND_COST: f32 = 1.568;
const BORDER_COST: f32 = 0.54;
const TOP_WALL_COST: f32 = 0.119;
const PANEL_COST: f32 = 0.269;
const BUTTONS_COST: f32 = 0.584;
const SCORES_COST: f32 = 0.396;

// ------------------------------------------------------------------------------------------------ start-up and the title
impl Jail {
    /// initialize_game_state_and_loop: the program. (The Workbench start-up message handling and the memory the
    /// original allocated and freed, run_level_script and process_game_objects, are the engine's job.)
    fn initialize_game_state_and_loop(&mut self) {
        tr!(self, "initialize_game_state_and_loop");
        tr!(self, "run_level_script");
        self.initialize_jail_state();
        self.running = -1;
        self.initialize_game_state();
        self.run_game_loop_3();
        self.close_game_libraries();
        tr!(self, "process_game_objects");
    }

    /// run_game_loop_3: the title screen's loop, which starts a game or the editor and comes back to the title.
    fn run_game_loop_3(&mut self) {
        tr!(self, "run_game_loop_3");
        loop {
            self.refresh();
            self.decrement_timer_and_trigger_event();
            self.update_prisoner_flags(false);
            if self.mode == M_PLAY {
                self.initialize_level_state();
                self.wait_vblank_and_refresh_loop();
                self.initialize_game_state();
            } else if self.mode == M_EDITOR {
                self.init_jail_custom_level();
                self.run_game_loop();
                self.initialize_game_state();
            } else if self.running == 0 {
                return;
            }
        }
    }

    /// initialize_jail_state: start-up: the intro, the tiles, the font and (originally) the copy check.
    /// open_dos_library, wait_for_timer_tick (the random seed, taken in `new`) and fill_bullet_positions (the sprites'
    /// buffers) are the engine's job.
    fn initialize_jail_state(&mut self) {
        tr!(self, "initialize_jail_state");
        tr!(self, "open_dos_library");
        tr!(self, "wait_for_timer_tick");
        // The random seed was taken from a timer with interrupts off: almost a frame (measured 0.99).
        self.busy(0.99);
        tr!(self, "fill_bullet_positions");
        self.clear_enemy_targets();
        self.show_jail_intro_sequence();
        self.load_jail_tile_graphics();
        self.copy_character_graphics();
        self.handle_jail_title_screen();
    }

    /// clear_enemy_targets: music and effects on (the sounds in files of their own share one buffer).
    fn clear_enemy_targets(&mut self) {
        tr!(self, "clear_enemy_targets");
        self.sound_flags |= 3;
    }

    /// show_jail_intro_sequence: the publisher's title (a hold-and-modify picture) and the credits, with their sounds.
    fn show_jail_intro_sequence(&mut self) {
        tr!(self, "show_jail_intro_sequence");
        self.clear_input_buffer();
        self.initialize_prisoner_positions(Shown::Intro);
        self.planes = 6;
        self.init_game_state("jailintro1");
        self.decode_map_tile_row();
        self.decode_into_sheet();
        self.ham = true;
        self.wait_for_input_2();
        self.sfx(0x1f);
        self.sfx(0x1f);
        self.process_sfx_jail_general();
        self.clear_input_buffer();
        self.planes = 5;
        self.init_game_state("jailintro2");
        self.decode_map_tile_row();
        self.decode_into_sheet();
        self.ham = false;
        self.wait_for_input_2();
        self.sfx(0);
        self.sfx(0);
        for _ in 0..=0xf0 {
            self.refresh();
        }
        self.clear_input_buffer();
        self.initialize_prisoner_positions(Shown::Display);
    }

    /// process_sfx_jail_general: reads the effects file (each sound into its own buffer).
    fn process_sfx_jail_general(&mut self) {
        tr!(self, "process_sfx_jail_general");
        self.busy(SFX_LOAD_COST);
    }

    /// load_jail_tile_graphics: the two tile sheets, cut into tiles with their masks (wait_for_input_frames,
    /// init_player_state_5, transpose_tilemap_rows: done when the assets load).
    fn load_jail_tile_graphics(&mut self) {
        tr!(self, "load_jail_tile_graphics");
        self.load_picture("jailtiles1", false, Dst::Work, false);
        tr!(self, "wait_for_input_frames");
        self.busy(SLICE_COST);
        self.load_picture("jailtiles2", false, Dst::Work, false);
        tr!(self, "wait_for_input_frames");
        self.busy(SLICE_COST);
        tr!(self, "transpose_tilemap_rows");
        self.busy(TILE_MASK_COST);
    }

    /// copy_character_graphics: the font. The file it names is not on the game's disk, so the read fails and the
    /// picture still in the file buffer (the second tile sheet) is decoded again; the font comes from it (see assets).
    fn copy_character_graphics(&mut self) {
        tr!(self, "copy_character_graphics");
        tr!(self, "init_game_state");
        self.wait_for_game_over_flag(Dst::Work);
        self.busy(FONT_COST);
    }

    /// handle_jail_title_screen: the copy check, removed (see the module comment): behaves as if the first word typed
    /// was right. Its one random draw (which word to ask for) is kept.
    fn handle_jail_title_screen(&mut self) {
        tr!(self, "handle_jail_title_screen");
        self.cheats_off = -1;
        tr!(self, "scramble_level_data");
        tr!(self, "draw_prisoner_sprite");
        self.random(0xb4);
        tr!(self, "scramble_level_data_2");
    }

    /// close_game_libraries: the end of the program: sounds off (the rest, giving the machine back to the OS with
    /// iterate_entity_callbacks and close_graphics_library, is the engine's job).
    fn close_game_libraries(&mut self) {
        tr!(self, "close_game_libraries");
        tr!(self, "iterate_entity_callbacks");
        tr!(self, "close_graphics_library");
        self.counters = [0; 4];
        for ch in 0..4 {
            self.sounds.push(Snd::Stop(ch));
        }
    }

    /// decrement_timer_counters: a fade to black over 16 steps of about half a second (every colour component one
    /// step darker each time). Nothing in the original calls it; kept so the whole program is here.
    #[allow(dead_code)]
    fn decrement_timer_counters(&mut self) {
        tr!(self, "decrement_timer_counters");
        for _ in 0..16 {
            for _ in 0..=25 {
                self.refresh();
            }
            for c in self.regs.iter_mut() {
                for sh in [0, 4, 8] {
                    if (*c >> sh) & 15 != 0 {
                        *c -= 1 << sh;
                    }
                }
            }
        }
    }

    /// clear_input_buffer: the screen slides down out of the window, 8 lines a frame.
    fn clear_input_buffer(&mut self) {
        tr!(self, "clear_input_buffer");
        self.window_top = 0x24;
        for _ in 0..=25 {
            self.refresh();
            self.window_top = self.window_top.wrapping_add(8);
        }
    }

    /// wait_for_input_2: the screen slides up into the window, 8 lines a frame.
    fn wait_for_input_2(&mut self) {
        tr!(self, "wait_for_input_2");
        self.window_top = 0xF4;
        for _ in 0..=25 {
            self.refresh();
            self.window_top = self.window_top.wrapping_sub(8);
        }
    }

    /// clear_entity_slots: every moving thing off.
    fn clear_entity_slots(&mut self) {
        tr!(self, "clear_entity_slots");
        for e in self.ents.iter_mut() {
            e.img = 0;
        }
        self.refresh();
    }

    /// initialize_game_state: the title screen.
    fn initialize_game_state(&mut self) {
        tr!(self, "initialize_game_state");
        self.mode = M_TITLE;
        self.clear_entity_slots();
        self.clear_input_buffer();
        self.load_picture("jailtitle2", true, Dst::Work, true);
        self.reset_game_state();
        self.load_picture("jailtitle2", false, Dst::Work, true);
        self.reset_game_state();
        self.load_picture("jailgadgets1", false, Dst::Work, true);
        self.sfx(0x20);
        self.sfx(0x20);
        self.wait_for_input_2();
        self.ents[POINTER].img = 0xfe;
        self.clear_player_flags(false);
        self.wait_for_player_input_3();
        self.check_game_over_2();
        self.check_game_over();
        self.wait_one_second(0x90, 0x30, self.start_level);
        self.wait_one_second(0x90, 0x110, self.cust_level);
    }

    /// decrement_timer_and_trigger_event: the title's countdown (after a game over with credits left the next game may
    /// continue from the level reached) and the credits.
    fn decrement_timer_and_trigger_event(&mut self) {
        tr!(self, "decrement_timer_and_trigger_event");
        if self.countdown != 0 {
            self.countdown = self.countdown.wrapping_sub(1);
            if self.countdown <= 0 {
                self.start_level = 0;
                self.credits = 0;
                self.wait_one_second(0x90, 0x30, self.start_level);
            }
        }
        self.wait_one_second(0x60, 0x30, (self.countdown as u16 / 60) as i16);
        self.wait_one_second(0x60, 0x110, self.credits);
    }

    fn buttons(&self, editor: bool) -> &[crate::assets::Button] {
        if editor {
            &self.a.editor_buttons
        } else {
            &self.a.menu_buttons
        }
    }

    fn flags(&mut self, editor: bool) -> &mut Vec<u8> {
        if editor {
            &mut self.editor_flags
        } else {
            &mut self.menu_flags
        }
    }

    /// update_prisoner_flags: the buttons: while the left button is held the one under the pointer shows pressed;
    /// on release the last one pressed acts.
    fn update_prisoner_flags(&mut self, editor: bool) {
        tr!(self, "update_prisoner_flags");
        let n = self.buttons(editor).len();
        if self.lmb() {
            self.button_hit = 0;
            for i in 0..n {
                let b = self.buttons(editor)[i];
                let (py, px) = (self.ptr_y, self.ptr_x);
                let inside = b.y <= py && b.y + b.h > py && b.x <= px && b.x + b.w > px;
                if inside {
                    self.flags(editor)[i] |= 2;
                    self.init_enemy_sprite_data(editor, i);
                    self.flags(editor)[i] |= 1;
                    self.button_hit = b.id;
                } else if self.flags(editor)[i] & 1 != 0 {
                    self.flags(editor)[i] &= !3;
                    self.init_enemy_sprite_data(editor, i);
                    self.wait_for_player_input_3();
                    self.check_game_over_2();
                    self.check_game_over();
                }
            }
        } else {
            let k = self.button_hit - 1;
            if k < 0 || k as usize >= n {
                return;
            }
            self.flags(editor)[k as usize] &= !2;
            self.init_enemy_sprite_data(editor, k as usize);
            self.sfx(S_CLICK);
            self.choose_game_mode();
        }
    }

    /// init_enemy_sprite_data: a button drawn pressed or released (from the button picture in the clean buffer).
    fn init_enemy_sprite_data(&mut self, editor: bool, i: usize) {
        tr!(self, "init_enemy_sprite_data");
        let b = self.buttons(editor)[i];
        let (sy, sx) = if self.flags(editor)[i] & 2 != 0 {
            b.on
        } else {
            b.off
        };
        self.masked(Src::Work, sy, sx, Dst::Display, b.y, b.x, b.h, b.w);
    }

    /// clear_player_flags / clear_player_flags_2: every button of the editor or the title drawn released.
    fn clear_player_flags(&mut self, editor: bool) {
        if editor {
            tr!(self, "clear_player_flags");
        } else {
            tr!(self, "clear_player_flags_2");
        }
        for i in 0..self.buttons(editor).len() {
            self.flags(editor)[i] &= !2;
            self.init_enemy_sprite_data(editor, i);
        }
        // Measured for the title's 16 buttons; the editor's 7 in proportion.
        self.busy(BUTTONS_COST * self.buttons(editor).len() as f32 / 16.0);
    }

    /// choose_game_mode: the editor's buttons or the title's.
    fn choose_game_mode(&mut self) {
        tr!(self, "choose_game_mode");
        if self.mode == M_EDITOR {
            self.handle_custody_state();
        } else {
            self.handle_input_command();
        }
    }

    /// The built-in levels into the level buffers (unless they are there).
    fn load_regular_maps(&mut self) {
        if self.maps_loaded != 1 {
            self.init_game_state("jailregbrick");
            self.bricks.copy_from_slice(&self.a.reg_bricks);
            self.init_game_state("jailregback");
            self.backs.copy_from_slice(&self.a.reg_backs);
        }
        self.maps_loaded = 1;
    }

    /// The custom levels into the level buffers (from the saved files).
    fn load_custom_maps(&mut self) {
        self.init_game_state("jailcustbrick");
        self.bricks.copy_from_slice(&self.cust_bricks);
        self.init_game_state("jailcustback");
        self.backs.copy_from_slice(&self.cust_backs);
    }

    /// handle_input_command: what a title button does.
    fn handle_input_command(&mut self) {
        tr!(self, "handle_input_command");
        if self.lmb() {
            return;
        }
        match self.button_hit {
            1 | 2 => {
                self.n_players = self.button_hit;
                self.check_game_over_2();
            }
            3 => self.mode = M_EDITOR,
            4 => {
                self.clear_entity_slots();
                self.players[0].level = self.start_level;
                self.players[1].level = self.start_level;
                if self.credits == 0 {
                    self.credits = 3;
                }
                self.cur = 1;
                self.load_regular_maps();
                self.mode = M_PLAY;
            }
            5 => {
                self.mode = M_PLAY;
                self.players[0].level = self.cust_level;
                self.players[1].level = self.cust_level;
                self.cur = 1;
                self.clear_entity_slots();
                self.start_level = 0;
                self.credits = 0;
                if self.maps_loaded != 2 {
                    self.load_custom_maps();
                }
                self.maps_loaded = 2;
                self.mode = M_PLAY;
            }
            6..=8 => {
                self.cur = 0;
                self.players[0].skill = self.button_hit - 6;
                self.wait_for_player_input_3();
            }
            9..=11 => {
                self.cur = 1;
                self.players[1].skill = self.button_hit - 9;
                self.wait_for_player_input_3();
            }
            12..=14 => {
                self.start_level = (self.button_hit - 12) * 25;
                self.wait_one_second(0x90, 0x30, self.start_level);
            }
            15 => {
                self.sound_flags ^= 2;
                self.clear_sprite_buffers();
                self.check_game_over();
                self.sfx(0x20);
                self.sfx(0x20);
            }
            16 => {
                self.sound_flags ^= 1;
                self.check_game_over();
            }
            _ => {}
        }
        self.button_hit = 0;
    }

    /// check_game_over_2 (misnamed by the naming pass): the one-player and two-player switches' lamps.
    fn check_game_over_2(&mut self) {
        tr!(self, "check_game_over_2");
        if self.mode != M_TITLE {
            return;
        }
        let a = if self.n_players == 1 { 0xc0 } else { 0x80 };
        self.init_player_state_2(0x80, a, 0x40, 0x10, 0x10, 0x40);
        let b = if self.n_players == 2 { 0xc0 } else { 0x80 };
        self.init_player_state_2(0x90, b, 0x40, 0xf0, 0x10, 0x40);
    }

    /// check_game_over (misnamed): the music and effects switches.
    fn check_game_over(&mut self) {
        tr!(self, "check_game_over");
        if self.mode != M_TITLE {
            return;
        }
        let a = if self.sound_flags & 2 != 0 {
            0x40
        } else {
            0x20
        };
        self.init_player_state_2(0x40, a, 0xc0, 0x50, 0x10, 0x20);
        let b = if self.sound_flags & 1 != 0 {
            0x80
        } else {
            0x60
        };
        self.init_player_state_2(0x40, b, 0xc0, 0xd0, 0x10, 0x20);
    }

    /// wait_for_player_input_3 (misnamed): both players' skill lamps.
    fn wait_for_player_input_3(&mut self) {
        tr!(self, "wait_for_player_input_3");
        if self.mode != M_TITLE {
            return;
        }
        for (p, x) in [(0usize, 0x60i16), (1, 0xd0)] {
            for y in [0x60, 0x70, 0x80] {
                self.init_player_state_2(0x40, 0x10, y, x, 0x10, 0x10);
            }
            let y = self.players[p].skill.wrapping_mul(16).wrapping_add(0x60);
            self.init_player_state_2(0x40, 0, y, x, 0x10, 0x10);
        }
        // The original left its current-player pointer on player two (the border style follows it).
        self.cur = 1;
    }
}

const SFX_LOAD_COST: f32 = 0.051;
const SLICE_COST: f32 = 4.883;
const TILE_MASK_COST: f32 = 21.56;
const FONT_COST: f32 = 0.3;

// ------------------------------------------------------------------------------------------------ the level editor
impl Jail {
    fn brick_cell(&self, y: i16, x: i16) -> usize {
        self.cust_level as usize * BRICK_MAP
            + (y as u16 >> 3) as usize * 13
            + ((x - 0x60) as u16 >> 4) as usize
    }

    fn back_cell(&self, y: i16, x: i16) -> usize {
        self.cust_level as usize * BACK_MAP
            + (y as u16 >> 4) as usize * 13
            + ((x - 0x60) as u16 >> 4) as usize
    }

    /// init_jail_custom_level: the editor's screen.
    fn init_jail_custom_level(&mut self) {
        tr!(self, "init_jail_custom_level");
        self.ed_running = -1;
        self.ed_layer = 0;
        self.clear_entity_slots();
        self.clear_input_buffer();
        if self.maps_loaded != 2 {
            self.load_custom_maps();
        }
        self.maps_loaded = 2;
        self.countdown = 0;
        self.credits = 0;
        self.start_level = 0;
        self.ed_brick = 0;
        self.ed_back = 0;
        self.load_picture("jailgadgets2", false, Dst::Work, true);
        self.reset_player_state_2();
        self.clear_player_flags(true);
        self.fill_level_tilemap();
        self.decode_map_data();
        self.wait_frames_2();
        self.decode_level_map();
        self.wait_one_second(0x30, 0x30, self.cust_level);
        self.clear_pill();
        self.init_game_state("jailtiles2");
        self.decode_map_tile_row();
        self.set_interrupt_mask();
        self.wait_for_input_2();
        self.clear_sprite_buffers();
        self.ents[POINTER].img = 0xfe;
    }

    /// run_game_loop: the editor's loop.
    fn run_game_loop(&mut self) {
        tr!(self, "run_game_loop");
        loop {
            self.refresh();
            self.check_player_input();
            self.handle_player_input();
            self.update_prisoner_flags(true);
            if self.ed_running != -1 {
                return;
            }
        }
    }

    /// handle_custody_state: what an editor button does.
    fn handle_custody_state(&mut self) {
        tr!(self, "handle_custody_state");
        if self.lmb() {
            return;
        }
        match self.button_hit {
            1 => {
                self.ed_layer ^= -1;
                if self.ed_layer != 0 {
                    self.draw_background_tiles();
                    self.wait_frames_2();
                    self.trigger_game_over_and_reset();
                } else {
                    self.fill_level_tilemap();
                    self.wait_frames_2();
                    self.decode_level_map();
                    self.clear_pill();
                }
            }
            2 => {
                self.clear_game_buffers();
                self.wait_frames_2();
                self.decode_level_map();
            }
            3 => {
                self.clear_entity_slots();
                self.load_custom_maps();
                self.set_interrupt_mask();
                self.ents[POINTER].img = 0xfe;
                self.wait_frames_2();
                self.decode_level_map();
            }
            4 => {
                self.clear_entity_slots();
                self.handle_jail_customer();
                self.set_interrupt_mask();
                self.ents[POINTER].img = 0xfe;
            }
            5 => self.ed_running = 0,
            6 => {
                if self.ed_layer != 0 {
                    self.decrement_timer_and_trigger_event_3();
                } else {
                    self.decrement_timer_and_update();
                }
            }
            7 => {
                if self.ed_layer != 0 {
                    self.increment_level_counter();
                } else {
                    self.increment_timer_and_update();
                }
            }
            _ => {}
        }
        self.button_hit = 0;
    }

    /// handle_jail_customer: saves the custom levels (both files).
    fn handle_jail_customer(&mut self) {
        tr!(self, "handle_jail_customer");
        self.cust_backs.copy_from_slice(&self.backs);
        self.cust_bricks.copy_from_slice(&self.bricks);
        self.save_out = Some((self.cust_bricks.clone(), self.cust_backs.clone()));
    }

    /// set_interrupt_mask: a one-second OS delay (and the program's own frame interrupt put back). The OS counts the
    /// delay in vertical-blank ticks and wakes the program at a frame interrupt, the first one at least DELAY_FRAMES
    /// frames after the call; the program goes on from the start of that frame. (Until 3 Nov 2026 the delay was 50
    /// frames from the call, so the program went on up to a frame early: in the golden run v2 the level after a
    /// delay was served 2 frames ahead of the original and the ball's path parted at about frame 5040. Measured
    /// against the reference harness with the OS delay: call in frame 1687, wake in frame 1737, here too now.)
    fn set_interrupt_mask(&mut self) {
        tr!(self, "set_interrupt_mask");
        if DELAY_FRAMES > 0.0 {
            let end = (self.time + DELAY_FRAMES as f64).ceil();
            self.busy((end - self.time) as f32);
        }
    }

    /// clear_game_buffers: every custom level emptied.
    fn clear_game_buffers(&mut self) {
        tr!(self, "clear_game_buffers");
        self.bricks.fill(0);
        self.backs.fill(0);
    }

    fn redraw_level(&mut self) {
        self.wait_one_second(0x30, 0x30, self.cust_level);
        self.decode_map_data();
        self.wait_frames_2();
        self.decode_level_map();
    }

    /// increment_timer_and_update: the next custom level.
    fn increment_timer_and_update(&mut self) {
        tr!(self, "increment_timer_and_update");
        self.cust_level += 1;
        if self.cust_level > 99 {
            self.cust_level -= 1;
            self.clear_sprite_buffers();
        } else {
            self.redraw_level();
        }
    }

    /// decrement_timer_and_update: the previous custom level.
    fn decrement_timer_and_update(&mut self) {
        tr!(self, "decrement_timer_and_update");
        self.cust_level -= 1;
        if self.cust_level < 0 {
            self.cust_level = 0;
            self.clear_sprite_buffers();
        } else {
            self.redraw_level();
        }
    }

    /// increment_level_counter: the next page of background tiles.
    fn increment_level_counter(&mut self) {
        tr!(self, "increment_level_counter");
        self.ed_page += 1;
        if self.ed_page > 8 {
            self.ed_page -= 1;
            self.clear_sprite_buffers();
        } else {
            self.draw_background_tiles();
        }
    }

    /// decrement_timer_and_trigger_event_3: the previous page of background tiles.
    fn decrement_timer_and_trigger_event_3(&mut self) {
        tr!(self, "decrement_timer_and_trigger_event_3");
        self.ed_page -= 1;
        if self.ed_page < 0 {
            self.ed_page = 0;
            self.clear_sprite_buffers();
        } else {
            self.draw_background_tiles();
        }
    }

    /// check_player_input: the right button clears the cell under the pointer.
    fn check_player_input(&mut self) {
        tr!(self, "check_player_input");
        if self.mode != M_EDITOR || !self.rmb() {
            return;
        }
        if self.ptr_y < 0 || self.ptr_x < 0x60 || self.ptr_y >= 0xd0 || self.ptr_x >= 0x130 {
            return;
        }
        if self.ed_layer != 0 {
            self.clear_tile_and_update();
        } else {
            self.clear_map_cell();
        }
    }

    /// wait_for_player_input (misnamed): picks a brick or pill from the palette, then waits for the button's release.
    fn wait_for_player_input(&mut self) {
        tr!(self, "wait_for_player_input");
        self.ed_brick =
            ((self.ptr_y - 0x40) as u16 >> 3) as i16 * 4 + ((self.ptr_x - 0x10) as u16 >> 4) as i16;
        self.clear_pill();
        self.trigger_game_over();
        loop {
            self.refresh();
            if !self.lmb() {
                break;
            }
        }
    }

    /// wait_for_input_and_update_state (misnamed): picks a background tile, then waits for the button's release.
    fn wait_for_input_and_update_state(&mut self) {
        tr!(self, "wait_for_input_and_update_state");
        let t =
            ((self.ptr_y - 0x40) as u16 >> 4) as i16 * 4 + ((self.ptr_x - 0x10) as u16 >> 4) as i16;
        self.ed_back = t + (self.ed_page << 4) + BACK_TILE;
        self.trigger_game_over_and_reset();
        loop {
            self.refresh();
            if !self.lmb() {
                break;
            }
        }
    }

    /// update_map_tile: puts the chosen brick or pill in the cell under the pointer (bricks on rows 5 to 20 only).
    fn update_map_tile(&mut self) {
        tr!(self, "update_map_tile");
        if self.ptr_y >= 0xa8 || self.ptr_y < 0x28 {
            return;
        }
        let k = self.brick_cell(self.ptr_y, self.ptr_x);
        let cell = self.bricks[k];
        let (y, x) = (self.ptr_y & !7, self.ptr_x & !15);
        if self.ed_brick == 0 {
            if cell & 15 == 0 {
                return;
            }
            self.bricks[k] = cell & 0xf0;
            let t = (cell >> 4) as i16 + 0x10;
            self.init_player_state_4(y, x, t);
            self.sfx(5);
            return;
        }
        if self.ed_brick == 16 {
            if cell == 0 {
                return;
            }
            self.bricks[k] = 0;
        } else if self.ed_brick < 16 {
            let v = cell & 0xf0;
            if v == 0 {
                return;
            }
            let v = v | self.ed_brick as u8;
            if v == cell {
                return;
            }
            self.bricks[k] = v;
        } else {
            if self.wait_for_vblank() {
                return;
            }
            let v = (cell & 15) | (((self.ed_brick - 16) as u8) << 4);
            if v == cell {
                return;
            }
            self.bricks[k] = v;
            if v & 15 != 0 {
                return;
            }
        }
        let t = self.ed_brick;
        self.init_player_state_4(y, x, t);
        self.sfx(5);
    }

    /// wait_for_vblank (misnamed): a row may hold only one moving brick: true when the pointer's row has one.
    fn wait_for_vblank(&mut self) -> bool {
        tr!(self, "wait_for_vblank");
        if self.ed_brick != 0x1f {
            return false;
        }
        let r = self.cust_level as usize * BRICK_MAP + (self.ptr_y as u16 >> 3) as usize * 13;
        self.bricks[r..r + 13].iter().any(|c| c >> 4 == 0xf)
    }

    /// update_map_tile_2: puts the chosen background tile in the cell under the pointer.
    fn update_map_tile_2(&mut self) {
        tr!(self, "update_map_tile_2");
        let k = self.back_cell(self.ptr_y, self.ptr_x);
        let v = (self.ed_back - BACK_TILE) as u8;
        if self.backs[k] != v {
            self.backs[k] = v;
            self.update_game_state();
        }
    }

    /// clear_tile_and_update: the background cell under the pointer back to the first tile.
    fn clear_tile_and_update(&mut self) {
        tr!(self, "clear_tile_and_update");
        let k = self.back_cell(self.ptr_y, self.ptr_x);
        if self.backs[k] != 0 {
            self.backs[k] = 0;
            self.trigger_level_events();
        }
    }

    /// clear_map_cell: the brick cell under the pointer emptied.
    fn clear_map_cell(&mut self) {
        tr!(self, "clear_map_cell");
        let k = self.brick_cell(self.ptr_y, self.ptr_x);
        if self.bricks[k] != 0 {
            self.bricks[k] = 0;
            self.init_player_state_4(self.ptr_y & !7, self.ptr_x & !15, 0);
            self.sfx(5);
        }
    }

    /// update_game_state (misnamed): draws the background tile just placed.
    fn update_game_state(&mut self) {
        tr!(self, "update_game_state");
        self.draw_bob_sprite(self.ptr_y & !15, self.ptr_x & !15, self.ed_back);
        self.sfx(5);
    }

    /// trigger_level_events (misnamed): draws the background cell just cleared.
    fn trigger_level_events(&mut self) {
        tr!(self, "trigger_level_events");
        self.draw_bob_sprite(self.ptr_y & !15, self.ptr_x & !15, BACK_TILE);
        self.sfx(5);
    }

    /// fill_level_tilemap: the palette of bricks and pills (the top halves of the first 32 tiles).
    fn fill_level_tilemap(&mut self) {
        tr!(self, "fill_level_tilemap");
        let mut t = 0;
        for y in (64..128).step_by(8) {
            for x in (16..80).step_by(16) {
                self.plain(Src::Tile(t), 0, 0, Dst::Display, y, x, 8, 16);
                t += 1;
            }
        }
    }

    /// draw_background_tiles: a page of 16 background tiles.
    fn draw_background_tiles(&mut self) {
        tr!(self, "draw_background_tiles");
        let mut t = tile_index((self.ed_page << 4) + BACK_TILE);
        for y in (64..128).step_by(16) {
            for x in (16..80).step_by(16) {
                self.plain(Src::Tile(t), 0, 0, Dst::Display, y, x, 16, 16);
                t += 1;
            }
        }
    }

    /// initialize_player_object (misnamed): the chosen brick or pill shown beside its name.
    fn initialize_player_object(&mut self) {
        tr!(self, "initialize_player_object");
        let t = tile_index(self.ed_brick);
        self.masked(Src::Tile(t), 0, 0, Dst::Display, 0x1c, 0x28, 8, 16);
    }

    /// init_player_state (misnamed): the chosen background tile shown.
    fn init_player_state(&mut self) {
        tr!(self, "init_player_state");
        let t = tile_index(self.ed_back);
        self.masked(Src::Tile(t), 0, 0, Dst::Display, 0x18, 0x28, 16, 16);
    }

    /// trigger_game_over_and_reset (misnamed): the background-tile box: level number and chosen tile.
    fn trigger_game_over_and_reset(&mut self) {
        tr!(self, "trigger_game_over_and_reset");
        self.reset_player_state_2();
        self.wait_one_second(0x30, 0x30, self.cust_level);
        self.init_player_state();
    }

    /// clear_pill (misnamed): the brick box: level number, chosen brick and its name.
    fn clear_pill(&mut self) {
        tr!(self, "clear_pill");
        self.reset_player_state_2();
        self.wait_one_second(0x30, 0x30, self.cust_level);
        self.initialize_player_object();
        let name = self.a.editor_names[(self.ed_brick as usize).min(31)].clone();
        self.process_input_string(&name, 0xb8, 16, 24);
    }

    /// trigger_game_over (misnamed): the chosen brick's or pill's sound.
    fn trigger_game_over(&mut self) {
        tr!(self, "trigger_game_over");
        let s = self.a.editor_brick_sfx[(self.ed_brick as usize).min(31)];
        self.sfx(s);
    }

    /// reset_player_state_2 (misnamed): the editor's empty level box.
    fn reset_player_state_2(&mut self) {
        tr!(self, "reset_player_state_2");
        self.init_player_state_2(0, 0xc0, 0x10, 0x10, 0x20, 0x40);
    }

    /// init_player_state_4: one editor cell: its background half, then its brick or pill.
    fn init_player_state_4(&mut self, y: i16, x: i16, t: i16) {
        tr!(self, "init_player_state_4");
        let k = self.back_cell(y, x);
        let bt = tile_index(self.backs[k] as i16 + BACK_TILE);
        let sy = if y & 8 != 0 { 8 } else { 0 };
        self.plain(Src::Tile(bt), sy, 0, Dst::Display, y, x, 8, 16);
        self.masked(Src::Tile(tile_index(t)), 0, 0, Dst::Display, y, x, 8, 16);
    }

    /// decode_level_map: every cell of the custom level (a pill shows over its brick).
    fn decode_level_map(&mut self) {
        tr!(self, "decode_level_map");
        let base = self.cust_level as usize * BRICK_MAP;
        for r in 0..26i16 {
            for c in 0..13i16 {
                let b = self.bricks[base + (r * 13 + c) as usize];
                let mut t = (b & 15) as i16;
                if t == 0 {
                    t = (b >> 4) as i16;
                    if t == 0 {
                        continue;
                    }
                    t += 0x10;
                }
                self.init_player_state_4(r * 8, 0x60 + c * 16, t);
            }
        }
    }

    /// wait_frames_2 (misnamed): the custom level's background.
    fn wait_frames_2(&mut self) {
        tr!(self, "wait_frames_2");
        let base = self.cust_level as usize * BACK_MAP;
        self.draw_backs(base, 13);
        self.busy(BACKGROUND_COST);
    }

    fn draw_backs(&mut self, base: usize, rows: i16) {
        for r in 0..rows {
            for c in 0..13i16 {
                let b = self.backs[base + (r * 13 + c) as usize] as i16;
                self.draw_bob_sprite(r * 16, 0x60 + c * 16, b + BACK_TILE);
            }
        }
    }
}

/// The original's one-second OS delay. The reference harness's OS returns from it at once, so the verification build
/// (feature "ref-harness") does too; the game itself waits the second the original did.
const DELAY_FRAMES: f32 = if cfg!(feature = "ref-harness") {
    0.0
} else {
    50.0
};

// ------------------------------------------------------------------------------------------------ the frame routine
impl Jail {
    /// wait_vblank_and_refresh: waits for the frame, then draws the bat and the sprites, runs the brick animations
    /// and the moving bricks, moves the balls, the pill and the shots, follows the mouse and reads the keys.
    fn refresh(&mut self) {
        tr!(self, "wait_vblank_and_refresh");
        self.frame();
        // The frame's own work took the original a part of the frame (measured: 0.05 on the title; in play 0.11, or
        // 0.26 on the frames of the brick animations' pass, every second frame: 0.19 on average).
        self.busy(if self.mode == M_PLAY { 0.19 } else { 0.05 });
        self.process_prisoner_cells_2();
        self.process_prisoner_cells();
        self.reset_level_tiles();
        self.update_enemy_ai();
        self.check_level_two_flags_and_update();
        self.update_player_position();
        self.handle_input_state();
    }

    /// wait_for_vblank_and_update_game: the pause: only the drawing goes on until a key is pressed.
    fn wait_for_vblank_and_update_game(&mut self) {
        tr!(self, "wait_for_vblank_and_update_game");
        loop {
            self.frame();
            self.process_prisoner_cells_2();
            self.process_prisoner_cells();
            self.reset_level_tiles();
            if self.poll_cia_timer() != 0 {
                return;
            }
        }
    }

    /// process_prisoner_cells_2: the bat's pieces drawn into the screen after the bat rows are restored.
    fn process_prisoner_cells_2(&mut self) {
        tr!(self, "process_prisoner_cells_2");
        self.draw_boss_health_bar();
        for i in (0..13).rev() {
            if !self.ents[i].sprite {
                self.set_blitter_copy_params(i);
            }
        }
    }

    /// draw_boss_health_bar (misnamed): the bat's rows (and the top bat's) restored from the clean picture.
    fn draw_boss_health_bar(&mut self) {
        tr!(self, "draw_boss_health_bar");
        if self.mode != M_PLAY {
            return;
        }
        self.plain(Src::Work, 200, 96, Dst::Display, 200, 96, 8, 208);
        if self.pow & P_DOUBLE != 0 {
            self.plain(Src::Work, 0, 96, Dst::Display, 0, 96, 8, 208);
        }
    }

    /// set_blitter_copy_params: one bat piece drawn (masked tile at any position).
    fn set_blitter_copy_params(&mut self, i: usize) {
        tr!(self, "set_blitter_copy_params");
        let e = self.ents[i];
        if e.img == 0 {
            return;
        }
        self.masked(
            Src::Tile(tile_index(e.img)),
            0,
            0,
            Dst::Display,
            e.y,
            e.x,
            16,
            16,
        );
    }

    /// process_prisoner_cells: the sprites placed and their pictures changed.
    fn process_prisoner_cells(&mut self) {
        tr!(self, "process_prisoner_cells");
        for i in (0..13).rev() {
            if self.ents[i].sprite {
                self.write_score_digit(i);
                self.sync_player_state(i);
            }
        }
    }

    /// write_score_digit (misnamed): a sprite's position, as the original wrote it (its line wraps at 256).
    fn write_score_digit(&mut self, i: usize) {
        tr!(self, "write_score_digit");
        let e = self.ents[i];
        if e.img == 0 {
            return;
        }
        let line = 0x24u8.wrapping_add(e.y as u8);
        let h = 0x40u8.wrapping_add((e.x as u16 >> 1) as u8) as i16 * 2 + (e.x & 1);
        self.sprites[e.spr] = Some((line, h));
    }

    /// sync_player_state: a sprite's picture changed when its thing's picture did.
    fn sync_player_state(&mut self, i: usize) {
        tr!(self, "sync_player_state");
        let e = self.ents[i];
        if e.img != e.last_img {
            self.ents[i].last_img = e.img;
            self.sprite_tiles[e.spr] = e.img;
            tr!(self, "copy_sprite_frames");
        }
    }

    /// reset_level_tiles: every second frame, each hit brick's animation moves on a frame (six frames each).
    fn reset_level_tiles(&mut self) {
        tr!(self, "reset_level_tiles");
        if self.mode != M_PLAY {
            return;
        }
        self.brick_anim_div -= 1;
        if self.brick_anim_div > 0 {
            return;
        }
        self.brick_anim_div = 2;
        for r in 0..26i16 {
            for c in 0..13i16 {
                let k = (r * 13 + c) as usize;
                let t = self.anim[k];
                if t == 0 {
                    continue;
                }
                self.draw_player_sprite(r * 8, 96 + c * 16, t as i16);
                self.anim[k] = t.wrapping_add(1);
                let v = (t as i32 + 1 - 0x8c) as u32;
                if v / 6 <= 0xffff && v.is_multiple_of(6) {
                    self.anim[k] = 0;
                }
            }
        }
    }

    fn map_at(&self, y: i16, x: i16) -> u8 {
        let k = (y as u16 >> 3) as usize * 13 + (x as u16 >> 4) as usize;
        self.players[self.cur].map.get(k).copied().unwrap_or(0)
    }

    /// update_enemy_ai (misnamed): the moving bricks go to and fro, turning at the walls and at other bricks.
    fn update_enemy_ai(&mut self) {
        tr!(self, "update_enemy_ai");
        if self.mode != M_PLAY || self.movers_on == 0 {
            return;
        }
        for i in 0..self.movers.len() {
            let m = self.movers[i];
            if m.kind == 0 {
                break;
            }
            let blocked = if m.dx >= 0 {
                if m.x > 0x11f {
                    true
                } else {
                    self.cell_blocks(self.map_at(m.y, m.x - 0x60 + 0x11))
                }
            } else if m.x < 0x60 {
                true
            } else {
                self.cell_blocks(self.map_at(m.y, m.x - 1 - 0x60))
            };
            if blocked {
                self.movers[i].dx = -m.dx;
                continue;
            }
            self.movers[i].x = m.x + m.dx;
            let m = self.movers[i];
            self.draw_background_tile(m.y, m.x, m.kind);
        }
    }

    fn cell_blocks(&self, cell: u8) -> bool {
        let t = cell >> 4;
        t != 0 && t != 0xf
    }

    /// check_level_two_flags_and_update (misnamed): the balls, the pill and the shots move.
    fn check_level_two_flags_and_update(&mut self) {
        tr!(self, "check_level_two_flags_and_update");
        if self.mode != M_PLAY {
            return;
        }
        if self.ents[BALL1].img != 0 {
            self.process_player_movement(V_BALL1, BALL1);
        }
        if self.ents[BALL2].img != 0 {
            self.process_player_movement(V_BALL2, BALL2);
        }
        if self.ents[BALL3].img != 0 {
            self.process_player_movement(V_BALL3, BALL3);
        }
        self.handle_player_escape();
        self.update_prisoner_state();
    }

    fn bat_move(&mut self, d: i16) {
        for i in [BAT_L, BAT_M, BAT_R, TOP_L, TOP_M, TOP_R] {
            self.ents[i].x = self.ents[i].x.wrapping_add(d);
        }
    }

    /// update_player_position: the mouse moves the bat (kept between the walls, and the waiting ball with it) or,
    /// on the title and in the editor, the pointer.
    fn update_player_position(&mut self) {
        tr!(self, "update_player_position");
        let dy = self.mouse_y.wrapping_sub(self.mouse_last.1) as i8 as i16;
        let mut dx = self.mouse_x.wrapping_sub(self.mouse_last.0) as i8 as i16;
        self.mouse_last = (self.mouse_x, self.mouse_y);
        if self.mode == M_PLAY {
            if self.pow & P_REVERSE != 0 {
                dx = -dx;
            }
            self.bat_move(dx);
            if self.ents[BAT_L].x < 0x60 {
                self.bat_move(0x60 - self.ents[BAT_L].x);
            }
            let r = self.ents[BAT_L].x + self.bat_w;
            if r >= 0x130 {
                self.bat_move(-(r - 0x130));
            }
            if self.serve_wait == 0 {
                return;
            }
            let bx = self.ents[BAT_L].x;
            if self.ents[BALL1].img != 0 {
                let x = self.ents[BALL1].x;
                if x < bx - 4 {
                    self.ents[BALL1].x = bx - 4;
                } else if x > bx - 0xb + self.bat_w {
                    self.ents[BALL1].x = bx - 0xb + self.bat_w;
                }
            } else {
                let b = if self.ents[BALL2].img != 0 {
                    BALL2
                } else if self.ents[BALL3].img != 0 {
                    BALL3
                } else {
                    return;
                };
                let x = self.ents[b].x;
                if x < bx - 8 {
                    self.ents[b].x = bx - 8;
                } else if x > bx - 8 + self.bat_w - 1 {
                    self.ents[b].x = bx - 8 + self.bat_w - 1;
                }
            }
        } else {
            let p = &mut self.ents[POINTER];
            p.x = p.x.wrapping_add(dx).clamp(0, 0x130);
            p.y = p.y.wrapping_add(dy).clamp(0, 0xc0);
            self.ptr_y = p.y + 7;
            self.ptr_x = p.x + 7;
        }
    }

    /// handle_input_state: the keys: Space pauses, Escape ends the game (or, on the title, the program). The letter
    /// keys that give power-ups work only with the developers' password, which nothing asks for any more.
    fn handle_input_state(&mut self) {
        tr!(self, "handle_input_state");
        if self.mode == M_PLAY {
            let k = self.poll_cia_timer();
            if k == 0 {
                return;
            }
            if k == K_SPACE {
                self.clear_sprite_buffers();
                self.wait_for_vblank_and_update_game();
                self.get_digit_tile();
                return;
            }
            if k == K_ESCAPE {
                self.playing = 0;
                self.mode = M_TITLE;
                self.credits = 0;
                self.countdown = 0;
                self.start_level = 0;
                return;
            }
            if self.cheats_off != 0 {
                return;
            }
            let e = match k as u8 {
                b'B' => 9,
                b'D' => 13,
                b'M' => 4,
                b'W' => 10,
                b'T' => 12,
                b'C' => 1,
                b'H' => 6,
                b'G' => 11,
                b'F' => 3,
                b'S' => 2,
                b'K' => 8,
                b'E' => 7,
                b'L' => 5,
                b'R' => 14,
                _ => return,
            };
            self.pill_effect = e;
            self.process_player_input();
        } else if self.mode == M_TITLE && self.poll_cia_timer() == K_ESCAPE {
            self.running = 0;
        }
    }
}

// ------------------------------------------------------------------------------------------------ playing
impl Jail {
    fn p(&self) -> &Player {
        &self.players[self.cur]
    }

    fn pm(&mut self) -> &mut Player {
        &mut self.players[self.cur]
    }

    /// initialize_level_state: a new game: each player's lives, score and first level, the screen built up, the
    /// level's music, the bricks drawn one by one.
    fn initialize_level_state(&mut self) {
        tr!(self, "initialize_level_state");
        self.clear_input_buffer();
        self.countdown = 0;
        self.load_picture("jailgadgets2", false, Dst::Work, true);
        self.cur = 0;
        self.reset_and_run_game_loop();
        self.serve_wait = -1;
        self.playing = -1;
        for i in [BAT_L, BAT_M, BAT_R] {
            self.ents[i].y = 0xc8;
        }
        for i in [TOP_L, TOP_R, TOP_M] {
            self.ents[i].y = 0;
        }
        self.ents[BAT_L].x = 0x60;
        self.ents[BAT_R].x = 0x70;
        self.ents[TOP_L].x = 0x60;
        self.ents[TOP_R].x = 0x70;
        self.ents[BALL1].y = 0xbe;
        self.ents[BALL1].x = 0x70;
        self.vels[V_BALL1].vy = 1;
        self.vels[V_BALL1].ys = 1;
        self.vels[V_BALL1].xs = 1;
        self.bat_w = 0x20;
        self.bat_img = 0x50;
        self.pow = 0;
        self.call_init_player_state_2();
        self.cur = 0;
        self.new_player();
        self.cur = 1;
        if self.n_players == 1 {
            self.pm().lives = -1;
        } else {
            self.new_player();
        }
        self.cur = 0;
        self.speed = self.p().skill;
        self.clear_game_state();
        self.set_tile_collision_flags(32);
        self.wait_frames();
        self.decode_map_data();
        self.play_prisoner_escape_jingle();
        self.wait_one_second(0x60, 0x30, self.p().level);
        self.reset_player_state_2_2();
        self.wait_frames_3();
        self.init_game_state("jailtiles1");
        self.decode_map_tile_row();
        self.get_digit_tile();
        self.set_interrupt_mask();
        self.wait_for_input_2();
        self.clear_tile_buffer_2();
        self.hits_to_speedup = 0x28;
        self.serve_timer = 0xb4;
        self.sfx(0x17);
        self.ents[BALL1].img = 0x4c;
    }

    fn new_player(&mut self) {
        let p = self.pm();
        p.lives = 3;
        p.next_life = 1000;
        p.score = 0;
        self.copy_tile_row();
        self.wait_three_frames();
    }

    /// call_init_player_state_2: the score panel.
    fn call_init_player_state_2(&mut self) {
        tr!(self, "call_init_player_state_2");
        self.init_player_state_2(0, 0x100, 0x10, 0x10, 0xb0, 0x40);
        self.busy(PANEL_COST);
    }

    /// reset_and_run_game_loop: the "player one" or "player two" screen, with its jingle.
    fn reset_and_run_game_loop(&mut self) {
        tr!(self, "reset_and_run_game_loop");
        self.clear_sprite_buffers();
        let one = self.p().id == 1;
        self.load_picture(
            if one { "jailplayer1" } else { "jailplayer2" },
            true,
            Dst::Work,
            false,
        );
        self.initialize_prisoner_positions(Shown::Work);
        self.wait_for_input_2();
        self.sfx(if one { 0x13 } else { 0x14 });
        for _ in 0..=0x64 {
            self.refresh();
        }
        self.clear_input_buffer();
        self.initialize_prisoner_positions(Shown::Display);
        self.load_picture("jailgadgets2", true, Dst::Work, false);
    }

    /// wait_vblank_and_refresh_loop: the game's loop, until both players are out (or Escape).
    fn wait_vblank_and_refresh_loop(&mut self) {
        tr!(self, "wait_vblank_and_refresh_loop");
        loop {
            self.refresh();
            self.handle_player_input();
            self.reset_player_state();
            self.decrement_animation_timer();
            self.decrement_timer_and_trigger_event_2();
            self.update_prisoner_timer();
            self.reset_player_timers();
            if self.playing == 0 {
                break;
            }
        }
        if self.mode == M_PLAY {
            self.run_jail_over_sequence();
        }
        self.clear_sprite_buffers();
    }

    /// run_jail_over_sequence: the game-over screens and the final scores; with credits left the title will offer to
    /// carry on from the level reached for a while.
    fn run_jail_over_sequence(&mut self) {
        tr!(self, "run_jail_over_sequence");
        self.credits = self.credits.wrapping_sub(1);
        if self.credits <= 0 {
            self.start_level = 0;
            self.countdown = 0;
        } else {
            self.countdown = 0x21c;
            self.cur = 0;
            self.start_level = self.players[0].level;
        }
        self.clear_entity_slots();
        self.clear_input_buffer();
        self.mode = M_TITLE;
        self.clear_sprite_buffers();
        self.init_game_state("jailover1");
        self.wait_for_game_over_flag(Dst::Display);
        self.decode_map_tile_row();
        self.wait_for_input_2();
        self.load_picture("jailover2", false, Dst::Work, true);
        self.sfx(0x1d);
        self.sfx(0x1d);
        self.reset_game_state();
        for _ in 0..=0x64 {
            self.refresh();
        }
        self.load_picture("jailover3", false, Dst::Work, true);
        self.reset_game_state();
        self.load_picture("jailgadgets1", false, Dst::Work, true);
        self.wait_for_vblank_twice();
        self.sfx(0x1e);
        self.sfx(0x1e);
        for _ in 0..=0x208 {
            self.refresh();
            if self.lmb() {
                break;
            }
        }
    }

    /// wait_for_vblank_twice (misnamed): both players' final scores in the big digits.
    fn wait_for_vblank_twice(&mut self) {
        tr!(self, "wait_for_vblank_twice");
        for (p, dy, h) in [(0usize, 0x8e, 24), (1, 0xa8, 20)] {
            let mut v = self.players[p].score as u16 as u32;
            let mut x = 0x104;
            for _ in 0..5 {
                let d = (v % 10) as i16;
                v /= 10;
                self.init_player_state_2(0xb0, d * 16 + 0x80, dy, x, h, 16);
                x -= 0x14;
            }
        }
        self.init_player_state_2(0xb0, 0x80, 0xa8, 0x118, 20, 16);
        self.init_player_state_2(0xb0, 0x80, 0x8e, 0x118, 20, 16);
        // The original left its current-player pointer on player two here.
        self.cur = 1;
        self.busy(SCORES_COST);
    }

    /// reset_player_state: the ball waiting on the bat goes by itself after a while.
    fn reset_player_state(&mut self) {
        tr!(self, "reset_player_state");
        if self.serve_timer < 0 {
            return;
        }
        self.serve_timer -= 1;
        if self.serve_timer < 0 {
            self.serve_wait = 0;
            self.launch_guard = -1;
            self.serve_timer = -1;
        }
    }

    /// handle_player_input: the left button launches the waiting ball or fires the cannon; in the editor it picks
    /// from the palettes or draws in the level.
    fn handle_player_input(&mut self) {
        tr!(self, "handle_player_input");
        if !self.lmb() {
            self.lmb_was_up = -1;
            return;
        }
        if self.mode == M_PLAY {
            if self.lmb_was_up == 0 {
                return;
            }
            self.lmb_was_up = 0;
            let on_bat = |e: &Ent| e.img != 0 && (e.y == 2 || e.y == 0xbe);
            if on_bat(&self.ents[BALL1]) || on_bat(&self.ents[BALL2]) || on_bat(&self.ents[BALL3]) {
                self.serve_wait = 0;
                self.launch_guard = -1;
                self.serve_timer = -1;
            } else if self.pow & P_CANNON != 0 {
                self.update_prisoner_positions();
            }
        } else if self.mode == M_EDITOR {
            let (y, x) = (self.ptr_y, self.ptr_x);
            if (0x40..0x80).contains(&y) && (0x10..0x50).contains(&x) {
                if self.ed_layer != 0 {
                    self.wait_for_input_and_update_state();
                } else {
                    self.wait_for_player_input();
                }
            } else if (0..0xd0).contains(&y) && (0x60..0x130).contains(&x) {
                if self.ed_layer != 0 {
                    self.update_map_tile_2();
                } else {
                    self.update_map_tile();
                }
            }
        }
    }

    /// update_prisoner_positions (misnamed): the cannon fires (from the top bat too, with the double bat).
    fn update_prisoner_positions(&mut self) {
        tr!(self, "update_prisoner_positions");
        if self.ents[SHOT1].img == 0 {
            self.sfx(17);
            self.ents[SHOT1].y = self.ents[BAT_R].y - 12;
            self.ents[SHOT1].x = self.ents[BAT_R].x - 8;
            self.ents[SHOT1].img = 0x4a;
            self.vels[V_SHOT1].ys = self.p().skill + 4;
            if self.ents[BAT_R].x <= 0xc8 {
                self.ents[SHOT1].x -= 1;
            }
        }
        if self.pow & P_DOUBLE != 0 && self.ents[SHOT2].img == 0 {
            self.sfx(17);
            self.ents[SHOT2].y = self.ents[TOP_R].y;
            // The original adds this to the first shot, not the second (kept).
            self.ents[SHOT1].y += 12;
            self.ents[SHOT2].x = self.ents[TOP_R].x - 8;
            self.ents[SHOT2].img = 0x4b;
            self.vels[V_SHOT2].ys = self.p().skill + 4;
            if self.ents[TOP_R].x <= 0xc8 {
                self.ents[SHOT2].x -= 1;
            }
        }
    }

    /// decrement_animation_timer: the mine and giant balls' animations.
    fn decrement_animation_timer(&mut self) {
        tr!(self, "decrement_animation_timer");
        self.ball_anim_div -= 1;
        if self.ball_anim_div > 0 {
            return;
        }
        self.ball_anim_div = 2;
        // Returns true when the picture wrapped round.
        let step = |e: &mut Ent, lo: i16, hi: i16| -> bool {
            if e.img != 0 {
                e.img += 1;
                if e.img >= hi {
                    e.img = lo;
                    return true;
                }
            }
            false
        };
        if self.pow & P_MINE != 0 {
            step(&mut self.ents[BALL1], 0x84, 0x8b);
            step(&mut self.ents[BALL2], 0x84, 0x8b);
            // As the original: only a wrap of the third ball's picture skips the giant ball's step.
            if step(&mut self.ents[BALL3], 0x84, 0x8b) {
                return;
            }
        }
        if self.pow & P_GIANT != 0 {
            for b in [BALL1, BALL2, BALL3] {
                step(&mut self.ents[b], 0x7c, 0x83);
            }
        }
    }

    /// decrement_timer_and_trigger_event_2: the mine ball ticks.
    fn decrement_timer_and_trigger_event_2(&mut self) {
        tr!(self, "decrement_timer_and_trigger_event_2");
        if self.pow & P_MINE == 0 {
            return;
        }
        self.mine_timer -= 1;
        if self.mine_timer <= 0 {
            self.mine_timer = 0x14;
            self.sfx(9);
        }
    }

    /// update_prisoner_timer: an extra life every 1000 points, and the score shown (with a 0 after it).
    fn update_prisoner_timer(&mut self) {
        tr!(self, "update_prisoner_timer");
        let s = self.p().score;
        if s >= self.p().next_life {
            self.pm().lives += 1;
            self.wait_three_frames();
            self.pm().next_life = self.p().next_life.wrapping_add(1000);
        }
        let y = (self.p().id - 1) * 0x60 + 0x34;
        let mut v = self.p().score as u16 as u32;
        let mut x = 64;
        let mut ch = 0x30;
        for _ in 0..6 {
            self.copy_tilemap_row(y, x, ch, 24);
            x -= 8;
            ch = (v % 10) as i16 + 0x30;
            v /= 10;
        }
    }

    /// reset_player_timers: a ball past the bat (or past the top bat) is lost; with none left the life is.
    fn reset_player_timers(&mut self) {
        tr!(self, "reset_player_timers");
        if self.playing == 0 || self.pow & P_KILL != 0 {
            return;
        }
        for b in [BALL1, BALL2, BALL3] {
            if self.pow & P_DOUBLE != 0 && self.ents[b].y < 0 {
                self.ents[b].img = 0;
                self.pow &= !P_BALLS;
            }
        }
        for b in [BALL1, BALL2, BALL3] {
            if self.ents[b].y >= 0xd2 {
                self.ents[b].img = 0;
                self.pow &= !P_BALLS;
            }
        }
        if self.ents[BALL1].img == 0 && self.ents[BALL2].img == 0 && self.ents[BALL3].img == 0 {
            self.initialize_game_state_2();
        }
    }

    fn park_all(&mut self) {
        for i in [PILL, SHOT1, SHOT2, BALL1, BALL2, BALL3] {
            self.ents[i].y = 0xd0;
        }
        self.refresh();
        for i in [PILL, SHOT1, SHOT2, BALL1, BALL2, BALL3, TOP_L, TOP_R, TOP_M] {
            self.ents[i].img = 0;
        }
        self.refresh();
    }

    /// initialize_game_state_2: a life lost (all balls gone, or the kill pill): the other player's turn if there is
    /// one, then a new ball on the bat.
    fn initialize_game_state_2(&mut self) {
        tr!(self, "initialize_game_state_2");
        self.normal_death = if self.pow & P_KILL != 0 { 0 } else { -1 };
        self.pow = 0;
        self.clear_sprite_buffers();
        self.park_all();
        self.decode_map_data();
        self.wait_frames_3();
        if self.normal_death != -1 {
            self.sfx(0x12);
            self.ents[BAT_L].img = 0x3c;
            self.ents[BAT_R].img = 0x3d;
            for d in (0..=34).rev() {
                self.refresh();
                if d % 5 == 0 {
                    self.ents[BAT_L].img += 2;
                    self.ents[BAT_R].img += 2;
                }
            }
            self.bat_img = 0x50;
            self.set_tile_collision_flags(32);
        } else {
            self.sfx(0x16);
            self.bat_img = 0x50;
            self.set_tile_collision_flags(32);
            for _ in 0..=0x55 {
                self.refresh();
            }
        }
        self.decrement_counter_and_check();
        if self.playing == 0 {
            return;
        }
        self.serve_wait = -1;
        self.speed = self.p().skill;
        self.hits_to_speedup = 0x28;
        self.wait_three_frames();
        if self.n_players == 1 {
            self.get_digit_tile();
        } else {
            self.clear_input_buffer();
            self.reset_and_run_game_loop();
            self.clear_game_state();
            self.wait_three_frames();
            self.wait_one_second(0x60, 0x30, self.p().level);
            self.wait_frames();
            self.play_prisoner_escape_jingle();
            self.reset_player_state_2_2();
            self.decode_map_data();
            self.wait_frames_3();
            self.get_digit_tile();
            self.set_interrupt_mask();
            self.wait_for_input_2();
            self.clear_tile_buffer_2();
        }
        self.serve_timer = 0xb4;
        self.ents[BALL1].y = 0xbe;
        self.ents[BALL1].x = self.ents[BAT_R].x;
        self.vels[V_BALL1].vy = 1;
        self.sfx(0x17);
        self.ents[BALL1].img = 0x4c;
    }

    /// decrement_counter_and_check: a life off; the turn passes to the other player while either has lives.
    fn decrement_counter_and_check(&mut self) {
        tr!(self, "decrement_counter_and_check");
        self.pm().lives -= 1;
        self.wait_three_frames();
        if self.p().id != 1 {
            self.cur = 0;
            if self.p().lives >= 0 {
                return;
            }
        }
        self.cur = 1;
        if self.p().lives >= 0 {
            return;
        }
        self.cur = 0;
        if self.p().lives >= 0 {
            return;
        }
        self.playing = 0;
    }

    /// init_game_state_3: a level finished: the last bricks go, the next level is built.
    fn init_game_state_3(&mut self) {
        tr!(self, "init_game_state_3");
        self.pow = 0;
        self.clear_sprite_buffers();
        self.park_all();
        self.clear_tile_buffer();
        self.sfx(0x15);
        self.clear_input_buffer();
        for _ in 0..=0x64 {
            self.refresh();
        }
        self.wait_one_second(0x60, 0x30, self.p().level);
        self.copy_tile_row();
        self.clear_game_state();
        self.serve_wait = -1;
        self.ents[BALL1].y = 0xbe;
        self.speed = self.p().skill;
        self.vels[V_BALL1].vy = 1;
        self.bat_img = 0x50;
        self.set_tile_collision_flags(32);
        self.wait_frames();
        self.decode_map_data();
        self.reset_player_state_2_2();
        self.wait_frames_3();
        self.get_digit_tile();
        self.set_interrupt_mask();
        self.sfx(0x15);
        self.wait_for_input_2();
        self.clear_tile_buffer_2();
        self.pm().score = self.p().score.wrapping_add(100);
        self.hits_to_speedup = 0x28;
        self.serve_timer = 0xb4;
        self.ents[BALL1].img = 0x4c;
        self.sfx(0x17);
    }

    /// clear_tile_buffer: the bricks left at the end of a level go one by one.
    fn clear_tile_buffer(&mut self) {
        tr!(self, "clear_tile_buffer");
        for k in 0..BRICK_MAP {
            let b = self.players[self.cur].map[k];
            self.players[self.cur].map[k] = 0;
            let t = (b >> 4) & 15;
            if t != 0 && t != 15 {
                self.sfx(0x11);
                self.refresh();
                self.anim[k] = 0xf8;
            }
        }
        for _ in 0..=0x32 {
            self.refresh();
        }
    }

    /// copy_tile_row: the player's level copied into the player's map.
    fn copy_tile_row(&mut self) {
        tr!(self, "copy_tile_row");
        let l = (self.p().level as usize).min(99) * BRICK_MAP;
        let src = self.bricks[l..l + BRICK_MAP].to_vec();
        self.pm().map.copy_from_slice(&src);
    }

    /// clear_game_state: no moving bricks.
    fn clear_game_state(&mut self) {
        tr!(self, "clear_game_state");
        self.movers.clear();
    }

    /// clear_tile_buffer_2: the level's bricks drawn one a frame, counted, and the moving bricks set going.
    fn clear_tile_buffer_2(&mut self) {
        tr!(self, "clear_tile_buffer_2");
        self.movers_on = 0;
        self.bricks_left = 0;
        let mut n = 0;
        for r in 0..26i16 {
            for c in 0..13i16 {
                let k = (r * 13 + c) as usize;
                let (y, x) = (r * 8, 96 + c * 16);
                self.anim[k] = 0;
                let b = self.players[self.cur].map[k];
                let t = (b >> 4) as i16;
                if t == 0 {
                    continue;
                }
                let t = t + 0x10;
                let tile = if t < 0x19 {
                    self.bricks_left += 1;
                    t - 0x11 + 0x20
                } else if t == 0x19 {
                    t
                } else if t == 0x1f {
                    self.draw_player_sprite(y, x, t);
                    self.set_sprite_dma_and_wait_vblank();
                    let m = Mover {
                        kind: 0x1f,
                        y,
                        x,
                        dx: 1,
                    };
                    if n < self.movers.len() {
                        self.movers[n] = m;
                    } else {
                        self.movers.push(m);
                    }
                    n += 1;
                    continue;
                } else {
                    self.bricks_left += 1;
                    0xf7
                };
                self.draw_player_sprite(y, x, tile);
                self.set_sprite_dma_and_wait_vblank();
            }
        }
        self.movers_on = -1;
    }

    /// set_sprite_dma_and_wait_vblank: a brick's sound and a frame.
    fn set_sprite_dma_and_wait_vblank(&mut self) {
        tr!(self, "set_sprite_dma_and_wait_vblank");
        self.sfx(5);
        self.refresh();
    }

    /// wait_three_frames (misnamed): the player's lives, as up to four lamps.
    fn wait_three_frames(&mut self) {
        tr!(self, "wait_three_frames");
        let mut n = self.p().lives;
        let y = (self.p().id - 1) * 0x60 + 0x40;
        let mut x = 0x40;
        for _ in 0..4 {
            n -= 1;
            let t = if n >= 0 { 58 } else { 59 };
            self.draw_bob_sprite(y, x, t);
            x -= 16;
        }
    }

    /// play_prisoner_escape_jingle (misnamed): the "one up" and "two up" labels, the playing player's lit.
    fn play_prisoner_escape_jingle(&mut self) {
        tr!(self, "play_prisoner_escape_jingle");
        let t = if self.p().id != 2 {
            [0x32, 0x33, 0x38, 0x39]
        } else {
            [0x36, 0x37, 0x34, 0x35]
        };
        self.draw_bob_sprite(0x20, 0x20, t[0]);
        self.draw_bob_sprite(0x20, 0x30, t[1]);
        self.draw_bob_sprite(0x80, 0x20, t[2]);
        self.draw_bob_sprite(0x80, 0x30, t[3]);
    }

    /// wait_frames (misnamed): the level's background tiles.
    fn wait_frames(&mut self) {
        tr!(self, "wait_frames");
        let base = (self.p().level as usize).min(99) * BACK_MAP;
        self.draw_backs(base, 13);
        self.busy(BACKGROUND_COST);
    }

    /// wait_for_input (misnamed): the background's top row (the double bat replaces the top wall).
    fn wait_for_input(&mut self) {
        tr!(self, "wait_for_input");
        let base = (self.p().level as usize).min(99) * BACK_MAP;
        self.draw_backs(base, 1);
    }

    fn border_base(&self) -> i16 {
        (((self.p().level as u16 >> 1) % 7) << 4) as i16 + 0x198
    }

    /// decode_map_data: the border around the level, in one of seven styles.
    fn decode_map_data(&mut self) {
        tr!(self, "decode_map_data");
        let base = self.border_base();
        for r in 0..13i16 {
            for c in 0..20i16 {
                let b = self.a.border_pattern[(r * 20 + c) as usize] as i16;
                if b != 0 {
                    self.draw_bob_sprite(r * 16, c * 16, b + base);
                }
            }
        }
        self.busy(BORDER_COST);
    }

    /// wait_frames_3 (misnamed): the top wall.
    fn wait_frames_3(&mut self) {
        tr!(self, "wait_frames_3");
        let base = self.border_base();
        for c in 0..13i16 {
            self.draw_bob_sprite(0, 0x60 + c * 16, base);
        }
        self.busy(TOP_WALL_COST);
    }

    /// get_digit_tile (misnamed): the level's music (one of five tunes).
    fn get_digit_tile(&mut self) {
        tr!(self, "get_digit_tile");
        let s = ((self.p().level as u16 % 10) >> 1) as i16 + 0x18;
        self.pitch = self.a.sfx[s as usize].period as i16;
        self.sfx(s);
    }

    /// set_tile_collision_flags: the bat's shape: its pieces' pictures for its width, growing or shrinking to the
    /// asked width (16, 32 or 48), and the top bat's with the double bat.
    fn set_tile_collision_flags(&mut self, want: i16) {
        tr!(self, "set_tile_collision_flags");
        let e2 = self.bat_img;
        let dbl = self.pow & P_DOUBLE != 0;
        if self.bat_w == 16 {
            if want == 16 {
                self.ents[BAT_L].img = e2 + 3;
                if dbl {
                    self.ents[TOP_L].img = e2 + 7;
                }
            } else {
                self.ents[BAT_L].img = e2;
                self.ents[BAT_R].img = e2 + 2;
                if dbl {
                    self.ents[TOP_L].img = e2 + 4;
                    self.ents[TOP_R].img = e2 + 6;
                }
                self.update_screen_scroll();
            }
        } else if self.bat_w == 48 {
            self.ents[BAT_L].img = e2;
            self.ents[BAT_M].img = e2 + 1;
            self.ents[BAT_R].img = e2 + 2;
            if want == 48 {
                self.ents[BAT_M].x = self.ents[BAT_L].x + 16;
                if dbl {
                    self.ents[TOP_L].img = e2 + 4;
                    self.ents[TOP_M].img = e2 + 5;
                    self.ents[TOP_R].img = e2 + 6;
                    self.ents[TOP_M].x = self.ents[TOP_L].x + 16;
                }
            } else {
                if dbl {
                    self.ents[TOP_L].img = e2 + 4;
                    self.ents[TOP_M].img = e2 + 5;
                    self.ents[TOP_R].img = e2 + 6;
                }
                self.animate_prisoner_cells();
                self.ents[BAT_M].img = 0;
                self.ents[TOP_M].img = 0;
            }
        } else {
            self.ents[BAT_L].img = e2;
            self.ents[BAT_R].img = e2 + 2;
            self.ents[BAT_M].img = 0;
            if dbl {
                self.ents[TOP_L].img = e2 + 4;
                self.ents[TOP_R].img = e2 + 6;
                self.ents[TOP_M].img = 0;
            }
            if want == 16 {
                self.animate_prisoner_cells();
                self.ents[BAT_R].img = 0;
                self.ents[BAT_L].img = e2 + 3;
                if dbl {
                    self.ents[TOP_R].img = 0;
                    self.ents[TOP_L].img = e2 + 7;
                }
            } else if want == 48 {
                self.ents[BAT_L].img = e2;
                self.ents[BAT_R].img = e2 + 2;
                self.ents[BAT_M].img = e2 + 1;
                self.ents[BAT_M].x = self.ents[BAT_L].x + 8;
                if dbl {
                    self.ents[TOP_L].img = e2 + 4;
                    self.ents[TOP_R].img = e2 + 6;
                    self.ents[TOP_M].img = e2 + 5;
                    self.ents[TOP_M].x = self.ents[TOP_L].x + 8;
                }
                self.update_screen_scroll();
            }
        }
        self.pow &= !(P_THIN | P_WIDE);
    }

    /// update_screen_scroll (misnamed): the bat grows by 16 pixels over 8 frames.
    fn update_screen_scroll(&mut self) {
        tr!(self, "update_screen_scroll");
        self.bat_w += 16;
        for _ in 0..8 {
            self.ents[BAT_L].x -= 1;
            self.ents[BAT_R].x += 1;
            self.ents[TOP_L].x -= 1;
            self.ents[TOP_R].x += 1;
            self.update_player_position();
            self.refresh();
        }
    }

    /// animate_prisoner_cells (misnamed): the bat shrinks by 16 pixels over 8 frames.
    fn animate_prisoner_cells(&mut self) {
        tr!(self, "animate_prisoner_cells");
        for _ in 0..8 {
            self.ents[BAT_L].x += 1;
            self.ents[BAT_R].x -= 1;
            self.ents[TOP_L].x += 1;
            self.ents[TOP_R].x -= 1;
            self.update_player_position();
            self.refresh();
        }
        self.bat_w -= 16;
    }
}

// ------------------------------------------------------------------------------------------------ the ball
impl Jail {
    /// The bat segment (0 to 7) a ball at `x` hits, if any: the original's unsigned division.
    fn segment_at(&self, x: i16) -> Option<i16> {
        let d = (x.wrapping_add(8).wrapping_sub(self.ents[BAT_L].x)) as u16 as u32;
        let div = (self.bat_w as u16 >> 3) as u32;
        let q = (d / div.max(1)) as u16 as i16;
        (0..=7).contains(&q).then_some(q)
    }

    /// A bat hit: the (silent) music pitch change, and every 40 hits the ball speeds up.
    fn bat_hit(&mut self, seg: i16) {
        self.segment = seg;
        self.pitch = self.pitch.wrapping_sub(1);
        let s = ((self.p().level as u16 % 10) >> 1) as i16 + 0x18;
        self.set_player_sprite(self.pitch, s);
        self.hits_to_speedup -= 1;
        if self.hits_to_speedup <= 0 {
            self.speed += 1;
            self.sfx(0xf);
            self.hits_to_speedup = 0x28;
        }
    }

    /// process_player_movement: a ball moves its steps for this frame, one pixel at a time, bouncing off the bat(s),
    /// the walls, the bricks and the moving bricks.
    fn process_player_movement(&mut self, vi: usize, ei: usize) {
        tr!(self, "process_player_movement");
        if self.serve_wait == -1 {
            return;
        }
        let mut d3 = self.vels[vi].ys.wrapping_add(self.speed);
        let mut d2 = self.vels[vi].xs.wrapping_add(self.speed);
        loop {
            if d3 > 0 {
                let y = self.ents[ei].y;
                let mut bounced = false;
                let mut skip_bottom = false;
                if self.pow & P_DOUBLE != 0 {
                    if (-6..=2).contains(&y) {
                        skip_bottom = true;
                        if let Some(q) = self.segment_at(self.ents[ei].x) {
                            self.bat_hit(q);
                            self.ents[ei].y = 2;
                            bounced = true;
                        }
                    }
                } else if y == 0x0c {
                    self.vels[vi].vy = 1;
                    self.sfx(7);
                    skip_bottom = true;
                }
                if !skip_bottom && (0xbe..=0xc6).contains(&y) {
                    if let Some(q) = self.segment_at(self.ents[ei].x) {
                        self.bat_hit(q);
                        self.ents[ei].y = 0xbe;
                        bounced = true;
                    }
                }
                if bounced {
                    self.set_guard_patrol_direction(vi, ei);
                    if self.serve_wait == -1 {
                        return;
                    }
                }
                self.ents[ei].y = self.ents[ei].y.wrapping_add(self.vels[vi].vy);
            }
            if d2 > 0 {
                let x = self.ents[ei].x;
                if x == 0x5a {
                    self.vels[vi].vx = 1;
                    self.sfx(3);
                } else if x == 0x126 {
                    self.vels[vi].vx = -1;
                    self.sfx(3);
                }
                self.ents[ei].x = self.ents[ei].x.wrapping_add(self.vels[vi].vx);
            }
            self.process_tile_interaction(vi, ei);
            self.check_object_collision(vi, ei);
            d3 -= 1;
            d2 -= 1;
            if d3 <= 0 && d2 <= 0 {
                return;
            }
        }
    }

    /// set_guard_patrol_direction (misnamed): a bat hit: caught (hold pill), or sent back at the angle of the segment
    /// hit.
    fn set_guard_patrol_direction(&mut self, vi: usize, ei: usize) {
        tr!(self, "set_guard_patrol_direction");
        if self.launch_guard != -1 && self.pow & P_HOLD != 0 {
            self.serve_timer = 0xb4;
            self.serve_wait = -1;
            self.sfx(10);
            self.launch_guard = 0;
            return;
        }
        let v = &mut self.vels[vi];
        let neg = if self.ents[ei].y > 0x68 {
            v.vy >= 0
        } else {
            v.vy <= 0
        };
        if neg {
            v.vy = -v.vy;
        }
        let t = match self.segment {
            0 => Some((-1, 1, 2)),
            1 | 2 => Some((-1, 1, 1)),
            3 => Some((-1, 2, 1)),
            4 => Some((1, 2, 1)),
            5 | 6 => Some((1, 1, 1)),
            7 => Some((1, 1, 2)),
            _ => None,
        };
        if let Some((vx, ys, xs)) = t {
            v.vx = vx;
            v.ys = ys;
            v.xs = xs;
            self.sfx(2);
        }
        self.launch_guard = 0;
    }

    /// process_tile_interaction: a ball (or shot) on a brick: scores, pills, hit animations, multi-hit and gold
    /// bricks, and the level's end when the last brick goes.
    fn process_tile_interaction(&mut self, vi: usize, ei: usize) {
        tr!(self, "process_tile_interaction");
        let (y, x) = (self.ents[ei].y, self.ents[ei].x);
        if !(8..0xc8).contains(&y) {
            return;
        }
        let k = ((y + 8) as u16 >> 3) as usize * 13 + ((x + 8 - 0x60) as u16 >> 4) as usize;
        if k >= BRICK_MAP {
            return;
        }
        let cell = self.players[self.cur].map[k];
        let t = ((cell >> 4) & 15) as i16 + 0x10;
        if t == 0x10 || t == 0x1f {
            return;
        }
        let mut level_done = false;
        if t < 0x19 {
            self.pm().score = self.p().score.wrapping_add(t - 0x10);
            self.store_player_state(cell, ei);
            let a = ((((cell >> 4) & 15) as i16 - 1) * 6) as u8;
            self.anim[k] = a.wrapping_add(0x8c);
            let img = self.ents[ei].img;
            if img == 0x4a || img == 0x4b || self.pow & P_MINE != 0 {
                self.anim[k] = self.anim[k].wrapping_add(0x30);
                self.sfx(8);
            } else {
                self.reflect_player_bounce(vi, ei);
                self.sfx(4);
            }
            self.players[self.cur].map[k] = 0;
            self.bricks_left -= 1;
            level_done = self.bricks_left <= 0;
        } else if t == 0x19 {
            self.anim[k] = 0xec;
            self.update_player_inventory_count(vi);
            self.reflect_player_bounce(vi, ei);
            self.sfx(7);
        } else if t == 0x1a {
            self.pm().score = self.p().score.wrapping_add(2);
            self.store_player_state(cell, ei);
            self.anim[k] = 0xbc;
            self.reflect_player_bounce(vi, ei);
            self.sfx(8);
            self.players[self.cur].map[k] = 0;
            self.bricks_left -= 1;
            level_done = self.bricks_left <= 0;
        } else {
            self.pm().score = self.p().score.wrapping_add(2);
            self.players[self.cur].map[k] = cell.wrapping_sub(0x10);
            self.anim[k] = 0xf2;
            self.update_player_inventory_count(vi);
            self.reflect_player_bounce(vi, ei);
            self.sfx(6);
        }
        if level_done {
            self.pm().level += 1;
            self.init_game_state_3();
            return;
        }
        match self.ents[ei].img {
            0x4a => {
                self.ents[SHOT1].y = 0xd0;
                self.refresh();
                self.ents[SHOT1].img = 0;
            }
            0x4b => {
                self.ents[SHOT2].y = 0xd0;
                self.refresh();
                self.ents[SHOT2].img = 0;
            }
            _ => {}
        }
    }

    /// reflect_player_bounce: the ball turns at the brick edge it reached.
    fn reflect_player_bounce(&mut self, vi: usize, ei: usize) {
        tr!(self, "reflect_player_bounce");
        let e = self.ents[ei];
        let v = &mut self.vels[vi];
        let cx = (e.x + 8) & 15;
        if (v.vx < 0 && cx == 15) || (v.vx >= 0 && cx == 0) {
            v.vx = -v.vx;
        }
        let cy = (e.y + 8) & 7;
        if (v.vy < 0 && cy == 7) || (v.vy >= 0 && cy == 0) {
            v.vy = -v.vy;
        }
    }

    /// check_object_collision: a ball on a moving brick bounces.
    fn check_object_collision(&mut self, vi: usize, ei: usize) {
        tr!(self, "check_object_collision");
        for i in 0..self.movers.len() {
            let m = self.movers[i];
            if m.kind == 0 {
                break;
            }
            let e = self.ents[ei];
            let (d1, d2) = (e.y + 8, e.x + 8);
            if d1 < m.y || d1 > m.y + 7 || d2 < m.x || d2 > m.x + 15 {
                continue;
            }
            self.sfx(6);
            let v = &mut self.vels[vi];
            let cx = d2 & 15;
            if (v.vx >= 0 && cx == 0) || (v.vx < 0 && cx != 15) {
                v.vx = -v.vx;
            }
            let cy = d1 & 7;
            if (v.vy >= 0 && cy == 0) || (v.vy < 0 && cy != 7) {
                v.vy = -v.vy;
            }
        }
    }

    /// update_player_inventory_count (misnamed): a hard brick nudges the ball's speeds at random.
    fn update_player_inventory_count(&mut self, vi: usize) {
        tr!(self, "update_player_inventory_count");
        let r = self.random(8);
        let v = &mut self.vels[vi];
        match r {
            0 => v.ys = (v.ys + 1).min(2),
            1 => {
                v.ys -= 1;
                if v.ys <= 0 {
                    v.ys = 1;
                }
            }
            2 => v.xs = (v.xs + 1).min(2),
            3 => {
                v.xs -= 1;
                if v.xs <= 0 {
                    v.xs = 1;
                }
            }
            _ => {}
        }
    }

    /// store_player_state (misnamed): a brick holding a pill lets it fall (one pill at a time).
    fn store_player_state(&mut self, cell: u8, ei: usize) {
        tr!(self, "store_player_state");
        if self.ents[PILL].img != 0 {
            return;
        }
        let p = (cell & 15) as i16;
        if p == 0 {
            return;
        }
        self.pill_effect = p;
        self.ents[PILL].img = p;
        self.ents[PILL].y = self.ents[ei].y;
        self.ents[PILL].x = (self.ents[ei].x + 8) & !15;
    }

    /// handle_player_escape (misnamed): the pill falls; caught by the bat it scores 10 and takes effect.
    fn handle_player_escape(&mut self) {
        tr!(self, "handle_player_escape");
        if self.ents[PILL].img == 0 {
            return;
        }
        self.ents[PILL].y += 2;
        let y = self.ents[PILL].y;
        if y < 0xbe {
            return;
        }
        if y >= 0xc6 {
            if y >= 0xd2 {
                self.ents[PILL].img = 0;
                self.ents[PILL].y = 0xd0;
            }
            return;
        }
        let d = self.ents[PILL].x - self.ents[BAT_L].x;
        if d < -16 || d > self.bat_w {
            return;
        }
        self.ents[PILL].y = 0xd0;
        self.refresh();
        self.ents[PILL].img = 0;
        self.pm().score = self.p().score.wrapping_add(10);
        self.process_player_input();
    }

    /// update_prisoner_state (misnamed): the shots fly (up from the bat, down from the top bat).
    fn update_prisoner_state(&mut self) {
        tr!(self, "update_prisoner_state");
        if self.ents[SHOT1].img != 0 {
            if self.ents[SHOT1].y >= 0 {
                self.vels[V_SHOT1].vy = -1;
                let d = (self.vels[V_SHOT1].ys as u16 as u32).wrapping_mul(0xffff) as u16 as i16;
                self.ents[SHOT1].y = self.ents[SHOT1].y.wrapping_add(d);
                self.process_tile_interaction(V_SHOT1, SHOT1);
            } else {
                self.ents[SHOT1].y = 0xd0;
                self.refresh();
                self.ents[SHOT1].img = 0;
                return;
            }
        }
        if self.pow & P_DOUBLE == 0 || self.ents[SHOT2].img == 0 {
            return;
        }
        if self.ents[SHOT2].y <= 0xd0 {
            self.vels[V_SHOT2].vy = 1;
            let d = self.vels[V_SHOT2].ys;
            self.ents[SHOT2].y = self.ents[SHOT2].y.wrapping_add(d);
            self.process_tile_interaction(V_SHOT2, SHOT2);
        } else {
            self.ents[SHOT2].y = 0xd0;
            self.refresh();
            self.ents[SHOT2].img = 0;
        }
    }

    fn ball_pic(&self) -> i16 {
        if self.pow & P_MINE != 0 {
            0x84
        } else if self.pow & P_GIANT != 0 {
            0x7c
        } else {
            0x4c
        }
    }

    /// process_player_input (misnamed): a pill's effect.
    fn process_player_input(&mut self) {
        tr!(self, "process_player_input");
        if self.pill_effect == 15 {
            self.pill_effect = self.random(15) + 1;
        }
        match self.pill_effect {
            1 => self.pill_cannon(),
            2 => {
                if self.speed != 0 {
                    self.sfx(16);
                    self.speed -= 1;
                    if self.speed < 0 {
                        self.speed = 0;
                    }
                }
            }
            3 => {
                self.sfx(15);
                self.speed += 1;
            }
            4 => {
                if self.pow & P_MINE != 0 {
                    return;
                }
                self.pow |= P_MINE;
                self.sfx(9);
                self.pow &= !P_GIANT;
                self.set_ball_pics(0x84);
            }
            5 => {
                self.pm().level += 1;
                self.init_game_state_3();
            }
            6 => self.pill_hold(),
            7 => {
                self.sfx(19);
                self.pm().lives += 1;
                self.wait_three_frames();
            }
            8 => {
                self.pow |= P_KILL;
                self.initialize_game_state_2();
            }
            9 => self.pill_balls(),
            10 => {
                if self.pow & (P_THIN | P_WIDE) != 0 || self.bat_w == 0x30 {
                    return;
                }
                self.pow |= P_WIDE;
                self.sfx(14);
                if self.pow & P_CANNON != 0 {
                    self.pow &= !P_CANNON;
                    self.bat_img = 0x50;
                }
                self.set_tile_collision_flags(48);
            }
            11 => {
                if self.pow & P_GIANT != 0 {
                    return;
                }
                self.pow |= P_GIANT;
                self.sfx(11);
                self.pow &= !P_MINE;
                self.set_ball_pics(0x7c);
            }
            12 => {
                if self.pow & (P_THIN | P_WIDE) != 0 || self.bat_w == 0x10 {
                    return;
                }
                self.pow |= P_THIN;
                self.sfx(13);
                if self.pow & P_CANNON != 0 {
                    self.pow &= !P_CANNON;
                    self.bat_img = 0x50;
                }
                self.set_tile_collision_flags(16);
            }
            13 => {
                if self.pow & P_DOUBLE != 0 {
                    return;
                }
                self.pow |= P_DOUBLE;
                self.sfx(20);
                self.wait_for_input();
                self.set_tile_collision_flags(self.bat_w);
            }
            14 => self.pow ^= P_REVERSE,
            _ => {}
        }
    }

    fn set_ball_pics(&mut self, p: i16) {
        for b in [BALL1, BALL2, BALL3] {
            if self.ents[b].img != 0 {
                self.ents[b].img = p;
            }
        }
    }

    fn pill_cannon(&mut self) {
        if self.pow & P_CANNON != 0 {
            return;
        }
        self.pow |= P_CANNON;
        self.sfx(17);
        self.vels[V_SHOT2].vy = 1;
        self.vels[V_SHOT1].vy = -1;
        self.pow &= !P_HOLD;
        self.bat_img = 0x68;
        self.set_tile_collision_flags(32);
        self.pow &= !P_BALLS;
        if self.ents[BALL1].img != 0 {
            self.ents[BALL2].img = 0;
            self.ents[BALL3].img = 0;
            self.vels[V_BALL1].vy = -1;
            return;
        }
        self.ents[BALL1].img = self.ball_pic();
        let (fe, fv) = if self.ents[BALL2].img != 0 {
            (BALL2, V_BALL2)
        } else {
            (BALL3, V_BALL3)
        };
        self.ents[BALL1].y = self.ents[fe].y;
        self.ents[BALL1].x = self.ents[fe].x;
        let f = self.vels[fv];
        let v = &mut self.vels[V_BALL1];
        v.vx = f.vx;
        v.ys = f.ys;
        v.xs = f.xs;
        self.ents[BALL2].img = 0;
        self.ents[BALL3].img = 0;
        self.vels[V_BALL1].vy = -1;
    }

    fn pill_hold(&mut self) {
        if self.pow & P_HOLD != 0 {
            return;
        }
        self.pow |= P_HOLD;
        self.sfx(10);
        self.bat_img = 0x58;
        self.set_tile_collision_flags(self.bat_w);
        self.pow &= !P_CANNON;
        self.pow &= !P_BALLS;
        if self.ents[BALL1].img != 0 {
            self.ents[BALL2].y = 0xd0;
            self.ents[BALL3].y = 0xd0;
            self.refresh();
            self.ents[BALL2].img = 0;
            self.ents[BALL3].img = 0;
        } else if self.ents[BALL2].img != 0 {
            self.ents[BALL3].y = 0xd0;
            self.refresh();
            self.ents[BALL3].img = 0;
        }
    }

    fn pill_balls(&mut self) {
        if self.pow & P_BALLS != 0 {
            return;
        }
        self.pow |= P_BALLS;
        self.sfx(12);
        self.serve_wait = 0;
        self.launch_guard = -1;
        self.serve_timer = -1;
        let by = self.ents[BAT_R].y - 16;
        let bx = self.ents[BAT_L].x;
        for (e, v, dx, vx) in [
            (BALL1, V_BALL1, 0, 1),
            (BALL2, V_BALL2, 7, -1),
            (BALL3, V_BALL3, 15, 1),
        ] {
            if self.ents[e].img == 0 {
                self.ents[e].y = by;
                self.ents[e].x = bx + dx;
                self.vels[v] = Vel {
                    vy: -1,
                    vx,
                    ys: 1,
                    xs: 1,
                };
            }
        }
        self.pow &= !(P_HOLD | P_CANNON);
        self.bat_img = 0x50;
        self.set_tile_collision_flags(self.bat_w);
        if self.pow & P_MINE != 0 {
            self.ents[BALL1].img = 0x84;
            self.ents[BALL2].img = 0x84;
            self.ents[BALL3].img = 0x84;
        } else if self.pow & P_GIANT != 0 {
            self.ents[BALL1].img = 0x7c;
            self.ents[BALL2].img = 0x7c;
            self.ents[BALL3].img = 0x7c;
        } else {
            self.ents[BALL1].img = 0x4c;
            self.ents[BALL2].img = 0x4d;
            self.ents[BALL3].img = 0x4d;
        }
    }
}
