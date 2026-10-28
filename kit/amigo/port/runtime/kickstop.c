/* kickstop.c (G2E10 kickinterp): the player's Kickstart executed STOP in Musashi; the native scheduler owns interrupts, so the stop
 * moves to cpu.stopped and Musashi's own flag is cleared (else every later m68k_execute returns at once: G1 STOPFIX, 26 Oct). */
#include "m68kcpu.h"
int kick_take_stop(void) { if (!(CPU_STOPPED & STOP_LEVEL_STOP)) return 0; CPU_STOPPED &= ~STOP_LEVEL_STOP; return 1; }
