//! The clean-sheet sound: the game's four Paula channels played from chip RAM at their periods, mixed to 44.1 kHz
//! stereo once per line, with the A500 or A1200 output stage. No DMA slots, no cycle model: each channel steps through
//! its sample block by colour clocks, as the C runtime does.
//!
//! Ported from `kit/amigo/port/runtime/intentrt.c` (audio_dmacon, aud_start, chan_step, audio_line, paula_setup, and
//! the AUDx cases of write_custom; M5 part 3, 28 Oct 2026), which is the oracle: `tests/paula_diff.rs` compiles that
//! part of intentrt.c with gcc and compares every output sample, every interrupt and the filter state bit for bit.
//! Float arithmetic follows the C's order and widths (f32 where the C has float, f64 where it has double) so the two
//! round alike.
//!
//! The band-limited step table and the RC filters below are a Rust port of `runtime/paula/pt2_blep.c` and
//! `pt2_rcfilters.c`, vendored by the C kit from pt2-clone (13 Oct 2026, Karl: reuse, don't reinvent). Their licence:
//!
//! > Copyright (c) 2010-2026, Olav Sørensen. All rights reserved. BSD 3-Clause License: redistribution and use in
//! > source and binary forms, with or without modification, are permitted provided that the following conditions are
//! > met: (1) redistributions of source code must retain the above copyright notice, this list of conditions and the
//! > following disclaimer; (2) redistributions in binary form must reproduce the above copyright notice, this list of
//! > conditions and the following disclaimer in the documentation and/or other materials provided with the
//! > distribution; (3) neither the name of the copyright holder nor the names of its contributors may be used to
//! > endorse or promote products derived from this software without specific prior written permission. THIS SOFTWARE
//! > IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING,
//! > BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! > DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
//! > INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
//! > SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
//! > THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
//! > IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//!
//! (The full text is `kit/amigo/port/runtime/paula/LICENSE-pt2-clone`.)

use std::f64::consts::PI;
use std::io::{self, Write};

use crate::display::Chip;

/// Output sample rate (main.c `--wav`, judge.py reads 882 samples per 50 Hz tick).
pub const SAMPLE_RATE: u32 = 44100;
const CC_PER_SAMPLE: f64 = 3546895.0 / 44100.0;
/// A PAL line in colour clocks.
const LINE_CC: f64 = 227.5;

/// The machine whose output stage is copied (intentrt.c paula_mode; the C reads AMIGO_PAULA, the product will take it
/// from the game's menu as paula_set_model does).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Model {
    /// No filters and no BLEP: each channel's held sample as is ("raw").
    Raw,
    /// A500 rev 6A: 4.4 kHz low-pass, 5.1 Hz high-pass, LED filter (the C default).
    #[default]
    A500,
    /// A1200 rev 1D4: 5.3 Hz high-pass and the LED filter only.
    A1200,
}

/// One channel's registers as the game last wrote them (amiga.h aud_t).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Voice {
    /// AUDxLC, the sample block's start (always even).
    pub lc: u32,
    /// AUDxLEN in words (0 plays 65536).
    pub len: u16,
    /// AUDxPER in colour clocks per byte (below 124 plays at 124).
    pub per: u16,
    /// AUDxVOL (above 64 plays at 64).
    pub vol: u16,
    pub dat: u16,
}

/// Where a channel is in its block (intentrt.c chan_t). `ptr` is kept unmasked: the block start is even, so masking
/// at each read (Chip::r16) gives the C's CHIP_WMASK-wrapped address.
#[derive(Debug, Clone, Copy, Default)]
struct Chan {
    ptr: u32,
    left: u32,
    word: u16,
    bytei: u8,
    phase: f64,
    cur: i8,
    stepped: bool,
}

/// pt2_blep.h: 16 zero crossings, 16 times oversampled, inserted at full rate (32-sample ring).
const BLEP_SP: usize = 16;
const BLEP_NS: usize = 16;
const BLEP_RNS: usize = 31;

