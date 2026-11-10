# Game 353 (Addams Family, The): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00353). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907088 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (0 by rule, 81 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 624 of 624 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 950 of 950 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 134114 |
| code | 3064 |
| data | 769902 |
| padding | 8 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Addams Family, The.adf:bootblock | 1 | 48 | 5 |
| logged disk load from Addams Family, The.adf boot-block load | 1 | 221 | 21 |
| logged disk load from Addams Family, The.adf boot-block load | 1 | 196 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 485 | 20 |

## Cases handled

- Addams Family, The/Addams Family, The.adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- Addams Family, The/Addams Family, The.adf: the boot block reads 159744 bytes from disk offset 17408 to an address (found statically); recovered as a unit
- Addams Family, The/Addams Family, The.adf: the boot block reads 9728 bytes from disk offset 17920 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Addams Family, The/Addams Family, The.adf: 2 logged loads cover 159744 bytes of the image
- Addams Family, The/Addams Family, The.adf: blocks 2-33 (bytes 1024-17408) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.13)
- 1 packed track region(s), 723968 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_and_main_loop` (runtime segment an address): Clears two global longwords, performs a series of initialization calls, and configures hardware registers including DMA and interrupts. Kind: disk hardware/track check. Pass path: The loop continues and increments DAT_00021f58/DAT_00021f64 if DAT_00021f58 != -2.

## Functions


### Addams Family, The.adf:bootblock

- `initialize_hardware_and_io`: Calls another routine, clears 32 words starting at the color register, saves a pointer from the stack, calls a routine via a pointer from a global variable to set a5, sets several fields in a structure pointed to by a1 using a5, calls another routine twice via a pointer from a global variable, calls a third routine via a pointer from a global variable, sets the status register, and calls two more routines.
- `save_registers_and_update_pointer`: Saves d0 and a0 to the stack, pushes a longword from an offset of 16 into the stack, updates a pointer at offset 16 with the address of the next instruction, and sets d0 to 8.
- `adjust_stack_and_return`: Adds 4 to the stack pointer at offset 2 of a6 and returns from an interrupt.
- `call_subroutine_and_jump`: Calls a subroutine at a fixed address and then performs an indirect jump using the address stored in a6.
- `perform_dma_io_operation`: Sets parameters in a structure pointed to by a1, calls a routine via a pointer from a global variable, calls a subroutine at address an address, and writes 384 to the bitplane register at offset 248 of the hardware registers.

### logged disk load from Addams Family, The.adf boot-block load

- `copy_data_and_jump`: Compares a0 to 4096, and if not equal, loads a pointer into a0, sets a1 to an address, sets d0 to 63, loops to move words from a0 to a1, and then jumps to the address in a1.
- `copy_bytes_backward`: Compares two pointers and, if the first is not less than the second, enters a loop that copies bytes from the end of the source and destination ranges toward the beginning using a counter in d0, then jumps to a new address.
- `copy_bytes_and_jump`: Copies bytes from a source pointed to by a0 to a destination pointed to by a1, decrementing a counter in d0 until it reaches zero, then jumps to another routine.
- `configure_hardware_and_copy_data`: Sets several bytes in d0, writes to the hardware registers at offset 244 and 240 of the custom chip base address, writes three bytes to a global variable and sets bit 1 of a value at offset 3841 of another global variable, then copies 325 bytes from a source to a destination using a loop and jumps to an address stored in a2.
- `copy_and_decompress_data`: Copies a block of data from one memory location to another, handling overlapping ranges by either copying forward or backward.
- `decrement_and_jump_indirect`: Decrements the word in d6, moves the result to d3, loads a constant into a6, sets d4 to 8, and jumps to the address in a4.
- `push_d6_and_loop`: Stores the byte from d6 on the stack at the location pointed to by a2, loops based on the value in d3, loads a constant into a6, and branches to another location.
- `test_d6_and_jump_2`: Tests the byte in d6 and branches if it is 0, otherwise it branches to another location, otherwise it sets d4 to 3, loads a constant into a6, and jumps to the address in a4.
- `setup_decompression_parameters`: Adds 6 to d6 and branches; otherwise, it loads a constant into d4, loads a constant into a6, and jumps to the address in a4; or it loads a constant into d4, branches, loads three pointers from a table into a0, a1, and a2, sets d7 to 0, loads a value from the pointer in a0 into d5, loads two addresses into a4 and a5, sets d2 to 0, loads a constant into a6, sets d4 to 3, and jumps to the address in a4.
- `branch_on_d6_value_2`: Compares the byte in d6 to 6 and branches if it is less than 6, otherwise it branches if it is equal to 6, otherwise it loads a constant into a6 and jumps to the address in a5, otherwise it loads a constant into a6 and jumps to the address in a5.
- `branch_on_d6_value`: Compares d6 to 2 and branches if it is less, otherwise it sets d4 to 1, loads a constant into a6, and jumps to the address in a4 if d6 is exactly 2.
- `branch_with_register_setup`: Sets d4 to 2 and jumps to the address in a4, otherwise it sets d3 to 3 and branches to a nearby location, or sets d3 to 1, tests d6, and branches based on the result.
- `branch_on_byte_test`: Tests a byte in d6 and branches to another location if it is zero; otherwise, it loads a pointer into a6 and branches to another location.
- `branch_on_byte`: Tests a byte in d6 and branches to one of two different locations based on whether the byte is zero.
- `compare_and_jump_indirect_2`: Compares the byte in d6 to 3 and branches if it is less than 3, otherwise it loads a constant into a6 and jumps to the address stored in a5.
- `compare_and_jump_indirect`: Compares d6 to 3 and branches if it is less, otherwise it loads a constant into a6 and jumps to the address in a5.
- `compare_and_jump`: Compares d6 to 3, and if it is less than 3, branches to a nearby location; otherwise, it sets d4 to 8, d3 to 10, loads a constant into a6, and jumps to the address in a4.
- `add_and_branch`: Adds a word in d6 to a word in d3, loads a new address into a6, and branches to another location.
- `test_d6_and_jump`: Tests d6, and if it is not zero, branches to a nearby location; otherwise, it sets d4 to 7, d2 to 2, and branches to a nearby location, or sets d4 to 8, d2 to 130, loads a constant into a6, and jumps to the address in a4.
- `copy_and_branch_by_count`: Copies bytes from a source to a destination using a loop controlled by a count in d3, then branches to a location based on the value in d6.
- `decompress_and_store_data`: Decompresses a data stream using a custom algorithm involving byte comparisons and length-based expansions, storing the results into a structure pointed to by a6.

### logged disk load from Addams Family, The.adf boot-block load

- `copy_longword_block_loop`: Compares a source address in a0 with a destination address loaded from a PC-relative offset, and if the source is not less than the destination, copies longwords from the source to the destination in a loop until a counter reaches zero.
- `copy_longword_block`: Copies a block of memory of a size specified by a register from a source address to a destination address using a loop that decrements the count by 4 each iteration.
- `decrement_and_jump_to_table`: Decrements a value in a register, moves that value to a counter register, and performs an indirect jump using a table located at a relative offset.
- `store_byte_and_loop`: Stores a byte from register d6 at the address pointed to by register a2, then decrements register d3 and branches if it is not zero, otherwise it loads a constant into register a6 and branches to another routine.
- `test_d6_and_branch_3`: Tests the byte in d6 and branches if it is 0, otherwise it branches to another location.
- `set_register_and_jump`: Sets d4 to 3, loads a constant into a6, and jumps to the address in a4.
- `increment_and_branch`: Adds 6 to a word in d6 and branches to another location.
- `transfer_control_via_jump`: Loads the value ten into d4, sets a6 to a location relative to the program counter, and jumps to the address in a4.
- `set_d4_and_branch`: Sets d4 to 16 and branches to a nearby location.
- `initialize_memory_move_parameters`: Loads a source pointer from a global variable, calculates a destination address by adding an offset to a second global variable, and performs an indirect jump to a routine that manages the iterative memory move process.
- `compare_and_jump_indirect_4`: Compares a byte in d6 to 6 and branches if it is less than or equal to 6, otherwise it branches if it is equal to 6, or loads a relative address into a6 and performs an indirect jump using the address in a5.
- `jump_indirect_via_a5`: Loads a constant into a6 and jumps to the address stored in a5.
- `test_and_branch_indirect`: Compares the byte in d6 to 2 and branches if it is less than 2, otherwise it branches if d6 is equal to 2, or it loads a constant into a6, sets d4 to 1, and jumps to the address stored in a4.
- `jump_via_a4`: Loads the value 2 into d4 and jumps to the address contained in a4.
- `set_d3_and_branch`: Sets d3 to 3 and branches to a nearby location.
- `test_d6_and_branch_2`: Sets d3 to 1 and branches if d6 is not zero, otherwise it branches to a nearby location.
- `load_a6_and_branch`: Tests d6 and branches if it is zero, otherwise it loads a constant into a6 and branches to a nearby location.
- `test_d6_and_branch`: Tests d6, and if it is zero, branches to a nearby location, otherwise it branches to another nearby location.
- `branch_if_d6_is_less_than_3`: Compares the byte in d6 to 3 and branches if it is less than 3; otherwise, it loads a constant into a6 and calls the address in a5.
- `compare_and_jump_indirect_alt`: Compares the byte in d6 to 3, and if it is not less than 3, loads a constant into a6 and jumps to the address in a5.
- `compare_and_jump_indirect_3`: Compares the byte in d6 to 3, and if it is not less than 3, loads a constant into a6 and jumps to the address in a4.
- `add_d6_to_d3`: Adds the word in d6 to the word in d3, loads a constant into a6, and branches to another location.
- `test_d6_and_branch_logic`: Tests d6, and if it is not zero, branches to a nearby location; otherwise, it sets d4 to 7, d2 to 2, and branches to a nearby location.
- `prepare_jump_parameters`: Sets d4 to 8, d2 to 130, loads a constant into a6, and jumps to the address in a4.
- `perform_iterative_memory_move`: Adds d2 to d6, sets a6 to the sum of a2 and d6, decrements a6 and a2 while moving a byte from the memory at the new a6 to the memory at the new a2, and loops using d3 until it branches to another location.
- `set_d3_and_branch_2`: Sets d3 to 7 and branches to another location.
- `set_d3_and_branch_alternative`: Sets d3 to 4 and branches to another location.
- `set_d3_based_on_d6`: Sets d3 to 2 and branches if d6 is less than 1, otherwise it sets d3 to 3 and branches to a nearby location if d6 is not 1.
- `clear_d6_and_branch`: Sets d6 to 0 and branches to another location.
- `shift_and_loop_jump`: Shifts the longword in d5 left by 1 bit, branches if the result is zero, otherwise it performs a rotate left through carry on the word in d6 and executes a loop using d4 that jumps to the address in a6.
- `store_longword_and_branch`: Compares a1 to a0 and branches if they are equal, otherwise it loads a longword from the memory pointed to by a0, stores it at an offset of 384 from a3, and branches to a nearby location.
- `decompress_and_branch`: Copies a block of bytes from one memory location to another, then processes a source buffer by comparing bytes against two constants and storing results into a destination buffer or a structure offset by 384 from a6.
- `store_and_jump`: Stores a byte from d0 to the address pointed to by a2 decremented by 1, loads a new address into a2 from a table, writes 33280 and 49152 to offsets 150 and 154 of the structure in a4, then jumps to the address in a2.
- `extract_and_store_bytes`: Moves two bytes from the memory at a0 into d2 and d1, stores d1 at a memory location offset by 384 from a4, and loops using d2 until it branches to another location.
- `extract_byte_and_branch`: Clears d1, moves a byte from the memory pointed to by a0 into d2, and branches to a nearby location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_main_loop`: Clears two global longwords, performs a series of initialization calls, and configures hardware registers including DMA and interrupts.
- `update_global_tables`: Performs multiple calculations involving adding 2800 to a value in d0 and writes the resulting high and low words to various global memory locations, including setting a long value at a global address and a word value at another.
- `initialize_memory_block`: Sets a global word and a byte, then loops to fill a block of memory with a specific word value.
- `initialize_memory_block_alt`: Sets a global word and a byte, then loops to fill a block of memory with a specific word value at a different starting address.
- `increment_table_values`: Loops through a table, checking if a byte at offset 1 is 223, and if so, sets it to 56, then increments the bytes at offsets 1, 9, and 17 by 1, 2, and 3 respectively.
- `handle_interrupt_request`: Saves registers, checks if bit 5 of the interrupt request register is set, and if so, calls two different routines and writes a value to the interrupt mask register before returning from interrupt.
- `load_addresses`: Loads the address of a global variable into a0, loads the address of another global variable plus 2310 into a2, and sets d7 to 44.
- `copy_data_from_table`: Reads a byte from a0, uses it to index into a table to find a source address, and then copies eight bytes from that source to a destination pointed to by a2 at specific offsets, looping until a zero byte is encountered or a counter in d7 reaches zero.
- `test_bit_and_loop`: Tests bit 14 of a byte at offset 2 of the structure in a5 and loops if the bit is set.
- `increment_global_pointer`: Compares a global long value to an address and, if they are not equal, increments the global value by 2.
- `initialize_structure_fields`: Calls another routine, sets a longword at offset 68 of a structure in a5 to all ones, sets a word at offset 64 to 2544, clears a word at offset 66, clears two words at offsets 100 and 102, sets a longword at offset 80 to an address, sets a longword at offset 84 to the value of a global variable, and sets a word at offset 88 to 534.
- `update_structure_and_copy_bytes`: Sets several longwords and a word in a structure pointed to by a5, decrements a global variable, and if that variable reaches zero, it performs a series of memory copies from a table to a destination structure based on values retrieved from a pointer.
- `store_pointer_to_global`: Stores the constant value a fixed address into a global variable.
- `update_pointer_address`: Writes the address of a specific memory location into a longword at a global variable address and returns.
- `copy_data_from_table_2`: Performs a triple nested loop to copy blocks of 8 bytes from a source table to a destination table, where the source address is determined by a value in a global variable.
- `process_keyboard_input_sequence`: Reads a byte from the keyboard serial register, compares it against several constant values, and for each match, clears a global variable, calls another routine with a specific pointer in a0, sets a global variable to either one or 255, and clears a byte at a specific offset in a series of global variables.
- `clear_status_and_call`: Clears a byte at a global address, loads a pointer into a2, checks a byte at another global address, and if not zero, calls another routine, sets a byte at a global address to 1, clears a byte at a global address, and returns.
- `reset_status_and_call`: Loads a pointer into a0, calls another routine, clears a byte at offset 534 of a global variable, sets a byte at offset 198 of a global variable to 255, and returns.
- `copy_memory_block`: Loads three pointers into a0, a1, and a2, moves a byte from the memory at a0 to the memory at a1 while incrementing both, and loops until a0 matches a2.
- `write_constant_values`: Writes two identical longword and word pairs to two different global variable locations.
