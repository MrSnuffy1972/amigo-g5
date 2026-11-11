# Game 587 (Nebulus): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00587). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (924016 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 77 raw-word lines |
| Functions named and described | pass, 174 of 174 named (0 author symbols), 174 of 174 described (9 by rule, 164 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1666 of 1666 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 297374 bytes, 17 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4150 of 4154 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33260 |
| code | 17576 |
| data | 328121 |
| padding | 1315 |
| unused | 543744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nebulus.adf:load | 1 | 106 | 7 |
| Nebulus.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3809 | 149 |
| Nebulus.adf:nebulus.code (raw code file, load address unknown) | 1 | 229 | 17 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 138 64 KB pages each with >= 29491 distinct PCs executed, 2260992 of its 2280375 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 66 64 KB pages each with >= 29491 distinct PCs executed, 1359871 of its 1364450 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 27 64 KB pages each with >= 29491 distinct PCs executed, 870731 of its 877133 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 1666 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Nebulus/Nebulus.adf:nebulus.code: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 297374 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `handle_game_event` (runtime segment an address): Executes a series of subroutines based on a global variable and then sets that variable to a specific value. Kind: sabotage path / integrity check. Pass path: the 'if' branch (u2__sub_4E78() returns 0).
- `execute_repeated_subroutines` (runtime segment an address): Executes a sequence of subroutines multiple times within a loop. Kind: loader/protection loop. Pass path: the loop completes 4 iterations and returns to the caller.

## Functions


### Nebulus.adf:load

- `initialize_system_and_copy_header`: Allocates memory and opens standard Amiga libraries for DOS, graphics, and intuition.
- `copy_bytes_from_source_to_dest`: The routine copies bytes from a source address to a destination address, decrementing a counter in D0 until it reaches zero.
- `shift_palette_entries`: The routine shifts a table of word values left by one bit and then copies a large block of 16-bit palette entries from a global table into four destination tables obtained from another routine.
- `store_handler_pointers_and_dispatch`: The routine stores two incoming values into global variables and then calls three routines taken from a table pointed to by a global variable, returning the result of the second call.
- `store_globals_and_call_handlers`: The routine stores two incoming values into two global variables and a third global variable is set from a value read from a table.
- `invoke_handler_chain`: The routine stores two incoming values into two global variables and then calls three routines taken from a table pointed to by a global variable, returning the result of the second call.
- `load_file_and_close`: The routine stores two input values into global variables, then calls another routine through a table to obtain a pointer, calls another routine through the same table to get a return value, calls a third routine through the table, and returns the value obtained.

### Nebulus.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `permute_and_mask_data`: Permutes a block of data by using values within the buffer as indices to fetch new values from a source table, then applies a series of bitwise AND and OR operations to a destination buffer using a bitmask table.
- `merge_bitmask_data_blocks`: Iterates through a source buffer to perform bitwise AND and OR operations on specific indices within a destination buffer.
- `update_sprite_attribute_table`: This routine updates a block of sprite or object attribute data by performing a series of lookups in a table and then copying the resulting values into a destination memory buffer.
- `dispatch_indirect_jump`: Calculates an offset based on a global variable and a value from a pointer, then performs an indirect jump to a table entry.
- `clear_memory_block`: Iterates through a memory block, setting specific offsets to zero and one other offset to all ones.
- `calculate_complex_offset`: Traverses a table to find a specific marker and then performs calculations to determine a destination address for an indirect call.
- `branch_on_table_value`: Checks a value in a table using an index derived from a global variable and performs an indirect jump if the value is not three.
- `check_table_value`: Checks a value in a table using an index derived from two registers and performs an indirect jump if the value is not three.
- `recursively_execute_routine`: Saves a global variable to a register, calls itself multiple times, and then restores the original value to the global variable before returning.
- `validate_table_entry`: Verifies a specific value in a table using a calculated index and executes an indirect jump if the condition is met.
- `blend_bitmaps_extended`: Performs a complex bitwise combination of source data into a destination buffer using multiple bitwise AND and OR operations across various offsets.
- `blend_bitmaps_partial`: Combines bits from a source table into a destination buffer using bitwise AND and OR operations for a set number of iterations.
- `blend_bitmaps_variant_2`: Iteratively applies bitwise AND and OR operations using five source words to update multiple specific offsets in a destination buffer.
- `blend_bitmaps_variant`: Iteratively applies bitwise AND and OR operations from a source list to a destination buffer using a specific pattern of five source words per iteration.
- `blend_bitmaps_simple`: Performs a basic bitwise combination of source words into a destination buffer using AND and OR operations.
- `blend_bitmaps_offset_2`: Performs bitwise operations on a destination buffer starting at a specified offset using values from a source table.
- `blend_bitmaps_offset`: Combines bits from a source table into a destination buffer starting at a specific offset, using bitwise AND and OR operations.
- `blend_bitmaps_minimal_2`: Uses a pattern of five source words to perform bitwise AND and OR operations on a destination buffer during each iteration.
- `blend_bitmaps_minimal`: Iteratively applies bitwise AND and OR operations from a source list to a destination buffer using a pattern of five source words per iteration.
- `blend_bitmaps_split_2`: Updates a destination buffer by applying two distinct sets of bitwise operations using source words from a table.
- `blend_bitmaps_split`: Combines bits from a source table into a destination buffer using two distinct sets of bitwise AND and OR operations per iteration.
- `blend_bitmaps_short`: Performs bitwise AND and OR operations to combine source data into a destination buffer using a pattern of three source words per iteration.
- `blend_bitmaps_fast_2`: Performs a simplified sequence of bitwise AND and OR operations on a destination buffer using source data.
- `blend_bitmaps_fast`: Combines bits from a source table into a destination buffer using a simplified sequence of bitwise AND and OR operations.
- `apply_bitmask_to_table_2`: Iterates through a source list to apply bitwise AND and OR operations to specific offsets within a destination table.
- `apply_bitmask_to_table`: Iterates through a list of values and performs bitwise AND and OR operations on specific offsets within a destination table using a counter.
- `fill_loop_counter`: Executes a loop that runs one hundred and twenty-eight times.
- `set_function_pointer`: Calculates a destination address based on input values and conditional checks, then stores that address into a table offset.
- `initialize_global_sequence`: Sets a series of specific values into a global variable in a fixed sequence.
- `update_global_with_search`: Saves a global variable, adds a value from a register to it, and calls another routine to potentially restore the original value.
- `process_conditional_arithmetic`: Performs arithmetic on a global variable and executes various subroutines or resets the variable based on its resulting value and range.
- `nop_placeholder`: This routine is empty and performs no operations.
- `reset_and_test_global`: Resets several global variables and performs conditional arithmetic and bit shifting based on their values.
- `conditionally_modify_global`: Checks a carry flag and a global variable before performing a masked addition and calling another routine to potentially revert the change.
- `clear_global_variable`: Sets a global variable to zero and returns a value derived from a register with the two most significant bits set.
- `process_state_and_shuffle_table`: Updates global state variables based on the return value of another routine and performs a complex permutation of a data table using indexed memory access.
- `update_global_counter`: Reads a byte from a memory location determined by a global variable, increments it, and resets it to zero if it reaches seven.
- `initialize_constant_globals`: Sets several global variables to zero or specific constant values and assigns a maximum unsigned value to a memory location.
- `initialize_global_values`: Sets several global variables and a memory location to specific constant values.
- `validate_table_entry_2`: Iterates through a table to find a specific marker and then checks if a value at that location falls within a defined range and satisfies a boundary condition against a global variable.
- `map_input_to_output`: Evaluates a global variable against several constants and performs complex conditional logic, including table searches and memory writes, to return a specific value.
- `update_conditional_global`: Resets a global variable and calls another routine, then updates that variable based on whether its value is below a certain threshold or matches a specific constant.
- `set_conditional_global`: Sets a global variable to one if a passed register value matches a specific constant, otherwise sets it to one after resetting it to zero.
- `set_global_one`: Sets a global variable to one.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `search_data_table`: Performs a complex search through a data table to find a specific byte value and updates several global variables based on the result.
- `scan_tile_grid_for_collision`: Initializes a set of scroll/position state variables, then walks a 2D table of per-cell flags row by row, and for each flagged cell checks whether the reference coordinate (taken from a saved variable) falls inside that cell's bounds; on a hit it resets the same state variables and returns early.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `search_data_table_2`: Performs a nested search through a memory-mapped data table to locate a byte with a specific value.
- `search_data_table_3`: Searches through a multi-dimensional data table for a byte with a negative value.
- `search_memory_table`: Performs a nested search through a memory range to locate a byte with a negative value and updates several globals based on the result.
- `search_data_table_5`: Performs a nested search through a memory range to find a byte with a negative value and updates several global variables based on the result.
- `search_data_table_4`: Iterates through a memory range to find a byte with a negative value and updates several global variables based on the result.
- `tiny_move_lsr_rts`: Small routine of 3 instructions that copies d1 to d0, then shifts d0 right by 2, then returns.
- `extract_table_nibble`: Calculates an offset into a table, checks neighboring bytes for specific bit patterns, and returns a four-bit value from a global table.
- `shift_global_variable`: Returns a value derived by shifting a global variable right by three bits and then left by eight bits.
- `filter_byte_value`: Retrieves a byte from memory and returns a masked version of it if the byte is negative and meets specific bitwise criteria.
- `map_byte_to_range`: Takes a byte from memory and returns a specific mapped value based on whether the input matches one of three distinct constants.
- `clear_byte_on_condition`: Checks if a byte from memory is negative and matches specific bit patterns, and if so, clears that byte to zero and calls another routine.
- `extract_masked_bits`: Evaluates a byte from memory against multiple bitwise patterns and returns either a constant or a masked version of the original byte.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 32 into d0, then merges 52 into d0.
- `set_global_three`: stores the value three into a global variable.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `reset_global_variable`: Sets a global variable to a specific value and then clears it to zero.
- `reset_global_variable_2`: stores the value three into a global variable and then stores zero into the same variable.
- `clear_global_values`: stores the value three and then zero into a global variable.
- `reset_global_variable_3`: stores the value zero into a global variable.
- `reset_global_values`: Sets a global variable to a specific value and then subsequently clears it to zero.
- `set_global_hundred`: stores the value zero and then one hundred into a global variable.
- `initialize_and_clear_data_buffers`: Populates a series of memory blocks with data from a table at specific offsets and then performs a multi-pass clearing of those same memory regions.
- `toggle_bits_and_extract_data`: toggles bits in a memory location using two different bit shifts, then calculates a value from a table and stores its bit-shifted components into three different locations.
- `toggle_bits_and_extract_data_alt`: toggles bits in a memory location using two different bit shifts, then calculates a value from a table and stores its bit-shifted components into three different locations in a different order.
- `extract_bit_fields`: Retrieves a value from a table based on a global index and extracts three distinct bit-shifted components into registers and globals.
- `extract_bit_shifted_values`: Checks a global variable and clamps its value if it exceeds a threshold.
- `copy_table_data`: copies multiple word values from a source table to various offsets within a destination buffer based on an index.
- `copy_data_block`: Copies a series of words from one memory location to various non-contiguous offsets in another memory location.
- `update_global_structure`: Compares a global variable against several fields in a structure and updates those fields if they do not match.
- `scatter_table_bytes`: Calls another routine multiple times to copy individual bytes from a source table to non-contiguous locations in a destination.
- `scatter_bytes_from_table`: Calculates an offset based on a value in a register and copies individual bytes from a source table to non-contiguous locations in a destination buffer.
- `copy_words_loop`: Executes a loop four times, performing a word-based copy operation during each iteration.
- `copy_memory_block`: Compares two values and, based on the result, copies a series of words from a source location to various offsets within a destination structure.
- `decrement_nested_counters`: Decrements three different global counters in a nested fashion, resetting them to a specific value if they fall below zero.
- `decrement_nested_counters_2`: Decrements three global counters in a nested fashion, resetting each to a specific value if it falls below zero.
- `update_bcd_packed_data`: Increments four consecutive bytes in a global memory structure using binary-coded decimal adjustment.
- `update_bcd_data`: Increments four consecutive bytes in a global structure using binary-coded decimal adjustments and conditionally increments a global counter.
- `add_to_global_short`: Adds a 16-bit value from a register to a global short integer.
- `subtract_with_clamp`: Subtracts a 16-bit value from a global short integer and clamps the result to zero if it becomes negative.
- `populate_entity_attribute_structure`: Calculates an index from an input byte to fetch a series of bytes from a data table and distributes them into multiple dispersed offsets within a structure.
- `copy_table_data_to_structure`: Calculates an index from an input byte and, if within range, copies a series of bytes from a data table into several dispersed offsets within a structure.
- `process_string_data`: Iterates through a null-terminated string and uses its contents to copy data from a table into dispersed offsets within a structure.
- `update_scaled_value`: Calculates a new value for a global variable by adding a second input to a value retrieved from a table using a third input as an index, then multiplying the table value by a constant.
- `execute_repeated_routine`: Executes another routine seven times in a loop.
- `clear_table_entries`: Iterates through a table and sets each word entry to zero.
- `sort_and_populate_list`: Processes a list of entries by calculating a masked index and fetching a byte from a table, then sorts the resulting list in ascending order based on a specific field.
- `process_state_machine_logic`: Manages a complex state machine by evaluating a control byte in a data structure to execute specific arithmetic, pointer updates, or recursive calls.
- `check_collision_conditions`: Compares entries in a table against a provided pointer to check for specific value ranges and bitwise flags in a global table.
- `evaluate_data_pointer`: Calculates a memory address from a pointer and bitfields, then performs conditional checks on the byte at that address to return a concatenated 32-bit value.
- `reset_status_flags`: Sets a global word to all ones and another global word to zero.
- `process_state_sequence`: Iterates through a sequence of values, updating global counters and modifying structure members based on specific value ranges.
- `execute_state_transitions`: Checks the value of a global variable and performs different arithmetic operations or calls other routines based on which specific value it holds.
- `initialize_data_structures`: Clears several consecutive global words to zero, clears a list of entries, and assigns specific constant values to two global variables.
- `update_dynamic_parameter_table`: Iterates through a table of values to perform arithmetic subtractions and clamping operations.
- `execute_indirect_calls`: Performs four consecutive indirect function calls.
- `fill_data_blocks`: Iterates through a source list to fill a destination buffer with repeated word values, optionally performing a byte swap on each word.
- `execute_indirect_jump`: Performs an indirect call to a function pointer retrieved from a table using a value in a register as an index.
- `scatter_data_elements`: Copies elements from a source table into a destination buffer using a non-linear, interleaved indexing pattern.
- `execute_recursive_calls`: Saves a global variable to a register, performs multiple recursive calls to itself, and restores the original value to the global variable before returning.
- `update_hardware_registers`: Updates hardware registers and populates a table with data derived from a global variable and specific memory offsets.
- `clear_memory_structures`: Traverses a linked list of pointers to clear specific memory ranges with zero or maximum values based on a condition.
- `reset_memory_tables`: Iterates through a list of pointers to initialize various memory offsets with specific constant values.
- `dispatch_via_jumptable`: Calculates an offset based on a value and executes a routine from a jump table.
- `copy_interleaved_word_blocks`: Copies words from a source buffer to a destination buffer using an interleaved pattern of forward and backward offsets.
- `execute_recursive_calls_2`: The routine performs two consecutive calls to itself before returning.
- `check_collision_bounds`: Checks if a specific value falls within a range defined by a table of structures and updates a status flag.
- `update_bcd_counters`: Checks a structure for specific range constraints and, if met, increments two bytes using binary-coded decimal adjustments.
- `handle_game_event`: Executes a series of subroutines based on a global variable and then sets that variable to a specific value.
- `insert_sorted_value`: Searches a descending list for a position to insert a value, then shifts subsequent elements to maintain order.
- `perform_list_insertion`: Locates the correct position in a list for a new value and shifts existing entries to accommodate it.
- `process_game_over_state`: Updates a global variable and performs data manipulation on a list of values.
- `handle_time_limit`: Executes three subroutines, sets a global variable to a specific value, and calls two other routines.
- `insert_sorted_value_2`: Searches through a list of values to find an insertion point and shifts existing elements to maintain a sorted order.
- `normalize_numeric_values`: Calculates a value from multiple sources and uses multiple loops to reduce both a local variable and a global variable to a range between zero and ninety-nine.
- `display_bonus_message`: Executes a series of subroutines and sets a global variable to a specific value before returning.
- `format_numeric_string`: Converts a numeric value into a three-digit ASCII string by incrementing character bytes.
- `initialize_options_menu_ui`: Populates UI string structures by copying data from a table and manages the menu state machine.
- `populate_ui_strings`: Calls another routine twice to iterate through null-terminated strings and copy data from a table into specific offsets within a structure.
- `tiny_ori_2`: Small routine of 1 instruction that merges 108 into d0.
- `transfer_and_increment_bytes`: Moves bytes from several source pointers into stack-like destinations or registers, then compares a byte from the first source to a constant and stores the result into a destination pointed to by a register.
- `conditionally_execute_subroutines`: Checks a global variable and executes a sequence of subroutines based on whether the variable equals one.
- `increment_wrap_counter`: Increments a 16-bit global variable and resets it to zero once it reaches a specific value.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d4.
- `set_global_and_call`: Stores a constant value into a global variable and then calls two other routines.
- `call_subroutine`: This routine calls another subroutine and then returns.
- `call_string_processor`: Calls a routine that processes null-terminated strings to copy data from a table into a structure.
- `loop_string_data`: Executes a loop four times to perform a series of data processing operations.
- `execute_repeated_subroutines`: Executes a sequence of subroutines multiple times within a loop.
- `tiny_ori_ori_move`: Small routine of 3 instructions that merges 0 into d0, then merges 46 into d0, then copies d0 to d7.
- `parse_amigados_hunk_types`: Iterates through a data structure to identify and process specific AmigaDOS hunk type codes.
- `clear_and_bit_reverse`: Clears a block of memory with zeros and then performs a bit-reversal operation on a series of bytes stored in a table.
- `update_index_and_lookup`: Increments a global counter with wrap-around logic and uses the result to perform multiple lookups in a table via a global variable.
- `update_global_value_with_mask`: Checks a global flag to determine whether to perform a masked addition and bitwise shift on a global variable or reset it to zero.
- `perform_math_on_global`: Performs a series of arithmetic operations and conditional checks on a global variable before clearing it.
- `increment_conditional_value`: Increments a global variable if it is less than itself, which is logically impossible in this context.
- `store_global_values`: Stores a global variable and a constant value into two consecutive memory locations.
- `reset_state_flags`: Sets two consecutive word-sized global variables and two consecutive byte-sized global variables to specific constant values.
- `store_constant_values`: Stores a global variable and the constant value zero-ff into two consecutive word-sized memory locations.
- `toggle_state_and_load_data`: Toggles a global variable between two values and, if a specific condition is met, loads two consecutive words from a memory location into that global variable.
- `duplicate_global_value`: Copies a global variable into two consecutive word-sized memory locations pointed to by a calculated address.
- `reset_state_flags_2`: Sets two consecutive word-sized global variables and two consecutive byte-sized global variables to specific constant values.

### Nebulus.adf:nebulus.code (raw code file, load address unknown)

- `apply_relocation_offsets_2`: Iterates through a table of pointers and updates each entry by adding a relative offset stored at that location to the pointer's value.
- `apply_relocation_offsets`: Iterates through a table of pointers and updates them by adding relative offsets to their stored values.
- `process_string_and_wait`: Calculates the length of a null-terminated string from a pointer and waits for a specific key press via multiple routine calls and traps.
- `execute_nested_calls`: Performs a series of conditional calls to another routine based on the return status of previous calls.
- `execute_conditional_calls`: Calls another routine and then performs a subsequent call if the first call returns a false status.
- `set_global_values`: Stores two specific constant values into a global variable sequentially.
- `initialize_global_constants`: Writes two constant values to a global variable.
- `loop_on_bit_status`: Repeatedly checks the lowest bit of a global variable and performs assignments until a condition is met.
- `check_bit_and_call`: Checks the highest bit of a value, calls another routine, and returns based on the bit status and a secondary condition.
- `toggle_bit_loop`: Enters a loop that waits for the lowest bit of a global variable to change state, decrementing a local counter until it reaches zero.
- `trigger_processor_trap`: Executes two subsequent calls to another routine before triggering a processor trap.
- `call_and_trap_2`: Executes two calls to another routine and then triggers a processor trap.
- `call_and_trap`: Calls another routine and then executes a processor trap.
- `trigger_software_interrupt`: Executes a software interrupt trap to transfer control to the operating system.
- `trap_until_zero`: Calls another routine and executes a processor trap repeatedly until the returned value is zero, followed by one final trap.
- `conditional_trap_loop`: Executes a processor trap and continues to execute subsequent traps based on the value of a local variable.
- `resolve_relocated_pointers`: Iterates through a table of addresses to perform pointer relocation by adding offsets to stored values.
