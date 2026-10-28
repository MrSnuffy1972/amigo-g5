"""cs_snapcfg.py TRIAGE: generic version of Claude Code's mk_snapcfg.py (which is IK+-specific: fixed code ranges and resload HLE).
Turns the harness snapshot (out/snap) into /w/game.json that boots the game's own state at the snapshot PC: no slave, no resload,
no OS layer. Segments = non-zero 256-byte blocks of chip and slow RAM. Seeds = executed PCs whose FIRST execution came at or after
the snapshot PC (loader code that only ran before it is left out). Code ranges = the snapshot segments. The original game.json is
kept as game-orig.json."""
import json, os, sys
W = "/w"; snap = os.path.join(W, "out", "snap")
regs = {}
for l in open(os.path.join(snap, "snap_regs.txt")):
    p = l.split()
    if len(p) >= 2 and (p[0] in [f"d{i}" for i in range(8)] + [f"a{i}" for i in range(8)] or p[0] in ("pc", "sr", "usp", "isp")):
        regs[p[0]] = int(p[1], 16)
chip = open(os.path.join(snap, "snap_chip.bin"), "rb").read()
slow = open(os.path.join(snap, "snap_slow.bin"), "rb").read()
B = 256
def blocks(buf):
    segs, cur = [], None
    for off in range(0, len(buf), B):
        if any(buf[off:off + B]):
            if cur and cur[0] + cur[1] == off: cur[1] += B
            else: cur = [off, B]; segs.append(cur)
    return segs
cs, ss = blocks(chip), blocks(slow)
os.makedirs(os.path.join(W, "snapcfg"), exist_ok=True)
out, code, pos = [], [], 0
with open(os.path.join(W, "snapcfg", "chip.bin"), "wb") as f:
    for off, ln in cs:
        f.write(chip[off:off + ln]); out.append({"file": "snapcfg/chip.bin", "addr": "0x%X" % off, "offset": pos, "length": ln}); pos += ln
        if off + ln > 0x100: code.append("0x%X-0x%X" % (max(off, 0x100), off + ln))   # v0d: floor $100 (code ran at $102, $240, $398)
    for off, ln in ss:
        f.write(slow[off:off + ln]); out.append({"file": "snapcfg/chip.bin", "addr": "0x%X" % (0xC00000 + off), "offset": pos, "length": ln}); pos += ln
        code.append("0x%X-0x%X" % (0xC00000 + off, 0xC00000 + off + ln))
