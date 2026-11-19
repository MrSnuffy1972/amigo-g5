# Game 1477 (castle master (1990)(domark)(m3)[cr][m acu]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01477). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908000 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 53 of 53 units identical (bytes and relocations), 64 raw-word lines |
| Functions named and described | pass, 132 of 132 named (0 author symbols), 132 of 132 described (28 by rule, 71 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 954 of 954 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 526110 bytes, 40 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4035 of 4041 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 155020 |
| code | 11006 |
| data | 526885 |
| padding | 2097 |
| unused | 212992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4 | 1 | 99 | 3 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:cm | 1 | 116 | 2 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 129 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 103 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 164 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 146 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 145 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 134 | 3 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4#unpacked | 1 | 1041 | 18 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x (raw code file, load address unknown) | 1 | 2 | 1 |
| Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown) | 1 | 1253 | 21 |

## Cases handled

- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 63 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 63 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 143 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 146 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 70 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 70 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4: crunched executable unpacked with ancient scan (TetraPack 2.2 (xfd), 10568 -> 53248 bytes), 9 of 9 windows confirmed in reference RAM, loaded at an address: recovered as u50
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- castle master (1990)(domark)(m3)[cr][m acu]/Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- 2 never-loaded region(s), 472862 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4

- `start_program_set_colour01`: This is the program entry point.
- `store_word_and_shift_stream`: Stores a 16-bit word built from the low byte of a longword and the high byte of a register, then returns that longword shifted right by one with its top bit forced on.
- `shift_bits_refilling_when_empty`: Shifts a longword right one bit per iteration over a count taken from a register, and reloads it through a call whenever it becomes zero.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:cm

