#!/usr/bin/env python3
import hashlib, os, sys
SKIP_DIRS={".git","__pycache__","out","build"}
BUILD_EXT={".c",".h",".inc",".py",".sh",".json",".mk",".cmake"}
BUILD_NAMES={"Makefile","OBJS.txt","CMakeLists.txt"}
TOOLS_EXT={".py",".sh",".json"}
# G4 install (Cowork 25 Oct 2026, Karl approved): the G4-only kit files are NOT clean-sheet G2/G3 inputs, so adding them (tag cs-g4-1)
# leaves the dev identity and kit_rev unchanged (no low-gate refresh, no G2/G3 result invalidated). Paths relative to the dev root.
G4_EXEMPT={"port/native_windows.fn","port/symcheck_g4.sh"}
def excluded(rel):
    parts=rel.split(os.sep)
    return any(p in SKIP_DIRS for p in parts) or any(".bak" in p or ".old-" in p or p.endswith(".pyc") or ".disabled-" in p for p in parts)
def tree(root,kind):
    rows=[]
    for base,dirs,files in os.walk(root):
        dirs[:]=sorted(d for d in dirs if not excluded(os.path.relpath(os.path.join(base,d),root)))
        for name in sorted(files):
            p=os.path.join(base,name); rel=os.path.relpath(p,root)
            if excluded(rel) or os.path.islink(p) or not os.path.isfile(p):continue
            if kind=="dev" and rel.replace(os.sep,"/") in G4_EXEMPT:continue
            ext=os.path.splitext(name)[1]
            if kind=="tools" and ext not in TOOLS_EXT:continue
            if kind=="dev" and ext not in BUILD_EXT and name not in BUILD_NAMES:continue
            h=hashlib.sha256()
            with open(p,"rb") as f:
                for b in iter(lambda:f.read(1048576),b""):h.update(b)
            rows.append((rel,h.hexdigest()))
    out=hashlib.sha256()
    for rel,digest in rows:out.update(rel.encode()+b"\0"+digest.encode()+b"\n")
    return out.hexdigest(),len(rows)
def combined_dev(root,kitfix):
    dev,n=tree(root,"dev")
    h=hashlib.sha256();h.update(b"dev-tree\0"+dev.encode()+b"\n")
    if kitfix:
        one=hashlib.sha256(open(kitfix,"rb").read()).hexdigest()
        h.update(b"kitfix\0"+one.encode()+b"\n")
    return h.hexdigest(),n
if __name__=="__main__":
    kind,root=sys.argv[1:3]
    print(tree(root,kind)[0])
