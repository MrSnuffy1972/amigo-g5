/* intentrt.c: the NATIVE-INTENT runtime (clean sheet, vertical slice, 15 Oct 2026). Replaces chipset.c in a native exe.
 * See claude-run/NATIVE-INTENT-SPEC.md and INTENT-DESIGN.md. What it is: the memory map, a register shadow (what the game has set), CIA and
 * keyboard/joystick input, Paula channel playback, interrupt request bookkeeping, and the DISPLAY composed once per tick from what the game
 * set up (intent.c: copper list read as a table, bitmaps, palette). What it is not: no copper execution, no beam rendering, no sprite or
 * blitter engine (a blit or sprite use is counted and logged as a finding, never emulated). Derived from chipset.c's memory, CIA and Paula code. */
#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <stdlib.h>
#include <math.h>
#include "amiga.h"

uint8_t chipram[CHIP_SIZE];
uint8_t slowram[SLOW_SIZE];
uint32_t framebuf[SCR_W*SCR_H];
uint64_t (*cpu_now)(void);
uint16_t custom[256];
uint16_t intena, intreq, dmacon;
int chip_frame_done; uint64_t chip_frame_count;
int joy_up, joy_down, joy_left, joy_right, joy_fire, joy_fire2;
uint16_t mouse_joy0dat; int mouse_lmb; uint16_t amigo_potgor_clear;   /* the mouse port (port 1), driven by the front end (menu.c), Cowork 17 Oct */
aud_t aud[4];
uint32_t smc_writes;
static void nolog(const char *f, ...) { (void)f; }
void (*hle_log)(const char *fmt, ...) = nolog;
static void noipl(void) {}
void (*ipl_changed)(void) = noipl;
void (*dbg_read_hook)(int reg);
static void nostall(uint64_t c) { (void)c; }
void (*cpu_stall)(uint64_t cycles) = nostall;
int blit_nasty = 0;
static int line;                 /* virtual line within the tick (the scheduler calls tick_line_step once per 454 cycles) */
static uint32_t cop1lc, cop2lc;
static uint32_t bplpt[6];
static uint16_t palette[32];
static int warned[64];
uint64_t native_blits, native_sprite_uses, native_unhandled;   /* findings: hardware use the intent runtime does not implement */
static uint16_t blt_busy_until_line;
static void prof_cia(int n, int reg, int wr) { (void)n; (void)reg; (void)wr; }
static void prof_aud(int n) { (void)n; }
#define WARN(n, ...) do { if (!warned[n]) { warned[n] = 1; hle_log(__VA_ARGS__); } } while (0)

/* ---------------- CIA ---------------- */
typedef struct {
    uint8_t pra, prb, ddra, ddrb, sdr, icr, icrmask, cra, crb;
    uint16_t ta, tb, ta_latch, tb_latch;
    uint32_t tod, tod_latch; int tod_latched, tod_stopped; uint32_t tod_alarm;
    double eacc;
} cia_t;
static cia_t cia[2];             /* 0 = CIA-A ($BFE001), 1 = CIA-B ($BFD000) */
static uint8_t kbd_queue[64]; static int kbd_head, kbd_tail; static int kbd_wait;
static int kbd_busy, kbd_hs, kbd_lines;   /* kbfix: byte in SDR awaiting the handshake, SPMODE seen high, lines waited (twin of chipset.c) */

static void cia_irq(int n, int bit) {
    cia[n].icr |= bit;
}
static void update_cia_intreq(void) {
    if (cia[0].icr & cia[0].icrmask & 0x1f) intreq |= 0x0008;
    if (cia[1].icr & cia[1].icrmask & 0x1f) intreq |= 0x2000;
}
static void cia_tick(int n, int ticks) {
    cia_t *c = &cia[n];
    if (c->cra & 1) {
        int t = c->ta - ticks;
        while (t < 0) {
            cia_irq(n, 1);
            if (c->cra & 8) { c->cra &= ~1; t = c->ta_latch; break; }
            t += c->ta_latch + 1;
            if (c->ta_latch == 0) { t = 0; break; }
        }
        c->ta = (uint16_t)t;
    }
    if ((c->crb & 1) && !(c->crb & 0x40)) {
        int t = c->tb - ticks;
        while (t < 0) {
            cia_irq(n, 2);
            if (c->crb & 8) { c->crb &= ~1; t = c->tb_latch; break; }
            t += c->tb_latch + 1;
            if (c->tb_latch == 0) { t = 0; break; }
        }
        c->tb = (uint16_t)t;
    }
}
static void cia_tod(int n) {
    cia_t *c = &cia[n];
    if (c->tod_stopped) return;
    c->tod = (c->tod + 1) & 0xffffff;
    if (c->tod == c->tod_alarm) cia_irq(n, 4);
}
void kbd_push(uint8_t rawkey, int down) {
    int nt = (kbd_tail + 1) & 63; if (nt == kbd_head) return;
    kbd_queue[kbd_tail] = (uint8_t)((rawkey & 0x7f) | (down ? 0 : 0x80)); kbd_tail = nt;
}
/* kbfix (Cowork 17 Oct), the same keyboard link as chipset.c: a byte stays in CIA-A SDR until the game's handshake (CRA SPMODE high
 * then low) or a timeout of 8 x 143 ms, then an 8-line gap before the next; nothing is received while SPMODE is an output. Every line. */
#define KBD_GAP_LINES 8
#define KBD_TIMEOUT_LINES 17872
static void kbd_service(void) {
    if (kbd_busy) { if (++kbd_lines >= KBD_TIMEOUT_LINES) { kbd_busy = 0; kbd_wait = KBD_GAP_LINES; } return; }
    if (kbd_wait > 0) { kbd_wait--; return; }
    if (kbd_head == kbd_tail) return;
    if (cia[0].cra & 0x40) return;   /* serial port is an output: nothing can be received */
    uint8_t code = kbd_queue[kbd_head]; kbd_head = (kbd_head + 1) & 63;
    /* keyboard transmits rol(code,1) inverted; code = rawkey | up<<7 */
    cia[0].sdr = (uint8_t)~((code << 1) | (code >> 7));
    cia_irq(0, 8);
    kbd_busy = 1; kbd_hs = 0; kbd_lines = 0;
}
static FILE *creadlog(void) { static FILE *f; static int i; if (!i) { i = 1; if (getenv("CREADLOG")) f = fopen(getenv("CREADLOG"), "w"); } return f; }
static uint8_t cia_read_raw(int n, int reg);
static uint8_t cia_read(int n, int reg) {
    prof_cia(n, reg, 0); { extern void hw_cia(int, int, int); hw_cia(n, reg, 0); }   /* CREADLOG also records CIA reads, as reg 'A'/'B' + register */
    uint8_t v = cia_read_raw(n, reg);
    FILE *rl = creadlog();
    if (rl) { fprintf(rl, "%llu %d C%c%X %02X\n", (unsigned long long)chip_frame_count, line, 'A' + n, reg, v); fflush(rl); }
    return v;
}
static uint8_t cia_read_raw(int n, int reg) {
    cia_t *c = &cia[n];
    if (dbg_read_hook) dbg_read_hook(0x1000 + n * 16 + reg);
    switch (reg) {
    case 0:
        if (n == 0) {
            uint8_t v = 0xff;
            if (joy_fire) v &= ~0x80;      /* port 2 fire */
            if (mouse_lmb) v &= ~0x40;     /* port 1 fire: the left mouse button */
            v &= ~0x20; v &= ~0x10;        /* disk ready/track0 lines low, no change seen by game */
            v |= 0x30;
            return (uint8_t)((v & ~c->ddra) | (c->pra & c->ddra));
        }
        return (uint8_t)((0xff & ~c->ddra) | (c->pra & c->ddra));
    case 1: return (uint8_t)((0xff & ~c->ddrb) | (c->prb & c->ddrb));
    case 2: return c->ddra; case 3: return c->ddrb;
    case 4: return c->ta & 0xff; case 5: return c->ta >> 8;
    case 6: return c->tb & 0xff; case 7: return c->tb >> 8;
    case 8: { uint32_t t = c->tod_latched ? c->tod_latch : c->tod; c->tod_latched = 0; return t & 0xff; }
    case 9: return ((c->tod_latched ? c->tod_latch : c->tod) >> 8) & 0xff;
    case 10: c->tod_latch = c->tod; c->tod_latched = 1; return (c->tod >> 16) & 0xff;
    case 12: return c->sdr;
    case 13: { uint8_t v = c->icr; if (v & c->icrmask) v |= 0x80; c->icr = 0; return v; }
    case 14: return c->cra; case 15: return c->crb;
    }
    return 0xff;
}
static void cia_write(int n, int reg, uint8_t v) {
    { extern void hw_cia(int, int, int); hw_cia(n, reg, 1); }
    prof_cia(n, reg, 1);
    cia_t *c = &cia[n];
    switch (reg) {
    case 0: c->pra = v; break; case 1: c->prb = v; break;
    case 2: c->ddra = v; break; case 3: c->ddrb = v; break;
    case 4: c->ta_latch = (c->ta_latch & 0xff00) | v; break;
    case 5: c->ta_latch = (uint16_t)((c->ta_latch & 0xff) | (v << 8));
            if (!(c->cra & 1)) c->ta = c->ta_latch;
            if (c->cra & 8) { c->ta = c->ta_latch; c->cra |= 1; } break;
    case 6: c->tb_latch = (c->tb_latch & 0xff00) | v; break;
    case 7: c->tb_latch = (uint16_t)((c->tb_latch & 0xff) | (v << 8));
            if (!(c->crb & 1)) c->tb = c->tb_latch;
            if (c->crb & 8) { c->tb = c->tb_latch; c->crb |= 1; } break;
    case 8: if (c->crb & 0x80) c->tod_alarm = (c->tod_alarm & 0xffff00) | v; else { c->tod = (c->tod & 0xffff00) | v; c->tod_stopped = 0; } break;
    case 9: if (c->crb & 0x80) c->tod_alarm = (c->tod_alarm & 0xff00ff) | (v << 8); else c->tod = (c->tod & 0xff00ff) | (v << 8); break;
    case 10: if (c->crb & 0x80) c->tod_alarm = (c->tod_alarm & 0x00ffff) | (v << 16); else { c->tod = (c->tod & 0x00ffff) | (v << 16); c->tod_stopped = 1; } break;
    case 12: c->sdr = v; if (c->cra & 0x40) cia_irq(n, 8); break;   /* serial output: byte "sent" at once */
    case 13: if (v & 0x80) c->icrmask |= v & 0x1f; else c->icrmask &= ~(v & 0x1f); break;
    case 14: if (n == 0 && kbd_busy) { if (v & 0x40) kbd_hs = 1; else if (kbd_hs) { kbd_busy = 0; kbd_wait = KBD_GAP_LINES; } }   /* kbfix: keyboard handshake = SPMODE high then low */
             c->cra = v & ~0x10; if (v & 0x10) c->ta = c->ta_latch; break;
    case 15: c->crb = v & ~0x10; if (v & 0x10) c->tb = c->tb_latch; break;
    }
}

