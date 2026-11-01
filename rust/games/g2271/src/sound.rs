//! The sound (part 3b), rewritten from the recovered routines. The game has its own four-channel replayer for its
//! `.mus` tunes and the title module (+ED3A parse_sample_header, +EEB8 process_dma_interrupt_state, +EFDE
//! sound_channel_step, +F1C6/+F1F8 the per-tick effects, +F354, +EE42 mute_audio_channels, +EE8C
//! negate_and_store_word, +EEA6 check_flag_and_call), run from the CIA-A timer interrupt (+C9FA: 14,640 ticks of
//! the 709,379 Hz clock, about 48.45 a second), and sampled effects (+F364 parse_8svx_header, +F44C
//! process_audio_trigger, +AC24 check_and_dispatch) that borrow a music channel until the sample ends (the audio
//! interrupt, +F13E/+F180). Paula's registers become commands to the engine's mixer, voices 0 to 3.
//!
//! Kept quirks: the instrument number of a note is looked up in the tune's instrument list with no end; the
//! portamento commands slide a per-channel word that no note ever sets (it starts at zero, so the first slide up
//! goes to the top note); the effect channel's commands are compared without masking the instrument bits; a tune
//! that mutes while an effect plays leaves that channel borrowed until the next tune is set up; the effect rate is
//! computed against the NTSC clock (3,579,545 / rate) and played on the PAL one.

use std::sync::Arc;

use native_engine::audio::{Block, Mixer, Pan};

use crate::assets::Assets;

/// The PAL clock the sample periods divide.
pub const PAL_CLOCK: f64 = 3_546_895.0;
/// CIA-A's clock and the replayer's timer period (+C9FA): replayer steps per second = 709,379 / 14,640.
const CIA_CLOCK: u64 = 709_379;
const CIA_PERIOD: u64 = 14_640;
/// Vertical blanks a second (the game's tick).
const VBL_HZ: u64 = 50;

/// The original's channels 0 and 3 were on the left, 1 and 2 on the right (softened for headphones).
pub const PANS: [Pan; 4] = [-0.6, 0.6, 0.6, -0.6];

/// What the sound did to Paula this frame, for the engine's mixer.
#[derive(Clone)]
pub enum Mix {
    Start {
        ch: usize,
        block: Block,
        next: Block,
        rate: f32,
        vol: u8,
    },
    Rate(usize, f32),
    Volume(usize, u8),
    Stop(usize),
}

pub fn apply(m: &Mix, mx: &mut Mixer) {
    match m {
        Mix::Start {
            ch,
            block,
            next,
            rate,
            vol,
        } => mx.start(*ch, block.clone(), next.clone(), *rate, *vol, PANS[*ch & 3]),
        Mix::Rate(c, r) => mx.set_rate(*c, *r),
        Mix::Volume(c, v) => mx.set_volume(*c, *v),
        Mix::Stop(c) => mx.stop(*c),
    }
}

fn rate_of(period: u16) -> f32 {
    if period == 0 {
        0.0
    } else {
        (PAL_CLOCK / period as f64) as f32
    }
}

fn ub(d: &[i8], o: usize) -> u8 {
    d.get(o).map(|v| *v as u8).unwrap_or(0)
}
fn uw(d: &[i8], o: usize) -> u16 {
    ((ub(d, o) as u16) << 8) | ub(d, o + 1) as u16
}
fn sw(d: &[i8], o: usize) -> i16 {
    uw(d, o) as i16
}
fn ul(d: &[i8], o: usize) -> u32 {
    ((uw(d, o) as u32) << 16) | uw(d, o + 2) as u32
}

/// `len` bytes of `d` from `start`, clamped to the data.
fn block(d: &Arc<Vec<i8>>, start: usize, len: usize) -> Block {
    let s = start.min(d.len());
    Block {
        data: d.clone(),
        start: s,
        len: len.min(d.len() - s),
    }
}

