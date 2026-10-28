/* whd.h: WHDLoad slaves without WHDLoad (runtime/whd.c). */
#ifndef WHD_H
#define WHD_H
#include <stdint.h>
#include "boot.h"
extern int whd_active, whd_exit_reason;
extern uint32_t whd_slave_addr, whd_basemem, whd_expmem, whd_expsize;
extern int whd_custom[5];
int whd_boot(const char *slave_path, uint32_t *regs, uint32_t *pc, uint16_t *sr);   /* 0 ok, 1 error, 2 kickemu */
const hle_t *whd_hle_find(uint32_t pc);
uint16_t whd_crc16(uint32_t addr, uint32_t len);
void whd_report(void);
#endif
