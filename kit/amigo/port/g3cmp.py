#!/usr/bin/env python3
"""g3cmp.py DIR FRAMES: G3 by code, the comparison half of `amigo_port.sh g3` (Ami-Go port kit, 13 Oct 2026).

Karl chose, 13 Oct: G3 for library games is the reference-interpreter match. The port (out/headless) and the
original 68000 code on Musashi (the harness) run against the same chipset runtime with the same input, so any
difference is the translation's fault, not emulator differences or input timing. Compares:
  pictures  every frame's hash (hash_frame in runtime/chipset.c), port.hash against ref.hash
  sound     the mixed Paula output, port.wav (44-byte header) against ref.raw, as a correlation of 20 ms energy
A run in which the original shows fewer than 8 different pictures proves nothing (black or frozen screen) and
fails as not conclusive. Writes DIR/result.json; prints the G3_MATCH line g3check.py reads."""
import array, json, math, os, sys

d, n = sys.argv[1], int(sys.argv[2])


def hashes(p):
    out = {}
    try:
        for l in open(p):
            f = l.split()
            if len(f) >= 2:
                out[int(f[0])] = f[1]
    except FileNotFoundError:
        pass
    return out


def energy(p, skip):
    try:
        b = open(p, "rb").read()[skip:]
    except FileNotFoundError:
        return []
    a = array.array("h"); a.frombytes(b[:len(b) // 2 * 2])
    win = 882 * 2                       # 20 ms of stereo samples at 44.1 kHz
    return [math.sqrt(sum(x * x for x in a[i:i + win:2]) / (win / 2)) for i in range(0, len(a) - win + 1, win)]


def corr(x, y):
    m = min(len(x), len(y))
    if m < 10:
        return None
    x, y = x[:m], y[:m]
    mx, my = sum(x) / m, sum(y) / m
    sxy = sum((a - mx) * (b - my) for a, b in zip(x, y))
    sx = math.sqrt(sum((a - mx) ** 2 for a in x)); sy = math.sqrt(sum((b - my) ** 2 for b in y))
    return sxy / (sx * sy) if sx > 0 and sy > 0 else (1.0 if sx == sy == 0 else 0.0)


port, ref = hashes(os.path.join(d, "port.hash")), hashes(os.path.join(d, "ref.hash"))
frames = list(range(1, n + 1))
matched = sum(1 for f in frames if f in port and port.get(f) == ref.get(f))
first_miss = next((f for f in frames if port.get(f) != ref.get(f)), None)
distinct = len(set(ref.values()))
pe, re_ = energy(os.path.join(d, "port.wav"), 44), energy(os.path.join(d, "ref.raw"), 0)
ref_loud = max(re_) if re_ else 0.0
c = corr(pe, re_)
res = {"frames": n, "port_frames": len(port), "ref_frames": len(ref), "matched": matched,
       "fraction": matched / n if n else 0.0, "first_miss": first_miss, "ref_distinct_pictures": distinct,
       "sound_ref_peak_rms": round(ref_loud, 1), "sound_corr": None if c is None else round(c, 4),
       "conclusive": distinct >= 8 and len(ref) >= n * 0.9}
json.dump(res, open(os.path.join(d, "result.json"), "w"), indent=1)
note = "" if res["conclusive"] else " NOT CONCLUSIVE: the original showed %d different pictures in %d frames" % (distinct, len(ref))
print("G3_MATCH %d/%d = %.4f against interpreter; first miss: frame %s; original shows %d different pictures; "
      "sound correlation %s (original peak RMS %.0f)%s" % (matched, n, res["fraction"], first_miss, distinct,
                                                          res["sound_corr"], ref_loud, note))
