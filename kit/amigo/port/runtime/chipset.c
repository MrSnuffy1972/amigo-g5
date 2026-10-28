/* OCS chipset model for the native port: memory map, custom registers, copper, bitplane and
 * sprite display, blitter, CIA timers/keyboard/joystick, Paula register state and interrupts.
 * Written for Ghosts 'n Goblins (Paradox release) - only the features the game uses are modelled
 * carefully; unusual features are logged once. */
#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <stdlib.h>
#include "amiga.h"

uint8_t chipram[CHIP_SIZE];
uint8_t slowram[SLOW_SIZE];
uint32_t framebuf[SCR_W*SCR_H];
uint64_t (*cpu_now)(void);
uint16_t custom[256];
uint16_t intena, intreq, dmacon;
int chip_vertb_cleared;   /* G2E9 osport (from the live kit, EPOCH18 VBLANK): INTENA writes that cleared VERTB so far (oshle_vblank reads it) */
int chip_frame_done; uint64_t chip_frame_count;
int joy_up, joy_down, joy_left, joy_right, joy_fire, joy_fire2;
int mouse_lmb, mouse_rmb, mouse_x, mouse_y;   /* g39: port-1 mouse, set only by script tokens V W M */
aud_t aud[4];
uint32_t smc_writes;
static void nolog(const char *f, ...) { (void)f; }
void (*hle_log)(const char *fmt, ...) = nolog;
static void noipl(void) {}
void (*ipl_changed)(void) = noipl;

void (*dbg_read_hook)(int reg);
static int line;                 /* current beam line */
static int bzero_flag;
static uint32_t cop1lc, cop2lc, coppc; static int cop_wait, cop_halt; static uint16_t cop_w1, cop_w2;
static uint32_t bplpt[6];
static uint32_t sprpt[8];
static uint16_t palette[32];
static int warned[64];

/* ---- INTENT PROFILE (dev only): what the game asks of the chips. INTENT_STATS=file ---- */
static FILE *istat; static int istat_init;
static uint64_t p_wcpu[256], p_wcop[256], p_rd[256], p_ciaw[2][16], p_ciar[2][16], p_blits, p_lines, p_frame_blits, p_frame_cw, p_frame_ww;
static struct { char k[110]; uint64_t n; uint64_t words; } p_sig[600]; static int p_nsig;
static char p_aud[60][100]; static int p_naud;
static uint32_t p_lists[16]; static uint32_t p_listh[16]; static int p_nlist;
static void prof_dump(void);
static FILE *is_(void) { if (!istat_init) { istat_init = 1; const char *e = getenv("INTENT_STATS"); if (e) { istat = fopen(e, "w"); atexit(prof_dump); } } return istat; }
static void prof_sig(const char *k, uint64_t words) {
    for (int i = 0; i < p_nsig; i++) if (!strcmp(p_sig[i].k, k)) { p_sig[i].n++; p_sig[i].words += words; return; }
    if (p_nsig < 600) { snprintf(p_sig[p_nsig].k, sizeof p_sig[0].k, "%s", k); p_sig[p_nsig].n = 1; p_sig[p_nsig].words = words; p_nsig++; }
}
static void prof_blit(uint16_t size) {
    if (!is_()) return;
    uint16_t c0 = custom[0x40 >> 1], c1 = custom[0x42 >> 1]; char k[110];
    if (c1 & 1) { snprintf(k, sizeof k, "LINE con0=%04X(minterm %02X) len=%d", c0 & 0x0fff, c0 & 0xff, (size >> 6) & 0x3ff); prof_sig(k, 0); p_blits++; p_frame_blits++; return; }
    int h = (size >> 6) & 0x3ff; if (!h) h = 1024; int w = size & 0x3f; if (!w) w = 64;
    snprintf(k, sizeof k, "con0=%04X con1=%04X w=%d h=%d moda=%d modb=%d modc=%d modd=%d fwm=%04X lwm=%04X", c0 & 0x0fff, c1 & 0x0fff, w, h,
             (int16_t)custom[0x64 >> 1], (int16_t)custom[0x62 >> 1], (int16_t)custom[0x60 >> 1], (int16_t)custom[0x66 >> 1], custom[0x44 >> 1], custom[0x46 >> 1]);
    prof_sig(k, (uint64_t)w * h); p_blits++; p_frame_blits++;
}
static void prof_write(int reg, uint16_t v, int from_copper) {
    if (!is_()) return; reg &= 0x1fe;
    if (from_copper) { p_wcop[reg >> 1]++; p_frame_cw++; } else { p_wcpu[reg >> 1]++; p_frame_ww++; }
}
#include "m68k.h"
static struct { uint32_t pc; uint64_t n; int reg; } p_pcrd[64]; static int p_npcrd;
static void prof_read(int reg) { if (!is_()) return; p_rd[(reg & 0x1fe) >> 1]++;
    if ((reg & 0x1fe) == 0x004 || (reg & 0x1fe) == 0x006 || (reg & 0x1fe) == 0x01e || (reg & 0x1fe) == 0x00c) {     /* census: which code polls the beam / interrupt flags */
        uint32_t pc = m68k_get_reg(NULL, M68K_REG_PPC); int i; for (i = 0; i < p_npcrd; i++) if (p_pcrd[i].pc == pc && p_pcrd[i].reg == (reg & 0x1fe)) break;
        if (i == p_npcrd && p_npcrd < 64) { p_pcrd[p_npcrd].pc = pc; p_pcrd[p_npcrd].reg = reg & 0x1fe; p_pcrd[p_npcrd++].n = 0; }
        if (i < p_npcrd) p_pcrd[i].n++; } }
