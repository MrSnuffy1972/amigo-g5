//! The game's music and sound: the Beathoven 1.09 sequencer (Th. Lopatic) the original plays from its vertical
//! blank, and the four sound channels it and the sound effects share. Ported from the recovered routines
//! `start_sequencer_channels`, `reset_sequencer_channel`, `music_tick` and `sequencer_channel_tick` (and the boot
//! program's older copy, `boot_*`, which has a fixed tempo of 2 and no second transpose).
//!
//! A channel is described by what the game needs from it: a sample block that starts when the channel is switched
//! on, the block that repeats after it (what the game last asked for), a pitch and a volume. Switching a playing
//! channel on again does nothing, as on the original; switching it off silences it.

use std::sync::Arc;

use native_engine::audio::{Block, Mixer, Pan};
use native_engine::pack::Pack;
use serde_json::Value;

/// The PAL clock the sample periods divide.
pub const PAL_CLOCK: f32 = 3_546_895.0;

#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    End,
    Instrument(usize),
    Portamento(i32),
    Legato(i32),
    Rest(u32),
    Arpeggio(u32, Vec<i32>),
    Transpose(i32),
    Note(i32, u32),
}

pub struct Instrument {
    pub vol: u8,
    pub len: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub one_shot: bool,
    pub data: Arc<Vec<i8>>,
}

/// The tunes, sequence lists, patterns and instruments of one copy of the player.
pub struct Song {
    /// Per tune and channel: the first sequence list and the list it loops to.
    pub tunes: Vec<[(usize, usize); 4]>,
    pub seqs: Vec<Vec<(usize, i32)>>,
    pub patterns: Vec<Vec<Cmd>>,
    pub instr: Vec<Instrument>,
    pub periods: Vec<u16>,
    /// The boot program's copy: tempo 2, no `$85` transpose.
    pub boot: bool,
}

impl Song {
    pub fn load(p: &Pack, v: &Value, boot: bool) -> Result<Song, String> {
        let obj = |k: &str| {
            v.get(k)
                .and_then(Value::as_object)
                .ok_or(format!("music {k}"))
        };
        let mut instr = Vec::new();
        let mut ins_id = std::collections::HashMap::new();
        for (k, i) in obj("instr")? {
            let n = i.get("sample").and_then(Value::as_str).ok_or("sample")?;
            let g = |f: &str| i.get(f).and_then(Value::as_u64).unwrap_or(0) as u32;
            ins_id.insert(k.clone(), instr.len());
            instr.push(Instrument {
                vol: g("vol").min(64) as u8,
                len: g("len"),
                loop_start: g("loop_start"),
                loop_end: g("loop_end"),
                one_shot: i.get("one_shot").and_then(Value::as_bool).unwrap_or(false),
                data: Arc::new(p.bytes(n)?.iter().map(|&b| b as i8).collect()),
            });
        }
        let mut pat_id = std::collections::HashMap::new();
        let mut patterns = Vec::new();
        for (k, ps) in obj("patterns")? {
            pat_id.insert(k.clone(), patterns.len());
            let mut out = Vec::new();
            for c in ps.as_array().ok_or("pattern")? {
                let c = c.as_array().ok_or("command")?;
                let n = |i: usize| c.get(i).and_then(Value::as_i64).unwrap_or(0);
                let op = n(0);
                out.push(match op {
                    0 => Cmd::End,
                    0x80 => Cmd::Instrument(
                        c.get(1)
                            .and_then(Value::as_str)
                            .and_then(|s| ins_id.get(s).copied())
                            .ok_or("instrument ref")?,
                    ),
                    0x81 => Cmd::Portamento(n(1) as i32),
                    0x82 => Cmd::Legato(n(1) as i32),
                    0x83 => Cmd::Rest(n(1) as u32 & 0xFFFF),
                    0x84 => Cmd::Arpeggio(
                        n(1) as u32,
                        (0..n(2) as usize).map(|i| n(3 + i) as i32).collect(),
                    ),
                    0x85 => Cmd::Transpose(n(1) as i32),
                    _ => Cmd::Note(op as i32, n(1) as u32),
                });
            }
            patterns.push(out);
        }
        let mut seq_id = std::collections::HashMap::new();
        let mut raw = Vec::new();
        for (k, s) in obj("seqs")? {
            seq_id.insert(k.clone(), raw.len());
            raw.push(s.clone());
        }
        let mut seqs = Vec::new();
        for s in raw {
            let mut out = Vec::new();
            for e in s.as_array().ok_or("seq")? {
                let p = e
                    .get(0)
                    .and_then(Value::as_str)
                    .and_then(|x| pat_id.get(x).copied())
                    .ok_or("pattern ref")?;
                out.push((p, e.get(1).and_then(Value::as_i64).unwrap_or(0) as i32));
            }
            seqs.push(out);
        }
        let mut tunes = Vec::new();
        for t in v.get("tunes").and_then(Value::as_array).ok_or("tunes")? {
            let mut ch = [(0, 0); 4];
            for (c, e) in t.as_array().ok_or("tune")?.iter().enumerate().take(4) {
                let r = |k: &str| {
                    e.get(k)
                        .and_then(Value::as_str)
                        .and_then(|x| seq_id.get(x).copied())
                        .ok_or("seq ref")
                };
                ch[c] = (r("start")?, r("loop")?);
            }
            tunes.push(ch);
        }
        let periods = v
            .get("periods")
            .and_then(Value::as_array)
            .ok_or("periods")?
            .iter()
            .map(|x| x.as_u64().unwrap_or(0) as u16)
            .collect();
        Ok(Song {
            tunes,
            seqs,
            patterns,
            instr,
            periods,
            boot,
        })
    }
}

