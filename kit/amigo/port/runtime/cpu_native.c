/* CPU scheduling for the NATIVE exe (clean sheet): runs translated chunks, delivers interrupts at tick-timeline boundaries. No Musashi. */
#include "whd.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include "cpu.h"
#include "boot.h"
#include "run.h"

cpu_t cpu;
uint32_t native_pc(void) { return cpu.PC & 0xffffff; }   /* c13 (19 Oct 2026): TICKPCLOG in main.c */
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

/* ---- no interpreter in a native exe: an address the translator did not cover is a FINDING and ends the run ----
 * DEV ONLY (-DAMIGO_PROBE, step nativeprobe): Musashi as a fallback, to find what the translator has not covered; never shipped. */
uint32_t interp_pcs[4096]; int n_interp_pcs;
uint32_t pc_ring[16]; int pc_ring_n;       /* the last chunk entries, printed on a finding (both builds) */
/* fix3b 19 Oct 2026 (wild-jump detection): a PC outside the port's memory map is a WILD JUMP, not code to discover. The map is
 * the chip window 000000-1FFFFF (chip RAM and its mirrors), the slow/ExpMem window C00000-D7FFFF and the HLE slots; the 68000
 * address bus is 24 bits, so a junk high byte (6A760000) is masked off before the test. 12 games filled interp_pcs[4096] with a
 * contiguous runaway through unmapped memory (15: 6A760000-6A763FFC after leaving the 005F74 wait loop). */
uint32_t wild_jump_to, wild_jump_from, wild_jump_chunk; int wild_jump_seen, wild_jump_interp, wild_jump_rom; unsigned long long wild_jump_tick;
/* wj2 19 Oct 2026: a PC in the Kickstart ROM window F80000-FFFFFF (24-bit masked, as the 68000 bus sees it) is reported as rom_call, not
 * wild_jump: it may be a real call into ROM the port does not map (54: FAFDA2 at tick 185), a different defect from a runaway. */
static int pc_mapped(uint32_t a) {
    a &= 0xffffff;
    if (a < 0x200000u || (a >= 0xC00000u && a < 0xD80000u)) return 1;
    return (hle_count || whd_active) && hle_find(a);
}
static uint32_t last_chunk_before(uint32_t a) {   /* newest chunk entry that is mapped and is not the wild target itself */
    for (int i = 15; i >= 0; i--) { uint32_t p = pc_ring[(pc_ring_n + i) & 15] & 0xffffff; if (p != (a & 0xffffff) && pc_mapped(p)) return p; }
    return 0xffffffffu;
}
static void wild_jump_note(uint32_t pc, uint32_t from, int interp) {
    if (wild_jump_seen) return;
    wild_jump_seen = 1; wild_jump_to = pc; wild_jump_from = from; wild_jump_interp = interp; wild_jump_tick = chip_frame_count;
    wild_jump_chunk = last_chunk_before(pc);
    wild_jump_rom = (pc & 0xffffff) >= 0xF80000u;   /* 24-bit bus: the high byte is ignored by the 68000 (54: 4EFAFDA2 = FAFDA2) */
    if (wild_jump_rom)
        fprintf(stderr, "FINDING: rom_call from %06X to %08X at tick %llu (%s; last native chunk %06X): the PC entered the Kickstart ROM window, which the port does not map\n",
                (unsigned)(from & 0xffffff), (unsigned)pc, wild_jump_tick, interp ? "interpreted code" : "native chunk dispatch", (unsigned)wild_jump_chunk);
    else
        fprintf(stderr, "FINDING: wild_jump from %06X to %08X at tick %llu (%s; last native chunk %06X): the PC left the memory map\n",
                (unsigned)(from & 0xffffff), (unsigned)pc, wild_jump_tick, interp ? "interpreted code" : "native chunk dispatch", (unsigned)wild_jump_chunk);
}
#ifdef AMIGO_PROBE
#include "m68k.h"
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
static uint32_t interp_last_pc;
static uint32_t ipc_set[8192];   /* fix3b: open-addressed set of recorded PCs (value = pc | 0x80000000), at most half full */
static int ipc_add(uint32_t a) {
    uint32_t k = a | 0x80000000u, h = (a * 2654435761u) >> 19;
    for (;; h++) { uint32_t *s = &ipc_set[h & 8191]; if (!*s) { *s = k; return 1; } if (*s == k) return 0; }
}
/* ---- per-block bisection (DEV ONLY, 18 Oct 2026): BISECT_INTERP=* | all | FILE runs the selected TRANSLATED code in Musashi
 * instead of its chunk, so cs_bisect.py can find which chunks make the native exe diverge from the harness. FILE: one entry per
 * line, "XXXXXX" (hex) = the whole 4 KB chunk containing it, "LO-HI" (hex, inclusive) = a range. Unset = all native. */
