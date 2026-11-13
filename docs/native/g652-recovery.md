# Game 652 (Ghouls 'N Ghosts): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00652). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911648 top-level bytes in 96 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 50 of 50 units identical (bytes and relocations), 70 raw-word lines |
| Functions named and described | pass, 131 of 131 named (0 author symbols), 131 of 131 described (9 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1258 of 1264 (99.53%) |
| Recovered without load address (counted in proof 1, flagged) | 81775 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3535 of 3575 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 129740 |
| code | 9480 |
| data | 739312 |
| padding | 12636 |
| unused | 20480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SENSEI | 2 | 24 | 4 |
| Ghouls 'N Ghosts.adf:boot | 3 | 223 | 6 |
| Ghouls 'N Ghosts.adf:intro | 2 | 206 | 7 |
| Ghouls 'N Ghosts.adf:nydos | 1 | 129 | 12 |
| Ghouls 'N Ghosts.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 117 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 179 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 622 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 306 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Ghouls 'N Ghosts.adf:boot#unpacked | 1 | 306 | 8 |
| Ghouls 'N Ghosts.adf:intro#unpacked | 3 | 786 | 22 |
| Ghouls 'N Ghosts.adf:ghouls.prg (raw code file, load address unknown) | 1 | 265 | 4 |

## Cases handled

- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:HIGH.T1: high-entropy data file (7.84 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL4.IFF: high-entropy data file (7.69 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:TITLE.T1: high-entropy data file (7.80 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL202.IFF: high-entropy data file (7.65 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:BIGCHAR.IFF: high-entropy data file (7.65 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL101.IFF: high-entropy data file (7.67 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:COUNT.T1: high-entropy data file (7.78 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LUCIFER.IFF: high-entropy data file (7.62 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL2.T1: high-entropy data file (7.81 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL5.IFF: high-entropy data file (7.68 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL3.T1: high-entropy data file (7.78 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL3.IFF: high-entropy data file (7.67 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL4.T1: high-entropy data file (7.74 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL5.T1: high-entropy data file (7.82 bits/byte), classified as packed data
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:LEVEL102.IFF: high-entropy data file (7.72 bits/byte), classified as packed data
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
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:boot: crunched executable decrunched by XFDMaster (TNM Cruncher 1.1 (xfd), 20236 -> 37460 bytes), 18 of 18 windows confirmed in reference RAM: recovered as a hunk executable
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:intro: crunched executable decrunched by XFDMaster (PowerPacker 2.3 (xfd), 67208 -> 114892 bytes), 38 of 38 windows confirmed in reference RAM: recovered as a hunk executable
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:ghouls.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate, 2 jump target
- Ghouls 'N Ghosts/Ghouls 'N Ghosts.adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 81775 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Ghouls 'N Ghosts.adf:intro
- Ghouls 'N Ghosts.adf:intro#unpacked

## Copy protection found

- `clear_memory_and_set_registers_3` (runtime segment an address (play-rs-x25k)): Sets a global variable, modifies a value through a pointer, clears a memory range, and writes to several global variables. Kind: loader/decryption setup. Pass path: the code following the memory clearing loop and register initialization.

## Functions


### SENSEI

- `call_library_entry`: Calls one routine and, if the zero flag is clear, calls a second routine and then jumps through a function pointer taken from a global table.
- `open_dos_library`: Opens dos.library through the exec OldOpenLibrary call and stores the returned library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Ghouls 'N Ghosts.adf:boot

- `forward_boot_entry_call`: A two-instruction boot entry that makes one call to another routine.
- `load_hunk_executable_sections`: Parses AmigaDOS hunk records from a loaded executable, allocating memory and copying section data with exec calls, and dispatches on hunk type codes 1001 to 1004 to handle code, data and relocation records.
- `stub_empty_return_routine`: The decompiled output contains only a return, so no visible work is done.
- `decompress_bitstream_into_buffer`: Decodes a bit-packed compressed stream into a buffer, with the same flag-bit logic for literal copies and back-references, and reads further bit words through a shift routine.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bits_from_stream_loop`: Repeats a count-controlled loop that shifts a bit register right; when the register empties it reloads a word from the source pointer and sets its top bit, storing the pointer in a global on each pass.

### Ghouls 'N Ghosts.adf:intro

- `allocate_and_load_hunk_sections`: Allocates a memory block for each section of an executable and records the addresses in a table, then walks the hunk type records and dispatches on codes 1001 to 1004.
- `return_empty_stub`: This routine consists of five instructions that perform no observable work before returning.
- `delay_loop_countdown`: Spins in a loop that decrements a count register until it reaches zero, which gives a timed wait.
- `decode_backward_run_copies`: Decodes a stream of run-length codes and copies bytes backward into a buffer, selecting run lengths from a small set of control values and stopping when the write pointer reaches the start of the buffer.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_space_wrap`: Decrements a byte held in a register once per count step, replacing it with a space character when it underflows to zero.
- `stub_bad_instruction_data`: The decompiled output is only a truncation at bad instruction data, so there is no real routine body.

### Ghouls 'N Ghosts.adf:nydos

- `copy_entry_block_to_memory`: At program entry, copies a block of 16-bit words from the code to a fixed low memory location, advancing both pointers until the source reaches a global end address.
- `load_pointer_and_call_routine`: Loads a pointer from a global value offset by a relative location, stores the value back in a global, and calls one other routine.
- `decode_backward_bitstream_copies`: Decodes a bit-packed stream read backward, writing bytes into a destination that also fills from the end, with copy lengths chosen by a bit pattern and back-references into earlier output.
- `decode_bitstream_selected_copy_modes`: Decodes the same backward bit-packed format, with an extra choice of copy mode from a small control value for cases one to three.
- `unpack_backward_bitstream_data`: Unpacks bit-packed data read backward from a source pointer into a destination filled from the end, shifting bits into bytes and copying earlier output by control value.
- `unpack_bitstream_copy_segments`: Unpacks bit-packed data into a destination filled from the end, reading control bits and copying earlier output in segments of a set length.
- `copy_backward_from_earlier_output`: Copies bytes backward from earlier output in a loop, reading control bits and writing a byte stream into the destination as it goes.
- `unpack_bits_with_shift_reader`: Unpacks bit-packed data into a buffer by reading bits from a shift routine, copying runs of earlier output as it goes.
- `decode_bits_backward_into_buffer`: Decodes bit-packed data into a buffer that fills from the end, selecting copy modes by control value and calling a shift routine together with two other routines.
- `shift_pointer_value_2`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bits_from_backward_source`: Shifts bits out of a bit register, refilling it from the 32-bit word just before the source pointer and setting the top bit whenever it refills, repeated for a count value.
- `refill_bits_backward_until_count`: Repeatedly refills a bit register from the longword before the source pointer and shifts bits out, returning when a count runs out.

### Ghouls 'N Ghosts.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `clear_memory_and_set_registers`: Fills a memory range with zeros and writes specific values to several global variables and hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_software_interrupt`: Triggers a software interrupt via a trap instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_set_registers_2`: Writes a value to a global variable, clears a memory range with zeros, and updates several other global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Stores a value into a global variable and executes a software interrupt.
- `clear_memory_and_set_registers_3`: Sets a global variable, modifies a value through a pointer, clears a memory range, and writes to several global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_block`: Fills a memory block with zeros using a loop and then writes specific values to several global variables and a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_transfer`: Sets a value in a custom register and enables a DMA channel by writing to the DMA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_structure`: Stores several values into a table and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_table_values`: Writes two values into a table at specific offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `split_and_store_word`: Splits a 32-bit register into two 16-bit values and stores them in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_table_entry`: Writes a 32-bit value and a 16-bit value into a table, then performs bitwise OR operations on a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_and_wait`: Stores a 32-bit value as two 16-bit words in a table and enters a loop that waits for a specific bit to be set in a byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_from_pointer`: Writes a value to a table, calls other routines, and then copies a 32-bit value from a location pointed to by a global variable into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_bit_set`: Clears a byte and a word in a table, calls other routines, and then loops until a specific bit is set in a byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: This routine contains no instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Performs a series of table writes, calls multiple routines, waits for a bit to be set, and finally copies a value from a pointer-based location into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_and_data`: Calls two other routines, stores a word into a location relative to a pointer, and copies a longword from a nested pointer structure into another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_and_update`: Executes several routines, performs a loop checking a bit in a byte, updates a word, calls more routines, and performs data transfers similar to R1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `partial_update_data`: Calls a routine and then stores a word and a longword into specific offsets relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_byte_to_memory`: Copies a single byte from a source address to a destination address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_pointer_data`: Stores a word and a longword into offsets of a pointer, where the longword is sourced from a nested pointer structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_input_and_sync`: Runs multiple routines and loops based on bitwise checks of a byte, clears a value, performs a bitwise OR on a longword, and copies a longword from a nested pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_longword_only`: Copies a longword from a nested pointer structure into a location relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic`: Performs a series of routine calls and loops that check specific bits in a byte before updating a longword and copying data from a nested pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_dereference`: Fetches a value from a nested pointer structure and stores it at an offset within a destination structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nested_loops`: Executes routines and loops controlled by bitwise flags, clears a value, modifies a longword with a bitwise OR, and copies a longword from a nested pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_dword_offset`: Copies a 32-bit word from an offset of a source pointer to an offset of a destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from a source address to a destination address using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_loop`: Performs a loop that polls bits in a structure, modifies specific byte values, and calls other routines before terminating.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables_and_copy_memory`: Populates a memory block with incrementing values in a loop, updates several global variables, and performs a block memory copy from a source table to a destination in chipset memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_tables`: Populates several memory tables with incremental values and copies data from a source buffer to a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables`: Populates a series of memory structures by copying values from source tables and performing arithmetic increments on a data stream.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_2`: Copies a block of data from a source table to a destination memory address using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables`: Populates a series of memory structures by copying values from source tables and performing arithmetic increments on a data stream.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_to_memory`: Performs a series of memory writes to a global structure and then copies a block of data from one memory location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables`: Populates a series of memory structures by copying values from source tables and performing arithmetic increments on a data stream.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_and_copy`: Writes to a structure within a loop, performs several specific memory writes, and copies a block of data to a destination address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_structured_table`: Fills a table with incrementing values, copies data to a specific address, and then populates a series of structure entries with calculated values.
- `execute_loop_five_times`: Calls another routine five times in a loop.
- `execute_loop_five_times_2`: Performs a loop that executes another routine five times.
- `update_structure_from_table`: Iterates through a table to update multiple fields within a structure based on values read from memory.
- `process_table_indices`: Increments a global variable and uses it to index into a table to modify values at specific memory locations based on a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `render_text_glyph_rows_2`: Checks the status of a hardware register and updates a counter in a global variable before calling a routine to copy rows of glyph data from a source table to a destination buffer based on a character index.
- `render_text_glyph_rows`: Reads characters from a text buffer, offsets each by a fixed base to index a glyph table, and copies the glyph's bit-plane bytes into a screen bitmap in column-major order across a fixed number of rows and columns.
- `copy_data_via_offsets`: Executes a triple-nested loop that reads from a source table and writes to a destination table using offsets derived from a byte value.
- `update_state_and_parameters`: Decrements a global variable and, upon reaching zero, updates several fields in a structure and performs a lookup to set a value.
- `unpack_run_length_literals`: Unpacks a run-length stream in which each control byte either copies the following literal bytes to the output or repeats the next byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_table_entry_data`: Updates a table by writing a 32-bit value and a 16-bit value, then modifies a pointer using a bitwise OR operation.
- `call_and_return_registers`: Calls another routine and returns the concatenated result in registers.
- `update_audio_parameter`: Updates specific global variables using values retrieved from a calculated offset in a table.
- `calculate_memory_offsets`: Sets a flag and performs a series of arithmetic operations to store calculated memory offsets into several global variables.
- `update_audio_channels`: Clears audio registers if a stop flag is set; otherwise, it manages a step timer and updates the pointer, length, period, and volume registers for four audio channels before enabling DMA.
- `spin_on_volatile_byte`: Repeats an empty loop that compares a byte with itself, so it only ends if the hardware changes that byte between reads.
- `advance_channel_sequence_pointer`: Advances one sound channel's event pointer through its data.
- `interpret_music_channel_commands`: Interprets a music command stream for one channel, where special command bytes select instrument or sample data from a table.
- `initialize_four_sound_channels`: Disables the four audio channel interrupts, then initialises four channel structures from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_copy_code`: Clears a memory range, manages system state through various routines, and copies a block of code to a new location.
- `open_library_and_set_state`: Calls library functions to open a library and manage system state while interacting with DMA and interrupt registers.
- `update_input_and_copy_tables`: Reads a byte from a memory location and performs conditional arithmetic and bitwise operations on a global variable.
- `render_text_glyphs_to_bitplane`: Takes a null-terminated string from a global buffer and, for each character, copies 8 columns of a 32-byte-per-character glyph table entry into interleaved screen memory, writing two bytes per column with a 200-byte stride and a 40-byte offset.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `copy_data_from_table`: Performs a nested loop to transfer data from a table to a memory location using offsets derived from a global variable.
- `update_structure_parameters`: Decrements a global variable and, upon reaching zero, updates several fields in a structure using values from a table and a string.
- `unpack_rl_encoded_stream`: Decompresses a run-length encoded stream where control bytes determine whether to repeat a single byte a specified number of times or copy a sequence of literal bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Ghouls 'N Ghosts.adf:boot#unpacked

- `initialize_system_state_2`: Sets DMA and interrupt registers, clears a memory block, performs several subroutine calls to initialize hardware, and finally copies a block of data to a destination address before jumping to a long address.
- `initialize_system_state`: Calls several library routines, sets interrupt and DMA control registers, and then performs multiple loops to copy data from various sources into memory before jumping to a new address.
- `update_input_and_copy_data`: Checks a bit in the CIAA Port A Data Register and updates a global variable based on its value, then performs a series of nested loops to copy data from a source table to a destination buffer using calculated offsets.
- `execute_trap_and_invert`: Moves the stack pointer to a register, executes a trap, and then inverts the bits of a 16-bit register.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_structured_data`: Uses nested loops to copy bytes from a source table to a destination, calculating source offsets based on a value retrieved from a table.
- `update_blt_parameters`: Decrements a global counter and, when it reaches zero, calculates a destination address and updates several hardware registers and a structure in a6.
- `process_byte_stream`: Iterates through a source buffer, handling specific byte values by either skipping them, copying them, or performing arithmetic on them to determine how many bytes to move to a destination.

### Ghouls 'N Ghosts.adf:intro#unpacked

- `set_global_and_trap_2`: Stores a longword value into a global variable and executes a software interrupt, then returns.
- `initialize_system_and_update_state`: Sets the status register, writes a longword and a byte to a global variable, calls three different routines, updates three offsets in a structure pointed to by a register, and finally updates a longword in that structure by dereferencing a pointer found within a table before returning from an interrupt.
- `initialize_data_tables_2`: Populates several memory locations with specific values and copies data from a source table into a destination table using three separate loops.
- `write_structured_data_buffer`: Iterates through a buffer five times to write multiple multi-byte values into a structured memory block at fixed offsets using a subroutine.
- `write_structured_data_indexed`: Calculates a base address from a table using a value at a specific offset, then iterates five times to write data from two buffers into a structured memory block via a subroutine.
- `write_structured_data`: Iterates through a buffer to write multiple multi-byte values into a structured memory block at fixed offsets.
- `wait_and_reset_data`: Loops until a specific byte in a structure is no longer negative, calls another routine twice, writes 32 to an offset and 0 to another offset in a structure, and then loops while bit 6 of the CIAA_PRA register is set; once the loop exits, it writes 15 to the first offset and returns.
- `modify_and_branch`: Increments a longword in a global variable, performs a bitwise AND with 1 on that same value, and branches to another routine if the result is zero; otherwise it returns.
- `process_table_and_decrement`: If a 16-bit value at a specific location is 512, it sets it to 12800, then uses a value from a table to calculate an offset into another table to check a byte; if that byte is negative, it calls a routine to decrement a 16-bit global variable, otherwise it calls another routine.
- `increment_table_values`: Iterates through a buffer, increments bytes in a table at specific offsets if they do not equal a constant, and calls another routine after incrementing a global counter.
- `decrement_global_value`: Tests a 16-bit global variable and, if it is not zero, decrements it before returning.
- `reset_table_index`: Calls a subroutine, clears a global variable, increments another global variable, and if the value at a calculated offset in a table is negative, resets that variable to zero; it then calls another subroutine and writes two constants to global variables.
- `set_hardware_parameters`: Checks a bit in a memory location and, if not set, writes several constant values to a structured memory block at fixed offsets.
- `branch_to_address`: Performs a short branch to another routine.
- `call_and_return_double_long`: Calls another routine and returns the concatenated result of the two 32-bit registers as a single 64-bit value.
- `load_audio_parameters`: Calls a subroutine, and if a value in d0 is non-zero, calculates an offset into a table to load two words and two longwords into a global structure and calls another subroutine; otherwise it sets a global variable and modifies the DMA control register.
- `initialize_memory_pointers`: If a global variable is zero, it sets it to one, enables a bit in the CIAA Port A Data Register, and calculates and stores several memory addresses into a table using various arithmetic operations and multiplications.
- `update_audio_channels_2`: Clears the four audio channel values when a stop flag is set; otherwise it counts down a step timer and, at zero, reloads it and clears the channel state.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `advance_channel_sequence_pointer_2`: Checks if a sound channel's current event or step counter has expired, and if so, loads a new event entry from a table, updating the channel's pointer, step count, and frequency-related parameters.
- `interpret_music_channel_commands_2`: Iterates through a command stream to update a single audio channel's properties, such as pointer, period, and volume, or to switch instrument/sample data or check for a specific four-letter signature in a data block.
- `initialize_four_sound_channels_2`: Disables DMA and audio interrupts, then initializes four sound channel structures from a table by setting their pointers, volume, period, and status.

### Ghouls 'N Ghosts.adf:ghouls.prg (raw code file, load address unknown)

- `read_cia_button_port`: Accesses the CIA port A register, which holds the joystick and mouse fire buttons and disk status, and calls one other routine.
- `run_update_sequence_until_done`: Repeats a sequence of six routine calls, clearing a flag byte when it is set, until a counter reaches the value in a table.
- `run_twelve_step_update_cycle`: Runs a fixed sequence of seven routine calls twelve times in a row.
- `run_twelve_step_update_cycle_2`: Runs a fixed sequence of seven routine calls twelve times in a row.
