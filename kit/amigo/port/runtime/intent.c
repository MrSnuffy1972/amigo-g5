/* intent.c: native-intent DISPLAY (clean sheet, M1.1; claude-run/NATIVE-INTENT-SPEC.md, INTENT-DESIGN.md).
 * No beam, no copper execution, no cycle timing. The copper list is READ ONCE per frame as a table of
 * "from line N, register R = V" (palette, bitplane base, scroll) plus "at line N raise interrupt bits" (callbacks);
 * the frame is composed from the screen the game set up (bitplane bases, strides, palette) line by line.
 * Handled since Cowork hamfix 18 Oct: hires (reduced to SCR_W), HAM6, EHB, mid-line copper colour changes. Not yet: dual playfield.
 * g50 (19 Oct): sprites (gate AMIGO_INTENT_SPRITES), mirroring chipset.c sprite_line and its display-window mix. */
#include <string.h>
/* intent.c compose core, Cowork hamfix 18 Oct 2026: hires, HAM6, EHB and mid-line copper colour changes (mirrors chipset.c's rules) */
typedef struct { int line, hpos, seq; uint16_t reg, val; } ichg_t;   /* hpos: copper colour clock the MOVE lands at (-1 = CPU write, time unknown) */
typedef struct { int line; uint16_t bits; } icb_t;
typedef struct { ichg_t chg[4096]; int nchg; icb_t cb[256]; int ncb; int skips; } itable_t;
typedef struct { uint16_t reg[256]; uint16_t pal[32]; uint32_t bpl[6]; uint32_t spt[8]; uint16_t dmacon; } istate_t;   /* g50: spt = running sprite DMA pointers, dmacon = effective DMACON */
static istate_t is_start; static itable_t it_start;
/* register writes made by the game's own code during the frame, stamped with the line they happen at (e.g. in a line-timed handler) */
static ichg_t cpuw[4096]; static int ncpuw;
static void intent_copper_cpu(int L, int reg, uint16_t v);
static void intent_cpu_write(int line, int reg, uint16_t v) { intent_copper_cpu(line, reg, v); if (reg >= 0x40 && ncpuw < 4096) { cpuw[ncpuw].line = line; cpuw[ncpuw].hpos = -1; cpuw[ncpuw].seq = ncpuw; cpuw[ncpuw].reg = (uint16_t)reg; cpuw[ncpuw++].val = v; } }
static uint64_t in_frames, in_pix_bad, in_pix_all, in_lines_bad, in_unsup[6];   /* seen (not unsupported any more for hires/ham/ehb): hires, ham, dpf, ehb, sprites, midline */
static FILE *in_log;
static int in_mid_on = -1, in_6pl_on = -1;   /* switches: AMIGO_INTENT_MIDLINE=0 puts copper colour moves back at line start; AMIGO_INTENT_6PL=0 clamps 6 planes to 5 as before */
static int in_hires_policy = -1;   /* hires to SCR_W: 0 LEFT (every other hires pixel, as chipset.c), 1 AVERAGE; env AMIGO_HIRES_POLICY=AVERAGE */

/* the copper list as a table. hpos follows chipset.c's copper timing: each fetch 4 colour clocks, a satisfied WAIT lands at its
 * position + 2; a MOVE's colour change shows from lores h = 2*hpos+1 (chipset.c colchg rule). The line still comes from the WAIT only. */
/* g49 (Cowork 19 Oct 2026, from Gemini g49): with AMIGO_INTENT_COPJMP on, the table follows COPJMP1/2 MOVEs ($088/$08A) to the
 * tracked COP1LC/COP2LC (MOVEs to $080-$086, starting from the frame's values), evaluates SKIP against the table's beam (skip the
 * next instruction once the position has passed) and reads up to 8000 instructions; a jump back to a target already taken at the
 * same line stops the read (the copper would only loop there until the next WAIT that can never be met). Off = the old reader. */
#ifndef AMIGO_INTENT_COPJMP
#define AMIGO_INTENT_COPJMP 1
#endif
static int in_cj_on = -1;
/* G2E7 coptime (Claude sub-agent, 26 Oct 2026): the table keeps chipset.c's copper clock and the compose step its phase split (see
 * the WAIT and compose comments below). Gate AMIGO_INTENT_COPTIME: compile default 1, environment 0 = the old reader and compose. */
