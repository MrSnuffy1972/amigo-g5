/* Ami-Go port kit, generic analysis harness (from the Ghosts 'n Goblins pilot; NOT part of the port).
 * Runs the game's original 68000 code on the Musashi
 * interpreter against the same chipset runtime the port uses. Used to (1) discover reachable code
 * (executed-PC log fed to the translator), (2) produce reference frames to compare the translated
 * build against instruction for instruction / frame for frame.
 * usage: harness <gamedir> <frames> [outdir] [script]
 * HLE routines from game.json run here exactly as in the port (same C functions, same cpu_t state).
 * Writes outdir/executed_pcs.txt (every PC executed, any address), the translator's best seed list.
 * script lines: "<frame> <keys>" where keys is a string of U D L R F (fire) or '-' for none;
 * a token Knn / Pnn / Qnn is a raw Amiga keycode nn (hex): K = press and release, P = press only, Q = release only */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include "m68k.h"
#include "amiga.h"
#include "boot.h"
#include "cpu.h"
#include "whd.h"
cpu_t cpu; uint64_t CYC, EVT;

static const char *gamedir;
static uint8_t *files[64]; static size_t flen[64]; static char fname[64][32]; static int nfiles;
typedef struct { const char *name; const unsigned char *data; unsigned len; } gfile_t;
extern const gfile_t game_files[]; extern const int game_nfiles;   /* gamedata.c (portgen.py) */
const uint8_t *gamefile(const char *name, size_t *len) {
    for (int i = 0; i < game_nfiles; i++) if (!strcmp(game_files[i].name, name)) { *len = game_files[i].len; return game_files[i].data; }
    for (int i = 0; i < game_nfiles; i++) if (amigo_streq_ci(game_files[i].name, name)) { *len = game_files[i].len; return game_files[i].data; }
    for (int i = 0; i < nfiles; i++) if (amigo_streq_ci(fname[i], name)) { *len = flen[i]; return files[i]; }
    char p[512]; snprintf(p, sizeof p, "%s/%s", gamedir, name);
    FILE *f = fopen(p, "rb"); if (!f) return NULL;
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    uint8_t *b = malloc((size_t)n); if (fread(b, 1, (size_t)n, f) != (size_t)n) { fclose(f); return NULL; } fclose(f);
    files[nfiles] = b; flen[nfiles] = (size_t)n; snprintf(fname[nfiles], 32, "%s", name); nfiles++;
    *len = (size_t)n; return b;
}

#ifdef MEMLOG
static uint8_t wr[0x100000]; static uint8_t rep[0x100000]; static int uninit_on;
static int ridx(uint32_t a) { a &= 0xffffff; if (a < 0x200000) return (int)(a & 0x7ffff); if (a >= 0xC00000 && a < 0xD80000) return 0x80000 + (int)((a - 0xC00000) & 0x7ffff); return -1; }
void memlog_byte(uint32_t a, uint8_t *p) { (void)p; wr[a & 0xfffff] = 1; }
static void chk(uint32_t a, int n) { if (!uninit_on) return; for (int i = 0; i < n; i++) { int r = ridx(a + i); if (r >= 0 && !wr[r] && !rep[r]) { rep[r] = 1; fprintf(stderr, "UNINIT read %06X pc %06X\n", a + i, m68k_get_reg(NULL, M68K_REG_PPC)); } } }
#else
#define chk(a, n) ((void)0)
#endif
unsigned int m68k_read_memory_8(unsigned int a) { chk(a, 1); return mem_rb(a); }
unsigned int m68k_read_memory_16(unsigned int a) { chk(a, 2); return mem_rw(a); }
unsigned int m68k_read_memory_32(unsigned int a) { chk(a, 4); return mem_rl(a); }
void m68k_write_memory_8(unsigned int a, unsigned int v) { mem_wb(a, v); }
void m68k_write_memory_16(unsigned int a, unsigned int v) { mem_ww(a, v); }
void m68k_write_memory_32(unsigned int a, unsigned int v) { mem_wl(a, v); }
unsigned int m68k_read_disassembler_8(unsigned int a) { return mem_rb(a); }
unsigned int m68k_read_disassembler_16(unsigned int a) { return mem_rw(a); }
unsigned int m68k_read_disassembler_32(unsigned int a) { return mem_rl(a); }

