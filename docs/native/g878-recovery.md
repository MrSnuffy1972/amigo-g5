# Game 878 (holiday lemmings 1994 (1994)(dma design)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00878). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 62 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 269 of 269 named (0 author symbols), 269 of 269 described (25 by rule, 43 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1110 of 1110 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10010 of 10010 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 314520 |
| code | 34810 |
| data | 154146 |
| padding | 15692 |
| unused | 381952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| why | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| failat | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| HolidayLemmings1994 | 4 | 7732 | 197 |
| Holiday Lemmings 1994 (1994)(DMA Design)[cr PDX].adf:PARADOX | 5 | 752 | 22 |
| Holiday Lemmings 1994 (1994)(DMA Design)[cr PDX].adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- `restore_chip_state_from_block` (HolidayLemmings1994): Writes the saved chip state back to the DMA, interrupt and audio control registers, with interrupts disabled first and the enable bits set afterwards. Kind: loader/sabotage path (hardware-specific register/register-state check). Pass path: The routine restores or sets specific hardware register states (like custom chip registers or DMA/bus-related registers) or checks certain memory-mapped I/O values, suggesting it is part of a protection-check routine that validates the disk.
- `initialize_custom_chip_state` (PARADOX): Clears two global blocks and records the status of the chip's interrupt, DMA and display control registers, then calls routines that set up the hardware. Kind: loader-based protection/sabotage path. Pass path: The routine initializes a state where certain pointers and registers (like `_DAT_00dff09a` and `_DAT_00dff096`) or branch targets (`_DAT_00000068`, `_DAT_0000006c`) are set to specific or masked values, likely preparing the environment for .

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longwords_to_pointer`: When the output pointer is non-null, copies sixteen longwords from a fixed offset in the base record into it.

### type

- `find_max_and_build_table`: Scans a linked list for the largest stored value, starting from a floor of 149, and uses it to size a new table.
- `copy_fixed_size_block`: copies a fixed number of words from a memory location or table into a stack-based or local buffer.

### why

- `update_linked_list_and_buffer`: Traverses a linked list to find the maximum value in a field, then updates a base record with this maximum and writes a header or copies a block of data if certain conditions or output pointers are met.
- `copy_buffer_block`: copies a fixed number of words from a memory location to a stack-based or temporary buffer.

### echo

- `find_max_and_initialize_buffer`: Iterates through a linked list to determine the maximum value in a specific field, then initializes a target buffer with a header and copies a fixed amount of data if a destination pointer is valid.
- `copy_data_to_stack`: copies a fixed number of words from a memory location or structure offset to the current stack pointer.

### failat

- `update_list_max_and_expand_header`: Traverses a linked list to find the maximum value in a record field, then writes this value to a header and expands a subsequent data table or header block.
- `copy_list_header_elements`: copies a fixed number of dwords from a memory-resident structure to a stack-allocated buffer.

### cd

- `update_max_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field and subsequently writes a header and copies a block of data or a constant-incrementing sequence into a destination buffer.
- `copy_data_block`: Copies a fixed number of words from a source address to a destination address if a pointer is non-null.

### IconX

- `parse_startup_command_arguments`: Program entry that allocates a process record and parses its argument text into words, unescaping quoted text and the star-N and star-E escape codes.
- `release_handles_and_free_state`: Stores the given value in the process record, closes an open file handle and library, and when a message record is present calls two exec vectors to forbid and reply.
- `prepare_icon_render_buffer`: Looks up dimensions through the icon.library entries, allocates a 1536-byte block and fills it with a pattern for each plane, then copies the result with a block copy.
- `dispatch_icon_request_command`: Builds a command line in a buffer and, for a single-dot response, appends the parameters and issues a follow-up call that depends on flags.
- `initialise_library_and_enable`: Creates a library from the given structure, adds its function table with the second and third arguments and enables it.
- `build_icon_path_from_list`: Walks a linked list of name entries and appends each printable name to a path string with separators.
- `join_three_string_arguments`: Calls routines on three string arguments in turn, using the length returned by one routine to find the end of the joined text, and then calls another routine there.
- `count_string_length`: Counts the bytes before the zero terminator and returns that count.
- `copy_null_terminated_string`: Copies bytes from the source to the destination one at a time, up to and including the zero terminator.
- `call_helper_at_string_end`: Advances to the zero terminator of the string and passes that end pointer and the second argument to a routine.
- `parse_signed_decimal_value`: Parses an optional leading plus or minus sign followed by decimal digits from a string and returns the signed integer value.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_with_stack_slot_argument`: Calls another routine with its two arguments and a pointer to a stack slot as a third argument, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_text_into_stack_buffer`: Calls the exec library's RawDoFmt vector through a global pointer to format text into a 140-byte stack buffer.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_raw_format_vector`: Jumps through the exec library's RawDoFmt vector using the stored base, passing the caller's arguments unchanged.
- `store_byte_at_output_pointer`: Stores the low byte of the data register at the address held in the address register and returns.

