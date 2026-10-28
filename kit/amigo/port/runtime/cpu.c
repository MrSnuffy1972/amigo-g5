/* CPU scheduling for the native port: runs translated chunks, delivers interrupts at line
 * boundaries and hands any untranslated address to the fallback interpreter (Musashi). */
#include "whd.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include "cpu.h"
#include "boot.h"
#include "m68k.h"
#include "run.h"

cpu_t cpu;
uint64_t CYC, EVT;
static uint64_t next_line;
uint64_t stat_interp_instr, stat_interp_calls, stat_chunk_calls;

/* Port clock against the harness: the harness runs (int)(CYC_PER_LINE / scale) Musashi cycles per beam line (412 at
 * 1126/1024) and its clock reads Musashi cycles x scale. The port counts beam cycles (CYC), so chunk cycles are
 * stretched by CYC_PER_LINE / line_den with the remainder carried (truncating each chunk made the port about 0.4% fast
 * against the beam and moved IK+'s interrupts, 14 Oct 2026: G3 173/1500 before, 1500/1500 after). */
uint64_t cpu_slow_num = 1126;     /* 1.1 x 1024: fitted average bus-contention slowdown (G3 measurement); GNG_CPUSCALE overrides */
static uint64_t line_den(uint64_t num) { uint64_t d = (uint64_t)(CYC_PER_LINE / (num / 1024.0)); return d ? d : 1; }
static uint64_t port_now(void) {   /* harness clock: Musashi cycles x scale */
    return (uint64_t)((double)(CYC * line_den(cpu_slow_num) / CYC_PER_LINE) * (cpu_slow_num / 1024.0));
}
static void port_stall(uint64_t c) { CYC += c; }

uint32_t cpu_exception(int vec, uint32_t pc) {
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc);
    A[7] -= 2; mem_ww(A[7], sr);
    CYC += 34;
    return mem_rl((uint32_t)vec * 4);
}

static void take_irq(int lvl) {
    uint32_t pc = cpu.PC;
    {   /* IRQLOG=file: every interrupt entry (frame line level interrupted-PC), compared with the harness by irqdiff.py */
        static FILE *il; static int ili; if (!ili) { ili = 1; if (getenv("IRQLOG")) il = fopen(getenv("IRQLOG"), "w"); }
        if (il) fprintf(il, "%llu %d %d %06X\n", (unsigned long long)chip_frame_count, chip_line(), lvl, pc & 0xffffff);
    }
    uint32_t sr = get_sr();
    if (!cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = 1; }
    A[7] -= 4; mem_wl(A[7], pc);
    A[7] -= 2; mem_ww(A[7], sr);
    cpu.IM = lvl; cpu.stopped = 0;
    cpu.PC = mem_rl((uint32_t)(24 + lvl) * 4);
    CYC += 44;
}

/* ---- fallback interpreter (Musashi), only for addresses the translator did not cover ---- */
unsigned int m68k_read_memory_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_memory_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_memory_32(unsigned int a) { return mem_rl(a); }
void m68k_write_memory_8(unsigned int a, unsigned int v) { mem_wb(a, v); }
void m68k_write_memory_16(unsigned int a, unsigned int v) { mem_ww(a, v); }
void m68k_write_memory_32(unsigned int a, unsigned int v) { mem_wl(a, v); }
unsigned int m68k_read_disassembler_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_disassembler_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_disassembler_32(unsigned int a) { return mem_rl(a); }
static int interp_steps;
uint32_t interp_pcs[4096]; int n_interp_pcs;
void harness_instr_hook(unsigned int pc) {
    stat_interp_instr++;   /* every instruction Musashi executes, including the one it stops after */
    if (interp_steps++ > 0 && (game_is_translated(pc) || (whd_active && whd_hle_find(pc)))) { m68k_end_timeslice(); return; }
    if (n_interp_pcs < 4096) {
        int i; for (i = 0; i < n_interp_pcs; i++) if (interp_pcs[i] == pc) break;
        if (i == n_interp_pcs) interp_pcs[n_interp_pcs++] = pc;
    }
}
static int musashi_ready;
static void interp_run(void) {
    if (!musashi_ready) { m68k_init(); m68k_set_cpu_type(M68K_CPU_TYPE_68000); m68k_pulse_reset(); musashi_ready = 1; }
    stat_interp_calls++;
    m68k_set_reg(M68K_REG_SR, get_sr());
    if (cpu.S) { m68k_set_reg(M68K_REG_A7, A[7]); m68k_set_reg(M68K_REG_USP, cpu.OSP); }
    else { m68k_set_reg(M68K_REG_A7, A[7]); m68k_set_reg(M68K_REG_ISP, cpu.OSP); }
    for (int i = 0; i < 8; i++) m68k_set_reg(M68K_REG_D0 + i, D[i]);
    for (int i = 0; i < 7; i++) m68k_set_reg(M68K_REG_A0 + i, A[i]);
    m68k_set_reg(M68K_REG_PC, cpu.PC);
    interp_steps = 0;
    int budget = (int)(EVT > CYC ? EVT - CYC : 4);
    int c = m68k_execute(budget);
    CYC += (uint64_t)c;
    for (int i = 0; i < 8; i++) D[i] = m68k_get_reg(NULL, M68K_REG_D0 + i);
    for (int i = 0; i < 8; i++) A[i] = m68k_get_reg(NULL, M68K_REG_A0 + i);
    uint32_t sr = m68k_get_reg(NULL, M68K_REG_SR);
    cpu.S = (sr >> 13) & 1; cpu.IM = (sr >> 8) & 7; set_ccr(sr);
    cpu.OSP = cpu.S ? m68k_get_reg(NULL, M68K_REG_USP) : m68k_get_reg(NULL, M68K_REG_ISP);
    cpu.PC = m68k_get_reg(NULL, M68K_REG_PC);
}

