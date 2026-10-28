/* Ami-Go port kit (generic; from the Ghosts 'n Goblins pilot, where it found and proved the translator).
 * Differential test: every translated instruction vs the Musashi 68000 interpreter.
 * For each instruction, TRIALS random register/flag states are tried. Address registers point into a
 * noise-filled RAM window so memory operands are random too. Instructions whose operands are fixed
 * hardware addresses (custom chips / CIA) are skipped (they have side effects, not comparable).
 * Compared after one instruction: D0-D7, A0-A7, PC, X N Z V C, S, interrupt mask, every RAM byte written. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "m68k.h"
#include "cpu.h"
#include "boot.h"
#include "run.h"

typedef struct { uint32_t pc; int (*fn)(void); const char *name; int abs_hw; } single_t;
extern const single_t singles[]; extern const int n_singles;
cpu_t cpu; uint64_t CYC, EVT, ICNT;
uint32_t cpu_exception(int vec, uint32_t pc) {
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc); A[7] -= 2; mem_ww(A[7], sr);
    return mem_rl((uint32_t)vec * 4);
}
int game_is_translated(uint32_t pc) { (void)pc; return 1; }
chunk_fn game_chunk(uint32_t pc) { (void)pc; return 0; }
uint64_t stat_interp_instr, stat_interp_calls, stat_chunk_calls; uint32_t interp_pcs[4096]; int n_interp_pcs;

/* game data: the embedded files (gamedata.c) */
typedef struct { const char *name; const unsigned char *data; unsigned len; } gfile_t;
extern const gfile_t game_files[]; extern const int game_nfiles;
const uint8_t *gamefile(const char *name, size_t *len) {
    for (int i = 0; i < game_nfiles; i++) if (!strcmp(game_files[i].name, name)) { *len = game_files[i].len; return game_files[i].data; }
    for (int i = 0; i < game_nfiles; i++) if (amigo_streq_ci(game_files[i].name, name)) { *len = game_files[i].len; return game_files[i].data; }
    return NULL;
}
/* the translator's view (boot_gen.c): code loaded at run time and code the game copies */
typedef struct { uint32_t addr; const uint8_t *data; uint32_t len; } bootseg_t;
extern const bootseg_t view_segs[]; extern const int view_nsegs;
extern const uint32_t view_copies[][3]; extern const int view_ncopies;
extern const uint32_t dt_win[2];

/* RAM write log with undo */
static uint32_t la[4096]; static uint8_t lo[4096]; static int nl;
void memlog_byte(uint32_t a, uint8_t *p) { if (nl < 4096) { la[nl] = a; lo[nl] = *p; nl++; } }
static uint8_t *ramp(uint32_t a) { return a < 0x80000 ? &chipram[a] : &slowram[a - 0x80000]; }
static void undo(void) { while (nl > 0) { nl--; *ramp(la[nl]) = lo[nl]; } }

unsigned int m68k_read_memory_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_memory_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_memory_32(unsigned int a) { return mem_rl(a); }
void m68k_write_memory_8(unsigned int a, unsigned int v) { mem_wb(a, v); }
void m68k_write_memory_16(unsigned int a, unsigned int v) { mem_ww(a, v); }
void m68k_write_memory_32(unsigned int a, unsigned int v) { mem_wl(a, v); }
unsigned int m68k_read_disassembler_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_disassembler_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_disassembler_32(unsigned int a) { return mem_rl(a); }
void harness_instr_hook(unsigned int pc) { (void)pc; }

static uint64_t rng = 88172645463325252ull;
static uint32_t rnd(void) { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; return (uint32_t)rng; }
static uint64_t now0(void) { return 0; }

