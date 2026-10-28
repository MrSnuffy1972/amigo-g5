"""judge.py REF_DIR NEW_DIR [--frames N] [--wav-ref A.wav --wav-new B.wav]: the per-tick judge (Karl 15 Oct approved lines):
picture = SSIM >= 0.90 on 95 percent of ticks, plus CONTENT and (Cowork 20 Oct) COLOUR, the reverse per-channel match; --legacy = the old judge (a +-1 tick shift is allowed: best of ticks t-1, t, t+1), sound = 20 ms log-spectral/envelope
correlation >= 0.85. Reads frames fNNNNN/pNNNNN as raw .rgba, .png or legacy .ppm (Cowork 18 Oct). Prints one summary line per measure and the worst ticks."""
import sys, numpy as np, os, struct
def _img(p):
    """Cowork 18 Oct: one frame as an H x W x 3 uint8 array from raw RGBA (.rgba: "RGBA", w, h as little-endian uint32, then RGBA
    bytes), PNG, or legacy PPM (kept so old runs still score). The format is read from the file's first bytes."""
    b = open(p, "rb").read()
    if b[:4] == b"RGBA":
        w, h = struct.unpack("<II", b[4:12]); return np.frombuffer(b, np.uint8, w * h * 4, 12).reshape(h, w, 4)[..., :3]
    if b[:8] == b"\x89PNG\r\n\x1a\n":
        from PIL import Image; return np.asarray(Image.open(p).convert("RGB"))
    parts = b.split(b"\n", 3); w, h = map(int, parts[1].split()); return np.frombuffer(parts[3], np.uint8).reshape(h, w, 3)
EXTS = ("rgba", "png", "ppm")
def _fp(d, x, t):
    for e in EXTS:
        q = "%s/%s%05d.%s" % (d, x, t, e)
        if os.path.exists(q): return q
    return None
def gray(a): return (0.299 * a[..., 0] + 0.587 * a[..., 1] + 0.114 * a[..., 2]).astype(np.float64)
def boxf(x, k):
    c = np.cumsum(np.cumsum(np.pad(x, ((1, 0), (1, 0))), 0), 1)
    return (c[k:, k:] - c[:-k, k:] - c[k:, :-k] + c[:-k, :-k]) / (k * k)
def ssim(a, b, k=7):
    a, b = gray(a), gray(b); C1, C2 = (0.01 * 255) ** 2, (0.03 * 255) ** 2
    ma, mb = boxf(a, k), boxf(b, k); saa = boxf(a * a, k) - ma * ma; sbb = boxf(b * b, k) - mb * mb; sab = boxf(a * b, k) - ma * mb
    n = k * k / (k * k - 1.0); saa *= n; sbb *= n; sab *= n
    s = ((2 * ma * mb + C1) * (2 * sab + C2)) / ((ma * ma + mb * mb + C1) * (saa + sbb + C2))
    return float(s.mean())
def wav(p):
    b = open(p, "rb").read(); off = 0 if p.endswith(".raw") else 44; return np.frombuffer(b[off:], np.int16).reshape(-1, 2).astype(np.float64)
OFF = 0
AUTO = None; REFX = None   # G2E10 tickoff
a, b = sys.argv[1], sys.argv[2]; N = 1500; wr = wn = None
def pfx(d): return "f" if _fp(d, "f", 1) else "p"
PFX_A, PFX_B = pfx(a), pfx(b)
for i, x in enumerate(sys.argv):
    if x == "--frames": N = int(sys.argv[i + 1])
    if x == "--offset": OFF = int(sys.argv[i + 1])
    if x == "--auto-offset": AUTO = int(sys.argv[i + 1])   # G2E10 tickoff
    if x == "--ref-extra": REFX = sys.argv[i + 1]          # G2E10 tickoff: reference frames outside 1..N (cs_align.py EXTRA)
    if x == "--wav-ref": wr = sys.argv[i + 1]
    if x == "--wav-new": wn = sys.argv[i + 1]
# G2E10 tickoff (Claude sub-agent, 27 Oct 2026; Karl 27 Oct): --auto-offset K picks ONE constant tick offset for the whole run, the one
# with the best mean CONTENT match over -K..+K (ties: the smaller |offset|, so 0 wins a tie), using only offsets for which every
# reference frame the judge needs exists (frames outside 1..N come from --ref-extra, written by cs_align.py). The thresholds and every
# measure below are unchanged; the chosen offset applies to SSIM, ACTIVE, CONTENT and COLOUR alike and is printed as TICK_OFFSET.
# With offset 0 every loop below is exactly the old one (reference ticks 1..N).
def _fa(t):
    q = _fp(a, PFX_A, t)
    if not q and REFX: q = _fp(REFX, PFX_A, t)
    return q
