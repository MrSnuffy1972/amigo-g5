#!/usr/bin/env bash
# export.sh: rebuild the G5 toolkit repo from Ryzen's live sources, read only, then leak-check it.
# Cowork sub-agent, 28 Oct 2026. Source trees are only read; nothing outside /srv/fast/amigo/export is written.
#
#   bash /srv/fast/amigo/export/export.sh            first export: builds g5-repo, git init, one commit
#   bash /srv/fast/amigo/export/export.sh --update   later: rebuilds into .stage, then refreshes only the exported
#                                                    trees (kit/, cleansheet/, g4/ and the exported docs) in g5-repo,
#                                                    leak-checks the result and commits. Pull main into g5-repo first.
#
# What goes in: the clean-sheet kit (kit-intent HEAD, git archive, minus port.old-* and the prebuilt SDL2 mingw
# libraries), the cleansheet tools, judge.py, the G4 build scripts, design docs, and the hand-written files in
# export-static/ (CLAUDE.md, tests, leakcheck, CI). What never goes in: Kickstart ROMs, WHDLoad RTBs, game files,
# disk images, frames, workspaces, reports, logs, databases, keys and tokens. The leak check enforces it and the
# script refuses to commit on any hit.
set -euo pipefail
EX=/srv/fast/amigo/export
REPO=${G5_REPO:-$EX/g5-repo}
STATIC=$EX/export-static
KI=/srv/fast/amigo/claude-run/intent/kit-intent
CS=/srv/fast/cleansheet
JUDGE=/srv/fast/amigo/claude-run/intent/judge.py
G4=/srv/fast/amigo/scratch/g4
ROMS=/srv/fast/amigo/tools/kickroms
RTB=/srv/fast/amigo/tools/whdload/rtb

MODE=${1:-first}
if [ "$MODE" = "--update" ]; then
  [ -d "$REPO/.git" ] || { echo "no $REPO/.git: run without --update first"; exit 2; }
  OUT=$EX/.stage
  [ -e "$OUT" ] && rm -rf "$OUT"   # export's own scratch folder only
else
  [ -e "$REPO" ] && { echo "$REPO exists: use --update"; exit 2; }
  OUT=$REPO
fi
mkdir -p "$OUT"

# 1. kit: committed HEAD of kit-intent only (no uncommitted work, no ignored files)
KREV=$(git -C "$KI" rev-parse --short HEAD)
mkdir -p "$OUT/kit"
git -C "$KI" archive --format=tar HEAD amigo | tar -x -C "$OUT/kit" \
  --exclude='amigo/port.old-*' --exclude='*.old-*' --exclude='amigo/port/sdl2-mingw' \
  --exclude='__pycache__' --exclude='*.pyc' --exclude='*.bak*'

# 2. cleansheet tools, judge, kit-rev helpers
mkdir -p "$OUT/cleansheet/tools"
rsync -a --exclude='__pycache__' --exclude='*.pyc' --exclude='*.bak*' --exclude='*.old*' --exclude='*.copilot-*' \
  --exclude='*.log' --exclude='*/' "$CS/tools/" "$OUT/cleansheet/tools/"
cp -p "$JUDGE" "$OUT/cleansheet/tools/judge.py"
cp -p "$CS/cs_content_id.py" "$CS/cs_gates.py" "$OUT/cleansheet/"

# 3. G4 build scripts (the installed g4_inner.sh, the prototype runner)
mkdir -p "$OUT/g4"
cp -p "$CS/g4/g4_inner.sh" "$OUT/g4/g4_inner.sh"
cp -p "$G4/run_all.sh" "$OUT/g4/run_all.sh"

# 4. design docs
mkdir -p "$OUT/docs"
cp -p "$G4/G4-DESIGN.md" "$OUT/docs/G4-DESIGN.md"
cp -p /srv/fast/amigo/kit/NATIVE-RUNTIME.md "$OUT/docs/NATIVE-RUNTIME.md"
cp -p "$CS/LESSONS.md" "$OUT/docs/CLEANSHEET-LESSONS.md"
cp -p "$CS/CLEANUP.md" "$OUT/docs/CLEANSHEET-CLEANUP.md"