#ifndef AMIGO_INTENT_COPTIME
#define AMIGO_INTENT_COPTIME 1
#endif
static int in_ct_on = -1;
static int in_coptime_on(void) { if (in_ct_on < 0) { const char *e = getenv("AMIGO_INTENT_COPTIME"); in_ct_on = e ? (e[0] != '0') : AMIGO_INTENT_COPTIME; } return in_ct_on; }
static int in_copjmp_on(void) { if (in_cj_on < 0) { const char *e = getenv("AMIGO_INTENT_COPJMP"); in_cj_on = e ? (e[0] != '0') : AMIGO_INTENT_COPJMP; } return in_cj_on; }
/* G2E9 coplc (Claude sub-agent, 27 Oct 2026): the COP1LC/COP2LC values the copper list itself MOVEs (last one in list order), recorded
 * for the table built at the start of a tick; intentrt.c starts the next tick's copper there, as chipset.c does (its copper MOVE to
 * $080/$082 sets cop1lc, and the copper restarts from cop1lc at the next vertical blank). */
static uint32_t in_ex_c1, in_ex_c2; static int in_ex_c1w, in_ex_c2w;
static uint32_t in_frame_c1, in_frame_c2; static int in_frame_c1w, in_frame_c2w;
static void intent_read_table_ex(const uint8_t *chip, uint32_t cop, uint32_t cop1, uint32_t cop2, itable_t *t, int start_line) {
    t->nchg = t->ncb = t->skips = 0; int line = start_line, hpos = 0;
    in_ex_c1w = in_ex_c2w = 0; in_ex_c1 = cop1; in_ex_c2 = cop2;   /* G2E9 coplc */
    int cj = in_copjmp_on(), lim = cj ? 8000 : 4000, skipnext = 0, njv = 0; uint32_t jv[32]; int jl[32];
    int ct = in_coptime_on();   /* G2E7 coptime */
    for (uint32_t a = cop & CHIP_WMASK, n = 0; n < (uint32_t)lim; a = (a + 4) & CHIP_WMASK, n++) {
        uint16_t w1 = (uint16_t)((chip[a] << 8) | chip[a + 1]), w2 = (uint16_t)((chip[(a + 2) & CHIP_WMASK] << 8) | chip[((a + 2) & CHIP_WMASK) + 1]);
        if (w1 == 0xffff && w2 == 0xfffe) break;
        if (ct && hpos >= 0xe4) { line++; hpos = 0; if (line > 312) break; }   /* G2E7 coptime: chipset.c runs the copper to $E4, then the next line from 0 */
        hpos += 4;
        if (skipnext) { skipnext = 0; continue; }   /* g49: a satisfied SKIP */
        if (!(w1 & 1)) {
            int reg = w1 & 0x1fe;
            if (reg == 0x80) { in_ex_c1 = (in_ex_c1 & 0xffff) | ((uint32_t)w2 << 16); in_ex_c1w = 1; } else if (reg == 0x82) { in_ex_c1 = (in_ex_c1 & 0xffff0000u) | (w2 & 0xfffe); in_ex_c1w = 1; }   /* G2E9 coplc */
            else if (reg == 0x84) { in_ex_c2 = (in_ex_c2 & 0xffff) | ((uint32_t)w2 << 16); in_ex_c2w = 1; } else if (reg == 0x86) { in_ex_c2 = (in_ex_c2 & 0xffff0000u) | (w2 & 0xfffe); in_ex_c2w = 1; }
            if (cj) {   /* g49: copper pointer moves and jumps */
                if (reg == 0x80) cop1 = (cop1 & 0xffff) | ((uint32_t)w2 << 16); else if (reg == 0x82) cop1 = (cop1 & 0xffff0000u) | (w2 & 0xfffe);
                else if (reg == 0x84) cop2 = (cop2 & 0xffff) | ((uint32_t)w2 << 16); else if (reg == 0x86) cop2 = (cop2 & 0xffff0000u) | (w2 & 0xfffe);
                else if (reg == 0x88 || reg == 0x8a) { uint32_t to = (reg == 0x88 ? cop1 : cop2) & CHIP_WMASK; int seen = 0;
                    for (int k = 0; k < njv; k++) if (jv[k] == to && jl[k] == line) seen = 1;
                    if (seen) break;
                    if (njv < 32) { jv[njv] = to; jl[njv++] = line; } else break;
                    a = (to - 4) & CHIP_WMASK; continue; }
            }
            if (reg == 0x9c) { if ((w2 & 0x8000) && t->ncb < 256) { t->cb[t->ncb].line = line; t->cb[t->ncb++].bits = w2 & 0x7fff; } }
            else if (reg >= 0x40 && t->nchg < 4096) { ichg_t *c = &t->chg[t->nchg]; c->line = line; c->hpos = hpos; c->seq = t->nchg; c->reg = (uint16_t)reg; c->val = w2; t->nchg++; }
        } else if (w2 & 1) { t->skips++;
            if (cj) { int v = w1 >> 8, h = w1 & 0xfe, vl = ct ? (line & 0xff) : line; if (vl > v || (vl == v && hpos >= h)) skipnext = 1; } }   /* g49: SKIP against the table's beam (G2E7: 8-bit line, as chipset.c) */
        else { int v = w1 >> 8, h = w1 & 0xfe;
            if (ct) {   /* G2E7 coptime: chipset.c copper_run_until. A WAIT fetched at or past $E4 is compared on the next line; the compare
                         * uses the 8-bit beam line (WAIT $FF,$DE then WAIT $1A waits for line $11A); a WAIT for h >= $E4 is met at the
                         * next line's position 2 */
                if (hpos >= 0xe4) { line++; hpos = 0; }
                int vp = line & 0xff;
                if (vp > v || (vp == v && hpos >= h)) hpos += 2;
                else { if (vp < v) line += v - vp; if (h >= 0xe4) { line++; hpos = 2; } else hpos = h + 2; }
                if (line > 312) break;
            } else {
            if (v > line) { line = v; hpos = h; } else if (v == line && h > hpos) hpos = h;
            hpos += 2; } }
    }
}
static uint32_t in_frame_cop, in_cop1, in_cop2;   /* g49: the frame's start pointer and the COP1LC/COP2LC the CPU has set since */
static void intent_read_table(const uint8_t *chip, uint32_t cop, itable_t *t) {
    in_frame_cop = cop; in_cop1 = cop; in_cop2 = ((uint32_t)is_start.reg[0x84 >> 1] << 16) | (is_start.reg[0x86 >> 1] & 0xfffe);
    intent_read_table_ex(chip, cop, in_cop1, in_cop2, t, 0);
    in_frame_c1 = in_ex_c1; in_frame_c2 = in_ex_c2; in_frame_c1w = in_ex_c1w; in_frame_c2w = in_ex_c2w;   /* G2E9 coplc */
}
static int coplc_plausible(uint32_t a) {   /* G2E9 coplc guard: a copper list that ends, not zero memory */
    a &= CHIP_WMASK; if (!(chipram[a] | chipram[a + 1] | chipram[(a + 2) & CHIP_WMASK] | chipram[((a + 2) & CHIP_WMASK) + 1])) return 0;
    for (int n = 0; n < 4000; n++, a = (a + 4) & CHIP_WMASK)
        if (chipram[a] == 0xff && chipram[a + 1] == 0xff && chipram[(a + 2) & CHIP_WMASK] == 0xff && chipram[((a + 2) & CHIP_WMASK) + 1] == 0xfe) return 1;
    return 0;
}
/* g49: the CPU wrote $080-$08A at line L mid-frame. COPJMP1/2 restarts the copper at COP1LC/COP2LC from L; a pointer write re-reads the
 * frame's list with the new pointers. Entries before L are kept (already shown or applied), the re-read supplies L onwards. */
