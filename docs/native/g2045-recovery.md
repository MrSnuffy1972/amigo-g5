# Game 2045 (sensible world of soccer (1996)(renegade)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02045). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 243 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 15367 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (2 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 132 of 132 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 770690 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16013 of 16013 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 207652 |
| code | 1992 |
| data | 1556206 |
| padding | 32294 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SSBOOT | 1 | 542 | 14 |
| Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:bootblock | 1 | 18 | 1 |
| Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2.REL#unpacked (raw code file, load address unknown) | 1 | 15451 | 1 |
| Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40384 -> 61804 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (165858 -> 351588 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER4.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2892 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3985 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/DISK2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1113 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER00.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2144 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/SWCPICH2.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12503 -> 39448 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER9.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2885 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER8A.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3593 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER7.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6583 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER5.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3347 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2929 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER10.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1754 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9607 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER01.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (1252 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER8B.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6599 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:grafs/LOADER6.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6073 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:sound/HERO.INS: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (250368 -> 292984 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (168095 -> 357298 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS.REL: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (39802 -> 60932 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCTEAM3.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6606 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SWCPICH6.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6883 -> 31384 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/MENUBG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8141 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCTEAM1.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6691 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCGRAFS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2684 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SWCPICH4.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12130 -> 39832 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCBITS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (978 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCBENCH.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2589 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CHARSET.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2321 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCTEAM2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6503 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SWCPICH5.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13578 -> 44312 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/CJCTEAMG.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3996 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/MENUS2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5733 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SWCPICH3.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14320 -> 43032 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SOCCER_S.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5771 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/SWCPICH1.MAP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (13395 -> 45592 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/MENUBG2.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (18119 -> 47872 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:grafs/MENUS.RAW: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5380 -> 40960 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.007: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4304 -> 10262 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.008: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (24669 -> 63614 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.062: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7949 -> 20522 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.064: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3521 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.065: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4658 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.066: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2956 -> 6842 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.010: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4065 -> 9578 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.067: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3029 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.011: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3020 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.012: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4724 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.069: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5709 -> 12314 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.013: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14638 -> 34886 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.014: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12875 -> 30098 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.015: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7350 -> 17102 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.016: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5871 -> 14366 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.017: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3615 -> 8894 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.071: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6304 -> 17786 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.018: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4143 -> 10262 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.072: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13780 -> 32834 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.019: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5120 -> 11630 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.073: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3283 -> 6842 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.075: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1810 -> 4106 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.076: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4486 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.077: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5819 -> 15050 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.020: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13506 -> 33518 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.078: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3600 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.021: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2980 -> 6842 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.022: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3514 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.079: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2292 -> 5474 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.023: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4914 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.024: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4226 -> 10262 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.025: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10992 -> 24626 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.026: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4446 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.080: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (16063 -> 33518 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.027: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5644 -> 12998 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.081: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9989 -> 23258 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.028: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5449 -> 12314 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.082: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3296 -> 6842 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.029: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6707 -> 17102 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.083: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4227 -> 9578 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/EUROCUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (6053 -> 10944 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.084: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9397 -> 21206 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.085: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1434 -> 2738 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.030: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6189 -> 15050 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/POOLPLYR.DAT: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (27305 -> 52200 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.031: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6395 -> 15734 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.032: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2969 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.033: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10502 -> 27362 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.034: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4877 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.035: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13053 -> 32834 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.036: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4918 -> 11630 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.037: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5269 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.038: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5578 -> 12314 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.039: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5315 -> 12314 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.040: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5397 -> 15050 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.041: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5204 -> 12998 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.042: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3816 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.043: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10448 -> 25994 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.044: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12333 -> 34202 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.045: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3686 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.046: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5848 -> 16418 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.048: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4551 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.049: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4692 -> 10946 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.050: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3610 -> 8210 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/EUROCWC.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11590 -> 21888 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/UEFACUP.TMD: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (11335 -> 21888 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.051: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9528 -> 31466 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.055: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7977 -> 19154 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.000: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5382 -> 13682 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.001: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4553 -> 9578 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.002: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7501 -> 15734 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.003: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5785 -> 15050 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.004: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5479 -> 12998 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.005: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4799 -> 11630 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.006: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5744 -> 13682 bytes)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf:data/TEAM.060: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9675 -> 27362 bytes)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2.REL#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sensible world of soccer (1996)(renegade)(disk 1 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- sensible world of soccer (1996)(renegade)(disk 2 of 2)/Sensible World of Soccer (1996)(Renegade)(Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 2 never-loaded region(s), 708886 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SSBOOT

- `start_program_and_load_rel`: Program entry point that checks available memory against a one-megabyte threshold, builds a filename from the command text with a rel extension and opens it through the exec library.
- `patch_addresses_and_jump`: Finds the lowest set bit of a mask and, when none exists, opens a library and loops forever.
- `copy_bytes_forward_and_jump`: Copies a counted block of bytes forward from the source pointer to the destination pointer, one byte per step, then transfers control through a pointer.
- `copy_bytes_backward_and_jump`: Copies a counted block of bytes backwards from the end of the source into the end of the destination, then transfers control through a pointer.
- `print_text_via_dos_output`: Opens a library through the exec library table, measures a null-terminated text string, and writes it out through the DOS output and write functions.
- `scan_memory_region_types`: Loops over 32 memory regions and probes up to 256 sample addresses in each through the exec memory-type call.
- `load_packed_file_into_memory`: Opens a file through the DOS library, seeks and reads its header, then allocates memory through the exec library and reads the body into it.
- `decode_compressed_chunk`: Checks a four-byte packed-data signature, then decodes a compressed stream by copying a literal block into a stack buffer in reverse order.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask from byte values taken from the matched entry and from register contents.
- `mask_and_conditional_call`: Ands one word with a second word and, when a byte in one register is less than a byte in another, calls a helper and uses its returned word as the result.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from a pointer and builds a 32-bit value by appending each byte in turn.
- `build_code_length_tables`: Reads an entry count and a list of code lengths through helper calls, then loops over bit lengths from one to sixteen.
- `empty_routine`: This routine contains no executable instructions.

### Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2.REL#unpacked (raw code file, load address unknown)

- `inspect_undecompiled_raw_region`: This raw code region is about 15451 instructions long and starts at its entry point, but the decompiler timed out.

### Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Sensible World of Soccer (1996)(Renegade)(Disk 1 of 2).adf:SWOS2#unpacked (raw code file, load address unknown)

- `stub_single_instruction_entry`: This raw code region holds a single instruction at its start.
