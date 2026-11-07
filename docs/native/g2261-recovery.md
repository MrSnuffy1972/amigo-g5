# Game 2261 (worms - custom levels (1995)(various)[h ramkids][a][addon levels][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02261). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 206 of 206 named (0 author symbols), 206 of 206 described (3 by rule, 33 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4161 of 4189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5466 |
| code | 12254 |
| data | 858521 |
| padding | 3375 |
| unused | 21504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Worms - Custom Levels (1995)(Various)[h RamKids][a][addon levels][WB].adf:lha | 3 | 4189 | 206 |

## Cases handled

- worms - custom levels (1995)(various)[h ramkids][a][addon levels][wb]/Worms - Custom Levels (1995)(Various)[h RamKids][a][addon levels][WB].adf:Worms.lha: high-entropy data file (8.00 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Worms - Custom Levels (1995)(Various)[h RamKids][a][addon levels][WB].adf:lha

- `initialise_program_and_run`: Program entry point: it opens the arp and dos libraries, allocates the working state block, sets the task priority and signal mask, then calls two routines to run the LhA program.
- `shutdown_program_resources`: Restores the task priority, closes the opened libraries and frees the allocated state memory through the exec library before returning.
- `process_entry_list`: When a state word is non-zero, walks a circular list of entries and calls routines for each entry's fields until the list returns to its head.
- `copy_entry_name_into_buffer`: When the mode is zero or three and the entry has a name or the name option is clear, copies the entry's name string into the destination buffer, then calls a routine.
- `clear_buffer_append_slash`: Clears a buffer, then appends a trailing slash and a terminator when a state field is zero.
- `append_trailing_slash_always`: Appends a slash and a terminator to the end of a string.
- `copy_state_string_to_buffer`: Copies the null-terminated string pointed to by a state field into the destination buffer, including its terminator.
- `read_confirmation_response`: Returns a stored yes or no decision from a state flag when one is already set.
- `test_whitespace_character`: Returns 1 when the character in the data register is a space, tab, line feed or carriage return, and 0 for any other character.
- `parse_lha_command_line`: Splits the command-line argument string into words and quoted strings, recognises option switches and at-sign include files, and reads the LHAOPTS environment variable when it is set.
- `parse_decimal_digit`: Returns the value of a decimal digit character and advances the caller's string position by one.
- `parse_lha_option_letter`: Dispatches on one option letter, storing values such as the multivolume size and task priority and parsing their numeric arguments.
- `normalize_option_defaults`: Fills in unset option values from the command letter and whether standard input is interactive.
- `forward_to_handler`: Trampoline that calls another routine through the library jump table and returns, with no other work of its own.
- `release_open_resources`: Walks three tables of open entries.
- `open_file_into_slot`: Searches a table of ten slots for an empty entry, opens a file through the library jump table and stores the returned handle in that slot.
- `close_and_clear_slot`: Finds the table slot holding the value passed in the data register, closes that handle through the library jump table and clears the slot.
- `lock_file_into_slot`: Finds a free entry in a 64-slot table, obtains a lock on a file through the library jump table and stores it there.
- `insert_into_list`: Counts leading non-zero entries in a list and stores a value into the next available table slot if the count is below a certain limit.
- `store_at_list_length_index`: Counts how many leading nonzero entries the pointed-to list holds, up to 64, and writes the register value into the table slot at that index when it is in range.
- `store_value_at_table_index`: Writes the value held in one data register into the 64-slot table at the index held in another data register, with no bounds check.
- `unlock_and_clear_slot`: Finds the 64-slot table entry matching the value in the data register, clears it, releases that lock through the library jump table and then runs a cleanup routine.
- `copy_stored_name_to_buffer`: Copies the zero-terminated name held in a state field into a local 256-byte buffer, then passes it on to two follow-up routines.
- `allocate_file_entry`: Finds a free slot in a ten-entry table, allocates a new entry record and fills its fields from the mode letter, covering write, multivolume and temporary variants.
- `allocate_file_entry_indexed`: Works like the file entry allocator but takes its free slot from a counter held in the caller's frame, then writes the new record into that slot.
- `build_file_entry_record`: Allocates and initialises an entry record from the mode letter and the caller's counter, then stores it in the ten-slot table.
- `finish_file_entry_setup`: Completes an entry record that is already allocated: sets its flags and fields from the mode letter, runs the writable-file helper where needed and stores the record in the ten-slot table.
- `set_entry_fields_for_mode`: Clears the record's first field and sets its counters and flags from the mode letter, running the lock step for write-capable modes.
- `apply_entry_mode_flags`: Checks a status result and the mode letter, setting the record's flags for the chosen mode, then records the entry in the ten-slot table.
- `examine_and_set_entry_mode`: Applies mode letter flags to an entry record after an optional release step, reads file information for the record through the library examine call, and stores the record in the ten-slot table.
- `set_mode_flags_and_store`: Applies the mode letter flags to an entry record, fills its name and memory fields through helper routines, and stores the record in the ten-slot table at the index from the caller's frame.
- `fill_entry_fields_and_store`: Fills the entry record's buffer fields from helper calls, clears its counters and stores the record in the ten-slot table.
- `fill_entry_name_and_store`: Fills the entry record's name and counter fields from helper calls and stores the record in the ten-slot table.
- `mark_entry_temporary_and_store`: Sets a group of flag bits in the entry record, fills its name and counter fields from helper calls and stores the record in the ten-slot table.
- `mark_entry_extended_and_store`: Sets a second group of flag bits in the entry record, fills its name and counter fields from helper calls and stores the record in the ten-slot table.
- `mark_entry_flag_and_store`: Sets the high flag bit in the entry record, fills its name and counter fields from helper calls and stores the record in the ten-slot table.
- `mark_entry_flag_and_fill`: Sets the high flag bit in the entry record, fills its name and counter fields through helper calls with a retry, and stores the record in the ten-slot table.
- `fill_entry_size_fields`: Clears a counter field and fills the entry's size and offset fields through helper calls.
- `retry_entry_name_fill`: Obtains a value from a helper and retries once with a second call when the first returns zero.
- `reset_entry_read_position`: Clears the entry's first field, fills its counter field from a helper that uses the library seek call, and stores the record in the ten-slot table.
- `release_entry_record`: Releases one entry record: removes its file when it was flagged for deletion, frees its name and buffer fields through helpers, and clears its slot in the ten-slot table.
- `seek_within_entry_segments`: Searches a table of up to 128 segment ranges for the one containing the requested offset, seeks to the matching position through the library jump table and returns the result, or -1 when the seek fails.
- `open_next_volume_file`: Builds the name of the next archive volume from the base name and a two-digit volume number, then opens it.
- `refill_read_buffer`: Allocates the read buffer on first use and refills it through the library read call when the current position runs past the buffered range.
- `refill_buffered_read_block`: Refills the buffer through the library read call when the buffered range is used up, allocating a new block when none is held.
- `read_block_with_error_code`: Allocates a buffer block when none is held, refills the read buffer through the library read call and returns the number of bytes available.
- `update_position_after_read`: After a successful read step, moves the buffered position counters forward and stores the new position returned by the library call.
- `read_into_buffer_block`: Moves the buffered position counters forward, reads the next block through the library read call and returns the result it stored in the entry record.
- `flush_entry_output_buffer`: Writes pending output for an entry record in write mode when it holds unsaved data, using the library write call.
- `seek_entry_position`: Repositions an entry record for an absolute or relative seek, returning -1 for an invalid mode or a failed helper.
- `reposition_entry_offset`: Repositions an entry record to the given offset through a helper, updating its buffered position fields.
- `set_entry_offset_fields`: Updates an entry record's buffered offset fields to the given position, calling a helper to refill the block when the position falls outside the current range.
- `set_entry_offset_relative`: Adds a relative displacement to the entry's current position and updates its buffered offset fields.
- `stub_return_failure`: Routine that only returns the value minus one, signalling failure to its caller, with no other work.
- `set_entry_offset_from_register`: Updates an entry record's buffered offset fields to the position held in a data register, calling a helper to refill the block when that position is outside the current range.
- `reset_entry_buffer_window`: Marks the entry's buffer window as invalid, sets its start offset to the value passed in and clears the buffered count, then calls a helper.
- `set_entry_buffer_count`: Stores the difference between the passed value and the entry's start offset as its buffered count, then calls a helper.
- `copy_from_entry_buffer`: Copies the requested number of bytes from the entry's buffer block through a copy helper, refilling the block when it runs out.
- `copy_entry_bytes_with_counter`: Copies bytes out of the entry's buffer block like the buffered copy routine, adding to the caller's running total as it goes.
- `copy_entry_bytes_until_end`: Repeatedly copies from the entry's buffer block until the requested count is satisfied or the end of the entry is reached, refilling as needed.
- `copy_entry_bytes_checked`: Copies from the entry's buffer block in steps, returning the bytes handled so far and marking the block as finished when the end of the entry is reached.
- `copy_entry_bytes_memory`: Copies bytes from the entry buffer into the target with the library memory copy call, refilling when the block is exhausted, and marks the entry as finished at the end.
- `copy_entry_bytes_block_refill`: Copies the entry's buffered bytes with the library memory copy call, refilling from the source as each block runs out.
- `copy_entry_bytes_marking_status`: Sets status bits on the entry record, then copies the requested bytes out of the buffer block, refilling as needed.
- `copy_entry_bytes_with_status`: Sets the status flag on the entry record, then copies bytes from the buffer block in steps, refilling as needed and advancing the high-water mark.
- `copy_entry_bytes_stepwise`: Copies bytes from the entry buffer block in steps, refilling when needed and recording the buffered high-water mark.
- `copy_entry_bytes_mark_flag`: Copies from the entry buffer block with the library memory copy call in steps, refilling between steps and setting the buffered flag.
- `copy_entry_bytes_tail_loop`: Finishes a copy from the entry's buffer block through the library memory copy call, setting the buffered flag and advancing the high-water mark.
- `write_byte_to_entry_buffer`: Appends one byte to the entry's output buffer, allocating the buffer on first use, advancing the write position and high-water mark, and flushing when the buffer is full.
- `append_byte_to_output_buffer`: Appends the byte in the data register to the entry's output buffer, advancing the write position and high-water mark, and flushes when the buffer is full.
- `store_byte_flush_when_full`: Stores the byte in the data register at the write position of the entry buffer, updates the high-water mark and flushes the buffer when it is full.
- `stub_return_zero`: Routine that returns zero immediately, with no other work.
- `read_byte_from_entry_buffer`: Reads the next byte from the entry's buffer after a refill check.
- `read_buffered_byte`: Returns the byte at the current read position of the entry's buffer and advances that position, with no refill check.
- `check_break_signal_once`: Polls the task signal mask through the library call and, when the break bit is set, runs two helpers and sets a state flag so that the handling happens only once.
- `read_char_checking_break`: Reads one character through two library helpers and, when it equals the break character code, runs the break signal check before returning it.
- `forward_to_helper`: Trampoline that calls another routine and returns, with no other work of its own.
- `append_path_component`: Appends a second string to the first as a path part, inserting a slash between them unless the first ends in a colon or slash.
- `unlink_list_node`: Removes a node from a doubly linked list by joining its neighbours, using the node's back and forward links.
- `scan_back_for_separator`: Scans backward through a path string looking for a directory separator, stopping at the start of the string or at a volume colon.
- `duplicate_string_copy`: Measures a zero-terminated string, allocates memory for a copy through a helper, copies the string into it and returns the new buffer, or zero when the allocation fails.
- `allocate_memory_retrying`: Requests a memory block through a helper and, when the request fails, checks the break flag and waits for the user's response.
- `allocate_with_retry`: Attempts to allocate a memory block and repeatedly retries or prompts the user based on a certain condition or character input.
- `init_empty_list_head`: Points both link fields of the node at itself, creating an empty circular list head.
- `insert_list_node_before`: Inserts a node into a doubly linked list immediately before the given node, relinking the neighbouring pointers on both sides.
- `dispatch_by_state_flag`: Calls one of two helpers depending on whether a state field is set, one of them through a different jump table offset.
- `emit_local_buffer_string`: When a state field is clear, calls a helper, measures a local 1024-byte text buffer up to its terminator, passes it through a library call and finishes with a cleanup helper.
- `call_helper_then_library`: Calls one helper and then a library routine through the jump table, returning nothing.
- `read_chunk_with_retry_limit`: Works out how many bytes remain in a window, caps that at a block size and reads the chunk through a helper.
- `read_chunk_into_window`: Reads a chunk of the requested size through a helper, advances the position and marks the buffer end with a zero byte when the window is full.
- `advance_window_position`: Advances the window position by the given count, marking the buffer end with a zero byte when the window is full.
- `close_window_handle`: Closes the window's handle: when its mode field is minus one it runs a helper, otherwise it calls the library function on the open handle.
- `flush_window_output`: Flushes the window's buffered output through the library write call when it has pending data, recording the library error code and returning -1 on failure.
- `return_window_field`: Returns the window field at a fixed offset with no other work.
- `return_helper_unless_stopped`: Returns zero when an error or stop state flag is set; otherwise it returns the result of a helper call.
- `emit_mode_dependent_value_2`: updates a rotating buffer or writes a constant or a supply value based on a mode flag in a state block.
- `finish_window_handle`: Finishes a window handle: when its mode field is minus one it runs a helper, otherwise, if a pending flag is set, it calls the library routine and records the library error code, returning -1 on failure.
- `flush_window_output_checked`: Writes out a pending window buffer through the library write call when the pending flag is set, storing the library error code and returning -1 on failure.
- `return_window_result`: Returns the stored result field of the window with no other work.
- `store_result_with_status`: Calls two helpers, stores the second result as a 16-bit value in the state block, and returns the first result with a minus one mapped to zero.
- `decompress_lzss_stream`: Decodes an LZSS compressed stream: it fills a 4096-byte ring buffer with spaces, then reads flag bits and copies either literal bytes or back-references from the ring buffer to the output until the input runs out.
- `stub_bad_instruction_data`: The bytes here do not decode to a valid instruction, so there is no real routine to describe.
- `report_stored_message`: Reads a value through a helper, then calls a formatting routine with a message containing the word Stored and the stored state values, followed by an optional helper when a flag is set.
- `emit_mode_dependent_value`: Dispatches on a mode value in the state block: mode one emits the next byte of a four-entry rotating buffer, mode two emits 100 or a supplied value, mode three emits a bounded value through helpers, and other modes call a fallback helper.
- `emit_mode_value_register`: Performs the same mode-driven value emission as the frame-based emitter, but takes its argument from a data register.
- `emit_rotating_buffer_byte`: Advances a two-bit index into a four-entry state buffer, emits the byte found there through a helper and returns zero.
- `emit_default_or_computed_value`: Emits a value through a helper: the fixed default of 100 when the data register is zero, otherwise a value computed by two helpers.
- `emit_constant_hundred`: Passes the constant 100 to a helper and returns zero.
- `update_stored_maximum`: Works out a limit of 19, or one less than a helper's value when the data register is nonzero, and when the stored 16-bit value is below that limit it calls two helpers and stores the new limit.
- `print_separator_line`: When the stored counter is below 19, prints a row of underscore characters through the library output and write calls and sets the counter to 19.
- `set_status_two_and_report`: Obtains a value from a helper, sets a state field to 2, passes four state fields to a helper and then runs four further helpers.
- `record_archive_name_fields`: Copies two leading bytes into state fields, sets a flag mask on an entry in a table and passes the remaining text on to a helper that prints an archive heading.
- `resolve_work_directory`: Resolves the work directory when none is recorded yet: it looks up the current task through the library call, reports a mismatch error when the lookup result does not match the expected value, and stores the resulting directory name.
- `initialize_program_state`: Performs complex setup involving command-line parsing, directory resolution, and status reporting while interacting with several internal helper routines.
- `write_output_if_enabled`: Calls two helpers, then sends a message through the library output and write calls only when two state fields are both clear.
- `replace_path_extension`: Scans backward from the end of the path for a dot that comes after the last directory separator.
- `format_nibble_digits`: Converts a value into text by repeatedly taking its top four bits and looking each up in a digit table, shifting left until no significant digits remain, then writes a zero terminator.
- `tiny_clr_rts`: Small routine of 2 instructions that clears (a0), then returns.
- `parse_signed_decimal`: Parses an optional leading minus sign and a run of decimal digits into a signed integer, stopping at the first non-digit character.
- `copy_string_with_limit_2`: Copies a string from a source pointer to a destination pointer until a null terminator or a certain count is reached.
- `copy_string_with_limit`: Copies the source string to the destination, returning as soon as the terminating zero is copied or the count runs out.
- `lowercase_or_map_string`: Rewrites a zero-terminated string in place.
- `match_wildcard_pattern`: Compares a pattern string with a name, treating a question mark as a single-character wildcard and an asterisk as a run of any characters.
- `match_pattern_character`: Compares one pattern character with one name character, advancing through the pattern and the name.
- `scan_name_for_character`: Scans backward through the name for a given character, calling a helper on each step.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `stub_return_one`: Routine that returns one, with no other work.
- `decrement_delay_counter`: decrements a 16-bit register value until it reaches zero.
- `count_down_delay`: Busy-wait loop that counts a 16-bit value down from the value passed in the data register, then returns.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it performs no work.
- `stub_return_constant_one`: Routine that only returns one, with no other work.
- `build_path_entry_node_checked`: Builds a list node for the base name of a path, linking it into a list and filling its fields from library lookups.
- `build_path_entry_node`: Builds a list node for the base name of a path, copying the name after the last separator, linking the node and filling its size and location fields from library lookups.
- `build_parent_directory_node`: Locks the parent directory through the library call and builds a list node for the base name of the path, linking it and filling its fields.
- `build_entry_node_value`: Builds a list node for the base name of the path, links it, fills its size and location fields from library lookups and returns the node's stored field value.
- `exchange_port_message_for_name`: Finds the base name of the path, builds a list node for it, then exchanges a message with another task through the library port calls, waiting for the reply before returning the stored field.
- `lock_and_store_pointer`: Finds a free slot in a table, locks a file using a library call, and stores the resulting pointer in that table slot.
- `clear_table_slot`: Searches a table for a matching value, clears that slot, and calls a library routine to release a lock.
- `store_value_in_free_slot`: Puts the value passed in the data register into the first empty entry of a 64-slot table.
- `format_text_via_library`: Calls a library routine that formats text through the jump table, and returns zero.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a3)+, then clears (a3), then returns.
- `store_register_to_memory`: Stores the value held in a register to a memory location pointed to by that same register.
- `free_all_list_nodes`: Walks a circular list headed in the state block and releases each node's memory through the library free call.
- `allocate_and_check_input`: Allocatively manages a linked list node and checks for a specific character input to decide whether to continue or exit.
- `allocate_memory_retrying_2`: Requests a memory block through the library allocation call and, when it fails, waits for the user's response with a retry or quit prompt.
- `free_memory_block`: Releases a memory block through the library free call, after a preparatory helper that works out the block size.
- `duplicate_string_content`: Finds the end of a source string and then copies characters from a source pointer to a destination pointer if a new memory allocation or pointer is valid.
- `unlink_list_node_2`: Updates pointers within a linked list structure to remove or redirect a node.
- `insert_list_node`: Inserts a new node into a linked list by updating the next-pointer or forward-link of the surrounding nodes.
- `insert_node_before`: Performs a pointer-based insertion of a node into a linked list structure, updating several pointers to link the new node.
- `insert_list_node_after`: Inserts a node into a doubly linked list immediately after the given node, relinking the neighbouring pointers on both sides.
- `set_register_to_self`: stores the value held in a register to a memory location pointed to by that same register, then does so for an offset or a second location.
- `execute_self_recursively`: calls itself and then invokes a library routine that performs a shutdown and exits.
- `send_port_message_and_wait`: Builds a message node for the given value, sends it to another task through the library port calls, waits for the reply and returns the stored reply field.
- `scale_operands_for_quotient`: Halves two operands repeatedly until their difference fits a fixed range, then calls two helpers to produce a result.
- `scale_operands_register_pair`: Performs the same operand reduction on the data registers directly.
- `return_zero_int`: returns a constant zero value.
- `check_signal_and_exit`: Checks a flag in a global structure and, if a specific bit is set, calls library routines to exit the program.
- `write_string_to_output`: When a state flag is clear, sends a zero-terminated string through the library output and write calls.
- `write_string_then_helper`: Writes a zero-terminated string through the library output and write calls, then calls a helper.
- `write_buffered_text_line`: Calls a helper, then writes the text in a local 1024-byte buffer through the library output and write calls when a state flag is clear.
- `format_and_clear_buffer`: Calls a library routine to format text and then clears a local buffer using a loop.
- `format_text_and_return_zero`: calls a library routine to format a string and then returns zero.
- `expand_bits_to_characters`: Expands the eight bits of a byte into a text string of eight characters, choosing each character from one of two state-table entries according to the bit, and terminates the string.
- `expand_bit_register_to_characters`: Performs the same eight-bit to character expansion as the byte expander, reading its bit mask from a data register and its characters from the state table.
- `expand_bit_runs_to_text`: Writes characters from a state table according to a bit mask, repeating the current character while the bit pattern does not change, and terminates the text with a zero byte.
- `branch_if_flag_clear`: Checks if a certain flag in a global structure is zero and calls another routine if so.
- `check_and_recurse_flag`: checks if a certain flag in a global structure is zero and, if so, calls itself recursively with a pointer to a global variable.
- `check_and_reset_buffer_flag`: Checks if a value at a certain offset in a structure is zero, then calls a routine to format and clear a buffer if so.
- `check_and_recursively_call`: checks if a value at a specific offset from a pointer is zero, and if so, calls itself passing a global variable as an argument.
- `call_helper_if_task_flagged`: When a state flag is clear, looks up the current task through the library call and calls a helper when the task's field is nonzero.
- `send_task_message_and_wait`: checks a global flag, finds a task, and sends a message to a port while waiting for a reply.
- `wait_for_key_press`: Prints a pause prompt asking for a key press, reads one key through the library input and read calls and runs the break check when the key is the break code.
- `write_lines_with_paging`: Writes a counted block of text line by line through the library output and write calls, counting newlines.
- `read_key_checking_break`: Reads one key through the library input and read calls, runs the break check when the key is the break code, and returns the key.
- `read_indexed_confirmation`: Checks a per-item confirmation byte selected by the index in the data register, returning a cached result when one is set.
- `manage_memory_with_retry`: performs memory allocation and deallocation, checks task status, and executes fixed-point division.
- `sum_three_helper_results`: Calls one helper for its side effect, then adds the results of three further helper calls and returns the total.
- `pack_dos_timestamp`: Packs a file's date and time into DOS-style bit fields of a 32-bit word.
- `pack_date_from_day_count`: Subtracts month lengths from a day count to find the month and day, then writes the year, month and day into the bit fields of the destination word.
- `pack_month_day_from_days`: Subtracts month lengths from a day count until the remainder fits the current month, then writes the month and day fields of the destination word.
- `find_month_and_pack_day`: Walks the month lengths subtracting from the day count, then writes the month and day fields of the destination word once the remainder fits the current month.
- `pack_date_fields`: subtracts month lengths from a day count and writes the resulting month and day into a bit-packed global variable.
- `unpack_dos_timestamp`: Reverses the packed DOS date and time: it extracts the minute, hour and year fields from the 32-bit word and computes a day count from the month-length table, adding a leap day where the year requires one, then stores the results.
- `sum_month_lengths_to_day`: Adds the lengths of the earlier months from a month-length table to the day count, adds a leap day for leap years and stores the resulting day number.
- `append_path_separator_2`: Checks if a path string ends with a separator and inserts a slash if it is missing, then appends a source string.
- `append_path_separator`: finds the end of a string, ensures a certain character or slash exists, and copies a source string to a destination pointer.
- `find_base_name_start_3`: scans a string backward from the end to find the position following a colon or a slash.
- `find_base_name_start`: Scans backward from the end of the path for a separator or volume colon and returns a pointer to the character after it, which is the start of the base name.
- `find_base_name_start_2`: Scans backward from the end of the path using a counter held in a data register and returns the position following the last separator.
- `select_library_call_by_flags`: Chooses between library calls using two state fields.
- `call_library_or_helper`: Calls a helper when a state field is clear, otherwise calls the library routine at the table entry held in that field.
- `execute_wildcard_match`: calls a routine that performs pattern matching using wildcards.
- `copy_string_or_library`: When a state flag is clear, copies a zero-terminated string byte by byte unless a state field selects a library routine for the copy.
- `copy_string_or_library_routine`: Copies a zero-terminated string byte by byte when a state field is clear, otherwise calls the library routine selected by that field.
- `copy_zero_terminated_string`: Copies a zero-terminated string from the source to the destination byte by byte, including the terminator.
- `shutdown_and_exit`: Runs a helper and then library shutdown calls, one of them only when a state field is set.
- `compute_fixed_point_product`: Multiplies two 32-bit values held in the data registers as fixed-point numbers.
- `divide_fixed_point_values`: Divides one 32-bit fixed-point value by another, handling the signs separately and normalising the divisor by shifts before dividing.
- `divide_with_remainder_pair`: Divides a 32-bit dividend by a 32-bit divisor and returns a 64-bit value that packs the quotient in the high part and the remainder in the low 32 bits.
- `fill_four_handler_entries`: Writes four routine addresses into consecutive words of the table it is given, filling a small dispatch table.
- `fill_handler_entries_with_stack`: Writes two routine addresses and one value taken from the caller's stack into consecutive words of the destination table.
- `return_constant_one`: Returns the integer value one.
- `lookup_error_code_entry`: Searches a table of four-word entries for the key in the data register, reports the matching entry's message through a library call and returns its status code.
- `search_error_table`: Searches through a table for a specific value and handles error reporting or status updates if a match or an end-of-list marker is found.
- `print_table_entry_message`: Prints the message stored in a table entry through a library call and returns the entry's status code, or the fallback code for the special key combination.