CMC = {}; AUTO_SCORES = {}
def _cmx(x, y, bg, tol=32):   # the CONTENT measure below (_cm), defined here because the offset is chosen before it is defined
    m = (np.abs(x.astype(int) - bg).max(2) > 0) | (np.abs(y.astype(int) - bg).max(2) > 0)
    if m.mean() < 0.002: return 1.0
    return float((np.abs(x.astype(int) - y.astype(int)).max(2) <= tol)[m].mean())
if AUTO is not None:
    PC = {}
    for t in range(1 - AUTO, N + AUTO + 1):
        q = _fa(t)
        if not q: continue
        ref = _img(q); v, c = np.unique(ref.reshape(-1, 3), axis=0, return_counts=True); bg = v[c.argmax()].astype(int)
        for u in range(t - 1 - AUTO, t + 2 + AUTO):
            if 1 <= u <= N and (t, u) not in CMC:
                if u not in PC:
                    qb = _fp(b, PFX_B, u); PC[u] = _img(qb) if qb else None
                if PC[u] is not None: CMC[(t, u)] = _cmx(ref, PC[u], bg)
        for k in [k for k in PC if k < t - AUTO - 2]: del PC[k]
    for o in range(-AUTO, AUTO + 1):
        tr = range(1 - o, N + 1 - o)
        if not all(_fa(t) for t in tr): continue
        AUTO_SCORES[o] = sum(max([CMC.get((t, t + d + o), 0.0) for d in (-1, 0, 1)]) for t in tr) / float(N)
    # G2E10 judgemargin (Claude sub-agent, 27 Oct 2026): ties go to 0 for real. A non-zero offset is chosen only if its mean content
    # match beats offset 0's by at least MARGIN; among the non-zero offsets that do, the smallest |offset| (then the lower one) whose
    # mean is within MARGIN of the best of them wins. Campaign 74: 77 of 126 non-zero picks beat 0 by < 0.002 (noise).
    MARGIN = 0.002
    base0 = AUTO_SCORES.get(0)
    cand = [o for o in AUTO_SCORES if o != 0 and (base0 is None or AUTO_SCORES[o] >= base0 + MARGIN - 1e-9)]
    if base0 is not None and not cand: OFF = 0
    elif cand:
        best = max(AUTO_SCORES[o] for o in cand)
        OFF = min((o for o in cand if AUTO_SCORES[o] >= best - MARGIN + 1e-9), key=lambda o: (abs(o), o))
    else: OFF = 0
    OFF_MARGIN = (AUTO_SCORES[OFF] - base0) if (base0 is not None and OFF in AUTO_SCORES) else None
TR = range(1 - OFF, N + 1 - OFF)
RLO, RHI = (1, N) if OFF == 0 else (1 - OFF - 1, N - OFF + 1)
S = []; worst = []
for t in TR:
    ref = _img(_fa(t)); best = 0
    for d in (-1, 0, 1):
        u = t + d + OFF
        if 1 <= u <= N and _fp(b, PFX_B, u): best = max(best, ssim(ref, _img(_fp(b, PFX_B, u))))
    S.append(best)
S = np.array(S); ok = float((S >= 0.90).mean())
act = []                                        # active ticks: the reference picture changed since the previous tick (a static title screen cannot show a timing error)
prev = None
for t in TR:
    cur = _img(_fa(t)); act.append(prev is not None and bool((cur != prev).any())); prev = cur
act = np.array(act); Sa = S[act] if act.any() else S
# CONTENT (Cowork 16 Oct 2026, Karl approved "code check + JEV"): SSIM is fooled by mostly-black Amiga screens (a port missing
# a logo, a "PRESENTS" line or the whole menu scored 0.92-0.99). Content match = share of the pixels that are not background
# (the reference's commonest colour) in EITHER picture that match within 32 per channel; best of ticks t-1, t, t+1 as SSIM.
# Validated on 38 hand-checked pairs: every identical pair 1.000, every visibly different one <= 0.96.
def _cm(x, y, bg, tol=32):
    m = (np.abs(x.astype(int) - bg).max(2) > 0) | (np.abs(y.astype(int) - bg).max(2) > 0)
    if m.mean() < 0.002: return 1.0
    return float((np.abs(x.astype(int) - y.astype(int)).max(2) <= tol)[m].mean())
CM = []
for t in TR:
    ref = _img(_fa(t)); v, c = np.unique(ref.reshape(-1, 3), axis=0, return_counts=True); bg = v[c.argmax()].astype(int); best = 0
    for d in (-1, 0, 1):
        u = t + d + OFF
        if 1 <= u <= N and _fp(b, PFX_B, u): best = max(best, CMC[(t, u)] if (t, u) in CMC else _cm(ref, _img(_fp(b, PFX_B, u)), bg))
    CM.append(best)