/* ---------------- beam ---------------- */
static uint64_t frame_base_cyc;   /* cycle count at start of current line */
int chip_line(void) { return line; }
/* G2E9 beamread (Claude sub-agent, 27 Oct 2026). Measured on game 16 (ATF II): frame_base_cyc moves with the scheduler clock CYC
 * (tick_line_step adds CYC_PER_LINE per line, the scheduler steps a line per 454 CYC), but beam_h() measured from cpu_now(), the harness
 * clock CYC x 412/454 x 1.1 = 0.998 CYC. After ~1 tick cpu_now() - frame_base_cyc is negative for good (frame 197: -56,000), so VHPOSR's
 * position byte read 0 for the whole run. And in preempt mode a slice runs to a block budget (cycles to the next line / 8 per block),
 * so a tight beam loop overran its line by several lines while `line` moved only between slices: VPOSR showed the slice's first line
 * for the whole overrun and the lines in between were never seen. ATF II's slave delay (1800 x "wait until line $10") then ran at
 * 0.45 calls a tick against the reference's 16 (harness IRQLOG: delay frames 278-390), and the port stayed black for all 600 ticks.
 * A beam READ now uses the scheduler clock: line = line + whole lines of CYC since the line start (wrapping at the end of the tick),
 * position = the rest / 2. Scheduling, interrupts and the copper table are unchanged. AMIGO_BEAMREAD=0 = the old reads. */
extern uint64_t CYC;   /* the scheduler clock (cpu.h; this file does not include it) */
#ifndef AMIGO_BEAMREAD
#define AMIGO_BEAMREAD 1
#endif
static int beamread_on = -1;
static int beamread(void) { if (beamread_on < 0) { const char *e = getenv("AMIGO_BEAMREAD"); beamread_on = e ? (e[0] != '0') : AMIGO_BEAMREAD; } return beamread_on; }
static int beam_h(void) {
    int64_t d = beamread() ? (int64_t)(CYC - frame_base_cyc) : (int64_t)(cpu_now() - frame_base_cyc);
    if (d < 0) d = 0;
    if (beamread()) d %= CYC_PER_LINE;
    int h = (int)(d / 2); if (h > 0xe2) h = 0xe2;
    return h;
}
static int beam_line(void) {   /* G2E9 beamread: the line the CPU has reached (`line` + whole lines of the running slice) */
    if (!beamread()) return line;
    int64_t d = (int64_t)(CYC - frame_base_cyc); if (d < 0) d = 0;
    return (int)((line + d / CYC_PER_LINE) % LINES_PER_FRAME);
}

static inline uint16_t crw(uint32_t a) { a &= CHIP_WMASK; return (uint16_t)((chipram[a] << 8) | chipram[a + 1]); }
void memlog_byte(uint32_t a, uint8_t *p);
static uint32_t rgb12(uint16_t c) {
    uint32_t r = (c >> 8) & 15, g = (c >> 4) & 15, b = c & 15;
    return 0xff000000u | (r * 17) << 16 | (g * 17) << 8 | (b * 17);
}

/* ---------------- audio (state + interrupt timing) ---------------- */
static int aud_on[4]; static int aud_irq_pending;
/* g47 (Cowork 19 Oct 2026, from Gemini g47): a channel switched on by DMACON latches LC/LEN at the start of the next line, not at
 * the DMACON write itself, so a game that writes DMACON before AUDxLC/LEN (254) plays the new block, not the stale pointer.
 * Gate AMIGO_AUD_DEFER (compile-time default below, environment AMIGO_AUD_DEFER=0/1 overrides). */
#ifndef AMIGO_AUD_DEFER
#define AMIGO_AUD_DEFER 0
#endif
static int aud_start_pending, aud_defer = -1;
static int aud_defer_on(void) { if (aud_defer < 0) { const char *e = getenv("AMIGO_AUD_DEFER"); aud_defer = e ? (e[0] != '0') : AMIGO_AUD_DEFER; } return aud_defer; }
static void aud_start(int n); static void aud_stop(int n);
static void audio_dmacon(uint16_t old, uint16_t now) {
    for (int n = 0; n < 4; n++) {
        int was = (old & 0x200) && (old & (1 << n)), is = (now & 0x200) && (now & (1 << n));
        if (is && !was) { aud_on[n] = 1; if (aud_defer_on()) aud_start_pending |= 1 << n; else aud_start(n); aud_irq_pending |= 1 << n; }   /* raised at the channel's next DMA slot (end of line); g47: start deferred to the next audio_line */
        else if (!is && was && (aud_start_pending & (1 << n))) { aud_on[n] = 0; aud_start_pending &= ~(1 << n); aud_stop(n); }   /* g47: off again before its deferred start */
        else if (!is && was) { aud_on[n] = 0; aud_stop(n); }
    }
}
/* Paula: DMA playback, interrupts at block (re)start, mixed to 44.1 kHz stereo */
typedef struct { uint32_t ptr; uint32_t left; uint16_t word; int bytei; double phase; int8_t cur; int stepped; } chan_t;
/* Output stage vendored from pt2-clone (BSD 3-Clause, runtime/paula/README.md): BLEP for Paula's zero-order-hold
 * steps, and the motherboard RC filters with schematic component values. Karl, 13 Oct 2026: reuse, don't reinvent. */
