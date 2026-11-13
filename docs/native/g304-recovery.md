# Game 304 (Zaxxon): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00304). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907200 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 18 raw-word lines |
| Functions named and described | pass, 286 of 286 named (0 author symbols), 286 of 286 described (15 by rule, 238 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3782 of 3782 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7583 of 7583 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 420896 |
| code | 22306 |
| data | 57572 |
| padding | 1434 |
| unused | 404992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zaxxon.adf:trainer | 3 | 160 | 5 |
| Zaxxon.adf:Game | 1 | 5224 | 224 |
| Zaxxon.adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 508 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 529 | 11 |
| Zaxxon.adf:trainer#unpacked | 1 | 1037 | 24 |

## Cases handled

- 8 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 317 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 419 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zaxxon/Zaxxon.adf:trainer: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 16096 -> 103584 bytes), 9 of 10 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 displacement
- Zaxxon/Zaxxon.adf:trainer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Zaxxon.adf:trainer
- Zaxxon.adf:trainer#unpacked

## Copy protection found

- none found

## Functions


### Zaxxon.adf:trainer

- `free_memory_and_clear_slot`: Calls a subroutine and a function pointer, calls the system memory-free function through a global, and zeroes a longword just before a pointer.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `unpack_stream_with_table_updates`: Decodes a packed stream into a buffer filled from its end toward its start.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Zaxxon.adf:Game

