# Game 1546 (gravity-force 2 v1.0 (1994)(bits)(pd)[a4]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01546). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 106 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 215 of 215 named (0 author symbols), 215 of 215 described (12 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5546 of 5546 (100.00%) |
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
| Gravity-Force 2 v1.0 (1994)(BiTS)(PD)[a4].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_hardware_and_load_data` (GF.EXE): Opens the graphics and dos libraries, loads the game data files from the named paths, and takes over the blitter. Kind: disk hardware/track check. Pass path: A6.

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_to_buffer`: When the pointer argument is non-null, copies sixteen longwords from offset 536 of the structure addressed by A2 into the buffer that the pointer names.

### assign

- `start_program_and_prepare_buffer`: Program entry routine of 115 instructions.
- `copy_memory_block`: Copies sixteen bytes from a memory location to a destination pointer.

### LogMerge

- `start_logmerge_with_checks`: Program entry for LogMerge.
- `run_logmerge_setup_calls`: Makes a fixed sequence of calls, one of which runs only when a global value is non-zero.
- `scan_entry_list_for_marker`: Walks a counted list of entries from a global base, skipping entries whose tag word equals 1234 and checking a byte for the ampersand character.
- `swap_buffer_pointers_and_update`: Clears a counter, points two buffer pointers at their fixed slots, and calls another routine before adding the counter into a running total.
- `process_paired_list_entries`: Loops over two entry lists, copying and comparing 32-bit values between them and calling another routine.
- `copy_zero_terminated_string`: Copies bytes from the source string to the destination, including the terminating zero, and stops after that zero.
- `store_negated_checksum_header`: Sums the longwords after the first one in a block and stores the negated total, XORed with a fixed four-character tag, into the first longword.
- `verify_checksum_header`: Recomputes the sum of the longwords that follow the header and folds in the stored tag-XORed value.
- `decode_word_chain_xor`: Walks a block of words backwards, replacing each word with the XOR of its successor, a fixed 16-bit constant, and its own value.
- `xor_scramble_word_pairs`: Walks a block of 16-bit words, replacing each word with its XOR against the next word and a fixed mask.
- `invoke_subroutine_five_times`: Calls the same subroutine five times in a row with no other work between the calls.
- `call_routine_if_flag_set`: Calls a subroutine once, then calls it a second time only when the shared flag byte is non-zero.
- `release_library_during_cleanup`: Calls a single subroutine and returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 20 into d0, then returns.
- `parse_log_file_argument`: Parses an argument string.
- `forward_log_setup_call`: Calls one subroutine and returns, with no logic of its own.
- `copy_token_until_delimiter`: Copies characters from the source string to the destination, starting after an optional opening double quote.
- `scan_string_for_stop_characters`: Scans a string for a two-character marker and returns zero when it finds one.
- `return_minus_one_status`: Returns a fixed failure value of minus one in the upper half of the result and passes the stack value through in the lower half.
- `scan_text_for_marker_status`: Scans a string and returns zero when the character after the current position matches one of two codes.
- `load_file_into_memory`: Locks a file, examines it to get its size, allocates a buffer of that size and reads the file into it.
- `write_buffer_to_file`: Opens a file through the dos library, writes the supplied buffer to it and closes it again.
- `write_null_terminated_text`: Finds the end of a zero-terminated string and passes the text to the dos write call.
- `open_dos_library_and_check`: Opens dos.library through the exec OldOpenLibrary call, then calls two library functions on it.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `allocate_tracked_memory_slot`: Finds the first free entry in a table of allocation records and calls the exec AllocMem routine with the requested size.
- `free_tracked_memory_blocks`: Walks the allocation table from the start and calls the exec FreeMem routine on each entry it reaches, stopping at the first empty entry.

### GF2_D2M.DAT