#include "paula/pt2_blep.c"
#include "paula/pt2_rcfilters.c"
static blep_t pblep[4]; static float plast[4];
static onePoleFilter_t pf_lo, pf_hi; static twoPoleFilter_t pf_led;
static int paula_mode = -1;      /* 0 raw, 1 A500, 2 A1200 (environment AMIGO_PAULA) */
static int paula_forced = -1;
void paula_set_model(int m) { paula_forced = m; paula_mode = -1; }   /* 0 raw, 1 A500, 2 A1200 (the game's menu) */
static void paula_setup(void) {
    const char *e = getenv("AMIGO_PAULA");
    paula_mode = (e && !strcmp(e, "raw")) ? 0 : (e && !strcmp(e, "a1200")) ? 2 : 1;
    if (paula_forced >= 0 && !e) paula_mode = paula_forced;
    const double fs = 44100.0;
    if (paula_mode == 1) {
        setupOnePoleFilter(fs, 1.0 / (2.0 * PI * 360.0 * 1e-7), &pf_lo);       /* R321 360 ohm, C321 0.1 uF: ~4421 Hz */
        setupOnePoleFilter(fs, 1.0 / (2.0 * PI * 1390.0 * 2.233e-5), &pf_hi);  /* R324+R325, C334+C335: ~5.13 Hz */
    } else {
        setupOnePoleFilter(fs, 1.0 / (2.0 * PI * 1360.0 * 2.2e-5), &pf_hi);    /* A1200: ~5.32 Hz high-pass only */
    }
    {   /* LED filter, 2-pole Sallen-Key: R322 = R323 = 10K, C322 6800 pF, C323 3900 pF: ~3091 Hz, Q ~0.660 */
        const double R1 = 10000.0, R2 = 10000.0, C1 = 6.8e-9, C2 = 3.9e-9;
        setupTwoPoleFilter(fs, 1.0 / (2.0 * PI * sqrt(R1 * R2 * C1 * C2)), sqrt(R1 * R2 * C1 * C2) / (C2 * (R1 + R2)), &pf_led);
    }
}
static chan_t ch[4];
int16_t audio_buf[16384 * 2]; int audio_len;     /* frontend drains this every frame */
static double out_frac;
static void aud_start(int n) {
    prof_aud(n);
    ch[n].ptr = aud[n].lc & CHIP_WMASK; ch[n].left = aud[n].len ? aud[n].len : 65536; ch[n].bytei = 2; ch[n].phase = 0;
}
static void aud_stop(int n) { ch[n].cur = 0; }
static void chan_step(int n, double cc) {
    chan_t *c = &ch[n];
    int per = aud[n].per < 124 ? 124 : aud[n].per;
    c->phase += cc;
    while (c->phase >= per) {
        c->phase -= per;
        if (c->bytei >= 2) {
            if (c->left == 0) {   /* block finished: reload and request an interrupt */
                c->ptr = aud[n].lc & CHIP_WMASK; c->left = aud[n].len ? aud[n].len : 65536;
                intreq |= (uint16_t)(0x80 << n);
            }
            c->word = crw(c->ptr); c->ptr = (c->ptr + 2) & CHIP_WMASK; c->left--; c->bytei = 0;
        }
        c->cur = (int8_t)(c->bytei == 0 ? (c->word >> 8) : (c->word & 0xff)); c->bytei++; c->stepped = 1;
    }
}
static void audio_line(void) {
    /* 227 colour clocks per line; 44100 Hz output = 80.43 colour clocks per sample */
    const double cc_per_sample = 3546895.0 / 44100.0;
    double t = 0, end = 227.5;   /* PAL line = 227.5 colour clocks */
    if (aud_start_pending) { for (int n = 0; n < 4; n++) if (aud_start_pending & (1 << n)) aud_start(n); aud_start_pending = 0; }   /* g47 */
    while (1) {
        double next = (1.0 - out_frac) * cc_per_sample;   /* cc until the next output sample */
        if (t + next > end) { out_frac += (end - t) / cc_per_sample; break; }
        for (int n = 0; n < 4; n++) if (aud_on[n]) chan_step(n, next);
        t += next; out_frac = 0;
        if (paula_mode < 0) paula_setup();
        float v[4];
        for (int n = 0; n < 4; n++) {
            int vol = aud[n].vol > 64 ? 64 : aud[n].vol;
            float val = aud_on[n] ? (float)(ch[n].cur * vol) : 0.0f;
            if (paula_mode == 0) { v[n] = val; ch[n].stepped = 0; continue; }
            if (val != plast[n]) {
                /* the step happened ch[n].phase colour clocks ago (or now, for a volume or DMA change) */
                float off = ch[n].stepped ? (float)(ch[n].phase / cc_per_sample) : 0.0f;
                if (off >= 1.0f) off = 0.999f;
                if (off < 0.0f) off = 0.0f;
                blepAdd(&pblep[n], off, plast[n] - val);
                plast[n] = val;
            }
            ch[n].stepped = 0;
            v[n] = pblep[n].samplesLeft > 0 ? blepRun(&pblep[n], val) : val;
        }
        {
            float io[2] = { (v[0] + v[3]) * 2.0f, (v[1] + v[2]) * 2.0f };
            if (paula_mode == 1) onePoleLPFilterStereo(&pf_lo, io, io);
            if (paula_mode > 0) {
                if (!(cia[0].pra & cia[0].ddra & 2)) twoPoleLPFilterStereo(&pf_led, io, io);   /* LED on: CIA-A PRA bit 1 low */
                onePoleHPFilterStereo(&pf_hi, io, io);
            }
            for (int k = 0; k < 2; k++) io[k] = io[k] > 32767.f ? 32767.f : io[k] < -32768.f ? -32768.f : io[k];
            if (audio_len < 16384) { audio_buf[audio_len * 2] = (int16_t)io[0]; audio_buf[audio_len * 2 + 1] = (int16_t)io[1]; audio_len++; }
        }
    }
    for (int n = 0; n < 4; n++) if (aud_on[n]) chan_step(n, end - t);
    for (int n = 0; n < 4; n++) if (aud_irq_pending & (1 << n)) intreq |= (uint16_t)(0x80 << n);
    aud_irq_pending = 0;
}

#include "intent.c"

/* bufix 19 Oct (g20c): progressive line-by-line compose is the default; AMIGO_INTENT_PROGRESSIVE=0 restores the end-of-tick compose */
static int in_progressive = 1;
static void in_progressive_init(void) { const char *e = getenv("AMIGO_INTENT_PROGRESSIVE"); in_progressive = !(e && e[0] == '0'); }
/* ---------------- tick timeline: one call per 454 cycles of the scheduler's clock ---------------- */
void tick_line_step(void) {
    audio_line();
    /* CIA-A/B timers tick at the E clock (CPU/10): 45.4 per line; CIA-B TOD counts lines (the game's own timer reads need them) */
    for (int n = 0; n < 2; n++) {
        cia[n].eacc += CYC_PER_LINE / 10.0;
        int t = (int)cia[n].eacc; cia[n].eacc -= t;
        cia_tick(n, t);
    }
    cia_tod(1);
    kbd_service();   /* kbfix: every line (was every 64th), as chipset.c */
    frame_base_cyc += CYC_PER_LINE;
    line++;
    if (in_progressive) intent_compose_to(chipram, line, framebuf);   /* bufix: compose the line the virtual beam has just passed */
    for (int i = 0; i < it_start.ncb; i++) if (it_start.cb[i].line == line) intreq |= it_start.cb[i].bits;   /* the copper list's line-timed interrupts (IK+: three splits per tick), from the table */
    if (line >= LINES_PER_FRAME) {
        if (in_progressive) intent_compose_to(chipram, 312, framebuf);   /* bufix: finish the lines not yet composed */
        else intent_compose(chipram, &is_start, &it_start, framebuf);      /* the picture of this tick, from what the game set up */
        line = 0; chip_frame_done = 1; chip_frame_count++;
        intent_frame_begin(cop1lc);                                   /* snapshot of the register shadow + the copper list as a table */
        {   /* G2E9 coplc (Claude sub-agent, 27 Oct 2026): a copper MOVE to COP1LC/COP2LC in this tick's list sets where the NEXT tick's
             * copper starts, as in chipset.c. Measured on game 108 (Rainbow Arts intro): its list at $30E6C is only MOVE COP1LC=$30DA0,
             * COLOR00, END; the native table restarted at $30E6C every tick, so the real list ($30DA0, display + COPER INTREQ) never ran:
             * no COPER interrupt, no picture for 600 ticks. A CPU write later in the tick still overrides it. AMIGO_INTENT_COPLC=0 = old. */
            static int on = -1; if (on < 0) { const char *e = getenv("AMIGO_INTENT_COPLC"); on = e ? (e[0] != '0') : 1; }
            /* Guard (measured on 410, whose list re-points to an all-zero area and went black): the new pointer is taken only when
             * it leads to a list that ends ($FFFF,$FFFE within 4000 instructions) and does not start with a zero word pair. */
            if (on && in_frame_c1w && coplc_plausible(in_frame_c1 & CHIP_WMASK)) cop1lc = in_frame_c1 & CHIP_WMASK;
            if (on && in_frame_c2w && coplc_plausible(in_frame_c2 & CHIP_WMASK)) cop2lc = in_frame_c2 & CHIP_WMASK; }
        intent_compose_begin(&is_start, &it_start);
        intreq |= 0x0020;                                             /* VERTB: the tick's interrupt */
        /* Cowork filedboot 18 Oct: a copper INTREQ move before the list's first WAIT has line 0, and the per-line check above runs
         * after line++ (lines 1..311), so it never fired. The copper restarts at the top of the frame, so it fires with VERTB.
         * Measured on Cannon Fodder (80): MOVE $8004 -> INTREQ (SOFT, level 1) at the head of its list sets the frame flag $8060E
         * that its main loop polls; without it the port spun at $864A0 for the whole run. */
        for (int i = 0; i < it_start.ncb; i++) if (it_start.cb[i].line == 0) intreq |= it_start.cb[i].bits;
        cia_tod(0);
    }
    update_cia_intreq();
}

