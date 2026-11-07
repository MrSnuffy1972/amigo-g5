# Game 2108 (ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02108). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (926128 top-level bytes in 147 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 104 of 104 units identical (bytes and relocations), 125 raw-word lines |
| Functions named and described | pass, 219 of 219 named (0 author symbols), 219 of 219 described (31 by rule, 185 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1849 of 1853 (99.78%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6609 of 6614 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 110247 |
| code | 19046 |
| data | 770499 |
| padding | 13536 |
| unused | 12800 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:installu6 | 2 | 267 | 8 |
| endcli | 2 | 117 | 2 |
| disk-validator | 1 | 386 | 1 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:INSTALL.HELP.INFO | 2 | 24 | 4 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:shut.txt | 2 | 267 | 8 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:pause2 | 2 | 3 | 1 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:FAST_EDDIE.RULES¸Ø | 2 | 24 | 4 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:vmk | 1 | 412 | 6 |
| Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 59 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 744 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 419 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 909 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1356 | 35 |

## Cases handled

- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:INTRO_3.NEW: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:intro.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:paper.bmp: high-entropy data file (7.93 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:mainmenu.new: high-entropy data file (7.64 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Ba.MOD: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:INTRO_1.NEW: high-entropy data file (7.85 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Ga.MOD: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Fa.MOD: high-entropy data file (7.91 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:u: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ultcli: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:intro_1a.new: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Da.MOD: high-entropy data file (7.84 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:intro_2.new: high-entropy data file (7.65 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:gypsy.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ultima6: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Aa.MOD: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:game: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:vellum1.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:intro_2a.new: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:title.new: high-entropy data file (7.77 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:ULTIMA_Ea.MOD: high-entropy data file (7.72 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:intro_1b.new: high-entropy data file (7.75 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:blocks.new: high-entropy data file (7.90 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 34 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 350 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 244 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 427 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 246 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- ultima vi - the false prophet (1992)(mindscape)[cr skr](disk 1 of 3)(install)/Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:installu6: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:installu6

- `unpack_relocatable_executable`: Parses a custom compressed data stream containing hunk-based relocation and code segments.
- `decrement_loop_counter_2`: Decrements a value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bit_stream`: Performs bitwise extraction and data movement from a source buffer into a target memory area based on a control value.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Iterates through a loop to perform bitwise shifts and extractions from a source buffer into a target value.
- `execute_system_trap`: Loads a word from a memory location into a data register, modifies a register with a bitwise operation, and triggers a software interrupt.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `branch_with_register`: Moves a long value from one register to another and performs a short relative branch.

### disk-validator

- `format_display_string`: Converts a numerical value into a character representation and stores it in a buffer.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:INSTALL.HELP.INFO

- `call_library_entry`: Invokes a routine and checks the zero flag; if not set, it calls a second routine and executes a function pointer from a table before calling a third routine.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library and stores the resulting library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:shut.txt

- `unpack_hunk_data`: Parses a custom data stream containing hunk type codes to allocate memory and relocate data blocks.
- `decrement_and_loop_2`: Decrements a 16-bit integer value until it reaches zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bit_stream_2`: Iterates through a buffer to perform bitwise shifts and data copying based on a control value.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: Subtracts a word from one register and stores the result in another.
- `extract_variable_bit_stream`: Extracts a sequence of variable-length bit fields from a source buffer and reconstructs them into a target destination.
- `execute_trap_sequence`: Moves a word from a memory location pointed to by a register, performs a bitwise OR operation, executes a software interrupt, and then performs a bitwise OR operation between two registers.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:pause2

- `wait_for_disk_status`: Polls a global variable until a specific bit is cleared.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:FAST_EDDIE.RULES¸Ø

- `execute_conditional_callback`: Invokes a routine and checks the zero flag; if the flag is not set, it calls a second routine, executes a function pointer retrieved from a global table, and then calls a third routine before returning.
- `open_dos_library_2`: Calls a function pointer stored in a global variable to open the DOS library and stores the resulting handle in a global variable.
- `call_close_library_2`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:vmk

- `verify_anti_piracy_measures`: Scans system memory and checks specific data structures for signatures associated with known cracking tools or unauthorized software.
- `call_two_functions`: Calls two different functions using pointers retrieved from a global variable.
- `format_hex_string`: Converts a value into a hexadecimal string representation and copies it into a global buffer.
- `update_intuition_state`: Checks a library return value and modifies several fields in a global structure, including bitwise inversion of a word.
- `copy_string_data`: Copies a sequence of bytes from a source pointer to a destination global variable until a terminator is reached.
- `call_global_function`: Calls a function located at a global variable address.

### Ultima VI - The False Prophet (1992)(Mindscape)[cr SKR](Disk 1 of 3)(Install).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_amigados_hunk_table`: Iterates through a memory structure to identify and process AmigaDOS hunk types, specifically handling executable and relocation data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_flag`: Calls another routine with a calculated offset and clears a value through a pointer if a specific condition in a nested table is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call`: Fills a block of memory with zeros using a loop and then executes two function calls via pointers stored in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `verify_and_reset_pointer`: Executes a routine with a scaled offset and sets a value to zero through a pointer based on a check of a nested table value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_buffer_and_dispatch`: Clears a memory range with zeros and performs two indirect function calls using values retrieved from a structure and a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_engine_state`: Clears a memory block and populates a structure with pointers and values retrieved from a global state object.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_device_interface`: Attempts to initialize a hardware device or resource by repeatedly calling a function through a pointer in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_state_structures`: This routine updates various internal state structures by retrieving values from a primary data object and storing them into a secondary management structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource_handles`: Repeatedly calls a function via a pointer in a structure to retrieve resource identifiers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_engine_subsystems`: Repeatedly calls two internal engine functions to populate a data structure with values retrieved from a global state object.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_installation_data_structures`: Repeatedly calls two internal functions to populate a structure with values derived from a global object.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_properties`: Iterates through a collection of data structures to update specific fields by calling external subroutines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_structures`: Initializes several global data structures by calling internal subroutines and updating pointers and values within a large state block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_buffers`: Iterates through a list of data structures to update specific fields by calling subroutines and writing results to a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_engine_state_structures`: Iteratively clears specific fields within a series of data structures by checking for a constant marker value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_state_structures`: Updates various internal state structures by retrieving values from a data table and writing them to specific offsets in a global object.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_state_structures_2`: Updates various global state variables and pointers by retrieving values from an offset in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_resource_metadata`: Updates a structure with values retrieved from a pointer-based table and performs conditional memory writes based on the contents of a linked list or array.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_resource_metadata`: Iterates through a data structure to identify and extract specific markers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_amigados_hunk_types`: Iterates through a memory segment to identify and process AmigaDOS hunk types.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scrub_memory_blocks`: Iterates through a data structure to identify and zero out specific memory segments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_resource_metadata_2`: Iterates through a structured data block to identify and extract specific metadata fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `patch_data_stream`: Iterates through a data stream to locate specific marker values and replaces them with a value stored in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_string_and_buffer_pointers`: Iterates through a list of data structures to search for specific markers and updates several pointers and length values within a global state structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_resource_table_entries`: Iterates through a data structure to find specific markers and updates associated values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_string_and_buffer_pointers`: Iterates through a list of data structures to search for specific markers and updates several pointers and length values within a global state structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_data_structure`: Iterates through a memory structure to locate specific marker values and extract string data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_string_and_buffer_pointers`: Iterates through a list of data structures to search for specific markers and updates several pointers and length values within a global state structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_data_structures`: Iterates through memory to locate null-terminated strings and updates internal data structures with their lengths and pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_non_zero_word`: Iterates through a memory range to find a non-zero word and performs a single word copy if a zero word is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_and_allocate`: Extracts a string from a data structure and calculates its length, then performs a memory allocation or pointer update based on the string's properties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_non_zero_word`: Iterates through a memory range to find a non-zero word and performs a single word copy if a zero word is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_buffer`: Calculates an offset from a pointer, updates a global structure with a pointer and a length, and enters a complex loop based on the result of a function call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `skip_null_bytes`: Increments a pointer and checks bytes in a loop, adjusting the pointer by a fixed amount when a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_buffer_logic`: Retrieves a pointer from a nested structure, updates a global variable with a calculated address and size, and processes data through a loop if a function returns non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_memory_bytes`: Decrements a counter and tests bytes at a pointer, incrementing the pointer by three if a non-zero byte is found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_amigados_hunks`: Iterates through a memory structure to identify and process AmigaDOS hunk types.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_amigados_hunk_types_2`: Iterates through a memory segment to identify and process AmigaDOS hunk type codes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_amiga_hunks`: Iterates through a list of memory hunks to perform relocation and data copying.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_amigados_hunks`: Parses a list of memory hunk descriptors to manually relocate and copy data segments into new memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_hunk_data_structures`: Iterates through a sequence of data structures, copying memory blocks and updating pointers based on values found within the structures.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_relocation_hunks`: Iterates through a list of relocation data to adjust memory addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_amiga_hunk_data`: Parses a list of memory hunk descriptors to allocate and populate new memory segments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_amiga_hunk_list`: Iterates through a list of memory segments to identify and process AmigaDOS hunk types.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_dos_hunk_list`: Iterates through a memory structure to identify and process AmigaDOS hunk type codes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_amigados_hunk_relocations`: Parses a memory segment by iterating through hunk type codes to perform relocation and data copying.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_amiga_hunk_relocations`: Iterates through a list of memory segments to perform relocation and data copying.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_hunk_relocations`: Iterates through a memory segment to apply relocation offsets and patch data based on hunk type codes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `realign_hunk_data`: Iterates through memory segments to adjust data alignment and relocate hunk contents.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_hunk_data`: Copies a block of memory to a new location and adjusts internal pointers or offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_amiga_hunk_relocations`: Parses a memory structure containing AmigaDOS hunk types to perform data relocation and memory copying.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_amigados_relocations`: Iterates through a list of relocation hunk entries to adjust memory addresses within a loaded segment.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `patch_amiga_hunk_headers`: Iterates through a memory block to modify hunk type codes, specifically setting values to represent executable or relocation types.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_data_structure`: Copies and rearranges blocks of memory based on length and offset parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_buffer_elements`: Iterates through a range of memory locations defined by an offset and a counter, writing specific byte values and copying a byte from another memory location into the target range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reconstruct_data_block`: Copies data from one memory location to another and inserts specific marker bytes into the resulting buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_byte_sequence`: Decrements a counter and copies a sequence of bytes from one memory location to another using an offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_reformat_memory_block`: Copies a block of memory to a new destination and performs a byte-level transformation on the data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_byte_and_branch`: Copies a byte from a source address using an index to a destination address, decrements a counter in a loop, writes a constant byte to a destination, pushes the destination address onto the stack, and branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_and_call`: Copies a block of longwords from a source to a destination, updates a pointer, and calls a function via a pointer stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_byte_and_branch`: Decrements a counter in a loop, writes a constant byte to a destination, pushes the destination address onto the stack, and branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_with_loop`: Copies a block of longwords from a source to a destination, calls a function via a pointer, and performs bitwise operations on a value in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_byte_and_stack`: Writes a constant byte to a destination, pushes the destination address onto the stack, and branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_conditional`: Copies a block of longwords from a source to a destination and calls a function if a specific condition is met, then performs bitwise operations on a value in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_address_and_branch`: Pushes the contents of a data register onto the stack and performs a short relative branch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_pointer_and_branch`: Clears a register, moves it to a destination, performs arithmetic on the destination and stack pointers, and branches.
- `process_nested_tables`: Traverses a complex structure of nested tables and pointers, performing arithmetic on values found at specific offsets.
- `decrement_and_loop`: Decrements a word value in a register until it reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_callback_with_context`: Sets a value to zero at an offset from a pointer, then executes a function pointer stored in another location while updating a return address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_add_d4`: Shifts the contents of register d1 left by one bit and adds the result to register d4 before branching.
- `decrement_loop_counter`: Decrements a value in register d1 until it reaches zero.
- `check_and_call_function`: Validates a value in register d1, executes a function via a pointer, and conditionally calls another function based on a value in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `release_system_resources`: Calls several functions via pointers, including those that appear to handle library or message management.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_shifted_pointer`: If a value pointed to by register a0 is zero, it stores the value of register a0 shifted right by two bits into a location pointed to by register a4.
- `process_bitstream_data_2`: Performs complex bitwise manipulations and rotations on data using multiple registers and copies data between memory locations based on calculated offsets.
- `test_bit_shift`: Performs a bitwise shift on a value using a mask from another register and checks if the result is non-zero.
- `process_bitstream_data`: Iterates through a loop to perform complex bitwise manipulations and rotations on data using values from multiple registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_branch`: Loads a value from the stack into register a0 and then performs a short branch.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_byte`: Tests a byte value and returns if the condition is met.
- `test_bit_shift_short`: Checks if a bitwise shift of one register by a masked amount from another results in a non-zero value.
- `subtract_word_2`: Subtracts a word value from another word value.
- `extract_bitstream_bits`: Iterates through a data stream to extract and shift bits into a target register using variable bit widths.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Performs a conditional loop to copy a block of memory from a source to a destination using a pointer and a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding`: This routine is empty and performs no operations.
- `check_flag_and_loop`: Calls a routine to retrieve a value, checks a specific bit in that value, calls another routine if the bit is set, and then enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_and_copy`: Executes a conditional sequence involving multiple function calls and a memory block copy, followed by updating a value through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_chained_calls`: Calls multiple routines in sequence, performing bitwise checks and executing a function pointer from a table if certain conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_memory_copy`: Enters a loop that repeatedly calls a routine, updates a value through a pointer, and performs a memory block copy until a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `evaluate_status_condition`: Checks if a value is zero, evaluates a bit in a memory location, and returns a specific value based on the results of other routines and a memory check.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_copy_loop`: Adds values to a register, compares them, and enters a loop that moves bytes from a source to a destination while decrementing a counter and a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `verify_memory_state`: Checks a status flag, evaluates a bit in a memory location, and returns a specific value based on the results of other routines and a memory check.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a short relative branch to a specific instruction offset.
- `copy_bytes_with_loop`: Calls another routine, then copies bytes from a source table to a destination by decrementing a pointer and using a loop counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_status_flag`: Calls another routine and returns a specific value if a certain memory location contains a non-zero value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_minimal`: Copies bytes from a source table to a destination using a loop counter and a decrementing pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_and_return_status`: Calls another routine and checks if a specific offset within a global structure contains a non-zero value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_branch_lower`: Compares two longwords and branches if the first is less than or equal to the second.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_and_return_status_2`: Calls another routine and then checks if a specific offset within a global structure contains a non-zero value.
- `write_string_to_buffer`: Iterates through a null-terminated string and copies each character into a buffered stream.
- `parse_script_tokens`: Parses a source string into a table of pointers, handling whitespace skipping and quoted strings.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `prepare_stream_buffer`: Checks if a stream lacks a buffer or has a specific reset flag set.
- `write_byte_to_stream`: Writes a single byte to a buffered stream unless the stream is in an error or flushing state.
- `move_word_to_register`: Moves a word from a memory location pointed to by a register into another register.
- `read_byte_from_stream`: Returns the next byte from a buffered input stream, refilling the buffer through another routine when it empties.
- `copy_string_limited`: Copies a null-terminated string from a source pointer to a destination buffer, limited by a length in D0 or the source length.
- `insert_block_into_free_list`: Returns a memory block to a sorted free list.
- `check_status_flag_2`: Calls two functions and checks a value at a specific offset in a global structure to return a status or a failure code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry_2`: Invokes another routine and checks the zero flag.
- `open_dos_library_3`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library_3`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_linked_list`: Calls a function via a pointer in a global structure to allocate memory and then initializes a new node by setting its members and updating several pointers in a global structure.
- `extract_string_data`: Reads bytes from a table while decrementing a counter in a global structure, stopping if the counter reaches zero or if a specific byte value is encountered.
- `allocate_block_from_free_list`: Allocates a block of at least the requested size from a sorted free list, rounding the size up to a multiple of four with a minimum of eight.
- `execute_dos_call_with_error_handling`: Executes a function via a pointer stored in a table and checks if the result is negative one.
- `call_dos_vector_with_status`: Clears the error state and calls a DOS vector function.
- `move_and_branch_2`: Moves the contents of one register to another and performs a short relative branch.
- `add_and_branch_2`: Adds the contents of two registers and performs a short relative branch.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `read_file_with_error_capture`: Clears the error state and calls a DOS read function.
- `multiply_fixed_point_values`: Multiplies two 32-bit fixed-point numbers by calculating the product of their high and low 16-bit components and summing the cross terms to produce a 32-bit result.
- `divide_fixed_point_values`: Performs a signed division of two 32-bit fixed-point values using normalization and bit shifting.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `get_table_entry_pointer_2`: Validates an index against a range and checks if the corresponding eight-byte entry in a table is non-zero.
- `process_library_callback`: Calls a function via a pointer in a global structure and, if certain bitwise conditions and non-zero results are met, clears a pointer and calls another routine.
- `initialize_engine_subsystems_2`: Clears a block of memory and initializes several engine structures by calling functions through a table of pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource_manager`: Clears a memory block and initializes a resource management structure by calling several internal functions to allocate and link data pointers.
- `branch_to_address`: Performs a short branch to a specific address.
- `branch_to_offset_2`: Loads a value from a stack offset into a register and performs a relative branch.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `validate_lookup_result_status`: Calls a lookup routine and returns minus one if it yields nothing.
- `copy_text_with_space_limit`: Copies a text string into an output buffer one character at a time while decrementing a remaining-space counter, calling a flush routine when the space runs out.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `prepare_stream_buffer_2`: Obtains a new buffer through a routine when the stream has none or a reset flag is set.
- `write_byte_to_stream_2`: Writes a single byte from a register into a buffered stream.
- `move_word_to_register_2`: Moves a word from a location pointed to by a register into another register.
- `read_byte_from_stream_2`: Reads the next byte from a buffered input stream, refilling the buffer when empty and handling end-of-file or error conditions.
- `copy_string_limited_2`: Copies a null-terminated string from a source pointer to a destination buffer, limited by a length in a register or the source length.
- `insert_block_into_free_list_2`: Inserts a memory block into a linked list of free memory segments, rounding the size up to a multiple of four with a minimum of eight.
- `execute_sequential_lookups`: Calls two separate routines sequentially and returns minus one if the first returns zero or if a field in the context block is nonzero after the second call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_byte`: Moves a value from a register to another, calls a routine, stores a byte into a table using an offset, increments a value, and branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_linked_list_node`: Allocates a node of the requested size plus twelve bytes through a library memory call, stores the size and a link field in its header, and appends it to a linked list.
- `allocate_memory_block`: Searches a linked list of free memory blocks to find a suitable candidate based on a requested size.
- `dos_call_with_error_code`: Clears the error state and calls a DOS vector function.
- `handle_library_callback`: Checks a global flag, clears a specific memory location, and calls a function from a table based on a pointer.
- `move_and_branch`: Moves a value from a register to another and branches to a different location.
- `add_and_branch`: Adds the contents of two registers and branches to a different location.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `execute_callback_logic`: Checks a global flag, clears a memory location, and calls a function via a pointer from a table.
- `close_file_handle`: Checks a global flag and calls a function through a pointer retrieved from a table offset.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point`: Performs a normalized fixed-point division of two 32-bit values, handling sign logic and bit shifting to maintain precision.
- `divide_fixed_point_2`: Performs division on 32-bit fixed-point values using bit shifts and integer division.
- `get_table_entry_pointer`: Returns the address of an entry in an eight-byte-stride table when the index is in range and the entry is in use.
- `check_and_clear_resource`: Calls a routine via a library offset, checks specific bits in the result, and if certain conditions are met, clears a value and calls another routine.
- `validate_installation_path`: Checks the user-provided installation path for valid Amiga filesystem syntax, specifically looking for drive colons and directory separators.
- `poll_entries_with_timeout`: Iterates through a table of entries, checking a clock value against a stored time to trigger memory reallocation and status updates.
- `scan_entries_and_free`: Iterates through a table of entries and checks their status flags and string identifiers.
- `send_port_message_and_wait`: Allocates a message structure, fills in its node links and two data fields, and sends it through a message port.
- `traverse_free_list`: Iterates through a linked list of memory blocks and calls another routine if a block meets specific bitmask criteria and has non-matching adjacent pointers.
- `validate_pointer_status`: Checks a pointer and its associated byte flags to return a specific status value, such as zero or all bits set.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `reset_file_error_state`: Sets specific bytes and words to a default error value and calls a function through a pointer in a structure.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_object_structure`: Calls a function via a table, and if successful, populates a structure with various data fields including an integer and a byte.
- `multiply_fixed_point_values_2`: Calculates the product of two 32-bit fixed-point numbers using their high and low 16-bit components.
- `read_byte_stream`: Reads bytes from a source through a pointer, decrementing a counter until it reaches zero, and stores the bytes into a destination buffer until a specific terminator is found.
- `zero_filled_data`: This routine consists of a series of instructions that perform logical operations to clear a register, likely representing a zero-filled data area.
