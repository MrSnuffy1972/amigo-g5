# Game 261 (Exile v1.3 0032): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00261). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911969 top-level bytes in 116 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 121 of 121 units identical (bytes and relocations), 149 raw-word lines |
| Functions named and described | pass, 855 of 855 named (0 author symbols), 855 of 855 described (31 by rule, 727 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 10521 of 10535 (99.87%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15647 of 17872 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 481283 |
| code | 65786 |
| data | 332546 |
| padding | 610 |
| unused | 31744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Exile.Slave | 1 | 288 | 30 |
| logged disk load from Disk.1 | 1 | 2 | 1 |
| logged disk load from Disk.1 | 1 | 4 | 2 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 422 | 11 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 109 | 23 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7974 | 335 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1422 | 56 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2279 | 148 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 49 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 34 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 238 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 177 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 793 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 695 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 834 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 854 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |

## Cases handled

- Exile_v1.3_0032/Exile/Disk.1: raw disk image (705024 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Exile_v1.3_0032/Exile/Disk.2: raw disk image (22528 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 13 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 4596 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1122 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 220 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 159 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 488 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 459 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 609 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 581 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 111 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +CB20, runtime segment an address +CB18, runtime segment an address +CB0E, runtime segment an address +CB06
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (9 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +438E
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (9 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (9 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2AE, Exile_v1.3_0032/Exile/Exile.Slave +29E
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2C4, Exile_v1.3_0032/Exile/Exile.Slave +2BA, Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (13 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2C4, Exile_v1.3_0032/Exile/Exile.Slave +2BA, Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2B4, Exile_v1.3_0032/Exile/Exile.Slave +2A4
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +3D0, runtime segment an address +3DA, runtime segment an address +3BA, runtime segment an address +3C4
- self-modifying code at an address (2 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +3D0, runtime segment an address +3DA, runtime segment an address +3BA, runtime segment an address +3C4
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E, Exile_v1.3_0032/Exile/Exile.Slave +288, Exile_v1.3_0032/Exile/Exile.Slave +27E
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E, Exile_v1.3_0032/Exile/Exile.Slave +288, Exile_v1.3_0032/Exile/Exile.Slave +27E
- self-modifying code at an address (9 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (9 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (9 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +298, Exile_v1.3_0032/Exile/Exile.Slave +28E
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: runtime segment an address +44A, runtime segment an address +43E, runtime segment an address +39C, runtime segment an address +3AE
- self-modifying code at an address (11 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: runtime segment an address +44A, runtime segment an address +43E, runtime segment an address +39C, runtime segment an address +3AE
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (10 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (5 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +32A, Exile_v1.3_0032/Exile/Exile.Slave +320
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +32A, Exile_v1.3_0032/Exile/Exile.Slave +320
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2D4, Exile_v1.3_0032/Exile/Exile.Slave +2CA
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-joy) and runtime segment an address): both forms recovered; writer: Exile_v1.3_0032/Exile/Exile.Slave +2D4, Exile_v1.3_0032/Exile/Exile.Slave +2CA
- Exile_v1.3_0032/Exile/Disk.1: 13 logged loads cover 476160 bytes of the image
- Exile_v1.3_0032/Exile/Disk.1: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.053, entropy 4.17)
- Exile_v1.3_0032/Exile/Disk.1: chunks 22-43 (bytes 5632-11264) hold ASCII text
- Exile_v1.3_0032/Exile/Disk.1: chunks 1936-2753 (bytes 495616-705024) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.63)
- Exile_v1.3_0032/Exile/Disk.2: chunks 0-0 (bytes 0-256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- Exile_v1.3_0032/Exile/Disk.2: chunks 16-16 (bytes 4096-4352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- 1 packed track region(s), 3072 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Exile.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_calls`: Loads a pointer, calls a routine via a pointer, and conditionally calls another routine via a pointer before modifying a global and a function pointer.
- `copy_code_block`: Copies a fixed number of bytes from a source address to a destination address.
- `install_jump_vectors_call_base`: Writes jump-instruction patches into several low-memory vector slots and into a block of code locations, makes many direct calls, then calls a routine through the library base pointer with a fixed argument and returns the register pair.
- `check_global_and_call`: Tests if a global variable satisfies a specific bitwise condition and, if not, calls a routine through a pointer stored at a fixed offset from another routine.
- `set_blitter_masks_and_jump`: Writes a pointer value and two blitter mask words, all ones for the first-word mask and 32767 for the last-word mask, then calls a routine through an unrecovered jump table.
- `patch_matching_table_entries`: Walks a table of word pairs and, wherever a word matches the input and the following word equals 88, overwrites that entry with a fixed long value.
- `store_word_and_wait_blitter`: Stores the input data-register word at offset 88 of the record and then waits in a loop until the blitter-busy bit clears.
- `store_word_d2_wait_blitter`: Stores the data-register-two word at offset 88 of the record, then waits until the blitter-busy bit clears.
- `store_word_d3_wait_blitter`: Stores the data-register-three word at offset 88 of the record, then waits until the blitter-busy bit clears.
- `store_word_d4_wait_blitter`: Stores the data-register-four word at offset 88 of the record, then waits until the blitter-busy bit clears.
- `store_word_d0_wait_blitter`: Stores the data-register-zero word at offset 88 of the record and then waits until the blitter-busy bit clears.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_global_word_wait_blitter`: Stores the shared global word at offset 88 of the record and waits until the blitter-busy bit clears.
- `store_and_poll_bit`: Stores a global variable into a memory location relative to a pointer in a register and then loops until a specific bit in that global variable is cleared.
- `store_param_word_wait_blitter`: Stores the word passed as its parameter at offset 88 of the record and waits until the blitter-busy bit clears.
- `store_long_low_word_wait`: Stores the low word of its long parameter at offset 88 of the record and waits until the blitter-busy bit clears.
- `store_constant_and_wait_blitter`: Stores the fixed value 524 at offset 88 of the record and waits until the blitter-busy bit clears.
- `write_constants_return_fixed_value`: Writes three fixed values into a block of memory and returns a fixed long constant.
- `swap_word_runs_in_place`: Swaps words between a forward pointer and a backward pointer, one pair per pass, for a count taken from the data register, which reverses a run of words in place.
- `copy_jump_vector_data`: copies a block of memory or a specific word or longword to a destination address.
- `set_global_to_two`: Stores the value two into a global variable.
- `store_constant_two_and_return`: Stores the constant two to a shared location and returns.
- `set_global_to_one`: Stores the value one into a global variable.
- `dispatch_code_then_call_base`: Compares a 16-bit code parameter with two fixed negative values; one match calls a routine through the library base pointer and then another routine, while other values call a different base routine and return.
- `add_constant_across_table_rows`: Walks packed entries on the stack, adding a fixed long constant to memory locations selected by nibbles of each entry, and writes the same constant to the shared global.
- `add_constant_table_walk_variant`: Performs the same table walk as its neighbour but returns early with the shared global written when an entry's count is zero.
- `negate_memory_value`: Negates a short-integer value stored at an offset from a pointer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine`: the routine contains no executable instructions and performs no operations or data manipulations.

### logged disk load from Disk.1

- `empty_routine_2`: The routine contains no executable instructions and performs no operations.

### logged disk load from Disk.1

- `halt_execution`: The routine contains bad instruction data and stops execution.
- `return_empty_routine`: The routine contains no valid instructions and performs no operations before returning.

### logged disk load from Disk.1

- `call_two_routines_passthrough`: Calls two other routines in sequence and returns the register pair unchanged.
- `restore_interrupt_state`: Restores an interrupt request state and writes a specific pattern of bytes to memory.
- `save_state_enable_interrupt_bit`: Saves the shared global word into two memory slots, then writes a pointer value and a set-bit interrupt-enable word to the shared global.
- `restore_interrupt_request_state`: When a saved status word has its bit 5 set, writes the value 32 to the shared global, then writes back the saved word.
- `check_interrupt_state_recursively`: Checks if a specific bit is set in a global variable and recursively calls itself if the condition persists.
- `scan_rows_clear_slot_table`: Scans byte rows until it finds a free index, clears a 30-long slot table with pointer links, sets a status bit, and writes a fixed set of zero fields to the shared global.
- `write_fifteen_then_four_zeros`: Writes the value 15 followed by four zero values to the shared global.
- `reset_audio_pointers_and_advance`: Advances a shared counter and, when the counter condition holds, waits in two timing loops, sets the top bit of a shared global, copies eight table words into the audio channel start-pointer registers, and advances an index until it reaches 1024.
- `update_channel_record_and_commands`: Loads a record from a stream, derives its start pointer and length from a table, stores masked word fields, and dispatches a command nibble that toggles a flag, sets a value or updates a counter; it touches the audio channel start and length registers and the DMA control.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to the low twelve bits, stores the result into an offset of the same structure and clears a byte at another offset.
- `dispatch_channel_command_slide`: Decodes a channel command nibble and applies its effect to a record through table-driven pitch lookups, slides, volume-style steps and offset steps, with some commands writing to the output; it also reads the fire-button input register.

### logged disk load from Disk.1

- `modify_global_and_pointer_target`: Calls another routine, performs an XOR operation on a pointer's target, sets a global variable, and applies a bitwise or to a calculated memory location.
- `execute_single_instruction`: Executes a single instruction and is called by another routine.
- `empty_routine_3`: the routine contains no executable instructions.
- `return_immediately_5`: The routine is empty and immediately returns.
- `store_and_xor_memory`: Stores a value from a register into a memory location and then performs a bitwise XOR on a value at a memory location using a register.
- `return_empty_routine_2`: An empty routine that performs no actions before returning.
- `empty_routine_4`: the routine contains no executable instructions and performs no operations.
- `halt_execution_2`: the routine contains invalid or incomplete instruction data that terminates the control flow.
- `return_void`: the routine is empty and performs no operations before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `void_routine`: The routine contains no valid instructions and performs no operations before terminating.
- `return_undefined_value`: Returns an undefined or uninitialized value.
- `return_immediately`: The routine is empty and performs no operations before returning.
- `frag_jsr`: Short code fragment that calls another routine.
- `set_byte_bit_mask`: Performs a bitwise OR on a byte at a memory location using a constant value.
- `do_nothing`: this routine is empty and performs no operations before returning.
- `mask_register_bits`: Returns the result of a bitwise AND between a register and a constant value.
- `set_multiple_bits`: Performs bitwise OR operations on two consecutive bytes in memory using constant values.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `clear_bit_in_memory_2`: Uses a value from another routine to clear a specific bit in a memory location and then copies that memory value to a pointer.
- `empty_routine_5`: the routine contains no executable instructions or functional logic.
- `execute_empty_routine`: The routine performs no operations and immediately returns.
- `empty_routine_6`: This routine contains no instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_string_replacement`: Repeatedly calls a routine that replaces string characters while updating a global variable with values from a memory address, continuing until a return value is zero.
- `replace_string_characters_2`: Iterates through a string and replaces certain byte values with alternative characters.
- `replace_string_characters`: Iterates through a string and replaces certain byte values with alternative characters or symbols.
- `set_high_bit_if_valid`: Checks if a value at a certain offset is non-zero and non-negative, then sets the high bit of that byte.
- `check_bit_and_set_zero`: Checks a bit in a register, calls another routine, and sets a global variable to zero.
- `return_no_op`: The routine consists of a single return instruction with no other effect: it performs no computation, no memory or hardware access, and no library calls.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `modify_global_and_structure_state`: Modifies a global variable and certain structure members through a series of conditional checks, assignments, and calls to other routines.
- `modify_structure_member`: Decrements a byte in a structure and conditionally updates a word or a byte in that structure.
- `decrement_and_store_masked_value`: Decrements a byte at an offset, and if it falls within a certain range, stores a masked or inverted version of a value from another routine into a table.
- `update_structure_flags`: Checks a bit in a global variable and, if certain conditions are met, sets a bit in a structure member.
- `update_structure_and_set_flags`: Updates a structure member or table entry using a combination of bitwise masking and bit-setting, while also performing conditional bit-masking on a global variable and a bit-set operation on a bitmask or flag byte based on a calculated index.
- `update_pointer_flags`: Decrements a global variable and conditionally sets bits in a buffer or updates a pointer offset.
- `check_bit_and_return_error`: Checks a bit in a byte at an offset and, if a certain condition or a returned value is met, returns a failure code.
- `update_global_state`: Updates a global variable and modifies a byte in a table or pointer-based memory based on a conditional check of a value at a pointer.
- `set_pointer_offset`: Calculates a signed offset based on a global variable and stores it in a structure, then copies a value from a table into a structure.
- `update_offset_and_scale_value`: Checks a bitmask in a structure, calculates a signed offset based on a global variable, and stores a value retrieved from a table into a structure along with its doubled value.
- `calculate_masked_offset`: Calculates a masked value based on a global variable and stores it into a structure offset.
- `add_masked_value_to_offset`: If a global variable is negative, calculates a masked value plus a constant and adds it to a value at a specific offset.
- `modify_global_and_structure`: Toggles a bit in a global variable and masks a byte at a calculated offset with a constant.
- `mask_byte_at_offset_2`: Toggles a bit in a global variable and masks a byte at a calculated offset with a constant.
- `reconfigure_buffer_pointers_and_flags`: Updates buffer pointer offsets and flags while conditionally resetting or reassigning memory pointers based on a global state.
- `set_buffer_pointers`: Masks a global variable and conditionally writes new pointer values to a structure.
- `set_status_flag`: Checks a condition involving a structure member and a global variable, then sets a bit in a structure member.
- `decrement_and_check_flag`: Decrements a byte in a structure and sets a bit in a structure member if the decremented value or a global variable meets certain criteria.
- `update_pointer_value`: Checks if a value at an offset from a pointer is less than or equal to a masked and offset global variable.
- `set_pointer_bit`: Compares a value at a pointer offset against a masked and offset global variable.
- `update_offset_with_masked_value`: Calculates a masked value, updates a value at an offset, and then stores a value from a table into a nearby offset.
- `update_structure_bounds`: Updates a structure member with a calculated offset or zero, decrements a counter, and potentially updates a pointer or counter if a limit is reached.
- `update_structure_and_global`: Updates a structure and a global variable by checking bitmasks, calculating offsets, and performing conditional writes or bit clearing.
- `validate_structure_bounds`: Performs checks on a structure member and a global variable, verifying if certain values fall within a specific range.
- `update_memory_with_conditional_logic`: Performs conditional checks on a global variable and a pointer, potentially incrementing a byte in memory and setting certain memory bytes to zero or a constant value.
- `overwrite_global_variable`: Overwrites a global variable with a constant value and values loaded from a memory buffer using an offset, while preserving the original value in a register.
- `clamp_and_store_global`: Checks a returned value against a constant, clamps a global variable, and writes that global variable to a memory buffer.
- `sum_table_elements`: Iterates through a table, summing values, and calculates a final value based on the sum and a carry check.
- `check_global_sign`: Checks if a global variable or a value at a pointer offset is negative and calls another routine.
- `modify_structure_and_global`: Performs complex conditional checks on structure members and a table, then modifies a global variable and several memory locations using bitwise operations.
- `mask_byte_at_offset_3`: Applies a bitwise AND mask to a byte at a calculated offset in memory.
- `clear_bits_and_set_constants`: Clears a bit in certain memory locations using a bitmask derived from a register and writes constant values to a structure.
- `set_structure_constants_2`: Writes constant values to specific offsets within a structure.
- `set_structure_constants`: Writes a constant value of 4 to several offsets within a structure and a different constant to another offset.
- `initialize_structure_constants`: Writes constant values to a structure and copies a value from a memory location using a bit-rotated index.
- `clear_bit_in_memory_3`: Clears a specific bit in a byte at a calculated offset, where the bit position is determined by a value that may be set or cleared based on a global variable and a structure member.
- `update_entity_attribute_flags`: Checks a value or bitmask and updates a byte or dword in a table or structure, potentially setting a high bit or masking bits based on a condition.
- `update_memory_with_bitmask`: Sets a global variable, performs pointer-based writes to a structure, and modifies a byte using a bitmask.
- `nop_padding`: The routine is empty and performs no operations.
- `write_structure_members`: Writes a constant, a register-derived value, and a global variable to a structure at a specific offset.
- `update_global_and_clear_offset`: Stores a value from a register into a global variable and clears a dword at an offset from a pointer.
- `modify_structure_on_match`: Clears a byte or dword in a structure and modifies a global variable if a certain value matches a global variable.
- `search_table_for_value`: Iterates through a table to find a matching value and returns a pointer offset if found, otherwise calls several other routines and performs a memory copy.
- `mask_byte_based_on_condition`: Checks a short value at an offset from a pointer and either masks a bit in a byte or clears a dword at an offset.
- `process_table_updates_2`: Updates a global variable and conditionally increments a value in a table or writes to a buffer.
- `update_entity_position_and_flags_2`: Calculates a new position or offset for an entity based on a scaled or decremented value, then updates a status byte and associated flags in memory.
- `update_pointer_and_bit`: Copies a word from a source pointer to a destination, updates a pointer or offset, and clears a bit in a global variable.
- `update_buffer_and_pointer`: Copies values from a source pointer to a destination, modifies a pointer or offset, and updates a bit in a table.
- `update_coordinate_offset_and_mask`: Calculates a masked or offset coordinate value and updates a pointer-based memory location if certain distance or conditional criteria are met.
- `compare_and_branch`: Compares a value at a pointer-based offset to another value and, if they differ, checks a bit in a byte and calls another routine.
- `decrement_and_check_structure_counter`: Decrements a word-sized counter in a structure and checks if it has reached zero or a certain threshold.
- `clear_offset_if_positive`: Clears a word at an offset from a pointer if a global variable is not negative.
- `set_structure_value`: Updates a value in a structure or modifies a bit in a byte if a certain condition involving a global variable is met.
- `update_struct_member_on_condition`: Calls a routine and checks a condition involving a global variable and a struct member, then copies a word from one struct member to another.
- `decrement_and_update_state`: Decrements a global variable and, if certain conditions involving a structure member and a bit-check are met, updates a byte in a table and modifies a global variable.
- `update_global_and_table_2`: Decrements a global variable and, if certain conditions are met, updates a table entry with a bitwise or conditional value and modifies a global variable via bitwise XOR.
- `set_memory_value`: Writes a constant byte to a memory location at a pointer offset and, if a global variable matches a certain value, calls two other routines and writes a different constant to a pointer offset.
- `modify_global_and_offset`: Increments a global variable and sets a bit in a structure member, or clears a 16-bit value at a structure offset if a threshold is exceeded.
- `execute_unnamed_routine`: This routine is empty except for a call to another routine.
- `clear_structure_offset`: Clears a 16-bit value at a structure offset if a certain value exceeds a threshold.
- `return_without_effect`: The routine consists of a single return instruction and performs no work.
- `store_values_to_table`: Stores two 32-bit values into a table at an offset and modifies a global variable.
- `update_struct_and_conditional_bounds`: Updates a structure member using a bit-shifted value and conditionally modifies a pointer-offset or a global table entry based on sign-checks and magnitude comparisons.
- `initialize_routine_state`: Calls another routine and then sets a bit in a global variable.
- `accumulate_structure_offset`: Adds a value from a structure offset to another offset in the same structure and clears a global variable.
- `initialize_complex_structure_5`: A 42-instruction routine that initializes structure members, copies memory, and updates various tables or global variables using bitmask checks and pointer-based copies.
- `initialize_structure_values`: Calls a routine to retrieve a value and writes it, along with a zeroed word and a constant, to a structure.
- `execute_initialization`: Executes a single routine that performs initialization.
- `initialize_struct_members`: Calls a routine based on a register value and then writes a word, a zeroed word, and a constant value to a structure.
- `set_global_constant_2`: Sets a global variable to a constant value.
- `set_global_high_bit`: sets the most significant bit of a global variable.
- `set_global_bit`: Sets the high bit of a global variable.
- `set_global_flag_bit`: sets the high bit of a global variable.
- `clear_bit_and_set_flag`: Clears a bit at a specific offset within a byte pointed to by a register and sets the high bit of a global variable.
- `clear_bit_at_offset`: Calculates an address from a register and clears a bit at a certain offset, then sets a global variable to a constant.
- `set_high_bit_in_global`: Sets the most significant bit of a global variable using a bitwise OR operation.
- `update_structure_fields`: If a global variable or certain bitmask matches, it copies the global variable and a value from another routine into several offsets of a structure.
- `update_byte_from_table`: Copies a byte from a table or sets a byte to a constant value based on a global variable and a bitmask check.
- `check_bit_and_branch`: checks if the third bit of a byte at a pointer offset is set and calls another routine if it is.
- `call_subroutine_3`: This routine calls another routine and then returns.
- `update_structure_members`: Checks a value in a structure, and if it falls within certain bounds, adds a shifted version of a structure member to itself with overflow protection, then calls another routine.
- `update_structure_value`: Calculates a new value by adding a shifted version of a structure member to itself, clamps the result to a maximum or minimum, and stores it back.
- `set_and_call_routine`: Stores a constant value through a pointer and calls another routine.
- `check_register_bit`: Checks if a specific bit is set in a value returned by another routine and returns if it is.
- `check_table_value`: Checks if a value at a calculated offset in a table matches certain constants.
- `get_value_from_pointer`: Reads a value from a pointer-based offset and returns a specific constant based on bitmask checks.
- `map_bitmask_to_value`: Checks a bitmask in a memory location and returns a specific constant or value based on which bit is set.
- `return_empty_routine_3`: the routine performs no operations and immediately returns to its caller.
- `do_nothing_routine`: the routine is empty and performs no operations or data transfers.
- `update_table_with_clamping`: Updates a global variable, potentially updates a table or structure, and subtracts a value from a table entry while clamping the result to zero.
- `set_table_entry`: Calls another routine and, if a certain flag is set and a register is non-zero, writes a constant value to a table entry.
- `subtract_until_limit`: Repeatedly subtracts a value from a structure member until it certain condition is met, then updates the structure member with the result.
- `update_table_with_offset`: Calculates a new value based on an absolute value from a pointer and an offset, then stores the result back to a table.
- `sync_structure_member`: If a global variable satisfies a condition, it copies a value from one offset in a structure to another offset and returns it.
- `sync_pointer_value`: Copies a value from one offset in a structure to another offset and returns it.
- `get_maximum_absolute_value`: Compares the absolute values of two numbers from a structure and returns the larger one.
- `find_maximum_absolute_value`: Calculates and returns the maximum of the absolute values of two or more values stored in a table.
- `calculate_maximum_difference`: Calculates the absolute difference between values in a table and a second set of values, returning the maximum difference found.
- `clear_bit_from_table`: Checks if a value is positive, calls another routine, and if the returned value meets certain criteria, clears a bit in a memory location using a calculated offset.
- `calculate_and_store_scaled_value`: Performs a loop that calculates a value, checks a condition, and writes a global variable to a table or structure while updating a counter.
- `update_coordinate_with_offset`: calculates a new value by adding a base value from a table to a bit-shifted value from a secondary table, then stores the result into a destination pointer.
- `scale_global_variable`: Multiplies a global variable by two and four, storing the results in a destination buffer.
- `extend_global_variable`: If a register matches a global variable, it overwrites the upper byte of that global variable with a constant.
- `validate_and_update_pointer`: Checks a value from another routine and, if certain conditions are met, updates a pointer-based structure or returns an error code.
- `initialize_structure_with_loop`: Iterates a certain number of times, performing several writes to a structure or table, including setting certain offsets to constant values and copying a global variable.
- `modify_memory_and_flags`: Writes a global variable to a memory location and toggles a bit in a byte using a bit-shift or bitwise OR. It also modifies a bit in a word or long-word via a pointer.
- `copy_memory_and_set_flag`: Performs a reverse copy of a memory block, sets a bit in a structure member, and modifies a global variable using a value from a pointer-offset calculation.
- `set_buffer_flags`: Sets specific bits in a byte at a buffer offset and writes constant values to subsequent buffer locations.
- `update_buffer_with_offset`: Modifies a byte in a table using a bitwise OR and writes constant values to a nearby buffer location.
- `modify_memory_and_set_flag`: If a certain condition is met, copies a memory block, sets a global variable, and clears a bit in a structure member or a global variable.
- `copy_and_modify_memory`: Copies a block of memory, updates a global variable, and performs bitwise modifications on a byte at a pointer-offset or a global variable.
- `do_nothing_4`: this routine is empty and performs no operations.
- `swap_global_values`: Swaps the values of two global variables if certain conditions involving a table lookup and a global variable check are met.
- `apply_bitwise_logic_to_table`: Iterates through a table, performing a bitwise AND between a source and a destination, and calls a routine via a function pointer table.
- `process_global_logic`: Performs a sequence of checks on a global variable, potentially calling other routines, updating a global variable with a constant, and performing memory writes or bitwise operations.
- `modify_memory_and_check_global`: Performs several memory writes and bitwise operations, including a check against a global variable and a subtraction.
- `reset_global_variable_3`: Sets a global variable to zero and checks a bit in that variable before potentially calling another routine and setting a global variable to a fixed value.
- `return_empty_routine_4`: The routine performs no operations and immediately returns to its caller.
- `execute_conditional_subroutines_2`: Checks a global variable and, if it matches a specific constant, calls a sequence of subroutines.
- `execute_memory_modification`: Calls another routine that performs memory writes and checks against a global variable.
- `repeat_memory_modification`: Executes a routine that modifies memory multiple times using a loop.
- `update_global_pointer`: Updates a global variable with a value loaded from a pointer-offset or a table.
- `return_immediately_stub_12`: The routine consists of a single return instruction and performs no work, so it does nothing when entered.
- `modify_memory_bits`: Sets a global variable to zero and performs bitwise operations on memory locations using a bitmask derived from another routine.
- `clear_global_and_mask`: Sets a global variable to zero and performs bitwise masking on a memory location using a bitmask derived from an input register.
- `set_global_constant_6`: Calls another routine and then sets a global variable to a fixed constant value.
- `set_global_constant_3`: sets a global variable to a constant value.
- `set_global_to_256`: Sets a global variable to the constant 256 and returns, with no other effects shown.
- `set_global_constant_4`: stores a constant value into a global variable.
- `return_noop_stub`: The routine consists of a single return instruction and performs no work: it neither reads nor writes any register or memory location.
- `set_global_constant_7`: Calls another routine and then sets a global variable to a constant value.
- `set_global_to_constant`: sets a global variable to a fixed constant value.
- `set_global_constant_5`: assigns a constant value to a global variable.
- `update_pointer_with_wrap_check`: Increments a value pointed to by a register and checks if a secondary pointer-based value or the incremented value requires a subtraction to wrap or reset.
- `toggle_bit_in_memory`: Toggles the highest bit of a word located at an offset from a pointer.
- `modify_if_negative`: Checks if a global variable is negative and, if so, calls another routine and restores a value to that global variable.
- `modify_global_if_negative`: If a global variable is negative, sets a bit in that variable and calls another routine.
- `return_no_operation`: The routine consists of a single instruction that returns immediately, performing no work.
- `modify_pointer_and_flags`: Performs pointer-based arithmetic and bitwise operations on a memory location, including clearing a bit in a bitmask.
- `modify_memory_with_logic`: Performs complex conditional checks on a global variable and a pointer, then modifies a memory location using a bitwise or and writes a constant to a structure.
- `update_memory_via_pointer`: A single instruction routine that performs a bitwise OR on a memory location derived from a pointer.
- `update_global_memory`: Performs conditional checks on a global variable and a value from another routine to potentially update a memory location via a pointer.
- `return_immediately_stub_9`: The routine consists of a single instruction that returns to its caller, with no other work performed.
- `load_table_to_global`: Sets a global variable to a value loaded from a table and calls a routine that clears a memory block.
- `return_no_op_9`: The routine consists of a single return instruction and performs no work.
- `return_no_op_4`: The routine consists of a single return instruction and performs no work.
- `update_global_with_table_value`: Updates a global variable with a value loaded from a table using an offset, and performs bitwise or arithmetic operations on memory or registers.
- `transform_integer_digits`: calculates a new value based on the digits of an input integer masked or otherwise modified.
- `mask_memory_word`: performs a bitwise AND operation on a word at a memory location using a constant mask.
- `recursively_check_global`: Recursively calls another routine if a global variable does not match certain other global or memory-based values.
- `execute_loop_and_offset`: executes a routine a fixed number of times and returns a value offset by a constant.
- `update_global_with_function_result`: updates a global variable using a value returned from another routine and then clears that global variable.
- `update_global_and_return`: If a global variable is non-zero, updates it using a value from another routine and then clears it, before calling another routine and returning a concatenated value.
- `execute_recursive_loop`: Checks if a global variable is zero and, if so, assigns a constant value to it before calling another routine twice and returning a concatenated value from registers.
- `update_global_with_mask`: Increments a global variable and applies a bitwise mask and a multiplier in a loop until a zero-value is found in a table.
- `increment_and_mask_loop`: Increments a global variable and applies a bitwise mask and a multiplier in a loop that continues until a certain value in a table is zero.
- `update_global_from_struct`: reads values from a structure using an offset from a register and updates a global variable.
- `execute_conditional_subroutines_3`: Checks a global variable and executes a series of subroutines if certain bits are not set.
- `execute_conditional_routines`: Checks specific bits in a register and calls different routines if those bits are not set.
- `clear_global_variable`: sets a global variable to zero.
- `reset_global_variable_2`: Sets a global variable to zero.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then clears a fixed memory address, then returns.
- `clamp_and_reset_global_variable`: Checks if a global variable is negative and resets it to zero, or otherwise subtracts a constant value and clamps the result within a specific range.
- `clamp_global_value_upward`: Checks if a global variable is negative and, if not, adds a constant and clamps it to a maximum value or calls another routine if it falls below a certain threshold.
- `apply_subtractive_clamping`: Subtracts a constant from a global variable and clamps the result to a minimum value or calls another routine if it falls below a certain threshold.
- `apply_additive_clamping`: Adds a constant to a global variable and clamps the result to a maximum value or calls another routine if it falls below a certain threshold.
- `check_register_and_set_global`: Checks a bit in a register and either calls another routine or sets a global variable to a constant.
- `set_global_to_max_3`: sets a global variable to a maximum byte value.
- `update_memory_and_global`: Performs complex conditional checks on a global variable and a value from another routine, potentially updating a memory location via a pointer, and performing bitwise OR or subtraction operations on memory or global variables.
- `load_from_table_offset_2`: Increments a global variable and loads a new value into it from a table using an offset-based pointer.
- `invert_global_variable`: Performs a bitwise NOT operation on a global variable and calls another routine.
- `process_entity_list_and_update_state`: Iterates through a list of entities or objects, updating certain flags and checking or modifying values in a table.
- `traverse_table_with_limit`: Increments a value and checks it against a limit while iterating through a table of signed shorts.
- `update_table_with_subtraction`: Checks a value in a table and, if a certain condition is met, subtracts a value from a table entry and adds a value to another table entry using a temporary variable.
- `modify_negative_global`: Checks if a global variable is negative and, if so, sets a bit in that variable and calls another routine.
- `update_global_state_3`: Assigns a register value to a global variable and executes a sequence of other routines if a pointer check passes.
- `set_global_on_condition`: Checks if a global variable equals zero and certain bitwise operations result in a specific value, then sets a global variable to a constant and calls another routine.
- `tiny_ori_2`: Small routine of 1 instruction that merges 5 into d0.
- `shift_and_test_global`: Shifts a global variable left and resets it to zero if a certain condition is met, followed by a series of bitwise checks and conditional decrements.
- `set_global_constant_8`: sets a global variable to a constant value and clears a memory location via a pointer.
- `reset_global_state_2`: Sets a global variable to a constant and writes zeroed or masked values to a memory location via a pointer.
- `update_pointer_table_entry_2`: Iterates through a table of structures to update pointer offsets and boundary checks, while conditionally resetting certain global flags and updating table-based memory or pointer values based on masked or bit-shifted calculations.
- `dereference_and_call_indirect`: Loads a value from a pointer-to-pointer, then uses it to perform an indirect call to a new address.
- `reset_global_to_zero`: Performs a conditional check on a global variable and then resets that global variable to zero.
- `get_masked_pointer_value`: Loads a value from a pointer-based table and returns the lower three bits of the loaded word.
- `update_table_values`: Iterates through a table to check if a value matches a certain key, then updates certain table entries with a constant or a pointer-derived value.
- `swap_and_reset_entity_state`: Swaps a pointer or ID with a new value and resets several associated state or attribute fields to zero.
- `calculate_offset_value`: Calculates a value based on the absolute value of an input, returning a specific offset or a default constant if certain conditions are met.
- `calculate_masked_address`: Performs a series of bitwise masking, bit-shifting, and addition operations to calculate a new address and store it in a global variable.
- `update_table_pointers_and_bounds`: Iterates through a list of table entries to update pointer offsets and boundary checks based on bitwise-masked or bit-shifted values.
- `rotate_and_update_table_entry`: Increments a global variable and copies a block of data from an old table index to a new table index.
- `increment_global_and_modify_byte`: Increments a global variable by a constant and, if a certain value is reached, modifies a byte at a pointer-based address.
- `apply_bitmask_to_byte`: Sets a specific bit in a byte located at an offset from a register-provided pointer.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `update_table_with_bit_shifts_2`: Calculates bit-shifted or conditional values based on flags in a global variable and writes the results into a table.
- `copy_table_data`: Copies a block of multi-byte data from one location in a table to another location in a table.
- `apply_bitmask_to_table`: Iterates through a list of pointers to perform bitwise masking and replacement on the data pointed to by those pointers.
- `update_table_with_bit_shifts`: Calculates bit-shifted values based on a global variable and writes them into a table, then updates a global variable and calls another routine.
- `update_global_with_shifted_values`: If a global variable is non-zero, it subtracts two half-words from a pointer-offset and then adds a bit-shifted value from another routine to the global variable.
- `validate_range_bounds`: Checks if a calculated value falls within a specific range relative to a register and a global variable.
- `calculate_table_offset`: calculates a masked or clamped offset or index by performing arithmetic or bitwise operations on values retrieved from a table.
- `update_global_pointer_2`: Calculates a new value for a global variable using a table-based lookup and bitwise masking, then applies a clamping or bounds-checking logic.
- `clamp_calculated_address_offset`: Calculates a new address or offset using table-based lookups and bitwise masking, then clamps the resulting value within a specific range.
- `update_global_bounds_check`: Calculates a value using a table-based lookup and bitwise masking, then checks if a secondary calculated value falls within a certain range.
- `update_global_range`: Updates a global variable using a calculation involving a pointer-based offset and a conditional check on a register value.
- `validate_and_update_global`: Calculates a value using a pointer and a global variable, checks if it falls within a certain range, and updates a global variable or calls another routine based on certain conditions.
- `update_global_range_alt`: Performs a similar calculation to R1 but uses a different initial value for the global variable check.
- `update_global_coordinate`: calculates a value by adding a register to a half-word read from a pointer-offset and stores it in a global variable.
- `decrement_global_via_loop`: Performs a loop that decrements a value until it reaches a specific constant.
- `update_global_coordinate_values`: Updates a global variable using a value retrieved from a table-based offset.
- `copy_word_values`: Copies a word from an offset of a pointer into a global variable and then stores the value of that pointer into another global variable.
- `compute_complex_value`: Performs a long sequence of comparisons and arithmetic to calculate a value, then stores a register-based value into a global variable.
- `calculate_leading_zeros`: Calculates a value based on the number of leading zeros in a bitmask and returns a bit-shifted version of a register value.
- `get_absolute_value`: Returns the absolute value of a global variable.
- `apply_conditional_shifts`: Performs a sequence of conditional bitwise shifts and additions or subtractions based on bits in a global variable.
- `clear_bit_in_memory`: Calculates a memory address based on a register and a bitmask, then clears a specific bit in a byte at that address and a byte pointed to by another register.
- `decrement_and_check_global_parity`: Decrements a global variable and checks if its least significant bit or parity has changed.
- `iterate_and_modify_global_state`: Iterates through a table, calling routines based on a sign check, and finally sets a global variable to a bitwise-inverted value from a memory location.
- `iterate_and_call_routines`: Iterates through a list of pointers, calling different routines based on whether a value at a memory location is negative.
- `check_global_xor`: Performs an XOR operation on a global variable with itself and checks if the result is negative.
- `execute_subroutine`: Calls another routine and returns without performing any further operations or modifying any registers or memory.
- `scan_and_recycle_slot_table`: Walks a table of 14-byte-or-larger records starting at a fixed base, comparing a per-record field against a global threshold and either returning early or invoking a sibling routine and overwriting the current record with the last table entry.
- `rotate_and_update_pointer`: Performs a one-bit left circular rotation on a 16-bit value and updates a global pointer or table entry with a modified address or value.
- `update_pointer_table_entry`: Updates a pointer in a table with a value from another pointer, while checking if the new pointer or a calculated offset falls within certain memory range boundaries.
- `validate_pointer_range`: Checks if a pointer or a value derived from a table entry falls within a specific memory range.
- `clear_table_entries_2`: Iterates through a table of pointers and checks if a calculated offset or range falls within certain bounds, clearing or updating memory or global flags if certain conditions are met.
- `rotate_and_store_pointer`: Rotates the lower or upper half of a 32-bit value and stores a pointer or value into a table or structure.
- `tiny_ori_3`: Small routine of 1 instruction that merges 220 into d0.
- `iterate_and_clear_table`: Iterates through a table, checking if a range or pointer falls within certain bounds, and clears certain memory locations.
- `clear_table_entries`: Iterates through a table and clears certain dwords if a pointer or range falls within certain bounds, while also modifying a global variable.
- `reset_global_state`: Sets a global variable to zero and calls routines that modify a table and a global variable.
- `initialize_global_state_2`: Calculates a table offset, iterates through a table to modify a global variable, and performs a series of writes to a global variable.
- `set_global_to_max`: Sets a global variable to a specific maximum value.
- `frag_clr_rts_2`: Short code fragment that clears a fixed memory address, then returns.
- `execute_unreachable_calls`: Calls two different routines and then enters an infinite loop.
- `calculate_absolute_sum`: calculates the sum of the absolute values of two signed short integers stored at an offset from a pointer-based base address.
- `modify_bit_and_set_flags`: toggles a bit in a byte via a pointer and sets specific bits in a subsequent byte or a global variable.
- `initialize_global_state`: Performs a series of writes to a global variable and calls another routine.
- `set_global_value`: sets a global variable to a specific constant value.
- `tiny_ori_4`: Small routine of 1 instruction that merges 216 into d0.
- `tiny_ori_5`: Small routine of 1 instruction that merges 140 into d1.
- `set_global_constants`: Sets a global variable to a specific constant value and copies a block of data from a source to a destination.
- `reset_and_call_routines`: If a global variable is non-zero, resets it to zero and calls two other routines.
- `copy_block_of_data`: Copies a fixed number of dwords from a source memory location to a destination memory location.
- `copy_and_patch_memory_block`: Compares a block of memory from two different source addresses and copies or patches bytes into a destination starting at a global variable, while also updating a counter and a pointer.
- `zero_out_memory_block`: Iteratively writes zero to a sequence of memory locations using a pointer.
- `initialize_global_values`: Sets a global variable to several constant values and calls a routine that clears a memory block.
- `patch_memory_with_loop`: Compares a block of memory against another and copies bytes from a source to a destination while updating a global variable.
- `initialize_memory_tables`: Clears a range of memory, initializes a global variable, and populates several memory locations with hardcoded or calculated values.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `scale_global_variable_2`: Multiplies a global variable by a power of two and adds a value from a register.
- `modify_global_variable`: Updates a global variable by adding itself to itself and a register, then performs a bitwise rotation and a final addition with a register.
- `update_table_values_2`: Iterates through a table, calculating a clamped offset and updating table entries with a value derived from a register and a previous table entry.
- `iterate_pointer_list`: increments a pointer through a memory-resident list until a value in a register is reached, then assigns a value from a fixed offset relative to that pointer to a global variable.
- `update_pointer_offset`: Reads a value from an offset of a pointer, then calculates a new value using another offset and stores it back into the pointer's structure.
- `add_unspecified_values`: Adds a register value to an unspecified value and stores the result in a global variable, repeating this for a second offset.
- `update_entity_bounds_and_scale_2_2`: Iterates through a table of entity pointers to check if their bounding box or scale parameters fall within a certain range, and if so, updates a global variable or pointer-based structure with a new calculated or scaled value.
- `shift_global_value`: Calls another routine and then shifts a global variable right by one bit if it is not negative.
- `shift_global_variable`: Shifts a global variable right by one bit and conditionally flips a bit in a register.
- `calculate_shifted_difference`: Calculates a difference between two registers, shifts it right, and then updates a global variable using a bit-shifted version of that difference.
- `update_global_with_difference`: Modifies a global variable and, if certain conditions are met, calculates a difference between a value in a table and a value in memory, storing the result back to memory.
- `update_global_and_table`: Modifies a global variable using bitwise operations and updates a memory location relative to a pointer with a calculated difference.
- `subtract_table_value`: Subtracts a value loaded from a table from a value in memory and stores the result back to memory, clamping the result to zero if an underflow occurs.
- `compare_word_to_global`: compares a 16-bit value to a global variable and checks if the global variable is larger or if a borrow occurs during subtraction.
- `calculate_table_difference`: Returns the difference between a value in a table at an offset and a value from another memory location.
- `subtract_constant_from_table`: Returns the result of subtracting a constant from a value located in a table at an offset.
- `set_global_byte_to_max`: calls another routine to obtain an index, then stores a maximum byte value into a table at that offset and updates a global variable with the same value.
- `set_global_and_table_byte`: Calls another routine and then writes a specific byte value to a global variable and a memory location relative to a returned pointer.
- `update_lookup_table`: Updates a small lookup table or counter-based array by decrementing or incrementing values at a specific memory offset.
- `update_string_encoded_table`: Modifies a string-encoded value in a table using a signed offset and then iterates through a secondary table or string-based list to execute a routine a certain number of times.
- `iterate_table_elements`: Calculates an offset based on a null-terminated string and executes another routine a certain number of times using a value retrieved from a table.
- `execute_conditional_routines_2`: Checks if a global variable is not equal to itself and calls a sequence of other routines if certain conditions are met.
- `update_global_pointer_3`: Updates a global variable with a masked value from a register and performs several conditional checks on the resulting value.
- `invoke_routines_repeatedly`: Calls a routine twice in succession and then calls another routine twice in succession.
- `execute_two_calls`: Calls another routine twice in succession.
- `invoke_routine_twice`: calls another routine two times in succession.
- `execute_dual_calls`: Calls another routine two times in succession.
- `calculate_scaled_bounds`: Performs a series of comparisons and arithmetic operations using values from a table, a structure in a1, and global variables to calculate a value in d1, then returns.
- `decrement_pointer_and_check_bound`: Decrements a counter or pointer-related value and checks if it has fallen below a threshold, potentially updating a value in a structure and branching to a new address or a routine call.
- `update_pointer_and_decrement_counter`: Decrements a counter or pointer-related value and updates a memory location or jumps to a new address based on a conditional check.
- `update_entity_position_and_bounds`: Decrements a counter or distance value while updating a pointer or offset, potentially triggering a jump to a new routine or updating a memory-resident table.
- `update_entity_bounds_and_scale_2`: Iterates through a list of entities to check or update pointer-related values and conditionally scales a value or a pointer-based structure member.
- `compare_array_elements`: Iterates through a table of 64-bit or 16-bit values, comparing an input value against table entries and decrementing a counter or adjusting a pointer-based index based on the comparison result.
- `update_entity_bounds_and_scale`: Iterates through a table to check or update pointer-related values, then conditionally scales a value or a pointer-based structure member by bit-shifting or subtracting a fraction of itself.
- `update_global_value_3`: Updates a global variable with the return value of another routine and conditionally modifies it based on a memory value.
- `update_global_value_2`: Updates a global variable using a bitmask and an offset, then modifies a value at a pointer offset and toggles a bit in a memory location.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `calculate_bit_rotated_value`: Checks a global variable and, if non-negative, calculates a bit-rotated value from a table and stores it in a structure member.
- `clamp_value_to_range`: Checks if a value is less than another value and returns a result where the lower or upper bound is applied, or returns a modified version of the input with the lowest bit set.
- `compare_absolute_value`: Compares the absolute value of a register against another register and returns if the latter is smaller.
- `reduce_value_by_scaled_amount`: Checks if a certain value at a pointer offset is non-zero, then either subtracts one-eighth of its value or decrements it by one if the shift-right result is zero.
- `decrement_struct_member`: Decrements a 16-bit value located at an offset within a structure by either dividing it by eight or subtracting one, provided the value is not zero.
- `decrement_value_by_bit_shift`: Checks if a value at a pointer offset is non-zero, then either subtracts its own value right-shifted by three or decrements it by one.
- `reduce_value_by_eighth`: Checks if a value is non-zero and, if it is greater than or equal to eight, subtracts one-eighth of that value from itself; otherwise, it decrements the value by one.
- `update_pointer_offsets_2`: Calculates a difference between a global variable and a pointer value, then updates a pointer offset using a clamped or absolute value.
- `update_pointer_offsets`: Calculates an absolute difference between a global variable and a value from a pointer, then adds this difference back to a pointer offset.
- `accumulate_returned_value`: adds the return value of another routine to a word stored at an offset within a pointer-based structure.
- `accumulate_value_at_offset`: reads a short value from a recursive call and adds it to a short value stored at a fixed offset within a structure or global memory area.
- `return_empty_routine_5`: The routine performs no operations and immediately returns to the caller.
- `update_global_on_condition`: Checks a global variable and, if certain conditions involving a pointer-based memory check are met, updates a global variable and calls another routine.
- `extend_global_variable_2`: Checks if a global variable is non-negative and, if so, prepends a byte to it.
- `clamp_sum_to_bounds_2`: Performs several checks and potentially adds a value to a structure member, clamping the result to a maximum or minimum bound if an overflow occurs.
- `clamp_sum_to_bounds`: Adds a value from a structure to a global variable and clamps the result or a derived value within a specific range, handling overflow and sign changes.
- `clamp_global_addition`: Performs an addition and clamps the result or a secondary calculation to specific maximum or minimum bounds.
- `clamp_and_offset_global_value`: Updates a global variable by adding a value from a memory offset, applying a clamp or offset if an overflow or certain threshold is reached, and ensuring the result or its absolute value remains within a specific range.
- `modify_memory_and_set_bits`: Clears a bit in a memory location pointed to by a register and sets certain bits in a 32-bit memory location.
- `modify_memory_bit`: Clears a specific bit in a memory location and sets a bit in a different memory location.
- `compute_complex_value_2`: Performs a series of bitwise shifts, loops, and conditional checks involving a global variable and a pointer-based memory check.
- `set_global_to_max_2`: sets a global variable to the maximum possible signed 16-bit integer value.
- `return_no_operation_4`: The routine consists of only two instructions and performs no work: its decompiled body is a bare return.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 255 into d0.
- `process_table_updates`: Iterates through a table to perform conditional additions to table entries and a global variable based on certain bitwise or arithmetic conditions.
- `shift_memory_block_2`: Shifts a small block of memory and updates a global variable and a memory location with a new value.
- `shift_memory_block`: Updates a global variable and, if a certain bit is set in a memory location, shifts a block of memory or pointers and updates a pointer-based value.
- `return_empty_routine_6`: the routine performs no operations and immediately returns.
- `copy_table_entry`: Copies a word and a longword from one memory location to another and updates a global variable.
- `update_pointer_table_2`: Checks a global variable and, if non-zero, copies a word from one memory location to another and updates a pointer-based table.
- `update_byte_if_greater`: Checks if a byte at a memory offset is non-zero and if a register value is greater than that byte, then updates the memory with the register value.
- `update_byte_with_min_logic`: Updates a byte at a memory offset if a register value is smaller, and also updates that same byte if a previously held register value is greater than the current memory content.
- `update_memory_if_smaller`: Checks a global variable and, if certain conditions involving a byte-based calculation are met, updates a byte at a pointer offset if the new value is smaller.
- `calculate_average_distance`: Calculates the average absolute difference between two values retrieved from a table and a global variable.
- `calculate_combined_distance_value`: Calculates an average absolute difference between table-derived values and a global variable, then returns a 32-bit value combining a register and that result.
- `tiny_move`: Small routine of 1 instruction that copies 128 to d0.
- `calculate_bit_reversed_value`: Iterates through a value bit-by-bit to create a bit-reversed version of that value and stores it in a local variable.
- `set_bit_in_bitset`: Calculates a bitmask or index based on a global variable and updates a bit in a bitset or a byte in a table.
- `modify_bit_in_bitset`: Calculates a bitmask or index based on a global variable and updates a bit in a bitset or a byte in a table.
- `update_bitset_and_global`: Increments a global variable and modifies a bit in a memory location or clears bits in another memory location based on certain conditions.
- `update_global_and_modify_bitset`: Increments a global variable and, if certain conditions are met, modifies a bit in a memory location and clears bits in another memory location using a calculated index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_joystick_input`: reads the state of a joystick port and compares it against a mask or constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_and_reset_global`: Iteratively checks a table-based value derived from a global variable and resets the global variable to zero if a certain condition is met.
- `shift_global_right`: Performs two right-shift operations on a global variable.
- `update_entity_state_and_timer`: Updates a global counter or timer-related value by checking bitmasks and decrementing or scaling certain memory-resident counters.
- `shift_global_left_on_condition`: Checks a bitmask against a global variable and shifts the global variable left if certain bit-level conditions or value ranges are met.
- `accumulate_value_at_offset_2`: Checks a global variable and, depending on a condition, adds a value to a memory location offset by a certain amount.
- `clamp_and_scale_weighted_value`: Calls another routine, checks if a value in register d2 is within a specific range, and if so, performs a signed multiplication using a constant and a table lookup to scale the value before adding it to a value in register a5.
- `return_without_action`: The routine as recovered by Ghidra decompilation contains no operations and immediately returns to its caller, although the disassembly reports 17 instructions, so the decompiled body does not match the instruction count and the recovered output is incomplete.
- `compute_weighted_values`: Subtracts d2 from d1 and d3, then uses bitwise operations and table lookups to multiply and shift values before returning a result in d1.
- `load_from_table_offset`: Loads a word from a table using an offset derived from the upper bits of a register.
- `traverse_table_with_clamp`: Iterates through a table using a pointer, calculating a clamped or modified value and returning a concatenated result.
- `compute_offset_and_traverse_table`: Calculates a signed offset or difference based on a pointer and a value, then uses that result to iterate through a table or pointer-indexed memory.
- `update_global_state_and_bitmask`: Checks a global variable and, if negative, modifies it and updates several bitmasks in memory using bitwise OR operations.
- `check_global_condition`: Checks if a global variable is zero and, if it is negative, calls another routine.
- `modify_pointer_bits`: Checks a global variable and either sets or clears certain bits in a word at a pointer location, or increments a value if a certain bitmask condition is met.
- `update_pointer_with_scaled_reduction`: Calculates a new value based on a bit-shifted constant and updates a pointer-based value by subtracting a scaled portion of itself or setting it to zero.
- `update_pointer_value_2`: Performs several checks on a global variable and a pointer, potentially updating a word at a pointer offset or returning a value based on a comparison.
- `reduce_pointer_value`: Checks a value at a pointer offset and, if certain conditions are met, subtracts a portion of that value from itself or sets it to zero.
- `update_global_from_pointer_diff`: Calculates the upper bits of the difference between a value at a pointer offset and a register, then stores the result in a global variable.
- `get_masked_value_minus_one`: Returns a value derived from masking a byte at a pointer offset and subtracting one.
- `update_offset_with_rotation`: Checks a bitmask in a structure and adds a rotated or bit-shifted value to a memory location based on a global variable's sign.
- `get_absolute_value_2`: returns the absolute value of a global variable.
- `return_no_operation_7`: The routine consists of a single return instruction and performs no work.
- `check_value_and_mask`: Compares a value from a pointer offset against a global variable and potentially masks a byte in memory.
- `return_immediately_stub_13`: The routine consists of a single return instruction and performs no work, so it has no effect when called.
- `search_table_and_store`: Iterates through a table using a pointer-based lookup and stores a value into a memory location if a condition is met.
- `return_table_lookup_result`: Performs a lookup via a routine and returns the result, or returns a bit-packed value if a certain condition is met.
- `count_zero_entries`: Iterates through a table and counts how many entries contain a zero value.
- `search_table_for_value_2`: Iterates backwards through a table to find a matching value, returning if a match is found or if the start of the table is reached.
- `combine_status_flags`: Combines several status flags into a single byte and returns it.
- `search_table_with_decrementing_step`: Iterates through a table using a pointer-based lookup, checking if a value matches a certain condition or if a secondary table entry matches a register value, while decrementing a loop counter by two in each iteration.
- `search_and_update_table_entry`: Iterates through a table using a decrementing index to check for matching or invalid entries, updating a global variable based on comparisons with values retrieved from a pointer-indexed table or another routine.
- `calculate_absolute_difference`: Calculates the absolute difference between a value derived from a pointer offset and a global variable.
- `calculate_range_bounds`: Performs a table-based lookup and returns a concatenated value based on comparisons or pointer-based arithmetic.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d1, then returns.
- `return_no_operation_2`: The routine consists of a single return instruction and performs no work.
- `update_pointer_referenced_value`: Checks a bitmask or sign-bit in a pointer-referenced word and conditionally updates that value or a secondary pointer-referenced field using a calculated offset or a loop-based decrementing counter.
- `modify_pointer_bits_2`: Modifies a bit in a pointer-referenced word and, if certain conditions are met, calculates and stores a new value into a pointer-referenced integer.
- `evaluate_sign_and_difference`: Checks if a global variable is non-negative and, if so, calculates the absolute difference between a value derived from a pointer offset and a global variable.
- `check_global_sign_2`: checks if a global variable is non-negative before calling another routine.
- `calculate_absolute_difference_2`: Calls a routine that calculates the absolute difference between a value derived from a pointer offset and a global variable.
- `store_register_to_memory`: Calls another routine and stores a register value into a memory location at a specific offset.
- `compute_clamped_sum`: Performs complex conditional checks and arithmetic on input registers and a global variable, potentially returning or updating values based on overflow or sign.
- `update_global_with_logic`: Performs conditional checks on a global variable and updates it using values loaded from memory or arithmetic operations.
- `update_global_bitmask`: Modifies a global variable using bitwise operations and conditional checks based on its current value.
- `increment_to_limit`: Increments a local variable by a fixed amount in a loop until it reaches a specific value.
- `update_pointer_data`: Updates a structure or table via a pointer, involving conditional writes of values from a table and arithmetic on certain offsets.
- `update_pointer_data_2`: If a value at a pointer does not match a register, it calls another routine and copies a word from a table or memory location to a structure offset.
- `set_pointer_offset_2`: Calculates a value based on a table lookup and writes it to an offset within a structure pointed to by a register.
- `compare_and_return_register`: Compares a register value with a value at a pointer and returns either the register or a value from another routine if they do not match.
- `sync_pointer_offset_value`: Checks if a pointer or value has changed, then copies a word from a location relative to the return value of a recursive call into a member of a pointer-based structure.
- `update_pointer_and_offset_table`: Updates a pointer-based structure with a new pointer or an offset-adjusted pointer, then conditionally calculates and stores a new value into a secondary offset or property based on a comparison of the newly written pointer.
- `select_and_reinitialize_buffer`: Iterates backwards through a table of structures to find a valid buffer entry based on specific bitmask criteria and a size check.
- `initialize_object_table_entry`: Iterates through a table or array, clearing or setting specific fields in a structure or object, and populating it with certain values or pointers from other memory locations or routine return values.
- `return_no_operation_6`: The routine consists of a single instruction that returns immediately, performing no work.
- `traverse_and_store_value`: Iterates backwards through a memory range, checking for a non-zero value, and stores a value into a table if a certain condition or end-of-range is met.
- `clear_memory_block_2`: Clears a block of memory starting at a fixed address by writing zero to a sequence of words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `assign_global_values`: Calls another routine and assigns the resulting or existing register values to a global variable.
- `update_global_from_routine_return`: Calls another routine and updates a global variable using the resulting values from the d1 and d6 registers.
- `initialize_complex_structure`: Calls another routine and traverses a table of addresses or offsets.
- `store_conditional_word`: Checks if a value at a pointer offset is negative, calls another routine, and if not, stores a word from a pointer to a structure offset.
- `store_multiple_words`: Checks if a value at a pointer offset is negative, calls another routine, and if not, stores two words from a pointer to a structure offset.
- `set_constant_value`: Checks if a value at a pointer offset is negative, calls another routine, and if not, stores a constant value to a structure offset.
- `populate_large_structure`: Calls a routine via a pointer, then if a register is non-negative, populates several fields of a structure with values from a pointer and a modified global variable.
- `update_table_with_shifted_value`: Calls another routine and, if certain conditions are met, writes a 32-bit value to a table, a bit-shifted word to a table, and a constant to a table.
- `update_global_and_structure`: Checks several conditions including a bitmask on a global variable, calls multiple routines, updates a global variable, and writes values to a structure or pointer-based memory.
- `check_masked_value`: Checks if the lower three bits of a value fetched from a pointer-based table or offset are less than two.
- `apply_mask_to_list`: Iterates through a list of pointers, applying bitwise masks or XOR operations to the data pointed to by those pointers.
- `rotate_global_value`: An unreachable routine that performs a bitwise rotation on a global variable if a certain condition is met.
- `apply_null_check_and_mask`: checks if a global variable is zero and, if a condition is met, masks a byte at a pointer location with a constant.
- `modify_global_and_mask_byte`: Updates a global variable using a register and masks a byte at a pointer location with a constant.
- `modify_global_and_mask`: Checks if a global variable is zero and, if a certain condition is met, masks a byte at a pointer location with a constant.
- `update_table_with_offset_3`: Updates a table by writing a value derived from a global variable and an offset.
- `update_table_with_offset_2`: Writes a value derived from a global variable and an offset into a table.
- `update_table_with_global`: If a certain bit in a global variable is set, it duplicates a 32-bit value into two different locations in a table using an offset.
- `populate_complex_structure`: Writes a global variable and several derived or offset-based values into a structure, including bit-shifted or negated values.
- `write_complex_struct_data`: Writes a global variable and several derived or offset-based values into a structure, including bit-shifted or negated values, and clears a bit in a pointer-referenced byte.
- `search_pointer_array`: Iterates backward through an array of pointers, checking certain bits in the referenced memory, and returns a value from a pointer if a condition is met.
- `nop_instruction`: the routine contains a single instruction that does not perform any operational or data-moving task.
- `return_from_subroutine`: the routine executes a single instruction that terminates the current execution flow and returns to the caller.
- `jump_to_populate_large_structure`: Jump stub: passes control straight on to populate_large_structure.
- `populate_large_structure_2`: Calls another routine and, if a register is non-negative, populates several fields of a structure using a pointer.
- `check_and_store_word`: Checks if a value at a pointer offset is negative, calls another routine if it is not, and otherwise stores a word from a pointer to a structure offset.
- `initialize_complex_structure_2`: The routine is empty and performs no operations before returning or terminating control flow.
- `store_conditional_word_alt`: Checks a pointer offset for a negative value, calls another routine, and conditionally stores a word to a structure offset.
- `store_conditional_word_v2`: Performs a conditional check on a pointer offset and stores a word from a pointer to a structure if the value is non-negative.
- `check_and_set_constant`: Checks if a value at a pointer offset is negative, calls another routine if so, and otherwise stores a constant value to a structure offset.
- `conditional_store_word`: Checks if a value at a pointer offset is negative, and if not, stores a word from a pointer to a structure offset while calling another routine.
- `set_constant_value_alt`: Checks a pointer offset for a negative value, calls another routine, and if not, stores a constant value to a structure offset.
- `check_and_store_word_2`: Checks if a value at a pointer offset is negative, calls another routine if so, and otherwise stores a word from a pointer to a structure offset.
- `store_conditional_word_v5`: Checks a pointer offset for a negative value, calls another routine, and if not, stores a word from a pointer to a structure offset.
- `initialize_complex_structure_3`: Traverses a table of addresses or offsets and calls another routine.
- `initialize_complex_structure_4`: The routine is empty and performs no operations before returning.
- `invoke_complex_structure_initialization`: Calls a routine that initializes a complex structure by traversing a table of addresses or offsets.
- `apply_mask_to_list_2`: This routine is a single-instruction wrapper that immediately calls a routine to apply bitwise masks or XOR operations to a list of pointers.
- `apply_mask_to_list_3`: This routine is a wrapper that executes the bitwise masking or XORing logic on a list of pointers.
- `apply_mask_to_list_4`: This routine is a wrapper that executes the called routine, which applies bitwise masks or XOR operations to a list of pointers.
- `populate_large_structure_3`: Calls a routine via a pointer and, if a register is non-negative, populates several fields of a structure with values from a pointer.
- `store_multiple_words_2`: Checks if a value at a pointer offset is negative and, if not, stores two words from a pointer to a structure offset.
- `populate_structure_members`: Writes a global variable and several derived or offset-based values into a structure.
- `populate_structure_from_pointer`: Calls a routine via a pointer and, if the resulting register value is non-negative, populates several fields of a structure using a pointer and a value.
- `empty_routine_7`: the routine contains no instructions and performs no operations before returning.
- `update_table_with_shifted_value_2`: Calls another routine and, if certain conditions are met, writes a 32-bit value to a table, a bit-shifted word to a table, and a constant to a location.
- `calculate_new_global_value`: Updates a global variable using a value from another routine and a register.
- `compute_bit_toggled_value`: Returns a value where a bit has been toggled based on a bitmask derived from a register.
- `call_update_linked_list_node`: Invokes a routine to update a node or entry within a linked structure by traversing a list and updating a table of pointers and metadata.
- `invoke_list_update`: Calls a routine that updates a linked list node or entry in a table of pointers and metadata.
- `return_immediately_stub_8`: The routine consists of a single instruction that returns to its caller without performing any work, touching no registers, memory, or hardware.
- `set_global_from_function`: Sets a global variable using a masked value from another routine and then calls another routine.
- `return_stack_value`: This routine returns a value from the stack.
- `update_global_with_bitwise_logic`: Performs bitwise negation and addition on a global variable before storing a new value back to it.
- `set_bit_in_table`: Calculates a target address using a bit-toggled offset and performs a bitwise OR with a constant on the value at that address.
- `calculate_global_from_mask`: Updates a global variable using a masked value from another routine and then calls another routine.
- `toggle_bit_at_offset`: Calculates a bitmask using a bit-toggled offset and XORs a single bit into a byte located before a given pointer.
- `clamp_global_value`: Calculates a new value for a global variable based on a register and then clamps it to a minimum value.
- `flip_bit_in_byte`: toggles a single bit within a byte at a memory location pointed to by a register using a bitmask calculated from other registers.
- `update_global_with_offset`: Retrieves a value from another routine, masks a bit, adds an offset, and stores it in a global variable before calling another routine.
- `flip_bit_at_offset`: performs a redundant load and then toggles a bit in a byte located at a memory address offset by one from a pointer register.
- `execute_linked_list_update`: Calls a routine that updates a linked list structure.
- `return_immediately_stub_7`: The routine consists of a single return instruction with no other operations, so it performs no work and simply returns to its caller.
- `update_global_with_different_offset`: Retrieves a value from another routine, masks a bit, adds a different offset, and stores it in a global variable before calling another routine.
- `return_empty_stack_value`: The routine returns an uninitialized value from the current stack frame without performing any operations.
- `update_global_with_third_offset`: Retrieves a value from another routine, masks a bit, adds a third offset, and stores it in a global variable before calling another routine.
- `return_stack_value_2`: The routine returns the value currently stored at the top of the stack without modifying it.
- `call_update_linked_list_node_2`: Executes another routine that updates a linked list node or entry in a table of pointers and metadata.
- `return_no_operation_3`: The routine consists of a single instruction that returns immediately, performing no work.
- `call_update_linked_list_node_3`: Invokes a routine to update a linked list node or entry in a pointer table.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_update_linked_list_node_4`: Executes another routine that performs a linked list update or node calculation.
- `return_no_op_3`: The routine consists of a single return instruction and performs no work: it reads no registers or memory, makes no library calls, touches no hardware registers, and has no callers shown in the evidence beyond being reached from outside the static flow.
- `invoke_linked_list_update`: Calls a routine to update a linked list node or entry in a pointer table.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work.
- `invoke_linked_list_update_2`: Calls a routine to update a node or entry in a linked structure and then immediately terminates or jumps to a non-instruction or invalid memory location.
- `return_no_op_6`: The routine consists of a single return instruction and performs no work, so it only returns control to its caller.
- `return_immediately_stub_16`: The routine consists of a single instruction that returns to its caller without performing any work, touching no registers, memory, library calls, or hardware.
- `call_update_linked_list_node_5`: Executes a single instruction that calls a routine to update a node or entry within a linked structure.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `update_linked_list`: Calls a routine that updates a linked list node.
- `return_immediately_stub_10`: The routine consists of a single instruction that returns to its caller, performing no computation, memory access, library call, or hardware register access.
- `frag_jsr_2`: Short code fragment that calls update_linked_list_node.
- `return_immediately_stub_3`: The routine consists of a single instruction that returns to its caller without performing any work, so it does nothing beyond returning control.
- `call_update_linked_list_node_6`: Executes a single instruction that calls a routine to update a linked list node or entry in a table of pointers and metadata.
- `return_immediately_stub_15`: The routine consists of a single return instruction and performs no work.
- `call_update_linked_list_node_7`: Executes another routine that updates a linked list node or entry.
- `return_no_op_7`: The routine consists of a single return instruction and performs no work.
- `invoke_list_update_2`: calls a routine that updates a linked list node or entry in a table of pointers and metadata.
- `return_no_op_5`: The routine consists of a single instruction that immediately returns to its caller, performing no work.
- `call_update_linked_list_node_8`: Executes a single instruction that invokes a routine to update a linked list node or entry.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `invoke_update_routine`: calls another routine that updates a linked list node or entry.
- `return_immediately_stub_6`: The routine consists of a single return instruction and performs no work: it neither reads nor writes registers, memory, or hardware, and makes no library calls.
- `invoke_update_routine_2`: Calls a routine that updates a linked list node or entry in a linked structure.
- `return_no_op_8`: The routine consists of a single return instruction and performs no work.
- `invoke_multiple_routines`: Executes a sequence of three different routine calls.
- `set_bit_and_update_list`: Sets a bit in a byte and calls a routine that updates a linked list node while storing a value into a global variable.
- `set_bit_in_byte`: sets a specific bit within a byte located at a memory address pointed to by a register.
- `update_linked_list_node`: Calculates a new node or entry in a linked structure by traversing a list and updating a table of pointers and metadata.
- `copy_memory_block_loop`: Decrements a global variable in a loop and then copies a block of words from a source address to a destination address.
- `copy_memory_block`: copies a block of words from a source address to a destination address, where the block size is determined by a global variable.
- `frag_rts_move_clr`: Short code fragment that returns, then copies 0 to d0, then copies d0 to a fixed memory address, then clears a fixed memory address, then returns.
- `traverse_lookup_table`: traverses a table of offsets or jump targets by calculating or reading relative or absolute addresses.
- `load_from_table`: loads a word from a table at a fixed offset using an index multiplied by two.
- `reset_global_and_clamp`: Sets a global variable to zero, calls another routine, and performs a clamped addition of a global variable to a value while storing a new coordinate.
- `reset_global_variable_4`: Sets a global variable to zero and calls another routine before returning.
- `update_coordinate_with_clamping_2`: Updates a target coordinate or object property by applying a scaling factor and performing clamped addition or subtraction, ensuring the resulting value stays within a valid signed range.
- `update_coordinate_with_clamping`: calculates new coordinate values using a scaling factor and stores them, while also performing clamped addition of a global variable to certain memory locations.
- `flip_buffer_bit`: Checks if a global variable or a register-provided value meets a certain threshold, then toggles the high bit of a word at a specific offset within a buffer.
- `flip_high_bit`: toggles the most significant bit of a word at a specific offset in a buffer.
- `apply_xor_to_pointer`: Calculates a difference between a pointer-referenced value and a global variable, then XORs that difference with a value at a pointer offset.
- `process_masked_value`: Performs bitwise operations on a value from a pointer, checks if a value is non-negative, and writes a value to a pointer offset or updates a pointer-based value using a bitwise XOR.
- `return_immediately_3`: This routine is empty and performs no operations before returning to another routine.
- `do_nothing_routine_2`: the routine contains instructions but performs no observable side effects or state changes before returning.
- `store_interpolated_values`: Stores a global variable into two different offsets of a structure and stores a return value from another routine into a third offset.
- `interpolate_coordinate_values`: calculates and stores an intermediate value between two points for a specific coordinate offset.
- `branch_on_positive_value`: checks if a value at a specific offset in a buffer is non-negative and calls another routine if so.
- `check_zero_flag_and_call`: Checks the zero flag and calls another routine if the flag is set.
- `modify_byte_on_match`: Checks if a value at a specific offset is non-negative, compares it against a value in a table, and if certain conditions are met, sets a bit in a byte at a calculated pointer.
- `validate_and_calculate_offset`: checks a global variable and a value in a table, then calls two other routines and calculates an absolute offset or returns an error code.
- `increment_byte_with_wrap`: Increments a byte at a specific offset in a structure and resets it if the incremented value reaches zero.
- `mask_byte_at_offset`: Performs a bitwise AND on a byte at a calculated offset using a constant mask.
- `update_global_value`: Calculates a new value using a function return and pointer-based memory access, then stores it in a global variable before masking a byte in memory.
- `update_entity_buffer_index`: Updates a global counter or index and, if certain conditions are met, copies a block of fields from a source structure or table entry to a destination structure.
- `copy_struct_member`: If a certain condition is met, copies several fields from one structure or table entry to another and increments a global counter.
- `traverse_linked_list_2`: Checks a bit in a memory location and either decrements a global variable or traverses a list backwards to update a pointer and a value.
- `modify_linked_list`: Checks a bit in a memory location and either increments a value at a pointer or traverses a list backwards to update a pointer and a value.
- `copy_structure_fields`: Copies several fields from one structure or memory location to another if a certain bit is not set.
- `initialize_global_state_and_table`: Decrements a global variable to zero, then resets or modifies certain pointers and structure constants.
- `update_global_state_2`: Checks a global variable, extracts bits from a returned value to modify a global variable, and if certain conditions or bitmasks match, updates a structure and calls another routine.
- `loop_until_threshold`: Repeatedly calls a function until the returned value is or exceeds a certain threshold, then masks a bit in a global variable.
- `mask_global_byte`: Masks a global variable with a constant and calls two other routines.
- `update_global_with_mask_2`: Masks an input word and calls another routine or returns a concatenated value based on a bit check.
- `transform_numeric_value`: Performs a loop that manipulates a numeric value using modulo and bitwise shifts, updating a local variable.
- `update_global_word`: Updates a global variable using a value from another routine and performs conditional logic to modify the lower byte.
- `modify_and_store_byte_pattern`: Checks if a value is non-negative, then applies a conditional offset and a conditional subtraction to that value before storing the result into a global variable along with a shifted version of the original value.
- `accumulate_and_shift`: Executes a loop that repeatedly updates a global variable by adding a constant and performing a left shift.
- `shift_bits_into_global`: Constructs a word from a constant and a pointer, then shifts bits from that word into a global variable.
- `return_constant_byte`: returns a constant byte value.
- `return_constant_byte_2`: Returns a constant byte value.
- `calculate_and_copy_table_data_2`: Calculates a range or offset based on bit-shifted or masked values from memory and copies a sequence of words from a source table to a destination.
- `calculate_and_copy_table_data`: Calculates a range or offset based on bit-shifted or masked values from memory and copies a sequence of words from a source table to a destination buffer.
- `set_memory_pointers`: Sets several memory pointers to offsets relative to a register and performs bitwise rotations on a byte before storing a value.
- `update_pointer_table_3`: Calculates an index using a bitwise mask and a bitwise rotation, then stores a new address into a table at that calculated offset.
- `execute_decrement_loop`: Executes a loop that decrements a counter until a value in a memory location exceeds that counter.
- `return_concatenated_value`: Returns a 32-bit value formed by concatenating a constant with a value from a memory location plus an offset.
- `initialize_memory_pointers`: Sets several memory pointers to specific values and executes a loop that copies data from a source to a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_struct_member_fields`: If a certain register or flag is zero, this routine initializes several fields within a structure or object, setting some values to zero or a constant and copying a value from a global variable or another memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_object_properties`: Initializes a structure or object by setting multiple member fields, including a pointer, a constant, and a value retrieved from a function call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_members`: Calls another routine and writes several values or masked or unmodified data into a structure or memory block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_8`: The routine contains no executable instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_modify_memory`: Executes a loop that calls another routine and performs bitwise or operations on a memory location until a counter reaches a certain value, then writes values to a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_modify_memory`: Checks a condition and either writes values to a structure or performs bitwise or operations on memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_4`: Calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_bytes_3`: Copies a byte from one memory location to another and copies a second byte from a different location to a third.
- `copy_memory_bytes_2`: Copies a byte from one memory location to another, then copies a second byte from a different location to a third.
- `initialize_memory_structure`: Initializes several memory locations with constant values and updates a structure with a calculated offset and pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_offset`: Calls another routine, calculates a new value based on a global variable and an offset, and stores the result in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_bounds`: Calculates and stores new bounding box or collision-related coordinates and dimensions into a structure or table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_position_and_type`: Calculates new coordinate or offset values and updates a structure or table with a new position, type, and a modified or masked type-identifier.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_position_and_state`: Calculates a new position or offset for an object and updates its internal state or pointer-based properties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_buffer`: Updates a global structure or buffer with a new input parameter and associated state flags.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_struct_member`: Initializes a structure or object by writing a provided parameter and several hardcoded or previously stored values into a memory block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_event_struct_and_state`: Updates a structure or object with a new parameter value and a calculated difference, while simultaneously overwriting several fields of a global structure with new or existing values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_event_struct`: Initializes a structure or object in memory by clearing certain fields, setting a pointer to a constant, and populating it with a provided parameter and a value derived from a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields_3`: Writes several values, including a parameter and a bit-shifted global, into a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_struct_and_flag`: Calculates a bit-shifted value and writes several fields to a structure before performing a bitwise OR on a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_struct_from_function`: Calls another routine and uses its return value to populate several fields in a structure, then modifies a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_range`: Calculates a value based on a global and a register, then writes a range or a modified value into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_9`: This routine contains 14 instructions and no visible calls or register-based data modifications in the provided metadata.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_10`: This routine contains 18 instructions and no visible calls or register-based data modifications in the provided metadata.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_11`: This routine contains 17 instructions and no visible calls or register-based data modifications in the provided metadata.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields_4`: Updates multiple fields within a structure or object, frequently recalculating a shared offset or index or checking a condition before writing a value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_with_offset`: Calculates a value using a global variable and an offset, then stores several calculated or register-based values into a structure or array.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_with_offset_extended`: Updates a structure with a calculated offset and register values, then calls another routine before performing further updates.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coordinate_bounds`: calculates a new offset or bound by adding a global value to a register and checks it against a threshold, then stores the result and several other values into a table or structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_state_with_offset`: Calculates an offset using a global variable and stores several values, including a constant, into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_state_minimal`: Calls another routine and stores certain register-based or stack-based values into a structure.
- `check_byte_flag`: Loops until a specific bit in a byte at an offset from a pointer is cleared.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_state_fixed_values`: Stores certain register or stack values and a constant into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_one`: executes another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a0, then returns.
- `check_byte_flag_loop`: Loops while a specific bit in a byte at an offset from a pointer remains set.
- `copy_memory_bytes_two`: Copies a byte from one memory location to another and copies a second byte from a different location to another.
- `modify_structure_fields`: Sets several memory locations to zero or constant values and updates a structure with a calculated value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_complex`: Calls another routine, stores a calculated value into a pointer, and updates a structure with a calculated offset and a complex bit-manipulated value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_position_and_bounds_2`: Calculates new coordinate or bounding box offsets and updates a structure or object in memory with new position or size data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_and_bounds`: Calculates new input or or coordinate-related values based on a bitwise-modified input or button state and updates a structure or table with these new bounds and associated metadata.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_bounds_2`: Calculates a new bounding box or collision area based on a distance or offset and updates a structure or object with the new coordinates and dimensions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_and_buffer`: Updates a structure or object with a new input value and calculates a delta or offset relative to a previous state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_state_and_position`: Updates a structure or object in memory with a new position and certain flags, while calculating a distance-related value or offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_event_structure`: Initializes a structure or object in memory by setting certain fields to constant values, calculating offsets or distances, and copying values from a source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields_5`: Updates several fields within a structure or object, calculating or retrieving new values from a table or another routine and storing them into specific offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_buffer_2`: Updates a structure or object in memory with new input or state values, calculating relative offsets or differences and storing them in a specific memory block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_2`: Updates a structure or object in memory with new position or attribute values, potentially adjusting or offsetting these values based on a stored or passed parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_and_flags`: Updates a global structure or object with new input or coordinate data, while simultaneously modifying bitmasks in a global flag or status variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_and_flags`: Updates a structure or object in memory with new coordinate or state data, while simultaneously performing bitwise OR operations on global flags or status registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_position_and_flags`: Calculates new coordinate or offset values using a wrap-around check against a constant, then updates a structure or object with these new coordinates and certain bitwise flags.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_bounds_and_state`: Updates a structure or object's bounding box or position-related fields using a calculated or offset value, while simultaneously updating bitmasks in a global table and clearing a pointer or structure head.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_and_modify_memory`: Calculates a value based on a global variable and a register, then stores several values into a structure or table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_structure_fields_2`: Writes a calculated value and several register-based values into a structure or table, and finally clears the first word of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_structure_values`: Updates a structure or table with a calculated value and certain register-based values, then clears the first word of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_member_with_offset`: Updates a structure or object by writing a calculated or passed value to multiple offset-based memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state_2`: Updates a global structure or object with a new input value and a calculated difference or a fixed value, while also updating a secondary pointer-based or or-masked value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_input_value`: Writes a value and a fixed constant to a global structure, and if a certain condition is met, calculates and stores a difference or a new value into that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_input_difference`: Calculates the difference between a register and a value from a table and stores the result into a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_difference`: If a condition is met, calculates the difference between a register and a value from a table, then stores this difference and a short value into a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_table`: Calls another routine and stores a register value into a global structure or table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_pointer`: Stores a register value into a memory location pointed to by another register.
- `compute_complex_offset`: Performs bitwise operations and arithmetic on values from a table to calculate and store a new offset or address into a global variable.
- `update_global_offset`: Calculates a new offset or address using bitwise operations and a table, then updates a global variable and a structure.
- `update_memory_structure_2`: Checks if a value is non-zero and not equal to a certain value, then copies a sequence of words and a byte from a source pointer to a destination offset in a structure while masking a value in a table.
- `update_memory_structure`: If a comparison or equality check passes, it copies a sequence of words and a byte from a source pointer to a destination offset in a structure.
- `mask_table_entry`: Masks a bit in a table entry and updates a memory location with a bitwise or operation.
- `iterate_and_mask_data`: Iterates a certain number of times, masking a value in a table and updating a global structure with a previously stored value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_mask_and_store`: Loops a certain number of times, masking a value in a table and storing a register value into a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_register_zero`: checks if a certain dword-sized register or memory location is zero and returns if so.
- `modify_struct_member_via_offset`: Calculates a pointer to a structure member using a base address and a scaled index from a table, then clears a bit in a word at that location.
- `update_struct_fields_6`: Stores a value from a pointer and a global variable into a structure, and conditionally updates a field with a subtraction result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_update_value`: Checks if a register value exceeds a certain threshold and, if so, calculates a difference and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_with_difference`: If a register value is greater than a threshold, it calculates a difference and stores it and a register value into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_indirect`: executes another routine without any further logic or side effects.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: the routine is empty and performs no action.
- `store_pointer_data`: Stores a value from a pointer and a global variable into a structure, and also stores a value from another pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forward_to_dispatch`: The routine performs no work of its own: it makes a single call to an unnamed target and then returns, so it acts as a pass-through stub.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_4`: This routine is empty and performs no operations before returning to the caller.
- `loop_through_data`: Iterates through a data source to copy values into a structure, including conditional logic and clearing a field with a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_memory_bits`: Decrements a counter and performs bitwise OR operations on two memory locations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_blocks`: If a counter is zero, it stores a bitwise NOT of a register value into a global variable and then copies data from a source pointer and a fixed address into a destination buffer using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_array_elements`: Executes a loop that copies a 32-bit value and a 32-bit register value into a destination buffer while incrementing a source pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_counter_zero`: Checks if a register contains zero and returns if so.
- `update_struct_member_range`: Calculates a range of indices or pointers and iterates through a table to modify or clear bits in a structure member.
- `find_limit_index`: Executes a loop that decrements a value by a fixed amount until a value in a global variable exceeds a calculated limit.
- `copy_global_bytes`: copies two bytes from one location in RAM to another location in RAM.
- `render_blt_sprites`: Configures and executes a series of bitplane-based block transfers or sprite blits using a source structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state_2`: Updates a global structure or object with new or existing joystick button and direction state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_properties`: Updates a series of fields within a structure or object, involving bitwise OR operations and conditional writes to certain offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_complex_2`: Updates several fields in a global structure using values from a pointer and a function return, including conditional updates based on a register value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_simple`: Updates certain fields in a global structure with values from a pointer and registers, including a conditional update.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_minimal`: Updates certain fields in a global structure using values from a pointer and a function return, including a conditional update.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields_7`: Updates multiple fields within a structure or array using a base pointer and an offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_with_function`: Updates certain fields in a global structure using a function return and a conditional update from a pointer offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_structure_data`: Writes values from a pointer and registers into a global structure, including a conditional write if a register is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_conditional`: Performs a conditional update to a global structure using a value from a pointer offset and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_conditional_data`: Writes a value from a pointer offset and a register into a global structure if a certain condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_n`: calls another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub_14`: The routine consists of a single return instruction and performs no work.
- `copy_list_elements`: Iterates through a list of a fixed size to copy values from a source pointer to a destination structure, then performs a final write to that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_update_structure`: Executes another routine while decrementing a counter, then updates a destination structure with a value from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_conditionally`: Checks if a register is zero, then either updates a destination structure with a value from a pointer or performs a bitwise or operation on a pointer's target.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: Executes another routine and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_empty`: The routine is empty and returns.
- `copy_memory_bytes`: Copies a byte from one memory location to another, then copies another byte from a different memory location to a third location.
- `update_struct_fields_and_call_subroutines`: Updates several fields within a structure or object using values from a pointer, then executes a sequence of subroutines that may or may not be conditional based on a previously checked value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_state`: Updates a global structure or object with new or existing joystick button and direction state, or resets certain flags if a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields`: Updates a structure or object by writing values from registers and memory-to-memory or memory-to-register operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_player_state_attributes`: Updates a global structure or object by copying or masking values from a source table or pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state`: Updates a structure or object or a list of objects by writing or masking certain bitflags and values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_state_with_input`: Updates a global structure with values from a pointer, potentially branching to another routine to update certain fields before performing bitwise or operations on a pointer-based value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_structure_fields`: Writes a value from a pointer and a register value into a global structure at specific offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_structure_from_pointer`: Updates a global structure with a value from a pointer, with a conditional branch to another routine that updates a different offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_structure_conditionally`: Conditionally writes a value from a pointer and a register to a global structure, then writes a value from a pointer and a register to another offset.
- `clear_memory_block`: Iteratively clears a block of memory by setting words to zero using a pointer and a loop counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_from_routine`: Calls another routine and then writes a value from a pointer and a register to a global structure.
- `clear_buffer_elements`: clears a sequence of 32 words in a memory buffer by setting each word to zero using a pointer-incrementing loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_struct_fields_2`: copies a longword from a pointer-referenced memory location to multiple offsets within a structure and writes a constant value to a field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_2`: This routine executes another routine and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `clear_memory_table`: clears a table of thirty-two words by storing zero through a pointer.
- `update_linked_list_nodes`: Calculates a pointer or index into a memory-mapped table and updates a linked list or tree structure by masking and inserting a value into a node.
- `return_register_value`: returns the value held in a data register.
- `return_no_operation_5`: The routine consists of a single return instruction with no other operations, so it performs no work and returns immediately.
- `update_memory_loop`: Updates a global variable with an offset and writes a modified register value to a memory location, then executes a loop that calculates and stores new values into a table using a pointer.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_global_and_memory`: checks if a register value minus a constant is within a certain range, then updates a global variable with an offset, writes a bit-shifted register value to a memory location, and stores a register value at an offset relative to the routine's own address.
- `update_pointer_and_store_value`: Checks if a value in a register minus a constant is within a certain range, then updates a global pointer and writes a bit-shifted register value and a register value to memory locations relative to that pointer.
- `update_memory_location`: updates a global variable with an offset and writes a modified register value to a memory location.
- `execute_recursive_swap`: A recursive routine that swaps several global memory locations and then executes a loop that increments a pointer-based value until a certain threshold is reached.
- `set_global_values`: overwrites a global variable multiple times with various constant or register-derived values.
- `check_and_clear_bits`: Compares a masked value from a global variable against another, and if they do not match, clears a specific bit in a memory location based on a bit-shift or rotation.
- `rotate_and_clear_bit_wrapper`: Performs a bitwise left rotation on a word at a specific offset and clears the least significant bit of a byte at a different offset.
- `rotate_and_clear_bit`: Performs a bitwise rotation on a value stored in a memory location and clears a bit in a byte at a memory location.
- `apply_scaled_offset_loop`: Performs a loop that updates a global variable and writes a bit-rotated value from a table into a memory location.
- `update_table_with_rotated_values`: Iteratively updates a table or memory area by calculating an index from a value, retrieving a value from a secondary table, and storing a bit-rotated and XORed version of that value back into the original memory location.
- `apply_scaled_offset`: updates a global variable using a combination of a register value, a bit-rotated version of that register, and conditional range checks.
- `update_global_with_rotation`: Updates a global variable using a combination of a register value and a bit-rotated version of that register, including conditional range checks.
- `update_global_with_rotation_alt`: Updates a global variable using a bit-rotated version of a register and conditional range checks, including logic to clamp a value to a certain constant.
- `set_global_byte`: sets a global variable to a certain constant value.
- `set_global_byte_2`: sets a global variable to a fixed maximum value.
- `execute_conditional_subroutines`: Executes several subroutines based on whether certain registers or global values are non-zero, and finally sets a global variable to a constant value.
- `mask_and_store_globals`: Clears the lower bits of a global variable and then calls another routine.
- `swap_function_pointers`: Loads several global variables into registers and swaps their contents with other global variables.
- `update_input_state`: Checks a bit in a global variable and updates a byte in a table or memory location based on the inverted and shifted value of that variable or a returned value from another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_input_table`: Updates a table or memory location with a boolean-derived value using a bit-shifted version of a value returned from another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_function_table`: Updates a global variable and writes a value to a table based on a bitmask check.
- `update_global_state_flags`: Checks specific bits in a global variable to determine whether to perform a series of memory writes to a table or trigger a sequence of other routine calls.
- `update_function_pointers_and_state`: Checks a bitmask in a global variable to determine whether to overwrite a sequence of function pointers in a table and then calls a series of unrelated subroutines.
- `update_system_state_flags`: Checks a bitmask in a global variable to conditionally update a table of pointers, reset a global variable, and execute a sequence of secondary subroutines.
- `modify_bit_and_set_byte`: Performs bitwise XOR and bitwise clear operations on a memory location using a bitmask derived from a register and a constant, while also performing several redundant memory writes.
- `check_global_bitmask`: Checks a bitmask in a global variable and conditionally resets a global variable.
- `check_interrupt_mask`: Checks if certain bits are set in a global variable and a hardware register, then calls another routine if they are.
- `update_pointer_table_iteratively`: Checks a bitmask in a global variable and, if a bit is set, iterates through a pointer table to update a pointer, increment a counter, and store a value from the table into a destination buffer.
- `traverse_pointer_table`: Checks a bitmask in a global variable and, if certain conditions are met, updates a pointer to traverse a table and stores a value from that table into a memory location.
- `update_pointer_table`: Checks a bitmask in a global variable and, if a bit is set, updates a pointer to a table with a value from that table.
- `reset_global_variable`: Sets a global variable to a specific value and then immediately resets it to zero.
- `decrement_and_lookup_table`: Decrements a global variable if it is positive and then updates that global variable with a value loaded from a table using the variable's previous value as an index.
- `update_interval_table`: Updates a table of timing or interval values by searching for a valid entry and recalculating or storing new timing parameters.
- `tiny_ori_6`: Small routine of 1 instruction that merges 62 into d2.
- `set_global_constant`: Sets a global variable to a constant value and returns a value derived from a register.
- `set_global_constant_to_fixed_value`: Sets a global variable to a fixed constant and returns a value composed of a bitwise OR between a register and a constant mask.
- `initialize_global_constants`: Sets three global variables to constant values and returns a value derived from a register.
- `compare_global_value`: Performs a loop that continues while a value returned from another routine is less than a global variable.
- `set_multiple_globals`: Sets a global variable to a constant value and then sets it again to another constant value.
- `initialize_table_values`: Checks a global pointer, and if it is non-zero, populates a table with a sequence of values using a loop.
- `do_nothing_2`: the routine contains instructions but performs no state-changing or computational operations, effectively acting as a no-op or a return-only stub.
- `configure_hardware_registers`: Updates a global variable using a table lookup and writes certain values to the DMA and interrupt enable hardware registers.
- `copy_and_expand_memory_blocks`: Performs a series of block copies from a source to a destination, followed by a conditional reverse-order copy or expansion based on values found in a table.
- `swap_memory_blocks`: Performs multiple loops that swap or move memory contents between a table and a global variable or a fixed address.
- `swap_array_elements`: Iterates through an array and swaps certain memory contents if a condition is met, then stores a value from another routine into a global or memory location.
- `execute_and_return_result`: Calls another routine and assigns its return value to a global variable.
- `set_memory_to_zero`: Calls two routines, sets a memory location to zero, sets a global variable to zero, and performs a series of bitwise OR operations on a memory location.
- `sum_array_elements`: Iterates through an array of shorts, adding each value to a running total stored in a register.
- `clear_memory_range`: Sets a range of memory or a global variable or a table or a pointer to zero using a loop.
- `traverse_string_and_update_global`: Iterates through a null-terminated string, updating a global variable with the length or pointer, and calls another routine.
- `update_global_from_table`: Calculates an index using a value from a memory location and a bitmask, then updates a global variable with a value fetched from a table at that index.
- `initialize_memory_structures`: Checks if a global variable is zero, and if so, initializes several memory locations with specific constants and populates pointers in a structure using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_pointer_list`: Iterates through a sequence of memory addresses, storing the same pointer into two different locations in a structure, and terminates when a function returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_counters`: Iterates through a bitmask or a table of bitmasks to increment a series of counters located at offsets from a base pointer.
- `filter_value_and_branch`: checks if a value is neither equal to a specific constant nor greater than or equal to another constant, and if so, calls another routine.
- `branch_on_value`: Checks if a register value is not equal to certain constants and calls another routine if the condition is met.
- `apply_bit_rotation`: Performs a bitwise rotation or bit-masking operation on a block of memory using a calculated bit-shift amount.
- `apply_bitmask_to_memory`: Checks a global variable and, if certain conditions are met, applies a bitmask to a value and stores it into a destination pointer.
- `call_with_calculated_parameter`: Checks if certain registers contain non-zero or certain-range values, then calls another routine using a parameter derived from a register and a constant.
- `process_memory_block`: If a register or variable is non-zero, it copies words from a source pointer to a destination and calculates a bitwise-inverted bitmask or value into a structure.
- `call_and_return_register`: Calls another routine and returns the current value of a register.
- `copy_to_global_buffer`: Copies several words from a fixed memory address into a buffer starting at a global variable offset.
- `copy_from_global_buffer`: Copies several words from a buffer starting at a global variable offset back to a fixed memory address.
- `loop_until_zero`: Repeatedly calls another routine and checks if its return value is zero.
- `update_bit_masks`: Modifies specific bits within several global memory locations using bitwise AND and OR operations with a provided register value.
- `execute_no_op_sequence`: performs a sequence of one hundred and eighty instructions that result in no state change or side effects.
- `recurse_if_nonzero`: If a global variable is not zero, the routine calls another routine and returns a concatenated value from registers.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, so it has no effect when called.
- `do_nothing_3`: the routine is empty and performs no operations.
- `update_hardware_and_memory`: Updates hardware registers including DMA and CIA, and performs a series of memory copies, table population, and bit-shifting operations.
- `reinitialize_display_and_coplist`: Initializes or resets certain display hardware registers, including bitplane or copper-related registers, and populates a copper list or display-related table with a sequence of instructions or or hardware-specific values.
- `update_hardware_registers`: Updates CIA or video registers and modifies a global variable with a constant value.
- `copy_and_populate_table`: Copies a range of memory, and if a certain value is found, populates a large memory block with a sequence of incrementing values.
- `initialize_array_with_stride`: Fills a memory range starting from a global pointer with an incrementing value.
- `traverse_linked_list`: Traverses a list of pointers, storing a calculated address into a table, and returns a count or a constant.
- `shift_and_store_bytes`: Iteratively shifts bits in a byte and stores it in a memory location, then copies data from a source to a destination using an offset.
- `copy_indirect_values`: Reads values from a source table using an offset and writes them to a destination table.
- `copy_word_with_offset`: Copies a word from an offset or pointer to another memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_display_interrupts_and_fill`: Builds a stack-resident table of pointers, adds a fixed long constant to rows of memory, copies a block of longs into shared memory, writes display and interrupt-enable registers, clears sprite and colour areas, and returns a constant; it loops forever if one of its calls fails.
- `unresolved_routine_17_instr`: A 17-instruction routine called from one place; the decompiler timed out, so no further behaviour is available.
- `spin_forever_unless_flag`: Returns when the zero flag is set; otherwise it spins in an endless loop.
- `spin_forever_loop`: Loops forever with no other work in the decoded body.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialise_record_and_ramp_table`: Clears several record fields, fills a 16-byte table with values that step by 8000 from a source long, copies the stepped value into four record slots, and sets two more fields.
- `copy_global_words_strided`: Copies 16 words from the shared global into it at a stride of two words, overwriting part of the block, and returns the input register unchanged.
- `set_flag_one`: Sets a shared flag word to one and returns.
- `advance_step_counter_on_timer`: Clears a flag and resets the step counter when the flag is set, then decrements a timer; when the timer expires it reloads it and, while the step is below 16, advances the step and calls a routine.
- `update_nibble_grid_flags`: Walks a 16-row grid of packed nibble values, adding a fixed bit pattern to rows where a mask comparison shows a change, then calls a routine.
- `wait_until_routine_reports_one`: Stores two values derived from a constant, makes a call, then repeatedly calls a routine until it returns one.
- `check_status_bits_and_code`: Returns zero when two status bits in the shared global are set and a stored value is neither of two codes; otherwise it returns minus one.
- `clear_big_table_then_wait`: Clears a 2048-long table, copies a few words into place, calls a routine, then calls it repeatedly until it returns one.
- `start_cia_timer_interrupt`: Saves the current timer state, then programs the CIA-A timer control and interrupt registers and sets the timer interrupt enable bit when a status flag is set.
- `restore_cia_timer_state`: When a stored flag is negative, writes the previously saved values back into the CIA-A timer control registers and sets the timer interrupt enable bit again.
- `poll_status_bit_sequence`: When a flag bit is set, writes a fixed four-value sequence to working variables, waits until a status bit is set, then clears a bit in the same variable.
- `handle_keyboard_serial_handshake`: When a flag bit is set, writes a fixed four-value sequence, waits until a status bit is set and clears a bit, while the facts show the keyboard serial register and timer registers in use.
- `tiny_ori_or_ori`: Small routine of 3 instructions that merges 7 into d0, then merges d0 into d0, then merges 0 into d7.
- `clear_state_after_helper`: Calls an unnamed routine and then zeroes one state word.
- `decrement_state_and_reload`: Subtracts two from a state word, and if the previous value was below two, reloads it from a value derived from another variable.
- `check_bounds_no_effect`: Compares two register arguments against fixed limits, but both branches return without changing anything, so the routine has no effect.
- `advance_channel_sequence_state`: Counts down a per-channel timer, and when it expires reloads several per-channel fields from a table.
- `update_channel_sequence_state`: Variant of the channel sequence routine that performs the same countdown, reload and byte-stream steps but returns no value.
- `blend_masked_bitplane_rows`: Merges a source block into a destination across four bitplanes, using a mask chosen from a table by one argument.
- `fill_block_with_ones`: Writes the value 255 to 1000 consecutive bytes starting at a fixed location, looping until the counter wraps.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_unresolved_helper`: Decompiler output is unavailable, so only the facts are known: an 8-instruction routine that calls one unnamed routine and is executed in the reference run.
- `forward_to_single_helper`: Decompiler output is unavailable; the routine is 3 instructions long, is called by a small return stub, and calls one unnamed routine.
- `call_helpers_using_text`: Decompiler output is unavailable; the routine is 24 instructions long, refers to a short text string, and calls four unnamed routines.
- `read_data_in_sectors`: Splits a requested byte count into 512-byte blocks and calls an unnamed routine for each block.
- `remap_status_code`: checks the return value of another routine and remaps certain error or status codes to new values, specifically converting certain codes to one and two while leaving others unchanged.
- `map_status_code_wrapper`: Calls an unnamed routine and converts return code 28 to 1 and return code 29 to 2, leaving other codes unchanged.
- `map_status_via_jump`: Calls another routine via a jump table and remaps certain return codes, specifically changing certain error codes to one or two.
- `map_status_code_via_jump`: Calls an unnamed routine through a jump-table entry and converts return code 28 to 1 and return code 29 to 2, leaving other codes unchanged.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `handle_vertical_blank_interrupt`: Increments two counters, calls a stored handler when one of them is nonzero, and copies the vertical blank request bit from the interrupt request register into a variable.
- `relay_call_to_routine`: Calls a single unnamed routine and returns, passing control straight through.
- `map_status_via_jump_2`: Calls another routine using a jump table that remaps certain error or return codes.
- `call_stored_pointer_routine`: Calls the routine whose address is held in a global variable.
- `execute_infinite_loop`: The routine executes an infinite loop without performing any operations or modifying any registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub_4`: The routine consists of a single instruction that returns to its caller without performing any work, so it has no effect on registers, memory, or hardware.
- `return_immediately_stub_11`: The routine consists of a single instruction that returns to its caller without performing any work, so it does no computation, writes no hardware registers, and makes no library calls.
- `return_immediately_stub_5`: The routine consists of a single instruction that returns to its caller without performing any work.
- `return_no_op_2`: The routine consists of two instructions and does nothing but return to its caller.