int chip_ipl(void) {
    update_cia_intreq();
    if (!(intena & 0x4000)) return 0;
    uint16_t p = intena & intreq & 0x3fff;
    if (!p) return 0;
    if (p & 0x2000) return 6;
    if (p & 0x1800) return 5;
    if (p & 0x0780) return 4;
    if (p & 0x0070) return 3;
    if (p & 0x0008) return 2;
    return 1;
}

static uint16_t joydat(void) {
    int r = joy_right, l = joy_left, u = joy_up, d = joy_down;
    return (uint16_t)((r << 1) | (l << 9) | ((r ^ d) & 1) | (((l ^ u) & 1) << 8));
}

/* HWSTAT=1 (dev): how many hardware register accesses the run makes, per register: the list step 3 must turn into named calls (deciding test 1: goal 0 at run time) */
static uint64_t hw_named; static uint64_t hw_rd[256], hw_wr[256], hw_ciard[2][16], hw_ciawr[2][16]; static int hw_on = -1;
static void hw_dump(void) {
    uint64_t tr = 0, tw = 0; for (int i = 0; i < 256; i++) { tr += hw_rd[i]; tw += hw_wr[i]; }
    fprintf(stderr, "HWSTAT custom register accesses: %llu reads, %llu writes in %llu ticks; of all accesses (custom + CIA) %llu came through named calls, the rest through the address decoder\n", (unsigned long long)tr, (unsigned long long)tw, (unsigned long long)chip_frame_count, (unsigned long long)hw_named);
    for (int i = 0; i < 256; i++) if (hw_rd[i] || hw_wr[i]) fprintf(stderr, "HWSTAT  %03X reads %llu writes %llu\n", i * 2, (unsigned long long)hw_rd[i], (unsigned long long)hw_wr[i]);
    for (int n = 0; n < 2; n++) for (int r = 0; r < 16; r++) if (hw_ciard[n][r] || hw_ciawr[n][r]) fprintf(stderr, "HWSTAT  CIA-%c reg %X reads %llu writes %llu\n", n ? 'B' : 'A', r, (unsigned long long)hw_ciard[n][r], (unsigned long long)hw_ciawr[n][r]);
}
static void hw_init(void) { hw_on = getenv("HWSTAT") != NULL; if (hw_on) atexit(hw_dump); }
static uint16_t read_custom(int reg) {
    if (hw_on < 0) hw_init(); if (hw_on) hw_rd[(reg & 0x1fe) >> 1]++;
    if (dbg_read_hook) dbg_read_hook(reg);
    switch (reg) {
    case 0x002: return (uint16_t)(dmacon & 0x3ff);                           /* blitter never busy: blits are not run */
    case 0x004: return (uint16_t)(0x8000 | ((beam_line() >> 8) & 1));       /* VPOSR/VHPOSR: only a clock for beam reads the translator did not recognise (G2E9 beamread) */
    case 0x006: return (uint16_t)(((beam_line() & 0xff) << 8) | beam_h());
    case 0x00a: return mouse_joy0dat;                                         /* JOY0DAT: port 1 (mouse counters, or a joystick) */
    case 0x00c: return joydat();
    case 0x00e: return 0;
    case 0x010: return custom[0x9e >> 1];
    case 0x016: return (uint16_t)(0xff00 & ~(joy_fire2 ? 0x4000 : 0) & ~amigo_potgor_clear);   /* POTGOR; bit 10 = right mouse button */
    case 0x018: return 0x3000;
    case 0x01a: return 0;
    case 0x01c: return intena;
    case 0x01e: return intreq;
    }
    return 0xffff;   /* a write-only or unknown register reads as $FFFF, as the reference harness's chipset.c does (30 Oct 2026, Qattro's line wait on $DFF026) */
}

static void write_custom(int reg, uint16_t v, int from_copper) {
    (void)from_copper;
    reg &= 0x1fe; if (hw_on < 0) hw_init(); if (hw_on) hw_wr[reg >> 1]++;
    intent_cpu_write(line, reg, v);                    /* the game's own register writes are stamped with the line they happen at */
    custom[reg >> 1] = v;
    switch (reg) {
    case 0x080: cop1lc = (cop1lc & 0xffff) | ((uint32_t)v << 16); break;
    case 0x082: cop1lc = (cop1lc & 0xffff0000) | (v & 0xfffe); break;
    case 0x084: cop2lc = (cop2lc & 0xffff) | ((uint32_t)v << 16); break;
    case 0x086: cop2lc = (cop2lc & 0xffff0000) | (v & 0xfffe); break;
    case 0x05c: break;          /* BLTSIZV (ECS): sets the height, the blit starts on BLTSIZH ($05E) - Cowork 17 Oct */
    case 0x05e: { static int (*d)(uint16_t, uint16_t); extern int intent_blit_dispatch_ext(uint16_t, uint16_t); uint16_t hh = custom[0x05c >> 1] & 0x7fff, ww = v & 0x7ff; intent_blit_dispatch_ext(hh ? hh : 32768, ww ? ww : 2048); native_blits++; intreq |= 0x40; break; }
    case 0x058:    /* BLTSIZE: native blit CLASSES (intentblit.inc, Cowork 17 Oct); unclassified ones are counted findings */
      { extern int intent_blit_dispatch_ext(uint16_t, uint16_t); uint16_t hh = (v >> 6) & 0x3ff, ww = v & 0x3f; intent_blit_dispatch_ext(hh ? hh : 1024, ww ? ww : 64); }
    { static FILE *bl; static int bln; if (!bl && bln == 0) { bln = 1; bl = fopen("out/blits.txt", "w"); }   /* BLITLOG (Cowork 17 Oct): every blit the game asks for, for the native blit classes */
      if (bl && bln < 20000) { bln++; fprintf(bl, "tick %llu line %d size %s %04X con0 %04X con1 %04X fwm %04X lwm %04X A %06X B %06X C %06X D %06X amod %d bmod %d cmod %d dmod %d adat %04X bdat %04X cdat %04X\n",
        (unsigned long long)chip_frame_count, line, reg == 0x058 ? "OCS" : "ECS", v, custom[0x40>>1], custom[0x42>>1], custom[0x44>>1], custom[0x46>>1],
        ((custom[0x50>>1] << 16) | custom[0x52>>1]) & 0x1ffffe, ((custom[0x4c>>1] << 16) | custom[0x4e>>1]) & 0x1ffffe, ((custom[0x48>>1] << 16) | custom[0x4a>>1]) & 0x1ffffe, ((custom[0x54>>1] << 16) | custom[0x56>>1]) & 0x1ffffe,
        (int16_t)custom[0x64>>1], (int16_t)custom[0x62>>1], (int16_t)custom[0x60>>1], (int16_t)custom[0x66>>1], custom[0x74>>1], custom[0x72>>1], custom[0x70>>1]); fflush(bl); } }
        native_blits++; intreq |= 0x40; break;
    case 0x096: {
        uint16_t old = dmacon;
        if (v & 0x8000) dmacon |= v & 0x7fff; else dmacon &= ~v;
        audio_dmacon(old, dmacon);
        if ((dmacon & 0x220) == 0x220) WARN(11, "FINDING: sprite DMA enabled - sprites are not implemented in the slice runtime\n");
        break; }
    case 0x09a: if (v & 0x8000) intena |= v & 0x7fff; else intena &= ~v; ipl_changed(); break;
    case 0x09c: if (v & 0x8000) intreq |= v & 0x7fff; else intreq &= ~v; ipl_changed(); break;
    default:
        if (reg >= 0x0e0 && reg < 0x0f8) {
            int p = (reg - 0xe0) >> 2;
            if (reg & 2) bplpt[p] = (bplpt[p] & 0xffff0000) | (v & 0xfffe);
            else bplpt[p] = (bplpt[p] & 0xffff) | ((uint32_t)v << 16);
        } else if (reg >= 0x140 && reg < 0x180) {
            native_sprite_uses++; WARN(12, "FINDING: the game wrote sprite registers directly - sprites are not implemented in the slice runtime\n");
        } else if (reg >= 0x180 && reg < 0x1c0) {
            palette[(reg - 0x180) >> 1] = v & 0xfff;
        } else if (reg >= 0x0a0 && reg < 0x0e0) {
            int n = (reg - 0xa0) >> 4, r = reg & 0xf;
            switch (r) {
            case 0x0: aud[n].lc = (aud[n].lc & 0xffff) | ((uint32_t)v << 16); break;
            case 0x2: aud[n].lc = (aud[n].lc & 0xffff0000) | (v & 0xfffe); break;
            case 0x4: aud[n].len = v; break;
            case 0x6: aud[n].per = v; break;
            case 0x8: aud[n].vol = v; break;
            case 0xa: aud[n].dat = v;
                if (!((dmacon & 0x200) && (dmacon & (1 << n)))) { intreq |= (uint16_t)(0x80 << n); ipl_changed(); }
                break;
            }
        }
        break;
    }
}

