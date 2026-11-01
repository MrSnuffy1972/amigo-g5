//! The title music's player (music_frame_all_channels, $D284, and music_channel_sequencer, $D308), rewritten on
//! typed channel records. The song is the original's data: per channel a list of note streams; a stream is words:
//! a period over 100 followed by a length in frames, or a command (instrument, next stream, volume envelope,
//! slide, vibrato, rest). Pointers in it are the original's addresses, read from the music block (`base`).
//! Each channel's output is what the original wrote to its sound channel: sample, period, volume, on and off.

use std::sync::Arc;

use native_engine::audio::Block;

use crate::assets::Assets;
use crate::co::Snd;

/// One channel record (62 bytes at d0_CFD4, d0_D012, d0_D050, d0_D08E).
#[derive(Default, Clone, Copy, Debug)]
struct Chan {
    stopped: bool,      // +0
    dur: u16,           // +2
    mode: u16,          // +4 0 = one sample; 3 = attack pending; 1 = attack playing
    period: u16,        // +6
    slide: u16,         // +8 1 = up, other = down, 0 = off
    slide_step: u16,    // +10
    slide_to: u16,      // +12
    slide_wait: u16,    // +14
    vib: u16,           // +16 0 = off, 1..4 the phase
    env: u32,           // +18 volume envelope
    stream: u32,        // +22
    list: u32,          // +26
    list_start: u32,    // +30
    env_start: u32,     // +34
    attack: (u32, u16), // +38, +42
    looped: (u32, u16), // +44, +48
    vib_n: u16,         // +50
    vib_reload: u16,    // +52
    vib_up: u16,        // +54
    vib_down: u16,      // +56
    dma: bool,
    loc: (u32, u16),
}

pub struct Music {
    a: Arc<Assets>,
    ch: [Chan; 4],
    init: [Chan; 4],
}

impl Music {
    pub fn new(a: Arc<Assets>) -> Music {
        let mut m = Music {
            a,
            ch: [Chan::default(); 4],
            init: [Chan::default(); 4],
        };
        for i in 0..4 {
            let b = m.a.music_channels[i];
            let c = Chan {
                stopped: m.w(b) != 0,
                dur: m.w(b + 2),
                mode: m.w(b + 4),
                period: m.w(b + 6),
                env: m.l(b + 18),
                stream: m.l(b + 22),
                list: m.l(b + 26),
                list_start: m.l(b + 30),
                env_start: m.l(b + 34),
                ..Default::default()
            };
            m.init[i] = c;
        }
        m.ch = m.init;
        m
    }

    fn byte(&self, addr: u32) -> u8 {
        let o = addr.wrapping_sub(self.a.music_base) as usize;
        if let Some(b) = self.a.music.get(o) {
            return *b;
        }
        let o = addr.wrapping_sub(self.a.sfx_base) as usize;
        self.a.sfx_bytes.get(o).copied().unwrap_or(0)
    }
    fn w(&self, addr: u32) -> u16 {
        ((self.byte(addr) as u16) << 8) | self.byte(addr + 1) as u16
    }
    fn l(&self, addr: u32) -> u32 {
        ((self.w(addr) as u32) << 16) | self.w(addr + 2) as u32
    }

    fn block(&self, (addr, len): (u32, u16)) -> Block {
        let n = len as usize * 2;
        let v: Vec<i8> = (0..n as u32).map(|k| self.byte(addr + k) as i8).collect();
        Block {
            data: Arc::new(v),
            start: 0,
            len: n,
        }
    }

    /// The song from its start (set_global_flags_2, $0DBE).
    pub fn start(&mut self) {
        self.ch = self.init;
    }

    /// reset_audio_dma_interrupts ($0D68): all four channels off.
    pub fn stop(&mut self, out: &mut Vec<Snd>) {
        for i in 0..4 {
            self.ch[i].dma = false;
            out.push(Snd::Stop(i));
        }
    }

    /// music_frame_all_channels ($D284): one frame of the four channels. While an effect plays on channel 3 the
    /// music's fourth channel runs but is not heard.
    pub fn frame(&mut self, volume: u16, sfx_busy: bool, out: &mut Vec<Snd>) {
        for i in 0..4 {
            let mut o = Vec::new();
            self.sequencer(i, volume, &mut o);
            if !(i == 3 && sfx_busy) {
                out.extend(o);
            }
        }
    }

    fn dma_on(&mut self, i: usize, o: &mut Vec<Snd>) {
        let c = &mut self.ch[i];
        if c.dma {
            return;
        }
        c.dma = true;
        let (b, p) = (c.loc, c.period);
        let blk = self.block(b);
        o.push(Snd::Start {
            ch: i,
            block: blk.clone(),
            next: blk,
            period: p,
            volume: 0,
        });
    }

    fn set_loc(&mut self, i: usize, loc: (u32, u16), o: &mut Vec<Snd>) {
        self.ch[i].loc = loc;
        if self.ch[i].dma {
            let b = self.block(loc);
            o.push(Snd::Next(i, b));
        }
    }