/// pt2_blep.c fMinBlepData, copied digit for digit (so the f32 rounding of each literal is the C's; the test checks
/// the bits against the compiled C table).
#[allow(clippy::excessive_precision)]
#[rustfmt::skip]
const MIN_BLEP: [f32; 257] = [
    1.0000477302613517416, 1.0000703265259194286, 1.0000262954869634235, 0.9999104247733368034,
    0.9997157443790558595, 0.9994330149197339086, 0.9990500857713285887, 0.9985511219195251087,
    0.9979157062335919370, 0.9971178326926340985, 0.9961248154952055955, 0.9948961485703640140,
    0.9933823593234317739, 0.9915239090030570912, 0.9892501993644792213, 0.9864787508331824828,
    0.9831146206825892575, 0.9790501304255075921, 0.9741649693586746928, 0.9683267357717054713,
    0.9613919686347883742, 0.9532077106463556770, 0.9436136285285890990, 0.9324446987272798637,
    0.9195344466691151020, 0.9047187060538732783, 0.8878398420296869098, 0.8687513593312519156,
    0.8473227944375107956, 0.8234447704476933749, 0.7970340756049164588, 0.7680386121007229949,
    0.7364420517831927748, 0.7022680303646210431, 0.6655837122341694556, 0.6265025644004150740,
    0.5851861905894381044, 0.5418450950558918455, 0.4967382699459244044, 0.4501715295677637396,
    0.4024945489393364495, 0.3540966015460170202, 0.3054010312278475636, 0.2568585342499346558,
    0.2089393685130542522, 0.1621246460974391512, 0.1168969014596899919, 0.0737301592279361734,
    0.0330797513739862215, -0.0046278475512338936, -0.0390048873823494666, -0.0697116292604941790,
    -0.0964647767093624875, -0.1190447905608251339, -0.1373018517595622767, -0.1511602687179081639,
    -0.1606211659994899177, -0.1657633375556412103, -0.1667421991415036220, -0.1637868293095470773,
    -0.1571951447710946692, -0.1473273120888395071, -0.1345975517409976063, -0.1194645407415074190,
    -0.1024206644738059890, -0.0839804056280038791, -0.0646681867786920578, -0.0450060021366877130,
    -0.0255011826063778063, -0.0066346360852734603, 0.0111501080726254947, 0.0274567449958385452,
    0.0419446584514933316, 0.0543357729883130469, 0.0644196131370170927, 0.0720564437641972172,
    0.0771784246024087850, 0.0797887728449645922, 0.0799589884682101459, 0.0778242556005649261,
    0.0735771878467293278, 0.0674601341940760518, 0.0597563033841086166, 0.0507799970999210509,
    0.0408662649895026181, 0.0303603067514581562, 0.0196069479612341578, 0.0089405070970495753,
    -0.0013246482081264665, -0.0109025865007772501, -0.0195431073564850052, -0.0270376576677117432,
    -0.0332237305394043891, -0.0379876606806542061, -0.0412657844962587075, -0.0430439870972044586,
    -0.0433557103124065854, -0.0422785437568130862, -0.0399295635284086167, -0.0364596188316990630,
    -0.0320467946929081995, -0.0268892981827763312, -0.0211980257636115339, -0.0151890704304555764,
    -0.0090764194553641132, -0.0030650773168921556, 0.0026551753615487940, 0.0079152062478091562,
    0.0125709938660189587, 0.0165070221469508817, 0.0196385421289703745, 0.0219126779344609758,
    0.0233083952284523256, 0.0238353891250968619, 0.0235319836335969659, 0.0224621650980049159,
    0.0207118967713227006, 0.0183848800034730822, 0.0155979391055239645, 0.0124762116364726186,
    0.0091483237641666864, 0.0057417218473517556, 0.0023783170654907890, -0.0008294194250053805,
    -0.0037817967606831484, -0.0063945924755681527, -0.0086010292023157020, -0.0103530098334944001,
    -0.0116216097291982345, -0.0123968518397700699, -0.0126868154975107086, -0.0125161512979873567,
    -0.0119240922816280862, -0.0109620650623401504, -0.0096910133459421205, -0.0081785503450736048,
    -0.0064960560250743584, -0.0047158301788018369, -0.0029084034555543611, -0.0011400962200064214,
    0.0005290998458667121, 0.0020472594272570623, 0.0033718407258998130, 0.0044705608305281395,
    0.0053218263185221635, 0.0059147333317129914, 0.0062486669637696907, 0.0063325432361187482,
    0.0061837478235316776, 0.0058268337450093155, 0.0052920453161976388, 0.0046137377508039690,
    0.0038287610068399431, 0.0029748730160543677, 0.0020892416238459640, 0.0012070867911180888,
    0.0003605053137761188, -0.0004224900212018401, -0.0011186958109406235, -0.0017101978657877311,
    -0.0021846059849888523, -0.0025350539493808797, -0.0027599836404749540, -0.0028627394193863481,
    -0.0028510045105526855, -0.0027361150176634062, -0.0025322893172763906, -0.0022558109708829808,
    -0.0019242020804577712, -0.0015554213581010150, -0.0011671173084782766, -0.0007759620906248776,
    -0.0003970861128210880, -0.0000436275087427707, 0.0002735953716143057, 0.0005462861799454866,
    0.0007687506805363838, 0.0009378627976334288, 0.0010529287404153116, 0.0011154645668973284,
    0.0011289048235580167, 0.0010982612080436753, 0.0010297505723515206, 0.0009304110777853725,
    0.0008077240290279897, 0.0006692569780186192, 0.0005223412430787977, 0.0003737941898757299,
    0.0002296936262087236, 0.0000952086254601105, -0.0000255118442794698, -0.0001293938220771447,
    -0.0002144405097769575, -0.0002796873836864508, -0.0003251174847873964, -0.0003515451441525616,
    -0.0003604769473845936, -0.0003539588239428851, -0.0003344178062763300, -0.0003045062980902643,
    -0.0002669556911830751, -0.0002244449539125465, -0.0001794884622946999, -0.0001343459365493336,
    -0.0000909559560488899, -0.0000508932202670833, -0.0000153485577887860, 0.0000148702975205883,
    0.0000393199152742675, 0.0000578876582698600, 0.0000707470635261192, 0.0000783058195432098,
    0.0000811499393448619, 0.0000799874519480083, 0.0000755945206110041, 0.0000687663822549972,
    0.0000602749277155524, 0.0000508341447950342, 0.0000410740603062377, 0.0000315232737098524,
    0.0000225996980950096, 0.0000146087321789519, 0.0000077477909467512, 0.0000021159270461592,
    -0.0000022728216146247, -0.0000054737276186147, -0.0000075946076493092, -0.0000087791482829482,
    -0.0000091912899144718, -0.0000090014159571477, -0.0000083748626165846, -0.0000074630357144956,
    -0.0000063972090792165, -0.0000052848949776790, -0.0000042085288174683, -0.0000032261024680931,
    -0.0000023733143901201, -0.0000016667787374929, -0.0000011078456395590, -0.0000006866249747596,
    -0.0000003858688186494, -0.0000001844454404328, -0.0000000602222727236, 0.0000000077407240470,
    0.0000000377088328857, 0.0000000444579428691, 0.0000000390342963101, 0.0000000289629328710,
    0.0000000187639946982, 0.0000000106369370086, 0.0000000051870995047, 0.0000000020936704675,
    0.0000000006489518121, 0.0000000001320180639, 0.0000000000115913357, 0.0000000000000000000,
    0.0000000000000000000,
];