cfg = json.load(open(os.path.join(W, "game-orig.json")))
for k in ("whdload", "translate_only", "hle"): cfg.pop(k, None)
cfg["name"] = cfg.get("name", "game") + " (clean-sheet v0 snapshot)"
cfg["segments"] = out; cfg["entry"] = "0x%X" % regs["pc"]; cfg["sr"] = "0x%X" % regs["sr"]
cfg["regs"] = {k: "0x%X" % v for k, v in regs.items() if k[0] in "da" and len(k) == 2}
cfg["sched"] = True
cfg["yields"] = []
cfg["hle"] = {}
RESLOAD_C = r'''
/* v0d resload services for the native exe (WORKAROUND in the per-game hooks until runtime/nativefs.c carries them; behaviour copied
 * from runtime/whd.c r_diskload / r_loadfileoffset / r_crc16). Embedded install files come from game.json "files". */
#include <stdio.h>
#include <stdlib.h>
#include "amiga.h"
#include "cpu.h"
#include "boot.h"
static void cs_rt(void) { cpu.PC = mem_rl(cpu.ar[7]); cpu.ar[7] += 4; cpu.N = (cpu.dr[0] >> 31) & 1; cpu.Z = cpu.dr[0] == 0; cpu.V = 0; cpu.C = 0; }
void cs_ret(void) { cs_rt(); }
void cs_ret0(void) { cpu.dr[0] = 0; cs_rt(); }
void cs_diskload(void) {             /* D0 offset, D1 size, D2 disk, A0 destination */
    char n[32]; snprintf(n, sizeof n, "Disk.%u", cpu.dr[2] & 0xFF);   /* disk number is the low byte (Disk.65537 = $10001 aborts seen in 8 G0 runs) */ size_t len = 0; const uint8_t *b = gamefile(n, &len);
    uint32_t off = cpu.dr[0], size = cpu.dr[1];
    if (!b || off + size > len) { fprintf(stderr, "FINDING: resload DiskLoad %s +$%X size $%X not available (embedded %zu bytes)\n", n, off, size, b ? len : 0); cpu.dr[0] = 0; cpu.dr[1] = 205; cs_rt(); return; }
    for (uint32_t i = 0; i < size; i++) mem_wb(cpu.ar[0] + i, b[off + i]);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; cs_rt();
}
void cs_loadfileoffset(void) {       /* D0 size, D1 offset, A0 name, A1 destination */
    char n[256]; int i = 0; for (; i < 255; i++) { uint8_t c = (uint8_t)mem_rb(cpu.ar[0] + i); if (!c) break; n[i] = (char)c; } n[i] = 0;
    size_t len = 0; const uint8_t *b = gamefile(n, &len); uint32_t size = cpu.dr[0], off = cpu.dr[1];
    if (!b || off > len) { fprintf(stderr, "FINDING: the game asked for file '%s' which is not embedded\n", n); cpu.dr[0] = 0; cpu.dr[1] = 205; cs_rt(); return; }
    if (off + size > len) size = (uint32_t)(len - off);
    for (uint32_t k = 0; k < size; k++) mem_wb(cpu.ar[1] + k, b[off + k]);
    cpu.dr[0] = 0xffffffffu; cpu.dr[1] = 0; cs_rt();
}
void cs_crc16(void) {                /* D0 length, A0 data -> D0 CRC-16 ($A001 reflected, start 0) */
    uint16_t c = 0; for (uint32_t i = 0; i < cpu.dr[0]; i++) { c ^= (uint16_t)mem_rb(cpu.ar[0] + i); for (int k = 0; k < 8; k++) c = (c & 1) ? (uint16_t)((c >> 1) ^ 0xA001) : (uint16_t)(c >> 1); }
    cpu.dr[0] = c; cs_rt();
}
'''
RESLOAD_SLOTS = {   # v0d (Argon untranslated answer, 58/58 key lines checked): resload slot offset -> native function in hooks.c / nativefs.c
    0x08: "native_loadfile", 0x0C: "native_savefile", 0x24: "native_getfilesize",
    0x10: "cs_ret0", 0x20: "cs_ret", 0x28: "cs_diskload", 0x30: "cs_crc16", 0x34: "cs_ret0", 0x4C: "cs_loadfileoffset",
    0x54: "cs_ret", 0x60: "cs_ret0",
    0x18: "native_decrunch", 0x1C: "native_loadfiledecrunch", 0x70: "native_getfilesizedec"}   # G2E9 ndecrunch (runtime/nativefs.c)
def _whd_curdir(p):
    """G2E9 nfiles: ws_CurrentDir of a WHDLoad slave, read as runtime/whd.c whd_boot reads it ("" when none or unreadable)."""
    import struct
    try:
        b = open(p, "rb").read(); be = lambda o: struct.unpack(">I", b[o:o + 4])[0]
        if be(0) != 0x3F3: return ""
        o = 4
        while be(o): o += 4 + 4 * be(o)
        o += 4; first, last = be(o + 4), be(o + 8); o += 12 + 4 * (last - first + 1)
        if be(o) & 0x3fffffff != 0x3E9: return ""
        code = b[o + 8:]; cd = struct.unpack(">H", code[26:28])[0]
        return code[cd:code.index(b"\0", cd)].decode("latin-1") if cd else ""
    except Exception:
        return ""
rbase = None
if sys.argv[1] == "whdload":
    import re as _re
    for l in open(os.path.join(W, "gates/CS/1-ref.log"), errors="replace") if os.path.exists(os.path.join(W, "gates/CS/1-ref.log")) else []:
        m = _re.search(r"slave \$([0-9A-F]{6})", l)
        if m: rbase = int(m.group(1), 16) - 0x100; break
    if rbase is None and len(slow) == 0x80000: rbase = 0xC7E000
    if rbase is not None:
        cfg["hle"] = {"0x%X" % (rbase + o): {"fn": f, "cycles": 200} for o, f in RESLOAD_SLOTS.items()}
        sl = json.load(open(os.path.join(W, "game-orig.json"))).get("whdload", {}).get("slave")
        if sl: cfg["files"] = [os.path.join(os.path.dirname(sl), "*")]   # v0d: embed the install's files (Disk.N, data) so native loads find them
        if sl:   # G2E9 nfiles (Claude sub-agent, 27 Oct 2026): the whole install, named as whd.c resolves it (portgen "files_tree")
            cfg["files_tree"] = {"dir": os.path.dirname(sl), "curdir": _whd_curdir(os.path.join(W, sl)),
                                 "exclude": json.load(open(os.path.join(W, "game-orig.json"))).get("whdload", {}).get("exclude", [])}
