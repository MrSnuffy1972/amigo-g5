/* whd.c: WHDLoad slaves without WHDLoad (Ami-Go, 14 Oct 2026).
 *
 * Karl's decision (kit/NATIVE-RUNTIME.md): WHDLoad games run through a clean-room C implementation of the resload
 * interface, with no WHDLoad binary and no Kickstart. The slave (a one-hunk, PC-relative 68000 program) is loaded
 * outside BaseMem, a 36-slot jump tower is built, and ws_GameLoader is entered as WHDLoad would: supervisor mode,
 * SR=$2000, SSP at the top of BaseMem, USP=SSP-$400, A0 = the tower. When the 68000 code reaches a slot, the C
 * function for it runs (through the HLE mechanism in boot.c) and returns to the caller.
 *
 * Written from the documented interface (the WHDLoad autodoc and the documented values of the tags and patch-list
 * commands). WHDLoad's own include files are not copied (they are all rights reserved); the constants below are
 * defined here from the documented numbers.
 *
 * Files: in a build-time run the install folder on disk (environment WHD_DIR); in a shipped port the files embedded
 * by portgen (gamefile()). Saves go to WHD_SAVE (default out/port/whdsave), never into the install.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/stat.h>
#ifdef _WIN32
#include <direct.h>
#define mkdir(p, m) _mkdir(p)
#endif
#include "amiga.h"
#include "cpu.h"
#include "boot.h"
#include "whd.h"

/* memory layout inside the slow RAM ($C00000-): ExpMem from the bottom, the tower, the stubs and the slave at the top.
 * As before for a slave that fits below $C80000 in the 512 KB model: tower $C7E000, stubs $C7E0A0, slave $C7E100.
 * k18 (14 Oct 2026): a larger slave, or a larger slow RAM (SLOW_SIZE), moves the three up to the top of slow RAM:
 * slave at (top - size) rounded down to $100, tower $100 below it (measured: 8028/8160-byte slaves did not fit).
 * kit/amigo/port/portg0.py whd_slave_addr() computes the same address for the translator. */
static uint32_t WHD_TOWER = 0xC7E000u, WHD_STUBS = 0xC7E0A0u, WHD_SLAVE = 0xC7E100u;  /* +0 interrupt stub, +16 exception stub, +32 game-loader return stub */
#define WHD_TOP     (SLOW_BASE + (uint32_t)SLOW_SIZE)
#define WHD_EXPMEM  SLOW_BASE
static void whd_layout(uint32_t clen) {
    if (SLOW_SIZE == 512 * 1024 && 0xC7E100u + clen <= WHD_TOP) { WHD_TOWER = 0xC7E000u; WHD_STUBS = 0xC7E0A0u; WHD_SLAVE = 0xC7E100u; return; }
    WHD_SLAVE = (WHD_TOP - clen) & ~0xFFu; WHD_TOWER = WHD_SLAVE - 0x100u; WHD_STUBS = WHD_TOWER + 0xA0u;
}

int whd_active;
uint32_t whd_slave_addr, whd_basemem, whd_expmem, whd_expsize;
static uint16_t ws_version, ws_flags;
static char whd_curdir[128];
static FILE *whd_log;
int whd_exit_reason;                /* set when the slave aborts or the game loader returns */
unsigned whd_calls[36];

static void wlog(const char *fmt, ...) __attribute__((format(printf, 1, 2)));
#include <stdarg.h>
static void wlog(const char *fmt, ...) {
    if (!whd_log) { const char *p = getenv("WHD_LOG"); whd_log = p ? fopen(p, "w") : stderr; if (!whd_log) whd_log = stderr; }
    if (getenv("WHDTIME")) fprintf(whd_log, "[%llu:%d] ", (unsigned long long)chip_frame_count, chip_line());   /* when, for timing comparisons */
    va_list ap; va_start(ap, fmt); vfprintf(whd_log, fmt, ap); va_end(ap); fflush(whd_log);
}

/* ------------------------------------------------------------------ files */
static const char *dir(void) { const char *d = getenv("WHD_DIR"); return d ? d : "."; }

static uint8_t *embedded(const char *name, size_t *len) {   /* shipped port: files embedded by portgen */
    char q[512]; snprintf(q, sizeof q, "%s%s%s", whd_curdir, *whd_curdir ? "/" : "", name);
    size_t n = 0; const uint8_t *e = gamefile(q, &n);
    if (!e) e = gamefile(name, &n);
    if (!e) return NULL;
    uint8_t *b = malloc(n ? n : 1); memcpy(b, e, n); *len = n; return b;
}
/* Save folder: whd_save_dir (set by main.c to the user's profile in SDL builds), else WHD_SAVE, else out/port/whdsave.
 * Read-back (whd_save_reload or WHD_SAVE_RELOAD=1): files saved there are loaded before the embedded copy, so high
 * scores survive a restart. Off for the harness, headless and G3 runs, which behave as before. */
const char *whd_save_dir;
int whd_save_reload;
static void save_path(char *p, size_t n, const char *name);
static uint8_t *save_load(const char *name, size_t *len) {
    const char *r = getenv("WHD_SAVE_RELOAD");
    if (!(whd_save_reload || (r && *r == '1'))) return NULL;
    char p[1300]; save_path(p, sizeof p, name);
    FILE *f = fopen(p, "rb"); if (!f) return NULL;
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    uint8_t *b = malloc(n > 0 ? (size_t)n : 1);
    *len = fread(b, 1, n > 0 ? (size_t)n : 0, f); fclose(f);
    wlog("WHD %s read back from %s (%zu bytes)\n", name, p, *len);
    return b;
}
static uint8_t *file_load(const char *name, size_t *len) {
    { uint8_t *s = save_load(name, len); if (s) return s; }
    if (!getenv("WHD_DIR")) return embedded(name, len);    /* shipped port: never look on the player's disk */
    char p[1024];
    snprintf(p, sizeof p, "%s/%s%s%s", dir(), whd_curdir, *whd_curdir ? "/" : "", name);
    FILE *f = fopen(p, "rb");
    if (!f) {   /* AmigaDOS names are case-insensitive: try a case-insensitive match in the folder */
        char d[1024]; snprintf(d, sizeof d, "%s/%s", dir(), whd_curdir);
        char cmd[1200]; snprintf(cmd, sizeof cmd, "find '%s' -maxdepth 3 -iname '%s' 2>/dev/null | head -1", d, name);
        FILE *pp = popen(cmd, "r"); char q[1024] = "";
        if (pp) { if (fgets(q, sizeof q, pp)) q[strcspn(q, "\n")] = 0; pclose(pp); }
        if (*q) f = fopen(q, "rb");
    }
    if (!f) return embedded(name, len);
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    uint8_t *b = malloc(n > 0 ? (size_t)n : 1);
    *len = fread(b, 1, (size_t)n, f); fclose(f);
    return b;
}
static void save_path(char *p, size_t n, const char *name) {
    const char *s = whd_save_dir ? whd_save_dir : getenv("WHD_SAVE"); s = s ? s : "out/port/whdsave";
    char d[1024]; snprintf(d, sizeof d, "%s", s);
    size_t dl = strlen(d); while (dl > 1 && (d[dl - 1] == '/' || d[dl - 1] == '\\')) d[--dl] = 0;   /* SDL_GetPrefPath ends in one */
    mkdir(d, 0777);
    snprintf(p, n, "%s/%s", d, name);
    for (char *c = p + strlen(d) + 1; *c; c++) if (*c == '/') *c = '_';
}
static void cstr(uint32_t a, char *out, int max) {
    int i = 0; for (; i < max - 1; i++) { uint8_t c = (uint8_t)mem_rb(a + i); if (!c) break; out[i] = (char)c; } out[i] = 0;
}

