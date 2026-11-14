# Game 1871 (flashback (1992)(u. s. gold)[cr interpol][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01871). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3609616 top-level bytes in 249 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 139 of 139 named (0 author symbols), 139 of 139 described (13 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1490 of 1490 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 47040 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3916 of 3918 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2092360 |
| code | 8856 |
| data | 1315643 |
| padding | 46325 |
| unused | 146432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:ExpLoad | 1 | 257 | 26 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:flashback | 7 | 63 | 1 |
| explode.library | 1 | 475 | 30 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:Failat | 2 | 297 | 11 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1205 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:INTERPOL#unpacked | 1 | 1205 | 29 |
| level3_2.obc#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:data/level1.ct: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (564 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:data/level1.sgd: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (45024 -> 74679 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:data/present.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (26652 -> 44806 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:data/perso.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (103888 -> 178647 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:cine/logos.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2976 -> 4035 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:cine/intro.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (42116 -> 48961 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/replicant.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (36320 -> 55636 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/garde.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (29004 -> 61045 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/level2.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (876 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/level1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (13352 -> 34050 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/junky.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (25140 -> 37142 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:data/level2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (27212 -> 66644 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/pont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4212 -> 4200 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/debut.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12120 -> 13644 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/genexp.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1936 -> 2000 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/memo.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (21052 -> 17712 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/map.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4264 -> 5428 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/missions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (5000 -> 4030 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/holoseq.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7064 -> 7280 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/metro.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2940 -> 3464 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/alimpont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2148 -> 2094 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:cine/serrure.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3512 -> 3873 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/replicant.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (36320 -> 55636 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/garde.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (29004 -> 61045 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/level3_1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (17148 -> 42748 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/dt.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12848 -> 33374 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/junky.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (25140 -> 37142 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/dt.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (728 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/glue.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (24224 -> 36958 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/level3_2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18856 -> 47040 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/level3_1.ct: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (812 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/voyage.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14556 -> 14703 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/espions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7148 -> 6917 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/taxi.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3916 -> 2659 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:data/level4_2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16264 -> 39106 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:data/level4_1.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (812 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:data/glue.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (24224 -> 36958 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:data/level4_1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14892 -> 36868 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/pont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4212 -> 4200 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/lift.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2736 -> 2698 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/debut.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12120 -> 13644 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/genexp.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1936 -> 2000 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/voyage.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14556 -> 14703 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/memo.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (21052 -> 17712 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/espions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7148 -> 6917 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/log.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2012 -> 1813 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/teleport.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4528 -> 5088 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/taxi.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3916 -> 2659 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf:cine/fin.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18072 -> 11829 bytes)
- runtime segment an address: 935 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:INTERPOL: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 14180 -> 33324 bytes), 9 of 10 windows confirmed in reference RAM: recovered as a hunk executable
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:flashback: Imploder Library (xfd) stream found by ancient scan (83312 -> 190628 bytes) but only 1 of 90 sampled windows are in reference RAM: not used
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf:data/level3_2.obc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr interpol][a](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol][a](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol][a](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 47040 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `initialize_data_structures`: Sets a value at a pointer offset, assigns a constant to a memory location, and calls another routine.
- `setup_stack_and_call`: Stores a value into a stack-relative location, pushes a certain address onto the stack, and calls a subroutine.
- `find_name_and_invoke_hook`: Uses a library call to find a name, parses a string entry, and invokes a function pointer while clearing flags.
- `branch_on_flag_status`: Performs a series of conditional branches and a branch-to-subroutine based on the status of a flag.
- `initialize_subroutine_with_pointer`: Stores a value from a register, conditionally calls a subroutine, and loads a new address into a register before calling another subroutine.
- `execute_function_pointer`: Calls a function through a pointer retrieved from a global variable.
- `call_hook_and_clear_flags`: Checks a pointer stored in a global variable; if non-zero, it invokes a function pointer located at an offset from that address.
- `clear_flag_and_call_hook`: The routine checks a flag stored at an offset from a global variable using a short index in D0.
- `parse_string_and_call_function`: Parses a string table entry using a value derived from a register and then calls a function pointer stored in a global variable.
- `parse_string_entry`: Calls a routine to parse a string table entry and then executes a function pointer stored in a global variable.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `setup_stack_and_call_subroutine`: Updates the stack pointer and calls a subroutine.
- `test_signal_and_callback`: Calls another routine to test a signal flag and subsequently calls a second routine while checking a zero flag.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `store_byte_and_advance_pointer`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `store_return_values`: The routine calls another routine, captures its return value and an extra output pointer, and stores both into two consecutive words of a caller-provided structure via a pointer in A4, then returns the original input value in D0.
- `update_register_with_offset_address`: Loads a pointer from a memory location offset by a register into another register.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.
- `set_signal_flag`: The routine calls another routine and tests a returned value for a specific bit.
- `clear_flag_in_structure`: The routine calls another routine and then writes a constant to a field of a structure pointed to by a register.
- `check_and_invoke_callbacks`: The routine calls another routine and then checks a zero flag.
- `store_parameter_to_global_offset`: Calls another routine and stores a passed parameter into a global variable at a fixed offset.
- `call_indirect_via_global_table`: The routine calls another routine through a pointer stored in a global variable.
- `count_string_bytes`: The routine receives a pointer in a0 and walks forward byte by byte until it finds a zero byte, then returns the number of bytes from the start pointer to the terminating zero.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:INTERPOL

- `parse_amigados_hunks`: Iterates through a memory block to identify and process AmigaDOS hunk types.
- `decrement_and_loop`: Decrements a value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `perform_memory_block_shift`: Performs complex bitwise checks and moves blocks of data through memory using loops and pointer arithmetic.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_rotation`: Performs a series of bitwise shifts and rotations on data based on a variable number of bits.
- `modify_memory_via_pointers`: Performs arithmetic on a value at a pointer address and updates a byte at another pointer address using a bitwise OR operation.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:flashback

- `initialize_game_state`: The routine checks a global variable and a flag, then calls another routine from a table and handles an error path.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `sync_object_metadata`: Retrieves a pointer from a library function and copies specific short and long values from the pointed-to memory block into a destination structure.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `update_library_structure`: As library code, it decrements a value in a structure and, if certain conditions are met, updates several pointers and calls multiple functions from a global variable.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_update_library`: Library code that checks magic values and function pointers within a structure to verify its integrity.
- `set_library_flag`: As library code, it sets a specific bit in a byte within a structure.
- `verify_library_signature`: Checks a memory location for a specific four-byte signature and a sequence of characters representing a library name.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `verify_library_name`: As library code, it checks a memory location for a specific four-byte signature and a sequence of characters representing a library name.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_timer_and_trigger_explosion`: Decrements a short integer within a structure and calls a routine to handle an effect.
- `calculate_bit_shift_table`: Computes a table of power-of-two values based on bit-shifted input bytes.
- `copy_and_accumulate_memory`: This routine performs bulk memory transfers using longword increments and handles conditional additive accumulation where values from a source buffer are added to a destination buffer.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copy_and_accumulate_blocks`: Copies blocks of memory using longword transfers and performs additive accumulation on destination memory locations based on values retrieved from a source bitstream.
- `copy_and_accumulate_memory_blocks`: Copies blocks of memory using longword increments and performs additive accumulation on target memory locations based on values retrieved from a source buffer.
- `copy_and_process_bitstream_data`: Copies a block of memory using word or longword increments and subsequently processes decoded values by updating a target buffer.
- `extract_variable_bitstream`: Reads a sequence of bits from a source buffer by decoding variable-length integer values.
- `decode_variable_length_integer`: Decodes a signed integer from a bit-packed stream by iteratively checking bits and performing left shifts.
- `parse_variable_length_bitstream`: Iterates through a data structure to decode bit-packed values by performing bit-shifting and conditional pointer increments.
- `decode_variable_length_integers`: Iterates through a data structure to decode variable-length integer values using bit-shifting and pointer increments.
- `parse_variable_length_offsets`: Iterates through a data structure to decode variable-length integer values using bit-shifting and table lookups.
- `parse_variable_length_bitstream_2`: Iterates through a data structure to decode variable-length bitfields by performing bit-shifting and table lookups.
- `calculate_buffer_segment_offsets`: Iterates through a data structure to compute specific memory offsets and lengths using bit-shifting and table lookups.
- `update_buffer_state`: Calculates new buffer offsets and lengths based on bit-shifted values and table lookups, then performs a memory shift and updates a control structure.
- `shift_buffer_data`: Shifts a block of memory within a buffer by a calculated offset.
- `reorder_bitstream_buffer`: Rearranges data within a buffer by decoding variable-length integers and performing block shifts.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies a block of sixteen bytes from a source address to a destination address and then calls another routine.
- `initialize_structure_and_call`: Stores a byte from a table into a structure at offset 12, clears offsets 4 and 16, sets offset 8 to 1, and calls a routine using a pointer from offset 40 of a structure in a2.
- `parse_structure_data`: Increments a value at an offset in a structure and enters a loop that compares bytes from a table at calculated offsets against specific constants.
- `process_structure_data_loop`: Calls a routine using a pointer from a table in a5, then enters a loop that reads bytes from a table in a0 using offsets derived from a structure in a1, updates a value at offset 4 of that structure, and increments a value at offset 8 of that structure until a condition is met.
- `process_structure_data_2`: Sets d2 to 20, d1 to 255, and d0 to 32, then calls a routine using a pointer from a table in a2, checks a long value at offset 16 of a structure in a1, and if it is not zero, negates a long value from offset 4 of a1 and stores it in d1.
- `jump_through_table`: Loads a long word from address a1 plus 4 into d1, then jumps to the address in a6 through a function pointer.
- `update_structure_index_and_call`: Calculates an index by shifting the address in a1 by two and storing it at offset 4 of the structure in a1, then performs a sequence of three calls to a routine via a pointer in a5 using various offsets from structures in a2.
- `test_and_call_routine`: Tests a long value at an offset in a0 calculated from a structure in a1, and if it is zero, loads a value from a structure in a2, sets up registers, and calls a routine using a pointer from a5.
- `update_structure_field_via_table`: Fetches a value from a table using an index in a structure, calls a routine to process it, and stores the result back into the structure.
- `copy_and_call_routine`: Copies a long value from offset 212 of a structure in a1 to offset 556 of a structure in a2, then calls another routine and jumps to the address in a6.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:endrun

- `reset_dma_and_parse_string`: Clears specific memory locations, sets others to all ones, and performs a series of pointer-based updates and function calls.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `enter_infinite_loop`: This routine enters an infinite loop that prevents further execution of the calling code.
- `reset_dma_parameters`: If a register contains a non-zero value, it clears two memory locations and sets two others to all ones before calling another routine.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 2 of 4).adf:bootblock

