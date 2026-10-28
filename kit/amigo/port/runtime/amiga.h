/* Amiga OCS chipset runtime used by the native port (and by the analysis harness).
 * Not an emulator of the whole system: no CPU here, no Kickstart, no disk. The game's 68000 code
 * is translated to C elsewhere; this file models the custom chips the game programs directly. */
#ifndef AMIGA_H
#define AMIGA_H
#include <stdint.h>

/* memory model (k18, 14 Oct 2026): 512 KB chip + 512 KB slow by default, as before. A game whose WHDLoad slave asks for
 * more (BaseMem 1-2 MB, ExpMem over 512 KB) is built with -DCHIP_SIZE / -DSLOW_SIZE from gates/G0/g0.json chip_kb /
 * slow_kb (amigo_port.sh memdefs). CHIP_SIZE must be a power of two (DMA pointers wrap at it, as Agnus does). */
#ifndef CHIP_SIZE
#define CHIP_SIZE  (512*1024)
#endif
#define SLOW_BASE  0xC00000u
#ifndef SLOW_SIZE
#define SLOW_SIZE  (512*1024)
#endif
#define CHIP_MASK  ((uint32_t)CHIP_SIZE - 1u)
#define CHIP_WMASK (CHIP_MASK & ~1u)
#define SLOW_IDX(a) (((a) - SLOW_BASE) % (uint32_t)SLOW_SIZE)

#define CYC_PER_LINE 454
#define LINES_PER_FRAME 313

/* output window (lores pixels); covers PAL overscan around the standard 320x256 window */
#define SCR_W 352
#define SCR_H 272
#define SCR_HSTART 0x70   /* lores hpos (DIW units) of x=0 */
#define SCR_VSTART 0x24   /* line of y=0 */

extern uint8_t chipram[CHIP_SIZE];
extern uint8_t slowram[SLOW_SIZE];
extern uint32_t framebuf[SCR_W*SCR_H];

/* cycle clock supplied by whoever runs the CPU */
extern uint64_t (*cpu_now)(void);

uint32_t mem_rb(uint32_t a);
uint32_t mem_rw(uint32_t a);
uint32_t mem_rl(uint32_t a);
void mem_wb(uint32_t a, uint32_t v);
void mem_ww(uint32_t a, uint32_t v);
void mem_wl(uint32_t a, uint32_t v);

/* chipset */
void chip_reset(void);
int  chip_ipl(void);               /* current interrupt priority level requested */
void chip_end_line(void);          /* called when the beam passes the end of a line (chipset.c); the native runtime calls it tick_line_step */
void tick_line_step(void);
int  chip_line(void);              /* current line 0..312 */
extern int chip_frame_done;        /* set when line 312 finished, caller clears */
extern uint64_t chip_frame_count;
extern uint16_t custom[256];       /* last written values, word index */
extern uint16_t intena, intreq, dmacon;

/* input state set by the frontend */
extern int joy_up, joy_down, joy_left, joy_right, joy_fire, joy_fire2;
void kbd_push(uint8_t rawkey, int down);
void paula_set_model(int m);        /* 0 raw, 1 A500 filters, 2 A1200 filters (runtime/paula) */

/* audio (Paula) - state only for now */
typedef struct { uint32_t lc; uint16_t len, per, vol, dat; } aud_t;
extern aud_t aud[4];
extern int16_t audio_buf[16384 * 2]; extern int audio_len;   /* 44.1 kHz stereo, drained by the frontend */

/* hooks */
extern void (*hle_log)(const char *fmt, ...);
extern uint32_t smc_writes;
extern void (*ipl_changed)(void);
void chip_poke(int reg, uint16_t v);
void chip_cia_poke(int n, int reg, uint8_t v);
extern void (*cpu_stall)(uint64_t cycles); extern int blit_nasty;
int chip_contention(void);
uint64_t hash_ram(void); uint64_t hash_frame(void);  /* write a custom register (used to set boot state) */        /* writes into code ranges (self-modifying code detector) */
#endif