static void prof_cia(int n, int reg, int wr) { if (is_()) { if (wr) p_ciaw[n][reg & 15]++; else p_ciar[n][reg & 15]++; } }
static void prof_aud(int n) {
    if (!is_() || p_naud >= 60) return;
    snprintf(p_aud[p_naud++], 100, "frame %llu ch%d lc=%06X len=%u per=%u vol=%u", (unsigned long long)chip_frame_count, n, aud[n].lc, aud[n].len, aud[n].per, aud[n].vol);
}
static void prof_frame(uint32_t cop1) {
    if (!is_()) return;
    if (chip_frame_count < 60 || chip_frame_count % 25 == 0)
        fprintf(istat, "FRAME %llu blits=%llu copper_writes=%llu cpu_writes=%llu bplcon0=%04X dmacon=%04X intena=%04X cop1lc=%06X\n", (unsigned long long)chip_frame_count,
                (unsigned long long)p_frame_blits, (unsigned long long)p_frame_cw, (unsigned long long)p_frame_ww, custom[0x100 >> 1], dmacon, intena, cop1);
    p_frame_blits = p_frame_cw = p_frame_ww = 0;
    uint32_t h = 2166136261u, n = 0; for (uint32_t a = cop1 & 0x7fffe; n < 400; a += 4, n++) { uint16_t w1 = (uint16_t)((chipram[a & 0x7fffe] << 8) | chipram[(a & 0x7fffe) + 1]), w2 = (uint16_t)((chipram[(a + 2) & 0x7fffe] << 8) | chipram[((a + 2) & 0x7fffe) + 1]);
        h = (h ^ w1) * 16777619u; h = (h ^ w2) * 16777619u; if (w1 == 0xffff && w2 == 0xfffe) break; }
    for (int i = 0; i < p_nlist; i++) if (p_listh[i] == h) return;
    if (!(cop1 & 0x7fffe)) return;        /* COP1LC not set yet (the first frames): nothing to list */
    if (p_nlist < 16) { p_lists[p_nlist] = cop1 & 0x7fffe; p_listh[p_nlist++] = h;
        fprintf(istat, "COPPERLIST at %06X (first seen frame %llu), decoded:\n", cop1 & 0x7fffe, (unsigned long long)chip_frame_count);
        for (uint32_t a = cop1 & 0x7fffe, i = 0; i < 400; a += 4, i++) {
            uint16_t w1 = (uint16_t)((chipram[a & 0x7fffe] << 8) | chipram[(a & 0x7fffe) + 1]), w2 = (uint16_t)((chipram[(a + 2) & 0x7fffe] << 8) | chipram[((a + 2) & 0x7fffe) + 1]);
            if (w1 == 0xffff && w2 == 0xfffe) { fprintf(istat, "  END\n"); break; }
            if (!(w1 & 1)) fprintf(istat, "  MOVE %03X <- %04X\n", w1 & 0x1fe, w2);
            else fprintf(istat, "  %s line %02X col %02X (mask %04X)\n", (w2 & 1) ? "SKIP" : "WAIT", w1 >> 8, w1 & 0xfe, w2); } }
}
static void prof_dump(void) {
    if (!istat) return;
    fprintf(istat, "\nBLITS total %llu in %llu frames; distinct setups %d (count, words, setup):\n", (unsigned long long)p_blits, (unsigned long long)chip_frame_count, p_nsig);
    for (int pass = 0; pass < p_nsig && pass < 40; pass++) { int b = -1; for (int i = 0; i < p_nsig; i++) if (p_sig[i].n && (b < 0 || p_sig[i].n > p_sig[b].n)) b = i;
        if (b < 0) break; fprintf(istat, "  %llu x %llu words  %s\n", (unsigned long long)p_sig[b].n, (unsigned long long)p_sig[b].words, p_sig[b].k); p_sig[b].n = 0; }
    fprintf(istat, "\nCPU writes to custom registers (reg: count):\n"); for (int r = 0; r < 256; r++) if (p_wcpu[r]) fprintf(istat, "  %03X %llu\n", r * 2, (unsigned long long)p_wcpu[r]);
    fprintf(istat, "\nCOPPER writes (reg: count):\n"); for (int r = 0; r < 256; r++) if (p_wcop[r]) fprintf(istat, "  %03X %llu\n", r * 2, (unsigned long long)p_wcop[r]);
    fprintf(istat, "\nCPU reads of custom registers (reg: count):\n"); for (int r = 0; r < 256; r++) if (p_rd[r]) fprintf(istat, "  %03X %llu\n", r * 2, (unsigned long long)p_rd[r]);
    fprintf(istat, "\nPCs that read VPOSR(004)/VHPOSR(006)/INTREQR(01E)/JOY1(00C) (pc reg count):\n"); for (int i = 0; i < p_npcrd; i++) fprintf(istat, "  %06X %03X %llu\n", p_pcrd[i].pc, p_pcrd[i].reg, (unsigned long long)p_pcrd[i].n);
    fprintf(istat, "\nCIA register writes / reads (chip reg: count):\n");
    for (int n = 0; n < 2; n++) for (int r = 0; r < 16; r++) if (p_ciaw[n][r] || p_ciar[n][r]) fprintf(istat, "  CIA-%c reg %X: writes %llu reads %llu\n", n ? 'B' : 'A', r, (unsigned long long)p_ciaw[n][r], (unsigned long long)p_ciar[n][r]);
    fprintf(istat, "\nAUDIO channel starts (first %d):\n", p_naud); for (int i = 0; i < p_naud; i++) fprintf(istat, "  %s\n", p_aud[i]);
    fclose(istat); istat = NULL;
}
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
static int kbd_busy, kbd_hs, kbd_lines;   /* kbfix: byte in SDR awaiting the handshake, SPMODE seen high, lines waited */
static FILE *cwritelog(void);

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
/* kbfix (Cowork 17 Oct): the keyboard serial link as the hardware runs it. The keyboard shifts one byte (rol 1, inverted)
 * into CIA-A SDR and sets ICR SP (bit 3); INTREQ PORTS follows only through the ICR mask (update_cia_intreq). It then
 * WAITS for the handshake (the game drives CRA SPMODE, bit 6, high then low) before it sends the next byte, and the CIA
 * cannot receive while SPMODE is an output. With no handshake the byte stays in SDR until a timeout: 8 x 143 ms, the
 * keyboard's resync (one bit per 143 ms) taking 8 bits to change SDR on hardware, so a game that only polls SDR still
 * sees the key. Was: a new byte every 128-192 lines whether or not the game had taken the last one, so a game reading
 * once a frame or slower saw only the final release code. Called every line; with an empty queue it changes nothing. */
