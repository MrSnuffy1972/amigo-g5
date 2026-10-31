#!/usr/bin/env python3
"""portcfg.py: game.json, the one per-game description the Ami-Go port pipeline needs.

Written 13 Oct 2026 (Karl: use the proven pilot toolchain after Ghidra has done its analysis, to take
load off the models). The pilot's translator, runtime and harness were hard-wired to Ghosts 'n Goblins
(its code ranges, its loader HLE, its memory builder). Everything game-specific now lives here, so the
same code translates, builds, tests and compares any game; a model only writes game.json and, where
the game talks to disk or the OS, a small hooks.c.

game.json (paths are relative to the work folder, /w in a session; numbers may be "0x..." strings):
  name            "3D Pool"
  segments        [{"file": "out/image.bin", "addr": "0x20000", "offset": 0, "length": null}, ...]
                  memory as the game's loader leaves it at the entry point (G0's image.bin at BASE)
  whdload         {"slave": "game/x/Game.slave"}: boot through the kit's C resload (runtime/whd.c), no WHDLoad, no
                  Kickstart; every file in the slave's folder is embedded (written by portg0.py for direct slaves)
                  except names matching the optional "exclude" list (fnmatch patterns, relative to that folder)
  translate_only  same form: memory the translator may read code from that is loaded later at run time
  copies          [{"src": "0x2006E", "dst": "0x100A", "len": "0x15D06"}]: block copies the game makes
                  of its own code before running it (a stub that moves the program). The translator sees
                  the copied bytes at dst; Ghidra seeds inside src are mapped to dst as well.
  code            ["0x20000-0x36004", ...] address ranges instructions may come from
                  (default: the `code` REGION lines of gates/G0/MEMMAP.md plus every copies dst)
  entry           start PC (default: ENTRY in MEMMAP.md)
  sr              start status register (default "0x2700": supervisor, interrupts masked)
  regs            {"a7": "0x7FFF0", "d0": ...} start registers (default a7 = 0x7FFF0)
  seeds           extra start addresses for code discovery: files of hex addresses, one per line,
                  e.g. "out/port/ref/executed_pcs.txt" (the harness writes it). Ghidra's functions.txt
                  files under evidence/precook are added automatically when "ghidra" is true (default)
  seed_addrs      ["0x7BF0E"] single extra seeds (an interrupt handler the loader installed, say)
  hle             {"0xC0C0D4": {"fn": "hle_disk_load", "cycles": 2000}} a C function in hooks.c
                  replaces the routine at that address (see runtime/boot.h for the contract)
  hooks_c         ["gates/G1/hooks.c"] game-specific C compiled into the port, the harness and difftest
  boot_extra      true: hooks.c defines game_boot_extra(regs, pc, sr), called after memory is set up
  pokes           [["0x100", "0x6804"], ...] custom chip registers (offset from $DFF000) set at boot
  files           ["game/*.paradox"] game files embedded for run-time loads (gamefile() in hooks.c)
  title           window title (default name)
"""
import glob, json, os, re

W = os.environ.get("AMIGO_W", "/w")
KIT = os.path.dirname(os.path.abspath(__file__))


def num(v):
    if v is None:
        return None
    if isinstance(v, int):
        return v
    v = str(v).strip().replace("$", "0x")
    return int(v, 0)


def path(p):
    return p if os.path.isabs(p) else os.path.join(W, p)


def parse_memmap(p):
    base = entry = None
    regions = []
    for line in open(p, errors="replace"):
        m = re.match(r"\s*BASE:\s*\$?(?:0x)?([0-9A-Fa-f]+)", line)
        if m:
            base = int(m.group(1), 16)
        m = re.match(r"\s*ENTRY:\s*\$?(?:0x)?([0-9A-Fa-f]+)", line)
        if m:
            entry = int(m.group(1), 16)
        m = re.match(r"\s*REGION:\s*\$?(?:0x)?([0-9A-Fa-f]+)\s*-\s*\$?(?:0x)?([0-9A-Fa-f]+)\s+(\w+)", line)
        if m:
            regions.append((int(m.group(1), 16), int(m.group(2), 16), m.group(3).lower()))
    return base, entry, regions


def ranges(spec):
    out = []
    for r in spec:
        if isinstance(r, (list, tuple)):
            out.append((num(r[0]), num(r[1])))
        else:
            a, b = str(r).split("-")
            out.append((num(a), num(b)))
    return out