- `check_library_status`: Calls a function via a pointer and returns a specific constant based on whether the return value is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter_and_wait`: The routine programs the Amiga blitter registers BLTCON0, BLTCON1, BLTBMOD, BLTBPTH, BLTDMOD, BLTDPTH and BLTSIZE and then repeatedly polls CIAA_PRA while performing a small blitter copy loop that moves bytes from a table into a set of memory locations.
- `step_blitter_pointers_on_cycle`: If a certain word in a global variable is zero, it sets it to four and increments two 32-bit pointer fields by two; otherwise, it subtracts four from that word.
- `reverse_blitter_pointer_step`: If a certain word in a global variable is four, it resets it to zero and decrements two 32-bit pointer fields by two; otherwise, it adds four to that word.
- `render_text_glyphs_to_bitplane`: Copies eight-byte glyph rows from a font table into a destination bitplane, advancing the destination pointer on newline characters and stopping on an escape byte.
- `build_copper_list_and_start_dma`: The routine builds a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor`: Opens the graphics library and allocates memory.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_button_release`: Polls a bit in a hardware register until it becomes zero.
- `wait_keyboard_input_2`: Waits for specific keyboard input sequence by polling the CIAA_SDR register, first waiting for byte value 57 then for byte value 55, similar to routine R2 but at a different address.
- `wait_for_vblank`: Polls a hardware register until its value becomes zero.
- `update_pointer_table`: Iterates through a table of values, updating a global pointer and a secondary table with calculated or or-ed values based on a counter.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `update_display_bounds_and_offset`: Updates a global variable or hardware register with a new offset or bound, performing conditional clamping or pointer-based lookups.
- `update_display_limit_and_offset`: Updates a hardware register or a global variable with a new offset or bound, performing conditional clamping or pointer-based lookups.
- `update_audio_dma_parameters`: Updates audio-related DMA and timing parameters by calculating new pointer offsets and buffer lengths from a lookup table.
- `initialize_interrupt_table`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `update_pointer_and_bounds`: Updates a pointer or offset in a structure based on a masked or bit-shifted value, while performing conditional bounds checking and clamping or resetting certain fields.
- `copy_struct_member`: copies a word from one structure or object to another, applying a fixed offset to the destination pointer.
- `search_table_and_update_state`: Searches a table for a value less than or equal to a masked pointer or register value, then updates a structure member with the found value.
- `update_position_and_lookup_value`: Updates a value in a structure based on a certain condition, then performs a loop to find a value in a table and stores it in a memory location.
- `calculate_offset_and_update_byte`: Calculates a value using bitwise operations and a table lookup, then stores the result into a memory location and updates a byte in a structure.
- `update_struct_field_and_return_offset`: Updates a field within a structure based on a masked or bit-shifted value from a source byte, while potentially adjusting a secondary pointer or counter.
- `modify_dma_control`: Writes a masked byte from a memory location into a DMA control register.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine`: The routine contains an infinite loop with no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: Executes a sequence of ten instructions that calls a library function using a negative offset.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### Flashback (1992)(U. S. Gold)[cr Interpol][a](Disk 1 of 4).adf:INTERPOL#unpacked