cfg["boot_extra"] = True
cfg["hooks_c"] = ["snapcfg/hooks.c"]
# Cowork 16 Oct 23:55 (measured on game 135): a FILED game's OS calls are served by oshle (game.json "hle": ~750 library
# vectors -> hle_lib, hooks oshle.c + gates/G1/oshle_cfg.c). v0 dropped both, so in the native exe every OpenLibrary/Open/AllocMem
# hit bare stub code and returned garbage: DOSBase 0, JSR -30(A6) to $FFFFFFE2 = "untranslated address". Keep them; oshle's own
# game_boot_extra is renamed (the snapshot already holds the booted state; snapcfg/hooks.c sets the stack).
_orig = json.load(open(os.path.join(W, "game-orig.json")))
# G2E7 oshkeep (Claude sub-agent, 26 Oct 2026): every non-WHDLoad triage whose prekit game.json carries oshle keeps it (was "filed"
# only: multi-disk and trackloader OS games got a native exe with 0 HLE routines and stopped at tick 0 on a garbage library base).
if sys.argv[1] != "whdload" and any(h.endswith("oshle/oshle.c") for h in _orig.get("hooks_c", [])):
    _k = [h for h in _orig["hooks_c"] if h.endswith("oshle/oshle.c")][0]
    _src = open(os.path.join(W, _k)).read().replace("\nvoid game_boot_extra(", "\nvoid oshle_boot_extra_snapshot_unused(")
    open(os.path.join(W, "snapcfg", "oshle_cs.c"), "w").write(_src)
    cfg["hooks_c"] = ["snapcfg/oshle_cs.c"] + [h for h in _orig["hooks_c"] if not h.endswith("oshle/oshle.c")] + ["snapcfg/hooks.c"]
    cfg["hle"] = dict(_orig.get("hle", {}))
    _osh_kept = True   # G2E7 stubseed
# Cowork 18 Oct (Karl approved): slow-RAM code range raised from the old $C80000 clip to the real top of slow RAM, i.e. the
# memory model the runtime and translator are built with (m68k2c.mem_sizes / amigo_port.sh memdefs: WHD_SLOW_KB, else
# gates/G0/g0.json slow_kb, else 512), never past the snapshot. WHDLoad puts ExpMem at $C00000-$C81000 and slave code up to
# about $D00000 on 1 MB games (game 78 stopped at $CFFF2A). Chip: see CHIPTOP below.
_sk = 512
try:
    _sk = int(json.load(open(os.path.join(W, "gates/G0/g0.json"))).get("slow_kb") or 512)
except Exception:
    pass
_sk = int(os.environ.get("WHD_SLOW_KB") or _sk)
SLOWTOP = min(len(slow), _sk * 1024)
# Cowork fx 18 Oct (Karl approved): chip code range widened the same way, to the real chip RAM size of the memory model the
# translator and runtime are built with (m68k2c.mem_sizes / amigo_port.sh memdefs: WHD_CHIP_KB, else gates/G0/g0.json chip_kb,
# else 512), never past the snapshot. Game 7 (chip_kb 1024) stopped at $080D0C past the old $80000 clip.
_ck = 512
try:
    _ck = int(json.load(open(os.path.join(W, "gates/G0/g0.json"))).get("chip_kb") or 512)
except Exception:
    pass
_ck = int(os.environ.get("WHD_CHIP_KB") or _ck)
CHIPTOP = min(len(chip), _ck * 1024)
try:   # Cowork 18 Oct: per-game rollback of the chip clip; /w/cs_chip_clip e.g. 0x80000 caps CHIPTOP
    _cc = open('/w/cs_chip_clip').read().strip()
    if _cc: CHIPTOP = min(CHIPTOP, int(_cc, 0))
except OSError: pass
cfg["code"] = ["0x100-0x%X" % CHIPTOP, "0xC00000-0x%X" % (0xC00000 + SLOWTOP)]   # v0c: clipped to the translator's memory windows (chip < $80000, slow < $C80000); beyond them m68k2c Mem.rb raises IndexError (8 build failures: Wings 56 at $C80000, Chaos Engine 81 at $80000), reported to Claude Code   # all RAM: the seeds decide what is code; code the game loads AFTER the snapshot is read from FIRSTBYTES (v0b: v0a used only the non-zero snapshot blocks, which left later-loaded code "out of code range")
order = [int(l.split()[0], 16) for l in open(os.path.join(W, "out/port/ref/first_bytes.txt")) if l.strip()]
sp = regs["pc"]
after = order[order.index(sp):] if sp in order else order
inmem = lambda a: a < CHIPTOP or 0xC00000 <= a < 0xC00000 + SLOWTOP
seeds = ["%06X" % a for a in after if inmem(a)]
if globals().get("_osh_kept"):   # G2E7 stubseed: oshle's execlib stub, entered by HLE PC jumps the reference does not record
    seeds += ["%06X" % a for a in (0x3E00, 0x3E16, 0x3E20, 0x3E24) if "%06X" % a not in seeds]
