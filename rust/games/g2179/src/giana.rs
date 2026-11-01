//! The Great Giana Sisters (Rainbow Arts / Time Warp, 1987), rewritten natively from its recovered code. Each method
//! keeps the name of the recovered routine it was written from (see `docs/native/g2179-native.md` and the function
//! map kept beside the asset pack), so the two read side by side.
//!
//! The program is written as the original was: nested routines that wait for the beam to reach a line. It runs on
//! its own thread in lock-step with the engine (`co.rs`); `wait_line` is the only place a frame ends. The original's
//! state is kept in typed fields named after what they hold; its pictures are kept as it kept them, as palette-index
//! pictures it draws bobs, tiles and text into (the playfield of a whole stage, the status bar, the message and
//! high-score screens), and the frame is composed from those as its display lists showed them.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use native_engine::gfx::{rgb12, Argb, Bitmap};
use native_engine::input::{Action, InputState};

use crate::assets::{self, Assets, HsEntry, Stage, CLEAR, H, W};
use crate::co::{FrameIn, FrameOut, Stop};
use crate::music::{Player, Sound};

/// The original's "nothing here" marker for a slot position.
pub const NONE: i32 = 0x1234;
/// Lines in a PAL frame.
const FRAME_LINES: i32 = 313;
/// The first display line (the original adds it to every line it waits for).
const TOP: i32 = 0x2C;

/// A composed frame: palette indices (0-31 the colour registers, 32-47 a line's colour bar), and a direct-colour
/// picture for the hold-and-modify title.
#[derive(Clone)]
pub struct View {
    pub px: Vec<u8>,
    pub pal: [Argb; 64],
    pub direct: Option<Vec<Argb>>,
}

/// What the display shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Disp {
    Logo,
    TitlePicture,
    /// The title: the text area with its colour bar over the scrolling attract stage.
    Title,
    Game,
    Message,
    HighScore,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tgt {
    Playfield,
    Status,
}

/// A bob waiting for the frame's drawing pass (`queue_bob`).
#[derive(Clone, Copy)]
pub(crate) struct Queued {
    tgt: Tgt,
    img: usize,
    x: i32,
    y: i32,
    save: bool,
}

/// The background under a moving bob, restored next frame (`save_bob_background`).
pub(crate) struct Saved {
    tgt: Tgt,
    start: usize,
    w: usize,
    rows: usize,
    px: Vec<u8>,
}

/// An enemy or item record (the original's 62-byte record; state 0 = free, 1 enemy, 2 item, 3 shot dead,
/// 4 squashed, 5 harmless to stand on).
#[derive(Clone, Default, Debug)]
pub struct Enemy {
    pub state: i32,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub anim: [usize; 6],
    pub points: u32,
    pub movement: i32,
    pub path: Arc<Vec<i32>>,
    pub w: i32,
    pub h: i32,
    pub tick: i32,
    pub shift: u32,
    pub xoff: i32,
    pub yoff: i32,
    pub step: i32,
    pub idx: i32,
    pub dy: i32,
    pub hp: i32,
}

/// A small fixed list of positions with a counter (falling bridge pieces, breaking bricks, dropped blocks, flying
/// diamonds): x 0 marks a free entry, as in the original.
pub type Slots = Vec<[i32; 3]>;

/// One player's saved turn.
#[derive(Clone, Copy, Default)]
pub struct Turn {
    pub score: u32,
    pub lives: u8,
    pub stage_bcd: u8,
    pub stage: i32,
    pub bonus_idx: i32,
    pub scroll: i32,
    pub active: bool,
}

pub struct Giana {
    pub a: Arc<Assets>,
    pub(crate) rx: Receiver<FrameIn>,
    pub(crate) tx: Sender<FrameOut>,
    pub input: InputState,
    pub(crate) beam: i32,
    pub frame_no: u64,
    pub snd: Sound,
    pub music: Player,
    pub(crate) boot_music: Player,
    pub(crate) boot_music_on: bool,
    /// Sound-effect step armed for the vertical blank (2 = switch on, 1 = then silence).
    pub(crate) sfx_step: u8,
    pub(crate) saved_hs: Option<Vec<HsEntry>>,
    pub(crate) save_out: Option<Vec<HsEntry>>,

