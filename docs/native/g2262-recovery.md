# Game 2262 (worms - custom levels (1995)(various)[h ramkids][addon levels][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02262). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 206 of 206 named (0 author symbols), 206 of 206 described (3 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4161 of 4189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5466 |
| code | 12254 |
| data | 690100 |
| padding | 3348 |
| unused | 189952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms - Custom Levels (1995)(Various)[h RamKids][addon levels][WB].adf:lha | 3 | 4189 | 206 |

## Cases handled

- worms - custom levels (1995)(various)[h ramkids][addon levels][wb]/Worms - Custom Levels (1995)(Various)[h RamKids][addon levels][WB].adf:Worms.lha: high-entropy data file (8.00 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `open_numbered_volume_file` (lha): Builds an output archive name with a numbered volume suffix and the .lha extension, then opens it. Kind: sabotage path. Pass path: bpl.b an address.

## Functions


### Worms - Custom Levels (1995)(Various)[h RamKids][addon levels][WB].adf:lha

- `program_startup_setup`: Program entry point: allocates a work block through the library, copies the initial data image into it, opens the system libraries by name, adjusts the task priority and runs its setup helpers before returning a result.
- `program_shutdown_cleanup`: Cleanup at exit: runs a helper and the library close and free routines, the close and free calls only when a state field is set.
- `process_list_nodes_when_enabled`: When a state flag is set, walks a circular list of nodes, calls a helper with each node's fields and runs a sequence of helper calls for entries that pass a check.
- `build_path_prefix`: Copies a stored text field into the destination, using a fallback field when the first is absent.
- `append_trailing_slash_if_flagged`: Walks to the end of the path string in the buffer and appends a slash and terminator when a flag in the state record is clear; when the flag is set it calls another routine instead.
- `append_slash_to_path_end`: Walks to the terminating zero of the path buffer, then writes a slash and a new terminator after it.
- `copy_string_from_record_field`: Copies a zero-terminated string from the pointer stored in a state-record field into the destination buffer, including the terminator.
- `prompt_overwrite_with_cached_answer`: Returns a cached per-file overwrite decision when one exists; otherwise it reads a single response character and maps the answers Yes, No, Quit, Skip and All to a return value.
- `test_whitespace_character`: Returns 1 when the character in the data register is a space, tab, newline or carriage return, and 0 otherwise.
- `parse_lha_command_line`: Parses the LhA command line and the optional LHAOPTS environment variable into command letters, options and file names.
- `parse_decimal_digit`: Returns the numeric value of a decimal digit character and advances the parse position past it.
- `parse_lha_option_letters`: Reads option letters from the LhA command line and sets the matching state fields, including task priority, multivolume size and header options.
- `resolve_lha_default_options`: Fills in default option values for the command letter and the interactive state of the input, and checks the header level is between 0 and 2.
- `forward_to_helper_routine`: A thin wrapper that calls one other routine with no arguments and returns.
- `close_open_files_and_locks`: Closes every file in the file-handle table and unlocks the directory locks, restoring the current directory through dos calls.
- `open_file_into_handle_table`: Opens a file through the dos library and stores the handle in the first free slot of a 10-entry table.
- `close_file_from_handle_table`: Finds a handle in the 10-entry file table, closes it through the dos library and clears that slot.
- `lock_path_into_lock_table`: Locks a file or directory through the dos library and stores the lock in the first free slot of a 64-entry table.
- `register_lock_handle`: Stores the value in the data register in the first free slot of the 64-entry lock table.
- `store_into_free_lock_slot`: Stores the value in the data register in the first free slot of the 64-entry lock table, scanning from the index in the next register.
- `store_lock_at_index`: Stores the value in the data register into the lock table slot at the index held in another register.
- `unlock_and_clear_lock_entry`: Searches the 64-entry lock table for the matching handle, clears its slot and releases the lock through the dos library.
- `build_lha_temp_filename`: Builds a temporary file name from the LhATmp prefix followed by a string copied from the state record, then calls two helper routines to finish it.
- `allocate_file_spec_record`: Finds a free slot in the 10-entry file table and creates a file-specification record there, filling in its mode flags from the option letter.
- `allocate_file_spec_alternate`: Alternate entry into the file-spec allocation: same slot search and option-letter flag setting, using a different register for the option letter.
- `configure_file_spec_flags`: Continues file-spec setup: sets mode flags from the option letter, and opens or locks the file when the write or read options apply.
- `set_file_spec_mode_flags`: Applies the option letter to a file-spec record as mode flags, then reads the next option state.
- `initialize_file_spec_fields`: Sets the default size, position and status fields of a new file-spec record and handles the write-mode options.
- `finish_file_spec_setup`: Completes file-spec setup for an option: sets the flags and fills in the record fields that depend on whether the file already exists.
- `examine_file_for_spec`: Examines an existing file through dos.Examine when a previous record is present, then sets its mode flags and buffer pointer.
- `register_file_spec_in_table`: Sets the mode flags for a file-spec record, allocates its two buffers and stores the record in the 10-entry file table.
- `complete_file_spec_registration`: Enters the shared tail of file-spec setup: allocates a buffer, stores the buffer pointer in the record and registers the record in the file table.
- `complete_file_spec_registration_alt`: Alternate entry into the same shared tail that allocates a buffer and registers a file-spec record in the file table.
- `set_temporary_option_flags_low`: Sets the lower temporary option bits on a file-spec record and then enters the shared allocation and registration tail.
- `set_temporary_option_flags`: Sets the upper temporary option bits on a file-spec record and then enters the shared allocation and registration tail.
- `set_file_spec_flag_byte`: Sets the high bit of the record's flag byte and then enters the shared allocation and registration tail.
- `finish_file_spec_flag_byte`: Fragment of the file-spec tail: sets the high bit of the flag byte, then allocates a buffer and registers the record in the file table.
- `allocate_file_spec_buffer_tail`: Fragment of the file-spec tail: clears the buffer pointer, allocates the buffer and stores the record in the file table.
- `finish_file_spec_registration_fragment`: Fragment of the file-spec tail: allocates a handle, sets the size and position fields and stores the record in the file table.
- `reset_file_spec_and_register`: Zeroes the leading fields of a file-spec record, seeks the file when a handle is present, and stores the record in the file table.
- `close_file_spec_record`: Releases a file-spec record: deletes the file through dos when it is flagged for removal, closes its handle and clears its slot in the file table.
- `seek_buffered_file_position`: Finds the cached block that contains a file offset in a 128-entry block list.
- `open_numbered_volume_file`: Builds an output archive name with a numbered volume suffix and the .lha extension, then opens it.
- `refill_file_read_buffer`: Refills the input buffer when the read position reaches the end of the current block, allocating a buffer first if none exists.
- `refill_file_read_buffer_alt`: Alternate form of the input buffer refill: same allocate-then-read sequence and error return, using a different register for the record pointer.
- `refill_buffer_return_length`: Refills the input buffer when the record has no data left, then returns the new buffer length.
- `advance_read_offset_counts`: Adds the buffered byte count to the file offset, clears the count, and reads the next chunk from the file.
- `read_next_file_chunk`: Adds the buffered byte count to the file offset, reads a chunk through dos.Read, and returns the number of bytes read.
- `flush_file_write_buffer`: Writes the buffered output through dos.Write when the record is in write mode, and reports the dos error code on failure.
- `set_file_position_in_buffer`: Moves the logical position of a file record for a from-start or from-current seek.
- `adjust_file_position_fragment`: Fragment of the buffered position update: computes the new offset from the window start and reloads the window when the target lies outside it.
- `reposition_buffer_window`: Moves the buffer window so it contains the target offset, reloading through a helper routine when the offset falls outside the current window.
- `reposition_buffer_window_offset`: Variant of the buffer window move that takes the target offset from a different register.
- `stub_bad_instruction_data`: Decodes to a single return with no body, so the bytes are not reachable code and no behaviour is recovered.
- `reposition_buffer_window_index`: Moves the buffer window to the offset in the data register and reloads through a helper when the offset is outside it.
- `set_buffer_window_start`: Sets the buffer window start to the requested offset, clears the buffered count and calls a helper to load the window.
- `set_buffer_count_from_offset`: Sets the buffered byte count to the difference between the requested offset and the window start, then calls a helper.
- `copy_bytes_through_file_buffer`: Copies a requested number of bytes through the file buffer, reloading the window through a helper whenever it runs out.
- `copy_bytes_updating_total`: Copies counted bytes through the file buffer while updating the running total in the caller's frame.
- `copy_remaining_buffer_bytes`: Copies the bytes left in the buffer window to the destination, refilling as needed until the requested count is satisfied.
- `copy_buffer_until_empty`: Loops over the buffer window, copying bytes until the requested count is met or the window is empty, and sets a status flag when the buffer is exhausted.
- `copy_chunks_with_copymem`: Copies blocks of buffered data to the destination with exec.CopyMem, looping until the buffer window is empty.
- `copy_chunks_refilling_buffer`: Copies buffered data in blocks with exec.CopyMem and refills the window through a helper whenever it is exhausted, updating the running counts.
- `write_bytes_through_file_buffer`: Writes a requested number of bytes through the file buffer, flushing via a helper when the window fills and marking the buffer as changed.
- `write_bytes_flush_fragment`: Fragment of the buffered write loop: marks the buffer as changed, flushes through a helper when the window fills and advances the counts.
- `write_buffer_loop_continue`: Continuation of the buffered write loop: flushes the window through a helper, advances the counts and returns -1 when a helper reports an error.
- `write_blocks_with_copymem`: Writes blocks of data into the buffer window with exec.CopyMem and flushes the window through a helper as it fills.
- `write_blocks_updating_window_end`: Copies data blocks into the write buffer with exec.CopyMem, extends the window end when a write passes it, and flushes through a helper when the window is full.
- `write_byte_to_buffer`: Stores one byte in the write buffer at the current index and extends the window end.
- `write_byte_to_buffer_alt`: Alternate form of the one-byte buffered write, taking the byte from a different register.
- `write_buffer_byte_tail`: Fragment of the one-byte buffered write: stores the byte, extends the window end and flushes when the buffer is full.
- `stub_bad_instruction_data_2`: Decodes to a single return with a three-instruction body of invalid data, so no behaviour is recovered.
- `read_byte_from_buffer`: Returns the next byte from the read buffer and advances its index.
- `read_next_buffer_byte`: Returns the byte at the current read index of the buffer and advances the index, with no bounds check.
- `run_once_system_setup`: Queries a system attribute and, when a bit is set, calls two library routines and records that setup has run, so it runs only once.
- `shutdown_system_resources`: Calls a sequence of helper and library routines to release resources, and calls one more routine when a stack flag equals 3.
- `forward_to_helper_routine_alt`: A thin wrapper that calls one other routine and returns.
- `join_path_components`: Appends the second string to the first to form a path, inserting a slash unless the first string already ends with a colon or slash.
- `unlink_list_node`: Removes a node from a doubly linked list by joining its neighbours to each other.
- `find_path_separator_end`: Scans backwards from the end of a string until it reaches a colon or slash, or the start of the string.
- `duplicate_string_copy`: Measures a string, allocates a buffer of that size through a helper, and copies the string into it when the allocation succeeds.
- `allocate_memory_with_retry`: Calls the allocator and records the requested size in the block header.
- `allocate_memory_with_retry_alt`: Duplicate of the allocation-with-retry routine that also records the requested size and asks the user on failure.
- `initialize_empty_list_head`: Sets both pointers of a list head to point to itself, which makes an empty circular list.
- `insert_list_node_before`: Inserts a node into a circular doubly linked list in front of the node given in the second argument.
- `release_optional_handle`: Frees a handle through one library routine when its pointer is set, and through a default routine when it is not.
- `report_error_message`: When a flag is clear, copies a text buffer and sends it through the output library routines.
- `forward_to_cleanup_routine`: Calls a helper and then one library routine through the jump table.
- `advance_stream_read_count`: Compares the remaining size of a stream with a limit and updates the count.
- `finish_stream_write_position`: Resets the retry counter, calls a helper when bytes are pending, adds the count to the write position, and writes a zero terminator when the end is reached.
- `advance_position_and_terminate`: Adds the byte count in the data register to the stream position and writes a zero terminator at the end of the buffer when the end is reached.
- `close_stream_handle`: Closes a stream through its library handle when one is set, records the error code when the close fails, and returns the stored value.
- `write_stream_buffer_to_file`: Writes the pending output buffer to the file through dos.Write when a buffer is set, records the dos error code on failure, and returns the stored status field.
- `get_stream_status_field`: Returns the status value stored at a fixed offset in the stream record.
- `query_when_flags_clear`: Returns the result of one helper routine only when two status flags are both clear, and returns 0 otherwise.
- `call_helper_and_return`: Calls one helper routine and returns.
- `close_stream_return_status`: Closes a stream through its library handle when one is set, then returns the stored status field.
- `write_stream_buffer_status_alt`: Duplicate of the stream buffer write: writes the pending output through dos.Write when a buffer is set, records the dos error on failure and returns the status field.
- `get_stream_status_alt`: Returns the status value stored at the same fixed offset as the other status getter.
- `store_value_with_default_zero`: Reads two values from helper routines, stores the second in a 16-bit state field, and sets the first to zero when it equals -1.
- `decompress_lzss_stream`: Fills a 4 KB sliding window with spaces, then decodes an LZSS-compressed input.
- `empty_routine`: This routine contains no valid instructions and performs no operations.
- `report_stored_entry`: Passes the entry's size fields and the Stored method label to a reporting routine, and calls a second routine when a flag is set.
- `decode_next_mode_value`: Decodes the next value according to a method-mode field: mode 1 takes a byte from a four-entry ring, while modes 2 and 3 read a count from the bit reader and clamp it.
- `decode_next_mode_value_alt`: Fragment of the mode-dependent decoder that takes the mode from a different register.
- `read_next_ring_byte`: Advances a two-bit index into a four-byte ring and passes the byte at that index to a helper routine.
- `select_count_for_mode`: When the mode value is zero it uses a fixed count of 100; otherwise it reads a value from the bit reader and passes it on to a helper routine.
- `emit_fixed_count_100`: Passes the constant 100 to one helper routine and returns.
- `select_19_or_read_count`: When the mode value is zero it uses a count of 19, otherwise it reads a count and reduces it by one.
- `print_separator_line_once`: When the stored count is below 19, writes a line of underscores through dos output and sets the count to 19, so the line appears only once.
- `start_progress_report`: Sets the progress mode to 2 and calls helper routines to print a header built from three state fields.
- `store_archive_header_info`: Copies two header bytes into the archive state, sets a flag bit on the entry, and reports the archive name through a message format.
- `resolve_work_directory`: Obtains the work directory name through a library call when none is set, reports an error if none can be found, and reports a mismatch when two stored values differ.
- `execute_lha_operations`: A large routine that manages file operations, command line parsing, directory resolution, and status reporting using various text strings.
- `write_message_if_quiet`: After two helper routines, writes a message through dos output when two state flags are both clear.
- `replace_file_extension`: Copies a file name into a buffer, replacing the text after its last dot with a new extension when a dot follows the last slash, and appending the extension otherwise.
- `format_nibble_digits`: Converts a value into digit characters from a lookup table, four bits at a time, stopping at the first zero nibble, then writes a terminator.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a0), then returns.
- `parse_signed_decimal`: Reads an optional leading minus sign and then a run of decimal digits from a string, and returns the signed integer value.
- `copy_limited_string`: Copies characters from source to destination for up to the count in the data register, stopping early at the string terminator.
- `copy_bounded_string_terminated`: Copies bytes from source to destination up to the count in another register, stopping after the first zero byte is copied.
- `convert_string_case`: When a flag is set, maps each character through a case table.
- `match_wildcard_pattern`: Compares a name against a pattern in which the question mark matches any single character and the asterisk matches a run of characters, returning whether the two match.
- `match_pattern_star_run`: Handles the asterisk case of the wildcard matcher by trying each following position in the name until the rest of the pattern matches.
- `find_last_matching_character`: Scans backwards from the end of a name for a given character, calling the matcher on each candidate until one matches or the start is reached.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `stub_bad_instruction_data_3`: Decodes to a return value with no reachable body, so no behaviour is recovered.
- `spin_delay_counter`: Spins for a specified duration by counting down a 16-bit value.
- `spin_down_delay_counter`: Counts a 16-bit value down until it wraps, spinning for the requested count without any other effect.
- `stub_returns_immediately`: Returns at once without doing any work.
- `return_constant_one`: Returns the integer value one immediately.
- `resolve_parent_path_component`: Finds the last slash or colon in a path and resolves the directory part before it through library calls.
- `resolve_parent_path_alt`: Alternate form of the parent path resolver that takes the path from a different register.
- `find_parent_directory_lock`: Locates the last slash or colon in a path and then looks up the parent directory through dos.ParentDir when one exists.
- `copy_path_tail_after_separator`: Finds the last slash or colon in a path, resolves the part before it through a library call, and copies the remaining name after that result.
- `run_path_request_via_port`: Splits a path at its last separator and sends the file-name part to a handler task through exec message-port calls, then waits for the reply.
- `lock_file_into_table`: Searches a table for the first empty slot and stores a value returned by another routine into it.
- `unlock_table_entry_alt`: Finds the matching handle in the 64-entry lock table, clears its slot and releases the lock through the dos library.
- `store_lock_handle`: Searches a table for the first empty slot and stores a value from a register into it.
- `format_message_into_buffer`: Formats a message from a format string and its arguments into a buffer through the exec RawDoFmt routine.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a3)+, then clears (a3), then returns.
- `call_circular_list_init`: Calls a routine to initialize a circular linked list within a memory block.
- `free_allocated_memory_list`: Walks a linked list of allocated blocks and returns each block to the system through exec FreeMem.
- `allocate_block_with_retry_prompt`: Calls exec.AllocMem and, when the allocation fails, prints a prompt to press Return to retry or Q to quit.
- `allocate_block_retry_alt`: Duplicate of the retry-prompt allocator: calls exec.AllocMem, reports failure and offers the retry or quit choice.
- `free_memory_block`: Returns a memory block to the system through exec FreeMem, after a helper routine has run.
- `duplicate_string_buffer`: Allocates a new memory block and copies a string into it.
- `unlink_list_node_2`: Updates pointers to remove an element from a linked list.
- `insert_list_node_before_alt`: Inserts a node into a doubly linked list in front of the node given in the second argument.
- `link_node_before_target`: Links the first node into a doubly linked list immediately before the target node given in the second argument.
- `link_node_after_target`: Links the first node into a doubly linked list immediately after the target node given in the second argument.
- `initialize_circular_list`: Sets the first two words of a memory block to point to the block itself, creating a circular list head.
- `exit_and_cleanup`: Calls another routine to perform cleanup and exit.
- `send_port_request_message`: Builds an exec message with the current task's reply port, sends it with PutMsg to a handler task, and waits for the reply through WaitPort and GetMsg.
- `normalize_division_operands`: Shifts two operands right until their difference fits in the low bits, a normalisation step of software arithmetic.
- `normalize_division_operands_alt`: Duplicate of the operand normalisation step, using a different pair of registers for the two operands.
- `stub_bad_instruction_data_4`: Decodes to a return with no body, so no behaviour is recovered.
- `report_break_signal_once`: Checks for a Ctrl-C signal through exec.SetSignal.
- `output_text_string`: When a flag is clear, writes a zero-terminated text string through dos output.
- `write_string_via_output`: Writes a zero-terminated string through dos.Output and dos.Write, then calls one helper routine.
- `write_buffer_message_output`: Writes a buffered message through dos output, calling helper routines before and after.
- `print_buffer_when_not_quiet`: When the quiet flag is clear, calls a helper and then writes the message buffer through dos output.
- `format_message_string`: Calls another routine to format a message into a buffer.
- `expand_bits_to_text`: Converts the eight bits of a byte into characters, choosing each character from one of two lookup tables according to its bit, and writes a zero terminator after them.
- `expand_bits_to_text_alt`: Duplicate of the bit-to-text expansion that takes the byte and the buffer from different registers, and writes the same terminator.
- `expand_bits_to_text_loop`: Converts the bits of a byte into characters from two lookup tables, writing them one at a time in a loop and ending the buffer with a zero terminator.
- `report_when_flag_clear`: When a flag is clear, passes a fixed message to a reporting routine.
- `report_when_flag_clear_alt`: Duplicate of the flag-guarded reporting routine: passes a fixed message to the reporter only when the flag is clear.
- `print_conditional_buffer`: Checks a flag in a global structure and calls another routine if the flag is zero.
- `check_global_flag`: Checks if a value at a specific offset within a global structure is zero, and if so, calls another routine with a global variable.
- `check_task_flag_before_report`: When a flag is clear, looks up the current task through exec.FindTask and calls a helper only when a field of that task is set.
- `check_task_flag_alt`: Duplicate of the task-flag check: looks up the current task and calls a helper only when its field is set, provided the flag is clear.
- `pause_until_key_pressed`: Writes a paused prompt, reads one key from the input stream, and checks for Ctrl-C through exec.SetSignal before returning.
- `write_text_with_paging`: Writes a block of text through dos output, counting line breaks, and pauses for a key press when the page is full.
- `pause_until_key_pressed_alt`: Duplicate of the key-wait routine, entered from other callers.
- `prompt_overwrite_answer_alt`: Duplicate of the overwrite prompt: returns a cached per-file answer when one exists, otherwise reads a response and maps Yes, No, Quit, Skip and All to a return value.
- `get_default_stack_size`: Returns a default stack size of 128 kilobytes, unless the current task has a stack size set, in which case it returns that value.
- `sum_three_helper_results`: Calls three helper routines and returns the sum of their results.
- `pack_file_timestamp`: Splits a date and time into their day, month, year, hour, minute and second fields and packs them into the bit positions of a DOS timestamp word.
- `pack_dos_date_word`: Converts a day count into a month and day using a table of month lengths, then packs the year offset from 1980 and the month and day into the bit fields of a DOS date word.
- `pack_dos_date_fragment`: Continuation of the DOS date packing: subtracts month lengths from the day count and writes the month field into the packed word.
- `pack_dos_month_day_fragment`: Continuation of the DOS date packing: writes the month and day fields of the packed word from the remaining day count.
- `pack_dos_day_fragment`: Continuation of the DOS date packing: writes the day field of the packed word, using the remaining day count and the month table.
- `unpack_dos_date_to_days`: Reads the year, month and day fields from a packed DOS date word, then converts them to a day count by summing month lengths and leap days.
- `sum_month_lengths_fragment`: Adds the length of each month up to the target month to a running total, adding a leap day when the year requires one, then stores the total.
- `join_path_components_alt`: Appends a path component to a base path, inserting a slash unless the base already ends with a colon or slash.
- `append_path_component_fragment`: Appends a path component to a buffer after its last character, inserting a slash when the buffer does not already end with one.
- `find_path_separator_end_alt`: Scans backwards from the end of a path until it reaches a colon or slash, or the start of the path.
- `find_basename_pointer`: Returns the address of the file-name part of a path by scanning backwards to the last slash or colon.
- `find_basename_pointer_alt`: Index-based form of the file-name lookup: scans backwards to the last slash or colon and returns the address of the name that follows.
- `release_optional_handle_alt`: Frees a handle through a library routine when its pointer is set, and through a default routine otherwise.
- `release_library_or_default`: Calls a library function through the stored library pointer when one is set, and calls a default routine otherwise.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `copy_string_or_dispatch_library`: Copies a zero-terminated string directly into the destination when no library handle is set.
- `copy_string_via_library`: Copies a zero-terminated string through a library routine when a library pointer is set, and copies it directly otherwise.
- `copy_string_loop`: Copies a zero-terminated string from source to destination byte by byte, including the terminator.
- `exit_and_free_resources`: Calls library cleanup routines and, when a state pointer is set, calls two more and stores a fixed return code before exiting.
- `multiply_32bit_from_halves`: Multiplies two 32-bit values by forming 16-bit partial products of their high and low halves, combining the cross terms into the result.
- `divide_signed_32bit`: Divides two signed 32-bit values by negating negative operands, calling the unsigned division routine and then applying the correct sign to the quotient.
- `divide_longs_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor in 16-bit steps and returns the quotient and remainder together in one 64-bit result.
- `fill_jump_table_four`: Writes four routine entries into consecutive words of a table, used to set up a dispatch table.
- `fill_handler_table_entries`: Writes three routine entries and one value into consecutive words of a handler table.
- `return_integer_one`: Returns the integer value one to the caller.
- `find_error_message_entry`: Searches a table of 16-bit error codes for the code in the data register, and returns the index that follows the matching entry.
- `find_error_message_entry_2`: Searches a table for a matching value, prints a message from the table if found, and returns a status code or a fallback value.
- `print_table_entry_message`: Prints the message stored in a table entry through a library call and returns the entry's status code, or the fallback code for the special key combination.