static struct { uint32_t lo, hi; } bisect_r[8192]; static int bisect_n = -1, bisect_all, interp_force_active;
uint64_t stat_bisect_calls;
static void bisect_init(void) {
    const char *e = getenv("BISECT_INTERP"); bisect_n = 0;
    if (!e || !*e) return;
    if (!strcmp(e, "*") || !strcmp(e, "all")) { bisect_all = 1; fprintf(stderr, "BISECT: all translated code interpreted\n"); return; }
    FILE *f = fopen(e, "r"); if (!f) { fprintf(stderr, "BISECT: cannot open %s\n", e); exit(4); }
    char ln[128]; unsigned lo, hi;
    while (fgets(ln, sizeof ln, f) && bisect_n < 8192) {
        if (sscanf(ln, "%x-%x", &lo, &hi) == 2) { bisect_r[bisect_n].lo = lo & 0xffffff; bisect_r[bisect_n++].hi = hi & 0xffffff; }
        else if (sscanf(ln, "%x", &lo) == 1) { bisect_r[bisect_n].lo = lo & 0xfff000; bisect_r[bisect_n++].hi = (lo & 0xfff000) | 0xfff; }
    }
    fclose(f); fprintf(stderr, "BISECT: %d ranges interpreted from %s\n", bisect_n, e);
}
static int bisect_interp_pc(uint32_t pc) {
    if (bisect_n < 0) bisect_init();
    if (bisect_all) return 1;
    pc &= 0xffffff;
    for (int i = 0; i < bisect_n; i++) if (pc >= bisect_r[i].lo && pc <= bisect_r[i].hi) return 1;
    return 0;
}
void harness_instr_hook(unsigned int pc) {
    stat_interp_instr++;   /* every instruction Musashi executes, including the one it stops after */
    if (interp_steps++ > 0) {
        if (whd_active && whd_hle_find(pc)) { m68k_end_timeslice(); return; }
        if (interp_force_active) {   /* bisection: keep interpreting while the PC is selected or untranslated; stop at native code and at hle slots */
            if ((game_is_translated(pc) && !bisect_interp_pc(pc)) || (hle_count && hle_find(pc))) { m68k_end_timeslice(); return; }
        } else if (game_is_translated(pc)) { m68k_end_timeslice(); return; }
    }
    /* fix3b: record the 24-bit PC (the 68000 bus), deduped by a hash set instead of an O(n) scan; stop recording at a wild jump */
    uint32_t a = pc & 0xffffff;
    if (!wild_jump_seen && !pc_mapped(a))
        wild_jump_note(pc, interp_steps > 1 ? interp_last_pc : last_chunk_before(a), interp_steps > 1);
    if (!wild_jump_seen && n_interp_pcs < 4096 && ipc_add(a)) interp_pcs[n_interp_pcs++] = a;
    interp_last_pc = a;
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

static void native_untranslated(void) { interp_run(); }
/* bisection: a selected translated PC runs in Musashi (returns 1); otherwise 0 and the caller runs the chunk as before */
static int bisect_interp_run(void) {
    if (!bisect_interp_pc(cpu.PC) || !game_chunk(cpu.PC)) return 0;
    stat_bisect_calls++; interp_force_active = 1; interp_run(); interp_force_active = 0;
    return 1;
}
#else
#define bisect_interp_run() 0      /* release and plain native builds: no bisection, the call folds away */
#ifdef AMIGO_TRACE
static struct { uint32_t pc, a7; } trc[256]; static int trc_n;
void intent_trace(uint32_t pc) { trc[trc_n & 255].pc = pc; trc[trc_n & 255].a7 = A[7]; trc_n++; }
static void trace_dump(void) { fprintf(stderr, "  block trace (oldest first):"); for (int i = 0; i < 256; i++) { int k = (trc_n + i) & 255; if (trc[k].pc) fprintf(stderr, " %06X/%X", trc[k].pc, trc[k].a7 & 0xfff); } fprintf(stderr, "\n"); }
#else
static void trace_dump(void) {}
#endif
static void native_finding(void) {   /* G2E10 kickinterp: was native_untranslated */
    if (!pc_mapped(cpu.PC)) wild_jump_note(cpu.PC, last_chunk_before(cpu.PC), 0);   /* fix3b: name a wild jump before the untranslated line */
    fprintf(stderr, "FINDING: untranslated address %06X reached at tick %llu (the translator must cover it; there is no interpreter in this exe)\n",
            (unsigned)(cpu.PC & 0xffffff), (unsigned long long)chip_frame_count);
    fprintf(stderr, "  last chunk entries:"); for (int i = 0; i < 16; i++) fprintf(stderr, " %06X", pc_ring[(pc_ring_n + i) & 15]);
    trace_dump();
    fprintf(stderr, "  longs at A3(=%06X): ", A[3]); for (int i = 0; i < 12; i++) fprintf(stderr, " %08X", mem_rl(A[3] + 4 * i)); fprintf(stderr, "\n  longs at A1(=%06X)-8: ", A[1]); for (int i = -2; i < 4; i++) fprintf(stderr, " %08X", mem_rl(A[1] + 4 * i)); fprintf(stderr, "\n");
    fprintf(stderr, "\n  SR=%04X IM=%d A7=%06X D0-D3 %08X %08X %08X %08X A0-A3 %08X %08X %08X %08X\n", get_sr(), cpu.IM, A[7], D[0], D[1], D[2], D[3], A[0], A[1], A[2], A[3]);
    if (n_interp_pcs < 4096) interp_pcs[n_interp_pcs++] = cpu.PC;
    fflush(NULL);
    if (getenv("HWSTAT") || getenv("DECSTAT")) exit(3);
    _exit(3);          /* not exit(): an atexit teardown of SDL/GL while the SDL thread holds the context crashed the Shield (SIGSEGV in NvEglDeinit, 15 Oct) */
}
#endif
#ifndef AMIGO_PROBE
/* G2E10 kickinterp (Claude sub-agent, 27 Oct 2026; Karl 27 Oct "port the fix"): a WHDLoad kickemu game boots the PLAYER'S own Kickstart
 * (runtime/rtkick.c reads it at run time; the exe embeds no Kickstart byte). The translator never sees ROM bytes, so the ROM's code
 * (exec's interrupt dispatch, its scheduler, the libraries the game calls) runs here in Musashi, and ONLY the ROM: the instruction hook
 * stops BEFORE the first instruction outside [whd_kick_base, +whd_kick_size) (longjmp; Musashi has finished the previous instruction,
 * so its registers are whole) and the native dispatcher takes that PC: translated game code runs as a chunk, an untranslated game PC is
 * still a FINDING. Measured before this (G2 scratch, 27 Oct): 649 671 223 653 "untranslated address C00D14 at tick 1" = a level-3
 * interrupt into exec. Built only when game.json has "kick" (amigo_port.sh kif/kis); every other native exe stays Musashi-free. */
#ifdef AMIGO_KICKINTERP
#include <setjmp.h>
#include "m68k.h"
#include "rtkick.h"
const int kick_interp_only = 1;   /* symcheck.sh: Musashi is linked ONLY for the player's Kickstart ROM */
unsigned int m68k_read_memory_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_memory_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_memory_32(unsigned int a) { return mem_rl(a); }
void m68k_write_memory_8(unsigned int a, unsigned int v) { mem_wb(a, v); }
void m68k_write_memory_16(unsigned int a, unsigned int v) { mem_ww(a, v); }
void m68k_write_memory_32(unsigned int a, unsigned int v) { mem_wl(a, v); }
unsigned int m68k_read_disassembler_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_disassembler_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_disassembler_32(unsigned int a) { return mem_rl(a); }
static int ki_steps, ki_ready; static jmp_buf ki_jb;
static int in_kickrom(uint32_t pc) { pc &= 0xffffff; return whd_kick_size && pc >= whd_kick_base && pc < whd_kick_base + whd_kick_size; }
void harness_instr_hook(unsigned int pc) {
    if (ki_steps++ > 0 && !in_kickrom(pc) && !(mem_rw(pc) == 0x4EF9 && in_kickrom(mem_rl(pc + 2))))   /* G2E10 kickjmp: a library vector into the ROM stays here */
        longjmp(ki_jb, 1);   /* left the ROM: this instruction is the native's */
    stat_interp_instr++;
}
int kick_take_stop(void);   /* runtime/rtkickstop.c */
static void kick_interp(void) {
    if (!ki_ready) { m68k_init(); m68k_set_cpu_type(M68K_CPU_TYPE_68000); m68k_pulse_reset(); ki_ready = 1; }
    stat_interp_calls++;
    m68k_set_reg(M68K_REG_SR, get_sr());
    if (cpu.S) { m68k_set_reg(M68K_REG_A7, A[7]); m68k_set_reg(M68K_REG_USP, cpu.OSP); }
    else { m68k_set_reg(M68K_REG_A7, A[7]); m68k_set_reg(M68K_REG_ISP, cpu.OSP); }
    for (int i = 0; i < 8; i++) m68k_set_reg(M68K_REG_D0 + i, D[i]);
    for (int i = 0; i < 7; i++) m68k_set_reg(M68K_REG_A0 + i, A[i]);
    m68k_set_reg(M68K_REG_PC, cpu.PC);
    ki_steps = 0;
    int budget = (int)(EVT > CYC ? EVT - CYC : 4); if (budget < 4) budget = 4;
    volatile int c = 0;
    if (!setjmp(ki_jb)) c = m68k_execute(budget); else c = m68k_cycles_run();
    CYC += (uint64_t)(c > 0 ? c : 4);
    for (int i = 0; i < 8; i++) D[i] = m68k_get_reg(NULL, M68K_REG_D0 + i);
    for (int i = 0; i < 8; i++) A[i] = m68k_get_reg(NULL, M68K_REG_A0 + i);
    uint32_t sr = m68k_get_reg(NULL, M68K_REG_SR);
    cpu.S = (sr >> 13) & 1; cpu.IM = (sr >> 8) & 7; set_ccr(sr);
    cpu.OSP = cpu.S ? m68k_get_reg(NULL, M68K_REG_USP) : m68k_get_reg(NULL, M68K_REG_ISP);
    cpu.PC = m68k_get_reg(NULL, M68K_REG_PC);
    if (kick_take_stop()) cpu.stopped = 1;    /* exec idles in STOP #$2000: the native's take_irq ends it (G1 STOPFIX) */
}
uint64_t stat_kick_vecjmp;
static void native_untranslated(void) {
    if (in_kickrom(cpu.PC)) { kick_interp(); return; }
    if (mem_rw(cpu.PC) == 0x4EF9) { stat_kick_vecjmp++; cpu.PC = mem_rl(cpu.PC + 2) & 0xffffff; CYC += 12; return; }   /* G2E10 kickjmp: an OS library vector */
    native_finding();
}
#else
static void native_untranslated(void) { native_finding(); }
#endif
#endif

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
    next_line = CYC_PER_LINE;
    return 0;
}

