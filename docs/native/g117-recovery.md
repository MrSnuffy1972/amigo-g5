# Game 117 (SuperCars v1.1 1527): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00117). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1293841 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | FAIL, 193 of 240 named (0 author symbols), 193 of 240 described (3 by rule, 190 by the local model) |
| Executed addresses (reference run) in recovered, described code | FAIL, 4451 of 4795 (92.83%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7591 of 7591 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 234923 |
| code | 34288 |
| data | 1024618 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SuperCars.Slave | 1 | 325 | 22 |
| logged disk load from Disk.1 | 1 | 5552 | 136 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 679 | 22 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 95 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 878 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 62 | 34 |

## Cases handled

- SuperCars_v1.1_1527/SuperCars/Disk.1: raw disk image (337920 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- SuperCars_v1.1_1527/SuperCars/Disk.2: raw disk image (915456 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 7 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate+address, 1 immediate
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
- `sub_010E`: undescribed.
- `wait_for_bit_transitions`: Repeatedly polls a global variable for specific bit states and waits for a single bit to toggle between zero and non-zero states for a fixed number of iterations.
- `check_status_conditions`: Checks a global variable and a bit in a system memory location to determine if specific bitwise conditions are met, returning a bit from the global variable.
- `xor_global_variable`: Performs a bitwise XOR operation on a global variable using a constant value.
- `update_callback_table`: If a specific value is in a register and a global variable is non-zero, it stores a constant into a table.
- `modify_callback_table`: If a bit is set in a register and the register contains a specific constant, it updates several entries in a table with function pointers and copies a block of data from a source to a destination.
- `wait_for_flag_clear`: The routine enters a loop while a specific bit in a global variable remains set, then updates that global variable with a value from a register.
- `spin_in_infinite_loop`: The routine enters an infinite loop and never returns to its caller.
- `sub_04F0`: undescribed.
- `sub_0506`: undescribed.
- `sub_0528`: undescribed.
- `update_function_pointers`: Checks specific memory locations for constant values and, if matched, stores a word and a function pointer into those locations.
- `store_constant_word`: Stores a two-byte constant value into a memory location offset from a pointer held in a register.
- `xor_memory_block`: Iterates through a block of memory, performing a bitwise XOR between a register value and each byte in the block.
- `xor_memory_block_2`: Performs a bitwise XOR operation between a register value and every byte within a specified memory block.
- `execute_conditional_callback`: Calls a function through a pointer stored in a table and, if the result is non-zero, executes another function through a different pointer and calls itself recursively.
- `copy_memory_and_call_indirect`: Copies a block of data from one location to another and executes a function through a pointer if a global variable is zero.
- `execute_indirect_calls`: Loads a pointer from a global variable, calls a function at an offset within that pointer, and if the result is non-zero, calls another function at a different offset and sets a global variable to a constant.
- `load_function_table`: Copies several values from a source table into a series of global variables and executes an indirect function call if a global variable is zero.
- `sub_072A`: undescribed.
- `handle_unreachable_code`: This routine is effectively empty.

### logged disk load from Disk.1

- `initialize_function_pointer_tables_4`: Populates a large series of global variables and multiple jump tables with function addresses and constant values.
- `sub_0EE6`: undescribed.
- `initialize_dispatch_tables`: Sets up multiple function pointer tables and global state variables through a series of conditional assignments and iterative updates.
- `initialize_memory_and_tables`: Sets a range of memory to all ones and then repeatedly calls another routine until a specific return value is met.
- `initialize_data_tables`: Initializes several memory blocks by setting specific offsets to zero, one, or a constant value, and performs a nested loop to populate a table with incrementing byte values.
- `configure_audio_and_raster`: Sets several global variables, performs a loop to update a value and call another routine, and eventually writes values to audio hardware registers.
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
- `apply_iterative_scaling`: Iterates through a table to transform bitwise data through a series of mathematical operations and bit shifts.
- `copy_structured_data_blocks`: Copies blocks of structured data from one memory area to another using fixed offsets and a loop.
- `execute_obfuscated_control_flow`: Performs a series of conditional and recursive calls to various subroutines based on the values of multiple global variables.
- `invert_memory_data`: Iterates through a table and performs a bitwise NOT on byte, word, and long values at each step.
- `map_data_to_graphics`: Reads values from a table, applies a transformation, and copies structured data to a destination based on those values.
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
- `write_interrupt_request`: Stores a constant value into a global variable and interacts with the interrupt request hardware register.
- `update_function_table`: Checks bits in a global variable and, if a specific bit is set, copies a block of data from one memory location to another.
- `dispatch_function_pointers`: Executes a function from a table and manages the assignment and execution of function pointers stored in global variables based on specific value comparisons.
- `update_car_physics_and_bounds_2`: Calculates directional velocity and position updates based on joystick input and applies smoothing to the movement.
- `store_word_from_register`: stores a word from a register into a global variable.
- `store_word_to_global`: Stores a 16-bit value from a data register into a specific global variable.
- `set_global_constant`: stores the constant value nine hundred ninety-nine into a global variable.
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
- `sub_8FE4`: undescribed.
- `sub_8FF4`: undescribed.
- `sub_900C`: undescribed.

### logged disk load from Disk.1

- `sub_000A`: undescribed.
- `reset_audio_dma_state`: The routine clears a global variable and four audio channel volume and period registers, then zeroes several RAM locations used for audio DMA state.
- `update_timer_callback`: Decrements a counter and calls a function pointer from a table, while also updating several global values based on comparisons.
- `decrement_timer_and_dispatch`: Checks a flag in a structure, decrements a counter, calls a function pointer, and copies summed fields to an output area.
- `update_structure_counters`: Decrements a value in a structure and updates a memory location with a bitwise ORed value when the counter reaches zero.
- `update_audio_counter`: Decrements a counter in a structure and updates a memory location with a bitwise ORed value when the counter reaches zero.
- `set_flag_and_accumulate_short`: The routine writes a constant word to an offset in a structure pointed to by an address register and then adds a short value from another structure pointed to by a different address register to a short field in the first structure, storing the sum back in that field.
- `process_audio_parameter`: Manages a counter and bitwise operations on structure fields to return updated audio-related values.
- `decrement_counters_and_flip_sign`: The routine decrements two short counters stored in a structure pointed to by an argument register and accumulates values from other fields of the same structure into a running total.
- `disable_dma_and_interrupts`: The routine writes to DMACON to disable DMA channels and writes to INTENA to disable interrupts, then writes to INTREQ and INTREQR to clear interrupt requests.
- `decrement_and_reset_counters`: Decrements a global counter and, if it reaches zero, resets several global variables and memory locations to specific constant values.
- `update_multi_precision_accumulator`: The routine increments a global variable and performs multi-precision addition using several global variables to update a running total and its carry state.
- `sub_1C94`: undescribed.
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
- `sub_000C`: undescribed.
- `sub_00AC`: undescribed.
- `initialize_audio_and_dma`: Configures audio hardware registers and enables DMA channels.
- `call_subroutines`: This routine calls two other routines sequentially.
- `update_state_and_call_subroutine`: Updates a global variable with a bitwise shifted value, resets another global variable to zero, executes a subroutine, and increments a third global variable by a fixed amount.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_state`: Increments a global variable by a fixed value.
- `sub_001A`: undescribed.
- `sub_00F8`: undescribed.
- `sub_0104`: undescribed.
- `sub_0342`: undescribed.
- `iterate_state_tables`: Iterates through a table of structures, comparing a flag and updating several values from the table into global variables.
- `recalculate_dynamic_jump_tables`: Calculates and updates several base addresses and offsets for a series of function pointer tables and jump tables.
- `modify_global_state`: Performs a series of bitwise OR operations and additions on various global variables and counters.
- `populate_data_table`: Writes a series of bytes and words into a table starting at a global pointer and increments that pointer by eight.
- `sub_1124`: undescribed.
- `sub_1B26`: undescribed.
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
- `sub_000A`: undescribed.
- `sub_000E`: undescribed.
- `sub_0012`: undescribed.
- `sub_0016`: undescribed.
- `sub_001A`: undescribed.
- `sub_001E`: undescribed.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `nop_placeholder`: The routine is empty and performs no operations.
- `sub_002A`: undescribed.
- `do_nothing`: This routine is empty and performs no operations.
- `return_empty_routine`: This routine performs no operations and returns immediately to its caller.
- `sub_0036`: undescribed.
- `sub_003A`: undescribed.
- `sub_003E`: undescribed.
- `sub_0042`: undescribed.
- `sub_0046`: undescribed.
- `sub_004A`: undescribed.
- `sub_004E`: undescribed.
- `sub_0052`: undescribed.
- `sub_0056`: undescribed.
- `sub_005A`: undescribed.
- `sub_005E`: undescribed.
- `perform_no_operation`: This routine is empty and performs no actions before returning.
- `sub_0066`: undescribed.
- `sub_006A`: undescribed.
- `sub_006E`: undescribed.
- `sub_0072`: undescribed.
- `sub_0076`: undescribed.
- `sub_007A`: undescribed.
- `sub_007E`: undescribed.
- `sub_0082`: undescribed.
- `sub_0086`: undescribed.
- `sub_008A`: undescribed.
