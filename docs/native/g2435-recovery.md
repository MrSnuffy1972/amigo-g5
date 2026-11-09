# Game 2435 (kult - the temple of flying saucers (1989)(exxos)(m3)[cr band][m tfs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02435). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904944 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 562 of 562 named (0 author symbols), 562 of 562 described (42 by rule, 517 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4848 of 4849 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 80682 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10729 of 10729 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 587838 |
| code | 41004 |
| data | 255359 |
| padding | 10503 |
| unused | 10240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:RUN | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:KULT | 2 | 9262 | 496 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:type | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:RUNBACK | 3 | 740 | 42 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:ENDCLI | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:intro | 1 | 99 | 3 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:bootblock | 1 | 35 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 219 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:PUZZL.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 00C0) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr band][m tfs]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:PUZZL.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 80682 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:intro

## Copy protection found

- `initialize_system_environment` (KULT): Sets up initial global pointers and constants, configures interrupt enable bits, and initializes a sequence of memory-based buffers or strings. Kind: disk hardware/track check. Pass path: the branch where bVar5 is false (the 'if (!bVar5)' or 'if (!(bool)uVar6)' paths that continue execution or proceed to further initialization/loading or the code_r0x01000b08 label).

## Functions


### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:RUN

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch`: Copies a longword from one register to another and then performs a relative branch to another routine.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:KULT

- `initialize_system_environment`: Sets up initial global pointers and constants, configures interrupt enable bits, and initializes a sequence of memory-based buffers or strings.
- `update_audio_state_and_global_logic`: Increments a global variable and replaces it with an offset-based value from a table, then executes a sequence of conditional checks that update a global variable and clear or set other global variables or pointers.
- `check_global_self_assignment`: Checks if a global variable is not equal to itself, then calls another routine and assigns the variable to itself.
- `process_conditional_pointer_logic_2`: Checks a global variable and, if a certain condition is met, executes a loop that calls routines using pointers retrieved from a table.
- `move_stack_pointer`: Moves the contents of a register into a memory location.
- `process_conditional_pointer_logic`: Checks a global variable and, depending on its high bits, either calls a routine at a pointer-based address or executes a sequence of logic that updates several global variables and checks certain thresholds.
- `update_global_values_conditionally`: Updates several global variables or modifies a global variable based on a threshold check.
- `store_pointer_and_decrement`: Stores a 16-bit value from a pointer-based memory location into a register and then calls a routine that decrements a register.
- `increment_and_load`: Increments a register and loads a byte from a memory location relative to that register into a register.
- `recurse_and_return_registers`: This routine calls another routine and returns the contents of two registers without modifying memory or registers other than those used during the recursive call.
- `process_char_and_recurse`: Updates a global variable and a memory location with a character derived from an input, then calls another routine recursively if a terminator is not reached.
- `update_and_recursively_traverse_char`: Updates a global variable with a character derived from a register and checks it against another global variable.
- `traverse_string_and_set_registers`: Iterates through a string, setting certain registers to a value or a pointer, until a terminator character is found.
- `search_string_for_terminator`: Iterates through a string or table, checking for a terminator character and recursively calling another routine with a pointer or a value from an offset.
- `traverse_string_with_recursive_calls_2`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine.
- `clear_memory_range_2`: Zeros out a range of memory starting from a global variable pointer back to itself.
- `traverse_string_with_recursive_calls`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine recursively.
- `negate_and_load`: Negates a register and loads a word from a relative offset into another register.
- `check_dos_info_validity`: Calls a routine via a pointer, and if the result is non-zero, calls another routine and checks a condition involving a value from a memory location before returning a constant.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `execute_conditional_dos_calls`: Calls a routine and, if the result is non-zero, calls two other routines and sets a return value to a maximum unsigned 16-bit value.
- `set_register_constants`: Loads three constant longwords into registers and calls a routine, potentially triggering further calls or setting a return value to a maximum unsigned 16-bit value.
- `load_register_and_branch`: Loads a constant value into a longword register and performs a short branch to another routine.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `apply_bitmask_to_memory_range`: Increments a global variable and then iterates through a list of offsets to apply a bitmask to a range of memory via bitwise operations and bit-rotated values.
- `traverse_linked_list_and_apply_mask`: Iterates through a linked list of memory offsets, checking bit patterns and updating a global variable or terminating if a specific end-of-list or control-bit pattern is encountered.
- `scan_for_pattern_and_decrement`: Iterates through a list of values, comparing them against a global value and decrementing a counter if certain conditions or patterns are unable to be met.
- `scan_variable_for_pattern`: Iterates through a memory range, checking if certain bits of a byte match a specific pattern and returning a calculated or incremented counter.
- `find_maximum_pattern_match`: Iterates through a memory range using a pattern-searching routine and tracks the highest-value match found until a certain global condition or a specific value is met.
- `calculate_pattern_distance`: Calculates the unsigned distance between a global variable and a value returned by a pattern-searching routine, updating a global variable with the result until a loop condition or global comparison is met.
- `check_and_assign_pointers`: Checks if a global variable equals a specific constant, then assigns constant values and a pointer to a global variable before performing recursive calls to itself.
- `check_global_value`: Checks if a global variable equals a specific constant and calls another routine recursively if it does not.
- `set_global_constants_2`: The routine assigns specific constant values to certain global variables and stores a pointer to a global variable before calling other routines.
- `process_bit_mask_loop`: Iteratively shifts a value left and updates a mask or bit-manipulation state using a decrementing counter or loop control variable.
- `sum_and_rebalance_count`: Accumulates the return values of a routine into a local variable until a global condition is met, then adjusts a global counter by a calculated offset.
- `decrement_and_branch_loop`: Calls another routine, tests a register, and if it is zero, branches away; otherwise, it decrements a register.
- `decrement_and_loop`: Decrements a register and repeats the routine until the register reaches a specific value.
- `increment_and_branch_to_subroutine`: Increments a register, clears a register, loads a byte from a pointer, sets a new pointer, calls a subroutine, and copies a 32-bit value from one memory location to another.
- `configure_bitmask_parameters`: Calculates and stores a set of bitmasking parameters and pointer offsets into a global structure or table.
- `write_table_from_memory`: The routine calls another routine and then writes a certain number of bytes from a memory location into a table at a calculated offset.
- `initialize_and_fill_buffer`: Sets a global pointer and a counter, then executes a loop that writes a null byte to a global variable while iterating through a memory range using a fixed offset.
- `update_global_pointer_with_offset`: Iterates through a list of pointers, adding a pointer's value to itself, and finally stores a new calculated pointer into a global variable.
- `traverse_buffer_with_jumps_2`: Updates a register and traverses a buffer, advancing a pointer by a value read from the buffer until a certain condition is met.
- `traverse_buffer_with_jumps`: Updates a global variable with a value and then traverses a buffer, advancing a pointer by a value read from the buffer, until a certain condition is met.
- `tiny_lsl`: Small routine of 1 instruction that shifts d0 left by 2.
- `load_address_to_register`: Loads a longword from a memory location into a register.
- `calculate_scaled_index`: Masks bits from a register, shifts the result, and combines it with a scaled value from another register into a single 32-bit return value.
- `set_pointer_and_branch`: Loads a constant into a register and branches to another routine.
- `initialize_pointer_and_branch`: Clears a register, loads a pointer from a memory location, and branches to another routine.
- `initialize_buffer_pointers`: Calls another routine, subtracts a register from another, clears two registers, and loads two bytes from a pointer-incrementing memory access.
- `store_shifted_value`: Clears a register, copies a register, shifts it left, and stores the result into a memory location.
- `load_pointer_from_address`: loads a longword from a fixed memory location into a register.
- `tiny_subq_subq`: Small routine of 2 instructions that subtracts 1 from d2, then subtracts 1 from d3.
- `apply_bitmask_to_buffer_complex`: A nested loop routine that performs bitwise rotation and logical masking, updating a destination buffer and a secondary buffer offset by a large amount, while adjusting a pointer or counter.
- `load_and_process_bytes`: Loads a pointer from a memory location, calls another routine, clears registers, reads two bytes from a pointer, calls a third routine, and stores a bit-shifted value into a global variable.
- `permute_bit_patterns`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value while iterating through nested loops and updating a buffer.
- `permute_bits_in_word`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value.
- `decrement_and_loop_counter_2`: Decrements a register and calls a routine repeatedly until the register reaches a terminal value.
- `decrement_and_loop_2`: Decrements a 16-bit signed integer and repeats a routine call until the value reaches negative one.
- `modify_register_value`: Loads a longword from a memory location into a register, shifts it left, and subtracts a small constant.
- `apply_rotated_bitmask_2`: Loads a value from a memory location and executes a loop that calls other routines and updates a global counter or a table or a buffer using a bit-rotated mask.
- `apply_bitmask_loop`: Performs a loop that executes a routine via a pointer-offset while updating a global variable using a bitwise mask.
- `execute_indirect_loop_2`: Loads a value from a memory location, updates a global variable, and enters a loop that calls another routine via a pointer-offset.
- `execute_indirect_loop`: Loads a value from a memory location, updates a global variable, and enters a loop that calls another routine, executes a function pointer, and increments a global variable.
- `apply_mask_to_buffer`: Performs a loop that executes a routine via a pointer-offset and updates a global index using a bitwise mask.
- `modify_register_value_2`: loads a longword from a global variable into a register, shifts it left by one, and then subtracts two from the result.
- `iterate_function_pointer_table`: loads a pointer from a memory location and executes a sequence of routine calls while incrementing a global counter and decrementing a loop counter.
- `execute_masked_index_loop`: Executes a loop that calls a routine via a pointer-offset and updates a global variable using a bitwise mask.
- `decrement_and_wrap_index_2`: Loads a value from a memory location, sets a memory location to a constant, calls a routine, and then decrements a global variable while wrapping it to a certain value if it reaches zero.
- `decrement_and_wrap_index`: Loads a value from a memory location and executes a loop that decrements a global variable, wrapping it to a certain value if it reaches zero.
- `decrement_and_loop_with_masking`: Decrements a local variable and a global variable, resetting the global variable to a certain value if it reaches zero, while calling a routine that applies a bit-rotated mask to a buffer and an infinite loop routine.
- `loop_with_fixed_decrement`: Sets a global variable to a constant and executes a nested loop structure that decrements a local variable and a global variable.
- `nested_decrement_loop`: Sets a global variable, calls a routine, and then executes a nested loop that decrements a register and a global variable while calling other routines.
- `decrement_and_loop_with_mask_params`: Decrements a local variable and a global variable while calling routines that merge or shift data and calculate mask parameters.
- `execute_recursive_calls_3`: Performs a sequence of recursive calls and buffer rotations or zeroing.
- `execute_recursive_branching`: Performs a recursive traversal where the number of subsequent recursive calls depends on whether a global variable is zero or non-zero.
- `test_byte_and_branch_to_subroutine`: Tests a byte at a memory location and conditionally calls another routine.
- `test_byte_and_modify_register`: Tests a byte in a global variable and conditionally branches or calls another routine.
- `execute_conditional_calls`: Checks if a global variable is zero and, if so, calls another routine multiple times, otherwise calls another routine twice.
- `execute_conditional_recursive_calls`: Performs a variable number of recursive calls to itself based on whether a global variable is zero or non-zero.
- `branch_on_global_zero`: Checks if a global variable is zero and, if so, calls another routine twice; otherwise, it calls another routine twice regardless.
- `branch_on_memory_test`: Tests a memory location and, if certain conditions are met, calls another routine and modifies a register.
- `update_table_with_values`: Searches a table for a zero value and then writes several register or local values into a table at a calculated offset.
- `set_register_and_branch`: Loads a constant value into a register and branches to a different location in the code.
- `set_table_entry_zero`: Calculates an offset based on an empty input value and sets a value in a table to zero.
- `check_table_and_loop`: Loads a pointer, compares a value at that pointer with a register, and executes a loop that increments the pointer or terminates.
- `decrement_and_loop_with_mask_params_2`: Decrements a local variable and calls a routine that calculates mask parameters and merges or shifts data.
- `decrement_and_loop_counter`: Decrements a value and executes a loop that calls another routine until a certain condition is met.
- `tiny_addq`: Small routine of 1 instruction that adds 1 to a6.
- `copy_table_blocks`: Copies large blocks of data from one memory area to another using a loop.
- `swap_table_elements`: Swaps or mirrors certain blocks of data between two memory areas using a loop.
- `clear_global_index`: Calls several routines and sets a global variable to zero.
- `execute_empty_calls`: This routine is empty, calling other routines without further logic.
- `update_pointer_and_clear_register`: Increments a pointer, clears a register, loads a byte from a pointer, and branches to another routine.
- `process_input_loop`: Checks a global variable and enters a loop that calls a routine to check button input and updates a global variable.
- `rotate_and_mask_result`: Executes a loop that calls a routine to rotate and invert bits, then returns a masked version of the result.
- `rotate_bits_and_invert`: Performs a bitwise rotation and inversion on a value from a global variable and returns it.
- `mask_and_delay_input`: Repeatedly calls another routine while masking a bit in a global variable, continuing as long as a specific bit remains set.
- `iteratively_mask_and_delay_input`: Repeatedly calls another routine while masking a bit in a global variable, continuing as long as a specific bit remains set, then resets the global variable and executes a final routine call.
- `check_value_and_branch`: Compares a memory value to a constant, branches if equal, calls a subroutine, sets registers to constants, checks a memory value, and calls another subroutine or returns.
- `process_recursive_loop`: Clears a global variable and enters a loop that calls another routine and passes a concatenated 16-bit value if a certain condition is met.
- `execute_conditional_loop`: Executes a loop that calls another routine and terminates if a certain condition is no longer met.
- `execute_routine_recursively`: Repeatedly calls a routine and checks a boolean flag, continuing or terminating the loop based on the return value or a flag update.
- `copy_data_blocks`: Copies a block of words from a source table to a destination buffer using a loop.
- `merge_and_shift_buffer_elements`: Merges specific bits from a source buffer into a destination buffer and copies subsequent buffer elements while incrementing pointers.
- `merge_and_shift_data_offset`: Merges bits from a source buffer into a destination buffer using a different starting offset and shifts pointer or data pointers.
- `update_global_pointers`: Copies several words from a source address to a global table and updates a pointer to a source buffer.
- `merge_buffer_elements_loop`: Executes a loop that performs bitwise OR and AND operations on memory or buffers and calls an infinite loop routine.
- `execute_infinite_loop_3`: The routine performs an infinite loop by calling itself recursively.
- `set_source_pointer`: Loads a pointer from a global address into a register.
- `load_multiple_registers_from_list`: Clears two registers and loads several bytes and a longword from a pointer-incrementing list into registers.
- `apply_bitmask_and_rotate_to_buffer`: Iterates through a source buffer, applying a bitmask-based bit-rotation or bit-shifting and writes the resulting bit-manipulated or bit-aligned words into a destination buffer.
- `load_address_from_memory`: Loads a longword from a specific memory address into a register.
- `apply_bit_rotation_to_buffer`: Iterates through a source buffer and writes bit-rotated or bit-shifted versions of specific bit-fields or words into a destination buffer using a nested loop structure.
- `permute_bits_in_buffer`: Performs bitwise masking and bit-rotation-like operations on a buffer using a calculated bitmask.
- `calculate_mask_parameters`: Calculates bitmask values and updates a global variable used for masking or bit-width calculation.
- `initialize_and_rotate_buffer`: Zeros out a buffer range and then applies a bit-rotated mask to a source buffer, writing the results to a destination buffer.
- `apply_bit_rotation_to_buffer_2`: Iterates through a buffer to apply bitwise masking and bit-rotation-like operations to specific memory offsets.
- `execute_recursive_calls`: Performs recursive calls to another routine and initializes a buffer with zeros.
- `apply_rotated_bitmask`: Calculates a bit-rotated mask and uses it to bitwise-AND and OR values into a table or buffer using a loop controlled by a global counter.
- `apply_mask_to_offset_buffer`: Applies a bit-rotated mask to a buffer using bitwise operations, where the source buffer is an offset version of the destination buffer.
- `load_and_initialize_buffer`: Loads a longword from a fixed memory location into a register and initializes a buffer with zeros.
- `initialize_buffer_and_mask`: Fills a buffer with zeros and updates global variables used for masking or bit-width calculation.
- `search_table_for_match`: Iterates through a table, checking if a masked byte from a fixed address matches a value in the table, and if so, updates a global variable with values from the table.
- `recursively_process_table_data`: Checks a byte in a table and, if it is not certain, recursively calls itself while updating global variables with values derived from a pointer-based lookup.
- `decrement_register_loop`: Decrements a value in a register until it reaches a certain limit.
- `populate_buffer_from_indexed_data`: Calculates an offset into a data table, retrieves values, and performs a series of nested loops to write these values and others into a target buffer.
- `setup_buffer_pointers`: Loads a constant into a register and calls two other routines.
- `process_buffer_with_offsets`: Iterates through a buffer, applying bitwise operations and pointer offsets, and calls other routines recursively.
- `update_table_with_branching`: Iterates through a table, performing different routine calls based on a bitmask check, and finally sets several global variables to specific values.
- `apply_bitmask_to_buffer`: Iterates through a buffer, performing bitwise OR, bitwise AND, and bit-rotation or bit-swapping operations before storing the result back to the buffer.
- `mask_and_store_bytes`: Applies a bitmask or its inverse to bytes in a buffer and stores them back into a buffer located at a calculated offset.
- `return_void`: The routine is empty and performs no operations before returning.
- `initialize_buffer_values`: Sets a global variable to a specific value and copies a sequence of words from one memory location to another, while also clearing a global variable.
- `update_global_pointer_constants`: Sets a global variable to a constant value based on its current state and then updates a pair of global variables with a new pointer value.
- `set_global_variable`: Assigns a constant value to a global variable based on a condition and copies a word from one memory location to another.
- `update_state_machine_and_pointers`: Updates a global state variable and executes a sequence of conditional branches that swap or assign pointer-based values and constants to a table of memory locations.
- `apply_bitmask_to_buffer_2`: Iterates through a buffer, applying a bitmask-based bit-rotation or bit-shifting and writes the resulting bit-manipulated or bit-aligned data to a destination buffer.
- `execute_recursive_calls_2`: Executes a sequence of calls to another routine.
- `set_table_and_registers`: Tests a memory location and, if certain conditions are met, calls another routine and writes constants to a memory location.
- `update_table_with_rotated_mask_2`: Updates a table using a bit-rotated mask and a loop that checks a certain limit.
- `update_table_with_rotated_mask_2_2`: Sets a global variable and certain registers, then enters a loop that updates a register and checks a calculated limit against a counter.
- `update_table_with_rotated_mask`: Uses a bit-rotated mask to perform bitwise AND and OR operations on specific offsets within a table.
- `initialize_and_iterate_table_limit`: Calculates a limit or offset based on a pointer-derived value and a register-multiplied constant, then enters a loop that increments a counter until it matches that limit.
- `apply_bitmask_to_buffers`: Calculates a bitmask using a bit-rotation-like operation and applies it to several distant memory offsets via a pointer.
- `execute_subroutine_chain`: This routine acts as a dispatcher that calls a sequence of other routines without performing any intermediate logic or data manipulation.
- `set_address_and_branch`: Stores a constant longword into a fixed memory location and branches to another instruction.
- `modify_buffer_bits`: Iterates through a buffer and modifies specific bits in a series of words based on a bitmask check.
- `toggle_bits_in_buffers`: Iterates through memory buffers to flip specific bits in a word based on a calculated bitmask.
- `toggle_bit_and_branch_2`: Clears a bit in a register, stores a word to an offset from a pointer, and then checks a bit in another word to branch or set a certain bit.
- `toggle_bit_and_branch`: Clears a bit in a register, stores a word to an offset from a pointer, and then checks a bit in another word to either branch or set a bit in a register.
- `modify_buffer_and_branch`: Clears a bit in a register, stores a register value to an offset from a pointer, and then performs conditional checks and branches.
- `modify_buffer_and_loop`: Clears a bit in a register, stores a register value to an offset from a pointer, and enters a loop that decrements a register and branches based on a loop counter.
- `modify_bit_patterns`: Iterates through a pointer and modifies specific bits in a destination pointer, a second pointer, a third pointer, and a fourth pointer based on a bitmask.
- `store_long_and_branch`: Stores a long-word constant into a memory location and then performs a short branch to another location.
- `update_bit_flags`: Iterates through a memory range and updates specific bits in several remote memory locations using a bitmask and a counter-based loop.
- `recurse_self_infinitely`: This routine calls itself indefinitely, creating an infinite recursive loop.
- `execute_unbounded_recursion`: Performs a sequence of recursive calls to itself, where the number or type of calls depends on the value of a global variable.
- `execute_unconditional_recursion`: Performs a sequence of recursive calls to itself, where the number of calls or the arguments passed depend on the value of a global variable.
- `execute_empty_calls_2`: This routine is empty and only calls another routine.
- `test_and_update_global_pointer`: Tests a byte at a global pointer, and if zero, clears a register and calls another routine.
- `update_global_counters_2`: Checks a memory location, clears a register, and calls two different routines while updating a global variable and incrementing a pointer.
- `process_table_recursively`: Performs an infinite loop by calling itself recursively.
- `copy_memory_block_strided`: Performs a nested loop to copy a block of memory from a source pointer to a destination pointer using a fixed stride.
- `get_unaligned_word`: Reads a 16-bit value from a pointer offset by two bytes.
- `update_pointer_and_store_value`: Reads a 16-bit value from a pointer, then updates a global pointer and a global variable with a new address.
- `advance_pointer_with_backup`: Stores the value at a global pointer into another global variable before incrementing that pointer by two bytes.
- `load_address_to_pointer`: Loads a constant 32-bit value into a pointer.
- `set_global_bounds`: Sets a global variable to a specific value and another global variable to a constant integer.
- `execute_infinite_loop`: Performs an empty infinite loop.
- `add_interrupt_handler`: Adds a new interrupt handler to the exec interrupt vector table and then calls the previous handler.
- `open_libraries_recursively`: Calls a library function via a pointer and checks the return value, repeating the process or entering an infinite loop if a return value is zero.
- `initialize_buffer_pointers_2`: Checks if several pointers retrieved from a global variable are non-zero and, if so, assigns new pointer values or offsets to a series of global variables.
- `execute_task_related_functions`: Calls three different functions via pointers retrieved from a global variable and its offsets, involving task and port management.
- `initialize_screen_pointers`: Opens a screen, sets several pointers to offsets from a returned address, and copies a small number of words from a source to a destination.
- `update_buffer_pointers`: Updates certain global variables using values derived from a pointer and a source buffer, while storing a byte and aword from that buffer into a destination pointer.
- `calculate_interpolation_parameters`: Calculates scaling or interpolation values and stores them in global variables before entering a loop that modifies a pointer or counter.
- `update_swapped_bytes`: Updates a global variable with a byte-swapped value and enters a loop that modifies a global variable via addition or subtraction.
- `execute_infinite_loop_with_swapped_bytes`: Updates a global variable with a swapped-byte value and enters a loop that modifies a global variable via addition or subtraction while checking a condition or calling another routine.
- `populate_table_from_source_2`: Iteratively copies or updates bytes in a destination memory area from a source pointer, using offsets and conditional checks against a table.
- `copy_displaced_bytes`: Copies bytes from a source pointer to a destination pointer with a fixed offset and performs a conditional copy or update based on a linked list or table.
- `populate_table_from_source`: Iteratively copies bytes or related data from a source pointer to a table while calculating offsets and checking for zero values.
- `expand_bitstream_to_buffer_2`: Expands a compressed bitstream into a destination buffer by extracting individual bits and shifting them into a word-aligned output.
- `expand_bitstream_to_buffer`: Decodes a bitstream by extracting bits from a source table and shifting them into a destination buffer.
- `copy_data_blocks_unrolled`: Copies a block of words from a source table to a destination buffer using an unrolled loop that processes four words per iteration.
- `expand_bitstream_to_buffers`: Extracts individual bits from a source table and shifts them into a set of destination buffers.
- `populate_bitstream_buffer`: Extracts bits from a source table and shifts them into a destination buffer, effectively performing a bit-stream or bit-unrolling operation.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `apply_bitmask_to_memory_buffer`: Iterates through a memory range, applying a bitwise mask and logical ORing a bit-rotated or bit-inverted value to a series of memory offsets.
- `rotate_and_branch_on_byte`: Checks if a byte at a pointer address equals one, then rotates a long register and branches or increments registers.
- `apply_bitmask_to_memory`: Performs bitwise masking and logical ORing on memory contents at a certain offset using a bit-rotated or bit-inverted value.
- `decrement_pointers_and_add`: Decrements two pointers by a constant amount and adds a word from a register to another register before branching.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_range_bounds`: Calculates and writes new range boundaries into global variables and executes a loop that modifies these bounds or pointers.
- `update_global_counters`: Calculates and stores new values in several global variables and executes a loop that decrements a value while calling another routine.
- `copy_memory_block`: copies a block of 4000 words from a source address to a destination address using a decrementing counter.
- `clear_registers_and_load_byte`: Clears two registers and loads a byte from a pointer-based offset into a register while incrementing a pointer.
- `set_pointer_and_loop`: Stores a byte into a global variable and executes a loop that increments a pointer until it reaches a previously stored pointer value.
- `process_memory_range_and_counters`: Iterates through a memory range, applying a bitmask to certain values and updating several global counters or pointers based on the data encountered.
- `calculate_pointer_offset`: Calculates a new pointer or value based on bit-shifting and conditional logic involving a global variable and a table.
- `execute_empty_recursive_loop`: The routine performs an infinite or recursive call to itself without any side effects or state changes.
- `update_pointer_offset`: calculates a new pointer or value using bit-shifting and conditional logic involving a global variable and a table.
- `load_byte_and_increment_pointer`: Loads a byte from a pointer-based offset into a register and increments a pointer.
- `update_pointer_metadata`: Calls a routine to populate a pointer table, then executes a loop that decrements a value and modifies a pointer-based structure, specifically decrementing a value at an unable offset and updating a pointer-based address.
- `increment_pointer_and_load_byte`: loads a byte from a memory location offset from a pointer and increments that pointer by two bytes.
- `modify_pointer_table_values`: Calculates a new value based on a pointer, updates a global variable, and then executes a loop that modifies a value in a table.
- `populate_pointer_table`: Iterates through a list of pointers to calculate and store new pointer values and associated metadata into a table.
- `iterate_and_recurse_on_byte`: Loads a byte from a memory location and enters a loop that executes a routine a fixed number of times, recursively calling itself regardless of the byte's value.
- `store_global_byte`: Loads a byte from a global variable and then stores it back into the same global variable.
- `populate_buffer_with_values`: Populates a memory buffer with a specific byte value and then clears a series of global registers or memory locations.
- `initialize_and_populate_buffer`: Clears a memory buffer and then copies a variable number of words from a source pointer to that buffer, using bit-mask-based conditional checks to determine the number of words to copy.
- `branch_to_subroutine`: Performs a subroutine call and then a branch to another location.
- `process_bitstream_metadata`: Initializes a global variable and certain pointers, then searches through a table to check for a bitmask match or a certain value.
- `traverse_table_and_clear`: Recursively calls another routine and checks a global variable before setting a global variable to zero.
- `dispatch_on_char`: Performs unconditional calls to another routine if a global variable matches any of several specific character constants.
- `shift_loaded_values`: Loads two words from a memory location into registers and performs a logical right shift on one of them.
- `check_input_range`: Checks if two input bytes fall within a range defined by a table.
- `set_registers_and_branch`: Loads a constant into a register, sets another register to a constant, and then branches to a new location.
- `search_for_valid_index`: Increments a value and checks it against a range, potentially setting a global or updating a pointer if a valid range or value is found.
- `invoke_subroutines`: Executes two calls to another routine.
- `find_matching_index_in_table`: Iterates through a table to find a matching index and updates a pointer or a global variable.
- `search_for_matching_pattern`: Iterates through a memory list to find an entry where a certain masked bit pattern matches a changing global value.
- `update_global_pointer_and_offset`: Updates a global variable with a value from a pointer-based offset and calculates a new pointer offset based on a value stored at that pointer.
- `load_pointer_and_calculate_offset`: Loads a value from a pointer-based offset into a global, updates a global with a byte from a pointer, and calculates a new pointer offset based on a value in memory.
- `initialize_global_state_conditional`: Checks if a global variable is below a certain value and, if so, performs several conditional checks and writes certain values to a pointer-based address.
- `tiny_move`: Small routine of 1 instruction that copies 150 to d1.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `load_and_check_table_value`: calculates a table offset, loads a word from that offset and a word from a fixed address into a global, and checks if the loaded value has changed from its previous state.
- `load_offset_based_pointers`: Calculates an offset by multiplying an input by four, then loads a word from a table at that offset and a word from a fixed address into global variables, and finally calculates a new address or offset and stores it in a global variable.
- `execute_if_value_changed`: Checks if a global value differs from a previously stored value and calls another routine if a change is detected.
- `process_conditional_pointer_logic_3`: Checks a global variable and, depending on its high bits, either calls a routine at a pointer-based address or executes a sequence of logic.
- `set_global_pointer_and_branch`: Stores a constant word into a global variable and branches to another routine.
- `set_global_pointer_and_branch_2`: Stores a different constant word into a global variable and branches to another routine.
- `update_global_and_decrement`: Sets a global variable to a constant, decrements a byte at a pointer address, and calls another routine.
- `update_table_values`: Iterates through a table, checking if a byte equals certain a value and replacing it with a different value.
- `update_table_with_logic_2`: Sets a global variable to zero, calls another routine, and then checks certain values in a table to potentially update a global variable or branch.
- `update_pointer_and_branch_2`: stores a word into a global pointer and branches to another routine.
- `update_table_with_logic`: Sets a global variable to zero, calls another routine, and then checks certain values in a table to potentially update a global variable or a pointer-based address.
- `decrement_and_branch`: Decrements a byte at a pointer address and branches to another location.
- `dispatch_pointer_logic_complex`: Sets a global variable and checks a pointer, then executes different logic or calls a routine based on the high bits of that pointer.
- `dispatch_conditional_pointer_logic`: Evaluates a global variable's high bits to either execute a pointer-based routine, perform a sequence of recursive routine calls, or iterate through a table of function pointers.
- `initialize_global_state_2`: Sets a global variable to zero or one and checks a character at an offset from a pointer-based address.
- `dispatch_pointer_based_logic`: Evaluates a global variable to determine a branching or calculation path, potentially updating a pointer or executing a routine via a table-based offset or a direct pointer-based jump.
- `resolve_conditional_pointer`: Loads a word from a table or pointer-based address and branches or executes logic based on the high bits of that value.
- `modify_global_pointer`: Loads a byte from a global address, compares it to a register, and if certain conditions are met, modifies a global pointer and performs bitwise masking before returning.
- `invoke_instruction_stream`: Calls a routine that processes a sequence of instructions from a memory buffer, performing no other operations before returning or passing control.
- `set_global_constant`: Sets a global variable to a constant value.
- `dispatch_table_pointer_logic`: Evaluates a global pointer or a value from a table to determine a new execution target or a memory-based pointer.
- `execute_pointer_logic`: Loads a value from a global variable and executes a sequence of logic or calls another routine depending on the high bits of that value.
- `compare_and_branch_values`: Compares a byte at a certain address against certain thresholds and branches or moves a new value into a global variable.
- `dispatch_virtual_machine_instruction`: Evaluates a global variable to select a target address or instruction, then executes a sequence of conditional jumps, recursive routine calls, or a loop that dispatches routine pointers from a table based on a calculated offset.
- `dispatch_conditional_pointer_logic_2`: Checks a global variable and, depending on its high bits, either executes a sequence of logic or calls a routine via a pointer-based address.
- `dispatch_pointer_logic_table`: Checks a global variable and, depending on its high bits, either executes a sequence of recursive calls and arithmetic or dispatches a pointer-based routine or a table-driven callback.
- `dispatch_conditional_pointer_logic_3`: Checks a global variable and, depending on its high bits, either executes a sequence of recursive calls and arithmetic or dispatches a pointer-based jump or a loop that executes routines from a table-based address or a pointer-offset address.
- `set_global_constants`: overwrites a global variable with two sequential constant values.
- `execute_fixed_pointer_logic`: Sets a global variable to a fixed value and executes a sequence of logic or calls another routine.
- `clear_and_mask_table_elements`: Iterates through a table of 40-byte-aligned entries, clearing the high bit of the second word in each entry.
- `shuffle_and_dispatch_pointer`: Swaps memory contents using a stride-based loop and then checks a global pointer or a masked bit-flag to either execute a routine or update audio and memory-mapped registers.
- `set_register_and_branch_3`: Sets a bit in a memory location, clears several registers, loads values from a pointer, and calls a subroutine before branching.
- `dispatch_pointer_table_callbacks`: Determines a target address or pointer-based routine by checking the high bits of a loaded value and then iterates through a table of function pointers, executing them if the table-traversing byte meets certain criteria.
- `dispatch_conditional_pointer_logic_4`: Checks a global variable and, depending on its high bits, either executes a sequence of logic or calls a routine via a pointer-based address.
- `dispatch_virtual_machine_instruction_2`: Executes a sequence of conditional or indirect instructions by checking high-order bits of a pointer or a global variable.
- `dispatch_conditional_pointer_logic_5`: Checks a global variable and, depending on its high bits, either executes a sequence of logic or calls a routine via a pointer-based address.
- `dispatch_indirect_function_pointer`: Evaluates a global value to determine a target address or a function pointer stored in a table.
- `resolve_pointer_from_register`: Loads a value from a register and executes a sequence of logic or calls another routine depending on the high bits of that value.
- `iterate_and_store_data`: Iterates through a table, checks a bitmask against a value, and stores certain bytes and pointers into a global location.
- `apply_mask_to_pointer_table`: loads a pointer from a memory location and applies a bitwise mask or bit-manipulated value to the memory pointed to by that pointer.
- `dispatch_conditional_pointer_logic_7`: Checks a global variable and, depending on its high bits, either calls a routine at a pointer-based address or executes a sequence of logic.
- `execute_pointer_loop`: Sets a global variable to a fixed value and then calls a sequence of routines via a pointer-based loop.
- `execute_subroutine_sequence`: Calls two subroutines and then performs a conditional branch, followed by writing a constant to a hardware register and an unconditional branch.
- `update_global_parameters`: Calculates and stores new values into several global variables based on existing register or global data, including a clamping check.
- `execute_conditional_pointer_loop`: Iterates through a list of pointers or logic, calling another routine or accessing memory based on the value of a global variable.
- `execute_branching_subroutines`: Executes a sequence of subroutines and writes a constant to a hardware register before branching to another routine.
- `reset_global_parameters`: Sets several global variables to specific constant values and enters a loop that continues while a specific bit in a global variable is set.
- `update_global_state_and_pointer_table`: Updates a global variable using a clamped or masked value and applies a bitmask-driven pointer-based update to a table of global variables.
- `dispatch_conditional_pointer_logic_6`: Checks a global variable's high bits to either execute a specific routine or iterate through a table of function pointers.
- `load_pointer_and_set_byte`: Loads a longword into a register from a memory location, writes a single byte to a memory location, and then performs an unconditional short branch.
- `scatter_data_to_buffers`: Iterates through a loop, calling a routine that copies or scatters data into a destination, and then executes a secondary loop.
- `scatter_table_data`: Copies bytes or words from a source table to a non-contiguous destination pattern.
- `clear_and_load_registers`: Clears certain registers and loads two bytes from the stack or a nearby memory location into registers, while also incrementing the stack pointer.
- `dispatch_table_callbacks`: Writes certain values to memory and iterates through a buffer, calling a routine via a pointer or handling specific byte-based branch cases.
- `calculate_offset_address`: Calculates a new address or offset based on specific bits in a register and stores it in a global variable.
- `load_and_branch_data`: Loads a byte from a pointer-incremented address into a register, branches to another routine, clears a register, and loads another byte from a pointer-incremented address.
- `clear_and_process_buffer`: Clears a block of memory to zero and then iterates through a buffer to perform bitwise operations and store results.
- `set_pointer_and_flag`: Writes a constant value to a memory location, writes a byte to another memory location, and branches to another routine.
- `move_and_branch_2`: Moves a word from one memory location to another and branches to another routine.
- `set_flag_and_branch`: Writes a constant value to a memory location and branches to another routine.
- `decrement_and_store_word`: Decrements a word from a memory location and stores the result into another memory location.
- `process_nested_loops`: Performs nested loops that check bits in a value and update a global variable or a register.
- `update_counter_based_on_flags`: Checks bits in a global variable and either increments or decrements a global counter or resets it to zero.
- `clear_global_variable`: Sets a global variable to zero.
- `copy_and_rearrange_buffer_blocks`: This routine performs a block-based memory copy that interleaves or redistributes data from a source buffer into a destination buffer using a calculated offset.
- `get_conditional_byte`: Checks flags in a byte and either returns a byte from a nearby offset or calculates a new address using a table lookup and a pointer-based offset.
- `check_bit_conditions`: Checks a series of bit-masking conditions against registers and returns without performing any side effects if any condition fails.
- `check_bit_and_branch`: Checks a bit in a byte and branches or calls other routines based on the result.
- `store_conditional_byte`: Checks a bit in a value and stores a byte from a source to a destination, regardless of the bit's state.
- `read_unaligned_word`: Checks a value and, if it is zero, constructs a word from two bytes located at an offset from a pointer.
- `process_button_input`: Checks certain bits in a register and updates a global variable based on the state of those bits.
- `update_mouse_position_and_sprites`: Updates a global counter or pointer and calculates a new sprite or pointer position based on mouse port movement.
- `increment_and_dereference_global`: Increments a global variable and then replaces its value with the value stored at that new offset in a table.
- `increment_and_load_pointer`: Increments a global variable and returns the value stored at the address or offset pointed to by that incremented value.
- `update_memory_constants`: Calls a routine, writes a constant to a memory location, calls another routine, and then writes zero to that same memory location.
- `modify_buffer_with_condition`: Checks a range or condition and clears a bit in a buffer if certain criteria are met.
- `copy_source_to_destination`: Calculates a source address based on a global variable and an offset, then copies a fixed number of words to a destination address.
- `increment_frame_pointer`: increments the value stored in the a6 register by one.
- `clear_and_load_register`: Clears a register and loads a byte from a memory location into it.
- `copy_memory_with_stride`: Calculates a memory range and copies a fixed number of words from a source table to a destination table using a loop.
- `load_destination_address`: Loads a constant address into a register.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `patch_data_table_and_update_nibbles`: Copies a block of data from a global variable to a destination table, then iterates through a memory range to increment or decrement specific nibbles within certain words.
- `update_memory_values`: Copies a sequence of words from a source address to a destination address and then updates a range of memory with modified values based on bitwise masking.
- `clear_buffer_and_check_table`: Zeros out a buffer and calls another routine if a bit is set in a table.
- `store_register_to_memory`: Stores the contents of a register into a memory location pointed to by another register.
- `validate_byte_range`: Checks if a byte loaded from a memory location falls within a specific range and branches to another routine if the value is outside or certain bounds are met, otherwise jumps to a different location.
- `validate_byte_range_2`: Clears a register and compares a byte from a global memory location against two constant values, branching to another routine if the value is within a specific range or otherwise branching to a distant address.
- `increment_frame_pointer_2`: increments the value stored in the address register a6 by one.
- `execute_recursive_loop`: Executes a routine that calls itself recursively while decrementing a register, repeating a fixed number of times.
- `check_pointer_and_recurse`: Checks if a pointer or value is non-zero and calls itself if so.
- `increment_a6_register`: adds one to the value stored in the a6 register.
- `traverse_and_process_buffer`: Iterates through a fixed-size sequence of memory-aligned words and calls another routine if a non-zero value is encountered.
- `update_bitmask_table`: Iterates through a table to check if a byte matches a masked value, then updates a byte in that table and a byte in another memory area.
- `search_and_update_buffer_2`: Iterates through a table to check if a byte matches a masked value, then updates a byte in the table and a byte in another memory area.
- `set_address_register`: Sets an address register to zero and branches to another location.
- `search_and_update_buffer`: Iterates through a table to check if a byte matches a masked value, then increments a byte at a calculated destination address and updates a global variable.
- `compare_memory_and_branch`: Compares a longword from a memory location and branches if a condition is met.
- `scan_buffer_for_pattern`: Iterates through a buffer and checks if certain bits in a byte match a specific pattern, then recursively calls another routine and updates a pointer.
- `load_and_check_value`: Clears a register, loads a byte from a pointer, calls another routine, and loads further bytes into registers while adjusting a pointer.
- `initialize_and_loop_on_bitmask`: Sets a global variable to zero and overwrites it with values from registers, then enters a loop that executes other routines and checks if a specific bit in the global variable remains set.
- `execute_recursive_calls_4`: performs a sequence of recursive calls to a routine before returning.
- `set_address_registers`: Loads several constant values into address registers and calls another routine.
- `execute_recursive_call`: Performs a recursive call to itself while incrementing a register value.
- `traverse_memory_recursively`: Executes several routine calls, including a recursive call that increments a register and other routines that process bitmasking or memory offsets.
- `apply_bitmask_and_recursively_call`: Performs a recursive call, applies a bitmask to a byte at an offset from a register, and then executes a sequence of further recursive calls.
- `modify_byte_and_recurse`: Copies a byte from one location to another and clears a bit in a byte at an offset from a pointer.
- `check_and_modify_memory`: Compares a byte at a pointer-based address to a constant and, if it matches, writes a byte to an offset relative to a register.
- `modify_memory_with_offsets`: Reads a byte from a pointer and writes modified or offset values to adjacent memory locations relative to a register-based address.
- `load_address_and_branch`: Loads a constant address into a register and branches to another routine.
- `load_address_and_branch_2`: loads a constant address into a register and then performs a short branch to another location.
- `set_address_and_branch_3`: loads a constant address into a register and performs a short branch to another location.
- `dispatch_conditional_pointer_logic_8`: Evaluates a pointer or a value from a table based on a character-based index, then either executes a sequence of recursive calls, jumps to a routine, or iterates through a list of function pointers until a terminator or a specific character limit is reached.
- `loop_with_decrementing_counter`: Sets a global variable and certain registers, then enters a loop that decrements a register while calling other routines.
- `save_registers_and_call`: Saves all registers to the stack, calls a subroutine, and then branches to another location.
- `execute_nested_loop`: Executes a nested loop structure that decrements registers, effectively performing a fixed number of empty or secondary routine calls.
- `update_masked_buffer`: Performs a bitwise or-not and and-with operation while copying or updating values in a large buffer using a nested loop structure.
- `copy_buffer_segments`: Copies data from a source buffer to a destination buffer using a nested loop structure.
- `update_buffer_with_mask`: Performs bitwise operations on a buffer using a mask and updates a global variable, then enters a loop that increments a register until a certain value is reached.
- `conditional_loop_on_byte`: Checks if a byte at a pointer is non-zero and then executes a loop a certain number of times, potentially calling another routine once more if the byte was non or zero.
- `loop_fixed_count`: Executes a loop that decrements a register from a fixed starting value down to a certain limit, calling another routine within the loop.
- `decrement_two_counters`: Executes a loop that decrements two different registers simultaneously, calling another routine with a changing or fixed value.
- `clear_global_in_loop`: Executes a loop that decrements a register and sets a global variable to zero during each iteration.
- `execute_infinite_loop_2`: This routine is empty and executes an infinite loop.
- `execute_recursive_loop_2`: Recursively calls another routine while decrementing a local variable until it reaches a certain value.
- `execute_descending_loop_sequence`: Executes another routine a fixed number of times while decrementing a local variable in a loop.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_pointer_offset`: Calls another routine and sets a global variable to a value derived from a bit-shifted byte.
- `process_buffer_and_update_memory`: Iterates through a buffer, updating a global variable and a memory location, and calls various subroutines based on certain conditions.
- `initialize_global_state`: Sets a global variable to a specific value and calls another routine.
- `reset_global_variable`: Sets a global variable to zero and then to a maximum byte value.
- `apply_relative_offset`: Iterates through a memory range and adds a value stored at a global pointer to the routine's own address if that value is non-zero.
- `validate_and_reset_global_range`: Performs a series of checks on memory and registers, potentially resetting a global variable to a certain value.
- `reset_global_range`: Sets a global variable to a zero value and then to a maximum byte value.
- `search_string_and_set_pointer`: Iterates through a buffer to find a specific character and sets a pointer or a global variable.
- `set_register_bounds`: Sets a register to a specific constant value and sets another register to an offset relative to a global variable.
- `search_and_modify_bitmask`: Searches a table for a matching value, updates a pointer and a memory location, and clears a bit in a calculated memory address.
- `modify_bitmask_and_pointer`: Sets a register to a constant, copies a byte from a memory location, and modifies a byte at a calculated pointer address using a bitmask.
- `clear_memory_locations`: Sets two specific memory-mapped or global addresses to zero.
- `search_string_and_set_address`: Iterates through a buffer looking for a specific character and sets a pointer or a global variable if a match is found.
- `set_memory_registers`: Assigns a constant to a global variable and updates another global variable with an offset relative to a third global variable.
- `search_and_update_bitmask`: searches a table for a matching byte, updates a pointer to a new value, and clears the high bit of a byte at a calculated memory offset.
- `update_pointer_and_bitmask`: Sets a global variable to a constant, copies a byte between memory locations, and modifies a byte at a calculated offset using a bitwise mask.
- `reset_global_variables`: clears a global variable and a secondary global variable by setting them to zero.
- `update_global_state`: Checks a global variable and updates several memory locations with a constant or a new value if certain conditions or string-traversal results are met.
- `dispatch_conditional_pointer_logic_9`: Checks a global variable and, depending on its high bits, either calls a routine at a pointer-based address or executes a sequence of logic.
- `check_and_clear_byte`: Compares a byte from a global variable against a constant, and if not equal, writes a constant to a global variable and clears a byte in another global variable.
- `clear_and_mask_byte`: Clears a byte in a global variable and another global variable, then conditionally clears a bit in a byte at a specific offset if a bitmask check passes.
- `test_and_call_routine`: Tests a byte at a global address and, if not zero, loads a constant into a register and calls another routine.
- `reset_global_and_branch`: Sets a global variable to zero and executes a sequence of other routines based on certain value checks.
- `tiny_move_2`: Small routine of 1 instruction that copies 63 to d3.
- `search_for_byte_in_list`: Iterates through a list of bytes, comparing a byte from a register against a value at an offset from a pointer, and returns if a match is found.
- `update_pointer_and_branch`: Masks a register, adds it to a base address, stores the result in a global pointer, and branches to another routine if a bit-test on a byte from the new address passes or fails.
- `update_global_pointer`: Stores a pointer in a global variable and sets another global variable to a calculated address based on a byte and a global variable.
- `update_pointer_conditionally`: Checks a byte at an offset, and if certain conditions are met, calls another routine and potentially updates a global pointer.
- `parse_bitstream_instruction`: Reads a value from a pointer, checks a bitmask, and conditionally updates several global variables or buffers based on certain memory contents.
- `update_object_list_and_dispatch_handlers`: Updates a global pointer-based table or list by calculating offsets and then dispatches or executes a sequence of routine calls based on the high bits of a control word.
- `calculate_clamped_offsets`: Calculates several values and stores them in global variables, applying clamping logic to ensure certain values do not exceed a specific limit.
- `update_bitmask_tables`: Performs a nested loop that calculates bitwise-manipulated bitmasks and writes them into a destination table and several offset tables.
- `update_handler_pointer`: Checks a bitmask in a memory location and, if a certain condition is met, modifies a byte and updates a global pointer or increments a global variable.
- `update_state_via_pointer_lookup`: Evaluates a character or byte at a pointer-based address and uses it to index into a table, subsequently updating a global state variable or a secondary global variable based on the retrieved value and certain conditional checks.
- `set_global_from_condition`: Evaluates certain register or memory values and calls another routine to determine a new value for a global variable.
- `update_global_based_on_check`: Calls another routine and updates a global variable based on the returned value.
- `modify_memory_buffer`: Sets a global variable, modifies a byte in a table, and updates a memory location via a pointer.
- `parse_bytecode_sequence_2`: Iterates through a memory buffer, checking for specific byte values and updating a global variable or a pointer, and conditionally calls other routines or clears a global variable.
- `check_table_constants`: Clears a memory location and checks if a byte at an offset matches certain constants, potentially clearing another memory location and setting up registers for a loop.
- `compare_memory_values`: Clears a register and compares a byte from a memory location against a byte in a table, potentially looping or branching based on the result.
- `validate_and_store_byte`: Tests a register and, if not zero, stores a byte from a pointer, checks if that byte matches certain constants, and potentially updates a global or calls another routine.
- `check_byte_and_branch`: Compares a byte at a pointer-based address to a constant and branches if not equal, otherwise loads a constant into a register and calls another routine.
- `parse_bytecode_sequence`: Reads a sequence of bytes to perform conditional jumps or calls other routines and updates certain memory locations.
- `update_pointer_and_branch_3`: Moves a longword from a global address to a pointer, loads a constant into a register, copies a byte from a pointer, and conditionally clears a register or calls other routines.
- `update_pointer_and_branch_4`: Loads a longword from a global address into a pointer, copies a byte from a pointer, and branches to another routine.
- `parse_instruction_bitstream`: Parses a bitstream or encoded instruction sequence by checking bitmasks and updating a pointer or global state.
- `modify_memory_with_offset`: Clears a bit in a byte at a pointer offset and calculates a new address stored in a register based on a global variable.
- `update_bitmask_table_2`: Iterates through a table to check for a bitmask match, then updates a byte in that table and a byte in another memory area.
- `modify_registers_and_memory`: Loads a pointer from a global variable into a register, clears a byte at a certain address, writes a constant byte to an offset of that pointer, and clears a word at a certain address.
- `traverse_string_with_recursive_calls_3`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `apply_bitmask_to_memory_2`: Performs a series of memory writes and bitwise operations, including clearing certain memory areas and checking bitmasks.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_block_with_loop`: Iterates through a memory block using a pointer, setting the contents of each longword to zero until a certain condition is met.
- `store_values_to_offset_addresses`: Calls another routine multiple times and stores the returned values into a sequence of memory locations using an offset.
- `execute_loop_decrement`: Executes a loop that decrements a register until it reaches a certain value, calling another routine in each iteration.
- `set_address_register_constant`: Sets an address register to a constant value and branches to another location.
- `apply_sprite_mask_and_transform`: Iterates through a table of sprite or object descriptors to apply bitwise masks and bitwise-rotated bitmasks to a series of memory-mapped or buffered sprite mask buffers.
- `update_sprite_mask_and_position`: Calculates a new bitmask and position using fixed-point multiplication and bitwise shifts, then applies these values to a table of sprite or object attributes using bitwise OR and AND operations.
- `clear_and_branch`: Clears a word at an offset from a register and branches to another location.
- `initialize_memory_blocks`: Sets a global variable to zero, fills a memory range with certain values using a loop, and clears a second memory range using a loop before calling other routines.
- `load_constants_and_branch`: Loads a byte and a word constant into registers and branches to another routine.
- `write_bytes_to_memory`: Writes a specific byte and a global variable value to a memory location, followed by several other routine calls.
- `update_global_pointer_2`: Calculates a new address based on a byte from a pointer and a global variable, then stores that address in a global pointer.
- `update_global_on_char`: Checks if a byte written to a memory location matches certain characters and increments or decrements a global variable accordingly.
- `increment_register`: Increments a register by one.
- `search_and_mask_table`: Searches a table for a specific byte pattern and then increments a global byte if certain characters are found while iterating through a pointer table.
- `update_memory_from_buffer`: Updates a global variable, writes a calculated byte and a constant to a memory location, and writes a modified value from a buffer to a memory location.
- `set_memory_with_offset`: Writes a specific calculated byte value to a memory location offset from a pointer and calls other routines.
- `search_for_char_in_list`: Iterates through a list using a pointer, checking if a byte at a certain offset matches a byte found in the list.
- `set_global_word`: Writes a constant word value to a global variable and then calls itself recursively or calls other routines.
- `execute_self_recursive_calls`: This routine performs a sequence of empty recursive calls to itself without modifying any registers or memory.
- `set_global_values`: Checks a global variable against a constant and, if certain conditions are met, resets several memory locations or a global variable to a specific value.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a nested loop structure and offsets.
- `set_address_and_branch_2`: loads a constant address into a pointer register and then performs a short branch to another location.
- `swap_bytes_in_buffer`: Writes a value to a memory location and then performs a byte swap on a word at a specific pointer location.
- `recursively_traverse_string_range`: Checks if a byte at a pointer offset falls within a specific range and, if so, recursively calls itself a fixed number of times while clearing and restoring a global variable.
- `execute_recursive_string_traversal`: Recursively calls a routine based on whether a global variable is non-zero, effectively traversing a string or a linked structure.
- `write_derived_char_and_loop`: Writes a character or a value derived from a global variable to a memory location and writes a subsequent character to the next memory location, then executes a loop that repeats if a specific bit in a global variable is set.
- `initialize_global_state_and_update_audio`: Checks if a global variable is zero to perform a sequence of register-related or pointer-based assignments, including a loop that decrements a register, and calls routines to update a global variable via a table and update audio DMA registers.
- `loop_until_specific_value`: Repeatedly executes a routine and checks a modified value against a constant, continuing the loop until the returned value equals a specific negative constant.
- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `relocate_amigadget_hunks`: Performs memory relocation or clearing by parsing a table of hunk type codes and updating memory offsets.
- `process_amigadget_hunk_relocation`: Parses a memory structure containing relocation or executable type codes to calculate new memory offsets and copy or clear large blocks of data.
- `initialize_graphics_parameters`: Sets a global variable to a constant, calls a routine, and then calls a graphics library routine.
- `empty_routine`: The routine contains an infinite loop and performs no other actions.
- `clear_list_elements`: Iterates through a list and sets any byte equal to a specific value to zero, then resets a global variable to zero.
- `initialize_list_pointers`: Increments a pointer, sets a pointer to a specific address, clears a byte in a table, and calls another routine twice with different pointer/count parameters.
- `replace_list_bytes`: Iterates through a list, and if a certain byte is unable to be found, replaces it with another byte and stores a global variable in the following byte.
- `modify_table_indices`: Performs a sequence of calls to other routines and decrements bytes at specific offsets relative to a global pointer.
- `reverse_list_segment`: Reverses a segment of a list by swapping bytes from a start pointer and an end pointer moving towards each other.
- `read_byte_and_update_audio`: Reads a byte from an offset and executes a routine that updates audio registers and handles pointer-based logic.
- `update_global_variable`: Checks if a global variable is zero and, if so, writes a constant value to that global variable.
- `set_byte_at_address`: Writes a constant byte value to a specific memory address.
- `tiny_clr`: Small routine of 1 instruction that clears d1.
- `update_audio_dma_registers`: Updates audio registers and enables a DMA channel.
- `set_audio_parameters_3`: Tests a byte and, if not equal to zero, writes a constant to an audio volume register and an audio or DMA register.
- `update_audio_state`: Checks if a global variable matches certain values and, if so, calls another routine twice.
- `increment_a6_register_2`: Adds a constant value to the a6 register.
- `process_buffer_with_recursion`: Checks a pointer, modifies a global byte, and performs a recursive call using a calculated or otherwise modified value.
- `call_audio_parameter_update`: Calls a routine that updates a table of audio-related parameters.
- `update_audio_parameter_table`: Updates a table of audio-related parameters, including volume and DMA settings, based on a conditional check of a value.
- `set_audio_parameters_2`: Writes a constant or a value from a table or register into a global variable and updates a hardware register.
- `set_audio_parameters`: Writes constant values to a global variable and a hardware register.
- `initialize_audio_state`: Opens a resource, clears a table of words, and populates a list with a calculated value before executing several indirect calls.
- `clear_audio_state`: Executes an indirect call to a routine that clears certain audio and DMA related global variables.
- `find_maximum_byte`: Iterates through a table of bytes and returns a value based on the maximum byte found in that table.
- `set_cia_register`: Writes a constant value to a global variable and copies a value from a global variable back to itself.
- `clear_four_word_table`: Clears a table of four-word entries by writing zero to each entry.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `clear_audio_and_dma_state`: Clears several audio and DMA related global variables and sets a global variable to a certain constant value.
- `execute_subroutine_recursively`: Calls another routine without performing any other operations or modifying any registers or memory.
- `increment_and_branch_on_limit`: Increments a global word and branches if it does not equal a certain value, otherwise clears it and calls another routine.
- `traverse_and_call_indexed`: Decrements a register and traverses a buffer, calling a routine at an address calculated from a global table.
- `compare_and_update_memory`: Compares two words from a memory location and, if certain conditions are met, updates a memory location with the smaller or larger value.
- `compare_and_select_register`: Compares two words from a pointer-based offset and moves the smaller or larger value into a register based on a conditional branch.
- `execute_conditional_table_logic`: Checks a byte or bitmask and either executes an indirect function or modifies a pointer or a global variable based on a table lookup.
- `set_button_bit`: Sets a specific bit in a global variable.
- `clear_button_bit`: Clears a specific bit in a global variable.
- `search_and_copy_word`: Searches a global list for a matching word and, if found, copies a word from a calculated offset into a destination table.
- `copy_word_with_conditional_offset`: Calculates a word-aligned offset from a register, negates it if a flag register or certain bit is zero, and copies a word from that offset-adjusted source address to a destination address.
- `store_word_to_offset`: Stores a word from a register into a memory location offset by a constant amount from a pointer.
- `store_byte_and_branch`: Stores a byte from a register into a memory location and then branches to another routine.
- `execute_indexed_function`: Calculates an address from a global table and calls the routine located at that address.
- `copy_word_from_table_2`: Calculates an offset, copies a word from a table into a register, and compares it against a value before branching to another part of the routine.
- `find_and_copy_word`: Iterates through a list of words until a match is found, then copies a word from a table using an offset derived from a byte.
- `find_and_copy_word_masked`: Iterates through a list of words until a match is found, then copies a word from a table using a masked byte as an offset.
- `copy_word_from_table`: Copies a word from a table or memory location to another memory location.
- `update_pointer_by_bit_shift`: Clears a register and loads a byte from a stack-relative location, then shifts it right by four bits.
- `decrement_and_store_value`: Checks a byte from a memory location and, if non-zero, subtracts that value from a pointer and stores the updated pointer to another location.
- `update_audio_registers`: Executes a loop that calls another routine, updates a global pointer, and writes certain values from a table into audio hardware registers using DMA enabled writes.
- `update_entity_attribute_and_limit`: Updates a record or pointer-based structure by calculating a new attribute value from a table, applying clamping or offsets, and updating a secondary limit or global variable.
- `update_record_and_limit`: Updates a value in a record, potentially clamps a value, and modifies a global variable or a pointer-based structure.
- `update_dma_limit`: Calculates a new limit based on a byte, writes it to a memory location, and clears a word in a record if a certain value is detected.
- `update_dma_channel_record`: If a pointer or record or flag is non-zero, it copies a longword, a word, and a short from a source record into a destination block and ORs a flag into a global mask.
- `copy_channel_sample_fields`: Copies a longword and two words from a channel record into an output block, and ORs the record's flag word into a running mask.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d1.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:type

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table header with this value and a sequence of incrementing or decrementing values.
- `move_register_and_branch`: Moves the contents of one register to another and then performs a relative branch.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:RUNBACK