### HolidayLemmings1994

- `run_main_frame_loop`: Program entry that sets a colour register and the DMA enable bits, then runs a frame loop.
- `save_and_reset_chip_interrupts`: Saves the DMA, interrupt and audio-control register values into the state block, then disables all interrupts.
- `restore_chip_state_from_block`: Writes the saved chip state back to the DMA, interrupt and audio control registers, with interrupts disabled first and the enable bits set afterwards.
- `open_console_and_report_errors`: Saves two state words and opens a console window through the DOS Open vector.
- `tail_call_exec_free_mem`: Jumps through the exec library's FreeMem vector using the stored base, passing the caller's arguments unchanged.
- `drive_port_around_helper_calls`: Writes a port value, calls a routine, writes a second value and calls the routine again, then writes the first value back.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `load_assets_run_frame_loop`: Opens graphics.library, reads resource data through DOS and allocates memory, then runs the frame loop.
- `forward_to_helper_routine`: Calls one routine with no arguments and returns.
- `advance_timed_sequence_step`: Advances a step counter unless the disable flag is set.
- `find_entry_near_point`: Scans a table of 22-byte entries and counts those whose position falls inside a small window around a reference point.
- `update_flag_from_bit_two`: Stores a fixed word in the record and sets a global flag to all ones when bit two of a status byte is clear, otherwise zero.
- `process_position_input_state`: Handles one input event for a position record, using horizontal and vertical limits and a state code from zero to three to decide what to update.
- `increase_value_to_limit`: Raises a 16-bit value at offset four by four, or by sixteen when a flag is set, stopping before 1280.
- `decrease_value_to_zero`: Lowers a 16-bit value at offset four by four, or by sixteen when a flag is set, without going below zero.
- `empty_routine`: the routine is empty and performs no instructions or data manipulations.
- `set_fixed_word_and_call`: Sets a screen-related word in the record to a fixed value and calls a routine.
- `store_fixed_word_then_call`: Stores a fixed word at offset 94 of the record and calls a routine.
- `compute_row_byte_offset`: Returns the byte offset of a row, computed as the index minus one multiplied by 44.
- `pack_row_column_words`: Combines a row index and a column value into a 64-bit result of two words.
- `pack_row_column_alt_b`: Combines a row index and a column value into a 64-bit result of two words, as the sibling packing routine does, but with the second word offset by 6144 instead.
- `pack_row_column_alt_c`: Combines a row index and a column value into a 64-bit result of two words, as the sibling packing routine does, but with the second word offset by 47232 instead.
- `return_empty_subroutine`: The routine is empty and performs no operations before returning.
- `update_global_value`: Writes the return value of a routine to a global variable and then sets another global variable to that value plus a large constant.
- `tick_flag_countdown`: Counts down a timer word while a toggle byte is clear.
- `handle_input_event_code`: Dispatches a character event code: digit characters set a selector and call a routine, z and x step the selector cyclically, and p or return toggles a flag.
- `draw_text_glyphs_to_bitplanes`: Renders a byte string of glyphs into four bitplanes until the end marker of 255.
- `acknowledge_pending_interrupt_event`: Reads a shared status byte and, when its low bit is set, calls through a stored routine pointer, then resets the byte to a fixed value.
- `decode_keyboard_serial_keycode`: Reads a raw key byte from the keyboard serial data register, handshakes through the CIA control register, and stores the decoded key state in a table, using a keyboard layout string to map each code.
- `split_value_at_ten_thousand`: Divides the 32-bit value in D0 by ten thousand and packs the quotient and remainder into one word, passes it to a routine, then calls a second routine.
- `convert_high_word_to_digits`: Takes the upper 16-bit word of the value in D2 and extracts its four decimal digits by repeated division by ten, returning them packed as ASCII characters in one 32-bit result.
- `shift_table_pointers_by_plane`: Flips a toggle byte, then writes four entries of low and high word pointers into a table, each pointer advanced by a fixed block size from the previous one.
- `wait_for_fire_button_press`: Loops calling a routine and reads the CIA port A data register until one of the fire-button bits goes low.
- `decrement_countdown_clock_fields`: Increments one counter, calls two routines, then decrements a three-field countdown.
- `clear_flag_then_wait`: Clears the byte at the address held in the A5 register, then spins until another path sets that byte to a nonzero value.
- `wait_for_flag_nonzero`: Repeatedly calls a routine until its return value is zero.
- `decrement_pending_count_and_fill`: When a pending count is nonzero, decrements it and writes three runs of words through a routine call, of thirty-two, eight and sixteen entries.
- `fill_nested_word_block`: Runs an outer loop of sixteen passes that calls a setup routine, then an inner loop of sixteen words per row, each produced by a routine call and stored at a fixed offset of an output pointer.
- `fill_rows_of_word_entries`: Runs sixteen outer passes that call a routine with the global block pointer, then write thirty-two words per pass through a routine call, each stored at a fixed offset of an output pointer.
- `step_packed_nibbles_toward_target`: Compares the low and middle fields of two packed words and moves the second value one step toward the first in each.
- `store_global_pointer_after_call`: Calls a routine, then stores the address of the global block into two pointer fields, one addressed through the A6 register and one through the A5 register.
- `set_global_pointers`: Calls another routine and then stores the address of a global variable into two different memory locations relative to registers.
- `write_to_pointer_offsets`: Stores the address of a global variable into two different offsets within memory structures pointed to by registers.
- `initialize_table_of_address_words`: Stores a series of fixed word constants into consecutive global table entries, several of them as pairs sharing an upper word, and zeroes the remaining slots.
- `build_stepped_pointer_tables`: Fills one table with four low and high word pointer pairs spaced by a large fixed step, then a second set of four pairs with a smaller step.
- `set_fixed_pointer_pair_words`: Writes four fixed low word and high word pairs into the global pointer slots and returns the input value in D0 unchanged.
- `set_alternate_fixed_pointer_pairs`: Writes four different fixed low word and high word pairs into the same global pointer slots and returns the input value in D0 unchanged.
- `write_four_stepped_pointer_pairs`: Writes four low and high word pointer pairs into a table, each pointer advanced by a fixed step from the previous one.
- `copy_longs_into_duplicated_bands`: Copies source longs into a destination area in three banded passes, storing each value at two or four offsets so the same data appears in several places.
- `copy_words_into_paired_offsets`: Copies source longs in two bands of three groups, writing each value at the base offset and again ten longs later.
- `zero_global_long_block`: Zeroes a large block of the global area, clearing two longs on each loop pass until the counter underflows.
- `update_timer_state`: Saves the current values of two 16-bit fields into backup memory slots and updates those fields with the results of two other routine calls.
- `save_fields_and_store_results`: Saves two 16-bit fields into the previous-value slots, then stores results from two routine calls into global cells.
- `update_clamped_position_from_delta`: Stores the new packed input in a record field and moves two 16-bit coordinates by the byte differences from the previous value.
- `update_progress_percentage`: calculates a ratio or percentage-based value and updates a status flag or record field.
- `update_percentage_and_status_flag`: Calls a series of routines, computes a percentage of one field divided by another and stores it in a record field.
- `select_role_label_from_status`: Chooses a fixed label such as ATHLETE, FLOATER, CLIMBER, WALKER, DIGGER, BUILDER, BLOCKER, BASHER, MINER or BOMBER from the bits of a status word and writes it into the output buffer.
- `format_three_digit_counter_text`: Writes three decimal digit characters packed in a 32-bit value into a text buffer.
- `format_two_digit_counter_text`: Writes two decimal digit characters from a 16-bit value into a text buffer.
- `convert_word_to_packed_digits`: Converts the 16-bit value in D0 into four decimal digit characters by repeated division by ten.
- `render_two_digit_glyphs`: For a nonzero value, splits it into tens and units and copies each digit's seven glyph rows from a table into the output at a fixed row spacing.
- `advance_record_counters_with_limits`: Loops over thirty-two records, skipping inactive ones and returning early when a control byte is set.
- `call_routine_for_active_records`: Walks thirty-two records and calls a routine for each one whose first word is nonzero.
- `zero_global_word_block`: Clears a large block of words in the global area, one word per loop pass, until the counter underflows.
- `fill_entry_rectangles_by_index`: For each of sixteen entries with a nonzero first word, computes a destination from its packed coordinates and an offset table.
- `fill_record_rectangles_with_constant`: Loops over thirty-two records and, for each nonzero one, fills a rectangle of bytes with the constant nine.
- `reload_timer_and_claim_slot`: Counts down a timer and, when it expires, reloads it and claims the next slot from a four-entry pointer list.
- `refresh_display_fields_on_change`: When a level counter differs from its last value, it recomputes the display field and writes a digit character into a text buffer, then calls routines to redraw it.
- `load_palette_from_colour_chunk`: When a flag is set, searches memory for the four-character colour map chunk identifier, then converts its three-byte colour entries into packed four-bit colour words.
- `copy_selected_frame_block`: Copies 512 longs from one of four stored blocks into the global area, choosing the block from a table index minus one, modulo four.
- `set_pointer_to_indexed_block`: Stores into a record field a pointer to a block in the global area, offset by a stored index times a fixed block size.
- `initialize_record_fields_from_globals`: Calls routines, then copies several global values into record fields, derives one field from a subtraction of halved offsets, and sets a long from a routine result.
- `copy_record_fields_and_clear`: Copies nine 16-bit fields from a source record into a destination record, then clears four counter words and a flag byte in the destination.
- `walk_list_then_call_records`: Calls setup routines, then either walks a minus-one terminated list calling a routine for each entry or makes a single call.
- `walk_terminated_list_then_setup`: Calls two routines, then either walks a minus-one terminated list calling a routine for each entry or makes a single call.
- `copy_record_rows_from_table`: For each nonzero record among thirty-two, copies thirty-four bytes from a table entry indexed by the record's second field into an output row.
- `setup_blit_clipping_parameters`: Calculates a row offset and width for an image before selecting a blit path based on flag bits in a register.
- `start_blit_after_busy_wait`: Sets a pointer and several fields to fixed values, then writes the same blit size word four times, waiting for the busy flag to clear before each write.
- `fill_scaled_index_table`: Fills a table of 184 words with each index multiplied by a fixed scale, then zeroes forty-eight words at a fixed absolute address.
- `format_percentage_saved_status`: Computes a percentage from two counters and writes it into a status line, with a plural suffix character chosen by whether a count is one.
- `copy_long_run_until_wrap`: Copies a large block of words from a pointer stored in a record into the global area, running until a 16-bit counter wraps around to minus one.
- `call_two_routines_in_sequence`: Calls two routines one after the other and returns, with no other work.
- `initialize_bitmap_parameters`: Computes the row byte count or a row offset and width, and stores these values or the bitmap dimensions into global variables.
- `collect_active_record_positions`: Collects the positions of records whose flag is one, packing each as a pair of words.
- `find_first_marked_record_value`: Searches thirty-two records for the first one whose marker field equals two, then copies its stored value into two words of the calling record and returns.
- `scramble_and_checksum_code_bytes`: Packs bits from several record fields into a sequence of bytes and adds a checksum nibble derived from their sum.
- `wait_for_video_position_changes`: Waits for the high byte of the video position counter to change, once for each count held in D0.
- `latch_fire_button_state`: Updates a latch byte from the fire-button input bit: it clears the latch when that bit is low, and sets it to all ones when the latch was empty and the bit is high.
- `blink_text_cursor_character`: Reads a character from a table at a stored offset.
- `read_file_or_report_error`: Calls DOS library routines to open and read a named file, and writes an error message with the file name through the output routines when the read fails.
- `copy_counted_byte_run`: Copies a counted run of bytes one at a time from the source pointer to the destination pointer.
- `run_code_entry_with_checksum`: Runs a nine-character text entry loop with dash fillers, handling delete, backspace and cursor keys.
- `configure_window_and_draw_calls`: Sets a window width of eighty and a height of two hundred eight, stores a pointer to the global buffer and marks a flag byte, then calls sixteen routines.
- `set_panel_marker_character`: Writes the character one into a global text slot, clears a flag byte in the record, and calls two routines.
- `advance_four_bit_frame_timers`: Loops over seven eight-byte records.
- `step_animation_records_with_delay`: Advances a four-bit frame counter on one record, reloading its delay on wrap, then walks the following records and decrements each nonzero delay until the count held in a register runs out.
- `initialize_string_input_buffer`: Sets up a global buffer with a default sequence of dash characters and initializes a checksum-related string entry routine.
- `initialize_text_box_record`: Sets two window size fields and a buffer pointer, and a limit of two hundred.
- `pass_through_routine_result`: Calls one routine and returns its result pair unchanged.
- `copy_column_strips_to_output`: Copies a sixteen-word column from the global area into output rows spaced eighty words apart, repeating over four word columns for each of eighty row groups.
- `draw_text_glyphs_into_planes`: Renders a terminator-ended list of text strings into a multi-plane output.
- `render_glyph_bitplanes`: Iterates through a string of glyphs and writes their bitplane patterns into a destination buffer.
- `update_bitmap_bits_from_records`: Zeroes a table of nineteen fixed records, then for each active record clears one bit in a first bitmap and sets it in a second bitmap, with the bit position taken from the record's coordinates.
- `build_pointer_list_from_widths`: Builds a sixteen-entry pointer list from a width table, advancing each pointer by the entry's width or repeating the previous pointer for empty entries.
- `invoke_basic_fx_handler`: Calls one routine and then calls through a stored function pointer, the same pointer used by the neighbouring effects routine.
- `invoke_full_fx_handler`: Calls one routine and then calls through a stored function pointer, the same pointer used by the neighbouring effects routine.
- `toggle_flag_then_call_handler`: When a flag byte is set, toggles one of two other flag bytes depending on whether a position is past a threshold.
- `decode_backward_bitstream_runs`: Decodes a compressed stream read backward from its end, using a bit buffer with a sentinel bit to choose between literal bytes and back-reference runs.
- `consume_bits_from_stream`: Shifts a bit buffer right by a counted number of bits and refills it from the preceding stream word, setting a sentinel bit, when the buffer empties.
- `unpack_iff_body_four_planes`: Finds the BODY chunk identifier in memory, then unpacks its byte-run data into four planes of eighty bytes per row.
- `or_three_bitplanes_together`: Combines three source bitplanes into one destination by bitwise OR, one long at a time, across a fixed number of longs.
- `unpack_iff_body_five_planes`: Finds the BODY chunk identifier in memory and unpacks its byte-run data into five planes of forty bytes per row, using the same run rules as its four-plane neighbour.
- `unpack_iff_body_from_pointer`: Unpacks byte-run data from the pointer in its input register into the plane buffer, without searching for the BODY chunk identifier first.
- `poll_video_position_with_timeout`: Polls a video position value for up to a fixed number of passes, setting a flag when it never passes a threshold.
- `call_sequence_of_routines`: Executes a long sequence of unrelated routine calls and returns the input register pair unchanged.
- `unpack_leveldata_bitstream`: Decodes a compressed stream read backward from its end into the output, copying literal and back-reference runs and accumulating a checksum with exclusive-or.
- `step_records_against_terrain_bits`: Advances each active record's horizontal position in steps and tests the terrain bitplane bits at the new position.
- `update_record_state`: The routine is empty and performs no operations.
- `walk_records_with_countdown`: Decrements a countdown on the current record and, when it expires, initialises the record's fields from a flag mask.
- `reset_record_motion_fields`: Sets a flag word from the input value with a fixed bit pattern, sets two signed offset fields to fixed negative values, and clears two counter fields.
- `mask_word_across_four_planes`: ANDs one destination word with a mask and applies the same mask to three words at fixed plane offsets, clearing the bits outside the mask in all four bitplanes.
- `draw_three_by_three_block`: Computes a destination from the record's packed coordinates and copies a three-row block of three bytes each from the record's stored pixel fields, using a fixed row stride.
- `apply_terrain_tile_behaviour`: Looks up a behaviour byte for the record's position in a tile map and stores it in the record.
- `initialize_scroll_bounds_and_sizes`: Stores a global value in a scroll field, sets two offsets to minus two and two size fields to forty, clears two counters, sets a limit of three hundred thirty-six, then calls a routine.
- `tally_and_project_visible_records`: Tallies active records into two counters by the sign of their vertical field.
- `mark_plane_three_then_scan`: Clears the selected bit in three bitplanes and sets it in the third, then walks the record list, tallying records by sign and handling those inside the clipping range.
- `mark_plane_two_then_scan`: Clears the selected bit in three bitplanes and sets it in the second, then walks the record list, tallying records by sign and handling those inside the clipping range.
- `update_lemming_visibility_and_bounds`: Updates bitmasks in a global buffer based on a register value and iterates through a list of active records to increment counters or perform indirect calls if certain position or type criteria are met.
- `update_entity_vertical_positions`: Iterates through a list of active records to update or validate vertical or horizontal position offsets and increments counters based on the sign of a vertical field.
- `set_planes_one_and_three`: Sets the selected bit in the first and third bitplanes and clears it in the second and fourth, then walks the record list, tallying records by sign and handling those inside the clipping range.
- `setup_blit_for_record`: For a record whose type is not one of three excluded types, clamps its coordinates, computes a destination address and a control word, and writes the blitter parameters into the custom chip block.
- `blit_four_planes_with_wait`: Computes a source offset and bit shift from a value, then starts a blit of four planes with the same size word.
- `draw_string_glyph_rows`: Draws a terminator-ended string into a bitplane buffer, looking up each character's glyph in a font table and copying its rows at a fixed row stride.
- `process_twenty_five_entries`: Calls one routine for each of twenty-five consecutive table entries, advancing the source by one word and the destination by eight bytes per entry.
- `pack_mask_bits_per_row`: Advances a twenty-five-step counter, then builds one output byte for each of twenty rows by setting a bit for each of eight source bytes whose masked bits are nonzero.
- `pack_mask_bits_from_pointers`: Packs the masked bits of eight source bytes into one output byte per row for twenty rows, with the source and destination pointers passed in registers instead of taken from the record.
- `clear_four_parallel_word_tables`: Clears four parallel word tables in one loop, with three of them at fixed offsets from the second base, for four hundred eight entries.
- `pack_bitplane_longs_to_bytes`: Converts a grid of longs into bytes by gathering every fourth bit of each long into one byte.
- `run_blitter_size_passes`: Sets blitter fields to fixed values, then writes the same size word four times, waiting for the busy flag to clear between passes.
- `or_edge_mask_into_planes`: Computes an offset from a record's position and ORs an edge mask into the word at that offset in two or more planes.
- `or_low_mask_into_planes`: ORs a low-byte edge mask into two bitplane words at a register offset.
- `or_high_mask_pair`: ORs a high-byte mask into two bitplane words at a register offset.
- `or_low_mask_plane_pair`: ORs the low-byte edge mask into two fixed bitplane words at a register offset.
- `or_low_mask_mixed_planes`: ORs the low-byte edge mask into a global block word and a second bitplane word at a register offset.
- `or_high_mask_plane_pair`: ORs a high-byte mask into two fixed bitplane words at a register offset.
- `advance_scroll_and_blit_strip`: Advances a scroll position by four units up to a limit, computes a source pointer and shift from it, then blits a strip of the global buffer and waits for the blitter.
- `advance_five_bit_counter`: Increments a field and keeps it within a five-bit range, then calls two routines.
- `store_bitmap_dimensions`: Stores the width and height inputs in globals, computes the row byte count as the width divided by eight, and stores the product of row bytes and height as the total size.
- `store_pair_of_globals`: Stores the two 16-bit inputs into two global words.
- `setup_clipped_blit_path`: Computes a row offset and width for an image, then chooses one of several blit paths from flag bits in a register.
- `count_tiles_in_clip_range`: Counts how many sixteen-pixel columns a horizontal span covers within the clip bounds stored in the record and stores the count.
- `call_six_routines_passthrough`: Calls six routines in sequence and returns its input register pair unchanged.
- `store_channel_sample_pointers`: Looks up a channel's table index from its number, stores half of two length fields from the source record, and stores a sample pointer offset by a fixed header size for that channel.
- `start_audio_channels_from_mask`: For each of four channel bits in the input, sets or clears the matching bit in a shadow byte.
- `call_four_routines`: Calls four routines in sequence and returns.
- `call_routines_when_flag_clear`: Calls three routines, then four more only when a global word is zero.
- `set_global_status_then_return`: Calls one routine, sets a global status byte to one and then to fifteen, and returns the input register unchanged.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `spin_writing_colour_register`: Repeatedly writes a fixed colour value to the colour zero register of the custom chip block, looping forever.
- `verify_words_sum_or_nop`: Adds four stored words and compares the total with a fixed value.
- `program_cia_timer_sequence`: For input values zero and one, points two buffer fields at the global area and writes the same five-byte sequence into the CIA timer and control registers.
- `clear_record_block_and_limits`: Stores a pointer in a record field, clears a block of record bytes, and sets four 16-bit fields to sixty-three.
- `call_three_routines_passthrough`: Calls three routines in sequence and returns its input register unchanged.
- `update_sample_pointers`: Calculates an offset using a value from a table and stores shifted register contents into a memory location.
- `search_for_four_byte_pattern`: Scans memory forward from the current pointer up to an end pointer, looking for the four bytes in the input register, and returns at the first match or at the end.
- `execute_nop_instructions`: performs a sequence of no-operation instructions without modifying any registers or memory.
- `call_routine_per_string_byte`: Calls one routine for each byte of a zero-terminated string, then returns the input register unchanged.
- `return_stack_argument`: Returns the value passed on the stack without any other work.
- `store_channel_header_bytes`: Looks up a channel's table index, stores three bytes from the pointer into that channel's entries, then points four channel pointers at the position nine bytes before it.
- `traverse_linked_list`: Iterates through a linked list of pointers, calling another routine for each node, until a null pointer is encountered.
- `return_stack_value`: Returns a value previously stored on the stack.
- `execute_two_subroutines`: Executes two other subroutines in sequence.
- `call_four_routines_passthrough`: Calls four routines in sequence and returns the input register value unchanged.
- `reset_record_head_fields`: Copies a word from a fixed slot into the first record word, sets one byte to all ones, and clears two more bytes in the record.
- `copy_channel_bytes_when_valid`: Calls four routines, then copies two stored bytes for a channel to two output locations, unless the channel's first byte is minus one.
- `reset_fields_and_rotate_byte`: When bit zero of the input byte is set, stores a pointer and clears a word and a byte in the record.
- `call_five_routines_passthrough`: Calls five routines in sequence and returns the input register pair unchanged.
- `store_sample_bytes_in_record`: When bit zero of the second input is set, copies two bytes of the first input into record fields, copies a third byte unless a flag is set, and clears a further field.
- `start_audio_channels_from_mask_2`: Uses the input mask to set or clear shadow bits for four audio channels.
- `execute_four_subroutines`: Executes four other subroutines in sequence.
- `set_record_sample_state`: When bit zero of the second input is set, sets a record flag byte to one, stores the low input byte in two record bytes, and sets a fourth byte to all ones.
- `execute_conditional_routine_calls`: Executes a sequence of routine calls if a global variable is zero, otherwise returns a concatenated register value.
- `cycle_countdown_from_five`: Steps a field down by one on each call, wrapping from zero back to five.
- `populate_record_from_table`: copies three bytes from a source table into a record structure.
- `copy_table_bytes_into_record`: When a record flag is clear, copies three bytes from a table at the input offset into record fields.
- `decrement_counter_and_advance_pointer`: Decrements a byte-sized counter and, if the counter reaches zero, increments a table pointer by a fixed offset of twelve.
- `advance_table_pointer_every_count`: Counts a byte down, and when it reaches zero advances a stored table pointer by twelve.
- `update_field_and_rotate`: Checks a condition and, if non-zero, copies an eight-byte record to a destination, stores an input word, clears the field, and sets a flag byte.
- `copy_record_and_rotate_left`: When a field is nonzero, copies the eight-byte record into the destination, stores the input word, clears the field and sets a flag byte.
- `execute_empty_sequence`: executes a sequence of recursive calls to itself before returning.
- `step_bounded_value_on_countdown`: Calls a routine, then when an enable byte is set decrements a frame countdown.
- `emit_record_pointer_or_default`: When a record flag is clear, stores the record's pointer and count into an output slot if the count is nonzero.
- `execute_five_routines`: Calls a sequence of five routine calls, including a routine that loads a pointer and associated fields from a table into a record using a counter-based index.
- `load_next_record_pointer_entry`: When a counter byte is nonzero, uses it as an index into a list to load the next entry's pointer and fields into the record, then clears the counter.
- `dispatch_routine_from_table`: Reads a byte from a record, clears it, and calls a routine selected from a table.
- `dispatch_routine_from_table_byte`: Reads a selector byte from the record, clears it, and calls the routine chosen by that byte from a table.
- `configure_audio_dma`: Sets or clears shadow bits for audio channels and modifies DMA or interrupt controller registers.

