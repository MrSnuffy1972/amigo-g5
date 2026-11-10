# Game 2288 (star wars - the empire strikes back (1988)(domark)[cr qtx][m black cat li]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02288). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (963488 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 252 raw-word lines |
| Functions named and described | pass, 276 of 276 named (0 author symbols), 276 of 276 described (9 by rule, 259 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3784 of 3787 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5283 of 5735 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32871 |
| code | 21540 |
| data | 369421 |
| padding | 1032 |
| unused | 538624 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Echo | 2 | 117 | 2 |
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX][m Black Cat Li].adf:Empire | 2 | 21 | 3 |
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX][m Black Cat Li].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3745 | 176 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 619 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 473 | 26 |
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
- runtime segment an address: 2435 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 529 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 373 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 369 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-long-keys) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (8 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (5 bytes change between runtime segment an address (play-long-keys) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-long-keys) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-keys) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-long-keys)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address (play-long-keys) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- star wars - the empire strikes back (1988)(domark)[cr qtx][m black cat li]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX][m Black Cat Li].adf:Empire: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_dma_control_bits` (runtime segment an address): Checks a global variable and, if non-zero, performs a loop to set specific bits in a global variable and updates another memory location. Kind: hardware register initialization/timing check. Pass path: the branch where cRam00052c01 is non-zero (the code inside the if statement executes).
- `update_hardware_and_tables` (runtime segment an address): Modifies a global variable based on a flag, iterates through a linked list to perform arithmetic, and updates hardware registers and memory tables. Kind: track/sector validation and table update. Pass path: The routine updates internal hardware/table pointers (likely disk geometry or sector offsets) based on the result of `u5__sub_10FA`. A successful check allows the subsequent loop to correctly populate the sector/track table (`pcVar5`) with .
- `configure_dma_parameters` (runtime segment an address): Updates memory tables and hardware registers, including DMA and Blitter parameters, based on calculated indices and input values. Kind: disk hardware/DMA configuration for custom track reading. Pass path: the routine completes and the subsequent code continues to load the actual game data/assets.

## Functions


### Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in a table into a caller-supplied buffer when that buffer pointer is set, then calls through a routine pointer.

### Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX][m Black Cat Li].adf:Empire

- `enter_supervisor_and_write_dma`: Program entry of twelve instructions that calls the Exec SuperState routine and writes the DMA control and interrupt enable registers, calling one unnamed routine.
- `run_handler_from_data_pointer`: Seven-instruction routine reached through a handler pointer stored in data, and run during the reference run.
- `write_dma_control_handler`: Two-instruction handler reached through a pointer stored in data that writes the DMA control register, where a set top bit enables the channels.

### Star Wars - The Empire Strikes Back (1988)(Domark)[cr QTX][m Black Cat Li].adf:bootblock

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
- `clear_global_variable`: Sets a global variable to zero through multiple subtraction operations.
- `process_table_loop`: Performs arithmetic on a global variable and then iterates through a table of function pointers to execute them.
- `increment_and_call_list`: Increments a global variable and then executes a series of function pointers stored in a linked list.
- `execute_function_list`: Executes a series of function pointers by traversing a linked list.
- `execute_indexed_function_list`: Executes a series of function pointers from a list, using an offset to locate each function.
- `average_three_values`: Calculates the arithmetic mean of three pairs of values and stores the results in a table.
- `sum_three_values`: Adds three pairs of values together and stores the results in a table.
- `subtract_three_values`: Subtracts one value from another for three pairs and stores the results in a table.
- `negate_three_values`: Negates three values retrieved from a table and stores them in another location.
- `copy_long_word`: Copies a long word and a word from one memory location to another.
- `calculate_weighted_averages`: Populates several global variables and a table with values derived from complex arithmetic and bit shifts based on input data.
- `calculate_weighted_values`: Performs a series of arithmetic operations using three input values to populate multiple entries in a large table.
- `update_scaled_table`: Iterates through a list of values and uses a routine to calculate scaled offsets, which are then stored in a global table.
- `fill_scaled_buffer`: Iterates through a list of values to populate a memory range with scaled data retrieved from a table.
- `fill_buffer_with_scaled_data`: Clears a memory range and then populates it with values retrieved from a global table using scaled indices.
- `process_data_list`: Iterates through a list of values, performing conditional memory fetches and updates based on bitwise checks and byte table lookups.
- `traverse_conditional_table_2`: Iterates through a table and performs conditional memory updates using bitwise checks and values from a byte table.
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
- `double_global_value`: Multiplies a global variable by two by adding it to itself.
- `double_global_value_2`: Performs a left shift by one on a global variable via addition.
- `double_global_value_3`: Updates a global variable by adding its current value to itself.
- `double_global_value_4`: Calculates twice the value of a global variable and stores the result back.
- `double_global_value_5`: Doubles the contents of a global variable through addition.
- `double_global_value_6`: Adds a global variable to itself to achieve a multiplication by two.
- `double_global_value_7`: Stores the result of a global variable added to itself back into the same location.
- `calculate_checksum`: Doubles a global variable and returns a value derived from an XOR operation between its low byte and the high byte.
- `decrement_and_initialize_sequence`: Decrements a global counter and, if it reaches zero, updates a global variable using a value from a table indexed by the variable's lower bits, then sets several global variables to constant values before calling another routine.
- `initialize_pointer`: Sets two consecutive global values and passes the address of the first to another routine.
- `modify_memory_and_globals`: Performs a bitwise OR on a value at an offset from a pointer, sets a global variable, and then clears several other global values.
- `initialize_global_sequence`: Performs a bitwise OR on a memory location and then sets a sequence of global variables to specific constant values.
- `manage_countdown_state`: Decrements a global counter and executes a function via a pointer from a table when the counter reaches zero.
- `nop_routine`: The routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `compute_matrix_products`: Performs multiple 32-bit multiplications using values from tables at specific offsets and stores the results in several global variables.
- `calculate_complex_products`: Performs multiple 32-bit multiplications using values from two different tables and stores the high and low results into several global variables.
- `sum_and_double_triplets`: Sums three consecutive 16-bit values from a table and doubles the result, repeating this for three distinct triplets.
- `scale_global_values`: Multiplies three different global variables by specific factors and stores the results back into a table.
- `scale_vector_data`: Multiplies three consecutive 16-bit values in a source table by three different factors and stores the results in a destination table.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a nested pointer structure.
- `execute_indirect_call_3`: Performs an indirect call to a function address retrieved through multiple levels of pointer indirection.
- `update_and_call_offset_eight`: Adds a value to a memory location at an offset of eight and then performs an indirect call using a pointer from a table.
- `update_and_call_offset_six`: Adds a value to a memory location at an offset of six and then performs an indirect call using a pointer from a table.
- `update_and_call_offset_ten`: Adds a value to a memory location at an offset of ten and then performs an indirect call using a pointer from a table.
- `call_indirect_via_pointer`: Adds a value from a pointer to another location and performs an indirect call using a table offset.
- `decrement_counter_at_pointer`: Updates a structure by storing two values and decrementing a 16-bit integer at a specific offset within that structure.
- `update_and_call_indirect`: Decrements a value at a memory location, stores a pointer, and performs an indirect call using a table offset.
- `call_indirect_via_double_pointer`: Performs an indirect call using a value retrieved through multiple levels of pointer indirection.
- `call_indirect_via_offset`: Performs an indirect call using a value stored at an offset within a provided pointer.
- `call_indirect_sequence`: Performs an indirect call through a pointer and then performs another indirect call using a table offset.
- `modify_and_call_indirect`: Subtracts a value from a memory location, adds another value to a different offset, and performs an indirect call.
- `zero_memory_block`: Fills a block of 1400 bytes with zeros using a loop.
- `clear_memory_block_5`: Fills a block of memory with zeros using a loop.
- `set_register_bits`: Performs a series of bitwise OR operations to set specific bits in register d4 and then returns.
- `decrement_global_variable_2`: Decrements a single global variable by one.
- `decrement_global_variable`: Decrements a global variable by one.
- `process_bytecode_instruction_stream`: Iterates through a sequence of data, interpreting values as instructions to perform indirect calls, memory updates, or conditional branching.
- `check_memory_sequence`: Iterates through a memory block and returns if it encounters a zero value.
- `do_nothing`: This routine is empty and performs no operations.
- `calculate_scaled_offsets`: Performs division and modulo operations on two input values using a third value to calculate and return scaled offsets added to a global base address.
- `clamp_global_value`: Checks if a global variable is negative and sets it to zero if it is.
- `update_global_counter`: Increments a value at a specific memory location and modifies a global variable based on the result.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 176 into d0.
- `update_memory_sequence`: Decrements a value at a pointer and updates a global variable based on a memory sequence.
- `update_coordinate_state`: Calculates absolute differences between global values and updates several global variables and counters based on specific range checks.
- `check_bounds_and_set_flag`: Compares two calculated differences against a constant and, if within bounds, calls other routines and sets a global variable to a constant.
- `execute_list_loop`: Iterates through a list of pointers derived from a global variable and calls other routines for each entry.
- `iterate_and_execute_list`: Iterates through a list of pointers, calling various routines and updating memory based on a counter.
- `decrement_and_process_list`: Decrements a byte at a pointer, removes the element from a list if it reaches zero, and updates a counter.
- `set_boundary_bitmask`: Clears bit 0 of d2 and checks d0 and d1 against certain values stored in a global table, setting bits 1, 2, or 4 in d2 if certain range conditions are met, otherwise it returns.
- `apply_bitmask_to_planes_2`: Calculates a destination address and performs bitwise OR operations on multiple memory planes based on bitmask conditions.
- `apply_bitmask_to_planes`: Calculates a destination address using a table and bitwise shifts, then performs bitwise OR operations on four different memory planes.
- `execute_complex_logic`: Performs bitwise OR operations on memory buffers and executes conditional arithmetic on a global variable.
- `update_sprite_graphics`: Updates graphical data by performing bitwise OR operations on multiple memory buffers using a byte from a table.
- `apply_bitmask_to_memory`: Calculates a destination pointer using a value from a table and applies a bitwise OR operation with a byte from a global variable across several memory offsets.
- `apply_bitwise_or_to_buffers`: Performs a bitwise OR operation between a byte and multiple memory locations at large offsets within a loop.
- `or_memory_at_offsets`: Performs bitwise OR operations on a byte and several memory locations at specific offsets within a loop.
- `execute_blitter_operations`: Configures and executes multiple blitter transfer operations using hardware registers.
- `process_complex_logic`: Executes a complex series of conditional checks and arithmetic operations on a global variable.
- `configure_blitter_transfer_2`: Sets up and executes multiple Blitter operations by configuring hardware registers and source/destination addresses.
- `add_to_memory_locations`: Adds two values to two different longwords located at a specific memory address.
- `reset_global_variable`: Sets a global variable to zero.
- `clear_data_register`: Performs a bitwise OR operation with zero on a byte in a data register.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_data_byte`: Performs a logical OR operation on a data register with zero.
- `clear_memory_block_2`: Fills a block of memory with zeros using a loop that iterates thirty-nine times.
- `clear_memory_block_3`: Clears a memory area by writing zeros to word-aligned locations for thirty-eight iterations.
- `clear_memory_block_4`: Performs a word-wise zeroing of a memory range through forty-nine iterations.
- `clear_memory_range`: Zeros out a block of memory by writing words to a location through a loop that runs forty-nine times.
- `search_and_initialize_data`: Decrements a value in a structure and, if it reaches zero, performs a series of memory writes and calls another routine.
- `search_for_zero_word`: Iterates through memory in eight-byte increments up to nine times, returning if a zero word is encountered.
- `update_memory_buffer`: Iterates through a memory range to modify values based on a comparison and arithmetic scaling.
- `calculate_scaled_coordinate_offsets`: Performs bitwise manipulation and arithmetic scaling on a global value to update a memory buffer.
- `process_complex_scaling`: Performs arithmetic operations including division and modulo to calculate scaled offsets, updates memory buffers, and modifies global state through conditional logic.
- `calculate_offset_differences`: Performs arithmetic operations involving a table of values and input registers within a loop.
- `update_coordinate_state_table`: Updates a coordinate table by performing division and arithmetic operations within a loop.
- `calculate_coordinate_state`: Performs division and conditional checks to update global variables and coordinate data.
- `update_coordinate_data`: Sets a global variable and performs a sequence of calls to update coordinate tables and pointer data.
- `populate_coordinate_table`: Initializes a table of values and performs a loop to update memory through arithmetic operations.
- `update_coordinate_table`: Calculates and stores four signed values into a global variable by adding and subtracting values from a source table.
- `update_pointer_data`: Repeatedly calls another routine and stores the resulting values from two registers into a memory location pointed to by a register.
- `initialize_data_parameters`: Sets several global variables to specific constant values and calls a routine that copies and doubles data.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `apply_color_effects`: Manipulates global variables and hardware color registers through bitwise operations and conditional arithmetic.
- `update_color_registers`: Writes values to color registers and checks a global variable before potentially calling another routine.
- `copy_data_loop`: Copies data from one location to another by repeatedly calling another routine and storing the results in a loop.
- `copy_and_double_data`: Copies data from one location to another in a loop while performing arithmetic on the values being stored.
- `apply_bitmask_loop`: Sets a global variable to a specific value and executes a loop forty-nine times that performs bitwise operations on memory planes.
- `initialize_global_table`: Sets several global variables and memory locations to specific constant values.
- `update_color_and_counters`: Iterates through a table to compare values against globals, updates hardware color registers, and manages several decrementing counters.
- `execute_blt_operations`: Performs bitwise OR operations on a buffer and sets up hardware parameters for bitblock transfers.
- `clear_data_padding`: This routine performs a no-operation by applying a logical OR with zero to a data register.
- `clear_data_register_d0`: Performs a logical OR operation on a data register with zero.
- `clear_data_byte_2`: Sets the least significant byte of a data register to zero.
- `bitwise_or_buffer`: Uses a loop to perform bitwise OR operations on data segments from a global variable into a destination.
- `set_color_register`: Writes specific values to a global variable and modifies a hardware color register.
- `transform_table_entries_2`: Updates a table by storing masked bytes and raw words while checking a global variable.
- `update_global_value`: Performs arithmetic on a global variable and conditionally assigns specific constant values to it.
- `execute_sequence_of_calls`: Executes a fixed sequence of several other routines.
- `iterate_and_call`: Loops through a sequence of memory locations and calls another routine if a byte is non-zero.
- `iterate_data_table_with_offsets`: Iterates through a loop nine times, calling two subroutines and a function that uses a calculated pointer derived from a base address and an incrementing offset.
- `get_offset_value`: Returns a word from a memory location calculated by adding an offset and a register value to a base address.
- `populate_data_structure_2`: Iterates through a loop to populate a memory structure with values from a sequence and a global variable.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `process_table_data`: Iterates through a table of words, performing arithmetic updates on each entry using values from other routines.
- `transform_data_table`: Iterates through a table of 16-bit values, applying a mathematical offset to each entry and updating the table with the results.
- `transform_data_table_2`: Iterates through a table of sixteen-bit values, performing arithmetic operations on each entry using a global variable and constant offsets, then stores the modified results back into the table.
- `get_constant_value`: This routine is used as a constant operand in arithmetic operations within another routine.
- `clear_table_words_loop`: The routine walks a 32-word table with a counter that counts down to minus one, performing a sequence of reads, arithmetic, and writes on each word before stepping forward by two bytes.
- `process_table_offsets`: Iterates through a table of short integers, performing arithmetic updates on each entry using a global variable and constants.
- `update_memory_via_bitmask`: Checks various bits in a variable to determine which constant values to store at a calculated memory address.
- `transform_table_entries`: Iterates through a table in increments of two, storing masked high bytes and raw words from three consecutive routine calls into the table.
- `update_structure_fields`: Modifies two fields within a structure pointed to by a register, including a wrap-around check.
- `process_shifted_data`: Iterates through memory to perform bitwise shifts and additions, followed by a loop that performs conditional memory copies.
- `check_global_state`: Evaluates a global variable and a flag to conditionally update the global variable and a constant.
- `update_global_state_2`: Evaluates a global variable and a flag to conditionally decrement a global variable and update another global variable with a constant.
- `update_sprite_attribute_lists`: Decrements counters and processes lists to copy attribute data from a source table to a destination buffer.
- `setup_blt_parameters`: Sets several hardware registers for bitblock transfers and conditionally updates memory locations with calculated values based on bitmask checks.
- `configure_blt_hardware`: Calculates dimensions and parameters based on global values and updates several Blitter registers and DMA control registers.
- `initialize_system_parameters`: Sets multiple global variables to specific constants and populates function pointers in memory.
- `initialize_system_state`: Initializes system parameters, clears a memory block, and populates a data structure.
- `execute_subroutine_sequence`: Executes a series of nested and sequential calls to various other routines.
- `reset_global_values`: Sets a global variable to zero and another variable to all ones.
- `check_global_condition`: Checks a global variable and calls another routine if the variable is not zero.
- `initialize_memory_buffers`: Fills memory areas with specific word values and performs bitwise OR operations on multiple memory locations.
- `decrement_and_initialize_table`: Decrements a global variable and, if it reaches zero, performs a series of operations to populate a data structure in memory.
- `initialize_data_structure`: Decrements a global variable and, if it reaches zero, populates a data structure in memory with various values and pointers.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d0.
- `fill_memory_block`: Fills a memory area pointed to by a register with a specific word value for a fixed number of iterations.
- `fill_memory_loop`: Repeatedly stores a word value into memory at a location specified by a register until a counter reaches zero.
- `adjust_table_values`: Increments or decrements values at specific offsets within a table based on a comparison with a global variable.
- `clear_memory_and_reset`: Fills a specific memory range with ones and resets several global variables to zero or specific constants.
- `process_delayed_update`: Decrements a counter and, if non-zero, fetches data from a table to update global variables and calls another routine.
- `fetch_and_update_state`: Reads a word from an offset in memory and updates several global variables and counters if a specific condition is met.
- `reset_delayed_state`: Sets a global variable and several counters to specific values if a condition is met.
- `set_global_constants`: Sets a global variable and a memory location to specific constant values.
- `update_global_state`: Decrements a global variable and, if it reaches zero, assigns it a new value, otherwise resets it to specific constants.
- `recursive_self_calls`: The routine calls itself twice and then calls another routine before returning.
- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller, performing no computation, memory access, library call, or hardware register write.
- `search_null_terminated_list`: Iterates through a list of pointers, calling another routine for each entry until a null pointer is encountered.
- `return_constant`: returns the value all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_registers`: Returns the combined contents of two 32-bit registers.
- `trigger_sprite_update`: Calls a routine to iterate through a data table and update sprite positions and attributes.
- `invoke_audio_buffer_clear`: Calls a routine that clears audio memory and resets hardware registers for volume and DMA control.
- `update_dma_control_state`: Checks a global variable and, if non-zero, executes a loop to set specific bits in a global variable and updates a separate memory location.
- `return_combined_registers`: Calls another routine and returns the combined contents of two 32-bit registers.
- `configure_dma_parameters_2`: Updates memory tables and hardware registers based on calculated indices and input values.
- `initialize_global_state_2`: Sets multiple global variables and memory locations to a value of one.
- `update_linked_list_nodes`: This routine modifies a global variable based on a condition, clears audio memory, and iterates through a linked list to initialize several data structures using values from a table.
- `clear_audio_buffers`: Writes zero to multiple memory locations and modifies hardware registers related to audio volume and DMA control.
- `initialize_global_flags`: Sets several global variables and memory locations to a value of one.
- `set_dma_control_bits`: Checks a global variable and, if non-zero, performs a loop to set specific bits in a global variable and updates another memory location.
- `update_sprite_attribute_table`: Iterates through a table of data to update sprite positions and attributes.
- `process_sprite_animation_data`: Iterates through a data table to update sprite attributes, including position, scaling, and animation frame indices.
- `update_sprite_attribute_table_2`: Iterates through a data table to calculate and update sprite position and attribute values.
- `update_object_kinematics`: Iterates through a table of objects to update their position and velocity based on internal counters and scaling factors.
- `update_object_positions_and_attributes`: Iterates through a table of data to calculate new coordinates and attributes for objects, updating hardware registers and internal state based on movement and scaling factors.
- `clear_memory_byte`: This routine clears audio memory and stores a zero byte at a specific offset from a pointer.
- `update_sprite_hardware_attributes`: Iterates through a data table to calculate new sprite positions and attributes, then writes these values directly to the hardware registers.
- `update_sprite_attribute_table_3`: Iterates through a data table to calculate and write new coordinate and attribute values into a sprite attribute table.
- `update_sprite_hardware_attributes_2`: Iterates through a data table to calculate new sprite coordinates and attributes, then writes these values directly to the hardware registers.
- `update_sprite_hardware_attributes_3`: Iterates through a data table to calculate and write sprite position and attribute values.
- `process_linked_list`: Iterates through a linked list of eight elements, performing arithmetic operations on data retrieved from each node and storing the results into a table.
- `update_hardware_and_tables`: Modifies a global variable based on a flag, iterates through a linked list to perform arithmetic, and updates hardware registers and memory tables.
- `update_data_tables`: Modifies several memory tables and hardware-related registers by calculating values based on an index and copying data from a source buffer.
- `configure_dma_parameters`: Updates memory tables and hardware registers, including DMA and Blitter parameters, based on calculated indices and input values.
- `update_dma_blitter_parameters`: Updates a data structure containing DMA parameters and modifies hardware registers to control the Blitter.
- `update_bitmask_flag`: Calculates an address using a pointer and a value from a table, then performs a bitwise OR on the byte at that location.
- `modify_bitmask_via_pointer`: Fetches a value from a table using an offset, applies a bitwise OR operation with a constant, and writes the result back to that memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_nested_data`: This routine performs bitwise operations on a memory location, decrements a global variable, and executes a complex sequence of recursive calls and data processing routines.
- `empty_routine`: The routine contains no valid instructions and performs no operations.
- `add_ascii_digits`: Iterates through two memory buffers to perform a decimal addition of nibbles and stores the result as ASCII characters.
- `populate_attribute_tables`: Iterates through a source table to populate a destination buffer by copying words and masking them with byte-sized values derived from a global variable.
- `copy_data_block`: Copies sixteen words from a source table to a destination buffer using a calculated offset.
- `clear_global_byte`: Sets a specific global byte to zero.
- `clear_global_word`: Sets a specific 16-bit global variable to zero.
- `copy_structured_data`: Decrements a global counter and performs a nested loop to copy data from a table into a destination, applying a bitwise AND with a mask.
- `execute_nested_indirect_calls`: Performs multiple indirect function calls using pointers from a table and executes nested loops to process memory and update global variables.
- `execute_indirect_call_and_toggle_state`: Executes a function pointer retrieved from a table using a calculated offset, then performs a recursive sequence that toggles a global variable between two specific values.
- `clear_data_byte_3`: Performs a logical OR operation on a data register with zero.
- `call_indirect_function_2`: Calls a function via a pointer stored in a table and updates a global variable based on the result.
- `set_global_value`: Sets a global variable to a specific constant value.
- `copy_memory_block`: Copies a block of data from one memory location to another using a calculated count and then writes specific byte and word values to the destination.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `traverse_nested_string_list`: Recursively iterates through a linked list of pointers to strings.
- `traverse_string_list`: Iterates through a list of strings, calling another routine for each non-null character encountered.
- `traverse_nested_string_list_2`: Recursively iterates through a linked list of pointers to strings.
- `store_global_pointer`: Stores the address of a global variable into a specific memory location before traversing a nested string list.
- `assign_function_pointer`: Stores the address of another routine into a memory location before traversing a nested string list.
- `iterate_string_list`: Iterates through a list of pointers and processes strings by checking for specific bit patterns or null terminators.
- `execute_conditional_callback`: Traverses a nested string list and executes a function pointer stored in a global variable if a specific character check passes.
- `branch_on_string_data`: Traverses a nested string list but may jump to a global variable address based on the value found at a pointer.
- `copy_buffer_data`: Calculates an offset based on an input value and copies data from a source table to several global variables.
- `apply_bitmask_to_buffers`: Calculates a source address based on a character value and performs a bitwise OR operation on four separate memory buffers using a loop.
- `apply_bitmask_to_buffers_2`: Performs a bitwise OR operation between a byte from one buffer and several bytes in another buffer across multiple iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `bitwise_or_and_and_buffers`: Performs a bitwise OR on one buffer and a bitwise AND with the bitwise NOT of the source on three other buffers in a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_or_to_offsets`: Performs a bitwise OR operation on two memory locations at specific offsets from a base address within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_and_to_offsets`: Performs a bitwise AND operation on two memory locations at specific offsets from a base address within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitwise_or_mask`: Performs a bitwise OR operation on two bytes at specific offsets within a memory block using a loop.
- `apply_nibble_pattern`: Reads values from a table and applies split nibbles to specific memory locations using bitwise OR operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_mixed_logic_mask`: Iterates through a memory block applying bitwise AND and OR operations using values derived from 32-bit registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitmask_to_buffer`: Iterates through a series of memory buffers, performing bitwise AND and OR operations on specific bytes using values derived from shifting and masking registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_rotated_or_mask`: Applies bitwise OR operations to a memory block using a 32-bit value that rotates its contents each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_partial_logic_mask`: Performs bitwise AND and OR operations on specific bytes within a memory block during a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_interleaved_logic_mask`: Performs bitwise AND operations on several bytes and a bitwise OR on another byte within a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_repeated_or_mask`: Performs bitwise OR operations on four different byte offsets within a memory block during each loop iteration.
- `clear_memory_block`: Sets a series of bytes to zero starting from a specific memory address.
- `check_memory_null_terminator`: Checks a memory block for a null byte at eight-byte intervals for a limited number of iterations.
- `process_data_blocks`: Iterates through a series of eight-byte blocks, modifying values and calling another routine based on the contents of the current block.
- `update_state_logic`: Manages a state machine by updating a global variable and conditionally modifying a value at an offset within a data structure.
- `copy_and_process_table`: Copies a block of data from a source to a destination using a loop, then iterates through the copied data to perform conditional pointer-based updates and calls another routine based on the values encountered.
- `clear_memory_byte_2`: Performs a bitwise OR operation with zero on a byte located at the address held in a register.
- `dispatch_by_state_value`: Branches on the sign and value of its single input, returning immediately for zero and otherwise selecting one of several sequences of table-driven computations that each end by invoking the same helper routines.
- `execute_complex_arithmetic_chain`: Performs a series of conditional arithmetic operations on a global variable and executes multiple subroutines based on the resulting values.
- `perform_arithmetic_operations`: Performs a series of additions, subtractions, and calls to another routine using a global variable.
- `execute_math_sequence`: Manipulates a global variable through various arithmetic operations and calls another routine multiple times.
- `execute_nested_calls`: Performs arithmetic on a global variable and makes several calls to another routine.
- `process_global_math`: Modifies a global variable using addition and subtraction and calls another routine.
- `copy_data_blocks`: Copies five words from two source tables into two separate destination memory areas using an offset.
- `decrement_and_check_counter`: Decrements a global variable and, if it reaches zero, resets it to a specific value and performs a conditional call based on a stored value.
- `copy_data_and_wait`: Copies data from several sources to a global variable, waits for a specific condition via a loop, and performs two subsequent routine calls.
- `initialize_global_state`: Sets numerous global variables and memory locations to specific constant values and performs a wrap-around check on a global counter.
- `wait_for_interrupt`: Increments a global variable, writes to a memory location, and loops until a specific bit is set in a memory-mapped register.
- `set_constant_value`: Stores the value 32767 into a global variable.
- `set_max_signed_integer`: Stores the maximum positive value for a signed sixteen-bit integer into a global variable.
- `update_conditional_value`: Compares a value against a global variable and, if the condition is met, updates a global variable with new data.
- `call_conditional_function`: Checks if a global variable does not equal a specific constant and then calls a function pointer stored at an offset from a global variable.
- `assign_and_call_routine`: Stores a constant value into a global variable and then calls another routine.
- `execute_indirect_call_2`: Clears a global variable and then executes a routine via a pointer calculated from that variable and a constant offset.
- `populate_data_structure`: Iterates through a loop to populate a structure in memory with values derived from other routines and global variables.
- `decrement_counter_variable`: Checks a global variable and a 16-bit value, potentially resetting the global variable before decrementing the 16-bit value.
- `calculate_offset_values`: Uses bit-shifted values from two global variables to perform arithmetic and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_hardware_bit`: Enters a loop that waits for a specific bit in a global variable to be cleared.
- `copy_and_configure_hardware`: Copies a block of data from a source table to a destination, then updates several hardware registers including display and DMA control.
