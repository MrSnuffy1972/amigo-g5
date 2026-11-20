# Game 3748 (FutureBikeSimulator v2.0 2149): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03748). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2197912 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10858 |
| code | 4 |
| data | 2171688 |
| padding | 2 |
| unused | 15360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |

## Cases handled

- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149.hdf: hard-disk image (AmigaDOS OFS, 2368 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree FutureBikeSimulator_v2.0_2149_hdf, which is recovered file by file; no block outside the filesystem holds data
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 0-2145 (bytes 0-549376) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.33)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 2151-2613 (bytes 550656-669184) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.30)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 2616-2658 (bytes 669696-680704) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.62)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 2664-2851 (bytes 681984-730112) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.66)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 2856-2997 (bytes 731136-767488) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.45)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 3000-3518 (bytes 768000-900864) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.46)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 3528-3590 (bytes 903168-919296) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.37)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 3600-3657 (bytes 921600-936448) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.39)
- FutureBikeSimulator_v2.0_2149/FutureBikeSimulator_v2.0_2149_hdf/FutureBikeSimulator/data/Disk.1: chunks 3672-3781 (bytes 940032-968192) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.78)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
