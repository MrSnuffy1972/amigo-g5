/* nativefs.c: file services for a NATIVE exe (clean sheet). The game's patch code (its WHDLoad slave, translated with it) asks for a file with
 * "jsr N(a2)" (a2 = the resload base); the translator maps those call sites to these functions through game.json "hle", so the exe has no
 * resload tower, no fake library table and no WHDLoad layer: "load file X into buffer Y" is a direct native call (embedded game files, or the
 * player's save folder for what the game saved). Behaviour copied from whd.c r_loadfile / r_getfilesize / r_savefile. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#ifdef _WIN32
#include <direct.h>
#define mkdir(p, m) _mkdir(p)
#endif
#include "amiga.h"
#include "cpu.h"
#include "boot.h"

const char *whd_save_dir;      /* set by main.c to the user's profile in SDL builds (same name as the harness-side runtime uses) */
int whd_save_reload;
static void save_path(char *p, size_t n, const char *name) {
    const char *s = whd_save_dir ? whd_save_dir : getenv("WHD_SAVE"); s = s ? s : "out/port/whdsave";
    char d[1024]; snprintf(d, sizeof d, "%s", s);
    size_t dl = strlen(d); while (dl > 1 && (d[dl - 1] == '/' || d[dl - 1] == '\\')) d[--dl] = 0;
    mkdir(d, 0777);
    snprintf(p, n, "%s/%s", d, name);
    for (char *c = p + strlen(d) + 1; *c; c++) if (*c == '/') *c = '_';
}
static uint8_t *file_load(const char *name, size_t *len) {
    const char *r = getenv("WHD_SAVE_RELOAD");
    if (whd_save_reload || (r && *r == '1')) {         /* what the game saved earlier (high scores) comes first */
        char p[1300]; save_path(p, sizeof p, name); FILE *f = fopen(p, "rb");
        if (f) { fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET); uint8_t *b = malloc(n > 0 ? (size_t)n : 1); *len = fread(b, 1, n > 0 ? (size_t)n : 0, f); fclose(f); return b; }
    }
    size_t n = 0; const uint8_t *e = gamefile(name, &n);
    if (!e) return NULL;
    uint8_t *b = malloc(n ? n : 1); memcpy(b, e, n); *len = n; return b;
}
static void cstr(uint32_t a, char *out, int max) { int i = 0; for (; i < max - 1; i++) { uint8_t c = (uint8_t)mem_rb(a + i); if (!c) break; out[i] = (char)c; } out[i] = 0; }
static void ret(void) {    /* return to the caller of the jsr; condition codes follow D0 */
    cpu.PC = mem_rl(cpu.ar[7]); cpu.ar[7] += 4;
    cpu.N = (cpu.dr[0] >> 31) & 1; cpu.Z = cpu.dr[0] == 0; cpu.V = 0; cpu.C = 0;
}
void native_getfilesize(void) {          /* A0 = name -> D0 = size or 0 */
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    cpu.dr[0] = b ? (uint32_t)len : 0; free(b); ret();
}
void native_loadfile(void) {             /* A0 = name, A1 = destination -> D0 = size, D1 = 0 */
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    if (!b) { fprintf(stderr, "FINDING: the game asked for file '%s' which is not embedded\n", n); cpu.dr[0] = 0; cpu.dr[1] = 205; ret(); return; }
    for (size_t i = 0; i < len; i++) mem_wb(cpu.ar[1] + (uint32_t)i, b[i]);
    free(b); cpu.dr[0] = (uint32_t)len; cpu.dr[1] = 0; ret();
}
void native_savefile(void) {             /* D0 = size, A0 = name, A1 = data -> D0 = -1 */
    char n[256], p[1300]; cstr(cpu.ar[0], n, sizeof n); save_path(p, sizeof p, n);
    FILE *f = fopen(p, "wb");
    if (f) { for (uint32_t i = 0; i < cpu.dr[0]; i++) fputc((int)mem_rb(cpu.ar[1] + i), f); fclose(f); }
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; ret();
}

