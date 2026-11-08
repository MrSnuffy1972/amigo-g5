# Game 149 (AlienBash v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00149). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (621896 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 223 of 223 named (0 author symbols), 223 of 223 described (14 by rule, 207 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1639 of 1639 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3748 of 3749 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 206926 |
| code | 16702 |
| data | 398172 |
| padding | 96 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AlienBash | 2 | 3542 | 198 |
| game.slave | 1 | 104 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 72 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 5 |

## Cases handled

- AlienBash_v1.0/AlienBash_v1.0.hdf: hard-disk image (AmigaDOS OFS, 672 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree AlienBash_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### AlienBash

- `initialize_program_state`: Sets a global variable and calls a routine via a pointer, then conditionally executes several other routines and updates two more global variables.
- `store_address_to_global`: Stores the current value of a register into a global variable.
- `poll_ciaa_status`: Repeatedly reads a global variable and calls another routine while a specific bit in that variable remains set, involving the CIAA port.
- `monitor_and_reset_state`: Enters a loop that monitors a global variable and a hardware register, eventually resetting several global variables to zero if a specific condition is met.
- `wait_for_condition`: Enters a loop that monitors a global variable and a hardware register, eventually resetting several global variables to zero if a specific condition is met.
- `loop_until_zero`: Repeatedly calls another routine until a global variable reaches zero, then clears that variable.
- `initialize_global_data`: Fills a table with a sequence of pointers incremented by fourteen bytes, sets several global variables to specific constants, and stores a global variable address into a register.
- `clear_memory_table`: Fills a series of word-sized memory locations with zero using a loop.
- `load_data_from_pointer`: Loads seven consecutive words from a memory location pointed to by a table entry into several global variables.
- `run_sequential_subroutines`: Executes a sequence of subroutines while decrementing a counter and updating various global variables and status flags based on specific conditions.
- `set_constant_value`: Stores a constant value into a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `reset_state_structure`: Executes a sequence of routines, performs a loop until a global variable reaches a specific value, and resets several global variables and counters.
- `invoke_data_copy`: Calls a routine that populates a destination structure using three source pointers and initializes status flags.
- `call_data_copy_routine`: Calls a routine that copies data from three source pointers into a destination structure and sets status flags.
- `call_data_copy_routine_2`: Calls a routine that copies data from three source pointers into a destination structure and sets status flags and default values.
- `execute_conditional_loop`: Executes a series of subroutine calls within a loop that terminates based on the state of specific bits in a global variable.
- `reset_global_flag`: Repeatedly calls another routine based on bits in a global variable before setting that variable to a constant value.
- `setup_callback_tables`: Populates several memory locations with pointers, sets multiple global variables to specific word values, and initializes a structure with a function pointer and associated data.
- `update_display_buffer`: Iterates through a table of structures to perform repetitive memory transfers and area fills.
- `perform_blitter_area_fills`: Uses the blitter to perform multiple area fills and data transfers across a table of structures.
- `update_high_score_display`: Sets a global variable to zero and then calls three routines to initialize a structure, convert a numeric value to an ASCII string, and initialize an object from that string.
- `generate_pattern_table`: Populates a table with a repeating pattern of values derived from a loop and an incrementing counter.
- `fill_complex_table`: Fills a memory area with a structured sequence of words, alternating between fixed constants and values from another source.
- `process_countdown`: Updates a global variable with character values and iterates through a loop that calls other routines and modifies a global word until a condition is met.
- `call_subroutine`: Calls another routine and returns zero.
- `loop_with_wait`: Executes a loop that decrements a value and calls another routine that waits for a specific bit to be set.
- `initialize_callback_structure`: Stores a function pointer, a data pointer, and several integer values into a contiguous block of memory.
- `initialize_callback_structure_2`: Stores a function address, a pointer to a global variable, and three integer values into a data structure.
- `initialize_callback_structure_3`: Populates a data structure with a function pointer, a pointer to a global variable, and three integer values.
- `initialize_callback_structure_4`: Sets a function pointer and several associated data values into a global structure.
- `initialize_callback_loop`: Sets up a callback structure with a function pointer and a data pointer, then enters an infinite loop.
- `execute_sequential_subroutines`: Calls four different subroutines in succession.
- `rotate_coordinate_tables`: Shifts several values between a set of global variables and updates three other memory locations with offset values.
- `initialize_graphics_system`: Allocates memory, configures hardware display registers including bitplanes and DMA, and populates several data tables with calculated addresses and patterns.
- `set_memory_boundaries`: Stores three calculated memory addresses into global variables.
- `write_constant_values`: stores four constant words into a table.
- `initialize_data_table_pointers`: Sets a series of global variables to specific constant values and populates several memory locations with pointer-like values and associated metadata.
- `configure_hardware_display`: Disables interrupts, sets several hardware registers for bitplanes and display timings, and populates a data block with addresses.
- `store_address_segments`: Splits three global pointers into high and low word segments and stores them in a table, then sets several more global variables to specific memory addresses.
- `configure_bitplane_registers`: Sets several hardware registers related to bitplanes and display timings, and stores segmented parts of three global pointers into a table.
- `initialize_graphics_library`: Opens a library, performs checks on a returned pointer, and populates a large data block with calculated addresses and values while enabling DMA.
- `set_and_wait_bit`: stores a byte into a memory location and waits until a specific bit in another memory location is set.
- `execute_loop_three_times`: Executes another routine three times within a loop.
- `update_status_logic`: Evaluates a global word through bitwise operations and conditional checks to update several global variables and call other routines.
- `dispatch_status_logic`: Evaluates a global word through bitwise operations to select and call various routines from a table while updating several status flags and counters.
- `manage_status_dispatch`: Calculates a value using bitwise operations to select a routine, updates a global status variable, and manages a countdown timer.
- `dispatch_conditional_logic`: Performs conditional checks on a value to set a routine pointer, modifies a word in memory, and decrements a global counter.
- `manage_timer_and_flags`: Decrements a global counter and modifies a status flag based on the state of another global variable.
- `check_joystick_status`: Checks a bit in a global variable related to hardware status and resets several global variables if a specific condition is met.
- `set_constant_one`: stores the value one into a global variable.
- `perform_bitwise_or`: Performs a bitwise OR operation on register d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero, effectively ensuring the register contains zero.
- `clear_data_register_d0`: Performs a logical OR operation on register d0 with zero to clear its value.
- `render_tile_grid_blit`: Walks a 32-by-32 grid of bytes, using each byte multiplied by 96 as an offset into a graphics table, and for every cell issues three blits through the custom chip blitter registers (waiting on its busy flag before each), writing source and destination pointers and a size word.
- `populate_data_structures`: iterates through a table to write calculated addresses and values into a large data block.
- `populate_data_tables`: Iterates through nested loops to write calculated addresses and values into a structured data block.
- `update_bitmapped_data`: Calculates an offset and performs bitwise operations to set specific bits in a memory buffer based on two input values.
- `decrement_and_branch_counter`: Decrements a global counter and performs conditional branching or routine calls based on the counter's value.
- `manage_handler_state`: Checks a global variable and a counter to conditionally modify a function pointer and decrement the counter.
- `process_callback_chain`: Checks a global counter and either modifies a function pointer or decrements the counter while calling another routine.
- `modify_function_pointer_3`: Performs bitwise manipulation on a function pointer and conditionally executes another routine or modifies a global variable.
- `modify_function_pointer_2`: Performs bitwise manipulation on a function pointer and conditionally calls another routine or modifies a global variable.
- `call_reset_routines`: Calls an empty routine and then a routine that writes predefined constant values to global variables.
- `modify_function_pointer`: Performs bitwise manipulation on a function pointer and conditionally calls another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_pointer_and_decrement`: Updates a global pointer based on a condition and decrements a global variable.
- `reset_data_tables`: If a specific condition is met, it populates several global memory locations with a set of predefined constant values or resets them to all ones.
- `reset_constant_tables`: Writes a set of predefined constant values to several global variables or fills them with all ones if a condition is met.
- `fill_memory_with_ones`: Writes a value of all ones to several offsets within a structure pointed to by a register and to several global variables.
- `initialize_constant_table`: Stores a specific sequence of constant values into a series of global variables.
- `return_concatenated_registers`: Returns a 64-bit value formed by concatenating the contents of two registers.
- `update_object_attributes`: Calculates a memory address using a pointer and an index, writes several bytes of data to that location, and updates a global pointer.
- `update_object_properties`: Calculates a memory address based on a pointer and an index, writes several bytes of data to that location, and updates several global pointers and a flag.
- `clamp_coordinate_values`: Updates two global variables by adding values to them and clamping the results within specific minimum and maximum ranges.
- `clamp_and_update_globals`: Clamps a word value to a maximum of three hundred and two and stores it in a global variable.
- `clamp_and_store_values`: Clamps a word value to a maximum of two hundred and forty and stores it in a global variable, then stores the original word value in another global variable.
- `update_position_counters`: Adjusts multiple global variables and memory locations by adding or subtracting values based on comparison with specific thresholds.
- `update_coordinate_state`: Updates several global variables and memory locations based on comparisons between register values and specific thresholds.
- `update_coordinate_state_extended`: Modifies multiple global variables and memory locations using conditional logic based on input register values.
- `update_state_counters`: Updates a global variable by adding a register value and modifies several other global variables based on comparisons between a register and a threshold, as well as the signs of other register values.
- `update_global_state_variables_2`: Subtracts a value from a global variable and updates two other global variables with values from registers.
- `update_conditional_state`: Updates several global variables based on conditions involving register values and existing global variable states.
- `update_global_state_variables`: Adds a value to a global variable and updates two other global variables with values from registers.
- `calculate_address_offsets`: Computes a base address using a pointer and two global variables, then stores that address and several offset versions of it into global variables.
- `dispatch_nested_calls`: Uses a series of nested conditional checks on a value to select and execute various routines from a table.
- `dispatch_indirect_call`: Performs conditional checks on a register value to decide which routine to call from a table.
- `execute_nested_table_calls`: Uses nested conditional checks on two registers to select and execute routines from a table.
- `execute_conditional_calls`: Performs a series of conditional checks on a word value to decide which routine to call from a table.
- `execute_indirect_call_3`: Compares a word to five and calls another routine if the condition is met.
- `execute_indirect_call`: Loads a function pointer from a table using an index and executes it.
- `set_flag_and_value`: Sets a bit in a byte and stores the value one hundred in a global variable if a specific global variable is zero.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `set_routine_flag`: Checks a bit in a value and, if not set, sets that bit and calls the resulting address.
- `set_global_flag`: Stores the value one in a global variable.
- `reset_status_variables`: Resets several global variables to zero and performs arithmetic comparisons against two thresholds to set specific values in a global variable and a table entry.
- `update_state_flags`: Resets several global variables and compares arithmetic sums against two different thresholds to set specific bitmask values and status flags.
- `adjust_global_counters`: Adds or subtracts a value from a global variable based on a comparison and updates a global word with a bitwise flag.
- `update_pointer_and_dispatch`: Modifies a function pointer based on a comparison between two values and updates a global word with a bitmask.
- `modify_register_values`: Performs arithmetic on a value and sets a bit in a register.
- `set_attribute_flag`: Sets a specific bit in a memory location and then calls another routine.
- `update_flag_and_call_function`: Sets a bit in a memory location pointed to by a register and then executes a function retrieved from a table using an index derived from a register.
- `update_flag_and_dispatch_function`: Sets specific bits in a memory location pointed to by a register and then executes a function retrieved from a table using a calculated index.
- `tiny_ori_2`: Small routine of 1 instruction that merges 160 into d0.
- `update_dispatch_table_entry`: Performs a bitwise OR operation on a word at a location relative to a pointer and then executes a function retrieved from a table using an index derived from a register.
- `dispatch_via_table`: Modifies a value in memory using a bitwise OR and executes a routine from a table based on an index in a register.
- `execute_table_function`: Calculates an offset into a table of function pointers using a value in a register and executes the resulting subroutine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d4, then merges 0 into d0.
- `execute_indexed_routine`: Uses a register to calculate an offset into a table and calls the resulting routine address.
- `modify_flags_and_register`: Sets the first bit of a global variable, sets the fourth bit of a byte, and stores the value seventy-eight in a register.
- `set_data_values`: Stores two specific values into memory locations and performs a bitwise OR on a global variable.
- `assign_different_values`: Stores two different values into memory locations and performs a bitwise OR on a global variable.
- `increment_global_and_variable`: Performs a bitwise OR on a global variable and increments another global variable by ten.
- `increment_and_dispatch`: Sets the first bit of a global variable and increments a global variable until it reaches nine, calling another routine during the process.
- `reset_global_values`: Performs a bitwise OR on a global variable and then overwrites that same global variable with a constant value.
- `increment_conditional_variable`: Performs a bitwise OR on a global variable and increments another global variable only if its value is not equal to three.
- `update_counters_and_flags`: Sets the high bit of a global variable, increments one global variable by ten, and decrements another by one before calling another routine.
- `call_dynamic_routine`: Calls a routine located at an offset within a table, where the offset is determined by a value in a memory location.
- `populate_data_structure`: If certain conditions are met, it extracts data from a table using an offset, performs additions, and stores the results into a destination pointed to by a pointer.
- `populate_multiple_structures`: Extracts and processes data from multiple offsets in a table to fill three consecutive destination structures with calculated values and pointers.
- `process_single_structure`: Extracts data from a table using an offset, performs additions, and populates a single destination structure while decrementing a global counter.
- `update_table_entries`: Iterates through a table of pointers and updates a structure with values from the table and associated data.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `calculate_and_store_data`: Processes a list of pointers to perform arithmetic calculations and populates a structure with the resulting values and bit-shifted data.
- `set_word_to_all_ones`: stores a value consisting of all bits set to one into a memory location pointed to by a register.
- `process_object_list_2`: Iterates through a list of pointers to update structure values, performs arithmetic on coordinates, and increments a global counter.
- `update_object_list`: Iterates through a list of pointers to update coordinates, performs boundary checks, and manages a secondary list of active objects.
- `update_object_position_and_list`: Updates an object's coordinates by adding offset values to its current position and checks if the new position remains within specific screen boundaries.
- `process_object_list`: Iterates through a list of pointers to update coordinates and manage a table of pointers based on bitwise conditions.
- `process_coordinate_list_2`: Iterates through a list of pointers to update coordinates and manage a table of pointers based on specific bitwise conditions.
- `process_coordinate_list`: Iterates through a list of pointers, performing bounds checking and coordinate updates based on values in the table.
- `fill_table_with_pointers`: fills a table with a sequence of pointers, incrementing each pointer by fourteen bytes per entry.
- `initialize_callback_sequence`: Manages a sequence of operations by initializing a structure and iterating through a string to trigger specific data updates.
- `fill_table_with_word`: fills a table with a specific word value using a loop.
- `compare_and_decrement_table`: compares bit patterns between two sets of tables and decrements values in the first set if the patterns do not match.
- `clear_memory_block`: fills a large block of memory starting at a specific address with zeros.
- `copy_structured_data_loop`: Performs a series of division and modulo operations to calculate counts and copies data from a source table to a destination buffer in a loop.
- `convert_number_to_string`: performs repeated division and modulo operations to convert a number into a sequence of ASCII characters stored in a global buffer.
- `initialize_object_from_string`: iterates through a string to calculate offsets and initialize multiple fields within a structure pointed to by a register.
- `initialize_object_list`: Iterates through a string to populate a table of structures with calculated offsets, coordinates, and pointers to data.
- `initialize_callback_structure_5`: Iterates through a list of bytes to find a non-zero value and populates a structure with constants and calculated offsets.
- `initialize_structure_from_list`: Iterates through a list of bytes to find a non-zero value, then populates a structure with various constants, calculated offsets, and pointers.
- `initialize_structure_fields_2`: Calculates an offset into a table and populates a structure with various values, including a pointer and several constants.
- `tiny_ori_3`: Small routine of 1 instruction that merges 100 into d0.
- `copy_data_block`: Copies eight 32-bit words from a source table to a destination starting at a global variable.
- `copy_non_contiguous_data`: Copies eight 32-bit words from a source table to a destination buffer.
- `update_entity_attributes`: Calculates an offset into a data table to load a block of attributes and coordinates into global variables.
- `populate_stride_table`: Iterates through a source list to copy data into a new table, setting specific fields to zero and appending a terminator.
- `generate_parameter_structure`: Calculates a value based on a global address and populates a structure with bit-shifted results and pointers.
- `setup_execution_context`: Writes several values and pointers into a memory block and conditionally calls another routine based on bitwise checks.
- `populate_parameter_block`: Fills a structure with a bitwise ORed value, a previously calculated value, and several zeroed fields.
- `performs_bitwise_or_on_d0`: Performs a bitwise OR operation on register d0 using a zero constant.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `reset_blitter_and_copper_state`: Waits for the blitter to become idle, then programs the custom chip registers addressed through A6 (blitter/copper-style control words and a 32-bit value written at offset 34 and 32) and clears the first word.
- `load_table_parameters`: Iterates through a table of pointers to extract and store specific word values into a structure.
- `process_linked_list_flags`: Traverses a linked list and updates bit flags within each node's structure based on specific conditions and a global variable state.
- `flag_linked_list_nodes`: Traverses a linked list and sets a specific bit in a byte within each node if certain conditions are met.
- `transfer_structure_fields`: Copies several fields from one structure to another, including a calculated value and a pointer.
- `update_entity_movement_and_direction`: Iterates through a list of entity pointers to update their horizontal and vertical coordinates based on velocity and direction flags.
- `load_interpolated_data_table`: Uses a calculated index to copy multiple fields from a data table into a destination structure and sets specific short values based on a comparison.
- `calculate_table_offset_2`: Initializes a structure with several values and calculates a table index based on the difference between two sets of coordinates.
- `update_interpolated_structure`: Copies data from a table into a structure and sets various bit flags based on comparisons between multiple values.
- `update_entity_state_and_timer`: Calculates new spatial or state values for an object based on a table lookup and updates its internal counters and flags.
- `transfer_global_word`: Copies a word from one global variable to another.
- `set_global_pointer`: Stores a constant value of all ones into the memory location pointed to by a global variable.
- `copy_list_elements`: Iterates through a linked list and copies various data fields from each element into a destination structure.
- `process_entity_list`: Iterates through a list of entities, checks specific bit flags and coordinate bounds, and if valid, calculates new positions and attributes to store in a structure.
- `calculate_linear_interpolation`: Performs a calculation involving a global variable and two input values to return a scaled result.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a7).
- `poll_global_variable_state`: Checks a global variable for specific bit patterns and enters loops that wait for a bit to change state.
- `transform_table_data`: Iterates through a table and replaces values based on a character lookup and mathematical transformations.
- `transform_lookup_table`: Iterates through a source table to find matching bytes and calculates new values based on a mathematical formula, storing the results in a destination table.
- `initialize_state_structure`: Copies several values from a source structure into a series of global variables.
- `decrement_nibbles`: Iterates through a table and decrements the lower nibble of each word if it is non-zero.
- `rotate_global_values`: Shifts three sets of 32-bit values through a series of global variables.
- `refresh_dynamic_pointers`: Permutes three source words into six global variables and updates three 32-bit pointers using the resulting values and table offsets.
- `update_bounds_and_counters`: Performs complex bounds checking and arithmetic on several global variables, including conditional negation and counter resets.
- `permute_global_words`: Permutes the assignment of three source words to six destination global variables based on a cycling counter.
- `update_calculated_pointers`: Calculates three 32-bit pointers using combinations of global words and table offsets, then stores them back into global word pairs.
- `increment_and_check_limit`: Increments a global variable by a fixed amount and sets a global variable to a constant value if the incremented variable exceeds a specific threshold.
- `clear_global_array`: Sets seven consecutive global word values to zero.
- `copy_table_data`: Loads a pointer from a calculated offset in a table and copies seven words from that location into global variables.
- `initialize_object_list_2`: Iterates through a list of pointers and populates a structure with specific constant values and data from the list.
- `transform_object_coordinates`: Performs a 2D rotation and scaling transformation on a list of objects using fixed-point arithmetic.
- `calculate_scaled_offset`: Performs arithmetic on an input value to calculate a base offset and a scaled remainder, returning them as a combined result.
- `loop_and_store_values`: Repeatedly calls another routine and stores the returned word into a memory location until a loop counter reaches zero.
- `decrement_loop`: Decrements a local variable in a loop until it reaches a specific value.
- `copy_block_on_counter`: Decrements a global counter and, if it reaches zero, resets it to a fixed value and copies a block of eleven words from one memory location to another.
- `update_sprite_attribute_table`: Iterates through a list of pointers to update sprite or object attribute data.
- `swap_and_update_pointers`: Swaps two values in global variables and updates several other global variables with the new value and its offset variations.
- `update_counters_and_timers`: Updates several global counters and timers based on the absolute value of a global variable and handles decrementing logic.
- `reset_status_flag`: Calls another routine and then clears a global word to zero.
- `copy_data_structures`: Copies data from three source pointers into a destination structure and sets specific status flags and default values.
- `initialize_structure_fields`: Writes a pointer and several short values into a structure and sets a status flag.
- `set_global_value`: stores a constant value into a global variable.
- `decrement_global_value`: Subtracts a constant from a global variable and stores the result back into itself.
- `reset_structure_fields`: Sets a status flag and a pointer and a short value within a structure.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d1.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_dma_and_interrupts`: Manipulates the DMA controller and sets up interrupt handlers by writing function pointers and specific values to memory locations.
- `xor_global_variable`: Performs two consecutive XOR operations on a global variable using two different constants.
- `execute_indirect_call_2`: Calls a function pointer located at an offset from a base address provided in a register.
- `halt_in_infinite_loop`: Enters an infinite loop that performs no operations.
- `assign_global_byte`: Stores a constant byte value into a global variable.
- `set_interrupt_handler`: Writes a pointer to another routine into a global variable, then writes a masked byte, another byte, and a word value to global variables.
- `update_status_bit`: Checks a bit in a hardware register and a global variable, then conditionally clears a bit in a global variable and writes a value to a table.
- `modify_memory_byte`: Checks a byte against global values and, if conditions are met, clears a specific bit in a memory location and writes a value to a table.
- `return_register_byte`: Returns the byte value currently held in a data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields_4`: Writes several values to a structure pointed to by a6, including a pointer, a 32-bit constant, and various 16-bit values at offsets 64, 66, 80, 84, 88, and 100.
- `wait_and_call_routine`: Polls a bit in a byte at offset 2 of a structure, and if a second bit is clear and a bit in the routine's own address is clear, it modifies its own address and calls another routine.
- `initialize_structure_fields_3`: Sets various fields in a structure pointed to by a6, including clearing offsets 114 and 116, storing a pointer at offset 80, storing d0 at offsets 76 and 84, storing 65534 at offset 164, storing 332 at offset 88, storing 0 at offset 66, storing 65536 at offset 68, storing 332 at offset 86, and storing 140 at offsets 98 and 102.
- `clear_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `update_structure_from_table`: Iterates through a table of pointers, waiting for a specific bit in a status byte to clear before copying values from the pointed-to structures into a structure pointed to by a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_and_branch`: Branches to an offset, copies a longword from the address in a0 to offset 84 of a structure in a6, copies a word from d0 to offset 88 of the structure, and branches to a fixed address.
- `store_address_to_global_2`: Stores the address in a0 into a global variable.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_status_and_constants`: Stores a word from d0 into offset 150 of a structure in a6, calls another routine, stores the result of that routine bitwise ORed with 32768 into offset 150, calls another routine, and stores 67690 at offsets 160, 192, and 208, and stores 1 at offsets 164, 196, and 212.
- `store_returned_word`: Calls another routine and stores the resulting word from register d0 into a structure at offset 150 of a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `return_with_unconditional_branch`: Returns immediately, otherwise it performs an unconditional branch to a fixed offset.
