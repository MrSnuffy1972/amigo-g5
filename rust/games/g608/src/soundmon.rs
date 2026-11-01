//! The crack intro's music player (runtime $070000, a SoundMon 2 replayer), rewritten: patterns of three-byte rows,
//! sample and synth instruments (a 32-byte waveform shaped each frame by its ADSR, LFO and EG tables), arpeggio,
//! slides and the speed and jump effects. It writes the four channels' settings as sound commands.

use std::sync::Arc;

use native_engine::audio::Block;

use crate::co::Snd;

const CLOCK: f32 = 3_546_895.0;

#[derive(Clone, Default)]
struct Ch {
    period: u16,
    vol: u8,
    instr: u8,
    rep_ptr: Option<usize>,
    rep_len: u16,
    note: u8,
    arp1: u8,
    slide: i8,
    arp2: u8,
    eg_pos: u16,
    lfo_pos: u16,
    adsr_pos: u16,
    eg_cnt: u16,
    lfo_cnt: u16,
    adsr_cnt: u16,
    synth: bool,
    eg_cur: u8,
    eg_ctl: u8,
    lfo_ctl: u8,
    adsr_ctl: u8,
    /// The synth waveform being shaped and its untouched copy.
    wave: Option<usize>,
    backup: [u8; 32],
    // the sound chip's registers as last written
    hw_per: u16,
    hw_vol: u16,
    hw_next: Option<(usize, u16)>,
}

pub struct SoundMon {
    m: Vec<u8>,
    periods: Vec<u16>,
    synth: usize,
    samples: [usize; 15],
    len: u16,
    pos: u16,
    row: u8,
    speed: u8,
    count: u8,
    arp: u8,
    rep: u8,
    ch: [Ch; 4],
    out: Vec<Snd>,
}

fn w(m: &[u8], o: usize) -> u16 {
    u16::from_be_bytes([*m.get(o).unwrap_or(&0), *m.get(o + 1).unwrap_or(&0)])
}

impl SoundMon {
    /// initiate_emotion_sprites (+$40): finds the synth tables and the samples after the patterns.
    pub fn new(module: &[u8], periods: &[u16]) -> SoundMon {
        let m = module.to_vec();
        let tables = if &m[26..29] == b"V.2" {
            m[29] as usize
        } else {
            0
        };
        let len = w(&m, 30);
        let mut maxpat = 1u16;
        for i in 0..len as usize * 4 {
            maxpat = maxpat.max(w(&m, 0x200 + 4 * i));
        }
        let synth = 0x200 + len as usize * 16 + maxpat as usize * 0x30;
        let mut d0 = synth + tables * 64;
        let mut samples = [0usize; 15];
        for (i, s) in samples.iter_mut().enumerate() {
            *s = d0;
            let e = 32 + 32 * i;
            if m[e] != 0xFF {
                d0 += w(&m, e + 24) as usize * 2;
            }
        }
        let ch = Ch {
            vol: 0xFF,
            rep_len: 1,
            ..Ch::default()
        };
        SoundMon {
            m,
            periods: periods.to_vec(),
            synth,
            samples,
            len,
            pos: 0,
            row: 0,
            speed: 6,
            count: 1,
            arp: 3,
            rep: 1,
            ch: [ch.clone(), ch.clone(), ch.clone(), ch],
            out: Vec::new(),
        }
    }

    fn period(&self, note: i32) -> u16 {
        *self
            .periods
            .get((note - 1).clamp(0, self.periods.len() as i32 - 1) as usize)
            .unwrap_or(&428)
    }

    fn block(&self, p: usize, len_words: u16) -> Block {
        let n = (len_words as usize * 2).min(self.m.len().saturating_sub(p));
        let d: Vec<i8> = self.m[p..p + n].iter().map(|&b| b as i8).collect();
        Block {
            data: Arc::new(d),
            start: 0,
            len: n,
        }
    }

    fn set_per(&mut self, c: usize, p: u16) {
        if self.ch[c].hw_per != p {
            self.ch[c].hw_per = p;
            if p != 0 {
                self.out.push(Snd::Rate {
                    ch: c,
                    rate: CLOCK / p as f32,
                });
            }
        }
    }

