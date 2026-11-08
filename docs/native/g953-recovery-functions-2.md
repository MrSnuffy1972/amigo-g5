# Game 953 (d-generation (1991)(mindscape)(disk 1 of 2)[cr flt]): code recovery summary

## Functions (part 2 of 2)

### D-Generation (1991)(Mindscape)(Disk 1 of 2)[cr FLT].adf:dgen
(continued)

- `validate_coordinate_proximity_2`: Compares the absolute difference between two byte values located at offsets 10 and 12 of two different structures against a threshold value provided in a register, returning 1 if both differences are within the threshold and 0 otherwise.
- `initialize_pointer_structure`: Stores a zero word at offset 46 of a structure and a 32-bit value at offset 48 of the same structure, calls another routine, then stores a zero byte at the location pointed to by the value at offset 48 before returning a 16-bit value from offset 46.
- `update_counter_and_flags_2`: Subtracts a word from a structure field and, if negative, performs arithmetic on a byte field, increments a byte field, and conditionally resets it if it reaches a specific value.
- `pack_sprite_attribute_bytes`: Constructs a byte by shifting and masking bits from a source structure at offsets 2 and 22, then stores this byte along with bytes from offsets 10 and 12 of the same structure into a destination buffer.
- `update_value_at_pointer_3`: Subtracts a word from the value pointed to by a0 from a value in a global structure; if the result is less than one, it clears the value at the pointer and returns one, otherwise it stores the result and returns zero.
- `split_word_into_bytes_3`: Calls another routine to obtain a 16-bit value, then stores the high byte into the memory location pointed to by a pointer in a0 and the low byte into the memory location pointed to by a pointer in a1.
- `jump_to_sprite_command_handler`: Performs an indirect jump to an address stored in a long word.
- `parse_data_stream_2`: Iterates through a data stream, interpreting bytes to call various routines and incrementing a pointer based on the command type.
- `jump_to_address_450`: Performs an indirect jump to the address stored at offset 450.
- `jump_to_offset`: Performs an indirect jump to the address stored at an offset of 14474 from the program counter.
- `dispatch_entity_state_logic_4`: Jumps to a routine located at a longword address stored in a pointer.
- `jump_to_indirect_address_6`: Performs an indirect jump to an address stored at a specific location in memory.
- `clear_structure_byte_conditionally`: Calls four different routines, then checks if a longword at a negative offset from a pointer in a4 is non-zero; if so, it clears a byte at offset 157 of the structure pointed to by another value at a negative offset from a4.
- `call_collision_check_2`: Loads two pointers from the stack into registers a0 and a1 and calls a routine to check entity collisions.
- `reset_flag_on_condition`: Checks a byte in a structure; if it is non-zero, it clears a byte at offset 27 of that structure, calls another routine, and if the result is zero, sets a byte at offset 156 and clears a byte at offset 157 of the structure.
- `set_flag_via_pointer`: The routine calculates an address using a global pointer and an offset, then stores the value one at that location.
- `adjust_offset_based_on_threshold`: Subtracts a word from a field in a structure and, if the result is negative, conditionally adds a value to that field and calls another routine.
- `update_entity_offset_loop`: Increments a short value at a specific offset in a structure by a constant amount, then loops through helper routines to check conditions and return a result.
- `verify_structure_fields`: Checks two bytes at specific offsets within a structure and returns zero if certain conditions are met, otherwise it returns.
- `validate_character_index`: Compares a byte from a structure against a calculated value, and if they match, calls two routines and potentially updates a byte in a global structure.
- `return_zero_and_restore_registers`: Clears the d0 register and restores d7 and a3 from the stack before returning.
- `dispatch_control_character`: Saves registers, moves a value from a0 to a3, and branches to different locations based on whether the byte in d0 is zero, four, eight, or nine.
- `copy_byte_and_branch`: Copies the address in register a3 to a0 and calls another routine, then branches to a nearby instruction.
- `call_pointer_and_branch`: Moves the value in a3 to a0, calls another routine, and then branches to another location.
- `transfer_pointer_and_branch`: Copies the value from a pointer in a3 to a0 and then branches to another routine.
- `set_return_value_one`: Copies the address from a pointer in a3 to a0, calls another routine, sets d0 to one, and branches to another location.
- `return_zero_4`: Clears d0 and restores registers d7 and a3 from the stack before returning.
- `copy_byte_and_fail`: Copies a byte from a source pointer into a global structure at a specific offset and returns a failure indicator.
- `initialize_object_fields_7`: Checks a byte in a global structure; if it is zero, it calls another routine, and if that returns non-zero, it calls a second routine and then sets several byte fields in a global structure using values from a pointer and a constant.
- `set_two_flags`: The routine sets two specific byte fields within a global structure to one.
- `call_stub_22`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `call_transform_routine`: Loads a pointer from a global structure and calls a routine at a specific offset.
- `dispatch_object_configuration`: Swaps the contents of two pointer registers and calls a routine to configure a target structure using a type byte from a source structure.
- `process_flag_byte_2`: Swaps two pointers in registers, extracts a byte from offset 19 of the second structure, shifts it left by 3, and then checks a byte at offset 159 of that structure to decide whether to set d1 to a constant.
- `process_flag_byte`: Ors a word from d1 into d0, then checks a byte at offset 156 of the structure in a3 to decide whether to clear d1 and branch.
- `pack_sprite_bytes_with_flag`: Combines bits from a source structure field, a register value, and a constant into a single byte, storing it at an offset in a destination buffer.
- `dispatch_object_config_4`: Swaps the contents of a0 and a1, then invokes a routine at a relative offset to configure properties in a target structure.
- `copy_byte_and_call_handler`: Copies a byte from a source structure offset to a destination buffer offset, then invokes another routine.
- `check_flags_and_call_handlers`: Checks two bytes at specific offsets in a structure and, if both are zero, calls four different routines before returning.
- `adjust_entity_position`: Subtracts a word from a value at offset 160 of a structure, and if the result is not positive, adds 3328 to that offset, clears a byte at offset 27, and calls two different routines.
- `set_flag_via_pointer_2`: Loads a pointer from a global variable, then sets a byte to one at offset 27 of the pointed-to structure before returning.
- `update_structure_state`: Checks several conditions in a structure pointed to by a4, and if met, calls two different routines and potentially updates several bytes at offsets 159, 163, and 8 of that structure.
- `check_flag_and_swap_pointers`: Checks a global flag byte; if clear, it copies structure fields to the stack, calls a method, and conditionally sets status bytes and updates pointers.
- `update_entity_state_8`: Checks several conditions involving global pointers and structure bytes, then calls routines to update various fields in a structure, including performing a division and setting multiple bytes.
- `clear_byte_if_condition`: Checks if a byte at offset 8 of a passed pointer exceeds a specific threshold; if so, it sets a byte at offset 155 of the same structure to zero.
- `update_position_and_check_flags`: Checks a byte in a global structure; if zero, it loops calling another routine until a non-zero status is returned, then updates position bounds and checks collision status.
- `execute_jump_table_entry`: Saves registers, extends a byte from d0 to a word, and uses d0 to index into a jump table to jump to another routine.
- `restore_registers_and_return_4`: Clears register d0 and restores several registers from the stack before returning.
- `check_room_input_condition`: Checks a character value from a global structure against a threshold and conditionally processes room input using a specific byte sequence.
- `call_stub_23`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `double_dispatch_sprite_transform_2`: Calls a routine that checks a validity flag in a structure and updates global position and dimension variables twice in succession.
- `execute_two_handlers`: Calls two different routines sequentially and then returns.
- `subtract_and_clamp_short`: Subtracts a global short value from a short located at an offset from a global base pointer, clamps the result to zero if negative, and returns the final value.
- `check_clear_overlap_flag`: Checks a byte in a global structure; if non-zero, it calls a routine to check rectangle overlap and clears the byte if the overlap check returns zero.
- `process_register_and_branch`: Saves d7 and a3 to the stack, moves d0 to d7, moves a0 to a3, extends d0 to a word, and branches based on whether d0 is zero or six.
- `branch_to_handler`: Copies the value of register a3 into a0, calls a routine, and then performs a relative branch.
- `transfer_pointer_and_branch_3`: Copies a longword from a pointer in a3 to a0, calls another routine, and then branches to a location slightly before the original call site.
- `restore_registers_and_return_5`: Clears register d0 and restores d7, a3, and four additional longwords from the stack before returning.
- `process_security_and_state`: Calls several routines to validate state, search a buffer for a byte, and update global structures based on the results.
- `write_word_to_a0`: Writes the word decimal 2560 to the address stored in a3 and branches to another routine.
- `handle_security_door`: Calls a routine, then if the result is non-zero and a byte at offset -10108 of a4 is not decimal 133, it calls a routine with decimal 22 in d0 and another routine with decimal 4 in d0 and decimal 3 in d1.
- `initialize_security_message`: Increments a value, pushes it onto the stack, and calls two routines to initialize a string buffer and update a structure.
- `check_state_and_invoke`: Calls a routine to validate state, and if the result is non-zero, sets a byte at offset 156 of a structure pointed to by a pointer in a4.
- `clear_and_invoke_state_check`: Clears a specific word-sized field in a structure pointed to by a4, calls a routine to validate state and invoke methods, and returns the value of that cleared field.
- `compare_and_invoke_method`: Saves registers to the stack, compares a byte from a structure to two and ten, and if it matches neither, it sets d0 to zero and branches to another routine.
- `compare_byte_with_seven`: Extends a byte from a structure at offset 52 of a3 to a word, compares it to seven, and branches accordingly.
- `compare_structure_byte`: Loads a pointer from a global variable, checks a byte at offset 166 of that structure, and if it is not zero, compares a byte at offset 154 of that structure with a value in d6.
- `compare_structure_bytes`: Compares two bytes at offsets 154 and 158 of the structure in a0 against the value in d6, and if neither matches, clears d0 and branches.
- `invoke_method_and_set_flag`: Stores a register value on the stack, copies a byte from a structure, invokes a method, and conditionally sets a word in a global structure to one.
- `compare_and_mask_bytes`: Subtracts a byte from another byte loaded from two different global structures, masks the result with seven, and returns one if the result is not less than five, otherwise it returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_stub_24`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `check_flag_and_dispatch_transform`: Checks a byte in a structure against a specific constant value.
- `setup_collision_buffer`: Copies a byte to a global variable, clears an 11-byte buffer, sets specific bytes in a structure, and computes collision offsets.
- `pack_sprite_bytes_and_call_2`: Swaps two pointers in registers, extracts a byte from offset 157 of the second structure, and performs conditional branches based on its value and the value of a byte at offset 166 of that structure.
- `set_d7_and_branch_2`: Sets d7 to 64 and branches to another location.
- `set_d7_and_branch_alt`: Sets d7 to 128 and branches to another location.
- `pack_sprite_bytes_and_call`: Combines bits from two structure fields into a single byte, stores it in a destination buffer, and calls another routine.
- `jump_to_logic`: Performs an indirect jump to the address stored at a long constant.
- `update_sprite_attributes_3`: Checks a byte at a specific offset in a structure; if non-zero, it calculates a new position by adding a value from a table to a global variable, copies two 16-bit values from the structure to global locations, updates two 32-bit values in the structure, and stores a byte into a global variable before calling update_sprite_offsets_flags twice.
- `jump_to_character_index_bounds_routine`: Performs a long jump to the compute_character_index_bounds routine.
- `search_byte_in_buffer_5`: Iterates through a buffer pointed to by a3, comparing each byte to d7 until a match is found or a negative byte is encountered.
- `jump_to_signed_difference_calculator`: Performs an indirect jump to a routine that calculates a signed difference.
- `jump_to_address_12`: Performs an indirect jump to an address stored in a register.
- `dispatch_indirect_jump`: Performs an indirect jump to a location stored in a longword at a specific address.
- `increment_and_pack_data_2`: Increments two word-sized values in a structure, then constructs and updates bytes in a destination buffer using bitwise operations on source data.
- `initialize_object_fields_5`: Clears a byte at offset 8 of a structure in a2, extracts bits from a byte in a3 to set a byte at offset 25 of the structure, and if certain conditions are met, sets various bytes at offsets 9c, 9e, 9f, a0, 100, and 1b of the structure in a1.
- `increment_timer_conditionally_2`: Checks two bytes in a structure and, if they match specific values, subtracts a word from a field and adds a constant to that field if the result is negative.
- `update_structure_state_flag`: Checks a byte at offset 156 of a structure pointed to by an offset of -11240 from a4; if non-zero, it sets a byte at offset 157 of that structure to 1, otherwise it clears the byte at offset 156 and sets a byte at offset 27 to 1.
- `initialize_string_buffer`: If a pointer in a0 is non-null, it calls another routine, clears a byte at offset 2b14 of a global structure, copies a byte from a table to offset 2b21, copies a byte from another table to offset 2b20, sets a byte at offset 7d66 to 1, and stores a 16-bit value calculated from d0 multiplied by 55 at offset 2b16 of a global structure.
- `jump_to_collision_resolver_2`: Redirects execution to the routine that resolves collision status for specific offsets and updates vertex coordinates in a global structure.
- `setup_object_properties_2`: Copies a byte from a source structure to a global variable, extracts bit fields to set properties at offsets 22 and 25 of a target structure, and calls another routine.
- `clear_eleven_bytes_via_pointer`: Reads a pointer from an offset of 78 in a structure pointed to by a register, then loops to zero out eleven consecutive bytes starting at that address.
- `check_list_for_active_item_3`: Iterates through an array of pointers up to a fixed limit and calls a routine for each entry.
- `dispatch_method_via_table_6`: Checks a global flag; if clear, it swaps pointers in a structure, calls a function pointer from a table indexed by a byte in that structure, and restores the original pointers.
- `jump_to_address_13`: Performs an indirect jump to an address stored in a register and returns 0.
- `backup_and_test_vertex_coordinates`: Saves two 16-bit values from a structure pointed to by a global variable into local storage, calls another routine to perform a calculation, and then restores the original values if the calculation result is non-zero or modifies a byte at offset 68 of the structure if the result is zero.
- `dispatch_collision_check`: Performs a long jump to another routine.
- `copy_target_coordinate_words_2`: Copies two word-sized values from a source structure pointed to by a register into a target structure located via a global variable and a fixed offset.
- `validate_coordinate_proximity_3`: Compares the absolute difference between two byte values located at offsets 10 and 12 of two separate structures against a threshold value provided in a register, returning 0 if either difference exceeds the threshold and 1 otherwise.
- `update_structure_and_call_subroutine`: Stores a zero word at an offset of 46 of a structure and a 32-bit parameter at an offset of 48 of the same structure, calls another routine, then stores a zero byte at the location pointed to by the value at offset 48 and returns a 16-bit value from offset 46.
- `pack_sprite_attribute_bytes_2`: Constructs a byte by shifting and masking bits from a source structure at offset 2 and offset 22, then stores it along with bytes from offsets 10 and 12 into a destination buffer.
- `update_entity_status_bits`: Extracts specific bit fields from a byte at a negative offset from a4 and writes them to a structure in a3.
- `jump_to_other_routine`: Performs a long jump to an address stored in a pointer.
- `dispatch_object_allocation_2`: Performs a long jump to a function located at a pointer stored in a specific memory address.
- `jump_to_indirect_address_7`: Performs an indirect jump to an address stored at a location specified by a long constant.
- `process_entity_buffer_flags`: Saves d7 and a2 through a3 to the stack, clears a byte at offset 35 of the structure in a3, and checks several byte values in the structure; if conditions are met, it updates global pointers and calls routines to update buffer flags and process flagged entities.
- `jump_to_indirect_routine_2`: Performs an unconditional jump to a location stored in a memory address.
- `load_word_from_table`: Loads a 16-bit value from a global table indexed by a register and returns it combined with a shifted version of the index.
- `load_word_from_alt_table`: Loads a 16-bit value from a different global table indexed by a register and returns it combined with a shifted version of the index.
- `trigger_trap_zero`: Executes a trap instruction with vector zero.

### D-Generation (1991)(Mindscape)(Disk 1 of 2)[cr FLT].adf:FairLight

- `process_amiga_hunk_relocations`: Iterates through a list of data structures, identifying specific hunk type codes and performing memory allocation and relocation operations.
- `process_linked_list_data`: Walks a linked list to find a maximum value, calculates a difference between a register and a pointer, calls a routine, and then calls another routine if a comparison with a constant succeeds.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Uses a loop to process data from a pointer in a3, performing bitwise shifts and calls to other routines to write bytes to a destination at a3 based on values derived from a0.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Iterates through a loop, performing bitwise shifts and rotations on data from a pointer and a long value, while conditionally calling other routines.
- `execute_trap_sequence`: Moves a word from an offset of a register into a register, performs a bitwise OR on a register with a constant, calls a trap, and performs a bitwise OR on a register with the result of the trap.

### D-Generation (1991)(Mindscape)(Disk 1 of 2)[cr FLT].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap`: Sets a global variable to a specific value and executes a trap instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_and_trap_2`: Stores the address of this routine into a global variable and then executes trap zero.
- `process_a5_state_loops`: Sets a byte and a word at fixed offsets in a structure addressed by a5 and runs nested polling loops that wait for those bytes to change, calling other routines during each wait.
- `toggle_flag_and_update_table`: Toggles a flag word in the structure addressed by a5, and when the flag becomes zero it advances a counter by two and writes one value into four table entries spaced sixteen bytes apart at a fixed offset of 168 bytes.
- `toggle_flag_and_fill_buffer`: Toggles a flag word at a fixed offset in the a5 structure, and when it becomes zero and a counter stays non-negative after subtracting two, it writes one value into four entries of the table at offset 168 with a stride of sixteen bytes.
- `process_nibble_table`: Calls another routine 32 times in a loop, then iterates over a table of 32 words to split each word into four-bit nibbles and store them at offsets 2, 6, and 10 of a destination structure, while also storing the negated value of each nibble at the preceding offset in that structure.
- `find_max_nibble`: Scans the four nibbles of a 16-bit word and keeps the largest one in a register.
- `update_timer_table_entries`: Decrements a countdown at a fixed offset in the a5 structure and, when it reaches zero, resets it to four and walks 32 timer records, decrementing each active count and otherwise comparing a masked value against a stored word and updating that word.
- `decrement_and_reset_table`: Decrements a countdown at a fixed offset in the a5 structure and, when it reaches zero, resets it to four and walks 32 words, subtracting one from each nonzero 4-bit field in the high, middle and low positions.
- `initialize_dma_and_audio_channels`: It writes the DMA and interrupt control registers, the CIA B port registers and the audio-channel registers, then waits for the vertical beam position counter to reach a threshold.
- `build_register_value_table`: Fills a table with four groups of paired words holding incrementing register-style values and the high and low halves of a fixed address, then writes 80 words of incrementing values interleaved with zeros.
- `restore_dma_and_interrupt_state`: Disables DMA and interrupt enables through the custom registers, restores saved values from globals, sets two global flags, then re-enable DMA by writing a fixed value.
- `copy_structure_fields`: Copies a 32-bit word from offset 38 of a structure in a5 to offset 128 of a structure in a6, then stores a 16-bit word from d0 into offset 136 of the structure in a6 and returns.
- `write_interrupt_request`: Stores the constant 255 into a global variable, stores the constant 32 into the interrupt request register, and returns from an interrupt.
- `configure_cia_interrupt_control`: Stores the constant 255 into a global variable, stores the constant 8192 into the interrupt request register, and returns from an interrupt.
- `modify_memory_and_test_bit`: Increments a 32-bit value in a register, performs a bitwise OR on a memory location pointed to by a register, and tests a bit in a value loaded from a memory offset.
