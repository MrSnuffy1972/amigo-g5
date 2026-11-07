# Game 692 (CurseOfEnchantia v1.3 Chip Files 0283): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00692). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (9953891 top-level bytes in 313 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 119 of 119 named (0 author symbols), 119 of 119 described (3 by rule, 116 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1516 of 1563 (96.99%) |
| Recovered without load address (counted in proof 1, flagged) | 436224 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4477 of 4477 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 718372 |
| code | 19550 |
| data | 9215445 |
| padding | 12 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 202 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 984 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2989 | 51 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 192 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 2 |
| disk1_2.dat (raw code file, load address unknown) | 1 | 1 | 1 |
| disk1_3.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| loader.cod (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/disk.2: raw disk image (512 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283.hdf: hard-disk image (AmigaDOS OFS, 10848 blocks): its filesystem parses and all 229 files in it are byte-identical to the extracted tree CurseOfEnchantia_v1.3_Chip_Files_0283_hdf, which is recovered file by file; no block outside the filesystem holds data
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area1.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (108285 -> 124000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area10.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (126330 -> 160000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area11.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (130950 -> 160000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area12.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (120073 -> 152000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area13.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (203071 -> 264000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area15.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (232230 -> 260000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area18.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (130599 -> 168000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area19.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (131401 -> 156000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area1_2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (89851 -> 120000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (27549 -> 40000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area20.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (193257 -> 232000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area3.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (195991 -> 256000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area4.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (65925 -> 80000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area4sfx.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19562 -> 25106 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area5.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (91820 -> 116000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area5sfx.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15433 -> 22156 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area6.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (106428 -> 136000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area7.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (78379 -> 100000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area8.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (81436 -> 108000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/area9.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (86340 -> 104000 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar1.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9826 -> 24272 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar10.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (3768 -> 9632 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar11.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9360 -> 23704 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar12.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10061 -> 24008 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar13.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10113 -> 28072 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar15.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (12573 -> 29648 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar18.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (10769 -> 24396 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar1_2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15768 -> 35764 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1687 -> 5684 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar20.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5950 -> 20904 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar3.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9793 -> 24136 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar4.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7854 -> 15924 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar5.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1690 -> 4532 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar6.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (528 -> 1940 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar7.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1340 -> 4312 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar8.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1796 -> 5800 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/br-ar9.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6834 -> 19696 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/endspr.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (14856 -> 62376 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh1.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2994 -> 5304 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh10.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (9875 -> 14612 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh11.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (2208 -> 3808 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh12.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8475 -> 13512 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh15.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (8133 -> 14268 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh18.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (7312 -> 11716 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh19.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (5470 -> 9124 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh1_2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (537 -> 748 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4316 -> 6732 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh3.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7008 -> 13560 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh4.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6560 -> 9548 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh6.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6638 -> 10108 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh7.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6630 -> 10708 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh8.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8835 -> 12788 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/icons1.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14381 -> 28080 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/icons2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12501 -> 28080 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj1.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (48754 -> 98612 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj10.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (16529 -> 37776 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj11.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (43744 -> 84536 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj12.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (26371 -> 65488 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj13.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (34025 -> 87068 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj14.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (37002 -> 99964 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj15.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (34324 -> 87480 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj16.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (27396 -> 67920 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj17.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (38612 -> 99360 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj18.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (44802 -> 99940 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj19.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (31131 -> 53880 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj1_2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (38174 -> 95292 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj2.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15378 -> 51160 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj20.pr2: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (48080 -> 99876 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj3.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (44514 -> 91172 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj4.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (42578 -> 99896 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj5.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (16215 -> 29996 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj6.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25933 -> 93008 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj7.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30386 -> 75768 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj8.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12167 -> 27600 bytes)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/obj9.pro: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (43554 -> 89304 bytes)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 436 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 839 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/loader.cod: 69 executed PCs whose RAM was overwritten before any snapshot match its bytes 2512-2976: recovered as a unit run at an address
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/disk1_2.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/disk1_3.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/loader.cod: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/gbh1_2.spr: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- CurseOfEnchantia_v1.3_Chip_Files_0283/CurseOfEnchantia_v1.3_Chip_Files_0283_hdf/CurseOfEnchantiaChipFiles/data/standard.fxt: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 3 never-loaded region(s), 436224 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_negative_one`: The routine returns a constant negative one value.
- `initialize_game_state`: The routine stores the incoming pointer in a global variable and calls several functions through offsets in that structure.
- `clear_global_variable`: The routine writes a constant to a global variable, then writes zero to the same global variable, then writes another constant to the same global variable and returns.
- `reset_globals`: The routine writes a constant to a global variable, then writes zero to the same global variable, then writes another constant to the same global variable.
- `initialize_jump_table`: The routine writes constant values into several global variables and memory locations, sets a pointer to another routine under a condition, and then performs an indirect jump through a table whose address is recovered from memory, subtracting an input value from a table entry before calling through it.
- `return_concatenated_registers`: The routine calls another routine via a table entry and then calls another routine, returning the concatenation of the two input registers.
- `invoke_table_entry_and_call`: The routine invokes a function pointer taken from an offset in a table and then calls another routine.
- `check_flag_and_dispatch`: The routine checks a short value in D3 against a sentinel and calls another routine, then dispatches to one of two entries in a table via a global variable, calls another routine, and returns the concatenated D0 and D1 values.
- `invoke_callback_table_entries`: The routine calls another routine, then calls a routine via an entry in a table of function pointers, then calls another routine again.
- `pack_word_pair_to_long`: The routine combines two 16-bit values from registers into a 32-bit result by scaling each with a constant factor and forming a high/low word pair.
- `call_indirect_table_entry`: The routine invokes a function through an indirect table entry and then calls another routine, returning the value that was in D1 on entry.
- `check_and_set_callback`: The routine checks a global variable and, if it matches a specific value, writes a different value to that variable and stores the address of another routine into a separate global variable.
- `set_display_mode_and_colors`: The routine checks a derived value from a global variable and, when it matches, writes a sequence of values to several custom chip registers and to a global variable, configuring the display mode and palette registers.
- `unknown_routine`: The routine calls another routine via a table and returns a 64-bit value formed by concatenating two 32-bit inputs.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_sprite_flags`: The routine writes zero to a set of sprite flag bytes referenced through a global pointer.
- `clear_audio_dma_state`: The routine clears a table of DMA pointers and resets a set of global variables, then writes to the audio volume registers and the CIA port register and updates the DMACON register to enable DMA channels.
- `reset_animation_state`: The routine resets a global flag and a counter, then clears a set of RAM locations and performs two short busy-wait loops.
- `update_audio_channel_pointers`: The routine updates four Amiga audio channel pointers AUD0LCH through AUD3LCH by calling four other routines, then manipulates a set of global variables and a small table indexed by a byte from a global variable.
- `process_audio_channel`: The routine reads a byte from a table using a nibble from a control word, copies several fields from a per-channel table into a workspace, computes a start address and length for a sound sample, and writes the length and channel pointers to the Amiga audio hardware registers.
- `process_animation_frame`: The routine updates a 16-bit animation counter stored in a structure pointed to by a register, using flags and masks from the same structure and a global variable.
- `copy_word_from_table`: The routine copies a 16-bit word from a table pointed to by a register to a location pointed to by another register.
- `find_next_table_entry`: The routine walks a table indexed by a byte from a pointer and a short counter, skipping entries whose value is greater than a masked word from the pointer, then selects an entry and writes it to a word field of the pointer structure, clears a byte field, and sets a flag byte based on comparison with another word field.
- `update_timer_and_select_table_entry`: The routine updates a 16-bit timer field in a structure pointed to by a register using a signed byte delta, clamps the timer to a limit field and clears the limit when reached.
- `compute_scaled_offset`: The routine reads bytes from a structure pointed to by a register, selects an index from a table or computes a value based on flags, scales it with a nibble and a sign, adds a base offset, stores the result, and updates a signed accumulator byte in the structure.
- `decode_input_byte`: The routine reads a byte from a table pointed to by a register, masks it to obtain a command nibble and then performs a series of command-specific updates to fields in the same table and to a global variable.
- `mask_byte_to_table_entry`: The routine masks the low nibble of a byte read from an offset in memory and stores the result into another byte offset in the same memory block.
- `apply_signed_delta_to_pointer`: The routine reads a nibble from a byte at a fixed offset in a structure, uses it as an index into a global table to fetch a signed delta, adds that delta to a signed byte field in the same structure, and if the result is negative it clears the field, adjusts a pointer field by moving it forward by a computed stride or falling back to a base pointer, and toggles the sign of the pointed-to byte.
- `update_routine_pointer`: Stores the address of a routine into a global variable.
- `empty_routine`: The routine contains no instructions and returns immediately.
- `increment_global_and_dispatch`: The routine increments a global variable twice, calls another routine three times, then calls a function through a pointer derived from the global variable, sets the global variable to a constant, and returns the combined input registers.
- `set_global_variable_repeatedly`: The routine repeatedly stores the address of another routine into a global variable seven times and then returns.
- `clear_global_variable_2`: The routine writes the same constant to a global variable twice and then returns.
- `reset_interrupt_and_dma`: The routine writes to a global variable three times and then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `clear_global_variable_3`: The routine writes a constant value to a global variable and then writes zero to the same global variable three times in succession, leaving the variable cleared.
- `write_debug_byte_to_global`: The routine stores a debug byte into a global variable and then writes the same byte into a second global location, indicating a small debug or flag update.
- `recursive_call_with_global_store`: The routine calls another routine, then stores the constant 8 into a global variable.
- `return_immediately`: The routine is empty.
- `update_timer_flag_and_byte`: The routine checks a global variable and, if a flag is set, reads a byte from another routine, transforms it, and writes the result through a pointer, updating the global variable to indicate completion.
- `set_flag_byte`: The routine compares the low seven bits of a byte in a register with a byte pointed to by a pointer, and writes the register byte to the pointer either at offset one or at offset zero depending on the comparison.
- `store_byte_via_pointer`: The routine stores a byte from a register through a pointer, choosing the offset based on whether the byte is zero.
- `set_global_flag_on_bit`: The routine tests bit zero of a value in D0 and, if set, writes the constant 8 to a global variable.
- `decode_input_state`: The routine reads the joystick and mouse ports and the CIAA port A register, then updates a global variable with derived button and movement bits using bitwise operations.
- `read_joystick_buttons`: The routine reads the joystick and mouse button states from the CIA and POT registers, combines them into a single byte and stores the result in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_and_dma`: The routine sets up display window registers and DMA control, enables and disables interrupts, and repeatedly calls other routines while initializing a global variable table and copying blocks of memory.
- `check_global_and_loop`: The routine checks a global variable against several constant values and, if the variable is not zero, it clears or sets the variable.
- `clear_flag_bits`: The routine manipulates a single global byte, clearing specific bits under various conditions and forcing the value to zero, one, or all bits set.
- `call_until_flag_cleared`: The routine calls another routine repeatedly in a loop until a 16-bit value from a register is zero.
- `populate_palettes_or_tilemaps`: The routine copies 200 entries of 37-byte records from a table selected by a byte to an output buffer, using an index multiplied by a fixed stride and a base global pointer; if the selector byte is a sentinel value it zero-fills the same 200 entries.
- `adjust_camera_offsets`: The routine adjusts two signed offset variables in RAM based on a global control variable that encodes directional flags and character commands, clamps the offsets to fixed limits, resets the control variable, and then derives two byte values from the offsets by adding constants and masking, storing the results back into the global variable.
- `clamp_and_pack_palette_flags`: The routine clamps two signed short values stored in RAM to fixed ranges and optionally resets them when a global variable equals a specific value, then clears the global variable.
- `reset_and_load_sprite_offsets`: The routine resets a global variable and then loads two byte offsets from a small RAM table into the same global variable, clamping each to a maximum of 255 and applying a small bit mask derived from another table entry.
- `copy_two_blocks_from_source`: The routine copies sixteen 32-bit words from a source area derived from a global variable and an input pointer into two fixed destination tables, then returns the original input value unchanged.
- `copy_data_blocks`: Decrements a global variable and copies two or more blocks of data from a source address to a destination address.
- `adjust_ram_counter_and_flags`: The routine reads a short from RAM, conditionally adjusts it using a global variable and several flag tests, writes the adjusted value back and may set a sentinel value and clear a global flag.
- `apply_delta_to_two_tables`: The routine adjusts two tables of short entries by adding a signed delta to the second field of each non-zero entry.
- `initialize_game_state_2`: The routine clears a block of memory, resets several global variables and then loads a set of values from a table indexed by a global variable into other globals and RAM locations.
- `reset_ui_buffers`: The routine clears several global variables and a set of RAM locations, then writes a series of space bytes into a table derived from a saved global value and writes a digit byte into another table entry.
- `sort_table_entries`: Performs a comparison-based sort on a table of pointers, swapping entries if a certain sum of values at an offset is lower.
- `set_blitter_copy_parameters`: The routine computes source and destination pointers and size values from a table entry and writes them to custom chip registers BLTCON0, BLTCON1, BLTSIZE, BLTAPTH, BLTAMOD, BLTBPTH, BLTBMOD, BLTCPTH, BLTCMOD, BLTDPTH, BLTDMOD, BLTAFWM and BLTALWM to configure a blitter copy operation.
- `process_sprite_collision_table`: The routine walks two tables of short entries, marks entries and compares signed 16-bit sums to decide when to set a flag.
- `set_sprite_animation_frames`: The routine writes several short and long fields into a structure pointed to by a register, using values derived from two input registers, and calls another routine multiple times with the structure updated.
- `reset_global_state_and_iterate_table`: The routine resets a global variable and several memory locations, then calls another routine.
- `reset_blitter_state`: The routine writes several values to a global variable and to chip registers, clears a few RAM locations, and then calls another routine.
- `wait_for_flag_clear`: The routine spins until a global variable has its bit 6 cleared, then returns.
- `process_script_entries`: The routine iterates over a table of script entries at a fixed address and processes each entry whose first word and a pointer field are non-zero.
- `process_script_table`: The routine iterates over a table of script entries stored in a global variable.
- `process_script_entries_2`: Iterates a table of script entries and updates each entry’s state from a referenced script buffer, handling control characters and terminating conditions.
- `initialize_global_constants`: Checks a global variable against a constant and, if matching, resets several global variables to specific values.
- `decrement_global_if_nonzero`: The routine checks a global variable and, if it is non-zero, decrements it by one and returns.
- `process_collision_response`: The routine reads and writes a set of global variables and performs arithmetic and conditional tests on them, including comparisons with character constants and bit masks.
- `fade_palette_to_target`: The routine adjusts a block of palette words toward a target block by stepping each nibble toward the target value, repeating the step for a fixed number of entries and iterations.
- `search_valid_bounds`: Calculates an offset and checks a list of pointer-based structures to see if a value falls within certain range limits or matches a specific byte.
- `validate_position_bounds`: The routine checks two signed differences derived from two input values and two pairs of short values stored in a global variable.
- `process_input_character`: The routine reads a byte from memory pointed to by a register, increments it and returns a transformed character based on a global variable and a series of character tests, with some branches writing constants to a global variable and invoking another routine recursively.
- `handle_input_character`: The routine reads a character from the input register and dispatches on its value, returning the input or a transformed value based on flags in a global variable and the result of calling through a function pointer table.
- `scramble_and_mask_return`: The routine stores a word from a register through a pointer, writes an offset into memory, reads a dword from a calculated address into a pointer, then scrambles a word at an address using a dword from a register and a word from memory before masking a register value and returning it.
- `modify_pointer_and_register`: Updates a pointer or a global variable by adding an offset and performs bitwise operations on a register or global variable.
- `increment_three_counters`: The routine increments a global counter and then adds constant values to three separate global variables.
- `scramble_global_value`: Calculates a complex bitwise-rotated value using the routine address and a global variable, then stores the result in a global variable.
- `clear_global_flags_and_constants`: The routine clears a bit in a global variable and then overwrites the same global variable with a series of constant values, ending with the value 6.
- `reset_global_flag`: The routine writes zero to a global variable and then returns.
- `empty_routine_2`: The routine contains no reachable instructions and is effectively empty.
- `empty_routine_3`: The routine contains no reachable instructions and ends in an infinite loop, so it performs no observable work.
- `set_display_window_parameters`: The routine writes a sequence of constant values to a global variable and then returns.
- `initialize_global_table`: The routine writes a series of constant values to a global variable, effectively initializing a table with five distinct entries followed by five repeated entries.
- `decrement_nibble_table`: The routine clears a global variable and then repeatedly processes a small fixed-size block of words starting at that variable.
- `smooth_palette_transition`: The routine repeatedly steps a 32-entry palette table toward a target table by adjusting each nibble of each 16-bit entry one step at a time, using a global variable as the working copy.
- `update_table_differences`: Iterates through a range of memory and modifies a global variable by adding or subtracting certain bit-masked values to match the contents of a source pointer.
- `align_pointer_to_pattern`: Adjusts a pointer or global variable by incrementing or decrementing it until its lower nibbles or higher nibbles match a source value.
- `align_table_to_source`: The routine copies a table from a source pointer to a destination pointer, adjusting each destination word so its low nibble, high nibble and high byte match the source word by incrementing or decrementing the destination word in small steps.
- `align_memory_values`: Compares a source pointer to a destination pointer and increments or decrements the destination pointer or its value until a certain bit-mask condition or equality is met.
- `clear_and_copy_color_table`: The routine clears a block of 32 words in a global variable and then copies 32 words from that global variable to a table at a fixed address, touching COLOR00.
- `copy_word_table`: The routine copies 31 words from a global variable table to a fixed memory table.
- `copy_32_word_table`: The routine copies 32 words from a global variable to the same global variable by moving a source pointer and a destination pointer in lockstep.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_undefined_word_to_global`: The routine copies a word from a register to a global variable and returns.
- `reset_global_state`: The routine writes a constant value to a global variable twice, then clears it twice, then writes another constant value to the same global variable twice, then clears it twice, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_short_to_object_field`: Adds a short from a register to the 10-byte offset field of a pointed-to structure and stores the result back.
- `dispatch_input_by_type`: The routine reads a type field from the input structure and, for each recognized type, calls another routine via a function pointer taken from a global variable and then writes a few bytes or words derived from an input word into fixed offsets of an output structure, sometimes also copying words from a second pointer and clearing a type field.
- `apply_input_parameter_effect`: Checks an input type code and applies a corresponding modification to a set of object or or attribute-related offsets, such as incrementing or setting values based on a masked or shifted input byte.
- `apply_parameter_effects`: The routine reads a short value from an input structure and, based on that value, copies fields from another structure, writes masked bytes from a register into several output bytes, and sets small integer fields in the output structures.
- `apply_item_effects`: The routine reads a short value from an object structure and, for a set of specific codes, calls another routine via a table and writes masked bits from a register to fields of the same or related object structures.
- `apply_effect_by_type`: The routine reads a type field from a structure pointed to by a0 and, depending on the value, calls another routine via a function pointer table and writes masked bits from d7 into fields of the structure or a related structure.
- `handle_event_flags`: The routine reads a short flag from an input structure and, depending on its value, updates several output structures with masked values from a register, clears flags, and invokes another routine via a global variable table.
- `apply_effects_by_type`: The routine reads a type field from a structure pointed to by A0 and, after calling another routine via a function pointer from a global variable, updates several byte fields in the same structure based on D7.
- `apply_random_to_three_bytes`: The routine reads a short value from an object structure and, depending on its value, calls another routine via a function pointer stored in a global variable.
- `apply_damage_to_target`: The routine checks a short field in the input structure against three distinct values.
- `apply_modifier_to_three_bytes`: The routine checks a short field in a structure pointed to by a register.
- `clear_flag_and_accumulate`: The routine checks a two-byte field in a structure pointed to by a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_item_to_table`: The routine searches a table of entries in a global variable for an empty slot and, if found, copies a fixed set of fields from a source structure pointed to by a register into that slot and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `return_immediately_4`: This routine is empty and performs no operations before returning to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_copy_data`: copies several blocks of data from various memory locations to a global table and performs hardware register writes including DMA and interrupt configurations.
- `initialize_and_run_engine_loop`: Performs hardware initialization including color table setup, copper list activation, and DMA resets before entering a main loop.

### disk1_2.dat (raw code file, load address unknown)

- `return_uninitialized_register`: This routine is empty and returns the current value held in a data register without performing any operations.
