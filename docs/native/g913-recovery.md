# Game 913 (tron 5000 (1988)(softgang)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00913). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Tron 5000 (1988)(Softgang).adf:   | 3 | 6983 | 202 |
| Tron 5000 (1988)(Softgang).adf:  | 3 | 421 | 9 |
| 8 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 53 |
| Tron 5000 (1988)(Softgang).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tron 5000 (1988)(softgang)/Tron 5000 (1988)(Softgang).adf: : a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### mem

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to register d0 and then branches to another location.

### Tron 5000 (1988)(Softgang).adf:  

- `call_clear_longword_run_and_call`: Calls a routine that clears a sequence of longwords starting from an offset relative to a stack-based address and updates two fields in a structure with the stack pointer and a global value.
- `call_stub_with_fixed_argument`: Calls a library stub with a base-relative address and the constant two, then returns.
- `initialize_graphics_and_load_assets`: Loads an ILBM image and validates its resolution and bitplane depth, then proceeds to open the diskfont library and a specific font.
- `update_counters_and_call_routine`: This routine increments two counters, sets a flag word, adjusts two values by fixed offsets, and passes them with constant arguments to another routine.
- `update_counters_and_call_routine_alt`: This routine increments a counter from a stored value, sets a flag word, increments a second counter, passes two offset values with constant arguments to another routine, then calls it with a constant.
- `process_conditional_parameters`: Checks a value at an offset from a pointer; if it is two, it calls a library stub with a specific address and returns; otherwise, it performs different sequences of library calls and arithmetic based on the value at another offset from the pointer.
- `advance_positions_five_steps`: Runs a five-step loop that adds velocity values to two stored positions, clears a flag word when a position leaves a fixed range, and calls library stubs to test and update the position each step.
- `update_position_with_bounds_check`: Runs a loop that adds velocity to two stored positions and calls library stubs to test them, but if a position leaves a specific range, it clears a flag and returns.
- `configure_slot_pair_alt_2`: This routine calculates two sums from a table and calls another routine.
- `process_conditional_parameters_2`: This routine checks a value at an offset from a structure in a4.
- `process_conditional_data_blocks`: This routine checks a parameter to decide between two execution paths, then performs a series of checks on global variables to call a routine with calculated offsets and constant arguments.
- `check_and_update_neighbour_positions`: Takes a mode value and a target position, sets a threshold count for modes one and two, then loops checking a library-tested value at the position and calling routines that update several neighbouring positions.
- `test_and_mark_position_pair`: Computes a pair of positions offset from stored coordinates, calls a library stub to test the first, and when the result is below a threshold issues two further stub calls on that pair.
- `execute_library_calls`: This routine performs several library function calls using offsets from a base address stored in a global variable.
- `draw_sequence_with_busy_waits`: Selects two coordinate values from a mode argument, then issues a series of drawing calls separated by busy-wait loops that count up to a fixed limit.
- `store_offset_rectangle_coords`: Stores four short coordinate values derived from stored positions with fixed offsets, then calls a library stub and two other routines to position a rectangle.
- `search_free_screen_position`: Searches candidate screen positions in nested loops over fixed ranges, testing each with a library stub and a fixed-point multiply until one passes the threshold checks, then calls further stubs at that position.
- `close_all_and_report_error`: Closes all open resources through the global base, passes its argument to a routine and then calls a final cleanup, with text for a close-all message and an error-number format.
- `wait_for_counter_band`: Busy-waits on a counter until it reaches a value band, then waits until it passes a second threshold before returning.
- `load_level_images`: Compares a value from a global variable against constants from one to twelve, then calls a routine to load an ILBM image and another routine to release non-zero slot handles based on the matched value.
- `process_system_data`: Retrieves pointers from a table using an index, calls several routines to handle data and memory, and then enters a loop that increments a local counter up to one thousand, branching to another routine based on the value of a parameter at offset eight of a structure.
- `format_score_digits_for_display`: Formats a score value into a six-character digit buffer padded with spaces, then passes the buffer to library routines at fixed positions, using the fixed-point multiply routine along the way.
- `call_four_element_routines`: Calls one routine and then four others with base-relative addresses of fixed elements, then returns.
- `advance_feedback_shift_state`: Shifts a 32-bit state word eight times, feeding back an exclusive-or of two low bits on each step, then stores the low word of the result.
- `update_bit_plane_entries`: Iterates over a table of short values, resetting entries below a threshold to a fixed value, clears bytes in up to three bitmap planes selected by each entry's index bits, and then reduces each entry by a stored step.
- `initialize_seventy_entry_tables`: Loops seventy times, filling three per-entry tables with values returned by other routines, offset by fixed amounts.
- `initialize_graphics_and_setup_display`: Loads an ILBM image file and validates that its resolution is 40 and its bitplane depth is either 200 or 256.
- `process_sixteen_row_columns`: This library routine loops sixteen times, calculating a column count from a value in a structure.
- `open_font_library_and_font`: Opens the diskfont library and then the font named in its data, reporting an error through the text messages if either open fails.
- `call_routine_return_zero`: Calls one other routine and returns zero.
- `initialize_tron_text_record`: Fills a small structure with fixed byte and pointer values, including a pointer to the text TRON, then calls a library stub with it and one other routine.
- `call_stub_with_constant_five`: Calls a library stub with the constant five and a base-relative address, then returns.
- `read_chip_register_with_flag`: Reads a 16-bit custom chip register word at an address indexed by its argument, then sets or clears bit four depending on a bit of a global value, and returns the adjusted word.
- `call_routine_and_return_zero`: Calls update_bit_plane_entries and returns zero in the d0 register.
- `initialize_tron_text_record_2`: Populates a structure with two constant bytes, a pointer to a global variable, a pointer to a specific memory location, and a pointer to another routine, then calls a routine with a constant and a pointer to this structure.
- `call_library_stub_with_five`: Calls a library function using a constant five and a pointer retrieved from an offset of a global variable, then returns.
- `fill_sixteen_by_sixteen_table`: This routine fills a sixteen-by-sixteen table by looping over both indices and storing a value read through two routines at each position, after an initial call.
- `fill_offset_sixteen_table`: This routine fills the same sixteen-by-sixteen table as its neighbour, but starts the row index at one and the column index at two, skipping the first row and the first two columns.
- `update_score_display_buffer`: Maintains a sorted list of up to ten values in a structure, then updates a character buffer with ASCII digits or symbols based on bitwise checks of a status word.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `move_d0_to_d0`: Moves the contents of register d0 to register d0.
- `maintain_sorted_list_and_update_display`: Maintains a sorted list of up to ten entries by inserting a new value and shifting existing entries, then updates a character buffer with ASCII digits or symbols based on bitwise checks of a value in a global structure.
- `copy_data_register_to_itself`: Copies the contents of data register 0 to data register 0.
- `reassigns_data_register_d0`: Copies the contents of data register d0 to itself.
- `execute_batch_operations`: Performs nested loops to write a byte to a table, calls several routines to process data, and handles conditional error reporting and cleanup based on returned values.
- `save_highscores_and_format`: Calls a routine to format text, stores the result in a global variable, and if the result is non-zero, calls a routine to write the data and another to clear the global variable.
- `read_chip_register_with_flag_2`: Reads a 16-bit custom chip register word at an address indexed by its argument, then sets or clears bit four depending on a bit of a global variable, and repeats the process if the bit is not set.
- `format_score_digits_for_display_2`: Initializes a six-byte buffer with spaces and populates it with ASCII digits derived from a value retrieved from a table, then passes this buffer and calculated fixed-point values to several library functions in a loop that runs up to ten times.
- `format_signed_number_string`: This routine converts a number to a decimal digit string in a buffer, prefixing a minus sign for negative values and ending with a zero terminator, then passes the string to another routine.
- `reverse_string_in_place`: Reverses the bytes of a buffer in place, using the length returned by another routine to set the bounds.
- `draw_two_numeric_labels`: Formats two stored numbers into small text buffers and draws them with a sequence of library stub calls at fixed positions.
- `next_random_value_below_limit`: Advances a stored 32-bit state with a shift-and-exclusive-or step, then returns the result reduced modulo the limit argument; with a non-positive limit it only stores a new state.
- `load_ilbm_image_file`: Opens a file and parses an IFF ILBM image by locating the BMHD, CMAP, and BODY chunks.
- `compare_four_bytes`: Compares four consecutive bytes from two different memory locations and returns zero if they are identical, otherwise it returns one.
- `format_string_with_arguments`: Checks global variables to call routines for cleanup or data processing, then uses a table to fetch arguments and calls a formatting routine to output a string with two parameters.
- `fill_slot_values_from_stub`: Loops over a count of four-byte slots, fetching a value through a routine and a library stub for each, storing it at a fixed offset in the slot and calling a further routine when the value is zero.
- `release_nonzero_slot_handles`: Walks a slot array whose length is given by a count byte, releasing each non-zero pointer through another routine and clearing it to zero.
- `initialize_sound_file_resources`: Checks if a sound file pointer exists in a table, then attempts to open the file and allocate memory for it.
- `initialize_audio_slots_and_records`: Fills two arrays of five four-byte slots with values returned by other routines, then opens the audio device and reports an error through the text system if it fails.
- `configure_channel_slot`: If the channel's flag is set, writes fixed field values and the channel's stored parameters into a slot structure and calls another routine.
- `configure_channel_slot_pair`: If the channel and a shared flag are clear, writes field values into a slot structure, passes it to another routine, then writes a second configuration with position fields when a computed threshold is met.
- `call_eight_routines_in_sequence`: Calls eight other routines in sequence with varying integer arguments and returns.
- `call_routine_with_zero`: Calls another routine with a zero argument and returns.
- `execute_single_slot_config`: Performs a single configuration task via a slot structure.
- `call_routine_with_zero_twice`: Calls another routine twice with a zero argument and returns.
- `execute_double_slot_config`: Performs two consecutive configuration tasks via slot structures.
- `call_subroutine_with_zero_arguments`: Pushes two zero words and the constant two onto the stack, calls another routine, and then returns.
- `invoke_subroutine_via_pointer`: Loads a value from a memory location into a register and performs a branch to that address.
- `return_from_subroutine`: This routine performs a single return from a subroutine.
- `return_from_subroutine_2`: The routine performs a single return instruction to the caller.
- `update_channel_slot_data`: Writes field values into a slot structure and passes it to another routine.
- `initialize_slot_configuration`: If the channel and a shared flag are clear, writes field values into a slot structure, passes it to another routine, then writes a second co.
- `call_routine_zero_then_arg`: Calls another routine with a zero argument, then again with the constant sixty-four, and returns.
- `execute_callback_routine`: Calls a routine via a pointer stored in a register.
- `execute_mixed_slot_configs`: Performs one single configuration task followed by another configuration task with a specific parameter.
- `configure_channel_slot_2`: Writes fixed field values and stored parameters into a slot structure and passes it to another routine.
- `call_routine_zero_four_times`: Calls another routine with a zero argument four times in sequence and returns.
- `configure_slot_pair`: Writes field values into a slot structure and then writes a second set of values into another structure.
- `configure_slot_pair_alt`: Performs a dual slot configuration by writing values into two separate structures.
- `clear_longword_run_and_call`: Clears a run of longwords starting at an offset from a stack-relative base, stores the stack pointer and a global value into two fields at specific offsets, then calls another routine.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `format_text_and_output`: Sets up a character output buffer and calls a formatting routine with a byte-output callback and the stack arguments, then passes the buffer start and length to an output routine, directly or through a global flag.
- `update_buffer_and_callback`: Writes a byte to a structure, checks if the buffer length reaches 40, and if so, calls another routine or a specific subroutine with a pointer and length.
- `flush_output_buffer_remainder`: Passes the bytes still held in the output buffer to an output routine, resets the buffer pointer, and returns a masked byte from a stored structure.
- `examine_unrecovered_routine`: Contains a loop that branches back to an earlier instruction and calls another routine.
- `parse_format_string`: Parses a format string to extract numeric values, floating-point precision, or string lengths, and calls other routines to process the resulting data.
- `perform_signed_division`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if the sign flag indicates a negative result.
- `perform_signed_division_2`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if necessary.
- `perform_division_wrapper`: Calls a routine and moves the result from register d1 to d0 before returning.
- `perform_32_bit_division`: Calculates the quotient and remainder of a 32-bit unsigned integer division using a software algorithm, returning the results concatenated in a 64-bit value.
- `loop_event_processing`: Runs a loop that calls another routine for a set number of iterations determined by a multiplication of two values in a5, returning 0 if the called routine returns -1.
- `process_event_list`: Iterates through a list of structures, calling another routine for each entry and writing a byte from a structure in a5 to a location pointed to by a structure in a2.
- `sweep_records_in_range`: Repeatedly calls another routine on record addresses stepped by 22 bytes across a fixed base-relative range, then returns.
- `check_event_status`: Loads a pointer from an offset of 8 in a5 into a2, then checks if a byte at offset 12 of the structure in a2 is zero; otherwise it returns a value of 255 in d0.
- `process_and_clear_event_structure`: Checks a status byte at offset 12 of a structure pointed to by a2; if specific bits are set, it calls various subroutines using data from offsets 8, 13, and 18 of the structure, then clears the first 12 bytes of the structure before returning.
- `process_event_structure_flags`: Checks bit 4 of a status byte at offset 12 of a structure; if set, it returns a value of negative one.
- `search_and_clear_slots`: Loops through a list of pointers, checking a byte at offset 12 of each, and if it is zero, clears the long value in a2 and the long values at offsets 4 and 8 of the structure pointed to by a2 before returning.
- `branch_to_offset`: Branches to a relative offset.
- `clear_and_move_pointer`: Clears the long value in a2 and the long values at offsets 4 and 8 of the structure pointed to by a2, moves a2 to d0, and branches to another routine.
- `check_indexed_table_entry_2`: Looks up a six-byte entry in a global table using the low word of its argument, passes that entry to a library call, and returns true when it finds a match.
- `setup_stack_and_load_pointer`: Sets up a stack frame, saves a2 and a3 to the stack, loads a pointer from a table using an offset from a4, and branches to another routine.
- `free_list_nodes_in_loop`: Repeats over a singly linked list held in a register, releasing each node with its stored size plus eight through a routine until the link is null, then clears the global list head.
- `manage_linked_list_node_2`: Calls another routine, and if the result is not zero, updates a global pointer and stores values into the returned structure.
- `forward_low_word_argument`: Passes the low 16 bits of its argument to a single unnamed routine and returns.
- `setup_pointer_and_branch`: Subtracts the value in a3 from itself, loads a pointer from a global variable into a2, and branches to another routine.
- `traverse_linked_list`: Compares a pointer in a6 with a2 and, if they are not equal, enters a loop that traverses a linked list by loading the next pointer from the address in a2 into a2 until a null pointer is encountered.
- `move_data_conditionally`: Moves the value from a2 to the location pointed to by a3 if a comparison of a3 and d3 does not result in zero, otherwise it branches to another location.
- `call_library_via_pointer`: Loads a pointer from a table using an offset from a4, moves a pointer from offset 4 of that structure into a6, pushes a2 to the stack, calls a routine at an offset from a6, cleans the stack, sets d0 to zero, and branches to another routine.
- `check_indexed_table_entry`: Looks up a six-byte entry in a global table using the low word of its argument, passes that entry to a library call, and returns true when the call returns a nonzero value.
- `call_library_and_store_result`: Calls two different library functions via a global base, then stores the result from the second call into a global variable at offset minus 30648 of a4 before returning.
- `set_zero_and_branch`: Sets d0 to zero and branches to another routine.
- `execute_conditional_callbacks_2`: Calculates an offset from a value in a5, checks if a pointer at that offset is non-zero, and if so, calls another routine with three arguments and potentially calls a second routine based on the result.
- `execute_conditional_callbacks`: Calls a routine and checks if the result has bit 12 set; if so, it checks a word at a negative offset of a4, and if that word is non-zero, it calls three separate routines in sequence before returning the result.
- `invoke_stored_handler_then_call`: Checks if a pointer stored at a negative offset of a4 is non-zero, calls that handler if it exists, then calls another routine and returns.
- `open_libraries_and_split_arguments`: Opens the dos and mathffp libraries and stores their bases in global variables.
- `do_nothing`: Performs a move of d0 to d0.
- `run_ten_steps_library_call`: Runs a ten-call sequence and optional cleanup as its neighbour, and through its callees it calls a library function.
- `set_d0_and_branch`: Sets d0 to 32767 and branches to another routine.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros and returns the destination pointer.
- `process_indexed_table_entry`: Checks if an index is within a specific range and if a table entry is non-zero, then either calls another routine if a bit is clear or clears the entry and returns zero.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a location relative to a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `dispatch_indirect_jump_6332`: Restores d1 and d2 from the stack, loads a pointer from an offset of 28676 from a pointer in a4 into a6, and performs an indirect jump to an offset of 102 from that pointer.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `dispatch_indirect_jump_636c`: Restores d1 and d2 from the stack, loads a pointer from an offset of 28676 from a pointer in a4 into a6, and performs an indirect jump to an offset of 84 from that pointer.
- `call_library_offset_30`: Performs an indirect jump to a library function at an offset of 30 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `dispatch_indirect_jump_6398`: Restores d1, d2, and d3 from the stack, loads a pointer from an offset of 28676 from a pointer in a4 into a6, and performs an indirect jump to an offset of 42 from that pointer.
- `dispatch_indirect_jump_63aa`: Restores d1 from the stack, loads a pointer from an offset of 28676 from a pointer in a4 into a6, and performs an indirect jump to an offset of 90 from that pointer.
- `call_library_offset_48`: Performs an indirect jump to a library function at an offset of 48 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_offset_414`: Performs an indirect jump to a library function at an offset of 414 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_record`: Calls a routine to find a free block, and if successful, initializes a structure at the returned address with values from a5 and a pointer from a sub-routine, then returns the address of the structure.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive and clears its counter field.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `update_record_state`: Calls a routine to find a free block, and if successful, sets a byte at offset 8 of the structure to 5, a word at offset 18 to a value from a5, and a long at offset 14 to a value from a5, then returns the address of the structure.
- `pop_and_dereference_pointer`: Pushes the value in a2 onto the stack, loads a new pointer from an offset of a5 into a2, and if that pointer is non-zero, loads the value at the stack pointer into a2 before returning.
- `initialize_structure_and_call_function`: Sets bytes at offset 8 of the structure in a2 to 255, sets 32-bit words at offsets 20 and 24 of that structure to all ones, pushes a word from offset 18 of the structure and the address in a2 onto the stack, calls a function at an offset from a global variable, cleans the stack, and branches to an earlier instruction.
- `call_library_offset_198`: Performs an indirect jump to a library function at an offset of 198 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_offset_294`: Performs an indirect jump to a library function at an offset of 294 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_offset_132`: Performs an indirect jump to a library function at an offset of 132 from a pointer stored at an offset of 28676 from a pointer in a4.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a library function using an address retrieved from a global variable and an offset.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `jump_to_library_function_2`: Performs an indirect jump to a library function using an address retrieved from a global variable and a different offset.
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

