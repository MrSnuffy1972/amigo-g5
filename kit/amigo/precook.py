#!/usr/bin/env python3
"""precook.py: pre-analyse one game with Ghidra before anyone works on it (runs INSIDE the tools container).

Karl, 13 Oct 2026: run the Ghidra process against every Amiga game in the library so the output is ready the
moment a game is added to the queue. Everything here is code (rungs 2-3): disk extraction, hunk loading and
Ghidra's own analysis. No model is asked anything.

Usage: python3 precook.py GAMEDIR OUTDIR
  GAMEDIR holds the unpacked game (ADFs, WHDLoad folders, loose files); OUTDIR receives:
    manifest.json            what was found and what was analysed
    SUMMARY.md               the same, for a model to read first
    disks/<n>/adf.json       per-disk file list and startup-sequence (DOS disks)
    exe/<name>/hunks.txt     hunkload's BASE/ENTRY/REGION lines
    exe/<name>/functions.txt and listing.txt.gz   Ghidra output for each main executable
    boot/<n>/functions.txt and listing.txt.gz     Ghidra output for each disk's bootblock (trackloaders)
Extracted files and memory images are NOT kept (they are game data and are rebuilt in seconds).
"""
import gzip, json, os, re, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import adfx, hunkload  # noqa: E402

SYSTEM_DIRS = {"c", "l", "libs", "devs", "fonts", "system", "prefs", "utilities", "tools", "storage", "locale",
               "classes", "keymaps", "monitors", "printers", "env", "t", "rexx", "expansion"}
MAX_EXES = 12
MAX_BOOT = 4


def ghidra(image, base, entry, out, scratch):
    os.makedirs(out, exist_ok=True)
    proj = os.path.join(scratch, "proj")
    os.makedirs(proj, exist_ok=True)
    cmd = ["timeout", "600", "analyzeHeadless", proj, "p%d" % os.getpid(), "-import", image, "-overwrite",
           "-loader", "BinaryLoader", "-loader-baseAddr", hex(base), "-processor", "68000:BE:32:default",
           "-scriptPath", os.path.join(HERE, "ghidra"), "-postScript", "AmigoExport.java", out, "%x" % entry,
           "-deleteProject"]
    t0 = time.time()
    r = subprocess.run(cmd, capture_output=True, text=True)
    open(os.path.join(out, "ghidra.log"), "w").write((r.stdout + r.stderr)[-20000:])
    m = re.search(r"AmigoExport: (\d+) functions, (\d+) instructions", r.stdout + r.stderr)
    lst = os.path.join(out, "listing.txt")
    if os.path.exists(lst):
        with open(lst, "rb") as f, gzip.open(lst + ".gz", "wb") as g:
            shutil.copyfileobj(f, g)
        os.replace(lst, os.path.join(scratch, "listing.done"))   # plain copy dropped; the .gz is kept
    return {"ok": bool(m), "functions": int(m.group(1)) if m else 0,
            "instructions": int(m.group(2)) if m else 0, "secs": round(time.time() - t0, 1)}


def is_hunk(path):
    try:
        with open(path, "rb") as f:
            return f.read(4) == b"\x00\x00\x03\xf3"
    except Exception:
        return False


