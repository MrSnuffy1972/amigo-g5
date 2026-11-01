//! The bridge between the engine's tick-at-a-time `Game` and IK+'s main program, which (like the original) is written
//! as ordinary nested code that waits for the next frame wherever the original did. The main program runs on its own
//! thread in strict lock-step with the engine: each engine tick hands it that tick's input and waits until the
//! program reaches its next frame wait, so the result is exactly as deterministic as a single thread.

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;

use native_engine::audio::{Block, Mixer, Pan};
use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{rgb12, Frame};
use native_engine::input::InputState;

use crate::assets::{Assets, H, W};
use crate::ik::Ik;

/// What the main program gets at each frame.
pub struct FrameIn {
    pub input: InputState,
}

/// A command for one of the four sound channels (the original's Paula channels 0 to 3).
#[derive(Debug, Clone, PartialEq)]
pub enum Snd {
    /// Start `block` now, then repeat `next` (as the channel's DMA did), at a period and volume.
    Start {
        ch: usize,
        block: Block,
        next: Block,
        period: u16,
        volume: u8,
    },
    Period(usize, u16),
    Volume(usize, u8),
    Next(usize, Block),
    Stop(usize),
}

/// The picture the main program hands back: the shown screen (4-bit colours) and the colours of every row.
#[derive(Clone)]
pub struct View {
    pub screen: Vec<u8>,
    pub rows: Vec<[u16; 16]>,
}

pub struct FrameOut {
    pub view: View,
    pub sounds: Vec<Snd>,
    pub state: Vec<(String, i64)>,
}

/// Unwinds the main program's thread when the engine drops the game.
pub struct Stop;

pub struct Native {
    assets: Arc<Assets>,
    seed: Option<u32>,
    tx: Option<SyncSender<FrameIn>>,
    rx: Option<Receiver<FrameOut>>,
    thread: Option<JoinHandle<()>>,
    last: Option<FrameOut>,
    done: bool,
}

/// Paula's channels 0 and 3 were on the left, 1 and 2 on the right.
const PANS: [Pan; 4] = [-0.6, 0.6, 0.6, -0.6];
const VOICE0: usize = native_engine::audio::MUSIC_VOICES;
/// The PAL Amiga's audio clock: a period of P plays 3546895 / P samples a second.
pub const PAULA: f32 = 3_546_895.0;

impl Native {
    pub fn new(assets: Assets, o: &Options) -> Native {
        Native {
            assets: Arc::new(assets),
            seed: o.seed,
            tx: None,
            rx: None,
            thread: None,
            last: None,
            done: false,
        }
    }

    fn start(&mut self) {
        let (tx_in, rx_in) = sync_channel::<FrameIn>(0);
        let (tx_out, rx_out): (Sender<FrameOut>, Receiver<FrameOut>) = channel();
        let a = self.assets.clone();
        let seed = self.seed;
        let h = std::thread::Builder::new()
            .name("g1907-main".into())
            .stack_size(16 << 20)
            .spawn(move || {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let mut g = Ik::new(a, seed, rx_in, tx_out);
                    g.run();
                }));
                if let Err(e) = r {
                    if e.downcast_ref::<Stop>().is_none() {
                        std::panic::resume_unwind(e);
                    }
                }
            })
            .expect("main program thread");
        self.tx = Some(tx_in);
        self.rx = Some(rx_out);
        self.thread = Some(h);
    }

    fn apply(s: &Snd, m: &mut Mixer) {
        let rate = |p: u16| PAULA / (p.max(113) as f32);
        match s {
            Snd::Start {
                ch,
                block,
                next,
                period,
                volume,
            } => m.start(
                VOICE0 + ch,
                block.clone(),
                next.clone(),
                rate(*period),
                *volume,
                PANS[ch & 3],
            ),
            Snd::Period(ch, p) => m.set_rate(VOICE0 + ch, rate(*p)),
            Snd::Volume(ch, v) => m.set_volume(VOICE0 + ch, *v),
            Snd::Next(ch, b) => m.set_next(VOICE0 + ch, b.clone()),
            Snd::Stop(ch) => m.stop(VOICE0 + ch),
        }
    }
}

impl Drop for Native {
    fn drop(&mut self) {
        self.tx = None;
        self.rx = None;
        if let Some(h) = self.thread.take() {
            let _ = h.join();
        }
    }
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        if self.done {
            return Flow::Quit;
        }
        if self.tx.is_none() {
            self.start();
        }
        let sent = self.tx.as_ref().is_some_and(|t| {
            t.send(FrameIn {
                input: cx.input.clone(),
            })
            .is_ok()
        });
        let out = if sent {
            self.rx.as_ref().and_then(|r| r.recv().ok())
        } else {
            None
        };
        let Some(out) = out else {
            self.done = true;
            return Flow::Quit;
        };
        for s in &out.sounds {
            Native::apply(s, cx.audio);
        }
        self.last = Some(out);
        Flow::Continue
    }

    fn render(&self, frame: &mut Frame) {
        let Some(out) = &self.last else {
            frame.image.fill(0);
            frame.direct = None;
            return;
        };
        render_view(&out.view, frame);
    }

    fn state(&self) -> Vec<(String, i64)> {
        self.last
            .as_ref()
            .map(|o| o.state.clone())
            .unwrap_or_default()
    }
}

/// The picture with the colours the original's display list gave each row (one colour a pixel: the sky and sea are
/// colour gradients down the screen).
pub fn render_view(v: &View, frame: &mut Frame) {
    frame.image.px.copy_from_slice(&v.screen);
    for i in 0..16 {
        frame.palette.colours[i] = rgb12(v.rows[H - 1][i]);
    }
    let mut d = Vec::with_capacity(W * H);
    for y in 0..H {
        let lut: Vec<_> = v.rows[y].iter().map(|c| rgb12(*c)).collect();
        d.extend(
            v.screen[y * W..(y + 1) * W]
                .iter()
                .map(|p| lut[(*p & 15) as usize]),
        );
    }
    frame.direct = Some(d);
}