/* ------------------------------------------------------------------ decrunching (ancient, BSD 2-Clause, linked when present) */
__attribute__((weak)) long whd_decrunch(const uint8_t *src, size_t srclen, uint8_t **out) { (void)src; (void)srclen; (void)out; return -1; }
static const char *packed_kind(const uint8_t *b, size_t n) {
    if (n < 4) return NULL;
    if (!memcmp(b, "RNC\001", 4) || !memcmp(b, "RNC\002", 4)) return "RNC";
    if (!memcmp(b, "IMP!", 4) || !memcmp(b, "ATN!", 4)) return "Imploder";
    if (!memcmp(b, "TPWM", 4)) return "TPWM";
    if (!memcmp(b, "CrM!", 4) || !memcmp(b, "CrM2", 4) || !memcmp(b, "Crm!", 4) || !memcmp(b, "Crm2", 4)) return "CrunchMania";
    return NULL;
}
/* returns the unpacked bytes (malloc) or NULL if not packed; aborts if packed and no decruncher can do it */
static uint8_t *unpack(const uint8_t *b, size_t n, size_t *outlen) {
    const char *k = packed_kind(b, n);
    if (!k) return NULL;
    uint8_t *o = NULL; long r = whd_decrunch(b, n, &o);
    if (r < 0) { wlog("WHD NEED DECRUNCH: %s data, no decruncher linked\n", k); whd_exit_reason = 100; return NULL; }
    *outlen = (size_t)r; return o;
}

/* ------------------------------------------------------------------ the resload functions */
static void ret(void) {    /* return to the caller of jsr (slot,An); condition codes follow D0 as documented */
    cpu.PC = mem_rl(cpu.ar[7]); cpu.ar[7] += 4;
    cpu.N = (cpu.dr[0] >> 31) & 1; cpu.Z = cpu.dr[0] == 0; cpu.V = 0; cpu.C = 0;
}
static void fail_or_ret(const char *what, const char *name, uint32_t err) {
    if (ws_flags & 2) {    /* WHDLF_NoError: any failure ends the program */
        wlog("WHD ABORT: %s %s failed (error %u) with NoError set\n", what, name, err);
        whd_exit_reason = 101; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return;
    }
    cpu.dr[0] = 0; cpu.dr[1] = err; ret();
}
static void put(uint32_t dst, const uint8_t *b, size_t n) { for (size_t i = 0; i < n; i++) mem_wb(dst + (uint32_t)i, b[i]); }

