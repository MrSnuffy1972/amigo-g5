#!/usr/bin/env python3
"""portg0.py: gate G0 by code for games whose program is an AmigaDOS hunk executable (Ami-Go port kit).

Karl, 13 Oct 2026: "structurally fix things so that the local models can work, in the future, without stalling."
Measured that day: Gemma spent 7 G0 sessions on 3D Pool without passing (it copied the hunk file instead of
loading it), and the fault analysis even advised writing a hunk parser. Loading a hunk program is solved
(hunkload.py, LoadSeg's rules), the library pre-analysis already says which file is the program and where it
sits, and the harness can measure what the program does after it starts (code it copies elsewhere). So G0 for
these games is code: this file writes gates/G0/rebuild_image.sh and gates/G0/MEMMAP.md from measurements only.

  python3 portg0.py make      choose the program, load it, write the G0 files, run the harness to measure
  python3 portg0.py rebuild   (called by rebuild_image.sh) rebuild out/image.bin from /w/game

WHDLoad installs with a direct slave (14 Oct 2026): the slave is run on the harness through the kit's C resload
(runtime/whd.c, no WHDLoad, no Kickstart); G0 records the slave as the boot (gates/G0/g0.json "whdload"), the slave's
code as out/image.bin and the code it loads at run time as REGION lines. Kickemu slaves, slaves calling a resload
function the kit lacks, and packed files without a decruncher exit 2 with the reason.

Games with a custom trackloader (non-DOS disks, no hunk program) are not handled: exits 2 and says why, and
the model does G0 as before.
"""
import glob, json, os, re, struct, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))          # kit/amigo: adfx, hunkload
import adfx, hunkload                              # noqa: E402

W = os.environ.get("AMIGO_W", "/w")
CFG = os.path.join(W, "gates", "G0", "g0.json")
SKIP_SUFFIX = (".slave", ".library", ".device", ".resource", ".datatype", ".font")


def find_disk(rel):
    p = os.path.join(W, "game", rel)
    if os.path.exists(p):
        return p
    want = os.path.basename(rel)
    m = re.match(r"(.+\.hdf)\.p(\d+)\.img$", want, re.I)
    for dp, dn, fn in os.walk(os.path.join(W, "game")):
        for f in fn:
            if f == want or (m and f == m.group(1)):
                return os.path.join(dp, f)
    return None


def disk_image(path, rel):
    """Bytes of the AmigaDOS volume: an ADF as is, a hard-disk partition cut out of its .hdf."""
    m = re.search(r"\.p(\d+)\.img$", rel)
    if path.lower().endswith(".hdf"):
        parts = adfx.partitions(path)
        name, off, ln = parts[int(m.group(1)) if m else 0]
        with open(path, "rb") as f:
            f.seek(off)
            return f.read(ln)
    return open(path, "rb").read()


def extract(cfg):
    disk = find_disk(cfg["disk_file"])
    if not disk:
        raise SystemExit("G0 KIT: disk %s not found under /w/game" % cfg["disk_file"])
    tmp = os.path.join(W, "out", "port", "g0")
    os.makedirs(tmp, exist_ok=True)
    img = os.path.join(tmp, "volume.adf")
    open(img, "wb").write(disk_image(disk, cfg["disk_file"]))
    info = adfx.extract(img, os.path.join(tmp, "vol"))
    exe = os.path.join(tmp, "vol", "files", cfg["exe_rel"])
    return exe, info


WHD_SLAVE_ADDR = 0xC7E100          # runtime/whd.c WHD_SLAVE
WHD_EXIT = {1: "the slave quit normally", 2: "the slave aborted", 3: "the game loader returned",
            100: "a packed file needs a decruncher the kit does not link yet",
            101: "a resload call failed with NoError set", 102: "the slave called a resload function the kit lacks"}


def find_slave():
    s = sorted(glob.glob(os.path.join(W, "game", "**", "*.[sS][lL][aA][vV][eE]"), recursive=True))
    return s[0] if s else vol_slave()


