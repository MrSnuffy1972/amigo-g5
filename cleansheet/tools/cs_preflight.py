"""cs_preflight.py PHASE [--mode off|observe|enforce] [--triage T] [--ticks N]: clean-sheet run evidence (Component 1 of the
Copilot architecture plan, staged 19 Oct 2026). PHASE is start | snapshot | generated | build | final. Called from cs_game.sh
inside the tools container (/w). Python 3 stdlib only.

Modes (--mode, else env CS_PREFLIGHT, else the file /w/cs_preflight_mode, else observe):
  off      no output, no effect (writes nothing at all)
  observe  writes evidence to /w/gates/CS/{preflight.json,run_manifest.json}; never fails, never changes the outcome class
  enforce  accepted, behaves exactly like observe (no enforcement rule has been approved yet)
Every phase updates the SAME versioned JSON (schema_version). Writes are atomic (temp file in the same dir, flush, fsync,
os.replace). This program always exits 0: any exception is caught, logged to stdout (cs_game.sh appends it to preflight.log)
and recorded under errors[] when the JSON can still be written. CS_PREFLIGHT_TEST_CRASH=PHASE forces an exception (tests).

Volatile fields (differ run to run by design, everything else should compare equal on a repeat run): listed in
volatile_fields in the JSON itself. run_manifest.json carries comparable_sha256 = sha256 of preflight.json minus those.
"""
import hashlib, json, os, re, subprocess, sys, time

SCHEMA_VERSION = 1
W = os.environ.get("CS_PREFLIGHT_ROOT", "/w")
CS = os.path.join(W, "gates", "CS")
PJ = os.path.join(CS, "preflight.json")
RM = os.path.join(CS, "run_manifest.json")
PHASES = ("start", "snapshot", "generated", "build", "final")
VOLATILE = ["phase_times", "updated_at", "stage_times"]
BIG = 1 << 20


def sha_file(p):
    if not os.path.isfile(p):
        return None
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for b in iter(lambda: f.read(BIG), b""):
            h.update(b)
    return h.hexdigest()


def tree_sha(root, keep=None, skip_dirs=("__pycache__", ".git")):
    """Ordered tree hash: sha256 over 'relpath NUL sha256(file) NL' for every regular file, sorted by relpath."""
    if not os.path.isdir(root):
        return None, 0
    items = []
    for d, dirs, files in os.walk(root):
        dirs[:] = sorted(x for x in dirs if x not in skip_dirs)
        for f in files:
            p = os.path.join(d, f)
            if os.path.islink(p) or not os.path.isfile(p) or f.endswith(".pyc"):
                continue
            rel = os.path.relpath(p, root)
            if keep and not keep(rel):
                continue
            items.append((rel, sha_file(p)))
    items.sort()
    h = hashlib.sha256()
    for rel, s in items:
        h.update(("%s\0%s\n" % (rel, s)).encode())
    return h.hexdigest(), len(items)


def rd(p, n=4096):
    try:
        with open(p, errors="replace") as f:
            return f.read(n).strip()
    except OSError:
        return None


def file_entry(p):
    if not os.path.isfile(p):
        return None
    return {"bytes": os.path.getsize(p), "sha256": sha_file(p)}


def files_present(paths):
    """{relpath: {bytes, sha256}} for the paths that exist (absent ones are simply not listed)."""
    out = {}
    for p in paths:
        e = file_entry(p)
        if e:
            out[os.path.relpath(p, W)] = e
    return out


def dir_files(d, pat=None):
    if not os.path.isdir(d):
        return []
    return [os.path.join(d, f) for f in sorted(os.listdir(d))
            if os.path.isfile(os.path.join(d, f)) and (pat is None or re.search(pat, f))]


def frames(d, prefix):
    """Ordered streaming aggregate over frame files prefixNNNNN.(rgba|png|ppm): count, first/last hash, aggregate sha256."""
    fs = dir_files(d, r"^%s\d+\.(rgba|png|ppm)$" % prefix)
    agg = hashlib.sha256()
    first = last = None
    for p in fs:
        s = sha_file(p)
        agg.update(("%s\0%s\n" % (os.path.basename(p), s)).encode())
        first = first or s
        last = s
    return {"dir": d, "count": len(fs), "first": os.path.basename(fs[0]) if fs else None, "first_sha256": first,
            "last": os.path.basename(fs[-1]) if fs else None, "last_sha256": last,
            "aggregate_sha256": agg.hexdigest() if fs else None}


