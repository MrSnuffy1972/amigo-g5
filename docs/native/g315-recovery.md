# Game 315 (UniversalWarrior v1.01 0597): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00315). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2319026 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 38 of 38 units identical (bytes and relocations), 691 raw-word lines |
| Functions named and described | pass, 377 of 377 named (0 author symbols), 377 of 377 described (10 by rule, 286 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6828 of 6829 (99.99%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16337 of 16356 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 57832 |
| code | 65382 |
| data | 1995607 |
| padding | 13 |
| unused | 200192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 162 | 12 |
| logged disk load from disk.1 | 1 | 85 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1089 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 411 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 71 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 148 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 85 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 85 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11586 | 201 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 555 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 103 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 849 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 377 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 87 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |
| disk.1:chunks 144-757 (bytes 36864-194048)+232#unpacked | 1 | 321 | 12 |

## Cases handled

- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: raw disk image (958464 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.2: raw disk image (36864 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597.hdf: hard-disk image (AmigaDOS OFS, 2400 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree UniversalWarrior_v1.01_0597_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 1 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-rs-m1500-unpk04): 643 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk04): 112 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk04): 71 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 99 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk04): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3796 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 543 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 85 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 803 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 230 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: 1 logged loads cover 36864 bytes of the image
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1:chunks 144-757 (bytes 36864-194048)+232#unpacked: hunk executable inside unpacked track data, recovered as unit u37 after the main pass
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 144-757 (bytes 36864-194048) hold 1 packed streams (BK: ByteKiller) found by scanning, 99% of the bytes
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 768-1855 (bytes 196608-475136) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.12)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 1872-1903 (bytes 479232-487424) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.71)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 1920-1951 (bytes 491520-499712) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 1968-1999 (bytes 503808-512000) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.63)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2016-2047 (bytes 516096-524288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.69)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2064-2095 (bytes 528384-536576) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2112-2143 (bytes 540672-548864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.51)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2160-2191 (bytes 552960-561152) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.21)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2208-2239 (bytes 565248-573440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2256-2288 (bytes 577536-585984) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.46)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2304-2335 (bytes 589824-598016) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2352-2383 (bytes 602112-610304) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.17)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2400-2431 (bytes 614400-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.66)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2448-2479 (bytes 626688-634880) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.54)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2496-2528 (bytes 638976-647424) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.43)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2544-2576 (bytes 651264-659712) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2592-2624 (bytes 663552-672000) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.36)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2640-2671 (bytes 675840-684032) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2688-2719 (bytes 688128-696320) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.28)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2736-2767 (bytes 700416-708608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.84)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2784-2790 (bytes 712704-714496) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2792-2813 (bytes 714752-720384) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.70)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2815-2815 (bytes 720640-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.07)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2832-2863 (bytes 724992-733184) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2880-2912 (bytes 737280-745728) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2928-2961 (bytes 749568-758272) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.67)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 2976-3007 (bytes 761856-770048) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.57)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3024-3055 (bytes 774144-782336) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3072-3103 (bytes 786432-794624) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.71)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3120-3152 (bytes 798720-807168) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.20)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3168-3199 (bytes 811008-819200) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.26)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3216-3247 (bytes 823296-831488) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3264-3295 (bytes 835584-843776) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.30)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3312-3344 (bytes 847872-856320) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3360-3392 (bytes 860160-868608) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.58)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3408-3439 (bytes 872448-880640) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.33)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3456-3488 (bytes 884736-893184) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.21)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3504-3536 (bytes 897024-905472) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.44)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3552-3583 (bytes 909312-917504) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.25)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3600-3632 (bytes 921600-930048) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.29)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3648-3680 (bytes 933888-942336) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.39)
- UniversalWarrior_v1.01_0597/UniversalWarrior_v1.01_0597_hdf/UniversalWarrior/disk.1: chunks 3696-3727 (bytes 946176-954368) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.22)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decode_block_then_initialise_system`: Decodes a bit-packed block backwards from the end of its input and returns minus one if the output stops short of the expected length.
- `initialise_system_and_run_loop`: Installs an unnamed routine pointer, disables all interrupts, clears a block of 65,536 longwords, a block of 2,048 longwords and a small global area.
- `decode_and_initialize_system`: Decodes a bitstream into ASCII characters and stores them into a global buffer.
- `store_message_then_initialise_system`: Stores a readable text message, built from ASCII letter pairs, into globals and then runs the same decode-and-setup sequence as its sibling routines.
- `unpack_bitstream_to_memory_2`: Decodes a variable-length bitstream into a destination memory buffer.
- `unpack_bitstream_to_memory`: Reads a bitstream from a source and reconstructs bytes by shifting and masking bits.
- `call_first_entry_then_clear`: Calls a function through a pointer at a fixed offset in a saved table, then clears a global and returns the incoming data register unchanged.
- `call_external_routine_and_clear_global`: Calls an external routine through a pointer and then sets a global variable to zero.
- `call_table_entry_by_flag`: Calls one of two functions from a saved table, chosen by whether a global is zero, then clears that global.
- `call_second_entry_then_clear`: Calls a function through a pointer at a second fixed offset in a saved table, then clears a global.
- `wait_flag_then_call_entry`: Loops while a flag byte in a global has its high bit set, then calls a function from a saved table and clears the flag.

### logged disk load from disk.1

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_button_then_advance_stack`: Calls several subroutines, sets state globals to eight and nine, then polls a status routine until bit four is set and adjusts the stack pointer.
- `large_multi_branch_state_dispatcher`: Large routine with many conditional branches and calls that dispatches on game state globals and invokes four unnamed subroutines.
- `load_level_record_fields`: Indexes a record table by the word at a fixed global, multiplies by 156, then copies two bytes and a word into globals and stores the record pointer.
- `copy_8x19_tile_block_to_2a008`: Copies eight rows of nineteen bytes from the global source buffer to memory at 2a008, then updates the global index and calls two routines before returning.
- `multiply_global_by_d1_and_check`: Calls two routines, multiplies the global word by the low word of d1, clamps a result of one to 2000, and if nonzero sets the global to 7 and calls further routines.
- `set_global_8_and_call_two`: Stores the value 8 into the global variable, then calls two unnamed routines and returns to its caller.
- `toggle_flag_on_selected_option_bit`: Clears the word at 50914, reads the option index from 50894 and the bit number from 50896, calls the bit-modify routine on the matching option variable, and inverts 50914 when the tested bit is set.
- `adjust_option_bit_by_direction`: Checks the direction word at 50916; if neutral it sets or clears bit d7 of d1, otherwise it walks the option bit index at 50900 through the option table at a fixed address, toggles the selected bit, and calls the flag and sound routines.
- `dispatch_option_action_by_index`: Calls the common setup routine, then branches on the option index at 50894 to one of six action routines, saving and restoring d0 around each call.
- `init_playfield_and_clear_buffers`: Writes 32 to a6 offset 150, sets several globals, calls five unnamed routines, fills 96 entries of 40 bytes at a fixed address with -1, masks low 24 bits of 26 entries in four buffers, then sets more globals and returns.
- `data_table_ori_word_a3`: A single ori.w immediate instruction whose bytes are data, not executable code; it is not a real routine and does not return.
- `call_five_helpers_then_clear_global`: Calls one unnamed routine, then calls another unnamed routine five times, sets a global to zero, calls a final unnamed routine and returns.
- `init_sequence_five_waits_clear_global`: Calls the setup routine, the sound routine, four wait routines and a delay routine, then sets a global variable to zero and returns.
- `sound_six_waits_clear_and_recurse`: Calls the sound routine, six wait routines, sets a global variable to zero, then calls itself recursively before returning.
- `sound_five_waits_recurse_clear_global`: Calls the sound routine, five wait routines, then calls itself recursively and sets a global variable to zero before returning.
- `play_runtime_main_loop`: Large runtime routine that calls thirteen unnamed subroutines and drives the main game flow, entered from outside static flow during the reference run.
- `copy_selected_level_tiles_to_buffer`: Uses the current level index to locate a tile block and copies eight rows of nineteen bytes each into the buffer at a fixed memory area, then calls three unnamed routines.
- `pick_random_level_and_copy_tiles`: Chooses a level index different from the previous one, copies eight rows of tile data expanding dollar markers via a helper, then resets two global pointers.
- `toggle_flag_and_start_sound_sequence`: Saves d6-d7/a0-a2, increments a0, complements the word at 6406, sets words at 6402 and 6404 to 1 and 7, loads d1 from 50932 and calls a subroutine.
- `fill_eight_buffers_with_spaces`: Walks eight pointer/length pairs from a fixed address, filling each buffer with space bytes, then calls a routine with a1 set to a fixed address and a0 to a fixed address.
- `clear_then_copy_selected_text_page`: Calls the space-fill routine, picks one of three source pages based on the word at 50918, copies it into the eight buffers, stores buffer pointers at 50924 and 50928 and sets 3674 to 1.
- `copy_selected_sprite_frames_to_planes`: Calls a setup routine, picks one of four source tables based on two state words, then copies eight frame records into the destinations listed in a pointer table and stores three new state values.
- `load_level_tiles_and_run_script`: Uses the current level index to fetch a tile block and a script pointer, copies the script header fields into state words, copies 19 bytes per row into tile memory, then calls three subroutines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_screen_buffers_and_dispatch_state`: Copies a word count and 400 longwords between two screen buffers, calls a routine, then branches on the state byte to run the matching update, reset or setup subroutines.
- `dispatch_player_input_bits_to_actions`: Clears a word flag, reads the input bitfield, and for each of five bits either calls a movement routine or, when the high bit is set, plays a sound and calls the routine, then clears a final word.
- `clear_five_long_player_state_block`: Loads the address of a player state block into a0 and writes five zero longs through it, clearing twenty bytes of state, then returns.
- `cancel_action_clear_bit_and_sound_slot`: Reads a word action index, and if nonzero clears it, clears the matching bit in a byte mask, clears a sound word, and zeroes eight longs of a sound structure.
- `clear_byte_flag_and_zero_block_a`: Loads a word flag, and if nonzero clears it, clears the matching bit in a byte mask, zeroes a word, then clears eight longs at the block pointer and returns.
- `clear_long_flag_and_zero_block_b`: Loads a word flag, and if nonzero clears it, clears the matching bit in a long mask, zeroes a word, then clears eight longs at the block pointer and returns.
- `clear_word_flag_and_zero_block_c`: Loads a word flag, and if nonzero clears it, clears the matching bit in a word mask, zeroes a word, then clears eight longs at the block pointer and returns.
- `clear_active_slot_and_bitmap`: Reads the word at the active slot pointer, and if nonzero clears it, clears its bit in the control word, zeroes a status word and clears eight longwords of a bitmap buffer.
- `refresh_slot_sprites_and_dispatch`: Calls three setup routines, copies four bytes from global state into sprite fields at offsets 18, 19, 22 and 23, calls the routine at address 1844 with a byte from offset 2, then calls three more routines.
- `copy_slot_record_to_table`: Computes a destination from the slot index times 32 plus a base, then copies eight longwords from a fixed source buffer into that destination and returns.
- `copy_32_byte_record_to_slot_a`: Reads a word index from memory, scales it by 32, and copies eight longwords from a fixed source table into that slot of the first destination table.
- `copy_32_byte_record_to_slot_b`: Reads a word index from a second counter, scales it by 32, and copies eight longwords from a second source table into that slot of the second destination table.
- `copy_32_byte_record_to_slot_c`: Reads a word index from a third counter, scales it by 32, and copies eight longwords from a third source table into that slot of the third destination table.
- `copy_selected_sprite_to_work_buffer`: Uses the word at a global index to pick a 32-byte record, then copies eight longwords from a fixed source table into that record.
- `swap_two_sprite_buffers_by_mode`: Checks a global mode word; if it equals two it copies 96 longwords from a1 into one buffer and a2 into another, otherwise swaps the destinations.
- `refresh_four_sprite_buffers_and_palette`: Copies four indexed 32-byte records into work buffers, calls two subroutines, patches palette bytes at offsets 18 and 22, then copies a fifth record.
- `copy_graphics_to_chip_ram`: Copies three blocks of graphics data into chip RAM at either the a fixed address or a fixed address base depending on a global flag, writing a word count then byte and longword streams, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fade_palette_colors_up_toward_target`: Waits for the vertical blank bit in the custom registers, then repeatedly increments the red, green and blue nibbles of each palette entry toward a target table, using per-entry offsets.
- `fade_palette_colors_down_to_black`: Waits for vertical blank, then repeatedly decrements the red, green and blue nibbles of each palette entry until all reach zero, clearing the fade flag at the end.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields`: Writes several constant values and a global variable into specific offsets of a structure pointed to by a register.
- `set_structure_values`: Writes a constant, a pointer to a global variable, and the global variable itself into a structure.
- `initialize_structure_data`: Writes a constant, a large integer, and a global variable into specific offsets of a structure.
- `update_data_structure_one`: Writes a short value, a long value, and several word values to offsets of a pointer.
- `update_data_structure_two`: Stores a short value and a long value into a memory block pointed to by a register.
- `update_data_structure_three`: Populates a memory area with a short value, a long value, and two word values derived from a global variable.
- `update_data_structure_four`: Writes a short value, a long value, and two word values to a memory location using a register as a base.
- `update_data_structure_five`: Sets a series of word and long values at specific offsets from a pointer.
- `decrement_and_update_state_flag`: Repeatedly decrements a global variable and updates a value at a memory offset through a pointer based on whether the variable has reached zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `intro_bytekiller_depack`: Decodes a packed stream while updating a hardware color register and performing checksum verification via bit reader calls.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_4cf_bytes_to_7d800`: Copies 1232 bytes from the source at a fixed address to the destination at a fixed address using a0 and a1, counting down d7, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_bitstream_to_1e0_buffer`: Reads a bitstream from a table, decodes runs and literals into the buffer at 480 via a2, flashes COLOR00 while waiting, and jumps through a vector when done.
- `fetch_bit_with_carry_into_d0`: Pops a longword from a0, XORs it into d5, sets the carry flag, rotates d0 right through extend and returns the shifted value.
- `shift_bits_from_stack_words_into_d2`: Shifts d0 right, refilling from words popped backwards through a0 and xoring d5, and rotates the carry into d2 for d1 iterations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_system_trap`: Stores a value into a global variable and then executes a software trap.
- `execute_complex_initialization`: Performs a series of recursive calls and updates various hardware registers and global variables.
- `clear_and_set_structure`: Clears a large memory range and then updates several fields within a structure based on an offset derived from a global variable.
- `clear_memory_block`: Fills a block of memory starting at a fixed address with zeros for a specific count.
- `update_global_state`: Repeatedly writes a calculated value to a global variable and calls another routine until the variable becomes zero.
- `copy_and_process_tables`: Copies data from one memory location to another through several loops and uses values from a list to index into a table.
- `fill_structure_with_patterns`: Iteratively populates a structure with values derived from three incrementing counters through repeated calls to another routine.
- `populate_object_table`: Calculates offsets based on a global variable to search through a data stream and populate a table with attributes if a slot is empty.
- `update_sprite_attribute_table_2`: Iterates through a data structure to find available sprite slots and updates their position and attribute data.
- `clear_multiple_buffers`: Clears a block of memory at an offset from a register and then clears a series of memory locations pointed to by a table.
- `initialize_and_transform_palette`: Clears a memory block and performs a series of bitwise transformations on palette data tables.
- `decrement_loop_counter`: Decrements a register from a large value down to zero using a loop.
- `normalize_and_scale_global_value`: Checks a bit in a structure and performs signed magnitude normalization and bitwise scaling on a global variable.
- `update_pointer_via_table`: Calculates a new address using a value from a global variable and a table, then stores the value found at that new address back into the global variable.
- `store_pointer_to_itself`: Stores the value of a global variable into the memory location pointed to by that same variable.
- `clear_structure_members`: Sets a byte at an offset of one and a word at an offset of two relative to a global pointer to zero.
- `update_audio_parameters_and_channels`: Loads audio data from a table into global variables and configures multiple audio channels by calculating frequency, period, length, and volume parameters.
- `read_audio_channel_state`: The routine has 16 instructions, so the Ghidra pseudo-code showing only a bare return is incomplete and should not be trusted for its behaviour.
- `load_audio_parameters`: Loads data from a table into global variables based on a pointer and an index, then performs conditional checks to configure audio settings.
- `configure_audio_channels`: Calculates a frequency value and updates multiple entries in a table with specific period, length, and volume parameters.
- `increment_global_and_call`: Increments a global variable by two and calls another routine.
- `poll_status_bit_sequence`: Repeatedly calls another routine and checks if a specific bit is set in the return value.
- `update_global_state_flags`: Checks specific bits within a global variable and performs arithmetic operations on that variable, such as doubling it or zeroing it, based on the bitmask state.
- `update_audio_channel_parameters`: Modifies a global variable through a series of recursive conditional checks and bitwise operations before updating the first audio channel hardware registers with new length, period, and volume values.
- `process_bitstream_state_machine`: Manages a state machine by evaluating a global variable to execute specific sequences of data copies, arithmetic adjustments, and recursive calls.
- `clear_global_variable`: If a global variable is non-zero, it is set to zero.
- `copy_data_and_invert_variable`: Copies a block of data from one memory location to another and then updates a global variable with a bitwise inversion.
- `restore_data_and_reset_variable`: Copies a block of data from one location to another, resets a global variable to zero, and then sets it to a specific constant value.
- `update_state_machine_and_copy_tables`: Manages a state machine by incrementing a global variable and executing conditional memory copies from source tables to a destination buffer.
- `update_entity_state_parameters`: Calculates a value based on a global variable and a table lookup, then performs a series of writes to an object structure.
- `initialize_object_properties`: Sets multiple fields in a data structure pointed to by a register, including zeroing specific offsets and assigning values from a function call.
- `iterate_and_update_data_tables`: Iterates through multiple data tables using calculated offsets and loops.
- `populate_data_table`: Calculates a pointer to a table entry based on an input value and, if the entry is empty, populates it with data from the input.
- `clear_data_table_entry`: Calculates a pointer to a table entry based on an input value and clears the contents of that entry.
- `initialize_audio_sample_tables`: Iterates through a data structure to configure multiple audio parameters and populates a series of memory locations with specific offsets and control values.
- `update_state_machine`: Manages a state machine by checking various global flags and updating a counter and a pointer based on the current state.
- `update_table_entries`: Writes several 16-bit and 32-bit values to specific offsets within a table pointed to by a global variable.
- `validate_global_bitmask_state`: Performs a series of conditional checks on a global variable to verify specific bit patterns and sign flags.
- `mask_audio_tables_2`: Calculates a bitmask based on a global variable and applies it to several entries in multiple tables using bitwise AND operations.
- `mask_audio_tables`: Calculates a bitmask based on a global variable and applies it to four entries in three different tables using a loop.
- `calculate_table_offsets`: Performs arithmetic on a global variable to derive an index, then calculates three memory addresses and stores a 32-bit value into two global variables.
- `return_concatenated_variable`: Returns a 32-bit value formed by concatenating a global variable with itself.
- `return_duplicated_variable`: Returns a 32-bit value composed of two copies of a global variable.
- `store_input_to_global`: Stores the lower and upper 16 bits of an input value into a global variable.
- `store_second_input_to_global`: Stores the lower and upper 16 bits of a second input value into a global variable.
- `execute_conditional_calls_2`: Evaluates bitwise conditions on a global variable to determine the number of times another routine is called.
- `execute_conditional_calls`: Evaluates multiple bitwise conditions on a global variable to determine how many times another routine is called.
- `process_input_state_recursion`: Evaluates a global variable against a set of specific character constants and, upon matching certain pairs, updates the variable to a terminal value and recursively calls itself twice.
- `clear_global_variable_2`: Sets a global variable to zero through a series of subtractions and conditional checks.
- `update_scaled_value`: Performs a complex bitwise manipulation and scaling operation on a global variable using bit shifts and masks.
- `process_input_and_state`: Manipulates a global variable and checks bits within a word retrieved from a memory offset, potentially calling another routine based on the results.
- `update_value_with_zero_flag`: Modifies a value pointed to by one register using a global variable, potentially toggling a byte flag if certain conditions are met.
- `update_value_with_ff_flag`: Modifies a value pointed to by one register using a global variable, potentially toggling a byte flag if it equals a specific non-zero value.
- `update_value_with_pointer`: Updates a value at a memory location using a global variable and a byte flag, with logic to handle zero-value cases.
- `update_value_with_ff_pointer`: Updates a value at a memory location using a global variable and a byte flag, specifically checking for a non-zero flag value.
- `transform_global_variable`: Updates a global variable by performing arithmetic and bitwise operations involving a value from a memory offset.
- `update_audio_state_logic`: Modifies a global variable and performs multiple conditional recursive calls to another routine based on bitmask flags.
- `update_audio_parameters`: Selects a destination pointer from a series of memory locations based on the contents of several global variables and writes multiple values to that location.
- `check_global_flags`: Checks four consecutive global bytes and calls another routine if any of them are non-zero.
- `update_entity_position_and_state`: Decrements a counter in a data structure and updates entity coordinates based on a global offset if the counter is non-zero.
- `construct_data_structure`: Calculates a value based on input registers and stores it into a new memory location, then initializes several fields within a structure pointed to by a global variable.
- `nop_operation`: The routine is empty and performs no operations before returning.
- `update_object_state`: Determines a movement offset based on a character value, checks for collisions within boundaries, and updates a data structure with new values and audio settings.
- `check_collision_bounds`: Adjusts coordinates based on input characters and iterates through tables to check for overlaps within specific boundaries.
- `toggle_global_arithmetic_state`: Modifies a global variable by adding it to itself if it is zero, or subtracting it from itself if it is non-zero, based on bitmask flags passed in a register.
- `calculate_variable_offset`: Updates a global variable by either subtracting or adding two values depending on the current state of the global variable.
- `update_global_arithmetic`: Modifies a global variable by either adding or subtracting two values depending on whether the current value of the global variable is zero.
- `initialize_data_tables_2`: Copies a block of data from one memory location to another and then performs a second block copy from a table based on a global index.
- `copy_table_data`: Copies eight 4-byte chunks from a table indexed by a global variable into a destination buffer.
- `fill_memory_tables`: Uses a value from a table to find memory addresses and fills those locations with specific byte patterns using loops.
- `fill_memory_with_pattern`: Iterates through a list of pointers derived from a global variable and fills the memory at those locations with a specific byte value.
- `fill_single_memory_range`: Fills a range of memory with a specific byte value if a pointer retrieved from a table is non-zero.
- `fill_memory_pattern`: Retrieves a pointer from a table and iterates through memory at fixed increments to write a specific byte value.
- `set_global_bytes`: Sets four consecutive bytes of a global variable to a specific value.
- `set_global_flags`: Sets a global variable to specific constant values based on its current state.
- `update_global_conditional`: If a global variable is non-zero, it sets it to a repeating byte value; otherwise, it sets it to a specific sequence of six bytes.
- `update_global_conditional_alt`: If a global variable is non-zero, it sets it to a repeating byte value; otherwise, it sets it to a different specific sequence of seven bytes.
- `dispatch_state_dependent_memory_updates`: Increments a global state variable and executes different memory manipulation routines based on its value.
- `execute_recursive_calls`: This routine performs four consecutive, identical calls to itself before returning.
- `load_byte_sequence`: Loads nine bytes from a table, using a global variable as an index, into a series of separate global variables.
- `load_byte_sequence_offset`: Loads nine bytes from a table into a different set of global variables using a global variable as an index.
- `load_byte_values`: Loads nine individual bytes from a table indexed by a global variable into several separate global variables.
- `load_masked_byte_values`: Loads nine bytes from a table indexed by a global variable into global variables after masking each byte with a bitwise AND operation.
- `update_sprite_positions`: Iterates through a table of coordinate data to calculate screen positions and updates hardware sprite attributes.
- `update_sprite_attribute_table_3`: Iterates through a data structure to identify active entries and updates sprite attribute data.
- `update_sprite_attribute_table`: Selects and writes a specific constant value to a memory location pointed to by a register based on the value in another register.
- `update_audio_sequence_and_parameters`: Copies a block of data from one memory location to another and then iterates through a table of structures to trigger specific audio update routines based on a type identifier.
- `interpolate_coordinate_values`: Adjusts two values in a memory block by incrementing or decrementing them toward a target value.
- `update_audio_parameter_coordinates`: Adjusts two coordinate values stored in memory by incrementing or decrementing them toward a target value.
- `select_audio_parameter_value`: Evaluates bits within a global variable to select and store a specific constant value into a memory location pointed to by a register.
- `update_audio_pitch_and_volume`: Checks a global variable to select a specific pitch and volume constant, then modifies a table of audio parameters.
- `update_audio_pitch_and_volume_2`: Checks a global variable to determine a specific frequency and volume value, then updates audio hardware parameters.
- `update_first_pointer_value`: Modifies the first word of a two-word structure by adding or subtracting a value based on flags in a control byte.
- `update_pointer_value`: Compares a value at a specific offset in a structure to a constant and either increments it by a fixed amount or sets it to a new constant value.
- `update_second_pointer_value`: Modifies the second word of a two-word structure by adding or subtracting a value based on flags in a control byte.
- `update_both_pointer_values`: Modifies both the first and second words of a two-word structure by adding or subtracting values based on flags in a control byte.
- `update_state_and_coordinates`: Increments or decrements a state byte within a structure and uses that state to add or subtract values from various offsets in a pointer-based structure.
- `update_entity_position_and_audio`: Updates the coordinates of an entity based on bitwise flags or boundary constraints, and triggers an audio channel configuration with specific pitch and volume parameters if certain conditions are met.
- `modify_coordinates_by_flags`: Checks specific bits in a byte to determine whether to increment or decrement values at two different offsets in a structure.
- `update_audio_parameter_state`: Updates a state byte based on comparisons between a global variable and values stored in a pointer-indexed structure.
- `update_audio_channel_2`: Manages audio playback by decrementing a counter in a structure, updating a position index in a table, and writing values to hardware registers based on specific bit flags and global variable comparisons.
- `configure_audio_channel_2`: Checks a global variable and a table value before iterating through hardware registers to find a zero value.
- `update_audio_channel_3`: Adjusts audio parameters by modifying a counter and a position index within a table, while performing conditional writes to hardware registers based on the state of a structure and a global variable.
- `configure_audio_channel_3`: Iterates through a series of memory locations to find a zero entry, then writes specific constants to hardware registers and updates a global variable.
- `update_audio_parameter_table`: Iterates through a table of audio structures to find an empty entry, then populates it with specific values and updates a secondary table of coordinates based on conditional logic.
- `update_audio_pitch_and_trigger`: Compares a global value against a range defined in a pointer to calculate a bitmask, then selects a specific frequency and length to update an audio structure.
- `decrement_and_update_structure`: Decrements a byte in a structure and, if it reaches zero, modifies a word and a byte within that structure before writing a value to a memory location.
- `decrement_and_update_structure_alt`: Decrements a byte in a structure and, if it reaches zero, modifies a word and sets a byte to zero within that structure before writing a value to a memory location.
- `decrement_and_branch_on_counter`: Decrements a byte value within a structure and recursively calls itself until the value reaches zero.
- `process_audio_sequence_buffer`: Iterates through a data structure to decrement a counter and update audio parameters.
- `map_character_to_address`: Compares a character input against several specific control characters and stores a corresponding address into a structure.
- `map_control_char_to_address`: Checks a character against a set of non-printable control characters and updates a structure with a specific address based on the match.
- `update_audio_sample_parameters`: Checks a global variable and a bitmask to select specific values from a table and writes them to memory.
- `configure_audio_parameters`: Checks a global variable against specific values and compares bits, then iterates through a table to update audio hardware registers and memory with period, length, and volume data.
- `clear_memory_block_2`: Fills a block of memory starting at a fixed address with zeros for sixteen consecutive double-word entries.
- `validate_tile_connectivity`: Checks a sequence of adjacent tiles in a grid to ensure they meet specific attribute and connectivity requirements.
- `resolve_entity_collisions`: Iterates through nested tables of coordinate data to detect overlapping bounding boxes.
- `update_audio_parameter_tables`: Iterates through multiple tables to check if a global value falls within specific ranges defined by pairs of short integers.
- `check_status_flags`: Checks a word at an offset within a structure for specific bit flags and calls another routine if any of those bits are set.
- `check_collision_bounds_2`: Iterates through tables to check if a specific object's coordinates overlap with the bounding boxes of other objects.
- `check_collision_bounds_alt`: Performs a collision detection loop by comparing coordinate ranges between an object and entries in a table.
- `update_global_on_collision`: Checks if a global variable is zero, then iterates through two different tables to see if the global variable's coordinates fall within the bounds of any listed objects.
- `increment_bcd_values`: Sets a global variable to a specific value and increments three BCD-encoded variables while performing BCD adjustments.
- `set_bcd_values`: Assigns a specific value to a global variable and updates three BCD-encoded variables with character constants and BCD adjustments.
- `update_bcd_counters`: Sets a global variable to a large value and increments two BCD-encoded variables while performing BCD adjustments.
- `apply_score_modifiers`: Iterates through a table to find an entry matching a specific bitmask.
- `fill_constant_data`: Stores a series of constant 16-bit values into a memory block relative to a pointer.
- `fill_global_data`: Copies the current value of a global variable into multiple consecutive 16-bit locations relative to a pointer.
- `process_audio_event_triggers`: Iterates through a table of data to check specific bit flags and compares values against a global variable.
- `update_audio_mixer_parameters`: Periodically resets a set of values in a table and then iterates through another table to modify two target values based on bitwise flags returned by a recursive call.
- `check_global_and_bit_status`: Tests if a global variable is zero and then checks if the most significant bit of a register is set.
- `update_status_flags`: Iterates through a table to check specific bit patterns in a byte array and updates a corresponding short value and a long value in a destination table.
- `select_data_pointer`: Compares values from two tables and, based on a counter and the value found, assigns a specific long value to a global variable or decrements the value in the table.
- `update_dynamic_parameter_table`: Calculates a value based on a global variable and its offset, then stores the result into a specific memory location.
- `configure_hardware_state`: Checks a bit in a memory location and, if set, performs several memory writes and a loop before writing a value to a specific offset.
- `configure_hardware_registers`: Writes a series of specific values to a global variable, which corresponds to configuring DMA and interrupt enable registers.
- `set_global_constant`: Stores the constant value three into a global variable.
- `toggle_audio_state`: Decrements a global variable and performs bitwise checks to set it to either zero or a specific maximum value.
- `find_optimal_audio_parameters`: Iterates through two tables to find a pair of values that minimizes a calculated sum, then updates a global pointer and configures audio hardware.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `recursively_evaluate_global_state`: Executes a recursive sequence of calls based on the value of a global variable, performing conditional arithmetic and bitwise operations before resetting the variable to zero.
- `process_audio_data_table`: Iterates through a data structure to perform arithmetic operations on bytes and stores the results back into the table.
- `copy_and_process_data`: Copies a block of memory, performs bitwise operations on a value retrieved from a table, and then updates a global variable based on several conditional range checks.
- `copy_and_branch_on_data`: Copies a block of data from a memory location to a destination table using a loop.
- `update_range_values`: Sets a global variable to one of several predefined long values based on its current value, then performs a bitwise NOT and a shift operation.
- `copy_modified_data`: Performs a bitwise NOT on a global variable, stores its shifted and original values into memory, and then copies a block of data from a source address to a destination address.
- `execute_nop_loop`: Executes a sequence of no-operation instructions within a loop that decrements a counter.
- `nop_sequence_execution`: Executes a sequence of fourteen instructions that perform no meaningful operation or state change.
- `update_bcd_game_state`: Updates BCD-encoded counters and processes a data table to modify a global variable based on specific conditional checks.
- `process_bcd_values`: Iterates through a data table to calculate a value, then updates several global variables using BCD adjustment logic.
- `decompress_bitstream_data`: Decodes a variable-length bitstream to populate a memory buffer.
- `parse_string_data`: Iterates through a string to calculate values and update a structure based on character offsets and table lookups.
- `tiny_ori`: Small routine of 1 instruction that merges 128 into d1.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.
- `clear_string_buffer`: Modifies a byte via a pointer and then performs string manipulation to replace characters with spaces based on a global variable length.
- `write_repeated_bytes`: Writes a specific byte to a memory location pointed to by a register three times, updating the value in a register between each write.
- `format_string_buffer`: Appends a colon to a string and replaces specific characters with spaces based on a global count.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d2.
- `return_from_subroutine_4`: The routine performs a single instruction to return control to the calling procedure.
- `increment_and_clear_memory`: Increments a global variable and, if it reaches a specific value, clears a large block of memory with zeros.
- `initialize_game_state_and_tables`: Copies a small block of data to a destination buffer and populates several global variables and memory regions with specific constant values.
- `execute_sequence_of_initializations`: Performs a series of data copies, memory fills, and table initializations, then decrements a global variable and conditionally calls another routine.
- `initialize_data_table`: Manages a global counter to conditionally populate a memory region with a series of predefined 16-bit constant values.
- `wait_for_bit`: Polls a specific bit in a byte located at an offset from a pointer until that bit is cleared.
- `set_audio_parameters`: Stores a constant, a global variable, and a second copy of that global variable into a series of offsets from a pointer.
- `initialize_audio_data`: Writes a constant, a global variable, and two copies of a constant into a table, then clears a global variable.
- `setup_audio_channel_one`: If a global variable is zero, it configures audio channel one with a specific frequency, length, and volume.
- `configure_audio_channel`: If a global variable is zero, it writes a constant, a value, and a volume level to specific hardware registers and a memory location.
- `setup_audio_channel_two`: If a global variable is zero, it configures audio channel two with a specific frequency, length, and volume.
- `initialize_audio_parameter_table`: Checks a global variable and, if zero, populates a memory table with a specific constant, a channel index, and a volume level before resetting the global variable.
- `clear_and_setup_audio`: Zeros out a block of memory starting at a global variable, then writes period, length, and volume values to two sets of audio hardware registers.
- `setup_audio_channel_three`: Writes a length, a period, and a volume value to the third audio channel hardware registers.
- `setup_audio_channel_zero`: Writes a length, a period, and a volume value to the first audio channel hardware registers.
- `clear_and_setup_dual_audio`: Zeros out a block of memory starting at a global variable, then writes period and length values to two different audio channels and sets their volume.
- `update_audio_channel_two`: Writes a length, a period, and a volume value to the second audio channel hardware registers.
- `setup_multiple_audio_channels`: Writes period, length, and volume values to two different sets of audio hardware registers and clears a global variable multiple times.
- `setup_audio_channel_one_alt2`: If a global variable is zero, it configures audio channel one with a different frequency, length, and volume.
- `configure_audio_channel_zero`: Sets the period, length, and volume for audio channel zero and enables the DMA channel.
- `reset_audio_state`: Sets a global variable to one and then calls another routine.
- `initialize_audio_channel_parameters`: Checks a global variable and, if zero, updates it and several memory locations with specific values before resetting the variable to zero.
- `initialize_audio_channel_state`: Checks if a global variable is zero, then updates that variable and several memory locations with specific values.
- `initialize_audio_channel_state_2`: Checks a global variable and, if zero, updates it and several memory locations with specific values.
- `initialize_audio_hardware_state`: Checks a global variable and, if zero, updates several memory locations with specific values and resets the variable to zero.
- `configure_audio_channel_two`: Sets the period, length, and volume for audio channel two and enables the DMA channel.
- `configure_audio_channel_two_2`: If a global variable is zero, it writes a different constant, a value, and a volume level to specific hardware registers and a memory location.
- `configure_audio_channel_three_a`: Sets the period, length, and volume for audio channel three and enables the DMA channel.
- `configure_audio_channel_three_b`: Sets the period, length, and volume for audio channel three with different parameters and enables the DMA channel.
- `conditionally_configure_audio_two`: If a global variable is zero, it sets the period, length, and volume for audio channel two and enables the DMA channel.
- `configure_audio_channel_three`: If a global variable is zero, it writes a third constant, a value, and a volume level to specific hardware registers and a memory location.
- `configure_audio_channel_four`: If a global variable is zero, it writes a fourth constant, a value, and a volume level to specific hardware registers and a memory location.
- `configure_audio_channel_five`: If a global variable is zero, it writes a constant, a value, and a lower volume level to specific hardware registers and a memory location.
- `initialize_audio_channel_parameters_2`: Checks if a global variable is zero, then writes a specific constant, a channel identifier, and a volume level to a memory location before resetting the global variable to zero.
- `configure_audio_channel_two_alt`: Sets the period, length, and volume for audio channel two with different parameters and enables the DMA channel.
- `update_variable_with_bounds`: Performs arithmetic on a global variable and stores the result into a memory location relative to a pointer, while also writing a constant to another location.
- `sequence_audio_parameter_updates`: Decrements a global counter repeatedly and, if the counter remains non-zero, writes a specific value and its bitwise complement to a memory location pointed to by a register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_4`: Small routine of 1 instruction that merges 4 into d0.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_memory_buffers`: Copies a block of data from one location to another and then clears several large memory ranges and a global variable to zero.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a loop and a table of offsets.
- `wait_for_hardware_bit`: Polls a bit in a memory location and writes a value to another location once the bit is set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Clears several memory ranges and sets multiple global variables to zero or specific constants.
- `clear_memory_and_execute_sequence`: Clears several blocks of memory using loops and then executes a sequence of calls to another routine with varying data.
- `setup_display_and_copy_tables`: Clears a memory block, copies three longword tables from one area into another, and programs the display window, bitplane, copper pointer and interrupt registers.
- `reprogram_display_on_flag`: Waits for a flag in a global to clear, then reprograms the display window, bitplane control and copper pointer registers.
- `call_seven_routines_then_reset`: Calls seven unnamed routines in sequence, then stores fixed values in globals, including a control word of 16 and a zero value.
- `toggle_frame_offset_pattern`: Toggles a flag word, then stores one of two sets of three offset values into a table depending on the result of the toggle.
- `clear_words_via_pointer_list`: Walks an array of longword pointers and stores zero into the word each one addresses, for a count taken from a stored value plus one.
- `set_pixel_bits_for_entries`: For each entry in a counted list, computes a screen offset from the entry's horizontal and vertical position, sets the matching bit in a 16-bit word of a bitplane, and stores that word's address in an output table.
- `adjust_entry_positions_with_limits`: For each entry in a list, calls an unnamed routine when its vertical value is below 21 or above 609, then subtracts a stored step from that value.
- `scale_entries_by_divisor`: For each entry, divides two coordinate fields by a third field and stores the two quotients in an output table.
- `compute_sprite_coordinates_from_inputs`: Converts three input values into two output words, each scaled by two and offset, and a third word from a scaled input.
- `update_checksum_byte`: Updates a stored checksum by rotating it left one bit, adding it to itself and mixing in a longword from memory with exclusive-or and subtraction.
- `increment_mismatched_colour_nibbles`: Waits for a hardware flag bit, then sixteen times increments each four-bit colour field that differs from its target value.
- `decrement_colour_nibbles_after_flag`: Waits for a hardware flag bit, then sixteen times decrements each nonzero four-bit colour field.
- `run_animation_script_commands`: Counts down a delay and, when it expires, reads the next command byte from a script.
- `call_three_routines`: Calls three unnamed routines in sequence and returns.
- `execute_sequential_subroutines_2`: Calls three separate subroutines in a specific order before returning.
- `execute_sequential_subroutines`: Calls three different subroutines in a specific order before returning.
- `execute_glyph_blits`: Calls another routine three times in succession.
- `blit_glyph_records_to_screen`: Draws a list of text records with the blitter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_table_values`: Iterates through a nested loop to increment nibble-based values within a table pointed to by a register.
- `decrement_table_values`: Iterates through a nested loop to decrement nibble-based values within a table pointed to by a register.
- `decrement_nibble_table`: Iterates through a loop to decrement nibble-based values within a table pointed to by a register, while updating several offsets in a structure based on a loop counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_max_byte_and_store`: Searches through a byte array to find the maximum value and stores it into a structure at an offset calculated from that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_repeatedly`: Repeatedly stores a 16-bit value into a memory location pointed to by a register until a global variable reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_max_byte_and_clear`: Searches through a byte array for the maximum value and, upon completion, clears a 32-bit value in a structure at an offset calculated from that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_nibble_packed_words`: Iterates through a sequence of words, treating each word as three packed nibbles and decrementing each nibble by one if it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_decrement_sprite_data`: Locates a specific entry in a table based on a byte comparison loop, clears a value at a calculated offset, and then performs a bitwise decrement on nibbles within a sequence of words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_nibble_data_block`: Iterates through a memory block and decrements each nibble within the words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_and_fill_table`: Searches a memory area for a specific byte value and then performs a nested loop to fill a destination table with calculated word values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_table_nested`: Performs a nested loop to write calculated word values into a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_fill_data`: Clears a series of memory locations and then executes a nested loop to fill a table with calculated word values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_table_masked`: Executes a nested loop to write word values into a table where the lower nibble of the source value is masked.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_fill_memory_table`: The routine clears a block of memory using a loop and then performs a nested loop to populate a table with decremented values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_table_nested_loop`: Executes a nested loop to write calculated word values into a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_fill_masked`: Clears a series of memory locations and then fills a table with word values using a bitwise OR operation and a nibble mask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_nested_loops`: Performs nested loops to write bitwise ORed values from three registers into a memory location pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_write_data`: Clears a memory range, iterates through a table to update a pointer, and then performs nested loops to write bitwise ORed values to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bitmask_data`: Writes bitwise ORed values to memory in a loop, then waits for a specific bit in a memory location to be set before updating several values in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_write`: Iterates through a table to update a pointer based on values found at an offset, then performs nested loops to write bitwise ORed values to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_bitwise_or_data`: Uses nested loops to write bitwise ORed values from three registers into a memory location.
- `wait_for_bit_and_return`: Polls a memory location until a specific bit is set, writes a value to another memory location, and returns a constructed word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_variables`: Sets a global variable to zero multiple times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_globals_and_return_word`: Sets a global variable to zero multiple times and returns a word constructed from a high byte of a memory value and a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_volume_register`: Sets a global variable to zero three times and modifies the third audio channel volume register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `merge_table_values`: Iterates through two tables, comparing bit segments of values and updating a destination table with merged bitwise results.
- `initialize_hardware_and_audio`: A 48-instruction routine that touches various hardware registers including DMA, Blitter, and interrupt control, and calls routines to reset audio channels and initialize memory buffers.
- `reset_audio_channels`: Sets a global variable to a specific value, calls another routine, and writes a byte to a structure offset.
- `update_structure_fields_2`: Updates several fields within a structure pointed to by a register using constant values and results from other routines.
- `update_entity_transform_data`: Updates a series of data structures within a block of memory using values derived from registers.
- `update_structure_loop`: Updates a structure with various values and performs a loop that writes to two memory locations and updates a field with a bitwise combination of registers.
- `update_structure_single_pass`: Updates a structure with specific values and writes a bitwise combination of registers to a field before returning.
- `calculate_input_state`: Reads a 16-bit value from a structure and performs bitwise logic to return a processed status byte.
- `process_encoded_data`: Calculates an encoded value using arithmetic and bits from a global variable, then stores a 16-bit result into a structure by splitting it into two bytes.
- `calculate_encoded_value`: Performs arithmetic operations on a value and combines it with bits from a global variable to return a concatenated result.
- `return_from_subroutine_2`: This routine performs no operations and immediately returns to the caller.
- `store_word_split`: Splits a 16-bit value into two bytes and stores them at specific offsets within a structure.
- `update_global_with_bit`: Reads values from other routines and stores them into a structure with the high bit set.
- `update_bitmask_via_pointer`: Retrieves a value from another routine and stores it at a memory location pointed to by a register, applying a bitwise OR with a specific bit set.
- `process_input_stream`: Parses a character stream from a pointer, performing calculations based on character values and updating a structure with various derived parameters.
- `wait_for_bit_clear`: Polls a byte within a structure until a specific bit is cleared.
- `enter_infinite_loop`: Enters an infinite loop that performs no operations.
- `initialize_memory_buffers_and_tables`: Clears several memory regions with zeros and fills others with specific bit patterns.
- `decrement_global_counter`: Decrements a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `increment_global_counter`: Increments a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `initialize_data_tables`: Fills several consecutive memory areas with repeating sequences of four 16-bit values and then writes a set of 16-bit values to a structure pointed to by a global variable.
- `decrement_table_values_2`: Iterates through a table of values, subtracting a value derived from a global variable from each entry and clamping the result to zero or a specific constant.
- `update_recursive_state_tables`: Updates a table of values by adding a global variable to each entry and clamping the result to zero if it exceeds a threshold.
- `update_structure_offsets`: Calculates a base value from a global variable and a pointer, then stores several derived 16-bit values into a structure at specific offsets.
- `transform_string_data`: Iterates through a sequence of bytes, modifies each byte based on specific character matches, and writes the results to a new memory location.
- `update_loop_counters`: Decrements a global variable and a loop counter repeatedly, resetting the loop counter to a specific value if it becomes negative.
- `store_calculated_pointers`: Calculates a memory address based on a value from a pointer and stores the resulting address and its high word into several global variables.
- `tiny_ori_6`: Small routine of 1 instruction that merges 23 into d4.
- `return_from_subroutine_3`: This routine contains a single instruction and is used as a return point via an absolute move.
- `tiny_ori_sub`: Small routine of 2 instructions that merges 3 into d0, then subtracts d0 from d0.
- `check_global_flag`: Checks if a global variable is zero and, if so, clears a memory location before calling another routine twice.
- `copy_memory_block`: Copies a block of data from a source location to a destination location using a loop that processes 20 bytes per iteration.
- `return_immediately`: This routine contains a single instruction and performs no operations before returning to its caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_sorted_pointer_table`: Scans a byte table for the first entry greater than a rising threshold, then fills a table of 30 pointers with addresses stepped by a per-entry size and clears the memory they point to.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `restart_four_audio_channels`: Advances a tick counter and, when it wraps, calls four unnamed routines, waits through two delay loops, sets the DMA enable bit and copies four audio channel pointer pairs from a table.
- `process_tracker_channel_row`: Reads a four-byte music pattern cell, looks up instrument data in a 30-byte-per-entry table, and updates the channel's period and volume.
- `compare_period_to_target`: Masks a 12-bit period word and stores it in a second field, clearing a flag byte.
- `update_tracker_channel_effects`: Applies the per-tick effect of one music channel.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_channels`: Writes to the DMA controller register and initializes a structure with a constant, a large integer, and a global variable.
- `initialize_structure_data_2`: Initializes a structure by writing a constant, a large integer, and a global variable to specific offsets.
- `configure_hardware_interrupts_and_dma`: This routine modifies the DMA control and interrupt enable registers while interacting with the interrupt request register.
- `return_from_initialization`: This routine is empty and contains a single instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `stub_return_only`: Contains only a return and does nothing.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### disk.1:chunks 144-757 (bytes 36864-194048)+232#unpacked