    fn sequencer(&mut self, i: usize, volume: u16, o: &mut Vec<Snd>) {
        if self.ch[i].stopped {
            return;
        }
        let c = self.ch[i];
        if c.dur == 0 {
            if !self.next_note(i, o) {
                return;
            }
            self.envelope(i, volume, o);
            return;
        }
        if c.dur == 1 {
            if c.mode != 0 {
                self.ch[i].dma = false;
                o.push(Snd::Stop(i));
            }
            self.ch[i].dur -= 1;
        } else if c.mode == 3 {
            let l = c.looped;
            self.set_loc(i, l, o);
            self.ch[i].dur -= 1;
            self.ch[i].mode = 1;
        } else {
            self.ch[i].dur -= 1;
        }
        // l0_CD34: slide and vibrato
        let mut changed = false;
        let c = &mut self.ch[i];
        let mut d2 = c.period;
        if c.slide != 0 {
            if c.slide_wait != 0 {
                c.slide_wait -= 1;
            } else {
                changed = true;
                if c.slide == 1 {
                    d2 = d2.wrapping_add(c.slide_step);
                    if c.slide_to < d2 {
                        d2 = c.slide_to;
                        c.slide = 0;
                    }
                } else {
                    d2 = d2.wrapping_sub(c.slide_step);
                    if c.slide_to >= d2 {
                        d2 = c.slide_to;
                        c.slide = 0;
                    }
                }
            }
        }
        if c.vib != 0 {
            if c.vib_n != 0 {
                c.vib_n -= 1;
            } else {
                c.vib_n = c.vib_reload;
                changed = true;
                if c.vib < 3 {
                    d2 = d2.wrapping_sub(c.vib_down);
                    c.vib += 1;
                } else {
                    d2 = d2.wrapping_add(c.vib_up);
                    c.vib += 1;
                    if c.vib == 5 {
                        c.vib = 1;
                    }
                }
            }
        }
        if changed {
            c.period = d2;
            o.push(Snd::Period(i, d2));
        }
        self.envelope(i, volume, o);
    }

    /// l0_CDF0: the volume envelope's next value (the last one holds at $FF).
    fn envelope(&mut self, i: usize, volume: u16, o: &mut Vec<Snd>) {
        let a = self.ch[i].env;
        let mut d0 = self.w(a.wrapping_sub(2));
        let mut a1 = a;
        if self.w(a) != 0xff {
            d0 = self.w(a);
            a1 = a + 2;
        }
        let v = ((d0 as u32 * volume as u32) >> 4) as u8 & 0x7f;
        o.push(Snd::Volume(i, v.min(64)));
        self.ch[i].env = a1;
    }

    /// l0_CE48: reads the stream until a note (or a rest, or the end).
    fn next_note(&mut self, i: usize, o: &mut Vec<Snd>) -> bool {
        let mut a1 = self.ch[i].stream;
        for _ in 0..256 {
            let d0 = self.w(a1);
            a1 += 2;
            if d0 > 0x64 {
                let c = &mut self.ch[i];
                c.period = d0;
                o.push(Snd::Period(i, d0));
                let d = self.w(a1);
                a1 += 2;
                let c = &mut self.ch[i];
                c.dur = d.wrapping_sub(1);
                c.stream = a1;
                c.env = c.env_start;
                if c.vib != 0 {
                    c.vib = 1;
                }
                if c.mode != 0 {
                    c.mode = 3;
                    let at = c.attack;
                    self.set_loc(i, at, o);
                }
                self.dma_on(i, o);
                return true;
            }
            match d0 {
                0 => return false,
                4 => {
                    let a3 = self.l(a1);
                    a1 += 4;
                    let m = self.w(a3);
                    if m == 0 {
                        self.ch[i].mode = 0;
                        let loc = (self.l(a3 + 2), self.w(a3 + 6));
                        self.set_loc(i, loc, o);
                    } else {
                        let at = (self.l(a3 + 2), self.w(a3 + 6));
                        let lp = (self.l(a3 + 8), self.w(a3 + 12));
                        let c = &mut self.ch[i];
                        c.mode = m;
                        c.attack = at;
                        c.looped = lp;
                    }
                }
                8 => {
                    let lp = self.ch[i].list;
                    if self.l(lp) == 0 {
                        let s = self.ch[i].list_start;
                        a1 = self.l(s);
                        self.ch[i].list = s + 4;
                    } else {
                        a1 = self.l(lp);
                        self.ch[i].list = lp + 4;
                    }
                }
                0xc => {
                    self.ch[i].env_start = self.l(a1);
                    a1 += 4;
                }
                0x10 => {
                    let (s, st, to, wt) =
                        (self.w(a1), self.w(a1 + 2), self.w(a1 + 4), self.w(a1 + 6));
                    a1 += 8;
                    let c = &mut self.ch[i];
                    c.slide = s;
                    c.slide_step = st;
                    c.slide_to = to;
                    c.slide_wait = wt;
                }
                0x14 => {
                    let v: Vec<u16> = (0..6).map(|k| self.w(a1 + 2 * k)).collect();
                    a1 += 12;
                    let c = &mut self.ch[i];
                    c.vib = 1;
                    c.vib_n = v[0];
                    c.vib_reload = v[1];
                    c.vib_up = v[2];
                    c.vib_down = v[3];
                }
                0x18 => self.ch[i].slide = 0,
                0x1c => self.ch[i].vib = 0,
                0x20 => {
                    let d = self.w(a1);
                    a1 += 2;
                    let c = &mut self.ch[i];
                    c.dur = d.wrapping_sub(1);
                    c.stream = a1;
                    o.push(Snd::Volume(i, 0));
                    c.env = 0;
                    return false;
                }
                _ => {}
            }
        }
        false
    }
}
