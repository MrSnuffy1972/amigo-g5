/* 68000 register state and flag helpers used by the translated game code (src/game/*.c). */
#ifndef CPU_H
#define CPU_H
#include <stdint.h>
#include "amiga.h"

typedef struct {
    uint32_t dr[8], ar[8];
    uint32_t PC;
    uint32_t OSP;          /* the inactive stack pointer (USP when S=1, SSP when S=0) */
    int S, IM;             /* supervisor bit, interrupt mask */
    uint32_t X, N, Z, V, C;
    int stopped;
} cpu_t;
extern cpu_t cpu;
extern uint64_t CYC, EVT;

#define D cpu.dr
#define A cpu.ar
#define XF cpu.X
#define NF cpu.N
#define ZF cpu.Z
#define VF cpu.V
#define CF cpu.C

#define RB(a) mem_rb(a)
#define RW(a) mem_rw(a)
#define RL(a) mem_rl(a)
#define WB(a,v) mem_wb((a),(v))
#define WW(a,v) mem_ww((a),(v))
#define WL(a,v) mem_wl((a),(v))

#define SX8(v)  ((uint32_t)(int32_t)(int8_t)(v))
#define SX16(v) ((uint32_t)(int32_t)(int16_t)(v))

static inline uint32_t get_sr(void) {
    return (uint32_t)((cpu.S << 13) | (cpu.IM << 8) | (cpu.X << 4) | (cpu.N << 3) | (cpu.Z << 2) | (cpu.V << 1) | cpu.C);
}
static inline void set_ccr(uint32_t v) { cpu.X = (v >> 4) & 1; cpu.N = (v >> 3) & 1; cpu.Z = (v >> 2) & 1; cpu.V = (v >> 1) & 1; cpu.C = v & 1; }
static inline void set_sr(uint32_t v) {
    int s = (v >> 13) & 1;
    if (s != cpu.S) { uint32_t t = A[7]; A[7] = cpu.OSP; cpu.OSP = t; cpu.S = s; }
    cpu.IM = (v >> 8) & 7; set_ccr(v);
}

/* size masks */
#define M1 0xffu
#define M2 0xffffu
#define M4 0xffffffffu
#define SB1 0x80u
#define SB2 0x8000u
#define SB4 0x80000000u

#define DEF_ARITH(SZ, MASK, SIGN) \
static inline uint32_t add##SZ(uint32_t s, uint32_t d) { \
    s &= MASK; d &= MASK; uint64_t r64 = (uint64_t)s + d; uint32_t r = (uint32_t)r64 & MASK; \
    cpu.C = cpu.X = (r64 >> (SZ * 8)) & 1; cpu.V = (((s ^ r) & (d ^ r)) & SIGN) ? 1 : 0; \
    cpu.N = (r & SIGN) ? 1 : 0; cpu.Z = r == 0; return r; } \
static inline uint32_t sub##SZ(uint32_t s, uint32_t d) { \
    s &= MASK; d &= MASK; uint32_t r = (d - s) & MASK; \
    cpu.C = cpu.X = s > d; cpu.V = (((s ^ d) & (r ^ d)) & SIGN) ? 1 : 0; \
    cpu.N = (r & SIGN) ? 1 : 0; cpu.Z = r == 0; return r; } \
static inline void cmp##SZ(uint32_t s, uint32_t d) { \
    s &= MASK; d &= MASK; uint32_t r = (d - s) & MASK; \
    cpu.C = s > d; cpu.V = (((s ^ d) & (r ^ d)) & SIGN) ? 1 : 0; \
    cpu.N = (r & SIGN) ? 1 : 0; cpu.Z = r == 0; } \
static inline uint32_t addx##SZ(uint32_t s, uint32_t d) { \
    s &= MASK; d &= MASK; uint64_t r64 = (uint64_t)s + d + cpu.X; uint32_t r = (uint32_t)r64 & MASK; \
    cpu.C = cpu.X = (r64 >> (SZ * 8)) & 1; cpu.V = (((s ^ r) & (d ^ r)) & SIGN) ? 1 : 0; \
    cpu.N = (r & SIGN) ? 1 : 0; if (r) cpu.Z = 0; return r; } \
static inline uint32_t subx##SZ(uint32_t s, uint32_t d) { \
    s &= MASK; d &= MASK; uint64_t r64 = (uint64_t)d - s - cpu.X; uint32_t r = (uint32_t)r64 & MASK; \
    cpu.C = cpu.X = ((uint64_t)s + cpu.X) > d; cpu.V = (((s ^ d) & (r ^ d)) & SIGN) ? 1 : 0; \
    cpu.N = (r & SIGN) ? 1 : 0; if (r) cpu.Z = 0; return r; } \
static inline uint32_t logic##SZ(uint32_t r) { \
    r &= MASK; cpu.N = (r & SIGN) ? 1 : 0; cpu.Z = r == 0; cpu.V = 0; cpu.C = 0; return r; }
DEF_ARITH(1, M1, SB1)
DEF_ARITH(2, M2, SB2)
DEF_ARITH(4, M4, SB4)

