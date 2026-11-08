# Game 518 (King's Quest Iv - the Perils of Rosella): code recovery summary

## Functions (part 2 of 2)

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 1 of 4).adf:prog
(continued)

- `clear_global_word`: Clears a 16-bit value in a global variable using a pointer or offset relative to a register, then returns.
- `conditional_store_and_set_flag`: Loads a pointer from a structure, checks if a certain word is zero, and if not, pushes a word from that pointer onto the stack before calling another routine, then checks if a word at a pointer in d0 is zero, and if so, sets a global variable to one.
- `clear_two_byte_field_3`: The routine clears a word at an offset from a pointer in a6 and then returns.
- `store_word_from_routine`: The routine calls another routine and stores the returned word into a global variable via a pointer offset.
- `copy_word_from_result`: The routine calls another routine and stores a 16-bit value from the result into a global variable via an offset from a register.
- `copy_two_bytes_from_callee`: The routine calls another routine and stores a two-byte value returned in a register from that call into a global variable via a pointer offset.
- `combine_three_values`: Loads a pointer from a structure, calls another routine with two values from that structure, calls another routine with the result and a third value from the structure, and stores the result in a global variable.
- `combine_two_values`: Retrieves two values from a parameter, calls another routine with the first, and then calls a second routine with that result and the second value, finally storing the result in a global variable.
- `combine_two_parameters`: Retrieves two words from a parameter structure, calls another routine with the first word, and then calls a second routine using the result of the first call and the second word, finally storing the result in a global variable.
- `traverse_linked_list_and_store_result`: Recursively follows a pointer located at an offset of 4 from the input parameter, then stores the final returned value into a global variable.
- `check_result_and_set_flag`: Calls another routine with a value from a structure, calls another routine with the result, and if the result is non-zero, calls another routine with a value from a structure and calls a third routine, then stores a boolean flag in a global variable.
- `store_returned_value_3`: The routine calls another routine and stores the returned 16-bit value into a global variable via a pointer offset from a register.
- `store_result_from_two_calls_2`: Calls another routine, then calls another routine using the result of the first call as an argument, and stores the final 16-bit result into a global variable.
- `store_computed_word_2`: Calls another routine twice, using the result of the first call as an argument to the second, and then stores the resulting 16-bit value into a global variable via a pointer offset from a register.
- `iterate_and_call_routine`: Decrements a value from a table entry and loops, calling another routine with a pointer and a value, then updates a global pointer and a counter.
- `decrement_loop_and_store_result`: Repeatedly calls another routine with a pointer from a global variable and a value built from a table entry, then decrements a short counter and advances a pointer by four bytes until the counter is non-positive, then calls the same routine once more with a single argument and stores the returned 16-bit value into a global variable.
- `pass_result_to_next_routine`: Calls a routine using a 16-bit value from a structure pointed to by a parameter, then calls another routine passing the low word of the first routine's return value as a parameter.
- `iterate_and_update_list`: Pushes a 16-bit word from a pointer retrieved from a global variable onto the stack, calls another routine, and returns.
- `extract_and_clamp_value`: Loads a 16-bit value from a structure, sign-extends it, and branches based on its value.
- `branch_on_register_value`: Compares a register to a constant, and if it is less than that constant, shifts the register and jumps to an address or a value loaded from a table.
- `unl_a5_only`: The routine clears the unused status register and then returns.
- `check_pointer_value`: Checks if a word at a pointer in a global variable is equal to one, otherwise it branches to another location.
- `store_word_from_table`: Reads a 32-bit value from a table, extracts a 32-bit word from it, passes the word to another routine, and then passes the result to a second routine before storing the returned 16-bit value into a global variable.
- `update_delta_and_copy_value_2`: Loads a pointer from a register, then subtracts a global variable from a value read from that pointer, stores the result in another global variable, and branches if the result is not zero.
- `update_delta_and_copy_value`: The routine reads a global variable, subtracts another global variable from it, stores the result as a short into a third global variable, then copies the first global variable into the second global variable.
- `parse_string_and_set_value`: Calls a routine to get a pointer, calls another routine, and if the return value is certain, calls a third routine and sets a local variable to 4.
- `set_local_value`: Writes the constant three to a local variable and branches to another routine.
- `branch_on_byte_comparison`: Compares a byte at an offset from a register to a constant, and if it is not equal, increments a pointer and writes a byte to that pointer before branching.
- `set_bytes_and_branch`: Writes a byte and a word to a location relative to a register and then branches to another routine.
- `parse_string_and_expand_arguments`: Reads a character from a pointer, checks if it is an alphabetic character, and if so, copies it to a local buffer while updating a length counter.
- `expand_argument_and_call`: Pushes a word from a pointer onto thestack, calls another routine, moves a value to a stack offset, pushes a pointer and a value, calls another routine, and then extends d0 and adds it to a stack-based value.
- `update_pointer_with_result`: Pushes a word from a pointer onto the stack, pushes a pointer onto the stack, calls another routine, adjusts the stack pointer, extends the return value, and adds it to a local variable before branching.
- `parse_string_and_expand_arguments_2`: Increments a counter and checks if it exceeds a certain limit, then loops to read characters from a source pointer and write them to a local buffer.
- `store_computed_word`: Calls a routine, uses its return value as an argument for another routine, and stores the resulting 16-bit value into a global variable via a pointer offset from a register.
- `traverse_linked_list_and_store_result_2`: Recursively follows a pointer located at an offset of 4 from the input parameter, then stores the final returned value into a global variable at a fixed offset.
- `process_table_value`: If a value at a pointer in a5 is two, it calls a routine twice with halves of a returned word, then calls another routine and stores the result in a global variable; otherwise, it calls a routine with a pointer from a structure, then calls the same sequence of routines and stores the result in a global variable.
- `store_word_from_table_2`: Reads a word from a table, splits a 32-bit value into two 16-bit halves, passes each half to another routine, and then calls another routine to combine the results and stores the result in a location relative to a register.
- `store_computed_string_length`: Calls a routine, uses its return value as an argument for a routine that calculates a string length, and then stores the resulting 16-bit value into a global variable via a pointer offset from a register.
- `extract_and_combine_words_2`: Copies a word from a pointer to a global variable, then checks a value in a table to decide whether to extract and combine words using several routine calls.
- `extract_and_combine_words`: Reads a word from a table, splits the resulting 32-bit value into high and low words, and passes both words to another routine.
- `process_parameter_and_branch`: Loads a pointer from a structure, calls another routine with a modified parameter, and then branches to another location.
- `increment_pointer_until_null`: Increments a word pointed to by a pointer stored at an offset from a register, looping until the pointed-to byte is zero, then calls another routine with the updated pointer value and stores the returned word into a global variable.
- `store_word_from_pointer_and_chain_calls`: Stores a word from a pointer into a global variable, then calls another routine and uses its result to call two more routines with derived values.
- `check_and_set_value`: Checks if a value at a pointer in a register is one, and if a certain byte at a pointer in a register is not zero, writes a word to a stack-relative location and branches.
- `clear_word_and_call_twice`: The routine clears a word at an address derived from a register and then calls another routine twice with the cleared value as argument.
- `store_word_from_pointer`: Stores a word from a pointer offset into a global variable and then calls another routine twice, passing a value obtained from the first call as an unable argument to the second.
- `store_returned_value_4`: Calls a routine and stores the returned 16-bit value into a global variable via a pointer offset from a register, then returns.
- `copy_byte_to_global_and_maybe_write_back`: Reads a byte from a location computed from a table index and a value returned by another routine, stores it into a global variable, and if the first element of the parameter array equals three it writes a byte from the parameter array back to the same computed location.
- `select_value_and_call`: Initializes a stack variable to 1, updates it with the third word of a pointer if the first word is 2, and then calls another routine with the resulting value.
- `traverse_pointer_list_and_call_handler`: Loads a pointer from a structure, updates a word at a relative offset, and calls another routine if a certain condition or a non-zero return value is met; otherwise it returns.
- `unlock_address_register_a5`: Clears the unable-to-interrupt or unable-to-interrupt-related flag or state associated with address register a5 and returns immediately.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 1 of 4).adf:AMIGASND.DRV

