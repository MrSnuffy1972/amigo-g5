//! The bridge between the engine's tick-at-a-time `Game` and Jail Break's main program, which (like the original) is
//! written as ordinary nested code that waits for the next frame wherever the original did. The main program runs on
//! its own thread in strict lock-step with the engine: each engine tick hands it that tick's input and waits until the
//! program reaches its next frame wait, so the result is exactly as deterministic as a single thread. The thread never
//! runs on its own.

use std::sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender};
use std::sync::Arc;
use std::thread::JoinHandle;

use native_engine::audio::{Block, Mixer, Pan};
use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{rgb12, Frame};
use native_engine::input::InputState;

use crate::assets::{Assets, H, W};
use crate::jail::{Jail, View};

/// What the main program gets at each frame.
pub struct FrameIn {
    pub input: InputState,
    /// The saved custom levels (bricks, backgrounds), given once with the first frame.
    pub saved: Option<(Vec<u8>, Vec<u8>)>,
}

/// A sound command for the mixer (channel = the original's sound channel 0 to 3).
#[derive(Debug, Clone, PartialEq)]
pub enum Snd {
    Play { ch: usize, sfx: usize },
    Stop(usize),
}

/// What the main program hands back when it reaches a frame wait.
pub struct FrameOut {
    pub view: View,
    pub sounds: Vec<Snd>,
    /// Custom levels to save (the editor's save button).
    pub save: Option<(Vec<u8>, Vec<u8>)>,
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
/// Mixer voices 0 to 3 belong to the engine's tracker player; this game's four channels are the effect voices after.
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
            .name("g336-main".into())
            .spawn(move || {
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    let mut j = Jail::new(a, seed, rx_in, tx_out);
                    j.run();
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

    fn apply(&self, s: &Snd, m: &mut Mixer) {
        match *s {
            Snd::Play { ch, sfx } => {
                let e = &self.assets.sfx[sfx];
                // The channel repeats the sample until the program turns it off (as the original's sound DMA did).
                let b = Block::whole(&e.sample);
                m.start(
                    VOICE0 + ch,
                    b.clone(),
                    b,
                    e.sample.rate,
                    e.volume,
                    PANS[ch & 3],
                );
            }
            Snd::Stop(ch) => m.stop(VOICE0 + ch),
        }
    }
}

impl Drop for Native {
    fn drop(&mut self) {
        // Closing the input channel makes the program's next frame wait unwind its thread.
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
            let get = |k: &str| cx.save.get(k).and_then(|v| v.as_str()).and_then(unhex);
            if let (Some(b), Some(k)) = (get("custom_bricks"), get("custom_backs")) {
                saved = Some((b, k));
            }
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
            self.apply(s, cx.audio);
        }
        if let Some((b, k)) = &out.save {
            let _ = cx
                .save
                .set("custom_bricks", serde_json::Value::String(hexs(b)));
            let _ = cx
                .save
                .set("custom_backs", serde_json::Value::String(hexs(k)));
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
        let Some(out) = &self.last else {
            frame.image.fill(0);
            frame.direct = None;
            return;
        };
        render_view(&out.view, &self.assets, frame);
    }

    fn state(&self) -> Vec<(String, i64)> {
        self.last
            .as_ref()
            .map(|o| o.state.clone())
            .unwrap_or_default()
    }
}

/// Where the display window's first line sits when the picture is fully shown, and how far a sprite's position is
/// from the picture's (the original placed sprites one pixel left of the playfield coordinate they were given).
pub const WINDOW_TOP: i32 = 0x24;
pub const SPRITE_DX: i32 = -1;

/// Draws a frame from the program's view: the shown picture slid down by the window wipe, then the sprites.
pub fn render_view(v: &View, a: &Assets, frame: &mut Frame) {
    for (i, c) in v.palette.iter().enumerate() {
        frame.palette.colours[i] = rgb12(*c);
    }
    let k = (v.window_top as i32 - WINDOW_TOP).clamp(0, H as i32) as usize;
    let img = &mut frame.image;
    for y in 0..H {
        let dst = &mut img.px[y * W..(y + 1) * W];
        if y < k {
            dst.fill(0);
        } else {
            dst.copy_from_slice(&v.screen[(y - k) * W..(y - k + 1) * W]);
        }
    }
    frame.direct = v.direct.as_ref().map(|d| {
        let bg = rgb12(v.palette[0]);
        let mut o = vec![bg; W * H];
        for y in k..H {
            o[y * W..(y + 1) * W].copy_from_slice(&d[(y - k) * W..(y - k + 1) * W]);
        }
        o
    });
    // Sprites: lower-numbered sprites in front; only inside the display window. The reference harness's stand-in OS
    // never turned sprites on (a real Workbench has them on, and the game needs them: its balls are sprites), so the
    // verification build (feature "ref-harness") leaves them out to compare like with like.
    if cfg!(feature = "ref-harness") {
        return;
    }
    for s in (0..8).rev() {
        let Some((line, hpos)) = v.sprites[s] else {
            continue;
        };
        let t = &a.tiles[(v.sprite_tiles[s] as usize).min(a.tiles.len() - 1)];
        for r in 0..16i32 {
            let sl = line as i32 + r;
            let y = sl - WINDOW_TOP;
            if sl < v.window_top as i32 || y < 0 || y >= H as i32 {
                continue;
            }
            for c in 0..16i32 {
                let p = t.at(c as usize, r as usize) & 15;
                let x = hpos as i32 - 0x80 + SPRITE_DX + c;
                if p != 0 && (0..W as i32).contains(&x) {
                    img.px[y as usize * W + x as usize] = 16 + p;
                    if let Some(d) = frame.direct.as_mut() {
                        d[y as usize * W + x as usize] = rgb12(v.palette[16 + p as usize]);
                    }
                }
            }
        }
    }
}
