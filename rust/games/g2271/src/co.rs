//! Runs the main program on its own thread in strict lock-step with the engine's ticks (the pattern of game 336):
//! each tick hands over that tick's input and waits for the program's next frame wait, so routines keep the
//! original's shape (frame waits deep inside nested loops) and the result is as deterministic as one thread.

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{rgb12, Frame};
use native_engine::input::InputState;

use crate::assets::{Assets, H, W};
use crate::castle::Castle;

pub struct FrameIn {
    pub input: InputState,
}

pub struct FrameOut {
    /// The high-score table to keep in the save store (`scores`), when it changed.
    pub save: Option<serde_json::Value>,
    /// The sound's commands for the mixer in this vertical blank.
    pub mix: Vec<crate::sound::Mix>,
    pub screen: Vec<u8>,
    pub palette: [u16; 32],
    pub quit: bool,
    pub state: Vec<(String, i64)>,
}

/// Unwinds the program thread when the engine drops the game.
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

    fn start(&mut self, saved: Option<serde_json::Value>) {
        let (tx_in, rx_in) = sync_channel::<FrameIn>(0);
        let (tx_out, rx_out): (Sender<FrameOut>, Receiver<FrameOut>) = channel();
        let a = self.assets.clone();
        let seed = self.seed;
        let h = std::thread::Builder::new()
            .name("g2271-main".into())
            .spawn(move || {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let mut c = Castle::new(a, seed, rx_in, tx_out);
                    c.scores = crate::hall::scores_from(saved.as_ref());
                    c.run();
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
            self.start(cx.save.get("scores").cloned());
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
        if let Some(v) = &out.save {
            let _ = cx.save.set("scores", v.clone());
            let _ = cx.save.flush();
        }
        for m in &out.mix {
            crate::sound::apply(m, cx.audio);
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
        frame.direct = None;
        let Some(out) = &self.last else {
            frame.image.fill(0);
            return;
        };
        for (i, c) in out.palette.iter().enumerate() {
            frame.palette.colours[i] = rgb12(*c);
        }
        let n = (W * H).min(frame.image.px.len());
        frame.image.px[..n].copy_from_slice(&out.screen[..n]);
    }

    fn state(&self) -> Vec<(String, i64)> {
        self.last
            .as_ref()
            .map(|o| o.state.clone())
            .unwrap_or_default()
    }
}
