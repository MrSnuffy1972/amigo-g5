# Game 60 (Zool2 v1.0 AGA 0415): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00060). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1492989 top-level bytes in 77 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 47 raw-word lines |
| Functions named and described | pass, 278 of 278 named (0 author symbols), 278 of 278 described (9 by rule, 268 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1786 of 1786 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 64383 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5201 of 5248 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1121116 |
| code | 22512 |
| data | 349341 |
| padding | 20 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zool2AGA.Slave | 1 | 263 | 13 |
| z2 | 4 | 223 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 788 | 49 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 748 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 28 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 625 | 38 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 456 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2062 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 8 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |
| mng.sod (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Zool2_v1.0_AGA_0415/Zool2AGA/data/mng.toomus: high-entropy data file (7.69 bits/byte), classified as packed data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 265 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 264 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 137 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 194 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 422 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zool2_v1.0_AGA_0415/Zool2AGA/data/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Zool2_v1.0_AGA_0415/Zool2AGA/data/mng.sod: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Zool2_v1.0_AGA_0415/Zool2AGA/data/z2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 64383 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zool2AGA.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `resolve_library_function_pointers`: Retrieves function pointers from a library or an object structure and checks a magic constant or a jump table.
- `apply_bitmask_to_memory`: The routine calculates a pointer, performs a bitwise AND with a constant on the value at that pointer, and calls a sequence of other routines or indirect calls.
- `update_global_mask`: Updates a global variable by setting a bit or a fixed value based on the value of another global variable.
- `load_from_indexed_table`: Loads a 32-bit value from a table using a base address and an indexed offset, then stores it at a destination address.
- `check_global_variable`: Compares a bit-shifted input value against a global variable.
- `set_global_max`: Sets a global variable to a fixed maximum value.
- `store_and_poll_blitter`: stores a byte from a register into a memory location and loops until the blitter-busy bit in a hardware register is cleared.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `check_highscore_table`: Loads a table pointer and a global variable, calls a routine through a pointer, and if the return value is non-zero, calls another routine through a pointer using the global variable.
- `execute_conditional_branch`: Checks a global variable and, if it is zero, calls a routine through a pointer and then calls another routine through a pointer.
- `xor_memory_with_decreasing_counter`: Xors a range of memory bytes with a decreasing counter value and returns a constant.
- `handle_unreachable_code`: The routine contains six 68000 instructions that decode from a code gap and are flagged as unreachable.

### z2

- `copy_block_recursively`: Recursively copies a block of memory or a linked list of data structures, using a recursive or iterative process to move bytes or words from a source or stack-based buffer to a destination buffer.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `fill_figure_table`: Reads a count, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_linked_list_data`: Iterates through a list of pointers, nullifying entries, and performs a pointer-based calculation involving a loop that resolves a nested value before updating a global memory area.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_string_for_delimiter`: checks a flag in a global structure and scans a string for a colon or null terminator, calling another routine if no colon is found.
- `find_string_delimiter`: Scans a string in memory and returns the value or a pointer if a colon or a null terminator is encountered.
- `find_string_delimiter_2`: Iterates through a string and returns when a colon or null terminator is found.
- `initialize_game_data_structure`: Checks if a global pointer is already initialized, otherwise calls another routine to allocate or retrieve a pointer and populates a large memory block with a pattern of constant values and recursive function pointers back to itself.
- `execute_descending_loop`: Sets a global variable to a decreasing value and calls another routine repeatedly until a counter reaches zero.
- `iterate_function_table`: iterates a loop that updates a global variable with a decreasing counter and calls a function from a table, where the table address is an offset from a base pointer.
- `update_global_pointer`: Updates a global variable with a value loaded from an offset or a pointer, then performs an indirect jump.
- `clear_two_word_table_entry`: Clears a two-word table entry pointed to by a global variable and then stores two input words into the next two words of that entry.
- `set_table_entry`: Writes a constant value and two word inputs into a table pointed to by a global variable.
- `update_counter_offset`: Reads an integer from a pointer, then adds a constant to a word at a fixed offset and increments a short at another fixed offset within that structure.
- `clear_message_struct`: Clears several fields in a message structure pointed to by a0, including the next pointer, the io request pointer, and two additional fields, and conditionally frees an associated memory block via a helper routine.
- `store_pair_to_table`: The routine writes a value from a register to the location pointed to by a pointer and writes a value from another register to the next entry in the same table.
- `set_player_stats`: Stores a 16-bit value from a register and a 32-bit value from a register into a structure pointed to by a pointer, writes the 16-bit value into an offset within the structure, computes a shifted value and stores it into two fields of the structure, and clears a field in the structure.
- `clear_nine_words`: Iterates a fixed number of times to write zero to a sequence of words starting at a pointer.
- `clear_table_and_set_byte`: Clears a sequence of words starting at a pointer and then writes a constant byte to an offset within that memory block.
- `multiply_input_by_eight`: The routine returns the value it receives in D0 multiplied by eight.
- `initialize_data_structure`: Initializes a large memory block or structure with a pointer to a global variable, several zero or constant values, and specific bit patterns.
- `store_byte_to_offset`: Stores a single byte from a register into a memory location at a fixed offset from a pointer.
- `store_byte_to_offset_2`: stores a byte from a data register into a memory location calculated by adding a constant offset to a pointer held in an address register.
- `store_byte_to_offset_extended`: Stores a byte from a register into a memory location at a fixed offset from a pointer.
- `store_two_words_to_offset`: Stores two consecutive words from registers into a memory location starting at a fixed offset from a pointer.
- `copy_player_stats_to_slot`: Copies a pointer from a source structure into a destination slot and clears a word in the destination.
- `apply_bitmask_transform_to_buffer_2`: Performs bitwise logical operations (AND, OR, XOR) or bit-shifting or bit-masking on a destination buffer using a lookup table or bitmask-driven logic.
- `transform_buffer_with_bitmask`: Applies bitwise XOR, OR, or AND operations to a destination buffer using a bitmask-driven lookup table.
- `apply_bitmask_transform_to_buffer`: Iterates through a data buffer, applying bitwise XOR, OR, or AND operations to each word based on a bitmask or a lookup table.
- `initialize_buffer_with_pattern`: Calculates an address based on a register and writes a pattern or calculated value, then fills a subsequent block of memory with a constant pattern.
- `update_global_pointer_2`: Updates a global variable with a value from a pointer, potentially clearing a memory field or performing bitwise operations on a table if certain conditions or zero-values are met.
- `store_color_word`: Stores a 16-bit value into a global variable table using an index derived from the low byte of a register doubled.
- `read_byte_from_table`: Computes an address from a global variable and an input value, then returns a byte formed by combining the byte at that address with the next byte shifted left by four bits.
- `extract_global_field`: Returns a value from a global variable that has been masked and right-shifted by eight bits.
- `calculate_bit_map`: Checks a bitmask in a table and accumulates set bits into a 32-bit register.
- `return_all_ones`: Returns a value where all bits are set.
- `modify_bit_mask_2`: Iterates through a list of pointers and uses a bitmask or bit-set/clear logic to modify a bit in a memory location.
- `modify_bit_mask`: Clears or sets a bit in a memory location based on a pointer and a loop counter.
- `return_all_ones_2`: Returns a 32-bit value where all bits are set to one.
- `write_byte_to_offset`: Stores a byte from a register into a memory location at a fixed offset from a pointer.
- `do_nothing`: The routine is empty and does nothing.
- `execute_nested_loop_calls`: performs a nested loop that repeatedly calls another routine while incrementing a register or comparing against a global value or register.
- `run_nested_loops`: Executes a nested loop that calls another routine while incrementing a register.
- `return_empty_routine_2`: the routine performs no operations and immediately returns to the caller.
- `clear_flag_and_table_entry`: Clears a bit in a byte at an offset from a pointer and writes a constant value to a table entry.
- `increment_and_set_bit_2`: Increments a value and sets a bit in a memory location, returning a failure code if a bit collision is detected.
- `return_all_ones_3`: returns a 32-bit or 32-bit-width register or value containing all bits set to one.
- `increment_and_set_bit`: Increments a value and sets a bit in a memory location until a bit collision occurs.
- `store_table_entry_and_enable_dma`: Stores a register value into a table indexed by a pointer-derived value and enables DMA channels by writing to a global variable.
- `calculate_and_store_values`: Calculates several values based on register inputs and a pointer-indexed table, then stores these bytes into a memory buffer.
- `wait_for_global_flag_clear`: Spins in a loop until a specific bit in a global variable is cleared.
- `tiny_moveq_not`: Small routine of 2 instructions that loads 0 into d7, then inverts d7.
- `apply_bitwise_mask`: Iterates through a pointer, performing a bitwise AND between a register and memory contents.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `apply_bitwise_and_mask`: Performs a bitwise AND between a register and a memory location pointed to by a descending pointer.
- `update_table_entry`: Calculates a new value based on several registers and writes a series of derived bytes into a table located via a pointer.
- `set_table_and_global`: Writes a value to a table entry using a pointer and then sets a global variable to a constant value.
- `clear_table_bit`: Clears a specific bit in a byte at a certain offset and sets a table entry to a full bitmask.
- `wait_for_bit_clear`: Enters a loop that continues as long as a specific bit in a global variable remains set.
- `initialize_struct_and_global`: If a certain register is zero, it initializes a structure with a zero, a pointer to a global, and a constant, otherwise it updates a pointer and a global variable.
- `apply_bitwise_mask_to_array`: updates a block of memory by performing a bitwise AND between a register and the memory contents, decrementing a pointer or counter-based address or loop limit or pointer during the process.
- `apply_bitwise_and_mask_loop`: Iteratively applies a bitwise AND between a register and a memory location using a descending pointer.
- `apply_bitwise_and_to_memory`: Iteratively applies a bitwise AND operation between a register and a memory location, while decrementing the pointer.
- `apply_bitwise_and_to_memory_2`: Iterates through a memory block, performing a bitwise AND between a register and the memory contents, while decrementing the pointer.
- `apply_bitwise_and_loop`: Decrements a pointer and performs a bitwise AND between a register and the memory location pointed to by that pointer in a loop.
- `execute_indirect_calls`: Performs several function calls including two indirect calls using an offset from a value stored in a global variable.
- `initialize_runtime_context`: Manages library-based resource locks and initializes or clears global state or runtime parameters, including setting certain hardware registers and sprite positions.
- `execute_with_locks`: Executes a function pointer from a structure while managing port or resource locks.
- `branch_on_global_flag`: Checks a global variable and calls another routine if the variable is non-zero.
- `save_and_reset_context_parameters`: Stores a value from a pointer-based offset into a global variable and resets that memory location to a maximum value.
- `access_library_function`: Checks a global variable and calls a function pointer from a library or table, then potentially calls another function via an offset.
- `fetch_string_length`: Retrieves a string length via a function pointer and calls another routine if a certain condition is met.
- `calculate_string_length`: Iterates through a null-terminated string and returns the negative or unsigned length of that string.
- `update_function_context`: Updates several global or table-based values and calls a function pointer before setting a global variable.
- `close_library_conditionally`: Checks if a global pointer is non-zero and calls a function at an offset from that pointer, specifically targeting a library closing routine.
- `initialize_global_state`: Checks if a global variable is zero, then calls two different functions using an offset from a global pointer before setting that global variable to negative one.
- `clear_global_state`: Checks if a global variable is non-zero, then calls two different functions using an offset from a global pointer before setting that global variable to zero.
- `traverse_and_accumulate_pointers`: Iterates through a list of pointers, accumulating their addresses into a global variable if certain conditions are met, while calling a routine.
- `clear_function_pointers`: Iterates through a buffer, calling a function pointer and then zeroing it out.
- `set_up_routine_context`: Modifies bytes in a buffer, calls a function via a global pointer, and populates a structure or routine with several values including a pointer to a global variable.
- `replicate_context_setup`: Performs the same logic as a previous routine but uses a specific memory address instead of a global variable to store and retrieve function pointers.
- `setup_device_context`: Performs device-related setup by calling library functions and updating a global pointer and several memory locations.
- `setup_function_context`: Calls a function pointer retrieved from a global variable offset, updates a value in a table, and stores a pointer to a global variable into another routine's offset.
- `traverse_pointer_list_2`: Follows a chain of pointers, checking a byte in a structure and setting a global variable to zero if a certain condition or null pointer is met.
- `return_zero`: This routine clears the accumulator and returns a zero value to the caller.
- `traverse_pointer_list`: Iteratively follows a chain of pointers and checks a certain byte in a structure until a null pointer or a specific byte value is encountered.
- `call_function_via_offset`: Executes a function call using an offset from a value held in a register, specifically using a library-related pointer.
- `initialize_structure_member`: Checks a value and, if a function returns a valid pointer, writes a value and a constant to a structure.
- `set_global_constant`: Assigns a constant value to a global variable.
- `initialize_structure_members`: Checks a return value from a function pointer and either updates a global or initializes a new structure with specific or pointer-based values.
- `clear_structure_fields`: Sets certain fields of a structure to a maximum value and calls routines to remove a port and free memory or signals.
- `initialize_runtime_parameters`: Reads a global variable, calls a function using an offset from that variable, and writes two constant values to specific memory locations.
- `execute_multiple_subroutines`: Executes three different subroutines using function pointers retrieved from a global variable.
- `execute_self_and_return`: Calls itself once and then returns the value stored in a register.
- `reset_global_state_2`: Clears a global variable, sets another, and calls functions via offsets from a global pointer, potentially handling a null return.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d6.
- `reset_global_state`: Clears a certain memory location and a global variable, then calls a function via an offset from a global pointer and performs conditional cleanup or assignment.
- `configure_dma_and_sprites`: Sets or resets a global variable and updates several sprite position registers and the DMA controller or color registers.
- `execute_subroutine_chain_node`: Executes a specific node in a sequence of person-specific subroutines.
- `execute_subroutine_chain_3`: Executes a sequence of person-specific subroutines, including recursive calls to itself.
- `execute_subroutine_chain_2`: Calls a long sequence of different subroutines and toggles a bit in a global variable.
- `recursive_self_calls`: The routine calls itself once and then calls two other functions before returning.
- `execute_dma_setup`: Writes several constant values to a global variable, calls a routine, and executes a loop that continues while a certain bit in a global variable is not set.
- `execute_subroutine_chain`: Executes a long sequence of twenty different subroutines.
- `enter_infinite_loop`: Performs an infinite loop that never exits.
- `loop_until_condition`: Resets a global variable and executes a routine repeatedly until a condition involving a register or a global variable is no longer met.
- `store_constant_to_global`: Stores a specific constant value into a global variable and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_dma_and_global`: Executes a sequence of subroutines, writes a constant to a global, and sets a global variable to a specific value while touching the DMA controller register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `update_lfsr_state`: Updates a global 32-bit value and a 16-bit value by performing bitwise operations and shifts.
- `branch_on_global_variable`: Checks if a global variable is non-zero to decide whether to perform a sequence of additions or a different set of additions using saved register values and a constant.
- `modify_global_variable`: Adds the current value of a global variable to a previously saved copy of itself and then assigns a constant value to a global variable.
- `recalculate_attribute_table_indices_2`: Recalculates pointer values or constants based on a global variable and writes them into a range of memory.
- `recalculate_attribute_table_indices_3`: Updates a range of memory with new pointer values or constants derived from a global variable.
- `perform_no_op_sequence`: Executes a series of instructions that read and write a global variable to itself without changing its value.
- `recalculate_attribute_table_indices`: recalculates pointer values or constants based on a global variable and writes them into a range of memory, then calls a routine to update an attribute table and another to update a pointer array.
- `update_attribute_table_recursive`: Recursively modifies a global variable and updates a range of memory with calculated pointer values or constants.
- `do_nothing_2`: The routine is empty and performs no operations before returning.
- `update_attribute_offsets_3`: Iteratively calculates a value based on a pointer and a global variable, storing the result in a destination address while decrementing a counter.
- `update_attribute_offsets_2`: Iteratively calculates a value based on a pointer and a global variable, storing the result in a destination address, while decrementing a counter.
- `update_attribute_offsets`: Iteratively calculates a new offset based on a pointer or value and stores it in a destination address, while decrementing a loop counter.
- `update_pointer_array_2`: Iteratively updates destination pointers and values while traversing an array and checking a bit in a global variable.
- `update_pointer_array`: Iteratively updates destination pointers and values while traversing an array, checking a bit in a global variable and updating a value base.
- `process_pointer_array`: Iteratively updates destination pointers and values while traversing an array, checking a bit in a global variable and updating a value based on a subtraction.
- `recursively_update_attributes`: Recursively reduces a global variable and then calculates a destination address to store a calculated value into a table.
- `update_attribute_offsets_4`: Calculates a destination address based on a global variable and a pointer, then iteratively stores a calculated value into a table.
- `update_attribute_offsets_3_2`: Calculates a destination address based on a global variable and a pointer, then iteratively stores a calculated value into a table while decrementing a counter.
- `recursively_update_attributes_alt`: Recursively adjusts a global variable before performing attribute offset updates via a calculated destination address and a table.
- `update_attribute_offsets_alt`: Modifies a global variable and uses it to calculate a destination address, then iteratively writes a value derived from a table entry into a memory location.
- `update_attribute_offsets_3_alt`: Modifies a global variable and uses it to calculate a destination address, then iteratively writes a value derived from a table entry into a destination address.
- `update_pointer_array_pointers`: Iteratively updates a list of pointers by calculating new destination addresses and associated values while checking a bit in a global variable.
- `update_pointer_array_3`: Traverses an array of pointers to update a destination address and a value while checking a bit in a global variable.
- `update_pointer_array_2_2`: Iteratively traverses an array of pointers to update a destination address and a value, while checking a bit in a global variable and decrementing a counter.
- `initialize_pointer_table_and_populate_ram`: Initializes a range of RAM with constant values and then iterates through a table to calculate and store new memory addresses or pointer-based offsets into a destination block.
- `initialize_pointer_table`: Initializes a block of memory with constant values and then iterates through a table to calculate and store new memory addresses.
- `initialize_and_update_pointer_table`: Initializes a block of memory with specific constant values and then iterates through a table to calculate and store new memory addresses based on existing pointer values and a global offset.
- `populate_table_with_offsets`: Writes a sequence of incrementing 32-bit values into two different memory tables and sets a global variable based on a calculation.
- `populate_data_tables`: Writes a global variable and its high-word/low-word parts into two different memory tables using a loop, then sets a global variable to a constant or a calculated value.
- `reset_global_to_zero`: Sets a global variable to zero.
- `set_global_to_max`: Sets a global variable to a specific maximum value.
- `set_global_to_constant`: Sets a global variable to a constant value.
- `apply_bitwise_mask_to_data`: Iterates through a table of pointers and applies a bitwise mask to a block of memory, masking certain bits to zero.
- `apply_complex_mask_to_data`: Iterates through a table of pointers and applies a bitwise mask involving an inverted operand to a block of memory, masking certain bits to zero.
- `process_attribute_buffer`: Iterates through a table of pointers to apply bitwise masks to memory blocks and calculates a value for a destination buffer based on an offset and a pointer-based value.
- `populate_attribute_buffer`: calculates a bitwise or of multiple adjacent memory or sprite-related attribute values and writes the result into a destination buffer.
- `clear_memory_range`: Sets a global variable to a constant and then clears a block of memory until a certain pointer value is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_values`: Sets a global variable to a specific constant value.
- `execute_nested_loops`: Modifies a global variable and executes a series of recursive loops that repeatedly call another routine a fixed number of times.
- `reset_global_variable`: clears a global variable by writing zero to it.
- `update_global_mask_2`: Updates a global variable by setting a bit or a fixed value and copies a fixed number of words from a global source to a different destination.
- `copy_memory_block`: Copies a fixed number of words from a global variable source to a specific memory destination.
- `copy_memory_block_alternate`: Copies a fixed number of words from a global variable source to a different memory destination.
- `apply_bitmask_to_data_3`: Recursively modifies a global variable and performs bitwise operations using a mask and a pointer, then enables DMA channels.
- `apply_bitmask_to_data_2`: Performs bitwise operations using a mask and a pointer, then enables DMA channels by writing to a hardware register.
- `apply_bitmask_to_data`: Performs bitwise operations using a mask derived from a register and a pointer, then stores the result to a destination pointed to by a register.
- `initialize_complex_pointers`: Performs a series of assignments to a global variable, including pointer arithmetic and setting a global variable to a calculated offset.
- `assign_multiple_globals`: Assigns two different constant values to two different global variables.
- `configure_hardware_registers`: Loops until a bit in a global variable is cleared, then writes specific values to several hardware registers and memory-mapped registers.
- `increment_global_variable`: Sets a global variable to a specific value and then increments it before calling another routine recursively.
- `set_audio_volumes`: Updates the volume levels for four audio channels.
- `do_nothing_3`: the routine is empty and performs no operations.
- `set_global_bit_flags`: Checks specific bits in a global variable and sets that global variable to a fixed constant or a different constant based on which bit is set.
- `set_global_constant_2`: sets a global variable to a fixed constant value.
- `set_global_constant_alt`: Sets a global variable to a different specific constant value.
- `update_global_value`: Modifies a global variable through a series of conditional checks and bitwise operations.
- `increment_global_variable_2`: Sets a global variable to a specific value and returns, having previously called another routine.
- `clear_memory_range_2`: Writes zero to a sequence of words in a specific memory range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_memory_with_shifted_values`: Reads a value from a source pointer, shifts it left, and writes it to a global variable and two other memory locations.
- `write_to_hardware_register`: Writes shifted values from a memory source to a hardware register or a specific memory address range.
- `write_hardware_register`: Writes a value from a memory source to a hardware register and enables DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_instruction_stream`: Iterates through a compressed or encoded instruction stream to expand or copy data.
- `clear_memory_ranges`: Writes zero to a sequence of bytes in a memory range, a memory block, and a global variable.
- `clear_byte_range`: Writes zero to a sequence of bytes in a specific memory range.
- `clear_memory_block`: sets a series of bytes to zero starting from a fixed memory address up to a fixed end address, skipping certain bytes in the sequence.
- `reset_global_variable_2`: sets a global variable to zero.
- `clear_memory_with_pointer`: Writes zero to a range of memorys determined by a value in a register.
- `store_split_values_to_table`: Splits a value into two parts and stores them into a table at a fixed offset.
- `store_split_table_values`: Splits a value into two parts and stores them into a table, then toggles the lowest bit of a global variable.
- `nested_loop_delay`: Executes another routine while running a decrementing loop, effectively creating a delay.
- `recursively_decrement_and_loop`: A recursive routine that decrements a register and calls itself twice before checking a bit in a global variable and executing a delay loop.
- `parse_string_to_value`: Iterates through a string buffer and updates a global variable based on whether a character is a specific control symbol or a digit.
- `modify_global_value`: Checks a global variable against specific values and updates it with constant or certain values, while also touching color registers.
- `clear_dma_buffer_and_reset_state`: Clears a block of memory by writing zero to a range of addresses and resets a global variable before re-enabling DMA or updating a table.
- `toggling_global_state_and_looping`: Toggles a global boolean or flag and executes a loop that decrements a counter while performing conditional checks and branching.
- `search_global_table`: Searches a table for a value matching a global variable, updates a global counter, and clears a specific hardware color register or memory address.
- `clear_color_registers`: Sets a global variable and a specific hardware color register or memory address to zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `parse_encoded_string`: Iterates through a string in memory, converting specific characters or numeric offsets into a global variable.
- `process_string_to_values`: Iterates through a string in memory, translating specific characters into numeric values stored in a global variable.
- `execute_indirect_call`: Calculates a target address using a global variable and a pointer table, then executes the routine at that address.
- `update_pointer_logic`: Checks a pointer or table entry against certain bounds and conditionally updates a global variable by calling other routines.
- `check_table_bounds`: Verifies a table entry or pointer and updates a global variable based on certain value conditions.
- `process_table_entry`: Loads a value from a table, performs boundary checks, and calls other routines while updating a global variable.
- `validate_pointer_reference`: Tests a pointer and its associated data against certain numeric ranges before performing conditional routine calls.
- `initialize_memory_values`: Sets several specific memory or register-mapped locations to fixed values after calling other routines.
- `increment_global_loop`: Loops while a global variable is not zero, setting it to a specific value and then incrementing it until a certain limit is reached.
- `set_global_value`: Assigns a specific value to a global variable and calls another routine.
- `wait_for_zero`: Loops until a global variable reaches zero.
- `execute_complex_routines`: Calls a sequence of other routines using calculated offsets and pointer-based arguments, returning a concatenated long value.
- `mutate_global_bits`: Performs a series of bitwise checks and conditional decrements or assignments on a global variable.
- `calculate_offset_index`: Calculates a new value based on a global variable and a fixed multiplier, then updates a table-driven state.
- `update_global_value_2`: Updates a global variable with a specific constant value and performs conditional calls to another routine based on that value.
- `parse_string_to_global_value`: Iterates through a string buffer and updates a global variable based on whether a character is a specific control symbol or a digit, then calls another routine and applies an offset to the global variable.
- `set_global_and_check`: Sets a global variable to zero, calls another routine, and then sets the global variable to a certain value if a memory location or certain byte is non-zero.
- `xor_memory_buffer`: Xors a global variable with a range of memory or a buffer and applies a bitwise rotation.
- `convert_string_to_digits`: Iterates through a buffer and replaces certain control characters or non-digit characters with or even certain printable digits or other symbols.
- `copy_block_of_longs`: Copies a fixed number of 32-bit words from one memory location to another using a loop.
- `initialize_buffer_pattern`: Iterates through a memory buffer, copying or propagating certain values, and fills a portion of memory with a repeating dot pattern.
- `initialize_buffer_with_dots_2`: initializes a memory buffer by filling it with a repeating pattern of a specific byte and then copying or propagating existing values or patterns backwards through a memory range using a loop.
- `initialize_buffer_with_dots`: Writes a specific value and a sequence of certain bytes to a global variable and its subsequent memory locations.
- `decrement_global_pointer_2`: Decrements a global variable or resets it to zero or other certain values based on its current state, including conditional calls to another routine.
- `decrement_global_pointer`: Decrements a global variable and potentially updates it if a certain value is reached.
- `process_string_with_function`: Iterates through a memory range, updating a global variable with a value from a pointer and calling another routine.
- `update_global_bits`: Performs complex bitwise manipulation and conditional updates on a global variable, checking or clearing specific bits.
- `branch_on_table_values`: Evaluates values from a table at an offset, and calls various routines or sets a global variable based on those values.
- `update_global_counter_loop`: resets a global variable and executes a sequence of routines while incrementing that variable in a loop.
- `parse_string_to_offset`: Iterates through a memory buffer, checking for certain character symbols and updating a global variable with or or derived from those characters.
- `execute_string_processing`: Executes a loop that calls two different routines and updates a global variable or certain memory locations based on certain conditions.
- `parse_string_to_numeric_offset`: Iterates through a string, converting certain character symbols or digits into a numeric value stored in a global variable.
- `process_string_with_offset`: Iterates through a buffer, copying characters to a global variable and calling another routine, checking for a null terminator.
- `copy_fixed_data_block`: Copies a fixed amount of data from a global variable address to a fixed destination address using a loop.
- `initialize_memory_table`: Iteratively populates a table in memory with a sequence of constant values, a global variable, and a value returned from another routine.
- `populate_memory_structure`: Populates a sequence of memory locations with a mix of constant values, a certain global variable, and a value returned from another routine.
- `approach_target_value`: Adjusts a signed integer value by increments or decrements to move it closer to a target value stored in a memory structure.
- `decompress_variable_instruction_stream`: Iterates through a compressed or encoded instruction stream, using a control byte to determine the length and type of data to copy or expand.
- `expand_instruction_stream`: Decodes a bit-masked instruction stream to perform a variety of memory operations, including direct copies, literal or byte-fills, and back-copying or relative-offset-based expansions into a target buffer.
- `decompress_instruction_stream_3`: Expands a compressed or bit-masked instruction stream into a target memory buffer using a combination of direct copies, literal/byte fills, and backward-copying or block-based expansion.
- `expand_instruction_stream_complex`: Expands a compressed instruction stream into a destination buffer using a variety of copy or expansion operations, including literal copies, block-based copies, and backward-copying or incremental-value-generation loops.
- `decompress_instruction_stream_2_2`: expands a compressed instruction stream into a destination buffer using a combination of literal copies, block-based copies, and backward-copying or pattern-generating instructions.
- `decompress_instruction_stream_2_3`: Implements a custom bit-masked decompression or instruction expansion routine that uses control bytes to perform block-based copies, backward-copying, and pattern-based expansions or repeats.
- `expand_encoded_bitstream`: Executes a custom decompression or expansion routine that parses a bit-masked instruction stream to perform or reverse copy-backward or relative-offset memory expansions.
- `decompress_bitstream_data`: Performs a custom decompression or expansion or de-obfuscation of a data stream using a specialized instruction set that supports copy-back, repeat, and additive-offset operations.
- `decompress_bitstream_data_2`: implements a custom decompression or decompression-like expansion routine that reads a control byte or a multi-byte instruction and copies or modifies a certain number of bytes or words into a destination buffer.
- `decompress_instruction_stream_2`: Decodes a bit-masked control byte to determine a copy length and an instruction type, then executes a block-based expansion or copy-backwards routine to populate a destination buffer.
- `return_from_subroutine`: the routine is empty and performs no operations before returning.
- `do_nothing_4`: This routine is empty and performs no operations before returning.
- `populate_table_from_pointer`: iterates through a table to store incrementing values and clears a secondary buffer based on a value found in a source buffer.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_and_object_state`: Updates audio channel low-address registers and modifies object property or scaled-value state.
- `set_audio_registers_2`: Stores a specific value and a memory address into a global variable.
- `set_audio_registers`: stores a series of specific values into several custom audio registers and sets a global variable.
- `update_object_property_state`: Updates or resets object-specific properties, such as scales, offsets, or certain limit-based flags, by reading or writing values from a lookup table.
- `update_scaled_value`: Updates a target location by adding or subtracting a signed, scaled value and handles a conditional reset if a certain limit is exceeded.
- `update_value_with_scaled_offset`: calculates a signed, scaled value using bits from a source pointer and adds it to a target location, then increments a byte in a table.
- `update_scaled_value_and_offset`: Updates or resets a target value using a signed, scaled offset or a direct value, depending on a control type or flag.
- `do_nothing_stub`: The routine is empty and returns immediately without performing any operations or modifying state.
- `assign_returned_register`: Copies the value stored in a register after a call to another routine into a global variable.
- `populate_table_from_pointer_2`: Sets a global variable to a constant value and then updates it again.
- `execute_empty_sequence`: The routine executes a sequence of no-operation instructions and empty calls before returning.
- `execute_no_op_sequence`: executes a sequence of instructions that results in no state change before returning.
- `loop_until_flag_cleared`: repeatedly calls another routine as long as a specific memory location is non-zero.
- `call_other_routine`: calls another routine and returns a concatenated 64-bit or double-word value.
- `do_nothing_stub_2`: The routine is empty and returns immediately without performing any operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_stub_3`: The routine is empty and returns immediately without performing any operations or modifying data.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning or exiting.
- `do_nothing_stub_4`: The routine is empty and returns immediately without performing any operations or accessing data.
- `do_nothing_5`: this routine is empty and performs no operations before returning.
- `do_nothing_6`: The routine is empty and performs no operations.
- `do_nothing_stub_5`: The routine is empty and returns immediately without performing any operations or modifying any data.
- `empty_routine`: the routine contains no executable instructions and performs no operations.
- `do_nothing_7`: This routine is empty and performs no operations.
