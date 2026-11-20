# Game 3463 (DrMario v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03463). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (259162 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (2 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 82 of 82 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8120 |
| code | 192 |
| data | 250850 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| DrMario.exe | 1 | 80 | 4 |
| game.slave | 1 | 2 | 1 |

## Cases handled

- DrMario_v1.0/DrMario_v1.0.hdf: hard-disk image (AmigaDOS OFS, 288 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree DrMario_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- DrMario_v1.0/DrMario_v1.0_hdf/DrMario/DrMario.exe: ByteKiller 3.0 (xfd) stream found by ancient scan (102472 -> 199956 bytes) but only 0 of 76 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### DrMario.exe

- `initialize_game_state`: Initializes game state variables and sets up pointers for memory operations, including loading data from a table and configuring registers for subsequent processing.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `shift_packed_longword_right`: Returns the longword four bytes before the address register shifted right by one bit, with the top bit set from a flag bit in a data register, which is a single step of a bit reader.
- `shift_bits_from_packed_longwords`: Shifts a register right by a count given in the argument, reloading from the previous packed longword whenever the register empties and feeding a flag into the top bit, which is a bit-reader loop.

### game.slave

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