def vol_slave():
    """A WHDLoad slave inside an AmigaDOS disk's files (k17, 14 Oct 2026). Measured on A10 Tank Killer: the ADF is a
    WHDLoad install (game.slave + data/), choose() took data/Sim, and run alone that prints "You can run A10 ver 1.5
    by typing 'A10'" and exits. The slave is the way in."""
    mf = os.path.join(W, "evidence", "precook", "manifest.json")
    if not os.path.exists(mf):
        return None
    man = json.load(open(mf))
    for i, d in enumerate(man.get("disks", [])):
        try:
            names = [x[0] for x in json.load(open(os.path.join(W, "evidence", "precook", "disks", str(i), "adf.json"))).get("files", [])]
        except Exception:
            continue
        sl = sorted(n for n in names if n.lower().endswith(".slave"))
        if sl:
            exe, _ = extract({"disk_file": d["file"], "exe_rel": sl[0]})   # the whole volume, so its data files sit beside it
            return exe if os.path.exists(exe) else None
    return None


def slave_header(path):
    """(first code hunk bytes, BaseMem, ExpMem wanted or 0) of a WHDLoad slave, read as runtime/whd.c reads it."""
    b = open(path, "rb").read()
    be = lambda o: struct.unpack_from(">I", b, o)[0]
    o = 4
    while be(o):
        o += 4 + 4 * be(o)
    o += 4
    first, last = be(o + 4), be(o + 8)
    o += 12 + 4 * (last - first + 1)
    clen = be(o + 4) * 4
    code = b[o + 8:o + 8 + clen]
    ver = struct.unpack_from(">H", code, 12)[0]
    basemem = struct.unpack_from(">I", code, 16)[0]
    exp = abs(struct.unpack_from(">i", code, 32)[0]) if ver >= 8 else 0
    return clen, basemem, exp


def whd_model(path):
    """The memory model a WHDLoad slave needs (k18, 14 Oct 2026): chip 512/1024/2048 KB to hold BaseMem, slow RAM
    512/1024/1536 KB to hold ExpMem, the tower and the slave. Measured over the 40 pre-port G1 failures: BaseMem up to
    $200000 and ExpMem up to $C0000 stopped about 15 WHDLoad installs at 'larger than the 512 KB chip RAM model'."""
    clen, basemem, exp = slave_header(path)
    chip = next((k for k in (512, 1024, 2048) if basemem <= k * 1024), 2048)
    fits512 = exp <= 0x7E000 and WHD_SLAVE_ADDR + clen <= 0xC80000          # the fixed 512 KB layout
    slow = 512 if fits512 else next((k for k in (1024, 1536) if exp + 0x1200 + clen <= k * 1024), 1536)
    return {"chip_kb": chip, "slow_kb": slow, "slave_addr": whd_slave_addr(clen, slow), "slave_bytes": clen}


def whd_slave_addr(clen, slow_kb):
    """Where runtime/whd.c whd_layout() puts the slave (keep the two the same)."""
    top = 0xC00000 + slow_kb * 1024
    if slow_kb == 512 and WHD_SLAVE_ADDR + clen <= top:
        return WHD_SLAVE_ADDR
    return (top - clen) & ~0xFF


def slave_image(path, addr=None):
    return hunkload.load(open(path, "rb").read(), base=addr or WHD_SLAVE_ADDR)


