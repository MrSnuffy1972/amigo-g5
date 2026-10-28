/* sched.h: the tick scheduler's interface for translated code (game.json sched:true, native-intent runtime). No CYC/EVT in the code:
 * a block that is a registered yield leader calls intent_yield_leader(); every other leader counts down intent_wd (a watchdog for busy loops). */
#ifndef SCHED_H
#define SCHED_H
#include <stdint.h>
extern int intent_wd;
int intent_yield_leader(uint32_t pc);   /* 1 = the main thread yields to the scheduler here (second arrival within one epoch), 0 = go on */
int intent_watchdog(uint32_t pc);
#ifdef AMIGO_TRACE      /* dev only: a ring of the last 256 block entries, printed when the run ends on a finding */
void intent_trace(uint32_t pc);
#define INTENT_TRACE(pc) intent_trace(pc)
#else
#define INTENT_TRACE(pc) ((void)0)
#endif       /* a busy loop without a yield: logs the PC (out/yield_candidates.txt) and yields as a fallback */
#endif
