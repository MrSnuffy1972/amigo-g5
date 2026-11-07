# Game 1933 (ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01933). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2750016 top-level bytes in 186 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 409 of 409 named (0 author symbols), 409 of 409 described (25 by rule, 48 by the local model) |
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
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:loading | 2 | 267 | 8 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:    | 1 | 642 | 36 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:Failat | 2 | 117 | 2 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:T.X | 10 | 251 | 6 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7011 | 353 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf: file s/startup-sequence (header 22 bytes, blocks hold 29): its blocks are judged as raw blocks outside the filesystem
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:IBOISHAR.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:GERDEP.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PRESENT1.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MAIN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PRESENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PRESOUND.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MESSAGE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:GESORT.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PRESENT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MONTAGNE.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PARAM.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:GEANT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:THEEND1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:EATT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:NAIN4.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:EDPLT.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:SCOMB.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:LOSTGAME.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:GUARD4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PIAF4.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:RHINO4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:SOURIS.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:DRUIDE4.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MESSAGED.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:BOB4.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:SMONTAGN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:OBJET.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:MESSAGEE.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:ODWOMAN5.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:IO.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:GOLEM1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:WITCH5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SDONJONS.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PP11.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:FLY1.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PP12.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PPSAC.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:VILLAGE1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJCOL.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PUIT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJTRAP.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:MONSTER5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:BOAT.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:LEZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:LIONMAN5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:ELMTFEU5.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJFONT.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJDEC.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJ2.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SKELETT5.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:KROG.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:ELMTAIR5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:AIGLE.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PCAVE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:MEDUSA5.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:MOMY5.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:FDJCAT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SORTMOR5.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SILE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:ORK5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:WIZARD5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DWIZARD5.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:TEXTINE.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PLAINE1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DRAGON5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJ1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:CHAOSGU5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:SORCIER5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:WIZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:POURS1.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:ANIMALS.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:FORET1.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:DJCAT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:FARBO.CO: high-entropy data file (7.73 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VMAIRIE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:SVILLE.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VHOUSE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:HOMOIS2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:FVILLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:SPAROLES.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:DRUIDE2.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VHOUSE2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:RELICMAN.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VPORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PP3BAN.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VBIBLI.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VARCHE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:ARBO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:LUTIN3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:EWOK2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VARCADE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VPONT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VPASSAGE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VILLE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VNIGHT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PP2I.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:TROUBA3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:DWOMAN2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:SCITE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PP3MN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:GUARD3.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VDEC1.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VDEC2.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VDEC3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:GUARD3HI.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VJAIL.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:ENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:PP3TB.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VTEMPLE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:VOLEUR3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- runtime segment an address: 3140 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf: blocks 838-838 (bytes 429056-429568) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.00)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:T.X: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 6050 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `xor_decode_byte_sequence`: It XORs a run of consecutive bytes in memory with a value that starts at 127 and decreases by two each step, processing one byte per iteration.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d5.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:loading

- `process_hunk_stream`: Parses a multi-word hunk-format stream, translating 1004 and 1010 markers and copying relocated data blocks into a target buffer while tracking offsets and sizes.
- `decrement_word_to_zero`: It decrements a word held in a register until that value reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_bit_stream_carry`: It shifts a multi-word bit buffer held in memory one bit at a time for a count held in a register, refilling the buffer from a memory stream when it empties.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_stream_with_carry`: Loops for a count held in a data word, shifting a multi-word bit buffer one bit at a time and refilling from memory when the buffer runs empty.
- `trap_and_write_memory`: Traps into the operating system, then writes a stack byte into two different memory locations and decrements a global word.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:   

