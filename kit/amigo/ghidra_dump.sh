#!/bin/bash
# ghidra_dump.sh IMAGE BASE ENTRY [OUTDIR] - disassemble a flat 68000 memory image with Ghidra headless and write
# OUTDIR/functions.txt and OUTDIR/listing.txt (default OUTDIR /w/out/ghidra/<image name>).
# Written 13 Oct 2026: no Ami-Go session had ever run Ghidra; they decoded bytes by hand with struct.unpack
# (100-150 calls per session on 3D Pool G0). This makes Ghidra one command. Takes 1-3 minutes.
# Example: bash /w/kit/amigo/ghidra_dump.sh /w/out/image.bin 0x20000 0x20000
set -eu
IMG=$1; BASE=$2; ENTRY=$3
NAME=$(basename "$IMG" .bin)
OUT=${4:-/w/out/ghidra/$NAME}
mkdir -p "$OUT" /w/out/ghidra/proj
timeout 900 analyzeHeadless /w/out/ghidra/proj "$NAME" -import "$IMG" -overwrite \
  -loader BinaryLoader -loader-baseAddr "$BASE" -processor 68000:BE:32:default \
  -scriptPath /w/kit/amigo/ghidra -postScript AmigoExport.java "$OUT" "$ENTRY" -deleteProject \
  > "$OUT/ghidra.log" 2>&1 || { echo "Ghidra failed; last lines of $OUT/ghidra.log:"; tail -15 "$OUT/ghidra.log"; exit 1; }
chmod -R a+rX "$OUT"   # Ghidra writes 0640 files as root; read_file runs on the host and could not read them
grep "AmigoExport:" "$OUT/ghidra.log" | tail -1
echo "Read $OUT/functions.txt first (one line per function), then the parts of $OUT/listing.txt you need."
