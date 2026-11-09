# Game 1838 (eye of the beholder - advanced dungeons& dragons (1990)(ssi)[cr skr](disk 1 of 3)[a3]): code recovery summary

## Functions (part 2 of 2)

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR](Disk 1 of 3)[a3].adf:eob2
(continued)

- `update_entity_state_and_timers`: Iterates through a loop to increment a counter in a structure while calling other routines, updates specific status bytes in a global structure based on a state value, and manages a linked list of entities by comparing and updating pointer values.
- `push_parameters_and_call_subroutine`: Pushes the value fifteen onto the stack, calls a subroutine, increments the stack pointer by two, writes the value one to an offset of twenty-eight from a pointer in register a5, and branches to another location.
- `update_entity_state_logic`: Checks status flags in a structure to reset counters, executes a loop that calls itself up to six times, and performs conditional updates to entity state values and status bytes based on specific integer comparisons.
- `compare_and_clear_register`: Compares a 16-bit value in a register to 110; if they are equal, it returns otherwise it clears a 16-bit register and returns.
- `reset_entity_status_flag`: Clears a 16-bit word at offset negative 28 from a pointer in a5 and then branches to another routine.
- `set_entity_state_flag_2`: Sets a 16-bit value of 1 at offset 28 from the address in a5 and then branches to another routine.
- `update_entity_state_and_status_2`: Checks flags in a structure to trigger a loop that resets a counter and calls itself repeatedly.
- `update_entity_status_index`: Multiplies a 16-bit value from a structure offset by 5, uses the result as an index to fetch a byte from a table, increments that byte, and stores the result back into a structure offset before calling another routine and updating a pointer in a structure.
- `update_entity_status_and_counters`: Updates a structure by calculating new counter values based on data from a table, then enters a loop that increments a value until a specific bit in a memory location is set.
- `compare_word_at_offset_2`: Restores registers from the stack, loads a pointer from an offset of sixteen into a3, and compares a 16-bit word at offset four of that pointer with a value in d0 before storing the result at offset twenty-four of the structure in a5.
- `process_entity_interaction_loop`: Iterates through a linked list of entities to find one that satisfies a range overlap check and a specific bit flag.
- `load_a3_from_pointer`: Loads a 32-bit pointer from the address held in a3 into a3 and branches to another routine.
- `update_entity_state_and_status`: Checks flags in a structure to trigger a loop that resets a counter and calls itself repeatedly.
- `update_inventory_item_state`: Iterates through a linked list of structures to compare and update item pointers, modifies specific bytes in a global structure based on state values, and performs a string search through a table to match a character value.
- `process_entity_list_and_toggle_state_2`: Iterates through a linked list of structures, checking a bitmask at offset 57 of each element and calling check_range_overlap to compare bounding boxes.
- `calculate_conditional_status`: Reads a 16-bit word from offset 22 of a structure in a5, negates it, extends it to a 32-bit signed integer, adds it to 32801, and stores the result in a 16-bit word at offset 28 of the structure in a5.
- `set_status_one`: Stores the constant value 1 as a 16-bit word at offset 28 of the structure in a5.
- `set_status_zero`: Stores the constant value 0 as a 16-bit word at offset 28 of the structure in a5.
- `iterate_entity_list_and_update_state`: Iterates through a linked list of structures, checking a bitmask at offset 57 of each element and calling check_range_overlap to compare bounds.
- `load_a2_from_pointer`: Loads a 32-bit pointer from the address held in a2 into a2 and branches to another routine.
- `process_entity_list_and_toggle_state`: Iterates through a linked list of structures, checking bit 1 of a byte at offset 57 of each element and calling another routine with a pointer, a 16-bit value at offset 8, and a length calculated from offsets 8 and 9.
- `initialize_font_structure_3`: Traverses a linked list, initializes a structure using a table of pointers, and stores a word at a fixed offset relative to a global pointer.
- `trigger_scatter_sequence`: Sets a 16-bit value in a global variable to 2, calls reset_status_and_scatter_data, clears two bytes in a global structure, and executes a loop of six calls to another routine.
- `scatter_bytes_to_buffers_6_2`: Jumps to a routine at a fixed address.
- `dispatch_via_jump_table_2`: Clears a global variable, calls another routine with three parameters, then performs a series of calls to different routines based on a byte value at offset 17620 of a structure in a6, finally returning a 32-bit value composed of a 16-bit result from one of those calls and a 16-bit value from an offset of 4 in a provided pointer.
- `insert_new_node_2`: Performs a long jump to the routine at address a fixed address.
- `update_parameter_match_state`: Compares a provided 16-bit value against a stored value in a structure; if they match and certain conditions are met, it calls another routine using data from a table and increments two counters.
- `jump_to_table_lookup_handler_2`: Jumps to another routine that retrieves a 16-bit index, a pointer, and three 16-bit words from a table using an index, then calls traverse_tree_without_flag_update.
- `append_to_list_chain_2`: Performs a long jump to the routine at address 24236.
- `jump_to_subroutine_5`: Jumps to a long address located at a specific memory location.
- `jump_to_function_pointer`: Jumps to a 32-bit address stored at a specific location.
- `jump_to_memory_copy`: Jumps to a routine that copies a block of memory.
- `normalize_and_validate_byte_pairs`: Iterates through ten blocks of memory, processing five pairs of bytes in each block by adjusting zero or negative values to specific positive constants and calling process_byte_pairs_2 if the second byte in a pair is less than the first.
- `swap_and_update_global_2`: Compares a global variable to the high byte of a parameter; if they differ, it stores the old global value in a register, updates the global variable with the high byte of the parameter, and calls another routine with the old value.
- `validate_sum_and_execute_sequence`: Iterates six times to accumulate the results of another routine into a local variable; if the final sum is zero, it calls a sequence of six routines, including one that takes a global variable as an argument, otherwise it returns.
- `jump_to_clear_flags`: Jumps to a routine that clears two 16-bit words at specific offsets within a structure.
- `jump_to_scatter_bytes_routine`: Performs a long jump to the routine that loads 16-bit values from the stack and moves a pointer from an offset of 20 on the stack into a0.
- `traverse_structure_list`: Clears a byte at offset 1 of a structure, iterates through a 27-entry table to call another routine for each non-zero 16-bit value, and then enters a loop that calls another routine for each non-zero 16-bit value at offset 152 of the structure until it reaches zero.
- `update_party_status_and_trigger_events`: Copies six bytes from a global variable to a local buffer and iterates through a table of six character structures.
- `process_combat_encounter_data`: Iterates through a set of six combat-related entries, performing bitwise checks and conditional logic based on values in a table.
- `execute_conditional_subroutine_call`: Checks a 16-bit value at an offset of 8 bytes before the address in a5; if non-zero, it pushes the constant 59 onto the stack, calls another routine, adjusts the stack pointer, and branches based on the return value, otherwise it clears d0 and branches to another location.
- `parse_and_format_dialogue_text`: Iterates through a list of data structures, checking specific byte conditions and incrementing local counters.
- `increment_offset_and_branch_2`: Increments a 16-bit word at offset 28 of a structure in a5 and branches to another routine.
- `format_text_string_with_placeholders`: Iterates through a string to identify control bytes, updating a global buffer and table, and performs arithmetic to replace specific character sequences with values from a table.
- `increment_a3_and_offset`: Increments a long value in a3 by one, increments a 16-bit value at offset negative 28 of the structure in a5 by one, and branches to another routine.
- `parse_text_control_sequence_5`: Iterates through a string to identify control bytes, updating a global buffer and table based on the character found, and calls parse_format_string, search_table_for_match_2, and increment_pointer_and_counter.
- `increment_pointer_and_counter`: Increments a longword pointer in register a3, increments a word value at a negative offset from register a5, and branches to another routine.
- `increment_offset_and_branch_3`: Increments a 16-bit value at offset negative 30 of the structure in a5 by one and branches to another routine.
- `process_spell_text_parameters_2`: Iterates through a structure of spell data, updating a buffer by comparing values from a table and performing arithmetic based on a modulo operation.
- `process_spell_text_parameters`: Iterates through a structure to process text parameters, updating a buffer with values from a table and calling format_spell_cast_text and accumulate_modulo_sum_3.
- `process_spell_cast_parameters`: Iterates through a structure of spell data, updating a buffer by comparing values from a table and performing arithmetic based on a modulo-based summation.
- `format_spell_cast_text`: Iterates through a structure to process text parameters, updating a buffer with values from a table and calling parse_format_string_3, parse_text_control_sequence_4, and accumulate_modulo_sum_3 to construct a string such as '%s casts healing on %s.'.
- `update_entity_attribute_and_loop_counters`: Adds a byte from a memory location to a byte at a calculated offset in a table, then enters a nested loop structure that increments several 16-bit counters and calls increment_entity_status_counter and other routines based on comparisons of values within a structure.
- `increment_entity_status_counter`: Tests a 16-bit word at offset 24 from a pointer in a5; if it is not zero, it increments a 16-bit word at offset 30 from that same pointer and branches to another routine.
- `update_entity_status_counters`: Iterates through a table of six entities, updating status counters and checking specific byte values at offsets within the entity data.
- `process_status_effect_logic`: Iterates through a table of status effects, updating counters and checking conditions against character data.
- `update_status_effect_counters`: Iterates through a table of status effects, incrementing or decrementing byte values at specific offsets based on comparison with threshold values and checking bitwise conditions.
- `multiply_and_decrement`: Multiplies d2 by d0, loads the address in a4 plus an offset of 6624 into a0, decrements the byte at offset 35 of the structure in a0 by 5, and branches to another routine.
- `process_status_effect_display`: Iterates through a structure to check status conditions, updates local counters, and modifies character values in a buffer based on values retrieved from a table.
- `update_character_stat_and_text_buffer`: Iterates through a list of structures, incrementing a byte value at an offset within each element and checking it against a table.
- `update_text_buffer_and_trigger_callback`: Iterates through a list of character structures, incrementing byte values and checking conditions against a table.
- `validate_table_bit_conditions_2`: Iterates six times, calling another routine to increment a value, and checks if bytes at specific offsets in a table satisfy certain conditions to find a minimum value.
- `increment_register_value`: returns the value of a register incremented by one.
- `process_graphics_and_string`: Performs a series of recursive calls, updates a global word, and executes a function pointer or copies a subregion of a buffer.
- `process_structure_loop`: Loads a 32-bit value from a global structure, calls another routine, and then enters a loop that iterates up to 30 times through a table of entries, calling another routine for each entry and performing various actions based on the returned value or specific conditions.
- `increment_d7_and_branch`: Increments a 16-bit value in d7 by one and branches to another routine.
- `initialize_font_data`: Initializes a font structure using a provided pointer.
- `search_table_and_verify_bytes`: Iterates through a table using a loop up to six times, calling another routine to check values at specific offsets and performing a loop to check for negative bytes in a sequence of thirty bytes if certain conditions are met.
- `increment_d6_and_branch_4`: Increments a 16-bit value in d6 by one and branches to another routine.
- `search_table_with_conditional_loops`: Iterates through a table using a loop, calling another routine to check values at specific offsets; it performs checks on bytes at various offsets within the table and returns 1 if a null byte is found during a loop, or 0 if a counter exceeds 5.
- `increment_register_and_branch_2`: Increments a 16-bit value in a register by one and branches to another routine.
- `increment_register_and_branch_3`: Increments a 16-bit value in register d7 and branches to another routine.
- `restore_registers_and_return_21`: Clears register d0 and restores several registers and a pointer from the stack before returning.
- `process_graphics_and_string_loop`: Performs a series of recursive calls, updates a global word, and executes a function pointer or copies a subregion of a buffer within a loop that runs up to 119 times.
- `call_delay_and_increment_d4`: Loads a pointer from a global variable into a6, calls a routine at an offset from a6, increments a 16-bit value in d4 by one, and branches to another routine.
- `initialize_font_structure`: Clears memory, initializes a font structure, and stores a word at a fixed offset relative to a global pointer.
- `populate_font_structure`: Copies several fields from a provided pointer into a global structure, including a pointer, a short, a byte with its fifth bit cleared, a byte, and a long value.
- `search_string_for_character`: Iterates through a string, decrementing a counter for each byte processed, until a matching character is found or the counter reaches zero.
- `jump_to_address_11`: Performs a long jump to the address contained in a specific memory location.
- `jump_to_conditional_handler`: Performs a long jump to another routine that handles conditional logic based on the value of a register.
- `search_table_for_match_2`: Performs a long jump to the routine at address 25032.
- `search_table_for_character_match_2`: Iterates through a table of 26 entries to find an entry where a non-zero value points to a location containing a forward slash character and a matching byte at a specific offset.
- `jump_to_address_3`: Jumps to a long address stored in a register.
- `apply_status_effect_to_entity`: Calls a routine to update a status structure, then if successful, subtracts a value from specific bytes at offsets 35, 229, and 224 within a structure calculated from a parameter, potentially resetting flags and calling another routine if certain thresholds are met.
- `recursively_traverse_pointer_tree`: Performs a double recursive traversal by calling itself twice with modified parameters, passing either a global byte or a byte retrieved from a memory offset via a pointer.
- `find_matching_index_2`: Compares a 16-bit value against the results of three separate calls to another routine and returns the index of the match or a failure constant.
- `initialize_buffer_and_update_sprites`: Initializes a memory buffer with an incrementing sequence, updates sprite attribute data from a source buffer, writes a returned value to two specific offsets in a global table, and clears a memory location at a negative offset from a pointer.
- `initialize_memory_and_call_subroutines`: Calls two routines, fills a block of 500 16-bit words with 65535, and then iterates through a table of pointers to call another routine for each entry, followed by another loop that calls another routine based on a count in a global variable.
- `iterate_and_call_subroutine`: Calls two routines and then iterates through a table of 16-byte entries, calling another routine for each entry based on a count stored on the stack, before calling another routine and returning.
- `populate_lookup_tables`: Iterates through multiple nested loops to calculate and store values in several destination tables using bit-extracted data from source tables.
- `initialize_memory_tables`: Sets a byte to 255 at one location, clears a 16-bit value, and then uses loops to zero out several tables of varying sizes before populating a table with indices and pointers returned by another routine.
- `initialize_attribute_lookup_tables`: Populates multiple destination tables by iterating through loops that extract data from a source table and perform calculations via another routine, then stores the results at various offsets in a global structure.
- `execute_complex_sequence_4`: Executes a large sequence of various routine calls that involve updating a global variable, modifying a memory offset, and passing or calculating pointers or values via registers.
- `validate_save_file_loading`: Iteratively calls various subroutines to validate resource or file loading, updating a global variable with status flags based on whether certain pointers or return values match specific error or success conditions.
- `copy_string_and_set_values`: Copies a null-terminated string from one memory location to another and then overwrites several bytes in a nearby memory range with fixed or calculated values.
- `execute_subroutine_sequence`: Calls seven different routines in succession and then returns.
- `update_dynamic_table_and_process_input`: Updates several 16-bit values in a table structure using offsets derived from a global index, then performs a series of calls to process a string buffer, including trimming trailing spaces and handling null-terminated input.
- `modify_font_and_call_routines`: Calls another routine, finds the end of a null-terminated string, modifies four 16-bit values at offsets relative to a calculated pointer, and calls three other routines.
- `increment_and_branch_loop`: Increments register d6 by one, compares it to three, and branches to another routine if they are not equal; otherwise, it calls another routine, clears the stack top, calls another routine, increments the stack pointer by two, and branches back to the start of this routine.
- `modify_font_data`: Calculates a pointer based on a value in a global variable and modifies certain memory locations relative to that pointer by adding or subtracting a constant.
- `update_sprite_attributes_2`: Executes three function calls from a pointer, copies sixteen 16-bit words from a source buffer into a global table at specific offsets, updates three values in a global structure from another structure, and calls a final function.
- `dispatch_via_stack_pointer`: Restores registers from the stack, loads a pointer from a specific offset in the stack frame into a0, and jumps to that address.
- `process_string_graphics`: Calls several subroutines using pointers from a global structure, iterates through a null-terminated string while calling another routine for each character, and returns a 16-bit value based on a table lookup.
- `jump_to_complex_logic`: Performs a long jump to execute_complex_logic_2.
- `calculate_buffer_offset_4`: Calculates a value by multiplying a byte, a byte, and a word from a pointer-referenced structure and adding a constant, otherwise it returns zero.
- `update_pixel_values_2`: Iterates through a range of 32 words, compares bit patterns at specific bitmasks between two sources, updates a destination word with modified bit values, and calls another routine if any bit differences were found.
- `accumulate_modulo_sum_3`: Iterates a specified number of times, calling another routine to get a byte, and adds the result of that byte masked with 127 modulo a value to a running sum.
- `manage_string_reference_table_2`: Resolves a string or pointer reference, copies the resulting string into a local buffer, and updates a global table with the resulting or cleared pointer.
- `find_string_end_2`: Iterates through a sequence of null-terminated strings a specified number of times and returns a pointer to the end of the last string processed.
- `call_and_set_word`: Calls another routine using a pointer from a global variable and then sets a 16-bit value at an offset of negative 12800 from a pointer in a register.
- `execute_graphics_sequence_3`: Executes three function calls using pointers retrieved from a global structure.
- `parse_text_control_sequence_4`: Iterates through a string to identify control bytes, triggering recursive calls to itself or updating a global buffer and table based on the identified sequence.
- `traverse_linked_list_and_decrement_2`: Traverses a linked list to find a pointer matching a specific value, updates that list entry with the matching value, calls another routine, and decrements a 16-bit value in a global variable.
- `execute_recursive_sequence_3`: Performs a series of five recursive calls with varying parameter increments and decrements.
- `jump_to_data_scatter_routine`: Performs a long jump to another routine that scatters bytes to buffers.
- `call_two_subroutines_with_stack_data_2`: Calls another routine twice via a jump table.
- `transform_string_data_2`: Iterates through a null-terminated string and adds 32 to each byte if a corresponding bit is set in a table.
- `process_string_input_buffer`: Calculates the length of a null-terminated string and truncates it if it exceeds a specified limit.
- `store_word_to_offset_3`: Stores a word from a register into a fixed offset relative to a global pointer and calls another routine.
- `dispatch_graphics_sequence_2`: Jumps to a calculated address derived from a table and a value at an offset, clears two 16-bit words at specific offsets relative to that calculated address, clears a 16-bit word in a global structure, and calls a function pointer located via a pointer in a global structure.
- `jump_to_recursive_logic`: Performs a long jump to another routine that executes recursive logic and string processing.
- `clear_list_elements`: Iterates through a list of 200 pointers, calls another routine if a pointer is non-zero, sets that pointer to zero, and clears a global byte.
- `update_table_with_offset_pointers`: Iterates through a table of values, replacing or updating entries using a fixed offset and a secondary table, then populates a subsequent memory range with repeated values or pointer-based increments.
- `test_bit_in_parameter`: Checks if a specific bit is set within a masked or modified version of the input parameter.
- `process_linked_list_nodes_2`: Recursively traverses a linked list, updating or creating nodes by reading values from a table and performing a bitwise XOR on a byte.
- `clear_multiple_lists`: Iterates through two different arrays of pointers, calling another routine if a pointer is non-zero and setting it to zero.
- `populate_door_attribute_tables`: Iterates through two separate sets of input parameters, each containing a loop that runs three times to read data from a source table and write calculated or direct values into a destination buffer.
- `process_pointer_range`: Calculates an end pointer and iterates through a range of pointers, calling another routine if a pointer is non-zero and setting it to zero.
- `reconstruct_coordinate_table`: Performs five sequential loops to populate a table by reading 32-bit values from a source table and calling another routine to process them.
- `update_structure_elements_with_shift_2`: Performs four sequential loops that call another routine to update a table at an offset from a pointer using values from a source table and a bit-shifted 16-bit value.
- `update_structure_elements_with_shift`: Iterates through three separate loops to call another routine using concatenated words from a table, stores the results in a global structure, and performs a second loop that calls another routine using a shifted 16-bit value.
- `update_structure_elements_parameters`: Iterates through a table to call update_structure_elements using values from specific offsets and a shifted 16-bit value, then performs a second loop to call decode_compressed_data with concatenated 16-bit values from a table, storing the results back into the first table and clearing a 16-bit word in a global variable.
- `update_structure_elements`: Loops through a table to call another routine with values from specific offsets, stores the result in a destination table, and clears a word in a global variable.
- `clear_word_at_offset`: Sets a word in a specific memory offset to zero.
- `reconstruct_coordinate_table_2`: Performs multiple sequential loops that read 32-bit values from a source table, process them via another routine, and store the results into a destination table at an offset from a pointer.
- `update_structure_elements_with_constant_2`: Performs four sequential loops that call another routine to update a table at an offset from a pointer using values from a source table and a 16-bit constant.
- `update_structure_elements_with_constant`: Iterates through three separate loops to call another routine using concatenated words from a table, stores the results in a global structure, and performs a second loop that calls another routine using a concatenated value containing a constant.
- `update_structure_element_attributes`: Iterates through two loops to update a table of values by calling update_structure_elements_alt and decode_compressed_data using parameters derived from a global structure and a secondary table.
- `update_structure_elements_alt`: Iterates through a loop to call another routine using data from a table and writes the returned value to a different offset in a destination table.
- `restore_registers_and_return_22`: Restores d2, d5, d6, d7, and a3 from the stack, unlocks a5, and returns.
- `reconstruct_coordinate_data_tables_2`: Performs five sequential loops that iterate through source tables to update a destination table by calling three different routines: one updates the first six entries, another processes four entries using a bit-shifted value, a third updates entries six through nine, a fourth processes entries six through nine with a constant, and a fifth updates entries twelve through fifteen.
- `reconstruct_coordinate_data_tables`: Performs four sequential loops that iterate through a source table to update a destination table by calling three different routines: one that performs bitwise shifts on data, one that concatenates two 16-bit values into a 32-bit word, and one that writes a constant value into a destination buffer.
- `update_structure_element_data_tables`: Performs three sequential loops that iterate through source tables to call another routine, storing the results into a destination table at an offset from a pointer in a register.
- `process_structure_element_data`: Iterates through a table to call update_structure_elements_alt2 using values from a source table and a global, then performs a second loop that calls decode_compressed_data to write results back into the structure table before clearing a 16-bit value in a global variable.
- `update_structure_elements_alt2`: Calls another routine using data from a table for four iterations, stores the results in a destination table, and clears a word in a global variable.
- `clear_word_at_offset_2`: Sets a word in a specific memory offset to zero.
- `copy_string_buffer`: Copies a string from a source address to a destination address, calls another routine, and then copies the string back from the destination to the source.
- `search_and_update_table`: Iterates through a table and calls another routine if a certain value in a table entry matches a global variable and a secondary value is non-zero.
- `clear_and_fill_memory_buffer`: Clears a 10,000 byte memory block by setting bytes to zero in a loop, then checks a bitmask against a value at offset 4728 of a structure in a4; if the condition is met, it copies 1,024 dwords from a source table into that memory block, otherwise it calls another routine.
- `parse_resource_string_parameters`: Parses a resource-formatted string or data stream to extract parameters, updating a global table and executing associated function pointers or instruction-driven logic.
- `copy_data_block_2`: Copies 1024 bytes from a source table to a destination buffer using nested loops, then checks a value from another routine to conditionally clear a 16-bit global variable and call four other routines before calling a function via a pointer in a global variable.
- `increment_and_branch_9`: Increments a 16-bit register and branches to another routine.
- `check_map_size_and_copy`: Retrieves a value from a table via a pointer, and if that value exceeds 1200, clears a 16-bit global variable and calls four other routines before calling a routine through a pointer stored in a global variable.
- `decompress_and_increment_table`: Calls a routine to decompress a data stream into a destination buffer, copies the decompressed data into a table using nested loops, calls a function via a pointer in a global variable, and then increments bytes at specific offsets in a table based on values stored in another table.
- `resolve_pointer_and_call`: Calls another routine in a loop until it returns a non-negative value, then uses the result to call another routine with split 32-bit components and other parameters.
- `set_bit_in_table`: Performs a series of routine calls and then sets a bit in a word at a specific memory offset, where the bit position is determined by the input parameter.
- `process_table_entries_2`: Runs a loop six times, calling another routine and then iterating ten times to subtract a value from a 32-bit integer at an offset of 174 within a table structure if certain conditions are met.
- `increment_and_branch_10`: Increments register d7 by 1, adds 244 to register a3, and branches to an offset within another routine.
- `restore_registers_and_return_23`: Restores d2, d6, d7, and a3 from the stack and returns.
- `accumulate_table_values`: Iterates through a loop six times, calling another routine, and if the result is non-zero, adds a value from a global variable to ten 32-bit integers in a table at specific offsets.
- `verify_routine_execution_chain`: Executes an extensive sequence of routine calls with various parameters and returns a boolean result based on whether multiple return values or state-dependent checks match specific constants.
- `validate_and_update_resource_table`: Validates a sequence of six table-based values against a calculated sum, then updates a temporary buffer with new pointers and executes associated function pointers.
- `process_table_and_string_data`: Clears a table of bytes, resets certain flags in a table, and then increments characters in a string if certain bitwise conditions are met.
- `dispatch_recursive_call_2`: Performs a recursive call to another routine using a pointer from a global variable, clears a byte at a fixed offset within a structure pointed to by a value in memory, and conditionally calls another routine based on a two-byte value.
- `execute_loop_calls_2`: Calls another routine six times in a loop.
- `initialize_structure_data_2`: Clears 27 bytes of memory starting at a calculated offset, then populates a structure with various parameters and values from a table, and calls another routine.
- `parse_attribute_data_2`: Jumps to a fixed address.
- `jump_to_object_pointer_validation`: Performs a long jump to the routine that validates and resets an object pointer.
- `jump_to_handler`: Checks if a long value at an offset in a structure is zero and calls another routine if it is, then executes a function pointer from a table.
- `jump_to_nested_pointer_update`: Jumps to another routine that recursively updates a pointer based on a condition and checks a table entry.
- `jump_to_handler_2`: Performs a long jump to an address located at a fixed offset.
- `teardown_system_resources`: Executes several function calls via pointers stored in various structures and global variables to perform cleanup operations.
- `execute_indirect_function_5`: Checks if a table entry is zero or if a specific bit is set in a table entry, then calls an indirect function through a pointer.
- `parse_format_string_3`: Stores two parameters into a structure, calls another routine, and returns a word from a specific offset in that structure.
- `update_linked_list_node_3`: Updates a linked list or table entry by modifying a pointer and associated data fields within a structure.
- `find_minimum_and_update_3`: Loops six times, calling another routine to check a condition; if met, it finds the minimum non-zero 32-bit value in a table of ten entries and updates a global value with the difference between that minimum and another value.
- `swap_word_bytes_3`: Swaps the high and low bytes of a 16-bit word and returns the result.
- `calculate_encoded_length`: Reads a word from a pointer and recursively calls parse_encoded_length_2 to determine a length based on whether the value is zero, three, or a value that results in zero after subtracting four from it.
- `validate_table_bit_conditions`: Calculates an offset into a table based on an input parameter and performs a series of bitwise and relational checks against bytes at specific offsets within that table entry.
- `jump_to_fixed_point_multiplication`: Jumps to compute_fixed_point_product_3 to calculate the 32-bit product of two 32-bit fixed-point values stored in registers.
- `jump_to_file_seek`: Jumps to another routine that performs a file pointer repositioning.
- `execute_function_pointer`: Calls another routine with a value from a global variable, then calls a function pointer located at an offset of negative 192 from that value twice.
- `update_pointer_table`: Calculates new masked values for a table and stores a corresponding pointer or index into a separate table.
- `write_parameter_offset`: Calls another routine using a parameter and the difference between a second parameter and the first parameter.
- `execute_parameter_offset`: A routine consisting of 148 instructions that calls another routine using a parameter and a calculated difference between two parameters.
- `write_parameter_sequence`: Calculates an offset by multiplying the first parameter by sixteen and calls a routine to write four 16-bit values from the parameters into a global memory table.
- `copy_data_block`: Copies a block of source words into a destination table if the source index or certain bounds checks pass.
- `map_and_modify_table_values`: Iterates through a source data structure and writes modified or flagged versions of those values into a destination table.
- `check_bit_mask`: Tests if bit 3 is clear in a byte retrieved from a table using an offset provided by a word on the stack and a base address in a4; it returns 1 if the bit is clear and 0 otherwise.
- `initialize_and_validate_lookup_table`: Initializes a block of memory to zero and populates a destination table with values retrieved from a source table via an index.
- `traverse_linked_table_nodes`: Traverses a linked list of table nodes while performing conditional lookups in a secondary data structure.
- `traverse_and_update_node_list`: Iterates through a table to find matching identifiers and stores pointers to those entries in a temporary array.
- `traverse_path_nodes`: Traverses a sequence of nodes by following pointers in a table, checking a secondary table for specific flag bits, and recursively calling itself if a value in a lookup table does not match a specific marker.
- `dispatch_table_logic`: Reads a value from a table and executes different sequences of routine calls or conditional logic based on that value.
- `traverse_linked_node_structure`: Iterates through a list of structure pointers, checking a target ID and performing recursive calls to another routine based on conditional pointer arithmetic and table lookups.
- `execute_nested_loops`: Executes another routine a fixed number of times within a nested loop structure if a certain condition is met.
- `traverse_node_list`: Iterates through a range of bytes and, if certain conditions or pointer comparisons are met, calls other routines using pointers or values retrieved from a table.
- `sync_global_value`: Checks if a 16-bit value at one location matches a 16-bit value at another; if they differ, it calls another routine three times and updates the first value to match the second.
- `execute_conditional_function_chain`: Executes a sequence of function pointers retrieved from a structure, conditionally calls another routine based on a global variable, and loops until a value at an offset from a pointer becomes zero.
- `return_incremented_value`: Pushes d7 onto the stack, sets d7 to 1, restores the original value of d7 from the stack, and returns.
- `jump_to_table_handler`: Performs a long jump to a location determined by a table lookup handler.
- `write_parameter_sequence_to_table`: Calculates an offset by multiplying the first parameter by sixteen and writes four 16-bit values from the parameters into a global memory table at that offset.
- `jump_to_dispatch_target`: Performs an indirect jump to another routine.
- `update_graphics_parameters_5`: Calls another routine, calculates a value based on a return value and a comparison, and stores a short value and a calculated result into a global structure.
- `jump_to_indirect_address_3`: Performs a long jump to an address retrieved from a table via a long pointer.
- `jump_to_subroutine_6`: Transfers execution to another routine via a long jump.
- `update_sprite_tile_attributes`: Iterates through a list of descriptors to update a buffer by copying or masking attribute data.
- `calculate_table_index`: Calculates an index based on the absolute difference of two values and returns a byte from a table at that index.
- `copy_memory_block_2`: Copies a block of memory using a loop and then calls a library function while checking a return value against a constant.
- `calculate_sorted_offset`: Calculates the absolute difference between two values, sorts them, and returns a value containing the smaller difference and a sum.
- `store_word_pairs`: Stores five 16-bit values into a structure at a specific offset by combining pairs of bytes from a source pointer, then calls another routine.
- `parse_resource_format_strings`: Iterates through a table of resource format string pointers to parse and deallocate string or data references.
- `decode_compressed_data`: Decodes a stream of data by reading from a source, applying bitwise shifts, and writing the result to a destination buffer.
- `check_available_memory_2`: Uses a library call to check for available memory and returns a boolean indicating if the result exceeds a certain threshold.
- `allocate_and_initialize_structure`: Allocates memory via a library call, sets a byte at offset 8 of the allocated block to 4, and calls setup_pointer_chain_2 with the address at offset 20 of that block.
- `free_memory_if_nonzero_2`: Checks if a passed pointer is not null, and if so, calls a library function to free memory using a pointer from a global variable.
- `free_two_pointers_2`: Loads a value from a global variable and calls a function pointer located at an offset of 210 from that value, then repeats this process using the same value as the base for a second function call at the same offset.
- `free_pointer_list_2`: Iterates through a list of four pointers at an offset of 4 from a provided address and calls another routine for each non-null pointer, then calls a function pointer stored at an offset of 210 from a global variable.
- `conditionally_free_memory_2`: Checks if a passed pointer is not null and if the value at offset 24 within that pointer is non-zero, then calls a library function to free memory.
- `free_memory_via_global_pointer`: Checks if a passed pointer is non-null; if it is not, it retrieves a function address from a global variable and calls it with the pointer in register a1, otherwise it returns zero.
- `free_memory_if_not_null`: Checks if the provided pointer in register a3 is non-null; if it is not null, it calls a function located at an offset of 210 from a global pointer using the address in a6, then returns the result of that call.
- `traverse_and_free_recursive_2`: Recursively traverses a pointer-based structure by checking a bitmask in a byte at offset 14, then decrements a value at offset 10 and calls a deallocation routine via a function pointer from a global variable if that value is non-zero, finally calling the deallocation routine again before returning.
- `traverse_and_free_list_3`: Traverses a linked list and conditionally deallocates memory or clears list flags or table offsets based on a certain condition.
- `decrement_loop_with_delay_2`: Calls another routine, then enters a loop that decrements a 16-bit value from a parameter while executing four consecutive delay loops, and finally calls another routine after the loop finishes.
- `initialize_audio_buffer_and_metadata`: Copies a string into a global buffer, allocates memory for a new structure, and initializes its members or pointers.
- `traverse_and_call_dual_lists`: Iterates through two different linked lists by reading pointers from offsets 1696 and 1700 of a structure, calls another routine for each node, and updates the list pointers with the value stored in the node.
- `clear_linked_lists`: Checks if a value in a global variable is non-zero, calls a function pointer, and then clears several pointers and values within a structure after calling other routines.
- `initialize_structure_list`: Iterates through a list to copy a dword and increment a value in a linked list or table if a certain condition is met, then sets a bitset flag.
- `parse_resource_header_stream`: Iterates through a stream of resource-defined data, identifying specific type markers to allocate memory, copy string or byte data, and update resource metadata or linked lists.
- `parse_resource_buffer_2`: Iterates through a buffer of tagged data, allocating memory to store content based on identified type markers.
- `allocate_and_copy_buffer`: Allocates memory using a library call and copies a specific amount of data from a source pointer to the new buffer.
- `parse_tagged_string_buffer`: Parses a buffer containing type-length-value or type-length-string tagged data, allocating or updating nodes in a linked list and copying string contents or metadata.
- `remove_node_from_list`: Traverses a linked list and removes any node that matches a certain string comparison, while updating head pointers or orphaning nodes.
- `insert_node_into_list`: Inserts a new node into a linked list or updates an existing list, potentially clearing certain flags and updating a table or structure if a bitset or flag is present.
- `calculate_string_length_recursive_2`: Iterates through a null-terminated string to find its length and then calls itself recursively using the calculated length as a new pointer.
- `parse_resource_stream_3`: Iterates through a tagged resource stream to allocate memory and populate a table with string or data references.
- `modify_bitmask_and_recurse_2`: Calls another routine and, if the return value is non-zero, clears bit 5 in a byte at offset 14 and calls another routine recursively.
- `clear_list_flags`: Disables interrupts, then traverses a linked list to clear a specific bit in a byte, and clears certain offsets in a table or structure.
- `trigger_system_event_2`: Calls a library function to cause a system event and returns zero.
- `update_pointer_with_offset_2`: Adds a 32-bit value located at offset 4 of a structure in a1 to the value at the base address of that structure, then calls three other routines and returns.
- `reorganize_linked_list`: Clears a pointer in a structure at offset 372 of a6, then enters a loop that removes nodes from a linked list by clearing bit 1 of a byte at offset 13 of each node, finds a node with the minimum value at offset 24 of a structure in a5, and updates the list and node status.
- `transfer_pointer_and_branch`: Copies a longword from offset 0 of a structure in a4 to offset 0 of the address in d6, updates a4 with that same longword, and branches to another routine.
- `find_minimum_node`: Iterates through a list to find a node with the minimum value at a specific offset, updates that node's status, and performs list manipulation.
- `restore_registers_and_return_24`: Restores d4 through d7 and a3 through a6 from the stack and returns.
- `test_structure_bit_and_call`: Loads a pointer from a global variable into a4, clears d7, and checks bit 0 at offset 14 of the structure in a4; if the bit is set, it calls another routine, otherwise it branches to another location.
- `test_and_call_routine`: Tests bit 1 of a byte at offset 14 of a structure in a4, calls another routine if the bit is set, and otherwise branches to another location.
- `traverse_linked_list_4`: Iterates through a linked list, checking specific bits at offset 14 of each element to determine whether to call another routine or update pointers in a global variable and a structure in a6.
- `update_linked_list_pointers`: Moves the value from a4 to offset 0 of the address in d7, updates a4 with the value at offset 0 of the address in a4, updates a global variable at offset 1684 of a5 with a new address, and branches to another routine.
- `update_pointer_and_branch`: Moves the value from a4 to d7, updates a4 with the value at the address pointed to by a4, and branches to another routine.
- `restore_registers_and_return_25`: Restores d7 and a4 from the stack and returns.
- `process_table_entry`: Loads a pointer from an offset of a structure in a4, clears d7, and enters a loop that calls another routine using a table entry at an offset determined by d7, incrementing d7 until it reaches zero.
- `clear_bit_conditionally`: Checks a byte in a global structure and, if it is zero, clears a specific bit in another byte.
- `clear_bit_in_structure`: Saves registers to the stack, compares a long value in a register to a value in a table, and if a condition is met, clears bit 7 of a byte at an offset of a structure in a3.
- `process_data_stream_buffer`: Iterates through a data buffer by reading bytes and updating a pointer, performing conditional arithmetic on a value in a register using a table lookup, or modifying a structure's field based on specific byte markers.
- `compare_and_branch_value`: Compares a byte in d6 to 129, and if equal, shifts d5 left by 2 bits, loads a pointer from a table using the result as an index, and moves that pointer to an offset in a2.
- `compare_and_branch_3`: Compares a byte in d6 to 136, branches to another location if they are not equal, otherwise stores a byte from d5 into a structure at offset 8 of a2 and branches to another routine.
- `compare_and_branch_2`: Compares the byte in d6 to 136 and branches if they are not equal, otherwise it performs a relative branch to another location.
- `check_byte_and_clear_bit`: Compares the byte in d6 to 255; if it is not equal, it branches to another routine, otherwise it clears bit 7 of the byte at offset 104 of a3 and branches to another routine.
- `branch_to_middle_of_routine`: Performs a short relative branch into the middle of another routine.
- `restore_registers_and_return_26`: Restores d4 through d6 from the stack and returns.
- `update_structure_pointer`: Saves registers, loads a pointer from offset 4 of a2, checks bit 1 of a byte at offset 14 of that pointer, and if not set, moves a value from offset 6 of that pointer to a global variable.
- `populate_fixed_point_structure`: Calculates a fixed-point value using a division-based approach or a bit-shifting loop and writes the resulting integer, fractional, and scaling components into a destination structure.
- `calculate_address_offsets`: Enters a loop that decrements d2 and performs arithmetic on d3, d4, and d5 based on comparisons of d1 and d2, and branches back to the start of the routine if d2 is greater than 1.
- `write_offset_addresses`: Calculates two addresses by adding d5 and d3 to a value from offset 24 of a0, stores these addresses at offsets 4 and 8 of a1, and then stores d3 at offset 12 and d4 at offset 16 of a1 before branching to another routine.
- `populate_structure_from_source`: Calculates a quotient using a constant divided by a 16-bit value from a source structure and writes it to two 16-bit fields at offsets 32 and 34 of a destination structure.
- `update_structure_values`: Stores a dword and two zeroed bytes into a structure at a specific offset, calculates a value based on two other sources, and writes that value to two short offsets.
- `store_values_to_struct`: Stores a dword and two zeroed bytes into a structure at a specific offset.
- `update_status_byte`: Checks bit 1 of the byte at offset 52 of a structure in a0; if set, it branches to another routine, otherwise it sets that byte to 7, copies a longword from offset 596 of a5 to offset 0 of a0, updates the longword at offset 596 of a5 with a0, and branches to another routine.
- `decrement_and_loop`: Adds 36 to a0, decrements d0, and loops back to the start of the routine if d0 is not zero, otherwise it returns.
- `clear_and_branch_status`: Clears bit 4 of a byte at offset 14 in a structure in a4, calls another routine, and if the result is zero, moves a pointer from d0 to a2, checks bit 1 of a byte at offset 14 of the structure in a4, and if that bit is not set, clears a longword at offset 28 and a longword at offset 40 of the structure in a2 before branching.
- `update_pointer_and_clear`: Loads a pointer from offset 6 of a structure in a4 into a4, then loads a pointer from offset 0 of the new a4 into a0, and if a0 is not zero, clears a longword at offset 28 of a2; otherwise it branches to another location.
- `clear_structure_fields`: Stores a0 at offset 28 of a2, clears the byte at offset 38 and the word at offset 36 of a2, then checks if the longword at offset 4 of a4 is non-zero; if it is, it branches to another routine, otherwise it clears the longword at offset 40 of a2 and branches to another routine.
- `calculate_and_store_values_2`: Writes several values to a structure, including a calculated value based on a division involving multiple sources, and sets two short offsets.
- `calculate_and_store_values`: Writes several values to a structure, including a calculated value based on a division involving multiple sources, and sets two short offsets to a value derived from a bit shift.
- `set_uniform_values`: Calculates a value by shifting a register and writes that value to two adjacent locations in a structure.
- `decompress_entity_data_streams`: Iterates through a list of structures to perform variable-length decoding on two separate data streams.
- `decode_compressed_data_streams`: Iterates through two separate data streams within a structure, using marker bytes to perform variable-length decoding, pointer updates, and arithmetic adjustments to offsets and counters stored at various offsets within the structure.
- `load_and_store_word`: Loads two bytes from a buffer into d1, shifts the first byte left by 8 bits, and stores the resulting word at an offset of 34 from a4 before branching to another routine.
- `decode_compressed_stream`: Checks if a byte in a0 is the end marker, otherwise it reads a byte from a0, increments it, shifts it left by one, and subtracts this value from a0 before branching back to the start.
- `parse_compressed_data_stream`: Iterates through a data stream to decode variable-length or compressed values into a structure, updating an offset and a 16-bit field based on byte markers.
- `decrement_and_adjust_pointer`: Compares a byte in d1 to a constant, and if not equal, clears d1, loads a byte from a0, increments it, shifts it left by 1, subtracts the result from a0, and loops back to the start of the routine.
- `calculate_scaled_value`: Loads a byte from a0 and increments a0, then if the byte is not negative, multiplies it by a word from offset 44 of a4, shifts the result right by 6 bits, and clamps the value to 16384.
- `update_offset_value`: Subtracts 192 from a word in d0, shifts it left by 8 bits, adds a word from offset 46 of a structure in a4 to d0, and if d1 is not zero, stores the resulting word into offset 46 of the structure in a4 before branching.
- `update_status_byte_2`: Stores the byte from d1 into offset 50 of a structure in a4, and if d1 is 1, stores the word from d0 into offset 46 of a4 and clears a word at offset 48 of a4; otherwise it branches to another location.
- `update_entity_state_from_data`: Performs a division and modulo operation on a value to update a 16-bit field in a structure, then checks a bitmask at offset 52 to conditionally call several subroutines that populate a new structure with data from the current entity's fields.
- `process_entity_data_list`: Iterates through a list of structures, performing bitwise operations on a byte at offset 52 and calling several subroutines to populate a new structure with data from offsets 4, 8, 12, 16, 24, 28, 30, and 32 of the current structure.
- `invoke_nested_routines_3`: Executes a sequence of calls to another routine, passing a global variable and two constant values, followed by three additional calls to that same routine with no arguments.
- `insert_new_node`: Traverses a linked list and inserts a new pointer or value into a list or table if certain conditions or empty pointers are met.
- `initialize_clipped_bitmap_2`: Checks if a specified rectangular area or pointer-based structure falls within certain bounds and initializes a bitmap.
- `validate_and_set_bitmaps`: Checks if a source and destination rectangle overlap or exceed certain bounds, then calculates a memory range to populate a table with a bitmap or area address.
- `check_byte_mismatch`: Compares the lower bits of two bytes and calls a routine if they do not match.
- `scatter_bytes_to_buffers_6`: Loads three 16-bit values and a pointer from a stack frame, calls another routine, and returns.
- `copy_block_with_offsets`: Copies bytes from a source pointer to five different destination pointers separated by large offsets using nested loops.
- `scatter_bytes_to_buffers_7`: Loads four 16-bit values and a source pointer from the stack, calls a routine to copy a block of memory to five destination pointers at fixed offsets, and returns.
- `copy_block_fixed_offset`: Copies a block of memory from a source pointer to five different destination pointers separated by large offsets.
- `execute_rect_copy_callback`: Restores a set of 16-bit values from the stack into registers, calls another routine, and then restores a set of 32-bit registers and an address register before returning.
- `copy_subregion_with_bounds`: Calculates a subregion within a larger buffer and copies bytes from a source to a destination using large offsets.
- `clear_font_structure_flags`: Clears two 16-bit words at specific offsets within a structure pointed to by a6 and returns.
- `clear_large_buffer`: Fills a large memory area with zeros starting from a pointer retrieved from a table.
- `clear_conditional_pointer`: Checks if a bit is set in a value at an offset from a4, potentially calling another routine, and then sets that value to zero.
- `retrieve_table_pointer_2`: Retrieves a pointer from a table using an index and calls another routine.
- `traverse_with_flag_update`: Recursively traverses a tree or linked structure while setting and clearing a bit in a flag or status byte.
- `copy_fixed_bytes_2`: Copies sixty-three bytes one at a time from one pointer to another.
- `execute_conditional_function_3`: Calls another routine and, if the return value is non-zero, uses a value from a structure in a4 to call a function pointer and then optionally calls a second function pointer.
- `execute_delay_if_nonzero`: Checks if a parameter is non-zero and calls a routine stored at an offset from a global pointer.
- `validate_and_reset_object_pointer_3`: Calls another routine and checks if the return value is zero; if it is non-zero, it uses that value as an index into a table to retrieve a pointer from offset 26 of the calculated address, calls another routine with that pointer, and then sets that table entry to zero.
- `execute_conditional_cleanup`: Checks if a value at a specific offset from a4 is non-zero, and if so, calls the function pointed to by that value.
- `process_string_and_call_function`: Iterates through a null-terminated string and calls another routine for each character.
- `execute_graphics_sequence`: Retrieves a pointer from a global variable and calls three different routines using offsets from that pointer.
- `fetch_table_entry_data_2`: Retrieves a 16-bit index, a pointer, and three 16-bit words from a table using an index, then calls traverse_tree_without_flag_update.
- `traverse_tree_without_flag_update`: Recursively traverses a structure using a value retrieved from a table, without modifying any flags in the input parameter.
- `update_pixel_values`: Iterates through a memory range, compares bit certain patterns, and updates a destination pointer with modified bit values before calling a routine if a change is detected.
- `execute_conditional_string_logic`: Executes a sequence of routines including a string search and a conditional function call, returning a value that determines if a certain pointer or value should be cleared.
- `execute_graphics_function_sequence`: Loads a pointer from a global structure and executes a sequence of three function pointers located at fixed offsets from that pointer.
- `execute_conditional_function_2`: Retrieves a pointer from a global structure and calls a function, then calls a second function if the first return value is non-zero.
- `find_string_index_3`: Iterates through a table of pointers stored in a global variable, calling a case-insensitive string comparison routine for each entry.
- `traverse_linked_list_and_decrement`: Traverses a linked list to find a matching pointer, updates a pointer in the list, calls a function, and decrements a global counter.
- `process_list_and_string`: Iterates through a null-terminated string to call a routine for each character and traverses a linked list to call another routine while decrementing a global counter.
- `extract_bits_from_table`: Calculates an address using a table and offsets, then iterates a fixed number of times to extract specific bits from memory into a register.
- `search_string_table_4`: Iterates through a table of pointers and performs a case-insensitive string comparison against a provided parameter.
- `sort_filtered_attribute_data`: Clears ten 4-byte values at an offset from a pointer in a6, then iterates through a list of structures to recursively process each entry and count how many have bit 7 set in a byte at offset 18.
- `setup_jump_parameters`: Saves registers d0 through d1 and a0, a2 through a5 to the stack, pushes the value in a0 to the stack, loads three constant values into a4, a3, and a2, and then performs a short branch to another routine.
- `process_event_linked_list_2`: Iterates through a linked list of structures to perform bitwise operations on a bitmask in a memory buffer and updates various status flags.
- `process_linked_list_events`: Iterates through a linked list of structures, performing bitwise operations on a bitmask in a memory buffer and updating various status flags and a circular buffer based on specific values found within the list nodes.
- `set_status_bits_5`: Compares a byte at offset 4 of a0 to 2, and if equal, checks if a word at offset 6 of a0 is 104, then sets bit 1 of a byte at offset 8 of a1 and bits 0 of two bytes at specific offsets from a4.
- `branch_to_following_code`: Performs a short branch to the instruction located at the specified offset.
- `set_status_bits_3`: Compares a word in d0 to 105, and if it is equal, sets bit 2 of a byte at offset 8 of a structure in a1 and sets bit 1 of two different bytes at negative offsets from a4, then moves 113 into d0 before branching.
- `branch_to_offset_alt`: Executes a short branch to a specific address.
- `modify_bits_and_branch`: Compares a word in d0 to decimal 232; if not equal, it branches, otherwise it clears bit 1 of a byte at offset 8 of a1, clears bit 0 of a byte at offset negative 12838 of a4, sets bit 0 of a byte at offset negative 12837 of a4, and branches to another routine.
- `check_and_clear_bits`: Compares a word in d0 to decimal 233; if equal, it clears bit 1 of a byte at an offset of a1 plus negative 12838 and bit 0 of a byte at an offset of a4 plus negative 12837, then sets bit 1 of a byte at an offset of a4 plus negative 12836; otherwise it branches to another routine.
- `process_event_linked_list`: Iterates through a linked list of event structures, performing bitwise operations on a bitmask in a memory buffer based on event codes and updating various status flags and a secondary buffer.
- `call_add_int_server_3`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `set_negative_sign_char`: Checks if an integer is negative and stores an ASCII minus sign in a local buffer if so, then recursively processes the buffer or a pointer offset.
- `scale_shift_register_value`: Calls update_bit_shifter to retrieve a value, calculates a divisor by subtracting two values from a structure and adding one, then returns a 32-bit result containing the quotient in the upper 16 bits and the remainder in the lower 16 bits.
- `subtract_two_short_values`: Subtracts a 16-bit value at a specific offset in a structure from another 16-bit value at a different offset in the same structure and returns the result.
- `iterate_and_resolve_glyph`: Iterates through a loop that calls another routine and updates a value using a glyph resolution routine.
- `process_conditional_d0`: Saves d6 and d7 to the stack, then if d7 is not zero, calls another routine; otherwise, it moves d0 to d6, and if d6 is not zero, branches, otherwise it sets d0 to zero and branches to another routine.
- `process_glyph_sequence`: Iterates through a loop calling another routine, increments and wraps a 16-bit value at an offset of a4 plus negative 12678 by modulo 16 if the returned value's low byte is zero, and returns the result when the low byte is non-zero.
- `restore_registers_and_return_27`: Returns the value in d7 and restores d6 and d7 from the stack.
- `traverse_linked_list_with_updates`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call.
- `call_nested_routines`: Pushes d7 to the stack, then if d7 is not zero, calls another routine; otherwise, it pushes d0 to the stack, calls another routine, adjusts the stack pointer by 4, moves d0 to d7, and branches to another routine.
- `call_and_return_d0`: Calls another routine and returns the value from register d0.
- `evaluate_and_clear_d0_result`: Calls another routine and, if the result in d0 is non-zero, moves that value into d6; otherwise, it clears d0.
- `push_and_branch_to_offset`: Pushes the contents of register d6 onto the stack, calls another routine, adjusts the stack pointer by four, masks the lower eight bits of register d0 into register d7, and branches to another routine.
- `restore_registers_and_return_28`: Moves the contents of a register into the return value register and restores the previously saved registers from the stack before returning.
- `resolve_character_glyph_index`: Calculates a final glyph or sprite index by looking up a base value in a table and applying bitmask-based offsets.
- `resolve_table_index_with_offsets`: Looks up a byte from a table using a masked input, applies several conditional offsets based on certain bits, and returns a calculated index.
- `update_state_and_retrieve_value`: Checks a global condition, calls a function, increments and wraps a global counter, and extracts or updates values from a table-based pointer.
- `get_table_value_2`: Checks if two consecutive halfwords in a global memory area are unequal and, if so, loads a longword from a table using an offset.
- `test_bit_in_bitset`: Checks if a specific bit is set within a bitset stored in a global memory area and returns all bits set if true, otherwise it returns zero.
- `search_and_compress_string`: Performs a substring search within a source buffer and writes a compressed representation of the match to a destination buffer.
- `compress_lzss_data_stream`: Performs LZSS compression on a source buffer by searching for the longest matching sequence within a sliding window.
- `compress_data_stream_via_lzss`: Performs a sliding window compression on a source buffer by searching for matching sequences in a previous data segment.
- `write_packed_data_structure`: Calculates a byte value by adding a byte from a register to a constant of 192, then stores this byte, followed by a 16-bit word and an 8-bit value, into a memory buffer pointed to by a register.
- `serialize_data_structure`: Writes a byte with value 255, followed by a 16-bit value derived from adding 3 to a register, and a 32-bit value split into two 16-bit words derived from swapping and shifting another register, then branches to another routine.
- `compress_data_stream`: Performs a variation of run-length encoding on a source buffer by comparing consecutive bytes and writing either literal byte sequences or length-prefixed run sequences into a destination buffer.
- `loop_and_write_byte`: Compares a6 to zero, and if not zero, writes the byte 128 to the address in a3 and increments a3, then checks a byte at the address in a6 to decide whether to continue the loop.
- `compress_lzss_data_stream_2`: Performs LZSS compression on a source buffer by searching for the longest matching sequence within a sliding window and writes the resulting compressed stream to a destination buffer.
- `set_bit_and_offset`: Writes a constant bit pattern to a memory location and returns a pointer offset by a certain amount.
- `decompress_compressed_stream_2`: Unpacks a custom-format compressed data stream from a source buffer into a destination buffer using a variable-length instruction-based decoding scheme.
- `initialize_font_cache`: Checks if a global value is zero and, if so, calls a library function to populate it.
- `execute_recursive_function_calls`: Performs a sequence of recursive calls or calls to other routines using values loaded from a table.
- `scatter_bytes_to_buffers_8`: Loads four 16-bit values from the stack, moves a pointer from an offset of 20 on the stack into a0, and calls copy_data_blocks before returning.
- `copy_data_blocks`: Calculates a source pointer using a base address and an offset, then performs a nested loop to copy blocks of data, where each copy operation transfers multiple words or bytes to a destination via a pointer-based loop.
- `update_graphics_parameters`: Iterates through a string to update a value from a table and calls graphics-related functions.
- `process_recursive_logic`: Performs a recursive check against a global value and potentially updates a table entry or executes an indirect function.
- `call_subroutine_via_stack`: Restores registers from the stack, loads a pointer from an offset of the stack frame into a0, and calls another routine.
- `allocate_and_initialize_buffer`: Allocates a buffer, populates it with an incrementing sequence of values, and calls functions to insert a node and process a bitstream.
- `traverse_and_clear_linked_list`: Iterates through a linked list starting from a global pointer and executes another routine on a data member of each node.
- `reset_global_values`: Sets two global variables to zero and calls two library functions using a pointer retrieved from a global address.
- `clear_font_pointer`: Retrieves a pointer from a global address and calls two library functions, the second of which is a pointer clearing routine.
- `update_sprite_attributes`: Executes several library calls, then runs a loop that copies attribute data from a source buffer into a global table.
- `call_sprite_move`: Calls a library function using a pointer retrieved from a global address.
- `execute_indirect_handler`: Loads a function pointer from a table located via an offset from a global pointer and executes it.
- `check_bounds_overlap`: Checks if a provided range or coordinate falls within a certain area defined by values stored in a global structure, returning one if inside and zero otherwise.
- `save_and_restore_value`: Saves a word from a memory location into a register, sets that memory location to zero, calls another routine, and then restores the original value.
- `update_ui_text_and_graphics`: Updates a global variable with a specific value, modifies table entries based on a calculated offset, and performs conditional text formatting by incrementing a character and appending a space.
- `process_disk_insertion_and_text_parsing`: Checks a character value at an offset in a structure, modifies a buffer with a character incremented by one and a space, and enters a loop that calls a function via a pointer in a table to update a value in a structure.
- `process_complex_data_structure_3`: Calculates an offset using a value from a structure, saves data from that offset, and updates two words at that location with the constants 31 and 19.
- `store_and_branch`: Stores the value negative 255 in a location at an offset of minus 40 from a pointer in a5, then branches to another routine.
- `update_global_pointer_and_value`: Updates a global pointer or a value at a fixed offset relative to a global pointer via a recursive or conditional call.
- `search_string_in_table_2`: Iterates through a table of pointers to find a null-terminated string that matches a provided string, then returns an index from the table.
- `calculate_offset_and_load`: Calculates an offset by multiplying the value in d7 by 20, loads a word from that offset relative to a zeroed address into d0, and branches to another routine.
- `increment_and_branch_11`: Increments the 16-bit value in d7 and branches to another routine.
- `return_negative_one_3`: Returns a value of negative one.
- `update_table_values_3`: Calculates an offset into a table based on a global value and updates four different entries in that table using a provided parameter or its multiples.
- `resolve_string_and_update_table_2`: Resolves a string or pointer reference, copies the resulting string into a local buffer, and updates a global table with the resulting or cleared pointer.
- `search_and_update_table_index`: Checks a table of values for a specific index and updates a pointer in a table if a condition is met.
- `find_min_index_by_byte`: Iterates through a table of structures to find the index of the entry with the smallest byte value at offset 18, provided that a 4-byte integer at offset 4 is greater than or equal to a value in a register and a 1-byte value at offset 18 is less than or equal to a value in a register.
- `teardown_display_and_resources`: Executes a sequence of cleanup or reset routines, including closing certain system resources, clearing a linked list, and resetting specific memory fields or display context parameters.
- `initialize_display_resource_nodes`: Allocates and initializes a list of nodes, setting up display-related properties and managing pointer-based buffers.
- `toggle_bit_planes`: Calls another routine and then returns.
- `set_bitfield_from_word`: Calculates a destination pointer based on input values and a base address, then iterates through a loop to set or clear bits in memory at 8000-byte intervals.
- `clear_and_set_bits`: Clears a bit in a byte at a certain offset and then sets a bit in a subsequent byte based on a bit-shifting loop.
- `call_avail_mem_3`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `update_bit_shifter`: Performs bitwise shifts and logical operations on bytes stored in a global memory area to propagate or clear certain bits.
- `check_and_execute_handler`: Checks if a pointer in a table is zero and calls another routine if so.
- `apply_bitmask_to_buffer_2`: Iterates through a buffer and uses bit-testing and bit-setting instructions to modify certain bits based on a lookup table.
- `call_rem_int_server_3`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `compare_string_against_list`: Iterates through a table of pointers to compare a target string against each entry.
- `swap_word_bytes`: Takes a 16-bit value and returns a new value where the high and low bytes have been swapped.
- `rotate_word_bits`: Performs a complex bit rotation and byte swapping on a 32-bit or 16-bit input value.
- `fetch_table_entry_and_call`: Retrieves a 16-bit index and a 32-bit pointer from a table using an offset from the stack, loads three 16-bit values into registers, and calls another routine.
- `set_attribute_flag`: Sets a bit in a byte at a specific offset and calls another routine using a value retrieved from a table-based pointer.
- `fetch_table_entry_and_call_2`: Retrieves a 16-bit index, a 32-bit pointer, and three 16-bit values from a table using an offset from the stack, then calls another routine with these values in d0, a0, and d1 through d3.
- `get_table_value_and_branch`: Retrieves a pointer or value from a table-based address and calls another routine if a certain bit is not set.
- `seek_file_position`: Performs a file pointer repositioning using a library call.
- `scatter_bytes_to_buffers_9`: Loads several 16-bit values from the stack into registers, moves a pointer from an offset of 20 on the stack into a0, calls another routine, and returns.
- `copy_large_block`: Copies large contiguous blocks of data from a source table to a destination buffer using a fixed stride and offset.
- `setup_font_data`: Copies certain fields from a provided pointer to a global structure and calls a routine repeatedly while iterating through a table of pointers.
- `copy_and_execute_functions`: Copies sixty-four bytes from a source location to a destination and executes a function pointer found at a certain offset.
- `shuffle_table_elements`: Iterates through a table and performs swaps of bytes between a source table and the target table based on calculated indices.
- `calculate_buffer_offset`: calculates a value by multiplying a byte, a byte, and a word from a pointer-referenced structure and adding a constant.
- `calculate_scaled_product_3`: Loads two 16-bit values from a stack offset, multiplies them together, masks the result to 16 bits, and then multiplies that value by five before returning.
- `update_circular_index`: Reads an index from a global variable, uses it to write a register value into a table, and increments the index modulo sixteen, updating the global variable if the new value differs from the current one.
- `iterate_string_and_set_graphics_params`: Iterates through a string, updating a value based on a table, and calls several graphics-related library functions.
- `copy_string_with_conditional_logic_2`: Parses a source buffer to find a length, then copies a certain number of bytes from the source to a destination buffer, handling different length or sign-based logic.
- `test_and_execute_function`: Checks if a 32-bit value read from a pointer is non-zero, and if so, calls a function pointer located via an offset from a global base address.
- `parse_encoded_length_2`: Checks a value at a pointer and recursively calls itself to calculate a length or skip certain bytes.
- `process_input_and_update_counters`: Decrements and checks counters at various offsets in a structure, increments a global counter, and updates a status byte based on a comparison of hardware register values.
- `update_input_status_flags_2`: Sets bit 1 of a value at a negative offset from a pointer, then compares two 16-bit values shifted right by 8 bits to set bit 0 of the same value if the first is greater than the second.
- `update_input_status_flags`: Sets a bit in a peripheral status byte, then checks if a value from a pointer-based source matches a stored value to update a secondary bit in that same byte.
- `process_input_and_update_table_3`: Modifies a hardware register based on certain bit patterns and updates a table or a global index using a loop.
- `call_wait_b_o_v_p`: Library stub: loads its arguments and the graphics library base, calls WaitBOVP and returns its result to the caller.
- `parse_text_control_sequence`: Iterates through a string to identify control bytes, which trigger recursive calls to itself or update a global buffer and table based on the control character encountered.
- `invoke_subroutines_on_stack_2`: Uses a local stack buffer to call another routine twice via a jump table.
- `call_two_subroutines_with_stack_data`: Pushes d7 onto the stack, loads a word from an offset of 8 relative to the stack pointer into d7, pushes that word onto the stack, calls two subroutines, and restores d7 from the stack before returning.
- `store_word_to_offset_2`: stores a word from a register into a fixed offset relative to a global pointer.
- `execute_graphics_sequence_4`: Executes a sequence of function pointers and clears certain memory locations, while waiting for a graphics blit to complete.
- `store_parameter_sequence`: Calculates an offset based on the first parameter and writes a sequence of four 16-bit values to a global memory table.
- `recursively_process_text_string`: Recursively traverses a string to calculate or update offsets and lengths based on a table or global structure, and may trigger a graphics callback if certain boundary or pointer conditions are met.
- `check_bit_and_execute`: Checks if a table entry is zero or if a specific bit is set in a table entry, then executes an indirect function.
- `call_indirect_via_stack_offset_2`: Loads a function pointer from an offset of 348 subtracted from a value stored on the stack and executes it.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `resolve_nested_pointer_reference`: Recursively traverses a pointer-based structure, checking a bitmask flag and a global status flag, returning a final pointer or a failure constant.
- `parse_format_specifier`: Parses a format string for a printf-like operation, handling flags, width, precision, and length modifiers, then retrieves a variable argument from a pointer and converts it to a string buffer using helper routines.
- `parse_format_string`: The routine walks a format string character by character.
- `handle_delete_file_error`: Attempts to delete a file and, if the deletion fails, retrieves an error or message, stores it in a global variable, and sets a flag and a return value.
- `parse_string_to_token_array`: Parses a source string into an array of pointers to tokens, handling quoted strings and expanding escape-like character sequences.
- `initialize_global_pointers`: Calls a routine to update several global memory locations with returned pointer values and assigns a routine address to a global pointer.
- `traverse_function_list`: The routine iterates through a list of function pointers, recursively calling another routine if a certain bit is set in a retrieved value, and finally calls a specific function pointer.
- `dispatch_cleanup_callback`: Checks if a function pointer at a specific offset within a structure is non-zero and calls it, then performs a series of conditional calls to other functions based on values stored in a structure and a global variable.
- `transform_string_case_2`: Iterates through a null-terminated string and, if a certain bit is set in a lookup table based on a character, subtracts thirty-two from that character.
- `copy_string_with_limit_4`: Calculates the length of a null-terminated source string and compares it against a provided maximum length.
- `transform_string_data`: Iterates through a null-terminated string and modifies each byte by adding 32 if a bit is set in a table indexed by that byte, then continues to the next byte in a loop.
- `store_byte_and_increment_pointer`: Stores a byte from d0 into the address pointed to by a2, increments a2 by one, and branches to another routine.
- `move_a3_to_d0`: Moves the contents of a3 to d0 and restores registers from the stack before returning.
- `compare_case_insensitive_strings_3`: Compares two null-terminated byte strings by iterating through them and treating uppercase letters as their lowercase equivalents.
- `append_string_to_buffer_2`: Locates the null terminator of a destination buffer and copies bytes from a source pointer into that location until a null terminator is encountered, then returns the original destination pointer.
- `find_byte_in_null_terminated_string`: Iterates through a null-terminated string to find a specific byte, returning a pointer to the first occurrence or zero if the byte is not found.
- `search_string_for_byte`: Retrieves a pointer to a string and a target byte from the stack, calls find_byte_in_null_terminated_string to locate the byte within that string, and returns the result.
- `find_last_occurrence_of_byte`: Iterates through a null-terminated string and returns a pointer to the last occurrence of a specific byte.
- `move_a2_to_d0`: Moves the contents of a2 to d0 and restores registers from the stack before returning.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `set_and_branch`: Sets bit 7 of d0, loads a 16-bit word from offset 8 of the stack into d0, and branches to another routine.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `modify_status_and_jump`: The routine performs bitwise operations on d0, loads two 16-bit values from memory using offsets from a3 and a7, calls another routine, and then branches to a fixed address.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `clear_and_load_d0`: Clears the long d0 register, loads a 16-bit word from offset 8 of the stack into d0, and branches to another routine.
- `process_signed_division`: Loads a 32-bit value from offset 8 of the stack into d0 and a pointer from offset 4 of the stack into a0, then if d0 is negative, it stores a byte, negates d0, and calls a routine to perform a normalized fixed-point division before storing a byte from d1 into a location pointed to by a1.
- `copy_and_clear_buffer`: If d0 is not zero, it copies a byte from the address pointed to by a1 to the address pointed to by a0 and increments a0; otherwise, it clears a byte at a0, sets d0 to the address in a0, performs an unlk on a5, subtracts a value from the stack at offset 4 from d0, and returns.
- `parse_signed_integer`: Converts an ASCII string representing a signed decimal integer into a 16-bit value, handling optional leading plus or minus signs.
- `increment_and_store_byte`: Increments a value at a certain offset from a pointer and then stores a byte through a pointer that is subsequently incremented.
- `parse_format_string_2`: Clears a word in a global variable, stores a parameter into a global variable, and then parses a format string while zeroing a byte at that global variable's location.
- `copy_memory_block_3`: The routine copies a block of bytes from one memory area to another using a count argument, handling overlapping source and destination by copying backwards when the source address is at or before the destination address, otherwise copying forwards.
- `traverse_and_free_list_2`: Traverses a linked list of pointers and calls a function pointer for each node before clearing the list head and a pointer in a global structure.
- `find_char_in_string`: It scans a string for a matching character, decrementing a remaining count each step.
- `retrieve_table_value`: Calls another routine to retrieve a value and then calls a second routine using a pointer from that value.
- `traverse_pointer_list_with_conditions`: Iterates through a linked list of pointers and calls another routine if certain bitmask flags or value comparisons are met.
- `handle_function_error`: Checks a global flag, calls a function from a structure in a4, and if the return value is negative one, calls another function and updates a global variable and a constant value.
- `seek_and_update_offset_2`: Executes a function from a table pointed to by a global variable, and if the result is negative, calls another routine to update a 16-bit value at offset 1236 in a structure and sets a 16-bit value at offset 1276 to 22.
- `initialize_structure_bitmasks_2`: Writes a full-byte bitmask to offset 8 of a structure and writes full-word bitmasks to offsets 20 and 24 of that structure, then calls a callback function located at a fixed offset within that structure.
- `cleanup_display_context_3`: Checks if a field at offset 10 of the passed structure is non-zero to invoke a callback, then writes 255 to offset 8 and -1 to offset 20 of that structure.
- `execute_indirect_function_3`: Checks a global variable and, if non-zero, recursively calls another routine before executing a function pointer retrieved from a global table.
- `compute_fixed_point_product_3`: Multiplies two 32-bit fixed-point values stored in two registers and returns the 32-bit product by calculating the product of the high and low 16-bit components and summing the cross terms.
- `negate_and_call_divide`: If d0 is negative, it negates d0; if d1 is negative, it negates d1, calls another routine, and then negates d1 again before returning.
- `negate_and_divide`: Calls another routine, then negates both d0 and d1 before returning.
- `conditional_negate_divide`: Checks if d1 is negative; if so, it negates d1, calls a routine, negates d0, and returns; otherwise it returns immediately.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `compare_and_branch_data`: Loads a pointer from an offset of a register, compares a byte at offset minus one of that pointer to seventy-nine, and branches if the byte is less than or equal to that value; otherwise it modifies registers and branches.
- `execute_conditional_logic_2`: The routine copies a block of memory, calls a routine via a pointer from a global variable, and then conditionally calls another routine based on the result of a pointer dereference.
- `call_library_and_store_result`: Loads an address from a global variable into a6, calls a routine at an offset from a6, stores the result in a memory location at an offset from a5, and if the result is not zero, sets d0 to all ones before branching to another routine.
- `update_and_check_result`: Stores a value into a memory location at an offset from a pointer in a5, calls another routine, and returns either zero or all ones based on the result in d0.
- `return_zero_constant_3`: Clears the d0 register and restores multiple registers from the stack before returning.
- `allocate_and_set_node_3`: If the provided parameter is non-zero, it allocates memory for a structure, sets a byte at offset 8 to 5, a byte at offset 9 to 0, and a longword at offset 14 to the value of the parameter, then returns the pointer; otherwise it returns zero.
- `create_message_node_3`: Obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.
- `get_table_entry_index`: Checks a value against a range and a table, then returns a calculated offset or a fallback value if any check fails.
- `traverse_function_list_2`: Checks a bitmask on a value returned from a library call and, if certain conditions involving a global pointer and a function pointer list are met, clears a pointer and recursively calls another routine.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Eye of the Beholder - Advanced Dungeons& Dragons (1990)(SSI)[cr SKR](Disk 1 of 3)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_large_routine`: This routine contains an address instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_3`: Restores registers a5 and a6 from the stack and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `perform_memory_subtractions`: Subtracts d5 from values at various memory locations using offsets from a0, a1, and a2, decrements a3 and a5, and moves words from memory into locations pointed to by a0 and a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_function`: Copies a global variable to a location, clears 152 bytes of memory starting at a pointer, and calls a function through a pointer stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_conditionally_on_memory`: Loads a long value into a4, moves a long value from a memory location to an offset of a6, and then branches if the result is less than zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_set_structure`: Clears 152 bytes of memory starting at a pointer, writes three values to a structure at offsets 1e8, 1ec, and 1f0, and calls a function through a pointer stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_query_data_structure`: Clears a memory block using a loop, then performs a series of calls through a table in a6 to retrieve a value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_stack_pointers`: Stores the value of a7 into two different offsets of a structure pointed to by a4, then checks a word at an offset of a5 against d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_data`: Sets values in a structure pointed to by a4, calls two routines via pointers in a6, and if a value is not zero, performs several memory operations and calls another routine using a pointer from a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_field`: Copies a value from a structure at an offset of a3 into a structure at an offset of a4, and if a value is not zero, clears bits in a register and writes to several memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_buffer_data`: Checks a value in a structure at an offset of a3, and if it is not zero, calculates a buffer size and calls a routine via a pointer in a6 to process data from a pointer derived from a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_adda_adda_move`: Small routine of 3 instructions that adds a1 to a1, then adds a1 to a1, then copies d3 to -(a0).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_buffer_parameters`: Reads a byte from a pointer in a1, calculates a buffer size and a pointer to the next byte, calls a routine via a pointer in a6, and stores the result in a structure at an offset of a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_arithmetic_registers`: Moves a longword from a1 into an offset of a4, adds d1 to d0 with carry, decrements d1 by seven, and performs a bit change on d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_format_buffer`: Calculates a buffer size, calls a routine via a pointer in a6, and then uses two loops to copy bytes from a source to a destination and insert specific bytes into the destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_cleanup_sequence`: Copies bytes in a loop from a source to a destination, writes a byte to the destination, and calls several routines, including some via pointers in a structure and a global variable.
- `load_global_values`: Loads several values from memory into a stack, calls a routine with these values, and then updates a global variable using concatenated values from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_data`: Loads a pointer from a location offset from a5 into a0, then performs two conditional sequences that move longwords from offsets -7 and -15 of a5 into specific global memory locations and set bits in d0 and d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_call_subroutines`: Copies two longwords from a0 to a1, writes four longwords to offsets -24, -20, -24, and -20 of a structure in a5, calls another routine, calls a second routine, and then conditionally updates a global variable using values from offsets -7 and -15 of a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_partial_pointer_data`: Loads a pointer from a0, then moves a longword from offset -15 of a5 into a global variable and moves a longword from offset -23 of a5 into another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_tasks`: Copies two longwords from a0 to a1, writes two longwords to offsets -24 and -20 of a structure in a5, and calls three different routines using offsets from a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `move_single_pointer`: Loads a pointer into a0 and moves a longword from offset -23 of a5 into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `cleanup_display_context_4`: Copies data between two pointers, calls another routine multiple times with different offsets, and executes a function pointer from a global structure before calling a final routine.
- `setup_pointer_chain_3`: Stores the address of a pointer argument into the memory location pointed to by that argument, increments the stored value by four, clears the following word, and stores the original pointer address into the word after that.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `reset_display_context`: Iterates through a loop checking certain structure members and executes a sequence of cleanup or deallocation callbacks.
- `parse_command_string`: Skips leading whitespace characters in a string and parses subsequent characters into a table of pointers and values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_copy_byte`: Shifts a 32-bit value left by two, decrements a 16-bit value at offset 50 of a structure in a5, stores a 32-bit value from a1 into a global variable at offset 29478 of a2, and if a byte in d0 equals zero, decrements a 32-bit value in a3, stores it at offset 9689 of a structure in a0, stores a 32-bit value from a1 at offset -17 of a5, and copies a byte from the address in a3 to the address pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_escape_sequence`: Increments a counter at offset 814 of a structure, then checks for a double quote character to initiate string parsing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_byte_from_pointer`: Adds d1 to a0, stores a 32-bit value from a1 into a global variable at offset 29478 of a2, and if a byte in d0 is zero, decrements a 32-bit value in a3, stores it at offset 9689 of a structure in a0, stores a 32-bit value from a1 at offset -17 of a5, and copies a byte from the address in a3 to the address pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_string_and_initialize_structure`: Parses a string by handling quoted text and escape sequences such as asterisk-E or asterisk-N, then uses the resulting data to populate a structure and calls several subroutines to initialize state and memory offsets.
- `copy_string_with_limit_2`: Calculates the length of a null-terminated string, determines a minimum length between the source and a provided limit, and copies characters from the source to the destination until the limit is reached or the source ends, ensuring the destination is null-terminated.
- `insert_into_list`: Attempts to insert a new block of memory into a linked list of memory blocks, updating a global counter and managing pointer redirection or splitting existing blocks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_memory`: Stores the value of d7 into a global variable at offset 261 of a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return_4`: Sets d0 to zero, restores registers d7, a2, a3, and a6 from the stack, unlocks a5, and returns.
- `traverse_and_clear_pointers`: Follows a linked list of pointers starting from an offset of 916 in a6, calls another routine for each element, and then stores zero into two global variables at offsets 948 and 952 of a4.
- `clear_global_pointers`: Subtracts the value in a0 from a0, then stores the resulting address into two global variables at offsets 952 and 948 of a4 before returning.
- `search_and_split_memory_block`: Checks a pointer at offset 960 of a structure in a4, potentially clearing it and calling another routine, then attempts to allocate a new block of memory of a specified size plus two bytes and returns the pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_pointer_to_list`: Loads an address from an offset of 103 of a structure in a3 into a0 and stores the value of a1 at the location pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_register_state`: Moves the value of a0 into d0, saves several registers to the stack, and returns.
- `manage_linked_list`: Calls a memory allocation routine and either initializes a new list node or inserts a new node into an existing linked list using pointers stored in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_data`: Performs several memory writes to various offsets of structures and triggers a trap if a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `insert_list_node`: Updates a linked list by setting the next pointer of a new node to a value from a structure, then updates several pointers in that structure to include the new node.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditional_pointer_store`: Checks a word at an offset of a5, and if it is zero, stores a3 at an offset of a4 and pushes a1 onto a stack pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_list_head`: Checks if a pointer at offset 1c of a structure is zero; if so, it stores a3 at that offset, prepares a pointer at offset 12 of a3, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_value_to_stack`: Loads a pointer from an offset of a4 into a3, calculates an offset of 16 from that pointer into a0, and pushes a1 onto the stack at a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `preserve_registers`: Moves the value of a0 into d0, saves d7, a2, a3, and a6 to the stack, and returns.
- `remove_memory_block`: Traverses a linked list of memory blocks to find a block with a size greater than or equal to the requested amount, then splits the block or removes it from the list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_modify_data`: This routine copies a long value from a0 to a location pointed to by a2, updates a2 with the value from a0, copies a long value from a3 to the location in a2, stores d0 at offset 4 of the location in a2, subtracts d7 from a value at offset 44 of the structure in a4, moves a3 to d0, and branches to another location.
- `execute_complex_logic`: This routine performs several memory accesses and calculations involving registers and stack values, and triggers a trap if a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_d1_and_push_word`: This routine decrements a long value in d1 by one and moves a 16-bit value from offset 325 of the structure in a5 to the location pointed to by decrementing a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_logic`: This routine performs several function calls and memory accesses, and returns a value based on the result of a conditional check.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutines_and_restore_registers`: Calls two subroutines and restores several registers from a stack frame before returning.
- `validate_and_store_index`: Performs bounds checking on a value derived from a register and compares it against a signed 16-bit integer, then stores the value into a structure at an offset of 989 and another location at an offset of -5 relative to a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_hunk_metadata`: Iterates through a list of pointers to calculate the total length of strings and populates a structured block of data with string lengths and specific constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_longword_to_structure_offset`: Stores a longword from register d1 into a memory location at an offset of 989 from the address held in register a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_and_validate_hunk_metadata`: Calculates the length of a null-terminated string by looping through its bytes, stores this length and a value from a global variable into a structure, and performs several checks against a value in a register before calling a trap and another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_long_values`: Copies the contents of register d1 to two longword locations at offsets 997 and 1001 of a structure in a4, then copies a longword from offset 269 of a structure in a5 to offset 8529 of a structure in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_string_and_trap`: Calculates the length of a null-terminated string pointed to by a pointer at offset 12 of a structure in a5, stores this length at offset 988 of a structure in a4, calls a function via a pointer in a global variable, performs several memory writes, and executes a trap 6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_suba_move_subq`: Small routine of 3 instructions that subtracts (a1) from a0, then copies a1 to -(a0), then subtracts 2 from d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `concatenate_string_list`: Iterates through a list of strings, concatenating them into a destination buffer with space characters between them and appending a newline character at the end.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_memory_pointers`: Stores the contents of register d1 into a longword at offset 273 of the structure in a0, decrements a longword at offset -23 of the structure in a5, and stores a longword from offset 269 of the structure in a5 into offset 8529 of the structure in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_and_update_state`: Increments a longword at offset -24 of the structure in a5, calculates the length of a null-terminated string pointed to by a pointer at offset 12 of the structure in a5, copies that string to a destination pointed to by a value at offset -24 of the structure in a5, and performs a trap if a comparison involving register d5 fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_structure_values`: Subtracts the value in a1 from a0, pushes a1 onto the stack, and stores three longwords from the structure in a5 at offsets 269, -23, and 269 into offsets 5000, 5081, and 5081 of the structure in a1 respectively, then branches if a longword at offset -23 of the structure in a5 is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `concatenate_string_list_and_call_function`: Concatenates a null-terminated list of strings into a single buffer, separated by spaces, and then performs a series of function calls through a table of pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_pointers`: Clears a pointer in a1, moves a longword from a global variable to an offset in a6, and performs several memory writes to structures pointed to by a5 and a0 using values from d1 and d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_task_and_data`: Calls a function via a pointer in a global variable, shifts a value from the returned pointer to an offset in a5, calls another function if a value at an offset in a structure is zero, calls a function via a pointer in a global variable, copies three values from an offset in a5 to a structure pointed to by a pointer in a5, and performs a trap if a value is out of range.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_data_and_check`: Pushes a1 to the stack, performs a check using a table and the stack pointer, moves a value from an offset in a5 to an offset in a0, and moves a value from a3 to d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_task_structure`: Copies several fields from one structure to another, updates status flags and pointers based on conditional checks of a linked list or sub-structure, and calls a routine to create a new object before calling another routine via a function pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_register`: Reads a byte from a memory location pointed to by a register, performs a bitwise AND operation with a constant, and stores the result into a bitfield within a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields`: updates several fields in a structure pointed to by a value at an offset from a6, including conditional logic to set a 16-bit value and a 32-bit value based on a check of a pointer's contents, then calls two other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `move_data_to_pointers`: moves a 32-bit value from an offset of a5 to an offset of a0, moves a 32-bit value from an offset of a5 to an offset of a1, and moves a 32-bit value from an offset of a5 to an offset of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_function_pointers`: stores two 32-bit values into a structure pointed to by a value at an offset from a5, then calls two functions via pointers stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_register_values`: stores the value of a3 into an offset of a0, stores a value from an offset of a5 into an offset of a1, and stores a value from an offset of a5 into an offset of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_looping_calls`: copies values between registers and offsets of a0 and a1, calls a function via a pointer in a6, then calls another function via a pointer in a6 and compares the result to a value at an offset from a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_pointer_values`: moves a 32-bit value from an offset of a1 to an offset of a0, and moves a 32-bit value from an offset of a5 to an offset of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_until_match`: calls a function via a pointer in a global variable, then enters a loop that calls another function via a pointer in a global variable, stores the result in a structure at an offset from a5, and calls a third function via a pointer in a structure if the result does not match a value at an offset from a5, finally calling two other routines before returning.
- `validate_and_search_path_string`: Iterates through a string to find a null terminator, a colon, or a forward slash, then performs a series of pointer dereferences and calls to other routines to traverse a linked list or structure hierarchy, returning a value or a failure constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_store_pointers`: Clears the a1 register, stores a longword from a global variable into a location at an offset of negative 37 from a pointer in a6, and then stores the contents of d1 into a location at an offset of 10636 from a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_file_path_and_open`: Checks if a string contains a directory separator, then attempts to open a file by retrieving a pointer from a structure and iterating through a list of potential file locations.
- `multiply_fixed_point_values`: Calculates the product of two 32-bit fixed-point values by multiplying their high and low 16-bit components and combining the results, returning the 32-bit product.
- `perform_fixed_point_division`: Performs a complex normalized fixed-point division using d0 and d1, handling sign logic and bit shifting to return the result in d0.
- `normalize_fixed_point_divide_2`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `update_node_fields`: It retrieves a value from a global variable through a table and populates a structure with several fields, including a pointer from another routine and conditional logic based on a parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_to_memory`: Stores a longword from a3 into a memory location at an offset of 267 from a2, pushes a longword from d7 onto the stack, and stores a byte from d1 into a memory location at an offset of 265 from a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields_2`: Sets four bytes at specific offsets within a structure pointed to by a2, calls another routine through a table to set a dword at offset 16, and then either initializes a linked list or calls another routine based on a parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_store_value`: Clears register a1, checks a value at an offset from a7, stores d1 at an offset in a2, pushes a3 onto the stack, and if a zero flag is set, stores a3 at an offset in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields_2`: Calls a routine via a pointer in a6, stores the result at an offset in a2, and if a3 is zero, initializes several fields in a structure starting at a2; otherwise, it calls another routine via a pointer in a6.
- `process_message_and_update_pointer`: Retrieves a pointer from a global variable, calls a routine via a pointer in that variable, calls two other routines via pointers in the variable, stores a value from a third routine into a global variable, calls a routine with an incremented pointer, and stores the original variable into a pointer location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bitmask_and_store_data`: Clears register a1, checks a value at an offset from a7, stores d1 at an offset in a4, increments a byte at a5 and increments a5, checks another value from a7, loads an address into a0, checks a third value from a7, and stores d1 at an offset in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_pointers`: Calls four different function pointers retrieved from offsets of a6, stores a value from a pointer in d1 into a global variable, and updates a function pointer in a structure before calling another routine.
- `restore_registers_and_stack`: Adds four to the stack pointer and restores registers a4 and a6 from the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_value_to_offset`: Stores the value from a1 into a structure at an offset of 433 from a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_routines`: Calculates a value using a pointer from a4, sets a global variable to a constant, calls a routine via a pointer, sets another global variable to zero, sets a third global variable to a4, and calls two routines via pointers in a6.
