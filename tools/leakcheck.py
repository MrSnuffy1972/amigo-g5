#!/usr/bin/env python3
"""leakcheck.py: fail if the G5 repo holds anything that must never leave Ryzen.

usage: leakcheck.py [--roms DIR] [--rtb DIR] [--hashes FILE] [--allow FILE] [--hosts FILE] PATH

Every file under PATH (except .git, __pycache__ and target folders) is checked:
  1. known hash: sha256 equals a Kickstart ROM or WHDLoad RTB. Hashes come from --roms / --rtb (read on Ryzen)
     and from --hashes (tools/leak-hashes.txt: sha256 only, never the bytes; export.sh writes it).
  2. Kickstart bytes (only with --roms, so only on Ryzen): the kcheck.py method from scratch/g1/fix-kickemu.
     32-byte windows every 256 bytes of each ROM (windows with fewer than 12 different byte values skipped,
     because fill and tables appear in any binary), plus the ROM's first 64 KB.
  3. ROM signatures that need no ROM (cloud and CI): a Kickstart reset header at offset 0
     (11 11 4E F9 / 11 14 4E F9 / 11 16 4E F9), Cloanto's "AMIROMTYPE1" header, ROM-sized files.
  4. game files and disk images: forbidden extensions (.adf .adz .dms .ipf .hdf .lha .lzh .lzx .rom .rtb
     .slave .key), names kick*.rom, ADF sizes, a "DOS" boot block, IPF "CAPS", DMS "DMS!", RDSK (HDF),
     LHA "-lh?-", and AmigaDOS hunk executables (000003F3 at offset 0: a game or a WHDLoad slave).
  5. pictures, sound and dumps: any file that is not UTF-8 text fails unless tools/leak-allow.txt lists its
     path AND sha256 (so a changed binary must be reviewed again).
  6. secrets: API keys, tokens, passwords, private keys (patterns below).
  7. LAN: private and CGNAT (100.64/10) IPv4 addresses fail; host names from --hosts FILE (kept outside the repo,
     so the list is not itself a leak) are reported as warnings.
Exit 0 = CLEAN, 1 = HIT, 2 = usage error. Prints one line per finding and a final LEAKCHECK line.
"""
import hashlib, os, re, sys

FORBIDDEN_EXT = {'.adf', '.adz', '.dms', '.ipf', '.hdf', '.lha', '.lzh', '.lzx', '.rom', '.rtb', '.slave', '.key',
                 '.png', '.ppm', '.pgm', '.bmp', '.gif', '.jpg', '.jpeg', '.iff', '.ilbm', '.rgba', '.raw', '.wav',
                 '.mp3', '.ogg', '.mp4', '.avi', '.mkv', '.tgz', '.zip', '.7z', '.db', '.sqlite', '.pem', '.p12'}
ADF_SIZES = {901120, 1802240}
ROM_SIZES = {262144, 524288, 1048576}
KICK_HEADERS = (b'\x11\x11\x4e\xf9', b'\x11\x14\x4e\xf9', b'\x11\x16\x4e\xf9')
SECRET_PATTERNS = [
    ('anthropic/openai/openrouter key', r'\bsk-(?:ant-|or-v1-|proj-)?[A-Za-z0-9_\-]{20,}'),
    ('github token', r'\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{30,}|\bgithub_pat_[A-Za-z0-9_]{30,}'),
    ('aws key', r'\bAKIA[0-9A-Z]{16}\b'),
    ('google api key', r'\bAIza[0-9A-Za-z_\-]{35}\b'),
    ('slack token', r'\bxox[abprs]-[A-Za-z0-9\-]{10,}'),
    ('hugging face token', r'\bhf_[A-Za-z0-9]{30,}'),
    ('private key', r'-----BEGIN (?:[A-Z]+ )?PRIVATE KEY-----'),
    ('bearer token', r'\bBearer\s+[A-Za-z0-9_\-\.=]{24,}'),
    ('password assignment', r'(?i)\b(?:password|passwd|pwd)\s*[:=]\s*["\'][^"\'\s]{4,}["\']'),
    ('token/key assignment', r'(?i)\b(?:api[_\-]?key|secret|token|auth)\s*[:=]\s*["\'][A-Za-z0-9_\-\.]{16,}["\']'),
]
PRIVATE_IP = (r'\b(?:192\.168\.\d{1,3}\.\d{1,3}|10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(?:1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3}'
              r'|100\.(?:6[4-9]|[7-9]\d|1[01]\d|12[0-7])\.\d{1,3}\.\d{1,3})\b')
# Build products that .gitignore keeps out of commits (Python bytecode, cargo output) are not scanned.
SKIP_DIRS = {'.git', '__pycache__', 'target'}
# Host names are not listed here (the list itself would leak them): --hosts FILE, one name per line, kept outside the repo.


def sha(b):
    return hashlib.sha256(b).hexdigest()


def files_under(top):
    if os.path.isfile(top):
        yield top
        return
    for d, ds, fs in os.walk(top):
        ds[:] = [x for x in ds if x not in SKIP_DIRS]
        for f in fs:
            yield os.path.join(d, f)


def load_roms(dirs):
    roms = []
    for d in dirs:
        for p in files_under(d):
            try:
                b = open(p, 'rb').read()
            except OSError:
                continue
            if len(b) == 2 * 262144 and b[:262144] == b[262144:]:
                b = b[:262144]
            if len(b) >= 65536:
                roms.append(b)
    return roms


def load_hash_file(p):
    out = set()
    if p and os.path.exists(p):
        for line in open(p):
            t = line.split()
            if t and re.fullmatch(r'[0-9a-f]{64}', t[0]):
                out.add(t[0])
    return out


