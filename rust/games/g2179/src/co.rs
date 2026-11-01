//! The bridge between the engine's tick-at-a-time `Game` and the game's main program, which (like the original) is
//! written as ordinary nested code that waits for the beam wherever the original did. The program runs on its own
//! thread in strict lock-step with the engine (the pattern of game 336): each engine tick hands it that tick's input
//! and waits until the program reaches the end of its next frame, so the result is as deterministic as one thread.

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::Frame;
use native_engine::input::InputState;
use serde_json::{json, Value};

use crate::assets::{Assets, HsEntry, H, W};
use crate::giana::{Giana, View};
use crate::music::{apply, Snd};

/// What the main program gets each frame.
pub struct FrameIn {
    pub input: InputState,
    /// The saved all-time high scores, given once with the first frame.
    pub saved_hs: Option<Vec<HsEntry>>,
}

/// What the main program hands back at the end of a frame.
pub struct FrameOut {
    pub view: View,
    pub sounds: Vec<Snd>,
    /// The all-time high scores to save (the mouse button on the high-score screen).
    pub save: Option<Vec<HsEntry>>,
    pub state: Vec<(String, i64)>,
}

/// Unwinds the main program's thread when the engine drops the game.
pub struct Stop;

pub struct Native {
    assets: Arc<Assets>,
    tx: Option<SyncSender<FrameIn>>,
    rx: Option<Receiver<FrameOut>>,
    thread: Option<JoinHandle<()>>,
    last: Option<FrameOut>,
    done: bool,
}

impl Native {
    pub fn new(assets: Assets, _o: &Options) -> Native {
        Native {
            assets: Arc::new(assets),
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
        let h = std::thread::Builder::new()
            .name("g2179-main".into())
            .stack_size(8 << 20)
            .spawn(move || {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let mut g = Giana::new(a, rx_in, tx_out);
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

pub fn hs_to_json(t: &[HsEntry]) -> Value {
    Value::Array(
        t.iter()
            .map(|e| {
                json!({
                    "name": e.name.iter().map(|&c| c as char).collect::<String>(),
                    "score": e.score,
                    "stage": e.stage,
                })
            })
            .collect(),
    )
}

pub fn hs_from_json(v: &Value) -> Option<Vec<HsEntry>> {
    let a = v.as_array()?;
    if a.len() != 5 {
        return None;
    }
    a.iter()
        .map(|e| {
            let mut name = [b' '; 10];
            for (i, c) in e.get("name")?.as_str()?.chars().take(10).enumerate() {
                name[i] = c as u32 as u8;
            }
            Some(HsEntry {
                name,
                score: e.get("score")?.as_u64()? as u32,
                stage: e.get("stage")?.as_u64()? as u32,
            })
        })
        .collect()
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        if self.done {
            return Flow::Quit;
        }
        let mut saved_hs = None;
        if self.tx.is_none() {
            self.start();
            saved_hs = cx.save.get("highscores").and_then(hs_from_json);
        }
        let sent = self.tx.as_ref().is_some_and(|t| {
            t.send(FrameIn {
                input: cx.input.clone(),
                saved_hs,
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
        if let Some(t) = &out.save {
            let _ = cx.save.set("highscores", hs_to_json(t));
            let _ = cx.save.flush();
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
        let v = &out.view;
        frame.palette.colours[..64].copy_from_slice(&v.pal);
        if frame.image.px.len() == W * H {
            frame.image.px.copy_from_slice(&v.px);
        }
        frame.direct = v.direct.clone();
    }

    fn state(&self) -> Vec<(String, i64)> {
        self.last
            .as_ref()
            .map(|o| o.state.clone())
            .unwrap_or_default()
    }
}
