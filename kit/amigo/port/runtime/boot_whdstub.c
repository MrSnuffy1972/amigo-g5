/* boot_whdstub.c: the generated-boot symbols for a WHDLoad run that has no game.json yet (amigo_port.sh whdrun).
 * The slave and its files are loaded by runtime/whd.c, so every table here is empty. */
#include <stdint.h>
#include "boot.h"
typedef struct { uint32_t addr; const uint8_t *data; uint32_t len; } bootseg_t;
const bootseg_t boot_segs[] = {{0, 0, 0}}; const int boot_nsegs = 0;
const uint16_t boot_pokes[][2] = {{0, 0}}; const int boot_npokes = 0;
const uint32_t boot_regs[16] = {0}; const uint32_t boot_pc = 0; const uint16_t boot_sr = 0x2700;
const char *game_title = "WHDLoad slave";
const char *boot_whd_slave = 0;
const uint32_t boot_kick_base = 0, boot_kick_size = 0; const uint16_t boot_kick_crc = 0; const char *const boot_kick_name = "";   /* G2E10 kick */   /* whdrun boots from WHD_SLAVE in the harness */
const hle_t hle_table[] = {{0, 0, 0, 0}}; const int hle_count = 0;
void boot_gen_extra(uint32_t *regs, uint32_t *pc, uint16_t *sr) { (void)regs; (void)pc; (void)sr; }
typedef struct { const char *name; const unsigned char *data; unsigned len; } gfile_t;
const gfile_t game_files[] = {{0, 0, 0}}; const int game_nfiles = 0;
