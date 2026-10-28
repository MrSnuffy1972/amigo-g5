#!/usr/bin/env python3
import hashlib,json,os,tempfile,time
W=os.environ.get("CS_ROOT","/w"); CS=os.path.join(W,"gates","CS")
OUT=os.path.join(CS,"promotion_decision.json")
def js(p):
 try:return json.load(open(p))
 except Exception:return {}
def sf(p):
 try:
  h=hashlib.sha256()
  with open(p,"rb") as f:
   for b in iter(lambda:f.read(1048576),b""):h.update(b)
  return h.hexdigest()
 except OSError:return None
def atom(p,v):
 os.makedirs(os.path.dirname(p),exist_ok=True);fd,t=tempfile.mkstemp(prefix=".promotion-",dir=os.path.dirname(p))
 try:
  with os.fdopen(fd,"w") as f:json.dump(v,f,indent=1,sort_keys=True);f.write("\n");f.flush();os.fsync(f.fileno())
  os.replace(t,p)
 finally:
  if os.path.exists(t):os.unlink(t)
def first_json(paths):
 for p in paths:
  if os.path.isfile(p):return p,js(p)
 return None,{}
res=js(os.path.join(CS,"result.json")); pf=js(os.path.join(CS,"preflight.json")); rm=js(os.path.join(CS,"run_manifest.json"))
pf=dict(pf.get("content_identity") or {},**pf)  # c16 A1: preflight keeps the tree hashes under content_identity
sa=js(os.path.join(CS,"snapshot_assessment.json")); dl=js(os.path.join(CS,"discovery_ledger.json")); ta=js(os.path.join(CS,"translation_attestation.json")); ba=js(os.path.join(CS,"behaviour_attestation.json"))
gp,golden=first_json([os.path.join(CS,"golden_verification.json"),os.path.join(W,"golden_verification.json"),os.path.join(W,"out","golden_verification.json")])
def status(x):return x.get("status") if x else None
def golden_state(g):
 if not gp:return "not-applicable"
 if not isinstance(g,dict) or g.get("schema_version")!=1 or g.get("kind")!="cs-golden-workspace":return "invalid"
 if g.get("applicability") not in ("required","informational") or g.get("outcome") not in ("matched","failed","pending"):return "invalid"
 if not isinstance(g.get("blocking"),bool) or not isinstance(g.get("checks"),list):return "invalid"
 if g["applicability"]=="informational":return "not-applicable"
 return g["outcome"]
def golden_ok(g):
 if not g:return None
 for k in ("match","matched","pass","passed","ok"):
  if k in g:return bool(g[k])
 return str(g.get("status",g.get("result",""))).lower() in ("match","matched","pass","passed","ok","golden-match")
checks=[]
def add(name,level,ok,evidence):checks.append({"name":name,"level":level,"ok":ok,"evidence":evidence})
add("result class is promotable","hard",str(res.get("class","")).startswith("pass:"),res.get("class"))
add("run return code is zero","hard",res.get("run_rc")==0,res.get("run_rc"))
add("symcheck passed","hard",res.get("symcheck")=="PASS",res.get("symcheck"))
add("preflight completed final phase","hard","final" in (pf.get("phases_run") or []),pf.get("phases_run"))
add("preflight has no errors","hard",not (pf.get("errors") or []),pf.get("errors"))
add("snapshot assessment available","soft",bool(sa),status(sa))
add("snapshot assessment did not fail","hard",bool(sa) and status(sa)!="fail",status(sa))
add("discovery ledger available","soft",bool(dl),status(dl))
add("discovery reached fixed point","soft",bool(dl) and bool(dl.get("fixed_point")),dl.get("terminal_reason") if dl else None)
add("translation attestation available","hard",bool(ta),status(ta))
add("translation attestation did not fail","hard",bool(ta) and status(ta)!="fail",status(ta))
add("behaviour attestation available","hard",bool(ba),status(ba))
add("behaviour attestation did not fail","hard",bool(ba) and status(ba)!="fail",status(ba))
gstate=golden_state(golden)
add("golden policy satisfied","soft" if gstate=="pending" else "hard",gstate in ("matched","not-applicable"),gstate)
add("content provenance available","hard",bool(pf.get("dev_kit_tree_sha256") and pf.get("clean_sheet_tools_tree_sha256")),{"dev":pf.get("dev_kit_tree_sha256"),"tools":pf.get("clean_sheet_tools_tree_sha256")})
hard_failed=[c for c in checks if c["level"]=="hard" and not c["ok"]]
soft_failed=[c for c in checks if c["level"]=="soft" and not c["ok"]]
if hard_failed:decision="blocked"
elif soft_failed:decision="diagnostic-only"
else:decision="eligible"
identity={"decision":decision,"result_class":res.get("class"),"snapshot_assessment_sha256":sf(os.path.join(CS,"snapshot_assessment.json")),"discovery_ledger_sha256":sf(os.path.join(CS,"discovery_ledger.json")),"translation_identity_sha256":ta.get("translation_identity_sha256"),"behaviour_identity_sha256":ba.get("behaviour_identity_sha256"),"golden_verification_sha256":sf(gp) if gp else None,"preflight_comparable_sha256":rm.get("preflight_comparable_sha256") or pf.get("preflight_comparable_sha256")}
h=hashlib.sha256()
for k in sorted(identity):h.update(k.encode()+b"\0"+str(identity[k]).encode()+b"\n")
out={"schema_version":1,"policy":"amigo-promotion-v1","decision":decision,"decision_identity_sha256":h.hexdigest(),"decision_identity_fields":identity,"checks":checks,"hard_failures":[c["name"] for c in hard_failed],"soft_failures":[c["name"] for c in soft_failed],"evidence":{"result_json_sha256":sf(os.path.join(CS,"result.json")),"preflight_json_sha256":sf(os.path.join(CS,"preflight.json")),"run_manifest_json_sha256":sf(os.path.join(CS,"run_manifest.json")),"golden_verification_path":os.path.relpath(gp,W) if gp else None},"generated_at":int(time.time())}
atom(OUT,out);print(decision,out["decision_identity_sha256"])
