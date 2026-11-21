# Game 4367 (Mercenary3 v1.2 0356): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04367). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1203374 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13168 |
| code | 4 |
| data | 1185082 |
| unused | 5120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: raw disk image (485376 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356.hdf: hard-disk image (AmigaDOS OFS, 1312 blocks): its filesystem parses and all 11 files in it are byte-identical to the extracted tree Mercenary3_v1.2_0356_hdf, which is recovered file by file; no block outside the filesystem holds data
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 0-91 (bytes 0-23552) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.55)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 93-103 (bytes 23808-26624) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.96)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 105-118 (bytes 26880-30464) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.54)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 126-128 (bytes 32256-33024) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 133-169 (bytes 34048-43520) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.40)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 171-487 (bytes 43776-124928) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.10)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 493-1694 (bytes 126208-433920) outside the filesystem hold data, not code (opcode density 0.021, entropy 6.69)
- Mercenary3_v1.2_0356/Mercenary3_v1.2_0356_hdf/Mercenary3/Disk.1: chunks 1696-1895 (bytes 434176-485376) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.81)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
