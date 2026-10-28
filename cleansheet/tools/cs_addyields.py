"""cs_addyields.py: yield discovery for the clean-sheet v0 run (Cowork 15 Oct, v0c; filter v0d 21:10). The native runtime's watchdog
writes every busy loop that spun a whole tick without yielding to out/yield_candidates.txt. Add the NEW ones to /w/game.json yields
so the next translate makes them wait for the next tick instead of burning the frame. Exit 1 when nothing new is added.
v0d filter (Gemini 3.1 Pro picture-fail answer, checked 80/80 quotes, 15 Oct): 6 of 16 picture-fail games had yields on WORK loops
(dbra copy/decrunch/plot loops at $00177A, $0400B2, $005A8C, $058144 ...), so one 1 KB copy took hundreds of ticks. A candidate is
now accepted only if it sits in a short backward loop whose body writes no ordinary memory (writes to $DFFxxx/$BFxxxx hardware are
fine): a real wait polls and compares, it does not store. Candidates whose code is not in the snapshot are accepted as before and
marked unverified. Rejections are logged with the reason.
v0e (22:50): also rejects loops that look like work: dbcc, shifts/rotates/addx, calls, address-register advances (8 games in Gemini's second picture-fail batch).
v0f (18 Oct, Karl approved): a conditional loop that reads no memory and no hardware, and changes a register, is computation, never
a wait: its exit can only depend on registers it modifies itself, so nothing outside the loop can end it sooner. Example: Xenon 2
(game 20) $DCC add.l d1,d1 / add.l d0,d0 / bcc.s $DCC, a normalise/bit-scan loop; yielding there jumped virtual time. An
unconditional self-loop (bra.s *) and a loop that changes no register are left as they were (idle loops waiting for interrupts)."""
import json, os, sys
WIDEN = os.environ.get("CS_YIELD_WIDEN", "1") != "0"   # g41 (19 Oct): cross jsr/bsr into helpers, stack pushes are not stores
p = "/w/out/yield_candidates.txt"
if not os.path.exists(p): sys.exit(1)
cand = []
for l in open(p):
    l = l.strip().split()[0] if l.strip() else ""
    if l and l not in cand: cand.append(l)
try:
    import capstone
    md = capstone.Cs(capstone.CS_ARCH_M68K, capstone.CS_MODE_M68K_000)
except Exception as e:
    md = None; print("yield filter OFF (no capstone: %s)" % e)
def rd(f):
    try: return open(f, "rb").read()
    except Exception: return b""
chip, slow = rd("/w/out/snap/snap_chip.bin"), rd("/w/out/snap/snap_slow.bin")
def mem(a, n):
    if a + n <= len(chip): return chip[a:a + n]
    if 0xC00000 <= a and a + n <= 0xC00000 + len(slow): return slow[a - 0xC00000:a - 0xC00000 + n]
    return b""
READONLY = ("tst", "cmp", "btst", "nop", "chk")
REGS = {"d%d" % i for i in range(8)} | {"a%d" % i for i in range(8)} | {"sr", "ccr", "usp", "sp"}
def last_operand(ops):
    depth, cut = 0, -1
    for i, ch in enumerate(ops):
        if ch == "(": depth += 1
        elif ch == ")": depth -= 1
        elif ch == "," and depth == 0: cut = i
    return ops[cut + 1:].strip() if cut >= 0 else ops.strip()
def is_branch(m):
    return (m.startswith("b") and not m.startswith(("bset", "bclr", "bchg", "btst", "bsr"))) or m.startswith("db")   # dsfix3 18 Oct (G5): bsr is a call, not a loop branch
def target(ins):
    t = ins.op_str.split(",")[-1].strip().lstrip("$")
    try: return int(t, 16)
    except ValueError: return None
def stack_op(d):
    d = d.replace(" ", "").lower()
    return d in ("-(a7)", "(a7)+", "-(sp)", "(sp)+")