def atomic_json(path, obj):
    d = os.path.dirname(path)
    tmp = os.path.join(d, ".%s.tmp-%d" % (os.path.basename(path), os.getpid()))
    with open(tmp, "w") as f:
        json.dump(obj, f, indent=1, sort_keys=True)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())
    os.replace(tmp, path)
    try:
        fd = os.open(d, os.O_RDONLY)
        try:
            os.fsync(fd)
        finally:
            os.close(fd)
    except OSError:
        pass


def symcheck(binary):
    """Run the kit's symcheck.sh. rc 0 = clean, 1 = forbidden symbols, 2 = no nm tool (2 is NOT a pass)."""
    sc = os.path.join(W, "kit", "amigo", "port", "symcheck.sh")
    if not os.path.isfile(sc):
        return {"rc": None, "verdict": "no-symcheck-script", "pass": False, "output": None}
    if not os.path.isfile(binary):
        return {"rc": None, "verdict": "no-binary", "pass": False, "output": None}
    try:
        r = subprocess.run(["bash", sc, binary], capture_output=True, text=True, timeout=180)
        rc, out = r.returncode, (r.stdout + r.stderr).strip()[-400:]
    except Exception as ex:
        return {"rc": None, "verdict": "symcheck-error: %s" % str(ex)[:160], "pass": False, "output": None}
    return {"rc": rc, "verdict": {0: "clean", 1: "forbidden", 2: "no-nm-tool"}.get(rc, "rc%d" % rc), "pass": rc == 0,
            "output": out.replace(W + "/", "/w/")}


def build_kind(binary):
    try:
        with open(binary, "rb") as f:
            m = f.read(20)
    except OSError:
        return None
    if m[:4] == b"\x7fELF":
        return "clean-sheet-native-linux" if m[18:20] in (b"\x3e\x00",) else "clean-sheet-native-elf-other"
    if m[:2] == b"MZ":
        return "clean-sheet-native-windows"
    return "unknown"


def runtime_config(a):
    """Per-run knobs the workspace carries (/w/cs_* files, not logs) plus the cs_game.sh arguments."""
    cfg = {"args": {"triage": a.triage, "ticks": a.ticks}, "files": {}}
    for f in sorted(os.listdir(W)):
        p = os.path.join(W, f)
        if f.startswith("cs_") and "." not in f and os.path.isfile(p):   # knobs have no extension (cs_imgfmt, cs_input_mode ...); logs/outputs do
            sz = os.path.getsize(p)
            cfg["files"][f] = rd(p, 512) if sz <= 256 else {"bytes": sz, "sha256": sha_file(p)}
    for k in ("AMIGO_J", "MAKEFLAGS", "AMIGO_IMG"):
        if k in os.environ:
            cfg.setdefault("env", {})[k] = os.environ[k]
    return cfg


def identity(a):
    kit = os.path.join(W, "kit", "amigo")
    kh, kn = tree_sha(kit)
    th, tn = tree_sha(os.path.join(W, "cs_tools"), keep=lambda r: r.endswith((".py", ".sh", ".json")))
    cfg = runtime_config(a)
    img = rd(os.path.join(W, "cs_container_image")) or "amigo-tools:3"
    return {"dev_kit_tree_sha256": kh, "dev_kit_files": kn, "clean_sheet_tools_tree_sha256": th, "clean_sheet_tools_files": tn,
            "container_image_claim": img, "container_image_digest": None,
            "runtime_config_sha256": hashlib.sha256(json.dumps(cfg, sort_keys=True).encode()).hexdigest(),
            "runtime_config": cfg}


def git_head():
    for d in (os.path.join(W, "kit", "amigo"), os.path.join(W, "kit")):
        try:
            r = subprocess.run(["git", "-C", d, "rev-parse", "HEAD"], capture_output=True, text=True, timeout=20)
            if r.returncode == 0 and r.stdout.strip():
                return {"value": r.stdout.strip(), "source": d.replace(W, "/w")}
        except Exception:
            pass
    return {"value": None, "source": "the workspace kit is an rsync copy without .git; see reported_kit_rev_head"}


