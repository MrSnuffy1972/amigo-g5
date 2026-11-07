# Game 1153 (double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01153). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (945008 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 191 of 191 named (0 author symbols), 191 of 191 described (2 by rule, 188 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2076 of 2076 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7030 of 7030 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12403 |
| code | 24732 |
| data | 902239 |
| padding | 2 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2)[cr SKR].adf:bootblock | 1 | 73 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6722 | 170 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 235 | 19 |

## Cases handled

- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)[cr skr]/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2)[cr SKR].adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 1998 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 78 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)[cr skr]/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2)[cr SKR].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)[cr skr]/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2)[cr SKR].adf: blocks 23-452 (bytes 11776-231936) hold a RNC1: Rob Northen RNC1 Compressor (old) stream (220160 -> 110296 bytes)
- 2 packed track region(s), 672256 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2)[cr SKR].adf:bootblock

- `initialize_system_boot`: Performs memory allocation, I/O operations, and system state changes before executing a series of indirect function calls.
- `open_graphics_set_copper`: Opens the graphics library through the exec library, clears 81 longwords at a fixed buffer, copies 30 longwords into a second buffer, and writes a copper list pointer and a DMA enable value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_entity_state_and_dialogue`: Updates entity spatial coordinates, bounding boxes, and linked list nodes while managing dialogue sequences and text branching.
- `manage_dma_transfer`: Resets a global variable, interacts with hardware via DMACON and BPLCON0, and enters a loop that waits for a specific byte in a structure to change.
- `process_text_sequence`: Checks a value in a table and, if it matches, performs a series of operations involving bitwise ORing buffer data and waiting for a flag.
- `handle_text_branch`: Checks if a table value equals seven and then executes a sequence of buffer data copies and wait loops.
- `check_end_condition`: Checks if a table value equals fourteen and then enters a loop that performs buffer operations and waits for a flag.
- `process_dialogue_state`: Checks a value in a structure to determine if it matches specific constants, then performs a series of data copies and bitwise operations before toggling a flag and recursively calling itself.
- `bitwise_or_buffer_data`: Performs a bitwise OR operation between bytes from a source table and a destination buffer at specific offsets.
- `copy_buffer_data`: Copies bytes from a source table into a destination buffer at specific offsets.
- `update_data_buffers`: Increments a pointer, copies a block of data to a new location, and fills a subsequent buffer with values from a table based on a conditional check.
- `update_buffer_and_table`: Copies a block of data to a new location and then fills a subsequent buffer with values from a table.
- `validate_entity_state_index`: Compares a value within a structure against a specific index or character constant.
- `update_structure_and_iterate_values`: Calculates a value using bitwise operations and addition to store in a structure member, then executes two nested loops that repeatedly call other routines while updating global variables with incrementing and decrementing values.
- `clear_flag_and_unlink_node`: Sets a flag bit in a status word in the A6-relative block, then clears it again at the end of the routine.
- `update_linked_list_and_decrement_counter`: Updates a doubly linked list by reassigning pointers and then decrements a value stored at a memory offset.
- `execute_nested_loops`: Performs multiple nested loops that call other routines while manipulating a bitmask in a global structure and decrementing a counter.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 8 into d7, then merges 3 into d2, then merges 5 into d4, then merges 6 into d3.
- `process_iterative_data`: Iterates through a sequence of values to call subroutines, updates a bitmask, and performs pointer swaps between memory locations.
- `update_linked_list_nodes`: Traverses and modifies a linked list by updating pointers and counters based on conditional checks of structure offsets.
- `process_complex_loop`: Performs a nested loop that checks bitmasks in a global structure and updates a linked list or decrements a counter based on specific conditions.
- `update_entity_spatial_state`: Updates a structure's coordinate and bounding box values by performing bitwise operations and arithmetic on various offsets.
- `process_structure_state`: Clears specific byte flags in a structure and performs calculations to update integer values at various offsets.
- `update_linked_list_node`: Reassigns pointers within a linked list structure and writes a constant value to a specific offset.
- `execute_blit_sequence`: Decrements a counter in a structure and, upon reaching a threshold, calculates memory offsets and prepares hardware registers for a transfer.
- `decrement_counter_and_execute_blit`: Decrements a byte in a structure and, if it is not negative, performs a series of calculations to populate memory locations and hardware registers before calling a bitplane transfer routine.
- `configure_blt_transfer`: Calculates memory offsets and populates hardware registers to prepare for a bitplane block transfer operation.
- `execute_blt_transfer_2`: Configures hardware registers for a block transfer operation, performs bitwise calculations to determine memory offsets, and manages a counter within a structure.
- `perform_graphics_transfer`: Configures memory addresses and hardware-related values to perform a data transfer, involving loops that wait for specific bit flags to clear.
- `prepare_blit_transfer`: Calculates memory offsets and configures hardware registers for a bitplane transfer operation.
- `setup_blit_parameters`: Updates structure members and prepares hardware registers for a bitplane transfer using calculated memory offsets and dimensions.
- `process_input_and_update_state`: Checks specific byte flags in a data structure, clears them if a condition is met, and performs pointer swaps within a linked list.
- `process_linked_list_updates`: Checks status flags in a structure, performs pointer swaps between linked list nodes, and updates coordinate or dimension values.
- `modify_list_pointers`: Updates the next and previous pointers of a linked list node and decrements a value in a global structure.
- `toggle_flag_and_update_pointer`: Toggles a bit in a global variable, updates a pointer, and modifies a value in a table using a bitwise operation.
- `handle_input_state`: Evaluates a character in a structure to toggle bits in a status word or increment a counter.
- `decrement_and_clamp_counter`: Decrements a value in a global variable and clamps it to zero if it becomes negative, provided a specific bit is not set.
- `decrement_value_conditionally`: Subtracts a value from a global variable if several conditions regarding other global variables are met.
- `increment_value_conditionally`: Adds a value to a global variable if the resulting sum is less than a specified limit stored in another global variable.
- `clear_and_wait`: Sets a byte in a global variable to zero and enters a loop that waits until that same byte is non-zero.
- `initialize_data_structure`: Initializes several global variables and copies data from a table into a structure, including a loop that processes elements from a pointer.
- `calculate_nested_offsets`: Uses nested loops to calculate and store memory offsets into a destination table based on values from a source table.
- `setup_blt_transfer`: Waits for a flag in a global variable to clear before configuring hardware registers for a block transfer operation.
- `configure_blt_parameters`: Waits for a flag in a global variable to clear and then writes specific values to hardware registers for a block transfer.
- `initialize_graphics_bitplane_tables`: Clears a memory block and populates structured tables with specific bit patterns, including alternating bit values and constant values.
- `initialize_graphics_buffer`: Clears a memory block with zeros and populates structured tables with specific bit patterns before performing a transfer.
- `initialize_graphics_buffer_tables`: Clears a large memory block and populates several structured tables with specific bit patterns, including alternating bit values and constant masks.
- `calculate_positional_offset`: Performs bitwise operations and arithmetic to update a global variable based on values from a table.
- `calculate_blt_offset`: Checks a value in a structure and, if non-zero, calculates a memory offset using a bitwise AND and a subtraction before calling other routines.
- `get_blt_parameters`: Returns a combined value from a structure if a specific field is zero, otherwise calls another routine.
- `calculate_difference`: Returns the difference between a provided value and a global variable, with specific handling for negative inputs.
- `populate_address_table`: Fills a table with incrementing values starting from zero with a fixed stride.
- `relocate_memory_block`: Uses a table of offsets to update pointers within a memory block, effectively relocating it.
- `initialize_list_elements`: Clears two fields in a structure and then iterates through a table to perform operations on multiple elements.
- `populate_structure_data`: Writes several values from registers into a structure at specific offsets and then calls another routine.
- `update_structure_member`: Stores a short value into a structure and calculates a new value for another member using a scaled index.
- `insert_into_linked_list`: Inserts a new structure into a linked list managed by a global pointer, updating neighbor references.
- `remove_from_linked_list`: Removes an element from a linked list and updates the list head or the next element's reference.
- `insert_sorted_into_list`: Traverses a linked list to insert a new node into a specific position based on a comparison value.
- `initialize_list_node`: Clears two fields in a structure and inserts it into a linked list pointed to by a global variable.
- `advance_linked_list_pointer`: Updates a pointer within a structure to point to the next element in a chain while clearing the current element's link field.
- `insert_into_conditional_list`: Inserts a structure into one of two different linked lists depending on the state of a global flag.
- `clear_linked_list`: Traverses a linked list starting from a pointer in a structure and clears the list head and a status byte.
- `calculate_blt_clipping_rect`: Calculates a clipped rectangular area for a bitplane transfer by comparing a source bounding box against a destination boundary.
- `prepare_blt_transfer_parameters`: Calculates memory offsets and hardware register values for a bitplane transfer by processing source rectangle dimensions and destination boundaries.
- `traverse_linked_list`: Iterates through two different linked lists stored in a global structure, calling another routine for each node and clearing the list head pointers upon completion.
- `execute_blt_transfer`: Calculates coordinates and dimensions for a hardware blitter transfer, configuring registers for a block move.
- `pack_bitfield_data`: Combines multiple short values and bitwise operations into a single 32-bit word.
- `initialize_entity_attribute_structures`: Initializes multiple memory structures by writing specific constant values and zeroing out corresponding offset fields.
- `clear_memory_block`: Fills a block of memory with zero values using a loop.
- `store_packed_values`: Calls a routine to pack bitfield data and stores the resulting word and its components into several global variables.
- `store_packed_values_alt`: Executes a bitfield packing routine and writes the returned values and additional components into a different set of global variables.
- `update_structure_fields`: Calls a routine to obtain a value and then writes that value and a secondary value into specific offsets of two different structures.
- `update_structure_fields_alt`: Obtains a value from another routine and stores it along with a second value into two separate memory structures at fixed offsets.
- `update_multiple_structures`: Performs two consecutive updates where a value is retrieved and written into the offset fields of two distinct memory structures.
- `update_multiple_structures_alt`: Repeats the process of retrieving values and writing them into the offset fields of two different memory structures twice.
- `execute_interleaved_copy`: Calls another routine three times in succession to perform interleaved data copying.
- `execute_interleaved_copy_alt`: Triggers a sequence of three calls to a routine that handles interleaved data transfers.
- `initialize_and_sparse_copy_data`: Clears a memory block, retrieves a value, and uses that value scaled by eight to offset a global variable for a sparse data copy operation.
- `initialize_memory_buffers`: Clears a block of memory with zeros and then copies sparse data from two source locations into a destination buffer.
- `copy_interleaved_data`: Reads two source values calculated from an offset and writes them into a destination buffer in an alternating pattern.
- `copy_sparse_data`: Copies specific bytes from two source locations into a destination buffer at regular intervals.
- `shift_data_blocks`: Calculates three offsets based on an input value and copies blocks of data from those locations to a global buffer.
- `reorder_data_blocks`: Uses an input value to determine three source positions and copies data from them into a global table.
- `duplicate_memory_range`: Copies a block of data from a global variable back onto itself using an incrementing pointer.
- `overwrite_global_data`: Performs a repetitive copy operation of data from a global variable to another location within the same global structure.
- `process_entity_attributes`: Iterates through nested loops to update entity attributes and manage a linked list based on specific bitmask checks and value comparisons.
- `update_attribute_buffer`: Calculates two indices based on the value of a register through iterative subtraction and uses them to fetch bytes from a table.
- `update_entity_attribute_table`: Calculates indices based on input values and performs a bulk transfer of byte data from a source table to multiple offsets within a target structure.
- `save_registers_and_update_interrupt`: Saves the current data and address registers to a memory block and updates a global variable.
- `save_register_state`: Stores the current contents of all data and address registers into a block of memory and sets a global variable to a constant value.
- `increment_and_set_flag`: Increments a global variable and stores a constant value of one into another global variable.
- `insert_into_linked_list_2`: Inserts a new node into a linked list by updating the pointers of the current node and the new node.
- `remove_from_linked_list_2`: Removes a node from a linked list and updates the surrounding pointers to maintain the chain.
- `populate_data_table`: Iterates through a series of memory locations to write a specific four-byte constant and zero out several other fields.
- `manage_linked_list_node`: Updates a linked list by traversing nodes to find a match, then reassigns pointers and updates a counter and a maximum value.
- `store_constant_value`: Stores a specific constant value into a memory location pointed to by a register.
- `set_table_values`: Sets several multi-byte values in a table based on the value of a specific byte.
- `update_state_on_change`: Checks if a byte matches a value in a global structure, updates it if not, and performs several conditional memory operations.
- `update_linked_list_nodes_2`: Traverses a list of structures to perform arithmetic on specific fields and reassigns pointers within a linked list based on bitmask conditions.
- `update_structure_offsets_32`: Uses nested loops to write packed 16-bit values into specific offsets of a structure pointed to by a secondary pointer.
- `update_structure_offsets_3a`: Performs nested iterations to store 16-bit values into structure offsets starting at a different base than the previous routine.
- `update_state_byte`: Updates a byte in a structure with a new value based on several conditional checks of other variables.
- `calculate_timer_values`: Calculates and stores multiple timer or duration values into a structure based on various state flags.
- `set_memory_bounds`: Sets two memory address values in a structure, potentially adding an offset to the second value.
- `perform_interleaved_copy_sequence`: Calls another routine three times in succession.
- `call_interleaved_copy_sequence`: This routine executes a sequence of three calls to a procedure that performs interleaved data transfers.
- `populate_data_array`: Fills a three-element array with values derived from global variables and specific offsets.
- `update_memory_pointers`: Calculates and stores three long values into a destination structure based on values and offsets from a source structure.
- `check_attribute_conditions`: Evaluates a byte and a short value from a structure to determine whether to call another routine.
- `update_function_pointers`: Clears a byte in a structure and assigns two different function addresses to specific offsets within that structure.
- `process_input_state`: Transforms a bitmask representing input states into a new bitmask using bitwise operations.
- `process_joystick_state`: Converts a raw input bitmask into a processed format by checking specific bits and applying bitwise logic.
- `fetch_byte_from_offset`: Returns a single byte from a memory location calculated by adding a fixed offset to a base address held in a register.
- `update_entity_state_and_logic`: Updates an entity's internal state, animation frames, and coordinate bounds based on status flags and specific control codes.
- `compare_short_values`: Compares a value pointed to by a register against a value stored in a global table plus a constant offset.
- `execute_conditional_branch`: Performs a comparison between a memory-resident short integer and a calculated threshold before calling another routine.
- `validate_entity_spatial_bounds`: Checks if an entity's coordinates fall within specific range boundaries or a predefined table of spatial zones.
- `update_entity_movement_state`: Checks entity status flags and compares position coordinates to determine if movement or state transitions are required.
- `update_entity_status`: Evaluates conditions based on structure members and global variables to update status flags and pointers.
- `check_collision_bounds`: Iterates through a linked list of structures to check if a coordinate falls within specific bounds defined by flags.
- `update_entity_status_and_position`: Checks entity status flags and coordinate bounds to update internal state, modify control bytes, and perform conditional pointer assignments.
- `update_entity_state_and_position_2`: Checks entity coordinate bounds and status flags to update internal state, modify control bytes, and perform conditional pointer assignments.
- `verify_entity_position`: Checks a linked list of structures to determine if a coordinate is within a range, potentially adjusting the value based on a global constant.
- `update_entity_state_and_position`: Checks an entity's status flags and updates its state, animation, and coordinate data based on movement or collision conditions.
- `process_collision_logic`: Performs arithmetic on structure members to check for equality and updates specific bytes and words based on comparison results.
- `update_collision_detection`: Checks for proximity between an entity and specific coordinate ranges or objects within a linked list.
- `update_entity_status_2`: Checks a byte flag in a structure and updates various fields, including a pointer to a global variable and a status byte, based on conditional comparisons and nested routine calls.
- `detect_overlap_range`: Checks if a coordinate is within a specific range defined by a linked list, and resets two bytes in a structure if a condition is met.
- `search_and_update_flags`: Iterates through a table to compare values and sets a specific bit in a byte within a structure based on a comparison result.
- `calculate_distance_metrics`: Evaluates several conditions within a structure to return either a masked byte value or the absolute difference between two memory values.
- `assign_callback_pointer`: Assigns either a routine address or a global variable address to a structure based on a byte value.
- `assign_callback_or_pointer`: Checks a byte at a specific offset within a structure to determine whether to store the address of another routine or a global variable into a pointer field at a different offset.
- `assign_global_pointer`: Stores the address of a global variable into a specific offset of a structure, regardless of the value at a preceding offset.
- `store_global_pointer`: Stores the address of a global variable into a pointer located at an offset within a structure.
- `initialize_structure_fields`: Sets multiple fields in a structure, including a byte value, a pointer to a global variable, and a constant value.
- `configure_structure_parameters`: Sets multiple fields in a structure, including bitwise flags, a global pointer, and specific memory range values.
- `set_status_bit`: Sets the third bit of a byte within a structure.
- `update_entity_state_and_bounds`: Updates an entity's status flags, state, and coordinate bounds based on specific bitmask checks and arithmetic comparisons.
- `compare_value_and_branch`: Compares a value in a structure against a calculated threshold and calls another routine based on the result.
- `execute_conditional_call`: Performs a single-instruction conditional check against a threshold and branches to another routine.
- `evaluate_threshold_condition`: Checks if a value in a structure is less than a value derived from a global offset and branches accordingly.
- `update_linked_list_coordinates`: Iterates through a linked list of structures to perform distance and boundary checks.
- `update_entity_collision_state`: Iterates through a linked list of structures to perform distance and boundary checks between entities.
- `validate_text_formatting_and_bounds`: Checks a character stream against specific control codes and structural constraints.
- `check_state_and_call`: Checks if a value in a structure matches one of several specific constants and if another field is less than one, then calls another routine.
- `compare_and_return_result`: Compares a structure value against a threshold and returns a concatenated 64-bit value.
- `update_active_object`: Modifies bits in a structure and copies data from that structure into a target table or a secondary buffer.
- `process_data_subtraction`: Modifies several fields in a structure, performs a subtraction between a value from a structure and a returned value, and updates a structure field with the result if it is not negative.
- `call_data_block_shifter`: Calls a routine that copies data blocks from three calculated offsets into a global buffer.
- `update_object_state`: Modifies several fields in a structure, including bitwise operations on a byte and updating a value based on a subtraction from a result returned by another routine.
- `update_object_state_alt`: Updates a structure by modifying specific bits in a byte and adjusting a short integer value based on the result of another routine.
- `update_state_flags`: Checks a byte at an offset and updates several values in a structure, including a pointer to a global variable and a counter.
- `modify_structure_bits`: Performs bitwise XOR and AND operations on a byte within a structure and conditionally assigns a pointer to a global variable.
- `set_status_bit_2`: Sets the third bit of a byte located at a specific offset within a structure.
- `process_linked_list`: Iterates through a list of elements, performing actions based on specific values, including calling a function pointer or updating a pointer from a table.
- `initialize_structure_loop`: Sets a byte to a specific value, performs a bitwise OR on a byte in a structure, clears two global values, and then executes a loop three times to call another routine with incrementing offsets.
- `link_structure_nodes`: Stores values into a structure and updates a global pointer to point to that structure, potentially linking it to another structure.
- `store_register_values`: Stores three values into a structure and a global memory location using provided registers.
- `unlink_structure_node`: Checks a global pointer, and if it is not zero, attempts to traverse a link within a structure to update the global pointer to the next node or clear it.
- `clear_structure_links`: Resets a field in a structure to zero and updates a global pointer using a value retrieved from that structure.
- `clear_structure_fields`: Resets two specific fields in a structure to zero and stores a value in a global memory location.
- `manage_entity_list`: Traverses a structure to manipulate linked list pointers and updates a counter or status bit based on specific bitwise checks and comparisons.
- `modify_structure_flags`: Checks a value in a structure and, if certain conditions are met, performs a series of operations and sets a specific bit in another structure member.
- `relink_linked_list_nodes`: Updates a doubly linked list by reassigning pointers between nodes and adjusting a counter.
- `update_linked_list_node_2`: Modifies a linked list by updating pointers and decrementing a counter within a structure.
- `process_linked_list_nodes`: Iterates through a list of values, decrementing a counter and updating pointers within a linked list structure based on specific memory comparisons.
- `relink_list_nodes`: Modifies bits in a structure, manipulates pointers in a linked list, and decrements a global counter.
- `relink_structure_nodes`: Modifies bits in a global value, manipulates pointers within a linked list structure to reassign connections, and decrements a global counter.
- `check_structure_flags`: Checks two specific byte flags in a structure and, if they meet certain criteria, performs a bitwise OR on a byte in another structure based on a condition.
- `update_state_counter`: Updates a state byte based on a series of conditional checks and increments it if two specific character flags are set.
- `execute_loop_sequence`: Performs a loop up to sixteen times, calling a sequence of functions with varying parameters and updating a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_loop`: Executes a loop that calls another routine multiple times, processing data based on a counter until a limit is reached.
- `copy_data_block`: Copies fifteen words from a source table into a destination starting at a global variable and updates three individual global variables.
- `advance_linked_node`: Moves a pointer to the next node in a linked list and clears a field in the current node.
- `insert_sorted_node`: Inserts a new node into a sorted linked list by traversing the list and updating pointers to maintain order.
- `poll_structure_status`: Enters a loop that polls bits in a global value and, upon finding a specific bit set, performs a linked list pointer reassignment and decrements a global counter.
- `initialize_dma_parameters`: Sets several global variables to specific values and enables DMA channels by writing to the DMACON register.
- `execute_recursive_logic`: Performs a series of conditional recursive calls based on the state of bits in a register.
- `execute_unreachable_code`: The routine consists of a series of recursive calls to itself that never reach a return instruction.
- `execute_conditional_recursion`: Performs recursive calls controlled by a logical evaluation of a bit state and the result of another routine.
- `perform_recursive_calls`: Executes a series of conditional recursive calls based on the state of a bit in a register.
- `evaluate_conditional_logic`: Performs a series of conditional checks based on the status of a flag and calls other routines if specific conditions are met.
- `compare_table_values`: Compares four values from a table against a value stored in a pointer to determine if a condition is met.
- `compare_table_offsets`: Checks if two specific values from a table are both less than a value pointed to by another register.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d1, then returns.
- `update_table_entry_2`: Calculates an offset into a table using an input value, stores a word from the table into a global variable, and writes a constant to the table.
- `update_table_entry`: Calculates an offset into a table, stores a value from that table into a global variable, and writes a constant to the table entry.
- `populate_table_structure`: Writes multiple values from registers and another table into a structured memory area and enables DMA channels.
- `execute_dma_call`: This routine calls another routine and interacts with the AUD0LCH hardware register.
- `execute_dma_routine`: A routine that touches the DMACON register and is called by one other routine.
