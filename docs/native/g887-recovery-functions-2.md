# Game 887 (international soccer challenge (1989)(micro style)[cr pdx][h spaikers][t +1 pdx]): code recovery summary

## Functions (part 2 of 2)

### mathieeedoubtrans.library
(continued)

- `load_library_function_pointer`: Loads a constant address into a0 that points to a location relative to the library base, then branches to a routine located at a fixed offset from that address.
- `call_mathieee_basic_slot_a`: Loads the library base pointer from a global variable, copies the values in d0 and d1 from the stack, and jumps to the function at a fixed offset of minus 54 in the library table, then restores the caller's registers and returns.
- `call_mathieee_basic_slot_b`: Loads the library base pointer from a global variable, copies the values in d0 and d1 from the stack, and jumps to the function at a fixed offset of minus 90 in the library table, then restores the caller's registers and returns.

### International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf:border

- `jump_to_next_instruction`: Performs a jump to an instruction located at a fixed offset relative to the program counter.
- `initialise_border_state`: Clears eight longwords of state and stores a value returned by a call, then conditionally executes library functions or other routines.
- `execute_library_hooks`: Checks a global pointer for a non-zero value, checks a byte at an offset for a non-zero value, and if valid, performs a sequence of routine calls using arguments or pointers retrieved from a structure or a global pointer.
- `call_present_resource_routines`: Calls routines only for resources that are present, checking a stored pointer and a status byte first.
- `setup_console_device_request`: Calls library vector functions and routines in sequence, referencing the console device text.
- `configure_border_structure_fields`: Initializes a structure by setting pointers from a table, clearing specific memory locations, and updating flag bits.
- `move_data_to_pointer`: Moves a 32-bit value from a pointer-indexed location to a location pointed to by a4, then moves a 32-bit value from a pointer-indexed location to a location pointed to by a2.
- `call_border_routines_with_coordinates`: Computes four offsets from stored structure fields and calls routines with those coordinates.
- `store_pointer_and_call_format`: Stores a pointer from a structure field into a temporary location, then calls a routine that parses a format string and returns.
- `initialize_border_state_and_call`: Sets a4 to a constant, then calls a routine to initialize a state structure, and if certain conditions are met, calls another routine or sets a value in a structure and calls a third routine.
- `set_a4_to_constant`: Loads the constant value 7ffe into a4 and returns.
- `initialize_border_state`: Initializes a state structure by calling a routine to retrieve a pointer, setting specific flag bits, and populating the structure with calculated values and a constant string.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_to_buffer`: Parses a source string by skipping whitespace and handling quoted or unquoted text, copying the resulting tokens into a destination buffer, and then populates a table with pointers to these tokens.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `set_initial_limit`: Sets a 16-bit register to a constant value and branches to another address.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: Compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `update_window_structure`: It calls several library functions to check or modify a structure, and if certain conditions are met, it updates a pointer in that structure and clears a global variable.
- `copy_register_to_register`: Copies the contents of one 32-bit register to another.
- `call_and_terminate_buffer`: Stores a pointer, calls a routine that parses a format string, then clears a byte at the stored pointer address and returns.
- `increment_pointer_and_store_byte`: Increments a global pointer and stores a byte from the first parameter at the original address.
- `call_parse_format_string`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, restores the frame and returns.
- `update_border_buffer`: It checks a value at an offset in a structure, and if certain conditions are met, it calls a routine with a stack argument, then writes a byte from a calculated address to a buffer and calls another routine.
- `parse_format_string`: Parses a string for format specifiers, updating a local buffer with or calculating or retrieving values from a provided pointer, and calls another routine to handle certain string-based or value-based formatting or length-related logic.
- `perform_signed_division`: It checks the signs of d0 and d1, updates a flag in d4, calls another routine, and if the flag is set, negates the result in d0 before returning a 64-reconstructed value.
- `prepare_signed_division_operands`: Saves d1 and d4, clears d4, and if d0 or d1 are negative, negates them and increments d4 accordingly before calling another routine and branching.
- `call_and_return_concatenated`: It pushes d1 to the stack, calls another routine, restores d1, and returns a 64-bit value formed by concatenating the original d1 contents with the value left in d1 by the call.
- `recursive_call`: It pushes d1 to the stack, calls itself, and then restores d1 before returning.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns its length, excluding the terminator.
- `check_structure_bit_and_call`: Checks if a value from a structure offset is equal to ten and tests bit seven of a byte in a secondary structure, then calls another routine if these conditions are met; otherwise it returns or calls a routine if the first condition is met but the bit test fails.
- `branch_to_target`: Performs a short branch to a target address.
- `process_pointer_and_call`: Sets up a stack frame, pushes a2 to the stack, loads a pointer from an offset of a structure in a5 into a2, loads a pointer from that pointer into a0, compares a pointer at offset 4 of that pointer with a0, and if not lower, masks a word from a structure in a5, pushes it to the stack, pushes a2 to the stack, calls another routine, adjusts the stack, restores a2, and returns.
- `update_pointer_and_copy_byte`: Copies a byte from a source offset of a structure in a5 to a destination pointer in a0, increments a pointer in a2, and branches to another location.
- `apply_routine_to_record_table`: Steps through a fixed table of 22-byte records from its first entry up to its end, calling another routine once for each record's address.
- `process_structure_and_clear_fields_2`: Loads a pointer from a structure, and if it is not null, sets d0 to zero before branching to another routine; otherwise, it sets d0 to negative one and returns.
- `process_structure_and_clear_fields`: Checks a status byte at offset 3 of a structure, then conditionally calls other routines using pointers or values found at offsets 2, 8, and 18, and finally clears the first 4 bytes and the status byte of that structure.
- `test_bit_and_branch`: Sets up a stack frame, loads a pointer from a structure, and tests a bit in that structure, otherwise it returns immediately.
- `update_structure_and_clear`: Checks a bit in a structure, and if set, calculates a size, calls another routine, and if the return value matches the size, sets a bit in the structure, clears two longwords in the structure, and returns a constant value, otherwise it branches to a location that clears a bit, clears two longwords, and returns a constant.
- `reset_structure_on_null_pointer`: Checks if a 16-bit value at an offset in a structure pointed to by a5 is not equal to all ones; if so, it clears bit 2 of a byte at an offset in a2, clears a 32-bit longword at that same address, and clears a 32-bit longword at a 4-byte offset from that same address, otherwise it returns.
- `update_structure_and_copy_byte_2`: Checks a pointer in a structure, and if it is not null, calls another routine; otherwise, it updates a pointer, sets a bit in a byte, and copies a byte from a structure to a new location.
- `update_structure_and_copy_byte`: Updates a pointer in a structure, adds a short value from a structure offset to a pointer, sets bit 2 of a byte in a structure, increments a pointer, and copies a byte from a5 to a destination pointer, then branches to another routine.
- `traverse_structure_list`: Iterates through a list of structures, checking if a byte at offset 12 is zero; if so, it clears three longwords at the start of the current structure and returns, otherwise it continues to the next structure until a certain limit is reached, otherwise it returns.
- `branch_to_target_offset`: Performs a short branch to a location within another routine.
- `clear_structure_members`: Clears a longword at an offset of a structure in a2, clears a longword at another offset, clears a longword at a third offset, and branches to another location.
- `update_border_pointer_and_flag`: Calls another routine and, if the return value is zero, sets a 16-bit value to 1024 at offset 16 of a structure and updates a pointer at offset 8 to a new address, otherwise it sets a 16-bit value to 1024 at offset 16, updates a pointer at offset 8 with the return value, and otherwise returns.
- `update_border_and_loop`: Writes a value to a pointer, sets a bit in a structure, loads a value from a pointer, calls another routine, and loops if the loaded value is not zero.
- `setup_list_traversal`: Sets a link, saves registers a2 and a3 to the stack, and loads a pointer from a global variable into a2 before branching to another location.
- `free_list_nodes_from_pointer`: Releases each node of a singly linked list starting at the node in the address register and stopping after the node whose link is empty, using each node's stored size plus eight, then clears the global head pointer.
- `setup_list_node`: It stores a pointer to a routine in a memory location, clears a stack value, and calls a routine with a pointer to a new stack-allocated argument before checking the return value.
- `copy_and_increment_pointer`: Copies a longword from a global variable to a structure, copies a longword from a structure to another structure, updates a global pointer, and branches to another location.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `set_pointer_and_branch`: Saves registers to the stack, calculates a pointer from a structure field, and branches to another instruction.
- `check_pointer_and_copy_long`: Compares a pointer in a5 with a2, and if they are equal, copies a longword from the address pointed to by a2 into the address in d0, otherwise it returns.
- `conditional_move_and_branch`: It moves a value from a register to another register and, if the value is not zero, copies a longword from a pointer in a2 to the address in a3 before branching.
- `call_function_with_arguments`: Loads a pointer from a global variable, calculates an offset, pushes a pointer and a value onto the stack, calls a routine, cleans up the stack, and branches to another location.
- `calculate_index_and_call_routine`: Multiplies a value in d0 by 6, adds it to a pointer from a4, checks if a certain value is zero, and if not, calls another routine using a pointer loaded from the new address.
- `calculate_and_call_routine`: It multiplies a value in d0, loads a pointer from a global address, pushes a value from that pointer onto the stack, calls a routine, and then conditionally sets d0 and branches.
- `clear_register_and_branch`: Clears register d0 and branches to another location.
- `execute_io_operations`: It pushes a value onto the stack, calls a routine, and if the return value is not zero, calls another routine, stores a value in a global variable, and returns.
- `clears_d0_and_branches`: Clears register d0 and branches to another routine.
- `update_status_and_branch_2`: Calls a routine, multiplies the return value by six, and checks if the result or a global variable matches a certain range or value before potentially writing a constant to a global variable and returning.
- `update_status_and_branch`: Checks if certain bits are set in a word from a2, and if not, writes a constant to a global variable and sets d0 to a constant before branching back to the start of the routine.
- `execute_write_with_error_check`: It pushes several values onto the stack, calls a routine, and if the return value is not certain, calls another routine, stores a value in a global variable, and branches.
- `move_and_branch`: Moves a longword from d5 to d0 and then branches to a relative address.
- `check_library_result_and_branch`: Calls a routine with a fixed argument, checks if bit 12 of the return value is set, and if so, checks a certain word in a global variable before potentially calling another routine and returning the result or zero.
- `execute_callback_and_branch`: Calls a routine and then branches to another instruction.
- `write_fixed_text_then_callback`: Obtains an output handle, writes a four-byte text fragment from a fixed address via DOS calls, and then executes a conditional function.
- `run_optional_hook_then_finish`: It checks if a global hook pointer is non-zero, calls that hook if it exists, then calls another routine and returns.
- `release_all_resource_tables`: Releases the program's resource tables by iterating through a table to free entries, running a global hook, and freeing various buffers.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_close_handle`: Checks if a calculated index or handle is within a valid range and if a pointer is non-null, then calls a library function to close a resource and clears a pointer if successful.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `jump_to_library_function`: Loads a function address from a global pointer plus an offset and jumps to it.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `jump_to_next_instruction_2`: Performs an indirect jump to an address calculated by adding a negative offset to a value loaded from a global variable.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Loads a function pointer from a memory location calculated by adding a negative offset to a base address stored in a global variable, then jumps to that address.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_offset_210`: Loads a library base from a global variable and calls a library function at an offset of minus 210 from that base.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_offset_552`: Loads a library base from a global variable and calls a library function at an offset of minus 552 from that base.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_offset_384`: Loads a library base from a global variable and calls a library function at an offset of minus 384 from that base.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.

### International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf:return

- `Start`: Opens dos.library, performs a loop involving standard output, and writes a sequence of bytes to a memory buffer, then closes the library.

### International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_memory_blocks`: Initializes four distinct blocks of memory by filling them with word-sized values of all ones using a series of loops.
- `copy_nested_word_blocks`: Copies 12-word blocks from two source tables to two destination tables using nested loops; the inner loop copies 12 words, the middle loop repeats this 143 times with an 8-word offset per block, and the outer loop repeats the process 3 times with an 8000-word offset per iteration.
- `update_bitmask_with_shifted_values`: Calculates a bitmask by or-ing several registers and inverting the result, then applies this mask to clear and set specific bits in a 16-bit word at the pointer in a2, specifically targeting the first word, the word at offset 4000, and the word at offset 8000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_menu_bitmask_attributes`: Calculates a bitmask using logical operations on registers d1, d2, d3, and d4, then applies this mask to update specific bit-fields or flags within a 16-bit array or structure pointed to by a2, specifically targeting or modifying bits at offsets 0, 4000, and 8000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_and_swap_registers`: Performs complex bitwise logic on several registers to create a mask, then applies this mask to update specific 16-bit words in a memory buffer via a2, and finally swaps the contents of all even-numbered or odd-numbered registers before applying further bitwise updates to a memory buffer via a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_shifted_values_2`: Shifts a value in d2 and d3 by the amount in d0, then uses a bitwise mask to clear certain bits and set others in a 16-bit word at the address held in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_menu_attribute_masks`: Calculates a bitmask using bitwise NOT, AND, and OR operations on registers, then applies this mask to update specific bit-fields or attribute flags at certain offsets within a memory structure pointed to by a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_shifted_values_3`: Calculates a bitmask by ORing several registers and inverting the result, then applies this mask to clear specific bits in a memory location pointed to by a1, before setting certain bits using a value from d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_menu_bitmask_attributes_2`: Calculates a bitmask using bitwise NOT, AND, and OR operations on registers and applies this mask to update specific bit-fields or flags within a memory structure pointed to by a1, specifically targeting offsets 0, 4000, and 8000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_bit_shifts`: Calculates bit-shifted or bit-rotated bitmasks using a bit-count or bit-index and applies these masks to a bitset or bitmask stored in a memory location, updating bits at certain offsets or indices.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_bit_shifts_2`: Calculates a bitmask using bitwise shifts and logical operations, then applies this mask to update specific bits in several memory locations or arrays via pointers in a1 and a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_shifted_values_4`: Calculates a bitmask using bitwise shifts and logical operations on values loaded from a routine, then applies this mask to update specific bits in a memory buffer or array via a pointer, using bitwise AND and OR operations to modify or preserve certain bits at fixed offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_attributes`: Calculates a bitmask by masking and ORing several registers and applies this mask to clear and then set specific bits in a memory buffer pointed to by a1, specifically targeting the first element, the element at offset 8000, and the element at offset 16000 (4000 or 8000 words away).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_menu_attribute_bits`: Calculates a bitmask from three input words and applies it to clear or set specific bits in a memory-mapped or global attribute table at certain offsets, specifically targeting bit-level modifications at the start of the buffer and at offsets 8000 and 16000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_with_shifts`: Shifts d2 and d3 left by the value in d0, then updates a word in a table pointed to by a2 by clearing bits present in a combination of d1, d2, d3, and d4, and setting bits from d1 and d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_table_bits`: Clears d4, then performs a branch to a later instruction, and subsequently updates a word in a table pointed to by a2 by clearing bits present in a combination of d1, d2, d3, and d4, and setting bits from d1 and d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_and_update_table`: Performs a bitwise AND on d4 with d2, then performs a branch to a later instruction, and subsequently updates a word in a table pointed to by a2 by clearing bits present in a combination of d5, d1, d2, d3, and d4, and setting bits from d1 and d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_flags`: Calculates a bitmask by ORing several registers and inverting the result, then applies this mask to clear and subsequently set specific bits in a memory location pointed to by a1 and a remote memory location at offset 4000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_menu_attribute_mask`: Calculates a bitmask using bitwise logic on registers and applies it to a table or structure pointed to by a1, then enters a loop that updates further offsets in the table using a calculated bitmask and a value derived from a pointer-based offset or otherwise returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_flags_2`: Calculates a bitmask by or-ing several registers and inverting the result, then applies this mask to clear and set specific bits in a 16-bit word at the address held in a2, including updates to a word at offset 4000 and a word at offset 8000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmask_and_loop_buffer`: Calculates a bitmask using bitwise logic on registers and applies it to a buffer at a pointer in a2, then enters a loop that updates further memory offsets or otherwise returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_init_chain_structures`: Saves all registers, calls a routine that initializes a chain of structures, and restores all registers before returning.
- `call_update_display_parameters`: Saves all registers, calls a routine that updates display parameters, and restores all registers before returning.
- `call_clear_and_set_globals`: Saves a3 to the stack, calls a routine that clears a run of byte locations and writes certain values to a global variable, restores a3, and returns.
- `call_enable_audio_channels`: Saves d1, d7, and a3 to the stack, calls a routine that checks a global flag and updates the audio control register, restores d1, d7, and a3, and returns.
- `call_initialize_menu_attribute_table`: Saves multiple registers, calls initialize_menu_attribute_table, restores the registers, and returns.
- `call_set_four_global_flags`: Saves register a3, calls set_four_global_flags, restores a3, and returns.
- `call_subroutine_and_restore`: Saves a3 to the stack, calls another routine, and restores a3 before returning.
- `init_chain_structures`: Sets a global flag, reads a byte from a table indexed by a computed offset, then fills four consecutive structures in a chain with pointers and constants before clearing globals and returning.
- `clear_and_set_globals`: Clears a run of byte locations in memory, then writes the decimal values 15, 255 and three zeros into a global variable, and returns.
- `set_four_global_flags`: Stores the value one into four bytes of a global structure at offsets 2070, 2030, 2054 and 2078, then returns.
- `enable_audio_channels`: Checks a global flag and, when it is set, builds a channel-enable value by setting four bits and stores it in the audio control register, then sets a follow-up flag and returns.
- `write_byte_to_memory`: Loads an address into a3, writes the value 3 to a memory location relative to a3, and then returns or branches to another routine.
- `update_display_parameters_and_dma`: Decrements a counter in a structure and, if not zero, calculates new display or DMA-related parameters, such as a new DMA-enabled address or a bitmask, and updates a structure or a global table with these new values, potentially updating certain hardware registers or bit-fields; otherwise, it resets certain global or structure-based flags and returns.
- `update_display_parameter_offset`: Reads a word from a structure, calculates a new pointer or offset, and checks a value at that calculated address; if the value is non-zero, it branches away, otherwise it updates a value in the structure and jumps to another routine.
- `initialize_dma_and_setup_structure`: Clears a 16-bit value at offset 32 of a structure in a0, copies two bytes from a buffer in a1 to offsets 20 and 21 of that structure, and enables a DMA channel by setting bit 1 of a control register.
- `update_display_parameters_and_dma_2`: Moves a word from offset 28 to offset 30 of a structure in a0, stores a3 at offset 4 of that structure, loads a long value from a global variable into a4, writes 32 to offset 4 of that global variable, and branches based on a byte test at offset 11 of the global variable.
- `reset_dma_and_audio_registers`: Clears several word-sized values at offsets within a structure pointed to by a3, enables DMA via the DMA control register, and sets the audio hardware registers to specific values before returning.
- `move_byte_and_branch`: Moves a byte from a pointer to a location relative to a pointer and branches to another instruction.
- `copy_bytes_to_structure`: Stores a byte at offset 1 of a structure in a0, then copies two bytes from a1 to offsets 44 and 46 of the structure in a0, clears offset 45, and branches to another routine.
- `set_status_and_branch`: Sets the negative flag based on a byte in a0 and branches to another routine.
- `updates_structure_byte_and_jumps`: Loads a byte from a pointer in a1 and stores it into a structure at offset 3 of a0, then jumps to a different part of the program.
- `write_bytes_to_structure`: Copies two bytes from a1 to offsets 8 and 9 of a structure in a0, sets a 16-bit value at offset 10 of the structure in a0 to zero, and branches to another routine.
- `extend_and_store_byte`: Copies a byte from a1 to d0, extends it to a word, stores the word at offset 1512 of a structure in a3, and branches to another routine.
- `store_bytes_to_structure`: Copies a byte from a1 to offset 1519 of a structure in a3, then copies a byte from a PC-relative address to offset 1520 of the structure in a3, and branches to another routine.
- `process_chain_entries`: Walks a chain of structures stored in a table, copying each entry's fields into a destination structure four times, then stores the final chain pointer and increments a counter, but only when the flag at offset 95 of the structure is zero.
- `initialize_menu_attribute_table`: Sets a bit in a custom register and clears a certain amount of memory or flags based on a value in a register, then updates a table of attribute bitmasks.
- `update_dma_and_structure_pointers`: Writes the values 1 and 124 to the custom DMA and interrupt control registers, calls a routine, updates a global variable with a value from d2, calls another routine, copies 11 words from one buffer to another using a loop, updates certain fields in a structure, calculates a new address and stores it in a structure, calls a routine, and writes a byte to a structure offset.
- `configure_dma_channels`: Performs a short busy-wait delay, then checks a table of flags to conditionally enable or disable DMA channels by calling another routine with a specific bitmask and a pointer to a hardware register (such as a custom chip register or a DMA control register).
- `enable_dma_channels`: Sets bit 15 of the DMACON register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_11`: Returns the constant -1 in d0 to the caller without any other effect.
- `parse_string_and_set_hardware`: Parses a string from a library-provided buffer to extract numeric values, storing them in global variables, then iterates through a list of pointers to update a table of hardware register values, specifically clearing certain DMA and interrupt registers and setting sprite or object coordinate tables.
- `update_sprite_hardware_sync`: Calculates a new horizontal offset, checks for certain button presses and a specific hardware status bit, then updates a table of sprite or object coordinates and writes new values to the hardware registers, including a loop that populates a table with or certain coordinate or pointer values.
- `update_sprite_position_and_sync_hardware`: Iterates through a list of objects to update their horizontal and vertical sprite or object coordinates, then checks a status bit and configures certain hardware registers, including disabling interrupts and DMA, before calling a library function and returning.
- `check_vblank_and_set_register`: Loops until bit 0 of a certain register is set, then writes a value from a table to a register and returns, or otherwise returns immediately with a different value.
- `call_library_vector_48_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_blitter`: Clears a large block of RAM, initializes several lookup tables, and sets the bitplane pointer registers, then waits for the blitter to finish or otherwise returns.
- `configure_audio_channels`: Checks a global variable and, when it is nonzero, writes the value 24596 to three audio channel registers derived from the pointer in a0, then restores the stack and returns.
- `set_relocation_value`: Loads a word from a table, then if the word is certain known values, writes a new word to a global variable, otherwise it loops to copy bytes from a source pointer to a destination pointer.
- `update_string_position`: Updates a global word with a value from d0 and d1, then loops to copy bytes from a source pointer to a destination pointer, advancing the destination pointer and updating a global word, otherwise it returns.
- `draw_text_glyphs_from_position`: Draws a null-terminated string of characters using eight-row font glyphs at the current screen position, advancing one column per character.
- `step_counter_toward_joystick_value`: Reads a joystick direction value from the CIAA_PRA register and increments or decrements a global counter toward that value, while updating a state byte and potentially filling a table with a repeating pattern and a running counter.
- `read_keyboard_status`: Reads the keyboard serial register after disabling interrupts, inverts and rotates the value, and stores it into a global variable only when bit 7 of the result is clear, otherwise it returns.
- `fill_gradient_table_rows`: Fills a table in nested loops, writing a stepped byte sequence across each row and advancing the increment as it goes, then null-terminating the rows.
- `increment_table_by_flag`: Selects a byte based on a global flag, then loops over a table adding that byte to values at spaced offsets and toggling the flag each iteration before returning.
- `wait_and_configure_port`: Waits in a loop until bit 14 of the byte at offset 2 of the structure in a6 becomes clear, then stores the values an address, 0, an address and 9876 at offsets 84, 100, 64 and 88 of that structure, and finishes by decrementing register d0 from 256 until it underflows to -1.
- `set_bitplane_bits_from_lookup`: Walks a nine-by-thirty-two grid of source bytes and uses two lookup tables to find the byte and bit to set in each of two bitplane-style buffers for every cell.
- `initialise_lookup_tables`: Subtracts 4112 from each of seven consecutive words in a global table while calling a routine for each, then fills 256 entries with multiples of 40 and 320 word pairs holding a right-shifted index and its inverted low three bits.
- `process_wait_structure`: Decrements a counter that resets to 8 when it reaches 0, skips past empty slots in a table, then loops eight times copying a value and adding it to a field while calling a wait routine before returning.
- `store_structure_fields_wait`: Reads a value from a structure field and computes an address from another field, then waits until a status bit clears before storing several words and longwords into fixed offsets of a structure.
- `wait_and_configure_port_2`: Waits in a loop until bit 7 of the byte at offset 2 of the structure in a6 becomes clear, then stores the values an address, 0, an address and 4052 at offsets 84, 100, 64 and 88 of that structure, then returns.
- `populate_relocation_table`: Fills a table with a sequence of longwords that increment or decrement a value, then writes several fixed longwords to a table, and finally clears a range of memory starting at a certain address.
- `clear_sample_ram`: Points a pointer at address an address, then writes zeros to every 40th pair of consecutive bytes starting there, repeating the store 37 times, and returns.
- `advance_pointer_list_flag`: Toggles a flag bit and, when the previous value was set and a counter is nonzero, reads a pointer from a list, stores it, and resets it to a default when the next entry is negative, otherwise it returns.
- `fill_nibble_packed_lookup_table`: Fills 320 table entries, each storing the index's high part shifted left by one and a packed word built from the index's high and low nibbles combined with a certain constant, then returns.
- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_update_audio_registers`: Increments a global byte, checks if it exceeds a certain limit, and if so, resets it to zero and calls a routine to update the certain audio registers (AUD0LCH-AUD3LCH) using values or pointers from a table.
- `update_object_position_and_state_2`: Updates a 16-bit position or pointer in a structure based on a masked button or direction input, potentially performing a boundary check or updating hardware registers, and contains a loop that iterates through a range of pointers.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_position_and_state`: Updates a 16-bit position or pointer in a structure based on a masked button or direction input, potentially performing a boundary check or a reverse-search loop through a table to find a valid or or clamped value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `search_table_and_reset_audio`: Sets a global variable to a maximum value, calls a library function, and then searches a table for two specific word values; if found, it increments or decrements two pointers based on a comparison with a keyboard or input register, resets several audio volume registers and a DMA/display register, and calls another routine before returning.

