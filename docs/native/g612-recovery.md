# Game 612 (Interphase): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00612). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928864 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 117 raw-word lines |
| Functions named and described | pass, 231 of 231 named (2 author symbols), 231 of 231 described (7 by rule, 215 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2380 of 2380 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 242578 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4439 of 4439 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7252 |
| code | 14380 |
| data | 268973 |
| padding | 1331 |
| unused | 636928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Interphase.adf:intload | 1 | 45 | 3 |
| Interphase.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4371 | 226 |
| Interphase.adf:int (raw code file, load address unknown) | 1 | 13 | 1 |

## Cases handled

- runtime segment an address: 2380 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Interphase/Interphase.adf:int: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 242578 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Interphase.adf:intload

- `start`: Program entry of twenty-seven instructions that allocates memory, opens dos.library, clears a block of custom chip registers, calls the loadfile routine, and then calls the code it loaded through a stored address.
- `call_loaded_entry_pointer`: Four-instruction routine, called by the program entry, that calls the code at an address stored in a global loader pointer and then returns.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.

### Interphase.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_runtime_environment`: Clears a memory range, initializes several global variables, and populates data tables through a series of nested routine calls.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `process_command_string_buffer`: Parses a null-terminated string to update global state through a series of conditional arithmetic operations, bitmask checks, and recursive function calls.
- `update_global_state_from_buffer`: Reads a bitmask from a memory buffer to set a global variable, then performs conditional arithmetic on subsequent bytes in the buffer to update that same global variable.
- `update_global_variable`: Modifies a global variable by performing arithmetic operations using values loaded from a memory buffer.
- `update_global_state_buffer`: Modifies a global variable through a series of arithmetic increments and bitwise masking operations.
- `execute_complex_state_machine`: Performs a series of conditional checks and mathematical updates on a global variable while calling various subroutines.
- `execute_state_loop`: Performs conditional function calls and iterates through a list of data until a null terminator is reached.
- `execute_recursive_sequence`: Increments a global variable and calls itself recursively until the variable reaches zero, then performs a loop based on a null terminator.
- `iterate_null_terminated_string`: Traverses a null-terminated string by calling another routine for each non-zero character.
- `subtract_constant_from_variable`: Subtracts a constant value from a global variable and stores the result back into that variable.
- `update_global_from_pointer_data`: Reads bytes from a pointer and updates a global variable based on certain bit flags and arithmetic or conditional checks.
- `clamp_accumulated_value`: Updates a global variable by adding or subtracting values from a pointer-referenced buffer, while clamping the result between certain bounds.
- `add_constant_to_variable`: Adds a constant value to a global variable.
- `add_constant_to_global`: Adds a constant value to a global variable and then enters an infinite loop.
- `random_below`: Returns a random number below the given limit.
- `multiply_and_store_variable`: Multiplies a value by two and stores the result into a global variable.
- `clear_memory_block_and_reset_state`: Returns immediately if the primary global pointer is zero; otherwise, when a status value is negative-checked and a derived value is below 256, it records a return-address marker, then zeroes a contiguous block of 32-bit words up to a computed bound, and invokes a sequence of helper routines including a recursive self-call and a multiply-and-store helper.
- `execute_recursive_loop`: Executes a loop twenty times that calls another routine and two instances of itself.
- `set_global_zero`: Sets a global variable to zero.
- `execute_nested_decrement_loops`: Sets a global variable to a specific value and then executes a nested loop structure that decrements two counters.
- `decrement_nested_loop_counters`: Decrements two nested loop counters while executing a sequence of subroutines and updating a global variable.
- `execute_nested_loops`: Executes a nested loop structure that calls other routines and modifies a global variable.
- `initialize_and_populate_data_tables`: Clears a memory range and copies data from one table to another using loops.
- `initialize_system_state`: Decrements a global counter and, upon reaching zero, performs a sequence of memory zeroing, data copying, and recursive calls to other routines.
- `update_coordinate_interpolation`: Calculates interpolated values by computing the difference between a global variable and two values stored in a table.
- `insert_into_sorted_list`: Inserts a value into a descending-order list by decrementing a counter and shifting existing list elements or pointers backward until the correct insertion position is found.
- `shift_and_call_table`: Shifts several values within a table and then performs an indirect call to a function pointer.
- `traverse_pointer_list`: Iterates through a list of pointers, comparing values and decrementing a counter until a null terminator or a specific condition is met.
- `update_linked_list_node_2`: Inserts a new entry into a linked list by shifting existing elements in a table to accommodate a new record.
- `insert_linked_list_node`: Inserts a new entry into a linked list by shifting existing elements in a table to accommodate a new record.
- `insert_into_sorted_list_4`: Inserts a new entry into a descending-order list by shifting existing elements within a table.
- `insert_into_sorted_list_3`: Calculates a value based on a global variable and a table, then inserts it into a descending-order list by shifting existing elements.
- `insert_into_list_with_logic`: Updates a table with new values and pointers based on a global variable and then performs a list insertion operation.
- `insert_into_sorted_list_2`: Inserts a value into a descending-order list by decrementing a counter and shifting existing list elements or pointers backward until the correct position is found.
- `insert_into_non_empty_list`: Checks if a global variable is non-zero before performing a sorted list insertion by shifting elements backward.
- `execute_recursive_logic`: Performs a series of function calls and then inserts a value into a descending-order list by shifting existing elements.
- `parse_command_string`: Parses a null-terminated string to execute various subroutines and update a global variable based on specific characters.
- `reset_and_load_global_pointer`: Resets a global variable to zero and attempts to load a new value from a table using an offset.
- `sync_variable_with_register`: Compares a global variable to a register value and calls another routine if they do not match, then updates the global variable.
- `format_timer_string`: Writes characters representing a numeric value into a memory location.
- `write_ascii_sequence`: Stores a sequence of ASCII characters, including a colon, a period, and a digit derived from a register, into memory locations.
- `store_ascii_char`: stores a byte calculated from a register and an offset into a memory location pointed to by a register.
- `do_nothing`: the routine performs a redundant load of a global variable and returns.
- `search_range_bounds`: Iterates through a list of four-byte blocks to check if a global variable falls within a range defined by certain memory values.
- `clear_global_variable`: Sets a global variable to zero and returns.
- `reset_global_variable`: Sets a global variable to zero and conditionally calls another routine based on its value.
- `conditional_execute_and_reset_state`: Checks if a global variable is greater than negative one and not equal to four to execute another routine.
- `update_global_from_register`: compares a global variable to a register value and calls another routine if they differ, then updates the global variable.
- `iterate_null_terminated_string_2`: Sets a global variable and then traverses a null-terminated string, calling another routine for each character encountered.
- `update_clamped_bounds_3`: Checks a value in a structure and, depending on whether it meets certain criteria, calls another routine and stores a result into a structure offset.
- `tiny_moveq_add`: Small routine of 2 instructions that loads 244 into d6, then adds (a1) to d6.
- `search_linked_list`: Traverses a linked list to compare values and returns a result based on matching criteria.
- `process_object_state_updates`: Iterates through a linked list to evaluate object properties and updates global state variables using bitwise operations and clamping logic.
- `reduce_and_clamp_global`: Calculates a value based on a global variable by performing a bitwise shift and subtraction, then resets the global variable to a specific constant if it reaches zero.
- `decrement_and_clamp_global`: Decrements a global variable by four and resets it to thirty-two if it falls below four or reaches zero.
- `get_and_set_global`: returns the previous value of a global variable and sets it to a certain value if it was zero.
- `calculate_scaled_coordinates`: Performs complex arithmetic involving bit-shifting and masking a global variable, then stores calculated values into a series of global memory locations.
- `calculate_scaled_values`: calculates a value based on bit-shifted and masked versions of a global variable.
- `process_indirect_tables`: Populates two tables with calculated values through a loop, performs nested indirect calls via a table, and conditionally executes another routine based on a table of bytes.
- `process_table_indices`: Reads values from a table, updates several global variables with calculated addresses, and traverses a linked list or loop to perform indirect calls or further routine calls.
- `traverse_linked_table`: Checks a bitmask or index in a register to access a table of offsets, then recursively calls itself by following a chain of signed integer pointers stored in that table.
- `empty_routine`: This routine is empty and contains no valid instructions.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `call_indirect_via_table`: calls a routine using an address retrieved from a table and an offset pointed to by a register.
- `execute_indirect_call`: Performs an indirect call to a routine by adding an offset found in a register to a global variable.
- `call_indirect_via_table_2`: Calculates an address by adding a value from a pointer to a global variable and calls the resulting address.
- `dispatch_indirect_call`: Calculates a target address by adding an offset from a pointer to a global variable and executes the resulting code.
- `call_complex_indirect`: calls another routine by calculating an address through multiple operations involving a global variable and a pointer.
- `update_coordinate_table`: Calculates and stores several coordinate-related values into a series of global memory locations.
- `update_bitmask_in_table`: Iteratively calculates an offset and sets a bit in a table, then stores certain values into a structure.
- `update_structure_offset_2`: Writes a constant to a structure and adds a signed value to a field within that structure.
- `modify_structure_value`: Adds a large constant or its negative counterpart to a value within a structure.
- `update_offset_value`: Adds or subtracts a large constant from a value stored at an offset within a memory structure.
- `perform_no_operation`: This routine is empty and consists of a sequence of no-operation instructions.
- `update_offset_value_2`: Adds or subtracts a large constant value to a longword located at an offset from a pointer.
- `update_structure_offset_field`: Sets a constant value at a specific offset within a structure and adds a signed integer to a different field within that same structure.
- `update_structure_offset_and_field`: Stores a constant value into a structure and adds a signed integer to a field within that same structure.
- `update_bitmask_table`: This routine calls another routine to iteratively calculate an offset, set a bit within a table, and store specific values into a structure.
- `set_structure_values`: Writes a large constant and a byte value into a structure.
- `initialize_structure_header`: Sets the first byte of a structure to a constant value and writes a large constant to an offset within that same structure.
- `call_bitmask_update`: This routine serves as a wrapper that invokes a procedure to iteratively calculate an offset, set a bit within a table, and store specific values into a structure.
- `nop_padding_sequence`: This routine is empty and consists of a sequence of no-operation instructions used as padding within the executable.
- `update_structure_offset`: Writes a constant to a structure and adds a value to a field within that structure.
- `initialize_structure_data`: Writes a constant and a large value into a structure.
- `initialize_structure_fields`: Writes several values, including a constant and values from routine calls, into a structure or block of memory located at an offset from a pointer.
- `process_linked_list_structures`: Iterates through a linked list of data structures to perform range checks and conditional updates.
- `traverse_and_update_linked_list`: Iterates through a linked list of data structures, performing conditional jumps based on value ranges and updating structure members.
- `populate_data_structure_list`: Iterates through a loop to write multiple values from registers into a series of memory structures until a threshold is met.
- `enter_infinite_loop`: An infinite loop that prevents further execution.
- `transform_and_copy_global_data`: Calculates a new value for a global variable using bit-shifts and arithmetic, then copies a block of memory from a source pointer to a destination pointer if certain conditions are met.
- `dispatch_indexed_state_handler`: Looks up a pointer from a table indexed by a global value scaled by four, clears that global twice, then walks a list of records in steps of 23 words, skipping zero entries and invoking the other routine when the entry is nonnegative; it returns the first negative-valued entry found.
- `traverse_pointer_list_2`: Iterates through a list of pointers, skipping any that point to a zero value, and stops if a negative value is encountered.
- `toggle_boolean_flag`: Sets a boolean value in memory to false and then performs another routine call.
- `execute_subroutines`: Executes two subroutines sequentially and continues in a loop if a boolean flag remains false.
- `find_free_list_slot`: Walks a linked array of fixed-size records of 23 words each, starting from a pointer selected by a global index, until it reaches a record whose first word is zero or negative.
- `scan_table_for_zero`: Scans a table of pointers and writes a zero to an offset if a zero-value or certain condition is encountered.
- `scan_list_clear_flag`: Walks a table of 46-byte records starting from a base pointer selected by a global index, stepping forward until a record with a non-positive first word is reached.
- `clear_entry_flag_on_match`: Walks a table of 23-word records starting from a base computed from D0, stopping when a record's marker word is non-positive; if the marker equals one of two specific values, it clears a word at a fixed offset within that record.
- `scan_list_for_value`: Scans a list of pointers and writes a zero to a distant offset if a specific value is detected.
- `clear_table_entry_on_match`: Traverses a table and sets a value to zero if a specific constant is encountered in the table.
- `traverse_linked_list`: Traverses a linked list of structures, calling another routine for each entry where a specific offset is non-zero.
- `traverse_linked_list_2`: Iterates through a linked list by following pointers, calling another routine if a specific offset in the current node is non-zero.
- `check_and_update_object_proximity`: Calculates the absolute difference between three coordinate or property fields of two objects and checks if they fall within a specific threshold.
- `update_bitmask_table_2`: Sets bits in a table based on values derived from another routine and stores several values into a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_object_state_and_recurse`: Checks a bitmask in a table to determine a branching path, then updates a global variable and potentially calls itself or other routines to modify object properties or table-based flags.
- `traverse_linked_list_recursive`: Iterates through a linked list, calling a routine pointer or recursing if a certain value in the list or a following offset is non-zero.
- `traverse_linked_list_recursive_2`: Iterates through a linked list, executing a function pointer found at the current node or recursing if a specific offset contains a non-zero value.
- `loop_infinitely`: The routine enters an infinite loop.
- `check_coordinate_bounds`: Checks if a value at a specific offset exceeds a threshold and performs several function calls and arithmetic operations.
- `fetch_table_word_2`: Calculates an index based on a value from a structure and loads a word from a table into a global variable.
- `select_table_entry`: Selects a table to read from based on a flag and then loads a word into a global variable using a calculated index.
- `fetch_offset_word`: Loads a word from a memory location determined by adding a constant offset to a pointer held in a register and stores it into a global variable.
- `fetch_indirect_word`: Loads a word from a memory location determined by adding an offset to a value retrieved from a pointer stored in a register.
- `transform_vector_table`: Iterates through a table of three-component vectors, applying a fixed-point matrix multiplication to each and storing the results in a destination buffer.
- `transform_vector_data`: Performs fixed-point matrix multiplication on three sets of three-component data from a table and stores the results in a destination.
- `transform_vector_array`: Iterates through a list of three-component vectors and applies a fixed-point matrix transformation to each.
- `transform_vector_set`: Iterates through a set of three-component data points, performing fixed-point matrix multiplication on each using values from a table and storing the results in a destination buffer.
- `populate_data_structure`: Copies a sequence of values from a global variable and a memory buffer into a series of consecutive memory locations.
- `store_transformed_values`: Stores several values from a source structure and a global variable into multiple memory locations.
- `store_conditional_negated_values`: Stores several values into memory, negating them if a specific value in a structure exceeds a threshold.
- `update_memory_with_negation`: Copies values from a structure to memory and applies negation to several entries if a comparison condition is met.
- `populate_triplet_vector_globals`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `update_triplet_vector_signs`: Loads a value and two offsets from a pointer into global variables and populates three sets of triplet data.
- `sync_vector_triplets_2`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `store_negated_values`: Stores several values from a source into memory, negating them if a specific value exceeds a threshold.
- `sync_triplet_vector_data_2`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `update_triplet_vector_data`: Copies a value and two associated offsets from a pointer into several global variables and conditionally negates some of them based on a comparison.
- `sync_triplet_vector_data_3`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `calculate_vertex_transform_parameters`: Calculates a set of geometric transformation coefficients by performing fixed-point multiplication and bit-shifting on input values.
- `copy_values_to_memory`: Copies several values from a source into a destination table.
- `update_pointer_offset`: Increments a value from a table and updates a pointer if a certain memory location contains zero.
- `transform_vector_table_3`: Performs fixed-point matrix multiplication on two sets of three-component data from a table and stores the results.
- `transform_vector_table_2`: Performs a fixed-point matrix multiplication on three sets of three-component data from a table and stores the results in a new destination.
- `transform_coordinate_triplet`: Performs a 2D rotation and scaling transformation on a triplet of coordinate pairs using values from a table.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `sync_triplet_vector_data`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `sync_vector_triplets`: Copies a value and two associated offsets from a pointer into several global variables in groups of three.
- `store_multiple_values_with_negation`: Stores several values from a source into memory, potentially negating them based on a comparison.
- `store_multiple_values`: Stores several values from a source and a global variable into a series of memory locations.
- `transform_three_vectors`: Performs a fixed-point matrix multiplication on three sets of three-component data and stores the results in a new table.
- `rotate_three_vectors`: Applies a 2D rotation transformation to three sets of two-component data using values from a table and stores the results.
- `rotate_vector_table`: Performs a 2D rotation on two sets of three-component data from a table using values from a source.
- `check_bounds_and_call`: Checks if values at specific offsets are within certain bounds and conditionally calls another routine.
- `fetch_table_word`: Calculates an index based on a value from a structure and loads a word from a table into a global variable.
- `load_word_from_offset`: Loads a word from a memory location calculated using a pointer and an offset into a global variable.
- `execute_infinite_loop`: The routine enters an endless loop that prevents further execution of the calling code.
- `update_interpolation_accumulator`: Decrements a counter in a structure and, if non-zero, accumulates scaled values into three integer fields.
- `update_linked_list_node`: Decrements a value in a structure and, if non-zero, updates several integer fields using a multiplier.
- `update_clamped_bounds_2`: Inverts a word in a structure and updates three integer fields using clamped calculations based on a global variable.
- `update_node_fields`: Decrements a value in a structure and, if non-zero, updates three integer fields using a constant multiplier.
- `update_structure_members`: Decrements a value in a structure and, if it is non-zero, performs arithmetic to update several fields using a table and other structure members.
- `update_interpolated_coordinates_2`: Calculates a weighted average of three values using a scaling factor derived from a table and a global variable, then updates a structure with the interpolated results.
- `update_interpolated_coordinates`: Calculates a weighted interpolation of structure members using a scaling factor derived from a dot-product-like operation.
- `interpolate_structure_state`: Calculates a weighted interpolation factor using a dot-product-like operation between a global vector and a structure's current state, then updates the structure's spatial members.
- `update_interpolated_vector`: Calculates a weighted interpolation factor based on the distance between a global vector and a target structure, then applies this factor to update the structure's coordinate components.
- `increment_state_counter`: Increments a counter in a structure and, upon reaching a specific value, initializes a new structure with several fields.
- `update_scaled_value_2`: Calculates a difference between a structure field and a global variable, applies a clamping range, and updates the structure.
- `traverse_and_update_pointer`: Traverses a list of pointers to find a value within a specific range and updates two short values in a structure.
- `manage_linked_list_node`: Increments a value within a structure and initializes a new node if a global variable is zero.
- `nop_placeholder`: This routine is empty and performs no operations.
- `execute_empty_subroutine`: This routine performs no operations and returns immediately to the caller.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `execute_empty_routine`: This routine is empty and performs no operations before returning.
- `execute_null_operation`: The routine is empty and performs no operations before returning.
- `execute_empty_routine_2`: This routine performs no operations and returns immediately upon entry.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that performs no operation, serving as a gap in the executable code.
- `return_immediately_2`: The routine is empty and performs no operations before returning to its caller.
- `execute_null_operation_2`: This routine is empty and performs no actions.
- `jump_to_update_transformed_geometry_bo`: Jump stub: passes control straight on to update_transformed_geometry_bounds.
- `return_from_routine`: The routine performs a return from a subroutine.
- `return_from_routine_2`: This routine executes a return instruction.
- `return_from_routine_3`: The routine exits by returning to the caller.
- `return_from_routine_4`: This routine is an empty return.
- `return_from_routine_5`: The routine ends by returning to the previous execution context.
- `return_from_routine_6`: This routine consists of a single return instruction.
- `return_from_routine_7`: The routine performs a return.
- `return_from_routine_8`: This routine executes a return to the caller.
- `return_from_routine_9`: The routine is a simple return.
- `return_from_routine_10`: This routine exits via a return instruction.
- `return_immediately_3`: The routine is empty and performs no operations.
- `return_immediately_4`: This routine contains no functional instructions.
- `return_immediately_5`: This is an empty routine.
- `update_coordinate_offset`: Adjusts a short value in a structure by a fixed amount based on the sign of a byte in the same structure.
- `return_immediately_6`: The routine performs no actions and returns.
- `update_transformed_geometry_bounds`: Calculates new vertex or coordinate positions using a transformation matrix and checks if the resulting coordinates fall within a valid or visible range.
- `create_scaled_node_2`: Checks if the difference between a global variable and values in a structure is within specific bounds, then initializes a new node with scaled and transformed coordinate data.
- `nop_padding_byte_2`: This routine is an empty instruction that serves as padding within the executable segment.
- `interpolate_and_update_structure`: Calculates a weighted interpolation of structure members using a scaling factor derived from a dot-product-like operation between a global variable and structure values.
- `update_interpolated_structure_coordinates`: Calculates a weighted interpolation of structure members using a global value and a scaling factor derived from a dot-product-like operation.
- `modify_offset_bits`: Adds a constant to a value at a specific offset and applies a bitwise mask to it.
- `initialize_linked_list_node`: Increments a value in a structure and, if a global variable is zero, initializes a new node with several values and a pointer.
- `initialize_list_node`: If a global variable is zero, it traverses a linked list and initializes a new node with several or certain values and a pointer to another structure.
- `invert_and_update_bounds_2`: Inverts a word at a specific offset in a structure and then updates three integer values within that structure using clamped calculations.
- `invert_and_update_bounds`: Inverts a word at a specific offset in a structure and then updates three integer values within that structure using clamped calculations based on a global variable and a pointer.
- `update_clamped_bounds`: Calculates a new position or size based on a pointer-based offset and updates a set of three values in a structure, applying certain range limits.
- `decrement_and_update_table`: Decrements a value in a structure and, if it is non-zero, performs arithmetic using a table and other structure members to update several fields; otherwise, it clears the first member of the structure.
- `tiny_clr_addq_rts`: Small routine of 3 instructions that clears (a1), then adds 4 to a7, then returns.
- `create_scaled_node`: Checks if differences between a global variable and values in a structure are within bounds, then initializes a new node with scaled versions of those differences.
- `update_scaled_value`: Calculates a difference between a value in a structure and a global variable, applies a clamping range, and updates the structure with a masked result.
- `mask_and_store_value`: Calculates a bitmask of a value minus a constant and stores the result into a structure offset.
- `calculate_scaled_values_2`: Performs arithmetic operations using a global variable and a table, then stores results into a structure.
- `transform_structure_coordinates`: Performs a complex coordinate or vector transformation using values from a table and a structure, then updates the structure or modifies a pointer-based offset.
- `populate_scaled_node_data`: Calls another routine and, if a zero flag is set, populates a structure with values from a global variable and a table.
- `execute_indirect_call_2`: Performs an indirect call using an address calculated by adding a global variable to a routine address.
- `process_large_integer`: Performs complex arithmetic and comparisons on a 64-bit value and updates a global variable.
- `process_pointer_table_2`: Iterates through a table of pointers, performing arithmetic and bitwise operations on values retrieved from memory based on specific conditions.
- `process_multiple_values`: Multiplies several values by a constant, performs comparisons, and shifts the results.
- `adjust_64bit_value_bounds`: Iteratively modifies a 64-bit value and a secondary value by adding or subtracting a global variable while checking against specific bounds.
- `adjust_pointer_range`: Iteratively modifies a set of pointers or values by adding or subtracting a global variable, while checking for certain bounds or sign changes.
- `calculate_affine_transform_coordinates`: Iteratively transforms a set of coordinates using a scale factor retrieved from a table.
- `modify_and_process_table`: Sets specific bits in two memory locations before iterating through a pointer table to perform calculations and bitwise transformations.
- `process_pointer_table`: Iterates through a table of pointers to perform calculations and bitwise transformations based on values retrieved from memory.
- `iterate_and_transform_table_values`: Iterates through a table of pointers, performing a non-linear bitwise transformation and scaling on a value retrieved from a secondary table, then checks for sign-consistency and a zero-terminator or negative-index condition to continue or terminate the loop.
- `store_split_64bit_value`: Splits a 64-bit value into 16-bit segments and stores them into memory via a pointer.
- `store_split_long_value`: Splits a 64-bit value into several 16-bit or 32-bit pieces and stores them into a memory location using a pointer.
- `compare_two_values`: Compares two values and determines the smaller of the two, then checks if that value falls within a specific range.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d7, then returns.
- `parse_string_with_commands`: Iterates through a null-terminated string, updating a global variable and calling different subroutines based on the character or certain escape sequences encountered.
- `process_linked_list_data`: Calculates several values based on an input, then traverses a linked list of pointers to perform operations on data retrieved from a table.
- `traverse_linked_list_of_pointers`: Traverses a list of pointers, using a table-based lookup to find a new pointer and updating a register with a modified value.
- `copy_memory_block`: Copies a block of data from a source address to a destination address using a loop.
- `update_global_pointer`: Modifies a global variable using bitwise operations and checks if the value at the resulting pointer is zero.
- `process_string_with_decrement`: Iterates through a string, calling subroutines if certain characters are found, and decrements a global variable until a certain condition or a null terminator is met.
- `update_global_and_loop`: Updates a global variable and enters a loop that modifies a pointer and checks a global variable against a value.

### Interphase.adf:int (raw code file, load address unknown)

- `initialize_unnamed_routine`: This routine consists of thirteen instructions and serves as the entry point for a raw code region.
