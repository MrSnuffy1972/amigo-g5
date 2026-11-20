# Game 2831 (BatmanCapedCrusader v1.1 0050): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02831). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1869912 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14029 |
| code | 4 |
| data | 1855367 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- BatmanCapedCrusader_v1.1_0050/BatmanCapedCrusader_v1.1_0050_hdf/BatmanCapedCrusader/Disk.1: raw disk image (809984 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BatmanCapedCrusader_v1.1_0050/BatmanCapedCrusader_v1.1_0050.hdf: hard-disk image (AmigaDOS OFS, 2016 blocks): its filesystem parses and all 11 files in it are byte-identical to the extracted tree BatmanCapedCrusader_v1.1_0050_hdf, which is recovered file by file; no block outside the filesystem holds data
- BatmanCapedCrusader_v1.1_0050/BatmanCapedCrusader_v1.1_0050_hdf/BatmanCapedCrusader/Disk.1: chunks 0-1 (bytes 0-512) outside the filesystem hold data, not code (opcode density 0.027, entropy 4.52)
- BatmanCapedCrusader_v1.1_0050/BatmanCapedCrusader_v1.1_0050_hdf/BatmanCapedCrusader/Disk.1: chunks 4-3163 (bytes 1024-809984) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.39)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
