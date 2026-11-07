# Game 1892 (gloom - special edition 97 (1997)(black magic)(disk 1 of 3)[a][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01892). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1244 raw-word lines |
| Functions named and described | pass, 369 of 369 named (0 author symbols), 369 of 369 described (9 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6390 of 6394 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 798 |
| code | 18140 |
| data | 654868 |
| padding | 2546 |
| unused | 224768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| lzx | 2 | 3153 | 181 |
| arp.library | 3 | 88 | 7 |
| Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[a][HD].adf:lzx | 2 | 3153 | 181 |

## Cases handled

- gloom - special edition 97 (1997)(black magic)(disk 1 of 3)[a][hd]/Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[a][HD].adf:GloomSE1.LZX: high-entropy data file (7.99 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### lzx

- `start_process_with_message_port`: Program entry that allocates memory, opens the dos library by name and duplicates the current directory lock.
- `release_process_resources_reply`: Releases process resources by unlocking a directory lock, closing a library, freeing allocated memory and replying to a message, with task switching forbidden during the message step.
- `call_stored_vector_with_global`: Calls a routine through a function pointer stored in a global, passing the address of a global variable, and then returns.
- `call_stored_vector_no_return`: Calls a routine through a function pointer stored in a global, passing the address of a global variable.
- `tokenize_command_line_arguments`: Walks a command-line string, skipping spaces, tabs and newlines, and splits it into tokens.
- `check_stream_and_set_flag`: Calls a helper on a fixed buffer, then a second helper on its result.
- `set_status_byte_to_one`: Sets a status byte in a structure to one and returns.
- `advance_counter_on_matched_read`: Checks a status byte and reads a chunk through a helper.
- `add_length_on_full_read`: Calls a helper to read data and compares its result with the requested length.
- `add_length_to_running_counter`: Adds a length value to a running counter in a structure, with no return value.
- `free_linked_node_list`: Walks a singly linked list, releasing each node through a helper call with a fixed size, then clears the list head and tail pointers.
- `clear_list_head_and_tail`: Sets the head and tail pointers of a list in a structure to zero and returns.
- `pack_six_flags_into_byte`: Packs six flag bytes in a structure into one value by OR-ing in the bit values four, eight, sixteen, thirty-two, sixty-four and one hundred and twenty-eight for each nonzero flag.
- `process_command_line_arguments`: Parses a string-based argument list and updates a global configuration structure with specialized settings, such as numeric limits, flag toggles, and or certain parameter values based on detected command characters.
- `scan_next_option_token`: Advances through an argument list, skipping entries that begin with a dash, and copies the first one or two characters of the next argument into an output byte.
- `scan_option_argument_entries`: Advances through an argument list in the same way as its neighbour but handles more cases.
- `next_nonoption_argument_2`: Walks an argument list, skipping dash-prefixed options and noting a double-dash terminator.
- `parse_slash_separated_range_fields`: Parses a slash-separated text field whose first number must fall in a fixed range, storing it as a character offset from the letter N. Further numeric fields are checked against bounds, and the routine returns one on a bad field and zero otherwise.
- `parse_slash_field_with_bounds`: Continues parsing slash-separated fields, storing a numeric field after the first and checking it against a bound.
- `parse_slash_field_remainder`: Skips a leading slash, then parses a numeric field and checks it against a bound.
- `parse_bounded_numeric_fields`: Parses a numeric field stored at a fixed offset and checks it against an upper bound of twelve, then parses a following slash-separated field checked against thirty-one.
- `parse_field_bounded_thirty_one`: Skips to the next slash-separated field and parses a numeric value that must not exceed thirty-one.
- `check_field_bounded_thirty_one`: Skips a leading slash, then parses a numeric value that must not exceed thirty-one.
- `store_bounded_field_thirty_one`: Parses a numeric value and stores it, returning one when the value is zero or above thirty-one and zero otherwise.
- `return_zero`: the routine is empty and returns a zero or null value.
- `initialise_playback_state_fields`: Sets a block of state flags and parameter fields in a structure to fixed starting values, including several counters and constants.
- `pause_task_until_key_press`: Looks up the current task and changes its priority through library calls when a pause flag is set.
- `skip_characters_in_class`: Scans a string through a character-class table, advancing past characters whose class bit is set, then calls one helper.
- `run_callback_list_then_helper`: Walks a linked list of callback nodes, calling the routine stored in each node, then makes one helper call with its parameter and returns.
- `configure_handler_and_dispatch`: Calls three helpers in sequence, makes two more calls when a mode byte is below two, installs a handler pointer in the structure, and then calls a helper with a stored configuration byte.
- `open_arp_library_and_set_mode`: Looks up the current task and, depending on a stored size field, either sets a flag and opens a library by name or sets a bit in a related structure.
- `forward_call_to_helper`: Calls one unnamed helper routine and returns.
- `save_and_clear_structure_slot`: Saves a 32-bit field from a structure into a global slot, overwrites that field with minus one, and sets a flag byte.
- `call_temp_file_name_helper`: Calls a helper with an address computed from a per-entry structure, passing the file-name template referenced in the text for a temporary LZX compression file.
- `clear_block_match_name`: Clears a block of 144 bytes, then compares the given name with a stored string and sets a result code when they match.
- `compare_name_against_constants`: Compares a name with a stored string and, on a match, sets a result code.
- `match_name_then_set_result`: Compares a name with a stored string and sets a result code on a match.
- `match_name_or_open_file`: Compares a name with a stored string and, when it does not match, passes the name and a fixed length to a helper.
- `process_file_handle`: Attempts to open a file or retrieve a handle, writing a returned handle to a pointer and potentially calling a list of handlers.
- `store_handle_or_close_on_failure`: Calls a library function to obtain a handle.
- `store_data_register_through_pointer`: Writes a data register value through an address register pointer and returns zero.
- `open_dos_file_handle`: Calls the DOS library's Open entry through the library base and stores the returned handle through the caller's pointer when the result is nonzero.
- `store_data_register_via_pointer`: Writes a data register value through an address register pointer and returns zero, with no other effects.
- `close_handle_and_clear_record`: If the handle pointer's stored value is nonzero, closes it through the DOS library; otherwise it reports the internal error text that a file handle was already closed.
- `close_file_and_clear_buffer`: Closes a file through the DOS library's Close entry and then zeroes 144 bytes starting at the pointer held in an address register.
- `read_dos_file`: Checks a parameter and calls a DOS library read routine if the parameter is greater than zero.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `forward_write_to_dos`: Calls the DOS library's Write entry through the library base with the arguments it was given, and returns nothing.
- `cache_standard_input_output_handles`: Stores the DOS standard input handle in a context record.
- `execute_dos_function`: Checks a flag in a structure and calls a DOS library function if the flag is cleared.
- `cleanup_and_report_user_abort`: Runs a cleanup routine and then the error reporting routine with the global message buffer.
- `set_flag_and_write`: Checks a flag in a structure, calls another routine with a constant, and then sets a byte in a structure to one.
- `set_flag_and_reset`: Checks a flag in a structure, calls another routine with a constant, and then sets a byte in a structure to zero.
- `write_to_dos`: Checks a flag and a pointer in a structure before calling a DOS library write routine.
- `write_clamped_blocks`: Iteratively calls a function pointer with a size clamped to a certain maximum if a flag is clear and a pointer is valid, decreasing a counter until it reaches zero.
- `write_blocks_loop`: Iteratively calls a DOS write routine with a decreasing or clamped block size until a count reaches zero.
- `write_fixed_blocks_until_done`: Writes blocks of 1024 bytes through DOS Write and calls a check between blocks until the remaining count is exhausted.
- `format_and_write_message`: When output is enabled and an output handle is set, formats a text line into a context buffer using a helper routine, then writes it through DOS Write with the length measured from its terminator.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_dos_read`: Checks a flag in a structure and calls a DOS library read routine if the flag is cleared.
- `execute_conditional_function`: checks a byte in a global structure and, if it is zero, calls a function pointer retrieved from a table offset by a fixed displacement.
- `read_with_surrounding_routines`: Runs one routine before and one after a DOS Read call, with extra routine calls around the pair when a status byte is set.
- `read_char_matching_set`: Reads characters through a helper and echoes each one through the text formatter, returning the first one that appears in the supplied string.
- `read_until_expected_char`: Reads and echoes characters until the input matches the character at the current position of the supplied string, folding letter case through the character-class table.
- `read_next_matching_character`: Advances through the supplied string and reads echoed characters until one matches the current string position, case-folding letters through the character-class table.
- `stub_empty_return`: Does nothing and returns, with no library calls, text or hardware access.
- `bracket_routine_with_messages`: Invokes the message routine with the global buffer, then runs a routine with no arguments, then invokes the message routine again with the global buffer.
- `open_intuition_compute_ratio`: Opens the intuition library through exec, looks up layout fields from the returned screen record and divides them to produce a clamped integer ratio.
- `contains_reserved_filename_char`: Copies an eight-character reference set from a global string and returns one if any character of the supplied string is in that set, or zero at its terminator.
- `scan_string_for_stack_set`: Walks a string held in an address register and returns one when a character matches any of eight entries from the caller's stack-passed set, or zero at the terminator.
- `scan_for_eight_char_set`: Walks a string through an address register and returns one when a character matches any of eight entries in the set passed on the stack, or zero when the string ends.
- `stub_return_zero`: Returns zero immediately, with no other effects.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `free_memory_block`: Calls the exec library's FreeMem entry for a block.
- `recursively_halve_memory_pointer`: Recursively calls itself while halving a pointer value and updating a destination pointer, terminating or resetting if a certain threshold or certain value is met.
- `process_checksum_data`: Modifies a value in a pointer, checks it against a constant, and calls another routine or a routine with a global variable address.
- `forward_two_arguments`: Forwards its two arguments unchanged to another routine and returns nothing.
- `consume_bits_refill_word`: Discards a number of bits from the LZX bit buffer by shifting it, and when fewer than sixteen bits remain, refills it with the next 16-bit word from the input stream.
- `read_bit_field_value`: Returns the low bits of the bit buffer as a field of the requested width, then consumes that many bits from the buffer.
- `decode_symbol_eight_bit_prefix`: Decodes one symbol by indexing a prefix table with the low eight bits of the bit buffer, following child tables for longer codes, and then consumes the code length given by a second table.
- `decode_symbol_six_bit_prefix`: Decodes one symbol by indexing a prefix table with the low six bits of the bit buffer, following child tables for longer codes, and then consumes the code length given by a length table.
- `reset_bit_buffer_from_input`: Loads the next two 16-bit words from the input stream into the bit buffer and sets the bit count to sixteen.
- `read_block_of_256_bytes`: Fills a 256-byte block by calling the byte reader once for each position, in order.
- `read_bytes_and_sum`: Reads a requested number of bytes through the byte reader into a buffer and returns their 16-bit sum.
- `build_huffman_table`: Allocates memory, initializes several pointers in a structure, and performs a sequence of memory-related and bit-stream processing tasks.
- `count_newline_characters`: Counts the newline characters in a zero-terminated string and returns the count.
- `invoke_routine_with_one`: Calls another routine with the constant one and returns nothing.
- `write_lines_within_width`: Writes a buffer line by line through the output routine, using a width computed from a stored offset, and reports an error when the abort check signals a break.
- `pack_four_nibbles`: Combines four bytes from a buffer into one 16-bit value by shifting in four bits per byte, and returns it.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `set_flag_and_call`: Checks a flag in a structure, sets that flag to one, and calls another routine with a zero argument.
- `compare_path_tails`: Finds the last path separator in each of two paths, then compares the text from those points with a helper.
- `check_path_tail_matches`: Finds the last path separator in a string and calls a comparison helper on the text from that point.
- `test_helper_result_is_zero`: Calls a helper and returns one when its result is zero, otherwise zero.
- `return_zero_2`: The routine returns a constant zero value.
- `check_path_ends_with_slash`: Returns one when the final character of the string is a forward slash, otherwise zero.
- `return_zero_unreached`: Returns a zero value and is part of an unreached code gap.
- `normalize_path_separators`: Replaces every backslash in a zero-terminated string with a forward slash, in place.
- `return_void`: Returns without performing any operation.
- `split_path_into_parts`: Splits a path into its device, directory and file name parts, clearing and filling each supplied buffer, and returns a value that marks where the device colon sits.
- `copy_path_segment_or_whole`: Copies a path into a buffer, taking either the whole string or the portion after a computed offset, and terminates it at the directory boundary.
- `copy_path_tail_trimmed`: Copies the remainder of a path after an offset into a buffer, then terminates it at the directory or device boundary found by scanning back for a slash or colon.
- `terminate_path_parts`: Writes terminators into path buffers at the directory and device boundaries computed from a separator offset, so the copied text is trimmed.
- `terminate_device_and_dir_fields`: Writes a terminator into the directory field when its buffer is supplied, and also terminates the device field when the character at the separator offset is a colon.
- `find_list_entry_for_request`: Walks a linked list of named entries and returns the first entry that matches the request, using an optional DOS pattern test and an optional case-folded copy of the name.
- `find_list_entry_with_mode`: Walks a linked list of named entries and returns the first one that compares equal to the request, copying names into stack buffers when a mode bit is set.
- `find_first_entry_compare`: Walks a linked list and returns the first entry whose name compares equal to the request, or zero when the list is exhausted.
- `scan_list_for_next_match`: Scans a linked list from the current entry and returns the next entry whose name matches the request, optionally using a DOS pattern check.
- `scan_list_with_dos_check`: Checks a DOS library condition before each step through a linked list of names, returning the first entry that satisfies the comparison.
- `scan_list_for_match_loop`: Repeatedly steps through a linked list of named entries and returns the first entry that compares equal to the request.
- `return_zero_3`: The routine is empty and always returns a zero value to the caller.
- `append_optional_unique_entry`: Adds a named entry to the end of a linked list.
- `append_unique_list_entry`: Scans the list for an existing entry with the same name, then allocates a new named record, zeroes it and appends it to the list.
- `append_list_entry_allocating`: Allocates a named record sized to hold the name, zeroes its header, copies the name into it and appends it to the list.
- `append_prepared_list_entry`: Zeroes a prepared header block, copies the given name into it, sets its entry flag and appends it to the list.
- `append_entry_or_report_error`: Appends a new entry to the tail of a linked list, and reports an internal list error through the message routine when the list has no tail.
- `append_entry_to_tail`: Links a new entry after the current tail of a linked list and makes it the new tail, without checking whether the tail exists.
- `open_output_file_or_report`: Opens a named output file through a helper and reports a cannot-open message containing the file name when the open fails.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `close_file_handle`: Checks if a certain global memory location contains a non-zero pointer and calls another routine using that pointer.
- `allocate_decoder_buffers`: Allocates the decoder working buffers, starting with a table sized from a stored count and followed by two 128K blocks, a block sized from a stored parameter, two 32K blocks and a 4K block.
- `allocate_window_and_table_buffers`: Allocates two 128K blocks, a block sized from a stored parameter, two 32K blocks and a 4K block, then initializes a 768-byte table.
- `allocate_secondary_window_buffers`: Allocates one 128K block, a block sized from a stored parameter, two 32K blocks and a 4K block, then initializes a 768-byte table.
- `allocate_block_and_tables`: Allocates a block sized from a stored parameter, two 32K blocks and a 4K block, then initializes a 768-byte table.
- `allocate_pair_32k_then_tables`: Copies a buffer pointer into a window slot, then allocates two 32K blocks, a 4K block and a 768-byte table, reporting an error and returning one if any allocation fails.
- `allocate_single_32k_then_tables`: Allocates one 32K block, a 4K block and a 768-byte table, reporting an error and returning one if any allocation fails.
- `allocate_4k_then_table`: Allocates a 4K block and then initializes a 768-byte table, reporting an error and returning one if the allocation fails.
- `init_table_and_finish_setup`: Initializes a 768-byte table, then sets two pointers to the midpoints of the 128K blocks and marks the buffers ready.
- `finish_buffer_setup`: Sets two pointers to the midpoints of the 128K blocks, calls two helpers, marks the buffers ready and returns zero.
- `free_decoder_buffers`: Releases every decoder buffer that is still allocated, including the table, the 128K blocks, the 32K blocks and the 4K block, clearing each stored pointer.
- `free_optional_block`: Releases a memory block recorded in the context when its pointer is set, using a size stored in the context, and then clears the pointer.
- `flush_pending_output_span`: When the first stored position is behind the second, writes the bytes between them through a helper and reports an error if that fails.
- `write_span_with_abort_check`: Returns one early when a status flag is clear and an abort check reports a break.
- `flush_buffer_with_flag_one`: Calls the buffer flush helper on a context buffer with the constant one and returns nothing.
- `allocate_history_block`: Allocates a 7,728-byte working block and, when that succeeds, stores three pointer fields into it at fixed offsets.
- `set_history_block_pointers`: Sets three pointer fields in the context to fixed offsets within the previously allocated working block and returns zero.
- `free_history_block`: Releases the 7,728-byte working block when its pointer is set, then clears the pointer.
- `build_doubling_run_table`: Fills a 512-entry byte table with values that step upward in runs whose lengths double, and performs this only once per context.
- `fill_doubling_run_table_rest`: Continues filling the same 512-entry table from its current position, with runs of stepping values whose lengths keep doubling.
- `fill_alternating_run_table`: Fills the same 512-entry table with alternating runs of values, doubling the run length on each pass, until the table is full.
- `allocate_zeroed_table_set`: Allocates a zeroed block sized from a count and stores four table pointers into the caller's record at fixed offsets.
- `zero_and_link_table_set`: Zeroes a block sized from a count and stores four table pointers into the caller's record at fixed offsets.
- `fill_and_link_table_set`: Fills a buffer with one repeated byte value and stores four table pointers into the caller's record at fixed offsets.
- `free_table_set`: Releases the block referenced by a table record when it is set, using a size computed from a count, and clears the record's four table pointers.
- `invoke_raw_do_format`: Calls the exec library's RawDoFmt entry through the global library base.
- `invoke_raw_do_format_2`: Calls the exec library's RawDoFmt entry through the global library base, with its format and argument data supplied by the caller.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `update_checksum_table_driven`: Updates a running CRC-style checksum by stepping through the input bytes with a 256-entry lookup table, processing sixteen bytes per pass and then the remainder.
- `run_checksum_in_megabyte_steps`: Clears the checksum state when given no buffer.
- `step_checksum_pieces_with_remainder`: Steps through a byte count in one-megabyte pieces, calling a helper for each piece, and handles the partial remainder with one final helper call when it is smaller than one megabyte.
- `step_checksum_remainder_last`: Calls a helper once per one-megabyte piece of a byte count, then makes one more helper call for the leftover bytes.
- `build_huffman_decode_table`: Counts the code lengths in an input array, derives canonical code offsets and fills a decoding table with the resulting codes.
- `run_pointer_list_handlers`: Steps through a zero-terminated list of handler pointers, calling each one through its entry, and sets a done flag when the list ends.
- `run_pointer_list_until_done`: Walks a zero-terminated list of handler pointers and calls each handler entry, sets a done flag once the list is exhausted, and returns the last handler result.
- `call_indirect_handler_return_constant`: Calls a handler through a pointer stored in a global variable, passing the address of a global variable, then returns the constant twenty.
- `invoke_global_callback`: Calls a function pointer stored in a global table, passing the address of a global variable as its argument, then returns.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time, converting lowercase letters to uppercase before comparing.
- `parse_signed_decimal_value`: Reads an optional plus or minus sign followed by decimal digits from a string, storing the signed value through an address register.
- `lowercase_string_in_place`: Converts characters flagged in a class table to lowercase in place within a zero-terminated string, and returns the string pointer.
- `lowercase_string_via_pointer`: Converts characters flagged in a class table to lowercase in place within a zero-terminated string addressed by a pointer register.
- `write_fill_then_lowercase_run`: Writes a fill byte at the current buffer position, then continues through the following characters and lowercases flagged letters until the terminator.
- `uppercase_string_in_place`: Converts characters flagged in a class table to uppercase in place within a zero-terminated string, and returns the string pointer.
- `uppercase_string_via_pointer`: Converts characters flagged in a class table to uppercase in place within a zero-terminated string addressed by a pointer register.
- `write_fill_then_uppercase_run`: Writes a fill byte at the current buffer position, then continues through the following characters and uppercases flagged letters until the terminator.
- `drain_pending_list`: Iterates through a linked list, updating pointers to remove nodes and executing function pointers or conditional callbacks.
- `drain_pending_list_nodes`: Repeatedly removes nodes from a pending list and frees each one through the exec library's FreeMem entry.
- `finish_call_with_result`: Stores the parameter in a stack-frame slot, calls an optional handler from a context record, runs a helper, and then calls a library entry on the frame.
- `split_command_line_words`: Splits a command line into words separated by spaces, tabs or newlines, keeping double-quoted text together, and counts the words first.
- `free_word_array_if_set`: Releases the parsed word-pointer array when both its count and its pointer are set, using FreeMem, and returns the library result.
- `compare_bounded_strings_signed`: Compares up to a given number of characters of two strings and returns zero when they are equal.
- `return_signed_comparison_result`: Returns minus one when the processor condition flags show a signed less-or-equal result, and one otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `append_string_to_buffer`: Finds the terminating zero byte of the destination buffer and copies the source string onto that position, including its terminator.
- `copy_bounded_string_return_dest`: Copies characters from a source string into a destination buffer up to a given count, stopping at the source terminator and zero-filling the remainder.
- `copy_bounded_string_padded`: Copies characters from a source string into a destination buffer up to a given count, stopping at the source terminator and zero-filling the remainder.
- `zero_fill_buffer`: Writes zero bytes into a buffer for the given count and returns nothing.
- `check_first_char_class`: Looks up the first byte of a string in a character class table and calls a helper when that class bit is clear.
- `run_handler_list_with_arg`: Calls each handler in a context list through its entry pointer, then calls a helper with the parameter.
- `run_handler_list_then_helper`: Calls each handler in a context list through its entry pointer, then calls a helper with no arguments and returns nothing.
- `drain_pending_list_frame`: Saves several context fields into a stack frame, then drains a pending list through library calls while the count is nonzero.
- `call_helper_and_return`: Calls another routine and then returns, with no other operations on data, registers or memory.

### arp.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `scan_arp_list_entries`: Within the arp shared library, stores the argument in a record and fetches up to four entries from a library list through a library call.
- `continue_arp_entry_scan`: Within the arp shared library, continues scanning the entry list from a stored count.
- `attach_arp_entry_record`: Within the arp shared library, sets up a record from a given entry by storing a derived pointer and calling two library entries.
- `retry_arp_step_until_flag`: Within the arp shared library, repeatedly calls a helper and a library entry until a flag is set.
- `clear_arp_link_and_call`: Within the arp shared library, clears a pointer field in the given record and calls an entry through a library pointer stored in the base record.
- `allocate_arp_linked_node`: In the arp shared library, allocates a memory block with AllocMem when given a nonzero size, and stores a link value offset by four in its first word.

### Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[a][HD].adf:lzx

- `run_program_entry`: Program entry point.
- `release_message_and_resources`: Stores a result in a stack-frame slot, calls an optional handler, then releases file locks, library bases and messages through exec and DOS library entries.
- `invoke_global_callback_3`: Calls a function pointer from a global table and passes the address of a global variable as an argument.
- `invoke_handler_with_constant`: Calls a function pointer from a global variable using a global variable as an argument and returns the constant twenty.
- `tokenize_command_string`: Parses a string into an array of pointers to individual words, handling quoted text and replacing certain escape or wildcard characters.
- `attempt_open_or_set_flag`: Clears a status byte, then calls a helper with a name buffer.
- `mark_status_flag_set`: Sets a status byte in the context to one and returns zero.
- `validate_and_update_stream`: Checks a flag in a structure and, if a stream read or helper call returns a certain length, updates a running counter in that structure.
- `check_span_write_result`: Calls a helper with a context buffer and compares its result with an expected count.
- `add_count_to_running_total`: Adds a count held in a register to a running total field in the context and returns zero.
- `free_node_list_entries`: Walks a linked list of nodes, calling a release helper on each node with a fixed code, then clears the list head and tail fields and returns zero.
- `clear_node_list_head_tail`: Sets the head and tail pointers of a context list to zero.
- `pack_option_flags_into_byte`: Builds one byte of option bits from eight context flag bytes, setting each bit whose source flag is nonzero, and stores it through the output pointer.
- `parse_command_line_arguments`: Parses command-line arguments or configuration options by checking character values and setting or updating various configuration flags and parameters in a global state structure.
- `next_option_argument_text`: Advances through an argument vector, recording the first option letter and returning the text that follows it, or zero when the vector is exhausted.
- `scan_string_table`: Iterates through a table of pointers, updating a counter and checking for specific characters or delimiters to locate or skip certain string contents.
- `next_nonoption_argument`: Returns the next argument from a vector.
- `parse_slash_separated_triple`: Parses a slash-separated triple from a string: a value between 1970 and 2099 that is stored as a letter offset, followed by two bounded numbers.
- `parse_range_fields`: Parses a string looking for forward slash delimiters and validates that any parsed numeric values fall within a specific range.
- `parse_month_day_fields`: Skips a leading slash, then parses a month between one and twelve and a day between one and thirty-one from a slash-separated string, storing each in a record.
- `parse_month_and_day_tail`: Parses a month between one and twelve and then a day between one and thirty-one from a slash-separated string.
- `parse_day_after_separator`: Scans forward to the next slash, skips it, and parses a day between one and thirty-one into a record.
- `parse_day_field_after_slash`: Skips a leading slash and parses a day between one and thirty-one into a record.
- `store_day_field_value`: Calls a helper to obtain a number and stores it as a day field, returning one when the value is zero or above thirty-one.
- `return_zero_constant`: The routine returns a constant zero value.
- `initialize_default_options`: Sets the default option and buffer-size fields in the context, copies a default name string into a context buffer, and selects a size value from a mode byte.
- `handle_pause_request`: Adjusts the task priority through exec when a flag is set and toggles a state byte.
- `traverse_character_class`: Iterates through a memory location using a character value as an offset and calls another routine if a bit in a lookup table is not set.
- `execute_handler_list`: Traverses a linked list of function pointers and executes each function in the list.
- `initialize_and_run_step`: Runs three setup helpers, then calls a helper with the value one hundred when a mode byte is below two.
- `detect_environment_mode`: Looks up the current task and opens the arp library when a version field meets a threshold, otherwise it flags the task.
- `execute_empty_wrapper`: The routine calls another routine and then returns.
- `save_and_clear_task_field`: Saves a field of a task record into a context slot, overwrites that field with minus one, and sets a status flag.
- `format_temp_file_name`: Formats a temporary file name from a prefix and a numeric suffix derived from a stored value, using the referenced name pattern, and returns nothing.
- `identify_file_type_and_open_handle`: Clears a memory buffer and compares an input string against a seriess of hardcoded string constants to determine a file type.
- `compare_and_resolve_handle`: Compares a string against a global string, and if no match is found, attempts to resolve a file handle or string through a series of library calls, potentially storing a returned handle in a pointer.
- `compare_and_open_file`: Compares a string against a global string and, if no match is found, attempts to open a file or retrieve a handle, potentially storing a returned handle in a pointer.
- `compare_string_and_open_file`: compares a string against a global variable and, if a match is found, attempts to open or retrieve a file handle via a function pointer table.
- `open_file_with_mode_code`: Reports a fixed message, then opens the file using the current mode code and stores the handle through the output pointer.
- `store_handle_or_release`: Calls a library entry and stores its nonzero result through the output pointer.
- `store_register_value`: Writes the contents of a data register to a location pointed to by an address register and returns zero.
- `open_file_and_store_handle`: Calls a library function to open a file and, if successful, stores the resulting handle into a pointer-referenced location.
- `store_register_and_return_zero`: Writes a data register value to a location pointed to by an address register and returns zero.
- `close_file_and_clear_buffer_2`: Closes a file using the DOS library and then zeroes one hundred and forty-four bytes starting at a pointer passed in an address register.
- `close_handle_and_clear_record_2`: Closes a file using the DOS library and then zeroes a block of memory using a pointer stored in an address register.
- `read_file_conditionally`: Checks if a provided integer is at least one, then calls the DOS read routine if the condition is met.
- `forward_read_to_dos`: Calls the DOS library's Read entry through the library base and returns nothing.
- `write_data_to_file`: Calls the DOS write routine using a function pointer or pointer obtained from a global library base.
- `update_input_output_buffers`: Calls a DOS input routine and stores the result in a global variable, then calls a DOS output routine or clears a global variable based on a flag.
- `dispatch_dos_message`: Checks a flag in a global variable and calls a DOS library routine if the flag is zero.
- `invoke_subroutine`: Calls a routine with a pointer to a global variable.
- `set_write_flag_and_log`: Checks a flag in a structure and, if not set, calls a routine to write a value and sets a certain byte in the structure to one.
- `clear_write_flag_and_log`: Checks a flag in a structure and, if not set, calls a routine to write a value and sets a certain byte in the structure to zero.
- `write_output_if_available`: Checks if an output flag is unset and a pointer is non-zero, then calls the DOS write routine.
- `write_blocks_iteratively`: Iteratively calls a DOS-related routine with a clamped or decreasing block size until a counter reaches zero, provided certain flags in a structure are met.
- `write_blocks_until_exhausted`: Iteratively calls a DOS write routine with a maximum block size of one thousand twenty-four bytes until a remaining count reaches zero or a call returns non-zero.
- `execute_write_loop`: Executes a loop that calls a function pointer via a pointer or table, using a decreasing counter or a fixed limit.
- `format_and_write_string`: Calls a formatting routine and then writes a null-terminated string to a file using the DOS write routine if certain flags and pointers are valid.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `call_dos_if_flag_clear`: Checks a flag byte in the global data block and, only when it is zero, calls a DOS library function through the library base.
- `call_dos_if_flag_clear_2`: Behaves like its neighbour: when the flag byte in the global data block is zero it calls the DOS library function, and otherwise it returns without action.
- `read_dos_data_block`: Calls a helper, then a DOS Read through the library base, then the helper again, with an extra helper call on each side when a flag byte is set.
- `match_input_char_in_set`: Reads characters through a helper and passes each one to a second helper, folding lowercase letters to uppercase where the class table marks them.
- `read_input_until_match`: Reads input characters through a helper and passes each to a second helper, stepping through the argument string until the typed character equals the current string byte.
- `advance_input_match_pointer`: Advances the string pointer by one, then reads input characters through a helper until the typed character equals the current string byte.
- `stub_empty_return_2`: Contains only a return instruction, so calling it has no effect.
- `call_helper_between_globals`: Calls a helper with the global data block as its argument, then calls a no-argument helper, then calls the first helper again.
- `compute_visible_text_rows`: Opens intuition.library, takes the active window's height less the screen's top area, and divides it by the screen font's line height to get the visible row count, with a minimum of 8.
- `check_for_wildcard_chars`: Copies an eight-byte set of wildcard and punctuation characters from the global data block, then returns 1 if the argument string contains any of them and 0 otherwise.
- `match_string_char_in_set`: Checks each character of the argument string against an eight-entry set kept in the caller's stack frame, returning 1 at the first match and 0 at the terminator.
- `test_next_char_in_set`: Same test as its neighbour, but the string pointer is advanced before each comparison: it returns 1 when a character matches the stack-held set and 0 at the terminator.
- `stub_return_zero_2`: Returns zero without doing anything else.
- `allocate_memory_via_exec`: Calls the exec library's memory allocation function through the exec library base, leaving the result for the caller.
- `free_memory_via_exec`: Calls the exec library's memory release function through the exec library base to free a block; the caller supplies the argument registers.
- `allocate_buffer_halving_size`: Tries to allocate a block of the requested size, halving the size after each failure while it stays above 256.
- `retry_allocation_halving_size`: Halves the size held in the stored cell before each attempt and stops once it reaches 256 or below.
- `forward_arguments_to_helper`: Passes its two arguments unchanged to a second helper and returns nothing.
- `consume_bits_and_refill`: Subtracts a bit count from the bit reader's counter and shifts the bit buffer right by that count.
- `read_bits_before_consume`: Saves the current bit buffer, consumes the requested number of bits through the bit reader helper, and returns the low bits of the saved buffer.
- `decode_main_tree_symbol`: Decodes one symbol from the main Huffman tree by indexing with the low eight bits of the bit buffer and walking the tree bit by bit for longer codes.
- `decode_six_bit_tree_symbol`: Decodes one symbol using a six-bit lookup table, walking the tree bit by bit for longer codes.
- `reset_bit_buffer_from_stream`: Reads two 16-bit words from the input stream, advances the stream pointer past them, sets the bit counter to 16 and loads the bit buffer with the two words.
- `read_256_raw_bytes`: Reads 256 bytes one at a time through a helper and stores them consecutively at the address given as argument.
- `read_block_and_checksum`: Reads a run of bytes through a helper into the caller's buffer, one more than the count argument, and returns the 16-bit sum of the bytes read.
- `read_lzx_block_header_tables`: Reads the header of an LZX block, including bit fields that give buffer and table sizes, then allocates the block buffer and reports an insufficient-memory error when that fails.
- `count_newlines_in_string`: Walks a null-terminated string and returns the number of newline characters in it.
- `call_helper_with_arg_one`: Calls a helper with the argument one and returns nothing.
- `print_text_wrapped_to_width`: Writes a text block one line at a time, breaking lines at newline characters and at a width obtained from a helper.
- `fold_four_bytes_to_word`: Folds four bytes starting at the argument address into one 16-bit value, shifting the running value left by four bits before adding each byte.
- `do_nothing_2`: This routine is empty and performs no operations.
- `set_initialization_flag`: Checks a flag in a structure and, if not set, sets that flag to one and calls another routine with a zeroed parameter.
- `compare_path_file_names`: Finds the file name after the last slash or colon in each of two paths and compares the two names with a string helper.
- `match_path_basename_via_helper`: Finds the file name after the last slash or colon in the current path and calls a helper on it.
- `test_helper_result_zero`: Calls a helper and returns true when its result is zero, false otherwise.
- `return_zero_4`: The routine always returns a zero or a null-equivalent value.
- `check_path_ends_with_slash_2`: Returns 1 when the last character of the argument string is a forward slash, and 0 when it is not or the string is empty.
- `return_null_integer`: The routine is empty and simply returns a zero or null value.
- `normalize_path_separators_2`: Walks a null-terminated path and replaces every backslash with a forward slash in place, leaving other characters unchanged.
- `return_empty_routine_2`: the routine performs no operations and immediately returns.
- `split_path_into_parts_2`: Splits a path into a volume or device prefix before a colon, a directory part before the last slash, and a file name, writing each into the supplied buffers when they are present.
- `copy_path_name_part`: Copies the file name portion of a path into the output buffer, or the whole string when no separator was found.
- `copy_path_name_after_index`: Copies the file name that follows a known split index into the output buffer.
- `terminate_split_path_parts`: Writes terminating null bytes at the ends of the directory and volume buffers, positioned from the split index, when the path contains a slash.
- `terminate_split_path_tail`: Writes a terminating null into the directory buffer, and when a colon sits at the split position, into the volume buffer as well.
- `find_matching_list_node`: Walks a linked list of named entries and returns the first entry whose name equals the argument string.
- `find_list_node_from_state`: Walks the same kind of named list using the flag bit from the caller's registers and the stack-held name buffers, returning the first matching entry or null.
- `find_list_node_by_helper`: Walks a linked list of entries and returns the first one that passes a name test, with a DOS library check when a flag is set.
- `find_next_list_match`: Tests the current list entry and, if it fails, steps through the list to the next entry that passes the same name test.
- `step_list_until_name_match`: Makes two DOS library calls at the top of each step and returns the current entry if the second result is nonzero.
- `advance_list_name_match`: Steps forward through the entries, testing each name with a helper, and returns the first entry that passes or null at the end of the list.
- `return_zero_5`: the routine returns a zero or null value without performing any operations.
- `add_named_list_entry_checked`: Creates a list entry holding a copy of the name, allocating its memory through a helper and zeroing its header, then appends it to the tail.
- `add_unique_named_list_entry`: Appends a named entry to the list only when no entry with that name already exists, returning null with status 2 otherwise.
- `append_named_list_entry`: Allocates an entry sized for the name, copies the name in, and appends the entry to the list tail without checking for duplicates.
- `initialize_and_append_entry`: Initializes an already-allocated list entry by clearing its header, copying the name in, and appending it to the list tail.
- `append_entry_or_report_error_2`: Appends an entry to the list when the tail pointer is set.
- `link_entry_to_list_tail`: Links an entry onto the list tail and sets the status to zero.
- `open_lzx_output_file`: Opens the output file named in the decoder state through a helper, and when that succeeds passes the name to a second helper.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `close_handle_if_nonzero`: If a certain memory location contains a non-zero value, it calls another routine using that value as an argument.
- `allocate_all_lzx_buffers`: Allocates every LZX decoder buffer in sequence, starting with the block buffer sized from the decoder state, then the windows, tables and table setup.
- `allocate_multiple_buffers`: It calls a memory allocation routine multiple times with different sizes and stores the resulting pointers in a table, or updates a flag if any allocation fails.
- `allocate_window_buffers`: Attempts a sequence of memory allocations of decreasing sizes, updating a global state or table with the returned pointers and checking for null.
- `allocate_lzx_buffers_from_block`: Allocates the decoder buffers starting from the block buffer sized by the decoder state, followed by the table and window buffers.
- `allocate_lzx_first_table_buffers`: Allocates the 32768-byte table buffers starting with the first, then the remaining buffers and the table setup.
- `allocate_lzx_second_table_buffers`: Allocates the buffers starting from a second 32768-byte table, then the remaining buffers, the table setup and the window pointers.
- `allocate_lzx_aligned_table_buffers`: Allocates a 4096-byte buffer, then runs the table setup, returning 1 after a reported failure and 0 on success.
- `init_lzx_tables_and_finish`: Runs a table helper, and on success sets the window pointers, calls two helpers and sets the ready flag, returning false.
- `finish_lzx_buffer_setup`: Sets the window pointers from the decoder buffers, calls two helpers and marks the decoder as ready, then returns zero.
- `free_all_lzx_buffers`: Frees each decoder buffer whose pointer is set, using the size recorded for each one, and clears its pointer.
- `free_optional_lzx_buffer`: Frees the buffer stored at a fixed offset in the decoder state when its pointer is set, using the size held in the state, and clears the pointer.
- `flush_pending_lzx_output`: Writes the bytes between the start pointer and the end pointer through a helper, and when that write succeeds calls a second helper with the value seven.
- `read_lzx_input_bytes_checked`: Reads the requested number of bytes into the input buffer through a helper and adds the count to the running total when all of them arrive.
- `call_state_block_helper_one`: Calls a helper with the decoder state block at a fixed offset and the value one, and returns nothing.
- `allocate_lzx_input_state_block`: Allocates a 7728-byte input state block through a helper, and on success points three decoder state fields into the block at fixed offsets.
- `reset_lzx_input_state_pointers`: Points three decoder state fields at offsets inside the already-allocated input state block, then returns zero.
- `free_lzx_input_state_block`: If the input state block pointer is set, frees the 7728-byte block through a helper and clears the pointer.
- `build_doubling_run_table_2`: Fills a 512-entry byte table in the decoder state, starting with entries zero to three and then writing runs whose length doubles each time.
- `continue_doubling_run_table`: Continues the doubling-run fill of the same 512-entry table from its current index, writing one run per pass until the table end is reached.
- `fill_doubling_run_table_tail`: Fills the remaining part of the same 512-entry table with alternating doubling runs, starting from the current index, until the table is complete.
- `allocate_zeroed_table_block`: Allocates a block whose size is derived from the count argument, zeroes it, and points four table fields into it at fixed multiples of the count.
- `zero_and_set_table_pointers`: Zeroes a block whose size is derived from the count argument, then points four table fields into it at the same multiples of the count.
- `fill_block_and_set_pointers`: Fills a block with one byte value using a countdown loop, then sets four table pointers at multiples of the count within the block.
- `free_table_block_sized`: Frees the table block when its pointer is set, using the size derived from the count argument, then clears its four table pointers.
- `format_text_raw_do_fmt`: Calls the exec library's RawDoFmt through the exec base to format text, with the output handled by a byte-copy routine.
- `execute_formatted_function`: It calls a function pointer located at an offset from a global variable.
- `store_byte_at_pointer`: Stores the low byte of the data register at the address held in the pointer register, then returns.
- `compute_crc32_of_buffer`: Updates a running reflected CRC-32 over a byte buffer using a 256-entry table in the decoder state, processing 16-byte groups before the remainder.
- `process_large_memory_blocks`: It checks a value, and if it is non-zero, it repeatedly calls another routine while decrementing a counter by a large constant amount.
- `decrement_byte_count`: Decrements a value in a register by one megabyte and calls itself recursively until the value reaches zero.
- `decrement_byte_count_loop`: Decrements a value in a register by one megabyte and calls itself if the value remains above a certain threshold.
- `build_huffman_decode_table_2`: Builds a Huffman decode lookup table from an array of code lengths by counting each length, deriving the first code for every length, and filling table entries.
- `run_handler_pointer_list`: Walks a null-terminated table of handler pointers and calls the handler at each step while the flag allows it.
- `dispatch_handler_table_entries`: Calls the handler at each entry of a null-terminated pointer table, guarding repeat calls with a flag, and sets a completed flag at the end.
- `call_indirect_handler_return_constant_2`: Calls the handler held in a global pointer with the address of a global variable, then returns the constant 20.
- `invoke_global_callback_2`: Calls the function pointer held in a global table with the address of a global variable as its argument.
- `compare_case_insensitive_strings`: Compares two zero-terminated strings character by character after converting lowercase letters to uppercase, and returns the difference between the first mismatching characters, or zero when they are equal.
- `parse_signed_integer`: Parses an optional plus or minus sign followed by decimal digits from a string, storing the signed value in the integer the second argument points to.
- `lowercase_string_in_place_2`: Walks a null-terminated string and adds 32 to each byte whose class-table flag marks it as an uppercase letter, converting it in place.
- `lowercase_string_via_pointer_2`: Converts uppercase letters to lowercase in a null-terminated string addressed by a pointer register, using the same class-table test, with no return value.
- `lowercase_string_after_seed_byte`: Writes a seed byte to the destination, then copies the characters that follow in the same string, lowercasing uppercase letters, until the terminator is reached.
- `uppercase_string_in_place_2`: Walks a null-terminated string and subtracts 32 from each byte whose class-table flag marks it as a lowercase letter, converting it in place to uppercase.
- `uppercase_string_via_pointer_2`: Converts lowercase letters to uppercase in a null-terminated string addressed by a pointer register, using the same class-table test, with no return value.
- `uppercase_string_after_seed_byte`: Writes a seed byte to the destination, then copies the following characters from the same string, uppercasing lowercase letters, until the terminator.
- `free_pending_list_nodes`: Pops each entry off the pending list, updating the current-entry field, and frees it through the exec library.
- `drain_pending_list_entries`: Pops and frees entries from the pending list in a do-while loop, updating the current-entry and status fields and calling an exec routine for the last one when a size threshold is met.
- `run_exec_calls_saved_arg`: Stores its argument in a stack slot, runs an optional callback and a helper, then calls several exec library routines in one of two branches before returning the stored value.
- `parse_command_line_into_argv`: Splits a command line into words, treating double-quoted spans as single words and an asterisk followed by E or N as an escape or newline character.
- `free_argv_array_if_set`: Frees the argument array through the exec library when its count and pointer are both nonzero, and returns the count otherwise.
- `compare_strings_up_to_length`: Compares two byte strings for at most the given number of characters, returning zero when they match.
- `map_condition_flags_to_sign`: Reads the processor condition flags left by a previous comparison and returns minus one when the result is less or equal, otherwise one.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `append_string_to_buffer_2`: Finds the terminating zero of the destination string and copies the source string, including its terminator, onto the end.
- `copy_string_limited_padded`: Copies at most the given number of bytes from the source to the destination, stopping after the source terminator and padding the rest of the count with zero bytes.
- `copy_string_and_zero_pad`: Copies a zero-terminated source into the destination and then clears any remaining count bytes with zeros, using register arguments and returning nothing.
- `zero_fill_counted_block`: Writes zero bytes starting at the destination address for the count in the data register plus one.
- `test_char_class_then_call`: Returns zero for a null argument.
- `run_callback_list_then_helper_2`: Walks the registered callback list and calls each entry, then passes its argument to a helper and returns nothing.
- `run_callback_list_then_finish`: Walks the registered callback list and calls each entry, then calls a helper with no arguments.
- `drain_pending_list_until_limit`: Saves the caller's arguments in a stack frame, then pops entries from the pending list and frees them through the exec library until the list is empty or reaches the saved stack position.
- `call_helper_and_return_2`: Calls a helper with no arguments and returns, doing nothing else.