static uint64_t cyc_done;
static double cpu_scale = 1126.0 / 1024.0;   /* >1: CPU slower than 7.09 MHz (DMA contention); SAME default as the port (runtime/cpu.c cpu_slow_num) so G3 compares like with like */
static uint64_t now(void) { return (uint64_t)((double)(cyc_done + (uint64_t)m68k_cycles_run()) * cpu_scale); }
static void ipl(void) { m68k_set_irq((unsigned)chip_ipl()); }
static void hstall(uint64_t c) { cyc_done += c; }
static void logf_(const char *fmt, ...) { va_list ap; va_start(ap, fmt); vfprintf(stderr, fmt, ap); va_end(ap); }

uint64_t frame_start_cyc, idle_cyc; static FILE *busyf;
static uint8_t seen[0x1000000 / 8], volat[0x1000000 / 8];
static uint16_t *firstop;   /* opcode word at each PC when it first ran */
static uint64_t instr_count;
static FILE *trace;
/* G2E10 rngseed (Claude sub-agent, 27 Oct 2026; Karl 27 Oct, TEST-ONLY): RNGREC=file RNGREC_PC=hex RNGREC_FRAME=n records, from the first
 * execution of the snapshot PC at or after the snapshot frame (the moment the native exe starts from), every read of a hardware source
 * a game can draw randomness from, with the PC of the instruction: "frame pc key value" (key = custom register, or $100/$200 + CIA-A/B
 * register). cs_rngtable.py turns it into the native's replay table. Off unless RNGREC is set; nothing else changes. */
static unsigned rr_pc; static int rr_on = -1; static FILE *rr_f; static unsigned rr_spc; static unsigned long long rr_sf;
void entropy_read_hook(int kind, int reg, unsigned v) {
    if (rr_on != 1 || !rr_f) return;
    int key = kind ? 0x100 * kind + (reg & 15) : (reg & 0x1fe);
    if (kind ? !((reg & 15) >= 4 && (reg & 15) <= 10) : !(key == 0x004 || key == 0x006 || key == 0x012 || key == 0x014 || key == 0x018 || key == 0x01A)) return;
    fprintf(rr_f, "%llu %06X %03X %X\n", (unsigned long long)chip_frame_count, rr_pc, key, v);
}
/* G2E10 kick (from the live kit's kit-kick): the Kickstart image a kickemu slave runs (whd_kick_base/size, the PLAYER'S file) never
 * leaves the harness: its PCs are left out of executed_pcs/executed_ops/FIRSTBYTES (counted in kick_pcs.txt, addresses only), its
 * bytes are zeroed in every RAM dump and snapshot, no snapshot is taken inside it, and the snapshot names it (snap_kick.txt) so the
 * native exe loads the same file into the same place at boot. */
#include "rtkick.h"
static int in_kick(uint32_t a) { return whd_kick_size && a >= whd_kick_base && a < whd_kick_base + whd_kick_size; }
static void write_slow(const char *p) { FILE *f = fopen(p, "wb"); if (!f) return; uint8_t *c = malloc(SLOW_SIZE); memcpy(c, slowram, SLOW_SIZE);
    if (whd_kick_size && whd_kick_base >= SLOW_BASE && whd_kick_base + whd_kick_size <= SLOW_BASE + SLOW_SIZE) memset(c + (whd_kick_base - SLOW_BASE), 0, whd_kick_size);
    fwrite(c, 1, SLOW_SIZE, f); fclose(f); free(c); }