### Holiday Lemmings 1994 (1994)(DMA Design)[cr PDX].adf:PARADOX

- `initialize_custom_chip_state`: Clears two global blocks and records the status of the chip's interrupt, DMA and display control registers, then calls routines that set up the hardware.
- `build_copper_list_entries`: Writes register and value word pairs into a list in a loop, covering display control registers and colour registers, and sets three global words to fixed values before the loop.
- `call_helper_twice_then_return`: Calls a helper routine twice in sequence and then returns.
- `repack_low_word_nibbles`: Rearranges the nibbles of the low word and merges in bits from the high word, returning the result masked to twenty bits.
- `return_empty_entry_point`: The routine body contains only a return instruction.
- `advance_buffer_fill_state`: Steps a small state machine that fills a buffer.
- `return_without_action_entry`: The routine body contains only a return instruction despite a call to another routine.
- `store_plane_pointer_pairs`: Stores the low and high words of a pointer, and of that pointer advanced by forty-four bytes, into two pairs of global pointer words.
- `restart_copper_list_at_raster`: When a flag bit is set, points the second copper list at its start and waits until the raster position passes a fixed line before restarting the copper.
- `spin_forever_when_flag_set`: Enters an infinite loop if a specific bit in a global variable is set, otherwise updates that variable and returns.
- `build_offset_sum_table`: Sums two table entries into a position field, then fills a table of twenty-four or forty entries, depending on a flag, with values derived from those sums.
- `xor_plot_bit_rows`: Draws forty-four rows of eight bit positions by exclusive-or into two bitplane addresses per bit, advancing through computed offsets taken from table entries.
- `swap_double_buffer_pointers`: Swaps two stored buffer pointers, then recomputes the pointer words for the front and back buffers, one offset by forty-four bytes and the other by a stored row offset.
- `initialize_cia_timer_interrupts`: Stores a pointer and a word from a record, then programs the second CIA timer and interrupt registers with a sequence of control words.
- `write_interrupt_control_sequence`: Clears one flag bit and writes a fixed sequence of control words to the interrupt enable and DMA registers, setting the low four bits last.
- `scale_four_values_into_record`: Scales four byte values by the global multiplier and a shift of six, storing each result in a record word at a fixed step.
- `increment_global_byte`: Adds one to a global byte.
- `store_scaled_step_delta`: Computes a scaled value of eight thousand from the global, then stores in a record byte the difference between the scaled values of the next and current counter steps.
- `run_indexed_handler_countdown`: Returns zero when a global flag is clear.
- `write_cia_timer_control_words`: Writes two fixed values and a table-based value to the CIA control area, then writes an interrupt request bit to the custom chip block.
- `process_routine_without_pseudocode`: The decompiler timed out, so no pseudo-code is available.
- `advance_position_by_velocity_byte`: Copies a nonzero byte from one record field into another, then moves a 16-bit position by the scaled negative of that byte and a 32-bit value by the byte multiplied by 256.

### Holiday Lemmings 1994 (1994)(DMA Design)[cr PDX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