- `wait_for_port_then_reply`: Finds the current task and, when its message-port field is empty, waits on the port and fetches a message.
- `wait_for_flag_then_halt`: Busy-waits until a global flag becomes all ones, then enters an endless loop that never returns.
- `copy_block_find_table_base`: Copies a block of longwords from an offset inside a larger area into the global area and checks a tag value, copying one byte when it matches.
- `update_audio_and_sprite_attributes`: Iterates through a data structure to update sprite positions, animation frames, and audio parameters, including volume and pitch adjustments.
- `update_sprite_and_audio_parameters`: Iterates through a data structure to update sprite attributes, position, and animation frames based on table lookups.
- `update_entity_state`: Loads entity attributes from a data table into a structure based on an index.
- `copy_table_entry_to_memory`: Copies a word from a table into two different memory locations.
- `iterate_object_properties`: Iterates through a list of objects and calls another routine to update their properties if a specific byte is non-zero.
- `update_interpolated_object_properties`: Updates several object properties by decrementing counters and fetching new values from a table when counters reach zero.
- `calculate_object_offsets`: Copies data from a table to a global variable, finds a maximum value in a list, and then populates a series of memory locations with calculated offsets.
- `populate_object_data`: Iterates through a table to calculate and store derived values, such as offsets and scaled constants, into a series of memory locations.
- `populate_hardware_registers_from_table`: Iterates through a table to fetch data and copies specific values into hardware registers and memory locations.
- `update_hardware_and_object_tables_2`: Iterates through a data table to calculate derived values and updates hardware registers, including copper and DMA settings.
- `calculate_memory_pointers`: Copies a block of data, finds a maximum value in a list, and populates several pointers with calculated memory addresses.
- `update_hardware_and_object_tables`: Iterates through a data table to calculate derived values and updates hardware registers, including copper and DMA settings.
- `update_maximum_value`: Compares a value in a register against a value at an offset from a pointer and updates that memory location if the register value is larger.
- `wait_for_bit_pattern`: Enters a loop that waits until a specific bit pattern is detected in a memory location.
- `wait_for_bit_match`: Enters a loop that continues until a specific bit pattern in a memory location matches a value derived from a register.
- `wait_for_bit_clear`: Enters two consecutive loops that wait for a specific bit in a byte at a memory offset to become zero.
- `rotate_bits`: Performs bitwise operations on a global variable to clear a bit and then performs a circular bit rotation.
- `update_hardware_registers`: Iterates through a table to fetch data and copies specific values into hardware registers and memory locations.
- `execute_repeated_routines`: Calls two other routines repeatedly in a loop until a specific character value is returned or a counter reaches zero.
- `execute_until_character`: Executes a routine repeatedly in a loop until a specific character is returned or a counter reaches zero.
- `loop_until_char`: Executes two routines repeatedly in a loop until a specific character is returned or a counter reaches zero.
- `populate_sprite_data`: Reads a list of indices and copies blocks of data from a source table into a destination buffer based on those indices.
- `initialize_constant_values`: Sets three specific global variables to constant values.
- `clear_memory_block_2`: Fills a large block of memory with zeros.
- `update_joystick_flags`: Reads the joystick port register and updates a global variable with bitwise flags based on the current input state.
- `update_input_state`: Reads hardware registers and compares current values with previous states to update a global variable representing various input flags.
- `manage_game_state_machine`: Monitors a global state variable to execute specific initialization sequences, including clearing memory tables and setting up data structures.
- `manage_state_transition_and_table_reset`: Monitors a global state variable to trigger specific memory initialization sequences and table clearing loops.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `frag_moveq_lea_bra`: Short code fragment that loads 2 into d6, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `return_without_action`: The routine is reported as having 3 instructions, but the decompiled pseudo-code shows only a bare return with no operations, and the evidence available here does not reveal what the three instructions are.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `update_entity_state_parameters`: Searches a table for a matching byte value and then modifies several numeric fields within a data structure based on bitwise flags.
- `update_table_entry`: calculates an index based on a value from a pointer, then writes zero or a specific constant to a table entry and a preceding word based on global flags.
- `search_and_reset_table`: Iterates through a table to compare values against a reference, then performs a block copy and fills a memory area with a constant if a match is found.
- `check_bounds_and_reset`: Iterates through a list of data structures, performing coordinate-based bounds checks, and executes a memory reset sequence if conditions are met.
- `verify_status_and_reset`: Checks global status values and calls another routine to determine if a specific memory reset and block copy sequence should be triggered.
- `execute_conditional_function`: Checks if a global function pointer matches a specific routine and a counter is within range, then executes that routine and performs a memory reset.
- `reset_memory_state`: Checks two status flags and, if both are non-zero, performs a block copy of data from a global variable to a destination table and subsequently fills a portion of the global variable with a constant value.
- `handle_collision_reset`: Checks two status flags and, if both are non-zero, updates a value in a table and triggers a memory reset sequence.
- `process_entity_status`: Checks a global flag and iterates through a table of data to evaluate status conditions for multiple entries, potentially triggering a memory reset and data copy sequence if specific criteria are met.
- `check_flag_and_reset`: Verifies a global value is above a threshold, clears a flag, calls several routines, and performs a memory reset if a specific flag is set.
- `monitor_flag_for_reset`: Calls two routines and checks a global flag to decide whether to trigger a memory reset and block copy sequence.
- `check_status_flag`: Compares a value at a pointer against a constant and sets a global flag if certain conditions are met.
- `set_status_flag`: Sets a global flag to one if the value at a specific pointer is non-zero.
- `update_entity_state_and_bounds`: Iterates through a list of data structures to update position values and check for boundary collisions.
- `copy_data_block`: copies eight 32-bit words from a source table to a destination buffer using a loop.
- `process_graphics_data`: performs bitwise operations on data in two memory locations or copies bytes from a source table to those locations based on specific state conditions.
- `initialize_memory_tables`: Sets several global variables to specific constant values and then enters a loop to fill a memory block with a repeating pattern of words and zeros.
- `clear_memory_tables`: Clears a block of memory with zeros and then performs loops to fill specific memory regions with either zero or a constant value based on the contents of a global variable.
- `clear_and_fill_tables`: Clears a block of memory with zeros and then performs two separate loops to fill different memory regions with the value zero or a specific constant based on the contents of a global variable.
- `initialize_state_parameters`: Sets various global pointers, counters, and constants based on the value of a global variable.
- `execute_nested_loops`: Selects a function pointer from a table based on a global value and executes it within nested loops that decrement a global counter.
- `pop_data_stack`: Decrements a counter and, if it is not zero, retrieves a word from a table and updates two other variables with the retrieved values.
- `manage_game_state_and_counters`: Updates multiple global counters and state variables through conditional decrements and increments.
- `update_loop_counters`: Increments and decrements multiple global counters and performs arithmetic on memory locations within a loop.
- `decrement_table_values`: Iterates through a table of sixteen-bit values, checking if a specific byte is set, and if so, decrements the associated word by eight if it remains above a certain threshold.
- `adjust_pointer_values`: Iterates through a table of four short integers and, if a value equals two, decrements a corresponding word in another memory area by eight.
- `process_loop_counters`: Updates several global counters and memory locations within a loop based on specific conditions.
- `copy_graphics_data`: Calculates source and destination addresses based on several global variables and performs a structured copy of data blocks into a target memory range.
- `call_indirect_table`: Calls a routine and then performs an indirect call using a value from a table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_offset_data`: Performs arithmetic on a value to determine memory offsets and performs bounds checking against global ranges.
- `branch_to_offset_114`: Performs a short branch to another location.
- `branch_to_offset_110`: Performs a short branch to another location.
- `branch_to_offset_16`: Performs a short branch to an instruction located 16 bytes ahead.
- `jump_to_populate_data_structure_2`: Jump stub: passes control straight on to populate_data_structure_2.
- `jump_to_populate_data_structure_2_2`: Jump stub: passes control straight on to populate_data_structure_2.
- `branch_to_next_instruction`: Performs a short branch to the instruction immediately following this one.
- `update_coordinate_data_2_2`: Multiplies d1 by 5 and adds d0, then stores the result in a global variable, calls populate_data_structure_2 and update_coordinate_data, and performs several conditional checks and stores based on the calculated value.
- `branch_to_coordinate_processing`: Performs a branch to another routine.
- `clear_and_copy_data`: Clears a block of memory with zeros, initializes several global variables, and copies a sequence of words from one location to another.
- `jump_to_populate_data_structure`: Jump stub: passes control straight on to populate_data_structure.
- `reset_and_copy_data_2`: Initializes several global variables to zero, calls reset_global_values and store_global_address_pointer, then enters a loop to copy 24 words from a global variable to another location.
- `populate_global_tables_and_branch`: Writes several constant values, pointers, and zeroed entries into various global variables and tables, then branches to another routine.
- `branch_to_table_initialization`: Performs a branch to another routine.
- `update_coordinate_data_2`: Multiplies d1 by 5 and adds d0, then stores the result in a global variable, calls populate_data_structure_2 and update_coordinate_data, and finally branches to another routine.
- `populate_data_structure_2`: Searches a table for a zero word and populates a structure at that location with several global values.
- `update_coordinate_data`: Retrieves a value from another routine and writes several words and a long value to a memory block at a specific offset.
- `store_result_to_memory`: Takes a value from another routine and stores it at a specific offset in memory along with a constant value.
- `initialize_complex_structure`: Sets several global variables and populates a memory structure with values derived from conditional checks.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `store_long_values`: Retrieves a long value and stores it along with several other global variables and constants into a memory structure.
- `store_long_result`: Retrieves a value from another routine and stores it as a 32-bit long value in a memory location, accompanied by other global data.
- `process_coordinate_table`: Performs a loop that calculates new values for a table by multiplying bytes by 8, adding 104, and multiplying by 8 again, then stores these results into a destination table.
- `reset_and_copy_data`: Initializes several global variables, calls reset_global_values and store_global_address_pointer, then enters a loop to copy data from one location to another before calling copy_data_to_self.
- `populate_data_structure`: Initializes several global variables and then executes a loop to fill a table with a repeating sequence of seven words containing incrementing and constant values.
- `initialize_global_tables_2`: Writes several constant values and a pointer to various global variables and tables, then branches to another routine.
- `initialize_global_tables`: Writes various constant values and addresses into several global variables and tables, then branches to another routine.
- `set_global_word`: Stores the value 1 into a global variable.
- `process_event_script_and_state`: Evaluates a state machine based on a global status variable and iterates through a data table to execute scripted commands.
- `update_status_flag`: checks a global variable and performs a bitwise XOR on another global variable if certain conditions are met, then decrements a counter.
- `process_bitmask_coordinate_offsets`: Reads a value from a pointer and uses its bits as flags to apply additive or subtractive offsets to a coordinate value.
- `clamp_coordinates`: checks if two values fall within specific bounds and stores them in a table, otherwise stores default values.
- `adjust_values_incrementally`: compares values in two tables and adjusts a third table by adding or subtracting a constant value based on those comparisons.
- `update_entity_list`: Iterates through a table to check status flags and performs memory copies and global updates if certain conditions are met.
- `check_bounds_collision`: Compares several values against a set of bounds stored in a table to determine if a coordinate is within range.
- `update_entity_states`: Iterates through a table of data to check specific status flags.
- `validate_and_update_position`: Updates a global index and checks if a coordinate pair falls within specific numeric bounds before storing it.
- `validate_and_store_coordinates`: Modifies a value via an offset and checks if two resulting values fall within specific numeric bounds.
- `validate_position_bounds`: Checks global values against specific bounds and, if valid, calculates a new position using a table and stores it in a global variable.
- `update_list_pointer`: Searches for a non-zero value in a list and updates a table with the resulting pointer and offset.
- `initialize_and_clamp_entity_table`: Initializes a table with default values if a specific global counter reaches a threshold, then iterates through a loop to process bitmask-driven data.
- `increment_pointer`: Increments the value at the address pointed to by a register by four.
- `increment_word_via_pointer`: Increments a 16-bit value at a memory location pointed to by a register by four.
- `jump_to_tiny_subq_rts`: Jump stub: passes control straight on to tiny_subq_rts.
- `jump_to_tiny_subq_rts_2`: Jump stub: passes control straight on to tiny_subq_rts.
- `increment_word_at_offset_two`: Adds 4 to a 16-bit word located at offset 2 of the structure pointed to by a0 and then branches to another routine.
- `increment_word_at_offset_two_2`: Adds 4 to a 16-bit word located at offset 2 of the structure pointed to by a0 and then branches to another routine.
- `decrement_word_at_offset_two`: Subtracts 4 from a 16-bit word located at offset 2 of the structure pointed to by a0 and then branches to another location.
- `decrement_word_at_offset_two_2`: Subtracts 4 from a 16-bit word located at offset 2 of the structure pointed to by a0 and then branches to another routine.
- `increment_structure_word_at_offset_four`: Increments a 16-bit word located at offset 4 of the structure pointed to by a0, then branches to another routine.
- `increment_word_at_offset_four`: Increments a 16-bit word located at offset 4 of the structure pointed to by a0 and then branches to another routine.
- `decrement_offset_word`: Performs a relative branch to another location.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to (a0), then returns.
- `tiny_subq_rts`: Small routine of 2 instructions that subtracts 4 from (a0), then returns.
- `increment_offset_two`: Adds 4 to a 16-bit word at offset 2 of the structure pointed to by a0.
- `decrement_offset_two`: Subtracts 4 from a 16-bit word at offset 2 of the structure pointed to by a0.
- `increment_offset_four`: Adds 1 to a 16-bit word at offset 4 of the structure pointed to by a0.
- `decrement_offset_four`: Subtracts 1 from a 16-bit word at offset 4 of the structure pointed to by a0.
- `update_entity_status_flags`: Iterates through a table of data to check for specific status values and increments a long value at a pointer-indexed location.
- `clamp_position_values`: Checks if a pair of values falls within a specific range and stores them as a long value, otherwise stores a default constant.
- `clamp_coordinate_values`: Checks if two values fall within specific bounds and stores them in a table, otherwise stores default values.
- `fetch_table_data`: Checks several global conditions and a counter before retrieving a long value from a table using an index.
- `fetch_table_entry`: Checks a global counter and, if a condition is met, retrieves a long value from a table using an index derived from a global variable.
- `update_position_data`: Increments a global variable with a wrap-around check and performs arithmetic on several global values to store a result in a table.
- `process_event_list_and_update_state`: Iterates through a data table to check for specific attribute matches and performs state transitions based on a control variable.
- `reset_state_variables`: Sets a pointer to zero and assigns specific constant values to two global variables.
- `process_status_events`: Checks a global variable for a specific state and iterates through a table to find non-zero entries.
- `process_loop_sequence`: Iterates through a sequence of data, updating a counter and performing conditional checks on global variables to manage a state machine.
- `reset_global_values`: Sets a global byte to zero and assigns a constant value to another global variable.
- `increment_status_counters`: Iterates through a table to find entries with a specific status and increments a byte value.
- `process_script_event_table`: Iterates through a table of 16 entries to find a match for a specific value, then constructs a linked list structure with modified coordinate data and calls process_linked_list_and_bounds.
- `process_linked_list_and_bounds`: Traverses a linked list structure to update values and performs bounds checking on coordinates before storing them in a table.
- `increment_and_fetch_table_entry`: Increments a global byte and returns a concatenated value from a table using the previous value as an index.
- `search_and_execute_jump_table`: Iterates through a list of data to find a specific marker and then executes an indirect call based on an offset.
- `search_memory_pattern`: Iterates through a memory block in fixed increments to search for a specific byte pattern.
- `process_linked_list`: Traverses a linked list by updating a pointer and a counter.
- `check_memory_bounds`: Compares a value derived from a pointer against a global variable and performs bounds checking against two other memory-related values.
- `update_object_state_parameters`: Initializes a data structure with several constant and register-derived values, then updates specific fields within that same structure after a subsequent routine call.
- `store_data_structure`: Writes several values, including a modified register and a pointer, into a data structure at a specific offset.
- `process_sprite_data`: Calculates memory offsets and performs bitwise shifts to copy and mask data from a source table into a destination buffer.
- `update_sprite_data`: Calculates an offset based on global values and performs bitwise shifts to copy data from a source table into a destination buffer.
- `initialize_and_copy_sprite_data`: Populates a data structure with configuration values and performs a bitwise-shifted copy of data from a source table to a destination buffer.
- `initialize_object_pointer_tables`: Sets several global pointers to a base address and populates memory tables with a sequence of pointers and offsets.
- `sort_word_list`: Performs a bubble sort on a list of 32-bit values in memory.
- `batch_store_word_pairs`: Repeatedly calls another routine to fetch word pairs and stores them into multiple global variables.
- `update_global_counters`: Executes several subroutines and then increments various global variables by specific increments if they do not meet a certain threshold.
- `calculate_and_store_attributes`: Calculates several values based on memory offsets and stores them into a data structure.
- `store_long_and_word_data`: Stores a long value and two additional words into a data structure at a specific offset.
- `calculate_and_store_offsets`: Calculates a series of values based on a global variable and stores them into a table using a loop.
- `iterate_data_table`: Iterates through a table of pointers and calls another routine for each entry until a null pointer is encountered.
- `store_calculated_values`: Calculates two values and stores them into a data structure along with a result from another routine.
- `store_returned_word`: Stores the result of another routine into a memory location.
- `update_if_not_equal`: Compares two global variables and updates the first one with the value of the second if they are not equal.
- `generate_character_data`: Processes bitwise data to populate a table with character-like bytes derived from arithmetic operations.
- `concatenate_shifted_words`: Performs bitwise shifts and concatenations on two words from a pointer to return a single long value.
- `shift_word_right`: Returns a word that has been bitwise shifted right by one.
- `process_masked_data`: Performs complex bounds checking and bitwise masking to copy data from a table to a destination, or performs a direct block copy if conditions are not met.
- `initialize_pointer_table`: Fills a table with a sequence of pointers, incrementing each subsequent pointer by a fixed offset.
- `fill_constant_table`: Fills a table with a repeating sequence of constant word values.
- `accumulate_and_store_values`: Stores a value into a table and then adds the previous value to the current one for the next iteration of the loop.
- `process_data_table_2`: Iterates through a data source to perform character transformations and copies blocks of data into a destination table using calculated offsets.
- `update_state_table`: Updates a four-entry circular buffer table with a value from a global variable.
- `update_buffer_and_execute_loops`: Updates a four-entry circular buffer with a global variable value and performs conditional execution of other routines based on specific global variable states and loop counters.
- `wait_and_execute_loop_2`: Decrements a global variable in a loop until it falls below a threshold, then executes another routine a fixed number of times.
- `wait_and_execute_loop`: Decrements a global variable in a loop until it falls below a threshold, then executes another routine a fixed number of times using a pointer-based loop.
- `copy_data_loop`: Calculates a destination address based on a register and a global variable, then copies eight words from a source to that destination using a loop.
- `modify_d0_and_call`: Pushes a longword from a0 onto the stack, performs a series of bitwise OR operations on d0 with various constants, and calls another routine if a byte loaded from a memory location is not equal to two.
- `execute_repeated_calls`: Executes another routine a fixed number of times based on a value from a global variable.
- `execute_conditional_calls`: Performs a loop that executes another routine multiple times based on a value from a global variable.
- `copy_interleaved_words`: Copies eight words from a source address calculated from two input registers to a destination address.
- `execute_conditional_loop`: Decrements a global variable and, if certain conditions are met, executes a loop that calls another routine and stores a value.
- `compare_nibbles`: Compares the lower four bits of two input words.
- `copy_and_clear_memory`: If a global flag is zero, it copies a block of data from a global variable to a destination address and then sets the flag to a new value.
- `store_global_address_pointer`: Stores the address of a global variable into another global variable, serving as a pointer assignment.
- `assign_global_address`: Stores the memory address of a global variable into another global variable.
- `store_address_pointer`: Loads the address of a global variable into a0, loads the address of another global variable into a1, and stores the value of a0 into the location pointed to by a1 before returning.
- `copy_data_to_self`: Copies a sequence of words from a global variable to itself using a loop.
- `copy_memory_block_3`: Performs a word-by-word copy of a data block.
- `clear_memory_block`: Fills a block of memory with zeros.
- `build_relative_address_table`: Calculates cumulative offsets from a list of integers and then converts those offsets into absolute pointers within a table.
- `initialize_engine_subsystems_2`: Copies a byte from a source pointed to by a0 into a structure at a6, then performs a series of logical OR operations on registers d0 and d1 using various immediate byte values.
- `initialize_asset_lookup_tables`: Clears several memory blocks and populates a table with calculated offsets and parameters derived from a source data structure.
- `initialize_sprite_data_table_2`: Clears a large memory block and populates a table with calculated sprite attributes, including offsets, dimensions, and scaling factors derived from a source data list.
- `initialize_engine_subsystems`: Clears several memory blocks and populates a table with calculated offsets and parameters derived from a source data structure.
- `transform_global_byte`: Performs bitwise operations on a global variable and returns the result.
- `poll_memory_pattern`: Repeatedly checks a memory location until it matches two specific bit patterns in sequence.
- `update_audio_parameters`: Iterates through a data structure to update audio registers and DMA settings using values from a table.
- `decrement_global_counter`: Decrements a global variable and sets another global variable to zero if the counter is zero.
- `process_input_state`: Evaluates a character input against several conditions and manages internal counters and state transitions.
- `initialize_global_state`: Sets several global variables to specific constant values.
- `initialize_global_state_2`: Checks if a global variable is zero and, if so, assigns it a constant value and sets another global variable to a specific constant.
- `process_nested_memory_loops`: Executes nested loops to move data between memory locations and updates several global variables.
- `copy_indexed_data`: Iterates through a byte sequence to use each byte as an index into a table, copying the resulting values to a destination.
- `copy_data_via_lookup`: Copies a word from one location and then uses a series of bytes as indices to copy subsequent bytes from a table to a destination.
- `decrement_global_counter_2`: Decrements a global variable and conditionally increments another global variable.
- `copy_data_loop_2`: Copies eight words of data from a source table to a destination memory area using a loop.
- `execute_indirect_call`: Decrements a global counter and executes a routine from a table using an index if the counter reaches zero.
- `return_without_action_2`: This routine is empty: its only instruction is an immediate return, so it performs no work and has no side effects.
- `initialize_data_copy`: Sets up indices and pointers to perform a block data copy from a table to a destination area.
- `initialize_data_copy_offset`: Sets up indices and pointers to perform a block data copy using a specific offset from a table.
- `copy_data_block_2`: Copies a block of data from a source table to a destination memory area based on calculated offsets.
- `transfer_memory_segment`: Copies eight words of data from a source address to a destination address using a loop.
- `manage_callback_list`: Iterates through a list of structures to find and update a specific entry, then executes a loop of calls.
- `process_string_data`: Reads a byte from a pointer, updates a destination with the byte and associated data, and increments the pointer.
- `copy_and_iterate_data`: Copies two 32-bit values from one memory location to another and then executes a loop that calls another routine while iterating through a table of values.
- `process_buffer_loop`: Performs a loop that iterates through a table and performs calculations based on a returned value.
- `process_numeric_buffer`: Clears a global flag and enters a loop that iterates a fixed number of times to call another routine, updating a pointer and a local counter during each iteration.
- `convert_digit_to_char`: Converts a numeric value to its character representation and stores it in a destination buffer, optionally setting a flag in a global variable.
- `initialize_system_tables_and_hardware`: Initializes several data tables with specific patterns and populates a jump table with function pointers.
- `configure_dma_and_interrupts`: Initializes DMA channels and custom hardware registers while setting up interrupt enable bits and COP-based jump tables.
- `wait_for_hardware_bit`: Enters a loop that waits until a specific bit pattern is present in a hardware register.
- `wait_for_bit_match_2`: Enters a loop that waits until a specific bit pattern in a memory location matches a provided value.
- `transform_byte_bits`: Performs bitwise operations on a global variable and returns the transformed byte.
- `decrement_and_update_accumulator`: Decrements a global variable and, if it reaches zero, initializes a pointer from a table to load values into other variables.
- `decrement_pointer_index`: Decrements a global value after conditionally initializing a pointer from a table.
- `initialize_data_structure`: Tests bit 0 of d7, performs two zeroing operations on a byte at the address in a4, and then writes the value 1 to four consecutive 16-bit words at a location relative to the program counter.
- `configure_dma_channels`: Updates DMA descriptors and interacts with audio and DMA control registers to manage data transfers.
- `calculate_table_values`: Iterates through a source table to perform various arithmetic operations and stores the results in a destination table.
- `store_memory_addresses`: Stores three consecutive memory addresses into a global variable sequence.

### Zaxxon.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_to_pointer_list`: Iterates through a list of longwords starting at the address in a5, adding the value in d0 to each non-zero longword, until the address in a4 is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_subq_rts_2`: Small routine of 2 instructions that subtracts 1 from (a5), then returns.
- `update_audio_dma_channels`: Updates audio data by reading from a table, writing to the DMA controller and the audio channel register, and performs a loop that decrements a counter and shifts a value.
- `clear_and_branch`: Clears a byte in a register and performs a relative branch.
- `update_dma_control`: Decrements a word at an offset of a pointer in a0, decrements a0 by 4, loads a value into a1, performs a loop based on d7, performs a second loop based on d0, and writes d5 to the DMA control register.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (ori.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `update_table_entries`: Processes a table of structures, calculating various values including a division by a constant and a division involving a value from a global variable, and writes these results into a destination structure while advancing through the table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to (a5)+, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_idle`: Clears a small block of state, sets up several global words and writes DMA and interrupt enable values.
- `toggle_nop_code_patches`: Swaps two sets of instruction words between their patched and original forms, using a state byte to choose which form is written.
- `toggle_keyboard_code_patches`: Switches two code patch sites between no-operation and original instruction words, selected by a state byte, then clears the state byte.
- `write_patch_word_and_toggle`: Writes a fixed instruction word to a location, then toggles a second instruction word between a no-operation and its original form when the state byte matches.
- `write_on_state_patch_words`: Writes two fixed instruction words to a location and clears the state byte, then returns a value derived from the input registers.
- `advance_table_on_timer`: Counts a delay timer down and, when it expires, resets it and advances a pointer through a table of words, wrapping at the end.
- `copy_glyph_to_two_rows`: Looks up a character code in a table of up to seventy-three codes, then copies its eight glyph bytes to two destinations a fixed distance apart, one row per byte.
- `rotate_points_and_start_blit`: Swaps two buffer pointers, then computes rotated and offset coordinates for eleven pairs of table entries using fixed-point multiplication and stores them.
- `reset_blitter_masks_and_modulo`: Waits until the blitter is idle, then writes full first and last mask words, a control word and two modulo values for the blitter.
- `build_row_offset_table`: Fills 256 consecutive words with offsets that increase by forty for each row, forming a lookup table of row start positions.
- `draw_line_with_blitter`: Orders two endpoints, computes the step counts and octant of a line, toggles one pixel bit at the start position, and programs the blitter with the modulo, control and size values to draw the line.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (addi.w, andi.l) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_sound_command`: Dispatches on the low two bits of its input.
- `initialize_channel_structure`: Clears several fields of a channel structure, stores a pointer taken from a second structure, and sets a fixed longword, a mask byte and a flag byte.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `store_low_nibble_field`: Stores the low four bits of its input into a byte field of the structure addressed by the base register.
- `set_or_clear_global_bit`: Sets bit one of a global byte when the input is zero, and clears that bit otherwise.
- `store_negated_low_byte`: Stores the negated low byte of its input into a word field of the structure addressed by the base register.
- `store_low_byte_word_field`: Stores the low byte of its input into a word field of the structure addressed by the base register.
- `store_input_byte_field`: Stores the input byte into a fixed byte field of the structure addressed by the base register.
- `store_low_six_bits_field`: Stores the low six bits of its input into a byte field of the structure addressed by the first address register.
- `store_six_bits_at_base`: Stores the low six bits of its input into a byte field of the block addressed by the base register.
- `store_table_pointer`: Masks its input to six bits, multiplies it by sixteen, adds a fixed base address, and stores the result as a pointer in a structure field.

### Zaxxon.adf:trainer#unpacked

- `branch_to_offset`: Performs a relative branch to a fixed offset.
- `write_zero_to_buffer`: Copies a long value from an offset of a6 to d7, performs an OR operation on a byte at an offset of a6, sets a0 to a constant, and then runs a loop that writes a zero to a location pointed to by a0 and decrements d0 until d0 reaches zero, before branching to another routine.
- `process_keyboard_event`: Decrements a value in a3, pushes it to the stack, compares a byte at a memory location to zero, and if non-zero, writes a constant to a memory location and branches to another routine.
- `handle_keyboard_input`: Compares a byte from the CIAA serial data register against specific values and, depending on the result, writes constants to a memory location or a hardware register.
- `configure_hardware_registers`: Writes zero to a hardware register, writes a constant to a memory location, adds a constant to a pointer, compares a byte at a memory location to 93, branches if not equal, adds a constant to the pointer, compares a word at the pointer to 18545, branches if equal, writes one to a hardware register, writes 18545 to the pointer location, and writes 18545 to two subsequent memory locations.
- `configure_system_hardware`: Initializes hardware registers, copies several words from various locations to a set of global variables, performs a loop to multiply and store values in a table, and contains complex logic to update hardware settings and global variables based on input from the mouse and keyboard.
- `update_table_data`: Checks a byte at a global variable and decrements a long value at another global variable; if that value reaches zero, it resets it to two, copies four words from a table to several global variables, increments a global offset, calls another routine, and calls a function via a pointer before returning with negative one in d0.
- `render_text_string_to_buffers`: Iterates through a string of character codes, looking up each code in a table to find its corresponding glyph data.
- `calculate_blitter_line_parameters`: Swaps two pointers, writes values to a global variable and blitter registers, waits for a DMA bit to clear, performs a loop to calculate and store transformed coordinates using multiplication and bit shifts, calls another routine, performs a second loop to call another routine, waits for a DMA bit to clear, and updates blitter registers with a new address and configuration.
- `reset_blitter_masks_and_modulo_2`: Polls the blitter status bit until the blitter is idle, then writes full mask words, a control word, and two modulo values to the blitter registers and returns.
- `fill_array_with_increments`: Fills a 256-word array starting at a location relative to the program counter with values that increment by 40 at each step.
- `draw_line_with_blitter_2`: Orders two endpoints, calculates the distance and octant of a line, toggles a single bit at the starting position, and populates a structure in a6 with the modulo, control, and size values required for a blitter operation.
- `update_music_channel_state`: Depending on the value in d0, it either updates a single byte in a table or performs a complex update of a music channel structure by reading sequence data, updating pitch and volume registers, and calling step_music_channel_sequence_2.
- `step_music_channel_sequence_2`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume.
- `update_sprite_animation_and_dma`: Updates an animation state by reading from a table, modifies a structure in a0 with new sprite data and timing, updates the DMACON register, and calls another routine via a jump table based on a value from a global table.
- `mask_byte_to_register`: Masks the lower four bits of d1 and stores the result in a byte at an offset of 47 from the address in a6.
- `set_ciaa_bit`: Tests the byte in d1 and, if it is non-zero, sets bit 1 of the CIAA Port A register before returning.
- `clear_ciaa_bit`: Clears bit 1 of the CIAA Port A register.
- `store_negated_byte`: Masks the lower byte of a register, negates it, and stores the resulting word at offset 30 of a structure pointed to by a6.
- `store_masked_byte`: Masks the lower byte of a register and stores the resulting word at offset 30 of a structure pointed to by a6.
- `store_byte`: Stores a byte from a register into a structure at offset 37 of the structure pointed to by a6.
- `store_masked_byte_offset_29`: Masks the lower 6 bits of a byte in a register and stores the result at offset 29 of a structure pointed to by a6.
- `store_masked_byte_a6`: Masks the lower 6 bits of a byte in a register and stores the result at offset 46 of a structure pointed to by a6.
- `calculate_table_pointer`: Masks the lower 6 bits of a register, shifts the value left by 4, and stores the resulting address calculated from a global table into a structure at offset 44 of the structure pointed to by a6.
