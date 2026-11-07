# Game 2037 (sensible world of soccer (1994)(renegade)[cr hlm][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02037). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1813088 top-level bytes in 224 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 27271 raw-word lines |
| Functions named and described | pass, 75 of 75 named (0 author symbols), 75 of 75 described (6 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1592 of 1592 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 781548 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 29889 of 29895 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 60287 |
| code | 8604 |
| data | 1337101 |
| padding | 28728 |
| unused | 378368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:ssboot | 3 | 160 | 5 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum | 2 | 239 | 8 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 892 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 823 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 313 | 7 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown) | 1 | 13800 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown) | 1 | 13646 | 1 |

## Cases handled

- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36038 -> 55200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (150060 -> 334650 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH4.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12206 -> 39832 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/DISK2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8324 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH2.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12574 -> 39320 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH5.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13649 -> 44312 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/LOADER1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8295 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH3.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14378 -> 42904 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:SOUND/MISS3C: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12713 -> 12788 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:SOUND/MENMUSIC.MOD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (150951 -> 337114 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (35672 -> 54584 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6251 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6338 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2928 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4864 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4593 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2143 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6158 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/SOCPIC.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6807 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4608 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5267 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/SWCPICH1.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13570 -> 45848 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUBG2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (18119 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4139 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/WINSCRN.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11386 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:SOUND/MENMUSIC.MOD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.007: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4064 -> 8894 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.062: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9589 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.008: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (29483 -> 63614 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.064: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6953 -> 16418 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.065: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5344 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.066: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3223 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.067: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3361 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.010: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4477 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.011: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3349 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.069: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7178 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.012: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4529 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.013: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15787 -> 32150 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.014: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13856 -> 28046 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.015: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6889 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.016: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5846 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.071: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7217 -> 17786 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.017: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3041 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.072: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9064 -> 19838 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.018: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4569 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.019: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4874 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.073: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2814 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.075: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1916 -> 4106 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.076: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5555 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.020: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15031 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.077: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4884 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.021: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3208 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.078: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3840 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.022: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3841 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.079: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2409 -> 5474 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.023: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3640 -> 7526 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.024: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3855 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.025: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12341 -> 24626 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.026: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4977 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.080: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (17086 -> 32834 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.027: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5228 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.081: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10788 -> 23258 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.028: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7095 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.082: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3145 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.029: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6454 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.083: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4341 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/EUROCUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5898 -> 10944 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.084: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10034 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.085: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1488 -> 2738 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/POOLPLYR.DAT: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (26365 -> 52200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.030: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6424 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.031: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7190 -> 15734 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.032: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3291 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.033: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12283 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.034: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6394 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.035: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14003 -> 30098 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.036: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4973 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.037: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5545 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.038: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6428 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.039: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5798 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.040: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6282 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.041: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5767 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.042: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4170 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.043: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12660 -> 28730 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.044: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (15401 -> 34886 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.045: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5047 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.046: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8846 -> 22574 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.048: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5938 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.049: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5387 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/EUROCWC.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11491 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.050: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3823 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/UEFACUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11462 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.051: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12102 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.055: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8972 -> 19154 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.000: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5268 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.001: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (4208 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.002: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7882 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.003: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5486 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.004: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5713 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.005: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5057 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.060: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11486 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.006: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5064 -> 10946 bytes)
- runtime segment an address: 797 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 273 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 129 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 671764 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_custom_chips_and_display` (runtime segment an address): Takes over the system, opens the graphics library, sets the display window, data fetch and bitplane registers, enables DMA and interrupts, and runs setup subroutines. Kind: disk hardware/sync check. Pass path: the branch where _DAT_00081d9a != 0 is taken.

## Functions


### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:ssboot

- `entry_point_call_library`: Program entry point that calls a library function through the library base, makes a few further calls, and clears a stored pointer before returning.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `decode_packed_record_stream`: Decodes a packed stream of records: reads a length code, copies runs of words or bytes into the destination through a subroutine, and adds decoded offsets into a table.
- `select_half_or_input`: Returns half of the masked word when a mode value equals one, otherwise returns the input word unchanged.
- `mask_with_table_word`: Looks up a word in a fixed table using an index doubled into a byte offset, and returns it ANDed with the caller's word.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum

- `allocate_and_locate_packed_block`: Program entry that allocates memory and then searches a linked list for a packed-data header tag to locate the block to unpack.
- `extract_flag_bit_from_table`: Scans a table of word-pair masks for the matching entry, reads a shift count from that entry, and extracts one bit from the masked word, calling a subroutine when a borrow condition is met.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_word_with_conditional_call`: Masks one word with another and, when a byte comparison fails, calls a subroutine and returns its word instead of the masked value.
- `return_immediately`: An empty stub that returns straight away without changing any memory or registers.
- `build_canonical_code_table`: Reads a list of code lengths through a subroutine, then for each length writes a mask and a bit-reversed code into a lookup table.
- `relocate_and_free_block_list`: Walks a list of allocated blocks, adds a base offset into each recorded slot, and releases the list with the system free routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_custom_chips_and_display`: Takes over the system, opens the graphics library, sets the display window, data fetch and bitplane registers, enables DMA and interrupts, and runs setup subroutines.
- `fill_four_stepped_addresses`: Fills four consecutive table entries with address words that grow by a fixed step each time, storing the high and low word of each.
- `fill_seven_stepped_addresses`: Fills seven consecutive table entries with address words that grow by a fixed step each time, storing the high and low word of each.
- `call_subroutines_and_return_result`: Calls three subroutine routines in order and returns the result of the third call.
- `advance_frame_and_wait_blitter`: Advances a set of frame counters with wrap-around, updates a sequence table, calls subroutines, and waits for the blitter to be idle before finishing.
- `reset_blitter_state_counters`: Waits for the blitter to be idle, clears a block of state words and counters, and sets a few fixed values for the next blit.
- `advance_wrapping_counter_array`: Advances an array of byte counters one step each, carrying into the next counter when a counter passes its limit, and calls a subroutine for each step.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_seven_words_in_slot`: Stores seven words from the input into a slot whose address is computed from a counter and a stride.
- `advance_flagged_counter`: When a flag byte from a table is nonzero, calls a subroutine and advances two counters with wrap-around, then calls a further subroutine.
- `store_seven_bytes_strided`: Stores seven bytes from the input into a destination computed from two counters, using a fixed row stride between the bytes.
- `copy_coords_and_toggle_dma`: Waits for the blitter, copies three stored coordinate words into state, and writes DMA control words that disable and then re-enable channels.
- `scale_three_arrays_fixed_point`: Scales three arrays of values in place by a fixed-point factor read from a table, using a multiply and shift for each entry.
- `compute_entry_differences_list`: Walks a zero-terminated list, looking up point coordinates from tables and computing scaled differences for each entry.
- `reset_blitter_parameters`: Waits for the blitter to be idle, clears a set of blitter parameter words, and sets a fixed pointer, modulo and control word.
- `advance_three_wrapping_coordinates`: Adds stored increments to three coordinate words and wraps each one back when it passes the upper limit.
- `copy_points_scaled_xy`: Scales a list of points by a factor and writes the scaled x and y values into two output arrays, copying the third component across.
- `scale_three_components_with_factor`: Scales a list of points by a factor and writes all three scaled components into three output arrays.
- `set_blitter_pointers_for_frame`: Waits for the blitter to be idle, then loads its source and destination pointers, a modulo and a control word for the next frame.
- `walk_entry_list_and_blit`: Waits for the blitter, sets clip and size values, then walks a pointer list calling subroutines for each entry.
- `setup_line_draw_increments`: Computes the horizontal span between two edge positions against a 64-unit limit, swaps the endpoints when needed, and sets up the increments for a line draw.
- `set_blitter_fill_parameters`: Waits for the blitter, programs its control word, first and last word masks and two pointers, then sets the blit size.
- `set_blitter_copy_parameters`: Waits for the blitter, programs its control word, masks and two pointers with a fixed source value, then sets the blit size.
- `call_subroutine_four_times`: Calls one subroutine routine four times in succession and returns.
- `set_blitter_pointers_from_args`: Waits for the blitter, sets its size and mask registers, loads its pointers from the two arguments, and starts the blit.
- `pack_bits_into_long`: Calls a routine to pack a shifted bit into a byte with an offset from another byte and stores the resulting long word at a pointer.
- `execute_bit_packing`: Invokes a routine that shifts a low bit into a packed byte using an offset and stores the long word at a memory location.
- `pack_shifted_bits_into_long`: Shifts the low bit of a value into a packed byte with an offset from a second byte, stores the packed long word at the pointer, and returns the value shifted right by one.
- `read_next_table_entry_wrapping`: Reads the next six-byte entry from a table, wrapping back to the start when an entry is marked with a negative value, and stores its fields into two state words.
- `copy_triplets_into_three_arrays`: Copies seventeen groups of three words from a source block into three destination arrays and advances the third value of each group by a computed offset.
- `store_offset_longs_and_constants`: Stores the global value and that value plus a constant into two output long words and writes two fixed words.
- `blit_fill_area_with_value`: Waits for the blitter, sets its control word, masks and size, and fills a rectangle with a value supplied by the caller.
- `scale_three_arrays_by_product`: Scales arrays of values in place by the product of a per-entry factor and a constant, adding each scaled amount back into its entry.
- `update_three_arrays_with_increments`: Updates three parallel arrays entry by entry with increments and limit checks, using the shared global value as the running offset.
- `set_three_state_constants`: Writes three fixed constant values into state words.
- `lookup_table_pairs_into_state`: Looks up pairs of values from two tables using stored indices, combines them with the current position value, and updates several state words.
- `tiny_ori_2`: Small routine of 1 instruction that merges 44 into (a6).
- `tiny_ori_ori`: Small routine of 2 instructions that merges 2 into d2, then merges 6 into d4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_audio_tables_and_volumes`: Fills a table of pointers with values stepping by a fixed amount, finds the largest byte in a table, and initializes the audio channel volume registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `advance_audio_frame_counter`: Advances a frame counter and, when it wraps, writes stored audio sample pointers and lengths to the audio channel registers after waiting for the DMA to be ready.
- `update_audio_channel_pointers`: Calls four subroutines, then sets the audio channel pointer words from stored values and returns a masked control word.
- `program_audio_channel_period`: Loads a sample pointer for a channel, looks up its period from a table and programs the channel's length and location registers after reading the raster position.
- `advance_music_channel_step`: Advances a music channel's pattern step, handles effect codes from the note data, and writes the channel volume and period.
- `lookup_note_period_for_channel`: Searches a row of a period table for the first value at or below the channel's current period, applies a half-step adjustment when a flag is set, and stores the result in the channel.
- `apply_channel_slide_step`: Applies a slide step to a channel's period, moving it up or down by a per-tick amount until it reaches its target.
- `compute_vibrato_waveform_offset`: Computes a waveform value for a channel's vibrato or tremolo from a table or a ramp chosen by the waveform selector, scales it by the depth, and stores the offset.
- `apply_channel_volume_slide`: Reads an effect nibble from a channel and, for volume slides, adjusts the channel volume by a step, clamping it at its limits.
- `latch_channel_effect_nibble`: Copies the low four bits of a stored pattern byte into the channel's effect field.
- `update_pointer_based_on_flag`: Checks a high bit in a channel field; when it is set, clears the field, advances a pointer through a table lookup with wrap-around, and negates the byte at the new position.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_program_loader`: Opens the system libraries, saves the stack pointer, disables interrupts through the processor status, takes over the custom chip registers, and runs the loader subroutines.
- `parse_loader_state_word`: Tests bit-fields of a loader state word, updates the state word and calls subroutines according to the flags it finds.
- `copy_bytes_then_jump_indirect`: Copies a block of bytes from a source to a destination one at a time, then jumps through a pointer that could not be recovered.
- `copy_bytes_backwards_then_jump`: Copies bytes backwards from the end of one buffer to the end of another, then jumps through a pointer; neighbouring text holds library-open and file-error messages.
- `call_library_with_string_argument`: Calls a library function through the stored library base with a pointer to a null-terminated string, skipping over the string, then calls two further library functions.
- `probe_memory_banks_by_type`: Probes memory in half-megabyte steps with the system memory-type call, setting a bit for each bank found and adding its size to running totals.
- `allocate_memory_and_call_library`: Allocates memory with the system allocator, calls several library functions and one other routine.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown)

- `process_large_code_block`: This routine consists of a large sequence of thirteen thousand eight hundred instructions at the start of a raw code region.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown)

- `stub_undecoded_six_instructions`: The decompiler produced no pseudo-code for this six-instruction routine, so its behaviour is not recovered; the recorded facts show no calls and no hardware register use.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown)

- `null_routine_placeholder`: This routine is empty and contains no instructions.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown)

- `execute_extensive_instruction_sequence`: This routine contains thirteen thousand six hundred forty-six instructions located at the beginning of a raw code region.
