#!/usr/bin/env python3
import hashlib,json,os,tempfile,time
W=os.environ.get("CS_ROOT","/w"); CS=os.path.join(W,"gates","CS"); GEN=os.path.join(W,"out","port","gen")
OUT=os.path.join(CS,"translation_attestation.json")
def sf(p):
 try:
  h=hashlib.sha256()
  with open(p,"rb") as f:
   for b in iter(lambda:f.read(1048576),b""):h.update(b)
  return h.hexdigest()
 except OSError:return None
def js(p):
 try:return json.load(open(p))
 except Exception:return {}
def tree(root,exts=None):
 rows=[]
 if not os.path.isdir(root):return rows,None
 for base,dirs,files in os.walk(root):
  dirs[:]=sorted(d for d in dirs if d not in ("__pycache__",".git"))
  for name in sorted(files):
   p=os.path.join(base,name); rel=os.path.relpath(p,root)
   if exts and not any(name.endswith(x) for x in exts):continue
   if os.path.islink(p) or not os.path.isfile(p):continue
   rows.append({"path":rel,"bytes":os.path.getsize(p),"sha256":sf(p)})
 h=hashlib.sha256()
 for r in rows:h.update(r["path"].encode()+b"\0"+r["sha256"].encode()+b"\n")
 return rows,h.hexdigest()
def atom(p,v):
 os.makedirs(os.path.dirname(p),exist_ok=True);fd,t=tempfile.mkstemp(prefix=".translation-attest-",dir=os.path.dirname(p))
 try:
  with os.fdopen(fd,"w") as f:json.dump(v,f,indent=1,sort_keys=True);f.write("\n");f.flush();os.fsync(f.fileno())
  os.replace(t,p)
 finally:
  if os.path.exists(t):os.unlink(t)
pf=js(os.path.join(CS,"preflight.json")); dl=js(os.path.join(CS,"discovery_ledger.json"))
pf=dict(pf.get("content_identity") or {},**pf);
files,gen_sha=tree(GEN,(".c",".h",".txt",".json"))
translated=os.path.join(GEN,"translated_pcs.txt");leaders=os.path.join(GEN,"leaders.txt")
def lines(p):
 try:return sum(1 for x in open(p,errors="replace") if x.strip())
 except OSError:return None
native=os.path.join(W,"out","native");game=os.path.join(W,"game.json")
build_logs=[]
for n in ["6-build.log"]+["6b-build-%d.log"%i for i in range(1,4)]:
 p=os.path.join(CS,n)
 if os.path.isfile(p):build_logs.append({"path":"gates/CS/"+n,"sha256":sf(p),"bytes":os.path.getsize(p)})
errors=[];warnings=[]
if not files:errors.append("generated translation tree missing")
if not os.path.isfile(native):errors.append("out/native missing")
if lines(translated) is None:errors.append("translated_pcs.txt missing")
if not build_logs:errors.append("native build log missing")
if not dl:warnings.append("discovery ledger unavailable")
elif not dl.get("fixed_point"):warnings.append("translation was built without a zero-address discovery result")
pre_gen=((pf.get("phases") or {}).get("generated") or {})
pre_build=((pf.get("phases") or {}).get("build") or {})
pre_final=((pf.get("phases") or {}).get("final") or {})
out={"schema_version":1,"status":"fail" if errors else ("warn" if warnings else "pass"),"translation":{"tree_sha256":gen_sha,"file_count":len(files),"files":files,"translated_pc_count":lines(translated),"translated_pcs_sha256":sf(translated),"leader_count":lines(leaders),"leaders_sha256":sf(leaders)},"build":{"native_sha256":sf(native),"native_bytes":os.path.getsize(native) if os.path.isfile(native) else None,"build_logs":build_logs,"game_json_sha256":sf(game)},"discovery":{"ledger_sha256":sf(os.path.join(CS,"discovery_ledger.json")),"fixed_point":dl.get("fixed_point") if dl else None,"terminal_reason":dl.get("terminal_reason") if dl else None,"seed_final":dl.get("seed_final") if dl else None},"preflight":{"reported_kit_rev":pf.get("reported_kit_rev"),"dev_kit_tree_sha256":pf.get("dev_kit_tree_sha256"),"clean_sheet_tools_tree_sha256":pf.get("clean_sheet_tools_tree_sha256"),"generated_phase":pre_gen,"build_phase":pre_build,"final_phase":pre_final},"warnings":warnings,"errors":errors,"generated_at":int(time.time())}
identity={"translation_tree_sha256":gen_sha,"native_sha256":out["build"]["native_sha256"],"game_json_sha256":out["build"]["game_json_sha256"],"discovery_ledger_sha256":out["discovery"]["ledger_sha256"],"dev_kit_tree_sha256":out["preflight"]["dev_kit_tree_sha256"],"clean_sheet_tools_tree_sha256":out["preflight"]["clean_sheet_tools_tree_sha256"]}
h=hashlib.sha256()
for k in sorted(identity):h.update(k.encode()+b"\0"+str(identity[k]).encode()+b"\n")
out["translation_identity_fields"]=identity;out["translation_identity_sha256"]=h.hexdigest()
atom(OUT,out);print(out["status"],out["translation_identity_sha256"])
