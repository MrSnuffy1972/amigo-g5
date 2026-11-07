# Game 1582 (pacmania (1988)(grandslam)[cr wild riders]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01582). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (939136 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 394 raw-word lines |
| Functions named and described | pass, 256 of 256 named (0 author symbols), 256 of 256 described (22 by rule, 234 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3445 of 3445 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5694 of 6836 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4898 |
| code | 27198 |
| data | 8494 |
| padding | 498 |
| unused | 898048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pacmania (1988)(Grandslam)[cr Wild Riders].adf:bootblock | 1 | 6 | 1 |
| logged disk load from Pacmania (1988)(Grandslam)[cr Wild Riders].adf boot-block load | 1 | 110 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6720 | 241 |

## Cases handled

- pacmania (1988)(grandslam)[cr wild riders]/Pacmania (1988)(Grandslam)[cr Wild Riders].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 3445 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pacmania (1988)(grandslam)[cr wild riders]/Pacmania (1988)(Grandslam)[cr Wild Riders].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk/track/sync check. Pass path: the branch where bVar3 is false (the break/exit condition).

## Functions


### Pacmania (1988)(Grandslam)[cr Wild Riders].adf:bootblock

- `execute_library_call`: Calls a library function using a pointer obtained from a libraryget function.

### logged disk load from Pacmania (1988)(Grandslam)[cr Wild Riders].adf boot-block load

- `execute_empty_sequence`: The routine is empty, consisting only of a sequence of calls to other routines.
- `initialize_library_pointers`: Calls two library functions and stores a constant value into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `clear_memory_range`: Zeros out a block of memory starting from a fixed address for a certain number of iterations.
- `update_function_pointers`: Updates a series of global function pointers with new or previously stored addresses.
- `execute_two_subroutines`: fetches two function pointers from a table or memory and executes them sequentially.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `execute_indirect_pointers`: fetches two function pointers from a table relative to a base address and executes them sequentially.
- `execute_indirect_calls`: Fetches and executes two function pointers from memory relative to a base address.
- `wait_for_register_bit`: Loops until a specific bit in a global variable or hardware register is cleared or a register matches a constant.
- `set_hardware_registers`: Repeatedly writes a constant value to a global variable.
- `decrement_register_to_zero`: Decrements a register or variable until it reaches a certain value.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_to_ram`: Stores a word into two consecutive memory locations.
- `update_global_with_offset_address`: Calculates a new address by adding an offset to a pointer and stores the result in a global variable.
- `return_global_value`: Returns the value stored in a global variable.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `reset_memory_values`: Clears a memory location and writes a sequence of constant values to a series of memory locations relative to a pointer.
- `copy_memory_to_audio`: Stores several words into memory and then copies a range of memory or hardware registers using a loop.
- `copy_memory_block`: Copies a fixed number of words from a source pointer to a destination pointer.
- `modify_memory_with_pointer_logic`: Performs bitwise operations and arithmetic on memory locations or registers using calculated pointers.
- `set_global_to_constant`: Calls another routine and then sets a global variable to zero or a constant value based on bitwise checks of a global variable or a register-related variable.
- `update_pointer_range_2`: Compares a value from a hardware port against a global variable and, if not equal, updates that global variable with the port value and performs a subtraction if the result is negative.
- `update_pointer_range`: Adds a byte from a register to a global variable and stores it if the result is within a certain range, then calculates a difference between high bytes of two registers and updates a global variable if the result is within a certain range.
- `update_global_pointer_with_offset`: Loads a byte from a register, adds it to a value from a global memory location, and stores the result back to that same memory location if the result is not negative or below a certain threshold.
- `compare_register_with_memory`: Compares a register against several memory locations and branches if a match is found.
- `check_register_against_memory`: Compares a register against two memory locations and branches if a match is found.
- `clear_global_variable`: sets a global variable to zero.
- `compute_register_flags`: Calculates a new value based on several bitwise checks of a global variable and a sequence of calls to other routines, potentially setting a global variable to a constant.
- `execute_subroutine_chain_2`: Calls a sequence of other routines and returns a value that may or may not have a bit masked or set based on a flag.
- `update_global_state_2`: Sets a global variable to a constant, increments a global counter, and conditionally calls other routines or updates global variables based on certain conditions.
- `clear_interrupt_request_bits`: Clears specific bits in a global variable, which corresponds to clearing certain interrupt request flags.
- `initialize_game_state_and_tables`: Clears a range of global variables and memory-mapped registers, then populates or resets certain data tables or string buffers with specific or zeroed values.
- `call_subroutines`: This routine calls two subroutines in sequence.
- `initialize_memory_and_registers`: Clears a range of memory with zeros and writes a sequence of constant values to a table or memory-mapped registers.
- `update_display_attribute_tables`: Updates multiple memory buffers by copying or masking attribute data from a source table using nested loops.
- `bulk_copy_memory_tables`: Performs a large-scale copy of data between multiple memory buffers using nested loops.
- `initialize_data_tables`: This routine performs a recursive setup that initializes several memory buffers with constant values and then populates large tables by copying or distributing bytes from a source table using an index-based lookup.
- `broadcast_global_variable`: copies a single global variable or register value into a contiguous sequence of seven memory locations.
- `set_global_values_2`: Writes a series of constant values and a calculated address into a sequence of global variables.
- `clear_data_register`: Performs a logical OR operation on a data register with zero, which does not change its value.
- `clear_d0_register`: clears the least significant byte of a data register to zero.
- `update_function_pointers_3`: Updates a table of function pointers with new addresses and executes a chain of subroutines while modifying a global counter.
- `update_global_values_2`: This routine updates a global variable and a second global variable using arithmetic and bitwise operations, and also updates a third global variable with a value derived from a register.
- `modify_global_variables`: Performs several arithmetic and bitwise operations to update various global variables.
- `clear_memory_block`: clears a block of memory by storing zero into a pointer-based destination for a fixed number of iterations.
- `set_blt_registers`: writes specific constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers.
- `clear_d0_register_2`: clears the d0 register using a logical or instruction.
- `clear_d0_register_3`: clears the least significant byte of a data register.
- `clear_d0_register_4`: clears the least significant byte of the d0 register to zero.
- `update_pointer_from_table`: Calls another routine and then potentially decrements a routine pointer or updates a global variable with a value loaded from a table using an offset.
- `update_global_variable_via_table`: Checks if a global variable is zero and, if so, recursively calls itself while updating a pointer and a secondary global variable.
- `update_global_with_table_lookup`: increments a global variable and then replaces its value with a value fetched from a table using a bit-masked or shifted index.
- `tiny_ori_subi`: Small routine of 2 instructions that merges 3 into d1, then subtracts 1 from d3.
- `test_register_bits`: Clears a global variable, calls another routine, and then performs a series of bit tests on a data register to branch to different locations.
- `update_state_and_pointer`: Checks bits in a global variable to decide whether to update a global variable with a constant and increment or decrement a global pointer or index.
- `update_global_from_table`: This routine checks if a global variable is non-zero, and if so, potentially loads a value from a table into that global variable or sets it to a constant.
- `process_table_values`: iterates through a table, calculating a new value based on the sum of the nibbles of a word and storing it back.
- `construct_ascii_character`: Extracts specific bits from registers to form a new value and returns a character or a modified character if the value exceeds a certain threshold.
- `save_registers_and_modify_d2`: saves registers to the stack, clears d0 and d1, moves d4 to d1, clears d4, and performs a rotate or shift on d2.
- `return_stack_value`: This routine calls a subroutine and returns the value that was present on the stack at the start of the routine.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `apply_masked_transform`: Calls another routine and returns the original register contents.
- `apply_masked_transform_to_tables`: iterates through a loop, reading a value, applying bitwise negation or masking based on certain flags, and storing the result into three different memory destinations.
- `set_blt_attributes`: writes constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers.
- `update_struct_fields`: Updates several fields in a structure or block of memory using a pointer, including an incremented value and a value added to a register.
- `iterate_string_and_call`: Iterates through a null-terminated string and calls another routine for every non-zero byte encountered.
- `scan_string_for_carriage_return`: Iterates through a memory buffer, checking for a carriage return character and calling another routine if the character is not a carriage return.
- `calculate_table_offsets`: Calculates a pointer or offset based on a global variable and writes a constructed word containing a calculated value and a pointer-derived value to multiple memory locations.
- `branch_to_address`: Performs a short branch to a specific address.
- `calculate_pointer_table`: This routine iterates through a table to calculate and store certain pointer-related or display-related values into a destination table, while also updating several hardware registers and memory locations.
- `execute_subroutine_chain`: This routine calls several subroutines, loads a value from a global pointer into a register, and performs a branch to another routine.
- `reorder_pointer_table_2`: Initializes several hardware registers and then iterates through a table, updating certain memory locations with offset pointers if the source value is non-zero.
- `copy_memory_block_2`: Copies a fixed number of longwords from a starting global variable address to a destination address using a loop.
- `reorder_pointer_table`: If a certain global flag is set, this routine populates a table with a sequence of pointers.
- `tiny_ori_2`: Small routine of 1 instruction that merges 62 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 110 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 158 into d0.
- `tiny_ori_5`: Small routine of 1 instruction that merges 206 into d0.
- `update_pointer_table`: Iterates through a table to calculate and store new coordinate or index values based on existing pointer or table data.
- `tiny_ori_6`: Small routine of 1 instruction that merges 0 into (a0)+.
- `update_pointer_table_copy`: Performs a loop that copies or updates a table with calculated values derived from a pointer or global variable.
- `process_table_indices`: Iterates through a table to check or update certain values and potentially increment a counter or modify a byte if a certain condition is met.
- `load_table_pointers`: Loads a constant into a register and initializes two pointers using values from memory.
- `calculate_display_attributes`: Iterates through a table to calculate and store display-related or pointer-related values into a destination table, including certain bitwise or arithmetic offsets.
- `load_pointer_and_set_counter`: loads a pointer from a global variable into a register and initializes a register with a constant value.
- `clear_table_values`: Iterates through a table and sets a value to zero if it is not already zero, while also setting certain hardware or global registers.
- `clear_bit_in_register`: Clears a specific bit in a register.
- `set_bit_at_address`: Sets a specific bit in a byte located at an address pointed to by a register.
- `clear_register_bits`: Performs a logical OR operation with zero on a register, which does not change its value.
- `clear_d0_register_5`: Clears the least significant bit of a data register.
- `set_d0_and_memory`: Sets a data register to a constant value and writes a constant longword to a memory location pointed to by a register.
- `tiny_ori_7`: Small routine of 1 instruction that merges 160 into d0.
- `set_d0_multiple_times`: Performs a series of or operations to set certain bits in a data register.
- `update_blt_attributes`: Updates a table of bitplane and attribute data, including bitplane counts and certain register-like values, by reading from a source table and writing to a destination table.
- `clear_d0_register_6`: clears the d0 register with a zero value.
- `clear_d0_register_7`: clears the lowest byte of the d0 register with a zero-valued or-instruction.
- `clear_d0_register_8`: Clears the lowest byte of the d0 register to zero.
- `clear_register_d0`: clears the least significant byte of register d0 to zero.
- `clear_data_register_low_byte`: clears the lower byte of a data register to zero.
- `clear_d0_register_9`: clears the lowest byte of the d0 register with a zero-fill instruction.
- `update_function_pointers_2`: Updates a global function pointer and several other global variables or pointers based on a value read from a table.
- `copy_data_block`: Decrements a global counter and copies a series of bytes and words from a source buffer to a destination buffer.
- `clear_color_register`: Sets a range of memory locations, starting from a hardware register, to zero.
- `recursively_call_routine`: The routine calls itself twice before returning.
- `update_memory_and_loop`: Copies values to a memory area, then enters a loop that increments a global variable and checks a condition, potentially updating a bit in a global variable and calling another routine.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `search_and_rotate_bytes`: Searches a memory area for a specific pattern, then rotates bits of a returned value and stores the resulting bytes into a destination buffer.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_hardware_register`: Reads a byte from a hardware register, clears a bit, increments the value, and writes it back to the same location.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is unable to be cleared.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: This routine compares two global variables and, depending on which is larger, modifies a memory location and then enters a loop that increments or decrements a global variable while calling another routine.
- `decrement_and_mask_register`: Decrements a register until a bit in a global variable is cleared, then masks a bit and writes a value to a hardware register or memory, followed by a recursive call and a fixed-count loop of another routine.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 1 into d0.
- `test_bit_in_register`: Tests a bit in a data register.
- `test_bit_in_data_register`: tests a specific bit within a data register and updates the condition codes.
- `test_two_bits_in_register`: Tests two bits in a data register sequentially.
- `test_bit_against_itself`: Tests a bit in a data register against itself.
- `validate_and_update_tile_state`: Calculates a new index or position based on a pointer and a table, then checks if certain bit-flags or conditions are met.
- `calculate_sprite_attributes`: Performs complex arithmetic on a value from a table to calculate and store new values into a global table.
- `set_pointer_attributes`: Iterates through a pointer-based list, clearing a byte at an offset and setting a bit in a memory location based on a returned value.
- `apply_multiple_or_masks`: Applies a series of OR masks to a data register.
- `determine_and_store_character`: Determines a character value based on checks of a memory location and stores it to a memory location, while returning a byte from a memory location.
- `determine_character_to_store`: Checks several memory locations and uses conditional logic to calculate a character value and store it in a global variable.
- `increment_pointer_value`: Increments a byte at a pointer offset and conditionally increments a global variable or a secondary global value based on a check.
- `store_subroutine_result_2`: Retrieves a global value and calls a routine that determines a character value and flips a bit before storing a 1updated 16-bit return value.
- `execute_character_logic`: A short routine that calls a routine to determine and store a character based on memory checks.
- `store_subroutine_result`: This routine calls a routine that determines a character value and calls another routine that flips a bit, then stores a 16-bit return value into a memory location relative to a pointer.
- `determine_and_flip_bit`: Calls a routine to determine and store a character value, then calls another routine to flip a bit in a target byte if a certain condition is met.
- `flip_target_bit`: If the zero flag is not set, it flips a specific bit in a byte at an offset from a pointer.
- `execute_character_logic_2`: Calls a sequence of routines that determine and store character or return values into memory locations.
- `update_string_buffer`: Increments a byte at an offset from a pointer and updates a character or a pointer-based value based on a table lookup.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 255 into d1, then returns.
- `modify_pointer_and_byte`: Tests a long-word and, if the zero flag is not set, writes a byte to a pointer offset and conditionally decrements a pointer-based value.
- `modify_memory_on_flag`: This routine checks a zero flag and, if set, writes a byte to a memory offset and potentially increments a value at a pointer-based address.
- `increment_pointer_value_2`: Calls another routine and, if a zero flag is set, checks a memory location and a global variable, then updates a memory location and increments or wraps a value at a pointer offset.
- `decrement_pointer_value`: Calls another routine and, if a zero flag is set, checks a memory location and a global variable, then updates a memory location and decrements or wraps a value at a pointer offset.
- `iterate_structure_pointers`: Iterates through a list of structures, decrementing a counter-based value in a structure and conditionally updating a pointer or a global variable.
- `calculate_table_attributes`: Performs complex arithmetic using a table, applies bitwise masks, and conditionally updates bytes in a structure or a global table, while checking a global variable and calling other routines.
- `update_masked_byte`: Clears a global variable and conditionally updates a byte in a structure using a bit-shifted and masked value based on certain memory checks.
- `update_table_attributes`: Calls a routine that performs arithmetic and bitwise masking to update a table or structure.
- `execute_subroutine_chain_3`: Executes a sequence of two calls to a routine that calculates table attributes.
- `decrement_and_set_flag`: This routine checks a global variable and, if it is zero, potentially decrements a byte at a certain offset and calls another routine to update a global variable or a memory byte.
- `compare_memory_threshold`: Loads two pointers, sets a register, calls another routine, and then performs comparisons against a byte at a pointer offset, branching based on the result.
- `update_global_variable_from_table`: Checks if a byte at a memory location is below a certain threshold, then sets a global variable to a fixed value and updates it with a byte retrieved from a table using an offset derived from a previously read byte.
- `calculate_vector_distance`: Calculates the absolute difference between two 16-bit or 8-bit values stored at offsets from two different pointers and returns a concatenated result.
- `update_bit_masked_value`: Checks certain conditions in a structure and, if a bit is not set, updates a byte in memory with a bit-shifted and masked value.
- `test_and_branch`: Tests a long-word value and branches or clears a register before returning.
- `test_long_word_and_branch`: Tests a long-word value and either branches or sets a register to one before returning.
- `test_and_branch_logic`: Tests a long-word and branches or clears a register, potentially creating a loop if a branch targets an earlier instruction.
- `test_bits_and_branch`: Tests two bits in a register and branches or clears a register, potentially creating a loop if a branch targets an earlier instruction.
- `store_routine_result`: Calls another routine and stores its 16-bit return value into a memory location at an offset from a pointer.
- `mask_and_fetch_index`: masks a word from a global variable and uses the result to fetch a new value from a table, then calls another routine.
- `calculate_bit_shifted_value`: Performs bitwise shifts and masking on registers and returns a calculated result.
- `modify_byte_with_mask`: Updates a byte in memory by oring it with a constant and a masked version of another value.
- `update_coordinate_values`: Modifies two values in a structure based on bits in a returned value, including wrap-around checks for a certain range.
- `execute_complex_jump_table`: Uses a value from a global variable to perform a switch-case jump, potentially modifying a global pointer and returning a masked address.
- `execute_complex_jump_table_copy`: Performs a jump-table-based branch or pointer modification based on a value read from a global variable.
- `update_routine_pointer_from_table`: Decrements a routine pointer and, if that pointer reaches zero, loads a new value from a table into the pointer variable.
- `update_table_pointer_2`: Decrements a routine pointer and updates a global variable with a value loaded from a table if the variable is zero.
- `conditionally_update_pointer`: Checks if a global variable is a multiple of four and returns a masked value, otherwise performs a routine pointer decrement and a table-based update if the variable is zero.
- `tiny_ori_8`: Small routine of 1 instruction that merges 135 into d0.
- `execute_string_loop`: Iterates through a null-terminated string, calling another routine for each non-zero byte, and modifies a pointer and a global variable.
- `modify_stack_and_pointer`: Decrements a pointer, moves a value from the stack to a pointer-relative address, decrements a byte in a table, and manipulates the stack.
- `update_global_variable_with_constant`: Checks a global variable and, if it does not equal a specific value, sets it to a constant value derived from a bitwise inversion.
- `parse_control_string_3`: Sets a global variable to a character-based value and loops while checking a global variable against a constant or a bitmask.
- `reset_audio_and_global_state`: Clears audio-related registers and resets several global variables to a default state.
- `initialize_memory_buffer`: Iterates a fixed number of times, writing certain constant values to a sequence of memory locations relative to a pointer.
- `tiny_ori_9`: Small routine of 1 instruction that merges 5 into d1.
- `update_string_state`: Updates a function pointer, modifies a global variable, and executes a loop that decrements a counter or a pointer-relative value.
- `clear_register_d0_2`: Performs a logical OR operation on a register with zero, which has no effect on the register value.
- `clear_d0_register_13`: Clears the contents of the data register zero by performing a logical OR operation with zero.
- `reset_hardware_and_branch_to_subroutine`: Clears a range of memory starting from a hardware register, resets an audio volume register, and then executes a routine at an address calculated from a global variable and a register value.
- `set_and_test_byte`: Sets a byte in a register to a constant value and compares a value at a memory location to a constant, branching if equal.
- `initialize_engine_state_2`: Initializes engine state by resetting global pointers and constants, then performs a series of block memory copies to populate or reset internal tables or data structures.
- `decrement_global_pointer`: Modifies a register and decrements a global word-sized pointer.
- `reset_global_variable`: Sets a specific global variable to zero.
- `decrement_and_test_global_counter`: Checks if a global word-sized variable is non-zero, and if so, decrements it by one before returning.
- `initialize_and_update_table`: Initializes a memory buffer with a specific byte pattern and then enters a loop that increments a global counter, updates a pointer-based table, and performs conditional checks or modifications to table entries based on the current counter value.
- `update_engine_state_and_table`: Increments a global counter and iterates through a loop to modify a pointer-based table, checking certain bit-flags and string-like characters.
- `increment_global_counter`: Increments a global variable if certain conditions involving other global variables are met, and returns a masked register value.
- `compare_and_branch`: Compares a byte at a specific memory address to a constant and branches if they are equal.
- `initialize_engine_state`: Sets up initial global pointers and constants, resets certain memory-resident flags, and executes a sequence of setup routines to prepare internal data structures and registers for operation.
- `set_and_test_byte_flag`: Sets a bit in a data register and checks if a byte at a certain memory location is zero, branching if so.
- `initialize_sprite_and_blt_tables`: Initializes several global variables and then performs a series of nested loops to copy or redistribute blocks of data between multiple memory buffers.
- `process_string_and_table`: Iterates through a memory buffer to check for a carriage return, copies or modifies data in a loop, and updates a global variable using a table lookup.
- `update_offset_with_masked_bits`: Extracts certain bits from a global variable and adds them to a byte at an offset from a pointer, with a conditional check for a specific bit pattern.
- `update_table_index_and_load_value`: Checks a global variable against a constant, and if a certain condition is met, resets it and recursively calls itself.
- `parse_control_string`: Iterates through a string, updating a global variable based on certain characters encountered.
- `parse_string_for_control_chars`: Iterates through a null-terminated string, updating a global variable or skipping bytes based on certain character values.
- `execute_infinite_loop`: Calls another routine and then enters an infinite loop.
- `update_hardware_registers`: Decrements a global variable and, if it reaches zero, updates a series of hardware or memory registers using values from a table or a calculated address.
- `set_initial_registers`: Sets a global variable and certain registers to specific constant values, and may call itself recursively.
- `set_global_values`: Stores a series of constant values into a global variable.
- `update_global_with_decrement`: Checks if a global variable is non-zero and decrements it, then stores a sequence of constant values into a global variable.
- `iterate_string_and_call_2`: Iterates through a null-terminated string starting from a global variable and calls another routine for every non-zero byte.
- `initialize_hardware_table`: Loads a hardware register and writes a calculated address or constant to a memory area.
- `initialize_table_entries`: Writes several constant values and a register value to a table or memory area using an offset from a pointer.
- `set_blt_registers_2`: Sets several hardware registers or memory locations to specific values based on a global variable comparison.
- `parse_control_string_2`: Iterates through a string, updating a global variable or resetting it based on certain characters encountered.
- `parse_control_string_logic`: Iterates through a string and updates a global variable based on certain characters encountered.
- `fill_memory_with_routine`: Executes a routine repeatedly while decrementing a value, effectively performing a loop-based memory write.
- `configure_memory_block`: Writes a sequence of constant values and calculated pointer or register values to a memory area using a pointer.
- `setup_blt_registers`: Sets certain hardware registers or custom memory-mapped registers to specific constant or calculated values.
- `save_registers_and_branch_2`: Saves all data and address registers to the stack and performs an unconditional short branch to another routine.
- `setup_string_execution`: Saves registers to the stack, loads a long-word into a register, loads an address into another, and performs an unconditional branch.
- `save_registers_and_branch`: Pushes all data and address registers onto the stack and performs a relative branch.
- `update_global_state`: Checks a global variable and, if certain conditions are met, modifies a global variable and a secondary global value.
- `parse_string_with_pointer`: Iterates through a string using a pointer and updates a global variable based on certain characters encountered.
- `format_numeric_string`: Calculates digits and stores them as characters into a series of memory locations.
- `load_from_offset_table`: Loads a word from a memory location calculated by adding an offset and a register to a register.
- `modify_and_load_byte`: Loads a word from a memory location, applies a bitwise OR to a byte at that location, and then loads the modified value into a register.
- `set_blt_parameters`: Writes constant values to certain hardware or custom registers.
- `traverse_string_and_execute`: Iterates through a null-terminated string of code pointers and executes each routine pointed to by the string.
- `recursively_initialize_memory_buffer`: Recursively calls itself based on the state of a global variable, and if a certain condition is met, writes a sequence of constant values to a memory buffer via a pointer.
- `decrement_and_copy_memory_block`: Iterates a fixed number of times, copying a value from a memory location and decrementing a pointer and a counter.
- `prepare_string_traversal`: Saves registers to the stack, loads a long-word into a register, and performs an unconditional branch.
- `call_another_routine`: executes another routine and returns.
- `copy_memory_block_3`: Copies a fixed number of words from one memory location to another, decrementing a counter or pointer-related value during the process.
- `reset_game_tables_and_memory`: Clears a range of global variables and memory-mapped registers, then populates or resets certain data tables or string buffers with specific values.
- `apply_no_op_to_register`: Performs a bitwise OR operation on a register with zero, which does not change the register's value.
- `enter_infinite_loop`: The routine enters an infinite loop that does not exit.
- `write_constant_sequence`: Writes a sequence of constant values to a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 222 into d0, then merges 0 into d0.
- `update_table_pointer`: Checks a global variable and, if a certain condition is met, loads a value from a table into a global variable.
- `execute_conditional_routine`: Checks if a routine pointer is null, updates a global variable, and then calls a routine.
- `initialize_data_structures_2`: Checks a global variable and, if zero, writes a constant value to a memory location, updates a global variable, and writes several constant values to a memory area relative to a pointer.
- `initialize_function_pointer`: Writes a routine address and a constant value to a memory location relative to a pointer.
- `initialize_data_structures`: Writes several constant values to memory locations relative to a pointer and updates a global counter or flag.
- `initialize_data_table`: Checks a global variable and, if zero, writes several constant values to a table, otherwise increments a global counter and may reset certain values.
- `increment_counter_and_reset`: Increments a global counter and, if it reaches a certain value, resets a global variable and a pointer-relative memory location.
- `update_function_pointer_table`: Checks a global variable and, if zero, writes a routine address and constants to a table, otherwise increments a counter and resets certain values if a limit is reached.
- `increment_and_reset_counter`: Writes constant values to a table if a global variable is zero, otherwise increments a global counter and resets certain variables if a limit is reached.
- `initialize_table_with_constants`: Writes a routine address and constant values to a table if a certain condition is met, otherwise increments a counter and resets certain values if a limit is reached.
- `initialize_memory_and_increment`: Loads a value from a hardware port, and if non-zero, writes several constant values to a memory location relative to a pointer and increments a global variable.
- `increment_and_compare_global_value`: Increments a word-sized global variable and compares it against a constant.
- `initialize_data_structures_3`: Checks a global variable and, if zero, writes several constant values to a memory location, updates a global variable, and increments a global counter.
- `clear_d0_register_10`: Clears the least significant byte of a data register using a logical or instruction.
- `clear_d0_register_11`: clears the least significant byte of the d0 register.
- `clear_d0_register_12`: clears the low byte of a data register to zero.
- `update_global_values`: Updates several global variables and a data register with specific constant values.
- `tiny_ori_10`: Small routine of 1 instruction that merges 54 into d1.
- `tiny_ori_11`: Small routine of 1 instruction that merges 174 into d1.
- `tiny_ori_12`: Small routine of 1 instruction that merges 38 into d1.
- `tiny_ori_13`: Small routine of 1 instruction that merges 0 into d6.
- `tiny_ori_14`: Small routine of 1 instruction that merges 0 into d7.
- `update_audio_and_table_pointers`: Iterates through a list of table entries to update audio channel or length registers and update pointers or or length-related values.
- `traverse_pointer_list`: Calculates a pointer using a global variable and an offset, then recursively traverses a list of shorts until a negative value is encountered.
- `recursively_update_register`: Clears a register, calls itself recursively, and then conditionally updates a register or a global variable or copies a value to a register.
- `clear_audio_volume`: Clears a word-sized area of the audio volume hardware register.
- `clear_global_variables`: Sets a global variable to zero multiple times.
