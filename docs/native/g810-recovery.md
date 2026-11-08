# Game 810 (CannonFodder2 v1.1 0104): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00810). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3675716 top-level bytes in 546 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 243 of 243 named (0 author symbols), 243 of 243 described (16 by rule, 227 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1539 of 1539 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 215766 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4703 of 4703 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 64472 |
| code | 18648 |
| data | 3592328 |
| padding | 268 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 144 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3483 | 164 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 429 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 636 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| CF2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| FONT.RAW#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104.hdf: hard-disk image (AmigaDOS OFS, 4192 blocks): its filesystem parses and all 316 files in it are byte-identical to the extracted tree CannonFodder2_v1.1_0104_hdf, which is recovered file by file; no block outside the filesystem holds data
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2675 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20555 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/3.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (817 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/4.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11505 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/5.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12228 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/6.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11160 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/7.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13089 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/8.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9783 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/9.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (942 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/A.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11726 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/B.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14149 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/C.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4086 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/CF2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (72121 -> 174774 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/CFTITLE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10199 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/D.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6913 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22133 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (576 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21226 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (153 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30737 -> 36694 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1145 -> 3145 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (111 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (31882 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19858 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (386 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13121 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DESSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/DISK2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11763 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/FONT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4371 -> 40992 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/HILLS.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15067 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22872 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (542 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13511 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (118 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (38322 -> 44704 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1203 -> 3207 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (79 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICECOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25932 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICEPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8561 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICESUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (278 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICESUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9291 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICESUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (89 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/ICESUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (51 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22781 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (877 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15248 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (203 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (34296 -> 41938 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1114 -> 3095 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25703 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20493 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11497 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/INTSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (60 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JON.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (94572 -> 107786 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JON.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2563 -> 7935 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (24375 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20851 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (39226 -> 47228 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1156 -> 3153 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (68 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28698 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20295 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12327 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/JUNSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (93 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/KILLER.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (273625 -> 296072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/KILLER.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1227 -> 2989 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM1.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (256 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM1.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (91 -> 100 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM10.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (875 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM10.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (193 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM11.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (868 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM11.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (195 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM12.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (829 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM12.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM13.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (778 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM13.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (180 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM14.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1459 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM14.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (211 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM15.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1274 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM15.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (196 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM16.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (382 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM16.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM17.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1127 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM18.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (386 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM18.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (170 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM19.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1137 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM19.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM2.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (327 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM2.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (120 -> 170 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM20.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (738 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM20.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (202 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM21.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (931 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM21.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM22.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (526 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM22.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM23.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (628 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM23.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (214 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM24.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (818 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM24.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (220 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM25.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (406 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM25.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (183 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM26.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (978 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM26.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (238 -> 380 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM27.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1672 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM27.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (234 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM28.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (566 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM28.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (188 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM29.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1081 -> 2984 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM29.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (198 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM3.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (592 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM3.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (184 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM30.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (352 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM30.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (121 -> 160 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM31.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1607 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM31.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (180 -> 250 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM32.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1116 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM32.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM33.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1515 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM33.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (213 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM34.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (703 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM34.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (210 -> 310 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM35.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1363 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM35.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (192 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM36.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (336 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM36.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 310 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM37.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1160 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM37.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (209 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM38.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (801 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM38.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (197 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM39.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1498 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM39.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM4.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (538 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM4.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (166 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM40.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1559 -> 5454 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM40.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM41.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (565 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM41.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (174 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM42.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1471 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM42.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM43.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (492 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM43.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (201 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM44.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1078 -> 4048 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM44.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (215 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM45.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1427 -> 4770 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM45.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM46.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1360 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM46.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM48.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (961 -> 4428 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM48.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (217 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM49.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (992 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM49.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (230 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM5.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (514 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM5.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (177 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM50.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1079 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM50.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (191 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM51.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1130 -> 3516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM51.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (192 -> 220 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM52.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (855 -> 3516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM52.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (187 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM53.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (822 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM53.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (215 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM54.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (562 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM54.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (216 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM55.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1303 -> 4516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM55.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (234 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM56.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (981 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM56.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (177 -> 230 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM57.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (977 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM57.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (225 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM58.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1345 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM58.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (229 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM59.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (281 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM59.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (176 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM6.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (373 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM6.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (149 -> 200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM61.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (467 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM61.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (109 -> 220 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM62.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (310 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM62.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (120 -> 200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM63.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (420 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM63.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (151 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM64.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (672 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM64.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM66.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (933 -> 2718 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM66.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 270 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM67.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (181 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM67.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (171 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM69.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (929 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM69.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (191 -> 230 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM7.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (355 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM7.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM70.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1145 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM70.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (204 -> 250 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM72.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (188 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM72.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (55 -> 70 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM8.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (747 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM8.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (200 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM9.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1207 -> 4464 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MAPM9.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22721 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (448 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12619 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (166 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35336 -> 40872 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1139 -> 3011 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (83 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30304 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORPHBIG.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10666 -> 67200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15367 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (185 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8971 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/MORSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (86 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/PSTUFF.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11229 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/RANKFONT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15102 -> 51200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/SENSPROD.PAL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (93 -> 96 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/SENSPROD.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7533 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/STARTUP.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8348 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/VIRGPRES.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5374 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/WON.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8328 -> 51400 bytes)
- runtime segment an address: 651 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 318 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 487 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/CF2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CannonFodder2_v1.1_0104/CannonFodder2_v1.1_0104_hdf/CannonFodder2/data/FONT.RAW#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 215766 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `process_and_clear_buffers`: Stores a pointer from a global variable into A0, calls three routines via a table, and if a specific value is returned, clears two memory buffers using loops and jumps to a routine at an offset from a global variable.
- `clamp_or_fetch_value`: Compares a value in D0 against a threshold; if it is below the threshold, it returns a value from a structure pointed to by A0, otherwise it returns the original value in D0.
- `validate_and_format_disk_strings`: Checks a value in d0 against three constants; if d0 is zero, it compares four strings against a list of expected disk identifiers, and if all match, it concatenates a path string and calls another routine.
- `compare_and_copy_strings`: Compares two strings; if they match, it performs a series of memory operations involving a table and a loop to copy or modify data, otherwise it returns a specific error value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_reset_state`: Initializes custom hardware registers, sets up DMA and interrupt configurations, and clears several global variables and tables.
- `initialize_system_state_and_resources`: Clears several word-sized fields in a global structure, sets two memory pointers, and calls several subroutines to reset the game state.
- `reset_game_state`: Clears a 16-bit word in a global variable, calls several subroutines to reset state, and performs a series of conditional checks and updates to various global variables before looping back to the start.
- `check_global_conditions`: Checks multiple global variables for specific values.
- `execute_infinite_loop`: Enters an infinite loop that prevents further execution.
- `update_global_state_vars`: Modifies several global variables based on their current values and conditions.
- `adjust_global_counters`: Checks if a global variable is zero and another value is above a threshold.
- `calculate_coordinate_offsets`: Reads several global variables, performs arithmetic operations including additions with specific constants, and stores the results back into a series of global variables.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_reset_memory`: Copies a block of data from one fixed memory address to another.
- `clear_global_flags`: Checks two global variables; if either is non-zero, it sets them to zero.
- `validate_and_reset_state`: Checks multiple global variables for zero or specific values.
- `adjust_signed_value`: Performs arithmetic operations on a global variable, including doubling, sign checks, and conditional assignments based on magnitude comparisons.
- `compute_shifted_mask`: Calculates a new value for a global variable by shifting, masking, and applying conditional logic based on the current value's magnitude and sign.
- `compute_table_offset`: Calculates a scaled index from a global variable, applying sign adjustments and bounds checking before storing the result.
- `calculate_table_offset`: Computes an index from a global variable, handles sign adjustments, and stores a scaled offset into a memory location pointed to by a register.
- `swap_word_blocks`: Iteratively swaps words between two memory locations pointed to by registers, decrementing a counter until the loop completes.
- `iterate_command_table`: Iterates through a table of command structures, checking a byte offset to decide between two recursive calls, and terminates when a negative value is encountered.
- `load_map_data_2`: Sets a pointer in a0 to a memory address, sets a1 to a specific address, sets d1 to negative one, and jumps to a subroutine.
- `copy_string_until_mismatch`: Compares two strings byte-by-byte; if they match up to a null terminator, it copies the first string to the second location, otherwise it stops at the first mismatch.
- `initialize_game_resources`: Sets global variables to specific addresses and calls multiple helper routines to load map data, process strings, and clear memory regions, likely preparing the game state.
- `initialize_string_buffer`: Calls several helper routines and then sets a global variable to zero, likely preparing a buffer for string operations.
- `load_killer_assets`: References text strings for 'KILLER.INS' and 'KILLER.SNG' and calls a command dispatch routine, suggesting it loads or initializes specific game assets.
- `copy_data_block`: Copies a fixed number of words from a specific source address in memory to a destination address, decrementing a counter until the transfer is complete.
- `copy_data_block_2`: Copies a fixed number of words from a specific source address to a global variable's address, decrementing a counter until the transfer is complete.
- `clear_memory_block_2`: Sets a fixed number of words to zero starting from a global variable's address, decrementing a counter until the block is cleared.
- `clear_memory_block_3`: Sets a larger fixed number of words to zero starting from a global variable's address, decrementing a counter until the block is cleared.
- `copy_80_word_table`: Copies 161 words from a fixed source address to a global variable buffer, decrementing a counter until the transfer is complete.
- `clear_global_array`: Iterates through a global array of word-sized entries, setting each element to zero.
- `load_map_data`: Copies data from a fixed memory address into a global buffer until a terminator byte is found, then copies the global buffer to another location.
- `loop_pixel_processor`: Iterates a counter from 73 with a step size of 12, calling a pixel data processing routine in each iteration until the counter reaches 230.
- `reset_state_counters`: Checks global state variables and resets them to specific values, then calls routines to process word arrays and temporarily zero out a global variable.
- `zero_global_fields`: Sets several consecutive word-sized fields within a global variable structure to zero.
- `reset_counter_table`: Checks a global counter; if it is zero, it clears a series of bytes in a table at a fixed address and manipulates other global variables and a lookup table.
- `increment_table_bytes`: Iterates through a table of structures at a fixed address.
- `load_table_values`: Loads a word from a table using an offset from a global variable, adds 24 to it, and moves the results into registers d6 and d7 before branching.
- `calculate_position_coords`: Computes coordinate values based on global variables and input parameters, calling helper routines to adjust values.
- `branch_forward`: Performs a short branch to another routine.
- `adjust_global_value`: Modifies a global variable by subtracting a constant and checking bounds.
- `toggle_global_bit`: Toggles a single bit in a global variable and calls another routine.
- `handle_command_dispatch`: Saves registers to a global location, sets d0 to 8, calls a subroutine, stores the result in a global variable, and then either loops through a table using a subroutine or performs a final subroutine call.
- `decode_compressed_data`: Saves registers to a global location, calls a subroutine, stores the result in a global variable, and then either loops through a table using a subroutine or performs a final subroutine call.
- `toggle_global_flag`: Checks a global variable against a specific constant value.
- `find_pattern_index`: Searches through a global array for a sequence of three bytes that match specific values stored in global memory.
- `parse_byte_table`: Scans a byte array at a fixed address, extracting values where the upper bits are set.
- `update_table_parameters`: Checks three global variables for zero and a positive sign, then reads two values from a table using an index derived from a global variable, shifts them right by four bits, and stores them in new global locations.
- `check_and_store_byte`: Checks if a global variable is zero and if a register value matches it.
- `adjust_byte_counter`: Modifies a byte in memory by incrementing or decrementing it based on comparisons with a global variable.
- `process_conditional_logic`: Stores a word in a global variable, checks if a global variable is zero and if a masked register value equals 14, then calls another routine if conditions are met.
- `execute_indirect_jump`: Loads a word from a global variable, adds four to it, and stores the result back in the global variable before performing an indirect jump to the address found at the original offset.
- `set_constant_values`: Stores the values 100 and 40 into two specific global variables and returns.
- `check_death_condition`: Compares a global variable to a constant, increments and checks another global variable, and if conditions are met, clears two variables, calls three routines, and sets a final global variable to five before returning.
- `copy_data_blocks_loop`: Sets global variables to specific values and calls helper routines in a sequence, likely managing data copying or memory initialization through repeated calls.
- `split_word_into_two`: If a global variable is 1, it loads a word from a table using an offset, splits it by logical shifting, and stores the two resulting parts in adjacent table entries.
- `move_and_split_word`: Copies a word from one location to another and clears the source, then optionally splits a word into two parts and stores them in new locations based on a global variable.
- `move_and_clear_word`: Copies a word-sized value from one memory offset to another within a base address, then clears the source location to zero.
- `setup_entity_structure`: Initializes a structure at a global pointer address with specific constant values, offsets, and a function pointer, then calls a helper routine if the pointer is valid.
- `call_update_routine`: Calls a subroutine and then either branches if the result is positive or returns.
- `update_entity_count`: Calls a subroutine, and if the result is not zero, stores a value from a memory location into a global variable, increments a global variable, and calls another subroutine.
- `set_flag_and_bytes`: Checks a word-sized flag in memory.
- `reset_pixel_shift_2`: Calls a subroutine, and if the result is not zero, checks a global variable and if it is not negative, sets it to negative 65535 and calls another subroutine.
- `reset_pixel_shift`: Checks if a global variable is non-zero; if so, it resets the variable to zero, sets it to one, and calls two other routines before returning.
- `call_subroutines`: Calls two subroutines in sequence and then returns.
- `initialize_entity_header`: Checks if a global pointer is non-zero, then writes a longword, two words, another word calculated from a table, and a function pointer into a structure at that address, before calling another routine.
- `call_and_branch`: Calls a subroutine and then branches to another routine if the result is not positive.
- `update_entity_flags`: Checks a global state and zero flag; if conditions are met, it increments a counter, resets it if too high, and sets specific byte values in memory tables.
- `check_and_set_global`: Calls a subroutine, and if the result is zero, checks a longword at a global address; if that value is negative, it sets it to negative 65535 and calls another subroutine.
- `check_global_state`: Reads a global variable and compares it against specific integer values, clearing it and calling another routine if it is non-zero.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `search_table_index`: Loads a pointer from a table, clears register d0, and checks the byte at that pointer; if the byte is non-zero, it increments d0 and checks if it is less than three, otherwise it branches to another routine.
- `conditional_pixel_shift`: Checks if a value is non-negative; if so, it calculates an offset, loads a word from a table, and calls a routine that performs bitwise shifts and logical operations on pixel data blocks.
- `return_pixel_shift_result`: Checks if a global variable is non-negative; if so, it calls a pixel data shifting routine and returns its result, otherwise returning the global variable itself.
- `setup_copy_parameters`: Loads a destination address into a0, loads a source address into a1, and calls a subroutine.
- `initialize_pointer_structure`: Initializes a structure at a location pointed to by a register with a longword, a word, and several other values, and conditionally sets a pointer in the structure based on a comparison.
- `branch_and_return`: Calls a subroutine and then returns if the result of a comparison is not negative.
- `toggle_global_flags_and_call_helpers`: Calls another routine, and if the result is zero, it sets two global variables to one, clears two other global variables, loads a word from a table into d0, and calls three distinct routines before setting a final global variable to one and clearing another.
- `iterate_table_bits`: Calls a subroutine, and if the result is non-zero, iterates through a table of four entries to check if a specific bit is set at each offset.
- `clear_bit_in_table_2`: Iterates through a table of 8 entries, checking specific bit flags and values before clearing several bytes and updating an offset in the table.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `count_active_entities`: Iterates through a table of 8 entries, counting how many meet specific criteria involving pointer validity and bit flags.
- `clear_bit_in_table`: Iterates through a table of 8 entries.
- `setup_entity_with_offset`: Iterates backwards through a global array to find a null terminator, then initializes a structure at a global pointer with specific values and a calculated offset.
- `toggle_tile_state_2`: Calls two different subroutines and branches if the result is positive.
- `toggle_tile_state`: Calculates an index from global variables, iterates through a table of structures to find a match, and toggles a byte flag within the matched structure.
- `copy_word_list_loop`: Copies a sequence of words and a longword from a source list in a1 to a destination in a0, looping until a negative value is encountered in the source.
- `store_pointer_constant`: Stores the value negative one as a longword into the location pointed to by a register and then updates a global variable with that pointer.
- `execute_entity_updates`: Writes a word from d1 to a table using d0 as an offset, calls a subroutine, and then conditionally calls a sequence of eleven other subroutines if a byte in a table is non-zero.
- `update_state_flag`: Checks a word-sized flag in memory.
- `update_entity_state`: Checks two word-sized flags in memory, and if both are zero, uses a value from a table to index into three other tables to conditionally call another routine or branch to a third routine.
- `process_state_arrays`: Iterates through two byte arrays and a word array, toggling values between 1 and 3 based on null terminators and global state.
- `check_global_state_value`: Checks a global variable against specific values and iterates through a table to find a matching entry.
- `check_global_state_2`: Clears two global variables, loads a word from a table into a global variable, and then enters a loop that checks a byte in another table; if the byte is non-zero, it clears a global variable, compares it to a value from a table, and conditionally calls two other routines while incrementing two global variables.
- `move_word_to_d0`: Moves a word from a memory location into register d0.
- `count_active_entities_2`: Loads a value from a table into d1, performs a bitwise shift, and loads a new value from a table; it calls two routines, then checks a byte in another table and a global variable, and if certain conditions are met, it calculates an offset and calls another routine.
- `load_word_to_d0`: Loads a 16-bit word from a memory location into data register 0.
- `check_entity_state_flags`: Loads a value from a table into d1, performs a bitwise shift, and loads a new value from a table; it calls two routines, then checks a byte in another table, a global variable, and a third global variable, and if all conditions are met, it calculates an offset and calls another routine.
- `update_global_variables`: Updates several global variables with values from registers and performs a comparison to set one of the globals to a specific value if a condition is met.
- `call_pixel_processor`: Loads a word from a table using an offset, clears register d2, and calls a routine to process pixel data.
- `check_and_shift_pixels`: The routine initializes a return value, checks a global variable condition, and conditionally calls functions to initialize state and shift pixel data blocks.
- `check_global_state_flag`: Checks a value derived from a global variable; if non-zero, it calls another routine and returns its result, otherwise returns zero.
- `call_check_global_state_value`: Calls check_global_state_value and then continues execution into the next routine.
- `shift_pixel_blocks`: The routine calls a function to perform bitwise shifts and logical operations on blocks of pixel data.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_pixel_tile_data`: Loads a byte from a table, and if it is non-negative, it enters a loop to call another routine and update a global variable; it then checks a second table and conditionally calls two routines, performs a multiplication, and calls a third routine based on several table-based comparisons and bit tests.
- `initialize_and_call_functions`: Sets a memory location to a specific value, updates global variables, and calls several functions in sequence.
- `write_to_memory_locations`: Writes values from registers to multiple memory locations in a loop, decrementing a counter until it reaches a specific value.
- `load_address_table`: Loads six longword addresses from memory into registers a0 through a5 and branches to another routine.
- `write_to_memory_locations_offset`: Writes values from registers to multiple memory locations in a loop, incrementing an offset and decrementing a counter until it reaches a specific value.
- `loop_copy_word_blocks`: Iterates through a global variable pointer while the value is non-negative, calling a routine to copy a block of 242 words.
- `load_and_call_copy`: Loads a longword into a1 from a global address and calls another routine before returning.
- `store_pointer_to_global`: Stores the value from register a1 into a global variable.
- `copy_data_to_memory`: Copies data from a source pointer to multiple memory locations, including a specific hardware register, in a loop until a counter reaches a specific value.
- `apply_bit_masks`: Performs a series of bitwise OR operations on register d0 using several constant values.
- `apply_bitwise_or_constants`: Performs a series of bitwise OR operations on register d0 using six different immediate byte constants.
- `copy_word_block_242`: Copies 242 words from a source pointer in a0 to a destination pointer in a1 using a loop.
- `extract_table_values`: Calculates an offset using d0, loads four words from a table at a0 into d0, d1, d4, and d5, then branches to another routine.
- `initialize_entry_table`: Sets two global words to -1 and initializes a table of 7 entries at a fixed address with -1, 0, and 3 values.
- `sort_and_compact_table`: Compacts a table by shifting valid entries down and then sorts the entries based on specific field comparisons.
- `manage_table_entries_loop`: Iterates through a table of entries, copying data between memory locations and checking for empty slots marked by -1.
- `allocate_table_entry`: Searches a table for an empty entry marked by -1, assigns it a new ID from a global counter, and initializes its fields.
- `copy_buffer_to_global`: Copies 1831 bytes from a fixed memory address in a1 to a global variable buffer in a0.
- `copy_global_to_buffer`: Copies 1831 bytes from a global variable buffer in a0 to a fixed memory address in a1.
- `process_tile_string_data`: The routine iterates through a string buffer, skipping spaces and null terminators, and conditionally calls a helper routine for non-space characters based on a global counter.
- `store_register_to_memory`: Stores the contents of register a5 into a memory location at an offset of 28672 from the address in a4.
- `shift_pixel_data_blocks`: Performs bitwise shifts and logical operations on blocks of pixel data to adjust alignment or blend colors.
- `shift_pixel_data`: Performs bitwise shifts and logical operations on blocks of pixel data to adjust alignment or blend colors.
- `check_global_value`: Calls another routine, compares a global variable to decimal thirty-two, and if equal, calls another routine; it then checks if the global variable is decimal sixteen and, if so, compares it against decimal one-hundred-twelve and decimal eighty, potentially inverting a global variable or setting another to decimal sixty-five-thousand-five-hundred-thirty-five.
- `clear_ciaa_interrupts`: The routine writes a value to the CIAA Interrupt Control Register to acknowledge or clear pending interrupts.
- `update_interrupt_state`: Saves registers, writes 4 to the INTREQ hardware register, calls a subroutine, restores registers, and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_byte_table_entry`: The routine reads a byte from a table using an index stored in a global variable.
- `clear_memory_regions`: Conditionally zeroes out two distinct blocks of memory by iterating through word-sized pointers if specific helper routines return non-zero values.
- `clear_other_regions`: Conditionally zeroes out two different memory blocks by iterating through word-sized pointers based on helper routine returns.
- `process_map_data`: Converts a string to lowercase, then iterates through a table of structures, copying fields, checking for specific byte values to increment counters, and setting flags based on content.
- `convert_string_to_uppercase`: The routine iterates through a null-terminated string pointed to by a register.
- `copy_string_with_lowercase`: The routine copies characters from a source string to a fixed memory location, converting uppercase letters to lowercase.
- `dispatch_command_by_type`: Checks the value of d0 against several constants and, depending on the match, calls different subroutines or performs string copying and decompression operations.
- `dispatch_conditional_logic`: Performs several conditional branches and calls to a routine with various parameters and addresses, eventually storing a result in a global pointer.
- `conditional_call_empty_routine`: Checks if a register value is not -1.
- `copy_and_convert_strings`: The routine copies null-terminated strings from source pointers to fixed memory locations, converting characters to lowercase or uppercase as specified by the called helpers.
- `copy_string_with_lowercase_2`: Copies characters from a source string to a fixed memory location, converting uppercase letters to lowercase.
- `call_subroutine`: Calls another routine and then returns.
- `process_string_buffer`: Checks a word at a global address and either jumps to a routine or returns immediately.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `calculate_bit_manipulated_difference`: Adds a byte to a long integer, performs a series of bitwise shifts, swaps, and rotations on values loaded from a table via a pointer in a3, and finally subtracts the modified value from a constant.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
- `tiny_ori`: Small routine of 1 instruction that merges 70 into -(a0).
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR operation on a register with zero.
- `reset_potgo_and_flags`: Sets the POTGO hardware register to all ones, calls another routine, sets two global variables to specific constant values, and clears two other global variables before returning.
- `process_mouse_movement`: Reads mouse port movement data from hardware registers and updates global variables representing position deltas.
- `check_joystick_button_state`: The routine reads hardware registers for joystick and mouse button states.
- `clear_color_registers`: The routine writes zero to a series of memory-mapped hardware registers, specifically clearing color registers and other display-related control registers in a loop.
- `update_state_counter`: Adjusts a global variable based on conditional checks and calls other routines to modify state values.
- `update_state_counter_2`: Checks four 16-bit global variables and, depending on their signed values, modifies one of them to either a specific constant or a value derived from a subtraction and addition.
- `process_word_array_2`: Loads two memory addresses into registers and a constant into a third, then branches to another routine to process data.
- `temporarily_zero_global`: Saves the current value of a global variable, sets it to zero, calls another routine, and then restores the original value of the global variable.
- `process_word_array`: Iterates through an array of words, performing bitwise operations and adjustments based on a global variable.
- `check_and_reset_flag`: Checks a global variable and, if it is non-zero, clears it and calls a subroutine before returning.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `configure_hardware_registers`: Sets specific hardware registers including BPLCON0, CIAA_ICR, DMACON, INTENA, and VPOSR to initialize or configure system settings.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_with_ff`: Fills a block of memory starting at a specific address with the value 255.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_interleaved_memory_blocks`: Copies words from a source address to a destination address using a loop, while simultaneously performing non-sequential reads from offset locations within the source block to a third destination.
- `compute_bitwise_operations`: Performs a series of bitwise operations and arithmetic on a global variable, adjusting its value based on specific conditions.
- `calculate_offset_value`: Checks a 16-bit global variable and, if it is zero, calculates a value by fetching a byte from a table using an offset, performing arithmetic shifts and additions, and then branches.
- `write_constant_to_memory`: Writes the value 188 to a global variable and returns.
- `initialize_memory_blocks`: Initializes memory blocks by setting specific values and performing bitwise operations on a global variable.
- `initialize_game_state`: Performs extensive initialization by setting global variables, calling multiple setup routines, and copying word-sized data blocks between fixed memory addresses.
- `manage_global_state_loop`: Stores a word from d0 into a global variable, calls three routines, and then enters a loop that calls a routine and decrements the value from the global variable until it reaches zero.
- `copy_memory_blocks`: Calls three different routines, updates two global variables, and then uses a loop to copy words between four memory locations, preserving one global variable's value during the process.
- `process_text_file`: Calls several routines to initialize memory, then iterates through a table of pointers; it checks the value at each pointer, conditionally copies a byte to a global variable, and performs loop-based processing with various routine calls and counter adjustments.
- `copy_word_data`: Copies word-sized data from one memory location to another, iterating through a fixed number of elements.
- `copy_word_data_with_offset`: Copies 16 words from a source table to a destination buffer, while simultaneously copying 16 words from a second source table starting at an offset of 16 words into the first table to a third destination.
- `clear_byte_array_2`: Sets eight consecutive byte-sized global variables to zero.
- `decrement_global_counter_2`: Decrements a global variable twice if it is non-zero, then calls another routine twice.
- `decrement_global_counter`: Decrements two consecutive 16-bit global variables if they are non-zero, then calls another routine twice before returning.
- `clear_byte_array`: Sets seven consecutive 16-bit global variables to zero.
- `swap_memory_blocks`: Rotates data between three memory regions by copying words from one pointer to another in a loop, shifting values through intermediate buffers.
- `copy_words_to_buffer`: Copies a sequence of words from a source pointer into a fixed memory address, decrementing a counter until completion.
- `copy_words_loop`: Copies words from a source pointer to a destination pointer at a fixed address using a loop controlled by a counter.
- `restore_second_block`: Copies words from a fixed memory address into another fixed memory address, restoring data for a specific block.
- `update_scroll_offset`: Increments a pointer with wraparound logic, calculates a new value using indexed memory and global variables, and updates a global state variable with the result.
- `initialize_status_flags`: Sets a series of global variables to specific constant values, mostly zero or -1, with one set to a small positive integer.
- `initialize_memory_blocks_2`: Clears a 16-bit global variable, calls another routine, and then sets that same global variable to a constant value.
- `clear_struct_fields`: Calculates an offset from a global variable and sets six specific word-sized fields at that offset to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_until_sentinel`: Copies words from one memory location to another until a sentinel value of -2 is encountered.
- `load_global_pointers`: Loads four consecutive word-sized values from a fixed memory region into a global variable pointer, effectively initializing a pointer to a data structure.
- `clear_memory_block`: Clears a block of memory (10280 longwords, from the count loaded into d0) starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `reset_audio_and_io`: Initializes global variables and clears specific memory locations likely used for audio buffers.
- `set_byte_values`: Writes the value 64 to four consecutive byte-sized memory locations at a fixed address.
- `clear_byte_values`: Writes zero to four consecutive byte-sized memory locations at the same fixed address used by the previous routine.
- `increment_byte_counters`: Increments four specific byte-sized memory locations by one if their current values are less than the ASCII character '@', contingent on a global variable being non-zero and divisible by four.
- `decrement_byte_counters`: Checks four specific byte-sized memory locations and decrements each by one if its current value is non-zero.
- `branch_to_address`: Performs a word-sized branch to a fixed address.
- `branch_to_sub_1552`: Performs a relative branch to another routine.
- `update_entity_status_flags`: Checks a bit in a global variable and, if set, updates a bit in another global variable using a bit shift, calculates a memory offset based on a register, and writes a value from a table and three constant bytes into that offset location; otherwise it returns.
- `branch_to_offset`: Performs a short branch to another location.
- `update_structure_pointers_2`: Sets a bit in a global variable based on a register value, calculates an offset via multiplication, and if a secondary value is non-zero, populates a structure with a pointer and several constant values derived from a table lookup.
- `initialize_audio_parameters`: Copies seven values from a table into a structure, sets several bytes and a word at specific offsets within that structure, and calls another routine four times to write specific word values to the hardware registers at offsets a fixed address, a fixed address, a fixed address, and a fixed address.
- `clear_audio_dma_buffer`: Clears a block of memory by writing zeros to word-sized elements, likely preparing a buffer for audio DMA operations given the hardware register context.
- `update_indexed_byte`: Saves registers to the stack, loads an address into a5, compares d1 to 64, and if not equal, masks d1 with 63.
- `set_byte_values_2`: Stores a byte from register d1 into four consecutive memory locations starting at offset 40 of a structure in a5, then returns.
- `update_structure_pointers`: Sets a bit in a global variable, calculates an offset using a multiplication, and if a secondary value is non-zero, copies a byte from a table into a structure and updates several fields in that structure before calling another routine.
- `process_entity_flags`: Saves registers, checks a bit in a structure, and if set, calculates an offset to write several bytes into a memory location; otherwise, it performs a series of conditional checks and calls a subroutine multiple times.
- `execute_script_logic`: Checks two conditions involving bytes at offsets 57 and 56 of a structure and a byte at offset 91 of another structure, then calls three subroutines or five subroutines based on the results.
- `process_flag_and_pointer`: Reads a byte at offset 57 from a structure, clears its lowest bit, and if the bit was set, updates a pointer and copies data from offsets 22 and 12 within the structure.
- `check_flag_and_set_status`: Reads a byte at offset 56 from a structure, clears its lowest bit, and if the bit was set, sets a byte at offset 57 to 255 and updates a word at offset 150 in another structure with a value from offset 46 ORed with 33280.
- `copy_word_value`: Copies a word-sized value from offset 46 of one structure to offset 150 of another structure.
- `process_script_operand_stream`: Reads bytes from a script data stream and dispatches operations based on the value, updating object state fields or calling another routine.
- `branch_to_update_linked_list_node`: Performs a short branch into the middle of the update_linked_list_node routine.
- `jump_to_offset`: Performs a relative branch to another location.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `branch_to_offset_197e`: Performs a short branch to a location within another routine.
- `branch_to_offset_6444`: Performs a short branch to an instruction located 6444 bytes ahead in the code segment.
- `transfer_structure_fields`: Calls a subroutine and then moves bytes from a pointer in a1 to offsets 86 and 88 of a structure in a0, or moves a byte to d0 and calls another subroutine if d0 is non-zero.
- `update_linked_list_node`: Reads a byte from a pointer in a1 and stores it at offset 48 of a0, then enters a loop that traverses a linked list or table structure by reading bytes to calculate offsets and updating a pointer at offset 78 of a0.
- `execute_three_subroutines`: Loads a pointer from a0 into a2, then calls three consecutive subroutines before returning.
- `calculate_clamped_value`: Computes a scaled product of two word values from a structure, clamps the result between 0 and 64, and stores the result back into the structure and another location.
- `update_velocity_component`: Updates a short value in a structure by adding a scaled product of a table entry and the current value.
- `calculate_scaled_value`: Performs a signed multiplication and division using bytes from a structure in a0, stores the result in a byte at offset 63 of the structure, and calls another routine if a specific byte at offset 58 is not negative.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_sequence`: Performs a relative branch, clears register d0, and then performs another relative branch.
- `initialize_velocity_value`: Sets a specific short field within a data structure to the constant value 64.
- `setup_color_attributes`: Initializes several byte fields in a structure, setting some to zero or 255, and copying a value from a table indexed by a pointer in the structure.
- `process_movement_and_timer`: Calculates a velocity adjustment using a table lookup and updates a position field.
- `initialize_object_state`: Populates a structure with values from a referenced table, including pointers and byte attributes, and sets up linked list pointers within the structure.
- `load_entity_properties`: Checks if a state byte has changed; if so, it loads multiple fields of a structure from a table indexed by that byte, including pointers and configuration values.
- `clear_flag_and_update_ptr`: Clears the least significant bit of a byte flag.
- `clear_and_store_byte`: Clears bit 0 of a byte at offset 144 of the structure; if the bit was set, it stores a byte from offset 145 into a register and loads the constant value 33248 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.

### CF2#unpacked (raw code file, load address unknown)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