static struct { istate_t s; int ci, wi, l; const itable_t *t; int spr312; } ic;
static itable_t it_tmp;
static void intent_copper_cpu(int L, int reg, uint16_t v) {
    if (!in_copjmp_on() || reg < 0x80 || reg > 0x8a) return;
    if (reg == 0x80) in_cop1 = (in_cop1 & 0xffff) | ((uint32_t)v << 16); else if (reg == 0x82) in_cop1 = (in_cop1 & 0xffff0000u) | (v & 0xfffe);
    else if (reg == 0x84) in_cop2 = (in_cop2 & 0xffff) | ((uint32_t)v << 16); else if (reg == 0x86) in_cop2 = (in_cop2 & 0xffff0000u) | (v & 0xfffe);
    if (reg == 0x80 || reg == 0x82) return;   /* COP1LC only matters at the next frame or a COPJMP1 (the re-read on $088 uses it) */
    if (reg == 0x88 || reg == 0x8a) intent_read_table_ex(chipram, reg == 0x88 ? in_cop1 : in_cop2, in_cop1, in_cop2, &it_tmp, L);
    else intent_read_table_ex(chipram, in_frame_cop, in_frame_cop, in_cop2, &it_tmp, 0);
    itable_t *t = &it_start; int k = 0, j = 0;
    while (k < t->nchg && t->chg[k].line < L) k++;
    for (int i = 0; i < it_tmp.nchg && k < 4096; i++) if (it_tmp.chg[i].line >= L) { t->chg[k] = it_tmp.chg[i]; t->chg[k].seq = k; k++; }
    t->nchg = k;
    while (j < t->ncb && t->cb[j].line < L) j++;
    for (int i = 0; i < it_tmp.ncb && j < 256; i++) if (it_tmp.cb[i].line >= L) t->cb[j++] = it_tmp.cb[i];
    t->ncb = j;
    if (ic.t == t && ic.ci > t->nchg) ic.ci = t->nchg;
    { int kk = 0; while (kk < t->nchg && t->chg[kk].line < L) kk++; if (ic.t == t && ic.ci > kk) ic.ci = kk; }
}
/* g50 (Cowork 19 Oct 2026, from Gemini g50, rebuilt against the real file): sprites. No beam: the sprite DMA of chipset.c sprite_line
 * is replayed per composed line. Pointers: the running SPRxPT (istate spt) starts each frame where the previous frame's compose left
 * it (first frame: the register shadow), copper MOVEs and CPU writes to $120-$13E set it at their line, each fetch advances it.
 * Per line (from line $19, while DMACON has SPREN+DMAEN): state 0 fetches POS/CTL (0/0 = done for the frame), the sprite arms at
 * VSTART, on VSTOP the next POS/CTL pair is fetched (reuse), DATA/DATB are fetched every shown line. Pairs are composed 3 to 0
 * (lower pair wins), attached pairs give 15 colours, even over odd. Mixed only inside the display window, in front of the
 * playfield when the pixel is background or the pair is below PF1P (single playfield; dual playfield uses PF1P/PF2P per plane set).
 * Gate AMIGO_INTENT_SPRITES (compile-time default below, environment overrides). Off = no sprite code runs (byte-identical). */
