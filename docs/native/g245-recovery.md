# Game 245 (Beastlord v1.0 2452): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00245). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1517788 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 45 of 45 units identical (bytes and relocations), 52 raw-word lines |
| Functions named and described | pass, 747 of 747 named (0 author symbols), 747 of 747 described (11 by rule, 735 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 8697 of 9125 (95.31%) |
| Recovered without load address (counted in proof 1, flagged) | 1792 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13934 of 14448 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 327341 |
| code | 54084 |
| data | 1123123 |
| padding | 8 |
| unused | 13232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Beastlord.slave | 1 | 54 | 6 |
| logged disk load from Disk.1 | 1 | 12988 | 672 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 825 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Disk.1 bytes 0-1792 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Beastlord_v1.0_2452/Beastlord/Disk.1: raw disk image (822624 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Beastlord_v1.0_2452/Beastlord/Disk.2: raw disk image (673056 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- runtime segment an address (play-rs-m1500-unpk03): 531 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Beastlord_v1.0_2452/Beastlord/Disk.1: bytes 0-1792 look like code no run loaded: recovered position-independently (load address unknown)
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
- Beastlord_v1.0_2452/Beastlord/Disk.1: 4 logged loads cover 388588 bytes of the image
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 8-8 (bytes 2048-2304) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 10-10 (bytes 2560-2816) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 12-12 (bytes 3072-3328) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 14-14 (bytes 3584-3840) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 16-16 (bytes 4096-4352) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 18-18 (bytes 4608-4864) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.13)
- Beastlord_v1.0_2452/Beastlord/Disk.1: chunks 20-21 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.035, entropy 3.93)
- Beastlord_v1.0_2452/Beastlord/Disk.2: chunks 0-7 (bytes 0-2048) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.02)
- Beastlord_v1.0_2452/Beastlord/Disk.2: chunks 19-920 (bytes 4864-235776) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.98)
- 1 never-loaded region(s), 1792 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 858496 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `interleave_and_verify_bits` (Disk.1 +an address loaded at an address (194168 bytes)): Initializes custom chip registers and a DMA buffer, then interleaves the bits of successive words into a destination buffer while verifying a running XOR against expected values. Kind: Bit-interleave and verify (data integrity check on the disk data). Pass path: uVar6 == expected XOR (returns in_D1) or *(unaff_A6+4) == 0 (returns concatenated uVar4/in_D1).

## Functions