open(os.path.join(W, "snapcfg", "seeds.txt"), "w").write("\n".join(seeds) + "\n")
cfg["seeds"] = ["snapcfg/seeds.txt"]
# Cowork 17 Oct: custom-chip state at the snapshot (snap_regs.txt "custom", "intena", "dmacon" lines), restored at boot by snapcfg/hooks.c
_cust = [0] * 256; _ie = _dm = 0
_CIA_KEYS = ("pra", "prb", "ddra", "ddrb", "icrmask", "cra", "crb", "ta", "tb", "talatch", "tblatch", "icr", "sdr", "tod", "todalarm", "todstop")
_cia = {}
for _l in open(os.path.join(snap, "snap_regs.txt")):
    _p = _l.split()
    if len(_p) == 3 and _p[0] == "custom": _cust[int(_p[1], 16) >> 1] = int(_p[2], 16)
    elif len(_p) == 2 and _p[0] == "intena": _ie = int(_p[1], 16) & 0x7fff
    elif len(_p) == 2 and _p[0] == "dmacon": _dm = int(_p[1], 16) & 0x7fff
    elif len(_p) >= 16 and _p[0] == "cia" and _p[1] in ("0", "1"):   # g46: CIA state ('cia N key val ...'), restored by chipset_restore_cia
        _kv = dict(zip(_p[2::2], _p[3::2])); _ciav = []
        for _k in _CIA_KEYS:
            if _k not in _kv: break
            _ciav.append(int(_kv[_k], 16))
        _cia[int(_p[1])] = _ciav
# G2E9 ospfix (Claude sub-agent, 27 Oct 2026): boot_usp is the native runtime's OTHER stack pointer (cpu.OSP). A user-mode snapshot
# (SR bit 13 clear: A7 is the USP) needs the supervisor stack there, or interrupts push onto the game's user stack (135: wild jump at
# tick 1). Supervisor-mode snapshots keep the USP as before. No ISP in snap_regs: leave OSP at the runtime default (reset vector).
_osp = regs.get("usp", 0) if (regs.get("sr", 0x2000) & 0x2000) else regs.get("isp", 0)
open(os.path.join(W, "snapcfg", "hooks.c"), "w").write('''/* clean-sheet v0 snapshot boot: the user stack pointer the loader left (Cowork cs_snapcfg.py, after Claude Code's mk_snapcfg.py) */
#include <stdint.h>
extern uint32_t boot_usp; extern int boot_usp_set;
void chipset_restore_words(const uint16_t *, uint16_t, uint16_t);\nvoid chipset_restore_cia(int, const uint32_t *, int);\nstatic const uint16_t cs_custom[256] = {%s};\nvoid game_boot_extra(uint32_t *regs, uint32_t *pc, uint16_t *sr) { (void)regs; (void)pc; (void)sr; boot_usp = 0x%X; boot_usp_set = %d; chipset_restore_words(cs_custom, 0x%X, 0x%X);%s }\n''' % (",".join("0x%X" % w for w in _cust), _osp, 1 if _osp else 0, _ie, _dm,
    "".join(" { static const uint32_t v[] = {%s}; chipset_restore_cia(%d, v, %d); }" % (",".join("0x%X" % x for x in _cia[n]), n, len(_cia[n])) for n in sorted(_cia) if len(_cia[n]) >= 7)) + RESLOAD_C)
# G2E10 kick (Claude sub-agent, 27 Oct 2026): a kickemu snapshot has the Kickstart region zeroed; the native exe loads the PLAYER'S
# ROM there at boot (kit runtime/kickrt.c). Only its place, size, CRC16 and name go into game.json.
_kp = os.path.join(snap, "snap_kick.txt")
if os.path.exists(_kp):
    _kb, _ks, _kc, _kn = (open(_kp).read().split() + ["", "", "", ""])[:4]
    cfg["kick"] = {"base": _kb, "size": _ks, "crc": _kc, "name": _kn}
json.dump(cfg, open(os.path.join(W, "game.json"), "w"), indent=1)
print("snapshot PC %06X, chip segments %d, slow segments %d, bytes %d, seeds %d (of %d first-executed), usp %X" % (
    sp, len(cs), len(ss), pos, len(seeds), len(order), regs.get("usp", 0)))