/// pt2_blep.c blep_t.
#[derive(Debug, Clone, Copy, Default)]
struct Blep {
    index: usize,
    samples_left: i32,
    buf: [f32; BLEP_RNS + 1],
}

impl Blep {
    /// pt2_blep.c blepAdd. `offset` is in [0, 1).
    fn add(&mut self, offset: f32, amplitude: f32) {
        let mut f = offset * BLEP_SP as f32;
        let fi = f as usize;
        f -= fi as f32;
        let mut src = fi;
        let mut i = self.index;
        for _ in 0..BLEP_NS {
            let (p1, p2) = (MIN_BLEP[src], MIN_BLEP[src + 1]);
            self.buf[i] += amplitude * (p1 + (p2 - p1) * f);
            src += BLEP_SP;
            i = (i + 1) & BLEP_RNS;
        }
        self.samples_left = BLEP_NS as i32;
    }
    /// pt2_blep.c blepRun.
    fn run(&mut self, input: f32) -> f32 {
        let out = input + self.buf[self.index];
        self.buf[self.index] = 0.0;
        self.index = (self.index + 1) & BLEP_RNS;
        self.samples_left -= 1;
        out
    }
}

/// pt2_rcfilters.c onePoleFilter_t: a 1-pole RC filter, used as low-pass or high-pass.
#[derive(Debug, Clone, Copy, Default)]
struct OnePole {
    tmp: [f32; 2],
    a0: f32,
    b1: f32,
}

