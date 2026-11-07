# Game 117 (SuperCars v1.1 1527): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00117). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1293841 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 307 of 307 named (0 author symbols), 307 of 307 described (8 by rule, 299 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4795 of 4795 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7492 of 7499 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 234357 |
| code | 33952 |
| data | 1025520 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SuperCars.Slave | 1 | 320 | 25 |
| logged disk load from Disk.1 | 1 | 5595 | 211 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 731 | 29 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 57 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 790 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |

## Cases handled

- SuperCars_v1.1_1527/SuperCars/Disk.1: raw disk image (337920 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- SuperCars_v1.1_1527/SuperCars/Disk.2: raw disk image (915456 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 7 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SuperCars_v1.1_1527/SuperCars/Disk.1: 7 logged loads cover 239896 bytes of the image
- SuperCars_v1.1_1527/SuperCars/Disk.1: chunks 148-159 (bytes 37888-40960) hold ASCII text
- 2 packed track region(s), 1010176 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SuperCars.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_callbacks`: Calls several functions through pointers retrieved from a table or global variable and performs conditional logic based on their return values.
- `wait_for_bit_transitions`: Repeatedly polls a global variable for specific bit states and waits for a single bit to toggle between zero and non-zero states for a fixed number of iterations.
- `check_status_conditions`: Checks a global variable and a bit in a system memory location to determine if specific bitwise conditions are met, returning a bit from the global variable.
- `xor_global_variable`: Performs a bitwise XOR operation on a global variable using a constant value.
- `update_callback_table`: If a specific value is in a register and a global variable is non-zero, it stores a constant into a table.
- `modify_callback_table`: If a bit is set in a register and the register contains a specific constant, it updates several entries in a table with function pointers and copies a block of data from a source to a destination.
- `wait_for_flag_clear`: The routine enters a loop while a specific bit in a global variable remains set, then updates that global variable with a value from a register.
- `spin_in_infinite_loop`: The routine enters an infinite loop and never returns to its caller.
- `update_function_pointers`: Checks specific memory locations for constant values and, if matched, stores a word and a function pointer into those locations.
- `store_constant_word`: Stores a two-byte constant value into a memory location offset from a pointer held in a register.
- `xor_memory_block`: Iterates through a block of memory, performing a bitwise XOR between a register value and each byte in the block.
- `xor_memory_block_2`: Performs a bitwise XOR operation between a register value and every byte within a specified memory block.
- `execute_conditional_callback`: Calls a function through a pointer stored in a table and, if the result is non-zero, executes another function through a different pointer and calls itself recursively.
- `copy_memory_and_call_indirect`: Copies a block of data from one location to another and executes a function through a pointer if a global variable is zero.
- `execute_indirect_calls`: Loads a pointer from a global variable, calls a function at an offset within that pointer, and if the result is non-zero, calls another function at a different offset and sets a global variable to a constant.
- `load_function_table`: Copies several values from a source table into a series of global variables and executes an indirect function call if a global variable is zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_trap_and_modify`: Executes a software trap and returns a value formed by bitwise ORing a register with a constant.
- `tiny_ori_2`: Small routine of 1 instruction that merges 136 into d0.
- `return_register_byte`: Returns the byte currently held in a register.
- `return_uninitialized_byte`: This routine returns an uninitialized byte from the stack.
- `return_uninitialized_byte_2`: This routine returns an uninitialized byte from a local stack variable.
- `return_register_value`: Returns the value currently held in a data register.
- `handle_unreachable_code`: This routine is effectively empty.

### logged disk load from Disk.1

- `initialize_function_pointer_tables_4`: Populates a large series of global variables and multiple jump tables with function addresses and constant values.
- `initialize_jump_table`: Populates a large table of function pointers and constant values.
- `initialize_jump_table_2`: Populates a large table of function pointers and constant values.
- `initialize_jump_table_3`: Populates a large series of global memory locations with a sequence of function pointers and constant values.
- `initialize_jump_table_4`: Populates a large table of function pointers and constant values.
- `populate_function_pointer_table`: This routine initializes a large table of function pointers and data values by performing a series of memory writes.
- `initialize_jump_table_5`: Populates a large table of function pointers and constant values.
- `initialize_function_pointer_table`: Populates a large table of function pointers and constant values by performing sequential memory writes.
- `initialize_function_pointer_table_2`: Populates a large table with various function addresses and constant values.
- `initialize_function_pointer_table_3`: Populates a series of global variables and a table with specific values and addresses.
- `initialize_jump_table_6`: Populates a large table of function pointers and constant values.
- `initialize_jump_table_7`: Populates a large table of function pointers and constants by performing sequential memory writes.
- `initialize_jump_table_8`: Populates a large series of global variables and a table with function pointers and specific constant values.
- `initialize_function_pointer_table_4`: Populates a large table of function pointers and constant values by performing a series of sequential memory writes.
- `initialize_function_pointer_table_5`: Populates a large table of memory addresses and constant values by performing sequential writes to a series of global variables and memory locations.
- `initialize_jump_table_9`: Populates a large table of function pointers and data values.
- `populate_function_pointer_table_2`: This routine populates a large table with various function addresses and constants.
- `initialize_function_pointer_table_6`: Populates a large table of function pointers and constant values by performing a series of sequential memory writes.
- `initialize_jump_table_10`: Populates a large table of function pointers and data values.
- `initialize_function_pointer_table_7`: Populates a large table with a series of function pointers and constant values.
- `initialize_jump_table_11`: Populates a large table with a sequence of function pointers and associated data.
- `initialize_jump_table_12`: Populates a large table of function pointers and constants by performing sequential memory writes.
- `initialize_jump_table_13`: Populates a large table of function pointers and data values.
- `initialize_jump_table_14`: Populates a large table of function pointers and data values.
- `initialize_function_pointer_table_8`: Populates a large table of function pointers and constant values by performing sequential memory writes.
- `initialize_jump_table_15`: Populates a large table of function pointers and constant values.
- `populate_function_pointer_tables`: Initializes a large set of global function pointers with specific routine addresses and populates several lookup tables with incrementing memory addresses.
- `initialize_jump_tables`: Populates a large series of global variables and memory locations with function pointers and constant values.
- `initialize_jump_table_16`: Populates a large table of function pointers and constants.
- `initialize_function_pointer_table_9`: Populates a large table with various function addresses and constant values.
- `initialize_function_pointer_table_10`: Populates a large table with various function addresses and constant values.
- `initialize_function_pointer_table_11`: Populates a large table of function pointers and constant values with specific memory addresses and data.
- `initialize_function_pointer_table_12`: Populates a large table of function pointers and constants by performing sequential memory writes.
- `initialize_jump_table_17`: Populates a large table of function pointers and constant values.
- `initialize_jump_table_18`: Populates a large series of global function pointers and data entries with specific addresses and values.
- `initialize_jump_table_19`: Populates a large table of function pointers and data values.
- `initialize_jump_table_20`: Populates a large table of function pointers and constant values.
- `initialize_jump_table_21`: Populates a large table of function pointers and data values.
- `initialize_function_pointer_table_13`: Populates a large table of function pointers and constant values by performing sequential memory writes.
- `initialize_jump_table_22`: Populates a large table of function pointers and constants in memory.
- `initialize_jump_table_23`: Populates a large table of function pointers and data values.
- `initialize_function_pointer_table_14`: Populates a large table with various routine addresses and constant values.
- `populate_function_pointer_table_3`: This routine initializes a large table of function pointers and constant values.
- `initialize_jump_table_24`: Populates a large series of global memory locations with various addresses and constants.
- `initialize_jump_table_25`: Populates a series of global variables and a large table with function pointers and specific constant values.
- `initialize_jump_table_26`: Populates a large table of function pointers and constant values by performing sequential memory writes.
- `populate_jump_table`: Initializes a large table of function pointers and constant values.
- `initialize_function_pointer_table_15`: Populates a large table with various routine addresses and constant values.
- `populate_function_pointer_table_4`: This routine initializes a large array of function pointers and constant values by sequentially writing specific addresses and small integers into a table.
- `initialize_jump_table_27`: Populates a large table of function pointers and constant values.
- `initialize_function_pointer_table_16`: Populates a large table of function pointers and constant values by performing sequential writes to memory.
- `initialize_function_pointer_table_17`: Populates a large table of function pointers and constant values.
- `initialize_function_pointer_tables`: Sets a series of global variables to specific values and populates a large table with function pointers and associated data.
- `initialize_function_pointer_tables_2`: Populates a series of global variables and tables with specific memory addresses and constant values.
- `initialize_function_pointer_tables_3`: Populates a large series of global variables and multiple jump tables with function addresses and constant values.
- `initialize_dispatch_tables`: Sets up multiple function pointer tables and global state variables through a series of conditional assignments and iterative updates.
- `initialize_memory_and_tables`: Sets a range of memory to all ones and then repeatedly calls another routine until a specific return value is met.
- `initialize_data_tables`: Initializes several memory blocks by setting specific offsets to zero, one, or a constant value, and performs a nested loop to populate a table with incrementing byte values.
- `configure_audio_and_raster`: Sets several global variables, performs a loop to update a value and call another routine, and eventually writes values to audio hardware registers.
- `store_routine_pointer`: Stores a routine address and two integer values into global variables.
- `setup_audio_parameters`: Stores a routine address and two integer values into global variables before calling another routine twice.
- `wait_for_joystick_input`: Calls another routine and then enters a loop that continues until a specific bit in a register is set.
- `populate_and_transform_table`: Fills a table with incrementing values and then performs a bitwise transformation on the data using a scaling factor.
- `execute_repeated_tasks`: Executes two specific routines in succession, repeating this sequence sixteen times while incrementing a global counter.
- `decrement_and_transform_table`: Fills a table with decrementing values and then performs a bitwise transformation on the data using a scaling factor.
- `wait_for_raster`: Repeatedly calls a routine that waits for the hardware beam to reach a specific raster line until a condition is met.
- `initialize_audio_registers`: Writes a series of constant values to a global variable, which are used to configure audio hardware registers.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `process_data_table`: Iterates through a table of values, performing arithmetic and bitwise operations to update entries in a global memory block.
- `conditionally_call_routine`: Checks if a global variable is non-negative and, if so, calls another routine.
- `calculate_obfuscated_value`: Performs a series of arithmetic operations including modulo and division on a value to determine a control flow path.
- `copy_memory_segments`: Performs a series of memory block transfers between source and destination locations using calculated offsets.
- `copy_memory_block_data`: This routine performs a memory transfer by calling a sub-routine that executes a nested loop to copy data between two memory locations using calculated offsets.
- `copy_memory_blocks`: Performs a nested loop to copy data between two memory locations using calculated offsets.
- `clear_data_register_byte`: Performs a logical OR operation on the lowest byte of a data register using a zero immediate value.
- `apply_iterative_scaling`: Iterates through a table to transform bitwise data through a series of mathematical operations and bit shifts.
- `copy_structured_data_blocks`: Copies blocks of structured data from one memory area to another using fixed offsets and a loop.
- `execute_obfuscated_control_flow`: Performs a series of conditional and recursive calls to various subroutines based on the values of multiple global variables.
- `invert_memory_data`: Iterates through a table and performs a bitwise NOT on byte, word, and long values at each step.
- `map_data_to_graphics`: Reads values from a table, applies a transformation, and copies structured data to a destination based on those values.
- `copy_structured_data`: Sets specific bits in several memory locations and then copies blocks of structured data from one table to another.
- `copy_indexed_structures`: Iterates through a table to copy structured data from a source to a destination using indices derived from a global variable.
- `initialize_large_data_tables`: Clears a memory region and then performs a complex nested loop to populate multiple tables with structured data.
- `calculate_address_offsets`: Initializes several global variables with base addresses and then adds return values from another routine to those globals.
- `initialize_address_tables_2`: Sets several global variables to base addresses and then adds return values from another routine to those globals.
- `relocate_hunk_data`: Copies data from one memory region to another, performing calculations based on values that resemble AmigaDOS hunk type codes.
- `populate_offset_tables`: Sets several global variables and then repeatedly calls another routine, adding the returned values to a global variable while incrementing another global variable by a fixed amount.
- `execute_recursive_calls_4`: Performs a series of recursive calls to itself based on the value of a global variable.
- `execute_recursive_calls_2`: Performs a series of recursive calls to itself.
- `call_sprite_offset_calculator`: This routine serves as a wrapper that calls a procedure to calculate sprite buffer offsets and coordinates using table indexing and arithmetic.
- `check_global_and_recurse`: Checks if a global variable is greater than negative one and calls itself recursively if the condition is met.
- `calculate_sprite_buffer_offsets`: Calculates memory offsets and coordinates for sprite data by indexing into a table and performing arithmetic on input values.
- `clear_global_word`: Stores a zero into a specific offset of a global variable.
- `update_blt_parameters`: Checks a global variable against a range and, if valid, copies a sequence of eight values from a table into a global variable and increments the table pointer.
- `initialize_dynamic_tables`: Calculates and stores several values into global variables and then repeatedly calls another routine while accumulating the returned values into two global variables.
- `initialize_conditional_tables`: Checks if a global variable is non-zero before calculating and storing values into several global variables and then repeatedly calls another routine while accumulating the returned values into two global variables.
- `toggle_global_state`: Sets a global variable to one value and another to zero, then sets the first to a different value and the second to one.
- `toggle_global_state_2`: Updates a global variable and a secondary variable with two different sets of values in sequence.
- `toggle_global_state_3`: Changes the values of a global variable and a secondary variable twice using different constants.
- `toggle_global_state_4`: Assigns specific values to a global variable and another variable, then repeats the process with different values.
- `toggle_global_state_5`: Updates a global variable and a secondary state flag with two different sets of values before calling another routine twice.
- `clear_instruction`: This routine contains no instructions.
- `clear_instruction_2`: This is an empty routine.
- `clear_instruction_3`: The routine performs no operations.
- `toggle_global_state_6`: Sets a global variable to a specific value and resets another global variable to zero before calling another routine, then repeats the process with different values.
- `clear_instruction_4`: This routine contains no code.
- `toggle_global_state_7`: Sets a global variable to a specific value and a secondary flag to zero before calling another routine, then repeats the process using different values for both.
- `toggle_global_state_flag`: Sets a global variable to a specific value and updates a state flag before calling another routine, then repeats the process with different values.
- `set_flag_and_return`: Sets a global variable to a specific value and updates a secondary flag before returning.
- `toggle_global_state_8`: Sets a global variable to a specific value and a flag to zero before calling another routine, then updates the same global variable and flag to different values before calling that routine again.
- `update_global_state`: Updates a global variable and another global variable with specific values.
- `modify_global_values`: Sets a global variable to one value and another global variable to a different value.
- `set_global_flags`: Performs bitwise operations or assignments to global variables.
- `store_global_value`: Moves a value into a global variable.
- `toggle_state_and_call`: Sets a global variable and a state flag to one set of values, calls another routine, then repeats the process with a different set of values.
- `iterate_attribute_table`: Iterates through a table of data structures, comparing a value against a global variable and calling another routine if a match is found.
- `calculate_sprite_attribute_offsets`: Calculates and populates a series of memory offsets and pointers for sprite or object data based on coordinate-driven arithmetic.
- `initialize_and_shuffle_table`: Initializes a short integer array with sequential values and performs a shuffle operation on the elements.
- `merge_memory_blocks`: Performs a nested loop to bitwise OR values from a source table into a destination buffer using specific offsets.
- `process_offset_table`: Processes a list of four values from a table, performing arithmetic on global variables and updating a status flag based on the value's sign.
- `calculate_dynamic_offset_tables`: Calculates several base addresses and offsets based on relative memory distances and populates multiple global tables.
- `populate_function_table`: Writes a series of function addresses and constant values into a table located at a global pointer and then increments that pointer.
- `calculate_nonlinear_values`: Computes two separate values using quadratic-like arithmetic based on global variables and stores them in different global locations.
- `fill_memory_with_pattern`: Initializes a memory range by filling it with a specific byte value, followed by a secondary loop that writes a repeating byte pattern and performs arithmetic transformations on the data.
- `initialize_memory_patterns`: Fills a memory area with a specific byte value, performs arithmetic on a value from a global variable to determine a count, and then fills a subsequent memory range with different byte values.
- `execute_recursive_calls`: This routine performs a series of recursive calls to itself.
- `call_memory_fill_routine`: This routine serves as a wrapper that calls a memory filling function.
- `call_memory_fill_routine_2`: This routine serves as a wrapper that calls a function to fill a memory range with a specific byte value.
- `call_memory_fill_routine_3`: This routine acts as a wrapper that calls a function to fill a memory range with a specific byte value.
- `call_memory_fill_routine_4`: This routine acts as a wrapper that calls a function to fill a memory range with a specific byte value.
- `fill_memory_with_values`: Writes a byte to a memory location, calculates a count based on a word value, fills a range with a specific byte, and increments a global variable if it does not match a specific character.
- `execute_recursive_calls_3`: Performs a series of self-referential calls to the same routine until the stack or execution flow terminates.
- `check_register_threshold`: Compares the value in a data register against a constant and returns if the condition is met.
- `copy_data_blocks`: Copies multiple blocks of data from a source memory location to a destination memory location using fixed offsets.
- `initialize_address_tables`: Fills two separate tables with 32-bit values derived from incrementing a base value by specific step sizes.
- `process_string_list`: Iterates through a list of pointers, calling another routine for each non-null entry, and then executes a series of subsequent routine calls.
- `process_data_and_increment`: Writes a byte to a memory location, calculates a count from a register, fills a memory range with a specific byte, and increments a global variable before returning a value plus one.
- `transfer_large_data_set`: Copies a large number of bytes from a source buffer to a destination buffer using a series of hardcoded offsets.
- `apply_bitwise_transform`: Iterates through a loop to perform a bitwise NOT on source data and OR the result with values from another memory location into a destination.
- `iterate_and_update_table`: Iterates through a table of twelve entries, reading values from memory and updating a global variable through a nested loop.
- `check_conditional_tables`: Scans two separate tables for specific constant values and calls another routine when a match is found.
- `perform_loop_calls`: Executes a routine four times using a loop counter.
- `clear_and_check_table`: Clears a memory range by writing zeros and then iterates through a table, calling another routine if a value is non-negative.
- `copy_input_string`: Executes two consecutive loops that read characters from a routine and write them into a buffer if they are not equal to negative one.
- `map_input_character`: Translates an input byte into a different character set or value based on several conditional checks and arithmetic offsets.
- `check_bits_and_call`: Iterates through a bitmask in a register and calls another routine if a specific bit is set.
- `process_interrupt_flag`: Checks a bit in a global variable, performs a bitwise rotation and inversion if set, and then resets the global variable to a constant value.
- `assign_constant_value`: Stores a constant byte into a global variable.
- `write_interrupt_request`: Stores a constant value into a global variable and interacts with the interrupt request hardware register.
- `update_function_table`: Checks bits in a global variable and, if a specific bit is set, copies a block of data from one memory location to another.
- `dispatch_function_pointers`: Executes a function from a table and manages the assignment and execution of function pointers stored in global variables based on specific value comparisons.
- `update_car_physics_and_bounds_2`: Calculates directional velocity and position updates based on joystick input and applies smoothing to the movement.
- `store_word_to_memory`: Stores a 16-bit value from a data register into a global variable.
- `update_function_pointer`: Stores a 16-bit value from a data register into a location used as a function pointer.
- `store_word_from_register`: stores a word from a register into a global variable.
- `store_word_to_global`: Stores a 16-bit value from a data register into a specific global variable.
- `set_global_constant`: stores the constant value nine hundred ninety-nine into a global variable.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `initialize_game_state_tables`: Copies data from a source table into a global memory region and populates several structured tables with zeroed or constant values.
- `execute_looping_calls`: Repeatedly calls another routine within a loop that increments a counter and resets it when it reaches a specific value.
- `perform_complex_math`: calculates two new values using multiplication, subtraction, and bit shifting, then stores them in a table via pointers.
- `set_value_and_call`: Stores a byte into a global variable and calls another routine twice.
- `merge_memory_blocks_2`: Performs a bitwise OR operation on five different memory sources and stores the result into a destination table using a loop.
- `set_status_byte`: Writes a specific byte value to a global variable and calls another routine twice.
- `combine_and_copy_data`: Merges five memory blocks using bitwise OR into a destination, then performs a secondary block copy from one memory region to another.
- `bitwise_or_merge`: Iterates through five memory segments, combining their values via bitwise OR into a single destination table.
- `update_status_register`: Sets a global byte to a specific value and executes another routine twice.
- `assign_constant_value_2`: Writes a constant byte to a global variable and calls another routine twice.
- `initialize_memory_with_constants`: Sets two global variables to specific values and then fills a memory range with a constant value if a third global variable equals three.
- `copy_string_table`: Searches for a specific four-character signature in memory and then copies a series of null-terminated strings into a table.
- `copy_data_after_signature`: Locates a specific four-character signature in memory and copies a large block of data from that point to a new destination.
- `execute_multiple_initializations`: Sequentially sets a global variable to several different values and calls another routine after each assignment.
- `set_control_byte`: Assigns a specific byte to a global variable and calls another routine twice.
- `set_status_byte_2`: Stores a specific byte value into a global variable and calls several initialization routines.
- `initialize_pointer_table`: Stores a routine address into a global variable, performs a memory read from a table offset, and sets a global pointer to a table.
- `merge_memory_tables`: Performs a bitwise OR operation across multiple memory tables and stores the result in a destination table using a loop.
- `set_global_constant_and_recurse`: Sets a global variable to a specific constant value and then calls itself twice.
- `swap_global_pointer`: Saves a global pointer, replaces it with another global pointer, and restores the original value after calling another routine.
- `swap_and_store_function_pointers`: Swaps the values of two global function pointers and stores a byte value into a third global variable before calling another routine.
- `search_global_array`: Iterates through a sequence of bytes in memory until it finds a specific byte value or an end-of-sequence marker.
- `decompress_bitstream_data`: Searches a table for a specific magic marker and then performs a bit-shifting decompression loop on the subsequent data.
- `execute_bit_shift_loop`: performs a loop that repeatedly doubles a value and decrements a pointer based on a counter.
- `copy_memory_block`: copies thirty-two bytes from a global variable to itself.
- `copy_offset_memory`: copies thirty-two bytes from a calculated offset in a global variable to the start of that same global variable.
- `update_game_state_and_input`: Updates a global state machine and processes directional input by incrementing or decrementing a character value within a specific range.
- `process_table_values`: iterates through a table of words, performing conditional subtractions based on the difference between adjacent values.
- `format_numeric_string_2`: Converts a numeric value into a string of ASCII characters.
- `update_state_parameters`: Checks a global variable and executes a loop of another routine if a condition is met.
- `process_bitfield_table`: Checks a global value to determine if a pointer should be calculated from a table.
- `calculate_scaled_value`: Calculates a value using a combination of bit shifts and multiplication, then stores the result in a global variable.
- `update_physics_state_parameters`: Updates multiple global variables using fixed-point arithmetic, bitwise masking, and conditional scaling based on a state value.
- `process_state_machine`: Updates a table of values based on a state variable and performs a search loop to match byte values.
- `scale_integer_values`: Iterates through a loop to perform fixed-point arithmetic on values retrieved from a routine and stores them in a table.
- `map_input_range`: returns a transformed value based on three different numeric ranges provided in a register.
- `map_input_range_simple`: returns a value calculated by either doubling the input or subtracting a constant depending on its magnitude.
- `parse_string_data`: Clears a block of memory using a loop and then parses a string to count elements separated by specific characters.
- `parse_formatted_string`: Iterates through a string to count elements and performs memory clearing operations on several locations based on a loop.
- `expand_formatted_string`: Iterates through a source string and copies characters to a destination, replacing specific control characters with strings from a table.
- `format_numeric_string`: clears a memory area, processes a list of values to increment bytes, and converts a resulting value into a six-character ASCII string.
- `copy_string_data`: Copies a sequence of bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `iterate_table_elements`: Iterates through a table of structures, comparing specific word values against constants and incrementing a global counter if certain conditions are met.
- `update_accumulated_values`: Performs conditional arithmetic on several global variables and uses recursion to update a value until a threshold is met.
- `extract_bitfield_data`: Loops until a specific bit pattern is found in a value, then extracts and stores masked bitfields into global variables.
- `increment_global_recursively`: Adds a value to a global variable and calls itself recursively until a specific threshold is reached.
- `update_entity_physics`: Updates the position and velocity of an entity by applying scaled values from a lookup table and performing distance-based collision checks.
- `check_collision_and_trigger_event`: Iterates through a table of coordinates to check if the distance between a moving object and a target point is within a specific threshold.
- `update_entity_coordinates_and_collision`: Iterates through a table of entities to update their positions and perform distance-based collision detection.
- `read_joystick_input`: Reads the state of the first joystick port and the fire button status from the hardware registers.
- `update_coordinates`: Calculates a scaled value from a table entry and adds the result to two consecutive memory locations.
- `calculate_indexed_byte`: Calculates a memory address using bitwise operations and arithmetic on two values from a structure, then returns a byte from that location modified by a conditional bit shift.
- `update_obfuscated_values`: Performs a series of arithmetic operations and comparisons to update several global variables using large constant values.

### logged disk load from Disk.1

- `initialize_object_property_tables`: Initializes a large block of memory by setting various pointers to a base address and populating a series of data structures with zero, one, or maximum byte values.
- `reset_global_variables`: Sets several global variables and memory locations to zero or a constant value.
- `store_word_to_global_2`: The routine stores a word from a register into a global variable.
- `initialize_hardware_registers`: Clears several global variables and specific memory locations, and resets various hardware-related values to zero or a constant.
- `reset_audio_dma_state`: The routine clears a global variable and four audio channel volume and period registers, then zeroes several RAM locations used for audio DMA state.
- `update_timer_callback`: Decrements a counter and calls a function pointer from a table, while also updating several global values based on comparisons.
- `decrement_timer_and_dispatch`: Checks a flag in a structure, decrements a counter, calls a function pointer, and copies summed fields to an output area.
- `update_structure_counters`: Decrements a value in a structure and updates a memory location with a bitwise ORed value when the counter reaches zero.
- `update_audio_counter`: Decrements a counter in a structure and updates a memory location with a bitwise ORed value when the counter reaches zero.
- `set_flag_and_accumulate_short`: The routine writes a constant word to an offset in a structure pointed to by an address register and then adds a short value from another structure pointed to by a different address register to a short field in the first structure, storing the sum back in that field.
- `process_audio_parameter`: Manages a counter and bitwise operations on structure fields to return updated audio-related values.
- `decrement_counters_and_flip_sign`: The routine decrements two short counters stored in a structure pointed to by an argument register and accumulates values from other fields of the same structure into a running total.
- `disable_dma_and_interrupts`: The routine writes to DMACON to disable DMA channels and writes to INTENA to disable interrupts, then writes to INTREQ and INTREQR to clear interrupt requests.
- `tiny_ori_3`: Small routine of 1 instruction that merges 1 into d1.
- `decrement_and_reset_counters`: Decrements a global counter and, if it reaches zero, resets several global variables and memory locations to specific constant values.
- `calculate_scaled_value_2`: Performs multi-precision arithmetic and bitwise operations to update a table of values and store results in a structure.
- `update_multi_precision_accumulator`: The routine increments a global variable and performs multi-precision addition using several global variables to update a running total and its carry state.
- `initialize_data_structures`: Initializes a large block of memory by setting various offsets to zero or one, and subsequently fills a specific range with a repeating pattern of bytes.
- `initialize_data_structures_2`: Initializes a large block of memory by setting multiple consecutive bytes to zero, one, or a specific repeating pattern.
- `initialize_system_structures`: Initializes a large block of memory with specific patterns and sets several pointers and global variables based on values read from a table.
- `parse_command_stream_2`: Iterates through a data stream to decode command bytes, updating internal state, pointers, and hardware-related parameters.
- `parse_command_stream`: Iterates through a data stream to decode command bytes, updating internal state, pointers, and hardware-related parameters.
- `parse_command_data`: Reads a byte from a source and performs different operations on a destination structure based on the value of that byte.
- `calculate_data_offset`: Evaluates a byte in a structure to perform conditional decrements and pointer increments, returning a calculated value based on a global table.
- `process_sprite_indexing`: Manages counters and pointers within a structure to calculate offsets and update data based on values retrieved from a global table.
- `update_sprite_data`: Calculates an offset based on a byte, checks a condition, and updates several bytes and a pointer in a structure.
- `clear_sprite_flags_and_advance_pointer`: The routine clears a bit in a byte at an offset from a pointer, writes two bytes derived from a byte register and a byte pointed to by a pointer, advances that pointer by one and stores it back, then returns twice the value of a signed byte at another offset from the pointer.
- `update_timer_counter`: The routine updates a 32-bit timer value stored in a global variable using a 16-bit delta from a pointer, then decrements a signed byte counter and clears its sign bit.
- `update_sprite_hardware_parameters`: Decrements counters in a structure and, upon reaching zero, calculates new sprite coordinates and dimensions using fixed-point multiplication.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_and_dma`: Configures audio hardware registers and enables DMA channels.
- `call_subroutines`: This routine calls two other routines sequentially.
- `update_state_and_call_subroutine`: Updates a global variable with a bitwise shifted value, resets another global variable to zero, executes a subroutine, and increments a third global variable by a fixed amount.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_state`: Increments a global variable by a fixed value.
- `toggle_global_state_flag_2`: This routine is empty.
- `toggle_global_state_flag_3`: Sets a global byte to a specific value and resets a state flag before calling itself recursively, then restores the original values.
- `toggle_global_flag`: Sets a global byte to a specific value and clears a state flag, then repeats the process with different values.
- `toggle_global_state_flag_4`: This routine is empty.
- `toggle_global_flag_2`: Sets a global byte to a specific value and toggles a global state flag before calling itself through an indirect jump.
- `toggle_global_state_flag_5`: Sets a global byte to a specific value and a state flag to zero, then repeats the process with different values for both.
- `clear_global_byte`: Clears a byte in a global variable.
- `increment_global_word`: Increments a word at a global variable address.
- `set_global_byte`: Stores a byte into a global variable.
- `add_to_global_word`: Adds a value to a word stored in a global variable.
- `modify_global_word`: Performs an addition or move operation on a global word.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `write_global_byte`: Writes a byte to a specific global variable location.
- `update_global_variable`: Performs bitwise AND, comparison, and subtraction on a global variable.
- `adjust_global_variable`: Increments a global variable and performs a comparison.
- `apply_bitwise_or`: Performs a bitwise OR operation on a global variable.
- `toggle_global_state_9`: Updates a global variable with two different values and calls another routine between the updates.
- `iterate_state_tables`: Iterates through a table of structures, comparing a flag and updating several values from the table into global variables.
- `recalculate_dynamic_jump_tables`: Calculates and updates several base addresses and offsets for a series of function pointer tables and jump tables.
- `modify_global_state`: Performs a series of bitwise OR operations and additions on various global variables and counters.
- `populate_data_table`: Writes a series of bytes and words into a table starting at a global pointer and increments that pointer by eight.
- `write_interrupt_request_2`: Stores a byte into a global variable and modifies the interrupt request register.
- `update_conditional_buffer`: Checks bits in a global variable to conditionally execute a subroutine, update a counter, and copy a block of data from a table to a destination.
- `dispatch_function_pointer`: Checks several global variables and a jump table to conditionally execute a function pointer and update a stored value.
- `update_mouse_position`: Calculates relative mouse movement by comparing current hardware input to previous values and updates internal coordinate variables.
- `store_word_register`: Stores a word from a register into a global variable.
- `store_word_to_memory_2`: Stores a word from a register into a different global variable.
- `set_constant_value`: Stores the constant value nine hundred ninety-nine into a global variable.
- `clamp_table_values`: Iterates through a table of shorts, calculating differences and updating the table entries with clamped values.
- `convert_number_to_string`: Converts a numeric value into a decimal string representation within a buffer and performs a memory shift operation.
- `update_game_state_counters`: Manages a complex state machine by checking various global flags and counters to increment or reset specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `nop_placeholder`: The routine is empty and performs no operations.
- `do_nothing`: This routine is empty and performs no operations.
- `return_empty_routine`: This routine performs no operations and returns immediately to its caller.
- `perform_no_operation`: This routine is empty and performs no actions before returning.