# 5. hand-written files (CLAUDE.md, tests, leakcheck, CI, notes); export.sh itself for the record
rsync -a --exclude='*.bak*' "$STATIC/" "$OUT/"
cp -p "$EX/export.sh" "$OUT/tools/export.sh"

# 6. hash list (sha256 of every ROM and RTB file; hashes only) so the cloud and CI check catches an exact copy
{ echo "# sha256 of Kickstart ROMs and WHDLoad RTBs on Ryzen (hashes only). Written by export.sh $(date -u +%F)"
  find "$ROMS" "$RTB" -type f -size +1k -print0 | sort -z | xargs -0 sha256sum | awk '{print $1}'
} > "$OUT/tools/leak-hashes.txt"

# 7. manifest
{ echo "export: $(date -u +%FT%TZ)"
  echo "kit-intent HEAD: $KREV"
  echo "judge.py sha256: $(sha256sum "$JUDGE" | cut -c1-16)"
  echo "g4_inner.sh sha256: $(sha256sum "$CS/g4/g4_inner.sh" | cut -c1-16)"
} > "$OUT/EXPORT-MANIFEST.txt"

# 8. leak check with the real ROMs and RTBs (Ryzen only); no commit on a hit
python3 "$OUT/tools/leakcheck.py" --roms "$ROMS" --rtb "$RTB" --hashes "$OUT/tools/leak-hashes.txt" \
  --allow "$OUT/tools/leak-allow.txt" --hosts "$EX/leak-hosts.txt" "$OUT" | tee "$EX/leakcheck-last.txt"
rc=${PIPESTATUS[0]}
[ "$rc" = 0 ] || { echo "LEAK CHECK FAILED (rc $rc): nothing committed"; exit 1; }

# 9. commit (local only: this script never adds a remote or pushes)
# --update refreshes ONLY the exported trees (kit/, cleansheet/, g4/), the four exported docs, the hash list and the
# manifest. Everything the repo has grown since (rust/, new tests, CLAUDE.md edits, docs/G5-NOTES.md) is left alone.
if [ "$MODE" = "--update" ]; then
  for d in kit cleansheet g4; do rsync -a --delete "$OUT/$d/" "$REPO/$d/"; done
  for f in docs/G4-DESIGN.md docs/NATIVE-RUNTIME.md docs/CLEANSHEET-LESSONS.md docs/CLEANSHEET-CLEANUP.md \
           tools/leak-hashes.txt tools/export.sh EXPORT-MANIFEST.txt; do cp -p "$OUT/$f" "$REPO/$f"; done
  python3 "$REPO/tools/leakcheck.py" --roms "$ROMS" --rtb "$RTB" --hashes "$REPO/tools/leak-hashes.txt" \
    --allow "$REPO/tools/leak-allow.txt" --hosts "$EX/leak-hosts.txt" "$REPO" > "$EX/leakcheck-last.txt" \
    || { grep -v '^WARN' "$EX/leakcheck-last.txt"; echo "LEAK CHECK FAILED on the merged repo: nothing committed"; exit 1; }
fi
cd "$REPO"
[ -d .git ] || git init -q -b main
git add -A
if git diff --cached --quiet; then echo "no change since the last export"; exit 0; fi
git -c user.name="Ami-Go export" -c user.email="amigo-export@localhost" commit -q -m "Export from kit-intent $KREV ($(date -u +%F))

Toolkit source only: no ROM, RTB, game, disk image, frame or secret (tools/leakcheck.py: CLEAN).

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_016VZ5nMzoy9LTKmqvV6AY77"
git log --oneline | head -3
echo "size: $(du -sh --exclude=.git . | cut -f1) working tree, $(du -sh .git | cut -f1) .git, $(git ls-files | wc -l) files"
