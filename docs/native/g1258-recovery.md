# Game 1258 (RaceDrivin v0601.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01258). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4046753 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (1 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 8192 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 612 of 612 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20577 |
| code | 258 |
| data | 2828347 |
| padding | 3 |
| unused | 1197568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 22 | 1 |
| Disk.2:bootblock | 1 | 26 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 18 | 2 |
| logged disk load from Disk.2 boot-block load | 1 | 544 | 37 |
| Disk.1 bytes 23552-26112 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 27648-33280 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: custom-format (non-DOS) disk, 1164 non-empty blocks: matched against track-loaded segments
- RaceDrivin_v0601.1/RaceDrivin_v0601.1.hdf: hard-disk image (AmigaDOS OFS, 4352 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree RaceDrivin_v0601.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: the boot block reads 8192 bytes from disk offset 15360 to an address (found statically); recovered as a unit
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.2: the boot block reads 153600 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: bytes 23552-26112 look like code no run loaded: recovered position-independently (load address unknown)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: bytes 27648-33280 look like code no run loaded: recovered position-independently (load address unknown)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: 3 logged loads cover 16384 bytes of the image
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 4-4 (bytes 2048-2560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.28)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 51-53 (bytes 26112-27648) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.99)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 68-1168 (bytes 34816-598528) hold 9 packed streams (Ice: Pack-Ice v2.0 - v2.20, SCO Compress LZH) found by scanning, 99% of the bytes
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1188-1188 (bytes 608256-608768) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1210-1210 (bytes 619520-620032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1232-1232 (bytes 630784-631296) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1254-1254 (bytes 642048-642560) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1276-1276 (bytes 653312-653824) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1298-1298 (bytes 664576-665088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1320-1320 (bytes 675840-676352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1342-1342 (bytes 687104-687616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1364-1364 (bytes 698368-698880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1386-1386 (bytes 709632-710144) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1408-1408 (bytes 720896-721408) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1430-1430 (bytes 732160-732672) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1452-1452 (bytes 743424-743936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1474-1474 (bytes 754688-755200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1496-1496 (bytes 765952-766464) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1518-1518 (bytes 777216-777728) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1540-1540 (bytes 788480-788992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1562-1562 (bytes 799744-800256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1584-1584 (bytes 811008-811520) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1606-1606 (bytes 822272-822784) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1628-1628 (bytes 833536-834048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1650-1650 (bytes 844800-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1672-1672 (bytes 856064-856576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1716-1716 (bytes 878592-879104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.1: blocks 1738-1738 (bytes 889856-890368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.2: 1 logged loads cover 0 bytes of the image
- RaceDrivin_v0601.1/RaceDrivin_v0601.1_hdf/RaceDrivin/data/Disk.2: blocks 11-21 (bytes 5632-11264) hold ASCII text
- 2 never-loaded region(s), 8192 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `allocate_and_initialize_memory`: Calls a routine to allocate memory, and if successful, populates a structure with specific values before calling another routine and performing a trap.

### Disk.2:bootblock

- `allocate_and_execute_jump`: Allocates memory, populates a structure, and calls a routine; if a specific byte in the structure is non-zero, it calls two more routines before jumping to the address returned by the initial allocation.

### logged disk load from Disk.1 boot-block load

- `copy_memory_block_loop`: Sets the status register, compares d0 to a constant, and if d0 is not greater than or equal to that constant, enters a loop that copies 29 bytes from a source at a1 to a destination at a0 before jumping to a location at offset 70000.
- `copy_memory_block`: Copies 1471 words from a source pointed to by a1 to a destination starting at a memory location pointed to by a0 using a loop.

### logged disk load from Disk.2 boot-block load

- `initialize_hardware_and_copy_data`: Sets the status register, calls another routine, and then uses a loop to copy 29 bytes from a memory location to a destination pointed to by a1 before jumping to a long address.
- `copy_memory_block_2`: Copies a block of bytes from a source pointed to by a1 to a destination pointed to by a0 using a counter in d7, and jumps to a location stored in a2 when finished.
- `copy_bytes_with_counter`: Copies bytes from a source pointed to by a1 to a destination pointed to by a0, decrementing a counter in d7 until it reaches zero, then jumps to the address in a2.
- `wait_for_interrupt_and_disable`: Polls hardware registers to wait for specific interrupt conditions, then writes values to several hardware registers and two memory locations.
- `store_address_to_global`: Stores the address of a local label into a memory location at an offset from the program counter.
- `store_longword_and_update_index`: Disables interrupts, compares a word at offset 4608 of a6 with a word at offset 4606 of a6, and if they are not equal, stores a longword from a4 into a destination at an offset from a0 determined by a word from a6, updates a word at offset 4608 of a6, and then re-enables interrupts before branching.
- `store_and_call_routine`: Stores a longword from a4 into a destination at an offset from a0, updates a value at an offset from a6, and calls a routine using a pointer from a4.
- `execute_four_subroutines`: Calls four different subroutines in sequence and then returns.
- `modify_and_branch_on_scaled_value`: Adds a 16-bit value to another, shifts the result right by one bit, and calls four separate routines; if the resulting value minus thirty-nine is not negative, it moves that value to d0 and calls a fifth routine before returning.
- `swap_and_copy_tables`: Compares two values in a global structure to determine an order, then copies two longwords from two different source tables into a destination table, repeating this process with a second comparison.
- `initialize_multiple_structures`: Calculates a value based on d1, and if a specific condition is met, performs four consecutive sequences where it sets up a local structure on the stack and calls another routine with varying parameters derived from a pointer at offset 21048 of a6.
- `initialize_structure_fields`: Writes several constant values and values from a4 and a6 into a structure pointed to by a5.
- `update_structure_subset`: Copies a word from a global variable and two values from a4 into a structure pointed to by a5.
- `update_structure_fields`: Copies a word from a global variable into a structure at offset 116 of a6, a 32-bit value from a pointer at offset -18 of a4 into offset 84 of a5, and a word from offset -36 of a4 into offset 88 of a5.
- `update_structure_fields_alt`: Copies a word from a global variable at offset 5226 of a6 into offset 116 of a5, a 32-bit value from offset -18 of a4 into offset 84 of a5, and a word from offset -36 of a4 into offset 88 of a5.
- `calculate_division_and_jump_2`: Calculates a value using a multiplication of a word in d0 by 160, modifies a pointer in a0 based on that result, and performs signed division using d0 and d1 before branching to another routine or performing a swap and jump.
- `calculate_division_and_jump`: Performs signed division using the high and low words of d1, and then branches to a routine using a target calculated from a value in a table at offset 21452 of a6.
- `execute_jump_table`: Performs arithmetic on d2 to calculate a jump target, loads a byte from a table at offset 21452 of a6, and jumps to a location calculated from the program counter and d5.
- `execute_loop_branch`: Decrements a counter in d1 and branches to a previous address if the counter is not zero, otherwise it returns.
- `update_multiple_structure_fields`: If d0 is not zero, calculates a value based on d0, and then performs four consecutive sequences where it sets up a local structure on the stack and calls another routine with varying parameters derived from a pointer at offset 21048 of a6.
- `initialize_structure_values`: Sets words at offsets 68, 70, 64, and 66 of a5 to specific constants, copies a word from offset -26 of a4 to offset 102 of a5, and copies a word from offset 5276 of a6, a 32-bit value from offset -18 of a4, and a word from offset -36 of a4 into offsets 116, 84, and 88 of a5 respectively.
- `update_structure_fields_v2`: Copies a word from a global variable at offset 5278 of a6 into offset 116 of a5, a 32-bit value from offset -18 of a4 into offset 84 of a5, and a word from offset -36 of a4 into offset 88 of a5.
- `update_structure_fields_v3`: Copies a word from a global variable at offset 5280 of a6 into offset 116 of a5, a 32-bit value from offset -18 of a4 into offset 84 of a5, and a word from offset -36 of a4 into offset 88 of a5.
- `update_structure_fields_v4`: Copies a word from a global variable at offset 5282 of a6 into offset 116 of a5, a 32-bit value from offset -18 of a4 into offset 84 of a5, and a word from offset -36 of a4 into offset 88 of a5.
- `calculate_and_queue_sprite_updates`: Calculates sprite position and attribute data based on input values and a structure in a6, then populates a local stack-based structure and calls another routine to queue updates for one or two sprite objects.
- `process_structure_updates`: Calculates a value from d7, and if it is not equal to 20, it performs two sequences of setting up a local structure on the stack and calling another routine with parameters derived from a1 and a2.
- `check_video_position`: Compares a word from a global variable at offset 5392 of a6 with a word at offset 5404 of a6, and if they are equal, checks if the vertical position from the hardware register is less than 204.
- `branch_based_on_signs`: Checks the sign of d1 and d0, and if both are not negative, calls another routine and branches to another routine.
- `negate_and_branch`: Negates the word in d0, calls another routine, negates d0 again, and branches to another routine.
- `conditional_negate_and_call`: Negates the word in d1, checks the sign of d0, and if d0 is not negative, calls another routine, negates d0, adds 32768 to d0, and branches to another routine.
- `negate_and_call_routine`: Negates the word in d0, calls another routine, adds 32768 to d0, and returns.
- `perform_signed_division_and_compare`: Performs signed division using d0 and d1, calculates an index, and compares a word from a table at an offset from a6 against d0, then sets d3 to either 2 or -2 based on the comparison result.
- `compare_and_increment_index`: Compares a 16-bit word at an offset determined by a register into a table with a value in a register, then either adds a value to that index register and branches to another routine or branches to another routine if the comparison is less than or equal.
- `subtract_shifted_word`: Shifts d1 left by 4 bits, then subtracts the new d1 from 16384 and branches to another routine.
- `calculate_division_and_compare`: Performs a signed division using the high word of d0 and d1, calculates an index based on the result, and compares a word at that index from a structure at offset -576 of a6 with d0, then branches to another routine or sets d3 depending on the comparison.
- `compare_and_add_index`: Compares a word at offset 2 plus d1 from a0 with d0, and if d0 is not less than or equal to that word, adds d3 to d1 and branches to another routine.
- `calculate_division_result`: Performs a division-related calculation using values from a table at a0 and d1, and returns the result in d0, potentially negating it based on d2.
