# Game 936 (Beastlord v1.01 2452): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00936). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3351477 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 44 of 44 units identical (bytes and relocations), 52 raw-word lines |
| Functions named and described | pass, 759 of 759 named (0 author symbols), 759 of 759 described (11 by rule, 677 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7257 of 7257 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14177 of 14691 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 334019 |
| code | 54698 |
| data | 2958080 |
| padding | 72 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 378 | 12 |
| logged disk load from Disk.1 | 1 | 12858 | 668 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 107 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 813 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 6 |

## Cases handled

- Beastlord_v1.01_2452/Beastlord_v1.01_2452_hdf/Beastlord/Disk.1: raw disk image (810160 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Beastlord_v1.01_2452/Beastlord_v1.01_2452_hdf/Beastlord/Disk.2: raw disk image (673056 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Beastlord_v1.01_2452/Beastlord_v1.01_2452.hdf: hard-disk image (AmigaDOS OFS, 3584 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree Beastlord_v1.01_2452_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 106 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 427 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk03)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk03) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- Beastlord_v1.01_2452/Beastlord_v1.01_2452_hdf/Beastlord/Disk.1: 3 logged loads cover 386796 bytes of the image
- Beastlord_v1.01_2452/Beastlord_v1.01_2452_hdf/Beastlord/Disk.2: chunks 0-7 (bytes 0-2048) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.02)
- Beastlord_v1.01_2452/Beastlord_v1.01_2452_hdf/Beastlord/Disk.2: chunks 19-920 (bytes 4864-235776) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.98)
- 3 packed track region(s), 858576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_bit_interleaved_data` (Disk.1 +an address loaded at an address (194168 bytes)): Performs bitwise interleaving on data from a source pointer and stores the result in a destination table, potentially verifying the data against a checksum. Kind: bit-interleaved data checksum/validation. Pass path: the branch where uVar4 == *(uint *)(unaff_A6 + 4) and the subsequent XOR checksum loop matches, returning in_D1.
- `update_global_via_offset_index` (Disk.1 +an address loaded at an address (194168 bytes)): Updates a global variable by using a value from a table, where the table index is retrieved from an offset of a pointer held in a register. Kind: disk hardware/track validation. Pass path: the branch that continues normal execution (likely the 'beq' or 'bmi' logic depending on the specific status flag set by the hardware check).
- `dereference_indexed_pointer` (Disk.1 +an address loaded at an address (194168 bytes)): Loads a value from a table using an index found at an offset from a pointer, then calls another routine. Kind: indexed pointer dereference (likely a table lookup for disk geometry or sector data). Pass path: the instruction immediately following the call to u1__sub_1F5A8().
- `set_global_byte` (Disk.1 +an address loaded at an address (194168 bytes)): Assigns a specific constant byte value to a global variable. Kind: checksum or state-based validation trap. Pass path: the branch where d0 is set to an address (success) vs an address (failure).
- `increment_indexed_value` (Disk.1 +an address loaded at an address (194168 bytes)): Modifies a global variable by setting specific bits and then increments a value at a calculated memory location. Kind: checksum or integrity trap. Pass path: the loop continues (bra.b an address) if the condition is not met, implying the incremented value is being compared against a target or used to verify a sequence/checksum.
- `decrement_indexed_value` (Disk.1 +an address loaded at an address (194168 bytes)): Modifies a global variable by setting specific bits and then decrements a value at a calculated memory location. Kind: checksum or integrity check. Pass path: the instruction immediately following the call (the code continues execution normally).
- `read_bitmask_from_register` (Disk.1 +an address loaded at an address (194168 bytes)): Iterates through a loop to build a bitmask based on the state of a hardware register bit. Kind: hardware register bitmask check. Pass path: uVar1 bitmask matches expected value (loop completes with specific bits set based on hardware state).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_memory_and_setup_interrupt`: Clears a large block of memory with zeros and sets an interrupt handler before performing conditional logic and updating a global variable.
- `store_word_to_memory`: Stores a specific word into a memory location determined by two registers.
- `execute_indirect_call_2`: Calls a function through a pointer located within a structure.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `set_interrupt_handler`: The routine writes a pointer to another routine into a global variable.
- `update_global_state`: Performs bitwise operations on a global variable and updates it based on specific bit patterns and hardware register status.
- `decompress_bitstream_data`: Parses a custom compressed data stream by processing bits through a series of shifts and carries.
- `process_byte_stream`: Processes a sequence of bytes by shifting them left and handling carries through a pointer.
- `copy_memory_block`: Copies a block of memory from one location to another using a loop that handles 32-bit words and 16-bit words.
- `execute_callback_and_clear`: Calls a function pointer from a table and, if the result is zero, calls another function with a specific constant and enters a loop to decrement a value.

### logged disk load from Disk.1