typedef struct { uint32_t d[8], a[8], pc, ccr, s, im, osp; uint32_t wa[64]; uint8_t wv[64]; int nw; } snap_t;
static void take(snap_t *s) {
    memcpy(s->d, cpu.dr, 32); memcpy(s->a, cpu.ar, 32); s->pc = cpu.PC; s->ccr = get_sr() & 0x1f; s->s = (uint32_t)cpu.S; s->im = (uint32_t)cpu.IM; s->osp = cpu.OSP;
    s->nw = 0;
    for (int i = 0; i < nl && s->nw < 64; i++) { s->wa[s->nw] = la[i]; s->wv[s->nw] = *ramp(la[i]); s->nw++; }
}

int main(int argc, char **argv) {
    int trials = argc > 1 ? atoi(argv[1]) : 16;
    cpu_now = now0;
    uint32_t regs[16], pc; uint16_t sr;
    if (boot_setup(regs, &pc, &sr)) { fprintf(stderr, "boot failed\n"); return 1; }
    for (int i = 0; i < view_nsegs; i++) for (uint32_t j = 0; j < view_segs[i].len; j++) mem_wb(view_segs[i].addr + j, view_segs[i].data[j]);
    for (int i = 0; i < view_ncopies; i++) for (uint32_t j = 0; j < view_copies[i][2]; j++) mem_wb(view_copies[i][1] + j, mem_rb(view_copies[i][0] + j));
    uint32_t cw = dt_win[0], sw = dt_win[1];
    for (uint32_t a = cw; a < cw + 0x10000; a++) mem_wb(a, (uint8_t)rnd());
    if (sw) for (uint32_t a = sw; a < sw + 0x10000; a++) mem_wb(a, (uint8_t)rnd());
    printf("noise windows $%06X%s", cw, sw ? "" : "\n"); if (sw) printf(" and $%06X\n", sw);
    m68k_init(); m68k_set_cpu_type(M68K_CPU_TYPE_68000); m68k_pulse_reset(); m68k_set_reg(M68K_REG_PC, pc); m68k_execute(1);
    int tested = 0, skipped = 0, failed_insns = 0, cyc_insns = 0; long fails = 0, runs = 0, cyc_fails = 0;
    static int failcount_by_name[1]; (void)failcount_by_name;
    for (int i = 0; i < n_singles; i++) {
        const single_t *t = &singles[i];
        if (t->abs_hw) { skipped++; continue; }
        int bad = 0, cyc_bad = 0;
        for (int k = 0; k < trials; k++) {
            uint32_t d[8], a[8]; uint32_t ccr = rnd() & 0x1f;
            for (int j = 0; j < 8; j++) d[j] = rnd();
            if (k & 1) for (int j = 0; j < 8; j++) d[j] = (rnd() % 7) - 3 + ((rnd() & 1) ? 0x8000 : 0);  /* small values / edges */
            for (int j = 0; j < 8; j++) a[j] = ((!sw || (rnd() & 1)) ? cw + 0x4000 + (rnd() % 0x8000) : sw + 0x4000 + (rnd() % 0x8000)) & ~1u;
            uint32_t osp = cw + 0x8000 + (rnd() % 0x1000) * 2;
            /* translated */
            memcpy(cpu.dr, d, 32); memcpy(cpu.ar, a, 32); cpu.S = 1; cpu.IM = 7; cpu.OSP = osp; set_ccr(ccr); cpu.PC = t->pc; cpu.stopped = 0;
            nl = 0; CYC = 0; t->fn(); uint64_t cyc_x = CYC;
            snap_t x; take(&x); undo();
            /* Musashi */
            m68k_set_reg(M68K_REG_SR, 0x2700 | ccr);
            m68k_set_reg(M68K_REG_A7, a[7]); m68k_set_reg(M68K_REG_USP, osp);
            for (int j = 0; j < 8; j++) m68k_set_reg(M68K_REG_D0 + j, d[j]);
            for (int j = 0; j < 7; j++) m68k_set_reg(M68K_REG_A0 + j, a[j]);
            m68k_set_reg(M68K_REG_PC, t->pc);
            nl = 0; int cyc_y = m68k_execute(1);
            if ((int)cyc_x != cyc_y) {   /* cycle cost differs from the reference: timing drifts even when results match */
                cyc_fails++;
                if (!cyc_bad) { printf("CYCLES %06X %-6s op=%04X translated %d reference %d\n", t->pc, t->name, (uint16_t)mem_rw(t->pc), (int)cyc_x, cyc_y); cyc_bad = 1; }
            }
            snap_t y;
            for (int j = 0; j < 8; j++) { y.d[j] = m68k_get_reg(NULL, M68K_REG_D0 + j); y.a[j] = m68k_get_reg(NULL, M68K_REG_A0 + j); }
            uint32_t msr = m68k_get_reg(NULL, M68K_REG_SR);
            y.pc = m68k_get_reg(NULL, M68K_REG_PC); y.ccr = msr & 0x1f; y.s = (msr >> 13) & 1; y.im = (msr >> 8) & 7;
            y.osp = y.s ? m68k_get_reg(NULL, M68K_REG_USP) : m68k_get_reg(NULL, M68K_REG_ISP);
            y.nw = 0; for (int j = 0; j < nl && y.nw < 64; j++) { y.wa[y.nw] = la[j]; y.wv[y.nw] = *ramp(la[j]); y.nw++; }
            undo();
            runs++;
            uint32_t ccrmask = 0x1f;
            if (!strcmp(t->name, "") || 0) {}
            /* documented-undefined flags: ABCD/SBCD/NBCD N,V; DIVx on overflow N,Z */
            uint16_t op = (uint16_t)mem_rw(t->pc);
            if ((op & 0xb1f0) == 0x8100 || (op & 0xffc0) == 0x4800) ccrmask &= ~0x0a;
            if ((op & 0xf0c0) == 0x80c0 && (x.ccr & 2)) ccrmask &= ~0x0d;   /* overflow: N,Z undefined; C: Musashi keeps old C, 68000 PRM says cleared */
            if ((op & 0xf1c0) == 0x4180) ccrmask &= ~0x0f;      /* CHK: only N defined on trap; others undefined */
            int ok = !memcmp(x.d, y.d, 32) && !memcmp(x.a, y.a, 32) && x.pc == y.pc && ((x.ccr ^ y.ccr) & ccrmask) == 0 && x.s == y.s && x.im == y.im;
            if (ok) {
                /* compare final values of every byte either side wrote */
                for (int j = 0; j < x.nw && ok; j++) { int f = 0; for (int q = 0; q < y.nw; q++) if (y.wa[q] == x.wa[j]) { f = 1; if (y.wv[q] != x.wv[j]) ok = 0; } if (!f && x.wv[j] != *ramp(x.wa[j])) ok = 0; }
                for (int j = 0; j < y.nw && ok; j++) { int f = 0; for (int q = 0; q < x.nw; q++) if (x.wa[q] == y.wa[j]) f = 1; if (!f && y.wv[j] != *ramp(y.wa[j])) ok = 0; }
            }
            if (!ok) {
                fails++;
                if (!bad) {
                    printf("MISMATCH %06X %-6s op=%04X trial %d: pc %06X/%06X ccr %02X/%02X", t->pc, t->name, op, k, x.pc, y.pc, x.ccr, y.ccr);
                    for (int j = 0; j < 8; j++) if (x.d[j] != y.d[j]) printf(" D%d %08X/%08X", j, x.d[j], y.d[j]);
                    for (int j = 0; j < 8; j++) if (x.a[j] != y.a[j]) printf(" A%d %08X/%08X", j, x.a[j], y.a[j]);
                    printf(" writes %d/%d\n", x.nw, y.nw);
                }
                bad = 1;
            }
        }
        tested++; if (bad) failed_insns++; if (cyc_bad) cyc_insns++;
    }
    printf("cycle cost: %d instructions differ from the reference in %ld trials (timing only; G3 drift)\n", cyc_insns, cyc_fails);
    printf("instructions tested %d, skipped (fixed hardware address) %d, trials %ld, failing trials %ld, instructions with any mismatch %d\n",
           tested, skipped, runs, fails, failed_insns);
    return failed_insns ? 2 : 0;
}