/// A tune as +ED3A parse_sample_header sets it up: the file ("SONG" with its samples in the instrument bank, or
/// "MODU" with its own after "SAMP"), the sample memory, where each instrument's sample starts (-$22BE), and the
/// position table (-$21E8), pattern table (-$21E4) and pattern data (-$21EC).
pub struct Song {
    file: Arc<Vec<i8>>,
    smp: Arc<Vec<i8>>,
    ins_ptr: [usize; 32],
    ins_tab: usize,
    npos: u16,
    data: usize,
    pats: usize,
    poss: usize,
}

const SONG: u32 = 0x534F_4E47;
const MODU: u32 = 0x4D4F_4455;
const SAMP: u32 = 0x5341_4D50;

impl Song {
    /// +ED3A: None where the original returned -1 (not a tune). The first long of each instrument's sample is
    /// cleared in the sample memory, as the original did (in the shared bank for a "SONG").
    pub fn parse(file: &Arc<Vec<i8>>, bank: &mut Vec<i8>) -> Option<Song> {
        let f = file.as_slice();
        let a2 = 4usize;
        let (mut smp, mut a1) = match ul(f, 0) {
            SONG => (std::mem::take(bank), 0usize),
            MODU => {
                let a1 = (a2 as isize + 4 + sw(f, a2 + 16) as isize + sw(f, a2 + 24) as isize)
                    .max(0) as usize;
                if ul(f, a1) != SAMP {
                    return None;
                }
                (f.to_vec(), a1 + 4)
            }
            _ => return None,
        };
        let mut ins_ptr = [0usize; 32];
        let mut a3 = a2 + 0x1A;
        let n = uw(f, a2 + 18) as usize;
        for _ in 0..n {
            let k = (ub(f, a3) as i8 as i16 as usize) & 31;
            ins_ptr[k] = a1;
            for b in smp.iter_mut().skip(a1).take(4) {
                *b = 0;
            }
            a1 += uw(f, a3 + 2) as usize * 2;
            a3 += 8;
        }
        let is_song = ul(f, 0) == SONG;
        let smp = if is_song {
            *bank = smp;
            Arc::new(bank.clone())
        } else {
            Arc::new(smp)
        };
        let d16 = sw(f, a2 + 16) as isize;
        let base = a2 as isize + d16;
        let pats = base - 2 * sw(f, a2 + 22) as isize;
        let poss = pats - 2 * sw(f, a2 + 20) as isize;
        Some(Song {
            file: file.clone(),
            smp,
            ins_ptr,
            ins_tab: a2 + 0x1A,
            npos: uw(f, a2 + 20),
            data: (base + 4).max(0) as usize,
            pats: pats.max(0) as usize,
            poss: poss.max(0) as usize,
        })
    }
}

/// One channel's record (-$232E + 0x1C a channel).
#[derive(Clone)]
struct Chan {
    /// +0: this row's note (period and instrument, command, parameter).
    note: [u8; 4],
    /// +4, +8: the sample and its length in words; +10, +14: the repeat; +16 the note's period; +18 the
    /// instrument's volume.
    ptr: usize,
    len: u16,
    rep: usize,
    replen: u16,
    period: u16,
    vol: i16,
    /// +22: borrowed by a sound effect; +23: its first audio interrupt is still to come.
    fx: bool,
    first: bool,
    /// +24: the last note's command and parameter; +26: the last instrument.
    last: [u8; 2],
    ins: i16,
    /// The audio interrupt, modelled: seconds of the borrowed effect still to play (infinite once the channel's
    /// DMA was switched off under it).
    fx_left: f64,
}

impl Default for Chan {
    fn default() -> Self {
        Chan {
            note: [0; 4],
            ptr: 0,
            len: 0,
            rep: 0,
            replen: 0,
            period: 0,
            vol: 0,
            fx: false,
            first: false,
            last: [0; 2],
            ins: 0,
            fx_left: 0.0,
        }
    }
}

/// One sound effect slot as +F364 parse_8svx_header fills it (10 bytes a slot at -$21B6's table).
#[derive(Clone)]
struct FxSlot {
    data: Arc<Vec<i8>>,
    ptr: usize,
    /// Length in bytes (the VHDR's one-shot length, or its repeat length when that is zero).
    len: u16,
    period: u16,
    vol: u8,
    loaded: bool,
}