- `initialize_hardware_and_load_tables`: Configures custom chipset registers, clears a memory block, and performs several data copying operations between tables.
- `perform_double_division`: Performs two successive unsigned divisions using the values in d2 and d1, swapping the high and low words of the operands and results throughout the process.
- `update_table_values`: Writes a byte from d0 to offset 2 of a structure in a0, then loops through a table starting at offset 12 of a0, performing calculations involving multiplication and subtraction to update bytes at offset 15 of each table entry.
- `process_ascii_strings`: Calls another routine to generate a character, stores it in a buffer, and then enters two separate loops to either clear or copy characters based on values from global variables and a source buffer.
- `copy_ascii_bytes`: Copies bytes from a source in a0 to a destination in a1, using a loop controlled by d1 and a value from a global variable, while checking if the source byte is 48.
- `write_ascii_bytes`: Writes the byte 48 or 32 to a destination pointed to by a1, incrementing the pointer, within a loop that runs based on a value in d1 and an offset in a0.
- `clear_word_and_jump`: Clears a 16-bit word at a global address, calls another routine, and then moves a value from a structure in a6 to d0 before jumping to another routine.
- `execute_sequence_and_jump`: Calls another routine, calls a second routine, writes 65535 to a global variable, moves a value from a structure in a6 to d0, and jumps to another routine.
- `initialize_global_pairs`: Writes a sequence of sixteen 16-bit values to a series of global variables, alternating between the values 4 and 18332.
- `calculate_status_flags`: Reads a 16-bit word from a structure in a6, performs bitwise tests to set various bits in d0 and d1/d2, and updates a global variable based on a byte read from a specific address.
- `clear_bit_four`: Clears bit 4 of d0 and returns.
- `copy_and_format_data`: Clears several memory locations, then uses a nested loop to copy bytes from a source table to a destination table, applying an offset of 48 to certain bytes and performing a secondary copy of 16 bytes from a different source.
