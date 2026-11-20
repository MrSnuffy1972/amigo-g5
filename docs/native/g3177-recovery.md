# Game 3177 (ColorBuster v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03177). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (947589 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (3 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 32416 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 60 of 60 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4466 |
| code | 234 |
| data | 942869 |
| padding | 20 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Color | 1 | 58 | 4 |
| game.slave | 1 | 2 | 1 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ColorBuster_v1.0/ColorBuster_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1024 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree ColorBuster_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- ColorBuster_v1.0/ColorBuster_v1.0_hdf/ColorBuster/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 32416 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Color

- `memory_copy_loop`: Performs a loop that copies data from one memory region to another, decrementing a counter and branching back until the counter reaches zero.
- `compare_and_branch`: Loads a long value from a fixed memory address into a register, compares it with another value from the same address, and branches to a specific location if they are not equal.
- `frag_lea_tst_bne_bra`: Short code fragment that loads the address of a fixed memory address into a0, then tests d7, then branches if the last result was not zero, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `frag_lea_move_subq_tst_bne_jmp`: Short code fragment that loads the address of a fixed memory address into a1, then copies 94976 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then subtracts 1 from d0, then tests d0, then branches if the last result was not zero, then jumps to another point in the code.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
