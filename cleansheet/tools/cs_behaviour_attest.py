#!/usr/bin/env python3
import glob, hashlib, json, os, tempfile, time
W=os.environ.get("CS_ROOT","/w"); CS=os.path.join(W,"gates","CS")
OUT=os.path.join(CS,"behaviour_attestation.json")
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
def rd(p):
 try:return open(p,errors="replace").read()
 except OSError:return ""
def aggregate(paths,root=None):
 rows=[]
 for p in sorted(set(paths)):
  if not os.path.isfile(p) or os.path.islink(p):continue
  name=os.path.relpath(p,root) if root else os.path.basename(p)
  rows.append({"path":name,"bytes":os.path.getsize(p),"sha256":sf(p)})
 h=hashlib.sha256()
 for r in rows:h.update(r["path"].encode()+b"\0"+r["sha256"].encode()+b"\n")
 return rows,h.hexdigest(),sum(r["bytes"] for r in rows)
def atom(p,v):
 os.makedirs(os.path.dirname(p),exist_ok=True);fd,t=tempfile.mkstemp(prefix=".behaviour-attest-",dir=os.path.dirname(p))
 try:
  with os.fdopen(fd,"w") as f:json.dump(v,f,indent=1,sort_keys=True);f.write("\n");f.flush();os.fsync(f.fileno())
  os.replace(t,p)
 finally:
  if os.path.exists(t):os.unlink(t)
res=js(os.path.join(CS,"result.json")); pf=js(os.path.join(CS,"preflight.json")); ta=js(os.path.join(CS,"translation_attestation.json"))
imgext=("rgba","png","ppm")
native=[];reference=[]
for e in imgext:
 native+=glob.glob("/tmp/nat/p*."+e); reference+=glob.glob("/tmp/ref/f*."+e)
nrows,nsha,nbytes=aggregate(native,"/tmp/nat"); rrows,rsha,rbytes=aggregate(reference,"/tmp/ref")
logs=[]
for n in ("7-run.out","7-run.err","8-ref2.log","8-align.log","9-judge.log","summary.log","run_rc","native_frames","snap_pc","snap_frame","input_mode"):
 p=os.path.join(CS,n)
 if os.path.isfile(p):logs.append(p)
lrows,lsha,lbytes=aggregate(logs,W)
audio=[]
for p in ("/tmp/nat.raw","/tmp/ref.raw"):
 if os.path.isfile(p):audio.append({"path":p,"bytes":os.path.getsize(p),"sha256":sf(p)})
findings=[x[:240] for x in rd(os.path.join(CS,"7-run.err")).splitlines() if x.startswith("FINDING")]
errors=[];warnings=[]
if not nrows:errors.append("native frames missing")
if not rrows:errors.append("aligned reference frames missing")
if not os.path.isfile(os.path.join(CS,"9-judge.log")):warnings.append("judge log missing")
if not ta:warnings.append("translation attestation unavailable")
elid=ta.get("translation_identity_sha256") if ta else None
if ta and ta.get("status")=="fail":warnings.append("translation attestation failed")
identity={"translation_identity_sha256":elid,"native_frames_sha256":nsha,"reference_frames_sha256":rsha,"audio_native_sha256":next((x["sha256"] for x in audio if x["path"]=="/tmp/nat.raw"),None),"audio_reference_sha256":next((x["sha256"] for x in audio if x["path"]=="/tmp/ref.raw"),None),"evidence_logs_sha256":lsha,"result_class":res.get("class"),"snap_pc":res.get("snap_pc"),"snap_frame":res.get("snap_frame"),"snap_rule":res.get("snap_rule"),"input":res.get("input")}
h=hashlib.sha256()
for k in sorted(identity):h.update(k.encode()+b"\0"+str(identity[k]).encode()+b"\n")
out={"schema_version":1,"status":"fail" if errors else ("warn" if warnings else "pass"),"behaviour_identity_sha256":h.hexdigest(),"behaviour_identity_fields":identity,"native_frames":{"count":len(nrows),"bytes":nbytes,"aggregate_sha256":nsha,"files":nrows},"reference_frames":{"count":len(rrows),"bytes":rbytes,"aggregate_sha256":rsha,"files":rrows},"audio":audio,"evidence":{"aggregate_sha256":lsha,"bytes":lbytes,"files":lrows,"findings":findings},"outcome":{"class":res.get("class"),"run_rc":res.get("run_rc"),"picture":res.get("picture"),"content":res.get("content"),"active":res.get("active"),"sound":res.get("sound"),"native_frames":res.get("native_frames"),"wild_jump":res.get("wild_jump"),"rom_call":res.get("rom_call")},"provenance":{"translation_attestation_sha256":sf(os.path.join(CS,"translation_attestation.json")),"translation_identity_sha256":elid,"preflight_comparable_sha256":pf.get("preflight_comparable_sha256") or js(os.path.join(CS,"run_manifest.json")).get("preflight_comparable_sha256"),"dev_kit_tree_sha256":pf.get("dev_kit_tree_sha256"),"clean_sheet_tools_tree_sha256":pf.get("clean_sheet_tools_tree_sha256")},"warnings":warnings,"errors":errors,"generated_at":int(time.time())}
atom(OUT,out);print(out["status"],out["behaviour_identity_sha256"])