/// pt2_rcfilters.c SMALL_NUMBER.
const SMALL: f64 = 1e-4;

impl OnePole {
    /// pt2_rcfilters.c setupOnePoleFilter (coefficients only; the state is kept, as in the C).
    fn setup(&mut self, rate: f64, cutoff: f64) {
        let cutoff = if cutoff >= rate / 2.0 {
            rate / 2.0 - SMALL
        } else {
            cutoff
        };
        let b1 = ((-2.0 * PI) * cutoff / rate).exp();
        self.b1 = b1 as f32;
        self.a0 = (1.0 - b1) as f32;
    }
    /// pt2_rcfilters.c onePoleLPFilterStereo.
    fn low_pass(&mut self, io: &mut [f32; 2]) {
        for (x, t) in io.iter_mut().zip(&mut self.tmp) {
            *t = *x * self.a0 + *t * self.b1;
            *x = *t;
        }
    }
    /// pt2_rcfilters.c onePoleHPFilterStereo.
    fn high_pass(&mut self, io: &mut [f32; 2]) {
        for (x, t) in io.iter_mut().zip(&mut self.tmp) {
            *t = *x * self.a0 + *t * self.b1;
            *x -= *t;
        }
    }
}

/// pt2_rcfilters.c twoPoleFilter_t: the 2-pole Sallen-Key low-pass (the "LED" filter).
#[derive(Debug, Clone, Copy, Default)]
struct TwoPole {
    tmp: [[f32; 4]; 2],
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
}

impl TwoPole {
    /// pt2_rcfilters.c setupTwoPoleFilter.
    fn setup(&mut self, rate: f64, cutoff: f64, q: f64) {
        let cutoff = if cutoff >= rate / 2.0 {
            rate / 2.0 - SMALL
        } else {
            cutoff
        };
        let a = 1.0 / ((PI * cutoff) / rate).tan();
        let r = 1.0 / q;
        let a1 = 1.0 / (1.0 + r * a + a * a);
        let a2 = 2.0 * a1;
        let b1 = 2.0 * (1.0 - a * a) * a1;
        let b2 = (1.0 - r * a + a * a) * a1;
        self.a1 = a1 as f32;
        self.a2 = a2 as f32;
        self.b1 = b1 as f32;
        self.b2 = b2 as f32;
    }
    /// pt2_rcfilters.c twoPoleLPFilterStereo.
    fn low_pass(&mut self, io: &mut [f32; 2]) {
        for (x, t) in io.iter_mut().zip(&mut self.tmp) {
            let out = (*x * self.a1) + (t[0] * self.a2) + (t[1] * self.a1)
                - (t[2] * self.b1)
                - (t[3] * self.b2);
            t[1] = t[0];
            t[0] = *x;
            t[3] = t[2];
            t[2] = out;
            *x = out;
        }
    }
}

