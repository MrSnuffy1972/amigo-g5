"""cs_pickpc.py TRIAGE: choose the snapshot PC = where the game's own code first runs after its loader, from the harness's
first-execution order (out/port/ref/first_bytes.txt is written in the order PCs first execute).
whdload: the first PC in chip RAM at or above $400 (the slave and its resload live at $C7Exxx; IK+'s answer is $600).
filed:   the first PC inside one of game.json's loaded segments (the game's hunks), skipping OSHLE's own stubs.
whdload, v0d (Cowork 15 Oct 21:10): when the reference log (argv[2], run with WHDTIME=1) shows resload DiskLoad/LoadFile/
LoadFileOffset/LoadFileDecrunch/Decrunch/Patch calls, print "frame N" instead: N = 2 frames after the last call of the startup
burst (calls separated by gaps under 100 frames, starting before frame 1000). The snapshot is then taken at the first main-line
game instruction from frame N (harness SNAP_FRAME). Reason: 7 of 16 picture-fail games were snapshotted before those loads.
Prints the PC in hex (or "frame N") on stdout; reasons on stderr."""
import json, re, sys
T = sys.argv[1]
# Cowork 16 Oct 19:10: per-game experiment hook. /w/cs_snap_override holds "frame N" or a hex PC and wins over triage.
# Reason: the seglen fix showed $004008 IS the first instruction of game 135's own hunk (segment $4000-$EAEC), so the
# filed-game failure is not a wrong segment; testing whether a later snapshot (after the game's own init) avoids it.
try:
    _ov = open("/w/cs_snap_override").read().strip()
    if _ov:
        sys.stderr.write("override from /w/cs_snap_override: %s\n" % _ov); print(_ov); sys.exit(0)
except OSError:
    pass
if T != "filed" and len(sys.argv) > 2:
    fr = []
    try:
        for l in open(sys.argv[2], errors="replace"):
            m = re.match(r"\[(\d+):-?\d+\] WHD (DiskLoad|LoadFile|LoadFileOffset|LoadFileDecrunch|Decrunch|Patch)", l)
            if m: fr.append(int(m.group(1)))
    except OSError:
        pass
    fr.sort()
    if fr and fr[0] < 1000:
        end = fr[0]
        for f in fr[1:]:
            if f - end >= 100: break
            end = f
        sys.stderr.write("resload load calls at frames %s; startup burst ends at frame %d\n" % (fr[:20], end))
        if end > 0:
            print("frame %d" % (end + 2)); sys.exit(0)
        sys.stderr.write("burst ends at frame 0: keeping the PC method\n")
    # v0d2 21:50: a "frame 12" default for games without startup loads was tried and REVERTED: IK+ (the only C2 pass) then
    # snapshotted at frame 149 mid-routine ($001A2A) and failed (picture 0.3%). Run-time code is for the version-dispatch design.
order = []
for l in open("/w/out/port/ref/first_bytes.txt"):
    p = l.split()
    if p: order.append(int(p[0], 16))
sys.stderr.write("first-execution order: %d PCs\n" % len(order))
if T == "filed":
    cfg = json.load(open("/w/game.json"))
    rng = []
    for s in cfg.get("segments", []):
        try:
            a = int(str(s.get("addr")), 16) if isinstance(s.get("addr"), str) else int(s.get("addr"))
            n = int(s.get("length") or s.get("size") or 0)
            if not n and s.get("file"):   # Cowork 16 Oct: G1's game.json segments carry file + addr only; v0 read no length,
                import os                 # so EVERY filed game fell back to "first chip PC >= $1000" ($004008, OSHLE start-up)
                fp = s["file"] if s["file"].startswith("/") else os.path.join("/w", s["file"])
                n = os.path.getsize(fp) if os.path.exists(fp) else 0
                if s["file"].endswith("execlib.bin"): n = 0   # OSHLE's exec stand-in is not the game
            if n: rng.append((a, a + n))
        except Exception:
            pass
    sys.stderr.write("segments: %s\n" % [(hex(a), hex(b)) for a, b in rng][:12])
    hit = [pc for pc in order if any(a <= pc < b for a, b in rng)]
    if not hit:
        hit = [pc for pc in order if 0x1000 <= pc < 0x200000]
        sys.stderr.write("no executed PC inside the segments; falling back to the first chip PC >= $1000\n")
else:
    hit = [pc for pc in order if 0x400 <= pc < 0x200000]
    if not hit:   # G2E9 slowpick (Claude sub-agent, 27 Oct 2026): the game runs only in slow/expansion RAM. Measured: 437 (trackloader,
        # every one of 591 PCs at $C2xxxx) and 736 (WHDLoad, slave $CFFxxx, game at $C00000) failed "no candidate PC" although the
        # reference ran 1500 frames. Take the first PC in $C00000-$D80000 outside the WHDLoad slave/resload area ($Cxx000-$Cxx0FF
        # tower below the slave, the slave itself), which is where the snapshot config and the translator already put slow RAM.
        sl = None
        try:
            for l in open(sys.argv[2], errors="replace") if len(sys.argv) > 2 else []:
                m = re.search(r"slave \$([0-9A-F]{6}) \((\d+) bytes\)", l) or re.search(r"Patch list \$([0-9A-F]{6})", l)
                if m: sl = int(m.group(1), 16); break
        except OSError:
            pass
        if sl is None:   # G2E10 kickpick: kit-intent's whd log does not name the slave; G0 measured it (gates/G0/g0.json slave_addr)
            try: sl = int(json.load(open("/w/gates/G0/g0.json")).get("slave_addr") or 0) or None
            except Exception: sl = None
        lo = (sl & ~0xFFF) - 0x1000 if sl else None
        hit = [pc for pc in order if 0xC00000 <= pc < 0xD80000 and not (lo is not None and pc >= lo)]
        if hit: sys.stderr.write("no chip candidate: slow-RAM pick (slave area from $%06X excluded)\n" % (lo or 0))
_kr = None   # G2E10 kickpick: never a PC inside the player's Kickstart
try:
    for l in open(sys.argv[2], errors="replace") if len(sys.argv) > 2 else []:
        m = re.search(r"kickstart pcs \d+ \(at \$([0-9A-Fa-f]+), size \$([0-9A-Fa-f]+)", l)
        if m: _kr = (int(m.group(1), 16), int(m.group(1), 16) + int(m.group(2), 16))
except OSError:
    pass
if _kr: hit = [pc for pc in hit if not _kr[0] <= pc < _kr[1]]
if not hit:
    sys.stderr.write("no candidate PC\n"); sys.exit(1)
print("%06X" % hit[0])
sys.stderr.write("snapshot PC %06X (position %d in first-execution order)\n" % (hit[0], order.index(hit[0])))