/* shifts/rotates: count already reduced (reg count mod 64, imm 1..8) */
#define DEF_SHIFT(SZ, MASK, SIGN, BITS) \
static inline uint32_t asl##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) { cpu.C = 0; cpu.V = 0; } else { \
      uint32_t v = 0; uint32_t r = d; int c = 0; \
      for (int i = 0; i < cnt; i++) { c = (r & SIGN) ? 1 : 0; r = (r << 1) & MASK; if (((r & SIGN) ? 1 : 0) != c) v = 1; } \
      d = r; cpu.C = cpu.X = (uint32_t)c; cpu.V = v; } \
    cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t asr##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) cpu.C = 0; else { int c = 0; for (int i = 0; i < cnt; i++) { c = d & 1; d = (d >> 1) | (d & SIGN); } cpu.C = cpu.X = (uint32_t)c; } \
    cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t lsl##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) cpu.C = 0; else { int c = 0; for (int i = 0; i < cnt; i++) { c = (d & SIGN) ? 1 : 0; d = (d << 1) & MASK; } cpu.C = cpu.X = (uint32_t)c; } \
    cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t lsr##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) cpu.C = 0; else { int c = 0; for (int i = 0; i < cnt; i++) { c = d & 1; d >>= 1; } cpu.C = cpu.X = (uint32_t)c; } \
    cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t rol##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) cpu.C = 0; else { int c = 0; for (int i = 0; i < cnt; i++) { c = (d & SIGN) ? 1 : 0; d = ((d << 1) | (uint32_t)c) & MASK; } cpu.C = (uint32_t)c; } \
    cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t ror##SZ(uint32_t d, int cnt) { d &= MASK; \
    if (!cnt) cpu.C = 0; else { int c = 0; for (int i = 0; i < cnt; i++) { c = d & 1; d = (d >> 1) | (c ? SIGN : 0); } cpu.C = (uint32_t)c; } \
    cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t roxl##SZ(uint32_t d, int cnt) { d &= MASK; \
    for (int i = 0; i < cnt; i++) { uint32_t c = (d & SIGN) ? 1 : 0; d = ((d << 1) | cpu.X) & MASK; cpu.X = c; } \
    cpu.C = cpu.X; cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; } \
static inline uint32_t roxr##SZ(uint32_t d, int cnt) { d &= MASK; \
    for (int i = 0; i < cnt; i++) { uint32_t c = d & 1; d = (d >> 1) | (cpu.X ? SIGN : 0); cpu.X = c; } \
    cpu.C = cpu.X; cpu.V = 0; cpu.N = (d & SIGN) ? 1 : 0; cpu.Z = d == 0; return d; }
DEF_SHIFT(1, M1, SB1, 8)
DEF_SHIFT(2, M2, SB2, 16)
DEF_SHIFT(4, M4, SB4, 32)

static inline uint32_t abcd(uint32_t s, uint32_t d) {
    uint32_t res = (s & 15) + (d & 15) + cpu.X;
    uint32_t pre = res;
    if (res > 9) res += 6;
    res += (s & 0xf0) + (d & 0xf0);
    cpu.C = cpu.X = res > 0x99;
    if (cpu.C) res -= 0xa0;
    cpu.V = ((~pre & res) & 0x80) ? 1 : 0;   /* undefined; Musashi formula */
    cpu.N = (res & 0x80) ? 1 : 0;
    res &= 0xff; if (res) cpu.Z = 0;
    return res;
}
static inline uint32_t sbcd(uint32_t s, uint32_t d) {
    uint32_t res = (d & 15) - (s & 15) - cpu.X;
    uint32_t pre = res;
    cpu.V = 0;
    if (res > 9) res -= 6;
    res += (d & 0xf0) - (s & 0xf0);
    cpu.C = cpu.X = res > 0x99;
    if (cpu.C) res += 0xa0;
    res &= 0x1ff;
    cpu.V = ((pre & ~res) & 0x80) ? 1 : 0;
    cpu.N = (res & 0x80) ? 1 : 0;
    res &= 0xff; if (res) cpu.Z = 0;
    return res;
}

static inline int cond(int cc) {
    switch (cc) {
    case 0: return 1; case 1: return 0;
    case 2: return !cpu.C && !cpu.Z; case 3: return cpu.C || cpu.Z;
    case 4: return !cpu.C; case 5: return cpu.C;
    case 6: return !cpu.Z; case 7: return cpu.Z;
    case 8: return !cpu.V; case 9: return cpu.V;
    case 10: return !cpu.N; case 11: return cpu.N;
    case 12: return cpu.N == cpu.V; case 13: return cpu.N != cpu.V;
    case 14: return cpu.N == cpu.V && !cpu.Z; default: return cpu.Z || cpu.N != cpu.V;
    }
}

/* raise an exception: pushes PC/SR, enters supervisor, returns new PC */
uint32_t cpu_exception(int vec, uint32_t pc);

/* dispatch interface of the generated code */
typedef int (*chunk_fn)(void);
chunk_fn game_chunk(uint32_t pc);      /* generated: function containing pc, or NULL */
extern const uint32_t game_code_ranges[][2];
extern const int game_code_nranges;
#endif