    // display
    pub disp: Disp,
    pub dma_display: bool,
    pub dma_sprites: bool,
    pub col: [u16; 32],
    pub pf: Bitmap,
    pub status: Bitmap,
    pub msg: Bitmap,
    pub hs: Bitmap,
    /// The playfield scroll the display list holds (`set_playfield_scroll`).
    pub shown_scroll: i32,
    pub hs_row: i32,
    pub hs_shift: Vec<i32>,
    pub logo_cycle_flip: bool,
    pub(crate) queue: Vec<Queued>,
    pub(crate) saves: Vec<Saved>,
    pub cloud_pos: [(i32, i32); 6],
    pub cursor: (i32, i32),

    // stage
    pub st: Stage,
    pub stage: i32,
    pub stage_bcd: u8,
    pub scroll: i32,
    pub prev_scroll: i32,
    pub bonus_return_scroll: i32,
    pub attract_dir: i32,
    pub cloud_acc: i32,
    pub clouds_x: (i32, i32),
    pub objects: [Option<Vec<(u16, u16)>>; 6],
    pub bridge_marks: Vec<(i32, i32)>,
    pub bonus_entry: Option<isize>,
    pub enemy_spawned: Vec<bool>,
    pub lift_x: i32,
    pub lift_y: i32,
    pub lift_dir: i32,

    // player
    pub px: i32,
    pub py: i32,
    pub saved_pos: (i32, i32),
    pub jump_i: i32,
    pub fall_i: i32,
    pub speed: i32,
    pub anim_tick: i32,
    pub saved_speed: i32,
    pub falling: u8,
    pub jumping: u8,
    pub airborne: bool,
    pub feet: u8,
    pub head_free: bool,
    pub head: u8,
    pub left_free: bool,
    pub right_free: bool,
    pub tiles: [u8; 12],
    pub enemy_tiles: [u8; 6],
    pub player_cell: Option<isize>,
    pub forced_jump: u8,
    pub jump_latch: bool,
    pub fire_latch: bool,
    pub on_lift: bool,
    pub in_bonus: bool,
    pub dream_over: bool,
    pub game_over: bool,
    pub warp: bool,
    pub game_ended: bool,
    pub clouds_on: bool,
    pub freeze: i32,
    pub facing_left: bool,
    pub saved_facing: bool,
    pub life_over: bool,
    pub quit: bool,
    pub dying: bool,
    pub cleared: bool,
    pub drawn: bool,
    pub frame_set: usize,
    pub walk_r: i32,
    pub walk_l: i32,
    pub transform_t: i32,
    pub death_t: i32,
    pub powered: u8,
    pub shot_level: u8,
    pub freezes: u8,
    pub kill_alls: u8,
    pub protection: u8,
    pub extra_life: u8,
    pub item_idx: i32,

    // joystick
    pub fire: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,

    // shot and effects
    pub shot: (i32, i32),
    pub shot_dx: i32,
    pub arc_dir: i32,
    pub arc_i: i32,
    pub homing: Option<usize>,
    pub homing_path: [(i32, i32); 10],
    pub shot_anim: i32,
    pub explosion: (i32, i32),
    pub explosion_t: i32,
    pub bump: [(i32, i32); 2],
    pub bump_t: [i32; 2],
    pub debris: [(i32, i32); 2],
    pub debris_t: [i32; 2],
    pub multi: [(i32, i32); 2],
    pub multi_t: [i32; 2],
    pub item_pos: (i32, i32),
    pub item_rise: i32,
    pub breaking: Slots,
    pub dropped: Slots,
    pub flying: Slots,
    pub falling_bridges: Slots,
    pub enemies: Vec<Enemy>,
    pub shimmer_t: i32,
    pub dornen_t: i32,
    pub flamme_t: i32,

    // score and turns
    pub score: u32,
    pub shown_score: u32,
    pub pending: u32,
    pub diamonds: u8,
    pub shown_diamonds: u8,
    pub lives: u8,
    pub shown_lives: u8,
    pub time: u8,
    pub shown_time: u8,
    pub second_t: i32,
    pub tally_t: i32,
    pub player_colour: u16,
    pub player: i32,
    pub turns: [Turn; 2],
    pub bonus_idx: i32,
    pub name: [u8; 5],
    pub trainer: bool,
    pub trainer_used: bool,
    pub hs_blink: u8,
    pub hs_count: i32,
    pub hs_cursor: usize,
    pub hs_touched: [bool; 10],
    pub hs_name: [u8; 10],
    pub logo_cycle: [u16; 16],
    pub t_score: [u8; 6],
    pub t_dia: [u8; 2],
    pub t_lives: [u8; 2],
    pub t_stage: [u8; 2],
    pub t_time: [u8; 2],

