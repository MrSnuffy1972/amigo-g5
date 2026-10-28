#!/usr/bin/env python3
"""m68k2rs.py: the Rust emitter for m68k2c.py (`--emit rust`). G5 milestone M2, 28 Oct 2026.

It shares m68k2c.py's decoder: every Insn the decoder produces carries its semantics as C statements (`ins.code`),
and this module turns exactly those statements into Rust, so the two outputs cannot disagree about what an
instruction means except through the transliteration here. Decision 28 Oct 2026: transliterate the decoder's C
text with a small expression parser now; give the decoder a structured form later if the subset grows past what
this handles. Anything the parser does not know is not guessed: the instruction becomes an arm that returns
Stop::Untranslated, and it is counted and listed.

Output is one `pub fn step(cpu, mem)` with a match arm per instruction address (the Rust form of the C
`emit_single` build). There is no fetch, decode or opcode table in it. It uses amigo_g5::cpu for the flag helpers.

Supported C subset: assignments and compound assignments to D[n], A[n], ea, ea2, s, t, r, PC_, the flags and
cpu.PC; `if (...) stmt [else stmt]` and blocks; block-local `uint32_t`/`int32_t` declarations; `for (; c; step)`;
`/` and `%`; the helper calls of runtime/cpu.h including cpu_exception; RB/RW/RL/WB/WW/WL; CYC += / -=;
`goto dyn_exit` (the only label the decoder emits). Literals take C's types (a plain decimal is a signed int), so
signed compares such as `q < -32768` in divs stay signed. Anything else is reported, not guessed.
Added 28 Oct 2026 (M2 completion): mul/div, chk, trap, trapv now translate; every opcode the decoder accepts does.
"""
import re

LOCALS = ('ea', 'ea2', 's', 't', 'r', 'PC_')
FLAGS = {'XF': 'x', 'NF': 'n', 'ZF': 'z', 'VF': 'v', 'CF': 'c'}
CPUFIELDS = {'PC': 'pc', 'S': 's', 'IM': 'im', 'OSP': 'osp', 'stopped': 'stopped'}
HELPERS_U32 = {'get_sr'}
HELPERS_VOID = {'set_sr', 'set_ccr'}
SIZED = ('add', 'sub', 'cmp', 'addx', 'subx', 'logic', 'asl', 'asr', 'lsl', 'lsr', 'rol', 'ror', 'roxl', 'roxr')
MEM_R = {'RB': ('read_u8', 'u8'), 'RW': ('read_u16', 'u16'), 'RL': ('read_u32', 'u32')}
MEM_W = {'WB': ('write_u8', 'u8'), 'WW': ('write_u16', 'u16'), 'WL': ('write_u32', 'u32')}
CASTS = {'uint32_t': 'u', 'int32_t': 'i', 'int16_t': 'h', 'int8_t': 'c', 'int': 'i', 'uint64_t': 'q'}


class Unsupported(Exception):
    pass


TOKEN = re.compile(r'\s*(0x[0-9A-Fa-f]+u?|\d+u?|[A-Za-z_][A-Za-z_0-9.]*|<<=|>>=|\|\||&&|==|!=|<=|>=|<<|>>|\+=|-=|\|=|&=|\^=|[-+*/%&|^~!<>=?:;,(){}\[\]])')


def tokenize(text):
    out, pos = [], 0
    text = text.strip()
    while pos < len(text):
        m = TOKEN.match(text, pos)
        if not m:
            raise Unsupported('cannot tokenise %r' % text[pos:pos + 12])
        out.append(m.group(1))
        pos = m.end()
    return out


# Expression types: 'u' u32, 'i' i32, 'h' i16, 'c' i8, 'b' bool, 'q' u64
RUST_TY = {'u': 'u32', 'i': 'i32', 'h': 'i16', 'c': 'i8', 'q': 'u64'}


def to_u(code, ty):
    if ty == 'u': return code
    return '(%s as u32)' % code


def conv(e, ty):
    """Value of expression e as Rust type ty, as a C assignment to a variable of that type converts it."""
    code, src = e
    if src == ty: return code
    if src == 'b': code = '(%s as u32)' % code
    return '(%s as %s)' % (code, RUST_TY[ty])


def promote(a, b):
    """C usual arithmetic conversions on (code, type) pairs for the types used here."""
    def norm(x):
        c, t = x
        if t == 'b': return ('(%s as i32)' % c, 'i')
        if t in ('h', 'c'): return ('(%s as i32)' % c, 'i')
        return x
    a, b = norm(a), norm(b)
    if a[1] == 'q' or b[1] == 'q':
        raise Unsupported('u64 arithmetic')
    if a[1] == 'u' or b[1] == 'u':
        return to_u(*a), to_u(*b), 'u'
    return a[0], b[0], 'i'


