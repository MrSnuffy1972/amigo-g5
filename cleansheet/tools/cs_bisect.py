#!/usr/bin/env python3
"""cs_bisect.py v3 (c13, 19 Oct 2026: tick alignment, reference-hash.txt, per-run HASHLOG/TICKPCLOG + PC window,
  blocks_of() keeps true leaders) [--base CHUNKS|FILE] [--iterate] [--blocks] [--max-iter 8]
  v2: base selection always interpreted; --iterate adds each guilty set to the base and repeats to N; --blocks narrows each
  guilty chunk to its translated blocks (leaders.txt ranges).
  CAVEAT (--blocks): the interpreter hook hands control back only AFTER the first translated PC outside the selection, so an
  interpreted block also runs the FIRST INSTRUCTION AFTER IT. A guilty block LO-HI therefore means "this block OR the next
  instruction" (Xenon 2: guilty block $568 was really the movem at $570). Check both before blaming the block.
cs_bisect.py [--ws /w] [--F F] [--N N] [--script S] [--jobs 4] [--margin 5] [--timeout 300] [--no-build]
                [--thresh 0.001] [--tol 1] [--persist 5] [--skip 1]
Per-chunk bisection of a clean-sheet native port against the reference harness (DEV ONLY, 18 Oct 2026, from Copilot's design,
divergence rule corrected after the Battletoads negative control). Runs INSIDE the tools container with the game's clean-sheet
workspace at /w. Needs the kit's cpu_native.c with the AMIGO_PROBE BISECT_INTERP patch. Steps: build out/nativeprobe; reference
frames (harness from power-on, frames F+1..F+N, as cs_game.sh ref2 + cs_align.py); all-native baseline; all-interpreted
baseline; ddmin over the translated 4 KB chunks (out/port/gen/chunk_XXXXXX.c) for the smallest set whose interpretation removes
the first divergence; confirm. Result: gates/CS/bisect/result.json.
A tick is BAD when, against each of the reference ticks t-tol..t+tol, more than thresh of the pixels have an RGB channel differing
by more than 8. The run DIVERGES at the first tick of the first stretch of `persist` consecutive bad ticks; the first `skip`
ticks are not judged. (Copilot's literal rule, any single tick with >0.1% bad pixels against tick t, failed the negative
control: Battletoads' palette fades run one tick early or late on the native port, and its first frames differ.)"""
import argparse, json, os, re, subprocess, sys, time, shutil
from concurrent.futures import ThreadPoolExecutor
import numpy as np

ap = argparse.ArgumentParser()
ap.add_argument("--ws", default="/w"); ap.add_argument("--F", type=int); ap.add_argument("--N", type=int)
ap.add_argument("--script"); ap.add_argument("--jobs", type=int, default=4); ap.add_argument("--margin", type=int, default=5)
ap.add_argument("--timeout", type=int, default=300); ap.add_argument("--no-build", action="store_true")
ap.add_argument("--thresh", type=float, default=0.001); ap.add_argument("--tol", type=int, default=1)
ap.add_argument("--persist", type=int, default=5); ap.add_argument("--skip", type=int, default=1)
ap.add_argument("--tag", default="")
ap.add_argument("--base", default=""); ap.add_argument("--iterate", action="store_true")
ap.add_argument("--blocks", action="store_true"); ap.add_argument("--max-iter", type=int, default=8)
a = ap.parse_args()
W = a.ws; CS = W + "/gates/CS"; OUT = CS + "/bisect" + a.tag; os.makedirs(OUT, exist_ok=True)
LOG = open(OUT + "/bisect.log", "a")
T0 = time.time()
def log(*m):
    s = "[%6.1fs] " % (time.time() - T0) + " ".join(str(x) for x in m); print(s, flush=True); LOG.write(s + "\n"); LOG.flush()
res = {"status": None, "tests": 0, "format": 3}
def finish(status, **kw):
    res.update(kw); res["status"] = status; res["seconds"] = round(time.time() - T0, 1)
    json.dump(res, open(OUT + "/result.json", "w"), indent=1); log("STATUS", status, json.dumps(kw)[:600]); sys.exit(0)

F = a.F if a.F is not None else int(open(CS + "/snap_frame").read().split()[0])
N = a.N if a.N is not None else int(json.load(open(CS + "/result.json")).get("ticks", 600))
mode = open(CS + "/input_mode").read().strip() if os.path.exists(CS + "/input_mode") else "none"
nscript = a.script or ("cs_tools/in_nat.txt" if mode in ("seq", "fire") else "cs_tools/noinput.txt")
rscript = (W + "/cs_tools/in_ref.txt") if mode in ("seq", "fire") else ""
res.update(F=F, N=N, input_mode=mode, script=nscript, margin=a.margin, rule=dict(thresh=a.thresh, tol=a.tol, persist=a.persist, skip=a.skip))
log("F", F, "N", N, "input", mode, nscript, rscript, "rule", res["rule"])
# v3 (c13): native tick t is harness frame F+t; the reference frame hashes rebased to native ticks
res["alignment"] = {"native_tick_1": F + 1, "formula": "harness_frame=F+native_tick"}
def _rebase_hash():
    src = W + "/out/port/whd/hash.txt"
    if not os.path.exists(src): res["reference_hash"] = None; return
    rows = {}
    for l in open(src, errors="replace"):
        q = l.split()
        if q and q[0].isdigit(): rows[int(q[0])] = q[1:]
    n = 0
    with open(OUT + "/reference-hash.txt", "w") as o:
        for t in range(1, N + 1):
            if F + t in rows: o.write("%d %s\n" % (t, " ".join(rows[F + t]))); n += 1
    res["reference_hash"] = {"source": "out/port/whd/hash.txt", "rows": n, "file": "reference-hash.txt"}