/// True when the LED filter is in: CIA-A PRA bit 1 driven low (intentrt.c audio_line `!(pra & ddra & 2)`).
pub fn led_filter_on(cia_a_pra: u8, cia_a_ddra: u8) -> bool {
    cia_a_pra & cia_a_ddra & 2 == 0
}

/// Paula as the game drives it. No global state: the runtime owns one of these.
#[derive(Debug, Clone)]
pub struct Paula {
    voice: [Voice; 4],
    on: [bool; 4],
    irq_pending: u8,
    start_pending: u8,
    defer_start: bool,
    ch: [Chan; 4],
    blep: [Blep; 4],
    last: [f32; 4],
    lo: OnePole,
    hi: OnePole,
    led: TwoPole,
    model: Model,
    out_frac: f64,
}

impl Default for Paula {
    /// The C's defaults: A500 output stage, channel start at the DMACON write (AMIGO_AUD_DEFER 0).
    fn default() -> Self {
        Paula::new(Model::default(), false)
    }
}

impl Paula {
    /// `defer_start` is the C's g47 switch (AMIGO_AUD_DEFER): a channel switched on by DMACON takes its block start
    /// and length at the next line, not at the DMACON write.
    pub fn new(model: Model, defer_start: bool) -> Self {
        let mut p = Paula {
            voice: [Voice::default(); 4],
            on: [false; 4],
            irq_pending: 0,
            start_pending: 0,
            defer_start,
            ch: [Chan::default(); 4],
            blep: [Blep::default(); 4],
            last: [0.0; 4],
            lo: OnePole::default(),
            hi: OnePole::default(),
            led: TwoPole::default(),
            model,
            out_frac: 0.0,
        };
        p.set_model(model);
        p
    }

    /// intentrt.c paula_set_model and paula_setup: the output stage's filter coefficients, from the schematics'
    /// component values. Filter state is kept, as in the C.
    pub fn set_model(&mut self, model: Model) {
        self.model = model;
        let fs = SAMPLE_RATE as f64;
        if model == Model::A500 {
            self.lo.setup(fs, 1.0 / (2.0 * PI * 360.0 * 1e-7)); // R321 360 ohm, C321 0.1 uF: about 4421 Hz
            self.hi.setup(fs, 1.0 / (2.0 * PI * 1390.0 * 2.233e-5)); // R324+R325, C334+C335: about 5.13 Hz
        } else {
            self.hi.setup(fs, 1.0 / (2.0 * PI * 1360.0 * 2.2e-5)); // A1200: about 5.32 Hz, high-pass only
        }
        // LED filter, 2-pole Sallen-Key: R322 = R323 = 10K, C322 6800 pF, C323 3900 pF: about 3091 Hz, Q about 0.660
        let (r1, r2, c1, c2) = (10000.0f64, 10000.0f64, 6.8e-9f64, 3.9e-9f64);
        let rc = (r1 * r2 * c1 * c2).sqrt();
        self.led
            .setup(fs, 1.0 / (2.0 * PI * rc), rc / (c2 * (r1 + r2)));
    }

    pub fn model(&self) -> Model {
        self.model
    }

    /// Channel `n`'s registers.
    pub fn voice(&self, n: usize) -> Voice {
        self.voice[n]
    }