/// A sound command for the mixer, collected during a frame and applied at the end of the engine's tick.
#[derive(Clone, Debug)]
pub enum Snd {
    Start {
        ch: usize,
        block: Block,
        rate: f32,
        vol: u8,
    },
    Next(usize, Block),
    Rate(usize, f32),
    Volume(usize, u8),
    Stop(usize),
}

/// The original's channels 0 and 3 were on the left, 1 and 2 on the right (softened for headphones).
pub const PANS: [Pan; 4] = [-0.6, 0.6, 0.6, -0.6];

pub fn apply(s: &Snd, m: &mut Mixer) {
    match s {
        Snd::Start {
            ch,
            block,
            rate,
            vol,
        } => m.start(
            *ch,
            block.clone(),
            block.clone(),
            *rate,
            *vol,
            PANS[*ch & 3],
        ),
        Snd::Next(ch, b) => m.set_next(*ch, b.clone()),
        Snd::Rate(ch, r) => m.set_rate(*ch, *r),
        Snd::Volume(ch, v) => m.set_volume(*ch, *v),
        Snd::Stop(ch) => m.stop(*ch),
    }
}

/// One sound channel as the game drives it.
#[derive(Clone)]
pub struct Channel {
    pub block: Block,
    pub on: bool,
    pub rate: f32,
    pub vol: u8,
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            block: Block::silence(),
            on: false,
            rate: 0.0,
            vol: 0,
        }
    }
}

/// The four channels and the commands made this frame.
#[derive(Default)]
pub struct Sound {
    pub ch: [Channel; 4],
    pub out: Vec<Snd>,
}

