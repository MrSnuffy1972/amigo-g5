# Game 914 (tron 5000 (1988)(softgang)[cr trilogy][m bfbs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00914). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907184 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 308 of 308 named (0 author symbols), 308 of 308 described (68 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 722 of 722 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9117 of 9117 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 487405 |
| code | 33284 |
| data | 133137 |
| padding | 11694 |
| unused | 241664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| mem | 2 | 117 | 2 |
| Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:hardware.default | 2 | 233 | 10 |
| Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:   | 3 | 6983 | 202 |
| Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:  | 3 | 421 | 9 |
| 8 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 67 |
| Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:bootblock | 1 | 64 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 467 | 16 |

## Cases handled

- runtime segment an address (play-long-joy): 388 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tron 5000 (1988)(softgang)[cr trilogy][m bfbs]/Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf: : a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### mem

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_and_call`: If the destination pointer is non-null, copies a fixed block of words from a base-relative area into it, then calls another routine through the global base.

### Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:hardware.default

- `load_trilogy_data_file`: Program entry for the hardware.default file: it forbids multitasking, opens the dos library, and then either clears a block of memory or opens, reads and closes the TRILOGY.1 data file through the dos calls.
- `copy_words_into_strided_table`: Copies thirty-two words from a source area into a table whose entries are four bytes apart, then calls library routines through the library base twice in sequence.
- `set_copper_list_pointers`: Uses the dos open, read and close calls and the exec library open and close calls, then stores the low and high halves of a series of addresses into a list and writes the copper list location and jump registers.
- `poll_frame_and_enable_dma`: Loops while a status bit is set, waiting on a counter and copying seven table entries, then touches the CIA port and the DMA control and vertical position registers.
- `expand_byte_table_stream`: Reads input bytes and, for each byte, uses an eight-byte entry of a lookup table in the global area to produce output bytes in a buffer, counting bit positions as it goes.
- `expand_byte_table_with_mask`: Reads input bytes and, for each byte, uses an eight-byte entry of a lookup table in the global area to produce output bytes, applying a bit mask at each step.
- `expand_byte_table_with_mask_2`: Reads input bytes and, for each byte, uses an eight-byte entry of a lookup table in the global area to produce output bytes, applying a bit mask at each step.
- `shift_bit_rows_left_one`: Shifts a series of rows, each ten longwords wide, left by one bit across the row, carrying the top bit of each following longword into the one before, and repeats for the row count.
- `shift_bit_rows_left_one_2`: Shifts a series of rows, each ten longwords wide, left by one bit across the row, carrying the top bit of each following longword into the one before, and repeats for the row count.
- `repack_bit_stream_into_words`: Walks a bit stream one bit at a time, shifting bits out of one longword sequence and into another, and writes a result word after every ten bits until the count runs out.

### Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:  

- `initialize_system_state`: Clears a block of memory using longword writes and updates a global pointer and a counter before transferring control to another routine.
- `call_stub_with_fixed_argument`: Calls a library stub with a base-relative address and the constant two, then returns.
- `initialize_graphics_and_load_assets`: Large initialization routine that opens the graphics library, refers to a Graphics Error message if the open fails, and sets up base-relative state.
- `update_counters_and_call_routine`: Increments two counters, sets a flag word, adjusts two values by fixed offsets, and passes them with constant arguments to another routine.
- `update_counters_and_call_routine_2`: Increments a counter from a stored value, sets a flag word, increments a second counter, passes two offset values with constant arguments to another routine, then calls it with a constant.
- `dispatch_mode_arguments`: Chooses one of two argument sets according to the input value and a stored flag word, calls a library stub with fixed coordinates, and passes the result to a second routine.
- `advance_positions_five_steps`: Runs a five-step loop that adds velocity values to two stored positions, clears a flag word when a position leaves a fixed range, and calls library stubs to test and update the position each step.
- `advance_position_until_bounds`: Repeatedly moves two stored positions by their velocity values for up to five steps, stopping early when a position leaves a fixed range, and calls library stubs on each step.
- `find_free_offset_near_point`: Checks a point through a library stub and, when the check fails, searches a small neighbourhood of offsets for one that passes, storing the offset that is found.
- `run_state_machine_step`: Runs a small state machine on a stored mode value: when a related flag is clear and the mode is non-zero, it runs the handler for mode one, two or three, resets the mode and calls other routines with stored positions.
- `place_flagged_markers_by_side`: Depending on the input value, checks one of two stored flags and, when it is set, computes a fixed-point offset and passes a marker position to another routine.
- `check_and_update_neighbour_positions`: Takes a mode value and a target position, sets a threshold count for modes one and two, then loops checking a library-tested value at the position and calling routines that update several neighbouring positions.
- `test_and_mark_position_pair`: Computes a pair of positions offset from stored coordinates, calls a library stub to test the first, and when the result is below a threshold issues two further stub calls on that pair.
- `execute_library_calls`: This routine performs several library function calls using offsets from a base address stored in a global variable.
- `draw_sequence_with_busy_waits`: Selects two coordinate values from a mode argument, then issues a series of drawing calls separated by busy-wait loops that count up to a fixed limit.
- `store_offset_rectangle_coords`: Stores four short coordinate values derived from stored positions with fixed offsets, then calls a library stub and two other routines to position a rectangle.
- `search_free_screen_position`: Searches candidate screen positions in nested loops over fixed ranges, testing each with a library stub and a fixed-point multiply until one passes the threshold checks, then calls further stubs at that position.
- `close_all_and_report_error`: Closes all open resources through the global base, passes its argument to a routine and then calls a final cleanup, with text for a close-all message and an error-number format.
- `wait_for_counter_band`: Busy-waits on a counter until it reaches a value band, then waits until it passes a second threshold before returning.
- `select_level_graphics_by_number`: Selects a level number from a stored value and, for each of twelve cases, runs the setup routines with fixed coordinates for the matching gfx/levelN file.
- `draw_brush_graphics_by_mode`: Draws a set of graphics elements at fixed positions and sizes using stored coordinates, branching on a mode value.
- `place_three_items_with_offsets`: Sets two state words to one, makes setup calls with fixed coordinates, then loops over three items, placing each at a computed position using a fixed-point multiply and a library stub.
- `call_four_element_routines`: Calls one routine and then four others with base-relative addresses of fixed elements, then returns.
- `advance_feedback_shift_state`: Shifts a 32-bit state word eight times, feeding back an exclusive-or of two low bits on each step, then stores the low word of the result.
- `update_bit_plane_entries`: Iterates over a table of short values, resetting entries below a threshold to a fixed value, clears bytes in up to three bitmap planes selected by each entry's index bits, and then reduces each entry by a stored step.
- `initialize_seventy_entry_tables`: Loops seventy times, filling three per-entry tables with values returned by other routines, offset by fixed amounts.
- `initialize_graphics_and_setup_display`: Attempts to open a graphics screen and validates the resolution and bitplane depth of a loaded image.
- `process_sixteen_row_columns`: Loops sixteen times over a number of columns set by a stored shift value, reads a value through two routines and passes it on with a column offset, then releases a stored handle through a library stub.
- `open_font_library_and_font`: Opens the diskfont library and then the font named in its data, reporting an error through the text messages if either open fails.
- `call_routine_return_zero`: Calls one other routine and returns zero.
- `initialize_tron_text_record`: Fills a small structure with fixed byte and pointer values, including a pointer to the text TRON, then calls a library stub with it and one other routine.
- `call_stub_with_constant_five`: Calls a library stub with the constant five and a base-relative address, then returns.
- `read_chip_register_with_flag`: Reads a 16-bit custom chip register word at an address indexed by its argument, then sets or clears bit four depending on a bit of a global value, and returns the adjusted word.
- `call_routine_return_zero_2`: Calls one other routine and returns zero.
- `initialize_sterne_text_record`: Fills a small structure with fixed byte and pointer values, including a pointer to the text Sterne, then calls a library stub with it and one other routine.
- `call_library_function`: Calls a function from a library loaded from a global variable.
- `fill_sixteen_by_sixteen_table`: Fills a sixteen-by-sixteen table by looping over both indices and storing a value read through two routines at each position, after an initial call.
- `fill_offset_sixteen_table`: Fills the same sixteen-by-sixteen table as its neighbour, but starts the row index at one and the column index at two, skipping the first row and the first two columns.
- `insert_value_into_sorted_table`: Searches a table of ten sorted entries for the insertion point of a value, shifts the lower entries down to make room, and stores the value with its fields, calling library stubs and the fixed-point multiply routine along the way.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `insert_value_into_sorted_table_2`: Searches a table of ten sorted entries for the insertion point of a value, shifts the lower entries down to make room, and stores the value with its fields.
- `insert_value_into_sorted_table_3`: Searches a table of ten sorted entries for the insertion point of a value, shifts the lower entries down to make room, and stores the value with its fields, calling library stubs and the fixed-point multiply routine along the way.
- `load_highscore_table_or_blank`: Fills a ten-row by eight-column text buffer with dash characters, then opens the df0:highscores file through the dos library and reads its data into the buffer, refilling it with dashes when the read is short.
- `load_highscore_table_or_blank_2`: Fills a ten-row by eight-column text buffer with dash characters, then opens the df0:highscores file through the dos library and reads its data into the buffer, refilling it with dashes when the read is short.
- `read_highscore_file_with_errors`: Fills a text buffer of highscore rows with dashes, reads the df0:highscores file through a dos library call, and shows a read-error message through the text if the read fails.
- `save_highscore_file_with_errors`: Opens the df0:highscores file for writing through the dos library, writes the highscore buffer, and shows a write-error or not-saved message through the text if the write fails.
- `wait_for_status_bit_set`: Repeatedly calls a routine with the argument two until a bit in its result is set, then returns.
- `format_score_digits_for_display`: Formats a score value into a six-character digit buffer padded with spaces, then passes the buffer to library routines at fixed positions, using the fixed-point multiply routine along the way.
- `format_signed_number_string`: Converts a number to a decimal digit string in a buffer, prefixing a minus sign for negative values and ending with a zero terminator, then passes the string to another routine.
- `reverse_string_in_place`: Reverses the bytes of a buffer in place, using the length returned by another routine to set the bounds.
- `draw_two_numeric_labels`: Formats two stored numbers into small text buffers and draws them with a sequence of library stub calls at fixed positions.
- `next_random_value_below_limit`: Advances a stored 32-bit state with a shift-and-exclusive-or step, then returns the result reduced modulo the limit argument; with a non-positive limit it only stores a new state.
- `load_ilbm_image_file`: Loads an IFF ILBM picture by opening the file through dos calls and parsing the chunks named BMHD, CMAP and BODY. It calls seven other routines, is called by four other routines, and reports failures through its error paths.
- `compare_four_byte_keys`: Compares the first four bytes of two buffers, returning zero when they all match and one otherwise.
- `release_and_format_sound_entry`: Releases a stored handle when one is held, then passes a second handle and a stored value to a routine and clears it, dispatching on the second argument with a special case for minus one and formatting the entry through the text format string.
- `fill_slot_values_from_stub`: Loops over a count of four-byte slots, fetching a value through a routine and a library stub for each, storing it at a fixed offset in the slot and calling a further routine when the value is zero.
- `release_nonzero_slot_handles`: Walks a slot array whose length is given by a count byte, releasing each non-zero pointer through another routine and clearing it to zero.
- `load_sound_sample_file`: Loads a sound sample file for a slot by allocating memory, opening the file, examining its size and reading it into memory, and reports an error message through the text if memory, opening, examining, existence or reading fails.
- `open_audio_device_and_slots`: Fills two arrays of five four-byte slots with values returned by other routines, then opens the audio device through the exec library and reports an error message through the text when the open fails.
- `configure_channel_slot`: If the channel's flag is set, writes fixed field values and the channel's stored parameters into a slot structure and passes it to another routine.
- `configure_channel_slot_pair`: If the channel and a shared flag are clear, writes field values into a slot structure, passes it to another routine, then writes a second configuration with position fields when a computed threshold is met.
- `call_eight_routines_in_sequence`: Calls eight other routines in sequence and returns.
- `call_routine_with_zero`: Calls another routine with a zero argument and returns.
- `execute_single_slot_config`: Performs a single configuration task via a slot structure.
- `call_routine_with_zero_twice`: Calls another routine twice with a zero argument and returns.
- `execute_double_slot_config`: Performs two consecutive configuration tasks via slot structures.
- `call_routine_with_zero_2`: Calls another routine with a zero argument and returns.
- `invoke_subroutine_via_pointer`: Loads a value from a memory location into a register and performs a branch to that address.
- `return_from_subroutine`: This routine performs a single return from a subroutine.
- `return_from_subroutine_2`: The routine performs a single return instruction to the caller.
- `update_channel_slot_data`: Writes field values into a slot structure and passes it to another routine.
- `initialize_slot_configuration`: Checks if a channel and a shared flag are inactive, then populates a slot structure with field values before passing it to another routine and writing a second value.
- `call_routine_zero_then_arg`: Calls another routine with a zero argument, then again with the constant sixty-four, and returns.
- `execute_callback_routine`: Calls a routine via a pointer stored in a register.
- `execute_mixed_slot_configs`: Performs one single configuration task followed by another configuration task with a specific parameter.
- `configure_channel_slot_2`: Writes fixed field values and stored parameters into a slot structure and passes it to another routine.
- `call_routine_zero_four_times`: Calls another routine with a zero argument four times in sequence and returns.
- `configure_slot_pair`: Writes field values into a slot structure and then writes a second set of values into another structure.
- `configure_slot_pair_alt`: Performs a dual slot configuration by writing values into two separate structures.
- `clear_longword_run_and_call`: Clears a fixed run of longwords at an offset from a stack-relative base, stores the stack address and a global value into two fields, then calls another routine.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `format_text_and_output`: Sets up a character output buffer and calls a formatting routine with a byte-output callback and the stack arguments, then passes the buffer start and length to an output routine, directly or through a global flag.
- `buffer_byte_flush_at_limit`: Stores one byte in an output buffer and, when the buffer reaches forty bytes, passes the buffer to an output routine and resets the pointer.
- `flush_output_buffer_remainder`: Passes the bytes still held in the output buffer to an output routine, resets the buffer pointer, and returns a masked byte from a stored structure.
- `examine_unrecovered_routine`: The decompiler could not produce pseudo-code for this 49-instruction routine, so its behaviour cannot be recovered from the evidence.
- `format_printf_style_string`: Walks a format string, copying literal characters through a callback and expanding each percent conversion into digits or text with width and sign handling, then returns the count of characters written.
- `perform_signed_division`: Computes the sign of the result from the signs of the two inputs, calls another routine to perform the division, and negates the quotient when the result should be negative.
- `perform_signed_remainder`: Computes the sign from the signs of the two inputs, calls another routine, and returns its second result, the remainder, negated when the sign is negative.
- `return_second_result_register`: Calls another routine and returns the value it leaves in D1.
- `perform_32_bit_division`: Divides an unsigned 32-bit value by another, using direct division when the divisor fits in 16 bits and a shift-and-subtract loop otherwise, and returns the quotient and remainder together in a 64-bit result.
- `read_until_count_or_sentinel`: Calls another routine repeatedly, counting up to the product of two packed word values, and returns the second word when the count is reached, or zero if the routine returns minus one first.
- `examine_unrecovered_routine_2`: The decompiler could not produce pseudo-code for this 22-instruction routine, so its behaviour cannot be recovered from the evidence.
- `sweep_records_in_range`: Repeatedly calls another routine on record addresses stepped by twenty-two bytes across a fixed base-relative range, then returns.
- `release_flagged_record_fields`: Calls release routines for the fields of a record whose flag bits are set, zeroes its pointer and flag fields, and returns a combined value, or minus one for a null record.
- `release_flagged_record_fields_2`: Calls release routines for the fields of a record whose flag bits are set, zeroes its pointer and flag fields, and returns a combined value.
- `buffered_write_byte`: Writes one byte into a buffered output record and returns that byte.
- `find_free_table_entry`: Scans a fixed table of 22-byte records for one whose flag byte is clear.
- `scan_free_record_slot`: Scans 22-byte records starting from an address held in a register until one has a clear flag byte.
- `clear_record_header_longs`: Writes zero to the first three longwords of the record addressed by a register and returns.
- `init_stream_buffer_pointer`: Probes a buffer source and, when the probe succeeds with a non-null pointer, stores a 1024 size with a buffered flag and that pointer.
- `release_all_list_nodes`: Walks a singly linked list from a global head pointer and passes each node, with its stored size plus eight, to a release routine.
- `free_list_nodes_in_loop`: Repeats over a singly linked list held in a register, releasing each node with its stored size plus eight through a routine until the link is null.
- `allocate_linked_list_node`: Allocates a block sized to the request plus eight bytes through a library call and pushes it onto the front of a global linked list with its owner address stored.
- `forward_low_word_argument`: Passes the low 16 bits of its argument to a single unnamed routine and returns.
- `unlink_and_release_list_node`: Searches a global linked list for the node whose data begins at the given address and unlinks it.
- `unlink_list_node_from_register`: Scans a linked list from a starting node in a register for the record whose data starts at the address in another register.
- `unlink_list_node_given_predecessor`: Unlinks a record from a singly linked list using a predecessor pointer in a register, treating a null predecessor as the list head.
- `set_list_head_to_node`: Sets the global list head to the record addressed by a register and then calls another routine.
- `check_indexed_table_entry`: Looks up a six-byte entry in a global table using the low word of its argument, passes that entry to a library call, and returns true when the call returns a nonzero value.
- `store_fallback_word_on_failure`: Calls a check routine with its argument and, when that check returns zero, fetches a value from another routine and stores its low word in a global.
- `return_zero_stub`: Does nothing and returns zero.
- `run_two_library_calls`: Makes two calls through library vectors and one call to another routine, in a body of 42 instructions.
- `test_status_bit_and_service`: Calls a routine with zero and 4096 and tests bit 4096 of the result.
- `invoke_stored_handler_then_call`: Calls the handler whose address is stored in a global when one is installed, then calls another routine and returns.
- `open_libraries_and_split_arguments`: Opens the dos and mathffp libraries through library calls and stores their bases in globals.
- `run_ten_steps_then_cleanup`: Calls a routine ten times in a row, then calls an optional handler and releases two globals when they are set.
- `run_ten_steps_library_call`: Runs the same ten-call sequence and optional cleanup as its neighbour, and through its callees it calls a library function.
- `append_string_bounded`: Appends a zero-terminated source string to the end of a destination string, copying at most 32766 bytes and terminating the result.
- `append_string_with_count_limit`: Appends a zero-terminated source string to the end of a destination string, copying no more than the count passed in, and terminates the result.
- `copy_string_until_terminator`: Copies bytes from a source pointer to a destination pointer up to and including the zero terminator.
- `measure_zero_terminated_length`: Walks a zero-terminated byte string and returns its length, not counting the terminator.
- `copy_string_with_zero_padding`: Copies a zero-terminated source into a destination for at most the given count of bytes, then fills any remaining part of that count with zero bytes.
- `release_indexed_table_slot`: Releases the block held in a numbered table slot when the index is in range and the slot is in use, clears the slot and returns zero.
- `call_library_function_36`: Loads a library base from a global variable and calls the library function at offset minus 36 from it.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_102`: Loads a library base from a global variable and calls the library function at offset minus 102 from it.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_function_84`: Loads a library base from a global variable and calls the library function at offset minus 84 from it.
- `call_library_function_30`: Loads a library base from a global variable and calls the library function at offset minus 30 from it.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_42`: Loads a library base from a global variable and calls the library function at offset minus 42 from it.
- `call_library_function_90`: Loads a library base from a global variable and calls the library function at offset minus 90 from it.
- `call_library_function_48`: Loads a library base from a global variable and calls the library function at offset minus 48 from it.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function_30_block`: Calls the library function at offset minus 30 from a base pointer stored in a field of the structure passed as its argument.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_414`: Loads a library base from a second global variable and calls the library function at offset minus 414 from it.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_init_record`: Obtains an index from a first routine and, when that succeeds, allocates a record through a library call.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive and clears its counter field.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `allocate_tagged_record`: When its first argument is nonzero, calls a routine to allocate a record using the second argument and a fixed flag.
- `mark_record_unused`: Sets a state byte and two index fields to a maximum value within a record.
- `mark_record_unused_in_register`: Marks the record addressed by a register as unused by setting its state byte and two index fields to minus one, then calls another routine.
- `call_primary_base_function_198`: Loads a library base from the primary global variable and calls the library function at offset minus 198 from it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_primary_base_function_294`: Loads a library base from the primary global variable and calls the library function at offset minus 294 from it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_primary_base_function_132`: Loads a library base from the primary global variable and calls the library function at offset minus 132 from it.
- `call_primary_132_alias`: Calls the same library function at offset minus 132 as its neighbouring stub, through the same library base.
- `call_primary_base_function_210`: Loads a library base from the primary global variable and calls the library function at offset minus 210 from it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `init_empty_list_header`: Initialises a list header in memory so that its head points past the header, its tail is cleared and its tail predecessor points back to the header.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_primary_base_function_552`: Loads a library base from the primary global variable and calls the library function at offset minus 552 from it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_second_base_function_30`: Loads a library base from the second global variable and calls the library function at offset minus 30 from it.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_second_base_function_552`: Loads a library base from the second global variable and calls the library function at offset minus 552 from it.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_second_base_function_414`: Loads a library base from the second global variable and calls the library function at offset minus 414 from it.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_second_base_function_198`: Loads a library base from the second global variable and calls the library function at offset minus 198 from it.
- `call_second_base_function_360`: Loads a library base from the second global variable and calls the library function at offset minus 360 from it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_second_base_function_216`: Loads a library base from the second global variable and calls the library function at offset minus 216 from it.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_second_base_function_210`: Loads a library base from the second global variable and calls the library function at offset minus 210 from it.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_second_base_function_306`: Loads a library base from the second global variable and calls the library function at offset minus 306 from it.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_second_base_function_354`: Loads a library base from the second global variable and calls the library function at offset minus 354 from it.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_second_base_function_60`: Loads a library base from the second global variable and calls the library function at offset minus 60 from it.
- `call_third_base_function_66`: Loads a library base from the third global variable and calls the library function at offset minus 66 from it.
- `call_third_base_function_198`: Loads a library base from a third global variable and calls the library function at offset minus 198 from it.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_second_base_function_384`: Loads a library base from the second global variable and calls the library function at offset minus 384 from it.
- `call_other_base_function_30`: Loads a library base from another global variable and calls the library function at offset minus 30 from it.

### Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf: 

- `load_executable_hunk_table`: Program entry that opens the dos library and walks the hunk table of an executable, allocating memory for its sections.
- `read_next_longword_from_stream`: Copies the longword at the current read pointer into a global variable and advances the pointer to the next entry.
- `allocate_block_into_table_slot`: Allocates a memory block through a library call and stores the pointer in the next free slot of a table, then returns it.
- `close_library_and_permit`: Calls one conditional library function when a flag is set, then calls another routine and a further library function, and returns zero.
- `free_two_stored_buffers`: Frees each of two stored memory blocks through a library call when its pointer is nonzero.
- `scan_string_between_library_calls`: Calls a library function, measures a zero-terminated string from an address held in a register, then calls a second library function.
- `decode_packed_bit_stream`: Decodes packed data by shifting bits out of a checksum-tracked bit buffer and copying literal bytes and repeated runs into an output area.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count held in a register, reloading the next longword from memory and shifting in a marker bit when the buffer runs out.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `obtain_block_and_store_value`: Library code that calls a library function to obtain a block, and when that succeeds stores the value passed in a register at offset 34 in the block before calling another library function.
- `obtain_and_tag_block`: Library code with the same logic as its neighbouring entry: it obtains a block through a library call, stores the value passed in a register in it and calls another library function.
- `open_dos_and_graphics_libraries`: Opens the dos and graphics libraries by name through library calls.
- `close_library_and_free_block`: Closes a library base and frees a memory block through two library calls, then returns zero.
- `unload_unused_list_entries`: Walks a list of entries and removes each one whose usage count is not positive, unlinking it and unloading its segment through the dos library.
- `release_unused_entry_or_mark`: When an entry has no users left, calls two library functions to close and free it, unlinks it from its list and returns the stored value.
- `mark_entry_pending_removal`: Sets the top flag bit in an entry's flag byte to mark it for removal and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_pending_flag_count_up`: Clears the top flag bit of an entry's flag byte and increments its usage counter.
- `decrement_count_release_if_flagged`: Decrements an entry's usage counter, and when the counter is still nonzero and the top flag bit is set, calls a library function and returns its result.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `scan_font_file_records`: Reads records from a font file through library calls, checks each header value against a fixed code and collects matching entries into an output list.
- `copy_font_data_blocks`: Copies successive byte blocks from a font data table into an output buffer, advancing the buffer and counters for each block.
- `copy_font_bytes_into_records`: Copies source bytes one at a time into a buffer and, when the buffer reaches its limit, writes a font entry record holding its size and position.
- `open_font_directory_and_files`: Opens the FONTS: directory and its .font files through dos library calls and reads their directory entries.
- `read_font_file_blocks`: Copies the bytes of each record into an output buffer and reads a four-byte header through a library call until the file block is used up.
- `read_font_file_entries`: Reads entry blocks from a font file through a library read call and checks each header against a fixed code.
- `read_font_entries_and_close`: Reads 260-byte entry blocks from a file through a read call, writes each accepted entry's header fields into an output table, and closes the file through a library call when finished.
- `unlock_and_restore_directory`: Releases a directory lock and restores the previous current directory through library calls.
- `return_zero_without_work`: Returns zero and performs no other work.
- `score_font_entry_match`: Computes a match score between two font entries from their size fields, then adjusts it by a per-bit weight table for each style bit that differs.
- `adjust_score_for_style_pair`: Adds or subtracts per-bit weights from a running score for each style bit that differs between two entries, working through the bits from the lowest up.
- `sum_style_bit_weights`: Walks seven bit positions of a style mask, subtracting a weight from the running total at each position where the mask applies.
- `sum_weights_from_second_mask`: Walks seven bit positions of a second style mask, subtracting a weight at each position where the mask applies.
- `return_constant_value`: This library routine returns a specific constant value.
- `select_best_matching_font`: Scores each candidate font entry against the requested entry and keeps the best-scoring one.
- `load_matching_font_entries`: Opens a font file, reads its header and entries through library calls, and keeps those that match the requested record in a list.
- `pick_best_candidate_index`: Scans a table of 260-byte records, scores each one against the request and records the index with the highest score.
- `load_best_candidate_segments`: Repeatedly chooses the best-scoring record and, for each accepted record, loads its segment through a library call inside a forbid and permit pair.
- `unload_font_records_and_segments`: Clears the flag word of each record in a table and unloads any loaded segments through library calls.
- `close_font_directory_and_release`: Clears the flag word of records in a table, then closes a file and unlocks a directory through library calls.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
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
- `copy_string_with_terminator_check`: Copies a zero-terminated byte string from a source to a destination, including the terminator.
- `append_string_to_end`: Moves to the terminating zero of a destination string and calls a copy routine with the source argument, so the source is appended.
- `forward_call_no_arguments`: Calls another routine with no arguments and returns, passing control on without other work.
- `count_bytes_until_nul`: Counts the bytes of a zero-terminated string and returns the count, not including the terminator.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `compare_strings_ordered`: Compares two zero-terminated strings byte by byte.
- `compare_last_byte_order`: Compares the byte before a reference pointer with the byte at a second pointer and returns minus one when the first is smaller, otherwise one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_in_string`: Searches a zero-terminated string for the given byte and returns a pointer to its first occurrence, or a null pointer when it is absent.
- `find_char_from_register`: Searches forward from the pointer in a register for the byte held in another register.
- `return_zero`: This library routine returns zero.
- `find_last_char_in_string`: Searches a zero-terminated string backwards for the given byte and returns a pointer to its last occurrence, or a null pointer when it is absent.
- `find_last_char_from_register`: Searches backwards from the pointer in a register for the byte held in another register.
- `return_zero_alt`: This library routine returns a zero value.
- `check_name_has_no_separators`: Looks for a slash and then a colon in a zero-terminated name.
- `return_void`: This library routine performs no operations and returns.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `dispatch_single_call_wrapper`: Invokes another routine and returns, with no arguments and no other work.
- `forward_to_second_routine_call`: Calls another routine and returns, with nothing else done.

### Tron 5000 (1988)(Softgang)[cr Trilogy][m BFBS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state_2`: Initializes a global pointer, populates a memory block with a repeating pattern, and clears a large block of memory to zero.
- `update_global_value`: Increments or decrements a global variable or resets it to zero based on its current value and a bitmask.
- `update_joystick_counter`: Checks a bitmask against a global variable and increments or decrements that variable or resets it to zero.
- `execute_indirect_calls`: Performs a series of calls to the same routine followed by an indirect call to a function pointer.
- `execute_recursive_calls`: Performs a variable number of recursive calls to itself based on the value of a global variable.
- `process_string_and_shift_bits`: Searches for a character in a string, copies a block of data from a source, and performs a bitwise left-shift and bit-insertion on a range of memory.
- `initialize_table_with_pattern`: Fills a block of memory with a repeating pattern of specific short integer values.
- `initialize_table_with_offset`: Fills a block of memory with a sequence of increasing short integer values while copying data from a source pointer.
- `copy_memory_blocks`: copies a block of memory using a word-sized loop and then copies a second, larger block using a word-sized loop with a stride of four bytes.
- `accumulate_values_to_tables`: Iterates through a block of memory and adds an offset to values stored in two different destination tables.
- `set_global_pointers_and_clear_memory`: Assigns several specific memory addresses to a global variable and then clears a large block of memory to zero.
- `reset_audio_volumes`: Writes zero to several global variables and then writes zero to the four audio channel volume registers, effectively silencing all audio channels.
- `reset_audio_dma`: Clears a global variable, writes a value to another global variable, writes to the audio volume registers, and writes to the DMACON register to enable DMA channels.
- `update_global_state`: Increments a global variable and, if it reaches a certain value, resets it and executes a sequence of recursive calls and conditional pointer updates.
- `modify_pointer_via_offset_and_table`: Checks a bitmask or value in a byte, then either adds or subtracts an offset or a nibble-extracted value from a pointer, or searches a lookup table to update a pointer-based memory location.
- `update_pointer_and_data_tables`: Reads a value from a pointer, calculates an offset, and updates several memory locations with data or calculated addresses from a lookup table.
