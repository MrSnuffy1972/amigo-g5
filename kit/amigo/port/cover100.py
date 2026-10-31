#!/usr/bin/env python3
"""The 100% gate (31 Oct 2026, Karl: "100% of the code needs to be translated, including multi disk games").

usage: cover100.py GENDIR [--extra FILE ...] [--json OUT]

Reads the translator's outputs in GENDIR (translated_pcs.txt, direct_targets.txt, executed_pcs.txt, translate_stats.json)
and fails (exit 1), naming the addresses, when any direct branch target or jump-table entry of the real code, any PC a
recording executed, or any address in an --extra file (recorded targets, one hex address per line) is not a translated
instruction. An arm the Rust emitter left as Untranslated does not count as translated. HLE PCs count as covered.
Kickstart ROM addresses (0xF80000 and up) are not game code and are left out. Direct targets come from the confirmed code
(not speculative roots); odd ones (address errors) and ones the decoder rejects as illegal 68000 instructions are listed
in --json, not failed.
"""
import json, os, sys

def hexes(path):
    out = set()
    if not os.path.exists(path): return out
    for l in open(path):
        t = l.split()
        if not t: continue
        try: out.add(int(t[0], 16))
        except ValueError: pass
    return out

def main():
    if len(sys.argv) < 2: sys.exit(__doc__)
    g = sys.argv[1]; extra = []; jout = None
    a = sys.argv[2:]
    for i, x in enumerate(a):
        if x == '--json': jout = a[i + 1]
        if x == '--extra':
            for f in a[i + 1:]:
                if f.startswith('--'): break
                extra.append(f)
    tsj = json.load(open(os.path.join(g, 'translate_stats.json'))); st = tsj['stats']
    # an address the decoder rejects as an illegal 68000 instruction (bad EA, line A/F, ...) is not code: listed, not failed.
    # Out of code range still fails (pilot 31 Oct, 800: bcc at 07E9DE into 'bad ea 7 6' on both paths = data, not code).
    illegal = {int(k, 16): v for k, v in tsj.get('bad', {}).items() if v != 'out of code range'}
    unt = {int(k, 16) for k in st.get('rust_untranslated', {})}
    hle = {int(h.split()[0], 16) for h in st.get('hle', [])}
    ok = (hexes(os.path.join(g, 'translated_pcs.txt')) - unt) | hle
    need = {'direct target': hexes(os.path.join(g, 'direct_targets.txt')),
            'executed': hexes(os.path.join(g, 'executed_pcs.txt'))}
    rec = set()
    for f in extra: rec |= hexes(f)
    need['recorded'] = rec
    miss = {}
    for kind, s in need.items():
        for p in s:
            if p >= 0xF80000 or p in ok or p in illegal: continue
            miss.setdefault(p, []).append(kind)
    odd = hexes(os.path.join(g, 'direct_targets_odd.txt'))   # 68000 address error targets: listed, not code
    rep = {'illegal_not_code': ['%06X %s' % (p, illegal[p]) for p in sorted(illegal)], 'odd_direct_targets_not_code': ['%06X' % p for p in sorted(odd)], 'checked': {k: len(v) for k, v in need.items()}, 'translated': len(ok), 'missing': len(miss),
           'missing_list': ['%06X %s%s' % (p, '+'.join(miss[p]), ' (Untranslated arm)' if p in unt else '') for p in sorted(miss)]}
    if jout: json.dump(rep, open(jout, 'w'), indent=1)
    if miss:
        print('cover100: FAIL, %d addresses outside translated code: %s' % (len(miss), ', '.join(rep['missing_list'][:40])))
        sys.exit(1)
    print('cover100: PASS (%s, %d translated)' % (', '.join('%d %s' % (v, k) for k, v in rep['checked'].items()), len(ok)))

if __name__ == '__main__':
    main()