    // high scores
    pub hs_all: Vec<HsEntry>,
    pub hs_today: Vec<HsEntry>,
    pub hs_loaded: bool,
}

pub fn slots(n: usize) -> Slots {
    vec![[0; 3]; n]
}

impl Giana {
    pub fn new(a: Arc<Assets>, rx: Receiver<FrameIn>, tx: Sender<FrameOut>) -> Giana {
        let st = a.stages[0].clone();
        let pool = a.enemy_pool;
        let hs_all = a.hs_all_time.clone();
        let hs_today = a.hs_today.clone();
        Giana {
            a,
            rx,
            tx,
            input: InputState::default(),
            beam: 0,
            frame_no: 0,
            snd: Sound::default(),
            music: Player::default(),
            boot_music: Player::default(),
            boot_music_on: false,
            sfx_step: 0,
            saved_hs: None,
            save_out: None,
            disp: Disp::Logo,
            dma_display: false,
            dma_sprites: false,
            col: [0; 32],
            pf: Bitmap::new(2048, 160),
            status: Bitmap::new(384, 48),
            msg: Bitmap::new(336, 200),
            hs: Bitmap::new(336, 400),
            shown_scroll: 0,
            hs_row: 0,
            hs_shift: vec![0; 200],
            logo_cycle_flip: false,
            queue: Vec::new(),
            saves: Vec::new(),
            cloud_pos: [(-100, 0); 6],
            cursor: (0, 0),
            st,
            stage: 0,
            stage_bcd: 0,
            scroll: 0,
            prev_scroll: 0,
            bonus_return_scroll: -1,
            attract_dir: 0,
            cloud_acc: 0,
            clouds_x: (0x0A, 0x82),
            objects: Default::default(),
            bridge_marks: Vec::new(),
            bonus_entry: None,
            enemy_spawned: Vec::new(),
            lift_x: 0,
            lift_y: 0,
            lift_dir: 0,
            px: 0,
            py: 0,
            saved_pos: (0, 0),
            jump_i: 0,
            fall_i: 0,
            speed: 0,
            anim_tick: 0,
            saved_speed: 0,
            falling: 0,
            jumping: 0,
            airborne: false,
            feet: 0,
            head_free: false,
            head: 0,
            left_free: false,
            right_free: false,
            tiles: [0; 12],
            enemy_tiles: [0; 6],
            player_cell: None,
            forced_jump: 0,
            jump_latch: false,
            fire_latch: false,
            on_lift: false,
            in_bonus: false,
            dream_over: false,
            game_over: false,
            warp: false,
            game_ended: false,
            clouds_on: false,
            freeze: 0,
            facing_left: false,
            saved_facing: false,
            life_over: false,
            quit: false,
            dying: false,
            cleared: false,
            drawn: false,
            frame_set: 0,
            walk_r: 0,
            walk_l: 0,
            transform_t: 0,
            death_t: 0,
            powered: 0,
            shot_level: 0,
            freezes: 0,
            kill_alls: 0,
            protection: 0,
            extra_life: 0,
            item_idx: 0,
            fire: false,
            right: false,
            left: false,
            up: false,
            down: false,
            shot: (NONE, 0),
            shot_dx: 0,
            arc_dir: 1,
            arc_i: 0,
            homing: None,
            homing_path: [(0, 0); 10],
            shot_anim: 0,
            explosion: (NONE, 0),
            explosion_t: 0,
            bump: [(NONE, 0); 2],
            bump_t: [0; 2],
            debris: [(NONE, 0); 2],
            debris_t: [0; 2],
            multi: [(NONE, 0); 2],
            multi_t: [0; 2],
            item_pos: (NONE, 0),
            item_rise: 0,
            breaking: slots(5),
            dropped: slots(15),
            flying: slots(15),
            falling_bridges: slots(5),
            enemies: vec![Enemy::default(); pool],
            shimmer_t: 0,
            dornen_t: 0,
            flamme_t: 0,
            score: 0,
            shown_score: 0,
            pending: 0,
            diamonds: 0,
            shown_diamonds: 0,
            lives: 0,
            shown_lives: 0,
            time: 0,
            shown_time: 0,
            second_t: 0,
            tally_t: 0,
            player_colour: 0,
            player: 0,
            turns: [Turn {
                active: true,
                ..Default::default()
            }; 2],
            bonus_idx: 0,
            name: [b' '; 5],
            trainer: false,
            trainer_used: false,
            hs_blink: 0,
            hs_count: 0,
            hs_cursor: 0,
            hs_touched: [false; 10],
            hs_name: [b' '; 10],
            logo_cycle: [0; 16],
            t_score: *b"000000",
            t_dia: *b"00",
            t_lives: *b"00",
            t_stage: *b"00",
            t_time: *b"00",
            hs_all,
            hs_today,
            hs_loaded: false,
        }
    }

