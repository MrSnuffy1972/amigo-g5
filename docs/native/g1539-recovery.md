# Game 1539 (ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01539). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911568 top-level bytes in 94 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 48 of 48 units identical (bytes and relocations), 73 raw-word lines |
| Functions named and described | pass, 126 of 126 named (0 author symbols), 126 of 126 described (6 by rule, 66 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1240 of 1240 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 81775 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3557 of 3557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 129740 |
| code | 9386 |
| data | 737344 |
| padding | 12570 |
| unused | 22528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:boot | 3 | 223 | 6 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro | 2 | 206 | 7 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:nydos | 1 | 129 | 12 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 119 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 179 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 622 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 306 | 8 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:boot#unpacked | 1 | 306 | 8 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro#unpacked | 3 | 786 | 22 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:ghouls.prg (raw code file, load address unknown) | 1 | 265 | 4 |

## Cases handled

- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:HIGH.T1: high-entropy data file (7.84 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL4.IFF: high-entropy data file (7.69 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:TITLE.T1: high-entropy data file (7.80 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL202.IFF: high-entropy data file (7.65 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:BIGCHAR.IFF: high-entropy data file (7.65 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL101.IFF: high-entropy data file (7.67 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:COUNT.T1: high-entropy data file (7.78 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LUCIFER.IFF: high-entropy data file (7.62 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL2.T1: high-entropy data file (7.81 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL5.IFF: high-entropy data file (7.68 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL3.T1: high-entropy data file (7.78 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL3.IFF: high-entropy data file (7.67 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL4.T1: high-entropy data file (7.74 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL5.T1: high-entropy data file (7.82 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:LEVEL102.IFF: high-entropy data file (7.72 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 106 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 99 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 442 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 123 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:boot: crunched executable decrunched by XFDMaster (TNM Cruncher 1.1 (xfd), 20236 -> 37460 bytes), 18 of 18 windows confirmed in reference RAM: recovered as a hunk executable
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro: crunched executable decrunched by XFDMaster (PowerPacker 2.3 (xfd), 67208 -> 114892 bytes), 38 of 38 windows confirmed in reference RAM: recovered as a hunk executable
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:ghouls.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate, 2 jump target
- ghouls 'n ghosts (1989)(u. s. gold)[cr][t][a]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 81775 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro
- Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro#unpacked

## Copy protection found

- `initialize_dma_and_clear_memory` (runtime segment an address (play-rs-x25k)): Configures DMA controller registers and clears two memory blocks using a loop. Kind: checksum/integrity trap. Pass path: The routine appears to be a sequence of memory clears and hardware register writes that culminate in a series of calls to functions that likely validate the integrity of the loaded code or hardware state. The presence of `halt_baddata()` an.

## Functions


### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:boot

- `program_entry_single_call`: Serves as the program entry and calls one other routine.
- `load_executable_hunks`: Allocates memory and copies hunk data while walking the hunk records, comparing their type codes with the AmigaDOS executable hunk codes.
- `nop_routine`: This routine is empty and performs no operations before returning.
- `decompress_bitstream_data`: Reads a bitstream to determine how many bytes to copy from a source table into a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bits_store_pointer`: Shifts a register right once per count, and when it runs out of bits it loads the next longword from memory and sets its top bit.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro

- `load_hunks_and_find_task`: Finds the current task and allocates memory for loaded hunks, copies hunk data while comparing hunk type codes, and applies relocation additions.
- `return_empty_stub`: This routine consists of five instructions that perform no observable work before returning.
- `spin_for_count_register`: Loops once per unit of the count held in a data register, decrementing it until it passes zero, then returns with no other effect.
- `unpack_lz_codes_backwards`: Decodes a compressed stream into a buffer filled from its end toward its start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_char_wrap_to_space`: Repeatedly decrements a character register for the count given in its input, wrapping to a space character whenever it reaches zero.
- `empty_routine`: This routine contains no executable instructions.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:nydos

- `copy_code_to_low_memory`: Copies a block of words from the routine's own code into low memory at a fixed address, stopping at a fixed end address.
- `load_pointed_longword_then_call`: Loads a longword from an address computed from a global pointer and an offset of minus four, stores it in a global variable, and then calls a subroutine.
- `unpack_lz_codes_refilling`: Decodes a packed stream into a buffer filled from its end toward its start.
- `unpack_lz_codes_nested_loops`: Decodes a packed stream backwards into a buffer, using nested loops that read control codes, copy earlier output and refill the bit reader from the packed data.
- `unpack_source_backwards_bits`: Decodes a packed stream by reading its source bytes backwards, shifting bits into an output byte and emitting bytes into a buffer that fills from its end.
- `unpack_lz_codes_with_jumps`: Decodes a packed stream backwards into a buffer, reading control codes that select literal bytes or copies of earlier output and using jumps between its decode steps.
- `unpack_lz_codes_two_branches`: Decodes a packed stream backwards into a buffer, with two branches for control codes that select literal bytes or copies of earlier output.
- `unpack_lz_codes_bit_reader`: Decodes a packed stream backwards into a buffer, reading control codes through a bit reader that loads the next longword from the packed data and sets its top bit.
- `unpack_lz_codes_restart_loop`: Decodes a packed stream backwards into a buffer in a restartable loop, reading control codes that select literal bytes or copies of earlier output.
- `shift_pointer_value_2`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `skip_bits_backwards_reload`: Shifts a bit buffer right by a count of bits, loading the previous longword from memory whenever the buffer empties and setting its top bit.
- `refill_bits_from_previous_longword`: Walks backwards through memory one longword at a time, shifting its bits into a bit buffer with a top bit set, and returns when the bit count is exhausted.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `initialize_memory_and_hardware`: Clears a block of memory starting at a specific address, updates several global variables, and modifies hardware registers to enable DMA and interrupts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_software_interrupt`: Triggers a software interrupt using a trap instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_state`: Clears a block of memory, sets values in global variables, and updates fields in a structure before executing a processor trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_software_interrupt`: Triggers a software interrupt using a trap instruction.
- `initialize_dma_and_clear_memory`: Configures DMA controller registers and clears two memory blocks using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_blocks`: Clears two large blocks of memory by filling them with zeros using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_memory_and_update_globals`: Fills a memory area pointed to by a register with zeros and modifies global variables and a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_and_data`: Stores a value into two global variables, performs several memory writes through a pointer, calls another routine, and updates the global variables with the returned value before calling two more routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_globals`: Stores a longword from a register into two global variables and performs several memory writes through a pointer before calling two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_and_update_memory`: Stores the contents of a data register into two consecutive global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_data`: Writes values to a memory location through a pointer, calls another routine, and then copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_pointer`: Stores a longword from a register into a memory location through a pointer and performs several memory writes through a pointer before calling two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_2`: This routine contains no instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_update_data`: Performs memory writes through a pointer, calls two routines, waits for a specific bit in a byte to be set, updates a value through a pointer, and copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_nested_pointer_data`: Calls two other routines and then copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_update_data_alt`: Performs a memory write through a pointer, calls three routines, waits for a specific bit in a byte to be set, updates a value through a pointer, and copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_and_data_2`: Calls two other routines, stores a word into a location relative to a pointer, and copies a longword from a nested pointer structure into another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_with_updates`: Calls several routines and enters a loop that checks bit flags in a structure, updates a word at a specific offset, and calls two more routines until a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `partial_update_data`: Calls a routine and then stores a word and a longword into specific offsets relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_with_updates_2`: Executes several subroutines and performs a loop that monitors specific bits in a structure while updating a word and calling additional routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_pointer_data`: Stores a word and a longword into offsets of a pointer, where the longword is sourced from a nested pointer structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_with_updates_3`: Performs a series of subroutine calls and enters a loop that checks bits in a structure, updates a word, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_longword_only`: Copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_and_copy`: Executes a loop based on bit flags in a structure, updates a word, and finally copies a longword from a nested pointer structure into a relative offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_indirect_pointer_value`: Fetches a value from a pointer located at an offset within a structure and stores it into another structure at a fixed offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_and_reset`: Runs a loop checking structure bits, updates a word, performs a second loop to check a bit, and then stores a word, a byte, and a longword into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_direct_longword`: Copies a longword from an offset of a pointer into another location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_reset_input_state`: Polls a bitmask in a data structure until a specific bit is set, then writes a value to a different offset within that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_and_update_status`: Copies a block of data from one memory location to another using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_and_copy_data`: Initializes several memory blocks by writing structured data and copying a sequence of words from a table to a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_3`: This routine contains no executable instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_and_update_memory`: Copies a block of data from one memory location to another and performs several repetitive memory writes and bitwise operations on various data structures.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_3`: This routine contains no executable instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dynamic_table_values`: Calculates new values using bitwise shifts and additions, then writes them into a memory table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_and_initialize_tables`: Copies a block of data to a specific memory location and performs multiple sequential writes to initialize various data structures and tables in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from a source address to a destination address using a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_structured_table`: Fills a table with incrementing values, copies data to a specific address, and then populates a series of structure entries with calculated values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_structured_table_2`: Copies a sequence of words from a source table to a destination, then populates a series of structures with incrementing values split across two word fields.
- `execute_loop_five_times`: Calls another routine five times in a loop.
- `execute_loop_five_times_2`: Performs a loop that executes another routine five times.
- `update_structure_from_table`: Iterates through a table to update multiple fields within a structure based on values read from memory.
- `repeat_call_five_times`: Invokes the routine it calls five times in a counted loop, after being called by update_structure_from_table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_animation_frame_data`: Updates a global state variable based on input and increments or decrements it through various conditional branches.
- `render_text_bitmap_to_planes`: The routine takes a string of characters from a global buffer, maps each character to a 32-byte glyph in a font table, and copies that glyph's bytes column by column into a bitmap destination in planar screen memory, stepping by a fixed row stride.
- `copy_data_via_offsets`: Executes a triple-nested loop that reads from a source table and writes to a destination table using offsets derived from a byte value.
- `update_state_and_parameters`: Decrements a global variable and, upon reaching zero, updates several fields in a structure and performs a lookup to set a value.
- `unpack_run_length_literals`: Unpacks a run-length stream in which each control byte either copies the following literal bytes to the output or repeats the next byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_data_2`: Writes values to a memory location through a pointer and copies a longword from a nested pointer structure.
- `call_and_return_registers`: Calls another routine and returns the concatenated result in registers.
- `update_global_values`: Sets a global variable to a constant value or calculates an offset to copy a word from a memory location into two different global variables.
- `initialize_memory_table`: Sets a flag in a global variable and performs a series of calculations to populate several memory locations using values from a table.
- `update_audio_channels`: Clears audio registers if a stop flag is set; otherwise, it manages a step timer and updates the pointer, length, period, and volume registers for four audio channels before enabling DMA.
- `spin_on_volatile_byte`: Repeats an empty loop that compares a byte with itself, so it only ends if the hardware changes that byte between reads.
- `advance_channel_sequence_pointer`: Advances one sound channel's event pointer through its data.
- `interpret_music_channel_commands`: Interprets a music command stream for one channel, where special command bytes select instrument or sample data from a table.
- `initialize_four_sound_channels`: Disables the four audio channel interrupts, then initialises four channel structures from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Clears a block of memory, sets several global variables, and copies a sequence of instructions to a memory location.
- `execute_library_calls`: An 86-instruction routine that opens a library and manages superstate using library calls while interacting with hardware registers and text data.
- `update_font_and_text_buffer`: Modifies a global variable through conditional logic and performs a nested loop to copy data from a table into a memory location.
- `trigger_system_trap_and_invert_word`: Copies the stack pointer to a data register, executes a software interrupt, and performs a bitwise NOT operation on a word in a data register.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `copy_data_from_table`: Performs a nested loop to transfer data from a table to a memory location using offsets derived from a global variable.
- `decrement_and_reinitialize_structure`: Decrements a global variable and, if it reaches zero, updates several fields in a structure and performs a lookup to set a value.
- `unpack_run_length_literals_2`: Decodes a run-length encoded stream where control bytes determine whether to repeat a single byte a specified number of times or copy a sequence of literal bytes to the destination.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:boot#unpacked

- `clear_memory_and_set_up_copper`: Disables all DMA and interrupts, zeroes a block of memory by writing zero longwords in a loop, stores a copper list address into the copper location register and triggers a copper jump, then enables DMA.
- `open_library_and_set_state`: Calls library functions to open a library and manage system state while interacting with DMA and interrupt registers.
- `update_input_and_copy_tables`: Reads a byte from a memory location and performs conditional arithmetic and bitwise operations on a global variable.
- `copy_stack_pointer_and_trap`: Copies the stack pointer a7 into address register a0, then executes trap 15 (the Amiga system trap for calls such as Exec or DOS), and finally inverts all 16 bits of data register d6 (not.w).
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_glyph_columns_to_buffer`: Walks a global variable as a character string, and for each character it looks up a 32-byte entry by subtracting 32 from the character code and shifting left by five, then writes two bytes per column into a 200-byte-stride destination in seven interleaved passes across a 27-character row count.
- `decrement_or_program_blitter`: Decrements a global variable by one and returns if it is nonzero; otherwise it subtracts three from it, reads characters from a string via a pointer and waits for the blitter busy flag to clear before programming the blitter pointers, size and modulo registers with a value taken from a table indexed by the character.
- `expand_packed_byte_stream`: Reads a longword count from the address in a0 and then decodes a byte stream, where a control byte of 128 is skipped, a positive control byte copies that many following bytes to the output at a1, and a negative control byte repeats the next byte, stopping once the count reaches zero.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:intro#unpacked

- `install_trap_vector_and_trap`: Stores the address of a routine into the vector at low memory address 128, executes trap #0, then sets d0 to zero and returns.
- `initialize_system_and_update_state`: Sets the status register, writes a longword and a byte to a global variable, calls three different routines, updates three offsets in a structure pointed to by a register, and finally updates a longword in that structure by dereferencing a pointer found within a table before returning from an interrupt.
- `initialize_data_tables`: Populates several memory locations with specific values and copies data from a source table into a destination table using three separate loops.
- `write_two_tables_five_times`: Loads address registers a0 and a3 with fixed addresses and sets d0, d5 and d7 to constants, then branches to a subroutine, advances a0 by 640 and a1 by 20, and repeats the sequence six times using a counter.
- `repeat_call_five_times_2`: Sets a 16-bit counter to five, calls a subroutine, decrements the counter, and repeats until the counter reaches negative one, then returns.
- `write_structured_data`: Iterates through a buffer to write multiple multi-byte values into a structured memory block at fixed offsets.
- `wait_and_reset_data`: Loops until a specific byte in a structure is no longer negative, calls another routine twice, writes 32 to an offset and 0 to another offset in a structure, and then loops while bit 6 of the CIAA_PRA register is set; once the loop exits, it writes 15 to the first offset and returns.
- `toggle_word_counter_bit`: Increments a 16-bit global variable, then clears all bits except bit 0, so it holds only the low bit, and returns if that bit is zero, otherwise it returns.
- `process_table_and_decrement`: If a 16-bit value at a specific location is 512, it sets it to 12800, then uses a value from a table to calculate an offset into another table to check a byte; if that byte is negative, it calls a routine to decrement a 16-bit global variable, otherwise it calls another routine.
- `increment_table_values`: Iterates through a buffer, increments bytes in a table at specific offsets if they do not equal a constant, and calls another routine after incrementing a global counter.
- `decrement_nonzero_word_global`: Tests a 16-bit global variable and, if it is nonzero, subtracts one from it, then returns.
- `reset_table_index`: Calls a subroutine, clears a global variable, increments another global variable, and if the value at a calculated offset in a table is negative, resets that variable to zero; it then calls another subroutine and writes two constants to global variables.
- `set_hardware_parameters`: Checks a bit in a memory location and, if not set, writes several constant values to a structured memory block at fixed offsets.
- `branch_into_shared_routine`: Branches unconditionally to a fixed location inside another routine, transferring control there without returning to this routine.
- `call_and_return_registers_2`: Saves registers d0-d7 and a0-a6 on the stack, calls another routine, restores them, and returns.
- `update_audio_parameter`: Updates specific global variables using values retrieved from a calculated offset in a table.
- `initialize_memory_pointers`: If a global variable is zero, it sets it to one, enables a bit in the CIAA Port A Data Register, and calculates and stores several memory addresses into a table using various arithmetic operations and multiplications.
- `update_audio_channel_registers`: Returns immediately after zeroing the four channel volume registers and disabling audio DMA when a global stop flag is nonzero; otherwise it decrements a global step counter and reloads it when it reaches zero.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `advance_channel_event_pointer`: Decrements a per-channel step counter held in the structure in A0; when the counter reaches zero or the current event's marker byte is 1, it fetches the next event record from a table, copies its status byte and a flag byte into the structure, and resets the event pointer.
- `process_channel_stream_commands`: Loops over a command stream held in a structure pointed to by a0, dispatching on command bytes: the 226 command writes the channel's enable bit into the DMA control register and loads a pointer, length and volume from a table entry, the 229 command loads a sample pointer and length from a table indexed by the following byte, and the 225 command copies bytes into a global variable until a non-matching byte is found.
- `initialize_audio_channel_structures`: Disables the audio channel interrupts and DMA, then loops over four channel structures, storing pointers, clearing counters and flags, and setting a volume byte of 255 and a period-related word from computed values.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr][t][a].adf:ghouls.prg (raw code file, load address unknown)

- `raw_code_region_entry`: The decompiled code is unavailable, so this description follows the listed facts only.
- `run_table_loop_until_end`: Loops over a table of values, calling a set of routines on each pass and waiting for a counter to reach three.
- `run_fixed_routine_sequence_loop`: Calls the same sequence of seven routines in a loop that runs thirteen times, then returns.
- `run_twelve_step_update_cycle`: Runs a fixed sequence of seven routine calls twelve times in a row.