def load(p=None):
    p = p or path("game.json")
    c = json.load(open(p))
    mm = path("gates/G0/MEMMAP.md")
    base = entry = None
    regions = []
    if os.path.exists(mm):
        base, entry, regions = parse_memmap(mm)
    c["_memmap"] = {"base": base, "entry": entry, "regions": regions}
    c.setdefault("name", "Amiga game")
    c.setdefault("title", c["name"])
    c["entry"] = num(c.get("entry", entry))
    c["sr"] = num(c.get("sr", "0x2700"))
    regs = {k.lower(): num(v) for k, v in (c.get("regs") or {}).items()}
    regs.setdefault("a7", 0x7FFF0)
    c["regs"] = regs
    for k in ("segments", "translate_only"):
        c[k] = [dict(s, addr=num(s["addr"]), offset=num(s.get("offset", 0)) or 0, length=num(s.get("length")))
                for s in c.get(k, [])]
    c["copies"] = [dict(src=num(x["src"]), dst=num(x["dst"]), len=num(x["len"])) for x in c.get("copies", [])]
    if c.get("code"):
        c["code"] = ranges(c["code"])
    else:
        c["code"] = [(a, b) for a, b, k in regions if k == "code"] + [(x["dst"], x["dst"] + x["len"]) for x in c["copies"]]
    c["hle"] = {num(k): (v if isinstance(v, dict) else {"fn": v}) for k, v in (c.get("hle") or {}).items()}
    c["seed_addrs"] = [num(x) for x in c.get("seed_addrs", [])]
    c["pokes"] = [(num(a), num(b)) for a, b in c.get("pokes", [])]
    c.setdefault("seeds", [])
    c.setdefault("hooks_c", [])
    c.setdefault("files", [])
    c.setdefault("ghidra", True)
    c.setdefault("whdload", None)
    return c


def seg_bytes(s):
    d = open(path(s["file"]), "rb").read()
    o = s.get("offset") or 0
    n = s.get("length")
    return d[o:o + n] if n is not None else d[o:]


def hex_lines(p):
    """Addresses from a file: the first hex token of each line (executed_pcs.txt, seeds.txt and Ghidra's
    functions.txt, whose lines are '00c00000  ENTRY  size=68  callers=0')."""
    out = []
    for line in open(p, errors="replace"):
        m = re.match(r"\s*(?:\$|0x)?([0-9A-Fa-f]{4,8})\b", line)
        if m:
            out.append(int(m.group(1), 16))
    return out


def ghidra_seeds(c):
    """Function starts from the library pre-analysis (evidence/precook/exe/*/functions.txt). Ghidra
    analysed each executable as hunkload laid it out (hunks.txt BASE); when G0 put the image somewhere
    else the addresses are shifted by the difference, and functions inside a block the game copies
    are mapped to where it copies them."""
    out = set()
    if not c.get("ghidra", True):
        return out, []
    used = []
    mine = {(a, b) for a, b, k in c["_memmap"]["regions"]}
    for fn in sorted(glob.glob(path("evidence/precook/exe/*/functions.txt"))):
        hk = os.path.join(os.path.dirname(fn), "hunks.txt")
        gbase = None
        if not os.path.exists(hk):
            continue
        regs = set()
        for line in open(hk):
            m = re.match(r"BASE:\s*\$([0-9A-Fa-f]+)", line)
            if m:
                gbase = int(m.group(1), 16)
            m = re.match(r"REGION:\s*\$([0-9A-Fa-f]+)-\$([0-9A-Fa-f]+)", line)
            if m:
                regs.add((int(m.group(1), 16), int(m.group(2), 16)))
        # only the executable G0 actually loaded: the same hunk layout, possibly at another base (13 Oct, 3D Pool:
        # the disk also holds Menu, linkprot and 3D-Pictures, whose functions are not this program's)
        if mine and gbase is not None and c["_memmap"]["base"] is not None:
            sh_ = c["_memmap"]["base"] - gbase
            if {(a + sh_, b + sh_) for a, b in regs} != mine:
                continue
        shift = 0
        mb = c["_memmap"]["base"]
        if gbase is not None and mb is not None:
            shift = mb - gbase
        addrs = [a + shift for a in hex_lines(fn)]
        for a in addrs:
            out.add(a)
            for x in c["copies"]:
                if x["src"] <= a < x["src"] + x["len"]:
                    out.add(a - x["src"] + x["dst"])
        used.append("%s (%d functions, shift %+#x)" % (os.path.relpath(fn, W), len(addrs), shift))
    return out, used


def observed_bytes(c):
    """address -> byte, from out/port/ref/first_bytes.txt (harness FIRSTBYTES): the bytes at each executed PC at the moment it first ran."""
    fx = {}
    for f, fill in ((path("out/port/ref/first_bytes.txt"), False), (path(PLAY + "/first_bytes.txt"), True)):
        if os.path.exists(f):
            _first_bytes_into(fx, f, fill)
    return fx


