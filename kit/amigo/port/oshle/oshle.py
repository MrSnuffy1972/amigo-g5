#!/usr/bin/env python3
"""oshle.py - give a hunk-program game a small AmigaOS on the harness, the port and difftest (k17, 14 Oct 2026).

Measured 14 Oct: all 40 library games whose pre-port passed G0 and failed G1 failed the same way. The program
calls the OS (exec AllocMem, dos Open/Read, intuition OpenScreen ...), the harness has no Kickstart, the call jumps
into nothing and the CPU runs off through empty memory (g1sane: "ran astray"). Claude fixed one of them by hand
(4th & Inches, gates/G1/hooks.c + mkexec.py); this makes that fix the kit's, for every game:

  * the program is laid out with real LoadSeg headers (hunkload seglist=True; g0.json "seglist": true)
  * gates/G1/execlib.bin at $0-$4000: AbsExecBase, a fake library base with an RTS jump table for each library
    in LIBS, a fake Process with a CLI (pr_CLI, cli_Module = the program's segment list), BRA.S * at $3FF0
  * every jump-table slot is an 'hle' entry calling hle_lib in kit/amigo/port/oshle/oshle.c
  * gates/G1/oshle_cfg.c: the boot disk's name and the free chip/slow RAM for AllocMem (outside the image)
  * game.json files: the boot disk image and every file of its volume (dos Open finds them by name)
usage: oshle.py [--check]        (in the work folder, /w; AMIGO_W overrides). Writes game.json.oshle-backup first.
WHDLoad games are left alone: they boot through the kit's C resload instead.
"""
import glob, json, os, re, shutil, struct, sys

W = os.environ.get("AMIGO_W", "/w")
os.environ["AMIGO_W"] = W
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
sys.path.insert(0, os.path.dirname(os.path.dirname(HERE)))
# slot counts cover every function in the NDK .fd files (lvo.json, from amitools 0.8.1's data/fd): with fewer slots a
# call to a deeper vector landed in zeros below the table and slid up into its last slot, so the log named the wrong
# function (measured 14 Oct: Karate Master "dos -$120" 25,083 times, icon "-$C0" 11 times; neither exists)
LIBS = [("exec.library", 0xC00, 140), ("dos.library", 0x1400, 170), ("graphics.library", 0x1C00, 180),
        ("intuition.library", 0x2400, 140), ("icon.library", 0x2C00, 28), ("trackdisk.device", 0x3400, 16),
        ("disk.resource", 0x3800, 16), ("ciaa.resource", 0x3C00, 16),
        ("mathffp.library", 0x3000, 16), ("dummy.device", 0x3200, 16),
        ("layers.library", 0x3600, 40)]   # Cowork 15 Oct: Above Top Secret, Genesia   # ROM in Kickstart 1.3 (HardBall! Alerts AO_MathLib without it)   # oshle_cfg.c carries them to oshle.c
TASK, CLI, EXIT_PC = 0xF00, 0x3F00, 0x3FF0
STUB = 0x3E00
SSP = 0x7C000      # supervisor stack top: $7A000-$7C000 is kept out of AllocMem; the user stack is the boot A7 (above)
# level 3 (VERTB) handler at $3E40: calls each AddIntServer'd server in the list at $3EC0 (A1 is_Data, A5 is_Code,
# A0 custom, A6 exec), clears the level 3 requests, RTE. Other autovectors get $3E70: read both CIA ICRs, clear the
# other requests, RTE. Without them an enabled interrupt jumped through a zero vector into the fake OS tables.
L3 = 0x3E40
L3_CODE = bytes.fromhex("48E7FFFE 49F83EC0 201C 6714 225C 2440 2A40 41F900DFF000 2C780004 4E92 60E8 33FC007000DFF09C 4CDF7FFF 4E73".replace(" ", ""))
LX = 0x3E70
LX_CODE = bytes.fromhex("4A3900BFED01 4A3900BFDD00 33FC3F8F00DFF09C 4E73".replace(" ", ""))
STUB_CODE = bytes.fromhex("48E77FFE 20383EF0 20783EF4 22783EF8 2C780004 4E91 4A80 6706 2C40 4EAEFFFA 4CDF7FFE 4E75".replace(" ", ""))