void chip_reset(void) {
    memset(custom, 0, sizeof custom); intena = intreq = dmacon = 0;
    memset(cia, 0, sizeof cia); cia[0].ta = cia[0].tb = cia[1].ta = cia[1].tb = 0xffff;
    cia[0].ta_latch = cia[0].tb_latch = cia[1].ta_latch = cia[1].tb_latch = 0xffff;
    cia[0].icrmask = 0x08;   /* kbfix (Cowork 17 Oct): CIA-A ICR mask SP on, as the OS keyboard driver leaves it (twin of chipset.c chip_reset) */
    kbd_busy = kbd_hs = kbd_lines = 0;
    line = 0; frame_base_cyc = cpu_now ? cpu_now() : 0;
    memset(bplpt, 0, sizeof bplpt); memset(palette, 0, sizeof palette);
    intent_frame_begin(0); in_progressive_init(); intent_compose_begin(&is_start, &it_start);
}
void chip_poke(int reg, uint16_t v) { write_custom(reg, v, 0); }
void chip_cia_poke(int n, int reg, uint8_t v) { cia_write(n, reg, v); }       /* a snapshot boot restores the CIA state the loader had set (e.g. the keyboard interrupt mask) */

/* g46 (Cowork 19 Oct 2026, from Gemini g46, generic form): restore both CIAs from a clean-sheet snapshot (snap_regs.txt 'cia'
 * lines, emitted into snapcfg/hooks.c by cs_snapcfg.py). Without it a snapshot boot started with reset CIAs: game 23's CIA-B CRA
 * was $00 instead of $08 (one-shot), so the VBLANK handler's TAHI write never started Timer A, the level 6 music interrupt never
 * fired and the music began 5.4 s late; CIA-A DDRA $00 instead of $03 made PRA read $FF instead of $FE. Fields are set directly
 * (no cia_write side effects: a TAHI write would start a one-shot timer). v[] order: pra prb ddra ddrb icrmask cra crb
 * [ta tb ta_latch tb_latch [icr sdr [tod tod_alarm tod_stopped]]]; older snapshots carry only the first 7.
 * Gate AMIGO_CIA_RESTORE (compile-time, default 1); the environment variable AMIGO_CIA_RESTORE=0 turns it off for an A/B run. */
#ifndef AMIGO_CIA_RESTORE
#define AMIGO_CIA_RESTORE 1
#endif
void chipset_restore_cia(int n, const uint32_t *v, int nv) {
    if (!AMIGO_CIA_RESTORE || n < 0 || n > 1 || nv < 7) return;
    { const char *e = getenv("AMIGO_CIA_RESTORE"); if (e && e[0] == '0') return; }
    cia_t *c = &cia[n];
    c->pra = (uint8_t)v[0]; c->prb = (uint8_t)v[1]; c->ddra = (uint8_t)v[2]; c->ddrb = (uint8_t)v[3];
    c->icrmask = (uint8_t)(v[4] & 0x1f); c->cra = (uint8_t)(v[5] & ~0x10); c->crb = (uint8_t)(v[6] & ~0x10);
    if (nv >= 11) { c->ta = (uint16_t)v[7]; c->tb = (uint16_t)v[8]; c->ta_latch = (uint16_t)v[9]; c->tb_latch = (uint16_t)v[10]; }
    if (nv >= 13) { c->icr = (uint8_t)(v[11] & 0x1f); c->sdr = (uint8_t)v[12]; }
    if (nv >= 16) { c->tod = v[13] & 0xffffff; c->tod_alarm = v[14] & 0xffffff; c->tod_stopped = v[15] ? 1 : 0; c->tod_latched = 0; }
}

int chip_contention(void) { return 0; }

/* ---------------- memory ---------------- */
#ifdef MEMLOG   /* test builds: record every RAM byte write */
#define LOGB(a, p) memlog_byte((a), (p))
#else
#define LOGB(a, p) ((void)0)
#endif
static inline int is_chip(uint32_t a) { return a < 0x200000; }
static inline int is_slow(uint32_t a) { return a >= 0xC00000 && a < 0xD80000; }

extern uint32_t pc_ring[16]; extern int pc_ring_n;
static struct { uint32_t addr, site; uint64_t n; } dec_hist[400]; static int dec_n; static int dec_on = -1;
static void dec_dump(void) { fprintf(stderr, "DECODER accesses (register-relative, not yet named calls), by address and last chunk entry:\n"); for (int i = 0; i < dec_n; i++) fprintf(stderr, "DECODER  %06X from code page %06X x%llu\n", dec_hist[i].addr, dec_hist[i].site, (unsigned long long)dec_hist[i].n); }
static void dec_access(uint32_t a) {      /* called when the address decoder (not a named call) serves a custom or CIA access */
    if (dec_on < 0) { dec_on = getenv("DECSTAT") != NULL || getenv("STRICT_HW") != NULL; if (dec_on) atexit(dec_dump); }
    if (!dec_on) return;
    uint32_t site = (pc_ring[(pc_ring_n + 15) & 15] & 0xffffff) & ~0xFFu;      /* the 256-byte code page of the last chunk entry */
    if (getenv("STRICT_HW")) { fprintf(stderr, "FINDING: hardware access %06X through the address decoder (chunk entry %06X, tick %llu)\n", a, site, (unsigned long long)chip_frame_count); exit(4); }
    int i; for (i = 0; i < dec_n; i++) if (dec_hist[i].addr == (a & ~1u) && dec_hist[i].site == site) break;
    if (i == dec_n && dec_n < 400) { dec_hist[dec_n].addr = a & ~1u; dec_hist[dec_n].site = site; dec_hist[dec_n].n = 0; dec_n++; }
    if (i < dec_n) dec_hist[i].n++;
}
static uint32_t watch_lo = 1, watch_hi;       /* WATCH=hex: report writes that touch [WATCH, WATCH+4) with the tick and the last chunk entries (dev only) */
static void watch_hit(uint32_t a, uint32_t v, int n) {
    if (watch_lo == 1) { const char *e = getenv("WATCH"); watch_lo = e ? (uint32_t)strtoul(e, 0, 16) : 0; watch_hi = watch_lo + 4; const char *c = e ? strchr(e, ',') : 0; if (c) watch_hi = (uint32_t)strtoul(c + 1, 0, 16); }
    if (!watch_lo || a + n <= watch_lo || a >= watch_hi) return;
    fprintf(stderr, "WATCH write %d bytes at %06X value %X tick %llu; last chunk entries:", n, a, v, (unsigned long long)chip_frame_count);
    for (int i = 10; i < 16; i++) fprintf(stderr, " %06X", pc_ring[(pc_ring_n + i) & 15]); fprintf(stderr, "\n");
}
uint32_t mem_rb(uint32_t a) {
    a &= 0xffffff;
    if (is_chip(a)) return chipram[a & CHIP_MASK];
    if (is_slow(a)) return slowram[SLOW_IDX(a)];
    if ((a & 0xfff000) == 0xdff000) { dec_access(a); uint16_t w = read_custom((int)(a & 0x1fe)); return (a & 1) ? (w & 0xff) : (w >> 8); }
    if ((a & 0xff0000) == 0xbf0000) {
        dec_access(a);
        if ((a & 0x1001) == 0x0001) return cia_read(0, (a >> 8) & 15);
        if ((a & 0x2001) == 0x0000) return cia_read(1, (a >> 8) & 15);
        return 0xff;
    }
    return 0;
}
uint32_t mem_rw(uint32_t a) {
    a &= 0xffffff;
    if (is_chip(a)) { a &= CHIP_WMASK; return (uint32_t)(chipram[a] << 8 | chipram[a + 1]); }
    if (is_slow(a)) { a = (SLOW_IDX(a) & ~1u); return (uint32_t)(slowram[a] << 8 | slowram[a + 1]); }
    if ((a & 0xfff000) == 0xdff000) { dec_access(a); return read_custom((int)(a & 0x1fe)); }
    return (mem_rb(a) << 8) | mem_rb(a + 1);
}
uint32_t mem_rl(uint32_t a) { return (mem_rw(a) << 16) | mem_rw(a + 2); }
void mem_wb(uint32_t a, uint32_t v) {
    a &= 0xffffff; watch_hit(a, v & 0xff, 1);
    if (is_chip(a)) { LOGB(a & CHIP_MASK, &chipram[a & CHIP_MASK]); chipram[a & CHIP_MASK] = (uint8_t)v; return; }
    if (is_slow(a)) { LOGB(CHIP_SIZE + (SLOW_IDX(a)), &slowram[SLOW_IDX(a)]); slowram[SLOW_IDX(a)] = (uint8_t)v; return; }
    if ((a & 0xfff000) == 0xdff000) { /* byte write to custom: both halves get the byte */
        dec_access(a); write_custom((int)(a & 0x1fe), (uint16_t)((v & 0xff) | ((v & 0xff) << 8)), 0); return; }
    if ((a & 0xff0000) == 0xbf0000) {
        if ((a & 0x1001) == 0x0001) cia_write(0, (a >> 8) & 15, (uint8_t)v);
        else if ((a & 0x2001) == 0x0000) cia_write(1, (a >> 8) & 15, (uint8_t)v);
    }
}
void mem_ww(uint32_t a, uint32_t v) {
    a &= 0xffffff; watch_hit(a, v & 0xffff, 2);
    if (is_chip(a)) { a &= CHIP_WMASK; LOGB(a, &chipram[a]); LOGB(a + 1, &chipram[a + 1]); chipram[a] = (uint8_t)(v >> 8); chipram[a + 1] = (uint8_t)v; return; }
    if (is_slow(a)) { a = (SLOW_IDX(a) & ~1u); LOGB(CHIP_SIZE + a, &slowram[a]); LOGB(CHIP_SIZE + 1 + a, &slowram[a + 1]); slowram[a] = (uint8_t)(v >> 8); slowram[a + 1] = (uint8_t)v; return; }
    if ((a & 0xfff000) == 0xdff000) { dec_access(a); write_custom((int)(a & 0x1fe), (uint16_t)v, 0); return; }
    mem_wb(a, v >> 8); mem_wb(a + 1, v);
}
void mem_wl(uint32_t a, uint32_t v) { mem_ww(a, v >> 16); mem_ww(a + 2, v); }

