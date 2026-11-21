# Game 4481 (Munsters v1.1 1265): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04481). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1309380 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 33792 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2 of 2 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12419 |
| code | 4 |
| data | 1286963 |
| padding | 10 |
| unused | 9984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1 bytes 539904-543232 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 543488-573952 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: raw disk image (573952 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Munsters_v1.1_1265/Munsters_v1.1_1265.hdf: hard-disk image (AmigaDOS OFS, 1408 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Munsters_v1.1_1265_hdf, which is recovered file by file; no block outside the filesystem holds data
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: bytes 539904-543232 look like code no run loaded: recovered position-independently (load address unknown)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: bytes 543488-573952 look like code no run loaded: recovered position-independently (load address unknown)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: 2 logged loads cover 33792 bytes of the image
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 0-200 (bytes 0-51456) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.62)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 202-577 (bytes 51712-147968) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.67)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 579-580 (bytes 148224-148736) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.32)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 583-584 (bytes 149248-149760) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.44)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 586-587 (bytes 150016-150528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.94)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 589-635 (bytes 150784-162816) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.96)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 637-706 (bytes 163072-180992) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.60)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 708-709 (bytes 181248-181760) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.29)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 711-724 (bytes 182016-185600) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.82)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 731-874 (bytes 187136-224000) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.33)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 876-876 (bytes 224256-224512) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 878-928 (bytes 224768-237824) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.98)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 930-930 (bytes 238080-238336) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 932-940 (bytes 238592-240896) outside the filesystem hold data, not code (opcode density 0.010, entropy 2.41)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 942-942 (bytes 241152-241408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 944-944 (bytes 241664-241920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 946-1693 (bytes 242176-433664) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.35)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1696-1720 (bytes 434176-440576) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.22)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1723-1723 (bytes 441088-441344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1725-1797 (bytes 441600-460288) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.24)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1799-1802 (bytes 460544-461568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.92)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1804-1805 (bytes 461824-462336) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.66)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1810-1950 (bytes 463360-499456) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.04)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1954-1982 (bytes 500224-507648) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.67)
- Munsters_v1.1_1265/Munsters_v1.1_1265_hdf/Munsters/Disk.1: chunks 1984-2107 (bytes 507904-539648) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.92)
- 2 never-loaded region(s), 33792 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