/// A tune of the table at -$3796 (0x16 bytes each): its file name, whether +A96E preloads it, and whether it is
/// played (silently) while the music is switched off.
#[derive(Clone)]
struct TuneEntry {
    name: String,
    preload: bool,
    when_off: bool,
}

/// The replayer, the effect slots and the loaded files: the state the original kept at -$2100 to -$2400(a4).
pub struct Sound {
    files: Arc<std::collections::BTreeMap<String, Arc<Vec<i8>>>>,
    /// -$368E: tune number to table entry; the table; -$3680: the effect files and their slots.
    tune_map: Vec<u8>,
    tunes: Vec<TuneEntry>,
    fx_files: Vec<(String, i16)>,
    /// -$2246: the period table the arpeggio steps through.
    periods: Vec<u16>,
    /// -$103A: the instrument bank (cvbass1, cvsolo1, cvbassdrum, snare6 one after another).
    bank: Vec<i8>,
    /// -$102A: the tune file last loaded (the cache of +ACB0).
    cached: i16,
    song: Option<Song>,
    fx: Vec<FxSlot>,
    /// -$21B8: the effect slots are set up.
    fx_ready: bool,
    ch: [Chan; 4],
    /// -$21CC playing, -$21D2 fade speed (negative fades), -$21D0 fade, -$21D4 ended or muted, -$21D8 speed,
    /// -$21DA tick in the row, -$21DE position, -$21E0 offset in the pattern, -$21CE channels started this row.
    pub playing: bool,
    fade_speed: i16,
    fade: i16,
    pub ended: bool,
    speed: i16,
    count: i16,
    pos: u16,
    row: usize,
    dma: u16,
    /// -$21FC: the words the portamento commands slide (one a channel, never set from a note).
    slide: [i16; 4],
    /// -$21C2 music off, -$21C0 effects off (the pause menu's switches).
    pub music_off: bool,
    pub fx_off: bool,
    /// The CIA timer, in 1/50 s steps of its clock.
    cia: u64,
    /// The commands for the mixer since the last frame.
    pub out: Vec<Mix>,
}

impl Sound {
    pub fn new(a: &Assets) -> Sound {
        Sound::with(a.sounds.clone(), &a.dat)
    }

    /// The sound from its files and the program's data area (the tables it reads).
    pub fn with(files: Arc<std::collections::BTreeMap<String, Arc<Vec<i8>>>>, d: &[u8]) -> Sound {
        let at = |g: usize| 0x7FFE - g;
        let tune_map = d[at(0x368E)..at(0x368E) + 13].to_vec();
        let mut tunes = Vec::new();
        for i in 0..12 {
            let e = at(0x3796) + i * 0x16;
            let name: Vec<u8> = d[e..e + 14]
                .iter()
                .take_while(|b| **b != 0)
                .copied()
                .collect();
            let w = |o: usize| u16::from_be_bytes([d[e + o], d[e + o + 1]]);
            tunes.push(TuneEntry {
                name: String::from_utf8_lossy(&name).to_lowercase(),
                preload: w(0x12) != 0,
                when_off: w(0x14) != 0,
            });
        }
        let mut fx_files = Vec::new();
        for i in 0..15 {
            let e = at(0x3680) + i * 10;
            let name: Vec<u8> = d[e..e + 8]
                .iter()
                .take_while(|b| **b != 0)
                .copied()
                .collect();
            let slot = i16::from_be_bytes([d[e + 8], d[e + 9]]);
            fx_files.push((String::from_utf8_lossy(&name).to_lowercase(), slot));
        }
        let periods = (0..48)
            .map(|i| u16::from_be_bytes([d[at(0x2246) + 2 * i], d[at(0x2246) + 2 * i + 1]]))
            .collect();
        let empty = FxSlot {
            data: Arc::new(Vec::new()),
            ptr: 0,
            len: 0,
            period: 0,
            vol: 0,
            loaded: false,
        };
        Sound {
            files,
            tune_map,
            tunes,
            fx_files,
            periods,
            bank: Vec::new(),
            cached: -1,
            song: None,
            fx: vec![empty; 32],
            fx_ready: true,
            ch: Default::default(),
            playing: false,
            fade_speed: 0,
            fade: 0,
            ended: true,
            speed: 6,
            count: 6,
            pos: 0,
            row: 0,
            dma: 0,
            slide: [0; 4],
            music_off: false,
            fx_off: false,
            cia: 0,
            out: Vec::new(),
        }
    }

