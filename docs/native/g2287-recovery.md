# Game 2287 (star wars - the empire strikes back (1988)(domark)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02287). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (963488 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 262 raw-word lines |
| Functions named and described | pass, 280 of 280 named (0 author symbols), 280 of 280 described (8 by rule, 258 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3730 of 3733 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5284 of 5736 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 37419 |
| code | 21618 |
| data | 726956 |
| padding | 1879 |
| unused | 175616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire_TRAINER | 2 | 112 | 9 |
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire | 2 | 21 | 3 |
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3743 | 175 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 619 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 473 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 616 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 2 |

## Cases handled

- runtime segment an address (play-long-joy): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2366 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 526 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 391 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m3000): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 369 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m3000) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-long-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-long-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (5 bytes change between runtime segment an address (play-rs-m3000) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m3000) and runtime segment an address (play-long-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-rs-m3000) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-long-mixed)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address (play-rs-m3000)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-mixed) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-rs-m3000) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- star wars - the empire strikes back (1988)(domark)[cr qtx]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire_TRAINER: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- star wars - the empire strikes back (1988)(domark)[cr qtx]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire_TRAINER

## Copy protection found

- `set_dma_control_bits` (runtime segment an address): Checks a global variable and, if non-zero, performs a loop to set specific bits in a register and updates a global variable and another memory location. Kind: hardware register initialization/timing check. Pass path: the branch where cRam00052c01 is non-zero.
- `update_hardware_and_tables` (runtime segment an address): Modifies a global variable based on a flag, iterates through a linked list to perform arithmetic, and updates hardware registers and memory tables. Kind: disk hardware/track validation and table update. Pass path: The routine updates internal hardware state tables (likely track/sector offsets or timing parameters) based on the result of `u5__sub_10FA`. A successful check allows the routine to proceed to the loop that populates `pcVar5` (the hardware/.
- `configure_dma_parameters` (runtime segment an address): Updates memory tables and hardware registers, including DMA and Blitter parameters, based on calculated indices and input values. Kind: disk hardware/DMA configuration for custom track reading. Pass path: the routine completes and returns to sub_0048, allowing the subsequent disk read/DMA operation to proceed normally.

## Functions


### Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire_TRAINER

- `configure_dma_and_interrupts`: Trainer program entry that writes the DMA control and interrupt enable registers and a CIA-B port register, then makes one call to an unnamed routine.
- `call_unnamed_routine_once`: A seven-instruction routine reached by address, called by one unnamed routine, that makes a single call to another unnamed routine.
- `depack_stream_using_bit_reader`: Depacker routine that decodes a packed stream by calling a bit-reader refill routine and a bit-read routine, with two further unnamed calls.
- `dispatch_to_unnamed_routine`: A three-instruction entry, reached through a pointer in data, that makes one call to an unnamed routine.
- `read_packed_bit_field`: Reads a field of bits from the packed stream through the bit-reader routine and makes two further unnamed calls.
- `set_background_colour_from_bits`: Reads bits from the packed stream through the bit-reader routine and touches the background colour register, with two further unnamed calls.
- `write_countdown_to_colour_register`: Counts a 16-bit value down from minus one through every value until it wraps back to minus one, writing each value to a global location on each step.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:empire

- `enter_supervisor_and_configure_dma`: Program entry that calls the exec library's SuperState routine to enter supervisor mode, writes the DMA and interrupt enable registers, then makes one unnamed call.
- `run_handler_then_call_routine`: Entered through an address in data as a handler and executed in the reference run, this seven-instruction routine makes one call to an unnamed routine.
- `write_dma_control_register`: A two-instruction handler reached through a pointer in data that writes the DMA control register.

### Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_decreasing_color_mask`: Iteratively applies a decreasing bitmask to a table of values and writes the results to a global variable while performing a delay loop.
- `apply_increasing_color_mask`: Iteratively applies an increasing bitmask to a table of values and writes the results to a global variable while performing a delay loop.
- `set_global_values`: Sets a global variable and another specific memory location to a constant value.
- `check_global_flag`: Checks if a global variable is non-zero and, if so, calls another routine.
- `wait_for_dma_interrupt`: Polls a hardware register bit until it is cleared.
- `configure_blitter_transfer`: Polls a hardware register bit, then writes values to several memory locations used for blitter configuration and execution.
- `clear_global_variable`: Sets a global variable to zero through multiple arithmetic operations.
- `process_table_loop`: Clears a global variable, performs arithmetic to scale it, and then iterates through a table of function pointers to execute them.
- `increment_and_execute_list`: Increments a global variable and then executes a series of function pointers from a list located via a pointer in a table.
- `execute_function_list`: Executes a series of function pointers from a list, using an offset to locate each function.
- `call_indexed_function_list`: Iterates through a list of pointers, applying a constant offset to each pointer before calling the resulting address.
- `average_three_values`: Calculates the arithmetic mean of three pairs of values and stores the results in a table.
- `sum_three_values`: Adds three pairs of values together and stores the results in a table.
- `subtract_three_values`: Subtracts one value from another for three pairs and stores the results in a table.
- `negate_three_values`: Negates three values retrieved from a table and stores them in another location.
- `copy_long_word`: Copies a long word and a word from one memory location to another.
- `calculate_weighted_averages`: Loads values from a pointer, performs a series of additions and bitwise shifts to calculate weighted values, and stores the results in multiple global variables.
- `calculate_weighted_values`: Performs a series of arithmetic operations using three input values to populate multiple entries in a large table.
- `update_scaled_table`: Iterates through a list of values and uses a routine to calculate scaled offsets, which are then stored in a global table.
- `fill_scaled_buffer`: Iterates through a list of values, calculates scaled indices using a global variable and a table, and stores the resulting values into a destination buffer.
- `fill_buffer_with_scaled_data`: Clears a memory range and then populates it with values retrieved from a global table using scaled indices.
- `process_data_list`: Iterates through a list of values, performing conditional memory fetches and updates based on bitwise checks and byte table lookups.
- `resolve_indirect_table_indices`: Iterates through a list of indices to perform multi-level pointer dereferencing and conditional updates to a global variable.
- `process_nested_table_indices`: Traverses a table of indices, performing conditional checks against a byte table before updating a global variable with values from another table.
- `traverse_conditional_table`: Iterates through a table, performing bitwise checks and conditional jumps based on values in a byte table to update a global variable.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies (a7)+ to d0, then loads 0 into d0, then returns.
- `set_byte_flags`: Iterates through a list and sets or clears bytes in a destination buffer based on a condition in a source table.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `call_indirect_function`: Loads a function address from a table and calls it with a pointer to the next element in that table.
- `update_global_accumulator`: Updates a global variable by adding values retrieved from a table and a fixed offset.
- `update_global_from_tables`: Updates a global variable by adding values retrieved from three different memory locations.
- `copy_global_to_multiple_locations`: Copies the value of a global variable into three different global variables.
- `check_range_bounds`: Checks if two sets of value pairs fall within a specific range relative to a global variable and calls another routine if they do.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `double_global_value`: Stores the result of a global variable added to itself back into the same location.
- `double_global_value_2`: Performs a doubling operation on a global variable and stores the result back into it.
- `double_global_value_3`: Multiplies a global variable by two and updates the original memory location.
- `double_global_value_4`: Calculates the sum of a global variable with itself and writes it to the same address.
- `double_global_value_5`: Updates a global variable by adding its current value to itself.
- `double_global_value_6`: Doubles the value stored in a global variable and saves the new value.
- `double_global_variable`: Loads a global variable into a register and adds that value to the original variable, then adds the resulting sum to itself.
- `calculate_checksum`: Doubles a global variable and returns a value derived from an XOR operation between its low byte and the high byte.
- `decrement_and_check_counter_2`: Decrements a global variable and, if it reaches zero, updates a global variable using a value from a table indexed by the previous variable's lower bits, sets two additional global variables, and passes a pointer to the first variable to another routine.
- `initialize_pointer`: Sets two consecutive global values and passes the address of the first to another routine.
- `modify_memory_and_globals`: Performs a bitwise OR on a memory location via a pointer, sets a global variable, and then clears or sets several other global variables.
- `initialize_global_sequence`: Performs a bitwise OR on a memory location and then sets a sequence of global variables to specific constant values.
- `manage_state_and_timer`: Decrements a global counter and executes a function pointer from a table once the counter reaches zero.
- `nop_routine`: The routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `compute_matrix_products`: Uses three input values to perform multiple 32-bit multiplications and stores the resulting high and low words into several global variables.
- `calculate_complex_products`: Performs multiple 32-bit multiplications using values from two different tables and stores the high and low results into several global variables.
- `sum_and_double_triplets`: Sums three consecutive 16-bit values from a table and doubles the result, repeating this for three distinct triplets.
- `scale_global_values`: Multiplies three different global variables by specific factors and stores the results back into their respective memory locations.
- `scale_vector_data`: Multiplies three consecutive 16-bit values in a source table by three different factors and stores the results in a destination table.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a table using an offset.
- `dispatch_indirect_function`: Retrieves a function address from a table using an offset derived from a pointer stored in a register and executes it.
- `update_and_dispatch_offset_8`: Adds a value from a pointer to a memory location at an offset of eight and then calls a function from a table using an index.
- `update_and_dispatch_offset_6`: Adds a value from a pointer to a memory location at an offset of six and then calls a function from a table using an index.
- `update_and_dispatch_offset_10`: Adds a value from a pointer to a memory location at an offset of ten and then calls a function from a table using an index.
- `dispatch_indirect_function_2`: Adds a value from a pointer to a register and calls a function from a table using an offset.
- `decrement_counter_at_pointer`: Updates a structure by storing two values and decrementing a 16-bit integer at a specific offset within that structure.
- `update_pointer_and_call`: Decrements a value at a memory location, updates a pointer, and calls a function from a table.
- `call_indirect_via_pointer`: Calls a function from a table using an offset retrieved from a pointer.
- `call_indirect_via_offset`: Calls a function from a table using an offset stored at a specific location relative to a register.
- `execute_sequential_calls`: Calls a function pointed to by a register and then calls another function from a table using an offset.
- `modify_data_and_call`: Subtracts a value from a memory location, adds a value to another memory location, and calls a function from a table.
- `clear_memory_block_2`: Fills a block of memory with zeros using a loop that runs seven hundred times.
- `fill_memory_with_zeros`: Fills a specific block of memory with zero values by iterating through a loop seven hundred times.
- `clear_register_contents`: clears the contents of a register and returns.
- `decrement_global_variable`: Decrements a global variable by one.
- `process_bytecode_instruction_stream`: Iterates through a sequence of data, interpreting values as instructions to perform indirect function calls, memory updates, or conditional branching.
- `check_memory_sequence`: Iterates through a memory block and returns if it encounters a zero value.
- `do_nothing`: This routine is empty and performs no operations.
- `calculate_scaled_offsets`: Performs division and modulo operations on two input values using a third value to calculate and return scaled offsets added to a global base address.
- `clamp_global_value`: Checks if a global variable is negative and sets it to zero if it is.
- `update_global_counter`: Increments a value at a specific memory location and modifies a global variable based on the result.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 176 into d0.
- `update_memory_sequence`: Decrements a value at a pointer and updates a global variable based on a memory sequence.
- `update_coordinate_state`: Calculates absolute differences between global values and updates several global variables and counters based on specific range checks.
- `check_bounds_and_set_flag`: Compares two calculated differences against a constant and, if within bounds, calls other routines and sets a global variable to a constant.
- `execute_list_of_routines`: Calculates a pointer into a table based on a global variable and an index, then executes a loop of several routines.
- `iterate_and_execute_list`: Iterates through a list of pointers, calling various routines and updating memory based on a counter.
- `decrement_and_process_list`: Decrements a byte at a pointer, removes the element from a list if it reaches zero, and updates a counter.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work.
- `set_bitmask_pixel_planes`: Calculates a destination address using a table and bitwise shifts, then performs bitwise OR operations on up to four memory planes based on a bitmask.
- `apply_bitmask_to_planes`: Calculates a destination address using a table and bitwise shifts, then performs bitwise OR operations on four different memory planes.
- `execute_complex_logic`: Performs bitwise OR operations on memory buffers and executes conditional arithmetic on a global variable.
- `apply_blitter_mask_to_planar_data`: Performs a bitwise OR operation on specific memory locations using a byte fetched from a table.
- `apply_bitmask_to_memory`: Calculates a memory address using a table lookup and applies a bitwise OR operation with a specific byte value to several memory locations spaced at regular intervals.
- `apply_bitwise_mask_to_memory`: Fetches data from a table and performs a bitwise OR operation between a byte and several memory locations in a loop.
- `apply_bitmask_to_memory_2`: Performs a bitwise OR operation between a byte and several memory locations within a loop.
- `perform_blitter_operations`: Configures and executes Blitter operations to manipulate memory.
- `process_complex_logic`: Executes a complex series of conditional checks and arithmetic operations on a global variable.
- `configure_blitter_transfer_2`: Sets up and executes multiple Blitter operations by configuring hardware registers and source/destination addresses.
- `add_to_memory_locations`: Adds two values to two different longwords located at a specific memory address.
- `reset_global_variable`: Sets a global variable to zero.
- `clear_memory_block_3`: Zeroes out a block of memory starting from its own location for a fixed number of words.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_data_register`: Performs a logical OR operation on a data register with zero.
- `clear_memory_block_4`: Fills a block of memory with zeros using a loop.
- `clear_memory_block_5`: Fills a memory area with zeros using a loop that runs thirty-nine times.
- `clear_memory_block_large`: Fills a memory area with zeros using a loop that runs forty-nine times.
- `clear_memory_range`: Zeros out a block of memory by writing words to a location through a loop that runs forty-nine times.
- `search_and_initialize_data`: Decrements a value in a structure and, if it reaches zero, performs a series of memory writes and calls another routine.
- `search_for_zero_word`: Iterates through memory in eight-byte increments up to nine times, returning if a zero word is encountered.
- `update_memory_buffer`: Iterates through a memory range to modify values based on a comparison and arithmetic scaling.
- `compute_scaled_coordinate_offsets`: Calculates scaled coordinate offsets by dividing a constant by a global variable and iteratively updates a memory block using a descending loop.
- `calculate_scaled_coordinate_offsets`: Calculates scaled offsets by dividing a constant by a global variable and iteratively updates a memory block using a descending loop.
- `calculate_offset_differences`: Performs arithmetic operations involving a table of values and input registers within a loop.
- `iteratively_scale_coordinates`: Calculates a scaling factor based on a division of a fixed value and enters a loop to update a series of memory locations using arithmetic offsets.
- `calculate_coordinate_scaling`: Performs arithmetic operations involving division and conditional checks to update global variables and coordinate data.
- `update_coordinate_data`: Sets a global variable and performs a sequence of calls to update coordinate tables and pointer data.
- `process_coordinate_loop`: Calculates a scaling factor and enters a loop to iteratively update a series of memory locations and global values.
- `update_coordinate_table`: Calculates and stores four signed values into a global variable by adding and subtracting values from a source table.
- `update_pointer_data`: Repeatedly calls another routine and stores the resulting values from two registers into a memory location pointed to by a register.
- `initialize_data_parameters`: Sets several global variables to specific constant values and calls a routine that copies and doubles data.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `apply_color_effects`: Manipulates global variables and hardware color registers through bitwise operations and conditional arithmetic.
- `update_color_registers`: Writes values to color registers and checks a global variable before potentially calling another routine.
- `copy_data_loop`: Iterates through a loop to move data from one location to another while performing nested calls.
- `copy_and_double_data`: Copies data from one location to another in a loop while performing arithmetic on the values being stored.
- `apply_bitmask_loop`: Sets a global variable to a specific value and executes a loop forty-nine times that performs bitwise operations on memory planes.
- `initialize_global_table`: Sets several global variables and memory locations to specific constant values.
- `update_color_and_counters`: Iterates through a table to compare values against globals, updates hardware color registers, and manages several decrementing counters.
- `execute_blt_operations`: Performs bitwise OR operations on a buffer and sets up hardware parameters for bitblock transfers.
- `zero_data_register`: Sets the contents of a data register to zero using a logical OR instruction.
- `bitwise_or_buffer_short`: Performs bitwise OR operations on data segments from a global variable into a destination.
- `bitwise_or_buffer_alt`: Executes bitwise OR operations on data segments from a global variable into a destination.
- `bitwise_or_buffer_full`: Uses a loop to perform bitwise OR operations on data segments from a global variable into a destination.
- `set_color_register`: Writes specific values to a global variable and modifies a hardware color register.
- `transform_table_entries_2`: Updates a table by storing masked bytes and raw words while checking a global variable.
- `update_global_value`: Performs arithmetic on a global variable and conditionally assigns specific constant values to it.
- `execute_sequence_of_calls`: Executes a fixed sequence of several other routines.
- `iterate_and_call`: Loops through a sequence of memory locations and calls another routine if a byte is non-zero.
- `iterate_data_table_with_offsets`: Iterates through a loop nine times, calling two subroutines and a function that uses a calculated pointer derived from a base address and an incrementing offset.
- `get_offset_value`: Returns a two-byte value from a table using an offset and a register.
- `populate_data_structure_2`: Iterates through a loop to populate a memory structure with values from a sequence and a global variable.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `process_data_table`: Iterates through a table of sixteen words, performing arithmetic operations and updating the table entries.
- `transform_data_table`: Iterates through a table of sixteen-bit values, performing arithmetic operations on each entry using a global variable and constant offsets, then stores the results back into the table.
- `transform_table_data`: Iterates through a table of sixteen words, performing arithmetic operations on each entry using a global variable and constant offsets.
- `get_sub_640e_value`: Returns a constant value used in table calculations.
- `test_bit_status`: Tests the status of a specific bit within a long word using a bit test instruction.
- `process_table_offsets`: Iterates through a table of short integers, performing arithmetic updates on each entry using a global variable and constants.
- `update_memory_via_bitmask`: Checks various bits in a variable to determine which constant values to store at a calculated memory address.
- `transform_table_entries`: Iterates through a table in increments of two, storing masked high bytes and raw words from three consecutive routine calls into the table.
- `update_structure_fields`: Modifies two fields within a structure pointed to by a register, including a wrap-around check.
- `process_shifted_data`: Iterates through memory to perform bitwise shifts and additions, followed by a loop that performs conditional memory copies.
- `check_global_state`: Checks a global variable and updates it or other status flags based on specific conditions.
- `update_global_state_2`: Evaluates a global variable and a flag to conditionally decrement a global variable and update another global variable with a constant.
- `update_sprite_attribute_lists`: Decrements counters and processes lists to copy attribute data from a source table to a destination buffer.
- `setup_blt_parameters`: Sets several hardware registers for bitblock transfers and conditionally updates memory locations with calculated values based on bitmask checks.
- `configure_blt_hardware`: Calculates dimensions and parameters based on global values and updates several Blitter registers and DMA control registers.
- `initialize_system_parameters`: Sets multiple global variables to specific constants and populates function pointers in memory.
- `initialize_system_state`: Initializes system parameters, clears a memory block, and populates a data structure.
- `execute_subroutine_sequence`: Executes a series of nested and sequential calls to various other routines.
- `reset_global_values`: Sets a global variable to zero and another global variable to all ones.
- `check_global_condition`: Checks a global variable and calls another routine if the variable is not zero.
- `initialize_memory_buffers`: Fills memory areas with specific word values and performs bitwise OR operations on multiple memory locations.
- `decrement_and_initialize_structure`: Decrements a global variable and, if it reaches zero, populates a data structure with various values and pointers.
- `initialize_data_structure`: Decrements a global variable and, if it reaches zero, populates a data structure in memory with various values and pointers.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d0.
- `fill_memory_block`: Fills a memory area pointed to by a register with a specific word value for a fixed number of iterations.
- `fill_memory_loop`: Fills a memory area pointed to by a register with a word value until a counter reaches zero.
- `adjust_table_values`: Increments or decrements values at specific offsets within a table based on a comparison with a global variable.
- `clear_memory_and_reset`: Fills a specific memory range with ones and resets several global variables to zero or specific constants.
- `process_delayed_update`: Decrements a counter and, if non-zero, fetches data from a table to update global variables and calls another routine.
- `fetch_offset_data`: Retrieves a word from a table using an offset and updates several global variables and counters.
- `reset_delayed_state`: Sets a global variable and several counters to specific values if a condition is met.
- `set_global_constants`: Sets a global variable and a memory location to specific constant values.
- `update_global_state`: Decrements a global variable and, if it reaches zero, assigns it a new value, otherwise resets it to specific constants.
- `recursive_self_calls`: The routine calls itself once and then calls two other functions before returning.
- `return_no_op`: The recovered routine consists of a single return instruction and performs no work: it executes no computation, makes no library calls, touches no hardware registers, and has no referenced text.
- `search_null_terminated_list`: Iterates through a list of pointers, calling another routine for each entry until a null pointer is encountered.
- `return_constant_byte`: The routine returns a single byte with all bits set to one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_concatenated_registers`: Concatenates two 32-bit registers into a single 64-bit return value.
- `trigger_dma_task_processing`: Calls a routine to update DMA control registers and iterate through a task list for instruction execution.
- `call_audio_clear`: Calls a routine that clears audio buffers and modifies audio hardware registers.
- `update_dma_control_state`: Checks a global variable and, if non-zero, executes a loop to set specific bits in a register before updating a global variable and a secondary memo.
- `trigger_hardware_update`: Calls a routine that updates hardware registers and memory tables based on a flag and a linked list.
- `update_dma_configuration`: Updates hardware registers and memory tables by passing calculated indices and input values to a routine that configures DMA and Blitter parameters.
- `call_global_initialization`: Calls a routine that sets multiple global variables and memory locations to one.
- `update_linked_list_nodes`: This routine modifies a global variable based on a condition, clears audio memory, and iterates through a linked list to initialize several data structures using values from a table.
- `clear_audio_buffers`: Writes zero to multiple memory locations and modifies hardware registers related to audio volume and DMA control.
- `initialize_global_flags`: Sets several global variables and memory locations to a value of one.
- `set_dma_control_bits`: Checks a global variable and, if non-zero, performs a loop to set specific bits in a register and updates a global variable and another memory location.
- `update_dma_and_process_task_list`: Updates DMA control registers and iterates through a table of task structures to execute a sequence of instructions.
- `process_sprite_attribute_table`: Iterates through a task list to update sprite or object attributes, calculating scaled coordinate values and writing them to hardware-mapped memory or a data buffer.
- `process_task_list_and_update_dma`: Iterates through a table of task structures to update DMA control registers and perform calculations involving bitwise shifts and pointer arithmetic.
- `update_sprite_hardware_registers`: Iterates through a task list to calculate new sprite positions and attributes, then writes these values to the hardware DMA registers.
- `update_hardware_registers_and_task_list`: Iterates through a table of task structures to update hardware registers, specifically targeting DMA control and custom chip registers, while managing task counters and pointer increments.
- `clear_memory_byte`: The routine stores a zero byte into a memory location relative to a value in a register.
- `update_sprite_attributes`: Iterates through a task list to calculate and update sprite or object attribute data.
- `process_sprite_attribute_table_2`: Iterates through a task list to update sprite-related data structures, performing fixed-point arithmetic to calculate coordinate offsets and scaling values.
- `update_sprite_attributes_and_dma`: Iterates through a task list to update sprite coordinate and attribute data, then modifies hardware registers to synchronize DMA transfers.
- `update_dma_and_process_task_list_2`: Updates DMA control registers and iterates through a table of task structures to execute a sequence of instructions.
- `process_linked_list`: Iterates through a linked list of eight elements, performing arithmetic operations on data retrieved from each node and storing the results into a table.
- `update_hardware_and_tables`: Modifies a global variable based on a flag, iterates through a linked list to perform arithmetic, and updates hardware registers and memory tables.
- `update_sprite_attribute_tables`: Calculates offsets based on an index and writes sprite-related data, including position and attribute values, into both a memory table and hardware registers.
- `configure_dma_parameters`: Updates memory tables and hardware registers, including DMA and Blitter parameters, based on calculated indices and input values.
- `update_dma_blitter_parameters`: Updates a data structure containing DMA parameters and modifies hardware registers to control the Blitter.
- `update_bit_field`: Calculates an address using a pointer and a value from a table, then sets specific bits in that memory location.
- `modify_memory_via_pointer`: Calculates an address by adding a value from a table to a register and performs a bitwise OR operation on the byte at that location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nested_data`: This routine performs bitwise operations on a memory location, decrements a global variable, and executes a complex sequence of recursive calls and data processing routines.
- `empty_routine`: The routine contains no valid instructions and performs no operations.
- `add_ascii_digits`: Iterates through two memory buffers to perform a decimal addition of nibbles and stores the result as ASCII characters.
- `populate_attribute_tables`: Iterates through a source table to populate a destination buffer by interleaving word values with byte-sized components derived from a global variable.
- `copy_data_block`: Copies sixteen words from a source table to a destination buffer using a calculated offset.
- `clear_register_value`: Sets a specific global variable to zero.
- `clear_global_word`: Sets a specific 16-bit global variable to zero.
- `copy_structured_data`: Decrements a global counter and performs a nested loop to copy data from a table into a destination, applying a bitwise AND with a mask.
- `execute_nested_indirect_calls`: Performs multiple indirect function calls using pointers from a table and executes nested loops to process memory and update global variables.
- `reset_data_register`: Clears a data register by applying a logical OR with zero.
- `nullify_data_register`: Uses a logical OR instruction to set a data register to zero.
- `execute_indirect_call_3`: Calls a routine via a pointer from a table and updates a global variable based on the result.
- `copy_memory_block`: Copies a block of data from one memory location to another using a loop and then writes specific byte values to the destination.
- `traverse_nested_string_list`: Recursively iterates through a linked list of character pointers.
- `traverse_string_list`: Iterates through a nested structure of pointers and characters until a null terminator is reached.
- `traverse_nested_string_list_2`: Recursively iterates through a linked list of character pointers and copies data from a source table into global variables.
- `traverse_nested_string_list_minimal`: Recursively iterates through a linked list of character pointers and stores a pointer into a global variable.
- `traverse_nested_string_list_with_mask`: Recursively iterates through a linked list of character pointers and stores a routine pointer into a global variable.
- `traverse_nested_string_list_basic`: Recursively iterates through a linked list of character pointers.
- `traverse_nested_string_list_conditional`: Recursively iterates through a linked list of character pointers and conditionally executes a routine stored in a global variable.
- `traverse_nested_string_list_branching`: Recursively iterates through a linked list of character pointers with a conditional jump to a global variable based on a value in the list.
- `copy_buffer_data`: Calculates an offset based on an input value and copies data from a source table to several global variables.
- `apply_bitwise_mask`: Iterates through a table to perform bitwise OR operations on a destination buffer at specific intervals.
- `apply_bitmask_to_buffers`: Performs a bitwise OR operation between a byte from one buffer and several bytes in another buffer across multiple iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `bitwise_or_and_and_buffers`: Performs a bitwise OR on one buffer and a bitwise AND with the bitwise NOT of the source on three other buffers in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_or_to_offsets`: Performs a bitwise OR operation on two memory locations at specific offsets from a base address within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_and_to_offsets`: Performs a bitwise AND operation on two memory locations at specific offsets from a base address within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_or_mask`: Performs bitwise OR operations on two specific byte offsets within a memory block using a value from a register.
- `apply_nibble_pattern`: Reads values from a table and applies split nibbles to specific memory locations using bitwise OR operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_complex_logic_mask`: Iterates through memory buffers performing a sequence of bitwise AND and OR operations using values derived from shifting and masking registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitmask_to_buffer`: Iterates through a series of memory buffers, performing bitwise AND and OR operations on specific bytes using values derived from shifting and masking registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_rotated_or_mask`: Applies bitwise OR operations to a memory block using a 32-bit value that rotates its contents each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_interleaved_logic_mask`: Performs bitwise AND operations on several bytes and a bitwise OR on another byte within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_interleaved_logic_mask_2`: Performs bitwise AND operations on three bytes at a specific offset within a loop and a bitwise OR on a single byte at a different offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_repeated_or_mask`: Performs bitwise OR operations on four different byte offsets within a memory block during each loop iteration.
- `clear_memory_block`: Sets a series of bytes to zero starting from a specific memory address.
- `check_memory_null_terminator`: Checks a memory block for a null byte at eight-byte intervals for a limited number of iterations.
- `process_data_blocks`: Iterates through a series of eight-byte blocks, modifying values and calling another routine based on the contents of the current block.
- `update_state_block`: Checks a global variable and modifies its value or decrements a field within a structure based on specific conditions.
- `copy_and_process_table`: Copies a block of data from a source to a destination using a loop, then iterates through the copied data to perform conditional pointer-based updates and calls another routine based on the values encountered.
- `evaluate_state_and_dispatch_subroutines`: Checks a global variable against zero and various integer thresholds to determine a control flow path.
- `execute_sequence_of_subroutines`: A wrapper routine that conditionally invokes a series of subroutines based on a series of conditional checks and arithmetic operations on a global variable.
- `dispatch_state_dependent_subroutines`: Evaluates a global variable against multiple integer thresholds to determine a control flow path.
- `perform_arithmetic_operations`: Performs a series of additions, subtractions, and calls to another routine using a global variable.
- `execute_math_sequence`: Manipulates a global variable through various arithmetic operations and calls another routine multiple times.
- `execute_nested_calls`: Performs arithmetic on a global variable and makes several calls to another routine.
- `process_global_math`: Modifies a global variable using addition and subtraction and calls another routine.
- `copy_data_blocks`: Copies five words from two source tables into two separate destination memory areas using an offset.
- `decrement_and_check_counter`: Decrements a global variable and, if it reaches zero, resets it to a specific value and performs a conditional call based on a stored value.
- `copy_data_and_wait`: Copies data from several sources to a global variable, waits for a specific condition via a loop, and performs two subsequent routine calls.
- `initialize_global_state`: Sets numerous global variables and memory locations to specific constant values and performs a wrap-around check on a global counter.
- `wait_for_interrupt`: Increments a global variable, writes to a memory location, and loops until a specific bit is set in a memory-mapped register.
- `set_global_constant`: Stores a constant value into a global variable.
- `initialize_max_signed_integer`: Stores the maximum positive value for a signed sixteen-bit integer into a global variable.
- `update_conditional_value`: Compares a value against a global variable and, if the condition is met, updates a global variable with new data.
- `call_conditional_function`: Checks if a global variable does not equal a specific constant and then calls a function pointer stored at an offset from a global variable.
- `assign_and_call_routine`: Stores a constant value into a global variable and then calls another routine.
- `execute_indirect_call_2`: Stores zero into a global variable and then calls a routine via an address calculated from a pointer and a value in a table.
- `populate_data_structure`: Iterates through a loop to populate a structure in memory with values derived from other routines and global variables.
- `decrement_counter_variable`: Checks a global variable and a 16-bit value, potentially resetting the global variable before decrementing the 16-bit value.
- `calculate_offset_values`: Uses bit-shifted values from two global variables to perform arithmetic and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_hardware_bit`: Loops until a specific bit in a hardware register is cleared.
- `copy_and_configure_hardware`: Copies a block of data from a source table to a destination, then updates several hardware registers including display and DMA control.
