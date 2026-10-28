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

/* op_blit_general (Cowork 17 Oct, from Copilot's blitter review, checked): the MEANING of any block-mode blit in plain C, used
 * when no intent class matches. Not a cycle model: it does at once what the blit achieves. Order per word: fetch, A first/last
 * word masks, A/B shift through the previous word (ascending right, descending left, carried across rows), minterm, area fill
 * (bit 0 upwards, carry reset to FCI at every row as the Hardware Reference Manual describes), write D. Disabled sources use
 * BLTxDAT. Measured need: 2 of the 9 clean-sheet runs since 12:11 logged unclassified blits (LF $FC with A+D, LF $0C with
 * A+C+D at 11 shifts) and those blits wrote nothing. */
static void op_blit_general(uint32_t a, uint32_t b, uint32_t c, uint32_t d, int16_t amod, int16_t bmod, int16_t cmod, int16_t dmod,
                            uint16_t h, uint16_t w, uint16_t fwm, uint16_t lwm, uint16_t con0, uint16_t con1, uint16_t adat, uint16_t bdat, uint16_t cdat) {
    int usea = (con0 >> 11) & 1, useb = (con0 >> 10) & 1, usec = (con0 >> 9) & 1, used = (con0 >> 8) & 1;
    int ash = (con0 >> 12) & 15, bsh = (con1 >> 12) & 15, desc = (con1 >> 1) & 1, ife = (con1 >> 3) & 1, efe = (con1 >> 4) & 1, fci = (con1 >> 2) & 1;
    uint8_t lf = (uint8_t)con0; int step = desc ? -2 : 2;
    uint16_t aprev = 0, bprev = 0;
    for (uint16_t y = 0; y < h; y++) {
        unsigned fc = (unsigned)fci;
        for (uint16_t x = 0; x < w; x++) {
            uint16_t araw = usea ? ib_r16(a) : adat, braw = useb ? ib_r16(b) : bdat, cv = usec ? ib_r16(c) : cdat;
            if (x == 0) araw &= fwm;
            if (x + 1 == w) araw &= lwm;
            uint16_t av, bv;
            if (!desc) { av = ash ? (uint16_t)(((uint32_t)aprev << (16 - ash)) | (araw >> ash)) : araw;
                         bv = bsh ? (uint16_t)(((uint32_t)bprev << (16 - bsh)) | (braw >> bsh)) : braw; }
            else       { av = ash ? (uint16_t)(((uint32_t)araw << ash) | (aprev >> (16 - ash))) : araw;
                         bv = bsh ? (uint16_t)(((uint32_t)braw << bsh) | (bprev >> (16 - bsh))) : braw; }
            aprev = araw; bprev = braw;
            uint16_t na = (uint16_t)~av, nb = (uint16_t)~bv, nc = (uint16_t)~cv, dv = 0;
            if (lf & 0x01) dv |= na & nb & nc;  if (lf & 0x02) dv |= na & nb & cv;
            if (lf & 0x04) dv |= na & bv & nc;  if (lf & 0x08) dv |= na & bv & cv;
            if (lf & 0x10) dv |= av & nb & nc;  if (lf & 0x20) dv |= av & nb & cv;
            if (lf & 0x40) dv |= av & bv & nc;  if (lf & 0x80) dv |= av & bv & cv;
            if (ife || efe) {
                uint16_t o = 0;
                for (int bit = 0; bit < 16; bit++) {
                    unsigned src = (dv >> bit) & 1, r = efe ? (src ^ fc) : (src | fc);
                    if (r) o |= (uint16_t)(1u << bit);
                    if (src) fc ^= 1u;
                }
                dv = o;
            }
            if (used) ib_w16(d, dv);
            if (usea) a = (uint32_t)((int32_t)a + step) & CHIP_WMASK;  if (useb) b = (uint32_t)((int32_t)b + step) & CHIP_WMASK;
            if (usec) c = (uint32_t)((int32_t)c + step) & CHIP_WMASK;  if (used) d = (uint32_t)((int32_t)d + step) & CHIP_WMASK;
        }
        if (usea) a = (uint32_t)((int32_t)a + (desc ? -amod : amod)) & CHIP_WMASK;  if (useb) b = (uint32_t)((int32_t)b + (desc ? -bmod : bmod)) & CHIP_WMASK;
        if (usec) c = (uint32_t)((int32_t)c + (desc ? -cmod : cmod)) & CHIP_WMASK;  if (used) d = (uint32_t)((int32_t)d + (desc ? -dmod : dmod)) & CHIP_WMASK;
    }
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
    if (con1 & 1) { blit_class_counts[6]++; return 0; }                      /* line mode: class defined, not built yet (no logged use) */
    uint16_t cdat = custom[0x070 >> 1];
    if (con1 & 0x18) { blit_class_counts[5]++; op_blit_general(a, b, c, d, amod, bmod, cmod, dmod, h, w, fwm, lwm, con0, con1, adat, bdat, cdat); return 1; }   /* area fill: general kernel (Cowork 17 Oct) */
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
    blit_class_counts[0]++;   /* Cowork 17 Oct: no intent class -> the general kernel does it; still logged so frequent signatures get a class */
    { static int nf; if (nf++ < 40) fprintf(stderr, "FINDING: unclassified blit (done by the general kernel) at tick %llu line %d: con0 %04X con1 %04X size %ux%u words\n", (unsigned long long)chip_frame_count, line, con0, con1, w, h); }
    op_blit_general(a, b, c, d, amod, bmod, cmod, dmod, h, w, fwm, lwm, con0, con1, adat, bdat, cdat);
    return 1;
}
