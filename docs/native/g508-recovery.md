# Game 508 (Heroes of the Lance V1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00508). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1817040 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (7 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1120 of 1120 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 7168 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2607 of 2607 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4704 |
| code | 9914 |
| data | 1785933 |
| padding | 105 |
| unused | 16384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf:bootblock | 1 | 102 | 3 |
| logged disk load from Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1689 | 63 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 106 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 135 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf bytes 1536-8704 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4E75) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: the boot block reads 512 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 449 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 106 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 340 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: bytes 1536-8704 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: 2 logged loads cover 7680 bytes of the image
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: 7 track-loaded blocks found in recovered runtime segments and classified through them
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 23-26 (bytes 11776-13824) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 30-157 (bytes 15360-80896) outside the filesystem hold data, not code (opcode density 0.042, entropy 6.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 159-159 (bytes 81408-81920) outside the filesystem hold data, not code (opcode density 0.031, entropy 1.37)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 161-806 (bytes 82432-413184) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.99)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 809-821 (bytes 414208-420864) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 825-879 (bytes 422400-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 882-894 (bytes 451584-458240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 1 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf: blocks 898-1759 (bytes 459776-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 2-411 (bytes 1024-210944) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.26)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 413-425 (bytes 211456-218112) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.54)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 427-442 (bytes 218624-226816) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 444-768 (bytes 227328-393728) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.94)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 770-1159 (bytes 394240-593920) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.91)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1161-1162 (bytes 594432-595456) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1164-1169 (bytes 595968-599040) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.96)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1171-1247 (bytes 599552-638976) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.43)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1249-1344 (bytes 639488-688640) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.09)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1346-1347 (bytes 689152-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1349-1360 (bytes 690688-696832) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.92)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1362-1363 (bytes 697344-698368) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.45)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1365-1366 (bytes 698880-699904) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.90)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1368-1378 (bytes 700416-706048) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1381-1382 (bytes 707072-708096) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.89)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1384-1400 (bytes 708608-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.36)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1402-1403 (bytes 717824-718848) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- heroes of the lance v1.0 (1988)(ssi)(disk 2 of 2)/Heroes of the Lance v1.0 (1988)(SSI)(Disk 2 of 2).adf: blocks 1405-1759 (bytes 719360-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.03)
- 1 never-loaded region(s), 7168 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Heroes of the Lance v1.0 (1988)(SSI)(Disk 1 of 2).adf:bootblock