void harness_instr_hook(unsigned int pc) {
    instr_count++;
    pc &= 0xffffff;
    rr_pc = pc;   /* G2E10 rngseed: the instruction whose reads entropy_read_hook records */
    if (rr_on < 0) { const char *e = getenv("RNGREC"), *p = getenv("RNGREC_PC"), *f = getenv("RNGREC_FRAME"); rr_on = 0;
        if (e && p) { rr_f = fopen(e, "w"); rr_spc = (unsigned)strtoul(p, 0, 16) & 0xffffff; rr_sf = f ? strtoull(f, 0, 10) : 0; } }
    if (rr_on == 0 && rr_f && pc == rr_spc && chip_frame_count >= rr_sf) { rr_on = 1; fprintf(rr_f, "# start frame %llu pc %06X\n", (unsigned long long)chip_frame_count, pc); }
    {   /* RNGLOG=file RNG_PC=hex: trace of a routine's calls (tick, entry registers D0-D2, result D0 on return): the PRNG draws of a reference run (15 Oct) */
        static FILE *rf; static int ri; static unsigned rpc; static uint32_t ret_pc, ret_sp; static int pending; static uint64_t ncall;
        if (!ri) { ri = 1; const char *e = getenv("RNGLOG"); if (e) rf = fopen(e, "w"); const char *p = getenv("RNG_PC"); rpc = p ? (unsigned)strtoul(p, 0, 16) : 0x785C; }
        if (rf) {
            if (pending && pc == ret_pc && m68k_get_reg(NULL, M68K_REG_A7) == ret_sp) { fprintf(rf, " -> %08X\n", m68k_get_reg(NULL, M68K_REG_D0)); pending = 0; }
            if (pc == rpc && !pending) { uint32_t sp = m68k_get_reg(NULL, M68K_REG_A7); ret_pc = mem_rl(sp) & 0xffffff; ret_sp = sp + 4; pending = 1; ncall++;
                fprintf(rf, "%llu %llu D0=%08X D1=%08X D2=%08X", (unsigned long long)chip_frame_count, (unsigned long long)ncall, m68k_get_reg(NULL, M68K_REG_D0), m68k_get_reg(NULL, M68K_REG_D1), m68k_get_reg(NULL, M68K_REG_D2)); }
        }
    }
    {   /* SNAP_PC=hex SNAP_OUT=dir: native-slice snapshot (15 Oct): at the first execution of that PC write the registers and all RAM, then stop.
         * The loader/WHDLoad layer has finished by then; the snapshot is the game's own state, so a native exe needs no slave, resload or fake library. */
        static int sp_init, sp_done; static unsigned sp_pc; static unsigned long long sp_frame;
        if (!sp_init) { sp_init = 1; const char *e = getenv("SNAP_PC"); sp_pc = e ? (unsigned)strtoul(e, 0, 16) : 0xffffffffu; const char *ef = getenv("SNAP_FRAME"); sp_frame = ef ? strtoull(ef, 0, 10) : 0; }
        /* dsfix 18 Oct (G2): interrupt depth (entries seen as in IRQLOG, left at RTE) and the real RAM windows. Rule A (unchanged): IPL mask 0,
         * PC in chip RAM or slow RAM below the WHDLoad tower. Rule B, only 50 frames after the target frame passed with no rule-A hit: depth 0
         * (main line, not inside an interrupt handler) with any IPL mask, PC in chip RAM, slow RAM below the tower or WHDLoad expansion memory. */
        static int sd_depth; static unsigned sd_lastim = 7; extern uint32_t whd_slave_addr, whd_expmem, whd_expsize;
        unsigned sd_sr = m68k_get_reg(NULL, M68K_REG_SR), sd_im = (sd_sr >> 8) & 7;
        if (sd_im > sd_lastim && (sd_sr & 0x2000) && pc == (mem_rl((24 + sd_im) * 4) & 0xffffff)) sd_depth++;
        sd_lastim = sd_im;
        uint32_t sd_top = (whd_slave_addr >= SLOW_BASE && whd_slave_addr < SLOW_BASE + SLOW_SIZE) ? whd_slave_addr - 0x100 : (SLOW_SIZE == 512 * 1024 ? 0xC7E000u : SLOW_BASE + SLOW_SIZE);
        int sd_ram = (pc >= 0x400 && pc < CHIP_SIZE) || (pc >= SLOW_BASE && pc < sd_top) || (whd_expsize && pc >= whd_expmem && pc < whd_expmem + whd_expsize);
        int sd_ruleA = sp_frame && chip_frame_count >= sp_frame && ((pc >= 0x400 && pc < 0x200000) || (pc >= 0xC00000 && pc < 0xC7E000)) && !(sd_sr & 0x0700);
        int sd_ruleB = sp_frame && chip_frame_count >= sp_frame + 50 && sd_ram && sd_depth == 0;
        if ((uint16_t)mem_rw(pc) == 0x4E73 && sd_depth > 0) sd_depth--;   /* RTE: counted after the test, so the RTE itself is still inside */
        extern int oshle_snap_hold(void) __attribute__((weak));   /* G2E9 osport: not while the fake OS's startup-sequence runner runs */
        int sp_held = oshle_snap_hold && oshle_snap_hold(); static int sp_pend;   /* the snapshot PC met while held: taken at the first instruction after */
        if (sp_held && pc == sp_pc) sp_pend = 1;   /* measured on 555: pick chose $80000, the runner's own entry, and the snapshot never came */
        if (!sp_done && getenv("SNAP_OUT") && (pc == sp_pc || (sp_pend && sd_depth == 0) || sd_ruleA || sd_ruleB) && !sp_held && !in_kick(pc)) {   /* G2E10 kick: never inside the Kickstart */
            if (!sd_ruleA && sd_ruleB && pc != sp_pc) fprintf(stderr, "SNAPSHOT rule B (depth 0, IPL mask %u)\n", sd_im);   /* SNAP_FRAME=n (Cowork 15 Oct): first main-line game instruction at or after frame n */
            sp_done = 1; char p[600]; const char *o = getenv("SNAP_OUT");
            snprintf(p, sizeof p, "%s/snap_regs.txt", o); FILE *f = fopen(p, "w");
            if (f) { for (int i = 0; i < 8; i++) fprintf(f, "d%d %08X\n", i, m68k_get_reg(NULL, M68K_REG_D0 + i));
                     for (int i = 0; i < 8; i++) fprintf(f, "a%d %08X\n", i, m68k_get_reg(NULL, M68K_REG_A0 + i));
                     fprintf(f, "pc %08X\nsr %04X\nusp %08X\nisp %08X\nframe %llu\n", pc, m68k_get_reg(NULL, M68K_REG_SR), m68k_get_reg(NULL, M68K_REG_USP), m68k_get_reg(NULL, M68K_REG_ISP), (unsigned long long)chip_frame_count);
                     for (int i = 0; i < 256; i++) if (custom[i]) fprintf(f, "custom %03X %04X\n", i * 2, custom[i]);
                     fprintf(f, "intena %04X\nintreq %04X\ndmacon %04X\n", intena, intreq, dmacon); { extern void chip_dump_cia(FILE *); chip_dump_cia(f); } fclose(f); }
            snprintf(p, sizeof p, "%s/snap_chip.bin", o); f = fopen(p, "wb"); if (f) { fwrite(chipram, 1, CHIP_SIZE, f); fclose(f); }
            snprintf(p, sizeof p, "%s/snap_slow.bin", o); write_slow(p);   /* G2E10 kick: Kickstart zeroed */
            if (whd_kick_size) { snprintf(p, sizeof p, "%s/snap_kick.txt", o); f = fopen(p, "w"); if (f) { fprintf(f, "%06X %X %04X %s\n", whd_kick_base, whd_kick_size, kick_crc, kick_name); fclose(f); } }
            fprintf(stderr, "SNAPSHOT written to %s at PC %06X (frame %llu)\n", o, pc, (unsigned long long)chip_frame_count);
            exit(0);
        }
    }
    { uint16_t op = (uint16_t)mem_rw(pc);
      if (!(seen[pc >> 3] & (1 << (pc & 7)))) { seen[pc >> 3] |= (uint8_t)(1 << (pc & 7)); firstop[pc] = op;
          static FILE *fb; static int fbi; if (!fbi) { fbi = 1; const char *o = getenv("FIRSTBYTES"); if (o) fb = fopen(o, "w"); }     /* FIRSTBYTES=file: the 10 bytes at every PC when it first runs: code that no longer exists at the end of the run (stack, vectors, overlays) can still be translated (15 Oct, 53 near-miss G1 games) */
          if (fb && !in_kick(pc)) { fprintf(fb, "%06X", pc); for (int i = 0; i < 10; i++) fprintf(fb, " %02X", mem_rb(pc + i)); fprintf(fb, "\n"); } }
      else if (firstop[pc] != op) volat[pc >> 3] |= (uint8_t)(1 << (pc & 7)); }   /* code changed under this PC */
    if (trace) fprintf(trace, "%06X\n", pc);
    {   /* IRQLOG=file: interrupt entries, same format as runtime/cpu.c take_irq (frame line level interrupted-PC) */
        static FILE *il; static int ili; static unsigned lastim = 7;
        if (!ili) { ili = 1; if (getenv("IRQLOG")) il = fopen(getenv("IRQLOG"), "w"); }
        unsigned sr = m68k_get_reg(NULL, M68K_REG_SR), im = (sr >> 8) & 7;
        if (il && im > lastim && (sr & 0x2000) && pc == (mem_rl((24 + im) * 4) & 0xffffff)) {
            uint32_t sp = m68k_get_reg(NULL, M68K_REG_A7);
            fprintf(il, "%llu %d %u %06X\n", (unsigned long long)chip_frame_count, chip_line(), im, mem_rl(sp + 2) & 0xffffff);
        }
        lastim = im;
    }
    const hle_t *h = (hle_count || whd_active) ? hle_find(pc) : 0;
    if (h) {   /* the same C replacement the port runs, on the same register state */
        uint32_t sr = m68k_get_reg(NULL, M68K_REG_SR);
        for (int i = 0; i < 8; i++) { cpu.dr[i] = m68k_get_reg(NULL, M68K_REG_D0 + i); cpu.ar[i] = m68k_get_reg(NULL, M68K_REG_A0 + i); }
        cpu.S = (sr >> 13) & 1; cpu.IM = (sr >> 8) & 7; set_ccr(sr);
        cpu.OSP = cpu.S ? m68k_get_reg(NULL, M68K_REG_USP) : m68k_get_reg(NULL, M68K_REG_ISP);
        cpu.PC = pc;
        h->fn();
        for (int i = 0; i < 8; i++) { m68k_set_reg(M68K_REG_D0 + i, cpu.dr[i]); m68k_set_reg(M68K_REG_A0 + i, cpu.ar[i]); }
        m68k_set_reg(M68K_REG_SR, get_sr());
        m68k_set_reg(M68K_REG_A7, cpu.ar[7]);
        if (cpu.S) m68k_set_reg(M68K_REG_USP, cpu.OSP); else m68k_set_reg(M68K_REG_ISP, cpu.OSP);
        m68k_set_reg(M68K_REG_PC, cpu.PC);
        ipl();
    }
    else { extern void oshle_note_pc(uint32_t) __attribute__((weak));   /* G2E9 osport (as the live kit's harness, G0FIX/G1E6): the fake OS's
           * startup-sequence runner rules (E23 prebudget, E25 prefail, E29 preidle) see every instruction */
        if (oshle_note_pc) oshle_note_pc(pc); }
    /* G2E9 osport (Claude sub-agent, 26 Oct 2026): when an HLE (or the runner rules above) set a new PC, Musashi fetches and runs the
     * instruction there without calling this hook, so its first run was never recorded (executed_pcs, FIRSTBYTES: the translator
     * missed it; G2E7 seeded oshle's stub by hand), an HLE slot there ran as its bare RTS stub, and a snapshot PC there was never
     * seen (measured on 199: E25 prefail sends the PC to the chosen program's entry $4008, which the snapshot waits for). The hook
     * runs for that instruction now, as Musashi would have called it; nested at most 8 deep. */
    { static int hx_depth; unsigned int np = m68k_get_reg(NULL, M68K_REG_PC) & 0xffffff;
      if (np != pc && hx_depth < 8) { hx_depth++; instr_count--; harness_instr_hook(np); hx_depth--; } }
}

