//! The bridge between the engine's tick-at-a-time `Game` and E-Motion's main program, which (like the original, and
//! like game 336's) is ordinary nested code that waits for the next frame wherever the original did. The main program
//! runs on its own thread in strict lock-step with the engine: each engine tick hands it that tick's input and waits
//! until the program reaches its next frame wait, so the result is exactly as deterministic as one thread.

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;

use native_engine::audio::{Block, Mixer, Pan};
use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{Argb, Frame};
use native_engine::input::InputState;

use crate::assets::{Assets, H, W};
use crate::emo::Emo;

/// What the main program gets at each frame.
pub struct FrameIn {
    pub input: InputState,
    /// The saved high-score table (given once, with the first frame).
    pub saved: Option<Vec<u8>>,
}

/// A sound command for the mixer. Voices are the original's four sound channels (the effect voices after the
/// engine's music voices).
#[derive(Clone)]
pub enum Snd {
    Start {
        ch: usize,
        block: Block,
        next: Block,
        rate: f32,
        vol: u8,
    },
    Next {
        ch: usize,
        block: Block,
    },
    Rate {
        ch: usize,
        rate: f32,
    },
    Vol {
        ch: usize,
        vol: u8,
    },
    Stop(usize),
}

/// What the main program hands back when it reaches a frame wait.
pub struct FrameOut {
    pub picture: Arc<Vec<Argb>>,
    pub sounds: Vec<Snd>,
    pub save: Option<Vec<u8>>,
    pub quit: bool,
    pub state: Vec<(String, i64)>,
}

/// Unwinds the main program's thread when the engine drops the game (restart or exit).
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

/// The original's channels 0 and 3 were on the left, 1 and 2 on the right.
const PANS: [Pan; 4] = [-0.6, 0.6, 0.6, -0.6];
const VOICE0: usize = native_engine::audio::MUSIC_VOICES;

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
            .name("g608-main".into())
            .spawn(move || {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let mut e = Emo::new(a, seed, rx_in, tx_out);
                    e.run();
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
}

pub fn apply(s: &Snd, m: &mut Mixer) {
    match s {
        Snd::Start {
            ch,
            block,
            next,
            rate,
            vol,
        } => m.start(
            VOICE0 + ch,
            block.clone(),
            next.clone(),
            *rate,
            *vol,
            PANS[ch & 3],
        ),
        Snd::Next { ch, block } => m.set_next(VOICE0 + ch, block.clone()),
        Snd::Rate { ch, rate } => m.set_rate(VOICE0 + ch, *rate),
        Snd::Vol { ch, vol } => m.set_volume(VOICE0 + ch, *vol),
        Snd::Stop(ch) => m.stop(VOICE0 + ch),
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

fn hexs(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok())
        .collect()
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        if self.done {
            return Flow::Quit;
        }
        let mut saved = None;
        if self.tx.is_none() {
            self.start();
            saved = cx
                .save
                .get("high_scores")
                .and_then(|v| v.as_str())
                .and_then(unhex);
        }
        let sent = self.tx.as_ref().is_some_and(|t| {
            t.send(FrameIn {
                input: cx.input.clone(),
                saved,
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
            apply(s, cx.audio);
        }
        if let Some(b) = &out.save {
            let _ = cx
                .save
                .set("high_scores", serde_json::Value::String(hexs(b)));
            let _ = cx.save.flush();
        }
        let quit = out.quit;
        self.last = Some(out);
        if quit {
            self.done = true;
            Flow::Quit
        } else {
            Flow::Continue
        }
    }

    fn render(&self, frame: &mut Frame) {
        frame.image.fill(0);
        frame.direct = self
            .last
            .as_ref()
            .filter(|o| o.picture.len() == W * H)
            .map(|o| o.picture.as_ref().clone());
    }

    fn state(&self) -> Vec<(String, i64)> {
        self.last
            .as_ref()
            .map(|o| o.state.clone())
            .unwrap_or_default()
    }
}
