# Game 2420 (nemac iv (1996)(zentek)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02420). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3612816 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 112 of 112 named (0 author symbols), 112 of 112 described (4 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 410 of 410 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4898 of 4898 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2930501 |
| code | 14500 |
| data | 164002 |
| padding | 8709 |
| unused | 495104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| joinz | 2 | 4300 | 87 |
| Nemac IV (1996)(ZenTek)(Disk 1 of 4)[HD].adf:NEMAC_4 | 2 | 196 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 177 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 8 |

## Cases handled

- runtime segment an address: 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 34 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 110 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nemac iv (1996)(zentek)(disk 1 of 4)[hd]/Nemac IV (1996)(ZenTek)(Disk 1 of 4)[HD].adf:NEMAC_4: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### joinz

- `program_entry_setup_and_run`: Program entry point: clears a 646-word block, opens the DOS and exec libraries, allocates memory and waits on a message port before running the main sequence of library calls.
- `finish_request_with_result`: Stores a result value in the state record, calls an optional callback and two helper routines, then either runs a closing chain of calls or a single cleanup call, and returns the stored result.
- `parse_command_line_arguments`: Splits a command line into quoted and unquoted words, decodes asterisk escapes such as newline and a few other characters, stores the words in an argument table, and sets up the input and output handles.
- `split_path_at_separator`: Scans a name backward for the last colon or slash, copies the text after it into a buffer, and passes the text before it to a routine; when no separator exists it passes at most 31 characters through instead.
- `stack_check_then_init_fields`: Checks stack space first; if it is short it reports an error and upper-cases the message text.
- `stack_check_then_init_field`: Checks stack space first; if it is short it reports an upper-cased error message.
- `read_yes_no_answer`: Reads input characters up to the end of the line, returning one for a yes answer and zero at end of input, and defers to another routine for other answers.
- `read_line_and_store_name`: Reads a line, strips a trailing newline, passes it to a handler, and copies the accepted line into a stored name field, returning one on success and zero when input ends or the handler rejects it.
- `process_yes_no_answers`: Repeatedly reads an answer: a no runs a further step and asks again, while a yes runs another step and sets a completion flag.
- `parse_tagged_file_record`: Parses a tagged record from a file: matches a three-character tag and, when it matches, reads a size field and several values into state, returning one on success and zero on an unmatched or malformed record.
- `invoke_hook_return_result`: Calls a routine through a stored pointer twice, discarding the first result and returning the second; when no pointer is set it returns zero.
- `prepare_and_lookup_name`: Clears a block of state, sets two fixed name strings, and if the name ends in a dot followed by three alphanumeric characters it strips that suffix before calling a routine to look up the name, recording the result in state.
- `open_input_stream_buffered`: Stores a name pointer and resets state, then opens an input stream through library calls; on success it allocates a buffer and reads characters up to a newline, and on any failure it closes what it opened.
- `read_line_for_letter_a`: Prints one of two prompts depending on a flag, reads characters to the end of the line, and returns one if an a or A appears or input has ended, otherwise zero.
- `read_big_endian_long`: Assembles four consecutive bytes, most significant first, into a 32-bit value and returns it.
- `write_big_endian_long`: Writes a 32-bit value into four consecutive bytes, most significant first, and returns the low byte of the value.
- `allocate_halving_buffer_size`: Caps a requested size at 32768, then halves it until a memory allocation succeeds or the size falls to 512 or less; it records the size used and the pointer it obtained.
- `sum_byte_range`: Adds up the given number of bytes from a buffer and returns the total.
- `scan_string_until_match`: Passes each character of a string to a check routine until the string ends, the check returns a nonzero result, or a control character appears; returns one when it stopped early and zero when it reached the end.
- `init_five_field_record`: Stores five arguments into a fixed record layout, zeroes a count and a trailing field, and sets a 16-bit field to two; returns zero.
- `init_stream_record_defaults`: Zeroes any of three optional output buffers that are supplied, stores them and a value into a record, then fills fixed default fields with sizes such as 24, 40 and 10 and clears several counters.
- `run_state_check_as_flag`: Passes a fixed state record to a routine and returns one if it succeeds, zero otherwise; when the stack is short it reports an upper-cased error instead.
- `test_and_fetch_stored_pointer`: Calls a library routine to test a condition; if it succeeds it reads a stored pointer, then releases the library call and returns the pointer, or zero when the test fails.
- `fetch_stored_value_after_test`: Runs a library test and, if it succeeds, copies a stored value into the result, then releases the library call and returns that value.
- `create_file_record_for_path`: Builds a file record from a name and a size, then uses library calls to look up a device and parent directory and set file protection, returning a pointer on success.
- `open_file_by_name`: Calls the same library open routine whether or not the second argument is zero, and returns its result; both branches are identical.
- `read_bytes_return_count`: Reads from a file through a library call and returns the number of bytes read, or zero when the call reports an error.
- `write_bytes_return_count`: Writes through a library call and returns the number of bytes written, or zero when the call reports an error.
- `close_file_handle`: Calls a library routine to close a file handle and returns whatever that routine returns.
- `seek_file_for_mode`: Seeks in a file through a library call for the start or current mode; for other modes it returns the adjusted mode value, and only seeks when that value is zero.
- `call_indirect_function`: Loads a routine pointer from an offset relative to a value passed on the stack, calls it, and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_without_action`: Does nothing: it returns immediately without changing any state or calling another routine.
- `lookup_entry_and_forward`: Looks up an entry by name and returns minus one if none is found; otherwise it flushes the entry when a flag bit is set, forwards the call with the entry's handle, and returns minus one when an error flag is set.
- `emit_string_characters`: Measures a string, then copies each character through an output routine that fills a buffer and flushes it when full, returning the length of the string.
- `format_text_output`: A printf-style formatter: it copies ordinary characters to an output routine and handles percent codes for decimal, pointer, string and hexadecimal arguments, returning the number of characters emitted.
- `parse_format_spec_and_emit`: Parses the flags, width and precision of a percent format specifier, taking a width from the argument list when it is a star, then formats the value for output.
- `emit_format_string_with_specs`: Walks a format string and passes plain characters to an output callback; a doubled percent sign emits one percent, and a single percent hands the rest of the text to the specifier parser.
- `ensure_stream_buffer_allocated`: Allocates a stream buffer when none exists or a flag asks for a fresh one; on failure it records an out-of-memory error code and returns minus one, otherwise it resets the buffer counters and returns zero.
- `write_char_to_stream_buffer`: Writes one byte into a stream buffer and decrements the remaining space, flushing through a helper when the space runs out; it sets error flags and returns minus one when the stream is in an error state.
- `read_char_from_stream_buffer`: Reads one byte from a stream buffer, refilling through a helper when it runs out; a carriage return returns the following byte, and a sentinel is returned when the stream reports an error or end.
- `flush_entries_then_finish`: Walks a table of 8-byte entries from last to first, calling a routine for each entry whose flag byte is set and whose bit four is clear, then passes the argument on to a final routine.
- `uppercase_string_in_place`: Converts the letters of a string to upper case in place, using a character-class table to decide which characters change, and returns the same pointer.
- `copy_string_zero_padded`: Copies bytes from a source up to a given count, stopping after a zero byte, then zero-fills the rest of the count, and returns the destination.
- `find_byte_in_string`: Searches a zero-terminated byte string for a given byte value and returns a pointer to its first occurrence, or zero if it is absent.
- `forward_to_helper_routine`: Passes its two arguments straight to another routine and returns nothing.
- `find_last_byte_in_string`: Searches a zero-terminated byte string and returns a pointer to the last occurrence of a given byte, or zero when it does not appear.
- `empty_return_routine`: The routine contains no instructions and simply returns without performing any operation.
- `format_number_as_digits`: Writes a value as decimal digit characters into a temporary buffer, copies them in reverse order into the destination, terminates the string and returns its length.
- `format_number_as_octal`: Writes a value as octal digit characters, taking three bits at a time into a temporary buffer, then copies them in reverse order to the destination, terminates it and returns the length.
- `format_number_as_hex`: Converts a value to hexadecimal characters one four-bit group at a time using a digit table, copies them into the destination in reverse order, terminates it and returns the length.
- `format_signed_decimal_string`: Writes a minus sign when the value is negative, converts the digits to decimal characters, copies them in reverse order after the sign, terminates the string and returns its length.
- `parse_signed_decimal_value`: Reads an optional sign and a run of decimal digits from a string, stores the signed value through a pointer, and returns the number of characters consumed.
- `append_byte_to_buffer`: Appends one byte to a buffer and increments a running count, keeping the write pointer in shared state.
- `format_into_buffer`: Resets a count and a write pointer, runs a formatter that sends each output byte to the byte-append routine, terminates the buffer and returns the count of bytes written.
- `release_block_to_free_list`: Returns a block of memory to a sorted free list, rounding its size up to a multiple of four with a minimum of eight bytes and merging it with adjacent free blocks; returns minus one if the block overlaps a free block or the size is not positive.
- `delete_file_recording_error`: Deletes a file through a library call; on failure it stores the reason code and an error number and returns minus one, otherwise it returns zero.
- `forward_call_to_named_entry`: Looks up an entry by name and, if found, passes the call on with its handle; returns minus one when the entry is missing or an error flag is set.
- `emit_byte_through_buffer`: Counts one emitted byte, then either stores it in the output buffer or, when the space is used up, passes it to a flush helper.
- `emit_byte_to_output`: Forwards its byte argument to the buffered output routine that uses the shared output state.
- `format_to_output_count`: Resets a count, runs a formatter that sends each output byte to the byte-emit routine, flushes the output and returns the count of bytes written.
- `copy_memory_block`: Copies a block of bytes from one address to another, copying backwards when the source is at or below the destination so overlapping regions are safe, and forwards otherwise.
- `free_all_list_blocks`: Walks a linked list of allocated blocks, releasing each through a system free call, then clears the list head and tail pointers and returns zero.
- `replace_tracked_allocation`: Releases the previously tracked allocation if one exists, then, when given a non-zero argument, allocates a new block and records its usable area, returning a pointer to it or zero.
- `lookup_entry_and_forward_2`: The routine forwards the first argument to another routine and, when that call succeeds, looks up a table entry by index.
- `allocate_linked_list_node`: Allocates a node large enough for the requested size plus a header, links it onto a tail-tracked list and onto the head list when that is empty, and returns a pointer past the header, or zero if the allocation fails.
- `allocate_from_free_list`: Searches a free list for a block at least as large as the rounded request, splitting off a remainder of eight bytes or more; when no block fits it requests more memory from the system and tries again, and returns zero for a non-positive size.
- `set_tracked_allocation_pointer`: When given a non-zero pointer, calls a release routine with a zero argument and records the pointer, less a four-byte header, as the tracked allocation; returns that routine's result, or zero when the pointer is null.
- `read_line_into_buffer`: Reads characters from a stream buffer into a fixed-size buffer until a newline or the size limit, terminates the text, and returns the buffer, or zero when nothing was read.
- `flush_pending_stream_writes`: Walks a chain of stream records and, for each one open for writing that has pending bytes, writes those bytes out through a write routine, then passes the argument to a final routine.
- `write_to_output_checking_errors`: Clears the stream error flag, writes through a library call, and when the write fails records the reason code and an error state, returning the write result.
- `seek_stream_with_error_record`: Seeks in a stream through a library call, recording the reason and an error state when the seek fails; a relative mode adds the returned position to the offset, and other modes use the offset adjusted by two.
- `read_from_stream_checking_errors`: Clears the stream error flag, reads through a library call, and when the read fails it records the reason code and error state five and returns minus one.
- `release_attached_resource_and_reset`: Releases a resource if one is attached, marks the state fields as unused with all-ones values, then calls two library release routines.
- `close_stream_handle`: Calls a library routine to close the stream and returns zero.
- `open_named_library_and_setup`: Opens a named system library, reporting an error if that fails, copies a stored name into a local buffer and passes it to a setup routine with sizes of 250 and 40, then calls a closing routine.
- `multiply_fixed_point_values`: Multiplies two 32-bit fixed-point values held in two registers by combining the cross terms of their high and low 16-bit halves, and returns the product in the same fixed-point format.
- `divide_signed_normalized`: Divides one 32-bit value by another, handling the signs of both operands separately and using a normalising step on the divisor; returns the division result.
- `divide_quotient_and_remainder`: Divides a 32-bit value by another 32-bit value using a normalised divisor, returning the quotient in the high 32 bits and the remainder in the low 32 bits of a 64-bit result.
- `show_message_or_requester`: Copies a message of up to 79 characters into a buffer; if an output target is attached it writes the text and a newline there, otherwise it opens a system library and passes the text to a setup routine with sizes of 250 and 60.
- `copy_text_and_show_fallback`: Copies a null-terminated string into a local buffer, writes it with a newline to an attached output when one exists, otherwise opens a system library and passes the text to a setup routine with sizes of 250 and 60.
- `open_library_and_setup`: Opens a system library and passes the stored state to a setup routine with sizes of 250 and 60; returns zero on success and minus one when the library cannot be opened.
- `run_setup_with_stored_text`: Passes a stored text pointer and fixed sizes of 250 and 60 to a setup routine, returning zero when it reports success and minus one otherwise.
- `return_zero_value`: Returns zero without doing anything else.
- `create_port_node_with_signal`: Allocates a signal and a memory node, fills the node with the given parameter and a fixed type byte, records the calling task, and either initialises an empty list or registers a port.
- `lookup_table_entry_checked`: Checks that an index lies within range and that its table slot is in use, returning a pointer to the 8-byte entry, or zero and an error code of nine when it is not.
- `check_break_signal_and_cancel`: Reads pending signal bits and, when bits twelve and thirteen are both set and a registered callback returns nonzero, clears that callback and records error code twenty.

### Nemac IV (1996)(ZenTek)(Disk 1 of 4)[HD].adf:NEMAC_4

- `restore_saved_vector_and_return`: Calls a stored routine and, if it reports a result, restores a saved word into a table slot; when a state field is set, it resets the table entry to a fixed value and writes the saved word to the alternative slot.
- `empty_return_routine_2`: The routine contains no instructions and performs no operations on any data or registers.
- `build_allocated_entry_list`: Walks a table of run-length style entries, allocating a block of four times the entry value plus eight bytes for each and linking it into a list through a running pointer; returns early when an allocation fails.
- `empty_return_routine_3`: This routine is empty, containing no instructions beyond returning to its caller.
- `copy_reversed_then_build_list`: Copies a run of bytes in reverse order into a buffer, then walks table entries allocating a block of four times each entry value plus eight bytes and linking it into a list, returning early when an allocation fails.
- `apply_relocation_table_entries`: Walks a table of offsets, adding a base value to the long word at each offset, skipping zero-terminated runs, until it reaches an end marker, then calls a finishing routine when the mode is set.
- `delay_loop_count`: Counts a 16-bit value down to minus one with no other effect, serving as a short busy-wait delay.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_main_with_message_loop`: Clears a block of shared state, derives a stack-relative size from a program table, allocates a working area when more space is needed, opens the DOS and exec libraries and runs the main message loop.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_stored_routine_with_arg`: Calls a routine through a stored pointer, passing a fixed address as its argument, then returns.
- `free_linked_list_nodes`: Walks a linked list, releasing each node through a library call, then clears the list head and returns zero.
- `return_low_byte_value`: Returns the low byte of the value currently held in the first data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_movea_rts`: Small routine of 3 instructions that merges 24 into d3, then copies (a7)+ to a5, then returns.
- `invoke_global_callback`: The routine calls a function pointer stored in a global table, passing the address of a global variable as argument.
- `invoke_global_callback_2`: Invokes a function pointer held in a global table, passing the address of a global variable as its argument, then returns.
- `call_routine_table_entries`: Walks a zero-terminated table of routine pointers, calling each one in turn until it reaches the zero entry.
- `free_linked_list_and_memory`: The routine traverses a linked list to call a function pointer within each node and then iterates through a list of memory blocks to free them using a library call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_and_print_message`: Opens a library by name and writes a message from the program text to the console, closes the library when it was opened, then calls a shutdown routine.
- `run_optional_hooks_and_release`: Stores a value on a stack, runs an optional hook when a flag is set, calls two helper routines, then calls a set of optional library routines depending on flags and returns a 64-bit pair of results.
- `tiny_move_ori`: Small routine of 2 instructions that copies a2 to d1, then merges 0 into d0.
- `dispatch_message_routines`: The routine stores a value into a stack location and decrements a counter before calling a function pointer from a global structure.
- `open_keymap_library`: Opens the keymap system library, stores the handle, and calls an error routine when the library cannot be opened.
- `open_diskfont_library`: Opens the diskfont system library, stores the handle, and calls an error routine when the library cannot be opened.
- `open_graphics_library`: Opens the graphics system library, stores the handle, and calls an error routine when the library cannot be opened.
- `open_intuition_library`: Opens the intuition system library, stores the handle, and calls an error routine when the library cannot be opened.