- `execute_initial_jump`: Performs a single jump to a routine located at a specific address.
- `run_background_task`: Parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `pop_and_modify_stack`: Pops several longwords from the stack into a register and sets a bit in a byte located at a stack offset.
- `initialize_buffer_and_set_pointers`: Sets a bit in a byte at a stack offset, clears a range of memory, and stores a pointer and a global variable into a table.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler_and_recurse`: Checks if a pointer in memory is non-zero to call a function, then calls itself recursively.
- `parse_command_line_string`: Parses a string to extract individual words and updates global state fields.
- `no_op_register`: Performs a no-operation by moving a register to itself.
- `execute_iterative_function_calls`: Performs a short loop calling another routine ten times and then inspects several global variables.
- `set_register_and_branch_2`: Loads a constant value into a data register and then performs an unconditional branch to a different location.
- `append_string_with_count`: Walks to the end of a destination string and appends bytes from a source string until a null terminator or a remaining count reaches zero, then null-terminates the destination.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies a certain number of bytes from a source to a destination, stopping if a zero byte is encountered, and then fills any remaining bytes in the count with zeros.
- `validate_and_traverse_table`: Checks if an index is within a valid range and if a table entry is non-zero, then recursively calls another routine using a pointer from the table before clearing the entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using an offset derived from a global library base pointer.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dereference_function_pointer`: loads a function pointer from a table or structure and executes it.
- `execute_indirect_library_function`: loads a function pointer from a table located at an offset from a global variable and executes it.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `fetch_library_function`: loads a function pointer from a library-relative offset and executes it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a function pointer from a table using a base address and calls it.
- `call_library_function_3`: Retrieves a function pointer from a table via an offset and executes it.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_function_4`: Performs an indirect call to a library function using a pointer from a global table.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_5`: Executes a library function located at a specific offset from a base address stored in a table.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:ENDCLI

