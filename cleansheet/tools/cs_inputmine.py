#!/usr/bin/env python3
# Copilot input intelligence (sha 85d9137a, kept verbatim as cs_inputmine.py.copilot-85d9137a) + Cowork c22l fixes F1 F2 F4.
import argparse,hashlib,json,os,re,tempfile
REGS={"DFF00A":"JOY0DAT","DFF00C":"JOY1DAT","DFF012":"POT0DAT","DFF014":"POT1DAT","DFF016":"POTINP","DFF034":"POTGO","BFE001":"CIAAPRA","BFEC01":"CIAASDR","BFED01":"CIAAICR","BFEE01":"CIAACRA"}
def h(p):
 try:return hashlib.sha256(open(p,"rb").read()).hexdigest()
 except:return None
def put(p,x):
 os.makedirs(os.path.dirname(p),exist_ok=True);fd,t=tempfile.mkstemp(dir=os.path.dirname(p))
 with os.fdopen(fd,"w") as f:json.dump(x,f,indent=1,sort_keys=True);f.write("\n");f.flush();os.fsync(f.fileno())
 os.chmod(t,0o644);os.replace(t,p)   # Cowork c22l F2: mkstemp makes 0600
def walk(root):
 for d,ds,fs in os.walk(root):
  ds[:]=[x for x in ds if x not in (".git","_old","__pycache__")]
  for n in fs:
   p=os.path.join(d,n)
   if n.endswith((".c",".h",".s",".asm",".txt",".log",".json",".jsonl",".md",".py")) and os.path.getsize(p)<8388608:yield p
# Cowork c22l (19 Oct 2026): forms the translator actually emits in out/port/gen (measured on ws 32 and 263). The REGS regex
# above missed all of them: "\bJOY0DAT\b" cannot match inside intent_JOY0DAT_r16 (underscore is a word character) and
# "0xBFE001\b" cannot match the C literal 0xBFE001u. Added: (1) named accessors intent_<REG>_r16()/_w16(); (2) hw_r16(0xA) /
# hw_w16(0x34) style offset accessors; (3) base-relative access: A[n] = 0xDFF000u (custom base) or 0xBFE001u (CIA-A base)
# followed in the same file by "A[n] + 0x<off>u" with off in the input register set (custom 0x00A JOY0DAT, 0x00C JOY1DAT,
# 0x012 POT0DAT, 0x014 POT1DAT, 0x016 POTINP, 0x034 POTGO; CIA-A from 0xBFE001: +0xC00 SDR, +0xD00 ICR, +0xE00 CRA) and
# RB(A[n]) on the CIA-A base (PRA). Any other write to A[n] forgets its base. Confidence: "named" for (1), "possible" else.
CUSTOM_OFF={0x00A:"JOY0DAT",0x00C:"JOY1DAT",0x012:"POT0DAT",0x014:"POT1DAT",0x016:"POTINP",0x034:"POTGO"}
CIAA_OFF={0x000:"CIAAPRA",0xC00:"CIAASDR",0xD00:"CIAAICR",0xE00:"CIAACRA"}
NAME_ADDR={v:k for k,v in REGS.items()}
GEN_NAMED=re.compile(r"\bintent_("+"|".join(sorted(set(CUSTOM_OFF.values())))+r")_[rw](?:8|16|32)\b")
GEN_HW=re.compile(r"\bhw_[rw](?:8|16|32)\(\s*(?:0x)?([0-9A-Fa-f]+)[uU]?\s*[,)]")
GEN_SETA=re.compile(r"\bA\[([0-7])\]\s*(=|\+=|-=)\s*([^;]*);")
GEN_REL=re.compile(r"\bA\[([0-7])\]\s*\+\s*0x([0-9A-Fa-f]+)[uU]?\b")
GEN_RB=re.compile(r"\b[RW]B\(\s*A\[([0-7])\]\s*[),]")
def gen_sites(s,rel):
 out=[];base={}
 def add(n,pos,conf):out.append({"source":n,"address":"0x"+NAME_ADDR[n],"file":rel,"line":s.count("\n",0,pos)+1,"confidence":conf,"context":" ".join(s[max(0,pos-120):pos+180].split())[:360]})
 for m in GEN_NAMED.finditer(s):add(m.group(1),m.start(),"named")
 for m in GEN_HW.finditer(s):
  n=CUSTOM_OFF.get(int(m.group(1),16))
  if n:add(n,m.start(),"possible")
 ev=[(m.start(),"set",m) for m in GEN_SETA.finditer(s)]+[(m.start(),"rel",m) for m in GEN_REL.finditer(s)]+[(m.start(),"rb",m) for m in GEN_RB.finditer(s)]
 for pos,kind,m in sorted(ev,key=lambda e:e[0]):
  r=m.group(1)
  if kind=="set":
   v=m.group(3).strip().rstrip("uUlL") if m.group(2)=="=" else ""
   try:base[r]=int(v,16) if v.lower().startswith("0x") else None
   except ValueError:base[r]=None
  elif kind=="rel":
   b=base.get(r);off=int(m.group(2),16)
   if b==0xDFF000 and off in CUSTOM_OFF:add(CUSTOM_OFF[off],pos,"possible")
   elif b==0xBFE001 and off in CIAA_OFF:add(CIAA_OFF[off],pos,"possible")
  elif kind=="rb" and base.get(r)==0xBFE001:add("CIAAPRA",pos,"possible")
 return out