#include "amigo_img.h"   /* Cowork 18 Oct: PNG + raw RGBA (AMIGO_IMG switch); PPM kept only as rollback */
static void save_ppm(const char *path) { amigo_img_save(path, framebuf, SCR_W, SCR_H); }   /* format = path extension */

extern void (*dbg_read_hook)(int reg);
static unsigned rdcount[0x1100][1]; static uint32_t rdpc[0x1100];
static void rdhook(int reg) { if (reg < 0x1100) { rdcount[reg][0]++; rdpc[reg] = m68k_get_reg(NULL, M68K_REG_PPC);
    if (reg == 4 || reg == 6 || reg >= 0x1000) { static uint32_t seen[64][2]; static int ns; uint32_t pc = rdpc[reg]; int i; for (i = 0; i < ns; i++) if (seen[i][0] == pc && seen[i][1] == (uint32_t)reg) break; if (i == ns && ns < 64) { seen[ns][0] = pc; seen[ns][1] = (uint32_t)reg; ns++; fprintf(stderr, "first read reg %04X at pc %06X\n", reg, pc); } } } }
int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr, "usage: %s gamedir frames [outdir] [script]\n", argv[0]); return 1; }
    gamedir = argv[1]; int frames = atoi(argv[2]); const char *outdir = argc > 3 ? argv[3] : "out/ref";
    int sframe[512]; char skeys[512][8]; int ns = 0;
    if (argc > 4) { FILE *s = fopen(argv[4], "r"); while (s && ns < 512 && fscanf(s, "%d %7s", &sframe[ns], skeys[ns]) == 2) ns++; if (s) fclose(s); }
    if (getenv("TRACE")) trace = fopen(getenv("TRACE"), "w");
    if (getenv("GNG_CPUSCALE")) cpu_scale = atof(getenv("GNG_CPUSCALE"));   /* the port reads the same variable */
    if (getenv("CPUSCALE")) cpu_scale = atof(getenv("CPUSCALE"));
    cpu_now = now; cpu_stall = hstall; hle_log = logf_; ipl_changed = ipl; if (getenv("RDLOG")) dbg_read_hook = rdhook;
    firstop = calloc(0x1000000, 2);
    m68k_init(); m68k_set_cpu_type(M68K_CPU_TYPE_68000);
    uint32_t regs[16], pc; uint16_t sr;
    /* WHD_SLAVE=path: boot a WHDLoad slave through the C resload (runtime/whd.c) instead of the game.json image */
    const char *wslave = getenv("WHD_SLAVE");
    if (wslave) { int r = whd_boot(wslave, regs, &pc, &sr); if (r) { fprintf(stderr, "WHD boot failed (%d)\n", r); return 10 + r; } }
    else if (boot_setup(regs, &pc, &sr)) { fprintf(stderr, "boot failed\n"); return 1; }
