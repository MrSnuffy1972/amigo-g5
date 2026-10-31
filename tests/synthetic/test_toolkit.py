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


class RustEmitterCoverage(unittest.TestCase):
    """m68k2rs.py (the --emit rust transliterator): every opcode the decoder accepts becomes Rust, and text it does
    not know becomes a Stop::Untranslated arm rather than a guess. Behaviour is checked against the C translation by
    rust/amigo_g5/tests/emit_diff.rs; this only checks coverage."""

    @classmethod
    def setUpClass(cls):
        sys.path.insert(0, PORT)
        import m68k2c, m68k2rs
        cls.m68k2c, cls.m68k2rs = m68k2c, m68k2rs

    class Words:
        """The translator's memory interface over one opcode followed by extension words of 4 (a valid even
        displacement, immediate and register mask for every addressing mode)."""
        snap = None

        def __init__(self, op):
            self.op = op

        def w(self, a):
            return self.op if a == BASE else 0x0004

        def l(self, a):
            return (self.w(a) << 16) | self.w(a + 2)

    def test_every_decodable_opcode_translates(self):
        decoded, bad = 0, {}
        for op in range(0x10000):
            try:
                ins = self.m68k2c.Decoder(self.Words(op)).decode(BASE)
            except self.m68k2c.Undecodable:
                continue
            decoded += 1
            _, err = self.m68k2rs.arm(BASE, ins, None)
            if err:
                bad.setdefault(err, '%04X' % op)
        self.assertGreater(decoded, 40000)
        self.assertEqual(bad, {})

    def test_unknown_text_is_reported_not_guessed(self):
        ins = self.m68k2c.Insn(BASE)
        ins.code = ['goto elsewhere;']
        text, err = self.m68k2rs.arm(BASE, ins, None)
        self.assertIn('goto elsewhere', err)
        self.assertIn('Stop::Untranslated(0x%X)' % BASE, text)
        for c in ('while (t) t -= 1;', 'x = 1;', 'D[0] = 1.5;', 'switch (t) {}'):
            with self.assertRaises(self.m68k2rs.Unsupported, msg=c):
                self.m68k2rs.translate([c])

    def test_c_literal_types_keep_signed_compares_signed(self):
        rs = ' '.join(self.m68k2rs.translate(['{ int32_t q = (int32_t)D[0]; if (q < -32768) VF = 1; }']))
        self.assertIn('32768i32.wrapping_neg()', rs)
        self.assertIn('let mut q: i32', rs)


class TranslatorCodeBytes(unittest.TestCase):
    """Where the translator reads code bytes (m68k2c.Mem.rb): inside a code range, bytes a segment loaded win over the
    end-of-run RAM, and bytes nothing loaded (the image's zero fill) give way to it (30 Oct 2026, Qattro's music player)."""

    @classmethod
    def setUpClass(cls):
        sys.path.insert(0, PORT)
        import m68k2c
        cls.m68k2c = m68k2c

    def mem(self):
        m = object.__new__(self.m68k2c.Mem)
        m.chipsz, m.slowsz = 0x1000, 0x100
        m.chip, m.slow, m.other = bytearray(m.chipsz), bytearray(m.slowsz), {}
        m.lchip, m.lslow = bytearray(m.chipsz), bytearray(m.slowsz)
        m.ranges, m.kick, m.fx, m.fxo, m.observed = [(0x100, 0x800)], None, {}, {}, {}
        ram = bytearray(m.chipsz)
        ram[0x200:0x204] = bytes.fromhex('6000fc6c')   # code the program unpacked after the snapshot
        ram[0x300:0x302] = bytes.fromhex('4e75')       # end-of-run RAM over code a segment loaded
        m.snap, m.fill = None, (bytes(ram), bytes(m.slowsz))   # no code observed outside the ranges: no snap, only the fill
        m.put(0x300, bytes.fromhex('4e71'))
        return m

    def test_unloaded_code_comes_from_the_end_of_run_ram(self):
        m = self.mem()
        self.assertEqual(bytes(m.rb(0x200 + i) for i in range(4)), bytes.fromhex('6000fc6c'))

    def test_loaded_code_keeps_its_image_bytes(self):
        m = self.mem()
        self.assertEqual(bytes(m.rb(0x300 + i) for i in range(2)), bytes.fromhex('4e71'))

    def test_first_execution_bytes_still_win(self):
        m = self.mem()
        m.fx[0x200] = 0x4e
        self.assertEqual(m.rb(0x200), 0x4e)


