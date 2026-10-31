//! The software mixer: a fixed set of voices playing 8-bit sample data at any rate, volume and pan, mixed to 16-bit
//! stereo at the output rate. A voice plays a block of a sample, then its "next" block (set at any time) repeatedly,
//! which covers one-shot sounds (next = silence), plain loops and tracker-style "play the attack, then loop the rest".
//! The tracker music player (tracker.rs) drives its own voices through the same interface.

use std::sync::Arc;

use crate::tracker::Tracker;

/// Signed 8-bit mono sample data and its natural rate.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub data: Arc<Vec<i8>>,
    pub rate: f32,
}

/// A block of sample data: `len` bytes from `start` (a length of zero is silence).
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub data: Arc<Vec<i8>>,
    pub start: usize,
    pub len: usize,
}

impl Block {
    pub fn silence() -> Block {
        Block {
            data: Arc::new(Vec::new()),
            start: 0,
            len: 0,
        }
    }
    pub fn whole(s: &Sample) -> Block {
        Block {
            data: s.data.clone(),
            start: 0,
            len: s.data.len(),
        }
    }
}

/// Where a voice sits in the stereo field: -1 left, 0 centre, 1 right.
pub type Pan = f32;

#[derive(Debug, Clone)]
struct Voice {
    cur: Block,
    next: Block,
    /// Position in `cur`, in bytes (fractional).
    pos: f64,
    rate: f32,
    /// 0..=64.
    volume: u8,
    pan: Pan,
    on: bool,
}

impl Default for Voice {
    fn default() -> Self {
        Voice {
            cur: Block::silence(),
            next: Block::silence(),
            pos: 0.0,
            rate: 0.0,
            volume: 0,
            pan: 0.0,
            on: false,
        }
    }
}

/// Voices 0..MUSIC_VOICES belong to the music player; the game's sound effects use the rest.
pub const MUSIC_VOICES: usize = 4;

pub struct Mixer {
    voices: Vec<Voice>,
    pub out_rate: u32,
    music: Option<Tracker>,
    /// The game's tick rate: the music player advances one tick per `tick()`.
    pub tick_hz: u32,
}

impl Mixer {
    /// `effects` sound-effect voices beside the music player's.
    pub fn new(out_rate: u32, tick_hz: u32, effects: usize) -> Mixer {
        Mixer {
            voices: vec![Voice::default(); MUSIC_VOICES + effects],
            out_rate,
            music: None,
            tick_hz,
        }
    }

    pub fn voices(&self) -> usize {
        self.voices.len()
    }

    /// Starts `block` on voice `v` from its beginning, followed by `next` repeated.
    pub fn start(&mut self, v: usize, block: Block, next: Block, rate: f32, volume: u8, pan: Pan) {
        if let Some(x) = self.voices.get_mut(v) {
            *x = Voice {
                cur: block,
                next,
                pos: 0.0,
                rate,
                volume: volume.min(64),
                pan,
                on: true,
            };
        }
    }

    /// A one-shot sound effect on voice `v`: the whole sample once, then silence.
    pub fn play(&mut self, v: usize, s: &Sample, rate: f32, volume: u8, pan: Pan) {
        self.start(v, Block::whole(s), Block::silence(), rate, volume, pan);
    }

    /// What voice `v` plays after its current block (takes effect when the block ends).
    pub fn set_next(&mut self, v: usize, next: Block) {
        if let Some(x) = self.voices.get_mut(v) {
            x.next = next;
        }
    }

    pub fn set_rate(&mut self, v: usize, rate: f32) {
        if let Some(x) = self.voices.get_mut(v) {
            x.rate = rate;
        }
    }

    pub fn set_volume(&mut self, v: usize, volume: u8) {
        if let Some(x) = self.voices.get_mut(v) {
            x.volume = volume.min(64);
        }
    }

    pub fn set_pan(&mut self, v: usize, pan: Pan) {
        if let Some(x) = self.voices.get_mut(v) {
            x.pan = pan;
        }
    }

    pub fn stop(&mut self, v: usize) {
        if let Some(x) = self.voices.get_mut(v) {
            x.on = false;
        }
    }

    /// True while voice `v` plays something audible (its current block is not silence).
    pub fn is_playing(&self, v: usize) -> bool {
        self.voices.get(v).is_some_and(|x| x.on && x.cur.len > 0)
    }

