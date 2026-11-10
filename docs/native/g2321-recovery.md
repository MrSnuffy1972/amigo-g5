# Game 2321 (SmashTV v1.0 2293): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02321). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1970760 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 126 raw-word lines |
| Functions named and described | pass, 316 of 316 named (0 author symbols), 316 of 316 described (5 by rule, 300 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4286 of 4286 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 406016 bytes, 14 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5427 of 5492 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 456930 |
| code | 17634 |
| data | 1496112 |
| padding | 84 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 188 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4353 | 252 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 464 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 5 |
| GAME (raw code file, load address unknown) | 1 | 0 | 0 |
| MUSC (raw code file, load address unknown) | 1 | 481 | 14 |

## Cases handled

- SmashTV_v1.0_2293/SmashTV_v1.0_2293.hdf: hard-disk image (AmigaDOS OFS, 2080 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree SmashTV_v1.0_2293_hdf, which is recovered file by file; no block outside the filesystem holds data
- runtime segment an address: 3757 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 362 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SmashTV_v1.0_2293/SmashTV_v1.0_2293_hdf/SmashTV/data/GAME: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- SmashTV_v1.0_2293/SmashTV_v1.0_2293_hdf/SmashTV/data/MUSC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 406016 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `invoke_and_reset_vectors`: It saves a pointer, installs a routine into a vector table, and invokes two stored handlers.
- `write_to_hardware_or_memory`: If a specific hardware register address is provided, it stores a word into a global variable; otherwise, it stores a longword into a memory location offset from the provided address.
- `xor_and_reset_global`: It XORs a global variable with several constants, then XORs it again with the same value twice and finally overwrites it with 255.
- `set_global_max`: Sets a global variable to a fixed maximum value.
- `toggle_bit_and_dispatch`: It calls a handler through a pointer stored in a global, toggles a bit in a byte pointed to by an argument, derives a masked value from two global bytes and stores pointers into several global fields depending on which bits are set.
- `wait_for_flag_cycles`: Invokes two handlers, then waits for a flag bit to be set and cleared by repeatedly calling a polling routine.
- `wait_for_flag_bits`: It spins until a byte at a fixed offset from a pointer becomes set, then spins again until that same byte becomes clear.
- `process_input_byte`: performs bitwise transformations on a global variable and uses the result to either fetch data from a table or reset a global variable.
- `invoke_handlers_conditional`: Calls a handler and, when a condition holds, invokes two more handlers before returning two register values.
- `call_handlers_when_zero`: When a global variable is zero, calls three handlers in sequence before returning two register values.
- `xor_byte_sequence`: It XORs a run of bytes through a pointer with a decreasing value, stopping when that value reaches zero, and returns the starting value.
- `call_structure_handler_slot`: It invokes the handler whose pointer is stored at a fixed offset in a structure held in a global, then returns.
- `invoke_callback_and_return_concat`: It calls another handler through a pointer stored in a global variable and then returns the concatenation of two input values.
- `call_offset_pointer_short`: It calls a function located at a specific offset from a pointer stored in a global variable and returns the values found in the data registers after the call.
- `set_global_and_jump_table`: It stores a constant into a global variable, then performs an indirect jump through an unrecovered jump table.
- `return_byte_value`: A trivial routine that returns a single byte value taken from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_input_byte`: Returns a byte after performing a bitwise OR operation with a constant.
- `manage_timer_and_array`: Manages a countdown timer and populates an array with incrementing values based on calculated parameters.
- `update_sprite_animation_state`: Checks if a sprite's position falls within a specific bounding box and, if valid, retrieves an animation frame index from a table.
- `copy_table_data`: iterates through a table to copy or fetch values based on the contents of a global variable.
- `populate_incrementing_table`: Writes a sequence of incrementing values and specific calculated constants into a table using a loop.
- `initialize_structure_fields_2`: Sets specific byte and word values at fixed offsets within a memory block.
- `calculate_table_values`: Performs complex arithmetic and conditional checks to populate a table with incrementing values.
- `update_structure_values`: calculates values using random number generation and writes a sequence of incrementing values into a global table.
- `call_function_from_table`: Checks if a value in a table is zero and if a range of values falls within specific bounds, then executes a routine from a table using an offset.
- `execute_table_callback`: manages timing and bounds checking before executing a routine retrieved from a table via an offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `nop_instruction`: This routine is empty and performs no operations.
- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller without performing any work, so it does nothing when executed.
- `check_global_variable_status`: Compares a global variable against zero twice and returns.
- `return_immediately_5`: The routine is empty and immediately returns to its caller.
- `return_immediately_6`: This routine performs no operations and returns to the caller.
- `return_immediately_stub_2`: The routine consists of a single instruction that returns control to the caller without touching registers, memory, or hardware.
- `return_from_subroutine_2`: The routine performs no operations and immediately returns to its caller.
- `compute_complex_value`: Calculates a 64-bit value based on several global variables and conditional logic involving division and modulo operations.
- `copy_data_block_2`: Checks if a value is within a specific range and, if so, performs a block memory copy from a source table to a destination address.
- `manage_timer_and_buffer`: Calculates differences between global variables to manage a countdown and conditionally updates several global values and memory addresses.
- `calculate_complex_value`: Performs arithmetic operations on global variables and returns a combined 64-bit value based on conditional range checks.
- `execute_sequence_of_subroutines`: Calls a series of subroutines and performs loops based on the values of a global variable and a local counter.
- `add_bcd_value`: Adds a 32-bit value to a 4-byte global variable using byte-wise addition and binary-coded decimal adjustments.
- `add_to_bcd_variable`: Adds a 32-bit value to a 4-byte global variable using byte-wise addition and BCD adjustment for each byte.
- `add_to_bcd_variable_2`: Performs byte-wise addition of a 32-bit input to a 4-byte global variable, applying BCD adjustment to each byte increment.
- `initialize_enemy_sprite_list`: Retrieves enemy data from a table and iterates through a list to initialize sprite properties.
- `clear_memory_block`: Sets several bytes and words at specific offsets within a memory block pointed to by a register to zero.
- `loop_and_call`: Iterates a number of times based on a global variable, calling another routine in each iteration and returning a value.
- `recurse_twice_until_counter_expires`: Decrements a 16-bit counter held in D7 and, while it has not reached minus one, invokes itself twice per pass before returning.
- `process_conditional_data`: Checks a range of values and, if valid, modifies a structure and masks the high bits of several bytes in a loop.
- `process_event_callback_list`: Iterates through a list of pointers to execute specific functions based on global state and conditional flags.
- `execute_indirect_call_2`: The routine performs an indirect call by retrieving a function pointer from a global variable and jumping to that address.
- `execute_indirect_call`: Calls a wrapper routine, retrieves a pointer from a global variable, and performs an indirect call using a value returned from that pointer.
- `initialize_data_structure_4`: Writes specific constant values to three different offsets within a memory block and returns a constant value.
- `sync_and_calculate_ratio`: Synchronizes a memory range, performs a calculation involving a table and structure offsets, and updates a global variable with the result.
- `update_state_and_sync`: Synchronizes memory bytes in a loop and calculates a value based on a table and structure offsets.
- `update_data_structure`: The routine compares memory blocks, performs a loop to synchronize data, and calculates a value based on offsets within a structure.
- `calculate_entity_bounding_box`: Calculates the dimensions and spatial bounds of an entity by performing fixed-point arithmetic on retrieved values.
- `update_entity_position_and_bounds`: Calculates new coordinates and bounding box dimensions for an object based on velocity and current position.
- `search_and_update_list`: Searches a list for a non-negative value and, if found, updates the entry with several new values from registers and a global variable.
- `initialize_data_structure_6`: Clears several global variables and writes specific constant values to a series of memory locations.
- `initialize_data_structure_5`: Clears several fields in a memory block and writes specific constant values to a structure pointed to by a register.
- `execute_dynamic_call`: Calls a routine via a pointer offset by a global value and then updates a global variable from memory.
- `update_global_counter`: Checks if a value in a register is above a threshold and, if so, increments a global variable by two.
- `calculate_offset_value_2`: Performs conditional arithmetic on a register value to return a calculated offset, while potentially updating a global variable.
- `update_global_counter_conditional`: Checks if a register value is below a threshold and, if so, increments a global variable by two.
- `return_void`: The routine is empty and performs no operations before returning.
- `calculate_conditional_value`: Returns a value based on a bit test performed on a global variable.
- `execute_repeated_calls`: Increments a global variable and calls another routine four times with a derived value.
- `call_subroutine`: Calls another routine located at a fixed address.
- `update_entity_state_table`: Iterates through a structured table to update entity properties based on arithmetic calculations and conditional checks.
- `populate_table_with_values`: Performs complex arithmetic using multiple routines to calculate values and populates a table with incrementing data.
- `update_table_values`: Performs arithmetic calculations and populates a series of memory locations with incrementing values based on a loop.
- `load_enemy_sprite_data`: Retrieves sprite configuration from a table based on an index and updates several global variables related to sprite dimensions, bitmasking, and memory offsets.
- `copy_data_block`: Copies specific non-contiguous words from a source table to a destination table in a loop.
- `reverse_copy_block`: Copies specific non-contiguous words from a destination table to a source table in a loop.
- `copy_and_shift_data`: Copies a block of data to a calculated destination and then shifts elements within a table forward.
- `update_state_logic`: Performs complex arithmetic on several global variables and executes a function pointer from a table based on a calculated index.
- `combine_with_constant`: Combines a value from a pointer offset with a constant value.
- `bitwise_or_constant`: Performs a bitwise OR operation between a word and a constant.
- `fetch_offset_word`: loads a 16-bit word from a memory location pointed to by an offset within a register and returns it.
- `apply_bitmask_to_register`: Performs a bitwise OR operation on the contents of a data register with a constant value and returns the result.
- `update_sprite_attribute_masks`: Iterates through a list of entity data structures to perform bitwise masking and OR operations on their attribute fields.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `bulk_copy_tables`: Copies large blocks of word data from one memory location to another using a loop.
- `bitwise_or_byte`: Performs a bitwise OR operation between a byte and a constant.
- `copy_data_blocks`: Copies multiple blocks of data from one memory location to another using a loop, then performs a secondary block copy of structured data.
- `copy_structured_data`: Copies several structured data blocks from a source to a destination using a loop.
- `fill_memory_with_mask`: Fills a memory range starting at a specific address with a constant value and resets two global variables.
- `set_global_constants`: Sets a global variable to a specific constant value.
- `concatenate_global_values`: Performs several conditional calls and returns a 32-bit value composed of two 16-bit segments derived from a global variable and other registers.
- `clear_if_less_than`: Compares a value from a table with a provided integer and sets a byte to zero if the value is less than or equal to the integer.
- `process_bitfield_data`: Performs complex conditional checks on input data and updates several fields in a structure based on values retrieved from tables.
- `transform_bitfield_value`: Performs bitwise checks on an input and returns a 64-bit value composed of a specific constant and a modified version of the input.
- `update_timer_and_reset_state`: Decrements several global counters and checks if they have reached zero to trigger a state reset.
- `clear_byte_on_threshold`: Sets a byte to zero if a value from a table is less than the sum of two provided integers.
- `mask_byte_by_bounds`: Modifies a byte by applying bitwise masks based on comparisons between several calculated sums and values in a table.
- `reset_byte_on_bounds`: Sets a byte to zero if certain conditions regarding calculated sums and table values are met.
- `mask_byte_complex_bounds`: Applies bitwise masks to a byte based on a series of conditional checks involving differences between integers and table entries.
- `clear_byte_on_range`: Sets a byte to zero if a calculated difference or a specific value falls outside of ranges defined in a table.
- `mask_byte_with_table`: Applies bitwise masks to a byte by comparing calculated differences against values stored in a table and another memory location.
- `check_range_bounds`: Compares a calculated value against two thresholds stored in a table and clears a byte in a structure if bounds are exceeded.
- `mask_status_byte`: Performs conditional bitwise masking on a byte within a structure based on comparisons between a value and table entries.
- `update_coordinate_table`: Updates two values in a structure by performing lookups in a global table using derived indices.
- `execute_callback_logic`: Evaluates a global structure to determine whether to call a function pointer or perform coordinate-based calculations and updates.
- `calculate_offset_value`: Calls another routine with a modified value and returns that value with a specific bitmask applied based on a parameter.
- `set_global_constant`: Sets a global variable to a specific constant value twice.
- `write_structure_data`: Writes a short value, a constant, and a calculated byte into a structure at specific offsets.
- `clear_high_bits`: Iterates through a series of memory locations, masking each byte to keep only the highest bit set.
- `execute_indirect_calls`: Executes a sequence of routines by iterating through a table of offsets and performing conditional logic based on a global flag.
- `initialize_data_sequence`: Performs complex calculations involving multiple global variables and calls another routine to populate a sequence of values in a table.
- `update_timer_and_sequence`: Decrements a timer value stored in a global structure and manages a sequence of values in a table.
- `update_table_entry_2`: Searches through a table for an entry with a non-negative value and writes several parameters into that entry if specific conditions are met.
- `decrement_and_store_byte`: Decrements a value by two in a loop and stores the result into a structure offset until the value is less than two.
- `execute_empty_routine`: This routine performs no operations and returns immediately.
- `update_table_entry_3`: Iterates through a table of 6-byte entries until it finds a valid entry marked by a non-negative value.
- `decrement_and_store_sequence`: Iteratively stores a decreasing byte value into a structure at a fixed offset from a pointer.
- `update_table_entry_4`: Iterates through a table of entries until a negative terminator or a specific memory boundary is reached.
- `store_conditional_values`: Compares two values and, if the second is not greater than the first, stores a byte and a word into a structure.
- `update_ram_values`: Updates two specific offsets in a structure using values derived from a global variable and a constant.
- `update_structure_values_3`: Iterates through a table to find a valid entry and then stores several values into that entry's memory location.
- `execute_indirect_loop`: Iterates through a table of offsets and executes the routine located at the resulting addresses.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `store_result_conditionally`: Checks a global byte and either stores a constant value or stores a result from another routine into a pointer.
- `update_pointer_from_global`: Checks a global byte and a structure field to decide whether to set a pointer to a specific value or store the result of another routine into the structure.
- `update_pointer_from_global_2`: Checks a global byte to decide whether to set a pointer to a specific value or store the result of another routine into the structure.
- `modify_structure_fields_2`: Subtracts a global value from a structure field and performs arithmetic to update other fields or resets the structure to default values.
- `decrement_structure_byte`: Subtracts a global byte from a structure byte and sets the structure pointer to a specific value if the result is zero or negative.
- `decrement_and_check_byte`: Subtracts a value from a byte stored at an offset from a pointer and updates the byte with the result.
- `modify_structure_fields`: Updates a byte in a structure by subtracting a global value, then conditionally performs further updates or resets the structure fields to default values.
- `assign_structure_fields`: Writes a short value, a byte, and another byte into a structure at specific offsets.
- `execute_indirect_jump`: Performs an indirect call to an address calculated from a bitwise operation on two registers.
- `subtract_registers`: Subtracts the value in one register from another and returns the result.
- `subtract_registers_alt`: Performs a subtraction between two registers and returns the difference.
- `jump_to_subtract_two_values`: Jump stub: passes control straight on to subtract_two_values.
- `add_registers`: Adds the values of two registers and returns the sum.
- `add_registers_alt`: Calculates the sum of two registers and returns the result.
- `add_registers_alt2`: Returns the sum of two input values provided in registers.
- `return_immediately_7`: The routine is empty and returns immediately.
- `subtract_two_values`: Subtracts the value in one register from the value in another and returns the result.
- `clamp_and_return_values`: Calls another routine and clamps the resulting 64-bit value's components within specific ranges before returning them.
- `perform_indirect_call`: Executes an indirect call to an address stored in a register.
- `return_immediately_8`: This routine performs no operations and returns.
- `add_two_shorts`: Adds two short integer values together and returns the result.
- `add_two_shorts_2`: Calculates the sum of two short values and returns it.
- `add_two_shorts_3`: Returns the result of adding two short integers.
- `return_immediately_9`: This is an empty routine.
- `subtract_two_shorts`: Subtracts one short value from another and returns the result.
- `subtract_two_shorts_2`: Calculates the difference between two short integers and returns it.
- `subtract_different_values`: Calculates the difference between two registers and returns the signed result.
- `update_table_elements`: Iterates through a data structure to perform conditional arithmetic and updates multiple fields based on values in a table and a pointer.
- `update_object_priority_queues`: Clears a table of values and iterates through two sets of object lists to update priority counters.
- `loop_through_table`: Iterates through a table of pointers and calls each routine pointed to by the table entries.
- `process_conditional_short`: Retrieves a short value from a pointer and returns it either directly or modified by a bitwise OR operation based on a comparison of a value at an offset.
- `combine_pointer_values`: Concatenates a short value and an integer retrieved from a pointer with a large constant using bitwise operations.
- `calculate_concatenated_64bit_value`: Retrieves values from a table using a pointer and returns a concatenated 64-bit result.
- `calculate_complex_64bit_value`: Performs arithmetic on values from a table and returns a concatenated 64-bit result.
- `calculate_offset_64bit_value`: Calculates a 64-bit result by subtracting values retrieved from a table and concatenating them.
- `calculate_complex_value_2`: Performs arithmetic operations on values retrieved from a pointer and a table to return a concatenated 64-bit result.
- `execute_conditional_loop`: Checks a byte in a global structure and, if non-zero, executes a loop that calls another routine four times.
- `iterate_and_call`: Checks a byte in a structure and, if non-zero, executes a loop that calls another routine four times based on values in a list.
- `initialize_and_process_list`: Clears several memory areas, sets global values, and iterates through a list of data to call another routine for each entry.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `add_bcd_to_global_2`: Adds a 32-bit value to a global variable using BCD adjustment for each byte.
- `add_bcd_to_global`: Adds a 32-bit value to a global variable using byte-wise addition and BCD adjustment.
- `store_constant_word`: Stores the constant value zero-fffc into the memory location pointed to by a register.
- `execute_callback_list_and_update_state`: Iterates through a table of function pointers to execute a sequence of routines, then performs conditional updates to global state variables and manages a secondary list of tasks based on calculated offsets and bitwise operations.
- `update_byte_with_subtraction`: Subtracts a global byte from a value at a pointer and updates the pointer with the result or a specific constant.
- `update_structure_counters`: Performs conditional arithmetic on structure fields based on a global value, updating counters or setting a pointer to a specific value.
- `preserve_and_update_structure`: Saves two words from a structure, calls another routine, restores the words, and then updates structure bytes based on global values.
- `update_structure_members`: Modifies bytes within a structure using values from global variables and performs an indirect call if certain conditions are met.
- `update_complex_state`: Evaluates structure members to perform bitwise rotations and conditional updates to a table of values.
- `update_complex_state_2`: Evaluates a structure's members and global variables to perform bitwise rotations, conditional updates to structure offsets, and indirect calls.
- `initialize_global_state`: Sets several global variables to specific constant values and performs two routine calls if a global variable is zero.
- `scale_pointer_value`: Multiplies a value from a pointer by two and stores the result in a global variable.
- `set_constant_byte`: Stores the value zero-ff into a global variable.
- `mask_table_bytes`: Iterates through four memory locations in a table and masks each byte to retain only the highest bit.
- `update_global_byte`: Checks a global byte and sets it to negative one if it is currently zero.
- `update_global_word_via_pointer`: Combines a byte from a register and a byte from a memory pointer to update a global word.
- `scale_and_store_word`: Combines a byte from a register and a byte from a pointer, multiplies it by a global value, and stores the result in a global word.
- `process_scaled_data`: Calculates a value based on a byte, potentially executes indirect calls from a jump table, and updates a global variable and a table.
- `set_initial_values`: Stores a word from a pointer into a global variable and sets another global variable to a constant.
- `mask_global_bytes`: Iterates through four memory locations starting from a global variable and masks each byte to keep only the highest bit.
- `update_global_word`: Combines a byte from a register and a byte from a pointer to update a global word.
- `calculate_scaled_value`: Multiplies a value by a global variable and shifts the result right by four bits before storing it in another global variable.
- `modify_pointer_offset`: Calls another routine and updates a specific offset in a structure if it matches a certain value.
- `update_pointer_value`: Checks if the word at a pointer is equal to a specific constant and, if so, overwrites it with negative four.
- `search_pointer_list`: Iterates through a list of pointers until it finds a negative value or reaches a specific memory boundary.
- `traverse_pointer_list`: Iterates backward through a list of pointers until a negative value or a specific memory boundary is encountered.
- `update_entity_visibility_and_indices`: Clears a data block and iterates through an entity table to increment visibility counters and update a secondary index list.
- `process_callback_list`: Iterates through a list of short integers to execute routines via function pointers found at calculated offsets.
- `calculate_combined_coordinates`: Computes a 64-bit value by combining offset-based calculations and structure member values.
- `get_concatenated_values`: Returns a 64-bit value composed of two words retrieved from a structure, with conditional modification of the second word.
- `compute_offset_result`: Calculates a 64-bit value by combining structure members with a value retrieved from a table using a calculated index.
- `execute_offset_call`: Performs an indirect call to an address determined by adding a register value to the current routine's address.
- `nop_routine`: The routine is empty and performs no operations.
- `call_via_register`: Executes an indirect call using an address provided in a register.
- `return_from_subroutine_3`: This is an empty routine that performs no operations before returning to the caller.
- `return_immediately_10`: This routine performs no operations and returns to its caller immediately.
- `return_immediately_11`: The routine performs no operations and returns to its caller immediately.
- `return_immediately_12`: This routine performs no operations and returns to the caller immediately.
- `return_immediately_13`: This routine performs no operations and immediately returns to its caller.
- `return_void_2`: The routine is empty and performs no operations before returning to the caller.
- `return_immediately_14`: This routine performs no operations and returns control to the caller immediately.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or functional output before returning.
- `calculate_table_index`: calculates an index based on the absolute difference between two pairs of values and returns a value from a table.
- `return_without_effect`: The routine consists only of a return with no other operations, so it does nothing when called.
- `update_table_entry_5`: Performs complex arithmetic on values within a structure, including subtraction from a global variable, conditional assignments, and bounds checking.
- `store_word_at_offset`: stores a word from a register into a memory location at a specific offset from a pointer.
- `update_table_entry`: Updates a table entry by adding a value to it or storing a constant if a threshold is met.
- `update_table_values_2`: adds a value to a table entry and another entry if a threshold is not met, otherwise stores a constant value in the first entry.
- `set_pointer_to_stack`: stores a specific constant value into a memory location pointed to by a register.
- `update_structure_and_decrement_value`: Increments a byte at a specific offset within a structure and decrements another byte at a different offset.
- `preserve_and_update_data`: Saves two words from a structure, calls another routine, restores the original words, and then potentially stores a new value in a third offset.
- `process_complex_state`: Evaluates multiple conditions involving global variables and structure members to perform bitwise rotations, indirect calls, and conditional updates to structure fields.
- `dispatch_priority_objects`: Iterates through a table to perform conditional indirect calls based on values and bounds checks within a structure.
- `decrement_and_dispatch`: Decrements a byte in a structure, potentially performs a bitwise OR, and executes an indirect call based on a table offset.
- `decrement_array_element`: Decrements a byte at a specific offset within a global array and performs a conditional indirect call or executes one of two subroutines based on a value in a table.
- `clear_offset_byte`: Stores a zero byte at a specific offset from a pointer held in a register.
- `return_no_op_3`: The routine consists of a single instruction that returns immediately, performing no work.
- `decrement_indexed_byte`: Decrements a byte at an offset within a global array calculated from a signed multiplier and performs several memory updates and register assignments.
- `decrement_global_array_element`: Decrements a byte at an offset within a global array determined by a signed multiplier and performs several register and memory updates.
- `clear_byte_at_offset`: stores zero at a specific offset from a pointer held in a register.
- `execute_callback_table`: Checks a global variable and iterates through a table of pointers, calling another routine for each valid entry found in the table.
- `execute_conditional_callbacks`: Iterates through a table to execute routines based on values in the table, with logic branching based on a flag in a structure.
- `return_without_action`: The routine consists of a single return instruction and performs no work.
- `validate_and_update_structure`: Checks if coordinates are within bounds defined by a table, and if so, updates a structure with values from a global array and performs bitwise masking on a series of bytes.
- `validate_and_update_structure_2`: Verifies bounds against a table and, if valid, updates a structure with data from a global array and applies a bitwise mask to a memory range.
- `check_bounds_and_update`: Compares structure values against a table to determine if a specific condition is met, then performs memory updates and bitwise masking in a loop.
- `process_conditional_data_2`: Performs multiple comparisons against a table, and if conditions are met, updates structure members and performs a bitwise loop.
- `check_flag_and_update`: Checks a byte flag to decide between an indirect call or a bounds-checked structure update that involves memory masking and data assignment.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `validate_table_bounds`: Checks if values are within specific table bounds and returns a modified value or a constant.
- `process_bcd_addition`: performs complex arithmetic on a multi-word table and updates a memory location with a value from a global variable if certain conditions are met.
- `update_clamped_values_2`: performs arithmetic on multiple table entries and updates two consecutive memory locations with clamped results.
- `update_clamped_values`: adds values to two consecutive memory locations while ensuring the results do not fall below zero or exceed a specific limit.
- `check_collision_bounds`: Iterates through a table of data to compare a pointer's values against bounds and updates a byte in a structure if a condition is met.
- `write_data_and_call`: Stores several values into a memory structure via a pointer and then calls a routine at an offset determined by a register.
- `nop_padding_placeholder`: The routine is empty and contains no executable instructions.
- `initialize_structure_members`: Clears specific bytes in a structure, sets one byte to a maximum value, and then stores a value returned from another routine into two different locations within that structure.
- `nop_padding_sequence`: This routine is empty and contains no functional instructions.
- `perform_no_operation_2`: This routine is empty and performs no functional instructions.
- `search_pointer_list_wrapper`: This routine acts as a wrapper that calls a routine to iterate through a list of pointers until a negative value or a memory boundary is encountered.
- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `set_structure_values`: Stores a word and a byte into a memory structure at specific offsets.
- `nop_padding_sequence_2`: This routine is empty and consists of non-functional instructions.
- `perform_no_operation_3`: This routine is empty and performs no functional operations.
- `perform_no_operation_4`: This routine is empty and contains only non-functional padding instructions.
- `perform_no_operation_5`: This routine is empty and contains only non-functional instructions.
- `execute_null_padding`: This routine is empty and consists only of non-functional instructions that perform no operations before returning.
- `null_operation_routine`: This routine is empty and performs no actions before returning.
- `populate_data_structure`: Iterates through a loop to fetch values from another routine and store them into a multi-word structure at a pointer location, performing bitwise rotations and arithmetic on the data.
- `update_audio_parameter_table`: Extracts bitfields from a global variable to populate three separate memory locations and then clears the source variable.
- `update_state_pointers`: Performs bitwise operations on two memory locations and assigns various pointer values to a global table based on the resulting bits.
- `update_audio_parameters`: Performs bitwise operations on global variables to select and store specific memory addresses into a table, while interacting with hardware registers for interrupts and DMA.
- `update_indexed_structures`: A large routine that calls several subroutines to update indexed tables, modify structure counters, and adjust status flags based on various conditions.
- `update_indexed_table`: A forty-instruction routine that calls another routine to update an indexed table entry.
- `accumulate_and_copy_data`: Adds a byte to another byte within a structure and then copies a multi-byte block to a new destination.
- `decrement_structure_counters`: Decrements various byte values within a structure and performs conditional arithmetic or value assignments based on the results.
- `modify_status_flags`: Compares two bytes in a structure and, if they match and a condition is met, modifies several bytes within that structure.
- `decrement_and_adjust_structure`: Decrements a byte in a structure and adds or subtracts a word value from another structure member based on a bit flag.
- `update_structure_values_2`: Checks specific bit and byte conditions in a structure, decrements a byte, and then adds or subtracts a word value from another part of the structure.
- `update_indexed_structures_and_clear_bytes`: This routine calls a procedure to update indexed tables and modify structure counters and status flags.
- `dispatch_indexed_table_updates`: Serves as a shared intermediate that sits between two table-update routines: it is invoked by update_indexed_table and by update_indexed_structures_and_clear_bytes, and in turn calls update_indexed_structures and update_indexed_table, so it chains these updates together.
- `execute_routine_with_call`: This routine consists of ten instructions and calls another routine.
- `update_indexed_values`: Increments a byte, fetches a value from a table using that byte as an index, and updates structure members based on various bit flags.
- `update_indexed_table_entry`: Calculates an index based on a byte value and an offset to fetch a word from a global table into a structure.
- `clear_structure_bytes`: Checks specific byte pairs within a structure and sets them to zero if certain conditions are met, then calls another routine.
- `update_indexed_structures_2`: A twenty-instruction routine that calls other routines to update indexed tables and structures.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_and_decrement_counter`: Copies a multi-byte value into a global, derives a masked field from it, clears the source global, then invokes another routine several times while decrementing a global counter and reloading it when it reaches zero.
- `toggle_bit_and_dispatch_2`: It toggles a bit in a byte pointed to by an argument, derives a masked value from two global bytes and stores pointers into several global fields depending on which bits are set, then returns a modified input value.
- `mask_and_dispatch_globals`: Reads two adjacent global bytes, ANDs them, and stores the masked bits into a global field.
- `initialize_and_store_pointers_2`: Initializes a structure by zeroing a byte, copying a byte between two fields, storing a passed-in value into a four-byte field, and storing a pointer into it.
- `unreached_gap_code`: This two-instruction routine is never reached and decodes as code only by coincidence.
- `initialize_data_structure`: Initializes a data structure by zeroing one of its bytes, copying a byte between two of its fields, storing a passed-in value into a four-byte field, and storing a pointer into another field.
- `initialize_and_store_pointers`: Zeroes a byte in a structure, copies a byte between two fields, stores a passed-in value into a four-byte field, and stores a pointer into a field using a table indexed by a structure byte.
- `initialize_data_structure_2`: Initializes a data structure by clearing one byte, copying a byte between two of its fields, writing a passed-in value into a four-byte field, and storing a pointer into another field.
- `initialize_structure_fields`: It zeroes a byte in a structure, copies a byte from one structure field to another, and stores a passed-in value into a four-byte field before returning.
- `store_accumulated_values`: When a structure byte is non-zero, it adds that byte to an adjacent field and calls another routine.
- `decrement_counter_and_shift`: Decrements a counter stored at offset 43 of a structure and, when a related byte reaches zero, shifts a value down and subtracts another byte from a field at offset 9, clamping it to zero.
- `update_flag_and_adjust_byte`: Compares two bytes of a structure and, when they match alongside a check on a pointed-to byte, sets a flag, clears another field, and adjusts a control byte's bits.
- `adjust_counter_field`: Checks a flag bit in a structure and, if set, decrements a counter byte then adjusts a two-byte field by adding or subtracting a value depending on its sign.
- `decrement_and_adjust_field`: Decrements a counter byte and, when a second value is within range, adds it to a field, otherwise subtracts a masked portion of it.
- `store_halved_indexed_flag`: Reads a two-byte value, halves it, and writes a fixed byte into a table using that value as an index, then either sets or clears a flag bit and stores the adjusted value back.
- `store_16bit_value_into_struct`: Stores a 16-bit value taken from the calling context into a field at offset 16 of a structure pointed to by a preserved register.
- `store_byte_set_flag`: Stores a fixed byte, then either sets a flag bit or writes an incremented value into a structure field.
- `call_two_handlers`: A routine that calls two other handlers and returns, with no decompiled body available to describe further.
- `init_data_structure`: Initializes a data structure by clearing one of its bytes, copies a byte from one field to another, writes a passed-in value into a four-byte field, and stores a pointer into a field.
- `initialize_data_structure_3`: Initializes a four-byte structure by clearing one byte, transferring a byte between two of its fields, writing a passed-in value into the four-byte field, and saving a pointer into another field.
- `call_initialization_routine`: A two-instruction routine that performs a tail branch to another routine.
- `init_and_store_pointers`: Initializes a data structure by clearing one byte, copying a byte between two of its fields, storing a passed-in value into a four-byte field, and saving a pointer into another field.
- `decrement_counter_and_clear_globals`: Decrements a counter at offset 1 of a structure and, upon reaching zero, moves a value, shrinks a 16-bit field, zeroes several fields, and clears four global variables.
- `update_indexed_pointer`: When a flag bit is set, it reads a byte from a table indexed by an incremented structure field, clamps and masks the result, and stores a computed pointer into a structure field.
- `store_indexed_pointer`: It stores a pointer into a structure field using a table indexed by a structure byte and a passed-in value.
- `handle_structured_counter`: It copies a byte from a structure field when a leading byte is zero, then branches on that value.
- `resolve_index_and_fill_struct`: Reads a non-zero index from a pointer, looks up a second value in a global table, writes a computed address and index into fields of a structure, zeroes several bytes, and sets two flag bytes.
- `unavailable_routine_body`: A 29-instruction routine invoked by a single caller that executes during the reference run but whose operations cannot be described from the unavailable decompiler output.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_no_operation_2`: The routine consists of a single instruction that immediately returns to its caller, performing no work, no memory or hardware access, and no library calls.
- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `return_immediately_4`: This is an empty routine that performs no operations and returns to its caller.
- `return_no_op_2`: The routine consists of a single instruction that returns immediately, so it performs no operations and has no side effects.
- `return_no_operation`: The routine consists of two instructions and does nothing but return to its caller, with no register, memory, hardware, or library side effects visible in the decompiled body.

### MUSC (raw code file, load address unknown)

- `advance_music_playback`: Loads the current music pointer and a mask from a global music structure, clears the pointer, then repeatedly invokes another routine to advance the playback state.
- `update_channel_registers`: Reads the current copper-list position from the VBLANK register and, for each of the four sound channels, copies a pair of values into the channel's length and left/right balance registers when the corresponding bit is set.
- `advance_structure_entry`: Copies several fields from one structure to another, then when a counter byte reaches zero it loads the next entry from a table and copies pointer and flag values into the current structure.
- `store_pair_from_call`: Adds a structure byte to another field, calls another routine to produce a pair of values, then stores that pair into a two-byte field pointed to by an offset.
- `advance_table_pointer`: Advances a pointer through a byte table by decrementing a counter and loading the next byte when it reaches zero, otherwise decrements a separate counter and subtracts a table value from a running index.
- `compare_and_adjust_flags`: Compares two bytes of a structure and, when they match alongside a check on a pointed-to byte, sets a flag, clears another field, and adjusts a flag bit.
- `decrement_structure_counter`: When a flag bit is set and a counter byte is non-zero, it decrements that byte and adjusts a 16-bit value in the structure by a small amount.
- `decrement_pointer_counter`: When a flag bit is set and a structure byte is non-zero, it decrements that byte and adjusts a structure pointer by a small value.
- `decrement_and_clear_counters`: Decrements a counter byte and, when it reaches zero, copies a field, decrements a second counter, and clears several structure and global fields.
- `advance_and_index_table`: Increments a byte counter and, if it overflows past its table, wraps it back to zero.
- `update_structure_table_value`: It looks up a 16-bit value in a table using an index derived from a register and a byte read from a structure, then stores that value at a fixed offset within the structure.
- `init_and_fill_structure`: Initializes a structure pointed to by a register by clearing its fields and counters, then, when a size byte is one to three, populates it by repeatedly invoking another routine to fill index pairs and setting several associated fields and global counters.
- `lookup_and_store_indices`: Reads a table index, looks up a second table value, and stores both into fields of a structure while setting several flag bytes.
- `process_structure_table`: Walks through a table of byte values, copying several fields from one structure to another for each valid entry.