class PlayCoverage(unittest.TestCase):
    """portcfg.observed_bytes with out/port/play (31 Oct 2026, "Record play, then translate"): first bytes recorded while
    the original was played fill addresses the no-input reference never ran, and never override it."""

    def test_play_bytes_fill_gaps_only(self):
        import tempfile
        sys.path.insert(0, PORT)
        import portcfg
        d = tempfile.mkdtemp()
        os.makedirs(os.path.join(d, 'out/port/ref')); os.makedirs(os.path.join(d, 'out/port/play'))
        open(os.path.join(d, 'out/port/ref/first_bytes.txt'), 'w').write('001000 4E 71 4E 75\n')
        open(os.path.join(d, 'out/port/play/first_bytes.txt'), 'w').write('001002 60 00\n002000 70 01 4E 75\n')
        old, portcfg.W = portcfg.W, d
        try:
            fx = portcfg.observed_bytes({})
        finally:
            portcfg.W = old
        self.assertEqual([fx[0x1002], fx[0x1003]], [0x4E, 0x75])          # the reference wins
        self.assertEqual([fx[0x2000 + i] for i in range(4)], [0x70, 0x01, 0x4E, 0x75])   # play fills the gap


class GamePack(unittest.TestCase):
    """tools/g5_pack.py on synthetic files: pack, list and unpack round-trip; damage is caught (M7, 29 Oct 2026)."""

    def setUp(self):
        self.w = tempfile.mkdtemp(prefix='g5p-')
        self.addCleanup(shutil.rmtree, self.w)
        g = os.path.join(self.w, 'game')
        os.makedirs(os.path.join(g, 's'))
        os.makedirs(os.path.join(g, 'data', 'lvl'))
        self.files = {'Game': bytes([0, 0, 3, 0xF3]) + bytes(range(256)), 's/startup-sequence': b'Game -x\n',
                      'data/lvl/one': b'\x00' * 3, 'data/b': b'', 'Zed': b'z'}
        for n, d in self.files.items():
            put(os.path.join(g, *n.split('/')), d)
        put(os.path.join(self.w, 'game.json'), '{"name": "Synthetic", "controls": {"pad": {"x": "key:Space"}}}')
        self.tool = os.path.join(ROOT, 'tools', 'g5_pack.py')
        self.pack = os.path.join(self.w, 'p.bin')
        r = run([sys.executable, self.tool, 'pack', '--game', g, '--config', os.path.join(self.w, 'game.json'),
                 '--out', self.pack])
        self.assertEqual(r.returncode, 0, r.stderr)

    def test_round_trip(self):
        out = os.path.join(self.w, 'out')
        r = run([sys.executable, self.tool, 'unpack', self.pack, '--out', out])
        self.assertEqual(r.returncode, 0, r.stderr)
        for n, d in self.files.items():
            self.assertEqual(get(os.path.join(out, 'files', *n.split('/')), 'rb'), d, n)
        self.assertEqual(get(os.path.join(out, 'game.json'), 'rb'), get(os.path.join(self.w, 'game.json'), 'rb'))
        # directory-walk order: names sorted per directory, a subdirectory's files where its name sorts
        lst = run([sys.executable, self.tool, 'list', self.pack]).stdout.split()[1::2]
        self.assertEqual(lst, ['game.json', 'files/Game', 'files/Zed', 'files/data/b', 'files/data/lvl/one',
                               'files/s/startup-sequence'])

    def test_damage_is_caught(self):
        b = bytearray(get(self.pack, 'rb'))
        b[30] ^= 0x10
        put(self.pack, bytes(b))
        r = run([sys.executable, self.tool, 'list', self.pack])
        self.assertEqual(r.returncode, 2)
        self.assertIn('checksum', r.stderr)


