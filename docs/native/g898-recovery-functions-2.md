# Game 898 (mig-29 - fulcrum (1991)(domark)[cr skr]): code recovery summary

## Functions (part 2 of 2)

### MiG-29 - Fulcrum (1991)(Domark)[cr SKR].adf:mig
(continued)

- `check_signal_and_clear_global_2`: Checks if a certain bitmask is present in a value returned from a global pointer, checks if a global pointer is non-null, and if a routine called via that pointer returns a non-zero value, nullifies the global pointer and calls another routine.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `handle_library_call_results`: Evaluates results from library calls and global state, setting return codes and invoking cleanup routines based on success or failure conditions.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `setup_stack_and_registers`: Saves registers d2-d7 and a2-a6 to the stack, increments the stack pointer, loads a constant into d0, clears d1 and d3, loads the address of a constant into a7, and loads a value from a pointer into a7 before an illegal instruction.

### Stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0`: Copies the contents of register d1 to register d0 and then jumps to another routine.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value stored in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to a relative address.

### Click

- `wait_for_button_release`: Checks bit 6 of the CIAA_PRA register in a loop and returns when that bit is cleared.

### Disk-Validator

- `format_string_and_initialize_buffer`: Converts a numeric value from a global variable into a string of ASCII digits and stores it in a buffer, then initializes a bitmask-based lookup table or buffer by clearing or setting bits or words in a loop.

### SkidRow

- `parse_amigados_hunk_relocations`: Parses a stream of AmigaDOS hunk-type codes to perform memory relocation and instruction patching, specifically handling code or data relocation types and updating pointers or instruction offsets.
- `decrement_and_loop_2`: Decrements a 16-bit value in d1 until it reaches zero and then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_loop`: Checks a bit in a value, then enters a loop that copies bytes from a source pointer to a destination pointer while decrementing a counter, or performs a block copy or a block move if certain conditions or bit flags are met.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Iterates through a loop, performing bitwise shifts and masking on registers and memory, and calls a routine that subtracts a value from d0.
- `move_and_trap_data`: Moves a word from an offset of 3277 in a2 to d4, calls a trap, and ors d2 into d0.

### MiG-29 - Fulcrum (1991)(Domark)[cr SKR].adf:intro

- `parse_amigadot_hunk_relocation`: Parses a list of executable hunk headers, handling relocation or executable loading types, and updates or copies memory blocks or relocation offsets based on the hunk type or size.
- `decrement_register_to_zero`: Decrements a 16-bit register value and loops until the register reaches zero, otherwise it returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `uncompress_bitstream_data`: Reads a length or flag from a memory location, then enters a loop that checks a bitmask and conditionally copies or shifts bytes from a source pointer to a destination pointer, or copies a block of bytes using a calculated offset, or otherwise returns.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_register_value`: Subtracts the value in data register d0 from the value in data register d4.
- `process_bit_stream_2`: Iterates through a loop that performs bitwise shifts and masking on registers and memory, and calls a routine that subtracts a value from d0.
- `load_and_execute_trap`: Loads a 16-bit word from a global variable into a register, executes a system trap, and then performs a bitwise OR of a register with another register.

