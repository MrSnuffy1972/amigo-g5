# Game 2291 (StarWars v4.0 0403): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02291). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (719295 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 572 of 572 named (0 author symbols), 572 of 572 described (48 by rule, 521 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5652 of 5653 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 32416 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18414 of 18414 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 207256 |
| code | 59988 |
| data | 452033 |
| padding | 18 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 152 | 8 |
| starwars | 3 | 17219 | 411 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 909 | 54 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 55 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 68 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| OSEmu.400#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- StarWars_v4.0_0403/StarWars_v4.0_0403.hdf: hard-disk image (AmigaDOS OFS, 800 blocks): its filesystem parses and all 27 files in it are byte-identical to the extracted tree StarWars_v4.0_0403_hdf, which is recovered file by file; no block outside the filesystem holds data
- StarWars_v4.0_0403/StarWars_v4.0_0403_hdf/StarWars/data/OSEmu.400: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (18909 -> 32416 bytes)
- 14 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 436 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- StarWars_v4.0_0403/StarWars_v4.0_0403_hdf/StarWars/data/OSEmu.400#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 32416 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_command_sequence` (starwars): Performs a series of function calls with specific arguments and waits for a memory location to reach zero. Kind: disk/track check. Pass path: the loop completes and execution continues to the final instructions after the do-while loop.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_library_request`: Processes a library request and uses a value from a global variable to index into a jump table for indirect calls.
- `execute_indirect_calls_2`: Calls two functions using pointers retrieved from a table and the stack.
- `update_global_state`: Checks if a register contains a specific character and, if so, performs bitwise negation and conditional assignments to a global variable.
- `process_library_request`: Uses a library to perform operations and manages a stack-based structure involving conditional checks and function calls.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `search_and_copy_table`: Searches a table for a matching value and, if found, copies a sequence of words to a new location.
- `traverse_linked_data`: Iterates through a data structure by reading values, performing bitwise masking, and updating a pointer based on values retrieved from a table.

### starwars