- `dispatch_deferred_callback`: Checks a one-bit flag in a global variable and, if clear, stores a 16-bit value from d0 into a global variable, calls a routine via an indirect pointer at an offset from a value in d0, and then clears the flag.
- `initialize_sound_command_header`: Initializes a sound command structure by clearing certain fields, setting a default timing or offset, and calculating a scaled timing value based on a length field.
- `initialize_sound_command`: Clears a byte at a pointer in a0 and writes a word to an offset of 18 in a structure pointed to by a2, then branches to another routine.
- `parse_sound_command_block`: Parses a sound command block by calculating lengths, setting up timing or offset values in a structure, and copying or modifying bytes in a buffer or sound stream.
- `decode_sound_command_block`: The routine initializes a sound command block in a caller-provided structure, decodes a stream of bytes from a pointer using a mask from a global variable to select up to four bytes, writes the selected bytes into a 16-byte buffer and terminates it, clears several command fields, and returns a status word from the structure.
- `check_bell_and_write_byte`: The routine checks if the first byte pointed to by a pointer is a bell character and a byte at an offset from another pointer is zero.
- `store_byte_conditional`: The routine checks a global variable for the value 4 and, if it is not equal, stores a byte from a pointer into a byte in a table addressed by a global variable and an unable to be determined index register.
- `calculate_pointer_difference`: Loads a byte from a pointer in a0, and if it is not equal to 127, calculates the difference between a pointer in a0 and a pointer from a table in a2, then stores the result as a word at offset 46 of the structure in a2.
- `store_word_to_offset`: Stores a word from d0 into a word at an offset of 24 from a pointer in a2.
- `evaluate_audio_state`: Checks a byte at an offset from a register, and if it is not zero, checks a byte at another offset; if that byte is not zero and a certain word is not certain, it calls another routine and branches.
- `set_sound_state_initial`: Loads a pointer into a1, clears a word at an offset of 26 from a1, writes a constant word to an offset of 24 from a2, writes a constant byte to an offset of 126 from a4, and branches to another routine.
- `configure_audio_state_2`: Compares a byte at an offset of a structure in a4 to a constant, and if equal, checks a pointer, writes a constant, and calls another routine before branching.
- `configure_audio_state`: Compares a byte at offset 126 of a structure in a4 to 3, and if equal, clears a byte at offset 53 of a structure in a2, calls another routine, calls another routine, and writes a byte to offset 126 of that structure; otherwise, it branches to a previous instruction.
- `decrement_and_clear_state`: Checks if certain bytes or words are non-zero; if so, it decrements a byte or a word in a structure or a global variable, otherwise it clears certain bytes in a structure and branches.
- `clear_sound_state`: Calls another routine and, if a global variable is zero, writes a constant word and a constant byte to specified memory locations.
- `parse_audio_stream_header`: Parses an audio stream header by checking a flag byte, decrementing a counter or a global counter, and then enters a loop that reads a pointer-based stream, updating a volume or offset or skipping bytes based on certain bit patterns and table lookups, while also updating a counter or clearing a value in a structure.
- `check_audio_buffer_status`: Checks if a 16-bit value at offset 48 of a structure in a2 is zero or if a byte at offset 52 of a2 is zero, otherwise it clears a 16-bit value at offset 48 of that structure and jumps to another instruction.
- `decrement_structure_value`: Decrements a word at offset 48 of the structure in a2 and then jumps to another instruction.
- `store_bytes_and_adjust_offset_2`: Compares a 16-bit value at an offset of a structure in a2 to a constant, and if not equal, otherwise clears a 16-bit value at that offset and branches.
- `parse_audio_command_stream`: Parses a stream of encoded audio commands, updating a pointer and a counter in a structure, and handles command-specific logic such as skipping bytes or updating a table or a pointer-difference value.
- `increment_audio_pointer`: Compares a byte in d6 to a constant, and if it is not equal to either constant, increments a pointer in a0 twice and increments a word at an offset of 50 from a2 before branching to another routine.
- `branch_on_d7_value`: Compares d7 against certain constants and, if d7 equals 8, writes a word to a structure in a2 before branching; otherwise it branches or returns.
- `compare_and_increment_pointer`: Compares d7 to zero and, if not equal, increments a byte at the address in a0 and checks if that byte is 247, otherwise it branches to another routine.
- `increment_pointer_conditionally_2`: Compares d7 to a constant, and if it is not equal, increments a pointer in a0 and branches to another routine.
- `increment_pointer_conditionally`: If d7 is equal to 3, increments the pointer in a0 and then jumps to another instruction; otherwise it jumps to another instruction.
- `check_audio_byte`: Compares a byte at an offset of 125 from a pointer in a4 to d5, clears a word in a structure in a2, and if d6 equals 144, calls another routine.
- `test_byte_and_branch`: Compares a byte in d6 to 128 and, if not equal, calls a routine and then branches to another instruction.
- `check_audio_condition`: Compares a byte in d6 to a constant, and if it is not equal, calls a library function and branches to another routine.
- `process_audio_buffer`: If a certain byte is not equal to 192, calls another routine, then clears d0, loads a byte from a0, and if that byte is at least 240, increments a0 and adds that byte to a word at offset 48 of the structure in a2 before jumping to another instruction; otherwise it jumps to another instruction.
- `branch_to_subroutine`: Calls a routine and then branches to another instruction.
- `store_bytes_and_adjust_offset`: The routine writes two byte values from registers into two consecutive bytes of a structure pointed to by a pointer in a register.
- `reset_sound_driver_state`: Writes a constant word to an offset of 24 from a2, copies a word from an offset of 46 from a2 to an offset of 14 from a2, calls another routine, writes a constant byte to an offset of 53 from a2, clears d0, writes d0 to an offset of 48 from a2, and returns.
- `clamp_and_scale_value`: If a 16-bit value at an offset in a structure pointed to by a2 exceeds 64, it sets that value to 64, then if a flag byte at another offset in that structure is zero, it copies the clamped value scaled by four into a 16-bit field in a second structure pointed to by a4.
- `restore_saved_word_and_call`: The routine saves a word from a structure pointed to by a2, clears that word, calls another routine four times, then restores the saved word.
- `set_audio_flag`: Calls another routine, writes a constant byte to an offset of 53 from a2, and returns.
- `set_flag_and_byte`: Sets a byte at an offset in a structure pointed to by a2 to 1 and another byte at a different offset in the same structure to 8.
- `configure_audio_channel`: Calls a routine via a pointer, then writes certain values from a structure pointed to by a0 to audio channel 0 hardware registers, including a scaled value, and calls another routine while disabling and re-enlagling interrupts and DMA.
- `configure_audio_channel_one`: Disables interrupts and DMA, then copies a new period and volume or length from a structure pointed to by a0 into the audio channel one registers, calculates a new period using a value from a4 and a multiplier from a0, sets a fixed length and volume, and calls another routine before re-enabling interrupts and DMA via a flag update in the structure.
- `stop_audio_channel_two`: Calls a routine via a pointer, then writes certain values from a structure pointed to by a0 to audio channel 2 hardware registers, including a scaled value, and calls another routine while disabling and re-enabling interrupts and DMA.
- `configure_audio_channel_three`: Calls a routine via a pointer, then writes certain values from a structure pointed to by a0 to audio channel 3 hardware registers, including a scaled value, and calls another routine while disabling and re-enabling interrupts and DMA.
- `set_audio_dma_control`: Writes a mask derived from an input value to a global variable and then writes a shifted version of that mask to another global variable.
- `process_audio_buffer_header`: Saves registers, loads a pointer, and if a byte at that pointer is not zero or three, calculates an offset, and if a certain condition is met, copies a byte from a table and calls another routine or modifies a counter before branching to another routine.
- `update_audio_channel`: Compares d1 to a constant, and if not equal, calculates an offset, checks a byte in a table, and if certain conditions are met, copies a byte, calls another routine, and potentially updates a counter or increments a pointer before branching to another routine.
- `decrement_audio_counter`: Subtracts a byte from an offset of 20 from a1, compares a byte at an offset of 20 from a1 to a byte at an offset of 24 from a1, and if the first is not less than or equal to the second, increments a counter and decrements a byte at an offset of 52 from a1 before branching to another routine.
- `clear_audio_registers_in_loop`: Calls another routine, then enters a loop that clears a byte at a certain offset in a structure, increments two counters, and repeats until a counter reaches 4, then restores registers and returns.
- `write_audio_register_value`: Multiplies a value from a global variable by a value from a register, shifts the result, and stores it to a hardware audio register whose address is derived from another register.
- `set_audio_channel_volume`: Multiplies a value from a global variable by a value from a register, shifts the result, then multiplies that by a byte from a table indexed by a register, shifts again, and stores the result to a hardware audio register whose address is derived from another register.
- `clear_sound_flags`: Saves registers, checks a bit in a byte at an offset of 165 from a4, and if set, calls two different routines twice, then repeats this for other bits, clears the byte at offset 165, and returns a concatenated value of registers.
- `initialize_audio_parameters_2`: If a word at a certain offset in a1 is zero, it sets that word to one and writes a constant word to offset 150 of a0, then returns.
- `initialize_audio_parameters`: If a word at a certain offset in a1 is equal to 1, clears a word at offset 26 of a1 and writes constant words to offsets 150 and 154 of a0; otherwise it returns.
- `copy_sound_driver_fields`: Copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields at fixed offsets within a destination structure pointed to by a0, then writes constant 16-bit values into two other fields of the destination structure.
- `initialize_header_field_if_zero`: Checks if a word at a certain offset in a structure is zero, and if so, writes a one to that location and writes a constant word to an offset in a structure, otherwise it returns.
- `initialize_header_fields`: If a word at a certain offset in a1 is equal to 1, clears a word at offset 26 of a1 and writes constant words to offsets 150 and 160 of a0; otherwise it returns.
- `copy_sound_header_fields`: Copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes constant 16-bit values into two other fields of the destination structure, then returns.
- `reset_sound_parameter_on_zero`: Checks if a word at a certain offset in a structure is zero; if so, it writes a one to that same location and writes the constant value 512 to an offset in a structure, otherwise it returns.
- `initialize_sound_parameters`: If a word at a certain offset in a1 is equal to 1, clears a word at offset 26 of a1 and writes constant words to offsets 150 and 512 of a0; otherwise it returns.
- `copy_sound_parameters`: Copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes the constant value 512 into two other fields of the destination structure, then returns.
- `toggle_sound_enable_flag`: Checks if a word at the pointer in a1 is zero; if not, it writes one to that location and writes a constant to a global memory address, otherwise it returns.
- `update_sound_parameters`: If the word at a certain offset in a structure pointed to by a1 is not one, it clears a word at offset 26 of that structure and writes certain 16-bit values to a destination structure pointed to by a0.
- `copy_sound_params`: Copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes the constant value 1024 into two other fields of the destination structure, then returns.
- `execute_cleanup_callbacks`: Calls a routine or a series or routine pointers passed via a6 or a4, and then calls another routine with a single argument passed via a1.
- `update_audio_state_and_sync`: Saves a 16-bit value from a structure, calls another routine, and if the result is not three, sets a flag byte at offset 52 of the structure, then calls another routine repeatedly until the 16-bit value at offset 14 of the structure no longer exceeds the saved value, clears the flag byte, restores the saved 16-bit value, and calls another routine.
- `increment_pointer_conditionally_3`: Tests a byte at offset 52 of a structure in a2 and, if it is not zero, increments the pointer in a0 before returning.
- `process_buffer_bytes`: Clears d3, loads a byte from the address in a0 and increments a0, then loads a byte from the new a0 address into an offset 40 of a4, and if that byte is not zero, decrements a0 by 2 and branches to another routine.
- `update_sound_parameters_2`: Calculates a pointer based on a register and a global variable, writes a value to that pointer, updates a bit in a global variable, and calls another routine or writes a value to a global variable before returning.
- `set_flag_and_write_byte`: Sets a bit in a byte located via a pointer, writes a 16-bit value from a register to a certain offset in a structure, calls another routine, and then stores a byte from a register into a table indexed by a short parameter.
- `branch_on_byte_test`: Tests a byte at an offset in a structure and, if it is zero, otherwise it branches to another instruction or increments a pointer and branches to another instruction.
- `compare_and_copy_byte`: The routine compares a byte pointed to by a pointer with a byte from a table indexed by a word offset.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `initialize_audio_channels`: Allocates memory for four audio channel control blocks by copying data from a global table, initializes their fields, and installs interrupt handlers for each channel.
- `configure_sound_channel_parameters`: Checks if a sound channel is active, then initializes a channel entry in a global table with a pointer to a sound sample, sets the sample's playback or loop parameters, and copies a sequence of volume or waveform-shaping bytes from a source table into the channel's state structure.
- `initialize_sound_channel`: The routine initializes a sound channel entry in a global table and sets up related per-channel state bytes in a global structure.
- `initialize_sound_driver_state_2`: Calculates an offset and retrieves a pointer to a structure, then clears certain fields and sets a pointer or value based on a conditional check of a global variable and a pointer in a structure.
- `initialize_sound_driver_state`: The routine writes constants and values read from an uninitialized pointer into fields of a structure pointed to by a0, clamping one field to a maximum, then returns the value originally passed on the stack.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 2 of 4).adf:jumptype

