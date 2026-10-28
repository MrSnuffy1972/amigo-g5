#!/usr/bin/env python3
"""adfx.py: extract the AmigaDOS files from an ADF (OFS or FFS, DD or HD), and the bootblock.

Written 13 Oct 2026 for Ami-Go's library pre-analysis. Code, not a model: the AmigaDOS disk layout is fixed.
Usage: python3 adfx.py DISK.adf OUTDIR
Writes OUTDIR/bootblock.bin (always) and, for a DOS disk, every file under OUTDIR/files/ with its directory
path, plus OUTDIR/adf.json {"dos": bool, "ffs": bool, "volume": str, "files": [[path, size], ...],
"startup": [lines]}. A non-DOS disk (custom trackloader, common for games) has only the bootblock.
"""
import json, os, struct, sys

BS = 512


def partitions(path):
    """The AmigaDOS partitions of a hard-disk image (.hdf): the whole file when it starts with DOS (a UAE
    hardfile), else each PART of its Rigid Disk Block. Returns [(name, offset, length)]."""
    d = open(path, "rb").read(1 << 16)
    if d[:3] == b"DOS":
        return [("hdf", 0, os.path.getsize(path))]
    out = []
    f = open(path, "rb")
    for b in range(16):
        f.seek(b * BS); blk = f.read(BS)
        if blk[:4] != b"RDSK":
            continue
        part = struct.unpack_from(">I", blk, 28)[0]
        hops = 0
        while part not in (0, 0xFFFFFFFF) and hops < 32:
            f.seek(part * BS); pb = f.read(BS)
            if pb[:4] != b"PART":
                break
            name = pb[37:37 + pb[36]].decode("latin1", "replace") or "part%d" % hops
            env = [struct.unpack_from(">I", pb, 128 + 4 * i)[0] for i in range(17)]
            surf, bpt, low, high = env[3], env[5], env[9], env[10]
            cyl = surf * bpt * BS
            out.append((name, low * cyl, (high - low + 1) * cyl))
            part = struct.unpack_from(">I", pb, 16)[0]
            hops += 1
        break
    return out


def extract(path, out):
    d = open(path, "rb").read()
    os.makedirs(out, exist_ok=True)
    open(os.path.join(out, "bootblock.bin"), "wb").write(d[:1024])
    info = {"dos": d[:3] == b"DOS", "ffs": bool(d[3] & 1) if len(d) > 3 else False, "volume": "",
            "files": [], "startup": [], "errors": []}
    if not info["dos"] or len(d) < 880 * 1024:
        json.dump(info, open(os.path.join(out, "adf.json"), "w"), indent=1)
        return info
    nblocks = len(d) // BS
    B = lambda n: d[n * BS:(n + 1) * BS] if 0 < n < nblocks else None
    L = lambda b, o: struct.unpack_from(">I", b, o)[0]
    S = lambda b, o: struct.unpack_from(">i", b, o)[0]
    nm = lambda b: b[433:433 + min(b[432], 30)].decode("latin1", "replace")
    # root key = (reserved + highest block) / 2 with 2 reserved blocks: 880 on a floppy, and the same formula on a
    # hard-disk partition (13 Oct: 526 WHDLoad .hdf files in the library were skipped before this)
    rk = (nblocks + 1) // 2
    root = B(rk)
    if root is None or L(root, 0) != 2 or S(root, 508) != 1:
        info["errors"].append("no root block at %d" % rk)
        json.dump(info, open(os.path.join(out, "adf.json"), "w"), indent=1)
        return info
    info["volume"] = nm(root)
    ffs = info["ffs"]
    seen = set()

    def read_file(h):
        size = L(h, 324)
        data = bytearray()
        blocks = []
        hdr, hops = h, 0
        while hdr is not None and hops < 2000:
            for i in range(72):
                p = L(hdr, 24 + 4 * (71 - i))
                if p:
                    blocks.append(p)
            ext = L(hdr, 504)
            hdr = B(ext) if ext else None
            hops += 1
        for p in blocks:
            b = B(p)
            if b is None:
                break
            if ffs:
                data += b
            else:
                data += b[24:24 + min(L(b, 12), 488)]
            if len(data) >= size:
                break
        return bytes(data[:size])

    def walk(blk, prefix, depth):
        if depth > 12:
            return
        for i in range(72):
            h = L(blk, 24 + 4 * i)
            hops = 0
            while h and hops < 500:
                if h in seen:
                    break
                seen.add(h)
                e = B(h)
                if e is None or L(e, 0) != 2:
                    info["errors"].append("bad header block %d" % h)
                    break
                name = nm(e).replace("/", "_").replace("\0", "")
                rel = (prefix + "/" + name) if prefix else name
                st = S(e, 508)
                if st == 2:
                    walk(e, rel, depth + 1)
                elif st == -3:
                    try:
                        data = read_file(e)
                        dst = os.path.join(out, "files", rel)
                        os.makedirs(os.path.dirname(dst), exist_ok=True)
                        open(dst, "wb").write(data)
                        info["files"].append([rel, len(data)])
                        if rel.lower() == "s/startup-sequence":
                            info["startup"] = data.decode("latin1", "replace").splitlines()[:60]
                    except Exception as ex:
                        info["errors"].append("%s: %s" % (rel, ex))
                h = L(e, 496)
                hops += 1

    walk(root, "", 0)
    json.dump(info, open(os.path.join(out, "adf.json"), "w"), indent=1)
    return info


if __name__ == "__main__":
    i = extract(sys.argv[1], sys.argv[2])
    print("%s disk '%s': %d files%s" % ("DOS" if i["dos"] else "non-DOS", i["volume"], len(i["files"]),
                                         ", %d errors" % len(i["errors"]) if i["errors"] else ""))