def make_whd(slave):
    rel = os.path.relpath(slave, W)
    model = whd_model(slave)
    env = dict(os.environ, AMIGO_W=W, WHD_CHIP_KB=str(model["chip_kb"]), WHD_SLOW_KB=str(model["slow_kb"]))
    r = subprocess.run(["bash", os.path.join(HERE, "amigo_port.sh"), "whdrun", slave, "1500"], env=env,
                       capture_output=True, text=True)
    G = os.path.join(W, "out", "port", "whd")
    log = open(os.path.join(G, "whd.log"), errors="replace").read() if os.path.exists(os.path.join(G, "whd.log")) else ""
    if "kickemu slave" in log:
        print("G0 KIT: %s is a kickemu slave (it boots a Kickstart); the kit's OS-call shim is not built yet: left to the model" % rel)
        sys.exit(2)
    m = re.search(r"loader \$([0-9A-F]+)", log)
    ex = re.search(r"^WHD EXIT: (\d+)", log, re.M)
    code = int(ex.group(1)) if ex else -1
    if not m or code in WHD_EXIT and code >= 100:
        why = WHD_EXIT.get(code, "the slave did not boot")
        detail = "; ".join(l for l in log.splitlines() if l.startswith(("WHD NOT", "WHD NEED", "WHD ABORT", "WHD: ")))[-400:]
        print("G0 KIT: WHDLoad slave %s: %s (%s): left to the model" % (rel, why, detail)); sys.exit(2)
    ops = os.path.join(G, "executed_ops.txt")
    pcs = [int(l.split()[0], 16) for l in open(ops) if l.strip()] if os.path.exists(ops) else []
    zero = sum(1 for l in open(ops) if len(l.split()) > 1 and l.split()[1] == "0000") if pcs else 0
    if not pcs or len(pcs) > 150000 or zero * 2 > len(pcs):
        print("G0 KIT: WHDLoad slave %s ran astray on the harness (%d addresses, %d zero words): left to the model" % (rel, len(pcs), zero))
        sys.exit(2)
    cfg = {"whdload": rel, "slave_addr": model["slave_addr"], "chip_kb": model["chip_kb"], "slow_kb": model["slow_kb"]}
    WHD_ADDR = model["slave_addr"]
    os.makedirs(os.path.dirname(CFG), exist_ok=True)
    json.dump(cfg, open(CFG, "w"), indent=1)
    res = rebuild(cfg)
    entry = int(m.group(1), 16)
    base_mem = int(re.search(r"BaseMem \$([0-9A-F]+)", log).group(1), 16)
    sh = os.path.join(W, "gates", "G0", "rebuild_image.sh")
    open(sh, "w").write("#!/bin/bash\n# Made by Ami-Go's port kit (kit/amigo/port/portg0.py): loads the WHDLoad slave %s\n"
                        "# with hunkload at $%06X (where runtime/whd.c puts it) into /w/out/image.bin.\n"
                        "set -euo pipefail\ncd /w\npython3 kit/amigo/port/portg0.py rebuild\n" % (rel, WHD_ADDR))
    os.chmod(sh, 0o755)
    slave_end = WHD_ADDR + len(res["image"])
    outside = [a for a in pcs if not WHD_ADDR <= a < slave_end]
    rr = ranges(outside)
    calls = re.search(r"^WHD CALLS: (.*)$", log, re.M)
    hashes = os.path.join(G, "hash.txt")
    pics = len({l.split()[1] for l in open(hashes) if len(l.split()) > 1}) if os.path.exists(hashes) else 0
    lines = ["# G0 memory map (written by Ami-Go's port kit from measurements; edit freely)", "",
             "BASE: $%06X" % WHD_ADDR, "ENTRY: $%06X" % entry, "",
             "REGION: $%06X-$%06X code WHDLoad slave %s (its loader and patches)" % (WHD_ADDR, slave_end, os.path.basename(rel)),
             "REGION: $000000-$%06X data BaseMem, the game's memory, filled by the slave through resload at run time" % base_mem]
    lines += ["REGION: $%06X-$%06X unpacked game code the slave loaded here at run time (seen running in the harness)" % (a, b)
              for a, b in rr[:16]]
    prose = ["", "## How the game is laid out (facts measured by code)", "",
             "The game is a WHDLoad install: %s is a WHDLoad slave, a small program that loads the game's files and "
             "patches them, calling WHDLoad's resload functions. The port boots it with the kit's own C resload "
             "(runtime/whd.c, no WHDLoad and no Kickstart): the slave sits at $%06X, the game's memory (BaseMem) is $0-$%06X, "
             "and the slave's game loader at ENTRY is entered in supervisor mode with A0 pointing at the resload table." % (
                 rel, WHD_ADDR, base_mem),
             "Running it for 1500 frames on the analysis harness executed %d instruction addresses (%d outside the slave, in "
             "%d range(s)) and showed %d different pictures. Resload calls made: %s. The game code itself is loaded at run "
             "time, so the translator reads it from the harness's memory after the run (game.json translate_only and "
             "out/port/ref/ram_chip.bin), and game.json boots through whd.c with every file of the install embedded." % (
                 len(pcs), len(outside), len(rr), pics, calls.group(1) if calls else "none")]
    open(os.path.join(W, "gates", "G0", "MEMMAP.md"), "w").write("\n".join(lines + prose) + "\n")
    print("G0 KIT: WHDLoad slave %s boots on the C resload: loader $%06X, BaseMem $%X, %d executed addresses, %d pictures; %s" % (
        rel, entry, base_mem, len(pcs), pics, (r.stdout or "").strip().splitlines()[-1:] ))