- `parse_string_and_manage_buffer`: Parses a string or command, checks for a space delimiter, and if a certain character range is met, calculates a buffer size and uses a loop to call a library function while incrementing a counter.
- `perform_dos_write_operation`: Calls a DOS library function using a pointer from a register, then sets up registers for a subsequent call to another routine and clears d0 before returning.
- `execute_write_operation`: Performs a write operation using a DOS library function via a sequence of six instructions.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 3 of 4).adf:jumptype

- `parse_string_and_manage_buffer_2`: Parses a string or command, checks for a space delimiter, and if certain character criteria are met, uses a DOS library function to check for a file or directory.
- `perform_dos_write_operation_2`: Sets up a DOS library write request with a destination pointer, a buffer size, and a fixed amount of data, then calls the library function and returns.
- `execute_write_operation_2`: Loads a function pointer from a table, clears a register, and calls the DOS write function using a value from a global pointer, then returns.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 4 of 4).adf:jumptype

- `parse_string_and_allocate_buffer`: Parses a string or command, checks for a space delimiter, and if a valid command or character is found, it calculates a buffer size, calls a routine to allocate memory, and then uses a loop to populate or clear a memory area while checking for a specific limit.
- `perform_dos_write_operation_3`: Sets up a DOS library write request using a source pointer, a buffer size, and a destination offset, then calls the DOS write function and returns.
- `execute_write_operation_3`: Clears a register and calls a DOS library function using a value from a global pointer and a value from a table, then returns.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 2 of 4).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 3 of 4).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### King's Quest IV - The Perils of Rosella (1990)(Sierra)(Disk 4 of 4).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_to_global_fields`: Writes a 16-bit constant 1 into two different offsets of a structure pointed to by a4, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