class Package(unittest.TestCase):
    """tools/g5_package.sh --dry-run on a synthetic pack (M9, 29 Oct 2026): the arguments and the pack are checked, the
    build commands are the packaging contract's (targets, the pack built into the exe but not into the APK's library,
    package org.amigo.g5.ID) and the APK manifest declares what the Shield and phones need. The real NDK and mingw
    builds run on Ryzen."""

    def setUp(self):
        self.w = tempfile.mkdtemp(prefix='g5k-')
        self.addCleanup(shutil.rmtree, self.w)
        g = os.path.join(self.w, 'game')
        os.makedirs(os.path.join(g, 's'))
        put(os.path.join(g, 'Game'), bytes([0, 0, 3, 0xF3]) + bytes(8))
        put(os.path.join(g, 's', 'startup-sequence'), 'Game\n')
        self.pack = os.path.join(self.w, 'p.bin')
        r = run([sys.executable, os.path.join(ROOT, 'tools', 'g5_pack.py'), 'pack', '--game', g, '--out', self.pack])
        self.assertEqual(r.returncode, 0, r.stderr)
        self.tool = os.path.join(ROOT, 'tools', 'g5_package.sh')
        self.out = os.path.join(self.w, 'out')

    def package(self, *extra, pack=None, ident='synth'):
        return run(['bash', self.tool, '--pack', pack or self.pack, '--name', 'Synth & "Co"', '--id', ident,
                    '--out', self.out, '--dry-run'] + list(extra))

    def test_windows_and_android(self):
        r = self.package('--windows', '--android')
        self.assertEqual(r.returncode, 0, r.stderr)
        cmds = [c for c in r.stdout.splitlines() if c.startswith('+ ')]
        builds = [c for c in cmds if ' cargo build ' in c]
        self.assertEqual(len(builds), 3, builds)
        win, arm64, armv7 = builds
        self.assertIn('AMIGO_G5_PACK=' + os.path.realpath(self.pack), win)
        self.assertIn('--offline --locked --target x86_64-pc-windows-gnu --bin amigo-g5-play', win)
        for c, t, abi in ((arm64, 'aarch64-linux-android', 'arm64-v8a'),
                          (armv7, 'armv7-linux-androideabi', 'armeabi-v7a')):
            self.assertIn('env -u AMIGO_G5_PACK ', c)
            self.assertIn('--target %s -p amigo_g5_android' % t, c)
            self.assertIn('native=/opt/sdl2-android/' + abi, c)
            self.assertTrue(any(abi + '/libSDL2.so' in x and 'libc++_shared.so' in x for x in cmds), abi)
        # every binary is symbol-checked unstripped, before it is stripped
        checks = [c for c in cmds if c.startswith('+ $G5_SYMCHECK ')]
        self.assertEqual([c.rsplit('/', 1)[1] for c in checks],
                         ['synth.exe', 'arm64-v8a-libmain.so', 'armeabi-v7a-libmain.so'])
        self.assertTrue(any(c.endswith('/work/assets/gamepack.bin') for c in cmds))
        self.assertTrue(any('apksigner sign' in c and c.split('--out ')[1].startswith(os.path.join(
            os.path.realpath(self.out), 'synth.apk')) for c in cmds))
        m = get(os.path.join(self.out, 'work', 'AndroidManifest.xml'))
        self.assertIn('package="org.amigo.g5.synth"', m)
        self.assertIn('android:label="Synth &amp; &quot;Co&quot;"', m)
        for want in ('android.intent.category.LAUNCHER', 'android.intent.category.LEANBACK_LAUNCHER',
                     'android:banner=', 'android:screenOrientation="sensorLandscape"',
                     'android:name="android.hardware.touchscreen" android:required="false"',
                     'android:name="android.hardware.gamepad"'):
            self.assertIn(want, m)
        j = get(os.path.join(self.out, 'work', 'java', 'org', 'amigo', 'GameActivity.java'))
        self.assertIn('"c++_shared", "SDL2", "main"', j)

    def test_x86_64_android_is_optional(self):
        env = dict(os.environ, G5_ANDROID_X86_64='1')
        r = run(['bash', self.tool, '--pack', self.pack, '--name', 'S', '--id', 'synth', '--out', self.out,
                 '--android', '--dry-run'], env=env)
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn('--target x86_64-linux-android -p amigo_g5_android', r.stdout)
        self.assertIn('ABIs arm64-v8a armeabi-v7a x86_64', r.stdout)

    def test_skip_symcheck_says_so(self):
        r = self.package('--windows', '--skip-symcheck')
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn('SYMCHECK SKIPPED', r.stdout)
        self.assertNotIn('$G5_SYMCHECK', r.stdout)

    def test_bad_arguments_and_packs_are_refused(self):
        self.assertEqual(self.package().returncode, 2)  # no target
        self.assertEqual(self.package('--windows', ident='Bad-Id').returncode, 2)
        self.assertEqual(self.package('--windows', ident='9lives').returncode, 2)
        self.assertEqual(run(['bash', self.tool, '--pack']).returncode, 2)
        self.assertEqual(self.package('--windows', '--icon', os.path.join(self.w, 'none.png')).returncode, 2)
        b = bytearray(get(self.pack, 'rb'))
        b[12] ^= 1
        bad = os.path.join(self.w, 'bad.bin')
        put(bad, bytes(b))
        r = self.package('--windows', pack=bad)
        self.assertEqual(r.returncode, 2)
        self.assertIn('not a good game pack', r.stderr)


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
