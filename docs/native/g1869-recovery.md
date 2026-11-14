# Game 1869 (flashback (1992)(u. s. gold)[cr interpol][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01869). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3609584 top-level bytes in 245 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 139 of 139 named (0 author symbols), 139 of 139 described (13 by rule, 23 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1445 of 1445 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 47040 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3916 of 3918 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1997489 |
| code | 8856 |
| data | 1410723 |
| padding | 45060 |
| unused | 147456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:ExpLoad | 1 | 257 | 26 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:flashback | 7 | 63 | 1 |
| explode.library | 1 | 475 | 30 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:Failat | 2 | 297 | 11 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:bootblock | 1 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1205 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:INTERPOL#unpacked | 1 | 1205 | 29 |
| level3_2.obc#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf: file data/level2.spc (header 13319 bytes, blocks hold 8784): its blocks are judged as raw blocks outside the filesystem
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: file music/level4 (header 33696 bytes, blocks hold 2762): its blocks are judged as raw blocks outside the filesystem
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: file music/journal (header 47482 bytes, blocks hold 32208): its blocks are judged as raw blocks outside the filesystem
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:data/level1.ct: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (564 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:data/level1.sgd: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (45024 -> 74679 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:data/present.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (26652 -> 44806 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:data/perso.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (103888 -> 178647 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:cine/logos.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2976 -> 4035 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:cine/intro.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (42116 -> 48961 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/replicant.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (36320 -> 55636 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/garde.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (29004 -> 61045 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/level2.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (876 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/level1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (13352 -> 34050 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/junky.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (25140 -> 37142 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:data/level2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (27212 -> 66644 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/pont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4212 -> 4200 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/debut.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12120 -> 13644 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/genexp.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1936 -> 2000 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/memo.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (21052 -> 17712 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/map.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4264 -> 5428 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/missions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (5000 -> 4030 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/holoseq.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7064 -> 7280 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/metro.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2940 -> 3464 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/alimpont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2148 -> 2094 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:cine/serrure.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3512 -> 3873 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/replicant.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (36320 -> 55636 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/garde.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (29004 -> 61045 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/level3_1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (17148 -> 42748 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/dt.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12848 -> 33374 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/junky.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (25140 -> 37142 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/dt.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (728 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/glue.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (24224 -> 36958 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/level3_2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18856 -> 47040 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/level3_1.ct: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (812 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/voyage.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14556 -> 14703 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/espions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7148 -> 6917 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/taxi.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3916 -> 2659 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:data/level4_2.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16264 -> 39106 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:data/level4_1.CT: ByteKiller EOF Data (xfd)-packed, unpacked with ancient (812 -> 7424 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:data/glue.spm: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (24224 -> 36958 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:data/level4_1.obc: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14892 -> 36868 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/chute.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18452 -> 19431 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/desinteg.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1692 -> 1523 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/pont.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4212 -> 4200 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/lift.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2736 -> 2698 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/debut.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (12120 -> 13644 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/genexp.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (1936 -> 2000 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/voyage.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (14556 -> 14703 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/carte.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2296 -> 2539 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/memo.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (21052 -> 17712 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/espions.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (7148 -> 6917 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/log.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2012 -> 1813 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/teleport.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (4528 -> 5088 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/objet.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (16668 -> 18141 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/over.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (736 -> 697 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/gen.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (8704 -> 7853 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/taxi.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (3916 -> 2659 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/asc.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (2472 -> 2166 bytes)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf:cine/fin.cmp: GuidePacker Phaze 2 Data (xfd)-packed, unpacked with ancient (18072 -> 11829 bytes)
- runtime segment an address: 890 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:INTERPOL: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 14180 -> 33324 bytes), 9 of 10 windows confirmed in reference RAM: recovered as a hunk executable
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:flashback: Imploder Library (xfd) stream found by ancient scan (83312 -> 190628 bytes) but only 1 of 90 sampled windows are in reference RAM: not used
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf:data/level3_2.obc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr interpol](disk 2 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf: blocks 565-594 (bytes 289280-304640) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.46)
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4)[a].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: blocks 2-48 (bytes 1024-25088) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.17)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: blocks 1637-1708 (bytes 838144-875008) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.31)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: blocks 1737-1746 (bytes 889344-894464) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.11)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.02)
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4)[a].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)[a]/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 47040 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 13312 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `init_expansion_loader`: Initializes a global structure by storing a value at a fixed offset and setting a counter field to 100, then invokes another routine to complete the setup.
- `process_library_data`: Interacts with explode.library, performs table lookups, and conditionally calls routines through pointers in a global variable.
- `resolve_library_dependency`: Searches for the name of a required library within the system and prepares it for subsequent function calls.
- `call_conditional_hook`: Checks a flag and conditionally calls a routine through a pointer located at a negative offset from a base pointer.
- `initialize_library_logic`: Opens a library and conditionally executes a routine through a pointer in a global variable based on various status checks.
- `call_library_function_indirect`: Conditionally stores a value into a stack-relative slot, then calls a library function through a negative offset from a base pointer.
- `call_hook_and_clear_flags`: Checks a pointer stored in a global variable; if non-zero, it invokes a function pointer located at an offset from that address.
- `clear_flag_and_call_hook`: The routine checks a flag stored at an offset from a global variable using a short index in D0.
- `execute_indirect_call`: Parses a string table entry and calls a routine through a pointer stored in a global variable.
- `parse_table_entry_then_call`: Calls the string-table parser, which reads a zero-terminated string from a pointer table and returns the offset of the next entry.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `perform_multiple_calls`: Calls another routine multiple times with various arguments and stack-based parameters.
- `test_signal_and_callback`: Calls another routine to test a signal flag and subsequently calls a second routine while checking a zero flag.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `store_byte_and_advance_pointer`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `store_return_values`: The routine calls another routine, captures its return value and an extra output pointer, and stores both into two consecutive words of a caller-provided structure via a pointer in A4, then returns the original input value in D0.
- `read_string_advance_pointer`: Reads a zero-terminated string from the source pointer stored in the context block, optionally skipping a leading no-op word, scans forward until the terminator, then writes the advanced pointer back and returns the number of characters consumed plus the terminator.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.
- `set_signal_flag`: The routine calls another routine and tests a returned value for a specific bit.
- `clear_flag_in_structure`: The routine calls another routine and then writes a constant to a field of a structure pointed to by a register.
- `check_and_invoke_callbacks`: The routine calls another routine and then checks a zero flag.
- `store_parameter_indirectly`: Calls another routine and stores the provided parameter into a memory location offset from a returned pointer.
- `call_indirect_via_global_table`: The routine calls another routine through a pointer stored in a global variable.
- `count_string_bytes`: The routine receives a pointer in a0 and walks forward byte by byte until it finds a zero byte, then returns the number of bytes from the start pointer to the terminating zero.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:INTERPOL

- `load_hunk_list_with_relocations`: Walks the AmigaDOS executable hunk list, allocating memory with the exec allocation call for each code, data and BSS hunk and applying relocation records that add a base address to listed longwords.
- `busy_wait_counter_down`: Decrements a counter from its input until it wraps past zero, producing a busy delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_bitstream_runs`: Decodes a compressed bit stream by filling an output buffer from its end.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_read_stream`: Repeatedly shifts a multi-word bit buffer by a count taken from a data register and decrements that count each pass.
- `invalidate_cache_then_store_flags`: Executes a trap and invalidates cache lines through a library call, then stores a negated byte and a flag field.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:flashback

- `initialize_game_state`: The routine checks a global variable and a flag, then calls another routine from a table and handles an error path.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `open_dos_library_into_structure`: Stores its argument in a structure, opens the library named by the referenced text through a library call, and stores the returned base address.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_and_dispatch_handlers`: Decrements a counter stored in a library structure and, when it reaches zero with a certain flag set, copies fields from that structure into a destination buffer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_update_library`: Library code that checks magic values and function pointers within a structure to verify its integrity.
- `set_handler_flag_bit`: Sets one flag bit in a byte of a structure and returns.
- `check_explode_library_name`: Calls an unnamed routine to get an index, then compares the stored longwords of an entry with the characters of the explode library name.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `check_explode_library_name_2`: Calls an unnamed routine to get an index, then compares the stored longwords of an entry with the characters of the explode library name.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_timer_then_call_effects`: Decrements a 16-bit counter field in a structure, calls two routines, and calls a function from a global table when the library version exceeds a threshold.
- `decrunch_block_into_buffer`: Sets up a decrunching state record from a packed header, builds a table of shifted weights, then loops copying literal runs and back-references into the destination buffer.
- `copy_decrunched_runs_loop`: Copies literal runs and back-reference runs into a buffer in a loop, checks a pointer pair and decrements a counter, and advances a second counter at a threshold.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copy_decrunched_runs_loop_2`: Copies literal runs and back-reference runs into a buffer, advancing a counter after each block and returning when the counter reaches a stored limit.
- `copy_decrunched_runs_loop_3`: Copies up to eight words of literal data per pass from the source into the destination, calls for a further run, and returns once a stored block counter reaches its limit.
- `copy_decrunched_runs_loop_4`: Copies literal and back-reference runs into the destination buffer, advancing a block counter and returning when it reaches a stored limit.
- `decode_backref_length_codes`: Decodes length and distance codes from a bit stream, using small tables held in the state record, then copies that many bytes backwards within the output buffer.
- `decode_backref_commands`: Decodes the next length and distance commands from a bit stream, using small tables held in the state record.
- `decode_backref_and_store_state`: Decodes back-reference commands from a bit stream in a loop, copying bytes from earlier in the output buffer and wrapping the source pointer at a buffer boundary.
- `enter_decoder_with_five_count`: Sets a starting count of five and a fixed table index, then falls into a shared decoder loop elsewhere in the library.
- `enter_decoder_mid_loop`: Enters a shared bit-stream decoder loop at a fixed table index.
- `decode_backref_length`: Decodes a small back-reference length from the compressed stream, reading bits and consulting a table to compute the copy length.
- `decode_small_length_code`: Reads bits from the input stream to decode a small length code, storing a class value and a count between zero and three, with a fallback branch when a leading bit is clear.
- `classify_bit_pattern_code`: Reads up to three bits from the stream with nested tests and maps the pattern to a class value and a code between zero and three.
- `decode_extended_bit_pattern`: Reads a bit pattern from the stream with nested tests, storing a class value and a code between zero and three, and extends the class with further bits when the first tests pass.
- `decode_length_extension_bits`: Reads a length code from the bit stream, selects a class value, and extends it with further bits when the leading test passes.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen words from a structure field into a destination buffer when the buffer pointer taken from the stack is non-null, then calls a function pointer.
- `initialize_structure_and_call`: Stores a byte from a table into a structure at offset 12, clears offsets 4 and 16, sets offset 8 to 1, and calls a routine using a pointer from offset 40 of a structure in a2.
- `parse_structure_data`: Increments a value at an offset in a structure and enters a loop that compares bytes from a table at calculated offsets against specific constants.
- `process_structure_data_loop`: Calls a routine using a pointer from a table in a5, then enters a loop that reads bytes from a table in a0 using offsets derived from a structure in a1, updates a value at offset 4 of that structure, and increments a value at offset 8 of that structure until a condition is met.
- `process_structure_data_2`: Sets d2 to 20, d1 to 255, and d0 to 32, then calls a routine using a pointer from a table in a2, checks a long value at offset 16 of a structure in a1, and if it is not zero, negates a long value from offset 4 of a1 and stores it in d1.
- `jump_table_dispatch`: Loads a long word from address a1 plus 4 into d1 and then jumps to the address in a6 using an indirect jump, likely dispatching to a function table.
- `update_structure_index_and_call`: Calculates an index by shifting the address in a1 by two and storing it at offset 4 of the structure in a1, then performs a sequence of three calls to a routine via a pointer in a5 using various offsets from structures in a2.
- `test_and_call_routine`: Tests a long value at an offset in a0 calculated from a structure in a1, and if it is zero, loads a value from a structure in a2, sets up registers, and calls a routine using a pointer from a5.
- `update_structure_field_via_table`: Fetches a value from a table using an index in a structure, calls a routine to process it, and stores the result back into the structure.
- `copy_and_call_routine`: Copies a long value from offset 212 of a structure in a1 to offset 556 of a structure in a2, then calls another routine and jumps to the address in a6.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:endrun

- `open_system_libraries_at_entry`: The program entry opens two system libraries through the exec open call and stores their results in a state structure when both succeed.
- `do_nothing_routine`: It contains no instructions and returns immediately without touching any data or registers.
- `spin_forever_no_exit`: Contains only an infinite loop with no exit or work, so execution stays there once reached.
- `reset_loader_slot_and_call`: When its argument is non-zero it clears a table slot, sets two words to all-ones and calls a library function through a negative offset.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4)[a].adf:bootblock

- `invoke_dos_boot_entry`: It runs from the boot block as the first code executed by the Kickstart strap and calls a routine from dos.library, identified by library number -96.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter_and_wait`: The routine programs the Amiga blitter registers BLTCON0, BLTCON1, BLTBMOD, BLTBPTH, BLTDMOD, BLTDPTH and BLTSIZE and then repeatedly polls CIAA_PRA while performing a small blitter copy loop that moves bytes from a table into a set of memory locations.
- `step_blitter_pointers_on_cycle`: When a state word is zero it sets it to four and advances two 32-bit pointer fields by two; otherwise it subtracts four from the state word.
- `reverse_blitter_pointer_step`: When the state word equals four it resets the word to zero and subtracts two from two 32-bit pointer fields; otherwise it adds four to the state word.
- `render_text_glyphs_to_bitplane`: Copies eight-byte glyph rows from a font table into a destination bitplane for each character, with each glyph row written at a fixed stride.
- `build_copper_list_and_start_dma`: The routine builds a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor`: Opens the graphics library and allocates memory.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_until_port_bit_clear`: Spins while a status bit from the fire-button port register remains set, then returns.
- `wait_keyboard_input`: Waits for specific keyboard input sequence by polling the CIAA_SDR register, first waiting for byte value 57 then for byte value 55, similar to routine R2 but at a different address.
- `wait_until_raster_value_zero`: Spins while a hardware value held in a global reads non-zero, then returns.
- `build_scanline_index_table`: Walks a table of byte values and fills a 30-entry longword table at a fixed address with values derived from a base address and a row stride.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `start_cia_timer_sequence`: Advances a state counter; on one path it clears state, sets CIA timer registers to fixed values and starts a timer, and otherwise it calls an unnamed routine.
- `select_display_window_stop_value`: Calls four unnamed routines, then selects a display window stop value from a state word using a modulo-three test and a mask on a second word.
- `setup_interrupts_and_dma_state`: Initialises interrupt and DMA state by calling an interrupt table initialiser and a helper that copies table words into globals.
- `initialize_interrupt_table`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `advance_signed_field_state`: When a masked word field is zero it copies a value into a second field and returns.
- `copy_position_word_to_field`: Copies a 16-bit field from one structure to another.
- `select_table_step_and_store`: Scans a 16-bit threshold table for the first entry that a masked field does not exceed, steps the index down one place when a flag bit is set, and stores the table value in a structure field.
- `advance_periodic_accumulator`: Adds a byte step to a 16-bit accumulator held in a structure and wraps it against a period value when it passes that value.
- `compute_table_indexed_offset`: Extracts a five-bit field from a structure and uses it as an index into a table of global bytes, then negates or scales the result by eight depending on a mode field and the sign of a second field.
- `update_control_nibble_state`: Reads a low nibble from a structure byte and, when it matches a fixed code, copies a byte field into another field; otherwise it takes a different path.
- `store_low_nibble_field`: Stores the low nibble of a structure byte into another byte field and returns.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine`: The routine contains an infinite loop with no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_ten_instructions`: A ten-instruction routine that calls a library function and is associated with the text endrun flashback.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4)[a].adf:INTERPOL#unpacked

- `initialize_graphics_subsystem`: Initializes Amiga graphics hardware by configuring blitter registers and setting up DMA transfer parameters.
- `increment_timer_counter`: The routine reads a 16-bit counter from a global variable and, if the counter is zero, it resets the counter to four and increments two 32-bit values built from word pairs in the same global structure, returning the high word of the second value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_two_word_counters`: The routine checks a global variable and, when it equals four, clears it and decrements two 32-bit counters stored as paired 16-bit words in a global structure, updating both halves.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
- `decode_text_to_tile_data`: Iterates through a text buffer, skipping newline characters by advancing a destination pointer by a fixed offset.
- `build_copper_list_and_start_dma_2`: Constructs a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor_2`: Opens the graphics library and allocates memory.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_joystick_fire`: Waits for the joystick or mouse fire button to be released by continuously reading the CIAA_PRA register and looping while bit 6 is set.
- `wait_keyboard_sequence`: Waits for specific keyboard input sequence by polling the CIAA_SDR register, first waiting for byte value 57 then for byte value 55.
- `wait_vertical_blank`: Waits for vertical blank by continuously reading and testing the VHPOSR register until it becomes zero, effectively polling for the start of a new frame.
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
- `poll_vertical_position`: Polls the vertical position register multiple times with a countdown loop, checking if the value changes between reads to synchronize with video timing.
