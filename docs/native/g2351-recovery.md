# Game 2351 (hitchhiker's guide to the galaxy, the - release 58 (1984)(infocom)[serial 851002][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02351). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901216 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 388 of 388 named (40 author symbols), 388 of 388 described (64 by rule, 123 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 254 of 254 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5187 of 5187 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 160354 |
| code | 17164 |
| data | 31749 |
| padding | 5357 |
| unused | 686592 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[serial 851002][WB].adf:Hitchhiker | 86 | 5163 | 383 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 5 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 81C2) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hitchhiker's guide to the galaxy, the - release 58 (1984)(infocom)[serial 851002][wb]/Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[serial 851002][WB].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- `validate_story_header_version` (Hitchhiker): Reads the story file header and reports a wrong version through the matching text when the check fails. Kind: version check. Pass path: The routine checks if the Z-machine version in the story header is compatible; a pass allows execution to continue, while a fail triggers the error 'Wrong Z-machine version'..
- `call_routine_with_argument` (Hitchhiker): Pushes a word from d0 onto the stack, calls another routine, restores the word to d0 from the stack, loads a constant address into a0, and branches to another routine. Kind: version/integrity check. Pass path: The branch where in_D0w == 0 or the logic following the version comparison succeeds without triggering the error message..
- `prompt_for_save_file_name` (Hitchhiker): Shows the save prompt that includes the default name and asks for a save disk, then clears the chosen name entry or passes zero to a routine. Kind: disk/file presence check. Pass path: the else branch (sVar1 != 0).

## Functions


### Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[serial 851002][WB].adf:Hitchhiker

