# Game 1545 (gravity-force 2 v1.0 (1994)(bits)(pd)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01545). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 106 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 215 of 215 named (0 author symbols), 215 of 215 described (12 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5582 of 5582 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9303 of 9303 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 255548 |
| code | 38440 |
| data | 153383 |
| padding | 24245 |
| unused | 429568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| LogMerge | 2 | 501 | 27 |
| GF2_D2M.DAT | 2 | 409 | 10 |
| GF.EXE | 7 | 7081 | 146 |
| GetPath | 2 | 159 | 5 |
| GF2_M.DAT | 4 | 32 | 1 |
| GF2_SPEC.DAT | 2 | 876 | 20 |
| Gravity-Force 2 v1.0 (1994)(BiTS)(PD)[a3].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `take_over_display_load_data` (GF.EXE): Takes over the display by disabling interrupts, owning the blitter and loading an empty view, then programs the DMA control and copper registers. Kind: bitmask/status check. Pass path: the branch at an address (when d0 is not < 1).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_and_call`: If the output pointer is set, copies sixteen 32-bit words from a fixed offset in the data into it, then calls a stored function pointer.

### assign

- `find_max_value_and_report`: Program entry for the assign tool.
- `copy_memory_block`: Copies sixteen bytes from a memory location to a destination pointed to by the stack.

### LogMerge

- `run_log_merge_entry`: Program entry for the log merge tool.
- `run_fixed_helper_sequence`: Runs a fixed sequence of helper calls, with one extra call when a flag is set, then returns.
- `emit_entries_skipping_marked`: Walks a list of 25-word entries and, for each entry whose key is not 1234, calls helpers to write its fields.
- `accumulate_counts_for_two_passes`: Runs a helper pass that adds its count to a running total.
- `merge_two_entry_lists`: Walks two lists of 25-word entries together and merges them, comparing their key fields and calling helpers for each matched or unmatched entry.
- `copy_zero_terminated_string`: Copies a zero-terminated string from the source to the destination, including the terminator.
- `write_negated_sum_checksum`: Sums a run of 32-bit values, then stores the negated sum XORed with a fixed tag into the first field of a header.
- `verify_checksum_and_reset_records`: Recomputes the checksum of a block.
- `xor_decode_words_backward`: Walks a buffer of 16-bit words from its end toward its start, XOR-combining each word with the word after it and a fixed key.
- `xor_fold_words_forward`: Walks a buffer of 16-bit words forward, replacing each word with the XOR of itself, the following word and a fixed key.
- `call_routine_five_times`: Calls the same unnamed routine five times in a row and then returns.
- `call_twice_if_flag_set`: Always invokes one routine, then invokes it a second time only when a global flag word is nonzero.
- `close_library_and_return`: Makes a single call to the library-closing wrapper and returns to its caller.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 20 into d0, then returns.
- `validate_path_argument_option`: Checks a command-line path argument.
- `call_shared_routine_once`: Makes a single call to a shared routine and returns, doing no other work of its own.
- `copy_quoted_or_space_token`: Copies one token from a source string into a destination buffer.
- `find_delimiter_pair_or_stop`: Scans a string for a specific delimiter character followed by one of two marker characters.
- `return_failure_with_stack_value`: Returns a 64-bit result whose upper half is minus one and whose lower half is the first stack argument.
- `scan_for_marker_or_stop`: Scans forward through a string for either of two marker characters, returning zero on the first match.
- `read_whole_file_by_size`: Gets a file's size by locking and examining it, then reopens the file and reads its contents into a buffer sized from that length.
- `write_buffer_to_file`: Opens a file through the dos library, writes the given buffer to it, and closes the file.
- `write_zero_terminated_text`: Advances to the end of a zero-terminated text string and then calls the dos write routine.
- `open_dos_library_and_streams`: Opens the dos library and fetches its standard input and output handles.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `allocate_into_free_slot`: Finds an empty slot in a fixed table of pointer pairs, records a requested size in it, and asks the exec library to allocate memory.
- `free_allocated_slot_list`: Walks a linked list of allocated blocks held in a global table, freeing each block through the exec library until a null link is reached.

### GF2_D2M.DAT

- `build_sorted_directory_index`: Opens the dos library and walks a directory listing, collecting entry names and sorting them with an exchange sort.
- `build_name_record_table`: Walks a list of zero-terminated names, case-insensitively matches each against a reference name, and writes fixed-size records of copied names and running counters into a table.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_single_instruction_handler`: The facts report a single instruction reached only through a handler pointer, so no behaviour can be recovered from it.
- `execute_indirect_calls`: Stores two values into global variables and performs three indirect function calls using pointers retrieved from a memory location.
- `open_read_close_file_chunk`: Opens a file through the dos library, reads from it and closes it, and it is called from the buffer preparation routine.
- `initialize_name_record_table`: Clears a memory block and populates a table with a series of formatted strings starting with a specific four-character prefix followed by incremental numeric characters.
- `initialize_player_name_table`: Initializes a table of player name records by populating them with a formatted string containing a prefix and a numeric suffix.
- `initialize_name_record_table_2`: Clears a memory buffer and populates it with a series of formatted strings starting with a specific prefix followed by incrementing numeric characters.