def main(gamedir, out):
    t_start = time.time()
    scratch = os.path.join(out, "scratch_tmp")   # no leading dot: Ghidra refuses path elements starting with "."
    os.makedirs(scratch, exist_ok=True)
    man = {"disks": [], "exes": [], "boot": [], "errors": [], "started": time.time()}
    disks = sorted(os.path.join(dp, f) for dp, dn, fn in os.walk(gamedir) for f in fn
                   if f.lower().endswith(".adf"))
    nfloppy = len(disks)
    # hard-disk images (WHDLoad packs ship as .hdf): each AmigaDOS partition is read like a big floppy
    for dp, dn, fn in os.walk(gamedir):
        for f in sorted(fn):
            if f.lower().endswith(".hdf"):
                hp = os.path.join(dp, f)
                try:
                    for j, (pn, off, ln) in enumerate(adfx.partitions(hp)):
                        img = os.path.join(scratch, "%s.p%d.img" % (re.sub(r"[^\w.\-]+", "_", f), j))
                        with open(hp, "rb") as src, open(img, "wb") as dst:
                            src.seek(off); dst.write(src.read(ln))
                        disks.append(img)
                except Exception as ex:
                    man["errors"].append("%s: %s" % (f, ex))
    startup_names = set()
    candidates = []
    for i, dpath in enumerate(disks):
        xd = os.path.join(scratch, "disk%d" % i)
        try:
            info = adfx.extract(dpath, xd)
        except Exception as ex:
            man["errors"].append("%s: %s" % (os.path.basename(dpath), ex))
            continue
        os.makedirs(os.path.join(out, "disks", str(i)), exist_ok=True)
        shutil.copy(os.path.join(xd, "adf.json"), os.path.join(out, "disks", str(i), "adf.json"))
        man["disks"].append({"n": i, "file": os.path.relpath(dpath, gamedir) if i < nfloppy else os.path.basename(dpath), "dos": info["dos"],
                             "volume": info["volume"], "files": len(info["files"])})
        for line in info.get("startup", []):
            for w in re.findall(r"[\w.\-]+", line.split(";")[0]):
                startup_names.add(w.lower())
        for rel, size in info["files"]:
            fp = os.path.join(xd, "files", rel)
            if is_hunk(fp):
                candidates.append(("disk%d:%s" % (i, rel), fp, size))
        if i < MAX_BOOT and i < nfloppy:
            man["boot"].append({"disk": i, "path": os.path.join(xd, "bootblock.bin"), "dos": info["dos"]})
    # loose executables (WHDLoad installs, files supplied outside disk images)
    for dp, dn, fn in os.walk(gamedir):
        for f in fn:
            fp = os.path.join(dp, f)
            if not f.lower().endswith((".adf", ".zip", ".rar", ".lha", ".lzh", ".7z", ".dms", ".ipf")) and is_hunk(fp):
                candidates.append((os.path.relpath(fp, gamedir), fp, os.path.getsize(fp)))

    def rank(c):
        # game executables first: anything in a system folder (c/, libs/ ...) goes last even when the
        # startup-sequence names it (3D Pool test, 13 Oct: c/Run and c/Type came first because the
        # startup-sequence says "run" and "type")
        name, fp, size = c
        rel = name.split(":")[-1]
        base = os.path.basename(rel).lower()
        top = rel.split("/")[0].lower() if "/" in rel else ""
        return (top in SYSTEM_DIRS, 0 if base in startup_names else 1 if base.endswith(".slave") else 2, -size)

    candidates.sort(key=rank)
    # AmigaDOS commands (c/Run, c/Type ...) are never game code: skipped, counted (measured 13 Oct: they were
    # 4 of 8 executables on 3D Pool and cost 5 s of Ghidra each)
    sysx = [c for c in candidates if rank(c)[0]]
    candidates = [c for c in candidates if not rank(c)[0]]
    man["system_exes_skipped"] = [c[0] for c in sysx]
    man["candidates"] = len(candidates)
    exts = {}
    for dp, dn, fn in os.walk(gamedir):
        for f in fn:
            e = os.path.splitext(f)[1].lower() or "(none)"
            exts[e] = exts.get(e, 0) + 1
    man["file_types"] = exts
    for name, fp, size in candidates[:MAX_EXES]:
        safe = re.sub(r"[^\w.\-]+", "_", name)[:80]
        eo = os.path.join(out, "exe", safe)
        os.makedirs(eo, exist_ok=True)
        try:
            res = hunkload.load(open(fp, "rb").read())
        except Exception as ex:
            man["exes"].append({"name": name, "size": size, "error": "hunkload: %s" % ex})
            continue
        img = os.path.join(scratch, safe + ".bin")
        open(img, "wb").write(res["image"])
        lines = ["BASE: $%06X" % res["base"], "ENTRY: $%06X" % res["entry"]] + [
            "REGION: $%06X-$%06X %s hunk %d (%s memory, %d bytes)" % (a, a + s, k, j, m, s)
            for j, (a, s, k, m) in enumerate(zip(res["addr"], res["sizes"], res["kinds"], res["mem"]))]
        open(os.path.join(eo, "hunks.txt"), "w").write("\n".join(lines) + "\n")
        g = ghidra(img, res["base"], res["entry"], eo, scratch)
        man["exes"].append({"name": name, "size": size, "base": res["base"], "entry": res["entry"],
                            "hunks": len(res["addr"]), "relocs": res["relocs"], "dir": "exe/" + safe, **g})
    for b in man["boot"]:
        bo = os.path.join(out, "boot", str(b["disk"]))
        g = ghidra(b.pop("path"), 0, 0xC, bo, scratch)   # bootblock code starts at offset 12
        b.update(dir="boot/%d" % b["disk"], **g)
    man["secs"] = round(time.time() - t_start, 1)
    json.dump(man, open(os.path.join(out, "manifest.json"), "w"), indent=1)
    md = ["# Pre-analysis of this game (Ghidra, done before the project started)", "",
          "Made by Ami-Go's library pre-analysis (code only; no model). Paths are relative to this folder.", ""]
    for d in man["disks"]:
        md.append("- Disk %d `%s`: %s%s" % (d["n"], d["file"], "AmigaDOS, volume '%s', %d files" % (d["volume"], d["files"])
                                           if d["dos"] else "non-DOS (custom trackloader; see its bootblock)", ""))
    md += ["", "## Executables (%d hunk files found, %d analysed, startup-sequence ones first)" % (
        man["candidates"], len(man["exes"]))]
    for e in man["exes"]:
        if e.get("error"):
            md.append("- %s: %s" % (e["name"], e["error"]))
        else:
            md.append("- %s (%d bytes): base $%06X entry $%06X, %d hunks, %d relocs; Ghidra %d functions, %d "
                      "instructions -> %s/functions.txt, listing.txt.gz, hunks.txt" % (
                          e["name"], e["size"], e["base"], e["entry"], e["hunks"], e["relocs"], e["functions"],
                          e["instructions"], e["dir"]))
    md += ["", "## Bootblocks (code from offset 12, analysed at base $0)"]
    for b in man["boot"]:
        md.append("- Disk %d (%s): Ghidra %d functions, %d instructions -> %s/" % (
            b["disk"], "DOS" if b["dos"] else "non-DOS", b["functions"], b["instructions"], b["dir"]))
    md += ["", "Ghidra only follows code it can see from the entry point. Code that the game copies, unpacks or "
           "loads from disk at run time is not in these listings: dump that region again with "
           "/w/kit/amigo/ghidra_dump.sh at its run-time base once you have it."]
    if man["errors"]:
        md += ["", "Errors: " + "; ".join(man["errors"])]
    open(os.path.join(out, "SUMMARY.md"), "w").write("\n".join(md) + "\n")
    subprocess.run(["rm", "-rf", scratch])      # extracted game files and images: copies, rebuilt in seconds
    subprocess.run(["chmod", "-R", "a+rX", out])
    print("PRECOOK OK %d exes, %d bootblocks, %.0f s" % (len(man["exes"]), len(man["boot"]), man["secs"]))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