/* ---------------- the tick scheduler (clean sheet, step 2) ----------------
 * Translated code built with game.json sched:true has no CYC/EVT. The main thread runs until it YIELDS at a spin-loop leader (a second arrival
 * within one epoch, sched.h) or the watchdog trips on a busy loop; then virtual time advances ONE LINE (tick_line_step: audio, CIA, the copper
 * table's line callbacks, the tick end with VBL), interrupts are taken at that safe point with a real stack frame (take_irq pushes PC and SR, so a
 * handler that rewrites its stacked return PC to pause or start a match keeps working), and the main thread resumes where it yielded. */
int intent_wd = 200000;
static int yielded; static uint32_t epoch = 1;
static struct { uint32_t pc, ep; uint64_t cc; } ytab[8192];
uint64_t native_yields, native_watchdog_trips;
static uint32_t wd_seen[256]; static int n_wd_seen;
/* G2E9 leaderexit (Claude sub-agent, 27 Oct 2026): a yield leader yielded on its second arrival in an epoch even when the loop had
 * been LEFT in between (the wait was satisfied, the code returned and called the wait again). Measured on game 16 (ATF II, BEAMLOG):
 * its slave delay "1800 x bsr wait-until-line-$10" read VPOSR exactly once per line; after line $10 satisfied one call, the next call
 * re-entered the leader in the same epoch, yielded to line $11 and then waited a whole tick, so the delay ran 0.45 calls a tick
 * against the reference harness's 16 (IRQLOG: reference delay frames 278-390; port still in it at tick 845) and the picture never
 * came. A second arrival now yields only when it comes from the same chunk call (the loop's own back branch, a real spin); arriving
 * from another chunk call (the loop was left and entered again) re-marks the leader. Loops that spin through a call into another
 * chunk then run on to the slice end (preempt mode: the next line), so time still passes. AMIGO_LEADER_EXIT=0 = old behaviour. */
