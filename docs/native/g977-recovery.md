# Game 977 (Gainforce v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00977). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2045424 top-level bytes in 81 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 80 of 80 units identical (bytes and relocations), 60 raw-word lines |
| Functions named and described | FAIL, 171 of 271 named (0 author symbols), 171 of 271 described (6 by rule, 112 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2325 of 2362 (98.43%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4876 of 4879 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 101240 |
| code | 16748 |
| data | 1903858 |
| padding | 26 |
| unused | 23552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 243 | 21 |
| Disk.1:bootblock | 1 | 7 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 691 | 43 |
| logged disk load from Disk.1 | 1 | 553 | 39 |
| logged disk load from Disk.1 | 1 | 107 | 3 |
| logged disk load from Disk.1 | 1 | 107 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1179 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 405 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 99 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 309 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 440 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| Disk.1 bytes 140800-141824 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: custom-format (non-DOS) disk, 1708 non-empty blocks: matched against track-loaded segments
- Gainforce_v1.0/Gainforce_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree Gainforce_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 4 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 805 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 400 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 96 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 23 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 309 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 4 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 16 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: bytes 140800-141824 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/game.slave +16A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/game.slave +16A
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +C, runtime segment an address +16, runtime segment an address +6, runtime segment an address +6
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +C, runtime segment an address +16, runtime segment an address +6, runtime segment an address +6
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +C, runtime segment an address +16, runtime segment an address +6, runtime segment an address +6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +C, runtime segment an address +16, runtime segment an address +6, runtime segment an address +6
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +1A, runtime segment an address (play-long-joy) +1A
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +6, runtime segment an address (play-long-joy) +0, runtime segment an address (play-long-joy) +E, runtime segment an address (play-long-joy) +18
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (59 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (54 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying code at an address (49 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +2A, runtime segment an address +2A, runtime segment an address +1A, runtime segment an address +1A
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: 5 logged loads cover 92920 bytes of the image
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: 1 track-loaded blocks found in recovered runtime segments and classified through them
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: blocks 278-282 (bytes 142336-144896) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.49)
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: blocks 638-645 (bytes 326656-330752) outside the filesystem hold data, not code (opcode density 0.046, entropy 5.20)
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: blocks 1287-1294 (bytes 658944-663040) outside the filesystem hold data, not code (opcode density 0.045, entropy 5.59)
- Gainforce_v1.0/Gainforce_v1.0_hdf/Gainforce/Disk.1: blocks 1749-1749 (bytes 895488-896000) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.80)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 7 packed track region(s), 772608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_jump_vector_stub`: Calls a function through the base pointer, then writes a jump instruction word and the address of an unnamed routine into a fixed memory slot, and calls ten unnamed routines to set up the environment.
- `patch_jsr_then_dispatch_indirect`: Writes JSR-style instruction words and routine addresses into four memory locations, calls an unnamed routine, then makes an indirect call through a jump table that could not be recovered.
- `copy_code_block_into_place`: Stores a JSR-style instruction word and a routine address in the global area, calls an unnamed routine, then copies bytes from the global area up to a fixed end address, advancing both pointers, and calls a second routine.
- `write_nop_instruction`: Writes a no-operation instruction word to a global variable and modifies a value at a pointer offset.
- `read_shifted_longword_with_flag`: Writes a no-operation instruction word to a global, calls an unnamed routine, and returns the longword four bytes before the pointer shifted right by one with its top bit set.
- `advance_pointer_and_dispatch`: Adds 512 to a shared word, writes JSR-style instruction words and routine addresses into memory, calls an unnamed routine, then makes an indirect call through a jump table that could not be recovered.
- `copy_block_to_fixed_limit`: Stores a jump-style instruction word and a routine address in the global area, calls an unnamed routine, then copies a byte block up to a fixed limit with both pointers advancing, and calls a second routine.
- `patch_calls_then_jump`: Writes JSR-style instruction words that call unnamed routines into four memory locations, calls one unnamed routine, and then calls the code at a fixed absolute address.
- `patch_handler_code_sequences`: Tests a shared word against a sentinel value and, when it matches, writes sequences of JSR, JMP and no-operation instruction words with routine addresses into memory, adding one more patch when a flag is set.
- `call_indirect_from_table`: Loads a code pointer from a fixed offset within a shared table and calls it, then returns.
- `spin_forever_without_exit`: Contains only an infinite loop with no exit; the decompiler removed unreachable blocks, and the keyboard serial register is listed among the hardware this routine touches.
- `halt_in_endless_loop`: Contains only an infinite loop with no exit; the decompiler removed unreachable blocks, and the vertical beam position register is listed among the hardware this routine touches.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `start_blit_and_wait`: Writes a fixed blitter size value at offset 88 of the base record, then loops while bit six of the shared variable is set, returning when it clears.
- `dispatch_on_scaled_length`: Multiplies the low word of the first argument by 5632 and compares the product with a fixed constant.
- `run_handler_when_check_zero`: Calls an unnamed routine and then a function-table entry; when that entry returns zero it calls a second table entry, an unnamed routine and a further table entry.
- `call_table_entry_after_call`: Calls one unnamed routine, then calls the code pointer at a fixed offset in the record, and returns.
- `forward_call_then_return`: Calls one unnamed routine and returns, passing control through unchanged.
- `stub_empty_return_text_ref`: Body is a bare return with no work.
- `wait_flag_then_set_one`: When a configuration word is non-zero, waits while bit seven of the shared variable is set, then sets the variable to one once bit six is clear.

### Disk.1:bootblock

- `issue_device_io_request`: Fills in a device request block with a length, a start address and a data pointer, calls the system I/O routine, and returns its result.

### logged disk load from Disk.1 boot-block load

- `sub_002E`: undescribed.
- `sub_003A`: undescribed.
- `sub_0056`: undescribed.
- `sub_0070`: undescribed.
- `sub_02CE`: undescribed.
- `sub_0DB8`: undescribed.
- `sub_0E1A`: undescribed.
- `sub_0E1E`: undescribed.
- `sub_0E64`: undescribed.
- `sub_0EA6`: undescribed.
- `sub_0EBA`: undescribed.
- `sub_0ED8`: undescribed.
- `sub_0F12`: undescribed.
- `sub_0F40`: undescribed.
- `sub_0F84`: undescribed.
- `sub_0F92`: undescribed.
- `sub_0FA8`: undescribed.
- `sub_1008`: undescribed.
- `sub_1044`: undescribed.
- `sub_10A0`: undescribed.
- `sub_110C`: undescribed.
- `sub_1132`: undescribed.
- `sub_113E`: undescribed.
- `sub_118A`: undescribed.
- `sub_11DC`: undescribed.
- `sub_1256`: undescribed.
- `sub_128E`: undescribed.
- `sub_135A`: undescribed.
- `sub_1366`: undescribed.
- `sub_1376`: undescribed.
- `sub_1394`: undescribed.
- `sub_13AA`: undescribed.
- `sub_13C2`: undescribed.
- `sub_13D2`: undescribed.
- `sub_145A`: undescribed.
- `sub_1616`: undescribed.
- `sub_1644`: undescribed.
- `sub_168A`: undescribed.
- `sub_1690`: undescribed.
- `sub_169E`: undescribed.
- `sub_16B6`: undescribed.
- `sub_16C8`: undescribed.
- `sub_16EE`: undescribed.

### logged disk load from Disk.1

- `sub_002E`: undescribed.
- `sub_0DB8`: undescribed.
- `sub_0E1A`: undescribed.
- `sub_0E1E`: undescribed.
- `sub_0E64`: undescribed.
- `sub_0EA6`: undescribed.
- `sub_0EBA`: undescribed.
- `sub_0ED8`: undescribed.
- `sub_0F12`: undescribed.
- `sub_0F40`: undescribed.
- `sub_0F84`: undescribed.
- `sub_0F92`: undescribed.
- `sub_0FA8`: undescribed.
- `sub_1008`: undescribed.
- `sub_1044`: undescribed.
- `sub_10A0`: undescribed.
- `sub_110C`: undescribed.
- `sub_1132`: undescribed.
- `sub_113E`: undescribed.
- `sub_118A`: undescribed.
- `sub_11DC`: undescribed.
- `sub_1256`: undescribed.
- `sub_128E`: undescribed.
- `sub_135A`: undescribed.
- `sub_1366`: undescribed.
- `sub_1376`: undescribed.
- `sub_1394`: undescribed.
- `sub_13AA`: undescribed.
- `sub_13C2`: undescribed.
- `sub_13D2`: undescribed.
- `sub_145A`: undescribed.
- `sub_1616`: undescribed.
- `sub_1644`: undescribed.
- `sub_168A`: undescribed.
- `sub_1690`: undescribed.
- `sub_169E`: undescribed.
- `sub_16B6`: undescribed.
- `sub_16C8`: undescribed.
- `sub_16EE`: undescribed.

### logged disk load from Disk.1

- `copy_relocated_block_to_limit`: Copies bytes from the loaded block into a destination area, advancing both pointers up to a fixed limit, then calls four unnamed routines.
- `store_byte_and_return_shifted`: Reads the longword four bytes before a pointer, stores a word built from one byte of it and a byte of the second argument through another pointer, and returns the value shifted right by one with its top bit set.
- `shift_bit_stream_bits`: Shifts a bit register right by one for a count given by the argument, reloading it from an unnamed routine whenever its bits run out, and returns when the count is used up.

### logged disk load from Disk.1

- `copy_large_block_to_limit`: Copies a large block of bytes from the loaded data into a destination area, advancing both pointers until a fixed limit is reached, then calls unnamed routines.
- `shift_and_combine_bits`: Shifts a value right by one and combines it with a byte from a pointer to form a new word.
- `shift_bit_stream`: Repeatedly shifts a value right by one and reloads it from another routine when it reaches zero for a specified count.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_right_set_top_bit`: Shifts the data register right by one, sets the top bit of the result, and returns it.
- `initialise_display_and_audio`: Saves the shared variable, clears about fifteen thousand longwords at a fixed address, writes the copper location and the DMA and interrupt enable values, fills table entries with addresses, and calls three unnamed routines.
- `acknowledge_interrupt_and_reset`: Calls an unnamed routine, then when the shared variable is non-zero calls four more routines.
- `scan_name_token_and_store`: Scans a text buffer for the end of a name token like its neighbour does for names, but stores the resulting position and a length through an unnamed routine and returns nothing.
- `add_table_record_without_return`: Finds the first empty fourteen-byte entry in a table, stores a pointer and two adjusted coordinates in it, then sets a count of four and clears two fields.
- `advance_table_records_each_frame`: Walks nineteen fourteen-byte table entries, adds per-step offsets from lookup tables to each entry's two coordinates, steps its index, and decrements a counter that is reset to four when it runs out.
- `call_two_routines_in_order`: Calls two unnamed routines in sequence and returns.
- `call_routine_between_guarded_calls`: Calls one unnamed routine unconditionally, and calls another before and after it only when the argument is non-zero.
- `clear_word_block`: Writes zero into the given number of consecutive words starting at the pointer and returns.
- `build_division_offset_table`: Fills a table of words, one per count step, with values taken from dividing a fixed constant by the count argument, accumulating the quotient each time and scaling the high byte by 132.
- `call_routine_return_if_nonzero`: Calls one unnamed routine when the argument is non-zero, calls another unconditionally, then calls a third and returns its result only when the argument is non-zero, otherwise returning zero.
- `repeat_call_until_zero`: Calls an unnamed routine repeatedly until the value it returns is zero, then returns.
- `execute_until_zero`: Repeatedly calls another routine in a loop until that routine returns zero.
- `clear_bit_column_across_rows`: For each of twelve table entries, clears one bit selected by a column value in three bitplane rows spaced forty bytes apart when the entry is zero; other entries take a separate branch that writes the same rows.
- `clear_bit_down_column_rows`: Clears one bit, chosen by a column value, in each of thirty-six bytes spaced forty bytes apart, starting at a pointer computed from the column.
- `return_after_dead_guard`: Its guard requires the shared variable to be both non-zero and zero, which cannot hold, so the inner call never runs and the routine returns without doing anything.
- `queue_blit_rectangle_at_position`: Computes a byte address from x and y coordinates using a 120-byte row stride and a fixed base, waits until the blitter is idle, then programs the blitter control, mask and size registers for a rectangle.
- `queue_blit_clear_rectangle`: Waits until the blitter is idle, then sets a blitter control value and fill mode for a rectangle, computing the destination address from coordinates with a 120-byte row stride and storing the modulo and size values.
- `clear_key_code_flag`: Calls an unnamed routine, then clears the shared word to minus one when it was non-negative; when the previous value was ninety-five it also sets a counter to 1452.
- `read_keyboard_serial_byte`: When bit three of the shared variable is set, marks bit six, calls an unnamed routine for the keyboard serial exchange, clears bit six and returns its value; otherwise it returns 127.
- `track_key_repeat_delay`: Calls an unnamed routine to obtain a key code.
- `step_colour_fade_with_counter`: Counts down a shared step counter and, when it wraps, runs a loop that moves the 4-bit red, green and blue fields of a colour entry by table deltas toward a target, counting how many components changed.
- `adjust_colour_components_toward_target`: When the shared flag is non-zero, moves the 4-bit red, green and blue fields of a colour entry toward a target by table deltas and counts the components that changed.
- `set_audio_channel_volumes`: Walks a sound table in a loop, selecting entries against a running threshold, and writes the four audio channel volume registers from the selected values.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_channel_playback`: Decrements a set of shared counters, then loads start addresses, lengths and volumes for the four audio channels from stored sample tables, and calls three unnamed routines.
- `load_sample_channel_registers`: Copies a longword from a sample table into a record and takes a nibble from it; when the nibble is non-zero it looks up a thirty-byte channel entry and loads the audio channel start address, length and volume values, using a masking subroutine that reduces a word to twelve bits.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_sample_channel_state`: Masks a record word to twelve bits; when the low bits are zero it copies a value and returns, otherwise it picks an entry from a counter modulo three and updates the channel record fields, using the masking subroutine for the comparison.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_structure_tables`: Clears several audio volume registers, initializes a large memory block with zeros, and populates a structure with various values and pointers through a loop.
- `acknowledge_interrupt_request`: Calls an unnamed routine, then writes fixed values of 1 and 32 to globals.
- `execute_indirect_jump`: Loads a word from a table using an offset in d0 and a base address in a0, then jumps to the address stored at that location in a1.
- `sub_023E`: undescribed.
- `call_and_advance_table_pointer`: Calls a routine through a stored function pointer, then advances a stored pointer by four bytes to the next table entry.
- `set_global_byte`: Stores the value one in a global variable and returns immediately.
- `compare_and_accumulate_nibbles`: If a global variable is non-zero, it loops through a table of words, comparing specific nibbles of a value at a pointer against a target value and adding the table entries to that value if they do not match.
- `start_blit_after_blitter_idle`: Sets a flag and calls another routine, then either returns a stored byte from an indexed entry and resets its state or advances the index.
- `initialize_structure_fields`: Loops until bit 6 of a byte at offset 2 of a structure in a6 is set, then writes several longwords and words to various offsets of that structure.
- `update_structure_values`: Checks bit 6 of a byte at offset 2 of a structure in a6; if not set, it writes four words to offsets 100, 98, 96, and 102 of the structure, loads three values from a table into d2, d3, and d4, and calls another routine if d4 meets certain criteria.
- `calculate_and_store_table_data`: Loops until bit 6 of a byte at offset 2 of a structure in a6 is set, then calculates several values based on registers d0 through d4 and stores them into the structure at various offsets.
- `swap_and_call_routines`: Stores the word from d0 into d5, moves the word from d1 into d0, calls another routine, moves the word from d5 into d0, calls another routine, and returns.
- `call_routine_with_optional_repeats`: Makes one call to an unnamed routine, plus an extra call before and after it when the incoming data register is nonzero.
- `clear_memory_array`: Decrements register d0 by 1 and then loops to clear words at the address pointed to by a5, incrementing the pointer after each write until d0 reaches -1.
- `fill_scaled_ramp_table`: Fills a word array with a linear ramp where each entry is the high byte of an accumulator multiplied by 30, and the accumulator grows by 12288 divided by the count at each step.
- `call_pair_return_optional_result`: Calls an unnamed routine, optionally once more before it, then returns zero unless the incoming count is nonzero.
- `loop_clear_column_bits`: Decrements a counter in d0, sets a pointer in a4 from a1, calls another routine, increments d6, and loops until d0 reaches zero.
- `execute_loop_until_zero`: This routine repeatedly calls another routine and continues execution as long as the returned value is non-zero.
- `update_bitplane_data`: Calculates bit offsets and indices to set or clear bits in a structure at a6 and a3 based on a bitmask retrieved from a global variable.
- `clear_bit_down_plane_column`: Clears one bit in 240 bytes spaced six bytes apart, stepping down a column of a bitplane.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_bitstream_to_memory`: Reads a bitstream from a source buffer and expands it into a destination memory area by shifting and masking bits to reconstruct bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_bitstream_to_memory_2`: Copies a block of memory to a new location and then performs bitstream decompression by reading bits from a source and writing expanded byte sequences into a destination buffer.
- `extract_and_store_byte`: Fetches a longword from a memory location pointed to by a register, extracts the lowest byte, and stores it as a word into a destination pointed to by another register.
- `process_bit_stream`: Decrements a value in d0 and, if it is not zero, calls another routine and loops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_interrupt_and_clear_buffer`: Sets bit 15 and bit 14 of a global variable, writes two values to a hardware register at offset 154 of a structure in a6, and clears a buffer of 7678 bytes starting at a global address through a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_hardware_and_clear_memory`: Sets bit 14 of a global variable, writes two values to a hardware register at offset 154 of a structure in a6, copies two longwords from global variables to a global address, clears 8192 bytes starting at a global address through a loop, and writes a value to a hardware register at offset 150 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_buffer_and_configure_dma`: Sets bit 15 and bit 14 of a global variable, writes two values to a hardware register at offset 154 of a structure in a6, clears a buffer of 7678 bytes starting at a global address through a loop, and writes two values to a hardware register at offset 150 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_configure_dma`: Writes two values to a hardware register at offset 154 of a structure in a6, copies two longwords from global variables to a global address, writes a value to a hardware register at offset 154 of a structure in a6, clears 8192 bytes starting at a global address through a loop, and writes a value to a hardware register at offset 150 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_init_dma`: Clears a block of memory starting at a specific address using a loop, then configures DMA controller registers and custom chip registers via the hardware base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_joystick_buffer`: Copies a longword from a global variable to a destination, sets a hardware register, and enters a loop that clears a memory block at a fixed address using a decrementing counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_init_cop`: Clears a block of memory starting at a specific address using a loop, then initializes several copper registers and custom hardware registers to set up display or DMA parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_hardware_registers`: Clears a block of memory starting at a specific address for eight thousand bytes, writes several values to a structure pointed to by a6, and waits for a specific bit to be cleared in a status register before writing further values to the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_configure_hardware`: Clears a longword at the address in a0 through a loop of two iterations, then configures several hardware registers via a pointer in a6, including setting a value at offset 150, offset 154, offset 128, and offset 136, and finally writes a longword to offset 80 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_setup_hardware`: Clears a longword at the address in a0 through a loop of two iterations, then sets offset 150 of the structure in a6 to 32768, offset 128 to a longword, offset 136 to 0, and offset 150 to 32768 again, before writing a longword to offset 80 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers`: Sets offset 150 of the structure in a6 to 34320, offset 154 to 128, offset 128 to a longword, offset 136 to 0, and offset 150 to 32768, then writes three longwords to offsets 80, 84, and 64 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_configure_hardware_extended`: Clears a longword at the address in a0 through a loop of two iterations, then sets offset 150 of the structure in a6 to 128, offset 128 to a longword, offset 136 to 0, and offset 150 to 32768, before writing three longwords to offsets 80, 84, and 64 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_minimal`: Sets offset 150 of the structure in a6 to 128, offset 128 to a longword, offset 136 to 0, and offset 150 to 32768, then writes three longwords to offsets 80, 84, and 64 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state_and_load_pointers`: Writes two 16-bit values and a 32-bit value to a structure in a6, clears a 16-bit word at offset 136, and then conditionally loads three 32-bit pointers into offsets 80, 84, and 64 of the same structure based on the state of bit 14 in a byte at offset 2 of a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_base`: Sets offset 128 of the structure in a6 to a longword, offset 136 to 0, and offset 150 to 32768, then writes three longwords to offsets 80, 84, and 64 of the structure in a6 if bit 14 of offset 2 is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_joystick_input_parameters`: Writes three longwords and a word to a structure via a6, clears a 16-bit value at offset 136, and sets a 16-bit value at offset 150.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_state`: Clears offset 136 of the structure in a6, sets offset 150 to 32768, and if bit 14 of offset 2 is not set, writes longwords to offsets 80, 84, 64, and 68 of the structure in a6, and a word to offset 100.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_joystick_state_and_load_params`: Clears a 16-bit value at offset 136 of a structure, writes a constant to offset 150, and checks bit 14 of a byte at offset 2 of the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_and_set_values`: Writes a 16-bit value to offset 150 of the structure in a6, checks bit 14 of offset 2 of the structure in a6, and if bit 14 is not set, writes three 32-bit values to offsets 80, 84, and 64, and a 16-bit value to offset 100.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_and_set_values_alt`: Writes a 16-bit value to offset 150 of the structure in a6, checks bit 14 of offset 2 of the structure in a6, and if bit 14 is not set, writes three 32-bit values to offsets 80, 84, and 64, and a 16-bit value to offset 100 using different 32-bit constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_on_bit_check`: Checks bit 14 of offset 2 of the structure in a6, and if not set, writes three 32-bit values to offsets 80, 84, and 64, and a 16-bit value to offset 100.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_bit_and_call_subroutine`: Checks bit 14 of offset 2 of the structure in a6, and if not set, writes three 32-bit values to offsets 80, 84, and 64, and two 16-bit values to offsets 100 and 102, then calls a routine at a long address and calls another routine via a word address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_with_register_load`: Otherwise it returns; otherwise it writes three 32-bit values to offsets 80, 84, and 64, and two 16-bit values to offsets 100 and 102, then writes a 16-bit value to offset 88 and loads a 8-bit value into d7.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_and_call_subroutine`: Writes three 32-bit values to offsets 80, 84, and 64, and two 16-bit values to offsets 100 and 102, then calls a routine at a long address and calls another routine via a word address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_with_offset_load`: Writes three 32-bit values to offsets 80, 84, and 64, and two 16-bit values to offsets 100 and 102, then writes a 16-bit value to offset 88 and loads an 8-bit value into d7.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_and_call_with_store`: Writes three 32-bit values to offsets 80, 84, and 64, and two 16-bit values to offsets 100 and 102, then calls a routine at a long address, calls another routine via a word address, and writes a 32-bit value to offset 108.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_decrement_d7`: Writes three 32-bit values to offsets 64, 68, and 100, and two 16-bit values to offsets 100 and 102, then writes a 16-bit value to offset 88 and loads an 8-bit value into d7, writes a byte to a global variable, and enters a loop that decrements d7 and branches back if d7 is not zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_values_and_call_with_store_alt`: Writes three 32-bit values to offsets 64, 68, and 100, and two 16-bit values to offsets 100 and 102, then calls a routine at a long address, calls another routine via a word address, and writes a 32-bit value to offset 108.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_loop_data`: Sets a 32-bit value at offset 68 of the structure in a6, sets 16-bit values at offsets 100 and 102, stores a 16-bit value at offset 88, sets d7 to 35, stores a byte in a global variable, and enters a loop that decrements d7 while checking if a byte at offset 6 of the structure in a6 equals 200.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_call_handlers`: Sets a 32-bit value at offset 68 of the structure in a6, sets 16-bit values at offsets 100 and 102, calls a routine at a fixed address, calls a routine at 1534, stores two 32-bit values in a global variable at offsets 108 and 100, and calls a routine at 7856.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_load_pointers`: Sets 16-bit values at offsets 100, 102, and 88 of the structure in a6, sets d7 to 35, stores a byte in a global variable, enters a loop that decrements d7 while checking if a byte at offset 6 of the structure in a6 equals 200, and then loads two addresses into a0 and a1 and sets d7 to 32.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_call_handlers_short`: Sets 16-bit values at offsets 100 and 102 of the structure in a6, calls a routine at a fixed address, calls a routine at 1534, stores two 32-bit values in a global variable at offsets 108 and 100, and calls a routine at 7856.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_load_pointers_partial`: Sets 16-bit values at offsets 102 and 88 of the structure in a6, sets d7 to 35, stores a byte in a global variable, enters a loop that decrements d7 while checking if a byte at offset 6 of the structure in a6 equals 200, and then loads two addresses into a0 and a1 and sets d7 to 32.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_call_handlers_minimal`: Sets a 16-bit value at offset 102 of the structure in a6, calls a routine at a fixed address, calls a routine at 1534, stores two 32-bit values in a global variable at offsets 108 and 100, and calls a routine at 7856.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_status_and_loop`: Polls a byte at offset 6 of a structure in a6 until it matches a specific value, then executes a loop that calls another routine and updates a global variable until that variable is zeroed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_handlers_and_clear_flag`: Calls a routine at a fixed address, calls a routine at 1518, stores two 32-bit values in a global variable at offsets 108 and 100, calls a routine at 7856, stores a byte in a global variable, and clears a 16-bit value in a global variable if it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_store_word`: Repeats a loop thirty-five times checking a byte at offset six of a pointer in a6, calls another routine, and then stores a 32-bit value from a function return into a structure pointed to by a0 at offsets 1954, 1958, 1962, and 1966.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_handlers_and_clear_flag_minimal`: Stores two 32-bit values in a global variable at offsets 108 and 100, calls a routine at 7856, stores a byte in a global variable, and clears a 16-bit value in a global variable if it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_loop`: Performs a loop that waits for a byte at an offset of 6 from a pointer in a6 to equal -56, calls another routine, and performs several loops involving bitwise operations and calls to other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequential_logic`: Calls a routine, sets a global variable to 1, and enters a loop that decrements a counter while clearing a bit in a memory location pointed to by a register and calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_modify_byte`: Enables interrupts via the INTENA register, then enters a loop that checks a global variable and clears bit 3 of a byte at a specific offset in a structure in a0 until the global variable is zero, finally setting the global variable to a value from a table.
- `call_indirect_routine`: Sets registers a1, d0, d1, and a0 to specific values and calls a routine using a pointer from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_mask`: Sets bits 5, 6, and 7 of a 16-bit word at an offset from a pointer in a4 using a bitwise OR operation.
- `set_global_values`: Calls two routines and then sets a global variable to 1 and a memory location to 4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sub_0000`: undescribed.
- `set_global_values_minimal`: Calls two routines, sets a global variable to 1 and a memory location to 4, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_routine_alt`: Sets registers a1, d0, d1, and a0 to specific values and calls a routine using a different pointer from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_interrupt`: Restores registers from the stack, sets a memory location to 4, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_routine_short`: Sets registers d0, d1, a0, and a2 to specific values and calls a routine using a pointer from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_routine_minimal`: Sets registers a0 and a2 to specific values and calls a routine using a pointer from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_state`: Calls three routines, executes a long jump, sets a value of 32772 to a hardware register, and then sets that register to 32 after returning from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_routine_2`: Loads an address from a location relative to the program counter into a register and calls the routine at that address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_audio_and_interrupt`: Calls two routines, executes a long jump, sets a hardware register to 32772, and then sets the register to 32 after returning from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_conditional_value`: Checks if a global variable is non-zero and if specific bits in a 16-bit value at an offset of 2 from a pointer match bits in another value, then adds a value from a pointer to that 16-bit value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_audio_update`: Calls a routine, executes a long jump, sets a hardware register to 32772, and then sets the register to 32 after returning from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nibble_differences`: Loops through a table, compares nibbles of values at offsets 2 and 6 between two pointers, adds values from a source table to a destination table if nibbles differ, and clears a byte in a global variable if no differences are found.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_state`: Enters a loop that calls a routine, checks a global variable, and either negates a byte in a global variable or sets a byte and a long value in global variables based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_nibble_offsets`: Loops through data, adding values from a source to a destination at specific offsets if the lower nibbles do not match, and updates a global variable if a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_logic`: Loads a pointer, calls a routine, and if a global variable is zero, negates it and potentially sets a byte and a long value in global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_nibble_adjustments`: Loops through a structure, adding a value to a 16-bit field at offset 2 if the lower nibbles of two values do not match, and clears a global variable if a counter remains zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_state_2`: Calls another routine, then checks a global byte; if it is zero, it negates it and either sets it to one and stores a pointer at offset 2 of a global structure, or sets it to one and stores a different pointer at offset 2 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values`: Loops through a table, adding a value from a register to a 16-bit integer at an offset of 2 from a1 if a comparison with a value in d1 passes, then sets a global variable to zero if a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_global_state_3`: Checks a byte at a global address; if it is zero, it negates a byte at another global address and sets a new global address and a status byte based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_add_to_table`: Compares d1 and d2, and if they are not equal, adds d5 to a 16-bit value at offset 2 of a1 while looping and incrementing registers and pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `negate_and_set_address`: Negates a byte at a global address and, depending on the sign of the result, sets a status byte and a new global address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_pointers_in_loop`: Increments d0, a0, and a1, then loops based on d7, clearing a byte at a global address if d0 reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `store_pointer_and_byte`: Stores the value one in a global variable and stores a pointer in another global variable before returning.
- `process_table_data`: Decrements a global counter and, when it reaches zero, loads values from a table and performs recursive calls.
- `accumulate_nibble_mismatches`: If a global variable is non-zero, it loops through a table of words, comparing specific nibbles of a value at a pointer against a target value and adding the table entries to that value if they do not match; if no changes are made during the loop, the global variable is cleared.
- `perform_infinite_recursion`: This routine calls itself twice before returning, resulting in an infinite loop of recursive calls.
- `call_routine_with_optional_repeats_2`: Calls another routine, then calls it again after subtracting the input value twice from 48, and finally calls it a third time if the original input value was nonzero.
- `clear_memory_array_2`: Decrements a value in register d0 by 1 and then loops to clear words at the address pointed to by a5, incrementing the pointer after each write until d0 reaches -1.
- `fill_table_with_scaled_values`: Populates a table with values derived from a counter that is incremented by a fraction of a constant.
- `call_pair_return_optional_result_2`: Calls another routine, and if the input value in d0 is non-zero, calls that same routine a second time before returning the result of the second call; otherwise, it returns zero.
- `loop_clear_bits_in_memory`: Decrements a counter in d0, sets a pointer in a4 from a1, calls clear_bits_in_memory, increments d6, and loops until d0 reaches zero.
- `loop_until_non_zero`: Repeatedly calls another routine in a loop until the return value is zero, using a calculated offset to increment a pointer in d2.
- `update_bit_array_from_source`: Iterates through a table of 48 entries, reading a value from a global variable to determine whether to set or clear specific bits at offsets 0, 6, and 12 of a destination structure calculated from a pointer in a6.
- `clear_bits_in_memory`: Clears specific bits within a memory block at regular intervals using a bitmask calculation.
- `initialize_structure_and_call_function`: Stores three long values into a structure at offsets 68, 100, and 64 relative to a6, then calls another routine.
- `sub_04BE`: undescribed.
- `calculate_and_set_bits`: Calculates values using a table and global variables, then sets specific bits in a memory buffer based on the results.
- `sub_05D8`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_channels_and_samples`: Decrements four consecutive bytes in a global array and, if they remain non-negative, updates the hardware audio channel pointers and volumes by calling another routine with specific bitmask parameters.
- `update_audio_sample_parameters`: Reads a structure from a pointer, calculates an index into a sample table using a masked value, and updates the structure with new sample data, volume, and pitch values.
- `mask_and_compare_word_2`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_joystick_input_and_audio_state`: Reads a 16-bit value from a structure and uses its lower 4 bits to branch into different logic paths that update a 16-bit value at offset 16, a 16-bit value at offset 24, or a 16-bit value at offset 6 of a second structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `stub_bare_return_entry`: Body is a bare return with no work, so the routine does nothing when called.
- `sub_000A`: undescribed.
- `sub_000E`: undescribed.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `sub_0016`: undescribed.
- `sub_001A`: undescribed.
- `sub_001E`: undescribed.
- `sub_0022`: undescribed.
- `sub_0026`: undescribed.
- `sub_002A`: undescribed.
- `sub_002E`: undescribed.
- `sub_0032`: undescribed.
- `sub_0036`: undescribed.
- `sub_003A`: undescribed.
- `sub_003E`: undescribed.
- `sub_0042`: undescribed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_two_loops`: Executes a loop that decrements d0 from 300 to 0, performs four bitwise OR operations on d0, and then executes a second loop that decrements d0 from 300 to 0.