### logged disk load from International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf

- `update_bitmask_and_buffer`: Performs a series of bitwise logical operations on 16-bit and 32-bit registers, then enters a loop that updates a buffer or table using bitwise OR and AND operations, or otherwise clears several bytes and a 16-bit value in a global memory area before returning.

### logged disk load from International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf

- `apply_bitwise_logic_to_registers`: Performs a series of bitwise AND, NOT, and OR operations on d1 through d6, then loads a value from a global pointer, swaps its halves, and loads two bytes from a buffer into d6 and d7.

### logged disk load from International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf

- `update_global_state_with_logic`: Performs bitwise logic on d1 through d6, then if a certain condition is met, it enters a loop that updates several global memory locations using bitwise operations, and finally clears several global variables or sets them to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_bitwise_updates_to_memory`: Executes a loop that performs bitwise AND, OR, and NOT operations, and writes the results to a global pointer and several offsets in a structure.
- `apply_bitwise_or_to_registers`: Performs bitwise AND and OR operations on d1, d4, d5, and d6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_with_bitwise_and`: Performs a bitwise AND on d2 and d4, then branches to a fixed address.

### International Soccer Challenge (1989)(Micro Style)[cr PDX][h Spaikers][t +1 PDX].adf:Intro.Pdx#unpacked