def ph_start(a, doc):
    pid = rd(os.path.join(W, "cs_project_id"))
    doc["project_id"] = int(pid) if pid and pid.isdigit() else None   # never inferred from the path
    if doc["project_id"] is None:
        doc["warnings"].append("start: /w/cs_project_id absent or not an integer; project_id left null")
    kr = rd(os.path.join(W, "cs_kit_rev"))
    doc["reported_kit_rev"] = kr                                       # exactly as cs_queue.kit_rev() formats it
    doc["reported_kit_rev_head"] = kr.split("+")[0] if kr else None
    if kr is None:
        doc["warnings"].append("start: /w/cs_kit_rev absent; reported_kit_rev null")
    doc["content_identity"] = identity(a)
    doc["dev_kit_git_head"] = git_head()
    return {"workspace_files": files_present([os.path.join(W, "prekit.json"), os.path.join(W, "cs_kitfix.log")])}


def ph_snapshot(a, doc):
    o = os.path.join(W, "out")
    S = os.path.join(W, "cs_tools")
    regs = os.path.join(o, "snap", "snap_regs.txt")
    r = {"game_orig_json": file_entry(os.path.join(W, "game-orig.json")), "game_json": file_entry(os.path.join(W, "game.json")),
         "g0_artefacts": files_present([os.path.join(o, x) for x in ("port/ref/first_bytes.txt", "port/ref/first_bytes.noinput.txt",
                                        "port/harness", "port/coverage.json", "g0ai_code.bin", "image.bin", "probe.txt")]),
         "snapshot_inputs": files_present(dir_files(os.path.join(o, "snap")) + dir_files(os.path.join(W, "snapcfg"))),
         "register_file": file_entry(regs), "snap_pc": rd(os.path.join(CS, "snap_pc")), "snap_frame": rd(os.path.join(CS, "snap_frame")),
         "input_mode": rd(os.path.join(CS, "input_mode")),
         "input_script": files_present([os.path.join(S, x) for x in ("noinput.txt", "in_nat.txt", "in_ref.txt")])}
    for k in ("game_orig_json", "game_json", "register_file"):
        if r[k] is None:
            doc["warnings"].append("snapshot: %s absent" % k)
    return r


def ph_generated(a, doc):
    g = os.path.join(W, "out", "port", "gen")
    gh, gn = tree_sha(g, keep=lambda x: x.endswith((".c", ".h")))
    chunks = dir_files(g, r"^chunk_.*\.c$")
    tp = os.path.join(g, "translated_pcs.txt")
    n_pcs = None
    if os.path.isfile(tp):
        with open(tp, errors="replace") as f:
            n_pcs = sum(1 for l in f if l.strip())
    stats = None
    try:
        with open(os.path.join(g, "translate_stats.json")) as f:
            stats = json.load(f)
    except Exception:
        pass
    disc = rd(os.path.join(CS, "5-discover.log"), 1 << 22) or ""
    rounds = [[int(x), int(y)] for x, y in re.findall(r"native_discover round (\d+): (\d+) interpreted addresses", disc)]
    if gh is None:
        doc["warnings"].append("generated: out/port/gen absent")
    return {"generated_source_sha256": gh, "generated_source_files": gn, "chunk_count": len(chunks),
            "translated_pc_count": n_pcs, "translated_pcs_sha256": sha_file(tp), "translate_stats": stats,
            "discover_rounds": rounds, "discover_mode": rd(os.path.join(CS, "discover_mode"))}


def ph_build(a, doc):
    b = os.path.join(W, "out", "native")
    log = rd(os.path.join(CS, "6-build.log"), 1 << 22) or ""
    m = re.findall(r"^.*SYMCHECK.*$", log, re.M)
    sc = symcheck(b)
    if sc["rc"] == 2:
        doc["warnings"].append("build: symcheck rc 2 (no nm tool) - NOT a pass")
    gh, _ = tree_sha(os.path.join(W, "out", "port", "gen"), keep=lambda x: x.endswith((".c", ".h")))
    gen = doc["phases"].get("generated") or {}
    if gen.get("generated_source_sha256") and gh != gen["generated_source_sha256"]:
        doc["warnings"].append("build: generated source changed between the generated and build phases")
    return {"build_kind": build_kind(b), "binary": file_entry(b), "symcheck": sc, "generated_source_sha256_at_build": gh,
            "build_log_symcheck_lines": [x.replace(W + "/", "/w/")[:240] for x in m[:4]]}


