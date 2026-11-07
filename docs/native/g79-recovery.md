# Game 79 (CannonFodder2 v1.1 0104): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00079). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1530394 top-level bytes in 545 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 243 of 243 named (0 author symbols), 243 of 243 described (6 by rule, 237 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1539 of 1539 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 215766 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4703 of 4703 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 65372 |
| code | 18648 |
| data | 1446362 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| CannonFodder2.Slave | 1 | 144 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3483 | 164 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 429 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 636 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| CF2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| FONT.RAW#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- CannonFodder2_v1.1_0104/CannonFodder2/data/1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2675 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20555 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/3.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (817 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/4.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11505 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/5.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12228 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/6.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11160 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/7.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13089 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/8.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9783 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/9.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (942 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/A.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11726 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/B.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14149 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/C.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4086 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/CF2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (72121 -> 174774 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/CFTITLE.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10199 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/D.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6913 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22133 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (576 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21226 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (153 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30737 -> 36694 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1145 -> 3145 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (111 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (31882 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19858 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (386 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13121 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DESSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/DISK2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11763 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/FONT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4371 -> 40992 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/HILLS.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15067 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22872 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (542 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13511 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (118 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (38322 -> 44704 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1203 -> 3207 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (79 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICECOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25932 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICEPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8561 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICESUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (278 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICESUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9291 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICESUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (89 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/ICESUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (51 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22781 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (877 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15248 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (203 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (34296 -> 41938 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1114 -> 3095 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25703 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20493 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11497 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/INTSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (60 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JON.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (94572 -> 107786 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JON.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2563 -> 7935 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (24375 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20851 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (39226 -> 47228 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1156 -> 3153 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (68 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28698 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20295 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12327 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/JUNSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (93 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/KILLER.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (273625 -> 296072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/KILLER.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1227 -> 2989 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM1.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (256 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM1.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (91 -> 100 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM10.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (875 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM10.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (193 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM11.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (868 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM11.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (195 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM12.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (829 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM12.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM13.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (778 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM13.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (180 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM14.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1459 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM14.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (211 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM15.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1274 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM15.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (196 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM16.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (382 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM16.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM17.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1127 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM18.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (386 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM18.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (170 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM19.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1137 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM19.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM2.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (327 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM2.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (120 -> 170 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM20.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (738 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM20.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (202 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM21.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (931 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM21.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM22.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (526 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM22.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM23.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (628 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM23.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (214 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM24.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (818 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM24.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (220 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM25.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (406 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM25.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (183 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM26.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (978 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM26.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (238 -> 380 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM27.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1672 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM27.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (234 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM28.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (566 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM28.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (188 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM29.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1081 -> 2984 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM29.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (198 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM3.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (592 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM3.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (184 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM30.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (352 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM30.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (121 -> 160 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM31.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1607 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM31.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (180 -> 250 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM32.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1116 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM32.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM33.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1515 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM33.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (213 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM34.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (703 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM34.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (210 -> 310 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM35.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1363 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM35.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (192 -> 260 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM36.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (336 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM36.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 310 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM37.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1160 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM37.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (209 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM38.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (801 -> 2072 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM38.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (197 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM39.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1498 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM39.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM4.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (538 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM4.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (166 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM40.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1559 -> 5454 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM40.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM41.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (565 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM41.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (174 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM42.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1471 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM42.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM43.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (492 -> 1806 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM43.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (201 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM44.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1078 -> 4048 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM44.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (215 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM45.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1427 -> 4770 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM45.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (224 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM46.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1360 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM46.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (222 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM48.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (961 -> 4428 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM48.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (217 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM49.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (992 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM49.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (230 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM5.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (514 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM5.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (177 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM50.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1079 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM50.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (191 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM51.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1130 -> 3516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM51.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (192 -> 220 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM52.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (855 -> 3516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM52.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (187 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM53.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (822 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM53.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (215 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM54.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (562 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM54.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (216 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM55.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1303 -> 4516 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM55.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (234 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM56.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (981 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM56.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (177 -> 230 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM57.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (977 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM57.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (225 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM58.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1345 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM58.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (229 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM59.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (281 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM59.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (176 -> 280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM6.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (373 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM6.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (149 -> 200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM61.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (467 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM61.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (109 -> 220 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM62.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (310 -> 1236 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM62.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (120 -> 200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM63.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (420 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM63.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (151 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM64.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (672 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM64.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM66.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (933 -> 2718 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM66.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (207 -> 270 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM67.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (181 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM67.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (171 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM69.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (929 -> 3060 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM69.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (191 -> 230 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM7.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (355 -> 1084 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM7.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 290 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM70.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1145 -> 4542 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM70.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (204 -> 250 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM72.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (188 -> 666 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM72.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (55 -> 70 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM8.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (747 -> 1578 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM8.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (200 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM9.MAP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1207 -> 4464 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MAPM9.SPT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (190 -> 300 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORARMY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22721 -> 53760 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (448 -> 1920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12619 -> 30720 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (166 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35336 -> 40872 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.SNG: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1139 -> 3011 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORBASE.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (83 -> 480 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORCOPT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30304 -> 53920 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORPHBIG.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10666 -> 67200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORPLAY.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15367 -> 36000 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORSUB0.BHT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (185 -> 1280 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORSUB0.BLK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8971 -> 20352 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORSUB0.HIT: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (114 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/MORSUB0.SWP: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (86 -> 320 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/PSTUFF.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11229 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/RANKFONT.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15102 -> 51200 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/SENSPROD.PAL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (93 -> 96 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/SENSPROD.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7533 -> 51400 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/STARTUP.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8348 -> 40960 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/VIRGPRES.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5374 -> 41120 bytes)
- CannonFodder2_v1.1_0104/CannonFodder2/data/WON.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8328 -> 51400 bytes)
- runtime segment an address: 651 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 318 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 487 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- CannonFodder2_v1.1_0104/CannonFodder2/data/CF2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CannonFodder2_v1.1_0104/CannonFodder2/data/FONT.RAW#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 215766 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### CannonFodder2.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_system_pointers`: Executes function pointers from a structure passed in A0, then clears two memory regions by writing zeros to fixed addresses.
- `clamp_or_fetch_value`: Compares a value in D0 against a threshold; if it is below the threshold, it returns a value from a structure pointed to by A0, otherwise it returns the original value in D0.
- `construct_save_disk_path`: Constructs a file path for saving game data by concatenating a base directory string with a disk identifier (e.g., DISK1, DISK2) and a filename, handling different disk selection modes via a switch on the input parameter.
- `compare_string_bytes`: Iterates through two strings byte by byte, comparing them until a mismatch or a null terminator is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_custom_hardware`: Configures Amiga custom chip registers for DMA and interrupts, initializes global memory tables and counters, and enters a main processing loop that dispatches commands and handles input.
- `initialize_game_state_and_loop`: Sets global variables to specific constants and initializes memory tables by copying data blocks and clearing fields.
- `run_main_game_loop`: Executes a continuous loop that repeatedly calls various subroutines to process game state, update arrays, and handle global variables.
- `check_global_conditions`: Checks multiple global variables for specific values.
- `infinite_loop_stub`: Contains an infinite loop with no operations, effectively doing nothing and never returning.
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
- `load_map_file`: Initializes global pointers and flags, then enters a loop that repeatedly calls a data loading routine and toggles a global bit until a zero return value is received.
- `copy_string_until_mismatch`: Compares two strings byte-by-byte; if they match up to a null terminator, it copies the first string to the second location, otherwise it stops at the first mismatch.
- `initialize_game_resources`: Sets global variables to specific addresses and calls multiple helper routines to load map data, process strings, and clear memory regions, likely preparing the game state.
- `initialize_string_buffer`: Calls several helper routines and then sets a global variable to zero, likely preparing a buffer for string operations.
- `load_killer_assets`: References text strings for 'KILLER.INS' and 'KILLER.SNG' and calls a command dispatch routine, suggesting it loads or initializes specific game assets.
- `copy_data_block_1`: Copies a fixed number of words from a specific source address in memory to a destination address, decrementing a counter until the transfer is complete.
- `copy_data_block_2`: Copies a fixed number of words from a specific source address in memory to a global variable's address, decrementing a counter until the transfer is complete.
- `clear_memory_block_1`: Sets a fixed number of words to zero starting from a global variable's address, decrementing a counter until the block is cleared.
- `clear_memory_block_2`: Sets a larger fixed number of words to zero starting from a global variable's address, decrementing a counter until the block is cleared.
- `copy_80_word_table`: Copies 80 words from a fixed source address to a global variable buffer.
- `clear_global_array`: Iterates through a global array of word-sized entries, setting each element to zero.
- `load_map_data`: Copies data from a fixed memory address into a global buffer until a terminator byte is found, then copies the global buffer to another location.
- `loop_pixel_processor`: Iterates a counter from a starting value with a specific step size, calling a pixel data processing routine in each iteration until the counter exceeds a limit.
- `reset_state_counters`: Checks global state variables and resets them to specific values, then calls routines to process word arrays and temporarily zero out a global variable.
- `zero_global_fields`: Sets several consecutive word-sized fields within a global variable structure to zero.
- `reset_counter_table`: Checks a global counter; if it is zero, it clears a series of bytes in a table at a fixed address and manipulates other global variables and a lookup table.
- `increment_table_bytes`: Iterates through a table of structures at a fixed address.
- `calculate_position_coords_2`: Computes coordinate values based on global variables and input parameters, calling helper routines to adjust values.
- `calculate_position_coords`: Computes coordinate values based on global variables and input parameters, calling helper routines to adjust values.
- `adjust_global_value_2`: Modifies a global variable by subtracting a constant and checking bounds.
- `adjust_global_value`: Modifies a global variable by subtracting a constant and checking bounds.
- `toggle_global_bit`: Toggles a single bit in a global variable and calls another routine.
- `handle_command_dispatch`: Checks a global flag and dispatches to different routines based on the value returned by a helper function.
- `decode_compressed_data`: Checks a global flag and, if set, decodes a sequence of bytes by combining shifted nibbles into words and storing them into a destination.
- `toggle_global_flag`: Checks a global variable against a specific constant value.
- `find_pattern_index`: Searches through a global array for a sequence of three bytes that match specific values stored in global memory.
- `parse_byte_table`: Scans a byte array at a fixed address, extracting values where the upper bits are set.
- `check_and_store_byte_2`: Checks if a global variable is zero and if a register value matches it.
- `check_and_store_byte`: Checks if a global variable is zero and if a register value matches it.
- `adjust_byte_counter`: Modifies a byte in memory by incrementing or decrementing it based on comparisons with a global variable.
- `check_global_and_dispatch`: Stores a value into a global variable.
- `dispatch_indirect_call`: Calculates an address using a global variable and an index.
- `set_global_constants`: Assigns specific constant values to two global variables.
- `check_death_condition`: Checks a global variable against specific values and, if conditions are met, resets the variable and calls other routines, likely associated with the text 'TIME TO DIE'.
- `copy_data_blocks_loop`: Sets global variables to specific values and calls helper routines in a sequence, likely managing data copying or memory initialization through repeated calls.
- `manipulate_word_pairs`: Conditionally moves or splits word-sized values within a memory structure based on a global variable, performing bitwise shifts and subtractions.
- `move_and_clear_word_2`: Copies a word-sized value from one memory offset to another within a base address, then clears the source location to zero.
- `move_and_clear_word`: Copies a word-sized value from one memory offset to another within a base address, then clears the source location to zero.
- `setup_entity_structure`: Initializes a structure at a global pointer address with specific constant values, offsets, and a function pointer, then calls a helper routine if the pointer is valid.
- `set_entity_flags_2`: Checks condition flags and increments a global counter modulo 3, then sets specific bytes in a memory table to 255 if a corresponding word flag is not already set.
- `set_entity_flags`: Checks a word-sized flag in memory and sets specific bytes to 255 if the flag is not already set.
- `set_flag_and_bytes`: Checks a word-sized flag in memory.
- `reset_pixel_shift_state`: Checks if a global variable is non-zero and resets it if necessary.
- `reset_pixel_shift`: Checks if a global variable is non-zero; if so, it resets the variable, sets it to one, and calls other routines to handle pixel shifting.
- `call_two_helpers`: Invokes two other routines sequentially and returns.
- `initialize_entity_header`: Checks if a global pointer is valid, then writes specific constant values and a function pointer into a structure at that address.
- `update_entity_state`: Checks global state and flags; if conditions are met, it increments a counter, resets it if too high, and sets specific byte values in memory based on the counter.
- `update_entity_flags`: Checks a global state and zero flag; if conditions are met, it increments a counter, resets it if too high, and sets specific byte values in memory tables.
- `toggle_global_state`: Evaluates a zero flag and global variable to either reset the global variable to zero and one, or set it to a maximum value, calling another routine in each branch.
- `check_global_state_2`: Reads a global variable and compares it against specific integer values.
- `check_global_state_3`: Reads a global variable and compares it against specific integer values.
- `scan_null_terminated`: Iterates through a small array of global variables, checking for null terminators; if a null is found and a global is negative, it returns, otherwise it calls other routines.
- `conditional_pixel_shift`: Checks if a global variable is non-negative; if so, it calls a routine that performs bitwise shifts and logical operations on pixel data blocks.
- `return_pixel_shift_result`: Checks if a global variable is non-negative; if so, it calls a pixel data shifting routine and returns its result, otherwise returning the global variable itself.
- `invoke_data_copy`: Calls a single routine that copies blocks of data from a source to a destination pointer until a negative value is encountered.
- `initialize_pointer_structure`: Writes specific values to memory locations pointed to by registers, including setting up a structure with fixed addresses and values.
- `toggle_flags_and_call`: Conditionally sets and clears several global variables based on processor flags.
- `toggle_global_flags_and_call_helpers`: Conditionally sets and clears specific global variables to one or zero, then invokes three distinct helper routines.
- `search_and_link_entities`: Iterates through a small table to find an empty slot, then scans a larger entity table to link entities based on pointer validity and bit flags, clearing specific fields.
- `clear_bit_in_table_2`: Iterates through a table of 8 entries.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `count_active_entities`: Iterates through a table of 8 entries, counting how many meet specific criteria involving pointer validity and bit flags.
- `clear_bit_in_table`: Iterates through a table of 8 entries.
- `setup_entity_with_offset`: Iterates backwards through a global array to find a null terminator, then initializes a structure at a global pointer with specific values and a calculated offset.
- `toggle_tile_state_flag`: Calculates an index from global variables, searches a table for a matching entry, and toggles a byte flag within that entry if found, then calls helper routines.
- `toggle_tile_state`: Calculates an index from global variables, iterates through a table of structures to find a match, and toggles a byte flag within the matched structure.
- `copy_struct_list_until_negative`: Copies blocks of data from a source pointer to a destination pointer in a loop, terminating when the source value is negative.
- `write_invalid_pointer`: Writes the value a fixed address to a memory location pointed to by a register.
- `process_pixel_data_block`: Writes a value to a memory address calculated from a helper routine's return value, then conditionally calls multiple helper routines based on a global variable check.
- `update_state_flag`: Checks a word-sized flag in memory.
- `set_flag_and_bytes_2`: Checks a word-sized flag in memory; if conditions are met, it sets specific byte values in memory based on the flag and a global variable.
- `process_state_arrays`: Iterates through two byte arrays and a word array, toggling values between 1 and 3 based on null terminators and global state.
- `check_global_state_value`: Checks a global variable against specific values and iterates through a table to find a matching entry.
- `check_global_state_4`: Checks a global variable against specific values and iterates through a table to find a matching entry, calling other routines if conditions are met.
- `call_subroutines_conditionally`: Calls other routines based on the value of a global variable.
- `count_active_entities_2`: Iterates through a table of entries, counting how many meet specific criteria involving pointer validity and bit flags, and updates global variables accordingly.
- `check_global_state`: Reads a global variable and compares it against specific integer values.
- `check_entity_state_flags`: The routine iterates through a table of entries, counting those that meet specific pointer and flag criteria.
- `update_global_variables`: Updates several global variables with values from registers and performs a comparison to set one of the globals to a specific value if a condition is met.
- `call_pixel_processor`: The routine invokes a helper function to process pixel data based on calculated addresses and global state.
- `check_and_shift_pixels`: The routine initializes a return value, checks a global variable condition, and conditionally calls functions to initialize state and shift pixel data blocks.
- `check_global_state_flag`: Checks a value derived from a global variable; if non-zero, it calls another routine and returns its result, otherwise returns zero.
- `call_state_check_routine`: Calls another routine twice in succession.
- `shift_pixel_blocks`: The routine calls a function to perform bitwise shifts and logical operations on blocks of pixel data.
- `empty_routine`: This routine is empty and does not perform any operations.
- `process_pixel_tile_data`: This routine calculates addresses using global variables and processes pixel data blocks through bitwise shifts.
- `initialize_and_call_functions`: Sets a memory location to a specific value, updates global variables, and calls several functions in sequence.
- `write_to_memory_locations`: Writes values from registers to multiple memory locations in a loop, decrementing a counter until it reaches a specific value.
- `write_registers_to_memory_loop`: Writes values from registers to six distinct memory locations in a loop, decrementing a counter until it reaches -1.
- `write_to_memory_locations_2`: Writes values from registers to multiple memory locations in a loop, incrementing an offset and decrementing a counter until it reaches a specific value.
- `loop_copy_word_blocks`: Iterates through a global variable pointer while the value is non-negative, calling a routine to copy a block of 242 words.
- `call_copy_word_block`: Calls a routine that copies 242 words from a source pointer to a destination pointer.
- `set_global_variable`: Sets a global variable to a value passed in a register.
- `copy_data_to_memory`: Copies data from a source pointer to multiple memory locations, including a specific hardware register, in a loop until a counter reaches a specific value.
- `copy_word_block_242`: Copies 242 words from a source pointer to a destination pointer.
- `copy_word_block_loop`: Copies a block of 242 words from a source pointer to a destination pointer using a decrementing counter loop.
- `copy_word_block_242_2`: Copies 242 words from a source pointer to a destination pointer without returning a value.
- `process_pixel_data_with_shifts`: Calculates an address based on register values and global variables, then calls a routine to process pixel data.
- `initialize_entry_table`: Sets a global variable to -1 and initializes a table of 7 entries at a fixed address with -1, 0, and 3 values.
- `sort_and_compact_table`: Compacts a table by shifting valid entries down and then sorts the entries based on specific field comparisons.
- `manage_table_entries_loop`: Iterates through a table of entries, copying data between memory locations and checking for empty slots marked by -1.
- `allocate_table_entry`: Searches a table for an empty entry marked by -1, assigns it a new ID from a global counter, and initializes its fields.
- `copy_buffer_to_global`: Copies 1831 bytes from a fixed memory address to a global variable buffer.
- `copy_global_to_buffer`: Copies 1831 bytes from a global variable buffer to a fixed memory address.
- `process_tile_string_data`: The routine iterates through a string buffer, skipping spaces and null terminators, and conditionally calls a helper routine for non-space characters based on a global counter.
- `process_tile_data`: Calculates an address based on input registers and a global variable, then calls other routines conditionally based on register values.
- `shift_pixel_data_blocks`: Performs bitwise shifts and logical operations on blocks of pixel data to adjust alignment or blend colors.
- `shift_pixel_data`: Performs bitwise shifts and logical operations on blocks of pixel data to adjust alignment or blend colors.
- `check_global_value`: Reads a global variable and compares it against specific constants.
- `clear_ciaa_interrupts`: The routine writes a value to the CIAA Interrupt Control Register to acknowledge or clear pending interrupts.
- `update_interrupt_state`: Sets a global variable to 4, calls a state update routine, and returns a value constructed from registers, while touching the INTREQ hardware register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_byte_table_entry`: The routine reads a byte from a table using an index stored in a global variable.
- `clear_memory_regions`: Conditionally zeroes out two distinct blocks of memory by iterating through word-sized pointers if specific helper routines return non-zero values.
- `clear_other_regions`: Similar to the previous routine, it conditionally zeroes out two different memory blocks by iterating through word-sized pointers based on helper routine returns.
- `process_map_data`: Converts a string to lowercase, then iterates through a table of structures, copying fields, checking for specific byte values to increment counters, and setting flags based on content.
- `convert_string_to_uppercase`: The routine iterates through a null-terminated string pointed to by a register.
- `copy_string_with_lowercase`: The routine copies characters from a source string to a fixed memory location, converting uppercase letters to lowercase.
- `dispatch_command_handler`: Checks a global flag and dispatches to different routines based on a value returned by a helper.
- `toggle_global_bit_flag`: The routine updates a global variable by calling a helper that toggles a bit and performs conditional checks on register values.
- `conditional_call_empty_routine`: Checks if a register value is not -1.
- `copy_and_convert_strings`: The routine copies null-terminated strings from source pointers to fixed memory locations, converting characters to lowercase or uppercase as specified by the called helpers.
- `copy_string_with_lowercase_2`: Copies characters from a source string to a fixed memory location, converting uppercase letters to lowercase.
- `call_recursive_routine`: The routine calls another routine and returns.
- `decode_compressed_byte_data`: The routine checks a global flag, and if set, decodes a sequence of bytes by combining shifted nibbles into words and storing them into a destination array.
- `build_huffman_decode_table`: The routine allocates a buffer on the stack and populates it with node data by reading from a source table, likely constructing a Huffman decoding tree or lookup table for decompression.
- `scan_table_for_match`: Iterates through a table of words to find an entry matching a condition involving a global register value.
- `mask_and_conditional_call`: Masks a word from a register with a word from another register.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer and builds a 32-bit value by concatenating each byte into the result.
- `build_huffman_tree_nodes`: The routine iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties like matched bytes and lengths into a destination array to build a tree structure.
- `tiny_ori`: Small routine of 1 instruction that merges 70 into -(a0).
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_global_flags`: The routine sets several global variables and memory locations to specific constant values, including zeroing some fields and setting others to non-zero constants.
- `reset_potgo_and_flags`: The routine sets the POTGO hardware register to trigger port reads, initializes several global variables and memory locations to specific constant values, and processes mouse movement data.
- `process_mouse_movement`: Reads mouse port movement data from hardware registers and updates global variables representing position deltas.
- `check_joystick_button_state`: The routine reads hardware registers for joystick and mouse button states.
- `clear_color_registers`: The routine writes zero to a series of memory-mapped hardware registers, specifically clearing color registers and other display-related control registers in a loop.
- `update_state_counter`: Adjusts a global variable based on conditional checks and calls other routines to modify state values.
- `adjust_state_counter`: Adjusts a global variable based on conditional checks.
- `process_word_array_2`: Iterates through an array of words, performing bitwise operations and adjustments based on a global variable.
- `temporarily_zero_global`: Saves the current value of a global variable, sets it to zero, calls another routine, and then restores the original value of the global variable.
- `process_word_array`: Iterates through an array of words, performing bitwise operations and adjustments based on a global variable.
- `check_and_reset_flag`: Checks a global flag and resets it if it is non-zero.
- `empty_placeholder`: The routine is empty, containing no executable instructions.
- `configure_hardware_registers`: Sets specific hardware registers including BPLCON0, CIAA_ICR, DMACON, INTENA, and VPOSR to initialize or configure system settings.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_with_ff`: Fills a block of memory starting at a specific address with the value 255.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_interleaved_memory_blocks`: Copies words from a source address to a destination address using a loop, while simultaneously performing non-sequential reads from offset locations within the source block to a third destination.
- `compute_bitwise_operations`: Performs a series of bitwise operations and arithmetic on a global variable, adjusting its value based on specific conditions.
- `set_global_to_bc_2`: Checks if a global variable is non-zero.
- `set_global_to_bc`: Sets a global variable to the value 188.
- `initialize_memory_blocks`: Initializes memory blocks by setting specific values and performing bitwise operations on a global variable.
- `initialize_game_state`: Performs extensive initialization by setting global variables, calling multiple setup routines, and copying word-sized data blocks between fixed memory addresses.
- `manage_global_state_loop`: The routine initializes a global variable and enters a loop that calls another routine repeatedly while decrementing a counter.
- `copy_memory_blocks`: Copies a fixed number of words from two source memory regions into two destination memory regions using a loop.
- `process_text_file`: Iterates through a table of pointers to strings, checking character values and conditionally copying memory blocks.
- `copy_word_data`: Copies word-sized data from one memory location to another, iterating through a fixed number of elements.
- `copy_word_data_duplicate`: Copies word-sized data from one memory location to another, iterating through a fixed number of elements, similar to another routine.
- `clear_byte_array_2`: Sets eight consecutive byte-sized global variables to zero.
- `decrement_global_counter_2`: Decrements a global variable twice if it is non-zero, then calls another routine twice.
- `decrement_global_counter`: Decrements a global variable twice if it is non-zero, then calls another routine twice.
- `clear_byte_array`: Sets eight consecutive byte-sized global variables to zero.
- `swap_memory_blocks`: Rotates data between three memory regions by copying words from one pointer to another in a loop, shifting values through intermediate buffers.
- `copy_words_to_buffer`: Copies a sequence of words from a source pointer into a fixed memory address, decrementing a counter until completion.
- `restore_memory_blocks`: Copies words from two fixed memory addresses back into two other fixed memory addresses, reversing a previous data shift.
- `restore_second_block`: Copies words from a fixed memory address into another fixed memory address, restoring data for a specific block.
- `update_scroll_offset`: Increments a pointer with wraparound logic, calculates a new value using indexed memory and global variables, and updates a global state variable with the result.
- `initialize_status_flags`: Sets a series of global variables to specific constant values, mostly zero or -1, with one set to a small positive integer.
- `initialize_memory_blocks_2`: Initializes memory blocks by setting specific values and performing bitwise operations on a global variable.
- `clear_struct_fields`: Calculates an offset from a global variable and sets six specific word-sized fields at that offset to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_until_sentinel`: Copies words from one memory location to another until a sentinel value of -2 is encountered.
- `load_global_pointers`: Loads four consecutive word-sized values from a fixed memory region into a global variable pointer, effectively initializing a pointer to a data structure.
- `clear_word_array`: Iterates through a block of memory pointed to by a register, writing zero to each word-sized element for a fixed number of iterations.
- `reset_audio_and_io`: Initializes global variables and clears specific memory locations likely used for audio buffers.
- `set_byte_values`: Writes the value 64 to four consecutive byte-sized memory locations at a fixed address.
- `clear_byte_values`: Writes zero to four consecutive byte-sized memory locations at the same fixed address used by the previous routine.
- `increment_byte_counters`: Increments four specific byte-sized memory locations by one if their current values are less than the ASCII character '@', contingent on a global variable being non-zero and divisible by four.
- `decrement_byte_counters`: Checks four specific byte-sized memory locations and decrements each by one if its current value is non-zero.
- `process_entity_flags_loop`: Checks a global condition and iterates through four indices, calling a helper routine for each while updating global state variables.
- `initialize_audio_registers`: Sets up audio hardware by writing specific values to memory-mapped registers at addresses starting with a fixed address and a fixed address.
- `update_entity_table_entry`: Conditionally updates a memory table by calculating an index from input registers, writing specific byte values, and copying a pointer from one global structure to another.
- `check_byte_sequence_nulls`: This routine iterates through a sequence of bytes derived from a base pointer and register values.
- `update_structure_pointers_2`: Calculates an offset based on an input value and updates a structure in memory with pointers and byte values.
- `clear_audio_buffers`: Initializes several global variables and memory locations to zero, including a block of memory likely used for audio DMA operations, and sets a global variable to the value 15.
- `clear_audio_dma_buffer`: Clears a block of memory by writing zeros to word-sized elements, likely preparing a buffer for audio DMA operations given the hardware register context.
- `update_ram_byte_array`: The routine calculates an index from a register value and stores a byte into a specific RAM location.
- `set_byte_values_2`: Stores a single byte value into four consecutive memory locations at offsets 40 through 43 from a base pointer.
- `update_structure_pointers`: Updates a byte in a global variable based on an input value, then calculates offsets to copy a word-sized value from one structure to another, setting specific bytes in a target structure to predefined values.
- `process_entity_flags_2`: Checks bytes at specific offsets in two structures and updates a global variable based on the results.
- `process_entity_flags`: Checks bytes at specific offsets in two structures.
- `process_flag_and_pointer`: Reads a byte at offset 57 from a structure, clears its lowest bit, and if the bit was set, updates a pointer and copies data from offsets 22 and 12 within the structure.
- `check_flag_and_set_status`: Reads a byte at offset 56 from a structure, clears its lowest bit, and if the bit was set, sets a byte at offset 57 to 255 and updates a word at offset 150 in another structure with a value from offset 46 ORed with 33280.
- `copy_word_value`: Copies a word-sized value from offset 46 of one structure to offset 150 of another structure.
- `process_script_operand_stream`: Reads bytes from a script data stream and dispatches operations based on the value, updating object state fields or calling sub-routines.
- `fetch_script_operand`: Reads a byte from the script data stream and uses it as an index to look up a value in a global table.
- `execute_script_bytecode`: Interprets a stream of script bytecode by dispatching operations based on instruction values and updating object state fields.
- `process_script_bytecode_stream_2`: Reads and interprets a stream of script bytecode, updating object state fields and dispatching operations based on instruction values.
- `process_script_bytecode_stream_3`: The routine reads a stream of script bytecode from a pointer, updates fields of a structure pointed to by a pointer, and dispatches operations based on instruction values.
- `decode_script_byte_sequence`: Reads a byte from a script stream and uses it as an index into a global lookup table to determine execution flow.
- `process_script_bytecode_stream`: Reads and interprets a stream of script bytecode, updating object state fields and dispatching operations based on instruction values.
- `parse_script_bytecode`: Executes a bytecode interpreter loop that reads instructions from a data stream, updates a program counter, and manipulates stack or state variables based on conditional logic and table lookups.
- `chain_velocity_calculations`: Sequentially invokes three helper routines to compute clamped values, update velocity components, and adjust a byte counter.
- `calculate_clamped_value`: Computes a scaled product of two word values from a structure, clamps the result between 0 and 64, and stores the result back into the structure and another location.
- `update_velocity_component`: Updates a short value in a structure by adding a scaled product of a table entry and the current value.
- `calculate_velocity_component`: Computes a value by multiplying and dividing bytes within a structure, subtracts the result from another field, and decrements a counter.
- `clear_pointer_offset`: Stores a zero byte into a memory location calculated by adding an offset to a pointer held in register A0.
- `copy_struct_bytes`: Reads bytes from a source structure pointed to by a global pointer and copies them into specific offsets of a destination structure, setting a status byte to two.
- `initialize_velocity_value`: Sets a specific short field within a data structure to the constant value 64.
- `setup_color_attributes`: Initializes several byte fields in a structure, setting some to zero or 255, and copying a value from a table indexed by a pointer in the structure.
- `process_movement_and_timer`: Calculates a velocity adjustment using a table lookup and updates a position field.
- `initialize_object_state`: Populates a structure with values from a referenced table, including pointers and byte attributes, and sets up linked list pointers within the structure.
- `load_entity_properties`: Checks if a state byte has changed; if so, it loads multiple fields of a structure from a table indexed by that byte, including pointers and configuration values.
- `clear_flag_and_update_ptr`: Clears the least significant bit of a byte flag.
- `check_flag_and_return_code`: Clears the least significant bit of a byte flag.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_routine`: This routine is empty and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_stub`: The routine is empty and returns immediately without performing any operations or modifying data.
- `do_nothing_r1`: The routine is empty and returns immediately without performing any operations.
- `return_immediately`: The routine contains no instructions and returns immediately.
- `do_nothing_empty_stub`: The routine is empty and immediately returns without performing any operations.

### CF2#unpacked (raw code file, load address unknown)

- `initialize_system_and_load_assets`: Configures hardware registers, clears memory, and initializes various system tables and interrupt states.