- `load_intro_file_via_dos`: Opens dos.library, allocates memory with exec.AllocMem, and opens a file named intro through the DOS Open, Read and Close calls.
- `print_null_terminated_string`: Measures a null-terminated string and passes its length through the library entry at offset -60.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_read_mouse_buttons`: Opens a library with exec.OldOpenLibrary and reads the POTGOR register, which holds the right and middle mouse buttons.
- `write_string_to_output`: Calls the library entry at offset -60, scans a string to its null terminator, and then calls the entry at offset -48 to write it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra_move`: Short code fragment that jumps to another point in the code, then copies d0 to a fixed memory address, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra_move_2`: Short code fragment that jumps to another point in the code, then copies d0 to a fixed memory address, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by writing zero to a series of words.
- `handle_keyboard_input`: Reads keyboard input from a hardware register and stores the result at specific locations in a table.
- `frag_st_movem_jmp`: Short code fragment that sets an indexed entry from a0 to all ones, then restores saved registers from the stack, then jumps to another point in the code.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `process_input_sequence_2`: Processes sequential keyboard input by reading from an input buffer and storing characters into another buffer.
- `frag_clr_addq_dbra_rts`: Short code fragment that clears the memory at an offset from a0, then adds 1 to a1, then loops back while counting down d0, then returns.
- `clear_memory_range`: Fills a range of 111 words starting at a specific memory address with zeros.
- `frag_movea_move_rts`: Short code fragment that copies a fixed memory address to a0, then copies a0 to a fixed memory address, then returns.
- `update_audio_parameters`: Checks for a specific memory signature and performs complex data manipulation involving multiple tables and hardware audio volume registers when the signature matches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_2`: Clears a block of memory by writing zero to a series of words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_clr_dbra_movea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then copies a fixed memory address to a0, then copies a0 to a fixed memory address, then copies an address to a fixed memory address, then returns.
- `process_memory_blocks`: Processes memory blocks by clearing flags, handling bit operations, and managing data transfer between different memory locations based on conditional checks.
- `frag_clr_addq_dbra_rts_2`: Short code fragment that clears the memory at an offset from a0, then adds 1 to a1, then loops back while counting down d0, then returns.
- `clear_memory_range_2`: Fills a range of 111 words starting at a specific memory address with zeros.
- `frag_movea_move_rts_2`: Short code fragment that copies a fixed memory address to a0, then copies a0 to a fixed memory address, then returns.
- `update_audio_parameters_2`: Checks for a specific memory signature and if matched, updates audio volume registers and performs table data manipulation involving multiple hardware audio channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_coprocessor`: Initializes coprocessor registers by setting up multiple data structures and performing bit manipulations on memory locations, then calls several routines for further processing.
- `fill_multiple_buffers`: Clears various offsets within a memory block by writing zero to them in a loop, performing multiple memory writes per iteration.
- `shift_and_write_values`: Reads 16-bit values from a table, shifts them based on a global variable's lower nibble, and writes the results to multiple destination buffers at different offsets.
- `copy_tile_data`: Copies tile data from a table to multiple memory locations using bit shifting, then updates global variables with computed values before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library`: Opens a library and initializes global variables based on the library's return value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_and_store`: Shifts data from multiple source locations and stores them into destination arrays with specific offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_render_data_3`: Sets up rendering parameters for a sequence of graphical elements by copying data from a source table to a target buffer, updating counters and pointers as it progresses through the data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_update`: Copies data from one memory location to another while updating counters and status flags, then continues processing until a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics`: Sets up graphics parameters by loading specific values into registers and calling a library function, then stores the result to a global memory location and branches based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_loop`: Processes data in a nested loop structure, updating pointers and counters while copying values between memory locations and calling a handler when a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_rendering`: Initializes rendering parameters by loading values from global memory, calling library functions, and setting up registers before checking a condition and branching accordingly.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_suba_bra`: Short code fragment that copies the memory at an offset from a0 to d2, then copies the memory at an offset from a0 to d3, then copies the memory at an offset from a0 to d4, then copies the memory at an offset from a0 to d5, then subtracts 40 from a0, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_rendering`: Executes a rendering operation by calling library functions, updating flags in memory, and entering an infinite loop that continuously sets a bit in a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_suba_bra_2`: Short code fragment that copies the memory at an offset from a0 to d5, then subtracts 40 from a0, then jumps to another point in the code.
- `frag_movea_dbra_addq_rts`: Short code fragment that copies the top of the stack to a0, then loops back while counting down d0, then adds 2 to a7, then returns.
- `copy_table_to_buffers`: Copies data from a source table into multiple destination buffers using nested loops, with the outer loop controlling the number of iterations and the inner loop processing individual elements.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_buffer_values`: Updates buffer values by loading a word from memory, storing it at multiple offsets, performing arithmetic operations, and then swapping bytes before storing results to specific memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_data_blocks`: Transfers data blocks between source and destination buffers using nested loops, copying values from a table to multiple memory locations while incrementing pointers and decrementing counters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_timer_values`: Updates timer-related values in a data structure by storing word-sized values at specific offsets, performing arithmetic operations with a constant, and swapping byte order during the process.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_rendering_data`: Updates rendering data by copying values from a table to memory locations with specific offsets, incrementing pointers, and using decrementing counters in nested loops for iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_graphics_parameters`: Updates graphics parameters by performing arithmetic operations on register values, storing results at specific memory offsets, and entering a loop that continues until a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_addq_dbra_lea_rts`: Short code fragment that copies d3 to the memory at an offset from a1, then copies d4 to the memory at an offset from a1, then copies d5 to the memory at an offset from a1, then copies d2 to the memory at a1 (advancing a1), then adds 2 to a0, then loops back while counting down d6, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_pairs`: Stores a 16-bit value and its swapped version into memory locations spaced by 8 bytes, repeating this pattern four times while incrementing the source pointer each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_addq_dbra_lea_rts_2`: Short code fragment that copies d5 to the memory at an offset from a1, then copies d2 to the memory at a1 (advancing a1), then adds 2 to a0, then loops back while counting down d6, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_swapped_words`: Writes the high and low words of a 32-bit value into memory locations with specific offsets, then performs a series of operations on global variables before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_addq_dbra_lea_rts_3`: Short code fragment that copies d2 to the memory at a1 (advancing a1), then adds 2 to a0, then loops back while counting down d6, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_buffer_with_data`: Copies data from a source location to multiple destinations in a buffer pattern, updating global variables and performing bitwise operations based on the copied values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_dbra_rts`: Short code fragment that loops back while counting down d0, then returns.
- `fill_buffers_from_table`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern, using loop counters to control the number of iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_swapped_values`: Processes a 32-bit value by swapping its bytes, adds specific constants, and stores the results at multiple memory locations while updating global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_rts`: Short code fragment that merges 20936 into a fixed memory address, then returns.
- `copy_table_values`: Reads word values from a source table and writes them into multiple destination buffers with specific offsets in a repeating pattern.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_and_fill`: Processes data from a table by copying values until reaching a specific marker, then fills buffers with repeated patterns while updating global variables and hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_buffers_with_pattern`: Copies data from a source table to multiple destination buffers in a repeating pattern, using loop counters to control the number of iterations and storing specific values at calculated offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_collision_data`: Updates collision detection parameters by swapping and adding a fixed value to a register, then storing the result in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_buffers_with_pattern_2`: Copies data from a source table to multiple destination buffers in a repeating pattern, using loop counters to control the number of iterations and storing specific values at calculated offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_pairs`: Copies word pairs from a source table to a destination buffer until encountering the value -2, then updates hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_dbra_rts`: Short code fragment that copies d4 to the memory at an offset from a1, then copies d1 to the memory at a1 (advancing a1), then loops back while counting down d5, then loops back while counting down d0, then loops back while counting down d7, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_loop`: Repeatedly copies words from a source table to a destination buffer in a loop until reaching the value -2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_dbra_rts_2`: Short code fragment that loops back while counting down d0, then loops back while counting down d7, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_until_end`: Copies words from one buffer to another in a loop until encountering the value -2, then performs several memory operations and calls another routine.
- `clear_multiple_offsets`: Clears specific longword offsets in a memory block through a loop, writing zero values to each offset.
- `write_shifted_values_to_buffers`: Reads values from a table, shifts them based on a global value, and writes them to multiple destination buffers.
- `copy_shifted_longs_to_planes_2`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_table_to_buffers_2`: Copies data from a source table into multiple destination buffers using nested loops with fixed iteration counts.
- `fill_buffers_from_table_2`: Reads values from a table and writes them into multiple destination buffers in a repeating pattern with specific offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_destination_area`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns.
- `clear_destination_buffer`: Clears a destination buffer by applying bitwise operations from a source table to the memory pointed to by a global variable.
- `execute_repeated_routine`: Calls another routine four times in a loop using a decrementing counter, with the called routine's return value preserved on the stack.
- `update_pattern_data`: Processes and updates pattern data using bit manipulation and table lookups based on input parameters, writing modified values back to memory locations.
- `load_and_branch`: Loads long values from memory locations relative to a0 pointer and branches to another routine based on a conditional check.
- `update_pattern_data_2`: Performs bitwise operations on multiple 16-bit values and writes modified results to specific memory locations relative to a pointer, using shift and mask operations based on input parameters.
- `update_bitwise_buffers`: Updates multiple buffers using bitwise operations on data from tables, applying masks and combining values before continuing the processing loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_destination_area_v2`: Iterates through a source table to perform bitwise masking and OR operations on a destination buffer to clear specific pixel patterns, using a different base address.
- `apply_bitwise_operations`: Processes a source table of 16-bit words through a destination buffer using bitwise AND, OR, and NOT operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bsr_clr_tst_bpl_addq_bra`: Short code fragment that calls another routine, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was not negative, then calls another routine, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (addi.l, cmpi.w) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_loop`: Sets up the main game execution loop by initializing various hardware registers, clearing global variables, and configuring interrupt handlers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_addq_move_rts`: Short code fragment that adds 2 to d0, then copies d0 to a fixed memory address, then returns.
- `interpolate_nibble_values`: Adjusts nibble values in two tables of 16-bit words toward target values through incremental steps.
- `execute_infinite_loop`: Contains an empty loop that repeatedly checks a condition and branches back to itself.
- `clear_memory_blocks`: Writes zero values to multiple memory locations in a data structure through four separate memory copy operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `interpolate_nibbles`: Adjusts three nibbles within each word of a table toward target values from another table, incrementing or decrementing each nibble by one step per iteration if the difference is less than sixteen.
- `update_timer_values_2`: Updates multiple timer-related values in a global structure starting at offset a fixed address, using data from global variables and a loop that increments a counter until it reaches 10000.
- `clear_memory_blocks_2`: Writes zero values to multiple memory locations in a global table, advancing the write pointer by ten elements each iteration for a total of 199 iterations.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:dawn4#unpacked

- `frag_lea_movea_jsr_move_bsr_btst_bne_br`: Short code fragment that loads the address of a location in the code into a1, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then loads the address of a location in the code into a0, then calls another routine, then tests bit 10 of a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `handle_file_operations`: Handles file operations by writing data to output streams using DOS library calls.
- `process_input_sequence`: Processes input bytes from a buffer and updates various game state variables based on the byte values.
- `initialize_render_data`: Sets up rendering parameters for multiple data entries using a global variable as a base, updating offsets in a structure referenced through a6 with values derived from shifts and additions.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_audio_channels`: Updates four audio channel parameters by decrementing countdown timers and reading values from tables into memory registers.
- `initialize_sequence`: Writes a series of longwords and words to memory locations starting at a global variable, updating values in a loop with incrementing counters and conditional data loading from another table.
- `frag_lea_move_add_addq_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then copies an address to d0, then copies an address to d1, then copies 31 to d2, then copies d1 to the memory at a0, then adds d0 to d1, then adds 8 to a0, then loops back while counting down d2, then returns.
- `manage_audio_channel_operations`: Manages audio channel operations by checking channel status flags, updating channel parameters, and writing values to audio registers.
- `configure_audio_parameters`: Configures audio parameters for multiple channels by setting up channel addresses, updating length and period values, and managing playback state through DMA control registers.
- `fill_data_structure`: Populates a data structure with repeated sequences of longwords and words, using nested loops to write fixed values and read from a source table while incrementing pointers.
- `setup_context`: Processes input word values through bit rotation and XOR operations before storing them at specific offsets within a structure pointed to by a6 register, along with other input values.
- `frag_move_clr_rts`: Short code fragment that copies 2304 to the memory at an offset from a6, then clears the memory at an offset from a6 (3 times in a row), then copies d1 to the memory at an offset from a6, then copies 33856 to the memory at an offset from a6, then copies a1 to the memory at an offset from a6, then copies d2 to the memory at an offset from a6, then copies 1024 to the memory at an offset from a6, then returns.
- `disable_audio_playback`: Clears the audio playing flag and zeroes four audio channel period registers.
- `initialize_game_object`: Sets up a game object's properties including position, graphics data, and audio parameters using values from predefined tables, then configures DMA and interrupt settings.
- `initialize_render_data_2`: Initializes rendering data structures by setting up parameters for multiple data entries using a global variable as a base, updating offsets in referenced structures.
- `update_channel_sequence`: Reads the next event byte from a per-channel pattern, moves the pattern pointer forward when a block ends, and updates the channel's volume and playback fields from the bytes that follow.
- `parse_command_stream`: Reads a byte stream to update internal state, manipulate a global bitmask, and control pointers based on encoded commands.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:x (raw code file, load address unknown)

- `stub_two_instruction_entry`: A two-instruction stub at the start of a raw code region, with no calls, callers, registers or text in the listed facts.

### Castle Master (1990)(Domark)(M3)[cr][m ACU].adf:intro (raw code file, load address unknown)

- `open_libraries_init_copper`: Opens a system library through exec.OpenLibrary and then calls library entries at offsets -30, -36 and -42.
- `clear_long_slots_across_planes`: Zeroes pairs of longword slots spread across four plane-sized regions over a fixed run of passes.
- `blit_shifted_source_to_planes`: Copies rows of a source table into four plane buffers, shifting each word left by the low four bits of a position value.
- `copy_shifted_longs_to_planes`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `copy_four_plane_block`: Copies a twenty-word-wide block from a source whose four sub-blocks sit twenty words apart into four plane buffers spaced 8000 bytes apart.
- `fill_run_length_plane_rows`: Reads records of a row count and three words from a table, then writes the three words across twenty columns of planes 0 to 2 and zeroes plane 3 for count plus one rows.
- `merge_masked_planes_and_clear`: Merges a twenty-column block from four source planes into four destination planes, keeping destination pixels wherever all four source planes are zero.
- `dispatch_on_phase_counter`: Calls unnamed routines depending on a signed state value tested against fixed ranges, and resets a phase counter to zero when it equals three.
- `advance_phase_counter_once`: Resets a signed counter to zero when it is below minus twelve, otherwise calls an unnamed routine and, when the counter equals three, resets it and calls the routine again.
- `draw_clipped_masked_block`: Draws a bit-shifted block from four source planes into four destination planes, limiting the column count by the distance to a 320-pixel edge and keeping destination pixels wherever all source planes are zero.
- `draw_wide_clipped_masked_block`: Draws a bit-shifted, masked four-plane block like the narrower variant, but limits the column count to eight and reads its source planes further apart.
- `draw_mode_selected_plane_row`: Draws one source word per row into destination planes after a sub-word shift, clamping the column count to two and adding an extra plane write when the mode value equals two.
- `draw_shifted_row_with_spill`: Draws up to eight source rows into four planes, each shifted by the low four bits of an x position with spilled bits carried into the following word.
- `merge_sparse_table_at_offset`: Walks a fixed source table four words at a time, skipping all-zero groups and merging the others into four destination planes at an offset while keeping background pixels where the source is zero.
- `merge_sparse_table_at_base`: Walks a second fixed source table four words at a time, skipping all-zero groups and merging the others into the base of the plane area.
- `repeat_call_four_times`: Loops four times, calling another routine on each iteration and decrementing a counter, then returns.
- `draw_shifted_masked_planes_wide`: Draws a bit-shifted, masked block of four consecutive source words per group into four destination planes, with a separate case for the final partial word.
- `wrap_position_and_read_words`: Adds an offset to a 16-bit position and wraps it when it passes 318, repeating for a second offset.
- `nudge_palette_components_toward_source`: Compares the 4-bit red, green and blue components of sixteen words with those of a second table, nudging a component one step toward it only when the two differ by fifteen.
- `spin_in_infinite_loop`: The decompiled body is an empty infinite loop with no calls and no hardware access, and its 40 instructions are not all visible.
- `clear_ten_longs_across_planes`: Zeroes ten consecutive longwords and matching slots in four plane-sized regions on each pass, advancing ten longwords per pass over a fixed run of passes.