/* G2E9 ndecrunch (Claude sub-agent, 27 Oct 2026): resload Decrunch ($18), LoadFileDecrunch ($1C) and GetFileSizeDec ($70) for the native
 * exe. cs_snapcfg maps those slots here; before, they were unserved, so a game that unpacked a file after the snapshot ran into the empty
 * slot (205 Sensible Golf: jsr $18(a2) on RNC data, then untranslated address). Behaviour copied from runtime/whd.c r_decrunch /
 * r_loadfiledecrunch / r_getfilesizedec; the unpacking is the same temisu/ancient glue (runtime/whd_ancient.cpp, BSD 2-Clause) the
 * harness links. A build without it gets this weak fallback: packed data is reported as a FINDING and not unpacked. */
__attribute__((weak)) long whd_decrunch(const uint8_t *src, size_t srclen, uint8_t **out) { (void)src; (void)srclen; (void)out; return -1; }
static const char *nd_packed_kind(const uint8_t *b, size_t n) {
    if (n < 4) return NULL;
    if (!memcmp(b, "RNC\001", 4) || !memcmp(b, "RNC\002", 4)) return "RNC";
    if (!memcmp(b, "IMP!", 4) || !memcmp(b, "ATN!", 4)) return "Imploder";
    if (!memcmp(b, "TPWM", 4)) return "TPWM";
    if (!memcmp(b, "CrM!", 4) || !memcmp(b, "CrM2", 4) || !memcmp(b, "Crm!", 4) || !memcmp(b, "Crm2", 4)) return "CrunchMania";
    return NULL;
}
void native_loadfiledecrunch(void) {     /* A0 = name, A1 = destination -> D0 = size (unpacked), D1 = 0 */
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    if (!b) { fprintf(stderr, "FINDING: the game asked for file '%s' which is not embedded\n", n); cpu.dr[0] = 0; cpu.dr[1] = 205; ret(); return; }
    const char *k = nd_packed_kind(b, len); uint8_t *u = NULL; long r = k ? whd_decrunch(b, len, &u) : -1;
    if (k && r < 0) fprintf(stderr, "FINDING: resload LoadFileDecrunch '%s': %s data not unpacked (no decruncher)\n", n, k);
    if (r >= 0 && u) { for (long i = 0; i < r; i++) mem_wb(cpu.ar[1] + (uint32_t)i, u[i]); free(u); len = (size_t)r; }
    else for (size_t i = 0; i < len; i++) mem_wb(cpu.ar[1] + (uint32_t)i, b[i]);
    free(b); cpu.dr[0] = (uint32_t)len; cpu.dr[1] = 0; ret();
}
void native_decrunch(void) {             /* A0 = source, A1 = destination -> D0 = unpacked length, 0 if not packed */
    uint32_t src = cpu.ar[0], dst = cpu.ar[1]; uint8_t hdr[12];
    for (int i = 0; i < 12; i++) hdr[i] = (uint8_t)mem_rb(src + i);
    const char *k = nd_packed_kind(hdr, 12);
    if (!k) { cpu.dr[0] = 0; ret(); return; }
    size_t n = 0x80000; uint8_t *b = malloc(n);     /* packed length is format-specific: read generously, the decruncher stops (as whd.c) */
    for (size_t i = 0; i < n; i++) b[i] = (uint8_t)mem_rb(src + (uint32_t)i);
    uint8_t *u = NULL; long r = whd_decrunch(b, n, &u); free(b);
    if (r < 0 || !u) { fprintf(stderr, "FINDING: resload Decrunch %s $%06X -> $%06X failed (no decruncher or bad data)\n", k, src, dst); cpu.dr[0] = 0; ret(); return; }
    for (long i = 0; i < r; i++) mem_wb(dst + (uint32_t)i, u[i]);
    free(u); cpu.dr[0] = (uint32_t)r; ret();
}
void native_getfilesizedec(void) {       /* A0 = name -> D0 = unpacked size (RNC header), else the file size, 0 if absent */
    char n[256]; cstr(cpu.ar[0], n, sizeof n); size_t len = 0; uint8_t *b = file_load(n, &len);
    if (b && len >= 8 && nd_packed_kind(b, len) && !memcmp(b, "RNC", 3)) len = (size_t)b[4] << 24 | b[5] << 16 | b[6] << 8 | b[7];
    cpu.dr[0] = b ? (uint32_t)len : 0; free(b); ret();
}