### Beastlord.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_and_monitor_globals`: Initializes a set of global variables with fixed startup values, then runs a loop that repeatedly calls another routine and reacts to flag bytes: when a flag is set it invokes a jump-table entry and waits for it to clear, and when a related value is zero it swaps two stored values.
- `pass_d1_through`: Passes the value in data register 1 straight through to the caller without altering it.
- `set_global_flag_bit`: Waits until a global flag bit clears, then sets that flag and returns the incoming data register pair unchanged.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_table_entry_passthrough`: Dereferences a fixed offset inside a jump table and calls the routine found there, returning the data register pair unchanged.

### logged disk load from Disk.1

- `run_setup_routine_chain`: Writes a series of fixed marker addresses into a global table through a pointer, then repeatedly calls dispatch_setup_routines to run a chain of setup routines.
- `dispatch_setup_routines`: Dispatches to a series of setup routines using a table of addresses, then enters a loop that repeatedly calls helper routines and branches based on status flags and counters stored in global variables.
- `initialize_structure_and_build_pointers`: Initializes a structure by writing fixed constants and copied values into its fields, clears several byte and word regions, then builds pointer tables by copying blocks and splitting addresses into low and high halves.
- `copy_and_interleave_tables`: Copies a 400-word block from one table to another, then interleaves a 79-word block into a destination table nine times per outer iteration.
- `fill_sprite_descriptor_block`: Computes several byte offsets from three input values and writes a block of sprite descriptor fields into a table, advancing the source pointers each iteration.
- `check_and_clear_status_byte`: Checks a byte in a hardware structure and returns early if it is zero, otherwise sets another byte to a fixed value and runs a series of loops that repeatedly clear bytes at fixed offsets until a status byte stops being set.
- `clear_and_repeat_routine`: Clears a word in memory via a pointer, then checks whether that word is zero.
- `dispatch_on_structure_field`: Tests a short field in a structure and, depending on whether it is zero, calls a helper routine either several times or only once before returning.
- `zero_table_entries`: Computes a base address and zeroes a repeating run of four-word entries in a table.
- `configure_blitter_offsets`: Derives offsets from a value and configures the blitter registers, adjusting the settings when a masked condition is met.
- `program_blitter_transfer`: Reads a value, computes a mask, and programs the blitter registers for a transfer.
- `set_blitter_copy_block`: Computes a column offset and sets the blitter registers to copy a source block to a destination.
- `program_blitter_per_entry`: Walks through a table of three-word entries until it reaches a sentinel value of negative one, invoking the blitter programming routine once per entry.
- `program_blitter_from_input`: Selects blitter parameters from an input value and programs the blitter registers, then continues configuring further fields.
- `copy_and_store_pointer`: Copies a 16-bit value from one offset of a structure to another, then stores a fixed routine address into a following field.
- `configure_blitter_sprites`: Clamps a passed value to a range and stores it, then, when it is below a threshold, computes an address from a base and configures the blitter registers and sprite pointers, walking through a table of entries until it finds a terminator.
- `dispatch_via_jumptable`: Looks up an entry in a jump table indexed by a value in a register and jumps to the corresponding routine.
- `dispatch_indirect_jump`: Reads an address offset from a register and jumps indirectly through a jump table located at that computed address.
- `wait_blitter_and_program`: Waits for a blitter operation to finish by polling a status byte, then programs the blitter registers and reads a byte from a pointer to determine whether to install a jump vector.
- `configure_sprite_blitter_2`: Decodes a compressed routine table and, when a special marker is present, dispatches to a routine from a sprite-blitter table.
- `configure_sprite_blitter`: Reads a sprite entry from a table indexed by a register value, then computes and stores blitter setup values (source/destination pointers, offsets, and size) into a global structure.
- `adjust_counter_and_store_offsets`: Adjusts a stored counter by decrementing it when a flag is clear and incrementing it when it holds a specific value, then derives a base offset from that counter and stores eight shifted and offset copies of the base into a run of consecutive words.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_rollover_counter`: Increments or decrements a byte counter and rolls over the higher digits of a multi-byte decimal counter, then adjusts a position byte and advances a date-like field, wrapping its components when they exceed a limit.
- `setup_blitter_and_store_pointers`: Sets two blitter size registers, derives a value from a hardware byte, and stores a global address into several fixed slots, then clears a block of eight buffer slots when a flag bit is unset and stores more addresses otherwise.
- `copy_paired_words`: Copies a series of paired words from a table indexed by a computed offset into a destination buffer, writing each source word twice.
- `decrement_clamped_counter`: Decrements a short counter stored in a structure and clamps it to a fixed maximum value when it would go negative.
- `advance_timer_tick`: Drives a repeating countdown timer stored in a small data block.
- `invoke_divide_and_copy_triple`: It calls a divide-and-copy routine three times in a row, each time passing the result of the previous call as its input.
- `divide_and_copy_table`: Writes zeros into a destination buffer when a divisor is zero and the dividend is smaller than the divisor, otherwise performs repeated subtraction to count how many times the divisor fits into the dividend and copies eight spaced values from a source table into the destination.
- `divide_and_copy_words`: Performs a repeated subtraction loop to divide two values, then copies a block of paired words from a source table into a destination buffer.
- `swap_structure_fields`: Calls a helper routine several times, then swaps two four-byte fields inside a structure before making one further helper call.
- `process_structure_command`: Dispatches on a command byte stored in a structure and updates its flag, counter, and pointer fields, calling helper routines to initialize and copy data.
- `init_struct_and_dispatch`: Initializes a structure by clearing several flag and vector fields, installing jump vectors at fixed offsets, and then dispatching through a flag-based jumptable.
- `copy_and_spread_words`: Copies three consecutive words from a source table into a destination table, then spreads those words into several other destination offsets and sets a fixed constant and a few flag bytes, before invoking several helper routines.
- `copy_memory_blocks`: Copies a block of longwords and shorts from one memory region into another at fixed offsets, using a routine to fetch the source pointers before copying.
- `merge_marker_entries`: Searches a null-terminated marker table for a matching value and, when found, looks up two entries in a per-marker data table.
- `decode_marker_stream`: Walks a null-terminated sequence of short markers, accumulating and transforming bytes from a source buffer into destination slots based on the marker lengths.
- `reset_structure_fields`: Copies two double-word fields from one structure location into adjacent fields and sets two other fields to a fixed constant, then writes a sentinel short into a derived location.
- `init_globals_and_fill_buffer`: Initializes a global variable, runs a short delay loop, and fills a byte buffer by shifting bits across eight rounds.
- `store_structure_values`: Fetches a value from a fixed offset in a structure and stores it into three consecutive 32-bit fields of that same structure.
- `increment_until_overflow`: Increments a stored value and stores it back while it stays within a bound, otherwise enters an endless idle loop.
- `route_on_value_sign`: Calls a routine with a passed short value, inspects the sign of that value and the contents of a pointed-to longword, stores a computed signed byte into a structure field, and returns a value depending on the sign.
- `store_start_pointer`: Stores the address of a named data block into a pointer field within a structure.
- `increment_counter_with_limit`: Adds a value to a counter stored in a global variable and stores the result back, but only when the incremented value does not exceed a threshold held in a second global variable; otherwise it enters an endless loop.
- `route_on_value_sign_alt`: Calls a routine with a passed short value, inspects the sign of that value and the contents of a pointed-to longword, stores a computed signed byte into a different structure field, and returns a value depending on the sign.
- `copy_doubleword_between_fields`: Copies a doubleword from one global variable into another at an adjacent offset, overwriting the first value with the second.
- `traverse_linked_list_by_table`: Walks a linked list of nodes using a delta stored in each node, reading a byte from a per-node table and branching on its value until a zero byte terminates the traversal.
- `process_file_header_block`: Reads a five-character ASCII identifier from a structure and, based on the following bytes, builds a small lookup table and a bit pattern, then walks that table to compute a signed offset that is stored back into the structure.
- `multiply_byte_chain`: Multiplies a byte held in a register by two across successive iterations, propagating a carry into the next byte when it overflows.
- `copy_buffer_by_blocks`: Copies a variable number of doublewords from one buffer to another, handling long runs in 32-element blocks and finishing the remainder two bytes at a time.
- `clear_word_array_2`: Walks a large word array starting from its last element, writing zero to each slot until the whole region is cleared.
- `clear_word_array`: Clears a fixed-size array of 11000 words pointed to by a global variable by writing zero to each element in a loop.
- `clear_global_variable_regions`: Clears successive regions of a global variable by writing zero to word-sized spans of differing lengths in five separate loops.
- `run_nested_decode_loop`: Runs a nested loop that repeatedly invokes another routine and then waits for a status byte to become zero before repeating.
- `decode_packed_nibbles`: Decodes a packed nibble value held in a word by subtracting one from each non-zero nibble and recombining the adjusted nibbles back into the same word.
- `drive_nibble_incrementer`: Runs a fixed sequence of nested loops that repeatedly invoke another routine to increment the differing nibbles of two words, then sets a flag byte at a fixed offset and waits for it to clear before repeating the whole sequence.
- `increment_differing_nibbles`: Compares the nibbles of two words and increments each differing nibble of the first word by one, storing the adjusted value back.
- `adjust_byte_with_flag`: Decrements a byte when a related flag is clear, calls another routine several times, and then copies one byte into a neighbouring field.
- `align_word_arrays_nibble_by_nibble`: Compares pairs of words from two arrays nibble by nibble and adjusts the first array's values by small offsets until the arrays match or a counter reaches its limit.
- `read_inputs_and_clear_flag`: Reads the joystick and mouse ports, clears a hardware request register, and stores a fixed constant into a global variable.
- `init_global_and_wait`: Initializes a global variable, performs a short software delay loop, and then clears it, while touching the CIA keyboard serial register and interrupt request register.
- `clamp_byte_to_zero`: Reads a signed byte from a fixed offset in a hardware structure and clamps negative values to zero before returning it.
- `read_joystick_store_byte`: Reads the joystick data register and combines its bits with a global flag, then stores a byte derived from a table lookup into a hardware register.
- `clamp_mouse_delta`: Reads the mouse port data register, computes the signed delta since the previous sample, clamps it to stored minimum and maximum limits, and stores the result back.
- `mask_button_state`: Reads a stored byte, derives a button mask from global flag bits, stores the mask, and clears the corresponding bits in a second stored byte.
- `wait_for_flag_spin`: Waits in a busy loop until a global variable becomes nonzero, then spins forever in an unconditional loop.
- `double_and_xor_value`: Doubles a stored 32-bit value and, if the shift carries out a bit or the result is zero, XORs it with a fixed constant before storing it back.
- `copy_buffer_and_increment_counter`: Calls a routine, then checks a flag, and if certain, copies a block of bytes from a source pointer to a destination pointer, increments a counter in a structure, and continues a loop or returns.
- `tiny_addq_moveq_rts`: Small routine of 3 instructions that adds 8 to a7, then loads 255 into d0, then returns.
- `interleave_and_verify_bits_2`: Selects a table entry from a small indexed table based on a passed value, stores a two-character signature ('1-DMB' or '2-DMB') into a nearby field, and then repeatedly drives another routine while polling a status bit and copying data between buffers until the operation completes or fails.
- `interleave_and_verify_bits`: Initializes custom chip registers and a DMA buffer, then interleaves the bits of successive words into a destination buffer while verifying a running XOR against expected values.
- `load_indexed_value`: Loads a value from a table indexed by a structure field, invokes another routine, and returns the original indexed value.
- `store_table_word_from_index`: Reads a word from a table indexed by a value, stores it into a global variable, invokes another routine, and returns the original D0 word.
- `set_constant_and_call`: Stores a fixed constant into a global variable and then invokes another routine.
- `set_bit_and_increment`: Sets a single bit in a global variable, then calls another routine and increments a short value pointed to by a computed offset.
- `set_bits_and_decrement`: Sets two adjacent bits in a global variable, then calls another routine and decrements a short value pointed to by a computed offset.
- `wait_indexed_value_settle`: Loads a value from a table indexed by a structure field, then repeatedly invokes another routine until the returned value stops changing in the expected direction, effectively waiting for that value to settle.
- `wait_flag_then_clear_indexed`: Waits in a loop while a bit in a global variable stays set, then clears a word inside a structure indexed by a structure field, and finally returns the value held in a register.
- `start_and_wait_timer`: Configures a timer by setting its control and interrupt registers, then waits until the timer's status bit is set.
- `call_routine_repeatedly`: Invokes the same routine six times in succession and then returns without modifying any data.
- `configure_and_wait_timer`: Configures a timer with different control values, then waits until the timer's status bit is set.
- `poll_bit_with_counter`: Initializes a global variable to two fixed byte values and a counter, then loops while polling a bit in the global variable and clearing another bit, decrementing the counter until it underflows.
- `build_bitmask_from_flags`: Returns a nonzero value when a short at a computed offset is zero, otherwise builds a bitmask by repeatedly testing a bit in a global variable and setting a corresponding bit before decrementing a counter.
- `return_negative_one`: An empty routine that immediately returns a fixed nonzero value.
- `check_signature_and_store_pointer`: Compares a four-character signature stored at an offset from a pointer against the ASCII bytes '1-DMB'. If the signature does not match, it stores the address of a global variable into two consecutive 16-bit words and assigns that same global variable to point at itself.
- `compute_coords_and_scan_records`: Reads structure fields to compute a pointer into memory, stores the resulting coordinates and size back into the structure, and then walks a linked list of records while invoking another routine for each entry.
- `update_sprite_descriptor`: Toggles a flag in a per-object structure and, when the object is a sprite of type 3, rewrites the sprite's custom-blitter descriptor block in place, shifting each field by one word to make room for a new entry.
- `setup_blitter_sprite`: Adjusts a sprite's vertical flip based on a small offset, then computes its source coordinates and size from a table.
- `store_coords_if_near`: Compares a base structure's coordinates against a passed-in position and, when both the horizontal and vertical separations fall within small ranges, stores two incoming words into adjacent fields of that structure.
- `clear_table_and_scan_records`: Clears a fixed-size doubleword table held in a structure and then scans a repeating record table, writing a constant into a field and copying a pointer into a linked slot whenever a flag bit is set and a matching value is found.
- `setup_blitter_tile_draw`: Reads tile metadata from a table, clamps a computed screen coordinate into range, and when it is valid sets up Blitter transfer parameters and sizes to draw the tile.
- `compute_min_distance`: Computes the absolute difference between a passed-in value and a stored coordinate, records it, tracks the smallest such value seen, and invokes a pointer stored in a record when surrounding conditions hold.
- `reset_field_if_value`: Checks whether a stored word equals a specific constant and, if so, overwrites it with another value held in a register.
- `copy_record_pair`: Reads a counter from a global structure and uses its value to index into a table of six-byte records, then copies those six bytes into a destination block by writing each byte to two separate offsets.
- `process_queue_entry`: Consumes a byte from a queue and, based on its value, clears or sets specific flag bytes in a global structure.
- `lookup_table_entry`: Uses a byte passed in a register as an index into a global table and returns the two-byte value located there.
- `initialize_queue_structure`: Initializes a global structure by filling its four queue slots and several fields with fixed marker values, zeroing a block of words, and writing a repeating byte pattern into a seven-byte region.
- `match_and_replace_string`: Clears a few structure fields, skips leading spaces in a source string, then walks a table of length-prefixed entries matching them against the source and stores the corresponding replacement bytes into separate fields based on a marker byte.
- `process_string_replacements`: Walks through a source string and compares it against a series of match-table entries, advancing to the next entry when the strings differ.
- `copy_byte_and_match_table`: Copies a byte from a source location into a destination through a pointer, then stores that byte into one of three distinct fields depending on which terminator byte follows it.
- `match_table_and_store`: Walks a table of length-prefixed entries, comparing each against an input string and matching on a terminator byte.
- `match_table_and_store_alt`: Walks a table of length-prefixed entries, comparing each against an input string and matching on a terminator byte.
- `match_table_and_store_reordered`: Walks a table of length-prefixed entries, comparing each against an input string and matching on a terminator byte.
- `clear_flag_byte`: Writes a zero byte into a single field of a structure.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a1)+, then returns.
- `compare_and_traverse_buffer`: Compares a sequence of bytes between a buffer starting at a certain offset in a1 and a buffer starting in a2, updating a1 to a new pointer or skipping forward based on a length byte if a mismatch or match occurs.
- `check_byte_and_branch`: Loads a byte from an offset of a structure in a1, compares it to a constant, and branches to another routine if it matches, otherwise it updates a pointer and branches to another routine.
- `execute_indirect_jump`: Loads a pointer from an offset of a register and jumps to that address.
- `reset_field_if_match`: Checks if a value at an offset in a structure equals a constant, and if so, updates a value in that structure with a value from a pointer-based address.
- `compare_byte_and_return`: Loads a byte from an offset of a1, compares it to a constant, and branches to another routine if it matches, otherwise it returns.
- `update_and_clamp_counter`: Initializes two structure fields to zero and increments a counter masked to a small range, then invokes a routine stored in the structure.
- `manage_state_and_animation`: Clears a flag and, based on a state byte, either calls a helper routine or decrements a sequence counter with wrap-around, then stores pointers and offsets into a structure.
- `self_register_if_flag`: Reads a short flag from a structure and, when it holds a sentinel value, stores its own address into a pointer field of that same structure.
- `store_pointer_if_flag_set`: Reads a 16-bit value from a location derived from a pointer stored at an offset, and if that value equals -1, stores the address of this routine into a function-pointer slot at another offset.
- `init_null_routine_ptr`: Reads a 16-bit value from a table entry and, when it equals -1, stores a routine address into a field of the same structure.
- `store_pointer_when_flag_set`: Reads a short value from a table indexed by a stored offset and, when that value equals -1, writes the address of a different routine into a function pointer field of the same structure.
- `noop_then_clear_global`: Runs an empty decrementing loop that performs no work and then stores a zero into a global variable.
- `loop_update_state_fields`: Increments a byte counter in a structure and, at counter values four and two, calls another routine while storing two computed short values into adjacent structure fields.
- `loop_and_assign_state`: Runs a repeating loop that increments a counter and, at specific counter values, invokes a routine while adjusting two structure fields.
- `store_pointer_if_byte_differs`: Compares a byte at a fixed offset in a structure against a constant and, when they differ, stores a routine address into a function-pointer slot in the same structure.
- `loop_update_state_fields_two`: Increments a byte counter in a structure and, at counter value four, calls another routine while storing two computed short values into adjacent structure fields.
- `loop_and_assign_state_two`: Runs a repeating loop that increments a counter and, at specific counter values, invokes a routine while adjusting two structure fields differently from the previous routine.
- `store_ptr_if_byte_differs`: Reads a byte from a fixed memory location and compares it against a specific value; when the values differ, it stores a routine address into a function pointer at a fixed offset.
- `check_scroll_flag_then_advance`: Tests a counter range and a status flag to set a pointer field, installs a callback, then runs the shared counter-increment routine that repositions coordinate fields.
- `advance_scroll_sequence`: Increments a byte counter and, once it reaches a threshold, repositions several coordinate fields, then adjusts a scroll counter and swaps two values while installing a callback pointer.
- `advance_and_track_timer`: Updates a timer-like counter by a variable amount and, when a separate byte counter reaches five, triggers another routine while storing several computed values into structure fields.
- `decrement_and_track_counters`: Increments a byte counter and, at value five, calls another routine while storing several computed values into structure fields.
- `advance_and_track_counters_2`: Updates a counter stored in a structure field by adding a value derived from another field, then adjusts that source field and, if a tracked value is still within range, copies it into a location pointed to by a nearby field.
- `advance_and_track_counters`: Increments a byte counter and, at value five, calls another routine while storing several computed values into structure fields.
- `check_scroll_state_conditions`: Checks several status flags and counter fields in a structure and, when they satisfy a set of range conditions, sets a byte to a sentinel value and installs a callback pointer so a follow-up routine runs.
- `manage_scroll_state_machine`: Reads status flags from a hardware register and dispatches to a state machine that adjusts two counter fields, clamps them to bounds, and occasionally updates a pointer and a stored value from a base address.
- `advance_scroll_background`: Manages a scrolling background by advancing a tile pointer and adjusting the horizontal and vertical scroll offsets.
- `advance_cyclic_angles`: Initializes two counter fields from a stored value when they are not already set, then clamps a position field between bounds based on flag bits.
- `clamp_byte_decrement_counter`: Clamps a byte field to a minimum value, decrements a short counter while preventing it from dropping below a floor, and then invokes another routine.
- `clamp_byte_increment_counter`: Clamps a byte field to a maximum value, increments a short counter while preventing it from exceeding a ceiling, and then invokes another routine.
- `clamp_and_half_decrement`: Clamps a counter to a minimum of negative six, then reduces a second field by half of that clamped value.
- `increment_and_store_pointers`: Records a handler pointer, increments a stored counter by three, caps it at a constant, and then stores another pointer along with computed base offsets.
- `increment_and_cap_counter`: Increments a stored counter by three, caps it at a constant when it reaches that value, and then records a handler pointer along with computed base offsets.
- `store_handler_and_offsets`: Stores a handler pointer and a byte constant into a structure, zeroes a flag byte, and writes a computed base address plus an offset into two fields.
- `handle_counter_and_pointer`: Increments a counter and, when it reaches a specific value alongside a nonzero flag, invokes a routine and stores its result along with several structure updates.
- `store_pointer_if_short_equals`: Checks whether a short value pointed to by a pointer equals a specific constant and, if so, stores a routine address into a function-pointer slot in the structure.
- `accumulate_byte_from_pointer`: Reads a byte through a pointer, advances that pointer, accumulates the byte into a short counter, and either stores a routine address or invokes another routine depending on a short value.
- `increment_and_check_counters`: Increments a counter stored in a data structure and adds the previous value to another counter.
- `advance_state_and_set_pointer_2`: Writes a fixed byte and two computed values into a structure, stores a function pointer, and increments a byte that, when it reaches eight and a nearby field is nonzero, triggers another routine and sets a flag.
- `advance_state_and_set_pointer`: Increments a byte at a fixed offset within a structure and, when that byte reaches a specific value and another field is nonzero, invokes a helper routine and initializes a nearby short to one if it was zero.
- `set_flag_when_needed`: Checks two flag bytes and, when the relevant condition is not met, records a pointer to itself and sets a flag byte before returning.
- `init_scroll_counter_sequence_2`: Initializes a scroll-control block by storing a state byte, a handler pointer, and two counter values derived from a base field, then calls a routine that clamps a stored counter upward and adjusts a running counter.
- `handle_state_transition`: It compares a field against a value returned by another routine and, when a condition holds, adjusts a counter and clamps a stored value to that returned value.
- `decrement_scroll_counter`: Runs two scroll-state helpers, decrements a counter field, and clears a flag while reinstalling a callback pointer when that counter reaches zero.
- `clamp_counter_upward`: Initializes several flag bytes and a handler pointer, then compares a stored counter against a routine result and clamps it upward using a capped addition when the condition holds.
- `update_counter_or_store_pointer`: Compares a value from a structure field against a routine result and, when a condition holds, updates a counter using a capped addition before invoking another routine.
- `return_empty`: An empty routine that returns immediately without touching any data or registers.
- `initialize_and_decrement_timer_2`: Initializes a timer block by writing a marker byte, storing a handler address, and presetting a counter with its active flag, then branches on a flag byte to either cap a computed counter value or decrement a countdown and clear it when it reaches zero.
- `advance_state_timer`: When a flag byte is not set, it compares a value from another routine against a counter and, if the counter is low enough, raises that counter by one and then clamps it to a second value before storing the result back.
- `decrement_and_store_pointer`: Decrements a counter stored at an offset within a data structure and, when it reaches zero, clears a nearby byte and stores a routine address into a field of that same structure.
- `init_and_cap_counter`: Initializes a structure by writing a marker byte, storing a handler address, setting two flag bytes, and computing two offset values from a base field.
- `update_counter_or_store_pointer_2`: Compares a value from a structure field against a routine result and, when a condition holds, updates a counter using a capped addition before invoking another routine.
- `do_nothing`: This routine contains no instructions and performs no operations.
- `decrement_five_counters_wrap`: Clears a flag byte, then reads a table entry and, depending on its value, decrements five counters by increasing amounts while wrapping them back into range and adjusting a shared timer.
- `decrement_sequence_counters`: Checks a flag and a position value in a table, then either reduces a counter by two and stops or decrements five consecutive counters by fixed amounts while wrapping each back into range when it reaches zero.
- `advance_position_counter`: Advances a cyclic position counter stored in a table and, depending on the current position, increments a timer value by a small or large amount while wrapping it into range.
- `increment_counters_wrap_slow_2_2`: It checks a flag bit and a modulo condition, then increments a counter by two when it stays below a threshold, otherwise it advances several counters by fixed amounts while wrapping each one back below its modulus.
- `scan_table_for_negative`: It walks through a table of signed words advancing by three each step, returning negative one when it finds a negative entry, and stopping when the current entry falls outside a range derived from a byte and a word, otherwise returning the matching table entry.
- `update_structure_and_check_byte`: Computes a value from structure fields, sets a status byte based on comparison results, marks a byte with a high bit, updates counters, writes computed coordinates into the structure, and then inspects a memory byte to decide whether to reset two fields.
- `fetch_table_byte`: Reads a byte from a table indexed by a value derived from fields in the structure pointed to by A6, returning that byte.
- `lookup_table_byte`: Reads a byte from a table indexed by a value stored in a global variable, using a pointer also held in that variable, and returns that byte or a fixed fallback value when the indexed entry is too large.
- `increment_word_checked`: Adds a passed value to a stored unsigned word, storing the sum back.
- `add_with_overflow_check`: Adds a value to the 32-bit number stored at a fixed memory location and writes the result back, setting it to -1 if the unsigned addition overflows.
- `increment_word_check_overflow`: Adds a value to a stored word and writes the result back, forcing the result to negative one when the addition overflows.
- `decrement_word_clamped`: Subtracts a passed value from a stored unsigned word, storing the result back.
- `decrement_and_clamp_u32`: Loads a 32-bit value from a memory location, subtracts a value passed in register D0, and stores the result back.
- `decrement_and_clamp_value`: It decrements a 32-bit value pointed to by a register by the amount in D0, then clears that value to zero if the original amount was smaller than the decrement.
- `adjust_word_signed`: It inspects a flag byte and, when set, calls three helper routines and returns; otherwise it reads a 32-bit value, branches on its sign, and calls a pair of helper routines to add or subtract a shifted amount from a stored word with overflow and clamping checks.
- `init_entity_state_block`: Initializes a per-entity data block by writing a routine pointer and several state bytes, loads a pointer and its first value from a table, sets a few counters to large values, and zeroes a long list of fields.
- `process_flags_and_count_list`: It sets or clears two flag bytes based on a status byte and a counter, then walks a linked list of pointers counting entries until it finds one whose following word equals a marker, storing the resulting count or a sentinel into a field.
- `set_flags_and_dispatch_jumptable`: It sets or clears two flag bytes depending on a counter value and a flag bit, and when a specific condition holds it dispatches to a routine selected from a jumptable using an index.
- `update_linked_list_status`: Checks a flag, retrieves a list length, and potentially updates a byte in a structure or inserts a node into a linked list.
- `update_list_state`: Updates a global variable, checks a list length, and potentially clears a byte in a structure or updates a byte in a structure if certain conditions or list lengths are met.
- `store_byte_at_register_plus_7`: It stores the low byte of a data register into the byte immediately following the address pointed to by an address register.
- `copy_block_of_words_3`: It invokes a single helper routine that copies a block of 16-bit words from one memory region to another, advancing both pointers by a fixed stride each iteration, then returns.
- `call_then_return`: This routine only invokes another routine before returning, performing no operations itself.
- `copy_block_of_words_4`: Copies a block of 16-bit words from one memory region to another, advancing both pointers by a fixed stride each iteration.
- `do_nothing_2`: The routine contains no instructions and returns immediately without performing any operation.
- `dispatch_from_jumptable_2`: It reads a word and, when it equals zero, dispatches to one of several routines selected from a jumptable using an offset, otherwise it returns without doing anything.
- `copy_block_of_words`: It copies a block of 16-bit words from one memory region to another, advancing both pointers by a fixed stride each iteration.
- `compute_and_store_offsets`: It reads several fields from a structure and writes computed values into a set of fixed offsets.
- `store_constants_into_struct`: It writes a fixed set of constant 16-bit values into a series of offsets in a structure, leaving the rest untouched.
- `copy_pair_of_words`: It copies two adjacent 16-bit values from one pair of offsets in a structure to another pair of offsets.
- `program_blitter_on_overflow`: Combines several signed fields from a structure and detects overflow in their sums.
- `blit_with_overflow_check`: It combines several fields of a structure, detects signed overflow in their sums, and on each overflow path waits for a blitter flag bit to clear before writing fixed values into custom registers and computed offsets into a structure, then continues with further flag-waiting and value-store steps.
- `wait_flag_and_store_values`: It repeatedly waits for a flag bit in a global variable to clear before writing a sequence of computed values into a structure.
- `wait_flag_and_accumulate`: It repeatedly waits for a flag bit in a global variable to clear before writing a sequence of values into a structure.
- `clamp_and_adjust_offset`: Clamps a value stored at offset two so it never exceeds the difference between two other fields, then adds a third field to it and stores the result back.
- `route_or_reset_handler`: Writes a short value into a structure slot, then compares two stored integers and inspects a stored byte.
- `clear_byte_and_set_vector`: Clears a byte field, invokes another routine, and installs a jump vector at a fixed offset before returning.
- `process_byte_stream_2`: Stores a counter when a field is zero, then walks a byte string dispatching on each byte to invoke other routines for special marker values and otherwise handling printable characters, and finally stores a routine address before returning.
- `do_nothing_3`: The routine contains no instructions and does nothing; it simply returns to its caller.
- `compute_division_loop`: Repeatedly decrements one unsigned value by another until it underflows, then compares the iteration count to a third value and calls another routine with their sum when they differ.
- `apply_bitmask_to_planes`: Iterates over a source table while applying bit-masked AND/OR/XOR operations to six interleaved planes in a destination bitmap.
- `reset_memory_blocks`: Walks a fixed-size block of memory, resetting paired regions to all-ones, zero, or a filled short pattern on each pass.
- `configure_blitter_transfer`: Compares a global value against the sum of two fields, then sets a pointer and a series of blitter control registers, waiting on a status bit before configuring the transfer.
- `program_blitter_transfer_2`: It branches on a global flag to update one of two linked-list counters, then programs a series of blitter control registers (pointer, size, and geometry fields) while waiting on the blitter status bit, finally returning a transfer length.
- `check_threshold_and_invoke`: Compares a stored value against the sum of two other values held in a table.
- `call_and_store_address`: Invokes another routine and then stores a routine address into a global slot before returning.
- `broadcast_nibbles_store_address`: It broadcasts nibble values into a destination buffer, then stores a routine address into a global slot before returning.
- `broadcast_then_return`: Calls a routine that broadcasts nibble values, then stores a return address into a global slot before returning.
- `broadcast_nibble_values`: Extracts a nibble from a source byte stream and broadcasts the value and its complement to a spread of fixed offsets in a destination buffer.
- `init_and_adjust_struct_fields`: Clears several byte and 16-bit fields in a structure, installs jump vectors at fixed offsets, calls another routine multiple times, then copies a 16-bit value, negates it into a nearby field, clears two adjacent bytes, and stores a scaled product of a shifted value and a signed offset.
- `copy_negate_and_scale`: Copies a field from one offset to another, negates the copied value, clears nearby flag fields, and computes a scaled product from two other fields before returning.
- `read_custom_registers`: Reads custom chip registers for blitter and copper settings, derives shifted and offset values from them, and stores the results into a global variable.
- `count_and_insert_list`: Traverses a linked list by following pointers until reaching a null terminator, counting the nodes along the way.
- `swap_memory_words`: Swaps two words between two different memory locations.
- `set_structure_byte`: Writes the constant value 130 to an offset of a pointer in a register.
- `insert_ordered_node`: Walks an ordered linked list comparing a field of the incoming record against each node, then inserts the record in place and rewires the surrounding pointers.
- `find_first_matching_node`: Walks a linked list of records, comparing a passed value against the second field of each node, and returns the one-based position of the first matching node or zero when no match is found.
- `return_position_from_counter`: Walks a linked list while decrementing a passed counter, returning the one-based position reached when the counter hits zero or the list length when the list ends first.
- `find_flagged_node_2`: It walks a linked list of records, decrementing a counter and testing a bitmap flag against each node, and returns the one-based position where the flag matches or the list ends.
- `find_flagged_node`: Walks a linked list while decrementing a counter, checking a bitmap flag against each node and returning a position when the flag matches or the counter reaches zero.
- `find_next_flagged_node`: Walks a linked list of records, skipping nodes whose second field is not the special marker until it finds a node flagged in a bitmap, and returns a running counter or a saved value when the list is exhausted.
- `tiny_move_rts`: Small routine of 2 instructions that copies d2 to d0, then returns.
- `return_next_flagged_position`: Walks a linked list of records, skipping nodes whose second field is not the special marker until it finds a node flagged in a bitmap, and returns a running counter or zero when the list is exhausted.
- `count_specific_nodes`: Traverses a linked list and counts how many nodes contain a specific value in a certain offset.
- `remove_nodes_by_value`: Traverses a linked list to find and remove all nodes that contain a specific value, updating pointers to bypass them.
- `insert_if_not_present`: Searches for an index in a linked list and inserts a new node if the index is not found.
- `initialize_structure_fields`: Stores a byte value of one hundred and twenty-seven at a fixed offset in a structure and writes a pointer to a global variable at another fixed offset.
- `process_flags_and_walk_list`: Copies one byte into an adjacent slot and clears that slot, then derives several values into nearby slots from a shifted register and a table, and finally walks a linked list of nodes, clearing a flag in each and calling the routine stored at each node until the list ends.
- `empty_return_routine_2`: An empty routine that returns immediately without modifying any data.
- `copy_structure_fields`: Copies several fields from a source structure to a destination structure, or initializes a field to a constant if a flag is not set.
- `update_state_and_counter`: Checks a bitmask in a pointer, increments a counter, and updates a secondary value based on that counter.
- `update_list_state_on_counter_match`: Checks a byte at offset 296 of a6 and, if zero, checks if a 16-bit value at offset 264 of a6 is zero; if so, it calls return_position_from_counter, and if a certain condition is met, updates a pointer in a linked list to point to a global variable before calling reset_field_with_pointer.
- `reset_field_with_pointer`: Loads a pointer into a2 and calls a reset routine.
- `update_list_node_on_counter_match`: Checks if a flag or counter in a structure is zero, then if a specific value is found at an offset, updates a pointer in a linked list or structure to point to a global variable and calls a routine to reset a field.
- `load_address_and_reset_field`: Loads the address of a string into a register and calls a routine that resets a field if a certain value is present, then returns.
- `load_address_and_reset_field_2_2`: Checks a byte at offset 360 of a6 and, if not zero, loads a longword address into a1 and a longword address into a2, then calls another routine.
- `load_address_and_reset_field_2`: Loads a long-word address into a register and calls reset_field_if_value, then returns.
- `increment_counter_and_offset`: Increments a value in a structure and calculates a new offset or value based on the incremented result.
- `load_address_and_call_8`: Loads a new address into a1 and calls another routine.
- `update_struct_and_branch_on_result`: Calls a routine to store a structure field, then if the zero flag is not set, loads a new address into a1 and calls another routine with d0 set to 32798; otherwise it returns.
- `call_init_and_set_da00`: Calls a routine, then if the zero flag is not or not set, loads a new address into a1 and calls another routine with d0 set to 36896.
- `call_and_update_pointer`: Calls a routine, then if the zero flag is not set, writes 200 to offset 34 of a structure pointed to by a0 and writes a new address to offset 2 of a structure pointed to by a0.
- `decrement_and_load_address_2`: Decrements a byte at offset 34 of a structure pointed to by a0, then if the zero flag is not set, loads a new address into a1 and calls another routine.
- `call_init_and_set_d801e`: Calls a routine, then if the zero flag is not set, loads a new address into a1 and calls another routine with d0 set to 32798.
- `call_init_and_set_d20`: Calls a routine, then if the zero flag is not set, loads a new address into a1 and calls another routine with d0 set to 33280.
- `updates_struct_and_branches_on_zero`: Calls a routine to increment a structure value, then conditionally branches to a different routine if the zero flag is not set, otherwise it loads a new address into a1 and calls another routine.
- `call_subroutine_and_load_address_4`: Calls a subroutine, otherwise if the zero flag is not unable to be certain, loads a long address into a1, moves a constant into d0, and calls another subroutine.
- `update_stream_pointer_and_counter`: Calls another routine, and if the zero flag is not set, updates a 16-bit value at offset 34 of the pointer in a0 to 20 and updates a 32-bit pointer at offset 2 of the pointer in a0 to a new address.
- `decrement_and_call_subroutine_2`: Decrements a byte at offset 34 of the pointer in a0, otherwise if the zero or not set, loads a long address into a1, moves a constant into d0, and calls another subroutine.
- `update_pointer_and_set_value`: Calls a subroutine, otherwise if the zero flag is not set, moves a constant into offset 34 of the pointer in a0 and moves a long address into offset 2 of the pointer in a0.
- `decrement_and_call_subroutine`: Decrements a byte at offset 34 of the pointer in a0, otherwise if the zero flag is not set, loads a long address into a1, moves a constant into d0, and calls another subroutine.
- `updates_pointer_and_calls_subroutine`: Calls a routine, then conditionally skips a branch to load a new address into a1 and calls another routine.
- `updates_address_and_calls_subroutine`: Loads a long address into a1, moves a constant into d0, and calls another routine, otherwise it returns if a condition is not met.
- `call_subroutine_and_load_address_3`: Calls a subroutine, otherwise if the zero flag is not set, loads a long address into a1, moves a constant into d0, and calls another subroutine.
- `call_subroutine_and_load_address_2`: Calls a subroutine, otherwise if the zero flag is not set, loads a long address into a1 and calls another subroutine.
- `check_flags_and_update_pointer_2`: Checks if bit 6, bit 4, or bit 5 of a byte pointed to by a0 is set; if none are set and a certain byte in a structure pointed to by a6 is zero, it copies a longword from offset 40 of the structure in a0 to offset 40 of the structure in a0 and updates a function pointer at offset 2 of that structure to a new routine, otherwise it returns.
- `check_flags_and_move_pointer`: Checks if bit 6, bit 4, or bit 5 of a byte pointed to by a0 is set, otherwise it moves a longword from offset 40 of the structure in a0 to offset 2 of the structure in a0 and returns.
- `call_struct_increment_and_set_pointer`: Calls a routine, then if the zero flag is not set, loads a pointer from a global address into a1 and calls another routine.
- `call_struct_fields_and_set_flags`: Calls a routine, then if the zero flag is set, loads a pointer from a global address into a1 and calls another routine with the constant value 32803 in d0.
- `call_struct_init_and_set_flags`: Calls a routine, then if the zero flag is set, loads a pointer from a global address into a1 and calls another routine with the constant value 3080 in d0.
- `process_object_list_node`: Calls a routine, then if the zero flag is set, loads a pointer into a1 and calls another routine with a constant value in d0.
- `update_object_counter_and_pointer`: Calls another routine, then if the zero flag is set, writes a 16-bit constant to an offset of 34 from the pointer in a0 and updates a long pointer at an offset of 2 from the pointer in a0.
- `decrement_and_branch_to_table_load`: Decrements a word at an offset of 34 from the pointer in a0, and if the result is not zero, loads a long pointer from a table into a1 and calls another routine; otherwise it returns.
- `execute_conditional_branch_and_load_table`: Calls a routine, then if the zero flag is not set, loads a long pointer from a table into a1 and calls another routine with the constant value 32800.
- `call_routine_and_update_register`: Calls a routine, then if the zero flag is not set, loads a long pointer from a table into a1 and calls another routine with a constant value in d0.
- `call_subroutine_and_update_register_2`: Calls a routine, then if the zero flag is not set, loads a long pointer from a table into a1 and calls another routine.
- `call_subroutine_and_update_register`: Calls a routine, then if the zero flag is not set, loads a long pointer into a1 and calls another routine with a constant value in d0.
- `update_structure_and_branch`: Calls another routine, and if the zero flag is not set, updates a 16-bit value at offset 34 of the structure in a0 to 30 and updates a 32-bit pointer at offset 2 of the structure in a0 to a new address; otherwise it returns.
- `decrement_and_update_table_pointer`: Decrements a byte at an offset of thirty-four from the pointer in a0, and if the result is not zero, loads a pointer to a table into a1 and calls another routine with a value of forty in d0; otherwise it returns.
- `update_counter_and_set_value`: Calls a routine, then if the zero flag is not set, loads a pointer to a table into a1 and calls another routine with the value forty in d0.
- `call_and_reset_object_counter`: Calls another routine, then if the zero flag is not set, writes one hundred to a word at offset thirty-four of the structure in a0 and updates a long pointer at offset two of the structure in a0 with a new address.
- `decrement_and_setup_pointer_2`: Decrements a word at offset 34 of a structure pointed to by a0, then if the zero flag is not unable to be cleared, loads a long pointer into a1 and calls another routine.
- `initialize_pointer_and_call_handler`: Calls a routine, then if the zero flag is not set, loads a long pointer from a constant address into a1, loads a constant word into d0, and calls another routine.
- `call_and_setup_pointer`: Calls a routine, then if the zero flag is not set, loads a long pointer into a1 and calls another routine.
- `call_struct_field_update_and_branch`: Calls a routine, then if the zero flag is not set, branches to a different location, otherwise it loads a new address into a1 and calls another routine with a 16-bit constant in d0.
- `update_structure_flag_and_load_address`: Calls a routine, then if the zero flag is not set, loads a new address into a1 and calls another routine with a 16-bit constant in d0.
- `call_and_update_structure`: Calls a routine, then if the zero flag is not set, moves a word into offset 34 of a structure pointed to by a0 and moves a long constant into offset 2 of that structure.
- `decrement_and_setup_pointer`: Decrements a word at offset 34 of a structure pointed to by a0, then if the zero flag is not set, loads a long pointer into a1 and calls another routine.
- `initialize_pointer_and_call_subroutine`: Calls a routine, then if the zero flag is not set, loads a long pointer into a1 and moves a word (32796) into d0 before calling another routine.
- `call_init_and_setup_pointer`: Calls a routine, then if the zero flag is not set, loads a long pointer into a1 and moves a word into d0 before calling another routine.
- `update_object_state_and_function_pointer`: Decrements a counter and, if not zero, checks if certain flags are clear in a byte pointed to by a0; if flags are clear and a certain global value is zero, it updates a structure's function pointer and copies a longword from an offset or otherwise returns.
- `check_bits_and_update_pointer_2`: Tests bit 6, bit 4, and bit 5 of a byte at a pointer in a0, and if none of those bits are set, copies a longword from offset 28 of a structure in a0 to offset 2 of that same structure, then loads a new address into a1 and calls a routine.
- `update_struct_and_branch_conditionally`: Calls a routine to update structure fields, then if the zero flag is not set, loads a new address into a1 and a constant into d0 before calling another routine.
- `process_struct_flag_and_update_pointer`: Calls a routine to set a flag, then if the zero flag is not set, loads a new address from a global pointer into a1, moves a constant into d0, and calls another routine.
- `update_pointer_and_register`: Calls a routine, then if the zero flag is not set, loads a new address from a global pointer into a1, moves a constant into d0, and calls another routine.
- `execute_conditional_branch_and_setup`: Calls a routine and, if the zero flag is not set, loads a new address into a1, moves a constant into d0, and calls another routine; otherwise it returns.
- `execute_conditional_setup_and_call`: Calls a routine, then if the zero flag is not set, loads a new address into a1, moves a constant into d0, and calls another routine.
- `call_with_setup_address_alt`: Calls a routine, and if the zero flag is not set, loads a new or different address into a1, moves a constant into d0, and calls another routine.
- `execute_conditional_loop_setup`: Calls a routine and, if the zero flag is not set, loads a pointer to a global variable into a1, moves ten into d0, and calls another routine.
- `call_subroutine_and_setup_loop`: Calls a routine, then if the zero flag is not set, loads a pointer to a global variable into a1, moves the constant fifty-two into d0, and calls another routine.
- `call_subroutine_with_conditional_setup_2`: Calls a routine, then if the zero flag is not set, loads a new address into a1, moves the constant 10 into d0, and calls another routine.
- `execute_conditional_subroutine_setup`: Calls a routine and, if the zero flag is not set, loads a new address into a1, moves the constant 50 into d0, and calls another routine.
- `prepare_and_call_routine`: Calls a routine, and if the zero flag is not set, loads a new address into a1, moves a constant into d0, and calls another routine.
- `setup_and_call_routine`: Calls a routine, and if the zero flag is not set, loads a constant into d0 and a new address into a1, then calls another routine.
- `call_routine_and_setup_argument`: Calls a routine, and if the zero flag is not set, loads a long-word address into a1 and moves a constant value into d0 before calling another routine.
- `branch_and_load_pointer`: Calls a routine, and if the zero flag is not set, loads a long pointer into a1 and calls another routine with a constant value in d0.
- `call_subroutine_and_load_address`: Calls a routine, then if the zero flag is not set, loads a long address into a1 before calling another routine.
- `call_subroutine_with_address`: Calls a routine, then if the zero flag is not set, loads a long address into a1 and a word into d0 before calling another routine.
- `call_routine_with_address_and_constant`: Calls a routine, then if the zero flag is not set, loads a long address into a1 and a constant into d0 before calling another routine.
- `load_address_and_set_value`: Calls another routine, and if the zero flag is not set, loads a long address into a1 and moves a constant into d0 before calling another routine.
- `set_pointer_and_value`: Calls a routine, and if the zero flag is not set, writes a word to offset 34 of a structure in a0 and writes a long address into offset 2 of a structure in a0.
- `check_flags_and_load_address_2`: Decrements a 16-bit value at offset 52 of a structure in a0 and, if it reaches zero, updates a pointer at offset 2 with a new address and a pointer at offset 40 with a new address.
- `check_flags_and_load_address`: Checks if bit 6, bit 4, or bit 5 is set in a byte at the pointer in a0; if any are set, it otherwise returns.
- `call_routine_with_address_load`: Calls a routine, then if the zero flag is not set, loads a longword address into a1, and calls another routine.
- `load_address_and_call_3`: Calls a routine, and if the zero flag is not set, loads a longempty address into a1, sets d0 to 32794, and calls another routine.
- `update_pointer_and_set_register`: Calls a routine, then if the zero flag is not set, loads a longword address into a1 and calls another routine with a constant value in d0.
- `update_pointer_and_value_2`: Calls a routine, and if the zero flag is not set, writes 100 to offset 34 of a structure in a0 and writes a longword address into offset 2 of a structure in a0.
- `decrement_and_call_routine_2`: Decrements a byte at offset 34 of a structure in a0, and if the result is not zero, loads a longword address into a1, sets d0 to a constant, and calls another routine.
- `update_structure_member_and_branch`: Calls a routine, then if the zero flag is not set, loads a longword address into a1 and calls another routine with a word value of 40 in d0.
- `execute_conditional_load`: Calls a routine, and if the zero flag is not set, loads a longword address into a1 and calls another routine.
- `execute_conditional_load_and_branch`: Calls a routine, then if the zero flag is not set, branches to a previous instruction, loads a longword address into a1, moves a word into d0, and calls another routine.
- `call_and_load_address`: Calls a routine, then if the zero flag is not set, loads a longword address into a1 and moves a word into d0 before calling another routine.
- `validate_and_update_pointer`: Calls a routine, then if the zero flag is not set, loads a longword address into a1 and calls another routine.
- `validate_and_update_structure_pointer`: Checks if bits 4, 5, and 6 of a pointer's target are clear; if so, it copies a longword from an offset of 4 bytes to an offset of 40 bytes and updates a function pointer at offset 2 of that structure to point to another routine, otherwise it returns.
- `copy_data_on_bitmask`: Checks if certain bits in a pointer's target are clear, then copies a longword from a higher offset to a lower offset.
- `load_address_and_call_routine`: Loads a long address into a1 and calls a routine.
- `update_struct_field_and_set_mask_2_2`: Calls a routine, then if the zero flag is set, loads a pointer to a global variable into a1, sets a mask of 32818 in d0, and calls another routine.
- `update_struct_field_and_set_mask_2`: Calls a routine, then if the zero flag is set, loads a global pointer into a1, sets a mask of 52758 in d0, and calls another routine.
- `update_struct_field_and_set_mask`: Calls a routine to increment a structure field, then if the zero flag is not set, loads a global pointer into a1, sets a mask of 17 in d0, and calls another routine.
- `update_flag_and_set_mask`: Calls a routine to set a flag, then if the zero flag is not set, loads a global pointer into a1, sets a mask of 3840 in d0, and calls another routine.
- `update_struct_value`: Calls a routine, and if the zero flag is not set, writes a word to offset 34 of a structure in a0 and writes a longword to offset 2 of a structure in a0.
- `decrement_and_call_handler`: Decrements a word at offset 34 of a structure, and if it is not zero, loads a longword address into a1 and calls a routine.
- `branch_and_load_address_for_subroutine`: Calls a routine, then if the zero flag is not set, loads a longword address into a1 and moves a constant into d0 before calling another routine.
- `load_address_and_call_4`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 2560, and calls another routine.
- `set_constant_and_call_2`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets a value at a certain address to a constant, and then calls another routine.
- `initialize_pointer_and_call`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 4148, and calls another routine.
- `call_and_initialize_pointer`: Calls another routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to a constant, and calls another routine.
- `initialize_pointer_and_loop_flags`: Calls a routine, then if the zero flag is not set, loads a longword address from a global variable into a1, sets d0 to 2560, and calls another routine.
- `load_pointer_and_loop`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to a certain value, and loops back to a previous instruction if d0 is not zero, then loads a longword address into a1, sets d0 to a certain value, and calls another routine.
- `update_flag_and_load_constant`: Calls a routine to set a flag, then if the zero flag is not set, loads a pointer to a global variable into a1, moves a constant into d0, and calls another routine.
- `update_structure_fields`: Calls a routine, then if the zero flag is not set, writes a word to offset 34 of a structure pointed to by a0 and writes a longword to offset 2 of a structure pointed to by a0.
- `update_object_function_pointer_2`: Decrements a byte at offset 34 of a structure, then if the result is not zero, loads a longword address into a1 and calls another routine.
- `update_object_function_pointer`: Checks if bits 4, 5, and 6 are clear in the byte pointed to by a0; if so, it checks if a certain word in a structure is zero, then copies a longword from offset 40 to offset 2 of that pointer's structure and updates the function pointer at offset 2 to a new routine, otherwise it returns.
- `copy_data_on_bitmask_2`: Checks if bits 4, 5, and 6 are clear in the byte pointed to by a0, then copies a longword from offset 40 to offset 2 of that same pointer's target, otherwise it returns.
- `load_address_and_call_6`: Loads a longword address into a1 and calls a routine.
- `load_address_and_call_5`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 32818, and calls another routine.
- `update_pointer_and_set_register_2`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 3870, and calls another routine.
- `call_and_set_pointer_2`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 17, and calls another routine.
- `decrement_and_set_pointer_4`: Calls another routine, and if the zero flag is not set, writes a 16-bit constant to offset 34 of a structure in a0, and writes a longword address to offset 2 of that structure.
- `decrement_and_load_address`: Decrements a word at offset 34 of a structure in a0, and if not zero, loads a longword address into a1, then calls another routine.
- `call_with_constant`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to a certain value, and calls another routine.
- `update_structure_and_call_handler`: Calls a routine, then if the zero flag is not set, loads a long-word address into a register and calls another routine with a constant value in d0.
- `modify_structure_and_pointer`: Calls a routine, then if the zero flag is not set, writes a word to offset 34 of a structure pointed to by a0 and writes a longand address from a global variable to offset 2 of a structure pointed to by a0.
- `decrement_and_check_structure_byte`: Decrements a byte at offset 34 of the structure in a0 and, if the result is non-zero, loads a pointer from a global variable and calls another routine; otherwise, it returns.
- `check_bitmask_and_update_pointer`: Calls another routine, then checks if bits 4, 5, and 6 of the byte pointed to by a0 are clear; if they are clear and a byte at offset 296 of a6 is zero, it copies a longword from offset 40 of a0 to offset 2 of a0 and sets offset 2 of a0 to a certain address, otherwise it returns.
- `copy_data_on_bitmask_3`: Checks if bits 4, 5, and 6 of the byte pointed to by a0 are clear, then copies a longword from offset 40 to offset 2 of the structure pointed to by a0; otherwise it returns.
- `check_bits_and_copy`: Checks if certain bits in a pointer's target are clear, and if so, copies a longword from a higher offset to a lower offset.
- `copy_data_on_bitmask_4`: Checks if certain certain bits in a pointer's target are clear, then copies a longword from a higher offset to a lower offset.
- `update_stream_state_branch`: Performs a short branch to a target address.
- `set_structure_constants`: Writes the constant an address to offset 2 of a0 and writes the constant an address to offset 28 of a0.
- `update_stream_state`: Checks a value in a structure and, if it does not match a specific constant, processes a byte stream and conditionally decrements a counter and updates another value using a conditional negation-based increment.
- `update_structure_pointers_and_value`: Writes two longwords to a structure in a0 and calls another routine, then masks a bit in d0, increments it, and writes the result to an offset in the structure.
- `update_structure_flag`: Calls a routine, then if a byte at offset 590 of a structure in a6 is not equal to 1, loads a longword address into a2 and calls another routine, then writes a byte to offset 590 of a structure in a6 and branches to another routine.
- `process_amigados_hunk_relocation`: Checks if a structure contains specific AmigaDOS hunk type codes and, if certain conditions are met, decrements a size field and updates a pointer or a secondary offset or otherwise returns immediately.
- `compare_and_branch`: Loads a word from an offset of a0, compares it to a value in a structure, and branches to another routine if the comparison fails or if a certain condition is met.
- `update_structure_fields_3`: Writes a longword address into offset 2 of a structure in a0, writes a longword address into offset 28 of a structure in a0, calls a routine, and then updates a 16-bit value at offset 18 of a structure in a0 with a value derived from d0.
- `process_amigadot_hunk_records`: Checks if a structure contains specific AmigaDOS hunk type codes and, if so, updates a pointer or a counter or modifies a linked list node's callback and flags.
- `update_structure_flags`: Calls a routine, then if the zero flag is not set, clears bit 2 of a byte at offset 0 of a0 and sets a pointer at offset 2 of a0, or otherwise performs a series of checks on a structure, calling a routine to reset a field and potentially updating a value or a pointer or a byte based on certain conditions or a search.
- `check_value_and_return`: Compares d0 to 3 and returns immediately if they are equal, otherwise it returns.
- `check_flag_and_call_routine`: Checks a byte in a global variable and, if it is non-zero, loads a longword address into a2 and calls another routine; otherwise, it returns.
- `initialize_structure_and_branch`: Loads a longword address into a2, calls a routine, and if the zero flag is not set, compares a word at offset 38 of a structure in a0 to a constant, and if certain conditions are met, moves a longword address into offset 2 of a structure in a0, a longword address into offset 28 of a structure in a0, and a word into offset 1134 of a structure in a6.
- `initialize_entity_chain_node`: Initializes a linked list node by setting a callback pointer, flags, and certain data members to fixed values, and then proceeds to recursively initialize or link further nodes in a chain if certain conditions in a structure or a scan routine are met.
- `initialize_entity_chain`: Initializes a linked list of structures by setting callback pointers, flags, and certain data members to fixed values.
- `update_callback_and_timer_bounds`: Updates a structure's timer or offset values using results from two calls to a bit-manipulation routine, clamps certain values within a range, and decrements a counter in a structure, clearing a bit and setting a new function pointer to an empty routine if the counter reaches zero; otherwise, it updates a function pointer and several fields in a separate structure.
- `update_callback_counter`: Decrements a counter in a structure and clears a callback or updates a function pointer if the counter reaches zero.
- `update_callback_and_decrement`: Updates a function pointer and several data fields using a value from a structure, then decrements a counter and potentially updates a global byte and a callback pointer if the counter reaches zero.
- `clear_bit_and_set_status_bit`: Clears bit zero of a byte at offset 665 in a global variable, sets bit zero of a byte pointed to by a6, and then moves a longword constant into a location offset by two bytes from a pointer.
- `update_entity_state_and_bounds`: Checks specific bits in a structure, calls a routine to calculate a new value, and then updates a member at offset 54 or 55 based on certain conditions.
- `decrement_and_update_struct_member`: Decrements a word at offset thirty-four of the structure in a0, and if the result is not zero, it calls a routine to mask and increment a value in d0, then updates a word at offset twenty-two of the structure with the difference between a value from offset six and this new d0 value, and finally updates a pointer in the structure.
- `update_struct_member_with_bounds_check`: Checks if a 16-bit value at offset 6 of a structure is greater than or equal to a certain limit and if subtracting 2 from it remains above a second limit at offset 34; if so, it updates the value at offset 6, increments a counter at offset 32, and calculates a new value for offset 26 based on the counter, otherwise it calls double_and_xor_value and updates a pointer and other structure members before returning.
- `increment_struct_member_with_bounds_check`: Checks if a 16-bit value at offset 6 of a structure is less than a certain limit and if adding 2 to it remains below a second limit at offset 34; if so, it updates the value, decrements a counter at offset 32, and calculates a new value for offset 26 based on the new counter value, otherwise it returns.
- `update_structure_and_flags`: Calls a routine, and if the zero flag is not set, writes d0 to offset 6 of a structure in a0, writes d1 to offset 10 of a structure in a0, writes 3 to offset 62 of a structure in a0, and sets bit 0 of a byte at offset 1 of a structure in a0.
- `set_flags_and_update_pointer`: Sets bit 2 and clears bit 3 of a byte at offset 1 of a structure in a0, then stores a constant 32-bit address into offset 2 of that structure.
- `update_entity_state_and_counter`: Checks a masked value from a function call and, if it falls within a certain range, compares a value from a structure in a6 with a value in a structure in a0, then updates a counter at offset 34 in a0 and updates a function pointer at offset 2 in a0.
- `decrement_counter_and_update_pointer`: Decrements a 16-bit counter at offset 34 of a structure in a0 and, if the counter or a range check fails, otherwise it returns.
- `update_structure_counter_and_pointer_5`: Decrements a 16-bit counter at offset 54 of a structure in a0 and, if the counter or a range check fails, otherwise it returns.
- `update_structure_counter_and_pointer_3`: Decrements a 16-bit counter at offset 34 of a structure in a0 and, if the counter or a range check fails, otherwise it returns.
- `decrement_counter_and_update_index_2`: Decrements a 16-bit counter at offset 54 of a structure in a0 and, if the counter or a range check fails, otherwise it returns.
- `decrement_counter_and_update_index`: Decrements a 16-bit value at offset 34 of a structure in a0 and, if the new value is non-zero and a certain range check passes, increments a 16-bit value at offset 6, decrements a 16-bit value at offset 32, and calculates a new index or offset stored at offset 26 or 26 (via d1) before returning; otherwise, it executes a complex branch or loop involving further pointer updates and conditional logic.
- `update_pointer_and_flag`: Calls add_byte_advance_pointer, then if a certain condition is met, writes a routine address to a pointer offset or sets bit 4 of a byte at offset 1 of a structure in a0.
- `update_structure_with_constants`: Subtracts a 16-bit value at offset 6 of a structure in a0 from a 16-bit value at offset 54 of a structure in a6, and if the result is not negative, writes a byte from a global variable to offset 126 of a structure in a6, then writes a longword address into offset 2 of a structure in a0 and another longword address into offset 28 of a structure in a0.
- `decrement_structure_byte`: Loads 120 into d0, calls another routine, and if the certain bit in d1 is clear, decrements a byte at offset 6 of a structure in a0 before returning.
- `update_function_pointer_and_set_flag`: Sets bit 2 and clears bit 3 of a byte at offset 1 of a structure in a0, then stores a 32-bit address into offset 2 of that structure, and branches to another instruction.
- `set_structure_bits_and_values`: Sets bit 1 or 2 of a byte at offset 1 of a structure in a0, overwrites a word at offset 16 of that structure with a constant, writes a 32-bit constant to offset 2 of that structure, writes a 32-bit constant to offset 28 of that structure, and writes a word to offset 1140 of a structure in a6.
- `clamp_structure_value`: Increments a word at offset 16 of a structure in a0, then checks if the sum of a word from offset 8 and the new value at offset 16 exceeds 130; if so, it sets d0 to 130 and jumps to another routine, otherwise it jumps to another routine.
- `decrement_struct_member`: Decrements a word at offset 6 of a structure in a0 and copies a word to offset 8 of that structure.
- `update_structure_with_constant_values`: Subtracts a byte loaded from a global variable from a byte at offset sixty-two of a structure in a0, and if the result is not negative, writes a word and a longword to offsets or certain offsets of that structure and a global variable, otherwise it returns.
- `increment_structure_byte`: Loads 120 into d0, calls another routine, and if the certain bit in d1 is clear, increments a byte at offset 6 of a structure in a0 before returning.
- `update_function_pointer_and_set_flag_2`: Sets bit 2 and clears bit 3 of a byte at offset 1 of a structure in a0, then stores a 32-bit address into offset 2 of that structure, and branches to another routine.
- `set_structure_members_2`: Sets bit 2 of a byte at offset 1 of a structure in a0, writes a word to offset 16 of a structure in a0, writes a longword to offset 2 of a structure in a0, writes a longword to offset 28 of a structure in a0, and writes a word to a location in a6.
- `clamp_struct_member_value`: Increments a word at offset 16 of a structure in a0, then checks if the sum of that word and the word at offset 8 exceeds 130; if so, it sets d0 to 130, otherwise it sets d0 to the new sum, and then branches to another routine.
- `increment_struct_member`: Increments a word at offset 6 of a structure in a0 and copies a word to offset 8 of that structure.
- `modify_memory_with_bit_or`: Performs a series of bitwise OR operations using immediate values on bytes located at addresses held in a0, a1, a2, a3, and a5, and performs a negation-based modification on the value at a4.
- `modify_memory_with_constants`: Writes a series of constant byte values to memory locations pointed to by a6, a0, a1, a2, and a3, while also clearing the d0 register.
- `test_bits_and_set_byte`: Tests bits 4 and 5 of d0 and sets bit 1 of d2.
- `write_sequential_bytes`: Writes a series of incrementing or decrementing bytes to memory using pointers in a5, a6, and a7, and writes bytes to memory using a0.
- `test_bits_and_set_byte_2`: Tests bits 4 and 5 of d0 and sets bit 2 of d6.
- `update_structure_counter_and_branch_5`: Decrements a 16-bit counter at offset 34 of a structure and updates a function pointer at offset 2 of that structure to point to another routine, otherwise it returns.
- `update_structure_counter_and_branch_3`: Checks specific bits in a structure, and if certain conditions are met, calls a routine to calculate a new value, updates a 16-bit counter at offset 34, and potentially updates a function pointer or a secondary counter at offset 6 before calling itself or another routine.
- `decrement_counter_and_branch`: Decrements a 16-bit value at offset 34 of a structure in a0, and if the result is zero, loads a constant address into a1, moves 42 into d0, calls another routine, and returns; otherwise, it stores a constant 32-bit address into offset 28 of the structure and stores another constant 32-bit address into offset 2 of the structure.
- `update_structure_counter_and_branch`: Decrements a 16-bit value at offset 34 of a structure and, if not zero, calls a routine to read a byte and advance a pointer; otherwise, it updates a counter or pointer-based value using a calculation involving a call to double_and_xor_value, and may update a function pointer at offset 2 or clear a byte at offset 32 before potentially calling itself or returning.
- `update_counter_with_bounds_range`: Checks if a 16-bit value at offset 6 of a structure is within a range defined by an offset 20 and offset 34; if so, it decrements the value by 2, updates the value, calls another routine, and calls advance_past_negative_bytes before returning; otherwise, it updates a different offset in the structure, calls double_and_xor_value, and enters a loop that modifies structure offsets and updates a function pointer.
- `update_counter_with_bounds`: Compares a word from offset 6 of a structure in a0 to a word at offset 16, and if the value is within a certain range, increments it and calls another routine, then loads a pointer and calls another routine before returning.
- `increment_or_decrement_counter`: Increments a 16-bit value at an offset in a structure if it is below 20, otherwise decrements it if it is below 40.
- `update_status_and_data`: Calls another routine, and if the zero flag is not set, writes a word from d0 to offset 6 of a structure in a0, writes a word from d1 to offset 10 of a structure in a0, and sets bit 0 of a byte at offset 1 of a structure in a0.
- `modify_entity_status_and_pointer`: Sets bit 2 and clears bit 3 of a byte at offset 1 of the structure in a0, then overwrites a longword at offset 2 of the structure with a constant value.
- `update_entity_chain_and_pointer`: Checks a bitmask or type-identifier, then either updates a pointer at offset 26 or 34 of a structure in a0 with a new address or a value derived from a counter, or modifies a 16-bit counter at offset 54 of a structure in a6, and calls double_and_xor_value.
- `update_entity_lifecycle_and_pointer_2`: Decrements a 16-bit counter at offset 34 of a structure in a0 and, if the counter reaches zero, modifies a status byte at offset 1, updates a pointer at offset 2, and calculates a new counter value using a call to double_and_xor_value before potentially traversing a linked list.
- `update_entity_lifecycle_and_pointer`: Decrements a 16-bit counter at offset 34 of a structure in a0 and, if not zero, checks a limit at offset 64; if certain conditions involving a comparison with a value from a global variable are met, it updates a pointer at offset 26 or modifies a status byte and a pointer at offset 2 or 6.
- `update_entity_counter_and_pointer`: Decrements a 16-bit counter at offset 34 of a structure in a0 and, if not zero, checks a limit at offset 64; if certain conditions involving a comparison with a value from a6 are met, it increments or decrements a 16-bit value at offset 10 of the structure, updates a pointer or a value at offset 26, and may call double_and_xor_value or update a function pointer at offset 2.
- `process_entity_chain_and_update_pointer`: Decrements a 16-bit counter at offset 54 of a structure in a0 and, if not zero, updates a pointer at offset 26 or 34 with a value derived from a bit-masked result of a call to double_and_xor_value.
- `increment_and_set_flag`: Increments a word at offset 34 of a structure in a6, and if that word equals 22, sets bit 4 of a byte at offset 1 of the structure.
- `update_pointer_and_value`: Reads a byte from a pointer, adds it to a value in a structure, updates a different value using a byte from the pointer plus a constant, and advances the pointer by two bytes.
- `update_structure_counter_and_pointer_4`: Updates a structure's flags and counters by calling another routine, then checks a value against a constant and either updates a 16-bit counter at offset 34 or modifies a 16-bit value at offset 26 and a 16-bit counter at offset 26 based on a sign check of a value at offset 36.
- `decrement_and_update_counter`: Decrements a 16-bit counter at offset 34 of the structure in a0, and if the counter reaches zero, it writes a byte to offset 24 of the structure, calls another routine, masks and increments a register, updates the counter, and overwrites a pointer at offset 2 and a value at offset 28 of the structure; otherwise it returns.
- `update_structure_counter_and_pointer_2_2`: Increments or decrements a 16-bit counter at offset 10 of a structure in a0 if it does not equal a value in a6, then checks a counter at offset 34 and potentially calls read_byte_advance_pointer or double_and_xor_value while updating structure pointers and counters.
- `update_structure_counter_and_pointer_2`: Updates a 16-bit counter at offset 8 of a structure using a value derived from a masked byte, decrements a 16-bit value at offset 6 of the structure, and conditionally sets bit 4 of a byte at offset 1 of the structure before returning.
- `update_structure_counter_and_pointer`: Checks if a 16-bit value at offset 64 of a structure in a0 exceeds 399, then increments or decrements a 16-bit counter at offset 10 of that structure if it does not match a value in a6.
- `modify_structure_offset`: Loads 184 into d0, calls another routine, and then masks d0, subtracts 16, and adds the result to a word at offset 8 of a structure in a0.
- `update_structure_state_and_pointer`: Updates a structure's position and flags, then calls a routine to calculate a new offset and updates a function pointer and a secondary offset or pointer in the structure.
- `update_structure_bounds_and_set_flag_3`: Adjusts a 16-bit position or offset in a structure based on a comparison with a target value, updates a secondary offset using a bit-shifted value, and potentially updates a function pointer and a status bit or counter if certain boundary or value conditions are met.
- `update_structure_position_and_pointer`: Adjusts a 16-bit position or offset in a structure based on a comparison with a target value, updates a secondary offset using a bit-shifted value, and calls a routine to read a byte and advance a pointer if certain boundary conditions or pointer-range checks are met; otherwise, it modifies a status byte and updates a function pointer or a counter.
- `update_structure_bounds_and_set_flag_2`: Increments a 16-bit value at offset 8 of a structure in a0 by 2 or 1 depending on its current value, and if a certain condition involving a value in a6 is met, decrements a 16-bit value at offset 6, updates a 16-bit value at offset 26, and sets bit 4 of a byte at offset 1, otherwise it returns.
- `update_structure_bounds_and_set_flag`: Checks if a 16-bit value at offset 8 of a structure in a0 is within certain bounds, incrementing it by 2 or 1 if certain conditions are met, and if a secondary comparison against a value in a6 passes, increments a value at offset 6 of the structure, writes a constant to offset 26, and sets bit 4 of a byte at offset 1, otherwise it returns.
- `update_counter_and_pointer`: Sets bit 0 of a byte at offset 0 of a structure in a0, compares a 16-bit value at offset 12 of a structure in a6 to a 16-bit value at offset 12 of a structure in a0, and if not equal, checks if a 16-bit value at offset 64 of a structure in a certain register is between 38 and 160, and if so, writes a longword address into offset 2 of a structure in a0 and another longword address into offset 28 of a structure in a0.
- `update_recursive_counter_and_status`: Compares a 16-bit value at offset 6 of a structure in a0 to a value at offset 22, then if the first is smaller, increments that value by 2 and recursively calls itself while updating a 16-bit counter at offset 10 and potentially updating a function pointer and a global variable or status byte.
- `increment_and_clamp_counter`: Increments a 16-bit value at offset 10 of a structure in a0 by 2 and clamps it to 59 if it reaches or exceeds 60, then updates a function pointer at offset 2 of the structure.
- `decrement_structure_counter_recursively`: Decrements a 16-bit value at offset 10 of a structure in a0, clearing it to zero if it reaches a certain threshold, and recursively calls itself or updates a function pointer at offset 2 of the structure.
- `process_recursive_structure_update`: Decrements a 16-bit value at offset 6 of a structure in a0 and recursively calls itself, then updates a 16-bit value at offset 10 of that structure based on the return value.
- `set_status_and_update_pointer_2`: Sets bit zero of a byte at offset sixty-eight in a6, calls another routine, and if the zero flag is clear, checks if a value at offset twenty-two in a0 is less than three, then overwrites a pointer at offset two in a0 with a constant and a pointer at offset twenty-eight in a0 with another constant.
- `set_status_and_update_pointer`: Sets bit zero of a byte at offset sixty-eight in a6 and calls a routine to advance a pointer, then either updates a function pointer or a status bit in a structure at a0 based on a bit test in d1.
- `update_status_and_increment_counter`: Sets bit zero of a byte at offset sixty-eight in a6, calls a routine to advance a pointer, and then either recursively calls another routine or increments a 16-bit value at offset ten of a structure in a0 and potentially clamps it to a maximum value before returning.
- `update_counter_and_branch_function`: Sets bit zero of a byte at offset sixty-eight in a6, calls a routine to advance a pointer, and then either recursively calls another routine or decrements a word-sized counter at offset ten in a structure and potentially updates a function pointer in that structure's table.
- `clear_bits_and_set_pointer`: Clears the least significant bit of a byte, sets two bytes at an offset to specific values, and stores a pointer to a global variable into a structure.
- `compare_and_update_status`: Calculates the difference between a value in a structure and a value in another structure, then potentially clears or increments a byte in a structure based on the result.
- `modify_byte_with_conditional_call`: Clears a byte in a structure, then checks a bit in a byte and either sets another bit or clears a bit, potentially calling another routine if certain conditions are met.
- `update_entity_and_pointer`: Calls another routine, and if the zero flag is not set, writes a word from d0 to offset 6 of a structure in a0, writes a word from d1 to offset 10 of a structure in a0, loads a pointer from offset 36 of a structure in a0 into a1, decrements the byte at the address pointed to by a1 minus 1 and stores it back, and sets bit 0 of a byte at offset 1 of a structure in a0.
- `modify_status_and_set_pointer`: Sets bit two of a byte at offset one in a0, clears bit three of a byte at offset one in a0, and then overwrites a pointer at offset two in a0 with a constant.
- `update_entity_state_and_function_pointer`: Checks a masked value from a function call and, if certain range or pointer conditions are met, updates a value in a structure, modifies a status byte, and updates a function pointer or a pointer-to-function in the structure.
- `decrement_and_update_entity_state`: Decrements a 16-bit counter at offset 52 of a structure and returns if the result is non-zero; otherwise, it modifies a status byte at offset 1, updates a function pointer, and calls a routine to calculate a new value, potentially updating a distance-related counter and a pointer-based state or function pointer if certain range or distance conditions are met.
- `update_object_distance_and_state`: Checks a flag and a counter in a structure, then conditionally increments or decrements a distance-related value in a0 based on a comparison with a global value.
- `update_object_state_and_pointer`: Checks a flag and a counter in a structure, then conditionally increments or decrements a counter or pointer-related value based on a comparison with a target value.
- `decrement_and_traverse_structure`: Decrements a counter in a structure and, if certain range or pointer conditions are met, updates a function pointer, modifies a status byte, and calls another routine or recursively calls itself via a loop or a pointer-advancing routine.
- `update_conditional_pointer`: Checks if a value at a pointer offset is negative, increments a pointer, and conditionally updates a pointer or sets a bit in a byte, then returns.
- `update_node_status_and_pointer_2`: Subtracts a value from a global variable and, if the result is not negative, subtracts a byte from a structure offset and updates a pointer and a value in a structure if a certain condition is met; otherwise it returns.
- `update_node_status_and_pointer`: Checks if a value pointed to by a pointer in a structure is negative; if not, it checks a bit in a status register, potentially decrements a 16-bit value at offset 6 of the structure, and then sets bit 3 and clears bit 2 of a byte at offset 1 of the structure, overwrites a pointer at offset 2 with a new address, and updates a pointer in a secondary structure.
- `clamp_and_update_node_value`: Increments a 16-bit value at offset 16 of a structure in a0, clamps the sum of that value and a value at offset 8 to a maximum of 130, and updates offset 8 with this result.
- `update_node_value_and_relink`: Increments a 16-bit value at offset 16 of a structure in a0 if a certain bit in d1 is set, otherwise it modifies a status byte at offset 1 of the structure, overwrites a pointer at offset 2 with a new address, and updates a pointer at offset 28 of a secondary structure or table.
- `update_node_value`: Increments a 16-bit value at offset 16 of a structure in a0, and if the value or its sum with a value at offset 8 is not less than 131, sets the value at offset 8 to 130, then updates a pointer and calls a routine to add a byte and advance a pointer, otherwise it sets a value at offset 34 and a function pointer at offset 2.
- `traverse_and_insert_node`: Traverses a linked list, checking if a short value at offset 4 of each node is between 8 and 15, then calls double_and_xor_value and insert_ordered_node to insert a cloned node if a certain condition is met.
- `toggle_bit_and_decrement_counter`: Toggles bit 2 of a byte at offset 1 of a structure in a0, decrements a 16-bit value at offset 34 of that structure, and if the result is zero, clears bit 0 and bit 2 of the byte at offset 1 and overwrites a pointer at offset 2 with a new address before returning.
- `set_pointer_and_value_2`: Writes a new address into offset 2 of a structure in a0 and writes a constant into offset 648 of a structure in a6 before returning.
- `validate_and_set_structure`: Calls a routine, and if bit 3 of a byte in a structure in a0 is clear, checks a value from a structure in a6 against a value in a structure in a0, and if certain, writes a byte to offset 24 of a structure in a0 and updates pointers at offset 2 and offset 28 of a structure in a0 before returning.
- `adjust_structure_value_on_condition`: Calls two different routines, then checks if a value at an offset in a structure in a0 is equal to a value in a register; if not, it checks if the value is less than a certain amount and increments that value in the structure by one before otherwise returning.
- `update_structure_and_read_byte`: Decrements a value at offset 10 of a structure in a0 and offset 6 of a structure in a0, calls a routine to read a byte and advance a pointer, and then performs several checks and conditional writes to a structure in a0, including setting a byte at offset 24 to a constant and updating pointers at offset 2 and offset 28.
- `branch_and_update_counter`: Calls a routine, and if the zero flag is clear, calls another routine, then subtracts a value from a register, and if the result is certain, increments a value at offset 10 of a structure in a0, then loops back to a previous instruction.
- `update_buffer_and_check_bounds`: Decrements a word at offset 10 of a0, increments a word at offset 6 of a0, calls a routine to read a byte and advance a pointer, and then checks if a certain bit is set or if a value at offset 64 of a0 is less than or equal to a value from a global variable.
- `check_and_set_byte`: Loads a word from offset 64 of a structure in a0, checks if it is greater than 24, and if not, loads a word from offset 60 of a6, subtracts a word from offset 10 of a0, adds 3, checks if it is less than 7, and if not, writes 255 to offset 24 of a0, then writes a certain address to offset 2 of a0 and another address to offset 28 of a0.
- `increment_pointer_only`: Increments a pointer and checks a condition that is always false, resulting in no further side effects.
- `set_pointer_to_address`: Sets a pointer at an offset of a structure in a6 to a specific address and returns.
- `check_and_set_byte_alt`: Loads a word from offset 64 of a0, checks if it is greater than 24, and if not, loads a word from offset 60 of a6, subtracts a word from offset 10 of a0, adds 3, checks if it is less than 7, and if not, writes 1 to offset 24 of a0, then writes a certain address to offset 2 of a0 and another address to offset 28 of a0.
- `update_pointer_and_set_bit`: Calls a routine to advance a pointer, then conditionally updates a pointer-sized entry in a structure and sets bit 4 of a byte at offset 1 of that structure if bit 6 of a register is set, otherwise it returns.
- `update_bit_and_call`: Checks bit 3 of a byte, and if set, sets bit 2 of that byte, loads an address into a2, and calls another routine before returning.
- `clear_bit_in_byte`: Clears bit 2 of a byte pointed to by a0.
- `validate_and_update_structure`: Compares a value from a structure against a value in a table, and if they match, checks if a value in a structure minus a constant is within a certain range, then potentially updates a value in a structure, clears a bit, or calls a routine to reset a field or update a pointer.
- `search_and_update_pointer`: Calls a routine, and if the zero flag is not set, loads a new address into a2, moves a constant into d0, calls a routine, and if the zero flag is not set, loads a new address into a1, moves a value from a structure into d0, and calls another routine.
- `update_structure_pointers_3`: Calls a routine, and if the zero flag is not set, writes a constant to an offset of a structure and writes a new address to another offset of that structure.
- `decrement_node_counter_and_insert`: Decrements a 16-bit value at offset 54 of a structure in a0, and if the result is zero, searches for a matching node, inserts a new node into a list, and updates a function pointer at offset 2 of the structure.
- `update_structure_and_call`: Decrements a byte at offset 34 of a structure in a0, otherwise if the zero flag is not set, clears bit 2 of a byte at offset 1 of a structure, sets bit 1 of a byte at offset 1 of a structure, clears bit 7 of a byte at offset 1 of a structure, loads a long address into a1 and calls a routine.
- `execute_conditional_load_and_call`: Calls a routine, then if the zero flag is not set, loads a long address from a global variable into a1 and a constant word into d0 before calling another routine.
- `load_address_and_call_with_constant`: Calls a routine, then if the zero flag is not set, loads a long address into a1 and a word into d0 before calling another or another routine.
- `load_address_and_call_7`: Calls another routine, and if the zero flag is not unable to be determined, loads a long address into a1 before calling another routine.
- `update_stream_status_and_pointer`: Calls a routine, and if the zero flag is not set, loads a long address into a1 and calls another routine with a constant value in d0.
- `branch_on_stream_status`: Calls a routine, and if the zero flag is not set, loads a long address into a1 and calls another routine; otherwise it returns.
- `process_stream_pointer`: Calls a routine, and if the zero flag is not set, loads a long address into a1 and calls another routine, while also loading a word into d0.
- `call_conditional_handler`: Calls a routine, then if the zero flag is not set, loads a long address into a1 and calls another routine.
- `execute_conditional_handler`: Calls a routine and, if the zero flag is not set, loads a long address into a1 and calls another routine with a constant value in d0.
- `initialize_structure_and_call_handler`: Calls a routine, then if the zero flag is not set, loads a long address into a1 and calls another routine with a constant value in d0.
- `calls_routine_and_conditionally_branches`: Calls a routine, then if the zero flag is not set, branches to a different instruction, otherwise loads a longword address into a1 and calls another routine.
- `call_conditional_pointer_load`: Calls a routine, and if the zero flag is not set, loads a longword address into a1 and then calls another routine.
- `call_conditional_branch_and_load_pointer`: Calls a routine and, if the zero flag is not set, loads a longword address into a1 and calls another routine.
- `load_address_and_call`: Calls another routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 32798, and calls another routine.
- `call_decrement_and_load_address`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, then calls another routine.
- `process_list_element_and_load_address`: Calls another routine, and if the zero flag is not set, loads a constant address into a1, moves a constant into d0, and calls another routine.
- `initialize_list_and_load_address`: Calls a routine, and if the zero flag is not set, loads a constant address into a1, moves a constant into d0, and calls another routine.
- `insert_node_into_list`: Calls a routine to increment a value, and if the zero flag is cleared, searches for a matching node, updates several fields in a structure at a0, calls a routine to insert an ordered node, and modifies flags and a pointer in that structure.
- `decrement_and_set_pointer_2`: Decrements a word at offset 34 of a0, and if the result is not zero, loads a certain address into a1, loads 7662 into d0, and calls another routine.
- `decrement_and_set_pointer`: Calls a routine, and if the result is not zero, sets a pointer at an offset of a0 to a specific address before returning.
- `update_node_flags`: Checks if a certain bit is set in a structure, and if so, sets another bit in that structure, potentially clearing a bit or calling a routine to reset a field if certain global or structure conditions are met, otherwise it clears a bit in the structure and returns.
- `check_d0_value`: Checks a byte in a global variable, and if it is not certain, checks another byte in a global variable, then compares d0 against certain constants and returns or branches.
- `reorder_linked_list_node`: Checks a counter in a structure, verifies a value in a linked list, and if a certain condition is met, swaps a node or inserts a new node into a list before calling count_and_insert_list and reset_field_if_value.
- `process_list_and_reset`: Saves registers, calls a routine, and if the result is greater than 3, calls a routine to count a list, updates a value in a6, calls another routine, and then calls a routine to reset a field before returning.
- `call_reset_routine`: Loads an address into a2 and calls another routine.
- `traverse_buffer_and_call`: Checks a byte in a global variable, and if it is not certain, loads a new address into a1 and a new address into a2, then calls a routine.
- `load_address_and_call_routine_2`: Loads a long-word address into a2 and calls a routine, then returns.
- `call_subroutine_and_update_status`: Calls a routine, then if the zero flag is not set, branches to another routine; otherwise, it loads a long-word address into a1 and calls another routine with a constant value in d0.
- `update_pointer_and_set_flag_2`: Calls a routine, then if the zero flag is not set, branches to another routine; otherwise, it loads a long-word address into a1, sets a 16-bit value in d0, and calls another routine.
- `update_pointer_and_set_flag`: Calls a routine, then conditionally branches to a different routine, otherwise loads a long-word address into a1, sets a 16-bit value in d0 to 3984, and calls another routine.
- `call_with_address`: Calls another routine, and if the zero flag is not set, loads a long address into a1 before calling another routine.
- `setup_struct_and_call`: Calls a routine, and if the zero unable to be determined, loads a long address into a1, loads a word into d0, and calls another routine.
- `execute_conditional_branch`: Calls a routine, and if the zero flag is not set, loads a long address into a1 and calls another routine.
- `execute_with_parameter`: Calls a routine, then if the zero flag is not set, loads a long address into a1, sets d0 to a constant, and calls another routine.
- `update_flag_and_load_pointer`: Calls a routine, then conditionally branches to a following instruction, loads a pointer from a global variable into a register, loads a constant into a register, and calls another routine.
- `update_struct_and_branch`: Calls a routine, then conditionally branches to another routine; otherwise, it loads a pointer from a global variable into a register, loads a constant into a register, and calls another routine.
- `updates_pointer_and_checks_status`: Calls a routine, then conditionally branches to another routine if the zero flag is set, otherwise loads a pointer from a global variable into a1 and calls a routine that updates a structure member and flag.
- `validate_status_and_initialize_pointer`: Calls another routine and, if the zero flag is not set, branches to a different location; otherwise, it loads a pointer into a1, moves the constant 32798 into d0, and calls another routine.
- `check_bits_and_update_pointer`: Calls a routine, and if the zero flag is not set, jumps to another routine, otherwise it jumps to a third routine, tests bits 6, 4, and 5 of a byte at offset 0 of a structure in a0, and if any bit is set, returns; otherwise it copies a longword from offset 2 of the structure to offset 40 of the structure and overwrites a pointer at offset 2 with a new address.
- `check_structure_bits_and_update_pointer`: Tests bits 6, 4, and 5 of a byte at offset 0 in a structure in a0; if any bit is set, it returns, otherwise it copies a longword from offset 40 to offset 2, sets bit 1 of a byte at offset 2, writes a constant longword to offset 2, and writes a constant word to offset 34.
- `update_structure_byte_and_pointer`: Decrements a byte at offset 34 of a structure in a0, and if not zero, clears bit 2 of a byte at offset 1 of a structure in a0, then otherwise clears bit 2 of a byte at offset 1 of a structure in a0 and moves a longword address into offset 2 of a structure in a0.
- `update_structure_fields_2`: Compares a word at offset 262 in a6 to a constant, and if not lower, returns; otherwise it clears bit 1 of a byte at offset 0 in a0, and writes a constant address to offset 2 of a0 and a constant word to offset 34 of a0.
- `update_status_and_pointer`: Decrements a word at offset thirty-four in a0, and if the zero flag is not clear, clears bit two of a byte at offset one in a0, tests bit zero of a byte at offset thirty-five in a0, and if the zero flag is clear, sets bit two of a byte at offset one in a0, then returns or overwrites a pointer at offset two in a0 with a constant.
- `calculate_complex_offset`: Checks a global variable, and if a certain condition is met, performs several arithmetic operations involving bit shifts and swaps on values in a structure at a0, and calls a routine to double and xor a value before updating a field in that structure.
- `decrement_and_branch`: Decrements a word at offset 34 of a0, and if the result is not zero, otherwise it returns; if the result is zero, it writes a certain address to offset 28 of a0, writes another address to offset 2 of a0, and writes a byte to offset 561 of a6.
- `check_and_decrement_value`: Calls a routine, and if the result is not negative, checks if a value at an offset of a0 is negative, and if so, calls another routine and conditionally decrements a value at an offset of a0.
- `clamp_and_adjust_handler`: Compares a counter at one structure with a value at another, then stores a handler pointer and adjusts a stored counter toward a target.
- `store_self_address_into_struct_2`: Stores its own return address into a field of the structure passed in the register, then stores a different global variable's address into another field.
- `clamp_and_store_handler_2`: Clamps a stored counter toward a target value by nudging it up or down by a fixed amount, then stores a global variable reference and a handler pointer into a structure.
- `store_pointer_at_offset`: Writes the address of this routine into a pointer field located two bytes past a base address held in a register.
- `clamp_and_store_handler`: Clamps a stored counter into a fixed range by nudging it toward the bounds, then stores a global variable reference and a handler pointer at fixed offsets in a structure.
- `adjust_linked_list_range_2`: Walks a doubly linked list of records, comparing a stored counter against a fixed range and adjusting each record's index by a signed amount until the counter leaves that range.
- `configure_range_handler`: Initializes a data structure by storing a global variable reference and a handler pointer at fixed offsets.
- `store_handler_in_range`: Invokes another routine, then compares a structure value against a range and stores a handler address into a pointer field when the value falls within that range.
- `store_self_address_into_struct`: Stores its own address into a field of the structure passed in the register, then stores a different global variable's address into another field of that same structure.
- `store_pointer_when_outside_range`: Compares a value from one structure against a range defined by another structure and stores its own address into a nearby pointer field only when the value falls outside that range.
- `store_self_pointer`: Stores a pointer to itself at a fixed offset of a structure pointed to by a register and then returns.
- `clamp_and_adjust_word`: Reads a signed word at a fixed structure offset, increments it when below an upper bound, decrements it when above a higher bound, and leaves it unchanged in between.
- `increment_structure_word`: Reads a word at a fixed offset in a structure, increments it by one, and writes the result back.
- `destructure_word_decrement`: Reads a word at a fixed offset in a structure, decrements it by one, and writes the result back.
- `adjust_linked_list_range`: Walks a doubly linked list of records, comparing a stored marker against a value and adjusting each record's index by a signed amount.
- `empty_return_routine_3`: This routine contains no instructions; it is an empty subroutine that returns immediately without touching any registers, memory, or hardware.
- `patch_pointer_offset`: Writes the address of a routine into the word located two bytes past the pointer held in A0, overwriting whatever was stored there.
- `adjust_linked_list_indices`: Walks a linked list of records, comparing a stored marker against a value and adjusting each record's index by a signed table.
- `adjust_structure_by_signed_table`: Compares a stored marker against a value and, when they match, walks a table of signed bytes to accumulate an index until a non-negative entry is reached.
- `set_vector_and_return`: Installs a jump vector at a fixed offset from a register and returns.
- `adjust_signed_byte_counter`: Compares a stored marker against a counter and, when it does not match, decrements the counter and recurses.
- `process_signed_byte_table`: Compares a stored marker against a counter and, when it does not match, decrements the counter and recurses.
- `conditionally_adjust_structure_2`: It compares a byte against a stored marker, then adjusts a counter field and a related value depending on whether the counter is above or below a threshold.
- `conditionally_adjust_structure`: Compares a passed value against a field in a structure and, when the difference falls within a small range, decrements a word in another structure and stores a byte value of fifty-nine at a nearby offset.
- `adjust_and_maybe_increment`: Increments a structure word when below a threshold, otherwise compares a register value against a global table entry and, when the result fits a small range, increments another field and clears a neighboring word.
- `check_range_increment_struct`: Compares a value from a global table against a register value, adding two and checking whether the result fits in a small range.
- `shift_right_return_value`: Reads a short value from a fixed offset in a structure and shifts it right by three bits, returning the scaled-down result.
- `copy_and_adjust_struct`: Copies three fields from a source structure into a destination structure, adding eight to one value and subtracting one from another, and records a pointer to itself with a flag byte when a stored counter is negative.
- `copy_and_invoke_routine`: Copies several values from a source block into a destination structure, stores a return address when a flag byte is positive, and then invokes another routine.
- `indirect_branch_from_table`: It loads a word from a table slot and branches to it, effectively dispatching to a handler routine.
- `patch_table_slots`: It sets several flag bits in a block of memory pointed to by registers and then writes the address of this routine into two slots in a table, repeating the writes until a byte at a fixed offset in a global variable becomes zero.
- `patch_and_reinstall_entry`: It sets specific bits in three bytes at two memory locations, then stores its own starting address into two fixed offsets of a table that begins two bytes after the first location.
- `install_self_pointer_loop`: Repeatedly clears a byte at a fixed offset within a block of memory pointed to by a register, then installs its own entry point at two fixed offsets within a structure pointed to by another register.
- `loop_until_complete`: Loops calling another routine until it signals completion, storing return addresses and clearing a flag byte while a tracked table byte remains nonzero.
- `reset_handler_entry`: Resets a handler entry by clearing a specific bit in a flag byte and installing a replacement handler address.
- `install_handler_with_checks_4`: Runs a setup routine and, depending on its result, branches to either a second setup routine, then inspects a byte flag field and either copies a four-byte value between two structure fields while installing a handler address or clears a flag bit and installs a handler address, looping back to the initial setup when a structure counter reaches a threshold.
- `install_handler_with_checks_3`: Invokes another routine, then based on a status field and flag bits in a structure copies a four-byte value between two fields and stores a handler address, setting or clearing a flag bit as needed.
- `install_handler_with_checks_2`: Repeatedly invokes another routine and, based on a status field and flag bits in a structure, copies a four-byte value between two fields and stores a handler address, setting or clearing a flag bit as needed.
- `wait_and_install_handler`: Waits for a status byte to clear its busy bits, then installs a handler into a structure field and runs the wait routine twice.
- `wait_and_update_status`: Waits for a hardware status byte to reach a ready state, setting and clearing a flag bit while a counter counts down.
- `update_status_struct_2`: It polls a status byte in hardware registers, clearing or setting a flag bit and looping until a counter reaches a limit, invoking a routine on each pass.
- `update_status_struct`: It polls a status byte located in hardware registers and returns early if a counter is already high.
- `set_flag_on_range_match`: Checks if a value in a structure matches a value in another structure and if a certain range is valid, then sets or clears a byte in a structure.
- `update_accumulated_values_with_scaling`: Checks a flag in a structure, and if non-zero, updates a linked list node's flags and pointers, calculates a scaled value using a constant multiplier, and updates several global or structure-based values before potentially updating a global counter and adjusting certain values or resetting a flag if a certain condition is met; otherwise it returns immediately.
- `update_accumulated_values`: Checks a value in a structure, and if not a specific constant, updates several global and structure-based values using arithmetic, otherwise it clears a bit, clears a byte, and sets several values before returning.
- `set_pointer_on_non_zero`: Tests a word at offset 658 in a6, and if not zero, writes a constant address to offset 2 of a0; otherwise it returns immediately.
- `update_structure_and_status`: Decrements a word at offset seven-hundred-fifty-four in a6, and if the zero flag is not clear, calls another routine, and if the negative flag is clear, overwrites a pointer at offset twenty-eight in a0 with a constant, overwrites a pointer at offset two in a0 with a constant, and moves a constant into a byte at offset one-thousand-one-hundred-thirty-four in a6.
- `set_pointer_and_branch`: Loads a constant into d0, calls a routine, and if the negative flag is set, sets bit 7 of a byte at offset 754 of a6 and then updates a pointer at offset 2 of a0.
- `set_pointer_and_decrement`: Sets bit 4 of a byte at offset 1 of a0, decrements a word at offset 754 of a6, and if the result is not zero, otherwise it returns; if the result is zero, writes a certain address to offset 28 of a0 and another address to offset 2 of a0.
- `set_pointer_if_true`: Increments a pointer and, if a condition is met, updates a pointer with the address of this routine.
- `set_byte_on_condition`: Sets a byte in a structure to a specific value if a certain condition is met, and always sets a byte at a different offset to a constant value.
- `set_bit_and_update_structure`: Sets bit 4 of a byte at offset 1 of a structure in a0 and returns; otherwise, it calls a routine, compares a word at offset 6 of a structure in a0 to a constant, and if certain conditions are met, updates a word at offset 34 of a structure in a0, a longword at offset 28 of a structure in a0, and a longword at offset 2 of a structure in a0.
- `update_structure_counter_and_branch_4`: Checks a bitmask in a structure, and if certain bits are clear, increments or decrements a 16-bit field at offset 10 or updates a 16-bit field at offset 34 based on a value, then calls a routine to read a byte and advance a pointer.
- `update_structure_counter_and_branch_2_2`: Calculates a new value based on a masked bit, updates a 16-bit field at offset 34 of a structure, and overwrites a function pointer and a global variable pointer at offsets 2 and 28 of that structure, then loops or returns based on a comparison of a 16-bit value at offset 6.
- `update_structure_counter_and_branch_2`: Checks a bitmask in a structure, and if certain bits are clear, increments a counter or updates a value in the structure based on a comparison, otherwise it calls a routine to read a byte and advance a pointer.
- `decrement_and_update_structure`: Decrements a 16-bit value in a structure and, if it reaches a certain threshold, clears a bit in a byte, calls a routine to process a value, and updates a pointer and a 16-bit value in the structure; otherwise, it decrements a different 16-bit value and calls a routine to read a byte and advance a pointer.
- `decrement_and_check_counter`: Decrements a word in a structure and, if the result is zero, updates a value in that structure, sets a bit in a byte, and overwrites a function pointer.
- `check_and_set_flag`: Checks a byte at an offset of a structure in a6 and, if it is non-zero, sets bit 2 of a byte at offset 1 of a0 and writes a long value to offset 2 of a0.
- `update_structure_and_recurse`: Updates a structure's offset and certain fields using values from a global table, then if a certain byte in a global variable is zero, it recursively calls itself with a new pointer, clearing bit 1 of a byte in the structure and setting a new function pointer and a 16-bit value before returning.
- `return_and_setup_pointer`: Returns immediately, otherwise it loads a long address into a1 and calls another routine.
- `call_subroutine_with_conditional_setup`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, sets d0 to 32798, and calls another routine.
- `update_pointer_and_call_handler`: Calls a routine, then if the zero flag is not set, loads a longword address into a1, sets d0 to 32798, and calls another routine.
- `load_address_and_call_2`: Calls a routine, and if the zero flag is not or not, loads a longword address into a1, sets d0 to 4960, and calls another routine.
- `update_struct_and_load_pointer`: Calls a routine to increment a structure member, then if the zero flag is not set, loads a longword address into a1 and calls another routine.
- `call_subroutine_and_set_register`: Calls a routine, and if the zero flag is not not set, loads a longword address into a1, moves a word into d0, and calls another routine.
- `initialize_address_and_load_data`: Calls a routine, then if the zero flag is not set, loads a longword address into a1, moves a word into d0, and calls another routine.
- `call_init_and_load_address`: Calls a routine, and if the zero flag is not set, loads a longword address into a1, moves a word into d0, and calls another routine.
- `update_structure_value`: Calls a routine, and if the zero flag is not set, writes a word to an offset of 34 of the structure in a0 and then writes a longword to an offset of 2 of the structure in a0.
- `check_flags_and_update_pointer`: Checks if bits 4, 5, and 6 of a byte at the pointer in a0 are clear; if they are, it copies a longword from offset 40 of the structure in a0 to offset 2 of the structure in a0 and updates a global pointer, otherwise it returns.
- `check_bits_and_copy_pointer_2`: Checks if bits 6, 4, and 5 of a byte at a pointer in a0 are clear, and if so, copies a longword from offset 28 of a0 to offset 2 of a0, then loads a certain address into a1, loads 3360 into d0, and calls another routine.
- `call_and_set_pointer`: Calls another routine, and if the result is not zero, loads a certain address into a1, loads 3360 into d0, and calls another routine.
- `check_bits_and_copy_pointer`: Calls a routine, then checks if bits 4, 5, and 6 of the byte pointed to by a0 are clear before copying a longword from offset 28 of that structure to offset 2 of that same structure and overwriting a pointer.
- `copy_data_on_bitmask_5`: Checks if bits 4, 5, and 6 of the byte pointed to by a0 are clear, then copies a longword from offset 40 to offset 2 of that same structure, otherwise it returns.
- `set_structure_fields`: Sets a byte at offset 24 of a structure in a0 to a certain value, stores a pointer to a global variable into offset 28 of the structure in a0, and stores a pointer to a certain address into offset 2 of the structure in a0, then calls another routine and returns.
- `update_structure_pointers_2`: Compares a value from a structure to a global variable, modifies a value at offset 10 of a0, and if certain conditions involving a pointer-b, sets function pointers at offset 2 and offset 28 of a0 and calls itself or another routine.
- `update_structure_pointers`: Compares a value from a structure to a global variable, modifies a value at offset 10 of a0, and if certain conditions involving a pointer-based check and a range check pass, updates function pointers at offset 2 and offset 28 of a0 and calls another routine.
- `set_function_pointers`: Writes a certain address to offset 28 of a0 and another certain address to offset 2 of a0.
- `update_pointer_on_range_check`: Calls a routine, then checks if a value in a structure, after subtracting a constant and a value from a global variable, falls within a certain range, otherwise it sets a bit in a byte and overwrites a function pointer or a global variable.
- `check_bit_condition`: Compares two or-values and checks if a value is within a certain range before setting a bit in a global variable.
- `decrement_and_index_table`: Copies a short field into a global variable, shifts it, reads a byte from a table indexed by the shifted value, decrements a counter, and when it reaches zero links a record and writes a constant byte into a global.
- `decrement_record_counter`: Decrements a counter field in a record and, when it reaches zero, links the record into a structure and writes a constant byte into a global field; otherwise it increments a global counter and stores a value based on its contents.
- `store_constant_global`: Writes a single constant byte into a fixed offset of a global structure, initializing that field.
- `store_constant_global_alt`: Writes a different single constant byte into the same fixed offset of a global structure, initializing that field to another value.
- `insert_ordered_node_2`: It stores a routine address into a record, marks a flag byte, copies several fields from the incoming record into a linked structure, then returns two passed values.
- `return_immediately`: Returns immediately without reading or modifying any data, performing no operation.
- `initialize_table_with_flags`: Performs a series of or operations on registers and memory or returns immediately.
- `run_helper_then_set_flag`: Invokes a helper routine once and, if it reports success, runs that routine many more times before writing a fixed byte into a structure field.
- `init_table_with_handler_and_flag`: Initializes a data table by storing several values and offsets into its fields, including a handler address and a couple of zeroed entries.
- `update_fixed_point_and_init_table`: It multiplies two fixed-point values and stores the results, then decrements a counter that eventually triggers a state change.
- `decrement_struct_short`: Reads a short value from a fixed offset in a structure and decrements it by one, storing the updated value back.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clamp_and_wrap_struct_field`: Loads a pointer from a structure, adds a constant to one of its word fields, clamps another word between zero and a maximum, and wraps a short counter by subtracting ten when it exceeds ten.
- `wrap_index_store_table`: Clears a byte field and copies a four-byte value into a structure, then wraps a short counter into a small range by subtracting ten when it exceeds that bound.
- `update_and_clamp_struct`: Adds a constant to a structure word, clamps another word between zero and a maximum, stores a fixed value, wraps a third word, and copies a table entry selected by the clamped value.
- `update_and_clamp_struct_2`: Adds a different constant to a structure word, clamps another word between zero and a maximum, stores a fixed value, wraps a third word, and copies a table entry selected by the clamped value.
- `update_structure_state_from_table`: Uses a value from a structure to index into a table, then updates the structure's counter and status bits based on the retrieved byte.
- `update_structure_counter_and_pointer_6_2`: Checks if a 16-bit value at offset 6 is less than a 16-bit value at offset 22, then potentially updates a counter at offset 22, increments a, and calls a routine to read a byte and advance a pointer while potentially updating a counter at offset 10 or setting a value to 10.
- `update_structure_counter_and_pointer_6`: Checks if a 16-bit value at offset 6 is less than a 16-bit value at offset 22, then potentially updates a counter at offset 22, increments a counter at offset 6, and calls a routine to read a byte and advance a pointer.
- `decrement_and_set_pointer_3`: Decrements a word at offset 6 of a structure in a0, moves a word into d0, calls a routine, and if the negative flag is set, returns; otherwise, it clears bit 2 of a byte at offset 1 of a structure in a0, moves a word into offset 22 of a structure in a0, and moves a longword address into offset 2 of a structure in a0.
- `decrement_and_set_mask`: Decrements a word at offset 34 of a structure in a0, and if the result is not zero, sets bit 2 of a byte at offset 1 of a structure in a0, then stores a longword into offset 28 of a structure in a0 and stores a longword into offset 2 of a structure in or a0.
- `increment_and_set_mask_logic`: Increments a word at offset 6 of the structure in a0, loads 630 into d0, and calls another routine; if the result is negative, it clears a word at offset 34 of the structure in a0 and overwrites two pointers at offset 28 and offset 2 of the structure in a0, otherwise it returns.
- `increment_and_set_pointer`: Increments a word at offset 6 of a0, loads 630 into d0, calls another routine, and if the result is negative, clears bit 2 of a byte at offset 1 of a0, otherwise it writes a certain address to offset 2 of a0, writes 38 to offset 22 of a0, and writes a certain address to offset 2 of a0.
- `decrement_and_branch_2`: Decrements a word at offset 22 of a structure and returns if the result is not negative, otherwise it sets bit 2 of a byte at offset 1 of a structure and continues.
- `decrement_and_call_routine`: Decrements a word at offset 6 of a structure, loads a constant into d0, calls another routine, and returns if the result is not negative.
- `update_memory_masks`: Performs a series of or operations on several memory locations using pointers and returns a value or.
- `update_bit_masks_subset`: Performs a subset of bitwise or-operations on several memory locations using pointers.
- `modify_structure_flags`: Clears bits 0 and 2 of a byte at offset 1 of a0, calls another routine, and if the zero flag is clear, writes a value from a table to offset 8 of a0, clears certain fields, sets flags, calls another routine, and writes a value to offset 34 of a0 and a pointer to a routine or a value to offset 28 of a0.
- `decrement_and_scale_structure_field`: Decrements a 16-bit value at offset 14 of a structure, clamps it to a minimum of -20, and adds a quarter of that value to a field at offset 6, then calls a routine to add a byte to a pointer.
- `set_structure_members_3`: Writes a constant byte to a memory location, updates a pointer in a structure, and stores a routine address into a structure member, then returns.
- `update_structure_and_clamp_value`: Calls two routines, then decrements a word at offset 14 of a structure, and if it is less than -20, sets it to -20, then adds the quotient of that value divided by 4 to a value at offset 6 of a structure.
- `clamp_structure_value_2`: Calls two routines, then increments a word at offset 14 of a structure, and if it exceeds 20, clamps it to 20, then calculates a quotient, adds it to a value at offset 6, and calls another routine; if a certain condition is met, it writes a byte to offset 24, a longword to offset 28, and a pointer to offset 2, then returns.
- `update_structure_counter_and_pointer_7`: Increments a 16-bit value at offset 14 in a0, clamps it to a maximum of 20, and updates a 16-bit value at offset 6 by adding a quotient.
- `update_structure_and_read`: Calls a routine, then decrements a 16-bit value at offset 14 of a structure in a0, clamps it to a minimum, and if it is not less than or equal to a certain value, adds a certain amount to a value at offset 6 of a structure in a0, and if a certain condition is met, loads a long address into a1, sets d0 to a constant, and calls a routine to read a byte.
- `update_structure_value_3`: Calls another routine, increments a 16-bit value at offset 14 of a structure in a0, and if that value is not less than or equal to 20, clamps it to 20, then divides a long-extended version of that value by 4 and adds the result to a 16-bit value at offset 6 of the structure, before checking if that value is less than a 16-bit value at offset 22 and calling a routine to read a byte and advance a pointer.
- `update_value_with_offset`: Calculates a signed offset based on a comparison and adds it to a value in a memory location.
- `update_structure_value_2`: Calculates a new value based on a subtraction and sign-check, stores it in a 16-bit field at offset 16 of a structure in a0, and updates a 16-bit field at offset 8 of that structure by adding the result of a division by 4.
- `update_structure_field_and_relink`: Calls add_bytes_accumulate, then updates a 16-bit field at offset 10 of the structure in a0 by calculating the difference between a value at offset 60 of a6 and the current field value, applying a sign-check, and adding the result.
- `process_structure_bounds`: Calls a routine, and if certain conditions involving a pointer or a value in a register are met, updates a pointer or a byte in a structure, or otherwise updates a pointer-referenced address or a structure member, then returns.
- `update_structure_bounds_and_pointer`: Checks certain range conditions on a structure in a0, and if met, updates a pointer, a function pointer, and a byte, or otherwise modifies a length or offset field in the structure and calls double_and_xor_value or read_byte_advance_pointer.
- `process_structure_data`: Performs several conditional checks on values in a structure, calls a routine, and if certain bit or range conditions are met, updates a value at offset 10 of a0, updates a byte at offset 24 of a0, and sets function pointers at offset 2 and offset 28 of a0.
- `accumulate_and_set_bit`: Loads a constant into d0, calls a routine, then checks bit 6 of d1 and, if set, sets bit 4 of a byte at offset 1 of a structure, otherwise it continues to update a pointer and a register before returning.
- `set_bit_on_byte_test`: Loads a constant into d0, calls a routine, then checks bit 6 of d1 and, if set, sets bit 4 of a byte at offset 1 of a structure, otherwise it returns if d1 is not negative.
- `set_structure_members`: Writes a value to a memory location, sets a byte in a structure to a fixed value, and copies a value from a pointer-referenced address.
- `decrement_and_process_stream_2`: Decrements a word at offset 6 of a structure, loads a pointer from offset 24 of that structure into a1, loads a word from that pointer into d0, updates a1 to point to a new address, and calls two routines.
- `decrement_and_process_stream_3`: Decrements a 16-bit value at offset 6 of a structure, then calls a routine to read a byte and advance a pointer, followed by a routine to search a table for a value, and returns.
- `update_struct_member_and_flag`: Writes a dword to a structure offset, sets a byte in a structure to one, and copies a dword from a table or structure offset into a third structure member.
- `increment_and_process_stream`: Increments a word at offset 6 of a structure, loads a pointer from offset 24 of that structure into a1, loads a word from that pointer into d0, updates a1 to point to a new address, and calls two routines.
- `increment_and_process_stream_2`: Increments a 16-bit value at offset 6 of a structure in a0, then calls a routine to read a byte and advance a pointer, followed by another routine call, and returns.
- `init_struct_set_flags`: Stores a two-byte value and a four-byte value into fixed offsets of a structure, sets a byte field to all ones, and copies a four-byte value from a location pointed to by another structure field.
- `decrement_and_return`: Decrements a structure word, invokes another routine, and returns the current value of that same word.
- `init_struct_set_flag_one`: Stores a two-byte value and a four-byte value into fixed offsets of a structure, sets a byte field to one, and copies a four-byte value from a location offset further along from another structure field.
- `increment_struct_value`: Increments a signed value stored at a fixed offset within a structure pointed to by a register, then invokes another routine.
- `store_struct_fields`: Stores a word from a register into a structure field, writes a longword into another field, and copies a longword from a table indexed by a pointer stored in the structure.
- `decrement_and_process_stream`: Decrements a value in a structure, potentially updating another value and clearing a bit, then calls a routine to process a byte stream.
- `tiny_clr_rts_2`: Small routine of 2 instructions that clears d0, then returns.
- `set_structure_members_extended`: Writes a word and a long to a structure and copies a value from a pointer-referenced address.
- `increment_and_check_limit`: Increments a value in a structure and checks if it exceeds a certain limit, potentially clearing a bit and incrementing another value before calling a stream processing routine.
- `update_structure_values`: Stores a byte and a long into a structure and updates a long value from a table based on a comparison.
- `compare_and_clear_register_2`: Loads a word from a structure into d0, compares it with a word at offset 22 of that structure, and if the comparison or a certain condition fails, clears d0 and returns.
- `update_pointer_target_bounds`: Updates a 32-bit value in a structure using a value from a secondary table, or replaces it with a different value from that table if a certain condition involving a comparison of two 16-bit values is met.
- `compare_and_clear_register`: Compares a value from a structure against another value in a structure, clearing a register if the comparison fails or if a certain condition is met, otherwise it returns.
- `update_state_function_pointer`: Decrements global and offset values and updates a function pointer in a structure based on a conditional check.
- `add_byte_advance_pointer`: Adds a byte from a table to a register value and stores the result as a word, then advances the table pointer by two bytes.
- `read_byte_advance_pointer`: Reads a byte from a pointer, adds the following byte to a register value and stores it as a word, advances the pointer by two bytes, and resets the pointer if the read byte was negative.
- `add_bytes_accumulate`: Adds a byte from a table to a register value and stores it as a word, adds two more bytes to two separate short fields, and advances the table pointer by four bytes.
- `advance_past_negative_bytes`: Walks a table of signed bytes, accumulating an index while the referenced byte stays negative until a non-negative value is found.
- `scan_records_check_flags`: Scans a series of fixed-size records, returning negative one when a flag bit is clear and stopping at the first record whose flag is unset.
- `scan_bytes_clear_bit`: Steps through a block of bytes in fixed increments until it finds one whose least significant bit is clear.
- `compare_words_return_quotient`: Reads a word from a fixed offset and, if it matches a word in a passed structure, returns the quotient of another field divided by a constant.
- `check_range_validity`: Checks if a value in a structure falls within a specific range defined in another structure.
- `validate_transformed_value`: Validates a transformed value against a range and a secondary calculated limit, returning a specific value if certain conditions are unable to be met.
- `traverse_table_for_sentinel`: Iterates through a table using a pointer derived from a structure, checking for a negative value or a specific sentinel value.
- `search_table_for_value`: Calculates a pointer into a table using a value from a structure in a0, then iterates through the table to check if a specific value exists.
- `initialize_struct_fields`: Initializes a large structure by writing zeros to most of its fields and a single byte to one field.
- `setup_audio_and_copy_block`: Waits for a custom register bit to clear, then programs audio and timer hardware registers and sets up DMA and interrupt control.
- `swap_globals_and_store_result`: Swaps two global variables and adjusts a third counter with wrap-around, assigning a routine address when the counter wraps.
- `set_globals_enable_int`: Sets two global variables to fixed values, enables a set of DMA channels, and enables interrupts.
- `copy_block_of_words_2`: Copies a block of forty words from a source table pointed to by a global variable into a destination buffer passed in a register, advancing the source pointer past the copied block.
- `store_global_pointer_pair`: Clears a two-byte field and stores a global variable's address into two consecutive pointer slots, splitting the address into high and low words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_return_routine_4`: This routine is empty and returns immediately without performing any operations.
- `empty_return_routine_5`: This routine contains no instructions and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_state_and_call_subroutines`: Writes a constant value to a global variable, updates a pointer and a stack-based variable, sets certain memory locations, calls another routine with a constant, and then calls two further routines before entering an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_context_and_disable_interrupts`: Writes a pointer from a1 to a memory location, sets two 32-bit constants in a structure, calls another routine, sets a pointer to the custom chip base, writes a constant to a global variable, clears a byte in a structure, disables interrupts by modifying the status register, and clears a byte in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_struct_and_execute_subroutines`: Updates a 16-bit value in a structure via a pointer, clears two bytes in a structure, and writes a global variable to a 32-bit field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_and_disable_interrupts`: Calls another routine, updates a pointer in a global structure, and writes a value to a custom hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_and_loop_counters`: Updates a global variable with a value from a structure, calls several subroutines, and executes multiple loops that wait for a 16-bit counter in a structure to reach 300.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_status_and_call_subroutine`: Disables interrupts, clears a byte in a structure, updates a global pointer, and calls another routine before restoring the status register and clearing a 16-bit value in a global variable or structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bit_and_sync_counter`: Resets a 16-bit counter in a structure, calls several subroutines, and performs a bitwise OR on a 16-bit value at a fixed offset, then uses a return value from another routine to flip a bit in a byte via a bit-shift, before finally updating a global variable with a 32-bit value from a structure offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_call`: Clears a byte at offset 108 of a structure, moves a value from that location to a new offset, sets that offset to a constant, restores the status register, loads a constant into a0, and calls another routine before clearing a global variable and a sequence of words in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_branch_and_call`: Calls another routine, clears a 16-bit value at offset 4 of a structure, and if that value is less than 300, calls another routine; otherwise, it loads a constant into a0, calls another routine, calls another routine, clears a 16-bit value at offset 4 of a structure, and if that value is less than 300, calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_table_and_call`: Moves a value from a global variable to an offset, sets that global variable to a constant, restores the status register, loads a constant into a0, calls another routine, clears a global variable, and clears a sequence of words starting at a table address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_toggle_bit`: Clears a 16-bit value at offset 4 of a structure and loops until that value is at least 300, then toggles a bit in a byte, calls another routine, calls another routine, clears a 16-bit value at offset 4 of a structure, and loops until that value is at least 300 before calling another routine and loading a 32-bit value into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_and_call`: Loads a constant into a0, calls another routine, clears a global variable, and clears a sequence of words in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_counter`: Calls two routines, sets a 16-bit value at an offset of 4 in a structure to zero, and loops until that value reaches 300 before calling a third routine and loading a 32-bit value into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_with_bit_toggle`: Calls another routine, clears a global variable, and clears a sequence of words in a table, then toggles a bit in a byte, calls another routine, calls another or routine, clears a 16-bit value at offset 4 of a structure, and loops until that value reaches 300 before calling another routine and loading a 32-bit value into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_with_interrupt_disable`: Calls two routines, clears a 16-bit value at offset 4 of a structure, and if that value is less than 300, calls another routine; otherwise, it saves the status register, disables interrupts, moves a value from a structure to a global variable, restores the status register, and restores a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_large_table`: Clears a global variable and a sequence of words starting at a table address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_restore_context`: Clears a 16-bit word at offset 4 of a6, loops while that word is less than 300, calls another routine, saves the status register, disables interrupts, moves a 32-bit value from offset 6 of a6 into a global variable, restores the status register, and restores a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_struct_members`: Clears or resets or certain 16-bit words at offsets 12 through 68 relative to a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_context_and_store_global`: Stores a 32-bit value from offset 6 of a6 into a global variable, restores the status register and a6, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_struct_members_offset_16`: Clears or resets or certain 16-bit words at offsets 16 through 68 relative to a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_status_and_a6`: Restores the status register and a6 from the stack and returns.
- `save_registers_and_increment_counter`: Saves all registers to the stack, increments a 16-bit value at offset 4 of a pointer in a6, writes 112 to the interrupt request register, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_struct_members_offset_28`: Clears or resets or certain 16-bit words at offsets 28 through 84 relative to a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_counter_and_signal_interrupt`: Increments a word at an offset of four from a global pointer, writes the value one hundred twelve to the interrupt request register, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_struct_members_offset_32`: Clears or resets or certain 16-bit words at offsets 32 through 84 relative to a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_counter_and_signal_interrupt_v2`: Increments a 16-bit value at offset 4 of a structure pointed to by a6, writes 112 to the interrupt request register, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_struct_members_offset_48`: Clears or resets or certain 16-bit words at offsets 48 through 100 relative to a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_table_with_stepped_values`: Loads a longword from a global variable, then loops 4 times writing the high word of that value to offset 2 and the low word to offset 6 of an 8-byte entry at a pointer, incrementing the source value by 10240 and advancing the pointer by 8 each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_entries`: Clears a series of 16-bit values at offsets 60 through 120 from the pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_five_entries_with_stepped_d0`: Loops 5 times writing the high word of d0 to offset 2 and the low word to offset 6 of each 8-byte entry at a0, adding 10240 to d0 and advancing a0 by 8 each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_table_range`: Clears a series of 16-bit values at offsets 68 through 120 from the pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_entries_stepped_by_d7`: Repeatedly writes the high word of d0 to offset 2 and thelow word to offset 6 of each 8-byte entry at a0, adding 10240 to d0 and advancing a0 until d7 counts down to -1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_table_with_function_result`: Clears a series of 16-bit values at offsets 50 through 7c from the pointer in a0, calls a routine, clears a byte at an offset in a global variable, and then loops writing the low word of a returned value to offset 6 of an 8-byte entry at a pointer, incrementing that value by 10240 and advancing the pointer by 8 each time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_pointer_and_loop`: Increments a0 by 8 and loops if d7 is not zero.
- `process_asset_data`: Searches for a specific 4-byte signature in a table, then loops 32 times writing a bit-manipulated value derived from bytes in the table to a destination pointer, then searches for another signature and clears several bytes at an offset in a pointer, calls a routine, clears a byte in a global variable, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_check_global`: Clears several 16-bit values at offsets 60, 68, 70, 78, and 7c from a pointer, calls a routine, clears a byte in a global variable, and loops until a certain value in a global variable reaches 30, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_complex_table`: Searches for a signature in a table, then loops 32 times writing a bit-manipulated value to a destination pointer, then searches for another signature, calls two routines, clears certain bytes, calls a third routine, clears a byte in a global variable, calls a fourth routine, writes a value to a global variable, and loops 32 times writing bit-manipulated values to a destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_patch_data`: Clears three 16-bit values at offset 112 of a structure in a6, calls a routine, then loops to write a custom bit-manipulated value to a destination pointer, and finally calls another routine 256 times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transform_and_repeat_data`: Performs a bitwise transformation loop on 3-byte chunks from a source in a1 to a destination in a2 using a mask from d3, searches for a 32-bit marker in a0, and then calls another routine 256 times in a loop.
- `expand_byte_stream`: Iterates through a source buffer, and if a byte is negative, copies a sequence of bytes or a single byte to a destination buffer, otherwise copies a single byte, while incrementing a counter.
- `patch_instruction_values`: Iterates through a range of instructions, and if certain bit patterns do not match, modifies the instruction or its operands by adding a constant value, then returns.
- `reduce_instruction_values`: Iterates through a range of instructions, and if certain bit patterns do not match, modifies the instruction or its operands by subtracting a constant value, then returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 130 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_pointer`: Calls two routines, then loads a 32-bit value from offset 118 of a structure in a6 into a global variable and updates a6 using a pointer from offset 114 of that same structure.
- `increment_and_reset_counter`: Saves registers to the stack, loads a pointer into a6, and if a byte at an offset of a6 is not zero, calls two routines, clears a byte at an offset of a6, increments a global variable, and calls another routine; if a certain bit in a register is not set, it restores registers, sets a register to a certain value, sets a register to a certain value, and returns via an exception return.
- `swap_structure_elements`: Swaps the 32-bit values located at offset 100 and offset 104 of a structure pointed to by a6.
- `store_split_values_to_table`: Takes a 32-bit value from offset 100 of a structure in a6, splits it into two 16-bit halves using a swap, and stores these halves into a table at a global address, repeating this five times with an increment of 10240 per iteration.
- `execute_nested_loop_sequences`: Executes a series of nested loops that repeatedly call increment_nibbles_at_pointer while clearing a 16-bit value at an offset of 110 from a pointer in a6.
- `increment_nibbles_at_pointer`: Increments the nibbles of a 16-bit word at the location pointed to by a6, where each nibble is incremented if it is not at its maximum value.
- `perform_iterative_nibble_decoding`: Executes a loop fifty times that clears a 16-bit word at offset 6e of a structure in a6 and calls another routine to decode packed nibbles.
- `decrement_nibbles_at_pointer`: Decrements the lower nibble of a 16-bit word at the location pointed to by a0, and if the high or low nibble is non-zero, it subtracts a constant or decrements the low nibble.
- `update_word_with_incremented_value`: If a certain condition involving d1, d2, and d6 is met, increments d0 and stores the result into a 16-bit word at the pointer in a0; otherwise it returns.
- `copy_and_search_data`: Copies a certain amount of data from a source pointer in a6 to a destination pointer in a6, copies 32 bytes from offset 16 of a6 to a global variable, searches for a specific 4-byte marker in a memory stream, and enters a loop that clears a byte at offset 108 of a6 and calls another routine until a 16-bit value at offset 110 of a6 is no longer less than a value at offset 4 of a6.
- `parse_and_initialize_data_structures`: Searches for specific four-byte markers in a memory stream, reconstructs 16-bit words from byte triplets, and populates several offsets in a structure in a6 with calculated values.
- `copy_byte_stream`: Copies bytes from one memory location to another, handling signed byte values by either copying a single byte or a sequence of bytes based on the value.
- `expand_conditional_data_blocks`: Calls another routine to expand data based on values found in a source buffer, conditionally executing the call up to five times depending on whether the values at specific offsets are non-zero.
- `process_encoded_stream`: Reads an encoded stream from a buffer, and depending on the byte value, either skips a certain number of bytes, copies a single byte, or copies a sequence of bytes to a destination buffer, while updating a destination pointer.
- `clear_audio_tables`: Iterates through a list of bytes to find a certain value, then clears a series of memory locations, updates a global pointer, and clears the four audio volume registers.
- `reset_audio_dma`: Clears a global byte, clears the four audio volume registers, and sets a value in a register that controls DMA or audio settings before returning.
- `increment_and_update_audio_registers`: Increments a global byte and checks it against a limit, otherwise it returns.
- `update_object_property_and_state`: Extracts an index from a structure, uses it to look up data in a table, and updates a structure with new pointer, position, and attribute values, then calls another routine to handle the side effects or state changes.
- `update_object_position_and_state_2`: Calculates a value by subtracting a global variable from a word at offset 18 of a6, and if the result is not negative, sets a value in a structure, otherwise it updates several pointers and registers, clears a byte, and potentially clears a DMA or audio setting or updates a counter.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_position_and_state`: Checks a type-identifier or flag in a structure, then either updates a 16-bit pointer or offset or modifies a 16-bit value or a 16-bit counter in the structure, potentially clamping or resetting it, and updates a secondary pointer or offset in a different structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: It is an empty routine that returns immediately without touching any data or registers.
