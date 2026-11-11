# Game 304 (Zaxxon): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00304). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906272 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 283 of 283 named (0 author symbols), 283 of 283 described (15 by rule, 223 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3730 of 3730 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6682 of 6689 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 419682 |
| code | 22952 |
| data | 57214 |
| padding | 1432 |
| unused | 404992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zaxxon.adf:trainer | 3 | 160 | 5 |
| Zaxxon.adf:Game | 1 | 5458 | 250 |
| Zaxxon.adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 512 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 541 | 13 |

## Cases handled

- runtime segment an address: 291 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 393 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zaxxon/Zaxxon.adf:trainer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Zaxxon.adf:trainer

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
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `wait_for_flag_then_halt`: Busy-waits until a global flag becomes all ones, then enters an endless loop that never returns.
- `copy_block_find_table_base`: Copies a block of longwords from an offset inside a larger area into the global area and checks a tag value, copying one byte when it matches.
- `update_audio_and_sprite_attributes`: Iterates through a data structure to update sprite positions, animation frames, and audio parameters, including volume and pitch adjustments.
- `update_sprite_and_audio_parameters`: Iterates through a data structure to update sprite attributes, position, and animation frames based on table lookups.
- `update_entity_state`: Loads entity attributes from a data table into a structure based on an index.
- `copy_table_entry_to_memory`: Copies a word from a table into two different memory locations.
- `iterate_object_properties`: Iterates through a list of objects and calls another routine to update their properties if a specific byte is non-zero.
- `update_interpolated_object_properties`: Updates several object properties by decrementing counters and fetching new values from a table when counters reach zero.
- `calculate_object_offsets`: Copies data from a table to a global variable, finds a maximum value in a list, and then populates a series of memory locations with calculated offsets.
- `initialize_table_entries_from_records`: Sets four global 16-bit-scaled words to a fixed value, then walks a sentinel-terminated table of five-word records, filling a parallel table of 18-byte entries with derived pointers, a halved value, a reciprocal-style quotient, a copied field, and a computed divisor-based value.
- `init_table_and_pointers`: Sets four 32-bit words to the fixed value 65536, then walks a table of 18-byte records terminated by a -1 sentinel, filling each output record with a pointer into a base buffer, a derived halved size and offset, a reciprocal-style value computed by dividing a constant by a record field, and a fixed sentinel value, with a conditional division when a flag field is zero.
- `initialize_data_table`: Assigns a constant to four global variables and processes a list of data into a destination table using arithmetic operations and bit shifts.
- `init_scale_factors_and_table`: Stores the fixed value a fixed address into four consecutive 32-bit globals (the 1.0 fixed-point scale constants), then walks a table of 18-byte records terminated by a -1 sentinel, filling each output record with a pointer, half-sized offsets, a reciprocal-derived value, a flag word, and a derived quotient.
- `populate_object_data`: Iterates through a table to calculate and store derived values, such as offsets and scaled constants, into a series of memory locations.
- `populate_hardware_registers_from_table`: Iterates through a table to fetch data and copies specific values into hardware registers and memory locations.
- `update_hardware_and_object_tables_2`: Iterates through a data table to calculate derived values and updates hardware registers, including copper and DMA settings.
- `calculate_memory_pointers`: Copies a block of data, finds a maximum value in a list, and populates several pointers with calculated memory addresses.
- `update_hardware_and_object_tables`: Iterates through a data table to calculate derived values and updates hardware registers, including copper and DMA settings.
- `initialize_hardware_and_state_tables`: Sets specific values in hardware registers and clears or populates various global memory locations and tables.
- `update_maximum_value`: Compares a value in a register against a value at an offset from a pointer and updates that memory location if the register value is larger.
- `wait_for_bit_pattern`: Enters a loop that waits until a specific bit pattern is detected in a memory location.
- `wait_for_bit_match`: Enters a loop that continues until a specific bit pattern in a memory location matches a value derived from a register.
- `wait_for_bit_clear`: Enters two consecutive loops that wait for a specific bit in a byte at a memory offset to become zero.
- `rotate_bits`: Performs bitwise operations on a global variable to clear a bit and then performs a circular bit rotation.
- `clear_data_register`: Clears the low byte of a data register.
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
- `reset_sprite_and_state_table`: Initializes several global state and coordinate variables to fixed starting values, selecting alternate values when a flag is set, then clears a block of four-entry records by writing paired constants and zeroing a counter field.
- `initialize_memory_tables`: Sets several global variables to specific constant values and then enters a loop to fill a memory block with a repeating pattern of words and zeros.
- `clear_memory_tables`: Clears a block of memory with zeros and then performs loops to fill specific memory regions with either zero or a constant value based on the contents of a global variable.
- `clear_and_fill_tables`: Clears a block of memory with zeros and then performs two separate loops to fill different memory regions with the value zero or a specific constant based on the contents of a global variable.
- `initialize_state_parameters`: Sets various global pointers, counters, and constants based on the value of a global variable.
- `clear_d0_register`: Performs a logical OR operation on a data register using a zero immediate value.
- `execute_nested_loops`: Selects a function pointer from a table based on a global value and executes it within nested loops that decrement a global counter.
- `pop_data_stack`: Decrements a counter and, if it is not zero, retrieves a word from a table and updates two other variables with the retrieved values.
- `update_game_state_and_counters`: Updates multiple global counters and variables by incrementing or decrementing them based on conditional checks.
- `clear_data_register_2`: Sets the low byte of a data register to zero.
- `manage_game_state_and_counters`: Updates multiple global counters and state variables through conditional decrements and increments.
- `update_dynamic_counters`: Decrements and increments various global counters and table values based on specific conditional checks.
- `update_loop_counters`: Increments and decrements multiple global counters and performs arithmetic on memory locations within a loop.
- `decrement_table_values`: Iterates through a table of sixteen-bit values, checking if a specific byte is set, and if so, decrements the associated word by eight if it remains above a certain threshold.
- `adjust_pointer_values`: Iterates through a table of four short integers and, if a value equals two, decrements a corresponding word in another memory area by eight.
- `process_loop_counters`: Updates several global counters and memory locations within a loop based on specific conditions.
- `copy_graphics_data`: Calculates source and destination addresses based on several global variables and performs a structured copy of data blocks into a target memory range.
- `call_indirect_table`: Calls a routine and then performs an indirect call using a value from a table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_offset_data`: Performs arithmetic on a value to determine memory offsets and performs bounds checking against global ranges.
- `clear_and_copy_data`: Clears a block of memory with zeros, initializes several global variables, and copies a sequence of words from one location to another.
- `calculate_and_store_offset_data`: Calculates several memory offsets through arithmetic operations on a value and performs bounds checking against global ranges.
- `populate_data_structure_2`: Searches a table for a zero word and populates a structure at that location with several global values.
- `update_coordinate_data`: Retrieves a value from another routine and writes several words and a long value to a memory block at a specific offset.
- `store_result_to_memory`: Takes a value from another routine and stores it at a specific offset in memory along with a constant value.
- `initialize_complex_structure`: Sets several global variables and populates a memory structure with values derived from conditional checks.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `store_long_values`: Retrieves a long value and stores it along with several other global variables and constants into a memory structure.
- `store_long_result`: Retrieves a value from another routine and stores it as a 32-bit long value in a memory location, accompanied by other global data.
- `clear_data_register_3`: Executes an or instruction to zero a data register.
- `convert_byte_quads_to_scaled_coords`: Reads four-byte records from a source byte buffer and writes each record as four 16-bit values into a destination table, scaling the first and third bytes by eight with an offset and shifting the second and fourth bytes left by six.
- `copy_scaled_coordinate_table`: Walks a packed source table of byte coordinate records, scaling each value by a fixed factor and writing the results as 16-bit words into a destination table; it also copies a trailing 32-bit value and two further bytes, then advances the source pointer past the consumed records.
- `copy_and_scale_vertex_records`: Reads a sequence of byte-sized coordinate records from a source pointer, scales each value into 16-bit words and writes them to a destination buffer, then copies a trailing 4-byte value and two 16-bit fields and advances the source pointer past the consumed records.
- `transform_data_table`: iterates through a source table to calculate and store new values into a destination table using bit shifts and arithmetic.
- `process_data_table`: Iterates through a table of words, performing arithmetic transformations on each element and storing the results in a new memory location.
- `populate_data_structure`: Initializes several global variables and then executes a loop to fill a table with a repeating sequence of seven words containing incrementing and constant values.
- `process_event_script_and_state`: Evaluates a state machine based on a global status variable and iterates through a data table to execute scripted commands.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 128 into -(a0), then merges 128 into d0, then merges 72 into d0.
- `update_status_flag`: checks a global variable and performs a bitwise XOR on another global variable if certain conditions are met, then decrements a counter.
- `process_bitmask_coordinate_offsets`: Reads a value from a pointer and uses its bits as flags to apply additive or subtractive offsets to a coordinate value.
- `clamp_coordinates`: checks if two values fall within specific bounds and stores them in a table, otherwise stores default values.
- `adjust_values_incrementally`: compares values in two tables and adjusts a third table by adding or subtracting a constant value based on those comparisons.
- `clear_data_register_4`: Clears a data register using a logical or instruction.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_multiple_registers`: Performs two consecutive or operations to clear a data register.
- `update_entity_list`: Iterates through a table to check status flags and performs memory copies and global updates if certain conditions are met.
- `check_bounds_collision`: Compares several values against a set of bounds stored in a table to determine if a coordinate is within range.
- `update_entity_status`: Iterates through a table of data to check status flags and increments a value at a pointer-indexed location if certain conditions are met.
- `update_entity_states`: Iterates through a table of data to check specific status flags.
- `tiny_ori_ori_ori_3`: Small routine of 3 instructions that merges 64 into -(a0), then merges 128 into d0, then merges 48 into -(a0).
- `validate_and_update_position`: Updates a global index and checks if a coordinate pair falls within specific numeric bounds before storing it.
- `validate_and_store_coordinates`: Modifies a value via an offset and checks if two resulting values fall within specific numeric bounds.
- `validate_position_bounds`: Checks global values against specific bounds and, if valid, calculates a new position using a table and stores it in a global variable.
- `advance_linked_list`: Traverses a list to find the next non-zero entry and updates a pointer table with the new values.
- `update_list_pointer`: Searches for a non-zero value in a list and updates a table with the resulting pointer and offset.
- `initialize_and_clamp_entity_table`: Initializes a table with default values if a specific global counter reaches a threshold, then iterates through a loop to process bitmask-driven data.
- `increment_pointer`: Increments the value at the address pointed to by a register by four.
- `increment_word_via_pointer`: Increments a 16-bit value at a memory location pointed to by a register by four.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to (a0), then returns.
- `tiny_subq_rts`: Small routine of 2 instructions that subtracts 4 from (a0), then returns.
- `update_entity_status_flags`: Iterates through a table of data to check for specific status values and increments a long value at a pointer-indexed location.
- `clamp_position_values`: Checks if a pair of values falls within a specific range and stores them as a long value, otherwise stores a default constant.
- `clamp_coordinate_values`: Checks if two values fall within specific bounds and stores them in a table, otherwise stores default values.
- `fetch_table_data`: Checks several global conditions and a counter before retrieving a long value from a table using an index.
- `fetch_table_entry`: Checks a global counter and, if a condition is met, retrieves a long value from a table using an index derived from a global variable.
- `update_position_data`: Increments a global variable with a wrap-around check and performs arithmetic on several global values to store a result in a table.
- `process_event_list_and_update_state`: Iterates through a data table to check for specific attribute matches and performs state transitions based on a control variable.
- `reset_state_variables`: Sets a pointer to zero and assigns specific constant values to two global variables.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into -(a4).
- `update_slot_score_and_flags`: Sets two flag bits in memory and, when the global state word equals 2, scans four fixed-stride table entries; for the first entry whose active word is nonzero and whose secondary guard is set, it adds 100 to a score word indexed by a table field, toggles a byte of a related slot, clears the active word and writes two copies of a constant into the entry, then sets a return-state value and returns.
- `update_status_flags`: Sets specific bits in two memory locations via pointers held in registers.
- `update_status_flags_2`: Sets the fourth bit of two bytes located at memory addresses derived from registers.
- `process_status_events`: Checks a global variable for a specific state and iterates through a table to find non-zero entries.
- `process_loop_sequence`: Iterates through a sequence of data, updating a counter and performing conditional checks on global variables to manage a state machine.
- `initialize_global_values`: Sets a global byte to zero and assigns a specific value to another global byte.
- `initialize_global_values_2`: Sets two global variables to zero and a third global variable to a specific constant value.
- `reset_global_byte`: Writes zero to a global variable.
- `assign_function_pointer`: Sets a global variable to zero, treating it as a code pointer.
- `reset_global_values`: Sets a global byte to zero and assigns a constant value to another global variable.
- `increment_status_counters`: Iterates through a table to find entries with a specific status and increments a byte value.
- `parse_scripted_event_table`: Iterates through a table of command structures to execute conditional logic and update global state variables.
- `process_linked_list_and_bounds`: Traverses a linked list structure to update values and performs bounds checking on coordinates before storing them in a table.
- `increment_and_fetch_table_entry`: Increments a global byte and returns a concatenated value from a table using the previous value as an index.
- `execute_jump_table`: Iterates through a table of entries to find a specific marker and then calls a function via an indirect pointer.
- `search_and_execute_jump_table`: Iterates through a list of data to find a specific marker and then executes an indirect call based on an offset.
- `search_memory_pattern`: Iterates through a memory block in fixed increments to search for a specific byte pattern.
- `process_linked_list`: Traverses a linked list by updating a pointer and a counter.
- `check_memory_bounds`: Compares a value derived from a pointer against a global variable and performs bounds checking against two other memory-related values.
- `update_object_state_parameters`: Initializes a data structure with several constant and register-derived values, then updates specific fields within that same structure after a subsequent routine call.
- `store_data_structure`: Writes several values, including a modified register and a pointer, into a data structure at a specific offset.
- `process_sprite_data`: Calculates memory offsets and performs bitwise shifts to copy and mask data from a source table into a destination buffer.
- `update_sprite_data`: Calculates an offset based on global values and performs bitwise shifts to copy data from a source table into a destination buffer.
- `initialize_and_copy_sprite_data`: Populates a data structure with configuration values and performs a bitwise-shifted copy of data from a source table to a destination buffer.
- `initialize_object_table`: Populates a data structure with pointers and configuration values by iterating through a source table.
- `initialize_object_pointer_tables`: Sets several global pointers to a base address and populates memory tables with a sequence of pointers and offsets.
- `populate_pointer_table`: Sets global pointers and populates a table with values from a source list through a loop.
- `assign_global_pointers`: Sets two global pointers to the address of a global variable based on a comparison of two values.
- `sort_word_list`: Performs a bubble sort on a list of 32-bit values in memory.
- `store_register_pairs_to_globals`: Stores pairs of 16-bit values from registers into a sequence of global variables by calling another routine multiple times.
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
- `clear_data_register_5`: Executes two or instructions targeting a data register.
- `wait_and_execute_loop_2`: Decrements a global variable in a loop until it falls below a threshold, then executes another routine a fixed number of times.
- `wait_and_execute_loop`: Decrements a global variable in a loop until it falls below a threshold, then executes another routine a fixed number of times using a pointer-based loop.
- `copy_data_loop`: Calculates a destination address based on a register and a global variable, then copies eight words from a source to that destination using a loop.
- `execute_repeated_calls_2`: Executes another routine a fixed number of times in a loop, with conditional logic based on the value of a global variable.
- `execute_repeated_calls`: Executes another routine a fixed number of times based on a value from a global variable.
- `execute_conditional_calls`: Performs a loop that executes another routine multiple times based on a value from a global variable.
- `copy_interleaved_words`: Copies eight words from a source address calculated from two input registers to a destination address.
- `execute_conditional_loop`: Decrements a global variable and, if certain conditions are met, executes a loop that calls another routine and stores a value.
- `compare_nibbles`: Compares the lower four bits of two input words.
- `copy_and_clear_memory`: If a global flag is zero, it copies a block of data from a global variable to a destination address and then sets the flag to a new value.
- `store_global_address`: Stores the address of a global variable into another global variable.
- `store_global_address_pointer`: Stores the address of a global variable into another global variable, serving as a pointer assignment.
- `assign_global_address`: Stores the memory address of a global variable into another global variable.
- `clear_and_copy`: Sets a global variable to zero and then copies a sequence of words from a global variable to itself using an incrementing pointer.
- `clear_data_register_repeatedly`: Executes a sequence of or instructions to clear a data register.
- `clear_data_register_low_byte`: Clears the lower byte of a data register to zero.
- `tiny_ori_ori_or_ori`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges d0 into d5, then merges 0 into d1.
- `copy_memory_block`: Copies a sequence of words from a global variable to itself using a decrementing counter.
- `copy_memory_block_2`: Copies a sequence of words from a global variable to itself using a decrementing counter.
- `copy_data_to_self`: Copies a sequence of words from a global variable to itself using a loop.
- `copy_memory_block_3`: Performs a word-by-word copy of a data block.
- `clear_memory_block`: Fills a block of memory with zeros.
- `build_relative_address_table`: Calculates cumulative offsets from a list of integers and then converts those offsets into absolute pointers within a table.
- `initialize_sprite_table`: Clears a large memory block and populates a table with structured data derived from a source list.
- `initialize_asset_lookup_tables`: Clears several memory blocks and populates a table with calculated offsets and parameters derived from a source data structure.
- `initialize_sprite_attribute_table`: Clears a large memory block and populates a table with calculated sprite attributes, including screen coordinates, priority, and animation frame data.
- `initialize_sprite_data_table`: Clears a large block of memory and populates a table with calculated values derived from a source list.
- `initialize_sprite_data_table_2`: Clears a large memory block and populates a table with calculated sprite attributes, including offsets, dimensions, and scaling factors derived from a source data list.
- `initialize_engine_subsystems`: Clears several memory blocks and populates a table with calculated offsets and parameters derived from a source data structure.
- `transform_global_byte`: Performs bitwise operations on a global variable and returns the result.
- `wait_for_memory_pattern`: Polls a memory location via a pointer until specific bit patterns are detected in sequence.
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
- `return_register_value`: Returns the current value held in a data register to the caller.
- `return_without_action_2`: This routine is empty: its only instruction is an immediate return, so it performs no work and has no side effects.
- `initialize_data_copy`: Sets up indices and pointers to perform a block data copy from a table to a destination area.
- `initialize_data_copy_offset`: Sets up indices and pointers to perform a block data copy using a specific offset from a table.
- `copy_data_block_2`: Copies a block of data from a source table to a destination memory area based on calculated offsets.
- `transfer_memory_segment`: Copies eight words of data from a source address to a destination address using a loop.
- `manage_callback_list`: Iterates through a list of structures to find and update a specific entry, then executes a loop of calls.
- `process_string_data`: Reads a byte from a pointer, updates a destination with the byte and associated data, and increments the pointer.
- `copy_stack_data`: Copies several double-word values from one memory location to another and performs a loop that calls another routine multiple times.
- `copy_stack_and_loop_data`: Copies several 32-bit words from one stack location to another and then executes a loop that calls another routine while iterating through a table of values.
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
- `reset_dma_and_buffer_flags`: Clears the low byte of a data register and two bytes at a pointer location, then writes a single-byte value to a sequence of adjacent memory locations or a small table.
- `configure_dma_channels`: Updates DMA descriptors and interacts with audio and DMA control registers to manage data transfers.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d1, then merges 0 into d1.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.
- `clear_alternate_register`: Clears the low byte of a different data register.
- `calculate_table_values`: Iterates through a source table to perform various arithmetic operations and stores the results in a destination table.
- `initialize_table_pointers`: Sets three global variables to a table address and two specific constant values.
- `store_memory_addresses`: Stores three consecutive memory addresses into a global variable sequence.