#ifndef AMIGO_LEADER_EXIT
#define AMIGO_LEADER_EXIT 1
#endif
static int leader_exit = -1;
int intent_yield_leader(uint32_t pc) {
    if (leader_exit < 0) { const char *e = getenv("AMIGO_LEADER_EXIT"); leader_exit = e ? (e[0] != '0') : AMIGO_LEADER_EXIT; }
    uint32_t h = (pc * 2654435761u) >> 19;
    while (ytab[h & 8191].pc && ytab[h & 8191].pc != pc) h++;
    h &= 8191;
    if (ytab[h].pc == pc && ytab[h].ep == epoch && !(leader_exit && ytab[h].cc != stat_chunk_calls)) { yielded = 1; native_yields++; return 1; }
    ytab[h].pc = pc; ytab[h].ep = epoch; ytab[h].cc = stat_chunk_calls; return 0;
}
/* cowork-slice (17 Oct, after Gemini job 6): in preempt mode each chunk call gets a block budget that ends at the next line, so a long
 * work loop (a decruncher) is sliced at line granularity and interrupts keep arriving; only 200000 blocks with no yield is a busy loop. */
static int wd_left = 200000, slice_set, sliced;
static int spin_mode = -1, spin_n, spin_have; static uint32_t spin_regs[18], spin_ep; uint64_t native_spin_yields;   /* g41 */
int intent_watchdog(uint32_t pc) {
    if (slice_set) { wd_left -= slice_set; slice_set = 0; if (wd_left > 0) {
        if (spin_mode < 0) { const char *e = getenv("AMIGO_SPIN_YIELD"); spin_mode = e ? atoi(e) : 2; }   /* g41 gate-on: unset = variant 2 */
        if (spin_mode > 0) {                     /* g41: the same block with the same registers on 5 consecutive slices = a wait */
            uint32_t r[18]; memcpy(r, cpu.dr, 32); memcpy(r + 8, cpu.ar, 32); r[16] = pc; r[17] = (uint32_t)(cpu.N | cpu.Z << 1 | cpu.V << 2 | cpu.C << 3 | cpu.X << 4);
            if (spin_mode == 1) { r[0] = 0; r[15] = 0; r[17] = 0; }   /* variant 1 (Gemini): D0, A7 and the flags are not compared */
            if (spin_have && spin_ep == epoch && !memcmp(r, spin_regs, sizeof r)) {
                if (++spin_n >= 5) { spin_n = 0; spin_have = 0; native_spin_yields++;
                    if (native_spin_yields <= 32 || !(native_spin_yields & 0xffff)) fprintf(stderr, "g41 SPINYIELD %06X n=%llu\n", (unsigned)pc, (unsigned long long)native_spin_yields);
                    { int k; for (k = 0; k < n_wd_seen; k++) if (wd_seen[k] == pc) break;   /* reg test: a spin yield is also a yield candidate */
                      if (k == n_wd_seen && n_wd_seen < 256) { wd_seen[n_wd_seen++] = pc;
                        FILE *f = fopen("out/yield_candidates.txt", "a"); if (f) { fprintf(f, "%06X\n", (unsigned)pc); fclose(f); } } }
                    yielded = 1; return 1; }
            } else { memcpy(spin_regs, r, sizeof r); spin_have = 1; spin_ep = epoch; spin_n = 0; }
        }
        sliced = 1; return 1; } }
    wd_left = 200000; intent_wd = 200000; native_watchdog_trips++;
    int i; for (i = 0; i < n_wd_seen; i++) if (wd_seen[i] == pc) break;
    if (i == n_wd_seen && n_wd_seen < 256) {
        wd_seen[n_wd_seen++] = pc;
        fprintf(stderr, "FINDING: busy loop without a yield near %06X (watchdog); add it to game.json yields\n", (unsigned)pc);
        FILE *f = fopen("out/yield_candidates.txt", "a"); if (f) { fprintf(f, "%06X\n", (unsigned)pc); fclose(f); }
    }
    yielded = 1; return 1;
}

