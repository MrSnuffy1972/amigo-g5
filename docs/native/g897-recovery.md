# Game 897 (mig-29 - fulcrum (1991)(domark)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00897). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 307 of 307 named (0 author symbols), 307 of 307 described (9 by rule, 296 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2430 of 2430 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12295 of 12295 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 537069 |
| code | 38340 |
| data | 240640 |
| padding | 12431 |
| unused | 72704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| MiG-29 - Fulcrum (1991)(Domark).adf:mig | 2 | 167 | 7 |
| stack | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| MiG-29 - Fulcrum (1991)(Domark).adf:intro | 2 | 11882 | 294 |
| MiG-29 - Fulcrum (1991)(Domark).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mig-29 - fulcrum (1991)(domark)/MiG-29 - Fulcrum (1991)(Domark).adf: blocks 11-21 (bytes 5632-11264) hold ASCII text
- mig-29 - fulcrum (1991)(domark)/MiG-29 - Fulcrum (1991)(Domark).adf:mig: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- MiG-29 - Fulcrum (1991)(Domark).adf:intro

## Copy protection found

- none found

## Functions


### MiG-29 - Fulcrum (1991)(Domark).adf:mig

- `parse_amigadot_hunk_relocations`: Checks for a specific magic constant in a data stream, then parses a table of relocation or data-copying instructions by calculating offsets and updating pointers or copying blocks of bytes.
- `accumulate_32bit_from_bytes`: Reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `decode_and_copy_bytes`: The routine decodes a byte from a global variable using a bit stream produced by another routine and then copies a block of bytes from one memory area to another using two pointers, decrementing a counter until it reaches -1.
- `test_and_restore_stack_pointer`: Checks if the least significant bit of a register is set, and if not, restores a byte from the stack to that register before returning.
- `decrement_until_minus_one`: The routine calls another routine repeatedly, decrementing a short result until it reaches -1, then checks a byte from a global variable table indexed by the result plus one.
- `process_table_and_update_pointers`: Checks a register and calls another routine in a loop.
- `decrement_and_branch_on_zero`: Decrements a 32-bit register in a loop and if the result reaches zero, it updates a register with a pointer and returns.

### stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_register_to_register`: Moves the contents of d1 to d0 and then branches to a fixed offset.

### endcli

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `transfer_register_value`: Copies a longword from one register to another and branches to another routine.

### MiG-29 - Fulcrum (1991)(Domark).adf:intro

