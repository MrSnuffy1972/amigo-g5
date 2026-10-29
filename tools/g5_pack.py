#!/usr/bin/env python3
"""g5_pack.py: the G5 game pack (M7, 29 Oct 2026).

A game's files plus its game.json in one blob, so the Windows exe can embed it and the Android APK can carry it as the
asset gamepack.bin. The format (docs/G5-PLAYABLE.md "Game pack format", read by rust/amigo_g5/src/pack.rs):

  b"AG5PACK1", then u32 LE entry count
  per entry: u16 LE name length, the name (UTF-8, '/' separated), u32 LE data length, the data
  trailer:   u64 LE FNV-1a 64 of every byte before it

Entries: game.json (optional) first, then files/<path> for each game file in the order a directory walk sorted by name
gives (each directory's entries by name, a subdirectory where its name sorts), the order amigo-g5-boot serves them in.

  g5_pack.py pack --game DIR [--config game.json] --out PACK.bin
  g5_pack.py list PACK.bin
  g5_pack.py unpack PACK.bin --out DIR      (writes DIR/game.json and DIR/files/...)

Runs on Ryzen on the real games; in this repo it is only ever tested on synthetic files.
"""
import argparse
import os
import struct
import sys

MAGIC = b"AG5PACK1"


def fnv1a64(data):
    h = 0xCBF29CE484222325
    for b in data:
        h = ((h ^ b) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def walk(root, prefix=""):
    """(relative path, bytes) for every file under root, sorted by name per directory (os.fsencode order, as Rust's
    OsString sort)."""
    out = []
    for name in sorted(os.listdir(root), key=os.fsencode):
        p = os.path.join(root, name)
        rel = prefix + name
        if os.path.isdir(p):
            out += walk(p, rel + "/")
        else:
            with open(p, "rb") as f:
                out.append((rel, f.read()))
    return out


def build(files, config=None):
    ents = ([("game.json", config)] if config is not None else []) + [("files/" + n, d) for n, d in files]
    b = bytearray(MAGIC + struct.pack("<I", len(ents)))
    for n, d in ents:
        nb = n.encode("utf-8")
        if len(nb) > 0xFFFF or len(d) > 0xFFFFFFFF:
            raise ValueError("entry too large: %s" % n)
        b += struct.pack("<H", len(nb)) + nb + struct.pack("<I", len(d)) + d
    return bytes(b + struct.pack("<Q", fnv1a64(b)))


def parse(b):
    """(config bytes or None, [(relative path, bytes)]); raises ValueError on a damaged pack."""
    if len(b) < 20 or b[:8] != MAGIC:
        raise ValueError("not a game pack (no AG5PACK1 header)")
    body, (want,) = b[:-8], struct.unpack("<Q", b[-8:])
    if fnv1a64(body) != want:
        raise ValueError("game pack checksum does not match (damaged or cut short)")
    at = 8
    (count,) = struct.unpack_from("<I", body, at)
    at += 4
    config, files = None, []
    for _ in range(count):
        (nl,) = struct.unpack_from("<H", body, at)
        name = body[at + 2:at + 2 + nl].decode("utf-8")
        at += 2 + nl
        (dl,) = struct.unpack_from("<I", body, at)
        data = bytes(body[at + 4:at + 4 + dl])
        at += 4 + dl
        if at > len(body):
            raise ValueError("game pack entry runs past its end")
        if name == "game.json":
            config = data
        elif name.startswith("files/"):
            files.append((name[6:], data))
        else:
            raise ValueError("unknown game pack entry %s" % name)
    if at != len(body):
        raise ValueError("game pack has bytes after its last entry")
    return config, files


def main(argv):
    ap = argparse.ArgumentParser(description="Ami-Go G5 game pack")
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("pack")
    p.add_argument("--game", required=True)
    p.add_argument("--config")
    p.add_argument("--out", required=True)
    p = sub.add_parser("list")
    p.add_argument("pack")
    p = sub.add_parser("unpack")
    p.add_argument("pack")
    p.add_argument("--out", required=True)
    a = ap.parse_args(argv)
    if a.cmd == "pack":
        config = None
        if a.config:
            with open(a.config, "rb") as f:
                config = f.read()
        files = walk(a.game)
        with open(a.out, "wb") as f:
            f.write(build(files, config))
        print("packed %d files%s into %s" % (len(files), " and game.json" if config else "", a.out))
        return 0
    with open(a.pack, "rb") as f:
        config, files = parse(f.read())
    if a.cmd == "list":
        if config is not None:
            print("%10d  game.json" % len(config))
        for n, d in files:
            print("%10d  files/%s" % (len(d), n))
        return 0
    if config is not None:
        os.makedirs(a.out, exist_ok=True)
        with open(os.path.join(a.out, "game.json"), "wb") as f:
            f.write(config)
    for n, d in files:
        if ".." in n.split("/") or os.path.isabs(n):
            raise ValueError("unsafe entry name %s" % n)
        p = os.path.join(a.out, "files", *n.split("/"))
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, "wb") as f:
            f.write(d)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except (ValueError, struct.error) as e:
        print("g5_pack.py: %s" % e, file=sys.stderr)
        sys.exit(2)
