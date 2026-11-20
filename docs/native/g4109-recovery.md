# Game 4109 (JumpNRoll v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04109). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (151242 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (3 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 108 of 108 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9445 |
| code | 262 |
| data | 141535 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Jnroll | 1 | 101 | 3 |
| game.slave | 1 | 7 | 3 |

## Cases handled

- JumpNRoll_v1.0/JumpNRoll_v1.0.hdf: hard-disk image (AmigaDOS OFS, 160 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree JumpNRoll_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- JumpNRoll_v1.0/JumpNRoll_v1.0_hdf/JumpNRoll/Jnroll: StoneCracker 4.04 Addr (xfd) stream found by ancient scan (56688 -> 176128 bytes) but only 0 of 34 sampled windows are in reference RAM: not used
- JumpNRoll_v1.0/JumpNRoll_v1.0_hdf/JumpNRoll/Jnroll: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Jnroll

- `initialize_and_process`: Initializes several registers and memory pointers, then processes data through multiple conditional branches and subroutine calls while updating counters and storing results in memory.
- `shift_and_accumulate`: Performs bitwise operations on word values from registers and memory locations, shifting and accumulating results in a loop structure that processes data from a pointer.
- `copy_byte_sequence`: Copies bytes between memory locations using decrementing counters and conditional branches, with a final comparison that may trigger a jump to another routine.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
