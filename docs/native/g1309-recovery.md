# Game 1309 (interphase (1990)(imageworks)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01309). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928864 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 117 raw-word lines |
| Functions named and described | pass, 231 of 231 named (2 author symbols), 231 of 231 described (7 by rule, 201 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2378 of 2378 (100.00%) |
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
| Interphase (1990)(ImageWorks).adf:intload | 1 | 45 | 3 |
| Interphase (1990)(ImageWorks).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4371 | 226 |
| Interphase (1990)(ImageWorks).adf:int (raw code file, load address unknown) | 1 | 13 | 1 |

## Cases handled

- runtime segment an address: 2378 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- interphase (1990)(imageworks)/Interphase (1990)(ImageWorks).adf:int: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 242578 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Interphase (1990)(ImageWorks).adf:intload

- `start`: Program entry that allocates memory, opens dos.library, clears a block of custom chip registers and sets the DMA and interrupt enable control registers.
- `call_saved_load_address`: Calls the routine whose address is held in a global variable set at startup, then returns.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.

### Interphase (1990)(ImageWorks).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_runtime_memory_tables_2`: Clears a range of memory and populates a series of tables with certain constant values and bitmasks.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `parse_command_string`: Iterates through a null-terminated string to execute a sequence of commands that modify global variables via pointer-based data, arithmetic, or conditional logic.
- `update_global_with_offset_logic`: Updates a global variable by adding or scaling values read from a memory pointer, where the logic or bitmasking applied depends on the least significant bit of the value at that pointer.
- `update_global_value`: Modifies a global variable by adding a value derived from a byte at an offset and then adding another byte from a different offset, applying a bitwise mask.
- `modify_global_with_clamping`: Performs bitwise masking on a global variable and executes a loop that applies a clamped difference between adjacent memory locations to a pointer-based value.
- `execute_complex_state_logic`: Performs a series of complex updates to a global variable, including bitwise masking and a loop that adjusts a pointer-based value by a clamped difference.
- `execute_recursive_logic`: Executes a sequence of function calls and conditional branches that increments a global variable and traverses a pointer-based list.
- `initialize_global_state`: Increments a global variable and calls itself recursively until it reaches zero, then resets a global variable and executes a loop that calls another routine until a null terminator is found.
- `traverse_null_terminated_string`: Iterates through a null-terminated string or buffer until a null byte is encountered.
- `subtract_constant_from_variable`: Subtracts a constant value from a global variable and stores the result back into that variable.
- `update_global_from_pointer_data`: Reads bytes from a pointer and updates a global variable based on certain bit flags and arithmetic or conditional checks.
- `clamp_accumulated_value`: Updates a global variable by adding or subtracting values from a pointer-referenced buffer, while clamping the result between certain bounds.
- `add_constant_to_variable`: Adds a constant value to a global variable.
- `add_constant_to_global`: Adds a constant value to a global variable and then enters an infinite loop.
- `random_below`: Returns a random number below the given limit.
- `multiply_and_store_variable`: Multiplies a value by two and stores the result into a global variable.
- `double_and_store_global`: multiplies a value by two and stores the result into a global variable.
- `decrement_and_call`: Decrements a local stack variable while calling other routines in a loop.
- `set_global_zero`: Sets a global variable to zero.
- `execute_nested_loop_with_global_reset`: Resets a global variable to a fixed value and executes a nested loop structure that calls other routines and modifies a global variable.
- `decrement_nested_loop_counters`: Decrements two nested loop counters while executing a sequence of subroutines and updating a global variable.
- `execute_nested_loops`: Executes a nested loop structure that calls other routines and modifies a global variable.
- `initialize_data_tables`: Clears a range of memory and populates a series of tables with or certain constant values and bitmasks.
- `initialize_runtime_memory_tables`: Clears a range of memory, copies a block of data, and resets several global pointers and counters.
- `evaluate_global_distance_threshold`: Calculates the absolute difference between a global variable and a value stored in a table, then checks if this difference exceeds a certain threshold.
- `insert_into_sorted_list`: Inserts a value into a descending-order list by decrementing a counter and shifting existing list elements or pointers backward until the correct insertion position is found.
- `shift_and_call_table`: Shifts several values within a table and then performs an indirect call to a function pointer.
- `traverse_pointer_list`: Iterates backward through a list of pointers, checking for null or matching values.
- `compare_table_value_to_threshold`: Calculates the absolute difference between a global variable and a value retrieved from a table, then checks if this difference exceeds a certain threshold.
- `update_table_entry`: Updates a table entry with a value and a pointer, potentially shifting existing table elements.
- `insert_list_element`: Shifts elements in a list or array backward to insert a new entry containing a pointer and a value.
- `update_list_node`: Calculates a new value based on a global variable and a table, then shifts elements in a list or array backward to insert a new entry containing a pointer and a value.
- `update_table_with_function_pointer`: Updates a table or array with a value and a function pointer, potentially using a conditional branch or a secondary routine.
- `insert_into_table`: Inserts a value and a pointer into a table while shifting existing entries backward.
- `shift_table_elements`: Shifts elements in a table backward to insert a new entry containing a value and a pointer.
- `insert_sorted_value`: Sets a global variable and then shifts elements in a list or array backward to insert a new entry containing a pointer and a value.
- `parse_string_commands`: Iterates through a null-terminated string and calls various subroutines or updates a global variable based on the characters encountered.
- `reset_and_load_global_variable`: Checks if a global variable is non-negative, resets it to zero, and then attempts to load a value from a memory offset into that variable if a certain condition is met.
- `sync_variable_with_register`: Compares a global variable to a register value and calls another routine if they do not match, then updates the global variable.
- `update_string_buffer_with_digits`: Updates a global memory location with a character representation of a value, while also writing fixed byte constants to other global memory locations.
- `write_ascii_char`: Stores an ASCII character or a digit calculated from a register into a memory location pointed to by a register.
- `store_ascii_char`: stores a byte calculated from a register and an offset into a memory location pointed to by a register.
- `do_nothing`: the routine performs a redundant load of a global variable and returns.
- `search_range_bounds`: Iterates through a list of four-byte blocks to check if a global variable falls within a range defined by certain memory values.
- `reset_global_variable`: Sets a global variable to zero and returns.
- `update_global_variable`: Sets a global variable to a constant and calls another routine if certain conditions are met.
- `update_global_constant`: checks if a global variable is certain values before calling another routine and then sets that global variable to a constant.
- `update_global_from_register`: compares a global variable to a register value and calls another routine if they differ, then updates the global variable.
- `traverse_string_buffer`: Sets a global variable and then iterates through a null-terminated buffer, calling another routine for each non-null byte.
- `update_clamped_bounds_2`: Checks a value in a structure and, depending on whether it meets certain criteria, calls another routine and stores a result into a structure offset.
- `tiny_moveq_add`: Small routine of 2 instructions that loads 244 into d6, then adds (a1) to d6.
- `compare_node_data`: Iterates through a list of nodes and checks if the absolute difference between certain values exceeds a threshold.
- `recursively_update_node_properties`: Iterates through a linked list of nodes to compare data thresholds and update global state or object properties.
- `modify_global_value`: Calculates a bit-shifted version of a global variable and updates that global variable with a new value or a fixed constant if certain conditions are met.
- `decrement_global_variable`: Decrements a global variable and resets it to a fixed value if it reaches zero or a certain threshold.
- `get_and_set_global`: returns the previous value of a global variable and sets it to a certain value if it was zero.
- `calculate_scaled_coordinates`: Performs complex arithmetic involving bit-shifting and masking a global variable, then stores calculated values into a series of global memory locations.
- `calculate_scaled_values`: calculates a value based on bit-shifted and masked versions of a global variable.
- `populate_table_and_execute`: Populates a table with calculated values, then traverses a list of pointers to execute other routines, and finally checks a global array to conditionally execute another routine.
- `process_table_indices`: Reads values from a table, updates several global variables with calculated addresses, and traverses a linked list or loop to perform indirect calls or further routine calls.
- `traverse_linked_table`: Checks a bitmask or index in a register to access a table of offsets, then recursively calls itself by following a chain of signed integer pointers stored in that table.
- `halt_execution`: the routine contains invalid or bad instruction data.
- `empty_routine`: the routine contains no executable instructions.
- `execute_indirect_call`: calculates a target address by adding a value from a pointer to a global variable and calls that address.
- `execute_indirect_call_2`: Calculates a target address by adding an offset from a global variable to a pointer and jumps to that address.
- `call_indirect_from_table`: Performs an indirect call to a routine using an address retrieved from a table offset by a global variable.
- `execute_indirect_table_call`: Executes another routine and then performs an indirect call to a target address calculated by adding an offset from a pointer-based memory access to a global variable.
- `execute_complex_indirect_call`: calls another routine and then executes an indirect call using a target address derived from a complex calculation involving a global variable.
- `update_coordinate_table`: Calculates and stores several orf-related or coordinate-related values into a series of global memory locations.
- `update_bitmask_in_table`: Iteratively calculates an offset and sets a bit in a table, then stores certain values into a structure.
- `update_structure_offset_v1`: Writes a constant value to a structure and adds a signed or unsigned constant to a field within that structure.
- `modify_table_value`: Updates a value in a table by adding a constant or a signed version of that constant to an existing value at an offset.
- `update_memory_offset_value`: stores a constant into a memory block and adds or subtracts a large constant to an existing value at a fixed offset from a pointer, depending on the state of a register.
- `update_memory_offset_value_2`: Writes a constant to a memory location and adds or subtracts a large constant to a value at an offset from a pointer, depending on the state of a register.
- `update_memory_offset_value_3`: stores a constant into a memory block and adds or subtracts a large constant to a value at a specific offset, depending on the state of a register.
- `update_structure_offset_v2`: Writes a different constant value to a structure and adds a signed or unsigned constant to a field within that or another structure.
- `update_structure_offset_with_constant`: Sets a byte in a structure to a constant value and adds a signed or unsigned large integer to a field within that same structure, depending on the state of a register.
- `update_bitmask_table`: An empty routine that performs no operations.
- `set_structure_field_constant`: Writes a constant value to a field in a structure and writes a constant to a different field in that structure.
- `set_structure_header_mask`: Sets a specific bitmask and a constant value into a structure header via a pointer.
- `return_no_op_stub_2`: The routine consists of a single return instruction and performs no work of its own.
- `initialize_memory_structure`: Writes a constant and a value from another routine into a memory block located at an offset from a pointer.
- `update_structure_offset`: Writes a constant to a structure and adds a value to a field within that structure.
- `initialize_structure_header`: Writes a constant value and a specific bitmask to a structure.
- `initialize_structure_fields`: Writes several values, including a constant and a value from a routine call, into a structure or block of memory located at an offset from a pointer.
- `initialize_global_structure`: Initializes a global memory structure by clearing a range of memory and setting certain fields.
- `traverse_linked_list_and_update_nodes`: Recursively traverses a linked list of data structures, checking certain value ranges and updating or inserting new nodes into the list.
- `update_list_elements`: Iterates through a list of pointers, writing certain register values and an incrementing counter to a memory offset.
- `execute_infinite_loop`: This routine implements an infinite loop that never exits.
- `transform_and_copy_global_data`: Calculates a new value for a global variable using bit-shifts and arithmetic, then copies a block of memory from a source pointer to a destination pointer if certain conditions are met.
- `traverse_table_and_reset`: Traverses a table and resets a global variable to zero while checking for non-zero values in a pointer-based list.
- `traverse_pointer_list_2`: Iterates through a list of pointers, skipping any that point to a zero value, and stops if a negative value is encountered.
- `execute_empty_routine`: The routine is empty and performs no operations.
- `execute_subroutines`: Executes two subroutines sequentially and continues in a loop if a boolean flag remains false.
- `traverse_linked_list`: Traverses a list of pointers, checking for a zero terminator and clearing a value at an offset if a zero is found.
- `scan_table_for_zero`: Scans a table of pointers and writes a zero to an offset if a zero-value or certain condition is encountered.
- `scan_entries_clear_flag`: Walks a table of 23-word records starting from a base pointer computed from a global index, stopping at the first record whose key is zero or negative.
- `scan_table_clear_flag`: Walks a table of 23-word records starting from a base offset derived from a pointer table indexed by the D0 input, stopping at the first record whose leading word is non-positive.
- `scan_list_for_value`: Scans a list of pointers and writes a zero to a distant offset if a specific value is detected.
- `clear_table_entry_on_match`: Traverses a table and sets a value to zero if a specific constant is encountered in the table.
- `traverse_linked_list_2`: Traverses a linked list of structures, calling another routine if a certain member of the structure is non-zero, and stops if a pointer or value becomes zero.
- `traverse_linked_list_3`: Iterates through a linked list by following pointers, calling a routine if a specific offset in the current node is non-zero.
- `check_and_update_object_proximity`: Calculates the absolute difference between coordinate or property fields of two objects and checks if they fall within a certain threshold.
- `update_bitmask_and_fields`: Updates a bit in a memory location using a bitmask derived from a register, and writes several values to a structure or block of memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_object_state_and_recurse`: Checks a bitmask in a table to determine a branching path, then updates a global variable and potentially calls itself or other routines to modify object properties or table-based flags.
- `test_and_branch_on_flag`: Loads a word from a global variable into a data register and performs a conditional branch if the value is zero.
- `traverse_linked_list_loop`: Iterates through a linked list, calling a routine pointer or recursing if a certain value in the list or a following offset is non-zero.
- `loop_indefinitely`: This routine contains an infinite loop.
- `check_coordinate_bounds`: Checks if a value in a structure exceeds a certain threshold and calls two other routines if it does.
- `load_scaled_index`: Calculates a value based on a structure member, clamps it, and loads a value from a table into a global variable.
- `load_conditional_table_value`: Loads a value from one of two possible tables into a global variable based on a condition, using a calculated index.
- `load_from_offset`: Loads a word from a table using an offset derived from a pointer into a global variable.
- `load_masked_offset`: Loads a word from a table into a global variable using a masked and offset pointer.
- `transform_vector_data_2`: Performs a 3D-to-2D projection or coordinate transformation by calculating a weighted sum of a 3-element vector using a transformation matrix, then stores the resulting coordinates into a destination buffer.
- `transform_vector_coordinates`: Performs a weighted sum of three values multiplied by a constant and stores the result into a destination buffer, then copies certain values from a structure.
- `transform_vector_coordinates_3`: Performs a fixed-point matrix-vector multiplication or weighted sum on a set of three-component vectors and stores the result in a destination buffer.
- `transform_vector_coordinates_duplicate`: Executes the same logic as the previous routine, calculating a weighted sum of vector components and writing the result to a memory buffer.
- `store_multiple_globals_2`: Stores several global or fixed-address values and values from atable into a sequence of global variables.
- `store_vector_components`: Stores several values from a pointer and a global variable into a series of fixed memory addresses.
- `update_vector_with_negation`: Updates several fixed memory addresses with values from a pointer and potentially negates those values if a certain condition is met.
- `sync_vector_direction`: copies vector components from a pointer-based structure to a set of global variables and conditionally negates them if a threshold-checking value exceeds a certain limit.
- `negate_vector_components_on_threshold`: Checks a value against a threshold and, if exceeded, writes the negation of certain vector components or table-derived values into a block of global memory, otherwise writes the original values.
- `negate_vector_components`: Checks a value against a threshold and, if exceeded, negates a set of vector components or table-derived values before storing them into a block of global memory.
- `sync_vector_components_2`: Copies values from a source pointer into a series of global variables and potentially negates those values based on a threshold check.
- `update_vector_components_negate`: Loads values from a pointer and stores them in global variables, potentially negating certain values if a certain threshold is exceeded.
- `update_vector_components_with_negation`: Loads a set of vector or table-derived values into a block of global memory, negating the values if a threshold check on a pointer-referenced value fails.
- `negate_vector_components_on_threshold_2`: Checks a value against a threshold and, if exceeded, writes a series of negated or copied vector components or table-derived values into a block of global memory.
- `update_vector_components_unreached`: This routine is unreached and contains a single instruction that decodes as code but is not part of the active control flow.
- `update_transformed_vertex_bounds`: Calculates new vertex or bounding box coordinates using fixed-point multiplication and bit-shifting, then stores the resulting or mirrored coordinates into a global structure.
- `store_vector_components_2`: Stores several global or fixed-address values and then loads new values from a pointer into a global variable and other fixed addresses.
- `update_pointer_offset`: Increments a value from a table and updates a pointer if a certain memory location contains zero.
- `transform_vector_coordinates_2`: Performs a fixed-point matrix-vector multiplication or weighted sum on a set of three-component vectors.
- `transform_vector_data_recursive`: Performs a weighted sum or matrix-like multiplication using values from a table and stores the result in a destination buffer, while also calling another routine.
- `rotate_vector_data_2`: Calculates new vector components using a combination of addition and subtraction, then stores the result in a buffer, if a certain memory location is zero.
- `do_nothing_2`: the routine is empty and performs no operations before returning.
- `sync_vector_components_3`: Copies or negates a set of vector components or table-derived values into a block of global memory based on a threshold check.
- `sync_vector_components`: Copies or negates a set of vector components or or table-derived values into a block of global memory based on a threshold check.
- `update_vector_components`: Updates several global or fixed-address values with values from a pointer and potentially negates them if a certain value exceeds a threshold.
- `store_multiple_globals`: Stores several global or fixed-address values and values from a table into a sequence of global variables.
- `transform_vector_data`: Performs a weighted sum or matrix-like multiplication using values from a table and stores the result in a destination buffer.
- `rotate_vector_data`: Calculates new vector components using a combination of addition and subtraction, then stores the result in a buffer.
- `transform_coordinate_table`: Performs a fixed-point-like rotation or scaling transformation on a sequence of three coordinate pairs or vectors.
- `check_bounds_and_call`: Checks if certain values in a table are within a certain range and calls a routine if certain conditions are met.
- `load_table_value`: Calculates an index based on a value from a structure and loads a value from a table into a global variable.
- `load_from_table`: Loads a value from a table into a global variable using an offset derived from a pointer.
- `execute_infinite_loop_2`: the routine implements an infinite loop that traps execution without performing any operations or modifying any registers.
- `update_structure_vector_and_bounds_2`: Decrements a value in a structure and, if non-zero, accumulates scaled values into other structure members, otherwise negates certain members and sets a value to a constant.
- `update_list_node_values`: Decrements a value in a structure and, if not zero, updates certain fields using a multiplication-based calculation.
- `update_node_with_clamping`: Inverts a bit in a structure and updates several fields using a calculation involving a global variable, applying clamping or certain bounds checks.
- `update_list_node_fixed_scale`: Decrements a value in a structure and, if not zero, updates certain fields using a fixed-scale multiplication.
- `update_structure_members`: Decrements a value in a structure and, if not zero, updates several structure members using pointer-based offsets and arithmetic.
- `update_node_values_2`: Calculates a weighted or scaled delta using a combination of global or pointer-based values and applies it to a node's coordinate or property fields.
- `update_structure_vector_and_bounds_3`: Calculates a new vector component using a weighted sum of existing structure values and a global variable, then applies a scaled reduction to update the structure's vector and bounds.
- `update_vector_attenuation`: Calculates a weighted reduction factor based on the distance between a target coordinate and a reference point, then applies this factor to update a 3D-like vector or position.
- `attenuate_object_position`: Calculates a weighted reduction factor based on the distance between an object's coordinates and a reference point, then updates the object's position or a related index using this attenuation factor.
- `update_node_counter_and_reset`: Increments a counter in a structure and, if a certain value is reached, resets certain fields and a global variable.
- `update_node_values_complex`: Calculates a difference between two structure fields and updates a field with a masked or scaled value, or updates a structure field using a value from a table.
- `traverse_and_update_node_bounds`: Traverses a list of pointers and updates certain fields in a structure if certain range conditions or distance checks are met.
- `increment_list_counter_2`: Increments a value in a structure and resets it to zero if a certain limit is reached, while also resetting a global variable.
- `return_empty_routine`: the routine is empty and performs no operations.
- `return_empty_routine_2`: The routine performs no operations and immediately returns to the caller.
- `return_empty_subroutine`: the routine performs no operations and immediately returns.
- `execute_empty_routine_2`: the routine performs no operations and returns immediately.
- `return_no_operation_stub`: The routine consists of a single return instruction and performs no work of its own.
- `return_no_op_stub_3`: The routine consists solely of a return instruction, performing no work of its own.
- `return_no_op_stub_5`: The routine consists of a single return instruction and performs no work.
- `return_without_action_3`: The routine consists of a single return instruction with no body, so it performs no work of its own.
- `return_empty_routine_3`: The routine or its code path is empty.
- `return_without_action`: The routine body is a single return instruction with no operations performed, so it has no effect on state or hardware.
- `return_no_operation`: The routine consists of a single return instruction and performs no work.
- `jump_to_update_transformed_geometry_bo`: Jump stub: passes control straight on to update_transformed_geometry_bounds.
- `nop_operation`: the routine is empty and performs no action.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work.
- `return_no_op_stub_4`: The routine consists of a single return instruction and performs no work.
- `return_no_op_stub_6`: The routine consists of a single return instruction and performs no work.
- `return_empty_routine_4`: This routine is empty and performs no operations before returning.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `return_without_action_2`: The routine consists of a single return instruction with no body; it performs no computation, stores no values and touches no hardware registers.
- `return_no_operation_stub_3`: The routine consists of a single return instruction with no body, so it performs no work of its own.
- `return_empty_routine_5`: The routine performs no operations and immediately returns to its caller.
- `return_no_operation_stub_2`: The routine consists of a single return instruction and performs no work.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work of its own.
- `update_structure_bounds`: Modifies a value in a structure by a fixed amount based on a flag, then clamps the absolute value and potentially flips a flag if a limit is exceeded.
- `return_immediately_stub_2`: The routine consists of a single return instruction with no body, so it performs no work of its own.
- `update_transformed_geometry_bounds`: Calculates new vertex or coordinate positions using a transformation matrix and checks if the resulting coordinates fall within a valid or visible range.
- `populate_node_data_2`: Checks if a global variable and values from a structure fall within a certain range, then writes scaled or modified values and certain constants to a destination structure.
- `void_routine`: The routine is empty and performs no operations before returning.
- `apply_weighted_vector_attenuation`: Calculates a weighted average or reduction factor based on the distance between a structure's coordinates and a global reference, then applies this scaled factor to update the structure's position or vector components.
- `update_structure_vector_and_bounds`: Calculates a new vector component using a weighted sum of existing structure values and a global variable, then applies a scaled reduction to the structure's coordinate or velocity members.
- `mask_pointer_bits`: Adds a constant to a value at a pointer offset and then applies a bitwise mask to that value.
- `increment_list_counter`: Increments a global counter and, if a certain limit is reached, resets it to zero.
- `initialize_list_node`: If a global variable is zero, it traverses a linked list and initializes a new node with several or certain values and a pointer to another structure.
- `update_pointer_offset_2`: Inverts a bit in a memory location and then updates several global or pointer-based values using a calculation involving a global variable and a clamped range check.
- `invert_pointer_value`: Inverts the least significant bits of a value stored at a pointer offset and updates a global pointer-based calculation.
- `update_clamped_bounds`: Calculates a new position or size based on a pointer-based offset and updates a set of three values in a structure, applying certain range limits.
- `decrement_and_update_structure`: Decrements a value in a structure and, if not zero, updates several structure members using a combination of pointer-based offsets and a constant multiplier.
- `tiny_clr_addq_rts`: Small routine of 3 instructions that clears (a1), then adds 4 to a7, then returns.
- `populate_node_data`: Checks if certain values relative to a global variable fall within a specific range, then populates a node with scaled versions of those values and a conditional constant.
- `update_node_values_3`: Checks if a difference between two values in a structure falls within a certain range and updates a field with a masked or scaled value.
- `mask_and_store_value`: Calculates a bitmask of a value minus a constant and stores the result into a structure offset.
- `update_node_values`: Multiplies a value by a constant and adds it to a global variable, then conditionally updates a field in a node based on a bitmask.
- `transform_structure_coordinates`: Performs a complex coordinate or vector transformation using values from a table and a structure, then updates the structure or modifies a pointer-based offset.
- `populate_node_data_3`: Populates a new node with data from a table and a global variable, then sets a value in the node based on a signed comparison.
- `execute_indirect_call_3`: Performs an indirect call to a routine using an address calculated by adding a global variable to another routine's address.
- `process_large_value_range`: Performs complex arithmetic and conditional checks on a 64-bit value and a 32-bit value, potentially updating a global variable or calling another routine.
- `transform_table_pointers_2`: Iterates through a table of pointers, retrieving values from secondary tables and applying bitwise scaling and bit-reversal-like transformations.
- `transform_pointer_range_complex`: Performs a series of shifts and assignments to several global variables based on a constant or a global variable, including conditional logic.
- `transform_pointer_range`: Iteratively modifies a 64-bit value and a secondary value by adding or subtracting a global variable, while checking certain bounds and calling another routine.
- `adjust_pointer_range`: Iteratively modifies a set of pointers or values by adding or subtracting a global variable, while checking for certain bounds or sign changes.
- `calculate_font_glyph_bounds`: Iterates through a range of values to calculate and store character or glyph-related scaling or positioning data into a series of global memory locations.
- `transform_table_pointers_3`: Iterates through a table of pointers, retrieving values from a secondary table to perform bitwise scaling and bit-reversal-like transformations.
- `transform_table_pointers`: Iterates through a table of pointers and performs bitwise transformations and scaling on values retrieved from a secondary table.
- `iterate_and_transform_table_values`: Iterates through a table of pointers, performing a non-linear bitwise transformation and scaling on a value retrieved from a secondary table, then checks for sign-consistency and a zero-terminator or negative-index condition to continue or terminate the loop.
- `split_and_store_long`: Extracts high-order bits from a 64-bit value and a 32-bit value, then stores these pieces into a destination buffer using a loop.
- `store_split_long_value`: Splits a 64-bit value into several 16-bit or 32-bit pieces and stores them into a memory location using a pointer.
- `compare_and_filter_range`: Compares two short values and checks if the larger value falls outside a specific range or if the smaller value is below a certain threshold.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d7, then returns.
- `parse_string_with_commands`: Iterates through a null-terminated string, updating a global variable and calling different subroutines based on the character or certain escape sequences encountered.
- `process_table_lookups`: Calculates several values based on a register, stores them in a table, and then traverses a list of pointers, performing a lookup and a secondary routine call for each valid entry found.
- `traverse_linked_list_of_pointers`: Traverses a list of pointers, using a table-based lookup to find a new pointer and updating a register with a modified value.
- `initialize_global_buffer_copy`: Initializes a stack-based or local variable and copies a block of memory or a table of a fixed size, then resets a global variable to zero.
- `update_global_with_mask`: Updates a global variable using a bitwise mask and checks if a value at a certain offset equals zero.
- `process_string_with_decrement`: Iterates through a string, calling subroutines if certain characters are found, and decrements a global variable until a certain condition or a null terminator is met.
- `update_global_and_loop`: Updates a global variable and enters a loop that modifies a pointer and checks a global variable against a value.

### Interphase (1990)(ImageWorks).adf:int (raw code file, load address unknown)

- `copy_words_then_jump_to_code`: Copies a block of 16-bit words downward in memory from the top of a source range to a destination, then transfers control to a routine at a computed address.
