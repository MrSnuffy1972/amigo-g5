# Game 2039 (sensible world of soccer (1994)(renegade)[cr hlm][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02039). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1832992 top-level bytes in 286 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 27275 raw-word lines |
| Functions named and described | pass, 208 of 208 named (0 author symbols), 208 of 208 described (12 by rule, 131 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2532 of 2533 (99.96%) |
| Recovered without load address (counted in proof 1, flagged) | 781548 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 32631 of 32649 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 60287 |
| code | 18542 |
| data | 1577221 |
| padding | 37166 |
| unused | 139776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:type | 2 | 117 | 2 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:ssboot | 3 | 160 | 5 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:hoodlum | 2 | 239 | 8 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:MB | 1 | 3 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 890 | 37 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 823 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 529 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2400 | 121 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown) | 1 | 13800 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown) | 1 | 13646 | 1 |

## Cases handled

- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36038 -> 55200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.062: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9589 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.064: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6953 -> 16418 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.029: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6454 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.030: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6424 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (150060 -> 334650 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.031: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7190 -> 15734 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.032: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3291 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12283 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.034: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6394 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.035: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14003 -> 30098 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.036: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4973 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.037: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5545 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.038: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6428 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.039: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5798 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/SWCPICH4.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12206 -> 39832 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/DISK2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8324 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/SWCPICH2.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12574 -> 39320 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/SWCPICH5.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13649 -> 44312 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/LOADER1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8295 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:GRAFS/SWCPICH3.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14378 -> 42904 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.040: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6282 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:SOUND/MISS3C: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12713 -> 12788 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:SOUND/MENMUSIC.MOD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.041: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5767 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.042: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4170 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.043: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12660 -> 28730 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.044: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15401 -> 34886 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.045: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5047 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.046: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8846 -> 22574 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.048: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5938 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.049: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5387 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (150951 -> 337114 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.050: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3823 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.051: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12102 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.055: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8972 -> 19154 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35672 -> 54584 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:TEAM.060: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11486 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6251 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6338 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2928 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4864 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4593 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2143 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6158 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/SOCPIC.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6807 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4608 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/MENUS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5267 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/SWCPICH1.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13570 -> 45848 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/MENUBG2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (18119 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4139 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:GRAFS/WINSCRN.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11386 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:SOUND/MENMUSIC.MOD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.007: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4064 -> 8894 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.062: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9589 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.008: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (29483 -> 63614 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.064: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6953 -> 16418 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.065: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5344 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.066: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3223 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.067: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3361 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.010: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4477 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.011: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3349 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.069: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7178 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.012: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4529 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.013: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15787 -> 32150 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.014: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13856 -> 28046 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.015: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6889 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.016: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5846 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.071: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7217 -> 17786 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.017: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3041 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.072: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9064 -> 19838 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.018: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4569 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.019: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4874 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.073: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2814 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.075: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1916 -> 4106 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.076: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5555 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.020: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15031 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.077: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4884 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.021: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3208 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.078: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3840 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.022: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3841 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.079: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2409 -> 5474 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.023: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3640 -> 7526 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.024: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3855 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.025: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12341 -> 24626 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.026: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4977 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.080: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (17086 -> 32834 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.027: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5228 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.081: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10788 -> 23258 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.028: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7095 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.082: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3145 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.029: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6454 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.083: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4341 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/EUROCUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5898 -> 10944 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.084: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10034 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.085: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1488 -> 2738 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/POOLPLYR.DAT: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (26365 -> 52200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.030: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6424 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.031: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7190 -> 15734 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.032: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3291 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12283 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.034: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6394 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.035: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14003 -> 30098 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.036: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4973 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.037: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5545 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.038: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6428 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.039: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5798 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.040: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6282 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.041: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5767 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.042: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4170 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.043: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12660 -> 28730 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.044: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15401 -> 34886 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.045: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5047 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.046: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8846 -> 22574 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.048: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5938 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.049: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5387 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/EUROCWC.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11491 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.050: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3823 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/UEFACUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11462 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.051: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12102 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.055: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8972 -> 19154 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.000: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5268 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.001: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4208 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.002: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7882 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.003: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5486 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.004: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5713 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.005: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5057 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.060: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11486 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 2 of 2).adf:DATA/TEAM.006: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5064 -> 10946 bytes)
- runtime segment an address: 823 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 281 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 447 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 572 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:hoodlum: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 671764 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_custom_chip_registers` (runtime segment an address): Opens a system library and, if that succeeds, configures the display window, data fetch, copper list, blitter and interrupt registers. Kind: disk hardware/track check. Pass path: the branch inside the if (_DAT_00081db2 != 0) block.
- `clear_cia_port_bit` (runtime segment an address (play-rs-m1500)): It derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit. Kind: checksum/integrity trap. Pass path: the code continues to the next instruction (addx.b d2, d2) assuming the bit manipulation was a valid state change or part of a larger calculation.
- `configure_disk_dma_transfer` (runtime segment an address (play-rs-m1500)): Initializes DMA control registers and disk path and length parameters, then loops to poll a status flag while managing a counter. Kind: track/disk verification loop. Pass path: the loop continues (sVar2 decrementing) if the check returns false, or breaks if the check returns true/fails.
- `configure_dma_registers` (runtime segment an address (play-rs-m1500)): It writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register. Kind: disk hardware/track check. Pass path: bne.b an address.
- `poll_flag_and_set_timer` (runtime segment an address (play-rs-m1500)): Polls a byte in a structure for a specific bit flag. Kind: disk hardware status check. Pass path: the loop continues (bVar2 is true), eventually leading to the 'return 0' path if the flag is set correctly.
- `set_bit_in_global_flag_2` (runtime segment an address (play-rs-m1500)): It reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index. Kind: checksum or integrity trap. Pass path: the branch following the call (the code continues normally if the bit is set/condition is met).
- `set_bit_in_global_flag` (runtime segment an address (play-rs-m1500)): It reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index. Kind: bitmask flag setter/check. Pass path: the branch at an address (the instruction following the call site).
- `wait_for_bit_clearance` (runtime segment an address (play-rs-m1500)): Sets a global variable to a constant value, calls another routine, and then loops until a specific bit in that global variable is cleared and another routine returns a specific value. Kind: disk hardware/sync check. Pass path: the loop continues until (DAT_00bfde00 & 1) == 0, meaning the bit clears, allowing the loop to proceed to the next iteration of the iVar1 check.
- `update_input_mask` (runtime segment an address (play-rs-m1500)): Modifies a global variable by masking a byte from a structure and setting a specific bit. Kind: input mask/manual check. Pass path: extraout_D1b >= 0.
- `mask_input_byte_2` (runtime segment an address (play-rs-m1500)): It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and returns the result. Kind: disk hardware/track check. Pass path: the instruction immediately following the call (e.g., moveq #an address, d0 or moveq #an address, d0).
- `mask_input_byte` (runtime segment an address (play-rs-m1500)): It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set. Kind: bitmask/status check. Pass path: the branch where the bit is NOT set (bclr.b #an address, d0).

## Functions


### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen long words from a record offset into a buffer whose address is taken from the stack, then calls a subroutine.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:ssboot

- `entry_point_call_library`: Program entry point that calls a library function through the library base, makes a few further calls, and clears a stored pointer before returning.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decode_packed_record_stream`: Decodes a packed stream of records: reads a length code, copies runs of words or bytes into the destination through a subroutine, and adds decoded offsets into a table.
- `select_half_or_input`: Returns half of the masked word when a mode value equals one, otherwise returns the input word unchanged.
- `mask_with_table_word`: Looks up a word in a fixed table using an index doubled into a byte offset, and returns it ANDed with the caller's word.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:hoodlum

- `allocate_and_locate_packed_block`: Program entry that allocates memory and then searches a linked list for a packed-data header tag to locate the block to unpack.
- `extract_flag_bit_from_table`: Scans a table of word-pair masks for the matching entry, reads a shift count from that entry, and extracts one bit from the masked word, calling a subroutine when a borrow condition is met.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Ands a word from one register with a word from another, and when a byte comparison between two more registers shows the first is lower, it calls another routine and returns that routine's word instead.
- `return_empty_routine`: Contains nothing but a return, so it changes no memory or registers.
- `build_bit_mask_table`: Reads a count and then that many bytes from a callee into a buffer.
- `accumulate_pending_list_entries`: When the first word matches a fixed tag, it clears the tag and walks a chain of entries, zeroing each entry's fields and adding a shift-derived offset value into a target table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:MB

- `wait_for_port_bit_clear`: Spins while bit six of a status value is set, then returns.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_custom_chip_registers`: Opens a system library and, if that succeeds, configures the display window, data fetch, copper list, blitter and interrupt registers.
- `fill_four_stepped_pairs`: Writes four pairs of 16-bit halves of a 32-bit value into a table, adding 5000 to the value after each pair.
- `fill_eight_stepped_pairs`: Writes eight pairs of 16-bit halves of a 32-bit value into a table, adding 264 to the value after each pair.
- `run_sequenced_calls_return_third`: Calls four routines in order and returns the result of the third call.
- `advance_frame_timers_and_chips`: Once per frame it reads the DMA status and waits for the busy flag to clear, then updates frame counters and table indices using thresholds of 700 and 699.
- `reset_blitter_and_counters`: Waits for the blitter to go idle, then writes initial values into a blitter control block and zeroes a 24-word table of globals.
- `step_wrapping_counters_and_reload`: Advances a set of counters that wrap once they pass 29 and carry into a neighbouring counter, calling a helper on each step.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_eight_words_with_stride`: Computes a destination pointer from a counter and an index, then copies eight words from the source buffer, advancing the destination by 220 bytes between words.
- `advance_position_when_flagged`: If a flag byte in a table is set, it calls a helper, advances a column index that wraps past 29, and adds nine to a row index on each wrap.
- `scatter_eight_bytes_with_stride`: Writes eight bytes from a source buffer into a destination table at a 40-byte step, after computing the destination from two counters.
- `configure_blitter_dma_priority`: Waits for the blitter to become idle, calls helpers, saves three words of a counter set, and then writes the DMA control register twice to clear and set the blitter priority and DMA enable bits.
- `update_two_arrays_by_coefficients`: Loops over a count of entries and, for each, adds a fixed-point product of a coefficient from a third array to two arrays, scaling the product down by 1024.
- `flag_triangle_orientation_records`: Walks a list of three-index records, computing a signed cross-product test from two coordinate tables for each record.
- `clear_blitter_control_fields`: Waits for the blitter to be idle, then zeroes four control words, stores a long value into one control field and writes fixed values into two more control words.
- `advance_three_counters_with_wrap`: Adds three increments to three counters and wraps each counter back by 720 when it passes 719.
- `scale_vertex_table_by_factor`: Loads a count from a data pointer and, for each entry, multiplies two stored coordinates by a scale value into two tables and copies a third value into a third table.
- `scale_vertex_table_all_axes`: Loads a count from a data pointer and, for each entry, multiplies three stored coordinates by a scale value and writes each result into its own table.
- `start_blit_with_fixed_addresses`: Waits for the blitter to become idle, loads two fixed addresses into the blitter pointer fields, clears one field and writes control and size values to start the operation.
- `walk_entries_and_call_helpers`: Waits for the blitter to be idle and sets up control fields, then walks a table of entries.
- `setup_blitter_line_draw`: Computes the horizontal and vertical extents of a line from two endpoints, then looks up an octant code and writes error and length values into the blitter registers.
- `write_blitter_block_odd_offsets`: Waits for the blitter to be idle, then writes a fixed set of values into a control block at odd offsets, including two pointer values and a count.
- `write_blitter_block_even_offsets`: Waits for the blitter to be idle, then writes a fixed set of values into a control block at even offsets, including two pointer values and a size field.
- `call_four_routines_in_order`: Calls four routines in sequence and does no work of its own.
- `start_blit_with_source_pointers`: Waits for the blitter to be idle, stores two passed-in pointer values into the blitter address registers, sets a count and writes control values to begin the operation.
- `run_four_routines_in_sequence`: Calls four routines one after another and does no work of its own.
- `chain_four_routine_calls`: Calls four routines in order, leaving the calls to do all the work.
- `pack_bits_into_word`: Takes a bit from the low end of a value, combines it with an offset byte and the shifted value into a packed 32-bit word, stores that word to memory and returns the value shifted right by one.
- `read_next_table_entry_wrapping`: Reads the next six-byte entry from a table and stores its first two words in globals.
- `copy_entries_and_update_third`: Copies 17 three-word entries into three tables and updates each source third word in place by adding twelve and its own high byte.
- `store_two_pointers_and_constants`: Stores the low and high halves of a 32-bit value, and of that value plus 40, into four 16-bit locations.
- `start_blit_with_shared_value`: Waits for the blitter to be idle, clears several control fields and stores a shared value into a pointer field.
- `scale_two_arrays_by_coefficient`: Loops over 17 entries and, for each, adds a fixed-point product of a coefficient from a third array, scaled by 64, to two arrays in place.
- `set_bitmap_bits_for_points`: Iterates over 17 points, converting each scaled position into a bit index and setting one bit in a bitmap.
- `store_three_fixed_constants`: Stores the constants 710, 670 and 10 into three fixed global locations.
- `rotate_points_by_two_angles`: Looks up coefficients for two angles from tables and rotates each stored point in fixed-point arithmetic, writing the rotated coordinates back in place.
- `tiny_ori_2`: Small routine of 1 instruction that merges 44 into (a6).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_audio_channel_state`: Fills a 32-entry table with values stepping by 30, finds the largest byte in a 128-byte block and clears an area sized from it, then points 31 channel records into that area.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `reload_audio_channel_pointers`: Advances a counter by sixteen steps, and when it wraps to zero, waits and copies eight audio channel location and length values from stored table entries into the custom chip registers.
- `process_tracker_effect_command`: Calls four helper routines, then reads a command from the channel row and dispatches on its nibble.
- `update_channel_playback_state`: Advances one audio channel from its pattern row, looking up a sample pointer and length from tables, updating volume and period fields, and calling helpers.
- `apply_channel_effect_command`: Performs a channel's effect for one tick by reading its fields, calling helpers and dispatching on the command nibble.
- `find_channel_target_step`: Searches a per-channel table for the first entry above the channel's current value, optionally stepping back one entry, and stores that entry as the target.
- `update_audio_parameter`: This routine modifies a value in a structure based on a signed increment or decrement and performs a table lookup to update a destination word.
- `compute_phase_modulated_offset`: Looks up a waveform table value at the channel's phase, scales it by a depth nibble, and writes the result plus the channel's base value to the output.
- `dispatch_channel_effect_command`: Dispatches a row command by its command nibble.
- `store_low_nibble_field`: Stores the low four bits of a source byte into a channel field at a fixed offset.
- `advance_wrapping_pointer_on_overflow`: When the upper nibble of a parameter selects a non-zero table entry, it adds that entry to a counter byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_relocate_resource`: Loads a file from disk, processes its string content, and performs a relocation pass by adding offsets to memory blocks to adjust pointers within the loaded data.
- `apply_memory_offset_to_block`: Iterates through a block of memory defined by a global variable and adds a constant byte offset to each word in that block.
- `copy_memory_block`: Copies a specified number of bytes from one memory location to another.
- `copy_memory_backward`: Copies a block of memory from a source to a destination by iterating backwards through the specified length.
- `write_message_to_dos_output`: Opens the DOS library through a library call, measures a zero-terminated message, then calls two DOS routines to write it to the output stream.
- `probe_memory_regions_by_type`: Probes memory regions by testing up to 256 sample addresses in each through the exec memory-type call, and records region presence and type in two bit masks.
- `handle_file_stream`: Manages file operations by calling library functions to open, read, and seek through data, including processing compressed chunks.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call_2`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine_2`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_cia_bit`: The routine loops until a specific bit in a global variable is cleared.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_patterns`: Fills multiple large memory ranges with specific repeating patterns and updates several global variables.
- `setup_memory_patterns`: Fills large memory ranges with specific repeating patterns and prepares global variables for subsequent execution.
- `initialize_hardware_state`: Initializes several memory tables with constant values and configures hardware registers including DMA and interrupt settings.
- `dispatch_function_pointer`: The routine calculates an address using a register offset and calls the function located at that address.
- `apply_bitmask_to_table`: Stores several values into global variables and then iterates through a four-word table, clearing entries where corresponding bits in a mask are not set.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `search_table_entry`: Iterates through a table of values, updating a global index and checking a condition until a match is found or a counter expires.
- `evaluate_status_conditions`: Checks multiple global variables for zero values through a series of conditional branches.
- `load_graphics_and_parse_input`: Loads a graphics resource from a text string and processes input data based on a checked value.
- `check_resource_status`: Calls a routine and returns true if the result is zero, otherwise it calls another routine and returns false.
- `update_status_flags`: Updates global variables based on the status of other global variables and performs conditional assignments.
- `assign_global_values`: Stores two specific values into two global variables.
- `load_grafs_resource`: The routine references a text string for a grafs loader resource and calls an external function twice.
- `parse_input_type`: Checks a value and, depending on its content, calls other routines to return or process different types of data or pointers.
- `find_free_script_slot`: The routine scans a small table of script slots by stepping through entries.
- `clear_memory_at_offset`: Calls another routine and then stores a zero value at a specific offset from a pointer held in a register.
- `store_min_value_to_offset`: The routine calls another routine, compares a value from offset 14 of register A4 with a register value, and stores the minimum at offset 6 of register A4.
- `update_buffer_bounds_and_counter`: Calculates a new buffer or range size based on the difference between two pointer-relative offsets and updates a global counter or limit.
- `mask_low_bits`: The routine reads an input value from the D0 register and returns the bitwise AND of that value with a constant mask, effectively keeping only the lowest ten bits.
- `compute_price_index_from_a4`: The routine reads a 16-bit value derived from the A4 register, multiplies it by 18, and uses the result as an index into a table.
- `process_tile_row_accumulation`: Iterates through a tile row to calculate a sum while copying data.
- `accumulate_tile_row_sum`: Repeatedly increments a counter and uses the returned value to drive a loop that copies a tile row while accumulating a sum in a local variable, returning a combined 64-bit value.
- `copy_tile_row`: The routine copies data from a source pointer to a destination pointer.
- `compare_structure_fields`: Compares two fields of a structure pointed to by a register and, if they match and a flag field is non-zero, obtains an identifier from another routine.
- `populate_event_log_buffer`: Iterates through a loop to populate a memory buffer with structured data, including counters and values retrieved from other routines.
- `handle_state_transition`: Performs a series of conditional checks on global variables and structure members to update a pointer and a global value.
- `set_flag_in_bitfield`: The routine computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `decrement_and_loop_until_match`: The routine loads a short from a register, decrements it, and repeatedly calls another routine with the updated value until the value equals a specific constant, then returns.
- `wait_for_specific_value`: Calls a routine once, then enters a loop that repeatedly calls another routine, checking if a short register value equals a specific constant until the condition is met.
- `combine_registers_into_return_value`: The routine receives two 32-bit values in registers and returns a 64-bit value formed by concatenating the first value as the high part and the second value as the low part.
- `clear_bit_from_table`: The routine clears a single bit in a 32-bit word table indexed from a 32-bit value in D0.
- `check_zero_and_return_constant`: Checks if a routine returns zero, and if so, calls another routine and returns either that value or a constant based on the state of the zero flag.
- `copy_name_bytes`: The routine copies a null-terminated byte string from a source pointer to a destination buffer located at an offset from a base pointer, advancing both pointers each iteration.
- `copy_bytes_from_table`: Iterates over a table of pointers and copies a fixed-size block from a global table into a buffer via a pointer.
- `clear_zero_table`: The routine clears a block of memory by writing zeros to consecutive 32-bit words.
- `initialize_disk_transfer_block`: The routine initializes a memory block by writing specific constant values to offsets, loops until a call returns a specific status code, and stores results from subsequent calls into memory locations.
- `tiny_movea`: Small routine of 1 instruction that copies a6 to a0.
- `parse_data_structure_fields`: Checks a pointer for null, repeatedly calls another routine to obtain values, stores parts of them in global and static variables, and loops over an array comparing bytes with table values.
- `parse_drive_identifier`: The routine checks a global condition and reads characters via another routine to verify a 'DF' sequence.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `empty_routine`: The routine is empty and performs no observable work.
- `copy_path_segment_to_buffer`: It scans a null-terminated string from a base pointer, advancing until it reaches a null terminator or a slash character.
- `check_extraout_flag`: Calls another routine and stores its result, then tests a flag and two fields of a pointer passed in a register.
- `validate_checksum_result`: Calls a routine to obtain a value and checks if the result is zero.
- `negate_and_check_sum`: The routine calls another routine to obtain a value and returns either that value or a constant based on the state of the zero flag.
- `sum_negate_array_words`: It adds 128 consecutive 16-bit words pointed to by a pointer in a register, accumulating the sum and returning the negated sum in the high 32 bits of the result.
- `store_value_to_offset`: It stores a 32-bit value into a memory location at a fixed offset from a pointer.
- `return_address_pointer`: The routine returns a single value, likely an address or pointer.
- `clear_memory_offset`: Writes zero to a longword at an offset from a pointer held in a register.
- `update_pointer_field`: Clears a field at a specific offset within a structure pointed to by a register, then updates that same field with the return value of a subsequent routine call.
- `update_pointer_offset_value`: Clears a value at a specific offset within a structure pointed to by a register, then updates that same offset with the return value of another routine.
- `store_sum_to_offset`: The routine stores zero at a fixed offset from a pointer, calls another routine to obtain a value, stores that value at the same offset, and then calls another routine.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `recursive_call_stub`: The routine calls itself recursively without performing any other operations or modifying memory.
- `return_void`: This routine is empty and performs no operations before returning to the caller.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `combine_divide_values`: It calls another routine that divides a register value, accumulating a product while looping, and combines the resulting quotient and remainder into a single value.
- `process_disk_dma_transfer`: Configures DMA registers for a disk operation, calculating sector offsets and lengths.
- `loop_toggle_cia_bit_2`: Loops a fixed number of times, calling a routine that toggles a CIA port bit and accumulating the results in a register.
- `loop_toggle_cia_bit`: It loops thirty times, each pass clearing a CIA port bit and recording the result.
- `clear_cia_port_bit`: It derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit.
- `validate_disk_transfer_state`: Performs a series of conditional checks on a structure to validate system state and disk-related flags.
- `configure_disk_dma_transfer`: Initializes DMA control registers and disk path and length parameters, then loops to poll a status flag while managing a counter.
- `loop_write_memory_block`: Loops a fixed number of times, writing a constant word to a memory address and calling other routines.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `validate_complex_state`: Performs a series of conditional checks against structure fields and bitwise flags to validate a specific system state.
- `return_immediately`: This routine performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `apply_interleave_xor_mask`: It calls a routine to interleave word bits and obtains a 32-bit value.
- `xor_sequence_and_mask`: It XORs a sequence of ten words starting from a pointer into an accumulator and returns the result masked to retain alternating bits.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `interleave_word_bits_2`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_clear_flag_bitwise_mix`: Receives two arguments and loops a number of times derived from the first argument.
- `modify_byte_via_flag`: It calls another routine twice and then modifies a byte based on the state of a bit in the preceding byte.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_registers`: It writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register.
- `poll_flag_and_set_timer`: Polls a byte in a structure for a specific bit flag.
- `set_bit_in_global_flag_2`: It reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 255 into d1.
- `set_bit_in_global_flag`: It reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index.
- `wait_for_bit_clearance`: Sets a global variable to a constant value, calls another routine, and then loops until a specific bit in that global variable is cleared and another routine returns a specific value.
- `update_table_entry`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table.
- `wait_for_input_flag_clear`: Polls a global variable flag in a loop, decrementing a counter until the flag is cleared or the counter expires, while also accessing the CIAA_PRA register.
- `update_input_mask`: Modifies a global variable by masking a byte from a structure and setting a specific bit.
- `mask_input_byte_2`: It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and returns the result.
- `mask_input_byte`: It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_flag_and_result`: It repeatedly sets a CIA timer until the routine returns one.
- `set_ciab_timer_2`: It writes three constant values to a global variable in sequence if a specific bit in a global variable is not set and a register value is not one.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
- `dispatch_function_pointer_2`: Calculates an address using a register offset and calls the function located at that address.
- `copy_data_and_call`: Copies a block of data from a global variable to another location using a loop, clears a global variable, and calls another routine.
- `call_version_patch_routine`: Acts as a wrapper to trigger the routine responsible for detecting the game version and patching code offsets.
- `patch_version_code`: Calls a routine to detect the game version and patch specific code offsets with new instructions or pointers.
- `process_bitfield_nodes`: Iterates through a bitfield to identify set bits and manages a linked list of nodes based on those positions.
- `manage_linked_list_nodes`: Manipulates a linked list structure by inserting, removing, or updating nodes based on provided values and a global counter.
- `handle_keyboard_interrupt`: Processes keyboard input by checking global flags and hardware registers, updating state variables, and managing a queue of input events.
- `update_system_state_2`: Checks a global variable against a range to trigger a sequence of subroutines and updates various global counters and table entries.
- `update_table_values`: It copies a 32-bit global variable into several memory locations and then populates a series of table entries with calculated values based on that variable.
- `assign_pointer_values`: It checks a global variable against a constant and then performs a series of assignments to a global variable using either its own address or specific constant values.
- `wait_and_clear_interrupts`: Waits for a vertical blank condition by polling a global variable, then clears interrupt requests and disables DMA and interrupts by writing to hardware registers.
- `execute_infinite_loop`: The routine is empty and consists only of an infinite loop.
- `process_input_loop`: Executes a series of subroutines and enters an infinite loop after setting a global variable to a constant value.
- `store_and_decrement`: It stores a 32-bit value from a pointer into another memory location and then decrements a value in a register through a loop.
- `read_joystick_one`: Reads joystick port one data and CIAA port A registers to determine button states and movement, processing the inputs into boolean flags.
- `read_mouse_port`: Reads mouse port data and CIAA port A registers to determine button states and movement, processing the inputs into boolean flags.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `apply_bitmask_to_memory`: Performs a bitwise AND operation between a value from a table and a sequence of values from a source pointer, using a variable shift amount.
- `fill_memory_with_pattern`: Writes a repeating pattern of bytes to a destination memory area using a loop.
- `scale_table_index_by_factor`: The routine reads a byte from a pointer, masks the low four bits, multiplies the result by an unsigned word from D2, and returns a 32-bit value formed by the high 16 bits of the product and the product shifted right by seven bits.
- `decrement_palette_timer`: Decrements a global counter variable, clamping it to zero if it becomes negative, and sets a flag to zero when the counter reaches a specific sentinel value.
- `increment_palette_timer`: Increments a global counter variable, clamping it to zero if it is negative, and sets a flag to one when the counter exceeds a threshold value.
- `wait_for_flag_clear`: Checks a global variable against a specific constant value.
- `wait_for_flag_change`: Polls a global flag variable until it changes state, initializing another global variable if necessary, and waits for the flag to clear.
- `configure_interrupt_state`: Updates global variables and writes to the INTENA hardware register to enable or disable interrupts based on the bit 15 setting.
- `update_system_state`: Sets a global variable and performs a series of function calls and a loop that checks for matching byte values in a memory range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `verify_byte_sequence`: Sets a global variable and compares a sequence of bytes in memory against a specific bitmask pattern.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_null_terminated_string`: Copies a string from a source pointer to a destination pointer until a null terminator is encountered.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown)

- `inspect_unrecovered_swos2_rel_region`: A raw code region of 13800 instructions, starting at its entry point.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown)

- `inspect_unrecovered_swos_region`: A raw code region of six instructions, starting at its entry point.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown)

- `null_routine_placeholder`: This routine is empty and contains no instructions.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown)

- `process_large_block`: This routine contains a large number of instructions but the control flow is interrupted by invalid data.