def ph_final(a, doc):
    b = os.path.join(W, "out", "native")
    bld = doc["phases"].get("build") or {}
    fb = file_entry(b)
    changed = bool(bld.get("binary") and fb and bld["binary"]["sha256"] != fb["sha256"])
    sc = symcheck(b) if (changed or not bld.get("symcheck")) else dict(bld["symcheck"], reused_from="build")
    nat = frames("/tmp/nat", "p")
    ref = frames("/tmp/ref", "f")
    nf = rd(os.path.join(CS, "native_frames"))
    if nf and nf.isdigit() and int(nf) != nat["count"]:
        doc["warnings"].append("final: native_frames %s but %d native frame files hashed" % (nf, nat["count"]))
    res = os.path.join(CS, "result.json")
    cls = None
    try:
        with open(res) as f:
            cls = json.load(f).get("class")
    except Exception as ex:
        doc["warnings"].append("final: result.json unreadable: %s" % str(ex)[:120])
    st = [l.split() for l in (rd(os.path.join(CS, "stages.txt"), 1 << 16) or "").splitlines() if l.strip()]
    doc["stage_times"] = {s[0]: int(s[1]) for s in st if len(s) > 1 and s[1].isdigit()}
    ci = doc.get("content_identity") or {}
    kh, _ = tree_sha(os.path.join(W, "kit", "amigo"))
    if ci.get("dev_kit_tree_sha256") and kh != ci["dev_kit_tree_sha256"]:
        doc["warnings"].append("final: workspace kit tree changed during the run")
    if changed:
        doc["warnings"].append("final: out/native rebuilt after the build phase (yield rounds); final binary differs")
    return {"build_kind": build_kind(b), "binary": fb, "binary_changed_since_build": changed, "symcheck": sc,
            "native_frames": nat, "reference_frames": ref,
            "audio": {"native_raw": file_entry("/tmp/nat.raw"), "reference_raw": file_entry("/tmp/ref.raw")},
            "input": {"manifest": file_entry(os.path.join(W, "gates", "INPUT", "input_manifest.json")), "g2_transitions": file_entry(os.path.join(W, "gates", "INPUT", "g2-transitions.json")), "native_events": file_entry(os.path.join(W, "gates", "INPUT", "g2-native-events.jsonl")), "reference_events": file_entry(os.path.join(W, "gates", "INPUT", "g2-reference-events.jsonl"))},
            "result_json": file_entry(res), "class": cls, "run_rc": rd(os.path.join(CS, "run_rc")),
            "fail": rd(os.path.join(CS, "fail")), "stages": [s[0] for s in st],
            "kit_tree_sha256_at_final": kh}


FUNCS = {"start": ph_start, "snapshot": ph_snapshot, "generated": ph_generated, "build": ph_build, "final": ph_final}


def new_doc(mode, a):
    return {"schema_version": SCHEMA_VERSION, "tool": "cs_preflight.py", "tool_sha256": sha_file(os.path.abspath(__file__)),
            "mode": mode, "project_id": None, "triage": a.triage, "ticks": a.ticks, "reported_kit_rev": None,
            "reported_kit_rev_head": None, "content_identity": None, "dev_kit_git_head": None,
            "phases_run": [], "phases": {}, "warnings": [], "errors": [], "phase_times": {}, "stage_times": {},
            "updated_at": None, "volatile_fields": list(VOLATILE)}


def comparable(doc):
    return hashlib.sha256(json.dumps({k: v for k, v in doc.items() if k not in VOLATILE}, sort_keys=True).encode()).hexdigest()


