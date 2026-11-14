# Game 406 (Ishar 2 - Messengers of Doom): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00406). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2750016 top-level bytes in 349 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 61 of 61 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 744 of 744 named (0 author symbols), 744 of 744 described (55 by rule, 653 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4009 of 4009 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2352250 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11424 of 11443 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 85791 |
| code | 42986 |
| data | 2505476 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9937 | 680 |
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
- runtime segment an address: 3760 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `tiny_ori_2`: Small routine of 1 instruction that merges 21 into (a5)+.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 40 into d1, then merges 24 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of longwords from a memory location or pointer or a table to a stack-based or global memory location.
- `init_and_parse_number`: Sets a bit pattern in a global flag word, stores a byte from a table into a structure field, resets several other fields, and calls a parser routine before entering a loop.
- `parse_number_from_buffer`: Increments a counter at offset 8 of the structure in a1, then reads a byte from a table in a0 at that index to classify it as a space, plus, minus, or digit.
- `dispatch_table_index_loop`: Calls a routine with a table index and a pointer, then advances through a byte table storing a computed index until a value falls outside the range 48 to 57 or the counter exceeds its limit, then calls a jumptable routine.
- `call_and_branch_result`: Loads a pointer from a2, calls a routine with a table index and that pointer, stores the returned value in a global variable, then branches to another routine based on the stored and freshly returned values.
- `load_value_and_jump`: Loads a 32-bit value from offset 4 of the structure in a1 into d1, then jumps to the address stored in a6.
- `dispatch_result_based_on_flags`: Stores a shifted pointer at offset 4 of the structure in a1, then makes a series of indirect calls through a5.
- `lookup_dispatch_result`: Tests a table entry at an offset from a0 and, if it is non-zero, loads a pointer from a table, sets up registers, calls a routine, and branches away.
- `lookup_and_dispatch_result`: Looks up a pointer from a table, calls a routine with it, stores the result, then branches on the result and the stored value to either call another routine or perform a further lookup and call before jumping through a table.
- `copy_data_and_call_subroutine`: Copies a longword from offset 212 of the structure in a1 to offset 556 of the structure in a2, sets d0, d1, and d2 to specific constants, calls a subroutine at a5, and jumps to the address in a6.

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
- `copy_data_block`: Copies a block of memory from a source pointer to a destination pointer and updates a global variable with the new pointer, then returns.
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
- `enable_interrupts`: Writes the value 32800 to the interrupt enable register to enable specific interrupt sources, then returns.
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
- `return_constant`: Returns the constant 0 in d0 to the caller without any other effect.
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `do_nothing_stub_3`: The routine is empty and returns immediately without performing any operations or accessing any data.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: calls another routine and returns the concatenated contents of two registers.
- `call_draw_and_move`: Calls another routine to clamp four 16-bit values, then calls two library functions via pointers stored in a global variable using coordinates and values retrieved from a table and registers, finally returning the modified 16-bit values.
- `call_indirect_through_table`: Calls another routine, then calls a routine loaded through a pointer read from a table at a fixed offset, and returns the values held in the data registers.
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
- `compare_and_branch_2`: Compares a byte in d0 against a constant and a global variable, then either returns, branches, or sets d0 to a constant value.
- `lock_directory_by_index`: If the input byte is less than two, stores it, selects a directory name by comparing it, locks that directory, and if the lock succeeds changes to it before returning the stored byte.
- `call_indirect_and_load`: Calls an indirect function, then loads a byte from offset an address of a global variable into d0 and returns it.
- `examine_directory_entry`: Calls two functions via pointers from a global variable to examine a directory and returns a value from a table if the result is non-zero.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_multiple_functions`: Calls a function pointer, then calls that same function again with a register, then calls a different function pointer, and finally assigns a return value to a global variable.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_conditional_function_2`: Checks a global variable and calls a function pointer or certain other functions depending on the value or result of a function call.
- `get_library_result`: calls a function via a pointer and if the result is non-zero, calls two other functions and stores a result in a global variable.
- `compare_global_values`: Compares a longword from a global variable with another longword and branches based on the result of a test on a 16-bit word.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters if a certain condition is met.
- `update_buffer_indices_recursively`: This routine recursively updates a pointer-based table and adjusts buffer start and end indices based on a certain size or offset.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table.
- `initialize_blit_parameters`: Sets several consecutive global bytes to a fixed block of values and calls a routine to configure a destination address and hardware registers.
- `allocate_and_initialize_block`: Computes a size from a stored word, calls an allocation routine, and if the returned pointer is non-zero it initializes the block and stores bookkeeping fields; otherwise it clears a flag.
- `allocate_block_and_write_header`: Computes a row size from a structure field, derives several sequential offsets and sizes from global values, stores the input pointer in a global, writes a fixed word and two differences into the block header, and stores a size value.
- `allocate_aligned_block`: Aligns the input value, subtracts a stored pointer from another stored pointer, and if the result is within bounds advances the stored pointer and stores the new size, otherwise returns a null pointer.
- `restore_registers_and_return_2`: Clears d0, restores d1 and a0 from the stack, and returns the value left on the stack.
- `advance_allocation_pointer`: If a global variable equals the value in d0 minus 8, it adds the longword stored at the address in the global variable to that address and updates the global variable with the result; otherwise it returns.
- `apply_memory_offsets`: Calculates and stores a series of offset pointers and initializes several global bytes.
- `check_d0_range`: Compares d0 against two constants and clears d0 if it falls within the specified range, otherwise it returns.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 6544 in d0 to the caller without any other effect.
- `write_register_and_call_routines`: Stores a longword from register a0 into a custom chip register, then calls six routines in succession, copies register d6 into register d0, and returns.
- `save_and_reset_hardware`: Copies several hardware register values into saved global variables, then writes constants to several hardware registers and resets certain globals.
- `clear_bits_and_wait`: Clears bits 7 and 3 of a hardware register, busy-loops a 100-iteration counter while repeatedly clearing bit 7 of that same register, and finally calls two other routines.
- `delay_3000_iterations`: Loops 3000 times decrementing a register to create a short delay, then returns.
- `wait_cia_port_bit_clear`: Loops until bit 5 of the CIA A port is clear, then returns.
- `wait_cia_port_bit`: Waits for a button press on the CIA port by clearing and setting bits of the CIAB port B register, then calls a helper routine and jumps back to itself to repeat.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `wait_cia_port_bit_loop`: Tests bit 4 of the CIAA port B register and, when that bit is set, calls a helper routine and loops back to test the bit again.
- `toggle_cia_port_bits`: Initializes a counter to 79 and repeatedly clears and sets two bits of the CIAB port B register while calling a helper routine, then decrements the counter until it reaches zero.
- `delay_15000_iterations`: Loops 15000 times decrementing a register to create a longer delay, then returns.
- `pulse_serial_port_until_ready`: Clears a bit in a global variable, writes a control word and a longword to a custom chip block, then polls a status register for a bit a fixed count of times or until a bit in a global variable changes.
- `clear_and_set_structure_offsets`: Clears the 16-bit value at offset 36 of the structure in a6, then stores the value 16 at offset 150 of the same structure.
- `wait_for_status_bit_set`: Reads the CIA B interrupt control register twice, then loops until bit 4 of the value is set before returning.
- `latch_status_into_four_words`: Sets bits 7 and 3 of the CIA B port B register, clears bit 3, and writes four 16-bit values ORed with bit 15 into a structure at offsets 150, 154, 156, and 158 relative to a6.
- `scan_table_and_flag`: Walks a table of 32-bit words, comparing shifted halves against a constant and calling a routine when a match occurs, then sets a global flag and returns.
- `loop_while_global_nonzero`: Repeatedly calls another routine while a global byte is non-zero, then returns.
- `find_value_and_store_index`: Walks a table of 16-bit words looking for one whose swapped bytes equal 17545, stores the matching index into a structure, and sets a flag byte; if none matches it clears the flag and returns.
- `store_value_into_structure`: Stores a 16-bit value from a register into a structure pointed to by a4 and sets a byte at offset 3486 of a global variable to one, unless the pointer already matches a base address in which case it clears that byte.
- `apply_coordinate_offsets`: Calculates a difference between two points and applies that difference to a list of coordinate values while updating certain global variables.
- `execute_indirect_call`: calculates a target address using a base register and an offset from a table, then jumps to that address.
- `initialize_memory_offsets`: calculates and stores a series of offset pointers and initializes several global bytes.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_store_pointer`: Returns a byte from a pointer in a3 and stores the incremented pointer into a memory location at an offset determined by d4 relative to a6.
- `read_word_and_store_pointer`: Reads a byte from a3, shifts it left by 8 bits, reads another byte from a3, and stores the incremented pointer into a memory location at an offset determined by d4 relative to a6.
- `store_pointer_to_table`: reads two bytes from a pointer and stores a pointer to a later offset of that same memory block into another memory location.
- `read_short_at_pointer`: returns a short integer read from a memory address held in a register.
- `read_word_from_pointer`: Reads a byte from the pointer in a3, shifts it left by 8 bits, reads another byte from the pointer, advances the pointer by two bytes, and returns the resulting 16-bit value.
- `read_three_bytes`: reads three consecutive bytes from a pointer and returns them as a single packed value.
- `update_memory_structure_3`: Updates a memory structure or a global variable with a pointer and a constant value based on certain conditional checks.
- `update_memory_structure_2`: Updates a memory structure or a global variable with a pointer and a constant byte value based on certain conditional checks.
- `update_memory_structure`: copies a word, then writes a pointer and a constant byte value into a memory structure at a calculated offset.
- `update_pointer_target`: Checks if a global variable is zero, then updates that variable and a pointer-based memory location with a new value.
- `return_empty`: The routine is empty and returns.
- `load_offset_data`: Loads three bytes from a pointer in a3, performs bit manipulation to combine them into a single value, and branches to another location.
- `handle_stack_value_index`: Tests a global byte and returns if it is nonzero, otherwise pops a value off the stack, stores it into a structure, indexes another structure by it, and saves the index before returning.
- `clear_offset_pointer`: Loads a word from an offset of a pointer in a6 into d4, and if it is non-zero, clears the word at that offset and loads a pointer from a table into a3 before returning.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_clear_node_pointer`: Checks if a global variable is null and compares a value against a register; if a certain condition is met, it clears a short value in a memory structure using a pointer-based offset.
- `get_dereferenced_value`: Stores a pointer in a global variable, calls another routine, and returns a value from that pointer or a returned short.
- `load_global_byte`: Stores a pointer in a global variable, calls another routine, and returns a byte loaded from a global variable or a returned word.
- `read_global_byte`: Returns a byte loaded from a global variable.
- `store_pointer_and_call_helper`: Stores a pointer from a register into a global variable, advances that pointer by three, loads a byte into a register, calls a helper routine, and returns if the result is zero.
- `read_global_three_bytes`: Loads a pointer from a global variable into a3, then constructs a 32-bit value from three bytes at that pointer and updates a3 by adding the resulting value.
- `select_pointer_byte_when_zero`: When the word in d7 is zero, it returns a byte from the address in a3 and updates a3 by adding the sign-extended byte; otherwise it returns the value already in d0 and increments a3 by 1.
- `conditionally_return_pointer_target`: If a certain flag is zero, it returns a value from a pointer, otherwise it returns a register value.
- `conditionally_return_three_bytes`: If a certain flag is zero, it constructs a 32-bit value from a pointer and returns it.
- `conditionally_return_pointer_byte`: If a certain flag is non-zero, it returns a byte loaded from a pointer, otherwise it returns a register value.
- `conditionally_return_pointer_target_2`: If a certain flag is non-zero, it returns a value from a pointer, otherwise it returns a register value.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a3, then returns.
- `process_three_bytes_conditionally`: If the word in d7 is zero, it returns a byte from a3, sign-extends it, swaps the word, adds another byte shifted left by 8 bits, adds a third byte to the address in a3, and returns; otherwise it increments a3 by 3 and returns.
- `tiny_addq_rts_3`: Small routine of 2 instructions that adds 3 to a3, then returns.
- `compare_and_return_byte`: If d7 is not equal to d6, it returns a byte from a3 and updates a3 by adding the sign-extended byte; otherwise it increments a3 by 1 and returns.
- `tiny_addq_rts_4`: Small routine of 2 instructions that adds 1 to a3, then returns.
- `read_two_bytes_high_low`: Compares two registers and, if they differ, reads two bytes from a pointer, shifts the first into the high byte, and advances the pointer by two before returning.
- `advance_pointer_by_two`: Adds 2 to the pointer in a3 and returns.
- `parse_extended_offset_data`: Compares two words in d7 and d6, and if they are not equal, otherwise it reads a byte from a3 and extends it to a word, swaps the high and low bytes, reads another byte from a3, shifts it left by 8 bits, adds it to the previous value, adds the total to a3, and returns.
- `tiny_addq_rts_5`: Small routine of 2 instructions that adds 3 to a3, then returns.
- `advance_pointer_by_signed_byte`: Compares two registers and, if they differ, reads a byte from a pointer, sign-extends it, adds it to the pointer, and returns.
- `advance_pointer_by_one`: Increments the pointer in a3 by 1 and returns.
- `advance_table_pointer`: Reads two consecutive bytes from the table pointed to by a3 into d0, shifts the first byte left by 8 bits, adds the second byte to it, then advances a3 past both bytes and returns.
- `advance_pointer_by_two_2`: Adds 2 to the pointer in a3 and returns.
- `advance_pointer_by_multi_byte`: Compares two registers and, if they differ, reads a byte and sign-extends it, then reads two more bytes combining them into a 16-bit value, adds that value to the pointer, and returns.
- `advance_pointer_by_three`: Adds 3 to the pointer in a3 and returns.
- `copy_register_d7_to_d6`: This routine copies the contents of one register to another.
- `execute_eight_instructions`: This routine executes eight instructions and is called by four other routines.
- `execute_indirect_call_2`: This routine calculates an address using a pointer and an offset and calls the resulting address.
- `copy_register_d7_to_d6_2`: This routine performs a single instruction that copies the contents of one register to another.
- `undefined_routine`: This routine contains eight instructions and is called by three other routines.
- `copy_register_value_2`: Performs a single instruction to copy the contents of one register to another and calls another routine.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `store_multiple_d7`: Calls another routine five times and stores the resulting value of d7 into five different global locations as either words or bytes.
- `store_d7_to_globals`: Calls another routine four times and stores the resulting value of d7 into four different global locations as either words or bytes.
- `clear_flag_then_call_routines`: Clears a byte in a global variable, then calls two other routines in sequence before returning.
- `clears_and_calls_routines`: Clears a byte in a global variable, then calls two other routines.
- `store_byte_and_branch`: Stores a byte from d0 through a0, then branches to another routine if the value is 13 or 8, otherwise it returns.
- `tiny_clr_rts_4`: Small routine of 2 instructions that clears (a0)+, then returns.
- `clear_byte_and_call_helper`: Checks a global pointer and returns if it matches a0, otherwise clears a byte at that pointer and calls a helper routine three times with the values 32, 32, and 8 before jumping to a shared section.
- `filter_and_store_byte`: Fetches a pointer from a global variable, then reads a byte and branches away when it is a carriage return, tab, hyphen, or outside the digit range.
- `clear_byte_then_scan`: Clears the byte at a pointer and advances the pointer, then calls another routine and returns.
- `store_zero_and_send_spaces`: Compares a pointer in a0 with a global address and branches away if they match, otherwise stores a zero byte at the pointer and calls the string handler with the space character, then with the value eight, before branching away.
- `iterate_until_negative`: Calls another routine and branches to a different location if the returned byte is negative, otherwise it restores registers and returns.
- `set_one_and_swap_globals`: Stores the value one into a global byte, calls the swap routine, then calls another routine before returning.
- `set_one_and_do_tiny_move`: Stores the value one into a global byte, calls the move routine, then calls another routine before returning.
- `set_two_and_swap_globals`: Stores the value two into a global byte, calls the swap routine, then calls another routine before returning.
- `set_two_and_do_tiny_move`: Stores the value two into a global byte, calls the move routine, then calls another routine before returning.
- `clear_byte_and_do_tiny_move`: Clears a global byte to zero, calls the move routine, then calls another routine before returning.
- `clear_byte_and_loop_buffer`: Clears a byte in a global variable, copies the next byte from the buffer in a3 to d0, and loops while that byte is not zero before returning.
- `call_handler_and_branch`: Calls the string handler and then branches to another routine, so it performs no other work of its own.
- `clear_byte_and_swap_globals`: Clears a global byte to zero, calls the move routine, then calls another routine before returning.
- `process_null_terminated_string`: Loads a pointer from a global variable, checks if the first byte is zero, and if not, calls another routine and branches to another location.
- `copy_d7_and_iterate_string`: Copies the value in d7 to d0, then calls the string iterator routine and returns.
- `store_and_call_routine`: Calls two routines, stores the resulting bytes from d7 and d6 into two global variables, and then calls another routine using d0 and d1 as arguments.
- `store_byte_and_call_tiny_move`: Calls the scaled offset calculator, stores the byte in d7 into a global variable, loads the byte at offset 1 of that variable into d1, calls the tiny move routine, and returns.
- `set_global_from_function`: Calls a routine and updates a global variable with either a returned value or a constant if a certain condition is met.
- `process_string_characters`: Modifies global or buffer-based counters or pointers based on whether a character matches newline, carriage return, or space, and calls another routine if certain conditions are met.
- `iterate_string_characters`: Iterates through a null-terminated string, calling a routine to process or convert each character or integer-string representation until a null terminator is encountered.
- `convert_integer_to_string`: Converts a signed integer or a value from a register into a string or a sequence of characters, including handling a negative sign, and stores the result in a buffer.
- `store_word_via_indexed_pointer`: stores a word from a pointer-referenced location into a destination address calculated using a base pointer and an index multiplied by two.
- `execute_eight_instructions_2`: This routine executes a fixed number of eight instructions and is called by several other routines.
- `store_table_result_values`: Pushes a register onto the stack, calls a table helper routine, stores three 16-bit values into a structure at offsets 24, 26 and 28, then jumps to another routine.
- `execute_table_operations`: Executes a fixed number of instructions, performs a binary search through a table of words, and resets a string buffer or linked list structure.
- `delete_memory_node`: Deletes a node from a managed memory list by shifting remaining pointers to close the gap and updating a linked list of pointers.
- `check_global_null`: Checks if a global variable is zero and, if not, calls another routine.
- `test_d5_and_branch`: Tests the word in d5 and branches away if it is nonzero, otherwise it jumps to another routine.
- `shrink_memory_block`: Reads a node pointer from a global variable and, if it is non-zero, shifts the node's data block left by a computed size and decrements the remaining size.
- `traverse_string_buffer`: Traverses a string buffer and performs conditional checks and updates or calls other routines if certain string termination or data conditions are met.
- `find_minimum_from_pointer`: If a certain value matches a value in a structure, it searches through a table to find the minimum integer value using a pointer obtained from another routine.
- `update_linked_list_node_2`: Modifies a bit in a byte, copies a sequence of bytes from one location to another, and updates several fields in a structure or list node.
- `store_byte_via_offset`: Stores a byte from a source pointer into a global buffer at an offset given by a signed word read from the same pointer.
- `set_global_if_zero`: Checks if a global variable is zero and sets it to one if it is.
- `reset_global_variable`: Sets a global variable to zero.
- `update_structure_fields_2`: Calls another routine three times, calculates a pointer using a global variable and an offset from a6, stores d7 into three consecutive word offsets of that pointer, and sets bit 7 of the byte at that pointer.
- `subtract_and_update_structure`: Calls another routine three times, calculates a pointer using a global variable and an offset from a6, subtracts values from offsets of that pointer from d7, stores the results into three word offsets, and sets bit 7 of the byte at that pointer.
- `update_if_different`: Compares a value from a pointer to a value at a calculated offset and updates that offset if they differ.
- `update_table_values_2`: If a certain value is non-zero, this routine writes a register or a value to a memory location calculated using a global variable and an offset, then sets a high bit in a byte at a pointer.
- `store_structured_status`: Calls two other routines, then if the 16-bit value at offset -22 of the structure in a6 is zero, stores the 16-bit values from registers d7 and d6 at offsets 18 and 20 of that structure indexed by that value and sets bit 7 of its base byte.
- `store_and_flag_entry`: Reads a 16-bit value from offset -22 of the structure in a6, and if it is nonzero stores the contents of register d7 at offset 14 and the contents of register d6 at offset 16 of a table indexed by that value, then sets bit 7 of the byte at the start of that entry.
- `store_byte_and_set_flag`: If a byte at offset -22 of the structure in a6 is not zero, it calls another routine three times, then stores the byte from d7 at offsets 34, 33 and 32 of a structure and sets bit 7 of its first byte.
- `store_byte_and_cross_products`: Checks a flag byte at offset 22 of the structure in a6 and, if set, calls execute_indirect_call_3 three times in a row.
- `copy_and_recompute_cross_products`: Reads three consecutive bytes from a table indexed by a global 16-bit value and copies them into three consecutive bytes of a structure in a0, then calls calculate_cross_products and sets bit 7 of the first of those bytes.
- `store_structured_status_bytes`: Reads a 16-bit value from a structure in a6 and, if it is nonzero, computes a pointer from it plus a global table entry, stores a byte from d7 at offset 29 and the decremented d6 value at offset 30, then stores the same byte from d7 at offset 31 and sets bit 7 of the byte at offset 0.
- `conditionally_set_status_flag`: Calls another routine, then reads a 16-bit value from offset -22 of the structure in a6; if that value is zero it returns without doing anything, otherwise it stores the byte in d7 at offset 28 plus that value into the structure pointed to by a global variable and sets bit 7 of that same byte.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `store_byte_and_set_flag_2`: Stores the byte in d7 through a pointer at offset 9C58 of a global structure, then stores the same byte as a word at offsets 9C5A, 9C5C and 9C5E, calling an indirect-call helper before each store.
- `modify_bit_flags`: This routine clears a specific bit and sets another bit in a byte located at an offset from a global variable, while also performing linked list insertion and removal operations.
- `set_bit_in_byte`: Sets the certain bit in a byte located at an offset determined by a pointer.
- `insert_into_linked_list`: Inserts a new value into a linked list or updates a pointer within a structure if the list is empty or certain conditions are met.
- `remove_from_linked_list`: Traverses a linked list to find a specific value and updates a pointer or replaces a node if a match is found.
- `calculate_cross_products`: Performs a series of signed byte multiplications and subtractions, storing the results in a structure.
- `set_byte_at_offset`: Sets a byte at a specific offset relative to a register value to one.
- `clear_a6_struct_byte`: Clears the byte at offset -3 of the a6 structure to zero and returns.
- `toggle_a6_struct_bit`: Toggles bit 0 of the byte at offset -3 of the a6 structure and returns.
- `process_string_data_stream`: Parses a data buffer to update a set of global counters and pointers, while conditionally copying or modifying word-sized or byte-sized values into a destination buffer based on certain flag or value checks.
- `copy_string_with_offset_and_transform`: Copies a sequence of words from a source buffer to a destination buffer while applying a signed or unsigned transformation to the data and updating a global offset.
- `parse_string_stream_tokens`: Iterates through a data buffer to extract and apply a sequence of length-prefixed or type-prefixed commands, updating global counters and pointers.
- `expand_and_transform_string`: Copies and expands a sequence of words from a source buffer to a destination buffer, applying a signed or unsigned transformation to the data.
- `parse_string_data_stream`: Iterates through a data buffer to update or copy a sequence of word-sized or byte-sized values into a destination buffer, or modifies a global counter and pointer based on conditional flags or signed-value checks within the stream.
- `store_constants_then_branch`: Stores the constant one into a global byte, copies the byte at offset -3 of the a6 structure into another global byte, then branches into another routine and does not return.
- `toggle_bit_then_branch`: Stores the constant one into a global byte, copies the byte at offset -3 of the a6 structure into a register, toggles bit 0 of that register, stores the result into another global byte, then branches into another routine and does not return.
- `init_and_process_string_stream`: Sets a global variable to 1, toggles bit 0 of the byte at offset -3 from a6, stores it at offset -10 from a6, and calls process_string_data_stream.
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
- `clear_byte_at_a6`: Clears the byte at offset -21 of the structure in a6 and returns.
- `set_byte_at_a6`: Sets the byte at offset -21 of the structure in a6 to 255 and returns.
- `select_lowest_set_bit`: Calls another routine, then selects the lowest set bit among bits 4 through 1 of the value in d7 (defaulting to 0 when none are set) and stores the result at offset -44 of the structure in a6.
- `store_d7_at_a6`: Calls another routine and stores the value from d7 at offset -43 of the structure in a6 before returning.
- `store_d7_and_conditional_move`: Calls the move routine, stores the value in d7 into a local slot, compares that value with nine and branches away if equal, then calls the move routine three more times, reloads the local slot into d0 and branches away if it is zero, otherwise returns.
- `reset_global_flags`: Clears a global byte, sets a global word at offset 39062 to 65535, and clears a global word at offset 39074 before returning.
- `store_d7_into_globals`: Copies a word from offset -14 of the structure in a6 to a global word, then ensures d7 holds 1 when it is zero before storing it into two global bytes.
- `compute_scaled_vector_offset`: Calculates a new position and direction by applying a bit-shifted scale factor to a vector, then updates global destination coordinates.
- `get_conditional_value`: Returns a value based on a comparison of a byte at an offset or a constructed 32-bit value.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `resolve_indirect_pointer_offset`: calculates a target address by traversing multiple levels of indirection and adding a scaled offset or a value retrieved from a pointer-based table.
- `calculate_pointer_offset`: Calculates a pointer or value by adding an offset and a value retrieved from a table or memory location.
- `toggle_global_bit`: Sets a global variable to a value loaded from a register and uses a loop to check if that value is zero.
- `search_and_traverse_list`: Sets a global byte to 1, then searches a pointer table.
- `traverse_linked_list_nodes`: Traverses a linked list and resets a global variable to zero if a certain condition is met or when the end of the list is reached.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `set_triple_word`: Assigns a 16-bit value to three consecutive memory locations or a table.
- `add_d7_to_a6_words`: Calls another routine, then adds the value from d7 to the words at offsets 0, 2, and 4 of the structure in a6, calling another routine before each addition, and returns.
- `add_bytes_to_a6_words`: Loads each byte at offsets 9, 10, and 11 of the structure in a6, sign-extends them to words, and adds each result to the corresponding word at offsets 0, 2, and 4 of the structure in a6 before returning.
- `accumulate_values_into_structure`: Adds three 16-bit values from registers to the words at offsets 0, 2 and 4 of a structure, then returns.
- `process_structure_bytes_via_indirect_call`: Calls another routine three times, then reads the three bytes at offsets 9, 10 and 11 of the structure in a6, passes them as 16-bit values to that routine, and writes the results back to the same three offsets.
- `store_three_bytes_indirect`: Calls another routine, then computes a pointer from a global variable and a value in a6, and stores three consecutive bytes from that pointer into offsets 9, 10 and 11 of the structure in a6.
- `load_pointer_and_jump`: Loads the constant value at offset an address into address register a0 and then jumps to the routine at offset 19954.
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
- `store_result_after_calls`: Loads the constant 16 into a register, calls a routine after setting that register, runs another routine, then stores the value left in a register at offset 26 below the base pointer, and returns.
- `write_max_value_after_call`: Sets a register to the constant 18, calls a routine, writes the constant 65535 at offset 26 below the base pointer, and returns.
- `store_result_then_return`: Runs another routine, then stores the value left in a register at offset 26 below the base pointer, and returns.
- `combine_table_values_signed_2`: Reads a byte from a table in a0 and branches to a helper routine when it is zero, negative, or one, otherwise it adds three 16-bit values from the table (each offset by half of a fourth value) and stores the combined result.
- `combine_table_values_signed`: Reads three signed 16-bit values from a table in a0, each offset by half of a fourth signed value read from the same table, and packs the three results into a 32-bit register.
- `combine_table_bytes_signed`: Reads four signed bytes from a table in a0, adds each of the first three to half of the fourth, and returns the combined signed result.
- `accumulate_string_buffer_offsets`: Initializes the string buffer, then adds three 16-bit values from offsets 0, 2 and 4 of the structure in a6 to a running total and stores the results into consecutive global variables, before calling another routine.
- `read_word_and_maybe_advance`: Loads a pointer from a global variable, reads a 16-bit word from that address into a register, and if the sign bit is set jumps to another routine; otherwise it increments the global pointer by 2 and branches to a third routine.
- `check_pointer_and_pop_stack`: Loads a pointer from a global variable, compares it against a fixed address, and if they differ pops a word off the stack, decrements the pointer by two, and branches away.
- `load_value_and_branch`: Loads the constant 65535 into a register, then branches to another routine.
- `store_pointer_and_branch`: Stores a fixed address into a global variable, then branches to another routine.
- `read_offset_word_and_branch`: Loads a pointer from a global variable, reads the 16-bit word at offset 254 from that address into a register, and branches to another routine.
- `walk_entries_and_push_stack`: Saves registers on the stack, stores a fixed address into a global variable, and returns immediately if a global word is negative.
- `match_and_push_stack`: Iterates a table of entries comparing fields, calls a routine per matching entry while storing values into a global stack, and records a sentinel when the loop ends.
- `handle_stack_result_flag`: Calls another routine, restores registers from the stack, and branches to a handler if the result is zero.
- `search_stack_entries_indexed`: Pushes a value onto a stack in a global variable, then walks entries by index until it finds a matching one or exhausts the stack, clearing the stack when none matches.
- `iterate_table_elements`: Reads a byte from the buffer in a3, and if it is nonzero iterates that many times calling another routine for each non-negative word in the buffer, stopping early when a word is zero.
- `index_table_and_check_sign`: Doubles a register, indexes into a table with the result, and branches away if the examined word is negative.
- `increment_a3_and_branch`: Increments a3 by two and branches away if the result is zero, then branches away if the byte at offset negative four of a3 is one, otherwise it returns.
- `compute_clamped_rect_bounds_2`: Reads six signed coordinates from a buffer, applies a sign flag from offset 3 of the structure in a6, clamps them into a bounding rectangle, stores the rectangle in global variables, then processes a table of 16-bit values in a loop, calling compute_clamped_rect_bounds for each entry until the end marker is reached.
- `compute_clamped_rect_bounds`: Reads six signed coordinates from a buffer, applies a sign flag from offset 3 of the structure in a6, clamps them into a bounding rectangle stored in a global table, then processes the marker buffer pointed to by a2 by calling intersect_rect_bounds for each entry or by calling traverse_buffer_with_bounds when the marker is negative.
- `return_without_effect`: The routine consists of a single return instruction and performs no work, so it acts as an empty stub that update_window_dimensions can call without side effects.
- `traverse_buffer_with_bounds`: Traverses a buffer while checking a length byte and calling another routine if a certain condition is met.
- `intersect_rect_bounds`: Checks if a rectangle, defined by a pointer-based offset and certain width/height modifiers, overlaps with a global boundary.
- `check_rect_intersection_2`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary and returns a modified pointer.
- `check_rect_intersection`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary, and returns a masked or modified pointer or value.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d1, then returns.
- `iterate_buffer_elements_2`: Reads a 16-bit value from a structure and returns early if it is negative or the stack pointer matches a saved base.
- `store_word_and_return_and`: Loads a 16-bit value from the address in a2, stores it at offset an address of a global variable, and returns the bitwise AND of that value with another value, then returns.
- `tiny_cmp_rts`: Small routine of 2 instructions that compares d0 with d0, then returns.
- `iterate_buffer_elements`: Reads a byte from the buffer in a2, and if it is nonzero iterates that many times calling another routine for each non-negative word in the buffer, stopping early when a word is zero.
- `index_table_and_branch_on_sign`: Doubles the value in d0, uses it as a byte offset to index a table through a2, then tests the byte at that location and branches back to a fixed address when it is negative.
- `store_and_and_word_global`: Reads a 16-bit value from the address in a2, stores it at offset an address of a global variable, and returns the bitwise AND of that value with the byte at offset an address of the same global variable in d0.
- `call_and_store_result`: Calls a routine and stores its result into a global variable, loads a fixed address into a register, clears a register, fetches a pointer from a global variable, then branches away and returns.
- `process_indexed_structure_buffer`: Reads a 16-bit value from a structure at offset 4 indexed by a counter, and if it is zero it stores a fixed address into a global variable and branches away.
- `prepare_dispatch_registers`: Calls another routine, then tests register d7 and clears its high byte when the sign bit is set.
- `index_table_and_dispatch_queue`: Reads a 16-bit value from a table indexed by a register and a pointer, then compares the pointed-to structure's 16-bit field at offset 16 with a register; if they differ or the pointer equals a6 it stores the index into a global variable and calls the queue handler, otherwise it returns.
- `call_indexed_dispatch`: Calls a routine that reads a 16-bit value from a table indexed by a register in a0, then looks up a structure using that value and compares its byte at offset 12, and returns.
- `store_index_and_dispatch`: Reads a 16-bit value from a table indexed by a register in a0, then looks up a structure using that value and compares its byte at offset 12 with a register and its address with a structure in a6.
- `store_table_value_indirect`: Loads a fixed global address into a2, clears d1, reads a 16-bit value from a table via a0, and if that value is nonzero stores it at the address in a2 while advancing that pointer, then returns.
- `store_pointer_and_branch_2`: Stores the value 65535 at the address in a2 and advances that pointer, then overwrites a2 with two fixed global addresses and finally a third fixed address before clearing a byte at offset 190 of a global structure and branching to another routine.
- `bubble_sort_two_arrays`: Performs a bubble sort on two adjacent arrays of 16-bit words, swapping neighboring entries when the second array's value is greater than the first, and repeating passes until no swaps occur.
- `compute_square_and_init_table`: Clears a byte in a global variable and stores the square of a 16-bit value in a global variable at offset 32 and a 16-bit copy at offset 28, then calls a routine.
- `store_conditional_buffer_entry`: Iterates over a buffer of 16-bit entries, skipping each one whose value is zero, and for the first non-zero entry stores the value 65535 into a pointer in a2 while decrementing it, then stores a 32-bit value into a pointer in a3 while incrementing it.
- `store_pointer_conditional_sort`: Stores the value 65535 at the address in a2 and advances that pointer, then returns immediately if a byte at offset 190 of a global structure is zero, otherwise calls another routine, loads a3 from the stack, sets a2 to a fixed address, clears that same byte, and branches to another routine.
- `square_and_process_table`: Squares the 16-bit value in d7, stores the result in a global variable, then calls another routine and, if the result is negative, clears a byte at offset 30 of a structure in d0 and loads pointers from fixed addresses to process a table.
- `match_table_entry_by_structure_fields`: Reads a 16-bit value from a table indexed by a0 and d1, then compares the pointer stored at the next table entry against a structure in a6 by their -16 offset and their 6 offset; if any comparison differs it returns, otherwise it calls another routine and stores a matching result into a global variable.
- `store_pointer_conditional_sort_2`: Stores the value 65535 at the address in a2 and advances that pointer, then returns immediately if a byte at offset 190 of a global structure is zero, otherwise calls another routine, loads a3 from the stack, sets a2 to a fixed address, clears that same byte, and branches to another routine.
- `square_index_and_clear_flag`: Clears a byte in a global variable, squares the value in d7 and stores the result in another global variable, then clears a flag in a structure and, if the flag is set, writes a large constant and a maximum value into a table.
- `match_and_store_entry`: Reads a 16-bit index from offset 4 of the table in a0, then for the entry it points to (address at offset 0) compares its byte at offset 12 with d7, its word at offset 6 with the word at offset 6 of the structure in a6, and its address with a6; if any differ it returns.
- `store_pointer_and_branch_3`: Stores the value 65535 into the word pointed to by a2 and advances the pointer, then checks a global variable; if it is zero it calls another routine and restores the stack pointer, otherwise it clears that global variable and branches to another routine.
- `store_squared_distance_index`: Squares the value in d7 and stores it in a global variable, then indexes a table with that squared value to read a pointer and compares its word at offset 6 with the word at offset 6 of the structure in a6.
- `store_squared_values`: Stores the byte value 1 into a global variable, then multiplies a 16-bit value in register d7 by itself and stores the 32-bit result into a global variable at offset 48 and a 16-bit copy into a global variable at offset 44, repeating this for a second 16-bit value in register d7.
- `compute_squared_distance_check`: Saves registers d1 through d5 on the stack, then computes the sum of the squared differences between the words at offsets 0, 2 and 4 of the structure in a1 and the corresponding words in the structure in a6.
- `restore_registers_and_return`: Restores the saved registers from the stack and returns immediately without modifying any data.
- `compute_weighted_sum_and_compare`: Calls a subroutine, then multiplies three signed bytes read at offsets 9, 10 and 11 of the structure in a6 by the values in d1, d2 and d3, adds the products together, and multiplies the sum by a global word.
- `set_global_byte_to_one`: Sets the byte at offset an address of a global variable to one and returns.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_block_4`: Copies a block of memory from a source pointer to a destination pointer, decrementing a counter and updating a pointer for each word or byte transferred.
- `search_table_for_value`: Iterates through a table or list, checking if a value matches a certain constant, and updates a pointer or counter during the process.
- `return_empty_routine_2`: The routine performs no operations and immediately returns to its caller.
- `tiny_adda_rts`: Small routine of 2 instructions that adds (a3)+ to a3, then returns.
- `read_byte_then_advance_pointer`: Calls an indirect routine, reads a byte from the pointer in a3 into d1, then advances a3 by one or two bytes depending on its alignment.
- `double_add_increment_a3`: Doubles the value in d1, adds two to it, adds the result to a3, and returns without storing anything.
- `do_nothing_stub_4`: The routine contains no instructions and returns immediately.
- `clear_table_entry`: If a certain condition is met, writes zero to a memory location determined by an offset in a table.
- `clear_byte_conditional_branch`: Clears the byte at offset 4 of the structure in a6, sets the byte at offset an address of a global variable, and branches to another routine unless that global byte is nonzero, in which case it returns.
- `set_table_entry_from_result`: Calls another routine, and if its result is nonnegative, uses it as an index into a table of pointers to store one at offset 4 of the pointed-to structure.
- `fill_sequence_via_indirect_calls`: Reads a byte from the address in a3, calls another routine, and if the result is valid, repeatedly calls it while storing the result index into a sequence and adjusting a counter until the sequence is complete.
- `advance_field_clear_flag`: Compares two 16-bit fields of the structure in a6 and returns negative one if they are equal, otherwise advances one field by a computed amount and clears a flag byte when the fields become equal.
- `copy_word_clear_flag_byte`: Copies the 16-bit value at offset 28 of the structure in a6 into offset 30 and clears bit 7 of the flag byte at offset 36.
- `set_flag_then_call_subroutine`: Sets bit 0 of the flag byte at offset -24 of the structure in a6, calls a subroutine, and returns.
- `clear_bit_zero`: Clears the least significant bit of a byte at a calculated offset from a pointer.
- `set_bit_one`: Sets the second least significant bit of a byte at a calculated offset from a pointer.
- `clear_bit_one`: Clears the second least significant bit of a byte at a calculated offset from a pointer.
- `fetch_indirect_call_target`: loads a byte from an indirect call target and stores it into a global variable.
- `copy_memory_block_5`: Copies a block of bytes from a source table to a destination address, where the block size is read from the source table.
- `insert_and_lookup_pairs`: Calls two helper routines, then if a value in d7 is negative it clears a global byte and several fields of a global structure and calls a table updater.
- `compare_word_then_store_and_call`: Compares the word at offset 2 of the structure in a0 with the value in d6 and returns immediately when they are equal.
- `clear_globals_and_update_tables`: Clears a byte at offset 117 of a global structure, zeroes a 16-bit value at offset 12 of a global variable, stores 255 at offset 16, and clears the 16-bit value at offset 18, then calls the routine that updates pointer and string tables.
- `set_global_to_one`: Masks a register value and then overwrites a global variable with a constant value of one.
- `update_global_pointers`: Calculates a new address based on a global variable and stores it into two global variables.
- `evaluate_state_and_sync_data`: Checks a global variable against several constant values and, depending on the result, recursively calls another routine or performs a block copy of data from a source pointer to two destination pointers.
- `copy_words_when_index_valid`: Calls an indirect routine and, when the value in d7 is between negative one and negative four inclusive, copies thirty-one consecutive 16-bit words from a source table into a destination table, otherwise it returns.
- `reset_global_variable_2`: Sets a global variable to zero through a series of assignments.
- `update_table_with_register`: If a global variable does not match certain allowed values, stores a register value into a table at an offset calculated from another register.
- `compute_digit_sum_table`: Calls two helper routines, then if a global byte is not one of four specific values it computes a digit sum by calling another routine three times, combining the results into a single value, storing that value at an indexed position in a table, and clearing a global flag before returning.
- `assemble_bytes_and_store_result`: Negates the value in d6 and calls a subroutine three times, each time shifting d1 left by 4 and then 8 bits and OR-ing the results into d3.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `check_nibble_difference`: Calculates the difference between two lower nibbles and returns if the result is negative or outside a certain range.
- `reset_and_copy_global_data`: Resets a global variable and performs conditional block copies of data between a source pointer, a global variable, and a destination pointer or another global variable.
- `reconstruct_nibble_data`: Iterates through a buffer to detect or modify nibble-based data encodings.
- `reset_and_copy_data`: Checks if a global variable matches certain values, resets it, and then copies a block of memory from a source pointer to a destination pointer, followed by a conditional copy of a small number of words.
- `verify_nibble_sum_threshold`: tests if the sum of two lower nibbles exceeds a constant value and returns the result.
- `compare_nibble_range`: Checks if the difference between two lower nibbles falls within a specific range or if the result is negative.
- `call_routines_then_set_flags`: Calls an indirect routine five times, pushing the value in d7 on the stack after each call, then pops the five values back into d0 through d6.
- `store_indirect_results_sequentially`: Invokes an indirect routine eight times in a row, storing the resulting 16-bit value from d7 into each of eight consecutive 16-bit fields of a global variable, then returns.
- `store_stack_word_and_call`: Loads a word from the stack into a global variable at offset 1024, then calls a routine four times in succession, and finally returns.
- `store_flag_and_call_routines`: Calls a routine, stores a 16-bit value from register d7 into a global variable, clears a 16-bit global variable, sets a byte at offset 317 of a structure to one, and then calls another routine.
- `store_stack_word_and_call_2`: Loads a word from the stack into a global variable at offset 1024, then calls a routine four times in succession, and finally returns.
- `set_negative_division_result`: Clears a 16-bit global variable, divides a shifted global value by another global word, and if the quotient is non-zero stores its negation into a global variable, sets a byte at offset 318 to three, and calls a routine.
- `store_divided_globals`: Clears a 16-bit global variable, then stores the byte at offset 52, the byte at offset 60, the word at offset 56, the byte at offset 54 and the word at offset 52 from the same register, calling the division routine before each store, and returns if that register is zero.
- `set_negative_division_result_two`: Clears a 16-bit global variable, divides a shifted global value by another global word, and if the quotient is non-zero stores its negation into a global variable, sets a byte at offset 318 to two, and calls a routine.
- `call_indirect_routine_four_times`: Calls an indirect routine four times in succession and then returns.
- `store_command_block_and_ratio`: Stores a word and bytes from register d7 into several global variables with calls between the stores, and when the input word is non-zero computes a scaled ratio, stores its negative, sets a global flag, and calls a routine.
- `write_command_record_by_type`: Saves registers on the stack, then compares a global byte and word against a set of fixed table entries to select a record layout.
- `store_byte_then_cast_unsigned`: Calls an indirect routine, stores the resulting byte from d7 into two consecutive global bytes, then calls a routine that casts a global value to unsigned and returns.
- `normalize_byte_then_scale_global`: Calls an indirect routine, replaces the d7 value with one when it is zero, stores that byte into a global variable, calls a routine that scales a global value, and returns.
- `test_flag_then_branch`: Tests a byte at offset 61 of a global variable and returns immediately if it is zero, otherwise it branches to another routine.
- `lookup_record_pointer_entry`: Computes a base pointer from a structure in a6, indexes into a table using four times the value in d7, and returns the sum of the pointed-to 32-bit value and that pointer address.
- `swap_context_and_call_subroutine`: Saves a6 to the stack, loads a new address from a global pointer into a6, calls a subroutine, and restores a6 before returning.
- `store_one_and_call_triple`: Stores the decimal value 1 into a global variable, then calls a routine three times in a row.
- `store_when_byte_matches`: Calls a routine three times, then checks the sign of a returned value and branches away if negative.
- `store_table_when_flag_matches`: Calls a routine, reads a byte from the returned pointer, and when that byte is one, two, or five stores a modified value and a register into a table indexed by a masked and shifted register.
- `set_flag_and_call`: Sets a byte at offset 145 of a global variable to one, then calls a routine and returns.
- `branch_when_byte_is_four`: Calls two routines, reads a byte from a pointer, and if that byte equals four branches to another routine, otherwise falls through.
- `setup_counters_and_divide`: Stores an incremented value into a global pointer, sets up three byte and two word counters, divides one counter by each of the others while guarding against a zero divisor, calls a routine at a fixed address with prepared arguments, sets a flag byte, and returns.
- `call_indirect_routine_five_times`: Calls an indirect routine five times in a row and then returns.
- `store_word_then_reset_global`: Calls an indirect routine, stores the resulting 16-bit value from d7 into a global variable, calls a routine that resets a global value, and returns.
- `store_d7_indirect_call`: Calls the indirect call routine, then stores the value in register d7 at the global variable at offset a fixed address, and returns.
- `store_command_and_flag_when_matched`: Clears a byte, stores several values from register d7 into global variables with calls between the stores, and when a byte is one or two and a flag is zero stores a pointer and a modified value, sets a flag, and calls a routine.
- `set_flag_and_call_2`: Sets a byte at offset 145 of a global variable to one, then calls a routine and returns.
- `call_execute_sequential_routines`: Call stub: calls execute_sequential_routines, then returns to its own caller.
- `read_word_then_branch_indirectly`: Copies the pointer in a3 into a0, then checks the byte it points to and returns immediately when it equals 255.
- `increment_pointer_branch`: Increments the pointer in a3, calls two indirect call routines, loads a global pointer into a0, copies d7 into d0, then branches to the routine at 38266.
- `update_object_position`: Calculates and applies a new position or offset to a structure in memory, including conditional logic for handling directional or certain type-specific movement adjustments.
- `update_window_dimensions`: Updates a set of window or clipping-area coordinates by adding or subtracting offsets or certain values retrieved from a table.
- `reset_global_variable_3`: Checks if a certain global variable and another global variable meet certain conditions, then clears that global variable.
- `check_flags_and_clear`: Tests the long at a fixed address and returns when it is zero, then tests a byte at another fixed address and returns when its high bit is set, and finally clears that same byte when the value equals two before returning.
- `set_byte_to_255`: Stores the constant 255 into the global byte at offset a fixed address, then returns.
- `clear_global_variable`: Performs several calls to other routines and then sets a global variable to zero.
- `clear_and_validate_pointer`: Clears a byte at a fixed address, calls two helper routines, then inspects the byte pointed to by a0 and returns when that byte is zero or equals sixteen or twenty.
- `store_a0_global`: Stores the value in register a0 into the global long at offset a fixed address, then returns.
- `call_scale_registers`: Calls an indirect call routine, copies register data, scales input values, then returns.
- `compute_byte_offset`: Reads a byte from the pointer in a3, doubles it, adds the previous byte, adds 3, advances a3 by that amount, and returns the computed value in d0.
- `read_combine_store`: Reads a byte from a3, shifts it left by 8 bits, reads the next byte, combines them into a word, and stores it at offset -32 in a6, then returns.
- `copy_bytes_offset`: Reads a byte from a3 as an offset, then copies five bytes from a3 into the structure at a6 using that offset, decrementing a counter, and returns.
- `store_two_bytes`: Reads a byte from a3 and stores it at offset -34 in a6, reads the next byte and stores it at offset -33 in a6, then returns.
- `call_indirect_twice`: Calls the indirect call routine twice in sequence, then returns.
- `store_d7_via_indirect_call`: Performs an indirect call, then stores the 16-bit value from register d7 into the 16-bit word at offset 2 of the structure pointed to by a0, and returns.
- `load_address_and_branch_2`: Loads an address from a fixed global location into a0, indexes it with a value in d7, and copies three words from the resulting structure into registers d1, d2 and d3.
- `load_address_and_branch`: Loads the address at a fixed address into a0 and then branches back to an earlier routine, performing no other work of its own.
- `accumulate_weighted_bytes`: Reads three signed bytes from consecutive offsets of the table in a0, multiplies each by the values in registers d1, d2 and d3 when the byte is nonzero, accumulates the products into d4, and returns the result in d7.
- `copy_register_data`: Copies the contents of one register to another register.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer located within a table.
- `swap_global_values`: Swaps the contents of two adjacent global variables and then performs an indirect call using a pointer retrieved from a table or memory location.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine_3`: the routine performs no operations and immediately returns to the caller.
- `copy_string_data`: Copies a null-terminated string from a source address to a destination address.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 6 instructions in the listing and performs no work.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or an immediate return, effectively acting as a null operation.
- `copy_string_via_index`: Computes an address from two bytes in the table at a3 scaled by 256 and added to the base in a6, then copies bytes from that address into the buffer at a0 until a zero byte is stored, and returns.
- `load_weighted_sum_result`: Builds a 16-bit index from two bytes in the table at a3 scaled by 256, calls the weighted-sum routine, loads the 16-bit result from the indexed offset of the structure in a6 into d7, extends it to a signed word, and returns.
- `load_weighted_product_result`: Builds a 16-bit index from two bytes in the table at a3 scaled by 256, calls the weighted-product routine, loads the 16-bit value from the indexed offset of the structure in a6 into d7, and returns.
- `copy_indexed_byte_sequence`: Loads two bytes from a table through a3 into d0, shifts the first left by eight, calls a routine, indexes a table in a6 by the result, then copies bytes from one table to another until a zero byte is reached, otherwise it returns.
- `return_immediately_2`: The routine executes a return instruction to exit the current function.
- `return_empty_routine_4`: The routine contains no executable instructions and performs no operations before returning.
- `copy_string_from_table`: Copies a null-terminated string from a location determined by an offset in a table to a destination address.
- `load_weighted_sum_zeroed`: Clears register d0, loads a byte from the table at a3 into it, calls the weighted-sum routine, loads the 16-bit result from the indexed offset of the structure in a6 into d7, extends it to a signed word, and returns.
- `load_weighted_product_zeroed`: Clears register d0, loads a byte from the table at a3 into it, calls the weighted-product routine, loads the 16-bit value from the indexed offset of the structure in a6 into d7, and returns.
- `accumulate_indexed_products_2`: Reads a byte from a table indexed by a0, sign-extends it into d1, then loops while d1 is non-negative decrementing that index and accumulating products.
- `load_indexed_value_extend`: Builds a 16-bit index from two bytes in the table at a3 scaled by 256, loads the 16-bit value from the indexed offset of the structure in a1 into d7, extends it to a signed word, and returns.
- `load_indexed_value_return`: Builds a 16-bit index from two bytes in the table at a3 scaled by 256, loads the 16-bit value from the indexed offset of the structure in a1 into d7, and returns.
- `copy_string_via_index_zeroed`: Builds a 16-bit index from two bytes in the table at a3 scaled by 256, copies bytes from the indexed address into the buffer at a0 until a zero byte is stored, and returns.
- `call_weighted_sum`: Builds a 16-bit index from two bytes read sequentially from the table pointed to by a3, then calls the weighted-sum routine with that index and returns the resulting byte from the structure in a1.
- `call_weighted_product`: Builds a 16-bit index from two bytes read sequentially from the table pointed to by a3, then calls the weighted-product routine with that index and returns the resulting word from the structure in a1.
- `combine_and_copy_string`: Combines two bytes from a table into a 16-bit value, adds it to a global variable, and copies the resulting null-terminated string into another global variable.
- `load_byte_from_indexed_table`: Builds a word offset from two consecutive bytes in the buffer pointed to by a3, indexes a word table via a6, then copies a byte from that indexed table location into d7 and sign-extends it to a word before returning.
- `lookup_indexed_value`: Reads a 16-bit index from a table pointed to by a3, looks up a pointer in a table indexed by that value, then reads a 16-bit value from a second table using the same index and stores it in d7 before returning.
- `calculate_weighted_sum`: Calculates a sum by multiplying a value from a pointer-indexed table with a value from another pointer-indexed table, repeating until a termination condition in a byte is met.
- `calculate_weighted_sum_2`: Performs a loop that accumulates a weighted sum or a product-based calculation into a return register using a pointer and a counter.
- `accumulate_weighted_product_2`: Multiplies values from two pointer-indexed tables and accumulates the result into a register until a counter reaches zero.
- `accumulate_weighted_product`: Multiplies a value from a pointer-indexed table with a value from another pointer-indexed table and accumulates the result until a counter reaches negative one.
- `accumulate_indexed_products`: Reads a byte from a table indexed by a0, sign-extends it into d1, then loops while d1 is non-negative decrementing that index and accumulating the product of the pointed byte and d7 into d0 before returning.
- `accumulate_pointer_products`: Loops while d1 is not negative, decrementing a pointer in a0 and incrementing a pointer in a4, accumulating the product of the two pointed words into d0, and returns the accumulated sum.
- `copy_register_value`: Calls another routine and then performs a single instruction that copies the contents of one register to another.
- `copy_register_d7_to_d6_3`: copies the contents of one data register to another.
- `copy_d7_to_d6`: Copies the contents of one data register to another.
- `call_tiny_move_invert`: Calls the tiny-move routine, then inverts bit 6 of d7 and complements the whole register before returning.
- `compare_registers_not_equal`: Calls another routine and then checks if two registers contain different values.
- `compare_registers_equal`: Calls another routine and then checks if two registers contain the same value.
- `compare_registers_and_branch`: compares two 16-bit values stored in registers and returns if the value in one register is less than the value in another.
- `compare_registers_and_branch_2`: Copies a value from one register to another and then checks if the new value is less than the original value, exiting if the condition is met.
- `compare_registers_and_branch_3`: compares two or more words and branches if the value in one register is not greater than the value in another.
- `compare_d6_d7`: compares a 16-bit value in a register against another 16-bit value and returns if the orignal is not greater.
- `copy_register_d7_to_d6_4`: Copies the contents of one data register to another data register.
- `call_indirect_function_4`: Loads a function pointer from a global variable and an indexed table, then executes the pointed-to routine.
- `copy_register_data_2`: Copies the contents of one data register to another using a specialized move instruction.
- `call_tiny_move_divide`: Calls the tiny-move routine, sign-extends d6 to 32 bits, divides it by d7, and returns the quotient in d7.
- `call_tiny_move_multiply`: Calls the tiny-move routine, multiplies d7 by d6, and returns the product in d7.
- `tiny_neg_rts`: Small routine of 2 instructions that negates d7, then returns.
- `ensure_non_negative`: Tests the sign of d7 and negates it when the value is negative, otherwise it returns.
- `store_linear_hash`: Computes a linear hash and stores the resulting 16-bit value into a global variable.
- `compute_linear_hash`: multiplies a 16-bit value by a constant and adds a constant.
- `check_register_sign`: Checks if a register is zero or negative and returns if so.
- `tiny_clr_rts_3`: Small routine of 2 instructions that clears d7, then returns.
- `stub_return_only_2`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 2 instructions in the listing and performs no work.
- `tiny_not_rts`: Small routine of 2 instructions that inverts d7, then returns.
- `reset_table_on_condition`: Iterates through a table to compare values and resets a portion of a table or a global variable to zero if a certain condition is met.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer.
- `call_swap_global_values`: Call stub: calls swap_global_values, then returns to its own caller.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `branch_on_global_zero`: Checks if a global variable is zero and calls another routine if it is.
- `execute_self_recursively`: The routine calls itself recursively.
- `update_button_mask_2`: Modifies a register value based on specific bits in a global variable to target hardware buttons.
- `branch_on_global_non_zero`: Checks if a global variable is non-zero and calls another routine if it is.
- `clear_d0_and_return`: clears a register and returns.
- `call_linear_hash`: Loads a word from the pointer in a4, rotates it right by six bits, calls the linear-hash routine, and returns the product of the hash and the rotated value.
- `traverse_list_by_value`: Checks a global variable and either returns or traverses a table using a pointer-based chain if certain value conditions are met.
- `call_other_routine`: Executes another routine without modifying any registers or returning a specific value.
- `store_zero_to_global`: Writes a zero byte to a global variable at an offset determined by a register.
- `copy_string_from_offset`: Finds the end of the null-terminated string at the global pointer, then copies the bytes from offset d7 of that string into the same global buffer, stopping at the null terminator and returning to the start pointer if the source index would go past it.
- `copy_string_with_limit`: Copies up to seven bytes from the source string at offset d7 into the global buffer, stopping early if a null terminator is reached, and appends a null terminator when the byte limit is exhausted.
- `measure_string_length`: Walks through the null-terminated string at the global pointer by incrementing a pointer until the terminator is found, then stores the string length in decimal bytes into register d7 and returns.
- `scan_ascii_number_string`: Clears two registers, loads a pointer from a global variable, and when the pointed string is non-empty scans its bytes, skipping past a leading minus sign and digits until a terminator or out-of-range byte is found.
- `parse_ascii_digit`: Increments a0 and reads a byte from it, then returns when that byte is zero or outside the printable ASCII range, otherwise converting the digit character into its numeric value by subtracting forty-eight and combining it into the low nibble of d7.
- `convert_byte_to_upper`: Converts the byte in d0 to upper case, then returns when it is not a letter between A and F, otherwise subtracts fifty-five from it and falls through to an earlier routine.
- `skip_if_d7_zero`: Tests the low byte of register d7 and returns immediately when that byte is zero, otherwise falling through to the next routine.
- `tiny_neg_rts_2`: Small routine of 2 instructions that negates d7, then returns.
- `load_first_string_byte`: Clears register d7, loads the byte at the global string pointer into it, and returns.
- `convert_d7_to_string`: Moves the value of register d7 into register d0, calls the integer-to-string conversion routine, and returns.
- `store_byte_and_terminate`: Stores the low byte of register d7 at the global string pointer, advances the pointer, and writes a null terminator after it before returning.
- `call_indirect_and_check`: Loads a long from a fixed address into a0, calls a routine through that address, and returns immediately when the result is zero, otherwise clearing d7 and returning.
- `set_d7_to_max`: Sets register d7 to the value 65535 and returns.
- `dispatch_by_character`: Compares the byte in d7 against the lowercase and uppercase letters a through d and the question mark, branching to a matching handler for each, otherwise setting d7 to negative one and returning.
- `call_with_zero_flag`: Sets d0 to zero, calls a routine, and returns immediately when the result is negative, otherwise falling through to an earlier routine.
- `call_with_one_flag`: Sets d0 to one, calls a routine, and returns immediately when the result is negative, otherwise falling through to an earlier routine.
- `call_with_two_flag`: Sets d0 to two, calls a routine, and returns immediately when the result is negative, otherwise falling through to an earlier routine.
- `call_with_three_flag`: Sets d0 to three, calls a routine, and returns immediately when the result is negative, otherwise falling through to an earlier routine.
- `call_add_offset_return`: Calls a routine with a value in d0, adds 65 to the returned result, stores it in d7, and returns.
- `load_byte_from_global`: Clears register d7, loads the byte at the global variable at offset 39683 into it, and returns.
- `call_move_and_copy`: Calls the small move routine, copies register d0 into register d7, and returns.
- `return_zero_to_d7`: Calls a routine that returns the constant 0 in d0, stores that value in d7, and returns.
- `compare_null_terminated_strings`: Initializes two global pointers, compares the strings they reference byte by byte and branches away when they differ, otherwise checking the preceding byte and returning when it is non-zero.
- `test_byte_before_pointer`: Tests the byte before the pointer in a0 and returns immediately if it is zero, otherwise it branches away.
- `subtract_bytes_via_pointers`: Reads a byte from the address one before the pointer in a0 and subtracts the byte one before the pointer in a1, leaving the result in d0.
- `reverse_string_in_place`: Finds the null terminator of a string pointed to by d0, and if that string has an odd length it stores a zero byte at the address decremented by the pointer in a4, then copies the string backwards into that destination before restoring the registers and returning.
- `copy_string_to_buffer`: copies a null-terminated string from a source pointer to a destination pointer.
- `trim_odd_string_buffer`: Finds the length of a null-terminated string in a global variable, then if that length is odd it removes the last byte from a stack buffer and shifts the remaining bytes down by one position to close the gap.
- `copy_string_to_pointer`: Pushes d0 onto the stack, loads a destination pointer, then copies bytes from the address incremented by a4 into the destination until a null byte is written, restoring d0 from the stack and returning.
- `call_helper_and_return`: Calls a helper routine and then returns without using its result.
- `swap_pointers_and_call`: Swaps two stored longword pointers in global variables, calls a helper routine, and then returns.
- `clear_when_strings_equal`: Calls two routines, and if the comparison result is equal clears register d7, otherwise it returns.
- `set_d7_to_negative_one`: Sets d7 to -1 and returns.
- `clear_when_strings_differ`: Calls two routines, and if the comparison result is not equal clears register d7, otherwise it returns.
- `set_d7_to_negative_one_2`: Sets d7 to -1 and returns.
- `call_then_check_flag`: Calls two routines in sequence, then branches to a later instruction when the status flag indicates a less-than-or-equal result, otherwise falling through.
- `store_max_in_d7_slot`: Stores the value 65535 into the 16-bit value at the address in d7, then returns.
- `call_then_check_flag_ge`: Invokes two routines in turn, then branches to a later instruction when the status flag indicates a greater-than-or-equal result, otherwise continuing.
- `store_fifteen_in_d7`: Stores the value 65535 into the 16-bit value at the address in d7, then returns.
- `call_then_check_flag_lt`: Runs two routines in sequence, then branches to a later instruction when the status flag indicates a less-than result, otherwise proceeding.
- `store_max_in_d7`: Stores the value 65535 into the 16-bit value in the d7 register and then returns.
- `call_then_check_flag_gt`: Executes two routines in turn, then branches to a later instruction when the status flag indicates a greater-than result, otherwise continuing.
- `set_d7_to_max_2`: It stores the value 65535 into the 16-bit register d7, then returns.
- `swap_global_string_buffers`: It calls swap_global_values, then scans a null-terminated string starting at one global pointer until it finds the terminator.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d6 to d7, then copies (a4)+ to d6, then returns.
- `execute_infinite_indirect_call`: Performs an infinite loop that calls another routine via an indirect pointer.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_offset`: stores a byte from a register into a memory location calculated from a pointer and an offset.
- `store_word_at_offset`: stores a 16-bit value from a register into a memory location calculated from a pointer and anoffset.
- `copy_string_to_destination`: Copies a null-terminated string from a global source to a destination calculated using a base address and an offset.
- `store_byte_from_pointer`: Takes a byte from a pointer-indexed table and stores it into a global variable at an offset or index.
- `store_pointer_destination`: Stores a value from a source pointer into a destination address calculated using a register and the return value of another routine.
- `copy_indexed_byte_sequence_2`: Loads a byte from a table into a 16-bit value, sign-extends another byte from the same table, adds it to a value from a global variable, and copies bytes from a source table to a destination table until a zero byte is written.
- `write_byte_at_offset`: Writes a single byte to a destination calculated by adding an offset from a pointer to a base address.
- `write_word_at_offset`: Writes a two-byte word to a destination calculated by adding an offset from a pointer to a base address.
- `copy_string_from_global`: copies a null-terminated string from a global memory location to a destination buffer calculated from a base address and an offset.
- `store_byte_weighted_index`: It calls calculate_weighted_sum to obtain an index, reads a byte from the source pointed to by a3, and stores that byte into the byte at the computed offset within the structure referenced by a6.
- `store_word_weighted_index`: It calls accumulate_weighted_product_2 to obtain an index, reads a 16-bit value from the source pointed to by a3, and stores that word into the 16-bit location at the computed offset within the structure referenced by a6.
- `accumulate_indexed_products_3`: Reads a byte from a table indexed by a0, sign-extends it into d1, then loops while d1 is non-negative decrementing that index and accumulating the products, and returns.
- `store_byte_at_offset_2`: It combines two consecutive bytes from the source pointed to by a3 into a 16-bit offset, then stores the byte in d7 into the memory location found by adding that offset to the global pointer.
- `store_word_at_global_offset`: It combines two consecutive bytes from the source pointed to by a3 into a 16-bit offset, then stores the 16-bit value in d7 into the memory location found by adding that offset to the global pointer.
- `copy_string_to_offset`: It combines two consecutive bytes from the source pointed to by a3 into a 16-bit offset to form a destination, then copies the null-terminated string from the global source into that destination, byte by byte, stopping after writing the terminator.
- `store_byte_via_weighted_index`: Loads a byte from the table pointed to by a3, shifts it left by 8 bits into d0, then reads another byte from the table and passes it to calculate_weighted_sum.
- `store_weighted_byte_pair`: Loads two bytes from the table pointed to by a3 into d0, shifts the first left by 8 bits, then stores the 16-bit value from the word at a4 into the location given by a global variable plus that shifted value, and returns.
- `copy_indexed_byte_sequence_3`: Doubles a byte read from a table in a3 into a 16-bit value, adds it to a global variable, indexes a second table by that sum, and copies the resulting byte sequence to the address in a0 until a zero byte is stored.
- `store_byte_indirect`: It reads a 16-bit offset from the source pointed to by a3, looks up a pointer in a table indexed by that offset, then reads a second offset from the next two source bytes and stores the byte in d7 into the location addressed by that pointer plus the second offset.
- `store_byte_via_indexed_pointer`: Reads a 16-bit offset from a table via a3, looks up a pointer in a table indexed by that offset at a6, then stores the byte in d7 at the computed destination and returns.
- `invoke_indirect_subroutine`: The routine executes an infinite loop that branches to another routine using a pointer stored in memory.
- `add_byte_to_destination`: Adds a byte to an existing byte at a destination calculated using a base address and an offset.
- `add_word_to_destination`: Adds a word to an existing word at a destination calculated using a base address and an offset.
- `copy_string_from_table_index`: Copies a NUL-terminated string from a global variable to a destination buffer whose base address is taken from a table indexed by a 128-bit value derived from two bytes read from the a3 buffer, then returns.
- `add_weighted_sum_to_byte`: Calls the weighted-sum routine, then adds the word loaded from a4 to the byte at the destination computed from a6 and the returned offset, and returns.
- `add_weighted_product_to_word`: Calls the weighted-product routine, then adds the word loaded from a4 to the 16-bit value at the destination computed from a6 and the returned offset, and returns.
- `accumulate_indexed_products_4`: Reads a byte from a table indexed by a0, sign-extends it into d1, then loops while d1 is non-negative, decrementing that index and accumulating the products.
- `add_byte_to_destination_2`: Clears d0, reads a byte offset from the table at a3, adds the byte in d7 to the byte at the destination computed from a6 and that offset, and returns.
- `increment_memory_at_offset`: adds a 16-bit value to a 16-bit value stored at a memory location determined by a base address plus an offset.
- `copy_string_to_destination_reverse`: Finds the end of a destination buffer and copies a null-terminated string from a global source into it.
- `add_weighted_sum_to_byte_offset`: Clears d0, reads a byte offset from the table at a3, calls the weighted-sum routine, adds the word loaded from a4 to the byte at the destination computed from a6 and the returned offset, and returns.
- `add_weighted_product_to_word_offset`: Clears d0, reads a byte offset from the table at a3, calls the weighted-product routine, adds the word loaded from a4 to the 16-bit value at the destination computed from a6 and the returned offset, and returns.
- `accumulate_indexed_products_5`: Reads a byte from a table indexed by a0, sign-extends it into d1, then loops while d1 is non-negative, decrementing that index and accumulating the product of each byte with its index into d0.
- `add_byte_to_global_offset`: Reads a byte from a table through a3, shifts it left by 8, reads the next byte from the same table, then adds the byte in d7 to the byte at a global variable indexed by the shifted value and stores the result back.
- `add_offset_to_global_word`: Reads a byte from the table pointed to by a3, shifts it left by 128, then reads the next byte and adds the value in d7 to the 16-bit word at that offset in a global variable, storing the result back.
- `copy_string_at_offset`: Reads a 16-bit offset from a table via a3, adds it to a string base pointer, then copies the NUL-terminated string at that offset into another string buffer starting at a global variable, copying one byte at a time until the NUL terminator is reached.
- `add_weighted_sum_byte`: Builds a 16-bit index from two bytes read through a3, calls a routine to compute a weighted sum, then adds that sum to a byte at a computed offset in a global variable and returns.
- `add_weighted_product_word`: Builds a 16-bit index from two bytes read through a3, calls a routine to compute a weighted product, then adds that product to a 16-bit value at a computed offset in a global variable and returns.
- `copy_indexed_string`: Reads two bytes from a table through a3, combines them into a 16-bit index, looks up a pointer in a table via that index, calls another routine to compute an offset, then copies the null-terminated string from a source table into the destination pointed to by the looked-up pointer.
- `add_byte_to_indirect`: Computes a pointer from a6 and a3, reads a byte through that pointer, then adds a byte from d7 to the byte at the computed address and returns.
- `add_word_to_indirect`: Computes a pointer from a6 and a3, reads a 16-bit value through that pointer, then adds a 16-bit value from d7 to the 16-bit value at the computed address and returns.
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
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `setup_blt_params`: calculates a destination address and configures certain hardware registers or a global variable, while waiting for a certain bit in a global variable to clear.
- `update_blt_attributes`: iteratively updates a global or a pointer-based address or a value by adding or subtracting an offset or a certain constant, while waiting for a certain bit in a global variable to clear.
- `copy_block_tiled_pattern`: Calculates a destination address based on a pointer and an offset, then copies a block of memory using a nested loop structure that increments source and destination pointers.
- `update_blt_parameters`: Calculates a new address and size for a bitboard transfer or a window and writes the result to a global variable.
- `calculate_final_address`: checks certain bits in a memory location and then updates a global variable with a value derived from other registers or a global, or or even a calculated offset.
- `calculate_blt_window_bounds`: Calculates the visible clipping or drawing boundaries by comparing certain pointer-based structures against limits and then populates a table or buffer with visibility or masking data using a loop that writes or copies values from an indexed source or table.
- `clip_blt_window_bounds`: Calculates the clipped or visible area of a bitboard transfer by comparing a source or destination rectangle against the screen or window boundaries.
- `store_status_for_matched_handler`: Clears d3, then if a3 matches one of four known handler addresses stores a value into that handler's second word and calls a helper; otherwise it returns without work.
- `setup_audio_channel_playback`: Disables interrupts and DMA, then programs one audio channel's pointer, length, period and volume registers, capping the volume at 100, records the channel in a global variable, and finally re-enables interrupts and DMA before returning.
- `update_bitmask_and_enable_dma`: Checks a condition using a global variable, clears a flag in a table, and sets a bitmask in a global variable while updating hardware registers.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set.
- `update_table_and_registers`: Decrements a value in a table and, if it reaches zero, resets several memory locations and hardware registers including audio and DMA or interrupt or even a bitmask.
- `move_word_to_register`: Copies a word from a source memory location or pointer-offset address into a hardware register.
- `execute_sequential_routines`: Calls another routine four times in succession.
- `do_nothing_6`: this routine is empty and contains no executable instructions.
- `frag_movem_jsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `reset_global_variable_4`: Sets a global variable to a constant value if a certain condition is met, otherwise calls another routine.
- `cast_global_to_unsigned`: Performs a redundant type cast or assignment of a global variable to itself.
- `scale_global_variable`: Multiplies a global variable by a constant and right-shifts the result.
- `clear_audio_volumes`: Sets several audio volume hardware registers to zero if a certain global variable is non-zero.
