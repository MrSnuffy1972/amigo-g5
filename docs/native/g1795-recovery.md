# Game 1795 (civilization v855.01 (1992)(microprose)[tr sp]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01795). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604656 top-level bytes in 320 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 30 of 30 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 489 of 489 named (0 author symbols), 489 of 489 described (79 by rule, 405 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 814 of 814 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2360 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10252 of 10380 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 856128 |
| code | 32214 |
| data | 1636546 |
| padding | 77272 |
| unused | 1002496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| binddrivers | 11 | 659 | 39 |
| Off | 3 | 617 | 41 |
| type | 2 | 156 | 3 |
| path | 2 | 117 | 2 |
| PPMore | 2 | 267 | 8 |
| endcli | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| disk-validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:TOAD | 1 | 54 | 1 |
| 1 | 1 | 51 | 2 |
| 2 | 1 | 51 | 2 |
| 3 | 1 | 51 | 2 |
| 4 | 1 | 2 | 1 |
| 5 | 1 | 2 | 1 |
| 6 | 1 | 2 | 1 |
| 7 | 1 | 2 | 1 |
| 8 | 1 | 51 | 2 |
| 9 | 1 | 2 | 1 |
| icon.library | 10 | 1375 | 109 |
| explode.library | 1 | 519 | 31 |
| diskfont.library | 15 | 1040 | 76 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:game | 120 | 9 | 1 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:rinstall | 3 | 3875 | 110 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:bootblock | 1 | 54 | 1 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:produce.txt (raw code file, load address unknown) | 1 | 0 | 0 |
| Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:fame.dta (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:diffs.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:ter257.pix: high-entropy data file (7.97 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:sp299.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:arch.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:spacest.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth7.pix: high-entropy data file (7.77 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth5.pix: high-entropy data file (7.77 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth3.pix: high-entropy data file (7.77 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth1.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth8.pix: high-entropy data file (7.76 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:planet2.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth6.pix: high-entropy data file (7.76 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:docker.pix: high-entropy data file (7.83 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth4.pix: high-entropy data file (7.77 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:logo.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth2.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:sprites.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:custom.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:sp257.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:birth0.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:cback.pix: high-entropy data file (7.81 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt1m.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt3a.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg3.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt0m.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt2a.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt0a.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg1.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:castle3.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt0m.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt0a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpgd.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:castle1.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:sad.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg8.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpgb.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpgt2.pix: high-entropy data file (7.96 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt2m.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg6.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt2a.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg4.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt1a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt2m.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg2.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt1m.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:castle4.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:ngovt1a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpge.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:castle2.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpgc.pix: high-entropy data file (7.88 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:govt3a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg7.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpga.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpgt1.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf:iconpg5.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king09.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back2m.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king07.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:kink03.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:slam1.pix: high-entropy data file (7.96 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back1m.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king05.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back1a.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king13.pix: high-entropy data file (7.96 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king03.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king11.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king01.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back3a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king08.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:slam2.pix: high-entropy data file (7.82 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back2a.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king06.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king04.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:kink00.pix: high-entropy data file (7.96 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back0m.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king12.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:back0a.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king02.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king10.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf:king00.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:love2.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:hill.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:nuke1.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:invader2.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:invaders.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:wonders.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:spacest.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:wonders2.pix: high-entropy data file (7.85 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:pop.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:discovr1.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:riot.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:citypix3.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:citypix1.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:love1.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:invader3.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:adscreen.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:planet2.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:docker.pix: high-entropy data file (7.83 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:discovr2.pix: high-entropy data file (7.95 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:riot2.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:citypix2.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:produce.txt: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- civilization v855.01 (1992)(microprose)[tr sp](disk 4 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:fame.dta: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- civilization v855.01 (1992)(microprose)[tr sp](disk 2 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 2 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- civilization v855.01 (1992)(microprose)[tr sp](disk 3 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:c/PPMore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- civilization v855.01 (1992)(microprose)[tr sp](disk 1 of 4)/Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:game: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 2360 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### binddrivers

- `initialize_device_list`: Iterates through expansion devices to build a list of device names.
- `invoke_two_function_pointers`: Calls two function pointers located at fixed negative offsets from a base register.
- `execute_dos_sequence`: calls a function from a library, and if the result is non-zero, performs a series of operations involving setting values in a structure and calling further functions.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `process_expansion_devices`: Opens expansion.library and icon.library to obtain a device list, then iterates through the entries to temporarily disable each device and invoke a handler.
- `scan_table_for_marker`: Walks a table of short values using an index derived from the parameter and a length from a global variable.
- `accumulate_via_loop`: Repeatedly calls another routine with an accumulator and a parameter, then calls another routine with the current parameter and a constant, updating the parameter from the result until it becomes zero.
- `validate_and_normalize_input`: This routine validates and normalizes a pair of integer inputs through a sequence of calls to another routine and conditional branches.
- `find_linked_node_with_flag`: iterates through a linked list by calling another routine to retrieve the next node pointer.
- `follow_linked_list_until_match`: Walks a linked list by following pointers stored at a fixed offset from each node.
- `check_char_range_loop`: Examines a character pointer and a count.
- `parse_decimal_string`: Reads a null-terminated string and converts consecutive ASCII digits into an integer value.
- `clamp_value_by_sum`: the routine calls another routine with the first two arguments and stores the result.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `compare_two_strings`: Compares two byte strings byte by byte until a zero byte is found.
- `compute_checksum`: Computes a checksum by decrementing a 32-bit accumulator for each byte in a null-terminated string and returning the bitwise NOT of the result.
- `compute_checksum_2`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string, returning the bitwise NOT of the final value.
- `find_char_in_string`: Searches for the first occurrence of a specific character within a null-terminated string and returns a pointer to it, or a null pointer if the character is not found.
- `copy_c_string`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `calculate_modified_checksum`: calls two routines to obtain values, performs a comparison on a high-word value, and if a third routine returns zero, returns a value derived from the sum and difference of the retrieved components.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### Off

- `initialize_and_execute_logic`: Clears a block of memory, retrieves a value from a global variable, and executes a function pointer from a structure based on a bit test.
- `resolve_resource_metadata`: Retrieves a pointer from a global variable and extracts dimensions and offset data from a nested structure.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to its caller.
- `initialize_resource_buffer`: Allocates a memory buffer, initializes header fields, and performs several indirect calls to resolve resource pointers and execute callbacks.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_list`: Parses a string into individual tokens by splitting on whitespace or handling quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `append_string_to_buffer`: Finds the end of a destination string and copies a source string into that position up to a maximum length.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `resolve_window_icon`: Traverses nested structures to retrieve and assign icon and window data to a target structure.
- `get_string_pointer`: Returns a pointer to the start of a string by calculating the offset from the end of the string.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `execute_callback_and_routine`: Checks if a value at a specific offset is non-zero and, if so, calls the function pointed to by that value before calling another routine.
- `process_system_events`: Iterates through a list of items, executes various function pointers from a structure, and performs conditional calls based on global flags and structure offsets.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_clear_entry`: Checks if a value at a pointer is within a specific range and not zero, then clears that value.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using an offset derived from a base address stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_indirect_offset_60`: Performs an indirect call to a function pointer located at a specific offset within a structure.
- `call_indirect_function_via_offset`: Retrieves a function pointer from a nested table structure using two offsets and executes it.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a value in a register and executes it.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.

### type

- `parse_command_line_arguments`: Iterates through a string to identify space-separated tokens and processes them based on character comparisons.
- `call_indirect_through_pointer`: Calls a routine through a pointer held relative to a register and then returns.
- `call_indirect_from_table`: Calls a routine taken from a global pointer table at a fixed offset and then returns.

### path

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block_2`: Copies sixteen bytes from a source offset to a destination pointer.

### PPMore

- `parse_amiga_hunk_overlay`: Parses a data stream containing AmigaDOS hunk type codes to perform manual relocation and memory allocation.
- `decrement_and_loop`: Decrements a value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs complex bitwise shifts and logical operations on data in a loop, including memory modifications via software interrupts.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs complex bitwise shifts and logical operations on data in a loop based on values in registers.
- `modify_memory_via_registers`: Performs a software interrupt and then modifies memory locations using values stored in registers.

### endcli

- `populate_device_metadata_table`: Iterates through a linked list to find the maximum value in a sequence, then initializes a structured table with calculated offsets and data.
- `transfer_data_chunk`: Transfers a fixed number of words from a memory location to a destination.

### loadwb

- `parse_command_string`: Parses a command string by splitting it into tokens based on whitespace and handling quoted substrings.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, which leaves it unchanged, and allocates a 160-byte block using icon, intuition and workbench library calls.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_indirect_function_3`: Fetches a function pointer from a global table and executes it.
- `call_indirect_function_4`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_function_5`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_function_2`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_function_3`: Calls a function located at an offset from a global pointer using a library call.
- `execute_indirect_call`: Performs an indirect jump to a subroutine address stored within a global data structure.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `invoke_offset_pointer`: Retrieves a function pointer from a specific offset in a global table and executes it.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `verify_workbench_version`: Compares a string against a global constant to check for a specific version identifier.
- `write_constant_bytes`: Stores a sequence of specific byte values into a memory location starting at a given offset.
- `compare_memory_strings`: Compares two null-terminated strings byte by byte until a mismatch is found or the end of a string is reached.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_library_offset_offset`: Calls a function located at a specific offset within a library base retrieved from a global variable.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### disk-validator

- `update_display_buffer`: Calculates character offsets and updates a memory buffer with numeric values converted from integers.

### Ram-Handler

- `execute_indirect_function_4`: Fetches a function pointer from a location relative to a global variable and executes it.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_ramdrive_device_parameters`: Iterates through a list of device structures to locate a specific configuration, then copies a header and configuration table into a newly allocated memory block.
- `initialize_ramdrive_device_structure_2`: Copies a header and configuration table into a newly allocated memory block.
- `initialize_ramdrive_device_structure`: Calculates a memory size based on several values, allocates a block of memory, and populates it by copying data from a table.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, and links it into a global list.
- `execute_function_pointer`: Calls a function pointer located at a specific offset from a value in a register.
- `search_list_and_decrement_refcount`: Iterates through a linked list to find a node matching a specific bitmask, then executes a function pointer from a table.
- `traverse_linked_list_2`: Iterates through a linked list to find a node matching a specific bitmask and updates a global byte value.
- `nop_placeholder`: This routine is empty and performs no operations.
- `decrement_and_check_byte`: Decrements a byte at a specific offset within a structure and sets a global variable to a constant if the byte reaches zero.
- `store_word_to_offset`: stores a 32-bit value into a memory location calculated by adding an offset to a pointer in a register.
- `process_device_message`: Executes a function pointer from a global structure, looks up a value in a table based on a field in a message structure, stores the result back in the message, and conditionally calls another function pointer before a final cleanup routine.
- `return_constant_3`: returns a constant value of negative one.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `update_device_state`: Modifies internal counters, traverses a linked list, executes a function pointer, and replaces a null terminator in a string with a colon.
- `terminate_string_with_colon_2`: Iterates through a string until a null terminator is found, replaces it with a colon, and executes a function pointer.
- `terminate_string_with_colon`: iterates through a string until a null terminator is found, replaces it with a colon, and executes a function pointer.

### Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:TOAD

- `initialize_system_resources`: Opens libraries, configures window properties, performs a delay loop, and closes libraries.

### 1

- `return_error_constant`: Returns a constant value of all bits set.
- `return_maximum_value`: This routine returns a value consisting of all bits set to one.

### 2

- `return_all_ones`: The routine returns a value consisting of all bits set to one.
- `return_constant_value`: The routine returns a constant value of all bits set to one.

### 3

- `return_all_ones_2`: Returns a value consisting of all bits set to one.
- `return_maximum_value_2`: This routine returns a constant value consisting of all bits set to one.

### 4

- `return_all_ones_9`: This routine returns a value consisting entirely of set bits.

### 5

- `return_all_ones_3`: Returns a value where all bits are set to one.

### 6

- `return_maximum_value_3`: Returns a constant value representing all bits set to one.

### 7

- `return_all_ones_4`: Returns a 32-bit value consisting entirely of set bits.

### 8

- `return_all_ones_5`: The routine returns a 32-bit value consisting entirely of set bits.
- `return_constant_value_3`: This routine returns a constant value of all bits set to one.

### 9

- `return_all_ones_6`: Returns a value consisting entirely of set bits.

### icon.library

- `return_all_ones_7`: Library code that returns a value consisting of all bits set to one.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: Library code that either inserts a node into a linked list or sets a status bit in a structure.
- `set_status_bit`: Library code that performs a bitwise OR operation to set the eighth bit of a byte at a specific offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_icon_handler`: This library routine calls a function pointer from a structure, stores the return value in a structure offset, and updates two global variables if the return value is non-zero.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_self_call`: Library code that calls itself.
- `call_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `trigger_icon_initialization`: This library routine acts as a wrapper that invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `call_increment_param_and_recurse`: This library routine calls another routine with the first argument unchanged and the second argument increased by a constant, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `dispatch_icon_handler`: This library routine acts as a wrapper that calls another routine and then returns.
- `invoke_handler_pointer`: This library routine retrieves a function pointer from a data structure and executes it.
- `call_other_routine_2`: This library code calls another routine and then returns.
- `call_attribute_iterator`: This routine calls another routine and then returns.
- `call_numeric_parser`: This library routine calls another routine and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `traverse_and_cleanup_linked_list`: Recursively traverses a linked structure and performs conditional cleanup of nodes.
- `update_icon_structure`: This library routine attempts to validate a resource, and if successful, copies a block of data and several individual fields from one structure to another.
- `copy_structure_data`: Library code that copies a block of data and several individual fields from one structure to another.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `rebuild_icon_linked_list`: Updates an icon structure by reallocating memory for its associated data and rebuilding its internal linked list of child elements or related objects.
- `update_icon_state_and_list`: Updates an icon structure's internal state and manages its relationship within a linked list or collection.
- `update_icon_list_2`: This library routine updates a list of structures by calling various functions to validate states, propagate changes, and iterate through nodes to update integer values within the structures.
- `validate_icon_state`: Library code that executes a series of function pointers in a loop to verify a state and updates a global variable.
- `call_and_return_zero`: Library code that calls another routine and always returns zero.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `execute_icon_callbacks_2`: Iterates through a series of callback pointers and data structures stored within an icon structure, invoking them sequentially.
- `execute_icon_callbacks`: Iterates through a series of callback pointers stored within an icon structure and executes them.
- `dispatch_icon_callbacks`: Library code that iterates through a table of pointers and executes specific callback routines based on structure offsets.
- `iterate_icon_table`: The library code walks a null-terminated table of pointers, calling a routine for each entry and updating a counter.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: Library code that traverses a linked list structure by following pointers stored at an offset from the current node.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `validate_and_reinitialize_resource`: This routine attempts to acquire a resource and, if successful, performs a conditional reinitialization or update based on the success of subsequent operations.
- `update_indexed_table_2`: This library routine decrements a counter through a pointer and uses the result to store two values into a table at a calculated offset, then returns zero.
- `return_zero_value`: This routine is an empty function that returns zero to the caller.
- `update_indexed_table`: The library code decrements a counter through a pointer and uses the result to store two values into a table at a calculated offset.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `reallocate_icon_data_buffer`: This routine attempts to reallocate a memory buffer for an icon, attempting to resize it or replace it with a new allocation.
- `empty_library_routine`: This library routine is empty.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `incremental_recursive_call`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_string_assignment`: This library code iterates through a list of pointers to find a string containing an equals sign character and returns a pointer to the value following that character.
- `find_string_delimiter`: This library routine compares memory blocks to find the position of an equals sign character within a sequence of pointers.
- `find_string_delimiter_2`: Library code that searches through a list of pointers to find a string containing an equals sign character.
- `iterate_icon_attribute_list`: Iterates through a linked list or sequence of attributes by performing arithmetic comparisons and conditional checks.
- `validate_sequence_with_conditional_loop`: Library code that executes a loop performing arithmetic comparisons and conditional calls to other routines.
- `execute_conditional_loop_2`: This library routine performs a loop involving arithmetic operations and conditional calls to another routine.
- `execute_conditional_loop`: Library code that performs a loop involving arithmetic operations and conditional calls to another routine.
- `validate_conditional_loop`: This library code performs arithmetic comparisons and conditional checks within a loop to determine if a specific sequence or condition is met.
- `parse_numeric_string_3`: This library routine parses a string to extract a numeric value, performs memory comparisons, and copies the resulting data into a destination structure.
- `parse_numeric_string_2`: This library code compares memory blocks and then iterates through a character string to build an integer value.
- `accumulate_integer_from_string`: This library code iterates through a character string to build an integer value by multiplying the current total by ten and adding the value of each character.
- `parse_numeric_string`: Library code that converts a string of numeric characters into an integer value.
- `offset_input_value`: Library code that subtracts a constant from an input value if it falls within a specific range, otherwise returns negative one.
- `return_negative_one_stub`: The routine returns the constant value of all bits set to one.
- `call_indirect_offset_a`: The library code calls a routine through a pointer located at a specific offset from a global base address.
- `call_indirect_offset_b`: The library code calls a routine through a pointer located at a different specific offset from a global base address.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `initialize_library_structure`: Library code that calls a function to obtain a pointer and then populates a structure with a specific value.
- `return_zero`: Library code that returns zero.
- `compare_and_reset`: Library code that calls another routine and returns zero if the returned value does not match a passed parameter.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `verify_library_result`: Library code that calls another routine and returns zero if the result does not match a provided value.
- `return_void`: Library code that performs an empty return.
- `perform_division_math`: The routine performs complex arithmetic operations involving bit shifts and modulo operations to calculate a result based on two input values.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: Library code that invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: The library code stores a single byte from a register into a memory location pointed to by another register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_character_in_string`: The library code iterates through a string until it finds a specific character or reaches the end of the string.
- `return_zero_3`: This library routine performs no operations and returns a value of zero to the caller.
- `copy_c_string_2`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_c_string_3`: Library code that finds the end of a null-terminated string and then copies bytes from a source pointer to a destination pointer.
- `compare_memory_blocks`: Library code that compares two memory blocks byte by byte for a specified length.
- `return_one_stub`: Library code that returns one.
- `iterate_string_pointer`: This library routine iterates through a string and decrements a counter until a null terminator is reached or the counter hits zero.
- `execute_indirect_call_2`: This routine performs an indirect jump to an address stored in a memory location.
- `copy_and_pad_string`: Library code that copies bytes from a source to a destination for a specified count and then pads the remaining destination bytes with null terminators.
- `return_from_subroutine_2`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `verify_status_loop_2`: This library routine repeatedly calls another routine and checks its return value, returning one if a specific value in a structure is zero.
- `verify_status_loop`: The library code repeatedly calls another routine and checks its return value, returning one if a specific value in a structure is zero.
- `return_zero_constant`: This routine is an empty function that returns a zero value to the caller.
- `allocate_icon_structure`: Library code that allocates a structure and populates its members through a series of conditional library calls.
- `return_error_code`: Library code that returns a value of all bits set.
- `update_icon_list`: This library code iterates through a linked list of structures, performing calculations and updating offsets within each node.
- `traverse_icon_list`: Library code that iterates through a linked list of structures, performing checks and updating pointers.
- `update_icon_list_node`: Iterates through a linked list of icon structures, updating specific offsets within each node based on values returned from subsequent routine calls.
- `return_all_ones_8`: Library code that returns a constant value of negative one.

### explode.library

- `return_negative_one`: The library code returns a value of all bits set to one.
- `process_explosion_data_2`: This library code extracts values from a data structure and conditionally executes function pointers based on the contents of the retrieved data.
- `process_explosion_data`: Library code that extracts values from a data structure and conditionally executes function pointers based on the contents.
- `process_data_structure_2`: This library routine copies a byte to a destination, checks a value at a pointer, and conditionally executes a function pointer from a table.
- `process_data_structure`: Library code that copies a byte to a destination, checks a value at a pointer, and conditionally executes a function pointer from a table.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `restore_jump_and_release_library_2`: This library routine checks whether a patched jump is still in place and, if so, restores the saved words and clears the processor cache.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `restore_jump_and_release_library`: Library code that checks whether a patched jump is still in place and, if so, restores the saved words and clears the processor cache.
- `set_bit_in_byte`: The library code sets the third bit of a byte located at an offset from a pointer held in a register.
- `verify_library_identity`: Checks if a memory block pointed to by a retrieved value contains a specific four-character string sequence.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `verify_library_name`: The library code checks if a memory block contains a specific string representing a library name by comparing multiple words against constant values.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_count_then_clear_cache`: Library code that decrements a 16-bit count field in a structure, calls one unnamed routine, and clears the processor cache when the library version is above a threshold.
- `reconstruct_pointer_table_2`: Rebuilds a table of memory addresses by decompressing a bitstream of variable-length integers.
- `reconstruct_pointer_table`: Iterates through a compressed data stream to rebuild a table of memory addresses.
- `copy_memory_block_optimized`: Performs a high-speed memory block transfer by copying data in large word or longword chunks.
- `copy_memory_block_4`: Performs a memory block transfer by iterating through a source range and writing values to a destination address.
- `copy_memory_block_3`: Performs a high-speed memory block transfer using an unrolled loop to copy multiple words at a time.
- `decompress_bitstream_data_2`: Extracts variable-length integers from a bitstream by performing successive bit shifts and carries.
- `extract_variable_bit_integer`: Reads a variable-length integer from a bitstream by iteratively shifting bits and checking the carry flag.
- `decode_bitstream_structure`: Decodes a variable-length bit-encoded data structure by performing bit-shifting and carry-flag manipulation to extract indices and lengths.
- `decode_bitstream_structure_2`: Decodes a variable-length bit-encoded data structure by performing bit-shifts and pointer increments based on carry flags.
- `decode_bitstream_structure_3`: Iterates through a bit-encoded data stream by performing bit-shifts and carry-flag manipulations to extract variable-length indices and lengths.
- `decode_bitstream_structure_4`: Iterates through a bit-encoded data stream by performing successive left shifts and carry-flag checks to reconstruct multi-bit values.
- `traverse_variable_length_structure`: Iterates through a complex, bit-encoded data structure by decoding indices and lengths through bit-shifting and carry-flag manipulation.
- `remove_string_element_2`: Locates a specific entry within a variable-length data structure using bit-shifting to decode an index and then shifts the subsequent bytes to close the gap.
- `remove_string_element`: Locates a specific entry within a variable-length data structure using bit-shifting to decode an index and then shifts the subsequent bytes to fill the gap.
- `decompress_bitstream_data`: Decompresses a bitstream by extracting variable-length integers and performing memory shifts or copies.
- `free_counted_memory_blocks`: Frees a counted list of memory blocks through the exec library's FreeMem function, working down from a block count stored in the structure, then clears the final link word.

### diskfont.library

- `update_font_handle_state`: Calls a routine via a table entry and, if successful, stores a value from a register into a specific offset of the returned pointer before calling another routine via a table entry.
- `check_and_set_font_handle`: This library code calls a routine via a table entry using the library base.
- `close_library_and_free_memory`: This library routine attempts to close a library and free a memory block by calling functions via offsets from a base pointer.
- `close_and_free_library`: This library routine calls functions to close a library and free a memory block before returning.
- `cleanup_library_resources`: This library routine iterates through a linked list to remove elements based on a status check, potentially calling another routine and updating a counter or setting a flag.
- `manage_library_resource`: This library routine checks specific status flags and, if certain conditions are met, calls another routine and manipulates a linked list structure.
- `set_high_bit`: Library code that sets the most significant bit of a byte at a fixed offset from a pointer.
- `empty_routine_2`: Library code that is empty.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_counter`: This library routine decrements a value at a specific offset within a structure and calls another routine if the value reaches zero and a bitwise flag is set.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.
- `populate_font_glyph_list`: Iterates through a linked list of font data to extract character metadata and string content.
- `populate_string_buffer_from_list`: Iterates through a linked list of data structures to calculate string lengths and copy character data into a destination buffer.
- `parse_font_data_stream`: Iterates through a linked list of data structures to extract and copy string segments into a destination buffer.
- `parse_font_glyph_metadata`: Iterates through a data stream to extract character dimensions and attributes, populating a list of font glyph structures.
- `parse_font_metadata_block_2`: Iterates through a structured data block to extract font character attributes and dimensions.
- `parse_font_metadata_block`: Iterates through a structured data block to extract font character attributes and dimensions.
- `load_font_data_from_disk`: Reads font data from a disk source into a memory buffer and parses a structured data block.
- `parse_font_metadata_block_3`: Iterates through a structured data block to extract font character attributes and dimensions.
- `return_zero_stub_2`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `calculate_glyph_index_offset`: Computes a signed index value by calculating the difference between two short integers and applying a bitwise search through a table.
- `compute_font_offset`: This library code performs a bitwise search through a table to compute a value based on a series of shifts and subtractions.
- `calculate_font_offset_2`: As library code, this routine performs a bitwise search through a table to compute a value based on a series of shifts and subtractions.
- `calculate_font_offset`: As part of library code, this routine performs a bitwise search through a table to compute a value based on a series of shifts and subtractions.
- `return_constant_value_2`: Library code that returns a constant value without using any arguments or registers.
- `update_glyph_metadata`: Iterates through a font glyph table to find a specific character and updates its associated metadata, such as bitmask flags and memory offsets.
- `rebuild_font_glyph_table`: Iterates through loaded font data to identify and update glyph metadata.
- `load_and_initialize_font_data`: Opens a font file from disk, allocates memory for the glyph data, and reads the file content into the new buffer.
- `find_max_glyph_index`: Iterates through a font glyph table to identify the index of the entry with the highest value based on a comparison criteria.
- `reorganize_font_glyph_table`: Iterates through a font data table to find and sort glyphs based on specific criteria.
- `update_font_glyph_metadata`: Iterates through a table of font data to find a specific glyph and updates its associated metadata, including incrementing a counter and modifying status flags within the glyph structure.
- `search_font_glyph`: Iterates through a table of font data to find a glyph matching a specific criteria.
- `parse_disk_font_data`: Iterates through a file to identify and extract font character definitions into a structured memory table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_exec_forbid`: Library code that calls a function pointer retrieved from a table to forbid processor interrupts or multitasking.
- `call_exec_permit`: Library code that calls a function pointer retrieved from a table to permit processor interrupts or multitasking.
- `call_exec_allocmem`: Library code that calls a function pointer retrieved from a table to allocate memory.
- `call_indirect_function`: As part of a library, this routine loads a function pointer from an offset relative to a value stored in a global variable and executes it.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_dos_lock`: Library code that calls a function pointer retrieved from a table to perform a file lock operation.
- `call_dos_unlock`: Library code that calls a function pointer retrieved from a table to perform a file unlock operation.
- `call_dos_examine`: Library code that calls a function pointer retrieved from a table to perform a file examination operation.
- `call_dos_exnext`: Library code that calls a function pointer retrieved from a table to perform a file enumeration next operation.
- `call_dos_currentdir`: Library code that calls a function pointer retrieved from a table to retrieve the current directory.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_dos_loadseg`: Library code that calls a function pointer retrieved from a table to load a segment.
- `call_dos_unloadseg`: Library code that calls a function pointer retrieved from a table to unload a segment.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_to_null_terminator`: This library routine increments a pointer until it reaches a zero byte and then calls another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: This library routine calculates the length of a zero-terminated string by incrementing a counter for each byte processed.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_null_terminated_strings_4`: A library routine that compares two null-terminated strings and returns a value indicating their lexicographical relationship.
- `compare_consecutive_bytes`: This library routine compares adjacent bytes in a sequence and returns a value based on whether the sequence is null-terminated or if a specific byte comparison condition is met.
- `compare_null_terminated_strings_3`: This library routine compares two null-terminated strings and returns one if the second string is lexicographically smaller than the first, or zero otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_matching_byte_2`: This library routine searches forward through a null-terminated string for a specific byte and returns a pointer to its first occurrence.
- `find_byte_in_sequence`: This library routine searches through a sequence of bytes until it finds a match for a target byte or reaches a null terminator.
- `find_matching_byte`: This library routine searches forward through a sequence for a byte that matches the initial byte, returning a pointer to the match or zero if the end of the sequence is reached.
- `search_backward_for_byte_3`: A library routine that searches backward through a memory sequence to find the last occurrence of a specific byte.
- `search_backward_for_byte`: This library routine searches backward through a sequence to find a byte that matches a given value.
- `search_backward_for_byte_2`: This library routine searches backward through a memory sequence to find the last occurrence of a specific byte.
- `search_font_character_markers`: Searches backward through a memory sequence for specific byte markers.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_another_routine`: This library routine acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:game

- `initialize_program_loop`: This routine repeatedly calls a function from a global variable until it returns zero, then attempts to call another function using the result.

### Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:rinstall

- `initialize_reinstallation_environment`: Clears a memory block and initializes several global variables before calling a subroutine via a pointer in a global structure.
- `cleanup_system_resources`: This routine executes a series of function pointers and library calls to release memory, close files, and close libraries.
- `call_indirect_function_2`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `return_from_loop`: This routine is an empty function that performs no operations before returning.
- `traverse_linked_list_3`: Recursively validates and traverses a linked list while retrieving table entries and executing functions via pointers from a global structure.
- `process_string_buffer`: Iterates through a null-terminated string to copy characters into a buffer while decrementing a counter and calling another routine when the counter reaches zero.
- `parse_configuration_string`: Parses a string into a list of tokens, handling quoted substrings and expanding special escape sequences.
- `process_installation_data`: Iterates through a data structure to write character values and formatted strings into a buffer.
- `validate_and_initialize_structure`: Checks if a structure's field is zero or if a specific bit is set, then calls another routine to update the structure's pointers and fields.
- `update_display_buffer_state`: The routine processes a display buffer structure and an index to update internal flags, counters, and status bits.
- `traverse_callback_table`: Iterates backwards through a table of pointers and executes a routine for each valid entry.
- `modify_linked_list_elements`: Writes specific values to memory locations pointed to by a pointer retrieved from a global offset and then calls another routine.
- `convert_to_lowercase`: If a character is within a certain range, it subtracts a constant value from it.
- `transform_string_by_table`: Iterates through a null-terminated string and modifies each byte based on a bitwise check against a table.
- `parse_file_path`: Splits a source string into its constituent components, such as drive, directory, filename, and extension.
- `copy_n_terminated_bytes`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, stopping early if a counter reaches zero and returning the original destination pointer.
- `copy_string_with_limit`: Copies a certain number of bytes from a source string to a destination string, ensuring the destination is null-terminated.
- `build_path_string`: Constructs a path string by concatenating multiple segments and inserting specific characters like colons or dots.
- `transform_string_to_uppercase`: Iterates through a null-terminated string and adds a constant value to each byte if a specific bit is set in a table lookup.
- `calculate_string_length`: Calculates the number of bytes in a null-terminated string by finding the distance between the start and the terminating zero byte.
- `copy_null_terminated_string_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compare_c_strings`: Compares two null-terminated strings and returns a value indicating if the first is greater, smaller, or equal.
- `copy_string_to_buffer_2`: Finds the end of a destination buffer by searching for a null terminator and then copies a null-terminated string into that position.
- `fill_pointer_table`: Fills a table of pointers with a provided string address and terminates the list with a null pointer.
- `copy_string`: Copies a null-terminated string from a source pointer to a destination pointer.
- `find_substring`: Checks if a string contains a specific character by iterating through the source string.
- `search_string_content`: Checks if a specific character exists within a provided string.
- `match_pattern`: Compares a string against a pattern that supports wildcard characters for matching sequences or single characters.
- `extract_drive_prefix`: Searches a string backward for a colon or slash to identify and extract a drive or path prefix into a destination buffer.
- `copy_and_null_terminate`: This routine copies a block of memory from a source to a destination and sets the final byte of the destination range to zero.
- `copy_and_clear_memory`: Copies a block of memory from a source to a destination and then sets a byte at a specific location to zero.
- `find_drive_prefix`: Searches backward through a string for a colon or slash to identify and extract a drive or path prefix.
- `clear_global_value`: Sets the value at a specific memory location to zero.
- `extract_filename`: Extracts a portion of a string by searching backwards for a directory separator and copying up to a maximum number of characters.
- `manage_memory_block`: Manages a linked list of memory blocks to allocate, merge, or split segments based on a requested size.
- `execute_nested_operation`: Calls a routine to retrieve a pointer, then uses that pointer to call another routine while checking a global status flag.
- `process_conditional_logic`: Iterates through a table to find an entry, performs bitwise manipulations on input parameters, and executes a sequence of conditional function calls through various pointers.
- `create_directory_wrapper`: Attempts to create a directory and handles the resulting error or success by updating global state.
- `free_linked_list`: Iterates through a linked list and calls a function through a pointer for each node before clearing the head and tail pointers in a structure.
- `manage_linked_list`: Clears a global pointer to a list and then allocates a new node containing a pointer to the provided value.
- `validate_and_traverse_linked_list`: Checks if a pointer is null before recursively calling another routine with a value retrieved from the current node.
- `allocate_and_link_node`: Allocates memory for a new structure, initializes its fields with provided values, and inserts it into several global linked lists.
- `allocate_memory_segment`: This routine searches a linked list for a memory block of sufficient size, splitting it if necessary, or allocates a new block if no suitable segment is found.
- `call_allocate_segment`: Calls another routine with a single parameter.
- `process_file_path_data`: Parses and constructs file path strings, verifies filename matches against wildcards, and manages memory blocks and file locking.
- `update_linked_list_pointer`: Updates a global variable with a value derived from an input parameter after potentially calling another routine.
- `traverse_linked_list_4`: Recursively traverses a linked list by following pointers.
- `execute_dos_function`: Calls a function via a pointer retrieved from a global structure and handles error states by updating global status values.
- `calculate_offset_value`: Calls a function via a pointer and performs arithmetic on a parameter based on the returned value and an input parameter.
- `execute_read_operation`: Calls a function via a pointer retrieved from a global structure and updates global status values if the result is negative one.
- `execute_open_operation`: Calls a function via a pointer and updates global status values if the result is zero.
- `verify_and_copy_file_data`: Checks if a specified file path matches a stored reference and validates the filename against wildcard characters.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d7.
- `copy_memory_block`: Allocates a temporary memory segment, copies data from a provided pointer to the segment, and then copies it back to the original destination.
- `process_dos_request`: Iterates through a loop performing operations involving pattern matching and system calls until a specific condition is met.
- `copy_data_with_allocation`: Allocates a memory segment and copies data from that segment into a provided pointer after performing several checks.
- `execute_conditional_dos_calls`: Performs a sequence of function calls through pointers retrieved from a global structure, updating status values upon failure.
- `execute_cleanup_operations`: Calls multiple functions through pointers retrieved from a global structure and handles error states by updating global status values.
- `execute_callback`: Checks a value at an offset and then calls a function pointer retrieved from a table.
- `initialize_intuition_buffer`: Copies a string from a global location into a local stack buffer and then calls another routine with several parameters.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point`: Performs a division of two 32-bit values using a normalized fixed-point method, handling sign and scaling.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `process_string_and_system_call`: Copies a string to a local buffer, checks a system status, and performs an indirect call if a condition is met.
- `initialize_system_io`: Copies a block of memory, checks a system status via a library function, and attempts to perform an operation using a global pointer.
- `initialize_library_function`: This routine calls a function via a pointer stored in a global variable and uses the result to determine whether to call another routine with specific parameters.
- `perform_conditional_test`: Stores a calculated value into a global variable and calls another routine, returning zero or all ones based on the result.
- `return_zero_constant_2`: The routine returns a constant zero value without performing any other operations or memory accesses.
- `process_table_entry`: This routine retrieves a pointer from a table, checks a bit flag within the pointed-to structure, and executes a callback before clearing the entry.
- `get_table_entry_address`: Validates an index against a table size and returns the address of the corresponding entry in a table, or a specific constant if invalid.
- `clear_signal_callback`: Checks a bitmask in a global value and, if a specific condition is met, clears a function pointer and calls another routine.
- `zero_memory_block`: Calls another routine to obtain a pointer and then fills a specified number of bytes starting at that pointer with zeros.
- `match_pattern_string`: Compares two strings using wildcard and special character matching logic.
- `compare_strings_case_insensitive`: Compares two strings byte by byte by converting each character to lowercase before comparison.
- `process_installation_data_sequence`: Iterates through a list of data to perform string transformations, directory management, and memory operations.
- `update_status_fields`: Clears several global memory locations with a specific byte value and calls another routine multiple times using different offsets.
- `process_list_data`: Checks a stack pointer, then accesses a list to write specific values to memory and performs a character transformation on a retrieved value.
- `evaluate_list_node`: Validates a stack pointer before accessing a list to write values and returning a modified character value based on a range check.
- `fetch_formatted_value`: Checks a pointer against a stack address, calls a function via a table, and performs character arithmetic on a retrieved value.
- `return_transformed_char`: Checks a pointer, calls a function through a table, and returns a value that may undergo character-based offset adjustment.
- `poll_status_flag`: Repeatedly calls another routine in a loop until a non-zero value is returned.
- `update_icon_resource_metadata`: Iterates through a linked list of data structures to perform character transformations and update global pointers.
- `iterate_and_validate_icon_attributes`: Iterates through a list of data structures, performing a series of conditional checks and string comparisons on specific offsets.
- `process_file_data_list`: Validates file names and copies data into a temporary buffer before iterating through a list to append strings to a destination.
- `call_function_via_pointer`: Calls a function through a pointer stored in a global variable.
- `validate_and_copy_file_data`: Compares a provided filename against stored reference strings and validates it using wildcard characters.
- `clear_memory_block`: Retrieves a pointer from a global offset and fills a specified number of bytes starting at that pointer with zeros.
- `execute_open_operation_2`: Checks a global variable, calls a function through a pointer, and if the result is zero, calls another function through a pointer and updates global status values.
- `process_table_entry_2`: Retrieves a pointer from a table, checks a bit flag within the pointed-to structure, and executes a callback before clearing the pointer.
- `execute_callback_if_set`: Checks if a value at a specific offset is non-zero and then calls a function pointer retrieved from a table.
- `compare_null_terminated_strings_2`: Compares two null-terminated strings byte by byte.
- `calculate_string_length_2`: Iterates through a null-terminated character sequence to determine the total number of bytes before the terminator.
- `execute_nested_operation_2`: The routine calls another routine to retrieve a pointer and then uses that pointer to call a third routine while checking a global status flag.
- `copy_data_via_temporary_buffer`: Allocates a temporary memory segment to hold a block of data, copies the input data into this segment, and then copies the data back from the segment to the original destination.
- `transform_string_by_table_2`: Iterates through a null-terminated string and modifies each byte by subtracting thirty-two if a bitwise check against a table returns a specific bit.
- `update_global_pointer`: Updates a global variable with a value derived from the input parameter minus four.
- `handle_directory_creation`: Attempts to create a directory using a function pointer from a structure and updates a global variable with the result or resets state on failure.
- `append_string_with_bounds_check`: Appends a character string to a destination buffer while performing length validation.
- `copy_null_terminated_string_3`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `copy_string_to_buffer_3`: Iterates through a null-terminated string to copy characters into a destination buffer while managing a counter and a secondary pointer.
- `extract_path_prefix`: Scans a string backward to find the last occurrence of a colon or forward slash.
- `append_null_terminated_string`: Locates the null terminator of a destination buffer and copies a source string starting at that position.
- `traverse_and_process_list`: Iterates through a linked list and checks specific bit flags and value differences within each node.
- `extract_filename_from_path`: Searches backwards through a string for a directory separator and copies the subsequent characters into a new buffer up to a maximum length.
- `process_conditional_logic_2`: The routine iterates through a table to find an entry and performs bitwise manipulations on input parameters to execute a sequence of conditional operations.
- `process_list_recursively`: Recursively processes a linked list by calling another routine and performing conditional checks on retrieved values and a global variable.
- `populate_string_pointer_array`: Iterates through a list of strings, storing the address of each string into a destination table.
- `multiply_fixed_point_values`: Calculates the product of two 32-bit fixed-point values using their high and low 16-bit components.
- `copy_string_including_terminator`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered.

### Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Civilization v855.01 (1992)(MicroProse)[tr Sp](Disk 4 of 4).adf:bootblock

- `check_library_open`: Calls a function through a pointer to check for success and returns zero or negative one based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state_and_restore`: Calls two consecutive functions with specific parameters, calls a third function with a pointer to a data structure, updates two global variables, and restores all registers from a table before jumping to a function pointer.
- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.