    /// A game write to $0A0-$0DF (intentrt.c write_custom, the AUDx cases). `dmacon` is DMACON after any earlier
    /// write. Returns the INTREQ bits to raise: AUDxDAT written with the channel's DMA off asks for the next word at
    /// once.
    pub fn write(&mut self, reg: u16, v: u16, dmacon: u16) -> u16 {
        let reg = reg & 0x1fe;
        if !(0x0a0..0x0e0).contains(&reg) {
            return 0;
        }
        let n = ((reg - 0xa0) >> 4) as usize;
        let a = &mut self.voice[n];
        match reg & 0xf {
            0x0 => a.lc = (a.lc & 0xffff) | (v as u32) << 16,
            0x2 => a.lc = (a.lc & 0xffff_0000) | (v & 0xfffe) as u32,
            0x4 => a.len = v,
            0x6 => a.per = v,
            0x8 => a.vol = v,
            0xa => {
                a.dat = v;
                if !(dmacon & 0x200 != 0 && dmacon & (1 << n) != 0) {
                    return 0x80 << n;
                }
            }
            _ => {}
        }
        0
    }

    /// intentrt.c audio_dmacon: DMACON went from `old` to `now`. A channel switched on starts its block (now, or at the
    /// next line when deferred) and asks for an interrupt at the end of the line; one switched off falls silent.
    pub fn dma_changed(&mut self, old: u16, now: u16) {
        for n in 0..4 {
            let was = old & 0x200 != 0 && old & (1 << n) != 0;
            let is = now & 0x200 != 0 && now & (1 << n) != 0;
            if is && !was {
                self.on[n] = true;
                if self.defer_start {
                    self.start_pending |= 1 << n;
                } else {
                    self.start(n);
                }
                self.irq_pending |= 1 << n;
            } else if !is && was {
                self.on[n] = false;
                self.start_pending &= !(1 << n);
                self.ch[n].cur = 0; // aud_stop
            }
        }
    }

    /// intentrt.c aud_start.
    fn start(&mut self, n: usize) {
        let (a, c) = (&self.voice[n], &mut self.ch[n]);
        c.ptr = a.lc;
        c.left = if a.len != 0 { a.len as u32 } else { 65536 };
        c.bytei = 2;
        c.phase = 0.0;
    }

    /// intentrt.c chan_step: advance channel `n` by `cc` colour clocks. Returns the INTREQ bit when its block ended
    /// and restarted.
    fn step(&mut self, chip: Chip, n: usize, cc: f64) -> u16 {
        let (a, c) = (&self.voice[n], &mut self.ch[n]);
        let per = a.per.max(124) as f64;
        let mut irq = 0;
        c.phase += cc;
        while c.phase >= per {
            c.phase -= per;
            if c.bytei >= 2 {
                if c.left == 0 {
                    c.ptr = a.lc;
                    c.left = if a.len != 0 { a.len as u32 } else { 65536 };
                    irq = 0x80 << n;
                }
                c.word = chip.r16(c.ptr);
                c.ptr = c.ptr.wrapping_add(2);
                c.left -= 1;
                c.bytei = 0;
            }
            c.cur = (if c.bytei == 0 { c.word >> 8 } else { c.word }) as u8 as i8;
            c.bytei += 1;
            c.stepped = true;
        }
        irq
    }

    /// intentrt.c audio_line: play one PAL line (227.5 colour clocks), appending its 44.1 kHz stereo samples (left,
    /// right) to `out`. `led` is the LED filter switch (see `led_filter_on`). Returns the INTREQ bits raised (a block
    /// that ended and restarted, and channels switched on since the last line).
    pub fn play_line(&mut self, chip: Chip, led: bool, out: &mut Vec<i16>) -> u16 {
        let mut irq = 0u16;
        let mut t = 0.0f64;
        for n in 0..4 {
            if self.start_pending & (1 << n) != 0 {
                self.start(n);
            }
        }
        self.start_pending = 0;
        loop {
            let next = (1.0 - self.out_frac) * CC_PER_SAMPLE;
            if t + next > LINE_CC {
                self.out_frac += (LINE_CC - t) / CC_PER_SAMPLE;
                break;
            }
            for n in 0..4 {
                if self.on[n] {
                    irq |= self.step(chip, n, next);
                }
            }
            t += next;
            self.out_frac = 0.0;
            let io = self.mix_sample(led);
            out.extend(io);
        }
        for n in 0..4 {
            if self.on[n] {
                irq |= self.step(chip, n, LINE_CC - t);
            }
        }
        for n in 0..4 {
            if self.irq_pending & (1 << n) != 0 {
                irq |= 0x80 << n;
            }
        }
        self.irq_pending = 0;
        irq
    }

