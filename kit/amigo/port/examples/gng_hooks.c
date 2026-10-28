/* Ghosts 'n Goblins (Paradox release): the game-specific parts of the pilot's boot.c, as Ami-Go hooks. */
#include <stdio.h>
#include <stdlib.h>
#include "amiga.h"
#include "boot.h"
#include "cpu.h"

static void put(uint32_t a, const uint8_t *d, size_t n) { for (size_t i = 0; i < n; i++) mem_wb(a + (uint32_t)i, d[i]); }

/* chipset as the Paradox loader programmed it, the trainer byte and the debug fill */
void game_boot_extra(uint32_t *regs, uint32_t *pc, uint16_t *sr) {
    (void)regs; (void)pc; (void)sr;
    mem_wb(0, BOOT_INFINITE_LIVES ? 0xff : 0);
    { const char *e = getenv("GNG_FILL_SEED"); if (e) { uint32_t r = (uint32_t)atol(e); for (uint32_t a = 0xC18AF8; a < 0xC20000; a++) { r = r * 1103515245u + 12345u; mem_wb(a, r >> 24); } } }
    for (int i = 0; i < 16; i++) chip_poke(0x180 + i * 2, (uint16_t)mem_rw(0x7bc00 + (0x3fe - 0x62) + i * 2));
    chip_poke(0x100, 0x6804); chip_poke(0x102, 0); chip_poke(0x104, 0x24);
    chip_poke(0x108, 0x15e); chip_poke(0x10a, 0x15e); chip_poke(0x092, 0x48); chip_poke(0x094, 0xc8);
    chip_poke(0x08e, 0x2ca1); chip_poke(0x090, 0x2ea1);
    uint32_t c1 = 0x7bc00 + (0x392 - 0x62), c2 = 0x7bc00 + (0x3ca - 0x62);
    chip_poke(0x080, (uint16_t)(c1 >> 16)); chip_poke(0x082, (uint16_t)c1);
    chip_poke(0x084, (uint16_t)(c2 >> 16)); chip_poke(0x086, (uint16_t)c2);
    chip_poke(0x088, 0);
    chip_poke(0x09a, 0x7fff); chip_poke(0x09a, 0xc010);
    chip_poke(0x096, 0x7fff); chip_poke(0x096, 0x8380);
}

/* $C0C0D4: a1 -> {dest.l, size.l, name.w}. Loads "NN.paradox" to dest, sets INTENA master on,
 * returns a5 = dest + size (then rts). */
void hle_gng_load(void) {
    uint32_t desc = A[1];
    uint32_t dest = mem_rl(desc), size = mem_rl(desc + 4); uint16_t nm = (uint16_t)mem_rw(desc + 8);
    char name[16]; snprintf(name, sizeof name, "%c%c.paradox", nm >> 8, nm & 0xff);
    size_t n; const uint8_t *f = gamefile(name, &n);
    hle_log("HLE load %s -> $%06X size $%X (file %u) frame %llu\n", name, dest, size, (unsigned)n, (unsigned long long)chip_frame_count);
    if (f) put(dest, f, n < size ? n : size);
    { static int nlev; if (dest == 0x50768 && ++nlev > 1) hle_last_load_frame = chip_frame_count; }
    chip_poke(0x09a, 0xc000);
    A[5] = dest + size;
    cpu.PC = mem_rl(A[7]); A[7] += 4;
}
