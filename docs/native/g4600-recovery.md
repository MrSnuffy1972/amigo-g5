# Game 4600 (OperationFirestorm v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04600). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2021531 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 106 of 106 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5034 |
| code | 268 |
| data | 1949157 |
| unused | 67072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 2 |
| Disk.1:bootblock | 1 | 99 | 3 |

## Cases handled

- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: custom-format (non-DOS) disk, 1627 non-empty blocks: matched against track-loaded segments
- OperationFirestorm_v1.0/OperationFirestorm_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree OperationFirestorm_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 2-340 (bytes 1024-174592) outside the filesystem hold data, not code (opcode density 0.028, entropy 5.16)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 342-360 (bytes 175104-184832) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.79)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 366-431 (bytes 187392-221184) outside the filesystem hold data, not code (opcode density 0.056, entropy 5.85)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 440-484 (bytes 225280-248320) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.77)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 486-506 (bytes 248832-259584) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.08)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 510-617 (bytes 261120-316416) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.80)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 638-829 (bytes 326656-424960) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.21)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 832-835 (bytes 425984-428032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 840-843 (bytes 430080-432128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.63)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 846-848 (bytes 433152-434688) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.86)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 853-856 (bytes 436736-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 858-858 (bytes 439296-439808) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 860-860 (bytes 440320-440832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 862-862 (bytes 441344-441856) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 864-864 (bytes 442368-442880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 866-866 (bytes 443392-443904) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 868-868 (bytes 444416-444928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.63)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 881-919 (bytes 451072-471040) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.33)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 934-956 (bytes 478208-489984) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.17)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 958-963 (bytes 490496-493568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.52)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 966-971 (bytes 494592-497664) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.52)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 974-974 (bytes 498688-499200) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.32)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 981-984 (bytes 502272-504320) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 986-986 (bytes 504832-505344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 988-988 (bytes 505856-506368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 990-990 (bytes 506880-507392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 992-992 (bytes 507904-508416) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 994-994 (bytes 508928-509440) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 996-996 (bytes 509952-510464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 998-1083 (bytes 510976-555008) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.15)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1087-1091 (bytes 556544-559104) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.45)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1095-1099 (bytes 560640-563200) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.51)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1102-1103 (bytes 564224-565248) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.44)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1105-1106 (bytes 565760-566784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1108-1112 (bytes 567296-569856) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.90)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1114-1114 (bytes 570368-570880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1116-1116 (bytes 571392-571904) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1118-1118 (bytes 572416-572928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1120-1120 (bytes 573440-573952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1122-1122 (bytes 574464-574976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1124-1124 (bytes 575488-576000) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1126-1211 (bytes 576512-620544) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.44)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1213-1219 (bytes 621056-624640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.45)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1221-1227 (bytes 625152-628736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.45)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1229-1231 (bytes 629248-630784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.36)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1233-1240 (bytes 631296-635392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.50)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1242-1242 (bytes 635904-636416) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1244-1244 (bytes 636928-637440) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1246-1246 (bytes 637952-638464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1248-1248 (bytes 638976-639488) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1250-1250 (bytes 640000-640512) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1252-1252 (bytes 641024-641536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1254-1339 (bytes 642048-686080) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.00)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1341-1347 (bytes 686592-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.50)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1350-1355 (bytes 691200-694272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.56)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1358-1362 (bytes 695296-697856) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.17)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1364-1368 (bytes 698368-700928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.87)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1370-1370 (bytes 701440-701952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1372-1372 (bytes 702464-702976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1374-1374 (bytes 703488-704000) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1376-1376 (bytes 704512-705024) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1378-1378 (bytes 705536-706048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1380-1380 (bytes 706560-707072) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- OperationFirestorm_v1.0/OperationFirestorm_v1.0_hdf/OperationFirestorm/Disk.1: blocks 1382-1759 (bytes 707584-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.75)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.

### Disk.1:bootblock

- `initialize_and_process`: Initializes registers and memory pointers, then processes data through multiple conditional branches and subroutine calls based on bit operations and counter values.
- `load_and_shift`: Loads a long word from memory, extracts a byte, stores it in a global variable, and performs a right rotate operation on the loaded value.
- `rotate_and_count`: Performs bit rotation operations on a value while decrementing a counter, and calls another routine when the counter reaches zero.