- `build_sorted_directory_listing`: Opens dos.library and walks a listing of the referenced directory paths, copying each name into a table.
- `build_records_from_matching_names`: Compares names in a table with a key while ignoring letter case.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `save_args_and_call_library`: Saves its two register arguments in shared variables, calls a library function through the stored library base, then calls two more library functions and returns.
- `execute_indirect_calls`: Stores two registers into global variables and executes two function calls via pointers retrieved from a memory location.
- `open_read_close_via_dos`: Saves its two register arguments, calls the dos library to open a file, then calls two more dos functions that read and close it.
- `initialize_data_record_table`: Clears a block of memory and populates a table with a series of formatted string records.
- `initialize_asset_metadata_table`: Initializes a global data structure by clearing memory and populating it with a series of formatted string entries.
- `initialize_data_record_table_2`: Clears a memory block and populates a table with string identifiers and associated pointers.

### GF.EXE

- `run_startup_setup_sequence`: Entry routine for the executable.
- `setup_hardware_and_load_data`: Opens the graphics and dos libraries, loads the game data files from the named paths, and takes over the blitter.
- `poll_key_and_store_input`: Calls the input routine and returns early when the key code is the escape code.
- `clear_word_buffer`: Writes zero into 32 consecutive 16-bit words starting at the buffer address.
- `classify_control_key_input`: Checks the incoming key code.
- `match_string_table_entry`: Walks a list of zero-terminated entries and compares each with a name string.
- `forward_to_subroutine`: Calls one subroutine and returns.
- `clear_state_and_call_handler`: Calls a routine, clears a shared state word, calls another routine, then calls the function pointer saved in a shared variable.
- `initialize_display_state_buffers`: Clears two large word buffers and splits an input block into two arrays.
- `seed_counters_and_fill_table`: Sets three 32-bit counters to a base of 15000 when they are zero, then derives their differences and a weighted average.
- `tiny_asl`: Small routine of 1 instruction that shifts d0 left by 1.
- `write_comma_and_repeat_byte`: Calls a routine, writes a comma character to the output buffer, and calls a routine again.
- `sort_records_by_key_fields`: Fills a table of five-word records with their indices, then sorts them by comparing up to three key fields.
- `convert_number_to_decimal_text`: Converts an unsigned 32-bit value to decimal digit characters, capped at 99999, and suppresses leading zeros.
- `write_four_digit_decimal_text`: Writes a 16-bit value as four decimal digit characters, capped at 9999.
- `copy_twelve_byte_record`: Copies a 12-byte entry from a table, selected by the index in the low word, into an output record as one 4-byte and one 2-byte field, then a second 4-byte and 2-byte pair.
- `validate_settings_block`: Copies up to eight table entries into a work area, then compares a set of stored settings with expected values.
- `shift_buffer_past_ampersand`: If the first character of a text buffer is an ampersand, shifts the next 18 bytes one place forward to drop it and writes a zero terminator.
- `reset_audio_and_display_state`: Clears state variables and counters, copies lookup values into working fields, and sets up the four audio volume registers, the copper and the DMA control register.
- `write_newline_to_two_outputs`: Calls a routine, writes a newline byte to one output pointer, calls the routine again, then writes a newline byte to a second output pointer.
- `build_scaled_index_table`: Copies a list of 16-bit entries into an index table, stopping at the first value below 257, and ends the list with minus one.
- `skip_strings_by_computed_count`: Computes a count from two shared values, then walks forward past that many zero-terminated strings in the data area.
- `skip_strings_by_second_count`: Skips a number of zero-terminated strings in the data area, with the count derived from a shared value and a second multiplier.
- `poll_fire_button_flags`: Chooses one of two sets of three shared words depending on a countdown and the fire button bit read from the joystick port register.
- `reset_state_and_clear_buffer`: Clears two shared counters and a 64-entry block of 32-bit words, copies a shared value into a state variable, calls a routine, then clears a flag.
- `wait_for_fire_button_press`: Samples the two fire-button bits of the button port repeatedly, waiting for the required pattern to appear.
- `wait_for_fire_button_match`: Waits until one fire-button bit reads high, then until the second bit is set and the sampling routine returns zero.
- `derive_packed_and_offset_values`: Builds derived words from a shared input value: a packed word, two adjusted copies, and two accumulators that grow by the input halves.
- `fill_block_cells_from_table`: Walks a table of records and, for each record with a non-zero index, fills an 8-by-8 block of 32-bit cells in a buffer.
- `adjust_bounded_level_from_input`: Converts a shared input value to a starting level, then raises or lowers it by one while the button and status flags allow.
- `fill_pattern_table_and_mask`: Fills a 127-entry table of 32-bit words with a repeating pattern, substituting a marker value at one index.
- `process_button_input_records`: Processes a table of records, comparing pairs of entries by their position fields and updating the entries and counters.
- `dispatch_record_command`: Reads a command code from a record and dispatches on it.
- `spin_until_byte_wraps`: Increments a shared byte in a loop until it wraps to zero with its low bit clear, then returns.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `increment_global_byte_until_zero`: Increments a global byte repeatedly until it reaches zero, continuing the loop as long as the byte is odd.
- `spin_wait_for_video_position`: Increments a shared byte until it wraps to zero with its low bit clear, which is a busy-wait.
- `wait_for_byte_match_sync`: Busy-waits until a shared byte equals the argument byte and its low bit is clear.
- `wait_for_zero_sync_byte`: Busy-waits until a shared byte reads zero, repeating while its low bit is clear.
- `wait_for_sync_byte_value`: Busy-waits for a shared byte to match a value.
- `run_frame_update_loop`: Runs the per-frame loop, calling a long series of routines and clearing two counters when their flags are set.
- `reset_state_blocks_when_flagged`: If the two tested values are not both zero, clears a 22-entry block of 32-bit words and sets a marker word to minus one.
- `advance_blitter_countdown_state`: Decrements a countdown and, when it reaches zero, reloads it and updates a state byte.
- `initialize_record_fields`: Stores the incoming register values and a computed pointer into fixed offsets of a record and clears several fields.
- `call_two_handlers_in_sequence`: Calls two subroutines in sequence and returns.
- `check_entry_hit_with_margin`: Checks whether a probe position falls inside a list entry's bounds, allowing a margin of seven units.
- `process_keyboard_serial_input`: Reads a keyboard code from the serial register once its ready bit is set, then decodes it.
- `program_blitter_from_table`: Counts down a delay, then selects a source address from a table entry and programs the blitter size, mode and pointer registers.
- `stub_return_immediately`: Returns at once with no work.
- `update_two_counters_and_words`: Updates two counters that step with a limit and reset when they pass a bound.
- `pick_shade_from_distance`: Scales two input values into a clamped range and picks a shade word from distance thresholds of 25, 10 and 4.
- `process_record_list_until_marker`: Walks ten three-word records and skips empty ones.
- `apply_offset_when_in_bounds`: Tests whether an entry lies within a bounding range around a reference point.
- `add_offset_when_in_bounds`: Tests whether an entry lies within the same bounding range used by its partner.
- `tick_timed_entries_and_reload`: Walks ten timed entries.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into -(a4).
- `advance_scroll_and_write_rows`: Returns early when the scroll step is zero.
- `update_scrolled_block_pointer`: Calculates and updates a pointer or index within a data structure used for managing scrolled graphical blocks.
- `update_scrolling_buffer_indices`: Calculates and updates internal pointers and index counters used for managing scrolled block data.
- `update_blit_scroll_parameters`: Increments internal scroll accumulators and updates the blitter configuration registers for row-based block transfers.
- `blit_scrolled_block_rows`: Advances the scroll accumulators, then programs the blitter's mask, modulo, pointer, control and size registers to copy a block of rows.
- `compute_entry_position_from_index`: Looks up two values from tables by index and stores them, shifted, into the entry's two 32-bit fields.
- `update_entry_position_or_clear`: If the index is below 17, updates the entry's two words from table values and adds a constant offset.
- `update_entries_by_state`: A long routine that branches on a state value and counters, then calls about ten helper routines to update entries.
- `apply_motion_type_to_velocity`: Clears a status word, then applies a motion type to a pair of velocity values.
- `step_entry_positions_by_type`: Returns at once when a timer is negative.
- `set_field_to_fifteen`: Sets a word to 15 when the entry's type byte equals 2, then calls a routine.
- `scan_positions_until_check_passes`: Tests successive positions, stepping by a fixed amount each time, until a check routine returns non-zero or the step limit is reached.
- `initialize_entry_if_active`: Returns when the entry's first word is zero.
- `update_entry_position_with_bounds`: Adds motion values to an entry's position, then clamps it against bounds.
- `store_clamped_coordinates_on_flag`: When a flag is set and one of two counters is negative, it clears the flag, writes a clamped pair of coordinates into the output, and zeroes the entry's two words.
- `subtract_deltas_when_flags_clear`: Subtracts two stored delta values from a pair of position words when a flag is clear, and does the same for a second pair under another flag.
- `compute_distance_squared_and_shade`: Computes the squared distance between an entry and a reference point, reduced by a shift.
- `copy_pairs_with_halved_offset`: Copies the first word of each source pair into two output tables.
- `place_position_marker_entries`: Compares a position with reference values and writes a marker character into the output buffer.
- `stop_audio_and_set_dma`: Computes a level from flags and halves it, then clears the audio channel pointers and volume values and waits in a loop.
- `advance_sequence_pointer_entry`: When the source pointer is null, stores a default pointer and a count of 2.
- `reverse_velocity_on_collision`: Checks two entries against a wall-like condition and, when a helper reports a hit, reverses that entry's velocity.
- `update_pair_state_on_hit`: Swaps two stored value pairs, then checks a type code in the range 7 to 9.
- `test_bitmap_cell_collision`: Computes a bitmap address from two coordinates, combines three 32-bit words from it, and tests a shifted bit mask against them.
- `scroll_and_build_list_entry`: Adjusts two scroll values by fixed steps and wraps them past a boundary.
- `build_list_node_from_record`: Adjusts scroll values, then fills a list node from the input record with a fixed header value, pointers and a fixed size field.
- `fill_list_node_fields`: Calls a helper and stores its word together with fixed fields into a list node: a cleared word, a minus-one sentinel, two pointers, the value 36 and the value 1731.
- `fill_list_node_masked`: Calls a helper, masks its word with a fixed value and stores it with the same fixed fields into a list node.
- `blit_active_entries_in_loop`: Loops over ten entries and, for each non-zero entry, waits for the blitter and programs its control, modulo, pointer and size registers.
- `queue_blit_node_after_wait`: Waits until a status bit clears, then writes a list node with a masked input word, a minus-one sentinel and fixed fields for the blit.
- `blit_active_entries_alt_header`: Loops over ten entries like its sibling routine.
- `compute_clamped_weighted_offset`: Computes a weighted sum from two inputs and a table value, scales it, clamps the result between two fixed limits and adds a base value.
- `update_positions_and_advance_steps`: Updates two position values from helper results when their flags are clear, then calls ten helper routines to advance the state.
- `try_advance_entry_position`: Checks a condition on an entry's position and a shared value.
- `try_advance_second_entry`: Applies the same test and step as its sibling routine to the second entry's position and counter.
- `update_countdown_flags_and_timers`: Handles two pending request flags, calling helpers and recording a pointer for each.
- `apply_velocity_damping_per_frame`: Checks two helper results and, on a hit, decrements a counter by four.
- `update_motion_from_fire_button`: Shifts the saved button flags, advances a two-step frame index and reads the fire button port.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `encode_input_bits_into_byte`: Packs the bits of an input word into a state byte.
- `decode_state_byte_bits`: Unpacks a state byte into an input word, reversing the packing done by its sibling routine.
- `read_joystick_movement_state`: Reads the mouse and joystick port movement registers and updates a stored state value from the helper results.
- `load_and_parse_text_file`: Locks a file and examines it for its size, then reads it and parses its contents into a table of entries.
- `store_masked_value_forms`: Stores a 12-bit value from the input word and sets a sign flag from its top bit.
- `allocate_tracked_memory_slot_2`: Finds the first free entry in an allocation table and calls the exec AllocMem routine with the requested size.
- `free_tracked_memory_blocks_2`: Walks the allocation table until an empty entry, freeing each live block with the exec FreeMem routine and skipping slots marked as released.
- `free_tracked_block_by_address`: Searches the allocation table for an entry matching the given address.
- `clear_joystick_state`: The routine is empty and performs no operations.
- `restore_system_state_on_exit`: Restores the system state at exit.
- `forbid_and_reply_when_flagged`: When a shared flag is set, calls the exec Forbid routine and then the ReplyMsg routine.
- `interpolate_line_points`: Computes midpoints between two endpoints and steps between them in twenty divisions, writing a point at each step.
- `build_padded_bounding_box`: Builds a box from the input corners, widened by four units on each side.
- `iterate_cells_with_stride`: Walks a grid of four rows by a set of columns, calling a routine for each cell with a fixed large stride.
- `iterate_cells_with_stride_2`: Walks a grid of five rows by a set of columns, calling a routine for each cell with a fixed large stride.
- `setup_blit_cell_registers`: Computes a blit offset from two coordinates and writes a fixed set of setup values, including the mask and size fields, to the blitter registers.
- `setup_blit_line_draw`: Computes direction flags and error terms for a line between two points, then writes the blitter's control, modulo and pointer registers to draw it.
- `format_numbered_label_text`: Builds a text label from a character, a digit derived from the argument and a source string.
- `build_text_rows_with_delay`: Copies a text string into a buffer, then loops through rows of entries calling helpers and writes a terminator.
- `decode_keyboard_serial_key`: Counts down a timer and, when the keyboard ready bit is set, reads a keyboard code from the serial register and decodes it.
- `tiny_moveq`: Small routine of 1 instruction that loads 7 into d2.
- `handle_control_code_write`: Checks a control code.
- `render_wrapped_text_line`: Lays out and draws a text string one character at a time.
- `render_wrapped_text_with_params`: Stores its layout arguments in shared words, then lays out and draws a text string with the same spacing, newline and width-limit logic as its sibling.
- `program_blitter_for_block`: Stores several values from helper results and inputs into shared words, then programs the blitter's mask, modulo, pointer, control and size registers to draw a block.
- `translate_text_control_codes`: Reads a text string and maps each character through a table of six character pairs, skipping an ampersand sequence.
- `update_timeout_and_status_bits`: Sets a timeout counter to 199 when input or control conditions apply.
- `advance_frame_and_toggle_words`: Increments a frame byte and counts down a timer.
- `multiply_absolute_values`: Takes the absolute value of two inputs, then returns a 16-bit product of the high part of one and the low part of the other, together with a rearranged second value.
- `run_steps_when_in_range`: Tests whether the difference between a position and a stored value lies between minus 16 and 107.
- `run_steps_in_second_range`: Applies the same range test as its sibling to a different pair of values.
- `initialize_scan_line_table`: Fills a table of five pointer entries that step by a fixed large stride, then fills a row of 16-bit values starting at 384 and rising by two per entry.
- `initialize_pointer_and_value_tables`: Fills four pointer entries that step by 42 bytes, then fills a second table of 15 values starting at 386 and rising by two per entry.
- `initialize_position_word_table`: Writes fixed 16-bit position and size words into a set of table slots, then runs a 544-step loop over the table.
- `compute_entry_pointers_from_index`: Clamps an index into a range derived from its inputs, then writes pointer fields in an entry, each offset by fixed steps of 42 bytes.
- `pack_entry_word_pair`: Packs a byte difference and a shifted value into a 16-bit word, adds a fixed offset, masks the high byte and sets a low bit from the input.
- `pack_entry_word_with_flag`: Adds 4096 to the input word, or 10496 when a flag is set and the target is one of two fixed entries.
- `select_flag_by_target_entry`: Chooses a flag from one of several shared values depending on which target entry is passed in, and branches to one of several paths.
- `clear_fifteen_longwords`: Writes zero into 15 consecutive 32-bit words starting after the given pointer.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `store_masked_control_bits`: Stores the masked bits of a shared input word into a shared state value, keeping only two bit positions in each byte.
- `decode_port_direction_bits`: Reads joystick and mouse port movement values and derives direction bits for each.
- `build_sprite_list_entry`: Computes a clamped horizontal offset and writes a sequence of list words for one sprite entry, including a masked control word and fixed header values.
- `build_sprite_entry_and_blit`: Computes the same clamped offset and sprite list words as its sibling, then programs the blitter registers and waits for the blitter to finish.
- `load_asset_with_blitter_locked`: Copies a file name from a table entry into a buffer, then takes the blitter and disables interrupts around the load.
- `compute_octant_index`: Converts two signed deltas into an octant-style index.
- `dispatch_pending_records`: Walks a list of twelve-word records and, for each non-zero record with matching flags, calls a routine with the record's byte value.
- `insert_record_into_sorted_table`: Walks a table of 16-bit records to find where a new record belongs, handling empty slots and negative markers.
- `check_three_bitmap_cells`: Computes a bitmap address from its inputs and reads three bitmap cells at fixed offsets, testing one bit in each.
- `insert_entry_into_list`: Walks a linked list of entries, comparing a computed key, and inserts a new entry at the first free or matching slot.