def execlib():
    m = bytearray(0x4000)
    struct.pack_into(">I", m, 0, SSP)                         # the supervisor stack (reset vector; the program runs in user mode)
    struct.pack_into(">I", m, 4, 0xC00)                       # AbsExecBase
    for name, base, n in LIBS:
        for i in range(1, n + 1):
            struct.pack_into(">HHH", m, base - 6 * i, 0x4E75, 0x4E71, 0x4E71)   # RTS NOP NOP: decodes, HLE'd
        struct.pack_into(">H", m, base + 0x14, 34)            # lib_Version (Kickstart 1.3)
        nm = base + 0x80; m[nm:nm + len(name)] = name.encode()
        struct.pack_into(">I", m, base + 0x0A, nm)            # ln_Name
    struct.pack_into(">H", m, 0xC00 + 0x212, 50)              # VBlankFrequency (PAL)
    struct.pack_into(">I", m, 0xC00 + 0x114, TASK)            # ThisTask
    m[TASK + 8] = 13                                          # NT_PROCESS
    struct.pack_into(">I", m, TASK + 0xAC, CLI >> 2)          # pr_CLI: started from the CLI (not Workbench)
    struct.pack_into(">H", m, EXIT_PC, 0x60FE)                # BRA.S *: where a program's final RTS lands
    m[STUB:STUB + len(STUB_CODE)] = STUB_CODE                 # runs a disk library's init and Open (oshle.c disklib_open)
    m[L3:L3 + len(L3_CODE)] = L3_CODE; m[LX:LX + len(LX_CODE)] = LX_CODE
    for v in (0x64, 0x68, 0x70, 0x74, 0x78):                 # autovectors 1, 2, 4, 5, 6 -> the default handler
        struct.pack_into(">I", m, v, LX)
    struct.pack_into(">I", m, 0x6C, L3)                       # level 3 -> the VERTB server runner
    return bytes(m)


def libs_c():
    """The library table and function names for oshle.c (one source: LIBS above and lvo.json)."""
    names = json.load(open(os.path.join(HERE, "lvo.json")))
    L = ["typedef struct { const char *name; unsigned base, slots; } oshle_lib_t;",
         "const oshle_lib_t oshle_libs[] = {"] + ['  {"%s", 0x%X, %d},' % (n, b, s) for n, b, s in LIBS] + \
        ["};", "const int oshle_nlibs = %d;" % len(LIBS), "typedef struct { unsigned base; int lvo; const char *fn; } oshle_name_t;",
         "const oshle_name_t oshle_names[] = {"]
    for n, b, s in LIBS:
        for bias, fn in sorted(names.get(n.split(".")[0], {}).items(), key=lambda x: int(x[0])):
            L.append('  {0x%X, -%d, "%s"},' % (b, int(bias), fn))
    L += ["  {0, 0, 0}", "};"]
    return "\n".join(L) + "\n"


def startup_args(cfg):
    """The arguments the startup-sequence gives the chosen program ("pp lab" -> "lab"), with the newline AmigaDOS
    adds. Measured 14 Oct on Karate Master: the program is a PowerPacker runner started as 'pp lab'; with no
    arguments it has nothing to run."""
    exe = os.path.basename(cfg.get("exe_rel", "")).lower()
    for f in sorted(glob.glob(os.path.join(W, "evidence", "precook", "disks", "*", "adf.json"))):
        try: lines = json.load(open(f)).get("startup", [])
        except Exception: continue
        for l in lines:
            w = l.split(";")[0].split()
            if w and re.split(r"[:/]", w[0])[-1].lower() == exe:
                return " ".join(w[1:]) + "\n"
    # not started by the startup-sequence: the disk ends with LoadWB/EndCLI and the game is a Workbench icon.
    # Measured 14 Oct on Amiga Karate: its startup code tests pr_CLI and returns at once when started from a CLI.
    return None


