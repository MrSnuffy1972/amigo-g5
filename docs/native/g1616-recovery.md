# Game 1616 (rick dangerous (1989)(micro style)[cr qtx][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01616). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (924432 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 109 raw-word lines |
| Functions named and described | pass, 247 of 247 named (0 author symbols), 247 of 247 described (12 by rule, 213 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2748 of 2749 (99.96%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5257 of 5706 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3027 |
| code | 22200 |
| data | 206352 |
| padding | 2165 |
| unused | 690688 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:rickdangrous | 2 | 166 | 9 |
| Run | 2 | 117 | 2 |
| type | 2 | 156 | 3 |
| Wait | 2 | 117 | 2 |
| Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:rick | 2 | 207 | 8 |
| Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 474 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4438 | 200 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |

## Cases handled

- runtime segment an address (play-long-joy): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 382 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2327 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rick dangerous (1989)(micro style)[cr qtx][a]/Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:rickdangrous: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `enter_supervisor_and_set_hardware` (rickdangrous): Switches to supervisor state through the system library, then writes to the CIA port, DMA control and interrupt enable registers. Kind: loader/environment setup with potential sabotage/integrity check. Pass path: not determined.
- `return_immediately` (runtime segment an address (play-long-joy)): The routine is empty and performs no operations before returning. Kind: sabotage path / dummy function. Pass path: the code following the call is skipped or the routine is a placeholder for a failed check.

## Functions


### Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:rickdangrous

- `enter_supervisor_and_set_hardware`: Switches to supervisor state through the system library, then writes to the CIA port, DMA control and interrupt enable registers.
- `merge_constant_via_routine`: Calls a one-instruction other routine that merges a constant into the data register.
- `tiny_ori`: Small routine of 1 instruction that merges 176 into d0.
- `store_index_and_call_handler`: Writes a scaled table index and a routine address into two global cells, then calls a other routine followed by a routine at a fixed address.
- `expand_backward_packed_data`: Reads a stream length from a header, then loops over flag bits.
- `expand_backward_packed_runs`: Decodes packed runs copied backwards from the end of the source, alternating literal copies with back-references whose lengths come from bit reads and tables.
- `decode_state_driven_packed_copies`: Keeps a state value that selects between a length decode and a distance decode, copying bytes backwards from the source for each back-reference.
- `copy_packed_runs_until_done`: Repeatedly reads length and distance codes from the packed bit stream and copies runs of bytes backwards from the source.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the argument pointer is non-null, copies sixteen longwords from a fixed offset in a table into that buffer.

### type

- `display_named_file_contents`: Opens the dos library, takes the first word of the argument string as a file name, and allocates a buffer to read the file into.
- `call_indirect_through_pointer`: Calls the routine whose address is held at a fixed offset from the base register, then returns.
- `call_indirect_from_table`: Calls a routine whose address is taken from a table cell at a fixed offset, then returns; the library facts list the dos Write call.

### Wait

- `build_handler_table_from_sizes`: Finds the largest value in a linked list, then fills a table sized from it with a counted pattern and stores a handler address in it.
- `copy_stack_and_jump`: Copies sixteen bytes from a memory location to the stack and then calls another routine.

### Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:rick

- `load_amiga_hunk_executable`: Walks the hunk records of an AmigaDOS executable, allocating memory for each code and data hunk and copying its contents in.
- `return_empty_stub`: Contains only a return and performs no work.
- `spin_register_until_zero`: Decrements a 32-bit register in a loop until it reaches zero, then returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 164 into d0.
- `decode_run_length_bytes_backwards`: Decodes a stream of run-length and back-reference codes, writing bytes backwards from the end of a buffer until the write pointer reaches the start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_wrap`: Decrements a byte counter for a given number of iterations and resets it to a space character whenever it reaches zero.
- `record_undecoded_routine`: The decompiler timed out on this three-instruction routine, so its behaviour cannot be recovered from the listed facts.

### Rick Dangerous (1989)(Micro Style)[cr QTX][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trap_and_init_hardware_state`: The routine invokes a trap, then writes a sequence of values to global and hardware-mapped locations, each followed by a call to the same helper, and then enters a polling loop that repeatedly calls the helper and tests flags until a condition is met.
- `return_immediately`: The routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_blocks`: copies a block of data from a source pointer to two different destination memory areas using a loop.
- `reset_hardware_and_globals`: Sets several global variables and hardware registers to zero or specific constant values.
- `evaluate_table_and_branch`: Checks a value in a table via an index and either calls another routine or updates a global variable and a status flag.
- `initialize_and_process_data_stream`: Resets hardware and global state, then parses a compressed data stream to populate a lookup table.
- `update_global_pointer`: Stores a pointer into a global variable.
- `process_bitfield_flags`: iterates through bits of a value to check specific flags and updates a global structure with values from a table based on those flags.
- `initialize_memory_tables`: clears several memory ranges, populates a table with specific values derived from an index, and fills a sequence of bytes with a repeating pattern.
- `parse_compressed_data_stream`: Decodes a custom compressed byte stream by interpreting control codes to expand data, handle jumps, and populate a lookup table.
- `set_byte_at_pointer`: Stores a single byte into the memory location pointed to by a register.
- `parse_bytecode_stream`: Iterates through a data stream to decode custom instruction bytes, updating internal counters and state structures.
- `process_conditional_byte`: Compares a byte from a pointer against several values and updates a table or a global byte based on the result.
- `manage_indexed_data`: Uses a byte from a structure to index into a table, updates several counters, and returns a calculated offset.
- `configure_hardware_registers`: Writes values to a structure, modifies hardware registers for DMA and interrupts, and sets up a pointer to a routine.
- `update_display_parameters_2`: Checks a bit flag in a global variable and writes several values to a memory location pointed to by a global variable.
- `update_display_parameters`: Checks a bit flag in a structure, then writes four values to a memory location pointed to by a field within that structure.
- `step_and_clamp_counter`: Adds a signed byte value to a counter and clamps the result if it falls below zero or exceeds a specified limit.
- `update_min_value_and_pointer`: Compares two byte values, updates a byte in a structure with the minimum of the two, and stores a routine address in that structure.
- `assign_routine_pointer`: If a value at an offset in a structure is zero, it stores a routine address into another offset within that same structure.
- `increment_accumulated_value_2`: Adds two bytes together, stores the result in a global variable, and updates a structure if the result is negative.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_repeated_routines`: Calls a specific routine multiple times in a loop.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `search_and_replace_string`: Searches through a data structure for a specific pattern and performs a character-based replacement.
- `search_and_replace_string_2`: Searches a table for a specific pattern using two counters to navigate a two-dimensional array.
- `update_string_table`: Iterates through a two-dimensional data structure to perform character-based replacements and pattern matching.
- `execute_triple_routine`: Calls a specific routine three times in succession.
- `set_global_byte`: Stores a constant byte into a global variable.
- `update_status_byte`: Stores the address of the routine itself into a global variable and then overwrites that same variable with a constant byte value.
- `process_cia_input`: Checks bits in a global variable and performs bitwise operations and assignments to a global variable while interacting with CIA hardware registers.
- `process_input_state`: Evaluates bits within a global variable and hardware input registers to calculate and store new values into a routine address and a global byte.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d7.
- `assign_routine_address`: Stores the address of the current routine into a global variable.
- `increment_and_reset_global`: Increments a global variable, calls two other routines, and then sets that same global variable to a fixed value.
- `execute_infinite_loop`: Enters an infinite loop that prevents further execution of the program flow.
- `wait_and_flip_bit`: Waits until a global variable is less than one, then performs a bitwise XOR on a register and a global variable.
- `setup_blt_registers`: Writes several constant values to a series of registers and memory locations, including hardware registers for DMA and bitplane operations.
- `shift_color_data`: Iterates through a source table and stores the left-shifted values into a global variable array.
- `transform_table_data`: Clears a sixteen-word block of memory and then performs a nested series of bitwise operations and shifts to transform the values of that block into a global variable table.
- `transform_color_table`: Zeros out a table and then performs a multi-pass transformation on sixteen words by applying conditional increments based on bitmasks and shifting values.
- `transform_and_copy_table`: Copies a block of memory to itself, then performs bitwise reductions on a sixteen-word table before shifting the results left and storing them in a global variable.
- `call_another_routine`: An empty wrapper that simply calls another routine and returns the concatenated results of two registers.
- `traverse_linked_list_structure`: Iterates through a linked list of data structures by following pointers until a null terminator is reached.
- `process_structured_data_list`: Iterates through a list of pointers, calling another routine for each element until a null terminator is encountered.
- `copy_structured_data`: Copies a series of bytes from a source buffer to a destination buffer using a non-contiguous pattern of offsets.
- `initialize_global_values`: Stores two specific longword values into two consecutive global variables.
- `update_obfuscated_values`: Performs bitwise shifts and arithmetic operations to update two global variables based on their current contents.
- `perform_logical_or`: Performs a bitwise logical OR operation on a data register using zero as the operand.
- `clear_high_bit_table_entries`: Walks a table of 523 six-byte records starting at a fixed base, clearing the top bit of the byte at offset 2 in each record and leaving the other bits unchanged.
- `mask_data_bytes`: Clears the most significant bit of bytes located at fixed offsets within a memory table.
- `clear_data_register`: Applies a bitwise logical OR with zero to a data register.
- `clear_high_bit`: Clears the top bit of bytes located at six-byte increments within a data structure.
- `mask_data_bytes_2`: Processes a data block by masking the high bit of bytes at regular intervals.
- `clear_high_bit_2`: Processes a memory block by masking the high bit of bytes at regular offsets.
- `mask_data_bytes_3`: Performs a bitwise AND operation on a series of bytes within a structured table.
- `clear_high_bit_3`: Clears the high bit of bytes in a table using a loop.
- `execute_conditional_calls`: Checks specific bits in a byte and conditionally calls another routine multiple times or once based on those bits.
- `validate_sprite_data`: Scans memory ranges to validate sprite data structures and updates bitfield data based on specific conditions.
- `initialize_sprite_object`: Populates a data structure with attributes and coordinate information loaded from a lookup table based on an input index.
- `execute_bytecode_instruction`: Updates a global state and performs a sequence of conditional operations based on the results of a register read.
- `execute_complex_loop`: Executes a series of calls to another routine within a loop that performs conditional arithmetic and bitwise checks on a return value to update a global variable.
- `assign_global_values`: Calls other routines and assigns a value from one global variable to another.
- `call_initialization_subroutines`: Calls two routines that transform a color table and assign global pointers.
- `assign_global_pointers`: Sets several global variables to specific addresses and values, then calls another routine.
- `process_data_table`: Iterates through a table using a pointer to perform conditional updates and assignments to global variables.
- `call_other_routine`: Calls another routine and then performs a block data copy from a lookup table.
- `call_unnamed_routine`: Performs a sequence of calls to another routine.
- `execute_bytecode_loop`: Decrements a value, then enters a loop that executes a routine multiple times while checking a global variable and performing blitter operations.
- `execute_bytecode_loop_alt`: Increments a value, then enters a loop that executes a routine multiple times while checking a global variable and performing blitter operations.
- `reset_and_call_routine`: Sets a function pointer to zero, clears a global variable, and then performs several calls to another routine.
- `execute_nested_loop_calls`: Executes a routine within a nested loop structure, continuing to iterate as long as the return values of the called routines are non-zero.
- `configure_blt_hardware`: Updates hardware registers for bitplane transfers and waits for the DMA controller to complete operations.
- `load_lookup_table`: Copies large blocks of data from a lookup table into a destination memory area using nested loops.
- `copy_structured_data_2`: Copies a series of bytes from a source table to a destination table using specific non-contiguous offsets.
- `wait_for_bit_clear`: Polls a global variable until a specific bit is cleared, then writes several values to global variables and hardware registers.
- `modify_byte_flags`: Performs bitwise OR and AND operations on specific bytes within a memory block pointed to by a register.
- `modify_bitmask_via_pointer`: Sets specific bits in a byte at a memory location pointed to by a register and clears the least significant bit in two other bytes at fixed offsets from that same pointer.
- `modify_bitmask_and_clear_flags`: Performs a bitwise OR operation on a byte at a pointer location and clears the least significant bit of two other bytes at specific offsets within the same memory block.
- `modify_bitmask_via_pointer_2`: Updates a byte at a pointer location by applying a bitwise OR with a specific mask and clears the least significant bit of two other bytes at fixed offsets from that same pointer.
- `modify_bitmask_via_pointer_3`: Sets specific bits in a byte and clears individual bits in two other bytes located at offsets within a memory block pointed to by a register.
- `modify_bitfield_data`: Sets specific bits in a byte and clears the least significant bit in two other bytes within a data structure pointed to by a register.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `modify_bitmask_via_pointer_4`: Sets specific bits in a byte pointed to by a register and clears the least significant bit in two other bytes at fixed offsets from that same pointer.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d1.
- `modify_bitfield_flags`: Sets specific bits in a byte and clears the least significant bit in two other bytes via a pointer.
- `modify_memory_bits`: Sets bits in a byte and clears the least significant bit in two other bytes via a pointer.
- `modify_memory_bits_2`: Performs bitwise operations to set specific bits and clear others at offsets from a pointer.
- `clear_memory_bits`: Clears the least significant bit of two bytes located at specific offsets from a pointer.
- `clear_memory_bits_2`: Uses bitwise AND operations to clear the lowest bit in two different memory locations.
- `clear_memory_bits_3`: Clears the lowest bit in two bytes at fixed offsets from a pointer using bitwise instructions.
- `clear_memory_bits_4`: Modifies two bytes at specific offsets from a pointer by clearing their least significant bit.
- `clear_memory_bits_5`: Clears the least significant bit of two bytes pointed to by a register.
- `clear_memory_bits_6`: Clears the least significant bit of two bytes at specific offsets from a pointer.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_memory_bits_7`: Clears the least significant bit of two bytes at specific offsets from a pointer through a large routine.
- `clear_memory_bits_8`: Clears the lowest bit in two bytes at specific offsets from a pointer.
- `clear_status_bits`: Clears the least significant bit of two bytes located at specific offsets from a pointer in a register.
- `clear_status_bits_2`: Performs a bitwise AND operation to clear a bit in two different byte locations relative to a pointer.
- `clear_status_bits_3`: Uses a bitwise AND to zero out the lowest bit of two bytes at fixed offsets from a pointer.
- `clear_status_bits_4`: Clears the first bit of two bytes stored at offsets from a pointer held in a register.
- `clear_status_bits_5`: Modifies two bytes at specific offsets from a pointer by clearing their lowest bit.
- `clear_status_bits_6`: Clears the lowest bit of two bytes at offsets from a pointer in a register.
- `reset_data_structures`: Iterates through a list of structures, setting specific word offsets within each structure to zero.
- `update_flag_byte`: Sets a bit in a byte at a specific offset from a pointer based on the state of a bit in a global variable.
- `zero_list_elements_2`: Iterates through a list of structures by following a pointer and sets two word-sized offsets to zero for each element until a terminator is reached.
- `zero_list_elements`: Iterates through a list of structures and sets two specific word offsets to zero until a terminator is found.
- `update_blitter_parameters`: Processes a list of descriptors to modify bitfield data and configure hardware registers for blitter operations.
- `process_virtual_machine_bytecode`: Iterates through a table of instruction descriptors to execute a custom bytecode interpreter.
- `reset_global_values`: Sets several global variables and memory locations to zero, six, or two hundred fifty-five.
- `reset_global_values_2`: Assigns specific constant values to multiple global variables and memory addresses.
- `reset_global_values_3`: Initializes a series of global variables and memory locations with zero, six, or two hundred fifty-five.
- `clear_subroutine_pointer`: Sets a global variable to zero.
- `initialize_global_constants`: Sets several global variables to specific byte values, including zero, six, and two hundred fifty-five.
- `initialize_global_constants_2`: Sets several global variables to specific constant values, including zero, six, and two hundred fifty-five.
- `assign_address_pointer`: Loads a memory address into a pointer.
- `initialize_global_state_variables`: Resets several global variables to zero, sets two to a specific small integer, and sets two others to a maximum byte value.
- `initialize_global_state`: Sets several global variables to zero, six, or two hundred fifty-five.
- `reset_game_state_variables`: Clears several global variables and sets specific memory locations to zero, six, or two hundred fifty-five.
- `clear_and_call_routine`: Calls another routine and then sets a global pointer to zero.
- `process_bcd_value`: Performs BCD adjustments and arithmetic operations on a value passed in a register and stores the results in multiple global variables.
- `clear_and_call_routine_2`: Checks if a function pointer is non-zero, calls it, and then sets that pointer to zero.
- `execute_and_clear_function_pointer`: Checks if a global function pointer is non-zero, executes the routine it points to, and then clears the pointer to zero.
- `clear_and_execute_callback`: Checks if a global pointer is non-null, executes the routine at that address, and then clears the pointer to null.
- `fill_memory_pattern`: Fills a memory range starting at a pointer with a repeating pattern of bytes.
- `load_table_entry_to_globals`: Calculates an offset into a table using a multiplier and loads a series of words and a longword into global variables.
- `copy_and_initialize_data`: Copies a block of memory from one location to another using a loop and then initializes several global variables and pointers with values from a data structure.
- `clear_memory_segments`: Clears four non-contiguous blocks of memory by writing zeros to them in a loop.
- `initialize_state_and_call_routines`: Sets two global variables to a maximum byte value and executes a sequence of four routine calls.
- `clear_memory_blocks`: Clears four large, non-contiguous blocks of memory by writing zeros to them in a loop.
- `update_table_index`: Updates a global index and pointer based on values read from a table, including conditional logic for resetting offsets and updating a secondary pointer.
- `set_flag_byte`: Stores zero into a global variable.
- `tiny_ori_6`: Small routine of 1 instruction that merges 16 into d0.
- `clear_global_byte`: Sets a specific global byte to zero.
- `decrement_and_update_timer`: Decrements a global variable and, if it reaches zero, calculates a value using subtraction and stores it in a global word.
- `clear_offset_word`: Stores zero into a word at a specific offset from a pointer and then calls another routine.
- `initialize_specific_values`: Sets a global variable to two hundred fifty-five, assigns a constant to another global, and stores a specific value into a third global.
- `clear_memory_offset_value`: Writes a zero value to a memory location calculated by adding an offset to a pointer held in a register.
- `execute_conditional_callback`: Checks a global variable and a numeric range before conditionally calling another routine and potentially performing a bitwise check on a structure member.
- `reset_system_state`: Clears several global variables and sets specific routine pointers to zero or specific values.
- `initialize_global_state_2`: Clears several global variables and sets specific memory locations to zero or fixed values.
- `clear_multiple_globals`: Clears several global variables and sets a specific global to a constant value.
- `initialize_global_state_3`: Clears several global variables to zero and assigns specific constant values to a set of other global variables and function pointers.
- `clear_multiple_globals_2`: Resets a collection of global variables to zero and assigns specific constants.
- `clear_multiple_globals_3`: Performs zeroing of multiple global variables and sets specific byte values.
- `clear_multiple_globals_4`: Clears a set of global variables including one specific memory location.
- `clear_all_globals`: Resets a large number of global variables, pointers, and memory locations to zero or specific constant values.
- `set_global_byte_2`: Sets a global byte to one.
- `assign_function_pointers`: Stores several addresses and values into global variables and memory locations.
- `assign_function_pointers_2`: Writes multiple addresses and values into a set of global variables.
- `assign_function_pointers_3`: Stores several addresses and values into a set of global variables and memory locations.
- `assign_function_pointers_4`: Calls another routine and then assigns several values and addresses to various global variables and pointers.
- `reset_system_state_2`: Clears multiple global variables and resets various system state flags and routine pointers to zero or specific constant values.
- `reset_state_and_check_flag`: Clears several global variables and sets a specific memory location to a constant value before performing a conditional check on a global byte.
- `update_offset_variable`: Adjusts a global variable by adding or subtracting eight based on the value of a pointer and performs conditional checks on a numeric range.
- `manage_recursive_table`: Checks a global flag to decide whether to traverse a table of pointers recursively.
- `return_concatenated_registers`: Calls another routine and returns the concatenated values of two registers.
- `update_global_flags`: Calls another routine and, if a condition is met, updates a global variable and sets a specific bit in another global variable.
- `validate_multiple_ranges`: Checks if two separate values fall within their respective defined ranges stored in a structure.
- `check_range_bounds`: Compares two short integers against two separate ranges defined by values stored within a structure.
- `nop_placeholder`: This routine is empty and performs no operations.
- `validate_range_bounds`: Checks if a value derived from a global variable and a pointer falls within two specified ranges defined by other global variables.
- `check_bit_and_bounds`: Verifies if a specific bit is set in a memory location calculated from two registers and checks if other registers fall within specific global bounds.
- `update_tile_attribute_data`: Calculates a memory offset using bitwise operations on registers and updates a series of bytes within a data table.
- `modify_byte_via_pointer`: Performs a bitwise OR operation on a byte located at an offset from a pointer.
- `store_long_value`: Stores a long value into a memory location at a fixed offset from a pointer.
- `update_structure_index`: Checks a value in a structure, and if a condition is met, increments a field and retrieves a value from a table via a pointer.
- `execute_conditional_callback_2`: Calls a routine and then executes a series of other routines if a specific carry flag is set.
- `process_table_entry`: Retrieves a value from a table using an index stored in a structure, updates the index, and calls another routine.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `execute_empty_call`: The routine performs a single call to another routine and then returns.
- `update_structure_data`: Updates values within a structure based on a counter, performing different arithmetic and calls depending on whether the counter is zero.
- `modify_memory_via_pointer`: Stores a constant value at an offset from a pointer and decrements a short value at another offset.
- `calculate_and_update_data`: Performs a calculation using a register value and a constant, stores the result in a structure, and calls other routines based on a boolean result.
- `update_entity_state_and_position`: Updates an entity's internal index and pointer based on a lookup table, while incrementing and checking coordinate bounds.
- `reset_entity_timer_on_flag`: Sets a four-byte value at an offset from a pointer to zero, then checks a specific bit in a byte at another offset; if that bit is set, it writes a constant value to the same four-byte location.
- `update_entity_position_and_timer`: Updates an entity's coordinate fields using 32-bit fixed-point arithmetic and modifies internal timer or counter values.
- `accumulate_structure_value_2`: Increments a field in a structure, performs a 32-bit arithmetic operation using a table, and adds a value to a long field in the structure.
- `update_entity_state_and_bounds`: Updates an entity's spatial coordinates and internal status flags based on conditional checks of its current velocity and movement bounds.
- `update_entity_movement_state`: Updates an entity's spatial coordinates and internal counters based on conditional checks of its current velocity and status flags.
- `update_entity_spatial_state`: Updates an entity's coordinate offsets and internal counters based on conditional checks.
- `recursively_calculate_offset_accumulation`: This routine uses recursion to traverse a structure based on comparison of two short integer fields, incrementing a counter at a specific offset during each step.
- `accumulate_structure_value_3`: Updates a long field in a structure by adding a value retrieved from a table based on conditional checks and byte values.
- `update_structure_field`: Modifies a structure by updating a counter and adding values from a table to a long field based on specific conditions.
- `update_and_accumulate_value`: Resets a short field, increments a counter, and adds a value from a table to a long field in a structure based on conditional checks.
- `update_entity_position_and_state`: Updates an entity's coordinate offsets and internal counters based on conditional checks.
- `update_entity_physics_and_counters`: Updates an entity's movement coordinates, direction flags, and internal counters based on conditional checks.
- `update_entity_state_and_accumulator`: Modifies an entity's status flags, direction, and internal counters based on conditional checks.
- `update_entity_velocity_and_state`: Modifies an entity's velocity and state values based on conditional checks of its current direction and internal counters.
- `update_entity_position_and_velocity`: Updates an entity's coordinate and velocity values by applying an offset retrieved from a table based on the entity's current state and direction.
- `update_entity_state_and_accumulate_offset`: Updates an entity's internal counters and state flags based on conditional checks and table lookups.
- `toggle_state_and_accumulate_offset`: Toggles a two-byte value between zero and a maximum value based on a comparison with a global variable.
- `toggle_and_accumulate`: Toggles a short value between zero and a constant and adds a value from a table to a long field in a structure.
- `decrement_and_accumulate`: Decrements a short field and updates a long field in a structure using values from a table.
- `update_structure_field_with_table_offset`: Copies a word from one offset to another within a structure and toggles a short value between zero and a specific constant.
- `accumulate_structure_value`: Updates a value in a structure by adding an offset derived from a table based on various conditional checks and byte values.
- `accumulate_offset_value`: Adds a value from a global table to a longword in a structure, using different source values based on the contents of specific bytes within that structure.
- `accumulate_value_with_offset`: Adds a value from a global variable to a longword in a structure, applying an additional offset if a specific byte in that structure is non-zero.
- `increment_accumulated_value`: Adds a value from a global variable to a longword in a structure, optionally adding an offset if a specific byte in that structure is non-zero.
- `initialize_structure_data`: Sets specific bytes and words within a structure and performs BCD arithmetic via multiple routine calls.
- `search_table_for_bounds_2`: Validates bounds against global variables or iterates through a table of descriptors to find an entry matching specific range criteria.
- `search_bounds_table`: Iterates through a table of values to find an entry that satisfies specific range comparisons against values stored in a structure.
- `search_table_for_bounds`: Iterates through a table of values to find an entry that satisfies multiple range comparisons against values stored in a structure.
- `return_empty_stack`: This routine is empty and performs no operations before returning.
- `return_stack_value`: returns the value currently stored at the top of the stack.
- `check_bounds`: Compares two short integers against a minimum and maximum range stored within a structure.
- `verify_multiple_ranges`: Performs several range checks by comparing values derived from global variables and pointers against bounds stored in a structure.
- `set_high_bit`: Sets the most significant bit of a byte located at an offset within a structure pointed to by a pointer in a structure.
- `calculate_composite_collision_mask`: Computes a bitmask by performing bitwise OR operations on multiple bytes retrieved from a table using various offsets.
- `update_collision_mask`: Performs a bitwise OR of multiple bytes retrieved from a table using offsets, applies bitwise AND operations with specific masks, and updates a global variable with the result.
- `update_masked_global`: Calculates a bitmask using different table offsets based on a bit test and applies it to a global variable.
- `mask_global_variable`: Calculates a bitmask by performing bitwise OR operations on several bytes from a table and applies it to a global variable.
- `return_empty_stack_2`: The routine performs no operations and immediately returns a value from the stack.
- `execute_bytecode_interpreter_loop`: Iterates through a sequence of addresses stored in a table, executing them as a custom bytecode stream.
- `clear_memory_block`: Fills a specific block of memory with zeros using a loop.
- `clear_global_byte_2`: Calls two other routines and then sets a specific global byte to zero.
- `copy_memory_block`: Copies a block of sixteen words from one location to another and toggles a global byte.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `mask_bytes`: Performs a bitwise AND operation on two consecutive bytes using a constant mask.
- `empty_routine_2`: This routine contains no executable instructions and performs no operations.
- `process_nested_loops`: Executes nested loops that check specific bits in registers and perform repeated routine calls.
- `copy_data_block`: Copies a block of data from a source table to four destination memory locations using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `record_undecoded_routine_2`: The decompiler timed out on this seven-instruction runtime routine, so its behaviour cannot be recovered from the listed facts.