    /// One output sample from the channels' held values: BLEP at each step, channels 0 and 3 left, 1 and 2 right, then
    /// the output stage (the inner part of intentrt.c audio_line).
    fn mix_sample(&mut self, led: bool) -> [i16; 2] {
        let mut v = [0.0f32; 4];
        for (n, out) in v.iter_mut().enumerate() {
            let vol = self.voice[n].vol.min(64) as i32;
            let val = if self.on[n] {
                (self.ch[n].cur as i32 * vol) as f32
            } else {
                0.0
            };
            if self.model == Model::Raw {
                *out = val;
                self.ch[n].stepped = false;
                continue;
            }
            if val != self.last[n] {
                // the step happened `phase` colour clocks ago (or now, for a volume or DMA change)
                let off = if self.ch[n].stepped {
                    (self.ch[n].phase / CC_PER_SAMPLE) as f32
                } else {
                    0.0
                };
                // the C's two tests, not a clamp: an offset in [0.999, 1) is kept as it is
                let off = if off >= 1.0 {
                    0.999
                } else if off < 0.0 {
                    0.0
                } else {
                    off
                };
                self.blep[n].add(off, self.last[n] - val);
                self.last[n] = val;
            }
            self.ch[n].stepped = false;
            *out = if self.blep[n].samples_left > 0 {
                self.blep[n].run(val)
            } else {
                val
            };
        }
        let mut io = [(v[0] + v[3]) * 2.0, (v[1] + v[2]) * 2.0];
        if self.model == Model::A500 {
            self.lo.low_pass(&mut io);
        }
        if self.model != Model::Raw {
            if led {
                self.led.low_pass(&mut io);
            }
            self.hi.high_pass(&mut io);
        }
        io.map(|x| x.clamp(-32768.0, 32767.0) as i16)
    }

    /// The state the next samples depend on, as bits: per channel the step phase, the last level, the held byte, the
    /// words left, the fetched word, the byte index and the BLEP ring, then the filters and the output fraction. For the differential test, which checks it against the C.
    pub fn state_bits(&self) -> Vec<u64> {
        let mut b = Vec::new();
        for n in 0..4 {
            b.push(self.ch[n].phase.to_bits());
            b.push(self.last[n].to_bits() as u64);
            let c = &self.ch[n];
            b.extend([
                c.cur as u8 as u64,
                c.left as u64,
                c.word as u64,
                c.bytei as u64,
            ]);
            let bl = &self.blep[n];
            b.extend([bl.index as u64, bl.samples_left as i64 as u64]);
            b.extend(bl.buf.iter().map(|x| x.to_bits() as u64));
        }
        for f in [&self.lo, &self.hi] {
            b.extend(
                f.tmp
                    .iter()
                    .chain([&f.a0, &f.b1])
                    .map(|x| x.to_bits() as u64),
            );
        }
        let l = &self.led;
        b.extend(l.tmp.iter().flatten().map(|x| x.to_bits() as u64));
        b.extend([l.a1, l.a2, l.b1, l.b2].iter().map(|x| x.to_bits() as u64));
        b.push(self.out_frac.to_bits());
        b
    }

    /// The BLEP table's bits, for the differential test.
    pub fn blep_table_bits() -> Vec<u32> {
        MIN_BLEP.iter().map(|x| x.to_bits()).collect()
    }
}

