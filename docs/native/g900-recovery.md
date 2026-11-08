# Game 900 (rampage (1989)(activision)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00900). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18 of 18 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 306 |
| code | 70 |
| data | 3090 |
| padding | 182 |
| unused | 897536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rampage (1989)(Activision).adf:bootblock | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |

## Cases handled

- rampage (1989)(activision)/Rampage (1989)(Activision).adf: file            (header 5124 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rampage (1989)(Activision).adf:bootblock

- `clear_memory_then_jump`: Clears a 128-word region of memory starting at address a fixed address by filling it with zeros, then jumps to the entry point at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_reset_structure`: Loops while a 16-bit value in d0 does not match the word at offset 5372 of the structure in a1, then checks whether the word at offset -31336 is zero.