/* g54 (Cowork 19 Oct 2026, reviewed from Gemini job g54): two scheduler gates, both compile-default OFF, the environment overrides.
 * AMIGO_SCHED_FIXCYC: run_frame_preempt carries the fractional bus-contention cycles from chunk to chunk, as run_frame carries
 *   cyc_carry; unset/0 keeps the old per-chunk truncation of (cycles x (cpu_slow_num-1024) / 1024) exactly.
 * AMIGO_SCHED_TRACE: one stderr line per tick (chunks, yields, g41 spin yields, watchdog trips, interrupt levels taken, INTREQ bits
 *   seen at line ends, cycles the truncation dropped, frame hash), at most AMIGO_SCHED_TRACE_MAX lines (default 2000). Log only. */
#ifndef AMIGO_SCHED_FIXCYC
#define AMIGO_SCHED_FIXCYC 0
#endif
#ifndef AMIGO_SCHED_TRACE
#define AMIGO_SCHED_TRACE 0
#endif
static int sched_fixcyc = -1, sched_tr = -1; static uint64_t sched_cyc_carry;
static uint64_t tr_lost, tr_lines; static uint32_t tr_irq; static uint16_t tr_intreq; static long tr_max = 2000, tr_n;
static void sched_gates_init(void) {
    if (sched_fixcyc < 0) { const char *e = getenv("AMIGO_SCHED_FIXCYC"); sched_fixcyc = e ? (e[0] != '0') : AMIGO_SCHED_FIXCYC; }
    if (sched_tr < 0) { const char *e = getenv("AMIGO_SCHED_TRACE"); sched_tr = e ? (e[0] != '0') : AMIGO_SCHED_TRACE;
        const char *m = getenv("AMIGO_SCHED_TRACE_MAX"); if (m) tr_max = atol(m); }
}
static void sched_trace_tick(int pre) {
    static uint64_t lc, ly, ls, lw;
    if (tr_n < tr_max) {
        tr_n++;
        fprintf(stderr, "SCHEDTRACE tick %llu %s chunks %llu yields %llu spin %llu wd %llu irq %02X intreq %04X lines %llu lost %llu carry %llu fix %d cyc %llu hash %016llX\n",
            (unsigned long long)chip_frame_count, pre ? "preempt" : "yieldsched", (unsigned long long)(stat_chunk_calls - lc), (unsigned long long)(native_yields - ly),
            (unsigned long long)(native_spin_yields - ls), (unsigned long long)(native_watchdog_trips - lw), (unsigned)tr_irq, (unsigned)tr_intreq,
            (unsigned long long)tr_lines, (unsigned long long)(tr_lost / 1024), (unsigned long long)sched_cyc_carry, sched_fixcyc, (unsigned long long)CYC,
            (unsigned long long)hash_frame());
        if (tr_n == tr_max) fprintf(stderr, "SCHEDTRACE cap %ld lines reached, trace stops\n", tr_max);
    }
    lc = stat_chunk_calls; ly = native_yields; ls = native_spin_yields; lw = native_watchdog_trips; tr_irq = 0; tr_intreq = 0; tr_lines = 0; tr_lost = 0;
}
static uint64_t cyc_last, cyc_carry; static int owed_lines; static int evt_mode = -1;
static void step_line(void);
static int irq_depth;
static void run_irq(int l) {
    irq_depth++; uint32_t a7_before = A[7], pc_before = cpu.PC; int s_before = cpu.S;            /* an interrupt taken at a safe point: the handler runs to its RTE (it may rewrite its stacked PC: pause, match start) */
    take_irq(l);
    while (cpu.IM >= l && !cpu.stopped) {
        if (hle_count) { const hle_t *h = hle_find(cpu.PC); if (h) { h->fn(); continue; } }
        pc_ring[pc_ring_n++ & 15] = cpu.PC;
        chunk_fn f = game_chunk(cpu.PC);
        if (!bisect_interp_run() && (!f || !f())) native_untranslated();
        if (yielded) { yielded = 0; epoch++; intent_wd = 200000; step_line(); }     /* a wait inside a handler: time passes (a higher-level interrupt may nest) */
    }
    irq_depth--;
    if (getenv("STACKCHK") && !(A[7] == a7_before || A[7] == a7_before + 0) ) fprintf(stderr, "STACKCHK tick %llu level %d: A7 before %06X after %06X (PC before %06X, now %06X, S %d->%d)\n", (unsigned long long)chip_frame_count, l, a7_before, A[7], pc_before, cpu.PC, s_before, cpu.S);
}
static void step_line(void) {           /* one line of virtual time: audio, CIA, the copper table's line callbacks, the tick end; then any interrupt it raised */
    CYC += CYC_PER_LINE; cyc_last = CYC;
    tick_line_step();
    if (sched_tr > 0) { tr_lines++; tr_intreq |= intreq; }   /* g54 trace */
    int l = chip_ipl();
    if (l > cpu.IM && !(irq_depth && getenv("NO_NEST"))) { if (sched_tr > 0) tr_irq |= 1u << l; run_irq(l); }
}
/* PREEMPT mode (env AMIGO_PREEMPT=1, game.json evt:true): the same tick scheduler, but virtual time ALSO passes while the main thread computes (CYC counts, x1.1),
 * so interrupts arrive mid-compute at line granularity as on the original, while a yield (a spin loop) still jumps virtual time to the next line instead of spinning.
 * Measured 15 Oct: without it IK+'s music sequencer races with the main thread differently and ends in a wild jump at tick 995. */
