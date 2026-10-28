/* kickrt.h: the player's own Kickstart for a kickemu game (runtime/kickrt.c, G2 Epoch 10 "kick"). */
#ifndef KICKRT_H
#define KICKRT_H
#include <stdint.h>
#include <stddef.h>
extern uint32_t whd_kick_base, whd_kick_size;   /* the Kickstart image in ExpMem (never written out: harness.c zeroes it in every dump) */
extern const char *kick_dir;                    /* the player's Kickstarts folder (main.c, SDL builds) */
extern char kick_missing[640];                  /* why no Kickstart could be loaded, for the player */
extern char kick_loaded_from[1024];
extern char kick_name[128];                     /* the image the slave asked for, and its CRC16 (for the snapshot's game.json "kick") */
extern uint16_t kick_crc;
int kick_load(uint32_t dst, const char *name, uint32_t size, uint16_t crc, void (*wb)(uint32_t, uint32_t), char *log, size_t ln);
int kick_has(const char *hay, const char *needle);   /* case-insensitive contains (strcasestr is glibc only) */
#endif
