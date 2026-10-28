/* nowhd.c: the native exe has no WHDLoad layer (no slave, no resload tower, no fake libraries). Inert definitions of what cpu.c and boot.c refer to. */
#include <stdint.h>
#include "boot.h"
#include "whd.h"
int whd_active = 0; uint32_t whd_basemem = 0;
const hle_t *whd_hle_find(uint32_t pc) { (void)pc; return 0; }
int whd_boot(const char *slave, uint32_t *regs, uint32_t *pc, uint16_t *sr) { (void)slave; (void)regs; (void)pc; (void)sr; return -1; }
int whd_exit_reason; uint32_t whd_slave_addr, whd_expmem, whd_expsize; int whd_custom[5];
uint16_t whd_crc16(uint32_t addr, uint32_t len) { (void)addr; (void)len; return 0; }
void whd_report(void) {}