CM = np.array(CM); cok = float((CM >= 0.95).mean())
# COLOUR (Cowork hj 20 Oct 2026, Karl approved "make the judge colour-aware"): CONTENT above compares colours per channel (tol 32)
# but only one way: each REFERENCE tick looks for its best port tick, so a port frame whose colours the original never shows near
# that moment is never looked at. COLOUR runs the same per-channel match the other way: each PORT tick against reference ticks
# u-1..u+1 (background = that reference tick's commonest colour). PICTURE now needs COLOUR >= 0.95 on 95% of ticks as well.
# --legacy restores the 15-18 Oct judge exactly (no COLOUR line; PICTURE = SSIM and CONTENT). Measured before deploy on 41 games.
LEGACY = "--legacy" in sys.argv; rok = 1.0
if not LEGACY:
    BGC = {}
    def _bg(u):
        if u not in BGC:
            v, c = np.unique(_img(_fa(u)).reshape(-1, 3), axis=0, return_counts=True); BGC[u] = v[c.argmax()].astype(int)
        return BGC[u]
    CR = []
    for u in range(1, N + 1):
        q = _fp(b, PFX_B, u); best = 0
        if q:
            nat = _img(q)
            for d in (-1, 0, 1):
                t = u + d - OFF
                if RLO <= t <= RHI and _fa(t): best = max(best, _cm(_img(_fa(t)), nat, _bg(t)))
        CR.append(best)
    CR = np.array(CR); rok = float((CR >= 0.95).mean())
print("CONTENT: %d ticks, content match >= 0.95 on %.1f%% of ticks (need 95%%), mean %.4f, min %.4f -> %s" % (N, cok * 100, CM.mean(), CM.min(), "PASS" if cok >= 0.95 else "FAIL"))
if not LEGACY: print("COLOUR: %d port ticks, reverse content match >= 0.95 on %.1f%% of ticks (need 95%%), mean %.4f, min %.4f, worst port ticks %s -> %s" % (N, rok * 100, CR.mean(), CR.min(), [(int(i) + 1, round(float(CR[i]), 3)) for i in np.argsort(CR)[:4]], "PASS" if rok >= 0.95 else "FAIL"))
print("PICTURE: %d ticks, SSIM >= 0.90 on %.1f%% of ticks (need 95%%), mean %.4f, min %.4f -> %s" % (N, ok * 100, S.mean(), S.min(), "PASS" if (ok >= 0.95 and cok >= 0.95 and rok >= 0.95) else "FAIL"))
print("ACTIVE ticks (reference picture changed): %d of %d, SSIM >= 0.90 on %.1f%% of them, mean %.4f, min %.4f" % (int(act.sum()), N, 100 * float((Sa >= 0.90).mean()), Sa.mean(), Sa.min()))
if AUTO is not None: print("TICK_OFFSET: %+d (auto over -%d..+%d; margin over offset 0 %s, need %.3f; mean content by offset %s)" % (OFF, AUTO, AUTO, "%+.4f" % OFF_MARGIN if OFF_MARGIN is not None else "none", MARGIN, {"%+d" % o: round(AUTO_SCORES[o], 4) for o in sorted(AUTO_SCORES)}))   # G2E10 tickoff
print("worst ticks:", [(int(i) + 1, round(float(S[i]), 3)) for i in np.argsort(S)[:6]])
if wr and wn:
    A, B = wav(wr), wav(wn); n = min(len(A), len(B)) // 882 * 882; A, B = A[:n].mean(1), B[:n].mean(1); T = n // 882; cs = []
    for t in range(T):
        x, y = A[t * 882:(t + 1) * 882], B[t * 882:(t + 1) * 882]
        sx, sy = np.log1p(np.abs(np.fft.rfft(x * np.hanning(882)))), np.log1p(np.abs(np.fft.rfft(y * np.hanning(882))))
        ex, ey = float(np.abs(x).mean()), float(np.abs(y).mean())
        if ex < 1 and ey < 1: cs.append(1.0); continue
        sx, sy = sx - sx.mean(), sy - sy.mean(); den = np.sqrt((sx * sx).sum() * (sy * sy).sum())
        cs.append(float((sx * sy).sum() / den) if den > 0 else 0.0)
    cs = np.array(cs); print("SOUND: %d ticks, mean spectral correlation %.3f, %.1f%% of ticks >= 0.85 -> %s" % (T, cs.mean(), 100 * float((cs >= 0.85).mean()), "PASS" if cs.mean() >= 0.85 else "FAIL"))
