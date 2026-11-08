# Game 218 (Tron 5000): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00218). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901212 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 272 of 272 named (0 author symbols), 272 of 272 described (88 by rule, 173 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2233 of 2233 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8368 of 8368 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 483202 |
| code | 29774 |
| data | 123943 |
| padding | 10853 |
| unused | 253440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| mem | 2 | 117 | 2 |
| Tron 5000.adf:   | 3 | 6983 | 202 |
| Tron 5000.adf:  | 3 | 421 | 9 |
| 8 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 53 |
| Tron 5000.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Tron 5000/Tron 5000.adf: : a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### mem

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to d0 and then branches to another routine.

### Tron 5000.adf:  

- `call_clear_longword_run_and_call`: Calls a routine that clears a sequence of longwords starting from an address derived from the stack and stores the stack pointer and a global value into two locations.
- `call_stub_with_fixed_argument`: Calls a library stub with a base-relative address and the constant two, then returns.
- `initialize_graphics_and_resources`: Loads an ILBM image file and validates its resolution and bitplane depth, then proceeds to open the diskfont library and a specific font.
- `update_counters_and_call_routine`: This routine increments two counters, sets a flag word, adjusts two values by fixed offsets, and passes them with constant arguments to another routine.
- `update_counters_and_call_routine_2`: This routine increments a counter from a stored value, sets a flag word, increments a second counter, passes two offset values with constant arguments to another routine, then calls it with a constant.
- `process_conditional_parameters`: Checks a value at offset 8 of a structure in a5; if it is two, it performs a series of library calls with varying arguments based on the contents of a global variable, otherwise it performs a different set of library calls.
- `advance_positions_five_steps`: Runs a five-step loop that adds velocity values to two stored positions, clears a flag word when a position leaves a fixed range, and calls library stubs to test and update the position each step.
- `update_positions_with_exit_condition`: Runs a loop that updates two positions by adding velocity values and calls library stubs to test them, but terminates and clears a flag word if a position leaves a specific range.
- `find_valid_slot_pair`: This routine attempts to find a valid pair of values by iterating through two nested loops and calling another routine to test combinations, eventually storing the successful pair in a global structure.
- `process_conditional_modes`: This routine checks a value at offset 46 of a structure in a4 and performs different sequences of library calls and data updates based on whether that value is 1, 2, 3, 4, 5, or 6.
- `process_conditional_data_blocks`: This routine checks a value in a structure; depending on that value and several non-zero checks in a global table, it performs multiple calculations and calls another routine with various parameters.
- `check_and_update_neighbour_positions`: Takes a mode value and a target position, sets a threshold count for modes one and two, then loops checking a library-tested value at the position and calling routines that update several neighbouring positions.
- `test_and_mark_position_pair`: Computes a pair of positions offset from stored coordinates, calls a library stub to test the first, and when the result is below a threshold issues two further stub calls on that pair.
- `execute_library_calls`: This routine performs several library function calls using offsets from a base address stored in a global variable.
- `draw_sequence_with_busy_waits`: Selects two coordinate values from a mode argument, then issues a series of drawing calls separated by busy-wait loops that count up to a fixed limit.
- `store_offset_rectangle_coords`: Stores four short coordinate values derived from stored positions with fixed offsets, then calls a library stub and two other routines to position a rectangle.
- `search_free_screen_position`: Searches candidate screen positions in nested loops over fixed ranges, testing each with a library stub and a fixed-point multiply until one passes the threshold checks, then calls further stubs at that position.
- `close_all_and_report_error`: Closes all open resources through the global base, passes its argument to a routine and then calls a final cleanup, with text for a close-all message and an error-number format.
- `wait_for_counter_band`: Busy-waits on a counter until it reaches a value band, then waits until it passes a second threshold before returning.
- `load_level_graphics_files`: Compares a value at an offset of a4 with constants from 1 to 12, and for each match, calls load_ilbm_image_file followed by release_nonzero_slot_handles with a specific pointer.
- `process_resource_and_loop_counter`: Calls two routines to retrieve data from a table, calls release_nonzero_slot_handles twice, performs several other routine calls, and then enters a loop that increments a value in a local variable until it reaches 1000, calling another routine when the loop completes or based on a conditional check of a parameter.
- `format_score_digits_for_display`: Formats a score value into a six-character digit buffer padded with spaces, then passes the buffer to library routines at fixed positions, using the fixed-point multiply routine along the way.
- `call_four_element_routines`: Calls one routine and then four others with base-relative addresses of fixed elements, then returns.
- `advance_feedback_shift_state`: Shifts a 32-bit state word eight times, feeding back an exclusive-or of two low bits on each step, then stores the low word of the result.
- `update_bit_plane_entries`: Iterates over a table of short values, resetting entries below a threshold to a fixed value, clears bytes in up to three bitmap planes selected by each entry's index bits, and then reduces each entry by a stored step.
- `initialize_seventy_entry_tables`: Loops seventy times, filling three per-entry tables with values returned by other routines, offset by fixed amounts.
- `initialize_graphics_and_setup_display`: Loads an ILBM image file and validates that its resolution and bitplane depth match supported modes.
- `process_sixteen_row_columns`: This library routine loops sixteen times over a number of columns set by a stored shift value, reads a value through two routines and passes it on with a column offset, then releases a stored handle through a library stub.
- `open_font_library_and_font`: Opens the diskfont library and then the font named in its data, reporting an error through the text messages if either open fails.
- `call_routine_return_zero`: Calls one other routine and returns zero.
- `initialize_tron_text_record`: Fills a small structure with fixed byte and pointer values, including a pointer to the text TRON, then calls a library stub with it and one other routine.
- `call_stub_with_constant_five`: Calls a library stub with the constant five and a base-relative address, then returns.
- `read_chip_register_with_flag`: Reads a 16-bit custom chip register word at an address indexed by its argument, then sets or clears bit four depending on a bit of a global value, and returns the adjusted word.
- `call_routine_and_return_zero`: Calls another routine using a pointer passed in a1 and returns zero in d0.
- `initialize_tron_text_record_2`: Populates a structure with two bytes, a pointer to a global variable, a 24-bit constant, and a pointer to another routine, then calls a routine using this structure and a constant value.
- `call_stub_with_constant_five_2`: Calls a library function using a constant five and a pointer retrieved from an offset of a global variable, then returns.
- `fill_sixteen_by_sixteen_table`: This routine fills a sixteen-by-sixteen table by looping over both indices and storing a value read through two routines at each position, after an initial call.
- `fill_offset_sixteen_table`: This routine fills the same sixteen-by-sixteen table as its neighbour, but starts the row index at one and the column index at two, skipping the first row and the first two columns.
- `update_score_display_buffer`: Maintains a sorted list of ten values in a table, updates a value if it exceeds the current threshold, and refreshes a string buffer with ASCII digits.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `move_d0_to_d0`: Moves the contents of register d0 to register d0.
- `maintain_sorted_list_and_update_display`: Maintains a sorted list of up to ten values by inserting a new value and shifting existing entries, then updates a character buffer with symbols based on bitwise checks of a global status word.
- `reassigns_d0_register`: Copies the contents of the d0 register to itself.
- `reassigns_data_register_d0`: Copies the contents of data register d0 to itself.
- `dispatch_error_and_cleanup`: Executes nested loops to write bytes to a global buffer, calls a formatting routine, and performs conditional cleanup and indirect jumps based on the results of several routine calls.
- `save_highscores_and_format`: Calls a routine to handle a value, stores the result in a global variable, and then conditionally calls a formatting routine or an indirect jump routine based on the result.
- `read_chip_register_with_flag_2`: Reads a 16-bit custom chip register word at an address indexed by its argument, then sets or clears bit four depending on a bit of a global status.
- `format_digit_buffer_and_update_display`: Initializes a six-byte buffer with spaces and fills it with ASCII digits derived from a value retrieved through a 32-bit division and fixed-point multiplication sequence.
- `format_signed_number_string`: This routine converts a number to a decimal digit string in a buffer, prefixing a minus sign for negative values and ending with a zero terminator, then passes the string to another routine.
- `reverse_string_in_place`: Reverses the bytes of a buffer in place, using the length returned by another routine to set the bounds.
- `draw_two_numeric_labels`: Formats two stored numbers into small text buffers and draws them with a sequence of library stub calls at fixed positions.
- `next_random_value_below_limit`: Advances a stored 32-bit state with a shift-and-exclusive-or step, then returns the result reduced modulo the limit argument; with a non-positive limit it only stores a new state.
- `load_ilbm_image_file`: Opens a file and parses its IFF chunks to extract the BMHD, CMAP, and BODY data.
- `compare_four_bytes`: Compares four consecutive bytes from two different memory locations and returns zero if they are identical, otherwise it returns one.
- `format_string_with_arguments`: Checks two global variables and calls a routine for each if they are non-zero, then processes a variable number of arguments by fetching values from a table and calling a formatting routine.
- `fill_slot_values_from_stub`: Loops over a count of four-byte slots, fetching a value through a routine and a library stub for each, storing it at a fixed offset in the slot and calling a further routine when the value is zero.
- `release_nonzero_slot_handles`: Walks a slot array whose length is given by a count byte, releasing each non-zero pointer through another routine and clearing it to zero.
- `initialize_sound_resource`: Checks if a sound resource exists in a table, then attempts to open it, load its data, and verify its integrity.
- `initialize_audio_slots_and_records`: Fills two arrays of five four-byte slots with values returned by other routines, then opens the audio device and reports an error through the text system if it fails.
- `configure_channel_slot`: If a flag at offset 0 of a table entry is non-zero, it writes fixed values to offsets 28 and 30 of a slot structure, copies a value from a global table to offset 42, writes a value from a parameter to offset 44, and calls another routine with the slot pointer.
- `configure_channel_slot_pair`: If a flag at offset 0 of a table entry is non-zero and a shared flag is clear, it writes field values into a slot structure, calls another routine, then writes a second configuration with position fields and a computed threshold into the same slot structure before calling another routine.
- `call_eight_sound_resource_initializations`: Calls initialize_sound_resource eight times in sequence with varying integer arguments, then returns.
- `call_routine_with_zero`: Calls another routine with a zero argument and returns.
- `execute_single_slot_config`: Performs a single configuration task via a slot structure.
- `call_routine_with_zero_twice`: Calls another routine twice with a zero argument and returns.
- `execute_double_slot_config`: Performs two consecutive configuration tasks via slot structures.
- `call_routine_with_zero_argument`: Pushes two zero words and the constant two onto the stack, calls another routine, and then returns.
- `invoke_subroutine_via_pointer`: Loads a value from a memory location into a register and performs a branch to that address.
- `return_from_subroutine`: This routine performs a single return from a subroutine.
- `return_from_subroutine_2`: The routine performs a single return instruction to the caller.
- `update_channel_slot_data`: Writes field values into a slot structure and passes it to another routine.
- `initialize_slot_configuration`: If a flag at offset 0 of a table entry is non-zero and a shared flag is clear, it writes field values into a slot structure, calls another routine, and returns.
- `call_routine_zero_then_arg`: Calls another routine with a zero argument, then again with the constant sixty-four, and returns.
- `execute_callback_routine`: Calls another routine via a pointer stored in a register and returns.
- `execute_mixed_slot_configs`: Performs one single configuration task followed by another configuration task with a specific parameter.
- `configure_channel_slot_2`: Writes fixed field values and stored parameters into a slot structure and passes it to another routine.
- `call_routine_zero_four_times`: Calls another routine with a zero argument four times in sequence and returns.
- `configure_slot_pair`: Writes field values into a slot structure and then writes a second set of values into another structure.
- `configure_slot_pair_alt`: Performs a dual slot configuration by writing values into two separate structures.
- `clear_longword_run_and_call`: Clears a run of longwords starting at an offset from a pointer derived from the stack, stores the stack pointer and a global value into two fields at an offset from that same pointer, then calls another routine.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `format_text_and_output`: Sets up a character output buffer and calls a formatting routine with a byte-output callback and the stack arguments, then passes the buffer start and length to an output routine, directly or through a global flag.
- `handle_buffer_overflow`: Increments a pointer in a global variable, writes a byte from offset 9 of a5 to that location, and if the distance between the pointer and a base address is 40, calls another routine or an output routine based on a global flag.
- `flush_output_buffer_remainder`: Passes the bytes still held in the output buffer to an output routine, resets the buffer pointer, and returns a masked byte from a stored structure.
- `examine_unrecovered_routine`: This routine contains a loop that performs 32-bit division and calls two other routines, but the decompiler could not produce pseudo-code for it.
- `parse_formatted_string`: Parses a string for format specifiers, converting numeric substrings into integers and handling various type codes.
- `perform_signed_division`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if the sign flag indicates a negative result.
- `perform_signed_division_2`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if necessary.
- `execute_unsigned_division`: Calls another routine and moves the result from register d1 to d0 before returning.
- `perform_32_bit_division`: Performs a 32-bit unsigned integer division using a software algorithm, returning the quotient and remainder concatenated in a 64-bit value.
- `iterate_and_call_handler`: Loops a specific number of times based on a product of two values from a structure in a5, calling another routine in each iteration and returning a value from a structure in a5 if the loop completes.
- `update_node_list`: Compares a value at offset 4 of a structure in a2 with a value at offset 6, and if the first is not less than the second, calls another routine and writes a masked byte from offset 8 of a5 to a location pointed to by a structure in a2.
- `sweep_records_in_range`: Repeatedly calls another routine on record addresses stepped by twenty-two bytes across a fixed base-relative range, then returns.
- `check_status_byte`: Loads a pointer from offset 8 of a structure in a5 into a2, then checks if bit 2 of the byte at offset 12 of that structure is set, calling another routine if it is and otherwise returning a value in d0.
- `process_and_clear_node_structure`: Checks a status byte at offset 12 of a structure in a2; if bit 2 is set, it calls another routine and combines the result with a value from offset 13; if bit 1 is set, it calls another routine using a pointer from offset 8; if bit 5 is set, it calls two different routines using a pointer from offset 18; finally, it clears the first 9 bytes of the structure and returns the combined result in d0.
- `manage_linked_node_list`: Checks a status byte at offset 12 of a structure and, if bit 4 is set, returns a specific value; otherwise, it evaluates bit 2 to either clear the structure or perform a recursive insertion into a linked list by calling another routine and updating pointers and status bits.
- `find_first_zero_slot`: Iterates through a table of pointers, checking a byte at offset 12 of each entry, and returns the first pointer where that byte is zero or the end of the table is reached.
- `branch_to_offset`: Performs a short branch to a specific instruction offset.
- `clear_and_branch`: Clears the long value in a2 and the long values at offsets 4 and 8 of the structure pointed to by a2, moves a2 to d0, and then performs a short branch.
- `update_indexed_table_entry`: Checks an indexed table entry via a routine, and if the result is non-zero, updates a 16-bit value at offset 16 and a byte at offset 12 of a structure in a2, or otherwise sets offset 16 to 1 and offset 8 to a pointer.
- `initialize_linked_list_traversal`: Sets up a stack frame, saves a2 and a3, loads a pointer from a table using a4, and performs a short branch.
- `free_list_nodes_in_loop`: Repeats over a singly linked list held in a register, releasing each node with its stored size plus eight through a routine until the link is null, then clears a global variable.
- `insert_into_linked_list`: Calls a routine to allocate memory, then if successful, inserts a new node into a linked list by updating pointers at a global location and the new node.
- `forward_low_word_argument`: Passes the low 16 bits of its argument to a single unnamed routine and returns.
- `initialize_list_pointer`: Subtracts a value from a register and loads a pointer from a global variable into a2, then branches to another routine.
- `traverse_linked_list`: Compares a pointer in a6 with a2 and loops through a linked list by moving the next pointer from the address in a2 into a2 and d3 until the pointer matches the value in a6, then sets d0 to 255 and returns.
- `move_data_conditionally`: If a register is non-zero, it moves a longword from the location pointed to by a2 to the location pointed to in d3, otherwise it branches.
- `call_library_via_pointer`: Loads a pointer from a table using a2 and a4, sets a6 to a value at offset 4 of that pointer, pushes a2 to the stack, calls a routine, cleans the stack, sets d0 to zero, and performs a short branch.
- `check_indexed_table_entry`: Looks up a six-byte entry in a global table using the low word of its argument, passes that entry to a library call, and returns true when the call returns a nonzero value.
- `call_and_store_result`: Calls two different routines and, if the result in d0 is not zero, stores d0 into a global variable at offset minus 30648 of a4, otherwise it calls another routine and stores d0 into a global variable at offset minus 30648 of a4.
- `zero_and_branch`: Sets d0 to zero and performs a short branch.
- `execute_conditional_handler_chain_2`: Calls a routine with a value from a structure, calculates an offset into a table, and if the table entry is null or meets specific bitwise criteria, updates a global variable and returns; otherwise it calls another routine with three arguments and returns the result.
- `execute_conditional_handler_chain`: Calls an unnamed routine, stores the result, and returns if bit 12 of that result is not set; otherwise, it checks a 16-bit value at an offset in a4, and if non-zero, calls three separate unnamed routines in sequence before returning the original result.
- `invoke_stored_handler_then_call`: Checks if a global variable at offset minus 31026 of a4 is non-zero, calls the routine at that address if it is, then calls another routine and returns.
- `initialize_libraries_and_parse_arguments`: Opens the dos and mathffp libraries and stores their bases in global variables.
- `copy_register_d0`: Copies the contents of register d0 to itself.
- `run_ten_steps_library_call`: Runs a sequence of ten calls and optional cleanup, including several library calls and a final routine, returning the result of the final routine or its argument.
- `set_d0_and_branch`: Sets d0 to a constant value and performs a short branch.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `process_table_entry`: Checks if an index is within a specific range and if its corresponding table entry is non-zero; if so, it checks a bit in the entry, potentially calls another routine, and clears the entry.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a location relative to a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `dispatch_indirect_jump`: Restores d1 and d2 from the stack, loads a pointer from an offset of minus 30488 from a4 into a6, and jumps to an address at offset minus 102 from that pointer.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `dispatch_indirect_jump_2`: Restores d1 and d2 from the stack, loads a pointer from an offset of minus 30488 from a4 into a6, and jumps to an address at offset minus 84 from that pointer.
- `call_library_offset_30`: Jumps to an address located at an offset of minus 30 from a pointer found at an offset of minus 30488 from a4.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `dispatch_indirect_jump_3`: Restores d1, d2, and d3 from the stack, loads a pointer from an offset of minus 30488 from a4 into a6, and jumps to an address at offset minus 42 from that pointer.
- `dispatch_indirect_jump_4`: Copies a 32-bit value from the stack into d1, loads a pointer from an offset of minus 30488 from a4 into a6, and jumps to an address at offset minus 90 from that pointer.
- `call_library_offset_48`: Jumps to an address located at an offset of minus 48 from a pointer found at an offset of minus 30488 from a4.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_offset_414`: Jumps to an address located at an offset of minus 414 from a pointer found at an offset of minus 28676 from a4.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_record`: Calls a routine to obtain a pointer, and if successful, populates a structure at that pointer with values from a5 and a previous routine result, then calls further routines to finalize the record.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive, clears its counter field, and then calls two routines with the record's stored index and a fixed code.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `assign_record_parameter`: Calls a routine to obtain a pointer, and if successful, sets a byte at offset 8 of that pointer, a word at offset 18, and a long at offset 14 of the structure.
- `branch_on_pointer_equality`: Loads a pointer from a global variable into a2, compares it with a value from a5, and branches to another routine if they are not equal, otherwise it returns.
- `initialize_structure_fields`: Sets a byte at offset 8 of the structure in a2 to 255, sets 32-bit values at offsets 20 and 24 of the structure in a2 to an address, pushes a word from offset 18 of the structure in a2 and the address in a2 onto the stack, calls another routine, cleans the stack, and branches back to an earlier instruction.
- `call_library_offset_198`: Jumps to an address located at an offset of minus 198 from a pointer found at an offset of minus 28676 from a4.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_294`: Jumps to an address located at an offset of minus 294 from a pointer found at an offset of minus 28676 from a4.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_offset_132`: Jumps to an address located at an offset of minus 132 from a pointer found at an offset of minus 28676 from a4.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function`: Performs an indirect jump to a function located at an offset of minus 542 from a library base stored in a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_2`: Performs an indirect jump to a function located at an offset of minus 552 from a library base stored in a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_30_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Tron 5000.adf: 

