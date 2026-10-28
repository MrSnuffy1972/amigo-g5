#!/usr/bin/env python3
import hashlib,json,os,re,tempfile,time
W=os.environ.get('CS_ROOT','/w'); CS=os.path.join(W,'gates','CS'); OUT=os.path.join(CS,'snapshot_assessment.json')
def rd(p):
 try:return open(p,errors='replace').read()
 except:return ''
def js(p):
 try:return json.load(open(p))
 except:return {}
def sf(p):
 try:
  h=hashlib.sha256()
  with open(p,'rb') as f:
   for b in iter(lambda:f.read(1048576),b''):h.update(b)
  return h.hexdigest()
 except:return None
def atom(p,x):
 os.makedirs(os.path.dirname(p),exist_ok=True); fd,t=tempfile.mkstemp(dir=os.path.dirname(p),prefix='.snapshot-assess-')
 try:
  with os.fdopen(fd,'w') as f:json.dump(x,f,indent=1,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
  os.replace(t,p)
 finally:
  if os.path.exists(t):os.unlink(t)
def one(n):
 s=rd(os.path.join(CS,n)).strip();return s or None
pf=js(os.path.join(CS,'preflight.json')); res=js(os.path.join(CS,'result.json'))
pf=dict(pf.get('content_identity') or {},**pf)  # c16 A1: preflight keeps the tree hashes under content_identity
log=rd(os.path.join(CS,'3-snap.log')); regs=rd(os.path.join(W,'out','snap','snap_regs.txt'))
m=re.search(r'SNAPSHOT written to .* at PC ([0-9A-Fa-f]{6}) \(frame (\d+)\)',log)
rule='B' if re.search(r'SNAPSHOT rule B \(depth 0, IPL mask \d+\)',log) else ('A' if m else None)
pc=one('snap_pc'); frame=one('snap_frame'); r_pc=None; r_frame=None
for line in regs.splitlines():
 q=line.split()
 if len(q)==2 and q[0]=='pc':r_pc=q[1][-6:].upper()
 if len(q)==2 and q[0]=='frame':r_frame=q[1]
files={}
for rel in ('out/snap/snap_regs.txt','out/snap/snap_chip.bin','out/snap/snap_slow.bin','gates/CS/snap_pc','gates/CS/snap_frame','gates/CS/3-snap.log','game.json','game-orig.json'):
 p=os.path.join(W,rel)
 if os.path.isfile(p):files[rel]={'bytes':os.path.getsize(p),'sha256':sf(p)}
warn=[]; err=[]
if not m:err.append('snapshot completion line missing')
if pc and r_pc and pc.upper()!=r_pc:err.append('snap_pc disagrees with snap_regs.txt')
if frame and r_frame and frame!=r_frame:err.append('snap_frame disagrees with snap_regs.txt')
if m and r_pc and m.group(1).upper()!=r_pc:err.append('3-snap.log PC disagrees with snap_regs.txt')
if m and r_frame and m.group(2)!=r_frame:err.append('3-snap.log frame disagrees with snap_regs.txt')
for rel in ('out/snap/snap_regs.txt','out/snap/snap_chip.bin','out/snap/snap_slow.bin'):
 if rel not in files:err.append('missing '+rel)
start_tree=((pf.get('phases') or {}).get('start') or {}).get('dev_kit_tree_sha256') or pf.get('dev_kit_tree_sha256')
final_tree=((pf.get('phases') or {}).get('final') or {}).get('dev_kit_tree_sha256') or pf.get('dev_kit_tree_sha256_final')
if start_tree and final_tree and start_tree!=final_tree:err.append('dev kit tree changed during run')
if not start_tree:warn.append('preflight dev_kit_tree_sha256 unavailable')
tools=((pf.get('phases') or {}).get('start') or {}).get('clean_sheet_tools_tree_sha256') or pf.get('clean_sheet_tools_tree_sha256')
if not tools:warn.append('preflight clean_sheet_tools_tree_sha256 unavailable')
g0={}
for rel in ('gates/G0/g0.json','gates/G0/MEMMAP.md','gates/G0/image.bin','gates/G0/g0ai_code.bin','gates/G0/probe.txt','out/port/ref/first_bytes.txt'):
 p=os.path.join(W,rel)
 if os.path.isfile(p):g0[rel]={'bytes':os.path.getsize(p),'sha256':sf(p)}
if not g0:warn.append('no G0 provenance artefact found')
out={'schema_version':1,'status':'fail' if err else ('warn' if warn else 'pass'),'snapshot':{'rule':rule,'pc':r_pc or pc,'frame':int(r_frame or frame) if str(r_frame or frame or '').isdigit() else None,'requested':one('snap_pc'),'log_pc':m.group(1).upper() if m else None,'log_frame':int(m.group(2)) if m else None},'provenance':{'reported_kit_rev':pf.get('reported_kit_rev'),'dev_kit_tree_sha256':start_tree,'clean_sheet_tools_tree_sha256':tools,'preflight_comparable_sha256':pf.get('preflight_comparable_sha256') or js(os.path.join(CS,'run_manifest.json')).get('preflight_comparable_sha256'),'snapshot_files':files,'g0_files':g0},'warnings':warn,'errors':err,'generated_at':int(time.time())}
atom(OUT,out);print(out['status'])
