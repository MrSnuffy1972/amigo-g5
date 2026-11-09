# Game 260 (Epic v1.3 1819): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00260). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2822637 top-level bytes in 49 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 100 of 100 units identical (bytes and relocations), 26 raw-word lines |
| Functions named and described | pass, 525 of 525 named (0 author symbols), 525 of 525 described (49 by rule, 475 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 9042 of 9161 (98.70%) |
| Recovered without load address (counted in proof 1, flagged) | 12288 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13587 of 14564 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 318115 |
| code | 45048 |
| data | 2459347 |
| padding | 127 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Epic.slave | 1 | 426 | 35 |
| Disk.1:bootblock | 1 | 202 | 9 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3882 | 167 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 514 | 43 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 639 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3740 | 69 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 159 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 438 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1237 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2605 | 82 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Disk.2 bytes 5632-11776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.3 bytes 5632-11776 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Epic_v1.3_1819/Epic/Disk.1: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- Epic_v1.3_1819/Epic/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Epic_v1.3_1819/Epic/Disk.3: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- 55 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 3087 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 452 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 338 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2575 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 97 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 265 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 375 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1626 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Epic_v1.3_1819/Epic/Disk.2: bytes 5632-11776 look like code no run loaded: recovered position-independently (load address unknown)
- Epic_v1.3_1819/Epic/Disk.3: bytes 5632-11776 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (18 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (22 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (20 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- Epic_v1.3_1819/Epic/Disk.1: 55 logged loads cover 309248 bytes of the image
- Epic_v1.3_1819/Epic/Disk.1: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- Epic_v1.3_1819/Epic/Disk.2: 1 logged loads cover 6144 bytes of the image
- Epic_v1.3_1819/Epic/Disk.2: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- Epic_v1.3_1819/Epic/Disk.3: 1 logged loads cover 6144 bytes of the image
- Epic_v1.3_1819/Epic/Disk.3: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- 2 never-loaded region(s), 12288 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 8 packed track region(s), 2365952 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_dispatch_tables` (Epic.slave): Sets several custom chip registers to specific values, clears a memory block starting at a specific address using a loop, and calls two routines through a table. Kind: disk hardware/track check. Pass path: the branch where _u0__d0_16BC != 0 is taken (the first major if block).

## Functions


### Epic.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_hardware_and_dispatch_tables`: Sets several custom chip registers to specific values, clears a memory block starting at a specific address using a loop, and calls two routines through a table.
- `copy_and_branch`: Copies bytes from a0 to a1 in a loop, calls another routine, and then branches to a new location.
- `set_global_and_call`: Stores the constant value an address in a global variable and calls another routine, then jumps to another routine.
- `perform_multiplication_and_call`: Stores a word from d0 and a constant into two global variables, calculates the product of d0 and 5120, calls another routine, and then clears a global word.
- `initialize_function_pointers`: Fills a memory block with repeating words via a loop and then populates several global variables with specific constants and the addresses of several different routines.
- `execute_dispatch_and_recursively_call`: Calls a routine through a pointer at offset 36 of a table in a2; if the return value in d0 is 128, it adds a constant to a pointer in a1 and calls another routine, otherwise it returns.
- `set_global_flag_and_call_subroutine`: Sets a longword in a global variable to an address and calls another routine, then returns via a jump.
- `copy_byte_loop`: Copies bytes from a source in a0 to a destination in a1 using a loop controlled by d0, then branches to another location.
- `copy_data_loop`: Copies 512 words from a source table to a destination pointed to by a0 using a loop, then stores the byte in d0 into a global variable and returns.
- `multiply_and_update_global_state`: Multiplies a value in a register by 5764, stores the result in a global variable, sets another global variable to 32, calls another routine using a byte loaded from a relative offset, and finally clears a 16-bit global variable before returning.
- `initialize_dispatch_tables`: Fills two memory areas with repeating words using loops, writes several constants and addresses to global variables, and subtracts a1 from itself before branching.
- `execute_indirect_dispatch`: Calls a routine through a pointer at offset 36 of a table in a2, and if the return value in d0 is 128, it performs a series of memory writes and calls before jumping to a pointer in a0.
- `set_global_flag_and_call_subroutine_2`: Sets a global variable to an address, calls another routine, and then returns via a jump.
- `copy_bytes_loop`: Copies bytes from a0 to a1 in a loop and then branches.
- `copy_memory_block`: Copies five hundred and eleven longwords from a table to a destination pointed to by a register using a loop, then stores the final value of the loop counter as a byte.
- `update_global_multiplier_and_call`: Multiplies a value in a register by 5632, stores the result in a global variable, sets another global variable to 32, calls another routine with a byte loaded from a relative offset, and finally clears a 16-bit global variable before returning.
- `setup_dispatch_table`: Fills two memory blocks with repeating words via loops, then populates a large table at offset 3786 of a1 with a constant and several routine addresses.
- `call_indirect_and_check`: Calls a routine through a pointer found in a table at an offset in a2, and if the returned value in d0 is 128, calls another routine.
- `mask_and_store_byte`: Reads a byte from a pointer in a0, and if it is not zero, masks the lower four bits and stores the result in a global variable.
- `compare_byte_and_return`: Loads a byte from an offset of a pointer in a1, pushes its bitwise NOT and rotated value onto the stack, compares it against two different values, and returns if the comparisons do not match.
- `set_d0_one`: Sets register d0 to 1 and branches to another routine.
- `set_d0_two`: Sets register d0 to 2 and branches to another routine.
- `set_d0_four`: Sets register d0 to 4 and branches to another routine.
- `set_d0_eight`: Sets register d0 to 8 and branches to another routine.
- `set_d0_sixteen`: Sets register d0 to 16 and branches to another routine.
- `set_d0_thirty_two`: Sets register d0 to 32 and branches to another routine.
- `set_d0_sixty_four`: Sets register d0 to 64 and branches to another routine.
- `set_d0_two048`: Sets register d0 to 2048 and branches to another routine.
- `set_d0_four096`: Sets register d0 to 4096 and branches to another routine.
- `set_d0_eight_thousand`: Sets register d0 to 8192 and branches to another routine.
- `write_interrupt_request`: Writes the constant value 16384 to the interrupt request register twice and returns.
- `call_indirect_function`: Calls a function through a pointer located at a specific offset within a table and returns the resulting values.
- `call_indirect_offset`: Loads a pointer from a table using a1 plus an offset in d0, calls the routine at that pointer, and then returns if d1 is not zero.
- `call_indirect_with_offset`: Increments a pointer in a1 by two, sets d0 to 128, and calls a function through a pointer in a table using d0 as an offset, then returns.

### Disk.1:bootblock

- `trigger_system_trap_2`: Loads a longword into a global variable using a pointer relative to the program counter and then calls a system trap.
- `boot_and_load_kernel`: Allocates memory, copies a table to a destination, sets up a structure with specific values, calls another routine, copies 99 words from one location to another, and finally jumps to a new address.
- `initialize_boot_sequence`: Copies 256 words from a source to a destination, performs a system trap, copies 16383 bytes from a1 to a destination, clears several memory locations, and calls two routines before jumping to a routine at a table offset.
- `load_and_execute_routine`: Loads a routine address from a pointer, calls a routine via a pointer, and then jumps to the loaded routine address after writing three values to the color register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_memory_and_setup_coprocessor`: Clears a block of memory with zeros, copies a data table to a new destination, and then copies a second table to a memory location starting at a specific address.
- `wait_for_vblank`: Loops until the value at the video beam position register is greater than or equal to -8000.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_stack_and_trap`: Sets the stack pointer to a constant value, loads a pointer into a0 from a relative offset, stores that pointer into a global variable, and executes a trap.
- `initialize_system_and_dispatch`: Disables interrupts, sets the stack pointer, clears a longword at a specific address, and calls six different routines.
- `clear_global_long`: Clears a 32-bit global variable.
- `swap_global_pointers`: Calls another routine and then swaps the values of two global long variables and a third global pointer.
- `update_bitmaps`: Sets the stack pointer using a value from a structure, then enters a loop that performs bitwise operations on four memory areas based on values from a table and a pointer, potentially calling another routine via an indirect jump.
- `update_bitmask_buffers_4`: Iterates through a list of values to perform bitwise AND and OR operations on four different destination buffers using a value fetched from a table, and calls another routine via a jump table.
- `update_bitmask_set_a`: Calculates an index using d7 and d0, fetches a 32-bit value from a table, and performs bitwise operations to update four destination buffers using the fetched value and its complement.
- `update_bitmask_arrays_4`: Iterates through a list of indices, fetches a 32-bit value from a table, and updates four memory locations using bitwise AND, OR, and NOT operations based on whether the index exceeds 31.
- `update_bitmask_arrays`: Fetches a 32-bit value from a table using an index derived from registers, then performs bitwise AND, OR, and NOT operations to update four separate memory arrays pointed to by registers using that value and its complement.
- `update_bitmask_arrays_2_2`: Processes a sequence of indices to fetch 32-bit values from a table and applies bitwise OR and AND operations to four destination buffers.
- `update_bitmask_arrays_2`: Calculates an index using a bitwise AND and multiplication, fetches a 32-bit value from a table, and performs bitwise OR and AND operations on four separate memory buffers pointed to by registers a2 through a5.
- `partition_bitmask_data_2`: Uses a table to perform bitwise AND, OR, and NOT operations on four memory locations, with specific logic for indices greater than 31.
- `partition_bitmask_data`: Uses a value from a table to perform bitwise AND, OR, and NOT operations on four separate memory locations pointed to by registers, then updates the stack and returns.
- `update_bitmask_layers_2`: Fetches 32-bit values from a table and performs bitwise OR operations on three memory locations and a bitwise AND on a fourth.
- `update_bitmask_layers`: Fetches a 32-bit value from a table using an index derived from registers, then performs bitwise OR operations on three memory locations and a bitwise AND with the bitwise NOT of that value on a fourth memory location.
- `update_bitmask_arrays_3_2`: Retrieves 32-bit values from a table and updates four memory locations using bitwise AND and OR operations.
- `update_bitmask_arrays_3`: Fetches a 32-bit value from a table using an index derived from a bitwise AND of two registers, then performs bitwise AND and OR operations on four separate memory locations pointed to by registers a2 through a5.
- `update_bitmask_with_initial_fill_2`: Updates four destination buffers by applying bitwise OR, AND, and NOT operations using values retrieved from a table.
- `update_bitmask_with_initial_fill`: Fills four destination buffers with initial 32-bit values, then enters a loop to update those buffers using bitwise operations based on values fetched from a table.
- `copy_and_mask_data_2`: Updates four memory locations by applying bitwise AND and OR operations using a value fetched from a table.
- `copy_and_mask_data`: Copies sixteen 32-bit words from four source pointers to four destination pointers, then performs a bitwise AND with a value retrieved from a table and applies bitwise AND and OR operations to the destination pointers.
- `copy_and_or_data_2`: Applies bitwise OR and AND operations to four memory locations using a 32-bit value retrieved from a table.
- `copy_and_or_data`: Copies twelve 32-bit words from four source pointers to four destination pointers, then performs a bitwise AND with a value retrieved from a table and applies bitwise OR and AND operations to the destination pointers.
- `mask_and_or_data_2`: Performs bitwise AND and OR operations on four destination pointers using a value fetched from a table.
- `mask_and_or_data`: Copies four 32-bit words from four source pointers to four destination pointers, then performs a bitwise AND with a value retrieved from a table and applies bitwise AND and OR operations to the destination pointers.
- `or_data_with_table_value_2`: Retrieves a value from a table and performs bitwise OR operations on four destination memory locations.
- `or_data_with_table_value`: Retrieves a value from a table using a calculation involving d0 and d7, then performs bitwise OR operations on four destination pointers using that value and calls another routine.
- `apply_sprite_mask_to_buffers`: Iterates through a list of sprite data to apply bitwise masks to four destination memory buffers.
- `update_sprite_attribute_masks`: Copies longwords from registers into four destination memory buffers, then performs bitwise masking operations on those buffers using a value retrieved from a table based on a register index.
- `apply_bitmask_to_buffers`: Iterates through a list of data segments to apply bitwise masks to four separate memory buffers, using alternating bit patterns to modify the contents, and calls another routine via a jump table.
- `update_bitmask_buffers`: Writes three longwords from registers d5, d6, and d3 into four separate memory buffers pointed to by a2, a3, a4, and a5, then performs a bitwise mask operation on those buffers using a value retrieved from a table based on the index in d1, and finally calls another routine.
- `apply_bitmask_to_buffers_2`: Iterates through a list of data structures to perform bitwise AND and OR operations on four memory buffers using alternating bitmasks.
- `update_bitmask_buffers_2`: Copies three sets of longwords from registers into four destination memory buffers, then performs a bitwise mask operation on those buffers using a value retrieved from a table based on a register index.
- `update_bitmask_buffers_3_2`: Iterates through a list of bitmask parameters to update four memory buffers by clearing and setting bits based on values retrieved from a table.
- `update_bitmask_buffers_3`: Copies four 32-bit values from registers to four pointers, then uses a value from a table to clear and set specific bits in those four memory locations before returning.
- `update_bitmask_buffers_alt_2`: Iterates through a list of bitmask updates, clearing specific bits in four memory locations and then setting them using values from a table, and calls another routine via a jump table.
- `update_bitmask_buffers_alt`: Uses a table to clear bits in four memory locations pointed to by a2 through a5, then sets bits in those locations using values from d6 and d3, and returns.
- `update_bitmask_buffers_double_2`: Iterates through a list of data structures to perform bitwise AND and OR operations on four memory locations spaced at fixed intervals, using bitmasks retrieved from a table.
- `update_bitmask_buffers_double`: Performs two consecutive sequences of copying 32-bit values from registers to four pointers, then clears and sets bits in those locations using a table-derived mask before returning.
- `update_bitmask_buffers_minimal_2`: Iterates through a list of bitmask parameters to clear and set bits in four memory locations spaced by fixed offsets, using values retrieved from a table.
- `update_bitmask_buffers_minimal`: Clears bits in four memory locations pointed to by a2 through a5 using a mask from a table, then sets bits in those locations using values from d6 and d3, and returns.
- `update_bitmask_buffers_via_table`: Iterates through a list of bitmask parameters to update four memory locations at fixed offsets from a base pointer, using values retrieved from a table and applying bitwise AND/OR operations with alternating bit patterns.
- `update_bitmask_buffers_d5_d6`: Clears bits in four memory locations pointed to by a2 through a5 using a mask from a table, then sets bits in those locations using values from d5 and d6, and returns.
- `update_bitmask_buffer_segments`: Iterates through a list of data structures to perform bitwise AND and OR operations on four memory segments, using alternating bitmask patterns.
- `update_bitmask_buffers_d5_d3_2`: Iterates through a list of bitmask parameters to update four memory locations by clearing specific bits and setting others using values from a table.
- `update_bitmask_buffers_d5_d3`: Clears bits in four memory locations pointed to by a2 through a5 using a mask from a table, then sets bits in those locations using values from d5 and d3, and returns.
- `clear_bits_in_memory_2`: Calculates a pointer using a global variable and an offset, then clears bits in that pointer and three subsequent offsets of 4000, 8000, and 12000 words.
- `set_and_clear_bits_2`: Calculates a pointer using a global variable and an offset, sets bits at that location using a value from a global variable, and then clears bits at offsets of 4000, 8000, and 12000 words.
- `branch_to_offset_506e`: Performs a relative branch to a different location.
- `branch_to_middle_of_routine`: Performs a short branch to an instruction located within another routine.
- `update_bit_mask`: Calculates a pointer using a global variable, an offset from a global variable, and a value derived from d0 and d1, then performs bitwise OR on that pointer and three other offsets from it, while performing a bitwise NOT and OR on a fourth offset, and then returns.
- `apply_bitmask_to_memory`: Calculates a pointer using a global variable, an offset from a global variable, and a value derived from d0 and d1, then performs bitwise OR and bitwise AND NOT operations on several 16-bit words at specific offsets from that pointer.
- `clear_bits_in_memory`: Calculates a pointer using a global variable, an offset from a global variable, and a value derived from d0 and d1, then performs bitwise AND operations using a bitwise NOT of a value from a global variable on four 16-bit words at specific offsets from that pointer.
- `set_and_clear_bits`: Calculates a pointer using a global variable, an offset from a global variable, and a value derived from d0 and d1, then performs a bitwise OR on one 16-bit word and bitwise AND NOT operations on three other 16-bit words at specific offsets from that pointer.
- `set_and_clear_bits_variant`: Calculates a pointer using a global variable, an offset from a global variable, and a value derived from d0 and d1, then performs bitwise OR operations on three 16-bit words and a bitwise AND NOT operation on a fourth 16-bit word at specific offsets from that pointer.
- `configure_blitter_parameters_2`: Calculates a value based on a global variable, performs an indirect jump to another routine, and then updates several hardware registers including the blitter and custom chip registers based on arithmetic results and bit tests.
- `update_pointer_table`: Tests bits in memory pointed to by a1 and a5, then enters a loop that iterates through a table of longwords, adding the address of the table to each entry until a negative value is encountered.
- `do_nothing`: Performs a bitwise OR of zero onto d0.
- `parse_and_allocate_data_structures`: Parses a data stream starting with a specific header, identifying markers to extract length and pointer information into a table.
- `process_structure_list`: Checks a global word, then loops through a list of pointers, swapping bits in three longwords from a structure and calling another routine for each entry.
- `update_structure_counters`: Loops through a list of pointers, decrementing values in a sub-structure and adding those values to corresponding offsets in a main structure.
- `remove_empty_structures`: Loops through a list of pointers and, if a sub-structure is empty, shifts the remaining pointers in the list to fill the gap and decrements a global counter.
- `initialize_data_tables`: Writes several constant values to a series of memory locations, clears a longword at a specific address, and calls two different routines before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `modify_registers`: Performs logical OR operations on d0 and d3 and subtracts a longword from d6.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_d0_byte`: Performs a logical OR operation with zero on the low byte of d0.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `clear_d0_byte_alt3`: Performs two consecutive logical OR operations with zero on the low byte of d0.
- `setup_exception_handlers`: Performs bitwise OR operations on a longword pointed to by a0, clears a word in a global variable, calls another routine, and then populates several global variables and a series of memory locations with the address of a routine, branching to a different routine to populate more memory locations if a comparison with a value in a global variable is not less than eight.
- `write_exception_vector`: Writes the word 4e71 to an offset of minus two from a pointer retrieved from the stack and then returns from exception.
- `initialize_global_pointers`: Stores two calculated addresses into global variables and calls another routine before returning.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `configure_display_registers`: Writes several values to hardware registers to set up display parameters, including bitplane information, window start and stop positions, and DMA settings.
- `save_and_reconfigure_interrupts`: Copies several values from hardware registers and global variables into a structure, updates three global variables with routine addresses, writes specific bytes to a memory range, sets the INTENA register to 49272, and returns with interrupts enabled.
- `configure_interrupt_registers`: Writes the value 16416 to the INTENA register, calls another routine, writes 16416 to the INTENC register, writes 49272 to the INTENA register, and returns from interrupt.
- `update_interrupt_registers`: Writes values to the interrupt enable and mask registers and then returns from an interrupt.
- `modify_hardware_state`: Disables interrupts, modifies a value in a hardware register by inverting and masking it, writes three longwords to specific memory locations, and updates interrupt control registers before restoring the status register.
- `update_system_state`: Calls several routines via pointers from a table, increments a longword in a global variable, increments a word in a global variable, clears a word in a global variable, modifies the color register, and performs conditional updates to global variables based on byte comparisons.
- `handle_interrupt_request`: Sets a value in a hardware register, saves registers to the stack, tests a bit in a hardware register and a bit in a byte from memory, and either calls another routine or returns via an interrupt return.
- `execute_external_function`: Saves registers to the stack, loads an address from a global variable into a0, calls the routine at that address, restores the registers, and returns.
- `set_global_and_spin`: Sets a global word to a value with its high byte filled with ones, then busy-waits until that word becomes zero.
- `update_blt_parameters`: Calculates a pointer using a table, updates several hardware registers related to bitplane and DMA settings, and performs a loop that writes values to memory and hardware registers.
- `clear_memory_block_loop`: Loads a pointer from a global variable, adds 32000 to it, and enters a loop that clears 50 blocks of 32 bytes each to zero using multiple block move instructions, then returns.
- `configure_blitter_parameters`: Checks a bit in a hardware register and, if not set, writes values to several blitter registers, performs arithmetic on d0 and d1 to calculate a new value, and enters a loop that increments a pointer in a2 and updates a blitter register until a counter reaches zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 117 into d0.
- `check_status_value`: Compares a longword at a specific address to zero or one and returns if either condition is met; otherwise it returns.
- `write_constant_values`: Calls another routine twice and then writes the value 128 to four consecutive longword locations in memory.
- `update_input_state_and_counters`: Reads the mouse position from hardware registers, calculates the delta, and updates two 16-bit counters with clamping and wrap-around logic.
- `set_bits_and_decrement_pointer`: Sets the highest bit of a 32-bit register and performs an OR operation using a 16-bit constant on the memory location pointed to by a4 while decrementing that pointer.
- `iterate_table_range_with_callback_2`: Calculates a range between two 16-bit values and iterates through a table of pointers, calling another routine for each entry and updating a pointer by an offset of 40.
- `configure_blitter_transfer_parameters`: Waits for the DMA controller to be idle, then initializes blitter mask and color registers, sets the blitter mode and area size, and updates a destination address pointer based on a conditional check of a value in a table.
- `configure_blt_dma_transfer`: Checks the interrupt status bit to wait for a condition, then sets the Blitter width to 1024, clears a value in a global variable, and configures the Blitter address and size using values from registers and an offset in a0 before calling another routine.
- `enable_dma_and_mask_stack_values`: Enables DMA channels by writing to the DMA controller and calculates a bitmask from two values popped from the stack, storing the result in a global variable before jumping to another routine.
- `iterate_table_range_with_callback`: Calculates a range between two 16-bit values and iterates through a table of pointers, calling another routine for each entry and updating the current pointer by a fixed offset of 40.
- `iterate_pointer_table_2`: Compares two words, then enters a loop that calculates a pointer from a table using a value in d1, calls another routine, and updates the pointer using values from a6 and d3.
- `iterate_pointer_table`: Calculates a pointer from a table using a value in d1, then enters a loop that calls another routine and updates the pointer using values from d4 and d6.
- `iterate_pointer_table_decrement`: Calculates a pointer from a table using a value in d1, then enters a loop that calls another routine and updates the pointer by adding d4 and d6.
- `iterate_table_with_difference`: Calculates the difference between d1 and d2, then enters a loop that calls another routine and increments the pointer by 40.
- `execute_calculated_jump`: Copies a pointer from a0 to d7, adds 7968 to a0, and if d0 is not zero, performs a jump to an offset calculated by shifting d1 right by 4 bits, masking it with 254, pushing the result to the stack, negating it, and using it as a relative offset.
- `fill_memory_and_jump_via_table`: Fills ten consecutive longwords at the address in a0 with the value in d2, then performs a table lookup using a masked and shifted value from d1 to write a bitwise-inverted longword to a0, and finally executes a jump to a destination calculated from a value at an offset in a stack-pointer-relative address.
- `calculate_jump_offset`: Shifts the word in d1 right by 4 bits, masks it with a bitwise AND, pushes the result onto the stack, negates it, and performs a jump to an address calculated from the program counter and the negated value.
- `compute_and_store_products`: Loads three words from a table at an offset determined by a global variable, performs several multiplications and swaps using these values, stores the resulting words into a destination table, and finally jumps to the address in a6.
- `update_runtime_parameters_2`: Stores a longword from an address into a global variable, sets a global word to 5, stores a constant address into a global variable, and then branches.
- `update_runtime_parameters`: Stores d0, d1, and d2 into global variables, then performs a trap to update several other global pointers based on a calculated offset.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_andi_add`: Small routine of 2 instructions that masks d0 with 63, then adds d0 to d0.
- `apply_bitmask_to_buffer`: Calculates a destination pointer and a source pointer using various offsets and global values, then enters a loop that extracts a word from the source, shifts it right by a value determined by a global variable, and performs bitwise OR operations to write the resulting bytes into four specific locations within a buffer.
- `or_byte_to_offsets`: Performs a bitwise OR of the byte in d2 with the bytes at offsets 1, 7969, 16001, and 23969 relative to the address in a0.
- `or_byte_to_aligned_offsets`: Shifts the word in d2 right by 8 bits and performs a bitwise OR of the resulting byte with the bytes at offsets 0, 7968, 15999, and 23968 relative to the address in a0.
- `manipulate_d2_and_a0`: Inverts d2 twice, adds 40 to a0, and if d0 is not zero, swaps d2 and returns; otherwise it performs a swap on d2 and returns.
- `loop_until_zero`: Calls another routine and loops using d0 as a counter until d0 reaches zero, then returns.
- `clear_status_bytes`: Clears two consecutive bytes at a specific memory location and then performs several conditional branches based on the value of the first byte; otherwise it returns.
- `update_tables_and_pointers_2`: Clears a byte in a global variable, performs several calls to other routines, and then enters a loop that checks a global byte against constants and performs a jump to another routine if conditions are met.
- `copy_memory_block_2`: Copies a block of data from the memory location pointed to by a5 to the memory location pointed to by a6, using a loop count determined by a global variable.
- `initialize_data_structure`: Checks a constant at a pointer, then copies a sequence of words from a source to a destination and calculates a new pointer based on multiplication.
- `decrement_and_call_subroutine`: Checks a global variable and, if it is zero, initializes two global pointers from other global locations; if it is one, it performs a bitwise shift on a value, loads two words into registers, and calls another routine before decrementing the global variable and returning the updated pointer.
- `store_zero_bytes`: Contains a series of sixteen instructions that perform an logical OR on register d0 with zero.
- `store_zero_byte`: Contains a single instruction that performs a logical OR on register d0 with zero.
- `clear_data_register_d0_2`: Performs a bitwise OR operation on data register zero with zero.
- `process_data_buffer`: Calculates a size using registers d0 and d1, then enters a loop that copies bytes from a source buffer to a destination buffer based on values read from a control buffer.
- `perform_nothing_operation`: Performs a logical OR operation on a byte in register d0 with zero, effectively leaving the register unchanged.
- `store_address_to_global`: Pushes a6 to the stack, loads a pointer from a local offset into a6, stores that pointer into a global variable, restores a6 from the stack, and returns.
- `decrement_and_call_on_zero_crossing`: Decrements a 16-bit value at a pointer stored in a global variable; if the value transitions from zero to negative, it loads a 16-bit value from an offset of 2 and a pointer from an offset of 4 within that same structure, updates the global variable with the new pointer, and calls another routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `execute_trap_loop`: Triggers a trap and then branches to itself in an infinite loop.
- `execute_system_sequence`: Performs a series of subroutine calls and pointer manipulations, then checks a byte in a global variable to decide whether to copy a block of data from a constant to a global table.
- `copy_memory_block_via_jump`: Loads a source address from a constant into a5, loads a destination address from a memory location into a6, and jumps to another routine to perform a memory copy operation.
- `update_pointer_and_call_subroutine`: Adds a word value from d6 to the memory location pointed to by a0, sets d0 to a constant value, calls another routine, and then performs a jump to a relative offset.
- `initialize_and_call_subroutines`: Copies several words from a source to a destination, calls multiple subroutines, and then checks a byte in a global variable to determine if it should copy data from a global pointer to a local buffer.
- `clear_and_call_routine`: Sets the stack pointer to a4, clears three consecutive longwords at a specific memory location, calls another routine, and then returns.
- `iterate_and_call_routine`: Reads a word from a table, calls another routine if the word is negative, and then stores a byte into a global variable before returning.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_update_pointer`: Calls two subroutines, then if a global word is non-zero, copies a block of words from a constant to a pointer stored in a global variable and updates that pointer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 62 into d3.
- `copy_data_via_function`: Calculates a length, calls a routine via a pointer, stores the result in two global variables, and then copies a block of memory using a loop.
- `parse_and_transform_string_2`: Calls four different subroutines and then performs a longword call before branching.
- `initialize_global_values`: Calls a routine with a constant, stores the result in a global variable, then repeats this with a different constant, adds 64 to that constant, and stores the result in another global variable.
- `clear_global_state`: Copies a pointer to a global variable and clears three other global words and a longword.
- `calculate_address_difference`: Subtracts one global address from another and stores the resulting pointer into a third global variable before returning.
- `execute_jump_table`: Clears a word in a global variable, sets another global variable to all ones, and then performs a jump to an address found in a table if a byte comparison matches.
- `parse_and_transform_string`: Increments a global counter and enters a loop that reads bytes from a source, transforms them by subtracting 32 if they are within a specific range, and writes them to a destination buffer.
- `compare_and_copy_byte`: Loads a byte from a buffer pointed to by a0, compares it to 10, and branches based on the result.
- `copy_string_segment`: Copies a sequence of bytes from a source pointer to a destination pointer until a space or null byte is encountered.
- `calculate_offset_value`: Loads a byte from a table, performs a bitwise AND with fifteen, and adds the result to a value derived from multiplying zero by ten.
- `initialize_data_array`: Clears twenty-four bytes in a sequence and then writes a series of zeros followed by a word of all ones into a memory block.
- `call_external_routine`: Calls a routine at a specific address and then returns.
- `initialize_and_validate_string_data`: Clears several global variables and initializes a structure with specific constants, then performs a series of conditional checks by calling multiple routines.
- `check_constant_match`: Compares a long value at a pointer in a2 to a constant and, if it matches, sets a global byte to all ones before returning.
- `check_and_copy_string`: Compares a longword at a2 with a constant, and if it matches, writes a 16-bit value to a global variable and calls another routine.
- `compare_long_word`: Compares a long word at the pointer in a2 with a constant value and branches if they are equal, otherwise it returns.
- `store_byte_to_memory`: Stores a byte with the value 255 into a global variable and then returns.
- `verify_and_store_word`: Compares a longword at a2 with a constant, and if it matches, calls two different routines.
- `process_string_data`: Checks if a longword at a2 matches a constant and if the byte at offset 2 matches a specific value.
- `initialize_and_copy_string_data`: Checks a global word for a non-zero value and otherwise clears three global variables, then copies a string from a table to a global buffer.
- `copy_and_branch_string`: Checks a word at a specific location, then calls another routine, copies a byte from one address to another in a loop until a null byte is found, calls two more routines, and finally jumps to an address in a table.
- `execute_conditional_logic`: Saves registers, calls two routines, and then calls a third routine.
- `execute_conditional_logic_branch`: Clears a byte in a global variable and calls another routine, then checks if that byte is zero to decide whether to call a third routine or jump to a different location.
- `process_data_structure`: Checks a word in a global variable and, if zero, calls a routine via a pointer; otherwise, it copies a pointer from a4 to a global variable, loads a structure into a6, copies a word from offset 16 of that structure to a global variable, and then either calls another routine or clears a global word depending on whether the value is 4.
- `initialize_data_array_2`: Sets a global pointer, copies 16 bytes from a source to a destination, calls another routine, and then enters a loop that copies bytes from a0 or a1 to a global variable and checks a global byte against constants.
- `perform_logical_or`: Performs a bitwise OR on register d0 with zero.
- `clear_and_call_subroutine`: Copies a pointer from a6 to a0, calls another routine, clears a word at a specific global variable, clears a byte at another global variable, and then returns.
- `update_tables_and_pointers`: Copies a series of words from a source table to a destination table using a loop, then swaps two global longwords and two global pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_linked_list_values`: Enters a loop that reads a longword from a pointer, performs arithmetic to calculate a new value, and writes it back to the same location.
- `restore_registers_and_return`: Restores multiple registers from a stack pointer and then returns.
- `execute_sequence_of_calls`: Performs a series of four subroutine calls and then moves a long value from one memory location to another.
- `update_pointer_table_2`: Loads four addresses into registers and then writes ten 32-bit values into a table pointed to by a global variable.
- `branch_to_address`: Performs a relative branch to another location.
- `trap_and_restore_registers_2`: Performs a jump to an address calculated by adding an offset to the program counter.
- `increment_stack_pointer_and_trap`: Adds 6842 to a value stored on the stack and then executes a software interrupt.
- `jump_to_address`: Performs a relative jump to another location.
- `increment_stack_pointer_and_trap_2`: Adds 8530 to a value stored at a location on the stack and executes a software interrupt.
- `jump_to_offset`: Jumps to an address calculated by adding a constant offset to the program counter.
- `modify_stack_pointer_and_trap`: Adds a constant value to the value at the current stack pointer and executes a software interrupt.
- `trap_and_restore_registers`: Jumps to an address calculated by adding a constant offset to the program counter, then performs a sequence of stack operations, register saves, an addition, a trap, and register restores before returning.
- `push_addresses_and_branch_2`: Pushes two different word-sized addresses onto the stack and branches to another routine twice.
- `push_addresses_and_branch`: Pushes three different addresses onto the stack and branches to the same destination after each push.
- `push_address_and_branch`: Pushes an address onto the stack and then branches to another routine.
- `search_and_jump_table_3`: Calculates several differences and stores them in global variables, populates several global pointers with addresses of other routines, and then enters a loop that performs a division and stores the result in a table.
- `search_and_jump_table`: Searches through a list of words for a match against a value in a structure, then performs an indirect jump to an address calculated from a table offset.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `search_and_jump_table_2`: Loads a pointer from a1, compares a word from a1 to a value at that pointer, and if the comparison conditions are met, performs a series of pointer arithmetic and register saves before jumping to an address from a table.
- `process_matrix_data`: Retrieves three 16-bit words from a table using indices in d0, d1, and d2, calls a routine to calculate matrix elements, then calls another routine to scale and store values into a destination in a4.
- `loop_until_threshold`: Increments a pointer, calls another routine, and loops if the word at the pointer is less than 32767; otherwise it returns.
- `update_and_jump_indirect`: Loads a 16-bit word from offset 16 of the structure in a1, multiplies it by four, stores the result back at offset 16 of the structure in a1, and performs an indirect jump using the modified value.
- `execute_indirect_jump_2`: Performs an indirect jump to an address calculated from a table using a value from a pointer in a1.
- `shift_word_left_four`: Shifts a 16-bit word at the address in a1 left by four bits and stores the result back at that address.
- `calculate_and_jump`: Loads a word from a pointer in a1, increments the pointer, and if the word is not zero, compares it to 150, multiplies it by 6, adds the result to a1, loads a new word from the new a1, and performs a jump to a location calculated from doubling that word twice.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_offset`: Performs a short branch to another routine.
- `branch_to_offset_1e20`: Performs a short branch to offset 1e20.
- `branch_to_offset_1e34`: Performs a short branch to offset 1e34.
- `branch_to_offset_1e4c`: Performs a short branch to offset 1e4c.
- `branch_to_offset_1e5e`: Performs a short branch to offset 1e5e.
- `branch_to_offset_1e72`: Performs a short branch to offset 1e72.
- `branch_to_offset_1e80`: Performs a short branch to offset 1e80.
- `branch_to_offset_1e0a`: Performs a short branch to offset 1e0a.
- `branch_and_store_word`: Branches to another routine, stores a word value of thirty-eight into the memory location pointed to by a1, and then branches to another location.
- `process_data_sequence`: Writes a series of 16-bit values to a table in a1 and calls various subroutines based on the values written.
- `write_and_branch`: Writes a 16-bit value to the address in a1, calls two subroutines, loads a 32-bit value into d7, and branches to another routine.
- `update_word_values`: Stores a 16-bit word into the address in a1, calls two routines, loads a 32-bit word from the incremented address in a1 into d7, and branches to another location.
- `shift_word_left_three`: Loads a 16-bit word from the address in a1 into d0, shifts it left by 3 bits, stores the result back to the address in a1 incremented by 2, and returns.
- `shift_word_left_two`: Loads a 16-bit word from the address in a1 into d0, shifts it left by 2 bits, stores the result back to the address in a1 incremented by 2, and returns.
- `multiply_word_by_eighteen`: Loads a 16-bit word from the address in a1 into d0, multiplies it by 18, stores the result back to the address in a1 incremented by 2, and returns.
- `calculate_matrix_elements`: Calculates several values based on three 16-bit words retrieved from a table using indices in d0, d1, and d2, then stores the resulting 16-bit values into a destination pointed to by a2.
- `zero_and_scale_values`: Stores two 16-bit words of zero into the destination in a4, then calculates and stores two more 16-bit words derived from scaling values at offsets 12, 8, 4, and 14 of the source in a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_and_store_data`: Calculates and stores 16-bit values into a destination in a4 by scaling d0 and values from a table at a3 by 32734 and performing various additions and shifts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_and_store_zeroed_data`: Stores 16-bit values into a destination in a4 by summing d0, d1, and d2, scaling values from a table at a3 by zero, and performing various additions and shifts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_scaled_values_to_buffer`: Calculates two 16-bit values by doubling the contents of register d3 and storing them at the address in a4, then calculates a second 16-bit value by multiplying a 16-bit word from a table at offset 12 of a3 by 32766 and stores it at the next position in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_and_store_subset`: Stores 16-bit values into a destination in a4 by summing d3, d4, and d5, scaling values from a table at a3 by zero, and performing various additions and shifts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_and_store_mixed_data`: Stores 16-bit values into a destination in a4 by scaling d0 by zero, scaling d1 and d4 by 32734, and retrieving values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_calculated_values`: Calculates two values by summing three registers, shifting the result, and storing them at the pointer in a4, then performs a second calculation using values from a table at a3 and stores that result at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_scaled_registers`: Multiplies d1 and d4 by 32766, calculates two results by summing the products with other registers and shifting, and stores them at the pointer in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_single_scaled_value`: Calculates a value by summing d0, d1, and d2, shifting it, and stores the result at the pointer in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_large_structure`: Performs multiple calculations involving scaling registers and values from a table at a3, storing the resulting 16-bit words into a large sequence of memory starting at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_mixed_data`: Calculates and stores two values at a4 using d0, d1, and d2, then performs a second set of calculations using values from a table at a3 to fill several subsequent entries at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_derived_values`: Calculates two values using d3, d4, and d5 and stores them at a4, then calculates two more values using d0, d1, and d2 and stores them at a4, using values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_partial_table_data`: Calculates and stores two values at a4 using d3, d4, and d5, then calculates and stores two more values at a4 using d0, d1, and d2, using values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_coordinate_table`: Calculates and stores several 16-bit values into a destination buffer pointed to by a register, using arithmetic operations and multiplications on data read from a source structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_triple_calculated_values`: Calculates and stores three separate values at a4 by summing and shifting different combinations of registers d0 through d5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_two_scaled_values`: Calculates and stores two values at a4 by scaling d1 and d2 and summing them with d0, using values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_calculated_values_2`: Calculates two values based on d0, d1, and d2, stores them at the pointer in a4, then performs a second calculation using values from a table at a3 and stores the result at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_scaled_words`: Calculates a value by summing d0 and d1, doubling the result, and storing the high word at the address in a4, then loads five 16-bit values from a memory location pointed to by a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_table_derived_values`: Calculates and stores a value at a4 using d0, d1, and d2, then performs a second calculation using values from a table at a3 and stores the result at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_scaled_table_values`: Calculates two values by multiplying d2 and d5 by 32766 and stores them at the pointer in a4, then loads two values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_multiple_calculated_values`: Calculates and stores a value at a4 using d0 and d1, then calculates and stores a second value using d3, d4, and d5, and finally performs a third calculation using values from a table at a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_mixed_scaled_values`: Calculates a value using d5 multiplied by 32766 and stores it at a4, then calculates a second value using d2 multiplied by 32766 and stores it at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_summed_values`: Calculates a value from the sum of d3 and d4 multiplied by two and stores it at a4, then performs a second calculation using values from a table at a3 and stores it at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_single_scaled_value`: Calculates a value by multiplying d2 by 32766 and stores the result at the pointer in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_summed_scaled_value`: Calculates the sum of d0 and d1, multiplies it by two, and stores the result at the pointer in a4.
- `process_and_trap_data`: Pushes d0 to the stack, loads several words from a table into registers, stores those registers into a structure at a1, performs a trap, and then manipulates d0 before returning.
- `calculate_scaled_values_2`: Performs arithmetic on three 16-bit words from a0, stores results into a table at a3, and enters a loop that performs division and addition based on a comparison with a1, eventually calling another routine via a jump table.
- `return_integer_one`: Loads the value one into a data register and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `calculate_and_loop_values`: Moves three words from a0 into registers, performs several arithmetic operations including additions and multiplications, stores three long words into a destination pointed to by a3, and enters a loop that performs divisions and additions on the values before returning.
- `write_and_loop`: Stores a 16-bit word and a 32-bit word into a destination at a3, then enters a loop that decrements a value from the stack and branches based on a comparison.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_negative_one`: Loads negative one into register d0 and returns.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_complex_arithmetic`: Subtracts 4660 from registers d0, d1, and d2, performs several multiplications by zero, adds various register combinations, performs a swap and bit shifts, and finally divides d3 and d6 by d7 before adding two values from a table and returning.
- `compute_matrix_transformation`: Calculates a series of products and sums derived from three input values and a lookup table, stores the resulting values into a sequence of offsets in a structure, calls three subroutines and a trap, and finally updates several word-sized fields in a structure using the results from the input registers.
- `validate_power_of_two`: Loads a 32-bit value from a pointer in a4 and a 16-bit value from a2, then checks if the 16-bit value is a power of two up to 256 by comparing it against a sequence of values in a4, returning a value from a4 based on the result.
- `clear_d0_and_shift`: Clears register d0 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d1_and_shift`: Clears register d1 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d2_and_shift`: Clears register d2 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d3_and_shift`: Clears register d3 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d4_and_shift`: Clears register d4 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d5_and_shift`: Clears register d5 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_d6_and_shift`: Clears register d6 and performs a series of sign extensions and left shifts by 2, 3, 4, 5, 6, 7, and 8 bits.
- `clear_and_shift_register_d7`: Clears register d7 to zero and performs a series of arithmetic shifts and extensions to set the register to a specific constant value.
- `call_and_store_results`: Calls another routine multiple times with different values in a4 and stores the resulting d0 value into a series of long word offsets within a structure pointed to by a3.
- `populate_data_structure`: Calls several routines with different addresses in a4 and stores the resulting d0 values into various offsets of a structure pointed to by a3.
- `populate_data_table_from_subroutines`: Calls several subroutines sequentially and stores the resulting value from register d0 into two distinct locations in a table for each call.
- `decrement_and_branch`: Subtracts the value in a2 from itself and branches to another location.
- `process_pointer_list_entry`: Reads a 16-bit value from an offset of 12 in a structure, negates it, and performs a series of arithmetic operations on registers before storing the results into the structure at offsets 0, 2, and 4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_scaled_values`: Multiplies a 16-bit value in d2 by 32766 and stores the result in a table pointed to by a1, then performs a bitwise shift and logical OR on the result before returning it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position`: Reads a 16-bit coordinate from a table using an index in d0, adds a value from a global variable to it, and writes the result back to the table before incrementing the index and looping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_vector_data`: Performs a series of fixed-point arithmetic operations involving multiplications by 32734 and subtractions to derive new coordinate values, then either stores these values into a table and a pointer list at a specific index or updates a global pointer to point to the calculated data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_coordinate_data`: Performs a series of arithmetic operations involving signed and unsigned values to derive new coordinates, then either stores the resulting data into a table and increments a global counter or prepends the pointer to a linked list in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_coordinate_bounds`: Performs complex arithmetic on signed 16-bit values to determine a bounding box or range, then stores the resulting coordinates into a structure and updates a global table if a counter is below 150.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_coordinate_structure`: Performs arithmetic on three 32-bit values to derive a new coordinate set, stores the result in a three-element array, and if certain bounds conditions are met, writes the array pointer and a calculated value into a table and increments a global counter; otherwise, it prepends the array pointer to a linked list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_values`: Calculates a value by multiplying a register by 32766, stores it and two other values into a structure pointed to by a6, and then performs conditional arithmetic to update those same structure members based on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_linked_list`: Performs arithmetic on d0, d1, d3, d4, d6, and d7, and either stores a value into a table or inserts a pointer into a linked list at a1.
- `clear_global_variables`: Clears a 16-bit value at a specific memory location and a 32-bit value at another memory location, then returns.
- `dispatch_table_elements`: Compares a value from a global variable, calls two routines, and enters a loop that retrieves a function pointer from a table and calls it until a pointer threshold is reached.
- `sort_data_tables`: Performs a loop that compares and moves 16-bit and 32-bit values between two tables until a specific end condition is met.
- `sort_elements_recursively`: Implements a recursive quicksort algorithm that swaps 16-bit and 32-bit elements within a table based on a pivot value retrieved from a global array.
- `execute_indirect_jump`: Loads a 16-bit word from the address in a0 into d0, calls a routine, and then either branches or performs an indirect jump using d0 as an offset.
- `calculate_area_and_call_indirect`: Loads four 16-bit words from a list pointed to by a0, uses two of these words as indices into a table to fetch four additional 16-bit values, calculates the signed product of the differences between these fetched values, and if the result is non-negative, increments the pointer in a0 by a value from the list before performing an indirect call using a 16-bit word loaded from the updated pointer location.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_call`: Calculates an address by adding a word from a buffer to a base pointer, performs a division using a table, and calls another routine.
- `process_segments_and_call`: Loads values from a pointer in a0, performs a calculation using a table, calls one routine, and then calls another routine before branching back.
- `calculate_offset_and_call`: Adds a word from a list pointed to by a0 to a constant base address in a4, loads a pointer into a6, sets d0 to 3, calls another routine, and branches to another location.
- `calculate_offset_and_call_2`: Adds a word from a list pointed to by a0 to a constant base address in a4, loads a pointer into a6, sets d0 to 4, calls another routine, and branches to another location.
- `calculate_offset_and_call_3`: Adds a word from a list pointed to by a0 to a constant base address in a4, loads a pointer into a6, loads a word from the address in a0 into d0, calls another routine, and branches to another location.
- `execute_jump_table_2`: Loads a 16-bit offset from the address in a0 and performs an indirect jump to that location.
- `process_geometry_and_call_routine`: Loads four 16-bit values from a list in a0 into registers, performs a geometric cross-product calculation using values from a global table, and then calls another routine using a pointer found at an offset within the list.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_call`: Calculates an address by adding a word from a buffer to a base pointer, checks a word at an offset in a table, and either calls another routine or branches.
- `calculate_table_offset`: Adds a 16-bit offset from a pointer in a0 to a base address in a4, calls another routine, and if the return value is less than 3, calls another routine and branches or performs a jump to another location.
- `calculate_pointer_offset_and_branch`: Adds a 16-bit value from a pointer in a0 to a base address in a4, then calls another routine and branches to another routine if the return value is 3 or greater; otherwise it calls a third routine and returns.
- `execute_indirect_call_2`: Calls another routine, and if the return value is greater than 2, calls another routine, then performs an indirect call using a pointer fetched from a table.
- `sort_and_validate_coordinate_set`: Iterates through a list of 32-bit values to find minimum and maximum 16-bit signed components, sorts the values into two separate buffers, performs a cross-product geometric check, and calls validate_and_interpolate_range if the bounds and orientation are valid.
- `sort_and_validate_geometry_data`: Iterates through a buffer to find minimum and maximum values while copying data into two separate tables.
- `process_and_sort_data`: Calculates offsets based on values from a buffer, copies data into two destination tables while performing comparisons to find minimum and maximum values, and performs an indirect jump through a register.
- `sort_and_copy_table_entries`: Reads a sequence of values from a source list pointed to by a0, using a table in a6 to resolve indices, and sorts these values into a new destination buffer while tracking the minimum and maximum values found.
- `validate_and_interpolate_range`: Validates that several 16-bit signed values fall within specific global bounds and performs a cross-product check on a set of values in memory; if valid, it calculates a range, populates a table with interpolated values, and calls another routine via an indirect jump.
- `populate_complex_coordinate_table`: Calculates a series of 16-bit coordinate pairs by iteratively adding values from registers to a base value and storing the results in a table pointed to by a6.
- `find_and_store_range_extremes`: Searches through two memory ranges to find the minimum and maximum 16-bit values, stores these values as a 32-bit word at a destination pointer, appends a constant value, and calls another routine via a jump table.
- `loop_and_call_routine`: Calculates an end address by adding a 16-bit value in a3 to a2, then loops through a table in a5, popping two 16-bit words into d1-d4 per iteration and calling a routine at a fixed address until the pointer reaches the end address.
- `resolve_indexed_table_values_2`: Iterates through a list of indices, fetches values from a table at a6, and calls either one or two different routines based on comparisons with a constant value.
- `process_indexed_values`: Loads two 32-bit values from a table using offsets in d1 and d2, calls another routine, and then writes two 16-bit values to a destination pointer in a5 based on range checks and bit manipulation.
- `resolve_indexed_table_values`: Fetches two 32-bit values from a table using offsets in d1 and d2, then performs a binary search on those values.
- `calculate_scaled_offsets`: Pops three 16-bit words from the stack, loads two 32-bit values from a table using d1 and d2, calls another routine, and performs bit shifts and additions before storing the results back to the stack.
- `pop_three_words`: Pops three 16-bit words from the stack into d2, d1, and d0, then sets the second bit of the condition code register and returns.
- `perform_binary_search`: Performs a binary search using d4, d5, d6, and d7 as working values against a limit in a3, updating the values through repeated division and comparison until the limit is reached or the loop terminates.
- `serves_as_data_table`: Acts as a sequence of zeroed bytes used as a data table by other routines.
- `empty_data_table`: Does nothing as it consists of a series of instructions that perform logical OR operations with zero on a data register.
- `push_function_pointers_to_stack`: Checks several 16-bit signed values against global constants and their respective counterparts; if a value exceeds its constant or is negative, it pushes a specific function pointer onto the stack.
- `clamp_values_in_loop`: Loops through a series of 16-bit words from a pointer in a6, clamping each value between negative 16383 and 16383, then returns.
- `compare_and_store`: Compares d1 and d3, and if d1 is not less than or equal to d3, stores d3 at an offset of negative 2 from a6 and enters a loop controlled by d6.
- `interpolate_signed_values`: Decrements d0 and returns if it is not greater than zero; otherwise, it iterates through a list of 32-bit values from a5, calling interpolation routines based on the sign of the values and storing results in a1.
- `interpolate_short_values`: Calculates an interpolated 16-bit value based on d1, d2, and d3, then stores the result and a zero at the pointer in a1 and returns.
- `interpolate_short_values_2`: Calculates an interpolated 16-bit value based on d1, d2, and d4, then stores the result and a zero at the pointer in a1 and returns.
- `interpolate_signed_with_limit`: Decrements d0 and returns if it is not greater than zero; otherwise, it iterates through a list of 32-bit values from a5, comparing them against a constant in d6 and calling interpolation routines based on the sign and comparison results.
- `interpolate_two_points`: Calculates an interpolated 16-bit value using d1, d2, d3, and d6, then stores the result and the value from d6 at the pointer in a1 and returns.
- `interpolate_two_values`: Performs linear interpolation between two 16-bit values based on a ratio derived from two other 16-bit values, storing the resulting pair of 16-bit words into memory pointed to by a1.
- `interpolate_signed_32bit`: Decrements d0 and returns if it is not greater than zero; otherwise, it iterates through a list of 32-bit values from a5, comparing the sign of the values and calling interpolation routines to store results in a1.
- `calculate_linear_intersection`: Calculates a linear intersection point using d1, d2, and d3, then stores a 16-bit result and a secondary 16-bit value at the pointer in a1 and returns.
- `calculate_linear_interpolation`: Performs linear interpolation using 32-bit values split into 16-bit halves and stores the resulting 16-bit word and a zero byte into a destination pointed to by a1.
- `interpolate_signed_with_fixed_limit`: Decrements d0 and returns if it is not greater than zero; otherwise, it iterates through a list of 32-bit values from a5, comparing them against a value derived from a constant in a2 and calling interpolation routines based on the sign and comparison results.
- `interpolate_with_fixed_point`: Calculates a value using 16-bit arithmetic and 32-bit fractional components, then stores a 16-bit word and a 16-bit result into a destination pointed to by a1.
- `interpolate_with_offset`: Computes a linear interpolation between two 32-bit values using a third 16-bit value and stores the result as two 16-bit words at the location pointed to by a1.
- `clamp_and_interpolate_segments`: Iteratively processes a series of coordinate pairs, performing linear interpolation and clamping values within specific bounds defined by global variables and looping until conditions are met.
- `tiny_nop_rts`: Small routine of 2 instructions that does nothing for one instruction, then returns.
- `calculate_intersection_intervals`: Calculates the intersection of two integer ranges and stores the resulting interval as a pair of 16-bit values in a destination buffer.
- `fast_integer_square_root`: Implements a bitwise algorithm to calculate a square root of a 32-bit value in d3 using a loop that shifts and subtracts values.
- `populate_division_table`: Sets a global variable to a specific value and then enters a loop that populates a memory range starting at an offset from that global variable with values calculated by dividing a constant by an incrementing counter, until the counter reaches two thousand four hundred ninety-six, then calls another routine.
- `populate_division_table_2`: Calls a routine to get a pointer, sets a 32-bit value at that pointer, and then fills a memory range with values calculated by dividing 32768 by an incrementing counter.
- `set_registers_and_branch`: Sets the bits of d0 and d7 using immediate values and returns if d2 is greater than 16.
- `scale_and_call_function`: Divides two 32-bit values by a 16-bit value, adds constants from global memory, and calls a function at an address calculated from a1 and a constant.
- `clear_memory_blocks`: Calls a routine via a pointer, then enters a loop to clear multiple 32-bit blocks of memory at a location derived from a4, using a count derived from d0.
- `update_buffer_pointer`: Increments a value in d0, masks the lowest bit, and updates a pointer and a counter at offsets 12 and 16 of a structure in a4.
- `call_and_branch`: Calls another routine and branches if the result is not zero, otherwise it returns.
- `execute_function_list`: Iterates through a list of pointers in a0, calling each function via a pointer in a1 until a null pointer is encountered.
- `copy_bytes_with_counts`: Copies bytes from a source pointed to by a1 to a destination pointed to by a0, using counts provided in the high and low words of d0, and loops until both counts reach zero.
- `setup_nested_loop_counters`: Moves a value from d2 to a1, sets d2 to the value in d1, swaps the high and low words of d1, and performs two descending loops using d2 and d1 as counters before returning.
- `fill_nested_blocks`: Fills a block of memory starting at a0 with the byte in d0, using d1 and d2 as nested loop counters.
- `copy_string_unrolled`: Copies bytes from a1 to a0 in unrolled chunks of up to six bytes at a time, stopping when a null byte is encountered.
- `find_null_terminator`: Searches for a null byte in a block of memory pointed to by a0, returning the pointer to the end of the block or a pointer to the first null byte found.
- `compare_strings_with_counts`: Compares two strings from a0 and a1 for equality, using counts from the high and low words of d0, and returns the difference of the first non-matching byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `execute_callback_chain`: Calls a routine at a pointer in a7, checks if the result is negative, calls another routine at a fixed address, and then calls a third routine using the result of the second.
- `execute_conditional_callback`: Calls a routine at a pointer in a7, checks if the result is negative, calls a second routine at a relative offset, and then calls a third routine using the result of the second.
- `process_structure_updates`: Checks if a long value at offset 1830 of a structure in a4 is less than a value at offset 12016, then conditionally calls several routines to update various pointers and values at offsets 1800, 1804, 1808, 1812, 1816, and 1820 of the structure in a4, and finally calls a routine four times with d0 set to 16, 17, 18, and 19.
- `check_status_byte`: Reads a byte from a table at offset 392, compares it against specific constants, and either writes a value to a global offset or returns a status code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_pointer_table_entry`: Saves registers to the stack, stores the value in d0 into the first word of a structure pointed to by a0, stores the value in d7 into the second word of that structure, and stores the pointer to the structure in a global variable at offset 2700 of a structure in a4 before calling another routine.
- `test_and_return_pointer`: Calls a routine, tests the result in d0, and returns the value in d0 or triggers an illegal instruction if the result is zero.
- `traverse_and_split_linked_list`: Calculates a threshold value, traverses a linked list starting at offset 2700 of a structure in a4, and if a node's value is less than the threshold, it either splits the node or replaces it before calling another routine.
- `manage_linked_list_insertion`: Traverses a linked list to find an insertion point based on a value in a0 and a list head at offset 2700 of a structure in a4, then performs node splitting or insertion and calls another routine.
- `find_max_in_list`: Traverses a linked list starting at an offset of a4, finds the entry with the largest value in its second word, updates two global pointers, and calls a routine.
- `execute_sequence_of_routines`: Clears a 16-bit global variable, calls a routine at a fixed address, moves a value from a global variable to a pointer, calls another routine, and finally calls a third routine using the value from the global variable.
- `iterate_function_table`: Iterates through a table of pointers starting at offset 9902 of a structure in a4, calling a function pointed to by a0 for each entry where the word at offset 2 of the entry is non-zero, incrementing the pointer by 14 each time.
- `decrement_table_entries`: Iterates through a table starting at offset 9902 of a structure in a4, decrementing a word at offset 4 of each entry and updating offset 2 and offset 6 with the new value, while potentially adjusting the table start pointer if a specific condition is met.
- `process_function_table_entries`: Sets up a table of entries at offset 9910 of a structure in a4, iterates through a provided list of bytes, and for each byte, populates a structure with metadata and calls various routines based on the byte value and structure state.
- `process_pointer_and_data`: Calls a routine to test a pointer, then performs several conditional checks and calls to other routines to update values at offsets 11032 and 1804 of a structure in a4, eventually calling a routine with a pointer in a0 and a value in a1.
- `execute_sequential_calls`: Clears a word at a global variable, calls a routine, moves a value from a global variable to a register, calls another routine, moves another value from a global variable to a register, calls a third routine, and finally calls a fourth routine.
- `update_table_offset`: Calls a routine, calculates a value based on the result, adds 8 to a 16-bit word at offset 2930 of a structure in a4, and calls another routine.
- `initialize_structure_parameters`: Calls two routines, clears a word at offset 12022 of a structure in a4, and performs a series of conditional calls and assignments to various offsets of the structure in a4, including setting several word and longword values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_if_nonzero`: Checks if a word at offset 72c of a structure in a4 is non-zero; if so, it calls a routine and then clears that word.
- `process_string_buffer`: Processes a buffer by skipping certain characters, copying bytes to a destination buffer while enforcing a minimum value of 32 and a maximum length, and performing case conversion on specific byte ranges.
- `extract_parenthesized_string`: Searches for a string within parentheses in a source buffer and copies the content between the parentheses to a destination buffer, updating the source pointer.
- `skip_whitespace_characters`: Iterates through a string, calling another routine whenever a space or tab character is encountered.
- `parse_and_dispatch_commands`: Checks a byte at a pointer in a0, validates its range, and then enters a loop that calls a routine from a table in a4 based on an index.
- `process_cheat_commands`: Parses a string to execute various commands, updating multiple offsets in a structure and potentially modifying a global variable.
- `initialize_state_constants`: Calls two routines and then writes several constant word values to offsets 2ed8, 2eda, 2ede, 2ee0, and 2ee2 of a structure in a4.
- `calculate_interpolated_offset`: Checks if a 16-bit value at offset 11994 of a structure is less than 199, then performs a series of bitwise shifts and arithmetic operations involving values from offsets 3134 and 11992 to compute a new value, which is then stored back into offset 11994 after calling two subroutines.
- `update_offset_by_value`: Reads a word from offset 3134 of a structure in a4 and adds a specific constant to the word at offset 11994 of the same structure based on the value read.
- `parse_string_with_tokens`: Iterates through a string in a0, handling specific characters by calling routines to search a table, copy strings, or skip spaces, and writes the resulting characters into a stack-based buffer.
- `search_table_for_pointer`: Iterates through a table in a structure in a4, calling a routine to check entries, and returns a value from a table or an offset when a zero entry is found.
- `copy_string_bytes`: Copies bytes from a source buffer in a1 to a destination buffer in a0 until a null terminator is reached, then places a null terminator in the destination.
- `copy_string_until_space`: Copies bytes from a source buffer in a1 to a destination buffer in a0 until either a space character or a null terminator is encountered, then places a null terminator in the destination.
- `null_terminate_and_skip_spaces`: Calls a routine to copy a string, writes a null byte to the first byte of the destination, and then iterates through the source string to skip any space characters following the null terminator before calling another routine.
- `decrement_and_call_if_empty`: Checks if bytes at offsets 392, 384, and 385 of a table are non-zero, then if d7 is non-zero, decrements d7 and calls another routine with d0 set to 1.
- `dispatch_by_table_value`: Reads a word from offset 3134 of a structure in a4 and calls one of four different routines based on the value, using d1, d2, and d6 as arguments.
- `dispatch_table_actions`: Calls a routine, then iterates through a list of bytes, performing different actions such as calling a routine via a pointer in a structure or updating a field at offset 12004 of a structure in a4 based on a value at offset 3134.
- `execute_dispatch_actions`: Calls three different routines via a jump table, stores the value 95 at an offset in a stack-based array, clears a byte at offset -2 of the stack frame, and calls another routine before returning.
- `update_structure_field`: Reads a word from offset 3134 of a structure in a4 and writes a specific constant to offset 12004 of that structure based on the value read.
- `initialize_and_process_state`: If offset 1824 of a structure in a4 is zero, it clears offset 3132, calls several routines to initialize data, and eventually calls another routine with a value from a pointer and a value from the stack.
- `dispatch_string_commands`: Calls several routines to initialize values at offsets 3134, 3136, 11996, 11994, and 11992 of a structure in a4, then performs multiple calls to other routines to process data and eventually copies strings based on byte comparisons.
- `process_string_buffer_2`: Iterates through a null-terminated string in a0, calling another routine for each byte and incrementing a value in a structure at offset 12004 by the length of the string.
- `parse_and_update_string_buffer`: Processes a string buffer by writing bytes, performing calculations involving a multiplier at offset 12004 of a structure in a4, and updating the buffer with specific characters based on values retrieved from a lookup table and various conditional checks.
- `search_table_for_zero`: Iterates through a table starting at an offset of 3190 from a base address, calling a routine to perform bitwise operations on each entry, and returns the count of entries processed until a zero value is encountered or a specific condition is met.
- `update_offset_from_table`: Checks a word at offset 20796 of a structure in a4, and if non-zero, calls several routines to set values at offsets 3134, 11996, 11994, 11992, and 1830.
- `process_complex_data_structure`: Performs a series of calls and data manipulations involving a table at offset 1834, including conditional loops that decrement a value in d7 and write to a stack-based structure, before clearing a value at offset 12006 of a global variable.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `validate_pointer_and_word`: Calls a routine and checks if the return value is 256; if so, it retrieves a pointer from a structure offset, checks if the word at that pointer equals 4662, and if so, calls another routine with a length of 128 and a calculated address, otherwise it returns -1.
- `verify_memory_address`: Pushes the value in a5 to the stack, loads a pointer from a table into a0, calls another routine, and branches based on whether d0 equals 256.
- `write_and_verify_value`: Calls another routine, writes 4662 to the address in a5, calls a third routine, and then calls a fourth routine using a pointer from a4 and the address in a5, returning 0 if the result is negative.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to a5, then returns.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `store_routine_result`: Loads a pointer from offset 3628 of a structure in a4, calls another routine, and stores the returned value into offset 3656 of the same structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pop_two_longwords`: Pops two longwords from the stack pointed to by a0 and moves the second longword into d0.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `perform_null_operations`: Performs two bitwise OR operations on d0 with zero.
- `clear_d0_repeatedly`: Performs four consecutive logical OR operations with zero on register d0.
- `update_audio_registers`: Iterates through a table to compare values against a global variable, then updates several hardware registers including audio parameters and DMA control, and writes values to a destination buffer based on a loop count.
- `initialize_attribute_table`: Clears a global word, calls another routine, writes 47 to a hardware register, and then enters a loop that writes zero to eight consecutive longwords starting at a specific memory location.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `handle_string_match`: Compares a four-byte sequence at a pointer to the string 'ANDY', and if it matches, increments a global word, updates a hardware register, calls a routine, and modifies two global pointers before returning a calculated address.
- `store_register_values`: Clears the low byte of d0, writes d0 and d1 to several global word locations, decrements a global word at offset 8829a, and stores bytes from a0 and d1 into other global locations.
- `copy_and_decrement_globals`: Copies a block of data from one location to another using a loop, decrements a global word, and if that word reaches zero, resets it to a new value and calls a routine to update another global variable.
- `clear_d0_once`: Performs a logical OR operation with zero on register d0.
- `clear_data_register_d0_3`: Clears the least significant byte of data register d0.
- `clear_data_register_zero_2`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `clear_global_words`: Clears two 16-bit words at the memory locations pointed to by the values in a0 and a6.
- `tiny_ori_3`: Small routine of 1 instruction that merges 64 into d1.
- `initialize_hardware_and_tables`: Calls several routines, populates a table with incrementing values starting from zero, clears a block of memory at offset 14a, and sets several hardware registers to specific values including bitplane addresses and DMA control.
- `update_sprite_attribute_tables`: Checks if a value from a register has changed, then updates a table of pointers and a secondary table of attribute data using a loop.
- `initialize_data_pointers`: Clears a 16-bit word at an offset of 65439 in a global variable, loads a constant address into a0, calls another routine, and then loads a different constant address into a0.
- `process_data_list`: Iterates through a list in a0, searching for a specific 16-bit value in the high word of each longword, and if found, modifies the list entries and potentially updates a global structure based on a sign test.
- `decrement_and_call_transform`: Checks if a global word is positive, and if so, calls a routine with a pointer and a negated version of that word, then decrements the global word and returns.
- `setup_transform_parameters`: Loads a 16-bit word from an offset of 4228 in the program counter relative memory into d6, loads a constant address into a1, calls another routine, loads a global address into a0, calls another routine, and returns.
- `transform_pixel_data`: Loops a specified number of times, extracting nibbles from a source table in a0, adding offsets from d0, d1, and d2, clamping the results between zero and fifteen, and storing the packed result into a destination table in a1.
- `update_and_test_globals`: Stores a0 into a global variable, sets a second global variable to fifteen, sets a third global variable to zero, and returns if the second global variable is zero; otherwise it returns.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_structure`: Stores a0 into a global variable, sets a second global variable to fifteen, and sets a third global variable to zero before returning.
- `decrement_and_call_transform_alt`: Checks if a global word is positive, and if so, decrements two different global words and calls a routine before returning.
- `populate_attribute_tables`: Populates two parallel tables with interleaved values by reading from a source table and applying various mathematical transformations and constant offsets based on the value of a global variable.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `copy_stack_words_to_table`: Checks if a 16-bit word at offset 4 of the stack frame is zero; if not, it loads a global address into a0 and copies four 16-bit words from offsets 6, 8, 10, and 12 of the stack frame into that table sequentially.
- `perform_logical_or_2`: Performs a bitwise logical OR on register d0 with zero twice.
- `calculate_relative_offsets`: Calculates several values based on the difference between two words at a pointer in a0, and stores the results in various global variables and a word at offset c0.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blitter`: Checks bit 6 of a status register and either sets several hardware registers to specific values including the blitter address and control words, or sets the blitter address to an offset of 16384 from a base address and updates the blitter control and size registers before returning.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `execute_chained_calls`: Saves registers to the stack, calls another routine with d0, calls a second routine with a0 in d1, and then calls a third routine with the result of the second routine in d0 before restoring registers and returning.
- `execute_parameterized_calls`: Saves registers to the stack, calls another routine with d0, calls a second routine with a0 in d1 and a1 in d2, calls a third routine, clears d0, and returns.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blitter_2`: Writes 1024 to a hardware register, sets d0 to 12820, loads an address from a pointer at offset 192 into a0, checks a bit in a hardware register, and depending on the result, writes several values to hardware registers including DMA and Blitter control registers, calls another routine, and performs further register writes and a second call to another routine before returning.
- `clear_relocation_table`: Compares a global variable against eight and, if greater, enters a loop to zero out a large block of memory starting at an offset from a pointer in a0 before setting a global variable to a specific value; otherwise, it performs a long sequence of manual zeroing operations on a memory range starting at an offset from the pointer in a0 and then enables DMA channels.
- `clear_indexed_tables`: Clears two 16-bit values at offsets determined by a word from the stack into two separate tables, one of which is located at an offset of 8000 from a global pointer, and repeats this process up to three times if the index remains non-negative.
- `process_pixel_table`: Loads addresses from relative offsets into a0 and a3, performs arithmetic on values loaded from a0 into d0, d1, and d2, writes the results back to a0, and enters a loop that performs bitwise operations and memory writes using a table lookup and a pointer in a1, calling another routine if a condition is met, and repeats this 29 times before returning.
- `process_pixel_table_alt`: Loads addresses from relative offsets into a0 and a3, performs arithmetic on values loaded from a0 into d0, d1, and d2, writes the results back to a0, and enters a loop that performs bitwise operations and memory writes using a table lookup and a pointer in a1, calling another routine if a condition is met, and repeats this 9 times before returning.
- `process_pixel_table_or`: Loads addresses from relative offsets into a0 and a3, performs arithmetic on values loaded from a0 into d0, d1, and d2, writes the results back to a0, and enters a loop that performs bitwise operations and memory writes using a table lookup and a pointer in a1, performing bitwise OR operations, and repeats this 99 times before returning.
- `copy_and_calculate_offset`: Performs a bitwise transformation on a value in d6, copies three 16-bit words from a source pointed to by a calculated address into a destination in a0, and stores a calculated 32-bit offset into a 16-bit word at offset 6 of the destination structure.
- `fill_buffer_pattern_rotate`: Initializes several registers and a loop counter, then enters a loop that writes two 16-bit words to the buffer in a0, updates values in d1 and d7 using a rotate operation, and performs arithmetic on d4 and d2, repeating this 159 times before returning.
- `fill_buffer_with_pattern`: Fills a buffer starting at a0 with a pattern of 16-bit words, where the values are updated based on a loop counter and a signed input, and if the loop terminates early, it fills the remaining space with 65535.
- `fill_buffer_pattern_ror`: Initializes several registers and a loop counter, then enters a loop that writes two 16-bit words to the buffer in a0, updates values in d1 and d7 using a rotate right operation, and performs arithmetic on d4 and d2, repeating this 159 times before returning.
- `fill_buffer_pattern_rol`: Initializes several registers and a loop counter, then enters a loop that writes two 16-bit words to the buffer in a0, updates values in d1 and d7 using a rotate left operation, and performs arithmetic on d4 and d2, repeating this 159 times before returning.
- `execute_initialization_sequence`: Calls four different routines and then stores a byte at a location pointed to by a0.
- `initialize_global_pointers_2`: Calls a routine five times with different constant arguments and stores each returned longword into five specific global variables.
- `execute_stored_routines`: Checks if a global variable is non-zero, and if so, calls five different routines using addresses stored in global variables before clearing the first global variable.
- `transform_and_copy_data`: Performs three loops that call another routine to copy data from various source addresses, then adds a value derived from a bitwise AND of a register and 60 to a destination at offset 6 of a structure, and finally stores a constant in a global variable.
- `fill_memory_with_ones`: Fills a memory area starting at a pointer from a global variable with 32-bit values of all ones using a loop controlled by a counter.
- `fill_buffers_with_patterns`: Executes eight consecutive loops, each calling a different routine to fill a buffer with patterns using a loop counter and specific values in d0 and d3.
- `update_function_pointers`: Stores a longword from an offset of a word into a global variable and then updates that word with a new constant value.
- `execute_stored_address`: Loads a longword from a global variable into the program counter to call another routine.
- `perform_zero_or_operation`: Performs a bitwise OR operation on d0 with zero.
- `update_status_byte`: Saves registers, calls three different routines, stores the value 255 at a location pointed to by a global variable, and updates a global word with a value from d6.
- `update_sprite_tables`: Initializes several pointers, performs multiple calls to update data, and fills a memory range with bytes using a loop.
- `execute_trap_and_rotate`: Clears d1, executes a trap, loads a pointer from a3 into a7, increments a4 by 8, and performs bitwise operations on d0.
- `compare_word_with_register`: Performs bitwise OR operations on d0, then compares a word at offset 6 of a pointer in a0 with d0 and returns if the comparison condition is met.
- `update_bitplane_registers`: Calculates offsets and values from a structure, performs arithmetic to determine bitplane mask and address updates, and calls a routine to update hardware registers if certain bounds and conditions are met.
- `update_hardware_registers`: Calculates an offset based on the negation of a register, then if a condition is met, writes a value from a table to the bitplane address register and 65535 to the bitplane mask register before calling another routine.
- `update_hardware_registers_alt`: Calculates an offset using a global variable and a register, then if a condition is met, writes a value from a table to the bitplane mask register and 65535 to the bitplane address register before calling another routine.
- `clear_data_register_zero_3`: Sets the least significant byte of data register zero to zero.
- `compute_scaled_quotient_remainder`: Computes a scaled product from the input word stored into a global variable, extracts its high word, and returns the quotient and remainder of dividing that value by the input word.
- `copy_data_block`: Stores a value from a global variable into another global variable, calls a routine at a fixed address, and then copies a block of bytes from a source address to a destination address using a count retrieved from a global variable.
- `process_data_and_clear_hardware`: Iterates through a data block to find a specific byte value, then performs a series of calculations to clear a table of 30 longwords, sets a bit in a hardware register, and clears several hardware registers including audio and DMA control registers.
- `update_audio_state`: Increments a byte in a global variable, compares a byte from a memory location to another byte, and then either calls another routine and branches or clears a byte in a global variable and calls another routine.
- `update_audio_channel_registers`: Updates the four audio channel registers by calling another routine four times with specific parameters, then calculates a value based on a byte from a table and a global variable to update a 16-bit value in a global variable.
- `process_audio_command_structure`: Extracts a command index from a structure in a6 to fetch parameters from a table in a3, then updates various fields in the structure including pointers, lengths, and hardware registers for audio and DMA. It also performs a loop to search a table for a value and updates a global variable and hardware registers based on specific nibble values.
- `dispatch_structure_command_nibbles_2`: Calls another routine and then branches based on a nibble extracted from offset 2 of a structure in a6 to perform various data manipulations, including table lookups, arithmetic updates to structure fields, and conditional pointer or value updates.
- `update_structure_parameter_from_nibble`: Extracts a 16-bit word from offset 16 of a structure in a6 and stores it into offset 6 of a structure in a5.
- `search_table_for_value`: Calculates an offset into a table using a byte from offset 18 of a structure in a6, iterates through the table to find a value that satisfies a comparison against a word at offset 0 of the structure, and updates offset 24 of the structure in a6.
- `update_scaled_value`: Updates a 16-bit value in a structure based on a signed byte, performs a loop to find a value in a table using a multiplier and index, and stores the result in a location pointed to by a5.
- `update_scaled_coordinate`: Calculates a scaled value by reading an index and multiplier from a structure in a6, applying a bitwise transformation and sign check, then stores the result in a 16-bit field at offset 6 of a structure in a5 and updates a byte at offset 27 of the structure in a6.
- `dispatch_structure_command_nibbles`: Calls another routine and then branches based on a nibble extracted from offset 2 of a structure in a6 to perform various data manipulations, including updating counters, modifying bitfields at offset 3, or updating a hardware register via the CIAA port.
- `modify_structure_field`: Extracts a 4-bit value from a byte at offset 3 of a structure and stores it in offset 18 of the same structure.
- `update_counter_and_pointer`: Checks a byte at an offset of a structure, adds a value from a table to a byte in the structure, and if a specific bit is set, updates a pointer in the structure and modifies a byte at the new pointer location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_22`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_23`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_24`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_stub`: The routine consists of a single return instruction followed by an unconditional branch that is never reached, so it performs no work and returns control to its caller at once.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_system_trap`: Triggers a software interrupt using trap number six.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_software_interrupt`: Triggers a software interrupt using exception one.
