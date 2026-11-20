# Game 3591 (F1Tornado v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03591). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (701945 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11423 |
| code | 4 |
| data | 670038 |
| unused | 20480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: raw disk image (295936 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- F1Tornado_v1.0/F1Tornado_v1.0.hdf: hard-disk image (AmigaDOS OFS, 768 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree F1Tornado_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 0-107 (bytes 0-27648) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.92)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 109-109 (bytes 27904-28160) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.68)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 111-111 (bytes 28416-28672) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 113-164 (bytes 28928-42240) outside the filesystem hold data, not code (opcode density 0.035, entropy 6.76)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 171-432 (bytes 43776-110848) outside the filesystem hold data, not code (opcode density 0.049, entropy 6.01)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 434-443 (bytes 111104-113664) outside the filesystem hold data, not code (opcode density 0.027, entropy 3.83)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 450-453 (bytes 115200-116224) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.51)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 456-664 (bytes 116736-170240) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.00)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 709-1039 (bytes 181504-266240) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.75)
- F1Tornado_v1.0/F1Tornado_v1.0_hdf/F1Tornado/Disk.1: chunks 1058-1155 (bytes 270848-295936) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.99)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
