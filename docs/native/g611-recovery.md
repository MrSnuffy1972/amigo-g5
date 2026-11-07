# Game 611 (Vaxine v1.0 0747): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00611). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (705064 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 46 raw-word lines |
| Functions named and described | pass, 166 of 166 named (0 author symbols), 166 of 166 described (6 by rule, 158 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3437 of 3442 (99.85%) |
| Recovered without load address (counted in proof 1, flagged) | 260672 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4681 of 4681 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 47746 |
| code | 14840 |
| data | 642401 |
| padding | 77 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 271 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4318 | 144 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 39 | 1 |
| vaxine (raw code file, load address unknown) | 1 | 13 | 1 |

## Cases handled

- Vaxine_v1.0_0747/Vaxine_v1.0_0747.hdf: hard-disk image (AmigaDOS OFS, 736 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Vaxine_v1.0_0747_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address: 3157 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Vaxine_v1.0_0747/Vaxine_v1.0_0747.hdf: 30 executed PCs whose RAM was overwritten before any snapshot match its bytes 98320-98480: recovered as a unit run at an address
- Vaxine_v1.0_0747/Vaxine_v1.0_0747_hdf/Vaxine/vaxine: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 260576-260672: recovered as a unit run at an address
- Vaxine_v1.0_0747/Vaxine_v1.0_0747_hdf/Vaxine/vaxine: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 260672 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_complex_transfer_sequence` (runtime segment an address): A large routine that initializes a global pointer, performs bitblt transfers, and updates coordinates or memory using a sequence of other routines and text-referenced strings. Kind: string-based manual/code-wheel lookup. Pass path: the branch or instruction following the check (implied by the successful string match or valid input or the next instruction in the sequence, e.g., moving a result into d0 or proceeding to the next logic block).

## Functions


### game.slave

- `return_all_ones`: The routine returns the constant all-ones value and performs no other operations.
- `init_library_and_decode_palette`: The routine stores constants into several global variables, invokes a set of library entry points through a pointer table, zeroes a memory range used as a screen buffer, and decodes a run-length encoded palette chunk into a table by reading the chunk header and expanding each triple of bytes.
- `dispatch_highs_table`: The routine sets a global variable to a value and then performs an indirect jump through a table that Ghidra could not recover.
- `decrypt_high_scores_table`: Checks a global variable and calls a function pointer if it is zero.
- `xor_decrypt_96_bytes`: The routine XORs 96 bytes starting at a global variable with a descending byte counter beginning at 96, writing the result back in place.
- `wait_for_global_flag_toggle`: The routine repeatedly waits for a global variable to change state, looping until the variable is set and then cleared, repeating this wait a fixed number of times.
- `wait_for_disk_ready`: The routine polls a global variable until a specific bit is set and another bit is clear, then returns.
- `loop_call_other_twice`: The routine initializes a short counter to zero and repeats a loop sixteen times.
- `call_other_routine_twice_in_loop`: The routine sets a short counter to fifteen and repeatedly calls another routine twice per iteration, decrementing the counter each time, until the counter reaches -1.
- `scale_color_table`: The routine reads a nibble from a global table and applies a signed offset to each nibble, clamping negative results to zero, then writes the resulting 12-bit color value to the hardware color registers.
- `wait_for_flag_toggle`: The routine spins until a global variable's least significant bit is cleared, then spins until that bit is set again.
- `setup_custom_chip_registers`: The routine builds a copper list in chip memory, waits for vertical blank to finish and start, clears a block of custom registers, then writes a set of values to custom registers including BPLCON0, BPLCON1, BPL1MOD, BPL2MOD, COLOR00, COP1LCH, DDFSTRT, DIWSTRT and DMACON, and waits for vertical blank again before returning.
- `decode_and_decode_message_block`: The routine walks a list of tagged blocks in a buffer and decodes a CMAP block into a table via nibble manipulation.
- `empty_stub_routine`: The routine contains 13 instructions but performs no observable work; it ends with an infinite loop and never returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_update_global`: Increments a register by two and updates a global variable before calling another routine in a loop.
- `configure_blt_and_update_table`: Configures certain bitplane or blitter-related hardware registers and then iterates through a table to store or update values based on a comparison or a bit flag status.
- `update_pointer_data`: Waits for a bit flag in a global variable to clear, then stores two 32-bit or 16-bit values and a 16-bit constant to a table.
- `expand_bitstream_to_table`: Expands a bitstream or bit-shifted pattern into a table of bit-shifted or bit-manipulated words.
- `initialize_global_pointer`: Assigns a pointer to a global variable and, if the pointer is null, initializes the global variable with a constant and a value from the pointed-to memory.
- `update_blt_mask_parameters`: Waits for a DMA or custom bus flag to clear, then calculates and stores a bitmask and a pointer-related value into a global variable.
- `traverse_string_pointer`: Iterates through a null-terminated string by updating a pointer with a value returned from another routine.
- `apply_bitmask_loop`: Iterates through a source table, applying a bitwise AND and OR or bitwise NOT and AND to specific offsets in a destination table.
- `apply_bitmask_partial`: Iterates through a source table to apply bitwise OR or bitwise NOT and AND operations to specific offsets in a destination table.
- `apply_bitmask_to_words`: Iterates through a source table, applying bitwise OR and bitwise AND or bitwise NOT and AND to 16-bit values at specific offsets in a destination table.
- `expand_string_data`: Parses a data buffer for specific magic markers and expands certain bytes into a destination buffer using a calculated repetition count.
- `decode_string_expansion`: Parses a string where certain byte values determine the number of times a subsequent byte or a calculated value is written to a destination buffer.
- `return_from_subroutine`: performs a standard return from a subroutine.
- `set_blt_parameters`: Waits for a bit flag in a global variable to clear, then sets a 32-bit value and a 16-bit value to a global variable.
- `configure_blt_transfer_2`: Iterates through a table of offsets, calculates destination or source coordinates, and sets hardware registers or global variables if certain bounds or flags are met.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_move_moveq`: Small routine of 2 instructions that copies 192 to d7, then loads 0 into d0.
- `configure_blt_transfer`: Waits for a bit flag to clear, sets a 32-bit value, and calculates certain destination or source addresses or offsets for a transfer.
- `empty_routine`: the routine is empty and performs no operations.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 0 into d0, then loads 0 into d1.
- `set_blt_struct_fields`: Waits for a DMA or custom bus flag to clear, then writes a 4-byte value and two 2-byte values into a global structure.
- `update_struct_fields`: Checks a bit in a global structure and stores a 4-byte value and two 2-byte values into that structure.
- `write_struct_fields`: Stores a 4-byte value and two 2-byte values into a global structure.
- `populate_lookup_tables`: Performs a complex loop that calculates and stores a series of calculated values into several tables.
- `modify_global_value`: Modifies a global variable using a sequence of conditional checks and bitwise or arithmetic operations.
- `update_clamped_global_value`: Checks a global variable and, if certain conditions or byte values at a pointer are met, updates a global variable with a clamped value derived from a pointer's contents.
- `process_input_and_clamp`: Reads a byte from a pointer, updates a global variable, and performs clamped addition or assignment based on certain bit flags.
- `modify_global_state`: Updates a global variable based on bits in a register, applying or clamping certain offsets, and resets a global variable to zero or a constant.
- `reset_global_variable`: Sets a global variable to zero.
- `map_input_to_global`: Checks an input byte against several constants and sets a global variable to a specific value or the input itself.
- `initialize_prefix_table`: Loads data from a table indexed by a value and then populates a secondary table with a sequence of incrementing sums.
- `initialize_pointer_table`: Loads several values from a source pointer into a global variable and then populates a table at a fixed address with a sequence of incrementing or offset-based values.
- `update_state_table_and_counters`: Updates a table of counters or state values based on bitmask-driven conditional logic and string-based triggers.
- `initialize_string_and_table_data`: Initializes a global pointer and a sequence of memory locations with specific values, then iterates through a null-terminated string to update a global pointer and a secondary pointer.
- `decrement_global_value`: Copies a global variable to a local register and then decrements that register in a loop until it reaches a certain value.
- `swap_global_with_constant`: Saves a global variable, sets it to a constant value, calls another routine, and then restores the original value.
- `initialize_global_pointer_and_loop`: Initializes a global pointer to a stack-allocated variable and enters a loop that increments a value and calls another routine until a certain threshold or value in a global variable is met.
- `update_digit_string`: converts a global integer value to its ASCII character equivalent and stores it in a global memory location.
- `update_string_digit`: converts a global variable to its ASCII digit equivalent and stores it in a string buffer.
- `dispatch_and_read_joystick_state`: The routine is the entry point of a one-instruction stub whose body is only shown through the decompiler, which contradicts the stated instruction count, so the evidence does not support a confident behavioural description.
- `reset_game_state_and_string_buffer`: Resets a global variable to zero and iterates through a loop to clear or initialize a memory buffer.
- `execute_complex_transfer_sequence`: A large routine that initializes a global pointer, performs bitblt transfers, and updates coordinates or memory using a sequence of other routines and text-referenced strings.
- `modify_string_characters`: Iterates through a memory range, modifies characters by adding a value, and updates a global variable or replaces a character with a space.
- `execute_repeated_subroutine`: Executes another routine a fixed number of times using a descending counter.
- `calculate_and_update_blt_offsets`: Calculates a new offset value based on a register-dependent scaling factor and updates a global variable.
- `iterate_and_update_offsets`: Iterates through a range of values and calls a routine that updates a global variable or register-related value via a pointer.
- `calculate_string_length_and_transform`: Calculates the length of a null-terminated string and performs a series of arithmetic and bitwise transformations on a value derived from that length.
- `transform_and_rotate_value`: Performs a multiplication and bitwise rotation on a value and checks a null terminator in a buffer.
- `empty_routine_5`: The routine is empty and performs no operations before returning.
- `iterate_string_and_accumulate_offset`: Iterates through a null-terminated string while calling another routine and checking a condition.
- `calculate_scaled_offset_and_loop`: Performs scaled arithmetic and checks a calculated value against a threshold before entering a loop that modifies a global variable and iterates through a table of offsets.
- `calculate_scaled_value`: Calculates a value based on a register-passed input and a division, then stores the result in a global variable before calling another routine.
- `update_blt_coordinate_offsets`: Iterates through a range of calculated or provided offset values to update a global variable or register-related value.
- `decrement_and_call`: Decrements a local variable and calls another routine until the variable reaches a certain value.
- `toggle_and_iterate_global`: Toggles a global variable and then enters a loop that updates a global variable using a value from a register and increments it.
- `copy_memory_block_2`: Copies a fixed number of longwords from one memory location to another and then clears a global variable.
- `initialize_linked_list_nodes`: Initializes a series of memory locations with a pointer-like structure and clears a global variable.
- `swap_pointer_in_list`: If a global variable is non-zero, it retrieves a pointer from a list, updates a pointer in the list, and then restores the original global variable value.
- `traverse_list_for_condition`: Traverses a linked list starting from a global variable, checking for a specific value and a certain condition at an offset, and returns a constant if a match is found.
- `update_list_node_properties`: Decrements a global variable and, if it reaches zero, initializes a memory block at a pointer retrieved from a global variable with several specific values and updates a linked list pointer.
- `find_minimum_distance_in_list`: Traverses a linked list and calculates the minimum Manhattan distance between a global pointer and certain coordinates stored in the list nodes.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_global_with_offset`: If a global variable is zero, it performs arithmetic on it; otherwise, it loads a value from an offset of the global pointer and adds a constant offset to it.
- `update_node_counter_and_link`: Decrements a counter in a structure and updates a pointer or a block of data.
- `update_linked_list_node`: Updates a linked list or a table of pointers by decrementing a counter and reassigning a pointer or a block of data.
- `update_global_state_and_copy_memory`: Updates a global variable with a calculated value and performs a memory copy or block transfer using a loop that increments a pointer and decrements a counter.
- `initialize_global_state`: Sets a global variable to zero or a specific value and performs a series of memory writes to a global buffer.
- `create_infinite_loop`: Performs an infinite loop that never exits.
- `dereference_global_pointer`: Updates a global variable with a value loaded from a pointer, where the pointer or offset is derived from the global variable itself.
- `dereference_register_pointer`: Updates a global variable with a value loaded from a pointer, where the pointer or offset is derived from a register.
- `initialize_global_state_2`: Initializes a global variable to a specific value and performs a series of memory writes to a global buffer.
- `enter_infinite_loop`: The routine executes an infinite loop that does not exit or modify any registers or memory.
- `update_global_state`: Checks if a global variable is non-zero and equal to zero, then sets it to one.
- `reset_global_variable_2`: sets a global variable to zero.
- `modify_string_bytes`: Iterates through a string backwards, adding a value to each byte and applying a conditional offset if the result exceeds a certain value.
- `apply_iterative_bitmask_to_struct`: Iterates through a source table to apply bitwise OR and AND operations to specific offsets within a destination structure, while simultaneously updating a bitmask or mask-like value via bit-shifting and negation.
- `apply_bitmask_to_table_elements`: Iterates through a source table to apply bitwise OR and bitwise AND operations to specific offsets in a destination table.
- `apply_bitmask_conditional_loop`: Calculates a source pointer based on a global variable and iterates through a table to apply bitwise masks to a destination table.
- `apply_bitmask_to_words_loop`: Iterates through a source table, applying bitwise OR and bitmasking to 16-bit values at specific offsets in a destination table.
- `update_global_with_modulo`: Updates a global variable and executes a routine with a calculated value if a certain condition is met.
- `decrement_and_loop_counter`: Initializes a register with a fixed value and decrements it within a loop until it reaches a termination condition.
- `compute_quotient_remainder`: Calculates a new value based on a doubled version of a global variable and computes a quotient and remainder relative to a global variable.
- `loop_until_bit_cleared`: Repeatedly calls another routine until a specific bit in the return value is cleared.
- `fetch_masked_value`: Calls another routine and returns a 16-bit value from a table using a masked version of the returned value as an offset.
- `update_global_with_multiplied_value`: Calculates a new value based on a doubled version of a global variable and stores it back into a global variable.
- `store_result_to_offset`: Stores a value returned by another routine into a memory location at a fixed offset from a pointer.
- `add_to_global_variable`: Returns the sum of a global variable and the return value of another routine.
- `compute_remainder_and_quotient`: Takes a 32-bit input, splits it, and returns a value composed of a quotient and a remainder relative to a global variable.
- `calculate_reciprocal_estimate`: Performs conditional checks on registers to return a fixed-point reciprocal or a constant value.
- `estimate_reciprocal_value`: Performs conditional checks on registers to return a fixed-point reciprocal estimate or a constant, or calculates a complex value using a division and a multiplication-based approximation.
- `estimate_quotient_reciprocal_2`: Performs a conditional check on two registers and returns either a fixed-point reciprocal estimate or a constant or a modified value.
- `return_constant`: Returns the constant 8192 in d0 to the caller without any other effect.
- `estimate_quotient_reciprocal`: Divides the first 32-bit input by the high word of the second, then refines the quotient with a fixed-point reciprocal estimate.
- `calculate_bit_shifted_value`: Performs a series of bit-shifting and arithmetic operations on a register-provided value, using a loop that terminates after a fixed number of iterations.
- `walk_linked_list_recursively`: Iterates over a singly linked list starting from a global head pointer, following the next-pointer stored at a fixed offset of each node, and for each node that has a non-null child pointer at a second offset it recurses into the routine itself before processing the node.
- `traverse_linked_list_2`: Traverses a linked list by following pointers and checks if a value at an offset or a secondary buffer overlaps certain bounds.
- `validate_buffer_overlap`: Checks if a range or certain byte values within a memory structure fall within specific bounds and if a secondary buffer or pointer overlaps with a primary range.
- `transform_string_buffer_data`: This recursive routine modifies a string buffer by adding a calculated offset to each byte and applying a conditional wrap-around if a certain threshold is exceeded.
- `update_string_buffer_values`: Sets a global variable and then iterates backwards through a buffer, modifying bytes based on a calculated value and a conditional offset.
- `populate_memory_structure`: Writes several values, including a masked or shifted value, into a memory structure while incrementing a counter.
- `populate_node_data_list`: Recursively populates a list of data structures or nodes with calculated or retrieved values.
- `populate_node_data_list_2`: Populates a list of data structures with values retrieved from a pointer and a constant, while updating a counter.
- `update_list_pointers_2`: Retrieves a pointer from a list and writes masked or offset values to memory while incrementing a counter and resetting it if it exceeds a certain limit.
- `clear_and_loop_d4`: Clears a data register and executes a decrementing loop based on a counter in another register.
- `update_list_pointers`: Retrieves a pointer from a list and writes a series of values, including a masked and offset value, to a memory location.
- `initialize_memory_block_3`: Writes a block of values, including a value from a global variable, into a memory area pointed to by a register.
- `decrement_and_loop_register`: Decrements a register value in a loop until it reaches a certain value.
- `initialize_memory_block_2`: Swaps a pointer in a list, updates a global variable with a multiplied value, and writes fixed or masked values to a memory block.
- `initialize_memory_block`: Writes a series of fixed or masked values to a memory block or table.
- `populate_data_structure_2`: Loops until a bit flag is cleared, then writes a constant to a global variable and a value to a memory structure offset.
- `set_structure_member`: Sets a constant value at a specific offset within a memory structure if a condition is met.
- `populate_data_structure_simple`: Writes a constant value to a memory structure offset if a boolean flag is false.
- `populate_data_structure`: Iteratively writes a series of values, including modified copies of other values, into a memory structure.
- `copy_and_modify_struct`: Copies certain fields from a structure and writes them to a new location, while calculating a new size or offset.
- `decrement_and_insert_node`: Decrements a value in a structure and, if zero, clears a pointer, otherwise attempts to insert the structure into a linked list or update a global pointer.
- `calculate_string_length_offset`: Calculates a value based on the difference between a global variable and a pointer, then checks the length of a null-terminated string to determine a final offset or return value.
- `update_list_with_offset`: Decrements a value in a structure and, if the result is non-zero and certain conditions are met, calculates a new offset and inserts the structure into a linked list.
- `update_linked_list_node_properties`: Updates a structure's size and weight-related fields, then performs a sorted insertion into a linked list based on a calculated distance or weight value.
- `calculate_and_insert_node`: Calculates a new offset and a secondary value based on a division, then inserts the structure into a linked list or updates a global pointer.
- `update_vector_coordinates_2`: Calculates a smoothed or error-corrected coordinate value and updates a structure or a linked list of pointers.
- `update_vector_coordinates`: Calculates a new position or direction using a distance-based error correction or smoothing algorithm and updates a global or structure-based coordinate.
- `modify_and_append_node`: Modifies a value in a structure based on a calculation and then attempts to append the structure to a linked list or update a global pointer.
- `update_linked_structure_geometry`: Updates a structure's spatial or dimensional parameters and, if certain conditions or counters aremet, inserts the structure into a linked list.
- `insert_structure_into_sorted_list`: Calculates a new position and scale-related value for a structure based on its spatial offset, then inserts the structure into a linked list sorted by a distance-based or offset-based key.
- `update_structure_lifecycle`: Decrements a structure's lifetime counter and, if it reaches zero, clears its pointer.
- `clear_structure_pointer`: Sets the first word of a structure to zero.
- `instantiate_and_link_structure`: Decrements a counter in a memory structure and, if zero, initializes a new structure with a fixed set of parameters and a loop-based reset.
- `update_structure_geometry_and_link`: Updates a structure's spatial or dimensional parameters and, if certain conditions or counters are met, inserts the structure into a linked list or table of pointers.
- `update_linked_list_node_2`: Decrements a value in a structure and, if zero, updates a global variable or inserts a pointer into a linked list structure.
- `swap_pointer_offset_value`: Swaps a value stored at an offset in a global variable with a value stored at an offset in a pointer-based or register-based structure.
- `traverse_and_modify_list`: Iterates through a linked list to execute a routine, then performs a second pass to modify list pointers and execute another routine.
- `traverse_linked_list`: Iterates through a linked list, calling another routine if a certain condition is met and executing a function pointer or routine found in the list structure.
- `iterate_range_with_check`: Checks if a calculated range or value is non-negative, then iterates through a sequence, updating a global variable and checking for a termination condition.
- `read_conditional_offset`: Checks the sign of a global variable to either return a value from an offset or iterate through a loop that updates a global variable.
- `return_immediately_2`: This routine is empty and performs no operations before returning a value.
- `iterate_if_positive`: Checks if a global variable is non-negative, then iterates through a sequence, updating a global variable and checking for a termination condition.
- `iterate_range_with_offset`: Checks if a difference between a global variable and a value in a structure is non-negative, then iterates through a sequence, updating a global variable and checking for a termination condition.
- `execute_complex_range_loop`: Performs a complex check involving division and overflow detection using values from a structure, then executes a loop that iterates through a sequence while updating a global variable.
- `apply_bitmask_to_struct_range`: Calculates a bitmask or bit-manipulated value based on a division-derived scale and applies it via a bitwise AND to specific members of a structure or array, iterating through a range of memory-mapped or pointer-offset locations.
- `apply_bitmask_to_memory_range`: Calculates a pointer and a bitmask, then iterates through a memory range applying a bitwise AND operation to a series of memory-mapped or global variables.
- `interpolate_clamped_values`: Performs a series of conditional linear interpolations or extrapolations between or against fixed boundary constants.
- `calculate_scaled_loop_offset`: Calculates a scaled offset and checks a threshold before entering a loop that modifies a global variable and an iterator.
- `check_scaled_threshold`: Performs a calculation involving a constant and a value from a structure, then potentially sets a global variable to a constant.
- `initialize_data_tables`: Clears, populates, and resets several memory-resident tables or arrays using hardcoded offsets and loop-driven assignments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: The routine contains no instructions and returns immediately.
- `empty_routine_2`: The routine contains no instructions and performs no operations.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `empty_routine_4`: The routine contains no instructions and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `decompress_data_stream`: Copies bytes from a source to a destination using length headers and offset values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `expand_compressed_buffer`: Iteratively reads a data stream to perform block copies and offset-based transfers into a destination buffer.

### vaxine (raw code file, load address unknown)

- `copy_memory_block`: Copies a block of words from a source address to a destination address in reverse order.