    fn file(&self, name: &str) -> Option<Arc<Vec<i8>>> {
        self.files.get(&name.to_lowercase()).cloned()
    }

    /// +F364 parse_8svx_header: effect slot `slot` from an IFF 8SVX file (volume 64, period 3,579,545 / rate). The
    /// first word of the sample is cleared (it is the silent loop after the one-shot). A file that is not 8SVX
    /// leaves the slot as it was (the original flashed the screen white).
    pub fn set_effect(&mut self, file: &Arc<Vec<i8>>, slot: i16) -> bool {
        let f = file.as_slice();
        if ul(f, 0) != 0x464F_524D {
            return false;
        }
        // VHDR, then BODY, each looked for one long at a time (at most 99 longs)
        let find = |from: usize, id: u32| -> Option<usize> {
            let mut a = from;
            for _ in 1..0x64 {
                let hit = ul(f, a) == id;
                a += 4;
                if hit {
                    return Some(a);
                }
            }
            None
        };
        let Some(v) = find(0, 0x5648_4452) else {
            return false;
        };
        let v = v + 4;
        let mut len = ul(f, v);
        if len == 0 {
            len = ul(f, v + 4);
            if len == 0 {
                return false;
            }
        }
        let mut sps = uw(f, v + 12) as u32;
        if sps == 0 {
            sps = 0x1F40;
        }
        let q = 0x0036_9E99u32 / sps;
        // divu: on an overflow the dividend's low word is kept
        let period = if q > 0xFFFF { 0x9E99 } else { q as u16 };
        let Some(b) = find(v, 0x424F_4459) else {
            return false;
        };
        let body = b + 4;
        let mut data = f.to_vec();
        for x in data.iter_mut().skip(body).take(2) {
            *x = 0;
        }
        if let Some(s) = self.fx.get_mut((slot as usize) & 31) {
            *s = FxSlot {
                data: Arc::new(data),
                ptr: body,
                len: len as u16,
                period,
                vol: 0x40,
                loaded: true,
            };
        }
        true
    }

    /// +A96E build_effect_script_table. Mode 0: the villain's laugh into slot 0 (for the title). Mode 1 (after
    /// waiting for the music to stop, which the caller does): the instrument bank, the preloaded tunes and the
    /// effects of the table at -$3680 (an entry with no name shares the sample of the entry before it).
    pub fn load_sounds(&mut self, mode: i16) {
        if mode == 0 {
            if let Some(f) = self.file("lwvillam.eff") {
                self.set_effect(&f, 0);
            }
            return;
        }
        self.cached = -1;
        let mut bank = Vec::new();
        for n in ["cvbass1", "cvsolo1", "cvbassdrum", "snare6"] {
            if let Some(f) = self.file(n) {
                bank.extend_from_slice(&f);
            }
        }
        self.bank = bank;
        let mut cur: Option<Arc<Vec<i8>>> = None;
        let names = self.fx_files.clone();
        for (name, slot) in names {
            if !name.is_empty() {
                cur = self.file(&format!("{name}.eff"));
            }
            if let Some(f) = cur.clone() {
                self.set_effect(&f, slot);
            }
        }
    }

    /// +EE8C negate_and_store_word: start fading the music out at `n` a step.
    pub fn fade_out(&mut self, n: i16) {
        self.fade_speed = n.wrapping_neg();
    }

    /// +EEA6 check_flag_and_call: true (and the music muted) when the tune has ended or was muted.
    pub fn check_ended(&mut self) -> bool {
        if self.ended {
            self.mute();
            true
        } else {
            false
        }
    }