- `dispatch_state_machine_sequence`: Executes a sequence of subroutines by updating a global control variable with specific values before each call.
- `execute_state_machine_sequence`: Manages a complex state machine by updating a global control variable and executing a sequence of subroutines with varying parameters.
- `initialize_executable_hunk_data`: Initializes a structure by clearing various memory areas, setting specific flags, and copying data from a source table.
- `copy_and_update_tables`: Performs multiple block memory copies between different tables and updates several memory locations while polling a global flag.
- `update_coordinate_table`: Calculates and writes multiple coordinate values into a table at specific offsets within a loop, while waiting for a global flag to clear.
- `process_hunk_data`: Checks a byte at an offset and enters a loop that modifies specific bytes in a structure based on bitwise checks and comparisons with AmigaDOS hunk type codes.
- `reset_structure_flag`: Sets a specific word in a data structure to zero and verifies the write.
- `check_offset_value`: Checks if a value at a specific offset in a structure is zero and, if so, executes a series of other routines.
- `clear_memory_buffers`: Sets several memory locations to specific values and then clears three large blocks of memory using a loop.
- `configure_blitter_parameters`: Calculates various offsets and dimensions based on input values and writes them to custom memory locations and hardware registers for a blitter operation.
- `setup_blitter_transfer`: Writes specific values to custom memory locations and configures hardware registers to prepare for a blitter transfer.
- `prepare_blitter_draw`: Calculates source and destination parameters and writes them to custom memory and hardware registers to set up a blitter operation.
- `iterate_pointer_table`: Iterates through a table of pointers, calling another routine for each valid entry found.
- `calculate_blitter_dimensions`: Determines blitter width, height, and position based on a comparison between an input value and a calculated offset, then writes these to custom memory and hardware registers.
- `update_structure_pointers`: Copies a word from one offset in a structure to another and stores a function pointer at a third offset.
- `configure_blitter_transfer`: Sets up blitter hardware registers and performs a series of memory operations based on values retrieved from a table.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_function`: Executes a function via a pointer stored at an offset within a structure.
- `update_blitter_parameters_2`: Checks a value in a structure against a stored copy and, if changed, performs a series of writes to hardware registers and memory locations.
- `parse_sprite_command_stream`: Iterates through a data stream to extract command parameters and uses them to calculate coordinate offsets and bitmask values.
- `configure_blt_graphics_parameters`: Calculates and writes values to the Amiga Blitter registers and associated memory locations based on input parameters and table lookups.
- `update_structure_indices`: Increments or decrements a value within a structure and calculates several memory addresses and offsets based on that value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_nested_counters`: Decrements a short value and performs complex multi-level increments and resets on several bytes and words within a structure based on specific threshold values.
- `initialize_hardware_parameters`: Sets values in specific memory locations, performs arithmetic on a global variable, and clears a series of double-words in a structure based on a bit test.
- `copy_pattern_data`: Calculates an offset into a table and copies several repeating word values from that table to a destination buffer.
- `decrement_conditional_counter`: Decrements a word at a specific offset if a condition is met, wrapping the value to nine if it becomes negative.
- `update_state_timer_logic`: Manages a state machine and countdown timer by comparing several short integer values against specific thresholds.
- `execute_empty_recursion`: The routine performs three consecutive recursive calls to itself before returning.
- `copy_interleaved_table_data`: Either clears a memory block or iterates through a table to copy specific word pairs from a source to a destination.
- `search_and_copy_table_data`: Iterates through a table using two input values to determine an offset, then copies a series of word pairs to a destination.
- `swap_structure_words`: Calls another routine and then swaps the contents of two 32-bit values located at specific offsets within a structure.
- `process_entity_state_transitions`: Updates an entity's state and attributes based on a control byte, modifying status flags, position coordinates, and internal counters.
- `copy_data_to_structure`: Copies several words and bytes from one memory location to another within a structure and updates a global pointer.
- `update_entity_state_attributes`: Copies a sequence of three words from a source pointer into a structure and redistributes these values to specific offsets within that same structure.
- `resolve_hunk_addresses`: Calculates and stores several memory addresses into a structure based on values retrieved from a table.
- `sync_entity_data_tables`: Iterates through a list of entity identifiers to find matching entries in multiple lookup tables.
- `update_calculated_table`: Iterates through a list of values to calculate new entries for a table based on data from multiple other memory locations.
- `initialize_system_parameters`: Sets several global variables to specific values and initializes a word at a table offset to all ones.
- `initialize_hardware_and_state`: Sets various global variables, performs bitwise shifts to populate a memory buffer, and interacts with hardware registers including CIA and DMA controllers.
- `store_clamped_values`: Stores three values into a table at specific offsets after they have been processed by another routine.
- `increment_and_clamp_value`: Adds a value to a memory location after clearing its least significant bit and stores the result if it remains below a specific threshold, otherwise enters an infinite loop.
- `validate_data_signature`: Checks a memory location for a specific signature and stores a calculated value into a structure offset.
- `set_start_pointer`: Stores the address of a specific label into a global variable.
- `increment_bounded_value_with_limit`: Increments a value at a specific offset by an input amount, ensuring the result is even and does not exceed a limit defined by another variable.
- `validate_data_signature_alt`: Verifies a magic signature in memory and writes a modified value to a different structure offset than another routine.
- `copy_long_word`: Copies a long word from one memory location to another.
- `traverse_linked_list`: Traverses a linked structure by following pointers and updating a destination address based on byte values.
- `decompress_compressed_data_stream`: Identifies a specific magic signature in memory to locate a data block and then performs bitwise decompression.
- `process_bit_stream`: Performs a bitwise shift and carry operation on a byte while decrementing a counter and traversing a memory pointer.
- `copy_memory_block_2`: Copies a block of memory from a source to a destination using a loop that handles both long words and words.
- `clear_memory_block_3`: Fills a memory area pointed to by a value in a table with zeros for a fixed number of iterations.
- `clear_memory_array`: Fills a large memory area pointed to by a global variable with zeros.
- `initialize_global_table`: Clears multiple sections of a global table using various loop increments and sizes.
- `loop_decrement_nibbles`: Executes a series of nested loops that call another routine multiple times and waits for a specific byte to reach zero.
- `decrement_nibbles`: Decrements specific nibbles within a word at a pointer location if they are non-zero.
- `execute_nested_loops`: Executes a series of nested loops that repeatedly call another routine a specific number of times.
- `increment_nibbles`: Compares nibbles of two words and increments the nibbles in the destination word if they do not match the source.
- `update_conditional_byte`: Decrements a byte at a specific offset and copies a byte from one location to another based on the state of two other bytes.
- `sort_memory_array`: Performs a comparison-based sort on an array of words using bitwise masks to determine order.
- `reset_global_state_and_interrupts`: Clears a global variable, sets another global variable to a constant value, and interacts with the interrupt request register.
- `reset_keyboard_state`: Interacts with CIA hardware registers to clear a global status variable.
- `clamp_byte_value`: Reads a byte from a memory location and returns it, ensuring the value is not negative.
- `update_indexed_memory_byte`: Calculates an index by performing bitwise operations on a global variable and uses it to retrieve a value from a table.
- `update_coordinate_limits`: Updates two separate sets of coordinate values by adding the difference between a global variable and a previous value, while clamping the results within specified minimum and maximum bounds.
- `modify_button_states`: Extracts specific bits from a global variable to update a byte in a table and modifies another byte in a table using a bitwise negation.
- `wait_for_zero`: Enters an infinite loop while a global variable is non-zero, eventually entering a permanent stall.
- `transform_uint_value`: Multiplies a value from a table by two and performs a bitwise XOR with a constant if the result is zero or an overflow occurs.
- `copy_and_increment_data`: Copies a block of memory from a global variable to a destination, then increments a value in a structure.
- `tiny_addq_moveq_rts`: Small routine of 3 instructions that adds 8 to a7, then loads 255 into d0, then returns.
- `manage_hardware_state_and_data_buffer`: Configures hardware registers and updates a data structure based on an index value.
- `process_bit_interleaved_data`: Performs bitwise interleaving on data from a source pointer and stores the result in a destination table, potentially verifying the data against a checksum.
- `update_global_via_offset_index`: Updates a global variable by using a value from a table, where the table index is retrieved from an offset of a pointer held in a register.
- `dereference_indexed_pointer`: Loads a value from a table using an index found at an offset from a pointer, then calls another routine.
- `set_global_byte`: Assigns a specific constant byte value to a global variable.
- `increment_indexed_value`: Modifies a global variable by setting specific bits and then increments a value at a calculated memory location.
- `decrement_indexed_value`: Modifies a global variable by setting specific bits and then decrements a value at a calculated memory location.
- `compare_and_loop_values`: Compares a value from another routine against a value retrieved from a table using an indexed pointer.
- `reset_indexed_value`: Checks a bit in a global variable and repeatedly calls another routine while a value is non-zero.
- `poll_interrupt_flag`: Enters a loop that waits for a specific bit in a hardware register to be set.
- `wait_for_interrupt_flag`: Enters a loop that repeatedly checks a specific bit in a hardware register until it is set.
- `poll_status_bit`: Enters a loop that waits for a specific bit in a hardware register to be set after loading a constant into a register.
- `poll_hardware_bit`: Enters a loop to wait for a specific bit in a hardware register to change state while decrementing a counter.
- `read_bitmask_from_register`: Iterates through a loop to build a bitmask based on the state of a hardware register bit.
- `return_error_code`: Returns a constant value of negative one.
- `store_pointer_address`: Checks if a value at a memory location matches a specific constant and then stores the address of a global variable into two separate memory locations.
- `update_blitter_parameters`: Calculates a destination address and various offsets based on a table and global values.
- `perform_blt_area_fill`: Uses the Blitter to fill a rectangular area of memory with a specific pattern or color.
- `update_blt_sprite_parameters`: Calculates and updates Blitter parameters, including source/destination addresses, dimensions, and color modes, based on a table lookup and coordinate calculations.
- `update_status_flags`: Checks multiple conditions against values in a table and updates a byte in a global structure with specific bitwise flags.
- `clear_and_link_list`: Clears a block of memory and iterates through a table to perform linked-list insertions based on specific bitwise criteria.
- `update_blt_sprite_parameters_2`: Calculates sprite or graphical object positioning and scaling data by reading from a source table and writing to a destination structure.
- `process_distance_and_callback`: Calculates an absolute difference, updates a table with the result, and conditionally executes a function pointer from a structure.
- `check_and_set_value`: Compares a value in a global structure to a constant and updates it if the condition is met.
- `update_table_index_and_copy_attributes`: Decrements a counter within a structure and iterates through a pointer list until a non-zero byte is found.
- `process_hunk_type`: Checks a byte against specific character codes and updates various global offsets based on the result.
- `fetch_table_byte`: Returns a byte from a table using an index provided in a register, concatenated with a bit from that index.
- `initialize_memory_tables`: Clears several memory areas with zeros, fills a small buffer with spaces, and calculates a pointer to be stored in global variables.
- `search_string_table`: Iterates through a table of strings to find a match and stores the resulting byte into a global structure.
- `match_and_assign_string_metadata`: Iterates through a table of strings to find a match for a target sequence.
- `search_and_store_string_2`: Searches a table for a matching string and stores a byte into one of three global locations based on a type flag.
- `search_and_store_string`: Searches through a structured table for a matching string and stores a byte into one of three possible global locations based on a type flag.
- `search_and_store_string_alt`: Iterates through a data table to find a string match and updates a specific global variable if a condition is met.
- `search_and_store_string_v3`: Performs a table search for a string and assigns a byte to a global variable depending on the associated type code.
- `reset_byte_value`: Sets a byte at a specific offset in a global structure to zero.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a1)+, then returns.
- `traverse_and_call_list_2`: Iterates through a list of structures, comparing bytes, and either calls a function or updates a global pointer based on certain conditions.
- `traverse_and_call_list`: Iterates through a list of structures, performing an indirect call if a certain byte equals one value, or updating a global pointer if a certain byte equals another value.
- `execute_indirect_call`: Performs an indirect call to a function pointer stored in a memory location.
- `update_value_on_match`: Checks if a value in a structure equals a specific constant and, if so, updates it with a value from a pointer.
- `branch_on_type_and_update`: Checks a type byte or a constant value in a structure, then either executes an indirect function or updates a value in a structure with a pointer, depending on the condition.
- `process_conditional_logic_2`: Updates a counter and performs conditional checks on a byte value to increment or decrement a short value in a structure.
- `update_entity_animation_and_timers`: Updates an entity's animation frame indices, state flags, and internal timers based on status bitmasks.
- `update_function_pointer_2`: Checks a value at an offset from a pointer and, if it equals negative one, stores the current routine's address into a table.
- `update_function_pointer_alt`: Writes the address of this routine into a specific memory location if a condition at a pointer offset is met.
- `update_function_pointer_v3`: Stores the address of this routine into a global pointer if a specific short value is found in a referenced structure.
- `update_function_pointer_v4`: Assigns the address of this routine to a pointer if a value at a specific offset within a structure is negative one.
- `clear_global_variable`: Performs a loop and sets a global variable to zero.
- `update_state_machine_a`: Increments a counter in a structure and performs conditional updates to offsets and function pointers within a loop based on specific byte comparisons.
- `execute_state_machine_step`: Increments a counter and performs conditional updates to structure offsets and function pointers within a loop.
- `update_function_pointer_a`: Checks a byte in a structure and, if it does not match a specific value, stores this routine's address into a pointer in that structure.
- `update_state_machine_b`: Increments a counter in a structure and performs conditional updates to offsets and function pointers within a loop using different constant values than another routine.
- `execute_state_machine_step_alt`: Updates structure values and manages function pointer transitions based on a counter and specific byte comparisons.
- `update_function_pointer_b`: Checks a byte in a structure and, if it does not match a different specific value, stores this routine's address into a pointer in that structure.
- `update_state_with_conditional_logic`: Checks a short value and a bitmask to update integer values, increments a byte counter, and adjusts a coordinate based on a signed value and a bitmask.
- `update_position_and_timer`: Increments a byte counter, performs conditional updates to several structure members including a short value, and modifies a coordinate based on a signed value and a bitmask.
- `update_structure_counters`: Updates a counter and a structure offset based on a value, while conditionally resetting a byte and updating a function pointer.
- `update_counter_and_offset`: Updates a counter by adding a value and then modifies a structure offset by adding half of that counter's value.
- `update_recursive_counter_and_offset`: Increments a byte counter and performs a recursive update of a short value by adding a derived offset.
- `update_counter_and_offset_alt`: Updates a counter by adding a value and then modifies a structure offset by adding half of that counter's value, followed by a call to itself.
- `validate_and_initialize_structure`: Checks multiple conditions within a structure, including bitmasks and coordinate ranges, then optionally sets specific bytes, updates a function pointer, and calculates new integer values based on a base offset.
- `update_entity_state_and_counters_2`: Updates an entity's internal state, timer, and directional orientation based on bitmask flags.
- `update_entity_state_and_counters`: Updates an entity's internal state, timer, and directional orientation based on bitmask flags.
- `update_synchronized_animation_offsets`: Updates a set of five synchronized table offsets using specific increment values and modular wrap-around logic based on a conditional check of a control byte.
- `decrement_value_and_limit`: Sets a byte in a structure to a specific value, then decrements a short value in the structure while ensuring it does not fall below a minimum threshold.
- `increment_value_and_limit`: Sets a byte in a structure to a specific value, then increments a short value in the structure while ensuring it does not exceed a maximum threshold.
- `adjust_value_and_offset`: Calculates a new value based on a table entry and a comparison, then updates both that entry and another offset.
- `update_state_counter`: Increments a short value within a structure and resets several fields if a specific threshold is reached.
- `increment_and_reset_state`: Increments a word in a structure and, if it reaches a threshold, resets several structure values and updates two other values using a calculation.
- `initialize_structure_state_2`: Sets several values in a structure, including a function pointer and a word, and updates two other word values using a calculation based on a global value.
- `update_linked_list_node`: Checks a counter and a pointer to conditionally insert a value into a structure and update a function pointer.
- `update_pointer_on_condition`: Checks if a value pointed to by a pointer in a structure is equal to a specific constant, and if so, stores this routine's address into a pointer in that structure.
- `accumulate_and_increment_pointer`: Increments a pointer in a structure, adds the dereferenced byte from that pointer to a short value in the structure, and conditionally updates a function pointer.
- `increment_pointer_and_accumulate`: Increments a pointer within a structure and adds the value pointed to by that pointer to a short value in a structure.
- `update_state_and_recursively_increment`: Sets several values in a structure, updates two integer fields by adding specific offsets to a third field, and increments a byte.
- `increment_timer_and_update_state`: Increments a byte at an offset from a pointer and performs conditional recursive calls and flag updates based on specific values.
- `set_function_pointer_and_byte`: Sets a function pointer and a byte value in a structure if a specific byte is zero.
- `manage_state_machine_transitions`: Updates a state structure by modifying function pointers, status bytes, and counter values based on conditional checks.
- `update_counter_and_reset`: Compares a value from another routine against a structure member to increment a counter and adjust a threshold, or resets a byte and a function pointer if a condition is met.
- `reset_byte_and_function_pointer`: Sets a byte at a specific offset to zero and updates a function pointer in a table with the address of this routine.
- `initialize_state_and_decrement`: Sets a byte and a function pointer, initializes a byte and a word, and then decrements a word value until it reaches zero.
- `decrement_and_reset_counter`: Decrements a short value in a structure and, if it reaches zero, resets a byte and updates a function pointer.
- `update_limit_value`: Sets several values in a structure and, if a condition is met, updates a word value by comparing it against a calculated sum.
- `update_counter_and_pointer`: Checks a value in a structure and, if it meets a condition, updates a counter and a secondary value based on a comparison.
- `set_status_and_pointer`: Stores this routine's address into a pointer in a structure and sets a byte at a specific offset to a specific value.
- `nop_placeholder`: This routine is empty and performs no operations.
- `manage_entity_state_machine`: Updates an entity's state, function pointers, and timers based on conditional checks of a data structure.
- `update_dynamic_limit_and_reset_pointer`: Checks a condition via a pointer; if met, it increments a counter and updates a limit value based on a comparison between a returned value and the current limit.
- `decrement_and_reset_timer`: Decrements a short integer value stored at a specific offset from a pointer.
- `initialize_and_recursively_update_state`: This routine initializes several fields in a data structure via a pointer, updates two integer values based on an offset from a third value, and performs a recursive call if a specific condition is met.
- `update_callback_and_limit`: Checks a condition via a pointer and, if met, updates a limit value by comparing it against a calculated sum of a counter and an offset.
- `do_nothing_routine`: This routine is empty and performs no operations before returning.
- `update_animation_frame_counters`: Decrements a primary counter and a set of five secondary counters based on a modulo-based frame check.
- `decrement_timer_array_elements`: Decrements five consecutive word-sized elements in a structure by specific constant values and resets them to a new base value if they fall below their respective thresholds.
- `update_synchronized_table_offsets`: Updates a set of five word-sized table offsets using specific increment values and modular wrap-around logic based on a conditional check of a bitmask in a data table.
- `update_table_offsets`: Increments several consecutive table entries by specific amounts using modular wrap-around logic based on a condition.
- `search_word_list`: Iterates through a list of words, returning negative one if a negative value is encountered or the end of the list is reached based on a calculated limit.
- `update_entity_attributes`: Calculates various values based on structure offsets and a table, then updates several fields in a structure including status flags and pointers.
- `fetch_table_byte_2`: Calculates an offset based on several values in a structure and returns a byte from a table at that location.
- `fetch_table_byte_offset`: Computes a memory address using arithmetic on structure members and returns a byte from that address.
- `add_to_global_value`: Adds a value to a global variable and sets it to negative one if an overflow occurs.
- `increment_global_value`: Adds a value to a global variable and handles the carry by setting the result to negative one.
- `add_to_offset_value`: Performs an addition on a global variable and sets the result to negative one if the addition overflows.
- `subtract_from_global_value`: Subtracts a value from a global variable and clamps the result to zero if the subtraction would result in a negative value.
- `subtract_from_offset_value`: Decrements a global variable by a specified amount and ensures the result does not fall below zero.
- `decrement_global_offset`: Subtracts a value from a global variable and resets it to zero if the result is less than the subtrahend.
- `process_conditional_logic`: Performs a series of calls to another routine based on the value of a byte and a signed integer in a structure.
- `initialize_entity_structure`: Initializes a large data structure by zeroing out numerous fields and setting specific initial values.
- `process_state_and_list`: Updates status bytes based on memory comparisons, counts nodes in a linked list, and performs an indirect call.
- `update_status_flags_2`: Sets specific bytes to maximum values or zero based on bitmask checks and performs an indirect call from a table.
- `update_linked_list_status`: Checks a flag, retrieves a list length, and potentially updates a byte in a structure or inserts a node into a linked list.
- `update_list_state`: Updates a global variable, checks a list length, and potentially clears a byte in a structure or updates a byte in a structure if certain conditions or list lengths are met.
- `store_byte_at_offset`: stores a byte from a register into a memory location at an offset of seven from a pointer.
- `call_data_copy_routine`: This routine serves as a wrapper that calls a procedure to copy structured data from a source table to a destination table using a fixed stride.
- `return_to_caller`: This routine performs a single return instruction to the calling routine.
- `invoke_data_copy`: Calls a routine that copies a series of words from a source table to a destination table using a fixed stride.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `call_if_zero`: Checks if a word at a specific offset is zero, and if so, performs an indirect call using a pointer stored at another offset.
- `copy_structured_data`: copies a series of words from a source table to a destination table using a loop with a fixed stride.
- `calculate_and_store_offsets`: calculates several values based on a source table and stores them into a destination structure at specific offsets.
- `initialize_structure_values`: writes zero and specific constant values into several offsets of a structure pointed to by a register.
- `copy_short_values`: copies two word values from one location in a structure to another location within the same structure.
- `validate_and_reset_blt_buffer`: Performs arithmetic overflow checks on structure values to validate buffer dimensions and then resets several memory locations and hardware-related parameters.
- `update_dma_parameters`: Updates several hardware registers and memory locations based on arithmetic checks and waits for a flag in a global variable.
- `perform_dma_transfers`: waits for a flag in a global variable and then performs multiple DMA transfers to update a structure with calculated coordinates.
- `perform_incremental_dma_transfers`: waits for a flag in a global variable and performs a sequence of DMA transfers that incrementally update a structure's values.
- `update_counter_value`: Updates a value in a structure by adding a constant to a field and performs bounds checking and overflow handling.
- `update_structure_state_2`: Updates a short value in a structure and conditionally executes a function pointer based on a comparison and a flag.
- `initialize_structure_state`: Sets a byte to zero and stores a function pointer into a structure offset.
- `parse_command_string_3`: Iterates through a string to process various character codes, performing linked list traversals and conditional logic based on the characters found.
- `nop_instruction_placeholder`: The routine is empty and performs no operations.
- `calculate_subtraction_loop`: Performs a repeated subtraction loop to determine a difference between two values.
- `apply_bitmask_pattern`: Performs bitwise operations to blend a source pattern into a destination buffer using bit shifts and masks.
- `initialize_data_table`: Fills a large block of memory with repeating patterns of specific bit values through a loop.
- `setup_blt_parameters`: Calculates memory offsets and populates several registers and variables to prepare for a bitblock transfer operation.
- `update_blt_buffer_metadata`: Calculates a memory pointer and several offset values based on a linked list length and structure, then populates a series of memory locations with these values and specific constants.
- `execute_conditional_callback`: Compares a short value against a sum of two values from a table and, if the condition is met, calls a function pointer stored in a structure.
- `store_routine_address_3`: Calls another routine and then stores its own address into a structure at a specific offset.
- `assign_callback_pointer_2`: Stores the address of another routine into a memory location at a specific offset.
- `assign_callback_pointer`: Calls another routine and then stores that routine's address into a global variable.
- `extract_bits_to_buffer`: Reads bits from a table and writes them to multiple dispersed locations in a buffer.
- `initialize_and_scale_structure`: Initializes multiple fields of a structure, including function pointers, and performs a calculation involving bit-shifting and negation on specific numeric fields.
- `negate_and_scale_value`: Negates a value in a structure and calculates a new value based on a bit-shifted field and a constant.
- `configure_dma_parameters`: Calculates values for a global variable and configures hardware registers for DMA operations.
- `insert_into_linked_list`: Traverses a linked list to find the end and inserts a new node at that position.
- `swap_memory_words`: Swaps two words between two different memory locations.
- `insert_node_by_priority`: Traverses a linked list to find an insertion point based on a priority value, then updates pointers to insert a new node.
- `replace_list_node`: Traverses a linked list to find a specific node and replaces it with a new pointer provided in a register.
- `find_linked_list_index`: Traverses a linked list to find the index of a node containing a specific short value.
- `get_list_length`: Traverses a linked list to count the number of nodes until a specified count is reached or the end of the list is encountered.
- `search_linked_list_mask`: Iterates through a linked list to find a node that satisfies a bitmask condition against a table.
- `search_linked_list`: Iterates through a linked list to find a node that matches a specific bitmask condition.
- `search_list_with_mask`: Iterates through a linked list and stops when a specific bitmask condition is met or a limit is reached.
- `tiny_move_rts`: Small routine of 2 instructions that copies d2 to d0, then returns.
- `search_list_with_offset`: Traverses a linked list while checking a bitmask against values located at an offset from a base address.
- `count_specific_nodes`: Traverses a linked list and counts how many nodes contain a specific value in a certain offset.
- `remove_nodes_by_value`: Traverses a linked list to find and remove all nodes that contain a specific value, updating pointers to bypass them.
- `insert_if_not_present`: Searches for an index in a linked list and inserts a new node if the index is not found.
- `set_list_header`: Writes a constant byte and a pointer to a global variable into a structure at a specified offset.
- `update_coordinate_and_execute_callbacks`: Calculates and stores several derived values into a structure based on existing offsets and bit-shifted inputs.
- `execute_empty_callback`: This routine is empty and performs no operations.
- `copy_structure_fields`: Copies several fields from a source structure to a destination structure, or initializes a field to a constant if a flag is not set.
- `update_state_and_counter`: Checks a bitmask in a pointer, increments a counter, and updates a secondary value based on that counter.
- `update_linked_list_state`: Checks a flag in a structure and, if certain conditions or list lengths are met, updates pointers or inserts nodes into a linked list.
- `call_check_routine`: Executes a routine that calls a check and set value function.
- `manage_linked_list_nodes`: Checks a flag, retrieves a list length, and potentially updates a pointer to a global variable, checks a value, and inserts a node into a linked list.
- `trigger_dialogue_check`: Executes a routine that checks a condition and updates a value, associated with a specific string of dialogue text.
- `branch_on_structure_byte`: Checks a byte in a structure and either calls another routine or calls a routine to check and update a value.
- `invoke_value_update`: Calls a routine that checks and updates a value.
- `increment_counter_and_offset`: Increments a value in a structure and calculates a new offset or value based on the incremented result.
- `reset_entity_timer_and_state_4_2`: Resets an entity's timer and state, and updates a function pointer or secondary field based on certain conditions.
- `reset_entity_timer_and_state_4`: Resets an entity's timer and state, and updates a function pointer or secondary field based on certain conditions.
- `reset_entity_timer_and_state_3`: Resets a timer or counter field within an entity structure and updates a function pointer or secondary field based on certain conditions.
- `reset_entity_timer_and_state_2`: Resets a timer or counter field within an entity structure and updates a function pointer or secondary field based on certain conditions.
- `decrement_entity_timer_2`: Decrements a short integer at a specific offset within an entity structure and checks if the value has reached zero.
- `reset_entity_timer_and_function_4`: Resets a timer field and a function pointer within an entity structure, and decrements a timer or counter field while updating a function pointer or secondary field.
- `reset_entity_timer_and_function_3`: Resets or decrements a timer field and a function pointer within an entity structure.
- `reset_entity_timer_and_function_2`: Resets a timer field and a function pointer within an entity structure, or decrements a timer and updates a secondary field or function pointer based on certain conditions.
- `update_entity_timer_and_function`: Resets or decrements a timer field within an entity structure and updates its associated function pointer.
- `reset_entity_timer_and_state`: Resets an entity's timer and function pointer or decrements an existing timer and checks for expiration.
- `decrement_entity_timer`: Decrements a short integer located at an offset within an entity structure and checks if the value has reached zero.
- `reset_entity_timer_and_function`: Resets a timer field and a function pointer within an entity structure, then decrements the timer and checks for expiration.
- `decrement_entity_timer_and_check_expiry`: Decrements a timer field within an entity structure and checks if it has reached zero.
- `reset_entity_timer_and_callback_2`: Resets an entity's timer or counter, assigns a new function pointer, and updates a secondary field or structure member.
- `reset_entity_timer_and_callback`: Resets an entity's timer or counter, assigns a new function pointer, and updates a secondary field or structure member.
- `configure_entity_timer_and_callback`: Initializes or resets an entity structure by setting a timer or counter value, assigning a function pointer, and decrementing or resetting certain structure fields.
- `initialize_entity_timer_and_callback`: Initializes an entity structure by setting a timer or counter value, assigning a function pointer, and decrementing or resetting certain structure members.
- `update_entity_state_and_timer_2`: Resets or decrements a timer or counter field within an entity structure and updates a function pointer or secondary field based on certain status flags.
- `reset_entity_attribute_on_condition`: Checks if certain flags in an entity's status byte are unset, and if so, copies a value from a secondary field back to a primary field.
- `initialize_entity_lifecycle_parameters`: Sets or resets certain entity structure members, including resetting a lifetime counter, setting a secondary value, and updating a function pointer to a specific routine.
- `reset_entity_lifetime_and_state_3`: Resets an entity's lifetime counter to zero, sets a secondary value, updates its function pointer, and decrements the lifetime counter.
- `reset_entity_lifetime_and_state_2`: Resets an entity's lifetime counter and secondary value, updates its function pointer, and decrements the lifetime counter.
- `reset_entity_lifetime_and_state`: Resets an entity's lifetime counter and secondary value, updates its function pointer, and decrements the lifetime counter.
- `reset_and_decrement_entity_lifetime`: Resets a lifetime counter and a secondary value in an entity structure, updates a function pointer, and decrements the lifetime counter while checking if it has reached zero.
- `decrement_entity_lifetime`: Decrements a lifetime counter stored in an entity structure and checks if it has reached zero.
- `reset_entity_lifetimes_2`: Resets certain entity lifetime counters to zero and updates function pointers or fixed values within an entity structure.
- `reset_entity_lifetimes`: Resets certain entity lifetime counters to zero and updates function pointers or fixed values within an entity structure.
- `reinitialize_entity_lifetimes`: Resets certain entity lifetime counters to zero and updates function pointers or fixed values within an entity structure.
- `reset_entity_lifetime_and_function_2`: Resets a lifetime counter to zero and sets a new function pointer or a fixed value within an entity structure.
- `reset_entity_lifetime_and_function`: Resets a lifetime counter to zero and sets a new function pointer or a fixed value within an entity structure.
- `decrement_entity_active_counter`: Decrements a word at an offset within an entity structure and checks if the resulting value reaches zero.
- `reset_entity_lifecycle_parameters_2`: Resets an entity's active counter and certain state-related bytes, then updates its function pointer and decrements the counter.
- `reset_entity_lifecycle_parameters`: Resets an entity's counter and certain state-related bytes, then updates its function pointer and decrements the counter.
- `decrement_entity_counter`: Decrements a word-sized counter or value within an entity structure and checks if it has reached zero.
- `reset_entity_health_and_function_2`: Resets a specific field within an entity structure to zero, sets a fixed value, and updates a function pointer.
- `reset_entity_health_and_function`: Resets a specific field within an entity structure to zero, sets a fixed value, and updates a function pointer.
- `reset_entity_properties_2`: Resets a specific field within an entity structure to zero, sets a fixed value, and updates a function pointer.
- `reset_entity_properties`: Resets specific fields within an entity structure, including a counter, a fixed value, and a function pointer, then decrements a counter or updates a field.
- `reset_entity_counters_2`: Resets a counter or a certain field in an entity structure to zero and sets a secondary field to a fixed value.
- `decrement_entity_count_and_recurse`: Decrements a counter in an entity structure and recursively calls itself or other routines if the counter remains non-zero.
- `initialize_entity_list_members`: Sets certain structure fields, including a function pointer and a counter, while decrementing or incrementing a limit value.
- `reset_entity_counters`: Resets a counter or a limit-check value to zero and assigns a new function pointer and a constant value to a structure field.
- `update_entity_state_loop`: Iterates through a sequence of structure initializations and field updates, resetting certain counters and updating function pointers within a data structure.
- `conditionally_update_pointer`: Checks if certain flag bits in a byte pointed to by a register are clear, and if so, copies a longword from a fixed offset in that structure to a different offset within the same structure.
- `update_structure_node_chain_2`: Updates a structure's member or pointer and decrements a counter, potentially updating a linked list or tree-like structure or a byte stream.
- `update_structure_node_chain`: Updates a structure's member or pointer and decrements a counter, potentially updating a linked list or tree-like structure or a byte stream.
- `traverse_linked_list_nodes`: Recursively traverses a linked list or tree-like structure by updating function pointers and decrementing counters within a structure.
- `update_callback_and_decrement_counter_6`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_callback_and_decrement_counter_5`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_callback_and_decrement_counter_4`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_callback_and_decrement_counter_3`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_callback_and_decrement_counter_2`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_callback_and_decrement_counter`: Updates a function pointer and a counter within a structure, and sets a byte at a fixed offset.
- `update_structure_and_set_callback`: Decrements a short integer within a structure, updates a byte at a fixed offset, and sets a function pointer to a routine.
- `update_structure_counter_and_pointer_3`: Decrements a short integer stored within a structure and updates a function pointer or a flag if the counter reaches zero.
- `update_structure_counter_and_pointer_2`: Decrements a short integer stored within a structure and updates a function pointer or a flag if the counter reaches zero.
- `decrement_structure_counter_5`: Decrements a short integer stored at a specific offset within a structure and updates a function pointer or a flag if the counter reaches zero.
- `update_structure_counter`: Decrements a short integer stored at a specific offset within a structure and updates a function pointer or a flag if the counter reaches zero.
- `run_structure_state_machine`: Repeatedly invokes a fixed sequence of helper routines that step a per-structure state, decrementing a counter at offset 34 and writing a function pointer into the structure, and exits the loop when a flag test fails.
- `update_node_and_link_list`: Decrements a counter within a structure member and updates a function pointer or a secondary counter if the value reaches zero.
- `decrement_node_counter_and_link`: Decrements a counter within a structure member and updates a function pointer or a secondary counter if the value reaches zero.
- `update_list_node_counter_2`: Decrements a counter within a structure member and updates a function pointer or a secondary counter if the value reaches zero.
- `update_list_node_counter`: Decrements a counter within a structure member and updates a function pointer or a secondary counter if the value reaches zero.
- `decrement_and_traverse_list`: Decrements a counter in a structure member and recursively calls itself or other routines if the counter is non-zero.
- `sync_structure_member_on_flags`: Checks if certain flag bits are unset in a structure member, and if so, copies a longword from a secondary offset back to a member offset.
- `update_structure_member_and_flag`: Updates a structure member with a new value, sets a flag, and potentially reassigns a function pointer or resets a secondary pointer if certain bitmask conditions or value-checks are met.
- `update_structure_counter_and_pointer`: Decrements a short value in a structure and resets a counter or reassigns a function pointer if the value reaches zero.
- `reset_structure_state`: Resets a structure's counter and function pointer, and conditionally reassigns a function pointer or secondary pointer based on a flag or a check of a global byte.
- `update_structure_state_and_pointers`: Updates a structure's counter and function pointer, and conditionally resets or reassigns a pointer or function address based on a flag in a global structure.
- `initialize_structure_loop`: Decrements a value in a structure and, if it reaches zero, enters a loop that initializes structure data and calls several other routines or updates function pointers and certain byte values.
- `update_entity_function_pointer_2`: Checks specific bits in a structure and, if certain flags are clear, overwrites a function pointer with a saved value while potentially updating a secondary pointer.
- `update_entity_function_pointer`: Checks a flag in a structure and, if certain bits are clear, overwrites a function pointer with a saved value while potentially updating a secondary pointer or a counter.
- `update_structure_function_pointer_4`: Updates a structure or table by overwriting a function pointer with another routine and potentially updating a secondary pointer or a counter.
- `update_structure_state_and_pointer`: Updates a structure or table by overwriting a function pointer with another routine and potentially updating a secondary pointer or a counter.
- `update_structure_function_pointer_3`: Updates a structure or table by overwriting a function pointer with another routine and potentially updating a secondary pointer or a counter-related value if certain bitmask conditions or flag checks are met.
- `update_structure_function_pointers`: Updates a structure or table by overwriting a function pointer with a new routine and potentially updating a secondary pointer if certain bitmask conditions or flag checks are met.
- `copy_data_on_bitmask`: Checks if certain bits in a pointer's target are clear, then copies a longword from a higher offset to a lower offset.
- `load_address_and_call`: Loads a long address into a register and calls another routine.
- `execute_instruction_stream_loop_4`: Executes a sequence of command bytes by updating a structure's counter and instruction pointer.
- `execute_instruction_stream_loop_3`: Executes a sequence of command bytes by updating a structure's counter and instruction pointer.
- `execute_instruction_stream_recursive`: Recursively processes a sequence of command bytes by updating a structure's counter and instruction pointer.
- `execute_instruction_stream_loop_2`: Iteratively processes a sequence of command bytes by updating a structure's counter and instruction pointer.
- `update_structure_control_flow`: Modifies a structure's counter and instruction pointer, then recursively or iteratively executes a sequence of instructions or command bytes.
- `decrement_and_branch_instruction_stream`: Decrements a counter in a structure and conditionally updates an instruction pointer or executes a sequence of instructions.
- `update_instruction_stream_counters`: Iteratively modifies a structure's counter or pointer or writes a new instruction pointer and counter value to a structure.
- `execute_instruction_stream_loop`: Iteratively decodes and executes a sequence of instructions by updating instruction pointers and managing structure-based counters.
- `execute_recursive_instruction_stream`: Recursively executes a sequence of instructions by updating instruction pointers and structure-based counters.
- `reinitialize_instruction_stream_state`: Resets or reinitializes a structure-based instruction stream by updating counters, resetting instruction pointers, and modifying certain structure fields or control flags.
- `update_instruction_stream_state`: Updates a structure-based instruction stream by modifying counters, updating instruction pointers, and resetting or reinitializing certain structure members.
- `execute_instruction_stream_sequence_2`: Executes a sequence of instruction streams by recursively calling a dispatcher, updating structure counters, and managing instruction pointers or conditional jump targets.
- `process_instruction_stream_recursive_2`: A recursive routine that executes a sequence of instruction streams by updating structure counters and instruction pointers.
- `execute_instruction_stream_sequence`: Iteratively initializes and executes a sequence of instruction streams by updating structure counters, setting instruction pointers, and decrementing loop counters.
- `process_instruction_stream_recursive`: Recursively processes a sequence of instructions by updating a counter in a structure and branching to a new instruction pointer.
- `decrement_and_update_instruction_stream`: Decrements a counter in a structure and recursively calls itself or other routines to process a sequence of instructions.
- `process_instruction_stream`: Decodes and executes a sequence of instructions or command bytes, updating structure fields and managing conditional pointer updates or jumps.
- `update_conditional_pointer_data`: Checks if certain bits in a byte pointed to by a register are clear, then copies a longword from a secondary offset of that pointer to a new offset of the same pointer.
- `reset_task_chain_and_set_counter_2`: Resets a task counter, updates a routine pointer, and reinitializes a task structure or chain.
- `reset_task_chain_and_set_counter`: Resets a structure's counter to a fixed value, sets a new routine pointer, and initializes certain structure fields.
- `reset_structure_task_chain`: Resets a structure's counter, assigns a new routine pointer, and initializes or updates structure data or fields.
- `reinitialize_structure_task_chain`: Resets structure counters, assigns a new routine pointer, and initializes structure data or updates fields.
- `reset_and_reinitialize_structure`: Resets a structure's counter and certain fields, then reassigns a routine pointer and initializes a new structure or updates existing structure fields.
- `decrement_structure_counter_4`: Decrements a word-sized value at a location relative to a pointer and stores the result back to that location.
- `reset_and_relink_structure_counters_2`: Resets a structure's counter and certain fields, then updates a function pointer within that structure to point back to itself.
- `reset_and_relink_structure_counters`: Resets a structure's counter and certain fields, then updates a function pointer within that structure to point back to itself.
- `update_structure_counter_and_relink`: Decrements a word-sized counter within a structure and, if the counter reaches zero, resets certain structure fields and updates a function pointer to point back to this routine.
- `decrement_structure_counter_2`: Decrements a word-sized value at an offset within a structure.
- `update_structure_fields_2_2`: Updates certain structure fields and conditionally resets a counter or writes a routine address and a constant value to a structure.
- `update_conditional_value`: Checks if certain bits in a byte pointed to by a register are clear, then copies a longword from a remote memory location to an offset within that same buffer.
- `update_conditional_pointer`: Checks if certain bits in a byte-pointed memory location are clear, then copies a 32-bit value and updates a function pointer if a certain byte is zero.
- `update_conditional_value_2`: Checks if certain bits in a byte-pointed memory location are clear, then copies a 32-bit value from a source offset to a destination offset relative to that same pointer.
- `validate_hunk_type_and_size`: Checks if a hunk type matches specific AmigaDOS executable or relocation codes and validates the associated hunk size or pointer.
- `update_stream_state_2`: Writes a routine address and a pointer to a global variable into a structure, then conditionally updates a counter or a value within a structure based on a comparison.
- `update_stream_state`: Checks a value in a structure and, if it does not match a specific constant, processes a byte stream and conditionally decrements a counter and updates another value using a conditional negation-based increment.
- `update_callback_and_bounds_wrapper`: Updates a function pointer and a pointer-based bound or flag within a structure, while checking or modifying certain bit flags.
- `update_callback_and_bounds`: Updates a function pointer in a structure and checks or modifies certain bit flags and pointer-based bounds.
- `parse_amigadp_hunk_headers`: Checks hunk type codes in a data stream to identify executable or relocation segments.
- `update_callback_pointer`: Updates a function pointer in a structure and modifies bit flags.
- `update_hunk_metadata_size`: updates a structure or a table entry with a new size or pointer based on a transformed value and checks for specific hunk type codes.
- `parse_hunk_metadata`: Checks hunk type codes within a data structure to determine if a routine should update structure flags, append new nodes to a linked list, or modify pointer-based callback or size-related fields.
- `update_structure_flags`: Modifies specific bits in a byte and conditionally updates or resets values and routine pointers within a structure based on a calculated value.
- `check_flag_and_update_value`: Checks if a register contains a specific byte value before checking a flag in a memory structure and calling another routine to update a value.
- `check_global_flag_and_update`: Checks if a byte at a certain offset in a structure is zero before calling a routine to update a value.
- `execute_callback_wrapper`: Calls a routine that checks and sets a value.
- `append_entity_chain_nodes`: Appends a sequence of new structures to a linked list, initializing each node with a specific callback, certain bit flags, and a pointer to the next node in the chain.
- `initialize_entity_chain`: Initializes a linked list of structures by setting callback pointers, flags, and certain data members to fixed values.
- `update_callback_and_transform_state`: Updates a callback counter and conditional function pointer, while calculating and clamping new offset or timer-related values based on transformed input and existing state.
- `update_callback_counter`: Decrements a counter in a structure and clears a callback or updates a function pointer if the counter reaches zero.
- `update_callback_and_decrement`: Updates a function pointer and several data fields using a value from a structure, then decrements a counter and potentially updates a global byte and a callback pointer if the counter reaches zero.
- `set_entity_state_bit`: Clears the least significant bit of a byte in a global structure and sets the least significant bit of a byte pointed to by a register.
- `update_entity_behavior_state_2`: Updates an entity's state, animation frame, and delay timer or next-action timer based on a random value.
- `update_entity_animation_frame_2_2`: Decrements a delay timer and, if it reaches zero, updates an entity's animation frame or delay value using a transformed random or pseudo-random value.
- `update_entity_animation_frame_2`: Updates an entity's animation frame or delay timer by decrementing or incrementing a counter and calculating a new frame offset or delay value based on a transformed random or or pseudo-random value.
- `update_entity_animation_frame`: Updates a pointer-based structure by incrementing or decrementing a counter and updating a frame or animation-related offset based on a bitmask or a transformed value.
- `configure_entity_behavior_and_lifetime`: Updates an entity's status flags and function pointers while calculating a new lifetime or delay value based on a transformed or validated range.
- `update_entity_state_and_bounds`: Updates an entity's status flags and certain coordinate-based properties while checking against boundary limits or a global state.
- `update_entity_position_and_state_3`: Updates an entity's position or state based on a transformed or random value, checking certain coordinate or counter boundaries.
- `update_entity_lifetime_and_state`: Decrements an entity's lifetime counter and updates its status flags or function pointers based on certain coordinate or range-based conditions.
- `update_entity_lifetime_and_position_2`: Decrements an entity's lifetime counter and updates its horizontal position or certain state flags based on a comparison with a global value.
- `update_entity_position_and_lifetime`: Decrements an entity's remaining lifetime counter and updates its horizontal position or certain state flags based on a comparison with a global coordinate.
- `advance_entity_animation_frame`: Decrements a per-entity countdown and, while it remains non-zero and a coordinate-based condition holds, steps the entity's vertical position and frame index, then selects an animation frame value from the stepped index.
- `decrement_counter_and_update_offset`: Decrements a counter in a structure and, if the counter is non-zero and a certain coordinate check passes, increments a secondary counter and updates a coordinate offset or value based on a bit-shifted register value.
- `update_pointer_and_flag`: Increments a pointer and conditionally updates a byte in a structure or writes a routine address to a pointer offset.
- `update_entity_timer_and_bounds`: Decrements a timer or counter value in an entity structure and updates certain status flags or coordinate-based properties.
- `decrement_entity_timer_and_update_state`: Decrements a timer or counter value in an entity structure and updates certain status flags or coordinate-based properties.
- `update_entity_state_and_bounds_2`: Updates an entity's status flags and certain coordinate-based properties while checking against boundary limits or a global state.
- `update_node_state_and_counter`: Sets a bit flag and updates a function pointer or link within a structure, while incrementing a counter and checking if a sum exceeds a certain threshold to potentially decrement a secondary counter.
- `decrement_struct_member_and_limit`: Increments a word in a structure, checks a sum against a constant, and potentially decrements another word in that structure or clamps a value.
- `decrement_struct_member`: Decrements a word at an offset in a structure and copies a word to a subsequent offset.
- `update_entity_movement_and_bounds`: Calculates a new position or direction for an entity and checks if it has hit a boundary.
- `update_entity_boundary_collision`: Checks an entity's position against a global boundary and updates its status flags and function pointers based on whether a collision or boundary violation has occurred.
- `update_entity_collision_and_flags`: Checks an entity's position against a global boundary and updates its status flags and function pointers based on whether a collision or boundary violation has occurred.
- `update_instruction_buffer_and_bounds`: Updates a buffer or structure by setting a flag, updating a function pointer, and incrementing a counter.
- `increment_struct_member_and_limit`: Increments a word in a structure, checks a sum against a constant, and potentially increments another word in that structure or clamps a value.
- `increment_struct_member`: Increments a word at an offset in a structure and copies a word to a subsequent offset.
- `modify_memory_pattern`: Updates a series of adjacent memory locations by applying bitwise OR operations with a sequence of incrementing or decrementing values.
- `update_entity_position_and_bounds`: Updates a memory-resident structure or object by modifying or checking certain byte-flags and calculating new coordinate or boundary values based on a transformed input.
- `update_buffer_bounds`: Modifies a sequence of bytes in memory using bitwise OR operations and updates a pointer-based structure or table with a new function pointer.
- `update_linked_list_nodes_3`: Modifies a sequence of memory-linked nodes by updating pointer-based links and adjusting certain byte-level flags or offsets.
- `update_entity_position_and_state`: Updates a pointer-based structure by modifying certain bytes and updating a position or boundary value.
- `parse_encoded_instruction_stream_2`: Parses a stream of encoded instructions to update buffer boundaries, pointer offsets, and jump table entries.
- `parse_encoded_instruction_stream`: Parses a stream of encoded instructions to update buffer boundaries, pointer offsets, and jump table entries.
- `update_encoded_stream_bounds`: Updates buffer boundaries, pointer offsets, and jump table entries by parsing a stream of encoded instructions.
- `parse_encoded_stream_buffer`: Parses a stream of encoded instructions to update buffer boundaries, pointer offsets, and jump table entries.
- `update_buffer_bounds_and_pointer_2`: Updates a buffer's boundary or offset and modifies a function pointer or jump table entry.
- `update_buffer_bounds_and_pointer`: Updates a structure's offset or boundary values and updates a function pointer or jump table entry.
- `increment_or_decrement_counter`: Increments a value at a certain offset in a structure if it is below a certain threshold, otherwise decrements it if it is below another threshold.
- `update_entity_proximity_logic`: Updates an entity's state and position-based flags or timers based on a distance check against a global reference.
- `update_entity_behavior_state_3`: Updates an entity's state, timer, and certain pointer-based properties based on a transformed or random value.
- `update_entity_behavior_state`: Updates an entity's state, timer, and certain pointer-based properties based on a transformed or random value.
- `update_entity_behavior_state_5`: Decrements an entity's active timer and updates its state or horizontal position based on a distance check relative to a global reference.
- `update_entity_behavior_state_4`: Decrements an entity's active timer and updates its horizontal position or state-based pointer based on a distance check relative to a global coordinate.
- `update_entity_state_and_position`: Decrements a counter in a structure and updates a distance-based or position-based value relative to a target or global reference.
- `update_entity_timer_and_state`: Decrements a timer or counter in an entity structure and updates a state-based or position-based value.
- `increment_and_set_flag`: Increments a word in a structure and sets a bit in a byte if the word reaches a certain value.
- `update_pointer_and_value`: Reads a byte from a pointer, adds it to a value in a structure, updates a different value using a byte from the pointer plus a constant, and advances the pointer by two bytes.
- `update_entity_state_and_lifetime`: Updates an entity's state flags, decrements a lifetime counter, and modifies its position or certain properties based on a transformed value.
- `update_entity_lifetime_and_position`: Decrements a lifetime counter in an entity structure and, if it reaches zero, updates the entity's state flags and position or certain properties using a transformed value.
- `update_entity_position_and_state_4_2`: Updates an entity's coordinate or offset by incrementing or decrementing it based on a comparison with a global value.
- `update_entity_position_and_state_4`: Updates a pointer-based structure by incrementing or decrementing a coordinate or offset value based on a comparison with a global value.
- `update_entity_position_and_state_2`: Updates a pointer-based structure by incrementing or decrementing a coordinate or offset value based on a comparison with a global value.
- `update_entity_state_and_pointer`: Updates a pointer-based structure or table by incrementing a value and modifying a flag, or updates a state-machine-like structure by updating a counter, a function pointer, and a conditional flag or value based on a transformed input.
- `update_entity_kinematics_and_state_2`: Updates an entity's position, velocity, and certain status flags or attribute-related counters.
- `update_entity_position_and_state_5`: Updates an entity's position and velocity-related counters, modifies a function pointer, and updates a global variable.
- `update_entity_kinematics_and_state`: Updates an entity's position and velocity-related counters, modifies a function pointer, and updates a global variable.
- `update_entity_attribute_and_position`: Updates an entity's internal counters, modifies a position-related offset, and updates a function pointer and a global variable.
- `update_entity_state_and_position_2`: Updates a structure's internal counters and position-related offsets, while conditionally updating a global variable and a function pointer or table entry.
- `update_node_and_recursively_traverse`: Updates a flag in a structure and checks a value against a global or external reference, potentially decrementing a counter or recursively calling itself to traverse a tree or linked structure.
- `traverse_and_update_structure`: Recursively traverses a linked structure or tree, updating counters and function pointers while checking or setting flags in a global status byte.
- `update_counter_and_branch`: Increments a pointer and checks a bit in a register to decide whether to recursively call another routine or increment a 16-bit counter in a structure.
- `decrement_counter_and_update_callback`: Decrements a 16-bit counter stored in a structure and updates a function pointer or callback address.
- `process_recursive_state_update`: Recursively updates a counter or pointer-based value and modifies function pointers or flags within a structure.
- `set_state_and_update_function_pointer`: Checks a bit in a structure, sets that bit, and if certain conditions involving a counter and a pointer-based value are met, updates a function pointer in a table and modifies a bit in another structure.
- `set_flag_and_update_state`: Sets a bit in a structure member, increments a pointer, and then either updates a function pointer or modifies a bit in a different structure member based on a register bit or a boolean flag.
- `update_recursive_state_and_counter`: Sets a flag in a structure, increments a pointer, and then either recursively calls another routine or increments a counter in a structure and clamps it to a maximum value.
- `update_counter_and_branch_function`: Sets a flag in a structure, increments a pointer, and then either recursively calls another routine or decrements a counter in a structure and potentially updates a function pointer in a table.
- `clear_bits_and_set_pointer`: Clears the least significant bit of a byte, sets two bytes at an offset to specific values, and stores a pointer to a global variable into a structure.
- `compare_and_update_status`: Calculates the difference between a value in a structure and a value in another structure, then potentially clears or increments a byte in a structure based on the result.
- `modify_byte_with_conditional_call`: Clears a byte in a structure, then checks a bit in a byte and either sets another bit or clears a bit, potentially calling another routine if certain conditions are met.
- `process_encoded_node_stream`: Recursively parses a bit-packed or variable-length data stream while updating node pointers and validating range or length constraints.
- `parse_encoded_stream_node_2`: Recursively parses a bit-packed or variable-length data stream by checking range or length validity and updating stream pointers or counters.
- `parse_encoded_stream_node`: Recursively parses a variable-length or bit-packed data stream by checking range or length validity and updating stream pointers or counters.
- `parse_encoded_stream_node_3`: Recursively parses a bit-packed or variable-length data stream by checking range or length validity and updating stream pointers or counters.
- `parse_recursive_bitstream_node`: Recursively parses a variable-length or bit-packed data stream by checking range or length validity and updating stream pointers or counters.
- `parse_encoded_stream`: Decodes a variable-length or bit-packed byte stream by updating a pointer and a counter.
- `decode_recursive_stream_header`: Recursively parses a bit-packed or variable-length encoded stream by checking range validity and updating stream pointers or counters.
- `update_conditional_pointer_2`: Checks if a value at a pointer offset is negative, increments a pointer, and conditionally updates a pointer or sets a bit in a byte.
- `update_pointer_and_set_value`: Stores the address of another routine into a pointer-sized field or a table, and writes a constant value to a global variable or a structure member.
- `update_entity_state_and_pointer_5`: Updates an entity's state, modifies a counter or pointer, and conditionally inserts or removes an entity from a linked list.
- `update_entity_state_and_pointer_4_2`: Updates an entity's state or pointer based on a comparison between a global value and a structure field, potentially decrementing or incrementing a counter and updating a function pointer or a global variable.
- `update_entity_state_and_pointer_4`: Decrements two short-integer fields within a structure, processes a signed byte stream, and conditionally updates a function pointer, a global variable, and a status byte based on range checks or certain flag conditions.
- `update_entity_state_and_pointer_3`: Updates a structure's internal state or pointer or a global variable based on conditional checks of certain flags and range-based or value-based comparisons.
- `update_structure_and_set_pointer`: Updates a structure's size and pointer offset, then conditionally updates a pointer with a routine address and a byte value or a global variable address based on a bit check and size comparison.
- `update_structure_state`: If certain conditions involving a structure's size and a pointer offset are met, it sets a byte to a maximum value, stores a routine address into a pointer, and assigns a global variable to a pointer.
- `increment_pointer_only`: Increments a pointer and checks a condition that is always false, resulting in no further side effects.
- `set_pointer_target`: Stores the address of this routine into a pointer located at an offset in a register.
- `set_structure_flag`: If certain conditions involving a structure's size and a pointer offset are met, it sets a byte to a specific value, stores a routine address into a pointer, and assigns a global variable to a pointer.
- `increment_pointer_no_branch`: Increments a pointer and checks a condition that is always false, performing no further operations.
- `modify_byte_bits`: Checks a bit in a byte, then either sets a bit or clears a bit in that same byte, and potentially calls another routine.
- `clear_bit_in_byte`: Clears the least significant bit of a byte pointed to by a register.
- `validate_range_and_identifier`: Checks if a value at a specific offset in a structure matches a specific constant and verifies if a certain value falls within a range defined by a global variable and a value at another offset in the structure.
- `manage_node_lifecycle_2`: Decrements a counter within a structure and manages the insertion or removal of nodes within a linked list.
- `manage_node_lifecycle`: Decrements a counter within a structure and manages the insertion or removal of nodes within a linked list.
- `decrement_and_relink_node`: Decrements a counter in a structure and, if it reaches zero, performs a linked list removal or replacement.
- `decrement_node_count_and_relink`: Decrements a counter in a structure and, if the counter reaches zero, modifies flags and updates a pointer to a new node or replaces a linked list node.
- `modify_node_attributes_and_relink`: Updates a structure or node's member or pointer and decrements a counter, potentially triggering a linked list node replacement or further s.
- `update_node_and_decrement_counter_3`: Updates a structure or node's member or pointer and decrements a counter, potentially triggering a linked list node replacement or further structure updates if the counter reaches zero.
- `update_linked_list_node_4`: Modifies a node in a linked list by updating its pointer, setting certain flags, and decrementing a counter or length field.
- `advance_node_state_machine`: A chain of conditional calls that steps through a staged sequence, each stage gated on the zero flag from the preceding call.
- `reset_node_and_update_list`: Resets a structure's fields, updates a linked list node's pointers and flags, and decrements a counter, potentially reinitializing the structure.
- `reinitialize_linked_list_node`: Resets a structure's fields, updates a linked list node's pointers and flags, and decrements a counter, potentially reinitializing the structure if the counter reaches zero.
- `reset_and_relink_node`: Resets a structure's certain fields, updates a linked list node's pointers and flags, and decrements a counter, potentially reinitializing the structure if the counter reaches zero.
- `update_linked_list_node_3`: Modifies a node in a linked list by updating its pointers, flags, and a counter, then potentially reinitializes a structure or updates a pointer if a counter reaches zero.
- `advance_node_state_and_reschedule`: Runs a chain of guarded helper calls that each test the zero flag, and when all pass it marks a structure's flag bits, clears a status bit, and sets a counter to 10 before decrementing it.
- `update_node_and_decrement_counter_2`: Updates a structure's bitflags and pointer, then decrements a counter and updates a pointer to this routine if the counter reaches zero.
- `update_node_and_decrement_counter`: Updates a structure's bitflags and pointer, then decrements a counter and updates a pointer to this routine if the counter reaches zero.
- `update_linked_list_node_2`: Modifies a structure or node by updating certain bitflags, writing a word and a long, and updating a pointer with the routine's own address.
- `update_structure_node_members`: Writes a word and a long to a structure, modifies certain bitflags in a byte, and updates a pointer with a routine address.
- `update_structure_node`: Updates a structure or linked list node by setting specific bitflags and updating a pointer with a routine address.
- `update_structure_fields_4`: Updates a structure or a linked list node by setting specific bitflags and updating a pointer with a routine address.
- `initialize_structure_data_alt_2`: Initializes a structure and potentially performs a linked list replacement or updates a pointer with a routine address.
- `initialize_list_node_on_decrement`: Increments a structure word and, if the zero flag is set, performs a linked list replacement, modifies flags in a structure, and potentially initializes structure data or updates a pointer.
- `decrement_structure_counter`: Decrements a word in a structure and, if the result is zero, initializes structure data and calls another routine.
- `update_pointer_on_decrement`: Decrements a word and, if the zero flag is set, updates a pointer with the address of this routine.
- `modify_entity_bit_flags`: Checks a bit-flag in an entity structure and updates that byte or a bit-flag within it.
- `update_entity_state_and_list`: Checks entity flags and property values to conditionally perform linked list pointer swaps or insertions, and updates entity status bytes or function pointers based on certain conditions.
- `manipulate_linked_list`: Checks a global flag and list length, then conditionally swaps pointers or inserts a node into a linked list while updating a global byte.
- `update_linked_list_nodes_4`: Updates a global pointer, calls a routine to check a value, and conditionally inserts a value into a linked list and updates a global byte.
- `invoke_value_check`: This routine calls a routine that checks and updates a value.
- `update_property_conditionally`: Checks a byte in an entity structure and either calls another routine or calls a routine to check and set a value.
- `update_entity_property`: Checks a bit-flag within an entity structure and, if certain conditions are met, updates a function pointer or a value.
- `update_entity_function_pointer_13`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous pointer value.
- `update_entity_function_pointer_12`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or returns a previously stored value.
- `update_entity_function_pointer_11`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous pointer.
- `update_entity_function_pointer_10`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or returns.
- `update_entity_function_pointer_9`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or returns otherwise.
- `update_entity_function_pointer_8`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or return address.
- `update_entity_function_pointer_7`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous function pointer.
- `update_entity_function_pointer_6`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or returns a previous value.
- `update_entity_function_pointer_5`: Checks specific bit-flags in an entity structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous value.
- `update_entity_function_pointer_4`: Checks specific bit-flags in a structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous value.
- `update_entity_function_pointer_3`: Checks specific bit-flags in a structure and, if certain conditions are met, copies a long-word and updates a function pointer or restores a previous function pointer.
- `update_entity_state_and_pointer_2`: Updates a structure's fields and checks bit-flags before conditionally copying a long-word and updating a function pointer.
- `copy_structure_long_conditionally`: Checks if certain bits in a byte are clear before copying a long from one structure offset to another.
- `transform_structure_data_2`: Modifies a structure or linked list node by clearing bits, updating a pointer with a routine address, and performing arithmetic on a structure member.
- `transform_structure_data`: Modifies a structure or a linked list node by clearing bits, updating a pointer with a routine address, and performing arithmetic on a structure member.
- `transform_structure_and_update_pointer_2`: Iteratively decrements a word in a structure and modifies flags or updates a pointer with a routine address if the word reaches zero.
- `transform_structure_and_update_pointer`: Iteratively modifies a structure's size and pointer, and if a certain condition is met, updates a pointer with a routine address and a global variable address while performing bitwise or arithmetic operations on structure members.
- `iterate_and_transform_pointer`: Iteratively decrements a word in a structure and, if it reaches zero, updates a pointer to a global variable and a routine address, potentially transforming a value and updating a pointer.
- `transform_and_decrement_pointer`: Iteratively transforms a value, updates a pointer, and updates a pointer-based value or a global byte while checking a condition.
- `align_node_offset_to_target`: Adjusts a word value within a structure by incrementing or decrementing it by two until it reaches a target value or falls within a specific range.
- `adjust_offset_bounds`: Increments or decrements a word value at a specific structure offset based on a comparison with a target value.
- `realign_list_node_offset`: Adjusts a word value within a structure by increments or decrements of two to bring it within a specific range relative to a target value.
- `store_routine_address`: Stores the address of the routine into a location two bytes after a pointer.
- `adjust_list_node_offset`: Increments or decrements a word value within a structure based on a comparison with a target value.
- `search_linked_list_bidirectionally_2`: Traverses a doubly-linked list in both directions from a starting node to find a target value or reach a specific boundary.
- `validate_and_assign_callback`: Checks if a value within a structure falls within a specific range relative to a value in another structure.
- `check_pointer_bounds`: Compares a value from a structure against a range and stores a function pointer if the value is within specific bounds.
- `validate_pointer_range`: Checks if a value at a specific offset falls within a calculated range relative to a base value.
- `verify_address_range`: Checks if a value at a specific offset falls within a calculated range and updates a function pointer if it does.
- `store_function_pointer_2`: Stores the address of the current routine into a location offset by two from a pointer.
- `clamp_value_range`: Increments or decrements a word at an offset within a structure based on specific threshold comparisons.
- `increment_value`: Increments a short value stored at an offset within a structure.
- `decrement_value`: Decrements a short value stored at an offset within a structure.
- `search_linked_list_bidirectionally`: Traverses a doubly-linked list in both directions from a starting node to find a target value or reach a specific boundary.
- `return_empty_routine_2`: This routine performs no operations and returns immediately to the caller.
- `store_routine_address_2`: Writes the current routine address to a memory location offset from a pointer.
- `traverse_and_update_buffer_indices`: Validates a buffer index against a threshold and performs a conditional traversal by incrementing or decrementing index values stored in a structure.
- `validate_and_traverse_buffer`: Checks a buffer index against a threshold and performs a conditional traversal using signed offsets.
- `store_function_pointer`: Stores a function pointer into a structure at a specific offset.
- `traverse_and_compare_linked_list`: Traverses a linked list by comparing values at specific offsets against a threshold stored in a global structure.
- `search_and_update_linked_list`: Traverses a linked list structure by comparing values against a threshold and checking specific offsets.
- `increment_counter_with_bounds`: Increments or decrements a counter at an offset within a structure based on a character check and threshold values.
- `update_structure_fields`: Decrements a short value and writes a constant word to a structure if a difference between two values is within a specific range.
- `increment_with_reset`: Increments a word at an offset if it is below a threshold, otherwise increments another word and resets the first word to zero if a condition is met.
- `increment_and_reset_value`: Increments a word at an offset within a structure if a comparison against a global value is within a specific range, then clears a byte at another offset.
- `shift_right_value`: Returns a word from a structure offset shifted right by three bits.
- `copy_structure_data_2`: Copies several values from one structure to another and updates a function pointer and a byte value based on a condition.
- `initialize_structure_data_2`: Copies several values from a source structure to a destination structure and updates a function pointer and a byte value.
- `execute_recursive_call`: The routine performs a direct call to itself, resulting in infinite recursion.
- `update_linked_list_nodes`: Performs bitwise OR operations on several memory locations via pointers and updates function pointers within a structure.
- `reset_function_table_entries`: Sets specific bits in a series of bytes and overwrites two entries in a table with the address of the current routine.
- `reset_function_pointers`: Checks a byte at an offset from a base pointer and, if non-zero, clears that byte and overwrites two entries in a table with the address of this routine.
- `update_linked_list`: Iteratively updates function pointers and modifies a byte in a global structure until a condition is met.
- `unlink_linked_list_node`: Modifies a node's bitmask and updates its pointer to bypass it within a linked list.
- `update_structure_state_flags`: Checks a global threshold value to determine whether to modify bitmasks and function pointers within a structure.
- `iterate_structure_list`: Traverses a list of structures to modify bitmasks and function pointers based on a global threshold.
- `process_linked_list_nodes_2`: Iterates through a linked list of structures, updating bitmasks and function pointers based on a global threshold value.
- `traverse_and_modify_linked_list`: Iterates through a linked list of structures, updating bitmasks and function pointers within each node.
- `update_linked_list_nodes_2`: Modifies bitmasks and function pointers within a linked list of structures based on a global threshold value.
- `modify_list_nodes`: Traverses a list of structures to modify bits and update function pointers based on a global value.
- `process_linked_list_nodes`: Traverses a list of structures, modifying bits and updating function pointers based on values in a global table.
- `set_flag_on_range_match`: Checks if a value in a structure matches a value in another structure and if a certain range is valid, then sets or clears a byte in a structure.
- `update_accumulated_values_2`: If a certain flag is set, it removes nodes from a linked list, updates several global and structure-based values using arithmetic, and potentially updates a routine pointer.
- `update_accumulated_values`: Checks a value in a structure, and if not a specific constant, updates several global and structure-based values using arithmetic.
- `decrement_structure_counter_3`: Decrements a word in a structure and, if the result is zero, updates a pointer with a routine address and modifies a global variable or a structure member.
- `decrement_structure_and_set_routine_2`: Decrements a word in a structure and, if it reaches zero, updates a pointer with a routine address and modifies a global byte or a pointer.
- `decrement_structure_and_set_routine`: Increments a pointer, modifies a bit in a global byte, and decrements a structure word, potentially updating a pointer with a routine address and a global variable address if the word reaches zero.
- `decrement_and_set_pointer`: Sets a bit in a byte, decrements a word in a structure, and if the word reaches zero, assigns a global variable and a routine address to a pointer.
- `set_pointer_if_true`: Increments a pointer and, if a condition is met, updates a pointer with the address of this routine.
- `set_byte_on_condition`: Sets a byte in a structure to a specific value if a certain condition is met, and always sets a byte at a different offset to a constant value.
- `set_structure_bit`: Sets a specific bit in a byte at an offset from a pointer.
- `update_stream_state_and_counters_3`: Updates a structure's counter or pointer based on a transformed or signed byte stream, while checking certain bitmasks and threshold values.
- `update_stream_state_and_counters_2`: Updates a structure's counter or pointer based on a transformed or signed byte stream, while checking certain bitmasks and threshold values.
- `update_stream_state_and_counters`: Updates a structure's counter or pointer based on a transformed or signed byte stream, while checking certain bitmasks and threshold values.
- `process_byte_stream_2`: Decrements a word in a structure and, if certain conditions or a certain value or a certain threshold are met, updates a byte or a pointer, or calls a routine to process a signed byte stream.
- `decrement_and_check_counter`: Decrements a word in a structure and, if the result is zero, updates a value in that structure, sets a bit in a byte, and overwrites a function pointer.
- `update_node_link_pointers_2`: Updates a node's flags and pointers with values from a source node and recursively updates a target node's link pointers.
- `update_node_link_pointers`: Updates a node's next-pointer or link-pointer with values from a source node and then recursively calls itself.
- `empty_routine_2`: The routine contains no executable instructions.
- `decrement_structure_member`: Decrements a word in a structure and checks if the result is zero.
- `update_structure_member`: Decrements a byte in a structure and checks if the result is zero.
- `decrement_structure_byte`: Decrements a byte at an offset from a pointer and checks if the result is zero.
- `increment_structure_word_2`: Increments a word in a structure.
- `update_structure_pointer_and_counter`: Updates a structure field with a new routine address and a constant value, while decrementing a counter and checking bitflags.
- `process_structure_stream`: Decrements a word in a structure and, if certain bitflags are clear, updates a pointer with a routine address and copies a long value.
- `update_structure_pointer`: Stores a dword from a pointer into a structure offset and updates a pointer with a routine address.
- `decrement_structure_word_2`: Decrements a word in a structure and checks if it has reached zero.
- `decrement_structure_word`: Decrements a word in a structure and checks if it has reached zero.
- `update_structure_data`: Checks if certain bits in a byte of a structure are clear, then copies a longword from a different offset in that structure.
- `update_structure_function_pointer_2`: Checks if certain bits in a byte of a structure are clear, then copies a longword and updates a function pointer.
- `update_structure_function_pointer`: Iteratively modifies a structure and, if certain bits in a byte are clear, copies a longword and updates a function pointer.
- `copy_word_on_bit_check`: Checks if certain bits are clear in a byte, and if so, copies a 32-bit value from one location in a structure to another location in the same structure.
- `set_structure_fields`: Sets a byte to a certain value and stores a pointer to a global variable into a structure.
- `update_structure_state_5`: Updates a value in a structure using a signed difference and conditionally modifies function pointers or bits in a global variable.
- `update_structure_state_4`: Updates a value in a structure using a signed difference, modifies function pointers, and conditionally sets a bit in a global variable based on a bit-test.
- `assign_function_pointers`: Writes a routine address into a structure or table at a specific offset.
- `update_conditional_bit_and_pointer`: Updates a bit in a memory location based on a bitmask check and conditionally modifies a pointer or a global variable or a function pointer or a table entry based on a range check or a comparison of a value and a pointer offset.
- `check_bit_condition`: Compares two or-values and checks if a value is within a certain range before setting a bit in a global variable.
- `update_entity_state_and_timer`: Copies a value from a structure to another, decrements a counter, and updates a state byte based on the counter's progress.
- `decrement_and_copy_data`: Decrements a word in a structure and, if it reaches zero, copies data from one structure to another and updates a global byte.
- `set_constant_value`: Stores the constant value twenty-two into a word at a specific offset from a global pointer.
- `set_constant_value_alt`: Stores the constant value twenty-three into a word at a specific offset from a global pointer.
- `copy_structure_data`: Copies several words from one structure to another and updates a byte in a global variable.
- `nop_instruction`: This routine performs no operation and returns immediately.
- `initialize_structure_sequence`: Repeatedly calls a routine to initialize structure fields and skip bytes, then sets a byte in a global structure to a maximum value.
- `set_global_flag`: Checks a condition via another routine and, if true, performs several subsequent calls before setting a byte in a global structure to a specific value.
- `initialize_structure_fields`: Populates a structure with various values from a source memory area, including coordinate-like data and a constant.
- `update_entity_position_and_timer`: Calculates new coordinate values by combining multiple structure fields and updates a decrementing timer.
- `decrement_word`: Decrements a word at a specific offset within a structure.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `update_structure_fields_3`: Updates a structure by incrementing a value, clamping a second value within a range, and updating a pointer using an offset from a table.
- `update_index_and_pointer`: Updates a word at an offset by incrementing or decrementing it within a range, then copies a word from a table into a structure based on that value.
- `update_state_and_pointer`: Modifies several fields in a structure, including a clamped value and a pointer retrieved from a table using an offset.
- `update_state_and_pointer_2`: Adjusts values within a structure and updates a pointer by indexing into a global table with a clamped value.
- `update_object_lifespan_and_flags_3_2`: Decrements an object's lifespan counter and updates its status flags and function pointers.
- `update_object_lifespan_and_flags_3`: Decrements an object's lifespan counter and updates its status flags and function pointers.
- `update_object_lifespan_and_flags_2`: Decrements an object's lifespan counter and updates its status flags and function pointers.
- `update_object_lifespan_and_flags`: Decrements an object's lifespan counter and updates its status flags and function pointers.
- `decrement_object_lifespan_and_relink`: Decrements a counter in an object structure and, if the counter reaches zero, updates object flags and modifies pointers to redirect the object's linked-list or or-masking entry to a global variable or another routine.
- `update_object_list_and_counters`: Iterates through a linked list or array of objects, incrementing or decrementing counters and updating pointers or flags within the object structures.
- `update_object_counter`: Increments a counter or a value at an offset from a pointer and checks if the new value is zero.
- `decrement_and_check_counter_2`: Decrements a counter stored at an offset from a pointer and checks if the new value is zero.
- `decrement_counter_and_update_pointer`: Decrements a value stored at an offset from a pointer and calls a routine to increment a pointer with a specific value.
- `update_bit_masks`: Sets or updates several bits in various memory locations using pointers and or-operations.
- `update_bit_masks_subset`: Performs a subset of bitwise or-operations on several memory locations using pointers.
- `modify_structure_flags`: Clears specific bits in a structure member and, if a bit was previously unset, updates a structure member with a pointer, writes a value from a transformed index, and sets flags in another structure.
- `update_structure_bounds_2`: Decrements a value in a structure, clamps it to a minimum limit, and adds a calculated offset to another structure member.
- `set_structure_members_2`: Writes a constant byte to a memory location, updates a pointer in a structure, and stores the address of a routine into a structure member.
- `update_clamped_value`: Decrements a word in a structure, clamps it to a minimum value, and updates another member with a fraction of the result.
- `clamp_structure_value`: Increments a word in a structure up to a certain limit and updates another member with a fraction of that value.
- `update_structure_member_clamped`: Increments a value in a structure up to a certain limit and updates another member with a fraction of that value.
- `increment_and_clamp_structure_member_2`: Decrements a value in a structure and clamps it to a minimum limit, then updates another structure member by adding a fraction of the new value.
- `increment_and_clamp_structure_member`: Increments a structure member and clamps it to a maximum value, then updates a secondary member by adding a fraction of the new value.
- `update_value_with_offset`: Calculates a signed offset based on a comparison and adds it to a value in a memory location.
- `update_structure_bounds`: Calculates a new value based on a pointer offset and a constant, then stores it back to a structure member and updates another member using a scaled version of the new value.
- `update_structure_bounds_and_state_3`: Updates a structure's coordinate or size-based bounds and modifies a status flag.
- `update_structure_state_logic`: Updates structure-related flags and coordinate or size-based bounds, potentially updating a function pointer and a global pointer or table entry.
- `update_structure_bounds_and_state_2`: Checks or updates certain bounds or state flags within a structure, potentially updating a function pointer and a global variable.
- `update_structure_bounds_and_state`: Checks or updates certain bounds or state flags within a structure, potentially updating a function pointer and a global variable.
- `update_structure_state_3`: Checks a bit in a register and conditionally sets a bit in a structure.
- `update_and_set_structure`: Calls a routine to update accumulators and conditionally sets a bit in a structure or updates a structure with a pointer and a global variable.
- `set_structure_members`: Writes a value to a memory location, sets a byte in a structure to a fixed value, and copies a value from a pointer-referenced address.
- `decrement_and_process_stream_2`: Decrements a value in a structure and calls a routine to process a signed byte stream.
- `traverse_and_process_stream`: Decrements a value in a structure and calls a routine to process a signed byte stream while traversing a pointer table.
- `update_struct_member_and_flag`: Writes a dword to a structure offset, sets a byte in a structure to one, and copies a dword from a table or structure offset into a third structure member.
- `increment_and_process_stream`: Increments a value in a structure and calls a routine to process a signed byte stream.
- `increment_and_traverse_table`: Increments a value in a structure and calls a routine to process a signed byte stream while traversing a table.
- `initialize_structure_data`: Writes a word, a long, and a byte to a structure, then copies a long from an offset of another structure pointed to by the current structure.
- `decrement_and_return_word`: Decrements a word at an offset within a structure and returns the new value.
- `initialize_structure_data_alt`: Writes a word, a long, and a byte to a structure, then copies a long from a different offset of another structure pointed to by the current structure.
- `increment_structure_word`: Increments a word located at an offset within a structure pointed to by a register and returns the updated value.
- `update_structure_fields_2`: Stores a word and a long value into a structure and copies a long value from a pointer located within that structure.
- `decrement_and_process_stream`: Decrements a value in a structure, potentially updating another value and clearing a bit, then calls a routine to process a byte stream.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d0, then returns.
- `set_structure_members_extended`: Writes a word and a long to a structure and copies a value from a pointer-referenced address.
- `increment_and_check_limit`: Increments a value in a structure and checks if it exceeds a certain limit, potentially clearing a bit and incrementing another value before calling a stream processing routine.
- `update_structure_values`: Stores a byte and a long into a structure and updates a long value from a table based on a comparison.
- `update_stream_index_and_wrap`: Increments or decrements an index within a structure and checks if it exceeds a certain limit or reaches zero, potentially incrementing a counter and masking a bit in a secondary value before processing a byte stream.
- `update_pointer_target_bounds`: updates a 32-bit value in a structure using a value from a secondary table, or replaces it with a different value from that table if a certain condition involving a comparison of two 16-bit values is met.
- `compare_struct_members`: Loads a word from a structure and compares it against another word in the same structure before branching or clearing a register.
- `update_state_function_pointer`: Decrements global and offset values and updates a function pointer in a structure based on a conditional check.
- `increment_pointer_with_value`: Adds a byte from a pointed-to location to a word in a structure and increments the pointer by two.
- `process_signed_byte_stream`: Reads a byte from a pointer to update a word in a structure, increments the pointer by two, and updates the pointer with another value if the first byte is negative.
- `update_multiple_accumulators`: Adds bytes from a data stream to two different short values in a structure and increments the data pointer by four.
- `traverse_buffer_with_offset`: Iterates through a buffer using signed offsets until a negative byte is found, then stores the final offset and a sum.
- `validate_bit_flags_2`: Iterates through a table to check specific bit patterns in a byte stream, returning a failure value if certain bits are not set.
- `skip_odd_bytes`: Increments a pointer by a fixed amount repeatedly while the byte at the pointer has its lowest bit set.
- `calculate_scaled_value`: Compares two words and, if they are equal, returns a value derived from dividing a short by a constant.
- `check_range_validity`: Checks if a value in a structure falls within a specific range defined in another structure.
- `validate_transformed_value`: Validates a transformed value against a range and a secondary calculated limit, returning a specific value if certain conditions are met.
- `traverse_table_for_sentinel`: Iterates through a table using a pointer derived from a structure, checking for a negative value or a specific sentinel value.
- `traverse_pointer_table`: iterates through a table of pointers, checking if any pointer-referenced value is negative, until a specific termination value or a value derived from a global variable is reached.
- `clear_memory_block_2`: Sets a large number of individual bytes and words within a memory block to zero.
- `relocate_data_block`: Calculates a destination address based on a hunk type, copies data, and modifies hardware registers and system memory.
- `update_audio_parameters`: Swaps values between global variables, modifies a counter, and updates hardware registers and memory locations.
- `configure_hardware_registers`: Writes specific values to three hardware registers.
- `copy_data_block`: Copies a large block of data from a source pointer to a destination pointer.
- `initialize_structure_pointers`: Sets a byte to zero and stores two pointers to a global variable at specific offsets within a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_bit_patterns`: Clears memory, performs bitwise rotations to generate a pattern of bytes, and executes a series of subroutines while updating a global value.
- `store_returned_values`: Stores three values returned from another routine into specific offsets within a structure.
- `increment_bounded_value`: Adds a value to a memory location and updates it only if the resulting sum is below a specific threshold, otherwise entering an infinite loop.
- `execute_infinite_loop`: An unreachable routine that performs an infinite loop and touches the first color register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_single_instruction`: A single instruction routine that is executed during runtime.
- `execute_single_instruction_2`: This routine consists of a single instruction.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_copper_and_jump_tables`: Stores immediate pointers into a6-relative memory, sets copper list pointers, calls three internal setup routines, then loops forever.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_a1_and_copper_list_setup`: Saves a1 into (a6), writes two immediate values to a6 offsets 100 and 104, calls a subroutine, loads the custom chip base into a5, writes a copper pointer, clears two bytes and masks interrupts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_or_bits_and_clear_state_bytes`: ORs immediate bits into two memory words, clears bytes at a6 offsets 113 and 108, stores pointers, calls setup routines, then waits on a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_copper_and_level_interrupts`: Writes a copper list pointer to custom register 104, calls the setup routine, points the level 3 vector at new code and clears the interrupt enable byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_level_sequence_with_delays`: Saves the old level 3 vector, installs a new one, calls several setup routines, waits in 300-iteration delay loops and toggles a flag byte before restoring the vector.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_level3_vector_and_reset_counters`: Saves and replaces the level 3 interrupt vector, calls a routine with a0 and d0 set, then clears two word counters in low memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_and_wait_loops`: Calls several internal routines, clears a word at offset 4 of a6, busy-waits until that word reaches 300, sets bits in a4-relative memory, toggles a byte bit selected by a returned value, then stores a longword into a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_sr_and_clear_display_words`: Clears a byte at offset 108 of a6, copies the low memory longword to offset 118, restores the status register from the stack, calls a routine with a0 pointing at a fixed address, then clears two words in display memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_two_hundred_ticks_twice`: Calls a delay routine, clears a word at offset 4 of a6, compares it against 300 and branches back while less, then calls another routine, loads a fixed address into a0 and repeats the wait sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_vector_clear_audio_state`: Saves the old level 3 vector, installs a new one, restores the status register, calls a sound routine with d0 zero, then clears the sound flag and six words of audio state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `delay_toggle_bit_and_call_sound`: Waits about 300 iterations, toggles a bit in a byte indexed by a4, then calls three sound routines and restores the saved vector.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_sound_flag_and_ten_words`: Calls a sound routine with d0 zero, clears the sound flag, then zeroes ten consecutive words starting at the audio state area.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_init_wait_300_then_store_global`: Calls two setup routines, clears the word at offset 4 of a6, busy-waits until that word reaches 300, calls a teardown routine, then copies the longword at offset 6 of a6 into a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_call_clear_state_and_toggle_flag`: Sets d0 to zero, calls a routine, clears a global word and ten words starting at the state block pointed to by a0, toggles a bit in a4-1, then runs the same init, wait and store sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_wait_300_restore_sr_and_frame`: Calls two setup routines, clears the word at offset 4 of a6, waits until it reaches 300, calls a teardown routine, then copies the longword at offset 6 of a6 to a low memory word with interrupts masked before restoring sr and a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_word_and_ten_a0_words`: Clears the word at absolute address a fixed address, loads a0 with a fixed address, then clears the word at a0 and the ten words at offsets 4 through 36 from a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_counter_wait_300_then_store_vector`: Clears the word at offset 4 of a6, spins while that word is below 300, calls a subroutine, then stores the longword at offset 6 of a6 into the low memory vector at 108 with interrupts masked.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_fifteen_words_at_a0_offsets`: Clears fifteen consecutive words starting at offset 12 from a0 through offset 68, leaving a0 and the other registers unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_a6_and_sr_from_stack`: Copies the longword at offset 6 of a6 into absolute address 108, then pops the status register and a6 from the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_fourteen_words_at_a0_offsets`: Writes zero to fourteen consecutive word fields starting at offset 16 of a0 through offset 68, then returns the low byte of d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_sr_and_a6_then_return`: Restores the status register and a6 from the stack and returns to the caller without touching any other register or memory.
- `level3_interrupt_save_all_and_ack`: Saves all data and address registers on the stack, increments the word at offset 4 of the structure pointed to by a6, restores registers, writes 112 to the interrupt request register and returns from exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_fifteen_words_at_a0_offsets_2`: Clears fifteen consecutive words starting at offset 28 from the address in a0, zeroing fields through offset 84, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `level3_interrupt_ack_without_saving`: Increments the word at offset 4 of the structure pointed to by a6, pops all registers from the stack, writes 112 to the interrupt request register and returns from exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_a0_words_32_to_84`: Clears fourteen consecutive 16-bit words at offsets 32 through 84 from the address in a0, leaving a0 and d0 unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `vblank_counter_increment_and_ack`: Increments the 16-bit counter at offset 4 of a6, restores d0-d7/a0-a6 from the stack, acknowledges interrupt request bits with value 112 at the custom chip register and returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_a0_words_48_to_100`: Clears fourteen consecutive 16-bit words at offsets 48 through 100 from the address in a0, leaving a0 and d0 unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `interrupt_return_stub`: Executes an rte instruction to return from an exception, restoring the saved status register and program counter from the supervisor stack.
- `fill_five_copper_entries_from_a6`: Reads a longword from the address in a6, then writes five pairs of high and low words into consecutive eight-byte entries starting at the copper list area, adding 10240 to each successive value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_a0_offsets_and_fill_copper_entries`: Clears fifteen word fields at offsets from 60 to 120 in the structure pointed to by a0, then writes five high and low word pairs into consecutive eight-byte entries from d0, adding 10240 each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_five_entries_with_stepped_d0`: Loops five times writing the high word of d0 to offset 2 and the low word to offset 6 of each 8-byte entry at a0, adding 10240 to d0 and advancing a0 by 8 each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_offsets_then_fill_five_entries`: Clears thirteen words at offsets 68 through 120 of a0, then writes the high and low words of d0 into offsets 2 and 6 of five 8-byte entries, stepping d0 by 10240.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_entries_stepped_by_d7`: Repeatedly writes the high word of d0 to offset 2 and the low word to offset 6 of each 8-byte entry at a0, adding 10240 to d0 and advancing a0 until d7 counts down to -1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_a0_offsets_and_fill_palette_steps`: Zeroes eleven words at offsets 80 to 124 from a0, calls a routine for a base value, clears a word at a6+110, then stores 32 words stepping by 10240 into successive 8-byte slots.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_a0_by_eight_and_loop_dbra`: Adds 8 to a0, decrements d7 and branches back 8 bytes while d7 is not minus one, then returns to the caller.
- `parse_cmap_and_body_chunks`: Scans a0 for the CMAP tag, decodes 32 packed colour words into a6+12, then scans for the BODY tag and clears words at offsets 96 to 132 from it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_sound_channel_fields_and_wait`: Zeroes seven word fields at offsets 96, 100, 104, 108, 112, 120 and 124 of the structure pointed to by a0, calls a sound routine, then spins until the word at a6+110 reaches 30 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_pmap_and_body_tags_load_palette`: Scans a1 for the PMAP tag, unpacks 32 palette entries into a6+12, scans a0 for the BODY tag, sets flags at a0+108, waits, then repeats a palette reload and delay loop 256 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reload_palette_from_body_and_delay`: Clears words at a0+112, 120 and 124, calls a sound routine, waits until a6+110 reaches 30, then unpacks 32 palette entries from the body structure into a6+12 and runs a 256-iteration delay loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unpack_3bit_planar_to_words`: Reads three masked bytes from (a1), combines them into one word via shifts and or, writes to (a2), repeats d7+1 times, then scans a0 for the BODY marker and calls the run-length routine five times per row over 256 rows.
- `rle_decode_40_bytes_to_a1`: Decodes a run-length stream from (a0) into (a1): a negative count byte repeats the next byte, a positive count copies that many bytes, until 40 output bytes are produced, returning the count in d0.
- `adjust_palette_entries_toward_target`: For 25 passes over 32 words, compares each palette word at a0 with the target word at a1 and increments the red, green and blue nibbles separately when they differ, storing the updated word back.
- `decrement_packed_bcd_nibbles_table`: Loops 25 times over 32 words at the pc-relative table, decrementing each nonzero nibble of every word by one and storing the result back, then returns.
- `packed_bcd_table_data_byte`: A single ori.b instruction whose bytes are table data reached by a pc-relative lea, not executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_stack_and_a6_then_return`: Calls two subroutines, copies the longword at offset 118 of a6 to absolute address 108, loads a7 from 114(a6), pops a6 from the stack and returns.
- `vblank_interrupt_dispatch`: Saves all registers, runs two subroutines and a counter when the flag at offset 108 is set, then acknowledges the level 3 interrupt and returns from exception.
- `swap_two_longwords_at_a6_offsets`: Exchanges the longword at offset 100 of the a6 structure with the longword at offset 104, using d0 as temporary, then returns.
- `copy_five_entries_to_hardware_list`: Reads the longword at offset 100 of a6 and writes five successive entries eight bytes apart starting at the fixed table, adding 10240 to each value, then returns.
- `increment_color_ramp_six_rounds`: Loops six times, waiting until word at offset 110 of a6 reaches one, then calls the color routine on six or thirty-two table entries before returning.
- `increment_packed_rgb_nibbles`: Reads a word from a0, increments its three nibble fields unless each is at maximum, writes the combined word back to a0 and returns.
- `increment_color_ramp_fifty_rounds`: Loops fifty times, waiting until word at offset 110 of a6 reaches four, then calls the color routine on thirty-two table entries before returning.
- `decrement_packed_bcd_nibbles`: Reads a word from (a0), subtracts one from each non-zero nibble of the three nibble groups, writes the result back to (a0) and returns.
- `copy_bitmap_and_palette_then_play`: Saves a0, calls the chunk parser, copies 32 longwords from the bitmap pointer to the screen pointer, copies 32 palette words, scans for the DLTA tag, then loops setting the frame flag, clearing the counter, calling the blitter routine and waiting until the counter reaches the frame limit.
- `parse_cmap_dpan_body_chunks`: Scans a0 for the CMAP tag, converts 32 packed three-byte colours into words at a6+16, sets five plane pointers spaced 10240 bytes apart, reads the DPAN frame count and delay, then scans for the BODY tag and calls the decompressor for each of the five planes.
- `rle_decompress_40_bytes_to_a1`: Reads run-length encoded bytes from a0, expanding signed counts as repeats and positive counts as literals into a1, until 40 output bytes are written, then returns.
- `dispatch_five_plane_bitmap_blocks`: Reads five length-prefixed blocks from a0 and calls the plane blitter for each into a1 plus offsets 0, 10240, 20480, 30720 and 40960, then advances a0 by 64 and returns.
- `blit_40_row_plane_block`: Decodes a column-oriented block at a0 plus d0 into a2, writing bytes down 40-row strides, handling zero-run, negative-run and skip codes, then returns.
- `init_audio_channels_and_voices`: Stores the voice base pointer, scans 128 bytes for the highest value, then clears 31 voice structures and fills the voice pointer table, finally resetting audio volumes and state flags.
- `stop_audio_and_disable_dma`: Clears the audio-active flag, silences all four audio volume registers and writes 15 to DMACON to enable only the four audio DMA channels.
- `per_frame_audio_voice_update`: Advances the frame counter, then updates each of the four audio voices' volume and pointer registers from the voice tables and current sample data.
- `load_channel_event_from_stream`: Reads a four-byte event from the stream at a0 plus d1, advances d1 by four, decodes the channel and command nibbles, fills the a6 channel block from the a3 table and calls the volume and pitch update routines.
- `start_audio_channel_playback`: Computes the remaining duration from the a6 channel block, writes the sample pointer, length and period into the a5 audio structure, sets the DMA control bits and advances the module position counter.
- `slide_channel_volume_toward_target`: Compares the target volume in the a6 channel block with the current volume, steps it by the stored delta, clamps at the target and writes the resulting volume to the a5 audio structure.
- `beastlord_entity_command_dispatch`: Reads the command word at offset 2 of the entity structure, masks it and branches to sub-handlers for movement, speed, direction and sound commands, writing results to offset 6 of the a5 structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_2`: this routine is empty and performs no operations.
- `return_immediately`: The routine is empty and performs no operations before returning to the caller.
- `return_no_op`: The routine consists of a single return instruction and performs no operations, no hardware register access, and no library calls.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, leaving no register or memory changes.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work.
- `do_nothing_3`: the routine is empty and performs no operations.
