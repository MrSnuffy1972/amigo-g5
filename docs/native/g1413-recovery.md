# Game 1413 (SoccerKid v1.0 1129): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01413). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (8192748 top-level bytes in 127 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 118 of 118 units identical (bytes and relocations), 853 raw-word lines |
| Functions named and described | pass, 881 of 881 named (0 author symbols), 881 of 881 described (40 by rule, 808 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 11508 of 11517 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 8704 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19830 of 19882 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22203 |
| code | 73798 |
| data | 8096229 |
| padding | 6 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 556 | 39 |
| Disk.1:bootblock | 1 | 164 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 95 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 218 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 97 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 119 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 50 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 63 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14901 | 567 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2490 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| Disk.1 bytes 511488-520192 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.1: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.3: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.4: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129.hdf: hard-disk image (AmigaDOS OFS, 8672 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree SoccerKid_v1.0_1129_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 73 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 94 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 62 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 97 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 8990 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.1: bytes 511488-520192 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (58 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +186, SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +17A
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
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +1B0
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/game.slave +1B0
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.1: 1 logged loads cover 8704 bytes of the image
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.1: 3 track-loaded blocks found in recovered runtime segments and classified through them
- SoccerKid_v1.0_1129/SoccerKid_v1.0_1129_hdf/SoccerKid/Disk.1: blocks 1017-1304 (bytes 520704-668160) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.45)
- 1 never-loaded region(s), 8704 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 5 packed track region(s), 3442176 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_constants` (runtime segment an address): Writes multiple constant values to a global variable and a memory location, while interacting with various hardware registers including DMA and interrupt control. Kind: hardware/constant initialization for disk access. Pass path: not determined.
- `initialize_system_tables_and_hardware` (runtime segment an address (play-long-joy)): Clears several memory regions and populates tables with incrementing values, pseudo-random data, and constant sequences. Kind: disk hardware/track check. Pass path: execution continues to the main game/application logic after the initialization of system tables and hardware constants.
- `write_dma_control_block` (runtime segment an address (play-long-joy)): The routine writes values to several memory locations and hardware registers to configure a DMA transfer. Kind: DMA control block setup for disk hardware access. Pass path: The routine writes a custom DMA descriptor to RAM, which is then used by the disk controller (ADKCON) to perform a specific hardware read/sync operation. A successful check allows the execution to continue to the next instruction in the cal.
- `write_dma_control_block_indexed` (runtime segment an address (play-long-joy)): The routine selects a pointer from a table using an index and writes values to memory and hardware registers to configure a DMA transfer. Kind: DMA control block manipulation for custom disk access. Pass path: not determined.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clear_low_memory_write_tags`: Calls through a library vector, clears a block of low memory, then writes a sequence of text tags into it.
- `dispatch_subroutine_tasks`: This routine executes a sequence of tasks including memory block manipulation, bitstream decompression, bit-shifting operations, and tag comparison.
- `execute_multiple_tasks`: Executes twelve instructions that include locating a memory block to write jump opcodes, writing a character to a disk label, and calling another routine.
- `patch_jump_vectors_into_block`: Locates a memory block through a library call, then writes jump opcodes and routine pointers at many offsets within it.
- `return_input_register_value`: Returns the data register D0 unchanged, with no other work.
- `store_longword_and_add_offset`: Stores a longword to the location just before a pointer, then returns a value formed by adding the lower three bytes of another longword to the input count.
- `return_data_register_unchanged`: Returns the data register D0 unchanged.
- `return_d0_unchanged`: The body only returns the data register D0 unchanged, with no other effect.
- `return_d0_value`: Returns the current value of a data register.
- `return_d0_value_2`: Returns the contents of a data register to the caller.
- `return_d0_value_3`: Passes the value held in a data register back to the caller.
- `return_d0_value_4`: Returns the value stored in a data register.
- `return_d0_value_5`: Provides the value from a data register as a return value.
- `return_d0_value_6`: Returns the value of a data register.
- `return_d0_value_7`: Returns the data register value to the calling routine.
- `return_d0_value_8`: Returns the value contained in a data register.
- `return_uninitialized_register`: This routine returns the current value held in a data register without performing any operations.
- `return_d0_value_9`: Returns the value of a data register to the caller.
- `return_d0_value_10`: Returns the current value of the D0 register.
- `return_uninitialized_register_2`: The routine returns the current value held in a data register without performing any operations.
- `store_masked_longword_and_word`: Stores a longword read from a source pointer, kept to its lower three bytes, at the location four bytes before a pointer, then stores a word value at the pointer.
- `update_memory_words`: Stores a 24-bit value from a pointer into a memory location and stores a 16-bit value from another register into the same location.
- `read_masked_longword_offset`: Reads a longword at a base pointer plus a 16-bit offset and returns only its lower three bytes.
- `add_masked_longword_at_offset`: Reads a longword at a base pointer plus a 16-bit offset, keeps its lower three bytes, and adds them to the incoming count, returning the sum.
- `store_sum_of_masked_longword`: Adds the lower three bytes of a longword read from a source pointer to the incoming count, and stores the sum at a destination pointer.
- `store_two_longwords_before_pointer`: Stores one longword at the location four bytes before a pointer and the lower three bytes of a source longword eight bytes before it, then returns the data register D0 unchanged.
- `return_register_value`: This routine returns the current value held in a data register to the caller.
- `write_two_digit_decimal`: Adds one to a stored value and writes its tens digit and units digit as two separate bytes at the destination pointer.
- `decompress_bitstream_data`: Reads a bitstream from memory to expand compressed data into a destination buffer.
- `run_bit_shift_byte_loop`: Runs a bit-shift loop over a seed word formed by XOR of two stored words, refilling the shift register from a called routine whenever it empties.
- `rotate_longword_right_through_extend`: Rotates a longword read from a pointer one bit to the right through the extend flag and returns the result with its top bit forced on.
- `rotate_multiword_bits_right`: Shifts a block of longwords right by one bit per pass, feeding each word's low bit into the next through the carry.
- `check_save_game_tag`: Compares a stored two-character tag with a fixed value.
- `invoke_library_vector_return`: Calls a routine through a function pointer taken from a base table at a fixed offset, then returns the data register D1 unchanged.
- `set_disk_label_digit_one`: Writes the character one into the disk label text entry, calls another routine, and returns the data register D1 unchanged.
- `write_disk_label_digit`: Computes a disk number character from the low four bits of the data register D3 and writes it into the disk label text entry.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `invoke_library_vector_pair`: Calls a routine through a function pointer at a fixed offset of a base table and returns the data registers D0 and D1 together.

### Disk.1:bootblock

- `load_boot_track_via_io`: Entered from the boot block through the system loader, it calls a library routine to look up a device and, on success, fills in an I/O request block and submits it with the exec library DoIO call.
- `install_vectors_copy_block`: Writes three routine addresses into a fixed low memory vector slot, clears a shared word, and copies a block of twenty-seven words from a routine to a fixed memory location.
- `copy_word_block`: Calls a routine that copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `copy_word_block_forward`: Copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `write_text_marker_by_flag`: Writes the four-character text marker FAT0 to a destination pointer, or FAT1 when a bit of a shared state word is set.
- `detect_refresh_rate_hz`: Resets the CIA port and timer registers, then reads a counter from two bytes and compares it with a fixed threshold.
- `wait_status_byte_clear`: Polls a status byte until its top bit is clear, returning minus one.
- `clear_buffer_and_fill_tags`: Clears a fixed-size buffer, writes two four-character text tags into it, then calls two other routines.
- `walk_list_until_bit_set`: Follows a linked list of nodes, testing in each node a bit selected by a counter, and stops at the first node whose bit is set or at the end of the list.
- `walk_list_from_given_node`: Walks the same kind of linked list starting from the pointer already held in a register, stopping at the first node with the selected bit set or at the end.
- `store_control_word_or_halt`: If a shared flag is clear, it stores a computed control word into a structure and returns.
- `copy_longword_and_tag`: Copies one longword from a source pointer to a destination pointer, then stores a tagged word that combines the data register D0 with a fixed two-letter tag into a structure field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_interrupts`: Sets a pointer, configures DMA and certain interrupt enable bits, sets certain hardware registers to a fixed value, disables interrupts, and clears a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_trap_sequence`: Executes multiple software traps and updates various global variables and memory locations in a sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_state`: Writes multiple values to memory and performs a software interrupt before calling two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_globals`: Writes values to hardware registers including CIAA, CIAB, DMAC, and INTENA, while also updating several global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_interrupts_and_dma_2`: Sets multiple global variables and performs a sequence of writes to hardware registers to enable interrupts and DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_hardware_registers`: Calls another routine and then writes specific values to a memory location and several hardware registers including CIA and DMA control registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_data`: Assigns several constant values to global variables and a memory location before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_interrupts`: Sets multiple global variables to zero and writes specific values to hardware registers including the CIAA port, color registers, DMA control, and interrupt enable registers before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_globals`: Writes several long values to global variables and a memory location, then enables DMA by writing to the DMACON register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_registers`: Sets several global variables and hardware registers, including CIA and custom registers, before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_parameters`: Writes a sequence of values to several global variables and sets a hardware register to a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dma_and_interrupts`: Sets hardware registers for color and DMA, calls another routine, and writes a sequence of values to global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_dma_parameters`: Sets a global variable to zero and writes a sequence of values to several other global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers`: Writes values to memory locations and performs bitwise OR operations on specific memory addresses to modify hardware register states.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_execute_jump_table`: Sets hardware registers for interrupts and DMA, then executes a series of functions from a table based on a calculated index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_state`: Modifies memory locations, calls multiple routines, and performs bitwise OR operations on specific memory addresses to update hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_dma_and_registers`: Writes a sequence of values to global variables and performs bitwise OR operations on specific memory addresses to modify hardware register states.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Sets several global variables to specific constant values and calls three other routines before terminating.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_constants`: Writes multiple constant values to a global variable and a memory location, while interacting with various hardware registers including DMA and interrupt control.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_word`: Calls several other routines and then performs a bitwise OR operation on a word at a specific offset from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_interrupts_and_dma`: Updates hardware control registers to enable interrupts and DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitmask`: Executes a sequence of other routines and then applies a bitwise OR mask to a word stored at an offset from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_structures`: Sets multiple multi-byte values into memory locations pointed to by registers and updates various global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state`: Updates multiple memory locations by performing bitwise OR operations with specific bitmasks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_cia_port_registers`: Modifies the Port A and Port B data registers of the CIAA and CIAB chips by performing bitwise OR operations with specific bitmasks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state_2`: Performs bitwise OR operations on several memory locations to set specific bits, likely representing button presses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_cia_peripheral_registers`: Writes several constant values to memory locations and updates the CIAA and CIAB Port A registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_dispatch_table`: Sets several global values and performs a bitwise OR on a memory location before executing a loop that calls routines from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_cia_port_registers_2`: Writes specific bitmask values to the CIAA and CIAB Port A and Port B registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_with_memory_setup`: Sets specific memory values and performs bitwise OR operations before executing a table-based routine loop and an indirect call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_peripheral_status`: Performs a bitwise OR operation on a memory location using a byte from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_table_loop`: Executes a loop that calls routines from a table based on a calculated index and then performs an indirect call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_cia_port_registers_3`: Writes values to the CIAA and CIAB Port A registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state_3`: Reads a value from a memory location and stores it into another memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_peripheral_registers`: Writes certain values to a memory-mapped location, sets up two pointers, and writes certain values to hardware registers or a table before reading from a register into a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_indexed_function_table`: Fetches a value from a routine, scales it, and uses it as an index to call a function from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indexed_function_table`: Calculates an offset based on a value derived from a register and executes a series of functions from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indexed_function_table_2`: Calculates an offset from a value in a register to call a sequence of functions from a table in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indexed_function_table_3`: Calculates an offset based on a value derived from a register and executes a function from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indexed_function_chain`: Enters a loop that uses a signed value to index into a table of function pointers, executing each called routine and updating the index based on the return value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_hardware_registers`: Writes certain values to hardware registers or a table, reads from two registers into a register, and then writes a new value to a register or a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_through_input_table`: Calculates an index based on a value, uses it to fetch and execute a routine from a table, and repeats this process until the calculated index is negative.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_memory_constants_2`: Writes several constant bytes to memory locations relative to two different pointers and increments a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_cleanup_routine`: Sets two global variables and then calls another routine via an indirect jump.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_memory_offsets`: Writes constant bytes to memory locations relative to two different pointers and increments a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_jump`: Sets a global variable to eight, clears a value at an offset from an unspecified pointer, and performs an indirect call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_blocks`: Writes specific byte values into several memory offsets relative to two different pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call_2`: Performs an indirect call to an address stored in a jump table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_table_values`: Writes specific byte and word values to offsets of two different pointers and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_store_result`: Executes a loop that calls a routine repeatedly, storing a high-word result into a location pointed to by a register until a return value is one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_copy_table`: Sets specific byte values at offsets within memory pointed to by registers, performs bitwise operations on a memory location, and copies a sequence of words from a table into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_copy_table_2`: Sets specific bytes in memory, waits for a global variable to become non-zero, and then copies a sequence of words from a table to a destination in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_copy_table_3`: Performs memory writes, waits for a global variable to be non-zero, and copies data from a table to a new location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_4`: The routine performs a single return instruction to exit the current execution flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_copy_data`: Decrements a value through a pointer, waits for a global variable, copies a table to memory, and then stores the result of a function call into a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_subroutines_and_loop`: Calls two subroutines and then enters a loop that calls a third routine until it returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_table_and_loop`: Waits for a global variable, copies a table to memory, and then enters a loop that calls a routine until it returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_return_value`: Calls a routine to obtain a value and then enters a loop that calls another routine until it returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_table_and_wait_for_limit`: Waits for a global variable, copies a table to memory, waits for a global variable to reach a threshold, and then enters a loop that calls a routine until it returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_return_value_2`: Enters a loop that calls a routine and stores the result until the routine returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_wait`: Copies a block of data, waits for a global variable to reach a threshold, and then performs a loop to extract high words from a function return into a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `extract_high_words`: Repeatedly calls another routine and stores the upper 32 bits of the returned 64-bit value into a destination until a specific return value is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_poll`: Copies a block of data, waits for a global variable to reach a threshold, and then sets a global variable to a specific value before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: Calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_wait`: Sets a global variable to zero, waits for it to reach a threshold, and then resets it to a specific value before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_subroutine`: Executes another routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_reset`: Waits for a global variable to reach a threshold, sets it to a specific value, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values`: Waits for a threshold, then enters a loop that fetches values from a table and adds them to a destination buffer based on indices returned by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_data_loop`: Iteratively fetches values from a table and adds them to a destination buffer using indices provided by another routine until a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values_2`: Iterates through a list of indices to fetch values from a table and add them to a destination memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values_loop`: Repeatedly fetches values from a table using indices and adds them to a destination memory location until a termination condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_memory_values`: Iterates through a sequence of values retrieved from a function to perform additions into a destination memory location using an offset derived from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values_while`: Performs a loop that adds values retrieved from a table to a destination memory location based on an initial index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_and_wait`: Clears a specific bit in a byte and enters a loop that waits for a global variable to reach a certain value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_values_from_list`: Iteratively adds a value to memory locations pointed to by a list of offsets, updating the value from a table periodically.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_interrupts`: Sets specific bits in the DMA control and interrupt enable registers while modifying the copper list pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_set_bits`: Decrements a value and enters a loop that performs bitwise operations on a byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_interrupts_and_dma_3`: This routine modifies the interrupt enable and DMA control registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `compare_memory_values`: Calls another routine and then compares two 16-bit values located at specific offsets from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_interrupts`: A seven-instruction routine that modifies the DMA and interrupt enable registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_memory_threshold`: Compares two 16-bit values at offsets from a pointer and returns a specific value if the first is less than or equal to the second.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_dma_and_interrupts`: A six-instruction routine that accesses the DMA and interrupt enable registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_condition_flag`: Checks a boolean condition and returns a specific value if the condition is false.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `access_system_registers`: A four-instruction routine that interacts with the DMA, interrupt enable, and interrupt request registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_control`: A three-instruction routine that modifies the DMA control and interrupt request registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_routine`: Calls another routine and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_dma_control`: A two-instruction routine that modifies the DMA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_port_state`: The routine has 15 instructions, but the supplied Ghidra pseudo-code shows only an empty return, so the body could not be checked against the listing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_instruction`: This routine consists of a single instruction that performs no operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_nineteen_instructions`: The routine consists of nineteen instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_instruction`: This routine consists of a single instruction that performs no operation.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state_buffer`: The routine retrieves a value from another routine and stores it into three consecutive word-sized locations within a structure pointed to by a register, then clears two subsequent bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines`: The routine calls three consecutive subroutines before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sort_linked_list`: Initializes several fields in a structure and then calls a routine to sort a linked list.
- `sort_linked_list_nodes`: Performs a comparison-based sort on a linked list of data structures.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_uninitialized_stack_value`: The routine returns a value located at the current stack pointer position without performing any operations.
- `execute_sequence_of_functions`: calls four different routines and then conditionally executes a fifth routine within a loop based on a register value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sort_table_elements`: iterates through a table of structures, comparing specific bitfields and swapping elements if the condition is met.
- `store_and_fetch_value`: Stores a longword into a structure and returns a value from a table using an offset derived from a structure field.
- `copy_table_word`: Copies a word from a pointer into a structure field and then calls another routine with an incremented pointer.
- `update_color_and_pointer`: masks a value, calls one of two routines based on a bit test, and stores the result into a table at an offset determined by a structure field.
- `increment_on_nonzero_flag`: checks a byte at a fixed offset in a structure and, when that byte is nonzero, clears it to zero and increments the adjacent short value.
- `init_global_pointer_fields`: stores the address of a global variable into a structure pointer slot, writes a data register value and a zero into two structure fields, then calls one unnamed routine.
- `increment_pointer_and_decrement_counter`: Checks a value in a structure; if it is between one and three, it reads a word from a pointer, increments that pointer twice, and decrements the structure value by two.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_with_counters`: Executes a loop that increments a byte at a pointer until it reaches a specific value, then performs conditional checks and calls other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_pointer_and_decrement_short`: Increments a long value pointed to by an offset in a structure and decrements a short value at another offset.
- `read_four_bytes_with_decrement`: reads a longword from a pointer, increments that pointer by two, and decrements a counter in a structure after calling an initialization routine.
- `increment_pointer_and_decrement_counter_2`: Checks if a short value in a structure is zero, then reads a value from a pointer, increments that pointer, and decrements the short value in the structure.
- `dispatch_jump_table_entry`: Performs an absolute jump to an address retrieved from a jump table.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `decompress_bitstream_data_2`: Processes a bitstream by extracting bits from a source and using them to determine how many bytes to copy from a source pointer to a destination.
- `decode_bitstream`: processes a stream of bits to extract values and write them into a buffer, using bitwise shifts and conditional logic to handle variable lengths.
- `extract_bit_stream`: Extracts bits from a longword source and stores them into a destination buffer using a variable number of bits per entry.
- `extract_bit_stream_variant`: Processes a longword to extract bits and writes them to a memory location based on conditional logic and bit shifts.
- `extract_bit_stream_loop`: Reads bits from a longword and writes them to a destination pointer, using a loop to handle bit-level extraction.
- `rotate_right_extend_long`: Returns a 32-bit word read from memory, shifted right one bit, with the extend-flag bit placed at the top and the top bit forced set.
- `shift_bits_loop`: Iterates through a loop to shift bits from a longword and perform bitwise operations based on the input value.
- `shift_and_store_bits`: Shifts bits from a longword into a destination, incorporating a carry bit and a fixed bit into the result.
- `decompress_bitstream`: Reads a bitstream from a source and reconstructs data by expanding bits into bytes and writing them to a destination memory area.
- `expand_bitstream_data`: Iterates through a bitstream to reconstruct or expand data by shifting bits from a longword and writing the resulting values to a memory buffer.
- `init_bitstream_state_fields`: Sets a 16-bit field to a fixed constant, copies a 32-bit value into another field, and adds 11 to a short field.
- `sum_tagged_ram_records`: Scans 64 entries of a fixed low-memory table and, for each entry whose tag matches the text RAM followed by M, adds the 32-bit value stored eight bytes after the tag to a running total, which it returns.
- `find_table_entry_by_key`: Searches a 64-entry table in a fixed low-memory area for a 32-bit key equal to a data register value.
- `initialize_and_verify_memory`: Clears a block of memory with zeros and performs several checks involving table lookups and memory size comparisons.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a0), then clears (a1), then returns.
- `accumulate_contiguous_ram_records`: Walks a chain of records tagged RAMP while each record's start address continues from the previous one, adding their sizes into a running total.
- `append_ram_split_record`: Finds the first empty slot in a table of four-long records and writes a RAMD tag record there.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_byte_with_threshold`: Checks if a value at a specific offset in a structure exceeds a threshold and sets a bit in a byte before storing it at a new location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: The routine is empty and performs an infinite loop.
- `write_memory_block`: Writes three specific longwords to a memory location pointed to by a register.
- `call_validate_magic_constant`: calls a routine that checks a register for a specific magic constant and performs conditional memory allocation.
- `validate_magic_constant`: Checks if a register contains a specific magic constant and performs conditional memory allocation based on the result.
- `execute_wrapped_routine`: Sets a byte at a fixed memory location to its maximum value, calls another routine, and then resets that byte to zero.
- `set_flag_around_routine_call`: Sets a byte at a fixed memory address to 255, calls another routine, and then clears that byte to zero.
- `validate_and_allocate_memory`: Checks if a register contains a specific magic constant and if a memory location is non-zero, then calls a memory allocation routine twice.
- `allocate_memory_block`: This routine calls a memory management function to reserve a block of memory from a pre-allocated pool.
- `allocate_block_from_pool`: Allocates a block from a pool whose remaining size is held in a word, rounding the requested size up to a multiple of four and adding a four-byte header.
- `initialize_zeroed_table`: Sets a longword to zero and then populates a table with zeroed entries based on a calculated index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_to_offset`: Stores a byte from a register into a memory location at a fixed offset from a pointer.
- `process_conditional_logic`: Executes a series of subroutines and conditional loops based on values retrieved from a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_range_2`: Clears a range of memory by writing zeros to a destination address incremented by a specified size.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies (a7)+ to d0, then loads 0 into d0, then returns.
- `clear_memory_range`: Fills a range of memory with zeros using a loop if a specific flag is not set.
- `call_routine_once`: Calls another routine once and returns its result.
- `return_routine_result_once`: Calls one unnamed routine once and returns its result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine performs a single return instruction to exit the current execution flow and return to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_conditionally`: Calls another routine based on the state of a zero flag and a boolean variable.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `return_pair_after_hardware_call`: Has the same code as the previous routine and returns the two data registers unchanged after one call to an unnamed routine.
- `process_joystick_input`: The routine performs eight instructions and is invoked by a single caller.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `parse_and_load_copper_list`: Parses a custom data structure by searching for specific magic markers and extracts bit-packed color or display information.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_data_table`: Checks a global variable to select a starting value and then fills a table with a series of incrementing values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_byte_value`: Checks the value of a byte and calls one of two different routines depending on whether the value is zero or one.
- `nop_padding_sequence`: This routine is empty and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_stack_routines`: Calls a sequence of several different routines in order.
- `return_stack_values`: Returns two short values stored on the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_stack_values_2`: This routine retrieves two short integers from the stack and returns them as a single 32-bit value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `decrement_global_variable`: Decrements a global variable and returns its new value.
- `perform_nested_loops`: Executes nested loops that decrement a global variable and perform memory copies and arithmetic operations.
- `copy_memory_blocks`: Copies a sequence of bytes from one memory location to another by repeatedly calling another routine to fetch values and incrementing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_data_table_2`: Selects a starting value based on a global variable and writes a sequence of incrementing values into a table.
- `modify_global_variable`: Performs bitwise operations on a global variable and updates it based on the results.
- `update_status_flags`: Checks bits in a local variable to conditionally modify global variables and memory locations.
- `execute_absolute_jump`: This routine performs a single absolute jump to a fixed address.
- `pass_through_jump_table`: This routine calls another routine and returns a concatenated value from two data registers.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_d4_constant`: Loads the value two hundred fifty-five into data register four and returns.
- `initialize_system_tables_and_hardware`: Clears several memory regions and populates tables with incrementing values, pseudo-random data, and constant sequences.
- `fill_incrementing_table`: Stores three 32-bit values into a table by incrementing a base value by a fixed amount for each entry.
- `fill_table_with_values`: Stores a sequence of three 32-bit values into a table by incrementing a base value by a fixed amount for each entry.
- `fill_two_tables`: Performs two separate loops to store two sets of three 32-bit values into a table using specific increments.
- `update_pseudo_random_table`: Iterates through a table of integers, updating each entry by multiplying it by two and conditionally applying a bitwise XOR operation based on a flag.
- `update_blt_and_pattern_table`: Updates a bit pattern table by performing a bitwise XOR and multiplication on values, then configures several blitter hardware registers and memory locations for graphical operations.
- `update_blt_parameters_from_table`: Iterates through a table, calculating a new address and updating several bitplane and blitter registers or a global variable based on the table contents.
- `transform_nibbles_in_memory`: Iterates through two memory buffers, extracting nibbles from words in the source and reassembling them into a new pattern in the destination.
- `populate_data_table_3`: Clears a specific memory location and waits for it to reach zero before copying blocks of data from a source table to a destination table in a nested loop structure.
- `copy_memory_blocks_2`: Repeatedly clears a memory location and copies data from one table to another using a loop.
- `set_blt_parameters_fixed`: Sets several hardware registers and a global variable to fixed or calculated values.
- `configure_blitter_registers`: Writes several values to blitter hardware registers and waits for a specific bit in a global variable to clear.
- `manipulate_global_and_branch`: Sets a global variable to a specific value and performs multiple conditional checks and routine calls based on its state.
- `concatenate_registers`: Returns a value formed by concatenating the contents of two registers.
- `execute_jump_table_entry`: This routine performs a single instruction that transfers control to a fixed address via a jump table entry.
- `execute_recursive_calls_2`: Calls another routine twice and stores the result in a global variable.
- `call_subroutine_twice_2`: Calls another routine two consecutive times before returning.
- `wait_and_store_longword`: Waits for a global variable to reach zero, then writes a longword composed of a fixed value and a register value into a table.
- `double_and_xor_global`: Multiplies a global variable by two and applies a bitwise XOR with a constant if the overflow flag is not set during the multiplication.
- `insert_node_into_sorted_list`: Inserts a new node into a linked list by traversing the list until a value comparison determines the correct position.
- `insert_node_into_sorted_list_2`: Traverses a linked list to find the correct insertion point for a new node based on a comparison with a target address.
- `validate_and_initialize_pointer`: Checks a value against specific thresholds through multiple conditional branches.
- `find_maximum_and_clear`: Compares a value against a register through multiple calls and, if it exceeds a threshold, clears a memory location and stores a calculated value.
- `clear_memory_locations`: Sets four consecutive memory locations to zero and modifies a global variable.
- `tiny_ori_2`: Small routine of 1 instruction that merges 112 into d5.
- `tiny_ori_3`: Small routine of 1 instruction that merges 200 into d5.
- `tiny_ori_4`: Small routine of 1 instruction that merges 104 into d5.
- `tiny_ori_5`: Small routine of 1 instruction that merges 240 into d1.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `corrupt_stack_data`: Performs bitwise operations and memory writes on values derived from the stack before encountering bad instruction data.
- `initialize_memory_and_tables`: Clears a block of memory and populates a table with incrementing longword values.
- `insert_into_sorted_list_2`: Iterates through a linked list to find the correct insertion point based on a comparison, then performs a large block memory copy to shift or place data into a new node structure.
- `search_table_for_value`: Iterates through a table in steps of seven words to check if a specific value matches any entry.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `traverse_and_copy_list`: Traverses a list to find a minimum value in a specific field and then copies data from one memory location to another.
- `insert_into_linked_list`: Inserts a value into a sorted linked list structure while managing memory blocks and potential splitting.
- `update_linked_list_node`: Traverses a linked list to find a specific memory range and updates the pointers and size values of the nodes.
- `sum_linked_list_values`: Traverses a linked list starting from a global variable and returns the sum of the second elements in each node.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_concatenated_values`: Returns a concatenated value from registers based on the zero flag status.
- `check_byte_and_branch`: Checks if a byte at a specific offset in a structure is zero and executes further routines accordingly.
- `call_update_indexed_table_entry`: Calls a routine that calculates an offset using a signed integer and a constant to store a register value into a table.
- `update_indexed_table_entry`: Calculates an offset using a signed integer and a constant, then stores a value from a register into a specific table at that location.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `check_range_and_call`: Checks if a value is within a specific range and calls another routine if it is.
- `increment_table_byte`: Checks a byte in a table against zero and, if non-zero, enters a loop that increments that byte until it reaches a specific value.
- `increment_byte_loop`: Increments a byte in a table repeatedly until it reaches a specific value, then resets it to zero.
- `call_reset_and_toggle_bit`: Calls a routine that resets a byte in a table and performs bitwise operations to clear and set a specific bit in a memory offset.
- `return_constant_value`: Returns a 64-bit value consisting of a constant and the contents of the stack.
- `do_nothing_routine`: The routine is empty and performs no operations.
- `store_split_word`: Stores a 32-bit value or parts of it into a memory block using a pointer.
- `process_buffer_stream`: Iterates through a memory buffer looking for a specific sentinel value and then copies or transforms data into a destination buffer.
- `validate_index_range`: Checks if a signed integer falls within a specific range and verifies if a corresponding byte in a table is non-zero.
- `initialize_table_values_2`: Copies a value from one memory location to another and writes several constant values to a table or memory-mapped registers.
- `write_fixed_values_2`: Writes two constant 16-bit values to specific offsets from a pointer after checking a global variable condition in a loop.
- `write_fixed_values`: Writes two constant 16-bit values to specific offsets from a pointer.
- `modify_device_register`: Modifies a byte at a specific offset using bitwise operations and a value from another function.
- `manage_dynamic_buffer_pool`: Iterates through a memory structure to locate specific markers and manages a pool of data by copying values into a linked list or buffer.
- `calculate_masked_bitwise_result`: Returns a value calculated by masking and bit-shifting a pointer-referenced value.
- `update_control_state_2`: Clears a global variable and writes a 16-bit value from a register to two memory locations via a pointer.
- `clear_register_bit`: Clears a bit in a memory location and then sets a different bit in the same location.
- `write_word_and_wait`: Clears a global variable, writes the low and high bytes of a register to two memory locations via a pointer, and loops until the global variable is no longer zero.
- `update_control_byte_bits`: Keeps the top three bits of one byte at a fixed offset from a base pointer while setting bit three, and writes the constant 130 into another byte at a fixed offset.
- `update_control_state`: Clears a global variable and writes a 16-bit value from a register to two memory locations via a pointer.
- `store_split_byte`: Stores a byte from a register into a table offset, splitting the byte into a low byte and a high byte stored at different offsets.
- `search_table_loop`: Compares a value from a table against a target and enters a loop that continues until a match is found.
- `search_table_for_value_2`: Iterates through a table, comparing entries against a target value and performing conditional bitwise operations based on a carry flag.
- `sync_table_entry_with_condition`: Compares a value from a table with a value at an offset from a global pointer, then conditionally performs a bitwise operation on a byte at a different offset.
- `modify_byte_with_mask`: Clears a bit at a certain position and sets certain bits in a byte located at an offset from a pointer.
- `modify_byte_with_mask_alt`: Clears a bit at a certain position and sets different bits in a byte located at an offset from a pointer.
- `update_memory_with_loop`: Checks a condition in a table, updates a byte in memory, and executes a loop that calls another routine while checking a bit in a pointer.
- `reset_and_toggle_bit`: Resets a byte in a table to zero, then performs a sequence of bitwise operations to clear and then set a specific bit in a memory offset, followed by a call to another routine.
- `compare_rotated_longwords`: Compares two longwords by checking their high bytes and performing a bitwise rotation on both values if they match, repeating this up to three times.
- `clear_memory_byte`: Stores a zero byte at a fixed offset from a pointer held in a register.
- `get_table_entry`: Returns a word from a table at an offset calculated from a register.
- `set_table_entry`: Writes a word from a register into a table at an offset calculated from a register.
- `increment_table_entry`: Increments a word in a table at an offset calculated from a register.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `verify_paired_routine_results`: Calls two unnamed routines and compares their results; when they match it calls both again and compares once more.
- `xor_fold_longwords_masked`: XORs together a run of longwords read through a pointer, with the run length taken from a data register divided by four, then returns that fold ANDed with a data register mask.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `set_control_bits_return_input`: Sets a fixed set of bits in a control byte at a fixed offset from a base pointer and returns its word input unchanged.
- `set_global_to_max`: Stores the value zero-ff into a global variable.
- `clear_global_variable`: Stores zero into a global variable.
- `write_longword_to_table`: Waits for a global variable to become zero, then writes a longword consisting of a fixed value and a register value into a table.
- `clamp_global_variable_range`: Clamps a global variable between zero and a maximum value of three hundred nineteen.
- `manipulate_global_and_hardware`: Sets a global variable, performs bitwise operations on it, and conditionally modifies a memory location and hardware registers including CIAA_PRA and CIAA_ICR.
- `process_joystick_bits`: Reads bits from a global variable and performs bitwise operations to update another global variable based on specific bit patterns.
- `update_button_state`: Reads several bits from a table and updates a global variable using bitwise OR operations based on those bits.
- `process_input_buffer`: Manipulates a pointer and a global variable to update or increment values within a data structure based on bit flags.
- `manipulate_bitmask`: Performs bitwise operations on a global variable and conditionally calls another routine.
- `update_global_state_recursively`: Modifies a global variable through bitwise operations and conditional increments, then performs multiple recursive calls to itself based on the resulting value.
- `set_memory_constants`: Clears a bit in a global variable and writes a sequence of constant values to a series of memory locations.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `clear_memory_blocks`: Sets four consecutive memory locations to zero and then modifies a global variable.
- `decrement_and_set_registers`: Decrements a global variable with a lower bound check and sets a bit in a memory location and several other memory addresses to specific values.
- `update_register_and_table`: Clears certain bits in a hardware register, clears several memory locations, and updates a global variable or loads data from a table based on a pointer offset.
- `return_immediately`: This routine is empty and returns to the caller without performing any operations.
- `process_conditional_logic_3`: Performs a series of conditional checks and recursive calls while modifying a global variable.
- `toggle_global_bit`: Checks the lowest bit of a global variable and, if set, clears it and sets the highest bit of the word.
- `manipulate_global_sequence`: Performs complex conditional logic involving multiple memory reads and bitwise shifts to update a global variable.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `assign_register_to_global`: If a global variable is zero, it assigns a value from a register to that global variable.
- `initialize_audio_buffers`: Clears memory buffers and certain registers, then populates a table with incrementing values and calculates a maximum value from a memory range.
- `infinite_loop`: This routine executes an infinite loop.
- `reset_audio_parameters`: Clears several memory locations and a global variable, then performs conditional logic and updates a global variable.
- `execute_conditional_subroutines`: Calls several subroutines in sequence and conditionally executes another routine based on the value of a global variable.
- `update_audio_sample_parameters`: Updates audio hardware registers and internal structures by fetching sample data and timing information from a lookup table.
- `process_parameter_control_block`: Interprets a control block to perform arithmetic operations, clamping, and table-based lookups on a target value.
- `adjust_coordinate_value`: Subtracts a value from a word at a pointer offset and clamps the result within a specific range before storing it in another location.
- `search_table_and_update`: Searches a table for a value based on an offset and updates a structure with the found value and a status flag.
- `update_scaled_value_2`: Adjusts a value based on a signed byte, clamps it to a limit, and searches a table to find a matching entry.
- `calculate_scaled_offset`: Calculates a value by scaling a byte from a table or performing arithmetic on a byte, then adds this to a value at a pointer offset.
- `calculate_nibble_weighted_sum`: Extracts the low and high nibbles from a byte at a pointer offset, calculates a weighted sum by multiplying the high nibble by sixteen, and stores the result in a global variable if the sum is less than sixty-four.
- `process_joystick_input_data`: Processes input values from a structure by applying bitwise masks and clamping results within specific ranges.
- `decrement_accumulator`: Decrements a value in a structure by a scaled amount and increments another value if a threshold is met.
- `process_scaled_decrement`: Subtracts a scaled byte from a memory location and updates a counter.
- `apply_scaled_offset`: Updates a memory offset using a value from a byte and modifies a secondary integer value.
- `adjust_memory_offset`: Modifies a value at a specific offset using a byte-scaled subtraction and updates a long integer.
- `scale_and_subtract`: Performs a subtraction on a memory location using a scaled byte and updates a global counter.
- `decrement_and_reset`: Decrements a byte from a memory offset and then writes a constant value to a global variable.
- `clamp_and_store_value`: Clamps a byte value to a maximum of sixty-four and stores it in a specific offset of a structure, then stores the difference between that value and a global variable into another structure.
- `calculate_weighted_nibble_sum`: Extracts the low and high nibbles from a byte at a pointer offset, calculates a weighted sum, and stores the result in a global variable.
- `process_joystick_input_2`: Parses a byte from a memory pointer to update various state variables, including a global variable and several offsets within a structure.
- `update_global_byte`: Checks if a byte at a specific offset is non-zero and, if so, copies it to a global variable.
- `modify_byte_bits`: Modifies a byte by setting its second bit based on a bit from a pointer offset and preserving other bits.
- `adjust_coordinate_with_clamp`: Subtracts a byte value from a word at a pointer offset and clamps the result to a minimum value before storing the lower twelve bits into a new location.
- `mask_and_merge_byte`: The routine clears the low nibble of a byte at an offset from an address in a register and then merges the low nibble of another byte at an offset from the same address into it.
- `mask_low_nibble_to_offset`: The routine stores the low nibble of a byte from an address derived from an uninitialized pointer into a byte at a fixed offset from that pointer.
- `update_pointer_based_on_flag`: The routine checks a high bit in a structure field; if set, it clears the field, adjusts a pointer based on a table lookup and offset, and negates the byte at the new pointer location.
- `compare_and_update_limit`: Compares a value from a table with a global variable and updates the global variable if the condition is met.
- `update_dma_descriptor_table`: Retrieves a pointer from a memory location and uses it to write several data words and a longword to a structured table.
- `write_dma_control_block`: The routine writes values to several memory locations and hardware registers to configure a DMA transfer.
- `write_dma_control_block_indexed`: The routine selects a pointer from a table using an index and writes values to memory and hardware registers to configure a DMA transfer.
- `decrement_and_check_global`: Decrements a global variable and clears it if it drops below zero, then checks a bitmask in a global variable to clear a byte in memory or set a value.
- `clear_masked_list_elements`: Iterates through a series of longwords and clears specific elements if they match a constant value and a bitmask condition.
- `search_table_for_value_3`: Iterates through a table of integers to find a match for a given value.
- `advance_to_list_tail`: Follows a chain of pointers until it reaches a node whose first long is null, then returns with no result.
- `update_pointer_structure`: Checks if a calculated address falls within a specific range and then stores a value and a pointer into a memory location.
- `insert_into_sorted_list`: Traverses a linked list to find an insertion point based on a value, then updates pointers to insert a new node.
- `insert_and_update_list_size`: Traverses a linked list to insert a node and updates a size counter if certain pointer conditions or offsets match.
- `sum_linked_list_values_2`: The routine traverses a linked list twice to calculate two separate sums of values stored within the list nodes.
- `execute_dual_instruction_sequence`: Executes four instructions and calls another routine that performs exactly two instructions before returning.
- `return_concatenated_registers`: Returns a value formed by concatenating two registers.
- `perform_two_instruction_sequence`: This routine executes exactly two instructions before returning.
- `reconstruct_bitstream_from_word`: Extracts bits from a 32-bit word and reconstructs them into a continuous bitstream by shifting and concatenating values into a destination buffer.
- `process_bit_stream`: Performs bitwise operations and shifts on a 32-bit word while moving data through a pointer.
- `expand_bit_data`: Manipulates a bit stream using shifts and conditional logic to write bytes to a memory location.
- `transform_bit_sequence`: Processes a 64-bit value by shifting and concatenating bits into a byte-oriented structure.
- `loop_bit_extraction`: Extracts bits from a word and writes them to a destination pointer using a nested loop structure.
- `rotate_right_extend_long_2`: Reads a 32-bit word from memory, shifts it right by one bit, and places the carry flag into the most significant bit while forcing the highest bit to one.
- `extract_bits_to_stream`: Extracts individual bits from a 32-bit word and reconstructs them into a continuous bitstream by shifting and concatenating with values from a memory pointer.
- `shift_bit_field`: Iteratively shifts bits from a 32-bit word into a byte while updating a carry-like flag.
- `reconstruct_bitstream_2`: Extracts bits from a source word and reconstructs them into a continuous bitstream within a destination buffer.
- `reconstruct_bitstream`: Extracts bits from a source and reconstructs them into a continuous bitstream stored in a destination buffer.
- `decrement_global_counter`: Sets a global variable to a fixed value and then executes a loop that decrements that variable until it reaches a certain value.
- `copy_memory_block`: The routine copies a sequence of words from a source pointer to two different destination pointers.
- `set_blt_registers_and_wait`: The routine writes several values to certain memory-mapped registers and waits for a bit in a global variable to clear.
- `update_bcd_value`: Performs a series of additions and BCD adjustments on a global variable and potentially resets it to a fixed value.
- `clear_status_bits`: Iteratively checks and clears specific bits within a global variable.
- `write_digit_tiles`: Clears several global memory locations and then copies tiles from a table into them based on the quotient and remainder of a global variable divided by ten.
- `copy_pattern_tables`: Performs bitwise rotations on a global variable and uses the resulting nibbles to copy data from a table into multiple locations in another table.
- `write_nibble_tiles`: Rotates a global variable and uses the resulting nibbles to copy data from a table into a destination table.
- `initialize_blt_hardware`: Resets a global variable and prepares hardware bitplane transfer parameters.
- `reset_global_variable`: Sets a global variable to zero and then to a specific byte value.
- `reset_global_variable_2`: Sets a global variable to zero twice and then assigns it a value of two hundred and fifty-five.
- `update_state_table`: Modifies a global variable and uses it to index into a table, performing iterative increments and conditional assignments based on the current state.
- `configure_blt_parameters_2`: Calculates and stores multiple values into a table of memory locations for hardware bitplane transfer operations.
- `setup_blt_parameters`: Calculates several values and writes them to memory locations used for hardware bitplane transfer operations, then waits for a specific bit in a global variable to clear.
- `split_integer_values`: Divides a value by one hundred and returns the quotient and remainder combined into a single 32-bit word.
- `modify_global_value`: Multiplies a global variable by two and conditionally applies a bitwise XOR if a certain flag is not set.
- `copy_relocation_table`: Copies a sequence of words from a source table to a destination address and then populates a secondary table with incrementing pointer values.
- `clear_global_variable_2`: Sets a global variable to zero twice before calling another routine.
- `setup_blt_parameters_2`: Sets several global memory locations or hardware registers to specific constants and calculates new pointer or attribute values based on input registers.
- `check_global_flag`: Checks a global variable and enters an infinite loop if certain conditions are met.
- `write_fixed_values_3`: Writes specific constant values to memory locations and enters an infinite loop.
- `execute_recursive_calls_5`: Performs several recursive calls and then enters an infinite loop after setting two memory locations.
- `initialize_and_process_lookup_table`: Clears a memory block and parses a structured data table to extract values based on indexed offsets.
- `process_sorted_list_data`: Iterates through a list of values to find insertion points, performs memory shifts, and updates a table with calculated offsets.
- `copy_structured_data`: Traverses a list of data to extract specific bytes and copies them into a destination table at a fixed memory location.
- `process_pointer_table`: Iterates through a table of pointers, clearing each non-zero entry and calling another routine for each.
- `iterate_long_table`: Iterates through a table of longwords, clearing non-zero entries and calling another routine for each.
- `process_short_table`: Iterates through a table of shorts, clearing non-zero entries and calling another routine for each.
- `return_from_subroutine_3`: The routine performs no operations and immediately returns to its caller.
- `execute_recursive_calls`: Calls another routine twice and returns the concatenated result of the registers used in the calls.
- `call_subroutine_twice`: This routine calls another routine two consecutive times before returning.
- `manage_linked_lists`: Performs several operations including clearing a global variable, executing recursive calls, and entering multiple infinite loops.
- `execute_recursive_calls_4`: Executes a sequence of recursive calls and modifies a global variable.
- `execute_infinite_loop_5`: Performs several routine calls and then enters an infinite loop.
- `clear_global_array`: Iterates through a small array and sets its elements to zero.
- `shift_and_remove_element`: Checks a table entry, clears it, and shifts subsequent elements in a block of memory to fill the gap.
- `shift_and_insert_elements`: Calculates a count from a word, calls another routine, and then shifts a block of memory forward to make room for a new element.
- `fill_word_table`: Calls another routine and then fills a table with the returned word value based on a global variable count.
- `find_first_non_negative`: Searches through a table of 32-bit integers for the first entry that does not equal negative one and returns the index of the block containing it.
- `copy_word_blocks`: Copies two separate blocks of 16-bit words from one memory location to another.
- `scan_table_for_mask`: Iterates through a table of 16-bit values to find an entry where the lower nine bits are all set, updating a global variable with a calculated offset.
- `clear_memory_block_2`: Fills a block of 32-bit words starting at a specific memory address with zeros.
- `shuffle_data_blocks`: Performs a complex series of nested loops to move and rearrange data between a source table and a destination buffer.
- `update_global_state`: Sets a global variable to a specific value, calls another routine, and then resets the global variable to zero.
- `search_and_increment_table`: Searches a memory block for a specific byte value and performs arithmetic operations on a value at a pointer offset.
- `loop_on_sign_bit`: Repeatedly executes another routine as long as a global variable remains negative.
- `update_global_and_render_string`: Increments a global variable and performs a conditional bitwise update to a memory location calculated from a table.
- `update_bcd_score_accumulator`: Increments a global variable and, if a specific condition is met, performs multi-byte BCD subtraction between a global value and a returned value from another routine.
- `traverse_nested_data_structure`: Recursively descends through a hierarchical data structure by decrementing a global counter and accessing elements from a table based on calculated offsets.
- `execute_multiple_calls`: Calls another routine four times.
- `convert_nibbles_to_ascii`: Takes a byte, splits it into two nibbles, and stores the resulting ASCII character representations into a destination string.
- `update_multi_byte_accumulator`: Increments a multi-byte value stored across several memory locations by adding a value from a register.
- `setup_blt_graphics`: Configures hardware registers for a Blitter operation to draw characters from a string at calculated screen coordinates.
- `generate_bit_pattern_table`: Populates a memory buffer with a sequence of 256 words where each word contains a bit-shifted pattern derived from an incrementing counter and a toggling boolean flag.
- `copy_string_bytes`: Copies a null-terminated string from a source address to a destination address byte by byte.
- `encode_decimal_digits`: Takes an 8-bit value and encodes its decimal digits into a 32-bit packed format.
- `reset_global_if_nonzero`: Checks if a global variable is non-zero, and if so, resets it to zero and executes other routines.
- `loop_and_check_byte`: Enters a loop that repeatedly calls another routine and checks a byte at a specific offset within a global structure, breaking or returning based on the value found.
- `initialize_data_tables_2`: Sets a global variable to zero, performs several routine calls, assigns constant values to a global variable, and enters an infinite loop.
- `manipulate_palette_and_status`: Updates color palette values and modifies a global status variable through a series of conditional bitwise operations and recursive calls.
- `recursively_update_global`: Recursively calls another routine and updates a global variable with the returned values if a global variable does not equal a specific constant.
- `check_global_value`: Checks a global variable against a constant and, if not equal, performs a series of calls and modifies a value through a pointer.
- `update_status_flags_2`: Performs bitwise checks and modifications on a global variable to update its state based on specific bit patterns.
- `initialize_memory_tables`: Clears a block of memory, fills several longword table entries with negative one, and executes multiple conditional routine calls.
- `update_animation_frame`: Increments or decrements a global index to select a memory offset from a table and copies a block of data to a destination buffer.
- `update_global_pointer`: Updates a global variable by reading a value from a memory location determined by the current value of that variable and then conditionally resets it to a fixed value based on a byte check.
- `decrement_and_copy_table`: Decrements a global variable and, if it remains above a threshold, copies a series of words from a calculated memory offset into a set of global variables.
- `copy_word_block_2`: Copies a series of words from a source buffer to a destination buffer using a loop.
- `unpack_nibbles`: Iterates through a source buffer and rearranges nibbles from each word into a new destination buffer by shifting and masking bits.
- `transform_nibbles`: Processes two source buffers to calculate new nibble values based on differences and shifts, storing the results in a destination buffer.
- `process_input_loop_2`: Stores the result of a routine into three consecutive memory locations and enters a nested loop that monitors a global variable.
- `setup_blt_registers`: Writes specific constant values to hardware registers associated with the Blitter to prepare for a graphics operation.
- `perform_complex_operations`: Executes a long sequence of various subroutines and constant assignments before entering an infinite loop.
- `initialize_system_state_and_loop`: Performs a sequence of global variable assignments and multiple calls to another routine before entering an infinite loop.
- `update_cop_status_register`: Modifies a global variable through arithmetic and conditional sign inversion, then updates a cop register.
- `update_sorted_table_and_shift_elements`: Iterates through a table to find an insertion point based on a global variable, then shifts subsequent elements in a secondary table to accommodate a new entry.
- `update_text_display`: Manipulates a global variable to navigate a character table and updates memory with specific characters based on the resulting index.
- `initialize_memory_blocks_2`: Sets several memory locations and global variables to specific constant values and clears a range of memory.
- `execute_scaled_values`: Calls another routine within a loop that iterates nine times.
- `generate_scaled_values`: Calculates values based on input bytes and stores them in a destination table, followed by a second loop that performs bitwise rotations and stores scaled results.
- `distribute_data_to_tables`: Copies data from a source table into three separate destination tables using nested loops and specific memory offsets.
- `configure_blta_operation`: Sets up Blitter Address mode registers using values from a table and configures Blitter control and size registers before entering a wait loop.
- `call_recursive_routine`: Calls another routine and then returns.
- `traverse_null_terminated_string`: Iterates through a pointer until a null byte is encountered, calling another routine during each step.
- `rotate_and_loop`: Performs bitwise rotations on a value and executes a loop based on the result of those operations.
- `copy_data_block`: Copies a block of eight bytes from a table located at an offset determined by an input value to a destination address.
- `copy_pattern_data`: Iterates through a list of source pointers to extract bytes and write them into a destination buffer using a calculated stride.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `setup_blt_transfer`: Configures hardware registers for a bitplane transfer and waits for a specific bit in a global variable to clear.
- `configure_blt_parameters`: Sets hardware registers for a bitplane operation and waits for a synchronization flag in a global variable.
- `initialize_blt_state`: Writes values to hardware registers and waits for a global flag to clear before calculating a destination address.
- `prepare_blt_transfer`: Sets hardware registers for a bitplane transfer and waits for a global status bit to clear.
- `execute_blt_blit`: Configures bitplane registers and calculates a destination address based on a product of two input values.
- `tiny_addi`: Small routine of 1 instruction that adds 64 to d0.
- `lookup_signed_index`: Converts a signed byte into an absolute index, performs a table lookup, and restores the sign.
- `lookup_scaled_ratio`: Compares two input words and returns a value from a global table based on their ratio or sign.
- `lookup_scaled_value`: Performs a scaled index lookup from a global table or returns modified values based on comparisons of two input words.
- `lookup_scaled_index`: Compares two values and either returns a space, performs a calculation, or retrieves a value from a global table using a computed index.
- `calculate_table_index`: Computes an index by dividing a shifted value by a divisor and performs a table lookup.
- `update_sprite_position_and_attributes`: Calculates new coordinate offsets and updates a structure containing sprite or object data, including bitwise manipulation of attribute bytes and writing values to a memory-mapped grid.
- `update_word_offset_fifteen`: Compares a word at an offset within a structure to a global variable and updates the global variable based on the comparison.
- `sync_global_state_with_memory`: Compares a global variable against a value stored in memory at an offset from a pointer.
- `fill_memory_grid`: Calculates memory offsets using a pointer and a scaling factor to write values into a two-dimensional grid structure.
- `populate_memory_via_nested_loops`: Calculates a base address using a global variable and bit-shifted values, then executes nested loops to write incrementing values into memory.
- `update_blt_graphics_parameters`: Updates hardware Blitter registers and modifies a data structure containing coordinate and offset values.
- `update_object_metadata_and_coordinates`: Modifies an object's attribute byte and timer-related values before calculating and storing a series of three offset coordinate pairs into a structure.
- `copy_memory_block_3`: Performs a block memory copy operation using a nested loop structure to transfer data from a source table to a destination address.
- `reset_blt_flag`: Checks a byte at an offset, and if non-zero, clears it and configures hardware registers for a bitplane transfer.
- `update_tile_index_table`: Calculates a destination address and populates a table with values derived from a source memory buffer through nested loops.
- `copy_attribute_table`: Uses a global variable and a table offset to perform a nested loop copy of data into a destination memory area.
- `copy_table_to_memory`: Calculates a source address based on a global variable and input registers, then uses nested loops to copy data from the source to a destination via a pointer.
- `update_global_word`: Updates a global variable by assigning it the result of another routine.
- `update_global_value`: Modifies a global variable based on its current value through a series of arithmetic comparisons and conditional increments or decrements.
- `iterate_table_entries_2`: Iterates through a table of values using a global variable as a base index and performs conditional checks on each entry.
- `process_global_pointer_table`: Checks if a global variable is within a specific range and then iterates through a table of pointers.
- `execute_infinite_loop_2`: Enters an infinite loop that prevents further execution.
- `configure_blt_hardware`: Updates hardware registers and a table of values if a specific field in a structure has changed.
- `modify_table_values`: Iterates through a table of words, performing bitwise manipulations to clamp and combine specific bit fields within each entry.
- `copy_data_to_memory`: Copies a short value from a global variable to two different memory locations using a loop.
- `calculate_and_discard_offset`: Calculates a value based on the distance between two code locations, then enters a loop that decrements this value until it reaches a fixed state before returning.
- `initialize_memory_block`: Sets specific values in a memory block and updates a global flag.
- `initialize_control_structure`: Sets specific fields within a data structure to zero or a fixed value and updates two global flags via bitwise OR operations.
- `initialize_structure_fields`: Sets specific fields in a memory block to fixed values and updates two global flags via bitwise OR.
- `initialize_pointer_structure`: Sets specific values in a memory block pointed to by a global variable and updates a flag.
- `initialize_memory_block_via_pointer`: Writes specific values to a memory block pointed to by a value within a passed structure.
- `update_scrolling_buffer`: Calculates new offsets and increments values within a data structure, then performs a loop to write zeroed or modified bytes into a memory buffer.
- `execute_nested_calls`: Calls another routine twice and returns the concatenated results of the data held in the data registers.
- `update_scrolling_structure`: Updates a data structure by calculating offsets, modifying sprite-like attributes, and performing memory writes.
- `process_flag_callbacks`: Checks three global flags and, if set, clears them and calls another routine before storing modified integer pairs into a buffer.
- `split_and_store_integer`: Reads a 32-bit integer from a pointer and splits it into two 16-bit halves, storing them into a target buffer along with a modified version of the original value.
- `store_modified_integer_pair`: Stores a 32-bit integer and a modified version of that integer into a target buffer as two pairs of 16-bit words.
- `process_data_blocks`: Either configures hardware registers or copies data from one memory location to another in a loop depending on a pointer value.
- `setup_blt_transfer_2`: Configures Blitter registers and copies data from a source pointer to a destination table using a loop.
- `rotate_buffer_elements`: Performs a cyclic shift of four 32-bit words within a block of memory, repeating the operation three times across a sequence of memory locations.
- `update_scrolling_buffer_parameters`: Calculates new scroll offsets and updates a data structure containing coordinate and timing information.
- `configure_blt_and_copy_data`: Sets several Blitter control and mode registers before performing a loop that copies data from a source pointer into two separate destination buffers in a de-interleaved or split fashion.
- `update_color_palette_table`: Updates a color palette table by iterating through a series of values and storing them into memory.
- `copy_data_block_2`: Copies a series of 32-bit words from a source table to two different destination locations.
- `increment_and_check_table`: Increments a global variable and checks if three consecutive 32-bit entries in a table equal negative one.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_table_values`: Sets a global variable to zero and fills three 32-bit table entries with negative one.
- `conditionally_call_twice`: Calls another routine twice if a global variable is non-zero.
- `conditionally_call_once`: Calls another routine once if a global variable is non-zero.
- `check_global_and_call`: Calls another routine if a global variable is non-zero.
- `iterate_table_entries`: Iterates through a table of pointers, performing conditional calls to another routine or applying bitmasks and clearing ranges in two tables based on the data found.
- `apply_bitmask_to_range`: Calculates an offset and applies a bitwise mask to a range of words in two tables based on bit-shifting logic.
- `clear_table_range`: Clears a specific range of 32-bit words in two tables if certain boundary conditions are met.
- `populate_data_structure`: Copies values from a source pointer and several registers into a destination memory block.
- `populate_structured_data_block`: Initializes a memory block by writing several constant values and copying data from two source locations.
- `populate_data_structure_2`: Copies values from a source pointer and several registers into a structured memory block.
- `populate_data_structure_3`: Copies values from a source pointer and several registers into a destination memory block.
- `populate_data_structure_4`: Copies values from several registers and a memory pointer into a structured block of memory.
- `populate_data_structure_5`: Initializes a memory block by writing several constant values and copying data from a source pointer.
- `populate_data_structure_6`: Initializes a memory block by writing several constant short and long values and copying data from two source locations into the structure.
- `populate_data_structure_7`: Populates a memory block pointed to by a register with a mix of hardcoded constant values and data copied from another memory location.
- `populate_data_structure_8`: Initializes a memory block by writing several constant values and copying data from two different sources into specific offsets.
- `write_structured_data_set_one`: Writes specific 16-bit and 32-bit values into a memory block pointed to by a register, including data copied from another memory location.
- `write_structured_data_set_two`: Populates a memory block with a series of constant values and copies data from a source pointer.
- `write_structured_data_set_three`: Stores several constant words and longwords into a structured memory area using a pointer.
- `write_structured_data_set_four`: Updates a memory block by writing specific constants and transferring data from another memory address.
- `write_structured_data_set_five`: Writes a sequence of constant values into a memory structure and copies values from a source pointer.
- `write_structured_data`: Writes a series of specific 16-bit and 32-bit values into a structured memory block.
- `calculate_table_parameters`: Computes and stores several derived values, including products and offsets, into a destination structure.
- `update_structure_data`: Calculates several derived values based on two input words and stores them into a table starting at a pointer.
- `write_table_fields`: Writes two input words and two values from other registers into a table.
- `calculate_and_store_table`: Stores two input words at the start of a table and populates several fields using bitwise operations and arithmetic on register values.
- `calculate_relative_offsets`: Calculates differences between coordinate pairs in a structure and iterates through a list of pointers to perform further operations.
- `update_entity_position_and_sprite`: Updates an entity's coordinate data and calculates new sprite attribute values based on movement flags and bounding constraints.
- `update_sprite_animation_frames`: Calculates and writes multiple sets of coordinate and attribute data into a sprite structure table to facilitate frame-based animation.
- `update_sprite_attribute_table`: Calculates and writes new coordinate and attribute data into a sprite structure within a table.
- `blit_recursive_table_elements`: Configures Blitter hardware registers and then iterates through a table of data, recursively calling itself for each valid entry found in the list.
- `loop_and_wait_for_dma`: Executes a routine repeatedly within a loop, writing a specific value to a memory location and then polling a global variable bit until it is cleared.
- `update_entity_animation_state`: Iterates through entity data structures to update animation frame indices and sprite attributes based on internal timers and state flags.
- `update_entity_animation_frames`: Calculates new coordinate and attribute values for a sprite data structure based on input flags and timer values.
- `update_sprite_attribute_data`: Calculates and writes new coordinate and attribute values into a sprite data structure.
- `setup_blt_hardware`: Configures Blitter registers and waits for a DMA controller bit to clear.
- `write_dmaconr_and_wait`: Writes a value to the DMA control register and enters a loop that waits for a specific bit in a global variable to be cleared.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or functional side effect before returning.
- `process_data_table_with_blitter`: Calculates a destination address based on structure values and iterates through a table to perform operations using blitter registers.
- `write_string_to_memory`: Iterates through a null-terminated string and writes segments of data to a memory location based on calculated offsets.
- `process_input_loop`: Initializes Blitter registers and iterates through a table of pointers to call another routine if the value at each pointer is non-zero.
- `process_joystick_input_buffer`: Iterates through a four-byte input buffer to detect non-zero values, then calculates new coordinates or offsets using a lookup table and updates a target structure.
- `update_table_values_3`: Selects a memory address based on a global variable and performs a series of arithmetic operations and bitwise XORs on a data block.
- `decrement_global_counter_2`: Decrements a global variable and calls another routine if the variable becomes negative.
- `update_table_counters_2`: Decrements a global counter and, upon reaching a threshold, performs XOR operations on specific bytes and updates two tables using a loop.
- `update_table_counters_alt`: Decrements a global counter and, upon reaching a threshold, performs XOR operations on different bytes and updates two tables using a loop.
- `update_table_counters`: Decrements a global counter and, when it reaches a specific threshold, performs XOR operations on bytes in a table and updates subsequent word elements using arithmetic.
- `update_table_values`: XORs two bytes at specific offsets in a table and then performs a series of arithmetic updates on subsequent word elements within that table.
- `process_data_tables`: Performs nested loops to transform values from a source table into a destination table based on bitwise operations and a secondary table lookup.
- `update_global_value_2`: Performs two successive multiply-and-subtract operations on a global variable.
- `scale_data_table`: Performs nested loops to multiply values from a source byte array by a constant and add an offset, storing the results in a destination table.
- `update_global_value_alt`: Performs two successive multiply-and-subtract operations on a global variable using different constant offsets.
- `copy_and_invert_data`: Copies data from a source table to a destination and then performs a bitwise inversion on a portion of that data within a loop.
- `copy_memory_blocks_3`: Executes a nested loop that copies data from a source memory location to a destination, including zeroing out specific offsets.
- `set_global_byte`: Calls another routine and then sets a global variable to a specific byte value.
- `copy_memory_block_2`: Copies a sequence of longwords from a source location to a destination location using a loop.
- `clear_memory_block_6`: The routine clears a specific block of memory by filling it with zeros.
- `clear_memory_block_3`: Fills a block of memory with zeros based on a size provided in a register.
- `clear_memory_block`: Clears a block of memory starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `reset_status_flags`: Clears a global variable and performs several conditional checks to reset or update various memory locations and status flags.
- `update_status_flags_4`: Checks specific byte and word values within two memory structures and modifies a flag or clears values based on the results.
- `check_value_range`: Compares two sets of values stored in memory against specific bounds and returns zero if they fall within those ranges.
- `check_value_range_alt`: Compares two sets of values stored in memory against different bounds and returns zero if they fall within those ranges.
- `calculate_clamped_offset`: Evaluates conditions within two memory structures to perform arithmetic updates on a third structure, including conditional zeroing of elements.
- `increment_decrement_clamped_value`: Calculates the difference between a register value and a memory location, then updates the memory location by adding or subtracting that difference, capped at a maximum magnitude of seven.
- `calculate_complex_value`: Performs a series of bitwise operations and arithmetic calculations involving global variables and a function call to produce a result.
- `update_blitter_rectangles`: Calculates dimensions based on values from a memory structure and updates hardware registers to perform multiple Blitter operations.
- `update_sprite_position_and_blitter`: Calculates new coordinates for a sprite or object using a weighted average loop and updates the hardware blitter registers to perform screen drawing operations.
- `configure_dma_transfer`: Calculates offsets from a table and writes a control word and several parameters to a memory-mapped structure.
- `configure_dma_transfer_alt`: Calculates offsets from a table and writes a control word with a different bitmask to a memory-mapped structure.
- `update_position_and_bounds`: Updates a coordinate by adding two values and checks if a value exceeds a threshold to update a global variable.
- `check_status_conditions`: Checks if a value in a memory structure is negative and if a global variable is zero.
- `check_global_status`: Returns immediately if a global variable is zero.
- `manage_timer_state`: Decrements a global variable and updates another value based on a subtraction and bitwise mask.
- `tiny_ori_6`: Small routine of 1 instruction that merges 172 into d4.
- `update_display_parameters`: Modifies values in a table based on a counter and updates a pointer-based structure with specific bit patterns.
- `update_object_state`: Calculates a value based on a global counter and a pointer derived from a global variable, then updates a memory block with specific constants and flags.
- `reset_and_update_state_variables`: Initializes several global variables to zero or specific constant values, performs a series of recursive or repeated calls to another routine, and updates a counter and two other global variables based on the results of those calls.
- `execute_indirect_call_3`: Performs an indirect call to an address calculated by adding a register value to the current instruction pointer.
- `execute_empty_stub`: This routine calls an empty stub and returns immediately to the caller.
- `update_structure_coordinates`: Modifies coordinate-like values within a structure based on sign checks and increments a counter until a threshold is reached.
- `update_structure_parameters`: Modifies various fields in a structure based on bitwise checks of a value and clamps a global variable within a specific range.
- `dispatch_indirect_call`: Clears specific structure fields and performs an indirect call to a function pointer derived from a structure offset.
- `update_entity_timer`: Adjusts a structure offset by a constant, updates a counter, and resets several fields if a threshold is exceeded.
- `manage_state_counters_2`: Updates various counters and status flags within a structure based on comparisons between two 16-bit values.
- `execute_indirect_call_4`: Updates several fields in a structure and then performs an indirect call using an offset stored in a structure field.
- `increment_and_reset_counter`: Increments a 16-bit value at an offset within a structure and resets it to zero if it exceeds a specific threshold.
- `set_constant_values`: Sets a global variable and populates parts of a structure with specific large constant values.
- `modify_structure_fields`: Updates various fields within a structure, including coordinate offsets and counters, based on conditional checks of existing values.
- `add_structure_offset`: Adds the value from one structure offset to another structure offset.
- `clear_structure_fields_2`: Sets a word at a specific offset within a structure to zero and a byte at another offset to a maximum value.
- `update_object_state_2`: Modifies various fields within a structure, including coordinate offsets, counters, and conditional state flags based on threshold checks.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_variables`: Checks several memory locations and updates multiple global variables based on their current values.
- `initialize_state_structure`: Performs recursive calls and writes specific values to offsets within a structure pointed to by a register.
- `update_entity_state_and_animation`: Checks status flags within a structure to determine if an animation sequence or state transition should occur.
- `update_pointer_via_table_lookup`: Checks a flag in a structure and, if not set, performs a table lookup using an index derived from a byte value and a sign check.
- `invert_sign_value`: Checks specific bits in a structure and negates a signed value if certain conditions are met.
- `process_input_state`: Evaluates bitmasks within a structure to update status bytes and modify a value by a constant offset.
- `update_joystick_input_state`: Processes joystick or controller input flags to update a directional state and a timer-based counter.
- `update_entity_state`: Modifies various fields within a structure, including counters and status flags, based on conditional checks of existing values.
- `test_bit_conditions`: Calculates a value from two structure fields and uses it to call another routine, setting specific bits in a structure field based on the results.
- `update_status_flags_5`: Performs bit pattern searches using values derived from a structure and sets a specific bit in a byte at an offset if the searches return non-zero results.
- `process_structure_values`: Modifies several 16-bit values within a structure based on conditional checks of other members and performs recursive calls to another routine.
- `call_other_routine`: This routine calls another routine and then returns.
- `check_structure_flags`: Evaluates several conditions involving structure fields and returns a value from another routine if those conditions are met.
- `search_bit_pattern`: Searches through a bitstream table for a specific bit pattern and updates a counter in a structure based on the result.
- `check_bitmask_conditions`: Compares values from a table against a bitmask and updates structure fields or returns a status code based on the results.
- `count_bit_differences`: Compares two bitstream segments and stores the count of zero bits and one bits into a structure.
- `find_first_zero_bit`: Iterates through a bitstream to find the index of the first bit that is not set.
- `update_timer_value`: Increments a value in a structure if a condition is met, or resets other fields if a threshold is exceeded.
- `calculate_input_state`: Compares two sets of bitstream data and updates a state variable in a structure based on the difference in bit counts.
- `update_global_index`: Updates a global variable and a structure field by scanning a bitstream for a specific bit state.
- `update_blt_parameters`: Calculates and updates Blitter parameters by masking and adjusting coordinate values from a structure.
- `update_sprite_animation_state`: Updates a sprite or object state by selecting an animation frame or index based on various conditional flags and counters.
- `increment_timer_and_jump`: Increments a byte in a structure and uses the result to calculate a jump address for a global variable.
- `increment_timer_offset`: Increments a byte in a structure and calculates a different jump address for a global variable based on the new value.
- `reset_structure_fields`: Sets several fields within a structure to specific constant values.
- `add_structure_value`: Adds a value from one offset in a structure to another offset in the same structure.
- `reset_structure_fields_2`: Sets several fields in a structure to specific constants and increments a word value if a global variable comparison is met.
- `process_complex_data_tables`: Updates a global variable, clears a range of memory, and iterates through two tables to modify values and update pointers.
- `assign_constant_to_global`: Stores the value three into a global variable.
- `reset_global_variable_3`: Sets a global variable to zero and then calls itself.
- `read_byte_from_offset`: Reads a byte from a memory location calculated using a global variable and returns it.
- `reset_status_bytes`: Sets a byte at an offset of a global variable to a specific value, clears another byte at a different offset, and writes a value through a pointer stored in the global variable.
- `clear_status_bytes`: Sets a byte at an offset of a global variable to a specific value and writes a value through a pointer stored in the global variable.
- `update_state_counters_2`: Updates a structure counter based on bitwise comparisons and modifies a signed value depending on specific bit flags.
- `update_state_counters`: Modifies various byte and word values within a structure based on conditional checks and bitwise operations.
- `increment_structure_value`: Adds a value from a register to a word stored at a specific offset within a structure if a bit in a register is set.
- `write_offset_values`: Writes a byte and a word to specific offsets within a memory block pointed to by a register.
- `update_index_table`: Updates a word in a table based on bitwise checks of a structure and a byte value, then calculates a new index using a global variable.
- `update_calculated_index`: Checks a bit in a register and a byte in a memory block to conditionally update a word at an offset using a combination of a global variable and a calculated value.
- `check_boundary_conditions`: Evaluates several conditions involving a global variable and values within a memory block to conditionally write a byte and two words to specific offsets.
- `update_range_parameters`: Updates several fields in a structure, including a value derived from a global variable and a selection from a range of constants.
- `set_range_value`: Compares a global variable against two thresholds to select one of three possible word values and stores the result in a memory block.
- `reset_structure_data`: Sets several fields within a structure to specific constant values and performs a calculation based on a global variable.
- `calculate_status_value`: Returns a value determined by checking a global variable and comparing values within a structure.
- `adjust_structure_values`: Modifies several fields in a structure, including a counter and coordinate values, based on conditional checks and thresholds.
- `clamp_and_clear_data`: Calculates a clamped value, adds it to a structure field, and clears multiple structure members if a threshold is met.
- `clear_structure_data`: Adds a value to a structure offset and clears multiple fields within that structure.
- `update_input_state`: Updates a structure based on a signed value and performs conditional logic involving a counter and bitwise operations.
- `clear_structure_offsets`: Sets two specific word offsets within a structure to zero.
- `reset_structure_values`: Writes zero to two word offsets within a structure.
- `evaluate_input_logic`: Checks bitmask flags in a structure to update status values and a counter based on several conditional comparisons.
- `set_state_offset`: Calculates a value based on the difference between two memory locations and stores it in a structure offset.
- `update_joystick_state_4`: Processes input data from a pointer-based structure to update internal state counters and trigger a jump table based on specific input patterns.
- `set_dac_parameters`: Sets a byte to zero and, if a condition is met, updates several values in a structure including two memory addresses and a short value.
- `set_dac_parameters_alt`: Sets a byte to zero and, if a condition is met, updates two memory addresses, a short value, and a short in a structure.
- `update_timer_state`: Manages a structure by conditionally resetting several fields, updating a short value, and calling another routine based on a comparison.
- `evaluate_conditional_logic`: Performs complex conditional checks on structure fields and a global variable to update multiple offsets and trigger an indirect call.
- `set_dac_values`: Increments a short in a structure and sets a byte and two memory addresses, with one address being conditional.
- `update_dac_state`: Sets a byte to a specific value and, if a condition is met, updates a memory address, a short value, and another memory address.
- `update_animation_frame_and_offset`: Updates a frame counter and calculates a new animation offset based on conditional checks of structure values and a global variable.
- `process_conditional_logic_2`: Performs complex conditional checks on structure values and a global variable to update multiple fields and offsets.
- `update_conditional_word`: Updates a word in a memory block based on comparisons of values at specific offsets within that block and another memory block.
- `process_input_logic`: Checks bitmask flags in a structure to update a short value, call another routine, and conditionally clear two short values.
- `increment_counter_with_limit_2`: Increments a word at an offset within a memory block, prevents it from becoming negative, and clears two other offsets if the value exceeds a limit.
- `increment_counter_with_limit`: Increments a word at an offset within a memory block, prevents it from becoming negative, and clears two other offsets if the value exceeds a threshold.
- `copy_indexed_byte`: Copies a byte from a table into a structure using an index derived from a bitwise mask.
- `update_state_and_increment_counter`: Sets a byte to a specific value based on comparisons of short integers within two structures.
- `increment_or_clear_byte`: Sets a byte to zero if a specific value is non-negative, otherwise increments a word in a structure.
- `update_state_and_counter`: Sets a byte to a specific value based on several conditions and increments a word in a structure.
- `check_dac_status`: Checks if a value in a structure is non-negative and, if not, increments a short in a structure.
- `copy_table_data`: Copies two 32-bit values from a table into a memory block using an index provided in a register.
- `set_global_and_constants`: Sets a global variable to a specific value and populates a structure with large constant values.
- `update_calculated_index_2`: Updates a word in a structure by calculating a new value from a bit-shifted index and a byte from a table.
- `clear_memory_offsets`: Sets two words at specific offsets within a memory block to zero.
- `validate_value_range`: Calculates the difference between two values and returns a specific constant if the result falls within a certain range.
- `validate_limit_and_halt`: Compares a value from a table against a global variable and updates that variable if a condition is met, then enters an infinite loop.
- `execute_infinite_loop_3`: This routine enters an infinite loop that prevents further execution.
- `update_structure_offsets`: Increments two values within a structure and performs a conditional update to a third value and a fourth value based on a threshold check.
- `decrement_and_check_value`: Decrements a value in a structure, sets a field based on a sign check, and performs multiple assignments to various memory locations if the decremented value falls below zero.
- `modify_structure_coordinates`: Updates multiple offsets in a structure, including coordinate-like values, based on bitwise checks and sign comparisons.
- `execute_conditional_callback_2`: Calls another routine and returns the concatenated result of two input values.
- `execute_conditional_callback`: Evaluates several conditions involving a global variable and structure values to decide which routine to call and how to update a global variable.
- `set_high_pointers`: Stores two specific high memory addresses into a global variable.
- `set_mixed_pointers`: Stores a high memory address and a low memory address into a global variable.
- `calculate_offset_value`: Calculates a signed value by adding a constant to a value from a structure based on two different conditional checks.
- `validate_pointer_range`: Checks if a pointer falls within a specific memory range and verifies that the value at that pointer has its most significant bit cleared.
- `update_global_stack`: Sets a bit in a word based on a byte test and then performs a series of word and long transfers into a global variable.
- `update_global_from_table_index`: Checks if a value is non-negative via a table iteration routine, then uses a shifted value from a register as an index to load a byte from a table into a global variable.
- `execute_conditional_calls`: Performs a series of indirect calls and updates a global variable if a signed value is non-negative.
- `execute_indirect_call`: Calls a routine using an address calculated by adding an offset to a global variable.
- `initialize_structure_from_byte`: Deconstructs a byte into bitfields and stores the resulting values into a structure at various offsets, then sets a bit in a byte at a specific offset.
- `reset_structure_values_2`: Writes constant values into a structure at specific offsets and sets a bit in a byte at a specific offset.
- `set_structure_flags_2`: Sets a bit in a byte within a structure and clears a word at a specific offset.
- `set_structure_flags`: Sets a bit in a byte and populates two word fields within a structure with constant values.
- `copy_and_transform_data`: Initializes a structure, updates flags, and iterates through a table to perform bitwise negation and copy operations on data blocks.
- `update_structure_and_decrement`: Populates a structure with constant values and then conditionally decrements values at specific offsets within a structure based on the value of a global variable.
- `sync_structure_with_global`: Extracts a bit from a global variable to set a flag, then populates multiple fields in a structure using the remaining bits of that global variable.
- `populate_status_from_global`: Sets bits in a structure and copies the absolute value of a global variable into two different structure offsets.
- `process_table_and_clear_flags`: Iterates through a table of short integers to call another routine for each non-negative entry, then clears the high bit of multiple global bytes.
- `call_conditional_offset`: Calls a routine using an offset from a structure if a specific value in that structure is non-negative.
- `increment_structure_counters`: Increments a value in a structure and conditionally updates other fields or performs a bitwise XOR on a byte based on comparisons between structure members.
- `process_data_stream`: Reads values from a table using an offset stored in a structure, updates that offset, and conditionally performs bitwise operations or resets values based on distance calculations.
- `update_timer_and_counter`: Subtracts a value from a short, increments a counter, and updates a value from a table based on indexed offsets.
- `clear_status_bit_and_reset_flag`: Clears a specific bit in a byte within a structure and, if a secondary bit is set, sets a different byte in that structure to all ones.
- `update_state_machine_structure`: Updates a state machine within a structure by incrementing a counter and transitioning through various state values.
- `decrement_structure_counter`: Increments a word in a structure if a global bit is clear and decrements another word while toggling a bit.
- `clear_structure_bit`: Clears a bit in a byte within a structure.
- `execute_recursive_calls_3`: This routine performs three consecutive calls to itself before returning.
- `fill_memory_block`: Fills a series of memory locations with a specific value using a loop and resets a global variable to zero.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_structure_with_data`: Populates a structure with several words and bytes from registers and then increments a global variable.
- `clear_structure_fields`: Clears multiple fields within a structure and populates the first four words with register values before incrementing a global variable.
- `initialize_structure_fields_2`: Populates various fields of a structure with specific values, sets a bit in a byte, and increments a global variable.
- `modify_structure_and_decrement`: Clears a bit in a byte within a structure, sets a value to all ones, and decrements a global variable before recursively calling itself.
- `copy_and_decrement_global`: Sets a value to all ones, copies a word from a structure to a global variable, and decrements a global variable before recursively calling itself.
- `decrement_and_recurse`: Sets a value to all ones, decrements a global variable, and recursively calls itself based on the new value of that global variable.
- `check_signed_value`: Checks if a value in a structure is negative and returns either zero or negative one based on the result.
- `update_scaled_value`: Calculates a value by subtracting a constant from a memory location and dividing by a constant, then stores it in a structure and increments a global variable.
- `insert_into_list`: Searches through a list of integers to find a match and inserts a new value into that position, shifting subsequent elements.
- `check_table_values`: Iterates through a table of short integers and returns zero if any value is negative, otherwise returns a specific constant.
- `decrement_and_clear_bit`: Clears a bit in a byte within a structure, sets a byte to all ones, and decrements a global variable.
- `compare_short_value`: Compares a short value at an offset within a structure against negative one and returns zero or negative one.
- `return_constant_5`: Returns a constant value of negative one.
- `search_table_for_index`: Iterates through a table of integers to find a value equal to negative one, then stores the calculated offset into a structure.
- `subtract_and_store`: Subtracts a constant from a value provided in a register and stores the result in a memory location offset from a pointer.
- `call_dual_boundary_check`: Calls a routine that performs two separate boundary checks using a global variable.
- `call_dual_boundary_check_2`: Calls a routine that performs two separate boundary checks using a global variable and returns the result.
- `check_bounds_dual`: Performs two separate boundary checks using a global variable and returns zero if both conditions are met, otherwise returns a constant.
- `check_bounds_alternative`: Compares two input values against a global variable and two different constants to determine if they fall within specific ranges.
- `process_bitfield_data`: Calculates a memory address based on multiple inputs and performs a bitwise check within a loop that decrements a counter.
- `validate_state_flags`: Checks multiple conditions involving a pointer and a value to return either zero or a constant.
- `scan_memory_for_bits`: Scans two memory areas for specific bit patterns in words and returns a combined value based on the loop counts.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `update_counter_and_calculate`: Increments internal counters, checks for overflows, and calculates a new value by multiplying a byte from a table with a multiplier and adding an offset.
- `update_value_from_table`: Retrieves a byte from a table using an index and updates a destination memory location with a calculated product and offset.
- `update_complex_state_2`: Increments counters in a structure, performs pointer arithmetic based on table values, and updates a structure with bitwise operations and arithmetic.
- `update_complex_state`: Modifies a bitmask in a structure, updates a byte value, performs arithmetic on a pointer-based value, and calculates a new value using a multiplication and addition into a long integer.
- `clear_two_bytes`: Sets two consecutive bytes at a specific offset within a structure to zero.
- `calculate_scaled_value`: Conditionally selects a value based on a bit test, stores it in a structure, and performs a multiply-accumulate operation into a long integer.
- `search_table_for_match`: Iterates through a table of structures to find an entry where a specific value matches a scaled register value.
- `compare_short_value_2`: Checks if a short value at a specific offset is non-negative and returns zero or negative one.
- `update_position_with_acceleration`: Increments a counter and updates a secondary value if a bit is set, then applies a signed, halved value to a primary pointer-based value.
- `apply_velocity_to_position_2`: Increments a value in a structure and modifies a primary pointer-based value using a halved signed value from that structure.
- `apply_velocity_to_position`: Adds a halved value to a secondary field and applies a signed, halved value from a structure offset to a primary pointer-based value.
- `update_velocity_and_position`: Conditionally updates a secondary field using a halved counter value, then applies a signed, halved value to a primary pointer-based value.
- `apply_dual_velocity_2`: Updates a secondary field using a halved value and then applies a modified signed value to a primary pointer-based value.
- `apply_dual_velocity`: Updates a secondary field using a halved value and an offset, then applies a signed, halved value to a primary pointer-based value.
- `reset_structure_on_condition`: Checks a bit in a structure and, if a specific condition is met, resets several structure fields and updates a global byte.
- `check_structure_condition`: Tests a bit in a structure and performs conditional routine calls based on whether a value in a pointer-based structure is negative.
- `return_negative_one`: Returns a value of all bits set to one.
- `execute_conditional_recursion`: Performs multiple recursive calls based on the state of specific bits within a structure.
- `process_bit_flags`: Evaluates bit flags in a structure to determine whether to execute subsequent recursive calls.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `check_signed_condition`: Tests if a short value at a specific offset is negative and executes a routine based on that result.
- `compare_register_values`: Returns zero if two specific register conditions are met, otherwise returns all ones.
- `test_register_zero`: Returns a specific value if a register contains zero, otherwise returns a different value.
- `check_distance_thresholds`: Compares differences between a global variable and two input values against specific thresholds using values from a memory structure.
- `check_threshold_bounds`: Compares two calculated differences against dynamic thresholds to determine if a condition is met, returning a boolean-style result.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `manage_state_counters`: Decrements byte counters in a structure and performs conditional updates to a global variable and other structure fields based on those values.
- `decrement_and_validate_counters`: Decrements a byte in a structure and, if it reaches zero, performs conditional updates to a global variable and other memory fields.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 254 into d0, then returns.
- `return_all_ones`: The routine returns a value consisting of all bits set to one.
- `validate_coordinate_offsets`: Calculates the absolute difference between a global variable and two input values to check if they fall within specified limits.
- `populate_status_structure`: Derives several values from a global variable and stores them into specific offsets within a structure.
- `update_structure_flags`: Sets a bit in a byte, modifies a global variable by masking its high bit, and stores the modified value into several fields of a structure.
- `update_structure_flags_and_values`: Extracts the high bit from a global variable to set a flag in a structure, then clears that bit and populates several fields in the structure with the remaining value.
- `modify_structure_bit`: Clears a specific bit in a byte within a structure and performs conditional branching based on a signed value.
- `execute_unreached_code`: This routine is unreached and contains code decoded from a gap in the executable.
- `decrement_and_toggle_bit`: Decrements a value in a table and toggles a bit in another location if a condition is met.
- `decrement_and_toggle_bit_2`: Decrements a value at a memory offset and, if the value is below a specific threshold, performs a bitwise XOR operation on a different memory location.
- `update_structure_and_counter`: Increments a short value in a structure if a global bit is clear, toggles a bit in another structure field, and decrements a counter before recursively calling itself.
- `set_structure_fields`: Sets a byte and two words within a structure using specific constant values.
- `modify_structure_bits`: Clears a specific bit in a byte within a structure and conditionally sets another byte to all ones.
- `update_structure_fields`: Stores constant values and register data into a memory block, modifies a byte via a bitwise OR, and performs several conditional operations on a structure pointed to by a register.
- `tiny_ori_ori_addi`: Small routine of 3 instructions that merges 6 into d4, then merges 3 into d1, then adds 9 to d7.
- `tiny_ori_subi`: Small routine of 2 instructions that merges 8 into d2, then subtracts 5 from d4.
- `return_register_value_2`: Returns the current value of a data register to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_with_ones`: Fills a block of memory with all ones using a loop and resets a global variable to zero.
- `find_and_reset_marker`: Iterates through a table of integers to find a value of negative one, then resets it to zero and increments a global variable.
- `zero_and_increment`: Stores zero at a location pointed to by a register and increments a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_nested_data_structure_2`: Iterates through a two-dimensional data structure using nested loops, performing conditional recursive calls based on the values found within the table.
- `traverse_data_table`: Performs a nested loop to iterate through a two-dimensional data structure.
- `execute_indirect_call_5`: Performs an indirect jump to a location determined by adding an offset to a base address.
- `update_animation_frame_2`: Compares a value against a global variable to determine if an animation sequence should advance.
- `check_proximity_and_reset`: Compares two input values against ranges in a structure and, if a global variable is one, clears four words and sets a byte in that structure.
- `initialize_global_variable`: Sets a global variable to zero and conditionally updates it to a maximum byte value based on the return value of another routine.
- `set_and_verify_global`: Sets a global variable to a constant value and checks if it is zero.
- `update_global_from_bounds`: Compares input values against ranges in a structure and updates a global variable if specific proximity conditions are met.
- `validate_and_clear_data`: Checks if several structure offsets fall within specific bounds and, if a global variable is one, clears four words and sets a byte in that structure.
- `clear_memory_block_4`: Fills a block of memory with zeros using a loop.
- `iterate_table_entries_3`: Loops through a table of values, calling two different routines to update counters and offsets for each non-zero entry found.
- `update_timer_and_offset`: Decrements a byte-sized counter within a structure and updates a secondary value based on the counter reaching zero.
- `decrement_counter_and_offset_2`: Decrements a byte in a structure, resets it to a specific value if it reaches zero, and updates a calculated offset based on a counter.
- `decrement_counter_and_offset`: Decrements a byte in a structure and updates a calculated offset based on a secondary counter value.
- `update_counter_and_offset`: Decrements a byte in a table and updates a calculated offset based on a counter.
- `update_table_and_offset`: Performs bitwise operations on a table value and calculates a new offset.
- `reset_table_values`: Sets a table value to zero, sets a specific offset to a constant, and stores a byte in a calculated memory location.
- `copy_table_data_4`: Compares a value against a global variable and, if they differ, iterates through a table to copy specific short words into a destination array.
- `process_timer_table_entries`: Iterates through a table of timer entries to decrement counter values and update associated memory locations.
- `update_timer_table_entries`: Iterates through a table of entries to decrement counter values and update associated memory locations.
- `update_timer_table_entries_2`: Iterates through a table of entries to decrement counter values and update associated pointers or offsets.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `copy_table_data_2`: If a bit in a global variable is not set, it copies a series of short values from a table to another memory location.
- `copy_table_data_3`: Checks a bit in a global variable and, if not set, copies words from a source table to a destination address until a counter reaches zero.
- `initialize_ram_values`: Calls several other routines and sets four specific memory locations to a maximum unsigned value.
- `scale_pixel_data`: Performs a nested loop to scale nibble values within words by multiplying them by a factor and shifting the result.
- `check_proximity_and_clear`: Compares input values against ranges in a structure and, if conditions are met, clears four words and sets a byte within that structure.
- `clear_structure_on_proximity`: Checks if input values are within a specific range of structure values and, if so, clears several fields and updates a global variable.
- `check_coordinate_bounds`: Compares two sets of values against specific ranges and calls another routine if the conditions are met.
- `decrement_and_loop_global`: Decrements a global variable and calls itself recursively until the value reaches zero.
- `update_table_values_4`: Checks several arithmetic conditions against values in a table and, if met, performs a series of recursive calls while updating a global pointer and writing data to memory.
- `validate_and_reset_global`: Checks if a global variable and two input values fall within specific ranges, then performs recursive calls and sets the global variable to a specific value.
- `increment_global_variable`: Increments a global variable if it meets specific range and value criteria, with logic to reset or wrap the value if it exceeds a threshold.
- `reset_global_variable_4`: Sets a global variable to a specific value and performs a conditional check based on the result of another routine.
- `update_table_values_2`: Updates a value in a structure based on range comparisons and conditionally negates a long integer at a fixed offset.
- `update_table_value`: Compares a value at a pointer against a threshold and a range, then updates the pointer's value and negates a long integer at a fixed offset.
- `update_status_flags_3`: Checks if a value is within a specific range and compares it to a threshold, then updates a memory location and clears a byte if certain conditions match.
- `reset_structured_data_block`: Validates that several offsets within a table fall within specific bounds before clearing four consecutive words and setting a byte at a specific offset to a constant value.
- `clear_structured_data`: Verifies several offset ranges and a byte value before clearing four consecutive memory words and setting a byte to a specific value.
- `validate_offset_ranges`: Verifies that multiple calculated offsets fall within specific bounds before calling another routine.
- `decrement_global_counter_3`: Decrements a global variable and, if it reaches zero and a specific bit is set, assigns two constant values to it.
- `process_entity_state`: Updates a global state and performs complex calculations involving pointers and offsets to store new values in a table.
- `execute_sequential_subroutines`: Calls a series of subroutines in a specific sequence and stores a value from a global variable into another global variable.
- `execute_sequence_of_calls`: Executes a series of calls to other routines and functions, finally storing a global variable into a specific memory location.
- `execute_reset_sequence`: Clears a global variable, calls a subroutine to fetch a value from a calculated memory offset, and then enters an infinite loop.
- `reset_and_loop_forever`: Sets a global variable to zero, calls another routine with stack values, and then enters an infinite loop.
- `reset_global_flag`: Calls another routine and then clears a specific byte in a global structure.
- `clear_memory_block_5`: Writes zero to eight consecutive words starting at a specific memory address.
- `execute_nested_loops`: Performs multiple nested loops that call other routines and manipulate a global variable and a pointer-based value.
- `process_string_data`: Iterates through a string, incrementing a counter for each character and resetting it upon encountering a carriage return.
- `fill_pattern_buffer`: Clears a block of memory and then fills it with repeating patterns of nibbles derived from three global bytes.
- `update_memory_with_bitmask`: Iterates through a memory range and performs bitwise AND and OR operations using values from a table and a mask.
- `initialize_hardware_and_state_tables`: Clears a range of custom chip registers, populates a table with incrementing values, and performs bitwise inversion on a block of memory.
- `set_memory_bits`: Performs bitwise OR operations on three specific memory locations using constant values.
- `execute_loop_until_bit_cleared`: Executes a sequence of calls including a loop that continues as long as a specific bit in a global variable is set.
- `increment_memory_value`: Increments a byte at a calculated memory location by three and stores a value to a stack-based location.
- `apply_bitmask_multiple_times`: Calls another routine multiple times using a calculated address based on a global variable shifted left by three.
- `apply_bitmask_twice`: Calls another routine twice using a calculated address based on a global variable shifted left by three.
- `bitwise_not_and_self`: Returns the result of a bitwise NOT operation on a global variable ANDed with itself.
- `infinite_loop_empty`: The routine is empty, consisting only of an infinite loop.
- `update_global_with_table_lookup`: Increments a global variable, resets it if it reaches a certain value, and then replaces it with a value loaded from a table using a calculated index.
- `update_indexed_data`: Loads a value from a table to use as an index, then copies data from two different memory locations into several destination registers or variables based on that index.
- `manage_state_variable`: Updates a global variable through a loop based on the results of another routine, potentially resetting it to zero or a specific value depending on bitwise flags.
- `fill_memory_buffers`: Iterates through a memory range in steps of sixty-four bytes, calling a routine to fill buffers within each step.
- `fill_multiple_buffers`: Fills several destination buffers with specific word values in a nested loop structure.
- `apply_bitmask_to_destination`: Reads a value from a list, uses it to index a table, and performs bitwise operations to update a destination buffer.
- `reorganize_nibbles_in_table`: Iterates through a table and rearranges the nibbles within each word based on a bit-shifting pattern.
- `copy_memory_blocks_4`: Performs a nested loop to copy multiple blocks of data from a source pointer to a destination address.
- `copy_memory_block_4`: Copies a block of words from a source pointer to a destination pointer using a nested loop structure.
- `initialize_relocation_data`: Performs a sequence of routine calls and copies a block of words from a source table to a destination address before populating a secondary table with incrementing pointer values.
- `initialize_data_tables`: Copies a sequence of words from one memory location to another and populates a table with incrementing values.
- `call_and_concatenate_result`: Calls another routine and returns a value composed of a constant and the result of the called routine.
- `execute_infinite_loop_4`: This routine is empty and performs no actions other than an infinite loop.
- `apply_bitmask_to_table`: Iterates through a list of values and performs bitwise AND and OR operations on specific offsets within a destination table based on data from a source table.
- `initialize_relocation_and_data_tables`: Initializes relocation data by copying multiple blocks of memory from a source table to a destination.
- `update_global_from_table`: Calculates an offset based on a global variable to fetch two words from a table and store them into that global variable.
- `call_subroutines_recursively`: This routine calls itself repeatedly through a series of recursive calls before returning.
- `call_set_address_offsets_alt`: Calls another routine that writes a fixed value to a memory offset and updates a pointer-based table with incremental address values.
- `update_address_table`: Updates a series of memory offsets with new address values while waiting for a global flag to clear.
- `set_address_offsets`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values.
- `set_address_offsets_alt`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values using a different source register.
- `mask_and_shift_value`: Applies a bitmask to an input value and shifts the result right by three bits.
- `update_global_threshold`: Compares a global variable against a series of constant thresholds and updates it with a specific value based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_no_effect_entry`: The body is only a return, reached by executing the program rather than by static flow.

The function list continues in: .