### Tron 5000 (1988)(Softgang).adf: 

- `parse_and_relocate_hunks`: Iterates through a linked list of memory hunks, identifies specific AmigaDOS hunk types, and performs relocation by updating pointers and adjusting memory offsets.
- `read_next_longword_from_stream`: Copies the longword at the current read pointer into a global variable and advances the pointer to the next entry.
- `process_list_and_execute_callbacks`: Calculates an index from a global table, calls a library function, and if the result is non-zero, stores it in a table; it then iterates through a linked list of structures, calling a routine for each non-zero entry and performing various library calls and memory operations.
- `close_library_and_permit`: Calls a library function to close a library if a global pointer is non-zero, calls another routine, and then calls a library function to re-enable multitasking before returning zero.
- `free_two_stored_buffers`: Frees each of two stored memory blocks through a library call when its pointer is nonzero.
- `scan_string_between_library_calls`: Calls a library function, measures a zero-terminated string from an address held in a register, then calls a second library function.
- `decompress_bitstream_to_buffer`: Reads a bitstream from a source pointer in a0, using a checksum to validate the data, and expands the compressed bits into a destination buffer in a1.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count held in a register, reloading the next longword from memory and shifting in a marker bit when the buffer runs out.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `branch_to_library_offset`: As library code, it performs a short branch to an offset of 80 decimal.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, it attempts to open and manage library resources to initialize a structure, potentially creating a linked list of entries if successful.
- `remove_node_from_list`: Library code that traverses a linked list from a pointer in a2, unlinks nodes where a 16-bit value at offset 84 is not greater than zero, calls a library function, and decrements a 16-bit counter at offset 64 of a structure in a6.
- `manage_linked_list_node`: Checks if two 16-bit values at offsets 58 and 40 of a structure are zero and less than one respectively; if so, it calls three external routines, performs pointer arithmetic on a buffer, and inserts a node into a doubly linked list.
- `set_bit_and_clear_d0`: As library code, it sets bit 7 of a byte at offset 14 from a pointer in a6 and clears d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `check_bits_and_jump`: As library code, it decrements a word at offset 38 from a6, branches if the result is zero, tests bit 7 of a byte at offset 14 from a6, and otherwise performs an indirect jump to an offset of -18 from a6.
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
- `increment_pointer_on_non_zero`: This library routine checks if the byte at the address in a0 is non-zero; if it is, it increments a0 and branches, otherwise it returns.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_bytes_in_tables`: This library routine compares a byte from a table at a0 with a byte from another table at a1, and then checks if the byte at a0 is non-zero.
- `compare_and_branch`: Loads a byte from a pointer in a0 into d0, decrements a1, and then compares the byte at the new pointer location with d0, branching if the value is less than or equal to d0 or setting d0 to a constant if it is greater.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_branch_2`: This library routine compares a byte from a pointer in a0 with a byte in d0, and if they are equal and d0 is not zero, it moves a0 to d0 and branches to another location; otherwise it returns.
- `increment_and_branch`: This library routine increments the value in a0 and branches to another location.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations with constants 47 and 58.
- `parse_font_metadata_and_copy_strings`: Calculates the remaining space in a destination buffer and, if a specific bit is set in a parameter, iterates through a linked list of font entries to copy string data into the buffer and populate a metadata array with pointers and length values.
- `decrement_font_buffer_pointer`: Decrements a destination pointer stored at a negative offset from a6 by a value in d4, then decrements a counter in d2 by the value in d4 plus 10.
- `copy_font_data_to_buffer`: Copies a sequence of bytes from a source pointer in a1 to a destination buffer pointed to by a value at a negative offset from a6, incrementing a counter in a4 and updating a structure at a2 with new metadata and a pointer to the next font entry.
- `parse_and_load_font_data`: Checks a bit in a register and, if set, opens a file, locks it, and examines its contents to locate font data.
- `update_font_buffer_offset`: Subtracts a 16-bit value from a pointer stored at offset negative 4 of the stack frame and updates that pointer, then decrements a remaining byte count in register d2 and branches to a copy loop if the count is non-negative.
- `copy_font_data_loop`: Copies bytes from a source buffer to a destination buffer pointed to by an offset in a stack parameter using a loop, then calls a routine to read a block of data and verifies that the read length is four and a specific constant is present in the stack.
- `read_and_parse_font_data`: Reads a block of data from a file and, if the read length matches a specific constant, parses the data into a structure by copying a 4-byte longword, a 2-byte word, and two bytes into a destination buffer.
- `execute_library_calls_and_negate`: This library routine calls three different routines, then checks a long integer in d2 and returns its negation if it is negative, otherwise it returns zero.
- `return_zero_constant`: The library routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Tron 5000 (1988)(Softgang).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_address`: Sets bit 0 of d0 and performs a long jump to a specific address.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