/// The 44-byte header main.c `--wav` writes: PCM, 2 channels, 16-bit, 44.1 kHz, `data_bytes` of samples after it.
pub fn wav_header(data_bytes: u32) -> [u8; 44] {
    let mut h = [0u8; 44];
    h[0..4].copy_from_slice(b"RIFF");
    h[4..8].copy_from_slice(&(36 + data_bytes).to_le_bytes());
    h[8..16].copy_from_slice(b"WAVEfmt ");
    h[16..20].copy_from_slice(&16u32.to_le_bytes());
    h[20..22].copy_from_slice(&1u16.to_le_bytes());
    h[22..24].copy_from_slice(&2u16.to_le_bytes());
    h[24..28].copy_from_slice(&SAMPLE_RATE.to_le_bytes());
    h[28..32].copy_from_slice(&(SAMPLE_RATE * 4).to_le_bytes());
    h[32..34].copy_from_slice(&4u16.to_le_bytes());
    h[34..36].copy_from_slice(&16u16.to_le_bytes());
    h[36..40].copy_from_slice(b"data");
    h[40..44].copy_from_slice(&data_bytes.to_le_bytes());
    h
}

/// Writes interleaved stereo samples as the WAV file main.c `--wav` makes and judge.py `wav()` reads (the samples
/// start at byte 44, little-endian i16, left then right).
pub fn write_wav<W: Write>(mut w: W, samples: &[i16]) -> io::Result<()> {
    assert!(
        samples.len().is_multiple_of(2),
        "stereo samples come in pairs"
    );
    let mut b = Vec::with_capacity(44 + samples.len() * 2);
    b.extend_from_slice(&wav_header((samples.len() * 2) as u32));
    for s in samples {
        b.extend_from_slice(&s.to_le_bytes());
    }
    w.write_all(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_wave_plays_and_interrupts_at_block_end() {
        // one block of 2 words: +64, -64, +64, -64 at period 200 (17.7 kHz), full volume, raw output
        let mut ram = vec![0u8; 1024];
        ram[0x100..0x104].copy_from_slice(&[0x40, 0xc0, 0x40, 0xc0]);
        let mut p = Paula::new(Model::Raw, false);
        assert_eq!(p.write(0x0a0, 0, 0), 0);
        assert_eq!(p.write(0x0a2, 0x100, 0), 0);
        p.write(0x0a4, 2, 0);
        p.write(0x0a6, 200, 0);
        p.write(0x0a8, 80, 0); // plays at 64
        p.dma_changed(0, 0x201);
        let mut out = Vec::new();
        let chip = Chip::new(&ram);
        let irq = p.play_line(chip, false, &mut out);
        assert_eq!(irq, 0x80, "the start interrupt");
        let mut irqs = 0;
        for _ in 0..311 {
            irqs += (p.play_line(chip, false, &mut out) & 0x80 != 0) as u32;
        }
        // 312 lines x 227.5 cc / 80.43 cc per sample
        assert_eq!(out.len() / 2, 882);
        // channel 0 is left only; the first two samples (80 and 161 cc) come before the first byte (200 cc)
        assert!(out
            .chunks(2)
            .all(|s| s[1] == 0 && [0, 8192].contains(&s[0].abs())));
        assert_eq!(out[..4], [0; 4]);
        assert!(out.chunks(2).skip(2).all(|s| s[0] != 0));
        // 4 bytes per block at 200 cc: one block end per 800 cc
        assert!((80..=90).contains(&irqs), "{irqs}");
    }

    #[test]
    fn dat_write_with_dma_off_asks_for_the_next_word() {
        let mut p = Paula::default();
        assert_eq!(p.write(0x0ca, 7, 0x0200), 0x200);
        assert_eq!(p.write(0x0ca, 7, 0x0204), 0);
    }

    #[test]
    fn wav_header_fields() {
        let mut b = Vec::new();
        write_wav(&mut b, &[1, -1, 2, -2]).unwrap();
        assert_eq!(b.len(), 52);
        assert_eq!(&b[0..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), 44);
        assert_eq!(u32::from_le_bytes(b[24..28].try_into().unwrap()), 44100);
        assert_eq!(&b[44..48], &[1, 0, 0xff, 0xff]);
    }
}
