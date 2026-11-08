# Game 133 (Overkill v1.1 AGA 0707): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00133). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1843933 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 46 raw-word lines |
| Functions named and described | pass, 264 of 264 named (0 author symbols), 264 of 264 described (19 by rule, 245 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1636 of 1636 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 20880 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5957 of 5959 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 935548 |
| code | 22736 |
| data | 885632 |
| padding | 17 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| OverkillAGA.slave | 1 | 80 | 9 |
| Disk.1:bootblock | 1 | 87 | 4 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 5730 | 240 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 55 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.1: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.2: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 10 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Overkill_v1.1_AGA_0707/OverkillAGA/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.1: 4 logged loads cover 515584 bytes of the image
- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.1: blocks 1007-1759 (bytes 515584-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.60)
- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.2: 6 logged loads cover 449536 bytes of the image
- Overkill_v1.1_AGA_0707/OverkillAGA/Disk.2: blocks 880-1759 (bytes 450560-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.71)
- 1 never-loaded region(s), 20880 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### OverkillAGA.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_jump_table`: Stores a pointer in a global variable, calls a routine via a pointer in a2, calls another routine via a pointer in a3, writes a byte to a hardware register, calls a library function, and performs an indirect jump through a pointer.
- `setup_jump_table`: Stores several longwords and a word into global variables and calls a routine before performing an indirect jump through a pointer in a1.
- `update_structure_fields`: Stores a 16-bit word from d0 at offset 28 of a structure in a1, stores a 32-bit long from a0 at offset 40 of the structure, and then stores a 32-bit long from a memory location into offset 24 of the structure.
- `decrement_and_store_byte`: Subtracts 49 from a byte in d0, stores the result as a 32-bit long into a global variable, and compares a value at a global address with d0 before returning.
- `dispatch_indirect_handler_pair`: Calls one routine, then calls through a pointer at offset 36 of a structure in a2, and if that returns non-zero, calls another routine and the pointer at offset 8 of that same structure.
- `load_text_pointers`: Loads three memory addresses into registers a0, a1, and a2 from locations relative to the program counter, including a pointer to a string in the text file, and then returns.
- `load_and_jump_indirect`: Stores a longword from a global variable into a location pointed to by a0, then calls one routine, sets d0 to 272, calls through a pointer at offset 12 of a structure in a2, and finally performs an indirect jump through a longword at a global address.
- `call_function_and_check_result`: Calls a function via a pointer in a table with d0 set to an address, then compares the result in d0 to a constant and returns either the result or a value from the stack.

### Disk.1:bootblock

- `initialize_and_execute_jump_table`: Calls two routines and two library functions, then enters a loop that modifies a jump table by updating entries with calculated offsets before performing an indirect jump through a value in the table.
- `initialize_device_and_memory`: Allocates memory and a signal, stores the resulting pointers in global variables, populates a structure with various values including a pointer and a byte, and calls a library function.
- `execute_library_sequence`: Calls a routine, then calls two different library functions using pointers from a global address, and finally performs an indirect jump through a value in a6.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.

### logged disk load from Disk.1

- `branch_to_address`: Branches to a relative address.
- `store_word_to_global`: Stores a 16-bit word from d0 into a global variable and returns.
- `calculate_scaled_value`: Subtracts 64 from a value in d0 if it is greater than 64, multiplies the result by a constant from a table, and stores the result in a 16-bit word at offset 8 of the structure in a1.
- `load_pointers_and_call`: Loads a pointer from a table into a2, calls another routine, loads a pointer from the same table into a0, and then branches to another location.
- `call_stub_2`: Performs a branch to a specific address.
- `relocate_pointer_table_entries`: Walks a chain of records and, for each active one, adds the record's base address to a run of longword fields starting at a fixed offset, using a count stored in the record.
- `add_memory_block`: Checks if a pointer in a0 is null; if not, it adds the value in d1 to the memory at the address pointed to by a0 and increments the pointer by four, then returns.
- `process_character_string`: Checks if a byte at a pointer in a2 matches a specific value, replaces it with another byte if it matches, and then enters a loop that traverses a linked list of pointers, calling several routines to process data at each step.
- `add_multiple_blocks`: Calls another routine three times, incrementing the pointer in a0 by 4 after each call.
- `process_counted_array`: Loads a pointer from a table, and if it is not zero, it loads a word from that pointer, decrements it, and enters a loop that calls another routine until the counter reaches zero; it then checks a byte at an offset in a table before returning.
- `apply_helpers_to_counted_list`: Checks if a pointer is non-zero, then walks a counted array of entries by loading a word from the pointer, calling two helper routines for each entry, and decrementing the counter in a loop.
- `update_pointer_chain`: Checks if a longword at offset minus 4 of a pointer in a0 is non-zero, then either calls another routine with an incremented pointer or processes a stack of pointers using a count from a structure.
- `set_dma_mask`: Calculates a bitmask by shifting d0 left by 2, uses it to index into a table to fetch a pointer, and if d0 is less than 4, writes a value to the DMA controller register at offset 96.
- `loop_clear_register`: Saves d2 to the stack, sets d2 to fifteen, and enters a loop that calls another routine fifteen times using d2 as a counter, then clears a longword at a fixed memory location and restores d2 before returning.
- `update_sprite_display_data`: Checks a bitmask in a global variable and, if set, updates DMA controller settings and calculates sprite dimensions and memory offsets from a table.
- `process_sprite_list`: Iterates through a linked list of sprite structures, decrementing a counter in a global variable and updating sprite properties such as visibility, dimensions, and memory offsets.
- `update_sprite_attribute_and_offset`: Updates a sprite's attribute in a hardware register table based on a value in a register, then calculates a new memory offset for the sprite structure by multiplying a value from a structure by a constant and adding a byte from a register, storing the result at offset 2 of that structure.
- `subtract_and_store_byte`: Subtracts a word from a global variable and stores the result as a byte in a global variable at offset 769, then branches to another routine.
- `update_sprite_dma_settings`: Iterates through a list of sprite structures to update their dimensions, memory offsets, and visibility, then modifies the DMA controller register to enable specific channels based on a bitmask.
- `enable_dma_channels`: Sets bit 15 of the DMA controller register to enable DMA channels and calls calculate_scaled_value, wait_for_vblank, and write_pending_entry_and_shift.
- `wait_for_vblank`: Compares the value in the video hardware register at offset 6 with itself in a loop, otherwise it returns.
- `write_pending_entry_and_shift`: Shifts an input flag value right by one bit.
- `update_tables_and_buffers`: Multiplies a byte from a structure by another byte and stores the result in a table, then either clears a buffer or copies data from a source structure into a destination table based on specific conditions.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `clear_structure_field`: If a global variable is not zero, it clears a 16-bit word at an offset of 40 from that global variable and then sets the global variable to zero.
- `increment_and_mask_global`: Increments a 16-bit global variable, uses the lower 4 bits of that variable to index into a table, and returns the lower 4 bits of the byte found at that index.
- `update_global_with_bit_manipulation`: Performs arithmetic and bitwise operations on a value from a table and stores the result in a global variable.
- `combine_shifted_values`: Calls another routine twice, shifting the first result left by 4 bits and the second result left by 8 bits, then combines them using a bitwise OR.
- `generate_obfuscated_value`: If a word from a global variable is zero, it performs bitwise operations using mouse port data and two constants to calculate a new value, storing the results in two global variables; otherwise, it calls another routine twice and combines the results.
- `multiply_and_swap`: Pushes a word from d0 onto the stack, calls another routine, multiplies the result by the word on the stack, swaps the upper and lower 16 bits of the result in d0, and returns.
- `negate_conditional_d0`: Pushes a long value from d0 onto the stack, calls another routine, and if d0 is negative, restores the long value from the stack and negates it before returning; otherwise it restores the value and returns.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `clear_color_registers`: Calls a routine via a pointer from a global variable; if the result is zero, it enters loops to write specific values to the color registers at offset 288.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `fill_memory_with_pointer`: Clears a 4-byte value at offset 4 of a structure in a2, calls another routine to get a pointer, stores that pointer at offset 16 of the structure, and then enters a loop to fill memory starting at that pointer with the value 65538 in increments of 65538.
- `multiply_scaled_values`: Performs a logical shift right on d0 by 3, then multiplies the result by d2 and d1, and finally calls another routine.
- `calculate_and_free_size`: Multiplies d0 by d2 and then by d1, and calls another routine.
- `update_system_state`: Checks a bit in a hardware register and, if not set, modifies a global variable with a bitwise OR, writes to several hardware registers, copies a block of data from a null pointer to a table using a loop, and stores the stack pointer into a global variable before returning.
- `write_hardware_values`: Writes several 16-bit values to specific hardware registers and then branches to an earlier instruction.
- `process_data_and_call_library`: Writes to hardware registers, copies a block of data from a table to a destination, calls a function via a pointer from a library, and updates several hardware registers and a global variable based on the return value.
- `assign_global_values`: Sets a global variable to 8190 and then sets it to 136.
- `setup_memory_structure`: Calls two routines, branches to another routine, and then performs arithmetic on d0 and d1 to calculate an address in a2.
- `calculate_and_store_offsets`: Performs arithmetic on registers to calculate an offset, uses a multiplication to find a memory location, and stores several calculated 16-bit and 32-bit values into a structure pointed to by a register.
- `populate_structure_from_registers`: Stores three values from registers into a destination pointed to by a5, then, if a specific bit in a hardware register is not set, populates a large structure with various register and memory values.
- `calculate_and_populate_structure`: Performs complex arithmetic involving multiplication and bit shifts to calculate values, stores them in a destination, and then, if a hardware bit is not set, fills a structure with calculated data and register values.
- `calculate_complex_offsets`: Uses a table to calculate several values through multiplication and bit manipulation, then, if certain conditions are met, populates a structure with these values and updates hardware registers.
- `populate_structure_with_calculated_data`: Calculates a pointer based on a global variable and d0, then, if a specific bit in a hardware register is not set, populates a structure in a6 with various values including a bitwise ORed version of d3 and several pointers and constants.
- `update_entity_state_structure`: Calculates an offset into a table based on a global value and a register, then populates a structure pointed to by a6 with derived pointers, modified bitmasks, and updated coordinate or timing values.
- `update_pointer_and_structure`: Calculates a value using a bitwise rotation and a pointer, then, if a hardware bit is not set, populates a structure with calculated values and stores a pointer into a global variable.
- `update_structure_with_rotation`: Calculates a value using a bitwise rotation, then, if a hardware bit is not set, populates a structure with calculated values and updates a global pointer and hardware registers.
- `populate_structure_with_scaled_data`: Calculates a pointer based on a global variable and d0, multiplies d1 by 10, and then, if a specific bit in a hardware register is not set, populates a structure in a6 with various values including a bitwise ORed version of d3 and several pointers and constants.
- `traverse_and_relink_linked_list`: Iterates through a linked list starting from a global variable, checking if a structure's field at offset 34 matches a value in d3 and if a field at offset 36 is negative.
- `find_max_value_in_list`: Traverses a linked list of structures, comparing a value in d3 to a field at offset 22, and updates d1 with the value from offset 26 if it is greater than the current d1 and matches the comparison value.
- `update_status_byte`: Loads a pointer from a global variable and, if the pointer is not zero and the word at offset 22 of the pointed structure matches d3, sets bit 7 of the byte at offset 20 of that structure.
- `calculate_and_update_structure`: Performs two additions involving values from a structure and another routine, then calls another routine and, if the result is zero, writes three 32-bit values to offsets 68, 60, and 64 of a structure in a1.
- `calculate_differences`: Saves registers to the stack, increments a pointer by an offset stored at that pointer, and then subtracts two values derived from words at the new pointer location from d0 and d1 respectively before restoring registers.
- `update_linked_list_structure`: Initializes a structure in a0 with several constants and zeroed fields, then, if a global pointer is not null, performs a series of pointer swaps and updates a linked list structure.
- `add_constant_to_global`: Adds an address to the value in a global variable and returns.
- `update_max_memory`: Calls a routine at an offset from a global variable, then updates that global variable with the absolute difference between its current value and the returned value if that difference is greater than the current value.
- `reset_memory_tracker`: Calls a routine at an offset from a global variable, stores the result in that global variable, and then clears the global variable at a different address.
- `initialize_hardware_registers`: Performs several writes to hardware registers and memory locations, including bitwise operations on a register at offset a fixed address and setting values at offsets 156 and 154 relative to a6.
- `set_hardware_config`: Writes 8192 to an offset of a6, sets a byte at offset a fixed address to 31, and performs bitwise operations on a byte at offset a fixed address.
- `handle_interrupt_request`: Saves registers to the stack, checks bit 0 of a byte at offset a fixed address, calls a routine if the bit is set, writes 8192 to a hardware register, and returns from interrupt.
- `update_system_parameters`: Writes 158 to a global word, conditionally writes 114 to it and calls another routine, then performs several calls with varying d0 and a0 values to update different memory locations, and finally writes 34308 to two global longwords before branching to another routine.
- `store_and_call_subroutines`: Stores a longword and a byte from d0 into two global variables, calls two routines, and then either calls two more routines and branches, or calls two different routines, loads two constants into d0 and d1, and calls a routine with a pointer to a global variable before branching to another routine.
- `transform_table_data`: Calls three different routines, then iterates through a table of words, negating the value of 256 minus the current word and setting the following word to zero.
- `initialize_system_and_run_sequences`: Writes a constant 32-bit value to two global variables, performs several routine calls to initialize system state, and then executes multiple loops that call various routines based on decreasing counters and hardware status checks.
- `process_table_data`: Calculates a base address using a table and a multiplier, then enters a loop that performs several writes to a structure in a6, including values derived from d0 and d2, while checking a bit in a hardware register.
- `process_table_entries_in_loop`: Calculates a base address from a table and d1, then enters a loop that calls another routine multiple times with different d1 values and performs writes to a structure in a6 based on the results.
- `initialize_system_state`: Performs several routine calls to initialize various pointers and values, writes specific constants to hardware registers and global variables, and executes a series of conditional branches and routine calls based on the status of a global variable and the CIAA_PRA register.
- `calculate_offset_and_branch`: Multiplies a word from a global variable by 49, adds 59, sets d1 to 92, sets d3 to 28, and branches to another routine.
- `calculate_offset_and_branch_alt`: Multiplies a word from a global variable by 49, adds 59, sets d1 to 92, sets d3 to 2, and branches to another routine.
- `swap_buffer_bytes`: Swaps bytes between two memory locations using a loop that runs d0 minus one times.
- `perform_multiple_memory_copies`: If a global variable is not zero, calls another routine four times to copy different amounts of data between various memory locations, and finally negates the global variable.
- `update_data_structures`: Iterates through a table of four entries, copying data from a source to a destination structure in a6, and sets specific bits in the destination values.
- `update_interrupt_and_dma`: Saves registers, reads a value from a hardware register, masks it, writes the result to another hardware register, and enters a loop that modifies a structure and hardware registers based on the bits of the masked value.
- `execute_dbra_loop`: Stores a 32-bit constant and a 16-bit constant into a structure in a1, adjusts pointers in a0 and a1, and then enters a loop using dbra.
- `populate_table_from_source`: Loops four times to copy data from a source pointed to by a2 into a destination structure in a6, using values from a2 and d1.
- `clear_and_populate_table`: Loops four times to copy data from a source pointed to by a2 into a destination structure in a6, while clearing a specific word in the destination.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d6.
- `decompress_data_block`: Performs a nested loop to expand a compressed data block into a destination buffer, handling both standard and negative length byte expansions.
- `manipulate_global_variable`: If a global variable is zero, it sets it to 50; otherwise, it decrements it and, if the result is zero, calls another routine and performs several operations including setting the global variable to 0, incrementing it, and eventually setting it to 65535.
- `update_active_object_pointer`: Checks a global variable against a constant, then performs several subroutines to initialize data.
- `execute_multiple_frees`: Calls several routines to calculate sizes and free memory, then calls another routine with a pointer from a table in a2.
- `free_multiple_memory_blocks`: Calls several routines to manage linked lists and status, then performs multiple memory free operations by multiplying register values and passing them to a routine with pointers from various global tables.
- `update_game_state_and_hardware`: Checks a global value against nine, pushes a constant to the stack, and executes a sequence of subroutines based on the result.
- `initialize_and_branch`: Loads values into d0 and d1, sets a pointer in a2, calls another routine, and then performs a short branch.
- `clear_d0_register`: Sets the lowest byte of the d0 register to zero five times and then returns.
- `calculate_and_free_memory`: Calculates a value by multiplying a constant by 512, then calls a routine to free a memory block at the address stored in a global variable.
- `initialize_branch_parameters_2`: Loads 128 into d0, 24 into d1, loads a relative address into a2, calls another routine, and then branches to another location.
- `initialize_branch_parameters`: Clears the low byte of register d0 five times, sets d0 and d1 to 32, loads a relative address into register a2, calls another routine, and branches to another routine.
- `initialize_branch_data`: Clears the low byte of d0 five times, sets d0 and d1 to 32, loads a relative address into a2, calls another routine, and branches to an earlier instruction.
- `initialize_and_branch_data`: Sets d0 to 20, sets d1 to 28, loads a pointer into a2 from a relative address, calls another routine, and branches to an earlier instruction.
- `initialize_and_branch_2`: Sets d0 to 32, sets d1 to 1048, loads a pointer into a2 from a relative address, calls another routine, and branches to an earlier instruction in this routine.
- `populate_data_tables`: Calls another routine multiple times to fill various global memory locations with specific values, performs two loops to copy words from a source table to a destination table, executes a loop to mask bytes in a table, and conditionally writes values to global variables and hardware registers based on the result of a routine call.
- `process_and_copy_data_tables`: Clears a global word, executes two loops involving routine calls and decrementing a value in d7, then executes two nested loops to write zero bytes to a table and a conditional loop to copy bytes from one table to another.
- `write_alternating_words`: Loops fifteen times to write alternating 16-bit words from a table into a destination structure in a6, while toggling a value in a hardware register at offset 106 of a6.
- `initialize_object_structure`: Clears several global variables and populates a structure with specific values, including a pointer to a hardware register and several constants, then updates a global counter and calls three other routines before returning.
- `store_constant_pointer`: Stores the constant value an address into a global variable at offset 40092.
- `loop_structure_population`: Decrements a value from 139 in a loop, calling another routine with a calculated offset and a divisor-based value, and populates a structure if a global value is less than the counter.
- `update_list_elements`: Iterates through a list in a structure, calls another routine twice to calculate a value, stores that value at offset 4 and zero at offset 6 of the current element, and updates a counter in d6 before moving to the next element.
- `configure_hardware_and_globals`: Calls another routine, writes 13 to two global words, conditionally calls another routine, performs two calls to another routine to store results in global longwords, calls another routine with specific d0, d1, and d2 values, writes 34308 to two global longwords, calls another routine, writes 12692 to a global longword, and writes values to offsets 154 and 156 of a structure in a6 before branching to another routine.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d2.
- `execute_indirect_jump`: Performs an indirect jump using a table at a global address and an offset in d2.
- `write_and_jump`: Writes the byte 159 to a global variable and performs an indirect jump using a value in d2.
- `set_and_jump`: Sets d2 to 8, calls another routine, writes 31 and 136 to a global variable, and sets a6 to the start of the hardware registers.
- `prepare_jump_offset_twelve`: Sets d2 to 12 and performs an indirect jump.
- `prepare_jump_offset_sixteen`: Sets d2 to 16 and performs an indirect jump.
- `prepare_jump_offset_twenty`: Sets d2 to 20 and performs an indirect jump.
- `prepare_jump_offset_twenty_four`: Sets d2 to 24 and performs an indirect jump.
- `copy_table_blocks`: Calculates a destination address based on a global value shifted right by 3, and copies six blocks of data from a source table to that destination.
- `populate_structure_and_copy`: Checks a 16-bit value in a global variable and, if zero, increments two global variables, calls another routine, adds 10 to a global variable, calls another routine, performs four BCD adjustments using a table, and loops six times to copy data from a source table to a destination address derived from a global variable.
- `copy_multiple_blocks`: Calculates a destination address using a global value shifted right by 3, then performs a loop four times to call another routine with values from a table and subsequently copies six blocks of data.
- `copy_data_from_table`: If certain conditions are met, copies six 480-byte blocks from a table to a destination pointed to by a1.
- `clear_memory_block`: Fills a block of memory starting at a pointer from a global variable with zeros using a loop that runs 38815 times.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 8 into d0.
- `initialize_data_tables`: Writes values to global variables, calls another routine, calculates a value based on the result, calls another routine, fills a table with repeating patterns in a loop, and finally populates a table with eight entries derived from a global value.
- `toggle_and_call_routines`: Inverts a word in a global variable and, if the result is not zero, inverts it again, calls two other routines, and branches to another routine.
- `process_coordinate_list`: Compares three 16-bit values from a source list against three other 16-bit values, calculates differences, calls another routine, and writes the results to a destination list before updating a global variable.
- `invert_and_branch`: Inverts a 16-bit value in a global variable and, if the result is zero, returns; otherwise, it calls another routine, sets a value in a global variable, and if a byte at a global variable is non-zero, calls another routine and branches to another routine.
- `copy_scaled_table_entries`: Calls another routine and, if it does not return with a non-zero flag, increments a global variable and, if it is below a threshold, loops 127 times to copy two 16-bit words from a table into a destination structure at 32-byte intervals.
- `initialize_and_populate_object_table`: Populates a table with bit-masked values using a nested loop, updates a global variable with concatenated values from multiple calls, and clears a series of word-sized entries in a table.
- `transform_data_table`: Performs a nested loop to XOR and bitwise NOT values in a table, starting at an offset of 448 from a pointer in a global variable.
- `copy_structured_data`: Iterates through a buffer and copies 32-byte chunks from a source table to a destination table based on values read from the buffer.
- `update_pointer_table`: Copies a 32-bit value from one global variable location to another.
- `manage_linked_lists`: Initializes two global values, checks a bit in a structure at offset 2 of a6, and then iterates through several linked lists to clear specific offsets in the structures and re-link nodes.
- `test_and_call`: Sets the status flag based on a byte in a global variable, tests that byte, and if it is zero, calls another routine and branches to another routine.
- `manage_dynamic_memory_and_tables`: Clears several global variables and tables, then enters a complex loop that updates memory pointers, performs arithmetic on a global counter, and manages a linked list of structures.
- `update_table_data_and_branch`: Calls a routine twice with specific parameters to update two data tables, then performs a loop based on a value derived from a carry flag and a bit test on a hardware register.
- `process_table_and_flags`: Performs several calls to another routine with varying d0, d1, and a0 values, clears a global word, loads a longword into memory, calls two other routines, and enters a loop that tests bits in a table and performs conditional writes and calls based on a byte value.
- `branch_with_values`: Loads the value 144 into d0, the value 208 into d1, and branches to another routine.
- `invert_byte_flags`: Inverts four specific bytes at fixed offsets from a calculated address derived from a global variable.
- `tiny_move_move_ori`: Small routine of 3 instructions that copies -(a0) to d0, then copies d0 to d0, then merges 0 into d0.
- `search_and_shift_table`: Searches a table for a value and, if found, shifts subsequent table entries to update a pointer.
- `execute_complex_sequence`: Calls several routines, clears several words in a global variable, and performs a loop that calls another routine multiple times based on a counter, while also checking a bit in a hardware register.
- `process_circular_buffer`: Updates a counter in a global variable and copies words from a source table to a destination table using a loop.
- `expand_bit_and_byte_data`: Increments a global variable, calculates a destination address using a value in d1 and a global pointer, performs several BCD adjustments via another routine, loops seven times to write rotated bits from a byte into a table, and then loops through a byte stream, calling another routine if a byte is non-zero.
- `calculate_offset_address`: Reads a word from a table, performs bitwise OR operations, and calculates a new address in a1 using multiplication and bit shifting.
- `expand_byte_stream`: Reads a byte from a buffer pointed to by a0, and if the byte is not zero, calls another routine and loops back to the start.
- `expand_bit_data`: Iterates through a source table and writes bytes to a destination buffer based on the bits of a value from another table.
- `copy_and_setup_parameters`: Calls another routine, checks a global word, and if zero, calls two other routines, copies three bytes from one location to another, clears a byte at the destination, and calls another routine with d0 and d1 values before loading a global address into a0.
- `store_registers_to_globals`: Stores the contents of a0 and d0 into two global variables and sets two other global variables to a constant value.
- `call_external_and_update_registers`: Calls a routine through a pointer in a global variable, decrements that global variable, writes the value 32 to an offset of the hardware registers, and returns via an interrupt return.
- `calculate_address_and_branch`: Loads a word from a global variable, checks a second global variable, and if non-zero, increments the first word, uses it to offset a pointer in a3, performs arithmetic with two other global variables, and branches to another routine.
- `update_global_values`: Loads a longword from a global variable into d0, copies it to another global variable, adds d0 to a third global variable, masks that variable with 4095, and subtracts a value from d0.
- `process_linked_list_nodes`: Iterates fifteen times through a linked list, updating node pointers and clearing specific offsets, then calculates and stores three values in a structure using a routine that generates obfuscated data.
- `process_list_with_conditional_logic`: Iterates ninety-nine times through a linked list, performing calculations and storing results in a structure, but may exit the loop early or take a different branch based on a comparison and a bitwise check.
- `calculate_offset_data`: Writes 1024 to a global variable, reads three values from global variables, and if a fourth global variable is non-zero, calculates a new address using an offset and a value from a table, then calls another routine and clears a value in a5.
- `update_object_structure`: Checks a bit in a global variable, and if set, performs a series of pointer swaps and field updates on a structure, including clearing several offsets and setting new values at offsets 60 and 64.
- `update_structure_from_list`: Checks a value from a routine, and if it meets certain criteria, updates a linked list and a structure with values derived from three separate calls to a routine that generates obfuscated data.
- `initialize_structure_loop`: Loads a pointer into a0, then performs two nested loops to write bytes to a structure at that pointer, and finally writes a value to a global variable and branches to another routine.
- `compare_structure_fields`: Reads a 16-bit value from offset 64 of a structure in a0, and if it is not negative, compares a 16-bit value at offset 32 of a0 with a global variable.
- `process_structure_data`: If d3 is non-negative, it calculates an offset based on d3 and a global variable to access a table, then performs complex conditional logic involving hardware registers and a structure in a6 to update various fields in a structure pointed to by a0.
- `copy_table_entries`: Writes 4000 to an offset in a6, then loops through a buffer in a0, modifying values based on a constant and copying data from a calculated source address to a destination buffer until a zero byte is encountered.
- `manage_timer_and_hardware`: Compares a global variable against two other global variables and, if it falls within a specific range, decrements a counter and performs conditional hardware register updates and structure field assignments based on CIAA status.
- `update_table_from_index`: Increments a global variable, and if it is less than a threshold, uses the new value as an index into a table to copy two 16-bit words into a destination table at a fixed address.
- `fill_data_table`: Uses a loop to write a 32-bit value into a table, splitting it into two 16-bit words at specific offsets, incrementing the value and the table pointer in each iteration.
- `set_status_word`: Sets the status flag based on a global variable and, if the flag is not zero, writes 384 to an offset in a6 and returns.
- `set_status_word_alternative`: Sets the status flag based on a global variable and, if the flag is not zero, writes 32640 to an offset in a6 and writes 0 to another offset in a6 before returning.
- `reset_and_update_attribute_tables`: Clears several 16-bit and 32-bit global variables, initializes a set of tables via multiple calls, and then enters a loop that scales values from a table by a global factor and accumulates the results into a destination structure.
- `process_complex_updates`: Sets the status flag based on a byte in a global variable and, if non-zero, calls several routines, updates a global variable, performs arithmetic on a value derived from a global pointer, and calls another routine multiple times with different parameters and loop counts.
- `process_linked_list_updates`: If a global variable is between 12 and 15, it traverses a list of 4-word entries, performing additions and updating values in a secondary table based on the results and carry flags.
- `free_and_call_function`: Checks if a pointer in a2 is not negative or all ones, then if the value at a2 is not zero, clears the value at a2, calls a function at an offset from a library base using the value from a2, and sets a2 to a new address.
- `call_nested_routines`: Pushes the contents of a0 to the stack, calls two different routines, restores a0 from the stack, calls a third routine, pushes a0 to the stack again, calls a fourth routine, restores a0, and returns.
- `traverse_and_update_list`: Iterates through a list of pointers, compares bytes from two sources, and if they match, calls another routine to update a structure; it also handles a loop that calls another routine and copies data from a source to a destination.
- `modify_and_call_routines`: Calls three different routines and writes zero to a global variable three times.
- `process_linked_list_node_data`: Iterates through a linked list to update a structure, performing linear interpolation on values and copying data from a table.
- `process_linked_list_and_update_structure`: Iterates through a linked list to populate a structure in a0 with data from a table, including performing linear interpolation on values and calling a function pointer.
- `calculate_and_store_offset`: Calls another routine, subtracts a global word from the result, adds 1520, masks the result with 2044, and stores the value at offset 20 of a structure in a0 while clearing offset 22 of the same structure.
- `update_structure_fields_2`: Copies a value from a1 to offset 44 of a structure in a0, clears offset 48 of a0, copies a value from offset 4 of a1 to offset 4c of a0, and copies a value from a pointer at offset c of a1 into offset 50 of a0.
- `check_bit_and_set_values`: Checks bit 1 of a global variable multiple times and, if certain conditions are met, writes 30 to offset 156 of a structure in a6, writes a value from a global variable to offset 6c, and writes c000 to offset 154 of a6.
- `mask_interrupt_requests`: Masks bits 6 and 7 of the interrupt request register, writes the result to the interrupt mask register, stores a byte to a global variable, and returns from an interrupt.
- `process_entity_list_and_callback`: Updates several global timing or counter values, then iterates through a linked list of structures, performing arithmetic and calling a function pointer stored at offset 44 of each structure if a specific field is positive.
- `validate_range_bounds`: Compares a value from a structure in a0 against several global variables and other values within the structure to determine if it falls within specific ranges, returning negative one if it does and zero otherwise.
- `check_and_update_structure`: Calculates sums from offsets 20, 80, 32, and 82 of a structure in a0, compares these against values in a structure pointed to by a global pointer, and if a specific bitmask check on offset 72 of that structure passes, decrements a global variable, updates three longwords in the structure, calls another routine, decrements a word at offset 92 of a0, and either jumps to an address in a1 or returns.
- `update_structure_list`: Calculates two values from offsets 80 and 20 of a structure in a0, then enters a loop that uses a pointer from offset 52 of a0 to update various fields in a new structure, including several longwords and a calculated value, based on a count from offset 8 of the original structure.
- `execute_callback_chain`: Sets two values in a6, calls two routines, and then iterates through a linked list to call functions at specific offsets and update structure fields based on a bit test.
- `clear_on_bit_test`: Tests bit 4 of a global variable and, if it is zero, clears a 32-bit value in a global variable and returns.
- `process_conditional_callbacks`: Iterates through a linked list, checking if a 16-bit value at offset 32 and a bitmask from a global variable meet certain conditions, then decrements a value at offset 36 and calls a function at offset 40.
- `increment_global_word`: Increments a 16-bit value at a global variable address and returns.
- `calculate_bit_shift`: Stores a word from d0 into a global variable, calculates the result of 256 bit-shifted right by the value in d0, and stores that result in a global variable.
- `accumulate_scaled_values`: Decrements a global counter and, if non-zero, iterates through a table to scale three 16-bit values by a shift amount, adds them to existing values in the table, and calls another routine.
- `combine_register_bits`: Combines bits from three different registers into a single twelve-bit value and returns it.
- `transform_table_values`: Calls another routine, then loops through a table starting at a pointer from a global variable, negating the value of 256 minus the current word and clearing the following word, repeating this three times per entry.
- `negate_and_shift_table`: Calls another routine, then loops through a table starting at a pointer from a global variable, negating the current word and storing the original word left-shifted by 8 bits in the following word, repeating this three times per entry.
- `clear_table_words`: Calls another routine, then loops through a table starting at a pointer from a global variable and clears the word at offset 2 from the current pointer, repeating this three times per entry.
- `repack_color_data`: Iterates through a table, extracts two 16-bit words from a structure pointed to by an element in the table, and repacks their nibbles into three new 16-bit words stored at offsets 0, 4, and 8 of the table entry.
- `insert_into_list`: Writes a 32-bit value from a1 into a structure at offset 12 of a global variable and then updates that global variable to point to the next entry at offset 16.
- `update_global_parameters`: If d3 is non-zero, it checks d4 and updates three global variables at offsets 9190, 9194, and 9198 with specific values, then stores d3 into a global variable at 9ea6; otherwise, it performs a different set of updates to those same global variables and stores a different value into 9ea6.
- `adjust_table_indices`: If d3 is non-zero, it checks d4 and d5 to determine whether to increment or decrement d4 or d5, and if certain conditions are met, it updates global variables at 9190, 9194, and 9198; if d3 is zero, it performs a similar conditional update to those same global variables based on d4 and d5.
- `process_input_status`: Reads a value from a pointer stored at 9cb0, performs bitwise tests on it and the CIAA_PRA register, and updates a global variable at 92b8 based on the results.
- `update_entity_attribute_and_buffer`: Calculates a new value based on a global variable and bitwise flags, then writes this value and several derived bytes into a structure pointed to by a table entry.
- `update_value_with_offset`: Adds a value from a global variable to a word in another global variable, compares the results of bitwise AND operations on the original and new values, and if they differ and a bit is set, calls another routine and branches to another routine.
- `update_structure_with_rotation_2`: Tests a word at a global variable, and if bit 0 is set, populates a structure in a6 with calculated values and calls another routine.
- `check_and_invert_flags`: Checks bit 0 of a global longword; if set, it tests a global word and, if non-zero, performs a bitwise NOT on that word and another global word, then branches to another routine.
- `clear_global_word`: Clears a word at a global variable and then checks a different global variable; if that variable is not zero, it branches to another routine, otherwise it returns.
- `check_and_clear_global`: Loads a longword from a global variable into d0, and if it is not zero, it clears a word at a global variable and returns; otherwise it clears a word at a global variable and returns.
- `modify_global_pointer`: Subtracts a constant from a long value at a global variable, and if the result is zero, writes a constant to a memory location and calls another routine; otherwise, it writes a constant to the global variable and returns.
- `write_constant_value`: Stores the word value 238 into the memory location at 9b67e.
- `initialize_attribute_tables`: Calls several routines to populate various tables using different offsets and parameters, then performs a branch to another routine.
- `generate_attribute_table`: Constructs a table of data by calculating bitwise combinations and offsets based on a loop counter and values from another routine, then calls that routine.
- `generate_attribute_data`: Loops d1 times, calculating two 16-bit values based on d0 and storing them along with a value from a buffer in a0 into a destination pointed to by a1, then increments d0 and a1.
- `update_object_properties`: Calculates several values based on d0, d1, and a pointer derived from d2, then writes these values to a structure at a location derived from d2 and updates a table using d3.
- `initialize_data_table`: Runs a loop eight times to write the value 0 into the second byte of each 8-byte block and the value 50352 into the sixth byte of each block, starting from offset 45584.
- `process_pointer_list`: Checks a byte at offset 2 of a6 and the CIAA_PRA register, then iterates through a list of values pointed to by a5, writing the values and their high words into a structure at a6.
- `process_data_list`: Checks a byte in a6 and the CIAA_PRA register, then iterates through a list of long values to store modified values into a structure in a6.
- `set_high_word_mask`: Checks a byte at offset 2 of a6 and the CIAA_PRA register, and if certain conditions are met, stores a fixed address into the memory location at offset 44 of a6.
- `populate_offset_table`: Runs a loop four times, writing the low and high words of a global variable into a table starting at 9b98, incrementing the global variable by 38 each iteration, and finally performs bitwise operations and memory reads on the resulting value.
- `initialize_data_table_2`: Runs a loop four times to write the value 50352 into offset 2 and offset 6 of a structure pointed to by a global variable, while also setting offset 2 to 0.
- `free_linked_list`: Iterates through a list of pointers starting at a global variable, calls a routine to free each memory block, and clears the global variable when the list ends.
- `update_global_counters`: Subtracts a value from a global variable, updates several other global variables using arithmetic, and if a specific bit is set, writes a modified value to a global variable at offset 46706.
- `traverse_linked_list`: Iterates through a list of values starting at a global variable, calling another routine when a word value of 384 is encountered, and returns when a long value of -2 is found.
- `process_linked_list_and_build_buffer`: Iterates through a linked list of structures, clearing a global variable and updating a list of pointers.
- `write_formatted_data`: Adds 61 to a value in d0 and, if the result is within a specific range and d3 is zero, writes a sequence of bytes to a buffer starting at a1.
- `pack_color_data`: If bit 0 of a pointer in a0 is set, it packs nibbles from four consecutive bytes into two 16-bit words and writes them to a buffer at a1 along with header values.
- `branch_to_offset_51d74`: Performs a relative branch to a specific instruction.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_to_middle_of_routine_2`: Performs a short branch to an instruction located within another routine.
- `branch_to_mid_routine`: Performs a short branch to an offset within another routine.
- `branch_to_next_routine`: Performs a short branch to a different routine.
- `branch_to_offset_24`: Performs a short branch to an instruction located 24 bytes ahead of the current location.
- `branch_to_subsequent_code`: Performs a short branch to a nearby instruction.
- `setup_device_structure`: Allocates memory and a signal, opens a device, and populates a structure with the resulting pointers and specific configuration values before calling another routine.
- `execute_library_calls`: Calls a routine, then uses a pointer from a global variable to perform two subsequent library calls and an indirect jump via the exec base.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `ensure_function_pointer`: Checks if a global variable contains a non-zero value; if it is zero, it allocates memory and calls a routine to initialize that pointer, then enters a loop calling that pointer until it returns zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_function_pointer`: Clears a global variable and then performs an indirect jump via the exec base.
- `initialize_function_pointer`: Compares a value in a global variable with register d0 and, if they are not equal, calls several routines with various arguments to update the global variable with the result in d0.
- `jump_to_routine`: Performs a short branch to another routine.
- `handle_interrupt_trap`: Checks bit 5 of the interrupt mask register; if not set, it loads a value into a global variable and triggers a trap, otherwise it performs a sequence of two traps with different values.
- `update_interrupt_mask_and_status`: Copies five longwords from a table into a destination pointed to by a0 and then sets the two most significant bits of the interrupt status register.
- `configure_interrupt_and_memory`: Sets interrupt mask registers, copies several longwords from a source to a destination, and performs a series of library calls and memory operations involving a loop.
- `process_memory_structure`: Compares a memory range, updates several global variables with calculated values, and calls a library function using a pointer from a structure.
- `fill_zero_data`: Contains eleven consecutive instructions that perform a bitwise OR with zero on register d0.
- `fill_zero_data_single`: Performs a bitwise OR with zero on register d0.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `clear_d0_and_return`: Performs two bitwise OR operations with zero on register d0, clears register d0 using a moveq instruction, and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a5, so the target is only known at run time.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_structure`: Calls a routine, stores the value from d1 at offset 36 of a structure in a1, stores the value from d2 at offset 44 of the structure, and then calls another routine.
- `ensure_library_base_and_loop`: If a global variable is zero, it loads a library base into that variable, then enters a loop that calls another routine until it returns zero.
- `clear_global_and_jump`: Clears a global variable, loads a pointer from a memory location into a1, and performs an indirect jump through a value in a6.
- `execute_external_function`: Calls another routine and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
