# Game 2375 (Trinity): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02375). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 350 of 350 named (42 author symbols), 350 of 350 described (53 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1684 of 1684 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 262144 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5003 of 5003 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2404 |
| code | 16612 |
| data | 296263 |
| padding | 2161 |
| unused | 583680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Trinity.adf:Trinity | 86 | 4993 | 349 |
| Trinity.adf:bootblock | 1 | 10 | 1 |
| Trinity.adf:Story.Data (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Trinity/Trinity.adf:Story.Data: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 262144 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_zmachine_story_header` (Trinity): Checks that the story version byte is four and otherwise reports a wrong-version error. Kind: version/format validation. Pass path: the branch that continues normal execution after the version check succeeds.
- `report_too_many_words` (Trinity): Continues the word split and, as its referenced text shows, reports that too many words were typed and discards the rest of the input. Kind: sabotage path / error trap. Pass path: The routine is not a check itself, but a failure handler. A successful check would bypass this call entirely or follow the instruction immediately following the call site (which would be the normal execution flow)..

## Functions


### Trinity.adf:Trinity

- `startup_build_command_and_run`: Program entry point.
- `release_saved_handles_and_return`: Calls a shared cleanup routine, releases two saved handles through library function pointers when they are non-zero, and returns the parameter together with a saved value.
- `invoke_two_function_pointers`: Calls two library functions through fixed negative offsets from the library base register, one after the other, and returns.
- `open_dos_library_or_fail`: Opens the dos library by name through a library function at a fixed offset and returns the handle when it is non-zero.
- `load_zmachine_story_header`: Checks that the story version byte is four and otherwise reports a wrong-version error.
- `check_objtab_alignment_flag`: Reads a 16-bit flags word from a structure at a fixed offset.
- `swap_halfwords_and_shift`: Swaps the two 16-bit halves of the 32-bit value in D0, then shifts the result right by 7 bits.
- `shift_result_right_nine`: Calls a routine and returns its result shifted right by nine bits, which divides the value by 512.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `return_value_via_pointer`: Returns the 16-bit value pointed to by a pointer in A0 without modifying any registers or memory.
- `store_word_big_endian`: Writes the low 16 bits of the data register to the address held in the address register, high byte first.
- `compare_limit_and_advance`: Compares its word argument with a limit from the global block and calls one of two routines depending on the result.
- `call_two_routines_in_sequence`: Calls two routines one after the other and returns, with no other work.
- `read_byte_and_advance`: Reads one byte from a buffer whose base pointer is held in a global slot, at the index stored in the global block, then advances that index by one.
- `read_word_big_endian`: Calls the byte reader twice and returns the first byte shifted left by eight bits, combined with the second byte as the low part.
- `read_operand_by_type`: Reads an operand selected by a type argument.
- `read_variable_by_number`: Returns a 16-bit variable chosen by a number.
- `store_variable_by_number`: Stores a 16-bit value into a variable chosen by a number.
- `shift_value_right_nine`: Returns its 16-bit input shifted right by nine bits.
- `extract_high_byte`: Returns the upper eight bits of its 16-bit input by shifting it right by eight bits.
- `shift_value_right_seven`: Returns its 16-bit input shifted right by seven bits.
- `store_word_by_variable_number`: Calls a routine for a number and stores the value into the destination it selects.
- `store_variable_by_index`: Stores a 16-bit value into a location determined by an index, which may target a stack offset, a table, or a specific memory structure.
- `update_variable_table`: Updates a value in a data structure based on an index.
- `decode_branch_and_jump`: Decodes a branch from one or two offset bytes, including its condition bit.
- `update_dynamic_data_structure`: Updates a complex data structure by performing bitwise arithmetic and shifting values into a buffer or table.
- `scale_input_by_constant`: Masks the input to its low sixteen bits and multiplies the result by fourteen, returning the product.
- `read_table_byte_doubled`: Calls a routine for an index, reads the byte at that index from a table whose base is held in a global slot, and returns the byte multiplied by two.
- `read_variable_length_byte`: Reads a byte from the address register.
- `mask_byte_bits`: Returns the lower six bits of a byte pointed to by a register.
- `fetch_operands_then_store`: Calls three routines to fetch its operands, then stores the result into the destination chosen by the returned number.
- `store_word_at_indexed_address`: Computes an address from a base pointer in a global slot plus two register offsets scaled by two, then writes the word there in big-endian order.
- `store_byte_at_indexed_address`: Writes one byte at the global base address plus the sum of its two register offsets.
- `scan_then_store_word`: Repeats a routine while a counter is below the low six bits of the byte at the address register, then stores the returned word.
- `scan_then_store_word_2`: Repeats a routine until a counter reaches the low six bits of the byte at the address register, then stores the returned word.
- `store_variable_indexed`: Stores a 16-bit value into a variable or a table based on a provided index and pointer.
- `store_word_indexed`: Writes a 16-bit word into a specific location determined by an index and a base address.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to -(a4), then returns.
- `increment_and_store_variable`: Calls a routine to read a value, adds one to it, and stores the result into a destination selected by a number.
- `compare_value_and_branch`: Compares a value from a routine with a second argument and calls one of two routines based on the result.
- `tokenise_text_into_entries`: Splits the input text into words at separator characters taken from a table, and for each word writes a four-byte entry holding its start position and length.
- `continue_text_tokenising`: Resumes the word split from the middle of the tokeniser loop, writing word entries and handling separator characters in the same way as the main scan.
- `handle_tokeniser_separator`: Handles a character that matches a separator during tokenising.
- `report_too_many_words`: Continues the word split and, as its referenced text shows, reports that too many words were typed and discards the rest of the input.
- `finish_word_entry_loop`: Writes the entry for the current word, then repeats the separator scan until the word limit or end of text is reached.
- `store_parsed_word_count`: Writes the current word count into the second byte of the parse header that a global slot points to.
- `read_input_line_lowercase`: Reads characters from the input routine into a buffer until a carriage return, removing the previous character on backspace and lowercasing uppercase letters, then returns the line length.
- `continue_input_line_read`: Continues line input from a first character already held in the data register.
- `read_input_chars_until_return`: Reads input characters into the line buffer until carriage return or backspace, moving the write position on backspace and stopping when the buffer is full.
- `lowercase_and_measure_input`: Converts uppercase letters in the input buffer to lowercase up to the end position, when the conversion flag is set, and returns the number of characters read.
- `compute_pointer_difference`: Returns the difference between the address register and a second address register.
- `binary_search_dictionary`: Performs a binary search over a sorted dictionary table, comparing the search word with entries and halving the range each step until the matching entry is found or the range is exhausted, then returns the entry address.
- `binary_search_dictionary_2`: Continues a binary search over a sorted dictionary table, comparing the search word with entries and narrowing the range by halves, and returns the entry address on a match.
- `binary_search_dictionary_3`: Continues a binary search over a sorted dictionary table from the current bounds, moving the lower bound forward on each comparison and returning the entry address when the word matches.
- `write_output_character`: Converts a tab character to a space, writes the character to the output buffer, and advances the buffer pointer.
- `write_decimal_number`: Converts a signed 16-bit value to decimal digits by repeated division by ten, passes the digits to an output routine in order, and writes a zero character to the buffer.
- `encode_dictionary_word`: Encodes the characters of a word as five-bit codes packed three to a 16-bit word, using the alphabet shifts for characters outside the first set, and sets the end bit on the final word.
- `select_block_for_address`: Splits a 16-bit address into a block number and an offset inside a 512-byte block.
- `return_value_to_variable`: Restores the saved frame pointer and program position from the call frame, then stores the returned word into the variable selected by the frame's store number.
- `write_newline_to_output`: Writes a carriage return to the output buffer when the output-state word is set, then calls two output routines when their flags are set.
- `store_global_word`: Stores the 16-bit argument into a fixed global cell.
- `set_output_stream_state`: Selects or deselects an output stream from its argument: zero clears the stream flag, while one sets it and saves the current position and table address returned by a routine.
- `clear_saved_long_value`: Calls a routine that returns zero and stores that 32-bit value into the saved state at a fixed frame offset, clearing it.
- `set_flag_by_argument`: When the argument is one it sets the flag word to one and clears the argument.
- `apply_stream_control_code`: Decodes a stream control code.
- `apply_stream_control_code_2`: Handles negative stream control codes: minus one clears the first flag and minus two clears the second flag and a bit in the status byte.
- `enable_output_flag`: Sets the output flag word at a fixed frame offset to one.
- `clear_output_flag`: Clears the output flag word at a fixed frame offset to zero.
- `set_second_flag_and_bit`: Sets the second flag word to one and sets the lowest bit of the status byte in the story memory header, then calls a routine.
- `clear_second_flag_and_bit`: Clears the second flag word and the lowest bit of the status byte in the story memory header, then calls a routine.
- `start_buffered_output`: Computes an output pointer from a base address plus a word offset in the argument, stores it, and sets the buffered-output flag.
- `end_buffered_output`: When the buffered-output flag is set, clears the flag and calls a routine to flush the buffer.
- `store_word_by_variable_number_2`: Calls routines for a number and stores the value into the destination it selects.
- `increment_or_forward_value`: For a non-zero argument it returns the argument plus one.
- `decrement_and_call_if_zero`: Subtracts one from the argument and calls a routine when the result is zero.
- `store_zero_based_position`: Adds a saved base offset to the first argument when the stream flag is clear, stores the first and second arguments minus one into two saved fields, and calls a routine.
- `finish_output_sequence`: Calls a sequence of output routines, calling extra ones when a stored pointer is non-zero or a flag is zero, and returns the result of the last call.
- `execute_two_calls`: Calls two separate routines in succession.
- `call_routine_with_arguments`: Saves the caller's frame and program position, pushes the given number of argument words, calls a routine, and stores its returned word into the destination selected by the call's store number.
- `advance_program_position`: Adds the argument minus two to the packed position, splits the result into a block number and an offset, and loads a new block through a routine when the block changes.
- `return_value_to_variable_2`: Restores the saved frame state from the call frame and stores the returned word into the variable selected by the frame's store number, handling the zero case as the other store routines do.
- `call_routine_and_store`: Builds a call frame on the stack, saves the program position and frame pointer, calls a routine, and stores its result into the destination selected by the store number.
- `update_variable_array`: Updates a value within a data structure or array based on an index.
- `dispatch_next_instruction`: Reads the next opcode byte and dispatches on its top bits: short, long and variable forms each decode their operands, and the low four bits select the matching operation through a switch.
- `report_bad_operation`: Runs a sequence of routines and, depending on a flag, further routines; its referenced text reads Bad operation.
- `encode_dictionary_word_2`: Processes a sequence of input values to generate a compressed or encoded representation.
- `classify_character_class`: Classifies a character into one of a few classes: zero, uppercase letters, other non-lowercase characters, and lowercase letters, which take a different path.
- `classify_character_type`: Determines if a character is an uppercase letter, a lowercase letter, or another type.
- `classify_uppercase_letter`: Returns one for an uppercase letter and two for any other character.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `find_alphabet_index`: Scans a null-terminated alphabet table for the character in the data register and, on a match, reduces an offset into the five-bit range by repeated subtraction of twenty-six.
- `reduce_to_five_bits`: Subtracts twenty-six from a value repeatedly while it is above thirty-one, reducing it into the five-bit range.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `encode_text_to_zchars`: Converts up to nine characters into five-bit codes, using the alphabet shifts for characters outside the first set, and stores them in a buffer of three-code groups for packing.
- `pack_zchars_into_words`: Pads the Z-character buffer with the padding code five, then packs groups of three five-bit codes into three bytes, setting the end bit on the last byte.
- `initialize_buffer_structure`: Initializes a seven-word structure in place: it points the first link at the structure body, stores the two arguments in two fields, clears a counter and a flag byte, and calls a routine first.
- `tiny_moveq`: Small routine of 1 instruction that loads 13 into d0.
- `buffer_character_output`: Writes a character into a buffer, converting tabs to spaces and advancing the pointer.
- `write_wrapped_output_character`: Writes a character into a line buffer and wraps the text at a word boundary when the line width is reached, flushing the buffer at carriage return.
- `reset_output_line_buffer`: Clears the column counter of the output structure, calls a routine, records the buffer start, and sets the flag byte.
- `start_output_line_buffer`: Calls a routine, copies the buffer start into the write position and sets the flag byte to one.
- `echo_input_character`: Reads a character through a routine and echoes it through an output routine when the echo flag is clear, except that backspace is not echoed while the position is zero.
- `write_wrapped_input_character`: Adds a typed character to the input line buffer, wrapping at word boundaries when the line width is reached and handling carriage return as the end of the line.
- `loop_until_address_match`: When its flag is set, repeatedly calls a routine until the address register matches the data register, then calls another routine.
- `update_bitfield_and_cache`: Updates a bitfield and a cached value within a structure, then conditionally calls another routine to update a pointer or a value based on a comparison.
- `find_cached_block`: Looks up a block number in a table of cached blocks.
- `find_least_recent_cache_slot`: Scans the cache table for the entry with the smallest use counter and returns its address, computed from the table base and a 64-byte entry size.
- `report_story_open_error`: Calls a sequence of routines to report a failure when the previous check did not succeed; its referenced text reads Story file open error.
- `report_memory_allocation_error`: When an allocation result is zero, calls a sequence of routines to report the failure; its referenced text reads Memory block allocation error.
- `report_insufficient_memory`: Calls one routine to report the condition; its referenced text says there is not enough available memory.
- `forward_word_argument`: Passes its word argument to a routine and returns.
- `forward_to_routine`: Calls one routine and returns without other work.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_saved_word_pair`: Calls a routine, then returns two saved global words combined into one result.
- `save_word_pair_and_call`: Saves its two word arguments into global cells, then calls a routine.
- `return_without_action`: Contains only a return, so it does nothing; the code is an empty routine in the unreached gap.
- `filter_input_character`: Calls another routine to retrieve a character and checks it against a specific value and a global flag before returning.
- `read_key_press`: Reads keys until a character at or above space arrives, ending on carriage return or backspace, and maps the delete key to backspace.
- `clear_input_mode_flag`: Clears the flag word at a fixed frame offset to zero, then calls a routine.
- `process_terminal_control_characters`: Handles specific keyboard input by translating carriage returns and backspaces into their corresponding terminal escape sequences or character outputs.
- `flush_when_line_full`: Compares the remaining line space, computed from two saved values, with a counter.
- `reset_and_forward_count`: Sets a value in a memory location to zero and then calls another routine.
- `flush_pending_output_range`: When the stream flag is clear it calls a routine.
- `increment_output_counter`: Calls two routines, then adds one to the counter at a fixed frame offset when the stream flag is clear.
- `return_without_action_2`: Contains only a return, so it does nothing; the code is an empty routine.
- `print_zero_terminated_text`: Passes each byte of a zero-terminated string to an output routine until the terminator, then calls another routine when the flag word is set and returns its result.
- `report_internal_error`: Finishes an output line: when the argument is non-zero it calls a routine, and it writes a carriage return and calls output routines depending on the flags.
- `branch_on_header_flag`: Tests the lowest bit of the status byte in the story memory header and calls one of two routines depending on the result, returning zero when the bit is set.
- `call_and_clear_on_flag`: Calls a routine and, when the condition flag is set, calls a second routine and returns zero.
- `pass_argument_to_routine`: Passes a 16-bit value as an argument to another routine.
- `stop_scripting_on_printer_error`: Clears the scripting bit in the story memory header and the printer flag, then calls two routines.
- `call_routine_if_nonzero`: Calls a routine and, when its result is non-zero, calls a second routine.
- `return_without_action_3`: Contains only a return, so it does nothing; it is an empty routine.
- `null_operation`: The routine is empty and performs no operations.
- `store_pending_value_on_success`: Calls a routine, and when it returns zero stores a saved global into the pending-value slot at a fixed frame offset and returns zero.
- `consume_pending_value`: Passes the pending value at a fixed frame offset to a routine, then clears that slot to zero.
- `call_other_routine`: The routine calls another routine and then returns.
- `report_story_read_error`: Calls a routine with the saved pending value and its arguments, then reports through a sequence of routines when the result is non-zero.
- `read_uppercase_letter`: Reads characters through a routine, folding lowercase letters to uppercase and skipping any that are not letters, until a carriage return or a letter arrives.
- `check_first_answer_letter`: Reads a character and scans the buffer, skipping spaces, for the first character that is not a space.
- `call_and_recover_on_zero`: Calls a routine with the arguments one and zero, and calls a second routine when that result is zero.
- `store_pending_value_on_success_2`: Calls a routine; when it returns zero it stores a saved global into the pending-value slot and returns zero, otherwise it calls a second routine.
- `return_argument_after_call`: Calls a routine and returns the argument register value.
- `handle_result_code`: Calls a routine with the pending value.
- `report_disk_full`: Calls a routine to report the condition; its referenced text reads Not enough room on disk.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `forward_pending_value`: Passes the pending value at a fixed frame offset to a routine and returns.
- `delete_named_file_3`: The routine calls another routine to delete a file named in a global variable.
- `forward_high_word`: Passes the upper sixteen bits of the data register to a routine, with the low bits cleared.
- `call_routine_with_one`: Calls a routine with the constant one.
- `report_icon_file_error`: Calls a routine and, when its result is non-zero, calls a second routine; its referenced text reads Couldn't create icon file.
- `return_empty_routine_2`: An empty routine that performs no operations and returns immediately.
- `prompt_for_save_file`: Runs a series of routines for the save-file prompt.
- `call_routine_with_zero`: Calls a routine with the constant zero, then calls a second routine.
- `invoke_system_trap`: Stores its argument in a global cell and executes the trap instruction with number one.
- `perform_device_io_command`: Fills the device request's data, offset and command fields from its two arguments and the command code three, then runs the synchronous device I/O routine on it.
- `send_device_io_command`: Sets the request command code to two and two further request fields, then sends the request asynchronously through the device send routine.
- `wait_for_message_reply`: Waits on the port, takes the next message from it, calls a routine, and returns the first byte of the message.
- `send_control_sequence`: Writes the byte 155 followed by a zero byte into a two-byte buffer, the control sequence introducer, then writes the argument and a routine result to output.
- `read_string_until_terminator`: Skips input bytes until the marker byte of minus 101 arrives, then copies the following bytes into a buffer, resetting the length on each marker, until the given terminator byte, and ends the string with zero.
- `read_bytes_until_marker`: Reads bytes through a routine into a buffer, ending it with zero, until the marker byte of minus 101 arrives; a matching terminator byte resets the buffer length.
- `store_byte_and_terminate`: Writes the byte at a fixed frame offset into the buffer at the index held in a frame slot, then writes a zero terminator after it.
- `beep_on_code_two`: When the argument is two it calls the display beep routine on a window pointer taken from a global structure; other values do nothing.
- `read_current_time`: Calls the system clock routine, which writes the seconds and microseconds into two local cells.
- `dispatch_by_mode_code`: Selects one of several calls according to a small mode number from zero to four, passing the global block to the same routine in each case and falling through the cases.
- `pass_global_block_to_routine`: Passes the address of the global block to a routine.
- `call_routine_with_global`: Calls another routine using a pointer to a global variable as an argument.
- `call_other_routine_2`: The routine calls another routine with a pointer to a global variable and then returns.
- `send_control_sequence_argument`: Writes a specific argument and a routine pointer into a buffer preceded by a two-byte control sequence introducer.
- `compute_grid_dimensions`: Computes the width and height of a grid from the window bounds minus margins, divides them by two cell sizes, stores the quotients and remainders in globals, and clamps the saved column and row limits.
- `reset_grid_cursor`: Sets the saved column limit to the cell count minus one and clears the saved row position.
- `write_cursor_position_sequence`: Converts the current row and column, each plus one, into text in two 32-byte buffers and writes them into a cursor-position escape sequence, then sends that sequence to output.
- `advance_output_line`: Moves to the next output line: when the column is before the last column it writes a carriage return and line feed, otherwise it returns the column to the saved start, clears the row, and calls routines that handle the wrap.
- `send_control_sequence_via_pointer`: Writes an argument and a routine pointer into a buffer preceded by a two-byte control sequence introducer using a pointer from data.
- `clear_column_range`: Loops over columns from the first argument up to the second, setting the column and clearing the row, and calls two routines for each column.
- `clear_grid_region`: Resets the saved column and row to zero, calls two routines, moves the column to the last cell, and calls another routine.
- `forward_with_extended_count`: Passes its pointer argument and its 16-bit count, widened to 32 bits, to a routine.
- `write_terminal_character`: Writes a carriage return through a routine, writes backspace as the three-byte sequence backspace, space, backspace, and writes any other character as a single byte.
- `wait_for_message_reply_2`: Waits for a message on a port and returns the first byte of the received message.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `display_more_prompt`: Writes a pad string and the [MORE] prompt through output routines, waits for input, and erases the prompt.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `output_global_block`: Passes the global block to an output routine on both branches, so the argument makes no difference to what is written.
- `output_global_block_2`: Passes the global block to an output routine twice.
- `trim_string_spaces`: Skips leading space characters, then removes trailing spaces by replacing them with zero terminators.
- `trim_trailing_spaces`: Removes trailing space characters from the string at the argument by replacing them with zero terminators.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `find_path_separator_end`: Searches the string for a slash and, when none is found, for a colon, and returns the position just after the match, or zero when neither exists.
- `split_directory_and_file_name`: Looks for a path separator in the global text.
- `forward_saved_pointer`: Passes the pointer saved in the frame to a routine.
- `read_file_block_at_offset`: Seeks to the requested offset when the position differs from the last one, reads the requested byte count into the buffer, and records the new position and the status or I/O error code.
- `write_file_block_at_offset`: Seeks to the requested offset, writes the given byte count from the buffer, and records the I/O error code when the seek or write fails.
- `write_buffer_to_file`: Writes the buffer to the open file through the write routine, stores the byte count it returns, and records the I/O error code when the count is minus one or differs from the requested count.
- `compare_memory_values`: Compares two values at specific offsets from a pointer and returns zero if they match, or a non-zero value otherwise.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_new_file`: Opens the named file in new-file mode and records the handle, reading the I/O error code when the open fails.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `open_existing_file`: Opens the named file in old-file mode and records the handle, reading the I/O error code when the open fails.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `close_file_handle`: Closes the file handle passed as the argument.
- `delete_named_file`: Deletes the file named in the global text; the result is not acted on.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `test_file_write_lock`: Tries to lock the named file for writing.
- `set_flag_and_return_saved`: Sets the flag to one and returns the value saved in the frame.
- `open_story_data_file`: Opens the story data file by name in old-file mode, recording the handle and reading the I/O error code on failure.
- `close_resource`: Calls a routine to close a resource and returns zero.
- `write_icon_file`: Sets two icon position fields to a sentinel value and writes the disk object through the icon library.
- `write_icon_file_2`: Sets two icon position fields to a sentinel value and writes the disk object through the icon library, returning the I/O error code or zero on success.
- `return_constant_zero`: Stores zero into a stack-frame slot and returns that same value.
- `write_with_elapsed_time_check`: Writes a buffer through the file write call and reads the system time before and after.
- `open_or_close_printer_handle`: With a zero argument it closes the stored handle if one is set and returns zero.
- `set_console_reset_state`: A zero argument clears the second flag and closes the first stage.
- `write_buffer_then_escape_sequence`: Writes the given buffer when the length is positive, and if that write returns zero, or the length is not positive, sends a two-byte escape sequence of escape followed by an uppercase E. It returns the result of the final write.
- `format_memory_status_report`: Reads the free public memory size and clamps it against a stored limit, then builds a status line in a 64-byte buffer using the referenced labels for maximum, bytes free and bytes used.
- `allocate_remembered_memory`: Allocates a block of the requested size and records it in the shared remember list so that all such blocks can be freed in one call.
- `open_custom_screen_and_window`: With a zero argument it closes the window and then the screen if they are open.
- `setup_console_resources_and_icons`: Opens the libraries named in the referenced text and the console device, temporarily changes one preference byte and restores it afterwards, creates a message port with a signal, copies a 48-byte block into a global, and loads the disk icon for a referenced name.
- `release_console_and_icon_resources`: Reverses the setup routine by releasing, in order, the disk icon, the icon library, the remembered allocations, the message port, the signal, the console device, and the library opened at setup.
- `handle_f_prefixed_path_argument`: Resets a stored value to minus one, then if the first argument is at least two and the second points to a string starting with the letter F in either case followed by a slash, it passes the rest of that string to another routine and resets the value on failure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `stub_bad_instruction_data`: Only bad instruction data was decoded as code here, so the routine has no meaningful behaviour.
- `_GetDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_PutDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `_FreeDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_AvailMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_GetMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `_WaitPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `_CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_SendIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_CloseScreen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_CurrentTime`: Calls the system library function that returns the current time, reached through the library base stored in a global variable.
- `_DisplayBeep`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_GetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `_OpenScreen`: Calls the system library function that opens a screen, reached through the library base stored in a global variable.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_SetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `_AllocRemember`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `_FreeRemember`: Calls the system library function that frees a remember list of allocations, reached through the library base stored in a global variable.
- `_Open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `_Close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `_Read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `_Output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `_Seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `_DeleteFile`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `_Rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `_Lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `_UnLock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `_IoErr`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `copy_null_terminated_string`: Copies a zero-terminated byte string from the second argument into the first, byte by byte including the terminator, and returns the destination pointer.
- `copy_string_with_length_limit`: Copies up to the given count of bytes from the source string into the destination, stopping early at a zero byte.
- `nullify_and_fetch_pointer`: Sets a longword at a specific offset within a structure to zero and returns a longword from a different offset within that same structure.
- `append_null_terminated_string`: Finds the end of the destination string by calling a routine that returns its length, then copies the zero-terminated source string onto that end, byte by byte including its terminator.
- `append_string_with_length_limit`: Finds the end of the destination string by calling a routine that returns its length, then copies up to the given count of bytes from the source, stopping early at a zero byte.
- `zero_pointer_and_return`: The routine stores zero at a location pointed to by an offset from a register and returns a value from another offset in that same register.
- `count_string_bytes`: Counts the bytes from a given pointer forward until a zero byte is reached, and returns that count without including the terminator.
- `format_integer_as_decimal`: Writes the given integer as a zero-terminated decimal string into the destination buffer, limited by the buffer size argument, and returns the number of characters written.
- `parse_signed_decimal_string`: Reads an optional plus or minus sign followed by decimal digits, using a character-class table to recognise digits and accumulating the value.
- `find_first_character_in_string`: Walks a zero-terminated string and returns a pointer to the first byte equal to the given character, or a null pointer if none is found before the terminator.
- `find_next_delimiter_match`: Advances a read position held in the caller's frame through a zero-terminated string and returns a pointer to the next byte matching the delimiter stored in the same frame, or a null pointer when the string ends.
- `forward_arguments_to_inner_routine`: Passes its two arguments unchanged to another routine and returns nothing.
- `find_last_character_in_string`: Scans a zero-terminated string and returns a pointer to the last byte equal to the given character, or a null pointer if no byte matches.
- `return_frame_local_value`: Reads a long value from the stack frame and returns it without writing anything.
- `initialise_global_table_slots`: Calls a routine twenty times and passes each result, together with an address into a global table, to another routine.
- `allocate_and_init_block`: Allocates a memory block of the requested size plus twelve bytes, stores that size in the block header, calls an initialization routine with a global and the block address, and returns the block address plus twelve, or zero if the allocation fails.
- `allocate_block_or_fail`: Calls the block allocator with the given size and returns its result, or minus one when that result is zero.
- `return_minus_one`: Returns the all-bits-set value, which is minus one, without any other work.
- `free_linked_memory_blocks`: Walks a linked list whose head is held in a global variable.
- `append_node_to_list`: Inserts the second node at the tail of a list whose head and tail pointers sit in the first structure.
- `split_arguments_and_open_console`: Splits the NUL-terminated argument string into at most 32 whitespace-separated words and stores a pointer to each in a global table.
- `divide_unsigned_longwords`: Divides the first unsigned 32-bit value by the second using a bit-by-bit shift and subtract loop, and returns the quotient.
- `empty_stub_function`: The routine contains a block of 31 68000 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `find_free_record_slot`: Scans a table of 24-byte records for the first one whose flag byte is clear, stopping at the end of the table.
- `forward_record_when_not_end`: Returns zero when the record pointer taken from its stack frame equals the end of the record table.
- `forward_record_arguments`: Passes three arguments taken from its stack frame to another routine and returns nothing.
- `open_file_with_mode_string`: Closes any file already attached to the record, then parses a file mode string whose first letter selects read, write or append, whose second letter selects an extra mode flag, and whose plus sign selects update.
- `open_file_from_frame_arguments`: Opens a file using the mode letter and plus sign found in its stack frame, following the same logic as the sibling open routine.
- `open_file_for_appending`: Opens the named file for appending with the flags it was given, seeks to the end of the file, and fills in the record stored in its frame.
- `seek_frame_handle_to_end`: Moves the file position of the handle held in the stack frame to the end of the file, then fills in the record stored in the frame and returns it.
- `finalise_file_record_fields`: Sets a mode value of two in its frame, fills the record's buffer pointers and mode byte, and applies an extra flag bit when a table entry is set.
- `open_file_with_access_flags`: Opens the file using the access flag held in its stack frame, and if the open fails returns a null pointer.
- `open_named_file_and_record`: Calls the open routine with the file name and mode values from the stack frame and returns a null pointer if that fails.
- `fill_record_with_mode`: Sets a mode byte in the stack frame from a flag, then fills the record's buffer pointers and mode byte and returns the record.
- `fill_record_with_fixed_mode`: Sets a fixed mode value of one in the stack frame, then fills the record's buffer pointers and mode byte and returns the record.
- `open_file_for_writing_record`: Opens the named file for writing with create or replace behaviour and the protection values in the code.
- `open_file_for_read_write`: Opens the named file for combined read and write access with create behaviour, then stores the handle and a mode byte in the record.
- `fill_record_mode_without_open`: Sets the record's mode byte to a read-write or write value depending on a flag, fills the buffer pointers and returns the record.
- `fill_record_fixed_mode_two`: Sets a fixed mode value of two in the stack frame, then fills the record's buffer pointers and mode byte and returns the record.
- `return_constant_zero_result`: Returns zero without any other effect.
- `fill_record_fields_from_frame`: Fills the record's buffer pointers and mode byte from values in its stack frame, applying a flag bit when a table entry is set, and returns the record.
- `reset_record_fields_and_return`: Clears a link field of the record, sets its buffer pointers and mode byte from the stack frame, and returns the record.
- `clear_object_links`: Releases the record's contents: if one flag bit is set it calls a routine with the record, and if the masked flags are clear and a second stored value is present it calls another routine with the two stored fields.
- `lookup_active_file_slot`: Checks that the index argument lies within the slot table and that the slot's flag byte has its top bit set.
- `allocate_file_slot_and_open`: Finds a free entry in the slot table and sets its flag byte from the requested access mode and option bits.
- `open_file_into_frame_slot`: Works like the slot allocator but takes the slot index from its stack frame, failing with a no-free-slot error if that index is past the table end.
- `open_file_into_slot_flags`: Sets the slot's flags from the mode and option bits in its stack frame, optionally deletes the file, opens it, and stores the handle in the slot.
- `open_file_slot_after_flags`: Marks the slot active and sets its access flags from the mode bits in the stack frame, deletes the file if requested, opens it and stores the handle.
- `open_file_write_access_slot`: Sets the write-type access flag on the slot, with the optional bit from the stack frame, deletes the file when requested, opens it with create or replace behaviour, and stores the handle.
- `open_file_write_flag_slot`: Sets the write-type access flag on the slot, deletes the file when a delete option is present, opens it, and stores the handle.
- `open_file_update_access_slot`: Sets the combined read and write access flags on the slot, with the optional bit from the stack frame, deletes the file when requested, opens it and stores the handle.
- `open_file_update_flag_slot`: Sets the combined read and write flags on the slot, deletes the file if requested, opens it with the mode from the stack frame and stores the handle.
- `reject_invalid_slot_argument`: Clears the slot's flag byte, records an invalid-argument error code and returns minus one.
- `delete_and_open_slot_file`: Deletes the file when the option bit is set, opens it using the mode derived from the flags, and stores the handle in the slot.
- `open_existing_file_into_slot`: Opens the named file for reading in the fixed existing-file mode and stores the handle in the slot.
- `return_frame_index_value`: Returns the long value from the stack frame unchanged, without any other effect.
- `split_open_flags_and_forward`: Passes the first argument with a combined set of open mode bits and the high flag bit of the second argument, and passes the second argument with that high flag bit cleared to another routine.
- `read_bytes_from_file`: The routine obtains a file handle from another routine and, if valid, reads a specified number of bytes into a buffer, recording an error in global variables if the read fails.
- `read_bytes_into_frame_buffer`: Reads the number of bytes given in its stack frame into the buffer, using the file handle stored in the frame.
- `write_to_handle_with_error_check`: Obtains a handle from another routine and, if the handle is valid and a flag in the handle is set, it calls another routine.
- `seek_file_with_offset_table`: The routine obtains a file handle from another routine and, if valid and the index is less than three, seeks the file to a position taken from a global table using a second argument, records an I/O error if the seek fails, then seeks the file back to the start and returns the result.
- `close_file_slot`: Looks up the slot and returns minus one if it is missing.
- `delete_named_file_2`: Deletes the file with the given name.
- `return_zero_status_stub`: Returns zero with no other effect.
- `open_global_file_handle`: Opens the file named by a global string in the existing-file mode and stores the handle in a global variable.
- `read_one_byte_with_pushback`: Returns a byte previously held in a global variable, clearing that holding.
- `read_one_byte_from_handle`: Reads a single byte from the open file handle held in a global variable into a stack slot, and returns that byte.
- `write_one_byte_to_handle`: Makes sure the file handle is available, writes the single byte argument to the open file, and returns that byte rather than the write result.
- `forward_two_arguments_onward`: Passes its two arguments unchanged to another routine and returns nothing.
- `read_next_buffered_byte`: Returns the next byte from a buffered stream and refills the buffer through another routine when it is empty.
- `read_next_byte_from_frame`: Works like its sibling buffered reader but takes the stream from its stack frame.
- `read_next_byte_with_refill`: Refills the stream's buffer through another routine, using the buffer size stored in the stream unless an unbuffered flag is set, then returns the next byte.
- `refill_then_read_next_byte`: Returns minus one if a status bit is set.
- `refill_and_read_stream_byte`: Marks the stream as buffered, refills it through another routine using the stored size, and records error flags for a failed or short read.
- `read_byte_from_buffer`: Takes the next byte from the stream's buffer and advances the position.
- `forward_stream_argument_call`: Passes the stream pointer from its stack frame to another routine and returns nothing.
- `write_byte_to_buffered_stream`: Writes one byte into a buffered output stream, first flushing the pending buffer through another routine when the buffer is full.
- `allocate_stream_buffer`: Allocates a buffer of the size stored in a global variable and records it as the stream's buffer pointer, clearing the status bits and counters.
- `shift_global_value_left_three`: Loads a global variable, shifts it left by three bits so it is multiplied by eight, and returns the result.
- `forward_value_twice`: Passes its 32-bit argument to another routine twice, once as each of two arguments, and returns nothing.
- `allocate_from_free_list`: Searches a linked list of free memory blocks for the first block large enough for the requested size.
- `forward_with_repeated_second_argument`: Passes its first argument, and its second argument again as a third, to another routine and returns nothing.
- `return_block_to_free_list`: Inserts a block of the given size into the sorted free list, using its computed address to find the position.
- `check_break_signal_and_report`: Reads the break signal bits and returns zero if neither relevant bit is set.
- `divide_signed_longwords`: Divides the first signed 32-bit value by the second using a restoring shift and subtract loop on their magnitudes, giving the quotient the sign of the xor of the operands.

### Trinity.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
