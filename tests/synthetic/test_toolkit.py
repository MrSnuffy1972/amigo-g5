#!/usr/bin/env python3
"""Unit tests that need no game: hand-assembled 68000 snippets through the toolkit, plus the leak check.

Every byte here was written for these tests (no game, ROM or disk data). Run from the repo root:
    python3 -m unittest discover -s tests -v
Needs python3 and gcc (gcc only for the compile test, which is skipped without it).
"""
import json, os, shutil, struct, subprocess, sys, tempfile, unittest

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
PORT = os.path.join(ROOT, 'kit', 'amigo', 'port')
AMIGO = os.path.join(ROOT, 'kit', 'amigo')
LEAK = os.path.join(ROOT, 'tools', 'leakcheck.py')

BASE = 0x20000
# moveq #5,d0 / moveq #3,d1 / loop: add.l d1,d0 / subq.l #1,d1 / bne.s loop / bra.s self
# After it runs: d0 = 5 + 3 + 2 + 1 = 11, d1 = 0. Encodings checked by hand against the 68000 manual.
SNIPPET = [0x7005, 0x7203, 0xD081, 0x5381, 0x66FA, 0x60FE]
SNIPPET_BYTES = b''.join(struct.pack('>H', w) for w in SNIPPET)


def run(cmd, cwd=None, env=None):
    return subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True, timeout=120)


def put(path, data):
    with open(path, 'wb' if isinstance(data, bytes) else 'w') as f:
        f.write(data)


def get(path, mode='r'):
    with open(path, mode) as f:
        return f.read()


class TranslatorSynthetic(unittest.TestCase):
    """m68k2c.py on a 12-byte synthetic program: every instruction found, none undecodable, C emitted."""

    def setUp(self):
        self.w = tempfile.mkdtemp(prefix='g5t-')
        os.makedirs(os.path.join(self.w, 'out'))
        put(os.path.join(self.w, 'out', 'image.bin'), SNIPPET_BYTES)
        cfg = {'name': 'synthetic', 'segments': [{'file': 'out/image.bin', 'addr': hex(BASE)}],
               'code': ['0x%X-0x%X' % (BASE, BASE + len(SNIPPET_BYTES))], 'entry': hex(BASE),
               'ghidra': False, 'seeds': [], 'hle': {}}
        put(os.path.join(self.w, 'game.json'), json.dumps(cfg))
        self.out = os.path.join(self.w, 'gen')
        os.makedirs(self.out)
        env = dict(os.environ, AMIGO_W=self.w)
        self.r = run([sys.executable, os.path.join(PORT, 'm68k2c.py'), self.out, '--config',
                      os.path.join(self.w, 'game.json'), '--no-speculative'], cwd=self.w, env=env)

    def tearDown(self):
        shutil.rmtree(self.w, ignore_errors=True)

    def test_translates_every_instruction(self):
        self.assertEqual(self.r.returncode, 0, self.r.stderr[-2000:])
        st = json.loads(get(os.path.join(self.out, 'translate_stats.json')))['stats']
        self.assertEqual(st['instructions_translated'], len(SNIPPET))
        self.assertEqual(st['undecodable_addresses_reached'], 0)
        pcs = [int(x, 16) for x in get(os.path.join(self.out, 'translated_pcs.txt')).split()]
        self.assertEqual(pcs, [BASE + 2 * i for i in range(len(SNIPPET))])

    def test_emits_c(self):
        self.assertEqual(self.r.returncode, 0, self.r.stderr[-2000:])
        cs = [f for f in os.listdir(self.out) if f.endswith('.c')]
        self.assertTrue(cs, 'no C file emitted')

    @unittest.skipUnless(shutil.which('gcc'), 'gcc not installed')
    def test_c_compiles(self):
        self.assertEqual(self.r.returncode, 0, self.r.stderr[-2000:])
        cs = [os.path.join(self.out, f) for f in os.listdir(self.out) if f.endswith('.c')]
        for c in cs:
            r = run(['gcc', '-fsyntax-only', '-w', '-I', os.path.join(PORT, 'runtime'), '-I', PORT, '-I', self.out, c])
            self.assertEqual(r.returncode, 0, '%s: %s' % (os.path.basename(c), r.stderr[-2000:]))


class HunkLoadSynthetic(unittest.TestCase):
    """hunkload.py on a minimal synthetic AmigaDOS hunk file (built here, never stored in the repo)."""

    def test_one_code_hunk(self):
        w = tempfile.mkdtemp(prefix='g5h-')
        try:
            code = SNIPPET_BYTES
            words = (len(code) + 3) // 4
            body = code + b'\x00' * (words * 4 - len(code))
            hunk = struct.pack('>IIIII', 0x3F3, 0, 1, 0, 0) + struct.pack('>I', words)
            hunk += struct.pack('>II', 0x3E9, words) + body + struct.pack('>I', 0x3F2)
            src = os.path.join(w, 'prog')
            put(src, hunk)
            img = os.path.join(w, 'image.bin')
            r = run([sys.executable, os.path.join(AMIGO, 'hunkload.py'), src, '--out', img, '--base', hex(BASE)])
            self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
            self.assertEqual(get(img, 'rb')[:len(code)], code)
            self.assertIn('ENTRY', r.stdout)
        finally:
            shutil.rmtree(w, ignore_errors=True)


class LeakCheck(unittest.TestCase):
    """tools/leakcheck.py passes a clean folder and catches each kind of leak."""

    def check(self, files):
        w = tempfile.mkdtemp(prefix='g5l-')
        try:
            for name, data in files.items():
                p = os.path.join(w, name)
                os.makedirs(os.path.dirname(p), exist_ok=True)
                put(p, data)
            return run([sys.executable, LEAK, w])
        finally:
            shutil.rmtree(w, ignore_errors=True)

    def test_clean(self):
        r = self.check({'src/a.c': b'int main(void){return 0;}\n'})
        self.assertEqual(r.returncode, 0, r.stdout)

    def test_catches(self):
        fake_key = 'sk-' + 'ant-' + 'A' * 30            # built at run time so this file holds no key pattern
        cases = {
            'disk.adf': b'\x00' * 16,
            'blank': b'\x00' * 901120,
            'kickstart.bin': bytes.fromhex('11144ef9') + b'\x00' * 60,
            'game': bytes.fromhex('000003f3') + b'\x00' * 60,
            'shot.png': b'\x89PNG\r\n',
            'cfg.py': ('KEY = "%s"\n' % fake_key).encode(),
            'hosts.txt': ('host ' + '192.' + '168.0.9\n').encode(),
        }
        for name, data in cases.items():
            r = self.check({name: data})
            self.assertEqual(r.returncode, 1, '%s not caught: %s' % (name, r.stdout))


if __name__ == '__main__':
    unittest.main()