int run_boot(void) {
    uint32_t regs[16], pc; uint16_t sr;
    cpu_now = port_now; cpu_stall = port_stall;
    memset(&cpu, 0, sizeof cpu); CYC = 0;
    if (boot_setup(regs, &pc, &sr)) return -1;
    memcpy(cpu.dr, regs, 32); memcpy(cpu.ar, regs + 8, 32);
    cpu.S = 1; cpu.IM = (sr >> 8) & 7; set_ccr(sr); cpu.OSP = whd_active ? whd_basemem - 0x400 : 0; cpu.PC = pc;   /* WHDLoad: USP = SSP - $400 */
    if (!(sr & 0x2000)) {   /* k17 oshle: an AmigaDOS program starts in user mode (its A7 is the USP); the supervisor stack is
                             * the reset vector's, as the harness's Musashi takes it at reset. Measured 14 Oct on 3D World Tennis:
                             * a decruncher using A7 as a data pointer was overwritten by interrupts on the same stack */
        cpu.S = 0; cpu.OSP = mem_rl(0);
    }
    { extern uint32_t boot_usp; extern int boot_usp_set; if (boot_usp_set) cpu.OSP = boot_usp; }
    { extern uint32_t boot_osp; if (boot_osp) cpu.OSP = boot_osp; }   /* G2E3 tlsnap: snapshot boot (live kit, Cowork 17 Oct) */
    next_line = CYC_PER_LINE;
    return 0;
}

uint64_t bpl_k = 0;                /* extra stretch per contended plane (1024 = +100%), GNG_BPLK */
void run_frame(void) {
    { static int init; if (!init) { init = 1; const char *e = getenv("GNG_CPUSCALE"); if (e) cpu_slow_num = (uint64_t)(atof(e) * 1024); e = getenv("GNG_BPLK"); if (e) bpl_k = (uint64_t)(atof(e) * 1024);
        e = getenv("GNG_NASTY"); if (e) blit_nasty = atoi(e); } }
    chip_frame_done = 0;
    while (!chip_frame_done) {
        while (CYC >= next_line && !chip_frame_done) { chip_end_line(); next_line += CYC_PER_LINE; }
        if (chip_frame_done) break;
        int l = chip_ipl();
        if (l > cpu.IM) take_irq(l);
        static uint64_t frac;   /* remainder of the scaled cycle count, carried between chunks */
        uint64_t num = cpu_slow_num + (uint64_t)chip_contention() * bpl_k, den = line_den(num);
        if (num == 1024) EVT = next_line;
        else {   /* stop the chunk where the SCALED count reaches the end of the line, not the raw count */
            uint64_t need = (next_line > CYC ? next_line - CYC : 0) * den;
            EVT = CYC + (need > frac ? (need - frac + CYC_PER_LINE - 1) / CYC_PER_LINE : 0);
        }
        if (cpu.stopped) { CYC = next_line; frac = 0; continue; }
        if (whd_active) {   /* WHDLoad resload slots (runtime/whd.c): C, entered by the slave's jsr/jmp */
            const hle_t *h = whd_hle_find(cpu.PC);
            if (h) { h->fn(); continue; }   /* no cycles charged: the harness charges none, so G3 compares like with like */
        }
        chunk_fn f = game_chunk(cpu.PC);
        stat_chunk_calls++;
        uint64_t c0 = CYC;
        if (!f || !f()) interp_run();
        if (num != 1024) {   /* bus contention model: CPU cycles stretched (planes 5/6 take even slots), remainder carried */
            uint64_t t = (CYC - c0) * CYC_PER_LINE + frac;
            CYC = c0 + t / den; frac = t % den;
        }
    }
}
