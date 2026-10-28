/* kickrt.c: the PLAYER'S OWN Kickstart for a WHDLoad kickemu game (G2 Epoch 10 "kick", Claude sub-agent, 27 Oct 2026; ported from
 * the live G0/G1 kit's runtime/whd.c KICKEMU, tag g1kickemu; Karl's decision 26 Oct, extended to the clean-sheet kit 27 Oct).
 * A port never carries, embeds or writes out any Kickstart byte. The reference harness loads the image when the slave asks for it
 * (whd.c), and the native exe loads the same image into the same place at boot (boot.c, game.json "kick" from the snapshot), so the
 * snapshot RAM, which has the Kickstart region zeroed, gets the player's ROM back at run time. Search order (first match by size and
 * CRC16; a 512 KB file holding the same 256 KB image twice also matches):
 *   1. AMIGO_KICKSTART: a ROM file, or a folder (searched two levels deep)
 *   2. WHD_KICKDIR (build-time runs)
 *   3. kick_dir: set by main.c in SDL builds to the folder "Kickstarts" next to the executable (the player's place)
 *   4. "Kickstarts" in the current folder
 *   5. /srv/fast/amigo/tools/kickroms: Karl's own ROMs, on Ryzen only (the build-time default; kickemu G2 runs are pinned there)
 * The relocation table kickNNNNN.MODEL.RTB (WHDLoad package) is read beside the ROM first, then from WHD_RTBDIR. When nothing is found,
 * kick_missing says which files the player must supply. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <dirent.h>
#include <sys/stat.h>
#include "kickrt.h"

uint32_t whd_kick_base, whd_kick_size;
const char *kick_dir;
char kick_missing[640];
char kick_loaded_from[1024];
char kick_name[128];
uint16_t kick_crc;

static uint16_t crc16_buf(const uint8_t *b, size_t n) {
    uint16_t c = 0;
    for (size_t i = 0; i < n; i++) { c ^= b[i]; for (int k = 0; k < 8; k++) c = (c & 1) ? (uint16_t)((c >> 1) ^ 0xA001) : (uint16_t)(c >> 1); }
    return c;
}
static uint8_t *slurp(const char *p, size_t *n) {
    FILE *f = fopen(p, "rb"); if (!f) return NULL;
    fseek(f, 0, SEEK_END); long l = ftell(f); fseek(f, 0, SEEK_SET);
    if (l <= 0) { fclose(f); return NULL; }
    uint8_t *b = malloc((size_t)l); *n = fread(b, 1, (size_t)l, f); fclose(f); return b;
}
static int rom_ok(uint8_t *b, size_t *n, uint32_t size, uint16_t crc) {
    if (b && *n == 2 * (size_t)size && !memcmp(b, b + size, size)) *n = size;   /* an image stored twice (512 KB file) */
    return b && *n == size && crc16_buf(b, *n) == crc;
}
static uint8_t *find_rom(const char *d, int depth, uint32_t size, uint16_t crc, char *out, size_t on) {
    DIR *dp = opendir(d); if (!dp) return NULL;
    struct dirent *e; uint8_t *hit = NULL;
    while (!hit && (e = readdir(dp))) {
        if (e->d_name[0] == '.') continue;
        char p[1024]; snprintf(p, sizeof p, "%s/%s", d, e->d_name);
        struct stat st; if (stat(p, &st)) continue;
        if (S_ISDIR(st.st_mode)) { if (depth > 0) hit = find_rom(p, depth - 1, size, crc, out, on); continue; }
        if ((uint32_t)st.st_size != size && (uint32_t)st.st_size != 2 * size) continue;
        size_t n = 0; uint8_t *b = slurp(p, &n);
        if (rom_ok(b, &n, size, crc)) { hit = b; snprintf(out, on, "%s", p); } else free(b);
    }
    closedir(dp); return hit;
}
static FILE *open_ci(const char *d, const char *want, char *out, size_t on) {   /* case-insensitive file name in folder d */
    DIR *dp = opendir(d); if (!dp) return NULL;
    struct dirent *e; FILE *f = NULL;
    while (!f && (e = readdir(dp))) if (!strcasecmp(e->d_name, want)) { snprintf(out, on, "%s/%s", d, e->d_name); f = fopen(out, "rb"); }
    closedir(dp); return f;
}
/* the WHDLoad .RTB relocation table: ULONG id, ULONG first offset, byte deltas (0 = UWORD delta at the next even position, 0 UWORD
 * ends), then $FFFFFFFF and absolute ULONG offsets of BCPL pointers ended by 0. Each longword gets (new base - ROM base) added, each
 * BCPL pointer (new base - ROM base) / 4. Returns the longword count, -1 if no table. */
