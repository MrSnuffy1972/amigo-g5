# Game 1738 (world class leader board (1988)(access)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01738). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (969440 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 356 of 356 named (0 author symbols), 356 of 356 described (35 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2457 of 2457 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2140 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18522 of 18526 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 760422 |
| code | 59848 |
| data | 118915 |
| padding | 12335 |
| unused | 17920 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Class Leader Board (1988)(Access)[a2].adf:Filter | 1 | 2 | 1 |
| World Class Leader Board (1988)(Access)[a2].adf:Golf | 7 | 233 | 7 |
| World Class Leader Board (1988)(Access)[a2].adf:nofastmem | 3 | 603 | 42 |
| World Class Leader Board (1988)(Access)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17675 | 303 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| HighScores (raw code file, load address unknown) | 1 | 0 | 0 |
| font (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 1834 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- world class leader board (1988)(access)[a2]/World Class Leader Board (1988)(Access)[a2].adf:WCLB/Data/HighScores: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- world class leader board (1988)(access)[a2]/World Class Leader Board (1988)(Access)[a2].adf:WCLB/Data/font: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- world class leader board (1988)(access)[a2]/World Class Leader Board (1988)(Access)[a2].adf:Golf: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 2140 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### World Class Leader Board (1988)(Access)[a2].adf:Filter

- `toggle_global_status_bit`: Flips the second bit of a global status word on each call and returns.

### World Class Leader Board (1988)(Access)[a2].adf:Golf

- `release_program_memory_blocks`: Runs a three-pass loop that calls a library routine stored behind a pointer, then calls the operating system's memory free routine through the exec library and clears a stored pointer slot.
- `decrement_until_minus_one`: Decrements a 16-bit counter until it reaches minus one and then returns, so it acts as a count-dependent busy delay.
- `accumulate_coded_values_into_table`: Runs three passes that read bit-coded values through a called routine and, for each code, adds a value into an output table at a running offset, updating the table in place.
- `depack_backward_bitstream_copy`: Decodes a bit-packed compressed stream: it reads bits to choose a length and distance from small code tables, then copies bytes from the stream into the output in reverse order.
- `copy_backward_literal_runs`: Continues the backward bit-stream depacker: copies literal bytes in reverse order from the compressed stream into the output, reading bit codes that set how many bytes each copy covers.
- `write_four_byte_groups_backward`: Writes output bytes four at a time while moving backward through the source, stopping when a count runs out, and returns the first longword it wrote.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### World Class Leader Board (1988)(Access)[a2].adf:nofastmem

- `initialise_library_state_block`: Clears 44 longwords of a state block at a computed offset, stores a base address register value and a global value into fixed fields of the block, then calls a routine.
- `call_library_branch_on_result`: Loads a library base from a global variable and calls a library function through it, then passes the result to one of two routines depending on whether it is zero.
- `free_linked_record_lists`: Walks up to ten bucket lists of linked records, releasing each record through a library call with its size, then releases the parent block and two further blocks through library calls and returns the last result.
- `allocate_and_link_state_blocks`: Allocates two blocks through library calls, sets fixed codes, sizes and flags in their fields, links the second block in, then walks a list of records and clears a flag on those that have it set.
- `initialize_dos_and_clear_memory`: Initializes a DOS library handle and clears a block of memory using a loop.
- `stub_no_operation_return`: The routine body is only a return with no work, so it is a no-op stub.
- `initialise_dos_library_handle`: Looks up the dos library and, if it is found, allocates a block, writes a four-character tag and a handle into it and calls library routines to set it up.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `close_library_resources_at_exit`: Runs a cleanup for ten indexed slots, then calls a stored close function and releases up to three stored resource pointers through library calls.
- `append_string_with_cap`: Appends a zero-terminated source string to the end of a destination string, copying at most roughly 32 thousand characters, then terminates the result and returns the destination.
- `append_string_with_limit`: Appends a zero-terminated source string to the destination string, copying no more characters than a length argument allows, then terminates the result and returns the destination.
- `copy_zero_terminated_string`: Copies a zero-terminated source string into the destination, including its terminator, and returns the destination pointer.
- `measure_string_length_2`: Walks a zero-terminated string to find its terminator and returns its length, not counting the terminator.
- `copy_string_with_zero_padding`: Copies up to a given number of bytes from a source string, stopping at its terminator, then fills the rest of the count with zero bytes, and returns the destination.
- `release_indexed_table_slot`: Checks that a slot index lies between 0 and 19 and that the slot is in use; otherwise it stores an error code and returns minus one.
- `multiply_two_longwords_to_64`: Multiplies two 32-bit values taken from registers by splitting each into 16-bit halves, summing the partial products, and returns the 64-bit result across a register pair.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_function_via_base`: Loads a library base from a global variable and calls the library function at a fixed negative offset from that base, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Performs an indirect call to a library function using an offset from a base address.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_via_offset`: Loads a library base from a global variable and executes a function at a specific offset within that library.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `dereference_indirect_pointer`: Loads a pointer from a location relative to a register, performs a second level of indirection, and executes the resulting address.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### World Class Leader Board (1988)(Access)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_polling_main_loop`: Stores a routine result in a global slot, then loops while a polled routine returns nonzero, calling library routines on each pass and extra routines on passes where a flag is clear.
- `load_palette_and_setup_display`: Copies 32 colour entries from a stored table into a local buffer, loads a data file named in the program through a library call, saves the loaded colours back into the table, then sets up a 320 by 200 display and applies the palette.
- `set_full_screen_area`: Calls one library routine with a zero argument, then sets the full 320 by 200 screen rectangle from the origin through a second library call.
- `initialise_state_tables_and_flags`: Sets several state flags and counters to fixed starting values, copies 13 bytes and then 14 bytes from two source tables into a working array, and copies a 10 by 4 block of source bytes into a second working table.
- `reset_session_tables_and_state`: Clears a counter, copies 13 bytes and then 12 bytes from two source tables into a working array, sets a state value, then calls a library routine and branches on its result to set up further state.
- `prepare_selection_state_tables`: Sets flags and a counter, copies 23 bytes from a source table into a working array, and calls a library routine.
- `run_player_setup_dialog`: Shows a setup screen whose on-screen text asks for the level of play, the number of players, player levels and names, and the number of holes.
- `draw_selection_menu_screen`: Sets a large timeout value and clears two state words, then draws a frame, a six-entry list at fixed spacing through library calls and a prompt reading Select One Please.
- `draw_frame_and_message_box`: Sets drawing state, draws a rectangle from x 65 to 255 and y 76 to 174 through a library call, then draws a message at position 136, 173 with a second library call and returns.
- `execute_library_sequence`: Executes a sequence of five library calls with various arguments and constants.
- `draw_cancel_button_frame`: Sets drawing state, draws the same frame rectangle and then a message at the same position; the referenced text is the label Cancel, so it draws a Cancel button.
- `read_changed_input_byte`: Compares a stored input byte with the current value of a hardware input register; it returns zero when the value is unchanged, otherwise it stores the new value and returns it.
- `swap_structure_pointer_slot`: Exchanges the 32-bit value in a fixed field of a structure with a global slot, so each call swaps the two stored values.
- `copy_graphic_rows_to_display`: Calls a routine, then copies a 192-pixel-wide strip for each row from 75 to 175 between two buffers at a fixed x offset through a library call, pausing after each row.
- `run_high_score_menu`: Counts the filled entries of a score table, then draws the menu labels Best Scores, Print Scores and QUIT and handles the menu choice through library calls.
- `run_input_update_loop`: Initialises several state fields, then runs an input loop that reads the pointer position and button state, updates a cursor value within limits, and calls library routines to respond to changes, until the loop state changes.
- `reset_cursor_position_from_state`: Sets the cursor's x and y from stored start values, or from lookup values indexed by a stored byte when a flag is set, clears a state byte, then calls a library routine.
- `draw_cursor_marker_at_position`: Draws a 7 by 7 marker at the stored cursor position through one library call, using a fixed column of 235 and a size argument of 192.
- `animate_cursor_to_target`: Moves the cursor position one step at a time toward a stored target x and y until both match, drawing each step through library calls.
- `draw_position_crosshair`: Draws a block at the stored column in a fixed row, then draws four short tick marks around that column, two above and two below, through library drawing calls.
- `draw_marker_block_at_column`: Draws a block of fixed size two pixels left of the stored column, in the fixed row 131, through a library call.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `compute_squared_distance_to_point`: Computes the squared distance between the stored position and one of two stored points, chosen by an index argument, and passes that value to a called routine.
- `fill_cell_grid_from_library`: Clears several state fields, then fills a grid of four by seventeen cells, calling a library routine for each cell and storing its result in the cell.
- `advance_timed_sequence_state`: Stores a state value and calls a routine, then loops through a sequence table of sixteen-byte entries, advancing the pointer until an end marker and calling library routines to act on each entry.
- `store_sums_of_library_results`: Calls a library routine several times and stores the sums of the returned values, plus a stored offset, into two fields of a record.
- `add_positioned_list_entry`: Calls library routines to obtain two values, stores their offsets as the x and y position in the next free sixteen-byte list record, and sets a further field from a stored value.
- `draw_paired_points_loop`: Loops a stored number of times; each pass takes two library values, offsets them into a screen point, and draws a pair of points on either side of it through a routine, with a short wait after each pass.
- `draw_record_span_and_marker`: For the current list record, when its position lies inside the screen bounds, draws paired points across a span set by two library values, then draws a marker at the record's position, with short waits between the steps.
- `draw_vertical_marker_lines`: Draws short vertical line pairs at fixed columns 37 and 38 and fixed rows near 173, with the far end offset by a value from a library call, then draws a further pair scaled by a stored value.
- `read_table_byte_at_index`: Uses the low byte of a stored word as an index into a byte table, stores that index in a separate word and writes the table byte into the low byte of the stored word.
- `read_table_byte_pair`: Reads the table byte at the stored index and the byte after it, advances the stored index by one, and returns the two bytes as a pair.
- `draw_view_with_here_label`: Sets a flag, then blits a 200-row view that is 224 or 320 columns wide, chosen by a stored flag, through a library call, and draws overlay items including the text Here.
- `draw_offset_value_and_wait`: Reads a stored value, draws it offset by 112 through a routine, then loops on a stored state until it clears, calling library routines to draw and wait, and draws a further element when a second flag is clear.
- `decode_packed_bitplane_stream`: Calls a library routine with a 200-row by 40-byte size, then decodes a packed bitmap stream into a destination plane, writing bytes down columns with a row stride of 40 bytes.
- `initialise_view_rectangle_and_state`: Sets up a view rectangle from column 32 to 224 down to row 199 through library calls, clears several state words and runs a further sequence of library calls to initialise the view.
- `process_entity_records_by_type`: Walks a count-limited table of 104-byte records in two passes.
- `dispatch_entity_by_type`: Reads the type value of the current entity record and branches on it, with specific handling for types 2, 6 and 18 and library calls for each branch.
- `build_entity_position_lists`: For the current entity record, loops over a stored count and builds two local lists of 108 longwords each, calling four routines to fill and process them.
- `populate_twelve_records_from_library`: Runs twelve passes, each storing two library values and a zero field in a state record, then writes a 12-byte entry into a table and calls a routine.
- `copy_pointer_field_to_state`: Copies a longword from a fixed field of a record reached through a stored pointer into a state field.
- `sync_counter_and_update_state`: When a stored word differs from a stored byte, calls a routine and, if it succeeds, updates the stored state and makes further library calls; otherwise it returns without change.
- `load_clamped_record_size`: Stores the low two bits of a record word, computes a pointer from a record offset, and clamps the value read through it to at most 195 before storing it in a state word, then continues with further processing.
- `convert_point_records_with_offset`: Copies three-word source records into six-byte destination entries, subtracting fixed offsets of 6160 and 1300 from the first two values and copying the third unchanged, repeated for each record.
- `sort_records_by_second_word`: Sorts six-byte records into ascending order by their second word, using a comparison and swap loop over the table.
- `fill_entity_points_from_stream`: Reads word pairs from a source stream, converts each pair by subtracting fixed offsets of 6160 and 1300, and stores them into 104-byte entity records, advancing the stream pointer as it goes.
- `read_entity_header_from_stream`: Reads a longword reference and two converted words from a stream, then two single bytes, into a state record, advancing the stream pointer past each field.
- `load_numbered_resource_block`: Calls a library routine with a buffer and a numeric identifier and, if it succeeds, loads and processes the returned block through further library calls.
- `check_signature_then_finish`: Calls a library routine, then compares a stored longword with a fixed signature value.
- `run_library_call_sequence`: Makes a fixed sequence of five library calls, the last of which returns a value, then returns.
- `run_three_library_calls`: Makes three library calls in order and returns.
- `filter_twelve_byte_records`: Walks a count-prefixed table of 12-byte records and compares one longword with the negated next longword, keeping or moving the records that pass and writing the rebuilt list through a second pointer.
- `copy_selected_twelve_byte_records`: Walks a count-prefixed table of 12-byte records and copies those chosen by a comparison into an output list, counting them, with a library call for each copy.
- `copy_records_with_library_calls`: Loops through records, making a library call for each step and copying field values between record arrays until a stop condition holds.
- `rebuild_twelve_byte_record_list`: Rebuilds a count-prefixed list of 12-byte records by comparing longword fields of neighbouring records and writing the result through a second pointer, making library calls along the way.
- `open_ciaa_resource_and_hook`: Stores a control value for the CIA A control register and a handler pointer in a structure, then opens the ciaa.resource through a library call and calls a failure routine if that open fails.
- `store_control_byte_and_signal`: Stores a byte from a structure into a global variable, then makes a library call with the stored resource, a zero argument and a structure field.
- `service_interrupt_request_flags`: Checks an interrupt request register through a library call and, when a request is pending, updates a global bit mask and state words according to the flags, then continues with the remaining request checks.
- `service_second_request_flag`: Sets a global bit flag and a state word, then when a second flag is set and a second bit mask is present, toggles a state word and clears a pending word.
- `service_third_request_flag`: Sets a global bit flag and a state word, then when a third flag is set and its bit mask is present, toggles a state word and clears a pending word, with DMA and interrupt request register activity.
- `service_fourth_request_flag`: Sets a global bit flag and a state word, then when a fourth flag is set and its bit mask is present, toggles a state word and clears a pending word, with DMA and interrupt request register activity.
- `set_final_request_mask`: Sets the last request bit mask in a global variable and marks its state word as pending, then returns.
- `reset_state_when_marked_two`: Clears two longword fields, and when the state byte for the current index equals two, makes library calls and stores a value taken from a stored slot.
- `set_position_pair_with_offset`: Stores two library-derived values as a position pair, offsetting one x value by 112, and stores a second pair through further library calls.
- `set_position_pair_from_library`: Stores a library-derived value in a state field, then offsets a second value by 112 and stores a further library value, setting a position pair.
- `update_position_values`: Updates three consecutive memory locations by calling library functions and storing the results.
- `update_coordinate_values`: Fetches values from two different function pointers and stores them into consecutive memory locations.
- `compute_indexed_state_value`: Reads two bytes for the current index; when the state byte is zero it clears a longword field, otherwise it makes library calls and stores a value computed from the second byte.
- `set_state_when_index_small`: Stores two library-derived values into state fields, and when a stored index is below three and a state field is nonzero, makes a further library call.
- `accumulate_library_value_into_state`: Adds two library-derived values together and adds the total into a stored longword state field.
- `read_mouse_movement_delta`: Reads the mouse port movement counter, subtracts the previously stored counter value, limits the difference to a signed byte range and returns the movement delta.
- `draw_animated_entity_frame`: Reads two library values into state, selects a frame and offset from position thresholds of 153 and 360, and draws the entity through library calls.
- `select_entity_graphic_by_type`: Chooses a graphic index from the current record's type byte: type 2 uses 26, type 6 uses 6, types 18, 19, 20 and 22 pass their own value, and any other type uses 11.
- `draw_entity_detail_block`: Reads a state entry for the current index, sets a drawing mode through a library call, fills a local table of fields from state, and then issues further drawing calls for the entity.
- `draw_entity_marker_by_state`: When a flag is clear, selects a value from the current state byte, chooses a position value for each case, and draws the marker through library calls.
- `check_zero_state_and_update`: When two stored state words are both zero and a flag is clear, it sets an index value and continues with further library-driven state updates.
- `draw_shot_setup_panel`: Draws a setup panel whose labels include Club, Power, Hook, Slice, Steepness, Wind, View, Feet, Hole and Driving Range, using eight library calls for the text and values.
- `draw_text_advancing_seven_pixels`: Draws each character of a string in turn through a library call, advancing the x position by seven per character, and stops at the length returned by another library call.
- `draw_load_save_abort_menu`: Draws a menu bar with the labels ABORT, LOAD and SAVE, using library calls to set the drawing mode, fill the box and place the text.
- `open_graphics_and_intuition_libraries`: Opens the graphics and intuition libraries through library calls, stores the returned base pointers in global fields and returns a result that shows whether the first library opened.
- `allocate_working_memory_block`: Stores a large size constant, allocates a memory block of that size through a library call with a flags argument, stores the returned pointer, clears related fields and then makes further setup calls.
- `handle_status_flag_events`: Checks bits of a stored status word and, for each set bit, makes library calls on stored handles to handle the matching event, then continues through the remaining checks.
- `call_handle_with_fixed_arguments`: Makes one library call on a stored handle with a buffer address, the values 2 and 16 and two zero arguments, then returns.
- `draw_abort_or_next_dialog`: Chooses a dialog variant from a numeric argument and draws its text, including Abort Game, Next Hole and OK options, through library calls.
- `draw_leader_board_screen`: Clears several score-related fields and draws the leader board screen, labelled LEADER BOARD and Round, through nine library calls.
- `draw_row_frame_outline`: Draws a rectangle outline at fixed columns 37 and 282 relative to a row argument, with its top and bottom edges offset by 8, 28 and 38, through library calls.
- `draw_player_score_row`: Draws one player's score row: the name at a fixed column, a score or dash mark chosen by a stored value, a second value at a fixed column and a loop of 18 cells, all through library calls.
- `save_high_scores_to_disk`: Builds a file name from a stored name and the suffix HS, writes the high score table to disk through library calls, and shows a Disk Error message if the write fails.
- `draw_scorecard_header`: Draws the scorecard header with the text ACCESS SOFTWARE INCORPORATED, WORLD CLASS LEADER BOARD SCORECARD and PAR, through six library calls.
- `pass_callback_to_routine`: Passes its argument together with a reference to a second routine to a called routine, which can then invoke it as a callback.
- `copy_longword_into_stack_slots`: Fills several consecutive stack slots with the same longword taken from a fixed offset, then continues with further stack setup and calls.
- `lookup_and_pass_value`: Calls a library routine with two arguments, then passes the returned value and a constant one to a called routine.
- `format_signed_number_text`: Converts a signed number into decimal text in a buffer, writing a minus sign for negative values and converting the digits through a called routine.
- `format_decimal_number_text`: Writes a minus sign for values below minus nine into a text buffer, then converts the number into decimal digits.
- `animate_marker_sweep`: Runs a timed animation loop that draws marker pairs at columns 249 and 251 with a sweeping row value, first advancing it and then holding, until a counter reaches nine and a flag is set.
- `load_font_data_block`: Opens the font data file named in the program through a library call, reads a 1440-byte block into a buffer through another library call, and releases the handle.
- `draw_formatted_text_line`: Formats a value into an 80-byte stack buffer, draws the buffer at the given position through a library call, then passes the buffer on for a further call.
- `draw_text_within_width`: Formats a value into an 80-byte stack buffer and, when its length is below a width argument, draws it at a position through library calls, passing the buffer on for a further call.
- `draw_clipped_text_line`: Formats a value into an 80-byte stack buffer and, when it fits within a width argument, draws it at a position through library calls, passing the buffer on to a further call.
- `measure_and_draw_string`: Sets the drawing position through a library call, measures the string argument through another library call, and draws the string with its measured length through a called routine.
- `convert_number_to_digit_buffer`: Converts an integer into decimal digit characters in a buffer, adds a minus sign when the input is negative, terminates the string and then passes the text to a called routine.
- `reverse_string_in_place`: Measures a string through a library call, then swaps its characters from both ends toward the middle to reverse it in place.
- `integer_square_root_by_sums`: Computes the integer square root of its argument by adding consecutive odd numbers until the running total passes the value, counting the steps.
- `forward_with_stored_argument`: Forwards its three arguments to a called routine together with a fourth argument taken from a stored longword.
- `pass_pair_to_handle_one`: Passes a pair of arguments to a library routine on a stored handle, using one library function, then returns.
- `pass_pair_to_handle_two`: Passes a pair of arguments to a library routine on a stored handle, using a second library function, then returns.
- `pass_pair_to_handle_three`: Passes a pair of arguments to a library routine on a stored handle, using a third library function, then returns.
- `forward_pairs_to_routine`: Forwards its four arguments to a called routine as two pairs, making one call per pair.
- `redraw_page_when_index_changes`: When a stored index differs from the last drawn index, updates position fields, labels the page with the index plus one, makes library blit calls for the page area and draws the entries of each row through a loop.
- `draw_block_rows_in_column`: Loops down a column of rows below 199, drawing a block of stored size through a library call for each row and recording a state value per row, across four passes.
- `setup_sprite_from_index_table`: Looks up a size and two offset values from tables using an index byte, copies them into position fields and loops over four parts to set up the entity's drawing data through library calls.
- `draw_clipped_entity_parts`: Derives a draw mode from bits of a state word, clips the entity's rectangle to the screen bounds of 224 columns and 199 rows, and draws its parts through library calls.
- `record_and_clip_entity_bounds`: Stores an entity's position, size and offsets in the next sixteen-byte list record, checks them against the screen edges of 224 columns and 199 rows, and draws the entity through library calls when it is visible.
- `scan_entity_list_for_match`: Walks the sixteen-byte entity list for an entry matching the current state, then draws runs of entity rows through library calls, repeating until the list is exhausted, and returns a count.
- `draw_entity_runs_until_empty`: Loops over runs of entity rows, reading library values and drawing each run that lies within the screen bounds, and returns a count when the list ends.
- `draw_entity_rows_with_checks`: Loops over rows of entity records, reading library values and drawing each row only when it lies within the screen bounds, and returns a count when the record list ends.
- `draw_entity_bitmask_columns`: Scans the entity list for an entry overlapping the current column window, builds eight-pixel bit masks from a table and draws the entity's column bytes through library calls, returning a status code.
- `draw_entity_bitmask_rows`: Scans the entity list and draws table-driven bit masks for a range of rows through library calls, returning a status code.
- `draw_entity_bitmask_strip`: Repeats the entity list scan and table-driven bit-mask drawing for a strip of rows, calling library routines at each step and returning a status code.
- `scan_entity_bitmask_block`: Scans the entity list with the same bit-mask pattern as its neighbours and draws a block of table-driven masks through a library call, returning a status code.
- `scan_entity_bitmask_tiles`: Scans the entity list and draws tile-sized bit-mask groups through a library call for each group, returning a status code.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `integrate_motion_until_done`: Runs a per-step loop that adds velocity-like values into position fields, applies a fixed decrement each step, clamps negative values to zero and draws the result through library calls until a flag ends the loop.
- `run_motion_frame_loop`: Runs a loop over animation frames, advancing a counter and drawing overlay items through library calls, marking progress in a state word and stopping when the wait counter reaches zero.
- `run_marker_update_loop`: Runs a per-step loop that draws marker elements at fixed columns once a position reaches 187, updates a counter and resets a state word, using library calls throughout.
- `run_marker_column_loop`: Runs a per-step loop that draws marker pairs at columns 230, 234 and 238 when a position reaches a limit, with a state-controlled wait, calling library routines for each marker.
- `run_marker_selection_loop`: Runs a loop that draws marker pairs at the same columns and checks four selectable items by index, calling library routines to draw each item and update its state.
- `run_marker_selection_timed_loop`: Runs a loop like its neighbours that draws marker columns and selectable items through library calls, with a timed wait and a set of library calls that update the display state.
- `run_meter_update_loop`: Runs a per-step loop that updates a meter position, redraws its marker columns through library calls and tracks state flags, ending when the step count passes its limit.
- `run_meter_input_loop`: Runs a loop that reads state flags and draws a meter with four selectable settings through library calls, updating stored values at each step.
- `run_meter_display_loop`: Runs a loop that draws the meter and its status with library calls, stepping a counter and updating stored state until the loop condition clears.
- `dispatch_input_event_code`: Counts down a delay, then dispatches on an event code byte: codes 9, 11, 16 and 17 each trigger a different library action, and a menu check through a library call looks for the value six.
- `run_meter_redraw_loop`: Runs a per-step loop that redraws marker columns at fixed positions when the position reaches 187, then processes event codes through library calls, using the same dispatch as its neighbours.
- `run_event_code_loop`: Runs a per-step loop that redraws two marker columns and dispatches event codes through library calls, including a check that returns the value six.
- `run_state_event_loop`: Runs a state-driven event loop: state value 2 reads a clamped value from the input source and updates the display, and other state values select further handlers.
- `run_state_event_loop_a`: Runs a state-driven event loop starting from a fixed position, handling state values through input reads and library drawing calls, with the same handler structure as its neighbours.
- `run_state_event_loop_b`: Runs a state-driven event loop starting from a second fixed position, using the same handler structure and library drawing calls as its neighbours.
- `run_option_event_loop`: Runs a loop that waits for a delay, then handles option events by index, drawing each of four options through library calls.
- `run_option_event_loop_b`: Runs an option event loop like its neighbour, with a different starting position and library calls for each of two drawn options.
- `run_option_event_loop_c`: Runs an option event loop that makes library calls for each step and draws its selected option, with an event code check and a stored state update.
- `run_frame_state_dispatch`: Runs one frame of a state machine: when the state word is nonzero it dispatches on the values 2, 4 and 5 to three handler paths, and otherwise adds two readings to a per-record position, clamps it against 27000 and redraws the record.
- `run_frame_state_dispatch_flagged`: Runs the same state machine as the frame routine with a non-zero state word, but first calls a helper with argument 3 and a no-argument helper when a flag byte is set.
- `run_idle_motion_and_timer`: Loops while the state word is zero, integrating the velocity pair with a constant -21000 step and clamping one accumulator at zero.
- `dispatch_event_code_then_frame`: Switches on an incoming event code: codes 9 and 11 set counters and a flag, code 16 clears the state word, codes 17 and 18 run a dialog check that sets a flag or triggers a helper, and code 26 clears the loop-exit flag.
- `step_state_dispatch_once`: Performs a single pass of the state dispatch without looping: values 2, 4 and 5 call their handlers, and any other nonzero value redraws the current record after adding two readings to its position and clamping it against 27000.
- `update_record_position_clamped`: Adds a sum and a difference of two input readings to the current record's position fields, calls a draw helper at the record index, and reverts that step when any of the three stored values exceeds 27000.
- `store_clamped_state_value`: Calls a value source, stores its result in a state word clamped to the range 0 to 3, calls a helper on the state block, runs the shared update routine, and redraws the record when a flag is zero.
- `cap_state_value_at_three`: Caps a state word at 3 when it exceeds that value, calls a helper, runs the shared update routine, and redraws the record when a flag is zero.
- `run_optional_redraw_then_helper`: Redraws the record only when a flag is zero, then calls a helper on the state block.
- `copy_seed_pair_into_entry`: Redraws the record and runs a helper with argument 7, copies two stored seed words into the current entry of a table, calls a helper with argument 300 and then redraws the record.
- `dispatch_state_code_once`: Performs one pass of the state dispatch selected by an event code argument: codes 2, 4 and 5 call their handlers, and any other code redraws the current record after adding two readings to its position and clamping it against 27000.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `run_indexed_helper_sequence`: When a counter exceeds 9, sets a fixed value in one record field, marks a flag, stores 64 minus a value-source reading in a second field, refreshes the record, calls a helper for indices 1 to 4 with argument 5 after each call, and finishes with a helper call with argument 7.
- `resolve_terrain_collision_step`: Looks up the code at the current grid position.
- `set_state_flag_and_return`: Sets a byte flag in the state block to 1 and returns zero.
- `apply_terrain_bounce_response`: Classifies the terrain status of the current step and applies a bounce: a small status restarts the velocity terms from a value-source angle, while a larger status reverses and halves the velocities once when a flag is zero.
- `reverse_and_damp_velocity`: When a flag is zero, negates one velocity term, halves the other two, stores 64 minus a value-source reading in a field, refreshes a record and sets the flag.
- `flag_or_bounce_by_code`: Sets a flag when the event code is one of a fixed set of values; for any other code it applies the bounce response and returns zero.
- `return_zero_immediately`: Returns the value zero without doing any work.
- `classify_terrain_at_position`: Returns a status code for the current grid position: 3 when the coordinates are out of range, 5 when the cell matches a stored target, 2 or 4 for special cells, and 1 when a cell code triggers a flag and a helper call.
- `classify_cell_then_reload`: Returns 5 when the cell matches a stored target, otherwise looks up the cell code and returns 2, 4 or 1 for special codes after setting a flag and a helper call.
- `classify_lookup_then_reload`: Looks up the cell code at the current position and maps it to a status of 2, 4 or 1 for special codes, with a flag set and a helper call for one case.
- `set_flag_and_return_one`: Sets a byte flag in the state block, calls a helper with argument 7 and returns one.
- `reload_velocity_within_bounds`: Reloads the velocity terms from a value source, optionally applies a secondary adjustment, sets a flag and returns 1 when the velocity terms stay within the bounds of 65527, and returns 0 otherwise.
- `classify_event_code_then_reload`: Maps the incoming event code to a status: 2 and 26 return 2, codes 3 and 6 set a flag and return 1 after a helper call, and code 5 scans forward along the column for a non-5 cell to return 4.
- `reload_velocity_terms_checked`: Reloads the velocity terms from a value source, optionally applies a secondary adjustment, sets a flag and returns 1 when the terms stay within the bounds of 65527, and returns 0 otherwise.
- `return_zero_status`: Does no work and returns zero.
- `copy_longword_block`: Copies longwords from a source buffer to a destination, reducing a byte count by four per step until the count reaches zero or drops to three or less.
- `clear_longword_buffer`: Zeroes 800 consecutive longwords starting at a buffer pointer kept in the state block.
- `draw_region_at_offset`: Computes a draw position by offsetting the current position fields by fixed amounts, stores a value scaled by 5 from its argument, and calls a graphics helper with a 225-wide region and parameters 5, 8, 16 and 192.
- `draw_region_at_stored_position`: Calls a graphics helper with a 225-wide region and parameters 5, 8, 16 and 192, using the position values stored in the state block.
- `plot_packed_bitmap_rows`: Selects a 16-row bitmap from a table by index and, for each row, plots every set bit as a point offset by the stored position, then calls a finishing helper.
- `initialize_position_and_draw`: Stores the start coordinates 111 and 188 with a second value of 188, sets a flag, draws two bitmap regions from the same position and calls a clear and a fixed-rectangle helper.
- `step_and_draw_object_sprite`: Moves the object's position by the difference to the input point and by a value-source step, resets the position to the input when it leaves the expected range, then draws its sprite and shadow regions clipped to fixed bounds and redraws the pieces that move with it.
- `clamp_value_plus_minus_ten`: Clamps a 16-bit value in place to the range from minus 10 to 10.
- `seed_start_position`: Sets a starting position by adding two scaled value-source readings to base offsets, copies a stored frame value into the timer field, and seeds a third coordinate from another value-source reading.
- `call_helper_if_nonzero`: Calls a value source and, when the result is nonzero, passes the two arguments and that result to a helper and returns the helper's value; otherwise it returns zero.
- `find_table_entry_by_name`: Scans an eight-byte-stride table with the string comparison helper for a matching name; when the table is exhausted it sets an error code of 6 and returns zero.
- `clear_large_longword_region`: Zeroes 10915 consecutive longwords starting at a computed offset into the state block, then stores a stack address and a global value into fields of that block and calls a setup routine.
- `compare_two_byte_strings`: Compares two zero-terminated byte strings character by character for at most about 32767 characters.
- `forward_with_constant_mode`: Passes its two arguments on to a shared routine together with the constant 769 as the mode argument, and returns nothing.
- `allocate_handle_slot`: Searches a table of twenty slots for a free entry and returns minus one with error code 8 when none is free.
- `apply_result_sign_fixup`: Calls an arithmetic helper and negates its result when exactly one of the two operand signs held in the data registers is negative.
- `return_second_arithmetic_result`: Calls an arithmetic helper and returns the value it leaves in the second data register.
- `perform_32_bit_division`: Divides an unsigned 32-bit dividend by a 32-bit divisor and returns the quotient and remainder together in a 64-bit value.
- `emit_bytes_until_failure`: Passes each byte of a buffer to a single-character helper, up to a count obtained from another helper call.
- `put_char_into_buffer`: Appends one byte to a bounded buffer, advancing its write pointer, and returns the byte.
- `initialize_stream_table`: Walks the stream table in 22-byte records from the start to the end of the table and calls a per-record helper on each record.
- `close_stream_record`: Flushes a stream record that has pending output, calls the close helper when the record is open, releases its buffer through a helper and clears its fields.
- `write_byte_to_stream`: Writes one byte into a stream record's buffer, flushing through a helper when the buffer is full or the record is unbuffered.
- `allocate_free_stream_record`: Searches the stream table for a record not in use, clears its fields and returns its address, or zero when every record is in use.
- `allocate_stream_buffer`: Tries to allocate a 1024-byte buffer for a stream record and records it with a buffered flag.
- `free_all_list_nodes`: Walks a singly linked list from the head stored in the state block, releases each node through a helper using its stored size plus an eight-byte header, and clears the head.
- `free_list_nodes_from_register`: Releases every node of a singly linked list starting at the pointer held in an address register, passing each node and its size plus an eight-byte header to a helper, then clears the list head in the state block.
- `allocate_list_node_at_head`: Allocates a block of the requested size plus an eight-byte header through a helper, links it at the head of the list and returns the address just past the header, or zero when allocation fails.
- `forward_single_argument_call`: Passes its single argument on to a helper call and returns nothing.
- `remove_list_node_and_free`: Searches the list for the node whose data address matches the argument, unlinks it, releases it through a helper with its size and an eight-byte header, and returns zero.
- `test_stream_record_open`: Looks up the stream record for an index in the stream table through a library call and returns true when the record pointer is non-null.
- `set_error_on_failure`: Calls a helper with its argument; on success it returns zero, and on failure it stores the helper's error value in the state block and returns minus one.
- `write_stream_checked`: Validates a stream index against the twenty-entry table, checks that the record exists and has an access mode set, then forwards the call to a helper.
- `check_signal_bit_then_helper`: Tests bit 4096 of a value returned by a helper; when it is set and a state counter is nonzero, it calls two helpers and returns the result of the second, and otherwise it returns zero.
- `check_word_mismatch_then_forward`: Compares a stored word with a value derived from the low word of a data register using fixed bit operations.
- `start_and_parse_command_line`: Opens the required library at version 31, then splits the command line into an argument array using a character class table, and calls the entry routine with that argument count and vector.
- `shutdown_and_release_resources`: Clears ten table slots through a helper, calls a stored callback, releases any non-null stored handles and buffers, and returns its argument.
- `execute_cleanup_sequence`: Iterates through a loop, calls several routines using pointers from a global structure, and performs conditional function calls based on global variable values.
- `append_string_bounded`: Appends a zero-terminated byte string to the end of a destination string, copying at most about 32767 characters and terminating the result.
- `append_string_with_count`: Appends up to a given number of characters from a source string to the end of a destination string, terminates the result and returns the destination pointer.
- `copy_string_to_destination`: Copies a zero-terminated byte string, including its terminator, into a destination buffer with no length limit, and returns the destination pointer.
- `copy_string_with_padding`: Copies characters from a source string into a destination buffer up to a given count or until a null terminator is reached, then fills the rest of the count with zero bytes.
- `close_stream_slot_entry`: Validates a stream slot index against the twenty-entry table and closes the stored handle through a helper unless a flag bit in the slot is set, then clears the slot and returns zero.
- `call_library_vector_minus_12`: Calls the library function at vector offset minus 12 from the library base it is given and returns nothing.
- `call_library_vector_minus_36`: Calls the library function at vector offset minus 36 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_126`: Calls the library function at vector offset minus 126 of the library base stored in the state block and returns nothing.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_minus_102`: Calls the library function at vector offset minus 102 of the library base stored in the state block and returns nothing.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_minus_144`: Calls the library function at vector offset minus 144 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_108`: Calls the library function at vector offset minus 108 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_54`: Calls the library function at vector offset minus 54 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_132`: Calls the library function at vector offset minus 132 of the library base stored in the state block and returns nothing.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_minus_84`: Calls the library function at vector offset minus 84 of the library base stored in the state block and returns nothing.
- `execute_library_function_via_offset`: Retrieves a library base address from a table and executes the function located at a specific offset within that library.
- `call_library_vector_minus_30`: Calls the library function at vector offset minus 30 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_60`: Calls the library function at vector offset minus 60 of the library base stored in the state block and returns nothing.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_minus_90`: Calls the library function at vector offset minus 90 of the library base stored in the state block and returns nothing.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_minus_48`: Calls the library function at vector offset minus 48 of the library base stored in the state block and returns nothing.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library2_vector_minus_108`: Calls the library function at vector offset minus 108 of the second library base stored in the state block and returns nothing.
- `call_library2_vector_minus_414`: Calls the library function at vector offset minus 414 of the second library base stored in the state block and returns nothing.
- `call_library_offset_minus_198`: Calls a function at a specific offset within a library base retrieved from a table.
- `call_library2_vector_minus_198`: Calls the library function at vector offset minus 198 of the second library base stored in the state block and returns nothing.
- `call_library2_vector_minus_294`: Calls the library function at vector offset minus 294 of the second library base stored in the state block and returns nothing.
- `call_library2_vector_minus_132`: Calls the library function at vector offset minus 132 of the second library base stored in the state block and returns nothing.
- `call_library2_vector_minus_210`: Calls the library function at vector offset minus 210 of the second library base stored in the state block and returns nothing.
- `execute_indirect_library_function`: Fetches a function pointer from a table located at an offset from a base address and executes it.
- `call_library2_vector_minus_372`: Calls the library function at vector offset minus 372 of the second library base stored in the state block and returns nothing.
- `call_library_offset_minus_372`: Calls a function at a specific offset within a library base retrieved from a table and then calls another function at a different offset.
- `call_library2_vector_minus_552`: Calls the library function at vector offset minus 552 of the second library base stored in the state block and returns nothing.
- `call_library2_vector_minus_378`: Calls the library function at vector offset minus 378 of the second library base stored in the state block and returns nothing.
- `execute_indirect_function`: Fetches a function pointer from a nested table structure and executes it.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library2_vector_minus_384`: Calls the library function at vector offset minus 384 of the second library base stored in the state block and returns nothing.
- `call_library3_vector_minus_258`: Calls the library function at vector offset minus 258 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_318`: Calls the library function at vector offset minus 318 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_354`: Calls the library function at vector offset minus 354 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_60`: Calls the library function at vector offset minus 60 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_324`: Calls the library function at vector offset minus 324 of the third library base stored in the state block and returns nothing.
- `call_library4_vector_minus_378`: Calls the library function at vector offset minus 378 of the fourth library base stored in the state block and returns nothing.
- `call_library4_vector_minus_210`: Calls the library function at vector offset minus 210 of the fourth library base stored in the state block and returns nothing.
- `call_library4_vector_minus_390`: Calls the library function at vector offset minus 390 of the fourth library base stored in the state block and returns nothing.
- `run_loop_until_helper_done`: Reads an initial value from a helper, then repeatedly calls a helper pair per iteration until a second helper returns zero, and finishes with fixed cleanup helper calls.
- `accumulate_even_increment_bounded`: Rounds its argument up to an even number and adds it to a stored accumulator, or calls a helper instead when the sum would exceed a stored limit.
- `draw_marker_at_position`: Selects a draw position from the state block, either the stored start point or a table entry, and blits a fixed-size region there.
- `step_marker_toward_target`: Repeatedly moves a stored position toward a target by at most 30 units per axis, redrawing a clipped region and a small marker pattern at each step until the position equals the target.
- `load_five_record_block`: Opens a resource through a helper, reads a header of 64 bytes and then five length-prefixed records, invoking an optional callback before and after each record, and closes the resource.
- `call_helper_with_length_32`: Calls a helper with a stored pointer, its argument and a length of 32.
- `copy_record_field_to_state`: Copies a longword from a fixed field of a record into a state field.
- `set_flags_from_status_bits`: Sets two state flags to one, clears the first when bits 192 of a global status word are all set, and clears the second and stores 52225 in the status word when bits 17408 are all set.
- `draw_characters_as_glyphs`: Draws each character of a zero-terminated string as four rectangular glyph segments, computing each glyph offset from the character code and adding 8 for codes above the letter z.
- `initialize_display_layout_values`: Stores fixed layout values for two rectangles and their offsets, copies a stored value into several state fields, and calls a helper with 1440.
- `select_slot_values_by_mask`: Fills five consecutive longword slots in the state block, each taking one of two stored values according to the matching bit of the argument.
- `call_helper_repeatedly`: Calls a helper with no arguments once for each iteration up to the given count.
- `call_handle_helper_with_arg`: Calls a library routine with the stored handle and the argument it receives.
- `call_second_handle_helper_arg`: Calls a different library routine with the stored handle and the argument it receives.
- `call_handle_helper_four_args`: Calls a library routine with the stored handle and four arguments supplied by the caller.
- `forward_three_args_with_handle`: Passes its three arguments to the four-argument handle routine, adding the stored handle as the fourth argument.
- `initialize_buffers_and_layout`: Initializes timing and layout fields, calls a helper with block sizes 9200, 6100, 5900, 4440 and 12800, sets five offsets from a stored value, and when a flag is set initialises four further offsets spaced 8000 apart.
- `initialize_and_save_records`: Initializes a table of twelve 16-byte entries from a stored base and writes it to the named file opened in old-file mode 1005, then closes the file.
- `save_two_block_file`: Opens the named file in old-file mode 1005, writes 45768 bytes from a stored address, seeks to position 21760, writes a further 19292 bytes from the next block and closes the file.
- `set_global_status_fifteen`: Stores 15 in a global status variable.
- `fade_palette_components_down`: Copies 32 color words into a working palette, then fifteen times lowers each 4-bit red, green and blue component of every color by one step, sending the palette to a helper after each step.
- `fade_palette_toward_target`: Clears a 32-word working palette, then fifteen times moves each 4-bit red, green and blue component one step toward the matching component of the target palette, sending the working palette to a helper after each step.
- `apply_sign_to_result`: Returns the result of another routine while applying the sign of the input registers to the output.
- `measure_string_length`: Counts the bytes of a zero-terminated string, excluding the terminator, and returns the count.
- `multiply_longwords_partial`: Multiplies two 32-bit values held in the data registers using 16-bit partial products and returns the low 32 bits of the product.
- `call_library_vector_minus_6`: Calls the library function at vector offset minus 6 from the library base it is given and returns nothing.
- `call_indirect_library_function`: Fetches a pointer from a table at a specific offset, retrieves a library base from that pointer, and executes the function located at a fixed offset within that library.
- `call_library_function_via_offset_2`: Retrieves a pointer from a table using an offset, subtracts a constant value from that pointer, and executes the resulting address.
- `call_library_vector_minus_42`: Calls the library function at vector offset minus 42 of the library base stored in the state block and returns nothing.
- `call_library_vector_minus_66`: Calls the library function at vector offset minus 66 of the library base stored in the state block and returns nothing.
- `call_library2_vector_minus_216`: Calls the library function at vector offset minus 216 of the second library base stored in the state block and returns nothing.
- `execute_indirect_library_function_2`: Retrieves a pointer from a table using an offset, accesses a secondary structure via that pointer, and executes a function located at a fixed offset within that structure.
- `execute_indirect_library_function_3`: Retrieves a pointer from a table using an offset, accesses a library base through that pointer, and executes a function at a specific offset within that library.
- `call_library2_vector_minus_498`: Calls the library function at vector offset minus 498 of the second library base stored in the state block and returns nothing.
- `call_library3_vector_minus_30`: Calls the library function at vector offset minus 30 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_300`: Calls the library function at vector offset minus 300 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_552`: Calls the library function at vector offset minus 552 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_246`: Calls the library function at vector offset minus 246 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_282`: Calls the library function at vector offset minus 282 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_192`: Calls the library function at vector offset minus 192 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_240`: Calls the library function at vector offset minus 240 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_306`: Calls the library function at vector offset minus 306 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_342`: Calls the library function at vector offset minus 342 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_348`: Calls the library function at vector offset minus 348 of the third library base stored in the state block and returns nothing.
- `call_library3_vector_minus_270`: Calls the library function at vector offset minus 270 of the third library base stored in the state block and returns nothing.
- `call_library4_vector_minus_78`: Calls the library function at vector offset minus 78 of the fourth library base stored in the state block and returns nothing.
- `call_library4_vector_minus_198`: Calls the library function at vector offset minus 198 of the fourth library base stored in the state block and returns nothing.
- `call_library4_vector_minus_204`: Calls the library function at vector offset minus 204 of the fourth library base stored in the state block and returns nothing.
- `call_library4_vector_minus_270`: Calls the library function at vector offset minus 270 of the fourth library base stored in the state block and returns nothing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_minus_one_status`: Does no work and returns minus one.