### Zaxxon.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_idle`: Clears a small block of state, sets up several global words and writes DMA and interrupt enable values.
- `set_flag_then_initialize_system`: Sets bits in a flag byte inside the base-addressed block, then runs the same setup as the segment entry: it clears state, writes DMA and interrupt enable values, scales a table, and ends in an endless wait loop.
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
- `tiny_ori_2`: Small routine of 1 instruction that merges 246 into d0.
- `set_bit_refill_random_table`: Sets one bit in a byte, then fills sixteen longwords by repeatedly rotating a state word, adding a constant and exclusive-ORing another.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_sound_command`: Dispatches on the low two bits of its input.
- `initialize_channel_structure`: Clears several fields of a channel structure, stores a pointer taken from a second structure, and sets a fixed longword, a mask byte and a flag byte.
- `generate_pseudo_random_sequence`: Fills sixteen longwords by repeatedly rotating a state word, adding a constant and exclusive-ORing another, while decrementing several byte counters and setting flags when they reach zero.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `store_nibble_and_mask_return`: Stores the low four bits of its second input into a field of the structure, and returns the first input with a fixed set of bits turned on.
- `store_low_nibble_field`: Stores the low four bits of its input into a byte field of the structure addressed by the base register.
- `set_or_clear_global_bit`: Sets bit one of a global byte when the input is zero, and clears that bit otherwise.
- `store_negated_low_byte`: Stores the negated low byte of its input into a word field of the structure addressed by the base register.
- `store_low_byte_word_field`: Stores the low byte of its input into a word field of the structure addressed by the base register.
- `store_input_byte_field`: Stores the input byte into a fixed byte field of the structure addressed by the base register.
- `store_low_six_bits_field`: Stores the low six bits of its input into a byte field of the structure addressed by the first address register.
- `store_six_bits_at_base`: Stores the low six bits of its input into a byte field of the block addressed by the base register.
- `store_table_pointer`: Masks its input to six bits, multiplies it by sixteen, adds a fixed base address, and stores the result as a pointer in a structure field.
