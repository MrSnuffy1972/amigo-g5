#!/usr/bin/env python3
"""Static 68000 -> C translator (Ami-Go port kit, generic version of the pilot's translator).

The pilot (Ghosts 'n Goblins, 12 Oct 2026) proved this decoder: 13,076 translated instructions x 128
random trials against Musashi with 0 mismatches, and frame 400 pixel-identical to the original. That
version was hard-wired to one game; this one reads everything game-specific from game.json (see
portcfg.py), so any game's image can be translated without a model rewriting it.

Discovers code by recursive descent from seeds (entry point, Ghidra's function list from the library
pre-analysis, executed-PC logs from the harness, jump-table heuristics), decodes every 68000
instruction it reaches and emits C that operates on the cpu_t state in runtime/cpu.h. Each instruction
becomes a labelled C block; code is grouped into 4 KB chunk functions with a switch-based entry so any
instruction address can be (re)entered. Branches inside a chunk are gotos; everything else returns to
the dispatcher. Addresses the game replaces by HLE (game.json "hle") call the named C function.

usage: m68k2c.py OUTDIR [--config game.json] [--seeds FILE ...] [--single FILE] [--no-speculative]
       [--coverage FILE]   (G1 coverage.json: reachable / translated / interpreted, with details)
       [--emit rust]       (G5 M2: write OUTDIR/translated.rs from the same decoder instead of the C chunks; m68k2rs.py)
"""
import re
import os, sys, struct, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import portcfg

CHUNK = 0x1000
# 68000 base cycle counts per opcode, exported from Musashi's table (cycles68000.bin)
CYCLES = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cycles68000.bin'), 'rb').read()
HLE = {}

