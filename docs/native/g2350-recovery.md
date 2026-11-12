# Game 2350 (hitchhiker's guide to the galaxy, the - release 58 (1984)(infocom)[a][serial 851002][wb]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02350). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901216 top-level bytes in 26 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 455 of 455 named (40 author symbols), 455 of 455 described (78 by rule, 92 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 254 of 254 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7106 of 7106 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 160982 |
| code | 22980 |
| data | 37932 |
| padding | 7066 |
| unused | 672256 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| EndCLI | 2 | 182 | 3 |
| Execute | 1 | 1182 | 33 |
| LoadWB | 12 | 537 | 30 |
| Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[a][serial 851002][WB].adf:Hitchhiker | 86 | 5163 | 383 |
| Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[a][serial 851002][WB].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 5 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 81C2) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_z_machine_story` (Hitchhiker): Starts the interpreter for a loaded story file. Kind: Z-machine version check. Pass path: The routine likely proceeds to load the story file into memory if the version header matches the expected interpreter version; the error text 'Wrong Z-machine version' indicates a failure branch when the version check fails..
- `call_error_handler_with_address` (Hitchhiker): Pushes d0 to the stack, calls another routine, pops the value back into d0, loads the address 140 into a0, and branches to another routine. Kind: sabotage path / error handler. Pass path: the branch where in_D0w == 0 (skipping the error printing/handling logic).

## Functions


### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: If the destination pointer is set, copies sixteen 32-bit words from a fixed block in the structure at the address register into the destination.
- `update_internal_state`: Performs a series of calculations and calls another routine multiple times to update a structure in a1, modifies a global variable at offset 564 of a2, and eventually jumps to a location in a6.

### Execute

- `initialize_command_execution_environment`: Allocates and initializes a data structure for command processing by populating it with function pointers and clearing internal buffers.
- `parse_script_directive`: Reads a word from the command line and selects one of eight directive kinds by index.
- `read_key_into_buffer`: Reads characters up to a newline, copying them into a buffer of at most twenty bytes in the structure at offset 120.
- `return_first_buffer_byte`: After checking for pending input, returns the first byte of the buffer at offset 120 in the structure.
- `check_single_character_argument`: Checks that the buffer at offset 120 holds exactly one character and returns that character.
- `initialize_structure_and_branch`: Clears two 4-byte fields at offsets 24 and 28 of a structure and sets a 4-byte field at offset 12 to zero, then loads a pointer from offset 16 into a register and branches to another routine.
- `validate_input_character_match_2`: Copies a sequence of bytes from a source to a destination using a loop, then compares a character from a structure in a3 against a value in d7 by calling two library functions via a pointer in a6.
- `process_input_character`: Compares a character from a structure in a3 with a stored value in d7 and, if they match, executes two library functions via a pointer in the structure to update d7.
- `execute_library_calls`: Calls a routine at an offset of negative 54 from a6, moves the result to d1, sets d2 to 255, calls another routine at an offset of negative 318 from a6, and then branches to either a location at offset 132 or a location at offset 166 based on whether d6 is non-zero.
- `process_input_buffer_loop`: Iterates through an input buffer, comparing characters against specific values in a structure and calling other routines based on the results.
- `call_subroutine_with_stack_parameters`: Moves d0 to a0, pushes several values onto the stack including pointers from a5 and a3, calls another routine, and if the result in d0 is not zero, branches to another routine; otherwise, it sets d0 to negative one and branches to another routine.
- `compare_and_call_functions`: Compares a byte from a structure at offset 14 of a3 with d7, and if they are equal, calls two different routines using a pointer from a6 before branching.
- `compare_byte_with_long`: Loads a byte from an offset of negative 12 from a3 into d0, compares d0 with d7, and if they are not equal or if d4 is zero, branches to another routine; otherwise it clears d4 and branches to another routine.
- `compare_input_value`: Checks if a long value at an offset of negative 28 from a5 is zero; if it is, it branches to another routine, otherwise it compares d7 to 34 and, if they match, sets d0 to 2 and branches to another routine; if they do not match, it sets d0 to 34 and branches to another routine.
- `validate_input_character_code`: Compares a value in d7 against 42; if they match, it calls three library routines to process the value and then compares the result against 69, setting d7 to 27 if they match, otherwise it returns.
- `validate_input_character_2`: Copies the value in d7 to d0, calls another routine, and then compares d0 with the constant 78; if they are not equal, it sets d7 to 10 and branches to another location, otherwise it branches to another location.
- `validate_input_character`: Compares a value in d7 against three specific constants, 59, 32, and 61, and if none match, calls another routine with parameters from a structure in a5 and the value in d7, then returns the result of that call or negative one.
- `validate_input_character_match`: Compares a character from a structure in a3 with a value in d7, and if they match, calls two library functions via a pointer in a6.
- `test_and_execute_logic`: Tests a value at an offset from a5, and if it is not equal to 61, compares d7 with 61 and calls two different routines using a pointer from a6 if they match.
- `clear_d0_and_branch`: Sets d0 to zero and branches to another location.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `compare_and_branch_on_value`: Saves registers to the stack, loads several values from the stack, compares a value from a pointer in a2 with a value from the stack, and branches if the value is not less than or equal to the comparison target.
- `store_and_increment`: Stores the byte from d0 at the address in a3 plus the offset in d5, then increments a value in a2 and returns.
- `setup_register_state`: Saves registers d6, d7, a2, and a3 to the stack, loads a pointer from an offset of 12 in a structure into a3, loads a long value from an offset of 16 into d7, loads a pointer from an offset of 20 into a2, sets d6 to zero, and branches to another routine.
- `push_and_call_routine`: Loads a byte from a table using a6 and d6, pushes four values onto the stack including a pointer from a5 and the loaded byte, calls another routine, restores the stack pointer, and branches to another location or clears d0 based on the value of the loaded byte.
- `decrement_and_check_table_index`: Increments d6, moves a pointer from a3 to a0, and checks if the byte at the address in a0 is non-zero; if it is zero, it subtracts 1 from a0, subtracts a3 from a0, compares a0 to d6, and if a0 is less than d6, it restores registers from the stack, unlinks a5, and returns; otherwise it continues.
- `call_two_library_functions`: Calls two functions through the library base stored in the structure at the address register, passing one value between them.
- `finish_and_release_output`: When either of two fields in the structure is set, calls library functions to finish and release the output it refers to.
- `call_two_library_functions_2`: Calls two functions through the library base at the address register, with no arguments and no result of its own.
- `store_registers_to_pointer`: Stores the data registers 1 to 7, the address registers 1 to 6, a fourteenth register value and a value from the stack into consecutive 32-bit slots starting at the pointer it receives, then returns zero.
- `normalize_param_and_copy_word`: Sets the second argument to one when it is zero, stores the structure's first 32-bit value at the address held in its fourteenth slot, and returns the adjusted argument together with the structure's second 32-bit value.
- `divide_with_sign_handling`: Divides a 32-bit dividend by a 32-bit divisor.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor using normalised 16-bit steps.

### LoadWB

- `start_loadwb_program`: Program entry point of the LoadWB command.
- `release_loadwb_resources`: Saves a parameter into the current control block, then calls cleanup routines through the library table for the block's file, library and task fields when they are set.
- `start_workbench_tool`: Starts a tool by finding its resident tag or launching it, and reports errors such as a missing resident tag or a tool that cannot start.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: Walks a table of linked entries, allocating a node for each one and storing a value derived from that entry in the node's second word.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_routine_with_stack_slot`: Calls another routine with its two incoming parameters and the address of a local stack slot, then returns.
- `format_and_forward_text_buffer`: Calls a library function through a global pointer, which fills a 140-byte stack buffer, then measures the null-terminated text in it.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compare_memory_blocks`: Compares a block of memory starting at a2 with a block of memory starting at a pointer found via a relative offset, branching based on the result.
- `write_dot_and_padding`: Writes a dot followed by four spaces into five consecutive bytes starting at offset 17 of the buffer at the address register, then returns.
- `compare_reference_prefix`: Compares the start of the buffer at the address register with a global reference string, advancing through both while they match.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use.

### Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[a][serial 851002][WB].adf:Hitchhiker

- `build_argument_string_and_run`: Program entry point.
- `release_resources_and_return_value`: Runs a cleanup routine, then closes or frees the library and message resources recorded in the global block when they are set.
- `call_two_library_functions_3`: Calls two functions through the library base at fixed offsets, with no arguments and no result of its own.
- `open_library_or_report_failure`: Calls a library function to obtain a library base and stores it in a global.
- `setup_io_request`: Stores two parameters and a constant into global variables before calling an I/O routine.
- `send_io_request`: Stores a constant, a pointer to a global variable, and a value into global variables before calling an I/O routine.
- `wait_for_message_return_byte`: Waits for a message port, fetches the message, saves its first byte, calls another routine, and returns that byte.
- `send_console_sequence_and_echo`: Writes a two-byte sequence whose first byte is 155 and whose second byte is zero through another routine with the incoming value.
- `read_string_until_delimiter`: Waits for a start marker value, then copies incoming values into the buffer until the delimiter byte arrives.
- `display_beep_on_code_two`: When the incoming code equals two, it calls the display beep function with a value read from the structure; otherwise it returns without action.
- `return_current_time_value`: Calls the current-time library function and returns the value it stores in its second output argument.
- `dispatch_by_value`: Calls another routine using a global variable as an argument based on the value of the input parameter, except when the parameter is three.
- `reply_pending_message`: Gets a message from the port stored in the structure and, when one is returned, replies to it.
- `compute_grid_cell_counts`: Computes column and row counts from two sizes in the structure, each reduced by two margins and divided by a cell size.
- `query_console_window_size`: Sends two console cursor-position queries and parses each reply at its separator, then stores the two numbers minus one in global variables as the window size.
- `format_two_numbers_as_text`: Converts two numbers, each increased by one, into 32-byte text buffers.
- `write_line_break_or_padding`: If a stored column value is below its limit, it writes a carriage return and a line feed.
- `emit_global_text_buffer`: Passes a global text buffer to one routine and returns.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `write_text_with_stored_index`: Calls one routine with zero arguments, passes a global text buffer to a second routine, then calls a third routine with a zero argument and a stored count minus one.
- `forward_call_with_short_argument`: Forwards its first argument and its 16-bit second argument to another routine with no other work of its own.
- `echo_character_with_backspace_handling`: Writes one character.
- `forward_call_no_arguments`: Calls one other routine with no arguments and returns.
- `draw_padded_text_field`: Writes a padded text field: it positions output with stored values, fills a computed run of spaces when there is room, writes the caller's text, and finishes by positioning the cursor at the stored values.
- `write_more_prompt`: Writes a six-character blank run, then the [MORE] prompt text through routines, calls another routine in between, and then writes the blank run again to clear the prompt.
- `stub_return_only_2`: Routine consisting only of a return, with no calls, memory access or hardware register use.
- `call_same_routine_either_way`: Calls one routine with the global data pair whether the incoming value is zero or not, so both branches do the same work.
- `forward_call_global_pair`: Calls one other routine with the global data pair and returns.
- `truncate_path_after_separator`: Searches a global path string for a slash or a colon.
- `save_disk_object_if_set`: If a stored disk object pointer is non-null, sets two fields of that object to a fixed sentinel value and saves it through the disk-object write wrapper.
- `save_disk_object_unconditionally`: Sets two fields of the stored disk object to a fixed sentinel value, saves it through the disk-object write wrapper, clears a result slot and returns zero.
- `return_zero_result`: Clears a result slot in the caller's frame and returns zero.
- `seek_and_read_block`: Seeks to a file position when the stored position differs, then reads a block of bytes into the buffer and records the new position.
- `seek_and_write_block`: Seeks to a file position, writes the given block of bytes, and returns zero on success.
- `write_block_from_frame`: Writes a block of bytes described by the caller's frame parameters through the write wrapper.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_new_file_or_fail`: Opens the file named by a global string in the new-file mode, stores the handle, and returns zero on success or the last I/O error code on failure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_existing_file_or_fail`: Opens the file named by a global string in the existing-file mode, stores the handle, and returns zero on success or the last I/O error code on failure.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `close_handle_return_zero`: Closes the file handle passed in and returns zero.
- `delete_named_file_2`: Deletes the file named by a global string.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `test_lock_on_named_file`: Tries to take an exclusive lock on the named file or directory.
- `unlock_saved_handle`: Releases the lock handle stored in the caller's frame and returns one.
- `open_story_data_file`: Opens the data file whose name is held in a global string in the existing-file mode, stores its handle, and returns zero on success or the last I/O error code on failure.
- `close_resource`: Passes a parameter to a routine that closes a resource and returns zero.
- `timed_io_request_with_timeout`: Performs an I/O request and reads the current time before and after it.
- `open_or_close_printer_port`: When the argument is non-zero, opens the printer device, allocates a signal, finds the current task and adds a message port.
- `send_escape_sequence_once`: On first use it initialises a one-time flag through another routine.
- `write_text_or_escape_sequence`: Writes the given number of text bytes when that count is positive.
- `format_memory_status_text`: Reads the available-memory figure, works out the bytes in use against a stored limit, and writes a status line with labels such as bytes free and bytes used.
- `allocate_tracked_memory`: Allocates a block of the requested size through the tracked-allocation call, using a global key so the block can be released later in one step.
- `open_window_and_devices`: Opens the intuition library and a fixed-size window, then the console device, a signal and a message port.
- `close_window_and_devices`: Releases what the setup routine acquired: it frees the disk icon object and the tracked memory list, removes the message port, frees the signal, closes the device, window and libraries.
- `read_and_echo_input_loop`: Reads characters until the quit character arrives.
- `process_input_loop_2`: Iterates through characters from a memory location, performing different subroutines based on whether a specific character is encountered.
- `process_input_loop`: Reads characters into a buffer and performs specific operations if the character is a certain value, continuing until a quit character is encountered.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `open_optional_file_argument`: Sets a file index to minus one.
- `initialise_z_machine_story`: Starts the interpreter for a loaded story file.
- `fetch_and_advance_story_page`: Reads a block of data from the story file into a buffer, updates a pointer to a new memory location offset by 512 bytes, sets bit 5 of a flag in a structure, and performs a bitwise shift on a value before calling routines to advance the page counter and dispatch the next opcode.
- `swap_halfwords_and_shift`: The routine swaps the two 16-bit halves of the 32-bit value in D0, then shifts the result right by 7 bits.
- `shift_result_right_nine`: Calls one routine and returns its result shifted right by nine bits.
- `add_partial_block_size`: Adds 512 to the value when its low nine bits are non-zero, and otherwise returns it unchanged.
- `return_value_via_pointer`: The routine returns the 16-bit value pointed to by a pointer in A0.
- `store_word_big_endian`: Writes a 16-bit value into the two bytes at the address register, high byte first.
- `advance_value_at_limit`: Compares the incoming 16-bit value with a limit stored in the frame, fetching a value through another routine on either side.
- `call_two_routines_in_order`: Calls two other routines one after another and returns, with no other work of its own.
- `read_next_byte_from_buffer`: Reads the next byte from a buffer whose base address is stored in the frame, advances the buffer offset by one, and calls another routine when the offset passes 511.
- `read_operand_by_type_code`: Reads one 16-bit operand chosen by a type code.
- `read_variable_word`: Reads a 16-bit variable chosen by number.
- `write_variable_word`: Writes a 16-bit value to a variable chosen by number.
- `write_variable_word_2`: Writes a 16-bit value into a memory location determined by an index, using either a direct table or an offset from a base pointer.
- `write_global_variable_word`: Writes a 16-bit value, high byte first, into the global variable table entry for the given number.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (andi.w) and then falls through into the code that follows, with no return of its own.
- `process_buffer_byte`: Copies the word from D1 to the stack at the location pointed to by A4 if the byte returned by another routine in D0 is zero, otherwise it calls another routine.
- `clear_d1_and_branch`: Clears the word in d1 and branches to another location.
- `decode_variable_length_integer`: Reads a variable-length integer from a buffer by checking specific bits to determine if additional bytes are required.
- `decrement_and_branch`: Decrements d0 by one and branches to an earlier instruction if the result is not zero, otherwise it branches to another location.
- `decrement_and_add`: Decrements d0 by one, adds the result to a value at an offset from a6, and branches to another location.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `shift_value_right_nine`: Returns the incoming value shifted right by nine bits.
- `return_high_byte`: Returns the high byte of the incoming 16-bit value.
- `shift_value_right_seven`: Returns the incoming value shifted right by seven bits.
- `add_and_branch`: Adds the word in d1 to d0 and branches to another location.
- `subtract_and_branch`: Subtracts the word in d1 from d0 and branches to another location.
- `multiply_and_branch`: Multiplies d1 by d0 and branches to another location.
- `divide_and_branch`: Extends d0 to a longword, divides it by the word in d1, and branches to another location.
- `divide_and_swap`: Extends the value in d0 to a long, divides d1 by d0, swaps the high and low words of d0, and branches to another routine.
- `update_status_word`: Swaps the words of a 16-bit value from a global variable, performs a logical shift right on a 32-bit value, and XORs the result into another 16-bit value at a global variable offset.
- `divide_unsigned_buffer`: Masks D1 with a bitmask, performs an unsigned division of D1 by D0, swaps the words of D1, increments the low word of D1, moves the result to D0, and branches to another routine.
- `compare_and_branch_less_than`: Compares d0 with d1 and branches to one of two locations based on whether d0 is less than d1.
- `compare_and_branch_greater_than`: Compares d0 with d1 and branches to one of two locations based on whether d0 is greater than d1.
- `test_bit_and_branch`: Inverts d0, performs a bitwise AND between d0 and d1, and branches to one of two locations if the result is not zero.
- `or_and_branch`: Performs a bitwise OR between d1 and d0 and branches to another routine.
- `not_and_branch`: Inverts the bits of the word in d0 and branches to another routine.
- `scale_word_by_nine`: Multiplies the low 16 bits of the incoming value by nine and returns the product.
- `read_byte_table_doubled`: Reads a byte from a table at the index returned by another routine and returns it multiplied by two.
- `increment_shifted_byte`: Loads a byte from the address in A0, shifts it right by 5 bits, increments the result, and adds this value to the address in A0 before returning.
- `swap_byte_values`: Calls two routines, then swaps a byte from offset 6 of a pointer in a1 with a byte from offset 5 of a pointer in a0, and stores a byte from d1 into offset 4 of the pointer in a0.
- `compare_and_copy_byte`: Moves a word from D0 to D1, calls another routine, moves a pointer from A0 to A1, loads a byte from offset 4 of the pointer into D0, and if it is not zero, masks D0 and calls another routine before comparing a byte from offset 6 of the structure in A0 with D1; if they match, it moves a byte from offset 5 of the pointer to offset 6 of the structure in A0 and branches to another routine.
- `loop_and_copy_byte`: Calls another routine in a loop until the byte at offset 5 of the structure in A0 matches the byte in D1, then copies a byte from offset 5 of the structure in A1 to offset 5 of the structure in A0 and clears bytes at offsets 4 and 5 of the structure in A1.
- `scale_and_load_byte`: Calls scale_word_by_nine, clears d0, loads a byte from offset 4 of the address in a0 into d0, and branches to another routine.
- `push_byte_and_call`: Calls another routine, clears d0, loads a byte from offset 6 of the buffer in a0 into d0, pushes that byte onto the stack, calls another routine, and branches to one routine if the byte at the top of the stack is non-zero, otherwise it branches to another routine.
- `push_byte_and_call_offset_5`: Calls another routine, clears d0, loads a byte from offset 5 of the buffer in a0 into d0, pushes that byte onto the stack, calls another routine, and branches to one routine if the byte at the top of the stack is non-zero, otherwise it branches to another routine.
- `call_and_compare_byte`: Calls a routine, compares a byte from a table at a0 plus 4 to d1, and branches to one of two locations based on the result.
- `call_two_routines`: Calls two different routines and then branches to another routine.
- `compare_masked_byte`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching based on the comparison result.
- `calculate_offset_and_call`: Subtracts 1 from d1, doubles d1, loads a pointer from offset -16 of a6 into a0, adds d1 to a0, calls another routine, and branches to another routine.
- `call_two_and_branch`: Calls two different routines and then branches to another routine.
- `branch_on_masked_byte`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching to a specific address if the values are equal or if the comparison is greater than.
- `test_bit_and_move`: Tests bit 5 of a byte at a0 and increments a0, then branches to another routine if the bit is not set, otherwise it moves a byte from d2 to a0 and returns.
- `move_and_branch`: Moves the word from d2 into d0 and branches to another routine.
- `call_and_test_d1`: Calls two routines, tests if d1 is zero, and branches to one of two locations based on the result.
- `branch_on_masked_byte_alt`: Calls a routine, loads a byte from the address in a0, masks it with 31, and compares it to d1, branching to a specific address if the values are equal or if the comparison is greater than.
- `mask_and_branch`: Calls another routine, loads a byte from the address in a0 into d0, masks d0 with 31, and branches to another routine.
- `mask_low_three_bits`: Returns the bitwise NOT of the 16-bit value passed in D0, masked to the low three bits.
- `test_bit_in_buffer`: Calls another routine, moves the word in d1 to d0, calls another routine, tests a bit in d0 at the location pointed to by a0, and branches to one routine if the bit is set, otherwise it branches to another routine.
- `read_line_into_buffer_lowercase`: Reads characters into a text buffer until a carriage return.
- `read_line_chars_until_return`: Stores characters from the input into the text buffer until a carriage return ends the line, lowercasing letters when a flag is set and backing up one character on backspace.
- `finish_line_input`: Continues the line-input loop after a character is stored, backing up on backspace and returning the stored length when a carriage return arrives.
- `subtract_call_results_pair`: Calls a nested routine twice and returns the difference between the two values it leaves in its result registers.
- `search_bounded_table_offset`: Repeatedly calls a subroutine and compares its result against two table words, adjusting a running offset and clamping it to a limit until a match is found.
- `step_clamped_table_search`: Calls a subroutine twice and compares each result with two table words, advancing a clamped offset in the same search loop.
- `walk_offset_with_clamp`: Adds or subtracts a subroutine result to a running offset, clamps the offset to a stored limit, and loops until its comparison with table words terminates.
- `append_byte_to_buffer`: Stores the low byte of the data register at the current position of a buffer pointer kept in the frame, then advances that pointer by one byte.
- `emit_zero_terminated_string`: Walks a zero-terminated string, passing each byte to a subroutine routine until the terminator is reached.
- `format_score_and_time_line`: Builds a status line in a 256-byte stack buffer, emitting the score and time fragments and a divided value when the counter is nonzero.
- `emit_char_when_enabled`: Appends the low byte of the data register to the current buffer when the buffer mode flag is set.
- `emit_decimal_digits`: Prints a number as decimal digits by repeated division by ten onto the stack, then emits them one at a time.
- `decode_packed_token_stream`: Decodes packed five-bit tokens from a stack-resident word buffer, dispatching each token class to a subroutine or to a small output record emitter.
- `decode_token_stream_logic`: Calls shift_value_right_nine and decode_token_stream_direct within a loop that processes a stream of data by extracting 5-bit values.
- `prepare_and_decode_tokens`: Calls three subroutines to set up state, then runs the same packed token decoding loop as its neighbouring routine.
- `update_packed_position_word`: Reads a subroutine word and splits it into a high part added to a stored value and a low nine-bit remainder.
- `call_two_subroutines`: Calls two consecutive subroutines and then branches to another location.
- `jump_to_tiny_moveq`: Jump stub: passes control straight on to tiny_moveq.
- `increment_and_call_if_greater`: Increments a 16-bit value in a global variable and, if the new value is greater than the original value, calls another routine with both values as arguments.
- `check_d0_value`: Compares the value in d0 to 1 and returns if it is equal to 1, otherwise it returns.
- `clear_and_copy_values`: If a 16-bit value at a negative offset from a6 is non-zero, it clears that value and copies two 16-bit values from offsets -8 and -6 of a6 into global variables at offsets -4 and -8.
- `initialize_and_reset_globals`: If a 16-bit value at a negative offset from a6 is zero, it sets it to one and copies two 16-bit values from global variables at offsets -4 and -8 into offsets -8 and -6 of a6; then it clears the global variables at offsets -4 and -8 and sets the one at offset -4 to one.
- `store_returned_long`: Calls another routine and stores the resulting long value from d0 at an offset of -530 from a6.
- `calculate_modulo_and_negative_difference`: Calls another routine, subtracts a 32-bit value at a negative offset from a6 from d0, divides the result by six, subtracts a 16-bit value at offset -206 from a6, and returns the negative of the quotient in the high word and the remainder in the low word.
- `branch_to_offset`: Performs a short relative branch to a location offset by 2436 bytes from the current instruction.
- `branch_to_offset_2`: Performs a short relative branch to a location offset from the current instruction.
- `branch_to_offset_3`: Performs a short relative branch to a location offset from the current instruction.
- `branch_to_offset_4`: Performs a short relative branch to a location offset by 2436 bytes.
- `branch_to_offset_5`: Performs a short relative branch to a location offset by 2434 bytes from the current instruction.
- `branch_to_distant_offset`: Performs a short relative branch to a target located at a fixed offset.
- `update_pointer_and_data`: Loads a pointer from a structure at offset -78 of a6, updates that pointer using a value at offset -70 of a6 and a long value from the pointer, stores two words from the pointed-to location into offsets -74 and -72 of the structure, calls another routine, and then calls a third routine before branching to another routine.
- `load_and_branch`: Loads the value 1 into d0 and branches to another routine.
- `clear_d0_and_branch_3`: Clears the d0 register and branches to another location.
- `modify_offset_and_branch`: Subtracts 2 from d0, adds the result to a 16-bit value at offset -74 of a6, and branches to another routine.
- `move_word_and_branch`: Moves a 16-bit word from the address in a4 to d0 and increments a4 by 2, then branches to another location.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a4, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_virtual_address_space_2`: Updates a virtual address space by reading a variable-length integer from a buffer, adjusting an offset and a page counter, and either updating a table entry or modifying a cached pointer structure based on the decoded value.
- `update_virtual_address_space`: Calls three separate routines, sets a register to one, calls a fourth routine, and branches to another routine.
- `process_scaled_position_data`: Attempts to read a variable-length integer and a scaled value from a buffer, updates a scaled position state in a structure, and branches to an error handler if validation fails; otherwise, it updates several fields in a structure using values from the buffer and returns.
- `decode_and_update_scaled_value`: Reads a variable-length encoded value from a buffer, updates a scaled position state in a structure, and either modifies a table entry or stores the decoded value into a buffer or a structure based on a subsequent byte.
- `call_error_handler_with_address`: Pushes d0 to the stack, calls another routine, pops the value back into d0, loads the address 140 into a0, and branches to another routine.
- `call_error_handler_with_address_alt`: Pushes d0 to the stack, calls another routine, pops the value back into d0, loads the address 164 into a0, and branches to another routine.
- `update_story_page_and_display`: Calls a routine to fetch and advance the story page, clears a 16-bit value at a negative offset from a pointer in a6, calls a routine to write text using a stored index, and then branches to another routine.
- `jump_to_report_story_file_error`: Jump stub: passes control straight on to report_story_file_error.
- `print_interpreter_version_and_update`: Prints the interpreter version and sub-version strings, then performs a loop to compare an index against a step value.
- `update_scaled_position_state_2`: Reads a value from a buffer and updates a scaled position structure in a global variable, potentially allocating a new table entry or modifying existing data at various offsets from a pointer in a6.
- `dispatch_coded_word_cases`: Reads a coded word and branches on its low four bits through a switch of subroutine calls that operate on indirect memory values.
- `clear_and_branch`: Clears the word in d0, loads a global address into a0, and branches to another location.
- `decode_token_stream_direct`: Decodes packed five-bit tokens from a stack buffer with the same dispatch as its sibling routines, but without the setup calls.
- `classify_character_case_class`: Returns a class code for the low byte of the data register: a terminator maps to 3, uppercase letters to 1, other characters to 2, and lowercase letters return the upper word of the input unchanged.
- `classify_letter_case`: Returns a class code for a character: uppercase letters give 1, lowercase letters return the upper word of the input unchanged, and any other byte gives 2.
- `test_uppercase_letter`: Returns 1 when the low byte of the data register is an uppercase letter and 2 for any other byte.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_clear_d0`: Loads a long value into a0, compares a byte at the pointer in a0 with d0, and if the byte is zero, clears d0 and branches; otherwise, if the byte is non-zero, it branches to another routine.
- `calculate_offset_difference`: Subtracts a 16-bit value at a negative offset from a6 from a0, adds five to d0, and if d0 is less than 32, branches to another routine; otherwise it subtracts 26 from d0 and branches to another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `process_character_stream`: Loops up to six times to load bytes from a1, classify them, and push various derived values onto the stack, then performs a second loop to combine these values into a table at a0.
- `populate_table_with_bit_shifted_values`: Runs a loop to write the value five to a stack-based array, then performs a second loop that reads two 16-bit values from a stack-based array, shifts and combines them into a single 16-bit value, stores the result in a table at a2, and sets the high bit of the last entry in the table.
- `initialise_linked_record`: Calls a subroutine, then sets up a small record: it writes a self-referencing pointer, clears a counter word, stores two argument values and clears a flag byte.
- `tiny_moveq`: Small routine of 1 instruction that loads 13 into d0.
- `buffer_char_or_update_state`: Appends the low byte of the data register to the buffer when buffer mode is active.
- `word_wrap_insert_character`: Adds one character to a line buffer while tracking the printed width.
- `reset_record_and_set_flag`: Clears a word counter in a record reached through the frame, then calls a subroutine.
- `reset_record_pointer_flag`: Calls a subroutine, then copies the record's first pointer into its second slot and sets the record's flag byte to one.
- `return_code_after_optional_echo`: Calls a subroutine to obtain a code and returns it.
- `edit_input_line_character`: Handles one input character for a line editor.
- `repeat_helper_until_count_zero`: When its flag is nonzero it repeatedly calls a subroutine while decrementing a 16-bit count until the count reaches zero, then calls a final subroutine.
- `update_scaled_position_state`: Adds the upper bits of a packed word into a stored value and masks the lower bits.
- `find_or_allocate_table_entry`: Scans a table of four-word entries for a matching key and updates the entry's stored counter when the key changes.
- `find_least_used_table_entry`: Scans a table of four-word entries and finds the one with the smallest counter value.
- `print_story_file_open_error`: Prints the story file open error message through output subroutines, with some calls gated by two state words.
- `print_not_enough_memory_message`: Checks a subroutine result and, when it is zero, runs output subroutines that print the not-enough-memory text, with some steps gated by state words.
- `report_memory_usage`: Calculates the difference between the total allocated memory and the currently used bytes.
- `pass_word_to_helper`: Forwards the low word of the data register as the argument of one subroutine call and then returns.
- `tail_call_helper_routine`: Calls one subroutine routine with no arguments and returns, performing no other work.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `report_story_file_error`: Displays an error message regarding the story file and waits for a key press from the user before exiting.
- `forward_call_to_helper`: Calls one subroutine routine and returns, adding no other work.
- `read_input_recursively`: Reads key codes and recursively calls itself based on the value of two state words and the character returned.
- `read_editing_key_code`: Sets two state words to one and reads keys until a carriage return or backspace arrives, skipping other control codes.
- `clear_state_and_call_helper`: Clears a state word in the frame, then calls one subroutine routine.
- `delegate_to_single_helper`: Calls one subroutine routine and returns with no other work.
- `cycle_counter_at_difference`: Increments a frame counter and, when it equals the difference between two global words, calls two subroutines and resets the counter to two.
- `reset_flag_word_then_call`: Clears a state word in the frame and then calls one subroutine routine.
- `store_helper_result_in_record`: Optionally calls a subroutine, depending on a state word and a comparison of two values, then calls another subroutine and stores its returned word at offset four of a record.
- `store_helper_word_in_record`: Calls a subroutine and stores its returned word at offset four of a record reached through the frame.
- `difference_with_conditional_helper`: Subtracts one value from another.
- `emit_string_return_optional_result`: Passes each byte of a zero-terminated string to a subroutine, then returns zero unless a state word is set, in which case it returns the result of another subroutine.
- `finish_internal_error_line`: Ends a line of output: when the buffer mode flag is set it appends a carriage return to the buffer, otherwise it calls output subroutines gated by state words.
- `branch_on_record_flag_bit`: Tests bit zero of a flag byte in a record reached through the frame and calls one of two subroutines depending on the result.
- `forward_word_argument_call`: Forwards the low word of the data register as the argument to one subroutine call, then returns.
- `clear_record_flag_bit`: Calls one subroutine, then clears bit zero of a flag byte inside a record reached through the frame.
- `call_helper_if_result_nonzero`: Calls one subroutine and, when its returned word is nonzero, calls a second subroutine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `stub_immediate_return`: Is an empty routine that returns immediately with no effect.
- `store_global_when_helper_zero`: Calls a subroutine and, when its result is zero, copies one global word into a frame slot.
- `pass_then_clear_frame_slot`: Passes a frame slot to a subroutine and then clears that slot to zero.
- `invoke_helper_then_return`: Calls one subroutine routine and returns, performing no other operation.
- `print_story_file_read_error`: Calls subroutines to read a value and returns zero when the result is zero.
- `read_uppercase_letter_or_return`: Reads characters through a subroutine until it gets an uppercase letter or a carriage return, converting lowercase letters to uppercase before testing them.
- `check_first_answer_letter`: Calls another routine and, if it returns non-zero, calls three other routines to emit a string, reset a buffer, and read an input line; it then scans the input buffer for the first non-space character, returning zero if that character is 'y' and one otherwise.
- `call_then_handle_zero`: Saves registers on the stack, pushes the arguments one and zero, calls a routine, then restores the registers.
- `call_and_update_global`: Saves registers, calls a routine, and if d0 is zero, copies a value from a global variable to an offset of negative 256 from a6 and clears d0.
- `save_arg_then_call`: Pushes the value of d0 onto the stack, calls another routine, restores d0 from the stack, pops the saved registers, and returns.
- `swap_and_write_block`: Swaps halfwords in d1 and d0 twice using a routine, pushes several values onto the stack including a value from a global variable, calls a routine, and then branches based on the result in d0.
- `report_disk_full`: Loads the address of a global table into a0, calls a routine to report the condition, restores the stack pointer and registers, and returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `forward_pending_value`: Pops the handle from the frame slot at offset -256 of a6, passes it to a routine that closes it, and returns the result in d0.
- `delete_named_file_3`: Calls a routine that deletes a file named in a global variable and returns its result in d0.
- `prompt_for_save_file_name_2`: Pushes two zero values onto the stack, calls a routine to prompt for a filename, restores the stack and registers, and returns the result in d0.
- `call_and_update_global_2`: Saves registers, calls a routine, and if d0 is zero, copies a value from a global variable to an offset of negative 256 from a6 and clears d0.
- `report_save_file_missing`: Loads the address of a global table into a0, calls a routine to report the condition, sets d0 to one, and returns.
- `read_swapped_block`: Calls a routine to swap halfwords twice, pushes several values including a pointer from a0 and a value from offset -256 of a6 onto the stack, calls a routine to read a block, and returns.
- `clear_d0_and_branch_2`: Clears the word in d0 and branches to another location.
- `call_routine_with_one`: Loads the constant one into d0, saves the registers, pushes the value, and calls another routine.
- `call_save_disk_object`: Saves registers to the stack, calls another routine, restores the registers, and returns.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `conditional_call_sequence`: Saves registers, moves d0 to d1, and if d1 is not zero, calls a routine and branches based on the result.
- `execute_sequence_of_routines`: Calls a routine, and if the result is not zero, loads a pointer into a0, then calls three consecutive routines before branching.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `prompt_save_disk_filename`: Calls several subroutines and then either passes zero to a subroutine or clears a global table entry chosen by a lookup.
- `call_helper_with_zero_then_return`: Calls one subroutine with a zero argument, then calls a second subroutine and returns the stacked word unchanged.
- `invoke_system_trap`: Stores the incoming argument in a global cell, then executes trap instruction number one, and restores the value from the global cell to the stack before returning.
- `execute_trap_two`: Moves a long value from the stack into a global variable, executes trap 2, and then restores the value to the stack before returning.
- `execute_trap_thirteen`: Moves a long value from the stack into a global variable, executes trap 13, and then restores the value to the stack before returning.
- `execute_trap_fourteen`: Moves a long value from the stack into a global variable, executes trap 14, and then restores the value to the stack before returning.
- `_GetDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_PutDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `_FreeDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `_AllocMem`: Calls the library function at offset minus 198 with the library base taken from a global variable, and returns its result.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_AvailMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_FindTask`: Thin wrapper that calls the library function at offset minus 294 with the library base taken from a global variable, and leaves its result in the register.
- `_SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_GetMsg`: Thin wrapper that calls the library function at offset minus 372 with the library base taken from a global variable.
- `_ReplyMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `_WaitPort`: Thin wrapper that calls the library function at offset minus 384 with the library base taken from a global variable.
- `_OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `_CloseLibrary`: Thin wrapper that calls the library function at offset minus 414 with the library base taken from a global variable.
- `_OpenDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `_CloseDevice`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `_DoIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_SendIO`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_CurrentTime`: Calls a system library function through a base pointer kept in a global word, at a fixed negative offset, to obtain the current time.
- `_DisplayBeep`: Calls a system library function through a base pointer kept in a global word.
- `_GetPrefs`: Calls a system library function through a base pointer kept in a global word, at a fixed negative offset.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_AllocRemember`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `_FreeRemember`: Calls a system library function through a base pointer kept in a global word, at a fixed negative offset.
- `copy_zero_terminated_string`: Copies bytes from a source string into a destination buffer up to and including the zero terminator, then returns the destination address.
- `copy_string_with_length_limit`: Copies up to a given number of bytes from a source string, stopping at its zero terminator.
- `clear_first_byte_and_return`: Writes a zero byte through a pointer kept in the frame and returns a word stored at a fixed frame offset.
- `format_unsigned_decimal_string`: Formats a number as decimal digits into a caller buffer, extracting each digit by repeated division by ten and writing at most the requested number of characters.
- `append_string_to_destination`: Finds the end of the destination string through a subroutine that measures its length, then copies the source string zero-terminated from that point.
- `append_string_with_length_limit`: Appends at most the given number of bytes from a source string after the destination's current end, stopping at the source terminator.
- `nullify_and_fetch_pointer`: Stores a zero byte at a location pointed to by an offset from a register and returns a value from another offset in that same memory block.
- `measure_zero_terminated_length`: Counts the bytes in a zero-terminated string starting at the given offset and returns that count without the terminator.
- `parse_signed_decimal_number`: Parses an optional plus or minus sign followed by decimal digits from a string, storing the signed value through a pointer.
- `find_first_character_occurrence`: Scans a zero-terminated string for the first occurrence of a given byte.
- `forward_word_and_byte_call`: Forwards its word and byte arguments to one subroutine routine and returns.
- `find_last_character_occurrence`: Scans a zero-terminated string and records each position holding a given byte.
- `return_frame_stack_word`: Returns the 32-bit value stored four bytes below the frame pointer without any other work.
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
- `initialise_indexed_table_entries`: Calls a subroutine twenty times, passing the address of a global table entry selected by each returned index.
- `allocate_and_init_block`: Allocates a memory block sized to the request plus twelve bytes through the allocation wrapper, records the size inside the block, and links the block through another routine.
- `allocate_block_or_return_error`: Calls the block allocation routine for the given size and returns its result when it is non-zero; otherwise it returns minus one.
- `return_minus_one`: Returns the constant minus one and nothing else.
- `call_then_clear_globals`: Calls another routine, then clears the long values stored at offsets 16, 8, 12, 8, 0, 12, and 4 of two global structures before returning.
- `free_linked_memory_blocks`: Walks a linked list whose head is held in a global variable.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are stored in the first structure.
- `copy_pointer_value`: Checks if a pointer retrieved from a stack-based structure is null, otherwise it copies a longword from offset 4 of that structure to offset 4 of another structure retrieved from a6 plus 8.
- `update_list_pointer_2`: Loads a pointer from the stack, accesses a second structure via that pointer, and copies a value from an offset of 4 in the second structure to an offset of 4 in the first structure; otherwise, it copies a value from the second structure to an offset of 8 from the stack pointer.
- `update_list_pointer`: Loads a pointer from a structure at offset 12 of the stack frame, accesses a second structure via an offset of 4 from that pointer, and stores the first pointer into the location pointed to by the second structure.
- `split_arguments_and_open_streams`: Splits the argument string in place into up to 32 words, storing a pointer to each word and clearing the separators, then opens the input and output handles through DOS calls and records them.
- `divide_with_restoring_loop`: Divides the first integer by the second using a 32-step shift-and-subtract loop and returns the quotient.
- `stub_no_observable_effect`: The recovered code is just a return, so the routine has no observable effect and is a stub.
- `claim_free_record_slot`: Scans a table of 24-byte records for one whose flag byte at offset sixteen is clear.
- `dispatch_unless_table_end`: Returns zero when a frame word equals the fixed end-of-table address.
- `forward_three_frame_arguments`: Passes three values from the frame to one subroutine routine and returns nothing.
- `open_file_with_mode_string`: Parses a file mode string whose letters select read, write or append, with optional update and binary flags.
- `open_file_by_mode_letter`: Reads the mode letter at a stored string position, opens the file with flags chosen by that letter, and fills a file record.
- `open_file_for_append_record`: Opens a file using the caller's flags with one extra fixed flag, seeks to its end, and fills in a file record with the mode byte.
- `seek_end_and_fill_record`: Seeks to the end of an already-open file, sets the mode byte, and fills in a file record.
- `fill_record_flag_checked_mode_two`: Sets the mode value to two and fills in a file record, with one field chosen by a flag test made through a subroutine.
- `open_file_read_or_update`: Opens a file with a flag derived from a stored byte, records a mode value, and fills in a file record.
- `open_file_record_mode_one`: Opens a file with the given arguments, records a mode value of one or 128 depending on a stored flag, and fills in a file record.
- `fill_record_with_read_mode`: Sets a mode value of one or 128 depending on a stored flag, then fills in a file record.
- `fill_record_mode_one`: Sets the mode value to one and fills in a file record.
- `create_file_for_writing`: Opens a file for writing with a flag chosen from a stored byte and creation settings, then records the mode and fills in a file record.
- `create_file_with_write_flags`: Opens a file with fixed create and write flags, records a mode of two or 128 depending on a stored flag, and fills in a file record.
- `fill_record_with_write_mode`: Sets a mode value of two or 128 depending on a stored flag, then fills in a file record.
- `fill_record_with_mode_two`: Sets the mode value to two and fills in a file record.
- `return_zero_status`: Returns zero with no other effect, reporting no file record.
- `fill_record_after_status_check`: Calls a subroutine and sets a record field depending on the flag it returns, then fills in the file record and returns it.
- `fill_record_without_flag_check`: Clears a field in a file record, then stores the mode value and pointers into it and returns the record.
- `clear_object_links`: Calls one subroutine with the record's link fields when its flag bits allow it.
- `lookup_open_table_entry`: Checks that an index lies within the table bounds and that the top bit of the entry's flag byte is set.
- `allocate_and_open_file_slot`: Finds a free entry in the file table, sets its access flags from the open mode bits, deletes the file first when requested, and opens it.
- `open_into_given_file_slot`: Uses an entry already chosen by the caller, sets its access flags from the mode bits, optionally deletes the file, and opens it.
- `open_file_entry_from_frame`: Sets access flags for a file entry chosen from the frame, optionally deletes the file, and opens it.
- `open_file_entry_marking_used`: Marks a file entry as in use, sets its access flags from the mode bits, optionally deletes the file, and opens it.
- `open_file_entry_append_mode`: Sets the write-access flag bit on a file entry, optionally deletes the file first, and opens it.
- `open_file_entry_write_flag`: Sets a write flag bit on a file entry, optionally deletes the file, and opens it for the caller.
- `open_file_entry_append_access`: Sets the append access bits on a file entry, optionally deletes the file, and opens it.
- `open_file_entry_combined_access`: Sets combined access bits on a file entry, optionally deletes the file, and opens it.
- `reject_invalid_open_mode`: Clears the entry's flag byte, sets the invalid-argument error code, and returns minus one.
- `delete_and_open_file_entry`: Optionally deletes the file, then opens it using the flags held in the frame.
- `open_file_entry_fixed_mode`: Opens a file using a fixed mode value without deleting it first, recording an I/O error and returning minus one if the open fails.
- `return_file_entry_index`: Returns the index stored four bytes below the frame pointer without other work.
- `open_with_fixed_access_flags`: Calls another routine with a fixed access value of 769 combined with the high bit of its flag argument, and passes the remaining bits as a third argument.
- `read_bytes_from_file_entry`: Reads a given number of bytes from a file entry's handle into a buffer through the system read call.
- `read_file_into_frame_buffer`: Reads bytes from the file handle held in the frame into a buffer and stores the byte count it returns in the frame.
- `write_bytes_to_file_entry`: Writes a buffer to a file entry's handle through the system write call, first seeking to the end when the entry's append bit is set.
- `seek_file_to_table_offset`: Seeks a file entry's handle to a position chosen from a global three-entry offset table, recording an I/O error if that seek fails, then seeks back to the start and returns the result.
- `close_file_entry`: Closes a file entry's handle unless a flag bit says not to, records an I/O error and minus one if the close fails, and clears the entry's flag byte.
- `delete_named_file`: Deletes a file through the system delete call, recording an I/O error and returning minus one if the delete fails, otherwise zero.
- `return_success_status`: Returns zero with no other effect, reporting success.
- `forward_delete_file`: Passes a single argument from the stack to a routine that deletes a file and returns.
- `rename_file_with_error_handling`: Calls a routine to rename a file using two arguments from the stack; if the result is zero, it calls an error routine, stores the error in a global variable, sets another global variable to 5, and returns -1, otherwise it returns 0.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, removes the stack frame with unlk, then returns to the caller.
- `open_default_file_handle`: Opens the file named by a global string in a fixed mode and stores the returned handle in a global variable.
- `read_one_byte_with_pushback`: Returns a previously stored byte from a one-byte buffer when one is pending, clearing the buffer.
- `read_byte_from_file`: Reads one byte from the file handle into a frame slot and returns that byte.
- `read_and_mask_byte`: Checks a byte at a global variable, and if it is non-zero, moves it to d0, clears the global variable, stores the byte at an offset of negative 1 from a6, and masks d0 with 255.
- `call_read_one_byte`: Calls another routine and then unlinks a6 before returning.
- `write_byte_to_default_file`: Writes its byte argument to the file handle, opening the default file first if the handle is unset, and returns the byte.
- `check_and_return_constant`: Checks a byte at a global variable, and if it is non-zero, sets d0 to 255 and returns.
- `store_byte_from_argument`: Copies the low byte of a value at offset 8 from a structure pointed to by a6 into a global variable and returns.
- `read_file_into_buffer`: Checks for signals, opens a file handle if necessary, reads up to 255 bytes into a buffer, and if the read returns a negative value, clears a byte at the calculated buffer position.
- `write_string_to_file`: Checks for signals, opens a file handle if necessary, calculates the length of a string from a stack argument, and writes that string to the file handle.
- `forward_two_args_call`: Forwards its two arguments to one subroutine routine and returns nothing.
- `read_buffered_byte_from_stream`: Returns the next byte from a buffered stream structure, refilling its buffer through a subroutine when it runs out.
- `read_buffered_byte_from_frame_stream`: Reads the next byte from the stream whose pointer is held in the frame, refilling its buffer through a subroutine when empty.
- `read_next_buffered_byte`: Returns the next byte from a buffered stream held in the frame, refilling it through a subroutine when empty.
- `fill_buffer_and_read_byte`: Marks a buffered stream as read-active, refills its buffer through a subroutine and then returns the next byte.
- `refill_and_read_stream_byte`: Marks a buffered stream as read-active, refills its buffer through a subroutine, and returns the next byte from it.
- `next_buffered_byte_no_refill`: Returns the next byte from a buffered stream's current position and advances it, calling a subroutine to refill only when its count is exhausted.
- `forward_stream_to_helper`: Passes a stream structure from the frame to one subroutine routine and returns nothing.
- `write_byte_to_buffered_stream`: Writes one byte into a buffered output stream, passing the data through a subroutine when the buffer count runs out, and sets error flag bits in the stream state when a write fails.
- `allocate_stream_buffer`: Allocates a buffer for a stream structure through a subroutine using a global size, storing its pointer and size and clearing the stream's flag bits and counters.
- `return_global_value_times_eight`: Loads a global word, shifts it left by three bits so it is multiplied by eight, and returns the result.
- `forward_value_twice`: Passes its 32-bit argument to one subroutine routine twice, as both arguments, and returns nothing.
- `allocate_block_from_free_list`: Walks a free list of memory blocks to find one at least the requested size, splitting or unlinking it and updating the free total.
- `forward_pointer_with_repeated_size`: Passes a pointer argument and a duplicate of the second argument to one subroutine routine, then returns nothing.
- `release_block_to_free_list`: Returns a block of the given size to a sorted free list, merging it with an adjacent block when they touch or linking it in as a new entry.
- `report_break_signal_status`: Checks signal bits reported by a system call and returns zero when neither of the two signal bits is set.
- `divide_signed_magnitude`: Divides the first value by the second using a restoring shift-and-subtract loop on magnitudes, then negates the quotient when the operands have opposite signs.

### Hitchhiker's Guide to the Galaxy, The - Release 58 (1984)(Infocom)[a][serial 851002][WB].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
