# Game 1932 (ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01932). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2727264 top-level bytes in 180 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 116 of 116 named (0 author symbols), 116 of 116 described (9 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1215 of 1215 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 6050 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2519 of 2519 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 103091 |
| code | 9302 |
| data | 2484710 |
| padding | 45169 |
| unused | 84992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:t.x | 8 | 251 | 6 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:LOADER | 2 | 267 | 8 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:bootblock | 1 | 20 | 1 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1160 | 61 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 765 | 37 |
| Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:IBOISHAR.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:GERDEP.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PRESENT1.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:MAIN.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PRESENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PRESOUND.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:MESSAGE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:GESORT.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PRESENT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:MONTAGNE.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PARAM.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:GEANT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:THEEND1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:EATT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:NAIN4.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:EDPLT.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:TEXTIN.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:LOGO.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:SCOMB.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:LOSTGAME.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:GUARD4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PIAF4.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:RHINO4.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:SOURIS.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:DRUIDE4.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:BOB4.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:SMONTAGN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:OBJET.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:IO.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:ODWOMAN5.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:GOLEM1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:WITCH5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SDONJONS.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PP11.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:FLY1.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PP12.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PPSAC.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJCOL.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:VILLAGE1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PUIT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJTRAP.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:MONSTER5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:BOAT.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:LEZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:LIONMAN5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:ELMTFEU5.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJFONT.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJDEC.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJ2.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SKELETT5.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:KROG.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:ELMTAIR5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:AIGLE.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PCAVE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:MEDUSA5.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:MOMY5.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:FDJCAT.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SORTMOR5.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SILE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:WIZARD5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:ORK5.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DWIZARD5.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PLAINE1.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DRAGON5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJ1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:CHAOSGU5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:SORCIER5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:WIZARD1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:ANIMALS.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:POURS1.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:FORET1.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:DJCAT.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 2 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:IO.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:FARBO.CO: high-entropy data file (7.73 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VMAIRIE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:SVILLE.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VHOUSE1.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:HOMOIS2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:FVILLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:SPAROLES.CO: high-entropy data file (7.67 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:DRUIDE2.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VHOUSE2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:RELICMAN.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VPORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PEOPLE1.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PP3BAN.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VBIBLI.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VARCHE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:ARBO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PEOPLE2.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PPECHAP.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:LUTIN3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:EWOK2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VARCADE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:MARCHAND.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VPONT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VPASSAGE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VILLE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VNIGHT.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PP2I.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:TROUBA3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:DWOMAN2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:SCITE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:TEXTIN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PP3MN.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:GUARD3.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VDEC1.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:HORNY4.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VDEC2.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:TAVERNE.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VDEC3.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:SMER.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:GUARD3HI.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VJAIL.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:SAUB.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:ENT2.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PPTRAVEL.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:PP3TB.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VTEMPLE.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:VOLEUR3.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- runtime segment an address: 572 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 303 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.51)
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 3 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:t.x: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ishar 2 - messengers of doom (1993)(silmarils)(fr)[cr skr](disk 1 of 3)/Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:LOADER: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 6050 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 11264 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:t.x

- `program_entry_setup_memory`: Program entry point that calls two unnamed routines, runs a short loop of routine calls, clears a stored pointer, and when a particular memory word matches a pattern it swaps it with a global value before returning.
- `decrement_until_minus_one`: Decrements a local 16-bit counter in a loop until it reaches minus one, then returns.
- `clear_cache_and_decode_passes`: Calls the exec cache-clear routine, runs four passes of a bit-stream decoder whose inner loop adds decoded values into an output buffer, and calls a library routine when a status word exceeds a fixed threshold.
- `decrunch_backward_bit_stream`: Decodes a backward-reading compressed bit stream with a bit reservoir and length tables, copying literal runs and back-references into a destination buffer until the output pointer reaches the start.
- `decrunch_backward_bit_stream_variant`: Performs the same backward bit-stream decoding as the neighbouring routine with a different exit path, copying literal runs and back-references and stopping on a flag set by the decoder state.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:LOADER

- `load_hunk_executable_sections`: Parses an AmigaDOS hunk executable: allocates memory for each code or data section, copies its contents, applies relocation records by adding the base address to listed longwords, and finishes with library calls when the end-of-file record is reached.
- `spin_countdown_delay`: Spins in a countdown loop over a 16-bit register until it wraps, acting as a fixed delay, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_backward_lz_block`: Decodes a bit-packed compressed stream into memory from the end towards the start, copying literal runs and back-references to earlier output.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_with_carry`: Shifts a multi-word bit buffer by the count in d0, refilling the top bits from the previous memory word when the buffer runs empty.
- `stub_bad_instruction_data`: Contains only a trap instruction followed by undecodable bytes, so it has no recoverable behaviour.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 2 of 3).adf:bootblock