    fn set_vol(&mut self, c: usize, v: u16) {
        if self.ch[c].hw_vol != v {
            self.ch[c].hw_vol = v;
            self.out.push(Snd::Vol {
                ch: c,
                vol: v.min(64) as u8,
            });
        }
    }

    fn set_next(&mut self, c: usize, p: Option<usize>, len: u16) {
        let k = p.map(|p| (p, len));
        if self.ch[c].hw_next != k || self.ch[c].wave.is_some() {
            self.ch[c].hw_next = k;
            let b = match p {
                Some(p) if len > 1 => self.block(p, len),
                _ => Block::silence(),
            };
            self.out.push(Snd::Next { ch: c, block: b });
        }
    }

    /// update_sprite_positions (+$100): one frame of the player.
    pub fn frame(&mut self) -> Vec<Snd> {
        for c in 0..4 {
            if self.ch[c].synth {
                self.synth_frame(c);
            }
        }
        self.arp = self.arp.wrapping_sub(1);
        for c in 0..4 {
            let s = self.ch[c].slide as i16;
            self.ch[c].period = (self.ch[c].period as i16).wrapping_add(s) as u16;
            if self.ch[c].lfo_ctl == 0 {
                let p = self.ch[c].period & 0x7FFF;
                self.set_per(c, p);
            }
            let (rp, rl) = (self.ch[c].rep_ptr, self.ch[c].rep_len);
            self.set_next(c, rp, rl);
            let (a1, a2, n) = (self.ch[c].arp1, self.ch[c].arp2, self.ch[c].note);
            if a1 != 0 || a2 != 0 {
                let note = match self.arp {
                    0 => (a1 >> 4) as i32 + (a2 >> 4) as i32 + n as i32,
                    1 => (a1 & 15) as i32 + (a2 & 15) as i32 + n as i32,
                    _ => n as i32,
                };
                let p = self.period(note as i8 as i32);
                self.set_per(c, p);
            }
        }
        if self.arp == 0 {
            self.arp = 3;
        }
        self.count = self.count.wrapping_sub(1);
        if self.count == 0 {
            self.count = self.speed;
            self.row_step();
            for c in 0..4 {
                if self.ch[c].period & 0x8000 != 0 {
                    self.start_note(c);
                }
            }
        }
        std::mem::take(&mut self.out)
    }

    /// process_input_events (+$230): the next row of each channel's pattern.
    fn row_step(&mut self) {
        for c in 0..4 {
            let pe = 0x200 + self.pos as usize * 16 + c * 4;
            let pat = w(&self.m, pe);
            let stran = self.m[pe + 2];
            let ntran = self.m[pe + 3];
            let base = 0x200
                + self.len as usize * 16
                + (pat as usize).wrapping_sub(1) * 0x30
                + self.row as usize;
            let (note, fx, par) = (
                *self.m.get(base).unwrap_or(&0),
                *self.m.get(base + 1).unwrap_or(&0),
                *self.m.get(base + 2).unwrap_or(&0),
            );
            let e = fx & 15;
            if note != 0 {
                self.ch[c].slide = 0;
                let mut n = note;
                if !(e == 0xA && par & 0xF0 != 0) {
                    n = n.wrapping_add(ntran);
                }
                self.ch[c].note = n;
                self.ch[c].period = self.period(n as i8 as i32) | 0x8000;
                self.ch[c].vol = 0xFF;
                let mut ins = fx >> 4;
                if ins == 0 {
                    ins = self.ch[c].instr;
                }
                if !(e == 0xA && par & 15 != 0) {
                    ins = ins.wrapping_add(stran);
                }
                self.ch[c].instr = ins;
            }
            match e {
                0 => self.ch[c].arp1 = par,
                1 => {
                    self.set_vol(c, par as u16);
                    self.ch[c].vol = par;
                }
                2 => {
                    self.count = par;
                    self.speed = par;
                }
                4 => {
                    self.ch[c].period = self.ch[c].period.wrapping_sub(par as u16);
                    self.ch[c].arp1 = 0;
                }
                5 => {
                    self.ch[c].period = self.ch[c].period.wrapping_add(par as u16);
                    self.ch[c].arp1 = 0;
                }
                6 => self.rep = par,
                7 => {
                    self.rep = self.rep.wrapping_sub(1);
                    if self.rep != 0 {
                        self.pos = par as u16;
                    }
                }
                8 => self.ch[c].slide = par as i8,
                9 => self.ch[c].arp2 = par,
                _ => {}
            }
        }
        self.row += 3;
        if self.row == 0x30 {
            self.row = 0;
            self.pos += 1;
            if self.pos >= self.len {
                self.pos = 0;
            }
        }
    }