### MiG-29 - Fulcrum (1991)(Domark)[cr SKR].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_script_string`: Parses a string to extract tokens or quoted substrings, performing escape character substitution for certain control characters.
- `jump_to_address_4`: Jumps to a fixed address stored in a global variable.
- `parse_string_to_table`: Parses a string or a quoted string, replacing certain escape characters with newlines or carriage returns, and stores the resulting string pointers or processed strings into a table.
- `manage_dma_and_callback_chain`: Disables interrupts to perform a sequence of indirect function calls and data manipulations, including DMA controller configuration and string processing.
- `jump_to_handler`: Performs an indirect jump to a longword address.
- `process_recursive_vector_list_2`: Recursively processes a list of data structures by performing fixed-point arithmetic and vector transformations based on a count value.
- `recursively_calculate_bounds`: Performs recursive calls with modified integer values and clamps the results against a global variable.
- `execute_callback_sequence_2`: Executes a series of function calls using a value from a global variable and several constant addresses.
- `process_conditional_loop`: Performs a series of loops that update a local array and a global variable based on the results of various subroutine calls.
- `execute_sequence_of_commands`: Executes a series of specific function calls with various immediate values and register contents to perform a sequence of operations.
- `execute_callback_sequence_3`: Executes a series of function calls including a conditional loop, state resets, and memory storage operations.
- `expand_input_bitmask`: Iterates through a 12-byte bitset to populate a linear flag buffer, then calculates a composite input code by checking specific bits in a global variable and adding offsets.
- `reset_and_update_globals`: Decrements a global variable and, if it reaches zero, sets it to one, calls a routine, and performs bitwise OR operations on a series of memory locations.
- `open_keyboard_device_2`: Opens the keyboard device using library calls, initializes a message structure, and sets a field in a global structure to point back to a global variable.
- `update_object_state`: Updates several fields within a structure, including a function pointer and numeric values, based on a global pointer.
- `cleanup_structure_data`: If a global variable is non-zero, it calls a routine pointer at an offset from the global variable, calls another routine pointer at a different offset, writes a constant to a structure, calls two more routine pointers, and recursively calls itself with the global variable as an argument.
- `cleanup_display_context_5`: Calls a routine pointer at an offset from a global variable, then calls another routine twice with that global variable as an argument.
- `store_mouse_coordinates_2`: Stores the lower or upper 16-bit word of three separate parameters into a sequence of global variables, where each parameter provides a 32-bit value split into two 16-bit halves.
- `check_input_state_flag_3`: Checks if a global variable is zero, calls two subroutines if it is, and then returns the value of that global variable while clearing it if it was non-zero.
- `find_valid_key_index`: Loops calling reset_and_update_globals if a global variable is zero, then continues looping while a global variable is outside the range of one to seventy-one or if the null-terminated character at an offset from that global variable is a null terminator; finally, it sets the global variable to zero and returns the character located at the offset from the last checked index.
- `process_callback_chain_loop`: Retrieves a value from a global variable and executes a routine via a pointer offset.
- `call_free_mem_pointer`: Calls a function via a pointer located at an offset from a global variable if that global variable is non-zero.
- `set_global_flag`: Sets a global variable to one if it is currently non-zero.
- `reset_globals_to_constant_2`: Clears a 16-bit global variable and calls another routine that writes zero to several global variables and a constant value to a fifth global variable.
- `jump_to_initialize_system_resources`: Performs a long jump to the routine that initializes system resources.
- `jump_to_sub_c21cf0`: Performs a long jump to an unlisted routine at a fixed address.
- `validate_signal_and_recurse_2`: Calls another routine and, if the return value is non-zero, calls that same routine again with a pointer offset by two bytes, provided a certain condition is met.
- `execute_callback_and_check_result_2`: Jumps to a routine pointer located at an offset from a global variable.
- `jump_to_target_routine_5`: Performs an indirect jump to a target address stored in a longword at a specific memory location.
- `execute_callback_and_check_result_3`: Writes certain constant values to a structure at an offset, then jumps to a routine pointer located at an offset from a global variable.
- `process_audio_parameters`: Performs complex calculations on data within a structure to update various memory locations and hardware audio volume registers.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `update_audio_register_and_counters`: Updates a hardware audio register or a global variable using a value or a masked/shifted version of a byte from a global variable, while also managing a counter that resets or increments a table index or a global variable if a certain value is reached.
- `update_object_kinematics`: Updates an object's position or coordinate values based on a mode flag and velocity data.
- `update_sprite_attributes`: Reads a value from a memory location to determine an index, then fetches sprite coordinate and frame data from a table to update a sprite structure.
- `copy_data_block`: Copies a block of 30 bytes from one memory location to another and increments a global index.
- `decrement_global_index`: Decrements a global variable and adjusts it by a fixed offset if it is not zero.
- `sum_triplets_with_scaling`: Calculates the sum of three sets of three values and stores the results in a table, applying a scaling factor if certain conditions are met.
- `accumulate_fixed_point_products`: Performs multiple fixed-point multiplications and sums the results into three consecutive integer slots in a destination table.
- `process_fixed_point_division_remainder`: Calls a routine with two arguments and, if a certain value in a local variable is positive, calls two more routines and then calls a routine via a pointer in a global variable.
- `average_and_store_pairs`: Calculates the sum of pairs of 16-bit values from a table, shifts the result right by nine bits, and stores the values back into the table.
- `update_table_elements`: Sums specific pairs of 16-bit values from a table, shifts the results right by nine bits, and updates the table at specific offsets.
- `process_table_averget_pairs`: Calculates the sum of pairs of 16-bit values from a table, shifts the results right by nine bits, and writes them back to the first six elements of the table.
- `calculate_complex_sum`: Performs several routine calls, calculates a sum of a value from a pointer and other routine results, and then writes a value to a destination pointer or writes a different value if a certain threshold is met.
- `compare_coordinate_differences`: Compares the difference between two pairs of 16-bit values and returns a boolean result.
- `compare_fixed_point_values`: Compares two values from a pointer, and if certain conditions or thresholds are met, copies values, or calls routines to add or subtract a fixed amount from a pointer's contents.
- `adjust_fixed_point_range`: Compares two 32-bit values and, if the second value is negative, updates a destination structure's low and high words using fixed-point multiplication and subtraction or direct assignment, otherwise it returns a status code or calls itself recursively.
- `modify_fixed_point_elements`: Compares two values from a pointer, and if certain conditions or thresholds are met, copies or modifies certain elements in a pointer's target using routine calls and fixed-point subtraction.
- `interpolate_vectors`: Calculates a midpoint between two sets of three-dimensional coordinates through an iterative loop until a threshold or certain condition is met, updating the destination pointer with the resulting coordinates.
- `interpolate_vectors_alt`: Performs an iterative midpoint calculation between two vectors, using a different initial summation for the first component.
- `compare_coordinate_vectors`: Compares the absolute values of elements within two arrays and performs recursive calls to sort or process them based on their magnitude.
- `compare_and_sort_vectors`: Compares the absolute values of elements within two vectors and recursively calls itself to sort or reorder them based on their magnitude and sign.
- `process_vector_intervals`: Performs recursive comparisons and processing on multiple parameters, including checks for sign and magnitude of values.
- `process_vector_parameters`: This routine invokes a function to perform recursive comparisons and processing on multiple parameters, including checks for sign and magnitude of values.
- `split_numerical_interval`: Calls a routine that recursively divides a numerical interval based on a global variable.
- `execute_recursive_logic_sequence`: Invokes a series of subroutines to divide a numerical interval, perform complex logic operations, compare and adjust 32-bit signed values, and multiply two 16-bit values.
- `execute_complex_vector_logic`: Performs a wide variety of mathematical and logical operations including coordinate comparisons, fixed-point arithmetic, vector sorting, and remainder calculations.
- `process_vector_list`: Iterates through a list of elements, calling a routine for each, and writes or calculates new values into a stack-based buffer based on the results and a global variable, then calls further routines if certain pointer offsets are non-zero.
- `process_recursive_vector_list`: Checks a value from a pointer, and if it exceeds a threshold, calls another routine.
- `copy_block_and_set_ptr_2`: Copies a block of data from a source pointer to a fixed destination address and stores that destination address into a global variable.
- `store_params_and_recurse_2`: Stores two 32-bit parameters into four consecutive 16-bit global memory locations, then calls another routine passing the upper 16-bit halves of the original parameters.
- `compare_signed_words`: Compares two 32-bit signed values represented as pairs of 16-bit words against a global value and returns a status code.
- `adjust_32bit_values_by_global`: Compares the high words of two 32-bit values against a global variable and performs arithmetic on the low words and high words of those values based on the comparison results.
- `split_interval_by_value`: Recursively divides a numerical interval into smaller sub-intervals based on whether a global variable falls within the current range, adjusting the boundaries of the interval segments to ensure the global value is isolated or correctly partitioned.
- `execute_function_via_pointer`: Retrieves a function pointer from a table and executes it with a value derived from another table entry.
- `toggle_dma_enable`: Updates a global variable based on its current value and executes a function pointer retrieved from a table using a value from another table, while setting bit 15 of the DMA controller register.
- `iterate_and_call_functions`: Loops through a table to execute function pointers and updates the table entries based on results.
- `execute_conditional_callbacks`: Executes several function pointers at various offsets from a global variable, including a loop that checks a table for non-zero values before calling a function.
- `initialize_library_pointers`: Attempts to open the graphics and intuition libraries, checking if the returned pointers are null and calling error-handling routines if a library fails to load.
- `initialize_registers`: Calls a function via a pointer and then sets several consecutive global values to specific constants.
- `populate_table_data`: Iterates through two nested loops to store a global variable and derived values into a table, while also updating specific byte and word entries within that table.
- `process_callback_table`: Manages memory and system resources before iterating through tables to execute function pointers and store derived values into a global table.
- `shutdown_graphics_system_2`: Executes cleanup functions for graphics resources, sets a global variable to a specific value, and enables DMA channels by writing to the DMacon register, then closes device and library handles via Amiga OS calls.
- `call_functions_on_condition`: Executes two functions via pointers if the first parameter is less than the second parameter.
- `clamp_and_call_indirect`: Adjusts two parameters to ensure they fall within a range defined by a global variable and then calls a function via an indirect pointer from a table.
- `clip_range_and_recurse`: Calculates new bounds for two parameters based on a global variable and a range, then calls itself recursively with the updated values.
- `call_clamped_function`: Calls another routine using a parameter adjusted by a value from a global variable.
- `expand_bit_packed_data`: Iterates through a collection of bit-packed source words, using bits from those words to index into a lookup table to extract and reassemble new bit patterns, which are then written back to a destination pointer array.
- `copy_and_iterate_table`: Copies 32 bytes from a source into a destination pointed to by a global variable and then executes two function pointers.
- `call_functions_on_range`: Checks if a global value falls within a specified range and executes two functions via pointers if the condition is met.
- `compare_and_call_function`: Compares two ranges and either calls a function directly or executes two specific functions via pointers if the ranges do not match.
- `execute_complex_logic`: The routine contains 111 instructions and calls several library functions and other routines.
- `execute_complex_logic_2`: Contains 212 instructions and calls several library functions and other routines.
- `process_complex_logic`: Compares two 32-bit signed values against a global value, adjusts them, and executes a complex logic routine.
- `parse_string_data`: If a routine returns zero, it compares a string against a constant, and if a mismatch is found, it searches for a delimiter in a stack buffer and calls a routine with a pointer to that buffer and a string, or calls a routine with a pointer to the buffer and the original string.
- `parse_bmp_load_view`: Calls a routine to parse BMP image data and another to copy buffers for loading a view, then conditionally calls another routine if a certain bit or value is non-zero.
- `load_pointer_and_call`: Loads a pointer from a global variable into a0 and calls a function via that pointer, then returns.
- `process_string_output`: Iterates through a null-terminated string, calling a routine for each character and incrementing a global variable, while resetting that variable upon encountering a newline character.
- `update_global_word`: Stores two consecutive bytes from a passed parameter into a global variable.
- `execute_callback_loop`: Sets a global variable to zero, calls a function via a pointer from a global variable, calls another function, and then loops through a table to execute several function pointers, updating a global variable and checking for zero values, while calling a function or calling itself recursively.
- `sum_function_results`: Executes a function via a pointer from a global variable and returns the sum of two results from other function calls.
- `execute_graphics_setup_string`: Invokes three function pointers from a global structure, iterates through a null-terminated string, and then invokes a final function pointer from the same structure.
- `free_list_elements`: Iterates through a list of pointers provided in a parameter and calls a function for each non-zero entry before calling another function.
- `check_block_allocation_marker`: Calls a routine to allocate a memory block and copy eight words to a stack buffer, returning one if the allocation succeeded and the first word of the buffer equals eighty-two, otherwise returns zero.
- `handle_callback_chain`: Iterates through a table of structures to check a bit in a byte, and if a certain character is found in a string, it calls a routine and updates a flag in the table, or calls another routine and updates the table with a new pointer and cleared fields.
- `deallocate_entity_pointer`: Checks if a pointer is non-null and a global variable is positive, then if a bit is set in a byte at offset 13 and a short at offset 8 is non-zero, calls another routine, clears a 16-bit value at offset 12, calls another routine, and decrements the global variable.
- `increment_and_fetch_byte`: Checks if a value in a structure has reached a threshold, and if so, calls a routine to reset that value, otherwise it increments a counter in the structure and returns a byte from a table using an offset or a pointer.
- `copy_bytes_with_threshold`: Copies bytes from a table to a destination buffer using an incrementing index, while checking a counter against a threshold and handling structure resets if the counter reaches a certain value.
- `jump_to_target_2`: Performs a long jump to a target address.
- `jump_to_address_5`: Performs an indirect jump to an address stored in a long register.
- `jump_to_target_routine_4`: Performs an indirect jump to a routine located at the address stored in a long-word pointer.
- `process_callback_chain`: Clears a global variable and iterates through a table of 6-byte entries to find a non-zero or non-null entry, then recursively calls itself.
- `compute_fixed_point_product_2_2`: Performs a 32-bit fixed-point multiplication using the lower and upper halves of two registers and returns the result via a jump to a fixed address.
- `decode_compressed_data_2`: Decompresses data from a stream into a buffer using run-length encoding logic, where the number of bytes to write or the value to write is determined by reading from a source pointer.
- `parse_bmp_chunks`: Iterates through a BMP file stream to identify and process specific chunk types, such as checking for certain magic strings, and if a certain chunk type is found, it extracts or skips data or performs a 24-bit to 16-bit color conversion or certain padding or alignment or skips certain bytes.
- `check_and_return_register`: Tests a bit in a global variable and if not set, moves a value from a register to d0 before returning; otherwise it returns.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_table_loop_2`: Iterates through a range defined by two registers, calling functions pointed to by a table indexed by a base pointer and an offset.
- `set_mask_and_branch_2`: Calculates a count and a pointer, then loops through a certain number of iterations, or-writing or and-writing a masked value to a memory location based on a bit test.
- `fetch_table_value_2`: If a 16-bit value in d6 is less than 16, it shifts d6 left by 1, uses the result as an index into a table at a2 plus offset 34, and loads that word into d0, otherwise it returns.
- `set_mask_and_branch`: Loads a word of all ones into d0 and d4, inverts d4, tests bit 0 of d3, and if clear, or-writes d0 into a memory location at a pointer offset.
- `apply_mask_and_branch_2_2`: Performs a bitwise AND between a word in d4 and a memory location, tests bit 1 of d3, and if clear, or-writes a word from d0 into a memory location via a pointer.
- `apply_mask_and_branch_2`: Performs a bitwise AND between a word in d4 and a memory location, tests bit 2 of d3, and if clear, or-writes a word from d0 into a memory location.
- `apply_mask_and_branch`: Performs a bitwise AND between a word in d4 and a memory location using a pointer in a1 and an offset in a5, then tests bit 3 of d3 and branches or ors a word from d0 into a memory location using a1 and a6.
- `update_pointer_and_branch`: Performs a bitwise AND between a word in d4 and a memory location, increments a pointer in a1, and branches back to an earlier instruction if d6 is less than ten, otherwise it modifies d6 and branches to another instruction.
- `restore_registers_and_return`: Restores registers from the stack and returns immediately.
- `increment_pointer_offset_2`: Increments a 32-bit value located at an offset of 8 bytes from a base pointer by 4.
- `increment_pointer_offset_c_2`: Increments a 32-bit value located at an offset of 12 bytes from a base pointer by 4.
- `execute_dispatch_loop`: Calls a routine to check a range or pointer-set in d5 and d6, calls a routine to increment a 32-bit value at an offset of 8 bytes from a base pointer by 4, calls a routine to increment a 32-bit value at an offset of 12 bytes from a base pointer by 4, and calls a routine that checks if d3 is greater than zero and otherwise decrements a value at an offset of 4 from a0.
- `loop_on_condition`: Checks if d3 is greater than zero, and if not, decrements a value at an offset of 4 from a0, and if that value is not zero, calls a routine and loops back to the start of this routine.
- `iterate_range_and_call`: If d5 is less than d6, it sets d6 to d5, calls a routine, increments a pointer in a1 by 28, decrements d3 and d1, and adds d2 to d5 and d4 to d6 before looping back to a previous instruction.
- `invoke_dispatch_table_loop`: Checks if a range or pointer-set in d5 and d6 is valid, synchronizes d6 to d5 if d5 is greater, and calls the dispatch table loop routine.
- `update_pointer_offsets_2`: Increments a 32-bit value at an offset of 8 bytes from a base pointer by 4, increments a 32-bit value at an offset of 12 bytes from a base pointer by 4, and calls a routine that compares two longwords and potentially updates the second with the first.
- `decrement_and_branch_pointer`: Tests d3, and if not greater than zero, decrements a 16-bit value at a 4-byte offset from a pointer in a0, then loops back to the start of the routine or calls another routine.
- `set_mask_and_branch_2_2`: Compares two longwords and, if the first is not greater than or equal to the second, updates the second with the first, calls another routine, and modifies several registers before branching back to an earlier instruction in the routine.
- `apply_bitmask_to_targets_2`: Compares two longwords and, if the first is not greater than or equal to the second, updates the second with the first, calls another routine, and restores several registers before returning.
- `check_word_and_branch`: Tests if a word in a global variable is zero and, if not, branches to another routine; otherwise it returns.
- `apply_bitmask_to_buffer`: Calculates a range or a mask-based bitwise operation (AND or OR) and applies it to a sequence of words in a memory buffer.
- `fetch_table_value_alt`: If a 16-bit value in d2 is less than 16, it shifts d2 left by 1, uses the result as an index into a table at a2 plus offset 34, and loads that word into d0, otherwise it returns.
- `invert_and_conditionally_or_word`: Loads a word containing all ones into a register, copies it to another register, inverts that register, and if bit zero of a register is clear, or-writes the word containing all ones into a memory location at a pointer plus an offset; otherwise it returns.
- `apply_mask_and_branch_3`: Performs a bitwise AND on a memory location using d4, tests bit 1 of d3, and if clear, or-writes dif d0 into a memory location at a pointer offset.
- `apply_mask_and_branch_alt_2_2`: Performs a bitwise AND between a word in d4 and a memory location, tests bit 2 of d3, and if clear, or-writes a word from d0 into a memory location via a pointer.
- `apply_mask_and_branch_alt_2`: Performs a bitwise AND between d4 and a memory location, tests bit 3 of d3, and if clear, or-writes a word from d0 into a memory location.
- `apply_mask_and_branch_alt`: Performs a bitwise AND between d4 and a memory location using a1 and a6, then tests bit 4 of d3 and branches or ors a word from d0 into a memory location using a certain offset.
- `process_table_index`: Performs a bitwise AND between d4 and a memory location, increments a pointer in a1, and branches back to an earlier instruction if d2 is not less than ten, otherwise it modifies d2 and branches to another instruction.
- `restore_registers_and_return_2`: Restores d3, d4, d5, a1, a2, a3, a4, a5, and a6 from the stack and returns immediately.
- `test_global_word_and_branch`: Tests if a word in a global variable is zero and, if so, branches to another routine; otherwise, it returns.
- `apply_bitmask_to_range`: Calculates a range of memory addresses and, for each address, updates a word in a global array by applying a bitmask (using bitwise AND or OR depending on a bit in a register) or by using a value from a lookup table.
- `fetch_scaled_table_entry_2`: Checks if a value in a data register is greater than or equal to sixteen; otherwise, it calculates an even-byte offset by doubling that value, loads a word from a table located at an offset of thirty-four bytes from a pointer in a2, and jumps to another routine.
- `set_table_entry_if_bit_clear`: Sets all bits in a word at a pointer-offset in a6 to one if bit zero of a register is clear, otherwise it returns or jumps to another routine.
- `update_table_entry_conditionally_2_2`: Performs a bitwise AND between a word in d4 and a memory location at a pointer-offset, then checks bit 1 of d3 and, if clear, or-writes a word from d0 into a memory location at a pointer-offset; otherwise it returns.
- `update_table_entry_conditionally_2`: Performs a bitwise AND between d4 and a memory location, tests bit 2 of d3, and if clear, or-writes a word from d0 into a memory location.
- `update_table_entry_conditionally`: Masks a word from a table using a pointer and an offset, checks bit 3 of a register, and if clear, or-writes a word from a register into a different table location using a pointer and an offset before branching to another routine.
- `update_table_pointer_and_decrement_index`: Reads a word from a table using a pointer and an offset, increments that pointer by two, and then either decrements a counter or jumps to another routine if the counter is or exceeds ten.
- `restore_registers_and_return_3`: Restores d3, d4, a1, a2, a3, a4, a5, and a6 from the stack and returns immediately.
- `mask_and_conditional_set_v2_2_2`: Saves d3 and d4 to the stack, loads a value into d4 and inverts it, loads a long from a pointer in a0 plus an offset into d3, and if bit 0 of d0 is clear, or-writes d5 into a memory location via a pointer.
- `mask_and_conditional_set_v2_2`: Performs a bitwise AND between d4 and a memory location, loads a long from a pointer in a0 plus an offset into d3, tests bit 1 of d0, and if clear, or-writes a word from d5 into a memory location.
- `mask_and_conditional_set_v2`: Performs a bitwise AND between d4 and a memory location using a1 and d3, loads a long value from a pointer in a0 plus an offset into d3, then tests bit 2 of d0 and branches or ors a word from d5 into a memory location using a1 and d3.
- `mask_and_conditional_set`: Performs a bitwise AND between d4 and a memory location, loads a long value from a pointer in a0 plus an offset, and if bit 3 of d0 is cleared, or even if it is set, performs a bitwise OR or branches to another instruction.
- `mask_and_set_bits`: Performs a bitwise AND between d4 and a memory location at a1 plus d3, then loads a value from a0 plus offset 68 into d3, and if bit 4 of d0 is not set, performs a bitwise OR of d5 into that same memory location before branching.
- `mask_memory_word_2`: Performs a bitwise AND between a 16-bit value in a register and a word at a memory location pointed to by a register, then restores registers and returns.
- `invert_and_set_bit`: Inverts a word in d4, loads a long from a memory location at an offset of 52 from a pointer in a0 into d3, and if bit 0 of d0 is clear, or-writes a word from d5 into a memory location at a1 plus d3, otherwise it jumps to another instruction.
- `mask_and_conditional_set_v3`: Performs a bitwise AND between d4 and a memory location using a1 and d3, loads a long value from a pointer in a0 plus an offset into d3, then tests bit 1 of d0 and branches or ors a word from d5 into a memory location using a1 and d3.
- `mask_and_conditional_set_alt`: Performs a bitwise AND between d4 and a memory location, loads a long value from a pointer in a0 plus an offset, and if bit 2 of d0 is cleared, or even if it is set, performs a bitwise OR or branches to another instruction.
- `apply_bitwise_and_to_memory_2`: Performs a bitwise AND between d4 and a memory location at a1 plus d3, then loads a value from a0 plus offset 64 into d3, and if bit 3 of d0 is not set, performs a bitwise OR of d5 into that same memory location before branching.
- `apply_bitwise_and_to_memory`: Performs a bitwise AND between a 16-bit value in d4 and a 16-bit value at a memory location pointed to by a1 and d3, then restores d3 and d4 from the stack and returns.
- `check_global_word`: Tests if a word in a global variable is zero and otherwise returns immediately.
- `validate_coordinate_range_2`: Stores two 16-bit values into global memory and checks if they fall within a bounding box defined by two other global variables; if the values are within the range, it calls another routine.
- `test_and_return`: Tests a long value at a memory location and otherwise returns immediately.
- `validate_coordinate_range`: Stores two 16-bit values into global memory and checks if they fall within a specific bounding box defined by two other global variables.
- `store_two_word_params_2`: Splits two 32-bit parameters passed on the stack into their constituent 16-bit halves and stores them into four consecutive 16-bit locations in a global RAM area.
- `copy_five_word_block_2`: Copies five consecutive 32-bit words from a source pointer passed via the stack into a contiguous block of global RAM memory starting at a fixed address.
- `copy_four_word_block_2`: Copies four consecutive 32-bit words from a source pointer into a contiguous block of global RAM memory.
- `multiply_stack_values_2`: Multiplies a 16-bit value from the stack by another 16-bit value from the stack and returns.
- `multiply_and_divide_registers`: Multiplies d0 by d1 and then divides the result by d2, then returns.
- `transform_vector_by_matrix_2`: Multiplies a 3-element vector by the rows of a 3x3 matrix stored in a table and adds a translation offset or a value from a separate memory location to each resulting component, storing the results in a destination array.
- `parse_complex_ascii_numeric_string`: Parses a string containing numeric values, handling integer components, decimal points, and scientific notation suffixes.
- `parse_formatted_expression_string`: Parses a string containing numeric values, signs, and format specifiers to evaluate or convert data.
- `parse_format_string`: Walks a string character by character and if a percent sign is found, calls another routine with a pointer to the next character and a stack slot, then if that call returns null, calls a function pointer from a table and skips ahead, otherwise it continues until a null terminator is encountered.
- `test_register_bits`: Sets certain bits in d0, loads two longwords from a structure offset 8 from a pointer in a5 into d0 and d1, and calls a routine that tests if certain bits are set, otherwise it loads two longwords from a structure offset 8 and offset 12 from a5 into d0 and d1 and jumps to another instruction.
- `load_and_test_register_bits`: Loads two 32-bit values from a structure offset into d0 and d1, calls a routine that tests if d0 or d1 or their or-ed result is zero and clears the lower five bits of d0, then restores d2 and d3 before returning.
- `parse_complex_string`: Parses a string by checking for specific characters and calling another routine to process segments, returning a count or a certain error value.
- `process_byte_from_memory`: Pushes d7 to the stack, clears d7, loads a pointer from a global address, loads a byte from that pointer into d7, restores the pointer, and if the byte was not zero, loads d7 into d0 and branches to another instruction.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `update_memory_pointer`: Pushes a value from d7 to the stack, loads a word from a pointer relative to a4, increments or decrements a value at a global address, and if a certain condition is met, copies d7 to d0 and writes a byte to a pointer in a0 before branching to another routine.
- `invoke_memory_callback_chain`: Moves a value from d7 to d0, pushes a pointer relative to a4 onto the stack, calls a routine that manages memory allocation and executes a sequence of chained callbacks, cleans up the stack, and returns.
- `execute_callback_sequence`: Clears a value at a memory offset, calls a routine with a pointer and stack reference, calls another routine, and returns the cleared value.
- `decrement_and_branch_on_value`: Pushes a3 to the stack, loads a pointer from a3 plus 8 into a3, and checks if a word loaded from that pointer is equal to one or two; otherwise it writes the value 33 to a global variable and jumps to another routine.
- `write_byte_to_offset`: Stores a single byte into a specific offset relative to a register value.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_branch_logic`: Pushes a3 to the stack, loads a pointer from a3 plus 8 into a3, and checks if a word loaded from that pointer is equal to one or two, otherwise it calls a routine and loads a 64-bit value from a memory location calculated by adding an offset to a register.
- `load_from_offset`: Loads a 64-bit value from a memory location calculated by adding an offset to a value held in a register.
- `format_numeric_string`: Converts a 64-bit integer value into a decimal-encoded string of characters.
- `decrement_and_loop`: Decrements a value in a register until it reaches zero, while rotating another register.
- `multiply_64bit_by_ten`: Multiplies a 64-bit value held in two 32-bit registers by ten, returning the high word from the upper input and carry bits and the low word as the lower input times ten.
- `shift_bits_into_decimal_digits`: Shifts a 64-bit value left one bit at a time for a fixed count of 64 steps, carrying each bit into a decimal digit accumulator and subtracting ten with a carry into the high word whenever the digit exceeds nine.
- `parse_ascii_floating_point`: Converts an ASCII string representing a floating-point number into a numeric value.
- `update_callback_and_counter`: Checks a value in a structure, executes a function pointer from that structure, and decrements a counter, potentially setting a bit in another structure when the counter reaches zero.
- `transform_integer_bit_pattern`: Performs complex bitwise shifts, rotations, and masking on a 32-bit value based on a modified offset or temporary or calculated value, using conditional branching to apply different bit-manipulation logic or recursively calling another routine with a modified parameter or a new pointer-based argument.
- `shift_left_register_high`: Shifts a 16-bit value from a register left by thirty-two bits and returns the result.
- `return_shifted_register_value`: Calls another routine and returns a 64-bit value where the contents of a register or a cleared register value is shifted into the upper bits.
- `call_with_parameter`: Passes a constant value to a function pointer and returns a bitwise modified value.
- `set_constant_return`: Calls another routine and returns a fixed 64-bit constant.
- `modify_and_swap_bits_2`: Checks a bit in a register, conditionally calls another routine, and returns a bit-swapped version of the input register.
- `apply_table_rotation`: Uses a table and complex bitwise rotations or shifts to calculate and return a new value.
- `swaps_and_tests_register_bits`: Pushes a 32-bit value from d0 onto the stack, or-s it with d1, and if the result is zero, clears the lower five bits of d0 before returning; otherwise, it returns.
- `rotate_bits_left`: Performs a bitwise left rotation on a 32-bit value for ten iterations.
- `parse_ascii_floating_point_2`: Performs bit-shifting and bit-manipulation on registers and conditionally calls another routine based on certain value checks.
- `update_and_call_function`: Stores a 16-bit value into a table at an offset from a pointer and executes a function pointer retrieved from that same table if it is not zero or one.
- `execute_unreachable_code`: Performs bitwise or operations on d0, saves registers d2 through d7 and a1 to the stack, calls another routine, restores the registers from the stack, and returns.
- `transform_bit_swapped_registers`: Performs bit-swapping and bit-masking on registers d0 and d2, checks for specific bit patterns or zero-values, and conditionally calls other routines or jumps to a fixed destination with further arithmetic or bit manipulation applied to the registers or a global variable.
- `compare_and_branch_registers`: Compares d7 and d2, and if d2 equals a certain constant, checks if d3 is zero before moving d2 and d3 into d0 and d1 and jumping to a fixed address.
- `return_fixed_constant`: Performs an unconditional jump to a fixed address.
- `return_xor_shifted_register`: If certain registers are zero, calls another routine, then returns a 64-bit value where a register XORed with a constant is shifted into the upper bits; otherwise, it returns the same XORed value shifted.
- `jump_to_address_2`: Performs a jump to an address calculated from the program counter and a constant.
- `test_register_bits_2`: Compares d2 with d7, checks if d2 equals a certain constant, tests if d3 is zero, and if not, copies d2 and d3 to d0 and d1 before jumping to another routine.
- `shift_left_register_high_2`: Jumps to another routine that shifts a 16-bit register left by thirty-two bits.
- `test_register_bits_and_branch`: Checks if any bits in a 16-bit value in d0 are set after masking with d7, and if any of the registers d0, d1, d2, or d3 contain non-zero values, otherwise it jumps to another routine.
- `shift_left_register_high_3`: Shifts a 16-bit value from a register left by forty-eight bits and returns the result.
- `call_rotation_and_skip`: Calls a routine that performs a table-based rotation and then branches to a different instruction or routine.
- `modify_d0_and_branch`: Performs an XOR and an eori on d0, and if d2 or d3 are non-zero, jumps to another instruction or jumps to another routine.
- `swap_registers_and_call`: Swaps several long-word registers, calls another routine, swaps the registers back, and otherwise jumps to another routine.
- `modify_registers_and_branch`: Performs an XOR and an eori on d2, subtracts a constant from d7, subtracts d7 from d5, and otherwise jumps to another routine.
- `transform_bit_pattern_and_branch_2_2`: Performs a complex sequence of bitwise rotations, shifts, and arithmetic operations on register contents, including conditional checks against a constant, and then branches based on the result.
- `transform_bit_pattern_and_branch_2`: Performs a series of bitwise shifts, rotations, and arithmetic operations on register contents, including a conditional check against a constant, and calls another routine with a calculated value and a bit-manipulated register or otherwise returns.
- `transform_bit_pattern_and_branch`: Performs complex bitwise manipulations, shifts, and rotations on register contents, including conditional checks against constants, and calls another routine or returns a calculated 64-bit value or 32-bit value depending on the state of a register.
- `evaluate_register_conditions`: Performs a series of comparisons on d0, d1, d2, and d3, returning or branching based on whether certain values match or certain bits are set.
- `compare_registers_and_branch`: Compares two registers and branches if not equal, or compares two other registers and branches if certain carry conditions or inequality exist, otherwise it returns immediately.
- `check_register_values`: Tests if d2 is negative, then checks if d3, d1, or d0 contain a specific bit, otherwise it returns.
- `compare_long_values_and_branch`: Compares two 32-bit values in d0 and d2, then compares two 32-bit values in d3 and d1, branching to other routines or otherwise returning.
- `set_ccr_bit_three`: Sets bit three of the condition code register and returns immediately.
- `clear_status_register`: Clears certain bits of the status register and returns immediately.
- `shift_and_rotate_register`: Checks if d0 is greater than a certain constant, then shifts d0 right and rotates d1, and if d5 is certain value, branches to another routine.
- `modify_register_and_branch`: Subtracts a constant from d5, and if d5 is greater than or equal to a certain value, it branches or jumps to another routine.
- `manipulate_bit_stream`: Performs bitwise shifts and logical operations on register contents while decrementing a counter.
- `complex_register_manipulation`: Performs bitwise operations and conditional calls to another routine, then returns a value derived from a bitwise modified register.
- `tiny_eor_swap_rts`: Small routine of 3 instructions that exclusive-ors d4 into d0, then swaps the halves of d0, then returns.
- `execute_indirect_function_from_stack`: Loads a function pointer from an offset of 348 minus a constant (relative to a value on the stack) and calls it.
- `call_indirect_via_global_with_args`: Loads a function pointer from a global variable and calls it using a pointer to a structure or argument list located at an offset from the stack pointer, while preserving all registers.
- `execute_recursive_callback_chain`: Executes a recursive callback chain that checks return values, checks a bitmask in a returned structure, and may perform recursive calls or execute a secondary routine using a pointer retrieved from a structure, returning a failure constant if a certain global variable is non-zero.
- `copy_data_register_d0_to_d5`: Copies the contents of data register d0 to data register d5.
- `jump_to_callback`: Performs a jump to a fixed address.
- `allocate_memory_block_2`: Checks a block's size and flags, then either recursively allocates a new block or updates a pointer and clears certain memory offsets.
- `manage_memory_callbacks`: Performs memory allocation and executes a sequence of chained callbacks based on a global variable and retrieved pointers.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `jump_to_absolute_address`: Performs an unconditional jump to an absolute 32-bit address.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts uppercase ASCII characters to lowercase based on a flag in a global lookup table.
- `copy_string_with_limit_2`: Calculates the length of a null-terminated string from a source pointer, then copies a number of bytes from that source to a destination pointer up to a specified maximum length, ensuring the destination string is null-terminated.
- `copy_string_with_overlap_2`: Calculates the length of two strings, then copies the second string into the first, handling potential memory overlap by copying backwards.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied, then returns the original destination pointer.
- `set_register_and_branch`: Sets bit 7 of d0, moves a word from a location relative to a7 into d0, and then branches to another instruction.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `set_and_branch_d0`: Sets bit 7 of d0 and loads a new value into d0 from a stack offset, then branches to another routine.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `prepare_register_and_branch`: Sets bit 5 of d0, moves a word from a table into d1, moves a word from a table into d3, calls another routine, and then clears d0 and moves a word from a7 into d0 before branching.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer`: Checks for a leading plus or minus sign, then parses a sequence of ASCII digits into a signed short integer and writes it to a memory location, returning the number of characters processed.
- `parse_numeric_string`: Parses a string of digits into a signed short and returns the number of bytes processed.
- `fill_memory_with_value_2`: Stores a byte value derived from a parameter into a memory block pointed to by another parameter, repeating the operation for a count specified in the parameter.
- `allocate_heap_block_2`: It rounds the requested size up to a four-byte boundary and searches a linked list of free blocks for a suitable gap, splitting or merging blocks as needed, and updates a global variable representing free memory.
- `call_and_return_value_2`: It forwards its arguments to another routine and returns whatever that routine produces.
- `reset_and_invoke_method_2`: Checks a global pointer and calls another routine if it is non-null, then nullifies the global variable.
- `process_callback_chain_and_update_table`: Clears a global variable and iterates through a table of 6-byte entries to find a non-zero or non-null entry, then recursively calls itself while performing bitwise modifications on a parameter and updating the table with a new status and a return value from a recursive call.
- `move_memory_block`: Moves a block of memory from a source address to a destination address, copying from the end of the block if the destination is lower than the source to prevent overwriting, otherwise copying from the beginning.
- `process_global_and_allocate_pointer`: Checks a global variable; if non-null, it calls another routine with that variable and then nullifies it.
- `validate_signal_and_recurse`: Calls another routine and, if the return value is non-zero, calls that same routine again with a pointer offset by two bytes, provided a global variable is zero; otherwise, it returns a value of all ones or a value from the recursive call.
- `allocate_linked_list_node_2`: It obtains a memory block via a library call and initializes its fields with a provided value and a global pointer, then inserts the new node into a linked list.
- `allocate_heap_block_3`: Aligns a requested size to a four-byte boundary and traverses a linked list of free memory blocks to find a suitable gap, splitting a block if the remaining space is large enough or returning the block if it matches the size exactly.
- `recurse_with_truncated_parameter`: Loads a 16-bit value from a stack-relative offset, pushes its lower 16-bit value onto the stack, calls another routine, and returns.
- `allocate_and_copy_block_2`: Allocates a memory block using a function pointer from a global variable, copies eight words from the new block to a destination pointer, and frees the block using another function pointer if the heap or a secondary check fails, returning a status code.
- `perform_library_operations_3`: Executes three specific Amiga OS library calls using negative offsets and performs string copying or length-related string manipulations using helper routines, then pushes the contents of a register onto the stack before returning.
- `pop_d0_to_stack`: Pushes the contents of register d0 onto the stack.
- `clear_global_update_pointer`: Checks a global variable; if non-null, it calls another routine with that variable and then nullifies it.
- `iterate_linked_list_entries`: The routine traverses a linked list of structures starting from a global variable.
- `clear_global_and_dispatch`: Checks if a global pointer is non-null and calls another routine if so.
- `handle_signal_with_offset_logic`: Checks for specific signal bits and a non-null global pointer, then nullifies the global variable and calls another routine if conditions or certain signal bits are met.
- `reset_and_invoke_method`: Checks a global pointer and calls another routine if it is non-null, then nullifies the global variable, invokes a method via a function pointer offset from the saved pointer, and conditionally updates the global variable based on the return value.
- `execute_callback_and_check_result`: Recursively checks a global variable and executes a function through a pointer, returning a specific constant if the result is zero.
- `handle_global_callback_chain`: Checks a global variable and recursively calls itself if non-zero, then invokes a series of functions pointed to by offsets within a structure.
- `execute_chained_callbacks`: Checks a global variable and recursively calls another routine if non-zero, then executes a sequence of function pointers located at certain offsets from the global variable.
- `invoke_method_if_present_2`: Checks if a global pointer is non-null and calls another routine if it is, then calls a function pointer located at an offset of minus thirty-six from that global pointer and returns zero.
- `compute_fixed_point_product_2`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `execute_fixed_point_division`: Performs a complex fixed-point division using bit-shifting and conditional logic, returning a normalized result.
- `divide_fixed_point_normalized`: Performs a 32-bit by 32-bit fixed-point division using a normalization-based method, returning a 64-bit result containing a quotient and a remainder.
- `check_pointer_and_branch`: Loads a pointer from a global address, checks a byte from that pointer against a constant, and if it is greater than or equal to that constant, loads a value into a register and jumps to a new address.
- `initialize_system_resources_2`: Copies a block of memory, writes a zero to a location, calls a routine using a pointer from a global variable, and if certain conditions or return values match, calls another routine and updates a global variable.
- `evaluate_library_call_result`: Loads a relative address into a1, clears d0, calls a routine using a pointer from a6, and stores the return value in a global variable at an offset of one hundred or otherwise sets d0 to a constant and jumps to another routine.
- `test_subroutine_return_value`: Calculates a value based on a stack pointer offset, calls another routine with several arguments, and returns zero if the call returns one, otherwise it returns a value representing all bits set.
- `initialize_system_resources`: Allocates memory and signals, finds a task, and manages an Amiga library port.
- `validate_and_clear_entry_3`: Checks if a pointer returned by another routine is null or has bit 4 set, then clears the pointer and returns a status code based on a global variable's state.
- `validate_table_entry_index_2`: Checks if a provided index is within the bounds of a global table and if the entry at that index is non-zero, returning a pointer to the entry or setting a global error code to 9 and returning null if invalid.
- `check_signal_and_clear_global`: Checks for specific signal bits and a non-null global pointer, then nullifies the global variable and calls another routine if conditions are met.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `handle_library_call_results_3`: Evaluates results from library calls and global state, setting return codes and invoking cleanup routines based on success or failure conditions.