- `find_max_and_prepare_buffer_3`: Traverses a linked list to find the maximum value in a specific field, then writes a header and copies a block of data or a sequence of incrementing values into a destination buffer if a certain condition is met.
- `move_and_branch`: Copies the contents of one register to another and then performs a relative branch.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:intro

- `uncompress_bitstream_data`: Decompresses a bit-encoded or run-length encoded stream of data into a destination buffer.
- `shift_and_store_word`: Loads a word from a memory location, stores it into a pointer, and returns a value that has been right-shifted and bit-set.
- `recursively_shift_and_decrement`: Recursively shifts a register right and decrements a local variable until a certain condition is met.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr Band][m TFS].adf:bootblock

- `execute_library_function`: Calls a routine and then executes a function pointer located at an offset from a library base.
- `initialize_bootblock_environment`: Closes a library and checks if certain global pointers or flags are null before proceeding to initialize a memory-resident structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_and_hardware`: Writes a value to a global variable and its bitwise or version, then enters a loop that checks a bit in a global variable.
- `set_memory_constant`: Writes a constant value to a memory location at an offset from a register.
- `initialize_memory_and_increment`: Loops until a bit in a memory location is cleared, then writes several constants to a memory block and increments a global variable.
- `increment_global_value`: Checks if a byte in a global variable is certain value, then writes a constant to a memory location and increments a value at that location.
- `initialize_memory_structure`: Loops until a bit in a memory location is cleared, then writes several constants and a calculated address or value to a memory block.
- `decompress_bitstream_data`: Decompresses a bit-encoded or run-length encoded stream of data into a destination buffer.
- `clear_memory_range`: Sets all bytes in a memory range from a certain address to a certain address to zero.
- `copy_and_append_data`: Copies a block of data from one location to another and appends three specific values to the end of the destination block.
- `toggle_bits_and_set_data`: Copies a word from a global variable, toggles certain bits in a byte, and then writes several constants and a global variable value to a memory block.
- `parse_library_list`: Uses library calls to open a library, iterates through a list of structures to update global pointers or values based on certain identifiers, and then copies a block of memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_via_pointer`: The routine performs a sequence of twenty instructions that uses a register-relative or pointer-based access to modify a value in memory.