    /// init_sprite_animation (+$44A): a note starts its instrument.
    fn start_note(&mut self, c: usize) {
        self.ch[c].period &= 0x7FFF;
        if let Some(wv) = self.ch[c].wave.take() {
            let b = self.ch[c].backup;
            self.m[wv..wv + 32].copy_from_slice(&b);
        }
        let per = self.ch[c].period;
        let ins = self.ch[c].instr as usize;
        let e = ins * 32;
        if ins == 0 || e + 32 > 0x200 {
            return;
        }
        let rate = CLOCK / per.max(1) as f32;
        self.ch[c].hw_per = per;
        if self.m[e] != 0xFF {
            // a sample
            self.ch[c].synth = false;
            self.ch[c].lfo_ctl = 0;
            let start = self.samples[ins - 1];
            let len = w(&self.m, e + 24);
            let mut vol = self.ch[c].vol as u16;
            if self.ch[c].vol == 0xFF {
                vol = w(&self.m, e + 30);
            }
            self.ch[c].rep_len = w(&self.m, e + 28);
            let ls = w(&self.m, e + 26) as usize;
            let (b, ptr) = if self.ch[c].rep_len == 1 {
                self.ch[c].rep_ptr = None;
                (self.block(start, len), None)
            } else {
                self.ch[c].rep_ptr = Some(start + ls);
                let rl = self.ch[c].rep_len;
                (self.block(start + ls, rl), Some((start + ls, rl)))
            };
            self.ch[c].hw_next = ptr;
            self.ch[c].hw_vol = vol;
            let next = if ptr.is_some() {
                b.clone()
            } else {
                Block::silence()
            };
            self.out.push(Snd::Start {
                ch: c,
                block: b,
                next,
                rate,
                vol: vol.min(64) as u8,
            });
            return;
        }
        // a synth sound
        let m = &self.m;
        let ch = &mut self.ch[c];
        ch.synth = true;
        ch.eg_pos = 0;
        ch.lfo_pos = 0;
        ch.adsr_pos = 0;
        ch.eg_cnt = w(m, e + 22) + 1;
        ch.lfo_cnt = w(m, e + 14) + 1;
        ch.adsr_cnt = 1;
        ch.eg_ctl = m[e + 17];
        ch.lfo_ctl = m[e + 9];
        ch.adsr_ctl = m[e + 4];
        ch.eg_cur = m[e + 19];
        let wave = self.synth + m[e + 1] as usize * 64;
        let wlen = w(m, e + 2);
        ch.rep_ptr = Some(wave);
        ch.rep_len = wlen;
        if ch.vol == 0xFF {
            ch.vol = m[e + 25];
        }
        let vol = if m[e + 4] != 0 {
            let t = self.synth + m[e + 5] as usize * 64;
            let d3 = (m[t].wrapping_add(0x80) >> 2) as u16;
            (d3 * ch.vol as u16) >> 6
        } else {
            ch.vol as u16
        };
        let mut backup = [0u8; 32];
        backup.copy_from_slice(&m[wave..wave + 32]);
        ch.backup = backup;
        ch.wave = Some(wave);
        ch.hw_vol = vol;
        ch.hw_next = Some((wave, wlen));
        if m[e + 17] != 0 && m[e + 19] != 0 {
            let n = (m[e + 19] >> 3) as usize;
            ch.eg_cur = n as u8;
            for k in 0..n.min(32) {
                self.m[wave + k] = (self.m[wave + k] as i8).wrapping_neg() as u8;
            }
        }
        let b = self.block(wave, wlen);
        self.out.push(Snd::Start {
            ch: c,
            block: b.clone(),
            next: b,
            rate,
            vol: vol.min(64) as u8,
        });
    }