### GF.EXE

- `start_and_wait_for_messages`: Program entry point that finds its own task, waits on a message port and retrieves messages from it.
- `take_over_display_load_data`: Takes over the display by disabling interrupts, owning the blitter and loading an empty view, then programs the DMA control and copper registers.
- `handle_key_into_text_buffer`: Clears a flag, then examines a key code passed in a register.
- `clear_word_table`: Clears thirty-two consecutive 16-bit words in memory to zero and returns.
- `classify_control_character`: Examines a character code passed in a register.
- `find_matching_string_entry`: Walks a table of zero-terminated names, comparing each with a buffer.
- `forward_to_shared_routine`: Makes one call to a shared routine and returns, doing no other work of its own.
- `clear_flag_and_call_pointer`: Calls one routine, clears a global word, calls another routine, and then calls through a stored function pointer.
- `initialize_game_state_and_display`: Initialises working variables and copies two tables into place, clears further blocks of memory, and sets the video and DMA state.
- `derive_default_totals_record`: Fills three default values in a small record when they are zero, derives a total from them and stores a value about two-thirds of that total.
- `tiny_asl`: Small routine of 1 instruction that shifts d0 left by 1.
- `write_separator_and_pair`: Writes a fixed separator character and then two stored bytes into the output buffer, making several calls to other routines as it goes.
- `sort_records_by_key`: Builds a table of records from two arrays and sorts them with an exchange sort.
- `format_five_digit_decimal`: Converts a number up to 99999 into decimal digits in a text buffer.
- `format_four_digit_decimal`: Writes a value into a text buffer as four decimal digits, capping it at 9999 and padding with zeros.
- `copy_indexed_twelve_byte_entry`: Copies a twelve-byte entry from a table, selected by an index, into two destination areas of six bytes each.
- `verify_display_configuration`: Compares a set of stored register values against expected constants and returns zero only when they all match.
- `shift_buffer_past_ampersand`: When the first byte of a fixed-size text buffer is an ampersand, the routine shifts the buffer left by one byte and terminates it.
- `reset_audio_and_display_state`: Clears several blocks of game variables and then resets the hardware for sound and display.
- `write_newline_to_two_buffers`: Writes a line-feed character into two output buffers, making calls to other routines before each write.
- `build_symbol_lookup_table`: Builds a table of 16-bit symbol values from a list, subtracting a fixed offset from each and ending the table with a sentinel value.
- `skip_computed_count_strings`: Advances a pointer past a computed number of zero-terminated strings, counting down one string at a time.
- `skip_strings_by_table_index`: Advances a pointer past a number of zero-terminated strings, with the count taken from a stored index multiplied by another stored value.
- `update_fire_button_palette`: Writes three palette colour values to fixed hardware addresses, choosing between two sets according to the fire-button state read from the CIA port register.
- `reset_game_variables_block`: Clears two counters and a block of sixty-four words of game variables, copies one stored value into another, calls another routine and then clears a further flag.
- `wait_for_fire_button_state`: Repeatedly calls four routines and polls the fire-button and direction bits of the input port, returning once the button state changes as expected.
- `wait_for_input_port_match`: Repeatedly calls three routines and reads two state bits of the input port until their combination matches the expected pattern, then returns.
- `compute_screen_offsets_from_value`: Derives several screen position values from one input by replicating its low nibble and adding fixed offsets.
- `draw_image_records_from_list`: Waits for the blitter, then walks a list of image records and draws each one into the display buffer.
- `track_stick_position_while_held`: Starts a position value from an input and loops while the input port reports the controls as held, nudging the value up or down according to the directional bits it reads.
- `fill_copper_pattern_table`: Fills a table of 127 entries with a repeating pattern, placing a special entry at one index.
- `process_object_list_frame`: Processes the per-frame object list by checking a pending flag and walking the entries to compare positions and velocity values, calling eleven routines to act on the results.
- `dispatch_table_entry_by_type`: Dispatches on a type code stored in a record.
- `spin_until_counter_wraps`: Increments a byte in a loop until it wraps to zero and the low bit is clear.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `increment_global_byte_until_zero`: Increments a global byte repeatedly until it reaches zero, continuing the loop as long as the value remains odd.
- `spin_until_counter_wraps_2`: Increments a byte in a loop until it wraps to zero and the low bit is clear.
- `wait_for_matching_counter_value`: Spins until a stored byte matches the value passed in the data register, then continues while its low bit is set.
- `spin_until_byte_nonzero`: Spins while a stored byte is zero, then repeats while its low bit is clear.
- `wait_byte_match_on_beam`: Spins until a stored byte matches the value passed in a register, with a second wait when a high flag bit is set.
- `run_frame_loop_mouse_buttons`: Runs the frame loop by calling a sequence of routines on each pass.
- `clear_conditional_word_blocks`: Clears one of two blocks of 22 words depending on two flag tests, setting a sentinel in one slot.
- `update_animation_countdown_state`: Counts down an animation timer and reloads it from one of a few values when it expires, updating a state byte along the way.
- `init_object_record_fields`: Initialises an object record by storing the register arguments into its fields.
- `call_two_routines_in_sequence`: Makes two calls to other routines in sequence and returns.
- `test_bounding_box_overlap`: Checks whether a record's coordinates overlap a bounding box with a margin of seven units.
- `decode_keyboard_serial_input`: Reads a keyboard code from the CIA serial register and converts it to a letter.
- `draw_sprite_row_blitter`: Counts down two timers and, when they expire, selects a source image address and starts a blitter copy of it into the display.
- `stub_return_only`: Contains only a return, so there is no behaviour to recover from it.
- `clamp_sprite_position`: Clamps two position values to a screen range and updates counters that step them.
- `compute_clipped_sprite_coordinates`: Scales two coordinates by a factor, clamps them to a fixed limit and sets their low bits.
- `process_active_entry_list`: Walks a table of ten three-word entries and, for each active entry, calls two routines.
- `adjust_field_when_in_bounds`: Checks whether one rectangle lies within a bounds range and, if so, subtracts a stored amount from a 32-bit field of the second record.
- `add_field_when_in_bounds`: Applies the same bounds test as its neighbouring routine and adds a stored amount to the 32-bit field when the test passes.
- `advance_sprite_animation_frames`: Walks ten animation records, decrementing each countdown and advancing its frame when it reaches zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into -(a4).
- `update_blitter_state_table`: Updates an internal state table and index using arithmetic operations on global counters.
- `update_blitter_state_table_2`: Updates a state table and internal counters based on bitwise manipulations of global tracking variables.
- `update_blitter_state_table_3`: Updates a state table and internal counters used by a blitter register copy routine.
- `stub_fallthrough_handler`: The facts report four instructions that fall through into the routine after it, reached only through a handler pointer.
- `program_blitter_copy_registers`: Advances a frame counter, then programs the blitter's source, destination, modulus and size registers for a copy, polling the blitter status before the operation starts.
- `fill_sprite_record_from_table`: Looks up two values by index in a table and writes them into a sprite record as adjusted coordinates, with a fixed offset applied.
- `fill_sprite_record_if_valid`: Looks up coordinate values for an index and writes them into a record when the index is in range, returning zero.
- `process_pending_draw_requests`: Walks a list of pending draw requests, calling ten routines to handle each one.
- `apply_motion_transform_by_type`: Adjusts a pair of coordinate values according to a type byte.
- `step_positions_within_limits`: Steps two position values by fixed increments while they remain under their limits, decreasing a timer on each step.
- `set_field_if_mode_two`: When a mode byte equals two it sets a field to fifteen.
- `scan_until_test_passes`: Loops over a set of entries, calling a test routine for each, and returns as soon as the test reports a hit.
- `draw_sprite_list_blitter`: Walks a list of sprite entries until a zero entry, filling each one's position and blitter values and waiting for the blitter before drawing.
- `clip_and_place_sprite_entry`: Updates a sprite's position from its stored velocity, clamps it at the screen edges and then places it.
- `clamp_bounds_on_flag`: When a flag is set and a coordinate is negative, computes a clamped position from two limits, stores it into the output pair, and clears the source entry.
- `subtract_stored_deltas_if_clear`: Subtracts stored position deltas from two offset pairs when their enable flags are clear, and preserves saved values across the call.
- `lookup_distance_falloff`: Computes a squared distance between a point and a stored reference, takes a value from a lookup table when the distance is small enough, and returns a scaled result.
- `copy_and_mirror_row_pairs`: Copies source words into two destination buffers and writes a derived value, shifted and offset, alongside each copy.
- `select_position_and_write_glyphs`: Chooses one of several screen positions by comparing a stored coordinate against thresholds, then writes a sequence of character codes and attribute bytes into a buffer.
- `reset_audio_channels_and_dma`: Clears the audio channel state, selects volumes from two flags, and writes the DMA control value to enable the channels.
- `fetch_next_audio_entry`: Reads the next three-word entry from a sound list, stores its fields and advances the list pointer.
- `bounce_on_wall_contact`: Checks two objects against boundary tests and, when a contact code matches one of a set of values, reverses the vertical or horizontal velocity.
- `update_collision_counters_by_type`: Swaps two saved coordinate pairs, then checks an object type code and, for matching types, adds fixed increments to a score or stat and updates a stored state.
- `compute_tile_cell_mask`: Computes the address of a tile cell from a row and column index, reads two words from it and combines them into a mask.
- `scroll_viewport_by_steps`: Moves a horizontal scroll position by fixed steps when a flag is set, then takes the next display column from a stored value and calls routines to draw it.
- `build_draw_request_for_object`: Fills a draw request for an object record when it is visible, storing its position and fixed blit values.
- `init_blit_record_defaults`: Stores a position and a fixed set of blit values into a record, setting two of its fields to thirty-six, and returns the value passed in.
- `init_blit_record_with_mask`: Stores a position combined with a fixed mask and several blit values into a record, setting two of its fields to thirty-six, and returns the value passed in.
- `blit_entry_list_to_display`: Walks a list of ten image entries and, for each one that has a value, programs the blitter's source, destination and size registers to copy it.
- `start_blit_request`: Waits until the blitter is free, then fills a blit request record with a control word, source and destination pointers and a size from its arguments.
- `blit_second_entry_list`: Walks a list of image entries one per pass and, for those without a value, programs the blitter to copy them into the display.
- `clamp_scaled_table_offset`: Computes a value from a table entry and two inputs, scales it by sixteen, clamps it to a fixed range and adds the offset passed in.
- `update_positions_and_run_routines`: Updates two position values from stored inputs when their flags are clear, then calls ten routines in turn.
- `try_advance_object_position`: Checks whether an object is clear of a bound and has room to move, then advances its position by three and calls another routine.
- `try_advance_second_object`: Performs the same position check and advance as its neighbouring routine, using the second object's fields.
- `process_pending_action_flags`: Handles pending action flags: when a flag is set it clears it and calls routines to attempt the action.
- `apply_collision_velocity_damping`: When an object is marked as hit, reduces its counter by four and shrinks its velocity components by one eighth.
- `process_player_input_frame`: Reads the fire button and direction state each frame, updates the player's velocity and fire state, and calls routines to move and act on them.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `return_empty_routine_2`: The routine is empty and performs no operations before returning.
- `pack_input_bits_into_byte`: Packs bits from an input word into a state byte, using the low two bits for the first two inputs and setting higher bits from the fire state.
- `unpack_input_state_word`: Unpacks a state byte into an input word, setting bits for the first two inputs and for the matching fire state.
- `read_mouse_and_joystick_ports`: Reads the mouse and joystick port movement registers, converts the movement into direction values, and updates the fire and position state.
- `load_file_size_and_contents`: Locks and examines a named file to get its size, then reopens it and reads the data into a buffer, closing the file afterwards.
- `store_palette_colour_entry`: Stores a 12-bit colour value and derives two further colour entries from it by shifting and masking.
- `allocate_tracked_memory_block`: Finds a free entry in a table of tracked memory blocks, records a requested size there and asks the exec library to allocate memory.
- `free_all_tracked_blocks`: Walks the table of tracked memory blocks and frees each block through the exec library, skipping entries that are already empty.
- `free_tracked_block_by_key`: Searches the tracked memory table for a given key, marks the matching entry as released and frees its block through the exec library.
- `fetch_input_state`: Reads a value from a memory location and returns it through a register.
- `restore_system_state_on_exit`: Restores the system before exit by waiting briefly, unloading the loaded segment, closing the libraries, re-enabling interrupts and disowning the blitter and view.
- `forbid_and_reply_message`: When a flag is set, it forbids task switching and replies to a message, then returns zero.
- `interpolate_line_points`: Computes midpoint coordinates for a pair of points and steps through interpolated positions between them for drawing.
- `set_clip_bounds_and_draw`: Sets a clipping rectangle expanded by four units beyond the given corners, then draws the requested shape within it, calling two routines to do the work.
- `draw_three_pass_layers`: Runs a drawing routine over three layers, stepping the buffer pointer by a fixed amount after each pass, and repeats until a counter wraps to zero.
- `draw_four_pass_layers`: Runs the same pass loop as its neighbouring routine, but over four layers, stepping the buffer pointer by a fixed amount each time.
- `start_blitter_copy_operation`: Works out a blitter source offset from two coordinates, waits for the blitter, then writes the modulus, pointer, first-word mask and size registers and starts the operation.
- `draw_blitter_line_octant`: Computes the absolute differences between two points and selects one of eight line directions from them.
- `format_numbered_label`: Builds a text label by placing a digit character first and then a source string, and copies the finished label into a destination buffer.
- `draw_menu_entries_with_input`: Copies a text label into a buffer, then loops over a set of entries, drawing each one by repeated calls to drawing routines.
- `decode_keyboard_key_to_char`: Reads a keycode from the keyboard serial register and maps it through a symbol table to a character.
- `tiny_moveq`: Small routine of 1 instruction that loads 7 into d2.
- `update_state_for_type_codes`: Stores a value into state bytes for three type codes and returns true for two of them, and for the third only under a condition.
- `forward_to_single_routine`: Makes one call to another routine and returns, with a few unreachable branches removed by the decompiler.
- `render_text_string_glyphs`: Walks a zero-terminated text string and renders each character as a glyph, computing its screen position from the running coordinates and the glyph width.
- `blit_masked_image_region`: Computes blit parameters for an image region from a set of offsets, programs the blitter's source, mask, destination and size registers, and calls routines to finish each row.
- `scan_text_for_control_code`: Scans a text string for a single special character, mapping characters through a six-entry substitution table.
- `clamp_input_position_on_fire`: Checks the fire and direction bits of the input port and, when a movement is blocked, sets a limit value of 199.
- `tick_frame_counter_and_toggle`: Increments a frame byte and counts down a timer.
- `absolute_product_swapped_halves`: Takes the absolute values of two inputs, multiplies the high word of one by the low word of the other, and returns the product with the second input's halves swapped.
- `check_horizontal_window_draw`: Checks whether a horizontal distance lies within a window running from just below zero to 106 units.
- `check_vertical_window_draw`: Applies the same window test as its neighbouring routine on the other axis.
- `build_address_table_entries`: Fills a five-entry table with addresses spaced 10752 bytes apart, then fills a block of entries starting from 384 and copies each to a companion word.
- `build_row_address_table`: Fills a three-entry table with addresses spaced 42 bytes apart, then fills a block of words starting from 386 and copies each to a companion word.
- `initialize_sprite_position_table`: Stores a sequence of preset coordinate pairs into a table and then copies entries in a loop over the whole table.
- `compute_row_address_pair`: Clamps a row index between fixed bounds and computes two addresses for that row.
- `pack_tile_attribute_word`: Packs a row, a column and flag bits into a 32-bit attribute word, adding a fixed offset and an extra bit when a special condition holds, then stores it.
- `pack_tile_attribute_pair`: Packs a 16-bit value and a flag into a 32-bit attribute word with a fixed offset, using a second offset when a special condition holds.
- `select_source_by_slot`: Picks one of several source values by comparing the target address against a set of slots, then copies the chosen value into a register and calls a routine.
- `clear_fifteen_long_words`: Clears fifteen consecutive 32-bit entries starting after the address passed in.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_masked_port_bits`: Masks a status word with a fixed bit pattern and stores the result in a global variable for later use by the input code.
- `decode_joystick_direction_bits`: Decodes direction bits read from the two joystick ports into direction flags, cancelling out opposing directions.
- `update_object_coordinates`: Calculates a memory offset based on input values and performs a bitwise division by two on two consecutive memory locations.
- `blit_sprite_frame_at_position`: Computes a clipped sprite position from a stored origin, waits for the blitter and programs its registers to draw the frame in a single blit operation.
- `load_level_graphics_and_enable`: Stores two file-name pointers from its argument block, then takes over the blitter and sets the DMA and background colour registers.
- `compute_line_slope_octant`: Works out which octant a line from one point to another lies in, using the signs and relative sizes of the two offsets.
- `process_timed_entry_list`: Walks a list of twelve-byte entries, skipping empty ones, and for each active entry checks a flag pair before calling a routine.
- `insert_entry_into_free_slot`: Scans a table of 16-bit entries, remembering the first empty slot, and stores a new entry into the appropriate position.
- `test_bitmap_cell_occupied`: Computes a byte address and bit within a bitmap from a position, and checks that cell and two neighbouring bytes for a set bit.
- `find_matching_entry_position`: Computes two candidate positions from table lookups and walks a linked list looking for an entry that matches them, returning the match.

### GetPath

- `setup_gf2_assign_and_copy`: Program entry for the path setup tool.
- `write_file_returning_status`: Opens a file through the dos library and, when it opened, writes the buffer and closes the file, returning one.
- `write_string_to_output`: Measures a zero-terminated string and writes it to the output handle through the dos library, returning its two register inputs unchanged.
- `open_console_window_for_output`: Opens the dos library and then a console window named by its text, keeping the handle for output.
- `close_console_and_library`: Closes the console handle if it was opened, then closes the dos library.

### GF2_M.DAT

- `fill_globals_with_constant`: Writes a single register value into a long list of global variables in one pass and returns.

### GF2_SPEC.DAT

- `start_spec_report_and_log`: Program entry point that sets up working values and calls ten routines in sequence, with the log file name among its text references.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 152 into d0, then merges 152 into d0, then merges 152 into d0.
- `format_report_field_values`: Formats a set of computed values into a report buffer through several calls, each one writing a field of the report.
- `format_percentage_gain_text`: Formats a percentage figure as two decimal digits and writes it into a text buffer, but only when a file signature check passes.
- `print_separator_line_with_flags`: Writes a line made of two dashes and a bracketed label through a stored output routine, adding a flag digit for each of two optional settings.
- `write_times_phrase_text`: Writes a short phrase ending in 'times.' into a text buffer through a stored output routine.
- `format_elapsed_time_text`: Converts a frame count, taken at fifty frames per second, into hours and minutes text with singular or plural wording.
- `format_unsigned_decimal_text`: Writes an unsigned number as decimal digits into a text buffer, using a descending series of place values and an optional leading character.
- `clear_record_and_mark_end`: Clears a 120-byte buffer and writes a terminator value into a word that the record refers to.
- `write_checksum_tag_into_header`: Sums a sequence of values and stores the negated total, XORed with a fixed tag, into a header word.
- `xor_decode_words_via_pointer`: Like its neighbouring routine, walks a buffer of 16-bit words backward from an end pointer, XOR-folding each word with the next one and a fixed key.
- `xor_fold_words_via_pointer`: Walks a buffer of 16-bit words forward from a pointer argument, replacing each word with the XOR of itself, the following word and a fixed key.
- `find_or_add_table_key`: Searches a table of keyed records for a given value and, if no record matches, appends a new record and increments the count.
- `process_edge_mask_buffer`: Clears and populates a temporary buffer with data from a source pointer, then performs a nested loop to transform or copy bitmask data using bitwise shifts and arithmetic operations.
- `blit_bitmap_with_edge_masks`: Blits a bitmap region with first and last word masks, computing the source and destination offsets from its arguments and calling routines to write each row.
- `copy_rows_with_stride`: Copies a block of words row by row from one buffer into another, moves the pointers back by a fixed stride, and copies a second block.
- `wait_for_blitter_idle`: Spins while the blitter busy bit of the DMA status register is set, then returns.
- `decode_packed_bitplane_rows`: Checks a two-byte BP signature and, when it matches, decodes run-length packed bytes into a buffer, repeating each run of values as the header describes.
- `fill_block_with_byte`: Fills a block of memory with a repeated byte value, aligning the start to a four-byte boundary first and then writing four bytes at a time.
- `write_log_file_or_flash`: Opens the log file through the dos library.

### Gravity-Force 2 v1.0 (1994)(BiTS)(PD)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
