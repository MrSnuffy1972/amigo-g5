# Game 1462 (blob (1993)(core design)[cr flt][t +10 god]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01462). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912544 top-level bytes in 205 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 172 of 172 units identical (bytes and relocations), 214 raw-word lines |
| Functions named and described | pass, 631 of 631 named (0 author symbols), 631 of 631 described (35 by rule, 596 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1133 of 1135 (99.82%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12692 of 12708 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 707789 |
| code | 53626 |
| data | 125550 |
| padding | 8171 |
| unused | 17408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:blob | 1 | 8199 | 263 |
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:faith | 1 | 1044 | 72 |
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:god | 1 | 813 | 30 |
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro | 2 | 267 | 8 |
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 3 |
| Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro#unpacked | 1 | 292 | 19 |

## Cases handled

- 75 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- blob (1993)(core design)[cr flt][t +10 god]/Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 5628 -> 39764 bytes), 8 of 9 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying code at an address (17 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (25 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (25 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (41 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (42 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (24 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (13 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (29 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (64 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target

## Cracktros (kept, not protection)

- Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro
- Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro#unpacked

## Copy protection found

- `reset_hardware_and_copy_data` (runtime segment an address): Saves current DMA, interrupt, and accelerator control registers, enables all interrupts and DMA channels, and updates copper pointers and beam control. Kind: disk hardware/timing check. Pass path: The loop following the POTGOR check and the subsequent data copy loop (puVar5 = puVar4) represents the successful path where hardware state is validated and data is moved into memory..
- `initialize_hardware_and_call_routine` (runtime segment an address (play-lgpwide)): Disables interrupts, writes values to several offsets of a structure in a5, calls another routine, writes more values to the structure in a5, loads a pointer into a4, and writes a byte to the address in a4. Kind: checksum trap / sabotage path. Pass path: The routine appears to be a .
- `initialize_hardware_registers` (runtime segment an address (play-lgpwide)): Writes 32767 to several offsets of a structure in a5, calls another routine, writes 32767 to three more offsets in a5, writes 3 to an offset in a5, loads a value from a table into an offset in a5, and writes 255 to the first byte of a longword at a specific address. Kind: disk hardware/track check. Pass path: not determined.
- `initialize_hardware_and_pointer` (runtime segment an address (play-lgpwide)): Writes specific values to three offsets of a structure in a5, writes a value from a memory location to an offset of that structure, sets three consecutive bytes at a fixed address, and copies a pointer from an offset of a6 to an offset of a5. Kind: disk hardware/CIAB_PRB check. Pass path: not determined.
- `initialize_hardware_and_pointers` (runtime segment an address (play-lgpwide)): Moves a word from a memory location to offset 457 of a structure in a5, writes three bytes to a hardware register, stores the address at offset 128 of a structure in a6 into offsets 128 and 132 of a structure in a5, and loads three different addresses into a0 from offsets 11844, 17408, and 19586 relative to a6. Kind: disk hardware/timing check. Pass path: the code continues to the pointer initialization and data copying loop after the CIA/disk status polling loops finish.

## Functions


### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:blob

- `initialize_memory_and_tables`: The routine obtains a memory block via Exec AllocMem, aborts if the allocation fails, and then sets up several global pointers and constants derived from the allocated block.
- `initialize_game_state`: The routine clears a large block of memory, builds a jump table from a table of offsets, and then writes a series of values to global variables and hardware registers.
- `set_global_and_call_indirect`: The routine writes a constant to a global variable, calls another routine, then calls the routine pointed to by the global variable offset twice in a row, calls another routine again, and returns.
- `reset_and_initialize_state`: The routine clears a set of global variables when a flag is set, loads values from a table indexed by a global index into globals, and calls another routine repeatedly.
- `clear_dma_and_call_handlers`: The routine clears a global variable, invokes another routine, then calls a handler taken from a table twice, invokes another routine again, and returns.
- `cleanup_on_exit`: The routine invokes several function pointers taken from a global table and from another global variable, calling each twice except one which is called once, with no other observable effect.
- `call_indirect_table_entry`: The routine calls a function pointer taken from a global variable table and discards the result.
- `initialize_hardware_and_call_routines`: The routine writes values to custom chip registers, sets a global variable, calls another routine, stores a byte in RAM, and then invokes three routines via offsets from the global variable.
- `invoke_other_routine`: The routine calls another routine and then returns a constant zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_negative_one`: The routine is empty except for returning a constant value.
- `initialize_hardware_interrupts`: The routine sets up a global variable and writes to hardware registers, then calls several routines via a table and configures interrupt and DMA control registers.
- `copy_global_table_to_code`: The routine copies the contents of a global variable table into a code table and then calls the first entry of that code table.
- `empty_routine`: The routine contains no instructions and returns immediately.
- `read_table_byte_to_globals`: The routine reads a byte from a table using an index derived from a global variable, stores the resulting byte into a global variable, and then clears two other global variables and stores a byte from a fixed memory location into a fourth global variable.
- `fill_linear_interpolation_table`: The routine fills a table in a global variable with ten interpolated values between two consecutive short entries.
- `compute_2d_rotation`: The routine reads two short values from a pointer in A3 and two short values from a global table using an index derived from a global variable.
- `decrement_loop_with_recursive_call`: The routine reads a short from a global variable and, if another global variable is non-zero, repeatedly calls another routine while decrementing the first value until it reaches -1.
- `recursive_call`: The routine consists of a single call to another routine and then returns.
- `recursive_call_2`: The routine calls another routine and then returns.
- `self_recursive_call`: The routine calls another routine and then returns.
- `copy_table_block`: The routine copies a block of 16-bit words from a source located at an offset from a global variable to a destination global variable, using a loop that decrements a counter until it reaches -1.
- `copy_table_block_2`: The routine copies a fixed-size block of 9000 words from a source table to a destination table.
- `set_dma_control_word`: The routine updates a set of global variables and repeatedly calls another routine, and writes to the DMACON hardware register to enable DMA channels.
- `reset_global_flag`: The routine clears a global variable to zero and then executes an empty loop whose condition tests another global variable, performing no further operations.
- `set_global_and_recurse`: The routine writes a constant byte value to a global variable and then calls another routine.
- `decrement_memory_block`: The routine decrements each byte in a fixed memory block by one, iterating through 160 bytes.
- `set_map_load_parameters`: The routine sets a set of global variables that control map loading and then repeatedly calls another routine, adjusting a load offset each time the call returns non-zero.
- `initialize_game_state_2`: The routine writes constants to a set of global variables and function-pointer slots, installs a callback pointer, calls several other routines in sequence, and then enters an infinite loop.
- `reset_global_and_recurse`: The routine writes zero to a global variable, then writes five to the same global variable and calls another routine, then returns.
- `conditionally_set_global_and_recurse`: The routine checks a global variable and, if it is greater than -1, stores the constant 5 into another global variable and calls another routine, then returns.
- `set_global_and_call_routine`: The routine stores the constant 5 into a global variable and then calls another routine.
- `initialize_game_state_3`: The routine clears and initializes a set of global variables and function pointers, then calls a routine via an offset from a global variable.
- `set_global_and_call`: The routine stores the constant 5 into a global variable and then calls another routine before returning.
- `toggle_global_counter`: The routine toggles a global flag and decrements a global counter, resetting the counter to ten when it underflows, then sets a global variable to five and calls another routine.
- `decrement_table_entries`: The routine loops seven times, each iteration decrementing a short field located four bytes after a pointer that steps through an eight-byte stride starting from a global variable.
- `copy_table_10000_words`: The routine copies 10001 words from one global table to another global table by repeatedly storing each source word to the destination and advancing both pointers.
- `self_recursive_call_2`: The routine contains five instructions that immediately call another routine and then return.
- `reset_state_machine`: The routine clears a global variable and sets a global pointer to a fixed routine, then repeatedly calls another routine and updates pointers from a table until a sentinel is reached.
- `clamp_copy_animate_table`: The routine clamps a small index variable to the range 0 to 2, wraps it on decrement, and copies 610 items from a source table indexed by that variable to a global destination table.
- `reset_game_state`: The routine writes a fixed set of constants and zero values into a collection of global variables, including pointers to a global variable and a few control flags.
- `fill_lookup_tables`: The routine fills a global variable with repeated division results.
- `initialize_collision_table`: The routine writes a set of short values into a table at a fixed offset, then searches the table for a sentinel value and overwrites the entry and its following fields with values taken from global variables and constants, finally storing a constant into another global variable.
- `update_blob_tile_table`: The routine updates a global counter and writes three 16-bit values from three globals into a table entry indexed by that counter.
- `fill_palette_entries`: The routine loops 40 times and writes three values into each of 40 consecutive entries in a table pointed to by a pointer argument.
- `wrap_positional_coordinates`: The routine updates three short values pointed to by a pointer passed in a register, applying a delta derived from global variables and wrapping each coordinate within a bounded range.
- `set_tile_bitmask`: The routine computes an index from two input values and a table base pointer, then builds a bitmask from the low nibble of one input and ORs it into a 16-bit word at the computed offset in one or two tables depending on a flag derived from another input.
- `swap_two_tables`: The routine swaps the contents of two global tables element by element.
- `copy_memory_block`: The routine copies a block of memory from a source address derived from a global variable plus an offset to a destination global variable, using a counter that starts at 5000 and decrements to -1.
- `configure_dma_wait_loop`: The routine writes to DMACON to enable DMA channels, sets up several control words and a small table of values, then enters a loop that waits for a flag to clear, toggles a byte, updates a counter and decrements a delay counter.
- `reset_game_state_loop`: The routine clears a set of global variables and repeatedly runs a loop that waits on a flag, updates counters, toggles a bit, checks a limit and a condition involving a global flag, then either exits via a callback or continues.
- `update_displayed_name`: The routine repeatedly polls input flags stored in global variables and updates a small set of counters.
- `set_memory_bytes`: The routine writes four constant byte values to consecutive locations of a global variable.
- `other_routine`: The routine sets a global variable to 5 and calls another routine.
- `initialize_display_buffer`: The routine writes two ASCII digits derived from a global counter into two bytes of a global buffer, then sets several global bytes to a space character and writes dot and hyphen characters into a table and a buffer at an offset computed from another global value.
- `reset_game_state_2`: The routine writes a constant value to several global variables and then writes a constant byte into a table indexed by a scaled value.
- `write_two_digit_ascii`: The routine writes two ASCII digits derived from a word in D0 into memory pointed to by A2.
- `wait_for_nonzero_result`: The routine repeatedly calls another routine until it returns a non-zero short value, then returns.
- `call_other_routine`: The routine invokes another routine and then returns.
- `call_other_routine_2`: The routine calls another routine and then returns.
- `decrement_global_loop`: The routine repeatedly calls another routine and updates two global variables in a loop.
- `reset_input_state`: The routine clears several global variables, clamps two globals to zero if negative, sets a global to 5000 and another to 1, then calls another routine repeatedly and enters an infinite loop.
- `set_global_then_call`: The routine stores the constant 5 into a global variable and then calls another routine.
- `update_input_buffer`: The routine maintains a small input buffer and writes incoming byte values into it, handling backspace and termination conditions, then it copies a global value into a table and formats that value as a decimal string into a buffer, and finally it advances a state counter and conditionally returns a constant.
- `check_global_and_recurse`: The routine checks a global variable and, if it is non-zero, calls another routine.
- `poll_until_zero`: The routine repeatedly calls another routine and stores its short result in a local variable.
- `decode_font_glyphs`: Copies 14 glyph bitmaps from a font table into a destination buffer.
- `check_and_set_global`: The routine checks whether a global variable holds a value less than two thresholds.
- `update_three_timers`: The routine updates three independent countdown timers.
- `recursive_self_call`: The routine contains no instructions and performs no operations.
- `copy_block_data`: The routine copies a block of bytes from a source area to a destination area, using a pointer offset derived from an input byte value and a base pointer.
- `set_vhposr_table_entries`: The routine fills a table of word entries with constants and values derived from a global variable, repeatedly calling another routine and updating three short fields in a structure pointed to by the stack using the low and high nibbles of the global variable.
- `format_two_digit_score`: The routine writes a two-digit decimal representation of a global variable into two global character locations, then calls another routine twice with different global control values set between the calls.
- `copy_palette_and_reset_state`: The routine copies a block of bytes from a table into a global buffer, then copies words from that buffer into itself until a sentinel value is reached, and then clears a set of global variables.
- `configure_cia_keyboard`: Writes specific values to the CIAA control and interrupt registers to configure the keyboard interface.
- `invert_global_variable`: The routine reads a global variable, rotates its bits left by seven and right by one, combines the results with a bitwise OR, and then inverts the whole value before writing it back to the same global variable.
- `reset_state_flags`: The routine checks two global variables and, if certain conditions are met, clears a set of global flags and returns.
- `reset_dma_control`: The routine writes to the DMACON register with bit 15 set to enable DMA channels and then clears several global variables used for DMA state.
- `decrement_counters_and_set_flag`: The routine decrements four separate counters stored in memory.
- `process_animation_counters`: The routine updates several global counters and repeatedly calls another routine while stepping those counters toward zero, clamping a counter to a maximum and resetting flags.
- `copy_blob_state`: The routine copies four values from a set of global variables into another set of global variables and computes a fifth value from one of the source globals with an offset, a right shift and a subtraction, storing the result in a global variable.
- `reset_state_and_sync_position`: The routine clears several global variables and, if a condition on a global variable holds, computes a position value from another global, compares it with a stored position, and if the difference is one step updates the stored position and triggers another routine.
- `set_status_flags`: The routine clears a global variable, takes the absolute value of another global variable, and sets the first global to 2 when the absolute value is less than 3 or unconditionally otherwise, also setting a second global to 1 in the negative case and clearing a third global when the value is non-negative and large.
- `find_nearest_object`: The routine scans a table of objects for the current entity and computes a squared distance to each entry, keeping the closest one within a radius.
- `compute_scaled_signed_value`: The routine reads a nibble from a structure pointed to by a pointer, computes the absolute difference between two short values, selects a signed entry from a table using that difference, and then computes a scaled signed value using another global short, a global short multiplier, and the table entry.
- `compute_scaled_signed_value_2`: The routine reads a nibble from a structure pointed to by a pointer, computes the absolute difference between two short inputs, selects a 16-bit entry from a table using that difference as an index, and then computes a signed scaled value from the absolute value of a global short, the table entry, and another global short, storing the result in a global short.
- `scale_global_variable`: The routine checks a global variable and a flag and, if a condition is met, halves a global variable and sets the flag.
- `reset_and_wait_loop`: The routine clears several global variables, sets a few global variables to constant values, calls another routine, then repeatedly waits for a flag to clear, decrements and wraps a counter, calls another routine three times, sets a flag and increments a counter, looping ten times, and then enters an infinite wait loop.
- `initialize_globals_and_spin`: The routine sets a global variable to a constant and calls another routine repeatedly, then waits on a flag, initializes several globals, and enters a countdown loop that decrements a counter, wraps an index, calls another routine repeatedly, sets a flag and decrements the counter.
- `set_global_and_call_routine_2`: The routine stores the constant 5 into a global variable and then calls another routine.
- `process_input_buffer`: The routine resets several global flags and reads a value from another routine, then updates a small state machine driven by a byte buffer in A0.
- `process_event_flags`: The routine examines a byte pointed to by a pointer and, depending on its value, clears the byte, updates several global variables with arithmetic based on nibbles from a word at an offset, and sometimes recurses.
- `update_blob_state`: The routine reads a short from a pointer and a nibble from the pointer, then updates several global variables based on the pointer value and on the absolute values of other globals, computes a scaled offset and stores it, and sets flags.
- `process_blob_state`: The routine writes a constant to a table entry addressed by a pointer and an index, stores the pointer target in a global variable, then updates several global variables based on the absolute values of other globals and a flag derived from the table entry.
- `update_player_state`: The routine reads a state from a table entry and a flag, then adjusts several global variables based on the horizontal offset and velocity components, sets timers and a counter, and updates a sub-state field with clamping.
- `clamp_and_set_flags`: The routine reads a global variable and a short value, computes an absolute scaled value of a global variable, clamps it to a maximum, and writes the result to a global variable while setting or clearing other global flags.
- `copy_font_glyphs`: The routine copies a small fixed number of glyph entries from a source table selected by a global flag and a per-character index into a destination buffer.
- `process_entity_collision`: The routine updates a word field in the entity structure by adding a delta and checks whether the resulting tile index changed.
- `reset_game_state_flags`: The routine checks two global variables and, if both are zero, writes constant values to two other global variables and calls another routine.
- `process_entity_collision_2`: The routine processes an entity pointed to by a pointer in A0, using fields from the entity and several global variables.
- `append_collision_entries`: The routine increments a global counter, calls another routine, then finds the end of a short list terminated by -999 and writes a series of fixed short entries into the list, creating four blocks with mirrored offsets and a terminating -999 pair, and finally sets two global variables.
- `process_object_collision`: The routine checks a global flag and, if clear, reads a packed object descriptor from a pointer.
- `update_object_state_table`: The routine walks a table of objects pointed to by a global pointer, reads a type field from each entry and updates per-object counters and position fields using other routines.
- `copy_short_list_to_previous`: The routine copies a short list starting at an offset from a pointer in a0, moving each entry down by four bytes until a sentinel value is found, then writes the sentinel into the slot preceding the list.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clamp_entity_motion`: The routine updates a small block of word values pointed to by a0, using several global variables as references.
- `update_blob_position_and_velocity`: The routine updates two signed position accumulators by adding velocity components and applies a global direction mask to move them, then damps them toward zero when a flag is set.
- `return_immediately`: The routine contains no instructions and returns immediately.
- `self_recursive_check`: The routine calls itself once at entry and then tests a global variable.
- `copy_tilemap_bytes`: The routine copies bytes from a global table into two global tables using indices derived from a global variable.
- `self_calling_loop`: The routine consists solely of five calls to another routine and then returns.
- `copy_palette_block`: The routine copies a fixed number of entries from a table indexed by a short value in D1 to two destination buffers pointed to by A1 and A2.
- `call_other_routine_if_flag_set`: The routine checks a global variable and, if it is positive, calls another routine.
- `check_global_and_recurse_2`: The routine tests a global variable and, if it is positive, calls another routine, then returns.
- `check_global_and_recurse_3`: The routine checks whether a global variable is greater than zero and, if so, calls another routine, then returns.
- `recursive_call_if_flag_set`: The routine checks a global variable and, if it is positive, calls another routine which may be itself.
- `wait_for_completion_flag`: The routine checks a global variable and, if it is positive, repeatedly calls another routine until that call returns one.
- `self_calling_stub`: The routine calls another routine and then returns.
- `self_recursive_call_3`: The routine consists of a single call to another routine and then returns.
- `wait_for_flag`: The routine repeatedly calls another routine until it returns 1, then returns 0.
- `wait_for_flag_loop`: The routine checks a global variable and, if it is positive, repeatedly calls another routine until that call returns 1.
- `copy_sprite_frame_data`: The routine copies four 16-bit words from a source table into two destination tables at an offset derived from an input index.
- `wait_for_other_routine_return_one`: The routine repeatedly calls another routine until it returns 1, then returns 0.
- `copy_tilemap_rows`: The routine copies a fixed number of 16-bit entries from a source table to two destination tables using an index derived from an input value, advancing the source pointer by a constant stride each iteration.
- `recursive_global_check`: The routine checks whether a global variable is greater than -1.
- `reset_game_state_flags_2`: The routine clears several global variables and sets a flag in two word locations referenced by registers.
- `copy_word_table_entries`: The routine copies a block of word entries from a source table to a destination table, using a counter in a register, and then writes four zero words at the end of the destination.
- `copy_word_pairs_from_table`: The routine copies a series of 16-bit words from a source table into a destination buffer, advancing the source pointer by a fixed stride each iteration and stopping after a counted number of iterations.
- `pack_two_bytes_and_flag`: The routine writes a 16-bit value composed from the low byte of the first input word and the high byte of the second input word to the memory location pointed to by a pointer argument, and stores a 16-bit constant with a bit set according to flags in the inputs to the next word.
- `initialize_game_state_4`: The routine writes four constant values into four consecutive words of a global variable, effectively initializing a small block of state.
- `initialize_level_data`: The routine clears several globals, calls another routine twice, sets up a few global pointers and function pointers, then scans a table at a fixed address to count specific tile types and adjust a counter, clamps a short value, writes a repeating pattern to a global buffer or an alternate buffer, and finally copies and shifts values from RAM into globals before returning.
- `decode_bytes_to_table`: The routine decodes a 32-bit value in a register into four 16-bit words and writes them through a pointer.
- `copy_word_table_until_marker`: The routine copies 16-bit words from one location in a global variable to the next, advancing two pointers in lockstep until the copied word equals -99, then it calls another routine.
- `copy_struct_array_with_termination`: The routine copies a series of eight-word entries from a global variable table to another location in memory, zeroing four words in each entry, and stops when a sentinel value is encountered.
- `empty_routine_2`: The routine is empty.
- `update_table_values`: Checks if a 16-bit value at a global variable is not equal to a specific constant; if it is not, it calculates a new value based on a word from a table, stores it in another table, and writes a constant to the next word in that table.
- `set_level_flags`: The routine prepares two global variables then scans a table of short entries for a match against a masked value derived from a global variable.
- `update_blob_collision_params`: The routine checks a byte from a pointer and several globals and, if the condition holds, computes a signed offset from a global and two table entries, stores scaled values into globals, computes a size limit from a division and stores derived values into globals, saves a pointer in a global and calls another routine.
- `select_and_test_collision`: The routine selects a table entry based on a word from a pointer and several global flags, loads a short and a word from the chosen entry, computes scaled offsets from two signed words in the pointer, checks the resulting coordinates against bounds with fallback calls to another routine, and on success stores a pointer and updates several global flags and variables.
- `set_global_state_and_maybe_call`: The routine sets a global variable to a constant and conditionally changes it based on an unrelated short value, then if another global is zero it computes an absolute difference between two values, stores a derived value into a global variable and conditionally calls another routine.
- `adjust_animation_frame_index`: The routine updates a global animation counter and a frame index based on a signed offset stored in a global variable.
- `check_cheat_conditions`: The routine checks a set of global flags and a value, computes an index into a table of short values, adds an offset and a constant, and tests the result against a range.
- `reset_global_counter`: The routine scans a short array starting at a global variable for a positive entry.
- `set_global_and_recurse_2`: The routine checks a global variable and, if it is greater than -1, stores 5 into another global variable and calls another routine, then returns.
- `update_game_state_flags`: The routine early-exits if a global flag is set and then updates several global counters and timers, clamping values and decrementing them each call.
- `set_state_and_recurse`: The routine checks two global variables and, if conditions are met, writes values to two global variables and calls another routine, then unconditionally writes a value to a global variable and calls another routine again.
- `self_recursive_setter`: The routine writes the constant 5 to a global variable, then calls another routine, which is itself, creating a self-recursive loop.
- `set_global_and_call_routine_3`: The routine stores the value 5 into a global variable and then calls another routine.
- `set_global_and_call_2`: The routine writes the constant 5 to a global variable and then calls another routine.
- `toggle_global_flag`: The routine writes a constant to a global variable and toggles a second global variable by bitwise NOT. If the toggled variable is non-zero, it overwrites the first global variable with a different constant.
- `update_global_counter`: The routine adjusts a signed global variable by a small step and clamps it, then uses the resulting value to modify another global variable with a scaled subtraction when a flag is zero.
- `toggle_and_cycle_counter`: The routine checks a global flag and a global toggle variable.
- `decode_font_glyphs_2`: The routine iterates over a byte buffer, converts each byte to an index by subtracting a base value and applying a special case for a space character, then maps the index to a bit mask for odd indices.
- `update_object_state_and_return_id`: The routine updates a set of global variables that track an object’s state and position, adjusts a signed offset against a limit, and clamps the resulting position.
- `looped_subroutine_calls`: The routine sets up a base address from a global variable plus an offset and a constant value, then repeatedly calls another routine with a decreasing short counter, an accumulator, an unused register, the constant, and the base address incremented by a fixed amount each iteration until the counter reaches zero.
- `self_calling_stub_2`: The routine calls another routine twice and stores the constant 5 into a global variable between the two calls.
- `clamp_and_branch_on_threshold`: The routine computes a signed offset from a global value, clamps it to a range using several conditional adjustments, then checks the adjusted value against two thresholds.
- `check_and_reset_flag`: The routine checks a global variable and, if it is greater than two, sets another global variable to five and calls another routine.
- `reset_global_flag_2`: The routine writes the constant 5 to a global variable, calls another routine, and if a global flag is non-zero it writes the constant 5 to the same global variable again, calls the same another routine, and clears the flag.
- `set_global_flag`: The routine stores a value into a global variable and writes a constant to a location in a table.
- `set_global_pointer`: Stores the current value of a register into a global variable, then sets a 32-bit value at an offset of 842 from a pointer stored in another global variable.
- `set_global_pointer_short`: Stores the current value of a register into a global variable, then sets a 32-bit value at an offset of 420 from a pointer stored in another global variable.
- `scale_and_clamp_values`: The routine computes two scaled values from a global variable and two other globals, stores the results in two global variables, then computes a clamped divisor from a constant divided by a global variable and stores derived offsets in two global variables before calling another routine.
- `initialize_object_table_entry`: The routine walks a short table of entries terminated by -999 starting at a global variable, finds the first free slot, and writes a new entry into it using values from a pointer argument and a global counter.
- `initialize_object_entry`: The routine initializes a global variable and a table entry when a flag is clear, writing several derived values into the table and terminating the entry with sentinel values.
- `spawn_enemy_object`: The routine creates an enemy object entry in a linked list when a spawn condition is met, initializing its position, size, orientation and type fields from the caller’s parameters and a few global variables.
- `process_input_buffer_2`: The routine reads a byte from a pointer and processes a small command set, updating several global variables and invoking other routines based on the command and sign flags.
- `check_level_index`: The routine computes a value from an input word and an unspecified word, then returns early if the input word differs from the computed value by more than one.
- `clamp_short_difference`: The routine computes the difference between the input value in D0 and a global variable offset by a constant, and returns that difference when the input is greater than or equal to the adjusted global value and the difference is non-zero; otherwise it returns the computed difference unchanged.
- `check_signed_sum_range`: The routine receives two short values in registers and returns early if they are not opposites and their sum is greater than -1.
- `check_player_invulnerability`: The routine computes a short value from a global variable and a register value, then returns early if a register input matches or is less than that value and a global flag is set.
- `check_d7_against_d1`: The routine compares a value derived from an input register with a constant offset and returns early if the condition is met.
- `check_division_threshold`: The routine receives a short value in D1 and two unsigned values in D6 and D7.
- `empty_six_instruction_routine`: The routine contains six instructions and performs no observable work; it ends without modifying memory or registers.
- `save_processor_registers`: Stores the contents of all data and address registers into a contiguous block of memory.
- `return_pointer_dereference`: The routine dereferences an address held in a register and returns the 32-bit value found there.
- `save_cpu_registers_to_ram`: The routine stores the incoming register values and the current contents of the other registers into a set of global variables, effectively saving the CPU context to RAM. It then returns without performing any further computation.
- `read_from_indirect_pointer`: The routine returns the contents of a global variable by dereferencing an address held in a global variable.
- `save_registers_to_ram`: The routine stores the incoming registers and the caller-saved registers into a block of RAM, preserving the caller's register state.
- `return_pointer_dereference_2`: The routine returns the value stored at the address held in an uninitialized pointer register.
- `wrap_animation_frame`: The routine updates a modulo 360 counter and a frame counter, clears and masks a flag variable, manages a small state machine, calls other routines conditionally on several flags, indexes a table with a counter, decrements a timer and triggers a short sequence when the timer expires, and returns the entry registers concatenated.
- `initialize_blob_memory`: The routine obtains a pointer from a global variable, aborts if the pointer is null, and then calls another routine twice while setting several global variables and a stack slot.
- `generate_bitmask_from_lfsr`: The routine uses a global variable as a linear feedback shift register seed, repeatedly doubles it, conditionally sets the least significant bit based on the sign of the xor with the seed, and accumulates the resulting bits into a mask by setting one bit per iteration for eight iterations.
- `decode_word_from_table`: The routine reads a 16-bit value from an offset in a table pointed to by a register, applies a bitwise transformation to the value, and returns a byte derived from the transformed bits.
- `scramble_word_bits`: The routine reads a 16-bit value from an offset in a structure pointed to by a register, scrambles its bits with XOR and masking, then returns the low byte of the result.
- `clear_object_pool`: The routine calls another routine and then zeroes a large contiguous block of 32-bit words in memory starting at an offset from a global variable, iterating 19 times.
- `copy_input_blocks_to_globals`: The routine copies five consecutive 32-bit words from a pointer in A0 into five global variables, and copies five consecutive 32-bit words from a pointer in A1 into five other global variables.
- `set_display_window_parameters`: The routine writes values to custom chip registers that control display window start and stop positions and bitplane control, effectively configuring the display window and bitplane modulation.
- `reset_global_state`: The routine repeatedly calls a function pointer taken from a global variable, decrementing the pointer each iteration until it reaches zero, then clears the global variable and writes zero to three other global variables and a constant to a fourth global variable.
- `set_entity_fields`: The routine writes several fields into a structure pointed to by a register, using values from registers and the result of another routine.
- `wait_for_dma_ack`: The routine spins until a bit in a global variable that mirrors a hardware register is cleared, then returns.
- `clear_global_state`: The routine repeatedly calls a function pointer taken from a global variable, decrementing the pointer each iteration until it reaches zero, then clears the global variable and two other global variables.
- `initialize_entity_state`: The routine writes several fields of a structure pointed to by a register, using values returned from another routine and from registers, and masks a word from a table with flags from a register and a global variable.
- `clear_blitter_setup`: The routine calls another routine and stores its two return values into two global variables.
- `clear_global_table`: The routine calls another routine and then clears a block of memory.
- `configure_blitter_copy`: Loads a pointer from a global variable, retrieves a value from that pointer, and calls a routine to configure the blitter.
- `recursive_self_call_2`: The routine calls another routine three times in a row and then returns.
- `configure_blitter_for_copy`: The routine sets up a series of hardware registers used by the Amiga blitter to define a copy operation and then stores the values returned by another routine into a global variable.
- `set_global_and_call_3`: The routine stores the constant 5 into a global variable and then calls another routine before returning.
- `set_flag_and_poll_loop`: The routine sets a global flag to one, calls another routine, stores the result of a second call, then calls several other routines.
- `call_two_routines`: Loads the value 974 into d2, calls another routine, and returns if the result in d0 is zero.
- `wrapper_write_and_close`: Loads two values from global variables and calls two different routines in sequence.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `handle_zero_return`: The routine calls another routine and, if the result is zero, sets a global variable, calls another routine, invokes a function pointer derived from the global variable twice, calls another routine and returns zero.
- `recursive_call_3`: The routine calls another routine and then returns.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer obtained by adding an offset to a global variable.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `reset_global_counter_and_loop`: The routine clears a global variable and a global function pointer, then repeatedly calls the routine pointed to by that pointer twice per iteration while incrementing the pointer.
- `initialize_global_counter`: The routine clears a global variable and a pointer to another routine, then repeatedly calls that routine twice per iteration while incrementing the pointer.
- `clear_global_and_loop_calls`: The routine clears a global variable and then sets another global variable to a small constant.
- `decrement_loop_calling_routine`: The routine clears a global variable and sets it to a small positive value, then repeatedly calls another routine twice per iteration while decrementing the variable until it reaches zero.
- `set_global_and_recurse_3`: The routine writes the constant 2 to a global variable and then calls another routine, which is itself.
- `set_global_and_recurse_4`: The routine writes the constant 1 to a global variable and then calls another routine, which is itself.
- `initialize_memory_table`: The routine writes constant values into a global table and a pointer, then calls another routine twice and clears the routine pointer.
- `fill_table_with_incrementing_values`: The routine writes a sequence of incrementing short values into a block of memory pointed to by a register, storing a zero short after each value.
- `process_data_loop`: Runs a loop eighteen times, where each iteration performs arithmetic on values from a source table and stores the results into a destination table.
- `decrement_global_and_store`: The routine checks a global variable and another global variable.
- `fill_noise_table`: The routine fills a table with a repeating pseudo-random sequence derived from a byte buffer, writing a short seed value and a derived short value per entry, and skips the final entry when a global flag is set.
- `self_calling_stub_3`: The routine consists only of a sequence of calls to itself followed by a return.
- `store_word_pair_to_table`: The routine writes four 16-bit values through a pointer in A0.
- `populate_jump_table`: The routine fills a table in memory with a sequence of short values derived from the addresses of another routine.
- `swap_global_pointers_and_recurse`: The routine swaps the values of two global variables by using a temporary local variable, then calls another routine twice.
- `locate_amiga_signature`: Scans the code segment for a specific two-word magic number sequence.
- `decode_and_shift_buffer`: The routine repeatedly processes a byte stream from a global pointer, copying or shifting bytes into a global buffer while adjusting a counter.
- `search_for_marker_bytes`: The routine scans a global table of short values for the byte sequence that spells the marker "BO" followed by "DY". It advances a pointer through the table, skipping entries until it finds the first marker, then checks the next entry for the second marker, repeating the search.
- `set_dma_handler_and_global`: The routine stores a pointer to another routine in a RAM location used as a DMA handler table entry and writes a constant to a global variable.
- `store_two_words_from_pointer`: The routine reads two 16-bit values from an address pointed to by a register and stores them into two global variables.
- `configure_hardware_registers`: Writes three longwords to specific memory locations, sets two values from a table into the COP1LCH and COP2LCH registers, and updates the INTENA and DMACON registers with bitwise ORed values.
- `initialize_audio_dma_tables`: The routine clears a block of a global variable, then builds a small table of cumulative offsets from a source table and installs a self-referencing handler table.
- `clear_audio_and_dma`: The routine clears four audio volume registers, clears a global variable four times, masks a global variable, writes a constant to a global variable, writes a constant to a custom chip register, and copies a value from a table into a global variable.
- `update_audio_channel_3`: The routine updates the Amiga audio channel 3 hardware registers using values taken from a global variable structure that is populated from a byte stream.
- `parse_script_command`: The routine reads bytes from a script buffer pointed to by a pointer and updates a working structure in memory.
- `parse_input_stream`: The routine reads a byte from a pointer, updates several words in a structure pointed to by another pointer, and uses a table lookup to dispatch to another routine based on a nibble value.
- `process_blob_data`: The routine writes input words into a destination buffer using a source pointer and a table of function pointers, updates several accumulator words and a global variable, and loops over bits of an input word while calling another routine through an indirect jump.
- `call_indirect_through_table`: The routine performs an indirect call through a table pointer.
- `decode_tile_map`: The routine walks a table of short entries using a pointer and a dispatch table, shifts a word in D0 and builds a 32-bit value from A0 and D1.
- `process_input_buffer_3`: The routine walks a pointer-based input buffer using a shift loop and updates several short fields in a block of memory, writing a byte from a pointer into a global variable and touching CIAB_CRA. It then dispatches through a table of function pointers indexed by a nibble derived from the buffer and returns.
- `decode_tile_stream`: The routine decodes a stream of bytes through a pointer, updating several short fields in a structure pointed to by a register and a byte in a global variable.
- `decode_tile_map_row`: The routine walks through a table of tile descriptors using a pointer and a bitmask in a register, updating counters and flags in memory and invoking a handler from a table when a condition is met.
- `decode_tile_map_2`: The routine walks a pointer table using a loop that shifts a word and builds an index from a byte stream, then writes values into several short fields of a structure and updates a global variable and a counter.
- `process_audio_dma_buffer`: Iterates through a buffer of audio DMA data, parsing bytes to update pointer offsets and control flags.
- `decode_sprite_command_stream`: The routine reads a command stream pointed to by a pointer argument and decodes bytes into fields stored in a table referenced by the arguments.
- `decode_tile_map_entry`: The routine decodes a tile map entry using a bitstream in D0 and a pointer table in A0, updating several short fields in a structure pointed to by A2 and A4 and writing values into a global variable and a table.
- `decode_message_block`: The routine decodes a message block by iterating over a table of pointers and updating counters and flags in memory.
- `find_and_call_handler`: The routine walks a linked list of 32-byte entries starting from a global variable, skipping entries whose 8-byte offset field is zero or greater than eight, for up to three steps.
- `traverse_linked_list_and_dispatch`: Iterates through a linked list of objects by following pointers at offset 40, decrementing a counter.
- `decrement_and_dispatch`: The routine subtracts a short from an offset in a structure, clamps the result to a minimum, copies it to a global variable, then iterates over a linked list of structures, decrementing a counter and dispatching through a jump table in another routine.
- `update_and_clamp_position`: The routine adds a short from an offset to a field in a structure, clamps the result to a fixed maximum, copies the field to a global word, then iterates over linked entries.
- `select_handler_and_dispatch`: The routine computes an index from two words in a structure, looks up a byte in a global variable and chooses a handler pointer from a table.
- `copy_sprite_frame_pointer`: The routine reads a short from an object structure, adds a nibble-derived offset and clamps the result, then stores a pointer from a table into a field of another structure.
- `dispatch_handler_via_table`: Copies a value to a local structure, then iterates through a linked list of structures until a valid index is found.
- `update_entity_velocity_and_flags`: The routine updates fields in a structure pointed to by a0, computes a signed offset from a global table using a field and a mask, adds a base value and stores the result, adjusts a direction byte, and conditionally updates a byte and two word fields using an input byte.
- `update_object_position`: The routine computes a signed offset from a global table and a field in the object, adds it to another field, and updates a direction byte.
- `update_blob_object_fields`: The routine updates several fields of a blob object structure using values from registers and a global variable, then if a condition on a byte field holds it modifies a byte counter and clamps a short field.
- `update_entity_timer_and_dispatch`: The routine updates a byte field by adding a value from another field, clamps a short timer field, mirrors it to a global variable, and then iterates over a linked list of entities using a loop counter.
- `dispatch_handler_with_state_copy`: Copies a register value to two different memory locations, then iterates through a linked list of structures until a valid index is found.
- `update_entity_state`: The routine updates fields of a structure pointed to by a pointer in a register, using values from the structure and from registers, and writes some of those values to fields of another structure referenced by a register.
- `update_entity_state_2`: The routine updates fields of a structure pointed to by a pointer, using values from the structure and from two registers, and writes some of those values to a second structure pointed to by another register.
- `update_object_timer_and_dispatch`: The routine updates a short field in the object pointed to by a0 by subtracting an input value, clamps it to a lower bound and copies the result to a global variable; it also updates a byte field with an added value and may set a flag if the byte overflows.
- `set_dma_interrupt_handler`: The routine stores a value and pointers into a global variable table and returns the input word.
- `reset_ciab_icr_table`: The routine writes a series of constant values to a global variable, effectively resetting a table of values used for the CIAB interrupt control register.
- `unknown_routine`: The routine stores seven values from a table into a global variable, then stores a constant into the same global variable, and returns.

### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:faith

- `initialize_game_and_copy_code`: The routine performs early hardware and memory setup for the Amiga, writes values to custom chip registers and disables interrupts, then copies a block of code from one area of memory to another and patches a few words in a loaded module.
- `set_custom_handler_table`: The routine uses a global variable as a key and writes constant values into different word locations of a structure referenced by that variable, effectively configuring entries in a custom handler table.
- `handle_input_and_update_state`: Polls joystick fire buttons and updates global state variables based on internal counters.
- `poll_joystick_fire_buttons`: The routine reads the joystick/mouse fire buttons and disk status from the CIAA port register, compares the current value with the previous one stored in a global variable, and updates a set of global flags and a table entry accordingly.
- `clear_global_flags`: The routine clears a global variable to zero and stores a constant word value into a location addressed via an uninitialized pointer.
- `toggle_state_and_set_timer`: The routine toggles a global variable and configures two global variables with a value derived from a selector global, then calls another routine; for a different selector value it writes a constant to a global variable.
- `initialize_graphics_library`: The routine calls a function pointer taken from a global table and stores a value from the returned pointer into a global variable.
- `update_mouse_position`: The routine reads the mouse movement from JOY0DAT and updates a pair of position globals by a fixed amount each time a counter reaches a threshold, incrementing or decrementing a step counter accordingly.
- `reset_fade_timer`: The routine increments a counter stored in a global variable and resets it when it matches another global value, clearing a second global variable as well.
- `update_mouse_position_and_flags`: Increments a counter and compares it to a threshold to reset flags.
- `load_character_data_from_table`: Iterates through a string to load character data from a global table into object structures.
- `set_global_table_entries`: The routine writes four 16-bit values to a global variable, effectively initializing a small table of entries.
- `increment_counter_and_load_table`: The routine increments a global counter and, on a specific count, selects a table entry based on a global pointer and copies several bytes from a table into global variables, then advances the pointer.
- `initialize_object_with_table_data`: Loads character data from a global table into an object structure based on an input value.
- `reset_game_state_3`: The routine clears a block of global variables, writing zero to most of them and setting a few to sentinel values.
- `increment_color_nibbles`: The routine walks two tables in parallel, comparing each byte from the first table with the corresponding 12-bit value in the second table.
- `decode_color_table`: The routine decodes a packed color table by comparing nibbles from a source byte stream with a destination word table and conditionally subtracting offsets before writing the adjusted word back.
- `self_invoke_other_routine`: The routine consists of a repeated sequence that calls another routine.
- `self_calling_loop_2`: The routine repeatedly calls itself nine times in a row and then returns.
- `toggle_unlimited_lives`: The routine toggles a global flag, clears two global bytes, calls another routine, and writes a byte to memory via an uninitialized pointer.
- `clear_unlimited_lives_flag`: The routine clears a global variable and sets another global variable, then calls another routine.
- `toggle_unlimited_energy_flag`: The routine flips a global variable, clears two global bytes, calls another routine, and writes a constant byte to memory via an address in A5.
- `set_unlimited_energy_flag`: The routine writes a constant to two global variables and then calls another routine.
- `toggle_unlimited_time`: The routine toggles a global flag, clears two global bytes, calls another routine, and writes a constant to a memory location pointed to by an uninitialized pointer.
- `set_unlimited_time_flag`: The routine clears a global variable, sets another global variable to a constant, calls another routine, and stores a constant into a byte at an offset from a pointer.
- `toggle_double_points_flag`: The routine toggles a global flag, clears two global byte variables, calls another routine, and stores a constant byte through an address in A5.
- `reset_double_points_flag`: The routine clears a global variable and sets another global variable, then calls another routine and writes a constant to memory through a pointer.
- `toggle_and_branch_state`: Inverts bit 0 of a byte at a global variable, compares it to 1, and branches to another routine if it is not 1; otherwise, it loads an address into a3 and branches to another routine.
- `check_jumps_state`: Compares a byte at a global variable to zero and branches if it is not zero; otherwise, it loads two addresses into registers, sets two global variables, and calls another routine before branching to another routine.
- `toggle_smartbomb_state`: Inverts the least significant bit of a byte at a global address, compares it to one, and if not equal, branches to another routine; otherwise it loads a new address into a3 and branches to another routine.
- `check_smartbombs_state`: Compares a byte at a global variable to zero and branches if it is not zero; otherwise, it loads two addresses into registers, sets two global variables, and calls another routine before branching to another routine.
- `toggle_firepower_state`: Inverts the least significant bit of a byte at a global address, compares it to one, and if not equal, branches to another routine; otherwise it loads a new address into a3 and branches to another routine.
- `initialize_unlimited_firepower_state`: Checks a global byte for zero, then loads a table address into a register and updates two global longwords with zero and ninety-four before calling a routine and branching to another routine.
- `toggle_and_check_flag`: Toggles the least significant bit of a byte in a global variable and branches to another routine if the resulting value is not one; otherwise, it loads a pointer into a3 and branches to another routine.
- `initialize_character_data`: Compares a byte at a global address to zero, and if it is not zero, it branches; otherwise it loads a pointer into a3, sets d7 to zero and two, loads a table address into a2, writes zero and one hundred and two to two global variables, and calls load_character_data_from_table.
- `increment_character_index`: Increments a long value at a global address by four and a byte at another global address by one; if the long value reaches thirty-six, it resets the long value to zero and the byte to one.
- `increment_character_index_alt`: Increments a long value at a global address by four and a byte at another global address by one; if the long value reaches twenty-eight, it resets the long value to zero and the byte to one.
- `toggle_and_check_collision_byte`: Inverts the least significant bit of a global byte, compares it to one, and if they are not equal, branches to another routine; otherwise, it loads a pointer from a table into a3 and branches to another routine.
- `check_collision_state`: Compares a byte at a global address to zero, and if it is not zero, it branches; otherwise it loads a pointer into a3, sets d7 to zero and forty, loads a table address into a2, writes zero and one hundred and twenty-six to two global variables, and calls load_character_data_from_table.
- `clear_global_flag`: The routine writes a constant value to a global variable and stores a constant word into a memory location addressed by an uninitialized pointer.
- `reset_audio_and_input_state`: The routine clears a table of pointers derived from a global variable and then writes zero to a series of global variables and to a hardware register, effectively resetting audio and input state.
- `mute_audio_channels`: The routine writes zero to four audio volume registers and then writes a value to DMACON with bit 15 set, which enables DMA channels.
- `reset_game_state_4`: The routine clears a global flag and repeatedly calls another routine, then performs two busy-wait delays and copies several words from a fixed table into RAM locations.
- `reset_audio_channels`: The routine calls another routine four times and then clears a global variable in a loop that increments it modulo 128, stopping when the variable becomes zero.
- `set_output_from_table`: The routine selects a 16-bit value from a table indexed by a global variable and a byte taken from an input structure, with a fallback to a direct field when the global variable modulo three is zero, and writes the chosen value to an output structure.
- `decode_table_entry`: The routine walks a table of shorts starting at a global variable and compares each entry against a limit from another routine’s context.
- `copy_word_from_table`: The routine copies a 16-bit word from an offset in a table pointed to by a register to an offset in a structure pointed to by another register.
- `lookup_and_copy_word`: The routine walks a short table starting at a global variable, comparing each entry with a value from an argument register.
- `write_registered_value`: This routine stores a 16-bit value from a register into a global variable offset from a pointer, and returns.
- `init_audio_channels`: The routine clears a global flag, calls another routine four times, then performs two short busy-wait loops.
- `decode_tile_attribute`: The routine reads a packed tile descriptor from memory pointed to by a pointer, extracts a nibble to index a table of pointers and a table of 30-byte entries, and writes selected fields back into the descriptor and an output structure.
- `process_input_buffer_4`: The routine reads fields from a pointer-based input structure and writes derived values into another structure via a pointer.
- `set_dma_control`: The routine writes to DMACON, masks channel bits, copies values from a table entry into a DMA structure and updates a global variable.
- `disable_audio_dma_channels`: The routine disables audio DMA by writing to DMACON with bit 15 set to enable the write, then clears the audio channel control and length registers for all four channels.
- `set_timer_and_check_flag`: The routine reads a word from a pointer, masks it, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `clear_flag_at_offset`: The routine stores zero into a 16-bit field located at a fixed offset from an address in a register.
- `process_timer_delta`: The routine updates a short counter stored in a structure pointed to by a register using a byte delta stored in the same structure.
- `decrement_and_clamp_value`: The routine subtracts a short value from a short field in a structure pointed to by an argument, then clamps the field to a minimum value stored in another field of the same structure.
- `apply_table_offset_to_signed_value`: The routine copies a byte from an offset in a structure to another byte in the same structure, then uses a byte from the structure as an index into a global variable table to form a scaled offset, adds a signed short from the structure, stores the result as a signed word in another structure, and updates a byte in the first structure with a transformed nibble.
- `adjust_score_and_flags`: Subtracts a 16-bit value from another and stores the result in a memory location.
- `copy_word_from_table_2`: The routine copies a 16-bit value from an offset in a table addressed by a register to an offset in a structure addressed by another register.
- `process_audio_channel_state`: Updates an audio channel structure by modifying its timer, volume, or pitch based on a mode flag.
- `update_entity_timer_from_nibble`: The routine reads a nibble from a byte at an offset in a structure pointed to by an argument, adds the high nibble to a short timer field in the same structure and clamps it to a maximum, or subtracts the low nibble and clamps it to zero, then copies the resulting timer value to a short field in another structure pointed to by a second argument.
- `decrement_and_clamp_value_2`: The routine reads a 16-bit value from an offset in a structure pointed to by an argument, subtracts the low nibble of a byte from another offset in the same structure, writes the result back, clamps it to zero if negative, and copies the resulting 16-bit value to an offset in a second structure pointed to by another argument.
- `decrement_and_clamp_angle`: The routine reads a byte from an object structure, subtracts it from a 16-bit field in the same structure, and then clamps the low 12 bits of that field to a minimum value.
- `update_timer_offset`: The routine adds a byte value from an object to a short field in the same object, then clamps the low 12 bits of that field to a minimum value by clearing the low bits and OR-ing the minimum if the value is too low.
- `decode_message_byte`: The routine reads a nibble from a byte pointed to by a pointer and, based on its value, updates a global variable or a byte in memory.
- `update_global_from_pointer`: The routine reads a byte from a memory location three bytes after an uninitialized pointer, tests bit 0 of that byte, and updates a global variable by setting bit 1 according to the test result while preserving the other bits of the global variable.
- `toggle_global_variable`: The routine inverts the value of a global variable by applying a bitwise NOT and then returns.
- `invert_byte_from_pointer`: The routine reads a byte from an address computed from an uninitialized pointer plus a small offset, subtracts one from it, then stores the bitwise NOT of the result into a global variable.
- `clamp_byte_at_offset`: The routine reads a byte from offset three of a pointer in a register, clamps it to a minimum value by writing a constant if it is lower, then copies the resulting byte to offset eight of a pointer in another register.
- `clamp_byte_and_copy`: The routine clamps a byte at an offset from an uninitialized pointer to a maximum value and, if the byte is non-zero, copies it to a global variable.

### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:god

- `initialize_trap_and_wait`: The routine allocates memory, opens the DOS and graphics libraries, sets up the custom chip registers for a copper list and disables then enables interrupts and DMA, and then enters a loop that polls the vertical beam position and joystick test registers.
- `patch_jump_table_entry`: The routine writes a constant word to a table entry and stores the address of another routine in the following table entry, then calls through the table.
- `unknown_routine_2`: The routine contains a single indirect call through a pointer loaded from a table.
- `lookup_byte_from_table`: The routine builds a mask from the input byte and walks a byte table starting at a global variable until it finds a matching entry or a sentinel.
- `call_indirect_via_table_2`: The routine reads a short value from a pointer in A1 and uses it as an index into a table of routine addresses, then calls the routine found at that entry.
- `xor_word_at_offset`: The routine receives a pointer in a1 and xors a word at a fixed offset from that pointer with a constant word.
- `xor_memory_values`: Performs an XOR operation on 16-bit words at offsets 11410 and 17918 of a structure in a1, then branches to another routine.
- `xor_memory_value`: Performs an XOR operation on a 16-bit word at offset 2650 of a structure in a1, then branches to another routine.
- `set_memory_word`: Stores the word value 6 at offset 8362 of a structure in a2, then branches to another routine.
- `set_memory_word_offset_20b0`: Stores the word value 96 at offset 8368 of a structure in a2, then branches to another routine.
- `set_memory_word_offset_20a8`: Stores the word value 96 at offset 8360 of a structure in a2, then branches to another routine.
- `set_memory_word_offset_20a6`: Stores the word value 1 at offset 8358 of a structure in a2, then branches to another routine.
- `write_global_word_and_return_registers`: The routine stores a word to a global variable and returns the concatenation of two registers.
- `clear_array_to_zero`: The routine clears a block of 32-bit words to zero.
- `wait_for_table_entry`: The routine calls another routine, stores its short result, then spins until the low 9 bits of a byte taken from a global variable via an indirect pointer equal the stored value plus one.
- `wait_for_word_match`: The routine loops until the low word of a global variable, masked to nine bits, equals a word value in D0.
- `iterate_table_and_advance_pointer`: The routine walks a byte table terminated by -1, calling another routine for each entry and updating a pointer from an output value.
- `copy_sprite_pattern_bytes`: The routine copies a series of bytes from a table pointed to by a global variable offset by a byte in D0, using a stride taken from another global variable, into a destination buffer pointed to by A1 at fixed offsets.
- `count_string_bytes_and_store_half`: The routine counts the number of bytes in a zero-terminated string pointed to by a pointer, stopping at the byte with value -1, and stores the negated half-length minus a constant into a 16-bit field at an offset from an address in A5.
- `adjust_animation_frame_index_2`: The routine adjusts a 16-bit frame index stored in a structure using a signed direction byte from another structure.
- `decrement_animation_index_backward`: Iterates backward through a character table starting from a stored index until a non-null entry is found.
- `increment_animation_index_forward`: Iterates forward through a character table starting from a stored index until a non-null entry is found.
- `decrement_counter_and_wrap_value`: The routine decrements a byte at an offset from a pointer in A5 and, if it reaches zero, resets it to one and updates a short counter at another offset, clamping it between zero and fifteen.
- `decode_command_stream`: The routine reads a byte stream from a global pointer, converts each byte by subtracting a constant, writes the result to a global output buffer and, depending on the value, writes a following byte to another global buffer or calls another routine.
- `process_input_event`: The routine reads a byte from a table pointed to by a structure field using an index from the structure, then updates several bytes in the structure, toggles a byte in another table, and conditionally sets a counter and pointer fields.
- `format_number_string`: The routine iterates over a byte string pointed to by a global variable and, for each entry marked with a specific marker, formats the following byte as a three-digit decimal string into a global buffer.
- `parse_string_table`: The routine walks a global string table and copies each character into a global buffer, subtracting a constant from each byte.
- `compare_and_adjust_tile_attributes`: A routine that decrements a global counter, loads a global value into another global location, then walks two parallel tables of 16-bit entries until it finds a sentinel.
- `increment_nested_values`: Decrements a word at offset 796 of a structure in a5, and if it reaches zero, iterates through a list pointed to by a0, incrementing specific bitfields within 16-bit words at the addresses contained in the list.
- `wait_for_byte_values`: Enters a loop checking a byte at offset 6 of a structure in a6 for the value 254, then enters a second loop checking for the value 255 before returning.

### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro

- `process_hunk_headers`: The routine walks a table of AmigaDOS hunk headers and rewrites the relocation entries in place.
- `decrement_until_zero`: The routine receives a 16-bit value in D1 and repeatedly subtracts one from it until it reaches zero, effectively counting down the value to zero.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `shift_and_fill_bytes`: The routine processes a byte buffer referenced by a pointer in A3 and a length in A0.
- `check_shifted_flag`: The routine receives two values in registers and tests whether a shift of the second value by a masked amount of the first value yields a non-zero result.
- `extract_bits_from_stream`: The routine extracts a sequence of bits from a bitstream held in a global variable and a table pointed to by a pointer.
- `extract_bits_to_word`: The routine extracts bits from a stream of bytes pointed to by a pointer, using a bit buffer and a bit count held in registers, and assembles them into a 32-bit word in a register.
- `unreachable_trap_stub`: The routine contains an illegal instruction that triggers a trap, causing the CPU to enter an exception.

### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.
- `modify_d0_register`: Copies the byte from d0 to d2 and performs three logical operations on d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `supervisor_call_wrapper`: Saves all registers, calls two setup routines, loads a5 with a program base, invokes exec Supervisor via the library vector at offset minus thirty, then calls two more routines and returns zero in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `supervisor_call_wrapper_2`: Saves all registers, calls two setup routines, loads a5 with a program base, invokes exec Supervisor via the library vector at offset minus thirty, then calls two more routines and returns zero in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_hardware_and_copy_data`: Saves current DMA, interrupt, and accelerator control registers, enables all interrupts and DMA channels, and updates copper pointers and beam control.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers_2`: Copies three words from custom chip registers to a table, sets the status register, writes thirty-two thousand seven hundred and sixty-seven to three custom chip registers, calls another routine, and writes thirty-two thousand seven hundred and sixty-seven to two more custom chip registers and three to a final custom chip register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_registers_3`: Writes values to several memory locations, sets bit 1 of the CIAA_PRA register, and copies the values of the CUSTOM_DMACONR, CUSTOM_INTENAR, and CUSTOM_ADKCONR registers to offsets 4be6, 4be4, and 4be8 of a6 respectively, then sets CUSTOM_INTENA to 32767.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_and_call_subroutine`: Writes two 16-bit values from offsets 28 and 16 of a structure in a5 to offsets 19688 and 19688 of a structure in a6, disables interrupts, writes 32767 to offsets 154, 150, and 158 of the structure in a5, calls another routine, writes 32767 to offsets 150 and 156 of the structure in a5, and writes 3 to offset 384 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_and_set_ciaa_bit`: It writes four longwords to a table, calls four routines, writes a word to a global variable, and sets bit 1 of the CIAA PRA register before calling four more routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_call_routine`: Disables interrupts, writes values to several offsets of a structure in a5, calls another routine, writes more values to the structure in a5, loads a pointer into a4, and writes a byte to the address in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointers_and_call_subroutines`: Stores two addresses into a structure at offsets 4bf8 and 4bf4 from a6, sets bit 1 of a hardware register, and calls four different subroutines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_registers`: Writes 32767 to several offsets of a structure in a5, calls another routine, writes 32767 to three more offsets in a5, writes 3 to an offset in a5, loads a value from a table into an offset in a5, and writes 255 to the first byte of a longword at a specific address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_ciaa_port_a`: Reads the current state of the CIAA Port A data register and stores the value into a memory location pointed to by a register, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_pointer`: Writes specific values to three offsets of a structure in a5, writes a value from a memory location to an offset of that structure, sets three consecutive bytes at a fixed address, and copies a pointer from an offset of a6 to an offset of a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_ciaa_pra_status`: Reads the value of the CIAA Port A Data Register and stores it into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_pointers`: Moves a word from a memory location to offset 457 of a structure in a5, writes three bytes to a hardware register, stores the address at offset 128 of a structure in a6 into offsets 128 and 132 of a structure in a5, and loads three different addresses into a0 from offsets 11844, 17408, and 19586 relative to a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_2`: Reads a 16-bit coordinate from a table using an index in a register, adds a velocity value to it, and writes the result back to the same table location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_3`: Reads a 16-bit coordinate from a table using an index in a register, adds it to a base address, and writes the result to a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_4`: Reads a 16-bit coordinate from a table using an index in a register and writes the resulting value to a memory location pointed to by another register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_parameters`: Writes two bytes to a memory location pointed to by a register, updates four pointers in a structure using offsets from a base register, calls another routine, and modifies two values in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_attribute_table`: Iterates through a table of sprite data, reading a byte from a memory location and writing it to a specific offset within a sprite attribute structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_fire_status`: Reads the status of the joystick or mouse fire buttons from the CIAA port A data register and stores the result in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_5`: Reads a 16-bit coordinate from a table using an index in a register and writes the resulting value to a specific memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_fire_status_2`: Reads the status of the joystick and mouse fire buttons from the CIAA Port A register and stores the value in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_6`: Reads a 16-bit coordinate from a table using an index in a register and writes the resulting value to a memory location used for sprite positioning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_trigger_subroutines`: Copies the contents of register a0 into two consecutive longwords at offsets 128 and 132 of a structure, updates two pointers at offsets 19432 and 19618 of a structure, calls three different subroutines, and sets bit 1 of a bitplane in the hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position_7`: Reads a 16-bit coordinate from a table using an index in d0, adds a velocity value from a second table, and writes the result back to the coordinate table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_and_hardware_state`: Stores a pointer from a register into a global variable, updates two pointers in a structure pointed to by a6, calls another routine, moves a value from a global variable into a register, writes a constant to a global variable, sets bit 1 of a bitmask in memory, and calls two subsequent routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_runtime_segment_data`: The provided pseudo-code is heavily corrupted by overlapping instructions and unreachable blocks, making the actual logic of the routine impossible to reconstruct from the decompilation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_fire_status_3`: Reads the status of the joystick and mouse fire buttons from the CIAA Port A register and stores the result in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_and_copy_data`: Polls the CIAA Port A Data register and enters a loop waiting for a specific bit in a structure member to change.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_play_lgpwide_state`: The routine performs a series of conditional checks and branches based on data in memory, though the provided disassembly is heavily corrupted by overlapping instructions and unreachable blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_ciaa_pra_status_2`: Reads the value of the CIAA Port A Data Register into a register and stores it into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_play_lgpwide_state_2`: The routine performs a series of conditional branches and jumps based on data in memory, though the provided code segment is heavily corrupted by overlapping instructions and unreachable blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_joystick_fire_input`: Checks if the fifth bit of the joystick fire button register is set, then modifies a value by setting its fifth bit and stores it into two different global variables before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_wide_sprite_position`: Updates the horizontal and vertical coordinates of a wide sprite by reading values from a table and writing them to the sprite's position fields in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_joystick_fire_status`: Reads the status of the joystick or mouse fire buttons from the CIAA Port A Data register and stores the result into two different memory locations using offsets 119 and 19188 from two different pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_ciaa_pra_bit_six`: Polls the sixth bit of the CIAA Port A register in a loop until it is cleared, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_joystick_fire_and_reset`: Polls the CIAA port until bit 6 is cleared, then calls two subroutines and sets several 16-bit values to 32767 within a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_and_copy_data`: Checks a bit in a structure, and if not set, updates two memory locations with a modified value from a global variable and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_and_poll_ciaa`: Enters a loop that XORs a value with 32, stores it in two different memory locations, and calls another routine; the loop continues until a bit is set in a structure at offset 22, at which point it checks a CIAA port and performs bitwise OR operations on a memory location via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_fire_and_update_state`: Enters a loop that updates two memory locations with a value derived from a XOR operation on a global variable, calls another routine, and waits for a specific bit in a structure to clear; it then checks the CIAA Port A Data Register and, if bit 6 is set, performs bitwise OR operations on a byte at an offset in a structure before exiting.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_structure_and_poll_input`: Sets several 16-bit fields in a structure to 32767, copies three bytes from a memory location to a global variable if a specific value is not present, and enters a loop that updates two memory locations with a bit-flipped value and calls another routine until a bit in a CIAA register is cleared.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_and_update_state`: Polls the CIAA Port A Data register to check for a button press, then updates two memory locations with a value derived from a XOR operation and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_structure_and_poll_ciaa`: Sets several 16-bit values to 32767 within a structure, copies three bytes from a memory location to a global variable if a specific 16-bit value is not 19985, and enters a loop that updates two memory locations with a XORed value and polls a CIAA register bit until it is cleared.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_joystick_fire_and_update_state_2`: Polls the CIAA Port A register to check the status of bit 6 and enters a loop that updates two 16-bit values at specific offsets in two different structures with a value derived from a global variable XORed with 32.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_hardware_flag_and_update_memory`: Polls a bit in a memory location via a pointer in a register and checks a bit in the CIAA Port A register in a loop, then calls two other routines and writes a 16-bit value of 32767 to an offset of 154 in a memory structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_ciaa_and_flag`: Repeatedly reads the CIAA_PRA register and spins until bit 10 of a 16-bit word at offset 22 of a structure in a5 is set, then breaks the loop if bit 6 of the CIAA_PRA register is clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_from_pointers`: Loads two longwords from a table into offsets 80 and 84 of a structure in a5, reads a 16-bit word from offset 136 of that structure into d0, sets bit 15 in three 16-bit words at offsets 4be6, 4be4, and 4be8 of a structure in a6, and finally stores those three words into offsets 150, 154, and 158 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_bits`: Sets bit 15 in three 16-bit words at offsets 4be6, 4be4, and 4be8 of a structure in a6, then copies these values to offsets 150, 154, and 158 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_and_set_status`: Stores a longword into offset 132 of a structure in a5, reads a 16-bit word from offset 136 of that structure, sets bit 15 in three 16-bit words at offsets 4be6, 4be4, and 4be8 of a structure in a6, copies them to offsets 150, 154, and 158 of the structure in a5, and returns via an exception return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_set_offset`: Calls two other routines and then writes 32767 to offset 154 of a structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_via_exception`: Sets bit 15 in two 16-bit words at offsets 4be4 and 4be8 of a structure in a6, then copies the values from offsets 4be6, 4be4, and 4be8 of that structure into offsets 150, 154, and 158 of a structure in a5 before returning via an exception return.
- `wait_for_flag_and_set_flag_2`: Spins until bit 5 of a 16-bit word at offset 30 of a structure in a5 is set, then writes 32 to offset 156 of that structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_status_update`: If a 16-bit word in d0 is zero, it stores two longwords into offsets 128 and 132 of a structure in a5, sets bit 15 in three 16-bit words at offsets 4be6, 4be4, and 4be8 of a structure in a6, and copies them to offsets 150, 154, and 158 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `wait_for_flag_and_set_flag_extended`: Contains two separate logic blocks: one that spins until bit 5 of a 16-bit word at offset 30 of a structure in a5 is set and writes 32 to offset 156, and another that checks bit 0 of a 16-bit word at offset 4 of a structure in a5 and returns based on its state.
- `increment_volume_to_maximum`: Calls a helper routine, adds 273 to a 16-bit word at offset 19508 of a structure in a6, and copies a fixed 16-bit word into offsets 19512 and 19516 of that structure, repeating this loop until the word at offset 19508 equals 4095.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_and_return_status`: Stores a long value from a memory location into a global variable, sets bit 15 of three words at specific offsets in a structure in a6, stores those words into a structure in a5, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_flag_toggle_2`: It spins on a flag byte in a global structure until the flag is set, then spins until the flag is cleared again, and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_and_return`: Sets bit 15 of three words at specific offsets in a structure in a6, stores those words into a structure in a5, and returns from an interrupt.
- `wait_for_bit_and_set_word`: Polls a bit at offset 30 of a structure pointed to by a5 until bit 5 is set, then writes the value 32 to offset 156 of that same structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_bit`: Tests bit 0 of d0 and returns if the bit is clear, otherwise it branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_status_bits_2`: Sets bit 15 of three words at specific offsets in a structure in a6 and stores those words into a structure in a5 before returning from an interrupt.
- `wait_for_bit_and_set_word`: Polls a bit at offset 30 of a structure pointed to by a5 until bit 5 is set, then writes the value 32 to offset 156 of that same structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_and_compare_value`: Branches if a condition is not met, otherwise it calls another routine, adds a constant to a word in a structure in a6, loads two words from a table into a structure in a6, and loops until a word in a structure in a6 equals 4095.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_flag_and_restore_context`: Sets bit 15 of a 16-bit word at offset 4be8 of a structure in a6, copies three 16-bit words from offsets 4be6, 4be4, and 4be8 of that structure to offsets 96, 9a, and 9e of a structure in a5, then performs an exception return.
- `wait_for_bit_and_set_word_2`: Polls a bit at offset 30 of a structure pointed to by a5 until bit 5 is set, then writes the value 32 to offset 156 of that same structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_branch_data`: Copies a 16-bit word from a location relative to the program counter to offset 4c38 of a structure in a6, copies another 16-bit word to offset 4c3c of that structure, and branches if the word at offset 4c34 of that structure is not equal to 4095.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_context_partial`: Copies three 16-bit words from offsets 4be6, 4be4, and 4be8 of a structure in a6 to offsets 96, 9a, and 9e of a structure in a5, then performs an exception return.
- `wait_for_bit_and_set_word_2`: Polls a bit at offset 30 of a structure pointed to by a5 until bit 5 is set, then writes the value 32 to offset 156 of that same structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_branch_data_short`: Copies a 16-bit word from a location relative to the program counter to offset 4c3c of a structure in a6, compares the word at offset 4c34 of that structure to 4095, and branches if they are not equal.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_context_minimal`: Copies two 16-bit words from offsets 4be4 and 4be8 of a structure in a6 to offsets 9a and 9e of a structure in a5, then performs an exception return.
- `wait_for_bit_and_set_word_2`: Polls a bit at offset 30 of a structure pointed to by a5 until bit 5 is set, then writes the value 32 to offset 156 of that same structure and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_value_and_return`: Compares the 16-bit word at offset 4c34 of a structure in a6 to 4095 and branches if they are not equal, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_word_and_return`: Copies a word from an offset of 17422 in a structure pointed to by a6 to offset 158 of a structure pointed to by a5, then returns from an interrupt.
- `wait_for_flag_and_set_flag_4`: Spins until bit 5 of a word at offset 30 of a structure in a5 is set, then writes the constant 32 to offset 156 of that structure and returns.
- `wait_for_flag_toggle_3`: Spins on bit 0 of a word at offset 4 of a structure in a5 until it is set, then spins until it is cleared, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_return`: Branches to a relative address and otherwise returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_bit_flag`: Returns from an interrupt, then checks if bit 5 of a word at offset 30 of a structure in a5 is set; if it is, it writes the value 32 to offset 156 of that same structure and returns.
- `wait_for_flag_toggle_3`: Spins on bit 0 of a word at offset 4 of a structure in a5 until it is set, then spins until it is cleared, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `move_and_test_word`: Copies two words from relative offsets to offsets 19512 and 19516 of a structure in a6, then tests the word at offset 19508 of that structure and branches if it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_write_and_return`: Branches to a relative address and otherwise writes the constant 32 to offset 156 of a structure in a5 before returning.
- `wait_for_flag_toggle_4`: Spins on bit 0 of a word at offset 4 of a structure in a5 until it is set, then spins until it is cleared, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `move_test_and_branch`: Copies a word from a relative offset to offset 19516 of a structure in a6, then tests the word at offset 19508 of that structure and branches if it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_and_return`: Reads a 16-bit word from offset 4 of a structure pointed to by a5, checks if bit 0 is set, and returns immediately if it is not; otherwise, it performs the same check again and returns if bit 0 is set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_counter_until_fifteen`: Calls a helper four times via dbra on d1, then increments the word at offset 19504 from a6 and repeats until that word equals 15, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_bit_zero`: Loops while bit 0 of the word at offset 4 of the structure in a5 is set, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_counter_until_fifteen_2`: Calls another routine four times via a loop, then increments a 16-bit value at offset 19504 from a6 and repeats this process until the value reaches 15, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_bit_zero`: Loops while bit 0 of the word at offset 4 of the structure in a5 is set, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_counter_value`: Compares the word at offset 19504 from a6 to 15 and branches if they are not equal, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_volume_up_until_max`: Calls a helper, adds 273 to the word at a6 offset 19508, copies a fixed word into offsets 19512 and 19516, and repeats until that word equals 4095.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_return`: Branches if a comparison result is not equal, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_volume_up_until_max`: Calls a helper, adds 273 to the word at a6 offset 19508, copies a fixed word into offsets 19512 and 19516, and repeats until that word equals 4095.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counter_until_three`: Calls a helper four times via a loop, then decrements a word at offset 19504 from a6 and repeats until that word equals 3, then returns.
- `wait_for_register_match_2`: Compares the high byte of a word at offset 6 from a5 with itself and loops if they are equal, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_compare_word`: Copies a word from a relative offset to a word at offset 19516 from a6, then compares the word at offset 19508 from a6 to 4095 and branches if not equal.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_check_counter`: Decrements a word at offset 19504 from a6, then compares it to 3 and branches if not equal.
- `wait_for_high_byte_change`: Compares the high byte of a word at offset 6 from a pointer in a5 to itself, effectively creating an infinite loop unless the high byte changes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_branch_and_return`: Branches if a condition is not met, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_high_byte_change`: Compares the high byte of a word at offset 6 from a pointer in a5 to itself, effectively creating an infinite loop unless the high byte changes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_update_values`: Calls another routine, decrements a word at offset 19508 from a6 by 273, and stores a value from a global variable into words at offsets 19512 and 19516 from a6 in a loop until the word at offset 19508 is 0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_keyboard_input`: Checks bit 3 of a word at offset 30 from a5, and if set, performs bitwise operations on a byte read from a hardware register, calls another routine in a loop, and modifies a global byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_check_counter`: Increments a 16-bit word at offset 4c30 of the structure in a6, then compares it to 15 and returns if they are not equal.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_keyboard_serial`: If bit 3 of d1 is set, it reads a byte from the keyboard serial register, sets bit 6 of a CIAA control register, calls another routine three times in a loop, and then clears bit 6 of the CIAA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_check_counter_2`: Increments a 16-bit word in a global variable and compares it to fifteen; otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_keyboard_serial_2`: If bit 3 of d1 is not set, it reads a byte from the keyboard serial register, sets bit 6 of a CIAA control register, calls another routine three times in a loop, and then clears bit 6 of the CIAA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_counter_loop`: If a 16-bit word at offset 4c30 of the structure in a6 is not 15, it returns; otherwise, it calls another routine three times in a loop, decrements the word at offset 4c30, and returns if it is not 3.
- `wait_for_register_match_4`: Loads d6 and d7 from a word at offset six from the address in a5, masks the high bytes, and returns if the masked values are equal; otherwise, it enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `open_library_and_call_offset_negative_888`: Loads a library name string, opens version 39 of the library, and if successful calls the library entry at offset negative 888 with zero in d0 before closing the library and returning.
- `call_supervisor`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_ascii_string`: Loads an address into a0 and writes the 16-bit word 20083 to that location.
- `tiny_rte_2`: Small routine of 1 instruction that returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `check_serial_input_and_wait`: Tests bit three of a word at an offset of thirty in a structure via a pointer and, if set, reads the CIA serial data register, sets bit six of the CIA control register, repeatedly calls another routine until a condition is met, and then clears bit six of the CIA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library`: Calls a routine to close a library and then uses the returned pointer to call another routine.
- `call_supervisor_2`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_constant_to_memory`: Loads an address into a0 using the program counter and writes the word value 4e73 to that location.
- `return_from_exception`: Returns from an exception by restoring the processor status register and program counter from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_serial_data`: Tests bit three of d1 and, if set, reads the CIA serial data register, sets bit six of the CIA control register, performs a loop three times calling another routine, clears bit six of the CIA control register, and modifies a global byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `call_supervisor_3`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_constant_to_memory_2`: Loads an address into a0 using the program counter and writes the word value 4e73 to that location.
- `return_from_exception_2`: Returns from an exception by restoring the processor status and program counter from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `open_library_and_call_entry`: Loads a library name string, opens library version 39 via exec, and if successful calls its offset -888 entry with d0 zero, then closes the library through exec.
- `call_supervisor_4`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_string_constant`: Loads the address of a string into a register and writes a two-byte constant to that location.
- `return_from_exception_3`: Returns from an exception by restoring the processor status and program counter from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_data`: Clears a 16-bit value at an offset of a global structure, checks if a value at offset 530 of a6 is 12850, and if so sets the value at offset 19442 of a0 to 32, then calls a routine at offset -408 of a6 and uses the result to update three values in the global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_close_library`: Calls a routine at offset -552 of a6, and if the result is not zero, calls its offset -888 entry with d0 zero and then closes the library through exec.
- `call_supervisor_5`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_string_constant_2`: Loads the address of a string constant into a register and writes the word value nineteen thousand five hundred seventy-one to that location.
- `return_from_exception_4`: Returns from an interrupt or exception by restoring the processor status register and program counter from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_close_library`: Clears a 16-bit value at an offset of a global structure, checks if a value at offset 530 of a6 is 12850, and if so sets the value at offset 19442 of a0 to 32, then calls a routine at offset -408 of a6, uses the result to update three values in a global structure, and finally calls a routine at offset -888 of a6 and closes the library through exec.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library_only`: Calls a routine at offset -378 of a6 and then closes the library through exec.
- `call_supervisor_5`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_string_constant_2`: Loads the address of a string constant into a register and writes the word value nineteen thousand five hundred seventy-one to that location.
- `return_from_exception_4`: Returns from an interrupt or exception by restoring the processor status register and program counter from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_structure`: Compares a value at offset 530 of a6 to 12850, and if equal sets a value at offset 4bf2 of a0 to 32, then calls a routine at offset -408 of a6 and uses the result to update three values in a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_library_2`: Calls a library function to close a library and then calls another routine via a pointer in a6.
- `call_supervisor_6`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_constant_word`: Loads a word constant into a memory location relative to the program counter and then executes three no-operation instructions.
- `return_from_exception_5`: Returns from an exception by restoring the processor status and program counter from the exception stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_routines`: If a zero flag is set, it writes 32 to a global variable, then calls a routine to retrieve a pointer, stores three values from that pointer into global variables, and calls three other routines using pointers from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_stub_2`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `call_supervisor_6`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_constant_word`: Loads a word constant into a memory location relative to the program counter and then executes three no-operation instructions.
- `return_from_exception_5`: Returns from an exception by restoring the processor status and program counter from the exception stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routines_from_pointer`: Calls a routine to retrieve a pointer, stores three values from that pointer into global variables, and calls three other routines using pointers from that retrieved pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor_7`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_constant_word_2`: Loads a word constant into a memory location relative to the program counter and then executes three no-operation instructions.
- `return_from_exception_6`: Returns from an exception by restoring the processor status register and program counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_and_call_routines`: Stores three values from a pointer in d0 into global variables, then calls three other routines using pointers from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_interrupt`: Returns from an interrupt or exception by executing an interrupt return instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routines_from_global`: Stores d0 into a global variable, loads a pointer from a global variable, stores a value from that pointer into another global variable, and calls three other routines using pointers from the loaded pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_nop_rte`: Small routine of 2 instructions that does nothing for one instruction, then returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routines_from_global_pointer`: Loads a pointer from a global variable, stores a value from that pointer into a global variable, and calls three other routines using pointers from the loaded pointer.
- `execute_indirect_table_sequence`: Loads a library pointer from a global variable, calls two routines using offsets from that pointer, calls exec.CloseLibrary, and writes a 16-bit value from a global variable into the beam control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_structure`: Checks a value in a structure for a specific constant, sets a flag, and calls a library function to obtain a pointer to a new structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_table_sequence_2`: It reads a global variable, calls three routines via offsets from that value and from a second global, then stores a different global variable into the first global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointers_from_library`: If a word at an offset from a6 is equal to 3232, it writes 20 to a location in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_stub_3`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `close_libraries_and_set_beamcon`: Calls two routines via offsets from a pointer in a1, closes a library, and writes a word from a global variable to the beam control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `cleanup_and_close_libraries`: If a condition is met, it writes 20 to a location in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_structure_fields_and_call_routines`: It stores three longwords from a structure pointed to by a1 into a table, stores a longword from d0 into a global variable, and then calls three routines via offsets from the value in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_global_and_call_routines`: It stores a longword from d0 and a longword from a global variable into a table, then calls three routines via offsets from that global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_offset_routines`: Calls three routines via offsets from a pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_position`: Reads a 16-bit coordinate from a table using an index in a register, adds a signed 16-bit offset to it, and stores the result back into the table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_libraries_and_set_beamcon_alt`: Calls two routines via offsets from a pointer in a6, closes a library, and writes a word from a global variable to the beam control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_table`: Loads a pointer from a global variable, checks a byte at that pointer, and if it is not zero, enters a loop that calls another routine, updates a global variable with a calculated value, and eventually updates a pointer and a 16-bit value in a structure in a6.
- `modify_d0_and_branch`: Subtracts 32 from d0, then branches if d0 is zero, otherwise it shifts d0 left by 3 bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_via_loop`: Performs a loop that adds a value derived from d1 to a longword at offset 4bf4 of a structure in a6, then executes a nested loop that calls another routine while incrementing a pointer and a counter, and finally updates a word at offset 4bfc and a pointer at offset 4bf8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_table_2`: Enters a loop that calls another routine, decrements a counter, and updates a global variable, then checks a byte in a table to decide whether to continue the loop or update a pointer and a 16-bit value in a structure in a6.
- `modify_d0_and_branch_2`: Subtracts 32 from d0, then branches if d0 is zero, otherwise it shifts d0 left by 3 bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_offset_and_update_structure`: Increments d0 while scanning a buffer in a3 for a null byte or a byte with value 255, calculates a new value for a longword at offset 4bf4 of a structure in a6 based on d0, then enters a loop that increments that longword by 800 while scanning a table until a byte with value 255 is found, finally updating a word at offset 4bfc and a pointer at offset 4bf8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_pointer`: Increments a longword at offset 4bf4 of a structure in a6 by 800 for each byte in a buffer pointed to by a3 until a byte with value 255 is encountered, then stores a word at offset 4bfc and a pointer at offset 4bf8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_offset`: Calculates a value based on a loop counter and a string length, then updates a structure in a6 with the resulting offset, a shifted byte, and a pointer to the string's remainder.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_scaled_byte_and_pointer`: Copies a longword from a4 to offset 4bf4 of a structure in a6, checks if a byte at a3 is 255, and if not, increments a3, reads a byte, multiplies it by 4, stores the result in offset 4bfc of the structure, and stores the updated a3 in offset 4bf8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_offset_2`: Calculates a value based on a loop counting null-terminated characters, updates a field in a structure using a multiplier, and then increments that field through a second loop while storing a pointer and a shifted byte into the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_scaled_byte_and_pointer_2`: Checks if a byte at a3 is 255, and if not, increments a3, reads a byte, multiplies it by 4, stores the result in offset 4bfc of a structure in a6, and stores the updated a3 in offset 4bf8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_string_and_update_counter`: Calculates the length of a null-terminated string, updates a 32-bit value in a structure at offset 4bf4 by adding a value derived from the length, calls another routine in a loop based on the string length, and finally updates a pointer at offset 4bf8 and a 16-bit value at offset 4bfc.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_structure`: Stores the contents of register a3 into a structure at offset 4bf8 of a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_length_and_call`: Calculates the length of a null-terminated string, updates a 32-bit value in a structure at offset 4bf4 using a value derived from that length, calls another routine, and updates the structure at offset 4bf4 with the result of that call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `blit_glyph_into_bitmap`: Subtracts 32 from the character code in d0, scales it by 8, indexes a glyph table via a1, loads the destination bitmap pointer from a6-relative memory, calls the glyph copy routine and stores the updated pointer.
- `copy_struct_fields`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_length_and_update`: Calculates the length of a null-terminated string pointed to by a3, then adds the difference between 80 and that length to a 32-bit value at offset 19424 of a structure in a6 before calling another routine and restoring the original value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `blit_glyph_into_bitmap_partial`: Scales d0 by 8, indexes a glyph table via a1, loads a pointer from a6-relative memory, calls another routine, and stores the result in a structure at offset 4bf4 of a6.
- `copy_struct_fields`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_loop_2`: Checks a zero flag and enters a loop that calls another routine, updates a global variable based on a calculation, and checks a byte in a table to either return or continue the loop.
- `update_structure_field_via_call`: Calls another routine and stores the resulting value into a field at offset 19170 of a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_from_call`: Adds d0 to a1, loads a pointer from a6-relative memory, calls another routine, and stores the result in a structure at offset 4bf4 of a6.
- `copy_struct_fields_2`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at increments of eighty decimal, then increments the destination pointer by one and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_complex`: Enters a loop that updates a global variable, calls another routine within a nested loop, updates a global variable, and checks a byte in a table to either break or continue.
- `update_structure_field_via_call`: Calls another routine and stores the resulting value into a field at offset 19170 of a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_short`: Calls another routine and stores the resulting pointer in a structure at offset 4bf4 of a6.
- `copy_struct_fields_offset_8`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at increments of 80 decimal.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_update_structure`: Decrements a counter in a loop, calls another routine, and updates a structure at offsets 4bf4, 4bfc, and 4bf8 from a6 when a byte in a table equals negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_to_offsets`: Copies six bytes from a source pointed to by a1 to offsets 160, 240, 320, 400, 480, and 560 from a0, then increments a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_with_loop`: Iterates through a table, calling another routine within a nested loop, and updates a structure at offsets 4bf4, 4bfc, and 4bf8 from a6 when a terminator is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_five_bytes_to_offsets`: Copies five bytes from a source pointed to by a1 to offsets 240, 320, 400, 480, and 560 from a0, then increments a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_scaled_byte_and_pointer_3`: Increments a pointer in a3, takes a byte from that pointer, shifts it left by two, stores the result as a word at offset 4bfc from a6, and stores the updated pointer at offset 4bf8 from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_single_byte_to_offset`: Copies a byte from a source pointed to by a1 to offset 560 from a0, then increments a0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_scaled_byte_and_pointer_short`: Takes a byte from a pointer in a3, shifts it left by two, stores the result as a word at offset 4bfc from a6, and stores the incremented pointer at offset 4bf8 from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_addq_rts`: Small routine of 2 instructions that adds 1 to a0, then returns.
- `store_two_pointer_pairs_in_state`: Loads the address of the first state block and the address of the second state block, then writes each pointer's low and high words into four state fields at offsets 4c60, 4c5c, 4c68 and 4c64 from a6.
- `clear_zero_table`: Clears a block of memory by writing zero to a sequence of longwords, iterating a fixed number of times while decrementing a counter and advancing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_offset_values`: Swaps the high and low words of a register, stores the low word at offset 4c5c of a structure in a6, stores the address at offset 4c82 of a6 into a register, stores that address as a word at offset 4c68 of the structure, swaps the register again, and stores the resulting low word at offset 4c64 of the structure.
- `clear_zero_table_2`: Clears a block of memory by writing zero to a sequence of longwords, iterating a fixed number of times while decrementing a counter and advancing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor_with_offset`: Shifts a value in d0 left by three, adds it to a pointer in a1, loads a pointer from a table into a0, calls another routine, and stores the resulting pointer at offset 4bf4 of a structure in a6.
- `copy_sparse_struct_bytes`: Copies a byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty bytes, and increments the destination pointer by one before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_pair`: Stores the low word of a register at offset 4c68 of a structure in a6 and stores the high word of that register at offset 4c64 of the structure.
- `clear_zero_table_3`: Clears a block of memory by writing zero to a sequence of longwords, iterating 4792 times while decrementing a counter and advancing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor_direct`: Adds a value in d0 to a pointer in a1, loads a pointer from a table into a0, calls another routine, and stores the resulting pointer at offset 4bf4 of a structure in a6.
- `copy_sparse_struct_bytes`: Copies a byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty bytes, and increments the destination pointer by one before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_high_word`: Swaps the high and low words of a register and stores the resulting low word at offset 4c64 of a structure in a6.
- `clear_zero_table_3`: Clears a block of memory by writing zero to a sequence of longwords, iterating 4792 times while decrementing a counter and advancing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointer_to_offset`: Calls another routine and then stores the resulting pointer from a0 into a global variable at offset 19664 of the structure in a6.
- `copy_struct_fields_spaced`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `clear_zero_table_4`: Clears a block of memory by writing zero to a sequence of longwords, iterating 4792 times while decrementing a counter and advancing a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `copy_struct_fields_spaced`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_fill_memory_block`: Fills a block of memory starting at the address in a0 with 4568 zeroed longwords using a loop, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointer_to_offset`: Calls another routine and then stores the resulting pointer from a0 into a global variable at offset 19664 of the structure in a6.
- `copy_struct_fields_spaced`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_and_loop`: Writes the value from d7 to the memory location pointed to by a0 and then returns if d0 is zero, otherwise it loops back to the same instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_three_bytes`: Copies three bytes from a source pointed to by a1 to offsets 400, 480, and 560 of a structure in a0, then increments a0 by 4 and returns.
- `store_two_pointer_pairs_in_state_2`: Loads the address of the first state block and the address of the second state block, then writes each pointer's low and high words into four state fields at offsets 4c60, 4c5c, 4c68 and 4c64 from a6.
- `clear_zero_block`: Clears a block of memory by writing zero to a sequence of longwords, iterating 4600 times while advancing a pointer and decrementing a counter, then returns.

### Blob (1993)(Core Design)[cr FLT][t +10 GOD].adf:fltintro#unpacked

- `initialize_system_and_execute_sequence`: Saves all registers, calls two setup routines, loads a5 with a program base, invokes exec.Supervisor via the library vector at offset minus thirty, then calls two more routines and returns zero in d0.
- `manage_copper_interrupt_state`: Saves custom chip configuration, reprograms DMA, color, and copper registers, polls the fire button and CIA ports, toggles a bit in the beam control register, and restores the original hardware state before returning from the exception.
- `wait_for_flag_and_set_flag`: It spins until a specific bit in a word at a certain offset from a pointer in a register is set, then writes a constant word to another offset from that same pointer and returns.
- `wait_for_flag_toggle`: It spins on a flag byte in a global structure until the flag is set, then spins until the flag is cleared again, and then returns.
- `step_volume_up_until_max_2`: Calls a helper, adds 273 to the word at a6 offset 19508, copies a fixed word into offsets 19512 and 19516, and repeats until that word equals 4095.
- `step_volume_down_until_zero`: Calls a helper, subtracts 273 from the word at a6 offset 19508, copies a fixed word into offsets 19512 and 19516, and repeats until that word reaches zero.
- `increment_counter_until_fifteen_3`: Calls a helper four times via a loop, then increments the word at offset 19504 from a6 and repeats until that word equals 15, then returns.
- `decrement_counter_until_three_2`: Calls a helper four times via a loop, then decrements the word at offset 19504 from a6 and repeats until that word equals 3, then returns.
- `wait_for_register_match`: It loads an address into a register and then loops while the high byte of a word at offset six from that address is equal to itself, effectively creating an infinite loop.
- `check_serial_input_and_wait_2`: Tests a flag in a structure via a pointer and, if set, toggles a bit in a CIA register while repeatedly calling another routine until a short condition holds, then restores the register bit.
- `open_graphics_library_and_call_offset`: Opens graphics.library version 39, calls its offset minus 888 with zero in d0, and then closes the library; otherwise it returns.
- `call_supervisor_8`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `open_library_and_store_vectors`: Clears a word in a global variable and, if a value at an offset in the exec base equals 12850, sets that global variable to 32.
- `execute_indirect_table_sequence_3`: It reads a global variable, calls three routines via offsets from that value and from a second global, then stores a different global variable into the first global variable.
- `draw_centered_text_block`: Calls a setup routine, counts null-terminated strings until a 255 byte, centers the block vertically in a 21-row area, then for each string centers it horizontally and calls the glyph blitter, finally storing the trailing width and pointer.
- `blit_glyph_into_bitmap_2`: Subtracts 32 from the character code in d0, scales it by 8, indexes a glyph table via a1, loads the destination bitmap pointer from a6-relative memory, calls the glyph copy routine and stores the updated pointer.
- `copy_struct_fields_4`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets at intervals of eighty decimal.
- `store_two_pointer_pairs_in_state_3`: Loads two addresses into registers, then writes the low and high words of each address into four consecutive 16-bit fields at offsets 4c5c, 4c60, 4c64, and 4c68 from a6.
- `clear_zero_memory`: Clears a block of memory by writing zero to a sequence of longwords, iterating 4600 times while decrementing a counter and advancing a pointer.

The function list continues in: .
