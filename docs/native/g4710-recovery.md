# Game 4710 (PinkPanther v1.0 0236): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04710). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1192280 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 3072 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7 of 7 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12673 |
| code | 22 |
| data | 1149121 |
| unused | 30464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 2 |
| Disk.1 bytes 0-512 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 318720-319488 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 319744-321536 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: raw disk image (522240 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236.hdf: hard-disk image (AmigaDOS OFS, 1280 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree PinkPanther_v1.0_0236_hdf, which is recovered file by file; no block outside the filesystem holds data
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: bytes 0-512 look like code no run loaded: recovered position-independently (load address unknown)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: bytes 318720-319488 look like code no run loaded: recovered position-independently (load address unknown)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: bytes 319744-321536 look like code no run loaded: recovered position-independently (load address unknown)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: 3 logged loads cover 3072 bytes of the image
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 4-51 (bytes 1024-13312) outside the filesystem hold data, not code (opcode density 0.052, entropy 5.71)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 54-430 (bytes 13824-110336) outside the filesystem hold data, not code (opcode density 0.028, entropy 5.63)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 440-590 (bytes 112640-151296) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.99)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 600-750 (bytes 153600-192256) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.58)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 758-907 (bytes 194048-232448) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.09)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 917-1047 (bytes 234752-268288) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.66)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1060-1075 (bytes 271360-275456) outside the filesystem hold data, not code (opcode density 0.030, entropy 5.23)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1080-1084 (bytes 276480-277760) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.34)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1087-1088 (bytes 278272-278784) outside the filesystem hold data, not code (opcode density 0.020, entropy 3.01)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1091-1093 (bytes 279296-280064) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.08)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1095-1097 (bytes 280320-281088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.97)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1099-1103 (bytes 281344-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.82)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1106-1120 (bytes 283136-286976) outside the filesystem hold data, not code (opcode density 0.042, entropy 3.68)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1129-1243 (bytes 289024-318464) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.24)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1260-1260 (bytes 322560-322816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1273-1372 (bytes 325888-351488) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.70)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1378-1406 (bytes 352768-360192) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.31)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1416-1613 (bytes 362496-413184) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.81)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1620-1658 (bytes 414720-424704) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.71)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1660-1859 (bytes 424960-476160) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.19)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1861-1867 (bytes 476416-478208) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.25)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1870-1872 (bytes 478720-479488) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- PinkPanther_v1.0_0236/PinkPanther_v1.0_0236_hdf/PinkPanther/Disk.1: chunks 1880-2039 (bytes 481280-522240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.96)
- 3 never-loaded region(s), 3072 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