    /// +EE42 mute_audio_channels: the replayer stops and all four channels' DMA goes off (the music's channel
    /// mask -$21C8 is zero, so no volume is touched).
    pub fn mute(&mut self) {
        if !self.playing {
            return;
        }
        self.playing = false;
        for c in 0..4 {
            self.out.push(Mix::Stop(c));
            if self.ch[c].fx {
                // no audio interrupt will come to give the channel back
                self.ch[c].fx_left = f64::INFINITY;
            }
        }
        self.ended = true;
    }

    /// +ED3A's set-up of the replayer once the tune is parsed.
    fn start_song(&mut self, s: Song) {
        self.fade = 0;
        self.fade_speed = 0;
        self.pos = 0;
        self.row = 0;
        self.speed = 6;
        self.count = 6;
        for c in self.ch.iter_mut() {
            c.fx = false;
            c.first = false;
            c.fx_left = 0.0;
        }
        self.song = Some(s);
        self.ended = false;
        self.playing = true;
    }

    /// The tune number's file as +ACB0 loads it (tune 1000 is the title module).
    fn play_file(&mut self, file: Option<Arc<Vec<i8>>>) {
        self.playing = false;
        self.fade_speed = 0;
        if let Some(f) = file {
            let mut bank = std::mem::take(&mut self.bank);
            let s = Song::parse(&f, &mut bank);
            self.bank = bank;
            if let Some(s) = s {
                self.start_song(s);
            }
        }
    }

    /// +ACB0 for tune 1000: the title module, at once.
    pub fn play_title(&mut self) {
        let f = self.file("hellojarre.mod");
        self.play_file(f);
    }

    /// The rest of +ACB0 once the music has stopped: tune `n` (0 to 12) through -$368E. With the music off only
    /// the tunes marked to play silently start; a preloaded tune plays from memory, the others from their file.
    pub fn play_tune(&mut self, n: i16) {
        let k = self.tune_map.get(n.max(0) as usize).copied().unwrap_or(0) as usize;
        let Some(e) = self.tunes.get(k).cloned() else {
            return;
        };
        if self.music_off && !e.when_off {
            return;
        }
        let f = self.file(&format!("{}.mus", e.name));
        if !e.preload {
            if self.cached != k as i16 && f.is_some() {
                self.cached = k as i16;
            }
            if self.cached != k as i16 {
                return;
            }
        }
        self.play_file(f);
    }

    /// +AC24 check_and_dispatch: the effects the game has samples for go to +F44C on channel 0 or 1; the other
    /// numbers are silent.
    pub fn effect(&mut self, n: i16) {
        if matches!(
            n,
            0x16 | 9 | 10 | 11 | 0xF | 0x10 | 0 | 0x12 | 1 | 7 | 4 | 5 | 2 | 0x14 | 0xD
        ) {
            self.trigger(n, 3);
        }
    }

    /// +F44C process_audio_trigger: effect slot `n` on the first channel of `mask` no effect holds.
    pub fn trigger(&mut self, n: i16, mask: u16) {
        if !self.fx_ready || self.fx_off {
            return;
        }
        let Some(e) = self.fx.get((n as usize) & 31).cloned() else {
            return;
        };
        if !e.loaded {
            return;
        }
        let Some(c) = (0..4).find(|&c| mask & (1 << c) != 0 && !self.ch[c].fx) else {
            return;
        };
        let ch = &mut self.ch[c];
        ch.fx = true;
        ch.ptr = e.ptr;
        ch.rep = e.ptr;
        ch.len = e.len >> 1;
        ch.note[0] = (e.period >> 8) as u8;
        ch.note[1] = e.period as u8;
        ch.replen = 1;
        ch.vol = e.vol as i16;
        ch.first = true;
        let rate = rate_of(e.period);
        let bytes = ch.len as usize * 2;
        ch.fx_left = if rate > 0.0 {
            bytes as f64 / rate as f64
        } else {
            f64::INFINITY
        };
        self.out.push(Mix::Start {
            ch: c,
            block: block(&e.data, e.ptr, bytes),
            next: block(&e.data, e.ptr, 2),
            rate,
            vol: e.vol.min(64),
        });
    }