class P:
    def __init__(self, toks, locals_=None):
        self.t, self.i = toks, 0
        self.locals = {} if locals_ is None else locals_   # block-local C variables: name -> type

    def peek(self):
        return self.t[self.i] if self.i < len(self.t) else None

    def next(self):
        v = self.peek(); self.i += 1; return v

    def eat(self, s):
        if self.peek() != s: raise Unsupported('expected %r got %r' % (s, self.peek()))
        self.i += 1

    # ---- expressions
    def expr(self):
        c = self.binary(0)
        if self.peek() == '?':
            self.next(); a = self.expr(); self.eat(':'); b = self.expr()
            ca, cb, ty = promote(a, b)
            return ('(if %s { %s } else { %s })' % (self.as_bool(c), ca, cb), ty)
        return c

    PREC = [('||',), ('&&',), ('|',), ('^',), ('&',), ('==', '!='), ('<', '>', '<=', '>='), ('<<', '>>'), ('+', '-'), ('*', '/', '%')]

    def as_bool(self, e):
        c, t = e
        return c if t == 'b' else '(%s != 0)' % c

    def binary(self, lvl):
        if lvl == len(self.PREC): return self.unary()
        left = self.binary(lvl + 1)
        while self.peek() in self.PREC[lvl]:
            op = self.next(); right = self.binary(lvl + 1)
            left = self.apply(op, left, right)
        return left

    def apply(self, op, a, b):
        if op in ('||', '&&'):
            return ('(%s %s %s)' % (self.as_bool(a), op, self.as_bool(b)), 'b')
        if op in ('<<', '>>'):
            ca, ta = a
            if ta in ('b', 'h', 'c'): ca, ta = '(%s as i32)' % ca, 'i'
            return ('(%s %s (%s))' % (ca, op, to_u(*b)), ta)
        ca, cb, ty = promote(a, b)
        if op in ('==', '!=', '<', '>', '<=', '>='):
            return ('(%s %s %s)' % (ca, op, cb), 'b')
        # / and % only follow a zero-divisor check in the decoder's text (divu/divs raise vector 5 first)
        fn = {'+': 'wrapping_add', '-': 'wrapping_sub', '*': 'wrapping_mul', '/': 'wrapping_div', '%': 'wrapping_rem'}.get(op)
        if fn: return ('%s.%s(%s)' % (ca, fn, cb), ty)
        return ('(%s %s %s)' % (ca, op, cb), ty)

    def unary(self):
        t = self.peek()
        if t == '~':
            self.next(); c, ty = self.unary()
            return ('(!%s)' % c, ty) if ty in ('u', 'i') else ('(!%s)' % to_u(c, ty), 'u')
        if t == '!':
            self.next(); e = self.unary()
            return ('(!%s)' % self.as_bool(e), 'b')
        if t == '-':
            self.next(); c, ty = self.unary()
            if ty == 'i': return ('%s.wrapping_neg()' % c, 'i')
            return ('%s.wrapping_neg()' % to_u(c, ty), 'u')
        if t == '(' and self.i + 2 < len(self.t) and self.t[self.i + 1] in CASTS and self.t[self.i + 2] == ')':
            ty = CASTS[self.t[self.i + 1]]; self.i += 3
            c, src = self.unary()
            rust = RUST_TY[ty]
            if src == 'b': c, src = '(%s as u32)' % c, 'u'
            return ('(%s as %s)' % (c, rust), ty)
        return self.primary()

    def primary(self):
        t = self.next()
        if t is None: raise Unsupported('unexpected end')
        if t == '(':
            e = self.expr(); self.eat(')'); return e
        if re.match(r'(0x[0-9A-Fa-f]+|\d+)u?$', t):
            # C literal types: a `u` suffix is unsigned; otherwise int if it fits, and a hex literal that does not
            # fit is unsigned int (a decimal one would be long, which the decoder never writes)
            v = int(t.rstrip('u'), 0)
            if not t.endswith('u') and v <= 0x7FFFFFFF: return ('%si32' % t, 'i')
            if v > 0xFFFFFFFF or not (t.endswith('u') or t.startswith('0x')): raise Unsupported('literal %s' % t)
            return ('%su32' % t.rstrip('u'), 'u')
        if re.match(r'[A-Za-z_]', t):
            if self.peek() == '[':
                self.next(); idx = self.next(); self.eat(']')
                if t not in ('D', 'A') or not idx.isdigit(): raise Unsupported('index %s[%s]' % (t, idx))
                return ('cpu.%s[%s]' % (t.lower(), idx), 'u')
            if self.peek() == '(':
                return self.call(t)
            return self.name(t)
        raise Unsupported('token %r' % t)

    def name(self, t):
        if t in self.locals: return (t, self.locals[t])
        if t in LOCALS: return (t.lower(), 'u')
        if t in FLAGS: return ('cpu.' + FLAGS[t], 'u')
        if t.startswith('cpu.') and t[4:] in CPUFIELDS: return ('cpu.' + CPUFIELDS[t[4:]], 'u')
        if t == 'CYC': raise Unsupported('CYC in expression')
        raise Unsupported('name %s' % t)

    def args(self):
        self.eat('(')
        out = []
        if self.peek() != ')':
            out.append(self.expr())
            while self.peek() == ',':
                self.next(); out.append(self.expr())
        self.eat(')')
        return out

    def call(self, fn):
        a = self.args()
        if fn in MEM_R:
            return ('(mem.%s(%s)? as u32)' % (MEM_R[fn][0], to_u(*a[0])), 'u')
        if fn == 'SX8': return ('sx8(%s)' % to_u(*a[0]), 'u')
        if fn == 'SX16': return ('sx16(%s)' % to_u(*a[0]), 'u')
        if fn == 'cond': return ('cpu.cond(%s)' % to_u(*a[0]), 'b')
        if fn == 'cpu_exception': return ('cpu.exception(mem, %s, %s)?' % (to_u(*a[0]), to_u(*a[1])), 'u')
        if fn in HELPERS_U32: return ('cpu.%s()' % fn, 'u')
        m = re.match(r'([a-z]+)([124])$', fn)
        if m and m.group(1) in SIZED:
            return ('cpu.%s(%s)' % (fn, ', '.join(to_u(*x) for x in a)), 'u')
        if fn in ('abcd', 'sbcd'): return ('cpu.%s(%s)' % (fn, ', '.join(to_u(*x) for x in a)), 'u')
        raise Unsupported('call %s' % fn)

    # ---- statements
    def stmts_until(self, end):
        out = []
        while self.peek() != end:
            if self.peek() is None: raise Unsupported('missing %s' % end)
            out.append(self.stmt())
        return out

    def stmt(self):
        t = self.peek()
        if t == '{':
            self.next(); body = self.stmts_until('}'); self.eat('}')
            return '{ %s }' % ' '.join(body)
        if t == 'if':
            self.next(); self.eat('('); c = self.expr(); self.eat(')')
            body = self.stmt()
            if self.peek() == 'else':
                self.next(); other = self.stmt()
                return 'if %s { %s } else { %s }' % (self.as_bool(c), body, other)
            return 'if %s { %s }' % (self.as_bool(c), body)
        if t == 'return':
            self.next(); v = self.next(); self.eat(';')
            if v != '1': raise Unsupported('return %s' % v)
            return 'return Ok(());'
        if t == 'goto':
            self.next(); label = self.next(); self.eat(';')
            if label != 'dyn_exit': raise Unsupported('goto %s' % label)
            return '{ cpu.pc = pc_; return Ok(()); }'   # the C single build's dyn_exit: label
        if t in ('uint32_t', 'int32_t'):
            return self.declaration()
        if t == 'for':
            return self.for_loop()
        if t in ('while', 'int', 'do', 'switch', 'break', 'continue'):
            raise Unsupported(t)
        if t == 'CYC':
            self.next(); op = self.next(); e = self.expr(); self.eat(';')
            if op not in ('+=', '-='): raise Unsupported('CYC ' + op)
            fn = 'wrapping_add' if op == '+=' else 'wrapping_sub'
            return 'cpu.cyc = cpu.cyc.%s(%s as u64);' % (fn, to_u(*e))
        if t in MEM_W:
            self.next(); a = self.args(); self.eat(';')
            fn, ty = MEM_W[t]
            return 'mem.%s(%s, %s as %s)?;' % (fn, to_u(*a[0]), to_u(*a[1]), ty)
        if t in HELPERS_VOID or re.match(r'cmp[124]$', t or ''):
            self.next(); a = self.args(); self.eat(';')
            return 'cpu.%s(%s);' % (t, ', '.join(to_u(*x) for x in a))
        # an expression whose value is discarded, e.g. logic4(s);
        if self.t[self.i + 1:self.i + 2] == ['('] :
            self.next(); a = self.args(); self.eat(';')
            m = re.match(r'([a-z]+)([124])$', t)
            if m and m.group(1) in SIZED:
                return 'cpu.%s(%s);' % (t, ', '.join(to_u(*x) for x in a))
            raise Unsupported('call statement %s' % t)
        return self.assignment()

    def declaration(self):
        """`uint32_t a = x, b = y;` -> one `let mut` per name (block-scoped as in C, by Rust's own scoping)."""
        ty = 'u' if self.next() == 'uint32_t' else 'i'
        out = []
        while True:
            nm = self.next()
            if not nm or not re.match(r'[a-z][a-z0-9]*$', nm) or nm in LOCALS: raise Unsupported('declaration %s' % nm)
            self.eat('='); e = self.expr()
            self.locals[nm] = ty
            out.append('let mut %s: %s = %s;' % (nm, RUST_TY[ty], conv(e, ty)))
            if self.peek() == ';': self.next(); return ' '.join(out)
            self.eat(',')

    def for_loop(self):
        """`for (init; cond; step) body` -> `init; while cond { body step }` (the decoder's loops have no continue)."""
        self.next(); self.eat('(')
        init = '' if self.peek() == ';' else self.stmt()
        if not init: self.eat(';')
        cond = 'true' if self.peek() == ';' else self.as_bool(self.expr())
        self.eat(';')
        depth, toks = 0, []
        while not (depth == 0 and self.peek() == ')'):
            tok = self.next()
            if tok is None: raise Unsupported('for step')
            depth += {'(': 1, ')': -1}.get(tok, 0); toks.append(tok)
        self.eat(')')
        step = P(toks + [';'], self.locals).assignment() if toks else ''
        body = self.stmt()
        return '{ %s while %s { %s %s } }' % (init, cond, body, step)

    def assignment(self):
        lv = self.primary()
        op = self.next()
        if op not in ('=', '+=', '-=', '|=', '&=', '^=', '<<=', '>>='): raise Unsupported('statement op %r' % op)
        e = self.expr(); self.eat(';')
        place = lv[0]
        v = conv(e, lv[1]) if op in ('=', '+=', '-=', '|=', '&=', '^=') else to_u(*e)
        if op == '=': return '%s = %s;' % (place, v)
        if op in ('+=', '-='):
            return '%s = %s.%s(%s);' % (place, place, 'wrapping_add' if op == '+=' else 'wrapping_sub', v)
        return '%s %s %s;' % (place, op, v)


