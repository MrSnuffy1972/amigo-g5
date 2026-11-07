# Game 54 (SensibleWorldOfSoccer v1.7 0840): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00054). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1469630 top-level bytes in 241 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 13738 raw-word lines |
| Functions named and described | pass, 222 of 222 named (0 author symbols), 222 of 222 described (16 by rule, 204 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1297 of 1297 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 390042 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18117 of 18126 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232686 |
| code | 16222 |
| data | 1220716 |
| padding | 6 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SensibleWorldOfSoccer.Slave | 1 | 557 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3394 | 175 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 354 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| SWOS2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| SWOS2.REL#unpacked (raw code file, load address unknown) | 1 | 13759 | 1 |

## Cases handled

- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/SWOS2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (149420 -> 335006 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/SWOS2.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35980 -> 55036 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/EUROCUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5913 -> 10944 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/EUROCWC.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11655 -> 21888 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/POOLPLYR.DAT: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (27025 -> 52200 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.000: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5270 -> 11630 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.001: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4208 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.002: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7888 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.003: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5488 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.004: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5707 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.005: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5055 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.006: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5062 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.007: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4065 -> 8894 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.008: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (29556 -> 63614 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.010: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4477 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.011: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3350 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.012: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4528 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.013: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15781 -> 32150 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.014: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13889 -> 28046 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.015: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6891 -> 14366 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.016: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5845 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.017: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3040 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.018: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4569 -> 10262 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.019: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4874 -> 10262 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.020: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15076 -> 31466 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.021: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3207 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.022: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3842 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.023: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3641 -> 7526 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.024: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3858 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.025: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12348 -> 24626 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.026: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4977 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.027: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5223 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.028: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7097 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.029: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6456 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.030: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6426 -> 13682 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.031: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7188 -> 15734 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.032: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3293 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12281 -> 27362 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.034: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6532 -> 13682 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.035: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14015 -> 30098 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.036: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4978 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.037: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5544 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.038: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6427 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.039: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5798 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.040: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6285 -> 15050 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.041: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5767 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.042: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4167 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.043: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12661 -> 28730 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.044: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15408 -> 34886 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.045: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5049 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.046: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8839 -> 22574 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.048: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5927 -> 12998 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.049: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5383 -> 11630 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.050: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3821 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.051: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12094 -> 31466 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.055: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9007 -> 19154 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.060: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11485 -> 27362 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.062: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9591 -> 21206 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.064: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6958 -> 16418 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.065: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5346 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.066: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3223 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.067: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3361 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.069: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7173 -> 14366 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.071: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7208 -> 17786 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.072: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15503 -> 32834 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.073: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2816 -> 6158 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.075: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1915 -> 4106 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.076: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5558 -> 12314 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.077: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4866 -> 10946 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.078: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3849 -> 8210 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.079: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2413 -> 5474 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.080: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (17076 -> 32834 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.081: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10790 -> 23258 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.082: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3477 -> 6842 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.083: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4338 -> 9578 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.084: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10125 -> 21206 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/TEAM.085: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1492 -> 2738 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/data/UEFACUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11321 -> 21888 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2143 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4593 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (972 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2928 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6338 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6158 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6251 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4608 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/DISK2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8324 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER00.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2144 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER01.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1252 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9011 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER10.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2258 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3434 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2929 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER4.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2892 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER5.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3347 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER6.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6073 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER7.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6583 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER8.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6825 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/LOADER9.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2885 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/MENUBG2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (18119 -> 47872 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4067 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/MENUS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5267 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SOCCER_S.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5771 -> 40960 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SWCPICH1.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13440 -> 45464 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SWCPICH2.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12574 -> 39320 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SWCPICH3.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14378 -> 42904 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SWCPICH4.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12186 -> 39704 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/grafs/SWCPICH5.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13632 -> 44184 bytes)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/sound/HERO.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (250368 -> 292984 bytes)
- runtime segment an address: 1078 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 66 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 28 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/SWOS2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/SWOS2.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SensibleWorldOfSoccer_v1.7_0840/SensibleWorldOfSoccer/swos11.dku: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 1 never-loaded region(s), 335006 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_disk_dma_transfer` (runtime segment an address): The routine initializes DMA control registers and disk path/length parameters, then loops to poll a status flag while managing a counter. Kind: disk hardware timing/checksum trap. Pass path: bne.b an address.
- `configure_dma_registers` (runtime segment an address): The routine writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register. Kind: disk hardware check. Pass path: bne.b an address (branch not taken, i.e., Z flag set).
- `poll_flag_and_set_timer` (runtime segment an address): Polls a byte in a structure for a specific bit flag. Kind: disk hardware timing check. Pass path: returns 0 (success) when the flag bit is set; returns -1 (failure) if the loop exits without the flag being set.
- `set_flag_and_wait_timer` (runtime segment an address): Sets a global variable to a constant and waits for another routine to return a specific value. Kind: disk hardware timing check. Pass path: returns 0.
- `mask_input_and_wait_timer` (runtime segment an address): Masks a byte from a routine call and sets a global variable with the result. Kind: disk hardware timing check. Pass path: returns 0.
- `mask_input_byte` (runtime segment an address): The routine builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set. Kind: disk hardware check (CIAB PRB port). Pass path: routine returns with D0 modified based on CIAB_PRB state; caller sub_1622 checks D1 (likely status/error flag) and branches if negative (fail), otherwise continues (pass).

## Functions


### SensibleWorldOfSoccer.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_constant_to_pointer`: The routine stores a specific 32-bit constant value into a memory location pointed to by a register.
- `patch_code_for_version`: Detects the game version by scanning memory for magic values and patches specific code offsets with new instructions or pointers based on the detected version.
- `scan_memory_for_magic`: Iterates through a block of memory, checking for specific magic values and a pointer match.
- `call_function_pointer`: The routine invokes a function located at an address stored in a global variable.
- `empty_stub_2`: The routine is empty and performs no operations.
- `validate_and_copy_path`: Checks if a pointer points to specific magic values (data, graf, or soun); if not, it copies the string from that pointer to a global buffer, then calls another routine.
- `copy_string_to_buffer`: Copies a null-terminated string from a source pointer to a global buffer, then calls another routine and returns a register value.
- `search_and_shift_buffer`: Iterates through a buffer comparing characters against a source string.
- `search_and_replace_string`: Iterates through a buffer in 32-byte chunks, comparing each chunk against a source string.
- `shift_buffer_and_copy`: Shifts bytes in a buffer pointed to by a register, copies a null-terminated string to a global buffer, and calls two other routines before returning.
- `search_and_concatenate_path_2`: Iterates through a table comparing strings.
- `search_and_concatenate_path`: Iterates through a table comparing strings; upon finding a match or reaching a limit, it copies a source string to a buffer, appends a second string, and calls two other routines.
- `concatenate_path_strings`: Copies a source string into a buffer, then appends a second string (referenced as 'SAVE/') to it.
- `call_and_return_zero`: The routine calls another routine and then returns a zero value.
- `call_routine_and_return`: Calls another routine via a pointer dereference and then returns zero.
- `call_library_function`: Calls a single unnamed routine and returns the value passed back in the D0 register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_offset_pointer`: The routine calls a function located at a specific offset from a pointer stored in a global variable.
- `call_global_table_offset_0x24`: Loads a function pointer from offset 36 of a global table and executes it.
- `tiny_ori_nop`: Small routine of 2 instructions that merges 0 into d0, then does nothing for one instruction.
- `call_global_function_pointer`: Loads a function pointer from a specific offset within a global variable and executes the routine it points to.
- `dereference_and_call_pointer`: The routine loads a pointer from a global variable and calls the routine located at that address.
- `call_global_function_pointer_2`: Loads a function pointer from a global variable and executes it.
- `call_offset_0x58_function`: Loads a function pointer from a global table at a specific offset and executes it.
- `call_offset_pointer_short`: The routine calls a function located at a specific offset from a pointer stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_interrupts`: Fills specific memory regions with alternating bit patterns and initializes hardware registers for CIA, DMA, and interrupts.
- `initialize_memory_patterns_and_interrupts`: Fills specific memory regions with alternating bit patterns and initializes hardware registers for CIA, DMA, and interrupts.
- `initialize_system_interrupts`: Clears memory regions and configures hardware registers for CIA, DMA, and interrupts.
- `dispatch_function_pointer`: The routine calculates an address using a register offset and calls the function located at that address.
- `filter_code_block`: The routine iterates through a block of code words, copying them to a destination while zeroing out specific words based on a bitmask.
- `call_two_routines`: Calls two other routines sequentially and then returns.
- `empty_unreached_stub_2`: The routine is empty and unreached, containing only three instructions that decode as code in a gap.
- `return_zero_constant`: The routine is empty and unreached, containing only three instructions that decode as code in a gap.
- `return_zero_constant_2`: The routine immediately returns a zero value without performing any other operations or modifying memory.
- `modify_memory_and_call_routines`: The routine modifies bytes at a pointer by ORing with constants, inverts bytes at calculated offsets, and calls two other routines.
- `empty_unreached_stub`: The routine is empty and unreached, containing only three instructions that decode as code in a gap.
- `tiny_moveq_tst_rts`: Small routine of 3 instructions that loads 0 into d0, then tests d0, then returns.
- `load_grafs_resource`: The routine references a text string for a grafs loader resource and calls an external function twice.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `dispatch_disk_parse_drive`: Checks a byte value to dispatch to disk transfer initialization, array summing, or drive identifier parsing routines.
- `dispatch_disk_process_audio`: Checks a byte value to dispatch to disk transfer initialization or audio buffer processing routines.
- `dispatch_disk_check_flag`: Checks a byte value to dispatch to disk transfer initialization or flag checking routines.
- `dispatch_disk_copy_table`: Checks a byte value to dispatch to disk transfer initialization or table copying routines.
- `dispatch_disk_init_block`: Checks a byte value to dispatch to disk transfer initialization or block setup routines.
- `dispatch_disk_alloc_string`: Checks a byte value to dispatch to disk transfer initialization or string buffer allocation routines.
- `dispatch_disk_mode_clear`: Checks a byte value to dispatch to specific disk transfer configuration or offset clearing routines.
- `dispatch_disk_operation_mode`: The routine checks a byte value in a register to determine the execution path, calling different helper routines for disk transfer configuration or structure offset updates based on the value, and returns a combined result.
- `conditional_min_store`: The routine checks a byte value in a register.
- `init_disk_transfer`: The routine calls another routine to configure DMA registers for a disk operation.
- `allocate_string_buffer_entry`: Iterates through a global table of structures to find an entry with a zeroed status field.
- `parse_string_segment_2`: Validates a string pointer and parses characters against a table.
- `clear_offset_from_a4`: The routine calls another routine and stores zero at an offset of 10 from the address in register A4.
- `store_min_value_to_offset`: The routine calls another routine, compares a value from offset 14 of register A4 with a register value, and stores the minimum at offset 6 of register A4.
- `update_structure_offset`: Calculates a delta between two fields in a structure pointed to by A4 and conditionally updates the offset field.
- `update_structure_counters`: Increments a counter within a structure and performs conditional checks against other fields.
- `mask_low_bits`: The routine reads an input value from the D0 register and returns the bitwise AND of that value with a constant mask, effectively keeping only the lowest ten bits.
- `compute_price_index_from_a4`: The routine reads a 16-bit value derived from the A4 register, multiplies it by 18, and uses the result as an index into a table.
- `loop_and_sum_array_2`: Performs a complex loop that checks flags, decrements counters, and accumulates values from an array into a register.
- `check_structure_fields_2`: Checks two fields of a structure pointed to by a register.
- `loop_and_sum_array`: Performs a complex loop that checks flags, decrements counters, and accumulates values from an array into a register, handling conditional returns based on zero flags.
- `copy_memory_block_aligned`: The routine copies a block of memory from the address in A0 to the address in A5, handling alignment by copying words or bytes depending on the source and destination addresses.
- `copy_memory_block_2`: The routine copies a block of memory from the address in A0 to the address in A5 one byte at a time, using a loop counter in D0.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a5 one byte at a time, using a dbra loop counter, and returns.
- `check_structure_fields`: Checks two fields of a structure pointed to by a register.
- `process_audio_buffer_loop_2`: Iterates through a data buffer, copying and modifying word-sized values while checking loop counters and state flags.
- `process_audio_buffer_loop`: Iterates through a data buffer, copying and modifying word-sized values while checking loop counters and state flags.
- `sum_negate_array_words_2`: Clears a word at an offset in a structure, calls a routine to sum and negate an array of words, stores the result back into the structure, and performs additional checks and assignments based on register values.
- `tiny_move_rts`: Small routine of 2 instructions that copies 221 to d0, then returns.
- `check_flag_and_store`: Checks global flags and structure fields, then conditionally stores computed values into specific offsets within a structure pointed to by a register.
- `set_flag_in_bitfield`: The routine computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `decrement_and_loop_until_match`: The routine loads a short from a register, decrements it, and repeatedly calls another routine with the updated value until the value equals a specific constant, then returns.
- `wait_for_specific_value`: Calls a routine once, then enters a loop that repeatedly calls another routine, checking if a short register value equals a specific constant until the condition is met.
- `combine_registers_into_return_value`: The routine receives two 32-bit values in registers and returns a 64-bit value formed by concatenating the first value as the high part and the second value as the low part.
- `clear_bit_from_table`: The routine clears a single bit in a 32-bit word table indexed from a 32-bit value in D0.
- `check_flag_and_call`: Checks a global variable for zero and calls another routine conditionally.
- `copy_name_bytes`: The routine copies a null-terminated byte string from a source pointer to a destination buffer located at an offset from a base pointer, advancing both pointers each iteration.
- `copy_bytes_from_table`: Iterates over a table of pointers and copies a fixed-size block from a global table into a buffer via a pointer.
- `clear_zero_table`: The routine clears a block of memory by writing zeros to consecutive 32-bit words.
- `initialize_disk_transfer_block`: The routine initializes a memory block by writing specific constant values to offsets, loops until a call returns a specific status code, and stores results from subsequent calls into memory locations.
- `tiny_movea`: Small routine of 1 instruction that copies a6 to a0.
- `parse_string_segment`: Validates a string pointer, parses characters against a table, and returns specific constant codes based on null terminators or character matches.
- `check_string_null_flag`: Checks if a character pointer is null and a separate byte flag is not equal to a specific value.
- `parse_string_and_return_code`: The routine checks a character pointer for null termination, calls other routines to process data, and returns specific constant values based on conditions involving global variables and register contents.
- `parse_drive_identifier`: The routine checks a global condition and reads characters via another routine to verify a 'DF' sequence.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `empty_routine`: The routine consists of seven instructions that perform no observable work; it does not access memory, registers, or call other routines, and ends with a return.
- `copy_path_segment_to_buffer`: The routine finds a slash character in a string, copies characters to a buffer, and computes a return value based on the processed data.
- `check_extraout_flag`: Calls another routine and stores its result.
- `call_and_check_zero`: Calls another routine and checks if the returned integer is zero.
- `call_sum_negate_array`: Calls another routine to sum and negate an array of words.
- `sum_negate_array_words`: The routine adds 128 consecutive 16-bit words pointed to by a pointer in A0, accumulating the sum in a register.
- `store_sum_negate_result`: Clears a word at an offset in a structure, calls a routine to sum and negate an array of words, stores the result back into the structure, and calls another routine.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `recursive_call_stub`: The routine calls itself recursively without performing any other operations or modifying memory.
- `call_other_routine`: The routine calls another routine and then returns immediately.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `init_disk_dma_transfer`: Configures the DMA registers for a disk operation by calculating sector offsets and lengths.
- `process_disk_dma_transfer`: Configures DMA registers for a disk operation, calculating sector offsets and lengths.
- `loop_toggle_cia_bit_2`: Loops a fixed number of times, calling a routine that toggles a CIA port bit.
- `loop_toggle_cia_bit`: Loops a fixed number of times calling a routine that toggles a CIA port bit.
- `toggle_cia_port_bit`: The routine computes a bitmask from a register value and performs a bitwise operation on a global variable, effectively toggling or clearing a specific bit associated with CIA hardware port status.
- `retry_disk_read_operation`: The routine retries a disk read operation up to three times by configuring DMA registers, polling hardware flags, and validating input status.
- `configure_disk_dma_transfer`: The routine initializes DMA control registers and disk path/length parameters, then loops to poll a status flag while managing a counter.
- `loop_write_memory_block`: Loops a fixed number of times, writing a constant word to a memory address and calling other routines.
- `validate_input_with_retry`: The routine repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `validate_input_flags`: Polls global flags and structure fields in a loop, returning specific status codes based on bitwise checks and counter comparisons.
- `empty_stub_routine`: The routine consists of five instructions that perform no observable work and then return.
- `set_flag_and_increment_counter`: The routine sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: The routine clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: Calls another routine to obtain a 32-bit value.
- `interleave_word_bits`: The routine reads two consecutive words from a pointer in A0 and returns a value formed by taking the low bits of the second word and the low bits of the first word shifted left by one.
- `apply_interleave_xor_mask`: Calls a routine to interleave word bits and obtains a 32-bit value.
- `xor_sum_and_mask`: The routine iterates nine times, XORing consecutive words from a memory block pointed to by A5 into an accumulator.
- `compute_xor_checksum`: The routine iterates through a sequence of words pointed to by an input pointer, XORing each word into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `interleave_word_bits_2`: The routine interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_clear_flag_bitwise_mix`: Receives two arguments and loops a number of times derived from the first argument.
- `adjust_byte_flag`: The routine reads a byte from memory and checks a flag in the preceding byte.
- `clear_flag_byte`: The routine reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_registers`: The routine writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register.
- `poll_flag_and_set_timer`: Polls a byte in a structure for a specific bit flag.
- `set_adkcon_bit_mask`: Sets a global variable to a constant.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 255 into d1.
- `set_bit_in_global_flag`: The routine reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index.
- `set_flag_and_wait_timer`: Sets a global variable to a constant and waits for another routine to return a specific value.
- `update_table_entry`: Reads a word from a global table using an index, conditionally calls another routine if negative, loops while decrementing a counter, and writes the result back to the table.
- `wait_for_input_flag_clear`: The routine polls a global variable flag in a loop, decrementing a counter until the flag is cleared or the counter expires.
- `mask_input_and_wait_timer`: Masks a byte from a routine call and sets a global variable with the result.
- `update_ciab_prb_mask`: Calls a routine that masks an input byte using a global variable and structure data, then stores the result into a global variable, touching the CIAB_PRB register.
- `mask_input_byte`: The routine builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_flag_and_result`: Repeatedly calls a routine to set a CIA timer until it returns 1.
- `set_ciab_timer_if_clear`: Checks if the least-significant bit of a global variable is clear and an input register is not 1.
- `set_ciab_timer`: The routine writes three values to a global variable in sequence.
- `trigger_version_patch`: Calls a routine that detects the game version and patches specific code offsets based on magic values found in memory.
- `process_compressed_data_stream`: The routine validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `execute_version_specific_copy`: Checks a global version flag to determine execution path.
- `patch_and_verify_version`: Copies a block of code from a source routine to a target location and calls the patched routine.
- `store_constant_word`: The routine stores a specific 32-bit constant value into a memory location pointed to by a register.
- `call_version_patch_routine`: Invokes the routine responsible for detecting the game version and patching code offsets based on magic values.
- `patch_version_code`: Calls a routine to detect the game version and patch specific code offsets with new instructions or pointers based on that detection.
- `clear_global_pointers`: Sets two global pointer variables to zero.
- `allocate_memory_block`: Traverses a linked list of memory blocks to find one with sufficient free space, splits it if necessary, and returns a pointer to the allocated region.
- `lookup_string_entry`: Iterates through a linked list of entries to find one where the stored string matches an input string, returning the associated value or an error code.
- `copy_memory_block_4`: Copies a block of memory from a source pointer to a destination pointer.
- `allocate_and_copy_string`: Allocates a memory block from a linked list.
- `return_constant_one`: Returns the integer value 1 in the data register.
- `return_constant_minus_one`: Returns the integer value -1 (255) in the data register.
- `initialize_string_block`: Allocates a memory block and copies a null-terminated string into it at a specific offset, followed by copying additional data bytes into the allocated structure.
- `decode_lzw_stream`: Decompresses a data stream using an LZW algorithm by reading bits from a source buffer and expanding codes into a destination buffer.
- `decode_compressed_chunk_2`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `decode_compressed_chunk`: The routine validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `decode_huffman_data`: Processes a sequence of words using a stack-based algorithm.
- `handle_keyboard_interrupt`: Processes keyboard input by checking global flags and hardware registers, updating state variables, and managing a queue of input events.
- `process_interrupt_handler`: This routine appears to be a data structure or jump table entry rather than executable code, as indicated by bad instruction warnings and references to global variables and other routines within its body.
- `update_game_state_flags`: Checks a global flag and a counter limit to update a state variable and reset the flag.
- `handle_interrupt_state`: Updates global state variables and counters based on hardware registers and flags, potentially swapping memory blocks and synchronizing indices.
- `initialize_dma_channels`: Configures DMA channels by setting control registers, clearing memory buffers, and waiting for hardware status flags to indicate readiness.
- `calculate_table_offsets`: Computes multiple offset values based on a global variable and a hardware register, storing the results into a series of memory locations in a loop.
- `swap_memory_blocks`: Conditionally exchanges two blocks of memory bytes based on a global variable check, copying data between specific RAM addresses in either direction.
- `wait_and_clear_interrupts`: Waits for a vertical blank condition by polling a global variable, then clears interrupt requests and disables DMA and interrupts by writing to hardware registers.
- `loop_forever`: Performs an infinite loop with no other operations.
- `wait_for_input_and_hang`: The routine performs initialization calls and polls a global variable until it changes state, then sets a specific global value to four before entering an infinite loop.
- `copy_and_delay`: Copies a word from a source pointer to a destination pointer and executes a busy-wait loop based on an input value multiplied by ten.
- `read_joystick_one`: Reads joystick port one data and CIAA port A registers to determine button states and movement, processing the inputs into boolean flags.
- `read_mouse_port`: Reads mouse port data and CIAA port A registers to determine button states and movement, processing the inputs into boolean flags.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `update_palette_colors_2`: Calculates color values from lookup tables and updates hardware palette registers.
- `update_palette_colors`: Calculates color values from a lookup table and updates the hardware palette registers with the new color data.
- `update_counter_value`: Adjusts a signed global counter by subtracting a value, clamping it to specific limits, and calling another routine if the counter is not at its sentinel value.
- `decrement_palette_timer`: Decrements a global counter variable, clamping it to zero if it becomes negative, and sets a flag to zero when the counter reaches a specific sentinel value.
- `increment_palette_timer`: Increments a global counter variable, clamping it to zero if it is negative, and sets a flag to one when the counter exceeds a threshold value.
- `wait_for_flag_clear`: Checks a global variable against a specific constant value.
- `wait_for_flag_change`: Polls a global flag variable until it changes state, initializing another global variable if necessary, and waits for the flag to clear.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `configure_interrupt_state`: Updates global variables and writes to the INTENA hardware register to enable or disable interrupts based on the bit 15 setting.
- `clear_interrupt_request`: Clears an interrupt request flag in the hardware registers and returns a value from the stack if the flag was previously set.
- `set_interrupt_flag`: Sets a global variable to a constant and updates state flags.
- `toggle_state_flag`: Toggles a global state flag and returns a value from the stack depending on the new state of the flag.
- `return_stack_value`: Returns a 64-bit value located at the top of the stack frame without performing any other operations.
- `empty_stub_3`: An empty routine that returns immediately to the caller without executing any instructions.
- `decode_copper_list_bytes`: Iterates through a byte array, performing bitwise manipulations to extract and combine values, then stores the resulting 16-bit words into a destination buffer.
- `configure_blitter_dma`: Waits for a global status flag to clear, then writes specific values to memory-mapped registers associated with the Blitter and DMA control.
- `enter_infinite_loop`: The routine executes a tight infinite loop without performing any operations or returning control.
- `find_string_terminator`: Iterates through a byte array pointed to by a register until a null terminator is found, advancing the pointer with each step.
- `process_string_offset`: Iterates through a string, skipping spaces, and calculates an offset using a lookup table based on character values, returning if the offset exceeds a limit.
- `calculate_string_position`: Similar to the previous routine, it processes a string by skipping spaces and computing a position value using a table lookup, returning on limit exceedance.
- `setup_player_sprite_blitter`: Calculates sprite geometry and animation offsets for a player entity, then configures the Amiga Blitter and DMA channels to render the sprite to the display buffer.
- `setup_player_animation`: Calculates animation parameters and memory offsets for a player entity based on its state and direction.
- `configure_copper_blitter_dma`: The routine configures DMA channels and sets up Blitter registers for a block transfer operation.
- `return_d0_byte`: Returns the value currently held in the lower byte of register D0 without modification.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `clear_global_bytes`: Sets several specific global byte variables to zero and one global byte to 65535, then returns the value in D0.
- `initialize_sound_state`: The routine sets several global variables associated with sound data to zero or specific constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_state`: Sets multiple global variables to zero or one, calls several initialization routines, and enters a loop that processes input sequences while checking flags.
- `modify_buffer_and_check`: Clears a bit in a global variable based on an input, ORs specific bytes into a buffer pointed to by A0, adds a value from D7 to a short in the buffer, updates a global pointer, and conditionally calls another routine if the pointer exceeds a limit.
- `modify_buffer_tail_and_check`: Adds a value from D7 to a short at the start of a buffer pointed to by A0, ORs specific bytes into subsequent buffer locations, updates a global pointer, and conditionally calls another routine if the pointer exceeds a limit.
- `loop_while_flag_set`: Checks a global boolean flag; if set, it iterates through a table of integers, calling a routine for each entry until a value of -1 is encountered.
- `loop_on_error_call`: Calls another routine and, if the result is -1, enters a loop that repeatedly calls another routine until a boolean flag becomes false.
- `scan_table_for_minus_one`: Checks a global flag and, if set, iterates through a table of integers until it finds a value of -1.
- `check_and_loop_on_error`: Calls another routine and checks if it returns -1.
- `process_input_sequence`: Executes a series of routine calls while checking a global variable flag.
- `call_sequence_with_check`: Calls another routine multiple times, passing a register value, and checks a global variable after each call.
- `call_sequence_return_value`: Similar to R1, but returns a value.
- `unreached_call_sequence`: An unreached routine that calls another routine multiple times, checks a global variable, and returns a value.
- `call_and_check_error`: Calls a routine passing a register value, then checks a global variable; if the variable is zero, it enters an infinite loop, otherwise it returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_3`: Copies a block of bytes from a source pointer to a destination pointer, unless a specific magic signature is detected in a global variable, in which case it delegates to another routine.
- `configure_blitter_dma_2`: Waits for a global flag bit to clear, then writes configuration values to a block of memory representing Blitter registers, including source/destination pointers, sizes, and control words.
- `convert_integer_to_string`: Takes a 16-bit integer value and converts it into a null-terminated ASCII string representation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_pointers`: Sets four specific global variables to fixed constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_routine_2`: This routine is empty and contains only a return instruction.
- `return_immediately`: The routine contains no instructions and returns immediately to the caller.
- `return_immediately_2`: The routine is empty and returns immediately without performing any operations or modifying data.

### SWOS2#unpacked (raw code file, load address unknown)

- `start_code_region`: Marks the beginning of a raw code segment.

### SWOS2.REL#unpacked (raw code file, load address unknown)

- `null_routine`: This routine is empty and contains no valid instructions.
