# Game 230 (AdventuresOfRobinHood v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00230). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1000726 top-level bytes in 129 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 126 of 126 units identical (bytes and relocations), 300 raw-word lines |
| Functions named and described | pass, 457 of 457 named (0 author symbols), 457 of 457 described (46 by rule, 374 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5037 of 5037 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 16256 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7054 of 7054 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22306 |
| code | 38086 |
| data | 833043 |
| padding | 27 |
| unused | 107264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AdventuresOfRobinHood.slave | 1 | 168 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5032 | 239 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 172 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 124 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 23 |
| Disk.0 bytes 8960-16128 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.0 bytes 30464-30720 (code never loaded by a reference run) | 1 | 9 | 1 |
| Disk.0 bytes 40448-40704 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 22784-31360 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: raw disk image (43461 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.1: raw disk image (878080 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.2: raw disk image (31360 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4229 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 149 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 150 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: bytes 8960-16128 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: bytes 30464-30720 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: bytes 40448-40704 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.2: bytes 22784-31360 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/AdventuresOfRobinHood.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (53 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: 3 logged loads cover 7680 bytes of the image
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: 92 track-loaded chunks found in recovered runtime segments and classified through them
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.53)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: chunks 21-21 (bytes 5376-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.90)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: chunks 26-33 (bytes 6656-8704) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.06)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: chunks 133-133 (bytes 34048-34304) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.54)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.0: chunks 160-169 (bytes 40960-43461) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.16)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.1: chunks 588-600 (bytes 150528-153856) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.67)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.1: chunks 1053-1111 (bytes 269568-284672) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.16)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.1: chunks 1127-1130 (bytes 288512-289536) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.2: 1 logged loads cover 8576 bytes of the image
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood/Disk.2: chunks 0-86 (bytes 0-22272) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.93)
- 3 never-loaded region(s), 16000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 29 packed track region(s), 758016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### AdventuresOfRobinHood.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_instructions_and_jump`: Calls two library functions through a pointer table and compares the second result with a fixed value.
- `call_routine_then_table_entry`: Calls an unnamed routine, then calls a function through a pointer at a fixed offset of an address register, and returns the incoming data register unchanged.
- `scale_low_words_by_constant`: Multiplies the low word of each incoming data register by 6272 and returns both products together as a 64-bit pair.
- `call_routine_then_table_entry_2`: Calls an unnamed routine, then calls a function through a pointer at a different fixed offset of an address register, and returns the incoming data register unchanged.
- `set_global_flag_to_one`: Stores the value 1 in a global flag and returns.
- `set_global_value_to_two`: Stores the value 2 in a global and returns.
- `halt_in_infinite_loop`: Contains an empty loop with no exit, so execution never returns from this routine.
- `patch_code_when_marker_matches`: Calls an unnamed routine and returns its result.
- `unpack_lz_stream_backward`: Decodes a compressed stream by reading flag bits and copying either a run of earlier output or a literal run, writing backwards from the end of the output buffer.
- `stub_no_effect_return`: Tests a bit buffer value and returns on both paths, so it has no effect.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_bits_from_backward_stream`: Shifts a requested number of bits out of a bit buffer held in the data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_byte_if_not_equal`: If the condition in the branch is not met, it sets d5 to 1 and returns; otherwise it returns immediately.
- `compare_and_set_byte_2`: Compares d5 and d6; if they are equal or if d5 is higher than d6, it returns, otherwise it sets d5 to 1 and returns.
- `compare_and_set_byte`: Compares a byte in d5 with a byte in d6; if d6 is not less than d5, it sets d5 to 1, otherwise it clears d5 and returns.
- `load_and_branch`: Loads a byte from a fixed memory location into d5, increments a pointer in a5 and loads a word from that pointer into d6, then performs an unconditional branch.
- `call_and_update_byte`: Loads a pointer into a2, calls another routine, stores the resulting byte from d5 into a fixed memory location, and then sets d5 to 0 or 1 based on whether d5 is zero.
- `increment_and_check_limit`: Loads a word from a pointer in a5, increments the next word in the list, and if the loaded word is greater than or equal to the incremented value, it clears the next word and sets d5 to 1, otherwise it clears d5 and increments the pointer by 2.
- `compare_with_constant`: Loads a word from a pointer in a5 into d5, compares it to a constant, and if they are not equal, sets d5 to 1, otherwise it returns.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d5, then returns.
- `check_status_byte`: Compares a byte at a fixed memory location with 1; if they are not equal, it sets d5 to 1, otherwise it clears d5 and returns.
- `search_table_for_value`: Calls another routine and checks a value at a pointer in a5; if that value is 3000, it loops through a table to find a matching byte, otherwise it checks if a byte at an offset of 510 from a value in d5 matches a specific byte, and updates a global variable with the value in d5 if a match is found.
- `validate_range_bounds`: Loads a pointer into a2, calculates an offset, loads four bytes from that offset into d3, d6, d4, and d7, loads two bytes from nearby memory into d2 and d5, and sets d5 to 1 if d2 is within the ranges defined by the pairs in d3/d6 and d4/d7, otherwise it clears d5.
- `check_offset_ranges`: Calls another routine, loads two bytes from offsets of 27435 and 27427 from a value in d5, calculates an offset from a pointer in a5, and checks if those two bytes fall within two specific ranges stored at that offset.
- `compare_with_global`: Calls another routine, compares a word in d5 with a value at a fixed memory location, and sets d5 to 1 if they are not equal, otherwise it clears d5 and returns.
- `search_table_index`: Calculates an offset from a global variable and searches through a table for a value, updating the global variable with the resulting index.
- `compare_table_values`: Compares a value from a pointer against entries in a table using various conditional logic and loops, potentially updating a global variable with a found index.
- `find_matching_entry`: Iterates through a table to find an entry that matches specific criteria involving a byte value and a character comparison, then updates a global variable.
- `check_byte_equality`: Compares a byte from a pointer with a byte at an offset of a global variable and returns one if a specific byte at another offset is not sixteen.
- `check_byte_equality_strict`: Compares a byte from a pointer with a byte at an offset of a global variable and returns one only if the byte at the other offset is sixteen.
- `combine_status_checks`: Calls two different routines and returns the result of the second routine if the first routine does not return a specific value and certain registers are not negative one.
- `verify_pointer_value`: Calls a routine to calculate a pointer and checks if the word at a specific offset of that pointer is negative six thousand five hundred fifty-three.
- `update_global_on_match`: Compares a byte from a pointer with a byte from a global variable and, if they match, stores a value into a global variable.
- `compare_word_data`: Compares a word from a pointer against a word located at the address stored in a global variable and returns one if they are identical.
- `check_pointer_byte`: Compares a byte from a pointer with a byte at the address stored in a global variable and returns one if they match.
- `check_status_and_byte`: Loads a word from a pointer in a5, checks if a global variable is 1 or 3, and if so compares a byte from the pointer with a constant to set d5 to 1 or 0.
- `verify_status_two`: Loads a word from a pointer in a5, checks if a global variable is 2, and if so compares a byte from the pointer with a constant to set d5 to 1 or 0.
- `process_table_loop`: Loads a word from a5 into d5 and a word from a global variable into d6, then loops through a table using d6 as a counter, comparing bytes from the table to d5 and incrementing a value in a2 before performing a jump through a pointer.
- `calculate_byte_difference`: Calls another routine, calculates the difference between a byte in d2 and a global variable, and checks if that difference is between 0 and 7.
- `compare_d5_after_calls`: Calls another routine twice, then compares the resulting value in d5 with its previous value in a2 to set d5 to 1 or 0.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies (a5)+ to d5, then copies 1 to d5, then returns.
- `check_flag_offset_two`: Checks if a byte at offset 2 of a structure pointed to by a4 is 1, then sets d5 to 1 or 0 accordingly.
- `compare_pointer_value`: Calls another routine, loads a word from a5 into d5, and compares that byte with a byte at an offset from a4 to set d5 to 1 or 0.
- `mask_and_return_word`: Calls another routine, performs a bitwise AND between a byte from a4 and a byte from a pointer in a5, and returns the result as a word.
- `compare_byte_threshold`: Compares a byte from a pointer in a5 with a byte from a structure in a4, setting d5 to 1 or 0 based on the unsigned comparison.
- `compare_offset_one`: Compares a byte from a pointer in a5 with a byte at offset 1 of a structure in a4, setting d5 to 1 or 0 based on the result.
- `check_constant_match`: Compares two values from a constant table against negative one; if both match, it sets a pointer in a6 and returns zero in d5, otherwise it checks a byte at an offset of a pointer in a4 and returns either zero or one in d5.
- `check_status_byte_2`: Checks if a byte at offset 3 of a structure pointed to by a4 is equal to one; if so, it returns one in d5, otherwise it returns zero in d5.
- `compare_table_value`: Loads a pointer and a word from a table using a5, then compares a byte at an offset of 7118 from that pointer with the word in d5, returning one if they match and zero otherwise.
- `check_double_negative`: Checks if two consecutive memory locations at offsets 8879 and 8880 contain negative one; if they do, it returns one in d5, otherwise it returns zero in d5.
- `check_specific_values`: Checks if a byte at offset 11016 is one or if a byte at offset 8879 is negative one; if either condition is met, it returns one in d5, otherwise it returns zero in d5.
- `validate_sequence_and_update`: Compares a byte at a pointer in a5 to six; if it matches, it validates a sequence of three bytes from a table against three constants, then clears a byte at a global variable and potentially writes a value to an offset of a6 and returns one in d5.
- `update_calculated_offsets`: Calls two routines and then calculates and stores two 16-bit values into a table at offsets determined by d5 and d2.
- `store_register_values`: Calls a routine and then stores the byte from d2, the byte from d5, and the constant negative one into a structure pointed to by a4 at offsets 734, 710, and 750 respectively.
- `copy_word_to_hardware`: Moves a word from a pointer in a5 to d6, loads a hardware address into a2, and calls another routine.
- `copy_word_to_offset_hardware`: Moves a word from a pointer in a5 to d6, loads a hardware address into a2, increments a2 by 2, and calls another routine.
- `call_subroutines`: Calls two different subroutines and then returns.
- `perform_arithmetic_operation_2`: Calls another routine, loads a pointer from a5 into a2, loads a word from a5 into d6, and branches to another routine.
- `perform_arithmetic_operation`: Uses a byte from a2 and a byte from d6 to perform an arithmetic operation on a byte at a pointer in a4, supporting assignment, addition, subtraction, multiplication, and division.
- `execute_subroutines_and_store_byte`: Calls two different routines and then stores a byte from a register into a memory location pointed to by a value in a6.
- `load_word_and_branch`: Loads a word from the location pointed to by a5 into d5, increments a5, and branches to another routine.
- `clear_byte_at_address`: Clears a byte at a specific memory address and returns.
- `increment_global_byte_2`: Increments a byte value at a fixed offset in memory by one and returns.
- `perform_arithmetic_operation_with_pointer`: Calls another routine, loads a pointer from a5 into a2, pushes a4 onto the stack, calls another routine, loads a byte from the pointer in a4 into d6, restores a4 from the stack, and branches to another routine.
- `call_routines_conditionally`: Calls two routines, checks a global variable, and then calls one of two different routines based on whether that variable is zero, before finally calling another routine with a value incremented by 2.
- `call_and_branch`: Calls another routine, clears d5, and branches to another routine.
- `clear_byte_at_offset_13888`: Clears a single byte in a global variable located at offset 13888 and returns.
- `store_word_from_register`: Calls a routine and then stores a 16-bit value from register d5 into a global variable at offset 264.
- `store_word_from_register_2`: Calls a routine and then stores a 16-bit value from register d5 into a global variable at offset 262.
- `update_global_structure`: Reads bytes from a structure pointed to by a global variable and writes them to different offsets within that same structure, then updates a global variable and calls another routine.
- `load_parameters_and_branch`: Loads a 16-bit value from a pointer in a5 into d7, loads another 16-bit value from the incremented pointer into a2, sets d1 to that value, loads a 16-bit address from a constant into d6, moves that address to a2, and branches to another routine.
- `load_parameter_and_call_routine`: Loads a 16-bit value from a pointer in a5 into d7, calls a routine, sets d2 to zero, loads a 16-bit value from the incremented pointer into a2, sets d1 to that value, loads a 16-bit address from a constant into d6, moves that address to a2, and branches to another routine.
- `load_parameters_and_branch_2`: Sets d2 to 3, loads a 16-bit value from a pointer in a5 into d7, loads another 16-bit value from the incremented pointer into a2, sets d1 to that value, loads a 16-bit address from a constant into d6, moves that address to a2, and branches to another routine.
- `set_word_at_address`: Stores the value 65535 into a 16-bit memory location and returns.
- `execute_subroutines_and_store_byte_2`: Loads two addresses into registers, moves a word from a pointer in a5 to d5, calls a subroutine, clears d5, and branches to another location.
- `calculate_and_store_offset_value`: Calls a subroutine, shifts the value in d5 left by 5, adds a constant to the result to form a pointer in a4, moves a word from a pointer in a5 to d5, and branches to another location.
- `increment_global_byte`: Increments a byte at offset 1 of a global variable and returns.
- `perform_arithmetic_and_fetch_data`: Moves a word from a pointer in a5 to a2, calls a subroutine, loads a pointer into a4 and increments it by 4, moves a byte from d2 to d6, pushes d5 to the stack, calls a subroutine, pops d5 into d6, increments a4, and calls another subroutine.
- `check_and_call_nested`: Compares a byte at offset 3848 to one, and if not equal, checks a byte in a table; if it passes, it calls another routine and then calls another routine after moving a value from a pointer to a global variable.
- `compare_and_increment`: Compares a word at a specific address to a constant and, if not equal, branches to another location; otherwise, it increments a word in a register and returns.
- `subtract_and_branch`: Subtracts a word value from a register, branches if not zero, adds two to another register, and returns.
- `store_and_call`: Stores a word value from a global table to a specific memory location, then calls two routines in sequence.
- `evaluate_arithmetic_expression`: Calls a subroutine, loads two words from a pointer in a5, fetches a byte from a calculated address using a pointer in a4 and a value from a table, and performs an arithmetic operation on the byte at the pointer in a4 based on a character code.
- `store_byte_to_offset_4`: Calls a routine, moves a byte from d2 to d1, sets d0 to the value in d5, loads a hardware address into a4, calls two routines, and stores d5 at an offset of 470 from a4.
- `update_status_from_differences`: Calls another routine, calculates the difference between two 16-bit values at specific offsets in two different structures, determines a bitmask based on the signs and relative magnitude of these differences, and stores a byte from a table at offset 470 of a global variable.
- `call_and_set_global`: Checks if a global variable is not equal to itself, otherwise it calls a routine, sets a global variable to one, calls another routine, sets the global variable to zero, and returns.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a5)+ to d5, then returns.
- `store_scaled_byte`: Loads a 16-bit word from a pointer in a5, calculates an offset by shifting a value from a global variable left by 5 and adding it to the base address, reads a byte from that calculated address, stores it in a global variable at offset 950, and sets a byte at offset 8470 of a global variable to 16.
- `set_byte_via_call`: Calls another routine and stores the byte value 1 in a location at offset 1070 of a structure pointed to by d5.
- `copy_byte_to_global`: Copies a byte from a location pointed to by a5 into offset 2 of a global variable.
- `clear_global_byte`: Clears a byte at offset 2 of a global variable.
- `modify_structure_values`: Calls another routine, adds two values from a pointer in a5 to 16-bit words at offsets 270 and 350 of a structure, and stores two bytes from the pointer in a5 into offsets 430 and 470 of another structure.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `write_byte_to_offset`: Calculates an address using a global variable and a fixed offset, then writes a byte value to that address.
- `store_byte_to_global`: Copies a byte from a location pointed to by a5 into offset 470 of a global variable.
- `update_data_and_call`: Loads a pointer and two bytes from a5, stores the second byte at offset 7150 and the first byte at offset 7170 of the structure pointed to by the loaded pointer, and calls another routine with a pointer to the third element of the original list.
- `adjust_globals_and_call`: Sets two global variables to specific values, calls another routine with an adjusted parameter, then resets one of the globals.
- `store_byte_to_offset_1ae`: Copies a byte from a location pointed to by a5 into offset 430 of a global variable.
- `update_byte_pair`: Searches for a byte value in a table, updates matching entries, or adds new entries if no match is found.
- `search_and_update_table_2`: Loads several values from a pointer in a5, performs calculations based on a byte in d7, searches a table for a matching byte, and either updates an entry in the table or appends a new entry.
- `search_table_and_reset_structure`: Loads a word from a pointer in a5, searches a table for a matching byte, and if found, updates a value; otherwise, it sets a byte at offset 1070 of a structure to 1 and clears the following three bytes.
- `reset_structure_fields`: Sets a byte at offset 1070 of a structure to 1 and clears the three subsequent bytes.
- `push_and_call_routines`: Moves a word from a table into d5, writes a byte from d5 to a global address, saves registers to the stack, calls three different routines, restores registers, and returns.
- `store_byte_to_offset`: Stores a byte from a pointer in a5 into a global variable at offset 3.
- `store_byte_to_offset_large`: Stores a byte from a pointer in a5 into a global variable at offset 7078.
- `set_flags_and_clear`: Sets a specific byte in global memory to one, copies another byte from global memory, and clears three consecutive bytes.
- `check_global_flag`: Calls another routine and returns the value from a5 if a global variable is not equal to one; otherwise it calls another routine and returns that result.
- `process_table_and_call`: Writes a byte to a global address, calls a routine, fetches a word from a table into d5, calculates an address using d5 and a table, fetches a word from that address into d5, calculates another address, writes a word to a global address, and calls two routines before returning.
- `call_and_restore_registers`: Saves all general-purpose registers to the stack, calls a routine, restores the registers, and returns.
- `manage_state_loop`: Sets a global variable to a byte from a pointer, calls another routine, and enters a loop that calls another routine and checks the global variable for specific values.
- `clear_specific_globals`: Calls three different routines, clears a byte at offset 3848, clears a longword at offset 82a2, clears a byte at offset 227d, and then returns.
- `process_and_update_counter`: Compares a byte from a PC-relative address with d0 and returns if they are not equal; otherwise, it pushes a5 to the stack, stores a byte in a global variable, calls another routine four times with different a2 values, and updates a global counter based on a value in a5.
- `populate_sprite_data_structure_2`: Checks if the byte at the pointer in a2 is 1; if so, it loads a word from an offset in a2 using a5, loads a word from a4, updates a2, and calls a subroutine.
- `fill_zero_data_2`: Loads a word from a pointer in a5, calculates an offset, stores a byte and two words at that offset, and then copies seven words from a pointer in a5 to the calculated location using a loop.
- `clear_global_bytes`: Clears several specific bytes in a global table and one word in a global table.
- `store_pointer_and_words`: Calls another routine, stores a word from a4 into a global long, and then stores two consecutive words from a5 into two global words.
- `load_indexed_bytes`: Calls another routine, loads a byte from a table using a4 as a base, and then loads a word from a5 into d5 before calling another routine.
- `transfer_register_bytes`: Calls another routine and moves bytes from d2, d5, d3, and d6 into d3, d4, d6, and d7 respectively, then loads a word from a5 into d5 and calls another routine.
- `initialize_registers_and_call`: Calls another routine, moves a byte from d5 to d7, sets d4 to a constant, sets d2 to a constant, and calls another routine.
- `execute_callback_list`: Loads a byte from a constant address into d1, loads another byte into d0, sets a2 to d0, increments a pointer in a5, and calls another routine with d2 set to 2.
- `execute_minimal_callback`: Sets d2 to 6 and calls another routine.
- `fetch_and_store_table_value`: Calls another routine, then fetches a byte from a table using an index in d0 and stores it at an offset of 8184 from a pointer in a4.
- `iterate_and_call_handler`: Iterates backwards through a table, calculates an address based on a byte at an offset, calls a routine at that address, and increments table entries if specific bit conditions are met.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d5, then returns.
- `call_indirect_via_offset`: Calls a subroutine, shifts a byte from d2 left by 4, uses the result as an offset into a table to load a pointer into a0, and calls that pointer.
- `update_offset_value`: Calculates an address using a pointer in a4 and a value in d5, then stores a byte from that address into a location at offset 950 from a4.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `set_d5_and_branch`: Sets d5 to 2 and branches to another routine.
- `set_d5_and_branch_alternate`: Sets d5 to 4 and branches to another routine.
- `branch_to_offset`: Loads the value negative two into d5 and branches to an offset.
- `increment_byte_at_offset`: Increments a byte at offset 174 of a structure pointed to by a4 and returns.
- `increment_offset_byte`: Increments a byte value at a fixed offset relative to a register by two and returns.
- `decrement_offset_byte`: Decrements a byte value at a fixed offset relative to a register by one and returns.
- `decrement_offset_byte_by_two`: Decrements a byte value at a fixed offset relative to a register by two and returns.
- `set_and_branch`: Sets a register to a constant value and branches unconditionally to another routine.
- `store_masked_byte`: Masks d2 with 3, stores the result at an offset of 470 in a4, and calls a subroutine.
- `set_structure_values`: Stores 16 at offset 8470 of a structure in a4, stores 1 at offset 1070 of the same structure, and sets d5 to 1.
- `update_coordinates_and_decrement`: If d5 is non-zero, it performs bitwise operations on d5, stores d2 at offset 7190 of a structure in a5, stores the modified d5 at offset 7191 of that structure, and subtracts the lower nibble of d2 from a byte at offset 8470 of a structure in a4; otherwise, it returns.
- `store_byte_to_offset_2`: Stores the byte in d5 into offset 1030 of a structure in a4 and sets d5 to 2.
- `process_attribute_data`: Saves registers to the stack, performs arithmetic on d7 using a value from a4, loads bytes from offsets of a4 and constant addresses into d3, d6, d1, and d0, sets a2 to d0, calls another routine with d2 set to 2, and restores registers.
- `update_coordinate_bounds_2`: Loads a byte from an offset of a4 into d0, sets a2 to d0, and fetches bytes from offsets of a2 into d2 and d5; it then performs conditional checks on bytes at various offsets of a4 to update them or calls other routines to modify values at offsets 470 and 8470 of a4.
- `update_coordinate_bounds`: Updates two byte values in a structure by checking if they fall within ranges defined in a table, or by incrementing/decrementing them based on bit flags in a lookup table.
- `check_range_overlap`: Iterates through a table pointed to by a2, checking if d2 and d5 fall within the bounds defined by two bytes at each entry, and returns if a match is found.
- `search_table_backwards`: Iterates backwards through a table pointed to by a2, checking if d2 and d5 fall within the bounds defined by two bytes at each entry, and returns if a match is found.
- `process_range_data`: Stores bytes from a4 into global memory, calls another routine, and performs a loop that modifies a global variable and a table entry based on bitwise checks and comparisons.
- `calculate_squared_distances`: Calculates squared differences between values in a structure and a table, stores the results in a table, and then performs a search to find and reset minimum values in a table.
- `check_range_overlap_2`: Checks if two values, derived from adding bytes to registers d2 and d5, fall within specific ranges defined by bytes in a table pointed to by a2, and returns a status in d5.
- `modify_coordinates`: Calculates a value in d0 based on comparisons of d1, d2, and d5, then uses it to select a byte from a table and modifies 16-bit values in a structure in a5 by adding or subtracting that byte.
- `update_byte_from_table`: Uses a byte from offset 470 of a structure in a4 as an index into a table to fetch a new byte, which is then stored back into offset 470 of the structure in a4.
- `update_byte_from_table_alt`: Uses a byte from offset 470 of a structure in a4 as an index into a different table to fetch a new byte, which is then stored back into offset 470 of the structure in a4.
- `validate_and_store_coordinates`: Checks if shifted values of d6 and d7 match specific bytes in a structure in a4; if they do, it stores d6 and d7 into a structure in a5, otherwise it performs a series of bitwise and table lookups to validate the values before storing them.
- `copy_word_buffer_2`: Stores a byte from d6 into a structure in a2 and then enters a loop that copies words from a buffer in a4 to a destination, terminating the loop if a word containing all ones is encountered or the counter in d6 reaches zero, then calls another routine.
- `copy_word_buffer`: Clears a byte at offset 8486 of a structure in a2, then copies 16 words from a buffer in a4 to a destination calculated from a2 and a5 using a loop.
- `compare_and_update_entity_relations`: Iterates through a table of structures to compare coordinate values and specific flags between entities, updating a global status byte based on proximity and relative positioning.
- `sort_and_copy_data`: Calculates a destination address based on values in d3 and d6, updates several global words, and enters a loop that copies words from a buffer to a destination using varying offsets based on a comparison of two values.
- `update_status_flags`: Iterates through a table using a pointer derived from a global variable, checks a specific bit in a byte at a calculated offset, and updates a global byte and another value if the bit state has changed.
- `calculate_buffer_division`: Calls another routine, iterates through a buffer starting at a constant address to count non-zero bytes in d5, divides a constant by d5, stores the result in a global variable, calls two other routines, and returns.
- `decrement_and_check_global`: Decrements a byte in a global variable and, if the result is zero, calls another routine and writes 65535 to a different global variable.
- `replace_string_delimiters`: Searches through a string for a specific character, and if it finds a sequence of characters, it replaces a space with a pipe or clears a byte at the end of a segment based on a counter.
- `store_byte_in_list`: Stores a byte from d5 into a table at an offset specified by a global variable in a2, then increments that global variable if it is less than 160.
- `parse_nested_bracket_sequence`: Clears two global variables and iterates through a buffer, processing nested structures delimited by specific characters.
- `filter_byte_values`: Increments a pointer in a4, compares the byte at that pointer to three specific constants, and calls another routine if the byte matches one of the first two constants.
- `process_list_with_offset_logic`: Executes a loop four times that performs arithmetic on a value in d5 using a word from a table, modifies a byte in a global variable, and calls another routine if specific conditions are met.
- `process_bracket_buffer`: If a value in a register is not negative one, it calculates a pointer from a table, counts consecutive '[' characters in a buffer, and if the count is non-zero, calls another routine and iterates through the buffer to find matching ']' characters.
- `check_register_value`: Compares a word in a register to a constant and returns if they are not equal, otherwise it branches to another location.
- `search_table_for_value_2`: Stores d5 in a global variable, increments d5, and uses it to index into two different tables to compare a byte against 91; if it does not match, it branches back to an earlier instruction.
- `skip_bracket_sequence`: Decrements a register, and if it is greater than zero, enters a loop that increments a pointer while checking for ']' characters; if the character at the pointer is not null, it calls another routine before returning.
- `scan_until_character`: Scans memory starting at an address calculated from a4 and d5, advancing the pointer until it finds a character that is not '['.
- `increment_and_restore`: Increments a register, restores multiple registers from the stack, and returns.
- `initialize_and_loop_rows`: Clears several global bytes, calls another routine, and then enters a loop that calculates a destination address and calls another routine for each non-zero byte in a table.
- `update_and_sort_entity_attributes`: Iterates through a table of entries in a loop, modifying coordinate or attribute values based on a type byte and performing arithmetic on specific offsets.
- `sort_table_entries`: Performs a bubble sort on a table of entries by comparing multiple bytes within each entry, including values at specific offsets, and swaps the entries if they are out of order.
- `update_table_pointers`: Loads a pointer from a table, performs arithmetic on it and other values, and if certain conditions are met, calls another routine and returns.
- `update_counters_and_call`: Compares d5 and d6 against two values from a table and returns if they do not match; otherwise, it adjusts a pointer in a2, updates two global words, and calls three routines.
- `iterate_data_table_blocks`: Calculates a base address from a global variable and an offset, then executes a nested loop that calls update_structure_parameters and collapse_data_table_via_bitwise_or for eight blocks of eight entries each.
- `iterate_entity_table`: Calls three routines, calculates an address for a table using a4 and d5, and enters a nested loop that iterates through the table, checking bits in bytes at offsets of a4 and calling other routines based on those checks.
- `load_table_values`: Compares a global byte with a value at a fixed address; if the condition is met, it loads two bytes from a table into global variables using offsets derived from a pointer in a4, otherwise it sets two global bytes to 255.
- `update_table_and_call`: Checks two global bytes, increments a global byte, and then performs a series of table lookups and calls to other routines including a recursive call to itself.
- `populate_sprite_data_structure`: Calculates a memory address based on scaled register values and populates a structure in a6 with several 32-bit and 16-bit constants.
- `restore_registers_and_return`: Saves the current contents of registers d0 through d7 and a0 through a5 onto the stack, then immediately restores them and returns.
- `update_structure_data`: increments a global byte, checks bit 6 of an offset 2 byte in a structure, and if bit 6 is not set, writes several values to offsets 68, 100, 64, 80, 84, and 88 of the structure before decrementing the global byte and returning.
- `update_structure_parameters`: Increments a global byte, checks bit 6 of a value at offset 2 of a structure in a6, and if not set, populates that structure with several longwords and a word at specific offsets including 64, 68, 80, 84, 88, and 100 before decrementing the global byte and returning.
- `update_structure_parameters_2`: Increments a global byte, checks bit 6 of a value at offset 2 of a structure in a6, and if not set, populates the structure at offsets 64, 68, 80, 84, and 100 with specific constants and decrements the global byte before returning.
- `conditional_update_structure`: compares a byte at a global address to 1, clears a byte at another global address, and if that byte is zero, increments a global byte, checks bit 6 of an offset 2 byte in a structure, and if bit 6 is not set, writes several values to offsets 68, 100, 64, 80, 84, and 88 of the structure before decrementing the global byte and returning.
- `collapse_data_table_via_bitwise_or`: Calculates an offset into a table using arithmetic on input registers and then performs a series of bitwise OR operations on groups of five 32-bit words from a source buffer, storing the resulting values into a sequence of global variables.
- `calculate_and_compress_palette_data`: Calculates a pointer into a table based on input values, then performs a series of bitwise OR operations on groups of five 16-bit words from that table to produce a compressed set of 16-bit values.
- `expand_palette_indices`: Reads pairs of bytes from a source buffer to use as indices into a palette table, then writes the resulting color bytes into a destination buffer in a global variable.
- `process_bitmask_loop`: Calculates a destination address using a table and an offset, then performs two loops to clear or set bits in a value based on the status of a register.
- `update_bitmask_table`: Calculates a destination address using a table and an offset, then loops to clear specific bits and set specific bits at offsets 32, 64, 96, and 128 of each target structure.
- `sum_and_process_list`: Iterates through a list of words starting at a global variable, summing them into d5, and then enters a loop that calls another routine and performs bitwise operations on a byte at an offset of a4.
- `compare_string_lists`: Compares two lists of bytes, treating characters in the range 97 to 122 as case-insensitive, and returns -1 if they differ or 0 if the first list ends and the second list's current byte is zero.
- `call_multiple_routines`: Saves registers to the stack, loads a value into a0, and calls seven different routines before restoring registers and returning.
- `set_bitfield_flags`: Sets or clears a specific bit at various offsets within a structure based on the status of bits in a register.
- `count_set_bits`: Counts bits set in a data structure at multiple offsets, accumulating a total count based on conditional checks.
- `initialize_and_process_loop`: Initializes a register with zero, loads an address from a fixed location into a5, clears a4, calls another routine, sets d5 to seven, calls two more routines, then branches back to the beginning of this routine.
- `write_bit_data`: Processes bits from a data structure through multiple iterations, accumulating results and writing bytes to memory locations with incrementing offsets.
- `process_flags_loop`: Processes a series of flag operations in a loop, calling another routine for each iteration and updating a data pointer.
- `process_flags_short_loop`: Processes a shorter series of flag operations in a loop, calling another routine for each iteration and updating a data pointer.
- `initialize_and_compare`: Initializes specific memory locations with predefined values, performs comparisons, and updates memory based on comparison results.
- `process_data_with_loop`: Processes data through a loop of flag operations, handling conditional branches and updating pointers during iteration.
- `validate_and_update_flags`: Validates data against a lookup table, updates flags based on comparisons, and performs additional flag toggling operations.
- `set_bitfield_flags_2`: Decrements a counter and loops to call another routine multiple times, setting specific bits in a value derived from a table and a register.
- `apply_bitmask_to_rows`: Calculates a destination address and loops to unpack bytes from a source table, applying a bitmask and bit-shifting the result into a target structure at fixed intervals.
- `initialize_bitmask_structures`: Performs a nested loop to initialize a series of structures by clearing bits and setting specific bits at offsets 56, 120, 184, and 248, while also zeroing out intermediate structures.
- `update_system_state`: Increments a byte at a global address, waits for a bit to clear in a structure, and then writes several longwords and a word to a structure pointed to by a6.
- `update_system_state_alt`: Increments a byte at a global address, waits for a bit to clear in a structure, and then writes several longwords and a word to a structure pointed to by a6 using different constant values than R6.
- `scan_buffer_for_marker`: Clears a global byte, scans a buffer pointed to by a4 for a specific byte, and calls another routine once or twice depending on whether the marker is found.
- `apply_bitmask_to_rows_2`: Iterates through a source buffer, skipping bytes that are zero or equal to a specific constant, and calls another routine to process valid bytes.
- `store_longword_from_offset`: Reads a longword from offset 18 of a structure in a0 and, if it is not zero, stores it in a global variable.
- `initialize_data_stream`: Calls another routine, loads three addresses into registers, and calls a second routine with specific parameters before checking a byte in a global variable.
- `initialize_data_stream_alt`: Calls another routine, loads three addresses into registers, and calls a second routine before checking a byte in a global variable.
- `clear_word_table`: Loops 30 times to write zero into a table of words starting at a location relative to the program counter.
- `decode_table_coded_rows`: When a block counter expires, unpacks rows of words from a byte stream, each value being a byte plus or minus a lookup-table entry, written at a fixed row stride.
- `process_data_stream`: Initializes a structure at a0 with several values, then enters a loop that reads words from two source lists, performs bitwise operations and subtractions, and stores the resulting bytes into a destination buffer pointed to by a3.
- `execute_indexed_callbacks`: Checks a bit in a long word, calls two routines, performs bitwise operations on d0, and enters a loop that calls a routine from a table based on a value in d2.
- `subtract_and_or_bytes`: Subtracts a value from memory and performs bitwise OR operations with multiple constants on a data register before returning.
- `fetch_table_byte`: Compares a word in a register to a constant and, if not equal, branches to another location; otherwise, it loads a byte from a table using an offset in a register and branches to another location if the byte is zero.
- `conditionally_call_update`: Tests a longword at offset 9c5a and, if it is non-zero, calls another routine before returning.
- `toggle_bit_and_call`: Clears a bit in a longword at a specific address and, if the result is non-zero, calls another routine before returning.
- `execute_conditional_updates`: Compares a word at offset 32a8 to 31984, and if equal and a longword at offset 9c5a is non-zero, calls two different routines before returning.
- `execute_conditional_sequence`: Tests a byte at a pointer and, if non-zero, calls three different routines; otherwise, it clears a byte at a pointer, calls two routines, executes a routine at a fixed address, and calls a final routine.
- `clear_word_table_2`: Fills a table of eight words with zero using a loop and then sets a global variable to negative one.
- `clear_word_array`: Fills an array of fifty-nine words with zero using a loop.
- `update_data_structures`: Clears a byte at a specific address, sets a hardware register to a value, and if a byte at a pointer is non-zero, performs a series of memory copies and writes from tables and registers before calling another routine and potentially calling a final routine if a comparison with a global longword succeeds.
- `update_table_elements_2`: Iterates through a table of 20 elements, and if a value in d7 matches a table entry, sets that entry to 65535, clears a byte in another table, and calls another routine before returning.
- `update_table_elements`: Checks a global longword against a constant, calls a routine, and if successful, enters a loop that compares a value in d7 against a table, modifies a pointer in a1, and calls other routines.
- `calculate_jump_offset`: Computes the absolute difference between d1 and d3, then the absolute difference between a2 and d6, compares these results, and if the first is smaller, performs an indirect jump to an address calculated from a byte at an offset of a table.
- `branch_to_logic_2`: Performs a short unconditional branch to another routine.
- `branch_to_logic_3`: Performs an unconditional short branch to another routine.
- `branch_to_logic`: Performs two unconditional branches and two comparisons of a word in d5 against a constant.
- `scale_value_by_table`: Clamps the value in d5 between zero and six, then multiplies a byte retrieved from a table using a4 as an index by the clamped value, divides the result by eight, and stores it back in d5.
- `update_structure_from_table_2`: Calls another routine, checks a byte in a table, and if non-zero, iterates through a loop that fetches values from tables to update a structure and calls three different routines.
- `search_and_update_table_3`: Checks two bytes in a table, and if they meet specific conditions, enters a loop that calls two routines and updates a byte in a table using d5 while decrementing a counter.
- `update_status_bytes`: Iterates three times to write either one or zero to a destination buffer based on whether the corresponding value at a pointer offset is non-zero.
- `search_table_for_match`: Loops through a table of bytes, checking if a byte is zero or if a corresponding word in another table matches a value in d0, returning zero if a match is found or negative one if the loop completes.
- `fetch_indexed_values`: Checks if a word at a specific memory location is negative; if so, it extracts two masked bytes from offsets of a pointer derived from that word and returns them in a2, otherwise it calculates a value using two constants and returns it in a2.
- `count_zero_bytes`: Counts how many zero bytes exist in a four-byte sequence and compares this count against a value from a table, returning zero if the count is greater than the table value and negative one otherwise.
- `process_byte_table`: Loops through a sequence of bytes, compares them against a value in d1, and writes the loop index to a table if the comparison passes.
- `return_negative_one`: Restores registers d1 through d4 from the stack and returns with d0 set to negative one.
- `update_table_values`: Loops through a sequence of bytes, compares a value in d1 against a table, and performs several writes to a table and a byte if the comparison matches.
- `update_structure_from_table`: Fetches values from tables using a5 and d0, then loops through a sequence to update various fields in a structure pointed to by a1 based on values from a source pointer.
- `update_structure_from_a4`: Uses a4 to fetch a byte from a table, then loops through a sequence to update fields in a structure pointed to by a1 using values from d0, d4, and d7.
- `copy_data_block_2`: Checks a byte in a global variable, sets a bit in another global variable, and loops to clear bytes and call a routine at a fixed address.
- `copy_data_block`: If a global variable is non-zero, it copies four bytes from a source address to a destination address in a loop that runs four times.
- `fetch_structured_pointer`: Calls another routine, sets a pointer at an offset of a6 to a global variable, and enters a nested loop that calls another routine and checks a byte at an offset of a6 to eventually update a global variable with a value from a table.
- `convert_value_to_string`: Calls another routine, performs arithmetic on d5 to calculate an offset, and updates a global variable multiple times using nibbles from bytes at that offset to form a character representation, then sets a pointer at an offset of a6 to the global variable.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `copy_structured_data`: Reads a byte from a pointer in a3, performs arithmetic to calculate an offset, and then loops to copy bytes from a source table into a destination structure at a6.
- `compute_offset_value`: Reads two bytes from a memory location pointed to by a3, processes them through bit shifts and arithmetic operations, then uses the result as an offset to load a memory address into a5.
- `initialize_zero_loop`: Sets a long value at a global address to zero, initializes a counter in d1 to 3275, and loops to decrement the counter before branching to another routine.
- `load_address_and_branch`: Loads a value from a global address into a5 and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reset_audio_dma_state`: Locates a byte in a table, computes an offset from its position, clears a block of memory, stores pointers into a second table, and clears several audio volume and DMACON registers before returning.
- `update_audio_state`: Increments a byte in a global variable and compares a byte from a PC-relative address with another.
- `perform_audio_dma_operations`: A 37-instruction routine that calls another routine and modifies the AUD0LCH, AUD1LCH, AUD2LCH, and AUD3LCH hardware registers.
- `perform_dma_operation`: This routine contains 91 instructions and touches the DMACON register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_and_call_subroutines`: Checks bits in a register to determine which flags to set in a structure, copies several 16-bit and 32-bit values from a source buffer into a destination structure, and calls two subroutines to perform further updates.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_channels`: Sets bit 15 of the DMACON register to enable DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_update_structure`: Tests bit 1 of d0 and, if not set, writes 1024 to offset 156 of the structure in a0, then tests bit 2 of offset 28 of the structure in a0, loads a pointer from a global variable into a1, calls another routine, and branches to an external location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_hardware_registers`: Adds the value in d1 to a1, stores a 16-bit word from a table at offset d0 from a1 into offset 16 of a structure in a6, restores registers from the stack, and then performs a series of conditional checks involving a global variable, a hardware register at offset 96, and a byte at offset 30 of a structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_status_and_branch`: Writes the word 1024 to an offset of 156 in the structure pointed to by a0, checks bit 2 of the byte at offset 28 of that structure, and if that bit is not set, loads a constant address into a1 and calls another routine before branching to a distant location; otherwise it branches to a distant location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_control_register`: Sets bits 3, 4, and 7 of a value pointed to by a register and calls another routine in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_bit_and_call`: Tests bit 2 of a byte at offset 28 of the structure in a0; if set, it loads a value into a1 and calls another routine, otherwise it returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_and_structure`: checks a condition at offset 2 of a structure in a6, and if met, updates a global variable, modifies bytes at offsets 27 and 29 of the structure, and copies data from offsets 4, 8, and 16 of the structure to a new destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_on_bit_set`: Tests bit 1 of a byte at offset 28 of a structure in a0; if set, it loads a constant address into a1, calls another routine, and then branches to a fixed address; otherwise it branches to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_register_3`: Reads a byte from a memory location pointed to by a register, performs a bitwise AND operation with a constant, and stores the result into a different memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_if_not_equal`: If a zero flag is not set, it loads a constant address into a1, calls another routine, and then branches to a fixed address; otherwise it branches to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_and_bitmask`: If a condition is not met, it clears a byte at offset 27 of the structure in a6, checks bit 6 of a byte at offset 30 of the structure in a6, and if that bit is set, clears a byte at offset 29 of the structure in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_address`: Performs an unconditional short branch to a specific address.
- `write_and_branch_on_bit`: Writes a 16-bit word to offset 156 of a structure in a0, checks bit 0 of offset 28 of that structure, and if the bit is not set, calls another routine with a pointer in a1 before branching to a fixed address; otherwise it branches to a fixed address.
- `push_registers_and_load_pointer`: Pushes a0 and d0 onto the stack and loads a2 with a pointer from offset 8 of the structure in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_field_via_pointer`: If a condition is met, it clears a byte in a structure, copies a longword pointer from a structure to a destination, writes a word from the source structure to an offset of that destination, and updates two word values in the destination and a global variable before branching to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_bit_and_call`: Checks bit 0 of offset 28 of a structure in a0, and if the bit is not set, loads a pointer into a1 and calls another routine before branching to a fixed address; otherwise it branches to a fixed address.
- `push_registers_and_load_pointer_2`: Pushes the contents of address register 0 and data register 0 onto the stack, then loads address register 2 with a value from an offset of 8 from the address in address register 1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_and_bitmask_minimal`: Copies a pointer from offset 4 of a6 to a location pointed to by a5, writes a word from offset 8 of a6 to offset 4 of that location, writes a word from offset 16 of a6 to offset 6 of that location, writes a word from offset 20 of a6 to d0, writes d0 to offset 6 of that location, and performs an OR operation on d0 with a value at a specific memory address before branching.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_with_pointer`: Loads a pointer into a1 and calls another routine, then branches to a fixed address.
- `push_registers_and_load_pointer_2`: Pushes the contents of address register 0 and data register 0 onto the stack, then loads address register 2 with a value from an offset of 8 from the address in address register 1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_data_to_structure`: Writes a word from offset 16 of a6 to d0, writes d0 to offset 6 of the location pointed to by a5, writes a word from offset 20 of a6 to d0, performs an OR operation on d0 with a value at a specific memory address, and branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_branch_2`: Calls another routine and then branches to a fixed address.
- `update_structure_values`: Pushes the contents of a0 and d0 onto the stack, decrements a 16-bit word at offset 14 of a structure in a1, and if a 16-bit word at offset 36 of that structure is not negative, loads a 16-bit word from offset 38 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_and_or_data`: Writes a word from d0 to offset 6 of the location pointed to by a5, writes a word from offset 20 of a6 to d0, performs an OR operation on d0 with a value at a specific memory address, and branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_and_branch_on_bit_alt`: Writes a 16-bit word to offset 156 of a structure in a0, checks bit 7 of offset 29 of that structure, and if the bit is not set, loads a pointer into a1 and calls another routine before branching to a fixed address; otherwise it branches to a fixed address.
- `restore_processor_state`: Restores all data and address registers from the stack and executes a return from exception.
- `update_structure_values`: Pushes the contents of a0 and d0 onto the stack, decrements a 16-bit word at offset 14 of a structure in a1, and if a 16-bit word at offset 36 of that structure is not negative, loads a 16-bit word from offset 38 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `bitwise_or_global_value`: Loads a 16-bit word from offset 20 of a structure in a6 into d0, performs a bitwise OR with that value into a global word, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_routine`: Loads a long value into a1, calls another routine, and then performs an absolute branch.
- `restore_context_and_return`: Restores all data and address registers from the stack and returns from an exception.
- `decrement_and_update_pointer`: Decrements a 16-bit value at offset 14 of a structure pointed to by a1, then if the 16-bit value at offset 36 is negative, it stores the sum of the 32-bit value at offset 24 and the 16-bit value at offset 38 into the 32-bit location pointed to by the address at offset 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_registers`: Calls another routine, sets a 16-bit word at offset 3724 of the hardware registers by ORing a value with bit 15, calls another routine, and then moves values from a table into hardware registers and address registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_external_routine`: Calls another routine and then performs an absolute branch.
- `restore_context_and_return`: Restores all data and address registers from the stack and returns from an exception.
- `decrement_and_update_pointer`: Decrements a 16-bit value at offset 14 of a structure pointed to by a1, then if the 16-bit value at offset 36 is negative, it stores the sum of the 32-bit value at offset 24 and the 16-bit value at offset 38 into the 32-bit location pointed to by the address at offset 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_registers`: Sets bit 15 of a word from a memory location into the audio control register, calls another routine, and then copies two 32-bit values and two 16-bit values from two different tables into a structure pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_3`: Restores d0 through d7 and a0 through a6 from the stack and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_audio_and_copy_data`: Sets bit 15 of d0 and writes it to the audio control register, calls another routine, and then copies two 32-bit values and two 16-bit values from two different tables into a structure pointed to by a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_audio_and_copy_data`: Writes d0 to the audio control register, calls another routine, and then copies two 32-bit values and two 16-bit values from two different tables into a structure pointed to by a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_coordinate_and_table_pointers`: Decrements a 16-bit counter at offset 14 of a structure in a1, then conditionally calculates a new 32-bit coordinate based on values at offsets 28, 36, and 38 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_channels`: Calls another routine and then updates the three Amiga audio channel length and starting position registers using values loaded from three separate data structures via PC-relative addressing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_and_table`: Updates a pointer and coordinate values within a structure based on conditional checks of offsets 14, 24, 26, and 28, then updates several global table entries with new values and increments a global variable by 16.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_audio_parameters`: Loads three sets of long and word values from a table into registers using offsets from a pointer in a6 and stores them into registers d0, d4, c0, c4, b0, and b4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_position_and_table`: Updates a 32-bit coordinate and a 16-bit offset in a structure pointed to by a register, then populates a table of pointers from memory and increments a global variable by 16.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_audio_parameters_to_a0`: Loads three sets of long and word values from a table into registers d0, d4, c0, c4, b0, and b4, and then loads a fourth set of long and word values into registers a0 and a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_values_2`: Adds a longword from offset 28 of a structure in a1 to d0, stores d0 in a structure in a2, calculates a new word value by subtracting a bit-shifted d1 from a word at offset 40 of a1, stores this result at offset 4 of a2, stores a word from offset 6 of a1 at offset 154 of a0, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_multiple_pointers`: Loads three sets of pointer and word pairs from three different locations into a5, using a6 as a base for each set of data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_word`: Subtracts a bit-shifted d1 from a word at offset 40 of a1, stores the result at offset 4 of a2, stores a word from offset 6 of a1 at offset 154 of a0, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_pointer_sets`: Loads three sets of pointer and word pairs into a5 by accessing data through three different addresses stored in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_modified_word`: Subtracts d1 from d0, stores the result at offset 4 of a2, stores a word from offset 6 of a1 at offset 154 of a0, and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_data_and_increment_global`: Loads two sets of pointer and word pairs into a5, increments a global longword by 16, and branches to another routine if a byte from a global variable is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_to_structure`: Stores a word from offset 6 of a1 into offset 154 of a0, stores a word from offset 6 of a1 into offset 156 of a0, and returns after popping two values from the stack.
- `conditional_copy_words`: Tests a word value at a fixed offset, branches if not zero, copies words from one location to another, and then branches unconditionally.
- `initialize_structure_data_2`: Writes a fixed value to a memory location pointed to by a2, sets a word at offset 4 of that location to 1, and copies a word from offset 6 of a1 to offset 156 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields_from_table`: Reads two words from a table using a pointer in a register, stores them into a structure at specific offsets, updates a global variable by adding sixteen, and branches based on a byte read from another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_to_structure_minimal`: Branches to another routine, then stores a word from offset 6 of a1 into offset 156 of a0 and returns after popping two values from the stack.
- `conditional_copy_words`: Tests a word value at a fixed offset, branches if not zero, copies words from one location to another, and then branches unconditionally.
- `initialize_structure_data_2`: Writes a fixed value to a memory location pointed to by a2, sets a word at offset 4 of that location to 1, and copies a word from offset 6 of a1 to offset 156 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_complex_structure`: Writes four pairs of longword and word values to a structure in a5, increments a global longword by 16, and performs conditional writes to a structure in a0 and a5 based on the global value, finally storing a word from offset 6 of a1 into offset 156 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_three_words`: Copies two 16-bit words from a table at a1 to offsets 96 and 9a of a structure in a0, then copies a third 16-bit word from offset 6 of a1 to offset 9c of a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_counter_and_byte`: Loads values from a table into a5, increments a 16-bit global variable by 16, and conditionally updates a byte at a global variable based on the previous value of another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_two_words`: Copies a 16-bit word from offset 2 of a1 to offset 96 of a0, then copies a 16-bit word from offset 6 of a1 to offset 9c of a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_byte_with_flag_reset`: Loads a 32-bit pointer and a 16-bit value from a table using a relative address, increments a global word by sixteen, and moves a byte from a global word to another global word if the first byte is non-zero, otherwise it clears the first byte and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_single_word`: Copies a 16-bit word from offset 6 of a1 to offset 9c of a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counter_and_byte`: Loads values from a table into a5, increments a 16-bit global variable by 16, and conditionally decrements a byte at a global variable and a 16-bit global variable based on several checks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_movea_rts`: Small routine of 3 instructions that copies (a7)+ to d0, then copies (a7)+ to a0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_and_register`: Clears a global variable, decrements it if it is non-zero, and performs a bitwise OR on the value pointed to by a register with a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_hardware_routine`: Checks if a 32-bit global variable is zero, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_test_status`: Conditionally updates a byte and a 16-bit global variable, then tests a byte at a global variable and sets the sign flag based on its value before returning with zero in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_bits`: Sets specific bits in a value pointed to by a4 and then returns immediately.
- `call_hardware_routine_2`: If the zero flag is not set, it saves registers, loads the address of the custom hardware into a0, loads a pointer from a global variable into a1, calls another routine, restores registers, and returns with d0 set to 0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_shift_values`: Tests a byte at a global variable and, if not zero, decrements it and checks if it is zero; if not, it subtracts 16 from a 16-bit global variable, tests a byte at another global variable, sets the sign flag based on that byte, loads a byte from a relative offset into d0, clears a byte at a global variable, and shifts d0 left by 4 bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_with_pointer`: Loads a pointer from a global variable into a1, moves a0 to a5, calls another routine, restores registers, and returns with d0 set to 0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_shift_values_conditional`: If the zero flag is set, it decrements a byte at a global variable and, if not zero, subtracts 16 from a 16-bit global variable, tests a byte at another global variable, sets the sign flag based on that byte, loads a byte from a relative offset into d0, clears a byte at a global variable, and shifts d0 left by 4 bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_simple`: Moves a0 to a5, calls another routine, restores registers, and returns with d0 set to 0.
- `sequence_of_calls_with_pointers`: Loads four different pointers into a4 sequentially and calls another routine after each of the first three loads.
- `calculate_offset_difference`: Moves a pointer from offset 8 of the structure in a4 to a5, writes a 16-bit value from offset 72 of the structure in a4 to offset 6 of the structure in a5, pushes d0 to the stack, loads a 16-bit value from offset 44 of the structure in a4 into d0, and subtracts a 16-bit value from offset 142 of the structure in a4 from d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: This routine contains 15 instructions but no specific logic is provided.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sequence_of_calls_with_pointers_alt`: Loads three different pointers into a4 sequentially and calls another routine after each of the first two loads.
- `update_clamped_value`: Calculates a difference between two 16-bit values from a structure, clamps the result to a minimum of zero, and stores it in a destination field within a pointer retrieved from the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_2`: This routine contains 14 instructions but no specific logic is provided.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sequence_of_calls_with_pointers_short`: Loads two different pointers into a4 sequentially and calls another routine after the first load.
- `enable_dma_channel`: Called by jump_vector_entry, this routine performs 17 instructions and writes to the DMACON register to enable DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unnamed_routine`: This routine performs 13 instructions and was entered from outside the static flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_channel_alt`: This routine performs 15 instructions and writes to the DMACON register to enable DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_values`: If a condition is met, it shifts a byte from a memory location left by 4 bits, stores the result in a global variable, clears two bytes in another global variable, increments a byte at offset 32 of a global variable while masking it with 127, and finally compares a byte from a memory location with a byte at an offset of a structure in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_structure_data`: Subtracts a 16-bit value at offset 142 of a structure in a4 from d0, and if the result is not positive, clears d0 and stores it at offset 8 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_conditionally`: Tests a byte at offset 52 of a global variable; if it is non-zero, it branches elsewhere, otherwise it restores registers from the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_offset_value_2`: Tests a 16-bit value at offset 38 of a structure in a4; if non-zero, it branches elsewhere, otherwise it stores a 16-bit word from offset 6 of a structure in a4 into offset 154 of a structure in a5 and returns, or sets a byte at offset 14 of a structure in a4 and stores a 16-bit word from offset 4 of a structure in a4 into offset 154 of a structure in a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_branch`: If a condition is met, it branches elsewhere, otherwise it restores registers from the stack and returns.
- `extract_bitfield_value`: Calls another routine and then performs bitwise operations on a 16-bit word from a structure in a6 to extract a value based on a 4-bit mask, returning a modified 16-bit value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_between_structures`: Copies a 16-bit word from offset 6 of a structure in a4 to offset 154 of a structure in a5 and returns.
- `set_flag_and_copy_word`: Writes 1 to offset 14 of a structure in a4, copies a 16-bit word from offset 4 of the structure in a4 to offset 154 of a structure in a5, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `validate_and_mask_bitfield`: Checks if a 12-bit value at an offset of two bytes from a pointer in a6 is zero, then performs bitwise operations on a global variable and calls another routine three times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 154 into d4, then returns.
- `load_address_sequence_into_a4`: Loads four consecutive memory addresses into register a4 by calling another routine after each load.
- `validate_and_load_pointer`: Checks if a longword at offset 58 of a structure is non-zero, then loads a longword from offset 20 of that structure into a register if a byte at offset 48 is non-zero, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_and_structure`: Uses a value from a register to clear a specific bit in a byte located at a negative offset from a pointer in a6, then copies a 16-bit word from a structure in a6 to another structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_addresses_and_call`: Loads three different addresses into a4 and calls a routine twice before loading a final address into a4.
- `decrement_and_call_on_zero`: Tests a longword at offset 58 of the structure in a4 and branches if it is zero; otherwise it loads a longword from offset 20 of the structure into d0, moves it to a3, and checks a byte at offset 48 of the structure.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manipulate_bitmask_and_decrement_counter`: Clears a bit in a byte pointed to by a register using a bit index from a register, decrements a byte at an offset in a structure, and calls another routine if that byte reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_load_address`: Calls another routine and loads a long value into a4.
- `decrement_and_call_if_nonzero`: Checks a long word at offset 58 of a structure; if it is not zero, it moves a pointer from offset 20 to a3, decrements a byte at offset 48, and calls another routine if the byte is not zero.
- `branch_on_status`: Tests a word at offset 50 of the structure in a4 and branches based on the result; if not zero, it loads a word from offset 52 of a4 into d0 and branches if d0 is non-zero, otherwise it stores a byte from a3 offset 1 into a4 offset 48.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_command_byte`: Evaluates a byte from a register to perform conditional data updates, including decrementing a byte at an offset in a structure, updating a counter in a table, or writing specific values to a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_address_and_decrement_loop`: Loads an address into a4, checks a long word at offset 58, and if non-zero, enters a loop that decrements a byte at offset 48 and calls another routine until the byte reaches zero.
- `branch_on_status`: Tests a word at offset 50 of the structure in a4 and branches based on the result; if not zero, it loads a word from offset 52 of a4 into d0 and branches if d0 is non-zero, otherwise it stores a byte from a3 offset 1 into a4 offset 48.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_command_byte_2`: Reads a byte from a register and performs conditional bitwise operations on a memory location based on its value.
- `copy_word_value`: Copies a 16-bit word from a location offset by 16 in a6 to another location offset by 6 in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_d0_value`: Compares the byte in d0 to 7 and branches if equal, otherwise compares d0 to 10 and branches if equal, otherwise it returns.
- `copy_word_to_offset`: Copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_parameters`: Copies a block of data from a global source to a destination table and populates multiple memory locations with specific values for audio configuration.
- `execute_repeated_routine`: Executes another routine three times in a loop, incrementing a counter each iteration and using it as an index for memory access.
- `configure_audio_structure`: Clears a block of 32 bytes in a structure, updates audio hardware registers from a table, and populates a structure with values from a global table and a constant data source.
- `process_event_queue`: Checks a pointer in a global variable to determine if an event queue is empty; if not, it decrements a counter and processes the next event by reading a value from the queue, updating several global variables, and calling two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_timer_value`: Adjusts a byte at offset 19 of a structure in a6 by adding or subtracting a value derived from offset 3, clamping the result between 0 and 64, then stores the result in a 16-bit field at offset 8 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_buffer_and_jump`: Writes a 3-byte sequence to a buffer pointed to by a0, copies a value from a pointer in a4 to offset 96 of a structure in a5, and then performs an indirect jump based on a value at offset 29 of a structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of a structure in a6 from the byte at offset 19 of the same structure, clamps the result to zero if negative, and stores the resulting byte as a word at offset 8 of a structure in a5.
- `decrement_and_store_byte_2`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_via_table`: Loads a byte from a pointer in a2, and if the byte is negative, subtracts 128 from it and performs an indirect jump using an address from a table; otherwise, it jumps to the address at the calculated offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `subtract_and_store_word`: Subtracts a byte in d0 from a byte at offset 19 of a structure in a6, and if the result is negative, clears the byte at offset 19 before storing the result as a word at offset 8 of a structure in a5.
- `decrement_and_store_byte_2_2`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_via_table`: If the value in d0 is negative, subtracts 128 from it and performs an indirect jump using an address from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_store_word`: If the value in d0 is negative, clears the byte at offset 19 of a structure in a6, then stores the byte at offset 19 as a word at offset 8 of a structure in a5.
- `decrement_and_store_byte_2_2`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_indirect_jump`: Performs an indirect jump using an address from a table at an offset determined by d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_as_word`: Copies the byte at offset 19 of a structure in a6 to offset 8 of a structure in a5 as a word.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_byte_and_store_2`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_address`: Clears d0 and jumps to the address stored in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_at_offset`: Stores the word in d0 at offset 8 of the structure pointed to by a5 and returns.
- `clear_global_byte_flag`: It stores one less than the value of a byte in a6 at offset 3 into a global variable, writes zero to a global variable, and finally sets the flag to all ones.
- `clamp_byte_to_word_and_store`: Reads a byte from offset 3 of a structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of the structure in a6, and stores it as a word at offset 8 of a structure in a5.
- `process_byte_value`: Reads a byte from offset 3 of a6, processes it through bit shifts and arithmetic operations, performs a comparison with 63, and stores the result to fixed memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_complex_structure`: Writes several values to a structure pointed to by a4, including setting specific constants and clearing fields, then calls two different routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_byte_to_global`: Reads a byte from offset 3 of the structure in a6; if non-zero, it clears a byte at a global address and writes that byte to another global address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_structure_fields`: Writes a word from offset 6 of a4 to offset 9c of a5, then initializes several fields in a structure pointed to by a0, and conditionally clears fields in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_global`: Stores the byte from register d0 into a global variable and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_calculate_d0`: Initializes several fields in a structure pointed to by a0, conditionally clears fields in a4, and performs arithmetic on d0 using a value from offset 140 of a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `update_status_register`: Reads a byte from a memory location pointed to by a register, performs a bitwise operation, and writes the result back to the same location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_attribute_from_table`: Sets two 16-bit values at offsets 4 and 6 of a structure in a0, clears several fields in a structure in a4, and then calculates an index to fetch a word from a table pointed to by a4 and stores it at offset 70 of the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position`: Reads a 16-bit coordinate from a table and writes it to a specific memory location used for sprite positioning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_2`: Reads a 16-bit coordinate from a table using an index provided in a register and writes the resulting value to a memory location used for sprite positioning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_register_2`: Reads a byte from a memory location pointed to by a register, performs a bitwise AND with a constant, and stores the result into a different memory location pointed to by another register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_state`: The routine performs a series of conditional checks and branches based on data in memory, resulting in a large number of unreachable code blocks and non-linear control flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_control_character_sequence`: Checks if a character in a register is a carriage return, then if it is a specific control character, extracts bits from a byte at an offset from a pointer in a6 to construct a 16-bit value and modifies a structure pointed to by a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_data_2`: Calculates an offset using a value from a table and a register, stores a 16-bit word from that location into a structure at offset 70, sets a byte at offset 46 to 255, calls two different routines, clears 16-bit words at offsets 134 and 136, stores registers A2 and A3 into offsets 58 and 24, and decrements a 16-bit value at offset 62 by 1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_conditionally`: Performs a conditional branch to an offset and then an unconditional branch to another offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_structure_fields`: Adds a 16-bit value in D0 to itself, loads a pointer from offset 16 of a structure into A0, stores a 16-bit word from the address at A0 plus D0 into offset 70 of the structure, sets a byte at offset 46 to 0, calls two different routines, clears 16-bit words at offsets 134 and 136, stores registers A2 and A3 into offsets 58 and 24, and decrements a 16-bit value at offset 62 by 1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_offset`: Performs an unconditional branch to another offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_counters`: Updates several fields in a structure pointed to by a register, including setting specific bytes to zero and decrementing a counter at offset 62.
- `store_byte_to_offset_3`: Moves a byte from the address pointed to by a2 into d0, stores d0 as a word at offset 68 of the structure in a4, and branches to a fixed address.
- `update_pointer_field`: Loads a longword from offset 54 of the structure in a4 into a2, stores that value at offset 58 of the structure in a4, and branches to a fixed address.
- `multiply_and_store_result`: Moves a byte from the address pointed to by a2 into d0, multiplies d0 by a word at the address pointed to by a6, stores the resulting word at offset 64 of the structure in a4, and branches to a fixed address.
- `divide_constant_by_byte`: Moves a byte from the address pointed to by a2 into d0, divides the constant 750 by d0, stores the resulting word at the address pointed to by a6, and branches to a fixed address.
- `copy_sprite_data_and_clear_structure`: Clears bits 2 and 3 of a byte at offset 138 of a structure, then enters a loop that copies 15 bytes from a calculated memory location into a structure starting at offset 94.
- `update_dma_control_register_2`: Writes values to several offsets of a structure pointed to by a4 and a5, clears a word at an offset of a4, and branches to an earlier instruction.
- `extend_and_store_byte`: Fetches a byte from a pointer in a2, extends it to a word, stores the word at offset 140 of the structure in a4, and branches to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_flag`: Checks a bit in a byte at offset 31 of a structure, updates a byte at offset 35 using a table lookup and an addition, and if a bit is set, resets that byte to zero and updates a pointer at offset 36 with a value from a table or another pointer, then negates the byte at the new pointer location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_data_from_calculated_offset`: Fetches a byte from a2, multiplies it by 14, adds an address, and uses the result as a pointer to load five words and two longwords into offsets 36, 38, 40, 28, and 16 of the structure in a4 before branching to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_2`: Restores d1 and a0 from the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_fields_three_passes`: Copies two word fields to two other offsets in a record on each of three passes, advancing the base pointer by two bytes per pass.
- `copy_word_fields_two_passes`: Copies two word fields to two other offsets in a record on each of two passes, advancing the base pointer by two bytes per pass.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `stub_return_only`: Contains only a return and does nothing.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### Disk.0 bytes 30464-30720 (code never loaded by a reference run)

- `modify_memory_with_bitmask_loop`: Performs a loop where it applies a bitwise AND to a byte at a pointer in a5, then bitwise ORs a byte at offset 32, bitwise ANDs a byte at offset 64, bitwise ANDs a byte at offset 96, and bitwise ORs a byte at offset 128, advancing the pointer by the value in a0 each iteration.

The function list continues in: .
