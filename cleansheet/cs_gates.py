"""cs_gates.py: record clean-sheet gates (Karl 15 Oct 19:20: "New CS-G2/CS-G3 gates", kept apart from the old G0-G5).
  CS-G4 = latest cs_g4_runs row class pass:native-windows (G4 install, 25 Oct 2026)
From each game's LATEST cleansheet_runs row, decided by code only:
  CS-G2 = out/native ran every tick (rc 0) and picture SSIM >= 0.90 on >= 95% of ticks, with a moving reference
          (active ticks >= max(5, ticks/20): a black or static screen on both sides is not a pass)
  CS-G3 = CS-G2 and sound mean spectral correlation >= 0.85
A gate row is added when first passed and never deleted; a later failing run marks it 'lost' with the time (history kept)."""
import json, sqlite3, time
db = sqlite3.connect("/srv/fast/amigo/amigo.db", timeout=60)
db.execute("""CREATE TABLE IF NOT EXISTS cs_gates (project_id INTEGER, gate TEXT, status TEXT, first_passed REAL, last_checked REAL,
              kit_rev TEXT, run_id INTEGER, evidence TEXT, PRIMARY KEY (project_id, gate))""")
rows = db.execute("SELECT id, project_id, kit_rev, result FROM cleansheet_runs WHERE id IN (SELECT MAX(id) FROM cleansheet_runs GROUP BY project_id)").fetchall()
now = time.time(); n = {"CS-G2": 0, "CS-G3": 0, "CS-G4": 0}
for rid, pid, rev, res in rows:
    r = json.loads(res or "{}")
    pic, act, snd = r.get("picture"), r.get("active"), r.get("sound")
    moving = bool(act) and int(act[0]) >= max(5, (r.get("native_frames") or 0) // 20)
    g2 = r.get("run_rc") == 0 and bool(pic) and pic[2] == "PASS" and moving and str(r.get("class", "")).startswith("pass")   # Cowork 16 Oct: the class carries the content + JEV verdicts
    g3 = g2 and bool(snd) and snd[1] == "PASS"
    for gate, ok in (("CS-G2", g2), ("CS-G3", g3)):
        ev = json.dumps({"picture": pic, "active": act, "sound": snd, "ticks": r.get("native_frames")})
        cur = db.execute("SELECT status FROM cs_gates WHERE project_id=? AND gate=?", (pid, gate)).fetchone()
        if ok:
            n[gate] += 1
            if cur: db.execute("UPDATE cs_gates SET status='pass', last_checked=?, kit_rev=?, run_id=?, evidence=? WHERE project_id=? AND gate=?", (now, rev, rid, ev, pid, gate))
            else: db.execute("INSERT INTO cs_gates VALUES (?,?,?,?,?,?,?,?)", (pid, gate, "pass", now, now, rev, rid, ev))
        elif cur and cur[0] == "pass":
            db.execute("UPDATE cs_gates SET status='lost', last_checked=?, kit_rev=?, run_id=?, evidence=? WHERE project_id=? AND gate=?", (now, rev, rid, ev, pid, gate))
# G4 install (Cowork 25 Oct 2026, Karl approved): CS-G4 = the latest cs_g4_runs row has class pass:native-windows (a Windows .exe of the
# clean-sheet native code, no emulator code inside, G2/G3 picture + sound check passed under wine). Same history rules as CS-G2/CS-G3.
try:
    g4rows = db.execute("SELECT id, project_id, kit_rev, class, result FROM cs_g4_runs WHERE id IN (SELECT MAX(id) FROM cs_g4_runs GROUP BY project_id)").fetchall()
except sqlite3.OperationalError:
    g4rows = []
for rid, pid, rev, cls, res in g4rows:
    r = json.loads(res or "{}"); ok = cls == "pass:native-windows"
    ev = json.dumps({"class": cls, "picture": r.get("picture"), "active": r.get("active"), "sound": r.get("sound"), "ticks": r.get("native_frames"), "symcheck": r.get("symcheck")})
    cur = db.execute("SELECT status FROM cs_gates WHERE project_id=? AND gate='CS-G4'", (pid,)).fetchone()
    if ok:
        n["CS-G4"] += 1
        if cur: db.execute("UPDATE cs_gates SET status='pass', last_checked=?, kit_rev=?, run_id=?, evidence=? WHERE project_id=? AND gate='CS-G4'", (now, rev, rid, ev, pid))
        else: db.execute("INSERT INTO cs_gates VALUES (?,?,?,?,?,?,?,?)", (pid, "CS-G4", "pass", now, now, rev, rid, ev))
    elif cur and cur[0] == "pass":
        db.execute("UPDATE cs_gates SET status='lost', last_checked=?, kit_rev=?, run_id=?, evidence=? WHERE project_id=? AND gate='CS-G4'", (now, rev, rid, ev, pid))
db.commit()
print("cs gates passing now:", n)