_rebase_hash()
K = W + "/kit/amigo/port"; ENV = dict(os.environ, AMIGO_IMG="rgba"); ENV.pop("BISECT_INTERP", None)

if not a.no_build:
    log("build nativeprobe")
    r = subprocess.run(["bash", K + "/amigo_port.sh", "nativeprobe"], cwd=W, stdout=open(OUT + "/build.log", "w"), stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
    if r.returncode or not os.path.exists(W + "/out/nativeprobe"): finish("error", why="nativeprobe build failed")

REF = W + "/bisect/ref"
def readf(p):
    for e in ("rgba", "png", "ppm"):
        if os.path.exists(p + "." + e):
            b = open(p + "." + e, "rb").read()
            if b[:4] == b"RGBA":
                w, h = int.from_bytes(b[4:8], "little"), int.from_bytes(b[8:12], "little")
                return np.frombuffer(b, np.uint8, w * h * 4, 12).reshape(h, w, 4)[..., :3].astype(np.int16)
            from PIL import Image
            return np.asarray(Image.open(p + "." + e).convert("RGB")).astype(np.int16)
    return None
if not any(os.path.exists("%s/f%05d.%s" % (REF, N, e)) for e in ("rgba", "png", "ppm")):
    log("reference: harness to frame", F + N + a.tol)
    ra = W + "/bisect/refall"; os.makedirs(ra, exist_ok=True); os.makedirs(REF, exist_ok=True)
    cmd = [W + "/out/port/harness", W + "/game", str(F + N + a.tol), ra] + ([rscript] if rscript else [])
    subprocess.run(cmd, cwd=W, env=dict(ENV, DUMPALL="1"), stdout=open(OUT + "/ref.log", "w"), stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, timeout=1800)
    n = 0
    for t in range(1, N + a.tol + 1):
        for e in ("rgba", "png", "ppm"):
            s = "%s/f%05d.%s" % (ra, F + t, e)
            if os.path.exists(s): shutil.copy(s, "%s/f%05d.%s" % (REF, t, e)); n += 1; break
    log("aligned", n, "reference frames from harness frame", F + 1)
_refc = {}
def ref(t):
    if t not in _refc: _refc[t] = readf("%s/f%05d" % (REF, t))
    return _refc[t]
def bad(t, d):
    x = readf("%s/p%05d" % (d, t))
    if x is None: return True                   # the run stopped early (a crash or a finding): bad from here
    v = []
    for k in range(-a.tol, a.tol + 1):
        r = ref(t + k)
        if r is not None and r.shape == x.shape: v.append((np.abs(r - x) > 8).any(axis=2).mean())
    if not v: return ref(t) is not None         # no reference frame at all: cannot judge (past the end)
    return min(v) > a.thresh
def first_div(d, H):
    run = 0
    for t in range(a.skip + 1, H + 1):
        run = run + 1 if bad(t, d) else 0
        if run >= a.persist: return t - a.persist + 1
    return None

seq = [0]
def run(sel, H, name=None):
    seq[0] += 1; k = name or "t%04d" % seq[0]; d = "%s/bisect/run/%s" % (W, k)
    if os.path.exists(d): os.rename(d, d + ".old%d" % int(time.time() * 1000))
    os.makedirs(d)
    env = dict(ENV, HASHLOG=d + ".hash", TICKPCLOG=d + ".pc")   # v3 (c13): per-tick hashes and native PCs
    if sel == "*": env["BISECT_INTERP"] = "*"
    elif sel:
        open(d + ".sel", "w").write("".join(x + "\n" for x in sorted(sel))); env["BISECT_INTERP"] = d + ".sel"
    try:
        r = subprocess.run([W + "/out/nativeprobe", "--frames", str(H), "--dump", d, "--script", W + "/" + nscript],
                           cwd=W, env=env, stdout=subprocess.DEVNULL, stderr=open(d + ".err", "w"), stdin=subprocess.DEVNULL, timeout=a.timeout)
        rc = r.returncode
    except subprocess.TimeoutExpired: rc = "timeout"
    res["tests"] += 1
    fd = first_div(d, H)
    if fd is not None:   # v3 (c13): native PCs around the divergence, ticks div-margin .. div+persist-1+margin
        lo, hi = fd - a.margin, fd + a.persist - 1 + a.margin
        try:
            with open(d + ".window", "w") as o:
                for l in open(d + ".pc"):
                    q = l.split()
                    if len(q) == 2 and q[0].isdigit() and lo <= int(q[0]) <= hi: o.write(l)
        except OSError: pass
    return fd, rc

# ---- v2 (18 Oct 2026, test copy): --base / --iterate / --blocks. Items are selection lines as BISECT_INTERP reads them:
# "XXXXXX" = the 4 KB chunk, "LO-HI" = a range (one translated block, from out/port/gen/leaders.txt).
G = W + "/out/port/gen"
chunks = sorted(int(m.group(1), 16) for f in os.listdir(G) for m in [re.match(r"chunk_([0-9A-Fa-f]{6})\.c$", f)] if m)
res["chunks"] = len(chunks); log(len(chunks), "translated chunks")
leaders = sorted(set(int(x.split()[0], 16) for x in open(G + "/leaders.txt") if x.strip())) if os.path.exists(G + "/leaders.txt") else []
def blocks_of(c):
    ls = [l for l in leaders if c <= l <= c + 0xFFF]
    if not ls: return ["%06X" % c]
    ends = [l - 1 for l in ls[1:]] + [c + 0xFFF]   # v3 (c13): the first range starts at the true first leader
    return ["%06X-%06X" % (lo, hi) for lo, hi in zip(ls, ends)]
cache = {}
def passes_at(H):
    def p(sel):
        key = (H, frozenset(sel))
        if key not in cache: cache[key] = run(set(sel), H)[0] is None
        return cache[key]
    return p
def ddmin(items, base, H, what):
    p = passes_at(H); cur = list(items); n = 2
    def pm(sets):
        with ThreadPoolExecutor(a.jobs) as ex: return list(ex.map(lambda s: p(base + s), sets))
    while len(cur) >= 2:
        sz = (len(cur) + n - 1) // n; subs = [cur[i:i + sz] for i in range(0, len(cur), sz)]
        rs = pm(subs)
        if any(rs): cur = subs[rs.index(True)]; n = 2; log(what, "reduced to subset of", len(cur)); continue
        comps = [[c for c in cur if c not in s] for s in subs] if n > 2 else []
        rc_ = pm(comps) if comps else []
        if any(rc_): cur = comps[rc_.index(True)]; n = max(n - 1, 2); log(what, "reduced to complement of", len(cur)); continue
        if n >= len(cur): break
        n = min(len(cur), 2 * n); log(what, "granularity", n, "set", len(cur))
    return cur
base = [x.strip().upper() for x in (a.base or "").split(",") if x.strip()]
if a.base and os.path.exists(a.base): base = [x.strip().upper() for x in open(a.base) if x.strip()]
res["base"] = list(base); chain = []; res["chain"] = chain
for it in range(a.max_iter):
    d, rc = run(set(base), N, "native_i%d" % it); log("round", it, "base", base, ": first divergence", d, "rc", rc)
    if it == 0: res.update(native_div=d, native_rc=rc, first_divergent_tick=d, first_divergent_harness_frame=(F + d) if d is not None else None)
    if d is None: finish("negative-control-pass" if it == 0 and not base else "matches-whole-N", final_base=base)
    H = min(N, d + a.persist - 1 + a.margin)
    dall, rc = run("*", H, "allinterp_i%d" % it); log("round", it, "all-interpreted to", H, ":", dall)
    if dall is not None:
        finish("all-interpreted-diverges" if it == 0 else "remaining-divergence-is-timing", timing_div=dall, final_base=base, last_div=d)
    items = ["%06X" % c for c in chunks if "%06X" % c not in base]
    cur = ddmin(items, base, H, "round %d chunks" % it)
    if not passes_at(H)(base + cur): finish("translation-set-not-confirmed", candidate=cur, final_base=base, last_div=d)
    e = {"round": it, "div": d, "horizon": H, "guilty": cur}
    if a.blocks:
        e["blocks"] = {}
        for c in cur:
            bl = blocks_of(int(c, 16)); rest = [x for x in cur if x != c]
            bb = ddmin(bl, base + rest, H, "round %d blocks of %s (%d)" % (it, c, len(bl)))
            e["blocks"][c] = {"n_blocks": len(bl), "guilty_blocks": bb, "confirmed": passes_at(H)(base + rest + bb)}
            log("round", it, "chunk", c, "guilty blocks", bb)
    chain.append(e); base = base + cur; log("round", it, "guilty", cur, "-> base", base)
    json.dump(res, open(OUT + "/result.json", "w"), indent=1)
    if not a.iterate: break
d, rc = run(set(base), N, "final"); res["next_div_with_base"] = d
finish("max-rounds" if a.iterate else "translation-set-found", final_base=base, guilty=chain[-1]["guilty"] if chain else None)