- `copy_code_to_allocated_block`: Calls a memory allocation function through a global base pointer, copies a long run of this routine's own code word by word into the new block, and then calls into the copy.
- `call_four_routines_in_sequence`: Calls four routines one after another and returns zero.
- `xor_table_with_mask`: Copies a block of words from a global data area into a destination buffer, XORing each word with a fixed constant as it goes.
- `service_keyboard_serial_counter`: Calls two library vector routines through a global base, then sets a state byte and advances a counter.
- `build_trap_block_and_call`: Writes a small set of words into memory, including an illegal-instruction opcode and pointers back into that area, then calls a library vector and stores the result.
- `compute_kick_data_checksum`: Stores a pointer in a global structure, calls the exec library checksum function for the kickstart data through the library base, and saves the returned value in that structure.
- `set_library_function_pointer`: Reads a function pointer from a global table, stores it in a range-checked location, and then calls a library function through the library base.
- `return_empty_subroutine_5`: The routine is empty and performs no operations before returning.
- `verify_code_checksum_or_wipe`: Sums a short word structure and stores its negated total in its last word, then sums a long-word run of the routine's own code and compares the total with a fixed value.
- `return_from_button_stub`: Despite the listed reads of the fire-button and right-button registers, the decompiled body is only a return, so it has no visible effect.
- `jump_vector_entry_2`: A jump-table entry that passes control straight on to a fixed address elsewhere in memory via a single absolute jump.
- `access_controller_port_registers`: Accesses the CIA port registers that carry fire buttons and disk status, along with a custom chip register, and calls a library function at a fixed offset.
- `xor_decode_block`: XOR-decodes a block of 128 long words using a running key that starts at 127 and decreases by two each step, then stores four values into a structure at fixed offsets and makes a library call.
- `scan_loaded_hunk_table`: Calls library and vector routines to read a block of loaded data, expands its bits into a byte table, and collects qualifying entries.
- `unpack_bitmask_into_flags`: Expands a run of 32-bit words into individual flag bytes, one per bit, walking forward until the end pointer is reached.
- `collect_matching_table_entries`: Scans a table of 22-byte records from a computed starting point, collecting up to seven indexes whose flag byte is set and ending the list with a zero word.
- `collect_flagged_entries_then_walk`: Collects up to seven flagged indexes from a table of 22-byte records starting at a fixed offset, then walks a linked list of nodes and compares a type field against two specific values.
- `poll_structure_flag_until_clear`: Reads a flag word inside a structure reached through a pointer and, while that flag is non-zero, keeps calling other routines.
- `check_loaded_block_size`: Reads a size-like word from a structure and, when it is below a fixed limit, sets up a pointer into that structure before calling further routines.
- `clear_block_and_sample_beam`: Stores a value into a structure, zeroes a block of 128 long words, sets a count word, and makes a library call.
- `shift_and_store_register`: Shifts the data register value left by nine bits and stores the result in a field of a structure reached through an address register.
- `store_call_result_in_field`: Clears a word-sized field in a structure, calls another routine, and stores that routine's return value in the same structure through a second pointer.
- `accumulate_pointer_values`: Walks 128 consecutive long words from a pointed-to block, subtracting each from an accumulator, and returns the total.
- `compare_string_with_case_insensitivity`: Compares a stored string inside a structure with a second buffer, converting lowercase letters to uppercase as it goes.
- `calculate_absolute_difference`: Subtracts a fixed constant from the byte in the data register and returns the absolute value of the result.
- `clear_bits_for_null_bytes`: For each word in a bitmask range, clears the bit positions whose matching byte in a string buffer is zero.
- `find_string_end_set_pointer`: Finds the null terminator of a string in a shared text buffer from the current index, advances the index past it, and stores the string's address in a structure field.
- `open_library_and_call_function`: Opens an exec library through a library base call, then calls a function at a fixed offset in the opened library.
- `write_digit_character`: Calls three routines, takes the low byte of the final result, and writes it to a buffer as an ASCII digit by adding the character code for zero.
- `convert_byte_to_ascii`: Adds the character code for zero to the low byte of the data register and stores the result at the address held in an address register.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `zero_memory_endless_loop`: Writes zero long words from a fixed memory address onward in an endless loop, so the routine never returns.
- `tiny_ori`: Small routine of 1 instruction that merges 6 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 21 into (a5)+.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 40 into d1, then merges 24 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_pointer`: Copies sixteen long words from a structure at a fixed offset to a destination when that destination pointer is non-null, then calls a routine through a function pointer.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:T.X

- `run_startup_and_swap_word`: Runs a sequence of calls, including a loop that repeatedly invokes a library vector, then frees memory through exec FreeMem.
- `decrement_until_minus_one`: Decrements a local counter word until it reaches minus one, which makes it a simple busy wait.
- `run_passes_then_clear_cache`: Runs six passes of nested loops that call helper routines and add values into a memory block.
- `copy_backwards_with_bit_reader`: Copies bytes from a source buffer to a destination in reverse order, shifting a control byte to decide each step.
- `copy_backwards_bytes_with_counter`: Copies bytes backwards from a source to a destination under the control of a shifting byte and a counter, using two nested loops.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a](Disk 3 of 3).adf:bootblock

- `open_disk_resource_for_boot`: Opens the disk resource through a library call and, if that succeeds, sets three fields of the returned structure to minus one before calling a second library function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_two_call_results`: Calls two routines and stores each return value in its own global word.
- `return_empty_subroutine`: The decompiled body is a bare return, so the routine does nothing when entered.
- `traverse_and_scale_pointer`: Walks a pointer chain for a count held in a data word: each step reads the value at the current pointer, multiplies it by four, and uses that as the next pointer.
- `return_concatenated_registers`: Returns two incoming register values concatenated into a single 64-bit result without modification.
- `copy_code_to_low_memory`: Copies a run of long words from this routine's own code area into a fixed low-memory destination, one at a time, until it reaches a global boundary.
- `patch_newline_on_byte_pattern`: Checks two byte patterns around a pointer and writes a line-feed value at a fixed offset when one of them matches.
- `check_pattern_and_write_newline`: Checks a byte pattern around a pointer and writes a line-feed value at a fixed offset when the pattern matches.
- `walk_entries_with_countdown`: Sets a group of global pointers, then walks a linked list of entries, decrementing a countdown byte in each and calling helper routines when a count reaches zero.
- `return_empty_subroutine_2`: The routine's decompiled body is a bare return, so it does nothing when entered.
- `validate_and_initialize_memory_pointers`: Calls the exec memory allocation through a library base several times and, when each allocation succeeds, stores the returned pointers in globals.
- `open_keyboard_device_with_port`: Finds the current task, adds a message port, sets two port fields, and opens keyboard.device through exec OpenDevice.
- `read_mouse_movement_register`: Copies the mouse port movement register value into a global word.
- `return_empty_subroutine_3`: The decompiled body is a bare return, so the routine does nothing when entered.
- `setup_relocation_tables`: Sets a series of global pointers from a loaded structure, allocates memory through exec AllocMem, and computes offset pointers into the new block.
- `copy_block_and_enable_dma`: Copies a block of words from a global area to a second location, updates several global pointers, and writes a DMA control value with its enable bit set.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of words for the value held in the data register, returning the step count at the first match or minus one if nothing matches.
- `do_nothing`: It contains exactly four instructions ending in an unconditional return and performs no operations before returning.
- `apply_scaled_transform`: For a non-negative table index, multiplies a stored value by a scaling word, adds the input value, and writes the result into several words of a table.
- `update_pointer_and_string_tables`: Walks a small table of word values, writing a character derived from each value into a text buffer and updating pointer fields for each entry.
- `return_empty_subroutine_4`: The decompiled body is a bare return, so the routine does nothing when entered.
- `call_indirect_function`: Calls a library function through a base pointer held in a global and returns the data register value unchanged.
- `wait_for_flag_bit_set`: Busy-waits until bit four of a global byte is set, then returns.
- `wait_for_flag`: Busy-waits until the lowest bit of a global byte is set, then returns.
- `call_routines_then_set_flag`: Calls two routines, then stores a single bit value into a global byte.
- `update_pointer_table`: Walks four pointer entries and, for each one, writes several words computed from a stored value, a scaling word and a table value.
- `run_with_supervisor_state_switch`: Sets two byte values in a memory buffer, calls several routines, and makes a library call pair that switches processor state.
- `call_inside_supervisor_state`: Calls two routines, sets a global control value, then calls a library function to enter supervisor state and later calls one to return to user state.
- `reset_state_and_switch_mode`: Zeroes a global control value, calls a library function that changes processor mode, and stores its return value.
- `restore_saved_processor_state`: Calls a library function that changes processor mode, stores its return value, restores a saved global value, and calls the library function that returns to user mode.
- `increment_counter_and_call_helpers`: Increments a global counter word, then calls helper routines in a loop and three further times.
- `clear_interrupt_enable_bit`: Writes a single value with the high bit clear to the interrupt enable register, which disables the bit it names.
- `update_channel_control_bits`: When a global byte is zero, looks up a per-channel counter and, once it reaches zero, writes shifted bit masks to the DMA and interrupt control registers.
- `program_audio_channel_from_table`: When a global byte is zero, reads an entry from a table and computes period and volume values from it.
- `empty_return_routine`: This routine contains no instructions and performs no operations.
- `empty_routine`: This routine does nothing; it returns immediately without performing any actions.
- `return_without_action_2`: The code is only a return, although the text references a main data file name in the executable.
- `do_nothing_2`: It contains no instructions and returns immediately without touching any data or registers.
- `copy_tagged_entries_to_buffer`: When a global bit is set, walks a list of records, selects entries whose type value is one or two and whose flag is set, and copies their data into a buffer in blocks of words.
- `expand_record_list_into_buffers`: Walks a counted list of relative records and expands each one into a destination buffer by copying words with strides and index-based lookups.
- `return_constant_three_thousand`: Returns the fixed value three thousand as a word and does nothing else.
- `tiny_move_rts`: Small routine of 2 instructions that copies 100 to d0, then returns.
- `replicate_global_byte`: Returns the global byte duplicated into both halves of a word.
- `do_nothing_return`: This routine performs no operations; it returns immediately without reading or writing any data, registers, memory, or hardware.
- `copy_pointer_listed_blocks`: Walks a table of source and destination pointer pairs until it reaches a null source, copying a block of words for each pair with a series of unrolled moves.
- `call_routine_then_library_function`: Calls one routine, then calls a library function through a base pointer held in a global, and returns the data register pair unchanged.
- `empty_routine_2`: It contains no executable instructions and performs no operations.
- `store_byte_and_call`: Stores the low byte of the data register into a global variable, calls a library function through a base pointer, and returns the data register unchanged.
- `store_register_and_call_function`: Stores the low byte of the data register into a global variable and then calls a library function through a base pointer.
- `update_global_with_conditional`: If the data register holds three, it is changed to four before the value is stored in a global variable.
- `iterate_until_value`: Calls one routine repeatedly until it returns a non-zero low byte, then calls another routine repeatedly until it returns zero.
- `handle_text_character_position`: Processes one character and moves a column position: a carriage return adds eight, a line feed resets it, and other characters wrap at a fixed width.
- `traverse_string_null`: Walks a null-terminated string, calling another routine for each non-zero character until it reaches the terminator.
- `calculate_scaled_offsets`: Clamps two register values to fixed upper limits and stores each one multiplied by a constant into two global words.
- `empty_return_routine_2`: This routine contains no instructions; it is an empty routine that returns immediately.
- `update_position_limits`: Adds a delta to two position words, clamps the first to zero through 639 and the second to zero through 399, then stores a masked value in a third global.
- `scale_input_values`: Doubles two input register values and stores each result in a global word.
- `get_scaled_value`: Returns a global word halved by a one-bit right shift.
- `find_changed_key_in_matrix`: Compares a twelve-byte state table against a second copy, finds the first changed bit, and converts its position into a key code stored in a global.
- `set_status_flags`: Calls a library function through a base pointer, then rebuilds a global status word from the bits of a status byte, mapping each set bit to a fixed flag value.
- `build_input_state_bits`: Builds a state word in the data register by mapping bits from a global flag word onto a set of output bits, and sets bit seven when the global is non-negative.
- `build_joystick_state_word`: Builds a state word from a global flag word and a byte in a register, setting output bits for each matching flag and bit seven when the byte's top bit is clear.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_permit_if_flag_set`: If a global counter is greater than one, calls the exec Permit function through the library base.
- `call_forbid_if_flag_set`: If a global counter is greater than one, calls the exec Forbid function through the library base.
- `call_library_checking_result`: Calls two routines, then calls a library function through a base pointer and returns minus one when it yields zero.
- `return_zero_value`: Returns zero and does nothing else.
- `call_library_pair_conditionally`: Calls one library function through a base pointer and, when it returns a non-zero value, calls a second one; two other routines bracket the sequence.
- `call_library_or_loop_forever`: Calls a library function through a base pointer, then either returns or, when a global flag is set, calls two more library functions and loops forever.
- `call_and_check_result`: It calls two routines and then invokes a function through a base pointer, returning negative one when that call yields zero.
- `invoke_conditional_pointer_return`: Invokes a routine twice, then dereferences a global pointer to call a function and reads its signed result.
- `call_library_via_base_pointer`: Calls two routines, then calls a library function through a base pointer at a fixed offset.
- `call_external_library_routine`: Calls a library function through a base pointer held in a global, at a fixed offset, and returns nothing.
- `ensure_minimum_value`: Raises a global counter to one if it is below two, calls a library function through a base pointer, and then sets the counter back to one.
- `prompt_disk_and_wait_return`: Waits on a status routine until a condition holds, then unless one of two key codes is pressed, shows a message through several text routines and waits for the return key.
- `move_and_clear_data`: Copies a grid of words from source blocks to a destination area, clearing each source word to zero as it goes.
- `copy_grid_from_staging_buffer`: Copies words from a staging buffer into four pointer-addressed blocks using nested loops with fixed counts.
- `empty_return_routine_3`: This routine performs no operations and returns immediately without modifying any data or registers.
- `wait_for_message_flag_bits`: Polls the exec GetMsg function until a message arrives, then keeps polling until a flag bit inside that message is set.
- `return_stack_argument`: Returns a value taken from the stack unchanged, so it hands back whatever the caller pushed.
- `store_result_or_report_error`: Calls a library function through a base pointer; when it returns zero it calls two routines, and otherwise it stores the result in a global and calls a second library function.
- `call_library_sequence_store_results`: Calls one library function through a base pointer and stores its return value, calls it again, then calls a second library function.
- `poll_until_status_ready`: When the input byte is below two, repeatedly calls two routines until a status value is no longer minus one, then returns the result of a third routine.
- `select_drive_or_fail`: Calls a library function through a base pointer; when that returns zero it yields minus one, and otherwise it stores the result, calls a second library function, and returns the result of a third.
- `tiny_andi_subq_ori`: Small routine of 3 instructions that masks d0 with 79, then subtracts 1 from d0, then merges 0 into d0.
- `swap_buffer_pointers_if_needed`: When a global value is negative, it selects one of two buffer pointers.
- `swap_buffers_when_pointer_differs`: When the data register differs from a stored buffer pointer, it resets five state words, swaps the two buffer pointers, restores the saved state words, and calls two further routines.
- `update_buffer_index_ranges`: Swaps buffer pointers, copies five table words into a new block, and recomputes start and end index values from a size-related offset, adjusting the table entries to match.
- `update_table_values`: Computes a value from a global by shifting it right three bits, adds that value to five consecutive table entries, and then calls a further routine.
- `reset_buffer_state_words`: Sets five state words to fixed starting values and then calls one further routine.
- `initialize_blit_parameters`: It stores several fixed values into consecutive global bytes and then calls a routine that programs the blitter pointer, modulo, mask and size registers.
- `return_zero_value_2`: It returns a constant zero value to its caller.
- `call_four_routines_in_sequence_2`: Calls four routines in sequence and returns.
- `dispatch_through_offset_table_2`: Reads a byte from a stream addressed by a register, uses it as an index into a table of 16-bit offsets, and calls the target at that offset.
- `call_return_zero`: It calls a routine that returns a constant zero value twice and then returns.
- `read_bytes_and_advance_pointer`: Reads two bytes from a pointer, returns them packed with a word from a helper call, and stores the pointer advanced by two into a structure slot.
- `read_short_at_pointer`: It reads a short integer from the memory address held in a register and returns it.
- `read_three_bytes`: Reads three consecutive bytes from a pointer and returns them packed into one value.
- `push_loop_return_frame`: Reads a byte from a stream and updates a small return stack in the caller's frame.
- `record_loop_return_frame`: Updates a small return stack in the caller's frame: when a global flag is clear it pushes a frame and records a position offset by a register value, and otherwise it records the current frame.
- `update_memory_structure`: Copies a word within a caller structure, writes a pointer computed from a register value into another slot, and sets two flag bytes to one.
- `push_value_if_slot_empty`: If a structure slot is zero, stores a value at an offset computed from a register and records that offset in the slot.
- `no_op_trio`: It contains three instructions that do not appear to perform any observable action on the data.
- `clear_slot_if_matching`: Clears a structure slot when it equals the data register value, but only when a global flag is set or the value reaches a global threshold.
- `get_dereferenced_value`: Stores the address register in a global, calls another routine, and then returns the short value stored at that address.
- `read_word_at_pointer`: Stores the address register in a global, calls another routine, and returns the word at that address.
- `read_value_at_stored_pointer`: Returns the value stored at the address held in a global pointer.
- `conditionally_return_pointer_target`: If the counter in a data register is zero, returns the word at the address register; otherwise it returns the data register value.
- `conditionally_return_three_bytes`: If the counter in a data register is zero, packs three bytes from the address register into one value; otherwise it returns the data register unchanged.
- `conditionally_return_pointer_byte`: If the counter in a register is non-zero, returns the byte at the address register; otherwise it returns the data register value.
- `return_word_if_counter_set`: If the counter in a data register is non-zero, returns the word at the address register; otherwise it returns the data register value.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a3, then returns.
- `call_register_copy_helper`: This one-instruction routine is known only from its listed call to a tiny helper that copies one data register into another.
- `stub_undecoded_routine`: The decompiler produced no pseudo-code for this eight-instruction routine, so its behaviour cannot be read from the evidence.
- `dispatch_through_offset_table_3`: Reads a byte from a pointer and calls the target at a table offset computed from that byte.
- `move_register_value`: Copies the value in one data register into another data register via a shared one-instruction routine.
- `stub_undecoded_routine_2`: The decompiler timed out and produced no pseudo-code for this eight-instruction routine, so its behaviour cannot be read from the evidence.
- `call_copy_helper_then_routine`: Calls the tiny helper that copies one data register into another, then calls one further routine.
- `set_global_from_function`: Calls a routine, then stores either the result of a second call or the constant one hundred in a global word, depending on a flag.
- `process_text_mode_character`: Dispatches one character according to a global mode value: mode zero calls a routine, while the other modes update text position words for newline, return and space characters and for ordinary characters within limits.
- `iterate_string_characters`: Calls one routine to obtain a pointer, then calls another routine for each character of a null-terminated string until the terminator.
- `convert_integer_to_string`: Writes a signed number as decimal characters into a buffer, adding a minus sign for negative values and suppressing leading zeros.
- `store_word_indexed_table`: It stores a word from a source pointer into a slot indexed by an offset table, then jumps to a data gap that decodes as code.
- `stub_undecoded_routine_3`: The decompiler timed out and produced no pseudo-code for this eight-instruction routine, so its behaviour cannot be read from the evidence.
- `stub_undecoded_routine_4`: The decompiler timed out on this thirty-instruction routine, so no pseudo-code is available.
- `compact_entry_block_downward`: Looks up a pointer from a table indexed by a register value and, when it is non-null, shifts a block of words in memory downward to close a gap.
- `remove_matching_table_entry`: Finds the table entry for the register index; if it is empty it clears a global flag, otherwise it shifts a block of words down to close the gap and searches the table for the matching value.
- `call_routine_when_flag_set`: If a global byte is non-zero, calls one routine and returns; otherwise it returns without action.
- `walk_entries_or_skip_string`: When a pointer's first word is non-zero, calls a routine and skips past a null-terminated string.
- `find_next_larger_table_value`: When a short field differs from a structure value, scans a table for the smallest entry larger than a value derived from the structure, then calls a further routine.
- `update_linked_list_node`: Sets a flag bit in a node's byte, copies a stretch of bytes into it, and updates several fields including a linked pointer and a fixed value.
- `set_global_if_zero`: Sets a global byte to one when it is currently zero and leaves it unchanged otherwise.
- `clear_global_flag`: Clears a global byte to zero and returns.
- `update_if_different`: Compares a value read through a pointer with a stored field and copies the new value over the field when they differ.
- `store_fields_and_set_flag`: After two unnamed calls, if a stored offset is non-zero, it writes two register values into fields of a table entry at that offset and sets the top bit of a flag byte.
- `modify_bit_flags`: Clears bit six and sets bit seven in a flag byte located at an offset given by a pointer, then calls two further routines.
- `set_bit_in_byte`: Sets bit six of a byte located at an offset read from a pointer.
- `insert_into_linked_list`: Clears the link field of a node, then appends it to the end of a singly linked list, or makes it the head when the list is empty.
- `remove_from_linked_list`: Searches a singly linked list for a node index and removes it by relinking its predecessor to its successor, or by moving the head when the head matches.
- `calculate_cross_products`: Computes three signed byte cross-product terms from six fields in a structure and stores each result in a further field.
- `set_flag_byte_to_one`: Stores one in a byte at a fixed offset from the frame pointer.
- `interpret_offset_command_records`: Interprets a command stream: a negative command code either copies a block of words into two destination buffers and ends, or steps through records that accumulate signed offsets into position globals and call a helper for each record.
- `initialize_record_state_from_frame`: Sets a set of state globals from a byte in the caller's frame and a computed word, then calls routines to continue.
- `init_state_from_frame_word`: Sets a computed state word from a frame byte and clears several globals, then calls routines.
- `init_record_state_from_byte`: Clears a state word and sets it from a byte in the caller's frame, then calls one unnamed routine.
- `init_record_state_with_flag`: Sets a global flag to one, loads a state byte from the caller's frame, and calls an unnamed routine.
- `advance_offset_command_records`: Steps through command records that accumulate signed offsets into position globals, calling a helper routine as it goes.
- `dispatch_offset_command_stream`: Reads command codes from a stream: a negative code either copies a block of words into two destination buffers and ends, or steps through records and accumulates their signed offsets.
- `compare_table_entry_fields`: Looks up an entry through a table index and compares two of its fields with a stored value and a global, but both branches return with no effect in the shown code.
- `compare_table_entry`: Compares two fields of a table entry against a stored value and a global, and returns without effect in both branches as shown.
- `search_linked_list`: Walks a linked list of entries, stopping early when a node's value passes a stored threshold or matches a global byte.
- `scan_list_until_threshold`: Walks a linked list of entries until a node's value is no longer below a stored value, then returns.
- `scan_sorted_list_for_key`: Walks a sorted linked list of entries, moving forward while each stored value is below the search value, and returns when the list ends or the value is reached.
- `return_empty_routine_2`: Performs no operations and immediately returns to its caller.
- `insert_into_list_structure`: Takes a node from a free list, links it into a list at a position chosen by an index, and fills its fields with a value, a flag byte and a constant.
- `store_byte_around_two_calls`: Makes two unnamed calls and stores the same byte register value into two frame slots, once after each call.
- `compute_scaled_vector_offset`: Computes the difference between a stored position and a reference position, scales it by a shift and a division, and derives a bounded value from a control structure.
- `replace_low_word_if_clear`: If a flag byte in a structure is negative, returns the register pair unchanged; otherwise it replaces the low word of the first register with a byte from the structure.
- `load_indexed_table_pointer`: When a global flag is clear, computes a pointer from a frame-held base and a scaled index, then returns the value found relative to that pointer.
- `calculate_pointer_offset`: Computes an address from a frame-held base and a scaled index plus a value read from memory, and returns it paired with the register value.
- `run_loop_until_flag_clear`: Stores a register byte in a global, calls routines in a loop whose continuation depends on the stored test, and then clears the global.
- `clear_flag_and_return`: Loops while a word read through a frame-held pointer is non-zero, calling another routine on each pass, then clears a global byte and returns.
- `traverse_and_update_linked_list`: Walks a linked list of nodes, flagging a node and relinking it onto a head list when a global flag is clear and its byte is non-negative.
- `unlink_node_from_chain`: Removes a node from a linked chain by pointing its predecessor at the node's successor, searching from the chain head when the node is not first.
- `fill_three_words_with_calls`: Calls one unnamed routine before each of three word stores, writing the register word into three consecutive slots at a pointer.
- `copy_two_longs_to_globals`: Copies two long words from a pointer-addressed structure into two global variables.
- `apply_offset_to_linked_entries`: Subtracts reference words from a three-word position record and adds the resulting delta to the three coordinate words of every entry in a linked list.
- `propagate_record_offsets_to_list`: Adds three offset words from a record to its three coordinate words, then walks a linked list and marks zero-state entries with value 2.
- `reset_bounds_and_process_list`: Clears the pending-update flag and copies four bounding words from a source table into working copies.
- `update_record_clip_rectangle`: When a record has its clip-enable bit set, it converts the record's origin and size into a bounding rectangle held in globals, then walks the entry list.
- `insert_and_dispatch_list_states`: Inserts one entry at the head of a linked list, then walks the list and handles each entry by its state byte: zero is skipped, value 2 and negative states take separate paths.
- `store_table_entry_fields`: Calls a subroutine, then writes a 16-bit link value into a table entry.
- `link_entry_at_list_head`: Links an entry at the head of the list kept in a global head pointer.
- `write_table_entry_after_calls`: Calls two subroutines, copies a link word into a table entry, and writes two long words, one word and the global state byte at fixed offsets.
- `reset_bounds_to_extremes`: Sets the four bounding globals to the largest positive and the most negative 16-bit values.
- `expand_global_bounds_with_entry`: When an entry's flag is non-negative, widens the four global bounding values so they include the entry's rectangle.
- `insert_entry_sorted_by_keys`: Walks a linked list to the first node whose sort key (one word, then two byte keys) is not smaller than the given entry's key.
- `update_overlapping_entries`: Returns at once if the record's clip-skip flag bit is set.
- `return_empty_stack_value`: This routine is empty and returns an uninitialized value left on the stack, then returns.
- `copy_bounds_into_table_entry`: Copies four 16-bit bounding values into the table at the current write pointer, advances the pointer by one four-word record, and calls a subroutine when the pointer reaches the table limit.
- `replay_bounds_records_in_table`: Walks the table of four-word bounding records from its start up to the current write pointer.
- `clamp_entry_to_bounds`: Compares an entry's rectangle with the working bounds globals and returns unchanged when they do not overlap.
- `test_overlap_update_bounds`: Tests whether an entry's rectangle overlaps a reference rectangle taken from the referenced shape.
- `compute_aligned_clip_intersection`: Initialises the working clip rectangle from an entry when no clip is active.
- `return_without_action`: Contains only a return instruction, so calling it performs no work.
- `empty_return_routine_4`: This routine contains no instructions and does nothing; it simply returns to its caller without modifying any data or registers.
- `scan_word_table_for_zero`: Walks a length-prefixed table of words and skips negative entries.
- `check_shape_box_within_limits`: Selects a shape descriptor from a table and checks its flag mask against the current flags.
- `check_fixed_box_within_limits`: Checks a fixed-layout shape descriptor against a flag mask.
- `check_wide_box_within_limits`: Checks a shape descriptor that stores full-width offsets against a flag mask.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d1, then returns.
- `copy_record_into_buffer_reverse`: Writes a counted record into a buffer from the end backwards.
- `search_word_table_for_key`: Scans a counted table of four-byte entries for a 16-bit key held in a register.
- `return_empty_routine_3`: This routine performs no operations and immediately returns to its caller.
- `tiny_adda_rts`: Small routine of 2 instructions that adds (a3)+ to a3, then returns.
- `empty_return_routine_5`: The routine contains no instructions and performs no operations on any data.
- `clear_indexed_entry_byte`: Calls a subroutine, then, when the index is non-negative, clears one byte inside the table entry selected by that index.
- `clear_status_low_bit`: Clears the lowest bit of a status byte located 36 bytes below the base pointer, leaving the other bits unchanged.
- `set_status_second_bit`: Sets the bit with value two in a status byte located 36 bytes below the base pointer, leaving the other bits unchanged.
- `clear_status_second_bit`: Clears the bit with value two in the same status byte located 36 bytes below the base pointer, leaving the other bits unchanged.
- `store_low_byte_global`: Calls a subroutine, then copies the byte held in the data register into a global state variable.
- `copy_bytes_to_base_offset`: Copies a block of bytes from a source record into an area addressed from the base pointer.
- `store_masked_then_set_global`: Calls a subroutine, stores the low two bits of a register into a global variable, and then overwrites that variable with one.
- `set_pointers_from_global_index`: Computes a pointer by shifting the global byte left by five bits and adding it to the address of that global.
- `update_global_buffer_from_state`: Clears a state byte, then picks a path from that byte and the sign of the index register.
- `forward_to_subroutine`: Calls one subroutine and returns, passing control onward without changing any values.
- `copy_entry_into_two_buffers`: When the global state is not one or five and a source word equals minus two, copies a 16-word entry into two destination buffers.
- `reset_global_state_byte`: Writes an all-ones byte and then zero twice, leaving a global state byte cleared.
- `store_word_in_indexed_table`: Calls two subroutines, then, unless the global state byte is one of a few excluded values, stores a register word into a table at an offset given by the index register.
- `test_low_nibble_sum`: Adds the low nibbles of two registers and compares the sum with fifteen.
- `test_low_nibble_difference`: Subtracts the low nibble of one register from another and tests the result's range.
- `copy_entry_and_update_state`: Returns early for certain global state values.
- `repack_word_nibble_fields`: Loops over 64 words and, wherever two words differ, compares their three four-bit fields and calls subroutines to adjust them.
- `copy_word_table_with_state`: Returns early for certain global state values.
- `call_one_subroutine_only`: Calls one subroutine and returns without doing any other work.
- `test_nibble_difference`: Calls a routine that subtracts the low nibble of one register from another and tests the result's range, then returns.
- `add_shape_entry_or_children`: Sets a working rectangle from global bounds, then either records a new shape entry with its position, flags and offsets, or walks a counted list of child entries.
- `add_shape_entry_from_offsets`: Walks a counted list of shape entries when the first word is not the marker value, applying each entry's offsets to the running position and calling subroutines for each one.
- `clear_state_if_flagged`: When a given global value is nonzero and the state byte is not positive, sets the state byte to two and then clears it to zero.
- `run_calls_clear_state`: Calls four subroutines in sequence, then clears a global state byte.
- `copy_data_register_value`: Calls a short subroutine that copies one data register into another, then returns.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `dispatch_through_offset_table`: Reads a byte from the source stream and uses it as an index into a table of 16-bit offsets.
- `swap_pointers_then_dispatch`: Swaps two global 32-bit values, then calls the routine selected by a byte at the source pointer through a table of 16-bit offsets.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine_4`: The routine executes four instructions and then returns to the caller without performing any observable operation on its data.
- `copy_null_terminated_string`: Copies a null-terminated string from the source pointer into the destination global pointer, including the terminating zero byte.
- `return_without_effect`: Contains only a return instruction, so calling it changes nothing.
- `return_without_work`: Contains only a return instruction and performs no work before returning.
- `return_immediately`: Executes a return and performs no other operation or data modification.
- `return_empty_routine`: Contains no executable work and returns straight away.
- `copy_string_from_offset_table`: Reads a byte from the source stream, forms an address by adding that byte to the base pointer, and copies the null-terminated string found there into the destination global pointer.
- `compute_weighted_sum_of_words`: Starts from a seed value and adds the products of two word sequences, one read forward from the source pointer and one walked backwards from a base offset.
- `compute_seeded_weighted_sum`: Calls subroutines twice, the second returning a base offset.
- `compute_weighted_sum_indexed`: Calls one subroutine first, then accumulates products of word pairs for a count held in a byte, starting from the index register doubled plus a base offset.
- `accumulate_word_products`: Repeatedly multiplies a word from the source pointer by a word walked backwards from another pointer, adding each product to a running total.
- `move_register_value_2`: Copies the value in one data register into another data register via a shared one-instruction routine, then returns.
- `copy_d7_to_d6`: Copies the value in d7 into d6 and returns.
- `decrement_byte_loop`: It decrements a byte stored through a pointer and then branches back to itself, forming an infinite self-referential loop.
- `compare_registers_after_copy`: Calls a subroutine that copies one data register into another, then compares two register words.
- `compare_registers_equal_after_copy`: Calls a subroutine that copies one data register into another, then compares two register words for equality.
- `compare_words_after_copy_less`: Calls a subroutine that copies one data register into another, then compares two register words.
- `compare_words_after_copy_order`: Calls a subroutine that copies one data register into another, then compares two register words in the opposite order.
- `compare_registers_branch`: Calls another routine and then returns if the value in register d7 is not greater than the value in register d6.
- `compare_words_after_copy_signed`: Calls a subroutine that copies one data register into another, then compares two register words as signed values.
- `call_routine_return`: Invokes another routine and then returns without modifying any data.
- `copy_d7_to_d6_2`: Copies the value in d7 into d6 and returns without modifying any other data.
- `copy_register_data`: Copies the contents of one data register to another using a specialized move instruction, then returns.
- `store_hash_result_global`: Calls a subroutine that computes a hash value and stores its 16-bit result in a global variable.
- `compute_linear_hash_word`: Multiplies the low word of the data register by a fixed constant and adds a fixed offset, returning the resulting 32-bit value.
- `test_sign_no_effect`: Tests the data register for zero or a negative sign, then returns on every path, so it has no effect.
- `tiny_clr_rts_3`: Small routine of 2 instructions that clears d7, then returns.
- `return_immediately_2`: Returns immediately without performing any operation.
- `call_routine_unless_global_set`: Returns at once when a global flag byte is nonzero.
- `call_single_routine_direct`: Calls one subroutine and returns without other work.
- `branch_on_global_zero`: Checks whether a global variable is nonzero and returns early if it is, otherwise invokes another routine.
- `forward_call_without_test`: Calls one subroutine and returns with no test beforehand.
- `store_button_mask_state`: If a global variable holds a non-zero value it returns early.
- `branch_on_global_flag`: Reads a global variable and returns early if it is nonzero, otherwise falls through to run a short subroutine that clears a data register and then returns.
- `call_register_clearing_routine`: Calls a short subroutine that clears the data register, then returns.
- `dispatch_selector_value_actions`: Dispatches on a selector value.
- `call_return_constant`: Invokes another routine that returns the fixed value three thousand and then returns.
- `clear_indexed_global_byte`: Calls a subroutine, then clears one byte of a global table at an offset given by the low byte of the data register.
- `copy_string_into_global_buffer`: Copies a null-terminated string from the source pointer into the global destination pointer, including the terminating zero byte.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d6 to d7, then copies (a4)+ to d6, then returns.
- `loop_calling_subroutine_forever`: Calls a subroutine in an endless loop, so it never returns on its own.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_word_offset`: Writes the byte in the data register to the address formed by the base pointer plus a word offset read from the source pointer.
- `store_word_at_word_offset`: Writes the 16-bit value in the data register to the address formed by the base pointer plus a word offset read from the source pointer.
- `copy_string_to_offset_destination`: Copies a null-terminated string from a global source pointer to the address formed by the base pointer plus a word offset read from the source pointer.
- `store_byte_at_index`: Calls a subroutine to obtain an index, then copies the byte read from the source pointer into the base-relative location at that index.
- `store_word_at_returned_offset`: Calls a subroutine to obtain an offset, then writes the 16-bit word read from the source pointer at the base-relative location.
- `write_byte_at_byte_offset`: Writes the byte in the data register to the address formed by the base pointer plus a one-byte offset read from the source pointer.
- `write_word_at_byte_offset`: Writes the 16-bit value in the data register to the address formed by the base pointer plus a one-byte offset read from the source pointer.
- `copy_string_to_byte_offset`: Copies a null-terminated string from a global source pointer to the address formed by the base pointer plus a one-byte offset read from the source pointer.
- `call_unresolved_routine_once`: The decompiler timed out, so the body is not recovered.
- `add_byte_at_offset`: Adds the byte in the data register to the byte stored at the base-relative address given by a word offset read from the source pointer, and stores the sum back there.
- `add_word_at_offset`: Adds the 16-bit value in the data register to the word stored at the base-relative address given by a word offset read from the source pointer.
- `add_word_at_byte_offset`: Adds the 16-bit value in the data register to the word stored at the base-relative address given by a one-byte offset read from the source pointer.
- `append_global_string_at_offset`: Scans forward from the address formed by the base pointer and a one-byte offset to the terminating zero byte, then appends a null-terminated string from the global source pointer there.
- `chain_three_subroutine_calls`: Calls a subroutine to produce a value, passes that value to a second subroutine, then calls a third subroutine with no arguments.
- `build_concatenated_text_string`: Clears three global values, then copies a source string into a buffer, writing a separator byte when a flag is zero.
- `initialize_globals_and_pointer_table`: Stores fixed initial values into several global variables and an object record.
- `copy_name_with_state_suffix`: Copies a file name into a buffer and replaces the character after its final period with a letter chosen from the global state.
- `expand_control_byte_stream`: Repeatedly reads control bytes from a stream and expands them.
- `read_byte_with_refill`: Returns the byte at the current read pointer.
- `write_byte_if_in_range`: Writes the byte in the data register to a destination computed from a base and an offset.
- `decode_packed_byte_stream`: Decodes a packed byte stream into a buffer.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `compute_absolute_byte_difference`: Returns the first byte when the second byte minus it is not negative.
- `reduce_offsets_by_record_area`: For a record whose type byte is zero or two and whose height field is nonzero, subtracts the record's area from the offset entries beyond it.
- `reduce_offsets_by_entry_area`: When an entry's height field is nonzero, subtracts the entry's area from the stored offset entries beyond it.
- `reduce_offsets_and_clear_entry`: Subtracts an entry's area from the offset table entries beyond it, marks the entry's size fields as cleared, then calls a subroutine.
- `shift_entries_and_offset_pointers`: Copies a block of words down a buffer and reduces the buffer's end counter.
- `offset_pointers_matching_entries`: Walks a linked list of entries and compares an identifier to decide whether to process each one.
- `offset_pointers_in_entry_list`: Walks a linked list of entries and, for each entry at or above a limit, subtracts a shift amount from its pointer fields.
- `offset_pointers_in_all_lists`: Walks every entry in the active list and, for each stored pointer lying beyond a limit, subtracts a shift amount from it.
- `offset_pointers_in_given_list`: Walks the list of entries reached from the supplied pointers and subtracts a shift amount from each stored pointer lying beyond a limit.
- `find_next_larger_table_entry`: When a key differs from the record's stored key, scans a table for the smallest entry above a reference value.
- `call_and_return_stack_value`: Calls one subroutine and returns an undefined value taken from the stack, adding no data of its own.
- `call_and_return_registers`: Calls one subroutine and returns the values held in the data registers unchanged.
- `binary_search_word_table`: Performs a binary search over a table of entries whose leading word is a sorted key.
- `initialize_free_list_entries`: Fills an array of six-byte list entries with increasing offsets and zero values.
- `insert_object_record_into_list`: Inserts a pointer into a key-ordered list and advances the list count.
- `initialize_object_record_from_input`: Takes an entry from a free chain and initialises a new object record from the input record's fields and fixed values.
- `allocate_or_reclaim_object_record`: Initialises a new object record from the input when space remains.
- `remove_object_record_entry`: Looks up an object record by index.
- `evaluate_flag_bits_result`: Tests bits of its input word and of a second word returned by a subroutine.
- `invoke_and_pass_value`: Calls one subroutine and returns the data register value unchanged.
- `retry_until_nonnegative`: Repeatedly calls a subroutine until it returns a non-negative value.
- `poll_until_nonnegative_stored`: Calls a subroutine and stores its result in a global.
- `loop_until_positive_result`: It repeatedly calls a routine until that routine returns a value greater than negative one, then returns the last value obtained.
- `wait_for_nonnegative_result`: Repeatedly calls a subroutine and, while its result is negative, calls a second subroutine.
- `repeat_until_nonnegative_call`: Calls a subroutine, then tests a second call's result and repeats while that result is negative.
- `retry_with_state_until_valid`: Calls a subroutine and returns its data value once the result is non-negative.
- `loop_until_nonnegative_value`: Calls a subroutine repeatedly until its result is non-negative, calling a second subroutine between attempts, then returns the data register.
- `loop_until_second_nonnegative`: Calls a subroutine and keeps its result, then calls a second subroutine and repeats while that second result is negative.
- `run_state_countdown_actions`: Steps a global countdown state through a short sequence, calling subroutines at each stage while the state stays positive.
- `advance_state_with_input_value`: Advances a global state byte when it is non-negative.
- `decrement_counter_and_accumulate`: Decrements a counter in a record.
- `call_four_routines_in_turn`: Calls four subroutines in sequence and then returns.
- `reset_record_fields`: Sets one record byte to zero, another to zero, a third to 128, and clears two word fields.
- `step_timed_value_with_counters`: Advances a value by a step held in a global and clamps the sum to the largest positive 16-bit value.
- `advance_value_with_step_clamp`: Adds a step to a global value and clamps the sum to the largest positive 16-bit value.
- `decrement_counter_and_call_twice`: Decrements a global counter and then calls two subroutines in sequence.
- `call_routine_then_clear_flag`: Calls one subroutine, then clears a global flag byte.
- `report_error_text_and_reset`: Clears a global flag and calls subroutines that use the referenced text Error.
- `return_with_nothing_done`: Contains only a return instruction, so it does nothing.
- `add_constant_when_flagged`: Calls a subroutine and, when its result is nonzero, adds two hundred thousand to a global variable.
- `pass_through_register_pair`: Calls one subroutine and returns the two data register values unchanged.
- `call_clip_and_shift`: Invokes another routine that clips a rectangle against bounds and shifts four bitplane words per column by a sub-word offset, then returns.
- `shift_and_merge_bitplanes`: Clips a rectangle against bounds, then shifts four bitplane words per column by a sub-word offset.
- `copy_four_plane_rows`: Copies sixteen rows of words for four planes from the source table into the destination pointers, stepping each row by fixed strides.
- `tiny_ori_4`: Small routine of 1 instruction that merges 117 into d0.
- `configure_blitter_and_start`: Computes a destination address from the rectangle globals and programs the blitter's pointer, modulo, mask and size registers.
- `blit_rows_with_wait_loops`: Loops over a number of rows, programming the blitter's pointer and size registers for each row and waiting for it to finish.
- `copy_block_across_plane_pointers`: Copies a rectangle across four plane pointers with a per-row stride.
- `program_blitter_copy_pass`: Computes a source offset from the rectangle globals and programs the blitter's size, modulo, pointer and control registers.
- `setup_blitter_masked_line`: When the record's two flag bits are both clear, programs the blitter's mask, offset and size registers.
- `copy_clipped_rows_via_lookup`: Clips a rectangle against the window globals, then copies rows of bytes through a 256-entry lookup table.
- `clip_object_return_status`: Clips an object's bounding box against the window globals and returns early when it lies outside them.
- `clip_object_and_program_blit`: Clips an object's bounding box against the window globals and returns without drawing when it lies outside them.
- `release_channel_and_set_mask`: When the high flag of a channel's status entry is set, clears that entry, sets a flag byte in the channel record and sets a single-bit mask for the channel index.
- `service_pending_interrupt_bits`: Calls a subroutine for each of four pending bit flags that are set in a global value.
- `advance_channel_countdown`: Decrements a per-channel countdown.
- `copy_word_to_audio_volume`: Copies a word from a pointer-offset location into the audio volume register selected by an index.
- `call_routine_four_times`: Calls one subroutine four times in succession and returns.
- `empty_return_routine_6`: This routine does nothing; it is empty and returns immediately without performing any operation.
- `set_flag_or_call_handler`: When a counter global is nonzero, sets the global flag to one and returns.
- `retain_global_value_only`: Copies a global variable onto itself twice, which leaves its value unchanged.
- `scale_global_by_fraction`: Multiplies a global variable by six and shifts the product right by five bits, storing the result back in the global.
- `clear_audio_channel_volumes`: When a global flag is nonzero, writes zero to the four audio channel volume registers.