/* test support: FNV-1a hashes of RAM and of the frame */
uint64_t hash_ram(void) { uint64_t h = 1469598103934665603ull; for (int i = 0; i < CHIP_SIZE; i++) h = (h ^ chipram[i]) * 1099511628211ull; for (int i = 0; i < SLOW_SIZE; i++) h = (h ^ slowram[i]) * 1099511628211ull; return h; }
/* frame hash = sum(pixel_i * W_i) mod 2^64, W_i = splitmix64(i); easy to reproduce with numpy */
uint64_t hash_frame(void) {
    uint64_t h = 0;
    for (int i = 0; i < SCR_W * SCR_H; i++) {
        uint64_t z = (uint64_t)i * 0x9E3779B97F4A7C15ull + 0x9E3779B97F4A7C15ull;
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ull; z = (z ^ (z >> 27)) * 0x94D049BB133111EBull; z ^= z >> 31;
        h += (uint64_t)(framebuf[i] & 0xffffff) * z;
    }
    return h;
}


void hw_cia(int n, int reg, int wr) { if (hw_on < 0) hw_init(); if (hw_on) { if (wr) hw_ciawr[n][reg & 15]++; else hw_ciard[n][reg & 15]++; } }

/* named hardware entry points (generated hwcalls.h calls these): the register shadow, reached without an address decode */
uint16_t hw_r16(int reg) { hw_named++; return read_custom(reg & 0x1fe); }
void hw_w16(int reg, uint16_t v) { hw_named++; write_custom(reg & 0x1fe, v, 0); }
uint8_t hw_ciar(int n, int reg) { hw_named++; return cia_read(n, reg & 15); }
/* G2E10 rngseed (Claude sub-agent, 27 Oct 2026; Karl 27 Oct, TEST-ONLY): AMIGO_RNG_REPLAY=file (written by cs_rngtable.py from the reference
 * harness's RNGREC) lists read sites "PC KEY n v1 .. vn" (hex). A read at a listed site still does the real read (side effects such as
 * the TOD latch stay), then returns the reference's next value for that site; when a site's list is used up, the live value again.
 * Without the variable (every shipped port, every run outside G2/G3) nothing is loaded and every read is the live value. */
typedef struct { uint32_t pc; uint16_t key; uint32_t n, i; uint32_t *v; } rngsite_t;
static rngsite_t *rng_s; static int rng_n = -1; uint64_t rng_replayed;
static void rng_load(void) {
    rng_n = 0; const char *p = getenv("AMIGO_RNG_REPLAY"); FILE *f = p && *p ? fopen(p, "r") : 0; if (!f) return;
    unsigned pc, key, n; int cap = 0;
    while (fscanf(f, "%x %x %x", &pc, &key, &n) == 3) {
        if (rng_n == cap) { cap = cap ? cap * 2 : 64; rng_s = realloc(rng_s, cap * sizeof *rng_s); }
        rngsite_t *s = &rng_s[rng_n]; s->pc = pc; s->key = (uint16_t)key; s->n = n; s->i = 0; s->v = malloc((n ? n : 1) * sizeof(uint32_t));
        for (unsigned k = 0; k < n; k++) { unsigned x = 0; if (fscanf(f, "%x", &x) != 1) { n = k; break; } s->v[k] = x; }
        s->n = n; rng_n++;
    }
    fclose(f); fprintf(stderr, "RNGREPLAY %d read sites from %s\n", rng_n, p);
}
static int rng_replay(uint32_t pc, int key, uint32_t *v) {
    if (rng_n < 0) rng_load();
    for (int k = 0; k < rng_n; k++) if (rng_s[k].pc == pc && rng_s[k].key == key) {
        if (rng_s[k].i >= rng_s[k].n) return 0;
        *v = rng_s[k].v[rng_s[k].i++]; rng_replayed++; return 1; }
    return 0;
}
uint16_t hw_r16_at(int reg, uint32_t pc) { uint16_t v = hw_r16(reg); uint32_t r; if (rng_n != 0 && rng_replay(pc & 0xffffff, reg & 0x1fe, &r)) v = (uint16_t)r; return v; }
uint8_t hw_ciar_at(int n, int reg, uint32_t pc) { uint8_t v = hw_ciar(n, reg); uint32_t r; if (rng_n != 0 && rng_replay(pc & 0xffffff, 0x100 * (n + 1) + (reg & 15), &r)) v = (uint8_t)r; return v; }
void hw_ciaw(int n, int reg, uint8_t v) { hw_named++; cia_write(n, reg & 15, v); }

/* Cowork 17 Oct: restore the custom chips from a clean-sheet snapshot (see chipset.c twin). Without it the native port started with
 * no copper list, no bitplane pointers and a black palette: 50 of 89 'picture-fail' games were a black port against a lit original. */
/* G3E1 audrestore (Claude sub-agent 26 Oct 2026): a snapshot restore loads Paula's channel state (LC, LEN, PER, VOL) from the restored
 * register shadow; before, a restored channel played chip RAM from 0 at volume 0 until the game rewrote it. Gate AMIGO_AUD_RESTORE (=0 off). */
static int aud_restore_on(void) { const char *e = getenv("AMIGO_AUD_RESTORE"); return !(e && e[0] == '0'); }
static void aud_restore_shadow(void) {
    if (!aud_restore_on()) return;
    for (int n = 0; n < 4; n++) {
        const uint16_t *r = &custom[(0xa0 >> 1) + n * 8];
        aud[n].lc = ((uint32_t)r[0] << 16) | (r[1] & 0xfffe); aud[n].len = r[2]; aud[n].per = r[3]; aud[n].vol = r[4];
    }
}
void chipset_restore_words(const uint16_t *c, uint16_t ie, uint16_t dm) {
    for (int i = 0; i < 256; i++) custom[i] = c[i];
    intena = ie; dmacon = dm;
    cop1lc = (((uint32_t)c[0x80 >> 1] << 16) | c[0x82 >> 1]) & CHIP_WMASK; cop2lc = (((uint32_t)c[0x84 >> 1] << 16) | c[0x86 >> 1]) & CHIP_WMASK;
    for (int k = 0; k < 32; k++) palette[k] = c[(0x180 >> 1) + k] & 0xfff;
    for (int k = 0; k < 6; k++) bplpt[k] = (((uint32_t)c[(0xe0 >> 1) + k * 2] << 16) | (c[(0xe0 >> 1) + k * 2 + 1] & 0xfffe)) & CHIP_WMASK;
    aud_restore_shadow();   /* G3E1 audrestore */
    audio_dmacon(0, dmacon);   /* dsfix 18 Oct (G10): start the Paula channels whose DMA is on in the snapshot; without it restored ports were silent (game 22: picture 100%, sound 0.000) */
}

