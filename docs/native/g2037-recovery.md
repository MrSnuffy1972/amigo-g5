# Game 2037 (sensible world of soccer (1994)(renegade)[cr hlm][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02037). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1813760 top-level bytes in 226 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 27269 raw-word lines |
| Functions named and described | pass, 128 of 128 named (0 author symbols), 128 of 128 described (7 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1837 of 1837 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 781548 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 31846 of 31846 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 60292 |
| code | 9194 |
| data | 1337174 |
| padding | 28732 |
| unused | 378368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:ssboot | 3 | 160 | 5 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum | 2 | 239 | 8 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 901 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 823 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 529 | 13 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum#unpacked | 1 | 1724 | 48 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown) | 1 | 13800 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown) | 1 | 6 | 1 |
| Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown) | 1 | 13646 | 1 |

## Cases handled

- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (36038 -> 55200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (150060 -> 334650 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/MENUBG.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH4.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12206 -> 39832 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/DISK2.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8324 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH2.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12574 -> 39320 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH5.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13649 -> 44312 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/LOADER1.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8295 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:GRAFS/SWCPICH3.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (14378 -> 42904 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:SOUND/MISS3C: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12713 -> 12788 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:SOUND/MENMUSIC.MOD: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (150951 -> 337114 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (35672 -> 54584 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM3.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6251 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM1.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6338 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUBG.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7927 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCGRAFS.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2928 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCBITS.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4864 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCBENCH.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4593 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CHARSET.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2143 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAM2.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6158 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/SOCPIC.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6807 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/CJCTEAMG.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4608 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUS2.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5267 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/SWCPICH1.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13570 -> 45848 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUBG2.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (18119 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/MENUS.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4139 -> 40960 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:GRAFS/WINSCRN.RAW: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11386 -> 47872 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:SOUND/MENMUSIC.MOD: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (35504 -> 47868 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.007: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4064 -> 8894 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.062: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (9589 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.008: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (29483 -> 63614 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.064: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6953 -> 16418 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.065: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5344 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.066: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3223 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.067: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3361 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.010: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4477 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.011: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3349 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.069: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7178 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.012: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4529 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.013: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15787 -> 32150 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.014: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13856 -> 28046 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.015: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6889 -> 14366 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.016: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5846 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.071: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7217 -> 17786 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.017: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3041 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.072: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (9064 -> 19838 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.018: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4569 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.019: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4874 -> 10262 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.073: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2814 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.075: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1916 -> 4106 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.076: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5555 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.020: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15031 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.077: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4884 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.021: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3208 -> 6842 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.078: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3840 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.022: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3841 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.079: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2409 -> 5474 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.023: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3640 -> 7526 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.024: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3855 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.025: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12341 -> 24626 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.026: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4977 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.080: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (17086 -> 32834 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.027: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5228 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.081: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (10788 -> 23258 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.028: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7095 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.082: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3145 -> 6158 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.029: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6454 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.083: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4341 -> 9578 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/EUROCUP.TMD: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5898 -> 10944 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.084: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (10034 -> 21206 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.085: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1488 -> 2738 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/POOLPLYR.DAT: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (26365 -> 52200 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.030: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6424 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.031: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7190 -> 15734 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.032: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3291 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.033: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12283 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.034: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6394 -> 13682 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.035: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (14003 -> 30098 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.036: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4973 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.037: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5545 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.038: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6428 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.039: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5798 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.040: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6282 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.041: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5767 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.042: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4170 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.043: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12660 -> 28730 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.044: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15401 -> 34886 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.045: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5047 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.046: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8846 -> 22574 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.048: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5938 -> 12998 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.049: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5387 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/EUROCWC.TMD: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11491 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.050: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3823 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/UEFACUP.TMD: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11462 -> 21888 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.051: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12102 -> 31466 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.055: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8972 -> 19154 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.000: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5268 -> 11630 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.001: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4208 -> 8210 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.002: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (7882 -> 15050 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.003: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5486 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.004: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5713 -> 12314 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.005: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5057 -> 10946 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.060: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11486 -> 27362 bytes)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 2 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 2 of 2).adf:DATA/TEAM.006: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5064 -> 10946 bytes)
- runtime segment an address: 816 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 281 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 344 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:ssboot: CrunchMania Normal (xfd) stream found by ancient scan (2144 -> 2700 bytes) but only 1 of 2 sampled windows are in reference RAM: not used
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum: crunched executable decrunched by XFDMaster (ProPack (RNC) (xfd), 38484 -> 111152 bytes), 29 of 32 windows confirmed in reference RAM: recovered as a hunk executable
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 8 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 8 immediate
- sensible world of soccer (1994)(renegade)[cr hlm][a2](disk 1 of 2)/Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 671764 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_custom_chips_and_display` (runtime segment an address): Takes over the system, opens the graphics library, sets the display window, data fetch and bitplane registers, enables DMA and interrupts, and runs setup subroutines. Kind: disk hardware/sync check. Pass path: the branch where _DAT_00081d9a != 0 is taken.
- `initialize_system` (hoodlum#unpacked): Opens a library, forbids execution, configures custom hardware registers including DMA and interrupts, performs a loop to shift bits in a table, and calls several routines to initialize audio, input, and graphics. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB (port B of CIA B) in a sequence that suggests checking for specific hardware states or presence of a disk drive, and then waits for a condition on CIAA_PRA (port A of CIA A) to proceed. If the check passes, it .

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
- `copy_and_clear_data`: Copies two sets of 7 words from two different source locations to a destination location and clears the word following each set.
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

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_relocate_resource`: Loads a file from disk, processes its string content, and performs a relocation pass by iterating through a block of memory to apply byte offsets to longword values.
- `patch_addresses_and_jump`: Finds the lowest set bit of a mask and, when none exists, opens a library and loops forever.
- `frag_move_subi_bne_jmp`: Short code fragment that copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then jumps to the address in a2.
- `copy_bytes_backward_and_jump`: Copies a counted block of bytes backwards from the end of the source into the end of the destination, then transfers control through a pointer.
- `print_text_via_dos_output`: Opens a library through the system table, measures a null-terminated text up to its terminator, and writes it out through the DOS output and write calls.
- `scan_memory_region_types`: Loops over 32 memory regions, testing up to 256 sample addresses per region through the exec memory-type call.
- `load_packed_file_into_memory`: Opens a file through the DOS library, seeks and reads its header, then allocates memory through the exec library and reads the body into it.
- `decode_compressed_chunk`: Checks the packer signature, reads the packed and unpacked sizes, and decodes the stream into a buffer using a 384-byte work area on the stack.
- `parse_bit_table`: Scans pairs of words until one matches the masked input, then reads a byte from that entry and builds a bit mask from it and the input.
- `mask_and_conditional_call`: ANDs two words together and, if one byte is less than another, calls a routine and returns its word instead.
- `frag_add_lsr_swap_addq_move_rol_sub_mov`: Short code fragment that adds d1 to d7, then shifts d6 right by d7, then swaps the halves of d6, then adds 4 to a3, then copies the memory just below a3 (moving a3 down) to d6, then rotates d6 left by 8, then copies the memory just below a3 (moving a3 down) to d6, then swaps the halves of d6, then subtracts d7 from d1, then loads 16 into d7, then subtracts d1 from d7, then returns.
- `frag_moveq_lsl_move_dbra_rts`: Short code fragment that loads 3 into d1, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then returns.
- `build_huffman_tree_nodes`: Reads a count and a list of bytes, then for each code length matching a running value writes a mask and shifted code bits into a destination array, building the nodes of a code tree.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:hoodlum#unpacked

- `initialize_system`: Opens a library, forbids execution, configures custom hardware registers including DMA and interrupts, performs a loop to shift bits in a table, and calls several routines to initialize audio, input, and graphics.
- `fill_four_stepped_addresses_2`: Fills four table entries, each a longword address stored as two words, with the address increasing by 5000 each time.
- `fill_seven_stepped_addresses_2`: Fills seven table entries, each a longword address stored as two words, with the address increasing by 264 each time.
- `call_helpers_return_third_result`: Calls four routines in order and returns the result of the third call.
- `advance_frame_and_wait_blitter_2`: Sets frame counters, clears two values when a counter is below 700, and loads a table of address words when another counter is zero.
- `reset_blitter_state_counters_2`: Waits for the blitter to be idle, clears a block of state words and counters, then sets fixed values for the next blit.
- `advance_wrapping_counter_array_2`: Advances an array of counters one step each, carrying into the next counter when a counter passes its limit of 29, and calls a helper for each step.
- `copy_and_clear_data_2`: Copies two sets of 7 words from two different source locations to a destination location and clears the word following each set.
- `store_seven_words_in_slot_2`: Stores seven words from an input list into a slot whose address comes from a counter and a stride.
- `advance_flagged_counter_2`: When a flag byte from a table is non-zero, calls a helper, advances two counters with wrap-around, then calls a further helper.
- `store_seven_bytes_strided_2`: Stores seven bytes from an input buffer into a destination computed from two counters, with a fixed stride of 40 bytes between each byte.
- `start_blit_frame_copy_coords`: Waits for the blitter to be idle, copies three coordinate words into fixed locations, calls several helpers and writes DMA control values around them.
- `scale_three_arrays_fixed_point_2`: Scales three arrays of values in place by a fixed-point factor read from a table, using a multiply and a ten-bit shift for each entry.
- `walk_edge_cross_product_list`: Walks a zero-terminated list of records, looks up point coordinates from a table for each entry and computes two cross-product terms, storing a result when the terms compare.
- `wait_for_blitter_idle`: Waits for the blitter to be idle by checking a specific bit in a structure, then clears multiple blitter parameter words and sets fixed values for pointer, modulo, and control words.
- `advance_three_wrapping_coordinates_2`: Adds stored increments to three coordinate words and wraps each one back by 720 when it passes 719.
- `scale_point_components`: Reads pairs of 16-bit values from a source table, scales them by a factor, and writes the results into two separate output tables while copying a third value unchanged to a third table.
- `scale_point_components_2`: Reads pairs of 16-bit values from a table, scales them by a factor, and writes the results into three separate output arrays.
- `frag_btst_bne_move_clr_rts`: Short code fragment that tests bit 14 of the memory at an offset from a5, then branches if the last result was not zero, then copies 66526 to the memory at an offset from a5, then copies 67582 to the memory at an offset from a5, then clears the memory at an offset from a5, then copies an address to the memory at an offset from a5, then copies 16898 to the memory at an offset from a5, then returns.
- `draw_blit_list_entries`: Waits for the blitter, sets its initial registers, then walks a zero-terminated list of entries and calls a drawing helper for each entry whose flag is set.
- `setup_line_draw_increments_2`: Computes the horizontal span between two edge positions against a 64-unit limit, swaps the endpoints when needed and sets up the increments for a line draw.
- `test_multiple_bits`: Tests bits in registers and memory locations pointed to by a1, a3, a5, and a7.
- `set_blitter_copy_parameters_2`: Waits for the blitter, programs its control word, sets the mask and two pointers from fixed values, then sets the blit size.
- `call_subroutine_four_times_2`: Calls one subroutine four times in succession and returns.
- `set_blitter_pointers_from_args_2`: Waits for the blitter, sets its size and mask registers, loads its pointers from the two arguments and starts the blit.
- `generate_bit_patterns`: Calls another routine four times, incrementing a value in d4 by 16 before each call, while passing d0 and d1 as arguments.
- `pack_shifted_bits`: Shifts the low bit of a value into a packed byte with an offset from a second byte, stores the packed longword at a pointer, and returns.
- `pack_shifted_bits_into_long_2`: Shifts the low bit of a value into a packed byte with an offset from a second byte, stores the packed longword at a pointer, and returns the value shifted right by one.
- `read_next_table_entry_wrapping_2`: Reads the next six-byte entry from a table, wrapping back to the start when an entry is marked with a negative value, and stores its fields into two state words.
- `copy_triplets_into_three_arrays_2`: Copies seventeen groups of three words from a source block into three arrays and advances the third value of each group by a computed offset.
- `store_offset_longs_and_constants_2`: Stores a shared value and that value plus a constant into two output longwords, then writes two fixed words.
- `blit_fill_area_with_value_2`: Waits for the blitter, sets its control word, mask and size, and fills a rectangle with a value supplied by the caller.
- `scale_and_accumulate`: Reads values from three arrays, scales them by a factor, shifts the result, and adds it back to the original values in two of the arrays.
- `update_three_arrays_with_increments_2`: Updates three parallel arrays entry by entry with increments and limit checks, using a shared value as the running offset.
- `frag_move_rts`: Short code fragment that copies 710 to a fixed memory address, then copies 670 to a fixed memory address, then copies 10 to a fixed memory address, then returns.
- `process_weighted_averages`: Calculates weighted averages for three sets of values using three different scaling factors and updates three tables via a loop controlled by a value in a global variable.
- `initialize_audio_and_input_state`: Sets up audio volume registers, initializes a table of pointers and values, clears memory regions, sets up global variables for input state, and zeroes out several global variables and a function pointer.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `reload_audio_channel_pointers`: Advances a frame counter.
- `update_audio_channel_control`: Calls four helpers, then reads a per-channel control word.
- `program_channel_from_sound_table`: Reads a nibble from a packed sound-table entry to select a 30-byte record, then loads that record's sample address, length and volume into an audio channel and updates the DMA control register.
- `tick_tracker_channel_effects`: Per-channel tick: after a helper call, if the channel's control nibble is zero it selects a parameter from one of three sources by a frame counter modulo three.
- `find_period_for_note`: Searches a 74-byte-per-row period table for the first entry at or above the channel's note value, steps back one entry when a flag bit is set, and stores the chosen period.
- `slide_value_toward_target`: Moves a value by a step toward a target, up or down depending on a direction byte, and stops at the target, clearing the active count when it is reached.
- `compute_waveform_modulation_offset`: Selects a waveform value from a table, a sawtooth-style ramp or a constant depending on a type nibble, scales it by a depth nibble, applies a sign and adds the result to a base field.
- `apply_channel_effect_command`: Reads an effect nibble from the channel record and handles the sample-offset and position-jump style commands directly.
- `copy_low_nibble_to_field`: Copies the low nibble of a channel byte into a second channel field.
- `update_pointer_based_on_flag_2`: The routine checks a high bit in a structure field; if set, it clears the field, adjusts a pointer based on a table lookup and offset, and negates the byte at the new pointer location.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2.rel#unpacked (raw code file, load address unknown)

- `process_large_code_block`: This routine consists of a large sequence of thirteen thousand eight hundred instructions at the start of a raw code region.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos#unpacked (raw code file, load address unknown)

- `stub_undecoded_six_instructions`: The decompiler produced no pseudo-code for this six-instruction routine, so its behaviour is not recovered; the recorded facts show no calls and no hardware register use.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos2#unpacked (raw code file, load address unknown)

- `null_routine_placeholder`: This routine is empty and contains no instructions.

### Sensible World of Soccer (1994)(Renegade)[cr HLM][a2](Disk 1 of 2).adf:swos.rel#unpacked (raw code file, load address unknown)

- `execute_extensive_instruction_sequence`: This routine contains thirteen thousand six hundred forty-six instructions located at the beginning of a raw code region.
