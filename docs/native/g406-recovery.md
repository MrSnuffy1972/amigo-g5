# Game 406 (Ishar 2 - Messengers of Doom): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00406). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2750016 top-level bytes in 349 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 61 of 61 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | FAIL, 400 of 759 named (0 author symbols), 400 of 759 described (20 by rule, 374 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3360 of 3388 (99.17%) |
| Recovered without load address (counted in proof 1, flagged) | 2352250 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11401 of 11420 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 85791 |
| code | 42908 |
| data | 2505554 |
| padding | 45619 |
| unused | 70144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 10 | 2 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:loading | 2 | 267 | 8 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:    | 1 | 642 | 36 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:Failat | 2 | 297 | 11 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X | 10 | 250 | 5 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9914 | 695 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IBOISHAR.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT2.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESOUND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPECHAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:FRISE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:THEEND1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EATT.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIN.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOSTGAME.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTINE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMONTAGN.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:OBJET.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGEE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:GOLEM1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SDONJONS.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP11.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP12.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPECHAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:VILLAGE1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MARCHAND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BOAT.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LEZARD1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIN.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MEDUSA5.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTINE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PLAINE1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DRAGON5.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SAUB.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:POURS1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FORET1.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SVILLE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SPAROLES.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPECHAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MARCHAND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP2I.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIN.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3MN.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIND.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTINE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MAP.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SAUB.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ENT2.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3TB.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE.CO#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf: file s/startup-sequence (header 22 bytes, blocks hold 29): its blocks are judged as raw blocks outside the filesystem
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IBOISHAR.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (85842 -> 228160 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2278 -> 4512 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GERDEP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (3990 -> 8736 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EAFF.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1650 -> 3304 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (14636 -> 42688 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAIN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (10254 -> 23976 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (37122 -> 64344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESOUND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (22648 -> 45856 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5792 -> 11904 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPECHAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6400 -> 14840 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GESORT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5328 -> 10776 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (38704 -> 79040 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MONTAGNE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (52942 -> 116760 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PARAM.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4314 -> 15288 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GEANT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13432 -> 24800 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:FRISE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (26100 -> 55176 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:THEEND1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (66970 -> 126920 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EATT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2756 -> 5952 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:NAIN4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7326 -> 13200 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:FMONT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2854 -> 10920 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:AFFOBJ.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (852 -> 1592 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EDPLT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2148 -> 4696 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8516 -> 16888 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOGO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8020 -> 21064 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SCOMB.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (11426 -> 11864 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:HORNY4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13384 -> 20344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17302 -> 29208 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8158 -> 16544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOSTGAME.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (20244 -> 49904 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GUARD4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6994 -> 13568 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PIAF4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5950 -> 11288 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:RHINO4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9340 -> 18688 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SOURIS.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2050 -> 3880 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTINE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8172 -> 16304 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMER.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (19768 -> 30000 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:DRUIDE4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9926 -> 21808 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (43360 -> 116936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGED.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5830 -> 12016 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPTRAVEL.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7088 -> 14656 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BOB4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2814 -> 4992 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMONTAGN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (20752 -> 34792 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:OBJET.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (25040 -> 54272 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGEE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5674 -> 11544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (23162 -> 46344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ODWOMAN5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2308 -> 5384 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:IO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2278 -> 4512 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:AVEUGLE5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1668 -> 3592 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:GOLEM1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (19008 -> 41032 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:OLBAR.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (878 -> 2376 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WITCH5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5912 -> 11552 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SDONJONS.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (11724 -> 25792 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PEOPLE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2512 -> 4992 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP11.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17728 -> 33576 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FLY1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5786 -> 11584 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PEOPLE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4058 -> 8544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP12.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (10504 -> 18936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPSAC.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (18240 -> 36848 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPECHAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6400 -> 14840 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:VILLAGE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (24808 -> 62768 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJCOL.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6328 -> 11240 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PUIT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5020 -> 11512 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MARCHAND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (32550 -> 54288 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJTRAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6226 -> 10176 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MONSTER5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7622 -> 16504 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FOND1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1628 -> 6696 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BOAT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (15418 -> 41104 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LEZARD1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (11938 -> 24056 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LIONMAN5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9896 -> 17200 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ELMTFEU5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (10788 -> 24240 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJFONT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8282 -> 18160 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJDEC.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (3602 -> 10536 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJ2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (36480 -> 84728 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SKELETT5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4264 -> 7936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:KROG.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8964 -> 21656 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ELMTAIR5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8642 -> 21608 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FDJ1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1852 -> 5744 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8516 -> 16888 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:AIGLE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4336 -> 8872 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PCAVE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8816 -> 24192 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FDJ2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1486 -> 4720 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MEDUSA5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2482 -> 6464 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MOMY5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (3066 -> 6104 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FDJCAT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6334 -> 16112 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SORTMOR5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (10024 -> 19464 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SILE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (18914 -> 33520 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17302 -> 29208 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8158 -> 16544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ORK5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7030 -> 12328 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WIZARD5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (12232 -> 22632 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DWIZARD5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7842 -> 15920 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TAVERNE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17376 -> 28936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTINE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6344 -> 16304 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PLAINE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (25232 -> 55208 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SMER.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (19768 -> 30000 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DRAGON5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (15008 -> 36184 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (43360 -> 116936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJ1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (36656 -> 81384 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SAUB.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (33042 -> 59176 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:CHAOSGU5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (16952 -> 36208 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SORCIER5.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7414 -> 21440 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WIZARD1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5336 -> 11304 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPTRAVEL.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7088 -> 14656 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:POURS1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13742 -> 29352 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ANIMALS.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4978 -> 9512 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FORET1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (24660 -> 50072 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJCAT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (27360 -> 65872 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (23162 -> 46344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:IO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2278 -> 4512 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:FARBO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2700 -> 7872 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VMAIRIE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9058 -> 18256 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SVILLE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8380 -> 17864 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VHOUSE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (19992 -> 36864 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:HOMOIS2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7296 -> 15176 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:FVILLE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (3574 -> 6568 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SPAROLES.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9240 -> 21952 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:DRUIDE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (9930 -> 22544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VHOUSE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (20164 -> 39928 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:RELICMAN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2452 -> 5048 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPORT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (15810 -> 33056 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PEOPLE1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2512 -> 4992 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3BAN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (31422 -> 51704 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VBIBLI.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (16690 -> 33080 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VARCHE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (16492 -> 30768 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ARBO.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (46210 -> 116048 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PEOPLE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4058 -> 8544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPECHAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6400 -> 14840 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:LUTIN3.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (3338 -> 6416 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:EWOK2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4638 -> 7792 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VARCADE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (18610 -> 38560 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MARCHAND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (32550 -> 54288 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPONT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8066 -> 13992 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPASSAGE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (4690 -> 9768 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VILLE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (21656 -> 39160 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VNIGHT.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (16300 -> 28504 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP2I.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (32892 -> 61872 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TROUBA3.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7736 -> 14400 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:DWOMAN2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6544 -> 10696 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MARIN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (2004 -> 4488 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SHANDAR.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (786 -> 1800 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SCITE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13148 -> 20552 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8516 -> 16888 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VLINGE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (1626 -> 6040 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3MN.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (29306 -> 61056 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:GUARD3.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8232 -> 16264 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC1.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7692 -> 19344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:HORNY4.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13384 -> 20344 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17302 -> 29208 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIND.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8158 -> 16544 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (5604 -> 12696 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTINE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (8172 -> 16304 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TAVERNE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (17376 -> 28936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC3.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7436 -> 14536 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SMER.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (19768 -> 30000 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MAP.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (43360 -> 116936 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:GUARD3HI.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (13184 -> 26472 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VJAIL.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (11374 -> 18944 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SAUB.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (33042 -> 59176 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ENT2.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (12540 -> 27584 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPTRAVEL.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (7088 -> 14656 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3TB.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (31170 -> 52968 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VTEMPLE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (14822 -> 25584 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VOLEUR3.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (6100 -> 13064 bytes)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE.CO: Silmarils Data Cruncher (xfd)-packed, unpacked with ancient (23162 -> 46344 bytes)
- runtime segment an address: 3140 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:loading: PowerPacker 4.0 (xfd) stream found by ancient scan (23492 -> 76140 bytes) but only 0 of 22 sampled windows are in reference RAM: not used
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X: Imploder (xfd) stream found by ancient scan (29552 -> 62592 bytes) but only 6 of 28 sampled windows are in reference RAM: not used
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IBOISHAR.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT2.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESOUND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPECHAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:FRISE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:THEEND1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EATT.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIN.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOSTGAME.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTINE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMONTAGN.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:OBJET.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGEE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:GOLEM1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SDONJONS.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP11.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP12.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPECHAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:VILLAGE1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MARCHAND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BOAT.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LEZARD1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIN.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MEDUSA5.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTINE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PLAINE1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DRAGON5.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SAUB.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:POURS1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FORET1.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SVILLE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SPAROLES.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPECHAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MARCHAND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP2I.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIN.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3MN.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIND.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTINE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MAP.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SAUB.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ENT2.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3TB.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE.CO#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate, 1 displacement
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf: blocks 838-838 (bytes 429056-429568) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.00)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 53 never-loaded region(s), 2352250 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `xor_self_with_value`: Xors the instruction at the current program counter with a byte that decrements every iteration, looping for a fixed number of iterations.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d5.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:loading

- `parse_amigadp_hunks`: Parses AmigaDOS hunk type codes to allocate memory and relocate or copy executable segments.
- `decrement_register_to_zero`: Decrements a register or variable until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_data`: Iteratively reads or writes bytes from a memory area using a loop, checking flags or bit-shifted values and copying data between memory locations or updating a global variable.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs complex bitwise shifts and logical operations on registers and memory using a loop counter.
- `update_memory_with_register`: Performs a system trap, invalidates cache lines, and stores a byte or word to an address relative to a pointer and a register.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:   

- `copy_function_block`: If a certain condition is met after calling another routine, it copies a block of memory from a source pointer to a destination pointer and then calls another routine.
- `execute_recursive_calls`: This routine performs a sequence of self-referential calls to itself, effectively creating an infinite or deep recursion loop or a chain of repeated calls.
- `xor_table_with_mask`: Copies a block of data from a global variable to a destination table while xor-ing each word with a constant mask.
- `update_counter_and_hardware`: Performs bitwise operations on a hardware serial register and updates a global pointer or counter.
- `store_pointer_and_value`: Stores a pointer or a value into a global variable or a memory address pointed to by a global variable.
- `dereference_and_store_pointer`: Loads a pointer from a global variable, calls the routine located at that pointer, and stores the return value into a global variable at a fixed offset.
- `update_function_pointer`: Updates a global pointer with a new address if the new address falls within a certain memory range.
- `return_empty_subroutine`: the routine contains no executable instructions and performs no operations before returning.
- `calculate_checksum_and_clear_memory`: Clears a memory block and calculates a checksum of a code range, storing the negative sum at the end of a pointer-based loop.
- `access_hardware_registers`: Accesses the CIAA or POT hardware registers.
- `modify_hardware_registers`: Accesses a hardware serial register and a color register while performing bitwise operations on custom hardware registers.
- `process_memory_and_registers`: Modifies CIAA_PRA and CIAB_PRB or custom hardware registers while executing a sequence of instructions that perform bitwise XOR operations on a data block and calculate a checksum or parse hunk headers.
- `decrypt_self_modifying_code`: Iteratively XORs a sequence of instructions with a decreasing byte value until a loop counter reaches a terminal value.
- `parse_amigadot_hunk_headers_3`: Parses executable hunk headers to calculate relocation offsets and update a relocation or jump table.
- `update_relocation_table`: Calculates an offset or size-based jump and writes a sequence of bytes to a memory buffer.
- `validate_amigadot_hunk_types_2`: Iterates through a linked list of data structures to check if specific memory offsets match certain AmigaDOS hunk type codes.
- `validate_amigadot_hunk_types`: Iterates through a linked list of data structures to check if certain memory offsets match specific AmigaDOS hunk type codes.
- `parse_amigadot_hunk_headers_2`: Iterates through a list of data structures to check for specific AmigaDOS hunk type codes.
- `process_amigadot_hunk_headers`: Iterates through a list of data structures and checks for specific AmigaDOS hunk type codes.
- `parse_amigadot_hunk_headers`: Iterates through a list of data structures, reading or copying certain values or instruction bytes to a destination buffer while checking for specific type codes.
- `shift_and_store_register`: Shifts the value of a register left by nine bits and stores the result into a memory location relative to a pointer.
- `store_accumulated_sum`: Calls another routine to get a sum and stores that sum into a memory location relative to a register.
- `accumulate_pointer_values`: Iterates through a list of values pointed to by a register, subtracting them from an accumulator and returning the final sum.
- `compare_string_with_case_insensitivity`: Iterates through a memory buffer, converting certain characters to lowercase and comparing them against a global variable until a mismatch or end-of-buffer occurs.
- `calculate_absolute_difference`: Calculates the absolute difference between a register value and a constant and returns the result.
- `clear_bits_based_on_string_nulls`: Iterates through a string, and if a null terminator is found, clears a corresponding bit in a global bitmask, repeating this for a range of memory.
- `find_string_length_and_set_pointer`: Calculates the length of a string located via a pointer and stores the resulting address into a memory location relative to another pointer.
- `execute_library_function`: Calls a library function using a pointer from a global or library-derived address and converts a result to an ASCII digit.
- `convert_to_ascii_digit`: Calls a routine several times and converts the lower byte of the result to an ASCII digit, storing it in a memory location.
- `convert_byte_to_ascii`: Adds a constant offset to a byte in a register and stores the result at a memory location pointed to by a register.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_memory_block`: Sets a sequence of longwords in memory to zero in an infinite loop.
- `sub_0B2C`: undescribed.
- `tiny_ori_2`: Small routine of 1 instruction that merges 21 into (a5)+.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 40 into d1, then merges 24 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of longwords from a memory location or pointer or a table to a stack-based or global memory location.
- `sub_1_0002`: undescribed.
- `sub_1_0030`: undescribed.
- `sub_1_00C4`: undescribed.
- `sub_1_0124`: undescribed.
- `sub_1_0142`: undescribed.
- `sub_1_0148`: undescribed.
- `sub_1_019A`: undescribed.
- `sub_1_01C4`: undescribed.
- `sub_1_0212`: undescribed.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements an uninitialized local variable until it reaches a specific value and then returns.
- `expand_and_copy_memory`: Performs a complex loop that checks certain bit patterns and copies or expands data blocks while checking a global variable.
- `copy_memory_block`: copies a block of memory from a source pointer to a destination pointer using a decrementing counter.
- `shift_and_copy_block`: This routine implements a bit-shifting or bit-replication-style block copy, where bytes or bits are propagated or shifted using a sequence of left-shift operations and conditional checks.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:bootblock

- `initialize_boot_resources`: Calls a function from a library and, if it returns a pointer, sets three dwords at that location to a maximum value before calling another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_values`: Stores the contents of two even-numbered registers into a global variable or a table.
- `empty_routine`: this routine is empty and performs no operations before returning.
- `traverse_and_scale_pointer`: Decrements a counter and updates a pointer by a scaled value in a loop until the counter reaches a certain value.
- `return_registers`: Returns the contents of two even-numbered registers as a single joined value.
- `sub_0098`: undescribed.
- `check_string_pattern_2`: Checks a buffer for specific byte sequences and writes a newline character to a fixed offset if a pattern matches.
- `check_string_pattern`: Checks a buffer for a specific sequence of bytes and writes a newline character to a fixed offset if the pattern matches.
- `process_string_list`: Iterates through a list of pointers and performs string-based checks and modifications on the data or associated pointers.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `validate_and_initialize_memory_pointers`: Checks the return or success of multiple memory allocation calls and, if successful, calculates and stores a series of offset pointers or buffer boundaries into global variables.
- `execute_function_pointer`: Calls a function pointer and updates a global variable with a pointer or a constant value before calling another function.
- `store_mouse_input`: Loads a value from a global variable into a register, which is otherwise used for mouse port movement.
- `do_nothing_2`: This routine is empty and performs no operations before returning.
- `setup_relocation_tables`: Initializes several global pointers, allocates memory via a library, and sets up relocation or header-related pointer offsets.
- `copy_and_set_pointers`: Copies a block of memory and updates several global pointers or values, potentially disabling certain hardware DMA or interrupts.
- `find_value_in_table`: Searches a table of shorts for a specific value and returns the number of steps taken or a negative error code.
- `do_nothing_3`: the routine is empty and performs no operations before returning.
- `apply_scaled_transform`: Calculates a value using a multiplication and addition, then stores the result as a dword into a table, while also modifying a hardware interrupt enable register.
- `update_pointer_and_string_tables`: Iterates through a table to update pointers with an offset-based calculation and writes a string of characters to a memory buffer.
- `do_nothing_4`: The routine is empty and performs no actions.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_bit`: Loops until a specific bit in a certain memory location or hardware register is set.
- `wait_for_flag`: Loops until the lowest bit of a certain memory location or hardware register is set.
- `set_interrupt_enable_bit`: Writes a value to a hardware register to enable certain bits, such as disabling or enabling interrupts.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `write_instruction_sequence`: Writes a short sequence of instructions to a memory location and then executes a series of function calls, including a call to an external function via a pointer retrieved from a global table.
- `execute_sequence`: Executes a sequence of routine calls and updates a global variable with a constant value before calling a function via a pointer.
- `reset_and_recurse_pointer`: Sets a global variable to zero and calls a routine pointer retrieved from a table offset by that global variable.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_and_recurse`: Increments a global variable and then performs a series of recursive or repeated calls to another routine.
- `set_interrupt_register`: Sets a specific value into a hardware register.
- `sub_0DD2`: undescribed.
- `modify_global_value`: Checks a global variable and, if zero, performs conditional logic to update a global variable or modify a memory location based on a bit-shifted or offset value.
- `update_hardware_parameters`: Checks a global variable and, if zero, updates several hardware registers or memory locations with new values based on a lookup or calculation.
- `do_nothing_5`: the routine is empty and performs no operations.
- `do_nothing_stub`: The routine is empty and returns immediately without performing any operations or modifying data.
- `return_from_subroutine`: the routine executes a return instruction.
- `do_nothing_stub_2`: The routine is empty and returns immediately without performing any operations.
- `process_string_decryption`: Iterates through a list of data structures to identify specific tagged entries, then performs a multi-pass decryption or transformation on a buffer using a lookup table and a running sum or offset.
- `expand_compressed_assets`: Iterates through a linked list of compressed data structures to decompress or expand certain byte sequences into a destination buffer.
- `return_constant_value`: Returns a constant value of 3000.
- `tiny_move_rts`: Small routine of 2 instructions that copies 100 to d0, then returns.
- `sub_134C`: undescribed.
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `do_nothing_stub_3`: The routine is empty and returns immediately without performing any operations or accessing any data.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: calls another routine and returns the concatenated contents of two registers.
- `sub_1438`: undescribed.
- `sub_1474`: undescribed.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `store_byte_and_call`: Stores a byte from a register into a global variable and calls another routine.
- `store_register_and_call_function`: stores the contents of a data register into a global variable and then calls a function pointer retrieved from a table or offset.
- `update_global_with_conditional`: Checks if a register contains a specific value and, if so, assigns a different value to a global variable before calling another routine.
- `iterate_until_value`: Iterates through a routine or table, checking return or or comparison values until a specific condition or non-zero value is met.
- `update_global_buffer`: Updates a global variable or a buffer-like structure with new values based on certain character checks and then calls a function pointer.
- `traverse_string_null`: Traverses a null-terminated string or buffer, calling another routine for every non-null character encountered.
- `calculate_scaled_offsets`: Clamps register values and stores the results multiplied by a constant into two global variables.
- `return_immediately`: The routine is empty and returns immediately without performing any operations or accessing data.
- `update_position_limits`: Updates two global variables by adding or subtracting a value, then clamps them within a certain range, and finally clears certain bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `get_scaled_value`: Returns a global variable bit-shifted right by one.
- `compare_and_reset_table`: Iterates through a table to compare values, and if a certain condition is met, updates a global variable or resets a portion of a table to zero.
- `set_status_flags`: Calls a function pointer and then modifies a global variable or sets specific bits within it based on the status of certain flags.
- `update_button_mask`: Modifies a register value based on certain bits in a global variable, specifically targeting bits related to hardware buttons.
- `calculate_bit_mask`: Constructs a value in a register by checking specific bits in a global variable and setting corresponding bits in the register, while also checking a sign bit.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `execute_permitted_function`: Checks if a global variable is greater than one, and if so, calls a function pointer located at an offset from that variable.
- `execute_indirect_function`: Checks if a global variable exceeds a certain threshold, then executes a function pointer located at an offset relative to that variable.
- `execute_conditional_function`: Executes a function pointer from a table and ensures a return value is not zero.
- `return_zero`: The routine is empty and always returns a zero value.
- `execute_two_functions`: Calls a function pointer and, if the return value is non-zero, calls a second function pointer.
- `execute_indirect_function_loop`: Executes a function pointer and, if a global variable is non-zero, executes another function pointer before entering an infinite loop.
- `call_function_with_fallback`: Executes a function pointer from a table and returns the result, or returns a certain value if the result is zero.
- `execute_conditional_pointer_2`: Calls a routine and then calls a function pointer from a table if a certain value is not zero, returning a signed integer.
- `call_indirect_function_3`: Calls a function pointer located at an offset from a global variable.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `ensure_minimum_value`: Checks if a global variable is less than two, sets it to one if so, and then calls a routine using a table-based function pointer.
- `wait_for_input_string`: Checks a global variable and enters a loop that processes characters or calls other routines until a carriage return or a specific character is encountered, while checking against a string literal.
- `move_and_clear_data`: Iteratively copies data from one set of pointers to a destination pointer while clearing the source memory locations to zero.
- `copy_memory_block_3`: copies a block of memory from a source address to a destination address using a nested loop structure.
- `do_nothing_routine`: The routine is empty and performs no operations.
- `process_message_queue`: Calls a function to retrieve a pointer, then checks a bit in a member of the retrieved structure, potentially looping to find a message that meets a specific bitmask condition.
- `return_stack_value`: returns a value loaded from the stack.
- `execute_conditional_pointer`: Executes a function pointer or updates a global variable based on the result of a previously called routine.
- `sub_1E84`: undescribed.
- `sub_1E9A`: undescribed.
- `sub_1F18`: undescribed.
- `sub_1F26`: undescribed.
- `sub_1F62`: undescribed.
- `execute_multiple_functions`: Calls a function pointer, then calls that same function again with a register, then calls a different function pointer, and finally assigns a return value to a global variable.
- `sub_1FDA`: undescribed.
- `execute_conditional_function_2`: Checks a global variable and calls a function pointer or certain other functions depending on the value or result of a function call.
- `get_library_result`: calls a function via a pointer and if the result is non-zero, calls two other functions and stores a result in a global variable.
- `sub_2504`: undescribed.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters if a certain condition is met.
- `update_buffer_indices_recursively`: This routine recursively updates a pointer-based table and adjusts buffer start and end indices based on a certain size or offset.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table.
- `initialize_blit_parameters`: Sets several consecutive global bytes to a fixed block of values and calls a routine to configure a destination address and hardware registers.
- `sub_294C`: undescribed.
- `sub_2986`: undescribed.
- `sub_2A32`: undescribed.
- `sub_2A66`: undescribed.
- `sub_2A6E`: undescribed.
- `apply_memory_offsets`: Calculates and stores a series of offset pointers and initializes several global bytes.
- `sub_2A96`: undescribed.
- `sub_2AAA`: undescribed.
- `sub_2AB0`: undescribed.
- `sub_2AB6`: undescribed.
- `sub_2ADA`: undescribed.
- `sub_2B40`: undescribed.
- `sub_2B60`: undescribed.
- `sub_2B6A`: undescribed.
- `sub_2B76`: undescribed.
- `sub_2BA6`: undescribed.
- `sub_2BA8`: undescribed.
- `sub_2BB8`: undescribed.
- `sub_2BEC`: undescribed.
- `sub_2BF6`: undescribed.
- `sub_2C44`: undescribed.
- `sub_2C50`: undescribed.
- `sub_2C64`: undescribed.
- `sub_2CB6`: undescribed.
- `sub_2CEE`: undescribed.
- `sub_2D0A`: undescribed.
- `sub_2D32`: undescribed.
- `apply_coordinate_offsets`: Calculates a difference between two points and applies that difference to a list of coordinate values while updating certain global variables.
- `execute_indirect_call`: calculates a target address using a base register and an offset from a table, then jumps to that address.
- `initialize_memory_offsets`: calculates and stores a series of offset pointers and initializes several global bytes.
- `sub_38F4`: undescribed.
- `sub_38F6`: undescribed.
- `sub_38F8`: undescribed.
- `sub_38FA`: undescribed.
- `sub_38FC`: undescribed.
- `sub_38FE`: undescribed.
- `sub_390C`: undescribed.
- `store_pointer_to_table`: reads two bytes from a pointer and stores a pointer to a later offset of that same memory block into another memory location.
- `read_short_at_pointer`: returns a short integer read from a memory address held in a register.
- `sub_393A`: undescribed.
- `read_three_bytes`: reads three consecutive bytes from a pointer and returns them as a single packed value.
- `update_memory_structure_3`: Updates a memory structure or a global variable with a pointer and a constant value based on certain conditional checks.
- `update_memory_structure_2`: Updates a memory structure or a global variable with a pointer and a constant byte value based on certain conditional checks.
- `update_memory_structure`: copies a word, then writes a pointer and a constant byte value into a memory structure at a calculated offset.
- `update_pointer_target`: Checks if a global variable is zero, then updates that variable and a pointer-based memory location with a new value.
- `return_empty`: The routine is empty and returns.
- `sub_39CE`: undescribed.
- `sub_39DC`: undescribed.
- `sub_3A00`: undescribed.
- `sub_3A14`: undescribed.
- `sub_3A16`: undescribed.
- `sub_3A18`: undescribed.
- `sub_3A1A`: undescribed.
- `sub_3A1C`: undescribed.
- `validate_and_clear_node_pointer`: Checks if a global variable is null and compares a value against a register; if a certain condition is met, it clears a short value in a memory structure using a pointer-based offset.
- `get_dereferenced_value`: Stores a pointer in a global variable, calls another routine, and returns a value from that pointer or a returned short.
- `load_global_byte`: Stores a pointer in a global variable, calls another routine, and returns a byte loaded from a global variable or a returned word.
- `read_global_byte`: Returns a byte loaded from a global variable.
- `sub_3A8E`: undescribed.
- `sub_3AA2`: undescribed.
- `sub_3AB8`: undescribed.
- `sub_3AC6`: undescribed.
- `conditionally_return_pointer_target`: If a certain flag is zero, it returns a value from a pointer, otherwise it returns a register value.
- `conditionally_return_three_bytes`: If a certain flag is zero, it constructs a 32-bit value from a pointer and returns it.
- `conditionally_return_pointer_byte`: If a certain flag is non-zero, it returns a byte loaded from a pointer, otherwise it returns a register value.
- `conditionally_return_pointer_target_2`: If a certain flag is non-zero, it returns a value from a pointer, otherwise it returns a register value.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a3, then returns.
- `sub_3B1E`: undescribed.
- `sub_3B34`: undescribed.
- `sub_3B38`: undescribed.
- `sub_3B46`: undescribed.
- `sub_3B4A`: undescribed.
- `sub_3B5A`: undescribed.
- `sub_3B5E`: undescribed.
- `sub_3B74`: undescribed.
- `sub_3B78`: undescribed.
- `sub_3B86`: undescribed.
- `sub_3B8A`: undescribed.
- `sub_3B9A`: undescribed.
- `sub_3B9E`: undescribed.
- `sub_3BB4`: undescribed.
- `copy_register_d7_to_d6`: This routine copies the contents of one register to another.
- `execute_eight_instructions`: This routine executes eight instructions and is called by four other routines.
- `execute_indirect_call_2`: This routine calculates an address using a pointer and an offset and calls the resulting address.
- `copy_register_d7_to_d6_2`: This routine performs a single instruction that copies the contents of one register to another.
- `undefined_routine`: This routine contains eight instructions and is called by three other routines.
- `copy_register_value_2`: Performs a single instruction to copy the contents of one register to another and calls another routine.
- `sub_3C1E`: undescribed.
- `sub_3C20`: undescribed.
- `sub_3C22`: undescribed.
- `sub_3C56`: undescribed.
- `sub_3C80`: undescribed.
- `sub_3C90`: undescribed.
- `sub_3CA0`: undescribed.
- `sub_3CC2`: undescribed.
- `sub_3CC6`: undescribed.
- `sub_3CE6`: undescribed.
- `sub_3D1C`: undescribed.
- `sub_3D24`: undescribed.
- `sub_3D44`: undescribed.
- `sub_3D58`: undescribed.
- `sub_3D6A`: undescribed.
- `sub_3D7C`: undescribed.
- `sub_3D8E`: undescribed.
- `sub_3DA0`: undescribed.
- `sub_3DB0`: undescribed.
- `sub_3DBE`: undescribed.
- `sub_3DD4`: undescribed.
- `sub_3DEA`: undescribed.
- `sub_3DF0`: undescribed.
- `sub_3DF8`: undescribed.
- `sub_3E18`: undescribed.
- `set_global_from_function`: Calls a routine and updates a global variable with either a returned value or a constant if a certain condition is met.
- `process_string_characters`: Modifies global or buffer-based counters or pointers based on whether a character matches newline, carriage return, or space, and calls another routine if certain conditions are met.
- `iterate_string_characters`: Iterates through a null-terminated string, calling a routine to process or convert each character or integer-string representation until a null terminator is encountered.
- `convert_integer_to_string`: Converts a signed integer or a value from a register into a string or a sequence of characters, including handling a negative sign, and stores the result in a buffer.
- `store_word_via_indexed_pointer`: stores a word from a pointer-referenced location into a destination address calculated using a base pointer and an index multiplied by two.
- `execute_eight_instructions_2`: This routine executes a fixed number of eight instructions and is called by several other routines.
- `sub_4070`: undescribed.
- `execute_table_operations`: Executes a fixed number of instructions, performs a binary search through a table of words, and resets a string buffer or linked list structure.
- `delete_memory_node`: Deletes a node from a managed memory list by shifting remaining pointers to close the gap and updating a linked list of pointers.
- `remove_memory_node`: Removes a node from a memory-managed linked list, shifts remaining node pointers to close the gap, and updates a global pointer-based list or table.
- `check_global_null`: Checks if a global variable is zero and, if not, calls another routine.
- `sub_4134`: undescribed.
- `sub_413E`: undescribed.
- `traverse_string_buffer`: Traverses a string buffer and performs conditional checks and updates or calls other routines if certain string termination or data conditions are met.
- `find_minimum_from_pointer`: If a certain value matches a value in a structure, it searches through a table to find the minimum integer value using a pointer obtained from another routine.
- `update_linked_list_node_2`: Modifies a bit in a byte, copies a sequence of bytes from one location to another, and updates several fields in a structure or list node.
- `sub_4218`: undescribed.
- `set_global_if_zero`: Checks if a global variable is zero and sets it to one if it is.
- `reset_global_variable`: Sets a global variable to zero.
- `sub_424E`: undescribed.
- `sub_427A`: undescribed.
- `update_if_different`: Compares a value from a pointer to a value at a calculated offset and updates that offset if they differ.
- `update_table_values_2`: If a certain value is non-zero, this routine writes a register or a value to a memory location calculated using a global variable and an offset, then sets a high bit in a byte at a pointer.
- `sub_42F0`: undescribed.
- `sub_4316`: undescribed.
- `sub_433C`: undescribed.
- `sub_4374`: undescribed.
- `sub_43B0`: undescribed.
- `sub_4400`: undescribed.
- `sub_4438`: undescribed.
- `sub_4456`: undescribed.
- `sub_4458`: undescribed.
- `sub_445A`: undescribed.
- `sub_445C`: undescribed.
- `modify_bit_flags`: This routine clears a specific bit and sets another bit in a byte located at an offset from a global variable, while also performing linked list insertion and removal operations.
- `set_bit_in_byte`: Sets the certain bit in a byte located at an offset determined by a pointer.
- `insert_into_linked_list`: Inserts a new value into a linked list or updates a pointer within a structure if the list is empty or certain conditions are met.
- `remove_from_linked_list`: Traverses a linked list to find a specific value and updates a pointer or replaces a node if a match is found.
- `calculate_cross_products`: Performs a series of signed byte multiplications and subtractions, storing the results in a structure.
- `set_byte_at_offset`: Sets a byte at a specific offset relative to a register value to one.
- `sub_45E0`: undescribed.
- `sub_45E6`: undescribed.
- `process_string_data_stream`: Parses a data buffer to update a set of global counters and pointers, while conditionally copying or modifying word-sized or byte-sized values into a destination buffer based on certain flag or value checks.
- `copy_string_with_offset_and_transform`: Copies a sequence of words from a source buffer to a destination buffer while applying a signed or unsigned transformation to the data and updating a global offset.
- `parse_string_stream_tokens`: Iterates through a data buffer to extract and apply a sequence of length-prefixed or type-prefixed commands, updating global counters and pointers.
- `expand_and_transform_string`: Copies and expands a sequence of words from a source buffer to a destination buffer, applying a signed or unsigned transformation to the data.
- `parse_string_data_stream`: Iterates through a data buffer to update or copy a sequence of word-sized or byte-sized values into a destination buffer, or modifies a global counter and pointer based on conditional flags or signed-value checks within the stream.
- `sub_469A`: undescribed.
- `sub_46AE`: undescribed.
- `sub_46C8`: undescribed.
- `copy_string_with_offset`: copies a sequence of words from a source buffer to a destination buffer while simultaneously updating a global offset or counter.
- `sync_linked_list_nodes`: Synchronizes or updates a linked list by traversing nodes and updating pointers or values based on a condition.
- `validate_table_entry_condition`: Checks if a table entry, determined by an offset, matches a value from another table and a global variable.
- `compare_table_entry`: Compares a value from a table against a global variable and checks if another table entry matches a specific value.
- `search_linked_list`: Traverses a linked list to check if any node contains a value equal to a global variable and a specific short value.
- `search_pointer_table`: Iterates through a table of pointers to find an entry that matches a specific value stored in a global variable.
- `traverse_table_list`: Iterates through a table of pointers until a zero or a certain value is encountered.
- `empty_routine_3`: the routine performs no operations and returns immediately.
- `insert_into_list_structure`: Swaps or inserts a value into a linked list structure using a global index, while updating pointers and clearing certain memory offsets.
- `store_indirect_byte`: Stores a byte from a register into two adjacent memory locations or a table.
- `sub_49B2`: undescribed.
- `sub_49B8`: undescribed.
- `sub_49C0`: undescribed.
- `sub_4A04`: undescribed.
- `sub_4A0E`: undescribed.
- `sub_4A34`: undescribed.
- `sub_4A4A`: undescribed.
- `compute_scaled_vector_offset`: Calculates a new position and direction by applying a bit-shifted scale factor to a vector, then updates global destination coordinates.
- `get_conditional_value`: Returns a value based on a comparison of a byte at an offset or a constructed 32-bit value.
- `sub_4C58`: undescribed.
- `resolve_indirect_pointer_offset`: calculates a target address by traversing multiple levels of indirection and adding a scaled offset or a value retrieved from a pointer-based table.
- `calculate_pointer_offset`: Calculates a pointer or value by adding an offset and a value retrieved from a table or memory location.
- `toggle_global_bit`: Sets a global variable to a value loaded from a register and uses a loop to check if that value is zero.
- `sub_4CCA`: undescribed.
- `traverse_linked_list_nodes`: Traverses a linked list and resets a global variable to zero if a certain condition is met or when the end of the list is reached.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `set_triple_word`: Assigns a 16-bit value to three consecutive memory locations or a table.
- `sub_4DDC`: undescribed.
- `sub_4DF4`: undescribed.
- `sub_4E12`: undescribed.
- `sub_4E22`: undescribed.
- `sub_4E5C`: undescribed.
- `sub_4E8C`: undescribed.
- `store_pointer_values`: Stores two consecutive words from a memory location into two global variables.
- `apply_coordinate_offset`: Calculates a difference between two points and adds that difference to a list of coordinate values.
- `offset_list_coordinates`: Adds values from a source or offset table to an existing list and updates a flag in a list.
- `initialize_clipping_table`: Sets several global bytes to fixed values and then iterates through a table of pointers to copy memory blocks and update a rectangular bounding box or clipping list.
- `update_rect_bounds_and_clip_list`: Updates a global rectangular bounding box or clipping area by calculating new minimum and maximum coordinates from a source structure, and then traverses a linked list of nodes to update their internal bounds or remove certain entries based on flags.
- `update_linked_list_node_bounds`: Iterates through a linked list or table-based chain of entries, updating table offsets and global bounds-related variables while checking for specific flags or termination conditions.
- `write_to_table_offset`: Writes several values, including a global variable, to a table or memory area using an offset.
- `copy_table_entry`: Copies a 16-bit value from one location to another and updates a global variable with a previously held value.
- `update_table_with_bounds`: Writes several values and a global variable to a table using an offset, copies a 16-bit value, and updates global bounds if a certain range is exceeded.
- `initialize_global_bounds`: Sets two global variables to a high value and two other global variables to a low value.
- `expand_global_bounds`: Updates certain global variables with new values if the new values exceed or fall below current bounds.
- `update_linked_list`: Traverses a linked list and replaces a node or updates a pointer to a new node if certain comparison conditions or certain value checks are met.
- `traverse_linked_list_with_bounds_check`: Iterates through a linked list of pointer-offset or size-prefixed entries, checking certain flags and validating that a secondary pointer or range does not exceed certain bounds.
- `return_empty_stack_value`: The routine is empty and returns an uninitialized value from the stack.
- `initialize_buffer_constants`: Copies a sequence of four orm-size or word-size values from one memory location to another.
- `copy_table_block`: Iterates through a table of 16-bit values, copying them into a set of global variables.
- `clamp_rect_bounds`: Checks if a range or area defined by a pointer-offset table exceeds certain global bounds and clamps those values to a minimum threshold if they are too small.
- `expand_rect_bounds`: Checks if a rectangular area defined by a table and certain global values is within a valid range, then expands the global bounds to include that area.
- `update_clipping_region`: Updates a global clipping or visibility rectangle by comparing a new area against existing bounds, including bitwise masking and calculating an offset or size.
- `return_empty_routine`: the routine performs no operations and immediately returns.
- `sub_5A36`: undescribed.
- `sub_5A4A`: undescribed.
- `sub_5A5C`: undescribed.
- `sub_5A66`: undescribed.
- `sub_5A9A`: undescribed.
- `sub_5AB8`: undescribed.
- `sub_5AE4`: undescribed.
- `sub_5D56`: undescribed.
- `sub_5D6C`: undescribed.
- `sub_5D88`: undescribed.
- `sub_5D90`: undescribed.
- `sub_5D9C`: undescribed.
- `sub_5DAA`: undescribed.
- `sub_5DE6`: undescribed.
- `sub_5E30`: undescribed.
- `sub_5E50`: undescribed.
- `sub_5E9E`: undescribed.
- `sub_5ECA`: undescribed.
- `sub_5EDA`: undescribed.
- `sub_5EEE`: undescribed.
- `sub_5F36`: undescribed.
- `return_without_effect`: The routine consists of a single return instruction and performs no work, so it acts as an empty stub that update_window_dimensions can call without side effects.
- `traverse_buffer_with_bounds`: Traverses a buffer while checking a length byte and calling another routine if a certain condition is met.
- `intersect_rect_bounds`: Checks if a rectangle, defined by a pointer-based offset and certain width/height modifiers, overlaps with a global boundary.
- `check_rect_intersection_2`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary and returns a modified pointer.
- `check_rect_intersection`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary, and returns a masked or modified pointer or value.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d1, then returns.
- `sub_60D2`: undescribed.
- `sub_6108`: undescribed.
- `sub_6112`: undescribed.
- `sub_6120`: undescribed.
- `sub_614C`: undescribed.
- `sub_6158`: undescribed.
- `sub_6168`: undescribed.
- `sub_6182`: undescribed.
- `sub_61BA`: undescribed.
- `sub_61D8`: undescribed.
- `sub_6210`: undescribed.
- `sub_6224`: undescribed.
- `sub_625C`: undescribed.
- `sub_627A`: undescribed.
- `sub_629A`: undescribed.
- `sub_62E6`: undescribed.
- `sub_6330`: undescribed.
- `sub_636C`: undescribed.
- `sub_6392`: undescribed.
- `sub_63E0`: undescribed.
- `sub_6420`: undescribed.
- `sub_6446`: undescribed.
- `sub_648A`: undescribed.
- `sub_64CA`: undescribed.
- `sub_64F0`: undescribed.
- `sub_6542`: undescribed.
- `sub_65E0`: undescribed.
- `sub_6626`: undescribed.
- `sub_662E`: undescribed.
- `sub_665A`: undescribed.
- `sub_6664`: undescribed.
- `copy_memory_block_4`: Copies a block of memory from a source pointer to a destination pointer, decrementing a counter and updating a pointer for each word or byte transferred.
- `search_table_for_value`: Iterates through a table or list, checking if a value matches a certain constant, and updates a pointer or counter during the process.
- `return_empty_routine_2`: The routine performs no operations and immediately returns to its caller.
- `tiny_adda_rts`: Small routine of 2 instructions that adds (a3)+ to a3, then returns.
- `sub_66C6`: undescribed.
- `sub_66EE`: undescribed.
- `do_nothing_stub_4`: The routine contains no instructions and returns immediately.
- `clear_table_entry`: If a certain condition is met, writes zero to a memory location determined by an offset in a table.
- `sub_6712`: undescribed.
- `sub_6722`: undescribed.
- `sub_673E`: undescribed.
- `sub_67A6`: undescribed.
- `sub_67BA`: undescribed.
- `sub_67CC`: undescribed.
- `sub_67DC`: undescribed.
- `sub_680A`: undescribed.
- `sub_6818`: undescribed.
- `clear_bit_zero`: Clears the least significant bit of a byte at a calculated offset from a pointer.
- `set_bit_one`: Sets the second least significant bit of a byte at a calculated offset from a pointer.
- `clear_bit_one`: Clears the second least significant bit of a byte at a calculated offset from a pointer.
- `fetch_indirect_call_target`: loads a byte from an indirect call target and stores it into a global variable.
- `copy_memory_block_5`: Copies a block of bytes from a source table to a destination address, where the block size is read from the source table.
- `sub_6860`: undescribed.
- `sub_68C0`: undescribed.
- `sub_68DA`: undescribed.
- `set_global_to_one`: Masks a register value and then overwrites a global variable with a constant value of one.
- `update_global_pointers`: Calculates a new address based on a global variable and stores it into two global variables.
- `evaluate_state_and_sync_data`: Checks a global variable against several constant values and, depending on the result, recursively calls another routine or performs a block copy of data from a source pointer to two destination pointers.
- `return_from_subroutine_2`: the routine performs a single jump or return instruction to a calling routine.
- `copy_data_with_conditional_offset`: Copies a sequence of words from a source pointer to two destination pointers, followed by a conditional copy of a block of bytes from a global variable.
- `sub_69C4`: undescribed.
- `reset_global_variable_2`: Sets a global variable to zero through a series of assignments.
- `update_table_with_register`: If a global variable does not match certain allowed values, stores a register value into a table at an offset calculated from another register.
- `sub_6AC0`: undescribed.
- `sub_6B40`: undescribed.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `check_nibble_difference`: Calculates the difference between two lower nibbles and returns if the result is negative or outside a certain range.
- `reset_and_copy_global_data`: Resets a global variable and performs conditional block copies of data between a source pointer, a global variable, and a destination pointer or another global variable.
- `reconstruct_nibble_data`: Iterates through a buffer to detect or modify nibble-based data encodings.
- `reset_and_copy_data`: Checks if a global variable matches certain values, resets it, and then copies a block of memory from a source pointer to a destination pointer, followed by a conditional copy of a small number of words.
- `verify_nibble_sum_threshold`: tests if the sum of two lower nibbles exceeds a constant value and returns the result.
- `compare_nibble_range`: Checks if the difference between two lower nibbles falls within a specific range or if the result is negative.
- `sub_6E20`: undescribed.
- `sub_6E84`: undescribed.
- `sub_6EE6`: undescribed.
- `sub_6F1C`: undescribed.
- `sub_6F3C`: undescribed.
- `sub_6F72`: undescribed.
- `sub_6FA8`: undescribed.
- `sub_6FDE`: undescribed.
- `sub_7014`: undescribed.
- `sub_7026`: undescribed.
- `sub_705C`: undescribed.
- `sub_7092`: undescribed.
- `sub_70E2`: undescribed.
- `sub_71E8`: undescribed.
- `sub_7236`: undescribed.
- `sub_724E`: undescribed.
- `sub_726A`: undescribed.
- `sub_7274`: undescribed.
- `sub_7298`: undescribed.
- `sub_72A6`: undescribed.
- `sub_72B8`: undescribed.
- `sub_72E4`: undescribed.
- `sub_7420`: undescribed.
- `sub_7432`: undescribed.
- `sub_7448`: undescribed.
- `sub_74F8`: undescribed.
- `sub_750E`: undescribed.
- `sub_7520`: undescribed.
- `sub_752C`: undescribed.
- `sub_75B8`: undescribed.
- `update_object_position`: Calculates and applies a new position or offset to a structure in memory, including conditional logic for handling directional or certain type-specific movement adjustments.
- `update_window_dimensions`: Updates a set of window or clipping-area coordinates by adding or subtracting offsets or certain values retrieved from a table.
- `reset_global_variable_3`: Checks if a certain global variable and another global variable meet certain conditions, then clears that global variable.
- `sub_79B0`: undescribed.
- `sub_79D8`: undescribed.
- `clear_global_variable`: Performs several calls to other routines and then sets a global variable to zero.
- `sub_7A18`: undescribed.
- `sub_7A3E`: undescribed.
- `sub_7A46`: undescribed.
- `sub_7A5A`: undescribed.
- `sub_7A6A`: undescribed.
- `sub_7A76`: undescribed.
- `sub_7A8A`: undescribed.
- `sub_7A94`: undescribed.
- `sub_7A9E`: undescribed.
- `sub_7AAE`: undescribed.
- `sub_7AE0`: undescribed.
- `sub_7AE8`: undescribed.
- `copy_register_data`: Copies the contents of one register to another register.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer located within a table.
- `swap_global_values`: Swaps the contents of two adjacent global variables and then performs an indirect call using a pointer retrieved from a table or memory location.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine_3`: the routine performs no operations and immediately returns to the caller.
- `copy_string_data`: Copies a null-terminated string from a source address to a destination address.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 6 instructions in the listing and performs no work.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or an immediate return, effectively acting as a null operation.
- `sub_7C42`: undescribed.
- `sub_7C58`: undescribed.
- `sub_7C6A`: undescribed.
- `sub_7C7A`: undescribed.
- `return_immediately_2`: The routine executes a return instruction to exit the current function.
- `return_empty_routine_4`: The routine contains no executable instructions and performs no operations before returning.
- `copy_string_from_table`: Copies a null-terminated string from a location determined by an offset in a table to a destination address.
- `sub_7CBE`: undescribed.
- `sub_7CCE`: undescribed.
- `sub_7CDC`: undescribed.
- `sub_7CF4`: undescribed.
- `sub_7D08`: undescribed.
- `sub_7D1A`: undescribed.
- `sub_7D36`: undescribed.
- `sub_7D52`: undescribed.
- `sub_7D6C`: undescribed.
- `sub_7D90`: undescribed.
- `sub_7DB2`: undescribed.
- `calculate_weighted_sum`: Calculates a sum by multiplying a value from a pointer-indexed table with a value from another pointer-indexed table, repeating until a termination condition in a byte is met.
- `calculate_weighted_sum_2`: Performs a loop that accumulates a weighted sum or a product-based calculation into a return register using a pointer and a counter.
- `accumulate_weighted_product_2`: Multiplies values from two pointer-indexed tables and accumulates the result into a register until a counter reaches zero.
- `accumulate_weighted_product`: Multiplies a value from a pointer-indexed table with a value from another pointer-indexed table and accumulates the result until a counter reaches negative one.
- `sub_7ED8`: undescribed.
- `sub_7EF0`: undescribed.
- `copy_register_value`: Calls another routine and then performs a single instruction that copies the contents of one register to another.
- `copy_register_d7_to_d6_3`: copies the contents of one data register to another.
- `copy_d7_to_d6`: Copies the contents of one data register to another.
- `sub_7F14`: undescribed.
- `compare_registers_not_equal`: Calls another routine and then checks if two registers contain different values.
- `compare_registers_equal`: Calls another routine and then checks if two registers contain the same value.
- `compare_registers_and_branch`: compares two 16-bit values stored in registers and returns if the value in one register is less than the value in another.
- `compare_registers_and_branch_2`: Copies a value from one register to another and then checks if the new value is less than the original value, exiting if the condition is met.
- `compare_registers_and_branch_3`: compares two or more words and branches if the value in one register is not greater than the value in another.
- `compare_d6_d7`: compares a 16-bit value in a register against another 16-bit value and returns if the orignal is not greater.
- `copy_register_d7_to_d6_4`: Copies the contents of one data register to another data register.
- `call_indirect_function_4`: Loads a function pointer from a global variable and an indexed table, then executes the pointed-to routine.
- `copy_register_data_2`: Copies the contents of one data register to another using a specialized move instruction.
- `sub_7FB6`: undescribed.
- `sub_7FC2`: undescribed.
- `sub_7FCA`: undescribed.
- `sub_7FCE`: undescribed.
- `store_linear_hash`: Computes a linear hash and stores the resulting 16-bit value into a global variable.
- `compute_linear_hash`: multiplies a 16-bit value by a constant and adds a constant.
- `check_register_sign`: Checks if a register is zero or negative and returns if so.
- `tiny_clr_rts_3`: Small routine of 2 instructions that clears d7, then returns.
- `stub_return_only_2`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 2 instructions in the listing and performs no work.
- `sub_8014`: undescribed.
- `reset_table_on_condition`: Iterates through a table to compare values and resets a portion of a table or a global variable to zero if a certain condition is met.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer.
- `sub_803A`: undescribed.
- `sub_8040`: undescribed.
- `branch_on_global_zero`: Checks if a global variable is zero and calls another routine if it is.
- `execute_self_recursively`: The routine calls itself recursively.
- `update_button_mask_2`: Modifies a register value based on specific bits in a global variable to target hardware buttons.
- `branch_on_global_non_zero`: Checks if a global variable is non-zero and calls another routine if it is.
- `clear_d0_and_return`: clears a register and returns.
- `sub_80A2`: undescribed.
- `traverse_list_by_value`: Checks a global variable and either returns or traverses a table using a pointer-based chain if certain value conditions are met.
- `call_other_routine`: Executes another routine without modifying any registers or returning a specific value.
- `store_zero_to_global`: Writes a zero byte to a global variable at an offset determined by a register.
- `sub_8196`: undescribed.
- `sub_81C4`: undescribed.
- `sub_81E8`: undescribed.
- `sub_81FE`: undescribed.
- `sub_8248`: undescribed.
- `sub_826C`: undescribed.
- `sub_8286`: undescribed.
- `sub_828E`: undescribed.
- `sub_8292`: undescribed.
- `sub_829E`: undescribed.
- `sub_82AC`: undescribed.
- `sub_82B8`: undescribed.
- `sub_82CC`: undescribed.
- `sub_82D2`: undescribed.
- `sub_8322`: undescribed.
- `sub_8332`: undescribed.
- `sub_8342`: undescribed.
- `sub_8352`: undescribed.
- `sub_8362`: undescribed.
- `sub_8374`: undescribed.
- `sub_837E`: undescribed.
- `sub_8392`: undescribed.
- `sub_83AE`: undescribed.
- `sub_83BA`: undescribed.
- `sub_83C4`: undescribed.
- `copy_string_to_buffer`: copies a null-terminated string from a source pointer to a destination pointer.
- `sub_8414`: undescribed.
- `sub_8444`: undescribed.
- `sub_8460`: undescribed.
- `sub_8466`: undescribed.
- `sub_8480`: undescribed.
- `sub_8490`: undescribed.
- `sub_8496`: undescribed.
- `sub_84A6`: undescribed.
- `sub_84AC`: undescribed.
- `sub_84BC`: undescribed.
- `sub_84C2`: undescribed.
- `sub_84D2`: undescribed.
- `sub_84D8`: undescribed.
- `sub_84E8`: undescribed.
- `sub_84EE`: undescribed.
- `sub_84FE`: undescribed.
- `sub_8504`: undescribed.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d6 to d7, then copies (a4)+ to d6, then returns.
- `execute_infinite_indirect_call`: Performs an infinite loop that calls another routine via an indirect pointer.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_offset`: stores a byte from a register into a memory location calculated from a pointer and an offset.
- `store_word_at_offset`: stores a 16-bit value from a register into a memory location calculated from a pointer and anoffset.
- `copy_string_to_destination`: Copies a null-terminated string from a global source to a destination calculated using a base address and an offset.
- `store_byte_from_pointer`: Takes a byte from a pointer-indexed table and stores it into a global variable at an offset or index.
- `store_pointer_destination`: Stores a value from a source pointer into a destination address calculated using a register and the return value of another routine.
- `sub_859C`: undescribed.
- `write_byte_at_offset`: Writes a single byte to a destination calculated by adding an offset from a pointer to a base address.
- `write_word_at_offset`: Writes a two-byte word to a destination calculated by adding an offset from a pointer to a base address.
- `copy_string_from_global`: copies a null-terminated string from a global memory location to a destination buffer calculated from a base address and an offset.
- `sub_85DE`: undescribed.
- `sub_85EE`: undescribed.
- `sub_85FE`: undescribed.
- `sub_8616`: undescribed.
- `sub_8628`: undescribed.
- `sub_863A`: undescribed.
- `sub_8656`: undescribed.
- `sub_8672`: undescribed.
- `sub_868E`: undescribed.
- `sub_86B2`: undescribed.
- `sub_86D2`: undescribed.
- `invoke_indirect_subroutine`: The routine executes an infinite loop that branches to another routine using a pointer stored in memory.
- `add_byte_to_destination`: Adds a byte to an existing byte at a destination calculated using a base address and an offset.
- `add_word_to_destination`: Adds a word to an existing word at a destination calculated using a base address and an offset.
- `sub_87D6`: undescribed.
- `sub_87F2`: undescribed.
- `sub_8804`: undescribed.
- `sub_8816`: undescribed.
- `sub_8836`: undescribed.
- `increment_memory_at_offset`: adds a 16-bit value to a 16-bit value stored at a memory location determined by a base address plus an offset.
- `copy_string_to_destination_reverse`: Finds the end of a destination buffer and copies a null-terminated string from a global source into it.
- `sub_8864`: undescribed.
- `sub_8874`: undescribed.
- `sub_8884`: undescribed.
- `sub_88A2`: undescribed.
- `sub_88B4`: undescribed.
- `sub_88C6`: undescribed.
- `sub_88E8`: undescribed.
- `sub_8904`: undescribed.
- `sub_8920`: undescribed.
- `sub_894A`: undescribed.
- `sub_896A`: undescribed.
- `reset_local_variables`: Resets a global variable to zero and processes a string or table.
- `copy_string_buffer`: Copies a string from a source address to a destination address and then copies a second string from a global pointer to a new location, terminating with a null byte.
- `initialize_data_structures`: Initializes a series of global variables and a table with specific values and a loop that populates a table with calculated pointers.
- `process_string_and_reinitialize_table`: Copies a source string to a destination buffer, replacing any period characters with a fallback character, and subsequently resets a table of pointers and size-related metadata in memory.
- `update_global_byte`: Sets a global variable to a constant and checks its bitmask before performing conditional writes or looping through calls that modify memory via pointer-based checks.
- `get_global_byte`: Checks if a register-held value is less than or equal to a pointer, then calls another routine and returns a byte from a global variable.
- `write_byte_with_bounds_check`: Writes a byte to a destination if a calculated address range check passes.
- `process_memory_with_offsets`: Performs byte-level writes with bounds checking, calculates byte differences, and uses a loop to copy data or manipulate memory based on a calculated offset.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `calculate_byte_difference`: Calculates the difference between two bytes and returns the result or its negation depending on whether the difference is positive or negative, potentially calling another routine if a condition is met.
- `process_indirect_data_decrement`: Performs indirect calls and, if a certain byte or certain value is not present, iterates through a linked list to decrement certain members and checks for threshold violations.
- `decrement_list_if_nonzero`: If a certain short value is non-zero, it iterates through a linked list to decrement certain members and checks for threshold violations.
- `decrement_linked_list_members`: Iterates through a linked list of structures to decrement certain integer or pointer-based members and checks for threshold violations.
- `update_linked_list_node_properties`: Iterates through a linked list of structures to decrement certain integer or pointer-based members and check for threshold violations.
- `update_linked_list_members`: Iterates through a linked list to decrement certain memory values and update other linked list members if certain thresholds are met.
- `update_linked_list_values`: Iterates through a linked list, updating certain memory values or decrementing a count if a condition is met, and potentially updating other list-linked values.
- `update_list_member_values`: Iterates through a linked list of structures and, if a member's value exceeds a certain threshold, replaces it with the difference between that value and a fixed amount.
- `decrement_list_values`: Iterates through a linked list and decrements certain member values if they exceed a specific threshold.
- `find_minimum_in_table`: If a certain value matches a value in a structure, it searches through a table to find the minimum integer value.
- `empty_routine_4`: the routine contains no instructions and performs no operations or data transfers.
- `invoke_and_return_pair`: Calls another routine and returns a concatenated 64-bit value formed from two 32-bit registers.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value.
- `initialize_list_structure`: Iterates through a memory range, writing incrementing offsets and zero-values to a list or buffer.
- `update_linked_list_nodes`: The routine modifies a linked list or table of structures by updating pointers and clearing or initializing certain memory-resident fields.
- `reinitialize_string_buffer_structure`: Recursively traverses a linked list or chain of data structures, clearing a range of memory and resetting a buffer or string header with a new length, a pointer to the source data, and a cleared or zeroed-out metadata or attribute field.
- `reset_string_buffer_structure`: Initializes a complex string or buffer structure by clearing a range of memory and setting certain header or metadata fields to default or zeroed values.
- `deallocate_and_relink_memory_node`: Removes a node from a memory-managed linked list, shifts remaining node pointers to close the gap, and updates a global pointer-based list or tree structure.
- `execute_conditional_functions`: Executes function pointers from a table and stores or returns results, while also initializing a global variable and a sequence of memory constants or flags.
- `execute_conditional_function_3`: Calls a function pointer and subsequently calls a second function pointer if the first call returns a non-zero value.
- `execute_indirect_loop`: Iteratively calls a function pointer and conditionally calls another function pointer based on a return value.
- `iterate_function_returns`: Repeatedly calls a routine and stores the return value in a global variable until a non-negative return value is encountered.
- `store_function_result`: Executes a function from a table and stores the result into a global variable.
- `recursively_call_routine`: A recursive routine that calls itself and another routine, continuing until a return value is not negative.
- `recursively_execute_routine`: repeatedly calls itself and a second routine until a negative return value is encountered.
- `call_external_with_args`: Calls an external function via a pointer or offset and updates registers with return values.
- `ensure_minimum_value_and_call`: Checks if a global variable is less than two and sets it to one if so, then calls a routine using a table-based function pointer.
- `loop_function_calls`: Executes a sequence of function calls within a loop that continues while a certain value is negative.
- `execute_recursive_logic`: Performs a series of conditional checks and recursive calls that modify a global variable.
- `update_global_and_copy`: Updates a global variable, checks boundary conditions, and copies a sequence of memory blocks from a source to a destination.
- `decrement_and_clamp_value`: Decrements a value at a pointer, then checks if a sum exceeds a limit, clamping it to a maximum or resetting several memory or register values to zero.
- `execute_sequential_calls`: Executes another routine or routine-like sequence four times in a row.
- `reset_memory_block`: Clears or initializes a specific block of memory by writing zero or a specific constant value to multiple offsets from a pointer.
- `update_and_clamp_value`: Checks several global variables, performs arithmetic or conditional assignments, and clamps a value to a maximum limit before calling another routine.
- `decrement_global_and_call_2`: Updates a global variable, clamps it to a maximum value, and calls another routine twice.
- `decrement_global_and_call`: Decrements a global variable and calls another routine twice.
- `reset_global_after_call`: Calls another routine and then sets a global variable to zero.
- `initialize_string_buffer_constants`: Sets a global variable to zero and initializes a sequence of memory locations with specific constant values.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `increment_global_on_condition`: Calls another routine and, if the returned value is non-zero, increments a global variable by a large constant.
- `return_concatenated_registers`: Calls another routine and returns a 64-bit value composed of two 32-bit registers.
- `update_bitmask_and_shift_data_2`: Checks certain boundary conditions and, if met, clears specific bits in a global variable and then performs a bitwise-masked bit-shift.
- `update_bitmask_and_shift_data`: Checks certain boundary conditions and, if met, clears specific bits in a global variable and then performs a bitwise-masked bit-shift (bit-slicing or bit-reconstruction) or bit-spreading on a block of data, writing the result back to a destination buffer.
- `copy_memory_blocks`: copies a sequence of 2-byte or 4-byte or even 8-byte blocks from a source address to a destination address calculated using a global or a table.
- `sub_A0A0`: undescribed.
- `setup_blt_params`: calculates a destination address and configures certain hardware registers or a global variable, while waiting for a certain bit in a global variable to clear.
- `update_blt_attributes`: iteratively updates a global or a pointer-based address or a value by adding or subtracting an offset or a certain constant, while waiting for a certain bit in a global variable to clear.
- `copy_block_tiled_pattern`: Calculates a destination address based on a pointer and an offset, then copies a block of memory using a nested loop structure that increments source and destination pointers.
- `update_blt_parameters`: Calculates a new address and size for a bitboard transfer or a window and writes the result to a global variable.
- `calculate_final_address`: checks certain bits in a memory location and then updates a global variable with a value derived from other registers or a global, or or even a calculated offset.
- `calculate_blt_window_bounds`: Calculates the visible clipping or drawing boundaries by comparing certain pointer-based structures against limits and then populates a table or buffer with visibility or masking data using a loop that writes or copies values from an indexed source or table.
- `clip_blt_window_bounds`: Calculates the clipped or visible area of a bitboard transfer by comparing a source or destination rectangle against the screen or window boundaries.
- `sub_B21E`: undescribed.
- `sub_B24E`: undescribed.
- `sub_B288`: undescribed.
- `sub_B306`: undescribed.
- `update_bitmask_and_enable_dma`: Checks a condition using a global variable, clears a flag in a table, and sets a bitmask in a global variable while updating hardware registers.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set.
- `update_table_and_registers`: Decrements a value in a table and, if it reaches zero, resets several memory locations and hardware registers including audio and DMA or interrupt or even a bitmask.
- `move_word_to_register`: Copies a word from a source memory location or pointer-offset address into a hardware register.
- `execute_sequential_routines`: Calls another routine four times in succession.
- `do_nothing_6`: this routine is empty and contains no executable instructions.
- `sub_B58E`: undescribed.
- `reset_global_variable_4`: Sets a global variable to a constant value if a certain condition is met, otherwise calls another routine.
- `cast_global_to_unsigned`: Performs a redundant type cast or assignment of a global variable to itself.
- `scale_global_variable`: Multiplies a global variable by a constant and right-shifts the result.
- `clear_audio_volumes`: Sets several audio volume hardware registers to zero if a certain global variable is non-zero.