static uint64_t pre_next_line;
static void run_frame_preempt(void) {
    if (!pre_next_line) pre_next_line = (CYC / CYC_PER_LINE + 1) * CYC_PER_LINE;
    chip_frame_done = 0;
    while (!chip_frame_done) {
        while (CYC >= pre_next_line && !chip_frame_done) { tick_line_step(); pre_next_line += CYC_PER_LINE; if (sched_tr > 0) { tr_lines++; tr_intreq |= intreq; } }
        if (chip_frame_done) break;
        int l = chip_ipl();
        if (l > cpu.IM) { if (sched_tr > 0) tr_irq |= 1u << l; take_irq(l); }
        EVT = pre_next_line;
        if (cpu.stopped) { CYC = pre_next_line; continue; }
        if (hle_count) { const hle_t *h = hle_find(cpu.PC); if (h) { yielded = 0; h->fn(); if (yielded) { epoch++; intent_wd = 200000; if (CYC < pre_next_line) CYC = pre_next_line; } continue; } }
        pc_ring[pc_ring_n++ & 15] = cpu.PC;
        chunk_fn f = game_chunk(cpu.PC);
        stat_chunk_calls++;
        uint64_t c0 = CYC; yielded = 0;
        { int64_t rem = pre_next_line > CYC ? (int64_t)(pre_next_line - CYC) : 0; int b = (int)(rem / 8); if (b < 1) b = 1; if (b > wd_left) b = wd_left; intent_wd = b; slice_set = b; sliced = 0; }
        if (!bisect_interp_run() && (!f || !f())) native_untranslated();   /* bisection inside the slice budget: the CYC scaling below covers interpreted work too */
        if (slice_set) { wd_left -= slice_set - (intent_wd > 0 ? intent_wd : 0); slice_set = 0; if (wd_left <= 0) wd_left = 1; }
        { uint64_t add = (CYC - c0) * (cpu_slow_num - 1024);                  /* the fitted bus-contention slowdown, as the reference harness */
          if (sched_fixcyc > 0) { sched_cyc_carry += add; CYC += sched_cyc_carry / 1024; sched_cyc_carry %= 1024; }   /* g54 FIXCYC: carry the fraction */
          else { CYC += add / 1024; if (sched_tr > 0) tr_lost += add % 1024; } }
        if (yielded) { epoch++; intent_wd = 200000; wd_left = 200000; if (CYC < pre_next_line) CYC = pre_next_line; }     /* a spin loop: virtual time jumps to the next line */
    }
    if (sched_tr > 0) sched_trace_tick(1);
}

