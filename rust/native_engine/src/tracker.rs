//! A native ProTracker module player (the 4-channel "M.K." module format). It follows the ProTracker 2 replay routine
//! tick for tick: one `tick()` per game frame (the format's "VBlank timing"), speed 6 at the start, the effects 0-F
//! and E0-EF with their usual quirks (a first portamento before any other moves nothing, sample-offset clamping, the
//! 37-step period search). The period tables come with the music (`TrackerTables`, from the game's asset pack), so a
//! game's own tables are used. The output is four mixer voices panned left, right, right, left.
//!
//! The module format counts pitch in "periods" of a 3 546 895 Hz clock (the format's definition, PAL timing); a voice's
//! rate is that clock divided by the period.

use std::sync::Arc;

use crate::audio::{Block, Mixer, Pan};

/// The clock the module format's periods divide (PAL).
pub const PERIOD_CLOCK: f32 = 3_546_895.0;
const PANS: [Pan; 4] = [-1.0, 1.0, 1.0, -1.0];

/// The replayer's tables: 16 finetunes x 36 periods, the 32-step vibrato sine, the 16 funk speeds.
#[derive(Debug, Clone, PartialEq)]
pub struct TrackerTables {
    pub periods: Vec<u16>,
    pub vibrato: Vec<u8>,
    pub funk: Vec<u8>,
}

