#!/usr/bin/env python3
import glob, hashlib, json, os, re, tempfile, time
W = os.environ.get("CS_ROOT", "/w")
CS = os.path.join(W, "gates", "CS")
OUT = os.path.join(CS, "discovery_ledger.json")
SEED_RE = re.compile(r"native_discover round (\d+): (\d+) interpreted addresses(?:,\s*(.*))?")
REDO_RE = re.compile(r"rediscovery (\d+): rc (-?\d+), interpreted (-?\d+) -> (-?\d+)")

def read(path):
    try:
        with open(path, errors="replace") as f:
            return f.read()
    except OSError:
        return ""

def sha_file(path):
    try:
        h = hashlib.sha256()
        with open(path, "rb") as f:
            for b in iter(lambda: f.read(1048576), b""):
                h.update(b)
        return h.hexdigest()
    except OSError:
        return None

def atomic_json(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fd, tmp = tempfile.mkstemp(prefix=".discovery-ledger-", dir=os.path.dirname(path))
    try:
        with os.fdopen(fd, "w") as f:
            json.dump(value, f, indent=1, sort_keys=True)
            f.write("\n")
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)

def result_json():
    try:
        with open(os.path.join(CS, "result.json")) as f:
            return json.load(f)
    except Exception:
        return {}

def preflight_json():
    try:
        with open(os.path.join(CS, "preflight.json")) as f:
            return json.load(f)
    except Exception:
        return {}

def resolve_seed_file():
    cfg = result_json()
    candidates = []
    try:
        with open(os.path.join(W, "game.json")) as f:
            game = json.load(f)
        for seed in game.get("seeds", []):
            if isinstance(seed, str):
                candidates.append(seed)
    except Exception:
        pass
    candidates += ["snapcfg/seeds.txt", "snapcfg/seed.txt", "out/port/ref/executed_pcs.txt"]
    for rel in candidates:
        path = rel if os.path.isabs(rel) else os.path.join(W, rel)
        if os.path.isfile(path):
            return os.path.relpath(path, W), path
    return None, None

def terminal_reason(text, rc, count):
    if rc == 124:
        return "timeout"
    if re.search(r"wild_jump", text):
        return "wild-jump"
    if re.search(r"rom_call", text):
        return "rom-call"
    if count == 0:
        return "zero"
    if count is None or count < 0:
        return "unknown"
    return "round-limit"

seed_rel, seed_path = resolve_seed_file()
seed_final = {
    "path": seed_rel,
    "sha256": sha_file(seed_path) if seed_path else None,
    "lines": len([x for x in read(seed_path).splitlines() if x.strip()]) if seed_path else None,
}
logs = []
paths = [("main", os.path.join(CS, "5-discover.log"))]
paths += [("rediscovery-%s" % re.search(r"(\d+)$", p[:-4]).group(1), p) for p in sorted(glob.glob(os.path.join(CS, "5c-discover-*.log")))]
yield_log = read(os.path.join(CS, "7b-yields.log"))
redo = {int(a): {"rc": int(rc), "before": int(before), "after": int(after)} for a, rc, before, after in REDO_RE.findall(yield_log)}
for name, path in paths:
    text = read(path)
    rounds = []
    for number, count, stats in SEED_RE.findall(text):
        rounds.append({"round": int(number), "interpreted_addresses": int(count), "stats": stats.strip() or None})
    index = 0 if name == "main" else int(name.rsplit("-", 1)[1])
    rc = redo.get(index, {}).get("rc")
    if name == "main":
        m = re.search(r"native_discover rc (\d+)", text)
        rc = int(m.group(1)) if m else (0 if rounds else None)
    count = rounds[-1]["interpreted_addresses"] if rounds else redo.get(index, {}).get("after")
    logs.append({
        "kind": name,
        "path": os.path.relpath(path, W),
        "sha256": sha_file(path),
        "rc": rc,
        "rounds": rounds,
        "terminal_reason": terminal_reason(text, rc, count),
        "interpreted_before": redo.get(index, {}).get("before"),
        "interpreted_after": count,
    })
counts = [x["interpreted_after"] for x in logs if isinstance(x.get("interpreted_after"), int) and x["interpreted_after"] >= 0]
non_decreasing = any(b >= a and a > 0 for a, b in zip(counts, counts[1:]))
terminal = logs[-1]["terminal_reason"] if logs else "unknown"
fp_status = {}
for line in read(os.path.join(CS, "discovery-fixedpoint.status")).splitlines():
    if "=" in line:
        k, v = line.split("=", 1)
        fp_status[k.strip()] = v.strip()
if fp_status.get("reason"):
    terminal = fp_status["reason"]
elif non_decreasing:
    terminal = "non-decreasing"
elif counts and counts[-1] == 0:
    terminal = "zero"
fixed_point = terminal == "converged-no-seed-or-yield-change" if fp_status.get("reason") else terminal == "zero"
errors = []
warnings = []
if not logs or not logs[0]["rounds"]:
    errors.append("main discovery evidence missing")
if seed_path is None:
    warnings.append("seed file not found")
if not fixed_point:
    warnings.append("discovery terminated: " + terminal)
pf = preflight_json()
pf = dict(pf.get("content_identity") or {}, **pf)  # c16 A1: preflight keeps the tree hashes under content_identity
out = {
    "schema_version": 1,
    "status": "fail" if errors else ("warn" if warnings else "pass"),
    "fixed_point": fixed_point,
    "terminal_reason": terminal,
    "fixed_point_status": fp_status or None,
    "iterations": logs,
    "seed_final": seed_final,
    "provenance": {
        "reported_kit_rev": pf.get("reported_kit_rev"),
        "dev_kit_tree_sha256": pf.get("dev_kit_tree_sha256"),
        "clean_sheet_tools_tree_sha256": pf.get("clean_sheet_tools_tree_sha256"),
        "game_json_sha256": sha_file(os.path.join(W, "game.json")),
        "translated_pcs_sha256": sha_file(os.path.join(W, "out", "port", "gen", "translated_pcs.txt")),
        "leaders_sha256": sha_file(os.path.join(W, "out", "port", "gen", "leaders.txt")),
        "coverage_sha256": sha_file(os.path.join(W, "out", "port", "coverage.json")),
    },
    "warnings": warnings,
    "errors": errors,
    "generated_at": int(time.time()),
}
atomic_json(OUT, out)
print(out["status"], out["terminal_reason"], "fixed-point" if out["fixed_point"] else "not-fixed")