static int apply_rtb(uint8_t *rom, uint32_t size, const char *kname, const char *romdir, uint32_t dst, char *log, size_t ln) {
    char want[300], path[1024] = ""; snprintf(want, sizeof want, "%s.RTB", kname);
    const char *rd = getenv("WHD_RTBDIR"); rd = rd ? rd : "/srv/fast/amigo/tools/whdload/rtb";
    FILE *f = open_ci(romdir, want, path, sizeof path);
    if (!f) f = open_ci(rd, want, path, sizeof path);
    if (!f) return -1;
    fclose(f);
    size_t n = 0; uint8_t *t = slurp(path, &n);
    if (!t || n < 12) { free(t); return -1; }
    #define TL(i) ((uint32_t)t[i] << 24 | (uint32_t)t[(i)+1] << 16 | (uint32_t)t[(i)+2] << 8 | t[(i)+3])
    uint32_t rombase = 0x1000000u - size, delta = dst - rombase, off = TL(4), nl = 0, nb = 0;
    size_t i = 8;
    for (;;) {
        if (off + 4 <= size) {
            uint32_t v = (uint32_t)rom[off] << 24 | rom[off + 1] << 16 | rom[off + 2] << 8 | rom[off + 3];
            v += delta; rom[off] = (uint8_t)(v >> 24); rom[off + 1] = (uint8_t)(v >> 16); rom[off + 2] = (uint8_t)(v >> 8); rom[off + 3] = (uint8_t)v; nl++;
        }
        if (i >= n) break;
        uint32_t d = t[i++];
        if (!d) { if (i & 1) i++; if (i + 2 > n) break; d = (uint32_t)t[i] << 8 | t[i + 1]; i += 2; if (!d) break; }
        off += d;
    }
    if (i & 1) i++;
    if (i + 4 <= n && TL(i) == 0xFFFFFFFFu) {
        for (i += 4; i + 4 <= n; i += 4) {
            uint32_t o = TL(i); if (!o) break;
            if (o + 4 > size) continue;
            uint32_t v = (uint32_t)rom[o] << 24 | rom[o + 1] << 16 | rom[o + 2] << 8 | rom[o + 3];
            v += (uint32_t)((int32_t)delta / 4); rom[o] = (uint8_t)(v >> 24); rom[o + 1] = (uint8_t)(v >> 16); rom[o + 2] = (uint8_t)(v >> 8); rom[o + 3] = (uint8_t)v; nb++;
        }
    }
    snprintf(log, ln, "%s: %u longword and %u BCPL relocations ($%06X -> $%06X)", path, nl, nb, rombase, dst);
    free(t); return (int)nl;
    #undef TL
}
static uint8_t *lookup(uint32_t size, uint16_t crc, char *path, size_t pn, char *tried, size_t tn) {
    const char *cand[5]; int nc = 0;
    const char *e = getenv("AMIGO_KICKSTART"); if (e && *e) cand[nc++] = e;
    e = getenv("WHD_KICKDIR"); if (e && *e) cand[nc++] = e;
    if (kick_dir && *kick_dir) cand[nc++] = kick_dir;
    cand[nc++] = "Kickstarts";
    cand[nc++] = "/srv/fast/amigo/tools/kickroms";
    tried[0] = 0;
    for (int i = 0; i < nc; i++) {
        size_t tl = strlen(tried); snprintf(tried + tl, tn - tl, "%s%s", tl ? ", " : "", cand[i]);
        struct stat st; if (stat(cand[i], &st)) continue;
        if (S_ISDIR(st.st_mode)) { uint8_t *r = find_rom(cand[i], 2, size, crc, path, pn); if (r) return r; continue; }
        size_t n = 0; uint8_t *b = slurp(cand[i], &n);
        if (rom_ok(b, &n, size, crc)) { snprintf(path, pn, "%s", cand[i]); return b; }
        free(b);
    }
    return NULL;
}
int kick_has(const char *h, const char *n) { size_t l = strlen(n); for (; *h; h++) if (!strncasecmp(h, n, l)) return 1; return 0; }
/* kick_load: the Kickstart `name` (size, CRC16) relocated to dst and written there through wb (mem_wb); 0 = done, else kick_missing
 * says why. log (may be NULL) gets one line per step for the caller's log. */
int kick_load(uint32_t dst, const char *name, uint32_t size, uint16_t crc, void (*wb)(uint32_t, uint32_t), char *log, size_t ln) {
    char path[1024] = "", tried[1400], rl[1400] = "";
    if (log && ln) log[0] = 0;
    kick_missing[0] = 0;
    const char *pre = strncasecmp(name, "kick", 4) ? "kick" : "";
    uint8_t *rom = lookup(size, crc, path, sizeof path, tried, sizeof tried);
    if (!rom) {
        snprintf(kick_missing, sizeof kick_missing,
                 "This game needs your own Amiga Kickstart ROM: %s%s (%u bytes, CRC16 $%04X), plus its relocation table %s%s.RTB from the "
                 "WHDLoad package. Put both in a folder called Kickstarts next to the game, or set AMIGO_KICKSTART to the ROM file. Looked in: %s",
                 pre, name, size, crc, pre, name, tried);
        return 1;
    }
    char kname[300]; snprintf(kname, sizeof kname, "%s%s", pre, name);
    char romdir[1024]; snprintf(romdir, sizeof romdir, "%s", path); { char *s = strrchr(romdir, '/'); if (s) *s = 0; else snprintf(romdir, sizeof romdir, "."); }
    if (apply_rtb(rom, size, kname, romdir, dst, rl, sizeof rl) < 0) {
        const char *r = getenv("AMIGO_KICK_NORTB_OK");
        if (!(r && *r == '1')) {
            snprintf(kick_missing, sizeof kick_missing, "Kickstart %s found (%s) but not its relocation table %s.RTB: put it next to the ROM file (it comes with WHDLoad).", name, path, kname);
            free(rom); return 1;
        }
    }
    for (uint32_t k = 0; k < size; k++) wb(dst + k, rom[k]);
    free(rom);
    whd_kick_base = dst; whd_kick_size = size; kick_crc = crc; snprintf(kick_name, sizeof kick_name, "%s", name);
    snprintf(kick_loaded_from, sizeof kick_loaded_from, "%s", path);
    if (log && ln) snprintf(log, ln, "%s%sLoadKick %s size $%X crc $%04X -> $%06X (from %s)", rl, *rl ? "\n" : "", name, size, crc, dst, path);
    return 0;
}