def writes_memory(ins):
    m = ins.mnemonic
    if WIDEN and m.startswith("pea"): return None                                  # g41: a push to the stack
    if m.startswith(READONLY) or is_branch(m) or m.startswith(("jmp", "jsr", "bsr", "rts", "rte")): return None   # dsfix3: bsr left is_branch, keep it out of memory writes
    if not ins.op_str: return None
    d = last_operand(ins.op_str).lower()
    if d in REGS or d.startswith("#"): return None
    if WIDEN and stack_op(d): return None                                          # g41: move.l d0,-(a7) is stack traffic, not a store
    if d.startswith("$"):
        try:
            a = int(d.lstrip("$").split(".")[0].split("(")[0], 16) & 0xFFFFFF
            if 0xDFF000 <= a < 0xE00000 or 0xBF0000 <= a < 0xC00000: return None   # custom chips / CIAs
        except ValueError: pass
    return "%06X %s %s" % (ins.address, m, ins.op_str)
WORK_MNEM = ("lsr", "lsl", "asr", "asl", "ror", "rol", "roxr", "roxl", "addx", "subx", "jsr", "bsr")   # dsfix3 18 Oct (G5): lea only counts as work when it advances its own register (below)
def work_sign(ins):
    """v0e (Gemini picture-fail2, 15 Oct 22:50): a wait loop polls and compares. Bit-shifting decrunch loops
    (lsr/roxl + dbra, store outside the inner loop), pointer-advance scans (addq #2,a0 / lea 2(a0),a0 / (a0)+) and loops
    that call subroutines are work; so is any dbcc-counted loop (counted delays are work too: Powermonger $1368)."""
    m, ops = ins.mnemonic, ins.op_str.lower()
    if m.startswith("db"): return "counted dbcc loop"
    if WIDEN and m.startswith(("jsr", "bsr")): return None                         # g41: calls are followed by call_ok() instead
    if m.startswith(WORK_MNEM): return m
    if m.startswith("lea"):
        src, dst = ops.rsplit(",", 1) if "," in ops else (ops, "")
        if dst.strip() and "(" + dst.strip() + ")" in src.replace(" ", ""): return "lea pointer advance"
    if WIDEN and all(stack_op(o) or ")+" not in o and "-(" not in o for o in split_ops(ops)): pass   # g41: only stack pointer pre-dec/post-inc
    elif ")+" in ops or "-(" in ops: return "address register post-increment/pre-decrement"
    if m.startswith(("addq", "subq", "adda", "suba", "add.", "sub.")) and last_operand(ops) in {"a%d" % i for i in range(8)}: return "address register advance"
    return None
def split_ops(ops):
    out, depth, cur = [], 0, ""
    for ch in ops:
        if ch == "(": depth += 1
        elif ch == ")": depth -= 1
        if ch == "," and depth == 0: out.append(cur.strip()); cur = ""
        else: cur += ch
    if cur.strip(): out.append(cur.strip())
    return out
def reg_or_imm(op):
    op = op.strip().lower()
    return op.startswith("#") or all(t.strip() in REGS for t in op.replace("-", "/").split("/"))
def register_only_compute(body, br):
    """v0f: the reason string when the loop reads no memory/hardware and modifies a register under a conditional exit, else None."""
    if br.mnemonic.startswith(("bra", "jmp")): return None
    changed = []
    for b in body:
        if is_branch(b.mnemonic): continue
        ops = split_ops(b.op_str)
        if not all(reg_or_imm(o) for o in ops): return None
        if ops and not b.mnemonic.startswith(READONLY) and not ops[-1].startswith("#"): changed.append(ops[-1].lower())
    if not changed: return None
    return "register-only computation, reads no memory or hardware, changes %s" % ",".join(sorted(set(changed)))
def call_target(ins):
    """absolute target of jsr/bsr, or None (jsr (a0), jsr -$xx(a6) and other computed calls stay unknown)"""
    t = ins.op_str.strip().lower()
    if ins.mnemonic.startswith("bsr"): return target(ins)
    if t.startswith("$") and "(" not in t:
        try: return int(t.lstrip("$").split(".")[0], 16) & 0xFFFFFF
        except ValueError: return None
    return None
