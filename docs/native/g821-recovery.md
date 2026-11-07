# Game 821 (faery tale adventure, the (1986)(microillusions)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00821). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901968 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 375 of 375 named (0 author symbols), 375 of 375 described (163 by rule, 206 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2371 of 2371 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13224 of 13256 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 90410 |
| code | 45808 |
| data | 708282 |
| padding | 3708 |
| unused | 53760 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf:fmain | 3 | 13114 | 291 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 81 |

## Cases handled

- runtime segment an address: 57 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 157-159 (bytes 80384-81920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.70)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.22)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.58)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 882-917 (bytes 451584-470016) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.28)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 920-1171 (bytes 471040-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.74)
- faery tale adventure, the (1986)(microillusions)[cr]/Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf: blocks 1280-1501 (bytes 655360-769024) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.69)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_game_state_and_memory` (fmain): Initializes global state variables, configures hardware, and populates memory tables with specific offsets and pointers. Kind: checksum trap. Pass path: if (iVar5 == 0).
- `execute_sequence_of_calls` (fmain): Executes a series of function calls via pointers, updates a global word and a global long value, and modifies two global bytes before calling a routine that performs a memory copy. Kind: loader/protection sequence. Pass path: the sequence continues to the next function call in the list.

## Functions


### Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf:fmain

- `initialize_memory_and_tables`: Calculates a base address from a global value, updates a table entry, and clears a block of memory by zeroing out words in a loop.
- `initialize_hardware_and_input`: Attempts to open an input device and configures its properties.
- `reset_hardware_parameters`: Updates various hardware-related configuration fields in a structure based on bitmask flags.
- `populate_pointer_table`: Reads a sequence of six values from a memory location and stores each value and its subsequent pointer into two separate destination tables.
- `update_map_tile`: Searches a table for a matching tile identifier and updates the game map data.
- `initialize_game_state_and_memory`: Initializes global state variables, configures hardware, and populates memory tables with specific offsets and pointers.
- `update_threshold_values`: Calculates new values for several global variables based on input parameters and existing global data, then calls another routine multiple times.
- `parse_hunk_metadata`: Iterates through a table of data structures to find a matching entry based on coordinate ranges.
- `update_state_variable`: Calculates a value by adding a returned byte to a value from a table and stores it in a global variable.
- `fetch_table_values`: Retrieves two word-sized values from a table using an index derived from a function return and a calculated offset.
- `generate_entity_coordinates`: Iterates through a loop to find valid coordinates using a random number generator and a collision check routine.
- `update_conditional_state`: Evaluates a table index to check specific byte values and updates global variables or calls other routines based on the results.
- `process_hunk_data`: Retrieves values from a table based on a parameter, performs range checks, and updates several global memory locations and status bytes.
- `load_scene_data`: Resets several global state variables and loads a set of coordinates and properties from a table based on a scene index.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `process_entity_command`: Updates an entity's data structure based on a command parameter, modifying internal state flags, coordinates, or timer values.
- `calculate_display_offsets`: Calculates several values based on a parameter and a function return, then stores them in multiple global variables and calls a routine with calculated offsets.
- `select_table_index`: Evaluates multiple conditions based on global values to determine an index, then calls a function using four values from a table at that index.
- `update_threshold_state`: Calculates a value using bitwise shifts and masks derived from two global variables and updates a third global variable with the result.
- `execute_callback_sequence`: Calls three different function pointers stored in a global structure, passing a parameter to the second one.
- `swap_and_update_pointers`: Swaps two pointers in a global structure and then uses those values to update other global memory locations and call several functions.
- `initialize_input_device`: Attempts to open an input device, retrieves its properties, and if successful, configures a structure with specific values and a global variable.
- `reset_device_parameters`: Updates specific fields in a structure pointed to by a global variable and then executes a sequence of function calls using that pointer.
- `refresh_entity_list`: Iterates through a table to find specific flagged entries, stores their indices, and then performs a series of memory updates and function calls using those indices.
- `update_display_state`: Calculates a value based on a table and global state, then performs several memory operations and function calls to update a buffer or display structure.
- `process_game_state_switch`: Dispatches logic based on a state variable stored in a global structure.
- `set_conditional_data`: Checks a bit in a global variable and, if it is not set, stores a passed parameter into a global variable and resets another global byte to zero before calling another routine.
- `update_attribute_bitmasks`: Iterates through three separate data sequences to update specific bits in corresponding bitmask arrays based on whether characters in the source strings are null or non-null.
- `wait_for_threshold`: Repeatedly calls another routine as long as a value in a global variable remains below ten.
- `update_entity_attributes`: Compares current entity properties against a cached state and triggers updates via a function pointer if changes are detected.
- `call_conditional_function`: Checks a bit in a global byte and, if set, calls a function using values from a table and a shifted value.
- `execute_conditional_drawing`: Determines a value based on a global short, then calls a function with specific coordinates and parameters depending on a global byte and a threshold check.
- `execute_sequence_of_calls`: Executes a series of function calls via pointers, updates a global word and a global long value, and modifies two global bytes before calling a routine that performs a memory copy.
- `call_function_with_pointer`: Calls a function using a pointer stored at a global offset and passes a parameter to it.
- `call_function_with_offset`: Calls a function using a pointer stored at a different global offset and passes a parameter to it.
- `process_input_event_list_2`: Iterates through a linked list of input events to update a state buffer and coordinate position values.
- `process_input_event_list`: Iterates through a linked list of input events to update a state buffer and coordinate position values.
- `update_input_state_buffer`: Processes a linked list of input events to update a state buffer and coordinate position values.
- `increment_and_fetch_byte`: Compares two global bytes, and if they differ, fetches a byte from a table using the first byte as an index and then increments that byte modulo one hundred and twenty-eight.
- `advance_pseudo_random_state`: Multiplies a stored 16-bit state by a constant, adds one and stores it back, then returns a value built from the new state.
- `execute_indirect_call_3`: Loads an address from a pointer stored in a table and branches to that address after setting up a register with a constant.
- `prepare_digit_conversion`: Loads an address into a register, sets a small constant in another register, and branches to another routine.
- `convert_and_store_digit`: Divides a word by ten and performs arithmetic to convert the quotient into an ASCII character.
- `fill_memory_with_space`: Fills a memory range starting from a given address with the value thirty-two using a loop.
- `nested_callback_loop`: Executes a triple nested loop that repeatedly calls several routines via function pointers stored relative to a global variable, with conditional extra calls when a counter exceeds a threshold, and returns the original input registers unchanged.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `process_string_tokens`: Iterates through a string, calling a function for each word found and calling another function if a specific byte is encountered.
- `load_stack_parameters`: Loads two longwords from the stack into registers.
- `get_pixel_color`: Calculates a color value by performing bitwise operations and table lookups based on input coordinates and global offsets.
- `call_and_decrement_pointer`: Calls a function through a pointer stored in a table and decrements a global value by four.
- `call_and_subtract_value`: Calls a function through a pointer in a table and subtracts a value derived from a global variable plus four from another global variable.
- `execute_multiple_calls`: Calls another routine using a pointer from a memory offset and then calls another routine eighteen times.
- `copy_block_to_multiple_buffers_2`: This routine calls another routine once and then returns.
- `copy_block_to_multiple_buffers`: Copies 16-bit words from a source pointer derived from a parameter and a register offset into five destination buffers using a loop that processes pairs of source words per iteration.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `execute_single_call`: Calls another routine using a pointer from a memory offset and then returns.
- `execute_many_calls`: Calls another routine using a pointer from a memory offset and then calls another routine seventeen times.
- `call_copy_block_to_multiple_buffers`: Calls a routine that copies 16-bit words from a source pointer into five destination buffers and then returns.
- `normalize_and_iterate_coordinates`: Calculates normalized coordinate offsets from input parameters and applies clamping logic to keep them within a specific range.
- `extract_bitstream_to_tables`: Reads a bitstream from a source buffer to populate five different destination tables with bit-shifted values.
- `copy_memory_block`: Copies a block of data from a source pointer to a destination pointer using a loop that moves four words per iteration.
- `calculate_clamped_indexed_offset`: Calculates two clamped indices by subtracting base values from input parameters and applying bitwise masks.
- `fill_table_from_source`: Populates a table with values retrieved from a nested lookup structure based on provided parameters and global offsets.
- `expand_compressed_data`: Decompresses data from a source buffer into a destination buffer using a scheme where bytes can represent single values or lengths of subsequent bytes.
- `scale_value_by_table`: The routine conditionally scales a short parameter by a value from a table indexed by the second parameter and adds the result to the first parameter, masking the result to fifteen bits.
- `scale_value_with_sign`: The routine conditionally scales a short parameter by a value from a table indexed by the second parameter, adds it to the first parameter, preserves the sign bit of the first parameter, and masks the result to fifteen bits.
- `modify_bit_fifteen`: Tests a bit in a value from the stack and either returns or sets the most significant bit of that value.
- `mask_fifteen_bits`: The routine performs a bitwise AND on a word to mask it to fifteen bits.
- `update_coordinate_values`: Calculates the difference between two sets of parameters and compares them against specific thresholds to increment, decrement, or reset two values stored in a table.
- `compare_and_decrement`: Compares a word in a register to a constant and, if the condition is met, subtracts that constant from a register and stores the result in a global variable.
- `decrement_global_counter`: Compares a word against a constant and decrements a value at a specific offset from a pointer if the comparison condition is met.
- `update_global_counters`: Compares a word against a constant and increments a short value at a specific offset from a pointer if certain conditions are met.
- `decrement_global_word`: Compares a word to a constant, and if it is less than or equal to that constant, subtracts the constant from a register and stores the result in a global variable.
- `decrement_conditional_short`: Compares a word to a constant and, if it is not greater than or equal to that constant, decrements a short value at a specific offset from a pointer.
- `increment_global_short`: The routine increments a short value at a specific offset from a pointer if a condition is met.
- `handle_error_message`: Checks if a parameter is not equal to two and, if so, calls three other routines with constant arguments and a global variable pointer while referencing error text.
- `select_blitter_mode`: The routine indexes a table with a short parameter, applies a series of range checks to choose a small index, then returns a combined value formed from that index and an input register.
- `perform_register_operations`: The routine moves a word from a pointer, modifies two bytes, and performs bit tests on specific registers.
- `update_joystick_index`: Reads joystick input to determine a value, validates it against a range, and updates a global index and another routine if the value changes.
- `process_joystick_input`: Reads joystick port data to determine a value, validates that value against a range, and updates a global variable and another routine if the value has changed.
- `validate_index_range`: Loads a word from a global variable, subtracts a constant, and branches based on whether the result is negative or within a specific range.
- `reset_index_to_nine`: Clears a stored index to zero, then sets it to nine and calls one routine with nine when the previous value was not nine.
- `sample_pixel_from_map_2`: The routine calls another routine twice under conditional tests on the returned short value and may return zero, otherwise it returns the value from the first call.
- `compute_morphed_bitmap`: Takes two pointer arguments and three short arguments and performs a nested loop that writes the bitwise NOT of the AND of five source words into a destination word for each iteration.
- `setup_blitter_loop`: The routine iterates four times to configure blitter registers using values from a table and a pointer.
- `set_blitter_regs`: Writes a constant and two parameters to a set of blitter registers and returns zero.
- `configure_blitter_transfer`: The routine iterates four times to set up blitter registers, including source, destination, and control parameters, based on values from a table and a pointer.
- `execute_blitter_setup`: The routine configures blitter registers with specific values and offsets derived from a table and a pointer.
- `update_blt_parameters`: Iterates through a table to configure hardware registers for bitplane transfers, calculating destination coordinates and dimensions based on values from a global structure.
- `update_blt_parameters_2`: Iterates through a table to calculate and write multiple Blitter control and size parameters to a destination structure.
- `execute_indirect_call`: Performs an indirect call to a function pointer stored in a global structure.
- `mask_test_and_add`: Performs a bitwise OR on a byte in a register using a constant mask, tests a bit in a longword, and adds zero to a byte in another register.
- `calculate_coordinate_differences`: Calculates differences between provided values and table entries, determines signs, and stores a result in a table or a global variable.
- `modify_register_values`: Negates a word in a register and performs conditional branches based on the sign of another register.
- `calculate_complex_state`: Performs arithmetic on registers, uses a value to index into a table, calls another routine, and stores results in various memory locations.
- `add_bytes_conditionally`: Calls another routine and, if a specific bit in the return value is not set, adds one byte to another.
- `calculate_byte_difference`: Calculates a value based on the difference between two bytes and stores it in a memory location, potentially setting another byte to a constant value.
- `modify_attribute_and_branch`: Decrements a value at a pointer location and clamps it to zero if it becomes negative.
- `execute_conditional_logic_2`: Compares a longword against a constant and branches if not equal.
- `branch_and_call_routine`: Tests a longword and branches if non-zero.
- `execute_conditional_logic`: Calls two functions via pointers, performs checks against specific byte values in a table, and conditionally executes other routines based on those results.
- `check_proximity_conditions`: Iterates through a loop to check if specific byte values in a table match certain criteria and calculates distance between coordinates.
- `branch_to_offset`: Clears a register and performs an unconditional branch to another location.
- `check_coordinate_proximity`: Iterates through a loop to compare values from a table against two short values, returning a constant if the differences are within specific small ranges.
- `increment_and_compare`: Increments a longword, loads a word from a memory offset, and compares the two values before branching.
- `setup_stack_frame`: Creates a stack frame, saves registers, clears a word in a global variable, and branches to another location.
- `find_minimum_value`: Iterates through a range to find the smallest value returned by another routine and stores it in a global variable.
- `calculate_value_differences`: Calls a routine multiple times to retrieve indices, calculates the absolute difference between values retrieved from a table, and performs conditional logic based on those differences.
- `clamp_value_to_limit`: Adds a longword to another and compares the result against a limit.
- `sum_and_call_routines`: Sums two long values, then calls two different routines sequentially with specific values in a register.
- `update_table_values`: Calls another routine using values retrieved from a table and, if the result is zero, updates the table with the returned values.
- `store_scaled_values`: Multiplies two different words by a constant and stores the results into a table at offsets calculated from those products.
- `draw_sprite_segments`: Retrieves coordinate data from a table and uses it to call a routine that performs hardware blitter operations for specific segments.
- `populate_coordinate_table`: Calculates and stores a series of values into a table based on conditional logic and input parameters, then calls a routine to process the table.
- `fill_table_with_calls`: Repeatedly calls a routine to fetch values and stores the lower bits of the results into a table, followed by a routine to process the table.
- `dispatch_conditional_routine`: Sets up a stack frame, compares two values from memory, and if they are not equal, calls another routine before branching.
- `dispatch_state_machine_logic`: Increments a counter and uses the resulting value to index into a table for a switch-based dispatch.
- `execute_jump_table`: Calculates an offset based on a comparison and uses it to perform an indirect jump to a location in a table.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `update_circular_buffer`: Updates a value in a table at an index determined by a counter, incrementing that counter in a circular fashion.
- `search_table_byte`: Iterates through a table to find the first non-zero byte.
- `call_library_functions`: Calls two different library functions using base addresses stored in a global variable.
- `print_counted_string`: Measures a zero-terminated string and calls another routine with the string and its length.
- `initialize_local_buffer`: Creates a stack frame, saves registers, loads a pointer from the stack, and initializes a local buffer by storing a calculated address and clearing a word.
- `parse_input_string_expansion`: Iterates through a character stream to populate a buffer, handling carriage returns by terminating the string and processing percent signs by copying a sequence of bytes from a secondary source into the destination buffer.
- `process_input_string`: Parses a character stream by copying bytes into a buffer, handling special characters like percent signs for expansion and carriage returns for termination.
- `branch_to_routine`: Performs a short branch to another routine.
- `expand_string_variables`: Iterates through a character stream to populate a buffer, handling carriage returns by terminating the string and processing percent signs by copying subsequent characters from a table into the buffer.
- `process_table_entry`: Increments a pointer in a global variable, clears a byte at the pointed address, and calls another routine after performing a comparison and stack manipulation.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `iterate_table_pointer`: Retrieves a pointer and a counter from the stack frame.
- `call_other_routine`: Calls another routine and returns.
- `print_found_string`: Searches a table for a non-zero byte and then prints a zero-terminated string using a counted string routine.
- `dispatch_indirect_calls`: Calculates an address from a table using an offset derived from a byte value and performs an indirect call.
- `print_string_length`: Calculates an offset based on a value in a table and calls another routine with a zero-terminated string.
- `initialize_structure_data`: Sets a pointer in a structure, copies a value from a nested pointer, calls several routines with zeroed parameters, and sets a word value and a byte value.
- `update_state_parameters`: Assigns a pointer to a structure, sets a word value, calls another routine, and updates a byte value.
- `populate_table_descending`: Executes a loop that calls a routine with decreasing values starting from one hundred.
- `populate_table_ascending`: Executes a loop that calls a routine with increasing values in increments of five.
- `clear_structure_fields`: Sets two word-sized fields at specific offsets within a structure to zero.
- `update_entity_state_table`: Copies several bytes from a table entry into a destination structure and conditionally calls another routine using values from that entry.
- `update_data_structure`: Calculates an offset into a table, copies a block of data, and updates several fields within that table entry before calling another routine.
- `copy_and_reset_data`: Copies a block of data to a local buffer and sets specific values in a global structure before calling another routine.
- `call_morphed_routine`: Calls another routine using five parameters retrieved from a table.
- `check_entity_state`: Compares a value in a structure against a constant and calls another routine if certain conditions are met.
- `process_audio_samples`: Retrieves data from a source, iterates through it to perform calculations involving a global offset, and stores the results in a table.
- `update_display_and_input`: Checks a global flag to determine if it should trigger a sequence of drawing operations and input polling.
- `render_graphics_loop`: Iterates through a loop to perform complex blitter operations based on values retrieved from tables and conditional checks.
- `check_key_press`: Calls a routine to check for input and stores whether the returned value matches a specific constant in a global variable.
- `allocate_and_copy_memory`: Checks if a pointer is within a certain range, and if not, allocates memory via a library call and copies a block of data to the new location.
- `calculate_polygon_area_and_draw`: Calculates the signed area of two triangles using a cross-product method and updates status flags based on the results.
- `update_entity_state`: Writes a constant to a memory location, calls a routine to check for spatial collisions, and performs conditional updates to a global variable based on a comparison.
- `update_data_from_table`: Copies a word from a table using an index and an offset, then increments a second word from that table and stores two bytes at a fixed location.
- `modify_byte_at_offset`: Calculates a memory address based on a return value from another routine and a table, then either updates a byte at a specific offset or stores a parameter byte.
- `update_byte_at_offset`: Checks a byte at a calculated memory location and either updates it with a specific value or stores a byte from a register.
- `store_byte_to_offset`: Stores a byte from a register into memory at a fixed offset from a register.
- `process_entity_interaction_and_collision`: Checks for spatial collisions and proximity between entities within specific screen boundaries.
- `update_entity_sprite_and_animation`: Updates an entity's screen coordinates and animation frame by calculating offsets from its position and checking boundary constraints.
- `update_entity_position_and_state`: Calculates screen coordinates by subtracting offsets from an entity's position and checks if they fall within specific rectangular boundaries.
- `branch_to_routine_2`: Performs a short branch to another location.
- `update_entity_animation_frame`: Iterates through a list of entity structures to check their state and screen boundaries.
- `update_entity_collision_bounds`: Calculates and updates an entity's bounding box coordinates and animation frame based on its current position and state.
- `update_entity_animation_and_bounds`: Updates an entity's position, animation frame, and bounding box coordinates by copying data from a source structure.
- `update_entity_animation_and_position`: Increments an animation frame counter and calculates a new memory address for an entity structure based on that index.
- `clear_structure_byte`: Compares a byte at an offset of a pointer to a constant and, if equal, clears a byte at a different offset within another structure.
- `update_sprite_position_and_state`: Updates a structure containing coordinate data by copying values from a source pointer and incrementing an index.
- `branch_to_address`: Performs a short branch to a specific memory location.
- `process_input_buffer`: Reads characters from a source and stores them into a table while handling backspaces and carriage returns.
- `execute_indirect_function_pointers`: Calls several function pointers located at specific offsets from two different base pointers.
- `process_entity_collision_and_movement`: Checks an entity's position against a set of boundary or collision values and executes specific response routines based on the match.
- `process_script_state`: Clears a byte at offset one of a structure, adds a 16-bit value from offset two to a 32-bit field at offset sixteen, and calls another routine four times if the first byte is zero.
- `parse_script_command`: Iterates through a data stream to interpret command bytes and their associated parameters.
- `decrement_counter_and_update_state`: Writes constant words to two offsets of a structure pointed to by the first argument, then reads a byte from an offset of the second argument.
- `setup_audio_parameters`: Writes three words to specific memory locations and sets several hardware registers, including audio and DMA control registers.
- `clear_audio_settings`: Clears a bit in a byte at an offset from a pointer and writes two words to specific hardware register locations.
- `store_four_pointers`: Stores four provided 32-bit values into a table at specific offsets from a base pointer.
- `initialize_audio_structures`: Stores four pairs of values into a table, clears several hardware audio volume registers, loads data from a pointer into a table, and sets a DMA control register.
- `reset_audio_volumes`: Sets four hardware audio volume registers to zero and writes a value to a DMA control register and a byte to a memory location.
- `setup_interrupt_handler`: Stores several values into global variables, updates a structure with pointers and constants, and registers an interrupt handler.
- `reset_audio_hardware`: Calls two different routines via pointers from global variables and writes a value to an offset of a structure pointed to by a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_memory_block`: Calculates an offset based on a value, clears a block of memory using a loop, and stores two values into a table.
- `execute_empty_routine`: The routine is empty and performs no operations.
- `parse_dos_command_string`: Parses a command string retrieved from a library function, handling quoted substrings and whitespace to extract parameters.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `load_short_and_branch`: Loads a short integer into a data register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `perform_signed_division`: Determines the sign of a result based on two input values, calls another routine to perform a calculation, and optionally negates that result before returning it combined with the second input.
- `prepare_signed_division`: Saves registers and performs sign checks on two values, negating them if necessary before calling another routine to perform a calculation.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `call_recursive_routine`: Calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1 through a bit-shifting algorithm or modulo arithmetic.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `execute_recursive_callback`: Checks if a pointer in a global variable is non-zero and, if so, calls the routine pointed to before calling itself recursively.
- `execute_callback_loop`: Executes a loop twenty times, then checks several memory locations for non-zero values to call various routines or library functions based on those values.
- `lookup_and_execute_callback`: Uses an index to find a pointer in a table, checks if the index is within bounds, and if the pointer is valid, calls another routine before clearing the pointer in the table.
- `call_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_function_2`: Calls a library function using an offset derived from a pointer stored in a global variable.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_3`: Performs an indirect call to a library function based on a pointer in a global variable.
- `call_library_function_4`: Uses a global variable to locate a library base and executes a function at a specific offset.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_function_5`: Executes a library function by accessing a pointer through a global variable.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_6`: Calls a function at a specific offset within a library base retrieved from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Calls a library function to allocate memory, and if successful, populates several fields of the new structure with provided parameters and results from other calls.
- `update_structure_data`: Recursively calls itself if a specific offset in a structure is non-zero, then sets bytes and words at fixed offsets within that structure before calling another routine and an indirect function.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_indirect_function`: Calls an indirect function passing a parameter and a constant value.
- `call_indirect_function_2`: Executes an indirect function call using a single parameter.
- `populate_structure_fields`: Calls an indirect function with a constant value and, if successful, writes a byte, a word, and a long value into the returned structure at specific offsets.
- `test_pointer_validity`: Loads a pointer from a structure and, if it is not null, restores a previously saved register from the stack.
- `prepare_and_call_routine`: Writes specific values to offsets of a structure, loads a word from a structure into a register, pushes two registers onto the stack, and calls another routine.
- `execute_indirect_call_2`: Performs an indirect function call by dereferencing a pointer located at an offset from a value stored in a global variable.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_function_7`: Executes a function by accessing a specific offset within a library base retrieved from a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `dispatch_indirect_pointer`: Loads a library base from a global variable and calls the function found at a specific offset within that library.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `invoke_library_offset`: Uses a global variable to locate a library base and then calls a function at a designated offset from that base.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_150_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_144_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Faery Tale Adventure, The (1986)(MicroIllusions)[cr].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