#ifdef MEMLOG
    uninit_on = getenv("UNINIT") != NULL;
#endif
    m68k_pulse_reset();
    for (int i = 0; i < 8; i++) { m68k_set_reg(M68K_REG_D0 + i, regs[i]); m68k_set_reg(M68K_REG_A0 + i, regs[8 + i]); }
    m68k_set_reg(M68K_REG_SR, sr); m68k_set_reg(M68K_REG_A7, regs[15]); m68k_set_reg(M68K_REG_PC, pc);
    /* k17 oshle: a program started in user mode (SR S=0) gets the reset vector's supervisor stack, as the port's run_boot
     * does (runtime/cpu.c). Measured 14 Oct: without it the ISP was 0 and 3D Galax's Supervisor() call pushed to $FFFFFFFA */
    if (!(sr & 0x2000)) m68k_set_reg(M68K_REG_ISP, mem_rl(0));
    if (whd_active) m68k_set_reg(M68K_REG_USP, whd_basemem - 0x400);   /* WHDLoad: USP = SSP - $400 */
    { extern uint32_t boot_osp; if (boot_osp) m68k_set_reg((sr & 0x2000) ? M68K_REG_USP : M68K_REG_ISP, boot_osp); }   /* G2E3 tlsnap: snapshot boot (live kit, Cowork 17 Oct) */
    int si = 0;
    for (int fr = 0; fr < frames; fr++) {
        while (si < ns && (sframe[si] < SCRIPT_ANCHOR ? sframe[si] <= fr : (hle_last_load_frame && (long)(hle_last_load_frame + (uint64_t)(sframe[si] - SCRIPT_ANCHOR)) <= fr))) {
            const char *k = skeys[si];
            if (k[0] == 'K' || k[0] == 'P' || k[0] == 'Q') {   /* kbfix: raw Amiga keycode in hex: K = tap (down+up), P = down, Q = up; K01 = '1', K44 = Return, K40 = space */
                uint8_t rk = (uint8_t)(strtoul(k + 1, NULL, 16) & 0x7f);
                if (k[0] != 'Q') kbd_push(rk, 1);
                if (k[0] != 'P') kbd_push(rk, 0);
                si++; continue;
            }
            joy_up = !!strchr(k, 'U'); joy_down = !!strchr(k, 'D'); joy_left = !!strchr(k, 'L');
            joy_right = !!strchr(k, 'R'); joy_fire = !!strchr(k, 'F');
            { extern int mouse_lmb, mouse_rmb, mouse_x, mouse_y, joy_fire2;   /* g39 mouse-port tokens */
              mouse_lmb = !!strchr(k, 'V'); mouse_rmb = !!strchr(k, 'W'); joy_fire2 = !!strchr(k, 'X');
              if (strchr(k, 'M')) { mouse_x += 10; mouse_y += 10; } }
            if (strchr(k, 'S')) { kbd_push(0x40, 1); kbd_push(0x40, 0); }
            for (const char *q = k; *q; q++) if (*q >= '1' && *q <= '8') { kbd_push((uint8_t)(0x50 + (*q - '1')), 1); kbd_push((uint8_t)(0x50 + (*q - '1')), 0); }     /* script keys 1-8 = F1-F8, as the port's main.c (15 Oct) */
            if (strchr(k, 'S')) { kbd_push(0x40, 1); kbd_push(0x40, 0); }
            si++;
        }
        if (whd_exit_reason) { fprintf(stderr, "WHD stopped at frame %d\n", fr); frames = fr; break; }
        chip_frame_done = 0; frame_start_cyc = now(); idle_cyc = 0;
        while (!chip_frame_done) {
            ipl();
            /* Musashi finishes the instruction that crosses the line's budget; carry that overshoot into the next
             * line so a line is CYC_PER_LINE long on average, as in the port (14 Oct 2026: without this the reference
             * CPU ran about 1% fast against the beam and G3 drifted for 300 frames of Turrican 2's intro) */
            static int carry;
            int want = (int)(CYC_PER_LINE / cpu_scale) - carry; if (want < 1) want = 1;
            int c = m68k_execute(want);
            carry = c - want; if (carry < 0) carry = 0;
            cyc_done += (uint64_t)c;
            /* keep the chipset at most one line behind */
            chip_end_line();
            ipl();
        }
        {   /* STATELOG=file STATEADDR=hex,hex,...: game variables (bytes) read from chip RAM at the end of every tick: the judge's state-variable check */
            static FILE *sf; static int si; static unsigned addr[64]; static int na;
            if (!si) { si = 1; const char *e = getenv("STATELOG"), *a = getenv("STATEADDR"); if (e && a) { sf = fopen(e, "w"); char *c = strdup(a); for (char *t = strtok(c, ","); t && na < 64; t = strtok(NULL, ",")) addr[na++] = (unsigned)strtoul(t, 0, 16);
                fprintf(sf, "tick"); for (int i = 0; i < na; i++) fprintf(sf, " %04X", addr[i]); fprintf(sf, "\n"); } }
            if (sf) { fprintf(sf, "%d", fr + 1); for (int i = 0; i < na; i++) fprintf(sf, " %02X", chipram[addr[i] & 0x7ffff]); fprintf(sf, "\n"); }
        }
        { static FILE *hl; static int init; if (!init) { init = 1; if (getenv("HASHLOG")) hl = fopen(getenv("HASHLOG"), "w"); }
          if (hl) fprintf(hl, "%d %016llx %016llx\n", fr + 1, (unsigned long long)hash_frame(), (unsigned long long)hash_ram()); }
        /* WAVOUT=FILE: the mixed Paula output, raw 44.1 kHz stereo s16 (G3 by code compares it with the port's --wav) */
        { static FILE *wo; static int wi; if (!wi) { wi = 1; if (getenv("WAVOUT")) wo = fopen(getenv("WAVOUT"), "wb"); }
          if (wo) fwrite(audio_buf, 4, (size_t)audio_len, wo); audio_len = 0; }
        if (!busyf && getenv("BUSYLOG")) busyf = fopen(getenv("BUSYLOG"), "w");
        if (busyf) fprintf(busyf, "%d %llu\n", fr + 1, (unsigned long long)idle_cyc);
        if (getenv("DUMPALL") || fr == frames - 1 || (fr % 50) == 49) {
            char p[512]; snprintf(p, sizeof p, "%s/f%05d.%s", outdir, fr + 1, amigo_img_ext()); save_ppm(p);
        }
    }
    if (getenv("RDLOG")) for (int r = 0; r < 0x1100; r++) if (rdcount[r][0]) fprintf(stderr, "read %04X count %u lastpc %06X\n", r, rdcount[r][0], rdpc[r]);
    char p[512]; snprintf(p, sizeof p, "%s/executed_pcs.txt", outdir);
    FILE *f = fopen(p, "w"); unsigned cnt = 0;
    unsigned kcnt = 0;
    for (unsigned a = 0; a < 0x1000000; a++) if (seen[a >> 3] & (1 << (a & 7))) { if (in_kick(a)) { kcnt++; continue; } fprintf(f, "%06X\n", a); cnt++; }
    fclose(f);
    if (whd_kick_size) { snprintf(p, sizeof p, "%s/kick_pcs.txt", outdir); f = fopen(p, "w"); for (unsigned a = 0; a < 0x1000000; a++) if ((seen[a >> 3] & (1 << (a & 7))) && in_kick(a)) fprintf(f, "%06X\n", a); fclose(f);
      fprintf(stderr, "kickstart pcs %u (at $%06X, size $%X, left out of the outputs)\n", kcnt, whd_kick_base, whd_kick_size); }
    /* for the translator: each executed PC with the opcode word it first ran with (V = it changed later,
     * self-modifying or overlaid code), and RAM at the end of the run (code the game copied or loaded) */
    snprintf(p, sizeof p, "%s/executed_ops.txt", outdir); f = fopen(p, "w");
    for (unsigned a = 0; a < 0x1000000; a++) if ((seen[a >> 3] & (1 << (a & 7))) && !in_kick(a)) fprintf(f, "%06X %04X%s\n", a, firstop[a], (volat[a >> 3] & (1 << (a & 7))) ? " V" : "");
    fclose(f);
    snprintf(p, sizeof p, "%s/ram_chip.bin", outdir); f = fopen(p, "wb"); fwrite(chipram, 1, CHIP_SIZE, f); fclose(f);
    snprintf(p, sizeof p, "%s/ram_slow.bin", outdir); write_slow(p);   /* G2E10 kick: Kickstart zeroed */
    if (wslave) whd_report();
    fprintf(stderr, "frames %d instructions %llu unique pcs %u pc=%06X\n", frames, (unsigned long long)instr_count, cnt, m68k_get_reg(NULL, M68K_REG_PC));
    if (trace) fclose(trace);
    return 0;
}