- `initialize_graphics_subsystem`: Configures blitter registers and initializes DMA channels for graphics operations, then performs a delay loop while polling input state and updating animation counters.
- `increment_timer_counter`: The routine reads a 16-bit counter from a global variable and, if the counter is zero, it resets the counter to four and increments two 32-bit values built from word pairs in the same global structure, returning the high word of the second value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_two_word_counters`: The routine checks a global variable and, when it equals four, clears it and decrements two 32-bit counters stored as paired 16-bit words in a global structure, updating both halves.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
- `decode_text_to_tile_data`: Iterates through a text buffer, skipping newline characters by advancing a destination pointer by a fixed offset.
- `build_copper_list_and_start_dma_2`: Constructs a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor_2`: Opens the graphics library and allocates memory.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_joystick_fire`: Waits for the joystick or mouse fire button to be released by polling the CIAA_PRA register until bit 6 becomes zero, then returns.
- `wait_keyboard_input`: Waits for specific keyboard input sequence by polling the CIAA_SDR register, first waiting for byte value 57 then for byte value 55.
- `wait_vertical_blank`: Waits for vertical blank by polling the VHPOSR register until its value becomes zero, then returns.
- `reset_input_state`: The routine walks a table of bytes starting at a fixed address, comparing each byte with a running maximum and using a counter to decide when to stop.
- `disable_dma_channels_2`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `reset_animation_frame_counter`: The routine increments a global counter and, when the counter wraps, it resets several globals and calls another routine four times.
- `update_display_window_stop`: The routine updates a hardware display window stop register and related control words using values from a global variable and several RAM locations.
- `update_sprite_animation_state`: The routine updates animation state for an entity using a table lookup and writes values into a structure.
- `initialize_interrupt_table_2`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers_2`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `process_timer_flags`: The routine reads flag bytes from a structure pointed to by an argument register and updates a timer word in the same structure using a global variable and a small table.
- `copy_word_to_a5_offset`: The routine copies a 16-bit word from an address derived from a register plus a constant offset to an address derived from another register plus a constant offset.
- `set_collision_flag`: The routine looks up a value from a global table using an index derived from a mask of a pointer, then writes that value into a field of a structure pointed to by a register and clears a byte in the same structure.
- `update_timer_and_lookup_value`: The routine updates a short timer field in a structure pointed to by a register using a byte delta, clamps it against a limit field and clears the limit when reached, then builds an index from the limit and a byte field, masks it, and looks up a value in a global table using the timer as a threshold.
- `update_animation_counter`: The routine reads a byte from a structure, indexes a global table or computes a signed offset, scales it by a nibble, adds a base offset, writes the result to another structure, and updates a counter byte in the structure.
- `decode_input_byte`: The routine decodes a byte from a table using a value derived from another routine and the CIAA_PRA register, updates several fields in a structure pointed to by a register, and returns a pointer or value derived from the decoded byte.
- `mask_byte_to_offset`: The routine masks a byte from a global variable with a constant and stores the result into another byte of the same global variable.
- `negate_and_advance_string_2`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine_2`: Contains an infinite loop with no operations that compares values from the VHPOSR register and decrements a counter until it reaches negative one.