def manifest(doc):
    ph = doc["phases"]
    fin, bld = ph.get("final") or {}, ph.get("build") or {}
    gen = ph.get("generated") or {}
    binr = fin.get("binary") or bld.get("binary") or {}
    sc = fin.get("symcheck") or bld.get("symcheck") or {}
    ci = doc.get("content_identity") or {}
    return {"schema_version": SCHEMA_VERSION, "kind": "cs-run-manifest", "mode": doc["mode"], "project_id": doc["project_id"],
            "triage": doc["triage"], "ticks": doc["ticks"], "reported_kit_rev": doc["reported_kit_rev"],
            "content_identity": {k: v for k, v in ci.items() if k != "runtime_config"}, "phases_run": doc["phases_run"],
            "generated_source_sha256": gen.get("generated_source_sha256"), "chunk_count": gen.get("chunk_count"),
            "translated_pc_count": gen.get("translated_pc_count"),
            "build_kind": fin.get("build_kind") or bld.get("build_kind"), "binary_sha256": binr.get("sha256"),
            "symcheck_rc": sc.get("rc"), "symcheck_verdict": sc.get("verdict"),
            "native_frames_aggregate_sha256": (fin.get("native_frames") or {}).get("aggregate_sha256"),
            "native_frames_count": (fin.get("native_frames") or {}).get("count"),
            "audio_native_sha256": ((fin.get("audio") or {}).get("native_raw") or {}).get("sha256"),
            "result_json_sha256": (fin.get("result_json") or {}).get("sha256"), "class": fin.get("class"),
            "input_manifest_sha256": (((fin.get("input") or {}).get("manifest") or {}).get("sha256")), "input_transition_sha256": (((fin.get("input") or {}).get("g2_transitions") or {}).get("sha256")),
            "warnings_n": len(doc["warnings"]), "errors_n": len(doc["errors"]),
            "preflight_comparable_sha256": comparable(doc), "updated_at": doc["updated_at"], "volatile_fields": ["updated_at"]}


def parse(argv):
    import argparse
    ap = argparse.ArgumentParser(add_help=True)
    ap.add_argument("phase", choices=PHASES)
    ap.add_argument("--mode")
    ap.add_argument("--triage")
    ap.add_argument("--ticks", type=int)
    return ap.parse_args(argv)


def main(argv):
    a = parse(argv)
    mode = (a.mode or os.environ.get("CS_PREFLIGHT") or rd(os.path.join(W, "cs_preflight_mode")) or "observe").strip().lower()
    if mode == "off":
        return 0                                   # off: no output, no effect
    note = None
    if mode not in ("observe", "enforce"):
        note, mode = "unknown mode %r treated as observe" % mode, "observe"
    crash = os.environ.get("CS_PREFLIGHT_TEST_CRASH", "").split(",")   # e.g. "snapshot,hard:final"
    if "hard:" + a.phase in crash:
        raise SystemExit(70)                       # test hook: the process dies with rc 70 before writing anything
    t0 = time.time()
    if not os.path.isdir(CS):
        print("preflight %s: %s missing, nothing written" % (a.phase, CS))
        return 0
    doc = None
    if a.phase != "start" and os.path.exists(PJ):
        try:
            with open(PJ) as f:
                doc = json.load(f)
            if doc.get("schema_version") != SCHEMA_VERSION:
                doc = None
        except Exception:
            doc = None
    if doc is None:
        doc = new_doc(mode, a)
        if a.phase != "start":
            doc["warnings"].append("%s: no usable preflight.json from earlier phases; started a new one" % a.phase)
    doc["mode"] = mode
    if note:
        doc["warnings"].append(note)
    try:
        if a.phase in crash:
            raise RuntimeError("CS_PREFLIGHT_TEST_CRASH=%s (simulated exception)" % a.phase)
        doc["phases"][a.phase] = FUNCS[a.phase](a, doc)
    except Exception as ex:
        import traceback
        doc["errors"].append({"phase": a.phase, "error": "%s: %s" % (type(ex).__name__, str(ex)[:300])})
        print("preflight %s EXCEPTION (recorded, run unaffected):\n%s" % (a.phase, traceback.format_exc()[-1500:]))
    if a.phase not in doc["phases_run"]:
        doc["phases_run"].append(a.phase)
    doc["phase_times"][a.phase] = {"at": time.strftime("%Y-%m-%dT%H:%M:%S%z"), "took_s": round(time.time() - t0, 3)}
    doc["updated_at"] = doc["phase_times"][a.phase]["at"]
    atomic_json(PJ, doc)
    atomic_json(RM, manifest(doc))
    print("preflight %s mode=%s took=%.2fs warnings=%d errors=%d" % (a.phase, mode, time.time() - t0, len(doc["warnings"]), len(doc["errors"])))
    return 0


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except SystemExit as ex:
        if ex.code == 70:
            raise
    except BaseException as ex:                    # never fail the run
        print("preflight: unexpected %s: %s (ignored)" % (type(ex).__name__, str(ex)[:300]))
    sys.exit(0)