- `initialize_system_and_game_loop`: Clears several RAM regions, initializes multiple lookup tables, and sets the bitplane pointer registers.
- `update_audio_channel_registers`: Checks a global variable and, if non-zero, writes the value 24596 to three specific offsets within a structure pointed to by a value on the stack, then returns.
- `update_global_values`: Iterates through a table of words, calling another routine for each entry, and writes specific constant values to a global variable based on the value found in the table.
- `draw_text_glyphs`: Iterates through a null-terminated string, calculating a source address for each character based on an offset from a font table, and copies glyph data to video memory.
- `draw_text_glyphs_to_video_memory`: Iterates through a null-terminated string, calculating a source address for each character based on an offset from a font table, and copies 8 bytes of glyph data to a destination address in video memory that increments by a fixed amount for each character processed.
- `step_counter_toward_joystick_value_2`: Reads a joystick direction value from the fire-button and disk status port and steps a stored counter up or down toward it, updating a state word across several calls.
- `poll_keyboard_status`: Disables interrupts via the CIAA control register, reads the keyboard serial register, clears the serial register, re-enables interrupts, and then performs a bitwise inversion and a right rotation on the read byte before storing it into a global variable if bit 7 of the result is clear; otherwise it returns.
- `generate_color_palette_table`: Fills a table in nested loops by writing a four-byte sequence consisting of a base byte, a calculated value, an incremented base byte, and a null terminator, then copies these sequences into a larger buffer using a secondary loop.
- `increment_color_table_by_flag`: Checks a global flag to determine an increment or decrement value, then loops four times to add that value to specific bytes at fixed offsets within a table, toggling the increment value between positive and negative after each iteration.
- `wait_and_set_structure_values`: Waits in a loop until bit 14 of the byte at offset 2 of the structure in a6 is cleared, then stores an address, 0, an address, and 9876 at offsets 84, 100, 64, and 88 of that structure, and finishes by decrementing a counter from 256 until it underflows.
- `set_bitplane_bits_from_lookup_2`: Iterates through a nine-by-thirty-two grid of source bytes using two nested loops.
- `initialise_lookup_tables_2`: Subtracts 4112 from each of seven consecutive words in a global table while calling a routine for each, then fills 256 entries at 7a000 with multiples of 40 and 512 entries at 7a800 with a right-shifted index and its inverted low three bits.
- `process_wait_structure_2`: Decrements a counter that resets to 8 when it reaches 0, skips past empty slots in a table, then loops 38 times copying a value and adding it to a field while calling a wait routine before returning.
- `store_structure_fields_wait_2`: Reads a value from a structure field and computes an address from another field, then waits until a status bit clears before storing several words and long words into fixed offsets of a structure.
- `wait_and_configure_port_3`: Waits in a loop until bit 14 of the byte at offset 2 of the structure in a6 becomes clear, then stores the values an address, 0, an address and 4052 at offsets 84, 100, 64 and 88 of that structure, then returns.
- `generate_data_tables`: Fills a series of memory locations with two sets of incrementing and decrementing values based on two loops, then clears a range of memory starting at a specific address.
- `clear_sample_ram_2`: Points a0 to an address derived from an address, then writes zeros to every 40th pair of consecutive bytes starting there, repeating the store 37 times, and returns.
- `toggle_and_advance_pointer_list`: Toggles the least significant bit of a global variable and, if the bit was previously set and a counter is non-zero, retrieves a word from a pointer list and advances the pointer; if the next word in the list is negative one, it resets the pointer to a default address and clears the counter, otherwise it returns.
- `fill_nibble_packed_lookup_table_2`: Fills 320 table entries, each storing the index's high part shifted left by one and a packed word built from the index's high and low nibbles combined with a fixed constant.
- `build_sample_pointer_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several audio volume registers, and returns.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_update_audio_channels`: Increments a byte in a global variable and, if it reaches a specific threshold, resets it and calls another routine four times to update the four audio channel registers and DMA control with values derived from a table.
- `update_audio_and_dma_parameters`: Processes a command structure to update audio channel settings, DMA control registers, and hardware registers by reading values from a table and writing them to the structure and hardware.
- `mask_and_compare_word_2`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_structure_values`: Processes a command byte from a structure in a6 to perform various arithmetic operations on 16-bit and 8-bit values at specific offsets, including clamping values to limits, updating a 16-bit value with a masked version, and toggling bits in the CIAA peripheral register.

### MUSIC.BIN (raw code file, load address unknown)

- `swap_registers_and_return`: Saves a large set of registers to the stack, calls another routine, restores the registers, and returns.
- `execute_subroutine_list`: Iteratively calls a series of subroutines, each saving and restoring a register, or calls a routine that clears and sets multiple registers before returning.
- `init_chain_structures_2`: Sets a bit in a CIAA_PRA register, calls two other routines, and then executes a loop that populates four consecutive structures in a chain with pointers, constants, and a calculated jump target, while reading an index-based value from a table and clearing certain global variables.
- `clear_and_set_audio_params`: Clears a series of memory locations using a pointer-based offset and sets the volume or certain DMA/audio control registers to specific values, then returns.
- `set_four_global_flags_2`: Stores the value one into four bytes of a global structure at offsets 526, 542, 550, and 558, then returns.
- `enable_audio_channels_2`: Checks a global flag and, if non-zero, uses a loop to set the first four bits of a value and writes it to the DMA or audio control register, then updates a follow-up flag and returns; otherwise it returns immediately.
- `set_byte_at_offset`: Writes the value 3 to an offset of 5ef relative to the program counter and writes the value 3 to an offset of 5f0 relative to the program counter, then returns.
- `update_relocation_table_entries`: Iterates through a table of structures, calculating and updating pointer targets and relative offsets, and updates a hardware register or a global variable with a new value or a bitmask, then returns or otherwise returns.
- `process_chain_entries_2`: If a flag in a structure is zero, it iterates through a chain of eight structures, calculating a new pointer and a value derived from a division, storing these into a destination structure, then updates a pointer and a counter in the source structure before returning.
- `store_index_and_clear_field`: Sets bit 1 of a global variable, loads 255 into it, then clears one of four one-byte fields at offsets 1485, 1493, 1501 and 1509 depending on whether the incoming data byte is zero, one, two or three, stores that byte and the other incoming byte into globals, and returns.
- `spin_short_delay_loop`: Counts a loop variable down from five hundred and twelve to zero and returns, giving a short busy-wait delay.
- `step_channel_envelope_and_slope`: Advances one channel's envelope counter and, when it expires, writes a stored value to the DMA control cell and clears a flag; otherwise it applies a pitch slope and a vibrato step to the channel period.