/* ==========================================================================
 * intentblit.inc - native blit CLASSES (Cowork 17 Oct 2026, from Gemini job 9, checked and corrected).
 * Not a blitter emulator: each BLTSIZE write is classified by what it achieves (clear, copy, shifted copy,
 * masked bob draw, area fill, line draw) and done directly on chip RAM. Anything else is a counted FINDING.
 * Evidence: blits_00201/00263/00279.txt (1,200 blits). Corrections to the Gemini draft, measured from the logs:
 *  - 1CFC is USEA|USEB with NO destination (use nibble $C): it writes nothing (a test/delay blit), not a wipe.
 *  - ECS BLTSIZH ($05E) starts the blit with the height from BLTSIZV ($05C); $05C itself starts nothing.
 * Included at the end of intentrt.c (uses chipram, custom[], native_unhandled, chip_frame_count, line).
 * ========================================================================== */
static inline uint16_t ib_r16(uint32_t a) { a &= CHIP_WMASK; return (uint16_t)((chipram[a] << 8) | chipram[a + 1]); }
static inline void ib_w16(uint32_t a, uint16_t v) { a &= CHIP_WMASK; chipram[a] = (uint8_t)(v >> 8); chipram[a + 1] = (uint8_t)v; }
uint64_t blit_class_counts[8];

static void op_blit_clear(uint32_t d, int16_t dmod, uint16_t h, uint16_t w, uint16_t fill) {
    int32_t stride = (int32_t)w * 2 + dmod;
    for (uint16_t y = 0; y < h; y++) {
        uint32_t p = d;
        for (uint16_t x = 0; x < w; x++) { ib_w16(p, fill); p += 2; }
        d = (uint32_t)((int32_t)d + stride) & CHIP_WMASK;
    }
}
static void op_blit_copy(uint32_t s, uint32_t d, int16_t smod, int16_t dmod, uint16_t h, uint16_t w, uint16_t fwm, uint16_t lwm) {
    for (uint16_t y = 0; y < h; y++) {
        for (uint16_t x = 0; x < w; x++) {
            uint16_t m = 0xFFFF; if (x == 0) m &= fwm; if (x == w - 1) m &= lwm;
            ib_w16(d, ib_r16(s) & m); s += 2; d += 2;
        }
        s = (uint32_t)((int32_t)s + smod) & CHIP_WMASK; d = (uint32_t)((int32_t)d + dmod) & CHIP_WMASK;
    }
}
/* D = (A shifted) | const, ascending (right shift) or descending (left shift); the shifter carries across rows as on the original */
static void op_blit_shift(uint32_t a, uint32_t d, int16_t amod, int16_t dmod, uint16_t h, uint16_t w, uint16_t fwm, uint16_t lwm, int ash, int desc, uint16_t orc) {
    uint16_t prev = 0;
    for (uint16_t y = 0; y < h; y++) {
        for (uint16_t x = 0; x < w; x++) {
            uint16_t m = 0xFFFF; if (x == 0) m &= fwm; if (x == w - 1) m &= lwm;
            uint16_t cur = ib_r16(a) & m, out;
            if (!desc) out = (uint16_t)((((uint32_t)prev << 16) | cur) >> ash);
            else       out = (uint16_t)((((uint32_t)cur << 16) | prev) >> (16 - ash));
            if (desc && ash == 0) out = cur;
            prev = cur; ib_w16(d, (uint16_t)(out | orc));
            a = desc ? a - 2 : a + 2; d = desc ? d - 2 : d + 2;
        }
        a = (uint32_t)((int32_t)a + (desc ? -amod : amod)) & CHIP_WMASK; d = (uint32_t)((int32_t)d + (desc ? -dmod : dmod)) & CHIP_WMASK;
    }
}
/* masked bob/font draw: D = (A & B) | (~A & C) with A (mask) and B (image) shifted */
static void op_blit_cookie(uint32_t a, uint32_t b, uint32_t c, uint32_t d, int16_t amod, int16_t bmod, int16_t cmod, int16_t dmod,
                           uint16_t h, uint16_t w, uint16_t fwm, uint16_t lwm, int ash, int bsh, uint16_t adat, uint16_t bdat, int usea, int useb) {
    uint16_t pa = 0, pb = 0;
    for (uint16_t y = 0; y < h; y++) {
        for (uint16_t x = 0; x < w; x++) {
            uint16_t m = 0xFFFF; if (x == 0) m &= fwm; if (x == w - 1) m &= lwm;
            uint16_t ca = (usea ? ib_r16(a) : adat) & m, cb = useb ? ib_r16(b) : bdat, cc = ib_r16(c);
            uint16_t sa = (uint16_t)((((uint32_t)pa << 16) | ca) >> ash), sb = (uint16_t)((((uint32_t)pb << 16) | cb) >> bsh);
            pa = ca; pb = cb;
            ib_w16(d, (uint16_t)((sa & sb) | ((uint16_t)~sa & cc)));
            if (usea) a += 2; if (useb) b += 2; c += 2; d += 2;
        }
        if (usea) a = (uint32_t)((int32_t)a + amod) & CHIP_WMASK; if (useb) b = (uint32_t)((int32_t)b + bmod) & CHIP_WMASK;
        c = (uint32_t)((int32_t)c + cmod) & CHIP_WMASK; d = (uint32_t)((int32_t)d + dmod) & CHIP_WMASK;
    }
}

/* G2E4 genblit (Claude sub-agent 25 Oct 2026): class 8, the general blit data path for every non-line blit the classes above do
 * not cover. D = minterm(A, B, C) per bit (LF bit 7 = ABC ... bit 0 = ~A~B~C); A and B pass the barrel shifter with the previous
 * word (ascending: right shift; descending: left shift, pointers and modulos run backwards); FWM/LWM mask A; a disabled channel
 * reads BLTxDAT; area fill (IFE/EFE, FCI) runs per row on the D data, bit 0 first. Pointers end where the hardware leaves them. */
static uint16_t ib_minterm(int lf, uint16_t a, uint16_t b, uint16_t c) {
    uint16_t r = 0, na = (uint16_t)~a, nb = (uint16_t)~b, nc = (uint16_t)~c;
    if (lf & 0x80) r |= a & b & c;   if (lf & 0x40) r |= a & b & nc;  if (lf & 0x20) r |= a & nb & c;  if (lf & 0x10) r |= a & nb & nc;
    if (lf & 0x08) r |= na & b & c;  if (lf & 0x04) r |= na & b & nc; if (lf & 0x02) r |= na & nb & c; if (lf & 0x01) r |= na & nb & nc;
    return r;
}
static void op_blit_generic(uint16_t con0, uint16_t con1, uint32_t a, uint32_t b, uint32_t c, uint32_t d, int16_t amod, int16_t bmod, int16_t cmod, int16_t dmod,
                            uint16_t h, uint16_t w, uint16_t fwm, uint16_t lwm, uint16_t adat, uint16_t bdat, uint16_t cdat) {
    int ash = (con0 >> 12) & 15, bsh = (con1 >> 12) & 15, use = (con0 >> 8) & 15, lf = con0 & 0xFF, desc = (con1 >> 1) & 1;
    int usea = (use >> 3) & 1, useb = (use >> 2) & 1, usec = (use >> 1) & 1, used = use & 1;
    int fill = (con1 & 0x18) != 0, ife = (con1 >> 3) & 1, fci = (con1 >> 2) & 1;
    int step = desc ? -2 : 2;
    uint16_t pa = 0, pb = 0;
    for (uint16_t y = 0; y < h; y++) {
        int fc = fci;
        for (uint16_t x = 0; x < w; x++) {
            uint16_t m = 0xFFFF; if (x == 0) m &= fwm; if (x == w - 1) m &= lwm;
            uint16_t ca = (usea ? ib_r16(a) : adat) & m, cb = useb ? ib_r16(b) : bdat, cc = usec ? ib_r16(c) : cdat, sa, sb;
            if (!desc) { sa = (uint16_t)((((uint32_t)pa << 16) | ca) >> ash); sb = (uint16_t)((((uint32_t)pb << 16) | cb) >> bsh); }
            else { sa = (uint16_t)((((uint32_t)ca << 16) | pa) >> (16 - ash)); sb = (uint16_t)((((uint32_t)cb << 16) | pb) >> (16 - bsh)); }
            pa = ca; pb = cb;
            uint16_t out = ib_minterm(lf, sa, sb, cc);
            if (fill) {
                uint16_t in = out;
                for (int i = 0; i < 16; i++) {
                    uint16_t bit = (uint16_t)(1u << i);
                    if (fc) out = ife ? (uint16_t)(out | bit) : (uint16_t)(out ^ bit);
                    if (in & bit) fc = !fc;
                }
            }
            if (used) ib_w16(d, out);
            if (usea) a = (uint32_t)((int32_t)a + step) & CHIP_WMASK; if (useb) b = (uint32_t)((int32_t)b + step) & CHIP_WMASK;
            if (usec) c = (uint32_t)((int32_t)c + step) & CHIP_WMASK; if (used) d = (uint32_t)((int32_t)d + step) & CHIP_WMASK;
        }
        if (usea) a = (uint32_t)((int32_t)a + (desc ? -amod : amod)) & CHIP_WMASK; if (useb) b = (uint32_t)((int32_t)b + (desc ? -bmod : bmod)) & CHIP_WMASK;
        if (usec) c = (uint32_t)((int32_t)c + (desc ? -cmod : cmod)) & CHIP_WMASK; if (used) d = (uint32_t)((int32_t)d + (desc ? -dmod : dmod)) & CHIP_WMASK;
    }
    custom[0x050 >> 1] = (uint16_t)(a >> 16); custom[0x052 >> 1] = (uint16_t)a; custom[0x04C >> 1] = (uint16_t)(b >> 16); custom[0x04E >> 1] = (uint16_t)b;
    custom[0x048 >> 1] = (uint16_t)(c >> 16); custom[0x04A >> 1] = (uint16_t)c; custom[0x054 >> 1] = (uint16_t)(d >> 16); custom[0x056 >> 1] = (uint16_t)d;
}

