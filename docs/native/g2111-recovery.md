# Game 2111 (ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02111). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 528 of 528 named (0 author symbols), 528 of 528 described (14 by rule, 161 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2060 of 2060 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15590 of 18651 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 280641 |
| code | 63540 |
| data | 511518 |
| padding | 9645 |
| unused | 35840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:demo | 22 | 14478 | 505 |
| endcli | 2 | 117 | 2 |
| Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro | 1 | 4041 | 19 |
| Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |

## Cases handled

- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:INTRO_3.NEW: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:mainmenu.new: high-entropy data file (7.64 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:INTRO_1.NEW: high-entropy data file (7.85 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro_1a.new: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:logo.new: high-entropy data file (7.70 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:DUMP0: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:DUMP2: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:DUMP4: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro_2.new: high-entropy data file (7.65 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:gypsy.new: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:vellum1.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:f&b.new: high-entropy data file (7.66 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:INTRO_2A.NEW: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:title.new: high-entropy data file (7.77 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro_1b.new: high-entropy data file (7.75 bits/byte), classified as packed data
- ultima vi - the false prophet (demo) (1992)(mindscape)[h trsi]/Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:blocks.new: high-entropy data file (7.90 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro

## Copy protection found

- none found

## Functions


### Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:demo

- `start_program_with_command_line`: Clears a block of roughly two thousand longwords, then stores library and interface pointers in the state block.
- `find_task_and_initialise`: Looks up a task through the exec FindTask library routine and stores its data pointers in the state block.
- `shutdown_and_reply_message`: Calls a stored cleanup pointer when it is set and calls two routines.
- `open_or_run_cleanup`: Calls a library routine at a fixed offset and stores its result.
- `call_two_table_entries`: Calls two library routines through the base register, one after the other, and returns.
- `stub_empty_return`: Returns to the caller immediately without doing anything.
- `select_dma_channel_mask`: Looks up a channel record from a table using the low bits of the argument.
- `loop_clear_global_flag`: Calls a routine sixteen times in a countdown loop from fifteen to zero, then clears a global flag variable.
- `start_channel_from_table_entry`: Looks up a table entry by index and returns if it is empty.
- `fill_channel_record_fields`: Calls a routine and stores its returned pointer and length into a channel record, sets the record's count fields from the arguments, and when a flag byte is nonzero clears the channel's counters and calls a routine.
- `store_channel_count_and_clear`: Stores the count and address-derived values into a channel record, and when a flag byte is nonzero clears the channel's counters before calling a routine.
- `reset_channel_fields_from_entry`: Stores a value into a channel record, sets the channel's fields from the arguments and a table entry, clears its counters, and calls a routine when a flag byte is nonzero.
- `select_channel_entry_and_reset`: Records a channel index and entry pointer, then either stores the entry's values from the arguments or fills the channel fields from a global table.
- `load_channel_from_global_table`: Fills a channel record's fields from a global table indexed by the argument, clears the channel's counters, and calls a routine.
- `advance_channel_command_stream`: Decrements a channel's tick counter and, when it expires, reloads it and runs commands from the channel's byte stream through a sixteen-way switch that updates volume, offsets and counters.
- `advance_channel_stream_entry`: Runs commands from a channel byte stream starting at the argument position through the same sixteen-way switch, then updates the channel's tick counters and period fields.
- `run_channel_command_loop`: Runs commands from a channel byte stream through the sixteen-way switch until a byte without its top bit is reached, then updates the channel's counters and period fields.
- `update_channel_tick_fields`: Stores a word in the channel's position field, advances its tick counter and, when it expires, runs the next command from the stream.
- `dispatch_channel_stream_command`: Runs one command from the channel byte stream through the sixteen-way switch, then updates the channel's sequence position and its volume and period fields.
- `read_channel_stream_until_command`: Runs commands from the channel byte stream until a byte with its top bit set is reached, then stores a table pointer in the channel record and updates its position fields.
- `apply_channel_volume_and_period`: Stores a position word in the channel record.
- `enter_infinite_loop`: Contains no work and loops forever, so it never returns to its caller.
- `conditional_pointer_copy`: When bit zero of the input is set and a stored value in the structure is nonzero, copies that value and a 16-bit field to the destination pointed to by another field.
- `process_flagged_channel_update`: When a global longword is nonzero, calls four routines, performs the same conditional copy as the bit-zero check, and returns the input shifted right by one bit.
- `step_channel_record_sequence`: Advances a global step counter and restarts it when it reaches the stored length.
- `step_channel_records_from_entry`: Performs the same per-channel record update as the sequence step, starting from the pointer argument and clearing the first output byte.
- `resume_channel_sequence_dispatch`: Enters the channel sequence loop at its command handler, handling the special codes 242, 254, 253 and 255, then advances the per-channel record loop.
- `dispatch_channel_sequence_commands`: Enters the channel sequence loop at its special-command checks for codes 254, 253 and 255, then advances the per-channel record loop.
- `advance_channel_sequence_records`: Enters the channel sequence loop at its record-advance step, handling codes 253 and 255 before reading the next three-byte record for each channel.
- `read_channel_sequence_records`: Enters the channel sequence loop at the record read, handling code 255 and then reading the next three-byte record for each channel and dispatching its command nibble.
- `increment_sequence_global_counter`: Increments a global counter, then runs the channel sequence loop, reading three-byte records and dispatching the command nibble for each channel.
- `apply_channel_effect_command`: Applies a command effect to the current channel for command codes fourteen, twelve, nine, eight, eleven and three, then continues the channel sequence loop.
- `apply_channel_effect_subset`: Applies command effects twelve, nine, eight, eleven and three to the current channel, then continues the channel sequence loop.
- `apply_channel_timing_commands`: Applies command effects nine, eight, eleven and three, which set timing, nibble fields, a position value and a table pointer, then continues the channel sequence loop.
- `apply_channel_nibble_commands`: Applies command effects eight, eleven and three to the current channel, then continues the channel sequence loop.
- `apply_channel_position_commands`: Applies command effects eleven and three, which set a position value and a table pointer, then continues the channel sequence loop.
- `apply_channel_table_pointer_command`: Applies command three, which stores a table pointer selected by an offset in the channel record, then continues the channel sequence loop.
- `run_channel_sequence_from_command`: Enters the channel sequence loop at the command decode, reading three-byte records and dispatching the command nibble for each channel.
- `dispatch_channel_position_effects`: Applies per-channel position effects from a command nibble, switching on the doubled command value.
- `finish_channel_effect_dispatch`: Updates a channel's delay field and, when its flag byte is zero, sets a bit in a global channel mask.
- `gather_channel_command_bytes`: Walks up to four channel records, copying a command byte from each into a register and decrementing its counter.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `lookup_or_divide_table_entry`: Looks up a 16-bit value in a global table when the index is below eleven, otherwise divides a global constant by the index.
- `lookup_or_divide_table_entry_2`: Checks if an input index is within bounds to read a 16-bit value from a global table; otherwise, it divides a global constant by the input.
- `tiny_ori_2`: Small routine of 1 instruction that merges 240 into d7.
- `compute_table_cell_address`: If a stored word is zero, stores the global buffer pointer and returns the structure's address.
- `split_cell_index_by_type`: Divides the argument by twelve to select a table row, then splits a stored longword by seven or thirty-one depending on a type value.
- `split_index_by_thirty_one`: Divides the argument by thirty-one to get a row and remainder, then returns an address offset from the global table scaled by the row, together with a shift derived from the remainder.
- `scale_source_bytes_into_table`: Multiplies sixteen source bytes by a scaling byte from the structure, shifts each product right by four and stores it in a table of global entries.
- `check_and_branch`: Calls a routine and, when it returns zero, calls a second routine and returns zero.
- `call_when_flag_then_finish`: Calls a routine only when a global flag byte is nonzero, then always calls a second routine and returns.
- `update_global_status_flags`: Calls a routine, then, when a global flag is set and the structure pointer argument is valid, clears the lowest bit of a flag byte.
- `clear_flag_and_call_handler`: When a global handler pointer is set, clears the lowest bit of a flag byte, clears the handler pointer and calls the handler, returning its result.
- `open_audio_and_cia_devices`: Allocates a signal and finds the current task, then opens audio.device and ciab.resource and stores their pointers in shared globals.
- `close_audio_device_free_signal`: Closes the audio device and frees its signal when the shared flag shows they are held, then returns the flag value.
- `return_input_byte_unchanged`: Returns the low byte of the data register it receives without changing it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_without_work`: Does nothing and returns at once.
- `seek_and_read_record_field`: Looks up a record through a helper and returns -1 if none exists.
- `write_string_to_bounded_buffer`: Copies a zero-terminated string character by character into a buffer tracked by a remaining-space counter.
- `format_printf_conversion`: Parses one printf-style conversion specification with its flags, width, precision and length modifier, then formats a character, string or number with padding.
- `parse_format_string`: Walks a format string, sending ordinary characters to a callback and treating a doubled percent sign as one literal percent.
- `ensure_record_handle_allocated`: Creates a handle through a helper when the record has none or is marked as needing one.
- `manage_record_data`: This routine performs multiple operations involving seeking, reading, validating, and allocating handles for records within a table.
- `invoke_flagged_table_entries`: Walks a table of records from the last entry to the first and calls a routine on the stored pointer of each record whose first byte is nonzero and whose flag word lacks a particular bit.
- `iterate_and_call_handlers`: Iterates through a table of values, checking specific bit flags, and calls another routine for each valid entry found.
- `forward_to_fixed_jump`: Holds a single jump that passes control on to a fixed address elsewhere in memory.
- `parse_zone_name_and_offset`: Copies a three-letter name from a source string, reads a numeric offset in hours and converts it to seconds, and copies an optional second three-letter name when one is present.
- `convert_uppercase_char`: Converts one lowercase letter to uppercase by subtracting 32 from its code.
- `compute_time_with_stored_offset`: Calls helpers for a stack record and a time value, then returns the sum of a stored offset in seconds, a stack byte and the helper result.
- `convert_string_to_uppercase`: Converts each character of a zero-terminated string to uppercase in place, using a flag table in shared data to decide which characters are lowercase.
- `copy_null_terminated_string`: Copies a zero-terminated string from the source to the destination, including the terminating zero.
- `skip_flagged_characters`: Advances through a string while each character has a particular class bit set in the shared character table, then returns the first position whose character lacks it.
- `reverse_digits_into_string`: Writes characters produced by a helper into a buffer in reverse order, then copies them back forward and ends the text with a zero byte.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_octal_text`: Converts a number to octal text by repeatedly taking its low three bits as a digit, stores the digits in reverse order and copies them forward into the destination with a terminating zero byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `parse_decimal_or_format_hex`: Depending on a flag on the stack, either writes an unsigned value as base-16 digits in reverse order into a buffer, or parses decimal digits from text into a 16-bit value with an optional minus sign.
- `convert_number_to_hex_text`: Converts an unsigned number to base-16 text by sending each four-bit group through a lookup table, writes the digits in reverse order and ends with a zero byte.
- `parse_signed_decimal_short`: Parses an optional plus or minus sign followed by decimal digits into a 16-bit value, negating it for a minus sign.
- `parse_decimal_digits_with_sign`: Reads decimal digits from a string into a 16-bit value, negating the result when the string begins with a minus sign.
- `append_byte_to_output_buffer`: Appends one byte to the buffer whose write position is kept in shared data and increments a count of bytes written.
- `format_into_buffer_with_count`: Resets the byte count and sets the write position to the given buffer, runs a formatting helper that appends bytes through the append routine, then ends the buffer with a zero byte and returns the count.
- `release_block_to_free_list`: Returns a memory block to a sorted free list, rounding its size up to a multiple of four with a minimum of eight bytes, and merges it with adjacent free blocks.
- `read_record_field_with_status`: Looks up a record through a helper and returns -1 if none exists.
- `copy_memory_block`: Copies a count of bytes from the source to the destination, going backwards when the source starts at or below the destination so overlapping regions are copied safely, and forwards otherwise.
- `free_all_allocated_nodes`: Walks a linked list of allocated nodes, releasing each one through the system memory free routine, then clears the list head and tail and returns zero.
- `replace_sized_buffer_allocation`: Releases the buffer already held, if any, then allocates a new buffer of the requested size plus a two-byte size header through a helper and stores the size in that header.
- `validate_and_fetch_record`: Calls another routine to retrieve a value and performs additional checks against a global status to return a specific error code if conditions are not met.
- `allocate_tracked_memory_node`: Allocates a block through the system memory routine with a twelve-byte header and links it at the tail of a tracked list, setting the head pointer when none exists.
- `compute_elapsed_seconds_from_base`: Calls a helper, subtracts a stored base time from the value at the pointer argument, adds one hour in seconds when a stored flag is set, and passes the result to another routine.
- `build_date_time_record`: Reads several time fields through helper calls and stores them in a shared record, including a year offset from 1900 and a day of the week taken modulo seven.
- `convert_day_count_to_year`: Converts a day count since 1970 into a year and a day within it, subtracting 366 or 365 days per year according to the divisible-by-four rule.
- `convert_year_day_to_month`: Finds the month for a day of the year using a table of month lengths, setting February's length to 29 in leap years and restoring it to 28 afterwards.
- `allocate_block_from_free_list`: Searches a sorted free list for a block of at least the requested size, rounding the size up to a multiple of four with a minimum of eight bytes.
- `fill_date_record_from_days`: Calls a library routine, then counts whole years from a fixed starting year to reduce a day count to a remainder.
- `store_sized_buffer_header_pointer`: When its argument is nonzero, calls a helper and then stores the argument minus two in a shared slot, which is the header position of a sized buffer.
- `flush_pending_list_entries`: Walks a linked list and, for each entry whose second flag bit is set and third flag bit is clear, calls a routine with the difference of two stored counters when that difference is nonzero.
- `call_library_and_record_error`: Clears a status word, calls a library routine and returns its result.
- `seek_offset_by_mode`: Calls a library position routine, recording an error code when it fails.
- `execute_record_read`: Checks for a signal, clears a status flag, and calls a function pointer from a table, handling errors by calling a secondary routine and updating status variables.
- `release_port_and_signal_resources`: Removes a message port from its list when one is attached, marks the record as unused with a fixed value, frees its signal and calls the system free routine.
- `release_held_handle`: If a held value is set, calls a routine on it and clears the holder.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values passed in registers by combining their 16-bit halves: the two cross products are added into the high word and the low halves give the low word.
- `divide_fixed_point_values`: Divides one 32-bit fixed-point value by another passed in registers, handling negative operands separately.
- `divide_with_quotient_and_remainder`: Divides a 32-bit value by another in a normalized fixed-point manner and returns a 64-bit result with the quotient in the high half and the remainder in the low half.
- `allocate_and_set_node`: Allocates a memory block through the system routine and, when it succeeds, stores a type value of five, clears one byte and records the argument in the node.
- `create_message_port_node`: Allocates a signal and a message node, fills the node with the argument, the signal, the current task and a list header, and adds it to a port when the argument is nonzero.
- `lookup_table_entry_by_index`: Checks that an index lies within the table count and that its entry is in use.
- `check_signal_and_run_handler`: Reads the pending signal bits and, when the flagged signal bits are set and a handler is registered, runs the handler.
- `forward_to_helper_routine`: Calls one helper routine and returns.
- `fetch_word_via_helper_chain`: Returns zero when the argument is zero.
- `return_absolute_value`: Returns the absolute value of a signed 16-bit argument by negating negative values and returning others unchanged.
- `tokenize_command_line`: Splits a command line into up to thirty-two tokens and stores a pointer to each in a table, counting them in shared data.
- `load_asset_files`: A routine that calls another routine to load files named blocks.new, dump0, dump2, dump4, and f&b.n.
- `iterate_handler_table`: Iterates through a table of values, checking bit flags and calling another routine for each entry that meets specific criteria.
- `decrement_global_counters`: Decrements two global variables if they are non-zero and calls another routine multiple times.
- `decrement_timers_with_reload`: Clears one counter, calls three helper routines, decrements a second counter when it is nonzero, and decrements a third counter that reloads to three and triggers a helper call when it reaches zero.
- `enter_infinite_loop_2`: Spins in an empty loop and never returns.
- `open_resource_and_query_value`: Clears three shared slots, opens a resource through a helper with the argument, and on success configures it with fixed parameters and looks up a second value through another helper.
- `configure_resource_with_value`: Clears three shared slots, opens a resource through a helper with the first argument, and on success configures it with fixed parameters and stores the second argument in shared slots.
- `decode_lzw_compressed_stream`: Decodes an LZW-style stream of nine-bit codes, handling the clear and end codes, expanding each code by walking a prefix table and writing output bytes into a buffer.
- `fill_row_offset_table`: Fills a table of two hundred words with offsets that step by forty, starting at zero.
- `output_character_with_line_handling`: Handles carriage-return and line-feed characters by calling line routines.
- `process_input_character`: Checks a character against specific values and, if not matching, updates several global variables using data from a pointer in a table.
- `process_input_character_2`: Compares a byte to specific values and updates global variables with data from a table if the comparison fails.
- `process_character_stream`: Checks a character for line-feed or carriage-return values, then extracts data from a pointer to update global variables and iterates through a buffer.
- `copy_word_from_record_field`: Copies a 16-bit field from the record referenced by a shared pointer into a fixed shared slot.
- `advance_position_within_limit`: Calls a helper, reads a 16-bit value from the buffer that a shared pointer addresses and adds it to a position counter.
- `copy_clipped_region_width`: Stores a field from each of two record arguments in shared slots, computes each record's width from its left and right edges, and takes the smaller width.
- `rotate_five_shared_words`: Rotates five shared 16-bit words left by one position, moving the first to the end, then calls a routine with a fixed address.
- `rotate_word_table`: Rotates four consecutive 16-bit values in a table one position and calls another routine.
- `decrement_nibble_fields_of_table`: Decrements a counter and, when it was not negative, calls a routine and then walks thirty-two words.
- `subtract_nibble_mask_from_table`: When a counter is not negative, decrements it and subtracts a shared mask from each of thirty-two words field by field, clamping each four-bit field at zero.
- `call_library_vector_36`: Loads a library base from a shared variable and calls the library routine at a fixed offset from it.
- `allocate_slot_in_fixed_table`: Finds the first empty entry in a table of fifty eight-byte slots, allocates a block through the system routine, and stores the block and the argument in that slot.
- `compute_packed_value_pair`: Computes a value from a shared counter and a constant offset, packs one byte from a shared byte into it, and stores the two-word result in shared data.
- `seek_then_read_when_nonnegative`: When its second argument is nonnegative, calls a library routine at one fixed offset first.
- `call_routine_with_record_rect`: Calls a routine with the first field of a record, its offset field and the width computed from its right and left fields.
- `call_library_checking_positive_result`: Calls a library routine at a fixed offset and returns its result, or minus one when that result is less than one.
- `return_immediately_without_work_2`: Does nothing and returns at once.
- `decode_clipped_run_records`: Walks eight-byte records, skipping any whose rectangle lies outside a clipping window, and decodes packed run codes from each visible record.
- `clip_and_decode_run_records`: Iterates through a list of eight-byte records to determine if their associated rectangles intersect a clipping window.
- `emit_packed_run_fragment`: Continues the packed-run decoding loop from the middle of its body, emitting runs through helper calls and reducing the remaining-count counters.
- `emit_packed_run_remainder`: Continues the packed-run decoding loop from the middle of its body, emitting the remaining run through helper calls and returning to the record loop when its counter reaches zero.
- `emit_packed_run_continuation`: Continues the packed-run decoder from the middle of its body, writing run bytes through helper calls.
- `decode_packed_run_codes`: Continues the packed-run decoder from the middle of its body, reading run codes and emitting literal or repeated values through helper calls.
- `emit_packed_run_segments`: Continues the packed-run decoder and emits run segments through helper calls, reducing the remaining-count counters as each segment is written.
- `emit_alternating_pattern_runs`: Continues the packed-run decoder and writes runs through helper calls, choosing one of two alternating pattern bytes from the parity of a record field before each run.
- `decompress_graphic_run_segments`: Iterates through a stream of compressed data to extract segment parameters, coordinates, and lengths.
- `decode_compressed_graphics_stream`: Parses a stream of compressed data to extract segment parameters and coordinates.
- `filter_visible_segments`: Iterates through a linked list of eight-byte records to identify segments that fall within a specific coordinate window.
- `filter_clipping_segments`: Iterates through a list of eight-byte records to identify segments that fall within a specific coordinate window.
- `process_packed_run_segments`: Iterates through a list of eight-byte records to identify those within a clipping window.
- `process_compressed_graphics_stream`: Iterates through a stream of compressed data to extract segment parameters and coordinates.
- `decode_packed_run_segments`: Iterates through a stream of compressed run-length encoded data to reconstruct segments.
- `emit_packed_run_segments_2`: Continues the packed-run decoder from the middle of its body, emitting run segments through helper calls and returning to the record loop when its counter is spent.
- `apply_mask_pattern_to_planes`: ANDs a mask byte into the second, third and fourth of five bitplanes at one byte offset, and ORs the input byte with a fixed set of bits into the fifth.
- `update_memory_masks`: Performs bitwise AND operations on four memory locations and a bitwise OR operation on a fifth memory location using values from registers.
- `set_plane_zero_mask_others`: ORs the input byte into the first of five bitplanes and ANDs the mask into the other four at the same byte offset.
- `set_plane_one_mask_others`: ORs the input byte into the second of five bitplanes and ANDs the mask into the other four at the same byte offset.
- `set_planes_zero_one_mask`: ORs the input byte into the first and second of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_plane_two_mask_others`: ORs the input byte into the third of five bitplanes and ANDs the mask into the other four at the same byte offset.
- `set_planes_zero_two_mask`: ORs the input byte into the first and third of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_one_two_mask`: ORs the input byte into the second and third of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_first_three_mask_rest`: ORs the input byte into the first three of five bitplanes and ANDs the mask into the last two at the same byte offset.
- `set_plane_three_mask_others`: ORs the input byte into the fourth of five bitplanes and ANDs the mask into the other four at the same byte offset.
- `set_planes_zero_three_mask`: ORs the input byte into the first and fourth of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_one_three_mask`: ORs the input byte into the second and fourth of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_zero_one_three`: ORs the input byte into the first, second and fourth of five bitplanes and ANDs the mask into the other two at the same byte offset.
- `set_planes_two_three_mask`: ORs the input byte into the third and fourth of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_zero_two_three`: ORs the input byte into the first, third and fourth of five bitplanes and ANDs the mask into the second and fifth at the same byte offset.
- `set_planes_one_to_three`: ORs the input byte into the second, third and fourth of five bitplanes and ANDs the mask into the first and fifth at the same byte offset.
- `set_first_four_mask_last`: ORs the input byte into the first four of five bitplanes and ANDs the mask into the fifth at the same byte offset.
- `return_input_byte_unchanged_2`: Returns the low byte of its input register unchanged.
- `render_text_run_by_character`: Draws a run of characters from a text buffer, passing each character to a handler looked up in a table and advancing word by word between spaces.
- `return_immediately_without_work_3`: Does nothing and returns at once.
- `return_from_subroutine_3`: This routine performs a single instruction to return from a function call.
- `return_from_subroutine_4`: This routine performs a single return instruction to exit the current function and return control to the caller.
- `return_from_subroutine_5`: This routine performs a single return instruction.
- `return_from_subroutine_6`: The routine is empty and performs no operations before returning to the caller.
- `execute_recursive_call`: The routine performs a recursive call to itself before returning.
- `scan_text_line_bounds`: Scans a text buffer from a start position to the end of the current line, recording the positions of spaces and line ends in shared variables.
- `advance_text_span_half_width`: When the span has characters and its stored start is not after its stored end, adds half of the span's width to a position field.
- `set_text_span_width_draw`: Stores the width of the current text span in a position field, taking it from the span end when the span has characters, then calls a helper for each character up to the span end.
- `call_helper_per_character`: Calls a helper once for each character from a start index until a limit or a zero byte is reached.
- `measure_word_and_spaces`: Walks a text line past leading spaces and across the next word, subtracting the width of each character from the span's remaining width, and counts the spaces in the remaining span.
- `mask_planes_return_pattern`: ANDs a mask byte into the second through fifth of five bitplanes at one byte offset, leaving the first plane untouched, and returns the input byte with a fixed set of bits added.
- `apply_masks_and_return`: Performs bitwise AND operations on four memory locations and returns a value resulting from a bitwise OR operation.
- `mask_bitwise_data`: Performs a bitwise AND operation between a byte from a register and four separate memory locations at fixed offsets from a base pointer.
- `modify_memory_bits`: Performs a bitwise OR on a memory location and bitwise AND operations on four subsequent memory locations at fixed offsets.
- `set_planes_one_four_mask`: ORs the input byte into the second and fifth of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_zero_one_four`: ORs the input byte into the first, second and fifth of five bitplanes and ANDs the mask into the third and fourth at the same byte offset.
- `modify_memory_bits_2`: Performs a bitwise AND on one memory location, a bitwise OR on another, and bitwise AND operations on three others at fixed offsets.
- `modify_memory_bits_set_a`: Performs bitwise OR operations on a memory location and three subsequent offsets using two different byte values.
- `set_planes_one_two_four`: ORs the input byte into the second, third and fifth of five bitplanes and ANDs the mask into the first and fourth at the same byte offset.
- `set_planes_except_three`: ORs the input byte into all of five bitplanes except the fourth, and ANDs the mask into the fourth at the same byte offset.
- `modify_memory_bits_clear_a`: Performs bitwise AND and OR operations on a memory location and four subsequent offsets using two different byte values.
- `update_bitmask_segments`: Performs bitwise OR and AND operations on five specific offsets within a memory block using values from two registers.
- `set_planes_one_three_four`: ORs the input byte into the second, fourth and fifth of five bitplanes and ANDs the mask into the first and third at the same byte offset.
- `set_planes_except_two`: ORs the input byte into all of five bitplanes except the third, and ANDs the mask into the third at the same byte offset.
- `mask_bitplane_data`: Performs bitwise AND and OR operations on specific offsets within a memory block using values from two registers.
- `update_bitmask_segments_2`: Performs bitwise OR and AND operations on five specific offsets within a memory block pointed to by a register.
- `set_planes_except_zero`: ORs the input byte into the second through fifth of five bitplanes and ANDs the mask into the first at the same byte offset.
- `set_all_planes_from_input`: ORs the input byte into all five bitplanes at one byte offset, with no mask.
- `set_plane_four_mask_others`: ORs the input byte into the fifth of five bitplanes and ANDs the mask into the first four at the same byte offset.
- `set_planes_zero_four_mask`: ORs the input byte into the first and fifth of five bitplanes and ANDs the mask into the middle three at the same byte offset.
- `modify_bitmask_table`: Performs bitwise AND and OR operations on specific offsets within a memory block pointed to by a register.
- `modify_memory_bits_set_d`: Performs bitwise OR and AND operations on a memory location and four subsequent offsets using two different byte values.
- `set_planes_two_four`: ORs the input byte into the third and fifth of five bitplanes and ANDs the mask into the other three at the same byte offset.
- `set_planes_zero_two_four`: ORs the input byte into the first, third and fifth of five bitplanes and ANDs the mask into the second and fourth at the same byte offset.
- `modify_bitmask_table_2`: Performs bitwise AND and OR operations on specific offsets within a memory block using values from two registers.
- `apply_bitmask_to_memory_blocks`: Performs bitwise OR operations on five specific memory offsets relative to a pointer using a byte from a data register, with one offset receiving a bitwise AND operation instead.
- `set_planes_three_four`: ORs the input byte into the fourth and fifth of five bitplanes and ANDs the mask into the first three at the same byte offset.
- `set_planes_zero_three_four`: ORs the input byte into the first, fourth and fifth of five bitplanes and ANDs the mask into the second and third at the same byte offset.
- `modify_bitmask_via_pointer`: Performs bitwise AND and OR operations on specific offsets within a memory block pointed to by a register.
- `apply_bitwise_or_mask`: Performs a bitwise OR operation on five specific memory locations using a value from a register.
- `set_planes_two_to_four`: ORs the input byte into the third, fourth and fifth of five bitplanes and ANDs the mask into the first two at the same byte offset.
- `set_planes_except_one`: ORs the input byte into all of five bitplanes except the second, and ANDs the mask into the second at the same byte offset.
- `apply_mixed_bitwise_mask`: Performs a bitwise AND on the first memory location and bitwise OR operations on the remaining four locations using values from registers.
- `apply_bitwise_or_to_memory_offsets`: Performs a bitwise OR operation between a byte from a register and a value at a pointer, then repeats this operation at fixed intervals of 8000 bytes from that same pointer.
- `start_blitter_rectangle_operation`: Sets up the blitter registers for a rectangular operation, computing the word-aligned width, shift, modulo values and blit size from its arguments, then starts it through several helper calls.
- `wait_for_bit_clear`: Polls a bit at a specific offset within a memory location until it is cleared.
- `call_helpers_if_size_positive`: Calls a helper, then calls a second helper only when both halves of its third argument are positive, and finally calls a third helper.
- `program_blitter_row_passes`: Programs the blitter registers from the arguments and a lookup table, then repeats a five-pass loop that picks one of two blitter control values by the parity of a row value and writes it with the destination and size registers.
- `call_own_blitter`: Calls the system routine that claims the blitter for exclusive use and returns its result.
- `call_disown_blitter`: Calls the system routine that releases the blitter after exclusive use and returns its result.
- `toggle_bit_and_mask_byte`: Toggles one bit, selected by the low three bits of a register, in one byte and ANDs a mask into a second byte.
- `call_other_routine`: The routine calls another routine and then returns.
- `blit_masked_rectangle`: Sets up the blitter for a rectangle with first and last word masks and computed shift and modulo values, then passes the rectangle to helper routines.
- `draw_table_entry_planes`: Looks up a table entry by index and, when the index is in range, steps through its data using the entry's bit mask.
- `draw_masked_bitplane_rows`: Walks rows across five bitplanes, clearing destination bits under an inverted mask byte and ORing in source bytes for each plane.
- `copy_five_bitplane_rows`: Copies bytes from five source bitplanes to the matching destination bytes, one byte per plane, repeating until the count runs out.
- `draw_masked_words_five_planes`: Walks 2000 words across five bitplanes.
- `draw_masked_word_rows`: Walks rows of masked words across five bitplanes, clearing destination bits under an inverted mask word and ORing in source words for each plane.
- `combine_five_planes_into_mask`: ORs the five bitplanes together word by word into a destination buffer placed 40000 bytes after the source, so each output word marks every pixel that has any bit set.
- `add_to_value_if_nonzero`: Adds the register value to the longword at the address argument when that longword is nonzero, and leaves it unchanged otherwise.
- `adjust_pointer_offsets`: Calls helper routines in a loop until a condition is met, then walks a table of record references and adds each record's base address to the pointers in any record whose flag is negative.
- `update_copper_list_pointer`: Calls a helper when a pending flag is set and clears the flag, then selects a copper list address from a stored value and records it in shared data.
- `handle_recursive_flag`: Checks a global flag to decide whether to call another routine recursively before updating a global variable with a value from a memory location.
- `store_copper_slot_address`: Copies a shared value into a slot, then stores the address of the slot in it, with the same result in both branches of a comparison.
- `update_global_pointer`: Copies a value from a global variable to another global variable.
- `calculate_packed_value`: Computes a 32-bit value by performing arithmetic on a global short and a memory value, then stores the result in a global long.
- `copy_words_at_stride_four`: Copies thirty-two words from the source list into a shared buffer, one word every four bytes.
- `copy_words_at_stride_four_2`: Copies thirty-two words from the source into a shared buffer, one word every four bytes.
- `set_copper_bitplane_pointers`: Fills a shared table with the high and low words of bitplane addresses spaced 8000 bytes apart, writes fixed words to four locations, and sets up the copper list pointer.
- `copy_words_to_fixed_table`: Copies sixteen words from the source pointer into a fixed table, one word every two bytes.
- `copy_memory_block_2`: Copies sixteen bytes from a source pointer to a destination pointer using a loop.
- `copy_word_block_2`: Copies a sequence of sixteen words from a source pointer to a destination address.
- `enter_infinite_loop_3`: Spins in an empty loop and never returns.
- `clear_table_and_set_limits`: Stores its argument in a shared slot, clears a 32-word table, and sets two shared values to the fixed limits 4095 and 15.
- `clear_table_and_set_limits_2`: Stores a parameter in a global variable, clears a 32-word table by iterating through it, and sets two global shorts to fixed values.
- `wait_until_byte_negative`: Spins while a shared byte is nonnegative and returns once it becomes negative.
- `subtract_nibble_masks`: Iterates through a table of words, subtracting specific bit-masked components from a global value and storing the result back into the table.
- `copy_longwords_to_fixed_table`: Copies sixteen longwords from the source pointer into a fixed table, one every four bytes, then stores the value seventeen in a shared slot.
- `wait_until_flag_negative`: Spins in an empty loop while a signed byte in a fixed memory cell is zero or positive, and returns once that cell turns negative.
- `decrement_nibbles_over_words`: Decrements a countdown byte and, while it was non-negative, calls an unnamed helper and then reduces every nonzero low, middle and top nibble of the first 32 words at the routine's own entry by one.
- `handle_input_event_record`: Processes one queued input record: type 2 adds two offsets to counters clamped to 0-319 and 0-199 and sets a flag for code 104 or clears it for code 232, while type 1 maps the Y and X key characters to a state byte and stores any other non-negative character.
- `process_input_record`: Processes an input record by either updating two clamped global counters based on offsets or updating a global variable based on specific character values.
- `update_parameter_state`: Processes a data structure to update two global counters with clamped values and modifies a global byte based on a short integer comparison.
- `process_event_parameter_data`: Updates two global counters with clamped values based on offsets from a provided pointer and modifies a global byte based on a short value.
- `process_input_event_record`: Processes input records by either updating two clamped counters and a state flag based on record offsets, or updating a state word and a byte based on specific key codes.
- `apply_clamped_deltas_and_flag`: Adds a record offset to a counter clamped to 0-319 and another to a counter clamped to 0-199, sets a state flag for code 104 and clears it for code 232, then clears the record type and calls a library routine.
- `apply_clamped_delta_and_flag`: Stores a register value into one state byte, adds a record offset to a counter clamped to 0-199, sets or clears a state flag for codes 104 and 232, clears the record type and calls a library routine.
- `store_clamped_value_and_flag`: Stores a register value into a counter clamped at zero and above, sets or clears a state flag for codes 104 and 232, clears the record type and calls a library routine.
- `store_value_and_set_flag`: Stores a register value into a counter without clamping, sets or clears a state flag for codes 104 and 232, clears the record type and calls a library routine.
- `handle_key_event_record`: Handles a key record only when its type is 1: the Y key code clears one state word, the X code sets it to 32, and any non-negative code is stored in a state byte.
- `store_key_code_from_record`: Applies a key record unconditionally: Y clears one state word, X sets it to 32, and a non-negative character code is stored in a state byte.
- `open_input_device_request`: Allocates a message port and an I/O request block, opens the input device by its name in that request, and stores both in the state block, running cleanup on failure.
- `send_input_device_command_10`: Sets the command value of the stored input device request to 10, points its data at the state block, and sends it synchronously through the library I/O call.
- `allocate_typed_record`: Returns zero for a zero argument.
- `allocate_linked_request_record`: Allocates a request record and fills in a value, a flag byte, type 4 and a library-obtained value; when the first argument is zero it links the record into a self-referencing list head, otherwise it calls a library routine to attach it.
- `release_request_block`: If the record's pointer field is set, passes it to a library routine.
- `forward_to_helper_routine_2`: Calls one unnamed routine with no arguments and returns.
- `scan_list_for_pending_entries`: Walks a linked list of records starting from a fixed offset in the state block.
- `load_named_asset_files`: Decompilation timed out, so this rests on the facts alone: the routine references the data file names blocks.new, dump0, dump2, dump4, f&b.new, gypsy.new, intro.ptr, logo.new and u6.set, and makes 22 calls to unnamed routines across 364 instructions.
- `change_current_selection`: When its argument differs from the value stored in a state word, it releases the previous selection if one is set, then for a nonzero new value it looks up a four-byte entry in a table indexed by that value and runs several unnamed routines with it.
- `store_high_word_field`: Stores the upper 16 bits of its 32-bit argument into a field of the state block.
- `initialize_record_tables_and_loops`: Copies several 32-bit values from the state block into record tables and writes fixed 16-bit bounds and coordinates into them, such as 319 and 55.
- `allocate_block_with_size_header`: Frees the block held in a state word if one is set, then for a nonzero size argument allocates that size plus two bytes, stores the size in the first word and returns a pointer just past it.
- `toggle_indexed_table_entry`: Reads a four-byte entry from a table selected by a state word, writes it to the argument and a state slot, and flips a state byte between 0 and 1.
- `call_scaled_index_helper`: Takes the upper 16 bits of its argument, multiplies them by 64 and adds a fixed offset into the state block, then calls an unnamed routine with the result and then another unnamed routine.
- `execute_handler_pointer`: Calls a routine through a pointer stored in a data structure.
- `clear_flag_call_if_set`: Returns zero unless a global flag is set; when it is, it clears bit 0 of a byte at a fixed address, zeroes a second variable and returns the result of an unnamed routine call.
- `relocate_pointer_table_entries`: Calls several unnamed routines, then repeatedly reads a value from a helper and, for each record whose 16-bit flag is negative, adds the record's own base address to each 32-bit entry of its pointer table.
- `call_library_stub_no_args`: Loads the base of a library from a global variable and calls the function at a fixed negative offset from that base, with no arguments.
- `call_library_pair_conditional`: Calls one library function at a fixed offset from a library base when the second argument is non-negative, then always calls another library function at a second offset.
- `stub_no_operation`: Its body is only a return instruction, so it does nothing when called.
- `fill_row_offset_table_2`: Writes 200 consecutive 16-bit values into the given array, starting at 0 and increasing by 40 at each step.
- `open_slot_and_read_status`: Clears three state words, asks an unnamed routine to open a slot for its 16-bit argument and, on a non-negative result, stores that slot and a second value, then runs two unnamed routines.
- `compute_offset_sum_from_record`: Calls an unnamed routine to fill a small local record, runs two loops whose bodies are empty in the pseudo-code, and calls several unnamed routines.
- `copy_32_words_to_state`: Copies 32 consecutive 16-bit words from a source block whose address is held in the state block into a fixed area of the state block.
- `configure_case_parameters_and_resources`: Selects one of about a dozen cases from its argument and sets a case-specific group of 16-bit parameters in the state block, then calls unnamed routines with fixed resource offsets.
- `assign_global_self_reference`: Copies a state word into a global variable and then assigns the global its own address.
- `initialize_state_and_run_loops`: Copies groups of 32-bit values between state blocks and writes several 16-bit bounds, then calls unnamed routines inside a 50-iteration loop, a 30-iteration loop and a 350-iteration loop.
- `store_zero_and_return`: Stores zero to one location and returns.
- `repeat_call_count_times`: Calls an unnamed routine once per step while its 16-bit argument counts down to zero, decrementing the argument each time.
- `query_library_or_minus_one`: Calls a library function at a fixed negative offset from a library base and returns its result, except that results below 1 are returned as -1.
- `assign_global_pointer_reference`: Copies the address of a specific global variable into another global variable.
- `stub_unreachable_code_only`: The decompiler reported nearly every block as unreachable and truncated the listing, so only a single call to an unnamed routine is visible.
- `initialize_system_resources_2`: The routine initializes several global variables, allocates memory, and sets up a state block through various function calls.
- `calculate_indirect_call_target`: Computes a target address by scaling a parameter and adding it to a base offset.
- `store_argument_to_state_field`: Stores its 32-bit argument into a fixed field of the state block.
- `return_global_if_call_succeeds`: Calls an unnamed routine with its argument and returns the value of a global variable when that call's 16-bit result is nonzero, otherwise returns zero.
- `convert_char_to_uppercase`: Converts a lowercase ASCII letter to uppercase by subtracting 32 and returns the character; all other values are returned unchanged.
- `copy_null_terminated_string_2`: Copies a null-terminated string from the second argument into the first, including the terminator, and returns the destination pointer.
- `execute_library_call`: Calls a function pointer retrieved from a library structure and ensures the return value is at least negative one.
- `execute_conditional_library_calls`: Calls two different function pointers from a library structure, where the first call is performed only if a provided argument is non-negative.
- `call_library_optional_then_final`: When its second argument is non-negative, calls a library function at a fixed offset from the library base, then always calls a different library function at another offset.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `process_slot_opening`: Calls a routine with an argument and returns a value from a global variable, which is set to zero if the called routine returns zero.
- `allocate_slot_in_table`: Scans a table of 50 eight-byte entries for a free one; if the table is full it returns 50.
- `free_table_entries_matching`: Walks a table of 50 eight-byte entries and, for each entry whose first word equals the argument, frees its memory block through the system and clears both fields of that entry.
- `free_all_table_entries`: Walks the same 50-entry table and frees the memory block of every nonzero entry through the system, clearing both fields of each freed entry.
- `do_nothing`: This routine is empty and performs no operations.
- `store_argument_to_global`: Stores a four-byte argument into a global variable.
- `loop_forever`: Enters an infinite loop that never returns.
- `write_byte_and_terminate`: Reads a write position from a field of the state block, advances it by one and stores the second byte of the argument there, then writes a zero byte just after it so the buffer stays terminated.
- `call_with_span_from_record`: Calls an unnamed routine with the first field of its record and a count equal to the difference between two 16-bit fields of that record plus one.
- `store_two_words_into_record`: Writes the two 16-bit halves of its 32-bit argument into the record that the state block points to, at offsets 4 and 6.
- `step_values_toward_targets`: On first use it computes per-entry step sizes from the difference between two input arrays and obtains three work arrays.
- `copy_pair_to_pointer_list`: Walks the pointer arguments that follow its own on the stack until a null pointer, copies its two 32-bit words into each one, and returns zero.
- `advance_state_seed_high_bits`: Calls an unnamed routine, adds the constant 12345 to its result, stores the sum in a state field and returns the upper 15 bits of that sum.
- `store_high_word`: Stores the upper sixteen bits of a passed value into a memory location offset from a pointer.
- `reallocate_buffer`: Releases an existing buffer and allocates a new one with a size header based on a provided length.
- `call_when_extents_positive`: Calls an unnamed routine, calls a second one only when both 16-bit halves of its third argument are positive, then calls a third routine.
- `record_block_header_for_release`: When its argument is nonzero, calls an unnamed routine and stores the argument minus 2 in a state field, which the block-allocation routine frees on its next call.
- `open_slot_with_global_key`: Clears three state words and asks an unnamed routine to open a slot for its 16-bit argument.
- `spin_forever`: Loops forever with no body, so a call to it never returns.
- `multiply_32bit_low_product`: Multiplies two 32-bit values held in the data registers from 16-bit partial products and returns the low 32 bits of the product.
- `initialize_system_resources`: The routine allocates memory, opens libraries, initializes several global variables, and sets up a state block.
- `rotate_four_word_table`: Rotates four consecutive 16-bit values one position, moving the first to the end, then calls an unnamed routine with a fixed address.
- `toggle_indexed_data`: The routine retrieves a value from a table using an index, updates two locations with it, and toggles a byte in a global variable.
- `update_state_and_toggle_flag`: fetches a 32-bit value from a table based on an index, stores it in two locations, and toggles a byte via an XOR operation.
- `wait_for_field_match`: Spins in an empty loop until bits 8 to 16 of a 32-bit field, read through a pointer in the state block, equal the argument shifted into that position.
- `return_from_field_match`: An empty routine that performs no operations before returning to the caller.
- `wait_until_field_reaches_value`: Spins in an empty loop while bits 8 to 16 of a 32-bit field, read through the state block, are below the argument shifted into that position, and returns once they reach it.
- `toggle_and_return_current_entry`: Flips a state byte between 0 and 1, then returns the four-byte entry from a table selected by a state word.
- `return_opposite_table_entry`: Returns the four-byte entry from a table at the index of a state word with its lowest bit flipped, so it selects the other entry of each pair.
- `return_current_table_entry`: Returns the four-byte entry from a table selected by a state word.
- `push_byte_into_ring_buffer`: Stores the second byte of its argument at the write position of a 100-entry ring buffer in the state block and advances that position, wrapping at 100.
- `return_pending_key_byte`: Returns the byte at a fixed state position that holds the most recently stored key code.
- `wait_and_take_key_mapping`: Spins in an empty loop until the pending key byte is nonzero, then clears it and returns the value from a table of signed bytes indexed by that key code.
- `snapshot_counters_and_mode_flags`: Copies two stored 16-bit counters into a snapshot area of the state block and records two flags from a mode word: one is set when the mode equals 1, and the other is 2 when the mode equals 2 and 0 otherwise.
- `call_own_blitter_2`: Library stub: loads its arguments and the graphics library base, calls OwnBlitter and returns its result to the caller.
- `call_disown_blitter_2`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `perform_indirect_call`: The routine calculates an address based on an argument and an offset to call another routine.
- `perform_indirect_call_variant`: The routine calculates an address using a bit-shifted argument and an offset to call another routine.
- `call_scaled_index_only`: Calls an unnamed routine with a scaled offset into the state block, computed from the upper 16 bits of its argument.
- `copy_word_block`: copies thirty-two words from a source address derived from a table entry to a destination address in a global buffer.
- `decrement_loop_counter`: The routine iterates a loop, calling another routine and decrementing a value until it reaches zero.
- `wait_for_negative_byte`: spins in a loop until a specific byte in memory becomes negative.
- `return_zero_or_minus_one`: Calls an unnamed routine and, if its result is zero, calls a second routine and returns 0.
- `copy_sixteen_longs_to_entry`: Copies sixteen 32-bit values from its pointer argument into memory starting at an unnamed routine's entry point, then sets a global to 17.
- `reset_words_and_set_globals`: Stores its argument in a global, zeroes 32 consecutive 16-bit words starting at an unnamed routine's entry point, and then sets two globals to 4095 and 15.
- `tick_timer_counters`: Clears one global, calls three unnamed routines and decrements a countdown global when it is nonzero.
- `wait_for_negative_byte_2`: Polls a specific byte in custom RAM until its value becomes negative.
- `clear_flag_then_assign_global`: If a state word is nonzero, calls an unnamed routine and clears that word.
- `fill_strided_pointer_tables`: Fills four groups of 32-bit values spaced 8000 apart into a global table, sets four pairs of 16-bit globals to 63234 and 256, and assigns the table pointer that the state uses.
- `copy_string_with_budget`: Copies a null-terminated string into a buffer one character at a time, using a remaining-space counter in the state block; when the counter runs out it calls an unnamed routine instead of storing the character.
- `enter_infinite_loop_5`: spins in an infinite loop and never returns.
- `process_linked_list`: traverses a linked list and calls another routine with a calculated difference if specific bits in a byte are set in a particular pattern.
- `update_copper_pointer`: Checks a flag, and if set, clears it and updates a global variable with a value from another global variable.
- `open_input_device`: Performs a series of operations involving an input device request, including storing results in a state block and calling another routine.
- `copy_words_into_global_array`: Copies 32 consecutive 16-bit values from its pointer argument into a global array, writing each value to the same location four times in a row.
- `configure_case_parameters_and_resources_2`: Selects one of about a dozen cases from its argument and sets a case-specific group of 16-bit parameters in the state block, then calls unnamed routines with fixed resource offsets.
- `copy_state_and_call_pair`: Copies one 32-bit state value into two other state slots, then calls an unnamed routine with a pointer, a 16-bit state field and a state value, followed by another routine on two state pointers.
- `locate_masked_block_and_call`: Looks up an entry in an offset table by index and, when it is in range, skips one block of data for each set bit in a four-bit mask, where each block's size comes from its row byte count and row count.
- `execute_recursive_call_2`: The routine performs a recursive call to itself before returning.
- `get_pending_key_2`: Returns a byte from a specific location in memory.
- `call_with_record_span_3`: Calls another routine using a pointer to a record, passing a value from the record and a count derived from the difference between two fields within that record.
- `call_with_self_and_stack`: Calls an unnamed routine with its own entry address, its argument and a pointer to its stack arguments.
- `wait_for_key_mapping`: Polls a memory location until it is non-zero, clears that location, and returns a signed byte from a table indexed by the retrieved value.
- `clip_spans_and_call_helper`: Stores one field from each of two records into globals, computes the smaller of their two spans (end minus start plus one), and calls an unnamed routine with fields from both records and that minimum span.
- `store_argument_to_global_2`: The routine stores a four-byte argument into a global variable using an offset.
- `copy_state_and_call_offsets`: Copies a 128-bit state block forward into a second state block, then calls an unnamed routine twice with the state block and the argument offset by 77 and by 79, and finally calls another routine.
- `advance_frame_and_schedule_entries`: Advances a frame counter and, every eighth frame, moves a scheduling index forward in a table and updates a 16-bit shift-and-XOR state.
- `advance_frame_schedule_only`: Advances the same frame counter and, every eighth frame, updates the scheduling index and shift-and-XOR state, calling unnamed routines for entries that are due.
- `run_scheduled_entries_from_index`: Halves a state counter, then from a starting index walks the scheduling table while the index is within a window set by the counter's remainder modulo 5.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d2, then returns.
- `decrement_and_loop`: Decrements a value passed in a register until it reaches zero, calling another routine during each iteration.
- `call_fixed_offset_sequence`: Makes nine calls to an unnamed routine on the same state-block address.
- `load_intro_and_starpos_data`: Decompilation removed most blocks as unreachable and truncated the listing, so this rests on the facts: the routine references the files intro.new and starpos.dat and makes calls to 32 unnamed routines across 2821 instructions.
- `process_table_entry_2`: Validates an index against a table size, retrieves an entry, and uses a bit mask to step through data via a recursive call.
- `return_empty_handler`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `process_nu_string`: Processes a string of repeating characters.
- `compute_interpolation_steps`: Calculates the distance and direction between two arrays of values to determine incremental step sizes.
- `nop_placeholder_routine`: This routine is empty and performs no operations before returning.
- `update_indexed_data_state`: Retrieves a longword from a table using an index, stores it in a provided pointer and a global location, and toggles a byte in a global variable.
- `call_scaled_index_2`: Calls another routine using a scaled offset into a state block derived from the upper 16 bits of the argument.
- `calculate_indirect_call_target_2`: Computes a target address by scaling a parameter and adding it to a base offset.
- `copy_state_to_four_slots`: Calls an unnamed routine on a state slot, then copies the 32-bit value in that slot into four other state slots.
- `handle_nunu_text_placeholder`: This routine is empty and performs no operations before returning to its caller.
- `copy_state_and_call`: Copies a 32-bit value into two separate state locations and then calls another routine twice with different parameters.
- `perform_no_operation`: This routine is empty and performs no actions before returning to its caller.
- `get_flipped_table_entry`: Returns a four-byte value from a table using an index derived by flipping the lowest bit of a state word.
- `perform_no_operation_2`: This routine is empty and immediately returns to its caller.
- `perform_no_operation_3`: This routine is empty and returns immediately to its caller.
- `free_matching_table_entries`: Iterates through a table of fifty eight-byte entries and frees the memory of any entry whose first word matches the provided argument.
- `return_empty`: The routine is empty and returns immediately.
- `update_state_seed_2`: Adds a constant to a value returned by another routine, stores the result in a state field, and returns the upper 15 bits of that sum.
- `configure_resource_2`: Clears three global variables, attempts to open a resource, and if successful, configures it with specific parameters and updates global variables.
- `call_with_record_span_2`: Calls another routine using a pointer to a record, passing a field from the record and a count calculated from the difference between two other fields within that record.
- `return_empty_with_text`: The routine is empty and returns immediately, though it references a text string.
- `enter_infinite_loop_4`: Enters an infinite loop and never returns.
- `get_table_entry`: Returns a four-byte value from a table using an index provided by a state word.
- `initialize_scenario_parameters`: Copies a block of data from a source table into a local buffer and then populates a state block with case-specific configuration values.
- `toggle_and_fetch_entry`: Flips a state byte between zero and one, then returns a four-byte value from a table indexed by a state word.
- `update_selection_state_2`: Compares an input value to a stored selection, releases the previous selection if necessary, and updates the state with a new selection and associated routine calls.
- `execute_indirect_jump`: Calculates a target address by combining a shifted value from a parameter with an offset and an existing register value, then performs an indirect call to that address.
- `update_global_conditional`: The routine calls another routine and returns a value from a global variable, which is set to zero if the called routine returns zero.
- `initialize_record_tables_and_loops_2`: Copies several 32-bit values from the state block into record tables and writes fixed 16-bit bounds and coordinates into them, such as 319 and 55.
- `traverse_table_entry_bitmask`: Iterates through a bitmask within a table entry to calculate memory offsets.
- `call_unnamed_routine`: This routine is empty and calls another routine.
- `update_indexed_table_state`: Retrieves a longword from a table using an index, stores it in two memory locations, and toggles a bit in a global byte.
- `access_scaled_index`: Calls another routine using a scaled offset into a state block calculated from the upper 16 bits of the input argument.
- `copy_and_dispatch_state`: Copies a 32-bit value from one location to two other locations and then calls two subsequent routines using state values and pointers.
- `fetch_flipped_table_entry`: Retrieves a four-byte value from a table using an index derived by flipping the lowest bit of a 16-bit value stored in a global variable.
- `configure_resource_with_parameters`: Clears three global variables, attempts to open a resource, and if successful, configures it with specific parameters and updates global values.
- `call_with_record_span`: Calls another routine using a pointer to a record, passing a starting value, a second value, and a count calculated from the difference between two fields in that record.
- `initialize_case_state_parameters`: Copies a block of data from a table into the state block and populates specific case-dependent fields based on an input index.
- `calculate_clipped_width`: Stores specific fields from two input records into global variables and calculates the minimum width between them based on their edge offsets.
- `toggle_table_index`: Flips a state byte using an XOR operation and returns a four-byte value from a table indexed by a global state word.
- `update_selection_state`: Compares an input value against a global selection, performs cleanup if the previous selection was active, and updates the selection with new data and various routine calls.
- `copy_pair_to_list`: Iterates through a list of pointers provided on the stack and copies a pair of 32-bit words to each non-null destination.
- `update_table_data`: Fetches a 32-bit value from a table using an index, stores it in two locations, and toggles a byte via an XOR operation.
- `clear_and_store_state_pair`: Clears one 16-bit state word and stores the low 16 bits of its argument in the adjacent state word.
- `copy_slot_and_call_indexed`: Copies a 32-bit state value into another state slot, then calls an unnamed routine with that slot, a 16-bit value looked up from a table by the argument plus a state field, and a 32-bit state value.
- `set_flag_if_result_27`: Calls an unnamed routine and returns its result; when the low 16 bits of the result equal 27, it sets a state flag to 1.
- `wait_or_poll_state_flags`: When two state bytes are both clear, it calls an unnamed routine and, if that returns nonzero, reads an entry whose flag bit 4 controls a follow-up call.
- `run_script_opcode_loop`: Reads bytes from a script buffer selected by a state index, storing values below 128 directly as the current value and dispatching codes 128 to 139 to special cases that set counters, copy bytes, index a table or end the sequence.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `add_record_field10_to_call`: Calls an unnamed routine and returns its 16-bit result added to the 16-bit field at offset 10 of the record passed in.
- `add_record_field12_to_call`: Calls an unnamed routine and returns its 16-bit result added to the 16-bit field at offset 12 of the record passed in.
- `execute_complex_logic`: This routine contains sixty-two instructions and calls another routine to update a state seed.
- `advance_entry_positions_and_wrap`: Loops over a count of entries, calling an unnamed routine for each and moving its position by a stored step; an entry that passes its limit is re-seeded from a call result.
- `store_countdown_and_pointer`: Stores one less than the low word of its second argument into a state record, clears the next word, stores the high word of the second argument, and stores the first argument as a 32-bit value.
- `update_active_entry_slots`: When the entry count differs from the pending count and a call result divides evenly, it fills the next slot with a type and position values derived from a record.
- `copy_block_call_fixed_points`: Copies 12 long values from a global block onto the stack, looks up a value from a 24-element stack array by a state field and stores it back, then makes four calls to an unnamed routine with fixed coordinates derived from its third argument.
- `scale_by_four_after_calls`: Calls two unnamed routines and returns its 16-bit argument multiplied by four.
- `multiply_difference_by_stored_word`: Returns the unsigned difference of two 16-bit register values multiplied by a 16-bit word read six bytes below a base address.
- `initialize_state_and_run_loops_2`: Copies groups of 32-bit values between state blocks and writes several 16-bit bounds, then calls unnamed routines inside a 50-iteration loop, a 30-iteration loop and a 350-iteration loop.
- `synchronize_state_values`: Retrieves a longword from a specific offset within a structure and copies it to four different memory locations.
- `divide_and_pack_quotient_remainder`: Calls an unnamed routine and divides its 16-bit result by the 16-bit argument, returning the quotient in the upper 16 bits and the remainder in the lower 16 bits.
- `process_table_entry`: Iterates through a table using a bit mask to calculate offsets and recursively calls another routine.
- `masked_plane_blit_rectangle`: Blits a rectangle across five 8000-byte planes with 40-byte rows: where the mask byte is 255 the source planes are copied, and elsewhere the destination is cleared by the inverted mask and ORed with the source.
- `return_empty_subroutine`: This routine performs no operations and immediately returns to its caller.
- `update_indexed_state_and_toggle_flag`: Retrieves a longword from a table using an index, stores it in a destination pointed to by a parameter and another memory location, and flips a bit in a global byte.
- `call_scaled_index`: Calls another routine using a scaled offset derived from the upper 16 bits of an argument.
- `dispatch_indirect_call`: Calculates a target address by shifting an input parameter and adding it to an offset before performing an indirect jump.
- `jump_to_return_from_handler`: Jump stub: passes control straight on to return_from_handler.
- `execute_indirect_handler`: Invokes a routine via a pointer retrieved from a data structure.
- `copy_state_values`: Copies 32-bit values between specific offsets in a state block and calls another routine twice.
- `return_immediately_4`: This is an empty routine that performs no operations and immediately returns to its caller.
- `get_pending_key`: Returns a byte from a fixed offset within a state block.
- `jump_to_return_immediately_3`: Jump stub: passes control straight on to return_immediately_3.
- `jump_to_return_from_subroutine`: Jump stub: passes control straight on to return_from_subroutine.
- `update_state_seed`: Adds a constant to a value returned by another routine, stores the result in a state field, and returns the upper 15 bits.
- `configure_resource`: Clears three global variables, attempts to open a resource, and configures it with provided parameters if successful.
- `stub_undecoded_single_call`: A one-instruction routine reached only as a gap in the file, which calls one other unnamed routine.
- `or_five_source_arrays`: Loops over 2000 words, ORing five source arrays spaced 2000 words apart and writing each combined word to a destination array 10000 words past the first source.
- `call_routine_with_span_length`: Reads a first field and two 16-bit fields from the parameter block, computes an inclusive span length as the difference between the two fields plus one, and passes the first field, the start field and that length to one routine.
- `sum_call_results_with_offset`: Calls one routine to fill a local buffer, runs two counting loops with empty bodies, makes several calls, and adds a value at a fixed offset from the base register, a local byte and a call result.
- `convert_value_to_word`: Returns zero when its argument is zero.
- `configure_state_by_mode`: Copies six entries from a fixed table into a local, repeatedly calls one routine and another until the first returns zero, and copies a few words into a state block.
- `wait_for_flag_and_lookup`: Busy-waits until a flag byte in the state block is nonzero, copies that flag, clears it, and returns the entry of a byte table at the index given by the flag value, widened to 16 bits.
- `store_mode_flags_from_state`: Copies two words from fixed memory locations into the state block, sets a boolean word to one when a stored state word equals one, and stores two in a byte when that state word equals two and zero otherwise.
- `return_absolute_word_value`: Returns the absolute value of its argument, truncated to 16 bits, by negating it when it is below zero.
- `stub_empty_return_2`: A routine with no body beyond return, so calling it does nothing.
- `call_routine_and_return`: Calls one other routine with no arguments and returns, forwarding control to it without any other work.
- `change_stored_selection`: Compares a 16-bit argument with a stored selection word and, when they differ, releases the previous selection if one was set.
- `store_pair_into_pointer_list`: Copies the two 32-bit words of its argument block into every non-null destination pointer in the trailing stack argument list, stopping at the first null entry, and returns zero.
- `call_routine_with_row_address`: Takes a 32-bit value, uses its upper 16 bits times 64 plus a fixed base offset as an address, passes that address to one routine, then calls a second routine with no arguments.
- `adjust_offset_and_pass_on`: Calls one routine, subtracts a stored word from the value pointed to by its argument, adds 3600 when a stored flag is nonzero, and passes the address of the result to another routine.
- `store_one_into_stub`: Stores one into a single location and returns.
- `stub_empty_handler`: A handler reached through a pointer stored in data, with no body beyond return, so calling it does nothing.
- `return_immediately_5`: The routine performs no operations and returns immediately to the caller.
- `return_from_subroutine_2`: the routine is empty and performs no operations before returning.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `return_from_handler`: This routine is empty and performs no operations before returning to its caller.
- `call_single_routine_handler`: A handler reached through a pointer stored in data.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_handler_2`: This routine is an empty function that immediately returns to its caller.
- `return_immediately_3`: This routine is empty and returns to its caller without performing any operations.
- `perform_no_operation_4`: This routine is empty and performs no actions.
- `call_routine_and_return_2`: Calls one other unnamed routine with no arguments and returns.
- `perform_no_operation_5`: The routine is empty and performs no actions before returning.
- `stub_empty_return_3`: A routine with no body beyond return, reached only as a gap in the file, so calling it does nothing.
- `return_without_output`: This routine performs no operations and immediately returns to the caller.
- `nop_padding_byte_2`: This routine is empty and contains no executable instructions.
- `store_zero_into_stub`: Stores zero into a single location and returns, reached through a handler pointer stored in data.
- `clear_bit_when_flag_set`: Returns zero unless a global flag byte is nonzero.
- `refresh_state_from_record`: Calls one routine, then when a global flag is set and a record pointer is nonzero it clears the low bit of a fixed byte and copies a state byte from the record into the global flag.
- `call_routine_per_string_char`: Walks a null-terminated byte string and calls the routine passed as its first argument once for each character, stopping at the terminator.
- `expand_text_format_codes`: Walks a text string and sends each ordinary character to a callback routine.
- `expand_text_format_codes_2`: Same text escape walker as its neighbouring routine: it sends ordinary characters to a callback and handles percent-sign codes that store argument values into a state record, emit strings or values, or invoke the callback.
- `call_routine_with_argument_list`: Calls one routine with the address of a routine as the first argument, its own argument, and the address of its incoming stack-argument list.
- `call_routine_with_argument_list_2`: Calls one routine with the address of a routine as the first argument, its own argument, and the address of its incoming stack-argument list.
- `call_routine_and_return_3`: Calls one other unnamed routine with no arguments and returns, forwarding control to it without any other work.
- `append_character_and_terminate`: Writes the low byte of its argument at the current position of a pointer stored in a state record, advances that pointer by one, and writes a zero byte at the new position so the buffer stays null-terminated.
- `handle_newline_or_emit_char`: Treats a carriage return and a line feed by making one call each.
- `run_callback_and_return_status`: Clears a 16-bit status word, saves its first argument as a pointer, and calls one routine with a callback, its second argument and the address of its trailing stack arguments.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_immediately_2`: This routine performs no operations and returns to its caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_then_call_pointer`: If its output pointer is non-null, copies sixteen 32-bit entries from a fixed table into that buffer.

### Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:intro

- `open_graphics_library_entry`: Program entry point.
- `wait_for_fire_button_release`: Clears the sprite zero pointer and the scroll control word, then loops while polling a counter and a fire-button byte, calling three routines each pass until the button bit clears.
- `program_blitter_and_expand_bits`: Waits for a status flag, then programs the blitter's control, mask, pointer and size registers.
- `program_blitter_and_expand_bits_2`: Programs the blitter with the same register sequence and bit-expansion loop as its neighbouring routine, but without returning a value from its final library call.
- `program_blitter_and_expand_bits_3`: Programs the blitter with the same register sequence and bit-expansion loop as its neighbouring routines, calling four unnamed routines and returning nothing.
- `read_next_offset_byte`: Reads the next byte at a cursor, wrapping the cursor back to the start of a buffer when it reaches a sentinel value of minus one.
- `call_handler_then_set_bit`: Calls the routine stored at a fixed pointer, then sets bit one of a global flag byte.
- `call_indirect_handler`: Calls a routine through a function pointer stored at a fixed location plus an offset, with no arguments.
- `reset_audio_channels_dma`: Zeroes four audio-related registers, writes fifteen to a DMA control location and then clears bit one of it.
- `build_four_entry_table`: Sets four base pointers into a table, clears a block of 32 longs, and loops four times writing structure entries whose pointers and constants come from base offsets and a lookup.
- `install_handler_once`: On its first use it saves the current global value, points the global at a routine and sets a one-time flag.
- `step_channel_sequence_counters`: Calls one routine four times, storing channel selector values of 1, 2, 8 and 4 before each call.
- `nop_routine`: The routine is empty and performs no operations.
- `step_sequence_record_state`: A large routine with 477 instructions whose pseudo-code is truncated in the bundle.
- `advance_four_audio_channel_states`: Advances a state byte for each of four channels through a one, two, three sequence.
- `negate_global_word`: Negates a word stored at a location calculated by adding an offset to a global variable.
- `blend_two_row_pairs`: When each of two counters is nonzero, it fills a 32-entry destination row with the midpoint of two source rows, using signed byte values offset by 128 before averaging and then restored.
- `empty_routine`: The routine contains no executable instructions.
- `empty_routine_2`: The routine is empty and contains no executable instructions.

### Ultima VI - The False Prophet (demo) (1992)(Mindscape)[h TRSi].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
