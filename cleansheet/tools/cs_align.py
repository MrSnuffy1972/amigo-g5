"""cs_align.py F N: the harness ran from power-on; the native exe starts at the snapshot (harness frame F). Copy harness frames
F+1..F+N to /tmp/ref/f00001..f0000N and cut the first F frames of its sound, so judge.py compares tick t with tick t."""
import os, shutil, sys
F, N = int(sys.argv[1]), int(sys.argv[2])
EXTRA = int(sys.argv[3]) if len(sys.argv) > 3 else 0   # G2E10 tickoff: reference frames -EXTRA+1..0 and N+1..N+EXTRA to /tmp/refx
n = 0
for t in range(1, N + 1):
    for e in ("rgba", "png", "ppm"):   # Cowork 18 Oct: frames may be raw RGBA, PNG or legacy PPM
        s = "/tmp/refall/f%05d.%s" % (F + t, e)
        if os.path.exists(s):
            shutil.copy(s, "/tmp/ref/f%05d.%s" % (t, e)); n += 1; break
if EXTRA:   # G2E10 tickoff (Claude sub-agent, 27 Oct 2026): the judge's per-game tick offset needs the reference ticks beside the window;
    # they go to /tmp/refx (not /tmp/ref, which keeps exactly ticks 1..N for every other reader). Needs the harness run to F+N+EXTRA.
    os.makedirs("/tmp/refx", exist_ok=True); nx = 0
    for t in list(range(1 - EXTRA, 1)) + list(range(N + 1, N + EXTRA + 1)):
        for e in ("rgba", "png", "ppm"):
            s_ = "/tmp/refall/f%05d.%s" % (F + t, e)
            if F + t >= 1 and os.path.exists(s_):
                shutil.copy(s_, "/tmp/refx/f%05d.%s" % (t, e)); nx += 1; break
    print("tickoff: %d extra reference frames (ticks %d..0 and %d..%d) to /tmp/refx" % (nx, 1 - EXTRA, N + 1, N + EXTRA))
FB = 882 * 4
# G3E1 audalign (Claude sub-agent 26 Oct 2026): the harness writes 885.35 samples per frame (44.1 kHz against a 312.5-line PAL frame),
# not 882, so cutting F * 882 samples left the reference sound F/263 ticks early (Lotus F=730: 2.8 ticks, sound 0.840; aligned 0.985).
# The cut is now F frames' worth of the run's own sample rate: round(F * samples / (F + N)). Off: /w/cs_audalign = 0 or CS_AUDALIGN=0.
def _audalign_on():
    try: f = open("/w/cs_audalign").read().strip()
    except OSError: f = ""
    return os.environ.get("CS_AUDALIGN", f or "1") != "0"
if os.path.exists("/tmp/refall.raw"):
    S = os.path.getsize("/tmp/refall.raw") // 4
    cut = int(round(F * S / float(F + N + EXTRA))) * 4 if _audalign_on() and F + N + EXTRA > 0 else F * FB   # G2E10 tickoff: the run is F+N+EXTRA frames long
    with open("/tmp/refall.raw", "rb") as a, open("/tmp/ref.raw", "wb") as b:
        a.seek(cut); b.write(a.read())
    print("reference sound cut at sample %d (%s)" % (cut // 4, "audalign" if cut != F * FB else "F*882"))
print("aligned %d reference frames from harness frame %d" % (n, F + 1))
