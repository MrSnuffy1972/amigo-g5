# Game 406 (Ishar 2 - Messengers of Doom): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00406). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2750016 top-level bytes in 186 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 409 of 409 named (0 author symbols), 409 of 409 described (25 by rule, 378 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3388 of 3388 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 6050 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8319 of 8338 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 103075 |
| code | 31836 |
| data | 2499355 |
| padding | 45606 |
| unused | 70144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 10 | 2 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:loading | 2 | 267 | 8 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:    | 1 | 642 | 36 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:Failat | 2 | 117 | 2 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X | 10 | 251 | 6 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7011 | 353 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf: file s/startup-sequence (header 22 bytes, blocks hold 29): its blocks are judged as raw blocks outside the filesystem
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IBOISHAR.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GERDEP.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT1.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAIN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESOUND.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GESORT.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PRESENT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MONTAGNE.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PARAM.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GEANT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:THEEND1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EATT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:NAIN4.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EDPLT.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SCOMB.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:LOSTGAME.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:GUARD4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PIAF4.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:RHINO4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SOURIS.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:DRUIDE4.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGED.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BOB4.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:SMONTAGN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:OBJET.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:MESSAGEE.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ODWOMAN5.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:IO.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:GOLEM1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WITCH5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SDONJONS.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP11.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FLY1.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PP12.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPSAC.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:VILLAGE1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJCOL.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PUIT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJTRAP.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MONSTER5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BOAT.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LEZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:LIONMAN5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ELMTFEU5.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJFONT.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJDEC.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJ2.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SKELETT5.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:KROG.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ELMTAIR5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:AIGLE.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PCAVE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MEDUSA5.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MOMY5.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FDJCAT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SORTMOR5.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SILE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ORK5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WIZARD5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DWIZARD5.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:TEXTINE.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PLAINE1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DRAGON5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJ1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:CHAOSGU5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:SORCIER5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:WIZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:POURS1.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:ANIMALS.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:FORET1.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:DJCAT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:FARBO.CO: high-entropy data file (7.73 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VMAIRIE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SVILLE.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VHOUSE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:HOMOIS2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:FVILLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SPAROLES.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:DRUIDE2.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VHOUSE2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:RELICMAN.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3BAN.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VBIBLI.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VARCHE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ARBO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:LUTIN3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:EWOK2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VARCADE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPONT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VPASSAGE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VILLE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VNIGHT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP2I.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TROUBA3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:DWOMAN2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SCITE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3MN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:GUARD3.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC1.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC2.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VDEC3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:GUARD3HI.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VJAIL.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:ENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:PP3TB.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VTEMPLE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:VOLEUR3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- runtime segment an address: 3140 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf: blocks 838-838 (bytes 429056-429568) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.00)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 6050 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

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
- `tiny_ori`: Small routine of 1 instruction that merges 6 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 21 into (a5)+.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 40 into d1, then merges 24 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a fixed number of longwords from a memory location or pointer or a table to a stack-based or global memory location.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:T.X

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements an uninitialized local variable until it reaches a specific value and then returns.
- `expand_and_copy_memory`: Performs a complex loop that checks certain bit patterns and copies or expands data blocks while checking a global variable.
- `copy_memory_block`: copies a block of memory from a source pointer to a destination pointer using a decrementing counter.
- `shift_and_copy_block`: This routine implements a bit-shifting or bit-replication-style block copy, where bytes or bits are propagated or shifted using a sequence of left-shift operations and conditional checks.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX](Disk 3 of 3).adf:bootblock

