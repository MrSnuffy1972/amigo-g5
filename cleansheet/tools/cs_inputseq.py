"""cs_inputseq.py F NAT_OUT REF_OUT (Cowork 18 Oct 2026, input discovery v1): write the input scripts for one clean-sheet run.
Reads this game's winner from cs_tools/cs_inputs.json (key = prekit.json "project"), found on the reference by
/srv/fast/tmp/inputdisc/inputdisc.py. The SAME sequence goes to both sides at the same frame offset from the snapshot:
native port frame = reference frame - F (the port starts at the snapshot), reference harness frame = reference frame.
Token syntax: when the kit's harness.c AND main.c both parse the raw-key tokens (K = tap, P = down, Q = up, hex Amiga
keycode), keys are held for exactly the discovered number of frames on both sides. Without them only fire, joystick
directions, space (old 'S') and F1-F8 (old '1'-'8') can be said, as taps; anything else is unsupported.
exit 0 scripts written, 2 no winner for this game, 1 cannot be expressed with this kit (reason printed)."""
import json, os, re, sys
T = os.path.dirname(os.path.abspath(__file__))
KEYS = {"space": 0x40, "return": 0x44, "escape": 0x45, "d1": 0x01, "d2": 0x02, "d3": 0x03, "d4": 0x04,
        "f1": 0x50, "f2": 0x51, "f3": 0x52, "f4": 0x53, "f5": 0x54, "f6": 0x55, "f7": 0x56, "f8": 0x57}


def raw_ok():
    k = "/w/kit/amigo/port"
    try:
        return all(re.search(r"==\s*'P'", open(os.path.join(k, f)).read()) and "'Q'" in open(os.path.join(k, f)).read() for f in ("harness.c", "main.c"))
    except OSError:
        return False


def mouse_ok():
    k = "/w/kit/amigo/port"
    try:
        return all("g39 mouse-port tokens" in open(os.path.join(k, f)).read() for f in ("harness.c", "main.c"))
    except OSError:
        return False


def steps(st, raw):
    """(frame, token) pairs relative to the step's reference frame 0, or None"""
    k, h = st["key"], max(1, int(st.get("hold", 2)))
    if k == "fire2": return [(0, "F"), (h, "-")]
    if k in ("up_fire", "down_fire"):
        d = "U" if k == "up_fire" else "D"
        return [(0, d), (h, "-"), (h + 2, "F"), (2 * h + 2, "-")]
    if k in ("up", "down", "left", "right"): return [(0, k[0].upper()), (h, "-")]
    if k in ("fire1", "fire1_alt", "fire2_alt", "mouse_move"):   # g39 mouse-port tokens: need the g39 parser in harness.c AND main.c
        if not mouse_ok(): return None
        if k == "mouse_move": return [(i, "M") for i in range(h)] + [(h, "-")]
        return [(0, {"fire1": "V", "fire1_alt": "W", "fire2_alt": "X"}[k]), (h, "-")]
    if k in KEYS:
        if raw: return [(0, "P%02X" % KEYS[k]), (h, "Q%02X" % KEYS[k])]
        if k == "space": return [(0, "S")]
        if k[0] == "f": return [(0, k[1])]
    return None


def main():
    F = int(sys.argv[1]); nat, ref = sys.argv[2], sys.argv[3]
    try:
        pid = str(json.load(open("/w/prekit.json"))["project"])
        db = json.load(open(os.path.join(T, "cs_inputs.json")))
    except Exception as ex:
        print("no input table or game id: %s" % ex); sys.exit(2)
    g = db.get(pid)
    if not g or not g.get("seq"):
        print("game %s: no discovered input" % pid); sys.exit(2)
    raw = raw_ok(); N, R = [], []
    for st in g["seq"]:
        ev = steps(st, raw)
        if ev is None:
            print("game %s: key %s needs the raw-key or g39 mouse tokens, which this kit's harness.c/main.c do not parse yet" % (pid, st["key"])); sys.exit(1)
        rf = int(st["ref_frame"]); pf = rf - F
        if pf < 1: rf, pf = F + 1, 1
        N += ["%d %s" % (pf + d, t) for d, t in ev]; R += ["%d %s" % (rf + d, t) for d, t in ev]
    open(nat, "w").write("\n".join(N) + "\n"); open(ref, "w").write("\n".join(R) + "\n")
    print("game %s: %s, raw tokens %s, snapshot frame %d\nport:  %s\nref:   %s" % (pid, g["seq"], raw, F, " | ".join(N), " | ".join(R)))
    sys.exit(0)


if __name__ == "__main__":
    main()