def rebuild(cfg):
    if cfg.get("whdload"):
        res = slave_image(os.path.join(W, cfg["whdload"]), cfg.get("slave_addr"))
        open(os.path.join(W, "out", "image.bin"), "wb").write(res["image"])
        return res
    exe, info = extract(cfg)
    # oshle.py sets seglist (k17). Its fake OS sits at $0-$4000, so chip hunks start right above it: measured 14 Oct on
    # 3D World Tennis, the image at $20000 split free chip RAM in two and its AllocMem of 370 KB chip failed
    res = hunkload.load(open(exe, "rb").read(), seglist=bool(cfg.get("seglist")),
                        chip_base=0x4000 if cfg.get("seglist") else 0x20000)
    open(os.path.join(W, "out", "image.bin"), "wb").write(res["image"])
    return res, info


SYSTEM_DIRS = ("c/", "s/", "l/", "libs/", "devs/", "system/", "utilities/", "prefs/", "fonts/")
COMMON_CMDS = {"run", "if", "else", "endif", "echo", "addmem", "addbuffers", "setpatch", "loadwb", "endcli", "cd", "assign",
               "copy", "delete", "path", "wait", "ask", "skip", "lab", "failat", "version", "stack", "makedir", "mount",
               "resident", "setmap", "setclock", "binddrivers", "type", "execute", "newshell", "newcli", "info", "dir",
               "list", "rename", "protect", "date", "quit", "why", "fault", "prompt", "noclick", "click", "reset",
               "closeworkbench", "bootwriter", "virusx", "virus_checker", "nofastmem", "fastmemfirst"}


def choose(man):
    """The game's main program (k17, 14 Oct 2026; was: the most code Ghidra could follow).
    Measured 13 Oct on 3D Pool: first-in-startup-order picked a 1.5 KB protection loader (linkprot, the only program the
    startup-sequence names), and largest-file picked a 330 KB picture file in hunk form; the game (3d-pool) has the most
    reachable instructions. Measured 14 Oct over the 40 pre-port G1 failures, most-code alone picked: a virus checker
    in s/ (Airborne Ranger), a RAM test the startup-sequence runs first (HardBall!), PowerPacker's picture viewer when the
    game is a Workbench icon (Karate Master), and a module the launcher LoadSegs (Gunship: scrnmule). So:
      1. never C:/S:/L:/Libs:/Devs: programs; 2. a startup-sequence that runs LoadWB with a program 4x larger than any
      it runs: that program (a Workbench game; most code, or a crunched program 4x larger); 3. the programs the startup-sequence runs (not AmigaDOS commands): the one
      with the most code, or the largest if all are small (crunched: a few hundred instructions of decruncher), when that
      one has 1000+ instructions or nothing on the disk has; 4. otherwise the most code, as before."""
    cmds, loadwb = [], False
    for i, d in enumerate(man.get("disks", [])):
        try:
            info = json.load(open(os.path.join(W, "evidence", "precook", "disks", str(i), "adf.json")))
        except Exception:
            continue
        for line in info.get("startup", []):
            ws = [w_ for w_ in re.findall(r"[\w.\-/:]+", line.split(";")[0]) if not w_.startswith(">")]
            while ws and ws[0].lower() in ("run", "execute"):
                ws = ws[1:]
            if ws:
                c = re.split(r"[:/]", ws[0])[-1].lower()
                loadwb |= c == "loadwb"
                if c not in COMMON_CMDS:
                    cmds.append(c)
    base = lambda e: os.path.basename(e["name"].split(":", 1)[1]).lower()
    rel = lambda e: e["name"].split(":", 1)[1].lower()
    allx = [e for e in man.get("exes", []) if e.get("name", "").startswith("disk") and ":" in e["name"]
            and not e["name"].lower().endswith(SKIP_SUFFIX) and not rel(e).startswith(SYSTEM_DIRS)]
    cands = [e for e in allx if not e.get("error")]
    ins = lambda e: e.get("instructions") or 0
    named = [e for e in allx if base(e) in cmds and e.get("size", 0) >= 1024]   # not 228-byte "nomem" tools
    if loadwb and cands:
        # the Workbench program: the most code; a crunched program (a few hundred instructions of decruncher in a file
        # 4x larger) beats it, data in hunk form (a sound: 0-1 instructions) never does. Measured on Karate Master
        # (crunched KarateMaster), Amiga Karate (not spdata), Marble Madness (not UltSnd), Mavis Beacon
        unnamed = [e for e in cands if base(e) not in cmds]
        wb = max(unnamed, key=lambda e: (ins(e), e.get("size", 0)), default=None)
        big = max(unnamed, key=lambda e: e.get("size", 0), default=None)
        if wb and big and big.get("size", 0) >= 4 * wb.get("size", 0) and ins(big) >= 100:
            wb = big
        if wb and wb.get("size", 0) >= 4 * max([e.get("size", 0) for e in named] or [1]):
            return wb
    if named:
        small = all(ins(e) < 1000 for e in named)
        best = max(named, key=lambda e: (e.get("size", 0),) if small else (ins(e), e.get("size", 0)))
        if ins(best) >= 1000 or all(ins(e) < 1000 for e in cands):
            return best
    return max(cands, key=lambda e: (ins(e), e.get("size", 0))) if cands else None