- `initialize_boot_resources`: Calls a function from a library and, if it returns a pointer, sets three dwords at that location to a maximum value before calling another function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_values`: Stores the contents of two even-numbered registers into a global variable or a table.
- `empty_routine`: this routine is empty and performs no operations before returning.
- `traverse_and_scale_pointer`: Decrements a counter and updates a pointer by a scaled value in a loop until the counter reaches a certain value.
- `return_registers`: Returns the contents of two even-numbered registers as a single joined value.
- `copy_memory_block_6`: Copies a block of memory from a starting address to a destination address.
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
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
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
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `do_nothing_stub_3`: The routine is empty and returns immediately without performing any operations or accessing any data.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: calls another routine and returns the concatenated contents of two registers.
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
- `execute_multiple_functions`: Calls a function pointer, then calls that same function again with a register, then calls a different function pointer, and finally assigns a return value to a global variable.
- `execute_conditional_function_2`: Checks a global variable and calls a function pointer or certain other functions depending on the value or result of a function call.
- `get_library_result`: calls a function via a pointer and if the result is non-zero, calls two other functions and stores a result in a global variable.
- `tiny_andi_subq_ori`: Small routine of 3 instructions that masks d0 with 79, then subtracts 1 from d0, then merges 0 into d0.
- `swap_global_parameters`: Swaps a set of global variables and potentially updates others if a certain condition is met.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters if a certain condition is met.
- `update_buffer_indices_recursively`: This routine recursively updates a pointer-based table and adjusts buffer start and end indices based on a certain size or offset.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table.
- `initialize_global_values`: sets several consecutive global memory locations to specific constant values.
- `initialize_blit_parameters`: Sets several consecutive global bytes to a fixed block of values and calls a routine to configure a destination address and hardware registers.
- `apply_memory_offsets`: Calculates and stores a series of offset pointers and initializes several global bytes.
- `apply_coordinate_offsets`: Calculates a difference between two points and applies that difference to a list of coordinate values while updating certain global variables.
- `execute_indirect_call`: calculates a target address using a base register and an offset from a table, then jumps to that address.
- `initialize_memory_offsets`: calculates and stores a series of offset pointers and initializes several global bytes.
- `store_pointer_to_table`: reads two bytes from a pointer and stores a pointer to a later offset of that same memory block into another memory location.
- `read_short_at_pointer`: returns a short integer read from a memory address held in a register.
- `read_three_bytes`: reads three consecutive bytes from a pointer and returns them as a single packed value.
- `update_memory_structure_3`: Updates a memory structure or a global variable with a pointer and a constant value based on certain conditional checks.
- `update_memory_structure_2`: Updates a memory structure or a global variable with a pointer and a constant byte value based on certain conditional checks.
- `update_memory_structure`: copies a word, then writes a pointer and a constant byte value into a memory structure at a calculated offset.
- `update_pointer_target`: Checks if a global variable is zero, then updates that variable and a pointer-based memory location with a new value.
- `return_empty`: The routine is empty and returns.
- `validate_and_clear_node_pointer`: Checks if a global variable is null and compares a value against a register; if a certain condition is met, it clears a short value in a memory structure using a pointer-based offset.
- `get_dereferenced_value`: Stores a pointer in a global variable, calls another routine, and returns a value from that pointer or a returned short.
- `load_global_byte`: Stores a pointer in a global variable, calls another routine, and returns a byte loaded from a global variable or a returned word.
- `read_global_byte`: Returns a byte loaded from a global variable.
- `conditionally_return_pointer_target`: If a certain flag is zero, it returns a value from a pointer, otherwise it returns a register value.
- `conditionally_return_three_bytes`: If a certain flag is zero, it constructs a 32-bit value from a pointer and returns it.
- `conditionally_return_pointer_byte`: If a certain flag is non-zero, it returns a byte loaded from a pointer, otherwise it returns a register value.
- `conditionally_return_pointer_target_2`: If a certain flag is non-zero, it returns a value from a pointer, otherwise it returns a register value.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a3, then returns.
- `copy_register_d7_to_d6`: This routine copies the contents of one register to another.
- `execute_eight_instructions`: This routine executes eight instructions and is called by four other routines.
- `execute_indirect_call_2`: This routine calculates an address using a pointer and an offset and calls the resulting address.
- `copy_register_d7_to_d6_2`: This routine performs a single instruction that copies the contents of one register to another.
- `undefined_routine`: This routine contains eight instructions and is called by three other routines.
- `copy_register_value_2`: Performs a single instruction to copy the contents of one register to another and calls another routine.
- `set_global_from_function`: Calls a routine and updates a global variable with either a returned value or a constant if a certain condition is met.
- `process_string_characters`: Modifies global or buffer-based counters or pointers based on whether a character matches newline, carriage return, or space, and calls another routine if certain conditions are met.
- `iterate_string_characters`: Iterates through a null-terminated string, calling a routine to process or convert each character or integer-string representation until a null terminator is encountered.
- `convert_integer_to_string`: Converts a signed integer or a value from a register into a string or a sequence of characters, including handling a negative sign, and stores the result in a buffer.
- `store_word_via_indexed_pointer`: stores a word from a pointer-referenced location into a destination address calculated using a base pointer and an index multiplied by two.
- `execute_eight_instructions_2`: This routine executes a fixed number of eight instructions and is called by several other routines.
- `execute_table_operations`: Executes a fixed number of instructions, performs a binary search through a table of words, and resets a string buffer or linked list structure.
- `delete_memory_node`: Deletes a node from a managed memory list by shifting remaining pointers to close the gap and updating a linked list of pointers.
- `remove_memory_node`: Removes a node from a memory-managed linked list, shifts remaining node pointers to close the gap, and updates a global pointer-based list or table.
- `check_global_null`: Checks if a global variable is zero and, if not, calls another routine.
- `traverse_string_buffer`: Traverses a string buffer and performs conditional checks and updates or calls other routines if certain string termination or data conditions are met.
- `find_minimum_from_pointer`: If a certain value matches a value in a structure, it searches through a table to find the minimum integer value using a pointer obtained from another routine.
- `update_linked_list_node_2`: Modifies a bit in a byte, copies a sequence of bytes from one location to another, and updates several fields in a structure or list node.
- `set_global_if_zero`: Checks if a global variable is zero and sets it to one if it is.
- `reset_global_variable`: Sets a global variable to zero.
- `update_if_different`: Compares a value from a pointer to a value at a calculated offset and updates that offset if they differ.
- `update_table_values_2`: If a certain value is non-zero, this routine writes a register or a value to a memory location calculated using a global variable and an offset, then sets a high bit in a byte at a pointer.
- `modify_bit_flags`: This routine clears a specific bit and sets another bit in a byte located at an offset from a global variable, while also performing linked list insertion and removal operations.
- `set_bit_in_byte`: Sets the certain bit in a byte located at an offset determined by a pointer.
- `insert_into_linked_list`: Inserts a new value into a linked list or updates a pointer within a structure if the list is empty or certain conditions are met.
- `remove_from_linked_list`: Traverses a linked list to find a specific value and updates a pointer or replaces a node if a match is found.
- `calculate_cross_products`: Performs a series of signed byte multiplications and subtractions, storing the results in a structure.
- `set_byte_at_offset`: Sets a byte at a specific offset relative to a register value to one.
- `process_string_data_stream`: Parses a data buffer to update a set of global counters and pointers, while conditionally copying or modifying word-sized or byte-sized values into a destination buffer based on certain flag or value checks.
- `copy_string_with_offset_and_transform`: Copies a sequence of words from a source buffer to a destination buffer while applying a signed or unsigned transformation to the data and updating a global offset.
- `parse_string_stream_tokens`: Iterates through a data buffer to extract and apply a sequence of length-prefixed or type-prefixed commands, updating global counters and pointers.
- `expand_and_transform_string`: Copies and expands a sequence of words from a source buffer to a destination buffer, applying a signed or unsigned transformation to the data.
- `parse_string_data_stream`: Iterates through a data buffer to update or copy a sequence of word-sized or byte-sized values into a destination buffer, or modifies a global counter and pointer based on conditional flags or signed-value checks within the stream.
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
- `compute_scaled_vector_offset`: Calculates a new position and direction by applying a bit-shifted scale factor to a vector, then updates global destination coordinates.
- `get_conditional_value`: Returns a value based on a comparison of a byte at an offset or a constructed 32-bit value.
- `resolve_indirect_pointer_offset`: calculates a target address by traversing multiple levels of indirection and adding a scaled offset or a value retrieved from a pointer-based table.
- `calculate_pointer_offset`: Calculates a pointer or value by adding an offset and a value retrieved from a table or memory location.
- `toggle_global_bit`: Sets a global variable to a value loaded from a register and uses a loop to check if that value is zero.
- `traverse_linked_list_nodes`: Traverses a linked list and resets a global variable to zero if a certain condition is met or when the end of the list is reached.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `set_triple_word`: Assigns a 16-bit value to three consecutive memory locations or a table.
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
- `return_without_effect`: The routine consists of a single return instruction and performs no work, so it acts as an empty stub that update_window_dimensions can call without side effects.
- `traverse_buffer_with_bounds`: Traverses a buffer while checking a length byte and calling another routine if a certain condition is met.
- `intersect_rect_bounds`: Checks if a rectangle, defined by a pointer-based offset and certain width/height modifiers, overlaps with a global boundary.
- `check_rect_intersection_2`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary and returns a modified pointer.
- `check_rect_intersection`: Tests if a rectangle, constructed from pointer-based offsets, intersects with a global boundary, and returns a masked or modified pointer or value.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d1, then returns.
- `copy_memory_block_4`: Copies a block of memory from a source pointer to a destination pointer, decrementing a counter and updating a pointer for each word or byte transferred.
- `search_table_for_value`: Iterates through a table or list, checking if a value matches a certain constant, and updates a pointer or counter during the process.
- `return_empty_routine_2`: The routine performs no operations and immediately returns to its caller.
- `tiny_adda_rts`: Small routine of 2 instructions that adds (a3)+ to a3, then returns.
- `do_nothing_stub_4`: The routine contains no instructions and returns immediately.
- `clear_table_entry`: If a certain condition is met, writes zero to a memory location determined by an offset in a table.
- `clear_bit_zero`: Clears the least significant bit of a byte at a calculated offset from a pointer.
- `set_bit_one`: Sets the second least significant bit of a byte at a calculated offset from a pointer.
- `clear_bit_one`: Clears the second least significant bit of a byte at a calculated offset from a pointer.
- `fetch_indirect_call_target`: loads a byte from an indirect call target and stores it into a global variable.
- `copy_memory_block_5`: Copies a block of bytes from a source table to a destination address, where the block size is read from the source table.
- `set_global_to_one`: Masks a register value and then overwrites a global variable with a constant value of one.
- `update_global_pointers`: Calculates a new address based on a global variable and stores it into two global variables.
- `evaluate_state_and_sync_data`: Checks a global variable against several constant values and, depending on the result, recursively calls another routine or performs a block copy of data from a source pointer to two destination pointers.
- `return_from_subroutine_2`: the routine performs a single jump or return instruction to a calling routine.
- `copy_data_with_conditional_offset`: Copies a sequence of words from a source pointer to two destination pointers, followed by a conditional copy of a block of bytes from a global variable.
- `reset_global_variable_2`: Sets a global variable to zero through a series of assignments.
- `update_table_with_register`: If a global variable does not match certain allowed values, stores a register value into a table at an offset calculated from another register.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `check_nibble_difference`: Calculates the difference between two lower nibbles and returns if the result is negative or outside a certain range.
- `reset_and_copy_global_data`: Resets a global variable and performs conditional block copies of data between a source pointer, a global variable, and a destination pointer or another global variable.
- `reconstruct_nibble_data`: Iterates through a buffer to detect or modify nibble-based data encodings.
- `reset_and_copy_data`: Checks if a global variable matches certain values, resets it, and then copies a block of memory from a source pointer to a destination pointer, followed by a conditional copy of a small number of words.
- `verify_nibble_sum_threshold`: tests if the sum of two lower nibbles exceeds a constant value and returns the result.
- `compare_nibble_range`: Checks if the difference between two lower nibbles falls within a specific range or if the result is negative.
- `update_object_position`: Calculates and applies a new position or offset to a structure in memory, including conditional logic for handling directional or certain type-specific movement adjustments.
- `update_window_dimensions`: Updates a set of window or clipping-area coordinates by adding or subtracting offsets or certain values retrieved from a table.
- `reset_global_variable_3`: Checks if a certain global variable and another global variable meet certain conditions, then clears that global variable.
- `clear_global_variable`: Performs several calls to other routines and then sets a global variable to zero.
- `copy_register_data`: Copies the contents of one register to another register.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer located within a table.
- `swap_global_values`: Swaps the contents of two adjacent global variables and then performs an indirect call using a pointer retrieved from a table or memory location.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine_3`: the routine performs no operations and immediately returns to the caller.
- `copy_string_data`: Copies a null-terminated string from a source address to a destination address.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 6 instructions in the listing and performs no work.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or an immediate return, effectively acting as a null operation.
- `return_immediately_2`: The routine executes a return instruction to exit the current function.
- `return_empty_routine_4`: The routine contains no executable instructions and performs no operations before returning.
- `copy_string_from_table`: Copies a null-terminated string from a location determined by an offset in a table to a destination address.
- `calculate_weighted_sum`: Calculates a sum by multiplying a value from a pointer-indexed table with a value from another pointer-indexed table, repeating until a termination condition in a byte is met.
- `calculate_weighted_sum_2`: Performs a loop that accumulates a weighted sum or a product-based calculation into a return register using a pointer and a counter.
- `accumulate_weighted_product_2`: Multiplies values from two pointer-indexed tables and accumulates the result into a register until a counter reaches zero.
- `accumulate_weighted_product`: Multiplies a value from a pointer-indexed table with a value from another pointer-indexed table and accumulates the result until a counter reaches negative one.
- `copy_register_value`: Calls another routine and then performs a single instruction that copies the contents of one register to another.
- `copy_register_d7_to_d6_3`: copies the contents of one data register to another.
- `copy_d7_to_d6`: Copies the contents of one data register to another.
- `compare_registers_not_equal`: Calls another routine and then checks if two registers contain different values.
- `compare_registers_equal`: Calls another routine and then checks if two registers contain the same value.
- `compare_registers_and_branch`: compares two 16-bit values stored in registers and returns if the value in one register is less than the value in another.
- `compare_registers_and_branch_2`: Copies a value from one register to another and then checks if the new value is less than the original value, exiting if the condition is met.
- `compare_registers_and_branch_3`: compares two or more words and branches if the value in one register is not greater than the value in another.
- `compare_d6_d7`: compares a 16-bit value in a register against another 16-bit value and returns if the orignal is not greater.
- `copy_register_d7_to_d6_4`: Copies the contents of one data register to another data register.
- `call_indirect_function_4`: Loads a function pointer from a global variable and an indexed table, then executes the pointed-to routine.
- `copy_register_data_2`: Copies the contents of one data register to another using a specialized move instruction.
- `store_linear_hash`: Computes a linear hash and stores the resulting 16-bit value into a global variable.
- `compute_linear_hash`: multiplies a 16-bit value by a constant and adds a constant.
- `check_register_sign`: Checks if a register is zero or negative and returns if so.
- `tiny_clr_rts_3`: Small routine of 2 instructions that clears d7, then returns.
- `stub_return_only_2`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 2 instructions in the listing and performs no work.
- `reset_table_on_condition`: Iterates through a table to compare values and resets a portion of a table or a global variable to zero if a certain condition is met.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer.
- `branch_on_global_zero`: Checks if a global variable is zero and calls another routine if it is.
- `execute_self_recursively`: The routine calls itself recursively.
- `update_button_mask_2`: Modifies a register value based on specific bits in a global variable to target hardware buttons.
- `branch_on_global_non_zero`: Checks if a global variable is non-zero and calls another routine if it is.
- `clear_d0_and_return`: clears a register and returns.
- `traverse_list_by_value`: Checks a global variable and either returns or traverses a table using a pointer-based chain if certain value conditions are met.
- `call_other_routine`: Executes another routine without modifying any registers or returning a specific value.
- `store_zero_to_global`: Writes a zero byte to a global variable at an offset determined by a register.
- `copy_string_to_buffer`: copies a null-terminated string from a source pointer to a destination pointer.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d6 to d7, then copies (a4)+ to d6, then returns.
- `execute_infinite_indirect_call`: Performs an infinite loop that calls another routine via an indirect pointer.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_offset`: stores a byte from a register into a memory location calculated from a pointer and an offset.
- `store_word_at_offset`: stores a 16-bit value from a register into a memory location calculated from a pointer and anoffset.
- `copy_string_to_destination`: Copies a null-terminated string from a global source to a destination calculated using a base address and an offset.
- `store_byte_from_pointer`: Takes a byte from a pointer-indexed table and stores it into a global variable at an offset or index.
- `store_pointer_destination`: Stores a value from a source pointer into a destination address calculated using a register and the return value of another routine.
- `write_byte_at_offset`: Writes a single byte to a destination calculated by adding an offset from a pointer to a base address.
- `write_word_at_offset`: Writes a two-byte word to a destination calculated by adding an offset from a pointer to a base address.
- `copy_string_from_global`: copies a null-terminated string from a global memory location to a destination buffer calculated from a base address and an offset.
- `invoke_indirect_subroutine`: The routine executes an infinite loop that branches to another routine using a pointer stored in memory.
- `add_byte_to_destination`: Adds a byte to an existing byte at a destination calculated using a base address and an offset.
- `add_word_to_destination`: Adds a word to an existing word at a destination calculated using a base address and an offset.
- `increment_memory_at_offset`: adds a 16-bit value to a 16-bit value stored at a memory location determined by a base address plus an offset.
- `copy_string_to_destination_reverse`: Finds the end of a destination buffer and copies a null-terminated string from a global source into it.
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
- `tiny_ori_4`: Small routine of 1 instruction that merges 117 into d0.
- `setup_blt_params`: calculates a destination address and configures certain hardware registers or a global variable, while waiting for a certain bit in a global variable to clear.
- `update_blt_attributes`: iteratively updates a global or a pointer-based address or a value by adding or subtracting an offset or a certain constant, while waiting for a certain bit in a global variable to clear.
- `copy_block_tiled_pattern`: Calculates a destination address based on a pointer and an offset, then copies a block of memory using a nested loop structure that increments source and destination pointers.
- `update_blt_parameters`: Calculates a new address and size for a bitboard transfer or a window and writes the result to a global variable.
- `calculate_final_address`: checks certain bits in a memory location and then updates a global variable with a value derived from other registers or a global, or or even a calculated offset.
- `calculate_blt_window_bounds`: Calculates the visible clipping or drawing boundaries by comparing certain pointer-based structures against limits and then populates a table or buffer with visibility or masking data using a loop that writes or copies values from an indexed source or table.
- `update_view_bounds`: Updates global boundary or clipping coordinates by comparing a pointer-based structure against a set of minimum and maximum limit constants.
- `clip_blt_window_bounds`: Calculates the clipped or visible area of a bitboard transfer by comparing a source or destination rectangle against the screen or window boundaries.
- `update_bitmask_and_enable_dma`: Checks a condition using a global variable, clears a flag in a table, and sets a bitmask in a global variable while updating hardware registers.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set.
- `update_table_and_registers`: Decrements a value in a table and, if it reaches zero, resets several memory locations and hardware registers including audio and DMA or interrupt or even a bitmask.
- `move_word_to_register`: Copies a word from a source memory location or pointer-offset address into a hardware register.
- `execute_sequential_routines`: Calls another routine four times in succession.
- `do_nothing_6`: this routine is empty and contains no executable instructions.
- `reset_global_variable_4`: Sets a global variable to a constant value if a certain condition is met, otherwise calls another routine.
- `cast_global_to_unsigned`: Performs a redundant type cast or assignment of a global variable to itself.
- `scale_global_variable`: Multiplies a global variable by a constant and right-shifts the result.
- `clear_audio_volumes`: Sets several audio volume hardware registers to zero if a certain global variable is non-zero.
