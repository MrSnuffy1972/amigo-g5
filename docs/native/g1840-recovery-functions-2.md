# Game 1840 (eye of the beholder - advanced dungeons& dragons (1990)(ssi)[cr skr][a2]): code recovery summary

## Functions (part 2 of 2)

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR][a2](Disk 1 of 3).adf:eob2
(continued)

- `traverse_linked_list_6`: Loads a word from a certain offset in a3, adds a value from a5, loads another word from a3, adds a value from a5, checks d6, and otherwise jumps to a fixed address.
- `traverse_linked_list_5`: Recursively calls itself by loading a new pointer or address from a memory location offset from a5, while saving and restoring registers on the stack.
- `populate_and_validate_index_table`: Copies a block of data from a global variable into a local stack buffer, then executes a loop that searches for valid indices by checking a bitmask against a table, storing valid or modified indices into a local array and calling other routines to further process or validate those indices.
- `increment_d7_register`: Increments a 16-bit value in d7 and then branches.
- `increment_d6_and_branch_3`: Increments a 16-bit register d6 and branches to another routine.
- `branch_on_register_test`: Tests a register and, if it is non-zero, otherwise it jumps to another routine after pushing a constant onto the stack and calling another routine.
- `update_string_buffer_and_render_2`: Iteratively checks a bitmask or flag in a table, and if a bit is set, updates a string buffer or list of pointers with an incrementing counter or index, then calls a rendering routine with a calculated offset or calls a routine to tokenize an input list.
- `update_string_buffer_and_render`: Iteratively updates a buffer or list of pointers, checking for a termination condition or a maximum count, and calls a rendering routine with a calculated offset and a pointer-based string or character data.
- `check_and_call_font_update`: Tests a word at a pointer-relative offset in a5 and, if non-zero, loads a constant into d0, pushes it onto the stack, calls another routine, and branches to another routine.
- `update_string_display_buffer_logic`: Iterates through a list of string pointers, calculating horizontal offsets and calling a routine to render text at specific positions, while updating a global counter and checking for string or index boundaries.
- `process_string_graphics_attributes_2`: Iterates through a list of pointers, calculating string offsets and calling a routine to render text at specific horizontal positions, while updating a bitmask in a global structure and clearing certain memory-resident flags or pointers.
- `update_string_graphics_attributes_3`: Iterates through a list of pointers in a global structure, calculating string offsets and calling a routine to render text at specific horizontal positions, while also updating a bitmask in a global variable and modifying a counter in a structure.
- `calculate_scaled_index_2`: Calculates a value by adding a word from a structure offset to a register, sign-extends it, and divides it by nine; if the result is less than a value from a structure offset, it writes the result to a structure offset and branches to another routine, otherwise it overwrites a structure offset with a value from another structure offset and branches to another routine.
- `update_string_display_buffer`: Iterates through a list of pointers in a global structure, calculating string offsets and calling a routine to render text at specific horizontal positions, while clearing certain flags and updating a counter or index in a global variable.
- `calculate_ratio_and_branch`: Calculates a ratio using a value from a structure in a5 and a temporary value, stores the high word of the result back into a structure in a5, and branches to another routine.
- `calculate_inverse_ratio_and_branch`: Increments a value from a structure in a5, performs a division using another value from a structure in a5, stores the high word of the result back into a structure in a5, and branches to another routine.
- `call_tokenize_input_list`: Calls a routine that tokenizes a linked list of strings into an array of pointers.
- `process_status_string_format`: Clears certain bytes in a structure, then iterates through a string-based format or status-related table to check for specific control characters, updating status flags and calling routines to handle specialized output types like bar graphs or specialized string formatting.
- `parse_format_string_3_2`: Parses a format string to handle specialized output types like bar graphs, sounds, and tunes.
- `increment_structure_member_and_branch`: Increments a 16-bit value at a negative offset from a5 and branches to another routine.
- `process_state_and_update_pointers`: Checks a value in a structure and, depending on its value, executes a sequence of other routines that update or clear pointers, counters, and flags within that structure or a global variable.
- `reset_string_state_and_branch`: Calls another routine, clears a 16-bit value at a negative offset from a5, and then branches to another routine.
- `initialize_and_iterate_counter`: Initializes two counters in a structure, then enters a loop that calls another routine up to six times, incrementing a secondary counter if the call returns a non-zero value.
- `branch_on_comparison`: Compares a value to a constant, and if it is less than that constant, pushes a constant onto the stack, calls another routine, adjusts the stack pointer, and jumps to another routine.
- `update_string_state_and_render_2`: Resets a global counter and clears certain global values if a flag or counter is non-zero, then updates a string pointer and a state-tracking counter.
- `test_and_clear_register_d6`: Calls another routine, then checks if a 16-bit register is zero; if not, it clears register d6 and otherwise returns.
- `reset_string_rendering_state`: Clears a 16-bit value at a certain offset in a structure, sets a 16-bit flag to one, and clears a 16-bit counter in a structure, then jumps to a different routine.
- `reset_string_state_and_buffer`: Clears a 16-bit value at a certain offset in a structure, calls a routine to clear a stack-allocated buffer, and then clears a 16-bit value at a certain offset in a structure before branching to a new location.
- `initialize_structure_counter`: Pushes several constants onto the stack, calls a routine, then clears a 16-bit value at a certain offset within a structure pointed to by a5, and finally branches to another routine.
- `set_string_state_flag`: Sets a 16-bit value to 2 at a negative offset of 28 bytes from a pointer in a5, then branches to a fixed location.
- `reset_string_render_state`: Checks if a byte at a certain offset in a structure pointed to by a4 is zero, then updates that same byte with the result, and writes the value two to a certain offset in a structure pointed to by a5 before branching back to a previous instruction.
- `reset_string_render_state_2`: Checks if a byte at a certain offset in a structure pointed to by a4 is zero, then stores that boolean result back to the same location, and finally writes the value two to a 16-bit field in a structure pointed to by a5 before branching back to a previous instruction.
- `reset_string_render_state_3`: Checks if a byte in a structure is zero, then sets that byte to zero if it was non-zero, and resets two 16-bit values in a structure to a fixed constant value before branching to another routine.
- `update_resource_state_and_font_attributes`: Checks a global flag and, if certain conditions or state flags are met, resets a counter in a structure, updates a pointer-based table with new values, and conditionally calls other routines to update font or resource attributes or clear certain status flags.
- `setup_stack_and_call_subroutine`: Clears a register, pushes a longword from a pointer-based table onto the stack, calls a subroutine, and then adjusts the stack pointer before branching to another location.
- `set_string_state_flag_2`: Sets a 16-bit value at a negative offset of 28 from a5 to one and then branches to a nearby instruction.
- `update_string_attribute_and_buffer`: Checks a bitmask in a structure and clears a counter or resets a flag, then updates a string or buffer by copying a pointer-relative value and potentially calls other routines to update or clear memory or reset string attributes.
- `reset_string_attribute_flag`: Sets a 16-bit value at a negative offset from a pointer to 1, calls another routine, and then jumps to a different part of the program.
- `update_string_render_state`: Checks a global flag and a counter, then loops to clear a counter and reset certain global values.
- `reset_string_state_counter`: Compares a global word-sized variable to one hundred and fear-four, and if it is not equal, clears a global word-sized variable and otherwise returns.
- `reset_string_render_state_4`: Clears a 16-bit value at a negative offset from a pointer in a5 and then jumps to another routine.
- `reset_string_render_state_5`: Sets a 16-bit value at a negative offset from a5 to one and then branches to a subsequent instruction.
- `reset_and_update_state_counters`: Resets certain global counters and flags, and if a specific condition is met, clears a counter via a loop, updates a pointer or value in a structure, and conditionally calls other routines to update or reset state-related values or pointers.
- `calculate_string_index_and_load_pointer`: Calculates a new index by multiplying a value from a structure by five, uses this to fetch a byte from a table, increments it, and pushes a pointer and a new value onto the stack before calling another routine and storing the result back into a structure.
- `update_string_render_state_2`: Updates a string's display range and length by reading characters from a table and calculating a new end-pointer, then loops to advance a pointer through a string or resets a counter if certain flags or conditions are met.
- `fetch_pointer_and_update_cache`: Loads a pointer from an offset in a structure, stores it back to a different offset in that same structure, and then jumps to a routine that performs a comparison-based lookup or string-related state update.
- `traverse_linked_list_and_update_state`: Iterates through a linked list of pointers, checking if a certain bit is set in a byte at offset 57 of the current pointer's target and if a range overlap exists; if a valid node is found, it updates a pointer in a global structure and breaks the loop.
- `update_pointer_and_branch`: Loads a new address from a pointer in a3 into a3 and branches to another routine.
- `update_string_state_and_render`: Checks a global flag and a counter, then loops to clear a counter and reset certain global values.
- `update_inventory_string_display`: Iterates through a null-terminated string list to find a matching character, updates a global pointer or structure with a new address, and modifies certain flags or status bytes in a global structure or table.
- `traverse_linked_list_and_update_status`: Iterates through a linked list of structures, checking a bitmask at offset 55 and calling another routine with a length and pointer, while updating a counter or status flag based on certain return values or pointer-based comparisons, and finally calls a function pointer from a global table before returning.
- `update_status_from_comparison`: Reads a word from a structure at a negative offset from a5, calculates its negation, and stores the result as a word at a negative offset from a5 before branching.
- `set_status_to_one`: Stores the constant one as a word into a structure at a negative offset from a5 and then branches.
- `set_status_to_zero`: Clears a word at a negative offset from a5 and a longword in d4 to zero before branching.
- `update_linked_list_node_bounds`: Iterates through a linked list of structures, checking if a range (defined by an offset and length) overlaps with a range retrieved from a table, and updates a status or boundary value in the current structure if an overlap or specific condition is met; otherwise, it traverses to the next node in the list.
- `update_pointer_and_branch_2`: Loads a new address from a pointer in a2 into a2 and branches to another routine.
- `process_linked_list_and_update_state`: Iterates through a linked list of structures, checking a bitmask at offset 57 and calling another routine with a length and pointer, while updating a status byte at offset 25 and a short value at offset 22 of a structure in a5.
- `initialize_font_structure_2`: Traverses a linked list, initializes a structure using a table of pointers, and stores a word at a fixed offset relative to a global pointer.
- `reset_status_and_render_2`: Sets a 16-bit value at a certain offset in a structure to 2, calls another routine, sets a byte in a structure to 0, sets another byte in a structure to 0, loops 6 times calling another routine, sets a 16-bit value in a structure to 0, calls another routine, and calls another routine before returning a value from a pointer.
- `jumps_to_secondary_routine`: Performs an indirect jump to another routine located at a fixed address.
- `dispatch_subroutine_by_type_branch`: Clears a global variable, calls a routine with a calculated pointer, calls another routine, and then branches to a different routine or returns a 16-bit value based on the value of a byte in a global variable.
- `insert_new_node_3`: Copies a block of seven or eight dwords from a global variable to the stack, searches for a terminator, and if a pointer in a table is null, inserts a new node.
- `jump_to_address_9`: Performs a long jump to a specific address.
- `fetch_table_entry_and_traverse_2`: Loads a 16-bit index and a pointer from a table, populates several registers with data from that table, and calls a routine to traverse a table.
- `append_to_list_chain_2`: Checks a global variable and, depending on its value, traverses a list of pointers and appends or replaces entries in a global pointer chain.
- `jump_to_target_address_5`: Jumps to a long address located at a fixed memory location.
- `jump_to_indirect_address`: Jumps to a long address stored at a certain location.
- `jump_to_copy_routine`: Jumps to a long address that performs a memory block copy.
- `normalize_and_compare_table_values`: Iterates through a table of 16-bit values, replacing any zero or negative value with a calculated positive replacement, and calls another routine if the second byte of a pair is less than the first byte.
- `swap_and_update_global_2`: Compares a global variable to a value in a parameter, and if they differ, stores the old global value, updates the global variable with a value from the parameter, and calls another routine with the old value.
- `sum_and_branch_on_zero`: Calls another routine six times, accumulating its return values into a local sum, and if that sum is zero, calls another routine multiple times with various arguments including a global variable.
- `jump_to_clear_routine`: Jumps to a long address that clears two 16-bit words in a structure.
- `jump_to_block_copy`: Redirects execution to a routine that copies a large block of data using a fixed stride and offset.
- `traverse_list_and_clear`: Jumps to a fixed address.
- `update_string_buffer_and_validate_bounds`: Copies a small block of data from a global variable to a local buffer, then iterates through a table of structures to check string lengths and character values, potentially updating a local counter and modifying a character in a string buffer before calling another routine with a pointer to a string or attribute-related data.
- `validate_and_iterate_string_table`: Iterates through a table of string-related data, checking certain byte-based conditions and updating a local counter, then calls another routine with a pointer-based argument and a calculated offset or value.
- `process_conditional_string_length`: Checks a word at an offset of 8 bytes before a pointer in a5; if not zero, it pushes the value 59 onto the stack, calls another routine, adjusts the stack, and checks if the return value was non-zero to decide whether to continue or exit via a branch to a different routine; otherwise, it clears d0 and branches away.
- `parse_text_format_string_2_2`: Iterates through a string, checking for control bytes and updating a counter in a structure, then calls another routine to handle specific formatting or data-driven string manipulation.
- `increment_structure_member_and_branch_2`: Increments a 16-bit word at a negative offset of 28 from the address in a5 and branches to another routine.
- `parse_text_format_string_2`: Iterates through a string, checking for control bytes and updating a counter in a structure, then calls a routine to handle specific formatting or certain values.
- `increment_a3_and_decrement_offset`: Increments a3 by one, decrements a 16-bit value at a negative offset from a5, and branches to another routine.
- `parse_text_format_string`: Iterates through a string, checking for control bytes and updating a counter in a structure, then calls a routine to handle specific formatting or text-related logic.
- `increment_pointer_and_value`: Increments a longword pointer in a3 and increments a word in a structure at a negative offset from a5, then branches to another routine.
- `increment_structure_member_and_branch_3`: Increments a 16-bit value stored at a negative offset from a5 and branches to another routine.
- `parse_string_format_and_update_buffer_2`: Iterates through a string buffer, checking for specific control characters and updating a destination buffer with either literal characters or calculated values.
- `update_string_buffer_with_values`: Iterates through a buffer, checking if characters are valid digits, and if so, updates a destination buffer with a calculated value or a character from a source table.
- `parse_string_format_and_update_buffer`: Iterates through a string, checking for specific character patterns and updating a destination buffer with either literal characters or values derived from a table, while also updating a counter in a structure.
- `format_string_and_update_text`: Iterates through a string to parse format control sequences, updating a destination buffer with characters or formatted values, and calls a delay routine.
- `update_map_cell_data`: Adds a byte from a memory location to a value at an offset in a table, then enters a loop that increments a counter and checks if it exceeds a certain value, while calling other routines and performing conditional checks on data or pointers, and potentially resetting counters or calling other routines if certain conditions or certain values in a table are met.
- `increment_attribute_if_non_zero`: Checks if a word at a certain offset from a5 is zero; if not, it increments that word by one and branches to another routine, otherwise it returns.
- `update_string_attributes_and_font_data`: Iterates through a buffer using a counter, checking or incrementing character or attribute values in a table, and calls other routines to update or reset certain bytes or pointers based on conditional checks or counter-based logic.
- `process_string_attributes_and_font_data`: Iterates through a table or buffer, checking and incrementing character values or updating a counter, while conditionally modifying a global or calling other routines to update string attributes, font strings, or handle character-specific formatting or attribute-based logic.
- `update_string_attribute_and_counter`: Iterates through a table or buffer, checking or incrementing character values and updating a counter, while conditionally modifying a global flag and calling other routines or returning based on certain bit or value checks.
- `multiply_and_modify_buffer`: Multiplies d2 by d0, loads an address into a0 based on a4, subtracts a constant from a byte at an offset in a0, and branches to another routine.
- `update_string_buffer_and_format_text`: Iterates through a buffer, incrementing certain bytes and checking for a termination condition, then updates a string buffer or pointer-based structure with new values and calls a routine to handle string rendering or formatting.
- `update_string_attributes_and_call_handler`: Increments a byte in a buffer, then loops to scan a string or table, potentially updating a character or attribute or calling another routine.
- `update_font_string_and_attributes`: Increments a byte in a buffer, then loops to scan a string or table, potentially updating a character or attribute or calling another routine with a pointer or offset, and finally calls another routine via an indirect pointer or with a calculated address.
- `find_minimum_value`: Loops or checks a sequence of values in a table and returns a value containing a minimum or a certain index and a status flag.
- `increment_register_value`: returns the value of a register incremented by one.
- `process_graphics_and_string`: Performs a series of recursive calls, updates a global word, and executes a function pointer or copies a subregion of a buffer.
- `process_font_and_table_data`: Loads a 32-bit value from a structure, calls another routine, sets a global variable, calls another routine twice, resets that global variable, and then loops through a table to call another routine or returns if a counter exceeds a certain value.
- `increment_d7_and_branch`: Increments d7 by one and branches to another routine.
- `initialize_font_data`: Initializes a font structure using a provided pointer.
- `search_table_and_verify_bytes`: Loops up to 6 times, checking if certain bytes at an offset from a pointer are non-zero, and if so, calls another routine with a pointer-based address.
- `increment_d6_and_branch_4`: Increments d6 by one and branches to another routine.
- `search_table_with_conditional_branch`: Loops through a table and calls another routine with a pointer-based address, and if a certain condition is met, it loops through a sequence of bytes and returns a value or calls another routine with a hardcoded address.
- `increment_register_and_branch_4`: Increments a 16-bit register and branches to another routine.
- `increment_d7_register_and_branch`: Increments a 16-bit value in d7 and branches to another routine.
- `clear_d0_and_restore_registers_6`: Clears d0 and restores d2, d3, d6, d7, and a3 from the stack before returning.
- `initialize_font_and_copy`: Loads a pointer from a4, calls a routine, stores a word, calls another routine, and loops through a sequence of instructions that may call another routine or jump to a function pointer.
- `execute_delay_and_increment_d4`: Loads a pointer from a4, calls a routine at that pointer, increments d4 by one, and loops back to an earlier instruction.
- `initialize_font_structure`: Clears memory, initializes a font structure, and stores a word at a fixed offset relative to a global pointer.
- `initialize_font_attributes_wrapper`: Copies certain fields from a provided pointer to a global structure, masking a bit in a byte, and then calls another routine repeatedly while iterating through a table of function pointers until a null pointer is encountered.
- `find_char_in_string_2`: Scans a string for a matching character, decrementing a remaining count each step, and returns a pointer to the found character or a null pointer.
- `jump_to_target_address_9`: Performs a long jump to a target address located at a fixed memory address.
- `jump_to_subroutine`: Performs an indirect jump to a long address stored in memory.
- `search_table_for_match_2`: Loops through a table, checking if a value in the table matches a provided parameter or if a parameter is negative one, and if a secondary check passes, returns the index.
- `find_matching_table_entry_3`: Loops through a table, checking if a value or a character at an offset from that value matches a certain character, and if a secondary check occurs, otherwise it returns.
- `jump_to_target_2`: Performs a long jump to a target address.
- `update_string_buffer_value_2_2`: Performs a jump to another routine.
- `recurse_with_offset_parameters_2`: Calls another routine twice with a modified pointer and a value loaded from a global or memory-based location.
- `find_index_in_small_set`: Compares a provided value against a hardcoded sequence of three possible return values (zero, one, and two) by calling another routine and returns the matching index or a failure code if no match is found.
- `initialize_buffer_and_update_sprites`: Initializes a memory buffer with an incrementing sequence, copies sprite attribute data from a source buffer into a global table, writes a returned value to a global variable, and clears a memory location.
- `initialize_memory_and_call_subroutines`: Fills a block of memory with a constant value and then calls another routine while iterating through a table or a list of pointers, and calls another routine with a pointer-based address.
- `iterate_and_call_subroutine`: Calls another routine with a hardcoded address and then iterates through a range of memory or a table and calls another routine for each entry.
- `populate_lookup_tables`: Iterates through multiple loops to calculate and store or update values in several destination tables using bit-extracted or offset-based data.
- `initialize_memory_tables`: Sets a byte to certain value, clears a 16-bit counter in a global variable, clears several memory ranges or tables by setting values to zero or a specific byte, and populates a table with an index and a pointer from another routine.
- `initialize_attribute_lookup_tables`: Calls another routine, then loops 6 times to extract values from a table and call another routine, storing the result in a table; subsequently calls another routine multiple times to update specific table entries with returned values, and finally uses nested loops to call another routine with calculated offsets and values while updating a table.
- `execute_complex_sequence_2`: Executes a large sequence of various routine calls that involve updating a global variable, modifying a memory offset, and passing or calculating pointers or values via registers.
- `validate_resource_load_status`: Iteratively calls subroutines to check return values, updating a global variable with status flags (such as setting it to certain byte values) based on whether certain pointers or return values match error or success conditions.
- `copy_string_and_set_values`: Copies a null-terminated string from one memory location to another and then overwrites several bytes in a nearby memory range with fixed or calculated values.
- `execute_sequence_of_subroutines`: Calls a sequence of seven different subroutines and then returns.
- `update_string_buffer_and_font_attributes`: Updates a string buffer and certain font or display attributes by calculating offsets into a structure, modifying or clearing bytes and words, and calling several routines to parse format strings and traverse tables.
- `modify_font_and_string`: Calls another routine with a pointer, iterates through a null-terminated string, and then modifies certain values at an offset of a global variable multiplied by 16 before calling other routines.
- `increment_and_branch_to_subroutine`: Increments d6 by 1, compares d6 to 3, and if not equal, branches to another routine, otherwise calls a routine and then calls another routine while adjusting the stack pointer.
- `modify_font_data`: Calculates a pointer based on a value in a global variable and modifies certain memory locations relative to that pointer by adding or subtracting a constant.
- `update_sprite_attributes_3`: Executes several library calls, then copies attribute data from a source buffer into a global table using a loop, and finally updates a global variable and calls another routine.
- `load_buffer_pointer_and_jump`: Loads a pointer from a structure in a6 into a0 and jumps to another routine.
- `update_string_graphics_attributes`: Iterates through a null-terminated string, calling graphics-related library functions and updating a value from a table or a global variable based on whether a pointer or a certain flag is null, then returns a value representing a graphics attribute.
- `execute_complex_logic_2`: Calls another routine located at a long-word address.
- `calculate_buffer_offset_4`: Calculates a value by multiplying a byte, a byte, and a word from a pointer-referenced structure and adding a constant, or returns zero if the pointer-referenced byte is zero.
- `update_pixel_values_3`: Iterates through a memory range, compares bit patterns, and updates a destination pointer with modified bit values, then calls another routine if a certain condition is met.
- `accumulate_modulo_sum_3`: Iterates a certain number of times, calling another routine and accumulating the masked result into a running sum, then returns a value plus the sum.
- `resolve_string_and_update_table_2_3`: Resolves a string or pointer reference, copies a string into a local buffer, and updates a global table with the resulting or cleared pointer.
- `skip_null_terminated_strings`: Iterates through a sequence of null-terminated strings by advancing a pointer until a null terminator is found, repeating this process a specified number of times or until a counter reaches zero.
- `jump_to_subroutine_2`: Performs an indirect jump to a long address after writing a 16-bit value to a global variable.
- `execute_graphics_function_sequence_5`: Loads a pointer from a global structure and calls a sequence of three function pointers located at fixed offsets relative to that pointer.
- `parse_text_control_sequence_2`: Iterates through a string, checking for control bytes (such as carriage return or custom formatting codes), and recursively calls itself to process or append string segments while updating a global buffer or pointer.
- `traverse_linked_list_and_decrement_3`: Traverses a linked list to find a matching pointer, updates a pointer in the list, calls another routine, and decrements a global variable.
- `execute_recursive_sequence_3`: Recursively calls itself with modified parameters if a certain value is not negative, otherwise it returns.
- `jump_to_target_routine_4`: Performs an indirect jump to a routine located at a destination address stored in a memory location or register.
- `call_two_subroutines_with_stack_data_2`: Jumps to a routine that loads a word from an offset of 8 relative to the stack pointer into d7, pushes that word onto the stack, and calls two other routines.
- `transform_string_bytes_2_2`: Iterates through a null-terminated buffer and, if a bit in a lookup table (offset 11777 or 12031) is set, adds 32 to the byte before storing it back in the buffer.
- `process_string_buffer_and_update_display`: Calculates the length of a null-terminated string, truncates it if it exceeds a certain length, and then enters a loop that checks a character against a table or a global variable, potentially shifting or modifying the string buffer and updating a global flag or a character in a global buffer based on certain character or length conditions.
- `store_word_to_offset_2_3`: Stores a word from a register into a fixed offset relative to a global pointer.
- `jump_to_graphics_sequence`: Calculates a target address based on a value in a global table, clears two 16-bit words at that calculated location, clears a 16-bit word in a global variable, and calls execute_graphics_sequence_4.
- `jump_to_target_address_6`: Performs an indirect jump to a target address and calls a recursive routine that copies a memory block, jumps to a target, and subsequently jumps to a routine that clears two 16-bit words in a structure.
- `clear_list_elements`: Iterates through a list of up to 200 pointers, calls another routine if a pointer is non-zero, sets that pointer to zero, and clears a global variable.
- `update_table_with_offset_pointers`: Iterates through a table of values, replacing or updating entries using a fixed offset and a secondary table, then populates a subsequent memory range with repeated values or pointer-based increments.
- `test_bit_in_parameter`: Checks if a specific bit is set within a masked or modified version of the input parameter.
- `process_linked_list_nodes_3`: Recursively traverses a linked list, updating or creating nodes by reading values from a table and performing a bitwise XOR on a byte.
- `clear_multiple_lists`: Iterates through two different arrays of pointers, calls another routine if a pointer is non-zero, and sets that pointer to zero.
- `populate_door_attribute_tables`: Iterates through a table of door-related parameters and copies or calculates related values into a destination buffer.
- `process_pointer_range`: Calculates an end pointer, then iterates through a range of pointers, calling another routine if a pointer is non-zero and setting it to zero.
- `update_table_indices_2`: Iterates through several loops to read 32-bit values from a table, pass them as arguments to another routine, and store the returned 32-bit pointer into a destination table, then clears a 16-bit value in a global variable before returning.
- `update_table_indices`: Loops through a range of indices to call a routine that updates table entries, then loops again to call another routine that updates a different range of table entries, and finally calls another routine.
- `update_table_and_store`: Loops through a range of indices to call a routine that updates table entries, then loops again to call another routine, and finally loops through a range of indices to call a routine that updates table entries and stores the result in a table.
- `update_table_entries_and_clear_buffer`: Loops through a range of indices to call a routine that updates table entries using a value from a table, then loops again to call a routine that decodes compressed data and writes the result to a destination, and finally clears a word in a global variable.
- `update_table_entries`: Loops through a range of indices, calls another routine with data from a table, and writes the result to a destination offset, then clears a word in a global variable.
- `clear_word_at_offset`: Sets a word in a specific memory offset to zero.
- `update_table_with_derived_values_2`: Iterates through several loops to read 32-bit values from a table, calls another routine to process them, and writes the returned 32-bit result into a destination table at an offset relative to a pointer in a6.
- `update_table_with_derived_values`: Loops through a range of indices to call a routine that updates table entries, then loops again to call a routine that updates a different range of table entries, and finally calls another routine.
- `update_table_indices_with_derived_values`: Iterates through specific ranges of a table to calculate new values using a routine that processes or updates table entries, then stores the resulting pointer or value into a destination table at an offset relative to a pointer in a6.
- `update_table_entries_no_clear_2`: Loops through a range of indices to call a routine that updates table entries using a value from a table, then loops again to call a routine that decodes compressed data and writes the result to a destination.
- `update_table_entries_no_clear`: Loops through a range of indices, calls another routine with data from a table, and writes the result to a destination offset.
- `unlock_a5_and_restore_registers`: Restores d2, d3, d5, d6, d7, and a3 from the stack, unlocks a5, and then returns.
- `populate_buffer_from_table`: Uses multiple loops to read 32-bit values or 16-bit values from a table and write them into a destination buffer, or calls another routine to update buffer values using data from a table or a constant value, and finally clears a 16-bit value in a global variable.
- `decode_table_to_buffer`: Loops through a range of indices to call a routine that updates buffer values from a table, then loops again to call a routine that updates a different range of table entries, and finally calls another routine.
- `update_buffer_and_decode`: Loops through a range of indices to call a routine that updates buffer values from a table, then loops again to call a routine that decodes, and finally loops through a range of indices to call a routine that updates buffer values from a table.
- `update_buffer_and_clear_word`: Loops through a range of indices to call a routine that updates buffer values from a table, then loops again to call a routine that decodes compressed data and writes the result to a destination, and finally clears a word in a global variable.
- `update_buffer_values_from_table`: Iterates through a table of pointers, calls a routine to compute a value, and stores the result into a destination buffer, then clears a word in a global variable.
- `clear_word_at_offset_2`: Sets a word in a specific memory offset to zero.
- `copy_string_buffer`: If a parameter is zero, copies a string from a source address to a destination address and then copies it back from the destination to the unable source; otherwise, it calls another routine with a certain amount of data, then calls another routine to clear a certain amount of memory, and finally copies a string from a destination address back to a source address.
- `search_and_update_table_2`: Iterates through a table and calls another routine if a certain value in a table entry matches a global variable and a secondary value is non-zero.
- `clear_and_fill_memory_buffer`: Clears a block of memory (10,000 bytes) by setting bytes to zero in a loop, then, if a bitmask check against a global value and a bitmask-masked value from a global variable results in zero, copies 1,024 dwords from a source table into that same memory area, otherwise it calls another routine and returns.
- `parse_resource_string_parameters`: Parses a resource-formatted string or data stream to extract parameters, updating a global table and executing associated function pointers or instruction-driven logic.
- `copy_map_data_block`: Uses a nested loop to copy bytes from a source table to a destination buffer, then checks if a value exceeds 1168 to clear a 16-bit global variable and calls several other routines before calling a function pointer from a global variable.
- `increment_register_and_branch_5`: Increments a 16-bit register and then branches to another address.
- `check_map_size_and_copy`: Retrieves a value from a global variable, and if it exceeds 1168, clears a 16-bit value in a global variable and calls several other routines, then calls a routine via a pointer stored in a global variable.
- `decompress_and_increment_table`: Decompresses a stream of data into a destination buffer and subsequently increments values in a table based on offsets stored in another table.
- `resolve_pointer_and_call`: Loops until a routine returns a certain value, then calls another routine, and then calls another routine using a value from a pointer and a zeroed high word, and finally calls another routine using a value from a global pointer and a certain value.
- `set_bit_in_table`: Performs a series of routine calls and then sets a bit in a word at a specific memory offset, where the bit position is determined by the input parameter.
- `update_table_values_6`: Iterates through a loop that calls another routine, and if it returns non-zero, checks a table and subtracts a value from a table entry or sets it to one.
- `increment_and_branch_6`: Increments a 16-bit register, adds a constant to another register, and branches back to an earlier instruction.
- `restore_registers_and_return_alt`: Restores d2, d3, d6, d7, and a3 from the stack and then returns.
- `accumulate_table_values`: Iterates through a loop, calls another routine, and if the return value is non-zero, adds a value from a global variable to certain pointer-referenced integers if a bit-based condition is met.
- `verify_routine_execution_chain`: Executes an extensive sequence of routine calls with various parameters and returns a boolean result based on whether multiple return values or state-dependent checks match specific constants.
- `validate_and_update_resource_table`: Validates a sequence of table-based values against a calculated sum, then updates a temporary buffer with new pointers and executes associated function pointers.
- `process_table_and_string_data`: Clears a table of bytes, resets certain flags in a table, and then increments characters in a string if certain bitwise conditions are met.
- `jump_to_target_address_7`: Performs an indirect jump to a target address stored in a register.
- `execute_repeated_jump`: Executes a jump to another routine a fixed number of times using a loop controlled by a counter.
- `initialize_structure_contents`: Clears a block of memory, then populates a structure with several passed parameters and values from a table, and calls another routine.
- `execute_indirect_jump`: Performs an indirect jump to a fixed address.
- `tail_jump_to_shared_handler`: The routine consists of a single unconditional jump to a fixed target located in the same module, so it performs no work of its own and simply forwards control.
- `execute_handler_if_zero_2`: Checks if a value in a table is zero and calls another routine if so, then performs an indirect jump to a function pointer.
- `update_nested_pointer_2`: Calls another routine that recursively updates a pointer if a certain condition is met and checks if a pointer in a table is non-null.
- `traverse_pointer_list_with_conditions_2`: Traverses a linked list of pointers and calls another routine if a bitmask check against a byte at an offset of 19 from the current pointer passes and a certain pointer comparison fails, otherwise it jumps to another routine.
- `teardown_resources_2`: Conditionally invokes a callback if a structure field is nonzero, writes a word and a byte to a structure, then calls a routine to clear a l.
- `execute_indirect_function_5`: Checks if a table entry is zero or if a specific bit is set in a table entry, then calls an indirect function pointer.
- `parse_format_string_3`: Clears a word in a global variable, stores a parameter into a global variable, and then parses a format string while zeroing a byte at that address.
- `update_linked_list_node_3`: Calls a routine that checks if a pointer in a memory location is null and initializes a new entry in a table or updates an existing entry with new parameters.
- `find_minimum_and_update_3`: Iterates through a list, finds the minimum value in a table, and stores a calculated difference into a table, while also calling another routine.
- `swap_word_bytes_3`: Swaps the high and low bytes of a 16-bit value and returns.
- `parse_encoded_length_4`: Recursively calls itself to parse an encoded length by checking if a byte is zero, or three, or if a byte minus four equals zero.
- `evaluate_bit_conditions_3`: Calls a routine that checks multiple bit-based conditions against bytes stored in a table and returns a boolean result.
- `compute_fixed_point_product_3_3`: Calls a routine that takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value using cross-multiplication and partial products.
- `seek_file_position_2`: Calls a routine that performs a file pointer repositioning.
- `execute_function_pointer`: Calls another routine and then executes a function pointer found at an offset from a pointer loaded from a global variable.
- `update_pointer_table`: Loops 18 times to calculate a masked value from a table, stores it in a global variable, and calculates a corresponding pointer or index in another table, then sets two or more memory locations to a constant value.
- `calculate_parameter_difference`: Calculates the difference between a second parameter and a first parameter and calls write_parameter_sequence with that difference and the first parameter.
- `execute_parameter_offset`: A routine consisting of 148 instructions that calls another routine using a parameter and a calculated difference between two parameters.
- `populate_parameter_sequence_table`: Calculates an offset based on a provided parameter and calls a routine to write a sequence of four 16-bit values to a global memory table.
- `copy_data_block`: If a certain byte in a table is non-zero, it copies a block of words from a source table to a destination table if certain bounds checks pass.
- `map_and_modify_table_values`: Iterates through a source data structure and writes modified or flagged versions of those values into a destination table if certain bounds checks pass.
- `check_bit_mask`: Checks if bit 3 is clear in a byte retrieved from a table using an offset, returning 1 if the bit is clear and 0 otherwise; otherwise it returns 0.
- `initialize_and_validate_lookup_table`: Clears a block of memory, copies a value from a table into a global variable and a destination pointer, and then performs a series of conditional checks against a secondary table or bitmask, calling other routines if certain flags or error conditions are met.
- `traverse_linked_table_nodes`: Traverses a linked list of table nodes while performing conditional lookups in a secondary data structure.
- `traverse_and_update_node_list`: Iterates through a table to find matching certain identifiers and stores pointers to those entries in a temporary array, then traverses these pointers recursively, updating a value or checking a bitmask based on a conditional check of a global byte or a table offset.
- `traverse_path_nodes`: Traverses a linked list or pointer-based path, checking a secondary table for certain flags and recursively calling itself if a specific value is not present in a lookup table.
- `dispatch_table_logic`: Reads a value from a table and executes different sequences of routine calls or conditional logic based on that value.
- `traverse_linked_node_structure`: Iterates through a list of structure pointers, checking if a certain field matches a target ID, and then recursively calls another routine or a secondary routine based on conditional pointer arithmetic and table lookups, while updating a local array of pointers.
- `execute_nested_loops_2`: Executes another routine a fixed number of times within a nested loop structure if a certain condition is met.
- `traverse_node_list`: Iterates through a range of bytes and, if certain conditions or pointer comparisons are met, calls other routines using pointers or values retrieved from a table.
- `sync_global_value`: Checks if a global variable matches another value and, if not, calls another routine and updates the global variable.
- `execute_conditional_function_chain`: Executes a sequence of function pointers retrieved from a table or global structure, conditionally branching or clearing a global variable based on its current value, and includes a loop that waits for a memory-resident value to become zero.
- `return_incremented_value`: Pushes d7 onto the stack, sets d7 to zero, increments d7 to one, restores d7 from the stack, and returns.
- `jump_to_table_handler_target`: Jumps to the routine located at the address stored in a table-based handler.
- `write_parameter_sequence`: Calculates an offset based on the first parameter and writes a sequence of four 16-bit values to a global memory table.
- `dispatch_via_jump_table`: Performs a long jump to an address retrieved from a jump table.
- `set_graphics_parameter_2`: Calls a routine that updates a global variable or a table entry with a value derived from a calculation and a string length, then stores a short value into a global variable.
- `jump_to_routine_3`: Jumps to another routine located at a fixed address.
- `jump_to_target_routine_6`: Performs an indirect jump to a routine located at a long-address or a pointer stored in memory.
- `update_sprite_tile_attributes`: Iterates through a list of tile or sprite descriptors to update a buffer, applying bitwise masking and potentially looking up or substituting byte values from a global table.
- `calculate_table_index`: Calculates an index based on the absolute difference of two values and returns a byte from a table at that index.
- `copy_memory_block_3`: Copies a block of memory using a loop and then calls a library function while checking a return value against a constant.
- `compare_and_sort_differences`: Calculates the absolute difference between the upper 27 bits of two 32-bit values and the absolute difference between the lower 5 bits of those same values, then returns a 32-bit value containing the smaller difference and the sum of both differences in the lower 16 bits.
- `store_word_pairs`: Stores or modifies a series of word-sized values by combining bytes from a source pointer into a destination table.
- `parse_resource_format_strings`: Iterates through a table of resource format string pointers to parse and deallocate string or data references.
- `decode_compressed_data`: Decodes a stream of data by reading from a source, applying bitwise shifts, and writing the result to a destination buffer.
- `check_available_memory_2`: Uses a library call to check for available memory and returns a boolean indicating if the result exceeds a certain threshold.
- `setup_pointer_chain_4`: Allocates memory via a library call and sets a byte at an offset in that memory to a constant, then calls another routine with an offset pointer.
- `free_memory_if_nonzero_2`: Checks if a passed pointer is not null, and if so, calls a library function to free memory using a pointer from a global variable.
- `free_two_pointers_2`: Loads a pointer from a global variable and calls a library function using a pointer located 210 bytes before that address, then repeats this process using a pointer located 210 bytes before the previously loaded address.
- `free_pointer_array_and_global`: Checks a table of four pointers for non-null values and calls a routine to free any non-null pointer found, then calls a library function to free a pointer located at a global variable offset.
- `conditionally_free_memory_2`: Checks if a passed pointer is not null and if a certain offset within that pointer contains a non-zero value, then calls a library function to free memory.
- `free_memory_from_pointer_2`: Checks if a passed pointer is non-null, and if so, calls a library function to free memory using a pointer stored in a global variable.
- `free_mem_if_not_null`: Checks if a passed pointer is non-null and, if so, calls the memory deallocation library routine using an offset from a global pointer, returning the result of that call or zero.
- `traverse_and_free_recursive_2`: Recursively traverses a pointer-based structure by checking if a bitmask in a byte at offset 14 contains certain flags, then decrements a counter at offset 10 in the structure and calls a deallocation routine, otherwise it calls the deallocation routine and returns.
- `traverse_and_free_list_4`: Traverses a linked list and conditionally deallocates memory or clears list flags or table offsets based on a certain condition.
- `decrement_loop_with_delay_2`: Executes a routine, then enters a loop that uses empty delay loops to decrement a value, and finally re-enables interrupts or calls a library function.
- `initialize_audio_buffer_and_metadata`: Copies a string into a global buffer, allocates memory for a new structure, and initializes its members or pointers.
- `traverse_and_process_linked_lists`: Iterates through two different linked lists located at offsets 1666 and 1670 of a structure, calling another routine for each node while updating the list pointers with the value previously stored in the node.
- `clear_linked_lists`: Checks if a global pointer is non-zero, calls a function pointer, and then traverses and clears multiple linked lists or pointers.
- `initialize_structure_list`: Iterates through a list to copy a dword and increment a value in a linked list or table if a certain condition is met, then sets a bitset flag.
- `parse_resource_header_stream`: Iterates through a stream of resource-defined data, identifying specific type markers to allocate memory, copy string or byte data, and update resource metadata or linked lists.
- `parse_resource_buffer_2`: Parses a buffer containing tagged resource or string data, allocating memory and copying or transforming content based on detected type identifiers.
- `allocate_and_copy_buffer`: Allocates memory using a library call and copies a specific amount of data from a source pointer to the new buffer, then updates a pointer in a structure.
- `parse_tagged_string_buffer`: Parses a buffer containing type-length-value or type-length-string tagged data, allocating or updating nodes in a linked list and copying string contents or metadata.
- `remove_node_from_list`: Traverses a linked list and removes any node that matches a certain string comparison, while updating head pointers or orphaning nodes.
- `insert_node_into_list`: Inserts a new node into a linked list or updates an existing list, potentially clearing certain flags and updating a table or structure if a bitset or flag is present.
- `calculate_string_length_recursive_2`: Calculates the length of a null-terminated string and, if the length is non-zero, recursively calls itself using the pointer to the start of the string.
- `parse_resource_stream_3`: Iterates through a tagged resource stream to allocate memory and populate a table with string or data references.
- `clear_bit_and_recurse`: Calls a routine and, if the return value is non-zero, clears bit 5 of a byte at offset 14 of the returned pointer and calls the routine again with that pointer as an argument, otherwise it returns.
- `clear_list_flags`: Disables interrupts, then traverses a linked list to clear a specific bit in a byte, and clears certain offsets in a table or structure, then enables interrupts.
- `trigger_system_event_2`: Calls a library function to cause a system event and returns zero.
- `update_pointer_with_offset_4`: Reads a value from a memory location, adds it to the value stored at that same memory location, and stores the result back into that location, then calls three other routines and returns.
- `update_priority_list`: Traverses a linked list to find a node with a minimum value at a specific offset, updates a pointer in a table, and clears a bit in a byte or word.
- `replace_pointer_and_update_a4`: Copies a longword from a pointer in a4 to a new destination, updates a4 to point to that new location, and then branches to another location.
- `update_list_with_priority`: Traverses a list to find a node with the minimum value at a specific offset, updates a pointer in a table, and clears a bit in a byte or updates a linked list head.
- `restore_registers_r3`: Restores d4, d5, d6, d7, a3, a4, and a6 from the stack and returns immediately.
- `check_pointer_validity_and_branch`: Loads a pointer from a global table, clears a register, and checks if that pointer is null or if bit 0 of a byte at offset 14 of the pointed-to structure is clear, calling another routine if the bit is clear before branching or otherwise returning.
- `check_bit_and_branch`: Tests bit 1 of a byte at an offset in a4 and, if clear, calls another routine before branching to a nearby instruction.
- `update_pointer_list_3`: Iterates through a linked list, checking a bit in a byte at offset 14 of a pointer, and if bit 4 or bit 0 or bit 1 or bit 2 is clear, it updates pointers in a table or a global variable or calls another routine.
- `update_pointer_list_2`: Copies a longword from a pointer in a4 to a destination, updates a pointer in a table, and updates another pointer in a table before branching to another location.
- `move_and_jump`: Copies a longword from a pointer in a4 to d7, updates a4 to point to the next pointer, and branches to another location.
- `restore_registers_r4`: Restores d7 and a4 from the stack and returns immediately.
- `traverse_pointer_list_and_clear_bit`: Loads a pointer from a structure in a4, then loops through a list of pointers, calling another routine if a certain bit is clear, and updates a register with a new pointer or increments a counter.
- `clear_bit_conditionally`: Checks a byte in a global structure and, if it is zero, clears a specific bit in another byte.
- `process_pointer_list_2`: Saves registers d4 through d6 to the stack, clears d4, compares a value from a pointer in a2 to a value in a5, and if certain conditions are met, clears a bit in a global variable at an offset of a3, then returns.
- `process_pointer_list`: Iterates through a list of pointers, updating a pointer or a value in a structure, and clearing a bit in a global variable if a certain condition is met.
- `branch_on_byte_match`: Compares a byte in d6 to a constant, and if not equal, branches to another routine; otherwise, it shifts d5 left by 2, loads a pointer from a table, and copies it to an offset in a2.
- `compare_and_branch`: Compares a byte in d6 to a constant, and if not equal, branches to another routine; otherwise, it stores a byte from d5 to an offset of a2 and branches to another routine.
- `compare_byte_and_branch`: Compares a byte in d6 to a constant and, if not equal, branches to another routine; otherwise, it clears bit 0 of a byte at an offset in a3 and branches to a target address.
- `branch_if_not_equal`: Compares a byte in d6 to a constant, and if not equal, branches to another routine; otherwise, it clears bit 0 of a byte at an offset in a3 and branches to another routine.
- `branch_to_target`: Branches to a relative address.
- `restore_registers_and_return_18`: Restores d4 through d6 and the stack pointer, then returns immediately.
- `update_scaled_values`: Saves registers, loads a pointer from an offset in a2, checks bit 1 of a byte at offset 14, and if clear, loads a pointer from an offset in a4, stores it in a global variable, and loads a new pointer into a4.
- `calculate_scaled_struct_values`: Calculates or updates a set of scaled or bit-shifted values and stores them into a structure, including a loop that performs bit-shifting and addition based on a counter, and sets several fields in a structure at a certain offset with values derived from a pointer or a product of two other values.
- `decrement_and_shift_values`: Checks if d2 or d1 exceed certain limits, then loops to add d3 and d4 to d5, shifts d3 and d4 left by one, shifts d1 right by one, and decrements d2 until d2 or d1 reach a certain value.
- `store_calculated_pointers`: Updates a pointer in a1 with a new address, updates another pointer in a1 with a second new address, and stores d3 and d4 into a structure at certain offsets before branching to another location.
- `store_values_to_struct_2_2`: Stores a dword and two zeroed bytes into a structure at a specific offset, calculates a product and stores it in two locations, and optional.
- `store_values_to_struct_2`: Stores a dword and two zeroed bytes into a structure at a specific offset, calculates a product and stores it in two locations, and optionally stores a value from a global variable into a structure.
- `store_values_to_struct`: Stores a dword and two zeroed bytes into a structure at a specific offset.
- `update_pointer_and_check_bit_2`: Checks if bit 1 of a byte at an offset in a structure is set; if not, it updates a pointer in a structure with a new address and otherwise updates the pointer with a value from a global address and returns.
- `decrement_and_loop`: Increments a pointer, decrements a register, and loops if the register is not zero, otherwise it returns.
- `check_bit_and_update_pointer`: Checks if bit 4 of a byte at an offset in a4 is clear, calls another routine, and if the bit or a certain condition is met, clears two longwords at offsets 28 and 40 of a2.
- `update_pointer_and_clear_field`: Updates a pointer in a4 to point to a new location, copies a longword from that location to a0, and if a0 is not null, clears a longword at an offset in a2.
- `clear_structure_fields`: Writes a longword from a0 to a structure, clears a byte and a word in a structure, and if a pointer in a4 is not null, clears a longword in a structure before branching to another location.
- `set_uniform_values_3_2`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure, while also performing several other operations.
- `set_uniform_values_3`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure, while also performing several other writes to a structure using values from a table or registers.
- `set_uniform_values`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure.
- `update_object_property_tables`: Iterates through a table of pointers, and if a pointer's value exceeds a certain threshold, it updates a certain byte in a structure, clears a pointer, and calls another routine; otherwise, it updates length or offset fields and processes encoded pointer or offset data using a loop that handles special escape or end-of-table markers.
- `parse_pointer_table_data`: Iterates through a table of encoded pointers and offsets, updating a structure's pointer and length fields while handling special escape or jump-like byte markers.
- `shift_and_store_bytes`: Loads two bytes from a pointer, shifts the first byte left by 8, combines it with the second byte, and stores the result in a structure before branching.
- `update_pointer_with_offset_3`: Checks if a byte in a0 is a specific value, then if not, loads a byte from a0, increments it, shifts it left, and subtracts it from a0 before looping back to the start.
- `traverse_data_table_with_jumps`: Iterates through a data table, using bytes to calculate new pointer offsets or jump-like offsets (using division and modulo or bit-shifting) to update a pointer and a secondary offset value in a structure, while checking for specific end-of-table or jump-back markers.
- `update_pointer_with_offset_2`: Checks if a byte in d1 equals a constant, and if not, clears d1, loads a byte from a buffer, and updates a pointer in a0 using a calculated offset before looping back.
- `calculate_scaled_value_3`: Clears d0, loads a byte from a0 and increments a0, and if the byte is not negative, clears d2, loads a word from an offset in a4, multiplies d0 by d2, shifts the result left by 6, and clamps the result to a maximum of 16384 before branching.
- `modify_table_value`: Subtracts a constant from d0, shifts it left, and adds a word from a4-relative offset 46, then if d1 is not zero, writes the result back to that same offset in a4.
- `update_object_properties`: Writes a byte from d1 to an offset in a4, and if d1 is not equal to one, writes a word from d0 to an offset in a4 and clears a word at another offset in a4.
- `update_object_properties_from_table_2`: Calculates a new index or pointer using a modulo or division operation, then checks flags or pointer validity in a structure to update or create object properties, including setting certain bytes or pointers, and calls other routines to handle the updated state.
- `update_object_properties_from_table`: Iterates through a table of pointers, and if a pointer's value exceeds a certain threshold, it updates a structure's certain bytes and calls a routine to update a linked list or object property, setting a type or flag or certain offset values to a constant or a value from a pointer-based structure.
- `invoke_nested_routines_4`: Calls another routine passing a global variable and two constant values, then calls that same routine three more times without arguments.
- `insert_new_node`: Copies a block of seven or eight dwords from a global variable to the stack, searches for a terminator, and if a pointer in a table is null, inserts a new pointer or value into a list or table and increments a global counter.
- `initialize_clipped_bitmap_2`: Checks if a specified rectangular area or pointer-based structure falls within certain bounds and initializes a bitmap.
- `validate_and_set_bitmaps`: Checks if a source and destination rectangle or area overlap or exceed certain bounds, then calculates a memory range to populate a table with a bitmap or area address.
- `check_byte_mismatch`: Compares the lower bits of two bytes and calls a routine if they do not match.
- `scatter_bytes_to_buffers_5`: Copies a source pointer to five destination pointers separated by large offsets and calls another routine.
- `copy_block_with_offsets`: Copies bytes from a source pointer to five destination pointers separated by large offsets, using a loop that increments the destination pointers.
- `scatter_bytes_to_buffers_6`: Copies a source pointer to five destination pointers separated by fixed large offsets and calls another routine.
- `copy_block_fixed_offset`: Copies a block of memory from a source pointer to five different destination pointers separated by fixed large offsets.
- `copy_rect_buffer_blocks_4`: Performs a block memory copy or transfer if a source and destination rectangle or certain boundary limits are valid, calling another routine.
- `copy_subregion_with_bounds`: Calculates a subregion within a larger buffer and copies bytes from a source to a destination using large offsets, while checking certain bounds.
- `clear_two_words_3`: Clears two 16-bit words at certain offsets in a structure pointed to by a6.
- `clear_large_buffer`: Fills a large memory area with zeros starting from a pointer retrieved from a table.
- `clear_conditional_pointer`: Checks if a pointer in a table is non-zero and if a certain bit is clear, potentially calling another routine, and then sets that table entry to zero.
- `retrieve_table_pointer_3`: Loads a word and a pointer from a table using an index and calls another routine.
- `traverse_with_flag_update`: Recursively traverses a tree or linked structure while setting and clearing a bit in a flag or status byte, and retrieves a value from a table using a calculated offset.
- `copy_fixed_bytes_3`: Copies sixty-three bytes one at a time from one pointer to another.
- `execute_conditional_function_3`: Calls a routine, retrieves a pointer from a global variable, and if the return value of the called routine is non-zero, calls another routine using a pointer from an offset in that retrieved pointer.
- `execute_delay_if_nonzero`: Checks if a parameter is non-zero and calls a routine stored at an offset from a global pointer.
- `validate_and_reset_pointer`: Checks if a value returned by another routine is zero to decide whether to retry or proceed, and if non-zero, retrieves a pointer from a table using an offset and sets that pointer to zero.
- `execute_conditional_cleanup_2`: Checks if a pointer at an offset from a4 is non-zero, and if so, executes the function pointed to by that location.
- `process_string_and_call_function`: Iterates through a null-terminated string and calls another routine for each character, then updates a global byte and calls a function pointer if a certain condition is met.
- `execute_graphics_sequence`: Retrieves a pointer from a global and calls three different routines using offsets from that pointer.
- `fetch_table_entry_and_traverse`: Loads a 16-bit index and a pointer from a table, populates several registers with data from that table, and calls a routine to traverse a tree structure.
- `traverse_tree_without_flag_update`: Recursively traverses a structure using a value retrieved from a table, without modifying any flags in the input parameter.
- `update_pixel_values`: Iterates through a memory range, compares bit certain patterns, and updates a destination pointer with modified bit values before calling a delay routine if a change is detected.
- `execute_conditional_string_logic`: Executes a sequence of routines including a string search and a conditional function call, returning a value that determines if a certain pointer or value should be cleared.
- `execute_graphics_function_sequence`: Loads a pointer from a global structure and executes a sequence of three function pointers located at fixed offsets from that pointer.
- `execute_conditional_function_2`: Retrieves a pointer from a global structure and calls a function, then calls a second function if the first return value is non-zero.
- `find_string_index_3`: Iterates through a table of strings, calling a case-insensitive string comparison routine on each entry, and returns the index of the first match or a failure code if a null pointer or empty string is encountered.
- `traverse_linked_list_and_decrement`: Traverses a linked list to find a matching pointer, updates a pointer in the list, calls a function, and decrements a global counter.
- `process_list_and_string`: Iterates through a null-terminated string to call a routine for each character and traverses a linked list to call another routine while decrementing a global counter.
- `extract_bits_from_table`: Calculates an address using a table and offsets, then iterates a fixed number of times to extract specific bits from memory into a register.
- `search_string_table_3`: Iterates through a table of pointers, calling a routine to perform a case-insensitive string comparison against a provided parameter, and returns the index or offset if a match is found or if a null terminator is encountered.
- `sort_filtered_attribute_data`: Clears a range of ten dwords in a table, then iterates through a linked list to count and extract flagged bytes into a new buffer, and finally sorts that buffer using a bubble sort algorithm based on the extracted byte values.
- `initialize_registers_and_branch_2`: Saves certain registers to the stack, pushes a pointer from a register onto the stack, loads three constant addresses into registers a4, a3, and a2, and then branches to another routine.
- `process_event_and_update_flags`: Iterates through a linked list of event structures to clear or set bits in a bitset, potentially toggling certain global flags, and updates a circular event queue with new event data or status flags.
- `update_event_queue_and_flags`: Iterates through a linked list of event structures, setting or clearing bits in a bitset and updating a circular buffer or queue with new event type or data.
- `set_status_bits_on_match_2`: Compares a byte at an offset in a0 to a constant, and if it matches, clears d1, loads a word from an offset in a0, and if that word equals a constant, sets bit 1 of a byte at an offset in a1 and sets bit 0 of two bytes at offsets from a4.
- `branch_to_following_routine`: Performs a short branch to a subsequent instruction or routine.
- `set_status_bits_on_match`: Compares d0 to a constant, and if it matches, sets bit 2 of a byte at offset 8 in a1-relative and sets bit 1 of two bytes at certain offsets relative to a4, then moves a constant into d0.
- `branch_to_instruction`: Performs a short branch to an instruction located at a different memory address.
- `update_status_bit_on_match`: Compares a word in d0 to 232, and if not equal, clears bit 1 of a byte at offset 8 in a1-relative and clears bit 0 of a byte at a certain offset from a4-relative before setting bit 0 of that same byte, otherwise it returns.
- `check_and_set_flags`: Compares a word to a constant, and if not equal, clears or sets bits in memory or registers before potentially branching to another routine or returning.
- `process_event_queue_and_update_flags`: Iterates through a linked list of event structures, updating a circular event buffer with new event data and certain event types, while clearing or setting bits in a bitset and toggling or setting bits in global status flags.
- `call_add_int_server_3`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `set_negative_sign_char`: Checks if an integer parameter is negative and, if so, stores an ASCII minus sign in a local buffer before recursively processing the remaining buffer or a pointer-offset otherwise.
- `divide_shift_register_output_2`: Subtracts a value from another, increments it, calls a routine, and then performs a division and swap or addition using the result and a value from the stack.
- `calculate_difference_3`: Subtracts a 16-bit value at a certain offset in a structure from another 16-bit value at a different offset in the same structure and returns the result.
- `iterate_and_resolve_glyph`: Iterates through a loop that calls another routine and updates a value using a glyph resolution routine.
- `clear_d7_and_call`: Saves d6 and d7 to the stack, clears d7, and if d7 is not zero, calls another routine; otherwise, it moves d0 to d6, checks if d6 is non-zero, and if so, branches to another routine, or clears d0 and branches to another routine.
- `iterate_and_update_counter`: Iterates through a loop that calls another routine, and if a certain bitmask check passes, increments and wraps a global counter or pointer-based value.
- `return_d7_register`: Moves the value from d7 to d0 and restores d6 and d7 from the stack before returning.
- `traverse_linked_list_recursive`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call, and returns the final pointer or index.
- `push_d7_and_call`: Pushes d7 to the stack, clears d7, and if d7 is not zero, calls another routine; otherwise, it pushes d0 to the stack, calls another routine, adjusts the stack pointer, and moves d0 to d7 before branching to another routine.
- `restore_d7_from_stack_2`: Calls another routine, moves the value from d7 to d0, and restores d7 from the stack before returning.
- `save_registers_and_branch`: Saves d6 and d7 to the stack, clears d7, and if d7 is not zero, calls another or branches to another; otherwise, it moves d0 to d6, checks if d6 is non-zero, and if so, branches to another routine, or clears d0 and branches to another routine.
- `save_d6_and_call_routine`: Pushes d6 onto the stack, calls another routine, adjusts the stack pointer, moves d0 to d7, masks the lower 8 bits of d7, and branches to a previous instruction in the routine.
- `restore_registers_and_return_19`: Moves the contents of a register into a destination register and restores previously saved registers from the stack before returning immediately.
- `resolve_character_glyph_index`: Calculates a final glyph or sprite index by looking up a base value in a table and applying bitmask-based offsets.
- `resolve_table_index_with_offsets`: Looks up a byte from a table using a masked input, applies several conditional offsets based certain bits, and returns a calculated index.
- `update_state_and_retrieve_value`: Checks a global condition, calls a function, increments and wraps a global counter, and extracts or updates values from a table-based pointer.
- `get_table_value_2`: Checks if two consecutive halfwords in a global memory area are unequal and, if so, loads a longword from a table using an unable-to-determine offset.
- `test_bit_in_bitset`: Checks if a specific bit within a bitset stored in a global memory area is set, returning all ones if the bit is set or zero otherwise.
- `search_and_encode_string_pattern`: Searches for a sequence of bytes within a source buffer and, if a match or partial match is found, encodes the length and a pointer or a truncated version of the pattern into a destination buffer.
- `encode_lz_run_tokens`: Scans the source buffer for the longest earlier match of the current position, emitting either a literal byte or a back-reference token (offset and length) into a packed output stream, and marks run-length groups of identical bytes with dedicated tokens.
- `compress_string_data_with_backreferences`: Scans a source buffer for repeated or long sequences, replacing them with a custom encoding that stores either a literal byte or a backreference (length and offset) or a specialized long-sequence marker, and writes the resulting compressed stream to a destination buffer.
- `write_string_metadata_node`: Calculates a length-prefixed string or data block by storing a length byte and a 16-bit pointer or value, then writes these bytes to a destination buffer pointed to by a3.
- `write_string_metadata_to_buffer`: Writes a byte, a modified length, and a 16-bit value or pointer-related data into a destination buffer pointed to by a3, then branches to another routine.
- `compress_string_data`: Performs a custom string compression or encoding routine that scans a source buffer for repeated or long sequences, replacing them with a compressed or encoded format in a destination buffer, and handles length-prefixed or escaped character sequences.
- `increment_and_copy_byte`: Checks if a pointer in a6 is zero, and if not, loads a pointer from a3 into a6, sets a byte at that pointer to a constant, checks a byte at a6 against a constant, and if it is less than or equal, resets a6 to zero and loops back to the start.
- `increment_counter_and_copy_byte`: Increments a byte-sized counter at the address pointed to by a6, copies a byte from the source pointer in a2 to the destination pointer in a3, and then jumps to another routine.
- `set_bit_and_offset`: Writes a constant bit pattern to a memory location and returns a pointer offset by a certain amount.
- `decode_custom_stream`: Decodes a variable-length instruction-based data stream from a source buffer into a destination buffer, using instructions that support direct copies, relative jumps, and length-prefixed or offset-based block copies.
- `initialize_font_cache`: Checks if a global value is zero and, if so, calls a library function to populate it.
- `execute_recursive_function_calls`: Performs a sequence of recursive calls or calls to other routines using values loaded from a table.
- `scatter_bytes_to_buffers_7`: Calculates a source pointer using a base address and an offset, then performs a nested loop to copy blocks of data, where each copy operation copies bytes from a source pointer to multiple destination buffers.
- `copy_data_blocks`: Calculates a source pointer using a base address and an offset, then performs a nested loop to copy blocks of data, where each copy operation transfers multiple words or bytes to a destination via a pointer-based loop.
- `update_graphics_parameters`: Iterates through a string, updating a value from a table and calling several graphics-related library functions, then calls a routine that negates a value if it is negative and writes a byte to a pointer.
- `process_recursive_logic`: Performs a recursive check against a global value and potentially updates a table entry or executes an indirect function.
- `initialize_buffer_sequence`: Loads a pointer from a structure in a6 into a0 and calls another routine.
- `allocate_and_initialize_buffer`: Allocates a buffer, populates it with an incrementing sequence of values, and calls functions to insert a node and process a bitstream, otherwise it returns immediately.
- `traverse_and_clear_linked_list`: Iterates through a linked list starting from a global pointer and executes another routine on a data member of each node, then resets the global pointer to null.
- `reset_global_values`: Sets two global variables to zero and calls two library functions using a pointer retrieved from a global address.
- `clear_font_pointer`: Retrieves a pointer from a global address and calls two library functions, the second of which is a pointer clearing routine.
- `update_sprite_attributes`: Executes several library calls, then runs a loop that copies attribute data from a source buffer into a global table.
- `call_sprite_move`: Calls a library function using a pointer retrieved from a global address.
- `execute_indirect_handler`: Loads a function pointer from a table located via an offset from a global pointer and executes it.
- `check_bounds_overlap`: Checks if a provided range or coordinate falls within a certain area defined by values stored in a global structure, returning one if inside and zero otherwise.
- `save_and_restore_word`: Saves a word from a global memory location into a register, sets that memory location to zero, calls another routine, and then restores the original value.
- `update_ui_text_attributes`: Updates a global structure with a new attribute, modifies a table entry or a global value, and executes a sequence of function pointers or graphics routines while checking a condition in a loop.
- `handle_disk_loading_and_text_output`: Checks a byte in a global structure, and if not negative, writes a character and a space to a buffer before calling a routine to display text.
- `update_pointer_and_value_2`: Saves a value from a calculated offset, updates a structure with new values, and enters a loop that calls another routine while updating a global pointer or value.
- `set_negative_offset_value`: Sets a 32-bit value at a negative offset from a pointer to a negative constant and branches to another routine.
- `update_global_pointer_and_value`: Updates a global pointer or a value at a fixed offset relative to a global pointer via a recursive or conditional call.
- `find_string_match_and_return_index`: Iterates through a list of pointers to find a string that matches a provided string, then returns an index or a value of all ones if no match or null terminator is found.
- `calculate_address_and_load`: Calculates a new address by adding a value to a zeroed pointer, loads a word from that address into a register, and branches to another routine.
- `increment_register_and_branch_6`: Increments a register by one and branches to another routine.
- `return_all_ones_2`: Sets a register to a value of all ones and returns.
- `update_table_values_3`: Calculates an offset into a table based on a global value and updates four different entries in that table using a provided parameter or its multiples.
- `resolve_string_and_update_table_2`: Resolves a string or pointer reference, copies a string into a local buffer, and updates a global table with the resulting or cleared pointer.
- `search_and_update_table_index`: Checks a table of values for a specific index and updates a pointer in a table if a condition is met, or calls another routine to clear a pointer in a table.
- `find_min_priority_index`: First, it calculates the number of entries in a table by checking for a null terminator in a certain field of each structure.
- `teardown_display_and_resources`: Conditionally invokes a callback if a structure field is nonzero, writes a word and a byte to a structure, then calls a routine to clear a linked list, calls a routine to process a string, and executes a sequence of conditional calls to cleanup display context and system resources, including closing certain system resources, clearing a linked list, and resetting certain memory fields or display context parameters.
- `initialize_display_resource_nodes`: Allocates and initializes a list of nodes, setting up display-related properties and managing pointer-based buffers.
- `toggle_bit_planes`: Toggles five individual bits across five bit planes based on the low five bits of a control value, setting or clearing each plane bit accordingly, then returns.
- `set_bits_in_buffer_with_offset`: Calculates a pointer to a buffer and then loops to set or clear a bit at a certain offset, advancing the pointer by a fixed amount each iteration.
- `clear_and_set_bits`: Clears a bit in a byte at a certain offset and then sets a bit in a subsequent byte based on a bit-shifting loop.
- `call_avail_mem_3`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `update_bit_shifter`: Performs bitwise shifts and logical operations on bytes stored in a global memory area to propagate or clear certain bits.
- `check_and_execute_handler`: Checks if a pointer in a table is zero and calls another routine if so.
- `apply_bitmask_to_buffer_2`: Iterates through a buffer and, for each bit position, checks a bitmask or lookup table to decide whether to set or clear a specific bit in the buffer's bytes.
- `call_rem_int_server_3`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `search_string_list_for_match`: Iterates through a table of null-terminated strings and compares each one to a target string, returning one if a match is found and a specific flag in a global variable is non-zero, or otherwise returning zero or checking the next pointer in the list.
- `swap_word_bytes`: Takes a 16-bit value and returns a new value where the high and low bytes have been swapped.
- `rotate_word_bits`: Performs a complex bit rotation and byte swapping on a 32-bit or 16-bit input value.
- `retrieve_table_pointer_4`: Loads a word-sized index from a table, retrieves a pointer from that table, and calls a routine using that pointer while saving and restoring registers.
- `set_attribute_flag`: Sets a bit in a byte at a specific offset, calls another routine using a value retrieved from a table-based pointer, and clears a bit in a byte at a specific offset.
- `retrieve_table_pointer_5`: Loads an index and a pointer from a table, then loads additional data into registers before calling a routine that uses the pointer and calls another routine based on a bit check.
- `get_table_value_and_branch`: Retrieves a pointer or value from a table-based address and calls another routine if a certain bit is not set.
- `seek_file_position`: Performs a file pointer repositioning using a library call.
- `copy_large_block_2`: Loads a source pointer from a stack offset and calls a routine that copies a large block of data using a fixed stride and offset.
- `copy_large_block`: Copies large contiguous blocks of data from a source table to a destination buffer using a fixed stride and offset.
- `initialize_font_attributes`: Copies certain fields from a provided pointer to a global structure and calls another routine repeatedly while iterating through a table of pointers.
- `copy_and_execute_functions`: Copies sixty-four bytes from a source location to a destination and executes a function pointer found at a certain offset.
- `shuffle_table_elements`: Iterates through a table and performs swaps of bytes between a source table and the target table based on calculated indices.
- `calculate_buffer_offset`: calculates a value by multiplying a byte, a byte, and a word from a pointer-referenced structure and adding a constant.
- `calculate_scaled_product_3`: Loads a 32-bit value from a pointer in the stack, multiplies its upper 16 bits by its lower 16 bits, masks the result to 16 bits, and multiplies that result by five before returning.
- `update_circular_index`: Reads an index from a global variable, uses it to write a register value into a table, and increments the index modulo sixteen, updating the global variable if the new value differs from the current one.
- `set_string_graphics_attributes`: Iterates through a string, updating a value from a table and calling several graphics-related library functions.
- `copy_string_with_conditional_logic_2`: Reads a 32-bit integer from a source buffer to determine a total byte count, then enters a loop to copy bytes to a destination buffer.
- `test_and_execute_function`: Checks if a 32-bit value read from a pointer is non-zero, and if so, calls a function pointer located via an offset from a global base address.
- `parse_encoded_length_2`: Reads a byte from a pointer and recursively calls itself with an incremented pointer if the byte is zero or three, or if the byte minus four equals zero after an offset of eight, otherwise it returns the decremented byte value.
- `update_input_status_and_counters`: Decrements or increments several counters or bytes in a structure based on certain conditions, and updates a global variable with a value from a hardware register or a pointer-based source.
- `update_input_status_flags_2`: Sets bit 2 of a byte at an offset from a pointer, then checks if a value from a pointer-based source matches a stored value to update a secondary bit.
- `update_input_status_flags`: Sets a bit in a peripheral status byte, then checks if a value from a pointer-based source matches a stored value to update a secondary bit in that same byte.
- `update_input_register_and_table`: Updates the CIAA_PRA register or a pointer's target based on certain bit patterns in a register, then uses a loop to update a table or a global variable using a circular index-based logic or a bit-shifting pattern.
- `call_wait_b_o_v_p`: Library stub: loads its arguments and the graphics library base, calls WaitBOVP and returns its result to the caller.
- `parse_text_control_sequence`: Iterates through a string to identify control bytes, which trigger recursive calls to process or append string segments, and updates a global buffer or table with characters or new offsets.
- `invoke_subroutines_on_stack_2`: Sets up a stack frame, pushes a pointer from a5, calls another routine, pushes a pointer from a5, calls another routine, loads a value from an offset in a5, and returns.
- `call_two_subroutines_with_stack_data`: Pushes d7 onto the stack, loads a word from an offset of 8 relative to the stack pointer into d7, pushes that word onto the stack, calls two subroutines, and restores d7 from the stack before returning.
- `store_word_to_offset_2`: stores a word from a register into a fixed offset relative to a global pointer.
- `execute_graphics_sequence_4`: Executes a sequence of function pointers and clears certain memory locations, while waiting for a graphics blit to complete.
- `store_parameter_sequence`: Calculates an offset based on the first parameter and writes a sequence of four 16-bit values to a global memory table.
- `recursively_process_text_string`: Recursively traverses a string to calculate or update offsets or lengths based on a table or global structure, and may trigger a graphics callback if certain boundary or pointer conditions are met.
- `check_bit_and_execute`: Checks if a table entry is zero or if a specific bit is set in a table entry, then executes an indirect function.
- `execute_indirect_function_from_stack_2`: Loads a function pointer from an offset of 348 (348) relative to a value stored on the stack and executes it.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `resolve_nested_pointer_reference`: Recursively traverses a pointer-based structure, checking a bitmask flag and a global status flag, returning a final pointer or a failure constant.
- `parse_format_specifier`: Parses a format string for a printf-like operation, handling flags, width, precision, and length modifiers, then retrieves a variable argument from a pointer and converts it to a string buffer using helper routines.
- `parse_format_string`: The routine walks a format string character by character.
- `handle_delete_file_error`: Attempts to delete a file and, if the deletion fails, retrieves an error or message, stores it in a global variable, and sets a flag and a return value.
- `parse_string_to_token_array`: Parses a source string into an array of pointers to tokens, handling quoted strings and expanding certain escape-like character sequences.
- `initialize_global_pointers`: Updates several global memory locations with specific constant values and routine addresses, then jumps to another routine.
- `traverse_function_list`: The routine iterates through a list of function pointers, recursively calling another routine if a certain bit is set in a retrieved value, and finally calls a specific function pointer.
- `execute_conditional_callback`: Checks if a function pointer at a certain offset from a base pointer is non-null and calls it, then proceeds to execute a series of conditional function calls or pointer-based calls depending on the value of a global variable and other pointer-based checks.
- `transform_string_case_2`: Iterates through a null-terminated string and, if a certain bit is set in a lookup table based on a character, subtracts thirty-two from that character.
- `copy_string_with_limit_4`: Calculates the length of a source string, then copies a number of bytes from the source to a destination pointer up to a specified maximum length, ensuring the destination string is null-terminated.
- `transform_string_bytes_2`: Iterates through a buffer, checks a bit in a table, and if set, adds 32 to the byte before storing it back in the buffer.
- `update_pointer_and_write_byte`: Writes a byte from d0 to a pointer in a2, then increments that pointer and branches to another routine.
- `restore_registers_and_return_20`: Copies the contents of a register to another register and restores a previously saved list of registers from the stack before returning.
- `compare_case_insensitive_strings_3`: Compares two null-terminated strings by checking if any character (excluding certain punctuation) or its lowercase equivalent differs, returning the difference between the first non-matching or null-terminated characters or zero if equal.
- `prepend_string_to_buffer`: Finds the null terminator of a destination buffer, then copies bytes from a source pointer into the buffer starting at that terminator, effectively prepending the source string to the existing buffer, and returns the original destination pointer.
- `find_byte_in_string_3`: Iterates through a null-terminated string, comparing each byte to a target byte, and returns a pointer to the first matching byte or zero if no match is found or the null terminator is reached.
- `search_string_for_byte`: Loads a target byte from a local stack offset and a string pointer from a local stack offset, then calls a routine to find the first occurrence of that byte within the null-terminated string.
- `find_last_byte_in_string`: Iterates through a null-terminated string and stores a pointer to the last occurrence of a specific byte if found.
- `restore_registers_and_return_21`: Copies the contents of a2 to d0 and restores d7 and a2 through a3 from the stack before returning.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `set_high_bit_and_branch`: Sets the high bit of d0 and branches to another routine.
- `convert_number_to_ascii`: Converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_nibbles_and_move`: Sets a bit in d0, moves a word from a table using d3, and calls a routine that expands nibbles, then clears d0 and loads a value from the stack before branching to a previous instruction.
- `reverse_nibbles_from_table`: Expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `clear_and_branch`: Clears d0, loads a word from a pointer on the stack into d0, then branches to another routine.
- `prepare_fixed_point_division`: Loads a long from a pointer, sets a pointer, and if a value is negative, negates d0, calls a routine, and writes a byte to a pointer.
- `move_buffer_range`: If d0 is not zero, copies a byte from a source pointer to a destination pointer and increments or decrements the source pointer; otherwise, it clears a byte at the destination pointer, sets d0 to the destination pointer, and returns.
- `parse_signed_integer`: Parses a signed decimal integer from a string, updating a 16-bit value at a pointer and returning the number of characters processed.
- `increment_and_store_byte`: Increments a value at a certain offset from a pointer and then stores a byte through a pointer that is subsequently incremented.
- `parse_format_string_2`: Clears a word in a global variable, stores a parameter into a global variable, and then parses a format string while zeroing a byte at that global variable's location.
- `copy_memory_block_4`: Copies a block of bytes from one memory area to another using a count argument, handling overlapping source and destination by copying backwards or forwards.
- `traverse_and_free_list_2`: Traverses a linked list of pointers and calls a function pointer for each node before clearing the list head and a pointer in a global structure.
- `find_char_in_string`: Scans a string for a matching character, decrementing a remaining count each step, and returns a pointer to the found character or a null pointer if the count is exhausted.
- `retrieve_table_value`: Calls another routine to retrieve a value and then calls a second routine using a pointer from that value.
- `traverse_pointer_list_with_conditions`: Iterates through a linked list of pointers and calls another routine if certain bitmask flags and a pointer comparison or null terminator check are met.
- `handle_function_error`: Checks a global flag and executes a function from a table, and if a library call returns an error, executes another function from a table and updates a global variable with a new value or a constant.
- `seek_and_update_offset_2`: Checks a global flag, then calls a routine via a pointer from a structure, and if the return value is negative, calls another routine to update a global word and sets a value at an offset in the structure to a constant.
- `initialize_structure_fields_3`: Writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets, then calls a callback located at a fixed offset within that structure.
- `cleanup_display_context_3`: Conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets and calls two cleanup callbacks from a structure and a global table.
- `execute_indirect_function_3`: Checks a global variable and, if non-zero, recursively calls another routine before executing a function pointer retrieved from a global table.
- `compute_fixed_point_product_3`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value using cross-multiplication and partial products.
- `negate_and_divide`: Checks if d0 or d1 are negative, negates them if so, and calls a routine to perform a fixed-point division.
- `negate_operands_and_return`: Calls a routine, then negates d0 and d1 before returning.
- `negate_if_negative`: If d1 is negative, negates d1, calls a routine, and negates d0 before returning; otherwise it returns immediately.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method, returning a 64-bit result containing the quotient and remainder.
- `check_status_byte`: Loads a byte from a pointer-based location, compares it to a constant, and if it is not less than or equal to that constant, sets a register to a constant and jumps to another location.
- `update_status_on_branch_2`: Copies a block of memory, writes a zero to a location, calls a routine, and if the result is non-zero, checks a pointer and potentially calls another routine and writes a value before returning a status or zero.
- `update_status_on_branch`: Loads an address into a1, clears d0, loads a value from a global variable into a6, calls a routine, and if the result is non-zero, writes d0 to a global variable; otherwise, it sets d0 to all ones and branches to a final instruction.
- `update_global_and_return_status`: Writes a value to a global variable, calls a routine, and returns either zero or all ones based on the result of that routine.
- `return_zero_constant_3`: Clears register d0 and restores any used registers from the stack before returning.
- `allocate_and_set_node_3`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `create_message_node_3`: It obtains a global variable and calls another routine through a table.
- `get_table_entry_index`: Checks a value against a range and a table, then returns a calculated offset or a fallback value if any check fails.
- `traverse_function_list_2`: Checks a bitmask on a value returned from a library call and, if certain conditions involving a global pointer and a function pointer list are met, clears a pointer and recursively calls another routine.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR][a2](Disk 3 of 3).adf:Council.Cps

- `initialize_game_environment_2`: Initializes a global buffer by zeroing it, allocates memory, and parses a string or script token into a destination buffer, potentially handling quoted strings and escape sequences.
- `execute_conditional_cleanup`: Checks a pointer in a structure and executes a routine if it is not null, followed by calls to free or close memory-related resources.
- `populate_pointer_table`: Zeros a byte at an offset of 3048 in a structure, calls another routine several times with different arguments, and then iterates through a table of pointers to pass each one to another routine.
- `populate_string_pointer_table`: Iterates through a newline-terminated sequence to retrieve pointers and store them into a table, while passing these pointers to another routine via a table offset.
- `execute_complex_sequence`: Performs a variety of memory operations including copying a local buffer, updating tables, zeroing memory, and executing a sequence of function pointers, or bitwise rotations.
- `update_state_tables`: Sets a value in a structure, calls a routine, and then executes a sequence of loops that call various routines to update state tables, copy memory blocks, and initialize linked list nodes.
- `execute_graphics_sequences`: Executes a series of graphics-related routines, including zeroing memory blocks, updating rectangle boundaries, and processing a loop that reads bytes from a buffer to look up and execute function pointers or modify memory.
- `update_graphics_state`: Updates tables, modifies rectangle boundaries, and performs pixel color or bitwise updates while executing a sequence of function pointers.
- `initialize_graphics_buffer_and_sprites`: Initializes a global graphics state and populates a buffer with a sequence of drawing or copying operations.
- `update_state_table`: Iterates through a loop, checks values against a constant, and updates a table or memory offset by incrementing or resetting a byte, while calling other routines with parameters from a table.
- `process_state_table`: Copies a small amount of memory, then checks parameters to update a table and call other routines, or checks a value and potentially updates a table or calls a routine.
- `execute_graphics_sequence_6`: Sets up a local buffer, then iterates through a memory range, toggling a boolean and calling a sequence of function pointers.
- `update_rectangle_bounds`: Iteratively modifies or checks the boundaries of a rectangular area by incrementing or decrementing coordinate-related variables and updating a global memory location.
- `clamp_input_value_2`: Returns negative one if the provided integer is less than zero, zero if it is zero, or one otherwise.
- `store_high_bits`: Stores the upper 16 bits of a passed parameter into a global memory location and loops if a certain flag or value is not zero.
- `execute_sequence_of_routines`: Sets a byte in a global structure, calls several subroutines with or without arguments, and executes two functions retrieved via pointers from a table.
- `initialize_display_context`: Allocates memory for a display context structure, initializes a color map, and configures a new screen or window.
- `set_register_to_zero`: Sets the lower byte of d0 to zero.
- `check_available_memory_3`: Calls a library function and returns a boolean indicating if the returned value exceeds 5000.
- `setup_pointer_chain_5`: Allocates memory via a library call and sets a byte at an offset in that memory to a constant value of 4.
- `free_memory_if_not_null`: Checks if a passed pointer is non-zero, and if so, calls the free memory library function using a pointer retrieved from a global variable.
- `free_two_pointers_3`: Calls a library function using a pointer from a global variable and then calls it again using a second pointer from that same global variable.
- `free_pointer_list_and_global`: If a pointer argument is non-zero, it iterates through a list of four pointers at offset 4 of that pointer, calling another routine to free any non-null pointer found, then calls a library function to free a pointer stored in a global variable.
- `conditionally_free_memory_3`: Checks if a passed pointer is not null and if a certain offset within that pointer contains a non-zero value, then calls a library function to free memory, and then calls the library function again using a pointer from a global variable.
- `free_if_not_null`: Checks if a passed pointer is null; if not, it loads a function pointer from a global variable and calls it to free the memory pointed to by the passed pointer, otherwise it returns zero.
- `call_free_mem_if_valid_2`: Checks if a pointer passed as an argument is non-null and, if so, calls the memory deallocation library routine using an offset from a global pointer; otherwise it returns zero.
- `traverse_and_free_recursive_3`: Recursively traverses a pointer-based structure by checking if a bitmask in a byte at offset 14 matches certain values, then decrements a counter at offset 10 in the structure before calling a deallocation routine and returning.
- `clear_list_attributes_2`: Calls a routine, and if a certain value is returned, checks bits in a byte at an offset and calls another routine if certain bits match, then decrements a counter in a structure and calls a routine.
- `push_and_branch`: Pushes a longword from a pointer at an offset onto the stack, calls another routine, and then restores the stack pointer before branching to another location.
- `push_pointer_and_call`: Pushes a 32-bit value from an offset of a structure in a2 onto the stack, calls another routine, and then jumps to another routine.
- `push_and_call_subroutine`: Pushes a 32-bit value from an offset of 6 in a structure pointed to by a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `push_pointer_and_call_routine`: Pushes a 32-bit pointer from an offset of 6 in a structure pointed to by a2 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `decrement_and_free_memory`: Recursively calls itself with a pointer from a structure, then decrements a counter in that structure and calls a library function to free memory.
- `decrement_loop_with_delay_3`: Disables interrupts via a library call, executes a routine, then enters a loop that uses empty delay loops to decrement a value, and finally re-enables interrupts or calls a library function.
- `initialize_audio_buffer_and_event`: Copies a null-terminated string to a global buffer and, if a certain flag is unset, allocates memory via a library call to initialize a series of audio-related data structures, copies a template buffer into a loop of orf-set buffers, and configures a routine for system event or interrupt handling.
- `traverse_and_process_linked_lists_2`: Traverses two separate linked lists by following pointers from a global structure, calls another routine on each node, and updates the list pointers with the value previously stored in the node.
- `free_linked_lists`: Traverses two linked lists and calls a routine to free memory if a pointer or a certain global structure member is not null.
- `process_list_elements`: Iterates through a list to copy a dword from a pointer and increment a value at a secondary location, finally setting a bitmask in a global variable.
- `parse_resource_tag_stream`: Parses a stream of tagged data structures, allocating memory or copying string/byte sequences based on specific header tags.
- `parse_resource_buffer_3`: Iterates through a buffer of typed resource headers to allocate memory and copy or modify data.
- `allocate_and_copy_buffer_2`: Allocates memory using a library call and, if successful, copies a range of bytes from a source buffer to the new memory area while updating a pointer.
- `parse_data_stream`: Iterates through a buffer of tagged data, allocating memory and managing linked list nodes or string buffers based on specific operation codes.
- `reorder_linked_list`: Traverses a linked list and moves or reinserts nodes into a target list or a secondary list based on string comparison results.
- `insert_list_element`: Inserts a new node into a linked list and optionally updates a global pointer or clears attributes in a table.
- `calculate_string_length_recursive_3`: Iterates through a null-terminated string to find its length, then recursively calls itself if the string is not empty.
- `parse_resource_stream_4`: Parses a stream of tagged data or instructions, allocating memory for resource buffers and updating a string table or linked list.
- `clear_bit_and_recurse_2`: Calls another routine and, if the return value is non-zero, clears bit 5 of a byte at offset 14 of the returned address and calls another routine recursively.
- `clear_list_attributes`: Disables interrupts, then traverses a linked list to clear a specific bit in a byte, and clears several pointers or values in a table before re-enabling interrupts.
- `trigger_system_event_3`: Calls a library function via a pointer stored in a global variable and returns zero.
- `update_pointer_with_offset_5`: Reads a longword from an offset of 4 in a structure, adds it to the value stored at that same memory location, and calls three other routines before returning.
- `process_linked_list_nodes_2`: Traverses a linked list, clearing a bit in a byte at offset 13 of a pointer, and updates a pointer in a structure.
- `copy_and_branch`: Copies a longword from a pointer in a register to another pointer, updates a register with that value, and branches to another location.
- `find_minimum_node`: Traverses a linked list to find a node with the minimum value at a specific offset, clears a bit in a certain byte, and updates a pointer in a list.
- `restore_registers_r6`: Restores d4-d7, a3-a4, and a6 from the stack and returns.
- `load_pointer_and_branch_2`: Saves registers to the stack, loads a pointer from an offset of 1664 in a structure into a register, clears a register, and then branches to another routine or calls another routine based on a bit test or a zero check.
- `test_bit_and_call_subroutine`: Tests if bit 1 of a byte at offset 14 of a structure in a4 is set; if it is cleared, it calls another routine and otherwise returns.
- `swap_and_update_pointers_2`: Iteratively traverses a linked list, updating a global pointer and a pointer in a structure with a new pointer, and calls another routine if a bit in a byte at an offset is set.
- `swap_and_update_pointers`: Copies a longword from a pointer-based location to a new destination, updates a global pointer with the value previously at that location, and updates a pointer in a structure with a new value from a structure offset before branching to another routine.
- `move_and_branch`: Moves a longword from a register to another register, updates a pointer, and branches to a different location.
- `restore_registers_r7`: Restores d7 and a4 from the stack and returns.
- `check_pointer_range_2`: Loads a pointer from a structure, then loops while checking if a value is zero, calling another routine and updating a register or bitmask if certain conditions are met.
- `clear_bit_conditionally_2`: Checks a byte in a global structure and, if it is zero, clears a specific bit in another byte.
- `check_pointer_range`: Clears d4, then checks if a pointer or value at a certain offset in a2 is within a specific range, and if so, clears bit 7 of a byte at offset 104 of a3 before branching back.
- `process_instruction_stream`: Iteratively reads bytes from a buffer, updates a pointer, and performs conditional branching or table-based pointer updates or bitmask clearing based on the byte value, or returns if a certain limit is reached.
- `check_and_load_table_entry`: Checks if a byte in d6 equals 129, and if not, calculates an offset by shifting a value in d5, loads a longword from a table using that offset, and stores it into a global variable before branching to another routine.
- `compare_and_branch_2`: Compares a byte to a constant and branches to a fixed address if not equal, otherwise stores a byte from a register into a global variable and branches to another location.
- `compare_byte_and_branch_2`: Compares a byte to a constant and branches to a fixed address if not equal, otherwise performs a no-operation and branches to a fixed address.
- `test_and_set_bit`: Compares a byte to a constant and, if not equal, clears a bit in a byte at an offset within a structure before branching to another location.
- `branch_to_target_2`: Performs a short branch to a fixed address.
- `restore_registers_r8`: Restores d4-d6 from the stack and returns.
- `update_structure_pointer`: Saves registers to the stack, checks a bit in a structure, and if certain conditions are met, updates a pointer in a global structure and jumps to another routine.
- `recalculate_structure_bounds`: Calculates or copies dimension and offset values into a destination structure, using a loop to bit-shift and accumulate values based on a certain count, and then sets several fields or clears certain offsets in that structure.
- `update_structure_pointers_2_2`: Checks if a register is greater than a constant, and if so, checks if another register is less than a constant, then loops to add registers, shift registers, and decrement a counter.
- `update_structure_pointers_2`: Calculates new pointer values by adding offsets to a pointer, stores these new pointers into a structure at offsets 4, 8, 12, and 16, and branches to another routine.
- `update_structure_size_2`: Calculates a value based on a division, copies several longwords and words between structures, and sets certain fields to zero or a calculated value before returning a value.
- `update_structure_size`: Stores a longword into a structure at offset 28, clears a byte at offset 38, and clears a word at offset 36, then returns a value.
- `store_dword_and_zero_bytes`: Stores a longword from a0 into a structure at offset 40 of a1, then clears a byte at offset 50 and a word at offset 48 of that same structure before returning.
- `update_pointer_and_branch_4`: Checks a bit in a structure, and if it is not set, updates a pointer in a structure with a new address and branches to another location.
- `decrement_and_loop_2`: Increments a register, decrements a register, and loops if the register is not zero, then returns.
- `update_structure_fields_5`: Saves registers to the stack, clears a bit in a byte at offset 14 of a structure, calls another routine, and if the return value is not zero, updates a pointer in a structure, checks a bit in a byte at offset 14 of a structure, and if a bit is not set, clears two longwords at offsets 28 and 40 of a structure before branching to another routine.
- `check_table_entry`: Updates a pointer, loads a pointer from a table, copies a longword, and loops if a certain condition is met, otherwise clears a longword at an offset and branches.
- `update_structure_fields_4_2`: Writes a longword to a structure, clears a byte and a word at certain offsets, and if a pointer is non-zero, updates a longword in a structure before branching.
- `update_structure_fields_4`: Updates several fields in a structure, including a longword, a byte, and a word, while calculating a value using a loop that shifts registers and adds a constant.
- `update_structure_fields_3`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure, while also updating several other offsets in the structure.
- `set_uniform_values_2`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure.
- `update_instruction_stream_pointers`: Iterates through a list of structures, updating or clearing pointers and offsets based on a byte-stream of encoded instructions.
- `parse_byte_stream_instructions_2`: Parses a stream of encoded instructions to update a pointer and an offset or index in a structure, handling jump-like instructions that modify a destination.
- `load_and_store_bits`: Loads two bytes from a pointer, shifts the first byte, combines it with the second, and stores the result into a structure at an offset before branching.
- `update_pointer_offset`: Checks if a byte from a buffer in a0 is certain value, then clears d1, loads a byte from a0 and increments it, shifts it left, and subtracts it from a0 before branching back to the start.
- `parse_byte_stream_instructions`: Parses a stream of bytes to update a pointer and an offset or index in a structure, handling jump-like instructions that modify a destination address or a relative offset, and calls a handler routine with the new pointer and a value.
- `process_byte_stream`: Checks if a byte is certain value, otherwise it clears a register, loads a byte from a pointer, increments and shifts it, and loops back to an earlier instruction.
- `limit_scaled_value`: Clears d0, loads a byte from a pointer in a0, and if that byte is not negative, calculates a scaled value using a word from a4, clamps it to a maximum of 16384, and returns.
- `update_event_fields_2`: Subtracts a constant from a register, shifts it, adds a value from a global variable, and if a register is not zero, writes the result back to a global variable before branching.
- `update_event_fields`: Writes a byte from a register to an offset 50 in a structure, compares it to a constant, and if not equal, writes a word from a register to offset 46 and clears a word at offset 48 of a structure before branching to another routine.
- `update_event_structure_and_queue`: Calculates a new index or pointer, updates a structure or message-like object with new data, and calls a routine to handle the updated event or message queue.
- `process_event_queue_and_update_structures`: Iterates through a list of structures, clearing certain bits in a status byte at offset 52 and updating a secondary structure or message-like object with new pointer and length data if certain conditions or non-zero pointers are met.
- `invoke_nested_routines`: Calls another routine passing a global variable and the constant 92, then calls another routine three more times without arguments.
- `allocate_and_link_node_2`: Copies a certain number of words from a global variable to a local buffer, calls a memory allocation routine, and if successful, inserts a new node into a linked list by updating a pointer in the new node and updating a global pointer and a counter.
- `validate_bitmap_bounds`: Loads a pointer to a structure, a source/destination area, and a width/height or boundary value from the stack, calls clip_and_initialize_bitmap to check if the area or structure falls within bitmap or structure bounds, and restores the registers before returning.
- `clip_and_initialize_bitmap`: Checks if a source or destination area (defined by a pointer in a1) falls within the bounds of a bitmap or structure, and if so, calculates the necessary memory offsets to clear or initialize a block of memory with a specific value or pointer.
- `prepare_scatter_bytes_to_buffers`: Loads a source pointer from a table and calls scatter_bytes_to_buffers, then returns.
- `scatter_bytes_to_buffers`: Copies bytes from a source pointer to five or more destination buffers offset by large even increments, using a nested loop structure.
- `copy_bytes_to_buffers_2`: Copies bytes from a source pointer to multiple destination buffers using a single loop structure.
- `copy_bytes_to_buffers`: Copies bytes from a source pointer to five or more destination buffers offset by large even increments, using a single loop structure.
- `copy_rect_buffer_blocks_2`: Performs a block memory copy or transfer if a source and destination rectangle or certain boundary limits are valid.
- `copy_rect_buffer_blocks_3`: Checks if a source and destination rectangle or certain boundary limits are within a valid range, then performs a block memory copy or transfer using a nested loop.
- `clear_global_shorts`: Clears two 16-bit values at certain offsets in a structure pointed to by a6.
- `zero_memory_block`: Fills a large block of memory starting from a pointer retrieved from a table with zeros.
- `clear_conditional_pointer_2`: Checks if a value at a calculated offset in a table is non-zero and calls a function if so.
- `copy_sixty_four_bytes`: Copies sixty-four bytes from a source pointer to a destination pointer using a decrementing loop, then returns.
- `execute_conditional_callback_2`: Retrieves a pointer from a table using an offset from a4, and if the pointer is non-zero, executes a function pointer located at a negative offset from that pointer, then checks if the return value is non-zero to execute another function pointer at a different offset.
- `call_delayed_function`: Checks if a parameter is non-zero before calling a function pointer retrieved from a global structure.
- `validate_and_reset_object_pointer_3`: Checks if a value returned by another routine is zero, and if not, retrieves a pointer from a table, calls another routine with that pointer, and sets that pointer to zero.
- `invoke_conditional_cleanup`: Checks if a certain global pointer or a value at an offset from a4 is non-zero, and if so, calls the function pointed to by that value.
- `get_string_index_result`: Iterates through a string to retrieve a value and stores a byte into a global variable, then calls a function via a pointer if a certain global value is non-zero.
- `execute_graphics_setup_2`: Retrieves a pointer from a global variable and calls three different functions located at specific offsets from that pointer.
- `retrieve_table_pointer_6`: Retrieves a pointer from a table using an index and calls another routine.
- `retrieve_table_pointer_2`: Retrieves a pointer from a table using an index and may call another routine or return a concatenated value from registers if a certain register or memory value is zero.
- `update_pixel_colors`: Compares and updates bits in a destination buffer to match a source buffer, and calls a delayed function if a change or a specific bit-mask condition is detected.
- `execute_conditional_logic_2`: Invokes a series of routine calls and checks a return value, potentially calling another routine and returning a status or a constant value or zero.
- `execute_graphics_function_sequence_2`: Loads a pointer from a global structure and executes three consecutive function pointers located at fixed offsets from that pointer.
- `toggle_resource_lock_2`: Calls a function to attempt a lock and, if successful, calls another function to unlock a resource.
- `find_string_index_4`: Iterates through a table of strings, performing a case-insensitive string comparison on each entry against a provided string, and returns the zero-based index of the first match or a failure code if a null pointer or empty string is encountered.
- `search_and_decrement_counter`: Iterates through a linked list to find a matching pointer and then decrements a global counter.
- `traverse_table_and_recurse`: Iterates through a table of pointers, calls another routine recursively if a pointer is non-null, and clears that pointer in the table.
- `search_string_table_4`: Iterates through a table of pointers, calls another routine to perform a case-insensitive string comparison, and if a match is found, continues to the next entry or returns a calculated offset.
- `setup_address_registers`: Saves registers to the stack, copies a pointer from a register to the stack, and loads three constant addresses into registers a2, a3, and a4 before branching to another location.
- `update_command_buffer_and_bitset`: Checks a command type and updates a bitset or command buffer, potentially clearing or setting bits in a bitset or appending new command instructions to a buffer, and may toggle certain global flags or update a command queue index.
- `update_bitset_and_command_buffer`: Sets a bit in a bitset or buffer, then checks a command or instruction type to update a global state or append a new command or instruction to a circular buffer.
- `validate_and_set_status_flags`: Checks if a byte at offset 4 of a pointer in a0 is equal to 2, and if a word at offset 6 of that pointer is equal to 104, then sets bit 1 of a byte at offset 8 of a pointer in a1 and clears bit 0 of two bytes in a global variable before branching to another routine.
- `branch_to_target_4`: Performs a word-width branch to a fixed address.
- `set_flags_on_match`: Compares a register to 105 and, if not equal, otherwise it sets bit 1 of a byte at offset 8 of a pointer, sets bit 1 of a byte at offset 1674 of a pointer, sets bit 1 of a byte at offset 1675 of a pointer, and branches to another instruction.
- `branch_to_next_instruction`: Performs a short branch to a nearby instruction or another routine.
- `update_flags_and_branch`: Compares a register to a constant and branches if not equal; otherwise, it clears bit 1 of a byte at an offset of 8 from a pointer, clears bit 0 of a byte at an offset of 1530 from a pointer, sets bit 0 of a byte at an offset of 1531 from a pointer, and branches to a fixed address.
- `check_register_and_branch`: Compares a register to a constant and branches if not equal, otherwise it clears certain bits in memory and calls another routine.
- `process_command_stream`: Iterates through a linked list of command structures, updating a bitset or command buffer and modifying global state flags or bitset bits based on the command type or an associated value.
- `call_add_int_server_4`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `scale_shift_register_output`: Calculates the quotient and remainder of a value from a bit-shifting routine divided by a range derived from two global values, returning the result as a single 32-bit value where the upper 16 bits contain the quotient and the lower 16 bits contain the remainder plus an offset.
- `update_register_from_call`: Saves d7 to the stack, checks if d7 is zero, calls another routine, saves d0 to the stack, calls another routine, adjusts the stack, and loops back to an earlier instruction if a certain bit in d7 is set.
- `restore_d7_from_stack_3`: Restores the value of register d7 from the stack and returns.
- `clear_register_and_branch`: Saves d6 and d7 to the stack, clears d7, and if d7 is not zero, calls another routine; otherwise, it moves d0 to d6, and if d6 is not zero, clears d0 before branching to a nearby instruction.
- `increment_counter_on_zero_result`: Calls a routine that checks a table value, and if the lower byte of the return value is zero, increments a 16-bit counter at an offset of 1866 in a structure and applies a modulo 16 operation.
- `restore_registers_and_return_22`: Moves the contents of a register into a destination register and restores the previously saved registers from the stack before returning.
- `traverse_linked_list_recursive_2`: Performs a recursive traversal of a structure or linked list, updating a pointer or value with the return value of each recursive call, and terminates when a null or zero-value pointer is encountered.
- `execute_conditional_branching`: Saves d7 to the stack, clears d7, and if d7 is not zero, branches to another address; otherwise, it calls a routine, pushes d0 to the stack, calls another routine, cleans up the stack, moves d0 to d7, and branches back to a previous instruction in this routine.
- `call_and_restore_d7`: Calls another routine and then restores a 32-bit value from the stack into a register.
- `check_table_value_2`: Saves registers to the stack, loads a long-word from a5, and if that value is non-zero, sets d0 to zero and jumps to another routine.
- `transform_table_value_4`: Reads a byte from a table based on an index and applies conditional bitwise or arithmetic offsets to that value, then returns the result.
- `transform_table_value_3`: Reads a byte from a table based on an index, applies conditional bitwise or arithmetic offsets to that value, and returns the result.
- `branch_with_subtraction`: Tests bit 3 of a register and, if not clear, subtracts a constant from a register before branching to a fixed address.
- `transform_table_value_2`: Masks certain bits of a register and, if the result is not zero, subtracts a constant from another register before branching to another location.
- `check_value_range`: Compares a register against a constant and, if the value is not between 113 and 114, updates a register with a new value and jumps to another routine.
- `transform_table_value`: Reads a byte from a table based on an index and applies conditional bitwise or arithmetic offsets to that value.
- `map_input_to_output_2`: Uses an input parameter to look up a byte in a table and applies conditional bitwise or arithmetic offsets to that byte.
- `update_state_from_table`: Checks if a counter range is valid, calls a function, increments a counter, and loads a 32-bit value from a table into two 16-bit global variables.
- `load_table_entry`: Checks if a counter range is valid and loads a 32-bit value from a table into a register.
- `update_counter_with_conditional_skip`: Calls a function, increments a counter, and loads a value from a table, potentially incrementing the counter further if a certain bitmask matches.
- `decode_custom_stream_2`: Decodes a custom-format compressed or encoded data stream from a source buffer into a destination buffer using a variable-length instruction-based scheme that handles direct copies, relative jumps, and length-prefixed or offset-based block copies.
- `traverse_pointer_table_3`: Recursively retrieves pointers from a table using an offset and calls another routine or calls itself with modified arguments or a different pointer.
- `load_registers_from_stack`: Restores registers from the stack, loads a pointer from a stack offset into a register, calls another routine, and then restores registers before returning.
- `copy_interleaved_data`: Copies bytes from several source pointers offset by a fixed stride into a destination buffer using a nested loop.
- `initialize_linked_list_node`: Loads a pointer from a stack-relative offset into a register and calls a routine to allocate and link a new node into a list.
- `allocate_and_link_node_2_2`: Initializes a block of memory with a repeating pattern, sets a size header, and calls a routine to link a new node into a list.
- `traverse_linked_list_2`: Iterates through a linked list, calling another routine for each node, and sets a global pointer to zero after the loop finishes.
- `execute_two_functions`: Sets two global bytes to zero and calls two function pointers located via an offset from a global pointer.
- `save_restore_global_word`: Saves a word from a global memory location, clears it, calls another routine, and then restores the original value.
- `update_ui_and_handle_input`: Updates a table entry with a new value, modifies a global structure, and enters a loop that calls a function pointer from a table while checking a return value to decide whether to call a cleanup routine or continue or exit.
- `handle_disk_prompt_and_input`: Checks a global variable for a specific value, and if not equal to negative one, writes a character and a space to a buffer before calling a string processing routine.
- `update_display_and_linked_list`: Calculates an offset, updates a table entry with a new value, calls another routine, and enters a loop that calls a function pointer from a table, updates a pointer, and potentially calls another routine or sets a value to a certain constant.
- `set_negative_offset`: Sets a register to a negative constant and stores it at a negative offset from a register, then jumps to another routine.
- `update_buffer_and_pointer`: Updates a memory location using an offset or a value from a pointer and performs several calls to other subroutines.
- `compare_string_list_4`: Iterates through a list of pointers, compares the strings pointed to by those pointers, and if a null-terminated string matches, it traverses a linked list or tree to return a pointer or value.
- `calculate_table_offset`: Multiplies a register by a constant, calculates a new address by adding a constant, loads a word from that address into a register, and jumps to another routine.
- `increment_register_and_jump`: Increments a register and jumps back to an earlier instruction.
- `set_return_value_2`: Sets a register to a constant value and returns.
- `update_table_values_4`: Calculates an offset from a global variable and updates four adjacent memory locations with a value and its multiples.
- `manage_string_table_entry`: Searches a string table for a match and updates or clears a corresponding entry or pointer.
- `validate_table_index`: Calls another routine and, if the return value is non-zero, checks a table of values for a matching index, potentially calling another routine and clearing a pointer or setting a pointer to zero.
- `find_max_index_2`: Iterates through a table of structures to find the index of an entry that satisfies certain conditional checks, specifically finding the entry with the highest value for a certain byte-sized field, provided other certain conditions or range checks are met.
- `cleanup_display_context_6`: Clears a display context structure and invokes a series of cleanup callbacks or deallocates display-related resources.
- `rotate_bits_in_bytes`: Performs bitwise shifts and logical OR operations to move bits between three adjacent global bytes.
- `execute_conditional_function_4`: Checks if a value at an offset from a global pointer plus a parameter-based offset is zero, otherwise it calls a function pointer retrieved from an offset relative to a global pointer.
- `apply_bitmask_to_buffer_3`: Iterates through a buffer and, for each bit position, checks if a bit is set and uses a lookup table to decide whether to set or clear that bit in the buffer.
- `call_rem_int_server_4`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `compare_string_list_3`: Iterates through a list of pointers, comparing strings, and returns a certain value if a null-terminated string or a specific flag in a global structure is encountered.
- `get_table_entry_and_pointer_2`: Loads a 16-bit index and a 32-bit pointer from a table into d0 and a0, then calls a routine to process or retrieve a value before returning.
- `initialize_bitmap_area`: Checks a condition and potentially calls a routine to retrieve a value from a table-based pointer before returning a concatenated value.
- `prepare_interleave_source_buffers`: Loads a pointer from a table into a register and calls interleave_source_buffers, then returns.
- `interleave_source_buffers`: Reorders bytes from a block of parallel source buffers into a single destination buffer by incrementing pointers with a fixed stride.
- `update_structure_and_run_functions`: If a pointer is non-zero, it copies several values from the structure pointed to by that pointer into a global structure, then loops through a table of function pointers and calls each one until a null pointer is encountered.
- `restore_registers_and_return_23`: Restores registers from the stack and returns immediately.
- `copy_and_run_routine`: Copies sixty-four bytes from a source pointer to a destination pointer and executes a routine or two via a pointer or library call.
- `shuffle_buffer_elements`: Iterates through a buffer and swaps bytes using a lookup table and a decrementing counter.
- `calculate_scaled_product_4`: Loads two 16-bit words from a pointer-referenced memory location, multiplies them, scales the 32-bit product by five, and returns the result in d0.
- `update_index_and_store_data_2`: Loads a short from a memory offset, writes a 32-bit register value to a table, and updates a global index if a certain condition or wrap-around occurs.
- `update_string_pointer_and_value`: Iterates through a null-terminated string, calling another routine for each character, and updates a value from a table if a certain pointer is non-zero.
- `search_string_and_update_pointer_2`: Calls a routine to find a target byte, then checks a global variable and, if non-zero, loads a pointer from a table and loads a word from an offset of that pointer.
- `search_string_and_update_pointer`: Iterates through a string, calls a routine to find a target byte, and if the target is not found, calculates a new pointer or increments a pointer at an offset of 8 from a pointer.
- `increment_pointer_on_condition`: Increments a value at an offset of 8 from a pointer if the byte at that pointer is zero, otherwise it returns immediately.
- `restore_registers_and_return_25`: Restores several data and address registers from the stack, unlocks a semaphore, and returns.
- `copy_string_with_conditional_logic_3`: Reads a 32-bit integer from a source buffer to determine a total byte count, then enters a loop that parses encoded length segments.
- `call_function_if_not_zero`: Checks if a 32-bit value is non-zero, then executes a function pointer retrieved from an offset relative to a library or a global structure.
- `parse_encoded_length_3`: Recursively inspects a byte at a pointer and skips a certain number of bytes or calls itself with an incremented pointer if the byte is zero or three, or otherwise calculates a length-related value if the byte minus four equals zero or otherwise returns the modified byte value.
- `update_input_counters_2`: Decrements bytes in a structure, checks bits in a global variable to potentially increment or decrement other global counters, and calls another routine to update input status and a table.
- `set_hardware_register_bit`: Sets bit 2 of a hardware register and then compares two registers, otherwise it sets bit 1 of the hardware register and returns immediately.
- `update_hardware_register_flags`: Modifies certain bits in the CIAA_PRA register and a global byte based on comparisons with a memory-resident value.
- `update_input_status_and_table_2`: Updates the CIAA_PRA register or a pointer's target based on bit patterns in a register, then executes a loop that calls another routine to update a table or a global index using a bit-masking or bit-shifting logic.
- `call_wait_b_o_v_p_2`: Library stub: loads its arguments and the graphics library base, calls WaitBOVP and returns its result to the caller.
- `process_string_with_line_wrapping_5`: Iterates through a source string, handling control characters to trigger line-wrapping or buffer-update logic, and recursively calls itself to process or clear string buffers if a line-wrap threshold or specific control character is encountered.
- `process_string_with_line_wrapping_6`: Iterates through a source string, copying characters to a destination buffer while checking for a newline or a special control character.
- `setup_string_processing_context`: Pushes a constant onto the stack, calls another routine, pushes another constant onto the stack, calls another routine, adjusts the stack pointer, and then branches to another routine.
- `handle_string_control_characters`: Checks for specific control characters such as a form feed, a tab, or a carriage return within a string, and if found, terminates the current string buffer, calculates line-wrapping or tab-spacing requirements based on a global structure, and updates a destination buffer or pointer.
- `process_string_with_line_wrapping_2`: Iterates through a source string, copying characters to a destination buffer while checking for control characters and calculating line-wrap or wrapping.
- `process_string_with_line_wrapping`: Iterates through a string, copying characters to a destination buffer while checking for control characters and calculating line-wrap or wrapping-related offsets.
- `process_string_formatting_control_chars`: Checks for control characters such as a form feed, a tab, or a carriage return within a string and updates a destination buffer, managing line wrapping or padding or resetting a character counter in a global structure, and calls other routines to handle line wrapping or padding or string copying.
- `copy_string_with_line_wrapping`: Iterates through a source string, copying characters to a destination buffer while checking for carriage returns or special control characters.
- `parse_string_with_line_wrapping`: Iterates through a string, copying characters to a destination buffer while checking for special control characters (13, 2, or 128-set bit).
- `parse_string_with_formatting`: Parses a source string to handle tab, carriage return, and space characters, updating a destination buffer and managing line wrapping or padding.
- `process_string_with_line_wrapping_3`: Iterates through a source string, stripping high-order bits from bytes and handling carriage returns or control characters, while checking if the current line length exceeds a certain limit and calling routines to prepare line wrap data or append strings to a buffer.
- `format_string_with_line_wrapping`: Iterates through a source string, stripping high-order bits and handling carriage returns or control characters, while calculating if a line wrap or a new line or a buffer reset is required based on a character width or a maximum line length stored in a global structure.
- `parse_string_and_update_buffer`: Iterates through a string or buffer, checking for specific terminator or control bytes, and updates a destination buffer or clears a memory range.
- `process_string_buffer_with_wrapping`: Checks a character at a negative offset from a pointer in a5 and, depending on whether it is a certain control character or a space, performs a recursive or iterative process that copies or appends characters to a buffer, while checking against a maximum buffer length or a character width or a certain value in a global structure, and potentially inserts spaces or null terminators.
- `append_string_to_buffer_2`: Clears a byte at a certain offset from a pointer in a5, copies a string from a0 to that location using a call to append_string_to_buffer, updates a register, and branches to another routine.
- `check_string_length_and_load_word`: Compares a value in d0 with a byte at a negative offset from a5, and if not equal, loads a word from offset 24 of a pointer or a long pointer from offset 1872 of a4 into d0 and d1, then branches.
- `wrap_string_to_buffer`: Iterates through a source string, checking if the current line length or a carriage return would exceed a certain width, and copies characters to a destination buffer while inserting spaces or null terminators to handle line wrapping or string termination.
- `process_string_with_line_wrapping_4`: Iterates through a source string, checking for a carriage return or a null terminator, and copies characters to a destination buffer while checking if the current line length exceeds a limit or if a line-wrap or carriage return is encountered.
- `load_string_word`: Checks if a long pointer in d2 is null, and if not, loads a word from offset 24 of the pointer in a0 into d0 before branching to another routine.
- `format_text_line_wrapping`: Iterates through a source string, copying characters to a destination buffer while checking for a line-length limit or a carriage return.
- `load_string_word_from_offset`: Checks if a long pointer at offset 1872 of a structure in a4 is null, and if not, loads a word from offset 24 of the pointer in a0 into d0 before branching to another routine.
- `format_string_with_line_wrap`: Iterates through a source string, checking for a carriage return or a null terminator, while checking if the current string length exceeds a certain width or if a line-wrap or newline-induced wrap-around is required, and calls other routines to append or wrap text.
- `update_pointer_and_byte`: Increments a pointer in a0, increments d6, and writes a byte from a pointer-referenced location to the new pointer location before branching to another routine.
- `update_string_wrap_threshold`: Sets a byte to zero at a negative offset from a5, then checks if a long pointer at offset 1872 of a4 is null to determine a value for s4 or s3, and calls other routines with modified pointer or length arguments.
- `calculate_string_wrap_threshold`: Calculates a threshold or remaining length by subtracting a value from a pointer-based length and compares it against a register-based length, then calls another routine or calls another routine twice with different pointer offsets depending on the comparison result.
- `prepare_string_wrap_data`: Decrements d3 by 8, compares d3 to d0, and if d0 is not higher, pushes a pointer from a0 onto the stack, calls a routine, clears a word on the stack, pushes a pointer from a5 onto the stack, calls another routine, and resets the stack pointer before branching to another routine.
- `set_string_wrap_offsets`: Calculates and passes two new pointer offsets to a routine that determines string wrapping and line breaks, then restores registers d2 through d7 and returns.
- `invoke_subroutines_on_stack_4`: Pushes a pointer to a local stack-allocated buffer into a6-relative address and calls a string-processing routine and an integer-to-string routine using that same buffer as an argument, then returns.
- `store_short_to_global`: Stores a 16-bit value passed in a register into a global memory location.
- `execute_function_sequence`: Sets a 16-bit value to zero at a certain offset from a pointer, clears a global variable, and calls execute_graphics_function_sequence which executes a sequence of three function pointers loaded from a global structure offset.
- `write_multiple_values_to_table`: Calculates an offset based on a parameter and writes several 16-bit values to a table at that offset.
- `prepare_line_wrap_data`: Pushes d2 through d7 onto the stack, loads a word from a5, loads a value from a4, extends it to a long, shifts it left by 4, loads a pointer from a4, and if not zero, loads a word from a pointer in a0 and branches to another routine.
- `calculate_line_wrap_offsets`: Recursively calculates line-break offsets for a string by checking if its width exceeds a certain limit and updates a table with the calculated offsets.
- `calculate_string_line_wrapping_2`: Recursively calculates line-break offsets for a string by checking if its width exceeds a certain limit, and updates a table with the calculated line-break or wrap-around positions.
- `calculate_string_line_wrapping`: Recursively calculates string wrapping or line-break offsets by checking if a string's width exceeds a certain limit, then updates a table containing string-related metadata, including string lengths and layout offsets, by traversing string pointers.
- `update_string_layout_metrics`: Recursively calculates and updates a table of string-related metadata, including string lengths and layout offsets, by traversing string pointers and checking against a global width or font-size constant.
- `recalculate_string_metrics`: Calculates a new string length and updates a table of string-related metadata, including a loop that traverses a string until a null terminator is found, and recursively calls itself to update or validate certain table-based string length or offset values.
- `calculate_string_length_3`: Loads an address from an offset of 484 in a pointer, reads a word from a table using an offset of 6 plus a register, masks the lower 16 bits, divides that value by 8, and if a register is not zero, loads a pointer from an offset of 20 of that pointer and branches to another instruction.
- `calculate_string_metadata_length`: Calculates a value based on a constant and a register, stores it in a register, reads a word from a table using an offset from a pointer in a4, sign-extends it, and shifts it left by 4 bits before either returning or moving a pointer from d1 to a0 and reading a word from an offset of that pointer into d1.
- `update_string_metadata_and_length`: Calculates the length of a null-terminated string and updates a corresponding entry in a table with a value derived from the string length and a width or height value, then resets the first byte of the string to zero.
- `update_string_table_entry`: Updates a value in a table using an offset, iterates through a null-terminated string to calculate its length, and updates a pointer or value in a structure based on certain conditions.
- `prepare_string_length_table`: Loads a word from a structure in a4, extends it to a long, and shifts it left by 4 bits; if a pointer at offset 750 of the structure in a4 is not null, it loads that pointer into a0 and then loads a word from offset 14 of that pointer into d1.
- `update_table_with_string_length`: Calculates a value based on a pointer, writes it to a table, iterates through a null-terminated string to find its length, updates a table entry with the length, and if certain conditions involving a table and a value are met, clears a table entry and increments a counter.
- `update_table_with_offset_2`: Checks if a value or a certain condition involving a table and a register is met, and if so, sets a table entry to zero and increments a value at a pointer in the table, then clears a byte at an offset from a5.
- `update_table_with_offset`: Subtracts a constant from a register, compares it to another register, and if certain conditions are met, loads a value from a table, extends it, and clears a word at an offset in a structure.
- `update_string_and_table_conditionally`: Calls a routine with a string pointer and certain table values, then if a certain value is zero, calculates a string length, updates a table entry with that length, and if certain conditions involving a table and a value are met, clears a table entry and increments a counter; otherwise, it clears a table entry and increments a counter.
- `increment_table_entry_conditionally_2`: Iterates through a null-terminated string, calculates a value based on the string length and a pointer, checks a condition involving a table and a register, and if met, resets a table entry to zero and increments a value in the table, then clears a byte at an offset from a5.
- `increment_table_entry_conditionally`: Checks if a value derived from a table and a register is less than another value, and if so, sets a table entry to zero and increments a value at a certain offset in a table.
- `reset_table_entry_and_call_function`: Checks a range, clears a value in a table, increments a value in another table, and calls a function pointer from an offset.
- `execute_indirect_function_from_stack_3`: Loads a function pointer from an offset of 348 minus a certain value from a stack-stored pointer and calls it.
- `update_pointer_chain_node_2`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the stored value by four, clears the next word in the block, and stores the original pointer value into the following word before returning.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `get_pointer_with_validation`: Checks if a pointer is valid and returns a default maximum value if the pointer or a certain global condition or table entry is invalid.
- `handle_file_error`: Checks the return value of a function called via a pointer, and if zero, calls another function to update a global variable and a global or pointer-offsets value.
- `parse_script_token_string`: Parses a string or a quoted string containing escape sequences, storing the resulting tokens or processed characters into a destination buffer.
- `set_d0_constant_2`: Moves a value from d0 to d5 and then performs several or operations with zero on d0 before setting d0 to a constant value.
- `execute_conditional_functions_2`: Iterates through a table of pointers and executes a routine if a specific bit in a table-referenced value is not set.
- `transform_string_bytes`: Iterates through a null-terminated string and subtracts a value from any byte if a bit in a table-referenced value is set.
- `copy_string_with_limit_5`: Calculates the length of a null-terminated string from a source pointer, then copies up to a specified maximum number of characters to a destination pointer, ensuring the destination string is null-terminated.
- `compare_case_insensitive_strings_4`: Compares two null-terminated strings by checking if any character falls within the range of lowercase letters, converting such characters to uppercase, and returns the difference between the first non-matching characters or zero if the strings are identical.
- `append_string_to_buffer`: Finds the null terminator of a destination buffer and copies bytes from a source pointer into the buffer starting at that location until a null byte is copied, then returns the original destination pointer.
- `find_byte_in_string_4`: Iterates through a null-terminated string to find a specific byte, returning a pointer to the first occurrence or zero if the null terminator is reached.
- `search_string_byte_3`: Loads a target byte from a structure and a string pointer from a global address, then calls a routine to find the target byte within that null-terminated string.
- `find_byte_in_buffer`: Iterates through a buffer, updating a pointer to the last occurrence of a specific byte, and returns that pointer.
- `save_a2_to_d0_2`: Moves the contents of a2 to d0 and restores several registers from the stack before returning.
- `format_integer_to_string_2`: Converts an integer to a string of ASCII digits and returns the length of the resulting string.
- `format_integer_to_string`: Converts an integer to a string of ASCII digits and returns the length of the resulting string.
- `traverse_and_clear_list_3`: Traverses a linked list and executes a callback for every node, then clears a pointer or a field in a structure.
- `get_valid_pointer`: Calls another routine and returns either a pointer or a default maximum value, while checking if a certain global variable or table entry is non-zero.
- `traverse_linked_list_conditionally`: Traverses a linked list of pointers and calls another routine if certain bits in a table-referenced byte do not match a specific pattern and certain pointer values are not equal.
- `handle_io_error`: Checks a pointer-referenced value and, if a certain condition is met, stores a result and updates a status field.
- `update_position_offset`: Clears a 16-bit value at an offset in a structure, calls a function pointer, and if the return value is negative, updates a 16-bit value in the structure and sets a 16-bit value to a constant.
- `initialize_structure_bitmasks`: Sets a byte at offset 8 and two dwords at offsets 20 and 24 of a structure to all ones, then calls a routine located at a fixed offset within that structure.
- `cleanup_display_context_4`: Conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets, and finishes by calling two cleanup callbacks from the structure and a global table.
- `call_conditional_function`: Checks if a global value is non-zero before calling another routine and then executes a function pointer from an offset.
- `compute_fixed_point_product_5`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value by multiplying high and low 16-bit parts and combining cross terms.
- `conditional_negate_and_call_2`: Checks if d0 is negative and negates it if so, checks if d1 is negative and negates it if so, calls another routine, and then negates d1 before returning.
- `negate_and_return`: Calls another routine, then negates the contents of d0 and d1 before returning.
- `conditional_negate_and_call`: If d1 is negative, it negates d1, calls another routine, and then negates d0 before returning; otherwise it returns immediately.
- `normalize_and_divide_fixed_point_2`: Performs a series of bit-rotations and conditional checks on d1 and d0, and then calculates a fixed-point division result.
- `normalize_and_divide_fixed_point`: Normalizes a bit-width or scale factor by checking if a value is below certain thresholds and performing bit-rotations, then calculates a reciprocal-related value using bit-shifting and division.
- `load_library_and_set_value_2_2`: Sets up a stack frame, saves registers, loads a pointer from a table, checks a byte from that pointer, and if certain conditions are met, moves a constant into a register and branches.
- `load_library_and_set_value_2`: Copies a block of memory, writes a zero to a certain offset, calls a routine using a pointer from a library, and if the return value is non-zero, performs further checks and calls another routine.
- `load_library_and_set_value`: Calls a routine using a pointer from a library, writes the return value to a global variable, and if the return value is not zero, sets d0 to a constant and jumps to another instruction.
- `execute_indirect_function_from_stack_3_2`: Writes a value to a global variable, calls a routine with several arguments, and returns either zero or all ones based on the result.
- `return_zero_constant_4`: Returns a constant zero value without performing any other operations or memory accesses.
- `allocate_and_set_node_4`: Allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter, returning the pointer or zero if the parameter is zero.
- `create_message_node_4`: Obtains a global variable and calls another routine through a table; if the result is a sentinel, it returns zero, otherwise it calls another routine and, if that returns zero, calls a third routine, or fills a newly allocated node with parameters and fields from a table.
- `get_table_entry`: Checks if an index is within a certain range and if a certain table entry is non-zero, otherwise returns a pointer or a default value.
- `execute_conditional_functions`: Checks a bitmask in a register and, if certain conditions and a pointer-referenced value are met, clears a memory location and calls another routine.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR][a2](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR][a2](Disk 2 of 3).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR][a2](Disk 3 of 3).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_large_instruction_block`: This routine executes a large sequence of instructions that includes a branch back to an earlier instruction, forming a loop, and calls other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_4`: Restores a5 and a6 from the stack and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `modify_string_buffer_bytes`: Subtracts bytes from a buffer or a global variable using offsets or scaled indices, while decrementing pointers and copying words from a table or memory into a stack-allocated buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_buffer_and_call`: Copies a global variable to a pointer, then loops to zero out a buffer of 152 bytes, sets a global variable to zero, and calls a function pointer relative to the copied global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_memory_value`: Loads a constant into a4, moves a value from a memory location into a location relative to a6, and branches if a certain condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_buffer_and_call`: Loops to zero out a buffer of 152 bytes, writes a constant to a memory location relative to a pointer, writes a global variable to a memory location relative to a pointer, writes zero to a memory location relative to a pointer, and calls a function pointer relative to a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_calculate_buffer_size`: Clears a memory range, calls a routine to retrieve a pointer, and then calculates a new buffer size or pointer or value based on a conditional check of a pointer's contents, potentially updating a global structure or table with a new size or address, and calls another routine with a pointer and a length-related value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_stack_pointers`: Stores the value of a7 into a global variable at offset 498 of a structure in a4, then stores a7 into a global variable at offset 493 of a structure in a4, and checks a bit in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_buffer_size`: Stores a value from a6 into a global variable, calls a routine, and if the return value is not zero, retrieves a pointer from a structure, checks a pointer, and calls another routine with a pointer and a value, then updates a global variable with the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_and_pointer`: Loads a value from a structure in a3 into a register, stores it in a global variable, checks a condition and calls a trap, otherwise clears a bit in a register, updates a global variable, and updates a pointer in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_buffer_pointer`: Checks if a pointer in a structure is null, otherwise calculates a pointer and a size, calls a routine, and updates a global variable with the return value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_adda_adda_move`: Small routine of 3 instructions that adds a1 to a1, then adds a1 to a1, then copies d3 to -(a0).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_next_buffer_address`: Reads a byte from a pointer, updates a global variable with a pointer and a calculated size, calls a routine, and updates a global variable with the return value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `updates_pointer_and_decrements_counter`: Stores the contents of a1 into a global variable at an offset of 503 from the pointer in a4, then adds d1 to d0 with carry, decrements d1 by 7, and flips the even/odd bit of d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_initialize_buffer`: Calculates a size, calls a routine, and if the return value is not null, copies bytes from a source table to a destination buffer in a loop, then writes a constant to the destination buffer and copies bytes from another source.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_cleanup_sequence`: Copies bytes in a loop from a source to a destination, writes a byte to the destination, and calls several routines, including some via pointers in a structure and a global variable.
- `load_global_from_stack`: Loads several values from a stack-based area into registers, calls a routine, and then updates a global variable with values from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_pointers_from_offset`: Loads a pointer from an offset of a5 into a global variable, checks a bit, and if a condition is met, loads another pointer from an offset of a5 into a global variable, then checks a bit again and loads a third pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_set_data`: Copies a 64-bit value from a source pointer in a0 to a destination in a1, writes four 32-bit values to a structure offset relative to a5, calls a routine with a pointer and a range, and then checks a value at a certain offset before updating a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_pointer_and_bit`: Loads a constant address into a0, loads a pointer from a structure offset relative to a5 into a1, moves a value from that pointer to a fixed address, sets bit 1 of d0, and if a condition is met, loads another pointer from a structure offset relative to a5 into a1 and moves a value from that pointer to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_call_routines`: Copies a 64-bit value from a source pointer in a0 to a destination in a1, writes two 32-bit values to a structure offset relative to a5, calls a routine with a pointer and a range, calls another routine, and calls a third routine with a pointer and a range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_pointer_to_fixed_address`: Loads a constant address into a0, loads a pointer from a structure offset relative to a5 into a1, and moves a value from that pointer to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `cleanup_display_context_5`: Copies data between two pointers, calls another routine multiple times with different offsets, and executes a function pointer from a global structure before calling a final routine.
- `update_pointer_chain_node`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments that stored value by four, clears the next word in the structure, and stores the original pointer address into the following word before returning.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `reset_display_context`: Iterates through a loop checking certain structure members and executes a sequence of cleanup or deallocation callbacks, then calls a routine with a pointer, calls a routine via a pointer in a global variable, and returns a value from a structure offset.
- `parse_string_literal`: Skips leading whitespace in a string and parses a quoted or asterisk-delimited string literal, storing a pointer to the string or its content into a table and incrementing a counter in a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_decrement_counter`: Shifts a register, decrements a 16-bit value at an offset of a5, updates a pointer, and if a condition is met, decrements a 32-bit value and copies a byte from a table to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_with_special_characters`: Increments a 16-bit counter, checks for a quote character to advance a pointer and update a pointer in a structure, and handles specific character sequences like an asterisk followed by certain letters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_decrement_counter_2`: Updates a global pointer with a value from a register, decrements a counter in a structure, and copies a byte from a memory location pointed to by that counter into a memory location pointed to by a register, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_with_escapes`: Parses a string to handle quoted text and escape sequences, such as replacing certain characters with a newline or a substitute, and writes the processed characters to a destination buffer.
- `copy_string_with_limit_2`: Calculates the length of a source string, determines a copy limit based on a provided size, and copies bytes from the source to the destination until the limit or a null terminator is reached.
- `insert_into_list`: Attempts to insert a new block of memory into a linked list of memory blocks, updating a global counter and managing pointer redirection or splitting existing blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_offset`: Stores the value of d7 into a global variable at an offset of 261 from a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return`: Sets d0 to zero, restores registers d7, a2, a3, and a6 from the stack, unlocks a5, and returns.
- `traverse_and_clear_list`: Traverses a linked list, calls a routine for each node, and then clears two global pointers at offsets 952 and 948 of a4.
- `clear_global_pointers`: Subtracts the value in a0 from a0, then stores the resulting address into two global variables at offsets 952 and 948 of a4 before returning.
- `search_and_split_memory_block`: Checks a pointer, potentially clears it and calls a routine, then attempts to allocate a new block of a certain size and stores the size at the start of the new block.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_to_stack_pointer`: Calculates a new pointer address and stores a 32-bit value at the memory location pointed to by that new address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_pointer_and_restore_registers`: Copies a pointer from a register to another register and restores several registers from the stack before returning.
- `allocate_and_initialize_list_node`: Calls a memory allocation routine and, if successful, initializes a new node with a provided value, then updates a global linked list head and potentially a list-head pointer in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_pointers`: Writes a register value to an offset in a structure, copies a value from a structure to a global variable, and performs a check on a value before writing a pointer to a structure and calling a trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `insert_list_node`: Updates a linked list by setting the next pointer of a new node to a value from a structure, then updates several pointers in that structure to include the new node.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_and_pointer`: Stores a register value into a structure, checks a condition, and if not met, stores a register value into another structure and updates a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_list_head`: Checks if a pointer at offset 1c of a structure is zero; if so, it stores a3 at that offset, prepares a pointer at offset 12 of a3, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_push`: Stores a longword from a3 into a global variable at offset 285 of the structure in a4, loads a pointer to an offset of 269 from that new address into a0, and pushes a longword from a1 onto the stack pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_return`: Copies a longword from a0 to d0, saves a set of registers including d7 and a2 through a3 and a6 onto the stack, and returns.
- `split_linked_list_node`: Traverses a linked list of structures, where each node contains a size or length, to find a node that can be split or replaced to accommodate a new requested size.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_modify_memory`: Copies a longword from a pointer in a0 to the address pointed to by a2, updates a2 to point to that address, copies a longword from a3 to the address pointed to by a2, stores d0 at an offset of 4 from the address in a2, copies a longword from a3 to d0, and branches to another instruction.
- `validate_structure_bounds`: Calculates a memory range or size based on a value from a global variable, updates a structure or stack-based pointer, and checks if a value from a table or structure is within a certain range, otherwise it triggers a trap.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_decrement_pointer`: Decrements d1 by one and moves a shortword from a structure offset 325 into the address pointed to by a0, then decrements a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic`: Reads a shortword from a structure offset 324, calls two subroutines, calculates a value, calls a third subroutine, and if the result is non-zero, calls a fourth subroutine and a fifth subroutine before returning a value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines_and_restore_registers`: Calls a subroutine, saves d7 to the stack, calls another subroutine, branches to a later instruction, restores d6, d7, a2, and a3 from a stack frame, and returns.
- `validate_and_store_index`: Checks if a value derived from a register or a pointer-based memory access falls within a certain range, and if so, stores a modified version of that value into a memory location at an offset of 1005 from a register, while also updating a pointer-based memory location and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_lengths_and_update_table`: Iterates through a list of pointers in a structure, calculating the length of each null-terminated string and updating a global table with these lengths and certain constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_memory`: Stores the contents of d1 into a memory location at an offset of a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_length_and_modify_hunk`: Calculates the length of a null-terminated string by looping through its bytes, updates a structure with this length, and modifies hunk type codes or relocation-related flags before calling a trap-based error handler or another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_long_values`: Copies a long-word from register d1 to two separate long-word locations at an offset of 997 and 1001 from the pointer in a4, then copies a long-word from a pointer-based address in a5 to a long-word location at an offset of 5457 in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_string_and_trap`: Iterates through a null-terminated string starting at a pointer in a5, calculates its length, stores the length in a global variable, calls a routine via a pointer in a global variable, and performs a series of writes to memory or registers before triggering a trap and checking a value against a memory range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_suba_move_subq`: Small routine of 3 instructions that subtracts (a1) from a0, then copies a1 to -(a0), then subtracts 2 from d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `format_string_buffer`: Calculates the length of a string from a list of pointers, constructs a new string by concatenating these strings with spaces, and appends a newline character.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_value`: Stores a 32-bit value from d1 through a pointer in a0 into a global variable, decrements a 32-bit value in a structure, and copies a 32-bit value from a structure into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_string_length_and_copy`: Increments a 32-bit counter in a structure, calculates the length of a null-terminated string pointed to by a structure member, and copies that string to a new location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_values_to_structure`: Subtracts a pointer in a1 from a0, pushes a3 onto the stack, and copies a 32-bit value from a structure to a global variable, a 32-bit value from a structure to a global variable, and a 32-bit value from a structure to a global variable, then branches if a register is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `concatenate_string_list_and_update_buffer`: Concatenates a list of null-terminated strings into a destination buffer, calculating the total length, and then updates a pointer-based list or buffer with a value returned from another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_check_bit`: Clears an address pointer, copies a value from a global variable to an offset in a structure, and checks a bit in a register before updating a pointer and a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_check_bounds`: Calls a routine through a table, shifts a value and stores it in a global variable, calls another routine if a certain value is zero, calls a library function, and copies several dwords from a global structure to a new pointer, then checks a register against a value in a table and executes a trap if the condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_stack_and_table_ops`: Pushes a register to the stack, checks a value in a table, moves a value from a stack-relative location to a register, and moves a value from a constant address to a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_task_structure_and_queue`: Updates a structure or task object by copying several pointer-based or integer values, modifies a flag or status byte, and uses a library call to add a new element to the end of a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_register_value`: Sets a 16-bit value in a register to a constant and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields`: Checks a condition based on a pointer offset, then either writes a zero or a value from a function call to a structure, and subsequently copies several values from a source structure or registers to a target structure, including a calculation involving a bit shift and a call to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `move_pointer_and_data`: Updates a pointer, copies a value from a pointer-based address to a global, and copies a value from a register to a global, then copies a value from a register to a global and copies a value from a pointer-based address to a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_function_calls`: Writes values to a structure, calls a routine via a pointer from a global variable, calls another routine via a pointer from a global variable, and writes the result to a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_register_to_global`: Copies a value from a register to a global, copies a value from a pointer-based address to a global, copies a value from a register to a global, and copies a value from a pointer-based address to a global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_chained_routines`: Loads a pointer, copies a value to a global, loads another pointer, loads a third pointer, loads a constant, calls a routine, loads a pointer, loads a value from that pointer, loads a constant, calls another routine, and compares a register against a global value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_values`: Copies a longword from an offset of a1 to an offset of a0, then copies a longword from an offset of a5 to another offset of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_match`: Calls a function via a pointer in a global variable, then enters a loop that calls another function via a pointer in a global variable, stores the result in a structure at an offset from a5, and calls a third function via a pointer in a structure if the result does not match a value at an offset from a5, finally calling two other routines before returning.
- `find_string_path_end`: Scans a string for a null terminator, a colon, or a forward slash, then uses the resulting pointer to traverse a linked list of structures, saving and restoring a value at an offset of 184 in a previously retrieved structure, and returns a value or a certain error code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_store_register`: Clears register a1, stores a 32-bit value from a global address into a structure at offset -37 using a6, and stores d1 into a structure at offset an address in a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_file_path_and_open`: Checks if a string contains a directory separator, then attempts to open a file by retrieving a pointer from a structure and iterating through a list of potential file locations.
- `compute_fixed_point_product_4`: Takes two 32-bit fixed-point values from d0 and d1 and returns their product as a 32-bit fixed-point value by multiplying high and low parts and combining cross terms.
- `normalize_fixed_point_division`: Performs a complex fixed-point division involving bit-shifting and conditional logic, returning a result that may involve a loop.
- `normalize_fixed_point_divide_2`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method, returning a 64-bit result with the quotient in the high 32 bits and the remainder in the low 32 bits.
- `allocate_and_initialize_structure`: Calls a function via a pointer in a global variable, and if the result is not a failure, calls another routine via a pointer in a6 to initialize a structure at a2 with a pointer, a byte, and other fields, then sets a pointer or calls another routine based on a parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_structure`: Stores a 32-bit value from a3 into a structure at offset 267 in a2, decrements a0, and stores a byte from d1 into a structure at offset 265 in a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields_4`: Sets four bytes at specific offsets within a structure pointed to by a2, calls another routine through a table to set a dword at offset 16, and then either initializes a linked list or calls another routine based on a parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_store_register`: Clears a1, checks a value at offset -37 from a7, stores d1 into a structure at offset 273 in a2, decrements a0, and if a3 is not null, stores a3 into a structure at offset 41048 in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields_2`: Calls a routine via a pointer in a6, stores the result at an offset in a2, and if a3 is zero, initializes several fields in a structure starting at a2; otherwise, it calls another routine via a pointer in a6.
- `process_message_and_set_pointer`: Calls a routine via a pointer in a global variable, calls another routine via a pointer in a6, calls a third routine via a pointer in a6, copies a 32-bit value from a structure at offset 24, calls a routine via a pointer in a6 with an incremented pointer, and stores the address of the current routine into a pointer in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_pointer_and_counter`: Clears a register, checks a value on the stack, copies a register to a global variable, increments a byte in a table, checks another stack value, loads a new address into a register, checks a third stack value, and copies a register to another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_pointers`: Executes several function pointers from a structure, writes a value to a global variable, and calls a routine that restores registers and increments the stack pointer.
- `adjust_stack_and_restore_registers`: Increments the stack pointer by 4 and restores a4 and a6 from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_memory_2`: Stores a 32-bit value from a register into a memory location at an offset from a pointer in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_routines`: Calculates a value using a pointer from a4, sets a global variable to a constant, calls a routine via a pointer, sets another global variable to zero, sets a third global variable to a4, and calls two routines via pointers in a6.