impl Sound {
    /// The block the channel plays next (and repeats).
    pub fn set_block(&mut self, c: usize, b: Block) {
        self.ch[c].block = b.clone();
        if self.ch[c].on {
            self.out.push(Snd::Next(c, b));
        }
    }
    pub fn on(&mut self, c: usize) {
        if !self.ch[c].on {
            self.ch[c].on = true;
            let x = &self.ch[c];
            self.out.push(Snd::Start {
                ch: c,
                block: x.block.clone(),
                rate: x.rate,
                vol: x.vol,
            });
        }
    }
    pub fn off(&mut self, c: usize) {
        if self.ch[c].on {
            self.ch[c].on = false;
            self.out.push(Snd::Stop(c));
        }
    }
    pub fn period(&mut self, c: usize, p: u16) {
        let r = if p == 0 { 0.0 } else { PAL_CLOCK / p as f32 };
        self.ch[c].rate = r;
        self.out.push(Snd::Rate(c, r));
    }
    pub fn volume(&mut self, c: usize, v: u16) {
        let v = v.min(64) as u8;
        self.ch[c].vol = v;
        self.out.push(Snd::Volume(c, v));
    }
}

/// A block of an instrument's sample from byte `start`, `words` words long (clamped to the data).
fn block(i: &Instrument, start: u32, words: u32) -> Block {
    let s = (start as usize).min(i.data.len());
    let l = (2 * words as usize).min(i.data.len() - s);
    Block {
        data: i.data.clone(),
        start: s,
        len: l,
    }
}

/// One sequencer channel (the original's 52-byte channel record).
#[derive(Clone, Default, Debug)]
pub struct SeqChannel {
    pub flags: u8,
    pub count: u16,
    /// Current sequence list and entry, and the list it loops to.
    pub seq: usize,
    pub seq_pos: usize,
    pub loop_seq: usize,
    pub pat: usize,
    pub pos: usize,
    pub transpose: i32,
    pub target: u16,
    pub cur: u16,
    pub porta: u16,
    pub len_half: u32,
    pub instr: Option<usize>,
    pub loop_start: u32,
    pub loop_end: u32,
    pub total: u32,
    pub arp: Vec<i32>,
    pub arp_i: usize,
    pub transpose2: i32,
}

/// The player: which tune plays, its tempo, whether channel four is left to the sound effects, and the channels.
#[derive(Default)]
pub struct Player {
    pub tune: usize,
    pub three: bool,
    pub tempo: u16,
    pub chans: [SeqChannel; 4],
}

impl Player {
    /// `start_sequencer_channels` / `boot_start_sequencer_channels`, then the channels are switched on (with
    /// `boot_start_music`).
    pub fn start(&mut self, s: &Song, snd: &mut Sound, tune: usize, tempo: u16, three: bool) {
        self.tune = tune;
        self.three = three;
        self.tempo = tempo;
        let t = s.tunes[tune - 1];
        for (c, tc) in t.iter().enumerate() {
            let ch = &mut self.chans[c];
            ch.seq = tc.0;
            ch.loop_seq = tc.1;
            self.reset_channel(s, snd, c);
        }
        for c in 0..4 {
            snd.on(c);
        }
    }

    /// `reset_sequencer_channel` / `boot_reset_sequencer_channel`.
    fn reset_channel(&mut self, s: &Song, snd: &mut Sound, c: usize) {
        let ch = &mut self.chans[c];
        if !s.boot {
            ch.transpose2 = 0;
        }
        ch.flags = 0;
        ch.count = 1;
        snd.volume(c, 0);
        ch.seq_pos = 0;
        let (p, tr) = s.seqs[ch.seq].first().copied().unwrap_or((0, 0));
        ch.pat = p;
        ch.pos = 0;
        ch.transpose = tr;
    }

    /// `stop_music`: no tune, all four channels off.
    pub fn stop(&mut self, snd: &mut Sound) {
        self.tune = 0;
        for c in 0..4 {
            snd.off(c);
        }
    }

    /// `music_tick` (from the vertical blank).
    pub fn tick(&mut self, s: &Song, snd: &mut Sound) {
        if self.tune == 0 {
            return;
        }
        for c in 0..3 {
            self.channel_tick(s, snd, c);
        }
        if !self.three {
            self.channel_tick(s, snd, 3);
        }
    }

