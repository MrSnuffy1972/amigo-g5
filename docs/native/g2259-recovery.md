# Game 2259 (worms (1995)(ocean)(m3)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02259). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2709160 top-level bytes in 226 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 62 of 62 units identical (bytes and relocations), 88 raw-word lines |
| Functions named and described | pass, 324 of 324 named (0 author symbols), 324 of 324 described (37 by rule, 86 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1272 of 1279 (99.45%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4726 of 4733 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2000757 |
| code | 14340 |
| data | 442876 |
| padding | 43315 |
| unused | 207872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:Run | 2 | 117 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:INSTALL | 3 | 205 | 7 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:OCEAN | 4 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:FLT.1995 | 1 | 233 | 14 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:RunBack | 3 | 163 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:TEAM17 | 4 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:FLOPPYWORM | 4 | 355 | 26 |
| mathtrans.library | 18 | 830 | 97 |
| diskfont.library | 15 | 1040 | 76 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:WORMS | 3 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:HDWORM | 4 | 338 | 26 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 49 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 111 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 138 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 84 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 197 | 5 |

## Cases handled

- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:TWSTUFF/TW6: CrM2: Crunch-Mania LZH-mode-packed, unpacked with ancient (41486 -> 61440 bytes)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 586 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 29 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 92 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 135 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 59 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 137 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: not identified statically (indirect store)
- worms (1995)(ocean)(m3)[cr flt](disk 2 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- worms (1995)(ocean)(m3)[cr flt](disk 3 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:INSTALL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:OCEAN: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:TEAM17: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:WORMS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When the destination pointer is non-null, copies sixteen longwords from a fixed table into it, then calls a code pointer held in a register.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:INSTALL

- `free_memory_and_run_cleanup`: Program entry that calls a routine, invokes a code pointer from a register and calls two more routines.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_decompression_tables`: Clears a memory block and populates several tables using bit-stream decoding and iterative value calculation.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:OCEAN

- `run_cleanup_then_free_memory`: Runs the same cleanup sequence as its duplicate in another installer: it calls routines, invokes a code pointer, frees memory through a library call and clears a longword.
- `decrement_to_minus_one_2`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:FLT.1995

- `run_startup_until_zero`: Program entry that runs several routines and repeatedly calls one of them until it returns zero.
- `open_intuition_and_disable_interrupts`: Opens the intuition library and the topaz font through library calls, then writes the interrupt control register to disable interrupts.
- `wait_until_result_nonzero`: Repeatedly calls a routine until the result register it reads from holds a nonzero value.
- `poll_routine_until_nonzero`: Calls a routine in a loop and stops when the result register holds a nonzero value.
- `advance_index_with_wrap_check`: Increments a byte counter stored in a global and compares it with a fixed limit of sixteen.
- `return_empty_stub`: Returns at once without any work.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `empty_routine_2`: the routine contains no executable instructions and performs no operations.
- `set_chip_bits_and_permit`: Calls two routines, then writes three saved words, each with the top bit set, into fixed offsets from a base register.
- `clear_chip_control_words`: Writes the value 32767 into three words at fixed offsets from a base register.
- `return_empty_stub_four`: Returns at once with no action.
- `open_graphics_save_chip_state`: Forbids multitasking and opens the graphics library through a library call.
- `wait_for_status_byte_sequence`: Polls a status byte at a fixed offset from a base register, first until it reads minus one and then until it reads the letter X. It returns nothing.
- `build_four_pointer_table_entries`: Fills four pairs of words in a table with the high and low halves of an address that steps by 10240 bytes each time.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:RunBack

- `start_command_as_background_task`: Program entry that parses its argument string, opens the dos library and starts the requested command in the background with input and output sent to the null device.
- `report_missing_name_message`: Measures a zero-terminated name and passes it to two library functions, with the message text 'Can't find' in its data.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:TEAM17

- `run_cleanup_then_free_block`: Performs the same cleanup sequence as its duplicates in other installers: it calls routines, invokes a code pointer, frees memory through a library call and clears a longword.
- `busy_wait_short_countdown`: Decrements a short counter held in a register until it reaches minus one, then returns.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:FLOPPYWORM

- `find_maximum_list_field_value`: Program entry that walks a linked list and tracks the largest value found in a field of each entry, starting from a fixed minimum of 149.
- `lookup_library_values_into_globals`: Calls a library function with a fixed string, clears a byte in a buffer and stores the returned value in a global.
- `free_memory_then_close_library`: Calls three routines, including wrappers for freeing memory and closing a library, and returns zero.
- `store_four_results_in_globals`: Calls a routine four times in succession and stores each returned value in one of four global variables at fixed offsets.
- `allocate_memory_plus_offset`: Allocates memory through a library call and, when the allocation succeeds, returns its address plus 32768.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `retry_open_library_until_success`: Calls a library open function repeatedly, with an empty loop body, until it returns a nonzero library base.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `pass_through_single_call`: Calls another routine and returns, with nothing else done.
- `copy_bytes_eight_byte_blocks`: Copies a byte sequence from a source pointer to a destination pointer, moving eight bytes per pass when the length is at least eight.
- `reset_two_globals_return_pointer`: Clears two global variables, calls another routine and returns a code pointer to a routine in the same file.
- `poll_until_condition_flag`: Calls a routine repeatedly until the condition flag from its last operation is set.
- `allocate_node_and_push`: Allocates a node through a library call and links it at the front of the list held in a global, storing the owner address in it.
- `unlink_list_node_and_free`: Searches the list held in a global for the node at the given address, unlinks it when found and releases it through a library call.
- `find_list_tail_pointer`: Walks a linked list from the pointer in a register until the next link is null, then stores the final node in a global.
- `preserve_global_across_two_calls`: Saves a global list pointer, calls two routines and restores the saved pointer afterward.
- `save_restore_global_d0_2`: The routine saves a global variable to a local register, calls two other routines, and then restores the global variable to its unable value.
- `allocate_block_and_track`: Allocates a block through a library call and links it at the front of the list held in a global.
- `release_node_from_global_list`: Searches the list held in a global for the node that matches its argument, unlinks it when found and releases it through a library call.
- `launch_background_run_command`: Walks a linked list of entries and, once it reaches the end, makes a library call with command text that contains RunBack and TEAM17 and redirects output to the null device.
- `allocate_and_push_global_node`: Allocates a block through a library call and pushes it onto the front of a global list, clearing its second longword.
- `free_global_list_nodes`: Removes the first node from a global list and releases it through a library call, then releases each node in the chain it points to the same way.
- `tiny_ori`: Small routine of 1 instruction that merges 5 into d0.
- `find_largest_field_then_setup`: Walks a linked list to find the largest value stored in a field of its entries, starting from a fixed minimum of 149.
- `copy_sixteen_longs_then_invoke`: When the destination pointer is non-null, copies sixteen longwords from a table in the object into it, then calls a code pointer held in a register.

### mathtrans.library

- `empty_routine_3`: the routine is empty and performs no operations or any modification to registers or memory.
- `store_argument_in_global`: Stores the register argument in a global variable and returns.
- `release_entry_when_unused`: When the entry's counter field is zero, unlinks it from its list, releases it through a library call and returns the stored global value.
- `mark_entry_pending_release`: Sets the flag bit with value eight in the entry's flag byte and returns zero.
- `acquire_library_entry_usage`: Adds a user to a library entry's counter unless its flag bit with value eight is set.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `release_library_entry_usage`: Removes one user from a library entry's counter.
- `process_float_mantissa`: This library routine handles a packed floating-point value by returning immediately if the value is zero or passing negative values to another routine.
- `run_float_mantissa_steps`: Handles a packed floating-point value with a 23-bit mantissa: it returns at once for zero, passes negative values to another routine, and otherwise runs a 23-step loop that shifts the mantissa against a fixed bias.
- `shift_subtract_bit_loop`: Repeatedly shifts and subtracts bit by bit using two input registers and a counter, stopping when the counter reaches minus one.
- `adjust_status_flag_bits`: Calls a library routine, then returns its result with the flag bit of value eight cleared and the flag bit of value two set.
- `run_nine_library_call_sequence`: Calls a fixed sequence of nine library functions through offsets from a base register, mostly alternating between two of them.
- `dispatch_float_operation_by_sign`: Runs library calls chosen by two sign flags and the sign of a value, then returns a result from another routine with bits adjusted.
- `dispatch_float_operation_variant`: Variant of its neighbouring entry that runs library calls chosen by sign and zero flags, then returns a masked result from another routine.
- `select_float_op_by_flags`: Selects library calls by a zero test on one value and a sign test on another, then returns a result from another routine with a bit mask applied.
- `dispatch_float_op_zero_test`: Calls library functions chosen by a zero test and a sign test on the operands, then returns a result from another routine with two flag bits cleared.
- `finish_float_op_clear_bits`: Calls library functions chosen by a sign test and a secondary value, then returns a result from another routine with two flag bits cleared.
- `finish_float_op_single`: Calls one library function, then returns the result of another routine with two flag bits cleared.
- `finish_float_op_double_call`: Calls a library function twice, then returns the result of another routine with two flag bits cleared.
- `finish_float_op_paired_calls`: Calls a library function twice through a second library base, then returns the result of another routine with two flag bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `normalise_float_bits`: Clears the sign bit of a value, checks an exponent range and shifts the bits by a count derived from the exponent before calling routines to store the result.
- `run_shift_compare_loop`: Runs a 24-step loop that shifts a value and compares it with another, subtracting the shifted amount at each step where it fits.
- `run_shift_add_loop`: Runs a loop that shifts and adds a value, then subtracts a shifted remainder while counting a bit position down to minus one.
- `check_operand_byte_range`: Tests the low byte of the operand against a fixed threshold and returns early when it lies outside the range handled here.
- `check_signed_byte_thresholds`: Tests a signed byte of the operand against fixed thresholds and returns early for values in the ignored range.
- `check_double_byte_threshold`: Doubles the low byte of the operand and compares it with two thresholds, returning early above the upper limit.
- `scale_operand_when_in_range`: Checks the low byte against a threshold and, when it is in the handled range, calls three routines.
- `handle_sign_bit_operand`: Splits the operand by its sign bit and calls routines for the negative and positive cases.
- `reduce_angle_by_fixed_constant`: Reduces an operand by repeatedly adding or subtracting a fixed multiple of a constant until it falls within one period.
- `compute_adjusted_operand_value`: Computes an adjusted operand from a fixed table of constants and the operand's low byte, looping until the value is in range.
- `iterate_constant_series_terms`: Iterates a series of shifted terms built from two fixed constants and the operand, updating an accumulator until a step count runs out.
- `accumulate_series_terms_loop`: Repeatedly adds and subtracts terms held in several registers while decrementing a count.
- `return_empty_stub_five`: Returns at once with no work.
- `recursive_call_wrapper`: The routine contains no instructions and immediately returns.
- `call_two_routines_in_order`: Calls two routines in sequence and returns, with no other work.
- `pack_integer_to_float_form`: Packs a 32-bit integer in a register into a floating-point form with a biased exponent, normalising it by shifting.
- `pack_integer_to_float_entry`: Packs the integer in a register into the same floating-point form as its neighbouring entry, normalising it by shifting.
- `call_one_then_two_routines`: Calls one routine, then calls two more because a local flag stays false.
- `call_one_then_three_routines`: Calls one routine, then calls three more because a local flag stays false.
- `step_byte_and_call_routines`: When the byte argument is nonzero and does not overflow when incremented, calls a routine, then calls two routines with the incremented byte and one more routine.
- `return_immediately`: Returns at once with no visible work in its body.
- `return_immediately_2`: Returns at once with no visible work; the body holds only a return.
- `return_without_action`: A bare return with no work, in mathtrans library code.
- `return_from_empty_stub`: The listing has five instructions, but the decompiled body is only a return, so no observable work happens.
- `exit_without_work`: Four instructions in mathtrans library code that return straight to the caller with no observable work.
- `normalize_float_mantissa`: Returns at once when its input register is zero.
- `call_four_library_routines`: Makes four calls to other mathtrans library routines in sequence and returns, with no arithmetic in its own body.
- `add_table_until_nonnegative`: Walks a table of 32-bit entries from a pointer and a count.
- `subtract_table_until_negative`: Adds the first table entry to an accumulator, then subtracts following entries while the accumulator stays non-negative.
- `run_bitwise_mantissa_division`: For a non-zero positive input it runs a bit-by-bit subtraction loop of up to 22 steps on a shifted mantissa, then applies a biased exponent shift of up to 71 steps.
- `return_immediately_3`: A two-instruction mathtrans library routine whose body is only a return.
- `run_bitwise_mantissa_loop`: Runs the same bit-by-bit mantissa subtraction loop of up to 22 steps as its sibling, without the zero or sign checks.
- `shift_and_subtract_bits_loop`: Repeatedly subtracts a shifted accumulator from a value while advancing a shift count, with a step limit.
- `call_three_library_routines`: Calls three other routines in order and returns.
- `dispatch_negative_or_normalize`: If the low byte is negative it calls one routine and returns.
- `normalize_float_after_calls`: Makes two calls first and returns at once when the input is zero.
- `return_without_work`: Its body contains no operation and only returns to the caller.
- `exit_without_operation`: Returns straight away with no operation in its body, although five other routines refer to it.
- `leave_without_action`: Ten-instruction mathtrans library routine whose decompiled body is only a return, so it performs no visible work.
- `return_without_action_2`: Six-instruction mathtrans library routine whose body is only a return, with no visible work.
- `empty_library_routine`: This library routine is empty and immediately returns.
- `return_from_empty_routine`: Five-instruction mathtrans library routine whose body is only a return, with no visible work.
- `return_immediately_4`: Two-instruction mathtrans library routine whose body is only a return, with no visible work.
- `call_pointer_return_value`: Calls the routine whose address is passed in an address register, then returns the value that was in the data register on entry, unchanged.
- `invoke_pointer_pass_value`: Calls the function whose pointer is held in an address register, then returns the value that was in the data register on entry.
- `call_routine_pass_value`: Calls one other routine and then returns the value that was in the data register on entry, unchanged.
- `call_library_stub_pass_value`: Calls one other routine and then returns the value that was in the data register on entry, unchanged.
- `lookup_and_clear_buffer`: This library routine calls a library function with a fixed string, clears a byte in a buffer, and stores the returned value in a global.
- `find_max_list_field`: This library routine calls a library function with a fixed string and then walks a linked list to find the largest value in a field, starting from a fixed minimum.
- `split_sign_bit_operand`: This library routine calls a library function with a fixed string and then splits an operand by its sign bit to call separate routines for positive or negative cases.
- `check_byte_threshold`: This library routine calls a library function with a fixed string and then doubles the low byte of an operand to compare it against two thresholds.
- `pass_register_value`: This library code returns the value held in a data register unchanged.
- `store_register_via_pointer`: The routine invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `store_value_after_call`: Calls one other routine, then writes the 32-bit value from the data register to the memory address held in an address register, and returns.
- `execute_multiple_routines`: This library code calls one routine and then calls two more routines.
- `step_and_call_routines`: This library code increments a byte and calls a routine, followed by two more routines if the byte does not overflow.
- `return_if_zero`: This library code returns immediately if the input register contains zero.
- `run_bitwise_division`: This library code runs a bit-by-bit subtraction loop on a shifted mantissa and applies a biased exponent.
- `call_three_routines`: This library code calls three other routines in a specific order and then returns.
- `dispatch_negative_input`: This library code calls another routine if the low byte of a register is negative.
- `return_without_work_2`: This library code contains no operation and simply returns to the caller.
- `exit_without_operation_2`: This library code returns immediately without performing any operations.
- `call_library_offset_pass_value`: Calls a library function at a fixed negative offset from a library base kept in a global variable, then returns the value that was in the data register on entry.
- `call_library_offset_pass_value_2`: Calls a different library function at another fixed negative offset from the global library base, then returns the value that was in the data register on entry.
- `call_library_offset_pass_pair`: Calls a library function through a fixed offset from the global library base, then returns the values in two data registers combined as one 64-bit result.
- `call_library_offset_pair`: Library code that calls another routine through a fixed offset and returns the combined contents of two registers.
- `call_library_offset_pass_value_3`: Calls a library function at a fixed negative offset from the global library base, then returns the value that was in the data register on entry.
- `call_library_indirect`: Library code that calls a routine at a library-relative offset and returns the value held in a data register.
- `call_library_offset_pass_pair_2`: Calls a library function through a fixed offset from the global library base, then returns the two data registers combined as one 64-bit value.
- `call_library_offset_pass_pair_3`: Calls a library function through a fixed offset from the global library base, then returns the two data registers combined as one 64-bit value.
- `call_library_offset_pair_alt`: Library code that executes a routine at a fixed library offset and returns a 64-bit value formed from two registers.
- `call_library_offset_pass_pair_4`: Calls a routine through a fixed offset from the global library base, then returns the two input registers combined as one 64-bit value.
- `call_library_indirect_offset`: Calls a routine at the global library base plus an offset passed in an address register, then returns the value that was in the data register on entry.
- `normalize_fixed_point_value`: Returns at once for a zero input.

### diskfont.library

- `store_value_in_handle`: Calls a library function through the library base in an address register.
- `set_handle_value_if_found`: Calls a library function through the library base and, when it returns a handle, stores a value at a fixed offset inside it.
- `open_libraries_init_list`: Stores its argument and the library base in globals, then opens two libraries through the exec open function.
- `close_library_and_free_memory`: Calls the library close function and then the memory free function on its argument, then returns zero.
- `remove_idle_entries_unload_seg`: Walks a linked list of entries, unlinking each one whose usage count is below one and decrementing a counter for each removal.
- `close_and_unlink_when_idle`: When the list is empty and a flag is clear, it closes two libraries, frees memory, unlinks its own node from the list and returns a stored value.
- `set_flag_bit_return_zero`: Sets the top bit of a flag byte in the structure passed in an address register, then returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_increment_counter`: Clears the top bit of a flag byte in the structure, then increments a 16-bit counter stored at a fixed offset.
- `decrement_counter_and_release`: Decrements a 16-bit counter in the structure.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `collect_font_entries_into_buffer`: Copies font names from a linked list into a caller buffer, filling a count and a fixed-size record for each entry.
- `append_font_entries_to_buffer`: Walks a linked list of font entries and copies each name and its record into a caller buffer while space remains.
- `parse_names_into_font_entries`: Scans a character string one name at a time, recording each name's length and position as an entry while the buffer has room.
- `scan_fonts_directory_entries`: Copies characters of a name into a scratch area and records an entry each time a name ends.
- `scan_dot_font_files`: Walks a list of font entries, checking each name against the text .font and recording matching files it opens through the dos library.
- `copy_font_name_strings`: Copies name characters from a string list into a caller buffer, storing each name's position and size as a record.
- `read_font_names_from_file`: Reads bytes from an open file through the dos library into a buffer, collecting name strings into records as it goes.
- `scan_and_release_font_files`: Walks directory entries through the dos library's examine-next and read calls, records font names into the buffer, and then releases locks and memory through close, unlock, current-directory and free calls.
- `return_zero_value`: Returns zero immediately, with no other work, in the diskfont library.
- `score_record_bit_differences`: Compares two packed records by adding a table-driven correction for each differing bit across seven bit positions.
- `score_byte_bit_differences`: Accumulates a score from seven bit positions where two byte fields differ, subtracting a table value for each.
- `accumulate_bit_table_penalties`: Walks seven bit positions across two byte fields, subtracting a table entry for each position where the bits differ, and returns the running total.
- `accumulate_offset_bit_penalties`: Accumulates table-based penalties for differing bits in two byte fields, shifting the bit mask before each test and stepping through the table at a different stride.
- `return_constant_sentinel`: Returns a fixed sentinel value of negative 32767 and reads no inputs.
- `open_or_load_font_entry`: Tries to open a font through the graphics library and returns the handle if that succeeds.
- `find_and_link_best_font`: Locks the FONTS: directory through the dos library, searches its record table for the best match, and links a new entry into a global list.
- `load_best_font_from_disk`: Opens font files through the dos library, reads their records, selects the best matching record, and links a new entry into a global list.
- `link_best_font_records`: Repeatedly picks the highest-scoring record from a table, allocates a linked entry for it through the library, and adds it to a global list until no candidates remain.
- `load_font_segment_entries`: Scores table records to pick the best candidate, changes into a directory and loads a code segment through the dos library, then links the resulting entry into a global list.
- `release_font_entries_and_segments`: Walks the font record table, clearing each slot and releasing the records it held, and unloads any loaded segments through the dos library.
- `close_font_handles_and_release`: Clears font table slots and closes the files they hold, releasing memory through the exec and dos library stubs as it goes.
- `allocate_record_for_font_name`: Allocates a 260-byte record through the exec library, copies the given name into a small scratch area and searches it for the extension dot.
- `allocate_font_record_table`: Measures a name, allocates a table of 260-byte records sized from the count it finds, and writes a header with a fixed magic value.
- `free_memory_block_with_size`: Frees a memory block whose size is stored in the four bytes before the given pointer, using the exec library free call.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: Copies a null-terminated string byte by byte into the destination, including its terminating zero.
- `append_null_terminated_string`: Moves to the terminating zero of the destination string, then copies the source string onto it.
- `seek_string_end_then_copy`: Scans forward to the terminating zero of a string held in an address register, then passes that position to another routine.
- `measure_null_terminated_length`: Counts the characters before the terminating zero of a string and returns the count.
- `count_string_characters`: Counts the characters of a null-terminated string, keeping the count in a data register, and returns it.
- `compare_strings_ordering`: Compares two null-terminated strings character by character.
- `compare_strings_until_end`: Compares two strings held in address registers character by character, returning zero when the first string ends, negative one when its differing character is smaller, and one otherwise.
- `compare_string_bytes`: Library code that compares two null-terminated strings and returns a non-zero value if a difference is found or if the first string terminates.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_forward`: Scans a null-terminated string forward for a given character and returns a pointer to its first match, or null if it is not present.
- `scan_for_character_match`: Scans forward through a string for the character held in a data register and returns the position of the first match, or null when the string ends.
- `find_next_character_position`: Advances one character at a time from a position in a string until it finds the wanted character, returning that position, or null when the string ends.
- `find_last_character_occurrence`: Finds the length of a string, then searches backwards from its end for the given character, returning the last occurrence or null if none.
- `scan_backwards_for_character`: Searches backwards through a string in an address register for a character held in a data register, stopping at a lower bound and returning null if it is not found.
- `find_last_char_before_limit`: Walks backwards through a string from an address register, stopping at a lower limit, and returns the first position holding the wanted character, or null if the limit is passed.
- `find_name_after_path_separator`: Finds the first slash or colon in a path string and returns the position just after it, which starts the file name part.
- `increment_data_register_value`: Adds one to the value in the data register and returns it, reading no other inputs.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_routine_then_return`: Calls one other routine and returns, with no other work visible in its body.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: The routine calls another routine and then returns without performing any other operations.
- `call_another_routine`: The routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:WORMS

- `run_entry_then_free_memory`: Program entry routine that calls a function pointer, invokes a short countdown routine, and frees a memory block through the exec library.
- `spin_decrement_until_minus_one`: Counts a 16-bit register value down in a tight loop until it reaches negative one, doing no other work.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:HDWORM

- `build_startup_table_entries`: Program entry that seeds a small stack buffer with a constant marker, finds the largest value in a linked list, and fills a table with alternating words and a header.
- `query_two_values_into_globals`: Clears the first byte of two string buffers referenced from a global base, calls a library function at a fixed offset twice, and stores each 16-bit result in a global.
- `call_three_routines_return_zero`: Calls three other routines in sequence and returns zero, reading no other inputs.
- `store_four_results_in_globals_2`: Calls four other routines in turn and stores each returned value into a global slot at a fixed offset from the base register.
- `allocate_memory_return_offset`: Calls the exec allocate-memory function through the library base in a global.
- `free_memory_via_global_base`: Calls the exec free-memory function through the library base stored in a global, releasing the block passed to it.
- `return_zero_value_2`: Returns zero with no other work.
- `retry_open_dos_library`: Repeatedly calls the exec open-library function for dos.library through the library base in a global, looping until the call succeeds.
- `close_library_via_global_base`: Calls the exec close-library function through the library base stored in a global, releasing the library it was given.
- `save_restore_global_d0`: The routine saves a global variable to a local register, calls two other routines, and then restores the global variable to its original value.
- `copy_memory_block_unrolled`: Copies a block of bytes from one address to another, moving eight bytes per pass when the length allows, using an unrolled loop.
- `reset_two_globals_return_pointer_2`: Clears two global words, calls one other routine in between, and returns a code address to its caller.
- `repeat_call_until_condition`: Repeatedly calls one other routine and loops while the condition it leaves behind is not met, then returns.
- `allocate_and_link_node`: Allocates a memory node through the exec library and links it at the head of a global list.
- `unlink_and_free_node`: Searches the global list for the node matching the argument, unlinks it, and frees its memory through the exec library.
- `find_list_tail_node`: Walks a linked list from the given start, following each link until the next link is zero, and stores the final node in a global.
- `preserve_global_around_calls`: Saves a global list pointer, calls two other routines, then restores the saved pointer.
- `preserve_global_around_calls_2`: Saves a global list pointer, calls two other routines, then restores the saved pointer.
- `allocate_and_link_node_2`: Allocates a memory node through the exec library, links it at the head of a global list and returns a pointer to its data area.
- `find_and_free_list_node`: Walks the global list to find the node at the given address, unlinks it, and frees its memory through the exec library.
- `walk_list_checks_counter`: Walks a linked list of entries to its end, then checks a 16-bit global counter and sets it to negative one when it is zero.
- `push_new_list_node`: Allocates a node through the exec library and pushes it onto the front of a global list, clearing its link field.
- `free_entire_node_list`: Removes the head node from a global list and frees it through the exec library, then walks the rest of the chain freeing each node.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.
- `load_font_table_records`: Loads font table records through the dos and exec libraries into a 140-byte scratch area, checks a magic value, and copies blocks of longwords from a table before finishing with library calls.
- `copy_sixteen_longwords_then_call`: Copies sixteen longwords from a table at a fixed offset into the buffer whose pointer is on the stack, then calls a library function through the library base in a register.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_value`: Returns negative one with no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from one memory location to another using a loop and then executes several function pointers stored in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_copy_block`: Opens libraries through the exec library and the library table, saves the bases in globals, and copies a block of longwords with a counting loop.
- `stub_undecompiled_routine`: The decompiler timed out on this routine, so only its facts are available.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_library`: Opens multiple Amiga system libraries and performs a memory block copy from one location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_tables`: Clears two separate memory areas by writing zeros to them and then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_area`: Clears a specific memory area by writing zeros to it and then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_offset_memory`: Clears a memory area starting at an offset calculated from two input values by writing zeros to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_function_pointers`: Executes several function pointers retrieved from a global table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_copy_memory_block`: Clears a specific range of memory with zeros and performs a block memory copy operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_library_call`: Calls a library function using a provided identifier.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_data_stream`: The routine processes a data stream by calling several subroutines to shift bits, perform table lookups, and calculate values before manipulating memory through various pointers and offsets.
- `shift_bit_stream_2`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables_2`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `calculate_table_values`: Performs a loop that calculates values based on a multiplier and an offset, storing them into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_offset`: Increments a value stored in a memory location by a constant amount.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_linear_sequence_table`: Iterates through a loop to populate a table by calculating values using a multiplier and an offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by writing zeros to a range of addresses determined by a pointer and a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_table_entry`: Performs arithmetic on a table entry using registers and checks a condition to return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_2`: Zeroes out a block of memory consisting of sixteen words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_calculated_value`: Stores a value calculated from two registers into a table and checks a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_call_subroutines`: Clears a memory range with zeros and then executes two subsequent subroutines within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_nothing`: This routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_subroutines`: Clears a block of memory by writing zeros to a series of words and then executes two subsequent routines in a loop based on a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: The routine is empty and performs no operations before returning to the caller.
- `copy_data_via_callback`: Executes a provided function pointer repeatedly to copy blocks of data from a source buffer to a destination buffer based on counts stored in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_update_data_tables`: Clears a small memory block and then iterates through a table of pointers to copy data into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_modify_data_blocks`: Iterates through a list of data structures to copy memory blocks using a secondary routine, then performs arithmetic updates on values within the copied data based on a calculated offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_nested_data_blocks`: Iterates through a list of data structures, calls a routine to fetch data, and copies blocks of memory based on counts found in the tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nested_data`: Traverses nested data structures to add a value to specific memory locations indexed by values found in the tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_register_to_memory`: The routine moves a value from a data register into a memory location pointed to by another register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_indexed_values`: Walks through a hierarchical table structure to add a constant value to elements at calculated offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_to_indexed_table`: Performs nested loops to add a value to memory locations determined by indices stored within a table structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_to_indexed_table_short`: Uses nested loops to add a value to memory locations calculated from indices within a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_value_to_pointer`: Adds a value to a memory location pointed to by a register within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_value_to_pointer_minimal`: Adds a value to a memory location pointed to by a register.
- `tiny_move_rts`: Small routine of 2 instructions that copies d3 to d6, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_compressed_data_stream`: Decompresses a bitstream using a variable-length coding scheme and expands the resulting data into a destination buffer.
- `bitstream_decode_value`: Processes a bitstream by shifting and masking bits to reconstruct a value from a sequence of data.
- `mask_table_value`: Returns a bitwise AND result of a value from a table and a provided mask.
- `sum_masked_table_values`: Reads a series of 16-bit values from a table using an index derived from an input word, stores them in a destination, and calculates their sum.
- `initialize_table_values_2`: Iteratively calculates and stores new values into a table using a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_libraries`: A routine consisting of 84 instructions that calls several library functions including FindTask and OpenLibrary.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_system_libraries`: A routine consisting of 29 instructions that calls FindTask and OpenLibrary.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_input_string`: Skips leading spaces in a string and checks if the first non-space character is a newline, question mark, or hyphen before calling a function through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: A 4-instruction routine that calls a function through a pointer stored at a negative offset from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_string_input`: Skips leading spaces in a string and performs checks for specific characters before executing a function via a pointer at a negative offset from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_rts_2`: Small routine of 2 instructions that copies d2 to d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_complex_input`: Parses a string for specific characters, copies data to a local buffer, and executes multiple functions through pointers derived from a value returned by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_string_callbacks`: Iterates through a null-terminated string and executes two functions via pointers located at negative offsets from a register.
- `iterate_string_and_call`: Iterates through a null-terminated string and then executes two functions through pointers at negative offsets from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_routine_segment`: A routine consisting of 25 instructions that was executed during a reference run.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_quoted_string`: Copies a string into a local buffer, handles specific character termination, and then executes two functions through pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_pointer`: Performs an indirect call to a function pointer located at an offset from a base address held in a register.
- `execute_repeated_calls`: Calls a specific routine twice before performing an indirect call to a function pointer located at an offset from a base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_parse_string`: Copies a null-terminated string to a new location and then parses a sequence of characters from a source until a delimiter or specific character is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_subroutines`: Executes a specific routine twice and then performs an indirect call to a function pointer located at an offset from a base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_and_format_string`: Extracts a string from a source buffer, handles optional quotation marks, and reassembles it into a new formatted string.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: The routine performs an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_command_string_and_dispatch`: Parses a command string by handling quoted text and delimiters, then executes a sequence of function calls through a table of pointers.
- `parse_command_string`: Parses a null-terminated string to extract a numeric value following a specific character prefix.
- `traverse_string_and_call`: Traverses a null-terminated string and then executes two functions through pointers at negative offsets from a register.
- `initialize_library_resources`: Opens several libraries, copies a block of data between two memory locations, performs multiple function calls via library vectors, and modifies a bit in a CIAA register.
- `modify_byte_via_pointer`: The routine performs a bitwise OR operation on a byte at a specific offset from a pointer stored in a register.