    /// update_animation_frames_2 (+$64A): the synth sound's ADSR (volume), LFO (vibrato) and EG (waveform) steps.
    fn synth_frame(&mut self, c: usize) {
        let e = self.ch[c].instr as usize * 32;
        if e + 32 > 0x200 {
            return;
        }
        if self.ch[c].adsr_ctl != 0 {
            self.ch[c].adsr_cnt = self.ch[c].adsr_cnt.wrapping_sub(1);
            if self.ch[c].adsr_cnt == 0 {
                self.ch[c].adsr_cnt = self.m[e + 8] as u16;
                let t = self.synth + self.m[e + 5] as usize * 64;
                let v = *self.m.get(t + self.ch[c].adsr_pos as usize).unwrap_or(&0);
                let d4 = ((v.wrapping_add(0x80) >> 2) as u16 * self.ch[c].vol as u16) >> 6;
                self.set_vol(c, d4);
                self.ch[c].adsr_pos += 1;
                if self.ch[c].adsr_pos == w(&self.m, e + 6) {
                    self.ch[c].adsr_pos = 0;
                    if self.ch[c].adsr_ctl == 1 {
                        self.ch[c].adsr_ctl = 0;
                    }
                }
            }
        }
        if self.ch[c].lfo_ctl != 0 {
            self.ch[c].lfo_cnt = self.ch[c].lfo_cnt.wrapping_sub(1);
            if self.ch[c].lfo_cnt == 0 {
                self.ch[c].lfo_cnt = self.m[e + 16] as u16;
                let t = self.synth + self.m[e + 10] as usize * 64;
                let mut d4 =
                    *self.m.get(t + self.ch[c].lfo_pos as usize).unwrap_or(&0) as i8 as i32;
                let depth = self.m[e + 11] as i32;
                if depth != 0 {
                    d4 /= depth;
                }
                let p = (self.ch[c].period as i32 + d4) as u16;
                self.set_per(c, p);
                self.ch[c].lfo_pos += 1;
                if self.ch[c].lfo_pos == w(&self.m, e + 12) {
                    self.ch[c].lfo_pos = 0;
                    if self.ch[c].lfo_ctl == 1 {
                        self.ch[c].lfo_ctl = 0;
                    }
                }
            }
        }
        if self.ch[c].eg_ctl != 0 {
            let Some(wave) = self.ch[c].wave else { return };
            self.ch[c].eg_cnt = self.ch[c].eg_cnt.wrapping_sub(1);
            if self.ch[c].eg_cnt == 0 {
                self.ch[c].eg_cnt = self.m[e + 24] as u16;
                let t = self.synth + self.m[e + 18] as usize * 64;
                let v = *self.m.get(t + self.ch[c].eg_pos as usize).unwrap_or(&0);
                let new = (v.wrapping_add(0x80) >> 3) as usize;
                let old = self.ch[c].eg_cur as usize;
                self.ch[c].eg_cur = new as u8;
                let bk = self.ch[c].backup;
                let (lo, hi) = (old.min(new).min(32), old.max(new).min(32));
                let dst = &mut self.m[wave + lo..wave + hi];
                if new > old {
                    for (d, b) in dst.iter_mut().zip(&bk[lo..hi]) {
                        *d = (*b as i8).wrapping_neg() as u8;
                    }
                } else {
                    dst.copy_from_slice(&bk[lo..hi]);
                }
                self.ch[c].eg_pos += 1;
                if self.ch[c].eg_pos == w(&self.m, e + 20) {
                    self.ch[c].eg_pos = 0;
                    if self.ch[c].eg_ctl == 1 {
                        self.ch[c].eg_ctl = 0;
                    }
                }
            }
        }
    }

    /// disable_interrupt_server (+$22): the music stops.
    pub fn stop(&mut self) -> Vec<Snd> {
        (0..4).map(Snd::Stop).collect()
    }
}