    /// `boot_music_tick`: all four channels.
    pub fn boot_tick(&mut self, s: &Song, snd: &mut Sound) {
        for c in 0..4 {
            self.channel_tick(s, snd, c);
        }
    }

    fn period(s: &Song, idx: i32) -> u16 {
        s.periods.get(idx.max(0) as usize).copied().unwrap_or(0)
    }

    /// `sequencer_channel_tick` / `boot_sequencer_channel_tick`.
    pub fn channel_tick(&mut self, s: &Song, snd: &mut Sound, c: usize) {
        let tempo = if s.boot { 2 } else { self.tempo } as u32;
        let ch = &mut self.chans[c];
        let mut restart = false;
        // The block that repeats: the instrument's loop, or silence after a one-shot sound or a rest.
        if ch.flags & 0x0C == 0 {
            if let Some(i) = ch.instr {
                let ins = &s.instr[i];
                snd.set_block(
                    c,
                    block(
                        ins,
                        ch.loop_start,
                        ch.loop_end.saturating_sub(ch.loop_start) / 2,
                    ),
                );
            }
        } else {
            snd.set_block(c, Block::silence());
        }
        ch.count = ch.count.wrapping_sub(1);
        if ch.count == 0 {
            let mut new_block = true;
            loop {
                ch.flags &= !8;
                let cmd = s.patterns[ch.pat].get(ch.pos).cloned().unwrap_or(Cmd::End);
                if cmd == Cmd::End {
                    ch.seq_pos += 1;
                    if ch.seq_pos >= s.seqs[ch.seq].len() {
                        ch.seq = ch.loop_seq;
                        ch.seq_pos = 0;
                    }
                    let (p, tr) = s.seqs[ch.seq].get(ch.seq_pos).copied().unwrap_or((0, 0));
                    ch.pat = p;
                    ch.pos = 0;
                    ch.transpose = tr;
                    continue;
                }
                if !matches!(cmd, Cmd::Rest(_)) {
                    ch.flags &= !0x10;
                    snd.off(c);
                    restart = true;
                }
                match cmd {
                    Cmd::Instrument(i) => {
                        let ins = &s.instr[i];
                        ch.instr = Some(i);
                        if ins.one_shot {
                            ch.flags |= 4;
                        } else {
                            ch.flags &= !4;
                        }
                        snd.volume(c, ins.vol as u16);
                        let l = if ch.flags & 4 != 0 {
                            ins.len
                        } else {
                            ins.loop_start
                        } / 2;
                        ch.len_half = l;
                        snd.set_block(c, block(ins, 0, l));
                        ch.loop_start = ins.loop_start;
                        ch.loop_end = ins.loop_end;
                        ch.total = ins.len;
                        ch.pos += 1;
                    }
                    Cmd::Portamento(v) => {
                        ch.porta = v as u16;
                        if v != 0 {
                            ch.flags |= 1;
                            snd.on(c);
                        } else {
                            ch.flags &= !1;
                        }
                        ch.pos += 1;
                    }
                    Cmd::Legato(v) => {
                        if v != 0 {
                            ch.flags |= 2;
                            snd.on(c);
                        } else {
                            ch.flags &= !2;
                        }
                        ch.pos += 1;
                    }
                    Cmd::Transpose(v) if !s.boot => {
                        ch.transpose2 = v;
                        ch.pos += 1;
                    }
                    Cmd::Transpose(_) => {
                        ch.pos += 1;
                    }
                    Cmd::Rest(len) => {
                        ch.flags |= 8;
                        if ch.flags & 4 == 0 {
                            snd.off(c);
                            restart = true;
                            if let Some(i) = ch.instr {
                                let ins = &s.instr[i];
                                snd.set_block(
                                    c,
                                    block(
                                        ins,
                                        ch.loop_end,
                                        ch.total.saturating_sub(ch.loop_end) / 2,
                                    ),
                                );
                            }
                        }
                        ch.count = (len * tempo) as u16;
                        ch.pos += 1;
                        new_block = false;
                        break;
                    }
                    Cmd::Arpeggio(len, notes) => {
                        ch.flags |= 0x10;
                        ch.count = (len * tempo) as u16;
                        ch.arp = notes;
                        ch.arp_i = 0;
                        ch.pos += 1;
                        break;
                    }
                    Cmd::Note(n, len) => {
                        let tr2 = if s.boot { 0 } else { ch.transpose2 };
                        ch.target = Self::period(s, n - 1 + ch.transpose + tr2);
                        ch.count = (len * tempo) as u16;
                        ch.pos += 1;
                        break;
                    }
                    Cmd::End => {}
                }
            }
            if new_block && ch.flags & 3 == 0 {
                if let Some(i) = ch.instr {
                    snd.set_block(c, block(&s.instr[i], 0, ch.len_half));
                }
            }
        }
        // Arpeggio, portamento or the plain note's pitch.
        let mut store = true;
        if ch.flags & 0x10 != 0 {
            let tr2 = if s.boot { 0 } else { ch.transpose2 };
            if let Some(&n) = ch.arp.get(ch.arp_i) {
                ch.target = Self::period(s, n - 1 + ch.transpose + tr2);
            }
            ch.arp_i += 1;
            if ch.arp_i >= ch.arp.len() {
                ch.arp_i = 0;
            }
            ch.cur = ch.target;
        } else if ch.flags & 1 != 0 {
            let cur = ch.cur;
            let tgt = ch.target;
            if cur == tgt {
                store = false;
            } else if cur > tgt {
                let d = (cur as i16).wrapping_sub(ch.porta as i16);
                ch.cur = if d < tgt as i16 { tgt } else { d as u16 };
            } else {
                let d = (cur as i16).wrapping_add(ch.porta as i16);
                ch.cur = if d < tgt as i16 { d as u16 } else { tgt };
            }
        } else {
            ch.cur = ch.target;
        }
        let _ = store;
        let cur = ch.cur;
        let flags = ch.flags;
        snd.period(c, cur);
        if restart || flags & 3 == 0 {
            snd.on(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn song() -> Song {
        Song {
            tunes: vec![[(0, 0); 4]],
            seqs: vec![vec![(0, 0)]],
            patterns: vec![vec![
                Cmd::Instrument(0),
                Cmd::Note(2, 1),
                Cmd::Rest(1),
                Cmd::End,
            ]],
            instr: vec![Instrument {
                vol: 40,
                len: 8,
                loop_start: 4,
                loop_end: 8,
                one_shot: false,
                data: Arc::new(vec![1; 8]),
            }],
            periods: vec![400, 300, 200],
            boot: false,
        }
    }

    #[test]
    fn a_note_starts_its_sample_at_its_pitch() {
        let s = song();
        let mut snd = Sound::default();
        let mut p = Player::default();
        p.start(&s, &mut snd, 1, 1, false);
        snd.out.clear();
        p.tick(&s, &mut snd);
        assert!(snd.ch[0].on);
        assert_eq!(snd.ch[0].vol, 40);
        assert!((snd.ch[0].rate - PAL_CLOCK / 300.0).abs() < 0.01);
        assert!(snd
            .out
            .iter()
            .any(|c| matches!(c, Snd::Start { ch: 0, .. })));
    }

    #[test]
    fn a_rest_plays_the_release_then_silence() {
        let s = song();
        let mut snd = Sound::default();
        let mut p = Player::default();
        p.start(&s, &mut snd, 1, 1, false);
        p.tick(&s, &mut snd);
        p.tick(&s, &mut snd);
        assert_eq!(p.chans[0].flags & 8, 8);
        // The release: the bytes after the loop end, restarted.
        assert_eq!((snd.ch[0].block.start, snd.ch[0].block.len), (8, 0));
        assert!(snd.ch[0].on);
    }
}