#define KBD_GAP_LINES 8          /* about 0.5 ms: 8 bits at the keyboard's ~60 us bit time */
#define KBD_TIMEOUT_LINES 17872  /* 8 x 143 ms at 64 us a line */
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
static FILE *cwritelog(void) { static FILE *f; static int i; if (!i) { i = 1; if (getenv("CWRITELOG")) f = fopen(getenv("CWRITELOG"), "w"); } return f; }
static FILE *creadlog(void) { static FILE *f; static int i; if (!i) { i = 1; if (getenv("CREADLOG")) f = fopen(getenv("CREADLOG"), "w"); } return f; }
static uint8_t cia_read_raw(int n, int reg);
static uint8_t cia_read(int n, int reg) {
    prof_cia(n, reg, 0);   /* CREADLOG also records CIA reads, as reg 'A'/'B' + register */
    uint8_t v = cia_read_raw(n, reg);
    FILE *rl = creadlog();
    if (rl) { fprintf(rl, "%llu %d C%c%X %02X\n", (unsigned long long)chip_frame_count, line, 'A' + n, reg, v); fflush(rl); }
    { extern void entropy_read_hook(int, int, unsigned) __attribute__((weak)); if (entropy_read_hook) entropy_read_hook(1 + n, reg, v); }   /* G2E10 rngseed */
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
            if (mouse_lmb) v &= ~0x40;     /* g39: port 1 (mouse) left button */
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
    prof_cia(n, reg, 1);
    { FILE *wl = cwritelog(); if (wl) { fprintf(wl, "%llu %d W%c%X %02X\n", (unsigned long long)chip_frame_count, line, 'A' + n, reg, v); fflush(wl); } }   /* kbfix: CWRITELOG=file, CIA writes */
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
static int beam_h(void) {
    int64_t d = (int64_t)(cpu_now() - frame_base_cyc);
    if (d < 0) d = 0;
    int h = (int)(d / 2); if (h > 0xe2) h = 0xe2;
    return h;
}

/* ---------------- blitter ---------------- */
static uint32_t blt_ptr(int r) { return (((uint32_t)custom[r] << 16) | custom[r + 1]) & CHIP_WMASK; }
static void blt_setptr(int r, uint32_t v) { custom[r] = (uint16_t)(v >> 16); custom[r + 1] = (uint16_t)v; }
static inline uint16_t crw(uint32_t a) { a &= CHIP_WMASK; return (uint16_t)((chipram[a] << 8) | chipram[a + 1]); }
#ifdef MEMLOG
void memlog_byte(uint32_t a, uint8_t *p);
#endif
static inline void cww(uint32_t a, uint16_t v) { a &= CHIP_WMASK;
#ifdef MEMLOG
    memlog_byte(a, &chipram[a]); memlog_byte(a + 1, &chipram[a + 1]);
#endif
    chipram[a] = (uint8_t)(v >> 8); chipram[a + 1] = (uint8_t)v; }
static uint64_t blit_count;
static uint64_t blit_end;      /* CPU cycle at which the current blit finishes (BBUSY until then) */
int blit_timing = 1; int blit_nasty = 0;   /* nasty-mode CPU freeze: off by default (made the frame match worse, see REPORT-G3) */
static void nostall(uint64_t c) { (void)c; }
void (*cpu_stall)(uint64_t cycles) = nostall;   /* blitter-nasty: the CPU (running from chip-bus RAM) waits */
static void do_line_blit(void);
static void do_blit(uint16_t size) {
    prof_blit(size);
    uint16_t con0 = custom[0x40 >> 1], con1 = custom[0x42 >> 1];
    if (con1 & 1) { do_line_blit(); return; }
    int h = (size >> 6) & 0x3ff; if (!h) h = 1024;
    int w = size & 0x3f; if (!w) w = 64;
    int ash = con0 >> 12, bsh = con1 >> 12, mt = con0 & 0xff;
    int usea = con0 & 0x800, useb = con0 & 0x400, usec = con0 & 0x200, used = con0 & 0x100;
    int desc = con1 & 2, fill = con1 & 0x18, fci = (con1 >> 2) & 1, efe = con1 & 0x10;
    uint32_t pa = blt_ptr(0x50 >> 1), pb = blt_ptr(0x4c >> 1), pc = blt_ptr(0x48 >> 1), pd = blt_ptr(0x54 >> 1);
    int16_t moda = (int16_t)(custom[0x64 >> 1] & 0xfffe), modb = (int16_t)(custom[0x62 >> 1] & 0xfffe);
    int16_t modc = (int16_t)(custom[0x60 >> 1] & 0xfffe), modd = (int16_t)(custom[0x66 >> 1] & 0xfffe);
    uint16_t fwm = custom[0x44 >> 1], lwm = custom[0x46 >> 1];
    uint16_t adat = custom[0x74 >> 1], bdat = custom[0x72 >> 1], cdat = custom[0x70 >> 1];
    uint16_t aold = 0, bold = 0; int inc = desc ? -2 : 2;
    int allzero = 1;
    blit_count++;
    for (int y = 0; y < h; y++) {
        int fc = fci;
        for (int x = 0; x < w; x++) {
            uint16_t a = adat, b = bdat, c = cdat;
            if (usea) { a = crw(pa); pa += inc; adat = a; }
            if (useb) { b = crw(pb); pb += inc; }
            if (usec) { c = crw(pc); pc += inc; }
            uint16_t am = a;
            if (x == 0) am &= fwm;
            if (x == w - 1) am &= lwm;
            uint16_t as, bs;
            if (!desc) {
                as = (uint16_t)((((uint32_t)aold << 16) | am) >> ash);
                bs = (uint16_t)((((uint32_t)bold << 16) | b) >> bsh);
            } else {
                as = (uint16_t)((((uint32_t)am << 16) | aold) >> (16 - ash));
                bs = (uint16_t)((((uint32_t)b << 16) | bold) >> (16 - bsh));
                if (ash == 0) as = am;
                if (bsh == 0) bs = b;
            }
            aold = am; if (useb) { bold = b; bdat = b; } else bold = b;
            uint16_t d = 0;
            for (int i = 0; i < 8; i++) if (mt & (1 << i)) {
                uint16_t t = (i & 4) ? as : (uint16_t)~as;
                t &= (i & 2) ? bs : (uint16_t)~bs;
                t &= (i & 1) ? c : (uint16_t)~c;
                d |= t;
            }
            if (fill) {
                uint16_t out = 0;
                for (int bit = 0; bit < 16; bit++) {
                    int m = (d >> bit) & 1;
                    if (efe) { out |= (uint16_t)((fc ^ m) << bit); if (m) fc ^= 1; }
                    else { if (m) fc ^= 1; out |= (uint16_t)((fc | m) << bit); }
                }
                d = out;
            }
            if (d) allzero = 0;
            if (used) { cww(pd, d); pd += inc; }
        }
        if (usea) pa += desc ? -moda : moda;
        if (useb) pb += desc ? -modb : modb;
        if (usec) pc += desc ? -modc : modc;
        if (used) pd += desc ? -modd : modd;
    }
    blt_setptr(0x50 >> 1, pa); blt_setptr(0x4c >> 1, pb); blt_setptr(0x48 >> 1, pc); blt_setptr(0x54 >> 1, pd);
    custom[0x74 >> 1] = adat; custom[0x72 >> 1] = bdat;
    bzero_flag = allzero;
    static double bscale = -1; if (bscale < 0) { const char *e = getenv("GNG_BLITSCALE"); bscale = e ? atof(e) : 1.0; }
    if (blit_timing && bscale > 0) {
        /* colour clocks per word by channel mask ABCD (same table as WinUAE's blit_cycle_diagram) */
        static const int ccpw[16] = { 2, 2, 2, 3, 3, 3, 3, 4, 2, 2, 2, 3, 3, 3, 3, 4 };
        int ccw = ccpw[(usea ? 8 : 0) | (useb ? 4 : 0) | (usec ? 2 : 0) | (used ? 1 : 0)];
        uint64_t dur = (uint64_t)((w * h * ccw * 2 + 8) * bscale);
        if ((dmacon & 0x400) && blit_nasty) { cpu_stall(dur); blit_end = cpu_now(); }   /* BLTPRI: CPU frozen for the whole blit */
        else blit_end = cpu_now() + dur;
    }
    intreq |= 0x0040;
}
static void do_line_blit(void) {
    /* Bresenham line mode, as documented in the Hardware Reference Manual */
    uint16_t con0 = custom[0x40 >> 1], con1 = custom[0x42 >> 1];
    int len = (custom[0x58 >> 1] >> 6) & 0x3ff; if (!len) len = 1024;
    int mt = con0 & 0xff, ash = con0 >> 12;
    int oct = (con1 >> 2) & 7, sing = con1 & 2;
    int16_t err = (int16_t)blt_ptr(0x50 >> 1) ; err = (int16_t)custom[0x52 >> 1];
    int16_t inc1 = (int16_t)custom[0x62 >> 1], inc2 = (int16_t)custom[0x64 >> 1];
    int16_t mod = (int16_t)custom[0x60 >> 1];
    uint32_t pc = blt_ptr(0x48 >> 1), pd = blt_ptr(0x54 >> 1);
    uint16_t adat = custom[0x74 >> 1], bdat = custom[0x72 >> 1];
    int bsh = con1 >> 12;
    uint16_t pat = (uint16_t)((bdat << bsh) | (bdat >> ((16 - bsh) & 15)));
    int dotdone = 0;
    WARN(1, "blitter line mode used\n");
    for (int i = 0; i < len; i++) {
        uint16_t c = crw(pc);
        uint16_t a = (uint16_t)(adat >> ash);
        uint16_t b = (pat & 0x8000) ? 0xffff : 0;
        uint16_t d = 0;
        for (int k = 0; k < 8; k++) if (mt & (1 << k)) {
            uint16_t t = (k & 4) ? a : (uint16_t)~a; t &= (k & 2) ? b : (uint16_t)~b; t &= (k & 1) ? c : (uint16_t)~c; d |= t;
        }
        if (!(sing && dotdone)) cww(pd, d);
        dotdone = 1;
        pd = pc;
        /* step */
        int sud = oct & 4, sul = oct & 2, aul = oct & 1;
        int xstep = 0, ystep = 0;
        /* octant decode per HRM: SUD/SUL/AUL */
        if (err >= 0) {
            /* diagonal step */
            if (sud) { ystep = sul ? -1 : 1; xstep = aul ? -1 : 1; }
            else { xstep = sul ? -1 : 1; ystep = aul ? -1 : 1; }
            err = (int16_t)(err + inc2);
        } else {
            if (sud) ystep = sul ? -1 : 1; else xstep = sul ? -1 : 1;
            err = (int16_t)(err + inc1);
        }
        if (sud) { /* y major */ }
        if (xstep) { ash += xstep; if (ash > 15) { ash = 0; pc += 2; } else if (ash < 0) { ash = 15; pc -= 2; } }
        if (ystep) { pc += ystep * mod; dotdone = 0; }
        pat = (uint16_t)((pat << 1) | (pat >> 15));
    }
    blt_setptr(0x48 >> 1, pc); blt_setptr(0x54 >> 1, pc);
    intreq |= 0x0040;
}

/* ---------------- display ---------------- */
typedef struct { int t; int reg; uint16_t val; } colchg_t;
static colchg_t colchg[256]; static int ncolchg; static int cop_in_line;
static uint8_t pix[1024];        /* colour index per lores hpos (0..0x1ff) */
static uint8_t pfmask[1024];     /* bit0: pf1 nonzero, bit1: pf2 nonzero */
static uint8_t sprpix[1024];     /* sprite colour index 16..31 or 0, high bits = pair */
static uint8_t sprpair[1024];
typedef struct { int state; uint16_t pos, ctl, da, db; int vstart, vstop, hstart, armed; } spr_t;
static spr_t spr[8];

static void write_custom(int reg, uint16_t v, int from_copper);

static uint32_t rgb12(uint16_t c) {
    uint32_t r = (c >> 8) & 15, g = (c >> 4) & 15, b = c & 15;
    return 0xff000000u | (r * 17) << 16 | (g * 17) << 8 | (b * 17);
}

static void sprite_line(int l) {
    memset(sprpix, 0, sizeof sprpix);
    if (!((dmacon & 0x220) == 0x220)) return;
#ifdef DEBUG_NOSPRITES
    return;
#endif
    int act[8] = {0};
    for (int n = 7; n >= 0; n--) {
        spr_t *s = &spr[n];
        if (l < 0x19) continue;
        if (s->state == 0) {
            uint32_t p = sprpt[n] & CHIP_WMASK;
            s->pos = crw(p); s->ctl = crw(p + 2); sprpt[n] = p + 4;
            if (s->pos == 0 && s->ctl == 0) { s->state = 3; continue; }
            s->vstart = (s->pos >> 8) | ((s->ctl & 4) << 6);
            s->vstop = (s->ctl >> 8) | ((s->ctl & 2) << 7);
            s->hstart = ((s->pos & 0xff) << 1) | (s->ctl & 1);
            s->state = 1;
        }
        if (s->state == 1 && l == s->vstart) s->state = 2;
        if (s->state == 2) {
            if (l >= s->vstop) {
                uint32_t p = sprpt[n] & CHIP_WMASK;
                s->pos = crw(p); s->ctl = crw(p + 2); sprpt[n] = p + 4;
                if (s->pos == 0 && s->ctl == 0) { s->state = 3; continue; }
                s->vstart = (s->pos >> 8) | ((s->ctl & 4) << 6);
                s->vstop = (s->ctl >> 8) | ((s->ctl & 2) << 7);
                s->hstart = ((s->pos & 0xff) << 1) | (s->ctl & 1);
                s->state = (l == s->vstart) ? 2 : 1;
                if (s->state != 2) continue;
            }
            uint32_t p = sprpt[n] & CHIP_WMASK;
            s->da = crw(p); s->db = crw(p + 2); sprpt[n] = p + 4;
        } else continue;
        act[n] = 1;
    }
    /* compose pairs; lower-numbered pairs have priority, so draw pair 3 first */
    for (int p = 3; p >= 0; p--) {
        spr_t *e = &spr[2 * p], *o = &spr[2 * p + 1];
        int ae = act[2 * p], ao = act[2 * p + 1];
        if (!ae && !ao) continue;
        int attached = ao && (o->ctl & 0x80);
        for (int i = 0; i < 16; i++) {
            int bit = 15 - i;
            if (ae) {
                int h = e->hstart + 1 + i;
                if (h >= 0 && h < 1024) {
                    int ve = ((e->da >> bit) & 1) | (((e->db >> bit) & 1) << 1);
                    if (attached) {
                        int oi = h - (o->hstart + 1), vo = 0;
                        if (oi >= 0 && oi < 16) vo = ((o->da >> (15 - oi)) & 1) | (((o->db >> (15 - oi)) & 1) << 1);
                        if (ve | vo) { sprpix[h] = (uint8_t)(16 + ((vo << 2) | ve)); sprpair[h] = (uint8_t)p; }
                    } else if (ve) { sprpix[h] = (uint8_t)(16 + p * 4 + ve); sprpair[h] = (uint8_t)p; }
                }
            }
            if (ao) {
                int h = o->hstart + 1 + i;
                if (h >= 0 && h < 1024) {
                    int vo = ((o->da >> bit) & 1) | (((o->db >> bit) & 1) << 1);
                    if (attached) {
                        int ei = h - (e->hstart + 1), ve = 0;
                        if (ae && ei >= 0 && ei < 16) ve = ((e->da >> (15 - ei)) & 1) | (((e->db >> (15 - ei)) & 1) << 1);
                        if (ve | vo) { sprpix[h] = (uint8_t)(16 + ((vo << 2) | ve)); sprpair[h] = (uint8_t)p; }
                    } else if (vo) {
                        /* even sprite of the same pair has priority over the odd one */
                        int ei = h - (e->hstart + 1), ve = 0;
                        if (ae && ei >= 0 && ei < 16) ve = ((e->da >> (15 - ei)) & 1) | (((e->db >> (15 - ei)) & 1) << 1);
                        if (!ve) { sprpix[h] = (uint8_t)(16 + p * 4 + vo); sprpair[h] = (uint8_t)p; }
                    }
                }
            }
        }
    }
}

static void copper_run_until(int l, int tmax) {
    while (cop_in_line < tmax && !cop_halt) {
        if (cop_wait) {
            int vp = l & 0xff, hp = cop_in_line;
            int wv = cop_w1 >> 8, wh = cop_w1 & 0xfe;
            int vm = 0x80 | ((cop_w2 >> 8) & 0x7f), hm = cop_w2 & 0xfe;
            if ((vp & vm) > (wv & vm)) { cop_wait = 0; }
            else if ((vp & vm) == (wv & vm)) {
                if ((hp & hm) >= (wh & hm)) cop_wait = 0;
                else {
                    /* advance to the first h that satisfies */
                    int t = hp; while (t < tmax && (t & hm) < (wh & hm)) t++;
                    cop_in_line = t; if (t >= tmax) return;
                    cop_wait = 0;
                }
            } else return;  /* wait for a later line */
            cop_in_line += 2;
            continue;
        }
        uint16_t w1 = crw(coppc), w2 = crw(coppc + 2); coppc = (coppc + 4) & CHIP_WMASK;
        cop_in_line += 4;
        if (!(w1 & 1)) {
            int reg = w1 & 0x1fe;
            if (reg < 0x40 && !(custom[0x2e >> 1] & 2)) { cop_halt = 1; return; }
            if (reg < 0x20) { cop_halt = 1; return; }
            write_custom(reg, w2, 1);
        } else {
            cop_w1 = w1; cop_w2 = w2;
            if (w2 & 1) { /* SKIP */
                int vp = l & 0xff, hp = cop_in_line;
                int wv = w1 >> 8, wh = w1 & 0xfe;
                int vm = 0x80 | ((w2 >> 8) & 0x7f), hm = w2 & 0xfe;
                if ((vp & vm) > (wv & vm) || ((vp & vm) == (wv & vm) && (hp & hm) >= (wh & hm))) coppc = (coppc + 4) & CHIP_WMASK;
            } else {
                if (w1 == 0xffff && w2 == 0xfffe) { cop_halt = 1; return; }
                cop_wait = 1;
            }
        }
    }
}

static void render_line(int l) {
    /* phase 1: copper until data fetch start */
    ncolchg = 0; cop_in_line = 0;
    uint16_t pal_start[32]; memcpy(pal_start, palette, sizeof palette);
    int copper_on = (dmacon & 0x280) == 0x280;
    int ddfstrt = custom[0x92 >> 1] & 0xfc;
    if (copper_on) copper_run_until(l, ddfstrt > 0x18 ? ddfstrt - 0x10 : 0x08);
    /* phase 2: playfield fetch */
    uint16_t con0 = custom[0x100 >> 1], con1 = custom[0x102 >> 1], con2 = custom[0x104 >> 1];
    int nplanes = (con0 >> 12) & 7; int hires = con0 & 0x8000, ham = con0 & 0x800, dpf = con0 & 0x400;
    if (nplanes > 6) nplanes = 6;
    uint16_t dws = custom[0x8e >> 1], dwp = custom[0x90 >> 1];
    int vstart = dws >> 8, vstop = (dwp >> 8) | ((dwp & 0x8000) ? 0 : 0x100);
    int hstart = dws & 0xff, hstop = (dwp & 0xff) | 0x100;
    int disp = (dmacon & 0x300) == 0x300 && l >= vstart && l < vstop && nplanes > 0;
    memset(pix, 0, sizeof pix); memset(pfmask, 0, sizeof pfmask);
    if (disp) {
        int ddfstop = custom[0x94 >> 1] & 0xfc;
        if (ddfstrt < 0x18) ddfstrt = 0x18;
        if (ddfstop > 0xd8) ddfstop = 0xd8;
        int units = ((ddfstop - ddfstrt) >> 3) + 1; if (units < 0) units = 0;
        int words = hires ? units * 2 : units;
        int first = hires ? ddfstrt * 2 + 9 : ddfstrt * 2 + 0x11;
        if (hires) WARN(2, "hires playfield used (rendered at lores resolution)\n");
        int d1 = con1 & 15, d2 = (con1 >> 4) & 15;
        static uint8_t idx[2048];
        int npix = words * 16 + 16;
        memset(idx, 0, (size_t)npix + 32);
        for (int p = 0; p < nplanes; p++) {
            uint32_t a = bplpt[p] & CHIP_WMASK;
            int dly = (p & 1) ? d2 : d1;
            if (hires) dly *= 2;
            for (int w = 0; w < words; w++) {
                uint16_t d = crw(a + w * 2);
                for (int b = 0; b < 16; b++) if (d & (0x8000 >> b)) {
                    int x = w * 16 + b + dly;
                    if (x < npix + 31) idx[x] |= (uint8_t)(1 << p);
                }
            }
            int16_t mod = (int16_t)custom[((p & 1) ? 0x10a : 0x108) >> 1];
            bplpt[p] = (a + words * 2 + mod) & CHIP_WMASK;
        }
        for (int h = hstart; h < hstop && h < 1024; h++) {
            int x = h - first; if (x < 0) continue;
            int xi = hires ? x * 2 : x;
            if (xi >= npix + 31) continue;
            uint8_t v = idx[xi];
            if (dpf) {
                int p1 = (v & 1) | ((v >> 1) & 2) | ((v >> 2) & 4);
                int p2 = ((v >> 1) & 1) | ((v >> 2) & 2) | ((v >> 3) & 4);
                pfmask[h] = (uint8_t)((p1 ? 1 : 0) | (p2 ? 2 : 0));
                if (con2 & 0x40) pix[h] = (uint8_t)(p2 ? 8 + p2 : p1);
                else pix[h] = (uint8_t)(p1 ? p1 : (p2 ? 8 + p2 : 0));
            } else {
                pix[h] = v; pfmask[h] = v ? 3 : 0;
            }
        }
    }
    sprite_line(l);
    /* phase 3: copper for the rest of the line */
    if (copper_on) copper_run_until(l, 0xe4);
    /* phase 4: compose */
    int y = l - SCR_VSTART;
    if (y < 0 || y >= SCR_H) return;
    uint16_t pal[32]; memcpy(pal, pal_start, sizeof pal);
    int ci = 0;
    int pf1p = con2 & 7, pf2p = (con2 >> 3) & 7;
    uint32_t *out = &framebuf[y * SCR_W];
    uint16_t hamc = pal[0];
    for (int x = 0; x < SCR_W; x++) {
        int h = SCR_HSTART + x;
        while (ci < ncolchg && colchg[ci].t * 2 + 1 <= h) { pal[colchg[ci].reg] = colchg[ci].val; ci++; }
        uint8_t v = pix[h];
        uint16_t c;
        if (disp && ham && nplanes == 6 && !dpf) {
            int ctl = v >> 4, dat = v & 15;
            switch (ctl) {
            case 0: hamc = pal[dat]; break;
            case 1: hamc = (uint16_t)((hamc & 0xff0) | dat); break;
            case 2: hamc = (uint16_t)((hamc & 0x0ff) | (dat << 8)); break;
            default: hamc = (uint16_t)((hamc & 0xf0f) | (dat << 4)); break;
            }
            c = hamc;
        } else if (v >= 32) {
            c = (uint16_t)((pal[v & 31] >> 1) & 0x777);
        } else c = pal[v];
        if (sprpix[h] && h >= hstart && h < hstop && l >= vstart && l < vstop) {   /* sprites only show inside the display window */
            int pair = sprpair[h];
            int front;
            if (dpf) {
                int m = pfmask[h];
                front = 1;
                if ((m & 1) && pair >= pf1p) front = 0;
                if ((m & 2) && pair >= pf2p) front = 0;
            } else front = !pfmask[h] || pair < pf1p;   /* single playfield: PF1P (verified against FS-UAE cut-scene frame) */
#ifdef DEBUG_SPRFRONT
            front = 1;
#endif
            if (front) c = pal[sprpix[h] & 31];
        }
        out[x] = rgb12(c);
    }
    while (ci < ncolchg) { ci++; }
}

/* ---------------- audio (state + interrupt timing) ---------------- */
static int aud_on[4]; static int aud_irq_pending;
static void aud_start(int n); static void aud_stop(int n);
static void audio_dmacon(uint16_t old, uint16_t now) {
    for (int n = 0; n < 4; n++) {
        int was = (old & 0x200) && (old & (1 << n)), is = (now & 0x200) && (now & (1 << n));
        if (is && !was) { aud_on[n] = 1; aud_start(n); aud_irq_pending |= 1 << n; }   /* raised at the channel's next DMA slot (end of line) */
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
static int intent_on = -1;

/* g40 INTENT_SWEEP (diagnostic, Cowork 19 Oct 2026, Gemini g40 adapted): runs only when INTENT_CHECK and INTENT_SWEEP are BOTH set
 * (INTENT_SWEEP = log path). For every frame where intent.c's end-of-frame picture differs from the beam it writes
 *   FRAME <intent frame> HF <harness frame> BAD <px> PBAD <px> BOX x0 x1 y0 y1 FLAGS ...
 * PBAD is the same compare for a PROGRESSIVE compose (each line composed from chip RAM as the beam leaves it, the g20c bufix):
 * BAD > 0 with PBAD = 0 measures "buffer changed after display" directly. FLAGS name things intent.c models coarsely or not at all.
 * On the first bad frame at or after INTENT_SWEEP_FROM a SNAPSHOT block (frame-start registers, the copper table and the CPU register writes of that frame). */
static int isw_on = -1, isw_first = 1; static FILE *isw_log; static uint64_t isw_frames, isw_bad_frames, isw_from;   /* INTENT_SWEEP_FROM: first harness frame eligible for the SNAPSHOT (the judged window start) */
static uint32_t isw_pbuf[SCR_W * SCR_H], isw_buf[SCR_W * SCR_H];
static int isw_mid(uint16_t lo, uint16_t hi, int *n) {   /* writes to [lo,hi] after line 0, copper table and CPU writes */
    int c = 0; for (int i = 0; i < it_start.nchg; i++) if (it_start.chg[i].reg >= lo && it_start.chg[i].reg <= hi && it_start.chg[i].line > 0) c++;
    for (int i = 0; i < ncpuw; i++) if (cpuw[i].reg >= lo && cpuw[i].reg <= hi && cpuw[i].line > 0) c++;
    if (n) *n = c; return c > 0;
}
static int isw_any(uint16_t lo, uint16_t hi, int mode) {   /* mode 0: copper table and CPU writes, 1: CPU only, 2: copper table only */
    int c = 0; if (mode != 1) for (int i = 0; i < it_start.nchg; i++) if (it_start.chg[i].reg >= lo && it_start.chg[i].reg <= hi) c++;
    if (mode != 2) for (int i = 0; i < ncpuw; i++) if (cpuw[i].reg >= lo && cpuw[i].reg <= hi) c++;
    return c;
}
static int isw_bit(uint16_t reg, uint16_t bit) {   /* any write of reg with bit set this frame */
    for (int i = 0; i < it_start.nchg; i++) if (it_start.chg[i].reg == reg && (it_start.chg[i].val & bit)) return 1;
    for (int i = 0; i < ncpuw; i++) if (cpuw[i].reg == reg && (cpuw[i].val & bit)) return 1;
    return 0;
}
static void isw_summary(void) {
    if (!isw_log) return;
    fprintf(isw_log, "SWEEPEND frames %llu bad_frames %llu\n", (unsigned long long)isw_frames, (unsigned long long)isw_bad_frames);
    fclose(isw_log); isw_log = NULL;
}
static void isw_frame_end(void) {
    if (!isw_log) { isw_log = fopen(getenv("INTENT_SWEEP"), "w"); if (isw_log) atexit(isw_summary); const char *e = getenv("INTENT_SWEEP_FROM"); isw_from = e ? strtoull(e, NULL, 0) : 0; }
    uint64_t b0 = in_pix_bad, fr = in_frames, pbad = 0;
    intent_compose_to(chipram, 312, isw_pbuf);                  /* finish the progressive picture before intent_frame_end resets ic */
    for (int i = 0; i < SCR_W * SCR_H; i++) if (isw_pbuf[i] != framebuf[i]) pbad++;
    intent_frame_end();
    uint64_t bad = in_pix_bad - b0; isw_frames++;
    if (!isw_log || (!bad && !pbad)) return;
    isw_bad_frames += bad ? 1 : 0;
    intent_compose(chipram, &is_start, &it_start, isw_buf);
    int x0 = SCR_W, x1 = -1, y0 = SCR_H, y1 = -1;
    for (int y = 0; y < SCR_H; y++) for (int x = 0; x < SCR_W; x++) if (isw_buf[y * SCR_W + x] != framebuf[y * SCR_W + x]) {
        if (x < x0) x0 = x; if (x > x1) x1 = x; if (y < y0) y0 = y; if (y > y1) y1 = y; }
    uint16_t con0 = is_start.reg[0x100 >> 1]; int nb, nc, nd;
    isw_mid(0xe0, 0xf6, &nb); isw_mid(0x100, 0x100, &nc); isw_mid(0x8e, 0x94, &nd);
    fprintf(isw_log, "FRAME %llu HF %llu BAD %llu PBAD %llu BOX %d %d %d %d FLAGS dpf=%d lace=%d con0mid=%d diwddfmid=%d bplptmid=%d cpubplpt=%d spr=%d sprw=%d skips=%d copjmp=%d cpucop=%d\n",
            (unsigned long long)fr, (unsigned long long)chip_frame_count, (unsigned long long)bad, (unsigned long long)pbad, x0, x1, y0, y1,
            (con0 & 0x400) ? 1 : isw_bit(0x100, 0x400) ? 2 : 0, (con0 & 4) ? 1 : isw_bit(0x100, 4) ? 2 : 0, nc, nd, nb, isw_any(0xe0, 0xf6, 1),
            (dmacon & 0x220) == 0x220, isw_any(0x120, 0x17e, 0), it_start.skips, isw_any(0x88, 0x8a, 2), isw_any(0x88, 0x8a, 1));   /* copjmp: COPJMP strobe in the list (not followed); cpucop: CPU COPJMP strobe mid-frame */
    if (isw_first && bad && chip_frame_count >= isw_from) { isw_first = 0;
        fprintf(isw_log, "SNAPSHOT frame %llu HF %llu\n", (unsigned long long)fr, (unsigned long long)chip_frame_count);
        fprintf(isw_log, "  BPLCON0 %04X BPLCON1 %04X BPLCON2 %04X DIWSTRT %04X DIWSTOP %04X DDFSTRT %04X DDFSTOP %04X BPL1MOD %04X BPL2MOD %04X\n",
                con0, is_start.reg[0x102 >> 1], is_start.reg[0x104 >> 1], is_start.reg[0x8e >> 1], is_start.reg[0x90 >> 1], is_start.reg[0x92 >> 1],
                is_start.reg[0x94 >> 1], is_start.reg[0x108 >> 1], is_start.reg[0x10a >> 1]);
        fprintf(isw_log, "  BPLPT %06X %06X %06X %06X %06X %06X DMACON %04X COP1LC %06X\n", is_start.bpl[0], is_start.bpl[1], is_start.bpl[2],
                is_start.bpl[3], is_start.bpl[4], is_start.bpl[5], dmacon, cop1lc);
        fprintf(isw_log, "  COPPER %d changes %d callbacks %d skips\n", it_start.nchg, it_start.ncb, it_start.skips);
        for (int i = 0; i < it_start.nchg && i < 300; i++) fprintf(isw_log, "  CHG line %d hpos %d reg %03X val %04X\n", it_start.chg[i].line, it_start.chg[i].hpos, it_start.chg[i].reg, it_start.chg[i].val);
        fprintf(isw_log, "  CPU %d writes\n", ncpuw);
        for (int i = 0; i < ncpuw && i < 300; i++) fprintf(isw_log, "  CPUW line %d reg %03X val %04X\n", cpuw[i].line, cpuw[i].reg, cpuw[i].val);
        fprintf(isw_log, "SNAPEND\n"); }
    fflush(isw_log);
}
void chip_end_line(void) {
    render_line(line);
    if (isw_on > 0) intent_compose_to(chipram, line + 1, isw_pbuf);   /* g40 INTENT_SWEEP progressive compose */
    audio_line();
    /* CIA-A ticks at E clock (CPU/10): 45.4 per line; CIA-B TOD counts lines */
    for (int n = 0; n < 2; n++) {
        cia[n].eacc += CYC_PER_LINE / 10.0;
        int t = (int)cia[n].eacc; cia[n].eacc -= t;
        cia_tick(n, t);
    }
    cia_tod(1);
    kbd_service();   /* kbfix: every line (was every 64th), handshake and timeout are line-timed */
    frame_base_cyc += CYC_PER_LINE;
    line++;
    if (line >= LINES_PER_FRAME) {
        if (intent_on < 0) { intent_on = getenv("INTENT_CHECK") != NULL; if (intent_on) atexit(intent_summary); isw_on = intent_on && getenv("INTENT_SWEEP") != NULL; }
        if (intent_on && chip_frame_count > 0) { if (isw_on > 0) isw_frame_end(); else intent_frame_end(); }
        line = 0; chip_frame_done = 1; chip_frame_count++; prof_frame(cop1lc);
        if (intent_on) intent_frame_begin(cop1lc);
        if (isw_on > 0) intent_compose_begin(&is_start, &it_start);   /* g40: progressive compose of the new frame */
        intreq |= 0x0020;                       /* VERTB */
        { extern void oshle_vblank(void) __attribute__((weak)); if (oshle_vblank) oshle_vblank(); }   /* G2E9 osport (live kit EPOCH18 VBLANK): what Kickstart does at vertical blank; inert without a fake-OS boot */
        cia_tod(0);
        coppc = cop1lc; cop_wait = 0; cop_halt = 0;
        for (int n = 0; n < 8; n++) spr[n].state = 0;
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


static uint16_t read_custom_raw(int reg);
/* CREADLOG=file: every custom register read (frame line reg value), the same in the port and the harness because both
 * use this file; tools/ diff of the two logs shows the first read where the two runs part (timing divergence finder) */
static uint16_t read_custom(int reg) {
    prof_read(reg);
    uint16_t v = read_custom_raw(reg);
    FILE *rl = creadlog();
    if (rl) { fprintf(rl, "%llu %d %03X %04X\n", (unsigned long long)chip_frame_count, line, reg, v); fflush(rl); }
    { extern void entropy_read_hook(int, int, unsigned) __attribute__((weak)); if (entropy_read_hook) entropy_read_hook(0, reg, v); }   /* G2E10 rngseed (harness.c RNGREC) */
    return v;
}
static uint16_t read_custom_raw(int reg) {
    if (dbg_read_hook) dbg_read_hook(reg);
    switch (reg) {
    case 0x002: return (uint16_t)((dmacon & 0x3ff) | (bzero_flag ? 0x2000 : 0) | (cpu_now() < blit_end ? 0x4000 : 0));
    case 0x004: return (uint16_t)(0x8000 | ((line >> 8) & 1));
    case 0x006: return (uint16_t)(((line & 0xff) << 8) | beam_h());
    case 0x00a: return (uint16_t)(((mouse_y & 0xff) << 8) | (mouse_x & 0xff));   /* g39: JOY0DAT mouse counters (0 until M) */
    case 0x00c: return joydat();
    case 0x00e: return 0;
    case 0x010: return custom[0x9e >> 1];
    case 0x016: return (uint16_t)(0xff00 & ~(joy_fire2 ? 0x4000 : 0) & ~(mouse_rmb ? 0x0400 : 0));   /* g39: W = port-1 right button */
    case 0x018: return 0x3000;
    case 0x01a: return 0;
    case 0x01c: return intena;
    case 0x01e: return intreq;
    case 0x088: coppc = cop1lc; cop_wait = 0; cop_halt = 0; return 0;
    case 0x08a: coppc = cop2lc; cop_wait = 0; cop_halt = 0; return 0;
    }
    return 0xffff;
}

static void write_custom(int reg, uint16_t v, int from_copper) {
    reg &= 0x1fe; prof_write(reg, v, from_copper);
    if (!from_copper && intent_on > 0) intent_cpu_write(line, reg, v);
    custom[reg >> 1] = v;
    switch (reg) {
    case 0x080: cop1lc = (cop1lc & 0xffff) | ((uint32_t)v << 16); break;
    case 0x082: cop1lc = (cop1lc & 0xffff0000) | (v & 0xfffe); break;
    case 0x084: cop2lc = (cop2lc & 0xffff) | ((uint32_t)v << 16); break;
    case 0x086: cop2lc = (cop2lc & 0xffff0000) | (v & 0xfffe); break;
    case 0x088: coppc = cop1lc; cop_wait = 0; cop_halt = 0; break;
    case 0x08a: coppc = cop2lc; cop_wait = 0; cop_halt = 0; break;
    case 0x058: do_blit(v); break;
    case 0x096: {
        uint16_t old = dmacon;
        if (v & 0x8000) dmacon |= v & 0x7fff; else dmacon &= ~v;
        audio_dmacon(old, dmacon);
        break; }
    case 0x09a: if (v & 0x8000) intena |= v & 0x7fff; else { intena &= ~v; if (v & 0x0020) chip_vertb_cleared++; } ipl_changed(); break;   /* G2E9 osport: VERTB clears counted (live kit EPOCH18) */
    case 0x09c: if (v & 0x8000) intreq |= v & 0x7fff; else intreq &= ~v; ipl_changed();
                break;
    default:
        if (reg >= 0x0e0 && reg < 0x0f8) {
            int p = (reg - 0xe0) >> 2;
            if (reg & 2) bplpt[p] = (bplpt[p] & 0xffff0000) | (v & 0xfffe);
            else bplpt[p] = (bplpt[p] & 0xffff) | ((uint32_t)v << 16);
        } else if (reg >= 0x120 && reg < 0x140) {
            int n = (reg - 0x120) >> 2;
            if (reg & 2) sprpt[n] = (sprpt[n] & 0xffff0000) | (v & 0xfffe);
            else sprpt[n] = (sprpt[n] & 0xffff) | ((uint32_t)v << 16);
        } else if (reg >= 0x140 && reg < 0x180) {
            int n = (reg - 0x140) >> 3, r = (reg >> 1) & 3;
            if (r == 0) { spr[n].pos = v; }
            else if (r == 1) { spr[n].ctl = v; }
            WARN(3, "CPU wrote sprite POS/CTL/DATA registers directly\n");
        } else if (reg >= 0x180 && reg < 0x1c0) {
            int n = (reg - 0x180) >> 1;
            if (from_copper && ncolchg < 256) { colchg[ncolchg].t = cop_in_line; colchg[ncolchg].reg = n; colchg[ncolchg].val = v & 0xfff; ncolchg++; }
            palette[n] = v & 0xfff;
            if (!from_copper) { /* CPU write: takes effect from the next rendered line */ }
        } else if (reg >= 0x0a0 && reg < 0x0e0) {
            int n = (reg - 0xa0) >> 4, r = reg & 0xf;
            switch (r) {
            case 0x0: aud[n].lc = (aud[n].lc & 0xffff) | ((uint32_t)v << 16); break;
            case 0x2: aud[n].lc = (aud[n].lc & 0xffff0000) | (v & 0xfffe); break;
            case 0x4: aud[n].len = v; break;
            case 0x6: aud[n].per = v; break;
            case 0x8: aud[n].vol = v; break;
            case 0xa: aud[n].dat = v;
                /* non-DMA (CPU) mode: the word is "played" after one period -> audio interrupt request */
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
    cia[0].icrmask = 0x08;   /* kbfix (Cowork 17 Oct): CIA-A ICR mask SP on, the state the OS keyboard driver leaves and a WHDLoad game
                              * inherits. Measured: ThunderStrike (114) and Rampage (201) never write the CIA-A ICR mask yet install
                              * level-2 keyboard handlers (Rampage's slave chains its quit-key check in front); with the mask at 0
                              * their handlers never ran and no key reached them. No-input runs are hash-identical with it on. */
    line = 0; frame_base_cyc = cpu_now ? cpu_now() : 0; cop_halt = 1;
}

/* the loader's state at `jmp $C00000`: its copper list, display and interrupts */
/* ---------------- memory ---------------- */
#ifdef MEMLOG   /* test builds: record every RAM byte write */
#define LOGB(a, p) memlog_byte((a), (p))
#else
#define LOGB(a, p) ((void)0)
#endif
static inline int is_chip(uint32_t a) { return a < 0x200000; }
static inline int is_slow(uint32_t a) { return a >= 0xC00000 && a < 0xD80000; }

uint32_t mem_rb(uint32_t a) {
    a &= 0xffffff;
    if (is_chip(a)) return chipram[a & CHIP_MASK];
    if (is_slow(a)) return slowram[SLOW_IDX(a)];
    if ((a & 0xfff000) == 0xdff000) { uint16_t w = read_custom((int)(a & 0x1fe)); return (a & 1) ? (w & 0xff) : (w >> 8); }
    if ((a & 0xff0000) == 0xbf0000) {
        if ((a & 0x1001) == 0x0001) return cia_read(0, (a >> 8) & 15);
        if ((a & 0x2001) == 0x0000) return cia_read(1, (a >> 8) & 15);
        return 0xff;
    }
    return 0;
}
uint32_t mem_rw(uint32_t a) {
    a &= 0xffffff;
    if (is_chip(a)) { a &= CHIP_WMASK; return (uint32_t)(chipram[a] << 8 | chipram[a + 1]); }
    if (is_slow(a)) { a = SLOW_IDX(a) & ~1u; return (uint32_t)(slowram[a] << 8 | slowram[a + 1]); }
    if ((a & 0xfff000) == 0xdff000) return read_custom((int)(a & 0x1fe));
    return (mem_rb(a) << 8) | mem_rb(a + 1);
}
uint32_t mem_rl(uint32_t a) { return (mem_rw(a) << 16) | mem_rw(a + 2); }
void mem_wb(uint32_t a, uint32_t v) {
    a &= 0xffffff;
    if (is_chip(a)) { LOGB(a & CHIP_MASK, &chipram[a & CHIP_MASK]); chipram[a & CHIP_MASK] = (uint8_t)v; return; }
    if (is_slow(a)) { LOGB(0x80000 + SLOW_IDX(a), &slowram[SLOW_IDX(a)]); slowram[SLOW_IDX(a)] = (uint8_t)v; return; }
    if ((a & 0xfff000) == 0xdff000) { /* byte write to custom: both halves get the byte */
        write_custom((int)(a & 0x1fe), (uint16_t)((v & 0xff) | ((v & 0xff) << 8)), 0); return; }
    if ((a & 0xff0000) == 0xbf0000) {
        if ((a & 0x1001) == 0x0001) cia_write(0, (a >> 8) & 15, (uint8_t)v);
        else if ((a & 0x2001) == 0x0000) cia_write(1, (a >> 8) & 15, (uint8_t)v);
    }
}
void mem_ww(uint32_t a, uint32_t v) {
    a &= 0xffffff;
    if (is_chip(a)) { a &= CHIP_WMASK; LOGB(a, &chipram[a]); LOGB(a + 1, &chipram[a + 1]); chipram[a] = (uint8_t)(v >> 8); chipram[a + 1] = (uint8_t)v; return; }
    if (is_slow(a)) { a = SLOW_IDX(a) & ~1u; LOGB(0x80000 + a, &slowram[a]); LOGB(0x80001 + a, &slowram[a + 1]); slowram[a] = (uint8_t)(v >> 8); slowram[a + 1] = (uint8_t)v; return; }
    if ((a & 0xfff000) == 0xdff000) { write_custom((int)(a & 0x1fe), (uint16_t)v, 0); return; }
    mem_wb(a, v >> 8); mem_wb(a + 1, v);
}
void mem_wl(uint32_t a, uint32_t v) { mem_ww(a, v >> 16); mem_ww(a + 2, v); }

void chip_poke(int reg, uint16_t v) { write_custom(reg, v, 0); }
void chip_cia_poke(int n, int reg, uint8_t v) { cia_write(n, reg, v); }
void chip_dump_cia(FILE *f) { for (int n = 0; n < 2; n++) fprintf(f, "cia %d pra %02X prb %02X ddra %02X ddrb %02X icrmask %02X cra %02X crb %02X ta %04X tb %04X talatch %04X tblatch %04X icr %02X sdr %02X tod %06X todalarm %06X todstop %d\n", n, cia[n].pra, cia[n].prb, cia[n].ddra, cia[n].ddrb, cia[n].icrmask, cia[n].cra, cia[n].crb, cia[n].ta, cia[n].tb, cia[n].ta_latch, cia[n].tb_latch, cia[n].icr, cia[n].sdr, (unsigned)(cia[n].tod & 0xffffff), (unsigned)(cia[n].tod_alarm & 0xffffff), cia[n].tod_stopped); }   /* g46: the full CIA state (the first 7 fields are the old format) */

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

/* bus contention: number of lores bitplanes above 4 fetched on the current line (they use the CPU's even slots) */
int chip_contention(void) {
    uint16_t con0 = custom[0x100 >> 1]; int np = (con0 >> 12) & 7;
    uint16_t dws = custom[0x8e >> 1], dwp = custom[0x90 >> 1];
    int vstart = dws >> 8, vstop = (dwp >> 8) | ((dwp & 0x8000) ? 0 : 0x100);
    if ((dmacon & 0x300) != 0x300 || line < vstart || line >= vstop) return 0;
    if (con0 & 0x8000) return np > 2 ? np - 2 : 0;
    return np > 4 ? np - 4 : 0;
}

/* Cowork 17 Oct: restore the custom chips from a snapshot (clean-sheet snapshot boot). Without it the port started with no copper
 * list, no bitplane pointers and a black palette: 50 of 89 'picture-fail' games were a black port against a lit original. */
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
    cop1lc = (((uint32_t)c[0x80 >> 1] << 16) | c[0x82 >> 1]) & 0x7fffe; cop2lc = (((uint32_t)c[0x84 >> 1] << 16) | c[0x86 >> 1]) & 0x7fffe;
    coppc = cop1lc; cop_wait = 0; cop_halt = 0;
    for (int k = 0; k < 32; k++) palette[k] = c[(0x180 >> 1) + k] & 0xfff;
    for (int k = 0; k < 6; k++) bplpt[k] = (((uint32_t)c[(0xe0 >> 1) + k * 2] << 16) | (c[(0xe0 >> 1) + k * 2 + 1] & 0xfffe)) & 0x7fffe;
    for (int k = 0; k < 8; k++) sprpt[k] = (((uint32_t)c[(0x120 >> 1) + k * 2] << 16) | (c[(0x120 >> 1) + k * 2 + 1] & 0xfffe)) & 0x7fffe;
    aud_restore_shadow();   /* G3E1 audrestore */
    audio_dmacon(0, dmacon);   /* dsfix 18 Oct (G10): twin of intentrt.c */
}

/* G2E3 tlsnap (Claude sub-agent 25 Oct 2026), from the live kit (Cowork 17 Oct): custom-register state at a trackloader settle,
 * so the reference harness can boot the settled state (copper, display, DMA, interrupts) the live pre-port saved in
 * gates/G0/tl_custom.bin. Same file format as the live kit's chipset_save_state. */
void chipset_save_state(const char *path) {
    FILE *f = fopen(path, "wb"); if (!f) return;
    fwrite(custom, 2, 256, f); fwrite(&intena, 2, 1, f); fwrite(&dmacon, 2, 1, f); fwrite(&cop1lc, 4, 1, f); fwrite(&cop2lc, 4, 1, f); fclose(f);
}
int chipset_restore_state(const char *path) {
    FILE *f = fopen(path, "rb"); if (!f) return 0;
    uint16_t c[256], ie = 0, dm = 0; uint32_t c1 = 0, c2 = 0;
    size_t n = fread(c, 2, 256, f); n += fread(&ie, 2, 1, f); n += fread(&dm, 2, 1, f); n += fread(&c1, 4, 1, f); n += fread(&c2, 4, 1, f); fclose(f);
    if (n < 260) return 0;
    for (int i = 0; i < 256; i++) custom[i] = c[i];
    cop1lc = c1; cop2lc = c2; coppc = cop1lc; cop_wait = 0; cop_halt = 0;
    intena = ie; dmacon = dm;
    for (int k = 0; k < 32; k++) palette[k] = custom[0x180/2 + k] & 0xfff;
    for (int k = 0; k < 6; k++) bplpt[k] = (((uint32_t)custom[0xe0/2 + k*2] << 16) | (custom[0xe0/2 + k*2 + 1] & 0xfffe)) & 0x7fffe;
    for (int k = 0; k < 8; k++) sprpt[k] = (((uint32_t)custom[0x120/2 + k*2] << 16) | (custom[0x120/2 + k*2 + 1] & 0xfffe)) & 0x7fffe;
    if (aud_restore_on()) { aud_restore_shadow(); audio_dmacon(0, dmacon); }   /* G3E1 audrestore: start DMA-on channels, twin of chipset_restore_words */
    return 1;
}
/* G2E3 tlsnap: the live kit calls this on every disk load (trackloader settle counting); the dev kit has no disk DMA, so it is
 * only defined (NULL) for the hooks that test it (oshle_tl.c DoIO/LoadSeg/Read). */
void (*dsk_load_hook)(const char *kind, int disk, int track, uint32_t disk_off, uint32_t dst, uint32_t byte_len) = NULL;