    /// The whole program: the boot program, then the game (`supervisor_start`).
    pub fn run(&mut self) {
        self.boot_main();
        self.main_loop();
    }

    /// The named state the verification compares with the original's memory.
    pub fn named_state(&self) -> Vec<(String, i64)> {
        let mut v: Vec<(String, i64)> = vec![
            ("disp".into(), self.disp as i64),
            ("display".into(), self.dma_display as i64),
            ("stage".into(), self.stage as i64),
            ("scroll".into(), self.scroll as i64),
            ("px".into(), self.px as i64),
            ("py".into(), self.py as i64),
            ("speed".into(), self.speed as i64),
            ("jumping".into(), self.jumping as i64),
            ("falling".into(), self.falling as i64),
            ("jump_i".into(), self.jump_i as i64),
            ("fall_i".into(), self.fall_i as i64),
            ("score".into(), self.score as i64),
            ("diamonds".into(), self.diamonds as i64),
            ("lives".into(), self.lives as i64),
            ("time".into(), self.time as i64),
            ("powered".into(), self.powered as i64),
            ("shot_level".into(), self.shot_level as i64),
            ("in_bonus".into(), self.in_bonus as i64),
            ("dying".into(), self.dying as i64),
            ("player".into(), self.player as i64),
            ("lift_y".into(), self.lift_y as i64),
            ("shot_x".into(), self.shot.0 as i64),
            ("trainer".into(), self.trainer as i64),
            ("hs_count".into(), self.hs_count as i64),
            ("hs_cursor".into(), self.hs_cursor as i64),
            ("hs_blink".into(), self.hs_blink as i64),
            ("beam".into(), self.beam as i64),
        ];
        for (i, e) in self.enemies.iter().enumerate() {
            v.push((format!("e{i}_state"), e.state as i64));
            v.push((format!("e{i}_x"), e.x as i64));
            v.push((format!("e{i}_y"), e.y as i64));
        }
        v
    }

    // ------------------------------------------------------------------ frames and timing

    /// The end of a frame: hand the picture and this frame's sounds to the engine, take the next frame's input, then
    /// do what the original's vertical-blank interrupt did.
    fn frame(&mut self) {
        let view = self.compose();
        let out = FrameOut {
            view,
            sounds: std::mem::take(&mut self.snd.out),
            save: self.save_out.take(),
            state: self.named_state(),
        };
        if self.tx.send(out).is_err() {
            std::panic::resume_unwind(Box::new(Stop));
        }
        let Ok(inp) = self.rx.recv() else {
            std::panic::resume_unwind(Box::new(Stop));
        };
        if inp.saved_hs.is_some() {
            self.saved_hs = inp.saved_hs;
        }
        if inp.input.pressed(Action::Secondary) {
            // The WHDLoad install's trainer key: life loss on or off (off by default). As there, a session that
            // used it no longer saves its high scores.
            self.trainer = !self.trainer;
            self.trainer_used = true;
        }
        self.input = inp.input;
        self.frame_no += 1;
        self.vertical_blank();
    }