- `process_dos_hunk_table`: Walks a linked list of memory hunks to identify specific AmigaDOS hunk types, performs memory allocation, and applies relocations to the data by updating pointers and offsets within the identified memory blocks.
- `read_next_longword_from_stream`: Copies the longword at the current read pointer into a global variable and advances the pointer to the next entry.
- `process_linked_list_and_allocate`: Performs several operations including calling a library function, managing a linked list of entries, and potentially calling another routine via a pointer from a stack-pushed address, while using a loop to iterate through entries.
- `close_library_and_permit`: Calls a library function to close a library if a global pointer is non-zero, calls another routine, and then calls a library function to re-enable multitasking before returning zero.
- `free_two_stored_buffers`: Frees each of two stored memory blocks through a library call when its pointer is nonzero.
- `scan_string_between_library_calls`: Calls a library function, measures a zero-terminated string from an address held in a register, then calls a second library function.
- `decompress_bitstream_to_buffer`: Reads a bitstream from a source buffer, validates it against a checksum, and expands the compressed data into a destination buffer using variable-length bit sequences.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count held in a register, reloading the next longword from memory and shifting in a marker bit when the buffer runs out.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `branch_to_library_routine`: As library code, it performs a short branch to an offset of 80.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, it attempts to open two libraries and, if successful, initializes a structure with a linked list and a zeroed field.
- `remove_font_buffer_node`: Library code that removes a node from a linked list by updating pointers and calls a library function, then decrements a 16-bit counter in a structure.
- `manage_font_buffer_list`: Checks if a specific flag is zero and a count is not greater than zero; if so, it calls two library functions, calculates a new pointer using an offset and a length, calls a third library function, and inserts a structure into a linked list.
- `set_bit_and_clear_d0`: As library code, it sets bit 7 of a byte at offset 14 from a pointer in a6 and clears d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `check_bits_and_jump`: As library code, it decrements a word at offset 38 from a6, and if the result is not zero and bit 7 of a byte at offset 14 is set, it performs an indirect jump to an offset of minus 18 from a6; otherwise it returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `increment_pointer_if_not_null`: This library routine checks if the byte at the address in a0 is not zero; if it is not zero, it increments a0 and branches.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_bytes_and_scan`: This library routine compares a byte from a pointer in a0 with a byte from a pointer in a1; if they are not equal, it checks if the byte at a0 is non-zero and loops if it is.
- `compare_byte_and_branch`: Loads a byte from a pointer in a0, decrements a1, and compares the byte at the new pointer location with d0; it then either sets d0 to a constant or branches.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_branch_byte`: This library routine loads a byte from a pointer in a0 and compares it to a byte in d0, then branches to another location if they are not equal.
- `increment_and_branch`: This library routine increments the value in a0 and branches to another location.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations with constants 47 and 58.
- `parse_font_metadata_list`: Iterates through a linked list of font entries, calculating the required buffer space by measuring string lengths and then copies string data into a destination buffer.
- `update_font_buffer_offset`: Subtracts a word value from a pointer stored at offset negative 4 of a structure, then decrements a counter by that word value plus 10.
- `copy_font_data_and_update_structure`: Copies a sequence of bytes from a source pointer to a destination buffer using a loop, then updates a structure by writing a status byte, a pointer, and several data values from a source table.
- `parse_font_data_file`: Opens a file, locks it, and examines its contents to parse font information.
- `decrement_font_buffer_pointer`: Decrements a 32-bit pointer stored at offset negative 4 from a6 by a 16-bit value in d4, then decrements d2 by that same value and branches to a loop if the result in d2 is non-negative.
- `copy_font_data_bytes`: Copies a sequence of bytes from a source buffer to a destination buffer using a loop, then calls a routine to read and parse font data if the byte count matches a specific value.
- `read_and_parse_font_data`: Reads a block of data from a disk source and, if the read length matches a specific constant, parses the data into a structure by copying a 4-byte longword, a 2-byte word, and two individual bytes into a destination buffer.
- `execute_library_calls_and_negate`: This library routine calls three different routines, then checks if a value in d2 is negative; if so, it negates it and returns the result, otherwise it returns zero.
- `return_zero_constant`: The library routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Tron 5000.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_address`: Clears bit 0 of d0 and performs an absolute long jump to a specific address.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