    /// One vertical blank: the CIA timer's interrupts that fall in it (the replayer), then the audio interrupts
    /// that give borrowed channels back when their effect has played.
    pub fn vblank(&mut self) {
        self.cia += CIA_CLOCK;
        while self.cia >= CIA_PERIOD * VBL_HZ {
            self.cia -= CIA_PERIOD * VBL_HZ;
            self.step();
        }
        for c in self.ch.iter_mut() {
            if c.fx {
                c.fx_left -= 1.0 / VBL_HZ as f64;
                if c.fx_left <= 0.0 {
                    c.fx = false;
                    c.first = false;
                }
            }
        }
    }

    /// +EEB8 process_dma_interrupt_state: one replayer step (the CIA timer interrupt).
    pub fn step(&mut self) {
        if !self.playing {
            return;
        }
        if self.fade_speed < 0 && self.count & 1 != 0 {
            self.fade = self.fade.wrapping_sub(self.fade_speed);
            if self.fade.wrapping_sub(0x40) >= 0 {
                self.mute();
                return;
            }
        }
        self.count = self.count.wrapping_add(1);
        if self.speed <= self.count {
            self.count = 0;
            self.row_step();
        }
        self.effects();
    }

    fn row_step(&mut self) {
        let Some(s) = self.song.as_ref() else {
            return;
        };
        let f = s.file.clone();
        let p = sw(&f, s.poss + 2 * self.pos as usize);
        let off = sw(&f, (s.pats as isize + 2 * p as isize).max(0) as usize);
        let mut a = (s.data as isize + off as isize + self.row as isize).max(0) as usize;
        let d1 = uw(&f, a);
        a += 2;
        self.row += 2;
        self.dma = 0;
        for c in 0..4 {
            self.chan_step(c, d1, &mut a);
        }
        self.ended = false;
        if d1 & 0x8000 != 0 {
            self.row = 0;
            self.pos = self.pos.wrapping_add(1);
            let npos = self.song.as_ref().map_or(0, |s| s.npos);
            if npos == self.pos {
                self.pos = 0;
                self.ended = true;
            }
        }
    }

    /// The volume a channel's register gets: less the fade, nothing with the music off.
    fn set_volume(&mut self, c: usize, v: i16) {
        let mut d = v.wrapping_sub(self.fade);
        if d < 0 || self.music_off {
            d = 0;
        }
        self.out.push(Mix::Volume(c, d.clamp(0, 64) as u8));
    }

    /// +EFDE sound_channel_step: channel `c`'s note of this row (when the row's mask has it).
    fn chan_step(&mut self, c: usize, d1: u16, a: &mut usize) {
        self.ch[c].note = [0; 4];
        let vol = if d1 & (1 << c) != 0 {
            let Some(s) = self.song.as_ref() else {
                return;
            };
            let f = s.file.clone();
            let n = [ub(&f, *a), ub(&f, *a + 1), ub(&f, *a + 2), ub(&f, *a + 3)];
            *a += 4;
            self.row += 4;
            self.ch[c].note = n;
            if self.ch[c].fx {
                return;
            }
            let mut d2 = ((((n[0] >> 4) & 1) << 4) | (n[2] >> 4)) as i16 - 1;
            if d2 < 0 {
                d2 = self.ch[c].ins;
            }
            self.ch[c].ins = d2;
            let per = u16::from_be_bytes([n[0], n[1]]) & 0xFFF;
            self.ch[c].note[0] = n[0] & 0x0F;
            if per != 0 {
                // the instrument list is searched with no end (bounded here)
                let mut e = s.ins_tab;
                let mut found = None;
                for _ in 0..256 {
                    if e + 8 > f.len() {
                        break;
                    }
                    if ub(&f, e) == d2 as u8 {
                        found = Some(e);
                        break;
                    }
                    e += 8;
                }
                if let Some(e) = found {
                    let smp = s.smp.clone();
                    let ptr = s.ins_ptr[(d2 as usize) & 31];
                    let ch = &mut self.ch[c];
                    ch.ptr = ptr;
                    ch.len = uw(&f, e + 2);
                    ch.vol = ub(&f, e + 1) as i8 as i16;
                    ch.rep = (ptr as isize + sw(&f, e + 4) as isize).max(0) as usize;
                    ch.replen = uw(&f, e + 6);
                    ch.period = per;
                    ch.first = true;
                    self.dma |= 1 << c;
                    self.out.push(Mix::Start {
                        ch: c,
                        block: block(&smp, ptr, ch.len as usize * 2),
                        next: block(&smp, ch.rep, ch.replen as usize * 2),
                        rate: rate_of(per),
                        vol: 0,
                    });
                }
            }
            self.ch[c].last = [n[2], n[3]];
            if n[2] & 0xF == 0xC {
                n[3] as i16
            } else {
                self.ch[c].vol
            }
        } else {
            let l = self.ch[c].last;
            if l[0] & 0xF == 0xC {
                l[1] as i16
            } else {
                self.ch[c].vol
            }
        };
        if !self.ch[c].fx {
            self.set_volume(c, vol);
        }
    }