    /// `vblank_interrupt`: the sound-effect step and the music.
    fn vertical_blank(&mut self) {
        if self.boot_music_on {
            // The boot program ticks its music in its own loop (`title_picture_loop`), once a frame.
            return;
        }
        self.sound_effect_dma_step();
        let a = self.a.clone();
        self.music.tick(&a.music, &mut self.snd);
    }

    /// `wait_raster_line` (and `boot_wait_raster_line`): waits until the beam reaches line `l` of the display. If the program is already past it,
    /// the wait runs into the next frame.
    pub fn wait_line(&mut self, l: i32) {
        let t = l + TOP;
        if t > self.beam {
            self.beam = t;
        } else {
            self.frame();
            self.beam = t;
        }
    }

    /// Time the original spent working, in display lines (measured costs of its long loops and drawing).
    pub fn busy(&mut self, lines: i32) {
        self.beam += lines;
        while self.beam >= FRAME_LINES {
            self.beam -= FRAME_LINES;
            self.frame();
        }
    }

    /// `delay_loop`: an empty counting loop of `n` steps (about 26 processor cycles each, 454 a line).
    pub fn delay_loop(&mut self, n: u32) {
        self.busy((n as i64 * 26 / 454) as i32);
    }

    /// `display_on`.
    pub fn display_on(&mut self) {
        self.wait_line(0xC8);
        self.dma_display = true;
        self.dma_sprites = true;
    }

    /// `display_off`.
    pub fn display_off(&mut self) {
        self.wait_line(0xC8);
        self.dma_display = false;
        self.dma_sprites = false;
    }

    // ------------------------------------------------------------------ input

    /// `read_joystick` (and the boot program's `boot_read_joystick`): fire, right, left, up (jump) and down; a forced-jump count holds up on its own.
    pub fn read_joystick(&mut self) {
        let i = &self.input;
        self.fire = i.held(Action::Fire);
        self.right = i.held(Action::Right);
        self.left = i.held(Action::Left);
        let up = i.held(Action::Up) || i.held(Action::Fire2);
        if self.forced_jump != 0 {
            self.forced_jump -= 1;
            self.up = true;
        } else {
            self.up = up;
        }
        self.down = i.held(Action::Down);
    }

    /// The left mouse button (quit the game; save the high scores).
    pub fn mouse_button(&self) -> bool {
        self.input.held(Action::Cancel) || self.input.held(Action::Primary)
    }

    /// `wait_fire`: waits for the fire button to be let go (if held) and pressed again.
    pub fn wait_fire(&mut self) {
        if self.fire {
            loop {
                self.read_joystick();
                if !self.fire {
                    break;
                }
                self.busy(1);
            }
        }
        loop {
            self.read_joystick();
            if self.fire {
                break;
            }
            self.busy(1);
        }
    }

    // ------------------------------------------------------------------ sound

    /// `play_sound_effect`: effect `n` on channel four, switched on at the next vertical blank.
    pub fn play_sound_effect(&mut self, n: usize) {
        let a = self.a.clone();
        let e = &a.sfx[n];
        self.snd.off(3);
        self.snd.set_block(
            3,
            native_engine::audio::Block {
                data: e.data.clone(),
                start: 0,
                len: e.data.len(),
            },
        );
        self.snd.period(3, e.period);
        self.snd.volume(3, e.volume as u16);
        self.sfx_step = 2;
    }

    /// `sound_effect_dma_step`.
    fn sound_effect_dma_step(&mut self) {
        match self.sfx_step {
            2 => {
                self.snd.on(3);
                self.sfx_step = 1;
            }
            1 => {
                self.snd
                    .set_block(3, native_engine::audio::Block::silence());
                self.sfx_step = 0;
            }
            _ => {}
        }
    }

    pub fn start_tune(&mut self, tune: usize) {
        let a = self.a.clone();
        let (tempo, three) = match tune {
            1 => (3, false),
            2 => (2, true),
            3 => (3, true),
            4 => (3, false),
            _ => (2, true),
        };
        self.music
            .start(&a.music, &mut self.snd, tune, tempo, three);
    }