- `find_and_init_io_request`: Initializes an I/O request in a structure, then searches a table of signatures until it finds a match and stores the matching entry's offsets back into that request.
- `send_zero_length_io_command`: Stores the decimal value nine at offset 28 of the structure pointed to by a1, clears the longword at offset 36 of the structure pointed to by a3, then calls the routine at offset -744 of the code referenced by a global variable and returns.
- `apply_relocation_table_offsets`: Loads a base offset from a global variable, adds it to each of three other global values, then reads a longword from a table and adds the base to every longword it addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_dynamic_program_sequence`: Sets a global variable and a memory location, calls another routine, then enters a loop that repeatedly updates a memory location and calls routines via offsets from a global variable.
- `execute_indirect_calls`: Loads a pointer from a global variable, calls the routine at the address found by adding an offset of negative 330 to that pointer, and repeats this process with several other offsets and values stored in the same structure.
- `loop_set_background_dark_grey`: Repeatedly writes the value 4095 to the COLOR00 hardware register in an endless loop that branches back to its own start.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `loop_set_background_magenta`: Repeatedly writes the value 3855 to the COLOR00 hardware register in an endless loop that branches back to its own start.
- `tiny_rte_2`: Small routine of 1 instruction that returns from the exception.
- `execute_trap_and_loop`: Copies a 32-bit value from a global variable to an address pointed to by another global variable, pushes two values onto the stack, executes a trap, adjusts the stack pointer, sets the status register to a specific value, and branches to itself.
- `process_global_state`: Increments several values in a structure pointed to by a global variable, and if a counter reaches zero, it enters a loop that masks bits 2184 of ten consecutive words in a structure before calling another routine.
- `process_bit_rotation`: Calculates a bit-rotated version of a global variable and, if the result differs from the original, updates the global variable and enters a loop that repeats the assignment five hundred times.
- `call_indexed_routine`: Disables interrupts, shifts the value in d0 left by two, adds it to a base address, and calls the routine located at the resulting address before returning from an interrupt.
- `clear_memory_blocks`: Calculates a loop count based on a 16-bit value in register d1, then repeatedly clears specific offsets within a structure pointed to by a global variable until the counter reaches zero.
- `calculate_signed_difference`: Saves registers to the stack, moves the constant 7968 into register a2, writes the value 65535 to an offset of a global variable, and performs a series of arithmetic operations involving a subtraction and comparisons to modify register d2 before branching.
- `apply_bitmask_to_buffer_2`: Calculates a bitmask based on an index and a value from a table, then enters a loop that performs bitwise AND operations to clear specific bits in a destination buffer using 16-bit words.
- `apply_bitmask_to_buffer`: Performs a bitwise AND operation using a calculated mask to update 16-bit words in a destination buffer.
- `calculate_signed_difference_2`: Saves registers to the stack, writes a constant to a global variable, and calculates the absolute difference between a value in a register and a global variable.
- `update_bitmask_buffer_masking`: Calculates a bitmask based on a value in a register and a global variable, then enters a loop that performs bitwise rotations and logical AND/OR operations on a list of 32-bit values to update a buffer.
- `update_bitmask_buffer`: Iterates through a list of 32-bit values, performing bitwise operations including rotations and logical AND/OR combinations to update a buffer of 16-bit words.
- `store_d1_to_global`: Stores the full 32-bit value of register d1 into a global variable and returns.
- `copy_memory_block`: Copies a block of long words from a source pointed to by a global variable to a destination pointed to by another global variable, using a count from a global variable.
- `initialize_memory_range`: Sets three global variables to specific memory addresses and clears a block of memory at the third address using a loop of 8192 iterations.
- `configure_hardware_registers`: Writes specific values to several hardware registers, including bit manipulation on a register read from a hardware address, and stores a value from a hardware address into a global variable.
- `toggle_and_add_constant`: Toggles the least significant bit of a global variable and then adds 48001 to it.
- `transform_and_store_data`: Reads a word from a list in a0, doubles it, adds 273, and stores the result into a series of words at a hardware register address, repeating this 16 times.
- `expand_compressed_bit_packed_data`: Reads data from a source table and expands it into four separate destination buffers by performing bitwise right shifts based on a calculated shift amount.
- `expand_bit_shifted_word_data`: Calculates a destination address and bit shift amount based on an input value, then uses nested loops to read words from a source table, apply a bit shift, and distribute the results into four separate memory buffers.
- `distribute_bit_shifted_word_data`: Calculates a bit shift amount based on a value in a register and uses nested loops to read words from a source table, apply the bit shift, and distribute the resulting values into four separate memory destinations at large offsets.
- `distribute_word_data`: Performs a nested loop to distribute word data from a source into four different memory destinations with increasing offsets.
- `distribute_byte_data`: Performs a nested loop to distribute byte data from a source into four different memory destinations with increasing offsets and step increments.
- `update_pixel_buffer`: Performs a nested loop to bitwise combine values from a source table into a destination buffer at an offset determined by a global variable and register values, while clearing specific word offsets within the loop.
- `apply_plane_masks`: Multiplies a 16-bit value in d3 by 36, then calls a routine twice using two different pointers from global variables to apply bitwise masks to memory planes.
- `apply_bitwise_mask_to_planes`: Iterates through a data structure to perform bitwise AND and OR operations on four separate memory planes using values from a source list and a calculated mask.
- `apply_bitwise_mask_to_planes_2`: Iterates through a source list of four-byte data sets to perform bitwise AND and OR operations on four separate memory planes.
- `execute_disk_io`: Sets up a structure with a command, a count, and two buffer addresses, then calls a function via a pointer in a global variable to perform an I/O operation.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_do_i_o_2`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `setup_io_request`: Sets a 16-bit value at offset 28 of a global variable, writes 3 to offset 1c of a structure pointed to by a global variable, sets offsets 24 and 2c of that structure to 512 and 1024 respectively, and calls another routine.
- `compare_null_terminated_strings`: Compares two strings pointed to by a0 and a1, returning 65535 if a mismatch occurs or if the loop counter reaches -1, otherwise it returns a 16-bit value composed of the last two bytes read.
- `iterate_string_table`: Iterates through a table of pointers, calling a routine to compare two strings at each entry, and returns -1 if a null pointer is encountered, a mismatch occurs, or the loop counter reaches -1.
- `update_structure_values`: Updates several fields in a structure pointed to by a global variable using values from other offsets within that structure, calls a routine via a pointer, and may recursively call itself if a specific value is found at a global variable.
- `setup_io_request_scaled`: Sets offset 28 of a global variable to itself, writes 3 to offset 1c of a structure pointed to by a global variable, and sets offsets 24 and 2c of that structure using values from offsets 0e and 0c of a global variable multiplied by 512, then calls another routine.
- `process_offset_table`: Reads a value from a calculated pointer, then enters a loop that adds a constant to values in a table and clears bytes, using the cleared byte to determine the next offset to jump to until a zero byte is encountered.
- `clear_io_request`: Writes 9 to offset 1c of a structure pointed to by a global variable, clears the 32-bit value at offset 24 of that structure, and calls another routine.
- `process_io_sequence`: Calls a routine to return a value to a global variable, then calls another routine to check a string table; if the result is non-negative, it updates a global variable and calls a third routine before finally calling a fourth routine to clear an I/O request.
- `initialize_disk_io_buffer`: Stores a pointer from a register into a global variable, calls another routine, and if a global flag is zero, clears a 511-byte buffer starting at a specific memory location and sets a value at offset 3 of that same location before returning.
- `copy_string_and_setup_io`: Sets up an I/O request, then if a value from a routine is negative, it copies a string from one location to another using a loop; it then updates a 16-bit value at an offset of 14 in a global variable and calls three separate routines.
- `check_io_status`: Calls another routine and, if the result is non-zero, calls a second routine, then clears a global variable and returns a 16-bit value derived from a table lookup based on the global variable's previous value.
- `poll_io_character`: Repeatedly calls another routine and, if the result is non-zero, calls a second routine, then loops until a non-zero character is retrieved from a table using a value from a global variable as an index.
- `apply_font_mask_to_buffers`: Iterates through a string of characters, looking up each character in a font table and applying its bitmask to four separate memory buffers using bitwise OR or AND operations based on flags in a register.
- `return_word_global`: Loads the 16-bit word from a global variable into d0 and returns, leaving the upper half of d0 unchanged.
- `store_registers_to_globals`: Stores the low word of d1, the full a1 register, and the a2 and a3 registers each into a separate global variable, then returns.
- `store_constant_pointer_global`: Writes the constant 51348 as a 32-bit value into a global variable and returns.
- `load_globals_into_d0_a0`: Loads a 32-bit global variable into d0 and another 32-bit global variable into a0, then returns.
- `split_register_to_globals`: Writes the low word of d1 into two consecutive global variables, swaps the halves of d1, and writes the new low word into a third global variable before returning.
- `execute_complex_table_updates`: Updates a global variable by shifting a value, then enters a series of nested loops that call other routines to process data from multiple tables based on offsets and bitwise masks.
- `dispatch_indexed_jump_table`: Adds 8 to a0, fetches a code address from a table at a fixed base using d7 as a word index, and jumps to that address.
- `copy_structured_data`: Copies 32 longwords from a source pointer in a1 to a destination pointer in a0 at various fixed offsets.
- `process_nested_data_tables`: Performs nested loops to call other routines while manipulating data in a table, and optionally writes a constant to a pointer retrieved from a table before restoring its original value.
- `dispatch_nested_table_entry`: Fetches a code address from a table at a fixed base using the word index in d7 and jumps to it.
- `scatter_bytes_to_table`: Reads bytes from a source buffer pointed to by a1 and writes them to specific, non-contiguous offsets within a destination table pointed to by a0.
- `dispatch_second_table_entry`: Fetches a code address from a second table at a fixed base using the word index in d7 and jumps to it.
- `scatter_data_bytes`: Reads bytes from a source buffer pointed to by a1 and writes them to non-contiguous offsets within a destination buffer pointed to by a0.
- `store_a0_global_masked_interrupts`: Sets the status register to 9984, masking interrupts, stores the 32-bit value of a0 into a global variable, and returns.
- `initialize_system_resources`: Performs a sequence of system calls to load data from memory locations, clear registers, and call a subroutine at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_and_store_address`: Stores register d1 into a global variable, calculates a value by adding three offsets from a table in a5 to register d0, adds a constant to d0, stores the result in a global variable, calls another routine, and sets register a0, d2, and d1 to specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_trap_sequence_and_call_subroutine`: Executes a series of software interrupts and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_call_routines`: Stores the constant an address in a global variable, calls another routine, executes a software trap, and calls a final routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_traps_and_update_global`: Performs several traps, calls another routine, adds 1024 to a value derived from a register and stores it in a global variable, then calls two more routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_with_offset`: Adds 1024 to a value in a register and stores it in a global variable, then calls another routine, performs a trap, calls another routine, and performs a trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_from_function_result`: Performs two traps, calls another routine, performs two more traps, calls another routine to get a value, adds 1024 to that value and stores it in a global variable, calls another routine, performs a trap, calls another routine, and performs a trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines_and_traps`: Calls another routine, performs a trap, calls another routine, and performs a trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_trap_sequence`: Executes a sequence of traps and calls to other routines, including a loop that continues until a specific value is returned from a routine, and performs a busy-wait loop on a global variable.
- `load_code_and_trap`: Loads the value 10 into d0, executes trap 3, and then returns when the value in d0 is not zero, otherwise it returns after the trap.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `convert_digit_to_ascii_and_trap`: Adds 48 to a byte in a register, stores the result in a global variable, executes four traps, and calls itself recursively.
- `execute_multiple_traps`: Executes six traps and returns the value 2.
- `check_global_and_branch`: Checks a global variable; if it is zero, it calls another routine, otherwise it calls another routine, then returns.
- `save_registers_call_then_set_flag`: Saves all data and address registers on the stack, calls the routine at a fixed offset, then stores the word value 1 into a global variable at address 52320 before restoring the registers and returning.
- `branch_to_middle_routine`: Unconditionally branches to a location 2448 bytes further into the code, so execution continues in another routine's body.
- `branch_to_later_code`: Unconditionally branches to a location 2712 bytes further into the code, transferring control out of this routine.
- `branch_to_later_block`: Unconditionally branches to a location 1164 bytes further into the code, transferring control to another block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_word_loop_until_greater`: Ors the byte in d1 with 252, then loops back to itself while the word in d0 is not greater than a 16-bit global variable, and returns once it is greater.
- `store_ascii_and_trap`: Adds 48 to a byte from d0, stores the result in a global variable, and executes four traps.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_hardware`: Updates a structure with values from a pointer, writes to custom hardware registers including DMA and interrupt control, performs calculations involving a table and several offsets, and calls two routines to modify pointer data and update structure values.
- `reset_audio_and_dma_registers`: Writes constant values to several global or memory-mapped registers, including audio-related registers and a DMA control register, and clears several other global or memory-mapped variables.
- `enable_dma_channels`: Sets bit 15 of a DMA control register and calls another routine four times.
- `test_word_at_offset_six`: Tests the 16-bit word at offset 6 of the structure in a3 and, if it is non-zero, branches to a later instruction at a different location; otherwise it returns.
- `decrement_value_at_pointer`: Calculates an address using a pointer from a structure, reads a 16-bit value from that address, and if the value is not zero, decrements it by 4 or sets it to zero if the result would be negative, then writes it back.
- `modify_pointer_data`: Decrements a 16-bit value at an offset of 6 from a pointer in a3, and if that value reaches zero, it performs bitwise negation on it and writes data to a memory location calculated from a pointer in a2.
- `modify_memory_and_clear`: Writes 32768 to an offset of 6 from a pointer in a3, moves a 16-bit value from an offset of 14 from a2 into d0, adds d0 to an offset of 12 from a3, and if the result is not negative, clears the 16-bit value at an offset of 12 from a3 before returning.
- `update_structure_and_registers`: Decrements a 16-bit value at an offset of 12 from a pointer in a3; if the value reaches zero, it performs bitwise operations and writes values to offsets 156 and 154 of a structure pointed to by a6.
- `decrement_counters_and_flip_sign`: Decrements two short counters stored in a structure pointed to by an argument register and accumulates values from other fields of the same structure into a running total.
- `initialize_data_structures`: Calls a routine to reset audio and DMA, sets several values at offsets 154, 156, and 158 of a structure in a6, sets bit 1 of a hardware register, and uses a value from the stack to index into a table to update four structures with new pointers and byte values before performing a loop that writes bytes to a memory area.
- `parse_audio_command_stream`: Iterates through a command buffer to parse control bytes, updating audio hardware registers and calculating sample offsets and volume levels using a lookup table.
- `process_audio_command_stream`: Iterates through a command buffer to parse control bytes, updating structure offsets and hardware audio registers.
- `update_structure_data`: Compares a byte from a pointer in a3 to several constants; it either writes a byte to offset 10 of a structure in a4, writes a byte to offset 12 of that structure, or performs a complex calculation involving a table and a pointer to update a 32-bit value at offset 7 of the structure.
- `modify_byte_conditionally`: Checks a byte at offset 32 of a structure in a4, masks it with 63, and if the result is not negative, branches to another routine; otherwise, it clears bit 7 of the byte at offset 32 of the structure in a4 and returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_saved_registers_and_return`: ORs the low byte of d1 with the constant 206 (decimal), then pops the registers d0 through d7 and a0 through a6 from the stack and returns to the caller.
- `update_global_byte_with_traps`: Compares a byte in a register to a global variable and, if they differ, updates the global variable and enters a loop that calls a trap and clears two bytes of the global variable until it matches itself.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_and_shift_interrupt_mask`: Reads the interrupt mask from a hardware register, masks it with a constant, writes the result to the interrupt enable and mask registers after a logical shift right, and returns.
- `fill_hardware_registers`: Runs a loop that writes the value an address to offset 384 of a structure in a6, decrementing the value until it reaches zero, then runs a second loop that decrements d0 from an address to zero without writing to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_bit_manipulation_and_traps`: Executes a nested loop to perform bitwise shifts and transfers data to a memory buffer, then performs a sequence of trap calls and writes values to a global variable before filling a memory range with the value thirty-two.
- `clear_memory_block`: Runs a loop 19 times to clear 3-byte words starting at a memory address in a0, then writes a byte with value 255 to the first byte of that address and returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_word_and_trap_six`: Loads a 16-bit global variable into d2, sets d0 to 6, executes trap 3, and returns with 6 in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_interrupt_and_jump`: Checks a bit in the interrupt mask and interrupt request registers and, if the fourth bit is set, calls a routine at a fixed address.
