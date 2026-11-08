# Game 1072 (ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01072). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2727712 top-level bytes in 187 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 90 of 90 named (0 author symbols), 90 of 90 described (10 by rule, 72 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3321 of 3321 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 6050 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4760 of 4760 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 103075 |
| code | 15668 |
| data | 2508753 |
| padding | 45944 |
| unused | 54272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:loading | 2 | 267 | 8 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:Failat | 2 | 117 | 2 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:T.X | 10 | 251 | 6 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2663 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 511 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 911 | 52 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:IBOISHAR.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:GERDEP.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PRESENT1.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MAIN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PRESENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PRESOUND.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MESSAGE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:GESORT.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PRESENT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MONTAGNE.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PARAM.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:GEANT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:THEEND1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:EATT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:NAIN4.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:EDPLT.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:SCOMB.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:LOSTGAME.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:GUARD4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PIAF4.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:RHINO4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:SOURIS.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:DRUIDE4.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MESSAGED.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:BOB4.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:SMONTAGN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:OBJET.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:MESSAGEE.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:ODWOMAN5.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:IO.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:GOLEM1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:WITCH5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SDONJONS.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PP11.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:FLY1.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PP12.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PPSAC.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:VILLAGE1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJCOL.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PUIT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJTRAP.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:MONSTER5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:BOAT.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:LEZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:LIONMAN5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:ELMTFEU5.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJFONT.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJDEC.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJ2.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SKELETT5.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:KROG.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:ELMTAIR5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:AIGLE.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PCAVE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:MEDUSA5.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:MOMY5.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:FDJCAT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SORTMOR5.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SILE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:ORK5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:WIZARD5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DWIZARD5.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:TEXTINE.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PLAINE1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DRAGON5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJ1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:CHAOSGU5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:SORCIER5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:WIZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:POURS1.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:ANIMALS.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:FORET1.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:DJCAT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:FARBO.CO: high-entropy data file (7.73 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VMAIRIE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:SVILLE.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VHOUSE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:HOMOIS2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:FVILLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:SPAROLES.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:DRUIDE2.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VHOUSE2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:RELICMAN.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VPORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PP3BAN.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VBIBLI.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VARCHE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:ARBO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:LUTIN3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:EWOK2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VARCADE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VPONT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VPASSAGE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VILLE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VNIGHT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PP2I.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:TROUBA3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:DWOMAN2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:SCITE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PP3MN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:GUARD3.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VDEC1.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:TEXTIND.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VDEC2.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:TEXTINE.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VDEC3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:GUARD3HI.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VJAIL.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:ENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:PP3TB.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VTEMPLE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:VOLEUR3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- runtime segment an address: 1952 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 499 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 374 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.33)
- ishar 2 - messengers of doom (1993)(silmarils)(m3)[cr pdx][a2](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:T.X: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 6050 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:loading

- `process_hunk_stream`: Parses an AmigaDOS hunk stream, tracking executable and relocation sections by their type codes, and applies relocations by copying data and calling a handler to fix up addresses.
- `decrement_loop_counter`: Decrements a data register in a loop until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_copy_buffer_bytes`: Reads a length from a buffer, shifts a value right by it, and copies bytes backward through memory until the source pointer reaches a limit.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_shifted_value`: Rotates a 64-bit value held across two registers right by a shift amount in a loop that runs a number of times taken from a register, reloading the high half from a buffer when needed.
- `store_word_via_movep`: Writes the 16-bit contents of data register 4 to the memory location pointed to by address register 2 plus offset 192, then returns.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_branch`: Copies the value in d1 into d0 and then branches to the target address, transferring control to the surrounding routine.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:T.X

- `call_helpers_and_swap_pointer`: Saves the caller's registers, calls a helper routine several times with different values, then loops calling a function pointer from a global variable until a counter reaches five.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_repeated_calls`: Calls a function pointer several times, then runs a loop that executes it repeatedly while its results are negative or non-zero, accumulating values into a table, and finally calls another function pointer if a word in a global variable exceeds 36.
- `set_a1_and_d0_values`: Loads the address of a global variable into address register 1 and places the value two into data register 0, then returns.
- `reverse_copy_decode_stream`: Copies a run of bytes from the source buffer in a0 to the destination buffer in a1, reversing their order, while decoding a compressed data stream in a4 that stores byte counts and copy actions.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(M3)[cr PDX][a2](Disk 3 of 3).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_blitter_transfer`: Copies a block of blitter registers from a lookup table into the blitter data registers, then sets up the transfer size and pointer registers from values in a structure, and returns.
- `setup_blitter_transfer`: Computes the minimum and maximum of the two 16-bit values passed in d1 and d3, storing the smaller in d3 and larger in d2, then derives a signed offset d6 from the pointer in a0 scaled by the smaller value and writes it to a global variable.
- `find_first_nonzero_entry`: Stores the address 33574 into a global variable at offset 12, then scans a byte table of 511 entries to find the first value greater than zero, computing a pointer into a second table and copying 30 consecutive 32-bit entries into a third table.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `manage_dma_pointer_update`: Increments a global counter and, if it does not wrap, compares two adjacent bytes in a table to decide whether to clear it and invoke another routine twice, otherwise it invokes that routine once.
- `setup_interrupts_and_dispatch_command`: Initializes the interrupt vector table and configures the CIAB parallel port, then reads a 4-bit command from a structure and applies it: copying a value, clamping a field up or down, or searching a table of values.
- `initialize_interrupt_and_audio_setup`: Initializes the interrupt vector table by saving the old interrupt handler and installing a new one, then configures the CIAB parallel port and enables audio DMA channels.
- `initialize_interrupt_table`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `dispatch_structure_command`: Reads a 4-bit command from a structure and dispatches on it: copying a value, adjusting a structure field with clamping, or searching a lookup table for a scaled result.
- `copy_structure_field_value`: Copies a 16-bit value from offset 16 of the structure in a6 to offset 6 of the structure in a5, then returns.
- `scale_value_via_lookup_table`: Searches a lookup table for the first entry that is greater than a masked 16-bit value in the structure at a6, then stores the corresponding scaled result into offset 24 of that structure.
- `compute_scaled_value_lookup`: Adjusts a value in a structure by a signed amount, clamps it, then looks up a scaled result in a table and stores it into another structure.
- `compute_scaled_value`: Reads nibbles and flags from a structure, computes a signed scaled value, stores it into a word in another structure, and updates a flag byte, otherwise it returns.
- `process_structure_command`: Reads a command byte from a structure and dispatches on its value to update that structure and a global variable, then returns.
- `store_low_nibble_value`: Stores the low nibble of a byte into a structure and returns, otherwise branches to a shared routine.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine`: The routine contains an infinite loop with no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `allocate_memory_and_build_bitmap`: Allocates several memory blocks from the system and stores their addresses in a global variable, then builds a custom chip-memory structure containing a bitmap and a table of 16-bit values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_two_values_then_dispatch`: Calls a routine twice, storing each returned value into two global variables, then selects a source based on a byte and stores a pointer plus a decremented address onto the stack before restoring the registers and returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `scale_pointer_in_loop`: Loads a pointer from the memory location in a0, scales it by four, stores the scaled value back, decrements the counter in d1, and loops until the counter reaches zero before returning.
- `enter_blitter_setup`: Pushes the return address onto the stack, saves the caller's registers, and jumps into the middle of another routine at its branch target.
- `copy_memory_block_loop`: Copies four bytes at a time from a source address to a destination word in a global variable, advancing both pointers, until the source reaches a fixed limit address, then returns.
- `check_pattern_and_write_2`: Tests a byte at a pointer and, when it is zero, compares bytes at fixed offsets against constant values, writing a newline byte at a large offset when either matching pattern is found.
- `check_pattern_and_write`: Compares four bytes at fixed offsets from the pointer in a3 against constant values and writes a newline byte at offset 860 when all four match, then returns the byte at offset zero.
- `empty_stub_5`: Does nothing; it is a single unconditional jump to another routine and returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `allocate_and_open_system_libraries`: Allocates graphics and intuition libraries and checks each allocation, then allocates a large chip-memory block and stores its boundaries in global variables.
- `initialize_and_call`: Calls a function pointer and stores its result, sets up fields of a structure with constants, then calls another function pointer and returns its value if non-zero.
- `copy_joy_pad_data`: Copies the value from the joy pad data register into a global variable and returns immediately.
- `setup_relocation_tables`: Initializes several global pointers from a base address, allocates memory through a library in a branch depending on a flag, and stores computed offsets and constants into global variables.
- `copy_table_and_disable_dma`: Copies a run of 16-bit words from a source table to a destination table until the source pointer reaches a fixed end address, then stores the copied length and several derived pointers into global variables.
- `compare_table_max_value`: Loads the maximum 16-bit value into d1 and the same value into d0, then compares it against the word at a table and branches back to the start when they are equal.
- `load_max_values`: Loads the value 65535 into both d0 and d1 and returns.
- `load_value_into_d1`: Loads the value 65534 into d1 and returns.
- `branch_on_d0_sign`: Tests d0 and branches to a fixed address when the result is negative or zero, otherwise it returns.
- `call_indirect_function`: Saves registers, loads a library base pointer from a global variable, clears d0, and calls the indirect function pointer at offset -234 before restoring the registers and returning.
- `wait_for_custom_bit`: Waits in a loop until bit 4 of the custom register at offset 29 becomes set, then returns.
- `wait_for_vpos_bit`: Saves d0, loops reading the vertical position register until bit 0 is set, then restores d0 and returns.
- `wait_for_vsync_bit`: Saves d0, waits for a vertical retrace bit to be set, then restores d0 and returns.
- `update_pointer_table`: Iterates over two nested loops, multiplying a value from a table by a constant, adding a pointer, and storing the 32-bit result at two offsets of a destination pointer before restoring registers and returning.
- `write_instruction_sequence`: Sets bit 1 of a global variable, copies a global pointer and its incremented value into two other globals, writes the bytes 80 and 176 at consecutive offsets from the original pointer, then calls four routines and writes the value 1920 to an interrupt-enable register.
- `clear_bytes_disable_ints`: Clears three consecutive bytes at offsets 19, 20 and 21 of a global variable, then saves the current processor status register on the stack and loads a privileged status value into it.
- `set_system_state`: Calls a routine through a pointer retrieved from a global variable, stores a global value into an interrupt-enable register, then calls another routine through a different pointer before returning.
- `increment_and_loop_calls`: Increments a byte in a global variable, calls a routine, then loops three times calling another routine via a subroutine, and finally calls two more routines before restoring registers and returning.
- `set_interrupt_enable_value`: Writes the value 32 to an interrupt-enable register and then returns.
- `test_flag_byte_zero`: Tests a byte in a global variable and returns immediately when it is zero.
- `update_channel_countdown_flags`: Indexes a table by a shifted register value, decrements a byte at offset 1 when two conditions hold, then writes derived bit-mask values to interrupt and DMA control registers depending on further flag checks.
- `store_byte_and_call`: Stores the low byte of a register into a global variable at offset 422, then calls a routine through a pointer retrieved from another global variable before returning.
- `store_register_and_call_function`: Stores the low byte of a register into a global variable at offset 423, then calls a routine through a pointer retrieved from another global variable before returning.
- `update_global_with_conditional`: Compares a register to the value 3, replaces it with 4 when equal, stores the result as a byte into a global variable, then calls a routine through a pointer before returning.
- `loop_until_nonzero_then_zero`: Calls a routine in a loop until it returns a non-zero value, then calls it again in a loop until it returns a zero value, then restores registers and returns.
- `handle_wrapped_line_buffer`: Dispatches to one of several routines based on a byte value, then updates a structure by handling carriage return, line feed, and wrapped-line cases before storing the byte and calling a function pointer.
- `traverse_buffer_until_zero`: Reads a byte from a pointer and advances the pointer, and calls a subroutine for each byte until a zero byte is reached, then returns.
- `calculate_scaled_offsets`: Clamps two register values to upper bounds of 39 and 24, multiplies each by 9 and 8 respectively, and stores the products into two global variables before returning.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `update_position_limits`: Reads mouse movement from a hardware register, subtracts stored values, adds the deltas to stored position variables, clamps each within fixed ranges, and saves the masked input and updated positions into global variables.
- `scale_input_values`: Doubles the contents of two registers and stores the results into two global variables before returning.
- `scan_table_for_differing_byte`: Scans a table of bytes looking for a differing value, computes a bit position for it, and either clears the table or stores a derived value into a global variable.
- `set_status_flags`: Calls a function pointer stored at offset -456 of a global variable, then clears a byte in a global variable and sets bits in it based on the flag states at offset 12 of a table, and stores the result at offset 21 of the same global variable.
- `read_fire_button_register`: Reads the fire-button register at offset 255 of the custom chip and branches to a shared handler when the input register is zero, otherwise falling through to return.
- `read_joy_set_flag_bits`: Reads the joystick data register into d1 and clears d0, then sets bit 2 of d0 when bit 1 of the register is set and bit 1 of d0 when bit 0 of the register is set.
- `test_bits_and_set_flag`: Tests bit 0 of a register and branches away when it is set, tests bit 9 and branches away when it is clear, sets bit 2 of another register, tests bit 8 and branches away when it is set, then sets bit 0 of that register before falling through.
- `test_bit_and_set_flag`: Tests bit 8 of d1 and returns when it is set, then tests d2 and sets bit 7 of d0 before restoring registers and returning.
- `execute_permitted_function`: Checks whether a global variable is greater than one and, if so, calls a function pointer stored at offset -138 of that variable, otherwise it returns.
- `execute_indirect_function`: Checks whether a global variable is greater than one and, if so, calls a function pointer stored at offset -132 of that variable, otherwise it returns.
- `call_functions_and_store_result`: Calls a subroutine, then calls a library function and stores its result into a global variable, calls the same library function again, calls a different library function, compares a value from a structure against a constant and branches when it is below it, calls another subroutine, and stores a byte into a global variable before returning.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `swap_state_words`: Compares a value against a global variable and, when they differ, swaps two words in a global structure and calls a routine through a pointer.
- `initialize_system_state`: If the byte in d0 differs from the stored expected value, it resets a global variable, sets a constant to 319 and another to 199, stores 80 in a third, calls a helper routine, swaps two 32-bit words in the global variable, then calls another routine twice before returning.