    /// `start_title_tune`.
    pub fn start_title_tune(&mut self) {
        self.start_tune(1);
    }
    /// `start_stage_tune` / `start_stage_tune_channels`.
    pub fn start_stage_tune(&mut self) {
        self.start_tune(2);
    }
    /// `start_castle_tune`.
    pub fn start_castle_tune(&mut self) {
        self.start_tune(3);
    }
    /// `start_highscore_tune`.
    pub fn start_highscore_tune(&mut self) {
        self.start_tune(4);
    }
    /// `start_get_ready_tune`.
    pub fn start_get_ready_tune(&mut self) {
        self.start_tune(5);
    }
    /// `stop_music`.
    pub fn stop_music(&mut self) {
        self.music.stop(&mut self.snd);
    }

    // ------------------------------------------------------------------ drawing

    fn surface(&mut self, t: Tgt) -> (&mut Bitmap, i32) {
        match t {
            Tgt::Playfield => (&mut self.pf, 159),
            Tgt::Status => (&mut self.status, 39),
        }
    }

    /// `blit_bob` with `blit_masked_bob`: a bob into a picture at (x, y), clipped at the top and at the picture's last row, cookie-cut by its
    /// mask. Like the original it does not clip sideways (a bob past the right edge continues on the next row) and
    /// drops the columns its shift pushes past the bob's width. With `save` the background under it is kept first.
    pub fn blit_bob(&mut self, t: Tgt, img: usize, x: i32, y: i32, save: bool) {
        let a = self.a.clone();
        let im = &a.imgs[img].bmp;
        let (w, h) = (im.w, im.h as i32);
        let (bmp, maxrow) = self.surface(t);
        let stride = bmp.w;
        let s = (x & 15) as usize;
        let base = (((x as u16) & 0xFFF0) >> 3) as usize * 8;
        if y > maxrow {
            return;
        }
        let (sy0, rows, dy) = if y >= 0 {
            let r = if y + h > maxrow { maxrow - y + 1 } else { h };
            (0, r, y)
        } else {
            let e = y + h;
            if e <= 0 {
                return;
            }
            (-y, e, 0)
        };
        let start = dy as usize * stride + base;
        let len = bmp.px.len();
        let saved = if save {
            let mut v = Vec::with_capacity(w * rows as usize);
            for r in 0..rows as usize {
                for c in 0..w {
                    v.push(bmp.px.get(start + r * stride + c).copied().unwrap_or(0));
                }
            }
            Some(v)
        } else {
            None
        };
        for r in 0..rows as usize {
            let srow = (sy0 as usize + r) * w;
            for c in 0..w - s {
                let v = im.px[srow + c];
                if v == CLEAR {
                    continue;
                }
                let o = start + r * stride + s + c;
                if o < len {
                    bmp.px[o] = v;
                }
            }
        }
        if let Some(px) = saved {
            self.saves.push(Saved {
                tgt: t,
                start,
                w,
                rows: rows as usize,
                px,
            });
        }
    }

    /// `restore_bob_backgrounds`: puts back what last frame's moving bobs covered, newest first.
    pub fn restore_bob_backgrounds(&mut self) {
        while let Some(s) = self.saves.pop() {
            let (bmp, _) = self.surface(s.tgt);
            let stride = bmp.w;
            let len = bmp.px.len();
            for r in 0..s.rows {
                for c in 0..s.w {
                    let o = s.start + r * stride + c;
                    if o < len {
                        bmp.px[o] = s.px[r * s.w + c];
                    }
                }
            }
        }
    }

    /// `draw_bob`: queues a bob for the playfield, and for the status bar too where it reaches above the playfield.
    /// `save` = a moving bob (its background is restored next frame); otherwise it is drawn for good.
    pub fn draw_bob(&mut self, img: usize, x: i32, y: i32, save: bool) {
        self.queue_bob(Tgt::Playfield, img, x, y, save);
        if y < 0 {
            let sx = (x - self.scroll + 0x1F) as i16 as i32;
            self.queue_bob(Tgt::Status, img, sx, y + 0x28, save);
        }
    }

    /// `queue_bob`.
    fn queue_bob(&mut self, tgt: Tgt, img: usize, x: i32, y: i32, save: bool) {
        self.queue.push(Queued {
            tgt,
            img,
            x,
            y,
            save,
        });
    }

    /// `draw_queued_bobs`: draws the frame's bobs in the order they were queued.
    pub fn draw_queued_bobs(&mut self) {
        let q = std::mem::take(&mut self.queue);
        for b in q {
            self.blit_bob(b.tgt, b.img, b.x, b.y, b.save);
        }
    }

