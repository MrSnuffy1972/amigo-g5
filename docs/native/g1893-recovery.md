# Game 1893 (gloom - special edition 97 (1997)(black magic)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01893). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1244 raw-word lines |
| Functions named and described | pass, 369 of 369 named (0 author symbols), 369 of 369 described (9 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6390 of 6394 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1186 |
| code | 18140 |
| data | 2022073 |
| padding | 7113 |
| unused | 654848 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| LZX | 2 | 3153 | 181 |
| Arp.library | 3 | 88 | 7 |
| Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[HD].adf:LZX | 2 | 3153 | 181 |

## Cases handled

- gloom - special edition 97 (1997)(black magic)(disk 1 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[HD].adf:GloomSE1.lzx: high-entropy data file (7.96 bits/byte), classified as packed data
- gloom - special edition 97 (1997)(black magic)(disk 2 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)(Disk 2 of 3)[HD].adf:GloomSE2.lzx: high-entropy data file (7.97 bits/byte), classified as packed data
- gloom - special edition 97 (1997)(black magic)(disk 3 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)(Disk 3 of 3)[HD].adf:GloomSE3.lzx: high-entropy data file (7.99 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### LZX

- `start_program_entry_point`: Program entry point that allocates a working copy of the global data block, relocates it, and rebuilds the command-line arguments inside it.
- `clean_up_process_resources`: Runs an optional cleanup hook, then either releases a stored memory block or unlocks, closes and replies to the outstanding resources.
- `invoke_global_callback_2`: Calls a function pointer stored in a global table with the address of a global variable as its argument, then returns nothing.
- `call_indirect_handler_return_constant_2`: Calls a handler through a function pointer held in a global variable, passing the address of a global variable, then returns twenty.
- `parse_command_line_arguments`: Splits a command line into arguments, honouring quoted strings and asterisk escapes for escape and newline characters.
- `open_named_input_stream`: Initialises a name buffer and opens the named input through a routine.
- `set_input_ready_flag`: Sets the input-ready flag in the state block and returns zero.
- `transfer_block_through_buffer`: Passes a block through a buffer routine and adds the byte count to a running total on a full transfer.
- `transfer_count_through_buffer`: Transfers the count held in a data register through the buffer routine, adds it to a running total when the whole count moves, and returns two on a short transfer.
- `add_count_to_running_total`: Adds the count held in a data register to a running total in the state block and returns zero.
- `free_linked_node_list`: Walks a linked list, releasing each node through a memory routine with a fixed size of 118 bytes, then clears the head and tail pointers and returns zero.
- `clear_linked_list_head_tail`: Sets the head and tail pointers of a linked list to zero without freeing its nodes.
- `pack_flag_bits_into_byte`: Combines eight state flags into one byte, setting each bit whose flag is nonzero, and stores the result through the pointer.
- `parse_command_line_arguments_2`: Iterates through a list of command-line arguments and updates a state structure based on specific character flags.
- `fetch_next_command_option`: Scans the command-line argument array for the next option, recording the option letter and the first argument it skips past.
- `scan_option_argument_entries`: Advances through an argument list and returns a pointer to a result string or a null pointer.
- `next_positional_argument`: Returns the next command-line argument that is not an option, treating a double-dash marker as the end of options, or zero when no arguments remain.
- `parse_date_string_fields`: Parses a date written as year, month and day separated by slashes.
- `store_prefix_and_parse_date`: Stores a given prefix byte in a date record, then parses a month of 1 to 12 and a day of 1 to 31 from a slash-separated string.
- `parse_month_and_day_fields`: Parses a month of 1 to 12 and a day of 1 to 31 from a slash-separated string, storing them in a date record.
- `store_month_and_parse_day`: Reads a month value through a routine and stores it, then parses a day of 1 to 31 from a slash-separated string and stores it.
- `walk_path_components_recursively`: Skips through a slash-separated path string one component at a time and recurses on the remainder.
- `check_path_after_slash`: Skips one optional leading slash, then recurses on the rest of the string and stores the returned byte in a state field.
- `check_recursive_result_range`: Calls itself without arguments, stores the byte it returns in a state field, and reports failure when that byte is zero or above 31.
- `return_zero_constant_2`: The routine is empty and simply returns a zero value to the caller.
- `initialise_state_and_buffer_size`: Sets a block of state flags and counters to fixed default values and copies a global string into a state buffer.
- `handle_pause_state_update`: While the pause flag is set it changes the task priority through exec library calls, and it then updates the paused and step flags from the stored counter.
- `scan_characters_using_class_table`: Walks a byte string, testing each character against a class table, then calls a routine and returns the value it left in a local variable.
- `run_registered_handler_list`: Walks a linked list of registered callbacks and invokes the function pointer stored in each node.
- `run_state_update_sequence`: Runs a fixed sequence of setup calls, calling a routine with the value 100 only when a mode byte is below two.
- `setup_task_and_library_mode`: Records the current task and opens the arp library when a stored size field is small, otherwise it sets a flag in the owning structure.
- `forward_call_without_arguments`: Calls another routine with no arguments and returns.
- `save_and_clear_pointer_field`: Copies a field of a linked structure into a state slot, overwrites that field with an all-ones value, and sets a flag byte to one.
- `format_temp_file_name_entry`: Passes the stored base name and a per-entry record address to a formatter that builds a temporary LZX compression file name.
- `clear_and_resolve_named_handle`: Clears a 144-byte state buffer, then compares the requested name with three known names and copies the name when none match.
- `resolve_resource_name_handle`: Compares a name with two known names and copies it when neither matches.
- `open_named_resource_handle`: Compares a name with two known names and copies it when neither matches.
- `open_resource_after_name_check`: Compares a name with one known name and copies it when there is no match.
- `resolve_resource_handle`: Attempts to resolve a resource handle by calling functions through a table and returns a success or failure status.
- `store_handle_or_release`: Stores a nonzero result from a library call through a pointer.
- `write_value_through_pointer`: Writes its argument value through a pointer register and returns zero.
- `try_open_file_handle`: Opens a file through the dos library and stores the returned handle when the call succeeds.
- `store_long_through_pointer`: Writes a 32-bit value from a data register through an address register pointer and returns zero.
- `close_file_and_zero_block`: Closes the file handle held in the structure through the dos library when the handle is non-zero, otherwise repeats the check on the global state block.
- `close_file_zero_block_direct`: Closes the file handle through the dos library's Close entry, then clears 144 bytes starting at the structure.
- `read_dos_file`: Calls a function through a table to read data if a provided size is greater than zero.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `forward_dos_write`: Calls the dos library's Write entry through the library base, passing the caller's arguments on, and returns nothing.
- `store_standard_handles`: Stores the result of the dos Input call in the state block.
- `call_dos_792_if_clear`: Calls a dos library entry at offset -792 only when a state byte is zero.
- `cleanup_after_user_abort`: Calls two helper routines in turn, the second one with the global state block.
- `apply_mode_five_set_flag`: When a state byte is zero, calls a helper with the global block and the value 5, then sets a companion flag byte to one.
- `apply_mode_four_clear_flag`: When a state byte is zero, calls a helper with the global block and the value 4, then sets a companion flag byte to zero.
- `write_if_enabled`: Calls a function through a table to perform a write operation if specific conditions in a state block are met.
- `write_output_in_blocks`: Writes the requested count to the output handle in pieces of at most 1024 bytes.
- `write_data_in_blocks`: Iterates through a data length, calling a write function in chunks of up to 1024 bytes.
- `write_chunks_until_exhausted`: Writes data in pieces of at most 1024 bytes until no bytes remain, returning zero on completion.
- `format_string_and_write`: When output is enabled and the output handle is set, formats a line into a buffer in the state block, measures its length by scanning to the terminator, then writes it with the dos Write entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_dos_426_if_clear`: Calls the dos library entry at offset -426 only when a state byte is zero.
- `call_dos_426_copy`: Calls the dos library entry at offset -426 only when a state byte is zero.
- `read_byte_with_state_wrappers`: Reads one value through the dos Read entry, with helper calls before and after it.
- `read_char_matching_set`: Reads characters through a helper, folds lowercase to uppercase using a character class table in the state block, and echoes each one through a formatting helper.
- `scan_input_for_match`: Reads characters through a helper, folds lowercase to uppercase using the class table, and checks each against the characters of a buffer.
- `scan_input_after_step`: Advances the buffer pointer, then reads characters through a helper, folding lowercase to uppercase with the class table and echoing each one through a formatting helper.
- `stub_empty_return`: Does nothing and returns immediately; it is an empty entry reached from one unnamed routine.
- `run_three_step_cleanup`: Calls the global-state helper, then a second helper, then the global-state helper again, and returns nothing.
- `open_library_compute_unit_size`: Opens a library by name through the exec base, reads a window measurement between a Forbid and Permit pair, and divides it by a per-unit size from the structure.
- `check_string_against_stacked_set`: Copies an 8-byte table from the global block onto the stack, then returns 1 as soon as a character of the string matches one of its entries, or 0 at the terminator.
- `check_string_cyclic_stacked_set`: Walks the string testing each character against the eight stacked entries by index, returning 1 on a match and 0 at the terminator.
- `check_string_stacked_set_ahead`: Advances to the next character before each test against the eight stacked entries; it returns 1 on a match and 0 when the terminator is reached.
- `stub_return_zero`: Returns zero immediately, with no other effects.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `release_memory_block`: Calls the exec FreeMem entry through the library base with the caller's arguments and returns nothing.
- `try_allocate_halving_sizes`: Repeatedly halves a stored count and asks the allocation helper for memory at each size, returning zero on the first success.
- `retry_allocation_halving_size`: Halves a size count and calls the allocation helper at each size, returning zero on the first success.
- `forward_pair_to_helper`: Passes its two arguments unchanged to a helper and returns nothing.
- `consume_bits_refill_word`: Discards a number of bits from the bit buffer and, when the bit counter runs out, refills the buffer with a 16-bit word from the input stream.
- `read_masked_bits_then_consume`: Takes the current bit buffer value, consumes the requested number of bits, and returns the low bits selected by a mask of that width.
- `decode_symbol_eight_bit_table`: Decodes one symbol using an eight-bit lookup table, walking a binary tree bit by bit when the entry is a link.
- `decode_symbol_six_bit_table`: Decodes one symbol using a six-bit lookup table, walking a binary tree bit by bit when the entry is a link.
- `refill_bit_buffer_words`: Reads two 16-bit words from the input stream, advances the stream pointer, loads them into the bit buffer, and sets the bit count to sixteen.
- `fill_256_decoded_values`: Calls a decoding helper 256 times and stores each returned value in turn into a 256-byte destination buffer.
- `read_bytes_and_sum_2`: Reads values through a helper into the destination while the counter is not negative, and returns the running sum truncated to 16 bits.
- `read_lzx_block_header`: Reads an LZX block header from the bit stream, including table sizes and fixed fields, then allocates the decoding tables through the allocator helper.
- `count_newline_characters`: Walks a zero-terminated string and returns how many newline characters it contains.
- `call_helper_with_flag_one`: Calls a helper with the value one and returns nothing.
- `write_wrapped_lines`: Walks a text buffer line by line, writing each line through a helper and wrapping long lines to a width computed from a stored column value.
- `combine_four_values_base16`: Combines four bytes read from a pointer into one 16-bit number by multiplying the running total by sixteen before each addition.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `init_once_with_zero_arg`: On the first call sets a state byte to one and calls a helper with zero; later calls see the byte set and do nothing.
- `compare_path_leaf_names`: Strips the directory part from two path strings by finding their last slash or colon, then compares the remaining names through a helper.
- `test_path_leaf_with_helper`: Finds the final separator in a path and passes the name after it to a helper.
- `is_helper_result_zero`: Calls a helper and returns true only when its result is zero.
- `return_zero_constant`: Returns the constant zero; three unnamed routines reach it through data pointers and calls.
- `test_path_ends_with_slash`: Returns one when the last character of a string is a forward slash, and zero otherwise.
- `return_zero_gap_fill`: Returns zero.
- `normalize_path_separators_2`: Replaces every backslash in a string with a forward slash, in place, up to the terminator.
- `stub_empty_return_two`: Does nothing and returns, and one unnamed routine calls it.
- `split_amiga_path_parts`: Splits an Amiga path into its device, directory and file parts at the last slash and colon, copying each piece into an optional output buffer.
- `copy_path_tail_parts`: Copies the portion of a path after the last separator into an output buffer, or the whole path when no separator exists.
- `copy_path_remainder_parts`: Copies the remainder of a path after a separator into an output buffer, then writes terminators into the directory and device buffers at the separator position.
- `terminate_path_pieces`: Writes terminators into output buffers at positions computed from the separator index, and writes a device terminator when a colon follows the directory.
- `terminate_device_directory_parts`: Writes a terminator into the directory buffer after the separator, and into the device buffer when a colon is found at the separator position.
- `find_matching_list_node`: Walks a linked list and returns the first node whose name matches the given name under mode flags.
- `find_list_node_flag_test`: Walks a linked list of named nodes and returns the first node that matches, using a mode flag to choose comparison on normalized names held in stack buffers.
- `find_list_node_by_name`: Walks the linked list and returns the first node whose name satisfies a comparison helper.
- `find_next_list_match`: Walks forward through a linked list from the given node and returns the first node whose name check passes.
- `find_list_node_library_check`: Makes two library calls at fixed negative offsets before walking the list for the first node that passes the name comparison.
- `find_list_node_or_zero`: Walks the linked list from the given node and returns the first node that passes the library-backed or name-based check.
- `return_zero_list_stub`: Returns zero; three unnamed routines reach it through data pointers and calls.
- `append_named_list_node`: Appends a new node holding a copy of the name to a linked list, rejecting duplicates when a flag is set by checking each existing name first.
- `append_named_node_skip_dup`: Allocates a node sized for the name, copies the name into it, and appends it to the list without a duplicate check.
- `allocate_and_append_named_node`: Allocates a node sized for the name, clears its header, copies the name into it, sets its flag, and appends it to the list tail.
- `initialize_and_append_node`: Clears a preallocated node header, copies the name into it, sets its flag, and links it at the list tail.
- `link_node_after_tail_checked`: Links a node after the list's tail pointer and sets the status to zero.
- `link_node_to_tail_unchecked`: Links a node after the list's tail pointer and sets the status to zero, without checking that a tail exists.
- `open_output_or_report_error`: Calls an open helper for the output file named in the state block.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `close_output_if_open`: Closes the output file held in the state block when its handle field is set.
- `allocate_multiple_buffers_2`: Attempts to allocate several memory blocks of varying sizes using a library call and stores the resulting pointers in a state block.
- `allocate_work_buffers_2`: Attempts to allocate several memory blocks of varying sizes and stores the resulting pointers in a state block.
- `allocate_work_buffers`: Attempts to allocate several buffers of varying sizes through a memory allocation routine and updates a state block.
- `allocate_buffers_from_size_field`: Allocates the work-buffer sequence from the buffer sized by a state field onward, returning one on any failed allocation.
- `allocate_buffers_from_first_small`: Allocates the work-buffer sequence from the first 32768-byte buffer onward, then sets a state flag on success.
- `allocate_buffers_from_second_small`: Allocates the final buffers from the second 32768-byte buffer onward, then sets a state flag on success.
- `allocate_buffers_from_4k`: Allocates the 4096-byte work buffer and the 768-entry table, then sets two output pointers and a state flag on success.
- `allocate_table_and_set_pointers`: Allocates the 768-entry table and, on success, sets two pointers and a state flag.
- `set_buffer_pointers_after_allocation`: Sets two pointers to the first two allocated buffers, each offset by 65536 bytes, sets a state flag, and returns zero.
- `release_all_work_buffers`: Frees each work buffer whose pointer is set, using the matching size, clears its pointer, and clears the state flag.
- `release_secondary_buffer`: Frees a buffer whose size is held in a state field, then clears its pointer when it was set.
- `flush_pending_output`: When the write position is behind the flush position, writes the pending bytes through a helper and reports error code 7 on failure.
- `write_bytes_with_counter`: Writes a block of bytes through a buffered helper and adds the count to a running total on success.
- `flush_buffer_once`: Calls a helper with the buffer held in the state block and the value one, and returns nothing.
- `allocate_table_block`: Allocates a block of 7728 bytes and, on success, sets two pointers offset by 1548 and 4638 bytes.
- `set_table_pointers_from_block`: Sets two pointers into an already allocated 7728-byte block, using offsets of 1548 and 4638 bytes, and returns zero.
- `release_table_block`: Frees the 7728-byte block when its pointer is set, then clears the pointer.
- `fill_code_length_table`: Once per state, fills the first 512 bytes of a table in the state block with increasing code values written in runs whose lengths double.
- `fill_remaining_table_upper_half`: Continues the table fill with doubling runs of code values, each run one higher than the last, and stops when the index passes 511.
- `fill_table_alternating_runs`: Fills the table with doubling runs that alternate between a base code and the code one higher, stopping once 512 entries are written.
- `allocate_four_pointer_table`: Allocates a table sized from a count, zeroes it, and stores four pointers into it at offsets derived from that count.
- `zero_four_pointer_table`: Zeroes a table sized from a count and stores four pointers into it at offsets derived from that count, then returns zero.
- `fill_region_and_set_pointers`: Fills a region with a constant byte, then stores four pointers into the table at offsets derived from a count, and returns zero.
- `free_four_pointer_table`: Frees the table sized from a count when its first pointer is set, then clears all four pointers.
- `format_text_into_buffer`: Calls the exec RawDoFmt routine through the library base with the caller's arguments and returns nothing.
- `format_text_stream_variant`: Calls the exec RawDoFmt routine with the caller's arguments and a character-copy routine as its output writer, then returns nothing.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `update_checksum_block`: Updates a running 32-bit checksum over a block of bytes using a 256-entry lookup table in the state block, keeps the running value between calls, and returns its complement.
- `validate_memory_checksum_blocks`: Iterates through a memory range by calling a checksum routine in increments of one megabyte.
- `process_checksum_data_block`: Iterates through a memory range by repeatedly calling a checksum update routine in increments of one megabyte.
- `update_checksum_block_2`: Iterates through a memory range to update a running 32-bit checksum using a lookup table.
- `build_canonical_decode_table`: Counts how often each code length occurs, accumulates the starting code for each length, and fills the decoding table for the symbols.
- `run_handler_list_until_null`: Walks a null-terminated list of handler pointers stored in the state block, calls each handler, and keeps the last result.
- `run_handler_list_do_while`: Calls handlers from a null-terminated pointer list, keeping the last result, and sets a state flag once the list has been walked.
- `call_indirect_handler_return_constant_3`: Calls a handler through a pointer held in a global variable, passing the address of a global variable, and returns the constant 20.
- `invoke_global_callback_3`: Calls a function pointer held in a global variable, passing the address of another global variable, and returns nothing.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time, converting lowercase letters to uppercase first.
- `parse_signed_decimal_integer`: Reads an optional plus or minus sign and then decimal digits from a string, stores the signed value through a pointer, and returns the offset of the first character after the digits.
- `lowercase_string_in_place`: Converts each uppercase letter of a zero-terminated string to lowercase in place, using the character class table in the state block, and returns the same pointer.
- `lowercase_string_loop`: Lowercases a zero-terminated string in place using the character class table and stops at the terminator; it returns nothing.
- `write_seed_and_lowercase_copy`: Writes a seed character into a string, then copies each following character, converting uppercase letters to lowercase, until the terminator.
- `uppercase_string_in_place`: Converts each lowercase letter of a zero-terminated string to uppercase in place, using the character class table, and returns the same pointer.
- `uppercase_string_loop`: Uppercases each letter of a zero-terminated string in place using the character class table, stopping at the terminator.
- `write_seed_and_uppercase_copy`: Writes a seed character into a string, then copies each following character, converting lowercase letters to uppercase, until the terminator.
- `free_linked_list_nodes`: Iterates through a linked list, freeing each node and updating pointers until the list is empty.
- `free_node_list_do_while`: Frees each node of a linked list in the state block through the exec FreeMem routine, with the same extra exec call when the library version is above 35.
- `run_frame_cleanup_sequence`: Saves the given value in a stack frame, calls a stored callback when one is set, then runs several exec library calls, including a library close and a memory free.
- `split_command_line_arguments`: Splits a command line into words, where a quoted string counts as one word and an asterisk escapes the next character: E gives an escape character and N a newline.
- `free_argument_array`: Frees the argument pointer array through the exec memory release routine when it was allocated, and returns the argument count.
- `compare_bounded_strings`: Compares up to a set number of bytes of two strings and returns minus one when the first differing byte is smaller or the strings end, one when it is larger, and zero when the compared bytes are equal.
- `derive_comparison_sign`: Converts the processor flags from a preceding comparison into minus one when the first value is less than or equal to the second, and one otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `append_string_to_buffer_2`: Finds the terminator of the destination string, copies the source string after it, and returns the destination pointer.
- `copy_bounded_string_padded`: Copies up to a given count of bytes from a source string into a destination, stopping at the source terminator and zero-filling the rest, then returns the destination pointer.
- `copy_bounded_string_loop`: Copies a source string into a destination of fixed length, stopping at the source terminator and zero-filling the remainder.
- `zero_fill_buffer`: Writes zero bytes across a region, one more byte than the given count.
- `test_first_char_class_bit`: Tests the first character of a string against a class-flag bit in the state block table and calls a helper when that bit is clear.
- `run_handler_records_with_value`: Walks a linked list of handler records in the state block, calls the handler stored in each record, then calls a helper with the given value.
- `run_handler_records_then_finish`: Walks the same handler record list, calling each handler, and then calls a helper without an argument.
- `restore_saved_context_frame`: Restores a saved register set onto a task stack.
- `forward_call_only`: Calls one helper and returns, with no other work.

### Arp.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `init_arp_pointer_list`: Part of Arp.library: stores a record reference, fills a pointer array through repeated library lookups, and on success links the record and runs a library finishing call.
- `fill_arp_pointer_list_loop`: Part of Arp.library: continues filling the pointer array from the lookup results, then links the record and runs the finishing sequence, leaving through the error exit when a lookup fails.
- `finish_arp_record_setup`: Part of Arp.library: stores a record pointer, calls several library routines including an allocation, keeps the allocated pointer and a flag in the record, and completes with a further library call.
- `repeat_arp_call_until_flag`: Part of Arp.library: repeats a library call together with a helper until the call's flag clears, then increments a counter field in the library base.
- `clear_arp_field_and_call`: Part of Arp.library: clears a field of the given structure and calls a library routine through the library base.
- `allocate_arp_size_block`: Part of Arp.library: when a size is non-zero, allocates memory through the exec allocator and writes the size plus four into the first word of the block.

### Gloom - Special Edition 97 (1997)(Black Magic)(Disk 1 of 3)[HD].adf:LZX

- `start_lzx_program`: Program entry point for the LZX program: it allocates a working block, copies in its initial data with pointer adjustments, and takes the current directory and start message.
- `finish_task_and_reply`: Stores the return value on the task stack, calls an optional callback, and unlocks a directory lock or replies to the start message depending on a flag.
- `invoke_secondary_callback`: Calls a function pointer held in a global variable, passing the address of a global variable, and returns nothing.
- `call_secondary_handler_constant`: Calls a function pointer held in a global variable with the address of a global variable as argument, and returns the constant 20.
- `split_command_line_words`: Repeats the command-line word splitting used at program start: it counts the words, allocates a pointer array, and fills it, honoring quotes and asterisk escapes.
- `lookup_or_set_state_flag`: Calls a lookup helper on a state buffer.
- `set_state_flag_return_zero`: Sets a state flag byte to one and returns zero.
- `write_block_counted_total`: Writes a block of the given length through a helper and adds the length to a running total on success.
- `flush_counted_write`: Calls a helper with the state buffer and the length in a data register, adds that length to the running total when the helper returns the same number, and returns 2 otherwise.
- `add_count_to_total`: Adds the value in a data register to the running total in the state block and returns zero.
- `free_list_nodes_and_clear`: Walks a linked list held in the state block, releasing each node through a helper with a size argument of 118, then clears the head and tail pointers and returns zero.
- `clear_list_head_tail`: Clears the head and tail pointers of a list held in the state block.
- `pack_option_flags_byte`: Packs eight state flags into the bits of one byte, writing one bit per flag into the byte at the given pointer.
- `parse_command_line_arguments_3`: Iterates through a list of command-line arguments and updates various state flags and configuration parameters based on the character value of each argument.
- `fetch_next_option_argument`: Scans the argument vector for the next option and returns a pointer to the text after its letter, or zero when no arguments remain.
- `fetch_next_option_copy`: A second copy of the argument scanner, returning the option text after its letter and storing the letter in a byte.
- `fetch_next_file_argument`: Returns the next argument string from the vector, skipping dash-marked entries unless a flag allows them, and returns zero when the list is exhausted.
- `parse_date_year_month_day`: Parses a date written as year, month and day separated by slashes.
- `parse_date_fields_with_base`: Stores a leading base byte in a record, then parses the month and day of a date string, requiring the month from 1 to 12 and the day from 1 to 31.
- `parse_month_day_pair`: Parses a month and a day separated by a slash, requiring the month from 1 to 12 and the day from 1 to 31, and stores both in a record.
- `parse_month_then_day`: Parses the month value into a record byte, skips the separator, then parses the day from 1 to 31 into the next byte.
- `parse_day_after_slash`: Skips to the character after the next slash, parses a day value from 1 to 31 into a record byte, and returns one when the value is invalid.
- `parse_day_field`: Skips an optional leading slash, parses a day value from 1 to 31 into a record byte, and returns one when the value is invalid.
- `store_and_check_day`: Stores a parsed value in the record and returns one unless the value is between 1 and 31.
- `return_zero_gap_stub`: Returns zero; five unnamed routines call it.
- `initialize_default_settings`: Sets the default option flags and buffer size values in the state block, copies a constant name string into a state field, and selects a size from a mode byte.
- `handle_pause_and_priority`: Adjusts the task priority through the exec library when a flag is set, and updates pause-related flags.
- `test_leading_char_class_bit`: Skips characters of a string whose class-flag bit is set, then calls a helper and returns the value it leaves.
- `run_handler_records_and_forward`: Walks the handler record list in the state block, calling each record's handler, and then calls a helper with the given value.
- `setup_callbacks_and_run`: Clears three state fields through helpers, installs a global callback pointer, and runs a helper with the two parameters.
- `detect_arp_and_mode`: Reads the exec library version: below 37 it opens the Arp library and sets a flag, otherwise it sets a bit in the task flags.
- `forward_call_again`: Calls one helper and returns with no other work.
- `save_and_clear_pointer_field_2`: Saves a pointer field of an object in the state block, sets that field to minus one, and sets a flag byte.
- `format_temp_file_name`: Builds a temporary file name by formatting the base name with a temporary-file suffix and a numeric index scaled by 92 bytes, then calls a helper with the result.
- `map_open_mode_name_code`: Compares a file open-mode name with three constant names and returns one of three numeric codes, 1005, 1004 or 1006, for the match found.
- `open_file_by_mode_name`: Maps an open-mode string to a numeric code.
- `open_file_mode_or_lock`: Maps an open-mode string to one of two numeric codes, then opens the file through the dos library, locking first for one of the codes, and stores the handle on success.
- `open_file_by_mode_short`: Checks the open-mode string against two constant names and otherwise reports an error through a helper.
- `report_unknown_open_mode`: Reports an unrecognized file open mode by calling a helper with the message and error code 40.
- `store_handle_or_unlock`: Calls a library routine through the dos base and stores a non-zero result through the output pointer.
- `store_value_return_zero`: Stores the value in a register through a pointer and returns zero.
- `open_file_handle_store`: Calls the dos library's Open entry through the library base, stores a non-zero handle through the output pointer, and returns true when the result is zero.
- `store_data_register_via_pointer`: Stores a data register value through an address register pointer and returns zero.
- `close_file_and_clear_buffer`: Closes a file through the DOS library and then zeroes 144 bytes of memory starting at the provided pointer.
- `close_file_and_clear_buffer_2`: Closes a file through the DOS library and then zeroes 144 bytes starting at the pointer held in an address register.
- `execute_conditional_function_pointer`: Checks if a third parameter is greater than zero.
- `forward_dos_read`: Calls the dos library's Read entry through the library base with the caller's arguments and returns nothing.
- `forward_write_to_dos`: Calls the DOS library's Write entry through the library base with the arguments it was given.
- `cache_standard_input_output_handles`: Stores the standard input handle in a context record and stores either the standard output handle or zero based on a suppression flag.
- `call_dos_message`: Calls a DOS library function if a specific flag in a global structure is zero.
- `execute_two_routines`: Calls two different routines sequentially.
- `set_state_flag_on`: Checks a byte in a state block and, if it is zero, calls a function through a table with a value of five and sets another byte in the block to one.
- `set_state_flag_off`: Checks a byte in a state block and, if it is zero, calls a function through a table with a value of four and sets another byte in the block to zero.
- `conditionally_write_to_dos`: Calls the DOS library's Write entry if a suppression flag is zero and an output handle is non-zero.
- `write_data_in_chunks_3`: Iterates through a data length, calling a function through a table to write data in chunks of up to 1024 bytes until the length is exhausted.
- `write_data_in_chunks`: Writes data in chunks of at most 1024 bytes using the DOS library until the amount of data is exhausted or an error occurs.
- `write_data_in_chunks_2`: Repeatedly calls a DOS write function to output data in blocks of at most 1024 bytes until a specified count is reached or an error occurs.
- `format_and_write_message`: When output is enabled and an output handle is set, formats a text line into a context buffer using a helper routine, then writes it through DOS Write with the length measured from its terminator.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `call_dos_if_flag_clear`: Checks a flag byte in the global data block and, only when it is zero, calls a DOS library function through the library base.
- `call_dos_if_flag_clear_2`: Behaves like its neighbour: when the flag byte in the global data block is zero it calls the DOS library function, and otherwise it returns without action.
- `read_block_through_dos`: Calls a library routine through a function table entry, the dos read call, with two helper calls before and after it when a flag byte is set.
- `read_key_matching_string`: Reads a key through a helper, echoes it through a formatting call with the referenced format text, and folds it to upper case when the character class table flags it.
- `read_key_until_string_match`: Reads a key through a helper while the string pointer sits on a zero byte, echoes it, folds its case by the class table, and advances through the string until the current byte matches the key.
- `advance_string_until_key_match`: Steps the string pointer forward, reads a key through the helper whenever the current byte is zero, echoes it, folds its case, and repeats until the key equals the current string byte.
- `return_without_action`: Returns straight away with no body and no library call.
- `bracket_step_with_global_helper`: Calls a helper with the global variable, then a parameterless helper, then the global helper again.
- `compute_visible_row_count`: Opens a library, finds the current task and reads a window pointer from it under a multitasking lock, then divides the space below the window's top by a font line height.
- `check_string_for_special_chars`: Copies eight bytes from a global table into a local buffer, then scans the argument string and returns 1 if any of its characters appear in that set, or 0 at the terminator.
- `search_char_set_resume`: Resumes a scan of the current string from a saved index, comparing each byte with the eight-byte set the caller passed on the stack.
- `search_string_for_char_set`: Scans the string from its start, comparing each byte with the eight-byte set passed on the stack.
- `return_zero_stub`: Returns zero immediately with no other work.
- `call_exec_alloc_mem`: Calls the exec library's memory allocation entry through the library base and returns nothing.
- `call_exec_free_mem`: Calls the exec library's memory release entry through the library base and returns nothing.
- `allocate_memory_halving_size`: Tries to allocate a block of the requested size and stores the pointer on success, returning 0.
- `retry_allocation_with_smaller_size`: Halves the stored size and attempts an allocation at each step, returning 0 on success.
- `forward_two_arguments`: Passes its two arguments straight on to another routine and returns nothing.
- `consume_bits_refill_buffer`: Removes the given number of bits from a bit buffer by shifting it right.
- `read_bits_from_buffer`: Captures the current bit buffer, consumes the requested number of bits through the bit reader, and returns the low bits of the captured value.
- `decode_eight_bit_huffman_symbol`: Looks up a symbol in a table indexed by the low eight bits of the bit buffer.
- `decode_six_bit_huffman_symbol`: Uses a six-bit lookup table, walking a binary tree for longer codes, then consumes the code length from a byte table and returns the symbol.
- `reset_bit_buffer_from_input`: Loads two 16-bit words from the input stream into the bit buffer, advances the input pointer past them, and sets the bit count to sixteen.
- `copy_256_bytes_from_reader`: Reads 256 bytes one at a time through a reader helper and stores them consecutively at the destination pointer.
- `read_bytes_and_sum`: Reads a counted run of bytes through a helper, storing each one at the destination and accumulating a 16-bit total.
- `read_lzx_header_and_tables`: Reads the LZX header fields from the bit stream, skipping a stored count, and allocates the decoding tables sized from the fields it reads.
- `count_newlines_in_string`: Counts the newline characters in the zero-terminated text passed in and returns the total.
- `exit_with_error_code_one`: Calls a helper with the value one and returns nothing.
- `print_paged_text_lines`: Prints text one line at a time, breaking it into pages of lines and calling a helper between pages.
- `combine_four_byte_digits`: Folds four bytes into one 16-bit value by multiplying the running total by sixteen before adding each byte, so each byte acts as a base-16 digit.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `run_once_then_set_flag`: Checks a one-time flag in the state block.
- `compare_path_tails`: Finds the final portion of each path after a slash or colon separator, passes the two portions to a comparison helper, and returns 1 when that helper reports a match.
- `check_path_final_component`: Trims the given path back to its last slash or colon separator and passes the result to a helper.
- `test_helper_result_zero`: Calls a helper with no arguments and returns true when its result is zero.
- `return_zero_success`: Returns zero with no other work.
- `check_path_ends_with_slash`: Returns 1 when the zero-terminated path ends with a forward slash, otherwise 0.
- `return_zero_without_effect`: Returns zero immediately, with no other work.
- `normalize_path_separators`: Walks the zero-terminated path in place and replaces every backslash with a forward slash.
- `return_nothing_from_hook`: Returns immediately without any work.
- `split_path_into_parts`: Splits a path at its last slash or colon, copying the directory and device parts into the supplied buffers when they are present.
- `copy_path_remainder`: Copies the tail of a path into a buffer, starting after the given index, or the whole string when the index is negative.
- `copy_path_tail_and_trim`: Copies the portion of a path after a given index into the buffer, then cuts the buffer at the last slash before that point and at any colon boundary.
- `trim_path_parts_at_separators`: Places terminators in the supplied directory and device buffers at the separator positions found by an earlier scan.
- `terminate_device_part_buffer`: Writes a terminator into the output buffer that holds the device part when the path has a colon at the given position, clamping that position to the buffer length.
- `find_matching_list_entry`: Walks a linked list of named entries and returns the first entry whose name matches the given text, optionally transforming each name by flag bits before comparing.
- `find_matching_list_entry_local`: Walks a linked list of named entries from a head pointer passed in, comparing names as its neighbour does, but writes transformed names into local buffers.
- `find_first_entry_matching_name`: Walks a linked list of named entries and returns the first entry for which a name check reports success, checking for a pending interrupt signal along the way.
- `scan_entries_with_name_filter`: Applies a name check to the first list entry and then walks the following entries, skipping those that fail an optional filter and returning the first match.
- `scan_entries_after_interrupt_check`: Checks for a pending interrupt signal first, then walks a linked list of named entries and returns the first entry whose name passes the check.
- `search_entries_for_name`: Walks a linked list of named entries, skipping entries that fail an optional filter and stopping on an interrupt signal, and returns the first entry whose name check passes.
- `return_zero_flag`: Returns zero with no other work.
- `add_named_entry_optional_check`: Allocates a new list entry sized for the given name, optionally rejecting duplicates first, then copies the name in, flags the entry and appends it to the list tail.
- `add_entry_checking_duplicates`: Checks the list for an existing entry with the same name, then allocates and appends a new named entry.
- `append_named_entry`: Allocates a list entry sized for the name, copies the name into it, flags it and appends it to the list tail.
- `init_named_entry_and_append`: Initialises a caller-supplied entry block by clearing its header, copying the name in, flagging it and appending it to the list tail.
- `link_entry_checked_tail`: Links a new entry onto the tail of a list when the tail pointer is set.
- `link_entry_to_list_tail`: Appends an entry to the list tail and sets the status to 0.
- `open_output_file_or_report`: Attempts to open the output file named in the state block, and when that fails prints a 'can't open for output' message with the file name.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `close_file_handle`: Checks if a specific integer in a global structure is non-zero and, if so, calls another routine to close that file handle.
- `allocate_engine_work_buffers`: Attempts to allocate a sequence of memory blocks of varying sizes, including two large buffers and several smaller ones, to populate a state block.
- `allocate_decoder_buffers_after_table`: Allocates the same decoder buffers as its neighbour without the first table, returning 1 on any allocation failure.
- `allocate_multiple_buffers`: Attempts to allocate several memory blocks of varying sizes using a library function, storing the resulting pointers in a table and setting a flag if all allocations succeed.
- `allocate_decoder_buffers_from_table`: Allocates the decoder buffers starting with a table sized from a stored count, then two 32768-byte blocks and a 4096-byte block, and sets up the window pointers.
- `allocate_small_decoder_buffers`: Allocates two 32768-byte blocks and a 4096-byte block, then sets up the window pointers.
- `allocate_last_decoder_buffers`: Allocates one 32768-byte block and a 4096-byte block, then sets up the window pointers.
- `allocate_final_decoder_buffer`: Allocates a 4096-byte block and fills a 768-byte table through a helper, then sets up the window pointers.
- `init_decoder_table_and_window`: Initialises a 768-byte table through a helper and, when that succeeds, sets the window pointers and marks the state ready.
- `set_window_pointers_and_ready`: Sets the two window pointers from the stored buffer addresses, calls two helpers, and marks the decoder state ready.
- `free_decoder_work_buffers`: Frees each decoder work buffer that is set, passing the matching size for each one, and clears its pointer.
- `free_optional_buffer_if_set`: Frees a buffer stored in the state block when it is set, passing a size taken from a stored field, and then clears the pointer.
- `flush_pending_output_span`: When the current output position is below the stored limit, writes the pending span through a helper and reports a failure through another helper if that write fails.
- `write_block_and_count_bytes`: Writes a block of bytes through a helper unless a stored flag lets an earlier check decide the result.
- `flush_state_buffer_once`: Calls a helper with the state block's buffer and the value one, returning nothing.
- `allocate_sequence_table_block`: Allocates a block of 7728 bytes and, when it succeeds, sets three internal pointers into it at fixed offsets.
- `reset_sequence_table_pointers`: Recomputes the same three internal pointers from the stored block base and returns 0.
- `free_sequence_table_block`: Frees the 7728-byte block when it is set and clears its pointer.
- `build_slot_lookup_table_once`: Builds a 512-entry lookup table on first use, starting with four direct entries and then filling runs whose length doubles at each step, and sets a built flag so this happens only once.
- `fill_slot_lookup_remainder`: Continues filling the same 512-entry slot lookup table with runs whose length doubles at each step, writing slot numbers until the table is full.
- `fill_slot_lookup_alternate_runs`: Fills the rest of the slot lookup table in alternating runs, with the second run of each pair one step above the first, until 512 entries are written.
- `allocate_zeroed_tree_arrays`: Allocates a block sized from the count, eight bytes per entry plus two, zeroes its first eight bytes per entry, and stores four pointers into it at fixed offsets.
- `zero_and_set_tree_arrays`: Zeroes the first eight bytes per entry of a caller-supplied block and sets the same four pointers into it.
- `fill_block_and_set_pointers`: Fills a block with a given byte value for a stated length, then sets four pointers into it at fixed offsets derived from the entry count.
- `free_tree_arrays`: Frees the tree-array block when it is set, using a size derived from the entry count, then clears the block pointer and the three pointers into it.
- `format_text_via_rawdofmt`: Calls the library formatting routine with arguments the caller has set up, returning nothing.
- `format_text_from_caller_args`: Calls the library formatting routine in the same way, using arguments prepared by the caller, and returns nothing.
- `store_formatted_byte_at_pointer`: Stores one byte at the destination pointer and returns nothing.
- `update_crc32_over_buffer`: Updates a running 32-bit checksum over a block of bytes using a 256-entry table in the state block, processing groups of sixteen bytes before the remainder.
- `verify_data_integrity_blocks`: Iterates through a memory range in one megabyte increments to perform checksum updates.
- `process_checksum_data_block_2`: Iterates through a large memory range by processing it in fixed-size chunks.
- `decrement_and_check_counter`: Decrements a value in a register by one megabyte and checks if it has reached zero.
- `build_canonical_huffman_table`: Counts how often each code length occurs, computes starting code offsets, and fills the decoding table with canonical codes, falling back to a helper when the counts are invalid.
- `run_handler_table_until_null`: Walks a table of handler function pointers held in the state block, calling each one until it reaches a null entry, then sets a done flag.
- `run_handler_table_do_loop`: Walks a table of handler function pointers in the state block, calling each one when the done flag is set or no result exists yet, until a null entry is reached.
- `call_indirect_handler_return_constant`: Calls a handler through a function pointer held in a global, passing the address of a global variable, then returns the constant twenty.
- `invoke_global_callback`: Calls a handler through a function pointer held in a global, passing the address of a global variable, and returns nothing.
- `compare_case_insensitive_strings`: Compares two zero-terminated strings letter by letter, folding lowercase letters to upper case, and returns the difference at the first mismatch or zero when they are equal.
- `parse_signed_integer`: Reads an optional plus or minus sign and then decimal digits from the string, storing the signed value at the given location.
- `convert_string_to_lower_case`: Converts each uppercase letter in the zero-terminated string to lower case in place, using a character class table, and returns the original pointer.
- `lower_case_string_in_place`: Lowers the case of each uppercase letter in the zero-terminated string at the given pointer, in place, using the same character class table.
- `copy_string_lowering_case`: Writes the given first character at the destination, then copies the rest of the source string while lowering uppercase letters, stopping at the terminator.
- `upper_case_string_in_place`: Converts each lowercase letter in the zero-terminated string to upper case in place, using the character class table, and returns the original pointer.
- `make_string_upper_case`: Converts each lowercase letter in the zero-terminated string at the pointer to upper case in place, using the character class table.
- `copy_string_uppercasing_from_char`: Writes the given first character at the destination, then copies the rest of the source string while raising lowercase letters to upper case, stopping at the terminator.
- `free_pending_node_list`: Walks a list of pending nodes, releasing each node's memory, and before the final node reports through a helper when a configuration field is above 35.
- `free_pending_node_list_do`: Releases the pending node list in the same way as its neighbour, using a do-while loop that frees each node and checks the same configuration field before the last one.
- `teardown_state_and_return`: Records the given value, runs an optional cleanup callback, releases buffers and library handles held in the state block, and returns the recorded value.
- `parse_command_line_into_array`: Splits a command string into words, honouring quoted text and asterisk escapes for escape and newline characters.
- `free_argument_array_if_set`: Frees the argument array from the parser when both its pointer and its count are set, then returns the stored count.
- `compare_strings_up_to_length`: Compares up to a given number of characters of two strings as signed bytes.
- `return_sign_from_comparison`: Converts the processor flags left by an earlier comparison into a result of minus one or one.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `append_string_to_buffer`: Finds the terminator of the destination buffer and copies the source string onto its end, including the terminator.
- `copy_string_with_zero_padding`: Copies up to a given number of bytes from the source string, stopping after its terminator, and zero-fills the rest of the destination up to that count.
- `copy_string_and_pad_zeros`: Copies a zero-terminated string to the destination and then writes zero bytes for the rest of the given count.
- `zero_fill_block`: Writes zero bytes to the destination for the given count and returns nothing.
- `check_char_class_then_call`: Checks a string's characters against a flag in the character class table and then calls a helper.
- `run_shutdown_hooks_then_call`: Walks a list of registered hook records in the state block, calling the function stored in each record, then passes its argument to a helper.
- `run_registered_hooks_then_release`: Calls the function stored in each registered hook record, then makes one further helper call with no arguments.
- `release_chained_blocks_to_bound`: Releases memory blocks from a chain held in the state block, stopping when the chain reaches the bound passed in.
- `call_helper_and_return`: Calls one helper with no arguments and returns nothing.
