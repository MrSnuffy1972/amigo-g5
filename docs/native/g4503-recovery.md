# Game 4503 (NavyMoves v1.2 2428): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04503). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2048035 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (7 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 173 of 173 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9358 |
| code | 494 |
| data | 2016167 |
| unused | 22016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 152 | 14 |
| Disk.1:bootblock | 1 | 21 | 1 |

## Cases handled

- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: custom-format (non-DOS) disk, 1715 non-empty blocks: matched against track-loaded segments
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree NavyMoves_v1.2_2428_hdf, which is recovered file by file; no block outside the filesystem holds data
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 2-22 (bytes 1024-11776) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.94)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 33-43 (bytes 16896-22528) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.21)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 66-643 (bytes 33792-329728) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.76)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 646-739 (bytes 330752-378880) outside the filesystem hold data, not code (opcode density 0.056, entropy 5.47)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 741-936 (bytes 379392-479744) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.12)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 938-1055 (bytes 480256-540672) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.05)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 1057-1185 (bytes 541184-607232) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.48)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 1188-1387 (bytes 608256-710656) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.10)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 1389-1507 (bytes 711168-772096) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.01)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 1510-1689 (bytes 773120-865280) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.17)
- NavyMoves_v1.2_2428/NavyMoves_v1.2_2428_hdf/NavyMoves/Disk.1: blocks 1691-1759 (bytes 865792-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_2`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_clr_pea_bra`: Short code fragment that clears d0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_movea_jmp`: Short code fragment that copies a location in the code to a2, then jumps to another point in the code.
- `initialize_and_process_data`: Initializes data structures and performs bit manipulation on word values before copying processed data back to the original location.
- `frag_moveq_lsl_move_dbra_rts`: Short code fragment that loads 3 into d1, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then returns.
- `copy_and_adjust_data_2`: Copies data from one memory location to another while adjusting values based on table lookups, and performs conditional operations depending on comparison results.
- `frag_add_bne_move_addx_rts`: Short code fragment that adds d7 to d7, then branches if the last result was not zero, then copies the memory just below a5 (moving a5 down) to d7, then adds d7 and the extend flag to d7, then returns.
- `frag_moveq_add_bne_move_addx_dbra_rts`: Short code fragment that loads 0 into d1, then adds d7 to d7, then branches if the last result was not zero, then copies the memory just below a5 (moving a5 down) to d7, then adds d7 and the extend flag to d7, then adds d1 and the extend flag to d1, then loops back while counting down d0, then returns.
- `process_data_fragment`: Processes a data fragment by reading values from a table, performing arithmetic operations, and copying bytes based on calculated offsets.
- `copy_and_adjust_data`: Copies data bytes from a source location to a destination, adjusting pointers and counters during the process, then continues with further data manipulation based on conditional checks.

### Disk.1:bootblock

- `read_disk_block_into_memory`: Allocates memory, sets up an I/O request with a length of 12288 bytes and an offset of 1024, executes a read command, clears the length, and executes a second command with code nine before returning zero.
