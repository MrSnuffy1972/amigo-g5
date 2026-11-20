# Game 4072 (JetSetWilly2 v0501.1 1115): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04072). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1685295 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (3 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 40 of 40 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9679 |
| code | 146 |
| data | 1675468 |
| padding | 2 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 40 | 4 |
| boot (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115.hdf: hard-disk image (AmigaDOS OFS, 1856 blocks): its filesystem parses and all 24 files in it are byte-identical to the extracted tree JetSetWilly2_v0501.1_1115_hdf, which is recovered file by file; no block outside the filesystem holds data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/credits: high-entropy data file (7.77 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/drainp: high-entropy data file (7.82 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/drains: high-entropy data file (7.82 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/endpic: high-entropy data file (7.66 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/island: high-entropy data file (7.83 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/islandp: high-entropy data file (7.82 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/jetset: high-entropy data file (7.88 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/main: high-entropy data file (7.84 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/manic: high-entropy data file (7.87 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/mansion: high-entropy data file (7.85 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/passage: high-entropy data file (7.86 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/secret: high-entropy data file (7.83 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/select: high-entropy data file (7.86 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/space: high-entropy data file (7.81 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/spacep: high-entropy data file (7.86 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/speccyver: high-entropy data file (7.87 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/teleport: high-entropy data file (7.80 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/teleportp: high-entropy data file (7.81 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/titles: high-entropy data file (7.93 bits/byte), classified as packed data
- JetSetWilly2_v0501.1_1115/JetSetWilly2_v0501.1_1115_hdf/JetSetWilly2/data/boot: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_input_state`: Reads and processes input from a hardware port, updates a global variable with the result, and performs a brief delay loop synchronized with vertical blanking.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack (3 times in a row), then jumps to another point in the code.
- `frag_pea_nop_move_addq_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then pauses for one no-operation step, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
