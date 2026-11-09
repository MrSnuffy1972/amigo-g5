# Game 2374 (trinity - release 11 (1986)(infocom)[serial 860509]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02374). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 350 of 350 named (42 author symbols), 350 of 350 described (63 by rule, 287 by the local model) |
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
| Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:Trinity | 86 | 4993 | 349 |
| Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:bootblock | 1 | 10 | 1 |
| Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:Story.Data (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- trinity - release 11 (1986)(infocom)[serial 860509]/Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:Story.Data: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 262144 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:Trinity

- `parse_and_execute_command`: Parses a command line string into up to 32 space-separated words, opens input and output streams for them, and dispatches to a command handler based on the first word, storing the resulting handler address in a table.
- `restore_context_and_free_memory`: Saves the incoming argument on the stack, calls a routine with it, then if the global at offset 48 is nonzero it calls the handler at offset -36 twice when the globals at offsets 32 and 64 are nonzero, switches to the base register, calls the routine at offset -132, then the routine at offset -378, and finally restores the saved argument, the stack pointer, and the registers before returning.
- `invoke_two_library_pointers`: Invokes the function pointer at offset -288 from the base register in a4, then invokes the function pointer at offset -372 from the same base register, and returns.
- `allocate_and_store_memory`: Allocates memory through the operating system and stores the returned pointer in the global variable at offset 40.
- `check_zmachine_version`: Verifies the Z-machine version stored in a global variable and, if it is not at least 79, writes the "Wrong Z-machine version" message and returns.
- `check_buffer_flag_return`: Loads a pointer from offset -54 of the frame in a6, reads the 16-bit word at that address into d0, and returns when bit 0 of that word is set.
- `swap_halfwords_and_shift`: The routine swaps the two 16-bit halves of the 32-bit value in D0, then shifts the result right by 7 bits.
- `shift_result_right`: Calls a routine to obtain a value in d0, then shifts it right by eight bits and once more by one bit before returning the result.
- `adjust_value_in_d0`: Masks the 11 low bits of the value in d0, adds 512 to it, stores the result back to d0, and returns.
- `return_value_via_pointer`: The routine returns the 16-bit value pointed to by a pointer in A0.
- `store_word_big_endian`: Writes a 16-bit value into the two bytes at the address register, high byte first.
- `advance_value_at_limit`: Saves an incoming value on the stack, compares it with a limit in the frame, and branches accordingly.
- `call_two_routines_in_order`: Calls a routine, shifts a register left by eight, and pushes the result on the stack.
- `read_byte_and_advance`: Reads one byte from a buffer whose base pointer is held in a global slot, at the index stored in the global block, then advances that index by one.
- `read_word_big_endian`: Calls the byte reader twice and returns the first byte shifted left by eight bits, combined with the second byte as the low part.
- `decrement_and_branch_value`: Decrements a 16-bit value in d0 and branches when it goes negative or reaches zero, otherwise calls a routine and branches on the result before loading the next word from the pointer in a4 and returning.
- `index_table_by_value`: Returns the word pointed to by a4 when the input is zero.
- `store_value_by_index`: If the value in d0 is zero it stores the value from d1 and returns.
- `shift_value_right_nine`: Returns the incoming value shifted right by nine bits.
- `return_high_byte`: Returns the high byte of the incoming 16-bit value.
- `shift_value_right_seven`: Returns the incoming value shifted right by seven bits.
- `add_words_d0_d1`: Adds the word in d1 to the word in d0 and then branches to the shared dispatch routine.
- `extend_and_divide_words`: Sign-extends the word in d0 to a long, divides that value by the word in d1, and branches to the shared dispatch routine.
- `extend_divide_swap_result`: Sign-extends the word in d0 to a long, divides that value by the word in d1, swaps the halves of the result, and branches to the shared dispatch routine.
- `compare_branch_d0_d1`: Compares the word in d1 with the word in d0, branches to one handler when d0 is less, and otherwise branches to another handler.
- `compare_greater_branch`: Compares the word in d1 with the word in d0, branches to one handler when d0 is greater, and otherwise branches to another handler.
- `scale_input_by_constant`: Masks the input to its low sixteen bits and multiplies the result by fourteen, returning the product.
- `read_table_byte_doubled`: Reads a byte from a table whose base is held in a global slot at offset 90 of the structure in a6, using an index returned by another routine, and returns that byte multiplied by two.
- `skip_byte_on_flag_bits`: Reads a byte from the address in a0 and advances the pointer, then returns early when bit 7 is set.
- `mask_and_skip_byte`: Reads a byte from the address in a0, keeps only its lower six bits, adds that value to the same address, and returns.
- `add_shift_and_move_d2`: Adds the word in d1 to the word in d0, calls the shift routine, calls the limit-checking routine, moves the value in d2 to d0, and branches to the shared dispatch routine.
- `store_word_from_registers`: Multiplies the value in d1 by two, adds it to d0, and uses the result as an offset from a global table at offset 90 of the structure in a6.
- `store_byte_at_indexed_address`: Adds the two 16-bit values in d0 and d1, then stores the byte in d2 at the base address plus that summed offset.
- `resolve_table_entry`: Calls scale_input_by_constant to compute a sixteen-bit product, reads a byte from a table using a doubled index, then reads and compares bytes from a table until a matching one is found.
- `read_byte_skip_loop`: Calls a routine to read a byte, masks the low six bits of that byte, compares it with the value in d1, and when the masked value is greater it loops back to repeat the read.
- `load_byte_check_bit7`: Loads a byte from the address in a0 and increments that pointer, branches when bit 7 of the byte is set, subtracts a long from the pointer in d0, and branches to the shared dispatch routine.
- `store_value_by_d0_index`: Stores a 16-bit value from d1 into memory selected by the 16-bit value in d0.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to -(a4), then returns.
- `swap_increment_lookup_store`: Swaps the two 16-bit values in d0 and d1, increments d0, calls the value-indexed lookup routine, then calls the index store routine in d0 before jumping to one of two handlers.
- `swap_and_index_loop`: Copies the value in d1 to d2 and the value in d0 to d1, calls a lookup routine, decrements d0, swaps d0 and d1, and when the swapped value is less than d2 loops back to repeat the lookup.
- `compute_offsets_and_call`: Computes two offsets from a base pointer stored in the frame and stores them into consecutive bytes of a structure, then calls another routine.
- `compare_byte_store`: Reads a byte from the address in a2 and compares it with the byte at the address in a1, returning when they match.
- `store_byte_stack_compare`: Computes a stack address in a0, branches when it differs from the value in a3, branches when the address in a1 is higher than a stored value, stores the byte in d0 at the pointer in a3, and branches onward.
- `discard_excess_words`: Increments a byte counter and compares it against the length of a string.
- `store_differences_and_call_packers`: Stores the difference between a2 and a global word at offset 2 of the structure in a4, then stores the difference between that global word and another global word at offset 3, clears a byte pointed to by a3, calls two routines to pack data, calls a third routine with the pointer in a4, and advances that pointer by four.
- `store_parsed_word_count`: Copies the byte at offset 14 of the structure in a6 into the second byte of the structure pointed to by the pointer at offset 136, then restores a4 from the stack and returns.
- `read_line_into_buffer_2`: Clears a counter at offset negative two hundred twenty-two of the frame, then reads characters through a routine, storing each into a buffer and advancing a pointer.
- `read_line_into_buffer`: Stores the character in d0 at the pointer in a2 and advances it, increments a counter at offset -222 of the frame, and loops while the pointer has not reached the end pointer in a3, then clears a flag at offset -162, reads characters through another routine until a carriage return or backspace, and calls another routine with the argument two when the character is a carriage return or backspace.
- `save_reg_and_branch`: Saves a data register, calls another routine, restores the register, sets a word at offset -162 of the frame, then branches into another routine without returning.
- `lowercase_string_in_place`: Calls two routines, then if a frame flag is set, converts each uppercase letter in the pointed-to string to lowercase by incrementing values between the constants, and finally returns the difference between two addresses.
- `compute_pointer_difference`: Subtracts the stack pointer in a1 from the address in a2, restores a1 through a3 from the stack, and returns the difference in d0.
- `clamp_value_in_table`: Computes a bit length from two global words, then walks through a table of values comparing an incoming value against each entry and clamps a result to a stored bound, returning early when the loop count is exhausted.
- `adjust_address_bound`: Adds the value in d3 to the address in a2 and returns immediately when the result is not below the stored value.
- `compare_and_restore_registers`: Subtracts the value in d3 from the address in a2, then compares d3 with a 16-bit value at offset 34 of the structure in a6.
- `branch_to_character_handler`: Immediately branches to the routine at offset 1450 without executing any other instruction.
- `emit_number_as_digits`: When the value in d0 is zero it stores a space at the pointer at offset -178 of the frame and returns, otherwise it converts the absolute value into its decimal digits, storing each one through another routine, and returns.
- `decode_packed_five_bit_tokens_2`: Decodes a run of packed five-bit tokens from a stack buffer, dispatching on each token value to store control codes, update a 16-bit accumulator in a global variable, and call another routine, then returns a 16-bit value from the stack.
- `restore_packed_pair`: Restores two 16-bit values from the frame at offsets -48 and -46, calls a routine to merge their upper bits into the first value, and then stores the results back to the same two locations.
- `chain_three_calls`: Calls a routine at offset b44, then calls a routine at offset b66, and finally branches to a routine at offset dd2.
- `store_padding_byte`: Stores the value 13 through a pointer at offset 178 when the word at offset 152 is nonzero, otherwise it returns without doing anything.
- `store_word_in_global`: Writes the 16-bit value in d0 into a global variable at offset 168 and returns.
- `set_output_stream_state`: Uses its argument to select a mode: when the value is zero it clears a flag word at offset -212 of the frame and returns, when the value is one and that flag is clear it sets the flag, saves the current position and table address returned by a routine into the frame, and returns, otherwise it returns without changing anything.
- `store_routine_result`: Calls a routine that leaves a value in d0, stores that value at offset 210 of the structure in a6, and returns.
- `set_flag_by_argument`: Decrements the argument and, when it becomes zero, sets a flag word at offset -160 of the frame and returns the argument; increments it and, when it becomes zero, calls a routine and clears the flag; otherwise sets the flag and returns.
- `decrement_and_branch_on_zero`: Loads a word from offset 4 of the pointer in a0 into d0, subtracts one three times in a row, and branches to offset 206, 334, or 658 of the structure in a6 each time the value reaches zero, otherwise falling through to offset 956.
- `increment_and_branch`: Increments the argument three times, returning after each increment when it reaches the constants at offsets 206, 334, and 658, then branches into another routine without returning.
- `set_output_buffer_flag`: Sets the flag at offset 98 of the structure in a6 to one and branches away.
- `clear_word_then_branch`: Clears the 16-bit word at offset 98 of the structure in a6, then branches to the routine at 3228.
- `set_output_flag_and_call`: Sets a flag at offset ninety-four of the structure in a6, loads a pointer from offset ninety-four, sets bit zero of a status byte at offset seventeen, calls a routine, then branches to another routine.
- `clear_output_flag_and_call`: Clears a flag at offset ninety-four of the structure in a6, loads a pointer from offset ninety-four, clears bit zero of a status byte at offset seventeen, calls a routine, then branches to another routine.
- `accumulate_offsets_then_branch`: Loads a 16-bit value from offset 4 of the pointer in a0 into d0, adds the long at offset 90 of the structure in a6, and stores the 32-bit result at offsets 90 and 86 of that structure.
- `flush_buffered_output`: Checks a flag at offset 152 of the structure in a6 and returns without doing anything when it is clear.
- `call_two_then_branch`: Calls two routines in sequence, then branches forward into a following routine without returning.
- `increment_or_forward_value`: For a non-zero argument other than one it returns the argument incremented by one, otherwise it calls a routine and then passes two saved global words to a further routine before returning.
- `decrement_and_call_if_zero`: Subtracts one from the argument and calls a routine when the result is zero, otherwise it returns the decremented value.
- `store_zero_based_position`: Reads two adjacent 16-bit global variables and adds a saved base offset to the first one when the stream flag at offset negative 212 of the frame in a6 is zero, then decrements both values by one and stores them into the fields at offsets negative 98 and negative 94.
- `jump_to_routine`: Jumps to a routine located at a different address in the same hunk.
- `execute_two_calls`: Saves a register on the stack, calls a routine at offset 14C8, calls another routine at offset 164, restores the stack, and returns.
- `process_packed_sequence`: Sets a flag word at offset -108 to one and saves the incoming 16-bit value at offset -106, then decrements a counter at offset -108 and pushes the frame's saved words onto the stack.
- `adjust_packed_index`: Subtracts two from the index in the global block and adds the result to the high part of a packed word, then masks the low part to nine bits.
- `restore_and_branch`: Pops a word off the stack, branches into the middle of another routine, and later restores the remaining registers from the stack before returning.
- `setup_and_store_result`: Calls two routines, and when either returns non-zero it branches forward.
- `call_routines_then_clear`: Calls three routines in sequence, sets register d0 to one, calls a fourth routine, clears d0, then branches forward into a following routine without returning.
- `decode_input_command`: Reads a byte from the input buffer and dispatches to a handler based on its value, using a table at offset 260 for values 128-175, another at offset 292 for 176-191, and a third at offset 308 for 192-255, while decoding multi-byte commands that push operands onto the stack.
- `clear_d0_jump_table`: Clears the 16-bit value in d0, loads the address of a global table into a0, and jumps to a shared routine.
- `decode_packed_five_bit_tokens`: Decodes packed five-bit tokens from a stack buffer, dispatching on the token value to emit control codes, build a 16-bit value from two nibbles, or update a running accumulator, and calls the character handler when the token is the end marker.
- `test_byte_then_branch`: Tests the byte in d0 and returns early when it is nonzero, otherwise loads the constant three into d0 and branches to the routine at 4806.
- `check_lowercase_range`: Compares the byte in d0 against 97 and 122, returning early when it falls outside that range.
- `check_uppercase_letter`: Checks whether the byte in d0 falls between the values 65 and 90, and loads 1 into d0 when it does, otherwise it returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_byte_against_table`: Compares the byte in d0 against successive entries of a table at a fixed address, advancing the pointer until a match is found or a zero byte is reached.
- `adjust_table_offset`: Computes an offset from a table address, adds 5 to it, and subtracts 26 from the result when the value exceeds 32, otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `pack_five_bit_codes`: Reads bytes from a source through a1 and pushes encoded values onto the stack, branching to a handler when a zero byte arrives.
- `pack_zchars_into_words`: Pads a buffer with the value 5 five times, then packs groups of three five-bit codes into three bytes, setting the high bit on the final byte, and returns.
- `allocate_and_process_buffer`: Adds 28 to the word in d1, sign-extends it, and calls a routine with that size, then stores the returned pointer and a second pointer at offsets 0 and 6 of a freshly allocated structure while recording the two arguments at offsets 16 and 12, clearing a counter and a flag byte, and returns.
- `tiny_moveq`: Small routine of 1 instruction that loads 13 into d0.
- `store_character_to_buffer`: Replaces a tab character in d0 with a space, then when a flag word at offset -152 of the frame is set it returns, otherwise it calls another routine when a flag at offset -148 is set and another when a flag at offset -146 is set, and finally stores the character at the pointer at offset -178 and advances that pointer.
- `process_buffered_input`: Returns immediately when a frame flag at offset -160 is clear, branches into another routine when the argument equals the constant thirteen, otherwise saves a register, calls a routine, updates buffer pointers and status bytes, restores the register, and returns.
- `clear_word_via_pointer`: Loads a pointer from a local variable at offset -106 of the frame and clears the 16-bit word at the address it points to, then returns.
- `reset_record_pointer_flag`: Saves a register, loads a pointer from the frame, and calls a routine with the record's first pointer.
- `branch_to_input_reader`: Immediately branches to the routine at offset 1742 without executing any other instruction.
- `append_character_to_buffer`: When the incoming character is a carriage return it updates a buffer's bookkeeping fields and returns.
- `loop_until_address_match`: Saves registers, copies the data register into a second register, and returns immediately when a frame flag is clear.
- `update_scaled_position_state`: Adds the upper bits of a packed word into a stored value and masks the lower bits.
- `find_or_allocate_table_entry`: Compares an incoming value with a stored counter and, if different, updates that counter and increments a usage tally.
- `find_least_used_table_entry`: Walks a table of four-word entries looking for the one with the smallest counter value, stopping at the -1 terminator, and returns the computed address of that entry's slot.
- `open_story_file_check`: Calls a routine that opens the story data file and returns when its result is non-zero.
- `allocate_and_check_memory`: Saves several registers on the stack, calls a memory allocation routine with the size in d0, and restores the registers.
- `format_memory_status_report_2`: Saves registers on the stack, pushes the argument in d0, calls a routine that formats a memory status report, restores the registers, and returns.
- `forward_word_to_caller`: Sign-extends the low word of d0, pushes the resulting value on the stack as an argument, calls a routine, restores the saved registers, and returns.
- `call_time_routine`: Saves several registers on the stack, calls a routine that returns the current time value, restores the registers, and returns the second output argument.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `combine_saved_word_pair`: Calls a routine, then combines the two saved global words at offsets 162 and 166 into a single 32-bit result in d0 and returns it.
- `save_word_pair_and_call`: Saves its two word arguments into global cells, then calls a routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `read_input_recursively`: Calls a routine to read a key code, then if a state word at offset -162 is set and the character is not a backspace or another state word is set, calls itself recursively before returning the character.
- `read_editing_key_code`: Sets two state words at offsets negative two hundred twenty and negative two hundred eighteen of the frame in a6 to one, then reads a key byte from another routine.
- `clear_local_word`: Clears the 16-bit word at the address held in the local variable at offset -220 of the frame, then returns.
- `call_with_data_register`: Saves several registers, pushes the data register onto the stack, calls a routine, cleans up the stack, restores the registers, and returns.
- `flush_when_line_full`: Computes the remaining line space by subtracting a saved offset from another saved value, decrements the result, and compares it with a counter in the frame.
- `reset_flag_then_call`: Clears the 16-bit word at offset 220 of the structure in a6, saves several registers on the stack, calls the routine at 1076, restores the registers, and returns.
- `flush_pending_output_range`: When a stream flag is clear it calls a routine, then subtracts two addresses and, when they differ and the start is not after the end, calls another routine with the difference before returning the result.
- `flush_and_increment_counter`: Calls a routine, then calls another routine with the constant thirteen.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `emit_string_return_result`: Passes each byte of a zero-terminated string in a0 to another routine until the terminator is reached, then returns zero unless a flag word at offset -160 of the frame is set, in which case it calls another routine and returns its result.
- `handle_buffer_flag_return`: When the pointer in a0 is not zero it calls a routine, then when a flag word at offset -152 of the frame is set it stores a carriage return at the buffer pointer and advances it, otherwise it reads a saved value and calls a routine.
- `check_printer_status_flag`: Reads a status byte from a structure pointed to by offset 90, and when bit zero is set it calls a routine that handles a printer error before returning.
- `handle_printer_error`: Calls a routine that sign-extends a value and pushes it on the stack, then tests the zero flag.
- `extend_word_then_call`: Sign-extends the 16-bit value in d0 to 32 bits, pushes it on the stack, calls the routine at 2564, discards the argument, tests the returned value, restores the registers, and returns.
- `stop_scripting_on_printer_error`: Clears bit zero of a byte at offset positive eleven of a structure loaded from a global variable and clears a word at offset negative ninety four of the frame in a6.
- `call_helper_if_result_nonzero`: Pushes a buffer pointer and its length onto the stack and calls a write routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `store_global_when_helper_zero`: Calls a routine that opens the story data file, and when its result is zero stores the global handle at offset 132 into the frame slot at offset -124 of a6, then returns the routine's result.
- `pass_then_clear_frame_slot`: Pops the handle from the frame slot at offset -124 of a6, passes it to a routine that closes it, clears that slot to zero, and returns the close result in d0.
- `save_call_restore_return`: Saves registers on the stack, loads the constant one into a register, calls a subroutine, restores the saved registers, and returns.
- `read_file_block_check`: Saves registers on the stack, calls a routine twice with swapped arguments, pushes several values, and calls a file access routine.
- `read_uppercase_letter_or_return`: Reads characters through a subroutine until it gets an uppercase letter or a carriage return, converting lowercase letters to uppercase before testing them.
- `check_first_answer_letter`: Locks a device and stores the returned handle, then reads a line into a buffer and scans it skipping spaces for the first character that is not a space, returning zero when that character is 'y' and otherwise returning one.
- `call_then_handle_zero`: Saves registers on the stack, pushes the arguments one and zero, calls a routine, then restores the registers.
- `open_video_mode_and_check`: Calls a routine that opens the video device and stores the handle at offset 132, then when the result is zero stores zero in d0 and branches to the shared exit that restores the saved registers and returns.
- `save_arg_then_call`: Pushes the value of d0 onto the stack, calls another routine, restores d0 from the stack, pops the saved registers, and returns.
- `call_file_with_swapped_args`: Saves registers on the stack, calls a routine twice with its arguments swapped between the two calls, then pushes several values and calls a file access routine.
- `report_disk_full`: Loads the address of a global table into a0, calls a routine to report the condition, restores the stack pointer and registers, and returns.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `forward_pending_value`: Pops the handle from the frame slot at offset -256 of a6, passes it to a routine that closes it, and returns the result in d0.
- `delete_named_file`: Calls a routine that deletes a file named in a global variable and returns its result in d0.
- `clear_and_branch_forward`: Clears the word in register d0 and then branches forward into the following routine without returning.
- `call_routine_with_one`: Loads the constant one into d0, saves the registers, pushes the value, and calls the routine at address five hundred twelve.
- `report_icon_file_error`: Saves registers on the stack, calls a routine that tests a global variable, and when its result is non-zero calls a second routine to report the error.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `read_and_emit_line`: Saves registers on the stack, loads a series of string addresses and calls a routine for each, then calls a routine to read a line into a buffer.
- `call_helper_with_zero_then_return`: Pushes a zero argument and calls a subroutine, then calls a second subroutine and returns the word left on the stack unchanged.
- `invoke_system_trap`: Stores the incoming argument in a global cell at offset 992, then executes the trap instruction with number one.
- `setup_io_request`: Stores the first parameter into the global variable at offset 52, the second parameter sign-extended into the global variable at offset 48, and the constant 3 into the global variable at offset 40, then calls the I/O routine with a pointer to that global variable and returns.
- `send_io_request`: Stores the constant 2 into the global variable at offset 88, a pointer to the global variable at offset 100, and the constant 1 into the global variable at offset 96, then calls the I/O routine with a pointer to that global variable and returns.
- `wait_for_message_return_byte`: Waits for a message on a port, fetches the message, saves its first byte, calls another routine, and returns that byte.
- `send_sequence_and_echo`: Stores the byte 155 and a zero byte on the stack, then passes them along with the first argument to another routine.
- `read_and_store_message`: Reads a byte from another routine, discarding it while it equals 155.
- `clear_word_and_branch`: Clears a word at offset negative 4 of the stack frame, then branches into the middle of another routine.
- `store_byte_and_terminate`: Reads a 16-bit value from offset -4 in the frame, uses it as an index into the buffer held at offset 8 in the frame, writes the byte at offset 15 in the frame to that buffer position, then writes a zero byte to the following buffer position and returns.
- `display_beep_on_code_two`: Compares the incoming parameter with the constant 2 and, when they are equal, reads a value from offset 46 of the structure at offset 142 and passes it to the display beep routine, otherwise it returns without doing anything.
- `return_current_time_value`: Calls the current-time library routine with two stack arguments and returns the value stored in the second output argument, without confirming the unit of that value.
- `dispatch_by_index`: Loads a word from offset ten of the structure in a6, extends it to a long, and branches when the value is greater than five.
- `call_with_constants_20_23`: Pushes the constant 20 onto the stack and calls another routine, then pushes the constant 23 and calls the same routine, before returning.
- `jump_into_routine`: Unconditionally branches into the middle of another routine, transferring control there without returning.
- `push_constants_and_call`: Pushes the constant 20 onto the stack, calls another routine, cleans up the stack, then pushes the constant 23 and calls the same routine again before returning.
- `send_control_sequence_argument`: Pushes the constant byte 155 and a zero byte onto the stack, calls a routine with the pointer argument and those two values, cleans up the stack, and returns.
- `compute_grid_cell_counts`: Reads two sizes and two margins from the structure at offset 142 to compute a column count and a row count, divides each by a cell size from the structure, clamps the column and row indexes to those counts, and returns the row index in the high word together with the column remainder in the low word.
- `reset_grid_cursor`: Sets the saved column limit at offset 162 to the cell count at offset 160 minus one and clears the saved row position at offset 166 to zero, then returns.
- `write_cursor_position_sequence`: Reads two adjacent 16-bit globals, increments each by one, formats each into a separate 32-byte buffer, copies both buffers into a 64-byte buffer, and sends the assembled cursor-position escape sequence to output.
- `advance_output_line`: Calls a setup routine, then when the column is before the last column writes a carriage return and line feed and passes them to another routine, otherwise it moves the column to the saved end, clears the row, and calls routines that handle the wrap.
- `send_control_sequence_byte`: Pushes the constant byte 41 and a zero byte onto the stack, calls a routine with the pointer argument and those two values, cleans up the stack, and returns.
- `clear_column_range`: Loops over columns from the first argument up to the second, setting the column and clearing the row, and calls two routines for each column, then restores the saved column and row and calls another routine.
- `clear_grid_region`: Clears the saved column and row to zero, calls two routines, moves the column to the last cell, and calls another routine.
- `forward_with_extended_count`: Passes its pointer argument and its 16-bit count, sign-extended to 32 bits, to another routine, then returns.
- `echo_character_with_backspace_handling`: When the character is a carriage return it calls a newline routine, and when it is a backspace it stores a space and a backspace then calls a routine with a length of three.
- `return_message_byte`: Reads a byte from another routine and returns it to the caller without modifying any data.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `display_more_prompt`: Sends a pad string and a prompt through output routines, waits for input, erases the prompt, then repeats the same sequence with a second prompt and returns.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `push_globals_if_nonzero`: Tests the word at offset 10 of the caller's frame and returns immediately when it is zero.
- `call_with_global_pair`: Pushes two global variables onto the stack and calls a routine with them as arguments, then cleans up the stack and returns.
- `trim_string_spaces`: Skips past any leading space bytes in the string pointed to by the argument by calling another routine for each one.
- `trim_trailing_spaces`: Counts the bytes of a string and then skips past any trailing spaces by zeroing them, shrinking the stored length each time.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `find_path_separator_end`: Searches the string for a slash and, when none is found, for a colon, then returns the position just after the match or zero when neither exists.
- `resolve_path_and_copy`: Calls a helper to find a path separator and, when it succeeds, clears the byte at the returned position and copies a string into a stack buffer and back, otherwise it forwards the found pointer to another routine.
- `forward_saved_pointer`: Loads the long pointer stored at offset negative four of the frame in a6 and passes it to another routine, then restores the stack and returns.
- `read_file_block_at_offset`: Seeks to the requested offset when the file handle or position differs from the last recorded values, then reads the requested byte count into the buffer, records the new position and status, and returns a zero on success, a one on a short read, or the I/O error code on failure.
- `seek_and_write_device`: Seeks the device by one less than the third argument and stores the result.
- `loop_with_counter_call`: Saves three frame values on the stack and calls a routine, storing its result at offset negative four.
- `compare_and_load_limit`: Loads a value from offset -4 in the frame, compares it with the value at offset 16 in the frame, and when they are equal loads the constant 999 into the result register, removes the stack frame held in a6, and returns.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `open_video_mode_handle`: Pushes the video mode register address and a device name, opens the device, stores the returned handle in a global variable at offset 132, and returns the error code if the handle is zero.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `open_graphics_mode_handle`: Pushes the graphics mode register address and a device name, opens the device, stores the returned handle in a global variable at offset 132, and returns the error code if the handle is zero.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `close_handle_return_zero`: Pops a file handle from the caller's stack frame, closes it through the close call, and returns zero.
- `delete_file_check_result`: Calls the delete routine with a device name, stores the result in a local frame slot, and returns zero when the result is zero or the result itself otherwise.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `lock_device_and_flag`: Locks a device and stores the returned handle in two frame slots.
- `set_flag_and_return_saved`: Sets a global word at offset 14 to one and returns the 32-bit value saved at offset 8 of the caller's frame.
- `open_story_data_file`: Opens the story data file named in the referenced text using the existing-file mode, stores the handle in a global variable at offset 132, and returns zero on success or the last I/O error code on failure.
- `close_handle_return_zero_2`: Restores the caller's frame pointer and pops the file handle argument from the stack, then calls the file-closing routine with that handle and returns zero.
- `place_disk_object_if_zero`: Tests a global variable and returns zero when it is nonzero.
- `place_disk_object`: Sets two fields of a disk object structure to a large negative constant, then calls a routine to place the disk object.
- `return_zero_result`: Clears the 32-bit value at offset 4 of the caller's frame and returns that cleared value.
- `write_with_elapsed_time_check`: Writes a buffer through the file write call and reads the system time before and after.
- `open_or_close_printer_handle`: With a zero argument it closes the stored handle if one is set and returns zero.
- `set_console_reset_state`: With a zero argument it clears a flag and closes the printer handle if one is open.
- `write_buffer_then_escape_sequence`: Writes the given buffer when its length is positive, and if that write returns zero or the length is not positive, sends a two-byte escape sequence of escape followed by an uppercase E. It returns the result of the final write.
- `format_memory_status_report`: Reads the available memory and clamps it against a stored limit, then builds a status line in a 64-byte buffer using the referenced labels for maximum, bytes free and bytes used, and returns the memory amount it chose to report.
- `allocate_tracked_memory`: Pushes a global key and a pointer to a global variable on the stack, then calls the tracked-allocation routine with the size in D0.
- `open_custom_screen_and_window`: With a zero argument it closes the window and then the screen if they are open.
- `setup_console_resources_and_icons`: Opens the referenced libraries and console device, temporarily changes a preference byte and restores it afterwards, creates a message port with a signal, copies a 48-byte block into a global, and loads the disk icon for a referenced name, returning zero on success or one of three distinct error codes.
- `release_console_and_icon_resources`: Reverses the setup routine by releasing, in order, the printer handle, the disk icon, the icon library, the remembered allocations, the message port, the signal, the console device, and the library opened at setup.
- `open_optional_file_argument`: Clears a global variable to minus one, then when the second argument is greater than one and the first character is F or f followed by a slash, opens the remainder of that path and clears the index on failure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_byte_from_d0`: It stores the low byte of the 16-bit value in d0 to the byte pointed to by a0, then returns.
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
- `_CurrentTime`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `_DisplayBeep`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_GetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `_OpenScreen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_SetPrefs`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `_AllocRemember`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `_FreeRemember`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
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
- `copy_string_including_terminator`: Copies each byte from the string pointed to by the second argument into the destination pointed to by the first argument, including the terminating zero byte, then returns the destination pointer.
- `copy_limited_string_return_base`: Copies bytes from the second argument to the first up to a remaining count, terminating the destination with a zero byte, and returns the first argument.
- `clear_byte_and_get_word_2`: Writes a zero byte to the memory location pointed to by the value at offset negative 4 of the stack frame, then returns the longword held at offset 8 of the same frame.
- `copy_string_to_offset`: Calls a helper with the first argument to find an offset, then copies a NUL-terminated string from the second argument to that offset in memory, and returns the first argument.
- `copy_limited_string_to_offset`: Calls a helper with the first argument to find an offset, then copies up to a remaining byte count from the second argument to that offset and terminates the result with a zero byte before returning the first argument.
- `clear_byte_and_get_word`: Writes a zero byte to the memory location pointed to by an offset in the stack frame and returns a word from a different offset in the same frame.
- `count_string_bytes`: Scans forward byte by byte from the string pointed to by the argument until it reaches a zero byte, counting the bytes examined, and returns that count as the string length.
- `format_unsigned_decimal_string`: Formats a number as decimal digits into a caller buffer, extracting each digit by repeated division by ten and writing at most the requested number of characters, then returns the count written excluding the zero terminator.
- `parse_signed_decimal_number`: Parses an optional plus or minus sign followed by decimal digits from a string, storing the signed value through a pointer and returning the number of characters consumed.
- `compare_byte_and_skip`: Loads the byte at the pointer in a0 and compares it to a byte at offset fifteen of the frame, then returns without doing anything when the two bytes differ.
- `load_byte_and_advance_pointer`: Loads the byte at the pointer in the stack frame into a register, advances that pointer, and returns zero when the byte is the end marker otherwise it returns.
- `forward_two_frame_values`: Loads the byte at offset fifteen of the frame in a6 into d0, pushes that value and the frame pointer from offset eight onto the stack, calls another routine, restores the stack, and returns.
- `find_matching_string_byte`: Walks the string pointed to by the stack argument comparing each byte to a value in the frame, storing the matching pointer and advancing the source pointer, then returns.
- `return_stored_value`: Loads the value stored at offset four in the stack frame into the return register, unwinds the stack frame, and returns it.
- `visit_global_table_indices`: Loops twenty times, calling a routine to obtain an index and passing the global table address offset by that index to a second routine, then loops a variable number of times passing each counter value to the second routine, and finally forwards two parameters to a third routine.
- `allocate_and_init_block`: Allocates a block sized to the input plus twelve bytes, stores that size at offset eight inside the block, and passes the global list head and the block address to another routine before returning the block address plus twelve.
- `call_and_store_pointer`: Sets up a stack frame, saves the incoming pointer argument, calls a routine with it, stores the returned pointer at offset four, and returns zero when that pointer is null.
- `return_minus_one`: Loads minus one into a register, copies it to the return register, unwinds the stack frame, and returns without performing any other operation.
- `free_linked_memory_blocks`: Walks a singly linked list whose head is held in a global, freeing each node with the size stored at its offset and following the next pointer until the list ends.
- `append_node_to_list`: Appends a node to a list whose head and tail pointers are held in the first structure, updating the next and head pointers and initializing the new node's next pointer.
- `split_arguments_and_open_streams`: Splits the argument string in place into up to 32 words, storing a pointer to each word and clearing the separators, then opens the input and output handles through DOS calls and records them.
- `compute_binary_weighted_sum`: Returns zero when either input is zero, otherwise iterates over 32 bits building a mask from the sign of the doubled value and conditionally subtracting the second input, then returns the modified first input.
- `empty_stub_function`: Contains a block of 31 instructions but ends by returning without performing any observable effect on memory, registers, or I/O.
- `check_pointer_and_increment`: Sets up a frame, stores zero at offset negative four, loads that value into a pointer, and returns immediately when the pointer is not the decimal value 224.
- `compare_pointer_to_limit`: Loads a pointer from offset negative four of a structure in a6, compares it with the decimal value 224, and returns immediately when they differ, otherwise falling through to the next routine.
- `forward_stored_arguments`: Pushes three values stored at offsets negative four, eight, and twelve from a structure in a6 onto the stack, calls another routine, and then returns.
- `classify_command_char_2`: Links a new frame, loads a pointer from offset 16 and returns immediately when the byte there is zero, otherwise pushing it and calling a routine.
- `classify_command_char`: Reads a character from a string pointer and, based on whether it is 'w', 'r', or 'a', calls another routine with a combined flag value and stores the returned handle and a status byte into the frame.
- `open_handle_with_flags`: Combines a stack argument with bits one and eight and passes it along with a constant and a frame pointer to a routine, storing the returned handle.
- `store_two_then_branch_2`: Loads the constant two into a register, pushes it and two zero words onto the stack, then pushes a frame pointer and calls a routine.
- `store_two_then_branch`: Loads the constant two into d0, stores it at offset 4 of the structure in a6, then branches to the routine at 504.
- `test_frame_flag_then_load`: Tests the byte at offset negative seventeen of the frame and branches away when it is zero, otherwise it loads two into a register and branches to another routine.
- `open_handle_single_flag`: Combines a stack argument with bit zero and passes it with a frame pointer to a routine, storing the returned handle.
- `store_one_then_branch_2`: Tests the byte at offset negative seventeen of the frame in a6 and branches away when it is zero.
- `store_one_then_branch`: Loads the constant one into d0, stores it at offset 4 of the structure in a6, then branches to the routine at 504.
- `test_frame_byte_zero`: Tests the byte at offset -17 of the frame and returns without doing anything when that byte is zero, otherwise it loads the value 2 into d0 and falls through to the following code.
- `open_handle_multi_flag`: Combines a stack argument with bits eight, nine and zero and passes it with a constant and a frame pointer to a routine, storing the returned handle.
- `store_one_then_branch_alt`: Tests the byte at offset negative seventeen of the frame in a6 and branches away when it is zero.
- `process_stream_record`: Stores the value 2 at offset -4 of the structure in a6, then calls another routine to fetch a record.
- `return_zero_value`: Loads zero into d0, restores the stack pointer, and returns the value zero.
- `link_stream_record`: Calls a routine to fetch a record, and when the byte at offset 1 of the derived address is zero stores zero at offset 12 of the record pointed to by the value at offset 16 of a6, otherwise sets bit 2 of the byte at offset -1 of a6 and links the record, then copies two saved words into the record and returns the record address.
- `clear_stream_record_fields`: Clears the record's count word, stores the incoming bytes into its state fields, links its buffer, and returns the record address.
- `clear_object_links`: Calls another routine with a negative value and a structure pointer when bit one of a byte at offset 16 is set, then calls another routine with two fields when bits two and three of that byte are clear and a longword at offset 12 is nonzero, clears those two longwords and the byte, and finally calls another routine with the byte at offset 17.
- `lookup_open_table_entry`: Calls a signal-dispatching routine, then checks that an index lies between zero and a saved table limit.
- `find_free_handle_slot`: Loops through a global table of handles up to a limit, calling a routine each iteration and stopping when it finds an empty slot, then when the limit is reached it sets a status byte at offset -20 to 24 and returns minus one, otherwise it stores status bits into the found slot and returns the slot index.
- `check_and_store_handle`: Loads a pointer from the frame at offset -4 into a data register and compares it with a global variable.
- `open_file_and_store_handle`: Calls a routine to open a file, combines the returned handle with a global status flag at offset 12 of the frame, records the handle at offset -12, sets status bits based on the file's state, then opens the file and stores its handle or records an error and returns minus one.
- `open_file_by_mode`: Sets an error flag in a slot's status byte, then based on a mode value sets additional status bits and opens a file, returning its handle or an error code when the open fails.
- `set_flag_then_branch_2`: Tests bit three of a byte pointed to by a pointer at offset positive twelve of the frame in a6.
- `set_flag_then_branch`: Sets bit five of a byte pointed to by a pointer at offset negative twelve of the frame in a6, then branches to the routine at address one hundred seventy.
- `load_eight_if_bit_set`: Checks bit 3 of the byte at offset 12 of the structure in a6, and if that bit is set loads the decimal value 8 into d0 before returning.
- `set_flags_then_branch`: Sets bits five and six of a byte pointed to by a pointer at offset negative twelve of the frame in a6, then branches to the routine at address one hundred seventy.
- `fail_slot_invalid_mode`: Clears the slot's status byte, records an invalid-argument error code in the global byte, and returns minus one.
- `check_flag_then_call`: Tests bit 1 of the byte at offset 14 of the structure in a6 and returns early when it is clear.
- `open_video_handle`: Stores the value 1005 in the local slot at offset -8 of a6, opens the file named by the argument at offset 8, stores the handle at offset 2 of the address at offset -12, and when the handle is zero records the I/O error and error code then returns -1, otherwise returns the value at offset -4.
- `return_saved_value`: Loads the value from the local variable at offset -4 of the frame into d0, restores the stack pointer, and returns it.
- `clear_status_flag_and_recurse`: Sets bit fifteen and clears bit fifteen of a first argument while setting bits zero through eight, then passes the modified and unmodified arguments along with a frame pointer to a routine and returns its result.
- `read_from_open_file`: When an index is valid it looks up a file handle and reads from it into a buffer, recording an error code when the read fails.
- `read_file_handle_buffer`: Reads from a file handle into the caller's buffer and length, storing the returned byte count, and records an error code when the read fails.
- `write_to_handle_checked`: Checks whether the argument is positive, obtains a pointer from another routine, and if the pointer is not null, checks a bit in the value at that pointer to decide whether to call another routine before writing data; if the write fails, it records an error status and returns minus one.
- `seek_file_with_offset_table`: When an index is valid and below three it seeks a file to an offset taken from a global table, records any I/O error, then seeks back to the start and returns the result.
- `close_file_entry_handle`: Fetches a file entry pointer and returns minus one when it is missing.
- `delete_file_check_result_2`: Deletes the file named by the argument, and when the result is zero records the I/O error and error code then returns -1, otherwise returns zero.
- `return_zero_value_2`: Loads zero into the return register, unwinds the stack frame, and returns without performing any other operation.
- `open_file_handle`: Opens the file named in the global text buffer using a fixed mode value and stores the resulting handle in a global variable.
- `read_single_file_byte_2`: Calls a signal-dispatching routine and returns early when a global flag is set.
- `read_single_file_byte`: Reads a single byte from the file handle into a stack slot and returns that byte value.
- `write_byte_to_default_file`: Ensures a default file handle is open, then writes the byte argument to that handle and returns the byte, unless a branch at offset 12 is taken, in which case it returns zero.
- `forward_two_args_call`: Sets up a stack frame, pushes the second argument followed by the first argument, calls a routine with them, and returns nothing.
- `mask_and_check_status_byte`: Loads the pointer at offset 8 of the frame into a0, reads the byte at offset 16 of that structure into d0, masks bits 5 and 6 of it, and returns zero when the masked result is zero, otherwise it loads the value 255 into d0 and returns.
- `check_status_bit_and_store_2`: Checks a structure flag and a status bit, returning zero when either is set.
- `check_status_bit_and_store`: Loads a pointer from offset 8 of the frame into a0, reads the byte at offset 10 of that structure into d0, and when bit two of that byte is set it stores one at offset -8 of the frame and branches to offset 128, otherwise returning.
- `check_status_bit_and_return`: Loads a pointer from the frame at offset 8 into a pointer register and reads the byte at offset 10 of that structure into a data register.
- `read_file_and_update_buffer`: Sets a status bit in the structure, calls a routine to read from a file, sets additional status bits based on the read result, advances the buffer pointers, then decrements a counter and returns a byte from the buffer or from another routine.
- `decrement_pointer_value`: Loads the longword at offset eight of the pointer in a0, decrements the value it holds by one, stores the result back, and returns zero when that value was zero.
- `pass_pointer_to_helper`: Loads the longword at offset eight in the stack frame and passes it to a helper routine, then returns.
- `write_byte_with_retry`: Writes a byte from its first argument into a buffer tracked by a status byte and a count field, updating the buffer position and counter, and retries through a recursive call when the counter underflows, setting status bits from the result.
- `reset_target_flags`: Returns zero immediately when the structure already has a target and no reset flag is set.
- `shift_global_variable_left`: Loads a global variable, shifts it left by three bits, unwinds the stack frame, and returns the result without performing any other operation.
- `call_find_node_by_distance_twice`: Loads a 32-bit value from offset eight of a pointer in a6, pushes it onto the stack twice as two separate parameters, calls a node-allocation routine, and returns.
- `find_node_by_distance`: Returns zero when the input is less than one, otherwise it allocates a block sized to the input plus twelve bytes and stores the size at offset eight.
- `call_with_second_repeated`: Sets up a stack frame, pushes the second argument followed by the first argument, calls a routine with them, and returns.
- `insert_or_update_list_node`: Computes a node offset from a global counter, then inserts the node into a list anchored by a global variable or updates an existing node's accumulated value depending on the traversal.
- `check_signals_and_dispatch`: Signals the two given bits and returns zero when neither is set, otherwise checks a global flag and calls another routine with 94, then with 67 or 68 depending on which bit was set, then with 10, then loops from zero up to a global count calling another routine with each index, and finally calls another routine with two zeros and returns its result.
- `compute_signed_magnitude_division`: Divides two register inputs by a restoring division loop over 32 bit iterations on their magnitudes, giving the quotient the sign of the xor of the inputs, and returns zero when the divisor is zero.

### Trinity - Release 11 (1986)(Infocom)[serial 860509].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