    /// +F1C6: every step, the commands of the channels whose note this row has a parameter.
    fn effects(&mut self) {
        for c in 0..4 {
            if self.ch[c].note[3] != 0 {
                self.command(c);
                if !self.playing {
                    break;
                }
            }
        }
    }

    /// +F1F8: one channel's command. 0 arpeggio, 1 and 2 portamento (+F354's slide words), C volume, E the
    /// filter (the power light; nothing to do here), F speed (a negative parameter stops the tune).
    fn command(&mut self, c: usize) {
        let n = self.ch[c].note;
        let fx = self.ch[c].fx;
        let d0 = if fx { n[2] } else { n[2] & 0xF };
        if !fx {
            match d0 {
                0 => return self.arpeggio(c),
                1 | 2 => {
                    if self.count < 6 {
                        let v = (n[3] & 0xF) as i16;
                        let s = &mut self.slide[c];
                        if d0 == 1 {
                            *s = s.wrapping_sub(v);
                            if *s < 0x71 {
                                *s = 0x71;
                            }
                        } else {
                            *s = s.wrapping_add(v);
                            if *s >= 0x358 {
                                *s = 0x358;
                            }
                        }
                        let r = rate_of(*s as u16);
                        self.out.push(Mix::Rate(c, r));
                    }
                    return;
                }
                0xC => return self.set_volume(c, n[3] as i16),
                _ => {}
            }
        }
        if d0 == 0xF {
            let p = n[3];
            if (p as i8) < 0 {
                self.mute();
            } else {
                self.speed = (p & 0xF) as i16;
            }
        }
    }

    /// The arpeggio: on the steps of a row in turn the note a high nibble up, a low nibble up, the note, nothing,
    /// found by walking the period table from its start to the note's period.
    fn arpeggio(&mut self, c: usize) {
        let p = self.ch[c].note[3];
        let x = match (self.count.wrapping_sub(1)) & 3 {
            0 => (p >> 4) as usize,
            1 => (p & 0xF) as usize,
            2 => {
                let r = rate_of(self.ch[c].period);
                self.out.push(Mix::Rate(c, r));
                return;
            }
            _ => return,
        };
        let base = self.ch[c].period;
        let t = &self.periods;
        let get = |i: usize| t.get(i).copied().unwrap_or(0);
        let mut d0 = x;
        for i in 0..t.len() {
            let mut d2 = get(i + d0);
            if d2 == 0 {
                d0 = d0.saturating_sub(1);
                d2 = get(i + d0);
            }
            if get(i) == base {
                let r = rate_of(d2);
                self.out.push(Mix::Rate(c, r));
                return;
            }
        }
    }