def multidisk(adf):
    """Method B (Cowork 15 Oct 2026): the files and image of every disk of a multi-disk AmigaDOS game, so the port
    never asks for a disk. Disk k>0 files go to out/port/g0/vold<k>/files; a name the boot disk (or an earlier
    disk) already has is also written as d<k>~<name>, which oshle.c tries first when the path names that disk's
    volume. Returns (volumes, adf basenames) in disk order, boot disk first."""
    import portg0, adfx
    mf = os.path.join(W, "evidence", "precook", "manifest.json")
    if not os.path.exists(mf):
        return [], []
    disks = [d for d in json.load(open(mf)).get("disks", []) if d.get("file")]
    if len(disks) < 2:
        return [], []
    boot = [d for d in disks if os.path.basename(d.get("file", "")) == adf] or disks[:1]
    order = boot + [d for d in disks if d is not boot[0]]
    vols, adfs, seen = [], [], set()
    vol0 = os.path.join(W, "out", "port", "g0", "vol", "files")
    for dp, dn, fn in os.walk(vol0):
        seen.update(f.lower() for f in fn)
    for k, d in enumerate(order):
        vols.append(d.get("volume") or ""); adfs.append(os.path.basename(d["file"]))
        if k == 0 or not d.get("dos"):
            continue
        out = os.path.join(W, "out", "port", "g0", "vold%d" % k)
        img = os.path.join(out, "volume.adf"); os.makedirs(out, exist_ok=True)
        disk = portg0.find_disk(d["file"])
        if not disk:
            print("OSHLE multi-disk: disk %d %s not found" % (k, d["file"])); continue
        open(img, "wb").write(portg0.disk_image(disk, d["file"]))
        adfx.extract(img, out)
        dup = os.path.join(out, "dup"); os.makedirs(dup, exist_ok=True)
        mine = set()
        for dp, dn, fn in os.walk(os.path.join(out, "files")):
            for f in fn:
                if f.lower() in seen:
                    shutil.copy2(os.path.join(dp, f), os.path.join(dup, "d%d~%s" % (k, f)))
                mine.add(f.lower())
        seen |= mine
    return vols, adfs


def multidisk_c(vols, adfs):
    q = lambda xs: ", ".join(json.dumps(x) for x in xs) or "0"
    return ("const int oshle_ndisks = %d;\nconst char *const oshle_vols[] = {%s};\nconst char *const oshle_adfs[] = {%s};"
            % (len(vols), q(vols), q(adfs)))