def ranges(pcs, gap=0x1000):
    out = []
    for a in sorted(pcs):
        if out and a - out[-1][1] <= gap:
            out[-1][1] = a + 2
        else:
            out.append([a, a + 2])
    return out


def make():
    mf = os.path.join(W, "evidence", "precook", "manifest.json")
    if not os.path.exists(mf) and find_slave():
        make_whd(find_slave()); return
    if not os.path.exists(mf):
        print("G0 KIT: no library pre-analysis for this game (evidence/precook): left to the model"); sys.exit(2)
    man = json.load(open(mf))
    e = choose(man)
    slave = find_slave()
    if slave and (not e or "--whd" in sys.argv or "/out/port/g0/vol/" in slave):
        make_whd(slave); return
    if not e:
        print("G0 KIT: no AmigaDOS program in the pre-analysis (custom trackloader or WHDLoad-only): left to the model")
        sys.exit(2)
    dn, rel = e["name"].split(":", 1)
    d = man["disks"][int(dn[4:])]
    cfg = {"disk_file": d["file"], "exe_rel": rel, "exe_name": e["name"], "volume": d.get("volume", "")}
    os.makedirs(os.path.dirname(CFG), exist_ok=True)
    json.dump(cfg, open(CFG, "w"), indent=1)
    res, info = rebuild(cfg)
    sh = os.path.join(W, "gates", "G0", "rebuild_image.sh")
    open(sh, "w").write("#!/bin/bash\n# Made by Ami-Go's port kit (kit/amigo/port/portg0.py): extracts %s from %s and loads it\n"
                        "# with hunkload (LoadSeg's layout and relocations) into /w/out/image.bin.\n"
                        "set -euo pipefail\ncd /w\npython3 kit/amigo/port/portg0.py rebuild\n" % (rel, d["file"]))
    os.chmod(sh, 0o755)
    regions = list(zip(res["addr"], res["sizes"], res["kinds"], res["mem"]))
    lines = ["# G0 memory map (written by Ami-Go's port kit from measurements; edit freely)", "",
             "BASE: $%06X" % res["base"], "ENTRY: $%06X" % res["entry"], ""]
    lines += ["REGION: $%06X-$%06X %s hunk %d (%s memory, %d bytes)" % (a, a + s, "code" if k == "code" else ("bss" if k == "bss" else "data"), j, m, s)
              for j, (a, s, k, m) in enumerate(regions)]
    if len([1 for l in lines if l.startswith("REGION:")]) < 2:
        lines.append("REGION: $%06X-$%06X bss end marker (the program has a single hunk; no further hunks)" % (
            res["base"] + len(res["image"]), res["base"] + len(res["image"]) + 4))
    startup = [l for l in info.get("startup", []) if l.strip()][:8]
    others = [x["name"] for x in man.get("exes", []) if x.get("name") != e["name"]][:8]
    prose = ["", "## How the game is laid out (facts measured by code)", "",
             "The game ships on an AmigaDOS volume named '%s' (%s). Its startup-sequence runs: %s." % (
                 cfg["volume"] or "?", d["file"], "; ".join(startup) or "no startup-sequence was found"),
             "The program chosen as the game is %s, an AmigaDOS hunk executable of %d bytes with %d hunks and %d "
             "relocations. AmigaDOS LoadSeg lays its hunks out one after another, hunks flagged for chip memory in chip "
             "RAM, and applies the relocations; hunkload.py does the same, so out/image.bin is the program as it sits in "
             "memory at its first instruction, ENTRY, which is the start of the first hunk." % (
                 rel, e.get("size", 0), len(regions), res["relocs"]),
             "Ghidra found %d functions and %d instructions reachable from ENTRY in the library pre-analysis." % (
                 e.get("functions", 0), e.get("instructions", 0)),
             ("Other programs on the disk, not chosen: %s." % ", ".join(others)) if others else ""]
    open(os.path.join(W, "gates", "G0", "MEMMAP.md"), "w").write("\n".join(lines + prose) + "\n")
    # measure what the program does after it starts: run the original code on the harness and look for code
    # executed outside the loaded hunks (a stub that copies or unpacks the program elsewhere)
    env = dict(os.environ, AMIGO_W=W)
    subprocess.run(["bash", os.path.join(HERE, "amigo_port.sh"), "init"], env=env, capture_output=True)
    r = subprocess.run(["bash", os.path.join(HERE, "amigo_port.sh"), "harness", "300"], env=env, capture_output=True, text=True)
    ops = os.path.join(W, "out", "port", "ref", "executed_ops.txt")
    extra = []
    if os.path.exists(ops):
        inside = lambda a: any(x <= a < x + s for x, s, k, m in regions)
        pcs = [int(l.split()[0], 16) for l in open(ops) if l.strip() and not inside(int(l.split()[0], 16))]
        pcs = [a for a in pcs if a < 0xF80000]           # not the Kickstart ROM area
        extra = ranges(pcs)
        if len(pcs) > 40000 or len(extra) > 12:
            # far more than a game's code: the run went astray (wrong program, missing OS call), so it proves nothing
            print("G0 KIT: harness run not conclusive (%d addresses outside the hunks in %d ranges); no run-time regions claimed" % (len(pcs), len(extra)))
            extra = []
    if extra:
        add = ["REGION: $%06X-$%06X unpacked code the program copied or unpacked here at run time (seen running in the harness)" % (a, b)
               for a, b in extra[:12]]
        txt = open(os.path.join(W, "gates", "G0", "MEMMAP.md")).read()
        txt = txt.replace("\n\n## How the game", "\n" + "\n".join(add) + "\n\n## How the game", 1)
        txt += ("\nAt run time the program does not stay where LoadSeg put it: running the original code for 300 frames "
                "on the analysis harness executed %d instruction addresses outside the loaded hunks, in %d range(s) "
                "starting at $%06X. So the loaded image contains a stub or loader that copies or unpacks the real game "
                "code there; the port kit translates that code from the harness's memory once it has run.\n" % (
                    sum(1 for _ in pcs), len(extra), extra[0][0]))
        open(os.path.join(W, "gates", "G0", "MEMMAP.md"), "w").write(txt)
    print("G0 KIT: %s loaded at $%06X entry $%06X, %d hunks, %d relocs; %d run-time code range(s); harness: %s" % (
        rel, res["base"], res["entry"], len(regions), res["relocs"], len(extra), (r.stdout or r.stderr).strip().splitlines()[-1:] ))


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "rebuild":
        rebuild(json.load(open(CFG)))
    else:
        make()