#ifndef AMIGO_INTENT_SPRITES
#define AMIGO_INTENT_SPRITES 1
#endif
static int in_spr_v = -1;
static int in_spr_on(void) { if (in_spr_v < 0) { const char *e = getenv("AMIGO_INTENT_SPRITES"); in_spr_v = e ? (e[0] != '0') : AMIGO_INTENT_SPRITES; } return in_spr_v; }
typedef struct { int state; uint16_t pos, ctl, da, db; int vstart, vstop, hstart; } ispr_t;
static ispr_t in_spr[8];
static uint32_t in_spt_next[8]; static int in_spt_valid;
static uint64_t in_spr_lines, in_spr_px;
static uint8_t in_sprpix[1024], in_sprpair[1024];
static inline uint16_t in_rw(const uint8_t *chip, uint32_t a) { a &= CHIP_WMASK; return (uint16_t)((chip[a] << 8) | chip[a + 1]); }
static void in_spr_poslc(const uint8_t *chip, istate_t *s, int n) {
    ispr_t *p = &in_spr[n]; uint32_t a = s->spt[n] & CHIP_WMASK;
    p->pos = in_rw(chip, a); p->ctl = in_rw(chip, a + 2); s->spt[n] = a + 4;
    p->vstart = (p->pos >> 8) | ((p->ctl & 4) << 6); p->vstop = (p->ctl >> 8) | ((p->ctl & 2) << 7); p->hstart = ((p->pos & 0xff) << 1) | (p->ctl & 1);
}
static void in_sprite_line(const uint8_t *chip, istate_t *s, int l) {
    memset(in_sprpix, 0, sizeof in_sprpix);
    if ((s->dmacon & 0x220) != 0x220) return;
    int act[8] = {0}, any = 0;
    for (int n = 7; n >= 0; n--) {
        ispr_t *p = &in_spr[n];
        if (l < 0x19) continue;
        if (p->state == 0) { in_spr_poslc(chip, s, n); if (p->pos == 0 && p->ctl == 0) { p->state = 3; continue; } p->state = 1; }
        if (p->state == 1 && l == p->vstart) p->state = 2;
        if (p->state == 2) {
            if (l >= p->vstop) { in_spr_poslc(chip, s, n); if (p->pos == 0 && p->ctl == 0) { p->state = 3; continue; }
                p->state = (l == p->vstart) ? 2 : 1; if (p->state != 2) continue; }
            uint32_t a = s->spt[n] & CHIP_WMASK; p->da = in_rw(chip, a); p->db = in_rw(chip, a + 2); s->spt[n] = a + 4;
        } else continue;
        act[n] = 1; any = 1;
    }
    if (!any) return;
    in_spr_lines++;
    for (int q = 3; q >= 0; q--) {
        ispr_t *e = &in_spr[2 * q], *o = &in_spr[2 * q + 1]; int ae = act[2 * q], ao = act[2 * q + 1];
        if (!ae && !ao) continue;
        int attached = ao && (o->ctl & 0x80);
        for (int i = 0; i < 16; i++) { int bit = 15 - i;
            if (ae) { int h = e->hstart + 1 + i;
                if (h >= 0 && h < 1024) { int ve = ((e->da >> bit) & 1) | (((e->db >> bit) & 1) << 1);
                    if (attached) { int oi = h - (o->hstart + 1), vo = 0; if (oi >= 0 && oi < 16) vo = ((o->da >> (15 - oi)) & 1) | (((o->db >> (15 - oi)) & 1) << 1);
                        if (ve | vo) { in_sprpix[h] = (uint8_t)(16 + ((vo << 2) | ve)); in_sprpair[h] = (uint8_t)q; } }
                    else if (ve) { in_sprpix[h] = (uint8_t)(16 + q * 4 + ve); in_sprpair[h] = (uint8_t)q; } } }
            if (ao) { int h = o->hstart + 1 + i;
                if (h >= 0 && h < 1024) { int vo = ((o->da >> bit) & 1) | (((o->db >> bit) & 1) << 1);
                    int ei = h - (e->hstart + 1), ve = 0; if (ae && ei >= 0 && ei < 16) ve = ((e->da >> (15 - ei)) & 1) | (((e->db >> (15 - ei)) & 1) << 1);
                    if (attached) { if (ve | vo) { in_sprpix[h] = (uint8_t)(16 + ((vo << 2) | ve)); in_sprpair[h] = (uint8_t)q; } }
                    else if (vo && !ve) { in_sprpix[h] = (uint8_t)(16 + q * 4 + vo); in_sprpair[h] = (uint8_t)q; } } }
        }
    }
}
static void intent_apply(istate_t *s, uint16_t reg, uint16_t val) {
    if (reg >= 0x180 && reg < 0x1c0) s->pal[(reg - 0x180) >> 1] = val & 0xfff;
    else if (reg >= 0xe0 && reg < 0xf8) { int p = (reg - 0xe0) >> 2; if (p < 6) s->bpl[p] = (reg & 2) ? ((s->bpl[p] & 0xffff0000u) | (val & 0xfffe)) : ((s->bpl[p] & 0xffffu) | ((uint32_t)val << 16)); }
    else { s->reg[reg >> 1] = val;
        if (reg >= 0x120 && reg < 0x140) { int n = (reg - 0x120) >> 2; s->spt[n] = (reg & 2) ? ((s->spt[n] & 0xffff0000u) | (val & 0xfffe)) : ((s->spt[n] & 0xffffu) | ((uint32_t)val << 16)); }   /* g50 */
        else if (reg == 0x96) { if (val & 0x8000) s->dmacon |= val & 0x7fff; else s->dmacon &= (uint16_t)~val; } }   /* g50: effective DMACON */
}
static inline uint16_t in_col(const uint16_t *pal, int v) { return v >= 32 ? (uint16_t)((pal[v & 31] >> 1) & 0x777) : pal[v]; }   /* EHB: 32-63 = half-bright 0-31 */
static inline uint32_t in_avg(uint32_t a, uint32_t b) { return 0xff000000u | ((((a >> 16) & 255) + ((b >> 16) & 255)) >> 1) << 16 | ((((a >> 8) & 255) + ((b >> 8) & 255)) >> 1) << 8 | (((a & 255) + (b & 255)) >> 1); }
/* compose one frame into out[SCR_W*SCR_H] (12-bit RGB turned into the frame buffer's format by rgb12) */
/* bufix 19 Oct 2026 (g20c): the frame can be composed PROGRESSIVELY, line by line as the tick's virtual beam passes each line
 * (intentrt.c tick_line_step), so each line shows chip RAM as it is when that line is reached. Composing the whole frame at the END
 * of the tick showed what the game had drawn after the display window: Xenon 2 (20) erases the buffer it has just shown and draws the
 * next one after line 258, so its title text reveal was invisible until the text was complete (content 90.3%). */
