# Game 1152 (double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01152). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (945008 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 192 of 192 named (0 author symbols), 192 of 192 described (3 by rule, 187 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2076 of 2076 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7086 of 7086 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18087 |
| code | 24866 |
| data | 897083 |
| padding | 876 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf:bootblock | 1 | 35 | 1 |
| logged disk load from Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf boot-block load | 1 | 94 | 2 |
| logged disk load from Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6722 | 170 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 235 | 19 |

## Cases handled

- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: the boot block reads 1536 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: the boot block reads 6144 bytes from disk offset 230400 to an address (found statically); recovered as a unit
- runtime segment an address: 1998 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 78 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: 2 logged loads cover 7680 bytes of the image
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- double dragon 3 - the rosetta stone (1991)(virgin mastertronic)(disk 1 of 2)/Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf: blocks 23-449 (bytes 11776-230400) hold a RNC1: Rob Northen RNC1 Compressor (old) stream (218624 -> 110296 bytes)
- 2 packed track region(s), 669184 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf:bootblock

- `initialize_boot_environment`: Allocates memory and performs I/O operations to set up a jump table and execute a routine from a global pointer.

### logged disk load from Double Dragon 3 - The Rosetta Stone (1991)(Virgin Mastertronic)(Disk 1 of 2).adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_state_and_graphics`: Updates entity physics, linked list structures, and sprite attribute tables while managing dialogue text buffers.
- `wait_for_dma_completion`: Sets a global variable to zero, interacts with hardware registers, and enters loops that wait for a specific memory byte to change.
- `process_text_sequence`: Checks a value in a table and, if it matches a specific constant, executes a series of subroutines including bitwise graphics operations and a wait loop.
- `handle_text_event`: Checks if a table value equals a specific constant and then executes a sequence of subroutines including graphics copying and a wait loop.
- `handle_end_sequence`: Checks if a table value matches a specific constant and then executes a subroutine followed by a loop that calls another routine.
- `update_dialogue_text_buffer`: Checks a state variable to determine if a specific dialogue sequence should proceed, then calculates a memory offset and writes a sequence of bytes to a destination buffer.
- `bitwise_or_graphics_data`: Iterates through a source table to perform bitwise OR operations on specific offsets within a destination buffer.
- `copy_graphics_data`: Performs a block copy of bytes from a source table into a destination buffer at fixed intervals.
- `copy_and_fill_data`: Copies a block of data to a new memory location and then fills a subsequent memory range with values from a table.
- `update_buffer_and_table`: Copies a block of data to a specific memory location and then populates a secondary table with values from another source.
- `validate_state_sequence`: Compares a value in a structure against a series of expected constants and state flags.
- `update_object_and_loop_parameters`: Calculates a new value by bit-shifting and adding values from a structure, then enters two nested loops that repeatedly call other routines while incrementing and decrementing various local and global variables.
- `clear_data_register_byte`: Clears the lowest byte of the data register zero seven times in succession.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 1 into d0, then merges 3 into d2, then merges 5 into d4, then merges 6 into d3.
- `process_nested_loops`: Performs multiple nested loops that update structure fields, modify linked list nodes, and store constant values.
- `tiny_ori_ori_ori_ori_2`: Small routine of 4 instructions that merges 8 into d7, then merges 3 into d2, then merges 5 into d4, then merges 6 into d3.
- `execute_iterative_sequence`: Executes a series of loops that call linked list updates and constant storage routines.
- `update_entity_linked_list`: Updates a linked list of entities by modifying node pointers and adjusting a counter.
- `process_conditional_loop`: Performs a series of nested loops that check bitmasks in a global structure and update linked list pointers or decrement a counter based on specific conditions.
- `update_entity_physics_and_state`: Updates spatial coordinates and velocity vectors within a structure using bitwise operations and arithmetic.
- `process_structure_data`: Updates various fields within a structure, including swapping word values, performing bitwise shifts, and decrementing a counter.
- `relink_list_node_elements`: Updates the forward and backward pointers of a linked list node and its neighbors.
- `execute_blitter_sequence`: Decrements a counter and performs a hardware blitter transfer using calculated memory addresses and bitmasks.
- `decrement_and_blit`: Decrements a byte value in a structure and, if it remains non-negative, performs a hardware blitter transfer using calculated memory addresses and bitmasks.
- `configure_blitter_transfer`: Calculates memory addresses and bitmask values for a hardware block transfer operation while polling a status flag.
- `update_blt_state`: Calculates memory offsets and configures hardware registers for a block transfer operation while managing a counter and a conditional flag.
- `execute_blitter_transfer`: Configures hardware registers and memory addresses to perform a data transfer, including a loop that waits for specific flag changes.
- `perform_blitter_transfer`: Configures and executes a hardware blitter operation using source and destination addresses derived from a structure.
- `update_object_blitter`: Updates a structure with new values and performs a hardware blitter transfer using calculated source and destination addresses and bitmasks.
- `process_input_and_update_state`: Checks specific byte flags to trigger another routine and clears them if a condition is met.
- `process_structure_updates`: Checks flags within a structure to trigger other routines, manipulates linked list pointers, and updates various numerical values and offsets within a structure.
- `relink_list_node_elements_2`: Updates a linked list by reassigning pointers between a node and its neighbors.
- `update_hardware_and_pointers`: Toggles a bit in a global word, modifies a value using a bitwise shift, and updates a pointer through a linked list structure.
- `handle_state_commands`: Evaluates a character from a structure to perform various bitwise operations on flags or update linked list nodes.
- `decrement_and_clamp_counter`: Decrements a value in a global word and clamps it to zero if it becomes negative.
- `decrement_value_with_bounds`: Subtracts a value from a global variable if a specific byte is non-zero, a short value is not ten, is less than twelve, and the value is less than or equal to the variable.
- `increment_value_with_bounds`: Adds a value to a global variable if a specific byte is non-zero, a short value is not ten, is less than twelve, and the resulting sum is less than a second global variable.
- `clear_and_wait_for_zero`: Sets a byte in a global variable to zero and enters a loop that waits until that same byte is confirmed to be zero.
- `initialize_data_structure`: Initializes several fields in a global structure from a pointer, including conditional logic for specific short values, and then iterates through a list to copy data into another structure.
- `calculate_offset_table`: Uses nested loops to calculate and store a series of integer values into a table based on a base value and bit-shifted elements from a source list.
- `setup_blt_transfer`: Waits for a specific bit in a global variable to clear, then configures several hardware registers for a block transfer operation using values from the stack and a calculated offset.
- `configure_blt_parameters`: Waits for a specific bit in a global variable to clear, then writes four values to hardware registers to set up a block transfer.
- `initialize_sprite_attribute_tables_3`: Clears a memory block and populates multiple tables with specific bit patterns, including alternating bit values and constant masks.
- `initialize_sprite_data`: Clears a memory block, populates tables with specific patterns, and performs a series of blitter-related operations.
- `initialize_sprite_attribute_tables_2`: Clears a large memory block and populates multiple tables with specific bit patterns, including alternating bit values and constant masks.
- `calculate_position_offset`: If a global variable is non-zero, it performs bitwise and arithmetic operations to update another global variable using a value from a table and a third global variable.
- `calculate_offset_and_store`: Checks a global variable and, if non-zero, calculates a value by masking a word from a table with the result of a recursive call, shifting it right by three bits, adding it to another global variable, and storing the result in a third global variable.
- `get_blt_parameters`: Returns a combined value containing a specific constant and data from a table if a condition is met.
- `calculate_difference`: Returns the difference between an input value and a global variable, applying specific logic if the input is less than the variable or if the input is negative.
- `populate_address_table`: Fills a list of memory locations with incrementing values starting from zero, with each entry increasing by a fixed constant, for a total of sixty-four iterations.
- `expand_pointer_table`: Reads a count from a memory location and uses it to iterate through a table, adding the base address to each entry and storing the result back in the table.
- `clear_table_entries`: Iterates through a table and sets specific fields within each entry to zero.
- `store_structure_fields`: Stores multiple values from registers into a structure at a memory location pointed to by a register.
- `update_object_data`: Stores a word into a structure and calculates a new value by multiplying a word by four, adding it to a pointer, and storing the result.
- `insert_into_linked_list`: Clears several fields in a structure and then inserts it into a linked list managed by a global pointer, potentially updating the previous node's link.
- `remove_from_linked_list`: Checks a global pointer for a list, retrieves the next element from the current node, and updates the global pointer to that next element while clearing the current node's link.
- `insert_sorted_list`: Traverses a linked list to find the correct position for a new node based on a comparison, then inserts the node while maintaining the list structure.
- `insert_into_list_b`: Clears two fields in a structure and inserts it into a linked list pointed to by a global variable.
- `pop_from_list_b`: Checks a global pointer for a list and, if the first node's link is non-zero, updates the global pointer to that linked node and clears the original node's link.
- `insert_into_conditional_list`: Checks a global flag to determine which of two different global pointers to use for inserting a structure into a linked list.
- `clear_linked_list`: Iterates through a linked list to clear the contents of each node and then nullifies the list head.
- `render_blt_sprite_clipping`: Calculates intersection boundaries between a source sprite and a screen clipping rectangle, then configures the Amiga Blitter hardware registers to draw the resulting sub-rectangle.
- `calculate_blitter_parameters`: Computes source and destination memory addresses and bitmask configurations for a hardware blitter operation based on sprite dimensions.
- `traverse_linked_lists`: Iterates through two separate linked lists stored in a structure and clears the list head pointers after processing each node.
- `setup_blt_transfer_2`: Calculates coordinates and dimensions for a hardware bitblock transfer, configuring DMA and Blitter registers based on values from a table and a structure.
- `pack_bitfield_data`: Performs bitwise shifts and logical operations on several input values to concatenate them into a single 32-bit result.
- `initialize_sprite_attribute_tables`: Initializes two sets of data structures by writing specific word values and zeroing out corresponding offsets.
- `clear_memory_block`: Fills a block of memory with zeros using a loop.
- `pack_multiple_bitfields`: Calls a bitfield packing routine multiple times and stores the resulting 16-bit values into various global variables and memory locations.
- `pack_multiple_bitfields_2`: Calls a bitfield packing routine multiple times and stores the resulting 16-bit values into different global variables and memory locations.
- `update_structure_fields`: Calls a bitfield packing routine and writes the resulting 16-bit values into specific offsets of two different structures.
- `duplicate_packed_data_to_structures`: Calls a routine to pack bitfield data and then writes the resulting 32-bit value and a separate 16-bit value into two different memory structures at specific offsets.
- `update_dual_structures`: Calls a bitfield packing routine twice and writes the results into the offset fields of two separate structures.
- `update_bitfield_structures`: Calls a routine to pack bitfield data and writes the resulting values into two separate memory structures at specific offsets.
- `execute_three_patterns`: Calls a data expansion routine three times in succession.
- `populate_interleaved_buffer`: Repeats a specific routine three times in succession.
- `process_data_sequence`: Clears a memory block, performs an interleaving operation at an offset calculated from a global variable, and executes several other routines.
- `interleave_and_clear_memory`: Clears a block of memory with zeros and then interleaves data from two source locations into a destination buffer.
- `expand_data_pattern`: Reads two source values and interleaves them into a destination buffer to create a repeating pattern.
- `interleave_data_subset`: Copies specific bytes from two source locations into a destination buffer in an alternating sequence.
- `shift_memory_blocks`: Calculates offsets based on an input value and copies three blocks of data from different locations within a global table to the start of that table.
- `calculate_offsets_and_copy_data`: Calculates three different offsets based on an input value through iterative subtraction, then copies three blocks of data from those calculated source locations to the start of a global variable.
- `copy_data_redundantly`: Copies a block of data to the same location using two overlapping pointers.
- `duplicate_memory_range`: Performs a redundant copy of a data range to itself using incrementing pointers.
- `process_attribute_data`: Iterates through nested loops to update structure fields, modify a linked list, and distribute bytes from a table based on calculated indices.
- `update_attribute_table`: Calculates indices based on input values through iterative subtraction and uses them to fetch bytes from a source table.
- `distribute_table_bytes`: Calculates two indices from a register value using two separate loops, then fetches bytes from a table at those indices.
- `save_register_context`: Saves the current processor registers into a memory block and updates a global variable.
- `save_register_state`: Stores the current contents of several registers into a block of memory and sets a global variable to a constant value.
- `increment_and_set_constant`: Increments a global variable and stores a constant value in a memory location.
- `insert_into_linked_list_2`: Inserts a new node into a linked list by updating the pointers of the current node and the existing head.
- `remove_from_linked_list_2`: Removes the head node from a linked list and updates the head pointer to the next node in the sequence.
- `initialize_data_table`: Iterates through a table to initialize multiple fields in each entry with specific constant values and zeros.
- `update_linked_list_node`: Modifies a linked list by updating node pointers and adjusting a counter based on specific conditions.
- `store_constant_value`: Stores a specific constant value into a memory location offset from a pointer.
- `set_table_values`: Sets several multi-byte values in a table based on the value of a specific byte.
- `check_and_update_state`: Compares a byte against a stored value and, if different, updates the stored value and calls several other routines.
- `update_linked_list_elements`: Traverses a list of structures to modify specific fields, update linked list pointers, and perform conditional table-based updates.
- `update_packed_structures`: Uses nested loops to pack bitfield data and write the resulting values into specific offsets within a series of structures.
- `write_bitfield_data`: Iterates through loops to pack data and write 32-bit and 16-bit values into specific offsets of a structure pointed to by a table.
- `update_state_byte`: Updates a byte in a structure with a new value based on several conditional checks of existing data.
- `calculate_timing_parameters`: Calculates and stores multiple timing or duration values into a structure based on various state flags.
- `initialize_memory_bounds`: Sets two memory address values in a structure, potentially adding an offset to the second value based on a flag.
- `execute_triple_pattern_expansion`: Calls a routine that expands data patterns three consecutive times.
- `execute_repetitive_routine`: Calls a specific routine three times in succession.
- `update_display_parameters`: Writes three longwords to a destination pointed to by a register, using values derived from offsets within a global structure.
- `write_display_buffer_parameters`: Writes three longwords to a destination pointed to by a register, using values derived from a global structure.
- `check_structure_conditions`: Evaluates specific bits and value ranges within a structure pointed to by a register before calling another routine.
- `set_function_pointers`: Clears a byte in a structure and assigns specific code addresses to two different pointer locations.
- `process_joystick_input`: Processes a bitmask from a global variable to return a new bitmask representing button and direction states.
- `process_second_joystick_input`: Processes a bitmask from a global variable to return a different bitmask representing button and direction states.
- `fetch_byte_from_offset`: Returns a single byte from a memory location calculated by adding a fixed offset to a base address held in a register.
- `update_entity_state_and_logic`: Iterates through a linked list of entities to update their status flags, spatial bounds, and movement timers.
- `compare_and_fetch_byte`: Compares a value pointed to by a register against a value at a fixed offset from a base address, then returns a byte from a memory location.
- `execute_conditional_fetch`: Performs a comparison between a memory value and a calculated threshold before returning a byte from a specific offset.
- `validate_entity_spatial_bounds`: Checks an entity's coordinates against a set of boundary limits and a table of spatial zones.
- `update_entity_movement_state`: Checks entity status flags and coordinate differences to determine if an entity has reached a target position or requires a state transition.
- `process_entity_state`: Evaluates entity coordinates and status flags to update state variables or modify structure members.
- `search_collision_list`: Iterates through a linked list of structures to find an entry matching specific coordinate and bitmask criteria.
- `update_entity_state_flags`: Checks entity status bits and coordinate bounds to update state flags, modify entity type bytes, and assign pointers to a global variable.
- `update_entity_state_and_bounds`: Checks entity coordinate bounds and updates state flags or pointers based on specific bitmask conditions and value comparisons.
- `check_boundary_collision`: Traverses a list of entities to determine if a coordinate falls within specific bounds defined by structure members.
- `update_entity_state_and_position`: Updates an entity's status flags, animation state, and coordinate data based on bitmask checks and movement calculations.
- `process_collision_logic`: Calculates differences between values to determine if specific status flags or coordinate data should be updated.
- `update_collision_and_object_bounds`: Checks for proximity between a moving entity and specific boundary or object coordinates to set status flags.
- `update_entity_status`: Checks entity bounds and status flags, potentially updating a pointer, setting a status byte, or executing a function pointer stored in a structure.
- `validate_entity_bounds`: Checks if an entity is within a specific range and updates its pointer or resets status flags based on the result.
- `update_status_flags`: Iterates through a table to compare values and sets specific bits in a byte within a structure based on a short value.
- `calculate_distance_offset`: Computes the absolute difference between two values based on various conditional checks of structure flags and pointers.
- `assign_callback_pointer`: Sets a pointer within a structure to either a routine address or a global variable based on a byte value.
- `assign_callback_or_pointer`: Checks a byte at a specific offset within a structure to determine whether to store the address of another routine or a global variable into a pointer field within that same structure.
- `assign_global_pointer`: Stores the address of a global variable into a structure at a specific offset.
- `store_global_pointer`: Stores the address of a global variable into a pointer located at an offset within a structure.
- `initialize_structure_data`: Sets multiple fields in a structure, including a byte at an offset, a pointer to a global variable, and a constant value.
- `configure_structure_parameters`: Sets multiple fields in a structure, including bitwise flags, memory addresses, and constant values.
- `set_status_bit`: Sets the third bit of a byte at a specific offset within a structure.
- `update_entity_state_and_bounds_2`: Iterates through a linked list of entity structures to evaluate status flags, coordinate bounds, and state transitions.
- `compare_value_and_branch`: Compares a value pointed to by a register against a calculated threshold and calls another routine.
- `execute_conditional_branch`: Performs a comparison between a value and a threshold to determine which routine to call.
- `branch_on_comparison`: Compares a short value to a threshold derived from a global offset and branches to another routine.
- `traverse_and_update_linked_list`: Iterates through a linked list of structures, performing conditional checks on specific bitmasks and values.
- `update_linked_list_state`: Iterates through a linked list of structures, performing conditional checks on bitmasks and specific integer values.
- `validate_text_input_range`: Checks a character buffer against specific control characters and validates that the input string falls within defined spatial or index boundaries within a data table.
- `check_state_and_call`: Checks if a short value matches several specific constants and if another value is less than one before calling another routine.
- `compare_and_branch_dual`: Compares a value against a threshold and branches to another routine based on the result.
- `process_state_transition`: Modifies bits in a byte and copies data from a structure into a table or a different memory location based on specific bit states.
- `update_structure_and_decrement`: Modifies several fields in a structure, updates a pointer, and performs a subtraction on a value, potentially clamping it to zero.
- `call_memory_block_shift`: Calls a routine that shifts memory blocks based on an input value.
- `update_structure_state`: Modifies various fields within a structure, including bitwise operations on a byte, and performs conditional updates based on a global value and a returned value from another routine.
- `update_structure_state_alt`: Updates a structure by modifying specific bits in a byte and performing arithmetic on a value retrieved from another routine.
- `update_state_flags`: Checks a byte at an offset in a structure and updates several fields including a pointer to a global variable and two short values based on the result.
- `modify_structure_flags`: Performs bitwise operations on a byte within a structure and conditionally assigns a pointer to a global variable.
- `set_status_bit_2`: Sets the third bit of a byte located at an offset within a structure.
- `process_linked_list`: Iterates through a linked list of elements, performing different actions such as updating a value, jumping to a subroutine, or loading a pointer based on the value of the current element.
- `initialize_structure_and_loop`: Sets specific bytes and clears two longwords in a structure, then executes a loop that calls another routine three times with incrementing offsets.
- `store_linked_values`: Stores integer values into specific offsets of a structure and updates a global pointer.
- `store_three_values`: Stores three integer values into specific offsets within three different memory locations.
- `traverse_and_clear_list`: Checks a pointer in a structure and, if valid, attempts to clear a linked list by updating the pointer to the next element and zeroing out a field.
- `clear_linked_values`: Clears integer values at specific offsets within a structure and updates a global pointer.
- `clear_three_values`: Sets two integer values to zero at specific offsets and stores a third value into a different memory location.
- `process_state_logic`: Evaluates status flags and counters within a loop, performing linked list updates and modifying structure values based on conditional checks.
- `modify_linked_list_state`: Checks a value in a structure and, if it matches specific constants, executes a routine and potentially sets a bit in a memory location.
- `reorder_linked_list_nodes`: Modifies a linked list by updating node pointers and adjusting a counter based on specific conditions.
- `update_linked_list_node_2`: Modifies a linked list by updating node pointers and adjusting a counter based on specific conditions.
- `manage_linked_list`: Iterates through a list to update pointers and counters, while performing bitwise operations on status flags and managing structure offsets.
- `update_linked_list`: Modifies bitmasks in a structure, reorders pointers in a linked list, and decrements a value in a structure.
- `manipulate_linked_list_nodes`: Modifies a bitmask in a structure, reorders pointers within a linked list structure, and decrements a short value.
- `check_state_and_set_bit`: Evaluates two bytes in a structure and, if certain conditions are met, sets a specific bit in a byte within another structure.
- `update_state_counter`: Evaluates a state variable to set a status byte and conditionally increments that status byte if two specific character flags are set.
- `execute_loop_sequence`: Runs a loop up to sixteen times, calling a sequence of subroutines with varying parameters and updating a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_copy_data`: Executes a loop that calls several routines repeatedly for a fixed number of iterations.
- `copy_data_block`: Copies a block of sixteen bytes from a source table into a destination starting at a global variable and updates three specific memory locations with the final elements of the source.
- `advance_linked_node`: Moves a pointer to the next node in a linked list and clears a value in the current node if the next node is null.
- `insert_sorted_node`: Inserts a new node into a sorted linked list by traversing the list and updating the pointers of the surrounding nodes.
- `monitor_flags_and_relink_nodes`: Enters a loop that monitors bitmasks in a structure and performs a linked list pointer reconfiguration when a specific bit pattern is detected.
- `initialize_dma_control`: Sets several global variables to specific values and enables DMA channels by writing to the DMACON register.
- `execute_conditional_subroutines`: Performs a series of conditional checks on a register and calls various other routines based on the resulting bit flags.
- `call_subroutines`: Calls four consecutive instances of the same routine before returning.
- `evaluate_conditional_logic`: Performs a series of conditional checks on a flag and calls other routines based on the results of those evaluations.
- `check_negative_flag`: Checks the status of the negative flag and conditionally calls another routine if the flag is not set.
- `execute_conditional_calls`: Calls a routine and checks its return value, then conditionally calls that same routine again if the initial result was false.
- `compare_table_values`: Compares four values from a table against a value stored in another memory location and returns if all conditions are met.
- `compare_offset_values`: Compares four values at specific offsets within a table against a value in another memory location and returns if all conditions are met.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d1, then returns.
- `update_table_entry_2`: Calculates an offset into a table using a multiplier and a global base, then stores a value from that table into a global variable.
- `update_table_entry`: Calculates an offset into a table based on an input, stores a value from that table into a global variable, and writes a constant to the calculated address.
- `populate_table_structure`: Writes multiple values into a table structure at calculated offsets, including values from other memory locations and constants.
- `execute_dma_routine_2`: This routine calls another routine and interacts with the AUD0LCH hardware register.
- `execute_dma_routine`: A routine that touches the DMACON register and is called by one other routine.
