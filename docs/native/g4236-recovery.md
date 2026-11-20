# Game 4236 (LegendOfRagnarok v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04236). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6040521 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (14 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 268 of 268 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9849 |
| code | 270 |
| data | 3332173 |
| padding | 1013 |
| unused | 2697216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 40 | 2 |
| Disk.2:bootblock | 1 | 10 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 216 | 19 |

## Cases handled

- LegendOfRagnarok_v1.0/LegendOfRagnarok_v1.0.hdf: hard-disk image (AmigaDOS OFS, 6496 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree LegendOfRagnarok_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- LegendOfRagnarok_v1.0/LegendOfRagnarok_v1.0_hdf/LegendOfRagnarok/Disk.1: the boot block reads 3072 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- LegendOfRagnarok_v1.0/LegendOfRagnarok_v1.0_hdf/LegendOfRagnarok/Disk.1: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `handle_input_and_graphics` (Disk.1 boot-block load +an address to an address (3072 bytes)): Performs calculations on input coordinates, sets up graphics registers, clears specific bit flags, and processes input from hardware devices. Kind: disk hardware check. Pass path: The check passes if the value at DAT_0000010a is non-negative after u3__sub_060A() is called, and the routine continues to u3__sub_05CA() and eventually reaches the end where CIAB_PRB is manipulated to signal success..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `initialize_system_hardware`: Initializes hardware registers including interrupt and DMA control, sets up memory pointers, and calls another routine before jumping to a program location.
- `initialize_memory_regions`: Tests memory regions for valid type and sets global variables with specific values based on the first successful test, returning immediately if none match.

### Disk.2:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Disk.1 boot-block load

- `frag_lea_movea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 1024 to a1, then copies 174 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_move_rts`: Short code fragment that copies 1050 to a fixed memory address, then copies 65535 to a fixed memory address, then copies 3 to a fixed memory address, then returns.
- `frag_movem_bsr_rte`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns from the exception.
- `handle_input_and_graphics`: Performs calculations on input coordinates, sets up graphics registers, clears specific bit flags, and processes input from hardware devices.
- `frag_move_bsr_bra`: Short code fragment that copies the top of the stack to d6, then copies 5632 to d7, then calls another routine, then jumps to another point in the code.
- `update_and_process_loop`: Processes a loop that updates counters, calls other routines, and modifies bit flags in memory based on conditional checks.
- `modify_bit_flags`: Sets and clears specific bit flags in memory locations based on a value read from another memory location.
- `process_data_blocks`: Processes 10 data blocks by reading from a fixed location, performing bit operations, and writing results to a calculated destination address.
- `conditional_data_process`: Processes 10 data blocks with conditional checks on calculated values, writing results only when the calculated value meets specific range criteria.
- `setup_graphics_registers`: Sets up graphics register values and waits for a bit to be cleared before proceeding with additional register updates.
- `frag_lsr_bcc_bclr_bra`: Short code fragment that shifts d0 right by 1, then branches if the carry flag is clear, then clears bit 2 of the memory at a5, then jumps to another point in the code.
- `update_counter_and_flags`: Updates a counter value, performs conditional operations based on the counter, and modifies flags in memory locations.
- `frag_bclr_subq_bsr_dbra_rts`: Short code fragment that clears bit 1 of the memory at a5, then subtracts 1 from d0, then calls another routine (2 times in a row), then loops back while counting down d0, then returns.
- `frag_bset_btst_beq_bsr_bra`: Short code fragment that sets bit 1 of the memory at a5, then tests bit 4 of the memory at a4, then branches if the last result was zero, then calls another routine (2 times in a row), then jumps to another point in the code.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_bclr_bsr_bset_rts`: Short code fragment that clears bit 0 of the memory at a5, then calls another routine, then sets bit 0 of the memory at a5, then returns.
- `frag_moveq_move_cmp_beq_dbra`: Short code fragment that loads 99 into d7, then copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then loops back while counting down d7.
- `frag_btst_bne_rts`: Short code fragment that tests bit 5 of the memory at a4, then branches if the last result was not zero, then returns.
- `frag_moveq_move_cmp_beq_dbra_rts`: Short code fragment that loads 5 into d7, then copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then loops back while counting down d7, then returns.