static void intent_compose_begin(const istate_t *init, const itable_t *t) { ic.s = *init; ic.ci = ic.wi = 0; ic.l = 0; ic.t = t; ic.spr312 = 0; for (int n = 0; n < 8; n++) in_spr[n].state = 0; }
static void intent_compose_to(const uint8_t *chip, int lend, uint32_t *out) {
    if (!ic.t) return;
    const itable_t *t = ic.t;
    if (in_hires_policy < 0) { const char *e = getenv("AMIGO_HIRES_POLICY"); in_hires_policy = (e && (e[0] == 'A' || e[0] == 'a')) ? 1 : 0; }
    if (in_mid_on < 0) { const char *e = getenv("AMIGO_INTENT_MIDLINE"); in_mid_on = !(e && e[0] == '0'); }
    if (in_6pl_on < 0) { const char *e = getenv("AMIGO_INTENT_6PL"); in_6pl_on = !(e && e[0] == '0'); }
    istate_t s = ic.s; int ci = ic.ci, wi = ic.wi; int spr = in_spr_on();
    static ichg_t mid[4096];
    if (lend > 312) lend = 312;
    for (int l = ic.l; l < lend; l++) {
        int nmid = 0, wi0 = wi, ndef = 0;
        /* G2E7 coptime: chipset.c render_line runs the copper until DDFSTRT-$10 (DDFSTRT as at the line's start), fetches the line, then
         * runs the rest; a non-colour MOVE of this line that executes at or after that point is applied after the line (shows from l+1) */
        int tm1 = 0x7fff; if (in_coptime_on()) { int d0 = s.reg[0x92 >> 1] & 0xfc; tm1 = d0 > 0x18 ? d0 - 0x10 : 0x08; }
        static ichg_t defr[4096];
        /* copper colour moves that land inside the visible part of THIS line take effect at their position; everything else at line start, as before */
        while (ci < t->nchg && t->chg[ci].line <= l) { const ichg_t *c = &t->chg[ci++];
            if (c->line == l && c->reg >= 0x180 && c->reg < 0x1c0 && in_mid_on && 2 * c->hpos + 1 > SCR_HSTART) mid[nmid++] = *c;
            else if (c->line == l && c->hpos >= 4 && c->hpos - 4 >= tm1 && !(c->reg >= 0x180 && c->reg < 0x1c0)) defr[ndef++] = *c;
            else intent_apply(&s, c->reg, c->val); }
        while (wi < ncpuw && cpuw[wi].line <= l) { intent_apply(&s, cpuw[wi].reg, cpuw[wi].val); wi++; }
        int y = l - SCR_VSTART;
        uint16_t con0 = s.reg[0x100 >> 1], con1 = s.reg[0x102 >> 1];
        int nplanes = (con0 >> 12) & 7, hires = con0 & 0x8000, ham = con0 & 0x800, dpf = con0 & 0x400;
        uint16_t dws = s.reg[0x8e >> 1], dwp = s.reg[0x90 >> 1];
        /* DIWSTOP vertical: bit 8 is the complement of bit 7 ($2C -> $12C, $F0 -> $0F0); do not force bit 8 */
        int vstart = dws >> 8, vstop = (dwp >> 8) | ((dwp & 0x8000) ? 0 : 0x100), hstart = dws & 0xff, hstop = (dwp & 0xff) | 0x100;
        int ddfstrt = s.reg[0x92 >> 1] & 0xfc, ddfstop = s.reg[0x94 >> 1] & 0xfc;
        if (hires && nplanes > 4) nplanes = 4;                       /* OCS hires: at most 4 planes */
        int ham6 = in_6pl_on && !hires && ham && !dpf && nplanes == 6, ehb = in_6pl_on && !hires && !ham && !dpf && nplanes == 6;
        int disp = l >= vstart && l < vstop && nplanes > 0;
        if (nplanes > 5 && !ham6 && !ehb) nplanes = 5;               /* 6-plane dual playfield (not handled yet) or BPU 7: as before, never EHB */
        static uint8_t pix[1024], pixb[1024]; memset(pix, 0, sizeof pix); if (hires) memset(pixb, 0, sizeof pixb);
        if (spr) in_sprite_line(chip, &s, l);                       /* g50: sprite DMA for this line (every line, also off screen) */
        if (disp) {
            if (hires) in_unsup[0]++; if (ham) in_unsup[1]++; if (dpf) in_unsup[2]++; if (ehb && y >= 0 && y < SCR_H) in_unsup[3]++;
            if (ddfstrt < 0x18) ddfstrt = 0x18; if (ddfstop > 0xd8) ddfstop = 0xd8;
            int units = ((ddfstop - ddfstrt) >> 3) + 1; if (units < 0) units = 0;
            int words = hires ? units * 2 : units, first = hires ? ddfstrt * 2 + 9 : ddfstrt * 2 + 0x11, npix = words * 16 + 16;
            static uint8_t idx[2048]; memset(idx, 0, (size_t)npix + 32);
            for (int p = 0; p < nplanes; p++) {
                uint32_t a = s.bpl[p] & CHIP_WMASK; int dly = (p & 1) ? ((con1 >> 4) & 15) : (con1 & 15); if (hires) dly *= 2;
                for (int w = 0; w < words; w++) { uint16_t d = (uint16_t)((chip[(a + w * 2) & CHIP_WMASK] << 8) | chip[((a + w * 2) & CHIP_WMASK) + 1]);
                    for (int b = 0; b < 16; b++) if (d & (0x8000 >> b)) { int x = w * 16 + b + dly; if (x < npix + 31) idx[x] |= (uint8_t)(1 << p); } }
                int16_t mod = (int16_t)s.reg[((p & 1) ? 0x10a : 0x108) >> 1];
                s.bpl[p] = (a + words * 2 + mod) & CHIP_WMASK;
            }
            for (int h = hstart; h < hstop && h < 1024; h++) { int x = h - first; if (x < 0) continue; int xi = hires ? x * 2 : x; if (xi >= npix + 31) continue;
                pix[h] = idx[xi]; if (hires) pixb[h] = idx[xi + 1]; }
        }
        uint16_t pal[32]; memcpy(pal, s.pal, sizeof pal); int mi = 0;
        if (nmid) in_unsup[5]++;
        if (y >= 0 && y < SCR_H) {
            uint16_t hamc = pal[0];                                  /* HAM hold: COLOR00 at the start of every line */
            for (int x = 0; x < SCR_W; x++) { int h = SCR_HSTART + x;
                while (mi < nmid && 2 * mid[mi].hpos + 1 <= h) { pal[(mid[mi].reg - 0x180) >> 1] = mid[mi].val & 0xfff; mi++; }
                uint8_t v = disp ? pix[h] : 0; uint32_t c;
                if (disp && ham6) { int dat = v & 15;
                    switch (v >> 4) { case 0: hamc = pal[dat]; break; case 1: hamc = (uint16_t)((hamc & 0xff0) | dat); break;
                                      case 2: hamc = (uint16_t)((hamc & 0x0ff) | (dat << 8)); break; default: hamc = (uint16_t)((hamc & 0xf0f) | (dat << 4)); break; }
                    c = rgb12(hamc); }
                else if (disp && hires && in_hires_policy == 1) c = in_avg(rgb12(in_col(pal, v)), rgb12(in_col(pal, pixb[h])));
                else c = rgb12(in_col(pal, v));
                if (spr && in_sprpix[h] && h >= hstart && h < hstop && l >= vstart && l < vstop) {   /* g50: chipset.c mix, display window only */
                    uint16_t con2 = s.reg[0x104 >> 1]; int pair = in_sprpair[h], front;
                    if (dpf) { int p1 = (v & 1) | ((v >> 1) & 2) | ((v >> 2) & 4), p2 = ((v >> 1) & 1) | ((v >> 2) & 2) | ((v >> 3) & 4);
                        front = 1; if (p1 && pair >= (con2 & 7)) front = 0; if (p2 && pair >= ((con2 >> 3) & 7)) front = 0; }
                    else front = !v || pair < (con2 & 7);
                    if (front) { c = rgb12(pal[in_sprpix[h] & 31]); in_spr_px++; } }
                out[y * SCR_W + x] = c; }
        }
        if (nmid) { while (mi < nmid) { pal[(mid[mi].reg - 0x180) >> 1] = mid[mi].val & 0xfff; mi++; }
            memcpy(s.pal, pal, sizeof pal);
            for (int k = wi0; k < wi; k++) if (cpuw[k].reg >= 0x180 && cpuw[k].reg < 0x1c0) intent_apply(&s, cpuw[k].reg, cpuw[k].val); }   /* CPU writes still win after the line, as before */
        for (int k = 0; k < ndef; k++) intent_apply(&s, defr[k].reg, defr[k].val);   /* G2E7 coptime: phase-3 moves, after this line's fetch */
    }
    if (spr && lend >= 312 && !ic.spr312) { ic.spr312 = 1; in_sprite_line(chip, &s, 312); }   /* g50: chipset.c also runs sprite DMA on line 312 */
    if (spr && lend >= 312) { memcpy(in_spt_next, s.spt, sizeof in_spt_next); in_spt_valid = 1; }   /* g50: the next frame starts from here */
    ic.s = s; ic.ci = ci; ic.wi = wi; if (lend > ic.l) ic.l = lend;
}
/* the whole frame at once (chipset.c INTENT_CHECK, and the old behaviour) */
static void intent_compose(const uint8_t *chip, const istate_t *init, const itable_t *t, uint32_t *out) {
    intent_compose_begin(init, t); intent_compose_to(chip, 312, out);
}
/* frame hooks, called from chipset.c when INTENT_CHECK is set: snapshot at frame start, compare at frame end */
static void intent_frame_begin(uint32_t cop) {
    for (int i = 0; i < 256; i++) is_start.reg[i] = custom[i];
    memcpy(is_start.pal, palette, sizeof palette);
    for (int p = 0; p < 6; p++) is_start.bpl[p] = bplpt[p];
    is_start.dmacon = dmacon;   /* g50 */
    for (int n = 0; n < 8; n++) is_start.spt[n] = in_spt_valid ? in_spt_next[n] : ((((uint32_t)custom[(0x120 >> 1) + n * 2] << 16) | (custom[(0x120 >> 1) + n * 2 + 1] & 0xfffe)) & CHIP_WMASK);
    intent_read_table(chipram, cop, &it_start); ncpuw = 0;
}
static void intent_frame_end(void) {
    static uint32_t buf[SCR_W * SCR_H]; if (!in_log) in_log = fopen(getenv("INTENT_CHECK"), "w");
    intent_compose(chipram, &is_start, &it_start, buf);
    uint64_t bad = 0; int badl = 0, lasty = -1;
    for (int y = 0; y < SCR_H; y++) { int lb = 0; for (int x = 0; x < SCR_W; x++) if (buf[y * SCR_W + x] != framebuf[y * SCR_W + x]) { bad++; lb = 1; } badl += lb; (void)lasty; }
    if (bad && in_log && (in_frames == 250 || in_frames == 450)) { int x0 = SCR_W, x1 = -1, y0 = SCR_H, y1 = -1, shown = 0;
        for (int y = 0; y < SCR_H; y++) for (int x = 0; x < SCR_W; x++) if (buf[y * SCR_W + x] != framebuf[y * SCR_W + x]) { if (x < x0) x0 = x; if (x > x1) x1 = x; if (y < y0) y0 = y; if (y > y1) y1 = y;
            if (shown < 6) { shown++; fprintf(in_log, "  diff at x=%d y=%d intent=%06X chip=%06X\n", x, y, buf[y * SCR_W + x] & 0xffffff, framebuf[y * SCR_W + x] & 0xffffff); } }
        fprintf(in_log, "  frame %llu mismatch box x %d..%d y %d..%d; dmacon=%04X spr0pt=%06X\n", (unsigned long long)in_frames, x0, x1, y0, y1, dmacon, 0); }
    in_frames++; in_pix_bad += bad; in_pix_all += SCR_W * SCR_H; in_lines_bad += badl;
    if (in_log && (in_frames < 40 || in_frames % 25 == 0 || bad)) { if (in_frames < 40 || in_frames % 25 == 0 || in_frames % 50 == 0) fprintf(in_log, "frame %llu mismatching pixels %llu of %d, lines %d, table: %d changes %d callbacks %d skips\n", (unsigned long long)in_frames, (unsigned long long)bad, SCR_W * SCR_H, badl, it_start.nchg, it_start.ncb, it_start.skips); }
}
static void intent_summary(void) {
    if (!in_log) return;
    fprintf(in_log, "\nSUMMARY %llu frames: %llu of %llu pixels differ (%.4f%%), %llu lines; unsupported seen: hires %llu ham %llu dpf %llu ehb %llu\n", (unsigned long long)in_frames,
            (unsigned long long)in_pix_bad, (unsigned long long)in_pix_all, in_pix_all ? 100.0 * in_pix_bad / in_pix_all : 0.0, (unsigned long long)in_lines_bad,
            (unsigned long long)in_unsup[0], (unsigned long long)in_unsup[1], (unsigned long long)in_unsup[2], (unsigned long long)in_unsup[3]);
    if (in_spr_on()) fprintf(in_log, "g50 SPRITES on: %llu lines with sprite DMA, %llu sprite pixels shown\n", (unsigned long long)in_spr_lines, (unsigned long long)in_spr_px);
    fclose(in_log); in_log = NULL;
}
