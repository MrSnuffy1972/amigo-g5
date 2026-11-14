# Game 141 (AdventuresOfRobinHood v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00141). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2195858 top-level bytes in 130 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 126 of 126 units identical (bytes and relocations), 300 raw-word lines |
| Functions named and described | pass, 457 of 457 named (0 author symbols), 457 of 457 described (46 by rule, 394 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5040 of 5040 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 16256 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7054 of 7054 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21406 |
| code | 38086 |
| data | 2029075 |
| padding | 27 |
| unused | 107264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 168 | 13 |
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

- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: raw disk image (43461 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.1: raw disk image (878080 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.2: raw disk image (31360 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1.hdf: hard-disk image (AmigaDOS OFS, 2336 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree AdventuresOfRobinHood_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 4232 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: bytes 8960-16128 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: bytes 30464-30720 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: bytes 40448-40704 look like code no run loaded: recovered position-independently (load address unknown)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.2: bytes 22784-31360 look like code no run loaded: recovered position-independently (load address unknown)
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
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1C6
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E2, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1B2
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1E8, AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/game.slave +1CC
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
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: 3 logged loads cover 7680 bytes of the image
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: 92 track-loaded chunks found in recovered runtime segments and classified through them
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.53)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: chunks 21-21 (bytes 5376-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.90)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: chunks 26-33 (bytes 6656-8704) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.06)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: chunks 133-133 (bytes 34048-34304) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.54)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.0: chunks 160-169 (bytes 40960-43461) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.16)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.1: chunks 588-600 (bytes 150528-153856) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.67)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.1: chunks 1053-1111 (bytes 269568-284672) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.16)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.1: chunks 1127-1130 (bytes 288512-289536) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.2: 1 logged loads cover 8576 bytes of the image
- AdventuresOfRobinHood_v1.1/AdventuresOfRobinHood_v1.1_hdf/AdventuresOfRobinHood/Disk.2: chunks 0-86 (bytes 0-22272) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.93)
- 3 never-loaded region(s), 16000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 29 packed track region(s), 758016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_low_memory_on_match`: Calls six unnamed routines, compares a returned identifier against a fixed value, and on a match writes a sequence of no-op, jump and subroutine call instructions into low memory before jumping to it; the entry references the text Disk.0.
- `chain_two_calls_return_d1`: Calls one unnamed routine, then calls a second routine through a pointer offset, and returns the D1 register value unchanged.
- `scale_word_pair_by_constant`: Masks the D0 and D1 inputs to their low words, multiplies each by a fixed constant, and returns the two products packed together.
- `chain_call_and_return_d1`: Calls one unnamed routine, then calls a second through a different pointer offset, and returns the D1 register value unchanged; the entry references the text Disk.2.
- `set_global_flag_one`: Stores the value one into a global variable and returns.
- `set_global_flag_two`: Stores the value two into a global variable and returns.
- `hang_in_infinite_loop`: Loops forever with no exit path and no other operations; the listed facts also name the beam position register as touched.
- `patch_delay_loop_on_match`: Calls one unnamed routine, then, if a fixed word matches a delay-loop instruction pattern, overwrites that location with no-op and call instructions pointing at other routines, and returns the first call's value.
- `unpack_backward_stream_data`: Reads a packed size from the longword before its input, then decodes literal runs and back-references while moving the output pointer downward toward a start limit; it calls four unnamed routines for bit and length reads.
- `test_d5_then_return`: Tests whether the D5 register shifted right by one is nonzero, then returns in either case without doing anything else.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bit_buffer_by_count`: Shifts a bit buffer held in D5 right once per count in D0, pulling the next longword from memory when the buffer empties and carrying the bit shifted out into the top.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_one_if_not_equal`: Branches back when the previous comparison was not equal, otherwise stores the value one in d5 and returns.
- `set_one_if_below`: Compares the bytes in d5 and d6 and branches away when they are equal or when d5 is above d6, otherwise stores the value one in d5 and returns.
- `compare_bytes_skip_set`: Compares two bytes carried in d5 and d6, and returns without modifying anything when d6 is less than d5, otherwise it stores the value one into d5 and returns.
- `load_byte_pop_pointers`: Loads a byte from a global variable at offset 52 into d5, then pops a pointer into a2 and a word into d6 from the stack, and falls through into the following routine.
- `call_store_byte_check`: Pops a pointer into a2 from the stack, calls a routine, stores the low byte of d5 into a global variable at offset 52, and returns zero in d5 when that value is nonzero, otherwise it stores one and returns.
- `compare_word_increment_stack`: Loads a word into d5 from the stack, increments a following word on the stack, and returns by clearing that word when it does not exceed the loaded value, otherwise it advances the stack pointer and returns zero.
- `compare_word_global_return`: Loads a word into d5 from the stack, compares it against a global variable at offset 16668, and returns one in d5 when they differ, otherwise it returns zero.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d5, then returns.
- `check_flag_byte_set`: Compares a global variable at offset 53 against the value one, and returns without modifying anything when they differ, otherwise it stores one into d5 and returns.
- `search_byte_table_index`: Calls a routine, then when a global variable at offset 1536 equals 3000 it scans a byte table searching for a byte equal to the carried value, storing the matching index, and otherwise it calls the routine again and stores the index only when no match is found.
- `validate_byte_ranges_table`: Doubles an index from the stack to form an offset, reads four bytes from two paired locations in a table, and returns one in d5 when both pairs fall within their expected ranges, otherwise it returns zero.
- `validate_byte_ranges_structure`: Calls a routine, reads two bytes from a structure using a carried value, then compares them against paired table bytes and returns when both pairs fall within range, otherwise it returns zero.
- `compare_word_global_match`: Calls a routine, compares a carried word against a global variable at offset 9040, and returns one in d5 when they match, otherwise it returns zero.
- `find_first_below_entry`: Walks a table of 16-bit entries starting at offset 1110 of a structure, using a counter derived from a global variable, and stores the index of the first entry whose value is below the byte at offset 0 of the structure in a5.
- `store_matching_entry_index`: Scans a table of 16-bit entries starting at offset 1110 of a structure for the first entry matching a byte-comparison condition, then stores half of its index into a global variable.
- `store_first_matching_index`: Searches a table of 16-bit entries starting at offset 1110 of a structure for the first entry satisfying a byte-comparison condition and stores half of its index into a global variable.
- `check_and_return_when_match`: Compares a byte from a structure in a5 with a byte at offset 8510 of another structure and a byte at offset 8470 with an immediate value, returning without changes when both conditions hold.
- `check_and_return_when_equal`: Compares a byte from a structure in a5 with a byte at offset 8510 of another structure and a byte at offset 8470 with an immediate value, returning without changes when the first matches and the second equals the immediate.
- `read_input_value`: Calls a routine to read a value, returns it directly when two referenced bytes are both negative one, otherwise calls another routine and returns its result.
- `check_flag_not_set`: Calls a routine, then checks whether a 16-bit value at offset 270 of a structure indexed by a doubled counter equals negative one and returns without setting a flag in that case.
- `store_byte_when_equal`: Compares a byte from a structure in a5 with a byte at offset 14494 of a global variable and stores the masked byte into offset 14496 when they match.
- `compare_two_byte_pair`: Compares two consecutive bytes from a structure in a5 with two bytes at the address held by a global variable and returns without changes when both match.
- `compare_single_byte`: Compares a byte from a structure in a5 with a byte at the address held by a global variable and returns without changes when they are equal.
- `check_flag_and_set_result`: Compares a byte at global variable offset 5655 with the values 1 and 3, then compares a byte from the address in a5 with d5, and sets d5 to 1 only when both the offset-5655 value is 1 or 3 and the compared bytes match, otherwise it clears d5.
- `verify_second_flag_value`: Checks whether the byte at global variable offset 5655 equals 2 and compares a byte from the address in a5 with d5, setting d5 to 1 only when both conditions hold and clearing it otherwise.
- `scan_table_for_value`: Decrements a global counter and returns its original value when it goes negative, otherwise it scans a table of 32-byte entries comparing each byte with d5, advancing a pointer while searching and finally jumping to the routine at address 3fec.
- `validate_range_values`: Calls the routine at address 3e24, then compares a byte from global variable offset 5655 with two byte values and returns without changes when both comparisons fall within the expected range, otherwise it clears d5.
- `compare_after_double_call`: Calls the routine at address 3dc8 twice, then compares the value in d5 with the value loaded into a2, setting d5 to 1 when they differ and clearing it when they are equal.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies (a5)+ to d5, then copies 1 to d5, then returns.
- `check_struct_flag_one`: Loads a pointer from global variable offset 3964 and compares the byte at offset 2 of that structure with the value 1, setting d5 to 1 when they match and clearing it otherwise.
- `compare_structured_byte`: Calls the routine at address 3dc8, compares a byte from the address in a5 with the byte at offset 5590 of the structure in a4, and sets d5 to 1 when the bytes are equal, otherwise it clears d5.
- `and_bytes_set_flag`: Calls the routine at address 3f38, bitwise ANDs a byte from the address in a4 with a byte from the address in a5, stores the result in d0, and sets d5 to 1 when the AND result is nonzero, otherwise it clears d5.
- `compare_byte_lower_or_equal`: Loads a pointer from global variable offset 3964 and compares the byte at its base address with d5, clearing d5 when the loaded byte is less than d5 and setting d5 to 1 otherwise.
- `compare_offset_one_byte`: Loads a pointer from global variable offset 3964, advances it by one, and compares the byte at that address with d5, setting d5 to 1 when the bytes are equal and clearing it otherwise.
- `check_flags_then_store_one`: Loads a byte and a word from a global variable, and if both equal 255 it clears d5 and returns; otherwise if the byte at offset 24906 of a structure in a4 is nonzero it returns, and if it is zero it stores 1 in d5 and returns.
- `compare_byte_then_set_flag`: Loads a structure pointer into a4 and compares the byte at offset 3 with 1, storing 1 in d5 if they match and clearing d5 otherwise before returning.
- `compare_pointer_byte_then_set`: Reads a word from a pointer in a5 and advances it, then compares the following byte with the value at offset 7406 of that address, storing 1 in d5 when they differ and clearing it otherwise.
- `check_globals_then_set_flag`: Compares the byte at offset 8815 with 255 and the word at offset 8816 with 65535, storing 1 in d5 when either check fails and clearing d5 when both pass before returning.
- `check_two_flags_then_set`: Compares the byte at offset 11016 with 1 and the byte at offset 8815 with 255, storing 1 in d5 when the first is not 1 or the second is not 255 and clearing d5 otherwise.
- `match_sequence_then_store_value`: Compares the byte at offset 13479 with 6 and returns with d5 cleared if it differs; otherwise it indexes three tables using words from a5, and if all three match it clears a flag, tests it, and stores 2296 in a structure before setting d5 and returning.
- `compute_scaled_table_entries`: Calls two routines, then stores a scaled value derived from d2 and d5 into a table at offset 270 and another scaled value into the same table at offset 350 before returning.
- `store_bytes_then_return`: Calls a routine, then stores the byte in d2 at offset 670 of a structure in a4, the byte in d5 at offset 710, and 255 at offset 750 before returning.
- `store_word_and_loop`: Loads the next word from the address in a5 into d6, sets up a pointer to a global variable, calls a routine that stores that byte and loops while decrementing a counter, then returns.
- `store_word_and_loop_offset`: Loads the next word from the address in a5 into d6, sets up a pointer to a global variable, increments that pointer by two, calls a routine that stores the byte and loops while decrementing a counter, then returns.
- `call_two_routines`: Calls two routines in sequence and then returns.
- `load_and_dispatch_arithmetic`: Calls a routine, loads a byte pointer from a5 into a2 and a word from a5 into d6, then branches to a dispatcher that applies the selected arithmetic operation to the byte.
- `apply_arithmetic_to_byte`: Selects an operation by the byte in a2, then applies addition, subtraction, multiplication, division or modulo to the byte pointed to by a4 using the byte in d6, storing the result back and returning early with 255 or 0 on overflow or divide-by-zero.
- `store_byte_via_pointer`: Calls two routines in sequence, then stores the byte in d5 through the pointer in a4, and returns.
- `load_word_and_store_byte`: Loads a word from a5 into d5, then branches to a routine that stores that byte through the pointer in a4.
- `clear_global_byte`: Clears a byte in a global variable to zero and returns.
- `increment_global_byte`: Increments a byte value at a fixed offset in global memory by one.
- `load_byte_and_dispatch`: Calls a routine, loads a byte pointer into a2 and pushes the current pointer in a4 on the stack, calls the routine again, loads a byte from a4 into d6, restores the pointer, then branches to a dispatcher that applies the selected arithmetic operation.
- `increment_and_dispatch`: Adds two to the value in a5, calls two routines, then branches on whether a global variable is zero to store zero and call one routine or call another routine, and finally calls a routine with the incremented value and returns.
- `clear_and_store_byte`: Calls a routine, clears d5, then branches to a routine that stores that byte through the pointer in a4.
- `clear_struct_byte_a6`: Clears a byte at offset 52 of the structure in a6 and returns.
- `store_word_from_d5`: Calls a routine, stores the word in d5 into a global variable, and returns.
- `store_value_after_call`: Calls another routine, then stores the 16-bit value from d5 into a global variable, and returns.
- `read_write_bytes_conditional`: Reads four consecutive bytes from a global variable, calls a routine, writes two of those bytes back into the structure at computed offsets, stores one of them at offset 3, and if a byte read earlier is zero it toggles a flag and calls another routine before returning that byte.
- `load_and_branch_routine`: Loads a word and an address from consecutive locations in the array pointed to by a5 into d7 and a2, copies a2 into d1, loads a word from a table into d6, places it into a2, and branches to another routine.
- `load_call_and_branch`: Loads a word from the array pointed to by a5 into d7, calls a routine, clears d2, loads an address from the next location into a2, copies a2 into d1, loads a word from a table into d6, places it into a2, and branches to another routine.
- `set_and_branch_routine`: Sets d2 to three, loads a word and an address from consecutive locations in the array pointed to by a5 into d7 and a2, copies a2 into d1, loads a word from a table into d6, places it into a2, and branches to another routine.
- `store_all_ones_global`: Stores the word value of all ones into a global variable and returns.
- `store_byte_from_stack`: Loads two global pointers into a2 and a4, fetches a word from the stack pointer into d5, calls a routine, then stores that byte through the pointer in a4 and returns.
- `init_struct_from_indexed`: Calls a routine, then multiplies a masked value from the stack by 32 to form a table index, sets a byte at offset 1070 of the structure in a2 to one, stores the masked value at its start, and clears the following bytes.
- `increment_byte_offset1`: Increments a byte stored at offset 1 of the structure pointed to by the global variable and returns.
- `apply_arithmetic_twice`: Fetches a byte from the stack into a2 and calls a routine, then loads a pointer, advances it by four, applies an arithmetic operation to the pointed byte, advances by one, and applies the operation again before returning.
- `check_flag_then_call`: Checks a global byte and returns early when it matches a value, then reads a word from a pointer and calls a routine with it before returning.
- `compare_global_word`: Compares a global word at offset 6262 with the value 65535 and branches away when they differ, otherwise it branches to another routine.
- `subtract_and_branch`: Subtracts a word value from a register, branches if result is non-zero, and increments another register.
- `store_and_call`: Stores a word value from a global table to a specific memory location, then calls two routines in sequence.
- `apply_arithmetic_to_byte_2`: Fetches two words and a pointer from the stack, loads a byte from a table indexed by a value, then applies equality, addition, subtraction, multiplication, division or modulo to the pointed byte with clamping, and returns.
- `call_store_byte_offset`: Calls a routine, copies a byte into d1, loads a zero into d0, copies another byte into d0, sets up a pointer from it, loads a global variable pointer, calls two more routines, and stores a byte into the structure at offset 470 before returning.
- `pack_comparison_flags`: Calls a helper routine, then compares two pairs of 16-bit values at offsets 270 and 350 and packs a 3-bit flag derived from the sign and magnitude comparison into a byte at offset 470 of the global structure.
- `set_flag_call_return`: Returns immediately when two global words are unequal, otherwise sets a global byte to one, calls a routine, clears that byte, and returns the value from the routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a5)+ to d5, then returns.
- `copy_scaled_table_byte`: Loads a word from memory through a5, scales the global pointer by 32, reads a byte at offset 5790 of the resulting address, stores it at offset 950 of the global structure, and writes 16 to offset 8470.
- `set_flag_after_helper`: Calls a helper routine, then sets bit value 1 at offset 1070 of the address given by the returned value and returns.
- `store_word_low_byte`: Loads a word from memory through a5, stores its low byte at offset 2 of the global structure and returns.
- `clear_byte_offset2`: Clears the byte at offset 2 of the global structure and returns.
- `accumulate_and_store_bytes`: Calls a helper routine, adds a loaded word to two 16-bit values at offsets 270 and 350 while clearing their low bits, then stores two bytes from consecutive words at offsets 430 and 470 of the derived address and returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `write_byte_to_offset`: Calculates an address using a global variable and a fixed offset, then writes a byte value to that address.
- `store_word_low_byte_high`: Loads a word from memory through a5, stores its low byte at offset 470 of the global structure and returns.
- `store_pair_then_call_helper`: Reads a word and a byte from memory through a5, stores them at offsets 7150 and 7170 of the pointed address, calls a helper routine with the following word, restores the pointer, and returns.
- `adjust_globals_and_call`: Sets two global variables to specific values, calls another routine with an adjusted parameter, then resets one of the globals.
- `store_word_low_byte_low`: Loads a word from memory through a5, stores its low byte at offset 430 of the global structure and returns.
- `update_byte_pair`: Searches for a byte value in a table, updates matching entries, or adds new entries if no match is found.
- `find_or_add_byte_pair`: Computes a value from a stack byte using a comparison character, multiplies it by another masked value, then searches a table of byte pairs for a matching first byte and stores the pair, otherwise appends it and increments a counter.
- `find_max_byte_pair`: Searches a table of byte pairs for the largest second byte, stores the matching first byte into a global structure at its start with a byte at offset 1070 set to one, and returns.
- `init_struct_from_d5`: Sets the byte at offset 1070 of the structure in a2 to one, copies the byte from d1 into the start of the structure in a4, and clears the next three bytes.
- `store_byte_call_routines`: Stores the byte at the address in a5 into a global byte at offset 10, then calls three routines in sequence before restoring the registers and returning.
- `store_popped_byte_offset3`: Pops a word off the stack into d5, loads a pointer to a global variable, and stores that byte at offset 3 of the structure.
- `store_popped_byte_offset7110`: Pops a word off the stack into d5, loads a pointer to a global variable, and stores that byte at offset 7110 of the structure.
- `set_flags_and_clear`: Sets a specific byte in global memory to one, copies another byte from global memory, and clears three consecutive bytes.
- `return_coded_string_result`: Calls a routine, reads a word from a register, and returns that word when a global byte equals one, otherwise it calls another routine and returns its result.
- `set_flag_and_init_tables`: Sets a global byte at offset 34 to one, then calls a routine, reads a word from a table via a5, sets a global word at offset 30 to -1, and calls two more routines before returning.
- `call_and_restore_registers`: Saves all general-purpose registers to the stack, calls a routine, restores the registers, and returns.
- `process_string_flag_loop`: Copies a byte from a register into two global variables, calls a routine, and clears two global variables.
- `clear_globals_then_call`: Clears three global bytes at fixed addresses, then calls four routines in sequence before returning.
- `compare_store_loop_call`: Compares a byte from a global variable with another and returns if they differ, otherwise stores the byte, loads a pointer, calls a routine four times with two different addresses, increments that pointer by two in a loop until it reaches sixteen, clamps it to zero if it exceeds sixteen, stores the result, restores the stack, and returns.
- `check_flag_and_queue`: Returns immediately unless a byte at a pointer equals one, in which case it loads two words from the structure, copies a pointer, calls a routine, and returns.
- `fill_data_area_from_stack`: Multiplies a masked stack value by 22 to index a data area, sets a byte there to one, stores two words, then copies seven more words from the stack into consecutive locations before returning.
- `clear_global_counters`: Loads zero into d5 and stores it into the bytes at offsets 21568, 21590, 21606, and 21620 of a global variable, then stores zero into the word at offset 21566.
- `store_registers_into_globals`: Calls a routine, stores a register into a global word at offset 42880, then loads two consecutive words from a register and stores them into global words at offsets 42884 and 42886 before returning.
- `prepare_and_dispatch_index`: Calls a routine, copies a register into two others, sets a register to a negative marker, reads two bytes from a structure, loads a word from a register, sets a register to two, and calls a dispatch routine before returning.
- `copy_registers_then_dispatch`: Calls a routine, copies two registers into two others, loads a word from a register, sets a register to two, and calls a dispatch routine before returning.
- `set_registers_then_dispatch`: Calls a routine, copies a register into another, sets a register to a negative marker, sets a register to three, and calls a dispatch routine before returning.
- `load_bytes_dispatch`: Loads two bytes from a global variable through a computed pointer, then returns immediately when a global flag bit is set.
- `set_value_dispatch`: Sets d2 to 6 and calls a dispatch routine, which returns immediately when a global flag bit is set and otherwise clears registers, sets d2 to 2, and dispatches to another routine.
- `index_table_into_struct`: Calls another routine, indexes a table by a value from the stack, and stores the resulting byte at offset 8696 of the structure in a4.
- `process_table_entries`: Walks a table decrementing a counter, calls a handler looked up through the current entry, increments that entry when a flag is clear, and returns when the counter underflows.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d5, then returns.
- `call_indexed_routine`: Calls a routine, multiplies a byte by four, loads a pointer from a table at that index, calls the pointed routine, clears a register, and returns.
- `store_indexed_table_byte`: Multiplies a value from d5 by 32, adds a masked second value, loads a byte from that computed table index, and stores it at offset 950 of the structure in a4.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `set_two_and_branch`: Loads the value two into d5 and branches into another routine without returning.
- `set_four_and_branch`: Loads the value four into d5 and branches into another routine without returning.
- `store_structure_bytes`: Loads the byte at offset 26363 of the structure in a4 into d2 and the byte at offset 26403 into d5, then stores both bytes into a global variable at consecutive offsets.
- `increment_byte_offset_270`: Reads a byte at offset 270 of the structure in a5, adds one to it, and stores the result back to the same location, then returns.
- `increment_global_byte_by_two`: Increments a byte value at a fixed offset in global memory by two.
- `decrement_global_byte`: Decrements a byte value at a fixed offset in global memory by one.
- `decrement_byte`: Decrements a byte value at a specific offset in a data structure and returns.
- `load_and_branch`: Loads an immediate value into a data register and unconditionally branches to another location.
- `store_masked_byte_and_call`: Masks a byte by three, stores it at offset 214 of a structure, calls a routine, clears a register, and returns.
- `set_flags_and_load_one`: Stores the value sixteen into the byte at offset 8470 of the structure in a4, stores the value one into the byte at offset 1070 of the same structure, loads one into d5, and returns.
- `adjust_value_and_store_pair`: When d5 is nonzero it masks d5, shifts it left by four bits, combines the result with d5, subtracts sixteen, and writes the original byte from d2 and the adjusted value into the words at offsets 7184 and 7185 of the structure in a5; otherwise it subtracts the low nibble of d2 from the byte at offset 8470 of the structure in a4.
- `store_byte_and_load_two`: Stores the byte in d5 into the byte at offset 1014 of the structure in a4, loads two into d5, and returns.
- `save_regs_dispatch`: Saves all data and address registers on the stack, saves a4 in d7, sets d4 to 255, sets the high byte of d7 to 255, loads two bytes from a structure, loads a byte from a table, loads another byte, sets d2 to 2, and calls a dispatch routine before restoring the registers and returning.
- `compare_store_bytes`: Loads two bytes from a global variable through a computed pointer, then compares bytes at offsets 750, 670, 790 and 710 of a structure and stores the loaded bytes when they match.
- `adjust_range_values_from_table`: Loads two bytes from a global variable, runs them through a routine that returns a 16-bit pointer, and stores that pointer in a global variable.
- `search_table_forward_bracket`: Decrements a counter loaded from a global variable and loops through a table of pairs of bytes at offsets 8938 and 9018, comparing each pair against the values in d2 and d5; when a pair brackets both values it returns immediately, and when the counter underflows it loads negative one into a2 and returns.
- `search_table_reverse_bracket`: Loads a counter from a global variable, multiplies it by two, and loops backward through a table of pairs of bytes at offsets 8938 and 9018, comparing each pair against the values in d2 and d5; when a pair brackets both values it returns immediately, and when the counter underflows it loads negative one into a2 and returns.
- `adjust_and_rank_points`: Adjusts a byte at offset 470 of the structure in a4 by XORing it with 3 and adding the constant 248, then clears a global flag byte and calls the distance-ranking routine.
- `compute_distances_and_rank_points`: Reads two bytes from the structure in a4, computes the squared distance from each entry in a table of three points to those coordinates, stores each sum into a global array, then repeatedly finds the smallest remaining value in that array, clears it, and records a decrementing index into a second global array before restoring a4 and returning.
- `add_offsets_and_store_pointer`: Adds the byte at offset 23630 of the structure in a2 to a value loaded from a global variable and the byte at offset 23634 to another global byte, then calls a helper routine.
- `index_table_or_adjust_values`: Computes a signed difference between two values and builds a one-hot index by negating and testing each operand, then reads a byte from a table indexed by that value.
- `lookup_byte_from_offset_470`: Reads a byte at offset 470 of the structure in a4, uses it as an offset into a table to fetch a byte, stores that fetched byte back into the byte at offset 470 of the structure in a4, and returns.
- `lookup_byte_from_offset_470_alt`: Reads a byte at offset 470 of the structure in a4, uses it as an offset into a different table to fetch a byte, stores that fetched byte back into the byte at offset 470 of the structure in a4, and returns.
- `check_ranges_and_store_pair`: Shifts the value in d6 right by three bits and compares it against two bytes in the structure in a4; when both match it stores the values from d6 and d7 into the words at offsets 270 and 350 of the structure in a5 and returns.
- `skip_neg_ones_and_call`: Stores the byte in d6 at offset 8510 of the structure in a2, then decrements a counter and loops.
- `copy_ten_words`: Clears a byte at offset 34 of the structure in a2, then copies ten 16-bit words from the source pointed to by a4 into a destination computed from a2, using a counter initialized to ten.
- `classify_pair_offsets`: Walks a table of paired byte records, comparing each entry with the one at a computed offset and writing a small classification byte into a global variable based on how their fields differ, then stores that classification into each record's status field at offset 1110 and returns.
- `sort_pair_offsets`: Computes signed differences of the byte pairs and stores their absolute values with signs into a global variable, then walks a byte table until it finds a value of 255, advancing a pointer by the stored offsets.
- `update_table_flags`: Iterates over a decreasing counter, reads a byte from a table indexed by two other table entries, compares its masked value against a stored flag, updates that flag, and sets a byte at offset 1070 when the value is nonzero.
- `count_string_store`: Calls a routine, then scans a null-terminated string counting its characters.
- `decrement_and_store_marker`: Decrements a global byte counter and, when it reaches zero, waits until bit 6 of the status byte at offset 2 of the structure in a6 becomes clear, then calls another routine and stores the value 65535 into the 16-bit value at offset 2 of the same structure.
- `wrap_string_tokens`: Scans a null-terminated string counting characters, replacing spaces after a run of sixty characters with a pipe marker, and inserting a null terminator when a pipe is encountered at the first position.
- `append_byte_to_array`: Stores the byte from d5 into a global array at the index held in a global counter, then increments that counter while it stays below one hundred and sixty.
- `scan_string_store_chars`: Clears two global counters, then scans a string in a4 handling special markers by calling routines and copying characters into a global array while advancing through the buffer.
- `handle_special_char`: Reads a byte from the address in a4 and returns immediately unless it is the letter o, the letter O, or the hash character, in which case it calls the string-conversion routine.
- `convert_table_values_to_string`: Loops five times over consecutive 16-bit entries in a global table, subtracting each value from a 16-bit register and wrapping it back into range when it goes negative.
- `skip_bracketed_sections`: Reads a global word, counts the opening brackets at the start of the pointed string, and when brackets are present it skips past the bracketed sections before calling the string scanner.
- `compare_register_d5`: Compares register d5 with the value 65535 and branches away when they differ, otherwise it returns.
- `store_byte_then_index`: Stores a register into a global byte, indexes a pointer by a doubled value, and returns when the pointed byte matches a marker.
- `skip_sections_then_scan`: Advances a pointer through a string, skipping each bracketed section up to a limit, then calls the string scanner when the terminator is not reached.
- `scan_until_character`: Scans memory starting at an address calculated from a4 and d5, advancing the pointer until it finds a character that is not '['.
- `increment_and_return`: Increments a data register and returns the stack contents as a value.
- `process_coded_string_loop`: Clears a global word and sets three consecutive global bytes to 32, then loops over a coded string calling the conversion routine for each character.
- `process_and_shift_table_records`: Walks a table of 16-bit records from the last entry toward the first, combining each record's two values with a stored sign byte and shifting the results right by three.
- `bubble_sort_entries`: Repeatedly compares adjacent entries of a table using several field offsets, swapping pairs when they are out of order, until a pass makes no swaps or the outer counter reaches zero.
- `clamp_globals_then_call`: Reads two global words, clamps each into the range zero to fifty-six, then calls a routine with the clamped values before returning.
- `adjust_globals_call_routines`: Returns immediately when two global words match both inputs, otherwise increments those globals by one in each direction and calls three routines in a loop until they match again.
- `index_and_pack_table`: Reads a byte from a table indexed by a global variable and passes it to another routine, then repeats this for 64 consecutive entries stored as 32-bit words.
- `walk_table_records`: Clears a global word, then walks a table of records in nested loops.
- `load_entry_fields`: Compares a passed byte against a global value, then either reads two fields from a table entry selected by that byte or stores the constant 255 into two consecutive global bytes.
- `load_entry_fields_2`: Compares two incoming bytes against global values and returns without effect if they do not match, otherwise it clears a flag and loads two fields from a table entry selected by a structure byte.
- `build_and_queue_buffer_struct`: Computes a table index from a2 and d7, then writes a 16-bit value and a modified copy into a structure in a6 at offsets 66 and 64 along with three 32-bit values at offsets 68, 96 and 100.
- `restore_registers_and_return`: Restores registers d0 to d7 and a0 to a5 from the stack and returns the original values of d0 and d1 to the caller.
- `configure_structure_a`: It increments a global byte counter, then waits in a loop until bit 6 of the byte at offset 2 of the structure in a6 becomes clear.
- `configure_structure_b`: It increments a global byte counter, then waits in a loop until bit 6 of the byte at offset 2 of the structure in a6 becomes clear.
- `wait_bit6_clear_store`: Increments a global byte counter, then waits until bit 6 of the status byte at offset 2 of the structure in a6 is clear, otherwise it returns.
- `conditionally_configure_structure`: It compares a global byte to one and returns immediately if they match.
- `combine_table_words_into_buffer`: Computes a pointer into a table using the passed-in values, then reads 45 consecutive 32-bit words from that table and ORs them together in groups of five, storing each 32-bit result into consecutive 32-bit slots of a global buffer.
- `assemble_and_store_word_block`: Resolves a source pointer from a table using an index byte, then ORs together groups of five 16-bit words from that source into a contiguous block of 16-bit words.
- `copy_byte_pairs_into_table`: Copies 32 consecutive pairs of bytes from the source buffer pointed to by a4 into a table of 16-bit words, using the value in d1 as an index offset, and stores the results at consecutive 16-bit word addresses.
- `call_flag_loop_shifted`: Loads a byte from the structure in a1 into d7, adds 8, then calls the flag routine with a divisor derived from the value in a2.
- `clear_and_set_flag_bytes`: Clears bit 0 of a byte at offset 0 of a structure in a5 and bits 0 and 2 of the bytes at offsets 32, 64 and 96, then sets bit 0 of the byte at offset 128, all using a mask derived from d7, and repeats this for a block count in d6 before restoring registers and returning.
- `accumulate_set_flag`: Accumulates values from a short array in a loop, then sets a flag byte at the end of a structure.
- `compare_strings_case_insensitive`: Compares two null-terminated strings one character at a time, converting any uppercase letters to lowercase, and returns -1 when the strings match or 0 when they differ or a string ends.
- `load_table_call_routines`: Loads a table address into a0, then calls six routines in sequence before restoring the registers and returning.
- `toggle_flag_bits_in_struct`: Sets or clears bit 0 of a byte at offset 0 of a structure in a5 depending on bit 0 of d5, then repeats the operation for the bytes at offsets 32, 64, 96 and 160 using the same bit position from d0.
- `count_set_bits`: Counts bits set in a data structure at multiple offsets, accumulating a total count based on conditional checks.
- `initialize_and_process_loop`: Initializes a register with zero, loads an address from a fixed location into a5, clears a4, calls another routine, sets d5 to seven, calls two more routines, then branches back to the beginning of this routine.
- `write_bit_data`: Processes bits from a data structure through multiple iterations, accumulating results and writing bytes to memory locations with incrementing offsets.
- `process_flags_loop`: Processes a series of flag operations in a loop, calling another routine for each iteration and updating a data pointer.
- `process_flags_short_loop`: Processes a shorter series of flag operations in a loop, calling another routine for each iteration and updating a data pointer.
- `initialize_and_compare`: Initializes specific memory locations with predefined values, performs comparisons, and updates memory based on comparison results.
- `process_data_with_loop`: Processes data through a loop of flag operations, handling conditional branches and updating pointers during iteration.
- `validate_and_update_flags`: Validates data against a lookup table, updates flags based on comparisons, and performs additional flag toggling operations.
- `toggle_flags_repeatedly`: Multiplies the incoming word by four, subtracts one, and returns immediately if that result is negative.
- `copy_and_shift_coded_bytes`: Loads a source table pointer from a global variable indexed by d5, then copies 40 bytes from that table into a destination block, masking and shifting each byte according to bit 0 of d0, and advances the destination by 40 bytes each time.
- `clear_and_set_flag_words`: Clears a 16-bit field in a global variable and sets the corresponding bits in five 16-bit fields of a structure at offsets 0, 32, 64, 96 and 128, then zeroes the remaining fields of that structure in a loop before returning.
- `wait_and_configure_struct`: Increments a global counter, waits until bit 6 of a status byte at offset 2 of a structure in a6 becomes clear, then stores a negative long at offset 68 and several fixed constants at offsets 64, 80, 84 and 88 before decrementing the counter and returning.
- `wait_and_configure_struct_alt`: Increments a global counter, waits until bit 6 of a status byte at offset 2 of a structure in a6 becomes clear, then stores a negative long at offset 68 and fixed constants at offsets 64, 80, 84 and 88 before decrementing the counter and returning.
- `walk_string_and_call`: Walks a byte string in a4 until it hits a zero or pipe character, sets a flag byte accordingly, then calls the string-processing routine with an adjusted pointer.
- `process_coded_string`: Walks a byte string in a4 counting characters until it hits a zero or a pipe byte, then computes a destination pointer from the remaining length.
- `store_long_if_nonzero`: Reads a long from offset 12 of a structure in a0 and stores it into a global variable only when that value is not zero, then returns.
- `init_and_store_diff`: Calls a global-counter clearing routine, then sets up three table pointers and a set of constants before calling a structure-initializing routine.
- `init_and_store_diff2`: Sets up three table pointers and a set of constants, then calls a structure-initializing routine.
- `clear_global_words`: Clears 30 consecutive 16-bit words of a global variable by writing zero to each in a loop before returning.
- `decode_table_coded_rows`: When a block counter at offset 8 of a structure in a0 underflows, copies a pointer from offset 6 to that counter and unpacks rows of 16 words from a byte stream, each value being a byte plus or minus a lookup-table entry, written at a fixed row stride, then marks the stream finished once the row count runs out.
- `store_diff_nibbles_loop`: Initializes a 22-byte structure at a0 with fixed header fields, then loops d0 times copying two input arrays and writing the low nibble differences, high nibble differences, and byte differences as pairs of bytes, advancing each pointer by its stride, and finally sets the first byte to 255.
- `dispatch_by_index`: Returns immediately when a global flag bit is set, otherwise it clears several registers, adjusts a value in d2, and dispatches to a routine selected by that value.
- `subtract_and_or_bytes`: Subtracts a value from memory and performs bitwise OR operations with multiple constants on a data register before returning.
- `load_indexed_table_byte`: Compares register d7 with the value 65535 and branches away when they are equal, then loads a byte from a table indexed by register d5 into register d0 and branches away when that byte is zero, otherwise it branches to another routine.
- `test_flag_then_call`: Tests a global byte, and when it is not zero it calls a routine before returning.
- `toggle_global_bit_zero`: Toggles bit 0 of a global byte at offset 9c58 and, when the previous value of that bit was zero, calls a routine before returning.
- `check_conditions_then_call`: Compares a global long against a constant and tests a global byte, and when both conditions hold it calls two routines before returning.
- `conditionally_call_routines`: Tests a global byte at offset 5c5a and, when it is nonzero, calls three routines in sequence before returning; otherwise it clears that byte, calls two routines, invokes an external library routine through a pointer, and calls one more routine before returning.
- `clear_long_globals`: Loops eight times writing a zero long word to a global long array, then writes a nonzero constant to a global long variable and returns.
- `clear_word_globals`: Loops 59 times writing a zero word to a global word array and returns.
- `disable_dma_channels_3`: Clears a global byte at offset 242a, disables all DMA channels by writing the value 15 to the DMA control register, then loops four times decrementing a counter until it reaches zero before returning.
- `match_then_set_globals`: Loops four times comparing a register to a global word, and when a match is found it stores a negative word and a zero byte into the global area before calling a routine and returning.
- `compare_set_globals_loop`: Loops four times comparing a register to a global word and, when equal, sets a global byte and three adjacent bytes, then branches into another routine.
- `select_min_dispatch`: Computes the absolute differences of two register pairs, negating each when negative, then selects the smaller value.
- `empty_branch`: Empty routine that immediately branches to the following instruction.
- `jump_to_next_routine`: Jumps to the routine at the next address without returning.
- `clamp_index_via_comparison`: Compares the value in d5 with 8 and, when d5 is at least 8, branches to a routine that follows it, otherwise it falls through to repeat the same comparison.
- `index_table_shift`: Clamps the value in d5 to a maximum of 6, indexes a byte table using that value, multiplies by the index, divides by 8, stores the result back in d5, and returns zero.
- `search_table_then_write`: Loops over global bytes, and when a non-zero byte is found it searches a table for a matching value, then writes that value into a global array and calls two routines before continuing.
- `check_globals_write_array`: Checks two global bytes, and when they satisfy a condition, calls two routines and then writes a set of bytes and a word into a global array in a four-iteration loop.
- `conditionally_clear_loop`: Loops four times writing 255 to a byte in a global array, clearing it when a long word at a fixed address equals zero, and advancing the address by 148 each iteration, then returns.
- `search_global_table`: Loops 20 times checking a byte in a global array, returning -1 when the byte is nonzero, and otherwise comparing a global word against the input value and returning 0 when they match.
- `read_flag_and_bytes`: Reads a sign flag and returns a nonzero constant when it is negative, otherwise reads two byte values from a base address, masks off their sign bits, and returns them.
- `count_zero_globals`: Loops four times counting zero bytes in a global array, then compares that count against a global byte and returns -1 when the count exceeds it, otherwise returning zero.
- `mark_matching_table_bytes`: Stores a negative marker word in a global structure, scans four bytes of a table for values matching a byte selected by a global index, and counts how many table bytes are non-zero.
- `restore_regs_set_flag`: Restores registers d1 through d4 from the stack, sets d0 to 255, and returns.
- `match_and_mark_table`: Iterates four bytes of a table, and for each byte that is positive and matches a word in a global variable, it marks the table byte, overwrites the matching global word, and writes four fixed values into consecutive global bytes.
- `fill_struct_from_tables`: Reads a byte from a table indexed by a5 and a byte from a second table indexed by that value, then in a four-iteration loop copies a byte from a source table into a structure at offset 12, sets flags at offsets 0, 4 and 8, stores the two bytes at offsets 16, 24 and the index word at offset 28, returning early if the flag at offset 12 is set or the source byte is zero.
- `fill_struct_from_tables_2`: Reads a byte from a table indexed by a4 and a byte from a second table indexed by that value, then in a four-iteration loop copies a byte from a source table into a structure at offset 12, sets flags at offsets 0, 4, 20 and 8, stores the two bytes at offsets 16, 24 and the d7 word at offset 28, returning early if the flag at offset 12 is set or the source byte is zero.
- `copy_bytes_conditional_2`: If a global byte is nonzero, it sets a flag bit and loops over four bytes, clearing each nonzero byte and calling a handler routine for it.
- `copy_bytes_conditional`: If a byte in a global variable is not zero, it copies four bytes from that global variable into another location, advancing the destination by 148 bytes each iteration, then returns.
- `scan_table_store_ptr`: Stores a pointer at offset 128 of a structure, then scans a table looking for a byte in a specific range.
- `split_byte_ascii`: Multiplies a register by 2, then splits a byte into two nibbles and converts each into an ASCII digit by adding 48.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `copy_rle_record_from_buffer`: Loads a pointer from a global variable and reads a byte from a buffer, branching away when the byte is zero.
- `compute_offset_value`: Reads two bytes from a memory location pointed to by a3, processes them through bit shifts and arithmetic operations, then uses the result as an offset to load a memory address into a5.
- `fill_words_zero_loop`: Fills 192 consecutive words in a global array starting at offset 150 with zero, using d1 as a loop counter, then branches into another routine.
- `load_ptr_then_branch`: Loads the address of a global array into a5 and immediately branches into another routine without modifying any data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `reset_audio_dma_state`: Searches a table of 128 bytes for the largest value, then uses its position to index a second table and stores pointers into it in a third table.
- `update_audio_flag_byte`: Saves registers on the stack, increments a global byte at offset 62, and reads a byte from a table to compare against another table byte.
- `touch_audio_channels`: A 37-instruction routine that touches the four left audio channel registers and calls a 91-instruction routine which itself calls another routine and accesses the DMA control register; it is invoked by two unnamed routines.
- `indeterminate_routine`: A 91-instruction routine that calls another routine and touches the DMA control register, but whose operations cannot be described from the available information.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `route_message_by_d0_flags`: Reads a 16-bit value from the low word of d0 and branches on its bits 0, 4, 5, and 7 to clear or set fields of a structure in a2 and flags in the byte at offset 7 of the structure in a0, then stores the 16-bit value 18246 and the value 1 at offsets 0, 2, and 6 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_dma_channels`: Disables all DMA channels by writing a value with bit 15 set to the DMA control register, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_store_word`: Tests bit 10 of the 32-bit value in d0 and, if it is clear, stores the word value 1024 at offset 156 of the structure in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_masked_value`: Loads a word from a table indexed by a register into a6 into a0, stores it at offset 16 of the structure in a6, restores the caller's registers, reads a word at offset 2 of the structure in a6, masks its high nibble, compares it with the decimal value 6128, and returns if they are equal.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_const_test_bit`: Stores the decimal constant 1024 into the 16-bit value at offset 96 of the structure in a0, then tests bit 2 of the byte at offset 28 of the same structure and returns if that bit is clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_state_and_wait_flag`: Copies a four-byte value and a word from the structure in a6 into the memory pointed to by a5, then repeatedly waits for a flag bit at offset 28 of the data in a4 to be set before calling a handler.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_bit_then_call`: It tests bit 2 of a byte at offset 28 in a0 and returns if it is clear, otherwise it loads an address into a1 and calls a routine, then falls through.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_channels`: Sets bit 15 of the DMA control register to enable the DMA channels and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_on_bit_set`: Tests bit 1 of the byte at offset 28 in a0, and when that bit is set it loads an address and calls a routine from outside this code before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_table_byte`: Loads a byte from the table pointed to by a0 into a1, advances a0, and returns when the value is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_call_return`: Returns immediately when the condition flag is clear, otherwise loads an address and calls a routine from outside this code before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_structure_globals`: Copies a 32-bit value and two 16-bit values from the structure in a6 to a global variable via the pointer in a5, then sets bit 14 of a global variable and jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_earlier_routine`: Jumps unconditionally to a routine located a few instructions earlier in memory, transferring control there without returning.
- `store_value_check_flag`: Stores the decimal constant 256 into the word at offset 12 of the structure pointed to by a0, then checks bit 0 of the byte at offset 28 of that structure and returns immediately if that bit is clear, otherwise it loads a pointer into a1 and calls a routine before returning.
- `save_registers_and_load_structure_field`: Pushes the contents of register a0 and register d0 onto the stack, then loads the value stored at offset 12 from the structure pointed to by register a1 into register a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_structure_state`: Checks a byte at offset 29 of the structure in a6 and, if it is nonzero, copies the 32-bit value at offset 4 and the 16-bit values at offsets 8 and 16 of that structure into a structure pointed to by a5, then sets bit 14 of a global variable before jumping to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_call_routine`: Checks bit 0 of the byte at offset 28 of the structure pointed to by a0 and returns immediately if that bit is clear, otherwise it loads a pointer into a1 and calls a routine before returning.
- `save_registers_and_load_offset`: Pushes the contents of registers a0 and d0 onto the stack and loads a2 with the value stored at offset 12 of the structure pointed to by a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_structure_fields`: Copies a 32-bit value from the structure pointed to by a5 to the one pointed to by a6, then stores two 16-bit fields from that structure into a global variable and sets bit 21 of that same global variable, before jumping to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_fall_through`: Loads a pointer into a1 and calls a routine, then falls through into the following routine without returning.
- `save_registers_and_load_offset`: Pushes the contents of registers a0 and d0 onto the stack and loads a2 with the value stored at offset 12 of the structure pointed to by a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_flag_status`: Copies the 16-bit value at offset 16 of the structure in a6 into the 16-bit value at offset 6 of the structure in a5, then sets bit 14 of the global variable at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_decrement_routine`: Calls a routine that pushes a0 and d0, decrements a 16-bit counter at offset 14 of the structure in a1 and checks its sign, then falls through into the following routine without returning.
- `push_regs_decrement_check`: Pushes the values of a0 and d0 onto the stack, decrements a 16-bit counter at offset 14 of the structure in a1, then checks the sign of the byte at offset 36 and, when it is non-negative, loads a 16-bit value from offset 38 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `merge_and_jump_routine`: Stores the 16-bit value of register d0 into a 16-bit global variable at offset 6 of the structure in a5, then loads a 16-bit value from offset 20 of the structure in a6, combines it with bit an address of a global variable using a bitwise OR, and jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_value_check_high_bit`: Stores the decimal constant 128 into the word at offset 12 of the structure pointed to by a0, then checks bit 7 of the byte at offset 29 of that structure and returns immediately if that bit is clear, otherwise it loads a pointer into a1 and calls a routine before returning.
- `restore_regs_rte`: Restores all general-purpose data registers and address registers from the stack, then executes a return from exception, so control passes back to the interrupted context.
- `push_regs_decrement_check`: Pushes the values of a0 and d0 onto the stack, decrements a 16-bit counter at offset 14 of the structure in a1, then checks the sign of the byte at offset 36 and, when it is non-negative, loads a 16-bit value from offset 38 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_flag_and_branch`: Loads a 16-bit value from offset 20 of the structure in a6 into d0, sets bit 15 of the global byte at a fixed address, and then branches away to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_call_and_jump`: Loads the address of a routine into a1, calls that routine, and then branches to a jump-table entry that passes control to a fixed address elsewhere in memory.
- `restore_registers_and_return_2`: Restores all eight data registers and seven address registers from the stack and returns to the caller.
- `decrement_and_update_offset_8`: Decrements the 16-bit value at offset 14 of the structure in a1, then, when the 16-bit value at offset 36 is negative, stores the sum of the 32-bit value at offset 28 and the zero-extended 16-bit value at offset 38 into the 32-bit location pointed to by the pointer at offset 8 of the structure in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_dma_setup_pointers`: Issues a no-op, calls an external routine, then loads a 16-bit value, sets bit 15 of it, and writes it to the DMA control register at a fixed address to enable DMA channels before calling the routine again and setting up pointers to the custom and chip memory regions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_jump`: Calls a nearby routine and then branches to a jump-table entry that passes control straight to a fixed address elsewhere in memory.
- `restore_registers_and_return_2`: Restores all eight data registers and seven address registers from the stack and returns to the caller.
- `decrement_and_update_offset_8`: Decrements the 16-bit value at offset 14 of the structure in a1, then, when the 16-bit value at offset 36 is negative, stores the sum of the 32-bit value at offset 28 and the zero-extended 16-bit value at offset 38 into the 32-bit location pointed to by the pointer at offset 8 of the structure in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_audio_dma_flag`: Sets bit 15 of a value loaded from a global variable, writes it to the DMA control register, waits one cycle, and calls the audio library routine, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_rte`: Restores all general-purpose registers from the stack and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_audio_dma_setup`: Sets bit 15 of register d0, writes it to the DMA control register, waits one cycle, and calls the audio library routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_dma`: Writes register d0 to the DMA control register, waits one cycle, and calls the audio library routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_update_structures`: Decrements a 16-bit counter at offset 14 of the structure pointed to by a1 and, when its value at offset 36 is negative, adds the 16-bit value at offset 28 to the 32-bit value at offset 28 and stores the result at offset 4 of the structure pointed to by a1, then copies the 16-bit value at offset 6 to offset 154 of the structure pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_channel_parameters`: Loads three audio channel volume and period values from a table and writes them into the custom sound registers, then calls the audio update routine and repeats the process for two more channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_structure_fields_2`: When the 16-bit value at offset 36 of the structure in a1 is negative, it adds the unsigned value at offset 38 to the 32-bit value at offset 28 and stores the result at the address in a2, subtracts half that value from the 16-bit value at offset 40, stores it at offset 39 of the same location, and copies the 16-bit value at offset 6 into offset 154 of the structure in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_audio_channel_params`: Loads base addresses of the Paula audio registers and a table of channel data, then writes the period and length values for three audio channels into the custom chip registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_channel_timing`: Computes channel timing values from a source structure based on a flag, then copies those values into the audio registers and increments a global counter by sixteen.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_four_channel_params`: Loads a table of channel data and writes the period and length values for four audio channels into the custom chip registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compute_half_subtract_store`: Subtracts half of the word at offset 28 of the structure in a1 from the value at offset 20, stores the result at offset 4 of the structure in a2, copies the word at offset 6 of a1 to offset 154 of the structure in a0, then jumps to offset 60.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_pointer_word`: Copies the pointer at offset 10 and the word at offset 14 of a table to offsets 212 and 212 of the structure in a5, then loads the table at offset 3416 into a6 and stores its pointer and word to offsets 192 and 196.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `combine_and_store_values`: Reads the 16-bit value at offset 40 of the structure in a1, subtracts the value in d1 after shifting it right by one bit, stores the result at offset 4 of the structure in a2, copies the 16-bit value at offset 6 of the structure in a1 to offset 154 of the structure in a0, then jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_pointer_word_2`: Loads the table at offset 3416 into a6 and stores its pointer and word to offsets 192 and 196 of the structure in a5, then loads the table at offset 3372 and stores its pointer and word to offsets 176 and 180.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `subtract_store_copy_word`: Subtracts the value in d1 from the value in d0, stores the result at offset 4 of the structure in a2, copies the word at offset 6 of a1 to offset 154 of the structure in a0, then jumps to offset 60.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_pointer_word_3`: Stores the pointer at offset 10 of the table in a6 to offset 192 of the structure in a5 and its word at offset 14 to offset 196, then loads the table at offset 3340 and stores its pointer and word to offsets 176 and 180.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_and_return`: Copies the word at offset 6 of the structure in a1 to offsets 154 and 156 of the structure in a0, then restores d0 and a0 from the stack and returns.
- `check_and_copy_words`: Tests a word value at offset 14 of the structure in a1, branches to offset 32 if the value is non-zero, otherwise copies a word from offset 6 of that structure to offset 154 of the structure in a0, and then copies a word from offset 2 of that structure to offset 150 of the structure in a0.
- `initialize_structure_data_2`: Writes a fixed value to a memory location pointed to by a2, sets a word at offset 4 of that location to 1, and copies a word from offset 6 of a1 to offset 156 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_pointer_word_4`: Stores the word at offset 14 of the table in a6 to offset 196 of the structure in a5, then loads the table at offset 3340 and stores its pointer and word to offsets 176 and 180.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_and_return_2`: Copies the word at offset 6 of the structure in a1 to offset 156 of the structure in a0, then restores d0 and a0 from the stack and returns.
- `check_and_copy_words`: Tests a word value at offset 14 of the structure in a1, branches to offset 32 if the value is non-zero, otherwise copies a word from offset 6 of that structure to offset 154 of the structure in a0, and then copies a word from offset 2 of that structure to offset 150 of the structure in a0.
- `initialize_structure_data_2`: Writes a fixed value to a memory location pointed to by a2, sets a word at offset 4 of that location to 1, and copies a word from offset 6 of a1 to offset 156 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_table_and_conditional_copy`: Stores two pointers and two words from a global table into offsets 176, 180, 176 and 180 of the structure in a5, increments a global variable by 16, and stores the word at offset 6 of a1 to offsets 154 and 150 of a0 when that variable is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_two_words_from_struct`: Copies a 16-bit value from offset 6 of the structure in a1 to offset 154 of the structure in a0, then copies the 16-bit value from offset 2 to offset 150 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_and_load_pointer`: Stores the 16-bit value from offset 14 of the structure in a6 into offset 180 of the structure in a5, then loads a table address into a6 and reads a pointer from offset 10 of that table into a0 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_two_words_offset_2_6`: Copies the 16-bit value from offset 2 of the structure in a1 into offset 150 of the structure in a0, then copies the 16-bit value from offset 6 into offset 156 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_pointer_and_store_word`: Loads a table address into a6, reads a pointer from offset 10 of that table into a0, and stores the 16-bit value from offset 14 of the table into offset 164 of the structure in a5 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_offset_6`: Copies the 16-bit value from offset 6 of the structure in a1 into offset 156 of the structure in a0 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counter_and_check_byte`: Reads a pointer from offset 10 of the table in a6 into a0, stores the 16-bit value from offset 14 of that table into offset 164 of the structure in a5, decrements a 16-bit counter at offset a fixed address, reads a byte from offset a fixed address, and branches when that byte is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_movea_rts`: Small routine of 3 instructions that copies (a7)+ to d0, then copies (a7)+ to a0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_and_or_register`: Checks a global byte and clears it when nonzero, then decrements a global counter and subtracts 16 from a 16-bit global while its byte is nonzero before clearing a global and ORing a constant into the value pointed to by a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_hardware_routine`: Checks whether a 32-bit global variable is zero and returns immediately if it is not, otherwise saves all registers, loads the hardware base address, calls another routine using that address and a table, restores the registers, and returns with zero in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_global_byte_and_return_zero`: Branches when d0 is zero, stores d0 into a global byte and clears another global byte, then decrements the stored byte and subtracts 16 from a 16-bit global while it is nonzero before testing a global byte and returning zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_memory_bits`: Performs a bitwise OR operation with a fixed value on the memory location pointed to by a4, then increments the pointer.
- `conditionally_init_custom`: If the zero flag is set it calls a routine located at offset 444, restoring the caller's registers afterwards.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counters_shift`: Decrements a byte counter at offset 3 of a global variable and, if it does not reach zero, decrements a word counter at offset 5 by 16.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_return_param`: Loads a nearby address into a1, copies a0 into a5, and calls a routine at offset 434.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counters_shift_2`: If the zero flag is clear it decrements a byte counter at offset 3 of a global variable and, when that does not reach zero, decrements a word counter at offset 5 by 16.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_return_zero`: Copies a0 into a5, calls a routine at offset 424, and restores the caller's registers from the stack.
- `call_routine_for_pointers`: Loads a pointer into a4 and calls a routine that copies a pointer from offset 8 of that structure into a5, then repeats this for three further pointers before loading a fourth pointer and falling through into the following routine without returning.
- `compute_offset_difference`: Copies a pointer stored at offset 8 of the structure in a4 into a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undocumented_routine`: A routine of 15 instructions entered from outside static flow; the decompiler timed out, so only the listed facts are known and no further behaviour can be described.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_store_difference_routine`: Calls a routine that copies a pointer from offset 8 of the structure in a4 into a5 and stores the word at offset 72 of that structure into offset 6 of the pointed-to data, then repeats this for two further pointers before loading a fourth pointer and falling through into the following routine without returning.
- `store_difference_test_word`: Copies a pointer stored at offset 8 of the structure in a4 into a5, stores the word at offset 72 of that structure into offset 6 of the pointed-to structure, and pushes d0 on the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undocumented_routine_2`: A routine of 14 instructions entered from outside static flow; the decompiler timed out, so only the listed facts are known and no further behaviour can be described.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_store_difference_pair`: Calls a routine that copies a pointer from offset 8 of the structure in a4 into a5 and stores the word at offset 72 of that structure into offset 6 of the pointed-to data, then repeats this for one further pointer before loading a fourth pointer and falling through into the following routine without returning.
- `enable_dma_channels_2`: Writes to the DMA control register with bit 15 set to enable the DMA channels, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_returns`: Performs a sequence of 13 instructions that touch no listed registers or calls, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `disable_dma_channels_2`: Clears bit 15 of the DMA control register at custom offset 0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_counter`: If the carry flag is set it returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clamp_and_store_value`: Subtracts a word at offset 226 of the structure in a4 from a register, clamps the result to zero when negative, and stores it at offset 8 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_if_global_zero`: Tests a global byte and returns immediately if it is nonzero, otherwise it restores the caller's registers and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_conditionally`: If a word at offset 38 of the structure in a4 is nonzero it returns, otherwise it stores a word from offset 6 of that structure at offset 154 of the structure in a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_and_return`: Returns immediately if the zero flag is set, otherwise it restores the caller's registers and returns.
- `process_masked_value`: Calls another routine, then reads a word at offset 2 of the structure in a6 and masks its low 12 bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_via_base_registers`: Copies a 16-bit value from the word at offset 6 of the structure in a4 to the word at offset 154 of the structure in a5, then returns.
- `set_flag_and_copy_word`: Stores the value 1 in the word at offset 14 of the structure in a4, then copies the 16-bit value from the word at offset 4 of that same structure to the word at offset 154 of the structure in a5, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `clear_nibble_and_flag`: Clears the low nibble of the byte at offset 2 of the structure in a6, then sets bit 2 of the 32-bit value stored through the pointer in a2 and calls another routine three times.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 154 into d4, then returns.
- `call_load_byte_pointers`: Loads a pointer into a4 and calls a routine that loads a byte from offset 52 of a global variable into d5, pops a pointer into a2 and a word into d6 from the stack, then repeats this for three further pointers before loading a fifth pointer and falling through into the following routine without returning.
- `check_dma_flag_then_branch`: Checks a flag at offset 58 of the structure in a4 and returns immediately if it is clear, otherwise reads a pointer stored at offset 20 of the same structure and branches to the routine that follows it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_unpack_command`: Clears a flag bit in a byte pointed to by a6 based on the low three bits of d0, then branches on the low four bits of d0 to update a global variable, decrement a counter at offset 24 of the structure in a4, or read and modify a value from a table, returning a result with certain bits set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_call_three_times`: Loads the address of a global variable into a4, calls a routine that fills it, reloads a4 with a second address, calls the same routine again, then loads a third address into a4 and returns without an explicit rts, running into the next routine.
- `check_pointer_decrement_flag`: Tests the long value at offset 58 of the structure in a4 and returns if it is zero, otherwise loads the pointer at offset 20 into a register and tests bit 0 of the byte at offset 48.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_store_byte`: Reads a byte from the register and shifts its high byte right by up to three bit groups, then stores the resulting byte at offset 2320 and offset 9312 of the structure in a0 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_set_flag_then_load_pointer`: Calls a routine that stores one in a register when another is below it, then loads a pointer from a global variable into a register before falling through into the next routine.
- `decrement_and_call_pointer`: Tests a byte at offset 58 of the structure in a4 and returns if it is zero, then loads a pointer from offset 20 into d0 and returns if that pointer is zero, copies it into a3, and decrements a byte at offset 48 in a loop until it reaches zero before calling a routine and returning.
- `store_byte_from_offset_one`: Tests the 16-bit value at offset 34 of the structure in a4 and, if it is zero, returns without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_byte_nibble`: Reads a byte from the value in d0 and, depending on its nibble, extracts a nibble from a byte at offset 3 of the structure in a6 or decrements a counter at offset 24 of the structure in a4, then stores a 16-bit value from the structure in a6 into offset 6 of the structure in a5 and returns a computed byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_decrement_and_call`: Loads the address of a global variable into a4, then tests a byte at offset 58 and returns if it is zero, loads a pointer from offset 20 and returns if it is zero, copies it into a3, and decrements a byte at offset 48 in a loop until it reaches zero before calling a routine and returning.
- `store_byte_from_offset_one`: Tests the 16-bit value at offset 34 of the structure in a4 and, if it is zero, returns without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `extract_nibble_and_init_struct`: Extracts a single nibble from the incoming byte in d0, shifting it left by one bit, and returns the result.
- `copy_word_value`: Copies a word from offset 16 of a6 to offset 6 of a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_byte_values`: Compares the byte in d0 against the values 7 and 10 and branches away when either matches.
- `copy_word_from_offset_sixteen`: Copies the 16-bit value at offset 16 of the structure in a6 into the 16-bit location at offset 6 of the structure in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_parameters`: Clears a flag in a global variable and disables audio, then copies a block of data from one global table to another.
- `execute_repeated_routine`: Executes another routine three times in a loop, incrementing a counter each iteration and using it as an index for memory access.
- `configure_audio_channel`: Clears a block of memory in a channel structure selected by the incoming index, then copies setup values from a table into that structure.
- `init_and_dispatch_buffered_structure`: Initializes a data structure by storing the value a fixed address and the value 1 at its first offsets, then branches to a shared exit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_clamp_high_nibble`: Reads the high nibble of the byte at offset 3 of the structure in a6 and, when it is nonzero, adds it to the byte at offset 19, clamps that byte to 64 if it would otherwise exceed it, and stores the result as a word at offset 8 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_dispatch_flag`: Writes a magic value and two one words into the structure pointed to by a0, copies a word from a flag byte into a field of the structure in a5, and returns through the default case of a dispatch when the flag byte is nonzero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte_2`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_byte_dispatch_table`: Loads a byte from the address in a2 and advances the pointer, and when the sign flag is clear subtracts 128 from the value in d0 before jumping through a program counter relative table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `subtract_clamp_byte_d0`: Subtracts the byte in d0 from the byte at offset 19 of the structure in a6, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte_3`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `subtract_and_jump_table`: When the sign flag is clear subtracts 128 from the value in d0 and then jumps through a program counter relative table indexed by that value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_byte_if_negative`: Clears the byte at offset 19 of the structure in a6 when the negative flag is set, then stores that byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte_3`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_indexed_table`: Loads an address from a program counter relative table indexed by the value in d0 and jumps to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_as_word_2`: Stores the byte at offset 19 of the structure in a6 as a word at offset 8 of the structure in a5.
- `clear_global_byte_flag`: It stores one less than the value of a byte in a6 at offset 3 into a global variable, writes zero to a global variable, and finally sets the flag to all ones.
- `clamp_byte_and_store_2`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_indirect_jump`: Clears the return value register and then jumps indirectly through the address held in a0, transferring control to that target.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_from_d0`: Stores the 16-bit value from d0 into the word at offset 8 of the structure in a5, then returns.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then returns.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5, then returns.
- `process_byte_value`: Reads a byte from offset 3 of a6, processes it through bit shifts and arithmetic operations, performs a comparison with 63, and stores the result to fixed memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_and_flags`: It stores a word at offset 42 of a4 and copies three word pairs from offsets 2, 6 and 6 of a4 into offsets 150, 154 and 156 of a5, then initializes a structure pointed to by a pointer at offset 8 of a4 with a marker and flags, and if a long at offset 20 of a4 is nonzero it sets several bytes and words at offsets 50, 52, 48 and 44, finally loading a word from a computed table index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_conditional`: It moves a byte from offset 3 of a6 into d0 and returns if that byte is zero, otherwise it clears a byte in a global variable, stores the byte there, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_flags_a5`: It stores a word at offset 156 of a5, loads a pointer from offset 8 of a4, clears a word at offset 8 of that pointer, stores a marker and two flags at offsets 0, 4 and 6, then if a long at offset 20 of a4 is nonzero it sets a byte at offset 50 and clears words at offsets 52 and 44.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_global`: It stores a byte from d0 into a global variable and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_struct_and_compute`: It loads a pointer from offset 8 of a4, clears a word at offset 8 of that pointer, stores a marker and two flags at offsets 0, 4 and 6, then if a long at offset 20 of a4 is nonzero it sets a byte at offset 50 and clears words at offsets 52, 44 and 48, and otherwise it doubles d0 several times and adds a word from offset 140 of a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `show_string_table`: Loads the address of a string table into a0 and the number of characters into a1, then calls a routine that displays the text.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_structure_and_index_table`: Sets two 16-bit fields at offsets 4 and 6 of the structure in a0 to one, then checks a flag at offset 20 of the structure in a4; if it is not set it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_table_value`: Loads a 16-bit value from a table indexed by a byte in a1, adds 1 to it, and stores the result back through a pointer in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `combine_offset_values`: Loads the 16-bit value stored at offset 20 of the structure in a6 into a0, then loads the 16-bit value at offset 22 into a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_offset20_value`: Reads a 16-bit value from offset 20 of the structure in a6 and stores it into a global variable, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_table_value_to_pointer`: Loads a 16-bit value from a table indexed by the byte at offset 14 of the structure in a5, then stores that value into the 16-bit word pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_control_byte`: Reads a byte from the input register and, when it is a control character, decodes a nibble pair from a byte at offset 3 of the structure in a6 to compute a value that is stored through a pointer, setting a flag byte at offset 48 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_indexed_word`: Multiplies a value passed in a register by two, adds it to a table index stored at offset 8c of the structure in a4, indexes into a table pointed to by offset 10, and stores a 16-bit word at offset 46 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_condition_flag`: Checks whether a condition flag is set and branches away if it is, otherwise it unconditionally branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `double_and_store_table_word`: Doubles a value in a register, indexes into a table pointed to by offset 10 of the structure in a4, and stores the resulting 16-bit word at offset 46.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unconditional_branch`: Unconditionally branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_structure_timers`: Copies a 16-bit value from the address in a0 plus the offset in d0 into a structure pointed to by a4, sets byte 46 to 255, then calls two routines.
- `store_byte_as_word`: Reads a byte from the byte stream pointed to by a2 into a data register, stores that value as a 16-bit word at offset 68 of the structure in a4, then branches to another routine.
- `copy_pointer_from_struct`: Loads a longword from offset 54 of the structure in a4 into a pointer register, writes that pointer to offset 58 of the same structure, then branches to another routine.
- `multiply_byte_by_register`: Reads a byte from the byte stream pointed to by a2 into a data register, multiplies it by the 16-bit word at the address in a6, stores the 16-bit product at offset 64 of the structure in a4, then branches to another routine.
- `divide_constant_by_byte`: Reads a byte from the byte stream pointed to by a2 into a data register, divides the constant 750 by that byte, stores the 16-bit quotient at the address in a6, then branches to another routine.
- `clear_structure_flags_and_copy_string`: Clears several flag bytes in a global structure, copies a short string from a computed table address into a buffer, loads a 16-bit value from the structure, and calls two other routines.
- `store_state_from_struct`: Stores a word from the structure in a4 into a memory location, copies two pointers from a2 into memory, copies three words from the same structure into another location, clears a word in the structure, then branches to another routine.
- `store_byte_via_pointer_2`: Loads a byte from the byte at the address in a2 into d0, sign-extends it to a word, stores that word at offset 140 in the structure in a4, then jumps to a routine entered from outside this code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `adjust_pointer_on_flag`: Reads a nibble from offset 31 of the record in a6, and when that nibble is nonzero and the byte at offset 35 becomes nonzero after adding a table value to it, it adjusts a pointer at offset 36 toward a limit derived from offset 14 and offset 10, then negates the byte it points to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_transformed_table`: Multiplies a byte from the address in a2 by fourteen, adds a constant to the product to form an address in a0, then loads two words and two long words from that address into offsets 36, 38, 40, 28 and 16 of the structure in a4 before jumping to code entered from outside.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_return`: Restores registers d1 and a0 from the stack and returns immediately.

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
- `stub_return_immediately`: Does nothing and returns; it is an empty routine with no calls or registers.
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

- `mask_or_bytes_loop`: Loops a number of times given by a register, each pass masking and ORing bytes at offsets 0, 32, 64, 96 and 128 of a structure while advancing the base pointer by another value, then restores registers and returns.

The function list continues in: .
