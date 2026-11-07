# Game 648 (Switchblade): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00648). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (956528 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 111 raw-word lines |
| Functions named and described | pass, 218 of 218 named (0 author symbols), 218 of 218 described (10 by rule, 206 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6464 of 6464 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9094 of 9432 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18551 |
| code | 35424 |
| data | 5737 |
| padding | 304 |
| unused | 896512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Switchblade.adf:bootblock | 1 | 7 | 1 |
| logged disk load from Switchblade.adf boot-block load | 1 | 128 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 652 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8645 | 194 |

## Cases handled

- Switchblade/Switchblade.adf: the boot block reads 2048 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 525 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5939 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Switchblade/Switchblade.adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Switchblade.adf:bootblock

- `initialize_boot_block`: The routine writes a byte from a register through a pointer and then issues an Exec DoIO request.

### logged disk load from Switchblade.adf boot-block load

- `manage_library_resources`: Performs library opening and closing and manages DMA and certain hardware registers including custom chiper and certain peripheral or disk status registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_clear_input_buffer`: Jump stub: passes control straight on to clear_input_buffer.
- `process_script_command`: The routine drives a byte-code interpreter that walks a script stored in RAM. It decodes opcodes, updates interpreter state variables, and dispatches work to other routines while manipulating Amiga custom hardware registers DMACON and INTENA to control DMA and interrupts.
- `clear_global_tables`: Clears several global variables and a series of memory locations by writing zero to them.
- `reset_audio_dma_state`: The routine clears a global variable and four audio channel volume and period registers, then zeroes several RAM locations used for audio DMA state.
- `decrement_counter_and_update_value`: Decrements a global counter and, if a certain condition or certain value is met, updates a global variable and a destination memory location with a clamped value or a sum of several global values.
- `decrement_timer_and_dispatch`: The routine checks a flag in a structure and returns if it is clear.
- `patch_table_entry`: The routine checks a counter stored in a table entry and, if the counter is positive, decrements it and writes a flag to a global variable.
- `set_flag_and_accumulate_short`: The routine writes a constant word to an offset in a structure pointed to by an address register and then adds a short value from another structure pointed to by a different address register to a short field in the first structure, storing the sum back in that field.
- `decrement_counter_and_return_flag`: The routine reads a counter from a global variable, decrements it while it is positive and returns a value derived from the variable with a flag bit set.
- `decrement_counters_and_flip_sign`: The routine decrements two short counters stored in a structure pointed to by an argument register and accumulates values from other fields of the same structure into a running total.
- `disable_dma_and_interrupts`: The routine writes to DMACON to disable DMA channels and writes to INTENA to disable interrupts, then writes to INTREQ and INTREQR to clear interrupt requests.
- `decrement_counter_and_reset_state`: The routine checks a counter and, if it is non-zero, decrements it and tests for zero.
- `decrement_counter_and_reset_table`: The routine decrements a counter and, when it reaches zero, reloads it and decrements a second counter.
- `clear_input_buffer`: The routine clears a block of RAM used as an input buffer and initializes a set of control flags and pointers.
- `process_script_interpreter`: The routine processes a script stored in a structure pointed to by a4, interpreting opcodes from a pointer and updating fields in the structure.
- `parse_command_byte`: The routine reads a command byte from a pointer and, based on its value, writes several bytes and flags into a structure pointed to by another pointer.
- `decode_sprite_animation_frame`: The routine reads a byte from an offset in a structure pointed to by A4 and returns it directly if a flag is set.
- `decrement_and_return_counter`: The routine decrements two counter bytes in a structure and, when both reach zero, advances a pointer through a table and updates the counters from the pointed byte.
- `reset_sprite_palette_entry`: The routine computes an index from a byte in a structure and uses it to address a global variable table.
- `clear_sprite_flags_and_advance_pointer`: The routine clears a bit in a byte at an offset from a pointer, writes two bytes derived from a byte register and a byte pointed to by a pointer, advances that pointer by one and stores it back, then returns twice the value of a signed byte at another offset from the pointer.
- `update_timer_counter`: The routine updates a 32-bit timer value stored in a global variable using a 16-bit delta from a pointer, then decrements a signed byte counter and clears its sign bit.
- `decrement_timer_and_schedule_palette_fade`: The routine decrements two countdown bytes in a structure and, when both reach zero, computes a palette fade value from a pointer table, writes several words to custom hardware registers via a base address derived from the table, and stores a callback pointer and parameters in a work area before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_concatenated_registers`: Calls another routine and returns a 64-bit value formed by concatenating the contents of two registers.
- `forward_call_pack_result`: Calls a routine with no arguments, discarding its effects, then returns a 64-bit value built from the two 32-bit register values D0 and D1 as they stand after that call.
- `reset_audio_parameters`: Checks two global variables and, if they meet specific criteria, clears several memory locations and a global variable.
- `manage_channel_tables`: Indexes a table to initialize channel structures and modifies bits in a global variable based on specific conditions.
- `check_global_flags`: Evaluates two global variables to determine if a sequence of nested conditional checks should proceed.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `assign_global_pointer`: Stores the address of the current routine into a global variable.
- `update_structure_from_table`: Iterates through a table to update values in a structure, performing bit tests and modifying pointers or flags based on specific bit patterns.
- `process_structure_data`: Updates values in a structure from a table, performs a bit-test loop on a value, and modifies flags and pointers within a structure.
- `initialize_channel_tables_from_index`: Uses the low byte of the data register to index a table of offsets, sets up three channel structures with pointers and flags, fills a buffer with an incrementing byte pattern, and writes initial values including a final byte.
- `parse_command_stream`: Parses a byte stream to interpret control codes and data values.
- `set_byte_value`: Stores the value one into a memory location pointed to by a register.
- `process_audio_channel_stream`: Parses a compressed or encoded data stream to update channel parameters and buffer pointers.
- `update_channel_pointer`: Checks a byte value and conditionally updates a pointer in a structure using an offset from a table.
- `calculate_table_index`: Performs complex bitwise manipulations and decrements on structure members to calculate and return an index into a table.
- `initialise_channel_record_fields`: Copies several fields from a source record into a channel record, sets two flag bits, and stores a handler pointer.
- `copy_structure_fields`: Copies several fields from one memory area to another based on a bitmask check and decrements a counter.
- `update_structure_data`: Checks a bit in a structure, copies several values from that structure into another memory area, updates a global variable, decrements a counter, and calls a function pointer from the structure.
- `step_and_clamp_counter`: Adds a signed byte to a counter and clamps the result if it falls below zero or exceeds a specified limit.
- `update_clamped_value`: Calculates a sum of two bytes, clamps the result based on a comparison with another byte, and stores a routine address into a structure if a specific condition is met.
- `assign_function_pointer`: Checks a word at an offset within a structure and, if it is zero, stores a routine address into a pointer at a different offset.
- `update_accumulator_value`: Adds two bytes together, stores the result, and conditionally updates a structure field and a pointer.
- `clear_structured_memory`: Executes several routines and then enters a loop that writes specific values to non-contiguous memory blocks.
- `shift_array_elements`: Shifts a series of data structures within a memory block by moving elements to new offsets.
- `shift_list_elements`: Shifts elements within a data structure by moving memory blocks to accommodate new entries.
- `manage_dynamic_array_and_parse_input`: Shifts elements within a data structure to maintain a sorted or ordered list and processes input bitmasks to update coordinate-based indices.
- `parse_input_and_update_buffer`: Processes input bitmasks to increment or decrement two counters, then uses these counters to index into a table and build a string.
- `shift_data_structures`: Executes a sequence of several routines.
- `clear_memory_segments`: Clears four non-contiguous blocks of memory by writing zeros to them in a loop.
- `clear_memory_blocks`: Clears four large, non-contiguous blocks of memory by writing zeros to them in a loop.
- `set_global_byte`: Stores a specific byte value into a global variable.
- `update_global_state`: Checks bits in a global variable and modifies it or other values based on the result.
- `set_global_constant`: Sets a global variable to a specific value based on the state of a register.
- `process_input_state`: Evaluates bits within a global variable and hardware input registers to calculate and store new values into a routine address and a global byte.
- `set_table_flags`: Iterates through a table and sets specific bits in two different offsets within each entry until a high bit is encountered.
- `update_blitter_parameters`: Processes a table of data to modify bit flags and calculate new values for hardware registers including blitter mode, pattern, and size.
- `update_object_trajectories`: Iterates through a linked list of data structures to update coordinate offsets and handle boundary conditions.
- `tiny_andi`: Small routine of 1 instruction that masks d0 with 7.
- `mask_byte_bits`: Performs a bitwise OR on a byte in a register with a constant value.
- `update_input_and_script_state`: Increments a global variable and checks two specific global flags to determine whether to execute another routine.
- `wait_for_global`: Enters an infinite loop if a global variable is less than a specific value.
- `toggle_global_bit`: Enters a loop until a global variable reaches a certain value, then performs multiple bitwise XOR operations on a byte and a global variable.
- `shift_color_data`: Iterates through a source table and stores the left-shifted values into a global variable array, affecting a color register.
- `transform_table_data`: The routine performs a series of bitwise operations on a sixteen-word table, applying conditional increments based on bitmask comparisons and then shifting the resulting values left by one bit before storing them into a global variable.
- `transform_color_table`: Zeros out a table and then performs a multi-pass transformation on sixteen words by applying conditional increments based on bitmasks and shifting values.
- `transform_and_copy_table`: Copies a block of memory to itself, then performs bitwise reductions on a sixteen-word table before shifting the results left and storing them in a global variable.
- `clear_memory_block`: Clears a large block of memory by masking values and updates several global pointers and variables.
- `execute_large_initialization`: A long routine that clears a memory block using bitwise masking and performs multiple pointer assignments.
- `perform_no_operation`: Performs a logical OR operation on a data register with zero, which results in no change to the register's value.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d3.
- `clear_memory_bits`: Performs a nested loop to clear the top two bits of words in a large memory block and updates several global pointers and variables.
- `set_high_bit_pattern`: Iterates through a memory block and sets the highest bit of words that match a specific bit pattern.
- `initialize_blt_attribute_tables`: Clears multiple memory blocks used for Blitter attribute data through nested loops.
- `fetch_indexed_value`: Calculates an offset using a global pointer and a multiplier to return a word from a specific memory location.
- `execute_sequence_of_routines`: Checks a global variable and then executes a specific sequence of other routines while modifying a global variable.
- `setup_blt_parameters`: Sets several global variables and interacts with hardware registers used for bitblock transfers.
- `initialize_pointer_table`: Calls a routine to obtain a value, handles a fallback if the value is a specific constant, and then calculates and stores a pointer based on that value.
- `initialize_memory_block`: Fills a large memory area with all ones and then performs a nested loop to copy data from one location to another.
- `initialize_and_process_color_table`: Sets several global variables and enters a loop that processes a table of pointers.
- `load_data_from_offset`: Checks a global variable and a status flag, then uses a value to index into a table to load several words into global variables.
- `wait_for_condition_loop`: Executes a series of routines, sets a global variable, and then enters loops that wait for a routine to return zero or for a bit in a global variable to be set.
- `fill_memory_pattern`: Iterates through a memory range to write a repeating pattern of bytes.
- `tiny_ori_3`: Small routine of 1 instruction that merges 220 into d4.
- `process_string_buffer`: Iterates through a nested loop structure to check bytes in a buffer and calls another routine if a byte is non-zero.
- `iterate_pointer_table`: Iterates through a table of pointers until a terminator is found, performing nested loops and bitwise checks on the data.
- `copy_data_with_offset`: Traverses a list of pointers to copy bytes from one location to another using an offset and a terminator check.
- `update_global_state_2`: Checks specific bits of a global variable to set it to zero or one before calling another routine.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 255 into d0.
- `initialize_data_table`: Initializes several memory locations and a table with specific constant values.
- `initialize_global_constants`: Sets numerous global variables and memory locations to specific constant values and patterns.
- `execute_unnamed_routines`: Executes a sequence of six calls to other routines.
- `fill_buffer_with_spaces`: If a global variable is non-zero, it clears that variable and fills a memory area with space characters.
- `transform_global_variable`: Multiplies a global variable by two and adds constant character values to it before calling another routine.
- `reset_global_variable`: Checks if a global variable is non-zero, calls another routine, and then clears the global variable.
- `accumulate_bcd_value`: Extracts bytes from a longword and adds them to a global variable using BCD adjustment.
- `clear_data_register`: Executes an instruction that logically ORs a data register with zero.
- `clear_global_variable`: Checks if a global variable is non-zero and, if so, calls another routine and sets the variable to zero.
- `fill_memory_with_pattern`: Fills a range of memory starting at a global variable with a specific byte value based on the contents of that variable.
- `double_call_and_clear`: If a global variable is non-zero, calls another routine twice and then clears the global variable.
- `fill_pattern_buffer`: Calculates a length based on a global variable and fills a memory range with a specific byte pattern.
- `process_value_range`: Performs conditional checks on a global variable to set a two-byte value based on its magnitude.
- `update_table_entry`: Compares a value in a table with a constant and updates the table entry or a global variable based on the result.
- `initialize_and_adjust_data`: Initializes a table and several memory locations, performs a BCD adjustment on a global variable, and sets a value to a constant.
- `initialize_global_data`: Checks a global variable and, if zero, populates a memory location with a specific longword value.
- `decrement_bcd_value`: Decrements a global variable using BCD adjustment in a loop until it reaches a negative value.
- `reset_system_state`: Sets multiple global variables and pointers to zero or specific constant values.
- `set_subroutine_pointers`: Updates several global variables and pointers to specific routine addresses and values.
- `update_control_variables`: Modifies a global variable using a bitwise OR and clears several other memory locations.
- `configure_runtime_parameters`: Assigns specific values and routine addresses to a series of global variables.
- `modify_global_state`: Performs bitwise operations on a global variable and resets various pointers and values.
- `clear_memory_structures`: Sets a collection of global variables and memory addresses to zero or specific constants.
- `tiny_ori_4`: Small routine of 1 instruction that merges 24 into (a7)+.
- `reset_system_state_2`: Modifies a global variable using bitwise operations and clears or assigns various global pointers and constants.
- `update_system_state_counters`: Updates several global counters and state variables based on conditional range checks.
- `process_system_state`: Performs complex memory management, table indexing, and attribute retrieval.
- `validate_word_range_2`: Checks if a value falls within a specific numerical range.
- `validate_word_range`: Checks if a value stored in a register falls within a specific numerical range.
- `validate_word_pattern`: Checks if a longword is not all ones and has its highest bit clear before calling another routine.
- `validate_input_range`: Checks a value against a set of bounds defined in a table.
- `validate_range_bounds_call`: This routine performs a call to a function that validates a value against a set of bounds stored in a table.
- `validate_input_range_2`: Checks a value against a set of bounds defined in a table via a pointer.
- `validate_range_value`: Checks if a value in a register is within specific bounds defined by a table.
- `validate_range_value_2`: Verifies a register value against three entries in a table to ensure it is within bounds.
- `validate_range_bounds`: Compares a value against bounds stored in a table pointed to by a register, checking multiple possible entries.
- `validate_range_limits`: Checks if a value in a register is within specific bounds defined by three different entries in a table.
- `check_value_range`: Verifies if a value extracted from a table entry falls within a specific numeric range.
- `initialize_state_constants`: Sets a global variable to a fixed value, copies a value from another global variable, and clears a third global variable.
- `reset_global_pointers`: Sets two global variables to zero and updates another using a value from a third global variable.
- `increment_global_pointers`: Updates two global variables by adding offsets to a masked value from a third global variable.
- `update_masked_value`: Calculates a value by subtracting a register from a constant and stores the result masked to four bits in a global variable.
- `update_masked_negative`: Calculates a value by adding a constant to a register, negating it, and stores the four-bit masked result in a global variable.
- `update_masked_constant`: Calculates a value by subtracting a register from a constant and stores the four-bit masked result in a global variable.
- `update_masked_index`: Calculates a value by adding ten to a register, applying a bitwise AND mask, and storing the negated result into a global variable.
- `fetch_next_table_entry`: Copies a value from a table into a global variable and increments a table index, resetting the index if a terminator is reached.
- `update_runtime_state_and_pointers`: Updates various global state variables and pointers based on conditional checks of register values and existing global flags.
- `fetch_object_attribute_data`: Calculates an offset into a data table based on a global index and retrieves various attributes including short integers and bytes.
- `manage_system_state`: Performs complex conditional logic to clear memory, update control variables, and reset various global pointers and state values.
- `initialize_memory_state`: Sets a global variable using a bitwise OR and resets two other global variables to zero before setting a third to a constant.
- `call_another_routine`: An empty wrapper that simply calls another routine and returns the concatenated results of two registers.
- `copy_data_list`: Iterates through a list of data pointed to by a register, calling another routine until a terminator value is reached.
- `copy_data_loop`: Traverses a pointer-based list and calls another routine repeatedly until a specific byte is encountered.
- `return_empty_value`: This routine is empty and performs no operations before returning.
- `copy_structured_data`: Copies a series of bytes from a source buffer to a destination buffer using a non-contiguous pattern of offsets.
- `initialize_global_values`: Stores two specific longword values into two consecutive global variables.
- `update_obfuscated_values`: Performs bitwise shifts and arithmetic operations to update two global variables based on their current contents.
- `clear_data_table`: Iterates through a table, zeroing out specific offsets within each entry until a termination marker is encountered.
- `process_data_list`: Iterates through a list of structures, calling routines via a table based on an offset, and performs bitwise operations on memory.
- `process_table_entries`: Scans a table to identify specific bit patterns, then populates a secondary structure and calls a function pointer retrieved from a table.
- `process_obfuscated_data`: Calculates an offset based on a global variable to access a table, then performs bitwise checks and updates several memory locations.
- `update_object_data`: Decrements a BCD value, updates obfuscated global variables, and initializes object data slots.
- `initialize_and_update_object_slots`: Initializes a memory structure by clearing specific fields, setting coordinate offsets, and populating attribute data.
- `process_entity_state_and_dispatch`: Updates entity status flags and coordinate offsets within a structure while managing a lifecycle counter.
- `update_object_state_and_bounds`: Updates an object's internal state and position by calculating offsets from a base address and comparing coordinate values against defined boundary limits.
- `update_entity_state_machine`: Manages an entity's lifecycle and state transitions by indexing a data table and updating position, velocity, and status flags.
- `update_entity_physics_and_state`: Updates an entity's position, velocity, and state flags based on its current status and collision or boundary conditions.
- `modify_structure_fields`: Sets a byte at an offset in a structure and conditionally sets another byte based on a comparison.
- `update_entity_state_logic`: Manages entity state transitions by evaluating a state byte and updating coordinate offsets, bounding box dimensions, and status flags.
- `check_range_bounds`: Performs multiple conditional checks against memory offsets and global values to determine if a range is valid.
- `set_structure_word`: Calls another routine and then writes a word to a specific offset within a structure.
- `process_entity_state_transitions_2`: Updates an entity's state, coordinate data, and bounding box based on a state byte and various conditional flags.
- `update_structure_offsets_2`: Updates various fields in two structures, including conditional negation of a word and copying a longword.
- `copy_word_offset`: Copies a longword from an offset of 44 in one structure to an offset of 32 in another.
- `update_entity_state_and_bounds`: Updates an entity's state byte and coordinate data based on conditional checks of flags and range bounds.
- `validate_and_update_coordinate_bounds`: Performs range checks on coordinate values against specific table entries and global boundaries.
- `validate_coordinate_bounds`: Performs range checks on short integer values against various global boundaries and table entries.
- `reset_global_variable_2`: Sets a global variable to zero and performs range validation checks.
- `process_entity_state_transitions`: Updates an entity's state byte and coordinate data based on conditional checks of flags and range bounds.
- `update_structure_state`: Updates a byte and a longword within a structure based on the value of a flag at a specific offset.
- `clear_structure_field`: Clears a byte in a structure and performs range validation checks.
- `update_structure_coordinate_offsets_2`: Updates a structure's coordinate field by selecting between different memory offsets based on a flag and comparing a global value against a range.
- `add_offset_to_value`: Adds a value from one memory location to another and performs range validation checks.
- `update_structure_coordinate_offsets`: Updates a structure's coordinate field by selecting between different memory offsets based on a flag and comparing a global value against range bounds.
- `select_structure_data`: Selects and copies a longword into a structure based on comparisons between a global value and various offsets.
- `copy_longword_to_offset`: Copies a longword to a specific offset in a structure and performs range validation checks.
- `update_conditional_data`: Updates obfuscated values and copies a longword to a structure based on a comparison.
- `copy_word_offset_alt`: Copies a longword from an offset of 16 in one structure to an offset of 32 in another.
- `validate_range_bounds_3`: Performs multiple conditional checks against various memory offsets and global values to determine if a range is valid.
- `check_hit_box_and_clear_flag`: Returns early if a global level value exceeds 9; otherwise tests whether a point derived from two global offsets falls inside a rectangle taken from the structure pointed to by A6 (position and size fields).
- `update_conditional_fields`: Performs multiple range checks on structure fields and global variables to conditionally update a value or call another routine.
- `update_structure_fields`: Clears specific fields in a structure and copies a longword from one location to another based on a conditional check.
- `copy_word_offset_short`: Copies a longword from an offset of 4 in one structure to an offset of 32 in another.
- `reset_structure_fields`: Clears two words and one byte at specific offsets in a structure while copying a word from another structure.
- `verify_coordinate_ranges`: Compares two short integers against ranges defined by global variables and other memory locations.
- `check_coordinate_bounds`: Compares two short integers against specific ranges defined by global variables and other memory values.
- `validate_coordinate_range`: Compares two short integers against two separate sets of bounds defined by global variables.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `validate_and_reset_state`: Checks if a pointer falls within a specific memory range and then clears a global variable and several pointers.
- `validate_range_bounds_2`: Compares values within a structure against global bounds to determine if they fall within a specific range.
- `update_structure_offsets`: Validates structure field ranges, calls another routine, and updates coordinate offsets and status flags based on the results.
- `search_memory_table`: iterates through a table of longwords to find an entry that matches specific bitwise criteria.
- `update_state_byte`: Checks a byte at an offset within a structure and updates it to a new value based on specific character or numeric matches.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_record_pointer`: Stores the address of a global variable into a structure and sets a specific bit in a byte.
- `set_pointer_and_flag`: Stores the address of a global variable into a memory location offset from a pointer and sets the third bit of the value at that pointer.
- `update_data_structure`: Writes several byte and word values into memory locations using two different pointers.
- `compare_and_branch_pointer`: Compares a value at a memory location against a constant and branches based on the result.
- `call_subroutine`: This routine performs a single jump to another routine and then returns.
- `return_from_subroutine_2`: The routine performs a single return instruction to exit the current execution flow.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `check_and_clear_bits`: Compares a value at a memory location against a constant and, if both a pointer and a global variable are non-zero, clears the lowest bit of a series of memory locations.
- `call_pointer_update_handler`: Invokes a routine that updates global variables based on shifted values and selects a pointer via range testing to act as a handler entry point for data block processing.
- `execute_conditional_call`: Checks if a global variable is zero and, if so, calls a routine using a value returned from that call.
- `process_blitter_data`: Iterates through a data block to clear bits, copies structured data, and manipulates hardware registers.
- `populate_data_structure`: copies data from a source list into a series of structured blocks in memory while setting specific bits and values.
- `clear_structure_bits`: iterates through a series of memory locations to clear the lowest bit of each byte.
- `clear_global_bits`: Sets several memory locations to zero and performs a bitwise AND operation on a global variable to clear its lowest bit.
- `reset_global_variables`: sets several global variables to zero and clears the lowest bit of a global variable multiple times.
- `conditionally_clear_variables`: checks several global variables and, if they are non-zero, resets them to zero and clears the lowest bit of another global variable.
- `initialize_object_data_slots`: Populates a series of data structures in memory with coordinates and attribute values retrieved from a table.
- `populate_object_data_slots`: Iterates through a series of available memory slots to initialize them with coordinate and attribute data.
- `copy_string_with_delimiter`: Copies bytes from a source buffer to a destination buffer, handling a specific delimiter byte that triggers a length decrement and a jump in the source pointer.
- `check_global_state`: Evaluates the state of a global variable against a constant and calls another routine based on the result.
- `modify_byte_bits`: Decrements a byte through a pointer, performs a bitwise XOR on a byte using a bit-shifted value, and performs a bitwise AND on a byte through a pointer.