    /// `print_text`: text in the 8 by 8 font. Each character sets the two colour bits it covers (bits 2 and 3 of the
    /// colour index) in its whole cell; a carriage return moves down 8 rows to the start of the line.
    pub fn print_text(&mut self, which: u8, text: &[u8]) {
        let a = self.a.clone();
        for &ch in text {
            if ch == 0 {
                break;
            }
            if ch == 13 {
                self.cursor.1 += 8;
                self.cursor.0 = 0;
                continue;
            }
            let g = (ch as i32 - 0x20).clamp(0, 63) as usize;
            let (bmp, x0) = match which {
                0 => (&mut self.status, 32),
                1 => (&mut self.msg, 16),
                _ => (&mut self.hs, 16),
            };
            let x = x0 + self.cursor.0 * 8;
            for r in 0..8 {
                for c in 0..8 {
                    let v = a.font.px[r * a.font.w + g * 8 + c];
                    let o = (self.cursor.1 as usize + r) * bmp.w + x as usize + c;
                    if o < bmp.px.len() {
                        bmp.px[o] = (bmp.px[o] & 3) | v;
                    }
                }
            }
            self.cursor.0 += 1;
        }
    }

    /// Clears a text picture as the original did: colour bits 0 and 1 off, 2 and 3 on (colour 12).
    pub fn clear_text_picture(bmp: &mut Bitmap) {
        bmp.px.fill(12);
    }

    // ------------------------------------------------------------------ the picture

    /// The frame as the display showed it.
    fn compose(&self) -> View {
        let mut pal = [0u32; 64];
        for (i, c) in self.col.iter().enumerate() {
            pal[i] = rgb12(*c);
        }
        for (k, c) in self.a.title_bar.iter().take(16).enumerate() {
            pal[32 + k] = rgb12(*c);
        }
        let mut px = vec![0u8; W * H];
        let mut direct = None;
        if self.dma_display {
            match self.disp {
                Disp::Logo => {
                    let b = &self.a.logo.bmp;
                    for y in 0..H.min(b.h) {
                        for x in 0..W.min(b.w) {
                            px[y * W + x] = b.px[y * b.w + x] & 31;
                        }
                    }
                }
                Disp::TitlePicture => direct = Some(self.title_picture_colours()),
                Disp::Title | Disp::Game => {
                    let title = self.disp == Disp::Title;
                    for y in 0..40 {
                        for x in 0..W {
                            let v = self.status.px[y * 384 + 32 + x];
                            px[y * W + x] = if title && v == 4 {
                                32 + (y % 16) as u8
                            } else {
                                v
                            };
                        }
                    }
                    let sx = (self.shown_scroll + 1) as usize;
                    for y in 40..H {
                        let row = &self.pf.px[(y - 40) * 2048..(y - 39) * 2048];
                        for x in 0..W {
                            px[y * W + x] = row[(sx + x).min(2047)];
                        }
                    }
                    if self.dma_sprites && self.clouds_on {
                        self.draw_clouds(&mut px);
                    }
                }
                Disp::Message => {
                    for y in 0..H {
                        px[y * W..(y + 1) * W]
                            .copy_from_slice(&self.msg.px[y * 336 + 16..y * 336 + 16 + W]);
                    }
                }
                Disp::HighScore => {
                    for y in 0..H {
                        let src = (self.hs_row + y as i32).clamp(0, 399) as usize;
                        let d = (self.hs_shift[y] & 15) as usize;
                        for x in 0..W {
                            let v = self.hs.px[src * 336 + 16 - d + x];
                            px[y * W + x] = if v == 4 { 32 + (y % 16) as u8 } else { v };
                        }
                    }
                }
            }
        }
        View { px, pal, direct }
    }

    /// The cloud sprites: behind the pictures, so they show where the picture has colour 0.
    fn draw_clouds(&self, px: &mut [u8]) {
        for (k, &(sx, sy)) in self.cloud_pos.iter().enumerate() {
            let b = &self.a.clouds[k];
            for r in 0..b.h as i32 {
                let y = sy + r;
                if !(0..H as i32).contains(&y) {
                    continue;
                }
                for c in 0..16 {
                    let x = sx + c;
                    if !(0..W as i32).contains(&x) {
                        continue;
                    }
                    let v = b.px[r as usize * 16 + c as usize];
                    let o = y as usize * W + x as usize;
                    if v != 0 && px[o] == 0 {
                        px[o] = 16 + 4 * (k as u8 / 2) + v;
                    }
                }
            }
        }
    }

