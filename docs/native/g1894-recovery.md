# Game 1894 (gloom - special edition 97 (1997)(black magic)[h dlm][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01894). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1244 raw-word lines |
| Functions named and described | pass, 369 of 369 named (0 author symbols), 369 of 369 described (9 by rule, 67 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6390 of 6394 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 798 |
| code | 18140 |
| data | 2019613 |
| padding | 5353 |
| unused | 659456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| lzx | 2 | 3153 | 181 |
| arp.library | 3 | 88 | 7 |
| Gloom - Special Edition 97 (1997)(Black Magic)[h DLM](Disk 1 of 3)[HD].adf:lzx | 2 | 3153 | 181 |

## Cases handled

- gloom - special edition 97 (1997)(black magic)[h dlm](disk 1 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)[h DLM](Disk 1 of 3)[HD].adf:GloomSE1.lzx: high-entropy data file (7.96 bits/byte), classified as packed data
- gloom - special edition 97 (1997)(black magic)[h dlm](disk 2 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)[h DLM](Disk 2 of 3)[HD].adf:GloomSE2.lzx: high-entropy data file (7.97 bits/byte), classified as packed data
- gloom - special edition 97 (1997)(black magic)[h dlm](disk 3 of 3)[hd]/Gloom - Special Edition 97 (1997)(Black Magic)[h DLM](Disk 3 of 3)[HD].adf:gloomSE3.lzx: high-entropy data file (7.99 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### lzx

- `initialise_lzx_decoder_and_run`: Program entry for the LZX archive tool: finds the current directory lock, allocates a memory block through Exec AllocMem, and copies a header table.
- `release_lzx_resources_and_reply`: Cleanup routine for the LZX archive tool: stores a reply code, optionally calls a hook, closes libraries and frees memory through Exec and DOS calls, then replies to the message port.
- `forward_call_via_pointer`: Forwards the global argument to a routine reached through an indirect pointer kept in the data area, then returns nothing.
- `forward_call_return_code`: Calls the same indirect pointer with the global argument and then returns the constant 20 as its result.
- `tokenise_command_line`: Splits a text line into whitespace-separated words, treating double-quoted runs as single words and star sequences as escapes for escape, newline or a literal character.
- `run_probe_and_flag_failure`: Runs an initial called routine, then a probe routine on a context sub-block and a global buffer.
- `mark_context_failed`: Sets the failure flag byte in the context block to one and returns zero.
- `write_block_update_total`: Returns one at once when the failure flag is clear and a status routine returns non-zero.
- `verify_write_adds_total`: Calls a write routine on the context block and compares its result with the length held in a data register.
- `add_count_to_total`: Adds the length in a data register to the running total stored in the context block and returns zero.
- `free_linked_node_list`: Walks the singly linked list whose head sits in the context block, calling a called routine on each node with the fixed size 118.
- `reset_node_list_head`: Sets the list head pointer and the node count in the context block to zero, emptying the list without visiting its nodes.
- `pack_context_flags_byte`: Packs eight flag bytes from the context block into one output byte, one flag per bit, and stores it through the caller's pointer.
- `parse_command_switches`: Parses the argument words of the command, each a dash-prefixed switch letter with an optional attached value.
- `next_option_value`: Walks the argument vector past words that do not start with a dash, records the index of the first option seen, and returns the text attached after the switch letter, or an empty string when nothing is attached.
- `scan_option_argument_entries`: Advances through an argument list in the same way as its neighbour but handles more cases.
- `next_positional_argument`: Returns the next argument that is not a dash-prefixed switch, skipping switches on the way.
- `parse_slash_date_fields`: Parses a slash-separated year, month and day string.
- `store_year_parse_month_day`: Fragment of the date parser entered after the year is known: it stores the year byte, then parses the month and day fields after the slash separators with the same range checks of 12 and 31.
- `parse_month_day_fields`: Parses the month and then the day from the remaining string, after an optional leading slash, with ranges one to 12 and one to 31.
- `parse_month_then_day`: Parses a month number from the start of the remaining string, then skips to the next slash and parses a day number, checking the ranges one to 12 and one to 31.
- `parse_day_after_slash`: Skips to the character after the first slash, then parses a day number from one to 31 into the output.
- `parse_day_skip_slash`: Skips an optional leading slash, then parses a day number that must be from one to 31 into the output at offset one.
- `parse_day_number_checked`: Parses a day number from the current position, requiring a value from one to 31, and stores it in the output at offset one.
- `stub_return_zero`: A stub that only returns zero, called from five places as a no-op default.
- `initialise_default_option_block`: Sets the option defaults in the context block, including flags, numeric limits and a buffer size chosen from a mode byte, and copies a fixed default string into a context buffer.
- `update_pause_flags_counter`: Decrements a counter in the context to update the pause flag and related status bytes.
- `check_char_class_table`: Looks up the first byte of the string in a character-class table and loops while one class bit is set, then calls a called routine.
- `notify_handler_list_then_call`: Walks the linked list of handler function pointers stored in the context, calling each one, then passes its argument to a finishing called routine.
- `run_mode_setup_sequence`: Runs three setup called routines, then when the mode byte is below two it passes a fixed string and the value 100 to a called routine.
- `detect_runtime_environment`: Finds the current task and checks a 16-bit version field: below 37 it opens the arp.library and records the library pointer, otherwise it sets a bit on the task flag word.
- `wrap_helper_call`: Calls a single called routine with no arguments and returns, acting as a thin wrapper.
- `save_field_set_minus_one`: Saves a 32-bit field from a structure referenced by the context into the context, overwrites that field with all ones, and sets a flag byte to one.
- `format_temp_file_name`: Formats a temporary file name from a fixed pattern containing the text LZXCmprTemp and a per-slot value, writing it into the slot's entry of a table whose records are 92 bytes apart.
- `clear_buffer_open_file`: Clears a 144-byte buffer, then matches the mode string against fixed strings to choose an open code.
- `open_file_by_mode`: Matches the mode string against fixed strings to choose an open code, reporting an unrecognised mode and exiting with code 40.
- `open_file_checked_mode`: Accepts one of two fixed mode strings to select an open code, and reports any other mode and exits with code 40.
- `open_file_single_mode`: Accepts a single fixed mode string to select an open code, and reports any other mode and exits with code 40.
- `attempt_file_open`: This routine attempts to open a file and manages error handling by calling various routines to format messages or notify handlers based on the success of the operation.
- `try_open_store_handle`: Calls a system routine through the library base and stores a non-zero handle into the output pointer.
- `store_value_return_zero`: Stores the 32-bit value passed in a data register through the output pointer and returns zero.
- `try_dos_open_handle`: Calls the dos.Open library routine and stores a non-zero handle through the output pointer.
- `store_data_register_via_pointer`: Writes a data register value through an address register pointer and returns zero, with no other effects.
- `close_and_clear_buffer_2`: Closes a file handle using a library call and then clears a buffer of 144 bytes.
- `close_and_clear_buffer`: Calls the dos.Close library routine, then clears a 144-byte buffer and returns zero.
- `call_read_2`: Loads arguments and a library base to call a read function, returning the result.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `wrap_dos_write`: Calls the dos.Write library routine with the arguments already in place, acting as a thin wrapper.
- `fetch_standard_io_handles`: Stores the dos.Input handle in the context, and stores the dos.Output handle too unless a flag byte is set, in which case the output handle is cleared to zero.
- `call_dos_routine_if_enabled`: When the flag byte in the context is clear, it calls a dos library routine at the library offset of minus 792; otherwise it returns without doing anything.
- `report_user_abort`: Runs a cleanup routine, then passes the user-abort message text to a print called routine.
- `report_code_five_set_flag`: When a status flag is clear, it reports the code 5 through a called routine and sets a second flag to one.
- `report_code_four_clear_flag`: When a status flag is clear, it reports the code 4 through a called routine and clears a second flag to zero.
- `write_when_output_open`: When a flag byte is clear and the stored output handle is non-zero, it calls the dos.Write library routine.
- `write_data_chunks`: This routine writes data in chunks by repeatedly calling a library routine and checking a status flag, decrementing a counter until the total amount is processed.
- `write_chunks_with_status_check`: Writes data in chunks up to a maximum size and checks a status routine between iterations.
- `write_1024_byte_chunks`: Writes data in fixed chunks of 1024 bytes through dos.Write, checking a status routine between chunks.
- `format_and_write_message`: When a flag is clear and the output handle is set, it formats a message into a context buffer using the caller's arguments, then measures the text and writes it through dos.Write.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_dos_if_flag_clear`: Checks a flag byte in the global data block and, only when it is zero, calls a DOS library function.
- `execute_dos_if_flag_unset`: Checks a flag byte within a global data block; if the byte is zero, it executes a function pointer retrieved from a table.
- `wrap_dos_read_call`: Surrounds a dos.Read library call with called routine calls, adding a further pair when a flag byte is set.
- `read_key_matching_set`: Repeatedly reads a key through a called routine and echoes it with a formatted print, folding lower case letters to upper case via a character-class table.
- `read_key_until_match`: A gap fragment of the key-matching loop: it reads a key, folds case through a character-class table and repeats until the key matches the character at the given pointer, then returns it.
- `read_key_until_match_2`: A gap fragment of the key-matching loop: it reads a key, folds case through a character-class table and repeats until the key matches the next character of the given string, then returns it.
- `stub_return_nothing`: An empty routine that only returns.
- `wrap_call_with_messages`: Passes a fixed text to a called routine, calls a called routine with no arguments, then passes the same fixed text to the called routine again.
- `compute_intuition_size_ratio`: Opens intuition.library and, under Forbid and Permit, reads the window dimension fields and divides the available size by a stored unit value, clamping the quotient to at least 8.
- `check_string_for_wildcards`: Scans the string for any of eight special characters copied from a fixed table, several of them AmigaDOS pattern symbols.
- `check_string_for_wildcards_2`: A gap fragment of the same character scan: it walks the string and returns one on the first character found in the eight-character table, or zero at the end of the string.
- `scan_string_for_wildcards`: Scans a string for any of eight special characters stored in a table.
- `stub_return_zero_2`: A stub that only returns zero.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `wrap_exec_free_memory`: Calls the exec.FreeMem library routine with the arguments already in place and returns, acting as a thin wrapper.
- `allocate_with_halving_retry`: Tries to allocate a block of the requested size, halving the size after each failure until it reaches 256 or less.
- `retry_halving_allocation_fatal`: Repeatedly halves a size value and asks the allocator for that much memory, storing the first successful block through a pointer and returning zero.
- `wrap_helper_with_two_args`: Passes its two arguments on to a called routine and returns.
- `consume_bits_refill_word`: Removes the given number of bits from the bit buffer in the context, and when the bit count goes negative it refills the buffer with the next 16-bit word from the input pointer.
- `read_bits_and_consume`: Captures the bit buffer, consumes the given number of bits, and returns the value of those low bits from the captured buffer.
- `decode_huffman_symbol_tree`: Decodes one symbol from a Huffman-style table: it looks up the next eight bits and, when the entry is a tree link, walks the tree one bit at a time.
- `decode_huffman_symbol_fast`: Decodes one symbol from a Huffman-style table using a six-bit lookup, walking the tree one bit at a time when the entry is a link.
- `initialise_bit_buffer`: Loads two 16-bit words from the input pointer into the 32-bit bit buffer, sets the bit count to 16, and advances the input pointer by two words.
- `fill_256_decoded_bytes`: Fills a 256-byte buffer by calling the byte decoder once per byte.
- `read_bytes_sum_checksum`: Reads the given count plus one bytes through a called routine into the buffer and returns their running 16-bit total.
- `read_lzx_block_header_tables_2`: Reads the header of an LZX block, including bit fields that give buffer and table sizes, then allocates the block buffer and reports an insufficient-memory error when that fails.
- `count_newlines_in_string`: Counts the newline characters in a zero-terminated string and returns the count.
- `wrap_helper_with_one`: Calls a called routine with the constant one as its argument and returns.
- `print_text_with_paging`: Writes a text block one line at a time, splitting it at newlines and starting a new page whenever the line count reaches the value computed from a context field.
- `pack_four_bytes_shifted`: A gap fragment that combines four bytes from the pointer into one 16-bit value, shifting left four bits before adding each byte.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `set_processing_flag`: This routine checks a byte in a global structure and, if it is zero, sets it to one and calls another routine.
- `compare_path_directories`: Finds the directory part of each path, which ends at the last slash or colon, and compares the two parts with a called routine.
- `check_path_directory_helper`: Finds the directory part of a path, ending at the last slash or colon, and passes it to a called routine.
- `test_helper_returns_zero`: Returns true when the called routine it calls returns zero.
- `return_zero_constant`: This routine is an empty function that always returns zero to the caller.
- `check_path_ends_slash`: Returns one when the string is non-empty and ends with a slash, and zero otherwise.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `normalise_path_separators`: Replaces every backslash in the string with a forward slash, in place.
- `stub_return_nothing_2`: An empty routine that only returns; it sits in a gap between functions.
- `split_path_into_parts`: Splits a path into a device name before a colon, a directory part and a final file name, storing each into the output buffers supplied.
- `split_path_tail_parts`: A fragment of the path splitter that copies the remainder of the path into an output, then extracts the directory part and any device name into the supplied buffers.
- `split_path_remainder_parts`: A fragment of the path splitter, entered after the remainder has been located, that copies the rest of the path and extracts the directory part and device name into the supplied buffers.
- `terminate_path_part_buffers`: A fragment that terminates the directory and device buffers at the computed lengths, without copying any text.
- `terminate_device_and_dir_fields`: Writes a terminator into the directory field when its buffer is supplied, and also terminates the device field when the character at the separator offset is a colon.
- `find_matching_list_node`: Walks a linked list of named nodes and returns the first one whose name matches the target under the mode bits, which can add a flag check and name normalisation.
- `find_matching_list_node_2`: Walks a linked list of named nodes and returns the first one whose name matches the target under the flag bits, which can add a flag check and name normalisation.
- `find_matching_list_node_3`: Walks a linked list of named nodes, comparing each name with the target under the flag bits and optional name normalisation.
- `find_matching_list_node_4`: Walks a linked list of named nodes, comparing each name with the target after an optional flag check and name normalisation.
- `search_list_after_dos_check`: Makes two dos library checks on the context before walking a linked list of named nodes for a name match.
- `find_matching_list_node_5`: Walks a linked list of named nodes and returns the first one whose name matches the target, with optional flag and name normalisation.
- `return_zero_value`: This routine is empty and simply returns a zero value to the caller.
- `add_named_list_node`: Checks existing nodes for a duplicate name when requested, then allocates a block of the name length plus 20 bytes, copies the name into it and appends it to the list.
- `add_named_list_node_2`: Allocates a block of the name length plus 20 bytes, copies the name into it and appends it to the list, after an inline check for an existing duplicate.
- `add_named_list_node_3`: Allocates a block of the name length plus 20 bytes, copies the name into it and appends it to the list.
- `initialise_named_list_node`: A gap fragment that zeroes a node's fixed header, copies the name into it and sets its flag byte, ready to be linked into the list.
- `append_list_node_or_error`: Appends a node to the list tail, but when the tail is missing it prints an internal list error and sets the status to one.
- `append_list_node_tail`: Links the node onto the list tail and clears the status code to zero.
- `report_output_open_failure`: Calls a called routine with the output name buffer from the context and, if it reports failure, prints the can't-open-for-output message.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_buffer_cleanup`: This routine checks if a pointer in a global structure is non-zero and, if so, passes that pointer to another routine.
- `allocate_multiple_buffers`: Attempts to allocate several blocks of memory of varying sizes through a library call.
- `allocate_work_buffers`: This routine attempts to allocate several memory blocks of varying sizes; if any allocation fails, it calls another routine to clean up and returns one.
- `allocate_lzx_work_buffers_4`: Attempts to allocate several memory blocks of varying sizes through a memory allocation routine and stores the resulting pointers in a structure.
- `allocate_lzx_work_buffers`: Allocates the working buffers in sequence from a block sized from a context value plus a fixed amount, then two 32768-byte buffers, a 4096-byte buffer and a 768-byte table.
- `allocate_lzx_work_buffers_2`: Allocates the working buffers in sequence from the first 32768-byte buffer: two of those, a 4096-byte buffer and a 768-byte table.
- `allocate_lzx_work_buffers_3`: Allocates the working buffers in sequence from a 32768-byte buffer, then a 4096-byte buffer and a 768-byte table.
- `allocate_work_tables_tail`: Allocates a 4096-byte buffer, runs a setup called routine on the 768-entry table, then sets the two stored buffer pointers offset by 65536 bytes and the ready flag.
- `setup_work_table_pointers`: Runs a setup called routine on the 768-entry table and, when it succeeds, sets the two stored buffer pointers offset by 65536 bytes and the ready flag.
- `finish_work_buffer_setup`: Sets the two stored buffer pointers offset by 65536 bytes, calls two called routines, sets the ready flag and returns zero.
- `free_lzx_work_buffers`: Frees each allocated work buffer whose pointer is set, passing each its size, and clears the stored pointers.
- `free_optional_buffer_sized`: When the buffer pointer in the context is set, it frees that buffer using the size held in the context and then clears the pointer.
- `update_counter_and_report`: A gap fragment: when the first context counter is below the second, it calls a called routine with the difference and, if that returns non-zero, calls another called routine with the context block and the value 7.
- `write_block_update_total_2`: Returns one when the failure flag is clear and a status routine returns non-zero.
- `flush_buffer_with_flag_one`: Calls the buffer flush helper on a context buffer with the constant one and returns nothing.
- `allocate_context_tables_block`: Allocates a 7728-byte block and, on success, stores it in the context and sets two table pointers into it at offsets 1548 and 4638 bytes.
- `reset_context_table_pointers`: Resets the two table pointers in the context to offsets 1548 and 4638 bytes into the stored block, and returns zero.
- `free_context_tables_block`: Frees the 7728-byte table block when its pointer is set, then clears the pointer.
- `build_doubling_run_table`: On first use it fills a 512-entry lookup table in the context with values in runs whose lengths double, starting from the values zero to three and rising by two per run.
- `build_doubling_run_table_2`: A gap fragment of the same table fill, continuing the doubling runs from its current position until the table index passes 511.
- `build_doubling_run_table_3`: A gap fragment of the same table fill, starting a doubled run and alternating the two values until the table index passes 511.
- `allocate_code_table_set`: Allocates a zero-filled block sized from a table length (eight bytes per entry plus two) and stores its start and three internal table pointers into the output array.
- `init_code_table_set`: Zero-fills an existing block sized from a table length and stores its start and three internal table pointers into the output array.
- `fill_code_table_block`: Fills a block with a byte value over the table length and stores its start and three internal table pointers into the output array.
- `free_code_table_set`: Frees the code table block when its start pointer is set, then clears all four stored pointers.
- `wrap_exec_raw_do_fmt`: Calls the exec.RawDoFmt library routine with the arguments already in place, which formats text through a callback, and returns.
- `invoke_raw_do_format`: Calls the exec library's RawDoFmt entry through the global library base, with its format and argument data supplied by the caller.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `update_crc32_checksum`: Updates a running CRC-32 over a buffer using a 256-entry table in the context, and stores the running value in the context.
- `process_buffer_in_chunks`: Iteratively calls another routine to process a data buffer in fixed-size blocks until the remaining size reaches zero.
- `iteratively_process_buffer_chunks`: Repeatedly calls another routine while decrementing a value by a fixed block size until the value reaches zero.
- `update_checksum_loop`: This routine executes a loop that calls another routine and performs arithmetic on a register until a specific value is reached.
- `build_huffman_decode_table`: Counts how many symbols use each code length, derives the first code of each length and fills the decode table from those codes.
- `run_init_function_list`: Walks a null-terminated table of function pointers in the context, calling each entry while a flag is set or the last result is zero.
- `run_pointer_list_until_done`: Walks a zero-terminated list of handler pointers and calls each handler entry, sets a done flag once the list is exhausted, and returns the last handler result.
- `call_indirect_handler_return_constant`: Calls the handler through a pointer stored in a global variable, passing the address of a global variable, and returns the constant 20.
- `invoke_global_callback`: Calls the function pointer stored in a global table, passing the address of a global variable, and then returns.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings one character at a time after converting lowercase letters to uppercase.
- `parse_signed_integer`: Reads an optional plus or minus sign and then the decimal digits of an integer from the string, storing the signed value through the output pointer.
- `lowercase_string_in_place`: Converts each uppercase letter in the zero-terminated string to lower case in place using the character-class table, and returns the start pointer.
- `lowercase_string_in_place_2`: A gap fragment of the same in-place lower-case conversion, without a return of the start pointer.
- `lowercase_string_tail`: A gap fragment of the case-folding copy: it walks the string from the given pointer, writing each character in lower case until the terminator.
- `uppercase_string_in_place`: Converts each lowercase letter in the zero-terminated string to upper case in place using the character-class table, and returns the start pointer.
- `uppercase_string_in_place_2`: A gap fragment of the same in-place upper-case conversion, without a return of the start pointer.
- `uppercase_string_tail`: A gap fragment of the case-folding copy: it walks the string from the given pointer, writing each character in upper case until the terminator.
- `traverse_and_free_pending_nodes`: Iterates through a linked list of pending nodes, updating the list head and a secondary pointer as it traverses.
- `free_pending_node_list`: Walks a linked list of pending nodes in the context, freeing each node with the exec.FreeMem library routine.
- `shutdown_and_return_value`: Saves the value passed in into the frame, runs an optional callback, then calls a sequence of library routines to release resources before returning the saved value.
- `tokenize_command_string`: Parses a string into individual tokens by splitting on whitespace and handling quoted substrings.
- `free_tokenised_args_buffer`: When the word count and the word array pointer are both non-zero, it frees the array with the exec.FreeMem library routine and returns that call's result.
- `compare_strings_bounded`: Compares up to the given number of characters of two strings, returning minus one, one or zero for less, greater or equal at the first difference.
- `return_signed_compare_result`: Returns minus one when the processor flags left by the caller show less-than-or-equal, and one otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `append_string_to_buffer`: Finds the terminating zero of the destination buffer and copies the source string onto its end, then returns the original destination pointer.
- `copy_bounded_string_padded`: Copies characters from the source string into the destination up to the given count, stopping at the terminator and zero-filling the rest of the count.
- `copy_bounded_string_tail`: A gap fragment of the bounded copy: it copies the source bytes up to the terminator, then zero-fills the remaining count.
- `zero_fill_bytes`: Writes zero to a run of bytes starting at the pointer, using the count as the loop bound.
- `check_char_class_table_2`: A gap fragment of the character-class scan: it checks the string's first byte against a class bit and calls a called routine.
- `notify_handler_list_then_call_2`: Walks the handler list in the context, calling each one, then passes its argument to a finishing called routine.
- `notify_handler_list_then_call_3`: Walks the handler list in the context, calling each one, then calls a finishing called routine with no arguments.
- `unwind_queue_free_nodes`: Restores several fields from its argument block and, while the queue in the context is non-empty, pops nodes and frees each with exec.FreeMem.
- `reset_input_state`: The routine calls another routine and then returns.

### arp.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `setup_arp_record_block`: Part of the arp.library shared library: it reads entries through a library call and, when the walk completes, fills in a structure and allocates a block for it.
- `read_arp_records_then_setup`: Part of the arp.library shared library: it reads a counted list of records through a library call, and when the count runs out it runs the same setup and allocation sequence as the record setup routine.
- `finish_arp_record_setup`: Part of the arp.library shared library: it fills in a structure from the record, allocates a block through a library call and links it in, or runs a cleanup library call and returns when the allocation fails.
- `increment_arp_counter_loop`: Part of the arp.library shared library, a gap fragment: it repeatedly calls a called routine and a library routine at offset minus 462 until a flag is set, then increments a 16-bit counter at offset 32.
- `clear_field_call_library`: Part of the arp.library shared library, a gap fragment: it clears a 32-bit field at offset 8 of the structure in the address register and calls a library routine through a base pointer stored in the structure.
- `allocate_sized_block_header`: Part of the arp.library shared library: when the size is non-zero it allocates a block with the exec.AllocMem library routine and, if the allocation succeeds, stores the size plus four at the start of the block.

### Gloom - Special Edition 97 (1997)(Black Magic)[h DLM](Disk 1 of 3)[HD].adf:lzx

- `run_program_startup`: Program entry point that allocates a block, copies 816 longwords of initial data into it and adds the block base to each entry of a relocation list.
- `release_process_resources_reply`: Releases process resources by unlocking a directory lock, closing a library, freeing allocated memory and replying to a message, with task switching forbidden during the message step.
- `execute_callback_with_argument`: Invokes a function pointer retrieved from a global table and passes a global variable as its parameter.
- `execute_handler_function`: Calls a function pointer stored in a global variable using a global variable as an argument and returns the constant twenty.
- `tokenize_and_expand_command_string`: Splits a string into an array of pointers based on whitespace while handling quoted substrings.
- `execute_conditional_task`: Calls a routine with two different sets of arguments based on a return value.
- `set_failure_flag`: Stores the value one into a specific offset within a structure.
- `verify_write_adds_total_2`: Checks a flag, calls a routine on a context block and a global buffer, and adds the result to a running total if the length matches.
- `wrap_dos_write_2`: Calls a routine on a context block and adds the result to a running total if the length matches.
- `add_length_to_running_counter`: Adds a length value to a running counter in a structure, with no return value.
- `reset_node_list_head_2`: Iterates through a linked list to call a routine on each node, then sets the list head and node count to zero.
- `clear_list_head_and_tail`: Sets the head and tail pointers of a list in a structure to zero and returns.
- `pack_six_flags_into_byte`: Packs six flag bytes in a structure into one value by OR-ing in the bit values four, eight, sixteen, thirty-two, sixty-four and one hundred and twenty-eight for each nonzero flag.
- `parse_command_line_arguments_2`: Iterates through a list of command-line arguments and updates a configuration context block based on specific character switches.
- `scan_option_argument_entries_2`: Advances through an argument list, skipping entries that do not start with a dash and returning a pointer to a result string or a null pointer.
- `scan_option_argument_entries_3`: Iterates through a list of pointers, skipping non-option arguments and returning a pointer to a specific character within an argument list.
- `next_nonoption_argument`: Walks an argument list, skipping dash-prefixed options and noting a double-dash terminator.
- `parse_year_date_fields`: Parses a year from a string and then attempts to parse subsequent month and day fields separated by slashes.
- `parse_month_day_fields_2`: Parses a month and a day from a string, checking that the values fall within valid numerical ranges.
- `parse_month_then_day_2`: Parses a month value and then a day value from a string, using a slash as a separator.
- `parse_day_after_slash_2`: Parses two numeric values from a string, separated by a slash, and stores them in a provided buffer.
- `walk_path_components_recursively`: Skips through a slash-separated path string one component at a time and recurses on the remainder.
- `check_path_after_slash`: Skips one optional leading slash, then recurses on the rest of the string and stores the returned byte in a state field.
- `check_recursive_result_range`: Calls itself without arguments, stores the byte it returns in a state field, and reports failure when that byte is zero or above 31.
- `return_zero_constant_2`: The routine returns a constant zero value to the caller.
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
- `attempt_file_open_2`: Checks a flag and a value before attempting to call a function through a pointer in a table.
- `store_handle_or_release`: Stores a nonzero result from a library call through a pointer.
- `write_value_through_pointer`: Writes its argument value through a pointer register and returns zero.
- `try_open_file_handle`: Opens a file through the dos library and stores the returned handle when the call succeeds.
- `store_data_register_through_pointer`: Writes the value held in a data register through an address register pointer and returns zero.
- `close_handle_and_clear_state`: Closes the stored file handle when one is set, or reports an internal error when the handle is already zero.
- `close_file_and_clear_block`: Calls the dos close function and then writes zero into each of 144 bytes starting at the given pointer.
- `read_bytes_if_count_positive`: Returns zero when the requested byte count is below one.
- `forward_dos_read_call`: Calls the dos read function with the arguments already in the registers and returns nothing.
- `forward_dos_write_call`: Calls the dos write function with the arguments already in the registers and returns nothing.
- `fetch_standard_input_output_handles`: Fetches the standard input handle and, unless a flag is set, the standard output handle, storing each in state fields.
- `execute_conditional_callback`: Checks a global flag and, if clear, calls a function pointer retrieved from a table.
- `report_user_abort_and_cleanup`: Runs a cleanup routine and then passes the fixed user-abort message text to an output routine.
- `write_when_output_open_2`: Checks a flag byte and, if clear, calls another routine with a specific constant and sets a flag.
- `print_message_and_clear_flag`: When a flag is clear, passes a fixed text and the value four to an output routine, then clears the flag to zero.
- `write_when_output_enabled`: Writes through the dos write function when an output-disabled flag is clear and a stored output handle is nonzero.
- `write_data_in_chunks`: Iterates through a loop to call a function through a pointer, decrementing a parameter by a maximum of 1024 each time.
- `write_chunks_checking_between_blocks`: Writes a count of bytes in blocks of at most one kilobyte, running a check routine before each block and returning 11 when that check fails.
- `write_blocks_until_count_exhausted`: Writes the count held in a data register in blocks of one kilobyte, running a check between blocks and returning 11 if the check fails.
- `format_and_write_output_line`: When output is enabled and a handle is stored, formats a value into a state text buffer and writes that null-terminated text through the dos write function.
- `return_void_stub`: Does nothing and returns immediately.
- `call_dos_if_flag_clear_2`: Checks a flag byte in the global data block and, only when it is zero, calls a DOS library function through the library base.
- `call_dos_if_flag_set`: Checks a flag byte in a global data block and, if it is zero, executes a function pointer retrieved from a table.
- `read_byte_via_dos`: Calls a routine, reads one byte through the dos read function into a stack slot, then returns that byte.
- `read_until_character_in_set`: Reads and echoes characters, folding lowercase letters to uppercase when a class table marks them.
- `read_key_matching_string`: Reads and echoes characters, folding lowercase letters to uppercase, until one matches a character in the given string.
- `read_key_from_candidate_list`: Reads and echoes characters, folding lowercase letters to uppercase, and returns the first one that matches an entry in a candidate string.
- `nop_padding_sequence_2`: This routine is empty and consists only of instructions that perform no operation.
- `wrap_routine_with_text_output`: Outputs a fixed text, calls a routine, and then outputs the same text again.
- `measure_window_space_per_item`: Opens the intuition library, reads screen and window geometry while task switching is forbidden, and closes the library.
- `string_contains_reserved_character`: Copies an eight-character table of reserved name characters onto the stack.
- `scan_string_for_table_match`: Walks a string and checks each character against an eight-entry character table.
- `find_table_character_in_string`: Advances through a string, checking each character against an eight-entry table held on the stack.
- `stub_return_zero_3`: Returns zero immediately, with no other effects.
- `forward_memory_allocation_call`: Calls the exec allocation routine through the system base with the arguments already in registers, and returns nothing.
- `forward_memory_free_call`: Calls the exec memory-release routine with the arguments already in registers, and returns nothing.
- `allocate_with_halving_fallback`: Stores the requested size, then calls an allocation routine, halving the size after each failure until it reaches 256 or less.
- `retry_allocation_with_smaller_sizes`: Halves a size value after each failed allocation attempt until it reaches 256 or less.
- `forward_two_argument_call`: Passes its two parameters on to another routine and returns.
- `consume_bits_refill_buffer`: Discards a number of bits from a bit buffer by shifting it.
- `read_bit_field_and_consume`: Returns the next requested number of bits from a bit buffer, masked to that width, and then consumes those bits from the buffer.
- `decode_huffman_symbol_from_table`: Looks up a symbol in a table indexed by the next eight bits of the bit buffer, walking a binary tree bit by bit when the entry is negative.
- `decode_six_bit_table_symbol`: Looks up a symbol using the next six bits of the bit buffer, walking a binary tree when the entry is negative.
- `initialise_bit_buffer_from_input`: Reads two 16-bit words from the input pointer, advances the pointer past them, and loads them into the bit buffer with a bit count of sixteen.
- `read_256_bytes_into_table`: Reads 256 values through a read routine and stores each one in consecutive bytes of the destination table.
- `read_bytes_and_sum`: Reads bytes through a read routine into a buffer until the count runs out, and returns the running total of the bytes read.
- `read_lzx_block_header_tables`: Reads the LZX block header by filling code length tables from the bit stream and allocating the decoding table memory.
- `count_newline_characters`: Counts the newline characters in a null-terminated string and returns the count.
- `forward_call_with_value_one`: Calls another routine with the value one and returns.
- `print_text_wrapped_to_width`: Outputs a text string line by line, breaking each line to fit the width derived from the window size and a stored margin.
- `combine_four_bytes_into_word`: Combines four bytes starting at the given address into one 16-bit value, shifting the running result left by four bits before adding each byte.
- `null_operation_routine`: This routine is empty and performs no operations before returning.
- `set_flag_and_call_once`: On its first run, sets a state flag to one and calls a routine with the value zero.
- `compare_path_base_names`: Finds the file-name part after the last slash or colon in each of two paths, compares those parts, and returns one when they match and zero otherwise.
- `test_last_path_component`: Finds the last path component of a non-empty string and calls a routine on it.
- `check_routine_result_is_zero`: Calls a routine and returns one when its result is zero, and zero otherwise.
- `return_zero_constant_3`: This routine returns a zero value to the caller.
- `check_path_ends_with_slash`: Returns one when the given string is not empty and its last character is a slash, and zero otherwise.
- `return_zero_2`: This routine performs no operations and returns a value of zero to the caller.
- `normalise_path_separators_2`: Replaces every backslash in a string with a forward slash, in place, up to the terminating zero.
- `return_from_subroutine`: This is an empty routine that performs no operations before returning.
- `split_path_into_parts_2`: Clears up to three optional output buffers, then splits a path into its device name before a colon and its directory and file parts, copying each part into the buffers that were supplied.
- `copy_path_component_buffers`: Copies the file-name part of a path into an output buffer, or the whole string when no separator is found.
- `copy_path_name_and_volume`: Copies the file-name part of a path into an output buffer, then copies the directory and device parts into their buffers when those are supplied.
- `copy_path_directory_and_device`: Copies the directory part of a path and its device part into their output buffers when those buffers are supplied.
- `copy_device_part_when_present`: Zero-terminates the directory output buffer and, when a device separator is present, copies the device part into its buffer.
- `find_entry_by_name`: Walks a linked list of named entries and returns the first entry whose name matches the given text.
- `scan_named_entry_list`: Scans a linked list of named entries from its head pointer and returns the first entry that matches the target name.
- `find_first_matching_list_entry`: Walks a linked list of entries and returns the first one that the comparison routine accepts.
- `search_entries_with_transform`: Checks each list entry against a comparison routine, optionally comparing a transformed copy of the name, and returns the first entry that matches.
- `scan_entries_with_break_check`: Repeatedly checks for a break through the dos library, then walks a list of entries and returns the first one that matches.
- `walk_entries_for_match`: Walks a list of entries, skipping those that fail a break-checked comparison, and returns the first entry the comparison accepts.
- `return_zero_value_2`: This routine is empty and simply returns zero to the caller.
- `append_named_list_entry`: Optionally rejects duplicate names, allocates a new list entry sized for the name, copies the name into it, and appends it to the list tail.
- `append_unique_named_entry`: Rejects a name already present in the list, then allocates an entry sized for the name, copies the name in, and appends it.
- `allocate_and_append_named_entry`: Allocates a list entry sized for the name, copies the name into it, and appends it to the list tail.
- `initialise_and_append_entry`: Clears a 20-byte entry record, copies the name into it, marks it as used, and appends it to the list tail.
- `link_entry_at_list_tail`: Links a new entry onto the tail of a list.
- `link_entry_at_tail_unchecked`: Links a new entry onto the tail of a list and sets the status to zero without checking the tail.
- `open_output_file_or_report`: Opens the output file named in the state block through a routine.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `call_stored_handler`: Checks if a pointer stored at a specific offset in a context block is non-zero, and if so, calls that routine.
- `allocate_lzx_working_buffers`: Allocates the LZX working buffers, with sizes derived from state values, and stores each pointer in the state block.
- `allocate_lzx_secondary_buffers`: Allocates the remaining LZX working buffers, with sizes derived from state values, and stores each pointer.
- `allocate_lzx_buffers_second_window`: Allocates the LZX working buffers from the second window buffer onward, storing each pointer in the state block.
- `allocate_lzx_buffers_from_table`: Allocates the LZX working buffers from the table buffer onward, storing each pointer in the state block.
- `allocate_lzx_buffers_after_table`: Allocates the remaining LZX buffers after the table buffer, recording the stream pointer and storing each allocated pointer.
- `allocate_lzx_buffers_second_block`: Allocates the LZX buffers from the second fixed-size block onward, storing each pointer.
- `allocate_lzx_small_block`: Allocates the last small LZX block, then sets up the lookup tables and the ready flag.
- `finish_lzx_buffer_setup_checked`: Sets up the lookup tables for the LZX buffers and, when that succeeds, stores the output pointers and sets the ready flag.
- `finish_lzx_buffer_setup`: Stores the output pointers into the state block, sets the lookup tables, and sets the ready flag.
- `free_lzx_working_buffers`: Releases each LZX working buffer that is currently allocated, clearing its stored pointer.
- `free_lzx_pending_buffer`: Releases the buffer held in a pending slot when one is stored, passing the recorded size, and then clears the slot.
- `flush_lzx_pending_output`: Writes out any bytes between the pending output position and the stored end position.
- `write_block_to_output_buffer`: Writes the requested number of bytes to the output through a buffer routine and adds that count to a running total.
- `report_error_code_one`: Passes the error code one, together with an error-state pointer, to an error routine and returns.
- `allocate_lzx_pointer_block`: Allocates one block of working memory and, when that succeeds, stores three pointers into it at fixed offsets.
- `reset_lzx_pointer_block`: Recomputes the three pointers into the existing working block from its stored base address and returns zero.
- `free_lzx_pointer_block`: Releases the working block when it is allocated and clears its stored address.
- `build_lzx_length_lookup_table`: On its first run, fills a 512-entry lookup table with values that rise in doubling runs and sets a done flag.
- `fill_lookup_runs_after_first`: Fills the remaining lookup entries in doubling runs, writing a raised value first and then the base value, until the table is full.
- `fill_lookup_runs_from_start`: Fills the remaining lookup entries in doubling runs, starting each run with a raised value and then the base value, until the table is full.
- `allocate_zeroed_tree_tables`: Allocates a block sized from a count, zeroes it, and stores four pointers into it at offsets derived from that count.
- `reset_zeroed_tree_tables`: Zeroes an existing block of the given size and stores the same four pointers into it.
- `fill_block_and_set_pointers`: Fills a block with a byte value for the given count, then stores four pointers into it at offsets derived from a size value.
- `free_zeroed_tree_tables`: Releases the block behind the first pointer when it is set, passing the size derived from the count, then clears all four pointers.
- `forward_formatted_output_call`: Calls the exec formatting routine through the system base with the arguments already in registers, and returns nothing.
- `invoke_raw_do_format_2`: Calls the exec library's RawDoFmt entry through a global variable, using format and argument data provided by the caller.
- `store_byte_at_pointer`: Writes the low byte of a data register to the memory address held in an address register.
- `update_crc32_checksum_2`: Updates a running CRC-32 checksum over a byte block using a 256-entry table in the state block.
- `checksum_in_megabyte_blocks`: When no data is given, sets the running checksum to all ones.
- `process_remaining_megabyte_blocks`: Splits a byte count into blocks of one megabyte, calling a routine for each full block and once more for the remainder.
- `process_megabyte_blocks_until_done`: Calls a routine for each full megabyte block of a count, subtracting the block size each time, and then calls it once more for any leftover bytes.
- `build_canonical_huffman_table`: Counts how often each code length appears in a byte array, derives the first code for each length, and fills a decoding table with the resulting entries.
- `run_handler_table_entries`: Walks a null-terminated table of handler pointers, calling each entry while the done flag is clear or no result exists yet.
- `run_handler_table_sequence`: Runs through a null-terminated table of handler pointers, calling each handler and keeping the last result.
- `call_indirect_handler_return_constant_2`: Calls a handler through a function pointer held in a global variable, passing the address of a global variable, then returns twenty.
- `invoke_global_callback_2`: Calls a function pointer stored in a global table with the address of a global variable as its argument, then returns nothing.
- `compare_case_insensitive_strings`: Compares two zero-terminated byte strings character by character, folding lowercase letters to uppercase.
- `parse_signed_integer_2`: Reads an optional plus or minus sign and then the decimal digits of a string, stores the signed value through a pointer, and returns the offset of the first character after the number.
- `lowercase_flagged_characters_in_place`: Converts each character in a zero-terminated string that a class table flags as uppercase to lowercase, in place, and returns the string pointer.
- `lowercase_flagged_tail_in_place`: Converts characters from a given pointer to the end of a zero-terminated string to lowercase where a class table flags them, in place, and returns nothing.
- `copy_string_with_case_table`: Stores a byte into a string buffer and then writes each following character, lowercasing those a class table flags, until the terminating zero is reached.
- `uppercase_flagged_characters_in_place`: Converts each character in a zero-terminated string that a class table flags as lowercase to uppercase, in place, and returns the string pointer.
- `uppercase_flagged_tail_in_place`: Converts characters from a given pointer to the end of a zero-terminated string to uppercase where a class table flags them, in place, and returns nothing.
- `copy_string_uppercase_flagged`: Stores a byte into a string buffer and then writes each following character, uppercased where a class table flags it, until the terminating zero is reached.
- `drain_and_free_queue`: Pops each node from a pending list, stores its next pointer and data, runs a flush routine when the list ends, and frees each node's memory.
- `drain_pending_list_entries`: Repeatedly pops nodes from a pending list, stores the next pointer, optionally runs a flush routine, and frees each node until the list is empty.
- `dispatch_message_with_hooks`: Records a value in a message frame, runs an optional hook, then either calls a cleanup routine or runs an exec library sequence to reply and release the message.
- `parse_command_line_arguments`: Splits a command line into arguments, honouring quoted strings and asterisk escapes for escape and newline characters.
- `free_block_if_allocated`: Releases a stored memory block when a counter and a pointer are both set, and returns the counter value.
- `compare_strings_up_to_length`: Compares up to a given number of characters of two strings.
- `return_comparison_sign`: Returns minus one when the preceding comparison shows the first value is less than or equal to the second, and one otherwise.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `append_string_to_buffer_2`: Finds the terminating zero of a destination buffer, copies the source string from there including its terminator, and returns the original destination pointer.
- `copy_string_with_zero_padding`: Copies up to a given count of characters from a source string into a buffer, stopping after the terminating zero, then zero-fills the rest of the count.
- `copy_string_pad_remainder`: Copies a zero-terminated string into a buffer of a given length and zero-fills whatever space remains.
- `zero_fill_memory_block`: Writes zero into every byte of a block, covering the given count plus one byte, and returns nothing.
- `scan_flagged_characters_then_call`: Scans characters of a string through a class-table flag, then calls a routine and returns a local value it leaves behind.
- `run_handler_list_then_forward`: Calls every handler in the global handler list, then passes its parameter on to another routine.
- `run_handler_list_then_finish`: Calls every handler in the global handler list in turn, then makes one further call with no arguments.
- `save_context_and_drain_queue`: Saves a block of context values onto a stack area, then pops queued nodes one at a time, calling a scheduling routine and freeing each node until the queue empties.
- `call_reset_input_state`: Calls another routine and then returns.
