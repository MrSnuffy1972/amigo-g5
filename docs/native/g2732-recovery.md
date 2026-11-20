# Game 2732 (ArmageddonMan v1.0 0629): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02732). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (829712 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (4 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 88900 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 28 of 28 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20537 |
| code | 92 |
| data | 809078 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 10 | 4 |
| Armageddon_Man | 1 | 18 | 1 |
| dat0 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ArmageddonMan_v1.0_0629/ArmageddonMan_v1.0_0629.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree ArmageddonMan_v1.0_0629_hdf, which is recovered file by file; no block outside the filesystem holds data
- ArmageddonMan_v1.0_0629/ArmageddonMan_v1.0_0629_hdf/ArmageddonMan/data/dat1: high-entropy data file (8.00 bits/byte), classified as packed data
- ArmageddonMan_v1.0_0629/ArmageddonMan_v1.0_0629_hdf/ArmageddonMan/data/dat2: high-entropy data file (8.00 bits/byte), classified as packed data
- ArmageddonMan_v1.0_0629/ArmageddonMan_v1.0_0629_hdf/ArmageddonMan/data/dat3: high-entropy data file (7.99 bits/byte), classified as packed data
- ArmageddonMan_v1.0_0629/ArmageddonMan_v1.0_0629_hdf/ArmageddonMan/data/dat0: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 88900 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_ori_pea_bra`: Short code fragment that merges 0 into d2, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### Armageddon_Man

- `xor_table_loop`: Performs XOR operations between values from two tables, updating the second table with the results through nested loops that iterate through specific offsets.
