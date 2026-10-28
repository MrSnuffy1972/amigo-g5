#ifndef RUN_H
#define RUN_H
#include <stdint.h>
int run_boot(void);
void run_frame(void);
int game_is_translated(uint32_t pc);
uint32_t native_pc(void);   /* c13 (19 Oct 2026): 24-bit PC after the tick, for TICKPCLOG (cpu_native.c only) */
extern uint64_t stat_interp_instr, stat_interp_calls, stat_chunk_calls, ICNT;
extern uint32_t interp_pcs[4096]; extern int n_interp_pcs;
#endif