def call_ok(ins, depth, seen):
    """g41: (None, None) when the callee, followed linearly to its rts (forward branches skipped over, depth 15, $100 bytes per level), has
    no store, no work sign and no unknown call; else the reason string."""
    t = call_target(ins)
    if t is None: return "computed call %s %s at %06X" % (ins.mnemonic, ins.op_str, ins.address)
    if depth > 15: return "call depth over 15 at %06X" % ins.address
    if t in seen: return None
    seen.add(t)
    code = mem(t, 0x100)
    if not code or not any(code): return "callee $%06X not in snapshot" % t
    for b in md.disasm(code, t):
        if b.mnemonic.startswith(("rts", "rtr")): return None
        if b.mnemonic.startswith(("rte", "jmp", "trap", "stop", "illegal")): return "callee $%06X leaves by %s at %06X" % (t, b.mnemonic, b.address)
        w = writes_memory(b)
        if w: return "callee $%06X stores to memory: %s" % (t, w)
        why = work_sign(b)
        if why: return "callee $%06X is work (%s at %06X)" % (t, why, b.address)
        if b.mnemonic.startswith(("jsr", "bsr")):
            r = call_ok(b, depth + 1, seen)
            if r: return r
    return "callee $%06X: no rts within $100 bytes" % t
def judge(c):
    """(accept, reason)"""
    if md is None: return True, "unfiltered"
    code = mem(c, 0x60)
    if not code or not any(code): return True, "unverified (not in snapshot)"
    for ins in md.disasm(code, c):
        if ins.mnemonic.startswith(("rts", "rte", "jmp")): return False, "no backward loop before %s at %06X (leaf/helper code)" % (ins.mnemonic, ins.address)
        if is_branch(ins.mnemonic):
            t = target(ins)
            if t is not None and c - 0x80 <= t <= c:
                body = list(md.disasm(mem(t, ins.address + ins.size - t), t))
                for b in body:
                    w = writes_memory(b)
                    if w: return False, "loop $%06X-$%06X stores to memory: %s" % (t, ins.address, w)
                    why = work_sign(b)
                    if why: return False, "loop $%06X-$%06X is work, not a wait (%s at %06X)" % (t, ins.address, why, b.address)
                    if WIDEN and b.mnemonic.startswith(("jsr", "bsr")):
                        r = call_ok(b, 1, set())
                        if r: return False, "loop $%06X-$%06X calls work: %s" % (t, ins.address, r)
                why = register_only_compute(body, ins)
                if why: return False, "loop $%06X-$%06X is computation, not a wait (%s)" % (t, ins.address, why)
                return True, "wait loop $%06X-$%06X (no stores%s)" % (t, ins.address, ", calls checked" if WIDEN and any(b.mnemonic.startswith(("jsr", "bsr")) for b in body) else "")
    return False, "no backward loop within $60 bytes"
cfg = json.load(open("/w/game.json"))
have = {int(str(y), 16) for y in cfg.get("yields", [])}
rej = set(cfg.get("_yields_rejected", []))
new = []
for c in cand:
    a = int(c, 16)
    if a in have or c in rej: continue
    ok, why = judge(a)
    print("yield %s %s: %s" % (c, "ACCEPT" if ok else "REJECT", why))
    if ok: new.append(c)
    else: rej.add(c)
cfg["_yields_rejected"] = sorted(rej)
json.dump(cfg, open("/w/game.json", "w"), indent=1)
os.rename(p, p + ".%d" % (len(cfg.get("yields", [])) + len(new)))     # keep the evidence; the next run writes a fresh file
if not new: sys.exit(1)
cfg["yields"] = cfg.get("yields", []) + ["0x%s" % c for c in new]
json.dump(cfg, open("/w/game.json", "w"), indent=1)
print("added yields:", " ".join(new), "-> total", len(cfg["yields"]))