static void r_abort(void) {   /* reached by JMP: (a7)=reason, 4(a7)=primary, 8(a7)=secondary */
    uint32_t sp = cpu.ar[7];
    int32_t reason = (int32_t)mem_rl(sp); uint32_t p1 = mem_rl(sp + 4), p2 = mem_rl(sp + 8);
    wlog("WHD ABORT: reason %d primary $%08X secondary $%08X (TDREASON_OK=-1 means a normal quit)\n", reason, p1, p2);
    whd_exit_reason = reason == -1 ? 1 : 2; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32;
}
static void r_loadfile(void) {
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len; uint8_t *b = file_load(n, &len);
    if (!b) { fail_or_ret("LoadFile", n, 205); return; }
    put(cpu.ar[1], b, len); free(b);
    wlog("WHD LoadFile %s -> $%06X (%zu bytes)\n", n, cpu.ar[1], len);
    cpu.dr[0] = (uint32_t)len; cpu.dr[1] = 0; ret();
}
static void r_loadfileoffset(void) {
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len; uint8_t *b = file_load(n, &len);
    uint32_t size = cpu.dr[0], off = cpu.dr[1];
    if (!b || off > len) { free(b); fail_or_ret("LoadFileOffset", n, 205); return; }
    if (off + size > len) size = (uint32_t)(len - off);
    put(cpu.ar[1], b + off, size); free(b);
    wlog("WHD LoadFileOffset %s +%u -> $%06X (%u bytes)\n", n, off, cpu.ar[1], size);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();   /* autodoc: D0 = BOOL success (TRUE = -1), not a length */
}
static void r_loadfiledecrunch(void) {
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len; uint8_t *b = file_load(n, &len);
    if (!b) { fail_or_ret("LoadFileDecrunch", n, 205); return; }
    size_t ul = 0; uint8_t *u = unpack(b, len, &ul);
    if (whd_exit_reason == 100) { free(b); cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
    if (u) { put(cpu.ar[1], u, ul); free(u); len = ul; } else put(cpu.ar[1], b, len);
    free(b);
    wlog("WHD LoadFileDecrunch %s -> $%06X (%zu bytes%s)\n", n, cpu.ar[1], len, u ? ", unpacked" : "");
    cpu.dr[0] = (uint32_t)len; cpu.dr[1] = 0; ret();
}
static void r_decrunch(void) {
    uint32_t src = cpu.ar[0], dst = cpu.ar[1];
    uint8_t hdr[12]; for (int i = 0; i < 12; i++) hdr[i] = (uint8_t)mem_rb(src + i);
    const char *k = packed_kind(hdr, 12);
    if (!k) { cpu.dr[0] = 0; ret(); return; }
    /* packed length is format-specific; read generously (to the end of the memory area) and let the decruncher stop */
    size_t n = 0x80000; uint8_t *b = malloc(n); for (size_t i = 0; i < n; i++) b[i] = (uint8_t)mem_rb(src + (uint32_t)i);
    size_t ul = 0; uint8_t *u = unpack(b, n, &ul); free(b);
    if (!u) { cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
    put(dst, u, ul); free(u);
    wlog("WHD Decrunch %s $%06X -> $%06X (%zu bytes)\n", k, src, dst, ul);
    cpu.dr[0] = (uint32_t)ul; ret();
}
static void r_getfilesize(void) {
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    cpu.dr[0] = b ? (uint32_t)len : 0; free(b); ret();
}
static void r_getfilesizedec(void) {
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    if (b && len >= 8 && packed_kind(b, len) && !memcmp(b, "RNC", 3)) len = (size_t)b[4] << 24 | b[5] << 16 | b[6] << 8 | b[7];
    cpu.dr[0] = b ? (uint32_t)len : 0; free(b); ret();
}
static void r_savefile(void) {
    char n[256], p[1300]; cstr(cpu.ar[0], n, sizeof n); save_path(p, sizeof p, n);
    FILE *f = fopen(p, "wb");
    if (f) { for (uint32_t i = 0; i < cpu.dr[0]; i++) fputc((int)mem_rb(cpu.ar[1] + i), f); fclose(f); }
    wlog("WHD SaveFile %s (%u bytes) -> %s\n", n, cpu.dr[0], p);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
static void r_savefileoffset(void) {
    char n[256], p[1300]; cstr(cpu.ar[0], n, sizeof n); save_path(p, sizeof p, n);
    FILE *f = fopen(p, "r+b"); if (!f) f = fopen(p, "w+b");
    if (f) { fseek(f, (long)cpu.dr[1], SEEK_SET); for (uint32_t i = 0; i < cpu.dr[0]; i++) fputc((int)mem_rb(cpu.ar[1] + i), f); fclose(f); }
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
static void r_deletefile(void) {
    char n[256], p[1300]; cstr(cpu.ar[0], n, sizeof n); save_path(p, sizeof p, n);
    char q[1400]; snprintf(q, sizeof q, "%s.deleted", p); rename(p, q);   /* moved aside, never removed */
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
static void r_diskload(void) {
    char n[32]; snprintf(n, sizeof n, "Disk.%u", cpu.dr[2] & 0xFF);   /* the disk number is the LOW BYTE of D2 (Cowork 15 Oct: Disk.65537 etc.) */ size_t len; uint8_t *b = file_load(n, &len);
    uint32_t off = cpu.dr[0], size = cpu.dr[1];
    if (!b || off + size > len) { free(b); fail_or_ret("DiskLoad", n, 205); return; }
    put(cpu.ar[0], b + off, size); free(b);
    wlog("WHD DiskLoad disk %u +$%X -> $%06X (%u bytes)\n", cpu.dr[2] & 0xFF, off, cpu.ar[0], size);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
uint16_t whd_crc16(uint32_t a, uint32_t n) {   /* "ANSI conform" CRC-16: polynomial $8005 reflected ($A001), start 0 */
    uint16_t c = 0;
    for (uint32_t i = 0; i < n; i++) { c ^= (uint16_t)mem_rb(a + i); for (int k = 0; k < 8; k++) c = (c & 1) ? (uint16_t)((c >> 1) ^ 0xA001) : (uint16_t)(c >> 1); }
    return c;
}
static void r_crc16(void) { cpu.dr[0] = whd_crc16(cpu.ar[0], cpu.dr[0]); ret(); }

/* resload_Control tags: TAG_USER ($80000000) + $8000000 + n, in the documented order */
#define WTAG(n) (0x88000000u + (n))
int whd_custom[5];
static void r_control(void) {
    for (uint32_t t = cpu.ar[0];; t += 8) {
        uint32_t tag = mem_rl(t);
        if (tag == 0) break;                         /* TAG_DONE */
        if (tag == 1) continue;                      /* TAG_IGNORE */
        if (tag == 2) { t = mem_rl(t + 4) - 8; continue; }   /* TAG_MORE */
        if (tag == 3) { t += 8 * mem_rl(t + 4); continue; }  /* TAG_SKIP */
        uint32_t v; int get = 1;
        switch (tag - 0x88000000u) {
        case 0: v = 0; break;                         /* ATTNFLAGS: plain 68000 */
        case 1: v = 709379; break;                    /* ECLOCKFREQ: PAL E clock */
        case 2: v = 0x00029004; break;                /* MONITOR: PAL_MONITOR_ID */
        case 6: v = 0; break;                         /* BUTTONWAIT off */
        case 7: case 8: case 9: case 10: case 11: v = (uint32_t)whd_custom[tag - WTAG(7)]; break;
        case 13: v = 0; break;                        /* CHIPREVBITS: OCS */
        case 14: v = 0; break;                        /* IOERR */
        case 17: v = 20; break; case 18: v = 1; break; case 19: v = 0; break;   /* VERSION 20.1 */
        case 20: v = 0; break;                        /* TIME: no clock */
        case 21: v = 0x0200; break;                   /* BPLCON0 of the OS */
        case 27: v = 0; break;                        /* LANG */
        case 40: v = whd_expsize; break;              /* EXPMEMSIZE */
        default: get = 0; v = 0; break;               /* setters and debug checks: accepted, nothing to do */
        }
        if (get) mem_wl(t + 4, v);
    }
    cpu.dr[0] = 0xffffffffu; ret();
}

/* ------------------------------------------------------------------ patch lists (documented command numbers) */
static int custom_val(int i) { return (i >= 0 && i < 5) ? whd_custom[i] : 0; }
static void patch_list(uint32_t list, uint32_t base) {
    uint32_t p = list;
    /* IF blocks nest (WHDLoad allows 31 deep): cond[d] = this block's own condition, on[d] = whether its
     * current branch is taken (all enclosing branches taken and the condition true, or false after ELSE) */
    int depth = 0, cond[40], on[40]; on[0] = 1;
    for (int guard = 0; guard < 100000; guard++) {
        int skip = !on[depth];
        uint16_t cw = (uint16_t)mem_rw(p); p += 2;
        uint16_t cmd = cw & 0x3fff; int ctrl = cw & 0x4000;
        if (!ctrl && cmd == 0) return;               /* END */
        if (ctrl) {
            int c = -1;
            if (cmd >= 28 && cmd <= 33) c = cmd == 28 ? 0 : custom_val(cmd - 29) != 0;              /* IFBW, IFC1-5 */
            else if (cmd >= 34 && cmd <= 38) { uint16_t bit = (uint16_t)mem_rw(p); p += 2; c = (custom_val(cmd - 34) >> bit) & 1; }
            else if (cmd >= 47 && cmd <= 51) { uint16_t v = (uint16_t)mem_rw(p); p += 2; c = custom_val(cmd - 47) == (int16_t)v; }
            else if (cmd >= 52 && cmd <= 56) { int16_t lo = (int16_t)mem_rw(p), hi = (int16_t)mem_rw(p + 2); p += 4; int v = custom_val(cmd - 52); c = v >= lo && v <= hi; }
            else if (cmd >= 57 && cmd <= 62) {
                uint32_t a = list + (uint16_t)mem_rw(p); p += 2;
                uint32_t v = (cmd == 57 || cmd == 60) ? mem_rb(a) : (cmd == 58 || cmd == 61) ? mem_rw(a) : mem_rl(a);
                if (cmd >= 60) { uint32_t e = cmd == 62 ? mem_rl(p) : mem_rw(p); p += cmd == 62 ? 4 : 2; c = v == e; } else c = v != 0;
            }
            if (cmd == 39) { if (depth > 0) on[depth] = on[depth - 1] && !cond[depth]; continue; }          /* ELSE */
            if (cmd == 40) { if (depth > 0) depth--; continue; }                                              /* ENDIF */
            if (c < 0 || depth >= 39) { wlog("WHD PATCH: unknown control command %u at $%06X\n", cmd, p - 2); return; }
            depth++; cond[depth] = c; on[depth] = on[depth - 1] && c;
            continue;
        }
        uint32_t adr = (cw & 0x8000) ? (uint16_t)mem_rw(p) : mem_rl(p); p += (cw & 0x8000) ? 2 : 4;
        uint32_t a = base + adr;
        uint32_t arg16 = 0;
        switch (cmd) {
        case 1: if (!skip) mem_ww(a, 0x4E75); break;                                        /* R */
        case 2: case 3: arg16 = (uint16_t)mem_rw(p); p += 2;                                 /* P, PS */
                if (!skip) { mem_ww(a, cmd == 2 ? 0x4EF9 : 0x4EB9); mem_wl(a + 2, list + arg16); } break;
        case 4: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) { mem_ww(a, 0x6000); mem_ww(a + 2, arg16); } break;   /* S */
        case 5: if (!skip) mem_ww(a, 0x4AFC); break;                                        /* I */
        case 6: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wb(a, arg16 & 0xff); break;     /* B */
        case 7: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_ww(a, arg16); break;            /* W */
        case 8: { uint32_t v = mem_rl(p); p += 4; if (!skip) mem_wl(a, v); } break;                 /* L */
        case 9: { uint32_t v = mem_rl(p); p += 4; if (!skip) mem_wl(a, base + v); } break;          /* A */
        case 10: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wl(a, list + arg16); break;    /* PA */
        case 11: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) for (uint32_t i = 0; i + 1 < arg16 + 1u && i < arg16; i += 2) mem_ww(a + i, 0x4E71); break;   /* NOP */
        case 12: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) for (uint32_t i = 0; i < arg16; i++) mem_wb(a + i, 0); break;   /* C */
        case 13: if (!skip) mem_wb(a, 0); break; case 14: if (!skip) mem_ww(a, 0); break; case 15: if (!skip) mem_wl(a, 0); break;
        case 16: { uint16_t d = (uint16_t)mem_rw(p), nb = (uint16_t)mem_rw(p + 2); p += 4;          /* PSS */
                   if (!skip) { mem_ww(a, 0x4EB9); mem_wl(a + 2, list + d); for (uint32_t i = 0; i < nb; i += 2) mem_ww(a + 6 + i, 0x4E71); } } break;
        case 17: { uint16_t d = (uint16_t)mem_rw(p); p += 2; list = list + d; p = list; continue; }   /* NEXT */
        case 18: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wb(a, (mem_rb(a) + arg16) & 0xff); break;   /* AB */
        case 19: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_ww(a, (mem_rw(a) + arg16) & 0xffff); break; /* AW */
        case 20: { uint32_t v = mem_rl(p); p += 4; if (!skip) mem_wl(a, mem_rl(a) + v); } break;                /* AL */
        case 21: { uint16_t nb = (uint16_t)mem_rw(p); p += 2; if (!skip) for (uint32_t i = 0; i < nb; i++) mem_wb(a + i, mem_rb(p + i)); p += (nb + 1u) & ~1u; } break;   /* DATA */
        case 22: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wb(a, mem_rb(a) | (arg16 & 0xff)); break;  /* ORB */
        case 23: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_ww(a, mem_rw(a) | arg16); break;           /* ORW */
        case 24: { uint32_t v = mem_rl(p); p += 4; if (!skip) mem_wl(a, mem_rl(a) | v); } break;                /* ORL */
        case 25: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wl(list + arg16, a); break;                /* GA */
        case 26: if (!skip) mem_ww(a, 0x4AFC); break;                                                           /* BKPT */
        case 27: p += 2; break;                                                                                 /* BELL */
        case 41: case 42: case 43: arg16 = (uint16_t)mem_rw(p); p += 2;                                         /* VB VW VL */
                 if (!skip) { if (cmd == 41) mem_wb(a, mem_rb(list + arg16)); else if (cmd == 42) mem_ww(a, mem_rw(list + arg16)); else mem_wl(a, mem_rl(list + arg16)); } break;
        case 44: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_wb(a, mem_rb(a) & arg16); break;           /* ANDB */
        case 45: arg16 = (uint16_t)mem_rw(p); p += 2; if (!skip) mem_ww(a, mem_rw(a) & arg16); break;           /* ANDW */
        case 46: { uint32_t v = mem_rl(p); p += 4; if (!skip) mem_wl(a, mem_rl(a) & v); } break;                /* ANDL */
        default: wlog("WHD PATCH: unknown command %u at $%06X; patch list abandoned\n", cmd, p - 2); return;
        }
    }
}
static void r_patch(void) { wlog("WHD Patch list $%06X base $%06X\n", cpu.ar[0], cpu.ar[1]); patch_list(cpu.ar[0], cpu.ar[1]); ret(); }
static void r_patchseg(void) {
    /* a BCPL segment list: each segment's BPTR points at (next BPTR, data); patch addresses count across the
     * segments' data as if they were one block (WHDLoad 19.1+ semantics); here applied segment by segment */
    uint32_t seg = cpu.ar[1] << 2;
    wlog("WHD PatchSeg list $%06X seglist $%06X\n", cpu.ar[0], seg);
    if (seg) patch_list(cpu.ar[0], seg + 4);
    ret();
}
static void r_nop_ret0(void) { cpu.dr[0] = 0; ret(); }
static void r_flush(void) { ret(); }
/* resload_Relocate (autodoc): size = resload_Relocate(A0 address, A1 tags). Relocates a standard AmigaDOS hunk executable that
 * sits in memory at `address`, over itself, with hunk starts rounded up to the alignment (default 4). Memory-flagged hunks go
 * to the CHIPPTR / FASTPTR areas when the tag list names them. The WHDLTAG_* numbers are not in the user package, so the tags
 * are told apart by their value: a longword-aligned address inside chip RAM is CHIPPTR, one inside slow/fast RAM is FASTPTR, a
 * power of two up to 4096 is ALIGN (14 Oct 2026, most-hit missing resload function: games 17, 49, 149, 248). */
static void r_relocate(void) {
    uint32_t src = cpu.ar[0], tags = cpu.ar[1], align = 4, chipptr = 0, fastptr = 0, chip_t = 0, fast_t = 0;
    for (uint32_t t = tags, n = 0; tags && n < 24; t += 8, n++) {
        uint32_t tag = mem_rl(t), v = mem_rl(t + 4);
        if (tag == 0) break;
        if (tag == 1) continue;
        if (tag == 2) { t = v - 8; continue; }
        if (tag == 3) { t += 8 * v; continue; }
        if (v >= 0x400 && v < CHIP_SIZE && !(v & 3)) { chipptr = v; chip_t = t + 4; }
        else if (v >= SLOW_BASE && v < WHD_TOP && !(v & 3)) { fastptr = v; fast_t = t + 4; }
        else if (v && v <= 4096 && !(v & (v - 1))) align = v;
        wlog("WHD Relocate tag $%08X = $%08X\n", tag, v);
    }
    uint32_t p = src;
    if (mem_rl(p) != 0x3F3) { wlog("WHD Relocate: $%06X is not a hunk executable\n", src); whd_exit_reason = 101; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
    p += 4;
    for (uint32_t n; (n = mem_rl(p)) != 0; ) p += 4 + 4 * n;     /* resident library names (none in practice) */
    p += 4;
    uint32_t first = mem_rl(p + 4), last = mem_rl(p + 8); p += 12;
    uint32_t nh = last - first + 1;
    if (nh == 0 || nh > 64) { wlog("WHD Relocate: %u hunks is not usable\n", nh); whd_exit_reason = 101; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
    uint32_t hsize[64], hflag[64], hdata[64], hdest[64]; int hbss[64];
    uint32_t *rel_off = malloc(4 * 262144), *rel_hunk = malloc(4 * 262144), *rel_tgt = malloc(4 * 262144); uint32_t nrel = 0;
    for (uint32_t i = 0; i < nh; i++) {
        uint32_t w = mem_rl(p); p += 4;
        hflag[i] = w >> 30; hsize[i] = (w & 0x3FFFFFFFu) * 4;
        if (hflag[i] == 3) p += 4;                               /* extended memory attributes follow */
        hdata[i] = 0; hbss[i] = 1;
    }
    for (uint32_t i = 0; i < nh; i++) {
        for (;;) {
            uint32_t ty = mem_rl(p) & 0x3FFFFFFFu; p += 4;
            if (ty == 0x3E9 || ty == 0x3EA) { uint32_t n = mem_rl(p); p += 4; hdata[i] = p; hbss[i] = 0; p += 4 * n; }
            else if (ty == 0x3EB) { p += 4; }
            else if (ty == 0x3EC) {
                for (uint32_t n; (n = mem_rl(p)) != 0; ) {
                    uint32_t tg = mem_rl(p + 4); p += 8;
                    for (uint32_t k = 0; k < n && nrel < 262144; k++, p += 4) { rel_off[nrel] = mem_rl(p); rel_hunk[nrel] = i; rel_tgt[nrel] = tg; nrel++; }
                }
                p += 4;
            }
            else if (ty == 0x3F7 || ty == 0x3FC) {                /* DRELOC32 / RELOC32SHORT: 16-bit counts, hunk numbers and offsets */
                for (uint32_t n; (n = mem_rw(p)) != 0; ) {
                    uint32_t tg = mem_rw(p + 2); p += 4;
                    for (uint32_t k = 0; k < n && nrel < 262144; k++, p += 2) { rel_off[nrel] = mem_rw(p); rel_hunk[nrel] = i; rel_tgt[nrel] = tg; nrel++; }
                }
                p += 2; p = (p + 3) & ~3u;
            }
            else if (ty == 0x3F0) { for (uint32_t n; (n = mem_rl(p) & 0xFFFFFF) != 0; ) p += 8 + 4 * n; p += 4; }
            else if (ty == 0x3F1) { p += 4 + 4 * mem_rl(p); }
            else if (ty == 0x3F2) break;
            else { wlog("WHD Relocate: hunk type $%03X is not handled\n", ty); free(rel_off); free(rel_hunk); free(rel_tgt); whd_exit_reason = 101; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
        }
    }
    uint32_t flen = p - src;
    uint8_t *img = malloc(flen ? flen : 1);
    for (uint32_t i = 0; i < flen; i++) img[i] = (uint8_t)mem_rb(src + i);      /* the executable is relocated over itself: keep a copy */
    uint32_t cur = src, chipcur = chipptr, fastcur = fastptr;
    for (uint32_t i = 0; i < nh; i++) {
        uint32_t *c = (hflag[i] == 1 && chipptr) ? &chipcur : (hflag[i] == 2 && fastptr) ? &fastcur : &cur;
        *c = (*c + align - 1) / align * align; hdest[i] = *c; *c += hsize[i];
    }
    for (uint32_t i = 0; i < nh; i++) {
        for (uint32_t k = 0; k < hsize[i]; k++) mem_wb(hdest[i] + k, hbss[i] || !hdata[i] ? 0 : img[hdata[i] - src + k]);
    }
    for (uint32_t k = 0; k < nrel; k++) {
        uint32_t tg = rel_tgt[k];
        if (tg >= nh) continue;
        uint32_t a = hdest[rel_hunk[k]] + rel_off[k];
        mem_wl(a, mem_rl(a) + hdest[tg]);
    }
    if (chip_t) mem_wl(chip_t, chipcur - chipptr);
    if (fast_t) mem_wl(fast_t, fastcur - fastptr);
    wlog("WHD Relocate $%06X: %u hunks, %u relocations, size %u%s%s\n", src, nh, nrel, cur - src, chipptr ? ", chip area" : "", fastptr ? ", fast area" : "");
    free(img); free(rel_off); free(rel_hunk); free(rel_tgt);
    cpu.dr[0] = cur - src; cpu.dr[1] = 0; ret();
}

/* ------------------------------------------------------------------ G2E10 kick: resload LoadKick, Examine / ExNext / ListFiles
 * Ported from the live G0/G1 kit (runtime/whd.c kit-kick 15 Oct, KICKEMU + EMBDOS 26 Oct, tag g1kickemu). A kickemu slave boots a
 * real Kickstart (the PLAYER'S own file, runtime/rtkick.c) in ExpMem; its DOS handler asks resload for the game's files by name.
 * Names are relative to the slave's data directory, matched without regard to case. Build-time runs with WHD_DIR read the install
 * folder on disk; otherwise (G2 reference, ports) the answers come from the embedded files (game_files[], names relative to the slave
 * folder; empty folders as "name/" markers, portgen EMBDIR). FileInfoBlock as dos/dos.i: DiskKey 0, DirEntryType 4, FileName 8,
 * EntryType 120, Size 124, NumBlocks 128. ExNext keeps its place in fib_DiskKey (handle << 16 | next entry). */
#include "rtkick.h"
#include <dirent.h>
#include <strings.h>
static void kick_wb(uint32_t a, uint32_t v) { mem_wb(a, v); }
static int load_kick(const char *name, uint32_t size, uint16_t crc) {
    if (!whd_expmem || whd_expsize < size) { wlog("WHD LoadKick %s: ExpMem $%X is smaller than the image ($%X)\n", name, whd_expsize, size); return 1; }
    if (kick_has(name, ".a1200") || kick_has(name, ".a4000") || kick_has(name, ".a3000") || kick_has(name, ".cd32")) {
        wlog("WHD LoadKick %s: an A1200-class Kickstart is 68020 code; this kit runs a 68000 (excluded games)\n", name); return 1; }
    char lg[2600];
    if (kick_load(whd_expmem, name, size, crc, kick_wb, lg, sizeof lg)) { wlog("WHD LoadKick %s: %s\n", name, kick_missing); fprintf(stderr, "%s\n", kick_missing); return 1; }
    wlog("WHD %s\n", lg);
    return 0;
}
static void r_loadkick(void) {   /* resload_LoadKick(D0 size, D1 crc16, A0 name): only returns on success */
    char n[256]; cstr(cpu.ar[0], n, sizeof n);
    if (load_kick(n, cpu.dr[0], (uint16_t)cpu.dr[1])) { wlog("WHD ABORT: LoadKick %s failed\n", n); whd_exit_reason = 104; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32; return; }
    ret();
}
typedef struct { const char *name; const unsigned char *data; unsigned len; } emb_file_t;
extern const emb_file_t game_files[]; extern const int game_nfiles;
static char ex_dirs[64][1024]; static int ex_ndirs;
static int ex_handle(const char *key) {
    int h = -1; for (int i = 0; i < ex_ndirs; i++) if (!strcmp(ex_dirs[i], key)) h = i;
    if (h < 0 && ex_ndirs < 64) { h = ex_ndirs++; snprintf(ex_dirs[h], sizeof ex_dirs[h], "%s", key); }
    return h;
}
static void fib_fill(uint32_t fib, const char *name, int isdir, uint32_t size, int isroot) {
    for (uint32_t i = 8; i < 224; i++) mem_wb(fib + i, 0);   /* through fib_Comment only (live kit, 16 Oct: Populous's 240-byte FIB) */
    int32_t type = isroot ? 1 : isdir ? 2 : -3;
    mem_wl(fib + 4, (uint32_t)type); mem_wl(fib + 120, (uint32_t)type);
    for (int i = 0; i < 107 && name[i]; i++) mem_wb(fib + 8 + (uint32_t)i, (uint8_t)name[i]);
    if (isdir) size = 0;
    mem_wl(fib + 124, size); mem_wl(fib + 128, (size + 511) / 512);
}
/* --- embedded files */
static void emb_path(const char *rel, char *q, size_t qn) {
    char t[700]; snprintf(t, sizeof t, "%s%s%s", whd_curdir, (*whd_curdir && *rel) ? "/" : "", rel);
    size_t i = 0;
    for (const char *c = t; *c && i + 1 < qn; c++) { if (*c == '/' && (i == 0 || q[i - 1] == '/')) continue; q[i++] = *c; }
    while (i && q[i - 1] == '/') i--;
    q[i] = 0;
}
static int emb_kind(const char *q, uint32_t *size, char *tname, size_t tn) {   /* 1 file, 2 directory, 0 not found */
    size_t l = strlen(q); *size = 0; if (tn) tname[0] = 0;
    if (!l) return 2;
    int isdir = 0;
    for (int i = 0; i < game_nfiles; i++) {
        const char *nm = game_files[i].name; if (!nm || strncasecmp(nm, q, l)) continue;
        const char *sl = strrchr(q, '/'); size_t b = sl ? (size_t)(sl - q) + 1 : 0;
        if (!nm[l]) { *size = game_files[i].len; snprintf(tname, tn, "%s", nm + b); return 1; }
        if (nm[l] == '/' && !isdir) { isdir = 1; snprintf(tname, tn, "%.*s", (int)(l - b), nm + b); }
    }
    return isdir ? 2 : 0;
}
static int emb_list(const char *q, char ***names, uint8_t **isdir) {
    size_t l = strlen(q); int n = 0, cap = 64; char **v = malloc(sizeof *v * cap); uint8_t *d = malloc((size_t)cap);
    for (int i = 0; i < game_nfiles; i++) {
        const char *nm = game_files[i].name; if (!nm) continue;
        if (l) { if (strncasecmp(nm, q, l) || nm[l] != '/') continue; nm += l + 1; }
        const char *sl = strchr(nm, '/'); size_t sn = sl ? (size_t)(sl - nm) : strlen(nm);
        if (!sn) continue;
        int k; for (k = 0; k < n; k++) if (strlen(v[k]) == sn && !strncasecmp(v[k], nm, sn)) break;
        if (k < n) { if (sl) d[k] = 1; continue; }
        if (n == cap) { cap *= 2; v = realloc(v, sizeof *v * cap); d = realloc(d, (size_t)cap); }
        v[n] = malloc(sn + 1); memcpy(v[n], nm, sn); v[n][sn] = 0; d[n] = sl != NULL; n++;
    }
    for (int a = 1; a < n; a++) for (int b = a; b > 0 && strcasecmp(v[b - 1], v[b]) > 0; b--) {
        char *t = v[b]; v[b] = v[b - 1]; v[b - 1] = t; uint8_t u = d[b]; d[b] = d[b - 1]; d[b - 1] = u; }
    *names = v; *isdir = d; return n;
}
static void free_list(char **v, uint8_t *d, int n) { for (int i = 0; i < n; i++) free(v[i]); free(v); free(d); }
/* --- the install folder on disk (WHD_DIR) */
static int ci_resolve(const char *rel, char *out, size_t on) {
    snprintf(out, on, "%s", dir());
    char tmp[512]; snprintf(tmp, sizeof tmp, "%s%s%s", whd_curdir, *whd_curdir ? "/" : "", rel);
    for (char *c = strtok(tmp, "/"); c; c = strtok(NULL, "/")) {
        if (!*c) continue;
        DIR *dp = opendir(out); if (!dp) return 1;
        struct dirent *e; int ok = 0; size_t l = strlen(out);
        while ((e = readdir(dp))) if (!strcasecmp(e->d_name, c)) { snprintf(out + l, on - l, "/%s", e->d_name); ok = 1; break; }
        closedir(dp); if (!ok) return 1;
    }
    return 0;
}
static int disk_list(const char *p, char ***names, uint8_t **isdir) {
    DIR *dp = opendir(p); if (!dp) return -1;
    int n = 0, cap = 64; char **v = malloc(sizeof *v * cap); uint8_t *d = malloc((size_t)cap); struct dirent *e;
    while ((e = readdir(dp))) { if (e->d_name[0] == '.' && (!e->d_name[1] || (e->d_name[1] == '.' && !e->d_name[2]))) continue;
        if (n == cap) { cap *= 2; v = realloc(v, sizeof *v * cap); d = realloc(d, (size_t)cap); }
        char q[1300]; struct stat st; snprintf(q, sizeof q, "%s/%s", p, e->d_name);
        v[n] = strdup(e->d_name); d[n] = !stat(q, &st) && S_ISDIR(st.st_mode); n++; }
    closedir(dp);
    for (int a = 1; a < n; a++) for (int b = a; b > 0 && strcasecmp(v[b - 1], v[b]) > 0; b--) {
        char *t = v[b]; v[b] = v[b - 1]; v[b - 1] = t; uint8_t u = d[b]; d[b] = d[b - 1]; d[b - 1] = u; }
    *names = v; *isdir = d; return n;
}
static void r_examine(void) {
    char n[256], key[1100], tn[120]; cstr(cpu.ar[0], n, sizeof n); uint32_t fib = cpu.ar[1], sz = 0; int k;
    if (getenv("WHD_DIR")) {
        char p[1024]; struct stat st;
        if (ci_resolve(n, p, sizeof p) || stat(p, &st)) { wlog("WHD Examine '%s': not found\n", n); cpu.dr[0] = 0; cpu.dr[1] = 205; ret(); return; }
        k = S_ISDIR(st.st_mode) ? 2 : 1; sz = k == 1 ? (uint32_t)st.st_size : 0;
        const char *hb = strrchr(p, '/'); snprintf(tn, sizeof tn, "%s", hb ? hb + 1 : p); snprintf(key, sizeof key, "%s", p);
    } else {
        char q[700]; emb_path(n, q, sizeof q); k = emb_kind(q, &sz, tn, sizeof tn);
        if (!k) { wlog("WHD Examine '%s': not found (embedded)\n", n); cpu.dr[0] = 0; cpu.dr[1] = 205; ret(); return; }
        snprintf(key, sizeof key, "\001%s", q);
    }
    fib_fill(fib, *n ? tn : "", k == 2, sz, !*n);
    int h = ex_handle(key); mem_wl(fib, h < 0 ? 0 : (uint32_t)h << 16);
    wlog("WHD Examine '%s' -> %s %u bytes\n", n, k == 2 ? "directory" : "file", sz);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
static void r_exnext(void) {
    uint32_t fib = cpu.ar[0], key = mem_rl(fib), h = key >> 16, idx = key & 0xFFFF;
    if (h >= (uint32_t)ex_ndirs) { cpu.dr[0] = 0; cpu.dr[1] = 232; ret(); return; }
    int emb = ex_dirs[h][0] == 1; const char *q = ex_dirs[h] + emb;
    char **v; uint8_t *d; int n = emb ? emb_list(q, &v, &d) : disk_list(q, &v, &d);
    if (n < 0 || (int)idx >= n) { if (n >= 0) free_list(v, d, n); cpu.dr[0] = 0; cpu.dr[1] = 232; ret(); return; }
    uint32_t sz = 0; char p[1300], tn[120]; snprintf(p, sizeof p, "%s%s%s", q, *q ? "/" : "", v[idx]);
    if (!d[idx]) { if (emb) emb_kind(p, &sz, tn, sizeof tn); else { struct stat st; if (!stat(p, &st)) sz = (uint32_t)st.st_size; } }
    fib_fill(fib, v[idx], d[idx], sz, 0);
    mem_wl(fib, h << 16 | (idx + 1));
    free_list(v, d, n);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}
static void r_listfiles(void) {
    char n[256], p[1024]; cstr(cpu.ar[0], n, sizeof n); uint32_t size = cpu.dr[0], buf = cpu.ar[1];
    char **v; uint8_t *d; int m;
    if (getenv("WHD_DIR")) m = ci_resolve(n, p, sizeof p) ? -1 : disk_list(p, &v, &d);
    else { char q[700], tn[120]; uint32_t sz; emb_path(n, q, sizeof q); m = emb_kind(q, &sz, tn, sizeof tn) == 2 ? emb_list(q, &v, &d) : -1; }
    if (m < 0) { wlog("WHD ListFiles '%s': not found\n", n); fail_or_ret("ListFiles", n, 205); return; }
    uint32_t o = 0, cnt = 0, err = 0;
    for (int i = 0; i < m; i++) {
        if (d[i]) continue;
        size_t l = strlen(v[i]) + 1;
        if (o + l > size) { err = 103; break; }
        for (size_t k = 0; k < l; k++) mem_wb(buf + o + (uint32_t)k, (uint8_t)v[i][k]);
        o += (uint32_t)l; cnt++;
    }
    free_list(v, d, m);
    wlog("WHD ListFiles '%s' -> %u names\n", n, cnt);
    cpu.dr[0] = cnt; cpu.dr[1] = err; ret();
}

static const char *const slot_name[36];
static void r_unimplemented(void) {
    uint32_t slot = (cpu.PC - WHD_TOWER) / 4;
    wlog("WHD NOT IMPLEMENTED: resload %s (slot $%02X) called from $%06X\n", slot < 36 ? slot_name[slot] : "?", slot * 4, mem_rl(cpu.ar[7]));
    whd_exit_reason = 102; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32;
}
static void r_exception(void) {
    if (whd_exit_reason) { cpu.PC = WHD_STUBS + 32; return; }
    wlog("WHD EXCEPTION: a vector without a game handler was taken; stacked PC $%06X SR $%04X\n",
         mem_rl(cpu.ar[7] + 2), mem_rw(cpu.ar[7]));
    whd_exit_reason = 103; cpu.stopped = 1; cpu.PC = WHD_STUBS + 32;
}
static void r_loader_returned(void) {
    if (!whd_exit_reason) { wlog("WHD: ws_GameLoader returned\n"); whd_exit_reason = 3; }
    cpu.stopped = 1; cpu.PC = WHD_STUBS + 32;   /* stay here */
}

static void (*const slot_fn[36])(void) = {
    r_unimplemented,   /* $00 Install (private) */
    r_abort, r_loadfile, r_savefile, r_nop_ret0 /* SetCACR */, r_listfiles, r_decrunch,
    r_loadfiledecrunch, r_flush, r_getfilesize, r_diskload, r_unimplemented /* DiskLoadDev */, r_crc16, r_control,
    r_savefileoffset, r_flush, r_flush, r_flush, r_flush /* Protect*: debug aids */, r_loadfileoffset,
    r_relocate, r_flush /* Delay */, r_deletefile, r_flush /* ProtectSMC */,
    r_nop_ret0 /* SetCPU */, r_patch, r_loadkick /* G2E10 kick */, r_unimplemented /* Delta */,
    r_getfilesizedec, r_patchseg, r_examine, r_exnext,
    r_unimplemented /* GetCustom */, r_unimplemented /* VSNPrintF */, r_unimplemented /* Log */,
    r_unimplemented /* ReadJoyPort */
};
static const char *const slot_name[36] = {"Install", "Abort", "LoadFile", "SaveFile", "SetCACR", "ListFiles", "Decrunch",
    "LoadFileDecrunch", "FlushCache", "GetFileSize", "DiskLoad", "DiskLoadDev", "CRC16", "Control", "SaveFileOffset",
    "ProtectRead", "ProtectReadWrite", "ProtectWrite", "ProtectRemove", "LoadFileOffset", "Relocate", "Delay",
    "DeleteFile", "ProtectSMC", "SetCPU", "Patch", "LoadKick", "Delta", "GetFileSizeDec", "PatchSeg", "Examine",
    "ExNext", "GetCustom", "VSNPrintF", "Log", "ReadJoyPort"};
static void dispatch(void) {
    uint32_t slot = (cpu.PC - WHD_TOWER) / 4;
    if (slot < 36) { whd_calls[slot]++; slot_fn[slot](); }
}
static hle_t whd_hle[39];

const hle_t *whd_hle_find(uint32_t pc) {
    if (!whd_active || pc < WHD_TOWER || pc >= WHD_STUBS + 48) return 0;
    if (pc < WHD_TOWER + 36 * 4) return (pc & 3) ? 0 : &whd_hle[(pc - WHD_TOWER) / 4];
    if (pc == WHD_STUBS + 16) return &whd_hle[36];
    if (pc == WHD_STUBS + 32) return &whd_hle[37];
    return 0;
}

void whd_report(void) {
    char b[1024] = ""; size_t o = 0;
    for (int i = 0; i < 36; i++) if (whd_calls[i]) o += (size_t)snprintf(b + o, sizeof b - o, "%s%s x%u", o ? ", " : "", slot_name[i], whd_calls[i]);
    wlog("WHD CALLS: %s\nWHD EXIT: %d\n", *b ? b : "(none)", whd_exit_reason);
}

/* ------------------------------------------------------------------ loading the slave and entering it */
int whd_boot(const char *slave_path, uint32_t *regs, uint32_t *pc, uint16_t *sr) {
    static uint8_t buf[65536]; size_t n = 0;
    FILE *f = getenv("WHD_DIR") ? fopen(slave_path, "rb") : NULL;
    if (f) { n = fread(buf, 1, sizeof buf, f); fclose(f); }
    else {   /* shipped port: the slave is embedded like the game files */
        const char *b = strrchr(slave_path, '/'); size_t en = 0; const uint8_t *e = gamefile(b ? b + 1 : slave_path, &en);
        if (!e) { wlog("WHD: cannot open %s\n", slave_path); return 1; }
        n = en < sizeof buf ? en : sizeof buf; memcpy(buf, e, n);
    }
    #define BE32(o) ((uint32_t)buf[o] << 24 | (uint32_t)buf[(o)+1] << 16 | (uint32_t)buf[(o)+2] << 8 | buf[(o)+3])
    if (n < 32 || BE32(0) != 0x3F3) { wlog("WHD: %s is not a hunk executable\n", slave_path); return 1; }
    size_t o = 4; while (BE32(o)) o += 4 + 4 * BE32(o); o += 4;
    uint32_t first = BE32(o + 4), last = BE32(o + 8); o += 12 + 4 * (last - first + 1);
    if ((BE32(o) & 0x3fffffff) != 0x3E9) { wlog("WHD: first hunk is not code\n"); return 1; }
    uint32_t words = BE32(o + 4); const uint8_t *code = buf + o + 8; uint32_t clen = words * 4;
    if (memcmp(code + 4, "WHDLOADS", 8)) { wlog("WHD: no WHDLOADS id\n"); return 1; }
    whd_layout(clen);
    if (WHD_SLAVE + clen > WHD_TOP || WHD_TOWER < WHD_EXPMEM + 0x1000u) { wlog("WHD: slave too large (%u bytes)\n", clen); return 1; }

    memset(chipram, 0, CHIP_SIZE); memset(slowram, 0, SLOW_SIZE);
    chip_reset();
    whd_slave_addr = WHD_SLAVE;
    for (uint32_t i = 0; i < clen; i++) mem_wb(WHD_SLAVE + i, code[i]);
    ws_version = (uint16_t)(code[12] << 8 | code[13]); ws_flags = (uint16_t)(code[14] << 8 | code[15]);
    whd_basemem = (uint32_t)code[16] << 24 | code[17] << 16 | code[18] << 8 | code[19];
    uint16_t gl = (uint16_t)(code[24] << 8 | code[25]), cd = (uint16_t)(code[26] << 8 | code[27]);
    whd_curdir[0] = 0; if (cd) snprintf(whd_curdir, sizeof whd_curdir, "%s", (const char *)code + cd);
    if (whd_basemem > CHIP_SIZE) { wlog("WHD: BaseMem $%X larger than the %u KB chip RAM model\n", whd_basemem, CHIP_SIZE / 1024); return 1; }
    whd_kick_base = whd_kick_size = 0;   /* G2E10 kick: the kickemu Kickstart is loaded below, once ExpMem exists */

    /* BaseMem: 0 at $0, a poisoned ExecBase at $4, $CCCCCCCC elsewhere (zero with WHDLF_ClearMem) */
    uint32_t fill = (ws_flags & 0x1000) ? 0 : 0xCCCCCCCCu;
    for (uint32_t a = 8; a < whd_basemem; a += 4) mem_wl(a, fill);
    mem_wl(0, 0); mem_wl(4, 0xF0000001u);
    /* exception and interrupt vectors point at stubs until the game installs its own: interrupts are
     * acknowledged and returned from; other exceptions are reported */
    for (uint32_t v = 8; v < 0x100; v += 4) mem_wl(v, (v >= 0x64 && v <= 0x7C) ? WHD_STUBS : WHD_STUBS + 16);
    static const uint16_t intstub[] = {0x33FC, 0x7FFF, 0x00DF, 0xF09C, 0x4E73};   /* move.w #$7fff,$dff09c; rte */
    for (int i = 0; i < 5; i++) mem_ww(WHD_STUBS + 2 * i, intstub[i]);
    mem_ww(WHD_STUBS + 16, 0x60FE); mem_ww(WHD_STUBS + 32, 0x60FE);   /* bra.s * : stay put once stopped */
    /* the tower: one 4-byte slot per function (rts as filler; the C function runs before it) */
    for (int i = 0; i < 36; i++) { mem_ww(WHD_TOWER + 4 * i, 0x4E75); mem_ww(WHD_TOWER + 4 * i + 2, 0x4E71); }
    for (int i = 0; i < 36; i++) whd_hle[i] = (hle_t){WHD_TOWER + 4u * i, dispatch, 200, slot_name[i]};
    whd_hle[36] = (hle_t){WHD_STUBS + 16, r_exception, 0, "exception"};
    whd_hle[37] = (hle_t){WHD_STUBS + 32, r_loader_returned, 0, "loader returned"};
    /* ExpMem: WHDLoad writes the address back into the slave; negative = optional */
    whd_expmem = whd_expsize = 0;
    if (ws_version >= 8) {
        int32_t want = (int32_t)((uint32_t)code[32] << 24 | code[33] << 16 | code[34] << 8 | code[35]);
        uint32_t need = (uint32_t)(want < 0 ? -want : want);
        if (want && WHD_EXPMEM + need <= WHD_TOWER) {
            whd_expmem = WHD_EXPMEM; whd_expsize = need;
            uint32_t ef = (ws_flags & 0x1000) ? 0 : 0xDDDDDDDDu;
            for (uint32_t a = 0; a < need; a += 4) mem_wl(WHD_EXPMEM + a, ef);
        } else if (want > 0) { wlog("WHD: ExpMem of $%X does not fit the slow RAM model\n", need); return 1; }
        mem_wl(WHD_SLAVE + 0x20, whd_expmem);
    }
    /* G2E10 kick: a kickemu slave (ws_Version >= 16, ws_kickname set): WHDLoad loads the Kickstart into ExpMem before the slave runs
     * (live kit whd.c, Epoch 12). ws_kickcrc $FFFF: ws_kickname points at (UWORD crc16, UWORD name offset) pairs ended by 0; the first
     * image found is used and its CRC16 is written back to ws_kickcrc (the kickemu glue picks that ROM's patch list by it). */
    if (ws_version >= 16 && (code[42] | code[43])) {
        uint16_t kn = (uint16_t)(code[42] << 8 | code[43]), kc = (uint16_t)(code[48] << 8 | code[49]);
        uint32_t ks = (uint32_t)code[44] << 24 | code[45] << 16 | code[46] << 8 | code[47];
        int ok = 0;
        if (kc != 0xFFFF) { ok = kn < clen && !load_kick((const char *)code + kn, ks, kc); }
        else for (uint32_t e = kn; e + 4 <= clen && !ok; e += 4) {
            uint16_t c = (uint16_t)(code[e] << 8 | code[e + 1]), no = (uint16_t)(code[e + 2] << 8 | code[e + 3]);
            if (!c) break;
            if (no < clen) ok = !load_kick((const char *)code + no, ks, c);
            if (ok) { mem_ww(WHD_SLAVE + 48, c); wlog("WHD: ws_kickcrc set to $%04X (the image loaded from the list)\n", c); }
        }
        if (!ok) { wlog("WHD: kickemu slave: no usable Kickstart image (size $%X crc $%04X)\n", ks, kc); return 2; }
    }
    /* the loader runs in supervisor mode with A0 = the tower; it returns to the "loader returned" stub */
    memset(regs, 0, 16 * 4);
    for (int i = 0; i < 8; i++) regs[i] = 0xD0D0D0D0u + (uint32_t)i * 0x01010101u;
    regs[8] = WHD_TOWER;
    regs[15] = whd_basemem - 4;
    mem_wl(regs[15], WHD_STUBS + 32);
    *pc = WHD_SLAVE + gl; *sr = 0x2000;
    whd_active = 1;
    wlog("WHD: slave v%u flags $%04X BaseMem $%X ExpMem $%X at $%06X, loader $%06X, dir '%s'\n",
         ws_version, ws_flags, whd_basemem, whd_expsize, whd_expmem, *pc, whd_curdir);
    wlog("WHD: memory model chip %u KB slow %u KB, slave $%06X (%u bytes)\n", CHIP_SIZE / 1024, SLOW_SIZE / 1024, WHD_SLAVE, clen);
    return 0;
}