void run_frame(void) {
    if (sched_fixcyc < 0) sched_gates_init();                          /* g54 gates, read once */
    if (evt_mode < 0) evt_mode = getenv("AMIGO_NOPREEMPT") == NULL;      /* preempt mode is the default (the attract run needs it); AMIGO_NOPREEMPT=1 selects the non-preemptive scheduler */
    if (evt_mode) { run_frame_preempt(); return; }
    chip_frame_done = 0;
    while (!chip_frame_done) {
        if (owed_lines > 0) { owed_lines--; step_line(); continue; }       /* lines the compute cost (charged at the last yield), still to pass */
        int l = chip_ipl();
        if (l > cpu.IM) run_irq(l);
        if (cpu.stopped) { step_line(); continue; }                         /* STOP: time passes until an interrupt */
        yielded = 0;
        while (!yielded && !cpu.stopped) {
            if (hle_count) { const hle_t *h = hle_find(cpu.PC); if (h) { h->fn(); continue; } }
            pc_ring[pc_ring_n++ & 15] = cpu.PC;
            chunk_fn f = game_chunk(cpu.PC);
            stat_chunk_calls++;
            if (!bisect_interp_run() && (!f || !f())) native_untranslated();
            l = chip_ipl();
            if (l > cpu.IM) run_irq(l);
        }
        if (getenv("SCHEDLOG")) { static int n; if (n++ < 20 || (n % 5000) == 0 || chip_frame_count >= (unsigned)atoi(getenv("SCHEDLOG"))) fprintf(stderr, "yield %d tick %llu line %d PC=%06X IM=%d owed=%d\n", n, (unsigned long long)chip_frame_count, chip_line(), (unsigned)cpu.PC, cpu.IM, owed_lines); }
        epoch++; intent_wd = 200000;
        cyc_carry += (CYC - cyc_last) * cpu_slow_num / 1024;                 /* compute since the last time step, charged as lines (x1.1: the fitted bus-contention slowdown the reference harness uses) */
        owed_lines += (int)(cyc_carry / CYC_PER_LINE); cyc_carry %= CYC_PER_LINE;
        step_line();                                                         /* the yield itself: at least one line passes */
    }
    if (sched_tr > 0) sched_trace_tick(0);
}


/* ---- native replacements of the game's own wait routines (game.json hle): the idiom is recognised from the code, the beam is never read ---- */
void native_wait_window(void) {      /* IK+ $00775C: wait until the raster line is in [D0, D1] (move.l $DFF004,d3 / lsr / and $1FF / cmp d0 / blt / cmp d1 / bgt / rts) */
    int d0 = (int)(D[0] & 0x1ff), d1 = (int)(D[1] & 0x1ff), L = chip_line();
    if (L >= d0 && L <= d1) {
        D[3] = (D[3] & 0xffff0000u) | (uint32_t)L;
        cpu.PC = mem_rl(A[7]); A[7] += 4;                      /* rts */
        cpu.N = (L >> 15) & 1; cpu.Z = 0; cpu.V = 0; cpu.C = 0;
        return;
    }
    yielded = 1;                                               /* not yet: virtual time passes, this routine is entered again at the same PC */
}