    /// Starts the tracker music (replacing any) and plays it from the start.
    pub fn music_play(&mut self, t: Tracker) {
        self.music = Some(t);
    }

    /// Stops the music and silences its voices.
    pub fn music_stop(&mut self) {
        self.music = None;
        for v in 0..MUSIC_VOICES {
            self.voices[v].volume = 0;
            self.voices[v].on = false;
        }
    }

    pub fn music_playing(&self) -> bool {
        self.music.is_some()
    }

    /// The music player's tick (it sets its voices). The game calls this at the point of its frame where the original
    /// advanced its music (once a frame for a 50 Hz game); the platform layer never does.
    /// Returns true when the music read a new row this tick (see `Tracker::tick`).
    pub fn tick(&mut self) -> bool {
        match self.music.take() {
            Some(mut t) => {
                let row = t.tick(self);
                self.music = Some(t);
                row
            }
            None => false,
        }
    }

    /// Mixes `frames` stereo frames onto the end of `out` (interleaved left, right).
    pub fn render(&mut self, frames: usize, out: &mut Vec<i16>) {
        let base = out.len();
        out.resize(base + frames * 2, 0);
        let mut acc = vec![0i32; frames * 2];
        let out_rate = self.out_rate as f64;
        for v in self.voices.iter_mut().filter(|v| v.on) {
            let step = v.rate as f64 / out_rate;
            let (gl, gr) = gains(v.pan);
            for f in 0..frames {
                if v.cur.len == 0 || step <= 0.0 {
                    // silence: a zero-length block or a stopped clock holds no sound; take the next block
                    if v.cur.len == 0 && v.next.len > 0 {
                        v.cur = v.next.clone();
                        v.pos = 0.0;
                    } else {
                        continue;
                    }
                }
                let i = v.cur.start + v.pos as usize;
                let s = v.cur.data.get(i).copied().unwrap_or(0) as i32 * v.volume as i32 * 2;
                acc[f * 2] += (s as f32 * gl) as i32;
                acc[f * 2 + 1] += (s as f32 * gr) as i32;
                v.pos += step;
                while v.pos >= v.cur.len as f64 && v.cur.len > 0 {
                    v.pos -= v.cur.len as f64;
                    v.cur = v.next.clone();
                    if v.cur.len == 0 {
                        v.pos = 0.0;
                    }
                }
            }
        }
        for (o, a) in out[base..].iter_mut().zip(acc) {
            *o = a.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
    }
}

/// Full gain on one side at the ends, half on each in the centre.
fn gains(pan: Pan) -> (f32, f32) {
    let p = pan.clamp(-1.0, 1.0);
    ((1.0 - p) * 0.5, (1.0 + p) * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone() -> Sample {
        Sample {
            data: Arc::new(vec![100, -100, 100, -100]),
            rate: 4.0,
        }
    }

    #[test]
    fn one_shot_plays_once_then_silence() {
        let mut m = Mixer::new(4, 50, 1);
        m.play(4, &tone(), 4.0, 64, -1.0);
        let mut out = vec![];
        m.render(6, &mut out);
        // left only, one sample a frame: 4 samples then silence
        let left: Vec<i16> = out.iter().step_by(2).copied().collect();
        assert_eq!(left, vec![12800, -12800, 12800, -12800, 0, 0]);
        assert!(out.iter().skip(1).step_by(2).all(|&r| r == 0));
        assert!(!m.is_playing(4));
    }

    #[test]
    fn next_block_loops_and_rate_scales() {
        let mut m = Mixer::new(4, 50, 1);
        let s = tone();
        let next = Block {
            data: s.data.clone(),
            start: 2,
            len: 1,
        };
        m.start(4, Block::whole(&s), next, 8.0, 32, 1.0);
        let mut out = vec![];
        m.render(4, &mut out);
        let right: Vec<i16> = out.iter().skip(1).step_by(2).copied().collect();
        // two samples a frame: 100, 100 (skips the -100s), then the loop of the single 100
        assert_eq!(right, vec![6400, 6400, 6400, 6400]);
        assert!(m.is_playing(4));
        m.stop(4);
        assert!(!m.is_playing(4));
    }
}