- `initialize_program_environment`: Initializes a global memory area, calls a routine to allocate memory, and checks if the allocation succeeded.
- `cleanup_resource_list`: Checks a pointer in a structure, calls a destructor if present, and then conditionally calls library functions to free memory or close libraries based on the presence of other pointers in the same structure, otherwise it returns.
- `free_linked_list_nodes_2`: Walks a linked list held in a global pointer, invoking a destructor stored at a fixed offset within each node.
- `parse_command_line_args_jump`: Performs an indirect jump to a target address, acting as a jump table or redirection for the relocation or command-line argument parsing process.
- `cleanup_display_context_3`: Performs a sequence of cleanup routine calls, clears a global variable, and manages the stack while potentially calling a routine to stop audio and clear a global variable.
- `execute_callback_chain_loop`: Sets the DMA enable register to a specific value, calls several routines that initialize or clear memory, and executes a routine via a pointer offset.
- `traverse_linked_list_and_recursively_call`: Traverses a linked list starting from a global variable, checking if a bitmask at offset 19 of the current structure matches a specific pattern and if a 16-bit value at offset 4 does not equal a 16-bit value at offset 8, then calls itself recursively for each valid node before finally returning.
- `process_recursive_vector_list_2`: Recursively processes a list of data structures by performing fixed-point arithmetic and vector transformations based on a count value, branching into different sub-routines depending on the magnitude of the input value.
- `recursively_calculate_bounds`: Performs recursive calls with modified integer values and clamps the results against a global variable.
- `execute_callback_sequence_2`: Executes a series of function calls using a value from a global variable and several constant addresses.
- `process_conditional_loop`: Performs a series of loops that update a local array and a global variable based on the results of various subroutine calls.
- `execute_sequence_of_commands`: Executes a series of specific function calls with various immediate values and register contents to perform a sequence of operations.
- `execute_callback_sequence_3`: Executes a series of function calls including a conditional loop, state resets, and memory storage operations, and performs pointer-based list processing and toggles a global variable via a table lookup.
- `decode_input_bitmask`: Iterates through a byte array to expand bits into a linear flag buffer, then calculates a composite input code based on global state flags.
- `reset_and_update_globals`: Modifies a global variable and, if it reaches a specific value, executes a function and performs bitwise OR operations on a series or series of memory locations.
- `open_keyboard_device`: Opens the keyboard.device using Amiga OS library calls, initializes message structures, and sets up device-specific fields in a global structure.
- `update_object_state`: Updates several fields within a structure, including a function pointer and numeric values, based on a global pointer.
- `cleanup_structure_data`: Executes several function pointers stored within a structure and updates a field within that structure.
- `cleanup_display_context_2`: Closes a device and invokes cleanup routines to reset structure fields and release resources associated with the display context.
- `store_mouse_coordinates`: Accepts three word-sized parameters and stores them into specific global variables, updating the current mouse position and delta values.
- `check_input_state_flag`: Calls input processing routines and returns the value of a global flag variable, resetting it if it is non-zero.
- `get_valid_key_code`: Loops calling input routines until a valid key index is found, then returns the corresponding character from a global string table.
- `process_callback_chain_loop`: Retrieves a value from a global variable and executes a routine via a pointer offset; if the value is greater than a fixed threshold, it enters a loop that repeatedly calls another routine in decrements of thirty thousand until the remainder is processed.
- `call_free_mem_pointer`: Calls a function via a pointer located at an offset from a global variable if that global variable is non-zero.
- `set_global_flag`: Sets a global variable to one if it is currently non-zero.
- `silence_and_stop_music`: Clears a global variable and calls a routine that stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `call_create_message_node`: Calls the routine at a fixed address that performs message node creation.
- `call_set_message_type_and_target`: Calls the routine at a fixed address that sets message type and target.
- `jump_to_r3`: Performs a long jump to the routine at address 9d88.
- `reset_display_context`: If a field at offset 10 of a structure in a1 is non-zero, it calls a function pointer from a global variable, then writes certain reset values to offsets 8 and 14 of the structure, and calls two other functions from a global variable and a pointer within a global variable.
- `jump_to_routine`: Performs a long jump to the routine at address 9198.
- `reset_context_fields_2`: Writes reset values to offsets 8, 14, and 18 of a structure in a1 and calls a function pointer located at an offset from a global variable.
- `process_audio_parameters`: Performs complex calculations on data within a structure to update various memory locations and hardware audio volume registers.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_register_and_counter`: Increments a counter in a global variable and, if it reaches six, resets it and executes a sequence of calls to a routine that modifies audio or hardware registers.
- `update_object_kinematics`: Updates an object's position or coordinate values based on a mode flag and velocity data.
- `update_sprite_attributes`: Reads a value from a memory location to determine an index, then fetches sprite coordinate and frame data from a table to update a sprite structure.
- `copy_data_block`: Copies a block of up to 30 bytes from one memory location to another and increments a global index.
- `decrement_global_index`: Decrements a global variable and adjusts it by a fixed offset if it is not zero.
- `sum_triplets_with_scaling`: Calculates the sum of three sets of three values and stores the results in a table, applying a scaling factor if certain conditions are met.
- `accumulate_fixed_point_products`: Performs multiple fixed-point multiplications and sums the results into three consecutive integer slots in a destination table.
- `process_fixed_point_division_remainder`: Performs a fixed-point division and checks if the resulting quotient exceeds a specific threshold.
- `average_and_store_pairs`: Calculates the sum of pairs of 16-bit values from a table, shifts the result right by nine bits, and stores the values back into the table.
- `update_table_elements`: Sums specific pairs of 16-bit values from a table, shifts the results right by nine bits, and updates the table at specific offsets.
- `process_table_averages`: Calculates the sum of pairs of 16-bit values from a table, shifts the result right by nine bits, and writes them back to the first six elements of the table.
- `calculate_complex_sum`: Performs fixed-point division and multiplication to calculate and store two 16-bit values into a destination pointer based on a conditional check of a sum.
- `compare_coordinate_differences`: Compares the difference between two pairs of 16-if values and returns a boolean result.
- `compare_fixed_point_values`: Compares two 32-bit values and adjusts a destination table using fixed-point multiplication and subtraction based on a conditional check of a sum.
- `adjust_fixed_point_range`: Compares two 32-bit values and modifies a destination table using fixed-point multiplication and subtraction if certain conditions are met.
- `modify_fixed_point_elements`: Compares two 32-bit values and updates specific elements in a destination table using fixed-point multiplication and subtraction.
- `interpolate_vectors`: Calculates a midpoint between two sets of three-dimensional coordinates through an iterative loop until a threshold or a certain condition is met, then writes the result back to a table.
- `interpolate_vectors_alt`: Performs an iterative midpoint calculation between two vectors, using a different initial summation for theif first component, and writes the result back to a table.
- `compare_coordinate_vectors`: Compares the absolute values of elements within two arrays and performs recursive calls to sort or process them based on their magnitude.
- `compare_and_sort_vectors`: Compares the absolute values of elements within two vectors and recursively calls itself to sort or reorder them based on their magnitude and sign.
- `process_vector_intervals`: Performs recursive comparisons and processing on multiple parameters, including checks for sign and magnitude of values.
- `process_vector_parameters`: This routine invokes a function to perform recursive comparisons and processing on multiple parameters, including checks for the sign and magnitude of values.
- `split_numerical_interval`: Recursively divides a numerical interval into smaller sub-intervals based on a global variable.
- `execute_recursive_logic_sequence`: Performs a sequence of instructions that includes a loop, and calls several subroutines, including one that performs a multiplication or other logic.
- `execute_complex_vector_logic`: Performs a wide variety of mathematical and logical operations including coordinate comparisons, fixed-point arithmetic, vector sorting, and remainder calculations.
- `process_vector_list`: Iterates through a list of elements, performing fixed-point arithmetic and vector transformations while recursively calling other routines based on a global variable.
- `process_recursive_vector_list`: Iterates through a list of data structures based on a count, processing each element with a specific routine and advancing through memory, then conditionally executes another routine based on values found at specific offsets.
- `copy_block_and_set_ptr`: Copies a block of data from a source pointer to a fixed destination address and stores that destination address into a global variable.
- `store_params_and_recurse`: Stores two 32-bit parameters into four consecutive 16-bit global memory locations and then calls another routine passing the upper 16-bit halves of the original parameters.
- `compare_signed_words`: Compares two 32-bit signed values represented as pairs of 16-bit words against a global value and returns a status code.
- `adjust_32bit_values_by_global`: Compares the high words of two 32-bit values against a global variable and performs arithmetic on the low words and high words of those values based on the comparison results, returning a status code.
- `split_interval_by_value`: Recursively partitions a numerical range by checking if a global variable or certain fixed boundary values fall within the current interval, adjusting the interval boundaries and calling itself to further subdivide the range.
- `execute_function_via_pointer`: Retrieves a function pointer from a table and executes it with a value derived from another table entry.
- `toggle_dma_enable`: Updates a global variable based on its current value and executes a function pointer retrieved from a table using a value from another table, while setting bit 15 of the DMA controller register.
- `iterate_and_call_functions`: Loops through a table to execute function pointers and updates the table entries based on results.
- `execute_conditional_callbacks`: Executes several function pointers at various offsets from a global variable, including a loop that checks a table for non-zero values before calling a function.
- `open_graphics_intuition_libs`: Opens the graphics and intuition libraries using exec.OpenLibrary, displaying error messages if they fail to unable load, and initializes a table of function pointers.
- `initialize_registers`: Calls a function via a pointer and then sets several consecutive global values to specific constants.
- `populate_table_data`: Iterates through two nested loops to store a global variable and derived values into a table, while also updating specific byte and word entries within that table.
- `process_callback_table`: Manages memory and system resources before iterating through tables to execute function pointers and store derived values into a global table.
- `shutdown_graphics_system`: Executes cleanup functions for graphics resources, sets a global variable to a specific value, and enables DMA channels by writing to the DMACON register.
- `call_functions_on_condition`: Executes two functions via pointers if the first parameter is less than the second parameter.
- `clamp_and_call_indirect`: Adjusts two parameters to ensure they fall within a range defined by a global variable and then calls a function via an indirect pointer from a table.
- `clip_range_and_recurse`: Calculates new bounds for two parameters based on a global variable and a range, then calls itself recursively with the updated values.
- `call_clamped_function`: Calls another routine using a parameter adjusted by a value from a global variable.
- `transform_bit_packed_data`: Iterates through a collection of bit-packed values, using each bit from a set of source words to index into a lookup table to extract new bit patterns, and then reassembles and writes the extracted bits back to the destination pointers.
- `copy_and_iterate_table`: Copies 32 bytes from a source into a destination pointed to by a global variable and then executes two function pointers.
- `call_functions_on_range`: Checks if a global value falls within a specified range and executes two functions via pointers if the condition is met.
- `compare_and_call_function`: Compares two ranges and either calls a function directly or executes two specific functions via pointers if the ranges do or do not match.
- `execute_complex_logic`: Contains a loop that calls several library functions and other routines while performing pointer-based table lookups and area-related operations.
- `update_table_bounds_and_copy`: Iterates through a table or range, performing conditional checks on pointer-based table lookups and copying data or updating a counter, then calls another routine with a pointer to a structure and a counter on the stack.
- `process_complex_logic`: Performs conditional 32-bit value comparisons against a global or local value and executes a loop that copies or modifies data in a table or buffer using pointer-based offsets.
- `parse_string_data`: Compares an input string against a constant and, if a mismatch is found, processes the string by searching for specific delimiter characters and performing recursive calls.
- `parse_bmp_load_view`: Calls a routine to parse BMP image data and another to copy buffers for loading a view, then conditionally invokes an additional routine based on a parameter value.
- `execute_function_pointer`: Loads a pointer into a register and calls a routine via that pointer, then returns.
- `process_string_output`: Iterates through a null-terminated string, calling a routine for each character and incrementing a global variable, while resetting that variable upon encountering a newline character.
- `update_global_word`: Stores two consecutive bytes from a passed parameter into a global variable.
- `execute_callback_loop`: Allocates memory and iterates through a table to execute several function pointers based on a global value.
- `sum_function_results`: Executes a function via a pointer from a global variable and returns the sum of two results from other function calls.
- `call_graphics_setup_routines`: Invokes several function pointers from a global structure and iterates through a null-terminated string without modifying it.
- `free_list_elements`: Iterates through a list of pointers provided in a parameter and calls a function for each non-zero entry before calling another function.
- `allocate_and_check_block`: Calls a routine to allocate a memory block and copy data.
- `handle_callback_chain`: Iterates through a global table of structures to check bitwise flags and executes function pointers based on specific character inputs.
- `free_entity_slot`: Validates an entity pointer, clears its flags and references, and decrements the global active count.
- `increment_and_fetch_byte`: Increments a counter in a structure and retrieves a byte from a table using an offset, while handling a reset condition when a threshold is met.
- `copy_bytes_with_threshold`: Copies bytes from a table to a destination buffer using an incrementing index, while checking a counter against a threshold and handling structure resets.
- `jump_to_address_3`: Jumps to the address stored in a register.
- `jump_to_address_2`: Performs an indirect jump to a target address stored in a register.
- `jump_to_r1`: Performs a long jump to the routine at address 9b00.
- `jump_to_r2`: Performs a long jump to the routine at address 9b48.
- `jump_to_address`: Performs an indirect jump to a fixed address.
- `decode_compressed_data`: Decompresses data from a stream into a buffer using run-length encoding logic based on control bytes.
- `load_bmp_image_data`: Parses a BMP file stream by reading headers and skipping non-essential chunks, then extracts pixel data from the body, optionally converting 24-bit color values to a 16-bit format, and returns the processed image buffer.
- `check_byte_and_return`: Checks if a byte at an offset of a certain address is zero, otherwise it returns the value in a0 in d0 and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_table_loop`: Iterates through a range defined by two registers, calling functions pointed to by a table indexed by a base pointer and an offset.
- `apply_mask_and_branch_4_2_2`: Calculates an address, retrieves a value from a table, and then enters a loop that applies a bitmask to a memory location based on a bit in a register, or otherwise or a value into a memory location.
- `fetch_table_entry`: If a register contains a value less than 16, it shifts that value left by 1, uses it as an offset to load a word from a table into d0, and otherwise jumps to another routine.
- `apply_mask_and_branch_4_2`: Sets d0 and d4 to a certain value, then checks bit 0 of d3 and otherwise or a word from d0 into a memory location pointed to by a1 and a3.
- `apply_mask_and_branch_4`: Performs a bitwise AND using a register and a pointer, checks bit 1 of a register, and otherwise or a word from d0 into a memory location before branching to another routine.
- `apply_mask_and_branch_3`: Performs a bitwise AND using a register and a pointer, then checks bit 2 of a register and otherwise or a word into a memory location before branching.
- `apply_mask_and_branch_2`: Performs a bitwise AND using a register and a pointer, checks bit 3 of a register, and otherwise or a word into a memory location before branching.
- `update_table_and_branch`: Performs a bitwise AND, increments a pointer, and contains a loop that checks a register against a constant and branches back to an earlier instruction or a different routine.
- `restore_registers_and_return`: Restores d2-d6 and a1-a2 from the stack and returns.
- `increment_pointer_offset`: Increments a 32-bit value located at an offset of 8 bytes from a base pointer by 4.
- `increment_pointer_offset_c`: Increments a 32-bit value located at an offset of 12 bytes from a unable to be determined base pointer by 4.
- `initialize_and_iterate_table`: Loads several values from a structure, saves registers, calculates a table address, calls two other routines, and then enters a loop that checks a register, decrements a value in a structure, and calls another routine or branches back to the start of the loop.
- `iterate_and_call_dispatch`: Checks if a register is greater than zero, otherwise decrements a value at an offset, and contains a loop that calls a routine and branches back to the start.
- `iterate_range_with_calls`: If d6 is greater than or equal to d5, it sets d6 to d5, calls a routine, increments a pointer in a1 by 28, decrements d3 and d1, and adds d2 to d5 and d4 to d6 before looping back to a previous instruction.
- `call_dispatch_loop`: If d6 is less than d5, it sets d6 to d5, calls the dispatch table loop routine, and restores registers d2 through d7 and d3 through d6 before returning.
- `initialize_structure_and_branch`: Loads a structure from a pointer, initializes several registers with cleared or swapped values, and calls two routines before conditionally decrementing a value in a structure and branching.
- `clamp_address_range_2`: Checks if a register contains a value greater than zero, otherwise decrements a value at an offset from a pointer, and loops if a certain condition is met or calls another routine.
- `apply_mask_and_branch_5`: Compares two registers and branches if the first is not less than the second; otherwise, it updates registers, calls another routine, and performs arithmetic on several registers before branching back to an earlier instruction.
- `clamp_address_range`: Checks if a certain address is lower than a target address, and if so, sets that address to the target value before calling a routine that applies a bitmask to a memory location and returns.
- `check_word_and_branch`: Tests if a word-sized value at a certain memory location is zero and, if not, branches to another routine; otherwise, it returns.
- `apply_bitmask_to_memory_range`: Calculates a range of memory or a table-driven mask and, if certain bounds checks pass, iterates through a sequence of memory locations, applying a bitwise AND or OR using a value or a mask-table-lookup, then returns.
- `fetch_scaled_table_entry`: Checks if a value in a register is less than 16, and if so, calculates an offset by doubling that value, uses it to fetch a word from a table located at an offset of 34 bytes from a pointer in a2, and then branches to another routine.
- `apply_mask_and_branch_custom_3_2_2`: Sets d0 and d4 to a constant, inverts d4, checks bit 0 of d3, and otherwise or a word from d0 into a memory location pointed to by a1 and a3.
- `apply_mask_and_branch_custom_3_2`: Performs a bitwise AND using d4 and a pointer, checks bit 1 of d3 and otherwise or a word from d0 into a memory location pointed to by a1 and a4.
- `apply_mask_and_branch_custom_3`: Performs a bitwise AND using a register and a pointer, checks bit 2 of a register, and otherwise or a word from d0 into a memory location before branching to another routine.
- `apply_mask_and_branch_custom_2`: Performs a bitwise AND using a register and a pointer, then checks bit 3 of a register and otherwise or a word into a memory location before branching.
- `apply_mask_and_branch_custom`: Performs a bitwise AND using a register and a pointer, checks bit 4 of a register, and otherwise or a word into a memory location before branching.
- `updates_table_index_and_pointer`: Reads a word from a table using a pointer and an offset, increments the pointer by two, and then checks if a value in d2 is less than sixteen to decide whether to branch to another routine or decrement d2 and branch back to the start of this routine.
- `restore_registers_and_return_large`: Restores d3-d5 and a1-a6 from the stack and returns.
- `check_word_and_return`: Tests if a word at a certain memory location is zero and otherwise returns immediately.
- `update_mask_table_range`: Calculates a range of memory or table locations based on certain bounds and then, if certain conditions are met, loops through these locations to bitwise AND or OR a value into them, where the bitmask or value is determined by a lookup in a table.
- `fetch_scaled_table_entry_2`: Checks if a value in d2 is less than or equal to 16, and if so, calculates an even offset by doubling it, then loads a word from a table located at a 34-byte offset from a pointer in a2 into d0; otherwise it returns.
- `mask_and_conditionally_update_table_3_2`: Sets d0 and d4 to a certain value, then checks bit 0 of d3 and otherwise or a word from d0 into a memory or table location pointed to by a1 and a3.
- `mask_and_conditionally_update_table_3`: Applies a bitmask to a word read from a table using an offset in a register, checks bit 1 of a register, and if clear, ors a word from d0 into a memory location before branching to another routine.
- `mask_and_conditionally_update_table_2`: Applies a bitmask to a word read from a table using an offset in a register, checks bit 2 of a register, and if clear, or-s a word from d0 into a word in a table using an offset in a5; otherwise it returns.
- `mask_and_conditionally_update_table`: Applies a bitmask to a word read from a table using an offset in a5, checks bit 3 of a register, and if clear, or-s a word from d0 into a table at an offset in a6 before branching to another routine.
- `update_table_pointer_and_index`: Reads a word from a table using an offset in a6 and a mask in d4, increments the table pointer in a1 by two, and if the value in d2 is greater than ten, subtracts ten from d2 and repeats the routine.
- `restore_registers_and_return_small`: Restores d3-d4 and a1-a6 from the stack and returns.
- `apply_mask_and_load_branch_3`: Performs a bitwise AND using a register and a pointer, loads a longword from a structure into a register, checks bit 0 of a register, and otherwise ors a word from d5 into a memory location before branching to another routine.
- `apply_mask_and_load_branch_2`: Performs a bitwise AND using a register and a pointer, loads a longword from a structure into a register, then checks bit 1 of a register and otherwise or a word into a memory location before branching.
- `apply_mask_and_load_branch`: Performs a bitwise AND using a register and a pointer, loads a longword from a structure into a register, checks bit 2 of a register, and otherwise or a word into a memory location before branching.
- `apply_mask_and_branch`: Performs a bitwise AND, loads a value from a global variable into a register, checks a bit in a register, and otherwise performs a bitwise OR before branching.
- `apply_bitwise_and_mask_2`: Performs a bitwise AND between a 16-bit value in d4 and a word at a memory location, loads a new value into d3 from a table, and if bit 4 of d0 is clear, or even if it is set, or even if it is set, it ors d5 into the memory location and jumps to another routine.
- `apply_bitwise_and_mask`: Performs a bitwise AND between a 16-bit value in d4 and a 16-bit value at a memory location pointed to by a1 and d3, then returns.
- `apply_mask_and_load_branch_alt_2`: Negates a word in d4, loads a longword from a structure offset 52 in a0 into d3, checks bit 0 of d0, and otherwise ors a word from a1 with a pointer offset d3 into a destination and branches.
- `apply_mask_and_load_branch_alt`: Performs a bitwise AND using a register and a pointer, loads a longword from a structure into a register, checks bit 1 of a register, and otherwise or a word into a memory location before branching.
- `apply_mask_and_branch_alt`: Performs a bitwise AND, loads a value from a global variable into a register, checks a bit in a certain register, and otherwise performs a bitwise OR before branching.
- `mask_memory_word_2`: Performs a bitwise AND between a 16-bit value in d4 and a word at a memory location, loads a new value into d3 from a table, and if bit 3 of d0 is clear, or even if it is set, or even if it is set, it ors d5 into the memory location and jumps to another routine.
- `mask_memory_word`: Performs a bitwise AND between a 16-bit value in a register and a word at a memory location pointed to by a register, then restores registers and returns.
- `check_global_word_and_return`: Tests if a word in a global variable is zero; if not zero, it returns, otherwise it returns immediately.
- `validate_and_fetch_table_index`: Checks if two 16-bit values provided via the stack fall within a specific range defined by a global structure, and if so, calculates a table offset and calls another routine with a value fetched from a table.
- `check_value_and_return`: Tests a word at a specific memory address and otherwise returns immediately.
- `validate_coordinate_range`: Stores two 16-bit values into global memory and checks if they fall within a specific bounding box defined by two other global variables.
- `store_two_word_params`: Stores or extends or copies four 16-bit or 32-bit values from the stack into a global RAM area at a fixed offset.
- `copy_five_word_block`: Copies five consecutive 32-bit words from a source pointer into a contiguous block of global RAM memory.
- `copy_four_word_block`: Copies four consecutive 32-bit words from a source pointer into a contiguous block of global RAM memory.
- `multiply_two_words`: Multiplies two 16-bit values from the stack and returns the result in d0.
- `multiply_and_divide_words`: Multiplies two 16-bit values, then divides the result by a third 16-bit value, and returns the result in d0.
- `transform_vector_by_matrix`: Computes the dot product of a 3-element vector with the rows of a 3x3 matrix stored in a table, adding an offset from the table to each result component.
- `parse_complex_ascii_numeric_string`: Parses a string containing numeric values, handling integer components, decimal points, and scientific notation suffixes.
- `parse_formatted_expression_string`: Parses a string containing numeric values, signs, and format specifiers to evaluate or convert data.
- `parse_format_string`: Walks a format string character by character; when it finds a percent sign, it calls another routine with the remainder of the string and a stack slot, and if that call returns null, it invokes a function pointer from a table.
- `check_register_bounds_2`: Sets d0 to a constant, loads d0 and d1 from a structure offset 8 in a5, clears d2 and d3, and calls another routine, otherwise it loads d0 and d1 from a structure offset 8 and 12 in a5 and branches.
- `merge_and_test_bits_2`: Loads d0 and d1 from a memory location, calls another routine, restores d2 and d3 from the stack, clears a register, and returns.
- `parse_complex_string`: Parses a string by checking for specific characters and calling another routine to process segments, returning a count or a specific error value.
- `load_byte_from_pointer`: Pushes d7 to the stack, loads a byte from a pointer stored in a4 into d7, updates the pointer in a4, and branches or jumps based on the loaded byte.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `update_and_copy_buffer`: Increments a global value, decrements another, and loops while copying a byte from a register to a pointer-based destination before branching to another routine.
- `invoke_memory_callback_chain`: Calls a routine that manages memory allocation and executes a sequence of chained callbacks retrieved from a global variable.
- `execute_callback_sequence`: Clears a value at a memory offset, executes a routine with a pointer and stack reference, and returns a value from that same memory offset.
- `write_byte_to_offset_2`: Pushes a register to the stack, loads a new address into a3, loads a word from that address into d0, decrements d0, and if d0 is zero, or if d0 is not zero, or if d0 is not zero, it writes a constant to a global variable and jumps to another routine.
- `write_byte_to_offset`: Stores a single byte into a specific offset relative to a register value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_call_subroutine`: Copies several longwords from a source structure to a destination structure, calls a routine with a pointer, and contains a conditional branch that checks a register or jumps to a different routine.
- `load_from_offset`: Loads a 64-bit value from a memory location calculated by adding an offset to a value held in a register.
- `format_numeric_string`: Converts a 64-bit integer into a decimal-encoded string of ASCII characters, handling sign detection and populating a character buffer.
- `decrement_and_loop`: Decrements a value in a register until it reaches zero.
- `multiply_64bit_by_ten`: Multiplies a 64-bit value held in two 32-bit registers by ten, returning the high word from the upper input and carry bits and the low word as the lower input times ten.
- `shift_bits_into_decimal_digits`: Shifts a 64-bit value left one bit at a time for a certain count, carrying each bit into a decimal digit accumulator and subtracting ten with a carry into the high word whenever the digit exceeds nine.
- `parse_ascii_floating_point`: Parses an ASCII string representing a floating-point number, handling optional signs, decimal points, and scientific notation exponents, and stores the resulting numeric value and a status flag into a structure via pointers.
- `update_callback_and_counter`: Checks a value in a structure, executes a function pointer from that structure, and decrements a counter, potentially setting a bit in another structure when the counter reaches zero.
- `transform_integer_bit_pattern`: Performs complex bitwise shifts, rotations, and masking on a 32-bit value based on a modified offset or temporary or calculated value, using conditional branching to apply different bit-manipulation logic or recursively calling another routine with a modified parameter or a new pointer-based argument.
- `shift_left_register_high`: Shifts a 16-bit value from a register left by thirty-two bits and returns the result.
- `return_shifted_register_value`: Calls another routine and returns a 64-bit value where a certain register value is shifted into the upper bits.
- `call_with_parameter`: Passes a constant value to a function pointer and returns a bitwise modified value.
- `set_constant_return`: Calls a function pointer with a constant and returns a fixed 64-bit constant.
- `modify_and_swap_bits`: Checks a bit in an input, conditionally calls another routine, and returns a bit-swapped version of the input.
- `apply_table_rotation`: Uses a table and complex bitwise rotations or shifts to calculate and return a new value.
- `merge_and_test_bits`: Merges a 32-bit value from d1 into a 32-bit value from d0 on the stack, then checks if the lower 8 bits of d0 are zero before clearing those bits and returning.
- `rotate_bits_left`: Performs a bitwise left rotation on a 32-bit value for ten iterations.
- `execute_complex_bit_manipulation`: Performs a complex sequence of bit-shifting, bit-testing, and conditional calls to another routine based on register or memory values.
- `update_and_call_function`: Stores a 16-bit value into a table at an offset from a pointer and executes a function pointer retrieved from that same table.
- `clear_registers_and_call_input_processor`: Clears the lower byte of a register, saves a list of registers to the stack, calls a routine that swaps words in registers and checks against a constant, restores the saved registers, and returns.
- `process_bit_swapped_input`: Swaps the high or low word of d0 and d2, checks if the resulting d0 or d2 values are less than a certain constant, and if d1 is non-zero, calls another routine or jumps to a routine that loads d2 and d3 into d0 and d1.
- `transform_and_jump_value_2`: Compares d7 and d2, checks if d2 equals a certain constant, checks if d3 is zero, and otherwise loads d2 and d3 into d0 and d1 and jumps to a constant address.
- `jump_to_constant_function`: Performs a jump to a function pointer that returns a fixed 64-bit constant.
- `transform_and_jump_value`: If d2 and d3 are non-zero, it calls a function with a value pushed onto the stack, then calculates a bitwise XOR of d4 and a constant, swaps the upper and lower words, and returns.
- `transform_and_jump`: Performs a jump to a function pointer that returns a bitwise modified value.
- `compare_and_branch_on_value`: Compares d7 and d2, and if d2 equals a certain constant, moves d2 and d3 into d0 and d1 before jumping to another routine.
- `shift_register_left_large`: Shifts a 16-bit value from a register left by forty-eight bits and returns the result.
- `filter_register_value`: Checks if d7 or d0 or d1 or d2 or d3 or d5 contains certain values and jumps or returns based on those checks.
- `shift_register_left_high`: Shifts a 16-bit value from a register left by thirty-two bits and returns the result via a jump to another routine.
- `apply_table_rotation_2`: Calls a routine and then branches to another routine.
- `modify_registers_and_jump_2`: Performs bitwise XOR and logical operations on registers and branches to another routine if certain registers are non-zero.
- `swap_registers_and_branch`: Swaps several 32-bit registers and calls another routine before branching to a relative address.
- `modify_registers_and_jump`: Performs arithmetic on registers and jumps to a relative address if a certain condition is met.
- `calculate_scaled_division_threshold_2_2`: Performs a complex series of bit-shifts, multiplications, and signed 16-bit divisions on register-based operands to calculate a new threshold value, then branches or jumps to another routine.
- `calculate_scaled_division_threshold_2`: Performs a signed 16-bit division using a modified or scaled dividend, then updates a threshold value in a register and jumps to another routine.
- `calculate_scaled_division_threshold`: Performs a signed 16-bit division using a modified or scaled dividend, then updates a threshold value in a register and jumps to another routine.
- `check_register_bounds`: Checks if d0 is negative, or if d2 is negative, or if d0, d1, or d3 are non-zero, or if d2 equals a specific constant, otherwise it returns.
- `compare_and_branch_on_registers`: Compares two 32-bit registers and branches if not equal, then compares two other 32-bit registers and branches if the carry flag is set or if not equal.
- `check_register_values`: Tests if d2, d3, or d1 are non-zero or if d0 equals a specific constant, otherwise it returns.
- `compare_long_registers_and_branch`: Compares two 32-bit registers and branches or checks a third comparison, otherwise it returns.
- `set_ccr_bit_three`: Sets bit 3 of the condition code register and returns.
- `clear_ccr_bits`: Clears all bits in the condition code register and returns.
- `modify_d5_threshold_2`: Compares d0 to a constant, otherwise shifts d0 right and d1 right, and checks if d5 is negative, otherwise it branches.
- `modify_d5_threshold`: Subtracts a constant from d5, and if d5 is greater than or equal to a certain value, it negates d5, shifts it right, adds a constant, and checks if it is less than or equal to a certain value before jumping to a fixed address.
- `manipulate_bit_stream`: Performs bitwise shifts and logical operations on register contents while decrementing a counter.
- `process_and_branch_on_value`: Performs bitwise shifts and XORs on d0, d1, and d2, then checks if a value plus d5 exceeds a certain threshold before branching or calling another routine.
- `tiny_eor_swap_rts`: Small routine of 3 instructions that exclusive-ors d4 into d0, then swaps the halves of d0, then returns.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `call_indirect_via_global`: Calls a function pointer taken from a global variable without performing other operations.
- `validate_table_entry_index_2`: Checks if an index is within bounds and if the corresponding entry in a global table is non-zero.
- `calculate_string_length_and_buffer`: Calculates the length of a string, then iterates through the string to either call another routine or copy characters into a global buffer while decrementing a global counter.
- `parse_command_line_args`: Tokenizes a string by splitting on whitespace and handling quoted sections with escape sequences, storing pointers to the resulting tokens in a global table.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `jump_to_address_4`: Performs a single jump to a constant address.
- `allocate_memory_block`: Checks a block's size and flags, then either recursively allocates a new block or updates a pointer and clears certain memory offsets.
- `manage_memory_callbacks`: Performs memory allocation and executes a sequence of chained callbacks based on a global variable and retrieved pointers.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `execute_pointer_jump`: Performs a jump to an address stored in a register.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts uppercase ASCII characters to lowercase based on a flag in a global lookup table.
- `copy_string_with_limit`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy if it exceeds a specified maximum length and ensuring null-termination.
- `copy_string_with_overlap`: Calculates the length of two strings, then copies the second string into the first, handling potential memory overlap by copying backwards.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `set_bit_and_branch`: Sets bit 7 of d0 and then branches to another routine.
- `reverse_buffer_and_return_length`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `set_register_and_branch`: Sets bit 7 of d0 and branches to another instruction.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `parse_and_branch_on_string`: Sets bit 5 of d0, loads a value from a table, and calls a routine to parse a string, then branches or clears d0 based on the result.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_integer_string`: Checks a string for a sign, parses a sequence of digits into a signed integer, and writes the result to a destination address.
- `parse_signed_integer_string`: Parses a string of digits, calculates a value, and writes a 16-bit signed integer to a destination address while returning the length of the processed string.
- `fill_memory_with_value`: Stores a byte value derived from a parameter into a memory block pointed to by another parameter, repeating the operation for a count specified in the parameter.
- `allocate_heap_block`: It rounds the requested size up to a four-byte boundary and searches a linked list of free blocks for a suitable gap, splitting or merging blocks as needed.
- `call_and_return_value`: It forwards its arguments to another routine and returns whatever that routine produces.
- `reset_invoke_method`: Checks a global pointer and calls another routine if it is non-null.
- `execute_callback_chain`: Recursively calls itself if a global variable is non-zero, then clears the variable.
- `copy_memory_block`: The routine copies a block of bytes from one memory area to another using a count argument, handling overlapping source and destination by copying backwards when the source address is at or before the destination address, otherwise copying forwards.
- `free_linked_list_nodes`: It walks a linked list held in a global pointer, invoking a destructor stored at a fixed offset within each node.
- `clear_global_and_update_pointer`: Checks a global variable; if non-null, it calls another routine with that variable and then nullifies it.
- `handle_signal_offset_logic`: Checks for specific signal bits and a non-null global pointer, then nullifies the global variable and performs arithmetic on a parameter base.
- `allocate_linked_list_node`: It obtains a memory block and initializes its fields with a provided value and a global pointer.
- `allocate_free_memory_block`: Traverses a linked list of free memory blocks to find a segment large enough for the requested size, splitting the block if necessary, and attempts a memory expansion if no suitable block exists.
- `reduce_and_return_value`: Calls itself with a reduced value derived from the input parameter and then returns that value.
- `allocate_and_copy_block`: Allocates a memory block from a global heap, copies eight words from the allocated block to a destination pointer, and frees the block if a condition is met, returning a status code.
- `perform_library_operations`: Executes three specific Amiga OS library calls, handling string copying and potential memory overlap or string length checks.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to -(a7).
- `clear_global_update_pointer`: Checks a global variable; if non-null, it calls another routine with that variable and then nullifies it, while also updating a global pointer with a decremented value.
- `iterate_linked_list_entries`: The routine traverses a linked list of structures starting from a global variable.
- `clear_global_and_dispatch`: Checks if a global pointer is non-null and calls another routine if so.
- `handle_signal_with_offset_logic`: Checks for specific signal bits and a non-null global pointer, then nullifies the global variable and performs arithmetic on a parameter based on another parameter's value.
- `reset_and_invoke_method`: Checks a global pointer and calls another routine if it is non-null.
- `reset_and_invoke_global`: Checks if a global variable is non-null and calls another routine if so.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `reset_context_fields`: Sets certain fields within a structure to a reset value and calls a destructor function.
- `handle_global_callback_chain`: Checks a global variable and recursively calls itself if non-zero, then invokes a series of functions pointed to by offsets within a structure.
- `execute_chained_callbacks`: Checks a global variable and recursively calls another routine if non-zero, then executes a sequence of function pointers located at certain offsets from the global variable.
- `invoke_method_if_present`: It checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `normalize_and_divide_signed`: Checks the sign of d0 and d1, negates registers as needed, and calls another routine to perform a normalized fixed-point division.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `copy_and_execute_library_call_2`: Loads a byte from a pointer offset 54 in a structure, compares it to a constant, and otherwise sets d7 and d0 to a constant and branches or loads a pointer into a1.
- `copy_and_execute_library_call`: Copies bytes from one pointer to another using a loop, clears a byte in a structure, calls a library function, and then checks a pointer or a value in a structure to conditionally call another library function and update a byte in a structure.
- `execute_indirect_library_call`: Loads a pointer, pushes arguments, calls a routine, and returns a value or a constant based on the result.
- `execute_indirect_with_arguments`: Loads a pointer into a global variable, pushes several arguments onto the stack, calls another routine, and returns zero or negative one based on the result.
- `return_zero_constant`: The routine returns a constant zero value without performing any other operations or memory accesses.
- `create_message_node`: Obtains a global variable and calls another routine through a table.
- `set_message_type_and_target`: If a non-zero parameter is provided, it calls another routine via a global table and populates the returned structure with a message type, a zero byte, the original parameter, and a 16-bit value.
- `validate_and_clear_entry`: Checks if a pointer returned by another routine is null or has a specific flag set, then clears the pointer and returns a status code based on a global variable's state.
- `validate_table_entry_index`: It checks if an index is within bounds and if the corresponding entry in a global table is non-zero.
- `check_signal_and_clear_global`: Checks if a signal bit is set and a global pointer is non-null, then calls another routine and clears the global pointer if certain conditions are met.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `handle_library_call_results`: Evaluates results from library calls and global state, setting return codes and invoking cleanup routines based on success or failure conditions.

### MiG-29 - Fulcrum (1991)(Domark).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_to_global`: Stores a byte from register d0 into a global variable.
