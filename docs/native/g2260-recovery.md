# Game 2260 (worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02260). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906920 top-level bytes in 107 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 62 of 62 units identical (bytes and relocations), 88 raw-word lines |
| Functions named and described | pass, 324 of 324 named (0 author symbols), 324 of 324 described (37 by rule, 92 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1272 of 1279 (99.45%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4726 of 4733 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 554051 |
| code | 14340 |
| data | 304644 |
| padding | 11869 |
| unused | 22016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:Run | 2 | 117 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:INSTALL | 3 | 205 | 7 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:OCEAN | 4 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:FLT.1995 | 1 | 233 | 14 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:RunBack | 3 | 163 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:TEAM17 | 4 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:FLOPPYWORM | 4 | 355 | 26 |
| mathtrans.library | 18 | 830 | 97 |
| diskfont.library | 15 | 1040 | 76 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:WORMS | 3 | 27 | 2 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:HDWORM | 4 | 338 | 26 |
| Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:bootblock | 1 | 10 | 1 |
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

- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:TWSTUFF/TW6: CrM2: Crunch-Mania LZH-mode-packed, unpacked with ancient (41486 -> 61440 bytes)
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
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:INSTALL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:OCEAN: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:TEAM17: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- worms (1995)(ocean)(m3)[cr flt](disk 1 of 3)[a]/Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:WORMS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:Run

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_header_then_invoke_callback`: Copies sixteen longwords from a fixed offset of the state structure into the buffer whose pointer is passed on the stack, skipping the copy when that pointer is null.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:INSTALL

- `free_block_and_clear_slot`: Runs an initial sequence of routine calls, including a delay loop through a called routine, then frees a memory block through the exec library and clears a stored slot.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_and_expand_data_tables`: Clears a memory block and populates multiple tables by iteratively calculating and storing values.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:OCEAN

- `release_resources_on_exit`: Repeats the same call sequence as the install entry: several routine calls, a call through a function pointer from the global base, a memory release through exec, a cleared slot, and a returned register pair.
- `decrement_to_minus_one_2`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:FLT.1995

- `poll_until_zero_then_return`: Calls three routines, repeatedly calls another routine until it returns zero, then calls three more routines and returns the value held in a register.
- `render_text_then_await_input`: Opens the system libraries and disables interrupts and sprite DMA, then renders a text string into screen bitplanes by copying glyph rows and handling control codes for spacing and line breaks.
- `wait_until_flag_clear`: Repeatedly calls a routine until the value it leaves in a register becomes zero, then returns.
- `wait_for_zero_result`: Calls a routine repeatedly until the value it leaves in a register is zero, then returns.
- `step_values_toward_targets`: Advances a stepped transition by one unit.
- `return_empty_stub_2`: Performs no operations and returns to the caller immediately.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `return_empty_stub`: The routine returns without any executable logic.
- `restore_custom_chip_state`: Writes saved DMA control, interrupt request and interrupt enable values back to the custom chip registers with the set bit, stores a copper list pointer from a saved structure, and then calls the exec Permit routine to re-enable multitasking.
- `disable_all_custom_interrupts`: Writes the all-bits-clear mask to the DMA control, interrupt enable and interrupt request registers, switching off every DMA channel and interrupt source.
- `return_no_op_stub`: Returns immediately with no effect and makes no register writes.
- `open_graphics_and_init_display`: Forbids multitasking and opens the graphics library, saving three of its words, then enables DMA channels and writes display configuration values into the custom chip registers.
- `wait_for_status_byte_sequence`: Polls a status byte at a fixed offset from the base register, first waiting for it to read as all ones and then waiting for it to read as the letter X. It returns when both conditions hold.
- `set_bitplane_pointers`: Stores four bitplane addresses, spaced at a fixed interval in memory, into a table as high and low halves, then points the copper list register at that table.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:RunBack

- `start_command_in_background`: Parses the command line, including a numeric stack-size option with a minimum value, then starts the requested command in the background with its output sent to the NIL device.
- `report_command_not_found`: Measures a name string, then uses two library output calls to write a 'can't find' message followed by that name.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:TEAM17

- `run_entry_cleanup_sequence`: Runs the entry sequence of the TEAM17 program: several routine calls, a call through a function pointer from the global base, a memory release through exec, a cleared slot, and a returned register pair.
- `spin_delay_loop`: Counts a 16-bit value down from its starting value to minus one as a busy delay, then returns.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:FLOPPYWORM

- `find_max_and_prepare_buffer`: Walks a linked list to find the largest stored value, starting from a floor of 149, then allocates memory and writes an incrementing sequence of words into it.
- `run_three_library_probes`: Performs three steps, each taking a string or data address, clearing a byte through a pointer and storing a 16-bit result from a library call in a global field.
- `empty_stub_routine`: The routine consists of a sequence of instructions that perform no observable operation before returning.
- `store_four_library_results`: Calls four routines in turn and stores each returned longword into a fixed slot of the global data area.
- `allocate_memory_with_bias`: Allocates memory through the exec library and returns the block's address plus a fixed bias, or zero when the allocation fails.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `retry_open_library_until_success`: Repeatedly calls the exec OpenLibrary routine until it returns a non-zero library base, doing nothing else on each failed try.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `forward_single_call`: Calls one routine and returns with no result.
- `copy_block_and_terminate`: Copies a block of the given length from a source pointer to a destination, moving eight bytes per step and then the remainder, and writes a terminating zero byte at the end.
- `init_list_and_return_handler`: Clears two global list fields, calls two routines in between, and returns the address of a handler routine.
- `poll_routine_until_flag`: Repeatedly calls a routine until the processor's zero flag becomes set, then returns.
- `allocate_and_link_node`: Allocates a node through the exec library and links it into a list held in a global slot, setting its size field and returning the address of its data area.
- `unlink_or_allocate_list_node`: Searches a list for the node named by its argument, and when found unlinks it and frees it through the exec library.
- `find_last_list_node`: Walks a singly linked list until it reaches the node whose link is zero, then stores that last node in a global slot.
- `preserve_slot_around_calls`: Saves a global slot, calls two routines, and then restores the slot to its saved value.
- `restore_slot_after_calls`: Keeps a global slot unchanged across two routine calls by saving and restoring it.
- `allocate_node_after_head`: Allocates a block through the exec library, links it after the head node of a list held in a global slot, and records its size field, then returns the address just past its header.
- `unlink_node_then_free`: Searches the list after its head for the node named by the argument and, when found, unlinks it and frees it through the exec library.
- `remove_entry_from_chain`: Walks a chain of entries to find the one matching the second argument, then unlinks it and frees it through the exec library.
- `push_allocated_node`: Allocates a node through the exec library, links it in front of the current list head, and clears its second field.
- `free_all_list_nodes`: Detaches the list head, frees that node through the exec library, then frees each remaining node of the chain in turn.
- `tiny_ori`: Small routine of 1 instruction that merges 5 into d0.
- `find_max_prepare_overlay_buffer`: Finds the largest value in a linked list starting from a floor of 149, allocates memory and fills it with an increasing sequence of words, and stores the maximum and a pointer structure.
- `copy_data_and_call_function`: Copies a certain number of words from a source address to a destination address and then calls another routine.

### mathtrans.library

- `stub_bad_instruction_data`: The body is a single unimplemented instruction marker, so the routine holds no recoverable behaviour.
- `store_argument_in_slot`: Stores its address argument into a global slot in the library data and returns.
- `release_entry_if_unused`: If the entry's usage count is zero, it unlinks the entry from its list, frees it through the exec library and returns a stored value.
- `mark_entry_pending_release`: Sets a flag bit in the entry's status byte so it is released later, and returns zero.
- `acquire_entry_reference`: If the entry is not marked for release, it increments the entry's reference count and, on the first reference, opens the mathffp library and stores its base.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `release_entry_reference`: Decrements the entry's reference count.
- `forward_call_then_return`: Calls one routine and returns.
- `compute_bitwise_square_root`: Computes the square root of a positive input bit by bit, running a 23-step loop that sets one result bit per step where the trial remainder is non-negative.
- `divide_mantissa_with_rounding`: Runs a bit-by-bit restoring division on the mantissa held in registers, adds a final rounding step, and returns the result packed with a small trailing field.
- `call_and_set_status_bits`: Calls a library function at a fixed offset and one routine, then returns the result with one status bit cleared and another set.
- `run_math_operation_sequence`: Makes a fixed sequence of nine library calls alternating among three math routines, leaving the outcome in the library's result registers.
- `compare_values_with_status`: Calls library routines to compare an operand, then branches on the comparison and the sign through a sequence of library calls, and returns the result with some status bits cleared.
- `compare_values_with_branch_status`: Compares an operand through library routines and chooses one of several library call sequences by the comparison and sign, then returns the result with some status bits cleared.
- `select_sign_dependent_operation`: Chooses one of a few library call sequences from the sign and a flag value, performs it, and returns the result with some status bits cleared.
- `dispatch_operation_by_flags`: Chooses between a few library call sequences using a sign test and a zero test on two operand registers, then returns the result with status bits cleared.
- `finish_operation_with_status`: Runs one library call selected by the sign test and the zero test, then returns the result with some status bits cleared.
- `invoke_math_routine_status`: Calls one library function and returns its result with status bits cleared.
- `apply_math_call_pair_status`: Calls the same library function twice, then one routine, and returns the result with some status bits cleared.
- `double_call_then_status`: Calls one library function twice and returns the result of a routine with status bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `compute_bit_serial_quotient`: Checks an exponent-like byte against a range and flags out-of-range inputs, then computes a bit-serial quotient over up to twenty-four steps.
- `accumulate_bit_serial_quotient`: Accumulates quotient bits from a bit-serial loop over up to twenty-four steps on the register value, and calls an error routine when the steps run out before the result is complete.
- `accumulate_bits_with_counter`: Runs a bit-serial accumulation loop driven by a 16-bit counter, adding shifted values until the counter runs out, and then calls an error routine.
- `compute_cordic_trig_rotation`: Reduces an angle argument by multiples of a half-turn constant, then runs a twenty-three-step CORDIC-style rotation that adds or subtracts table angles and shifted values.
- `compute_cordic_trig_checked`: Range-checks an angle argument, reduces it, and runs a CORDIC-style rotation loop with table angles.
- `compute_cordic_trig_short`: Reduces an angle argument by multiples of a half-turn constant and runs a shorter CORDIC-style rotation loop, then calls an output routine.
- `compute_cordic_trig_output`: Reduces an angle argument and runs a CORDIC-style rotation loop when the argument is in range, then calls output routines that depend on a sign and a mode value passed on the stack.
- `compute_cordic_angle_only`: Range-checks an angle argument, reduces it, and runs a CORDIC-style loop that accumulates only the angle component, then calls an output routine.
- `compute_cordic_with_stack_mode`: Reduces an angle argument, runs a twenty-three-step CORDIC-style rotation, and calls output routines chosen by a mode value passed on the stack.
- `compute_cordic_reduced_angle`: Reduces an angle argument by repeatedly adding multiples of a constant, then runs a CORDIC-style rotation and finishes with output routines selected by a mode value on the stack.
- `compute_cordic_rotation_direct`: Runs a CORDIC-style rotation loop directly on the angle argument without range reduction, then finishes with output routines selected by a mode value on the stack.
- `shift_add_multiply_loop`: Runs a shift-and-add loop over a count held in a register, accumulating shifted values from a table, then finishes with output routines selected by a mode value on the stack.
- `return_empty_math_stub`: Returns immediately without any work.
- `forward_call_pass_through`: Calls one routine and returns.
- `call_two_routines_in_turn`: Calls two routines one after the other and returns.
- `normalize_float_mantissa`: Normalizes a value held in a register by shifting it left until its top bit is set, adjusting an exponent word for each shift, with a separate path for negative values.
- `normalize_mantissa_if_needed`: Returns at once when a stored byte passes a range check, otherwise normalizes a register value by shifting it left until its top bit is set and adjusts an exponent word for each shift.
- `call_three_routines_unconditionally`: Calls one routine and then two more in sequence, behind a flag that is always clear, so the later calls always run.
- `call_four_routines_in_sequence`: Calls one routine and then three more, behind a flag that is always clear, so the later calls always run.
- `call_chain_for_nonzero_byte`: When a byte argument is non-zero and incrementing it does not overflow, it calls a routine and then calls two routines with the incremented value and one more.
- `empty_library_routine`: An empty library routine that performs no action.
- `return_no_op_section_stub`: Returns immediately with no work.
- `return_immediately`: This library routine is empty and performs no executable work before returning.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `empty_routine`: The library code consists of four 68000 instructions and performs no observable work, immediately returning to its caller.
- `prepare_float_digits`: Returns at once for a zero value.
- `call_four_setup_routines`: Calls four routines one after another and returns.
- `accumulate_signed_series`: Walks an array for a count held in a register, adding each value to a running total while the total is negative and subtracting it otherwise.
- `accumulate_series_pre_add`: Adds the first array value to a running total, then walks the rest, subtracting values while the total is non-negative and adding them when it is negative, until the count is used up.
- `scale_and_normalize_value`: Returns for a zero value.
- `return_immediately_section_six`: Returns immediately with no work.
- `divide_and_normalize_value`: Runs a bit-serial division loop with an error path on its input unless a flag is set, then normalizes the mantissa by shifting it left up to a fixed limit and calls two routines to finish.
- `bit_serial_divide_normalize`: Runs a bit-serial division loop driven by a 16-bit counter and calls an error routine when the counter runs out, then normalizes the result by shifting it left up to a fixed limit and calls two routines.
- `call_three_setup_routines`: Calls three routines one after another and returns.
- `pack_float_into_stack_word`: Handles the sign and exponent of a value: a negative value takes one path, while a non-zero positive value is shifted into a small stack word, made positive when needed, and finished by library routines.
- `pack_nonnegative_float_word`: For a non-zero value, shifts the mantissa into a small stack word using its exponent, makes the word positive when needed, and calls library routines to finish the pack.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `return_with_no_work`: This library code performs no operations before returning.
- `execute_empty_routine`: This library code contains instructions that perform no observable work before ending immediately.
- `empty_library_routine_2`: The library code contains six instructions and performs no observable work; it ends immediately.
- `return_immediately_2`: This library code is empty and performs no operations before returning.
- `return_from_subroutine_2`: This library routine is empty and performs no operations before returning.
- `return_nothing_section_eight`: Returns immediately with no work.
- `call_pointer_argument_return`: Calls the routine whose address is passed in an address register, then returns the data register value unchanged.
- `call_indirect_return_d0`: This library code calls a function pointer provided in a register and returns the value that was in that register on entry.
- `forward_call_return_value`: Calls one routine and returns the data register value unchanged.
- `forward_math_entry_result`: Calls one internal routine, ignoring its result, and returns the data register value unchanged.
- `return_value_in_d0`: This library code returns the value that was present in a register upon entry.
- `return_value_in_d0_2`: This library code returns the original value held in a register.
- `return_value_in_d0_3`: This library code returns the value that was in a register without modifying it.
- `run_short_rotation`: Library code that calls a short CORDIC-style rotation loop after reducing an angle argument.
- `run_checked_rotation`: Library code that performs a range-checked rotation using a table of angles.
- `store_result_through_pointer`: Calls one routine, then stores the data register value through the address held in a second data register.
- `run_sequential_routines`: Library code that executes a sequence of four routines.
- `run_unconditional_chain`: Library code that executes a sequence of three routines.
- `run_conditional_increment_chain`: Library code that calls a chain of routines if a byte argument is non-zero and won't overflow when incremented.
- `return_zero_float_digits`: Library code that returns immediately if a certain value is zero during floating point digit preparation.
- `return_zero_normalized_value`: Library code that returns immediately if a certain value is zero during scaling and normalization.
- `run_setup_sequence`: Library code that calls three setup routines in sequence.
- `call_indirect_return_d0_2`: The library code calls a function pointer provided in a register and returns the value that was in that register on entry.
- `run_three_library_probes_2`: The library code executes a sequence of instructions that calls another routine and returns a value stored in a register.
- `return_register_value`: This library code calls another routine and returns the value held in a register.
- `call_library_vector_keep_d0`: Calls a routine through a function pointer at a fixed negative offset from a base value kept in a global, then returns the D0 value unchanged.
- `call_library_vector_keep_d0_2`: Calls a routine through a function pointer at a fixed negative offset from a global base value and returns the D0 value unchanged.
- `call_library_vector_return_pair`: Calls a routine through a function pointer at a fixed negative offset from a global base value and returns D0 and D1 together as one 64-bit result.
- `call_library_vector_return_pair_2`: Calls a routine through a function pointer held in a global base table and returns the D0 and D1 results together as a 64-bit value.
- `call_library_vector_keep_d0_3`: Calls a routine through a function pointer at a fixed negative offset from a global base and returns the D0 value unchanged.
- `call_library_vector_keep_d0_4`: Calls a routine through a function pointer at a fixed negative offset from a global base and returns D0 unchanged.
- `call_library_vector_return_pair_3`: Calls a routine through a function pointer in a global base table and returns D0 and D1 together as a 64-bit value.
- `call_library_vector_return_pair_4`: Calls a routine through a function pointer in a global base table and returns D0 and D1 together as a 64-bit value.
- `return_library_pair`: Library code that calls a routine via a global table pointer and returns a 64-bit pair of registers.
- `call_library_vector_return_pair_5`: Calls a routine through a function pointer held in a global base table and returns D0 and D1 together as one 64-bit value.
- `call_vector_offset_keep_d0`: Calls a routine whose address is a global base plus an offset passed in A0, then returns the D0 value unchanged.
- `normalize_fixed_point_value`: Normalizes the signed 32-bit value in D6: it takes the magnitude, swaps the two 16-bit halves when the magnitude is small, then shifts left one bit at a time while counting down until the top bit is set or the count wraps.

### diskfont.library

- `store_argument_in_block`: Calls a library function through the library base; if it returns a non-null block, stores the A0 argument into a fixed field of that block and calls a second library function.
- `store_argument_in_block_2`: Calls a library function through the library base; if it returns a non-null block, stores the A0 argument into a fixed field of that block and calls a second library function.
- `open_libraries_init_list`: Opens two libraries through the exec library base, and if both succeed initializes an empty list header inside the supplied structure and returns that structure.
- `close_library_and_free_block`: Closes a library through a stored library pointer, frees a memory block with a second call through the same library base, and returns zero.
- `release_unused_list_entries`: Walks a linked list and unlinks every entry whose count field is not positive, releasing each one through a library call and decrementing the list's count.
- `free_entry_when_idle`: When the entry's owner flag is clear and its count is not positive, it closes two libraries, frees the entry, unlinks it from its list and returns the value stored in it.
- `set_entry_flag_high_bit`: Sets the top bit of a flag byte at a fixed field of the structure and returns zero, calling nothing else.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Clears the top bit of a flag byte in the structure and increments a 16-bit counter in another field, with no return value.
- `decrement_count_and_finalize`: Decrements a 16-bit counter in the structure and, when it reaches zero with the high flag bit set, calls a library routine through the library base and returns its result, otherwise returning zero.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_font_entries_into_buffer`: Fills a caller-supplied buffer with fixed-size records describing linked-list entries, copying each entry's name and fields and counting the records in the first word, and a second flag bit also queries further entries through library calls.
- `append_list_entries_to_buffer`: Loops over a linked list, copying each entry's name and fields into a caller buffer as a fixed-size record and incrementing the record count, then continues with library queries when a flag bit is set.
- `scan_name_string_into_records`: Walks a character string one byte at a time and, at the end of each name, writes a record for the current list entry into the buffer and advances to the next entry, using the same library query path when a flag bit is set.
- `scan_fonts_directory_entries`: Copies a path string that names the FONTS: directory into a buffer and walks it through the dos library's lock, examine and current-directory calls, allocating memory through exec and storing a record for each entry found.
- `scan_font_files_for_records`: Walks a linked list of directory entries and, for names whose dot extension matches the .font suffix, opens the file, reads its header and appends a record for each entry.
- `parse_font_entry_records`: Parses name strings from the current directory entry and, for each record read from the file, appends a fixed-size record to the output buffer, then closes the file.
- `read_font_entries_from_file`: Copies name bytes into a buffer while using the dos library read call to fill records from the file, appending each parsed entry to the output buffer.
- `scan_directory_entries_with_cleanup`: Iterates over directory entries through the dos scan calls, appending matching entry records to the buffer, and at the end closes, unlocks and frees the scan state, returning a count derived from the last result.
- `return_zero_stub`: A do-nothing stub that returns zero and touches no registers, memory or hardware.
- `compute_position_flag_score`: Computes a signed score from two records: it takes the difference of a 16-bit field in each, scales it by a fixed shift, then subtracts a table value for each of seven bit positions where the two flag bytes differ.
- `accumulate_flag_bit_differences`: Starts from a precomputed value and subtracts a table entry for each of seven bit positions where the two records' flag bytes differ.
- `accumulate_flag_difference_tail`: Continues a seven-step loop over bit positions of two flag bytes, subtracting a table entry for each position where the bits differ, and returns the running total.
- `subtract_flag_terms_from_total`: Runs a seven-step loop over bit positions of two flag bytes, subtracting table entries from a running total wherever a flag bit is set, and returns the total.
- `return_constant_value`: Returns a fixed constant without reading arguments or registers, and touches no memory.
- `lookup_or_open_font_record`: Copies the requested font name and attributes from its argument and looks them up through a library call, returning the match when one exists.
- `lock_fonts_directory_for_scan`: Locks the FONTS: directory through the dos library, then looks up and examines its first entry through the same calls, storing the results in the scan state.
- `examine_font_entries_into_records`: Examines each directory entry in the scan state through the dos calls, reads a small header, allocates a buffer through exec for matching font data, and fills in font records.
- `select_best_scoring_record`: Scores every record in a table of fixed-size font records with a comparison routine and picks the record with the highest score, then processes that record through library calls.
- `add_best_scoring_font_entry`: Finds the highest scoring record in the table, then adds it to the font list under forbid and permit protection, loading its segment and linking it through the dos, exec and graphics calls.
- `unload_font_and_release_state`: Clears a field of the current font record, unloads a segment through the dos call inside a loop, and releases the scan state before returning a value taken from a stored result.
- `close_font_scan_and_release`: Clears a field in the current font record and walks the records, then closes the file and directory handles and frees the scan state through the dos, exec and graphics calls.
- `create_font_record_from_name`: Takes a font file name and its length, allocates a record through exec, copies the name into a path buffer, strips the extension and looks up the matching entry in the FONTS: directory through dos calls.
- `build_font_table_block`: Counts the records in a list, allocates one block sized for those records plus a header through exec, fills in the header and count, then copies each record's entries into it while scanning the directory through dos calls.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
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
- `copy_null_terminated_string`: Copies bytes from a source string to a destination until the zero terminator has been copied.
- `append_null_terminated_string`: Moves to the end of the destination string and passes that end pointer and the source to the string copy routine, which appends the source.
- `advance_to_terminator_and_call`: Advances a pointer until it reaches the zero terminator of a string and then calls another routine with that pointer.
- `measure_string_length`: Walks a zero-terminated string counting its bytes, not including the terminator, and returns the count.
- `count_null_terminated_bytes`: Advances through a zero-terminated byte sequence, adding one to the starting value in D0 for each byte including the terminator, and returns the total.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte, returning minus one if the first is smaller at the first difference, one if it is larger, and zero when they match.
- `compare_strings_in_registers`: Compares the zero-terminated string at one address register with the string at the other, returning zero on a match, minus one when the first is smaller and one when it is larger.
- `compare_strings_in_registers_2`: Compares two zero-terminated strings held in address registers byte by byte, returning minus one, one, or zero on a match.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string`: Scans a zero-terminated string for a given byte and returns a pointer to its first occurrence, or null if it is absent.
- `find_byte_in_string_registers`: Scans forward from a string pointer for a byte held in a register, returning a pointer to the match or null when the zero terminator comes first.
- `find_next_byte_after_pointer`: Steps past the current byte and scans forward for the byte in a register, returning a pointer to the match or null if the terminator is reached first.
- `find_last_byte_in_string`: Measures a string's length and scans backward from its end for a given byte, returning a pointer to its last occurrence or null when none is found.
- `scan_backward_for_byte`: Scans backward from a pointer toward a lower limit for a byte held in a register, returning a pointer to the first match or null if the limit is passed.
- `scan_backward_bounded_for_byte`: Steps backward from a pointer and checks the limit before each comparison, returning a pointer to the byte held in a register or null when the limit is passed.
- `skip_past_path_separator`: Finds the first slash in a path string, or failing that the first colon, and returns a pointer just past it; if neither is present it returns the original pointer.
- `increment_returned_value`: Returns the value passed in D0 plus one, touching nothing else.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `forward_call_without_arguments`: Calls another routine with no arguments and returns, doing nothing else.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns without performing any other operations.
- `call_dispatch_font_request`: This library code acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:WORMS

- `run_program_entry_sequence`: Program entry point for the WORMS file that runs a sequence of calls, calls a routine through a global pointer that the facts identify as a memory release, clears a longword just below a pointer, and returns the D0 and D1 pair.
- `decrement_word_until_minus_one`: Repeatedly decrements a 16-bit value in D6 until it wraps to minus one, then returns.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:HDWORM

- `find_max_and_prepare_buffer_2`: Walks a linked list of values to find the largest stored field, then fills a table with a sequence of values that rise by two per entry, starting from a constant.
- `store_two_library_results`: Clears a flag byte before each of two library calls and stores each returned word into a fixed global slot.
- `call_three_routines_return_zero`: Calls three other routines in sequence and returns zero.
- `store_four_call_results`: Calls four routines in turn and stores each returned value into its own fixed global slot.
- `allocate_memory_with_offset`: Calls the exec AllocMem stub; if the allocation returns a non-zero address it returns that address plus a fixed bias, otherwise zero.
- `call_free_memory_stub`: Calls the exec FreeMem stub through the library base and returns, doing nothing else.
- `return_zero_stub_2`: Returns zero without touching anything else.
- `retry_open_library_until_success_2`: Repeatedly calls the exec OpenLibrary stub through the library base until it returns a non-zero handle, then returns.
- `call_close_library_stub`: Calls the exec CloseLibrary stub through the library base and returns.
- `forward_call_without_arguments_2`: Calls another routine with no arguments and returns, doing nothing else.
- `copy_byte_buffer`: Copies a block of bytes from a source to a destination, moving eight bytes per iteration and then the remaining bytes one at a time.
- `reset_two_globals_and_call`: Clears two global words to zero, calls another routine between them, and returns.
- `repeat_call_until_zero_flag`: Calls another routine repeatedly until the zero condition flag is set, then returns.
- `allocate_and_link_block`: Calls the exec allocation stub through the library base and, if it succeeds, links the new block at the head of a global list and returns a pointer just past its header; on failure it calls another routine.
- `free_or_allocate_list_block`: Searches a global linked list for the block passed in and, when found, unlinks and frees it through the exec free stub; when no block matches and a counter word is zero, it sets the counter and tries to allocate and link a new block.
- `find_list_tail`: Follows a linked list through its next pointers until it reaches the null entry, then stores the address of the last node in a global word.
- `save_call_restore_global`: Saves a global word, calls two other routines, and then restores the global to its saved value.
- `save_call_restore_global_2`: Saves a global word, calls two other routines, and then restores the global to its saved value.
- `allocate_and_link_block_2`: Calls the exec allocation stub through the library base and, if it succeeds, links the new block at the head of a global list and returns a pointer just past its header; on failure it calls another routine.
- `free_or_allocate_list_block_2`: Searches a global linked list for the block passed in and, when found, unlinks and frees it through the exec free stub; when no block matches and a counter word is zero, it sets the counter and tries to allocate and link a new block.
- `free_matching_list_block`: Walks a global linked list to the entry preceding the block passed in, unlinks that block and frees it through the exec free stub; when the list ends without a match it sets a counter word and allocates a new block if the counter was zero.
- `allocate_list_head_block`: Allocates a block through the exec allocation stub and pushes it onto the head of a global list, clearing its second word; on failure it calls another routine.
- `free_all_list_blocks`: Removes the head entry from a global list and frees it, then frees each block in the chain that hangs from that entry, using the exec free stub.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.
- `compute_max_and_fill_table`: Computes the largest value stored in a linked list and writes it into a table entry, then fills the following table words with a sequence that rises by two per entry, making further setup calls.
- `copy_sixteen_words_and_call`: If the destination pointer is non-null, copies a fixed block of words from the base area into it, then calls another routine through the global base.

### Worms (1995)(Ocean)(M3)[cr FLT](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns minus one without touching registers or memory and calls nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_resources`: Retrieves several function pointers from a global structure and performs a large memory block copy from one location to another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_libraries_and_copy_table`: Opens libraries through the exec OldOpenLibrary call and, if the opens succeed, copies a block of words from a fixed source area into memory.
- `examine_unrecovered_routine`: The decompiler could not produce pseudo-code for this three-instruction routine, so its behaviour cannot be recovered from the evidence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_system_data`: Obtains function pointers through a global structure and copies a large block of data to a new destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_call_functions`: Zeros out two separate sixteen-element arrays in memory and then executes a series of function calls, including one that processes bit-shifted values from global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_memory_array`: Executes three function calls and then fills a sixteen-element array in memory with zeros before calling two subsequent routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_zero_buffer`: Fills a sixteen-element memory buffer, located at an offset from two input parameters, with zeros and then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_stored_functions`: Calls four different functions by retrieving their addresses from offsets within a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_copy_memory_block`: Clears a specific range of memory with zeros and performs a block memory copy operation using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: Calls a library function using a specific library index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_data_stream`: The routine performs bit shifting, table lookups, and memory copying operations to manipulate data structures and update values in a destination table.
- `shift_bit_stream_2`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables_2`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `process_data_table`: Iterates through a table to perform arithmetic operations and stores the results in a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unknown_routine`: This routine contains twenty instructions but the code is unavailable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_table_values`: Performs a loop that calculates values based on a multiplier and an offset, storing them into a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory with zeros and checks a value from a pointer before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_table_entry`: Performs arithmetic on a table entry using registers and checks a condition to return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_memory_range`: Fills a specific memory range with zeros and evaluates a short value from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_calculated_value`: Stores a value calculated from two registers into a table and checks a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_call_subroutines`: Clears a memory range and then executes two other routines within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_nothing`: This routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_2`: Clears a block of memory by writing zero to sixteen consecutive words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_3`: This is an empty routine that performs no operations and immediately returns to its caller.
- `process_nested_data_structures`: Iterates through a table of pointers to execute a callback routine and copy data blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_update_data_tables`: Clears a small memory block and then iterates through a table of pointers to copy data into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_modify_data_tables`: Iterates through a series of tables to copy data blocks using a secondary routine, then performs arithmetic additions on specific values within the copied data based on calculated offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_blocks`: Iterates through a list of pointers to copy blocks of data from a source to a destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nested_data`: Traverses nested data structures to add a value to specific memory locations indexed by values found in the tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_memory_block`: Copies a sequence of bytes from one memory location to another using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_indexed_values`: Walks through a hierarchical table structure to add a constant value to elements at calculated offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_table_values`: Performs a nested loop to add a value to memory locations determined by a table of indices.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_to_indexed_memory`: Uses a table to find memory addresses and adds a value to the contents at those locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_offset_loop`: Repeatedly adds a value to a memory location within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_add`: Decrements a counter and then adds a value to a memory location.
- `tiny_move_rts`: Small routine of 2 instructions that copies d3 to d6, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `unpack_and_process_compressed_data`: Decompresses a bitstream using a variable-length encoding scheme and performs arithmetic operations on the resulting data by iterating through a table of values.
- `shift_and_lookup`: Performs bitwise shifts and logical operations on a value while iterating through a table, eventually returning a value from a table lookup.
- `mask_table_entry`: Returns a bitwise AND result of a value from a table and a provided mask.
- `populate_masked_table`: Reads a 16-bit value from a table using an index derived from an input word, masks it, and stores the results into a destination buffer while accumulating a sum.
- `initialize_table_values_2`: Iteratively calculates and stores new values into a table using a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_libraries`: An 84-instruction routine that calls several library functions including FindTask and OpenLibrary.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_task_environment`: A 29-instruction routine that calls FindTask and OpenLibrary.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_input_string`: Skips leading spaces in a string and checks if the first non-space character is a newline, question mark, or hyphen before calling a function via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: A 4-instruction routine that calls a function through a pointer stored at a negative offset from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_command_character`: Skips leading spaces in a string and performs checks on the subsequent character to determine whether to call a function via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_rts_2`: Small routine of 2 instructions that copies d2 to d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_complex_input`: Parses a string for specific characters, copies data to a local buffer, and performs indirect calls based on the results of character comparisons.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_calls`: Iterates through a null-terminated string and performs two indirect calls using offsets from a register.
- `iterate_and_call`: Iterates through a null-terminated string and then performs two indirect calls using offsets from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_subroutine`: A 25-instruction routine that was entered from outside the static flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `format_quoted_string`: Copies a string into a local buffer and appends characters until a specific delimiter is reached, then performs two indirect calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_function`: Performs an indirect call to a function pointer stored at an offset from a base address.
- `execute_repeated_calls`: Calls the same function twice before performing an indirect call to a function pointer at an offset from a base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_parse_string`: Copies a null-terminated string to a new location and then parses a subsequent string based on a delimiter or specific character.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_repeated_function`: Calls a specific function twice and then executes an indirect call to a function pointer at an offset from a base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_quoted_string`: Extracts a string from a source buffer by searching for delimiters and handling optional quotation marks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: The routine performs an infinite loop that never exits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_command_string`: Parses a command string by handling quoted text and whitespace, then executes a sequence of indirect function calls based on the parsed content.
- `parse_command_string`: Parses a null-terminated string to extract a numeric value following a specific character prefix and then executes a series of indirect function calls based on the parsed data.
- `traverse_null_string`: Traverses a null-terminated string and then executes two indirect calls using offsets from a register.
- `initialize_library_resources`: Opens several libraries, performs a bitwise OR operation on a byte via a pointer, and touches a CIAA register.
- `modify_byte_via_pointer`: The routine performs a bitwise OR operation on a byte located at an offset from a pointer held in a register.