    /// Named state for the comparison with the original.
    pub fn state(&self) -> Vec<(String, i64)> {
        vec![
            ("snd_playing".into(), self.playing as i64),
            ("snd_pos".into(), self.pos as i64),
            ("snd_row".into(), self.row as i64),
            ("snd_count".into(), self.count as i64),
            ("snd_speed".into(), self.speed as i64),
            ("snd_fade".into(), self.fade as i64),
            ("snd_tune".into(), self.cached as i64),
            (
                "snd_fx".into(),
                self.ch
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (c.fx as i64) << i)
                    .sum::<i64>(),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bare() -> Sound {
        Sound::with(Arc::new(Default::default()), &vec![0u8; 0x8000])
    }

    /// A one-instrument "SONG": one position, one pattern of one row (channel 0, period 428, instrument 1).
    fn song() -> Arc<Vec<i8>> {
        let mut f = vec![0u8; 54];
        f[..4].copy_from_slice(b"SONG");
        let a2 = 4;
        let w = |f: &mut Vec<u8>, o: usize, v: u16| f[o..o + 2].copy_from_slice(&v.to_be_bytes());
        w(&mut f, a2 + 16, 38);
        w(&mut f, a2 + 18, 1);
        w(&mut f, a2 + 20, 1);
        w(&mut f, a2 + 22, 1);
        // instrument 0: volume 64, 4 words, repeat at 0 for 1 word
        f[30] = 0;
        f[31] = 64;
        w(&mut f, 32, 4);
        w(&mut f, 34, 0);
        w(&mut f, 36, 1);
        // row: end of pattern, channel 0
        w(&mut f, 46, 0x8001);
        f[48..52].copy_from_slice(&[0x01, 0xAC, 0x10, 0x00]);
        Arc::new(f.into_iter().map(|b| b as i8).collect())
    }

    #[test]
    fn a_song_plays_its_row_and_ends() {
        let mut s = bare();
        s.bank = vec![9; 8];
        s.play_file(Some(song()));
        assert!(s.playing && !s.ended);
        assert_eq!(
            s.bank[..4],
            [0, 0, 0, 0],
            "the first long of the sample is cleared"
        );
        s.step();
        let started = s.out.iter().any(|m| {
            matches!(m, Mix::Start { ch: 0, rate, .. } if (*rate - (PAL_CLOCK / 428.0) as f32).abs() < 0.01)
        });
        assert!(started);
        assert!(s.out.iter().any(|m| matches!(m, Mix::Volume(0, 64))));
        assert!(s.ended, "one position: the song has ended (and loops)");
    }

    #[test]
    fn a_fade_at_four_takes_33_steps() {
        let mut s = bare();
        s.bank = vec![9; 8];
        s.play_file(Some(song()));
        s.fade_out(4);
        let mut n = 0;
        while s.playing && n < 1000 {
            s.step();
            n += 1;
        }
        // the fade moves on the odd steps of a row (1, 3, 5): 16 of them, from the third step
        assert_eq!(n, 33);
        assert!(s.check_ended());
    }

    /// An 8SVX file of 8 samples at 8,000 a second.
    fn svx() -> Arc<Vec<i8>> {
        let mut f = Vec::new();
        f.extend_from_slice(b"FORM\0\0\0\x2C8SVXVHDR\0\0\0\x14");
        f.extend_from_slice(&[
            0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0x1F, 0x40, 1, 0, 0, 1, 0, 0,
        ]);
        f.extend_from_slice(b"BODY\0\0\0\x08");
        f.extend_from_slice(&[5, 6, 7, 8, 9, 10, 11, 12]);
        Arc::new(f.into_iter().map(|b| b as i8).collect())
    }

    #[test]
    fn effects_borrow_channel_0_then_1_and_give_them_back() {
        let mut s = bare();
        assert!(s.set_effect(&svx(), 0x16));
        let e = &s.fx[0x16];
        assert_eq!((e.len, e.period, e.vol), (8, 447, 64));
        assert_eq!(
            e.data[e.ptr..e.ptr + 3],
            [0, 0, 7],
            "the first word is cleared"
        );
        s.effect(0x16);
        s.effect(0x16);
        s.effect(0x16);
        assert_eq!((s.ch[0].fx, s.ch[1].fx, s.ch[2].fx), (true, true, false));
        assert_eq!(s.out.len(), 2, "no channel left for the third");
        s.effect(3);
        assert_eq!(s.out.len(), 2, "effect 3 has no sample");
        s.vblank();
        assert!(!s.ch[0].fx && !s.ch[1].fx);
    }
}