- `claim_disk_resource_for_boot`: Opens the disk resource through the exec library and, if it exists, sets three unit fields to minus one before a second library call.

### Ishar 2 - Messengers of Doom (1993)(Silmarils)(Fr)[cr SKR](Disk 3 of 3).adf:bootblock

- `initialize_system_resources`: Calls two functions via a pointer and, if the first returns a non-zero value, writes a constant value of all ones to three consecutive longwords at an offset from that returned pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helpers_store_results`: Calls two helper routines in sequence and stores each result in its own global.
- `return_without_action`: Contains only a return instruction, so it performs no work.
- `follow_scaled_pointer_chain`: Repeatedly replaces the address with the longword stored there multiplied by four, for the count held in d1.
- `return_stored_d0_with_d1`: Returns the d0 value saved by the earlier helper call in the high 32 bits together with the incoming d1 value in the low 32 bits.
- `copy_longwords_to_low_memory`: Copies longwords one at a time from its own code start up to a global boundary into a fixed low-memory destination.
- `patch_newline_by_byte_pattern`: Checks the bytes around the current pointer against fixed values and, when they match, writes a newline into one of two fixed offsets ahead.
- `write_newline_on_pattern_match`: Checks the bytes around the current pointer against fixed values and, on a match, writes a newline at a fixed offset ahead.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `nop_padding`: This routine is empty and performs no operations.
- `open_libraries_check_chip_memory`: Opens the required system libraries in turn, checks that enough chip memory is available, and allocates the working buffers.
- `open_keyboard_device_with_port`: Finds the current task, adds a message port, sets two pointer fields and opens the keyboard device.
- `read_joystick_port_counter`: Reads the joystick and mouse port movement register into a global and returns.
- `allocate_buffers_and_set_limits`: Computes buffer sizes from a value in memory, allocates the block and its four-field header, and clears the header.
- `copy_block_and_start_copper`: Forbids multitasking, copies a block of words to a new location while adjusting pointers, and sets the copper list pointer.
- `find_word_index_in_table`: Searches a table of words in memory for the value in d0, stepping two bytes at a time.
- `return_minus_one_word`: Returns the 16-bit value minus one with no other effect.
- `return_without_action_2`: Contains only a return instruction and performs no work.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `call_library_function_passthrough`: Calls one library function through a base pointer held in a global, then returns the d0 register unchanged.
- `wait_until_bit_four_set`: Loops until bit four of a memory location is set, then returns.
- `wait_for_vertical_beam_bit`: Loops until the lowest bit of a shadowed vertical beam position status value is set, then returns d0 unchanged.
- `clear_interrupt_enable_bit`: Calls two helper routines and then writes a value with bit four set and bit fifteen clear to the interrupt enable register, which disables that interrupt.
- `scale_add_rows_by_table`: Walks a table of destination pointers and, for each one, multiplies four source longwords by a scale value and adds an offset.
- `run_setup_between_supervisor_calls`: Sets bits in a global, writes two byte values through a pointer and runs four helper routines.
- `enter_supervisor_then_restore_user`: Clears a global, calls the exec supervisor-state function, stores the routine's own code address in the global, and then calls the exec user-state function to restore the mode.
- `switch_to_supervisor_and_back`: Calls the exec supervisor-state function and then the user-state function with no other work, switching the mode and back again.
- `increment_counter_and_run_calls`: Increments a global counter, then makes a fixed sequence of calls.
- `disable_vertical_blank_interrupt`: Writes a value with bit five set and bit fifteen clear to the interrupt enable register, which clears the vertical blank interrupt enable.
- `update_guarded_channel_countdown`: Returns early when a global flag is set.
- `update_channel_countdown_flags`: Indexes a per-channel record by the channel number and decrements its countdown byte.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `transfer_records_between_bitplanes`: Walks a linked list of image records and, for each flagged record, moves data between packed source words and four bitplane buffers in 40-byte rows.
- `return_constant_value`: Returns a fixed count of three thousand in d0 and does nothing else.
- `store_byte_then_call_library`: Stores the low byte of d0 into a global, calls a library function through its base pointer, and returns d0 unchanged.
- `store_register_and_call_function`: Stores the low byte of d0 into a global and calls a library function through its base pointer, then returns d0 unchanged.
- `remap_value_then_call_library`: Maps the value three to four in d0, stores the low byte of the result in a global, and calls a library function through its base pointer.
- `wait_for_nonzero_then_zero`: Calls a routine repeatedly until it returns a non-zero byte, then calls it again until it returns zero.
- `print_character_with_line_wrap`: Handles one character for an on-screen text line: a carriage return advances the position by eight, a line feed resets it to zero, and any other character is passed to a library routine and advances the position by nine.
- `call_routine_for_each_char`: Walks a null-terminated byte string and calls one routine for each non-zero byte, stopping at the terminator.
- `clamp_and_scale_coordinates`: Caps the first input at 39 and the second at 24, then stores them multiplied by nine and eight into two globals.
- `nop_padding_2`: This routine is empty and contains no executable instructions.
- `update_mouse_position_clamped`: Adds the mouse counter deltas to a position, clamps the horizontal result to 0 through 639 and the vertical result to 0 through 399, then masks the stored bits.
- `double_two_inputs_into_globals`: Doubles the d0 and d1 input values and stores each result in its own global variable.
- `find_changed_table_entry`: Compares a 12-byte table with a copy stored 16 bytes ahead to find the first byte that changed.
- `perform_io_and_remap_flags`: Runs an exec I/O request through the library, then rebuilds a status value from a flag byte by mapping each set source bit to a fixed output bit.
- `compute_status_bits_from_flags`: Builds a status word from bits of a global flag value, keeps the upper 16 bits of d0, and sets result bits according to which flag bits are set.
- `decode_joystick_port_one_bits`: Decodes the joystick port one movement state into direction flags, keeping the upper word of d0.
- `branch_to_subsequent_routines`: This routine performs a sequence of unconditional branches to two different routines.
- `nop_sequence`: The routine is empty and performs no operations.
- `permit_multitasking_if_nested`: Calls the exec permit function to re-enable multitasking, but only when a global counter is greater than one.
- `forbid_multitasking_if_nested`: Calls the exec forbid function to stop multitasking, but only when a global counter is greater than one.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_library_or_loop_forever`: Calls one library function and returns when a global is zero.
- `call_library_remap_zero_result`: Calls two helper routines, then calls a library function and returns its result, except that a zero result is returned as minus one.
- `call_external_library_function`: Calls one function through the library base pointer held in a global and returns.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `wait_for_flagged_message_port`: Fetches messages from an exec message port and, if the first one has a certain flag combination, keeps fetching until one arrives with the top flag bit set.
- `return_stack_argument_unchanged`: Returns the 64-bit value found on the stack without changing it.
- `call_library_pair_store_result`: Calls one library function twice, calls a second library function, and then stores the result of a helper routine in a global.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_global_on_condition`: Calls another routine and, if the returned value is non-zero, increments a global variable by a large constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_until_counter_ends`: Sets several global pointers, then copies bytes from a source pointer to a destination pointer until a counter in a register reaches its end value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_after_setting`: Writes 255 to a global and then writes zero to it twice, leaving the value cleared.
- `copy_register_data`: Copies the contents of one register to another register.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `dispatch_jump_table_by_byte`: Reads a byte at the current pointer and calls the routine selected by that byte from a table of word offsets relative to a base address.
- `chain_three_helper_calls`: Calls a helper routine, passes its 64-bit result to a second helper, then calls a third helper.
- `initialize_screen_bounds_and_table`: Stores the bounds 319 and 199 into state fields and clears several flags.
- `copy_name_set_extension_letter`: Copies a null-terminated name into a buffer and replaces the character after its last dot with a letter chosen from a global state value.
- `decode_run_length_stream`: Loops reading control bytes from a stream.
- `read_byte_refilling_buffer`: Returns the byte at the current pointer, first refilling the pointer from a global buffer through a helper when it reaches its limit.
- `write_byte_with_bounds_check`: Writes the low byte of d0 to a destination computed from a global base minus an offset, skipping the write when a limit check passes and the offset is not positive.
- `decode_bit_packed_runs_forward`: Decodes a bit-packed stream into a buffer from the start forwards, reading length codes in which a value of three or seven extends the count before bytes are written.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `return_absolute_byte_difference`: Computes the byte difference between two values and returns the first value when the difference is non-negative.
- `subtract_block_size_from_table`: Clears a global and calls a helper.
- `update_table_sizes`: Calculates a size based on two short values from a structure and subtracts that value from multiple entries in a table and two specific fields in a different structure.
- `subtract_block_size_and_reset`: Computes a block size from the record's two size fields and subtracts it from every table entry beyond the record and from two more fields.
- `shift_and_adjust_table_entries`: Copies a block of words to a new position, subtracts a delta from table entries beyond a limit, and moves a second list before adjusting its entries.
- `subtract_delta_from_record_list`: Walks a linked list of records and, for each record whose field passes a threshold, subtracts a delta from its position fields and from entries in its sub-list.
- `adjust_record_chain_by_delta`: Walks a chain of records and compares a key with each one.
- `subtract_delta_from_entry_fields`: Walks a chain of records and their sub-lists, subtracting a delta from each 24-bit field that is above a threshold.
- `subtract_delta_from_sublists`: Walks a chain of records passed by pointer and subtracts a delta from each sub-list field that is above a threshold.
- `call_helper_return_register_pair`: Calls a helper routine and returns the d0 and d1 register values as a 64-bit pair.
- `binary_search_word_table`: Binary-searches a sorted table of pointers to words for the value in d2, stopping as soon as it finds a match.
- `initialize_offset_list_entries`: Fills a list with entries holding increasing offsets, stepping by six up to the list end, and zeroes each entry's paired field.
- `insert_sorted_entry_shift_table`: Finds the insertion point for a new entry in a sorted pointer table by comparing word keys.
- `create_record_from_template`: Builds a new record in the free area from a template record.
- `remove_record_and_compact`: Looks up a record by its key in a linked table and returns early when it is absent.
- `poll_status_until_ready`: Calls a status routine through a global pointer and loops until it returns a value with the top bit set and bit ten clear.
- `call_routine_keep_d0`: Calls one routine through a fixed pointer and returns the d0 value unchanged.
- `retry_until_result_nonnegative`: Calls a routine and, while its result is negative, calls a helper and retries.
- `retry_fixed_call_until_nonnegative`: Stores the result of a fixed routine in a global and, while it is negative, calls a helper and retries.
- `retry_store_call_until_nonnegative`: Stores the result of a routine in a global and, while it is negative, calls a helper and retries.
- `retry_until_nonnegative_fixed`: Repeatedly calls a fixed routine and a helper until the routine returns a non-negative value.
- `retry_helper_while_negative`: Calls a helper and then a fixed routine, repeating while the routine's result is negative.
- `retry_pair_until_nonnegative`: Calls a routine with its register arguments and, while the result is negative, calls a helper and retries.
- `call_four_routines_in_sequence`: Calls four routines one after another and returns without a result.
- `initialize_block_header_fields`: Clears a status byte and a flag byte, sets the next byte to 128, and zeroes two words at fixed offsets from the block pointer.