def collect(a):
 roots=[os.path.join(a.root,x) for x in ("evidence","out/port/gen","out/port/ref","gates/G0","gates/G1","snapcfg")]
 if a.phase=="snapshot":roots=[os.path.join(a.root,"snapcfg")]   # Cowork c22l F4: the other roots were just scanned by g0-static
 roots+=[x for x in (a.reference_log,a.first_bytes,a.discover_log,a.snapshot) if x]
 sites=[];seen=set()
 for r in roots:
  if not os.path.exists(r):continue
  for p in ([r] if os.path.isfile(r) else walk(r)):
   if p in seen:continue
   seen.add(p)
   try:s=open(p,"rb").read().decode("latin1","replace")
   except:continue
   for x,n in REGS.items():
    for m in re.finditer(r"(?i)(?:0x|\$)?0*"+x+r"(?:[uUlL]{1,3})?\b|\b"+n+r"\b",s):
     sites.append({"source":n,"address":"0x"+x,"file":os.path.relpath(p,a.root),"line":s.count("\n",0,m.start())+1,"confidence":"possible","context":" ".join(s[max(0,m.start()-120):m.end()+180].split())[:360]})
   if p.endswith((".c",".h")):sites+=gen_sites(s,os.path.relpath(p,a.root))
 u={(x["source"],x["file"],x["line"]):x for x in sites};c={}
 for x in u.values():c[x["source"]]=c.get(x["source"],0)+1
 put(a.output,{"schema_version":1,"kind":"amigo-input-evidence","phase":a.phase,"files_scanned":len(seen),"source_counts":c,"sites":sorted(u.values(),key=lambda x:(x["source"],x["file"],x["line"]))})
def propose(a):
 try:d=json.load(open(a.static))
 except:d={}
 src=sorted((d.get("source_counts") or {}).keys());tok=[]
 for s in src:
  q=["L","R","U","D","F"] if s.startswith("JOY") else (["SP","ENTER","ESC","F1"] if s=="CIAASDR" else ["F"])
  for x in q:
   if x not in tok:tok.append(x)
 put(a.output,{"schema_version":1,"kind":"amigo-input-candidates","sources":src,"controls":tok,"sequence":[{"tick":60+i*60,"token":x,"duration":8} for i,x in enumerate(tok[:8])],"confidence":"static-possible","complete":False})
def merge(a):
 d=os.path.join(a.root,"gates/INPUT");arts=[];counts={};sites=[]
 for n in sorted(os.listdir(d)) if os.path.isdir(d) else []:
  if not n.endswith(".json") or n=="input_manifest.json":continue
  p=os.path.join(d,n)
  try:j=json.load(open(p))
  except:continue
  arts.append({"file":n,"sha256":h(p),"kind":j.get("kind"),"phase":j.get("phase")});sites+=j.get("sites") or []
  for k,v in (j.get("source_counts") or {}).items():counts[k]=max(counts.get(k,0),v)
 u={(x.get("source"),x.get("file"),x.get("line")):x for x in sites}
 put(a.output,{"schema_version":1,"kind":"amigo-input-manifest","status":"observation-only","complete":False,"source_counts":counts,"artifacts":arts,"sites":list(u.values())})
def ev(p):
 z=[]
 try:
  for l in open(p,errors="replace"):
   try:z.append(json.loads(l))
   except:pass
 except:pass
 return z
def correlate(a):
 n,r=ev(a.native_events),ev(a.reference_events);ns={x.get("source") for x in n if x.get("source")};rs={x.get("source") for x in r if x.get("source")};both=sorted(ns&rs)
 put(a.output,{"schema_version":1,"kind":"amigo-input-transitions","phase":"cs-g2-g3-observe","classification":"input-wait:confirmed" if both else "unknown-static","native_event_count":len(n),"reference_event_count":len(r),"common_sources":both,"proven_controls":[],"transitions":[],"manifest_sha256":h(a.manifest),"judge_sha256":h(a.judge),"native_audio_sha256":h(a.native_audio),"reference_audio_sha256":h(a.reference_audio),"enforced":False})
def main():
 p=argparse.ArgumentParser();s=p.add_subparsers(dest="cmd",required=True)
 q=s.add_parser("collect");q.add_argument("--phase",required=True);q.add_argument("--root",default="/w");q.add_argument("--reference-log");q.add_argument("--first-bytes");q.add_argument("--discover-log");q.add_argument("--snapshot");q.add_argument("--generated");q.add_argument("--script");q.add_argument("--output",required=True)
 q=s.add_parser("propose");q.add_argument("--root",default="/w");q.add_argument("--static",required=True);q.add_argument("--snapshot");q.add_argument("--output",required=True)
 q=s.add_parser("merge");q.add_argument("--root",default="/w");q.add_argument("--output",required=True)
 q=s.add_parser("correlate");q.add_argument("--root",default="/w");q.add_argument("--manifest",required=True);q.add_argument("--native-events",required=True);q.add_argument("--reference-events",required=True);q.add_argument("--native-frames");q.add_argument("--reference-frames");q.add_argument("--native-audio");q.add_argument("--reference-audio");q.add_argument("--judge");q.add_argument("--output",required=True)
 a=p.parse_args();globals()[a.cmd](a)
if __name__=="__main__":main()