# 31 Oct 2026 (Karl: "Record play, then translate"): the reference run is recorded with no input, so code a game only reaches
# after player input was never seen and never translated (Theme Park stopped at tick 1119, Jail Break at 519, when played).
# Ryzen records the original again with scripted input and merges those runs here: executed_pcs.txt (more seeds) and
# first_bytes.txt (the bytes they ran with). The no-input reference wins wherever both saw an address.
PLAY = "out/port/play"


def _first_bytes_into(fx, f, fill):
    """first_bytes.txt lines into fx; fill=True only adds addresses fx does not have yet."""
    dist = {}   # G2E2 fxown (carried over from G1E3, 25 Oct 2026): a byte comes from the line whose PC is nearest below it (its own
    for line in open(f):   # instruction's line). First-line-wins let a lower PC's 10-byte look-ahead, taken before a copy loop rewrote
        t = line.split()   # the bytes, win over the bytes the instruction really ran with.
        if len(t) < 2:
            continue
        a = int(t[0], 16)
        for i, b in enumerate(t[1:]):
            if fill and a + i in fx and a + i not in dist:
                continue
            if a + i not in dist or i < dist[a + i]:
                dist[a + i] = i; fx[a + i] = int(b, 16)


def observed(c):
    """Code the harness saw running outside the configured code ranges (code the game copied, unpacked or
    loaded at run time). Each such PC is translated from the harness's end-of-run RAM only when its opcode
    word there is the one it first ran with, and never when it changed during the run (self-modifying or
    overlaid code stays with the fallback interpreter)."""
    ops = {}
    vol = 0
    f = path("out/port/ref/executed_ops.txt")
    if not c.get("runtime_code", True) or not os.path.exists(f):
        return ops, vol
    rng = c["code"]
    for line in open(f):
        t = line.split()
        if len(t) < 2:
            continue
        a, op = int(t[0], 16), int(t[1], 16)
        if any(s_ <= a < e for s_, e in rng):
            continue
        if len(t) > 2:
            vol += 1
            continue
        ops[a] = op
    return ops, vol


def all_seeds(c):
    s = set(c["seed_addrs"])
    if c["entry"] is not None:
        s.add(c["entry"])
    for pat in c["seeds"]:
        for f in sorted(glob.glob(path(pat))):
            s.update(hex_lines(f))
    g, used = ghidra_seeds(c)
    return s | g, used


def init(force=False):
    """Write game.json from G0's results (MEMMAP.md + out/image.bin) when there is none."""
    p = path("game.json")
    if os.path.exists(p) and not force:
        return p, False
    base, entry, regions = parse_memmap(path("gates/G0/MEMMAP.md"))
    if base is None or entry is None:
        raise SystemExit("gates/G0/MEMMAP.md has no BASE/ENTRY line: finish G0 first")
    g0 = {}
    try:
        g0 = json.load(open(path("gates/G0/g0.json")))
    except Exception:
        pass
    if g0.get("whdload"):
        # a WHDLoad slave: the kit's C resload (runtime/whd.c) loads the slave and its files at boot, so there are no
        # image segments; out/image.bin (the slave's code) is only the translator's view of the loader
        c = {
            "name": os.environ.get("AMIGO_GAME", "Amiga game"),
            "whdload": {"slave": g0["whdload"]},
            "segments": [],
            "translate_only": [{"file": "out/image.bin", "addr": "0x%X" % base}],
            "entry": "0x%X" % entry,
            "sr": "0x2000",
            "regs": {},
            "seeds": ["out/port/ref/executed_pcs.txt", "gates/G1/seeds.txt"],
            "hle": {},
            "hooks_c": [],
            "files": [],
            "ghidra": False,
            "_note": "made by portgen.py init for a WHDLoad slave (gates/G0/g0.json); edit freely (see kit/amigo/port/portcfg.py)",
        }
        json.dump(c, open(p, "w"), indent=1)
        return p, True
    c = {
        "name": os.environ.get("AMIGO_GAME", "Amiga game"),
        "segments": [{"file": "out/image.bin", "addr": "0x%X" % base}],
        "entry": "0x%X" % entry,
        "sr": "0x2700",
        "regs": {"a7": "0x7FFF0"},
        "seeds": ["out/port/ref/executed_pcs.txt", "gates/G1/seeds.txt"],
        "hle": {},
        "hooks_c": [],
        "files": [],
        "_note": "made by portgen.py init from gates/G0/MEMMAP.md; edit freely (see kit/amigo/port/portcfg.py)",
    }
    json.dump(c, open(p, "w"), indent=1)
    return p, True
