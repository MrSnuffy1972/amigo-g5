# Game 3222 (CountDuckula v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03222). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (748408 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11194 |
| code | 4 |
| data | 728506 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: raw disk image (326656 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- CountDuckula_v1.1/CountDuckula_v1.1.hdf: hard-disk image (AmigaDOS OFS, 800 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree CountDuckula_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: chunks 0-246 (bytes 0-63232) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.67)
- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: chunks 258-285 (bytes 66048-73216) outside the filesystem hold data, not code (opcode density 0.018, entropy 4.12)
- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: chunks 287-320 (bytes 73472-82176) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.30)
- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: chunks 323-782 (bytes 82688-200448) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.16)
- CountDuckula_v1.1/CountDuckula_v1.1_hdf/CountDuckula/disk.1: chunks 784-1256 (bytes 200704-321792) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.52)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