def main():
    import portg0
    g0p = os.path.join(W, "gates", "G0", "g0.json")
    cfg = json.load(open(g0p))
    if cfg.get("whdload"):
        print("OSHLE: WHDLoad game, not needed (the C resload is its OS)"); return 0
    for f in glob.glob(os.path.join(W, "evidence", "precook", "disks", "*", "adf.json")):
        try: names = [x[0] for x in json.load(open(f)).get("files", [])]
        except Exception: continue
        if any(n.lower().endswith(".slave") for n in names):
            # measured 14 Oct on A10 Tank Killer: G0 chose data/Sim from a WHDLoad install; run on its own it prints
            # "You can run A10 ver 1.5 by typing 'A10'" and exits. The slave is the way in (portg0.py --whd)
            print("OSHLE: a WHDLoad install (%s on the volume): use the slave (portg0.py make --whd), not oshle" %
                  [n for n in names if n.lower().endswith(".slave")][0]); return 3
    cfg["seglist"] = True
    # Cowork 15 Oct: OS games size memory with AvailMem and quit on 1 MB (measured: Nigel Mansell, K240 "Insufficient
    # free memory"); give them a 1.5 MB trapdoor (slow RAM to $D80000, amigo_port.sh memdefs reads slow_kb)
    cfg["slow_kb"] = max(int(cfg.get("slow_kb") or 512), 1536)
    json.dump(cfg, open(g0p, "w"), indent=1)
    res, _info = portg0.rebuild(cfg)
    base, size = res["base"], len(res["image"])
    end = (base + size + 0xFFF) & ~0xFFF
    in_chip = base < 0x80000
    chip_lo, chip_hi = (max(0x4000, end) if in_chip else 0x4000), SSP - 0x2000
    slow_lo, slow_hi = (max(0xC00000, end) if not in_chip else 0xC00000), 0xC00000 + cfg["slow_kb"] * 1024
    if True:   # Cowork 15 Oct: always (was: under 256 KB left); games size chip with AvailMem(MEMF_CHIP)
        # Cowork 15 Oct: less than 256 KB of chip left beside the image -> a 1 MB chip, AllocMem from its upper half
        cfg["chip_kb"] = max(int(cfg.get("chip_kb") or 512), 1024)
        json.dump(cfg, open(g0p, "w"), indent=1)
        chip_lo, chip_hi = max(0x80000, end if in_chip else 0x80000), 0x100000
    if chip_lo >= chip_hi:
        print("OSHLE: the image fills chip RAM ($%06X-$%06X); no room for AllocMem" % (base, base + size)); return 1
    g1 = os.path.join(W, "gates", "G1"); os.makedirs(g1, exist_ok=True)
    open(os.path.join(g1, "execlib.bin"), "wb").write(execlib())
    adf = os.path.basename(cfg.get("disk_file", "")) or ""
    args = startup_args(cfg)
    md_vols, md_adfs = multidisk(adf)
    open(os.path.join(g1, "oshle_cfg.c"), "w").write(
        "/* written by kit/amigo/port/oshle/oshle.py: this game's numbers for oshle.c */\n#include <stdint.h>\n"
        "const char *oshle_adf = %s;\nconst uint32_t oshle_chip_lo = 0x%X, oshle_chip_hi = 0x%X;\n"
        "const uint32_t oshle_slow_lo = 0x%X, oshle_slow_hi = 0x%X;\nconst uint32_t oshle_seglist = 0x%X;\nconst char *oshle_args = %s;\nconst int oshle_wb = %d;\n"
        "const char *oshle_exe = %s;\n"
        % (json.dumps(adf) if adf else "0", chip_lo, chip_hi, slow_lo, slow_hi, res.get("seglist") or 0,
           json.dumps(args if args is not None else ""), int(args is None),
           json.dumps(os.path.basename(cfg.get("exe_rel", "game")))) + multidisk_c(md_vols, md_adfs) + "\n" + libs_c())
    gj = os.path.join(W, "game.json")
    g = json.load(open(gj)) if os.path.exists(gj) else {"name": os.environ.get("AMIGO_GAME", "game")}
    if not os.path.exists(gj + ".oshle-backup") and os.path.exists(gj):
        shutil.copy2(gj, gj + ".oshle-backup")
    code = ["0x%X-0x%X" % (a, a + s) for a, s, k in zip(res["addr"], res["sizes"], res["kinds"]) if k == "code"]
    code += ["0x%X-0x%X" % (b - 6 * n, b) for _, b, n in LIBS] + ["0x%X-0x%X" % (EXIT_PC, EXIT_PC + 2), "0x%X-0x%X" % (STUB, STUB + len(STUB_CODE)),
             "0x%X-0x%X" % (L3, L3 + len(L3_CODE)), "0x%X-0x%X" % (LX, LX + len(LX_CODE))]
    g["segments"] = [{"file": "gates/G1/execlib.bin", "addr": "0x0"}, {"file": "out/image.bin", "addr": "0x%X" % base}]
    g["entry"] = "0x%X" % res["entry"]
    g["code"] = code
    g["hle"] = dict(g.get("hle") or {}, **{"0x%X" % (b - 6 * i): {"fn": "hle_lib", "cycles": 0} for _, b, n in LIBS for i in range(1, n + 1)})
    hooks = [h for h in (g.get("hooks_c") or []) if "oshle" not in h]
    g["hooks_c"] = hooks + ["kit/amigo/port/oshle/oshle.c", "gates/G1/oshle_cfg.c"]
    g["boot_extra"] = True
    g["ghidra"] = False                 # Ghidra's addresses assume the layout without LoadSeg headers
    g["seed_addrs"] = sorted(set((g.get("seed_addrs") or []) + ["0x%X" % res["entry"]]))
    files = set(g.get("files") or [])
    vol = os.path.join(W, "out", "port", "g0", "vol", "files")
    for dp, dn, fn in os.walk(vol):
        if fn:
            files.add(os.path.relpath(os.path.join(dp, "*"), W))
    for f in glob.glob(os.path.join(W, "game", "**", glob.escape(adf)), recursive=True)[:1] if adf else []:
        files.add(os.path.relpath(f, W))
    for k in range(1, len(md_vols)):                    # Method B: the other disks' files, duplicates and images
        for sub_ in ("files", "dup"):
            for dp, dn, fn in os.walk(os.path.join(W, "out", "port", "g0", "vold%d" % k, sub_)):
                if fn:
                    files.add(os.path.relpath(os.path.join(dp, "*"), W))
        for f in glob.glob(os.path.join(W, "game", "**", glob.escape(md_adfs[k])), recursive=True)[:1]:
            files.add(os.path.relpath(f, W))
    g["files"] = sorted(files)
    g["_oshle"] = "k17 14 Oct 2026: kit/amigo/port/oshle (fake exec/dos/intuition + LoadSeg headers); backup game.json.oshle-backup"
    json.dump(g, open(gj, "w"), indent=1)
    print("OSHLE: image $%06X-$%06X (%s, %d hunks, seglist BPTR $%X); AllocMem chip $%06X-$%06X slow $%06X-$%06X; "
          "%d HLE slots; %d file patterns; boot disk %s" % (base, base + size, "chip" if in_chip else "slow", len(res["addr"]),
          res.get("seglist") or 0, chip_lo, chip_hi, slow_lo, slow_hi, len(g["hle"]), len(g["files"]), adf or "none")
          + ("; %d disks embedded (Method B)" % len(md_vols) if md_vols else "") + ("; Workbench start" if args is None else "; CLI start, arguments %r" % args.strip()))
    return 0


if __name__ == "__main__":
    sys.exit(main())