def translate(code_lines):
    """C statements (a list of lines, as in Insn.code) -> list of Rust statements, or raises Unsupported."""
    p = P(tokenize(' '.join(code_lines)))
    out = []
    while p.peek() is not None:
        out.append(p.stmt())
    return out


def arm(pc, ins, jump_c):
    """One match arm for an instruction; mirrors emit_single in m68k2c.py. Returns (text, supported)."""
    nxt = pc + ins.length
    body = list(ins.code)
    if ins.kind == 'dbcc':
        body = [b.replace('BRANCH', 'cpu.PC = 0x%Xu; return 1;' % ins.static_target) for b in body]
    try:
        lines = ['cpu.cyc = cpu.cyc.wrapping_add(%d);' % ins.cyc] + translate(body)
        done = False
        def jump(t): return 'cpu.pc = 0x%X; return Ok(());' % t
        if ins.kind in ('bra', 'bsr'):
            lines.append(jump(ins.static_target)); done = True
        elif ins.kind == 'bcc':
            lines.append('if cpu.cond(%d) { %s }' % (ins.cc, jump(ins.static_target)))
            lines.append('cpu.cyc = cpu.cyc.wrapping_add(%du64);' % ((-2 if (ins.op & 0xff) else 2) & 0xffffffffffffffff))
        elif ins.text in ('jsr', 'jmp') and hasattr(ins, 'static_target'):
            lines.append(jump(ins.static_target)); done = True
        elif ins.dyn:
            lines.append('cpu.pc = pc_; return Ok(());'); done = True
        if not done:
            lines.append(jump(nxt))
    except Unsupported as e:
        return '        0x%X => return Err(Stop::Untranslated(0x%X)), // %s: %s' % (pc, pc, ins.text or ins.kind, e), str(e)
    txt = ['        0x%X => { // %s' % (pc, ins.text or ins.kind)]
    txt += ['            ' + l for l in lines]
    txt.append('        }')
    return '\n'.join(txt), None


def emit_rust(insns, path, hle=None):
    """Write the Rust form of the translated instructions. Returns {'translated': n, 'untranslated': {pc: reason}}."""
    L = ['// generated by m68k2c.py --emit rust (kit/amigo/port/m68k2rs.py), do not edit',
         '#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, clippy::all)]',
         'use amigo_g5::cpu::{sx16, sx8, Cpu, Stop};',
         'use amigo_g5::mem::GuestMemory;', '',
         'pub fn step(cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {',
         '    let (mut ea, mut ea2, mut s, mut t, mut r, mut pc_) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);',
         '    match cpu.pc {']
    bad = {}
    n = 0
    for pc in sorted(insns):
        if hle and pc in hle: continue
        text, err = arm(pc, insns[pc], None)
        L.append(text)
        if err: bad[pc] = err
        else: n += 1
    L += ['        pc => return Err(Stop::Untranslated(pc)),', '    }', '    Ok(())', '}', '']
    open(path, 'w').write('\n'.join(L))
    return {'translated': n, 'untranslated': bad}