    /// The interlaced hold-and-modify title picture, both fields shown at once (each output line the mix of the two
    /// lines the fields showed there).
    fn title_picture_colours(&self) -> Vec<Argb> {
        let p = &self.a.title_picture;
        let b = &p.bmp;
        let mut lines = vec![[0u8; 3]; b.w * b.h];
        for y in 0..b.h {
            let (mut r, mut g, mut bl) = split(p.colours.first().copied().unwrap_or(0));
            for x in 0..b.w {
                let v = b.px[y * b.w + x];
                let k = v & 15;
                match v >> 4 {
                    0 => (r, g, bl) = split(p.colours.get(k as usize).copied().unwrap_or(0)),
                    1 => bl = k,
                    2 => r = k,
                    _ => g = k,
                }
                lines[y * b.w + x] = [r, g, bl];
            }
        }
        let mut out = vec![0u32; W * H];
        for y in 0..H {
            for x in 0..W.min(b.w) {
                let a = lines[((2 * y).min(b.h - 1)) * b.w + x];
                let c = lines[((2 * y + 1).min(b.h - 1)) * b.w + x];
                let m = |i: usize| ((a[i] as u32 + c[i] as u32) * 17 / 2).min(255);
                out[y * W + x] = 0xFF00_0000 | m(0) << 16 | m(1) << 8 | m(2);
            }
        }
        out
    }
}

fn split(c: u16) -> (u8, u8, u8) {
    (((c >> 8) & 15) as u8, ((c >> 4) & 15) as u8, (c & 15) as u8)
}

pub fn bcd_add(a: u8, b: u8, x: &mut bool) -> u8 {
    let mut lo = (a & 15) as u32 + (b & 15) as u32 + *x as u32;
    let mut hi = (a >> 4) as u32 + (b >> 4) as u32;
    if lo > 9 {
        lo -= 10;
        hi += 1;
    }
    *x = hi > 9;
    if hi > 9 {
        hi -= 10;
    }
    ((hi << 4) | lo) as u8
}

pub fn bcd_sub(a: u8, b: u8, x: &mut bool) -> u8 {
    let mut lo = (a & 15) as i32 - (b & 15) as i32 - *x as i32;
    let mut hi = (a >> 4) as i32 - (b >> 4) as i32;
    if lo < 0 {
        lo += 10;
        hi -= 1;
    }
    *x = hi < 0;
    if hi < 0 {
        hi += 10;
    }
    ((hi << 4) | lo) as u8
}

/// The three low bytes of `add` added to the three low bytes of `to` in packed decimal (`add_points`: the top byte
/// is left alone, as the original's three `abcd` steps did).
pub fn bcd_add3(to: u32, add: u32) -> u32 {
    let mut x = false;
    let mut out = to & 0xFF00_0000;
    for k in 0..3 {
        let a = (to >> (8 * k)) as u8;
        let b = (add >> (8 * k)) as u8;
        out |= (bcd_add(a, b, &mut x) as u32) << (8 * k);
    }
    out
}

/// `bcd_to_text`: packed decimal bytes to digits.
pub fn bcd_text(v: &[u8]) -> Vec<u8> {
    let mut o = Vec::new();
    for b in v {
        for d in [b >> 4, b & 15] {
            if d < 10 {
                o.push(b'0' + d);
            }
        }
    }
    o
}

pub use assets::{JUMP_L, JUMP_R, SKID_L, SKID_R, STAND_L, STAND_R, WALK_L, WALK_R};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_decimal_adds_carry_and_wrap() {
        let mut x = false;
        assert_eq!(bcd_add(0x99, 0x01, &mut x), 0x00);
        assert!(x);
        assert_eq!(bcd_add3(0x0000_0990, 0x25), 0x0000_1015);
        let mut x = false;
        assert_eq!(bcd_sub(0x10, 0x01, &mut x), 0x09);
        assert_eq!(bcd_text(&[0x01, 0x25]), b"0125".to_vec());
    }
}