impl TrackerTables {
    pub fn check(&self) -> Result<(), String> {
        if self.periods.len() < 16 * 36 || self.vibrato.len() < 32 || self.funk.len() < 16 {
            return Err(
                "tracker tables: need 576 periods, 32 vibrato steps, 16 funk speeds".into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct Chan {
    note: u16,
    cmd: u16,
    start: usize,
    length: u16,
    loopstart: usize,
    replen: u16,
    period: u16,
    finetune: u8,
    volume: u8,
    dmabit: u16,
    toneportdirec: u8,
    toneportspeed: u8,
    wantedperiod: u16,
    vibratocmd: u8,
    vibratopos: u8,
    tremolocmd: u8,
    tremolopos: u8,
    wavecontrol: u8,
    glissfunk: u8,
    sampleoffset: u8,
    pattpos: u8,
    loopcount: u8,
    funkoffset: u8,
    wavestart: usize,
    reallength: u16,
}

impl Chan {
    fn param(&self) -> u8 {
        self.cmd as u8
    }
    fn set_param(&mut self, p: u8) {
        self.cmd = (self.cmd & 0xFF00) | p as u16;
    }
    fn effect(&self) -> u8 {
        (self.cmd >> 8) as u8 & 0x0F
    }
}

/// The four output channels' registers as the replayer sets them.
#[derive(Debug, Clone, Default)]
struct Out {
    lc: usize,
    len: u16,
    per: u16,
    vol: u16,
    dma: bool,
}

/// A loaded module and the replayer's state.
pub struct Tracker {
    data: Arc<Vec<i8>>,
    t: TrackerTables,
    starts: [usize; 31],
    ch: [Chan; 4],
    out: [Out; 4],
    speed: u8,
    counter: u8,
    song_pos: u8,
    pbreak_pos: u8,
    posjump_flag: bool,
    pbreak_flag: bool,
    low_mask: u8,
    patt_del_time: u8,
    patt_del_time2: u8,
    pattern_pos: u16,
    dmacon_temp: u16,
    /// The first of the four mixer voices it plays on.
    pub voice0: usize,
}

fn rd(d: &[i8], o: usize) -> u8 {
    d.get(o).copied().unwrap_or(0) as u8
}
fn rw(d: &[i8], o: usize) -> u16 {
    (rd(d, o) as u16) << 8 | rd(d, o + 1) as u16
}

impl Tracker {
    /// Loads a module and starts it from the beginning (the replayer's init): speed 6, position 0, every channel
    /// silent. As the replayer does, the first four bytes of each sample are cleared (a silent loop for one-shots).
    pub fn new(module: &[u8], t: TrackerTables) -> Result<Tracker, String> {
        t.check()?;
        if module.len() < 1084 {
            return Err("module too short".into());
        }
        let mut data: Vec<i8> = module.iter().map(|&b| b as i8).collect();
        // the highest pattern in the 128-entry order list (signed compares, as the replayer)
        let mut max: i8 = 0;
        for &b in &module[952..1080] {
            if (b as i8) > max {
                max = b as i8;
            }
        }
        let mut p = 1084 + (max as usize + 1) * 1024;
        let mut starts = [0usize; 31];
        for (i, s) in starts.iter_mut().enumerate() {
            for k in 0..4 {
                if let Some(x) = data.get_mut(p + k) {
                    *x = 0;
                }
            }
            *s = p;
            p += rw(&data, 42 + 30 * i) as usize * 2;
        }
        let mut ch: [Chan; 4] = Default::default();
        for (i, c) in ch.iter_mut().enumerate() {
            c.dmabit = 1 << i;
        }
        Ok(Tracker {
            data: Arc::new(data),
            t,
            starts,
            ch,
            out: Default::default(),
            speed: 6,
            counter: 0,
            song_pos: 0,
            pbreak_pos: 0,
            posjump_flag: false,
            pbreak_flag: false,
            low_mask: 0,
            patt_del_time: 0,
            patt_del_time2: 0,
            pattern_pos: 0,
            dmacon_temp: 0,
            voice0: 0,
        })
    }

    /// (song position, row) for tests and state dumps.
    pub fn position(&self) -> (u8, u16) {
        (self.song_pos, self.pattern_pos / 16)
    }

    // ---- the output channels -------------------------------------------------------------------------------
    fn block(&self, lc: usize, len: u16) -> Block {
        let bytes = if len == 0 { 131_072 } else { len as usize * 2 };
        Block {
            data: self.data.clone(),
            start: lc,
            len: bytes.min(self.data.len().saturating_sub(lc)),
        }
    }
    fn dma_off(&mut self, mask: u16, m: &mut Mixer) {
        for i in 0..4 {
            if mask & (1 << i) != 0 {
                self.out[i].dma = false;
                m.stop(self.voice0 + i);
            }
        }
    }
    fn dma_on(&mut self, mask: u16, m: &mut Mixer) {
        for (i, &pan) in PANS.iter().enumerate() {
            if mask & (1 << i) != 0 && !self.out[i].dma {
                self.out[i].dma = true;
                let o = self.out[i].clone();
                let b = self.block(o.lc, o.len);
                m.start(
                    self.voice0 + i,
                    b.clone(),
                    b,
                    rate(o.per),
                    o.vol.min(64) as u8,
                    pan,
                );
            }
        }
    }
    fn set_lc_len(&mut self, i: usize, lc: usize, len: u16, m: &mut Mixer) {
        self.out[i].lc = lc;
        self.out[i].len = len;
        if self.out[i].dma {
            let b = self.block(lc, len);
            m.set_next(self.voice0 + i, b);
        }
    }
    fn set_per(&mut self, i: usize, p: u16, m: &mut Mixer) {
        self.out[i].per = p;
        m.set_rate(self.voice0 + i, rate(p));
    }
    fn set_vol(&mut self, i: usize, v: u16, m: &mut Mixer) {
        self.out[i].vol = v;
        m.set_volume(self.voice0 + i, v.min(64) as u8);
    }

    // ---- the replayer --------------------------------------------------------------------------------------
    /// One tick (mt_music). True when the tick read a new row (the replayer's heaviest tick: it restarts the voices'
    /// sample playback, which games that time their own frames around the music have to allow for).
    pub fn tick(&mut self, m: &mut Mixer) -> bool {
        self.counter = self.counter.wrapping_add(1);
        if self.counter < self.speed {
            self.no_new_all_channels(m);
            self.no_new_pos_yet();
            return false;
        }
        self.counter = 0;
        let row = self.patt_del_time2 == 0;
        if row {
            self.get_new_note(m);
        } else {
            self.no_new_all_channels(m);
        }
        // mt_dskip
        self.pattern_pos = self.pattern_pos.wrapping_add(16);
        if self.patt_del_time != 0 {
            self.patt_del_time2 = self.patt_del_time;
            self.patt_del_time = 0;
        }
        if self.patt_del_time2 != 0 {
            self.patt_del_time2 -= 1;
            if self.patt_del_time2 != 0 {
                self.pattern_pos = self.pattern_pos.wrapping_sub(16);
            }
        }
        if self.pbreak_flag {
            self.pbreak_flag = false;
            self.pattern_pos = self.pbreak_pos as u16 * 16;
            self.pbreak_pos = 0;
        }
        if self.pattern_pos >= 0x400 {
            self.next_position();
        }
        self.no_new_pos_yet();
        row
    }

    fn next_position(&mut self) {
        self.pattern_pos = self.pbreak_pos as u16 * 16;
        self.pbreak_pos = 0;
        self.posjump_flag = false;
        self.song_pos = (self.song_pos.wrapping_add(1)) & 0x7F;
        if self.song_pos >= rd(&self.data, 950) {
            self.song_pos = 0;
        }
    }

    fn no_new_pos_yet(&mut self) {
        while self.posjump_flag {
            self.next_position();
        }
    }

    fn no_new_all_channels(&mut self, m: &mut Mixer) {
        for i in 0..4 {
            self.check_efx(i, m);
        }
    }

    fn get_new_note(&mut self, m: &mut Mixer) {
        let pat = rd(&self.data, 952 + self.song_pos as usize) as usize;
        let mut d1 = 1084 + pat * 1024 + self.pattern_pos as usize;
        self.dmacon_temp = 0;
        for i in 0..4 {
            self.play_voice(i, d1, m);
            d1 += 4;
        }
        // mt_SetDMA: start the triggered channels, then every channel's loop
        let dm = self.dmacon_temp;
        self.dma_on(dm, m);
        for i in (0..4).rev() {
            let (ls, rl) = (self.ch[i].loopstart, self.ch[i].replen);
            self.set_lc_len(i, ls, rl, m);
        }
    }

    fn play_voice(&mut self, i: usize, row: usize, m: &mut Mixer) {
        if self.ch[i].note == 0 && self.ch[i].cmd == 0 {
            self.per_nop(i, m);
        }
        let c = &mut self.ch[i];
        c.note = rw(&self.data, row);
        c.cmd = rw(&self.data, row + 2);
        let s = ((c.cmd >> 12) as u8) | ((c.note >> 8) as u8 & 0xF0);
        if s != 0 {
            let h = 20 + 30 * (s as usize - 1);
            let d = &self.data;
            c.start = self.starts[s as usize - 1];
            c.length = rw(d, h + 22);
            c.reallength = c.length;
            c.finetune = rd(d, h + 24);
            c.volume = rd(d, h + 25);
            let rep = rw(d, h + 26);
            let replen = rw(d, h + 28);
            if rep != 0 {
                c.loopstart = c.start + rep as usize * 2;
                c.wavestart = c.loopstart;
                c.length = rep.wrapping_add(replen);
            } else {
                c.loopstart = c.start;
                c.wavestart = c.loopstart;
            }
            c.replen = replen;
            let v = c.volume as u16;
            self.set_vol(i, v, m);
        }
        // mt_SetRegs
        let c = &self.ch[i];
        if c.note & 0x0FFF == 0 {
            self.check_more_efx(i, m);
            return;
        }
        if c.cmd & 0x0FF0 == 0x0E50 {
            self.set_fine_tune(i);
        } else {
            match c.effect() {
                3 | 5 => {
                    self.set_tone_porta(i);
                    self.check_more_efx(i, m);
                    return;
                }
                9 => self.check_more_efx(i, m),
                _ => {}
            }
        }
        self.set_period(i, m);
    }

    /// The period table index of `period` in finetune row 0's 37-step search (index 37 when none matches).
    fn find_note(&self, period: u16) -> usize {
        (0..37).find(|&k| self.t.periods[k] <= period).unwrap_or(37)
    }

    fn set_period(&mut self, i: usize, m: &mut Mixer) {
        let k = self.find_note(self.ch[i].note & 0x0FFF);
        let row = self.ch[i].finetune as usize * 36;
        self.ch[i].period = self.t.periods.get(row + k).copied().unwrap_or(0);
        if self.ch[i].cmd & 0x0FF0 == 0x0ED0 {
            self.check_more_efx(i, m);
            return;
        }
        let bit = self.ch[i].dmabit;
        self.dma_off(bit, m);
        let c = &mut self.ch[i];
        if c.wavecontrol & 4 == 0 {
            c.vibratopos = 0;
        }
        if c.wavecontrol & 0x40 == 0 {
            c.tremolopos = 0;
        }
        let (st, ln, per) = (c.start, c.length, c.period);
        self.set_lc_len(i, st, ln, m);
        self.set_per(i, per, m);
        self.dmacon_temp |= bit;
        self.check_more_efx(i, m);
    }

    fn per_nop(&mut self, i: usize, m: &mut Mixer) {
        let p = self.ch[i].period;
        self.set_per(i, p, m);
    }

    fn check_efx(&mut self, i: usize, m: &mut Mixer) {
        self.update_funk(i);
        if self.ch[i].cmd & 0x0FFF == 0 {
            self.per_nop(i, m);
            return;
        }
        match self.ch[i].effect() {
            0 => self.arpeggio(i, m),
            1 => self.porta_up(i, m),
            2 => self.porta_down(i, m),
            3 => self.tone_portamento(i, m),
            4 => self.vibrato(i, m),
            5 => {
                self.tone_port_no_change(i, m);
                self.volume_slide(i, m);
            }
            6 => {
                self.vibrato2(i, m);
                self.volume_slide(i, m);
            }
            0xE => self.e_commands(i, m),
            e => {
                self.per_nop(i, m);
                if e == 7 {
                    self.tremolo(i, m);
                } else if e == 0xA {
                    self.volume_slide(i, m);
                }
            }
        }
    }

    fn arpeggio(&mut self, i: usize, m: &mut Mixer) {
        let c = &self.ch[i];
        let off = match self.counter % 3 {
            0 => {
                let p = c.period;
                self.set_per(i, p, m);
                return;
            }
            1 => (c.param() >> 4) as usize,
            _ => (c.param() & 0x0F) as usize,
        };
        let row = c.finetune as usize * 36;
        for k in 0..37 {
            if c.period >= self.t.periods.get(row + k).copied().unwrap_or(0) {
                let p = self.t.periods.get(row + k + off).copied().unwrap_or(0);
                self.set_per(i, p, m);
                return;
            }
        }
    }

    fn porta_up(&mut self, i: usize, m: &mut Mixer) {
        let d = (self.ch[i].param() & self.low_mask) as u16;
        self.low_mask = 0xFF;
        let c = &mut self.ch[i];
        c.period = c.period.wrapping_sub(d);
        if (c.period & 0x0FFF) < 0x71 {
            c.period = (c.period & 0xF000) | 0x71;
        }
        let p = c.period & 0x0FFF;
        self.set_per(i, p, m);
    }

    fn porta_down(&mut self, i: usize, m: &mut Mixer) {
        let d = (self.ch[i].param() & self.low_mask) as u16;
        self.low_mask = 0xFF;
        let c = &mut self.ch[i];
        c.period = c.period.wrapping_add(d);
        if (c.period & 0x0FFF) >= 0x358 {
            c.period = (c.period & 0xF000) | 0x358;
        }
        let p = c.period & 0x0FFF;
        self.set_per(i, p, m);
    }

    fn fine_porta_up(&mut self, i: usize, m: &mut Mixer) {
        if self.counter == 0 {
            self.low_mask = 0x0F;
            self.porta_up(i, m);
        }
    }

    fn fine_porta_down(&mut self, i: usize, m: &mut Mixer) {
        if self.counter == 0 {
            self.low_mask = 0x0F;
            self.porta_down(i, m);
        }
    }

    /// The tone-portamento target from the note's period in the channel's finetune row. (This game's replayer
    /// searches from an offset left in a register; the module it plays never uses effects 3 or 5, so the format's
    /// documented search is used, which is what other modules need.)
    fn set_tone_porta(&mut self, i: usize) {
        let c = &self.ch[i];
        let want = c.note & 0x0FFF;
        let row = c.finetune as usize * 36;
        let k = (0..36)
            .find(|&k| self.t.periods[row + k] <= want)
            .unwrap_or(35);
        let target = self.t.periods[row + k];
        let c = &mut self.ch[i];
        c.wantedperiod = target;
        c.toneportdirec = 0;
        if c.period == target {
            c.wantedperiod = 0;
        } else if (target as i16) < (c.period as i16) {
            c.toneportdirec = 1;
        }
    }

    fn tone_portamento(&mut self, i: usize, m: &mut Mixer) {
        let p = self.ch[i].param();
        if p != 0 {
            self.ch[i].toneportspeed = p;
            self.ch[i].set_param(0);
        }
        self.tone_port_no_change(i, m);
    }

    fn tone_port_no_change(&mut self, i: usize, m: &mut Mixer) {
        let c = &mut self.ch[i];
        if c.wantedperiod == 0 {
            return;
        }
        let d = c.toneportspeed as u16;
        if c.toneportdirec == 0 {
            c.period = c.period.wrapping_add(d);
            if (c.wantedperiod as i16) <= (c.period as i16) {
                c.period = c.wantedperiod;
                c.wantedperiod = 0;
            }
        } else {
            c.period = c.period.wrapping_sub(d);
            if (c.wantedperiod as i16) >= (c.period as i16) {
                c.period = c.wantedperiod;
                c.wantedperiod = 0;
            }
        }
        let mut p = c.period;
        if c.glissfunk & 0x0F != 0 {
            let row = c.finetune as usize * 36;
            let k = (0..36)
                .find(|&k| self.t.periods[row + k] <= p)
                .unwrap_or(35);
            p = self.t.periods[row + k];
        }
        self.set_per(i, p, m);
    }

    fn vibrato(&mut self, i: usize, m: &mut Mixer) {
        let c = &mut self.ch[i];
        let p = c.param();
        if p != 0 {
            let mut d2 = c.vibratocmd;
            if p & 0x0F != 0 {
                d2 = (d2 & 0xF0) | (p & 0x0F);
            }
            if p & 0xF0 != 0 {
                d2 = (d2 & 0x0F) | (p & 0xF0);
            }
            c.vibratocmd = d2;
        }
        self.vibrato2(i, m);
    }

    fn wave(&self, pos: u8, kind: u8) -> u16 {
        let idx = ((pos as u16) >> 2) & 0x1F;
        match kind & 3 {
            0 => self.t.vibrato[idx as usize] as u16,
            1 => {
                let r = ((idx << 3) & 0xFF) as u8;
                if (pos as i8) < 0 {
                    (0xFFu8.wrapping_sub(r)) as u16
                } else {
                    r as u16
                }
            }
            _ => 0xFF,
        }
    }

    fn vibrato2(&mut self, i: usize, m: &mut Mixer) {
        let c = &self.ch[i];
        let amp = self.wave(c.vibratopos, c.wavecontrol);
        let d = (amp * (c.vibratocmd & 0x0F) as u16) >> 7;
        let p = if (c.vibratopos as i8) < 0 {
            c.period.wrapping_sub(d)
        } else {
            c.period.wrapping_add(d)
        };
        self.set_per(i, p, m);
        let c = &mut self.ch[i];
        c.vibratopos = c.vibratopos.wrapping_add((c.vibratocmd >> 2) & 0x3C);
    }

    fn tremolo(&mut self, i: usize, m: &mut Mixer) {
        let c = &mut self.ch[i];
        let p = c.param();
        if p != 0 {
            let mut d2 = c.tremolocmd;
            if p & 0x0F != 0 {
                d2 = (d2 & 0xF0) | (p & 0x0F);
            }
            if p & 0xF0 != 0 {
                d2 = (d2 & 0x0F) | (p & 0xF0);
            }
            c.tremolocmd = d2;
        }
        let c = &self.ch[i];
        let amp = self.wave(c.tremolopos, c.wavecontrol >> 4);
        let d = ((amp * (c.tremolocmd & 0x0F) as u16) >> 6) as i16;
        let v = c.volume as i16;
        let mut nv = if (c.tremolopos as i8) < 0 {
            v - d
        } else {
            v + d
        };
        nv = nv.clamp(0, 0x40);
        self.set_vol(i, nv as u16, m);
        let c = &mut self.ch[i];
        c.tremolopos = c.tremolopos.wrapping_add((c.tremolocmd >> 2) & 0x3C);
    }

    fn volume_slide(&mut self, i: usize, m: &mut Mixer) {
        let p = self.ch[i].param();
        if p >> 4 != 0 {
            self.vol_slide_up(i, p >> 4, m);
        } else {
            self.vol_slide_down(i, p & 0x0F, m);
        }
    }

    fn vol_slide_up(&mut self, i: usize, d: u8, m: &mut Mixer) {
        let c = &mut self.ch[i];
        c.volume = c.volume.wrapping_add(d);
        if (c.volume.wrapping_sub(0x40) as i8) >= 0 {
            c.volume = 0x40;
        }
        let v = c.volume as u16;
        self.set_vol(i, v, m);
    }

    fn vol_slide_down(&mut self, i: usize, d: u8, m: &mut Mixer) {
        let c = &mut self.ch[i];
        let r = c.volume.wrapping_sub(d);
        c.volume = if (r as i8) < 0 { 0 } else { r };
        let v = c.volume as u16;
        self.set_vol(i, v, m);
    }

    fn check_more_efx(&mut self, i: usize, m: &mut Mixer) {
        self.update_funk(i);
        match self.ch[i].effect() {
            9 => self.sample_offset(i),
            0xB => self.position_jump(i),
            0xD => self.pattern_break(i),
            0xE => self.e_commands(i, m),
            0xF => self.set_speed(i),
            0xC => self.volume_change(i, m),
            _ => self.per_nop(i, m),
        }
    }

    fn sample_offset(&mut self, i: usize) {
        let c = &mut self.ch[i];
        if c.param() != 0 {
            c.sampleoffset = c.param();
        }
        let d = (c.sampleoffset as u16) << 7;
        if (d as i16) < (c.length as i16) {
            c.length -= d;
            c.start += d as usize * 2;
        } else {
            c.length = 1;
        }
    }

    fn position_jump(&mut self, i: usize) {
        self.song_pos = self.ch[i].param().wrapping_sub(1);
        self.pbreak_pos = 0;
        self.posjump_flag = true;
    }

    fn volume_change(&mut self, i: usize, m: &mut Mixer) {
        let v = self.ch[i].param().min(0x40);
        self.ch[i].volume = v;
        self.set_vol(i, v as u16, m);
    }

    fn pattern_break(&mut self, i: usize) {
        let p = self.ch[i].param();
        let row = (p >> 4) * 10 + (p & 0x0F);
        if row > 0x3F {
            self.pbreak_pos = 0;
        } else {
            self.pbreak_pos = row;
        }
        self.posjump_flag = true;
    }

    fn set_speed(&mut self, i: usize) {
        let p = self.ch[i].param();
        if p != 0 {
            self.counter = 0;
            self.speed = p;
        }
    }

    fn e_commands(&mut self, i: usize, m: &mut Mixer) {
        let p = self.ch[i].param();
        match p >> 4 {
            0 => {} // filter on/off: the output has no switchable filter
            1 => self.fine_porta_up(i, m),
            2 => self.fine_porta_down(i, m),
            3 => {
                let c = &mut self.ch[i];
                c.glissfunk = (c.glissfunk & 0xF0) | (p & 0x0F);
            }
            4 => {
                let c = &mut self.ch[i];
                c.wavecontrol = (c.wavecontrol & 0xF0) | (p & 0x0F);
            }
            5 => self.set_fine_tune(i),
            6 => self.jump_loop(i),
            7 => {
                let c = &mut self.ch[i];
                c.wavecontrol = (c.wavecontrol & 0x0F) | ((p & 0x0F) << 4);
            }
            9 => self.retrig_note(i, m),
            0xA if self.counter == 0 => self.vol_slide_up(i, p & 0x0F, m),
            0xB if self.counter == 0 => self.vol_slide_down(i, p & 0x0F, m),
            0xC => {
                if p & 0x0F == self.counter {
                    self.ch[i].volume = 0;
                    self.set_vol(i, 0, m);
                }
            }
            0xD => {
                if p & 0x0F == self.counter && self.ch[i].note != 0 {
                    self.do_retrig(i, m);
                }
            }
            0xE => {
                if self.counter == 0 && self.patt_del_time2 == 0 {
                    self.patt_del_time = (p & 0x0F) + 1;
                }
            }
            0xF if self.counter == 0 => {
                let c = &mut self.ch[i];
                c.glissfunk = (c.glissfunk & 0x0F) | ((p & 0x0F) << 4);
                if p & 0x0F != 0 {
                    self.update_funk(i);
                }
            }
            _ => {}
        }
    }

    fn set_fine_tune(&mut self, i: usize) {
        let p = self.ch[i].param();
        self.ch[i].finetune = p & 0x0F;
    }

    fn jump_loop(&mut self, i: usize) {
        if self.counter != 0 {
            return;
        }
        let p = self.ch[i].param() & 0x0F;
        if p == 0 {
            self.ch[i].pattpos = (self.pattern_pos >> 4) as u8;
            return;
        }
        let c = &mut self.ch[i];
        if c.loopcount != 0 {
            c.loopcount -= 1;
            if c.loopcount == 0 {
                return;
            }
        } else {
            c.loopcount = p;
        }
        self.pbreak_pos = self.ch[i].pattpos;
        self.pbreak_flag = true;
    }

    fn retrig_note(&mut self, i: usize, m: &mut Mixer) {
        let n = self.ch[i].param() & 0x0F;
        if n == 0 {
            return;
        }
        if self.counter == 0 && self.ch[i].note & 0x0FFF != 0 {
            return;
        }
        if self.counter.is_multiple_of(n) {
            self.do_retrig(i, m);
        }
    }

    fn do_retrig(&mut self, i: usize, m: &mut Mixer) {
        let bit = self.ch[i].dmabit;
        self.dma_off(bit, m);
        let (st, ln) = (self.ch[i].start, self.ch[i].length);
        self.set_lc_len(i, st, ln, m);
        self.dma_on(bit, m);
        let (ls, rl, per) = (self.ch[i].loopstart, self.ch[i].replen, self.ch[i].period);
        self.set_lc_len(i, ls, rl, m);
        self.set_per(i, per, m);
    }

    /// Invert loop (EFx): flips one byte of the looped part at the funk speed.
    fn update_funk(&mut self, i: usize) {
        let c = &mut self.ch[i];
        let s = c.glissfunk >> 4;
        if s == 0 {
            return;
        }
        c.funkoffset = c.funkoffset.wrapping_add(self.t.funk[s as usize]);
        if c.funkoffset & 0x80 == 0 {
            return;
        }
        c.funkoffset = 0;
        let end = c.loopstart + c.replen as usize * 2;
        let mut a = c.wavestart + 1;
        if a >= end {
            a = c.loopstart;
        }
        c.wavestart = a;
        let d = Arc::make_mut(&mut self.data);
        if let Some(b) = d.get_mut(a) {
            *b = (-1i8).wrapping_sub(*b);
        }
    }
}

fn rate(period: u16) -> f32 {
    if period == 0 {
        0.0
    } else {
        PERIOD_CLOCK / period as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic module: one pattern, one sample, notes on channel 0.
    fn module(rows: &[(usize, u16, u8, u16)]) -> Vec<u8> {
        let mut m = vec![0u8; 1084 + 1024 + 64];
        // sample 1: 32 words, volume 40, no loop (replen 1)
        m[20 + 22..20 + 24].copy_from_slice(&32u16.to_be_bytes());
        m[20 + 25] = 40;
        m[20 + 28..20 + 30].copy_from_slice(&1u16.to_be_bytes());
        m[950] = 1;
        m[1080..1084].copy_from_slice(b"M.K.");
        for &(row, period, sample, cmd) in rows {
            let o = 1084 + row * 16;
            m[o..o + 2].copy_from_slice(&(period | ((sample as u16 & 0xF0) << 8)).to_be_bytes());
            m[o + 2..o + 4].copy_from_slice(&(cmd | ((sample as u16 & 0x0F) << 12)).to_be_bytes());
        }
        for (k, b) in m[1084 + 1024..].iter_mut().enumerate() {
            *b = (k as u8).wrapping_mul(7);
        }
        m
    }

    fn tables() -> TrackerTables {
        // finetune 0 row: the format's documented note periods; the other rows a copy (enough for the tests)
        let row: Vec<u16> = vec![
            856, 808, 762, 720, 678, 640, 604, 570, 538, 508, 480, 453, 428, 404, 381, 360, 339,
            320, 302, 285, 269, 254, 240, 226, 214, 202, 190, 180, 170, 160, 151, 143, 135, 127,
            120, 113,
        ];
        TrackerTables {
            periods: row.iter().cycle().take(576).copied().collect(),
            vibrato: (0..32).map(|k| (k * 8) as u8).collect(),
            funk: (0..16).map(|k| k as u8).collect(),
        }
    }

    #[test]
    fn a_note_starts_after_speed_ticks_with_the_sample_volume() {
        let mut m = Mixer::new(44100, 50, 0);
        let mut t = Tracker::new(&module(&[(0, 428, 1, 0)]), tables()).unwrap();
        // speed 6: the first row plays on the 6th tick (counter reaches speed)
        for _ in 0..5 {
            t.tick(&mut m);
            assert!(!m.is_playing(0));
        }
        t.tick(&mut m);
        assert!(m.is_playing(0));
        assert_eq!(t.out[0].per, 428);
        assert_eq!(t.out[0].vol, 40);
        assert_eq!(t.position(), (0, 1));
        // the first 4 bytes of the sample were cleared
        assert_eq!(&t.data[t.starts[0]..t.starts[0] + 5], &[0, 0, 0, 0, 28]);
    }

    #[test]
    fn effects_speed_volume_slide_and_first_porta_quirk() {
        let mut m = Mixer::new(44100, 50, 0);
        let rows = [
            (0, 428, 1, 0x0F03),
            (1, 0, 0, 0x0A02),
            (2, 0, 0, 0x0105),
            (3, 0, 0, 0x0C7F),
        ];
        let mut t = Tracker::new(&module(&rows), tables()).unwrap();
        for _ in 0..6 {
            t.tick(&mut m);
        }
        assert_eq!(t.speed, 3);
        // row 1 plays on tick 9; the volume slides down by 2 on ticks 10 and 11
        for _ in 0..5 {
            t.tick(&mut m);
        }
        assert_eq!(t.out[0].vol, 36);
        // row 2 (ticks 12-14): the first portamento ever moves nothing (mask 0), the next moves 5
        for _ in 0..3 {
            t.tick(&mut m);
        }
        assert_eq!(t.out[0].per, 423);
        // row 3: set volume clamps to 64
        t.tick(&mut m);
        assert_eq!(t.out[0].vol, 64);
    }

    #[test]
    fn pattern_end_wraps_to_the_song_start() {
        let mut m = Mixer::new(44100, 50, 0);
        let mut t = Tracker::new(&module(&[(0, 0, 0, 0x0D00)]), tables()).unwrap();
        for _ in 0..6 {
            t.tick(&mut m);
        }
        // D00 on row 0: break to row 0 of the next position, which wraps to position 0
        assert_eq!(t.position(), (0, 0));
    }
}