def load_allow(p):
    out = {}
    if p and os.path.exists(p):
        for line in open(p):
            t = line.split(None, 2)
            if len(t) >= 2 and re.fullmatch(r'[0-9a-f]{64}', t[0]):
                out[t[1]] = t[0]
    return out


def is_text(b):
    if b'\x00' in b:
        return False
    try:
        b.decode('utf-8')
        return True
    except UnicodeDecodeError:
        return False


def main(argv):
    roms_d, rtb_d, hashes_p, allow_p, hosts_p, top = [], [], None, None, None, None
    a = list(argv)
    while a:
        x = a.pop(0)
        if x == '--hosts': hosts_p = a.pop(0)
        elif x == '--roms': roms_d.append(a.pop(0))
        elif x == '--rtb': rtb_d.append(a.pop(0))
        elif x == '--hashes': hashes_p = a.pop(0)
        elif x == '--allow': allow_p = a.pop(0)
        elif x.startswith('--'):
            print('unknown option ' + x); return 2
        else: top = x
    if not top:
        print(__doc__); return 2
    known = load_hash_file(hashes_p)
    roms = load_roms(roms_d)
    for b in roms:
        known.add(sha(b))
    for d in rtb_d:
        for p in files_under(d):
            try:
                known.add(sha(open(p, 'rb').read()))
            except OSError:
                pass
    wins = []
    for b in roms:
        wins += [b[o:o + 32] for o in range(0, len(b) - 32, 256) if len(set(b[o:o + 32])) >= 12]
    firsts = [b[:65536] for b in roms]
    allow = load_allow(allow_p)
    sec = [(n, re.compile(p)) for n, p in SECRET_PATTERNS]
    hosts = []
    if hosts_p and os.path.exists(hosts_p):
        hosts = [l.strip() for l in open(hosts_p) if l.strip() and not l.startswith('#')]
    ipre = re.compile(PRIVATE_IP)
    hostre = re.compile(r'(?i)\b(?:%s)\b' % '|'.join(re.escape(h) for h in hosts)) if hosts else None
    hits, warns, nfiles, nbytes = [], [], 0, 0
    for p in sorted(files_under(top)):
        rel = os.path.relpath(p, top)
        try:
            b = open(p, 'rb').read()
        except OSError as e:
            hits.append('%s: unreadable (%s)' % (rel, e)); continue
        nfiles += 1; nbytes += len(b)
        h = sha(b); base = os.path.basename(p).lower(); ext = os.path.splitext(base)[1]
        why = []
        if h in known: why.append('sha256 matches a Kickstart ROM or WHDLoad RTB')
        if ext in FORBIDDEN_EXT: why.append('forbidden file type ' + ext)
        if re.fullmatch(r'kick.*\.rom', base): why.append('named kick*.rom')
        if b[:4] in KICK_HEADERS: why.append('Kickstart reset header at offset 0')
        if b'AMIROMTYPE1' in b[:64]: why.append('Cloanto AMIROMTYPE1 ROM header')
        if len(b) in ADF_SIZES: why.append('ADF-sized (%d bytes)' % len(b))
        if b[:3] == b'DOS' and len(b) >= 1024 and not is_text(b): why.append('AmigaDOS boot block at offset 0')
        if b[:4] == b'CAPS': why.append('IPF (CAPS) image')
        if b[:4] == b'DMS!': why.append('DMS image')
        if b[:4] == b'RDSK': why.append('RDSK hard-disk image (HDF)')
        if len(b) > 7 and b[2:4] == b'-l' and b[6:7] == b'-' and not is_text(b): why.append('LHA archive')
        if b[:4] == b'\x00\x00\x03\xf3': why.append('AmigaDOS hunk executable (game or WHDLoad slave)')
        if roms:
            if any(f in b for f in firsts): why.append("Kickstart's first 64 KB present")
            n = sum(1 for w in wins if w in b)
            if n: why.append('%d Kickstart 32-byte windows present' % n)
        text = is_text(b)
        if not text:
            if allow.get(rel) == h:
                pass
            elif len(b) in ROM_SIZES:
                why.append('binary of exactly a ROM size, not on the allow list')
            else:
                why.append('binary file (%d bytes) not on tools/leak-allow.txt with this sha256' % len(b))
        if len(b) > 4 * 1024 * 1024 and not allow.get(rel) == h:
            why.append('larger than 4 MB')
        s = b.decode('utf-8', 'replace') if text else b.decode('latin-1')
        for name, rx in sec:
            m = rx.search(s)
            if m: why.append('secret pattern: %s (line %d)' % (name, s.count('\n', 0, m.start()) + 1))
        m = ipre.search(s)
        if m: why.append('private IP address %s (line %d)' % (m.group(0), s.count('\n', 0, m.start()) + 1))
        hn = hostre.findall(s) if hostre else []
        if hn: warns.append('%s: %d fleet host-name mention(s) (%s)' % (rel, len(hn), ', '.join(sorted(set(x.lower() for x in hn)))))
        for w in why:
            hits.append('%s: %s' % (rel, w))
    for w in warns:
        print('WARN ' + w)
    for x in hits:
        print('HIT  ' + x)
    print('LEAKCHECK %s: %d files, %d bytes searched; %d hit(s), %d warning(s); %d ROM(s) loaded for byte search, %d known hashes'
          % ('CLEAN' if not hits else 'HIT', nfiles, nbytes, len(hits), len(warns), len(roms), len(known)))
    return 0 if not hits else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