- `enter_program_by_launch_context`: Program entry point that branches on whether the process was started from the command line.
- `clean_up_and_reply`: Calls a helper routine, then calls two library routines at fixed offsets when two stored values are nonzero.
- `call_two_library_functions`: Calls two library routines through fixed negative offsets from a base register, then returns.
- `try_dos_library_then_cleanup`: Calls a library routine at a fixed offset and returns its result if nonzero, referring to the text dos.library.
- `set_io_parameters`: Stores two parameters and a constant into global variables before calling an I/O routine.
- `initialize_io_request`: Sets a global variable with a constant, a pointer to a global variable, and a constant before calling an I/O routine.
- `get_message_from_port`: The routine waits for a port, retrieves a message, and returns the value of a global variable.
- `exchange_two_byte_sequence`: Builds a two-byte sequence in a stack buffer and passes it through a routine with the parameter.
- `read_characters_into_buffer`: Reads input until a sentinel value arrives, then reads characters into a buffer until the terminator value.
- `trigger_conditional_beep`: Checks if a parameter equals a certain value, then triggers a beep using a value fetched from a table offset by a pointer.
- `get_current_time`: Calls a library routine to retrieve the current time and returns a value from the stack.
- `dispatch_io_by_type`: Checks a parameter and calls another routine with a pointer to a global variable for certain input values.
- `reply_pending_port_message`: Gets a message from a port held in a fixed structure, and if one arrives it replies to it.
- `compute_grid_cell_counts`: Computes how many columns and rows fit in a rectangle, after subtracting two margin values from each dimension and dividing by the cell sizes.
- `build_text_pieces_in_buffer`: Uses a 64-byte stack buffer and calls routines to build two text pieces in it, terminating each with a zero byte.
- `combine_two_text_fields`: Copies two 32-byte text fields from its arguments into stack buffers, appends them with a global text piece into one 64-byte buffer, and passes that buffer to a final routine.
- `emit_line_break_or_position`: Calls a helper, then compares two stored counters.
- `pass_global_to_routine`: Passes a global value to another routine and returns.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `emit_global_text_sequence`: Calls a routine with zero arguments, then with the global text, then with a stored count minus one as the position.
- `forward_pair_to_routine`: Forwards its two arguments unchanged to another routine and returns.
- `emit_character_with_control_codes`: Sends one character: a carriage return triggers a line-break routine, a backspace is expanded into the three bytes that erase the previous character, and any other character is written as a single byte.
- `call_no_argument_routine`: Calls one routine with no arguments and returns.
- `write_padded_text_field`: Positions the output from its arguments and a stored width, pads with spaces when the width leaves room, writes the second text piece and then the text saved at the start.
- `display_more_prompt`: Writes six-character blocks from a fixed text buffer that holds a [MORE] prompt, with a helper call between writes.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `call_global_pair_either_branch`: Calls one routine with the global value as both arguments, regardless of its argument, because both branches make the same call.
- `call_global_pair_once`: Calls a routine with the global value as both arguments and returns.
- `trim_path_after_separator`: Searches a string for a slash or colon path separator, trying the slash first, and when one is found it cuts the string just after that separator and stores the result back.
- `save_disk_object_if_loaded`: If an object pointer is set, it marks two fields with a fixed flag value and saves the object with PutDiskObject.
- `save_disk_object_unchecked`: Sets two fields of the disk object held in a global to a fixed flag value and saves it with PutDiskObject, without a null check.
- `return_zero_result`: Sets a local result to zero and returns it.
- `read_block_at_offset`: Seeks to an offset in a file unless the cached handle and position already match, then reads a block of bytes into the buffer.
- `write_block_at_offset`: Seeks to an offset in a file and writes a block of bytes from the buffer.
- `write_block_from_frame`: Writes a block described by fields of a frame record: the handle, buffer and length.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_new_file_for_writing`: Opens a file in new-file mode and stores the handle in a global.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_existing_file_handle`: Opens an existing file in old-file mode and stores the handle in a global.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `close_file_handle`: Closes the file handle it is given and returns zero.
- `delete_file_by_name`: Deletes the file named by a global string and returns zero when the delete succeeds, otherwise one.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `test_lock_on_global_name`: Tries to lock the file or directory named by a global string and releases the lock again.
- `release_stored_lock`: Releases the lock whose handle is held in a stack slot and returns one.
- `open_story_data_file`: Opens the file named by the text Story.Data in old-file mode and stores its handle in a global.
- `close_given_file_handle`: Closes the file handle passed to it and returns zero.
- `calculate_time_difference`: Updates global variables via parameters, calls an I/O routine, and calculates if a certain amount of time has passed since a previous time check.
- `open_or_close_printer_device`: With a nonzero argument it opens printer.device, allocates a signal, finds the current task and adds a message port.
- `send_escape_sequence_once`: Sends a two-byte escape sequence of 27 then 99 when a stored flag is clear and the argument is nonzero, and records success in the flag.
- `write_block_then_escape_fallback`: Writes a block of the given length when that length is positive.
- `format_memory_status_line`: Reads the available memory and builds a status line with the free amount, the used amount and a maximum marker, using the text bytes free and bytes used.
- `allocate_remembered_block`: Allocates memory of the requested size through a remember key held in a global, using the public memory flag.
- `open_window_and_resources`: Opens the libraries and device named in its text, a fixed-size window and a message port.
- `release_window_and_resources`: Releases what the opening routine acquired: it frees the disk object and remembered memory when present, removes the port, closes the device and the window, and closes the libraries.
- `echo_input_until_quit`: Reads characters and echoes each one until the letter q is read, reading and echoing a line with a carriage return whenever z arrives.
- `relay_keys_until_quit`: Echoes the character held in a stack slot and reads the next one, repeating until q arrives, with a z triggering a line read and carriage return.
- `echo_keys_with_line_read`: Echoes a stored character, then reads characters until q, reading and echoing a line whenever z arrives.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `parse_f_slash_argument`: Checks whether a string argument begins with the letter F or f followed by a slash, and if so passes the rest of the name to a routine.
- `validate_story_header_version`: Reads the story file header and reports a wrong version through the matching text when the check fails.
- `fetch_and_advance_story_page`: Reads a block of data from the story file into a buffer, updates a pointer to a new memory location offset by 512 bytes, sets bit 5 of a flag in a structure, and performs a bitwise shift on a value before calling routines to advance the page counter and dispatch the next opcode.
- `swap_halfwords_and_shift`: The routine swaps the two 16-bit halves of the 32-bit value in D0, then shifts the result right by 7 bits.
- `shift_result_right_nine_bits`: Calls a routine and returns its result shifted right by nine bits, which divides it by 512.
- `add_block_size_if_unaligned`: Adds 512 to its argument when the low nine bits are nonzero, otherwise it returns the argument unchanged.
- `return_value_via_pointer`: The routine returns the 16-bit value pointed to by a pointer in A0.
- `store_word_big_endian`: Stores its 16-bit argument at the target address with the high byte first.
- `compare_index_and_step`: Compares its argument with a stored value and calls one of two routines accordingly.
- `call_routine_twice`: Calls two routines in sequence and returns.
- `read_next_buffer_byte`: Returns the byte at the current index of a buffer and advances the index, calling a refill routine when the index passes 511.
- `read_selected_word_value`: Returns a value chosen by its argument: a non-positive argument reads two bytes and joins them, the value one reads the next buffered byte, and other values read a word from one of two tables depending on whether they are below sixteen.
- `read_word_from_selected_table`: Returns a word from a pointer when its argument is zero, from a first table for arguments below sixteen, and from a second table otherwise, with the table index taken from the argument.
- `store_word_by_selector`: Stores its 16-bit value at a pointer when its selector is zero, in a first table for selectors below sixteen, and otherwise in a second table.
- `store_word_in_table_slot`: Stores its 16-bit value in one of two tables depending on whether the selector is below sixteen, writing the high byte first in the second table.
- `store_word_in_second_table`: Stores a 16-bit value in the second table at an index derived from its selector, writing the high byte first.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (andi.w) and then falls through into the code that follows, with no return of its own.
- `process_buffer_byte`: Copies the word from D1 to the stack at the location pointed to by A4 if the byte returned by another routine in D0 is zero, otherwise it calls another routine.
- `clear_and_branch_2`: Clears the word in d1 and branches to another routine.
- `decode_variable_length_integer`: Reads a variable-length integer from a buffer by checking specific bits to determine if additional bytes are required.
- `decrement_and_branch_2`: Decrements d0 by one and branches to another routine if the result is not zero, otherwise it branches to a different routine.
- `decrement_and_branch`: Decrements a 16-bit value at offset -74 of the structure in A6 by one and branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `shift_word_right_nine`: Returns its word argument shifted right by nine bits, which divides it by 512.
- `return_high_byte_of_word`: Returns the high byte of its word argument.
- `shift_word_right_seven`: Returns its word argument shifted right by seven bits, which divides it by 128.
- `add_to_buffer`: Adds a word from D1 to D0 and branches to another routine.
- `subtract_from_buffer`: Subtracts a word from D1 from D0 and branches to another routine.
- `multiply_buffer_values`: Multiplies D0 by D1 as a word and branches to another routine.
- `divide_buffer_values`: Extends D0 to a longword, divides it by D1 as a word, and branches to another routine.
- `divide_and_swap_buffer`: Extends D0 to a longword, divides it by D1 as a word, swaps the high and low words of D0, and branches to another routine.
- `update_status_word`: Swaps the words of a 16-bit value from a global variable, performs a logical shift right on a 32-bit value, and XORs the result into another 16-bit value at a global variable offset.
- `divide_unsigned_buffer`: Masks D1 with a bitmask, performs an unsigned division of D1 by D0, swaps the words of D1, increments the low word of D1, moves the result to D0, and branches to another routine.
- `compare_and_branch_less`: Compares d1 to d0 and branches to one routine if d1 is less than d0, otherwise it branches to another routine.
- `compare_and_branch_greater`: Compares d1 to d0 and branches to one routine if d1 is greater than d0, otherwise it branches to another routine.
- `test_bit_and_branch`: Inverts the word in d0, performs a bitwise AND between d0 and d1, and branches to one routine if the result is zero, otherwise it branches to another routine.
- `or_buffer_values`: Performs a bitwise OR between D1 and D0 as a word and branches to another routine.
- `not_buffer_value`: Performs a bitwise NOT on the word in D0 and branches to another routine.
- `multiply_word_by_nine`: Returns its low word multiplied by nine.
- `read_table_byte_doubled`: Reads a byte from a table at an index supplied by a helper routine and returns it doubled.
- `increment_shifted_byte`: Loads a byte from the address in A0, shifts it right by 5 bits, increments the result, and adds this value to the address in A0 before returning.
- `swap_byte_values`: Calls two routines, then swaps a byte from offset 6 of a pointer in a1 with a byte from offset 5 of a pointer in a0, and stores a byte from d1 into offset 4 of the pointer in a0.
- `compare_and_copy_byte`: Moves a word from D0 to D1, calls another routine, moves a pointer from A0 to A1, loads a byte from offset 4 of the pointer into D0, and if it is not zero, masks D0 and calls another routine before comparing a byte from offset 6 of the structure in A0 with D1; if they match, it moves a byte from offset 5 of the pointer to offset 6 of the structure in A0 and branches to another routine.
- `loop_and_copy_byte`: Calls another routine in a loop until the byte at offset 5 of the structure in A0 matches the byte in D1, then copies a byte from offset 5 of the structure in A1 to offset 5 of the structure in A0 and clears bytes at offsets 4 and 5 of the structure in A1.
- `call_multiply_and_check_byte`: Calls another routine, clears d0, loads a byte from offset 4 of a0 into d0, and branches to another routine.
- `push_byte_and_call`: Calls another routine, clears d0, loads a byte from offset 6 of the buffer in a0 into d0, pushes that byte onto the stack, calls another routine, and branches to one routine if the byte at the top of the stack is non-zero, otherwise it branches to another routine.
- `push_byte_and_call_offset_5`: Calls another routine, clears d0, loads a byte from offset 5 of the buffer in a0 into d0, pushes that byte onto the stack, calls another routine, and branches to one routine if the byte at the top of the stack is non-zero, otherwise it branches to another routine.
- `compare_byte_and_branch`: Calls another routine, compares the byte at offset 4 of the buffer in a0 to d1, and branches to one routine if they are equal, otherwise it branches to another routine.
- `call_two_routines`: Calls two different routines in succession and then branches to another routine.
- `compare_masked_byte`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching based on the comparison result.
- `calculate_offset_and_call`: Subtracts 1 from d1, doubles d1, loads a pointer from offset -16 of a6 into a0, adds d1 to a0, calls another routine, and branches to another routine.
- `call_two_routines_alt`: Calls two different routines in succession and then branches to another routine.
- `branch_on_masked_byte`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching to a specific address if the values are equal or if the comparison is greater than.
- `test_bit_and_move_byte`: Tests bit 5 of a byte at the address in a0 and increments a0; if bit 5 is set, it branches to another routine, otherwise it moves a byte from d2 to the address in a0 and returns.
- `branch_with_data`: Moves the word from D2 into D0 and branches to another routine.
- `call_and_test_d1`: Calls two different routines in succession, tests if d1 is zero, and branches to another routine based on that result.
- `branch_on_masked_byte_alt`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching to a specific address if the values are equal or if the comparison is greater than.
- `mask_byte_and_branch`: Calls another routine, loads a byte from a0 into d0, masks d0 with 31, and branches to another routine.
- `mask_low_three_bits`: Returns the bitwise NOT of the 16-bit value passed in D0, masked to the low three bits.
- `test_bit_in_buffer`: Calls another routine, moves the word in d1 to d0, calls another routine, tests a bit in d0 at the location pointed to by a0, and branches to one routine if the bit is set, otherwise it branches to another routine.
- `read_edited_input_line`: Reads characters until a carriage return, moving back one position on backspace and folding capitals to lower case when a flag is set.
- `read_input_line_bounded`: Stores typed characters into a buffer up to a bound held in a register, handling backspace and folding capitals to lower case when a flag is set.
- `finish_input_line_read`: Continues line reading: it stores characters, handles backspace and carriage return, and returns the number of characters stored.
- `return_pointer_difference`: Calls two routines and returns the difference between two buffer pointers, which gives the number of bytes between them.
- `binary_search_sorted_table`: Performs a binary search over a sorted table of entries read through a helper routine, moving the probe position by halving steps, and returns the result of the search.
- `continue_table_search`: Adjusts the probe position of a table search from entries read through a helper routine and returns the matching position.
- `step_back_table_search`: Steps the table search position back using helper results and returns once a match or limit is reached.
- `append_byte_to_output_buffer`: Stores its byte argument at the current position of an output buffer and advances the position.
- `emit_string_characters`: Passes each character of a zero-terminated string to a routine in turn, then returns zero.
- `draw_status_line`: Draws the status line with the labels Score and Time, choosing one of two layouts from a flag.
- `append_character_or_flush`: When a mode flag is set it appends its character to a buffer, otherwise it calls a routine and clears a state field.
- `print_decimal_number`: For a nonzero argument it negates negative values, splits the magnitude into decimal digits by repeated division by ten and writes them with a routine.
- `decode_zstring_text`: Decodes packed text: each 16-bit word yields three 5-bit characters, with handling of alphabet shifts, abbreviation codes and the end bit set on the last word.
- `decode_z_string_data`: Decodes a compressed string by iterating through bytes, performing bitwise shifts and masking to extract values, and calls decode_zstring_text_body or write_character_or_flush_pending based on the extracted bits.
- `decode_zstring_text_entry`: Enters the same packed text decoder as its neighbour, after first making three helper calls.
- `update_paged_address_base`: Splits a 17-bit address into a 512-byte page number and an offset, and when the page changes it stores the new page and computes its base address through a routine.
- `update_paged_address_base_and_branch`: Updates the paged address base, calls a routine to set a constant and branch, and then performs a relative branch to another location.
- `jump_to_tiny_moveq`: Jump stub: passes control straight on to tiny_moveq.
- `increment_and_call_if_greater`: Increments a 16-bit value in a global variable and, if the new value is greater than the original value, calls another routine with both values as arguments.
- `compare_d0_and_return`: Tests d0; if d0 is zero it branches to another routine, if d0 is one it branches to another routine, otherwise it returns.
- `clear_and_copy_values`: If a 16-bit value at a negative offset from a6 is non-zero, it clears that value and copies two 16-bit values from offsets -8 and -6 of a6 into global variables at offsets -4 and -8.
- `initialize_and_reset_globals`: If a 16-bit value at a negative offset from a6 is zero, it sets it to one and copies two 16-bit values from global variables at offsets -4 and -8 into offsets -8 and -6 of a6; then it clears the global variables at offsets -4 and -8 and sets the one at offset -4 to one.
- `store_cleared_result`: Calls another routine and stores the resulting 32-bit value from d0 into a 32-bit location at a negative offset from a6.
- `calculate_modulo_and_negative_difference`: Calls another routine, subtracts a 32-bit value at a negative offset from a6 from d0, divides the result by six, subtracts a 16-bit value at offset -206 from a6, and returns the negative of the quotient in the high word and the remainder in the low word.
- `branch_to_distant_routine`: Performs a short relative branch to a routine located at a fixed offset.
- `branch_to_offset_f42`: Performs a short relative branch to the instruction located at offset f42.
- `branch_to_relative_address`: Performs a short relative branch to a specific offset in the code.
- `branch_to_distant_routine_2`: Performs a short relative branch to a routine located at a fixed offset.
- `branch_to_text_output_routine`: Performs a short relative branch to another routine.
- `branch_to_address`: Performs a relative branch to a specific offset.
- `update_buffer_pointer`: Calculates a new address by adding a value from a global variable to a pointer stored at an offset in a6, updates three words at offsets -4a, -48, and -4e of the structure in a6, calls advance_page_counter_cached, calls process_buffer_byte, and branches to another routine.
- `set_constant_and_branch`: Sets d0 to 1 and branches to another routine.
- `clear_and_branch`: Clears d0 and branches to another routine.
- `decrement_and_update_counter`: Subtracts two from d0, adds the result to a 16-bit value at a negative offset from a6, and branches to another routine.
- `load_and_branch`: Loads a word from the address in a4 and increments a4, then branches to another routine.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a4, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_virtual_address_space_2`: Updates a virtual address space by reading a variable-length integer from a buffer, adjusting an offset and a page counter, and either updating a table entry or modifying a cached pointer structure based on the decoded value.
- `update_virtual_address_space`: Calls three separate routines, sets a register to one, calls a fourth routine, and branches to another routine.
- `parse_variable_length_data_and_update`: Checks several conditions by calling other routines, then reads a variable-length integer from a buffer pointed to by a value at offset 70 of the structure in a6.
- `decode_variable_length_integer_2`: Reads a variable-length integer from a buffer, performs bitwise manipulations to reconstruct the value, and updates a structure in a6 by either incrementing a counter or inserting the value into a table or list.
- `call_routine_with_argument`: Pushes a word from d0 onto the stack, calls another routine, restores the word to d0 from the stack, loads a constant address into a0, and branches to another routine.
- `call_routine_with_address`: Pushes a word from d0 onto the stack, calls another routine, restores the word to d0 from the stack, loads a constant address into a0, and branches to another routine.
- `refresh_story_page`: Clears a 16-bit value at a negative offset from a6, calls a routine to output a text sequence, calls a routine to read a block from the story file into a buffer, updates a pointer with a 512-byte offset, sets bit 5 of a byte at a specific offset in a structure, and branches to another routine.
- `jump_to_return_constant_one`: Jump stub: passes control straight on to return_constant_one.
- `print_interpreter_version_and_update`: Prints the interpreter version and sub-version strings, then performs a loop to compare an index against a step value.
- `decode_variable_length_integer_3`: Reads a variable-length encoded integer from a buffer and updates a value and a secondary counter within a structure pointed to by a6.
- `dispatch_instruction_opcode`: Reads an opcode byte and dispatches on its low four bits through a switch, calling helper routines for each case.
- `clear_d0_and_load_address`: Clears d0, loads the address of a global variable into a0, and branches to another routine.
- `decode_zstring_text_body`: Decodes packed text in the same way as its neighbouring entries: three 5-bit characters per word, shift and abbreviation codes, and an end bit.
- `classify_character_case`: Returns three for a zero byte, the argument with its low word cleared for a lowercase letter, one for an uppercase letter and two for any other character.
- `classify_letter_case`: Returns the argument with its low word cleared for a lowercase letter, one for an uppercase letter and two for any other character.
- `test_uppercase_letter`: Returns one when its character is an uppercase letter and two otherwise.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_clear_d0`: Loads a long value into a0, compares a byte at the pointer in a0 with d0, and if the byte is zero, clears d0 and branches; otherwise, if the byte is non-zero, it branches to another routine.
- `calculate_offset_difference`: Subtracts a 16-bit value at a negative offset from a6 from a0, adds five to d0, and if d0 is less than 32, branches to another routine; otherwise it subtracts 26 from d0 and branches to another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `process_character_stream`: Loops up to six times to load bytes from a1, classify them, and push various derived values onto the stack, then performs a second loop to combine these values into a table at a0.
- `populate_table_with_bit_shifted_values`: Runs a loop to write the value five to a stack-based array, then performs a second loop that reads two 16-bit values from a stack-based array, shifts and combines them into a single 16-bit value, stores the result in a table at a2, and sets the high bit of the last entry in the table.
- `init_buffer_header`: Initialises a buffer header: it points the first pointer field at the data that follows, clears a count, stores two argument values and clears a flag byte.
- `tiny_moveq`: Small routine of 1 instruction that loads 13 into d0.
- `write_character_or_flush_pending`: When a mode flag is zero it runs a flush step guarded by two flags, otherwise it appends the character to the output buffer.
- `buffer_text_with_word_wrap`: Buffers output characters into words and flushes them at spaces or carriage returns, keeping a running width count.
- `reset_buffer_pointer_and_flag`: Clears a count, calls one routine, resets the buffer pointer to the start of the buffer and sets a flag byte to one.
- `restart_buffer_after_call`: Calls one routine, then resets the buffer pointer to the start of the buffer and sets a flag byte to one.
- `pass_character_through_echo`: Reads a character through a routine and, when a mode flag is set and the character is not a backspace or a counter is nonzero, passes it to a second routine.
- `append_char_with_wrap`: Appends a character to a line buffer, flushing at a carriage return and wrapping at spaces when the buffered width is too great.
- `call_counted_loop_when_nonzero`: Calls one routine first.
- `advance_page_counter_cached`: Advances a page counter by the high bits of an offset and keeps the low nine bits, and when the page changes it stores the previous state and reloads the page base through a routine.
- `find_or_create_table_entry`: Searches a table of four-word entries for the given key, updating the entry's counter and remembering the matching entry.
- `find_least_used_entry`: Scans a table of entries ended by a sentinel and returns the address of the entry with the smallest counter, computed from a base address and a 64-byte entry size.
- `handle_story_open_error`: Runs the error path for a failed story file open: it calls routines to clean up and, when a status value is nonzero, to report it, then returns the status.
- `report_not_enough_memory`: If a helper returns zero, it calls routines to report the referenced message Not enough memory and clean up, then returns the result.
- `forward_call_to_routine`: Calls one routine and returns.
- `forward_word_argument`: Passes its word argument to one routine and returns.
- `relay_call_to_routine`: Calls one routine and returns, with no other work.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `return_constant_one`: The decompiled body returns the constant one.
- `pass_through_routine_call`: Calls one routine and returns, with no other work.
- `read_and_echo_character`: Reads a character through a routine and, when a mode flag is set and the character is not a backspace or a counter is nonzero, echoes it through a second routine.
- `read_key_mapping_delete`: Sets two state fields to one and reads keys until a carriage return, backspace or printable character arrives.
- `clear_state_then_call`: Clears a state field and calls one routine.
- `call_subroutine`: The routine calls another routine and returns.
- `count_and_flush_at_difference`: Increments a counter and, when it reaches the difference between two stored values, calls two routines and resets the counter to two.
- `clear_state_field_and_call`: Clears a state field and calls one routine.
- `store_result_in_record`: Conditionally calls routines based on a flag and a comparison of two values, then stores the result of a further call as a word in a record at offset 4.
- `store_helper_result_in_record`: Stores the word returned by a helper routine in a record at offset 4.
- `return_difference_or_helper`: Returns the difference between two arguments, or the result of a helper routine when the difference is nonzero and the first argument is not below the second.
- `emit_string_with_optional_result`: Passes each character of a zero-terminated string to a routine, then returns zero, or the result of a further routine call when a state field is nonzero.
- `emit_line_end_or_call`: When its argument is nonzero it calls a routine, and when a mode flag is set it appends a carriage return to the buffer, otherwise it calls routines chosen by two flags.
- `call_routine_by_record_flag`: Tests bit zero of a flag byte at offset 17 in a record and calls one of two routines accordingly, returning its result.
- `pass_word_to_routine`: Passes its word argument to one routine and returns.
- `clear_record_flag_bit`: Calls one routine, then clears bit zero of a flag byte at offset 17 in a record.
- `call_if_result_nonzero`: Calls a routine and, when its result is nonzero, calls a second routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately_unreached`: Does nothing and returns.
- `store_global_on_success`: Calls a routine and, when the result is zero, stores a global value in a frame slot and returns zero, otherwise it returns the nonzero result.
- `consume_saved_argument`: Passes a saved frame value to one routine and then clears that slot to zero.
- `call_routine_then_return`: Calls one routine and returns, performing no other operations.
- `read_story_file_block`: Reads a block from the story file by calling a read helper, returns zero when the read fails and otherwise calls further routines.
- `read_capital_letter_or_return`: Reads characters until a carriage return or an uppercase letter arrives, folding lowercase letters to capitals, then returns the stored value.
- `check_first_answer_letter`: Calls another routine and, if it returns non-zero, calls three other routines to emit a string, reset a buffer, and read an input line; it then scans the input buffer for the first non-space character, returning zero if that character is 'y' and one otherwise.
- `call_then_handle_zero`: Saves registers on the stack, pushes the arguments one and zero, calls a routine, then restores the registers.
- `call_and_update_global`: Saves registers, calls a routine, and if d0 is zero, copies a value from a global variable to an offset of negative 256 from a6 and clears d0.
- `save_arg_then_call`: Pushes the value of d0 onto the stack, calls another routine, restores d0 from the stack, pops the saved registers, and returns.
- `swap_and_write_block`: Swaps halfwords in d1 and d0 twice using a routine, pushes several values onto the stack including a value from a global variable, calls a routine, and then branches based on the result in d0.
- `report_disk_full`: Loads the address of a global table into a0, calls a routine to report the condition, restores the stack pointer and registers, and returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `forward_pending_value`: Pops the handle from the frame slot at offset -256 of a6, passes it to a routine that closes it, and returns the result in d0.
- `delete_named_file_2`: Calls a routine that deletes a file named in a global variable and returns its result in d0.
- `prompt_for_save_file_name_2`: Pushes two zero values onto the stack, calls a routine to prompt for a filename, restores the stack and registers, and returns the result in d0.
- `call_and_set_d0`: Saves registers, calls a routine, and if d0 is zero, copies a value from a global variable to an offset of negative 256 from a6 and clears d0.
- `report_save_file_missing`: Loads the address of a global table into a0, calls a routine to report the condition, sets d0 to one, and returns.
- `read_swapped_block`: Calls a routine to swap halfwords twice, pushes several values including a pointer from a0 and a value from offset -256 of a6 onto the stack, calls a routine to read a block, and returns.
- `clear_d0_and_branch`: Clears the word in d0 and branches to another routine.
- `call_routine_with_one`: Loads the constant one into d0, saves the registers, pushes the value, and calls another routine.
- `tail_call_helper_routine`: Calls one subroutine routine with no arguments and returns, performing no other work.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `conditional_call_sequence`: Saves registers, moves d0 to d1, and if d1 is not zero, calls a routine and branches based on the result.
- `execute_sequence_of_routines`: Calls a routine, and if the result is not zero, loads a pointer into a0, then calls three consecutive routines before branching.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `prompt_for_save_file_name`: Shows the save prompt that includes the default name and asks for a save disk, then clears the chosen name entry or passes zero to a routine.
- `call_with_zero_then_return`: Calls one routine with a zero argument, then calls another with no arguments, and returns the value it was given.
- `invoke_system_trap`: Stores the incoming argument in a global cell at offset 776, then executes the trap instruction with number one.
- `execute_trap_two`: Moves a long value from the stack into a global variable, executes trap 2, and then restores the value to the stack before returning.
- `execute_trap_thirteen`: Moves a long value from the stack into a global variable, executes trap 13, and then restores the value to the stack before returning.
- `execute_trap_fourteen`: Moves a long value from the stack into a global variable, executes trap 14, and then restores the value to the stack before returning.
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
- `_ReplyMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `_WaitPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `_CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_SendIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_CurrentTime`: Calls the library function at a fixed offset of 84 bytes below the stored library base and returns.
- `_DisplayBeep`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_GetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_AllocRemember`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `_FreeRemember`: Calls the library function at a fixed offset of 408 bytes below the stored library base and returns.
- `copy_zero_terminated_string`: Copies a zero-terminated string from the source to the destination and returns the destination address.
- `copy_string_with_limit`: Copies at most a given number of characters, stopping early at a zero byte, then terminates the destination and returns it.
- `terminate_string_at_pointer`: Writes a zero byte through a stored pointer and returns a value from the frame.
- `format_decimal_digits`: Writes the decimal digits of a value into an eleven-character buffer from the right, copies at most the requested number of trailing digits to the output and returns the count.
- `copy_string_to_offset`: Calls a routine with its first argument, then copies a zero-terminated string from the second argument to the address formed by the returned value plus the first argument.
- `copy_string_to_offset_limited`: Like its neighbour, copies a string to an offset computed by a routine, but copies at most a given count of characters and terminates the destination with a zero byte.
- `terminate_string_at_slot`: Writes a zero byte through a stored pointer and returns a frame value.
- `count_string_bytes`: Scans forward from the offset given as its argument until a zero byte and returns the number of bytes before it.
- `parse_signed_decimal`: Parses an optional plus or minus sign and a run of decimal digits from a string, stores the signed value through a pointer and returns the number of characters consumed.
- `find_first_character`: Searches a zero-terminated string for the given character and returns a pointer to its first occurrence, or null when it is absent.
- `pass_search_args_onward`: Passes its arguments to one routine and returns.
- `find_last_character`: Scans a zero-terminated string and returns a pointer to the last occurrence of the given character, or null when there is none.
- `return_frame_slot_value`: Returns a value stored in a frame slot.
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
- `fill_global_table_entries`: Calls a routine twenty times and passes the address of a global table entry, chosen by each returned index, to a second routine.
- `allocate_sized_block_with_header`: Allocates a memory block 12 bytes larger than its argument and returns zero if the allocation fails.
- `return_result_or_minus_one`: Calls a routine with its argument and returns that result when it is nonzero, otherwise it returns minus one.
- `return_minus_one`: The routine is unreachable and consists of a single instruction that returns a constant value.
- `call_then_clear_globals`: Calls another routine, then clears the long values stored at offsets 16, 8, 12, 8, 0, 12, and 4 of two global structures before returning.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global value.
- `append_node_to_list`: Appends a node to a list whose head and tail are held in the two words of the first argument.
- `copy_pointer_value`: Checks if a pointer retrieved from a stack-based structure is null, otherwise it copies a longword from offset 4 of that structure to offset 4 of another structure retrieved from a6 plus 8.
- `update_list_pointer_2`: Loads a pointer from the stack, accesses a second structure via that pointer, and copies a value from an offset of 4 in the second structure to an offset of 4 in the first structure; otherwise, it copies a value from the second structure to an offset of 8 from the stack pointer.
- `update_list_pointer`: Loads a pointer from a structure at offset 12 of the stack frame, accesses a second structure via an offset of 4 from that pointer, and stores the first pointer into the location pointed to by the second structure.
- `split_words_and_get_handles`: Splits a text string into up to 32 words at separator characters, storing a pointer to each word and terminating it with a zero byte.
- `divide_with_shift_subtract`: Divides its first argument by its second using a 32-step shift and subtract loop, returning the quotient, or zero when either argument is zero.
- `empty_stub_function`: The routine contains a block of 31 68000 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `claim_free_table_entry`: Searches a table of 24-byte entries for the first one whose flag byte at offset 16 is clear, then passes the arguments and that entry to a routine.
- `call_routine_with_table_entry`: Returns zero when the entry pointer is the table end marker, otherwise it passes the stored arguments and the entry pointer to a routine.
- `pass_table_entry_arguments`: Passes stored arguments from a frame to one routine and returns.
- `open_file_with_mode_string`: Parses a mode string built from the letters r, w and a with optional plus and binary markers, opens the named file with matching flags and creates write and append files with permission 420.
- `open_stream_from_frame_arguments`: Opens a stream the same way as its neighbour, but takes the mode arguments directly from the frame.
- `open_file_and_position`: Opens a file with flags taken from a frame value plus a fixed bit, positions it with a seek, and fills a stream record with its handle and flags.
- `position_stream_and_fill_record`: Seeks the open file with a fixed mode and fills a stream record with its handle, flags and buffer pointer, then returns the record.
- `initialise_handle_record`: Stores a fixed mode value, consults a flag byte through a table lookup and a called routine, then sets the link field, clears the pointer fields and returns the record.
- `open_handle_and_initialise_record`: Picks an access value from a flag byte and passes it with a stored field to a called routine, returning null if that call fails.
- `open_record_with_arguments`: Passes its two arguments to a called routine and returns null if the result is minus one.
- `set_record_mode_and_initialise`: Stores a flag-selected value of one or the other in a local, consults a table flag through a called routine, then sets the link field, clears the pointer fields and returns the record.
- `initialise_record_with_mode_one`: Stores the constant one as the mode value, consults a table flag through a called routine, then sets the link field, clears the pointer fields and returns the record.
- `open_record_with_write_flags`: Combines an access value chosen from a flag byte with the caller's request bits and fixed bits, then passes the result to a called routine with a permission value.
- `open_record_with_create_flags`: Combines the caller's request bits with a fixed bit pattern and passes them to a called routine with a permission value.
- `set_record_flags_and_initialise`: Stores a flag-selected value of one or the other as the mode, consults a table flag through a called routine, then sets the link field, clears the pointer fields and returns the record.
- `initialise_record_with_mode_two`: Stores the constant two as the mode value, consults a table flag through a called routine, then sets the link field, clears the pointer fields and returns the record.
- `return_zero`: The routine is empty and returns a zero value to the caller.
- `initialise_record_from_lookup`: Calls a routine and then either clears the link field or sets a flag bit and points the link field at an offset within the record.
- `finish_record_without_link`: Clears the link field of the record, copies a saved value into it, clears the two pointer fields and returns the record.
- `clear_object_links`: Releases the first linked object when a flag bit is set, and releases the second linked object when its flag bits are clear and its pointer is non-zero.
- `lookup_valid_table_entry`: Checks that the index is non-negative and below a limit and that the table entry at that index is marked in use, then returns the entry's address.
- `claim_slot_and_open_file`: Finds a free entry in a fixed table, marks it with access and direction bits taken from the mode argument, optionally deletes the named file and then opens it.
- `open_slot_from_record`: Uses the record's stored slot to set its direction and access bits, optionally deletes the named file and opens it, storing the returned handle in the record.
- `open_record_slot_for_request`: Derives the access and direction bits for the record's slot from its mode field, optionally deletes the named file and opens it, storing the handle in the record.
- `open_record_after_slot_setup`: Sets the slot's base flag, adds access bits derived from the mode field, optionally deletes the named file and opens it, storing the handle.
- `open_slot_with_mode_bit`: Adds a mode-derived bit to the slot byte, optionally deletes the named file and opens it, then stores the handle.
- `open_slot_with_fixed_mode`: Sets a fixed access bit in the slot byte, optionally deletes the named file and opens it, then stores the handle.
- `open_slot_with_combined_mode`: Sets a flag-selected bit and a combined access pattern in the slot byte, optionally deletes the named file and opens it, then stores the handle.
- `open_slot_with_access_pattern`: Adds a fixed access pattern to the slot byte, optionally deletes the named file and opens it, then stores the handle.
- `reject_invalid_mode_request`: Clears the slot byte, records error code 22 and returns minus one, rejecting the request without opening anything.
- `open_named_file_from_flags`: Optionally deletes the named file, chooses an open mode from the request bits and opens the file, storing the handle in the slot.
- `open_file_with_constant_mode`: Opens the named file with a fixed mode constant and stores the returned handle in the record.
- `return_saved_local_value`: Returns the four-byte value held in a local stack slot without doing anything else.
- `forward_open_request_with_flags`: Forwards the first argument unchanged to another routine, together with two values derived from the second: its top bit combined with a fixed set of access bits, and its remaining bits unchanged.
- `read_buffer_from_handle`: The routine checks if a table index is valid, retrieves a handle, and reads a specified number of bytes into a buffer, recording an error if the read fails.
- `read_bytes_into_buffer`: Reads the requested number of bytes from the handle held in a local record into the given buffer and stores the result.
- `write_to_handle_with_error_check`: The routine obtains a handle from another routine and, if the handle is valid and a flag in the handle is set, it calls another routine.
- `seek_file_with_offset_table`: The routine obtains a file handle from another routine and, if valid and the index is less than three, seeks the file to a position taken from a global table using a second argument, records an I/O error if the seek fails, then seeks the file back to the unable and returns the result.
- `close_handle_if_active`: The routine retrieves a handle from a table, checks a flag, and if the flag is not set, closes the handle and records an error if the close fails.
- `delete_named_file`: Deletes the named file and returns zero on success.
- `return_zero_2`: The routine returns a zero value and does not perform any other operations.
- `forward_delete_file`: Passes a single argument from the stack to a routine that deletes a file and returns.
- `rename_file_with_error_handling`: Calls a routine to rename a file using two arguments from the stack; if the result is zero, it calls an error routine, stores the error in a global variable, sets another global variable to 5, and returns -1, otherwise it returns 0.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `open_default_file_into_global`: Opens the named file with a fixed mode constant and stores the returned handle in a global variable, with no error handling.
- `read_buffered_byte`: Returns a pending byte from a one-byte buffer and clears it when one is set.
- `read_single_byte_from_handle`: Reads one byte from the global handle into a local stack slot and returns that byte.
- `read_and_mask_byte`: Checks a byte at a global variable, and if it is non-zero, moves it to d0, clears the global variable, stores the byte at an offset of negative 1 from a6, and masks d0 with 255.
- `call_read_buffered_byte`: Calls a routine to read a buffered byte and then returns.
- `write_single_byte_to_handle`: Writes its single byte argument to the global handle, first calling a routine if no handle has been set, and returns that byte.
- `check_and_return_constant`: Checks a byte at a global variable, and if it is non-zero, sets d0 to 255 and returns.
- `store_byte_from_argument`: Copies the low byte of its stack argument into a global variable and returns.
- `read_file_into_buffer`: Checks for signals, opens a file handle if necessary, reads up to 255 bytes into a buffer, and if the read returns a negative value, clears a byte at the calculated buffer position.
- `write_string_to_file`: Checks for signals, opens a file handle if necessary, calculates the length of a string from a stack argument, and writes that string to the file handle.
- `forward_two_arguments`: Forwards its two arguments unchanged to another routine and returns without any other work.
- `read_byte_from_refilled_buffer`: Returns the next byte from the record's buffer and decrements its count, refilling the buffer through a read when it is empty.
- `read_buffered_byte_after_init`: Initialises the record's buffer through a called routine when no buffer is set and the record's mode bit is clear, returning minus one if that fails.
- `read_byte_refilling_buffer`: Returns the next buffered byte, first refilling the buffer through a read when it is empty.
- `fill_buffer_and_return_byte`: Returns minus one if the record's flag bit is set, otherwise it reads a fresh buffer load through a called routine and records error or end flags from its count.
- `refill_and_read_buffer_byte`: Marks the record as in use and fills its buffer through a called routine, recording error or end flags from the result.
- `take_next_buffered_byte`: Decrements the buffer count and returns the next byte, calling a routine to refill the buffer when the count goes negative.
- `forward_record_to_routine`: Passes the record pointer held in a local slot to another routine and returns nothing else.
- `write_byte_into_buffer`: Writes one byte from the argument into the record's output buffer and decrements its count, flushing through a called routine when the buffer is full.
- `allocate_record_buffer`: Returns zero if the record already has a buffer and its flag bit is clear.
- `scale_global_by_eight`: Returns a global variable multiplied by eight, using a left shift by three bits.
- `forward_value_twice`: Passes its single argument to another routine as both arguments and returns.
- `allocate_from_free_list`: Allocates a block of the requested size from a free list kept in globals, splitting a larger block when needed and reducing the free total.
- `forward_with_repeated_second`: Forwards its two arguments to another routine with the second argument repeated as a third argument, then returns.
- `release_block_to_free_list`: Returns a block to the address-sorted free list held in globals, merging it with an adjacent block when they touch and adding its size to the free total.
- `report_pending_break_signal`: Checks the task's signal mask for the Ctrl-C and Ctrl-D break bits and returns zero when neither is set.
- `compute_signed_magnitude_division`: Divides two 32-bit values using their magnitudes in a restoring shift-and-subtract loop over 32 bits, then applies the sign of the xor of the inputs to the quotient.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