- `initialize_memory_block`: Clears a block of memory and performs conditional calls to functions stored in a global variable.
- `call_extract_masked_value`: This routine acts as a wrapper that calls a function to extract a masked value from a memory block.
- `extract_masked_value`: Checks specific bytes within a memory block and, if certain conditions are met, stores a masked portion of a word into a destination pointer.
- `update_audio_and_input_state`: Updates audio hardware registers for four channels and processes input data from joystick or mouse ports to calculate movement deltas.
- `check_and_clear_short`: Reads a short value from a memory offset, clears it if it is non-zero, and returns the original value.
- `dispatch_callback_with_buffer`: Calls a function through a pointer and, if successful, populates a local buffer using another subroutine.
- `execute_command_sequence`: Performs a series of function calls with specific arguments and waits for a memory location to reach zero.
- `copy_word_to_memory`: Copies a word from one memory location to another using parameters passed through registers.
- `call_and_store_result`: Calls a function with a specific constant and stores the return value in a global variable.
- `check_and_call_function`: Checks if a global variable is non-zero, calls a function with that value, and if the returned value is non-zero, calls a second function with that new value.
- `execute_callback_sequence_2`: Calls a function through a pointer and, if the result is non-zero, passes the result and other parameters to a subsequent routine.
- `dispatch_function_call`: Calls a function via a pointer with specific parameters and, if successful, executes two subsequent function calls using the returned value.
- `execute_conditional_sequence`: Performs a series of conditional checks and function calls through pointers, updates specific memory values, and enters loops that wait for memory thresholds or zero values.
- `process_conditional_sequence`: Executes a complex sequence of conditional checks, function pointer calls, and loops that wait for specific memory values or thresholds.
- `wait_and_execute_conditional`: Waits for a memory value to reach a threshold, calls a function, and if successful, performs a sequence of operations and waits for a global bit to clear.
- `process_conditional_logic`: Performs a loop that updates memory values and calls various functions based on the results of comparisons and pointer dereferences.
- `execute_conditional_block`: Checks a value from a function call and, if non-zero, executes a sequence of function calls and waits for a specific condition in a global variable.
- `update_nested_table`: Executes a series of function calls using values derived from a nested loop structure and a provided table.
- `decrement_and_update_table`: Iterates through a loop that modifies values in a table and calls two functions using calculated offsets.
- `update_table_values`: Iterates through a loop that performs calculations using values from a table and updates memory locations via function calls.
- `setup_blitter_loop`: Calculates several values based on input parameters and global data, then enters a loop that configures blitter registers and writes to several memory locations, including bitplane and address registers.
- `update_and_return_shifted_value`: Initializes a global variable using a value from a table if it is currently zero, calls a function, updates the global variable with the result plus one, and returns the value shifted right by eight bits.
- `conditionally_call_routine`: Checks if a value at a specific offset from a pointer is non-zero and, if so, calls another routine.
- `trigger_dma_routine`: Calls another routine that interacts with the DMA controller.
- `execute_dma_routine`: A routine consisting of 22 instructions that interacts with the DMA controller register.
- `wait_for_zero_value`: Executes a routine and then enters a loop that waits until a specific word in memory becomes zero.
- `check_memory_flags`: Checks several memory locations for non-zero values and calls other routines or decrements a byte based on those values.
- `verify_memory_status`: Checks memory flags and waits for a specific memory location to reach zero.
- `configure_blitter_parameters`: Calculates dimensions and offsets from input parameters and executes a loop that updates blitter registers and multiple memory locations for several iterations.
- `run_blitter_with_fixed_height`: Configures and executes a blitter operation with a fixed height parameter.
- `run_blitter_with_calculated_offset`: Calculates a destination address using a formula involving multiple table values and executes a blitter operation.
- `run_blitter_with_swapped_parameters`: Executes a blitter operation using a destination address and width derived from a calculation involving multiple table values.
- `execute_simple_callback`: Calls a function through a pointer and executes a sequence of subroutines if the returned value is zero.
- `call_function_with_two_params`: Calls a function using two provided parameters.
- `call_function_with_constant`: Calls a function passing the constant value three as an argument.
- `call_unnamed_function`: Calls a function through a pointer stored in a global location.
- `execute_stored_function`: Executes a function via a pointer retrieved from a global variable.
- `update_state_from_function_call`: Calls a function pointer located at an offset from a base address and stores the returned value into another offset from that same base address.
- `initialize_state_structure`: Sets several bytes and pointers within a structure and then calls a function using a pointer to that structure.
- `initialize_resource_buffers`: The routine fetches multiple resource identifiers and validates them before allocating memory blocks.
- `dispatch_callback_table`: Iterates through several data structures to check for non-zero pointers and executes them with specific integer arguments.
- `dispatch_callback_sequence`: Performs a sequence of subroutine calls, modifies global variables, and executes functions using pointers retrieved from memory while interacting with hardware registers.
- `copy_bytes_backward`: Copies a sequence of bytes from a source address to a destination address, supporting backward copying if the source address is lower than the destination address.
- `copy_bytes`: Copies a sequence of bytes from a source address to a destination address using a counter.
- `copy_words_backward`: Copies a sequence of 16-bit words from a source address to a destination address, supporting backward copying if the source address is lower than the destination address.
- `copy_words`: Copies a sequence of 16-bit words from a source address to a destination address using a counter.
- `absolute_value`: Returns the absolute value of a signed 16-bit integer.
- `absolute_value_long`: Returns the absolute value of a signed 32-bit integer.
- `mul_div`: Multiplies two 16-bit signed integers and divides the 32-bit product by a 16-bit signed divisor, returning the quotient and remainder.
- `calculate_squared_sum`: Calculates the sum of the squares of two 16-bit values.
- `multiply_values`: Returns the product of two 16-bit signed integers.
- `update_global_table_values`: Performs multiple table lookups using wrapped indices and stores the resulting 16-bit values into several consecutive global memory locations.
- `swap_global_word_values`: Performs three empty calls and then swaps the values of two consecutive 16-bit words stored in a global table.
- `store_lookup_results`: Performs multiple table lookups and stores the resulting values into a destination pointer provided in the arguments.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `wrap_and_lookup`: Performs modular arithmetic and range clamping on an input before using the result to fetch a value from a table.
- `wrap_and_lookup_alt`: Calculates a wrapped index through division and modulo, then retrieves a value from a table based on that index.
- `calculate_scaled_offsets`: Calculates and stores four scaled values into a global table using division by a parameter plus a constant.
- `update_scaled_coordinates`: Calculates and stores two scaled values into a global table using division by a parameter plus a constant.
- `increment_bcd_value`: Adds four separate byte values to a four-byte buffer and applies BCD adjustment to each byte.
- `update_bcd_values`: Adds values to four consecutive bytes and applies binary-coded decimal adjustments to each.
- `increment_bcd_value_2`: Increments a byte in memory and adjusts it using binary-coded decimal logic.
- `scale_table_elements`: Iterates through two tables to scale and store values using a parameter and a constant.
- `swap_consecutive_word_values`: Swaps the values of two consecutive 16-bit words stored in a global table.
- `apply_linear_transformations`: Performs three separate loops to add scaled linear offsets to elements within a table and updates status flags based on comparisons.
- `process_multiple_data_tables`: Iterates through three separate data sets using table lookups to copy and modify values into new destination tables based on calculated offsets.
- `transform_coordinate_tables`: Updates global data tables by performing linear transformations and scaling operations.
- `transform_coordinates`: Calls another routine to apply a fixed-point scaling and translation transformation to a set of coordinate triplets.
- `transform_coordinate_table`: Applies a fixed-point scaling and translation transformation to a set of coordinate triplets.
- `copy_data_structure`: Copies data from one memory location to another in a loop and performs several word-sized assignments to a destination table.
- `update_buffer_parameters`: Sets several global variables to specific values based on the state of individual bits within the input parameter.
- `store_word_to_memory`: Stores a 16-bit value from a register into a global variable.
- `calculate_interpolated_value`: Stores two values into a global table and calculates a third value using a complex series of multiplications and shifts.
- `update_interpolated_value`: Calculates and stores a single interpolated value into a global variable using multiple multiplications and shifts.
- `perform_coordinate_interpolation`: Performs conditional linear interpolation on coordinates based on bit flags and stores the result if it falls within specified bounds.
- `update_bitmapped_display_buffer`: Calculates bitwise offsets and updates specific bits within a memory-mapped display buffer or sprite data.
- `configure_blitter_line_draw`: Sets up the Amiga Blitter hardware registers to perform a series of line or shape drawing operations.
- `store_interpolated_values`: Stores two 16-bit values from a parameter into a global variable.
- `store_parameters`: Stores two 16-bit values from a parameter into a global table and calls another routine.
- `set_buffer_bits`: Checks if a value falls within a specific range and, if so, sets a bit at a calculated offset across four memory buffers.
- `set_bit_in_four_buffers`: Sets a specific bit within a byte at four different memory locations spaced by fixed offsets.
- `update_bitmapped_buffers`: Iteratively sets bits in four memory buffers and performs bulk byte copies into those buffers based on a counter and a source table.
- `clear_object_data_buffer`: Calculates a memory address based on values stored in a global structure and performs a nested loop to zero out a large block of memory.
- `process_coordinate_divisions`: Performs multiple divisions of values from a table by a parameter and calls another routine to set bits in memory buffers.
- `calculate_projected_points`: Performs fixed-point division and scaling on values from a table to populate a destination table with calculated coordinates.
- `initialize_game_parameters`: Calls several routines to initialize data and sprite commands, then clears two specific memory locations.
- `update_countdown_and_handle_state`: Calculates a character digit for a countdown display based on a timer and increments a frame counter.
- `initialize_sequence_data`: Performs a series of memory writes and subroutine calls, including a loop that executes subroutines based on returned values to process data.
- `run_wait_loop`: Enters a loop that repeatedly calls several subroutines until a memory value reaches a threshold or a specific condition is met.
- `populate_data_table_3`: Iterates through a loop to fill a table with values derived from bit shifts and arithmetic, while making several function calls.
- `execute_function_table`: Calls multiple functions stored in a table using offsets derived from a base pointer and various parameters.
- `call_indexed_functions_2`: Calls several functions using offsets from a table and then performs a final calculation involving a pointer and a multiplier.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d3.
- `call_indexed_functions`: Calls a series of functions using values retrieved from a table and specific constant arguments.
- `execute_sprite_commands`: Calls a sequence of functions using various offsets from a table and performs a final calculation with a pointer and a multiplier.
- `execute_short_routine`: A routine consisting of 47 instructions that is called by two other routines.
- `update_coordinate_table`: Updates a table of values by adding calculated offsets to existing entries within a loop.
- `execute_medium_routine`: A routine consisting of 30 instructions that is called by one other routine.
- `update_sprite_attribute_list`: Iterates through a table of offsets to update specific sprite or object attributes.
- `execute_complex_loop_sequence`: Executes multiple loops that call various subroutines to update tables, modify hardware registers, and perform arithmetic based on conditional status codes.
- `determine_state_category`: Evaluates short values in memory against specific thresholds to return one of several integer status codes.
- `update_buffer_values`: Iterates through a table to update values based on a comparison, potentially calling another routine to set bits in memory buffers.
- `populate_data_table`: Fills a table with values derived from the high bytes and bitwise results of repeated function calls.
- `select_table_offset`: Calculates a value based on a pointer and a table offset, then selects a specific memory address from a table using a provided index.
- `process_data_table`: Executes a long sequence of function calls using offsets from a table and performs a final calculation with a pointer and a multiplier.
- `execute_loop_with_conditional_branching`: Iterates up to a fixed limit while executing a sequence of function pointers retrieved from a base pointer offset.
- `process_buffered_data_loop`: Iterates through a loop to update memory buffers and tables while performing conditional checks on status values and calling subroutines.
- `update_state_via_loop`: Performs a loop that updates a table and modifies memory values based on the results of several subroutine calls and status checks.
- `store_byte_to_table`: Stores a 16-bit value into a table at an offset determined by the lower nibble of a parameter.
- `wait_for_value`: Enters a loop that waits until the value at a specific memory location reaches a target value.
- `calculate_integer_square_root`: Performs an iterative calculation to find the square root of a value using a division-based approach.
- `nop_padding_sequence_2`: This routine is empty and consists of non-operational instructions used as padding within the executable.
- `write_three_words`: Stores three specific word values into a global memory area using an offset from a register.
- `write_six_words`: Writes six specific word values into a global memory area using offsets from a register.
- `decrement_table_entries`: Iterates through a table of shorts, subtracting a value from each and applying wrap-around logic based on a threshold.
- `process_table_elements`: Decrements a value in a table and wraps it within a specific range if it falls below a threshold.
- `decrement_table_values_2`: Iterates through a table to subtract a value from each entry and applies a wrap-around check if the result exceeds a certain limit.
- `decrement_table_values`: Decrements values in a table and calls another routine to perform calculations or set bits.
- `update_table_with_offset`: Subtracts a parameter from values in a table and performs conditional arithmetic and bit setting via another routine.
- `execute_indirect_calls_3`: Performs a sequence of indirect function calls and waits for a specific memory value to reach a threshold.
- `execute_indirect_call`: Loads a function pointer from a global memory location and executes it.
- `iterate_vertex_list`: Iterates through a table of short integers, calling another routine for each entry and updating a global counter and pointer.
- `process_vertex_data`: Checks conditions against a table, calls a function via a pointer, and performs arithmetic on specific table elements before updating a status flag.
- `search_table_and_call`: Iterates through a table of words to find an entry matching a parameter, then calls another routine with the resulting pointer.
- `process_table_data`: Iterates through a table to perform calculations and store results in a destination buffer, then optionally calls another routine for each entry in the table.
- `update_coordinate_table_2`: Iterates through a table to modify coordinate values using arithmetic, decrements a counter, and calls a routine to handle remaining entries.
- `render_polygon_vertices`: Calculates vertex coordinates using fixed-point multiplication and scales them based on a global scale factor.
- `update_indexed_counters`: Updates three different memory locations with values derived from a primary counter, which is then incremented if it is below a limit.
- `execute_pointer_list_2`: A short routine that calls another routine to execute a list of function pointers.
- `execute_pointer_list`: Iterates through a list of pointers and calls each one until a counter reaches zero.
- `call_two_functions`: Calls two different subroutines using arguments passed to the routine.
- `execute_function_list`: Calls another routine twelve times in succession.
- `process_function_loop`: Executes a loop that calls multiple subroutines, clears specific memory locations, and decrements a local counter.
- `execute_function_loop`: Executes a series of function pointers and updates a short value in memory.
- `clear_and_call_routine`: Repeatedly calls a routine to clear values until a specific memory location is zeroed, then calls another routine using a value from a table.
- `populate_data_series`: Uses loops to call subroutines with a series of calculated values and concatenated constants.
- `update_shield_status`: Checks a short value to determine which parameter to pass to another routine and updates various memory locations.
- `update_conditional_counter`: Increments a global value by a parameter or resets several memory locations if specific conditions and thresholds are met.
- `add_to_global_counter`: Copies a value to a temporary location, increments a global short by adding another value to it, and sets a constant.
- `handle_game_state`: Processes different commands passed via a parameter to update flags, clear memory, or toggle a byte.
- `update_sprite_animation_state`: Updates sprite coordinate offsets based on a state variable and toggles a bit to alternate between two sets of positional adjustments.
- `update_sprite_visual_state`: Updates sprite properties and toggles a state byte to alternate between two sets of coordinate offsets.
- `process_sprite_animation_frames`: Iterates through a series of function pointers using offsets to update sprite or object properties.
- `execute_pointer_array_operations`: Calls several subroutines using offsets derived from a value in a table and performs arithmetic on a short value.
- `execute_pointer_array_ops`: Calls a series of functions using offsets derived from a short value and executes another routine if a returned value is non-zero.
- `calculate_clamped_parameters_2`: Calculates several values using arithmetic and bit shifts, then clamps the results within specific ranges before storing them in a table.
- `calculate_clamped_parameters`: Performs arithmetic on multiple memory locations, including scaling and clamping values within specific ranges.
- `update_global_parameters`: Clamps a short value within a specific range and performs arithmetic to update several other global variables.
- `update_global_state_2`: Updates several global variables using arithmetic, toggles a byte via XOR, and calls another routine.
- `set_constant_and_call`: Stores a constant value into a global memory location and then calls another routine.
- `iterate_parameter_bits`: Iterates through a value by shifting it right, calling another routine in each loop and performing specific actions when a counter reaches zero.
- `manage_counter_and_pointers`: Decrements a global counter and, if it reaches zero, executes several function pointers and performs word-sized data swaps.
- `initialize_sprite_parameters`: Calls several functions to set up memory and then passes specific values from a table to another routine.
- `update_global_data`: Stores multiple word values into global memory areas and updates two short values and a byte based on a conditional check.
- `process_coordinate_updates`: Writes specific word values to global memory and modifies two short values and a byte.
- `calculate_offset_coordinates`: Calculates two coordinate values using bit shifts and subtraction, then passes these values and various memory offsets to another routine.
- `return_negative_one`: Returns a value of all bits set to one.
- `copy_table_data`: Decrements a global counter and copies a series of values from a source table to a destination table using a loop.
- `fill_memory_ranges`: Executes a series of loops that call a subroutine with decreasing address values and a constant byte value.
- `update_object_coordinates`: Updates two global short values and a zeroed byte based on a switch case, then executes a series of calls to another routine using various offsets of a provided parameter.
- `update_global_values`: Stores two values into global memory, calls a subroutine, modifies one of the values, and calls the subroutine again.
- `execute_sprite_command_stream`: Parses a stream of command bytes to update object coordinates and trigger a sequence of function calls.
- `parse_sprite_command_stream_2`: Iterates through a data stream to interpret command bytes that manipulate sprite coordinate offsets and trigger a series of function calls.
- `render_sprite_command_stream`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream_3`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with various parameters.
- `parse_sprite_command_stream_4`: Iterates through a data stream to interpret command bytes that update coordinate offsets and trigger function calls with specific parameters.
- `execute_sprite_command_stream_2`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream_5`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with various parameters.
- `parse_sprite_command_stream_6`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a function pointer with various parameters.
- `parse_sprite_command_stream_7`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with specific parameter offsets.
- `parse_sprite_command_stream_8`: Iterates through a data stream to interpret command bytes that manipulate sprite coordinate offsets and trigger a function pointer with various parameters.
- `parse_sprite_command_stream_9`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with varying parameters.
- `render_sprite_command_stream_2`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream_10`: Iterates through a data stream of command bytes to update coordinate offsets and execute a series of function calls.
- `parse_sprite_command_stream_11`: Iterates through a data stream to interpret command bytes that modify coordinate offsets and trigger a series of function calls with varying parameters.
- `parse_sprite_command_stream_12`: Iterates through a data stream to interpret command bytes that modify coordinate offsets and trigger a function pointer with varying parameters.
- `parse_sprite_command_stream_13`: Iterates through a data stream to interpret command bytes that modify coordinate offsets and trigger a series of function calls.
- `parse_sprite_command_stream_14`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a series of function calls with varying parameters.
- `parse_sprite_command_stream_15`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a function pointer with specific parameter offsets.
- `parse_sprite_command_stream_16`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream_17`: Iterates through a data stream to interpret command bytes that update coordinate offsets and trigger function calls with varying parameters.
- `parse_sprite_command_stream_18`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with varying parameters.
- `parse_sprite_command_stream_19`: Iterates through a data stream to interpret command bytes that update coordinate offsets and execute a series of function calls with varying parameters.
- `render_sprite_command_stream_3`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream_20`: Iterates through a data stream to interpret command bytes that update coordinate offsets and trigger function calls with specific parameters.
- `execute_sprite_command_stream_3`: Iterates through a data stream of command bytes to manipulate sprite coordinate offsets and trigger specific drawing or transformation subroutines.
- `parse_sprite_command_stream_21`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a series of function calls with varying parameters.
- `parse_sprite_command_stream_22`: Iterates through a data stream to interpret command bytes that modify coordinate offsets and trigger a function pointer with varying parameters.
- `parse_sprite_command_stream_23`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a sequence of function calls with varying parameters.
- `parse_sprite_command_stream_24`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a sequence of function calls with varying parameters.
- `parse_sprite_command_stream_25`: Iterates through a data stream to interpret command bytes that manipulate coordinate offsets and trigger a function pointer with varying parameters.
- `parse_sprite_command_stream_26`: Iterates through a data stream to interpret command bytes that modify coordinate offsets and trigger a series of function calls via a pointer.
- `parse_sprite_command_stream_27`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a function pointer with varying parameters.
- `parse_sprite_command_stream_28`: Iterates through a data stream to interpret command bytes, updating coordinate offsets and executing a function pointer with varying parameters.
- `render_sprite_command_sequence`: Iterates through a command stream to update sprite coordinate offsets and trigger a series of drawing operations with specific parameter offsets.
- `parse_sprite_command_stream_29`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_sprite_command_stream`: Iterates through a data stream to parse command bytes, updating coordinate offsets and triggering a function pointer with varying parameters.
- `parse_command_stream`: Iterates through a data stream to find specific command bytes and executes a routine with a value from that stream.
- `process_input_string`: Updates a buffer with characters based on comparisons of a global string and a counter.
- `update_status_parameter`: Calculates a value based on a flag and a global variable, calls a function through a pointer, and updates a table entry.
- `populate_player_data_table`: Iterates through a data stream to initialize a table of player or entity status parameters.
- `accumulate_and_compare_table_values`: Sums the results of a function called repeatedly through a pointer in a loop.
- `process_conditional_table`: Iterates through a table of values, compares them against a threshold, and calls a function through a pointer if the condition is met.
- `iterate_and_call_table`: Loops through a table of values, compares them against a threshold, and calls a function through a pointer if the condition is met.
- `reset_combat_variables`: Clears multiple global memory locations and calculates a value based on a short variable.
- `update_particle_coordinates`: Iterates through a list of thirty structures to update coordinate values using modular arithmetic and wrapping logic.
- `store_queried_values`: Queries a subroutine twice and stores the results into a structure with bit shifts applied.
- `populate_data_structure`: Queries a function multiple times to populate a large structure with bit-shifted values and specific constants.
- `update_entity_physics_and_lifetimes`: Decrements lifetime counters for various object properties and updates position coordinates by adding velocity vectors.
- `process_table_entries`: Iterates through a table to find entries matching a specific condition, then calls a function with derived values and updates a structure.
- `execute_sequence_of_calls`: Executes a specific sequence of calls to two different subroutines.
- `dispatch_table_values`: Calls several functions stored in a table using values extracted from a provided structure.
- `execute_sequence_calls`: Executes a sequence of function calls from a table based on flags within a provided structure.
- `process_vertex_sequence`: Iterates through a table of vertex structures to evaluate specific coordinate and state conditions.
- `search_and_remove_element`: Iterates through a table to find an entry matching specific criteria, then moves subsequent elements to fill the gap and decrements a counter.
- `update_game_state`: Calculates several values, calls a function through a pointer, and modifies global variables and flags.
- `increment_and_call_function`: Increments a global counter if it is below a limit and calls a function with calculated parameters based on a bitmask check.
- `update_object_spatial_state`: Updates object coordinates or dimensions based on a conditional check of a global counter.
- `call_routine_in_loop`: Iterates through a table and calls another routine for each entry using a value from the table and a constant.
- `process_conditional_logic_2`: Checks a global short value and decrements another short value in a loop, executing various function pointers and clearing memory based on the results.
- `calculate_scaled_value`: Calls a function with a constant value and returns the result shifted right by thirteen bits.
- `call_scaled_value_function`: This routine calls a function that performs a fixed-point scaling operation on a constant value and then returns.
- `store_pointer_and_data`: Stores a pointer and an associated long value into a global memory location.
- `populate_data_buffer`: Initializes a memory buffer by repeatedly calling another routine to fetch values and storing them sequentially.
- `populate_data_table_2`: Fills a table with a sequence of values derived from repeated function calls and arithmetic increments.
- `call_repeatedly_and_delay`: Calls a specific function ten times and then executes a long decrementing loop to create a delay.
- `initialize_laser_sprite_data`: Clears a small memory block by zeroing out six consecutive entries.
- `process_parameter_data`: Retrieves a value via a function call, populates a structure, and conditionally performs further function calls based on a comparison.
- `initialize_and_format_debug_string`: Clears a memory block and calls several subroutines to populate data structures, interpolation tables, and linked lists.
- `initialize_resource_allocations`: Clears several memory locations and populates them by calling a function with various integer arguments.
- `execute_indirect_pointers`: Retrieves two function pointers from a table using an offset from a base address and executes them sequentially.
- `initialize_global_data`: Sets numerous global memory locations to specific constant values and clears a table of thirty-one entries using a loop.
- `initialize_system_state`: Performs a sequence of subroutine calls and writes several constant values to global memory locations.
- `execute_nested_loops_2`: Executes a series of nested loops that call various subroutines and check memory conditions to determine when to return.
- `extract_and_store_values`: Retrieves data from a structure, performs arithmetic, and stores the results into a provided array.
- `allocate_and_deinterleave_buffer`: Allocates a memory block based on calculated dimensions and populates it by deinterleaving data from a source buffer into four separate destination streams.
- `update_blt_sprite_position`: Calculates new coordinates and dimensions for a Blitter-based object, performing clipping against screen boundaries.
- `initialize_interpolation_tables`: Populates several tables with hardcoded values and calculated interpolation data.
- `update_linked_list_nodes`: Traverses a linked list structure by following pointers and applying bitwise offsets to calculate node addresses.
- `update_audio_parameters`: This routine processes two streams of data to update audio hardware registers, including period, volume, and length.
- `initialize_hardware_and_tables`: Sets multiple values in a table, updates a global variable, and modifies audio volume, DMA control, and interrupt enable registers.
- `clear_global_flag`: Sets a global variable to three and writes zero to a specific memory location.
- `set_global_flag`: Writes a value of one to a specific memory location.
- `get_global_flag`: Returns a two-byte value from a specific memory location.
- `dispatch_function_pointer`: Evaluates a parameter and either calls a function through a pointer in a table or performs arithmetic on the parameter.
- `initialize_tower_sequence_data`: Resets several global counters and calculates initial positioning values for a sequence of objects.
- `update_data_table`: Iterates through a table to update various numeric fields using arithmetic and a function call, then performs a conditional copy of table entries.
- `update_object_positions_and_wrap`: Updates a table of coordinate values by applying offsets and performing modular arithmetic to wrap values within a specific range.
- `populate_particle_buffer`: Iterates through a circular buffer of data to extract attributes and populate a structured table.
- `dispatch_object_properties`: Checks a type identifier in a structure and executes a series of function calls to update or pass various data members based on that type.
- `populate_event_table`: Searches through a table of structures and, based on specific conditions and function return values, copies data into a new destination table.
- `update_object_collision_state`: Iterates through a list of vertex structures to perform collision detection and bounding box checks against a specific coordinate.
- `process_object_list`: Iterates through a table of objects, performing conditional checks and calling subroutines to update object states or reset specific global values.
- `update_position_values`: Calculates and stores updated coordinate or distance values into a table based on various conditional arithmetic operations and function calls.
- `calculate_physics_values`: Performs arithmetic on global values to update several short and word variables, then conditionally toggles a byte and calls another routine.
- `update_conditional_state_and_callback`: Checks a value against a threshold to determine whether to increment a counter and execute specific function pointers.
- `execute_callback_sequence`: Calls several functions through pointers, passing high-order bits of a global value as arguments.
- `update_object_sequence_and_physics`: Iterates through a table of coordinate values to apply movement offsets and perform modular arithmetic for wrapping.
- `calculate_scaling_factors`: Divides two sets of global values by a calculated divisor and stores the results in a table.
- `calculate_offset_ratios`: Divides a provided parameter offset by a divisor and stores the resulting quotients in a table.
- `calculate_multiple_ratios`: Performs multiple divisions using two different divisors and stores the resulting quotients into a table.
- `calculate_projection_coordinates`: Calculates three different division results using a parameter and two different divisors, storing them in a table.
- `update_game_state_and_text`: Updates various game state parameters and text strings by calling specific subroutines with constant identifiers.
- `initialize_global_tables`: Resets several global variables to zero or specific constants and populates two tables with incrementing values.
- `update_object_positions`: Updates a series of object coordinate tables by decrementing values and shifting elements through a buffer when thresholds are met.
- `render_object_attributes`: Iterates through a list of data structures, extracting values through pointers and passing them to subroutines for processing.
- `display_game_over_sequence`: Constructs a string representing a numeric value and executes a series of loops and function calls to handle game state transitions.
- `process_conditional_bitmask_logic`: This routine checks specific bits in a parameter and performs a series of comparisons against values stored in a global data structure.
- `dispatch_render_commands`: Checks a parameter and executes a specific sequence of subroutines to pass various global values to a rendering or output routine.
- `process_recursive_bitmask_scaling`: Iteratively extracts bit segments from a parameter to drive a series of recursive calls and scaling calculations.
- `copy_coordinate_data`: Copies four short values from one table to another and then calls a subroutine with a value selected from a third table.
- `update_conditional_state`: Checks a global flag and performs a conditional function call and state updates based on comparisons of various table values and bitwise checks.
- `update_sprite_coordinates`: This routine updates a set of coordinate values by performing arithmetic operations and copying data between memory locations.
- `process_list_elements`: Iterates through a list while incrementing a counter and decrementing a limit, calling another routine during each iteration.
- `populate_particle_effect_buffer`: Iterates through a bitmask to spawn multiple particle or projectile entities into a dynamic array.
- `draw_conditional_graphics`: Performs multiple conditional calls to a routine that takes coordinate-based parameters to draw or manipulate graphical elements.
- `process_sprite_attribute_list`: Iterates through a list of sprite or graphical element attributes, performing conditional updates and coordinate transformations.
- `copy_coordinate_data_to_stack`: Copies a series of short values from a global table into a local stack buffer.
- `iterate_and_update_tables`: Executes a loop that calls several subroutines to update global variables, manipulate coordinate data, and perform conditional graphical operations.
- `manage_game_state_loop`: Executes a loop that calls various subroutines and updates global flags until specific memory conditions are met.
- `manage_state_loop`: Executes a loop that calls various subroutines and updates global flags until specific memory conditions are met.
- `clear_and_alert`: Clears a range of memory and executes a routine that may trigger an alert or call a library function based on a global variable.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `initialize_system_structures`: Initializes various memory structures and performs multiple library calls to manage file and window data.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_list`: Parses a null-terminated string by skipping whitespace and extracting individual tokens, including those enclosed in double quotes.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping when a null terminator is reached or a specified counter reaches zero.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `initialize_window_structure`: Performs a series of library calls to open a file and retrieve window information, storing the resulting pointers and values into a structure.
- `store_and_call_routine`: Stores a value into a global variable, calls another routine with that variable and a second parameter, and then clears the global variable.
- `store_formatted_data`: Stores a value into a location pointed to by a global variable and uses a format string to parse data.
- `update_pointer_data`: Increments a global counter and writes a byte to the location pointed to by a value stored in a global variable.
- `parse_format_string_and_copy`: Parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `perform_recursive_concatenation`: A 48-instruction routine that calls another routine and returns a 64-bit value formed by concatenating register contents.
- `parse_format_string`: Parses a format string containing conversion specifiers to extract and format various data types.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `perform_signed_division_2`: Calculates a 32-bit division and adjusts the sign of the quotient based on the input values.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `execute_subroutine_sequence`: Executes a sequence of subroutine calls, including one that performs no operations and others that increment a pointer.
- `dispatch_subroutine_calls`: This routine executes a sequence of calls to other routines, including one that performs no operations and others that increment a pointer to store a byte from a table and update an iterator state via bitmasks and counters.
- `return_from_subroutine_2`: This routine is an empty function that performs no operations before returning.
- `increment_and_store`: Increments a pointer stored in a global variable and stores a byte from a table into the resulting location.
- `increment_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from a table into the location pointed to by the original pointer.
- `iterate_data_structure`: Recursively traverses a block of memory by incrementing a pointer through a series of structured elements.
- `clear_data_structure`: Recursively calls subroutines based on bit flags in a structure before zeroing out the structure's contents.
- `clear_structure_data`: Checks a byte for specific bit flags to determine which subroutines to call before zeroing out a four-word structure.
- `manage_iterator_state`: Updates a data structure by checking bitmasks and modifying pointer values and counters.
- `update_list_iterator`: Manages a list iterator by advancing pointers, updating counters, and performing conditional logic based on bitmasks and memory values.
- `manage_list_iterator_state`: Updates a list iterator by advancing a pointer and incrementing a counter, or resets the iterator structure if a termination condition is met.
- `update_list_element_2`: Updates a pointer and a counter within a structure and conditionally copies a byte from a table into a memory location.
- `update_list_element`: Updates a pointer and a counter within a structure, sets a bit in a status byte, and stores a byte from a table into a new location.
- `search_and_clear_table_entry`: Iterates through a table to find an entry where a specific byte is zero, then clears three consecutive longwords at that location.
- `search_and_clear_table`: Iterates through a table of entries, and if a specific byte is zero, clears the entire entry and returns a pointer to it.
- `clear_table_entry`: Sets three consecutive longwords at a location pointed to by a global variable to zero.
- `update_parameter_status`: Updates a structure by setting specific word and byte values based on the result of a subroutine call.
- `update_status_flags`: Modifies specific bytes in a structure and sets a bit in a byte based on the result of another routine.
- `traverse_linked_list_2`: Iterates through a linked list of pointers, calling another routine with a value derived from each element, and clears a global pointer when finished.
- `traverse_linked_list`: The routine iterates through a linked list of pointers, calling another routine with a value derived from the current element, and finally stores zero in a global variable.
- `push_linked_list_element`: Updates a global pointer to point to a new structure containing a value and a pointer, effectively inserting an element into a linked list.
- `push_list_element`: Copies a pointer and a value from two sources into a new structure and updates a global pointer to point to this new structure.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Traverses a linked list to find a specific node and removes it by updating pointers.
- `traverse_linked_list_3`: The routine traverses a linked list to find a specific node and then copies a value from that node to a destination.
- `update_global_pointer_2`: Copies a value from one pointer to either a global variable or another provided pointer location.
- `update_global_pointer`: Copies a value from a pointer into a global variable and then calls another routine.
- `check_interactivity_status_2`: The routine validates an index against a table range and checks a status value within that table to determine interactivity.
- `check_interactivity_status`: Retrieves a value from a table using an offset and checks if the resulting status indicates interactivity.
- `return_zero`: An empty routine that returns zero.
- `handle_file_deletion_error`: Attempts to call a routine and, if it returns zero, calls another routine to store a result in a global variable and returns a failure code.
- `return_zero_2`: This routine performs no operations and returns a value of zero to the caller.
- `execute_conditional_logic`: Performs conditional checks and executes either a routine with three parameters or a subroutine based on bitmask results.
- `execute_conditional_write_2`: The routine performs a conditional check on a bitmask and either executes a routine with three parameters or stores an error value in a global variable.
- `execute_conditional_write`: Calls a routine with three parameters and, if it returns a failure code, calls another routine to store an error value in a global variable.
- `return_void_2`: An empty routine that returns.
- `execute_conditional_subroutine`: Calls another routine and checks a bit in the return value.
- `call_indirect_routine`: Calls another routine and returns zero.
- `execute_indirect_call_2`: The routine performs a series of operations involving output and write functions before executing a function call through a pointer stored at an offset from a register.
- `execute_conditional_pointer`: Checks if a value in a global memory location is non-zero and, if so, executes the function pointed to by that value.
- `process_complex_structures`: Iterates through a loop based on a global value, executes function pointers from a table, and calls various routines using values from a global structure.
- `process_complex_data`: The routine iterates through a table, executes several function pointers from a structure, and calls library functions based on global flags.
- `process_conditional_data`: Checks global variables to decide whether to call another routine with specific data or call two different routines, finally returning a value from a memory location.
- `call_library_functions`: The routine calls two different library functions using offsets from values stored in registers and returns a value from a memory location.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `process_table_element`: Uses an index to access an element within a table.
- `close_and_clear_pointer`: Checks a bit in a value, calls another routine if the bit is not set, and then clears the pointer.
- `call_close_handler`: The routine executes a function call through a pointer retrieved from a table using an offset from a value in a register.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a data structure and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_parent_dir`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested structure within a global data table and executes it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a pointer from a table located at an offset from a global variable and executes the function at that address.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Calls a library function to allocate memory, initializes several fields in the allocated block using provided parameters, and manages the resulting pointer.
- `initialize_recursive_structure`: If a specific offset in the passed pointer contains a non-zero value, the routine calls itself recursively.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `destroy_display_entry`: It calls another routine passing the input pointer as an argument.
- `allocate_and_populate_structure`: If the first argument is non-zero, it calls a library function to allocate memory.
- `initialize_structure_fields_2`: Sets a byte and two double words within a structure to their maximum values, then calls another routine using a pointer from the structure.
- `initialize_structure_fields`: Sets a byte to its maximum value and two dwords to their maximum values within a structure, then calls another routine.
- `execute_indirect_library_call`: This routine performs an indirect call to a function located at an offset within a table retrieved from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_library_call_2`: This routine performs an indirect call to a function located at a different offset within a table retrieved from a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_4`: Retrieves a function pointer from a nested offset within a table and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: Retrieves a pointer from a location offset from a global variable and executes the function at the resulting address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_6`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_main_loop`: Executes a loop that repeatedly calls various routines to perform calculations, string copying, and bitwise operations on global variables.
- `write_structure_constants`: The routine stores two constant values into consecutive fields of a structure pointed to by a register.
- `calculate_offset_address`: The routine returns the sum of a value stored in a global variable and a constant offset.
- `copy_data_buffer`: If a register is zero, it initializes a destination buffer with a constant and a specific value.
- `ensure_path_separator`: The routine copies a null-terminated string from a source pointer to a destination buffer.
- `update_string_buffer_2`: Calculates the length of a string within a data structure and clears the remaining space in a buffer.
- `check_string_for_colon_and_call`: Checks a global variable for a non-zero flag and, if set, scans a zero-terminated string in a register for a colon character, calling another routine when the colon is absent.
- `find_colon_character`: The routine iterates through a string looking for a colon character.
- `store_task_reference`: The routine calls another routine through a pointer stored in a global variable and, if a register value is zero or a table entry is negative, stores the register value into a field of the returned structure.
- `return_constant`: The routine returns a constant value.
- `modify_path_string`: Iterates through a string to find a forward slash and replaces it with a null terminator.
- `copy_string_with_limit_2`: The routine checks a global variable and, if valid, copies characters from a source to a destination while decrementing a counter and checking for a null terminator or a specific count.
- `initialize_copper_dma_tables`: The routine obtains a pointer from another routine, stores a constant word and a 32-bit value into several offsets of the pointed-to block, and writes a jump opcode followed by a pointer to itself or to a global variable at many other offsets.
- `loop_call_table_entries_with_mask`: The routine loops seven times, writing a decreasing counter to a global variable and calling a routine from a table.
- `loop_call_table_entries`: The routine loops seven times, each time writing a constant to a global variable and then calling a routine taken from a table pointed to by a register.
- `update_and_jump`: The routine updates a global variable by adding an offset to its current value and then performs an indirect jump to a location.
- `clear_two_word_table_entry`: Clears a two-word table entry pointed to by a global variable and then stores two input words into the next two words of that entry.
- `set_table_entry`: Writes a constant value and two word inputs into a table pointed to by a global variable.
- `update_counter_offset`: Reads an integer from a pointer, then adds a constant to a word at a fixed offset and increments a short at another fixed offset within that structure.
- `clear_message_struct`: Clears several fields in a message structure pointed to by a0, including the next pointer, the io request pointer, and two additional fields, and conditionally frees an associated memory block via a helper routine.
- `store_pair_to_table`: The routine writes a value from a register to the location pointed to by a pointer and writes a value from another register to the next entry in the same table.
- `set_player_stats`: Stores a 16-bit value from a register and a 32-bit value from a register into a structure pointed to by a pointer, writes the 16-bit value into an offset within the structure, computes a shifted value and stores it into two fields of the structure, and clears a field in the structure.
- `zero_nine_byte_table_entries`: Clears a nine-word block pointed to by the first argument register by storing zero repeatedly into consecutive words.
- `zero_long_block_and_byte`: Clears a block of nine longwords pointed to by a pointer and then writes a single byte to an offset within that block.
- `multiply_input_by_eight`: The routine returns the value it receives in D0 multiplied by eight.
- `initialize_object_descriptor`: The routine writes a fixed set of constants and pointers into a structure pointed to by A1, initializing fields with a global variable reference, zero values, sentinel values, and a few non-zero constants.
- `store_byte_to_offset`: Stores a single byte from a register into a memory location at a fixed offset from a pointer.
- `store_byte_at_offset`: Stores a byte from a register into a memory location at a specific offset from a pointer.
- `store_byte_at_offset_2`: Stores a byte from a register into a memory location at a different offset from a pointer.
- `store_word_at_offset`: Stores two consecutive words from registers into a memory location starting at a specific offset from a pointer.
- `copy_player_stats_to_slot`: Copies a pointer from a source structure into a destination slot and clears a word in the destination.
- `apply_bitstream_masking_to_buffer`: Iterates through a source bitstream to extract values and applies them to a destination buffer using various bitwise logical operations.
- `decompress_bitstream_data`: Expands a variable-length encoded bitstream into a destination buffer using bitwise logical operations and lookup tables.
- `expand_compressed_data`: Decompresses a bitstream into a destination buffer using a variable-length encoding scheme.
- `initialize_tile_buffer`: The routine writes a header into a 16-bit word buffer pointed to by a pointer argument, storing a computed alignment value, an unchanged word, a zero byte, a byte from a register, and a zero word, then fills the next seven 32-bit slots with a constant pattern.
- `store_color_word`: Stores a 16-bit value into a global variable table using an index derived from the low byte of a register doubled.
- `read_byte_from_table`: Computes an address from a global variable and an input value, then returns a byte formed by combining the byte at that address with the next byte shifted left by four bits.
- `extract_global_field`: Returns a value from a global variable that has been masked and right-shifted by eight bits.
- `extract_bitfield_from_table`: Checks if a coordinate is within bounds and then iterates through a table to build a 32-bit value by testing specific bits in memory.
- `return_all_ones`: Returns a 32-bit value consisting of all ones.
- `modify_bit_sequence_2`: Iterates through a list of pointers to set or clear specific bits in memory based on a bitmask and a counter.
- `modify_bit_sequence`: Iterates through a list of pointers to clear or set specific bits in memory based on a bitmask and a counter.
- `return_all_ones_2`: Returns a value consisting of all bits set to one.
- `write_byte_to_offset`: Stores a byte from a register into a memory location at a fixed offset from a pointer and executes an empty operation.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_nested_loops`: Performs a nested loop structure that increments two counters and calls another routine.
- `execute_nested_loop`: Performs a nested loop structure that repeatedly calls another routine.
- `nop_padding_sequence_3`: This routine is empty and consists of two instructions that perform no operation.
- `clear_flag_and_table_entry`: Clears a bit in a byte at an offset from a pointer and writes a constant value to a table entry.
- `increment_bit_field_2`: Sets bits in a byte at a specific offset until a bit collision occurs, then updates a value in a table.
- `return_all_ones_3`: Returns a value consisting of all bits set.
- `increment_bit_field`: Checks a byte against a value in a table and, if they differ, repeatedly sets bits in that byte until a collision occurs, then updates a value in a table.
- `store_table_entry_and_enable_dma`: Stores a register value into a table indexed by a pointer-derived value and enables DMA channels by writing to a global variable.
- `update_table_data_loop`: Performs a loop that calculates several values and stores them into a table entry pointed to by an indexed value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_data_structure_elements`: Calculates several values based on a function return and an offset, then stores them into a multi-byte structure via a pointer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `strip_path_suffix`: Searches for a forward slash in a string and replaces it or the preceding characters with a null terminator.
- `update_bitmask_and_call_subroutines`: Performs a bitwise OR operation on a memory location using a constant mask and executes a sequence of subroutines while updating a stack-like pointer with specific values.
- `copy_string_to_table`: Executes a loop that copies a string into a table and sets specific fields within that table entry.
- `manage_resource_loop`: Manages a complex loop involving DMA register updates, routine calls, and copying string data into a table structure.
- `update_pointer_value`: Calls a function via a pointer stored in a global variable and conditionally stores a value into a memory offset of the returned address.
- `copy_null_terminated_string`: Performs a loop that copies a null-terminated string from one memory location to another and modifies specific values in a structure.
- `branch_on_byte_value`: Checks a byte at a specific offset within a structure and calls one of two different routines based on whether that byte is zero.
- `update_string_buffer`: Calculates the length of a null-terminated string and clears a portion of memory based on a calculated offset.
- `copy_data_block`: If a register is non-zero, it copies a block of bytes from a table to a destination pointer; otherwise, it initializes a destination with a constant and sets a value at an offset.
- `process_string_loop`: Repeatedly calls other routines and performs a string copy from a source pointer to a destination offset within a structure until a null terminator is reached.
- `execute_string_copy`: Enters a loop that modifies a global value, calls other routines, and copies a null-terminated string into a structure.
- `initialize_string_copy`: Calls an initial routine before entering a loop that performs bitwise operations on a global variable and copies strings into a structure.
- `perform_string_transfer`: A loop that waits for a specific routine to return a non-zero value before copying a null-terminated string into a structure.
- `update_register_state`: Modifies a value at an offset from a register and calls other routines in a loop.
- `copy_null_terminated_string_2`: A routine that performs bitwise OR operations on a global variable and copies a string into a structure after ensuring a pointer is non-null.
- `loop_string_copy_with_check`: Checks a pointer via a routine and, if valid, copies a null-terminated string into a structure.
- `transfer_string_data`: Repeatedly executes routines and copies a null-terminated string into a structure at a specific offset.
- `handle_string_copy_loop`: A loop that verifies a pointer is not null and then copies a null-terminated string into a structure.
- `execute_unknown_routine`: A routine consisting of thirty-two instructions with no identifiable external calls or register usage in the provided snippet.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: An empty routine that performs a single return instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_infinitely`: A routine that enters an infinite loop.
- `fetch_and_copy_data`: Calls a routine to retrieve a value, then either performs several calls including a data transfer and a cleanup call, or copies data from a buffer into a stack-based array.
- `store_retrieved_value`: Calls a routine to get a value, stores it in a global variable, and if that value is non-zero, passes it to another routine.
- `call_with_global_value`: Checks a global variable and, if non-zero, passes the result of another routine to a third routine.
- `transfer_data_on_success`: Calls a routine to retrieve a value; if successful, it passes three parameters to a data transfer routine and a cleanup routine, otherwise it performs several other calls.
- `transfer_data_if_nonzero`: Calls a routine to retrieve a value; if the result is non-zero, it passes the result and two other parameters to a data transfer routine and a cleanup routine.
- `perform_nested_loops`: Executes nested loops that call two different routines multiple times using calculated offsets based on input parameters.
- `update_hardware_registers`: Calculates several values based on global variables and input parameters, then enters a loop that updates multiple memory-mapped registers and waits for a bit flag to clear.
- `update_counter_and_call_function`: Updates a value in a global structure using a short from a table and a bitwise shift, then calls a function pointer and returns a shifted result.
- `call_function_on_condition`: Checks if a value at an offset in a parameter is non-zero and calls another routine if it is.
- `wait_for_zero_and_recurse`: Calls a routine and enters a loop that waits until a specific short value in a table becomes zero before calling itself.
- `execute_conditional_tasks`: Checks three different values in a table to conditionally call routines, decrements a byte, and sets a word to a constant value.
- `call_routine_twice`: An empty routine that calls another routine twice.
- `calculate_and_store_offsets`: Performs arithmetic on input parameters to calculate offsets and then enters a loop that writes several values to a table and a global variable.
- `call_routine_with_calculated_args`: Calls another routine using arguments derived from arithmetic operations on values stored in a table.
- `handle_function_return_failure`: Calls a function pointer with a parameter and, if the result is zero, calls two routines, a third routine with a constant, and a fourth routine.
- `call_function_with_constant_2`: Calls a function pointer with the constant value three.
- `call_function_pointer`: Calls a routine through a function pointer stored in a global structure.
- `initialize_data_structure`: Populates a memory block with specific byte and word values and stores a calculated pointer.
- `initialize_resource_handles`: Requests and validates several resource handles through a function pointer table, checking for null returns to trigger an error routine.
- `execute_indirect_calls`: Performs several indirect function calls using values retrieved from memory and updates a global variable with two specific values.
- `multiply_two_shorts`: Returns the product of two 16-bit values passed within a single 32-bit parameter.
- `store_and_return_values`: Calls three different routines and stores two 16-bit values into a pointer provided by the caller.
- `lookup_scaled_value`: Calculates an index based on an input, performs wrapping logic, and retrieves a signed 16-bit value from a table.
- `lookup_scaled_value_alt`: Performs a modulo and division operation to index into a table and returns a value with a sign bit determined by an input flag.
- `calculate_scaled_offsets_2`: Computes four different 16-bit values using division and bit shifting, then stores them in a global table.
- `update_scaled_coordinates_2`: Calculates two 16-bit values using bit-shifted addition and division, then stores them in a global table.
- `add_bcd_values`: Adds four separate byte values from two parameters to a four-byte buffer and applies BCD adjustment to each byte.
- `update_status_table`: Sets a 16-bit value in a global variable and updates a series of four table entries based on specific bits in the input parameter.
- `store_word_to_global`: Stores a 16-bit value from a parameter into a global memory location.
- `calculate_complex_value`: Stores two short values into a table and calculates a third value using a series of arithmetic operations and shifts before storing it.
- `store_and_call`: Stores two short values from a parameter into a table and then calls another routine.
- `store_and_return`: Stores two short values from a parameter into a table, calls another routine, and returns a concatenated result.
- `set_bits_loop`: Iterates through a memory range, setting a specific bit in each byte based on a value in a register.
- `set_bits_with_mask`: Iterates through a memory range, applying a bitwise OR with a constant mask to a register and setting a bit in each byte.
- `set_bits_loop_alt`: Performs a loop that decrements a pointer and sets a bit in the target byte using a register value.
- `set_bits_loop_copy`: Decrements a pointer and sets a bit in the destination byte using a bitwise OR operation within a loop.
- `enter_infinite_loop`: Empty routine that enters an infinite loop.
- `execute_infinite_loop`: The routine enters an endless loop that prevents further instruction execution.
- `execute_infinite_loop_2`: The routine enters an infinite loop that prevents further execution.
- `execute_infinite_loop_3`: The routine consists of an empty infinite loop that prevents further execution flow.
- `nop_padding_gap`: The routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `execute_infinite_loop_4`: The routine enters an infinite loop that prevents further execution of the current thread.
- `nop_padding_sequence_4`: The routine is empty and consists of a single instruction that serves as a gap or padding within the executable segment.
- `execute_infinite_loop_5`: This routine enters an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_6`: This routine enters an infinite loop that prevents further execution of the calling code.
- `execute_infinite_loop_7`: The routine enters an unconditional loop that prevents further execution of the calling code.
- `execute_infinite_loop_8`: This routine enters an infinite loop that prevents further execution of the current thread.
- `execute_infinite_loop_9`: The routine performs an infinite loop that prevents further execution of the program flow.
- `execute_infinite_loop_10`: The routine enters an endless loop that prevents further execution of the current thread.
- `execute_empty_loop`: The routine contains an infinite loop and performs no operations.
- `execute_empty_loop_2`: This routine consists of a single instruction that results in an infinite loop.
- `execute_empty_loop_3`: The routine enters an infinite loop without modifying any data.
- `execute_empty_loop_4`: This routine is an infinite loop that performs no functional tasks.
- `execute_empty_loop_5`: The routine performs no actions and loops indefinitely.
- `execute_empty_loop_6`: This routine executes an infinite loop and does not interact with memory.
- `execute_empty_loop_7`: The routine is an infinite loop that performs no logic.
- `execute_empty_loop_8`: This routine consists of a single instruction that creates an infinite loop.
- `execute_empty_loop_9`: The routine performs no operations and remains in an infinite loop.
- `execute_empty_loop_10`: This routine is an empty loop that does not change the state of the processor.
- `nop_padding_sequence_5`: This routine is empty and consists of a single instruction that does not perform any operations.
- `execute_infinite_loop_11`: Enters a loop that never terminates.
- `trap_in_loop`: Contains a do-while loop that repeats indefinitely.
- `loop_indefinitely`: Executes a continuous loop without exit.
- `run_infinite_loop`: Enters a block that loops forever.
- `stay_in_loop`: Performs a repetitive loop that does not end.
- `loop_forever`: Executes an infinite loop structure.
- `halt_in_loop`: Enters a loop that prevents further execution.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_instruction`: The routine is empty and performs no operations.
- `exit_empty_routine`: An empty routine that returns immediately.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller without performing any work, touching no registers, memory, or hardware.
- `perform_empty_operation`: This routine performs no operations and returns immediately to the caller.
- `return_no_op`: The recovered routine consists of a single instruction that returns immediately, performing no work.
- `return_no_op_2`: The routine consists of a single instruction that immediately returns to its caller, performing no computation, memory access, library call, or hardware register write.
- `return_void`: This is an empty routine that performs no operations and immediately returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_in_hardware_register`: Clears a single bit in a hardware register using a value stored at an offset from a pointer in a0.
