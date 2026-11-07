# Game 2267 (worms - xtra (1996)(various)[h rippa - internet][addon levels][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02267). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| asset | 3640 |
| code | 12254 |
| data | 815778 |
| padding | 2376 |
| unused | 67072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms - Xtra (1996)(Various)[h Rippa - Internet][addon levels][WB].adf:lha | 3 | 4189 | 206 |

## Cases handled

- worms - xtra (1996)(various)[h rippa - internet][addon levels][wb]/Worms - Xtra (1996)(Various)[h Rippa - Internet][addon levels][WB].adf:WRMS_XTRA.lha: high-entropy data file (8.00 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms - Xtra (1996)(Various)[h Rippa - Internet][addon levels][WB].adf:lha

- `start_lha_program_setup`: Program entry point of the LhA archiver: it opens the arp and dos libraries, allocates working memory, and sets the task priority and signal state before running the command.
- `release_lha_resources`: Restores the task priority, closes the libraries and frees the working memory before returning the result.
- `process_pending_archive_entries`: Walks a linked list of archive entries and calls a routine for each one.
- `copy_entry_name_by_mode`: Copies an entry's name into a buffer, taking either a stored name or the path depending on a mode value, and returns a status value.
- `start_path_with_separator`: Clears a path buffer and appends a directory separator, unless a stored value says the path is already built, in which case it calls another routine.
- `append_path_separator`: Moves to the end of a path string and appends a directory separator followed by a terminator.
- `copy_stored_path_string`: Copies a zero-terminated string from a field of a record into the destination buffer, including its terminator.
- `prompt_overwrite_choice`: Checks a stored overwrite flag, then unless it is set reads one-letter answers: A for always, N for no, Q for quit, S for skip and Y for yes.
- `is_whitespace_character`: Returns one when a character is a space, tab, line feed or carriage return, and zero otherwise.
- `parse_lha_command_line_options`: Parses the LhA command line and its environment options, handling quoted strings, include files and illegal command messages.
- `read_decimal_digit_value`: Reads one character and returns its numeric value when it is an ASCII digit, otherwise returns an error code of 1.
- `parse_lha_option_letter`: Dispatches on a single option character to update configuration settings.
- `set_lha_mode_flags`: Derives several option flags from a single mode letter in the state block and resets the header level to zero when it falls outside 0 to 2, reporting that with the bad header level message.
- `jump_to_shared_routine`: A single-instruction stub that forwards control to another routine with no arguments of its own.
- `release_open_files_and_locks`: Walks three fixed tables of file records and locks, closing or unlocking each active entry and deleting a flagged temporary file by name.
- `open_file_into_free_slot`: Finds the first empty entry among the ten file slots and opens a file into it through the DOS open call.
- `close_file_handle_slot`: Finds the slot holding the given file handle among the ten file slots, closes that handle through the DOS close call, and clears the slot.
- `lock_file_into_free_slot`: Finds the first empty entry among sixty-four lock slots and takes a DOS lock on a file, storing the returned lock handle there.
- `insert_into_pointer_list`: Counts non-zero entries in a list of up to sixty-four pointers and stores a value in the first available slot.
- `store_value_at_list_length`: Counts the non-zero entries in a caller-supplied list of up to sixty-four pointers and stores the value passed in another register at that index of the lock table.
- `store_value_at_lock_index`: Writes the value passed in one data register into the lock table at the index held in another data register.
- `unlock_and_clear_lock_entry`: Finds the lock table entry matching the given handle, clears it, and unlocks the file through the DOS unlock call.
- `prepare_temporary_file_name`: Copies a name string from the state block into a local buffer and calls routines around that copy.
- `allocate_file_entry_record`: Finds a free entry in the ten-slot file table and allocates a record for it.
- `scan_and_allocate_file_entry`: Scans the ten-slot file table for a free entry, allocates a record there, and fills its fields from the mode letter.
- `allocate_record_and_set_fields`: Allocates one entry record, fills its size and attribute fields from the mode letter, and adjusts a flag word based on the option letter.
- `set_entry_fields_from_mode`: Fills the size, mode and flag fields of an existing entry record from the mode letter.
- `reset_entry_and_apply_mode`: Clears the size and flag fields of an entry record, then applies the mode letter to set its flag bits and its mode value.
- `finish_entry_mode_flags`: Applies the mode letter as flag bits on an entry record and, when a required flag is clear, obtains a value through a routine call and stores it in the record.
- `release_then_finish_entry`: Releases a held file handle when one is passed in, applies the mode letter to the entry's flags, and reads file attributes through the DOS examine call.
- `apply_mode_letter_to_entry`: Runs an initial routine, then applies the mode letter to an entry's flag bits and chooses a value for a flag word.
- `fill_entry_record_fields`: Sets the first two words of an entry record from its flag bits, fetching values through routines or zeroing them, and then sets its size and offset fields.
- `fetch_entry_value_once`: Sets the first two words of an entry record from its flag bits, fetching a value through a routine without a retry.
- `set_entry_flags_then_fill`: Sets a group of flag bits on an entry record, then runs the standard field-filling sequence that fetches its values and clears its counters.
- `set_alt_flags_then_fill`: Sets a different group of flag bits on an entry record, then runs the standard field-filling sequence that fetches its values and clears its counters.
- `set_entry_hold_flag_fill`: Sets the top bit of the mode flags on an entry record, then fills its fields and size values without a second fetch.
- `set_flag_retry_fill`: Sets the top bit of the mode flags on an entry record, then fills its fields, retrying the second value fetch when the first returns null.
- `fetch_entry_value_with_retry`: Clears a field of an entry record and fetches a value through a routine, retrying once when the mode value asks for it.
- `link_entry_into_file_table`: Fetches a value for an entry record, retrying once for mode 1, and fills its size fields on success.
- `register_entry_with_position`: Clears the first field of an entry record, obtains a file position through a library call, and stores the record in the file table slot selected by the state counter.
- `release_entry_record_and_file`: Releases an entry record by closing or deleting its file when the flags require it, freeing its attached buffers, and clearing its slot in the file table.
- `find_block_containing_offset`: Searches a table of up to 128 stored address ranges for the one that contains the given offset, and moves to that range when it differs from the current one.
- `open_next_archive_volume`: Builds the file name of the next multi-volume archive part from a numbered pattern and opens it.
- `refill_entry_buffer_block`: Makes sure an entry has a buffer, then reads the next block from its file when the buffered data is used up.
- `refill_buffer_without_allocation`: Refills an entry's existing buffer when its remaining data is used up, reading the next block through a routine and updating the offset fields.
- `read_block_into_buffer`: Allocates the entry buffer when one is needed and reads the next block into it, updating the buffer position fields.
- `refill_and_read_chunk`: Refills an entry's buffer if needed, then advances its offset past the consumed data and reads the next chunk through the DOS read call.
- `read_chunk_into_entry`: Advances an entry's offset past the consumed data and reads the next chunk through the DOS read call.
- `flush_entry_buffer_to_file`: Writes the buffered data of an output entry to its file when the write flag is set, allocating the buffer when needed.
- `seek_within_entry_buffer`: Moves an entry's read position to a new offset, treating the argument as absolute or relative according to the mode value.
- `seek_entry_position_by_mode`: Moves an entry's position to a new offset, treating the argument as absolute or relative according to the mode value.
- `reposition_entry_buffer`: Sets an entry's read offset to the target value passed in.
- `reposition_entry_relative`: Adds two offset values to get a target position, then moves the entry there the same way as its neighbours, refilling the buffer when the target falls outside the current window.
- `stub_return_failure`: Returns minus one and does nothing else.
- `set_entry_offset_refill`: Sets an entry's read offset to the value passed in, refilling the buffer through a routine when that offset lies outside the current window.
- `initialize_buffer_header`: Sets a four-byte value to all ones, stores a value at a specific offset, and clears another offset before calling another routine.
- `set_buffer_used_count`: Sets the used-byte count of an entry's buffer to the offset passed in minus the window start, then calls a routine to synchronise the buffer.
- `read_bytes_from_entry`: Reads up to the requested number of bytes from an entry into a caller buffer, refilling through a routine as needed.
- `consume_entry_bytes`: Consumes a requested count of bytes from an entry's buffer, refilling when the buffer runs dry and marking the entry at end of data when none remain.
- `copy_entry_data_to_buffer`: Repeatedly copies data from an entry's buffer into a destination area, refilling when the buffer runs dry and advancing both position counters until the requested amount is copied or the data ends.
- `copy_entry_until_end`: Copies data from an entry into a destination buffer until the requested count is met or the entry runs out, setting its end-of-data flag when it reaches the end.
- `copy_entry_chunks_memory`: Copies an entry's data into memory in chunks using the system memory-copy routine, advancing its buffer window after each chunk.
- `copy_entry_blocks_memory`: Copies data out of an entry in successive blocks through the system memory-copy routine, advancing the source window and the running total each time, until a zero-length request ends the transfer.
- `fill_output_entry_bytes`: Writes a count of bytes into an output entry's buffer, flushing through a routine when the buffer fills and marking the entry as modified.
- `write_output_chunks`: Continues writing data into an output entry while bytes remain, flushing through a routine whenever the buffer fills and setting the modified flag each time.
- `flush_output_until_done`: Repeats an output step while data remains, flushing the buffer through a routine each time it fills.
- `copy_into_output_buffer`: Copies bytes from a source into an output entry's buffer using the system memory-copy routine, flushing when the buffer reaches its limit.
- `copy_bytes_to_output_entry`: Copies a block of bytes into an output entry through the system memory-copy routine, extending its recorded length and marking the entry as modified.
- `write_byte_to_output`: Stores one byte, passed in a data register, in an output entry's buffer and extends its recorded length.
- `store_byte_in_entry`: Appends one byte, passed in a data register, to an entry's buffer and updates the recorded length.
- `append_byte_flush_result`: Appends one byte to an entry's buffer and updates its recorded length.
- `return_zero`: The routine performs three instructions to clear a register and return zero.
- `read_byte_from_entry`: Returns the next byte from an entry's buffer and advances its read position.
- `read_byte_unchecked`: Returns the byte at an entry's current read position and advances the position.
- `run_setup_once_on_flag`: Tests a status word from a library call and, when a particular bit is set and a one-time marker is clear, runs two setup calls and sets the marker so they run only once.
- `run_calls_return_code`: Runs a sequence of library calls and returns a one-byte result code.
- `execute_empty_routine`: An empty routine that immediately returns.
- `append_path_component`: Appends a source string to a destination path, inserting a slash separator first unless the destination is empty or already ends with a colon or slash.
- `unlink_list_node`: Removes a node from a doubly linked list by pointing its two neighbours at each other.
- `scan_path_for_separator`: Scans a path string backwards for a directory or device separator and stops at the first one it finds.
- `duplicate_string_allocated`: Measures a string, allocates a buffer of that size through a routine, and copies the string into it.
- `allocate_block_with_quit_prompt`: Tries to allocate a memory block of the requested size and returns it with its size field set.
- `allocate_block_retry_loop`: Runs the same allocation loop as its neighbour: it tries to allocate a block, retries when a flag is set, and reads a character that can end the loop when it fails.
- `initialise_empty_list_head`: Points both links of a list header at itself, which leaves the list empty.
- `insert_list_node_after`: Links a node in after a given position in a doubly linked list, updating its own pointers and those of its neighbours.
- `dispatch_optional_library_call`: Calls one of two library routines depending on a mode flag and whether a handle is set.
- `write_terminated_buffer_line`: When a flag is clear, fills a 1 KB stack buffer through one library call, finds its terminator, and passes it to a second library call.
- `run_then_call_library`: Runs one routine and then makes a library call through the library table.
- `read_limited_chunk_with_retries`: Computes how many bytes remain, capped at a per-call limit, and reads that chunk through a routine.
- `read_chunk_and_advance`: Reads the requested chunk through a routine and advances the read position.
- `advance_position_terminate_buffer`: Adds a count to an entry's position and writes a zero terminator at the buffer's end when the position reaches its limit.
- `finish_entry_write_count`: Writes out an entry's pending data when its handle is open and records the DOS error code on failure, returning minus one.
- `write_entry_buffer_count`: Writes an entry's buffered data when a handle is open and records the DOS error code on failure, returning minus one.
- `return_entry_count`: Returns the count stored in an entry record without other work.
- `run_if_no_error`: Runs a routine only when no error flag is set and the state flag is clear.
- `dispatch_mode_logic`: Calls a routine that evaluates two values within a structure to select between a byte ring cycle, arithmetic operations, or executing function pointers from a table.
- `sync_entry_write_result`: Writes out an entry's pending data when a handle is open, recording the DOS error code and returning minus one on failure.
- `sync_output_buffer_count`: Writes an output entry's buffered data when a handle is open and records the DOS error code on failure.
- `read_entry_count_field`: Returns the count field of an entry record without other work.
- `store_status_word_result`: Calls two routines, stores the second result as a status word in the program state, and returns the first result with a minus-one value converted to zero.
- `decode_lzss_stream`: Decompresses an LZSS-style stream: it pre-fills a 4 KB ring window with spaces, then reads flag bits and either copies a literal byte or copies a back-reference from the window.
- `stub_bad_instruction_data`: The decompiled body is only bad instruction data and truncates immediately, so no behaviour is recovered from it.
- `report_stored_entry_status`: Calls routines to report the status of an entry with the message text Stored, passing its size and offset fields.
- `process_mode_logic`: Checks two values in a structure to determine whether to cycle a byte ring, perform arithmetic, or execute function pointers from a table.
- `emit_mode_value_unchecked`: Depending on a mode value, either cycles a four-entry byte ring and passes the next byte, shifted into the high word, to a routine, or passes a fixed value of 100 or a computed value.
- `emit_next_ring_byte`: Advances a four-entry byte ring, passes the new byte, shifted into the high word, to a routine, and returns zero.
- `emit_value_or_hundred`: Passes the value 100 to a routine when the argument is zero.
- `emit_fixed_hundred_value`: Passes the fixed value 100 to a routine and returns zero.
- `update_stored_limit_value`: Computes a limit from its argument, either the fixed value nineteen or a computed value minus one.
- `write_underscore_separator_line`: When the stored counter is below nineteen, writes a separator line of underscores through the DOS output and write calls, then resets the counter to nineteen.
- `initialise_run_state`: Sets a state field to two, passes four state words to a routine, and then calls several routines to continue the run.
- `record_archive_name_and_flag`: Stores two characters from its argument in the state block, sets a flag bit on an indexed entry, and passes the string argument to a routine.
- `verify_work_directory_setup`: Checks that a work directory can be found, reporting the error Unable to find work directory when it cannot and reporting a mismatch between two values.
- `process_lha_operations`: A large routine that manages archive operations, command line parsing, directory verification, and status reporting.
- `flush_console_output_if_enabled`: When two state flags are clear, it calls two DOS routines to write out the buffered console output.
- `replace_file_extension`: Scans a file name backwards to its last dot before any directory separator and overwrites the extension with the supplied suffix.
- `format_value_digit_string`: Converts a 32-bit value into a string by reading it four bits at a time from the top and looking each digit up in a table.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a0), then returns.
- `parse_signed_decimal`: Reads a signed decimal number from a string, skipping an optional leading minus sign and accumulating digits until a non-digit is reached.
- `copy_string_limited`: Copies a null-terminated string into a destination buffer, stopping after a count limit or at the terminator, whichever comes first.
- `copy_string_with_count`: Copies a string into a destination, stopping after the terminator or once the count limit is used up.
- `convert_string_case`: Converts a string in place, using a mapping table when a flag is set and otherwise lowercasing ASCII letters from A to Z.
- `match_wildcard_pattern`: Compares a text string against a pattern containing question-mark and star wildcards, returning whether they match.
- `match_pattern_star_branch`: Continues wildcard pattern matching from an inner state, handling the star wildcard by backtracking over the remaining text.
- `scan_back_for_character`: Repeatedly reads a character and scans backward through a string for it.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `return_one_status`: Returns one and does nothing else.
- `forward_to_routine_stub`: A short stub that calls another routine and returns, adding no logic of its own.
- `busy_wait_countdown`: Counts a 16-bit value down from the argument until it wraps around to its sentinel, spending CPU time in a timing loop.
- `return_from_subroutine`: Performs a return from a subroutine using a single instruction.
- `return_one_stub`: Returns one with no other work.
- `resolve_path_with_library`: When a flag is clear, checks a first routine's result and then calls a library routine to resolve a path string, which it passes on to two further routines.
- `free_path_buffer_after_check`: Returns zero when a first check fails.
- `step_to_parent_directory`: Frees a buffer through the library, then walks a path up to its parent directory with the DOS parent-directory call.
- `trim_last_path_component`: Scans a path string backwards to find its last slash, which marks where the final path component begins.
- `wait_for_port_message`: Scans a name backwards for a path separator or device colon, then waits on the current task's message port and exchanges messages through the exec library.
- `reserve_lock_slot_entry`: Finds the first empty entry among sixty-four lock slots and stores a new lock handle from the DOS lock call there.
- `clear_lock_entry_value`: Finds the lock table entry that matches the value passed in and clears it.
- `record_lock_value_first_free`: Stores the value passed in the data register in the first empty entry of the sixty-four-entry lock table.
- `format_message_via_rawdofmt`: Formats text through the exec library's formatting routine and returns zero.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a3)+, then clears (a3), then returns.
- `forward_call_without_args`: Calls one other routine and returns, with no logic of its own.
- `free_list_entries`: Walks a linked list headed in the state block and frees every entry through the exec library's memory release call.
- `allocate_memory_with_retry_prompt`: Allocates a memory block through the exec library and, when it fails, prompts for RETURN to retry or Q to quit, then acts on the reply.
- `allocate_memory_retry_prompt_alt`: A variant of its neighbour that allocates a memory block through the exec library and offers the same retry or quit prompt when the allocation fails.
- `free_memory_block`: Runs a routine on a block and then releases it through the exec library's memory release call.
- `duplicate_string_allocation`: Calculates the length of a string, allocates a new memory block, and copies the original string into the new block.
- `unlink_list_entry`: Removes an entry from a doubly linked list by pointing its neighbours at each other.
- `insert_list_entry_after`: Links an entry into a doubly linked list after a given position, updating its own pointers and those of its neighbours.
- `insert_list_entry_before`: Links an entry into a doubly linked list just before a given position, updating the pointers of the entry and its neighbours.
- `push_list_entry_front`: Inserts an entry at the front of a doubly linked list headed by the given node, updating the pointers of the entry, the old first node and the head.
- `reset_list_head_self`: Points both links of a list header at itself, leaving the list empty.
- `execute_chained_library_calls`: Invokes a library function, checks a state field to determine if a second library function should be called, and then executes a final library function.
- `send_request_wait_reply`: Builds a message record for the current task from its arguments, exchanges it with a port through the exec message calls, and waits for the reply.
- `normalise_pair_to_range`: Halves two values until their difference fits in the bottom twenty bits, then calls routines when the reduced value is non-zero.
- `normalise_registers_to_range`: Shifts two values right together until their difference fits in the bottom twenty bits, then calls two routines when the reduced value is non-zero.
- `return_zero_2`: Clears the accumulator and returns zero to the caller.
- `check_signal_bit_once`: When a flag is clear, tests a signal word from the exec library and, when a particular bit is set, writes a message through the DOS output and write calls.
- `write_string_by_mode`: Writes a string through the DOS output and write calls when a state flag is clear.
- `write_string_then_continue`: Writes a string through the DOS output and write calls, then calls one routine.
- `write_buffered_line_output`: Fills a 1 KB stack buffer through a routine and writes it through the DOS output and write calls when a state flag is clear.
- `write_buffered_line_when_enabled`: When a state flag is clear, writes a 1 KB stack buffer through the DOS output and write calls, running two routines before and after.
- `format_text_message`: Formats a text string using a library routine and returns zero.
- `expand_bits_to_table_bytes`: Expands the eight low bits of a value into eight table bytes, taking each byte from one of two tables depending on the bit, and writes a terminator byte after them.
- `expand_bits_reverse_order`: Walks the bits of a value from the high end, copying a byte from one of two tables for each bit into a buffer until the count is used up.
- `fill_bytes_from_table_run`: Copies a run of bytes from a table into a buffer one step at a time, then writes a zero terminator once the count runs out.
- `check_state_and_call`: Checks a value at an offset within a structure and calls another routine if that value is zero.
- `call_global_routine_guarded`: Uses the same guard as its neighbours: it calls one routine when a state word is zero.
- `call_routine_if_zero_state`: Calls one routine with the global address when a state word is zero, and returns otherwise.
- `check_state_flag`: Checks if a value at a specific offset in a structure is zero and calls another routine if it is.
- `run_when_task_has_entry`: When a state word is zero, looks up the current task and runs a routine if the task's entry field is non-zero.
- `check_task_entry_then_run`: Uses the same check as its neighbour: when a state word is zero, it looks up the current task and runs a routine when the task's entry field is non-zero.
- `pause_for_keypress`: Writes a message asking the user to press any key to continue, then waits for a keypress through the DOS input and read calls and clears a signal bit before returning.
- `write_text_lines`: Writes a counted buffer through the DOS output and write calls one line at a time, splitting at newline characters.
- `read_key_or_break`: Waits for a key through the DOS input and read calls and returns its code.
- `prompt_per_item_answer`: Looks up a per-item answer flag and, when it is unset, asks the user a question and records the reply.
- `choose_default_task_size`: Chooses a default of 128 KB unless the current task's entry field is set, in which case it passes that field to a further routine.
- `sum_four_routine_results`: Calls four routines in turn and returns the sum of their results.
- `pack_code_word_fields`: Gathers values from four routines and packs them into a code word through two pointers, shifting and masking the bit fields.
- `emit_code_length_field`: Sets a high-order field of a code word from a computed count, then steps through a table of stored counts until the remaining value is exceeded, updating the word's fields as it goes.
- `subtract_code_length_counts`: Repeatedly subtracts counts taken from a table from a remaining value, updating a code word's fields for each step.
- `subtract_counts_by_index`: Subtracts table counts from a remaining value by index, with the same field updates as its neighbour, and stops when the value is used up.
- `scan_length_table_entries`: Scans a table of stored counts and sets fields in a code word for each entry that fits the remaining value.
- `set_tree_offset_fields`: Sets fields of a tree entry from values gathered through two routines, adding a fixed offset and a rounded count, then loops over the bits that are set.
- `sum_table_range_total`: Sums table bytes over a range of indices, adding one extra for a particular index when a condition holds, then stores the total with an adjustment taken from a stored field.
- `append_text_to_buffer`: Appends a source string to a destination buffer, inserting a slash separator first unless the destination is empty or already ends with a colon or slash.
- `join_path_with_separator`: Joins a directory and a name by inserting a slash separator when needed, then copies the second string after it.
- `locate_path_separator_end`: Scans a path string backwards looking for a separator and stops early at a device colon.
- `return_path_basename`: Returns a pointer to the part of a path after its last slash, or to the start of the string when none is found.
- `return_name_after_separator`: Scans backward from a given index for a directory separator or colon and returns a pointer to the character after it.
- `dispatch_dos_call_or_fallback`: Calls one of two library routines depending on a state word, and otherwise calls a fallback routine.
- `call_library_or_fallback`: Calls a library routine at a fixed table offset when a handle is set, otherwise calls a fallback routine.
- `execute_wildcard_match`: Calls a routine to compare a text string against a pattern containing wildcards.
- `copy_string_or_library`: When no handle is set and an optional flag is clear, copies a string into the destination buffer byte by byte.
- `copy_string_unless_library_set`: Copies a string byte by byte when a state field is clear, and otherwise calls a library routine at a fixed table offset.
- `copy_string_bytes`: Copies a null-terminated string byte by byte, including the terminator.
- `run_library_calls_in_order`: Calls a library routine, calls a second one when a state field is set, and then runs a final library call.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values taken from the registers, combining the high and low halves of each input, and returns the 32-bit fixed-point product.
- `apply_operand_signs_result`: Calls a routine to get an unsigned magnitude and negates the result depending on whether each register argument is negative.
- `divide_unsigned_with_remainder`: Divides a 32-bit unsigned dividend by a 32-bit divisor using shifted partial steps, returning the quotient and remainder packed into a 64-bit value.
- `store_four_handler_pointers`: Writes four routine addresses into the first four words of the buffer it is given.
- `store_handler_pointer_table`: Writes routine addresses into a four-entry table, taking one entry from the stack argument.
- `return_constant_one`: Returns the integer value one.
- `lookup_keyed_table_entry`: Searches a table of keyed records for the given key and calls a library routine with the matching record's value.
- `lookup_table_entry`: Searches a table for a matching value and returns a status or error code based on the result.
- `report_table_entry_status`: Passes a table entry's message to a routine and returns the entry's status value.
