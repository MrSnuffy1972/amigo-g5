# Game 1652 (shanghai (1986)(activision)[m headbanger][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01652). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911904 top-level bytes in 124 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 66 of 66 units identical (bytes and relocations), 34 raw-word lines |
| Functions named and described | pass, 1356 of 1356 named (52 author symbols), 1356 of 1356 described (263 by rule, 1006 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1165 of 1165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 31628 of 31630 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 352251 |
| code | 107382 |
| data | 426732 |
| padding | 16323 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Shanghai (1986)(Activision)[m Headbanger][a].adf:headbanger | 95 | 3760 | 157 |
| Shanghai (1986)(Activision)[m Headbanger][a].adf:shanghai | 180 | 17140 | 545 |
| ced | 7 | 45 | 2 |
| AddBuffers | 1 | 101 | 1 |
| CLI | 11 | 607 | 25 |
| Add44K | 3 | 475 | 32 |
| csh | 6 | 45 | 2 |
| Degrader | 3 | 3038 | 91 |
| Shanghai (1986)(Activision)[m Headbanger][a].adf:Info | 2 | 117 | 2 |
| 16 | 1 | 2 | 1 |
| 9 | 1 | 72 | 2 |
| 12 | 1 | 51 | 2 |
| 8 | 1 | 2 | 1 |
| mathtrans.library | 18 | 830 | 97 |
| icon.library | 16 | 1444 | 110 |
| asl.library | 3 | 101 | 9 |
| mathieeesingtrans.library | 2 | 710 | 58 |
| mathieeedoubbas.library | 4 | 579 | 42 |
| diskfont.library | 13 | 819 | 66 |
| mathieeedoubtrans.library | 2 | 1043 | 52 |
| sound1 | 1 | 2 | 1 |
| sound2 | 1 | 2 | 1 |
| sound3 | 1 | 2 | 1 |
| sound4 | 1 | 2 | 1 |
| sound7 | 1 | 2 | 1 |
| sound8 | 1 | 2 | 1 |
| sound9 | 1 | 2 | 1 |
| extra | 13 | 0 | 1 |
| Shanghai (1986)(Activision)[m Headbanger][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 7 |

## Cases handled

- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- shanghai (1986)(activision)[m headbanger][a]/Shanghai (1986)(Activision)[m Headbanger][a].adf:c/ced: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- shanghai (1986)(activision)[m headbanger][a]/Shanghai (1986)(Activision)[m Headbanger][a].adf:c/csh: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_state_and_program_chips` (shanghai): Copies a 20-byte global block into the first argument, updates a global byte, and stores one bit chosen by the second argument with the third argument in the copied block. Kind: state initialization and hardware configuration. Pass path: the routine proceeds to the next instruction after the loop and register writes, which likely initiates the actual disk read/DMA process..

## Functions


### Shanghai (1986)(Activision)[m Headbanger][a].adf:headbanger

- `initialize_environment_and_open_device`: Checks a global variable to determine whether to perform a standard initialization or a task-based setup.
- `execute_callback_chain`: Checks several global pointers and, if non-zero, calls the routines pointed to by those locations or by a pointer found at an offset from a global variable.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `call_callback_and_store_result`: Loads a pointer into a1, calls a routine at an offset from a6, stores the result in a global variable, and returns if the result is zero.
- `initialize_graphics_and_fonts`: Opens the graphics and disk font libraries, initializes a view, bitmap, and rastport, allocates five rasters, clears them, and loads a font before calling a routine to load the view.
- `teardown_graphics_and_parse_string`: Releases graphics resources, closes libraries, and fonts, then parses a null-terminated string into a table of pointers.
- `handle_fatal_error`: Releases graphics resources and parses a format string into a table of pointers, then checks if the table is empty.
- `load_scroll_text_file`: Attempts to open a file and reads its contents into a table of pointers using a loop that parses delimited strings.
- `load_and_parse_asset_file`: Opens a file and reads thirty-two sets of three bytes to construct a table of sixteen 16-bit values by bit-shifting and combining the bytes.
- `parse_and_process_string_data`: Iterates through a null-terminated string to populate a table with pointers, then performs conditional logic based on the number of elements found to call various routines and manipulate a global variable via bitwise OR operations.
- `insert_and_sync_data`: Inserts a value into a table at a specific index by shifting subsequent elements and then calls a routine to synchronize the data.
- `parse_resource_table_and_open_files`: Iterates through a table of pointers to strings, copying valid entries into a new table while skipping entries with a specific bit set.
- `process_object_table_and_initialize_io`: Iterates through a table of pointers, checking a flag bit at offset 2707 of the pointed-to structure, and copies valid entries into a destination buffer while clearing the original entries.
- `initialize_audio_device`: Allocates a memory block for a device structure, creates a communication port, and opens the audio device.
- `manage_io_device_operations`: Initializes an I/O device and enters a loop that repeatedly calls _BeginIO and _CheckIO to manage data transfers.
- `parse_and_initialize_io`: Parses a null-terminated string into a table of pointers while skipping specific bytes, then performs conditional Amiga I/O operations including opening, inputting, or outputting based on the number of parsed elements.
- `clamp_and_update_structure_values`: Clamps a value between a minimum and maximum range, stores the result in a pointer, and updates two integer fields at offsets 34 and 34 of a structure.
- `parse_and_open_file`: Parses a null-terminated string from a memory location into a table while skipping specific characters, then uses the resulting string to open a file and performs input and output operations based on whether the table is empty.
- `display_scroll_text_and_exit`: Initializes graphics, allocates memory for a text buffer, and enters a loop that renders characters from the buffer to the screen while checking for an exit character.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `_CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_CheckIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `_AbortIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `tokenize_and_open_device`: It scans a text string, replacing each delimiter byte with a null and storing pointers to the resulting tokens in a table, then opens a device and reads from it, choosing its setup based on whether any tokens were found.
- `allocate_and_init_block`: Allocates a block sized to the input plus a constant, stores that size inside the block, passes the global block and the block address to another routine, and returns the block address plus a constant offset.
- `allocate_and_check_block`: Calls another routine, stores the result in a local variable, and returns if the result is zero; otherwise it returns.
- `return_minus_one`: Returns minus one without other work.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure by updating the next and head/tail pointers.
- `_Text`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `_SetFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `_CloseFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_LoadRGB4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `_InitRastPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_InitVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_MrgCop`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_MakeVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_LoadView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `_Move`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `_WaitTOF`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `_BltClear`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `_SetAPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `_InitView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_InitBitMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `_ScrollRaster`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `_AllocRaster`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `_FreeRaster`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `_FreeVPortCopLists`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `_FreeCprList`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `_GetColorMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `compute_binary_weighted_sum`: If either input is zero, it returns zero; otherwise, it iterates thirty-two times, doubling the first input and building a bitmask from its sign bit to conditionally subtract the second input and increment the first input.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `_Output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `_Exit`: Library stub: loads its arguments and the dos library base, calls Exit and returns its result to the caller.
- `_CreatePort`: The routine allocates a signal bit and a memory block, initializes fields in the block with parameters and the current task pointer, and either initializes a message list or adds the port to a global list.
- `_DeletePort`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `_BeginIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_table_entry_and_dispatch`: Walks a global table of 24-byte entries until it finds one whose flag field is empty or reaches the table end.
- `parse_command_string_2`: Parses a short command string whose second character selects a mode and whose first character selects a code, then calls one routine with the derived flags.
- `clear_object_links`: Calls one routine with the object when flag bit two is set, then clears its two pointer fields and flag byte, and finally passes a byte from the object to another routine.
- `fill_buffer_from_refillable_table`: Fills a destination buffer with bytes from a table by iterating through a specified count, advancing a pointer within a structure and calling another routine to refill the table when the pointer reaches zero.
- `emit_formatted_string_output`: Walks a percent-escaped format string, writing each literal byte and each expanded conversion to the output, and collapsing a doubled percent sign to one.
- `_OpenDiskFont`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `visit_global_table_indices`: Loops twenty times calling one routine to get an index and passing the global table address offset by that index to a second routine, then loops a variable number of times passing each counter value to the second routine, and finally forwards two parameters to a third routine.
- `copy_string_to_offset`: Calls a helper with the first argument to find an offset, then copies a NUL-terminated string from the second argument to that offset in memory, and returns the first argument.
- `copy_string_to_buffer_2`: Copies bytes from a source string into a buffer at an offset found through a helper, stopping at the NUL byte or when the count runs out, writes a terminating zero and returns the first argument.
- `check_flag_and_return_pointer`: Optionally calls a helper, then, if the index is in range and the table entry has its flag bit set, returns a pointer into the table.
- `open_file_with_flags`: Iterates through a table to find an entry where the first byte is zero, then sets specific flag bits in that entry based on the provided mode argument.
- `clear_status_flag_and_recurse`: Clears a status bit in the second argument and calls the routine again with the modified arguments, passing the first argument unchanged.
- `read_with_error_handling`: Obtains a file handle and reads the requested number of bytes into the buffer.
- `write_to_handle_checked`: Obtains a pointer via another routine and, if the pointer is not null and a specific bit is set, seeks to a position and writes a buffer of a given length, recording an error code and status if the write fails.
- `seek_file_with_offset_table`: Obtains a file handle and, for an index below three, seeks to an offset taken from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_file_entry`: Closes a file entry's handle unless a flag bit says not to, records an I/O error and minus one if the close fails, and clears the entry's flag byte.
- `delete_file_and_handle_error`: Attempts to delete a file and, if it fails, stores an error code in a global variable, sets a second global variable to two, and returns negative one.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_file_handle`: Opens the file named in the global text buffer with a fixed mode value and stores the resulting handle in a global variable.
- `read_file_byte_with_buffer`: Checks a global signal and a global file handle, then either returns a byte from a global variable after clearing it or reads a single byte from a file into a stack slot.
- `read_single_file_byte`: Reads one byte from the file handle into a stack slot with a single-byte read call, and returns that byte.
- `write_single_byte_file`: Writes a single byte argument to an output file handle after checking for error conditions.
- `check_signal_and_dispatch`: Reads the signal state for two bits, and when neither is set returns zero.
- `empty_stub_function`: The routine contains a block of 31 68000 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `_NewList`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `shift_global_variable_left`: Returns a global variable shifted left by three bits and performs no other operation.
- `call_with_value_twice`: Passes its single argument to another routine twice, as both parameters, and returns without further work.
- `find_node_by_distance`: Walks a linked list of nodes from a global head, moving by the requested distance and updating a global counter.
- `call_with_second_repeated`: Calls another routine with its first argument followed by its second argument repeated.
- `insert_or_update_linked_list`: Calls compute_signed_magnitude_division to determine an index, then traverses a linked list starting from a global variable to either insert a new node at a specific position or update an existing node's value and link.
- `read_control_structure_byte`: Checks status bits in a structure and, if valid, calls another routine to read data, updates the structure's status and data members, and returns the byte read or an error value.
- `write_byte_with_retry`: Writes a byte from its first argument into a buffer tracked by a state byte and a count field, updating the buffer position and counter.
- `reset_target_flags`: Returns zero immediately when the structure already has a target and no reset flag is set.
- `close_active_handles`: Loops over a table of six-byte entries up to a global count, closing each entry that is non-zero and does not have flag bit four set, then calls another routine with its original parameters.
- `_AutoRequest`: It invokes a routine stored at a negative offset within a base address held in a global variable and then returns.
- `count_string_bytes`: Scans forward from a pointer byte by byte until it reaches a zero byte, counting the bytes examined, and returns that count as the length of the string.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs.
- `init_intuition_request_2`: Opens the intuition library, stores the handle in a global variable, and performs an auto request with fixed dimensions.
- `parse_and_execute_command`: Checks a byte at a pointer for specific values, increments the pointer, and enters a loop that calls another routine while checking a bit in a memory location at an offset of an address from the pointer.
- `manipulate_buffer_and_branch`: Sets a byte to 1 at an offset of 44 bytes below a6 and then branches to another location.
- `set_buffer_status_and_branch`: Sets a value of 1 at an offset of 44 bytes below a6 and then branches to another location.
- `process_buffer_and_update_counters`: If a pointer at a negative offset of a6 is non-zero, it performs a series of loops to copy bytes from a source buffer into a destination buffer, using counters at various offsets within the structure to track progress.
- `decrement_and_store_byte`: Decrements a 16-bit value at an offset in the stack frame, and if the result is not negative, stores a byte from another stack offset into a destination address calculated from the stack frame.
- `decrement_and_store_byte_2`: Decrements a 16-bit value stored on the stack and, if the result is not negative, stores a byte from a stack offset into a destination address derived from other stack values.
- `loop_decrement_and_store_bytes`: Decrements a 16-bit value in the stack frame and, while the value is not negative, loops to store bytes from a source pointer into a destination pointer, incrementing both pointers and the counter each iteration.
- `copy_value_and_increment_counter`: Stores a value from a stack slot through a pointer held in the frame, then returns a counter from the frame plus one.
- `set_numeric_length`: Compares a long value at offset negative 28 of a6 with negative one; if not equal, it sets a long value at that offset to 6, otherwise it compares it with 20 and sets the value at that offset to either 19 or 13.
- `format_numeric_string_buffer_3`: Calculates the required length for a numeric string based on sign, integer, and fractional parts, then writes the resulting ASCII characters to a destination buffer.
- `format_float_to_string`: Calculates the required string length for a floating-point number based on its sign, integer part, and fractional part, then writes the formatted ASCII representation to a destination buffer.
- `modify_stack_pointer`: Loads a longword from an offset of minus 28 from a pointer in a6, adds 6 to it, stores the result back to an offset of minus 4 from the same pointer, then checks a longword at offset minus 40 from a6 to conditionally negate it and branch.
- `format_floating_point_string`: Converts a floating-point value into an ASCII string by handling the sign, integer part, decimal point, and fractional part.
- `format_numeric_string_buffer_2`: Writes a sequence of ASCII characters, including hyphens, periods, and digits, into a destination buffer pointed to by an offset in a6.
- `update_char_and_branch_2`: Decrements a value at an offset in a6, and if the result is not negative, copies a byte from one pointer to another and branches to another routine; otherwise it branches to another routine.
- `update_char_and_branch`: Stores the byte value 48 at the address pointed to by an offset in a6, increments that address, and branches to another routine.
- `update_buffer_and_copy_data_2`: Decrements a counter in a structure and loops to write bytes from a source pointer to a destination pointer, optionally appending specific bytes based on the state of other structure members.
- `write_byte_and_increment_pointer`: Writes the byte value 48 to the memory location pointed to by the value at offset 12 of the structure in a6, then increments the pointer at offset 12 of the structure in a6 by 1 and branches to another routine.
- `update_buffer_and_copy_data`: Decrements a counter in a structure and loops to write bytes from a source pointer to a destination pointer, optionally appending specific byte values.
- `increment_pointer_and_store_byte`: Loads a pointer from an offset of 12 in a structure in a6, stores the value 48 at that pointer, increments the 32-bit value at offset 12 of the structure, and branches to another routine.
- `copy_byte_from_source`: Decrements a value at an offset of 12 from the stack pointer, and if that value is not negative, copies a byte from a source pointed to by an offset of 48 from the stack pointer to a destination pointed to by an offset of 12 from the stack pointer before branching to another location.
- `format_numeric_string_with_suffix`: Writes a decimal representation of a value to a buffer, including a sign character and a specific character constant, then enters a loop that calls update_char_and_increment_pointer and write_signed_digit_character to populate a temporary array with digit characters.
- `update_char_and_increment_pointer`: Writes the byte forty-eight to a memory location pointed to by an offset of twelve in a structure in a6, increments a longword at offset twelve of that same structure by one, and branches to another routine.
- `write_signed_digit_character`: Writes the character 69 to a pointer from an offset in a6 and increments it, then if a value at an offset in a6 is negative, writes the character 45 to the pointer and increments it; otherwise it writes 69 and increments it.
- `convert_and_copy_digit_buffer`: Iteratively calls another routine to convert values into ASCII characters stored in a local buffer, then copies a sequence of bytes from that buffer to a destination pointer if specific conditions are met.
- `process_buffer_data`: If certain conditions are met, decrements a value in a structure, then loops to copy bytes from a source offset to a destination pointer while incrementing both, finally storing a value at a pointer in the structure and returning a value from the structure plus one.
- `copy_value_and_increment_counter_2`: Stores a value from a stack slot through a pointer held in the frame, then returns a counter from the frame plus one.
- `convert_integer_to_string_2`: Converts a numeric value into a decimal string representation by repeatedly dividing by ten and storing the resulting ASCII characters in a buffer.
- `format_numeric_string_2`: Converts a numeric value into a decimal string representation by repeatedly dividing by ten and storing the resulting ASCII characters in a buffer.
- `increment_and_call_routine`: Calls another routine and increments a 16-bit value at offset 6 of a structure in a1, then branches to another routine.
- `update_and_shift_values`: Adds 4 to a value at offset 4 of a structure in a1, calls decrement_loop_until_zero and multiply_64bit_by_ten, decrements a value at offset 6 of the structure, and enters a loop that performs bit shifts and decrements based on the sign of d0.
- `check_and_multiply_values`: Clears d6, pushes d0 onto the stack, performs an OR operation, and if d0 is not zero, calls multiply_64bit_by_ten; it then checks d6 and d2 to decide whether to decrement a value at offset 6 of a structure in a1 before branching.
- `convert_integer_to_string`: Converts a numeric value into a decimal string representation by repeatedly dividing by ten and storing the resulting ASCII characters in a buffer.
- `decrement_loop_until_zero`: Decrements a 16-bit counter in a loop until it reaches zero and then returns, doing no other work.
- `multiply_64bit_by_ten`: Multiplies a 64-bit value held in two 32-bit registers by ten, returning the high word from the upper input and carry bits and the low word as the lower input times ten.
- `shift_bits_into_decimal_digits`: Shifts a 64-bit value left one bit at a time for a fixed count of 64 steps, carrying each bit into a decimal digit accumulator and subtracting ten with a carry into the high word whenever the digit exceeds nine.
- `store_param_to_global`: Copies its single 32-bit argument into a global variable and returns, performing no other operation.
- `mask_and_clear_registers`: Copies d0 to d2, masks d0 with a constant, and if the result is not zero, branches to another routine; otherwise, it clears d0, d1, and d2 before branching to another routine.
- `shift_sign_bits_to_result`: Shifts a value left ten times, collecting the bit shifted out at each step into a result register, then returns that result with its top bit set.
- `process_bitfield_data`: Checks if two 32-bit values are non-zero, performs a series of bitwise shifts and arithmetic adjustments on those values within loops, and calls another routine with either 1 or 2 as an argument based on the resulting calculations.
- `normalize_and_shift_bitfield`: Performs a bitwise left shift on two 32-bit values within a loop until a specific bit is set, then applies arithmetic adjustments and conditional calls to another routine based on the resulting value before returning a concatenated 64-bit result.
- `call_routine_and_clear_d0`: Pushes the constant one onto the stack, calls another routine, clears d0, and branches to another routine.
- `manipulate_and_store_word`: Pushes the value 2 onto the stack, calls another routine, performs bitwise operations involving a swap and an OR on d2 and d0, clears d0, adjusts the stack pointer, and clears d1 before branching to another routine.
- `execute_library_calls`: Loads a pointer from a global variable into a7, calls the routine at that address, pushes the constant 20 onto the stack, and calls another routine.

### Shanghai (1986)(Activision)[m Headbanger][a].adf:shanghai

- `initialize_system_environment_2`: Checks a global variable to determine whether to perform a standard initialization or a task-based setup.
- `execute_callback_functions`: Checks if two specific bytes are non-zero, and if so, calls two function pointers stored at offsets in a structure, then calls two more function pointers from a global variable and another from a global variable.
- `invoke_two_function_pointers_2`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `fetch_and_store_dos_result`: Loads a pointer into a register, calls a function, stores the returned value into a global variable, and branches to another routine if the result is zero; otherwise it returns.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `parse_string_table`: Splits a command string into tokens separated by spaces, keeping double-quoted text as one token and storing a pointer to each token in a table.
- `allocate_and_init_block_2`: Allocates a block sized to the input plus 12, stores that size inside the block, passes the global block and the block address to another routine, and returns the block address plus 12.
- `call_subroutine_and_store_result`: Calls another routine and stores its return value at offset minus 4 of the stack frame, then returns if the result is not zero.
- `return_minus_one_2`: Returns minus one without other work.
- `free_linked_memory_blocks_2`: Iterates through a singly linked list starting from a global pointer, calling a routine to free each node using the size stored at an offset within the node before following the next pointer to the subsequent node.
- `append_node_to_list_2`: Appends a new node to a linked list by updating the next pointer of the current tail and updating the head and tail pointers within the list structure.
- `open_required_system_libraries`: Opens the diskfont, graphics, icon, intuition, and layers libraries, raising a specific alert code if any fail to open.
- `allocate_fixed_size_buffers`: Allocates several fixed-size blocks through library calls, including blocks of 400, 880, 200 and 220 bytes, and raises alert code 10 for each allocation that fails.
- `initialize_game_resources`: Allocates large display buffers and five game buffers, reporting error codes 21, 31, or 41 if any allocation fails.
- `load_winter_tile_resources`: Loads three named resources from the winter directory, Doc.Paper, green.felt and tile2, through a loader routine.
- `load_title_graphic`: Loads a title graphic by passing a pointer to a global variable and several parameters to a routine that copies picture rows into a buffer.
- `trace_rectangle_outline_calls`: Issues five calls that take the four parameters in turn, tracing the corners of a rectangle and closing back at the first corner.
- `report_open_failure_error`: Looks up a numeric code in a table and, when it is recognised, reports an error through a formatted message.
- `check_range_bounds`: Compares a value against a range defined by a table and checks if another value falls within a second range defined by that table, returning 1 if all conditions are met and 0 otherwise.
- `clear_a6_and_return`: Clears register d0 and unlinks a6, then returns.
- `compute_bit_masks_once`: On first use it calls a routine with a fixed value and sets a flag.
- `poll_request_completion_status`: Reads a request record through a stored pointer and copies its header into a local buffer.
- `run_setup_sequence_until_ready`: Runs a fixed series of setup calls that configure a display or device, repeating one query until it returns a nonzero value.
- `copy_picture_rows_to_buffer`: Copies a specified number of 40-byte rows from a source table into a local buffer by calculating each row's address using a width and offset, then passes the buffer, a calculated size, and a bitmask derived from the row count to another routine.
- `emit_packed_word_fields`: Walks a range of 16-bit words, splitting each into a high byte and two four-bit fields.
- `reset_thirty_two_entries`: Calls one library routine for each index from zero to thirty-one, passing zeros as the remaining arguments.
- `flush_format_stream_state`: Passes the global stream state to a helper that refills or flushes the buffer, then returns.
- `call_stream_handler`: Pushes a global variable onto the stack and calls a routine at an offset relative to the value stored in that variable, then returns.
- `select_record_and_set_mode`: Reads a 16-byte table entry chosen by the argument and passes its fields to a routine.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `emit_formatted_string_output_2`: Walks a percent-escaped format string, writing each literal byte and each expanded conversion to a global buffer, collapsing a doubled percent sign to one.
- `open_timer_device_request`: Creates a message port and an I/O request block, then opens timer.device with that request.
- `prepare_timer_request_block`: Fills a timer request block with command code nine, zeroes one field, and stores a value in another.
- `release_request_structure`: For a non-null argument, passes the structure's embedded pointer to a helper if it is set, then calls a library routine on the structure.
- `release_two_global_handles`: Calls one library routine on the handle at offset eight, then on the handle at offset four, then on the first handle again.
- `open_input_device_request`: Creates a message port and an I/O request block, then opens input.device with that request.
- `send_control_request_with_buffers`: Runs a library call with a size of 256 for each of two handles, fills a request block on the stack with a command code and a pointer, and sends it.
- `apply_mode_table_entry`: Applies the 16-byte table entry selected by the argument, clamping any index above one to zero.
- `store_bit_mask_result`: Computes a single-bit mask from a byte held in a structure reached through a global, and passes that mask to a routine.
- `run_winter_intro_animation`: Loads the winter picture files and sets up colour and timing tables, then draws sprite rows in staged loops.
- `draw_winter_frame_sprites`: Draws a set of sprites at positions derived from two timing counters, using one of two layouts depending on a flag.
- `repeat_routine_by_count`: Calls one routine repeatedly, the number of times given by the argument, and returns.
- `iterate_global_table_and_indices`: Loops twenty times calling a routine to obtain an index and passing a global table address offset by that index to another routine, then loops a variable number of times based on a global variable to pass each counter value to that same routine, and finally passes two parameters to a third routine.
- `build_sprite_entry_table`: Fills a table of 34-byte entries, one per item in four groups whose sizes come from a global array, linking each entry to its position and a pointer.
- `retry_library_until_nonzero`: Calls a library routine repeatedly with a global argument until it returns a nonzero result.
- `mark_list_entry_character`: Follows a linked list from a head pointer for a given number of steps, then writes either a space or an asterisk into that node's text buffer, depending on the flag argument.
- `copy_bytes_forward`: Copies the given number of bytes one at a time from a source buffer to a destination buffer.
- `store_long_in_linked_field`: Writes a 32-bit value into a field of the structure that a pointer at offset twenty-six of the argument refers to.
- `blt_bit_map`: Calls the library routine 30 bytes below the base stored in a global, then returns.
- `call_library_vector_54_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_282_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `allocate_five_game_buffers`: Allocates five buffers of 2560, 260, 36, 160 and 1024 bytes with memory-type flags, reporting an error code if any allocation fails.
- `call_routine_with_fallback`: Calls a routine with the fixed argument four and two stored values, then subtracts one from its result.
- `request_numbered_resource`: Makes a library call with two stored pointers, the identifier 1830 and a flag of one, and passes any nonzero result to a second routine.
- `call_with_fallback_five`: Calls a routine with the fixed argument five and two stored values, then subtracts one from its result.
- `update_tile_list_and_handle_input`: Updates a list of structures by populating pointers to a data table based on a global index and a loop of six iterations.
- `update_game_state_and_render`: Iterates through a list of structures to update a global index and performs conditional logic based on returned values, including updating a global counter, modifying a pointer to a table of elements, and calling drawing or state-update routines.
- `update_pointer_and_increment`: Stores a byte from a pointer into a memory location, increments a value on the stack, and branches to another routine.
- `process_event_queue_and_update_state`: Iterates through a linked list of entries, updating a global index and calling various routines based on the value returned from each entry.
- `draw_frame_around_structure`: Computes a rectangle from fields of a structure, offset by fixed amounts, and passes its four coordinates to a drawing routine.
- `draw_rectangle_border`: When a stored item is present, fills a rectangle inside the given corners and draws its outline with eight line-drawing calls.
- `redraw_stored_item_region`: Runs three setup calls, then calls routines on the stored item's field and on a second global offset to redraw its region.
- `bracket_routine_with_handle_calls`: Brackets a call to another routine between two calls on a stored handle, one passing zero beforehand and one passing minus one afterwards.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `forward_library_call`: Passes two global pointers and a global address to a library routine 174 bytes below the library base, then returns.
- `parse_and_process_signed_table`: Skips leading spaces in a string, selects a record from a table, and iterates through a list of signed short values.
- `process_signed_value_table`: Iterates through a table of signed short values, checking if a specific field is negative, and if so, performs a search and stores the resulting value into a destination buffer while incrementing a counter.
- `filter_signed_values`: Calls two library routines to examine data, checks for a specific byte value, and then enters a loop to process and store signed short values into a table.
- `filter_and_store_signed_values_2_2`: Iterates through a list of records, performing signed magnitude division on each and storing the resulting short value into a table at an offset.
- `filter_and_store_signed_values_2`: Iterates through a list of records, performing signed magnitude division on each and storing the resulting short value into a table at an offset, while handling string comparisons and DOS library calls.
- `filter_and_store_signed_values`: Iterates through a list of records, performing signed magnitude division on each and storing the resulting short value into a table at an offset determined by a global counter.
- `process_file_records`: Enters a loop that selects records from a table, performs signed magnitude division, and updates a global counter and a list of values.
- `unlock_listing_and_return`: Unlocks the lock held in the frame and returns three to its caller.
- `store_pointer_to_offset`: Loads a pointer from a global variable, then stores a second global pointer into the address at offset 184 of the first pointer's target.
- `build_game_file_path`: Builds a full path from a directory and a file name, adds a separator when needed, then locks the result.
- `read_or_write_data_file`: Opens a file through a library call chosen by the mode argument, then reads or writes a block of data with it.
- `request_resource_in_current_dir`: Locks the current directory, makes the numbered resource request with the given pointer, then unlocks the directory.
- `calculate_sum_of_table_values`: Performs a 32-bit division, then iterates through a table to accumulate its values into a running total, and finally compares this total against a previously calculated sum.
- `accumulate_table_values`: If a returned value is non-zero, it iterates through a table using a counter, adding values from the table to a running total stored in a structure, and calls another routine if the final total does not match a specific value.
- `compare_stored_totals`: Compares two stored totals and calls a routine when they differ, then calls a second routine.
- `apply_stored_option_settings`: Reads several stored option flags and selects matching setup calls for each one, then loops over four slots.
- `invoke_record_for_large_key`: Searches a table of three-field records for a matching key value.
- `search_record_table`: Iterates through a table of three-word records, comparing a field in each record to a provided parameter, and calls another routine with a field from the matching record if the search succeeds.
- `invoke_record_when_within_count`: Passes a field of a stored record to another routine when a value at offset minus 8 of a structure is at most a value at offset minus 12 of that structure.
- `search_string_table`: Compares a string against a table of strings, checking for equality and null terminators within a loop.
- `compare_stack_bytes`: Compares two bytes from the stack and, if they are equal, writes a one to a stack location and increments another stack value before branching to another routine.
- `load_offset_from_a6`: Loads a longword from offset minus 16 of the address in a6 into d0, unlinks a6, and returns.
- `convert_char_to_uppercase`: Returns the byte unchanged unless it is a lowercase letter, in which case it returns the matching uppercase letter.
- `accumulate_file_info_value`: Locks a named path and reads its information block into a 36-byte buffer.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_150_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_info`: Library stub: loads its arguments and the dos library base, calls Info and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `open_audio_device_channels`: Allocates a 1496-byte block and a message port, reporting an error code of 400 if either fails.
- `test_and_return_long_pointer`: Tests if a long value at a fixed offset from the stack pointer is zero; if it is not zero, the routine returns, otherwise it returns.
- `iterate_and_call_pointers`: Reads a 16-bit counter from a location relative to a6, then loops up to ten times, calling another routine with a pointer from a table if that pointer is non-zero.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `start_sound_voices`: Clamps a level argument to a maximum of three and selects five or nine voices based on whether the argument is two or not.
- `start_sound_from_table`: Clamps a level argument to eight, releases the channel bits held in a mask, then allocates free channel bits for each entry of the table selected by that level.
- `process_audio_event`: Sets a 16-bit value at offset 926 of a global variable to 8, sets a byte at offset 930 to 0, and performs several library calls using values from a global variable before returning.
- `trigger_audio_event`: Tests a long value at an offset of 174, and if it is not zero, loads a pointer from a global variable and its offset 14, moves a byte from offset 15 of that pointer into d0, shifts it left by one, pushes the result onto the stack, and calls another routine.
- `unlink_a6`: Unlinks a6 and returns immediately.
- `create_port_and_initialize_fields`: Calls a routine to allocate a signal bit and a memory block, then initializes the block's fields at offsets 8, 9, 10, 14, and 15 with the constant 4, a provided byte, a provided integer, zero, and the signal bit respectively.
- `delete_port`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `execute_no_effect_sequence`: Executes a sequence of instructions that performs no observable changes to registers, memory, or I/O before returning.
- `process_system_state`: Executes a series of conditional checks and calls various routines to manage global counters, file entries, and drawing operations, including an indirect jump to a routine at an offset from a value in a register.
- `clear_data_register_and_branch`: Clears the least significant byte of the data register zero and performs a relative branch to a fixed offset.
- `update_global_counter`: Compares a 16-bit word from offset 24 of a structure in a4 with a global variable, increments a global variable if they match, and clears a long word at a global variable if bit 7 of the structure word is set.
- `test_and_clear_word`: Loads a 16-bit word from offset 24 of the structure in a4, tests bit 7, and if set, clears a long word at a global variable before branching.
- `scan_game_file_entries`: Iterates through a directory listing to find files with specific extensions, specifically .slt and .trn.
- `delegate_single_helper_call`: Calls one helper with no arguments and returns.
- `copy_string_to_offset_2`: Calls a helper to find an offset from a pointer, then copies a NUL-terminated string from a second argument to that offset plus the first argument, and returns the first argument.
- `copy_string_to_offset_buffer`: Calls a routine to determine a length, calculates a destination address by adding that length to the first argument, and then copies bytes from the second argument to that destination until a zero byte is encountered or the third argument reaches zero.
- `expand_format_string`: Walks a percent-escaped format string, writing each literal byte and each expanded conversion to a bounded output buffer through a count-checked store, handling doubled percent signs and escape sequences.
- `evaluate_linked_list_flags`: Loads a pointer from offset 8 of a structure in a6 into a0, tests bits 4 and 5 of the byte at offset 16 of that pointer, and if both bits are zero, it loads the value 255 into d0 and returns; otherwise it returns immediately.
- `process_linked_list_element`: Checks flags in a structure pointed to by a value at offset 8 of a6, calls another routine with data from that structure, and updates the structure or a linked list based on the result.
- `check_status_bit_2`: Loads a pointer from offset 8 of a structure in a6, tests bit 1 of a byte at offset 16 of the pointed-to structure, and if the bit is clear, stores a value in a location at offset minus 8 of the structure in a6.
- `check_status_bit`: Loads a pointer from offset 8 of the structure in a6 into a0, reads a byte from offset 16 of the structure pointed to by a0, and tests bit 1 of that byte; if the bit is not set, it sets d0 to 255, unlinks a6, and returns; otherwise it returns.
- `read_and_increment_stream`: Sets a bit in a status byte, calls a routine to read data into a buffer, updates a length field, and then either decrements a counter or increments a pointer and returns the next byte from the buffer.
- `decrement_stream_length`: Decrements a 32-bit value at an offset of 4 from a structure pointed to by a6, and if the result is non-negative, loads a byte from a new pointer and updates the structure.
- `forward_stream_to_helper`: Passes the stream pointer from the frame to a helper routine and returns.
- `write_byte_with_retry_2`: Writes a byte from a register into a buffer tracked by a pointer and a remaining count field within a structure, updating the position and decrementing the count.
- `reset_target_flags_2`: Returns zero immediately if the structure at a6 has a target at offset 18 and bit 3 of the byte at offset 4 is not set; otherwise, it calls a helper with a global pointer, stores the result in the first and fourth 32-bit words of the structure, clears bit 3 and bit 4 of the byte at offset 4, resets the 16-bit words at offsets 4 and 6, and sets a global variable to 12 on failure.
- `execute_conditional_command`: Checks for specific characters at the start of a buffer, skips them if present, and enters a loop that increments the pointer while checking a bit in a structure, eventually calling a routine via a pointer found in a table.
- `merge_data_buffers_conditionally`: Sets a flag in a structure and performs a conditional merge of two data buffers by copying bytes from one memory location to another using a loop.
- `merge_and_copy_data_buffers_2`: Sets a flag in a structure and performs a conditional merge of two data buffers by copying bytes from one memory location to another using two loops, then updates a length value in a structure and returns.
- `merge_and_copy_data_buffers`: If a value at a specific offset in a structure is non-zero, it performs a series of loops to copy bytes from one memory location to another, alternating between two source buffers based on a condition, and updates a length counter.
- `decrement_and_copy_byte`: Decrements a longword at offset -36 of the stack frame, and if the result is not negative, copies a byte from offset -32 of the stack frame to a destination address calculated from offsets -4 and 12 of the stack frame, then branches back to the start of the routine.
- `decrement_and_copy_byte_3`: Decrements a longword at offset minus 36 of the stack frame; if the result is non-negative, it copies a byte from an address derived from the stack to another address derived from the stack and loops.
- `decrement_and_copy_byte_2`: Decrements a longword stored at a negative offset from the stack pointer, and if the result is non-negative, copies a byte from a source address pointed to by a value at offset negative 48 from the stack pointer to a destination address calculated from a value at offset negative 4 from the stack pointer, then increments both pointers and loops back to the start of the routine.
- `copy_stack_value_and_increment_counter`: Stores a longword from a stack slot into a destination address pointed to by a value in the stack frame, then returns the value at offset 8 of the stack frame incremented by one.
- `update_numeric_length`: Compares a long value at offset negative 28 of a structure in a6 to all ones; if not equal, it branches to another routine, otherwise it sets a long value at that same offset to 6, compares it to 20, and if it is less than 20, sets the value to 19 and branches to another routine.
- `format_numeric_string_buffer_4`: Calculates the required length for a numeric string representation, including sign and decimal components, and writes the resulting ASCII characters into a destination buffer.
- `format_scientific_notation_string_2`: Calculates the length and components of a numeric value to construct a scientific notation string, writing a sign, mantissa, decimal point, exponent, and the character 'E' into a destination buffer pointed to by a structure in a6.
- `adjust_offset_value`: Adds a constant to a value at offset minus 28 of a structure in a6, and if that value is negative, negates it before branching.
- `format_scientific_notation_string`: Converts a numeric value into a scientific notation ASCII string by writing a sign, a mantissa, a decimal point, and an exponent suffix into a buffer.
- `format_floating_point_string_2`: Converts a numeric value into an ASCII string representation within a buffer, handling sign characters, decimal points, and scientific notation formatting.
- `decrement_and_copy_byte_loop`: Decrements a longword at offset minus 12 of the stack frame; if the result is non-negative, it copies a byte from an address at offset minus 48 of the stack to an address at offset 12 of the stack, increments both addresses, and loops.
- `write_byte_and_branch`: Writes the decimal value 48 to the address pointed to by a value at offset 12 from the stack pointer, increments that address by one, and branches to another routine.
- `format_numeric_buffer`: Decrements values in a structure and writes ASCII characters to a buffer, including handling a sign and a decimal point.
- `write_ascii_char_and_increment_pointer`: Writes the byte value 48 to the memory location pointed to by the value at offset 12 of the structure in a6, then increments the pointer at offset 12 by 1 and branches to another routine.
- `update_status_and_buffer`: Decrements a long value at offset -28 of a structure in a6 in a loop, and if a long value at offset -4 is less than a long value at offset -36 and a long value at offset -16 equals 1, it decrements a long value at offset -36 and copies bytes from a buffer to a pointer at offset 12 of the structure.
- `update_status_byte`: Loads a pointer from an offset of 12 in a structure in a6, stores the byte 48 at that pointer, increments a long value at offset 12 of the structure, and branches to another routine.
- `copy_byte_from_table`: Decrements a value from the stack, and if it is not negative, copies a byte from a table at an offset in a0 to a destination at an offset in a1.
- `format_numeric_string_3`: Converts a signed integer into a decimal string representation within a buffer, handling sign characters and digit conversion via a loop.
- `update_status_byte_and_increment_counter`: Writes the value forty-eight to a byte at the address stored in a pointer at offset twelve of the structure in a6, increments a longword at offset twelve of that same structure, and branches to another routine.
- `write_sign_and_buffer`: Writes a character to a buffer at an offset from a6, then checks a value relative to a6 to either write a minus sign and negate the value or continue.
- `convert_decimal_and_copy_buffer`: Calls another routine within a loop to convert values into ASCII characters stored in a local buffer, then performs a 32-bit signed division via another routine.
- `transfer_buffer_data`: If a long value at offset -16 of a structure in a6 equals 1 and a long value at offset -4 is less than a long value at offset -36, it decrements the value at offset -36 in a loop, copying bytes from offset -32 to a pointer at offset 12 and incrementing both the pointer and the value at offset -4.
- `copy_stack_slot_and_increment_counter`: Stores a value from a stack slot through a pointer held in the stack frame, then returns the value at offset 8 of the stack frame incremented by one.
- `create_ext_io`: Allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: Clears specific fields in the I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `create_std_io`: The routine calls another routine to create an extended I/O structure for the supplied pointer, passing a constant size argument of 48, and then returns.
- `delete_std_io`: The routine receives a pointer argument and forwards it unchanged to another routine that deletes an extended I/O, then returns.
- `combine_pair_into_value`: Calls a routine that fills two values, then returns the first multiplied by two hundred and fifty-six plus the second shifted right by twelve bits.
- `draw_menu_list_items`: Selects a 30-byte list record and stores a value in its text field.
- `draw_cursor_at_index`: Computes a cursor position from the current index and a stored offset, then passes that position to two drawing routines.
- `draw_dual_framed_panels`: Calls clip_and_draw_rectangle twice to draw two framed panels using provided coordinate pairs and fixed dimensions of twenty and twenty-three, once with a specific color value and once with a different color value.
- `draw_four_corner_markers`: Sets a mode value of fifteen, then draws four markers at the corners of a rectangle offset by nineteen and twenty-two units from the given position.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_entry_flag_bit`: Looks up an entry by index, then clears bit sixteen in its flag word.
- `clear_entry_flags_masked`: Looks up an entry by index, then clears the bits given by a mask from its flag word.
- `find_list_node_at_index`: Follows the next pointers of a linked list from the head pointer stored at an offset, for the given number of steps.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `store_four_word_fields`: Writes four 16-bit values into consecutive fields of the record at the pointer argument.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `next_pseudo_random_word`: Shifts a 32-bit state left by one bit and XORs in a feedback constant when the top bit was set, then returns the low sixteen bits.
- `count_list_entries`: Counts the nodes of the linked list whose head pointer is stored at a fixed offset in the structure argument.
- `draw_entry_when_visible`: Checks if the high bit of a 16-bit word at offset 38 of a structure is clear; if so, it calls a routine to draw a rectangle around the coordinates at offset 4 and 6 of that structure and then calls another routine, otherwise it calls a routine on the structure.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `draw_two_screen_bands`: Draws two horizontal bands across a screen width of 320 units at fixed positions, each followed by a second drawing call.
- `set_entry_highlight_state`: Sets or clears the high flag bit of an entry, calling library routines when the state changes.
- `draw_three_screen_bands`: Draws three horizontal bands across a screen width of 320 units at fixed offsets, each followed by a second drawing call.
- `draw_text_string`: Sets drawing mode values, measures the length of a null-terminated string, then draws the string with a library call.
- `draw_text_with_colour`: Measures a null-terminated string and draws it, but takes the drawing mode from its second argument.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `forward_two_word_arguments`: Passes its two 16-bit arguments to a routine and returns.
- `copy_eight_bytes_to_local`: Copies eight bytes from the source pointer into a local buffer through a loop.
- `draw_clamped_cell_rectangles`: Measures distances from a position to the neighboring cell boundaries using a 16-bit table, clamping negative results to zero.
- `check_bounding_box_intersection_2`: Calculates several values using a global table and a pointer, then performs multiple conditional checks to determine if a bounding box intersection occurs.
- `check_bounding_box_intersection`: Calculates two non-negative values by finding the difference between entries in a global table and a pointer, then performs multiple conditional checks to determine if a rectangular area defined by a pointer at offset 12 of a structure overlaps with a bounding box defined by four integer values at offsets -16, -20, -24, and -28.
- `validate_and_calculate_overlap`: Calculates a value based on a difference between two 16-bit values from a global table and a pointer, ensures the result is not negative, and then checks if a rectangular area overlaps with a bounding box defined by a pointer in a structure.
- `validate_rect_overlap_2`: Checks if a rectangular area defined by a pointer in a structure at offset 12 in a6 overlaps with a bounding box defined by values within the structure.
- `validate_rect_overlap`: Checks if a rectangular area defined by a pointer in a structure at offset 12 in a6 overlaps with a bounding box defined by values within the same structure.
- `check_bounds_intersection`: Compares a pointer and values from a table against four boundaries stored in a structure in a6 to return one if the range is within the bounds, otherwise it returns zero.
- `check_bounds_in_structure`: Compares a pointer from a structure in a6 against several values in a global structure and returns one if all conditions are met, otherwise it returns zero.
- `return_zero_and_restore_registers`: Clears register d0 and restores the previous register state from the stack before returning.
- `lookup_grid_neighbour_value`: Computes a neighbouring cell index on a fifteen-column grid from a base index and offsets.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `mark_entry_and_refresh_rectangle`: Sets the high flag bit of a sprite entry, computes a rectangle enlarged by a few units around its position and size, and passes it to a drawing routine after a library call.
- `forward_record_position`: Passes the first two 16-bit fields of a record, together with a further argument, to a routine, then returns.
- `forward_zero_argument`: Calls one routine with the argument zero and returns.
- `advance_record_position`: Increments two fields of a record by one, then passes the record and a value to another routine.
- `draw_bordered_frame_pattern`: Draws a patterned border around a rectangle described by a record, repeating the edge segments once per step of a count derived from a global value.
- `draw_record_rectangle`: Copies the four 16-bit fields of a record and passes them, with two of them as coordinates, to a drawing routine.
- `draw_layered_record_rectangle`: Draws a rectangle described by a record in several passes, each with a different mode value, using the record's position and size.
- `clip_and_draw_rectangle`: Clips a rectangle's width and height to the bounds held in a structure, then passes the clipped values to a drawing routine.
- `store_argument_in_shared_word`: Stores its 16-bit argument into a shared variable and returns.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `execute_dos_open`: Calls a routine to open a file and returns the result in d0, otherwise it returns.
- `write_multiple_blocks`: Loops four times to call a write routine with data from a table, then calls a write routine with a global variable and a size of thirty-two, and finally calls a write routine with no arguments.
- `write_header_and_close`: Writes a 32-byte block from a global to an open file and then closes it.
- `draw_fixed_marker_set`: Sets a draw mode of four, then draws a fixed set of eight markers at stored positions offset by fixed amounts.
- `set_global_flag`: Compares a passed pointer to a global variable and sets a byte in a global variable to one if they match.
- `compute_signed_magnitude_division_2`: Performs a 32-bit restoring division algorithm on the absolute values of two inputs, returning the quotient with a sign determined by the XOR of the original inputs and returning zero if the divisor is zero.
- `round_trip_value_through_helper`: Calls a helper in mode zero with the address of a local slot, makes two further helper calls, stores a register value into the slot, calls the helper again in mode one, and returns the slot's value.
- `call_helper_in_mode_one`: Calls a helper with mode one and the address of a stack slot, then returns; it passes the slot address without reading or writing it.
- `transfer_global_word_by_flag`: When the flag argument is zero, stores the global word into the location pointed to by the second argument.
- `check_status_byte`: Initializes a local variable to zero, compares it to 480, and if it is less than 480 it branches to another location, otherwise it tests a byte at offset 16 of the value in a0 and branches if it is zero or if the comparison failed, otherwise it adds 24 to the local variable and branches to another routine.
- `compare_and_return_zero`: Loads a longword from an offset of 4 bytes below a6 into a0, compares it to 480, and if they are not equal it branches to another location; otherwise it clears d0, unlinks a6, and returns.
- `forward_entry_arguments`: Passes two stored arguments and a table entry pointer to another routine before returning.
- `process_command_string`: Parses a command string starting with 'w', 'r', or 'a' and modifies a command flag byte at offset 4 of a structure based on the character and a '+' modifier.
- `process_command_character`: Reads a character from a structure and branches based on whether it is 'w', 'r', or 'a'. Depending on the character and a boolean flag, it calls another routine with various bitwise-combined flags and updates a pointer-based structure with the result and a status byte.
- `update_status_with_offset_eight`: Calls another routine using a pointer from offset 8 of a structure in a6, then modifies a structure at offset 10 of a pointer in a6 based on a byte at offset 17 of a6.
- `setup_stack_and_call_routine`: Pushes the value 2 and a null value onto the stack, calls a routine using a pointer from offset minus 8 of a structure in a6, and then branches based on a byte at offset minus 17 of that structure.
- `update_record_buffer_two`: Stores the value 2 at offset minus 4 of the stack frame and branches to another routine.
- `branch_on_status_byte`: Tests a byte at offset minus 17 of a structure in a6; if the byte is not zero, it loads the value 2 into d0 and branches to the next instruction, otherwise it continues to the next instruction.
- `update_status_with_offset_minus_twelve`: Calls another routine using a value from offset minus 12 of a structure in a6, then modifies a structure at offset 10 of a pointer in a6 based on a byte at offset 17 of a6.
- `test_and_set_d0`: Tests a byte at an offset from a6; if it is non-zero, it sets d0 to a constant and branches to another instruction, otherwise it returns.
- `update_record_buffer_one`: Stores the value 1 at offset minus 4 of the stack frame and branches to another routine.
- `test_and_set_constant`: Tests a byte at offset negative seventeen from the structure in a6; if the byte is zero, it branches to another routine, otherwise it loads the value two into d0 and branches to another routine.
- `update_status_with_bitwise_or`: Calls another routine using a bitwise OR of a value from offset minus 12 of a structure in a6 and 769, then modifies a structure at offset 10 of a pointer in a6 based on a byte at offset 17 of a6.
- `test_and_set_flag`: Tests a byte at offset negative seventeen from a pointer in a6; if the byte is zero, it branches to another routine, otherwise it loads the constant one hundred twenty-eight into d0 and branches to another routine.
- `set_stack_status_and_branch`: Stores the value two into a stack-relative location and branches to a nearby instruction.
- `clear_and_return_zero`: Clears register d0 and returns.
- `update_record_buffer`: Checks a byte at an offset from a pointer; if it is zero, it sets a longword at an offset of 12 in a structure pointed to by a value at offset 16 of another structure to zero, otherwise it sets bit 2 of a byte at offset -1 of a structure and sets a longword at offset 12 of a structure to the value at offset 16 plus 22.
- `clear_stream_record_fields`: Clears the record's count word, sets its buffer fields and state byte, then returns the record.
- `unlink_and_propagate_object`: Checks if bit 2 of a flag byte at offset 16 of a structure is set and calls close_handle_if_flag_clear if so; if bits 2 and 3 are clear and a pointer at offset 18 is non-zero, it calls call_with_second_argument_repeated using the values at offsets 12 and 18; finally, it clears the pointers at offsets 12 and 18 and the flag byte at offset 16, then calls write_byte_with_retry_2 using a byte from offset 17.
- `validate_index_and_get_table_pointer`: Checks if a global flag is set to call a routine, then verifies if the provided index is within a specific range; if the index is valid and the corresponding table entry has its high bit set, it returns a pointer to that entry, otherwise it stores nine in a global variable and returns a null pointer.
- `update_status_table`: Iterates through a table to find a null entry, then modifies a byte at that location using bitwise operations based on a parameter, and finally calls another routine with a value from a stack-based constant.
- `initialize_status_value`: Loads a longword from offset minus 4 of the structure in a6 and compares it to zero; if it is zero, it stores the value 24 into a global variable, sets that same global variable to 255, unlinks a6, and returns; otherwise it returns.
- `update_status_and_call_handler`: Modifies a bit in a 32-bit value at offset 12 of a structure, updates a pointer at offset minus 12 with a calculated address, sets a byte at that address based on various conditions, and calls another routine with parameters derived from the structure.
- `update_slot_status`: Modifies a status byte in a structure based on a mode value, potentially setting specific bits and calling another routine, then calls another routine with two arguments and returns a value from a structure.
- `test_bit_and_load_constant`: Tests bit 3 of a byte at an offset from a6, and if it is zero, loads the constant 8 into d0 and branches to another instruction.
- `set_status_bit_and_call`: Sets bit 5 of a byte at an offset from a pointer stored on the stack, then calls another routine with arguments from the stack and returns a value.
- `test_bit_and_load_constant_2`: Tests bit 3 of a byte at an offset from a pointer in a6; if the bit is set, it loads the constant 8 into d0 and branches to another routine, otherwise it branches to another routine.
- `set_status_bits_and_call`: Sets bits 5 and 6 of a byte at an offset from a pointer stored on the stack, then calls another routine with arguments from the stack and returns a value.
- `fail_slot_with_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code and returns minus one.
- `check_and_delete_file`: Tests bit 1 of a byte at offset 14 of the stack frame; if set, it calls another routine with arguments from the stack, then checks bit 8 of a value at offset 12 of the stack frame to either store 590 at offset minus 8 of the stack frame or branch.
- `open_file_and_check_error`: Stores 997 in a stack slot, calls a routine with a pointer and that value, then stores the result in a longword at offset 2 of a structure.
- `return_value_from_stack`: Returns the 32-bit value stored at an offset from the stack pointer.
- `clear_status_bit_and_recurse`: Clears a bit in the second argument and calls itself with the modified argument and the first argument unchanged.
- `read_file_with_error_handling`: If a parameter is non-negative, it calls another routine to obtain a handle and then calls another routine to read data into a buffer, returning an error code and setting a global variable if the read fails.
- `read_into_buffer_checked`: Calls a read library routine on the stored handle with the caller's buffer and length, and keeps the byte count.
- `write_to_handle_checked_2`: Validates a parameter, retrieves a pointer from a structure, and if the byte at that pointer has bit 3 set, calls a routine with zero as the second parameter before calling another routine to perform a write operation.
- `seek_file_via_offset_table`: Obtains a file handle and, for an index below 3, seeks to an offset retrieved from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_handle_if_flag_clear`: Obtains a structure from its argument and returns success without closing when its flag bit 2 is set.
- `delete_file_with_error_check`: Calls a delete-file library routine with the argument.
- `forward_single_argument`: Forwards its single argument to another routine and returns nothing.
- `rename_file_and_handle_error`: Calls a routine with two arguments and, if the result is zero, sets a global byte to 5 and returns -1; otherwise it returns 0.
- `clear_d0_and_unlock_a6`: Sets register d0 to zero, unlocks the stack pointer in register a6, and returns.
- `open_file_from_buffer`: Opens a file using a name stored in a global text buffer with a fixed mode and stores the resulting handle in a global variable at offset 40.
- `get_character_from_source`: If a global byte is non-zero, clears it and returns its previous value; otherwise it calls another routine to read a byte into a stack slot and returns that byte.
- `read_byte_from_file`: Reads a single byte from a file handle into a stack slot and returns that byte in the d0 register.
- `write_single_byte_file_2`: Checks two global bytes, calls a routine if either is non-zero, and then calls a routine to write a single byte from the argument to a file handle stored in a global variable.
- `conditional_store_byte`: Tests a byte at a specific address; if it is zero, it returns, otherwise it sets register d0 to 255 and returns.
- `store_byte_from_argument`: Copies the low byte of its stack argument into a global variable and returns.
- `close_active_handles_2`: Loops over a table of six-byte entries up to a count stored in a global variable, calling a routine with the first four bytes of an entry if the entry is non-zero and bit 2 is not set, then calls two additional function pointers and finally calls call_close.
- `copy_state_and_program_chips`: Copies a 20-byte global block into the first argument, updates a global byte, and stores one bit chosen by the second argument with the third argument in the copied block.
- `return_constant`: Returns the constant 6875 in d0 to the caller without any other effect.
- `copy_string_to_destination`: Copies a null-terminated string from the second argument into the first, byte by byte including the terminator, and returns the destination.
- `copy_bounded_string`: Copies up to a counted number of bytes from the source to the destination, stopping after a NUL byte.
- `reset_global_state_fields`: Clears a set of global state variables to zero and fills two global buffers through library calls.
- `toggle_around_paired_calls`: Calls a routine with 31, flips a global toggle between 0 and 1, calls the same routine with 15, and flips the toggle back.
- `update_timed_counter_state`: Compares a time reading against a stored base, and when the difference exceeds a threshold or a flag is set it decrements a global counter and calls a routine with the active table entry.
- `format_time_as_string_and_output`: Converts a global value into a minutes and seconds format or a single value if under sixty, stores the resulting five-character string in a local buffer, and then calls another routine five times to output each character at a position offset from two provided arguments.
- `call_offset_sequence_at_position`: Issues a fixed sequence of seven routine calls with the two arguments offset by small constants, one of which passes a block of eleven values.
- `store_slot_and_reset_state`: Calls routines with two global references, stores the argument into a global slot after reading its previous value, then calls routines with fixed values and a global reference.
- `process_panel_logic_2`: Updates several values in a structure, sets up global table references, and enters a loop that processes entries and updates linked list nodes based on returned values.
- `process_panel_logic`: Enters a loop that updates a local integer from a routine, and if the value is between 3 and 6, it performs several operations including updating a global table, calling a routine with multiple parameters from a structure, and setting a global variable based on a bit in a structure.
- `test_structure_bit_and_set_register`: Loads a pointer from a global variable into a0, reads a 16-bit word at offset 268 of that structure, and tests bit 7 of that word.
- `clear_flag_and_call_pair`: Clears a global flag, then calls a routine with a global reference and a stored value.
- `update_linked_list_nodes`: Traverses a linked list and, for each node meeting specific bitwise criteria, writes a value from a register into an offset of 18 or 28 of that node.
- `call_library_function_2`: Calls a routine at an address stored in a global variable.
- `show_end_of_game_result`: Outputs one of several result messages built from score globals, such as a winner, a tie or a no-more-moves ending.
- `center_entries_in_width`: Walks four records from a linked list in a global structure.
- `draw_indexed_entry_fields`: Uses the record selected by a global index to output a formatted two-digit value and several fixed fields, at positions taken from that record and the argument.
- `find_fitting_text_length`: Measures a null-terminated string and shortens its length one character at a time until a measuring routine reports a width at or below the limit.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `update_small_index_pair`: For each of two arguments below five, calls a routine twice on the global table with the argument and a flag value, using zero for the first argument and one for the second.
- `initialize_setup_state`: Clears several global state fields and sets others to fixed values, then calls routines to set up three global entries.
- `forward_argument_with_globals`: Forwards its argument with three global buffer references to a routine, then calls another routine with a fixed value and zero.
- `check_and_call_library`: Clears a word at a negative offset from a stack pointer, compares it to nine, and if it is less than nine, calls a library function via a pointer.
- `unlock_stack_pointer_and_return`: Unlocks the stack pointer and returns immediately.
- `draw_indexed_element_group`: Returns a computed counter value when a global flag is set; otherwise, it reads a counter and, when two global flags are set and a second value is non-zero, draws a fixed-position group of elements chosen by a switch on the argument from zero to five before returning the value.
- `clear_saved_selection_state`: If a global flag is set, it calls output routines based on the value of a state variable, then clears several state globals to zero.
- `draw_indexed_panel_item`: Positions a numbered item from a table of coordinate values, using a second layout when a global flag is set and the index is from 84 to 87.
- `enable_panel_pair`: Calls a routine for two panel indices with fixed arguments, sets a global flag to one, then calls a routine with a global value.
- `disable_panel_pair`: Calls a routine for two panel indices with fixed arguments, clears a global flag, then calls a routine with a global value.
- `enable_alternate_panel_pair`: Calls a routine for two other panel indices with fixed arguments, and sets a global flag to one.
- `disable_alternate_panel_pair`: Calls a routine for two other panel indices with fixed arguments, and clears a global flag to zero.
- `call_fixed_argument_pair`: Calls a routine with a fixed value and zero.
- `compare_routine_results`: Calls seven different routines, compares the results of the first and eighth calls, and if they are equal, calls another routine with a global variable and returns 1; otherwise, it calls another routine with the value 4 and returns 0.
- `call_with_four_return_zero`: Calls a routine with the value four and returns zero.
- `set_flag_and_run_steps`: Sets a global flag to one and then calls three routines in sequence, passing a global buffer reference to them.
- `read_input_and_clear_flag`: Reads into a 256-byte local buffer through a routine.
- `advance_state_and_store_pair`: When a global mode equals four it increments a per-slot counter and sets two flags.
- `clear_and_call_bounds`: Clears a 16-bit word in a global variable and calls another routine.
- `draw_bounds_for_index_range`: Calls a routine with four computed bounds, then for indices from 84 to 87 it calls routines with values from a table and computed rectangle positions.
- `show_status_and_wait`: Outputs a status message chosen by a state global and a flag, and sets the state to 10, 6 or 5 according to its value.
- `process_table_chain_2`: Traverses a chain of pointers from a structure, performing signed magnitude division, alternating table output, and flag activation, then increments a global counter and calls another routine when the counter reaches 144.
- `process_table_chain`: Traverses a chain of pointers from a structure, performing signed magnitude division, alternating table output, and flag activation, then increments a global counter and calls a routine if the counter reaches 144.
- `output_alternating_table_values`: Outputs values from table entries at the index in alternating order, then adds two to a counter and calls a routine with fifteen when the counter reaches 144.
- `activate_flag_and_set_limit`: Sets a global flag to one and a global value to 88, then calls a routine.
- `advance_index_chain_from_frame_2`: Sets a table entry from the value it points to, marks the entry it reaches as one, increments the counter for the index taken from the calle and calls itself recursively.
- `advance_index_chain_from_frame`: Sets a table entry from the value it points to, marks the entry it reaches as one, increments the counter for the index taken from the caller's frame, and calls a routine with that index.
- `check_table_entry`: Calculates an address by shifting a word from a structure in a6 and adding a constant, then checks if the word at that address is non-zero.
- `check_value_validity_2_2`: Compares a 16-bit value at offset 10 of a structure in a6 to the range between 84 and 87, and checks if a global word is non-zero; otherwise it returns.
- `check_value_validity_2`: Checks if a 16-bit value at offset 10 of a structure in a6 is less than 59 and performs several checks against values stored in a global table.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `check_value_validity`: Checks if a 16-bit value at offset 10 of a structure in a6 is less than 59 and performs several checks against values stored in a global table via pointers.
- `check_structure_value`: Checks if a 16-bit value at offset 10 of the structure in a6 is less than 59, then checks if a 16-bit value at a location pointed to by a global table is non-zero, returning 1 or 0 accordingly.
- `return_boolean_one`: Sets the return register to one, unlocks the stack pointer, and returns.
- `return_constant_one`: Sets the return register to one and unlocks the address register a6 before returning.
- `draw_remaining_tiles_panel`: Draws a rectangle using fixed coordinate values, formats a string containing a three-digit value from a global variable, draws that string, and then draws another fixed text line.
- `dispatch_layout_by_index`: Returns minus one if the input argument is less than one.
- `emit_flagged_argument_list`: Optionally calls a routine with an identifier computed from the second argument.
- `test_state_one_to_four`: Returns one if the value of a global variable is between one and four inclusive, and zero otherwise.
- `update_indices_and_call_arguments_2`: Checks a global byte for the values 5 or 6, updates it to 1 or 2 respectively, calls a routine with two arguments, and then calls another routine with either eleven specific integer arguments or a different sequence of eleven integer arguments based on a second global byte.
- `call_argument_list_with_flags`: Calls a routine passing a specific sequence of eleven integer arguments after optionally calling a routine with an identifier computed from the second argument.
- `update_indices_and_call_arguments`: Sets a global byte to 2, calls a routine with two arguments, and then calls another routine with either eleven specific integer arguments or a different set of eleven integer arguments depending on the value of a global byte.
- `call_argument_list`: Calls another routine passing a specific sequence of eleven integer arguments.
- `pop_linked_list_element`: Calls a routine with a given index, clears the link at that index within a global table, updates the index to point to the next linked value, decrements the value at that index, and calls the same routine again with the updated index.
- `draw_grid_cell_and_neighbours`: Computes the screen position of a cell from its index in a 15-column grid, choosing offsets for edge and centre cells.
- `run_input_until_result`: Stores an argument and a routine's value in a record, then repeatedly calls a routine until it returns one or two.
- `process_string_and_wait`: Calls a routine with an argument, then scans a string to find unquoted segments, copies segments into a structure, and repeatedly calls a routine until it returns 1 or 2.
- `wait_for_choice_result`: Calls a routine repeatedly until its return value is 1 or 2, storing the result into a memory location at offset -8 from a pointer in a6, then returns that result.
- `wait_until_result_is_one`: Initializes a record with an argument and a fixed byte value, calls a routine for a value, then calls a routine repeatedly until it returns one and returns that value.
- `drain_entry_list_to_count`: Repeatedly takes an entry from a list.
- `forward_with_global_table_pair`: Forwards its argument to a routine together with two global table references and a global reference.
- `build_display_item_records`: Walks a linked list of entries for one slot and fills consecutive 44-byte display records from them.
- `fill_display_record_from_item`: Writes a 23-byte record from an item's position and size fields, sets its type byte to 23, and clears its other fields.
- `search_slot_table_for_entry`: Searches up to five global table entries for one whose key matches the record's entry, and calls a routine with the matched value.
- `count_string_bytes_2`: Scans forward from a pointer byte by byte until it reaches a zero byte, counting the bytes examined, and returns that count as the length of the string.
- `set_layout_limit_by_state`: Sets two global limit values to 63 when the state global is three, and to 47 otherwise.
- `compare_strings_counted`: Compares up to a counted number of bytes of two strings.
- `scan_unquoted_characters`: Scans a string, skipping quoted sections and backslash escapes, and passes each unquoted character to a routine until that routine returns non-zero.
- `handle_game_event`: Performs no observable work on data or registers and returns immediately.
- `compare_strings_full`: Compares two null-terminated strings and returns the byte difference at the first mismatch, or zero when they are equal.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `initialize_slot_tables`: Clears seven state globals, sets a count global to 144, calls a routine with a fixed value, and fills a global table with specific index and value patterns across several loops.
- `return_absolute_value`: Returns the absolute value of a 16-bit argument.
- `return_sum_of_two_calls`: Calls two routines that each return a value and returns their sum.
- `shuffle_index_table`: Fills a 144-entry array with the indices one to 144, then shuffles it by swapping each entry with an earlier entry chosen by a routine.
- `reset_count_and_call_pair`: Sets a count global to 144, calls a routine, then calls a routine with two global references.
- `output_table_value_and_char`: Calls a routine twice with the table value at the first index, then calls a routine with the byte at the second index as a signed value.
- `call_fixed_sequence_for_pair`: Issues a fixed sequence of five routine calls, using fixed indices and the global pair positions offset by fixed amounts.
- `draw_grid_and_report_state`: Clears a global counter and calls another routine.
- `draw_positive_entry_frame`: Calls a routine with the entry value at the index.
- `process_selection_bounds`: Calculates two values based on a pointer argument, clamps them between 0 and 12 or 0 and 6, then enters a loop that iterates up to 9 times to perform bounding box intersection checks and update a global index and associated data from a table.
- `update_selection_data`: Checks a global flag, performs a bounding box intersection check, and then updates a global variable with a value from a table based on an index stored in another global.
- `copy_selected_entry_value`: Copies a value from a global table at the current selection index into a second global.
- `update_selection_state_machine_2`: Evaluates a state byte and compares it against several global variables to manage selection transitions.
- `update_selection_state_machine`: Evaluates a state byte and compares it against several global variables to manage selection transitions.
- `manage_selection_state`: Evaluates a state byte and several global variables to update a state byte, copies values between global variables, and calls several other routines based on the comparison results.
- `update_selection_logic`: Manages a state machine by comparing various global variables and updating a state byte, several other globals, and calling various other routines based on the current state and comparisons.
- `update_selection_state`: Updates a global state byte and several other global variables based on comparisons of existing global values, then calls another routine with various arguments.
- `update_state_and_call_routines`: Increments a byte in a global variable, copies two words from a table into other global variables, calls another routine with a value from a global variable, and calls another routine with two values from the table.
- `reset_pick_state`: Calls three different routines with specific arguments, sets a global variable to 1, and returns.
- `handle_matching_selections`: Checks if a global variable is 2, and if so, compares two other global variables to determine if it should update several global values and call four different routines with specific arguments.
- `revert_unmatched_second_pick`: If a global comparison matches, it decrements a pick state, clears two globals, and calls several output routines including one with a global reference and a fixed value.
- `process_selection_event`: Checks several global conditions and a word at an offset determined by a global variable, then either updates state variables and calls other routines or resets state variables and calls other routines.
- `validate_word_range`: Tests a word in a global variable and returns if it is zero; otherwise, it compares a word from a pointer in a register against the range between 83 and 84, returning if the value is outside this range.
- `compare_table_values`: Decrements a word from a global variable and loops if the result is not zero, otherwise it compares a word from another global variable with a word from a third global variable and unlinks a6 if they are not equal.
- `toggle_selection_mode`: Checks a global variable and, if it is 0, sets it to 1, updates two global variables, and calls four routines; otherwise, if it is 1, sets it to 0, clears two global variables, and calls four routines.
- `cancel_pending_pick`: If a global variable equals one, it resets that variable and two other globals to zero, then calls several output routines.
- `compare_and_reset_selection`: Compares a 16-bit word from a global variable with another 16-bit word from a global variable, then calls a routine and branches to another routine if they are not equal.
- `reset_selection_state`: Checks if a global value is between 84 and 87 and non-zero, then decrements a global counter.
- `reset_pick_state_and_output`: Calls two routines with fixed values, then clears the pick state and four pick globals.
- `validate_index_status`: Checks if a parameter is 2 and evaluates a logical condition involving three global variables, or checks if a parameter is not 2 and evaluates conditions involving a table lookup and a global flag, calling another routine and returning either 0 or 1.
- `validate_index_range`: Checks if a value at a specific offset in a structure is non-zero and matches a pattern, or checks if a global flag is set and a value in a structure is within a specific range.
- `check_index_range`: Checks if a 16-bit value at offset 10 of a structure in a6 is between 54 and 57, and if a global word is non-zero; if so, it calls a routine with the value 5 and returns 0, otherwise it returns 1.
- `return_one`: Sets register d0 to one and returns.
- `process_event_dispatch_loop`: Executes a nested loop that evaluates a global variable and a pointer to determine whether to call various subroutines based on specific return values and state flags.
- `dispatch_long_index`: Checks if a long value at a global variable is non-zero, and if it is, calls another routine with the value 5 passed on the stack before unlinking a6 and returning.
- `dispatch_word_index`: Loads a 16-bit word from a global variable, and if it is non-zero, calls another routine with the value 3 passed on the stack.
- `dispatch_record_index`: Loads a pointer from a global variable, checks if the byte at that pointer is non-zero, and if it is zero, calls another routine with the value 6 passed on the stack.
- `search_record_table_and_dispatch`: Checks a value at a pointer, and if it is zero and a value at an offset of minus two from a pointer in a6 is one, calls another routine.
- `restore_selection_and_refresh_state`: Sets a global mode to three and, if a value at an offset of minus two from a pointer in a6 is one, calls a routine with a pointer, then potentially calls another routine with several arguments.
- `restore_selection_and_refresh_2`: Calls a routine, then if the return value is not zero, loads a 16-bit word from an offset of minus 4 from a pointer in a6 into a global variable and returns.
- `restore_selection_and_refresh`: Loads a 16-bit value from an offset of -4 from a pointer in a6 into a global variable, then calls two routines using global pointers as arguments and calls a third routine before returning.
- `load_tour_file_state`: Calls a routine with a global variable and a constant, then calls another routine with multiple arguments including a global variable, and finally calls routines to handle the result based on whether a value was returned.
- `draw_panel_wait_for_choice`: Draws a panel from fixed entries and a record, then waits in a loop until a routine returns one or two.
- `process_tournament_input`: Calls another routine to get an input, then enters a loop that calls another routine with data from a table based on the input, and finally updates a global variable with a value determined by a switch statement.
- `process_linked_list_nodes_2`: Writes a value to an offset of minus 22 from a6, stores a pointer to a global variable at an offset of minus 26 from a6, and calls another routine with the value nine.
- `iterate_record_table`: Enters a loop that continues while a value at an offset of minus 14 from a pointer is not 1 or 2, performing table lookups and calling another routine within a nested loop, then performs a switch-based update to a global variable and calls several more routines.
- `search_record_table_2`: Copies a string, searches a table for a matching record, and if a match is found, calls another routine and returns one; otherwise it returns zero.
- `return_zero_and_restore_registers_2`: Clears register d0 and restores the previous register state from the stack before returning.
- `return_integer_one`: Loads the value one into a register and returns it to the caller.
- `draw_time_left_label`: Outputs a label at a position offset by six and twelve from two global coordinates, then calls output routines at the same coordinates twice.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `build_score_entry_records`: When a count global is non-zero, builds that many text records from table values, storing each text pointer, a position and a computed centre offset.
- `clear_score_entry_count`: Calls a routine with two global references, then clears the score entry count.
- `reset_score_display_state`: Calls routines to reset the display and output fixed values, stores the current count in a second global, sets a flag to one, and stores the result of a routine in another global.
- `tick_countdown_timer`: When the elapsed time since the last tick exceeds 256, decrements a remaining-time counter.
- `show_tournament_result_text`: Builds a results message from the placement, the tie wording and the scores, outputs it, and waits in a loop until a routine returns one.
- `insert_score_into_table`: Inserts a score into a table of up to five sorted entries and returns six when the table is full and the score is below the last entry.
- `shift_global_variable_left_three`: Loads a 32-bit global variable, shifts its value left by three bits, and returns the result.
- `call_routine_with_duplicate_argument`: Loads a 32-bit value from an offset of 8 within a structure pointed to by a6, pushes this value onto the stack twice as two separate parameters, calls another routine, and then returns.
- `traverse_linked_list`: Walks a linked list from a global head, updating a global counter and adjusting pointers based on a requested distance.
- `call_with_second_argument_repeated`: Pushes a value from a pointer into the stack twice and calls another routine with the first value followed by the second value repeated as the third argument.
- `insert_or_update_linked_list_2`: Calls a division routine to determine an index, then traverses a linked list to either insert a new node at a specific position or merge a new value into an existing node.
- `format_numeric_string_buffer`: Converts a numeric value into a decimal string representation by writing ASCII characters to a buffer, utilizing convert_integer_to_ascii_string and format_numeric_string to populate the target memory area.
- `convert_integer_to_ascii_string`: Converts a numeric value into a decimal string representation by writing ASCII characters to a buffer.
- `increment_and_branch`: Calls another routine, increments a 16-bit value at offset 6 of a structure in a1, and then branches to another location.
- `multiply_and_decrement`: Adds 4 to an offset of 4 in a1, calls two routines, decrements a 16-bit value at offset 6 in a1, and if d0 is not negative, decrements the value at offset 4 in a1, shifts d1 left, and rotates the carry into d0 before looping.
- `conditional_multiply_loop`: Clears d6, pushes d0 and d1 onto the stack, and if d0 is not zero, calls another routine and loops if d6 is not zero or if d2 is not 1, while also decrementing a 16-bit value at offset 6 in a1.
- `format_numeric_string`: Converts a numeric value into a decimal string representation by writing ASCII characters to a buffer.
- `decrement_loop_until_zero_2`: Decrements a 16-bit counter in a loop until it reaches zero and then returns.
- `multiply_64bit_by_ten_2`: Multiplies a 64-bit value composed of two 32-bit registers by ten, returning the high 32 bits and low 32 bits in the same registers.
- `convert_binary_to_bcd_digits`: Iterates sixty-four times to shift a 32-bit value left one bit at a time, accumulating the bits into a decimal-coded format by subtracting ten and incrementing a high-word carry whenever a digit exceeds nine.
- `dispatch_signal_codes`: Checks a value from a library call and, if a global byte is non-zero, calls another routine with a character code, a line-break code, and then calls another routine in a loop using an index up to a global count.
- `call_fixed_setup_pair`: Calls a routine with two global references and the value twelve, then calls it with a global reference and the value 2560.
- `draw_indexed_segment_outline`: Clears a pending state when one is set, resets the fields of a record, and draws a set of outlined segments around the first two arguments.
- `clamp_coordinates_then_call`: Clamps the first and second arguments to be non-negative and within the width and height stored in a structure, then calls a routine with the clamped values.
- `clamp_coordinates_then_call_2`: Clamps the second and third arguments to be non-negative and within the bounds defined by a width and height stored in a structure pointed to by the first argument, then calls another routine with the clamped values.
- `format_string_with_directives`: Copies a format string into an output buffer, turning each doubled percent sign into one percent sign and expanding other percent directives through a called routine that fills a temporary buffer.
- `self_calling_stub`: It calls another routine and then returns, with no other processing performed in the body.
- `push_and_call_routine_3`: Pushes a 16-bit value derived from the bitwise AND of d2 and 32767 with d3 onto the stack, calls another routine, and if the result is zero, clears d0 and d1 before branching.
- `check_and_repeat_call`: It checks whether a flag in one register is non-zero or the magnitude of a value in another register is non-zero, and if so it calls another routine five times in a row.
- `shift_subtract_loop`: It runs a 32-iteration loop that repeatedly shifts three unsigned values left by one bit, inserting borrow and sign bits, and conditionally decrements a counter based on comparisons of the values.
- `clear_and_branch_2`: Saves registers d2 through d7 to the stack, clears register d7, and branches to another routine.
- `execute_conditional_calls`: Calls another routine twice and, if a borrow occurs during a subtraction, enters a loop that decrements a value until it reaches negative one, then calls another routine and returns.
- `execute_nested_calls`: Calls another routine twice, then if a borrow occurs during a subtraction, it enters a loop that decrements a value until it reaches -1, then calls another routine and returns.
- `decrement_and_loop`: Decrements a 16-bit register, performs a logical shift left on d0, rotates the carry into d1, and loops using a dbra instruction based on the decremented register before moving d6 to d3 and branching to another routine.
- `compare_and_xor`: If d3 equals 0, it compares d3 to 64 and branches if it is less than 64; otherwise, it performs an eXclusive OR on d2 using d7 and branches to another routine.
- `invoke_and_return_2`: Decrements a value in a register through a loop and then calls another routine before returning.
- `invoke_and_return`: It invokes another routine and then returns without performing any additional operations.
- `clear_d0_based_on_d2`: Pushes d2 onto the stack, copies d0 to d2, clears bit 31 of d2, and if d2 is not zero, sets d0 to zero and branches to another routine; otherwise it returns.
- `modify_register_value`: Calls another routine, increments d2, and then performs conditional branches based on the result of subtracting 32 from d2, eventually clearing bit 31 of d0 and branching.
- `shift_and_sign_extend`: Takes a value from a register and a signed shift count from another register, shifts the value right by the low six bits of the negated shift count, and negates the result if the shift count is negative.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `find_char_in_string`: Scans a zero-terminated string for the given character and returns a pointer to its first match, or null if it is absent.
- `forward_two_arguments_unused`: Forwards its two arguments to another routine and then returns.
- `compute_binary_weighted_sum_2`: Takes two integer inputs and returns an integer result.
- `process_menu_selection_loop_2`: Performs several initialization calls, updates a global variable, and enters a loop that repeatedly calls another routine to check for a value between two and eight; if so, it updates a global variable and iterates through a loop to call another routine with data from a structure before calling another routine again.
- `process_menu_selection_loop`: Enters a loop that processes menu selections, updates a global short, and iterates through a sequence to call a routine with parameters derived from a global pointer and a calculated index.
- `forward_three_offset_calls`: Calls another routine three times with fixed leading arguments, passing the input plus zero, four and eight as the last argument each time.
- `position_entry_and_forward_calls`: Makes one call with constant arguments, then calls another routine with position values built from the two inputs plus fixed offsets, and finishes with a call whose value is read from a table indexed by the third input.
- `dispatch_menu_selection`: Stores four 16-bit values into a record, calls a drawing routine, and then executes one of several branches based on the value of a parameter.
- `restore_registers_and_unlink`: Restores d2 and d3 from the stack, unlinks a6, and returns.
- `forward_two_values_to_call`: Forwards its two word-sized arguments, together with a fixed base value, to another routine and then returns.
- `update_table_and_process_linked_list`: Iterates through a linked list of 16-bit values, updating a running sum and performing calculations based on values retrieved from a table.
- `process_linked_list_and_counters`: Iterates through a linked list of short integer pairs, updating a counter in a structure and comparing values.
- `process_linked_list_and_table_data`: Iterates through a linked list of 16-bit values stored in a structure, performing recursive calls to another routine for each element.
- `increment_and_call_handler`: Increments a 16-bit word at an offset from a pointer, calls a routine with a value from a global variable, and then calls another routine before clearing a word at an offset from a pointer.
- `finish_table_update_cleanup`: Makes two library calls using a shared global argument, repeats the second call if a value at an offset of negative 16 from a pointer in a6 is non-zero, then calls another routine and returns.
- `call_with_fixed_parameter_block`: Calls a drawing routine with a fixed list of constant numeric parameters, taking one of those parameters from a structure pointed to by a global value.
- `poll_until_ready_then_call`: Calls one routine with a constant, then calls another with a global argument, polls a routine repeatedly until it returns one, and makes a final call with the same global argument.
- `clear_registers_on_nonzero`: Copies d0 to d2, clears the highest bit of d0, and if d0 is not zero, branches to another routine; otherwise, it clears d0, d1, and d2 and branches to another routine.
- `extract_sign_bits_and_set_high_bit`: Iterates ten times, shifting a value left and accumulating the sign bit of each intermediate result into a new value, then returns that value with the most significant bit set.
- `process_bit_stream`: Performs bitwise shifts on d0 and d1 within a loop, modifies a value in d2 based on the results, and calls another routine twice with different arguments depending on the final state of the registers.
- `perform_bitwise_shift_loop`: Performs a loop that shifts bits from d1 and d0 into new values until a specific bit in d0 is cleared.
- `push_and_call_routine`: Pushes the constant 1 onto the stack, calls another routine, and clears d0.
- `process_and_branch`: Pushes the value 2 onto the stack, calls another routine, performs bitwise operations on d0 and d2 involving a swap and an OR, clears d0, adjusts the stack pointer, and branches to another routine.
- `store_param_to_global_2`: Copies its single 32-bit argument into a global variable and returns, performing no other operation.

### ced

- `run_hdinstall_entry`: Program entry for the HDinstall tool.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### AddBuffers

- `add_disk_buffers_command`: It is the entry point for a command that adds disk buffers to a drive.

### CLI

- `parse_command_line_and_start`: Opens the DOS library and allocates a control block to parse the command line into individual argument strings.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `open_cli_window_and_run_startup`: Creates a console window and shell control block based on a specification, then executes the system startup scripts.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `traverse_and_free_linked_list`: Recursively traverses a linked list by calling another routine on the second element of each node before freeing the node itself.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `call_and_negate_on_flag`: Calls another routine and returns its first result, or the negation of that result if the second result left in an address register is nonzero.

### Add44K

- `call_slot_record_clearer`: Calls a routine via a jump to a longword address.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries by name through the library-open routine, keeping both bases.
- `clear_slot_record_store_stack`: Clears forty-two longwords of a per-slot record, stores the stack position and a global value into it, then calls another routine.
- `open_dos_and_mathffp_libraries`: Opens the dos and mathffp libraries and stores their bases in global variables; if the first open fails, it calls another routine, otherwise it checks a structure and proceeds to call various library functions to initialize data, including a loop that parses a string into a table of pointers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `close_stored_libraries_and_resources`: Runs a ten-step wait, closes each stored library base that is nonzero and releases a stored resource through its routine.
- `set_d0_and_branch`: Sets register d0 to 32767 and performs a short branch to another location.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `process_table_entry`: Checks if an index is within a specific range and if a pointer in a table is non-zero, then either calls another routine using that pointer or sets a global variable to 3 and returns -1; otherwise it clears the pointer in the table and returns 0.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a function located at an offset from a library base stored in a global variable.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_library_function_2`: Performs an indirect jump to a function located at a different offset from a library base stored in a global variable.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### csh

- `initialize_aga_patch`: Executes a sequence of function calls, performs a loop of four calls including one via a pointer in a global variable, and checks a value at an offset from a pointer to update a global variable and a memory location.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Degrader

- `initialize_system_environment`: Clears a block of memory, calculates a stack offset based on a structure's size, and performs several calls to initialize global pointers and system state.
- `process_linked_lists`: Performs several operations involving a structure in a4, including calling a function pointer from a global variable if a condition is met, calling a function pointer from a structure at offset 5644, and calling various other function pointers located at offsets within a structure or a global variable.
- `compare_and_execute_table_functions`: Compares a value in a register against a global variable and, if the value is less than or equal to the global variable, calls three functions from a table via a pointer in a6 before returning.
- `manage_task_resource_lifecycle`: Allocates memory for a structure, populates it with several values from registers and a string from a pointer, and updates a global pointer with the new address.
- `parse_hex_string`: Parses a string to convert hexadecimal or decimal numeric characters into an integer, handling a leading minus sign and a dollar sign prefix.
- `copy_string_and_allocate_structure`: Copies a string from one memory location to another until a null terminator is reached, otherwise it performs a series of memory writes to a structure calculated from a global offset, calls a routine via a pointer in a global variable, and copies a byte sequence into the structure before calling another routine via a pointer in a register.
- `modify_status_and_manage_buffer`: Sets or clears the most significant bit of a byte at offset 13 of a structure in a parameter, then either calls two function pointers from a table or performs a series of stack-based operations involving a memory copy loop and calls to other routines.
- `mask_byte_and_call_pointers`: Clears the most significant bit of a byte at offset 13 of a structure in a3, then calls two function pointers located at negative offsets from a pointer stored at offset 436 of a4.
- `process_system_messages_and_update_status`: If a condition is met, it copies a sequence of bytes from a memory location into a new structure, updates a pointer in a structure, and calls another routine.
- `validate_and_dispatch_callbacks`: Performs a checksum validation on a data block and, if successful, iterates through a list of pointers to execute a series of callback functions, updating a status byte with bitwise flags based on the return values of those calls.
- `remove_from_linked_list_2`: Iterates through a linked list starting at offset 322 of a global variable, and if a matching pointer is found, calls two function pointers, sets a byte at offset 9 of the matching element to 128, and calls another function pointer.
- `jump_to_graphics_config`: Performs a jump to a long address.
- `manage_library_reference_2`: Calls a function via a pointer at a negative offset from a global variable, and if the result is non-zero, calls a function via a pointer at offset -888 from the result and another function via a pointer at a negative offset from a global variable.
- `update_minimum_value_in_linked_list`: Disables interrupts, compares a parameter against a value at offset 62 of a global structure, and traverses a linked list to update minimum values within the list nodes.
- `compare_byte_strings`: Compares two strings byte by byte in a loop, returning 1 if the strings differ or if the first string contains a zero before the second string reaches a value of 33, otherwise it returns 0.
- `jump_to_subroutine`: Performs a long jump to another routine.
- `dispatch_resource_callback_2`: Fetches a function pointer from a global structure, calls another routine, and then calls the retrieved function if it is not null.
- `set_ciaa_bit_2`: Sets bit 1 of a global variable by performing a bitwise OR with two.
- `update_dynamic_callback_state_2`: Reads a value from a global structure, applies bitwise modifications based on flags, and executes a loop that calls a function via a pointer.
- `execute_nested_callback_chain_2`: Fetches a function pointer from a table via a global variable and calls it if it is non-null; if successful, it calls a function at an offset.
- `dispatch_callback_via_pointer`: Retrieves a function pointer from a table at an offset from a global variable, and if the pointer is non-zero, calls another routine and then executes the retrieved function.
- `execute_recursive_callback_chain_2`: Fetches a function pointer from a global variable via an offset, calls it, and if the returned address is non-zero, calls another function.
- `check_vertical_blank_status_2`: Returns the result of masking bit 13 of a global variable.
- `update_vbi_register_2`: If bit 13 of a global variable is set, it overwrites that global variable with the 16-bit value provided in a parameter.
- `jump_to_callback`: Performs a long jump to a specific address to execute a callback routine.
- `clear_ciaa_bit_2`: Clears bit 1 of a global variable by performing a bitwise AND with 253.
- `call_external_function_conditionally`: Decrements the stack pointer by four, sets a register to zero, and if that register is not zero, loads a function address from a global pointer and calls it before restoring the stack pointer and returning.
- `call_external_function_via_stack_pointer`: Decrements the stack pointer by four, moves zero into the address register, and pushes it onto the stack; if the value is not zero, it loads a function pointer from a location relative to the stack pointer, calls that function, and then increments the stack pointer before returning.
- `calculate_string_length`: Iterates through a null-terminated string and returns the difference between the end pointer and the start pointer.
- `traverse_and_free_linked_lists`: Iterates through a linked list starting at an offset of 5644 in a structure to call a function pointer for each element, then processes a second linked list starting at offset 5484, updating a pointer at offset 4308 and potentially calling another function if a condition is met.
- `allocate_sub_block`: Checks bit 3 of a byte at offset 27 of a structure in a0; if not set, it calls another routine, stores the result in offsets 4 and 16 of the structure, and if the result is not zero, copies a value from a global variable to offset 20 of the structure, clears bit 4 of the value at offset 24, and sets offsets 12 and 16 to zero; otherwise, it sets a global variable at offset 4988 to 12 and returns -1.
- `process_error_and_status`: Checks a return value from another routine to determine a status code, updates a value at offset 4472 of a structure, and calls a function from a table via a pointer at offset 5000 or 5120 depending on specific bit flags and conditions.
- `test_bit_and_load_registers`: Tests bit 0 of a long word in d6, then loads d0 from a stack offset of 4, a0 from a stack offset of 8, and d1 from a stack offset of 12.
- `process_string_data`: Checks a value from another routine and updates a structure at offset 5484 or 4344 based on bit flags, then calls a function from a table and potentially processes a string by copying characters from a buffer while skipping carriage returns.
- `update_structure_status`: Updates a structure at offset 5484 or 5144 based on the result of another routine, and calls a function from a table if a parameter equals 2.
- `process_linked_list_nodes`: Sets a byte at offset 3906 of a structure to 1 and iterates through a linked list starting at offset 4076 of a structure, calling either a routine at offset 5000 minus 36 or a routine at offset 2000 based on the values in the list nodes.
- `parse_command_string`: Parses a string by skipping whitespace and splitting it into tokens, handling quoted substrings and expanding escape sequences like asterisk-E or asterisk-N. It populates a table of pointers with these tokens and subsequently performs string operations and conditional calls based on the parsed content.
- `move_and_reload_d0`: Copies the contents of d0 to d5 and then loads a new value into d0 from a stack offset of 4.
- `process_string_and_callback`: Copies a string from a pointer into a local buffer, performs checks against a global structure and its members, and eventually stores a function pointer into a table.
- `validate_and_traverse_list`: Sets a value at offset 4312 of a structure to 9 if a signed integer in d0 is negative or exceeds a value at offset 4072, otherwise it decrements d0 while traversing a linked list from an offset of 4076, setting the value at offset 4312 to 5 if the list ends before d0 reaches zero.
- `process_string_and_callback_2`: Copies a string from a pointer into a local buffer, performs checks against a global structure and its members, and eventually stores a function pointer.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: Finds the end of a string pointed to by a0, then copies bytes from a source pointed to by a1 to the destination starting at the end of the first string, decrementing a counter in d0 and filling remaining bytes with zero until the counter or the source string ends.
- `copy_bytes_with_termination`: Copies bytes from a source pointed to by a1 to a destination pointed to by a0 until a zero byte is encountered, then uses a counter in d0 to fill the remaining destination bytes with zero bytes.
- `allocate_sub_block_from_pool`: Attempts to find and carve a block of memory from an existing linked list of memory pools based on a requested size; if no suitable block is found, it calls a memory allocation routine to create a new pool node and inserts it into the list.
- `iterate_linked_lists_and_call`: Iterates through a linked list to call a routine for each node, processes a second linked list where it conditionally calls a routine based on bit flags, and finally calls another routine.
- `process_file_node_input`: Checks flags in a file node structure to validate state and then writes a byte from a register into a buffer pointed to by offset 4 of the structure, incrementing the current offset at offset 4 and the counter at offset 12.
- `update_file_node_state_2`: Decrements a counter at offset 8 of a structure and updates status flags at offset 27 based on the result and other bitwise checks.
- `update_file_node_state`: The routine checks flags in a file node structure, iterates through a linked list to verify conditions, updates internal counters and status bytes based on helper return values, and returns a status code.
- `release_resource_and_cleanup`: Sets a byte at offset 8 and a longword at offset 14 of a structure to all-ones, then calls routines to free memory, free a signal, and remove a port.
- `mark_block_unused_and_free`: Marks a structure as unused by setting one byte at offset 8 and three longword fields starting at offset 14 to all-ones, then calls a routine to free memory.
- `initialize_task_structure`: Calls a routine to find a task, and if successful, populates a structure with provided parameters and offsets, then either initializes linked list pointers or calls another routine.
- `update_structure_fields`: Calls a routine to allocate memory and, if successful, writes a byte, a byte, a longword, and a word into the allocated structure at specific offsets.
- `return_status_pointer`: Returns a pointer or value from a structure to the caller, specifically used to retrieve status information or string data during error and string processing.
- `reset_input_state`: The routine calls another routine and then returns.
- `remove_from_linked_list`: Iterates through a linked list stored in a global structure, and if a matching pointer is found, removes it from the list, sets a byte at offset 9 to 128, and calls another routine.
- `update_minimum_value`: Disables interrupts, updates a minimum value in a global structure if the parameter is smaller than the current value, traverses a linked list to find a suitable block, updates the list and a counter, and then enables interrupts.
- `dispatch_resource_callback`: Retrieves a function pointer from a global structure, calls another routine, and then calls the retrieved function if it is not null.
- `execute_callback`: Retrieves a function pointer from a library, checks if it is not zero, calls another routine, and then calls the retrieved function.
- `execute_nested_callback_chain`: Fetches a function pointer from a table via a global variable and calls it if it is non-null; if successful, it calls a function at an offset from the global variable, calls another routine, and then calls the original function pointer again.
- `execute_recursive_callback_chain`: Fetches a function pointer from a global variable via an offset, calls it, and if the returned address is non-zero, calls another function via an offset from the same global variable, calls itself recursively, and finally calls the previously fetched function pointer again.
- `update_global_function_pointers`: Calls a routine using a pointer from a structure in a6, then calls another routine twice using a pointer from a structure in a6 and stores the results in two global variables.
- `modify_status_bits`: Sets bit 1 and clears bit 2 of a register, then pulls a longword from a memory location onto the stack before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `test_and_set_bit`: Tests bit 2 of d1; if set, it branches to another location, otherwise it sets bit 1 of d1 and returns using a value from a memory location.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `performs_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `update_dynamic_callback_state`: Reads a value from a global structure, applies bitwise modifications based on flags, and executes a loop that calls a function via a pointer until it returns zero.
- `update_resource_status`: Calls a routine using a pointer from a global structure, and if the result is not zero, calls another routine, then updates a global variable with the low byte of a word and sets bit 7 and bit 0 of that global variable.
- `test_dma_control_register`: Tests the word at the DMA control register address.
- `clear_and_call`: Clears d0 multiple times, sets bit 6 of d0, calls another routine, and then loads a new stack pointer from a memory location.
- `check_vertical_blank_status`: Reads the vertical position register, masks bit 13, and returns the result.
- `update_vbi_register`: Checks if bit 13 is set in the value at the address pointed to by the second word on the stack, and if so, writes the first word from the stack to the video port register.
- `configure_graphics_library`: Calls a library function, and if the result is not zero, updates a byte at offset 530 of a global structure and modifies several fields in the library structure based on the value of the parameter, finally setting bit 1 of a byte at offset 14 of the library structure and calling two other routines.
- `set_structure_fields`: Writes values to a structure pointed to by a1 at offsets 110, 212, and 216, sets bit 1 of a byte at offset 14, and calls two functions via pointers stored in a6.
- `set_ciaa_bit`: Sets bit 1 of the CIAA Port A Data register.
- `clear_ciaa_bit`: Clears bit 1 of the CIAA Port A Data register.
- `load_pointers`: Loads two pointers from the stack into registers a0 and a1.
- `compare_byte_sequences`: Compares two sequences of bytes in a loop, checking if bytes are identical or match when bit 5 is ignored, and returns 1 if the loop terminates via a mismatch or 0 if a specific condition is met.
- `manage_library_reference`: Calls a function via a pointer in a global variable, and if the result is non-zero, calls two other functions via pointers located at offsets -568 and -318 from that result.
- `call_routine_with_constant`: Saves d0 and d1 to the stack, loads the constant 8465 into d0 and d1, calls another routine, restores d0 and d1, and returns.
- `call_routine_with_different_constant`: Saves d0 and d1 to the stack, loads 8192 into d0 and 255 into d1, calls another routine, restores d0 and d1, and returns.
- `conditional_cache_control`: Checks if bit 1 is set in a byte at offset 329 of a global variable, then calls one of two functions via pointers based on whether a 16-bit value at offset 20 of the global variable is less than 37.
- `dispatch_conditional_callback`: Checks if bit 1 is set in a byte at offset 359 of a global structure; if so, it calls one of two function pointers stored at offsets 30 and 604 of that structure depending on whether a 16-bit value at offset 20 is less than 37.
- `execute_library_functions`: Calls a function from a table at an offset of a global variable, and if the result is non-zero, calls three more functions from offsets of that same global variable before returning.
- `mask_pointer_bits`: Calls a library function, and if the result is not zero, performs a bitwise AND on the value at offset 4 of the returned pointer with 61439 and then bitwise ORs it with a global value.
- `tiny_adda_rts`: Small routine of 2 instructions that adds 16 to a7, then returns.
- `modify_pointer_mask`: Takes a pointer from d0, masks the lower 14 bits of the value at offset 4 of that pointer, and then performs a bitwise OR with a constant value before storing the result back at that offset.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `calculate_checksum_and_update`: Calculates a checksum by summing 23 16-bit values from a table starting at offset 34 of a global structure, stores the negated sum at the end of the table, calls a library function, stores the result at offset 586 of the global structure, sets bit 1 of a byte at offset 14, and calls another routine.
- `jump_to_indirect_address`: Calculates an address by loading a value from a global pointer, subtracting 2 from it, and jumping to that location.

### Shanghai (1986)(Activision)[m Headbanger][a].adf:Info

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch_2`: Moves the contents of d1 to d0 and then branches to a different location.

### 16

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### 9

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_value`: Clears the accumulator and returns zero to the caller.

### 12

- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_value_2`: The routine returns zero and does nothing else.

### 8

- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.

### mathtrans.library

- `execute_illegal_instruction`: library code contains an illegal instruction.
- `store_pointer_to_global`: library code stores the 32-bit value from a0 into a global variable at offset 188.
- `update_list_status`: As library code, this routine checks if a value at an offset of 8 in a structure pointed to by a6 is zero; if so, it performs a series of pointer and value assignments and calls another routine, otherwise it sets bit 3 of a byte at offset 14 of the structure and returns.
- `set_status_bit`: library code sets bit 3 of a byte at offset 14 of a structure pointed to by a6, then returns.
- `increment_counter_and_call_library`: library code checks if bit 3 of a byte at offset 14 of a structure in a6 is clear; if so, it increments a 16-bit word at offset 32 of that structure, and if the new value is 1, it calls another routine via a global variable and calls a library function.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `mask_and_branch`: Library code that performs a bitwise AND on register d0 with 127 and calls another routine.
- `calculate_bitfield_difference`: As library code, this routine checks if the byte in d0 is zero or negative, otherwise it enters a loop that performs bitwise operations and arithmetic involving a doubling accumulator and a running difference until a specific condition is met.
- `adjust_bitfield_value`: library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `modify_register_bits`: As library code, this routine moves d0 to d7, calls another routine, sets bit 1 of d0, clears bit 3 of d0, and then branches to another location.
- `call_jump_table_entries`: library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_and_branch_3`: As library code, it clears d1 and branches to another location.
- `execute_conditional_library_calls_2`: As library code, this routine performs a series of conditional calls to routines located at various offsets from a pointer in a5, depending on the value of a flag and a register.
- `execute_conditional_library_calls`: This library code calls a routine at an offset of minus 42 from a pointer in a5, then performs conditional calls to routines at offsets of minus 78, minus 48, minus 48, minus 66, minus 66, minus 74, and minus 72 from the same pointer based on the values of d6 and d7.
- `execute_conditional_calls_2`: This library code performs a series of conditional calls to routines located at various offsets from a pointer in a5, depending on the values of d6, d4, and d7, and finally returns a bitwise masked version of the result from another routine.
- `execute_multiple_calls`: Library code that moves d7 to d1, calls a routine at an offset from a5, and if the result is not negative, performs two more calls using constant values in d0 and d1 before moving d0 to d2 and branching.
- `call_offset_routine`: As library code, this routine moves d2 to d0, calls another routine using an offset from a5, moves the result from d0 to d2, and then branches to another location.
- `branch_on_d4_value`: As library code, this routine moves d2 to d0, tests d4, and if d4 is not zero it branches to another location; otherwise it loads a constant into d2 and branches to another location.
- `mask_return_value`: library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `perform_bit_shifting_loop`: This library code performs a loop that shifts a value and decrements a counter, calling another routine if the counter reaches minus 1, and uses bitwise operations to calculate a result based on a value in d7.
- `initialize_registers_and_branch`: This library code performs a logical shift right on d7, clears d6, loads a constant into d5, loads the address of a 4-byte value into a0, and sets d1 and d2 to specific constants before branching to another routine.
- `execute_loop_with_branching`: Library code that executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch`: As library code, it pushes the word value 65534 onto the stack and then branches to an offset.
- `push_and_branch_alt`: As library code, it pushes the word value 65535 onto the stack and then branches to an offset.
- `push_and_branch_short`: As library code, it pushes the word value 1 onto the stack and then branches to an offset.
- `compare_and_return`: As library code, it compares a byte in d7 to 184; if it is higher, it branches to another routine, otherwise it increments the stack pointer by 2, tests d7, and returns.
- `process_byte_data`: As library code, it performs a series of comparisons on a byte in d7 and calls another routine if certain conditions are met.
- `perform_division_via_subtraction`: Performs a division-like operation using a loop of subtractions and bit shifts on the values in d6 and d7, while simultaneously adding values from a memory pointer into d6 and subtracting values from that same pointer into d7.
- `perform_complex_math_2`: This library code executes a loop that performs arithmetic operations on d6 and d7 using values from a pointer in a0 and a global variable.
- `perform_complex_math`: As library code, it executes a loop that performs arithmetic operations on d6 and d7 using values from a pointer in a0 and a global variable, while decrementing a counter in s3.
- `execute_complex_loop`: Library code performs a loop that manipulates d5, d6, and d7 using values from a pointer in a0, performing arithmetic and bit shifts, and calls other routines based on the results.
- `load_constant_and_branch`: As library code, it loads the constant an address into d7 and branches to another routine.
- `call_recursive_wrapper`: This library code checks if d6 is zero, otherwise it moves d5 to d6, calls another routine, moves the result to d7, tests d7, restores registers from the stack, and returns.
- `push_and_call_routine_2`: This library code pushes the contents of d5 onto the stack, calls a routine, pushes d6 onto the stack, restores d6 from the stack, and then branches to another routine.
- `compare_and_branch_2`: Library code that moves d6 to d4, branches if d4 is negative, otherwise compares d6 to 255 and branches if it is higher, or clears d6 and returns.
- `shift_and_jump`: As library code, it performs an arithmetic shift right on d4 by 8 bits, increments d4 by 1, and if d4 is zero, it branches to another routine; otherwise it jumps to a zero address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `process_conditional_calls`: Library code checks a byte in d7, and if it is non-zero, performs a rotate right through the carry and potentially calls several other routines based on the result.
- `rotate_and_branch`: As library code, it loads the constant an address into d7, performs a rotate right through the carry on d7 by 1 bit, and branches to another routine.
- `load_constant_to_d7`: Loads the constant value a fixed address into register d7 and then branches to another routine.
- `check_stack_word`: Loads a word from the stack into register d6, tests if it is positive, and otherwise clears register d7 before branching to another routine.
- `set_condition_code_and_restore_registers`: Sets bit 1 of the condition code register and restores registers d1 through d6 and a0 from the stack before returning.
- `set_register_and_return`: Loads a constant value into a data register, adjusts the stack pointer by an offset of 30, and returns.
- `process_conditional_data`: This library code checks if d7 is zero, otherwise it performs a series of conditional calls to other routines and manipulates a 16-bit value derived from d7.
- `execute_unreachable_calls`: This library code performs four consecutive calls to another routine and then returns.
- `perform_arithmetic_loop`: Library code that increments d2, moves d5 and d6 to d3 and d4, performs arithmetic shifts on d3 and d4, and then enters a loop that adds d4 to d5, adds d3 to d6, and subtracts a value from a table pointed to by a0 until a counter in d1 reaches zero.
- `subtract_table_from_accumulator`: Subtracts register d4 from d5, subtracts register d3 from d6, adds a value from a table pointed to by a0 to d7, and repeats this in a loop while decrementing register d1 until it reaches a sentinel value.
- `test_and_modify_d7_2`: This library code tests the byte in d7, branches if it is zero or positive, otherwise it performs a bitwise AND on d7, calls another routine, and sets bit 1 of the condition code register before returning.
- `set_d7_and_branch`: Sets register d7 to negative one and branches to another routine.
- `perform_complex_arithmetic_loop`: This library code performs a series of calls to other routines and executes a loop that performs bit-shifting and arithmetic operations on values derived from d7 and a constant.
- `complex_loop_with_branching`: Library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_call_routine`: As library code, it tests d7, branches if positive, otherwise masks d7 with 127, calls another routine, and sets the bit 1 of the condition code register.
- `process_conditional_logic`: Library code calls a routine, saves d3 through d5, calls another routine, and then performs a series of conditional checks and calls based on the value in d7 before jumping to another routine.
- `transform_register_value`: Adds the contents of a register to itself, then conditionally performs a series of bitwise operations including an XOR, an arithmetic shift, a subtraction, a word swap, and a left rotation before returning.
- `rotate_and_add_d7`: This library code swaps the high and low words of d7, performs a seven-bit left rotation, performs an exclusive OR on the byte, adds d7 to itself, and uses conditional branches based on the overflow flag to perform further additions and rotations.
- `test_and_modify_d7`: This library code branches based on a condition, compares d7 to specific byte constants, and if certain conditions are met, adds a value to d7, performs a rotate left through carry, and tests the result before branching.
- `mask_and_test_d7`: Performs a bitwise AND on register d7 with the mask 65279, tests if the result is zero, and if not tests the byte value of the result before branching to another routine.
- `test_and_shift`: Library code that compares d7 to 254, branches if not equal, otherwise performs two logical shifts on d7 and branches based on the result.
- `shift_and_set_cc`: Shifts register d7 left by 8 bits, loads 255 into d7, rotates the bit from d7 through the carry flag, sets bit 1 of the condition code register, and then branches to another routine.
- `clear_d7_and_branch`: Clears register d7 and then branches to another routine.
- `invoke_callback_and_return`: library code that receives a pointer in a0 and a value in d0, calls the routine pointed to by a0, and returns the value originally in d0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_4`: Library code that loads a longword from a constant address into a0 and branches to the next instruction.
- `load_address_and_branch_9`: This library code loads a long value from a constant address into a0 and then performs an unconditional branch.
- `load_address_and_branch_7`: As library code, it loads a 32-bit constant into a0 and branches to the start of the routine.
- `library_return_zero`: Library code that loads a zero into a0 and branches to the next instruction.
- `load_address_and_branch_10`: As library code, it loads an address from a memory location into a0 and then branches to the next instruction.
- `load_address_and_branch_11`: It is library code that loads a long value from an offset into a0 and then branches to the address in a0.
- `load_pointer_and_branch_4`: Loads the address of a global variable into a0 and then branches to a relative offset.
- `store_register_via_pointer`: It is library code that invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `load_address_and_branch_8`: Loads the address of a global variable into a0 and then performs an unconditional branch to the next instruction.
- `load_null_address_and_branch`: As library code, it loads a null pointer into a0 and branches to the start of the routine.
- `load_callback_address`: Loads the address of a global variable into a0 and returns immediately.
- `load_address_and_branch_5`: As library code, this routine loads a constant address into a0 and then branches to the next instruction.
- `load_address_and_branch_6`: Library code loads a longword from an offset of 20 from a pointer in a0 into a0 and then branches.
- `load_pointer_and_branch_3`: Loads a longword from a global variable into a0 and then performs a short branch to the next instruction.
- `load_address_and_branch`: Loads a zero value into a register and branches to another routine.
- `load_zero_address`: This library code loads a longword of zero into a0 and then branches to another routine.
- `load_callback_pointer`: Loads a pointer from a location at offset 20 of a data structure into register a0 and then returns immediately.
- `prepare_indirect_call`: As library code, this routine loads a constant into a0 and branches to another routine.
- `dispatch_indirect_call`: Loads the address of a global variable into a register and branches to another routine to perform an indirect call using an offset from that variable.
- `load_pointer_and_branch`: Loads a longword from a global variable into address register zero and performs a short branch to another routine.
- `load_pointer_and_branch_2`: As library code, it loads a long value from a specific memory location into a0 and branches to another routine.
- `load_indirect_call_address`: Loads the address of a global variable into a register and branches to another routine.
- `call_indirect_via_offset`: Loads a pointer into a register and branches to another routine that calls a function at an address calculated by adding an offset to the value in that register using a global variable.
- `load_address_and_branch_2`: Loads a longword into a register from a global variable and branches to a location within another routine.
- `load_address_and_branch_to_offset`: Loads a longword from a global variable into a0 and branches to a relative offset.
- `load_address_and_branch_3`: Loads a longword into a register from a global variable and branches to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### icon.library

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_status`: Library code that checks if a 16-bit value at offset 8 of a structure is zero; if so, it calls a routine through a pointer at offset 34, updates a pointer in a list, and returns a value; otherwise, it sets bit 3 of a byte at offset 14 and returns zero.
- `set_bit_in_structure`: The library code sets bit 3 of a byte at offset 14 of a structure pointed to by a register and returns 0.
- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `update_icon_data`: The library code copies two 32-bit values from a table into a structure, calls a routine via a pointer in a library base, and updates global variables based on the result.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `perform_logical_or`: The library code performs a bitwise logical OR on register D0 with zero.
- `clear_d0_register`: This library routine performs an OR operation on register d0 with zero.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `manage_icon_chain_3`: This library code calls another routine using a pointer in a6 and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `manage_icon_chain_4`: Pushes two pointers onto the stack and calls another routine to validate and update an icon structure, then returns.
- `call_incremental_offset_routine`: Saves two registers to the stack, calls another routine, and then returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_other_routine_2`: Calls another routine and then returns.
- `call_parse_icon_string`: Calls parse_icon_string and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clears_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `recursively_update_icon_chain`: Calls another routine with the first argument, and if the result is non-zero, calls another routine with the first argument and a second argument equal to the result plus 140 decimal; otherwise it calls another routine with the first argument and the result, then returns 0.
- `copy_and_update_structure`: Copies 43 bytes from a source pointed to by a6 to a destination pointed to by a2 using a loop, then copies several longwords from the source to specific offsets within the destination structure, and finally updates a pointer at offset 76 of the destination structure if the longword at offset 76 is not zero.
- `return_zero_stub`: This library routine clears register d0, unlocks register a6, and returns.
- `copy_icon_data_to_stack`: The routine copies a fixed number of bytes from a structure pointed to by the third argument into a local stack buffer, then copies several fields from the same structure into local stack slots and calls another routine with the first two arguments and a pointer to a two-word stack header.
- `populate_icon_structure_data`: Validates a structure by checking for specific magic values and flags, then populates various offsets within the structure by calling other routines.
- `populate_icon_slot_data`: Checks a structure for specific magic values and flags, then recursively calls other routines to populate various offsets within that structure or a destination pointer.
- `execute_callback_chain_3`: Library code traverses a structure by repeatedly calling two different routines, storing results in a pointer-based destination and decrementing a counter.
- `execute_callback_chain_2`: This library code executes a loop that calls two different routines repeatedly, storing a result in a pointer-based destination and decrementing a counter in a structure, then performs conditional calls to update a value at an offset of 70 in a structure.
- `clear_and_branch`: Sets d6 to zero and then branches to another routine.
- `unlock_and_return`: This library routine unlocks register a6 and then returns.
- `traverse_icon_data_structure`: Iterates through a complex icon data structure by following various pointers and nested lists, including a loop that traverses a list of pointers to call another routine for each entry.
- `traverse_icon_structure_links`: Iterates through a linked list of pointers and nested structures within an icon data block, updating a counter based on the number of elements processed.
- `process_string_list_3`: Library code that iterates through a list of pointers, calling dispatch functions for each entry and updating a value at an offset of 26 in a str.
- `process_string_list_2`: Library code iterates through a list of pointers, calling dispatch functions for each entry and updating a value at an offset of 26 in a structure.
- `process_string_list`: This library routine iterates through a list of pointers, calling dispatch functions for each entry.
- `traverse_icon_chain`: Iteratively calls another routine with an incremented pointer, and if the result is non-zero, calls that result as a routine.
- `restore_registers_and_return`: Restores d2 and d3 from the stack and returns.
- `incremental_offset_recurse`: Library code that calls another routine with the same first argument and the second argument increased by 88 decimal, then returns.
- `execute_conditional_library_calls_3`: As library code, it calls a routine to get a value, and if that value is non-zero, it calls another routine with an offset of 142 from that value, then calls a third routine with the original parameter and an offset of 140 from that value.
- `call_routine_with_parameters`: Pushes a pointer and two registers onto the stack, calls another routine, and if the result in d0 is zero, moves the value from the pointer in a2 into d0; otherwise it returns.
- `clear_d0_and_restore_registers_3`: Sets d0 to zero, restores d2, d3, d4, and a2 from the stack, and returns.
- `update_table_element_2`: It is library code that decrements a 16-bit counter through a pointer, and if the result is zero or the original value was less than one, it calls another routine; if that routine returns a non-zero value, it decrements the counter again, then uses a value from the structure to compute an address and stores two 32-bit values at that address.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, computes an address from a table base and the counter, and stores two 32-bit values from registers and the stack into consecutive slots of that table.
- `validate_and_update_icon_structure`: Checks if a routine returns a non-zero value, then performs a series of conditional calls to update or reset values within a structure based on the results of those calls.
- `move_and_branch`: Copies the contents of d6 into d0 and then branches to another routine.
- `call_lib_func_and_return_zero`: Invokes a function pointer retrieved from a global variable associated with library index -210.
- `incremental_offset_recurse_2`: Library code that calls another routine with the same first argument and the second argument increased by 78 decimal, then returns.
- `search_memory_for_character`: This library code enters a loop that calls another routine to traverse a list of pointers, comparing bytes at specific memory locations until a specific character is found or the list ends.
- `clear_d0_and_restore_multiple_registers`: Sets d0 to zero, restores d2, d3, a2, and a3 from the stack, and returns.
- `iteratively_search_and_validate_data`: Calls another routine to initialize a value, then enters a loop that repeatedly calls another routine with a constant value of 124 to update a parameter.
- `process_icon_data`: As library code, it performs a loop that compares bytes from two pointers in a0 and a1, and calls a supervisor routine if a condition is met.
- `branch_on_register`: Library code that checks if register d4 is zero, otherwise it sets d0 to the value of d4 plus one, moves it to d2, and branches to another routine.
- `clear_and_branch_short`: Sets d2 to zero and then branches to another routine.
- `clear_d0_and_restore_data_registers`: Sets d0 to zero, restores d2, d3, d4, d5, and d6 from the stack, and returns.
- `parse_icon_string`: Library code that clears a byte at offset 30 of a structure, calls another routine with a global variable, and then performs a loop to convert a sequence of bytes into an integer using a multiplier of 10.
- `update_icon_pointer_and_call`: Increments a pointer in a2 by 5, pushes two constants onto the stack, calls a routine using the pointer in a2, and then either branches to a different location or sets d5 to 1 and increments a2 by 3 before branching to another location.
- `clear_d5_and_branch`: This library code moves zero into d5 and then branches to another routine.
- `calculate_icon_offset`: As library code, it performs a loop that updates a value in a0 using a multiplier, and calls a library function at an offset from a global variable if a condition is met.
- `calculate_offset_range`: Loads a longword from the stack into d1, compares it against constant values, and sets d0 to either a constant, a calculated difference, or a specific value based on the comparison results before branching to another routine.
- `return_negative_one_3`: Sets d0 to negative one and returns.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `complex_library_traversal`: Library code that calls another routine and uses the result to conditionally call a second routine or perform a cleanup call.
- `update_table_element`: As library code, it calls a routine to get an index, checks if the result is non-zero, and if so, writes the value 10 to a 16-bit word at an offset of 14 from a pointer returned by another routine, then writes 10 to a pointer passed in a parameter and calls another routine with that pointer and the returned pointer, before clearing d0 and restoring d2, d3, and a2.
- `clear_d0_and_restore_registers`: Clears register d0 and restores d2, d3, and a2 from the stack before returning.
- `verify_library_return`: Calls a library function, compares the return value in d0 to a parameter in d4, and if they are not equal, sets d0 to zero before returning.
- `move_d1_to_d0`: This library routine copies the contents of register d1 into register d0 and then returns.
- `validate_return_value`: Calls another routine and compares its return value to a fourth parameter; if they are not equal, it returns zero, otherwise it returns the result of the called routine.
- `transfer_register_to_d0`: Copies a longword from register d1 into register d0 and restores the previous values of registers d2 and d3 from the stack before returning.
- `compute_weighted_sum_wrapper`: This library routine calls another routine to compute a binary weighted sum of two parameters.
- `dispatch_library_function`: This library routine retrieves a function pointer from a global table and invokes it with a single parameter.
- `rotate_32bit_value_2`: If a value in a register is greater than 65535, it performs sixteen iterations of a left bit rotation on a 32-bit value, adjusting the result by a subtraction if it exceeds a third value; otherwise, it performs modulo and division operations using the third value.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_and_decrement_pointer`: Library code that loads a pointer from the stack into a0 and a comparison value into d0, then reads a byte from the address in a0 into d1; if d1 matches d0, it decrements the pointer in a0 and returns, otherwise it branches to another routine.
- `branch_to_offset_3`: Library code that clears register d0 and performs a relative branch.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `search_for_null_terminator`: Restores registers from the stack and checks if the byte at the address in a0 is zero; if it is not zero, it branches to another routine, otherwise it decrements a0 and branches to a later instruction within the same routine.
- `compare_memory_bytes`: Library code that decrements a value from a stack parameter, then compares a byte from a pointer in a1 with a byte from a pointer in a0, returning zero if they match and the byte in a1 is zero, otherwise it returns.
- `set_conditional_d0`: Sets d0 to one if a comparison succeeds, otherwise sets d0 to negative one and returns.
- `check_byte_and_decrement_d0`: Restores a0 and a1 from the stack, loads a value from the stack into d0, checks a byte at the address in a0, and branches to another routine if the byte is zero; otherwise, it decrements d0 and branches based on its value.
- `decrement_and_call`: Decrements the address in a0 by one and calls another routine before returning.
- `copy_bytes_with_counter_limit`: Decrements d0 and branches if it is less than zero; otherwise, it copies a byte from the address in a1 to the address in a0 and loops if the byte was not zero, or clears a byte and branches if d0 reaches zero or less.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the value at that location by four, clears the word at offset four from that location, and stores the original pointer address into the word at offset eight from that location.
- `conditional_library_branching`: The library code checks a parameter and, if non-zero, calls another routine and potentially calls a third routine using a value from a pointer offset.
- `traverse_linked_list_2`: This library code enters a loop that repeatedly calls another routine with values derived from a structure in a0, updating the pointer to the next element in the list until a zero value is encountered or a call returns zero.
- `set_d0_and_branch_2`: Sets register d0 to 1 and then branches to another routine.
- `clear_d0_and_restore_registers_2`: Clears register d0 and restores registers d2 through d5 and a2 from the stack before returning.
- `execute_nested_logic`: Library code that performs a series of conditional calls using 32-bit integer multiplication and callback invocation, storing a result at offset 10 of a structure and returning either a pointer or negative one.
- `return_negative_one`: This library routine sets register d0 to negative one and then returns.
- `process_linked_data_2`: This library code enters a loop that repeatedly calls other routines to traverse a structure, storing results at specific offsets and returning a value or negative one based on the success of the calls.
- `process_linked_data`: Library code that enters a loop to repeatedly call routines and store results at offsets 8 and 12 of a structure, returning a value from a register or negative one if a condition is not met.
- `transfer_and_branch`: Copies the contents of a longword from a memory location pointed to by a3 into d0 and then branches to another routine.
- `return_negative_one_2`: Sets the d0 register to all bits set and restores all registers from the stack before returning.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### asl.library

- `return_negative_one_4`: Loads negative one into a register and returns.
- `update_callback_data`: As library code, it updates several fields in a data structure using values from registers and results from other routines, and populates a table with a series of descending values using a loop.
- `execute_indirect_jump`: Library code that calls a routine at an offset of 556 decimal from a6, then loads a pointer from the stack and jumps to it if the return value in d0 is non-zero.
- `execute_indirect_call`: Calls a routine located at an offset of negative 108 from the address in a6, then restores d7, a0, and a5 from the stack and returns zero.
- `execute_indirect_calls`: Calls a routine located at an offset from the address in a6 seven times.
- `copy_pointer_to_register`: Copies the longword from the address pointed to by register a0 into register d1 and then branches to another routine.
- `copy_byte_loop`: Copies a byte from a memory location pointed to by a1 to a location pointed to by a0, decrements d0, and loops if d0 is not zero, otherwise it branches to another routine.
- `zero_fill_byte_buffer`: Clears bytes in a buffer pointed to by a0 for the count specified in d0 using a loop, then returns.
- `copy_c_string_2`: Advances to the terminating zero of the string at a0, then copies the bytes from a1 onto its end, including the zero.

### mathieeesingtrans.library

- `executes_illegal_instruction`: Triggers an illegal instruction exception by executing an undefined opcode.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `update_math_structure_3`: As library code, it saves registers to the stack, calls a routine at an offset from a6, and stores the result in a location specified by a0.
- `update_math_structure`: Library code that calls two different routines, stores a 16-bit value in a global variable, and populates a structure with several pointers and bytes before calling a final routine.
- `update_structure_fields_2`: Calls a routine to get a value, stores that value in a 16-bit global variable, and writes several fields including a pointer, a byte, and two 32-bit values into a structure pointed to by the returned value.
- `raise_exception_for_code`: Masks the low word of a data register and compares it against two specific codes.
- `compare_threshold_value`: Library code that compares a 16-bit value in d7 to 7680 and branches to another routine if the value is greater than or equal, otherwise it returns.
- `branch_on_threshold`: Library code that compares a 16-bit value in d7 to 7680 and branches to another routine if the value is greater than or equal, otherwise it branches to another routine.
- `compare_d7_and_branch`: Library code that compares a 16-bit value in register d7 to 24064 and branches if it is greater than or equal, otherwise it returns.
- `compare_and_branch_d7`: Compares a 16-bit value in d7 to 24064 and branches to a different location if it is greater than or equal, otherwise it branches to another location.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `process_table_data`: As library code, it accesses a 16-bit word from a table at an offset of 8 from a pointer found at an offset of 36 from a6, writes a 32-bit value to a location offset by 8 from that pointer, and enters a loop that checks values in the table and calls another routine if certain conditions are met.
- `load_conditional_value`: As library code, it iterates through a table pointed to by a0, calling another routine if a value is not equal to 4900 or between 901 and 7679, and stores a result in the location pointed to by a1.
- `check_value_and_load`: Compares d7 against two specific 16-bit constants and branches to another routine if they match, otherwise it loads a 32-bit value from a1 into d0, tests a word at a0, and either branches to another routine if the value is negative or loads a 32-bit value from the stack into d7 and returns.
- `process_table_data_2`: It is library code that iterates through a table, calling another routine if values meet specific criteria, and writes values from parameters into the table.
- `validate_and_fill_table`: As library code, it iterates through a table, calling another routine if values meet specific criteria, and writes values from parameters into locations pointed to by the table elements after performing various checks and writes to offsets of the table elements.
- `fetch_validated_pointer`: As library code, it iterates through a table pointed to by a0, calling another routine if a value is not equal to 4900 or between 901 and 7679, and stores a parameter from the stack into the location pointed to by a1.
- `validate_and_fetch_pointer`: Compares a 16-bit value in register d7 against two specific constants and, if neither matches, loads a longword from the address pointed to by a1 into d0, checks the sign bit of a word at the address in a0, and either branches to another routine or increments a pointer in a7 before returning.
- `return_zero`: Clears a register and returns.
- `check_word_threshold`: Library code that loads a 32-bit value from the stack, clears bit 31, swaps the high and low words, and if the resulting 16-bit value is greater than 13183, branches to another routine; otherwise it loads a constant into d0 and returns.
- `call_library_function_3`: It is library code that compares a 16-bit value in d1 to 14711; if it is higher, it branches to another routine, otherwise it saves a6 to the stack, loads a new address into a6, sets d1 to a constant, and calls a routine at a negative offset from a6 before restoring a6 and returning.
- `compare_and_branch_3`: As library code, it compares a 16-bit value in d1 to 15792 and branches to another routine if it is higher, otherwise it branches to a different offset.
- `compare_word_and_branch`: Compares a 16-bit value in d1 to 15664 and branches to another routine if the value is higher, otherwise it branches to another routine.
- `set_floating_point_infinity_2`: Library code that compares a 32-bit value in d0 to a constant and, if the value is greater than that constant, sets d0 to zero and returns.
- `set_floating_point_infinity`: Library code that compares a 32-bit value in d0 to a constant and, if the value is greater than that constant, sets d0 to a specific value before returning; otherwise it branches to another routine.
- `evaluate_floating_point_exponent`: Checks a value in d0 against a constant, and if it exceeds it, performs a series of library calls and conditional branches based on a 15-bit mask.
- `set_d1_and_call_library`: Compares a 16-bit value in d1 to 14711; if it is higher, it branches to another location, otherwise it sets d1 to a constant and calls a routine at an offset of minus 66 from a pointer in a6 before branching to another location.
- `compare_and_branch_on_value`: Compares a 16-bit value in d1 to 15312; if the value is higher, it calls another routine, otherwise it calls a different routine and then branches to another location.
- `branch_math_logic`: As library code, it compares a 16-bit value in d1 to a constant and branches to a routine or another location based on the result.
- `calculate_iterative_value`: Library code that uses a loop to decrement a value using elements from a global array, then calls a routine and returns the result adjusted by a constant.
- `process_data_loop`: Enters a loop that modifies d0 using values from a pointer in a0, and contains a nested loop that decrements d6, updates a0, and checks d0.
- `perform_math_operations`: Calls a routine at an offset from a6, then calls another routine at a different offset twice using modified values in d1.
- `call_library_functions`: Loads a pointer from a global variable and calls the function at offset minus 78 of that pointer three times, then calls the function at offset minus 66 once, and the function at offset minus 66 once more, and finally calls the function at offset minus 66 once more, then returns.
- `process_floating_point_value`: As library code, it loads a value from an offset of minus 56 from a pointer in a5 into d0, calls another routine, and if the result is not greater than zero, stores that result into offset minus 4 of the structure in a5 before branching to another routine.
- `perform_floating_point_sequence`: Manipulates a 16-bit value at a negative offset from a pointer by masking and setting specific bits, then executes a long sequence of calls to another routine, storing each returned value into a series of consecutive memory locations within a structure.
- `compute_floating_point_absolute_value`: Performs complex floating-point arithmetic and sign manipulation, including conditional absolute value calculations and bitwise XOR operations on the sign bit, while utilizing a provided function pointer for specific mathematical operations.
- `update_structure_math_fields`: Updates various fields within a structure pointed to by a5, including performing calculations based on values at offsets minus 64 and minus 76, and conditionally executing other routines to update offsets minus 64, minus 60, minus 56, minus 44, minus 48, minus 36, minus 44, minus 40, minus 36, minus 32, minus 28, minus 20, minus 48, minus 28, minus 20, minus 12, minus 20, minus 16, minus 8, and minus 4.
- `branch_to_offset`: As library code, it performs a short branch to another location within the library.
- `update_structure_fields_conditionally`: Updates various fields in a structure pointed to by a5 based on the results of multiple calls to other routines.
- `update_structure_values_2`: As library code, it updates various fields in a structure in a5 by calling other routines and storing the results in specific offsets.
- `loop_update_structure_values`: As library code, it loads values from offsets minus 84 and minus 68 of a structure in a5 into d0 and d1, calls another routine, stores the result in offset minus 28 of the structure, moves that value to offset minus 84, and loops until d2 is zero before branching to another routine.
- `update_structure_values`: As library code, it calls another routine using values from offsets minus 68 and minus 44 of a structure in a5, stores the result in offset minus 52, and subsequently updates offsets minus 28, minus 54, minus 24, minus 20, and minus 4 of that structure based on various conditional calls to another routine.
- `branch_conditionally_on_values`: As library code, it loads values from offsets minus 68 and minus 76 of a structure in a5 into d0 and d1, calls another routine, and then branches to either another routine or a different location based on the comparison result.
- `execute_library_function`: Calls a function using two arguments from offsets minus 68 and minus 68 of a structure in a5, stores the result in offset minus 52 of that structure, copies that result to offset minus 4 of the structure, and returns the value.
- `return_zero_2`: Clears register d0 and returns immediately.
- `perform_nested_library_calls`: As library code, it calls a routine with three parameters, then calls another routine with no parameters, then calls a third routine with the result of the second call as its parameter, and returns the final result.
- `perform_nested_library_calls_2`: As library code, it calls a routine with three parameters, then calls another routine with no parameters, and finally calls a third routine with one parameter.
- `modify_word_data`: As library code, it checks a value at a pointer; if a calculation involving that value is less than negative 125, it clears the word at that pointer, otherwise it masks the word and updates it by shifting a calculated value left by 7 bits and performing a bitwise OR.
- `modify_word_at_pointer`: Takes a word from register d1, adds 126 to it, masks the result to 8 bits, shifts it left by 7 bits, and performs a bitwise OR with the word at the address stored in a1 after masking the original word with 32767.
- `call_library_function_via_offset`: Loads a library base from a global variable, calls the function located at offset minus 78 from that base, and returns the result to the caller.
- `call_library_function_via_offset_2`: Fetches a function pointer from a global variable, calls the function at a negative offset from that pointer, and returns.
- `call_library_function_via_offset_3`: Loads a function pointer from a global variable, calls the function at a negative offset from that pointer, and returns.
- `call_library_function_via_pointer`: Fetches a function pointer from a global variable and calls it using an offset of 84 bytes from that pointer, then returns.
- `call_library_function_via_offset_4`: Loads a function pointer from a global table, calls the routine at a negative offset from that pointer, and returns.
- `call_library_function_via_offset_5`: Loads a function pointer from a global variable, calls the routine at an offset of negative 30 from that pointer, and returns.
- `call_library_offset_minus_36`: Library code that loads a base address from a global variable, calls a routine at an offset of minus 36 from that base, and returns.
- `call_library_offset_minus_54`: Library code that loads a base address from a global variable, passes the value in D0 to a routine at an offset of minus 54 from that base, and returns.
- `call_library_offset_minus_90`: Library code that loads a base address from a global variable, passes the value in D0 to a routine at an offset of minus 90 from that base, and returns.

### mathieeedoubbas.library

- `return_constant_9`: Returns the constant -1 in d0 to the caller without any other effect.
- `clear_d0_and_return`: Library code that clears the D0 register and returns immediately.
- `initialize_ieee_operation_structure`: Obtains an operation structure via a function call, populates it with two routine addresses, the caller's arguments, and a flag, then calls another routine before returning.
- `init_ieee_operation_struct`: As library code, it calls a vector to obtain a structure, stores two routine addresses and the caller's arguments into it, and sets a flag byte from a bit in a global variable.
- `raise_ieee_exception_code`: Library code that stores two into a 16-bit field of the structure at A0, then, depending on the low byte of the exception code in D7, deliberately divides by zero or raises the overflow trap.
- `process_exception_status`: As library code, it performs a loop that inspects a word at a pointer in a6 and calls another routine based on specific bit patterns.
- `process_exception_status_loop`: Enters a loop that inspects a word at a pointer in a0, calling another routine and writing either 16384 or 29696 to offset 20 of the structure in a0 based on specific bit patterns and values found in the word.
- `compare_and_load_registers`: As library code, it compares d7 to two specific constants, and if neither matches, it loads values from a pointer in a1 into d0 and d1, tests a word at a pointer in a0, and if that word is not negative, loads a value from the stack into d7 before returning.
- `reset_and_check_status_word`: Loads a pointer from an offset of a6, writes zero to offset 6 of that pointer, and returns if the high byte of the word at offset 6 is zero; otherwise it returns.
- `validate_and_set_exception_flag`: Enters a loop that repeatedly reads a word from a pointer in a0 until a non-negative value is found, then writes 36864 to offset 10 of that structure.
- `compare_and_clear_register`: As library code, it compares d7 to two specific constants, and if neither matches, it stores a constant in a1, tests a word at a pointer in a0, and if that word is not negative, clears d0 and loads a value from the stack into d7 before returning.
- `calculate_double_precision_division`: Performs high-precision floating-point division by extracting mantissas and exponents from input registers, normalizing the values, and calculating the quotient and remainder through a series of bitwise shifts and arithmetic operations.
- `compare_math_registers`: As library code, it compares d7 with d2 and a0 with d2, then checks if d3 is zero before branching to another routine.
- `return_hardcoded_double`: Returns a hardcoded 64-bit floating-point constant and then branches to another routine.
- `xor_and_swap_register`: As library code, it checks if d2 or d3 are non-zero; if both are zero, it copies a0 to d0, performs an XOR on d0 with d4, swaps the high and low words of d0, and restores d2 through d7 from the stack before returning.
- `branch_to_xor_routine`: As library code, it performs a short branch to another routine.
- `compare_and_branch_registers`: Compares a word in d7 with d2 and branches if d7 is less than d2, otherwise it compares a longword in a0 with d2 and branches if they are not equal, then tests d3 and branches if it is zero before moving d2 to d0, d3 to d1, and branching to another routine.
- `shift_16_bit_value_left`: Shifts a 16-bit value left by 48 bits and returns the 64-bit result.
- `test_registers_and_branch`: Performs an AND operation between d7 and d5, then branches if the result is non-zero; otherwise, it tests d0, d1, d2, and d3, branching to different locations if any of these registers are non-zero, and finally performs an unconditional branch to another location.
- `branch_to_shift_routine`: Performs a short branch to a routine that shifts a 16-bit value left by 48 bits to produce a 64-bit result.
- `branch_to_subroutine`: Calls another routine and then branches to another location.
- `modify_d0_and_branch_on_d7`: Performs an XOR on d0 with d5, an XOR on d0 with 16, and an AND on d7 with d2, then branches to another routine if the result of the AND is non-zero; otherwise, it checks if d2 and d3 are both zero before branching to another routine.
- `exchange_and_call`: As library code, it exchanges d0 with d2, d1 with d3, and d5 with d7, calls another routine, then performs the same exchanges again before branching to an offset.
- `modify_registers_and_branch`: As library code, it performs arithmetic operations on d7, d2, and d5, then branches to another routine if the overflow flag is clear, otherwise it branches to another routine.
- `perform_64_bit_integer_division`: Executes a multi-step division of a 64-bit integer by a 16-bit divisor using high and low word manipulations and iterative arithmetic adjustments.
- `divide_64_bit_integer`: Performs a 64-bit integer division by a 16-bit divisor using a multi-step arithmetic process involving high and low word manipulations, then returns the quotient and remainder.
- `perform_64bit_bit_shift_2`: Performs a bitwise logical shift on a 64-bit value using a shift amount provided in a register, involving a loop to process the bit manipula.
- `perform_64bit_bit_shift`: Performs a bitwise logical shift on a 64-bit value using a shift amount provided in a register, involving a loop to process the bit manipulation and returning the resulting 64-bit value.
- `modify_math_constant`: As library code, it negates, shifts, and adds to a 16-bit value in d5, then branches based on a comparison with a constant.
- `scale_double_register_pairs`: Fragment of double-precision arithmetic that shifts register pairs in a loop until a count reaches its end, with a call to another routine.
- `shift_and_branch_d5`: As library code, it performs a logical shift right on d0, rotates the result into d1, and branches based on a comparison of d5 with zero, otherwise it branches to another location.
- `perform_bit_manipulation`: As library code, it performs bitwise shifts and logical operations on 64-bit data based on a value in d5, and either enters a loop to process the data in chunks or returns a calculated result.
- `branch_to_another_routine`: Performs a short relative branch to another routine.
- `count_leading_zero_bits`: As library code, it shifts the 32-bit value formed by d0 and d1 left in 16-bit and single-bit steps until its top bit is set, uses a table to determine further shifts, and returns the adjusted values in registers.
- `calculate_double_precision_value`: Library code that performs complex bitwise manipulations and shifts on input values from D0 and D1, using a table lookup to determine shift amounts and combining results into a 64-bit value.
- `shift_right_pair_by_count`: Shifts a register pair right by a count taken from the 16-bit value in D5 and returns the shifted pair, with the exponent word of the operand XORed into it.
- `shift_word_left`: Library code that shifts a 16-bit value left by 48 bits and returns it as a 64-bit result.
- `promote_half_to_upper`: Library code that takes the low 16 bits of a 32-bit value held in a register and places them into the upper half of a 64-bit result, leaving the lower half zero.
- `build_fixed_double_constant`: Library code that combines a 16-bit value with a constant mask, scales it, and returns a fixed 64-bit constant pattern.
- `shift_xor_word_left`: Library code that xors a 16-bit value with a constant, shifts the result left by 48 bits, and returns it as a 64-bit value.
- `return_hardcoded_64bit`: Library code that returns a single hardcoded 64-bit constant.
- `set_signaling_bit_return_pattern`: Library code that if bit three of D0 is set, clears it and sets bit one, then returns a 32-bit pattern formed from the value with a fixed mask OR-ed in and its two halves swapped.

### diskfont.library

- `branch_to_font_handler`: Branches to another routine via a table entry at an offset from a library base, then if the result is non-zero, stores the contents of a register into a structure at an offset of 34 and calls another routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: Library code that attempts to open two libraries, and if successful, initializes a structure at offset 42 of the input pointer with a new pointer and sets offset 50 to zero.
- `manage_font_library_state_2`: As library code, it iterates through a linked list starting at offset 42 of a structure in a6; it performs memory operations and calls another routine if a value at offset 21 of a list element is less than one, and if certain conditions in the structure in a6 are met, it calls another routine twice and returns a value from offset 34.
- `manage_font_library_state`: Library code that checks two conditions in a structure in a6; if both are met, it calls two functions, retrieves a value from offset 34, calls another function, and updates a linked list, otherwise it sets bit 7 of a byte at offset 14 of the structure in a6 and returns 0.
- `set_font_status_bit`: Library code that sets bit 7 of a byte at offset 14 of the structure in a6.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `check_font_and_jump`: Library code that decrements a word at offset 38 of the structure in a6, and if it is not zero and bit 7 of a byte at offset 14 of the structure in a6 is set, jumps to an offset relative to a6.
- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `serialize_font_list_to_buffer`: Iterates through a linked list of font structures, calculates the length of each string, and copies the string data into a destination buffer while populating an array of metadata structures with string lengths and character data.
- `concatenate_font_string_list`: Iterates through a linked list of string structures, calculating the length of each string and copying its contents into a destination buffer while updating a length counter and a structure at a specific offset.
- `copy_string_to_buffer_4`: Copies a sequence of bytes from a source pointer to a destination buffer and updates a length counter.
- `parse_font_directory_entries`: Iterates through a directory structure to locate font files, calculates string lengths, and copies character glyph data into a memory buffer.
- `copy_font_glyph_data`: Calculates a destination address by subtracting a value from a pointer, then copies a sequence of bytes from a source buffer to that destination using a loop.
- `parse_font_glyph_table`: Reads font data from a disk file and populates a structured table with glyph information, including character codes and offsets.
- `read_and_parse_font_data`: Reads data from a disk file and parses it into a structured format, copying string data into a buffer while updating length counters and structure offsets.
- `process_directory_and_memory`: Library code that calls three different routines and returns the absolute value of a 16-bit register if it is negative, otherwise it returns zero.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `calculate_font_index`: As library code, it calculates a signed long value by comparing words at offsets 256 and 258 of two structures, then performs a loop that iterates seven times to subtract values from a global table based on bitwise comparisons of bytes at offsets 258, 259, 259, and 260 against bytes at offsets 6 and 7 of the first structure, and calls a routine to traverse a bit tree and another to load a constant long.
- `traverse_font_bit_tree_2`: As library code, it performs a bitwise traversal of a tree structure by checking bits in a register against two bytes at offsets 258 and 259 and subtracts values from a 32-bit integer using a table.
- `traverse_font_bit_tree`: As library code, it performs a bitwise traversal of a tree structure by checking bits in a register against two bytes at offsets 258 and 259 of a structure in a1, updating a value by subtracting elements from a table indexed by a shifting and incrementing counter up to six times before returning.
- `fetch_font_glyph_offset`: Calculates an offset into a table by shifting a value in d6 and subtracting a value from a table at a fixed address, then updates d3 and d6 before returning the calculated offset in d0 or branching to another routine.
- `return_constant_long`: Library code that loads the constant a fixed address into D0 and returns.
- `update_font_entry_metadata`: Iterates through a table of font entries to find the entry with the maximum value at a specific offset, then updates a pointer, increments a counter in a global variable, and modifies flags and values within the selected font structure.
- `search_and_update_font_entry`: Iterates through a table of font entries to find the entry with the maximum value at a specific offset, updates a pointer and a counter in a structure, and modifies specific fields within the selected font entry if certain conditions are met.
- `initialize_font_registers`: Clears a 16-bit word at an offset of 22 from a6, loads 255 into d3, clears d7, moves d7 into a2, and branches to another routine.
- `find_max_font_index`: Iterates through a table of font entries to find the entry with the maximum value at a specific offset, updating a pointer and a counter in a structure, then returns.
- `calculate_and_clear_offset`: As library code, it calls another routine, calculates a new address by multiplying a word in d2 by 259 and adding d5 to it, clears a word at offset 256 of that address, and branches to another routine.
- `search_font_index`: Iterates through a table of font entries to find the entry with the maximum value at a specific offset, updating a pointer and a counter in a structure based on the result.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine_2`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next_2`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg_2`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg_2`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `copy_string_loop`: Library code that pushes a pointer to the stack, loads a source address from the stack, and loops through the source bytes, incrementing the pointer until a null byte is found.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `test_and_increment_byte`: Loads a pointer from the stack into a0, tests the byte at that location, and if the byte is not zero, increments d0 and branches to another instruction; otherwise it continues.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `compare_bytes_and_set_zero`: As library code, it compares a byte from a pointer in a0 with a byte from a pointer in a1, and if the byte in a0 is zero, it sets d0 to zero and returns.
- `compare_byte_and_set_d0`: As library code, it compares a byte from a pointer in a0 with a byte in d0, and if the byte in a0 is less than or equal to d0, it sets d0 to 255 and branches to another routine; otherwise it returns.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `check_byte_equality`: As library code, it loads a pointer from the stack, compares a byte at that pointer with a byte in d0, and if they are equal, it loads the pointer into d0 and branches to another routine.
- `increment_pointer_and_branch`: Increments the pointer in a0 by 1 and branches to another routine.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `search_backward_for_byte`: As library code, it searches backward from a pointer in a0 for a specific byte value provided in param_2, returning the pointer to the found byte or zero if the search reaches the limit defined by param_1.
- `decrement_pointer_and_branch`: Decrements the pointer in a0 by 1 and branches to another routine.
- `clear_d0_and_restore_registers_4`: Sets d0 to zero and restores d2 and d3 from the stack before returning.
- `search_backward_for_characters`: As library code, it searches backward from a pointer in a0 for the byte value 47, and if not found, searches for the byte value 58, otherwise it returns the original pointer.
- `move_d3_to_d0_and_restore`: Moves the value from d3 into d0, restores d2 and d3 from the stack, and then returns.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### mathieeedoubtrans.library

- `triggers_illegal_instruction_exception`: Executes an illegal instruction to trigger a processor exception.
- `clear_d0_and_return_2`: Clears d0 and returns immediately.
- `update_math_structure_and_store`: As library code, it stores a pointer in a global variable, calls another routine, and if the return value in d0 is not zero, stores that value in a global variable.
- `update_math_structure_2`: As library code, it calls another routine to retrieve a value, updates several fields in a structure including a pointer, a byte, and several longwords, and calls another routine before returning.
- `initialize_structure_from_function`: Calls a function and stores the result in a global variable, then writes the address of this routine to offsets 52 and 56 of the structure pointed to by the returned value.
- `raise_exception_for_code_2`: Masks the low word of d7 and compares it against two specific codes.
- `compare_threshold_and_branch`: As library code, it compares the word in d7 to 7680 and branches if it is greater than or equal to that value, otherwise it returns.
- `compare_d7_threshold_2`: Compares the low word of d7 to 7680 and branches to a different location based on the result.
- `compare_d7_threshold`: As library code, it compares the low word of d7 to 23040 and branches to another routine if it is greater than or equal, otherwise it returns.
- `compare_and_branch`: Compares the low word of d7 against 22400 and branches to another routine if it is greater than or equal, otherwise it branches to another routine.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `process_word_data`: As library code, it writes a 32-bit value formed by two 16-bit registers to a destination pointed to by a structure offset in a6, then enters a loop that checks words at a pointer in a0 and calls another routine if certain conditions are met.
- `process_word_loop`: As library code, it enters a loop that checks words at the pointer in a0, calls another routine if certain conditions are met, and eventually writes two 32-bit values from d0 and d1 to the location in a1.
- `check_exception_and_pop`: Compares d7 against 34592 and 50944, and if neither matches, copies the contents of a1 into d0 and d1, checks if the word at a0 is negative, and if not, pops a value from the stack into d7 before returning.
- `update_word_sequence_buffer`: Updates a buffer at a pointer derived from a structure in a6 by writing two 32-bit values and then enters a loop that writes a final 32-bit value and returns a 32-bit result based on conditions checked against a word in the buffer.
- `store_word_sequence`: As library code, it enters a loop checking words at a pointer in a0 and calls another routine if specific conditions are met, then stores two 16-bit parameters into a destination pointed to by a1, followed by storing two other 16-bit parameters into the same destination after a loop that checks words at a0.
- `fetch_and_store_stack`: As library code, it enters a loop checking words at the pointer in a0, calls another routine if specific conditions are met, and eventually writes two 32-bit values from the stack to the location in a1.
- `validate_and_fetch_pointer_2`: Compares a 16-bit value in register d7 against two specific constants and branches to another routine if they match; otherwise, it loads two longwords from the address in a1 into d0 and d1, checks if the word at the address in a0 is negative, and if not, loads a longword from the address in a7 into d7 before returning.
- `return_zero_3`: Clears the d0 register and returns immediately.
- `compute_double_precision_transcendental`: Performs a series of mathematical operations using a sequence of constants and function calls to compute a double-precision floating-point value.
- `validate_and_zero_structure_fields`: Pushes two longwords onto the stack, loads two consecutive longwords from an offset of eight from a5 into d0 and d1, and calls another routine; if the call results in a negative flag, it stores zero into two different longword fields at offsets of negative 204 and negative 208 from a5, otherwise it branches to another routine.
- `push_constants_and_call_routine`: Pushes two longwords onto the stack, loads two consecutive longwords from a memory location offset by eight from a5 into registers d0 and d1, calls another routine, and if the result is less than zero, writes two longwords to specific offsets within a structure in a5; otherwise, it branches to another routine.
- `perform_double_precision_arithmetic`: Compares the sign bit of a value in a structure with a boolean flag, then performs a series of floating-point operations including exponent and mantissa updates, constant loading, and multiple writes to a structure at various offsets.
- `update_exponent_and_mantissa`: As library code, it calculates an exponent value based on a word in a structure, then either clears two longwords, updates the mantissa and exponent in the structure, or sets two specific longwords depending on the calculated value.
- `compare_long_and_branch`: Compares d2 against 1024 and branches to another routine if it is less than or equal, otherwise it branches to another routine.
- `set_double_exponent_field`: Replaces the exponent field of a double-precision value with one derived from the argument and returns the result, leaving the mantissa unchanged.
- `calculate_logarithm_double`: As library code, it checks if a parameter is zero or if its sign matches a specific condition, then performs a series of double-precision mathematical operations using a polynomial evaluation routine and several library calls.
- `evaluate_logarithm_double_full`: Reduces a double-precision argument against fixed bounds and applies a long polynomial with fixed coefficients, calling unnamed routines at each step and returning the double result.
- `evaluate_double_precision_equality_2`: Compares two double-precision floating-point values by checking their signs and exponents, and performs a series of conditional checks to determine if they are equal.
- `evaluate_double_precision_equality`: Compares two double-precision floating-point values by checking their signs and exponents, then performs a series of conditional checks and mathematical operations to determine if they are equal or to handle special cases like zero or infinity.
- `branch_to_offset_2`: Performs a short branch to an instruction located at a fixed offset within the same library.
- `process_double_precision_exponent`: Compares the sign and value of a 32-bit integer at an offset within a structure, then branches to perform various mathematical operations involving double-precision values.
- `evaluate_math_chain_branching`: Evaluates a mathematical expression by branching into different logic paths based on a returned integer value.
- `execute_math_chain_2`: As library code, it loops a specified number of times, copying values from a structure into a stack-based buffer, calling a routine to process them, and finally updating a structure with the results.
- `execute_math_chain`: As library code, it performs a sequence of calls to a handler via a pointer, storing intermediate double-precision results into a structure at offsets -48, -8, and -56, and updates several 32-bit values within that structure before returning a final 64-bit value from offset -104.
- `prepare_arguments_and_call`: Clears two 32-bit values on the stack, loads a pointer from an offset of a5 into a0, moves two 32-bit values from the location pointed to by a0 into d0 and d1, calls another routine, and branches to another routine if the result is not greater than zero.
- `forward_math_call_four_args`: Forwards four arguments to a handler reached through a pointer and returns its double-precision result.
- `return_zero_stub_2`: Clears the 32-bit result register d0 and returns, so the routine always yields zero.
- `chain_three_math_calls`: As library code, it passes its two arguments to one unnamed routine, then feeds each result into the next unnamed routine in turn, returning the final double-precision value.
- `update_double_precision_value`: Calculates a value based on the contents of a pointer, and if that value is less than -1021, clears four 16-bit words at the pointer location; otherwise, it modifies the first 16-bit word at the pointer using a bitwise mask and a shifted version of the calculated value.
- `rebias_double_exponent_field`: Replaces the exponent field of the double-precision value at a pointer with a biased value from a register, leaving the sign and mantissa bits in place.
- `dispatch_via_table_offset`: Calls a routine found at an offset from the stored library base pointer, with the offset supplied in a register.
- `load_library_pointer_offset_4e`: As library code, it loads a pointer from a global variable into a0 and branches to another routine.
- `dispatch_library_function_2`: Loads a constant value into a register and branches to another routine.
- `load_library_base_and_branch`: Loads the base address of a library into a register and branches to another routine at a fixed offset from that base address.
- `dispatch_library_function_3`: Loads a constant address into a register and branches to another routine.
- `call_library_slot_e`: Calls a further library function at another fixed offset from the stored library base pointer, with no other work.
- `dispatch_via_register_offset`: Calls a routine located at an offset from the stored base pointer, with the offset taken from a register.
- `jump_to_offset_routine`: Loads a constant address into a register and performs a short branch to another routine located at a fixed offset from that address.
- `jump_to_offset_routine_2`: Loads the address of a global variable into a register and performs a short branch to another routine.
- `call_indirect_library_function`: As library code, it calls a function through a pointer located at a fixed offset from a stored library base pointer.
- `call_library_function`: This library code performs an indirect call to a function pointer retrieved from a table using an offset.

### sound1

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.

### sound2

- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### sound3

- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.

### sound4

- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.

### sound7

- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.

### sound8

- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.

### sound9

- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.

### extra

- `return_immediately`: The routine does nothing and returns immediately.

### Shanghai (1986)(Activision)[m Headbanger][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_20`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_external_function`: Calls another routine and returns its result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a6, then returns.
- `call_library_vector_348_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_task_data`: Calls a routine to find a task, then performs conditional data manipulation involving pointer arithmetic and library calls based on the task's properties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_task_structure`: Calls a routine and, if the result is non-zero, writes a value to an offset of 34 in the returned structure and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_field`: Calculates a value based on a pointer within a structure, stores it in a global variable, calls a routine via a pointer in a6, and if the result is non-zero, stores a 16-bit value at an offset of 34 in the returned structure before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callback`: Calls a function through a pointer and, if the result is non-zero, stores a value into a structure and executes another function.
- `save_registers_to_globals`: Saves the current stack pointer to a global variable, moves a value from d0 to a0, saves the current a6 to a global variable, restores a6 from d0, saves d0 to a global variable, and loads a relative address into a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_stack_data`: Calculates a pointer and a global variable from a structure, clears a 16-bit value and a 32-bit value on the stack using an offset derived from a byte at that pointer, calls a routine via a pointer in a6, and if the result is non-zero, stores the 32-bit stack value at an offset of 34 in the returned structure and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_and_store_stack_pointer`: Calls a routine using an offset from a6 with d0 set to 56, then if the return value is non-zero, stores the value from the stack at offset 34 of the returned structure and calls another routine using an offset from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_and_call_routine`: Derives a pointer and a global variable from a structure, clears a 16-bit value and a 32-bit value on the stack using an offset derived from a byte at that pointer, calls a routine via a pointer in a6, and if the result is non-zero, stores the 32-bit stack value at an offset of 34 in the returned structure and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callback_2`: Calls a routine using an offset from a6, restores registers from the stack, and if the return value in d0 is non-zero, stores a value from the stack at an offset of 34 in the returned structure and calls another routine via an offset from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_stack_value`: Calculates a pointer and a global variable from a structure, determines a 32-bit value based on a byte at that pointer, and clears a 16-bit value on the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_function`: If a register is non-zero, stores a value from the stack into a table offset and calls a function via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_backwards`: Copies a sequence of bytes from a source address to a destination address in reverse order, then writes a space character to the destination address at an offset determined by the length of the copied data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `copy_string_to_buffer_3`: Calls a function via a pointer in a register, then copies a string of bytes from a source address to a destination buffer, appending a space character at the end of the sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_decrement_global_buffer`: Copies a block of bytes from a global buffer to a local stack area, appends a space character, and then performs a similar copy-and-append operation after decrementing the global buffer pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_linked_list_and_update_structure`: Follows a chain of pointers through a structure, calling a routine at a specific offset from each pointer; if two consecutive calls succeed, it updates a pointer and several fields within the original structure, otherwise it calls a cleanup routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_copy_string`: Decrements a global variable and uses its value to copy a sequence of bytes from a memory location into a stack-based buffer, appending a space character at the end.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_copy_linked_structures`: Follows a pointer within a global variable to access a structure, then performs a series of loops to copy data from memory locations derived from that structure into a stack-based buffer before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_process_string_data`: Copies a sequence of bytes from a source address to a destination on the stack, appends a space character, and then performs a series of pointer dereferences and conditional loops to manipulate data structures and call another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_linked_list_and_copy_data`: Traverses a linked list of structures via pointers, performs a loop to copy bytes from a source buffer to a stack-based destination, and calls another routine before continuing to process subsequent list elements.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_call_structure_method`: Copies a block of memory from a source address to a destination, calls a method via a pointer in a global variable, and then performs a conditional memory copy based on the result of that call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_stack_data_via_pointer`: Calls a function via a pointer stored in a global variable, then performs a series of loops to copy bytes from a memory location to the stack, ending with a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_and_call_subroutine`: Copies a sequence of bytes from a source table to a destination stack area using a loop, then calls another routine and branches to a fixed location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_pointer_and_copy_stack`: Calls a function pointer from a global variable, then either updates a structure at offset 2a of a6 with a pointer and increments it, or copies bytes from a0 and a1 to the stack and calls a routine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_and_call_subroutine`: Copies a sequence of bytes from a source table to a destination stack area using a loop, then calls another routine and branches to a fixed location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_or_copy_stack`: Calls a function pointer from a global variable, then either updates a structure at offset 2a of a6 with a pointer and increments it, or copies bytes from a1 to the stack and calls a routine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_call_stack_data_2`: Copies bytes from a source in a1 to the stack using d2 as a counter, then calls a routine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_or_copy_stack_2`: Calls a function pointer from a structure in a6, then either updates a structure at offset 2a of a6 with a pointer and increments it, or copies bytes from a1 to the stack and calls a routine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_and_call_subroutine_2`: Copies a sequence of bytes from a source at a1 to a destination at a7 using a count in d2, then pushes the destination pointer onto the stack and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_or_copy_stack_3`: Checks if a value in d0 is zero, then either copies bytes from a1 to the stack and calls a routine at a fixed address, or updates a structure at offset 2a of a6 with a pointer and increments it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_via_stack`: Copies the current stack pointer to a1, pushes a1 onto the stack, and calls a routine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_structure`: If a zero flag is set, it calls two function pointers from a global variable and another from a pointer at a negative offset of a6; otherwise, it stores the address of a calculated pointer into a structure at offset 42 of a6, sets the value at offset 46 to the pointer itself, and sets the value at offset 46 of a6 to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_indirect_function_pointers`: Calls a routine, then if a global variable is non-zero, it calls several function pointers located at various offsets from a register and from the global variable itself before returning a concatenated value.
- `invoke_two_function_pointers_3`: Calls two function pointers at fixed negative offsets from a base register, then returns.
- `update_global_from_library_call`: Loads a pointer from a global variable into a1, calls a routine via an offset from a6, stores the result in a global variable, and returns if the result is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `jump_to_disk_buffer_command`: Performs an indirect jump to a routine located at a constant address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unlk_and_return`: Unlocks the stack pointer in a6 and returns.
- `modify_memory_and_call_routines`: If a global variable is less than a stack address, it writes values to a global variable, calls a routine, writes values to a global variable, calls another routine, and performs a bitwise OR on a byte at an offset calculated from a0 and d4; otherwise, it clears a global variable, calls three different routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_intuition_request`: Loads a pointer from a global variable into a7, calls a routine, pushes the constant 20 onto the stack, calls another routine, performs a bitwise OR on d0 with 212, and stores the result at offset 86 of a structure pointed to by a0.
- `init_intuition_request`: Opens the intuition library, stores the handle in a global variable, and performs an auto request with fixed dimensions of 250 and 40.
- `call_library_vector_294_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_408_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_indirect_pointer`: Calls a function pointer located at a negative offset from a global variable.
- `increment_and_mask_byte`: Clears bit 7 of a byte at offset 14 of a structure in a6, increments a 16-bit word at offset 38 of the same structure, and calls a function via a pointer located at a negative offset from a global variable.