/* G2E6 lineblit (Claude sub-agent 25 Oct 2026): blitter line mode, ported unchanged from the reference harness's chipset.c
 * do_line_blit (Bresenham per the Hardware Reference Manual) so the port draws what the reference draws. */
static void op_blit_line(uint16_t con0, uint16_t con1, uint16_t len) {
    int mt = con0 & 0xff, ash = con0 >> 12;
    int oct = (con1 >> 2) & 7, sing = con1 & 2;
    int16_t err = (int16_t)custom[0x52 >> 1];
    int16_t inc1 = (int16_t)custom[0x62 >> 1], inc2 = (int16_t)custom[0x64 >> 1];
    int16_t mod = (int16_t)custom[0x60 >> 1];
    uint32_t pc = (((uint32_t)custom[0x48 >> 1] << 16) | custom[0x4A >> 1]) & CHIP_WMASK;
    uint32_t pd = (((uint32_t)custom[0x54 >> 1] << 16) | custom[0x56 >> 1]) & CHIP_WMASK;
    uint16_t adat = custom[0x74 >> 1], bdat = custom[0x72 >> 1];
    int bsh = con1 >> 12;
    uint16_t pat = (uint16_t)((bdat << bsh) | (bdat >> ((16 - bsh) & 15)));
    int dotdone = 0;
    for (int i = 0; i < len; i++) {
        uint16_t c = ib_r16(pc);
        uint16_t a = (uint16_t)(adat >> ash);
        uint16_t b = (pat & 0x8000) ? 0xffff : 0;
        uint16_t d = 0;
        for (int k = 0; k < 8; k++) if (mt & (1 << k)) {
            uint16_t t = (k & 4) ? a : (uint16_t)~a; t &= (k & 2) ? b : (uint16_t)~b; t &= (k & 1) ? c : (uint16_t)~c; d |= t;
        }
        if (!(sing && dotdone)) ib_w16(pd, d);
        dotdone = 1;
        pd = pc;
        int sud = oct & 4, sul = oct & 2, aul = oct & 1;
        int xstep = 0, ystep = 0;
        if (err >= 0) {
            if (sud) { ystep = sul ? -1 : 1; xstep = aul ? -1 : 1; }
            else { xstep = sul ? -1 : 1; ystep = aul ? -1 : 1; }
            err = (int16_t)(err + inc2);
        } else {
            if (sud) ystep = sul ? -1 : 1; else xstep = sul ? -1 : 1;
            err = (int16_t)(err + inc1);
        }
        if (xstep) { ash += xstep; if (ash > 15) { ash = 0; pc += 2; } else if (ash < 0) { ash = 15; pc -= 2; } }
        if (ystep) { pc += ystep * mod; dotdone = 0; }
        pat = (uint16_t)((pat << 1) | (pat >> 15));
    }
    custom[0x48 >> 1] = (uint16_t)(pc >> 16); custom[0x4A >> 1] = (uint16_t)pc; custom[0x54 >> 1] = (uint16_t)(pc >> 16); custom[0x56 >> 1] = (uint16_t)pc;
}

/* classify one blit by intent; returns 1 when done natively, 0 when it is a counted finding */
static int intent_blit_dispatch(uint16_t h, uint16_t w) {
    uint16_t con0 = custom[0x040 >> 1], con1 = custom[0x042 >> 1], fwm = custom[0x044 >> 1], lwm = custom[0x046 >> 1];
    uint32_t c = (((uint32_t)custom[0x048 >> 1] << 16) | custom[0x04A >> 1]) & CHIP_WMASK;
    uint32_t b = (((uint32_t)custom[0x04C >> 1] << 16) | custom[0x04E >> 1]) & CHIP_WMASK;
    uint32_t a = (((uint32_t)custom[0x050 >> 1] << 16) | custom[0x052 >> 1]) & CHIP_WMASK;
    uint32_t d = (((uint32_t)custom[0x054 >> 1] << 16) | custom[0x056 >> 1]) & CHIP_WMASK;
    int16_t cmod = (int16_t)custom[0x060 >> 1], bmod = (int16_t)custom[0x062 >> 1], amod = (int16_t)custom[0x064 >> 1], dmod = (int16_t)custom[0x066 >> 1];
    uint16_t bdat = custom[0x072 >> 1], adat = custom[0x074 >> 1];
    int ash = (con0 >> 12) & 15, bsh = (con1 >> 12) & 15, use = (con0 >> 8) & 15, lf = con0 & 0xFF, desc = (con1 >> 1) & 1;
    if (con1 & 1) { blit_class_counts[6]++; op_blit_line(con0, con1, h); return 1; }   /* line mode: G2E6 lineblit (length = BLTSIZE height) */
    if (con1 & 0x18) { blit_class_counts[5]++; op_blit_generic(con0, con1, a, b, c, d, amod, bmod, cmod, dmod, h, w, fwm, lwm, adat, bdat, custom[0x070 >> 1]); return 1; }   /* area fill: G2E4 genblit */
    if (!(use & 1)) { blit_class_counts[7]++; return 1; }                    /* no destination (e.g. 1CFC): writes nothing */
    if (use == 1 && (lf == 0x00 || lf == 0xFF || lf == 0xF0)) {              /* class 1: clear / constant fill */
        blit_class_counts[1]++; op_blit_clear(d, dmod, h, w, lf == 0x00 ? 0 : lf == 0xFF ? 0xFFFF : adat); return 1;
    }
    if (use == 9 && lf == 0xF0 && ash == 0 && !desc) { blit_class_counts[2]++; op_blit_copy(a, d, amod, dmod, h, w, fwm, lwm); return 1; }   /* class 2: copy A */
    if (use == 5 && lf == 0xCC && bsh == 0 && !desc) { blit_class_counts[2]++; op_blit_copy(b, d, bmod, dmod, h, w, 0xFFFF, 0xFFFF); return 1; }   /* class 2: copy B */
    if (use == 9 && lf == 0xF0) { blit_class_counts[3]++; op_blit_shift(a, d, amod, dmod, h, w, fwm, lwm, ash, desc, 0); return 1; }   /* class 3: shifted copy */
    if ((use & 3) == 3 && lf == 0xCA && !desc) {                            /* class 4: cookie-cut bob / font */
        blit_class_counts[4]++; op_blit_cookie(a, b, c, d, amod, bmod, cmod, dmod, h, w, fwm, lwm, ash, bsh, adat, bdat, (use >> 3) & 1, (use >> 2) & 1); return 1;
    }
    blit_class_counts[0]++;   /* G2E4 genblit: class 8, the general data path (was an unclassified-blit finding, not done) */
    op_blit_generic(con0, con1, a, b, c, d, amod, bmod, cmod, dmod, h, w, fwm, lwm, adat, bdat, custom[0x070 >> 1]);
    return 1;
}

int intent_blit_dispatch_ext(uint16_t h, uint16_t w) { return intent_blit_dispatch(h, w); }
