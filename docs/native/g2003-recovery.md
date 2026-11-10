# Game 2003 (operation stealth (1990)(u. s. gold)[cr pdx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02003). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2740432 top-level bytes in 93 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 62 of 62 units identical (bytes and relocations), 27 raw-word lines |
| Functions named and described | pass, 672 of 672 named (0 author symbols), 672 of 672 described (56 by rule, 255 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3102 of 3102 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12251 of 12253 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2361112 |
| code | 39810 |
| data | 274396 |
| padding | 10842 |
| unused | 54272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| NARRATOR.DEVICE | 3 | 3782 | 268 |
| MESSAGE | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:endcli | 2 | 117 | 2 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:Intro | 2 | 174 | 12 |
| TRANSLATOR.LIBRARY | 2 | 490 | 50 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:PAUSE | 2 | 3 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:delphine | 2 | 207 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:add21k | 3 | 637 | 38 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:PPMORE | 2 | 209 | 8 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:message | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 3 of 3).adf:paradox | 2 | 24 | 4 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:bootblock | 1 | 10 | 1 |
| Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 3 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 901 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 380 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 393 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 219 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 160 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 147 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1011 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 256 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1326 | 75 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 315 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 798 | 53 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 394 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 113 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 268 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 91 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 83 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 216 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 188 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 664 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 214 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 437 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (19 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (33 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- operation stealth (1990)(u. s. gold)[cr pdx][a](disk 1 of 3)/Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:delphine: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- operation stealth (1990)(u. s. gold)[cr pdx][a](disk 2 of 3)/Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:PPMORE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:Intro

## Copy protection found

- none found

## Functions


### NARRATOR.DEVICE

- `initialise_narrator_device_unit`: Allocates a unit structure for the narrator device, initialises its linked lists and fields, allocates a second block for a request, and links the two together.
- `queue_narrator_request_or_fail`: Handles a narrator request: when the input value is zero and a unit is already active, it fills the request record with default pitch, rate and volume and links it.
- `start_or_queue_narrator_request`: Starts a narrator request when no speech is in progress, allocating a message and recording an error code if the allocation fails.
- `allocate_narrator_message_or_fail`: Allocates a message for a narrator request and stores the new pointer in the unit.
- `fill_narrator_request_defaults`: Stores the value passed in the data register as the unit's current request and increments the request counters.
- `store_request_status_byte`: Stores the low byte of the data register as the request's status and sets its completion marker to minus one.
- `requeue_or_update_narrator_request`: Clears the request's flag and either copies the current speech mode from the unit into it or puts the request back on the unit's waiting list.
- `queue_narrator_request_on_unit`: Places the request on the unit's waiting list, or when it already matches the pending request, copies the current speech mode into it.
- `update_request_mode_or_requeue`: Copies the current speech mode into the request when it differs from the unit's setting, otherwise it requeues the request on the unit's list while adjusting the nesting counter.
- `copy_speech_mode_nibbles`: Copies the speech mode byte from the source record into the request, splitting it into low and high nibbles stored separately.
- `validate_and_apply_speech_parameters`: Checks each speech parameter in the request, such as rate, pitch, volume and the mode flags, against its allowed range and copies the valid values into the unit.
- `apply_speech_parameters_checked`: Decrements the request counter, then checks the speech parameters in the request and applies each valid value to the unit.
- `finish_narrator_request_and_notify`: Removes matching requests from the unit's waiting list by calling a routine for each match, then decrements the active count.
- `decrement_active_count_and_shutdown`: Decrements the active count, and when it reaches zero calls two library routines to shut the unit down.
- `dispatch_narrator_mode_handler`: Uses the mode value stored in the request to call one of eight handlers from a table.
- `unlink_request_or_mark_pending`: Removes the request from its doubly linked list and calls library routines to release it when its status word is zero.
- `unlink_request_and_release`: Removes the request from its doubly linked list, calls library routines to release it, and returns the entry's stored value.
- `process_narrator_request_state`: Updates the status and flags of a speech request based on its current state code.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `set_state_seven_and_finish`: Clears a flag in the unit, marks the unit with state code seven and sets its busy bit, then calls library routines and finishes the request with a status of zero.
- `set_state_six_and_busy`: Sets a flag bit in the unit, marks the unit with state code six and its busy bit, calls a library routine, and finishes the request with a status of zero.
- `bracket_two_calls_with_flag`: Sets bit 7 of the unit's flags, calls two routines, then clears that bit again so both calls run with the flag set.
- `bracket_one_call_with_flag`: Sets bit 7 of the unit's flags, calls a routine and clears the bit again, then sets the request status to zero and calls a second routine.
- `flush_request_list_entries`: Calls a routine for the unit's pending request if one exists, then calls a routine for each entry on the request list while holding the nesting counter.
- `decrement_active_count_only`: Decrements the active count in the device data and, when it reaches zero, sets the global control word to a fixed value.
- `abort_pending_request`: If the request is the active one, it marks the request with a flag bit.
- `remove_request_from_list`: Searches the device's request list for the given request.
- `unlink_request_mark_aborted`: Unlinks the request from its list, sets its status to an abort code, decrements the active count, and calls a library routine.
- `decrement_counter_and_set_global`: Decrements a byte in a structure pointed to by a register and, if the previous value was less than one, writes a constant to a global variable.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `fail_request_with_status_code`: Sets the request's status to a fixed error code and calls a library routine.
- `complete_request_with_zero_status`: Sets the request's status to zero to mark it complete and calls a library routine to finish it.
- `clear_unit_flag_and_status`: Clears bit 0 of the unit's flags and zeroes the request's status byte, then calls a library routine.
- `apply_speech_parameters_from_start`: Checks each speech parameter in the request against its allowed range, starting from the first one, and applies the valid values to the unit.
- `apply_speech_parameters_from_pitch`: Applies the request's speech parameters beginning with the pitch value, checking each against its allowed range.
- `apply_speech_parameters_from_rate`: Applies the request's speech parameters beginning with the rate value, using the same range checks.
- `apply_speech_parameters_from_mode`: Applies the request's remaining parameters beginning with the mode values, checking each range before storing it in the unit.
- `apply_speech_parameters_from_flag`: Stores a flag value from the request, then applies the remaining parameters beginning with the volume range, jumping to the finishing step when a check fails.
- `apply_speech_parameters_from_volume`: Applies the request's parameters starting from the volume-range value, checking each against its range before storing it and finishing the request on failure.
- `check_level_then_apply_parameters`: Checks a level value against its maximum of 64.
- `start_speech_and_finish_request`: Starts speech output through library calls, then completes the request.
- `store_request_result_and_status`: Copies a result field from the request into its output slot, stores the status byte passed in the data register, and calls a library routine.
- `clear_result_and_store_status`: Clears the request's result field to zero, stores the status byte passed in the data register, and calls a library routine.
- `find_text_split_point`: Scans a text buffer backwards for a comma or hyphen and, failing that, for a space near three quarters of the way along, recording that position as the split point.
- `find_space_split_point`: Looks for a space near three quarters of the way along a text buffer and records it as the split point, falling back to half the buffer length when no space is found.
- `split_at_half_length`: Records half the buffer length as the split point, then finishes the request and stores a status taken from a library call.
- `initialise_speech_channel_state`: Clears the channel's flag and count fields, sets default channel values, copies two settings from the request, and calls library routines.
- `select_speech_table_entry`: Looks up a routine in a table using the request's selector byte.
- `copy_channel_template_blocks`: Stores the buffer pointer in the channel, then copies a 34-word template into the buffer and a second area, repeating once for each channel slot.
- `allocate_and_copy_channel_template`: Allocates a buffer for the channel and, when that succeeds, copies the channel template into it once for each channel slot.
- `copy_channel_template_to_buffer`: Copies the channel template into the given buffer, repeating a 34-word block once for each channel slot, and returns zero.
- `release_channel_buffers_with_status`: Sets the channel mode to nine, frees the channel's two buffers when they are allocated, clears the length field, and returns a fixed status value of two.
- `release_channel_buffers`: Sets the channel mode to nine, frees the channel's two buffers when they are allocated, clears the length field, and returns the register values unchanged.
- `translate_text_to_phonemes`: Returns at once when the channel's length is below one.
- `translate_text_to_phonemes_2`: Resets the phoneme output fields and walks the input text, matching two-character sequences against a rule table of about 112 entries and emitting phoneme codes until the text ends or a hash character is reached.
- `emit_phoneme_codes_for_text`: Continues phoneme translation: digit characters are converted to numeric phoneme codes and other characters are copied to the output, until the text ends or the length limit is reached.
- `continue_phoneme_translation_after_match`: Continues phoneme translation after a rule match: a digit range is converted to numeric phoneme codes and the rule flags decide whether the character is copied, until the text ends or the length limit is reached.
- `copy_phoneme_characters_with_marks`: Copies the current character into the phoneme output buffer and sets marker flags for control codes five and six.
- `set_phoneme_mark_flags`: Sets a marker flag on the current phoneme for control codes five and six, then continues the text scan.
- `set_phoneme_mark_flag_six`: Sets the marker flag for control code six on the previous phoneme entry and resumes the rule match over the text.
- `store_phoneme_code_with_limit`: Writes the phoneme code into the output buffer and advances the length.
- `resume_phoneme_text_matching`: Resumes the rule match at the current text position, checking the position against the stored limit before matching the next two characters.
- `stub_empty_return_routine`: The decompiled body is empty apart from a return, so this routine has no visible behaviour.
- `terminate_phoneme_output`: Appends a sentinel value of 255 to each of the parallel phoneme arrays and stores the final length.
- `pad_and_terminate_phoneme_output`: Appends the sentinel terminator to the phoneme output, first padding with a code of four when the output has more than three entries.
- `store_phoneme_output_length`: Stores the current output position in the channel's length field and returns.
- `mark_phoneme_context_flags`: Walks the phoneme output and sets a context marker bit on entries based on the class flags of the surrounding phonemes, including the preceding one or two codes.
- `propagate_phoneme_context_flags`: Walks forward through the phoneme output, setting context marker bits on the preceding entries for each phoneme whose class flags match, and returns at the sentinel value.
- `mark_phoneme_class_boundaries`: Walks forward through the phoneme output from a stored class value and sets marker bits on the entries two and three places back.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `flag_phoneme_runs_with_markers`: Walks the phoneme output and marks the start of each run of phonemes with bit 7, copying a flag bit from the first entry across the run.
- `spread_flags_across_phoneme_run`: Marks a run of phoneme entries with bit 7 and spreads a flag bit from the run's first entry across it.
- `copy_run_flags_to_phonemes`: Marks the current phoneme and, when a run start is known, copies a flag bit from the run's first entry across the remaining run length.
- `spread_run_flags_forward`: Computes the run length from the current position and spreads the flag bit of the run's first phoneme across it, marking separators along the way.
- `stub_empty_return_routine_2`: The decompiled body is a bare return with no work, so the routine has no visible behaviour.
- `match_phoneme_pattern_rule`: Compares the phoneme output at the current position with a pattern, where a sentinel byte in the pattern matches anything.
- `match_rule_pattern_at_position`: Checks the phoneme rule at the current position against its pattern bytes, treating a sentinel as a wildcard.
- `scan_rule_table_for_match`: Scans forward through the rule table testing each position against the pattern, where a sentinel byte matches anything.
- `scan_rule_table_from_index`: Continues scanning the rule table from the index in the data register, checking each position against the pattern.
- `test_phoneme_flag_mask`: Walks a table of flag entries, inverting or masking each value according to its control bits, and returns once the tested bit is set.
- `shift_phoneme_buffers_right_2`: Increments a value in a structure and shifts three memory areas right by one position, inserting a byte and two zeros.
- `shift_phoneme_buffers_right`: Increments the output length and shifts the phoneme output buffers right by one place, inserting the given marker at the current position and clearing the vacated slots.
- `run_fixed_routine_sequence`: Calls seven routines in a fixed sequence and returns zero.
- `clear_rule_table_flags`: Walks a table of eight-byte records up to the sentinel, clearing the last byte of each record, and then sets the same byte in the terminating entry to a fixed value.
- `clear_flags_until_matching_entry`: Clears the last byte of each eight-byte record until it reaches a record whose first byte matches the data register, then sets the byte in the channel's entry to a fixed value.
- `average_neighbour_phoneme_flags`: Walks the phoneme output and, for code 23, averages the small numeric values of the two preceding entries.
- `apply_phoneme_rule_flags`: Applies per-phoneme rule flags: code 23 averages the two preceding values, and other codes copy a selected flag into the output entry according to their class bits.
- `write_phoneme_output_entry`: Writes the output entry for the current phoneme from a lookup table, then sets a low class code of 4, 5 or 6 derived from the following phoneme's flags.
- `write_phoneme_entry_from_flags`: Writes a phoneme entry from the lookup table indexed by the data register and sets the class code derived from the next phoneme's flags, then advances.
- `set_phoneme_class_code`: Sets the low class code of the current output entry to 4, 5 or 6 based on the flag bits in the data register, then advances.
- `set_low_class_code`: Sets the low class code of the current output entry to 4, or to 5 when bit 5 of the flag value is set, then advances through the phoneme output.
- `set_low_class_code_four`: Sets the low class code of the current output entry to 4 and advances through the phoneme output, averaging code 23 with its neighbours as it goes.
- `write_phoneme_frames_until_sentinel`: Walks the phoneme output and writes frame entries from a lookup table for each phoneme, skipping code 23 and setting class bits along the way.
- `allocate_phoneme_frame_buffers`: Adds up the masked values of the phoneme output to get a total, then stores that total and a buffer size of eight bytes per unit plus eight.
- `allocate_frame_buffers_for_size`: Stores the given count and a buffer size of eight bytes per unit plus eight, allocates the frame buffer, and when a flag is set allocates a second buffer.
- `store_frame_buffer_pointer`: Stores the given buffer pointer as the channel's frame buffer, then allocates a second buffer when a flag is set.
- `build_phoneme_frame_output`: Builds the frame output for each phoneme from the lookup tables, adjusting the values according to the neighbouring class bits and the mode flag.
- `adjust_phoneme_frame_values`: Applies the lookup adjustments to each phoneme frame, adding a small offset to the stored values for stressed classes and capping one value at 31.
- `interpolate_phoneme_frame_values`: Interpolates frame values between neighbouring phonemes and writes the results into the output buffers, repeating for each frame until the sentinel value.
- `compute_phoneme_transition_values`: Compares neighbouring phoneme values and fills a short run of transition values for each pair, stepping through five positions and choosing between the two neighbours at each step.
- `smooth_phoneme_transition_values`: Compares neighbouring phoneme values and smooths the transition between each pair, stepping through a short run of positions.
- `smooth_phoneme_frame_steps`: Smooths the phoneme transition values in steps, comparing each pair of neighbours and writing a blended value at each position.
- `interpolate_between_neighbour_values`: Blends each frame value toward its neighbour by a weighted fraction using a five-bit shift, and advances through the frames until the sentinel value.
- `advance_to_next_frame_record`: Advances to the next frame record and repeats the pass, checking each phoneme code against the sentinel value.
- `total_frame_durations`: Walks the phoneme output and the frame list, advancing the frame pointer by eight bytes per unit of each phoneme's duration, and returns the accumulated value.
- `compute_frame_neighbour_values`: Computes each frame's values from the current phoneme and its neighbours, applying class-based overrides, and writes the results into the frame record.
- `compute_frame_neighbour_mix`: Computes a blended value for each frame from the neighbouring phoneme values, zeroing it for flagged entries, and stores the result in the frame record.
- `compute_frame_values_by_class`: Computes frame values for each phoneme class, mixing in the neighbouring values and clearing them where the class flags require.
- `compute_frame_flag_values`: Computes per-frame flag values from the current and neighbouring phoneme entries, setting a secondary marker when the class bits match.
- `fill_frame_transition_values`: Fills the transition values for a frame by stepping down through the values and blending them toward a target, using a per-frame step.
- `fill_frame_records_with_value`: Writes a run of four-word frame records, clearing each one and storing the given value into its value field unless the high flag is set, then walks the phoneme output to the next entry.
- `fill_frame_record_run`: Clears a run of four-word frame records and fills the value field of each one, then advances the phoneme output to the next entry.
- `fill_frame_records_for_phonemes`: Walks the phoneme output and writes a block of frame records for each phoneme, clearing each record and storing its value field.
- `fill_frame_records_with_neighbours`: Walks the phoneme output and writes frame records using the neighbouring phoneme values, clearing each record first.
- `clear_counted_frame_records`: Clears a counted run of four-word frame records and stores the value field in each record whose high flag is clear, then steps to the next phoneme.
- `compact_frame_arrays`: Copies the phoneme and frame arrays into compact form, skipping entries whose flag bit is set, until the sentinel value.
- `write_byte_to_three_pointers`: Stores one byte from a data register into the three memory locations addressed by three address registers, then returns.
- `call_two_routines_in_sequence`: Calls two other routines one after the other and returns; the decompiler could not resolve either call target.
- `clear_nibbles_and_mark_runs`: Clears the low nibble of each byte in one table up to a terminator, then walks a second table and sets flag bits 1, 2 and 4 on the entries that bound each run.
- `mark_run_flags_from_table`: Walks a table of indices, reads a word per index and sets flag bits in a parallel table on the entries at the start and end of each run, until a terminator.
- `mark_run_flags_by_index`: Reads an index table and a second table in step, setting flag bits on entries that begin and end each run, and stops at a terminator value.
- `mark_flags_forward_scan`: Scans forward through the two tables, setting flag bits when a boundary condition is met, and jumps back into the scan loop to continue marking.
- `mark_flags_backward_scan`: Scans through the two tables with a jump back into the loop, setting boundary flag bits as it goes and returning when the terminator is reached.
- `scan_and_set_flag_bits`: Loops over a table of indices and a parallel flag table, setting the flag bits on the current and following entries when a bit test is met.
- `set_flag_bits_then_scan`: Sets three flag bits on the current and preceding table entries, then scans forward setting boundary flags until a terminator is reached.
- `compute_scaled_buffer_writes`: Divides a fixed constant by a stored word to get a per-step value, then walks two tables writing scaled byte results into a buffer at computed offsets.
- `fill_buffer_with_scaled_values`: Walks two tables, dividing a stored constant by table values and writing the quotients into a buffer at computed offsets until a terminator.
- `fill_interpolated_buffer_values`: Accumulates offsets from one table, writes scaled quotients into a buffer and computes a difference between two table values to place intermediate samples.
- `compute_weighted_interpolation_steps`: Takes absolute differences of two stored values, divides a scaled count by their sum and writes the resulting interpolation step into a buffer.
- `write_weighted_interpolation_tail`: Repeats the weighted interpolation step over a table, writing each computed value into the buffer and finishing with a final division write.
- `write_scaled_samples_with_step`: Loads three table values per entry, writes a division result into the buffer and then walks a flag table to find the next run before writing again.
- `write_divided_levels_stride_eight`: Divides a stored value by a fixed constant and writes the result into every eighth byte of a buffer until a terminator value is found.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `process_channel_table_entries`: Iterates over channel table entries, calls two subroutines to fetch values and writes computed results into a buffer, using a terminator to end.
- `process_channel_entries_forward`: Steps through channel table entries, calling a subroutine for each marked entry and writing results into a buffer until the terminator is reached.
- `advance_and_process_channel_entries`: Advances two table pointers together, calls a subroutine on flagged entries and writes the output values, returning at the terminator.
- `process_table_entries_with_helpers`: Walks a table of entries and calls up to three subroutines for each one, using the neighbouring table bytes to choose which subroutine runs next.
- `select_neighbour_flags_and_process`: Reads flag bits from the current table entry and, depending on them, either calls a subroutine or jumps to the next neighbour-processing step.
- `read_previous_neighbour_bytes`: Reads the bytes before and after the current table position, substituting an earlier byte when one is zero, and calls subroutines depending on flag bits.
- `accumulate_neighbour_table_values`: Looks up table values for the neighbouring positions of each entry and combines them, using substitute bytes where a neighbour is zero.
- `compare_neighbour_table_flags`: Compares the flags of the two neighbours of each entry and calls a subroutine when the flag pattern matches a boundary case.
- `process_flagged_neighbour_entries`: Processes table entries whose flag bits indicate a neighbour boundary, calling a subroutine and recording the result in a flag buffer.
- `scale_table_values_by_flags`: Reads a table value for each entry, shifts it right when a flag is clear, and writes the result back in a buffer used by the next step.
- `blend_table_values_toward_target`: Computes a weighted difference between a stored value and a target value, shifts the result and adds it back to the current sample.
- `write_blended_sample_values`: Applies the same weighted blend to each table entry and stores the blended bytes into the output buffer until the terminator is reached.
- `compute_scaled_shifted_product`: Multiplies a 16-bit input by a 16-bit value, then returns the high word of the product together with the low word after adding a constant and shifting right by five.
- `initialize_and_clear_buffers`: Stores offsets relative to a base pointer into a state structure, advances some pointers, clears several word fields and zeroes six blocks of memory.
- `advance_counter_and_run_steps`: Increments a 16-bit counter in the state structure and, while it has not wrapped and the increment did not overflow, calls four subroutine steps.
- `pack_nibble_values_into_buffer`: Walks a table of entries, extracts a scaled nibble for each marked entry, counts marked positions and packs the results into a buffer, returning once a fixed count is reached.
- `pack_entries_into_buffer_forward`: Scans table entries forward, packs a nibble value for each into a buffer and records the first and last positions where a flag is set.
- `pack_entries_and_mark_end`: Packs flagged table values into a buffer and sets an end-of-run flag bit on the last written byte before returning.
- `set_flag_and_store_indices`: Sets an end-of-run bit on the byte at the current index and stores two index values into the state structure, copying one conditionally.
- `fill_flag_buffer_from_tables`: Reads paired table entries, sets flag bits in the output buffer for each entry that matches a pattern and stops when the terminator is reached.
- `set_flag_bits_in_buffer`: Walks the buffer and writes a fixed flag value into positions that the table marks, moving forward through the buffer until the terminator.
- `set_flag_bits_backward`: Walks the buffer backward from the current position, setting flag bits and copying a marker value when the table indicates a run start.
- `propagate_flags_backward_through_buffer`: Walks the buffer backward, propagating a flag value to earlier positions until a boundary entry is found, and clears the source byte when moved.
- `propagate_marker_flags_backward`: Walks a buffer backward ORing a flag into each byte and moves marker bits between neighbouring positions when a table value indicates a transition.
- `scan_for_flag_markers`: Scans leftward through a table for the next flagged position, moving marker bits and clearing the source byte when a target is found.
- `swap_and_clear_byte_flags`: Moves a table byte from one position to another, clears the source when the positions differ, decrements the index and propagates a flag bit forward or backward.
- `set_low_bit_across_range`: Sets bit zero on each byte in a range of positions, counting down from the highest index to zero.
- `set_low_bit_counting_down`: Sets bit zero on each byte from the current index down to zero, stopping after the first position.
- `or_mask_with_early_exit`: Chooses a mask from a control byte and ORs it into a run of bytes in one buffer, returning early once a flag in another buffer is set.
- `write_mask_clear_nibble_leftward`: ORs a mask into the current byte and, when a flag is set, scans leftward to the nearest flagged entry and clears its low nibble before setting bit two.
- `accumulate_table_entries`: Adds a word value to eight consecutive table entries, first calling up to seven subroutine steps chosen by two stored flags.
- `compute_scaled_table_byte`: Derives a byte from a computed scale value, clamps it into a fixed range and stores it into a table entry.
- `clamp_value_and_store_char`: Clamps a 16-bit value to a fixed lower and upper bound and stores the resulting byte at an indexed table position.
- `distribute_quotient_across_range`: Divides a character difference by a count and distributes the quotient and its remainder across a range of flagged positions.
- `adjust_table_values_loop`: Repeatedly computes an absolute difference between two table entries, scales it into a destination byte and scans backward for flagged entries, then applies a scaled correction to the flagged range.
- `scale_flagged_entries_into_tables`: Multiplies table values for each flagged entry by fixed scale factors, writes the products into two output tables and applies a rounding adjustment.
- `scale_entries_and_store_results`: Computes scaled values for each entry in a range using multiplication and shifts, storing them into two tables and adjusting a running accumulator.
- `scale_flagged_entries_and_finish`: Repeats the scaled-value computation over flagged entries and, at the end of the range, copies a table byte and applies a fixed offset to it.
- `adjust_pairs_of_flagged_entries`: Walks flagged entries backward in pairs, applying scaled corrections to both positions in each pair, and handles a two-step gap separately.
- `adjust_flagged_entry_pairs_inplace`: Applies scaled corrections in place to adjacent flagged entry pairs, using a comparison of the two neighbours to choose which value to update.
- `update_pair_with_difference`: Adjusts a pair of table entries by a rounded difference of their neighbours and a scaled term, storing the result back in place.
- `update_entries_in_reverse_order`: Updates table entries from the highest index downward, combining each with a scaled difference and writing the results back into the output tables.
- `update_entries_with_difference_pass`: Performs one pass over the table entries applying the scaled difference correction and storing the adjusted values for the next stage.
- `scale_pair_differences_into_tables`: Computes differences between paired table entries, scales them and writes the results into two output tables over a range of flagged positions.
- `find_running_max_and_scale`: Tracks a running maximum over a range of table bytes and writes a scaled value derived from it into the output buffer, with a constant correction.
- `apply_scaled_offsets_to_pairs`: Applies scaled offsets to pairs of table entries selected by a mode value, updating the destination bytes and a running accumulator.
- `apply_scaled_offsets_with_mode`: Applies mode-dependent scale factors to a pair of table entries and then clears the source bytes when a flag in the mask is not set.
- `apply_mode_offsets_to_pair`: Adds a mode-selected offset to the first entry of a pair and a second mode-selected offset to the second, with a fixed correction when a flag is clear.
- `apply_mode_offsets_and_scan`: Applies mode-selected offsets to a pair of entries, then scans through flagged positions updating the running maximum.
- `scan_running_max_with_offsets`: Scans flagged positions in a range, applying scaled offsets to each entry and updating the running maximum until the start of the range.
- `compute_running_max_scaled_byte`: Finds the largest value in a range of table bytes, scales it and stores the result into the output buffer as a correction for the current position.
- `compute_scaled_running_max_backward`: Repeats the running-maximum computation backward through the table and stores each scaled correction, stopping when a non-flagged entry is reached.
- `adjust_bytes_by_running_maximum`: Updates a destination byte from a source byte by a scaled running-maximum correction, then propagates a fixed adjustment over flagged positions in the range.
- `scale_nonzero_nibbles_backward`: Walks a buffer backward and, for every byte with a nonzero high nibble, multiplies its value by a signed scale factor and adds the result to the accumulator.
- `scale_bytes_with_high_nibble`: Scales bytes that have a nonzero high nibble by a signed factor selected from a flag bit, accumulating the products into the output buffer.
- `scale_bytes_by_signed_factor`: Walks a byte range backward, skipping bytes whose high nibble is zero, and multiplies the destination byte by a sign-selected factor to update the accumulator.
- `shift_and_blend_table_bands`: Computes shifted differences over bands of flagged table entries, scales them and blends the results into two tables, repeating over the band count.
- `copy_and_offset_table_bytes`: Copies table bytes forward across a range of flagged positions and adds a fixed offset to each copied value in the output table.
- `copy_runs_with_nibble_offset`: Chooses one of two copy routines by a count, otherwise copies runs of bytes and adds a nibble-scaled offset to each copied byte in the output table.
- `copy_bytes_with_nibble_offset`: Copies bytes across a range of flagged positions and adds a nibble-scaled offset to each copied byte, looping until the count is exhausted.
- `blend_run_pairs_halfway`: Blends paired run values by half their difference, writing the adjusted values back into two tables and continuing for each flagged run.
- `blend_run_pairs_alternate`: Blends paired run values by half their difference as in the neighbouring routine, with the adjustment applied to a different output table.
- `fill_runs_with_nibble_steps`: Fills each run of positions with values in fixed steps of a nibble-scaled amount, spreading the result between the run's start and end.
- `fill_range_with_scaled_steps`: Writes a nibble-scaled value into each byte of a range, then scans for the start of the next flagged run and fills it with a divided step value.
- `compute_run_max_steps`: Computes the running maximum over a run of table bytes and writes scaled step values into the output buffer, with a copy of the boundary byte.
- `shift_run_and_write_maximum`: Shifts a run of table bytes by one position, then writes the maximum of the run as a scaled value into the output buffer.
- `write_running_maximum_correction`: Finds the largest table byte in a range and stores a scaled correction derived from it at the end of the range.
- `write_scaled_running_maximum`: Accumulates table bytes over a range, keeping a running maximum, and writes a scaled value and a derived value into several output buffers.
- `scale_backward_from_flagged_entries`: Walks flagged entries backward and applies a scaled correction to the output tables, using a constant factor for the first entry of each run.
- `scale_backward_flagged_pairs`: Walks flagged entries backward in pairs, scaling each entry with a constant factor and applying the result to two output tables.
- `scale_and_accumulate_flagged_run`: Scales the entry at the current position and accumulates the result into the output table, then walks the preceding flagged positions in the same run.
- `scale_run_with_offset_table`: Scales the value at each position of a run using a factor from an offset table and writes the result into the output table, moving back through the run.
- `scale_and_store_backward_run`: Applies a scaled correction to each position of a backward run, storing the adjusted values into the output tables until the run start is reached.
- `scale_entries_with_fixed_factor`: Multiplies table entries by a fixed factor selected by a count and applies the rounded product to the output table over a range.
- `scale_entries_with_alternate_factor`: Multiplies table entries by a second fixed factor and applies the rounded product to the output table, walking the range backward.
- `scale_entries_with_small_factor`: Multiplies table entries by a small fixed factor and applies the rounded product to the output table, walking the range backward until the start.
- `adjust_buffers_by_signed_delta`: Walks the range backward, applying a signed delta derived from a table value to several output buffers wherever the flag bits match.
- `copy_bytes_with_feedback_step`: Copies bytes between two buffers while updating a feedback value from a table entry, writing the adjusted bytes to a third buffer.
- `adjust_bytes_with_feedback`: Copies bytes between buffers, updates a feedback value by multiplying it with a table entry and shifting, then subtracts the feedback from each byte before writing it out.
- `check_narrator_flag_and_dispatch`: Calls a fixed series of routines, then checks one flag byte in the state structure and calls one more routine when it is nonzero.
- `call_three_routines_in_sequence`: Calls three other routines one after the other and returns; the decompiler could not resolve the call targets.
- `interpolate_bytes_at_stride_eight`: Interpolates linearly between two source bytes and writes the results into every eighth byte of the output buffer.
- `scan_stride_records_and_dispatch`: Steps through eight-byte records until a terminator, advancing the pointer for the first record and calling a subroutine for each later record that is not a marker.
- `dispatch_stride_records_until_end`: Steps through eight-byte records until a terminator, advancing the record pointer on each full stride and calling a subroutine for the others.
- `skip_stride_records_and_call`: Walks eight-byte records, calls a subroutine when a record is not the first in its group, and returns when the terminator value is reached.
- `scan_stride_eight_until_marker`: Scans eight-byte records for the next non-zero entry, advancing the pointer after each full stride, and returns on the terminator.
- `advance_stride_records_and_call`: Advances through eight-byte records, calling a subroutine for each boundary record until the terminator is found.
- `skip_stride_records_to_marker`: Searches eight-byte records for the next nonzero byte, calling a subroutine whenever the search leaves the current group, and stops at the terminator.
- `render_table_entries_to_buffer`: Walks paired table entries and, for each flagged entry, copies a fixed pattern of rows into the output buffer; otherwise it advances the output by the entry count.
- `copy_glyph_rows_for_entries`: Copies five-row pattern data for each flagged entry into the output buffer, choosing the pattern from the symbol byte and stepping through the entry count.
- `copy_glyph_rows_stepping_entries`: Copies a five-row pattern for each flagged entry into the output buffer, selecting one of three patterns by symbol value and advancing by the entry count.
- `copy_rows_for_run_lengths`: Copies pattern rows for each run, adds a shading value to the first row of each half-run and applies a nibble reduction to the rest of the run.
- `copy_rows_run_lengths_alt`: Repeats the run-length row copy with the shading applied to a different row of each run, then finishes the remaining run.
- `copy_rows_for_second_symbol`: Copies a fixed pattern of rows for entries whose symbol byte is the second symbol value and advances the output by the entry count.
- `copy_glyph_rows_default_symbol`: Copies the default pattern rows for each entry, shading the first row of each run and reducing the nibble of the remaining rows.
- `copy_previous_entry_rows`: Copies the three bytes from the previous eight-byte row into the current row for marked entries, and handles run boundaries with a nibble update.
- `copy_previous_rows_with_mask`: Copies the previous row values forward for marked entries and updates the nibble fields of the boundary rows using a mask from the control table.
- `copy_previous_rows_nibble_mask`: Like the neighbouring routine, copies previous row values and updates nibble fields under a mask, with an extra clear step when a flag is set.
- `adjust_nibble_fields_for_entries`: Adjusts the high and low nibble fields of boundary rows for each entry, copying rows forward where the control byte requires it and clearing the fields on a sentinel.
- `adjust_nibble_fields_until_marker`: Adjusts nibble fields in the output rows for each entry and stops once the terminator value is found.
- `remap_rows_through_lookup_table`: Walks output rows until a terminator, skipping the marker value and replacing the three row bytes in each row with values from a lookup table.
- `copy_rows_from_lookup`: Copies three lookup-table bytes into each eight-byte row, stepping through the rows until a marker value is reached.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `average_column_bytes_into_row`: Sums seven bytes down a run of records, stopping at a terminator, and adds the total into the eighth byte of the record as an averaged value.
- `smooth_running_sum_into_rows`: Keeps a running sum of record bytes and writes an averaged value into each record's last byte until the terminator, with a correction from the previous row.
- `combine_weighted_row_sums`: Computes weighted sums of record bytes and stores the combined result in the record's last byte, returning when a terminator is found.
- `combine_weighted_running_sums`: Accumulates weighted byte pairs across records and writes the combined value into the final byte of each record, looping until the terminator.
- `add_class_offset_to_rows`: Adds a class-dependent offset to the rows of each entry, using a fixed value for the second symbol class and a scaled term for the rest.
- `add_class_offset_per_symbol`: Adds a symbol-dependent offset to the rows of each entry and handles one special symbol class by writing a fixed value into each row.
- `add_offset_rows_by_symbol`: Adds a fixed offset to every row of each entry, choosing the offset from the symbol class and skipping rows for the flagged symbol.
- `add_negative_offset_to_rows`: Adds a negative offset to each row of an entry, with the offset value chosen from the symbol class, and advances the row pointer.
- `add_negative_offset_rows_variant`: Adds a negative offset to rows of each entry with the same class logic as the neighbouring routine, returning on the terminator.
- `add_negative_offset_rows_alt`: Repeats the negative-offset row update for entries of a different class and returns when the terminator is reached.
- `subtract_offset_then_add_class`: Subtracts a class offset from entries that carry the flagged bit, then adds a second class-dependent value to the rows that follow.
- `apply_slope_then_offset_rows`: Applies a scaled slope term to each entry's rows and then adds a class offset to the rows of the next entry, until the terminator.
- `compute_nibble_checksums_in_groups`: Sums the low and then the high nibbles of byte groups in two passes and writes the masked result into the third byte of each group, the same checksum shape used by other routines.
- `allocate_signal_and_wait_loop`: Allocates an exec signal, reads the current task and waits on the signal mask in a loop, calling a subroutine each time a wake-up arrives.
- `poll_io_request_until_done`: Repeatedly checks an I/O request's flag bits and calls the library routine that starts it, clearing the request flags when it finishes.
- `clear_io_request_flags`: Clears two flag bits in an I/O request structure, leaving the other flags unchanged.
- `setup_speech_request_io`: Fills in a speech I/O request with its buffer, length and attribute fields, then runs the exec DoIO call to send it.
- `copy_blocks_enable_audio_dma`: Copies blocks of data into audio buffer records and writes the DMA control register to enable the audio channels, restoring the previous state afterwards.
- `mix_channel_samples_to_output`: Adds sample bytes from two channel tables for each output position, writes each sum into the output buffer twice and decrements a per-channel counter.
- `advance_audio_channel_state`: Updates a channel's phase accumulators and periods, writes sample bytes to the output and toggles DMA control when a per-channel counter reaches zero.
- `mix_channel_samples_into_buffer`: Mixes sample bytes from channel tables into an output buffer, updating each channel's position and looping through a lookup table for the next sample.
- `mix_channel_samples_with_lookup`: Adds two lookup-table sample values for each output position and writes the sum to two adjacent output bytes, switching channel buffers when a counter wraps.
- `advance_audio_sample_stream`: Advances through sample stream records, writes combined sample bytes to the output and calls subroutines when the per-channel counter expires.
- `parse_sample_stream_records`: Reads sample stream records, sets record fields from the record bytes and clears the channel fields when a new record starts.
- `wait_io_and_reenable_dma`: Waits for each pending I/O request in turn and enables the audio DMA channels in the control register if the channel flag is set.
- `enable_dma_if_flag_set`: Tests a flag byte in the channel structure and, if it is set, writes the DMA control register with the enable bit set.
- `update_channel_and_toggle_interrupts`: Advances a channel counter and copies a new byte into the channel structure, calling a library routine when the pending list is ready and setting the interrupt control register on the final step.
- `decrement_counter_and_enable_interrupts`: Decrements a counter byte in the channel structure and, when it was zero or one, writes the interrupt control register with the enable bit set.
- `stop_audio_channels_and_wait`: Loops over the channel records, calling a library routine on each one and marking each record as active, then checks a flag to decide whether to stop.
- `stop_audio_records_library`: Walks the channel records, calls a library routine for each, and restarts DMA in the control register when the record count is exhausted.
- `wait_io_then_restart_dma`: Enables DMA through the control register, waits on each channel request with a library call and marks the channel done when the request completes.
- `set_channel_done_flag`: Sets the done flag bit in a channel structure and returns.

### MESSAGE

- `call_library_entry`: Calls one routine and, if the zero flag is clear, calls a second routine and a function taken from the library table, then calls a third routine before returning.
- `open_dos_library`: Calls the exec library function to open the DOS library and stores the returned library handle in the global library variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_and_call_pointer`: Copies a fixed block of longwords from a table into the destination when the destination pointer is non-null, then calls a function pointer.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:Intro

- `run_entry_with_imp_text`: Program entry point that references the IMP! text and calls one other routine.
- `write_background_colour_register`: Writes the background colour hardware register and then calls one other routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `unpack_bit_stream_block`: Reads a packed header, shifts bits out of the source bytes and copies runs of bytes backward into the destination block until the output start is reached.
- `decode_bit_stream_length_codes`: Decodes length codes from shifted bits in a packed stream, calling subroutines between each shift to build the run length.
- `decode_bit_stream_distance_codes`: Decodes distance codes from a packed bit stream by shifting bits and accumulating them, returning the decoded value to the calling copy loop.
- `decode_bitstream_chunks_count_five`: Decodes table-indexed values from a packed bit stream with the first chunk count set to five, writing each decoded run into the output buffer.
- `decode_bitstream_chunks_count_four`: Decodes table-indexed values from a packed bit stream with the first chunk count set to four, writing each decoded run into the output buffer.
- `decode_bitstream_chunks_count_three`: Decodes table-indexed values from a packed bit stream with the first chunk count set to three, writing each decoded run into the output buffer.
- `decode_bitstream_chunks_count_two`: Decodes table-indexed values from a packed bit stream with the first chunk count set to two, writing each decoded run into the output buffer.
- `decode_bitstream_alt_count_two`: Decodes table-indexed values from a packed bit stream using a count-two variant, writing decoded runs into the output and advancing the source pointer.
- `copy_backref_runs_from_bitstream`: Reads run lengths and distances from a packed bit stream, then copies earlier output bytes forward to fill each run until the output start is reached.

### TRANSLATOR.LIBRARY

- `make_and_add_library_node`: Allocates a library structure with the exec MakeLibrary call, stores the list pointers in it and adds it to the library list, returning zero if allocation fails.
- `bump_open_count_clear_flag`: Increments the 16-bit open count of the library structure and clears one flag bit in its flags byte.
- `release_library_and_expunge`: Decrements the library open count and, when it reaches zero with the expunge flag set, calls the library routine that removes it.
- `unlink_or_mark_expunge`: Unlinks the library node and calls the library removal routine when the open count is zero, otherwise sets the delayed-expunge flag.
- `unlink_library_node_and_free`: Unlinks a library node from its list by repointing its neighbours, then calls a library routine to free it and returns its result.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `format_text_into_buffer`: Copies template text into a buffer padded with spaces, expanding the marker character by calling subroutines for each field.
- `return_low_word`: Returns the low 16 bits of the data register as the result.
- `terminate_string_and_return_word`: Calls a subroutine, clears the byte after the last written position to terminate the string and returns the low word of the result.
- `parse_text_fields_into_buffer`: Parses the input text field by field, calling subroutines for each character and storing the field text into the buffer until the marker is reached.
- `parse_text_fields_until_end`: Parses text fields in the same way as the neighbouring routine, storing each field and returning the final length when the input is exhausted.
- `scan_text_with_backslash_delimiters`: Scans text for backslash and backquote delimiters, calling subroutines at each delimiter and returning when the end condition is met.
- `scan_text_until_delimiter`: Scans characters up to a backslash or backquote delimiter, calling subroutines at each delimiter and returning once a final counter reaches zero.
- `stub_immediate_return`: Contains only a return instruction, so calling it leaves all state unchanged.
- `match_bracketed_text_pattern`: Scans to the opening bracket character and compares the following characters with a pattern string until the closing bracket, returning when the match ends.
- `match_reverse_text_rule`: Compares text read backward from the current position against a rule string, using a table of character flags, and returns early when a flagged character breaks the match.
- `library_empty_routine`: This library code is empty.
- `match_reverse_text_rule_variant`: Compares backward text against a rule string with the same flag test as the neighbouring routine, calling a subroutine once when a flagged character is found.
- `match_text_until_equals`: Compares forward text against a rule string character by character until an equals-sign terminator, using character flags to decide when to stop early.
- `match_rule_until_equals_variant`: Repeats the forward rule match until an equals-sign terminator, calling a subroutine on flagged characters and returning when the rule is exhausted.
- `copy_text_run_with_flag`: Copies a run of text characters into a buffer while checking each character's flags, calling a subroutine for flagged characters and returning the copied length.
- `copy_text_run_into_buffer`: Copies a fixed-length run of characters into the output buffer and sets a flag bit when the character after the run is a backquote.
- `copy_fixed_length_text`: Copies a run of bytes into the destination buffer, then sets a flag bit when the final byte matches the backquote delimiter.
- `terminate_text_at_word_end`: Scans backward from the current position over characters with a flag bit set, then writes a zero byte to end the word.
- `dispatch_symbol_handler_by_char`: Searches a short table of special symbol characters for the given character and calls the handler routine at the matching table index.
- `return_from_subroutine_3`: Performs a standard subroutine return to the calling routine.
- `stub_empty_return_slot`: This library code contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `forward_to_handler_routine`: Calls one subroutine and returns; the subroutine is shared with other table entries.
- `loop_while_flag_set`: Calls a subroutine repeatedly for as long as a flag bit is set in the caller's state, then returns.
- `forward_to_second_handler`: Calls a subroutine once and returns, passing control on to a second handler in the library table.
- `check_letter_pair_and_return`: Calls a subroutine to fetch a character and returns early unless it matches one of two expected letters.
- `return_from_library_routine`: This library routine is empty.
- `match_suffix_keyword_letters`: Matches a short suffix keyword by reading letters from the text and calling subroutines to check the matched pattern and flag bits, returning when a letter does not fit.
- `match_ng_s_suffix`: Matches the letters N and G in the text, then checks a further S letter and flag bit, returning early when any test fails.
- `return_from_subroutine_2`: This routine performs a single return instruction.
- `forward_call_and_return`: Calls one other routine and returns straight away, with no other work.
- `call_indirect_routine`: This library code calls another routine before returning.
- `call_indirect_routine_2`: This library code performs a call to another routine and then returns.
- `call_indirect_routine_3`: This library code executes a call to another routine and returns.
- `poll_while_bit32_set`: Calls a routine repeatedly for as long as the bit with value 32 is still set in a status register, then returns.
- `call_indirect_routine_4`: This library code invokes another routine and then returns.
- `poll_while_bit2_set`: Calls a routine repeatedly for as long as the bit with value 2 is still set in a status register, then returns.
- `call_then_test_top_bit`: Calls two routines in turn, then tests the top bit of a register, but both outcomes return with no change.
- `return_immediately_stub`: Contains only a return, with no calls, memory access or register work.
- `copy_normalized_text_field`: Copies characters from a source text into a fixed buffer, turning control codes and the delete code into spaces and changing lowercase letters to uppercase.
- `write_space_before_pointer`: Stores a space character two bytes before the address held in a pointer register, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `insert_marker_on_table_match`: Walks a text buffer and, when a byte matches an entry in a fifteen-entry table of word codes, shifts the rest of the buffer one place to the right and inserts the character 52.
- `shift_buffer_insert_marker`: Moves the bytes from a position up to a stored write pointer one place forward, places the character 52 in the freed slot, and advances the stored pointer by one.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:PAUSE

- `poll_ciaa_port_until_clear`: Busy-waits while the bit with value 64 is set in the CIA A port A register, which carries the fire buttons and disk status, then returns.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:delphine

- `load_hunks_and_relocate`: Reads the executable's hunk list, allocates memory for each code, data and BSS section through the system library, and then applies 32-bit relocation entries to the loaded code.
- `return_immediately_stub_2`: Contains only a return, with no calls, memory access or register work.
- `spin_down_count_delay`: Loops counting a register down until it passes zero, burning time with no memory access.
- `tiny_ori`: Small routine of 1 instruction that merges 80 into d0.
- `unpack_rle_stream_backwards`: Decompresses a run-length packed stream into memory, filling from a high address back toward a low limit.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_wrapping_to_space`: Decrements a byte held in a register once for each count given in the input register, and when the byte reaches zero it is set to a space character instead.
- `nop_placeholder`: This routine is empty and contains no valid instructions.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:add21k

- `clear_block_and_start`: Clears a block of twenty-three longwords, saves the system base pointer and a stored library pointer, then calls an optional startup hook when a flag is set.
- `open_required_libraries_and_setup`: Opens the libraries named in the referenced text, which are dos, graphics and intuition, through the exec library.
- `return_immediately_stub_3`: Contains only a return, with no calls, memory access or register work.
- `setup_tagged_dos_stream`: Allocates a cleared block sized at six bytes per stored entry, fills in its header fields and a four-character name tag, then looks up the current directory and input and opens a stream through the DOS library.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_command_line_args`: Copies a string into an allocated buffer and splits it into words, using spaces, tabs and line breaks as separators.
- `append_string_capped`: Finds the end of the destination string and appends the source string to it, copying up to 32767 characters.
- `append_string_limited_count`: Walks to the end of the destination string and appends the source, stopping at the source's zero byte or when the supplied count runs out.
- `copy_string_and_zero_pad`: Copies bytes from a source string into a destination until it meets a zero byte or the count runs out, then fills whatever space is left in the count with zeros.
- `open_icon_library_fill_record`: Opens the icon library, and when that succeeds follows a chain of library calls to find an entry.
- `run_optional_hook_then_return`: Calls a stored hook routine when its pointer is set, then calls another routine and returns.
- `execute_cleanup_pointer`: Checks a memory location for a non-zero value and executes the function pointed to by that value.
- `release_setup_resources`: Tears down the state built earlier, releasing each recorded resource pointer in turn.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_indexed_entry_slot`: Checks that an index lies inside the entry table and that its slot is in use, then closes the handle in that slot unless a flag marks it protected, and clears the slot.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_dos_output_routine`: Calls a function through a stored library base at a fixed negative offset, which the facts identify as the DOS output routine.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `invoke_library_vector_414`: Calls the library function at a fixed negative offset from a library base pointer kept in memory, passing through the registers already set, then returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_function_2`: Performs an indirect call to a function located at a specific offset within a library base retrieved from a global variable.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:PPMORE

- `apply_amiga_hunk_relocations`: Parses a memory block containing AmigaDOS hunk type codes to perform relocation and data copying.
- `return_immediately_stub_4`: Contains only a return, with no calls, memory access or register work.
- `decrement_register_loop`: Decrements a register value until it reaches zero to create a delay.
- `tiny_ori_2`: Small routine of 1 instruction that merges 220 into d0.
- `unpack_rle_stream_backwards_2`: Decompresses packed data backwards from the end of a region toward its start, copying runs from earlier output and writing repeated bytes.
- `decrement_byte_once_to_space`: Decrements a byte register once and replaces a zero result with a space character.
- `decrement_byte_wrapping_to_space_2`: Decrements a byte register once for each count given in the input register, and when the byte reaches zero it is set to a space character instead.
- `null_procedure`: The routine is empty and performs no operations.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:message

- `write_message_and_close_library`: Fetches the DOS output handle, and when an earlier check is not zero, writes text through the DOS library's Write function.
- `open_dos_library_2`: Opens the DOS library by name through the exec library's OldOpenLibrary call and stores the returned library handle in a global variable.
- `close_library_handle`: Closes a library handle held in a global variable by calling the exec library's CloseLibrary function through the stored base.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 3 of 3).adf:paradox

- `call_library_entry_2`: Invokes another routine and checks the zero flag.
- `open_dos_library_3`: Calls the exec library function OldOpenLibrary to load dos.library.
- `close_library`: Calls a function pointer retrieved from a global variable to close a library.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 2 of 3).adf:bootblock

- `check_boot_library_call`: Runs as the boot block entry and calls a library function at a fixed offset from the library base.

### Operation Stealth (1990)(U. S. Gold)[cr PDX][a](Disk 3 of 3).adf:bootblock

- `check_library_status`: Calls a function pointer and returns zero if the result is zero, otherwise returns a value of all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Loads minus one as the result and returns, touching no memory and making no calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_and_recurse`: Performs a bitwise OR on a byte at a calculated address and then recursively calls another routine based on a value in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: This routine is empty and performs no actions before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_subroutines`: Clears a block of memory, updates several global values, and conditionally executes a function pointer from a global variable before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_block`: Copies a block of memory, fills a global variable with a descending sequence, and performs bitwise OR operations.
- `copy_and_modify_data`: Copies a block of memory, fills a global variable with a descending sequence of values, and performs bitwise OR operations on a word and a byte at specific memory locations.
- `modify_memory_via_pointer`: Performs bitwise OR operations on a word at a specific memory location and a byte at a calculated offset.
- `copy_word_via_pointers`: Copies a word from a location pointed to by one register to a location pointed to by another register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic_chain`: Performs a series of conditional checks on a global pointer, calling various subroutines, and updating memory or configuration tables based on certain values.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_and_scale_value`: Checks a value, multiplies a global variable by two, applies an XOR mask if a carry or zero condition occurs, and returns a value involving a division and remainder calculation.
- `handle_keyboard_serial`: The routine returns a byte computed from the bitwise complement of a global variable when a flag bit is set, performs a decrementing delay loop, and clears a flag bit in the global variable.
- `populate_table_with_offsets`: Copies a block of data from a source table and populates a destination table with a series of calculated values or offsets derived from input parameters.
- `write_config_table`: Stores a series of hardcoded word values into a table located via a pointer in a global variable.
- `write_config_table_2`: Stores hardcoded word and long values into a table located via a pointer in a global variable.
- `load_pointer_from_table`: Returns a value loaded from a memory address calculated from a global variable.
- `store_raster_coordinates_2`: Updates a global integer and stores multiple sets of vertical and horizontal coordinates, including offsets, into a table.
- `store_raster_coordinates`: Waits for a specific raster line and then stores several sets of vertical and horizontal coordinates into a table.
- `copy_source_to_dest`: Copies a sequence of words from a source buffer to a destination buffer located in a table.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `perform_empty_return`: The routine performs no operations and immediately returns to its caller.
- `update_display_raster_parameters`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line, then performs a word-based memory copy from a source buffer to a destination buffer.
- `execute_nested_calls`: Performs a sequence of calls to another routine using values passed or stored on the stack.
- `get_and_increment_index`: Reads a byte from a table using an index, then increments that index stored in a pointer.
- `initialize_and_fill_table`: Clears a block of memory, then iterates through a table to clear or update entries with values retrieved from a recursive call or other routine.
- `fetch_nested_pointer`: Retrieves a long value from a memory location pointed to by a nested pointer if a previous routine returns a non-zero value.
- `fetch_nested_pointer_value`: Checks the return value of another routine and, if non-zero, retrieves a short integer from a memory location pointed to by an offset within a structure.
- `fetch_table_value`: Calls another routine to retrieve a pointer, then accesses a word from that pointer's offset plus four.
- `process_input_and_table`: Retrieves a pointer from a table to access a word and calls a routine to read input flags.
- `read_input_flags_2`: A 24-instruction routine called by one unnamed routine.
- `update_entity_state_and_initialize_tables`: Updates a status flag based on a conditional check and scales a value by two or six depending on a bitmask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic_chain_2`: Performs a series of conditional checks on a global pointer, calling various subroutines and updating configuration tables based on certain values.
- `wait_for_vposr_value`: Polls the vertical position register until it matches two specific hardware values in sequence.
- `update_and_scale_random_state`: Reads a 32-bit state word from a global at a fixed A4-relative offset; if the 16-bit argument is positive, it doubles the state, XORs it with a constant when the carry or zero condition is set, stores it back, and returns the state divided and remaindered by the argument packed into the high and low words of the result.
- `handle_keyboard_serial_2`: Checks a flag bit in a global variable, returns a byte derived from the bitwise complement of that variable, executes a decrementing delay loop, and clears a specific bit in the global variable.
- `fill_sprite_table_entries`: Brackets a series of hardcoded word stores between two library calls, writing fixed 16-bit values at fixed offsets into a structure reached through a pointer held in an A4-relative global.
- `update_config_parameters`: Stores a long value and several word values into a table located via a pointer in a global variable.
- `dereference_offset_pointer`: Calculates an address by adding a negative offset to a value held in a register and returns the longword stored at that location.
- `populate_coordinate_offset_table`: Checks if two parameters are valid and then populates a table with a series of 16-bit values and their corresponding high-byte components.
- `store_raster_coordinates_3`: Calculates several vertical and horizontal coordinate sets with offsets and stores them into a table after waiting for a raster line.
- `copy_words_to_table`: Copies sixteen words from a source buffer to a destination buffer located within a table.
- `nop_placeholder_routine`: This routine is empty and performs no operations.
- `return_from_subroutine_4`: The routine performs a single return instruction to exit the current execution flow.
- `update_display_raster_parameters_2`: Updates a table with new offset and bitmask values by calling a routine that waits for a specific raster line.
- `allocate_call_and_release`: Calls itself recursively with a stack-local slot to obtain a value, then makes a library call that uses that value together with the first parameter and a constant, passes the second parameter and the stack value to a second library call, and finally releases the handle obtained from the first library call.
- `fetch_and_increment_index`: Reads a byte from a table using an index provided by a pointer, then increments the value at that pointer.
- `initialize_data_structures`: Clears a series of memory blocks in a loop and then performs a series of operations involving a linked list or table.
- `process_table_entries_2`: Iterates through a table, performing conditional logic and updating memory locations with values from a routine and a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_memory_values`: Sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_entries`: Iterates through a table, calling a routine for each entry, and updates a counter and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `traverse_and_reset_list`: Iterates through a linked list, calling a routine for each node, and then sets a 4-byte memory location to zero and a 2-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_data_transfer`: Iterates through a table using an index, stores values into a destination buffer, and calls a subroutine for each entry until a limit is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_linked_list`: Follows a linked list of pointers, passing each node to another routine until a null pointer is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_indexed_transfer`: Calculates an offset based on a counter and a bitwise operation to fetch data from a table and store it in a buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list`: Iterates through a linked list by updating a pointer to the next element in the chain via a subroutine call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_buffer_fill`: Repeatedly stores a constant value into a memory location and increments a counter until a specific threshold is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_external_routine`: Executes another routine before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_5`: This routine performs a return from a subroutine.
- `execute_conditional_function`: Calls a routine with specific parameters and, if successful, performs further operations using a value from a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `swap_linked_list_nodes`: Traverses a linked list and performs a series of pointer swaps and assignments involving a returned value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_cleanup`: Checks a status flag and, if not set, calls two routines using values from a local variable and a global variable.
- `fetch_data_from_pointer`: Attempts to retrieve a value from a pointer stored at an offset from a global variable and returns a short value from a secondary pointer.
- `fetch_data_from_pointer_2`: A 43-instruction routine called by one unnamed routine.
- `initialize_game_state_tables`: Initializes a series of global pointers and clears or populates memory-resident tables, including a loop that zeros out a large block of memory.
- `clear_specific_memory_values`: Sets a four-byte memory location to zero and a two-byte memory location to all ones.
- `traverse_linked_list_and_reset`: Iterates through a linked list, calling another routine for each node, and then sets a four-byte memory location to zero and a two-byte memory location to all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list_2`: Recursively calls itself using a pointer or value from a table and updates a global variable before setting a certain memory location to a maximum value.
- `fill_buffer_with_values`: Iterates through a buffer to fill it with bytes from another routine and then checks a table of values to return an index.
- `search_table_recursively`: Iterates through a table to find a matching value by performing calculations involving division and nested routine calls.
- `search_table_loop`: Iterates through a table to find a matching value and performs calculations involving division and nested routine calls.
- `search_table_with_recursion`: Iterates through a table to find a matching value, performing calculations involving division and nested routine calls.
- `search_and_recurse_table`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values if a match is found.
- `search_and_recurse_table_2`: Iterates through a table of structures to find a matching key, then recursively calls itself or another routine on a calculated range of values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_linked_list_3`: Recursively traverses a linked list by calling itself with a pointer retrieved from a table entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_recursive_operations`: Calls another routine with a fixed value, and if successful, performs several recursive calls involving calculated or passed parameters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_memory_range`: Fills a memory range with bytes obtained from another routine based on a count, then performs a loop before returning a masked value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_process_table`: Iterates through a table to clear specific memory locations, then performs several calls using calculated offsets and values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_action`: The routine consists of two instructions that do nothing but return to the caller, with no referenced text, library calls, hardware register accesses, or other observable effects in the evidence provided.
- `find_matching_table_entry`: Scans a table of 20-byte records for one whose key word equals the second parameter; for a matching record it walks a sub-list in steps of 13, recursively invoking itself on each element and stopping if any call returns zero.
- `validate_and_modify_entry`: Checks a global variable and attempts to locate and modify a specific entry in a table through multiple conditional checks and memory operations.
- `calculate_and_store_area_data`: Retrieves two short values from a memory location, calculates their product, and performs a series of structured writes to a stack-based buffer or data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_multiple_subroutines`: Calls three different subroutines with specific offsets and then stores a value from a register into a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_product_and_update`: Calculates the product of two short values, stores the result in a global variable, and performs several subroutine calls and updates.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_complex_sequence`: Performs multiple subroutine calls, calculates a product of two short values, and updates a global variable before calling other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_extended_sequence`: Calculates a product of two short values, updates a global variable, and executes a series of subroutine calls using various offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_minimal_sequence`: Calculates a product of two short values, stores it in a global variable, and calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_product_and_update_state`: Multiplies two short values and stores the result in a global variable.
- `decode_bitstream_run_length_data`: Decodes a variable-length bitstream to reconstruct data through a combination of run-length expansion and literal bit extraction.
- `unpack_bitstream_data`: Processes a bitstream to extract values and writes them into a memory location pointed to by a pointer.
- `decode_variable_length_bitstream`: Reads a bitstream to reconstruct values and writes them to memory using a pointer.
- `unpack_bitstream_data_2`: Reads bits from a buffer to reconstruct values and performs memory copies based on the decoded data.
- `decrement_global_counter`: Decrements a global variable from an initial value derived from a register until it reaches zero.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `return_empty_stack`: The routine performs no operations and returns a value currently held on the stack.
- `process_compressed_data_stream`: Iterates through a table to clear specific entries before performing a bitstream decompression routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_disk_error`: Updates a global variable and enters an infinite loop after performing several memory operations and function calls.
- `infinite_loop_with_function_call`: Executes a loop that calls itself and another routine repeatedly without a termination condition.
- `update_graphics_parameters`: Performs several routine calls and calls a function with calculated offsets and a fixed value, likely intended for a display or certain hardware parameters.
- `calculate_text_box_dimensions`: Iterates through a null-terminated string to find the longest line by counting characters and identifying line breaks.
- `parse_and_render_text_string`: Iterates through a null-terminated string to calculate text wrapping and line positioning.
- `parse_sprite_animation_stream_3`: Iterates through a data stream to extract frame indices and calculate memory offsets for sprite rendering.
- `parse_sprite_animation_stream_2`: Iterates through a data stream to extract frame indices and calculate memory offsets for sprite rendering.
- `parse_sprite_animation_stream`: Iterates through a data stream to extract frame indices and calculate memory offsets for sprite rendering.
- `process_sprite_animation_sequence`: Iterates through a data table to fetch frame indices and updates memory offsets for drawing graphics.
- `update_sprite_animation_state`: Iterates through a sprite data table to fetch frame indices and updates memory offsets for drawing.
- `render_sprite_table_elements`: Iterates through a data table to retrieve byte values and uses them to calculate memory offsets for drawing graphics.
- `process_input_string_buffer`: Iterates through an input stream to validate alphanumeric characters and control codes.
- `parse_and_render_string`: Iterates through a character buffer to validate alphanumeric characters and write them to a display buffer.
- `render_text_buffer`: Iterates through a character buffer to calculate display offsets and write character data.
- `update_position_and_buffer_2`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source.
- `update_position_and_buffer`: Calculates a difference between a table value and a global variable, updates a global offset, and copies a sequence of 16 words from a source table to a destination buffer.
- `copy_cstring`: Copies a null-terminated byte string from a source pointer to a destination pointer, byte by byte, and terminates the destination with a zero byte.
- `search_string_table`: Iterates through a table of pointers to strings and uses string comparison to find a match for a provided string.
- `find_trackdisk_device`: Calls the exec library to locate the trackdisk.device and extracts a value from a device structure, returning it combined with an input register.
- `initialize_trackdisk_device`: Locates a trackdisk device and attempts to perform an operation on it, returning a failure code if the device cannot be accessed or if a loop limit is exceeded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_stack_data`: Manipulates several local buffers through a series of function calls and conditional branches based on an input parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_decryption_loop`: Initializes several values on the stack and enters a loop that performs XOR operations on a byte through a pointer until a function returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_xor_transformation`: Repeatedly calls functions and performs a bitwise XOR operation on a byte at a specific memory location until a condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_linked_list_nodes`: Traverses a linked list by following pointers stored in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_recursive_loop`: Recursively calls itself and other routines to perform a series of operations involving a loop that updates a pointer and writes values to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_conditional_recursion`: Checks values at specific offsets from a pointer to decide whether to recurse or execute a loop that modifies memory via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_memory_update_loop`: Executes a loop that writes specific constant values and data from a table to memory locations using a pointer that increments by six each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_linked_structure`: Recursively traverses a structure by following pointers and copies a value from a specific offset of a retrieved structure into a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `switch_record_and_copy_words`: Returns early if the requested index equals the currently stored index.
- `copy_string_offset_based`: Copies a null-terminated byte string from a source address calculated by multiplying a parameter by four to a destination address.
- `search_linked_list_string`: Traverses a linked list of structures to find a node where a specific string matches a value within the node.
- `get_trackdisk_device_id`: Locates the trackdisk.device using a library call and extracts its device number from the device structure.
- `poll_disk_status`: Repeatedly calls a routine to check a status bit, incrementing a counter if a specific flag is set.
- `allocate_and_copy_data`: Allocates a block of memory, attempts to copy data from a source to the new block, and retrieves a pointer from the allocated block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_initialize_session_block`: Allocates a 6-byte-per-entry block sized from a stored count, zeroes and sets the status fields of the new entry, and writes a four-character tag into a freshly allocated node chained from a global list pointer.
- `sync_data_buffers`: Compares two data strings and, if they match, traverses a linked list of pointers to populate a new data structure.
- `synchronize_dos_lock_and_buffer`: Compares a string against a stored value and, if they match, calculates a list of linked memory pointers.
- `execute_indirect_table_calls`: Compares a byte from a table with a value from a global variable and, if they match or if the value is zero, enters a loop that performs indirect calls to addresses retrieved from a table using offsets and calls a library function via a global base.
- `copy_null_terminated_string`: Copies a null-terminated string from a source address to a destination address.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `clear_and_update_globals`: Zeroes out a block of memory, updates a global or a table entry with a constant, and then copies a global variable to a new memory location.
- `return_from_subroutine_6`: The routine is empty and performs no operations before returning.
- `parse_quoted_string_tokens`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `parse_window_data`: Parses a format string and copies data into a structure.
- `store_and_parse_string`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte into the memory location pointed to by the loaded pointer.
- `recurse_with_concatenation`: Recursively calls itself while returning a 64-bit value formed by concatenating a register's previous contents with the new return value.
- `parse_format_string`: Parses a formatted string containing specifiers for integers, floating-point numbers, and strings.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `execute_conditional_pointer`: Checks if a value at a certain offset from a pointer is non-zero and, if so, executes the function pointer located at that memory location.
- `execute_conditional_table_logic`: Checks various memory locations or table-based pointers and executes corresponding functions or jumps if the pointers or certain flags are non-null.
- `nop_sequence`: The routine is empty and contains no executable instructions.
- `close_resource_by_index`: The routine validates an index against a range and a null check, then either calls another routine with a pointer from a table or sets a status value and returns an error.
- `execute_indirect_function_3`: The routine retrieves a function pointer from a table offset and executes it.
- `call_dos_close`: Calls a function via a pointer located within a structure.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_4`: The routine retrieves a function pointer from a different table offset and executes it.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure and executes it.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_dos_input`: Loads a function pointer from a table offset and calls the DOS input routine.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer_11`: Retrieves a pointer from a table using an offset from a base address and executes the function located at the resulting destination.
- `call_dos_open`: Loads a function pointer from a table offset and calls the DOS open routine.
- `call_dos_output`: Loads a function pointer from a table offset and calls the DOS output routine.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_dos_seek`: Loads a function pointer from a table offset and calls the DOS seek routine.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_dos_unlock`: Loads a function pointer from a table offset and calls the DOS unlock routine.
- `call_dos_write`: Loads a function pointer from a table offset and calls the DOS write routine.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_function`: Loads a function pointer from a table offset and calls it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_12`: Retrieves a pointer from a table using an offset and executes the function located at the resulting address.
- `call_library_function_3`: Loads a function pointer from a table offset and calls a library routine.
- `initialize_device_structure`: The routine locates a device, allocates a structure, populates several fields within that structure using provided parameters, and then returns a pointer to it.
- `initialize_device_structure_2`: Allocates a structure, populates several fields within it using parameters, and executes a nested function pointer.
- `perform_recursive_data_copy`: Copies data from one memory location to another and performs a series of recursive calls based on the contents of a memory location.
- `offset_recursive_call`: Calculates an offset by adding a constant to a register value and passes the result as an argument to another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_nested_function_pointer`: Retrieves a function pointer from a nested table structure by applying an offset to a pointer found within a table and executes it.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested table structure and executes it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Fetches a function pointer from a table using an offset from a base address and executes it.
- `execute_indirect_function_pointer_2`: Fetches a pointer from a table located at an offset from a base address and executes the function at that location.
- `execute_indirect_pointer_2`: Retrieves a function address from a table via an offset and performs an indirect call.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `execute_indirect_pointer_3`: Calls a function through a pointer located at a specific offset within a table.
- `call_indirect_function_3`: Executes a function via a pointer located at a specific offset within a table.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_indirect_function_4`: Calls a subroutine by accessing a pointer through a base address and a negative offset.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_indirect_pointer_4`: Retrieves a function address from a table and jumps to that location.
- `call_indirect_function_5`: Retrieves a function address from a table and jumps to it.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_indirect_function_6`: Uses an offset to locate and execute a function pointer from a data structure.
- `update_state_structure`: Updates several fields in a global structure, including writing a constant value and performing conditional logic based on function return values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reinitialize_string_structure`: Resets a string structure's metadata, sets certain flags, and writes a constant string or a new pointer back to a global structure.
- `sync_data_buffers_2`: Compares two strings and, if they match, calculates the size of a linked list and populates a new structure with the list elements.
- `sync_disk_buffer_state`: Compares a string in a data buffer with a stored string and updates internal tracking structures.
- `execute_indirect_calls`: Performs indirect calls to a target address retrieved from a table using a negative offset and calls a library function via a global base.
- `copy_null_terminated_string_2`: Copies bytes from a source address to a destination address until a null terminator is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_byte_strings_with_limit`: Compares two null-terminated byte strings byte by byte up to a maximum length.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_table`: Iterates through a list of values to perform lookups in a table and checks for specific bit patterns or equality against a constant list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub_5`: The routine consists of a single return instruction and performs no work: it saves nothing, touches no hardware registers, calls no library routines, and references no text.
- `check_processor_flags`: Returns one if the carry and zero flags are both clear, otherwise returns a negative value.
- `memmove_overlapping_bytes`: Copies a specified number of bytes from a source to a destination.
- `clear_and_update_globals_2`: Zeroes out a block of memory, updates a global or a table entry with a constant, and then copies a global variable to a new memory location.
- `return_empty_subroutine`: This routine performs no operations and immediately returns to its caller.
- `parse_command_string`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `copy_string_with_limit_4`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures the destination is null-terminated.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `update_global_and_parse`: Stores a passed parameter into a global variable and then calls a routine to parse a format string.
- `increment_pointer_and_store_byte`: Loads a pointer from a global variable, increments that global variable, and stores a byte from a register into the memory location pointed to by the loaded pointer.
- `perform_recursive_division`: Calls a routine to perform a 32-bit integer division and concatenates the result with a register to return a 64-bit value.
- `parse_format_string_2`: Parses a format string to extract and convert values from a source buffer into specific data types.
- `compute_signed_division_remainder`: Determines the sign of a quotient based on two signed inputs, performs a 32-bit division, and returns a 64-bit value containing the signed quotient and the original divisor.
- `concatenate_call_result`: Calls another routine and returns a 64-bit value by concatenating the original contents of a register with the new value left in that register by the called routine.
- `execute_recursive_division`: Calls a routine that performs a 32-bit integer division and then returns.
- `perform_32bit_division_2`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `call_dos_close_2`: Calls a function pointer located within a table to execute a DOS close operation.
- `call_dos_createproc`: Calls a function pointer located within a table to execute a DOS create process operation.
- `call_dos_currentdir`: Calls a function pointer located within a table to execute a DOS current directory operation.
- `call_dos_duplock`: Calls a function pointer located within a table to execute a DOS duplicate lock operation.
- `call_dos_examine`: Calls a function pointer located within a table to execute a DOS examine operation.
- `call_dos_info`: Calls a function pointer located within a table to execute a DOS info operation.
- `call_dos_input_2`: Calls a function pointer located within a table to execute a DOS input operation.
- `call_dos_lock`: Calls a function pointer located within a table to execute a DOS lock operation.
- `call_dos_open_2`: Loads a function pointer from a table offset and calls the routine associated with opening a file.
- `call_dos_output_2`: Loads a function pointer from a table offset and calls the routine associated with outputting data.
- `call_dos_read`: Loads a function pointer from a table offset and calls the routine associated with reading a file.
- `execute_indirect_function_pointer_13`: Retrieves a pointer from a table using an offset, applies a negative displacement to that pointer, and executes the resulting address.
- `call_dos_seek_2`: Loads a function pointer from a table offset and calls the routine associated with seeking within a file.
- `call_dos_unlock_2`: Loads a function pointer from a table offset and calls the routine associated with unlocking a file.
- `execute_indirect_function_pointer_14`: Retrieves a pointer from a table, applies a negative offset to it, and calls the resulting address.
- `call_dos_write_2`: Loads a function pointer from a table offset and calls the routine associated with writing data.
- `call_library_function_4`: Loads a function pointer from a table offset and calls an unspecified library routine.
- `call_indirect_from_table`: Loads a function pointer from a table offset and calls it.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a table located at an offset from a base address and executes the function found at the resulting destination.
- `execute_indirect_function_2`: Fetches a function pointer from a nested memory structure and executes it.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a table using an offset.
- `update_object_properties_2`: Updates a structure with provided parameters and executes an indirect function pointer.
- `update_object_properties`: Copies data from a source structure into a destination structure and updates a pointer field.
- `execute_indirect_chain`: Stores a pointer into a memory location, increments it, and then executes a series of indirect function calls using pointers from a table or nested structure.
- `setup_pointer_chain_3`: Stores the address of a pointer argument into a memory location and increments the stored value by four.
- `execute_indirect_function_pointer_4`: Fetches a pointer from a table located at an offset from a base address, applies a negative offset to that pointer, and calls the resulting address.
- `execute_indirect_function_pointer_5`: Fetches a function pointer from a nested offset within a structure and executes it.
- `call_indirect_pointer`: Fetches a pointer from a table using an offset and executes the routine at that address.
- `execute_indirect_function_pointer_6`: Fetches a pointer from a table located at an offset from a base address and executes the function at that location.
- `execute_indirect_function_pointer_7`: Fetches a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_function_pointer_8`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `execute_indirect_pointer_5`: Fetches a pointer from a table using an offset from a base address and executes the function found at a secondary offset within that pointer.
- `execute_indirect_function_pointer_9`: Fetches a pointer from a table located at an offset from a base address and executes the function found at a secondary offset within that pointer's target.
- `execute_indirect_function_pointer_10`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_indirect_via_offset`: Fetches a function pointer from a nested structure using a base address and two offsets, then executes it.
