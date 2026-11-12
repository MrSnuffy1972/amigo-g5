# Game 1379 (super cars ii (1991)(gremlin)[cr hyperion]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01379). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1832496 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 138 raw-word lines |
| Functions named and described | pass, 225 of 225 named (0 author symbols), 225 of 225 described (5 by rule, 219 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1034 of 1034 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4532 of 4532 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6348 |
| code | 17676 |
| data | 1808453 |
| padding | 19 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf:bootblock | 1 | 19 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4357 | 206 |

## Cases handled

- super cars ii (1991)(gremlin)[cr hyperion](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr hyperion](disk 2 of 2)/Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 38 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 996 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement
- 2 packed track region(s), 1800192 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Cars II (1991)(Gremlin)[cr Hyperion](Disk 1 of 2).adf:bootblock

- `trap_then_spin_forever`: Executes a trap instruction and then loops forever, so the boot block never returns control.
- `load_nu_text_and_call_stub`: It loads the address of a text string containing the repeated characters 'NuNuNuNu' and then calls an empty stub that returns to the caller immediately without doing anything.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_resource_header`: Checks a global word, calls a routine via a pointer, and if the result is not 253, loads a 32-bit value from a table using an index derived from the result and calls another routine.
- `update_table_and_register`: Loads a value from a table using an index and stores a register value into a global variable.
- `toggle_status_bit`: Compares a 16-bit value at a global offset to 25, and if equal, toggles the least significant bit of a 16-bit value at another global offset.
- `update_memory_blocks`: Adds 65537 to four consecutive 32-bit words starting at a specific memory address and applies a bitwise AND with an address to each result.
- `populate_pointer_table`: Iterates through a table of 16 16-bit values, uses a byte from a second table to calculate an index, and stores a series of calculated 32-bit pointers into a destination table.
- `restore_context_and_set_register`: Stores a byte from d1 into a global variable, sets bit 6 of a hardware register, writes 8 to a location at offset 156 from a pointer in a6, and restores the processor state.
- `set_bit_zero`: Sets bit 0 of d1 and branches to another routine.
- `set_bit_one`: Sets bit 1 of d1 and branches to another routine.
- `set_bit_three`: Sets bit 3 of d1 and branches to another routine.
- `set_bit_two`: Sets bit 2 of d1 and branches to another routine.
- `set_bit_four`: Sets bit 4 of d1 and branches to another routine.
- `clear_bit_zero`: Clears bit 0 of d1 and branches to another routine.
- `clear_bit_one`: Clears bit 1 of d1 and branches to another routine.
- `clear_bit_three`: Clears bit 3 of a byte in register d1 and branches to another routine.
- `clear_bit_two`: Clears bit 2 of a byte in register d1 and branches to another routine.
- `clear_cia_control_bit`: Clears bit six of the first CIA control register, writes an all-ones value to a shared register, and saves the previous byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_joystick_port_state`: Reads the mouse and joystick port movement registers, the right and middle button register and the fire-button register.
- `initialize_linked_list`: Iterates through a list of pointers, setting certain memory locations to zero and updating a global pointer-based structure.
- `call_audio_update_twice`: Calls another routine, checks a longword in a global variable, and if it is not zero, calls that same routine again with different values in d0 and d1.
- `modify_audio_length`: Loads a pointer to a global variable into a0, reads a word from it, and if that word is neither zero nor 2, writes a word from an offset of 8 in a0 to an offset of 68 from the custom chip base address, then decrements the word at the pointer in a0.
- `update_counter_and_register`: Sets bit 0 of d7, performs an OR operation on d0, writes the result to an offset of 150 from a pointer in a6, decrements a word at a pointer in a0, increments that pointer by 2, adds 16 to a1, increments d7, and loops if d7 is not equal to 4.
- `update_audio_registers`: Calculates an offset based on d0, sets a bit in a register at offset 150 from a pointer in a6, and writes several values to audio hardware registers using an offset derived from d1.
- `execute_subroutine_chain`: Calls a routine and then calls another routine.
- `conditionally_call_routine`: Checks a global variable and calls another routine multiple times if certain conditions are met.
- `reset_and_copy_memory`: Zeros out a block of memory, copies a large block of data from one location to another using a stride, and clears a small array of shorts.
- `copy_memory_with_stride`: Copies a block of data from a source address to a destination address, clearing certain offsets in the destination block during the process.
- `bulk_copy_memory`: Copies a large block of data from a source address to a destination address using a stride-based copy method.
- `load_table_to_globals`: Calculates an offset from a global variable and loads a sequence of words from a table into several global variables.
- `store_table_to_globals`: Stores a sequence of words from a set of global variables into a table at a calculated offset.
- `load_table_data`: Calculates an offset based on a global variable and loads a sequence of words from a table into a set of global variables.
- `merge_buffer_data`: Performs a bitwise OR and mask operation to merge data from a source pointer into a destination pointer, masking certain bits, while incrementing pointers.
- `copy_buffer_blocks`: Copies a block of data from a source pointer to a destination pointer, while also performing certain writes to fixed offsets relative to a destination pointer.
- `scatter_buffer_data`: Writes data from a source pointer to a destination pointer, while also writing specific values to fixed offsets relative to a destination pointer, within a nested loop structure.
- `return_from_subroutine`: the routine performs a single jump or return or any other instruction that results in a return or a jump to a new location.
- `execute_recursive_call`: The routine performs a single recursive call to itself before returning.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `conditional_execute_routine`: Checks if a global variable is zero and, if so, executes another routine.
- `update_function_pointers`: Updates two global function pointers if a register value meets certain range requirements, and stores a value into a table.
- `execute_repeated_routine`: Calls another routine and then executes a specific routine five consecutive times.
- `recursively_call_routine`: The routine calls itself indefinitely without performing any other operations or modifying any state.
- `recurse_indefinitely`: The routine calls itself indefinitely, creating an infinite loop of self-referential calls without performing any other operations or modifying any registers or memory.
- `initialize_global_variable`: Sets a global variable to zero, then repeatedly calls another routine while incrementing that global variable until it reaches a certain value.
- `check_global_and_call`: Checks if a global variable is zero and, if so, calls another routine.
- `execute_empty_routine`: the routine calls another routine and returns without performing any other operations.
- `empty_routine`: The routine is empty and performs no operations before returning.
- `execute_empty_routine_2`: the routine calls another routine and then returns without performing any other operations.
- `execute_empty_routine_3`: the routine calls another routine and then returns without performing any further operations.
- `execute_empty_routine_4`: this routine calls another routine and then returns without performing any other operations.
- `return_from_subroutine_2`: the routine executes a single instruction to return from a subroutine call.
- `empty_routine_2`: This routine contains no executable instructions.
- `execute_empty_routine_5`: this routine performs no operations and immediately returns to its caller.
- `copy_memory_block_wrapper`: This routine calls a routine that copies a block of memory from a source address to a destination address.
- `copy_memory_block_wrapper_2`: This routine executes a recursive call to itself before returning.
- `copy_memory_block_wrapper_3`: This routine executes a memory block copy operation.
- `execute_empty_routine_6`: the routine performs no operations and immediately returns.
- `copy_memory_block`: This routine performs a nested loop to copy a block of memory from a source address to a destination address.
- `execute_empty_routine_7`: the routine performs no operations and immediately returns or calls itself, resulting in an empty execution flow.
- `execute_recursive_call_2`: the routine performs a single call to itself before returning.
- `call_other_routine`: This routine calls another routine located at a fixed address.
- `call_subroutine_only`: This routine executes another routine and then returns.
- `configure_interrupt_and_copy_buffer`: Configures the certain interrupt enable bit and copies a block of memory using a stride.
- `update_global_pointer_value_2`: Compares a 16-bit value against 0, 2, 3, or 4; if none match, it calculates a new value using two global words and updates a third global word, otherwise it calls another routine.
- `update_global_pointer_value`: Checks a global flag and either adds a constant or a pointer-offset-based value to a global variable, clamping negative results to zero.
- `update_global_value`: Reads a value from a global variable, adds a constant, and stores the result back to that variable while clamping any negative result to zero.
- `initialize_string_and_copy_buffer_2`: Fills a memory range with characters and then copies a null-terminated string from a source address to a destination address.
- `initialize_string_and_copy_buffer`: Fills a memory range with forward slash characters and then copies a null-terminated string from a source address to a destination address.
- `set_global_constants`: Assigns specific constant values to two global variables.
- `set_global_values`: This routine assigns specific constant values to two global variables.
- `initialize_player_data_tables`: Clears and initializes a series of global variables and memory buffers with default or reset values.
- `compare_memory_blocks_2`: Compares a block of memory against itself and sets a global variable to one or zero based on whether the contents match.
- `assign_global_variables`: This routine assigns the value of one global variable to another, and then assigns the value of a third global variable to a second, before finally assigning the first updated value back to the third.
- `store_constant_to_offset`: This routine stores a constant value to a memory location calculated by adding an offset to a register-based pointer.
- `service_movement_interrupt_request`: Checks a hardware register bit, updates several global variables, and potentially calls a routine from a function table or increments a global counter.
- `copy_memory_block_3`: Copies 765 32-bit words from a source table to a destination table using a loop.
- `xor_global_byte`: Performs a bitwise XOR between a byte in d0 and a byte at a specific memory address and returns.
- `return_immediately`: The routine is empty and performs no operations before returning to the caller.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `call_set_global_bounds`: Call stub: calls set_global_bounds, then returns to its own caller.
- `process_buffer_state_table`: Checks a global word value and, if it equals four, executes a sequence of buffer manipulation and table decrementing routines before setting a global word to three.
- `update_and_recurse_state_pointers`: Updates global pointers and counters based on conditional flags, then recursively calls itself or dispatches new routine addresses or state-machine-driven execution paths based on values read from a data stream.
- `process_instruction_stream`: Iterates through a stream of 4-byte-aligned data, checking for a null terminator and validating a certain number of bytes.
- `nop_placeholder`: This routine is empty and performs no operations.
- `dispatch_recursive_state_machine`: This routine implements a complex state machine that dispatches or recursively calls other routines based on the value of a global variable.
- `execute_subroutines`: Calls two instances of itself recursively.
- `set_global_flag_and_recurse`: Sets a global variable to one and then calls itself recursively a fixed number of times.
- `swap_and_update_routine_pointer_2`: Checks if a register value is less than a constant and, if so, swaps a routine address with a global variable before executing.
- `decrement_table_and_accumulate`: Decrements a value in a table if it is non-zero and adds a value from a table to a global variable.
- `update_table_and_decrement_counter`: Decrements a global variable and increments a value in a table if certain conditions involving a table and a global variable are met.
- `conditionally_swap_and_execute_routines`: Checks if a global variable is less than a constant, and if so, conditionally swaps a routine address with a global variable before executing a sequence of routine calls and updating another global variable.
- `assign_global_pointer`: Calls a routine, then stores a value from a register into a global pointer, followed by further routine calls and a global assignment.
- `swap_and_update_routine_pointer`: Checks if a register value is less than a constant, and if so, conditionally swaps a routine address with a global variable before performing a sequence of recursive routine calls and updating another global variable.
- `update_function_pointer`: Calls a routine, updates a global pointer with a register value, and calls several other routines.
- `swap_and_update_pointers`: Swaps a routine address with a global pointer, updates a global pointer, and executes a sequence of routine calls.
- `reset_and_recursively_call_routine`: Sets a global variable to zero and then calls itself a fixed number of times.
- `recursively_clear_and_call`: Checks if a global variable's value is greater than one, then updates that variable with the routine's own address, sets it to zero, and calls itself twice recursively.
- `recursively_update_global_pointer`: Checks if a global variable's value is greater than one, then updates that global variable to a fixed value, updates another global variable with the routine's own address, and calls itself twice recursively.
- `set_routine_pointer`: Sets a routine pointer or a global variable to a constant value.
- `execute_triple_call`: Sets a global variable to a constant value and then calls another routine that same number of times.
- `clear_table_and_accumulate`: Clears a value in a table to zero and adds a value from a table to a global variable if certain conditions involving a table and a global variable are met.
- `copy_buffer_metadata`: This routine copies a block of data from a source pointer to a destination pointer while simultaneously writing specific values at fixed offsets.
- `copy_buffer_data_2`: Calls a routine that writes data from a source pointer to a destination pointer while writing specific values to fixed offsets.
- `copy_buffer_data_3`: This routine copies a block of data from a source pointer to a destination pointer, while simultaneously writing specific values at fixed offsets relative to the destination pointer.
- `copy_buffer_data`: Calls a routine that writes data from a source pointer to a destination pointer while writing specific values to fixed offsets relative to the destination pointer.
- `scatter_buffer_data_dest`: Writes data from a source pointer to a destination pointer while writing specific values to fixed offsets relative to the destination pointer.
- `execute_repeated_calls`: Executes another routine a fixed number of times using a loop counter.
- `execute_repeated_calls_scaled`: Executes another routine a fixed number of times while calculating and storing a scaled value.
- `execute_repeated_routine_2`: executes another routine a fixed number of times by decrementing a local counter or a global variable.
- `traverse_table_and_decrement`: Traverses a table while comparing values against a register and potentially executing a loop that decrements a global counter.
- `traverse_and_decrement_table_2`: Traverses a table while checking a global flag, decrementing a pointer, and potentially executing a loop that decrements a global counter.
- `increment_global_and_decrement_pointer`: Increments a global variable and decrements a pointer until it reaches a certain address, while checking a condition and potentially decrementing a global variable in a loop.
- `decrement_pointer_and_check_limit`: Decrements a pointer and checks if it has reached a specific memory address, while performing a secondary loop that decrements a global variable and checks a value against a stack-based or register-based limit.
- `decrement_table_elements`: Iterates through a table and decrements its contents, checking each element against a register value.
- `traverse_table_and_decrement_2`: Iterates through a table of values, comparing them against a register and decrementing a global counter if certain conditions are met.
- `clear_and_fill_memory_block`: Clears a small block of memory to zero using a nested loop and then fills a larger, non-contiguous memory range with a repeating byte pattern.
- `write_pattern_to_memory`: The routine writes a repeating byte pattern to a series of offsets in a memory block.
- `apply_bitmask_to_memory_2`: Reads values from a table and applies a bitmask to a memory block using bitwise OR operations.
- `apply_bitmask_to_memory`: The routine reads values from a table and applies a bitmask to a memory block using bitwise OR operations.
- `scale_and_store_value`: Calculates a scaled value based on a function return and stores it in a memory location.
- `update_global_table_values`: Updates a series of global variables with scaled values and performs a weighted redistribution of values in a table.
- `encode_value_to_buffer`: The routine encodes a value and its properties into a buffer, setting specific bits in a byte based on certain conditions.
- `set_global_bounds`: Checks if certain global variables fall outside a range and, if so, sets a set of global variables to specific constant values.
- `set_global_values_2`: Stores three specific constant values into three different global variables.
- `clear_and_execute_sequence`: Sets a global variable to zero and calls a sequence of other routines.
- `call_instruction_parser`: Calls a routine that parses a custom instruction stream to perform bulk structure member updates.
- `update_player_data`: Copies a block of memory, updates certain global variables, and converts a number to a string or character sequence.
- `swap_table_elements`: Copies bytes from one memory location to another and then swaps elements within a table if certain comparison conditions are met.
- `copy_memory_block_2`: Searches for a value in a table and then copies a fixed number of bytes from that location to a destination pointer.
- `update_string_from_table_2`: Parses a custom instruction stream and copies a block of memory from a table while updating a string buffer.
- `parse_custom_instruction_stream_r4`: Parses a custom instruction stream to perform bulk structure member updates.
- `execute_custom_instruction_stream`: calls a routine that parses a custom instruction stream to perform bulk structure member updates.
- `copy_table_segment`: Copies a segment of data from a source table or memory location to a destination pointer or a global variable.
- `invoke_custom_instruction_parser`: This routine calls a routine that parses a custom instruction stream to perform bulk structure member updates.
- `invoke_custom_instruction_stream`: Calls a routine that parses a custom instruction stream to perform bulk updates to structure members.
- `invoke_instruction_parser`: calls a routine that processes a custom instruction stream for bulk structure member updates.
- `invoke_custom_instruction_parser_2`: calls a routine that processes a custom instruction stream to perform bulk updates to a structure's members.
- `execute_custom_instruction_stream_2`: Invokes a routine that parses a custom instruction stream to perform bulk updates on structure members.
- `load_structure_parameters`: Checks a word in a global variable and, if non-zero, loads two 16-bit values from adjacent memory locations into registers d6 and d7 and loads a pointer from a third location into a1 before branching to another routine.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `parse_custom_instruction_stream`: Parses a custom instruction stream to perform bulk structure member updates.
- `update_bitmask_table`: This routine performs bitwise operations and shifts to update a destination table using values or bitmasks derived from a source table and a global variable.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_cyclic_index_and_transform_string`: Calls a logic routine and updates a cyclic index in a global variable by incrementing or decrementing it based on bits in a status byte, wrapping the value between zero and seven.
- `transform_and_copy_string_chars`: Iterates through a table of bytes, applying a case-conversion offset to certain characters and writing them to a destination buffer while updating a pointer.
- `update_pointer_with_wrap_2`: Checks a bit in a global variable to determine whether to increment or decrement a value through a pointer, wrapping the pointer to a certain limit if necessary.
- `update_pointer_and_copy_data`: Updates a pointer or wraps it based on a bit in a global variable and copies nineteen bytes from a table into a global variable.
- `copy_instruction_data_and_update`: copies nineteen bytes from a table-indexed location into a global variable and then updates a pointer using a conditional wrap-around check.
- `process_string_data`: Iterates through a memory range and writes or modifies bytes in a destination buffer based on values found in a source range.
- `decrement_and_copy_to_self`: Decrements a global variable and copies nineteen bytes from a global variable back into itself.
- `process_string_characters`: Iterates through a table of bytes, performing conditional offsets or replacements on certain character values before writing them to a destination buffer.
- `decrement_global_and_write_byte`: If a certain value is non-zero, it writes a byte to a memory location calculated from a pointer and that value, then decrements a global variable and calls itself recursively.
- `recursively_calls_itself`: The routine calls itself indefinitely, resulting in an infinite loop or stack overflow.
- `execute_self_recursive_loop`: the routine performs a recursive call to itself, resulting in an infinite loop or a stack overflow.
- `update_pointer_and_copy_struct_alt`: Updates a pointer or wraps it based on a bit in a global variable and copies nineteen bytes from a table into a global or struct variable.
- `update_pointer_with_wrap`: Checks a bit in a global variable to decide whether to increment or decrement a value through a pointer, wrapping that value to zero or a previously held value before calling another routine.
- `increment_and_wrap_pointer`: Increments a value through a pointer and resets it to zero if it matches a certain value before calling another routine.
- `decrement_and_wrap_pointer`: Decrements a value through a pointer and resets it to a previously held value if it reaches negative one before calling another routine.
- `traverse_and_lookup_byte`: Iterates through a memory range a fixed number of times and then uses a byte from that range to perform a secondary lookup in a global table.
- `clear_and_apply_logic`: Sets a global variable to zero and then performs a bitwise OR or bitwise AND-NOT operation on a memory range.
- `clear_and_apply_logic_alt`: Clears a global variable and applies bitwise logic to a buffer range.
- `copy_data_from_table`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_data_to_self`: Copies nineteen bytes from a global variable back into itself.
- `copy_data_to_self_2`: copies nineteen bytes from a global variable back into itself.
- `copy_data_from_table_2`: Copies nineteen bytes from a location calculated from a table offset into aget global variable.
- `copy_custom_instruction_data`: copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_custom_struct_data`: Copies nineteen bytes from a table location, calculated using an offset, into a global variable.
- `copy_custom_instruction_data_2`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `copy_custom_struct_data_2`: Copies nineteen bytes from a location calculated from a table offset into a global variable.
- `initialize_and_shuffle_table`: Initializes a sequence of values in a memory block, performs a series of swaps or updates to a table based on calculated values, and then writes specific constants or existing values back to a buffer or table.
- `update_score_string_and_table_6`: Converts a numeric value into an ASCII-encoded decimal string and updates a table of score-related values.
- `find_maximum_scaled_value`: Iterates through a list of ten values, calculates a scaled or offset value for each, and stores the maximum or a related value into a table.
- `update_score_table_and_string`: Iterates through a table, copying or adding values and converting a numeric value into a decimal string stored in a global buffer.
- `convert_number_to_string`: Calculates the hundreds and tens digits of a value and converts them into a character sequence.
- `check_global_conditions`: Compares multiple global variables against specific constant values and calls another routine if conditions are met.
- `invoke_buffer_logic_twice`: Loads a word from a global variable into d0, calls another routine with a pointer in a1, then if a global variable is non-zero, loads a different word into d0 and calls that same routine again.
- `invoke_buffer_logic_offset_20`: Loads a word from a global variable into d0, calls another routine with a pointer in a1, then if a global variable is non-zero, loads a different word into d0 and calls that same routine again.
- `invoke_buffer_logic`: Calls a routine that performs bitwise logic on a memory range and then returns.
- `apply_bitwise_logic_to_buffer`: Performs a bitwise OR or a bitwise NOT-AND (AND-NOT) on a range of memory, depending on a register value.
- `apply_bitwise_or_to_buffer`: Iterates through a memory range, performing a bitwise OR between the current memory contents and a value located at a fixed offset of two thousand words ahead in memory.
- `update_string_from_table`: Copies a block of memory from a table, calculates a decimal string representation of a value, and writes it to a global buffer.
- `copy_memory_blocks`: Copies two separate blocks of data from a source address to a destination address using a fixed loop count.
- `initialize_scaled_pointer_table`: Sets a global character, initializes a register with a specific bitmask, and populates a table with incrementing pointer-like values.
- `apply_bit_manipulation_to_table`: Iteratively applies a bit-shifting and masking pattern to a source table and stores the result in a destination table.
- `update_global_char`: Adds one to a value loaded from a memory location and stores it in a global variable.
- `parse_resource_header`: Checks a header magic number and, if a match is found, parses a header structure to copy a block of memory and calculate a total buffer size.
- `update_value_with_table_lookup`: Repeatedly doubles a value, using a table pointer to reset or modify the value if it reaches zero, for a fixed number of iterations.
- `process_memory_buffer`: Checks a global value, copies a block of memory, and then performs a loop that decrements a pointer and updates a global counter.
- `looped_shift_with_table_lookup`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementing a short counter derived from an input register until it reaches minus one.
- `copy_buffer_backwards_decrement`: Decrements a value from a global variable and copies a block of memory backwards using a pointer.
- `copy_buffer_backwards_zero_offset`: Copies a block of memory backwards using a pointer.
- `copy_buffer_backwards_increment`: Increments a value and then copies a block of memory backwards using a pointer.
- `copy_buffer_backwards_offset_two`: Increments a value by two and then copies a block of memory backwards using a pointer.
- `copy_buffer_backwards_offset_three`: Increments a value by three and then copies a blockly of memory backwards using a pointer.
- `copy_buffer_backwards_offset_four`: Increments a value by four and then copies a block of memory backwards using a pointer.
- `copy_buffer_backwards_offset_five`: Increments a value by five and then copies a block of memory backwards using a pointer.
- `copy_ring_buffer_offset_7`: Copies a block of memory backwards using a pointer, where the number of bytes copied is determined by a constant offset added to a counter.
- `shift_and_copy_buffer_elements`: Calculates a source and destination index, then copies bytes backwards from a source address to a destination address using a decrementing counter.
- `shift_and_copy_buffer_backwards`: Calculates a source and destination index, then copies a block of memory backwards using a decrementing counter.
- `copy_ring_buffer_offset_21`: Copies a block of memory backwards using a pointer, where the number of bytes copied is unable to be determined by a constant offset added to a counter.
- `copy_ring_buffer_offset`: Sets a counter to a certain value plus a constant, then copies a block of memory backwards using a pointer.
- `fill_ring_buffer_run_2`: Writes a byte value repeatedly backwards through a ring buffer, where the number of writes is determined by a global variable and a register.
- `fill_ring_buffer_run`: Writes a byte value repeatedly backwards through a ring buffer, where the number of writes is determined by a global variable and a register value.
- `populate_ring_buffer_backwards`: Iterates a fixed number of times to write a byte value into a ring buffer using a decrementing index.
- `fill_ring_buffer_increment`: Writes a byte value repeatedly backwards through a ring buffer, where the byte value is an incremented version of a register value.
- `fill_ring_buffer_decrement`: Iteratively writes a byte value to a ring buffer while decrementing the buffer index and a loop counter.
- `extract_bit_fields_to_table`: Iteratively reads bytes from a source pointer and shifts their bits into several registers using a carry-based bit-extraction method.
- `extract_five_bit_planes`: Reads a sequence of bytes and extracts certain bit positions into a set of five destination words.
- `extract_four_bit_planes`: Reads a sequence of bytes and extracts certain bit positions into a set of four destination words.
- `extract_bitplane_bits`: Reads a sequence of bytes and extracts certain bit positions into a set of destination words.
- `init_eight_table_entries`: Initialises eight consecutive eight-byte table entries, writing a fixed word and the value seven into each.
- `set_audio_registers`: Performs two loops that write specific 16-bit values to the audio hardware registers at offset 1984, then restores d7 from the stack before returning.
- `call_subroutine_if_flagged`: Checks if a global flag is non-zero, then calls another routine.
- `set_global_values_3`: Stores a constant value into a global variable and clears another global variable.
- `set_global_flag`: Sets a global variable to a constant value.
- `mt_init`: Walks a byte table to build a pointer table, clears several global variables, and resets certain counters or registers.
- `reset_global_and_audio_pointers`: Resets a global variable to zero and increments a global pointer until a certain value is reached.
- `set_four_audio_pointers`: Runs a helper routine four times, once for each audio channel.
- `start_audio_dma_channels`: Clears a global, performs delays, enables audio DMA channels, and loads four audio location pointers from a table.
- `parse_music_row_entry`: Parses one four-byte entry of music data for an audio channel.
- `update_pointer_and_bounds`: Updates a pointer or a counter-based value and checks or modifies its bounds.
- `parse_resource_header_2`: Parses a list of words to check for a termination or magic value, then copies a block of memory or executes a sequence of routine calls.
- `modify_memory_via_pointer`: The routine performs a sequence of instructions that modifies a value in memory using a pointer held in a register.