# ---- named hardware calls (native-intent step 3): a literal $DFFxxx / $BFxxxx access in translated code becomes a named call (no address decode at run time) ----
def _custom_name(off):
    n = {0x000:'BLTDDAT',0x002:'DMACONR',0x004:'VPOSR',0x006:'VHPOSR',0x008:'DSKDATR',0x00A:'JOY0DAT',0x00C:'JOY1DAT',0x00E:'CLXDAT',0x010:'ADKCONR',0x012:'POT0DAT',0x014:'POT1DAT',
         0x016:'POTGOR',0x018:'SERDATR',0x01A:'DSKBYTR',0x01C:'INTENAR',0x01E:'INTREQR',0x020:'DSKPTH',0x022:'DSKPTL',0x024:'DSKLEN',0x026:'DSKDAT',0x028:'REFPTR',0x02A:'VPOSW',
         0x02C:'VHPOSW',0x02E:'COPCON',0x030:'SERDAT',0x032:'SERPER',0x034:'POTGO',0x036:'JOYTEST',0x040:'BLTCON0',0x042:'BLTCON1',0x044:'BLTAFWM',0x046:'BLTALWM',0x048:'BLTCPTH',
         0x04A:'BLTCPTL',0x04C:'BLTBPTH',0x04E:'BLTBPTL',0x050:'BLTAPTH',0x052:'BLTAPTL',0x054:'BLTDPTH',0x056:'BLTDPTL',0x058:'BLTSIZE',0x060:'BLTCMOD',0x062:'BLTBMOD',0x064:'BLTAMOD',
         0x066:'BLTDMOD',0x070:'BLTCDAT',0x072:'BLTBDAT',0x074:'BLTADAT',0x07E:'DSKSYNC',0x080:'COP1LCH',0x082:'COP1LCL',0x084:'COP2LCH',0x086:'COP2LCL',0x088:'COPJMP1',0x08A:'COPJMP2',
         0x08C:'COPINS',0x08E:'DIWSTRT',0x090:'DIWSTOP',0x092:'DDFSTRT',0x094:'DDFSTOP',0x096:'DMACON',0x098:'CLXCON',0x09A:'INTENA',0x09C:'INTREQ',0x09E:'ADKCON',0x100:'BPLCON0',
         0x102:'BPLCON1',0x104:'BPLCON2',0x106:'BPLCON3',0x108:'BPL1MOD',0x10A:'BPL2MOD'}.get(off)
    if n: return n
    if 0x0A0 <= off < 0x0E0:
        ch, r = (off - 0xA0) >> 4, off & 0xf
        return 'AUD%d%s' % (ch, {0:'LCH',2:'LCL',4:'LEN',6:'PER',8:'VOL',0xA:'DAT'}.get(r, 'R%X' % r))
    if 0x0E0 <= off < 0x100:
        p = ((off - 0xE0) >> 2) + 1; return 'BPL%dPT%s' % (p, 'L' if off & 2 else 'H')
    if 0x110 <= off < 0x120: return 'BPL%dDAT' % ((off - 0x110) // 2 + 1)
    if 0x120 <= off < 0x140: return 'SPR%dPT%s' % ((off - 0x120) >> 2, 'L' if off & 2 else 'H')
    if 0x140 <= off < 0x180: return 'SPR%d%s' % ((off - 0x140) >> 3, ['POS','CTL','DATA','DATB'][(off >> 1) & 3])
    if 0x180 <= off < 0x1C0: return 'COLOR%02d' % ((off - 0x180) >> 1)
    return 'REG%03X' % off
_CIA = ['PRA','PRB','DDRA','DDRB','TALO','TAHI','TBLO','TBHI','TODLO','TODMID','TODHI','R0B','SDR','ICR','CRA','CRB']
HW_USED = {}      # function name -> (kind, args) for the generated hwcalls.h
_HWRE = re.compile(r'\b(R|W)([BWL])\(0x(DFF[0-9A-Fa-f]{3}|BF[ED][0-9A-Fa-f]{3})u(,|\))')
_EAC = re.compile(r'^\s*ea = 0x(DFF[0-9A-Fa-f]{3}|BF[ED][0-9A-Fa-f]{3})u;\s*$')
_EAINC = re.compile(r'^\s*ea \+= (\d+);\s*$')
_EAINC_TAIL = re.compile(r';\s*ea \+= (\d+);\s*$')
_EAUSE = re.compile(r'\b(R|W)([BWL])\(ea(,|\))')
def rewrite_hw_ea(text):
    """Track `ea = 0xDFFxxxu;` / `ea += n;` inside one instruction (movem and friends with an absolute hardware address) and turn WL(ea, x) / RL(ea) into named calls."""
    out = []; cur = None
    for line in text.split('\n'):
        if line.startswith('L_') or line.startswith('  case') or line.startswith('int chunk'): cur = None
        m = _EAC.match(line)
        if m: cur = int(m.group(1), 16) | 0xDFF000 * 0 | (0xDFF000 if False else 0); cur = int(m.group(1), 16); out.append(line); continue
        m = _EAINC.match(line)
        if m and cur is not None: cur += int(m.group(1)); out.append(line); continue
        if cur is not None and 0xDFF000 <= cur < 0xE00000 or (cur is not None and 0xBFD000 <= cur < 0xBFF000):
            def sub(mm):
                rw, sz, tail = mm.group(1), mm.group(2), mm.group(3)
                return rw + sz + '(0x%Xu%s' % (cur, tail)
            line = _EAUSE.sub(sub, line)
            mi = _EAINC_TAIL.search(line)
            if mi: cur += int(mi.group(1))   # movem emits `WL(ea, Dn); ea += 4;` on ONE line: advance after the access
        elif cur is not None and ('ea' in line and not line.strip().startswith('ea')):
            pass
        out.append(line)
    return '\n'.join(out)
_ENTROPY_CUSTOM = (0x004, 0x006, 0x012, 0x014, 0x018, 0x01A)   # G2E10 rngseed: beam, POT, SERDATR, DSKBYTR reads carry the instruction's PC
_LBL = re.compile(r'^L_([0-9A-F]{6}):')
def rewrite_hw(text):
    text = rewrite_hw_ea(text)
    out = []; cur = [None]   # G2E10 rngseed: the current instruction's PC (its L_xxxxxx label), for the site-tagged entropy reads
    for line in text.split('\n'):
        ml = _LBL.match(line)
        if ml: cur[0] = int(ml.group(1), 16)
        out.append(_HWRE.sub(lambda m: _hw_sub(m, cur[0]), line))
    return '\n'.join(out)
def _hw_sub(m, pc):
    if True:
        rw, sz, addr, tail = m.group(1), m.group(2), m.group(3).upper(), m.group(4)
        if addr.startswith('DFF'):
            raw = int(addr[3:], 16); off = raw & 0x1fe
            nm = _custom_name(off) + ('' if sz != 'B' else ('_odd' if raw & 1 else '_even'))
            name = 'intent_%s_%s%s' % (nm, 'r' if rw == 'R' else 'w', {'B':'8','W':'16','L':'32'}[sz])
            if rw == 'R' and pc is not None and (off in _ENTROPY_CUSTOM or (sz == 'L' and off + 2 in _ENTROPY_CUSTOM)):   # G2E10 rngseed
                HW_USED[name + '_at'] = ('custom_at', rw, sz, off, raw & 1)
                return name + '_at(0x%Xu' % pc + (')' if tail == ')' else ', ')
            HW_USED[name] = ('custom', rw, sz, off, raw & 1)
        else:
            if sz != 'B': return m.group(0)
            n = 0 if addr.startswith('BFE') else 1; reg = (int(addr[3:], 16) >> 8) & 15
            name = 'intent_CIA%s_%s_%s8' % ('AB'[n], _CIA[reg], 'r' if rw == 'R' else 'w')
            if rw == 'R' and pc is not None and 4 <= reg <= 10:   # G2E10 rngseed: CIA timer and TOD reads carry the PC
                HW_USED[name + '_at'] = ('cia_at', rw, 'B', n, reg)
                return name + '_at(0x%Xu' % pc + (')' if tail == ')' else ', ')
            HW_USED[name] = ('cia', rw, 'B', n, reg)
        return name + ('()' if tail == ')' else '(')
def write_hwcalls(outdir):
    L = ['/* generated by m68k2c.py: the hardware accesses of this game as NAMED calls (native-intent step 3); nothing here decodes an address */', '#ifndef HWCALLS_H', '#define HWCALLS_H', '#include <stdint.h>',
         'uint16_t hw_r16(int reg); void hw_w16(int reg, uint16_t v); uint8_t hw_ciar(int n, int reg); void hw_ciaw(int n, int reg, uint8_t v);',
         'uint16_t hw_r16_at(int reg, uint32_t pc); uint8_t hw_ciar_at(int n, int reg, uint32_t pc);   /* G2E10 rngseed */']
    for name, t in sorted(HW_USED.items()):
        if t[0] == 'custom_at':   # G2E10 rngseed: as 'custom' reads, with the instruction's PC
            _, rw, sz, off, odd = t
            if sz == 'W': L.append('static inline uint32_t %s(uint32_t pc) { return hw_r16_at(0x%X, pc); }' % (name, off))
            elif sz == 'L': L.append('static inline uint32_t %s(uint32_t pc) { return ((uint32_t)hw_r16_at(0x%X, pc) << 16) | hw_r16_at(0x%X, pc); }' % (name, off, off + 2))
            else: L.append('static inline uint32_t %s(uint32_t pc) { return (hw_r16_at(0x%X, pc) >> %d) & 0xff; }' % (name, off, 0 if odd else 8))
            continue
        if t[0] == 'cia_at':
            _, rw, sz, n, reg = t
            L.append('static inline uint32_t %s(uint32_t pc) { return hw_ciar_at(%d, %d, pc); }' % (name, n, reg)); continue
        if t[0] == 'custom':
            _, rw, sz, off, odd = t
            if rw == 'R':
                if sz == 'W': L.append('static inline uint32_t %s(void) { return hw_r16(0x%X); }' % (name, off))
                elif sz == 'L': L.append('static inline uint32_t %s(void) { return ((uint32_t)hw_r16(0x%X) << 16) | hw_r16(0x%X); }' % (name, off, off + 2))
                else: L.append('static inline uint32_t %s(void) { return (hw_r16(0x%X) >> %d) & 0xff; }' % (name, off, 0 if odd else 8))
            else:
                if sz == 'W': L.append('static inline void %s(uint32_t v) { hw_w16(0x%X, (uint16_t)v); }' % (name, off))
                elif sz == 'L': L.append('static inline void %s(uint32_t v) { hw_w16(0x%X, (uint16_t)(v >> 16)); hw_w16(0x%X, (uint16_t)v); }' % (name, off, off + 2))
                else: L.append('static inline void %s(uint32_t v) { hw_w16(0x%X, (uint16_t)(((v & 0xff) << 8) | (v & 0xff))); }' % (name, off))
        else:
            _, rw, sz, n, reg = t
            if rw == 'R': L.append('static inline uint32_t %s(void) { return hw_ciar(%d, %d); }' % (name, n, reg))
            else: L.append('static inline void %s(uint32_t v) { hw_ciaw(%d, %d, (uint8_t)v); }' % (name, n, reg))
    L.append('#endif')
    open(os.path.join(outdir, 'hwcalls.h'), 'w').write('\n'.join(L) + '\n')

def mem_sizes():
    """chip and slow RAM sizes in bytes as the runtime is built (env WHD_CHIP_KB/WHD_SLOW_KB, else gates/G0/g0.json chip_kb/slow_kb, else 512/512): the k18 memory model (15 Oct: Mem.rb raised IndexError at $80000 chip and $C80000 slow for the 1 MB and AGA games)."""
    ck = sk = 512
    try:
        g = json.load(open(portcfg.path('gates/G0/g0.json')))
        ck = int(g.get('chip_kb') or 512); sk = int(g.get('slow_kb') or 512)
    except Exception:
        pass
    ck = int(os.environ.get('WHD_CHIP_KB') or ck); sk = int(os.environ.get('WHD_SLOW_KB') or sk)
    return ck * 1024, sk * 1024

class Mem:
    """The translator's view of memory: chip RAM ($0-$7FFFF) and slow RAM ($C00000-$C7FFFF) as the
    runtime models them (zero where nothing is loaded), plus any other loaded block."""
    def __init__(self, cfg):
        self.chipsz, self.slowsz = mem_sizes(); self.chip = bytearray(self.chipsz); self.slow = bytearray(self.slowsz); self.other = {}
        self.snap = None; self.ranges = list(cfg['code']); self.observed = {}; self.fx = {}; self.fxo = {}
        self.kick = None   # G2E10 kickxlate: game.json "kick" (base, size hex) = the player's Kickstart, run at run time, never translated
        try:
            _k = cfg.get('kick')
            if not _k:
                import json as _j; _k = (_j.load(open(portcfg.path('game.json'))) or {}).get('kick')
            if _k and int(str(_k.get('size') or '0'), 16):
                _b = int(str(_k['base']), 16); self.kick = (_b, _b + int(str(_k['size']), 16))
        except Exception:
            self.kick = None
        for s in cfg['segments'] + cfg['translate_only']:
            self.put(s['addr'], portcfg.seg_bytes(s))
        for x in cfg['copies']:
            self.put(x['dst'], bytes(self.rb(x['src'] + i) for i in range(x['len'])))
        self.ranges = list(cfg['code'])
        # code seen running elsewhere (portcfg.observed), read from the harness's end-of-run RAM
        self.observed, self.volatile = portcfg.observed(cfg)
        self.fx = portcfg.observed_bytes(cfg)      # bytes at first execution: they win over the end-of-run snapshot for run-time code
        # Cowork 17 Oct (measured on 17 of 49 WHDLoad "untranslated address" clean-sheet runs): in a snapshot workspace the code ranges are
        # all RAM, so fx never applied; code the game loads AFTER the snapshot was translated from the snapshot's fill bytes ($CCCC...).
        # For PCs that first ran after the snapshot (snapcfg/seeds.txt), the bytes they ran with win.
        self.fxo = {}
        _sd, _fb = portcfg.path('snapcfg/seeds.txt'), portcfg.path('out/port/ref/first_bytes.txt')
        if os.path.exists(_sd) and os.path.exists(_fb):
            _pcs = set(int(x, 16) for x in open(_sd).read().split())
            for _l in open(_fb):
                _t = _l.split()
                if len(_t) > 1 and int(_t[0], 16) in _pcs:
                    _a = int(_t[0], 16)
                    for _i, _b in enumerate(_t[1:]): self.fxo.setdefault(_a + _i, int(_b, 16))
        self.snap = None
        rc, rs = portcfg.path('out/port/ref/ram_chip.bin'), portcfg.path('out/port/ref/ram_slow.bin')
        if self.observed and os.path.exists(rc) and os.path.exists(rs):
            self.snap = (open(rc, 'rb').read(), open(rs, 'rb').read())
            bad = [a for a, op in self.observed.items() if self.wsnap(a) != op and a not in self.fx]
            for a in bad: del self.observed[a]
            self.volatile += len(bad)
        elif self.observed:
            self.observed = {}
    def wsnap(self, a):
        c, s = self.snap
        if a < self.chipsz: return (c[a] << 8) | c[a + 1] if a + 1 < len(c) else -1                 # bounds checked (15 Oct, Colonization: a snapshot shorter than the model)
        if 0xC00000 <= a < 0xC00000 + self.slowsz: o = a - 0xC00000; return (s[o] << 8) | s[o + 1] if o + 1 < len(s) else -1
        return -1
    def put(self, a, data):
        for i, b in enumerate(data):
            self.wb(a + i, b)
    def wb(self, a, v):
        if a < self.chipsz: self.chip[a] = v
        elif 0xC00000 <= a < 0xC00000 + self.slowsz: self.slow[a - 0xC00000] = v
        else: self.other[a] = v
    def rb(self, a):
        if a in self.fxo: return self.fxo[a]
        if a in self.fx: return self.fx[a]   # Cowork 17 Oct smc: the bytes the original ran with win inside code ranges too.
        # 3D World Tennis patches its own 'jmp $00000000' at $91E0 to 'jmp $3E40' (the old VBL vector) before it first runs;
        # the image bytes made the translation jump to 0 and the port ran zeroed memory from frame 2.
        if self.snap is not None and not self.incode(a):
            if a < self.chipsz and a < len(self.snap[0]): return self.snap[0][a]
            if 0xC00000 <= a < 0xC00000 + self.slowsz and a - 0xC00000 < len(self.snap[1]): return self.snap[1][a - 0xC00000]
        if a < self.chipsz: return self.chip[a]
        if 0xC00000 <= a < 0xC00000 + self.slowsz: return self.slow[a - 0xC00000]
        if a in self.other: return self.other[a]
        raise IndexError(hex(a))
    def incode(self, a):
        if self.kick and self.kick[0] <= a < self.kick[1]: return False   # G2E10 kickxlate: the player's Kickstart is never translated
        return any(s <= a < e for s, e in self.ranges)
    def inrange(self, a):
        if self.kick and self.kick[0] <= a < self.kick[1]: return False   # G2E10 kickxlate
        return self.incode(a) or a in self.observed
    def w(self, a): return (self.rb(a) << 8) | self.rb(a + 1)
    def l(self, a): return (self.w(a) << 16) | self.w(a + 2)

class Undecodable(Exception): pass

SZ = {1: ('1', 'M1', 'SB1'), 2: ('2', 'M2', 'SB2'), 4: ('4', 'M4', 'SB4')}

def s8(v): return v - 0x100 if v & 0x80 else v
def s16(v): return v - 0x10000 if v & 0x8000 else v
def h(v): return '0x%Xu' % (v & 0xffffffff)

class EA:
    """An effective address. pre: C statements to run once; read/write: C for value access."""
    def __init__(self, kind, size, reg=None, addr=None, imm=None, pre=None):
        self.kind, self.size, self.reg, self.addr, self.imm = kind, size, reg, addr, imm
        self.pre = pre or []
    def read(self):
        s = self.size
        if self.kind == 'D':
            return {1: '(D[%d] & 0xff)', 2: '(D[%d] & 0xffff)', 4: 'D[%d]'}[s] % self.reg
        if self.kind == 'A':
            return {2: '(A[%d] & 0xffff)', 4: 'A[%d]', 1: '(A[%d] & 0xff)'}[s] % self.reg
        if self.kind == 'I':
            return h(self.imm)
        return {1: 'RB(%s)', 2: 'RW(%s)', 4: 'RL(%s)'}[s] % self.addr
    def write(self, v):
        s = self.size
        if self.kind == 'D':
            if s == 4: return 'D[%d] = %s;' % (self.reg, v)
            if s == 2: return 'D[%d] = (D[%d] & 0xffff0000u) | ((%s) & 0xffff);' % (self.reg, self.reg, v)
            return 'D[%d] = (D[%d] & 0xffffff00u) | ((%s) & 0xff);' % (self.reg, self.reg, v)
        if self.kind == 'A':
            return 'A[%d] = %s;' % (self.reg, v)
        if self.kind == 'I': raise Undecodable('write to immediate')
        if self.kind == 'M' and self.imm is not None: raise Undecodable('write to pc-relative ea')
        return {1: 'WB(%s, %s);', 2: 'WW(%s, %s);', 4: 'WL(%s, %s);'}[s] % (self.addr, v)
    def is_mem(self): return self.kind == 'M'

class Insn:
    def __init__(self, pc):
        self.pc = pc; self.length = 2; self.code = []; self.targets = []; self.fall = True
        self.kind = 'op'; self.text = ''; self.cyc = 4; self.ends_block = False; self.dyn = False
        self.call = False

class Decoder:
    def __init__(self, mem):
        self.mem = mem
    def ext(self, ins):
        v = self.mem.w(ins.pc + ins.length); ins.length += 2; return v
    def ea(self, ins, mode, reg, size, tag='ea', calc_only=False):
        """Return EA. For memory modes, emits address computation into pre with temp named tag."""
        if mode == 0: return EA('D', size, reg=reg)
        if mode == 1: return EA('A', size, reg=reg)
        inc = size
        if reg == 7 and size == 1 and mode in (3, 4): inc = 2
        if mode == 2: return EA('M', size, addr='A[%d]' % reg)
        if mode == 3:
            return EA('M', size, addr=tag, pre=['%s = A[%d]; A[%d] += %d;' % (tag, reg, reg, inc)])
        if mode == 4:
            return EA('M', size, addr=tag, pre=['A[%d] -= %d; %s = A[%d];' % (reg, inc, tag, reg)])
        if mode == 5:
            d = s16(self.ext(ins)); ins.cyc += 4
            return EA('M', size, addr=tag, pre=['%s = A[%d] + %s;' % (tag, reg, h(d))])
        if mode == 6:
            e = self.ext(ins); ins.cyc += 6
            return EA('M', size, addr=tag, pre=['%s = A[%d] + %s + %s;' % (tag, reg, h(s8(e & 0xff)), self.idx(e))])
        if mode == 7:
            if reg == 0:
                a = s16(self.ext(ins)) & 0xffffffff; ins.cyc += 4
                return EA('M', size, addr=h(a))
            if reg == 1:
                a = self.mem.l(ins.pc + ins.length); ins.length += 4; ins.cyc += 8
                return EA('M', size, addr=h(a))
            if reg == 2:
                base = ins.pc + ins.length; d = s16(self.ext(ins)); ins.cyc += 4
                return EA('M', size, addr=h(base + d), imm=(base + d) & 0xffffffff)
            if reg == 3:
                base = ins.pc + ins.length; e = self.ext(ins); ins.cyc += 6
                return EA('M', size, addr=tag, pre=['%s = %s + %s;' % (tag, h(base + s8(e & 0xff)), self.idx(e))],
                          imm=('pcidx', base + s8(e & 0xff)))
            if reg == 4:
                if size == 1: v = self.ext(ins) & 0xff
                elif size == 2: v = self.ext(ins)
                else: v = self.mem.l(ins.pc + ins.length); ins.length += 4
                ins.cyc += 4 if size < 4 else 8
                return EA('I', size, imm=v)
        raise Undecodable('bad ea %d %d' % (mode, reg))
    def idx(self, e):
        r = (e >> 12) & 7; ad = 'A' if e & 0x8000 else 'D'
        if e & 0x800: return '%s[%d]' % (ad, r)
        return 'SX16(%s[%d])' % (ad, r)

    def decode(self, pc):
        ins = Insn(pc)
        op = self.mem.w(pc)
        ins.op = op
        try:
            self._decode(ins, op)
        except IndexError:
            raise Undecodable('out of range')
        # cycle model: Musashi's 68000 base table plus the run-time parts Musashi adds itself
        base = CYCLES[op]
        if (op & 0xf000) == 0xe000 and (op & 0xc0) != 0xc0 and not (op & 0x20):
            base += 2 * (((op >> 9) & 7) or 8)                    # immediate shift count
        if (op & 0xfb80) == 0x4880 and (op & 0x38) != 0:          # movem: per register
            base += bin(self.mem.w(pc + 2)).count('1') * (8 if op & 0x40 else 4)
        ins.cyc = base
        return ins

    # ---- helpers
    def size_std(self, op):
        s = (op >> 6) & 3
        if s == 3: raise Undecodable('size 3')
        return (1, 2, 4)[s]

    def emit_ea_pre(self, ins, *eas):
        for e in eas:
            ins.code += e.pre

    def _decode(self, ins, op):
        c = ins.code
        hi = op >> 12
        if hi == 0: return self.g0(ins, op)
        if hi in (1, 2, 3): return self.move(ins, op)
        if hi == 4: return self.g4(ins, op)
        if hi == 5: return self.g5(ins, op)
        if hi == 6: return self.bcc(ins, op)
        if hi == 7:
            if op & 0x100: raise Undecodable('moveq bad')
            v = s8(op & 0xff) & 0xffffffff
            c.append('D[%d] = logic4(%s);' % ((op >> 9) & 7, h(v))); ins.text = 'moveq'; return
        if hi in (8, 0xC): return self.g8c(ins, op)
        if hi in (9, 0xD): return self.addsub(ins, op)
        if hi == 0xB: return self.gb(ins, op)
        if hi == 0xE: return self.shift(ins, op)
        if op == 0x4afc: raise Undecodable('illegal')
        raise Undecodable('line %X' % hi)

    # ---- group 0: immediate ops, bit ops, movep
    def g0(self, ins, op):
        c = ins.code
        if op & 0x100:
            if (op & 0x38) == 0x08:  # movep
                dn = (op >> 9) & 7; an = op & 7; d = s16(self.ext(ins)); m = (op >> 6) & 3
                c.append('ea = A[%d] + %s;' % (an, h(d)))
                if m == 0: c.append('D[%d] = (D[%d] & 0xffff0000u) | (RB(ea) << 8) | RB(ea + 2);' % (dn, dn))
                elif m == 1: c.append('D[%d] = (RB(ea) << 24) | (RB(ea + 2) << 16) | (RB(ea + 4) << 8) | RB(ea + 6);' % dn)
                elif m == 2: c.append('WB(ea, D[%d] >> 8); WB(ea + 2, D[%d]);' % (dn, dn))
                else: c.append('WB(ea, D[%d] >> 24); WB(ea + 2, D[%d] >> 16); WB(ea + 4, D[%d] >> 8); WB(ea + 6, D[%d]);' % (dn, dn, dn, dn))
                ins.cyc = 16; return
            bitsrc = 'D[%d]' % ((op >> 9) & 7)
            return self.bitop(ins, op, bitsrc)
        sub = (op >> 9) & 7
        if sub == 4:  # static bit op
            b = self.ext(ins) & 0xff
            return self.bitop(ins, op, h(b))
        if op in (0x003c, 0x023c, 0x0a3c):  # to CCR
            v = self.ext(ins) & 0xff; o = {0: '|', 1: '&', 5: '^'}[sub]
            c.append('set_ccr((get_sr() & 0xff) %s %s);' % (o, h(v))); ins.cyc = 20; return
        if op in (0x007c, 0x027c, 0x0a7c):  # to SR
            v = self.ext(ins); o = {0: '|', 1: '&', 5: '^'}[sub]
            c.append('set_sr(get_sr() %s %s);' % (o, h(v))); ins.cyc = 20
            ins.sr_change = True; return
        size = self.size_std(op)
        imm = self.ext(ins) & 0xff if size == 1 else (self.ext(ins) if size == 2 else None)
        if size == 4: imm = self.mem.l(ins.pc + ins.length); ins.length += 4
        mode, reg = (op >> 3) & 7, op & 7
        dst = self.ea(ins, mode, reg, size)
        if dst.kind == 'A' or dst.kind == 'I': raise Undecodable('imm to An')
        self.emit_ea_pre(ins, dst)
        n = SZ[size][0]
        ins.cyc = 8 + (8 if dst.is_mem() else 0) + (4 if size == 4 else 0)
        if sub == 0: c.append(dst.write('logic%s(%s | %s)' % (n, dst.read(), h(imm))))
        elif sub == 1: c.append(dst.write('logic%s(%s & %s)' % (n, dst.read(), h(imm))))
        elif sub == 5: c.append(dst.write('logic%s(%s ^ %s)' % (n, dst.read(), h(imm))))
        elif sub == 2: c.append(dst.write('sub%s(%s, %s)' % (n, h(imm), dst.read())))
        elif sub == 3: c.append(dst.write('add%s(%s, %s)' % (n, h(imm), dst.read())))
        elif sub == 6: c.append('cmp%s(%s, %s);' % (n, h(imm), dst.read()))
        else: raise Undecodable('g0 sub 7')

    def bitop(self, ins, op, bitsrc):
        c = ins.code; typ = (op >> 6) & 3; mode, reg = (op >> 3) & 7, op & 7
        if mode == 0:
            c.append('t = 1u << ((%s) & 31);' % bitsrc)
            c.append('ZF = (D[%d] & t) == 0;' % reg)
            if typ == 1: c.append('D[%d] ^= t;' % reg)
            elif typ == 2: c.append('D[%d] &= ~t;' % reg)
            elif typ == 3: c.append('D[%d] |= t;' % reg)
            ins.cyc = 6 if typ == 0 else 8; return
        if mode == 1: raise Undecodable('bitop An')
        e = self.ea(ins, mode, reg, 1)
        if e.kind == 'I' and typ != 0: raise Undecodable('bitop imm')
        self.emit_ea_pre(ins, e)
        if e.kind == 'M' and e.addr != 'ea' and typ != 0:
            c.append('ea = %s;' % e.addr); e.addr = 'ea'
        c.append('t = 1u << ((%s) & 7);' % bitsrc)
        c.append('r = %s;' % e.read())
        c.append('ZF = (r & t) == 0;')
        if typ == 1: c.append(e.write('r ^ t'))
        elif typ == 2: c.append(e.write('r & ~t'))
        elif typ == 3: c.append(e.write('r | t'))
        ins.cyc = 8 if typ == 0 else 12

    # ---- move
    def move(self, ins, op):
        size = {1: 1, 3: 2, 2: 4}[op >> 12]
        src = self.ea(ins, (op >> 3) & 7, op & 7, size, tag='ea')
        dmode, dreg = (op >> 6) & 7, (op >> 9) & 7
        if size == 1 and src.kind == 'A': raise Undecodable('move.b An')
        c = ins.code
        self.emit_ea_pre(ins, src)
        c.append('s = %s;' % src.read())
        n = SZ[size][0]
        ins.cyc = 4 + (4 if src.is_mem() else 0) + (4 if size == 4 and src.is_mem() else 0)
        if dmode == 1:
            if size == 1: raise Undecodable('movea.b')
            c.append('A[%d] = %s;' % (dreg, 'SX16(s)' if size == 2 else 's')); ins.text = 'movea'
            return
        dst = self.ea(ins, dmode, dreg, size, tag='ea2')
        if dst.kind in ('I',) or (dst.kind == 'M' and isinstance(dst.imm, int)) or (dst.kind == 'M' and isinstance(dst.imm, tuple)):
            raise Undecodable('move to pc-rel/imm')
        self.emit_ea_pre(ins, dst)
        c.append('logic%s(s);' % n)
        c.append(dst.write('s'))
        if dst.is_mem(): ins.cyc += 4 if size < 4 else 8
        ins.text = 'move'

    # ---- group 4: misc
    def g4(self, ins, op):
        c = ins.code
        if op == 0x4e71: ins.text = 'nop'; return
        if op == 0x4e75:
            c.append('PC_ = RL(A[7]); A[7] += 4;'); ins.dyn = True; ins.fall = False; ins.cyc = 16; ins.text = 'rts'; return
        if op == 0x4e73:
            c.append('t = RW(A[7]); PC_ = RL(A[7] + 2); A[7] += 6; set_sr(t);'); ins.dyn = True; ins.fall = False; ins.cyc = 20; ins.text = 'rte'; return
        if op == 0x4e77:
            c.append('set_ccr(RW(A[7])); PC_ = RL(A[7] + 2); A[7] += 6;'); ins.dyn = True; ins.fall = False; ins.cyc = 20; return
        if op == 0x4e72:
            v = self.ext(ins); c.append('set_sr(%s); cpu.stopped = 1;' % h(v)); ins.sr_change = True; ins.text = 'stop'; return
        if op == 0x4e70: ins.text = 'reset'; ins.cyc = 132; return
        if op == 0x4e76:
            c.append('if (VF) { PC_ = cpu_exception(7, %s); goto dyn_exit; }' % h(ins.pc + 2)); return
        if (op & 0xfff0) == 0x4e40:
            ins.trap = True
            c.append('PC_ = cpu_exception(%d, %s);' % (32 + (op & 15), h(ins.pc + 2))); ins.dyn = True; ins.fall = False; ins.cyc = 34; return
        if (op & 0xfff8) == 0x4e50:
            r = op & 7; d = s16(self.ext(ins))
            c.append('A[7] -= 4; WL(A[7], A[%d]); A[%d] = A[7]; A[7] += %s;' % (r, r, h(d))); ins.cyc = 16; return
        if (op & 0xfff8) == 0x4e58:
            r = op & 7; c.append('A[7] = A[%d]; A[%d] = RL(A[7]); A[7] += 4;' % (r, r)); ins.cyc = 12; return
        if (op & 0xfff8) == 0x4e60:
            c.append('if (cpu.S) cpu.OSP = A[%d];' % (op & 7)); return
        if (op & 0xfff8) == 0x4e68:
            c.append('if (cpu.S) A[%d] = cpu.OSP;' % (op & 7)); return
        if (op & 0xffc0) in (0x4e80, 0x4ec0):  # jsr / jmp
            mode, reg = (op >> 3) & 7, op & 7
            if mode in (0, 1, 3, 4) or (mode == 7 and reg >= 4): raise Undecodable('jmp mode')      # 4EBC/4EFC (jsr/jmp #imm) are not instructions: data decoded as code emitted 'PC_ = None' (15 Oct, 3 games)
            e = self.ea(ins, mode, reg, 4, tag='ea')
            is_jsr = (op & 0xffc0) == 0x4e80
            self.emit_ea_pre(ins, e)
            if e.kind == 'M' and e.addr.startswith('0x') and not e.pre:
                tgt = int(e.addr[:-1], 16)
                ins.targets.append(tgt); ins.static_target = tgt
                # Cowork 17 Oct (Gemini job 17, adopted in part): an abs.l jmp/jsr whose operand changes after its first run
                # (first-execution bytes differ from the end-of-run RAM) reads its target at run time instead of baking one in
                if mode == 7 and reg == 1 and self.mem.snap is not None:
                    hi, lo = self.mem.wsnap(ins.pc + 2), self.mem.wsnap(ins.pc + 4)
                    if hi >= 0 and lo >= 0 and ((hi << 16) | lo) != tgt:
                        ins.targets.remove(tgt); del ins.static_target; e.addr = 'RL(0x%Xu)' % (ins.pc + 2); ins.dyn = True
                # G2E9 jmpzero (Claude sub-agent, 27 Oct 2026): an abs.l operand of $0-$FF is a placeholder patched at run time (336/337
                # 'jmp $0.l' at $8C4A, then the old VBL vector); read the target at run time. Unchanged operand = the same target.
                if mode == 7 and reg == 1 and not ins.dyn and tgt < 0x100:
                    ins.targets.remove(tgt); del ins.static_target; e.addr = 'RL(0x%Xu)' % (ins.pc + 2); ins.dyn = True
            elif mode == 7 and reg == 2:
                tgt = e.imm; ins.targets.append(tgt); ins.static_target = tgt
            else:
                ins.dyn = True
                if mode == 7 and reg == 3: ins.jtable = e.imm[1]
            c.append('PC_ = %s;' % e.addr)
            if is_jsr:
                c.append('A[7] -= 4; WL(A[7], %s);' % h(ins.pc + ins.length))
                ins.call = True; ins.cyc = 18
            else: ins.cyc = 10
            ins.fall = False; ins.text = 'jsr' if is_jsr else 'jmp'
            return
        if (op & 0xffc0) == 0x4840 and (op & 0x38) == 0:
            r = op & 7; c.append('D[%d] = logic4((D[%d] >> 16) | (D[%d] << 16));' % (r, r, r)); return
        if (op & 0xffc0) == 0x4840:  # pea
            e = self.ea(ins, (op >> 3) & 7, op & 7, 4)
            if e.kind != 'M' or (op & 0x38) in (0x18, 0x20): raise Undecodable('pea')
            self.emit_ea_pre(ins, e)
            c.append('t = %s; A[7] -= 4; WL(A[7], t);' % e.addr); ins.cyc = 12; return
        if (op & 0xfeb8) == 0x4880:  # ext
            r = op & 7
            if op & 0x40: c.append('D[%d] = logic4(SX16(D[%d]));' % (r, r))
            else: c.append('D[%d] = (D[%d] & 0xffff0000u) | logic2(SX8(D[%d]));' % (r, r, r))
            return
        if (op & 0xfb80) == 0x4880:  # movem
            return self.movem(ins, op)
        if (op & 0xf1c0) == 0x41c0:  # lea
            e = self.ea(ins, (op >> 3) & 7, op & 7, 4)
            if e.kind != 'M' or (op & 0x38) in (0x18, 0x20): raise Undecodable('lea')
            self.emit_ea_pre(ins, e)
            c.append('A[%d] = %s;' % ((op >> 9) & 7, e.addr)); ins.text = 'lea'
            if isinstance(e.imm, int): ins.lea_target = e.imm
            return
        if (op & 0xf1c0) == 0x4180:  # chk.w
            e = self.ea(ins, (op >> 3) & 7, op & 7, 2)
            self.emit_ea_pre(ins, e)
            dn = (op >> 9) & 7
            c.append('s = %s; t = D[%d] & 0xffff;' % (e.read(), dn))
            c.append('if ((int16_t)t < 0) { NF = 1; PC_ = cpu_exception(6, %s); goto dyn_exit; }' % h(ins.pc + ins.length))
            c.append('if ((int16_t)t > (int16_t)s) { NF = 0; PC_ = cpu_exception(6, %s); goto dyn_exit; }' % h(ins.pc + ins.length))
            ins.cyc = 10; return
        if (op & 0xffc0) == 0x40c0:  # move from sr
            e = self.ea(ins, (op >> 3) & 7, op & 7, 2); self.emit_ea_pre(ins, e)
            c.append(e.write('get_sr()')); return
        if (op & 0xffc0) == 0x44c0:  # move to ccr
            e = self.ea(ins, (op >> 3) & 7, op & 7, 2); self.emit_ea_pre(ins, e)
            c.append('set_ccr(%s);' % e.read()); ins.cyc = 12; return
        if (op & 0xffc0) == 0x46c0:  # move to sr
            e = self.ea(ins, (op >> 3) & 7, op & 7, 2); self.emit_ea_pre(ins, e)
            c.append('set_sr(%s);' % e.read()); ins.sr_change = True; ins.cyc = 12; return
        if (op & 0xffc0) == 0x4ac0:  # tas
            e = self.ea(ins, (op >> 3) & 7, op & 7, 1); self.emit_ea_pre(ins, e)
            if e.kind == 'M' and e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
            c.append('r = logic1(%s);' % e.read()); c.append(e.write('r | 0x80')); return
        if (op & 0xffc0) == 0x4800:  # nbcd
            e = self.ea(ins, (op >> 3) & 7, op & 7, 1); self.emit_ea_pre(ins, e)
            if e.kind == 'M' and e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
            c.append(e.write('sbcd(%s, 0)' % e.read())); ins.cyc = 8; return
        sub = (op >> 8) & 0xf
        if sub in (0, 2, 4, 6, 0xa) and (op & 0xc0) != 0xc0:
            size = self.size_std(op)
            e = self.ea(ins, (op >> 3) & 7, op & 7, size)
            if e.kind == 'A' or e.kind == 'I': raise Undecodable('unary An')
            self.emit_ea_pre(ins, e)
            if e.kind == 'M' and e.addr != 'ea' and sub != 0xa and sub != 2:
                c.append('ea = %s;' % e.addr); e.addr = 'ea'
            n = SZ[size][0]
            ins.cyc = 4 + (8 if e.is_mem() else 0)
            if sub == 0: c.append(e.write('subx%s(%s, 0)' % (n, e.read())))
            elif sub == 2:
                c.append('NF = 0; ZF = 1; VF = 0; CF = 0;'); c.append(e.write('0')); ins.text = 'clr'
            elif sub == 4: c.append(e.write('sub%s(%s, 0)' % (n, e.read())))
            elif sub == 6: c.append(e.write('logic%s(~%s)' % (n, e.read())))
            else: c.append('logic%s(%s);' % (n, e.read())); ins.text = 'tst'
            return
        raise Undecodable('g4 %04x' % op)

    def movem(self, ins, op):
        c = ins.code
        to_reg = op & 0x400; size = 4 if op & 0x40 else 2
        mask = self.ext(ins); mode, reg = (op >> 3) & 7, op & 7
        regs = []
        if mode == 4:
            # predecrement: mask bit0 = A7 .. bit15 = D0; store in order A7..D0 going down
            lst = [('A' if i < 8 else 'D', 7 - (i % 8)) for i in range(16) if mask & (1 << i)]
            c.append('ea = A[%d];' % reg)
            for (k, r) in lst:
                c.append('ea -= %d; %s(ea, %s[%d]);' % (size, 'WL' if size == 4 else 'WW', k, r))
            c.append('A[%d] = ea;' % reg)
            ins.cyc = 8 + len(lst) * (8 if size == 4 else 4); return
        lst = [('D' if i < 8 else 'A', i % 8) for i in range(16) if mask & (1 << i)]
        if mode == 3:
            c.append('ea = A[%d];' % reg)
        else:
            e = self.ea(ins, mode, reg, size)
            if e.kind != 'M': raise Undecodable('movem mode')
            self.emit_ea_pre(ins, e)
            c.append('ea = %s;' % e.addr)
        for (k, r) in lst:
            if to_reg:
                if size == 4: c.append('%s[%d] = RL(ea); ea += 4;' % (k, r))
                else: c.append('%s[%d] = SX16(RW(ea)); ea += 2;' % (k, r))
            else:
                c.append('%s(ea, %s[%d]); ea += %d;' % ('WL' if size == 4 else 'WW', k, r, size))
        if mode == 3: c.append('A[%d] = ea;' % reg)
        ins.cyc = 12 + len(lst) * (8 if size == 4 else 4)

    # ---- group 5: addq/subq/scc/dbcc
    def g5(self, ins, op):
        c = ins.code
        if (op & 0xc0) == 0xc0:
            cc = (op >> 8) & 15
            if (op & 0x38) == 0x08:  # dbcc
                r = op & 7; base = ins.pc + 2; d = s16(self.ext(ins)); tgt = (base + d) & 0xffffffff
                ins.targets.append(tgt); ins.kind = 'dbcc'; ins.static_target = tgt
                # cycles as Musashi's 68000 (base 12 from the table): taken 10, counter expired 14, condition true 12
                c.append('if (!cond(%d)) { t = (D[%d] - 1) & 0xffff; D[%d] = (D[%d] & 0xffff0000u) | t; if (t != 0xffff) { CYC -= 2; BRANCH } CYC += 2; }' % (cc, r, r, r))
                ins.cyc = 10; return
            e = self.ea(ins, (op >> 3) & 7, op & 7, 1)
            if e.kind in ('A', 'I'): raise Undecodable('scc')
            self.emit_ea_pre(ins, e)
            c.append(e.write('cond(%d) ? 0xff : 0' % cc)); ins.cyc = 6; return
        size = self.size_std(op)
        q = (op >> 9) & 7 or 8
        e = self.ea(ins, (op >> 3) & 7, op & 7, size if ((op >> 3) & 7) != 1 else 4)
        self.emit_ea_pre(ins, e)
        sub = op & 0x100
        if e.kind == 'A':
            c.append('A[%d] %s= %d;' % (e.reg, '-' if sub else '+', q)); ins.cyc = 8; return
        if e.kind == 'I': raise Undecodable('addq imm')
        if e.kind == 'M' and e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
        n = SZ[size][0]
        c.append(e.write('%s%s(%d, %s)' % ('sub' if sub else 'add', n, q, e.read())))
        ins.cyc = 4 + (8 if e.is_mem() else 0)

    # ---- bcc / bra / bsr
    def bcc(self, ins, op):
        cc = (op >> 8) & 15; d = op & 0xff; base = ins.pc + 2
        if d == 0: d = s16(self.ext(ins))
        elif d == 0xff: raise Undecodable('bcc.l (020)')
        else: d = s8(d)
        tgt = (base + d) & 0xffffffff
        ins.targets.append(tgt); ins.static_target = tgt
        if cc == 0:
            ins.kind = 'bra'; ins.fall = False; ins.cyc = 10
        elif cc == 1:
            ins.kind = 'bsr'; ins.fall = False; ins.call = True; ins.cyc = 18
            ins.code.append('A[7] -= 4; WL(A[7], %s);' % h(ins.pc + ins.length))
        else:
            ins.kind = 'bcc'; ins.cc = cc; ins.cyc = 10

    # ---- or/and/mul/div/abcd/sbcd/exg
    def g8c(self, ins, op):
        c = ins.code; is_and = (op >> 12) == 0xC
        dn = (op >> 9) & 7; opm = (op >> 6) & 7; mode, reg = (op >> 3) & 7, op & 7
        if opm == 3 or opm == 7:  # mul/div
            e = self.ea(ins, mode, reg, 2)
            if e.kind == 'A': raise Undecodable('muldiv An')
            self.emit_ea_pre(ins, e)
            signed = opm == 7
            c.append('s = %s;' % e.read())
            if is_and:
                # 68000 timing as Musashi: MULU +2 per set bit of the source, MULS +2 per 0/1 transition of the
                # sign-extended source (G3 drift was measured from cycle differences like this, 14 Oct 2026)
                if signed:
                    c.append('{ uint32_t y = (uint32_t)(int32_t)(int16_t)s, f = 0; for (; y; y >>= 1) if ((y & 1) != f) { CYC += 2; f ^= 1; } }')
                    c.append('D[%d] = logic4((uint32_t)((int32_t)(int16_t)s * (int32_t)(int16_t)D[%d]));' % (dn, dn))
                else:
                    c.append('{ uint32_t y = s & 0xffff; for (; y; y >>= 1) if (y & 1) CYC += 2; }')
                    c.append('D[%d] = logic4((s & 0xffff) * (D[%d] & 0xffff));' % (dn, dn))
                ins.cyc = 70
            else:
                c.append('if ((s & 0xffff) == 0) { PC_ = cpu_exception(5, %s); goto dyn_exit; }' % h(ins.pc + ins.length))
                if signed:
                    c.append('{ int32_t q = (int32_t)D[%d] / (int16_t)s, rm = (int32_t)D[%d] %% (int16_t)s;' % (dn, dn))
                    c.append('  if ((int32_t)D[%d] == (int32_t)0x80000000 && (int16_t)s == -1) { VF = 1; CF = 0; } else' % dn)
                    c.append('  if (q < -32768 || q > 32767) { VF = 1; CF = 0; } else { D[%d] = ((uint32_t)(rm & 0xffff) << 16) | (q & 0xffff); NF = (q & 0x8000) ? 1 : 0; ZF = (q & 0xffff) == 0; VF = 0; CF = 0; } }' % dn)
                    ins.cyc = 158
                else:
                    c.append('{ uint32_t q = D[%d] / (s & 0xffff), rm = D[%d] %% (s & 0xffff);' % (dn, dn))
                    c.append('  if (q > 0xffff) { VF = 1; CF = 0; } else { D[%d] = (rm << 16) | q; NF = (q & 0x8000) ? 1 : 0; ZF = q == 0; VF = 0; CF = 0; } }' % dn)
                    ins.cyc = 140
            return
        if opm in (4,) and mode in (0, 1):  # abcd/sbcd (opm 4 size byte, mode 0/1)
            fn = 'abcd' if is_and else 'sbcd'
            if mode == 0:
                c.append('D[%d] = (D[%d] & 0xffffff00u) | %s(D[%d] & 0xff, D[%d] & 0xff);' % (dn, dn, fn, reg, dn))
            else:
                ia = 2 if reg == 7 else 1; ib = 2 if dn == 7 else 1
                c.append('A[%d] -= %d; s = RB(A[%d]); A[%d] -= %d; ea = A[%d]; WB(ea, %s(s, RB(ea)));' % (reg, ia, reg, dn, ib, dn, fn))
            ins.cyc = 6 if mode == 0 else 18; return
        if is_and and opm in (5, 6) and mode in (0, 1) and (opm, mode) != (6, 0):  # exg
            ry = reg
            if opm == 5 and mode == 0: a, b = 'D[%d]' % dn, 'D[%d]' % ry
            elif opm == 5 and mode == 1: a, b = 'A[%d]' % dn, 'A[%d]' % ry
            else: a, b = 'D[%d]' % dn, 'A[%d]' % ry
            c.append('t = %s; %s = %s; %s = t;' % (a, a, b, b)); ins.cyc = 6; return
        size = (1, 2, 4)[opm & 3]
        n = SZ[size][0]; o = '&' if is_and else '|'
        if opm < 4:
            e = self.ea(ins, mode, reg, size)
            if e.kind == 'A': raise Undecodable('and An')
            self.emit_ea_pre(ins, e)
            d = EA('D', size, reg=dn)
            c.append(d.write('logic%s(%s %s %s)' % (n, d.read(), o, e.read())))
            ins.cyc = 4 + (4 if e.kind != 'D' else 0)
        else:
            e = self.ea(ins, mode, reg, size)
            if e.kind != 'M': raise Undecodable('and to ea')
            self.emit_ea_pre(ins, e)
            if e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
            c.append(e.write('logic%s(%s %s D[%d])' % (n, e.read(), o, dn)))
            ins.cyc = 12

    # ---- add/sub
    def addsub(self, ins, op):
        c = ins.code; is_sub = (op >> 12) == 9
        dn = (op >> 9) & 7; opm = (op >> 6) & 7; mode, reg = (op >> 3) & 7, op & 7
        fn = 'sub' if is_sub else 'add'
        if opm in (3, 7):  # adda/suba
            size = 2 if opm == 3 else 4
            e = self.ea(ins, mode, reg, size); self.emit_ea_pre(ins, e)
            v = 'SX16(%s)' % e.read() if size == 2 else e.read()
            c.append('A[%d] %s= %s;' % (dn, '-' if is_sub else '+', v)); ins.cyc = 8; return
        size = (1, 2, 4)[opm & 3]; n = SZ[size][0]
        if opm >= 4 and mode in (0, 1):  # addx/subx
            if mode == 0:
                d = EA('D', size, reg=dn)
                c.append(d.write('%sx%s(%s, %s)' % (fn, n, EA('D', size, reg=reg).read(), d.read())))
            else:
                ia = 2 if (reg == 7 and size == 1) else size; ib = 2 if (dn == 7 and size == 1) else size
                rd = {1: 'RB', 2: 'RW', 4: 'RL'}[size]; wr = {1: 'WB', 2: 'WW', 4: 'WL'}[size]
                c.append('A[%d] -= %d; s = %s(A[%d]); A[%d] -= %d; ea = A[%d]; %s(ea, %sx%s(s, %s(ea)));' % (reg, ia, rd, reg, dn, ib, dn, wr, fn, n, rd))
            ins.cyc = 8; return
        if opm < 4:
            e = self.ea(ins, mode, reg, size)
            if e.kind == 'A' and size == 1: raise Undecodable('add.b An')
            self.emit_ea_pre(ins, e)
            d = EA('D', size, reg=dn)
            c.append(d.write('%s%s(%s, %s)' % (fn, n, e.read(), d.read())))
            ins.cyc = 4 + (4 if e.kind not in ('D', 'A') else 0)
        else:
            e = self.ea(ins, mode, reg, size)
            if e.kind != 'M': raise Undecodable('add to ea')
            self.emit_ea_pre(ins, e)
            if e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
            c.append(e.write('%s%s(D[%d], %s)' % (fn, n, dn, e.read())))
            ins.cyc = 12

    # ---- cmp/eor
    def gb(self, ins, op):
        c = ins.code; dn = (op >> 9) & 7; opm = (op >> 6) & 7; mode, reg = (op >> 3) & 7, op & 7
        if opm in (3, 7):  # cmpa
            size = 2 if opm == 3 else 4
            e = self.ea(ins, mode, reg, size); self.emit_ea_pre(ins, e)
            v = 'SX16(%s)' % e.read() if size == 2 else e.read()
            c.append('cmp4(%s, A[%d]);' % (v, dn)); ins.cyc = 6; return
        size = (1, 2, 4)[opm & 3]; n = SZ[size][0]
        if opm < 3:
            e = self.ea(ins, mode, reg, size)
            if e.kind == 'A' and size == 1: raise Undecodable('cmp.b An')
            self.emit_ea_pre(ins, e)
            c.append('cmp%s(%s, %s);' % (n, e.read(), EA('D', size, reg=dn).read())); ins.cyc = 4; return
        if mode == 1:  # cmpm
            ia = 2 if (reg == 7 and size == 1) else size; ib = 2 if (dn == 7 and size == 1) else size
            rd = {1: 'RB', 2: 'RW', 4: 'RL'}[size]
            c.append('s = %s(A[%d]); A[%d] += %d; t = %s(A[%d]); A[%d] += %d; cmp%s(s, t);' % (rd, reg, reg, ia, rd, dn, dn, ib, n))
            ins.cyc = 12; return
        e = self.ea(ins, mode, reg, size)
        if e.kind in ('A', 'I'): raise Undecodable('eor')
        self.emit_ea_pre(ins, e)
        if e.kind == 'M' and e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
        c.append(e.write('logic%s(%s ^ D[%d])' % (n, e.read(), dn))); ins.cyc = 8

    # ---- shifts
    def shift(self, ins, op):
        c = ins.code
        names = ['as', 'ls', 'rox', 'ro']
        if (op & 0xc0) == 0xc0:  # memory, word, count 1
            typ = (op >> 9) & 3
            if op & 0x800: raise Undecodable('shift mem bad')
            d = 'l' if op & 0x100 else 'r'
            e = self.ea(ins, (op >> 3) & 7, op & 7, 2)
            if e.kind != 'M': raise Undecodable('shift mem ea')
            self.emit_ea_pre(ins, e)
            if e.addr != 'ea': c.append('ea = %s;' % e.addr); e.addr = 'ea'
            c.append(e.write('%s%s2(%s, 1)' % (names[typ], d, e.read()))); ins.cyc = 12; return
        size = self.size_std(op); n = SZ[size][0]
        typ = (op >> 3) & 3; d = 'l' if op & 0x100 else 'r'; r = op & 7; cr = (op >> 9) & 7
        if op & 0x20: cnt = '(int)(D[%d] & 63)' % cr; c.append('CYC += 2 * (D[%d] & 63);' % cr)
        else: cnt = str(cr or 8); ins.cyc = 6 + 2 * (cr or 8)
        de = EA('D', size, reg=r)
        c.append(de.write('%s%s%s(%s, %s)' % (names[typ], d, n, de.read(), cnt)))

def discover(mem, dec, seeds):
    insns = {}; bad = {}
    work = list(seeds); leaders = set(seeds)
    jtables = []
    while work:
        pc = work.pop()
        while True:
            if pc in insns or pc in bad: break
            if not mem.inrange(pc) or pc & 1: bad[pc] = 'out of code range'; break
            try:
                ins = dec.decode(pc)
            except Undecodable as e:
                bad[pc] = str(e); break
            insns[pc] = ins
            for t in ins.targets:
                leaders.add(t); work.append(t)
            if getattr(ins, 'jtable', None) is not None:
                jtables.append((pc, ins.jtable))
                p = ins.jtable
                # jump table of bra.w / bra.b / jmp abs.l directly at the base
                for _ in range(256):
                    try: w = mem.w(p)
                    except IndexError: break
                    if w == 0x6000: leaders.add(p); work.append(p); p += 4
                    elif (w & 0xff00) == 0x6000 and (w & 0xff) not in (0, 0xff): leaders.add(p); work.append(p); p += 2
                    elif w == 0x4ef9: leaders.add(p); work.append(p); p += 6
                    else: break
            if ins.call or ins.fall is False and ins.kind in ('bsr',) or ins.call:
                leaders.add(pc + ins.length)
                work.append(pc + ins.length)
                break
            if not ins.fall: break
            if getattr(ins, 'sr_change', False): leaders.add(pc + ins.length)
            if ins.kind in ('bcc', 'dbcc'): leaders.add(pc + ins.length)
            pc += ins.length
    return insns, bad, leaders, jtables

def speculative(mem, dec, insns, leaders):
    """Extra roots: every PC-relative lea/pea target and every absolute lea / move.l #imm value that
    points into a code range. A root is accepted only if its whole closure decodes cleanly."""
    cands = set()
    for pc, ins in list(insns.items()):
        t = getattr(ins, 'lea_target', None)
        if t is not None and mem.inrange(t): cands.add(t)
        for line in ins.code:
            for tok in line.replace('(', ' ').replace(')', ' ').replace(',', ' ').replace(';', ' ').split():
                if tok.startswith('0x') and tok.endswith('u'):
                    try: v = int(tok[:-1], 16)
                    except ValueError: continue
                    if mem.inrange(v) and not v & 1 and v not in insns: cands.add(v)
    accepted = 0; rejected = 0
    for c in sorted(cands):
        if c in insns: continue
        ni, bad, nl, jt = discover(mem, dec, [c])
        if bad: rejected += 1; continue
        # reject if the closure overlaps existing instructions at different alignment (Cowork 17 Oct overlap: this loop
        # was empty, so Alien Breed 2's speculative root $CFFFDC - the register mask of movem.l at $CFFFDA - was decoded as
        # mulu.w and run by C fall-through, corrupting D0 for resload_DiskLoad)
        clash = False
        for p, i in ni.items():
            if p in insns: continue
            for q in range(p - 8, p + i.length):
                j = insns.get(q)
                if j is not None and q != p and q < p + i.length and p < q + j.length: clash = True; break
            if clash: break
        if clash: rejected += 1; continue
        accepted += 1
        for p, i in ni.items():
            if p not in insns: insns[p] = i; i.speculative = True
        leaders |= nl
    return accepted, rejected, len(cands)

def emit(mem, insns, leaders, outdir):
    os.makedirs(outdir, exist_ok=True)
    chunks = collections.defaultdict(list)
    for pc in sorted(insns): chunks[pc // CHUNK].append(pc)
    names = []
    for ck, pcs in sorted(chunks.items()):
        fname = 'chunk_%06X' % (ck * CHUNK)
        names.append((ck, fname))
        L = []
        L.append('/* translated from 68000 code at $%06X-$%06X - generated by m68k2c.py, do not edit */' % (ck * CHUNK, ck * CHUNK + CHUNK - 1))
        L.append('#include "cpu.h"')
        L.append('#include "boot.h"')
        L.append('#include "run.h"')
        if SCHED: L.append('#include "sched.h"')
        for fn in sorted({v['fn'] for v in HLE.values()}): L.append('void %s(void);' % fn)
        L.append('int %s(void) {' % fname)
        L.append('  uint32_t ea, ea2, s, t, r, PC_; (void)ea; (void)ea2; (void)s; (void)t; (void)r; (void)PC_;')
        L.append('  switch (cpu.PC) {')
        for pc in pcs: L.append('  case 0x%X: goto L_%06X;' % (pc, pc))
        L.append('  default: return 0;')
        L.append('  }')
        inchunk = set(pcs)
        def jump(t):
            if t in inchunk:
                if SCHED: return 'goto L_%06X;' % t          # the target block's own leader check decides whether to yield
                return 'if (CYC >= EVT) { cpu.PC = 0x%X; return 1; } goto L_%06X;' % (t, t)
            return 'cpu.PC = 0x%X; return 1;' % t
        for pc in pcs:
            ins = insns[pc]
            L.append('L_%06X: /* %s */' % (pc, ins.text or ins.kind))
            if pc in HLE:
                L.append('  %s(); CYC += %d; return 1; /* HLE */' % (HLE[pc]['fn'], HLE[pc].get('cycles', 2000)))
                continue
            if pc in leaders and pc not in (0,):
                if SCHED: L.append('  INTENT_TRACE(0x%X);' % pc)
                if SCHED and EVT_CHECK: L.append('  if (CYC >= EVT) { cpu.PC = 0x%X; return 1; }' % pc)
                if not SCHED: L.append('  if (CYC >= EVT) { cpu.PC = 0x%X; return 1; }' % pc)
                elif pc in YIELDS: L.append('  if (intent_yield_leader(0x%X)) { cpu.PC = 0x%X; return 1; }' % (pc, pc))
                else: L.append('  if (__builtin_expect(--intent_wd <= 0, 0)) { if (intent_watchdog(0x%X)) { cpu.PC = 0x%X; return 1; } }' % (pc, pc))
            L.append('  CYC += %d; ICNT++;' % ins.cyc)      # sched mode keeps the cycle COUNT: compute is charged to virtual time at the next yield (no EVT preemption)
            body = list(ins.code)
            nxt = pc + ins.length
            if ins.kind == 'dbcc':
                br = jump(ins.static_target)
                body = [b.replace('BRANCH', br) for b in body]
            for b in body:
                L.append('  ' + b)
            if ins.kind == 'bra':
                L.append('  ' + jump(ins.static_target))
            elif ins.kind == 'bsr':
                L.append('  ' + jump(ins.static_target))
            elif ins.kind == 'bcc':
                L.append('  if (cond(%d)) { %s }' % (ins.cc, jump(ins.static_target)))
                L.append('  CYC += %d; /* not taken */' % (-2 if (ins.op & 0xff) else 2))
            elif ins.text in ('jsr', 'jmp') and hasattr(ins, 'static_target'):
                L.append('  ' + jump(ins.static_target))
            elif ins.dyn:
                L.append('  cpu.PC = PC_; return 1;')
            elif getattr(ins, 'sr_change', False):
                L.append('  cpu.PC = 0x%X; return 1;' % nxt)
            if ins.fall and not ins.dyn and ins.kind not in ('bra',):
                following = pcs[pcs.index(pc) + 1] if pcs.index(pc) + 1 < len(pcs) else None
                if nxt not in inchunk:
                    L.append('  cpu.PC = 0x%X; return 1;' % nxt)
                elif following != nxt:   # Cowork 17 Oct overlap: the next label in the text is not the next instruction
                    L.append('  ' + jump(nxt))
                elif nxt in leaders:
                    pass  # falls into the next label, which checks events
        L.append('dyn_exit:')
        L.append('  cpu.PC = PC_; return 1;')
        L.append('}')
        txt = '\n'.join(L) + '\n'
        if SCHED: txt = rewrite_hw(txt).replace('#include "sched.h"', '#include "sched.h"\n#include "hwcalls.h"', 1)
        open(os.path.join(outdir, fname + '.c'), 'w').write(txt)
    if SCHED: write_hwcalls(outdir)
    # dispatcher
    D = ['/* generated dispatcher */', '#include "cpu.h"', '#include "run.h"']
    for ck, f in names: D.append('int %s(void);' % f)
    allpcs = sorted(insns)
    D.append('uint64_t ICNT;')
    D.append('static const uint32_t pcs[%d] = {' % len(allpcs))
    for i in range(0, len(allpcs), 12): D.append('  ' + ', '.join('0x%X' % p for p in allpcs[i:i+12]) + ',')
    D.append('};')
    D.append('int game_is_translated(uint32_t pc) { int lo = 0, hi = %d - 1; while (lo <= hi) { int m = (lo + hi) / 2; if (pcs[m] == pc) return 1; if (pcs[m] < pc) lo = m + 1; else hi = m - 1; } return 0; }' % len(allpcs))
    D.append('chunk_fn game_chunk(uint32_t pc) {')
    D.append('  switch (pc >> 12) {')
    for ck, f in names: D.append('  case 0x%X: return %s;' % (ck, f))
    D.append('  default: return 0;')
    D.append('  }')
    D.append('}')
    open(os.path.join(outdir, 'dispatch.c'), 'w').write('\n'.join(D) + '\n')
    return names

def emit_single(insns, path):
    """Test build: one C function per instruction (for differential testing against Musashi)."""
    L = ['/* generated: one function per translated instruction, for tests/difftest.c */', '#include "cpu.h"', '#include "boot.h"', '#include "run.h"']
    names = []
    for pc in sorted(insns):
        ins = insns[pc]
        if pc in HLE: continue
        fn = 'I_%06X' % pc; names.append((pc, fn, ins.text or ins.kind))
        L.append('static int %s(void) {' % fn)
        L.append('  uint32_t ea, ea2, s, t, r, PC_; (void)ea; (void)ea2; (void)s; (void)t; (void)r; (void)PC_;')
        nxt = pc + ins.length
        jump = lambda t: 'cpu.PC = 0x%X; return 1;' % t
        body = list(ins.code)
        if ins.kind == 'dbcc': body = [b.replace('BRANCH', jump(ins.static_target)) for b in body]
        L.append('  CYC += %d;' % ins.cyc)
        for b in body: L.append('  ' + b)
        if ins.kind in ('bra', 'bsr'): L.append('  ' + jump(ins.static_target))
        elif ins.kind == 'bcc':
            L.append('  if (cond(%d)) { %s }' % (ins.cc, jump(ins.static_target)))
            L.append('  CYC += %d; /* not taken, as in the chunk code */' % (-2 if (ins.op & 0xff) else 2))
        elif ins.text in ('jsr', 'jmp') and hasattr(ins, 'static_target'): L.append('  ' + jump(ins.static_target))
        elif ins.dyn: L.append('  cpu.PC = PC_; return 1;')
        L.append('  cpu.PC = 0x%X; return 1;' % nxt)
        L.append('dyn_exit:')
        L.append('  cpu.PC = PC_; return 1;')
        L.append('}')
    L.append('typedef struct { uint32_t pc; int (*fn)(void); const char *name; int abs_hw; } single_t;')
    L.append('const single_t singles[] = {')
    for pc, fn, nm in names:
        code = ' '.join(insns[pc].code)
        hw = any(tok in code for tok in ('0xDFF', '0xBFE', '0xBFD'))
        L.append('  {0x%X, %s, "%s", %d},' % (pc, fn, nm, 1 if hw else 0))
    L.append('};')
    L.append('const int n_singles = %d;' % len(names))
    open(path, 'w').write('\n'.join(L) + '\n')

def blocks(ins):
    """Basic-block leaders of a set of instructions (as the pilot's coverage.py counted them)."""
    fallen = set(); lead = set()
    for pc, i in ins.items():
        nxt = pc + i.length
        if i.fall and not i.call and i.kind not in ('bra',) and not i.dyn: fallen.add(nxt)
        for t in i.targets: lead.add(t)
        if i.kind in ('bcc', 'dbcc') or i.call or not i.fall or i.dyn or getattr(i, 'sr_change', False): lead.add(nxt)
    for pc in ins:
        if pc not in fallen: lead.add(pc)
    return sorted(l for l in lead if l in ins)

def main():
    global HLE, SCHED, YIELDS, EVT_CHECK
    outdir = sys.argv[1]
    cfgp = None; extra = []
    for i, a in enumerate(sys.argv):
        if a == '--config': cfgp = sys.argv[i + 1]
        if a == '--seeds':
            for f in sys.argv[i + 1:]:
                if f.startswith('--'): break
                extra.append(f)
    cfg = portcfg.load(cfgp)
    HLE = cfg['hle']
    SCHED = bool(cfg.get('sched')); EVT_CHECK = bool(cfg.get('evt'))
    YIELDS = {portcfg.num(x) for x in cfg.get('yields', [])}
    seeds, ghidra_used = portcfg.all_seeds(cfg)
    executed = set()
    for f in extra:
        executed.update(portcfg.hex_lines(f))
    for pat in cfg['seeds']:
        if 'executed' in pat:
            import glob as _g
            for f in _g.glob(portcfg.path(pat)): executed.update(portcfg.hex_lines(f))
    seeds |= executed
    mem = Mem(cfg); dec = Decoder(mem)
    if mem.kick:   # G2E10 kickroots: every PC the reference ran may be entered from the interpreted Kickstart
        try:
            for _l in open(portcfg.path('out/port/ref/first_bytes.txt')):
                _t = _l.split()
                if _t: seeds.add(int(_t[0], 16))
        except (OSError, ValueError):
            pass
    outside = sorted(s for s in seeds if not mem.inrange(s))
    seeds = {s for s in seeds if mem.inrange(s)}
    insns, bad, leaders, jt = discover(mem, dec, seeds)
    confirmed = dict(insns)
    spec = (0, 0, 0)
    if '--no-speculative' not in sys.argv:
        for _ in range(3):
            spec = speculative(mem, dec, insns, leaders)
    rust = None
    if '--emit' in sys.argv and sys.argv[sys.argv.index('--emit') + 1] == 'rust':
        import m68k2rs
        os.makedirs(outdir, exist_ok=True)
        rust = m68k2rs.emit_rust(insns, os.path.join(outdir, 'translated.rs'), HLE)
        names = []
    else:
        names = emit(mem, insns, leaders, outdir)
    for i, a in enumerate(sys.argv):
        if a == '--single': emit_single(insns, sys.argv[i + 1])
    # statistics
    blks = sorted(l for l in leaders if l in insns)
    stats = {
        'instructions_translated': len(insns),
        'block_leaders_translated': len(blks),
        'undecodable_addresses_reached': len(bad),
        'seeds': len(seeds),
        'seeds_outside_code_ranges': len(outside),
        'jump_tables': len(jt),
        'chunks': len(names),
        'bytes_of_code': sum(i.length for i in insns.values()),
        'confirmed_instructions': len(confirmed),
        'speculative_instructions': len(insns) - len(confirmed),
        'speculative_roots_accepted_last_pass': spec[0], 'speculative_roots_rejected_last_pass': spec[1],
        'confirmed_block_leaders': len([l for l in blks if l in confirmed]),
        'hle': ['%06X %s' % (k, v['fn']) for k, v in sorted(HLE.items())],
        'runtime_code_pcs_used': len(mem.observed), 'runtime_code_pcs_changed_left_to_interpreter': mem.volatile,
        'ghidra_seed_files': ghidra_used,
    }
    if rust is not None:
        stats['rust_instructions_translated'] = rust['translated']
        stats['rust_untranslated'] = {('%06X' % k): v for k, v in sorted(rust['untranslated'].items())}
    json.dump({'stats': stats, 'bad': {('%06X' % k): v for k, v in sorted(bad.items())}}, open(os.path.join(outdir, 'translate_stats.json'), 'w'), indent=1)
    open(os.path.join(outdir, 'translated_pcs.txt'), 'w').write(''.join('%06X\n' % p for p in sorted(insns)))
    open(os.path.join(outdir, 'leaders.txt'), 'w').write(''.join('%06X\n' % p for p in blks))
    for i, a in enumerate(sys.argv):
        if a == '--coverage':
            cb = blocks(confirmed)
            # an undecodable address the discovery reached is run by the fallback interpreter (runtime/cpu.c)
            # if the game ever gets there: it is reachable and interpreted, never silently "translated"
            interp = sorted(bad)
            ex_not = sorted(p for p in executed if mem.inrange(p) and p not in insns)
            cov = {'reachable': len(cb) + len(interp), 'translated': len(cb), 'interpreted': len(interp),
                   'interpreted_list': ['%06X: %s (fallback interpreter)' % (k, bad[k]) for k in interp][:200],
                   'speculative_extra_blocks': len(blocks(insns)) - len(cb),
                   'executed_pcs_logged': len(executed), 'executed_pcs_not_translated': len(ex_not),
                   'executed_not_translated_list': ['%06X' % p for p in ex_not[:200]],
                   'code_ranges': ['%06X-%06X' % r for r in mem.ranges],
                   'runtime_code_pcs_used': len(mem.observed), 'runtime_code_pcs_changed_left_to_interpreter': mem.volatile,
                   'seeds': len(seeds), 'seeds_outside_code_ranges': len(outside),
                   'ghidra_seed_files': ghidra_used,
                   'made_by': 'kit/amigo/port/m68k2c.py'}
            json.dump(cov, open(sys.argv[i + 1], 'w'), indent=1)
    print(json.dumps(stats))
    print('undecodable:', len(bad), list(sorted(bad.items()))[:10])

if __name__ == '__main__':
    main()