### GetPath

- `prepare_gf2_assign_and_copy`: Reads text from the dos input and checks it against a prefix length of eleven characters.
- `write_buffer_to_file_2`: Opens a file through the dos library and, if the open succeeds, writes a buffer and closes the file.
- `find_string_end`: iterates through a memory buffer until it encounters a null terminator.
- `open_console_window_handle`: Opens the dos library, then calls one of its functions, likely to open the console window named in the text, and stores the handle.
- `close_handle_and_library`: If the stored file handle is set, closes it through the dos library and then closes the exec library.

### GF2_M.DAT

- `bulk_assign_initial_values`: Stores the same input register value into about two dozen shared variables, then returns a fixed pair of values.

### GF2_SPEC.DAT

- `find_or_append_record_entry`: Searches a table of 25-word entries for a matching key.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 152 into d0, then merges 152 into d0, then merges 152 into d0.
- `copy_name_before_dot`: Copies a file name into a buffer up to the first period, then terminates it.
- `format_two_digit_percentage_text`: Writes a two-digit percentage of 100 minus a value into the text buffer.
- `write_flag_status_digits`: Writes a short status text of three dashes and a closing parenthesis.
- `build_count_text_line`: Builds a text line from helper output and a fixed text fragment ending in times.
- `format_elapsed_time_text`: Converts a time value to whole minutes and hours with rounding, then writes the hour and minute text, choosing singular or plural forms.
- `write_decimal_with_prefix`: Writes a fixed two-byte prefix, followed by a flag character when one is given.
- `clear_buffer_and_set_marker`: Clears 120 bytes at a buffer pointer, then stores a minus-two word through a second pointer.
- `write_negated_sum_checksum`: Sums a run of 32-bit values, then stores the negated sum XORed with a fixed tag into the first field of a header.
- `xor_scramble_words_backward`: Walks a block of 16-bit words backward, replacing each word with its XOR against the following word and a fixed constant.
- `xor_scramble_words_forward`: Walks a block of 16-bit words forward, replacing each word with its XOR against the next word and a fixed constant.
- `update_metadata_table`: Searches through a linked list or array of structures to find a matching entry based on a specific value.
- `lookup_bitmap_pixel_values`: XORs a shared byte with an input word and looks up a table value from its high bit.
- `blit_pixel_rows_from_table`: Performs the same table lookups as its sibling routine, then programs the blitter's mask, modulo, pointer, control and size registers before writing the rows.
- `copy_rows_with_stride`: Copies a run of 16-bit words, then repeatedly copies rows of twenty words, stepping back by one row of 42 bytes.
- `wait_while_status_bit_set`: Busy-waits while bit 6 of a shared status byte is set, then returns.
- `decode_bp_packed_image`: Checks that the input begins with the two-letter signature BP, then unpacks a packed image by reading pairs of bytes and writing them out in runs.
- `fill_memory_block_with_byte`: Fills a block of memory with a single byte value.
- `open_file_or_fill_colour`: Calls a library function to open a file.

### Gravity-Force 2 v1.0 (1994)(BiTS)(PD)[a4].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
