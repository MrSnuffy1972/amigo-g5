# Game 2436 (kult - the temple of flying saucers (1989)(exxos)(m3)[cr bm]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02436). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903696 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 555 of 555 named (0 author symbols), 555 of 555 described (44 by rule, 507 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4540 of 4541 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 80682 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10577 of 10577 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 584263 |
| code | 40466 |
| data | 262850 |
| padding | 10997 |
| unused | 5120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:RUN | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:KULT | 2 | 9336 | 502 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:RUNBACK | 3 | 740 | 42 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:Black | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:ENDCLI | 2 | 117 | 2 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 139 | 3 |
| Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:PUZZL.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 00C0) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr bm]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:PUZZL.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- kult - the temple of flying saucers (1989)(exxos)(m3)[cr bm]/Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf: blocks 22-43 (bytes 11264-22528) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- 1 never-loaded region(s), 80682 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system_and_load_assets` (KULT): Initializes hardware registers, sets up global pointers, and manages the loading of external data files. Kind: disk hardware/track check. Pass path: The loop involving u1__sub_0_17B4 and the subsequent check on uVar4. If the character check (likely looking for specific disk signatures or magic numbers in a sector) fails, it enters a loop that eventually hits the `code_r0x01000b08` label.
- `delay_and_modify_hardware` (KULT): Executes several software delay loops and then modifies bits in a value derived from a register. Kind: disk hardware timing/sync check. Pass path: an address.
- `modify_hardware_and_scan` (KULT): Modifies a bit in a value based on a calculated bit position and iterates through a string until a specific byte is found. Kind: disk hardware/sync check. Pass path: the loop continues/the routine completes normally.
- `modify_bits_and_scan_string` (KULT): Performs bitwise operations on a global variable and then iterates through a string until a specific byte is encountered. Kind: disk hardware/track validation check. Pass path: the loop continues or the routine returns normally if the character match fails, but the specific bit manipulation of hardware registers suggests a check for specific disk controller states or track lengths.

## Functions


### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:RUN

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a long value from one register to another and then performs a short relative branch.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:KULT

- `initialize_system_and_load_assets`: Initializes hardware registers, sets up global pointers, and manages the loading of external data files.
- `update_audio_state_and_global_logic`: Increments a global variable and replaces it with an offset-based value from a table, then executes a sequence of conditional checks that update a global variable and clear or set other global variables or pointers.
- `check_global_self_assignment`: Compares a global variable against itself and, if they are not equal, calls another routine and assigns the value back to the global variable.
- `execute_table_commands`: Checks a global value and, if non-zero, iterates through a table to call functions based on values found in a memory buffer.
- `move_stack_pointer`: Moves the value from the stack pointer register into a memory location.
- `dispatch_conditional_command`: Checks a global variable to determine whether to execute a function from a table, perform recursive calls, or update several global variables based on specific bit patterns.
- `update_global_values_conditionally`: The routine calls other routines and then conditionally updates several global variables or modifies a global variable based on a threshold check.
- `store_pointer_and_decrement`: Stores a 16-bit value from a pointer-based memory location into a register and then calls a routine that decrements a register.
- `increment_and_load`: Increments a register by two and loads a byte from a memory location relative to that register into a register.
- `recurse_and_return_registers`: This routine calls another routine and returns the contents of two registers without modifying any memory or registers other than those returned or used during the recursive call.
- `process_char_and_recurse`: Updates a global variable and a memory location with a character derived from an input, then calls another routine recursively if a terminator is not reached.
- `process_offset_char_and_recurse`: Updates a global variable and a memory location with a character derived from an input offset, then calls another routine recursively if a terminator is not reached.
- `traverse_music_string`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine.
- `parse_dialogue_data_structures`: Iterates through a data stream until a specific delimiter is encountered.
- `parse_string_data_structures`: Iterates through a sequence of data blocks until a specific terminator character is encountered.
- `clear_memory_range`: Zeros out a range of memory starting from a global variable pointer back to itself.
- `traverse_string_with_recursive_calls`: Iterates through a string until a terminator character is found, passing a pointer or a value from an offset or a constant to another routine recursively.
- `negate_and_load`: Negates a word in a register and loads a word from a memory location into another register.
- `check_file_info_and_lock`: Calls two library functions to retrieve information and lock a file, then performs comparisons on data from a memory location.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `execute_conditional_dos_calls`: Calls a routine and, if the result is non-zero, calls two other routines and returns a maximum unsigned 16-bit value.
- `initialize_and_call_dos`: Stores three long values into memory, calls a routine, and then conditionally calls two other routines based on the result.
- `load_and_branch`: Loads a constant value into a register and then performs a short relative branch.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `apply_bitmask_to_memory_range`: Increments a global variable and then iterates through a list of offsets to apply a bitmask to a range of memory via bitwise operations and bit-rotated values.
- `traverse_linked_list_and_apply_mask`: Iterates through a linked list of memory offsets, checking bit patterns and updating a global variable or terminating if a specific end-of-list or control-bit pattern is encountered.
- `scan_for_pattern_and_decrement`: Iterates through a list of values, comparing them against a global value and decrementing a counter if certain conditions or patterns are met.
- `scan_variable_for_pattern`: Iterates through a memory range or a global buffer, checking if certain bits of a byte match a specific pattern and returning a calculated or incremented counter.
- `find_maximum_pattern_match`: Iterates through a memory range or buffer using a pattern-searching routine and tracks the highest-value match found.
- `calculate_pattern_distance`: Calculates the unsigned distance between a global variable and a value returned by a pattern-searching routine, updating a global variable with the result.
- `check_and_initialize_globals`: If a global variable does not match a specific constant, it assigns several values to global variables and calls other routines.
- `check_global_value`: Compares a global variable against a constant and calls another routine if the condition is not met.
- `set_global_constants`: The routine assigns specific constant values to certain global variables and stores a pointer to a global variable before calling other routines.
- `process_bit_mask_loop`: Iteratively shifts a value left and updates a mask or bit-manipulation state using a decrementing counter or loop control variable.
- `accumulate_and_adjust_value`: Iterates through a loop to accumulate a sum from subroutine returns, then conditionally modifies a global value and executes further subroutine calls.
- `decrement_and_call_conditional`: Calls a subroutine, tests a register, and decrements that register if it is not zero.
- `decrement_and_call`: Calls another routine and decrements a value in a register.
- `update_pointer_and_call`: Increments a register, clears a word, loads a byte from memory into that word, loads a constant into another register, calls another routine, and then moves a long value from one memory location to another.
- `parse_and_dispatch_command_data`: Extracts parameters from a data structure and uses a series of conditional checks to call another routine with modified arguments.
- `write_table_from_memory`: The routine calls another routine and then writes a certain number of bytes from a memory location into a table at a calculated offset.
- `initialize_and_fill_buffer`: Sets a global pointer and a counter, then executes a loop that writes a null byte to a global variable while iterating through a memory range using a fixed offset.
- `update_global_pointer_with_offset`: Iterates through a list of pointers, adding a pointer's value to itself, and finally stores a new calculated pointer into a global variable.
- `traverse_buffer_with_jumps_2`: Compares a register to a constant, performs a bitwise rotation, and advances a pointer using a byte from memory.
- `traverse_buffer_with_jumps`: Updates a global variable with a value and then traverses a buffer, advancing a pointer by a value read from the buffer, until a certain condition is met.
- `tiny_lsl`: Small routine of 1 instruction that shifts d0 left by 2.
- `load_pointer_to_register`: Loads a long value from a memory location into a register.
- `calculate_scaled_index`: Masks bits from a register, shifts the result, and combines it with a scaled value from another register into a single 32-bit return value.
- `load_address_and_branch`: Loads a constant into a register and then performs a short relative branch.
- `prepare_pointer_for_call`: Clears a register, loads a long value from a global variable into another register, and calls a routine.
- `pop_bytes_to_registers`: Calls another routine, subtracts a register from another, and then loads two consecutive bytes from a pointer into two different registers.
- `store_shifted_word`: Clears a register, moves a word into it, shifts it left by one bit, and stores the result in a global variable.
- `load_global_to_register`: Loads a long value from a global variable into a register.
- `tiny_subq_subq`: Small routine of 2 instructions that subtracts 1 from d2, then subtracts 1 from d3.
- `apply_bitmask_to_buffer_complex`: A nested loop routine that performs bitwise rotation and logical masking, updating a destination buffer and a secondary buffer offset by a large amount, while adjusting a pointer or counter.
- `update_global_pointer_with_offset_2`: Loads a pointer from a global variable, calls a routine, reads two bytes from the pointer, calls another routine, and stores a shifted version of one byte into a global variable.
- `permute_bit_patterns`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value while iterating through nested loops.
- `permute_bits_in_word`: Performs bitwise masking, shifting, and logical ORing to rearrange specific bits within a 16-bit value.
- `decrement_and_loop_counter`: Decrements a register or memory-based counter value and calls itself or another routine until the counter reaches a terminal value.
- `decrement_and_loop`: Decrements a 16-bit signed integer and repeats a routine call until the value reaches negative one.
- `modify_global_value`: Loads a long value from a global variable, shifts the lower word left by one bit, and subtracts two from the result.
- `apply_rotated_bitmask_2`: Uses a loop to call a routine that performs bitwise operations on a table or buffer using values controlled by a global variable.
- `apply_bitmask_loop`: Performs a loop that executes a routine via a pointer-offset while updating a global variable using a bitwise mask.
- `iterate_function_pointer_table`: Iterates through a table of function pointers, executing each one in sequence.
- `execute_indirect_loop`: Updates several global variables and executes a loop that calls a function pointer retrieved from a table.
- `apply_mask_to_buffer`: Performs a loop that executes a routine via a pointer-offset and updates a global index using a bitwise mask.
- `transform_pointer_value`: Loads a longword from a memory location into a register, shifts it left by one bit, and then subtracts two from the result.
- `initialize_and_loop`: Loads a long value from a memory location and performs a loop that executes a routine via a pointer-offset while updating a global variable using a bitwise mask.
- `execute_masked_index_loop`: Executes a loop that calls a routine via a pointer-offset and updates a global variable using a bitwise mask.
- `iterate_pointer_table_with_wrap`: Iterates through a table of function pointers using an offset from a base address.
- `loop_with_index_wrap`: Executes a loop that calls a routine via a pointer-offset and decrements a global variable, resetting it to a fixed value if it reaches zero.
- `decrement_and_loop_with_masking`: Decrements a local variable and a global variable, resetting the global variable to a certain value if it reaches zero, while calling a routine that applies a bit-rotated mask to a buffer and an infinite loop routine.
- `nested_decrement_loop`: Performs a nested loop structure that decrements a local variable and a global variable while calling other routines.
- `execute_nested_loop_sequence`: Performs a nested loop structure that repeatedly calls a specific routine a variable number of times based on decremented register and global values.
- `decrement_and_loop_with_mask_params`: Decrements a local variable and a global variable while calling routines that merge or shift data and calculate mask parameters.
- `execute_conditional_recursive_calls`: Performs a sequence of recursive calls to itself, with the number of calls depending on whether a global variable is zero.
- `execute_conditional_recursion`: Performs a series of recursive calls to itself based on the value of a global variable.
- `branch_and_call_subroutine`: Tests a byte at a memory location and branches if non-zero, otherwise calls a subroutine, performs a left shift on a register, and manages a stack frame before returning.
- `modify_and_call_subroutine`: Tests a byte at a memory location and branches if non-zero, otherwise calls a subroutine, performs arithmetic and bit shifts on a register, and manages a stack frame before returning.
- `execute_conditional_nested_calls`: Checks if a global variable is zero and executes a specific number of subroutine calls based on that condition.
- `execute_extended_conditional_calls`: Checks if a global variable is zero and executes a larger number of subroutine calls if the condition is met.
- `execute_variable_conditional_calls`: Checks if a global variable is zero and executes a different number of subroutine calls compared to other routines.
- `process_byte_and_call_subroutine`: Tests a byte at a memory location, clears a register, loads a byte from a stack offset, and calls two different subroutines with a modified register.
- `update_table_with_values`: Searches a table for a zero value and then writes several register or local values into a table at a calculated offset.
- `load_short_to_register`: Loads the value two into a register and performs a relative branch.
- `set_table_entry_zero`: Calculates an offset based on an input value and sets a value in a table to zero.
- `loop_and_clear_table_entry`: Compares a value at a pointer against a register and uses a loop to set a table entry to zero based on the result.
- `decrement_and_loop_with_mask_params_2`: Decrements a local variable and calls a routine that calculates mask parameters and merges or shifts data.
- `execute_repeated_calls`: Executes a routine a specific number of times based on a value in a register.
- `tiny_addq`: Small routine of 1 instruction that adds 1 to a6.
- `copy_table_blocks`: Copies large blocks of data from one memory area to another using a loop.
- `swap_table_elements`: Swaps or mirrors certain blocks of data between two memory areas using a loop.
- `clear_global_index`: Sets a global variable to zero and calls other routines.
- `execute_empty_calls`: This routine is empty, calling other routines without further logic.
- `load_byte_from_offset`: Increments a pointer, clears a word register, loads a byte from the new pointer address into that register, and branches to another location.
- `process_input_loop`: Checks a global variable and enters a loop that calls a routine to process button input.
- `rotate_and_mask_byte`: Performs bitwise operations and returns a byte masked with a specific value.
- `rotate_bits_and_invert`: Performs a bitwise rotation and inversion on a value from a global variable and returns it.
- `poll_input_bit_state`: Repeatedly calls another routine while masking a specific bit within a global variable.
- `iteratively_mask_and_delay_input`: Repeatedly calls another routine while masking a bit in a global variable, continuing as long as a specific bit remains set, then resets the global variable and executes a final routine call.
- `check_and_call_subroutines`: Compares a long value at a memory location to a constant, calls a subroutine, sets several registers to specific values, tests a byte at a pointer, and conditionally calls another subroutine.
- `loop_and_clear_table_entry_2`: Sets a global variable to zero and enters a loop that repeatedly calls other routines based on a boolean condition.
- `conditional_loop_calls`: Enters a loop that calls a routine and then conditionally calls another routine based on a boolean result.
- `execute_repeatedly_until_false`: Calls a routine repeatedly in a loop until that routine returns a false boolean value.
- `copy_data_blocks`: Copies a block of words from a source table to a destination buffer using a loop.
- `merge_and_copy_buffer_segments`: Merges specific bits from a source buffer into a destination buffer at fixed offsets and then copies the remaining buffer contents word by word.
- `copy_and_mask_buffer_segments`: Copies data from a source buffer to a destination buffer using a loop, while applying bitwise masking operations to specific word elements at the beginning and end of the transfer.
- `load_control_parameters`: Copies several words from hardware registers into a table and loads two pointers into registers.
- `merge_multiple_buffer_streams`: Iterates through multiple buffers to perform bitwise OR and AND operations on specific memory offsets.
- `execute_infinite_loop_3`: The routine calls itself twice and then returns.
- `set_source_pointer`: Loads a value from a global variable into a register.
- `load_parameter_block`: Loads several bytes and words from a source pointer into registers and a destination pointer.
- `apply_bitmask_and_rotate_to_buffer`: This routine iterates through a source buffer, applying a bitmask-based bit-rotation or bit-shifting (using a rotating bit-mask pattern) and writes the resulting bit-manipulated or bit-aligned words into a destination buffer.
- `load_address_from_memory`: Loads a long value from a specific memory location into a register.
- `apply_bit_rotation_to_buffer`: Iterates through a source buffer and writes bit-rotated or bit-shifted versions of specific bit-fields or words into a destination buffer.
- `permute_bits_in_buffer`: Performs bitwise masking and bit-rotation-like operations on a buffer using a calculated bitmask.
- `calculate_mask_parameters`: Calculates bitmask values and updates a global variable used for masking or bit-width calculation.
- `initialize_and_rotate_buffer`: Zeros out a buffer range and then applies a bit-rotated mask to a source buffer, writing the results to a destination buffer.
- `apply_bit_rotation_to_buffer_2`: Iterates through a buffer to apply bitwise masking and bit-rotation-like operations to specific memory offsets.
- `execute_recursive_calls`: Calls itself twice and fills a buffer with zeros while updating global variables used for masking.
- `apply_rotated_bitmask`: Calculates a bit-rotated mask and uses it to bitwise-AND and OR values into a table, or potentially a buffer, using a loop controlled by a global counter.
- `apply_mask_to_offset_buffer`: Applies a bit-rotated mask to a buffer using bitwise operations, where the source buffer is an offset version of the destination buffer.
- `execute_recursive_calls_2`: Calls itself twice before returning.
- `initialize_buffer_and_mask`: Fills a buffer with zeros and updates global variables used for masking or bit-width calculation.
- `search_structured_data`: Iterates through a list of six-byte entries, comparing a byte from a global variable against a masked value, and returns a byte from an entry if a match is found.
- `process_structured_data`: Extracts values from a structured data source, updates several global variables, and performs recursive calls based on a pointer check.
- `decrement_register_loop`: Decrements a value in a register until it reaches a certain limit.
- `populate_buffer_from_indexed_data`: Calculates an offset into a data table, retrieves values, and performs a series of nested loops to write these values and others into a target buffer.
- `call_subroutines_with_address`: Loads a constant into a register and calls two subsequent subroutines.
- `iterate_and_process_buffer`: Iterates through a buffer in three-byte increments, performing conditional recursive calls based on the value of each byte.
- `iterate_and_modify_buffer`: Iterates through a buffer by increments of three, performing conditional logic based on a bit test of each byte.
- `apply_bitmask_to_buffer`: Iterates through a buffer, performing bitwise OR, bitwise AND, and bit-rotation or bit-swapping operations before storing the result back to the buffer.
- `mask_and_store_bytes`: Applies a bitmask or its inverse to bytes in a buffer and stores them back into a buffer located at a calculated offset.
- `return_void`: The routine is empty and performs no operations before returning.
- `initialize_buffer_values`: Sets a global variable to a specific value and copies a sequence of words from one memory location to another, while also clearing a global variable.
- `update_global_pointer_constants`: Sets a global variable to a constant value based on its current state and then updates a pair of global variables with a new pointer value.
- `set_global_variable`: Assigns a constant value to a global variable based on a condition and copies a word from one memory location to another.
- `update_state_machine_and_memory`: Increments a global state variable and executes a sequence of conditional branches based on its value.
- `apply_bitmask_rotation_to_buffers`: Performs complex bitwise rotations and masking operations on multiple buffer segments using nested loops and bit-shifting logic.
- `execute_recursive_calls_3`: Executes a sequence of seven subroutine calls to itself.
- `update_table_and_registers`: Checks a byte at a memory location, clears a word in a register, calls another routine, and updates a global pointer and a word in a table.
- `iterate_table_with_modulo_wrap`: Calculates a product of three values and uses it as a limit for a loop.
- `calculate_loop_limit`: Sets several global variables based on a value, then enters a loop that performs arithmetic and modulo-like wrap-around logic until a calculated limit is reached.
- `update_table_with_rotated_mask`: Uses a bit-rotated mask to perform bitwise AND and OR operations on specific offsets within a table.
- `process_recursive_data_loop`: Checks a global variable and enters a loop that performs recursive calls, updates several global variables, and performs arithmetic on a counter until a condition is met.
- `apply_bitmask_to_buffers`: Calculates a bitmask using a bit-rotation-like operation and applies it to several distant memory offsets via a pointer.
- `execute_placeholder_calls`: This routine is empty and serves only to call other routines without performing any internal logic or data manipulation.
- `write_long_and_branch`: Stores a longword into a specific memory location and performs a short branch.
- `modify_buffer_bits`: Iterates through a buffer and modifies specific bits in a series of words based on a bitmask check.
- `update_bit_patterns_in_memory`: Iterates through a series of memory offsets to set or clear specific bits in words based on a calculated bitmask and a control value.
- `modify_bit_and_memory`: Clears a bit in a register, stores a word at a pointer, and then conditionally sets or clears a bit in another location based on a bit test.
- `update_bit_status_table`: Clears a bit in a register, stores that register's value into a table, and then conditionally sets or clears a bit in a different table entry based on a bit test of a third value.
- `modify_bit_and_branch`: Clears a bit in a register, stores a word to a table, and then conditionally sets a bit in a word based on a bit test.
- `modify_and_loop_bits`: Clears a bit in a register, stores a word to a memory offset, and enters a loop that decrements a register and performs a branch based on a test.
- `modify_bit_patterns`: Iterates through a pointer and modifies specific bits in a destination pointer, a second pointer, a third pointer, and a fourth pointer based on a bitmask.
- `write_long_and_branch_2`: Stores a longword into a memory location and performs a short branch.
- `update_bit_flags`: Iterates through a memory range and updates specific bits in several remote memory locations using a bitmask and a counter-based loop.
- `execute_empty_recursive_calls`: An empty routine that calls itself twice before returning.
- `execute_conditional_recursive_calls_2`: Checks a global variable and, if zero, performs multiple recursive calls and calls another subroutine with specific arguments.
- `execute_recursive_branching_logic`: Performs a series of recursive calls based on the value of a global variable.
- `execute_callback_chain`: This routine executes a sequence of function calls retrieved from a data structure.
- `execute_conditional_subroutines`: Tests a byte at a memory location and, if non-zero, calls two different subroutines and updates two global variables.
- `update_global_pointers`: Tests a byte at a memory location and, if non-zero, calls two different subroutines and updates two global variables with values from registers.
- `execute_recursive_self_calls`: Calls itself twice before returning.
- `copy_memory_block`: Performs a nested loop to copy data from a source pointer to a destination pointer with a fixed stride.
- `get_unaligned_word`: Reads a 16-bit value from a pointer offset by two bytes.
- `update_pointer_and_store_value`: Reads a 16-bit value from a pointer, then updates a global pointer and a global variable with a new address.
- `advance_pointer_with_backup`: Stores the value at a global pointer into another global variable before incrementing that pointer by two bytes.
- `store_constant_to_address`: Stores a 32-bit constant into a specific memory location.
- `set_global_bounds`: Sets a global variable to a specific value and another global variable to a constant integer.
- `execute_infinite_loop`: Performs an empty infinite loop.
- `install_interrupt_handler`: Stores the current routine's address into a location pointed to by a global variable, calls the previous handler found at an offset from that global variable, and then sets the global variable to a constant value.
- `check_library_open_status`: Attempts to call a function via a pointer in a global structure multiple times and enters an infinite loop if the returned values are zero.
- `initialize_buffer_pointers`: Retrieves several pointers from a global variable and its offsets, then writes a series of calculated memory addresses into various global variables.
- `execute_task_related_functions`: Calls three different functions via pointers retrieved from a global variable and its offsets, involving task and port management.
- `initialize_screen_structure`: Retrieves a pointer from a global variable and, if non-zero, calculates three offset addresses within that memory block and copies four words from a global variable to a destination.
- `update_buffer_pointers`: Updates certain global variables using values derived from a pointer and a source buffer, while storing a byte and a word from that buffer into a destination pointer.
- `calculate_scaled_buffer_parameters`: Performs complex arithmetic to calculate scaling factors and bounds, storing the results in several global variables before entering a loop that updates them.
- `process_swapped_data_loop`: Swaps the bytes of a value, updates a global variable, and enters a loop that performs arithmetic and conditional updates on global variables.
- `swap_bytes_and_decrement_loop`: Updates a global variable with a swapped-byte value and enters a loop that modifies a global variable via subtraction while checking a condition or calling another routine.
- `transfer_structured_data_table`: Iterates through a table to perform conditional memory copies and updates pointers based on structured offsets.
- `transfer_structured_data`: Copies bytes between memory locations using a structured offset and performs conditional word copies based on a linked list of entries.
- `populate_table_from_source`: Iteratively copies bytes or related data from a source pointer to a table while calculating offsets and checking for zero values.
- `unpack_bitstream_to_buffers`: Extracts variable-width bit sequences from a source data stream and distributes them into four separate destination buffers.
- `bit_stream_unpack_to_buffers`: Extracts bits from a source table using a variable bit-width and distributes them into four separate destination buffers.
- `scatter_data_to_buffers`: Copies words from a source table into four distinct destination buffers using a nested loop structure.
- `unpack_bitstream_to_buffer_2`: Extracts individual bits from a compressed bitstream source and distributes them into four parallel destination buffers.
- `unpack_bitstream_to_buffer`: Reads a bitstream from a table using a pointer and bit-shifting logic to extract individual bits.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `merge_bit_streams_with_rotation`: Merges two data streams into a destination buffer using bitwise logical operations and bit rotation.
- `rotate_and_increment_pointers`: Compares a byte at a pointer to one, rotates a long value, and increments two pointers based on a carry flag.
- `merge_bit_streams`: Performs bitwise logical operations to merge data from two pointers into a destination buffer while rotating a value and incrementing pointers based on a carry flag.
- `decrement_pointers_and_branch`: Decrements two longwords in registers by a constant value, adds a word to a third register, and performs a short branch to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_range_bounds`: Calculates and writes new range boundaries into global variables and executes a loop that modifies these bounds or pointers.
- `update_global_counters`: Calculates and stores new values in several global variables and executes a loop that decrements a value while calling another routine.
- `copy_interleaved_data_blocks`: Copies data from a source table to a destination table using a loop that processes multiple offsets simultaneously.
- `clear_registers_and_increment_pointer`: Clears two registers and loads a byte from an offset of a pointer into a register, then increments that pointer by two bytes.
- `set_pointer_and_loop`: Stores a byte into a global variable and executes a loop that increments a pointer until it reaches a previously stored pointer value.
- `scan_and_store_bitstream_data`: Iterates through a memory buffer, checks byte values against a range, and updates several global variables and pointers based on the results.
- `process_bit_patterns`: Manipulates bit patterns by shifting and masking values derived from a source buffer and storing the results.
- `process_bit_patterns_2`: Performs bitwise manipulation and pattern processing through nested routine calls.
- `execute_empty_routine`: This routine is empty and performs no operations before returning.
- `increment_pointer_after_loading_byte`: Loads a byte from an offset of the address pointed to by a register into another register, then increments that pointer by two.
- `update_pointer_metadata`: Calls a routine to populate a pointer table, then executes a loop that decrements a value and modifies a pointer-based structure, specifically decrementing a value at an offset and updating a pointer-based address.
- `load_byte_to_register`: Loads a byte from a location relative to a pointer into a register.
- `modify_pointer_table_values`: Calculates a new value based on a pointer, updates a global variable, and then executes a loop that modifies a value in a table.
- `populate_pointer_table`: Iterates through a list of pointers to calculate and store new pointer values and associated metadata into a table.
- `execute_nested_calls`: Loads a byte from a memory location and executes a series of nested routine calls within a loop.
- `store_global_byte`: Reads a byte from a global variable and stores it back into the same location.
- `fill_data_buffer`: Fills a memory area with specific byte values based on a condition and then clears several global variables.
- `unpack_variable_length_data`: Reads a sequence of data structures where each entry contains a bitmask determining the number of subsequent bytes to copy into a destination buffer.
- `call_subroutine_and_branch`: Executes a word-sized subroutine call and then performs a word-sized unconditional branch to another location.
- `process_bitstream_metadata`: Initializes a global variable and certain pointers, then searches through a table to check for a bitmask match or a specific value.
- `traverse_table_and_clear`: Recursively calls another routine and checks a global variable before setting a global variable to zero.
- `dispatch_on_char`: Performs unconditional calls to another routine if a global variable matches any of several specific character constants.
- `shift_and_store_values`: Loads two words from memory and performs a logical shift right on the first value.
- `check_input_range`: Checks if two input bytes fall within a range defined by a table.
- `initialize_registers`: Loads a long value into a register, sets a word value in another register, and performs a short branch.
- `search_for_valid_index`: Increments a value and checks it against a range, potentially setting a global or updating a pointer if a valid range or value is found.
- `invoke_subroutines`: Executes two calls to another routine.
- `find_matching_entry`: Iterates through a table to find an entry where a byte matches a specific value and updates several registers with the result.
- `search_for_matching_pattern`: Iterates through a memory list to find an entry where a certain masked bit pattern matches a changing global value.
- `update_pointer_and_offset`: Updates a global variable with a value from a table, stores a byte from a table into another global, and calculates a new pointer offset using a value from the table multiplied by six.
- `load_pointer_and_calculate_offset`: Loads a value from a pointer-based offset into a global, updates a global with a byte from a pointer, and calculates a new pointer offset based on a value in memory.
- `update_state_from_value`: Checks a global value against several thresholds and updates a memory location or a global variable based on those comparisons.
- `tiny_move`: Small routine of 1 instruction that copies 150 to d1.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `perform_recursive_calls`: Executes two consecutive calls to another routine before returning.
- `load_calculated_offsets`: Calculates an offset by multiplying an input value by four, then loads two values from that offset and a fixed address into global variables.
- `execute_if_value_changed`: Checks if a global value differs from a previously stored value and calls another routine if a change is detected.
- `process_conditional_logic`: Evaluates a character from a table and updates several global variables and memory locations based on specific character matches and comparisons.
- `store_word_and_branch`: Stores a word into a global variable and branches to another location.
- `store_word_and_branch_2`: Stores a different word into a global variable and branches to another location.
- `update_state_and_recurse`: Sets values in memory, updates a global variable based on a comparison, and performs recursive calls.
- `update_table_values`: Iterates through a table, checking if a byte equals a certain value and replacing it with a different value.
- `execute_conditional_update`: Checks a character at a calculated offset and modifies a table of values and a global variable if certain character conditions are met.
- `update_mouse_pointer_position`: Writes a word value to a specific memory location and performs a short relative branch to another instruction.
- `process_conditional_data`: Calls a routine and, if a global variable is non-zero, modifies memory locations and performs conditional logic based on values at specific offsets.
- `decrement_and_branch`: Decrements a long value at a pointer and branches to another location.
- `dispatch_command_table`: Evaluates a value to determine whether to execute a function from a table, perform complex arithmetic updates to global variables, or set a pointer.
- `execute_command_bytecode`: Interprets a stream of command bytes by branching into different execution modes based on high-order bits.
- `initialize_global_state`: Sets a global variable to zero or one and checks a character at an offset from a pointer-based address.
- `dispatch_virtual_machine_instructions`: Interprets a sequence of commands by reading bytes from a data stream and executing corresponding subroutines from a jump table.
- `dispatch_conditional_command_2`: Evaluates a value from a table to determine whether to execute a function, perform recursive calls, or update several global variables.
- `process_byte_difference`: Compares a byte from a pointer with a register, performs bitwise masking and subtraction on a value from a pointer, and branches based on the result.
- `invoke_instruction_stream`: Calls a routine that processes a sequence of instructions from a memory buffer and performs no other operations.
- `set_global_constant`: Calls another routine twice and then sets a global variable to a constant value.
- `dispatch_command_table_entry`: Evaluates a global variable to determine a command or offset, then either executes a function from a table, performs a series of nested routine calls, or iterates through a sequence of function pointers based on byte values found in memory.
- `dispatch_conditional_command_alt`: Processes a value from a global variable to execute a function from a table or perform a series of recursive calls and updates.
- `check_status_and_set_value`: Compares values at two different memory locations and writes a specific word to a global variable if certain conditions are met.
- `execute_bytecode_interpreter`: Interprets a sequence of commands by reading bytes from a memory location and branching to specific subroutines based on high-nibble opcode values.
- `dispatch_conditional_command_alt2`: Uses a global variable to either execute a function via a table or trigger a complex sequence of recursive calls.
- `dispatch_command_stream`: Interprets a stream of command data by evaluating high-order bits to determine the operation type.
- `execute_command_dispatch`: Evaluates a command value to either execute a function from a table, perform a sequence of recursive calls, or iterate through a list of function pointers stored in memory.
- `assign_constant_values`: Stores two constant integer values into a global variable.
- `execute_recursive_sequence`: Sets a specific value to a global variable and initiates a sequence of recursive calls.
- `clear_and_mask_table_elements`: Iterates through a table of 40-byte-aligned entries, clearing the high bit of the second word in each entry.
- `dispatch_command_and_update_state`: Processes a command stream by iterating through a data structure, performing memory swaps, and executing function pointers from a table based on specific byte matches.
- `modify_and_dispatch`: Sets a bit in a byte at an offset from a pointer, clears several registers, loads two bytes from a pointer into registers, and calls two different subroutines before branching.
- `dispatch_command_buffer`: Iterates through a data buffer to execute commands based on marker bytes.
- `dispatch_conditional_command_alt3`: Checks a global variable and either executes a function from a table or performs recursive operations.
- `execute_bytecode_interpreter_2`: Interprets a stream of command bytes by fetching values from a table or memory and executing corresponding functions.
- `execute_command_bytecode_2`: Evaluates a value from a global variable to determine whether to execute a function from a table, perform recursive calls, or access data via an offset.
- `dispatch_command_table_entry_2`: Decodes a command value to either perform a direct function call from a table, execute a specific sequence of subroutines with calculated parameters, or iterate through a list of function pointers stored in a buffer until a null or out-of-range byte is encountered.
- `dispatch_conditional_command_alt5`: Uses a value from a register to execute a function from a table or perform a series of recursive calls.
- `iterate_and_store_data`: Iterates through a table, checks a bitmask against a value, and stores certain bytes and pointers into a global location.
- `call_empty_routine`: Calls another routine and then returns.
- `execute_command_sequence_2`: Interprets a command value by either performing direct memory operations, executing a function from a table, or recursively calling itself to process a sequence of instructions.
- `execute_command_loop`: Sets a global variable and then iterates through a list of function pointers to execute them.
- `execute_conditional_branches`: Calls two subroutines, performs a conditional branch, writes a word to a specific memory location, and then branches to another routine.
- `update_global_parameters`: Calculates and stores several values into global variables based on arithmetic operations and conditional checks.
- `execute_command_sequence`: Iterates through a sequence of bytes, using each byte to index into a table of function pointers that are then called.
- `execute_conditional_branches_with_offset`: Calls three subroutines, performs a conditional branch, calls a fourth subroutine, performs another conditional branch, writes a word to a specific memory location, and then branches to another routine.
- `reset_global_parameters`: Sets several global variables to specific constant values and enters a loop that continues while a specific bit in a global variable is set.
- `update_global_state_and_pointer_table`: Updates a global variable using a clamped or masked value and applies a bitmask-driven pointer-based update to a table of global variables.
- `execute_conditional_command_table`: Iterates through a table of command entries, using a control variable to determine whether to execute function pointers from the table, perform recursive calls, or update global state.
- `store_byte_to_address`: Loads a long value into a register, stores a byte into a memory location, and performs a short branch.
- `scatter_data_to_buffers_2`: Iterates through a loop, calling a routine that copies or scatters data into a destination, and then executes a secondary loop.
- `scatter_table_data`: Copies bytes or words from a source table to a non-contiguous destination pattern.
- `clear_and_fetch_bytes`: Clears a long register and two word registers, then loads a byte from an offset of a stack pointer into the long register and fetches two consecutive bytes from the incremented stack pointer into the word registers.
- `dispatch_table_commands`: Iterates through a buffer, checking for specific marker bytes to either extract data or call subroutines via a table of function pointers.
- `calculate_offset_address`: Calculates a new address or offset based on specific bits in a register and stores it in a global variable.
- `fetch_and_branch`: Loads a byte from a pointer into a register, calls another routine, clears a register, and loads another byte from the pointer.
- `clear_and_process_buffer`: Clears a block of memory to zero and then iterates through a buffer to perform bitwise operations and store results.
- `set_hardware_registers`: Stores a word and a byte into specific memory locations and then performs a relative branch.
- `copy_memory_and_branch`: Copies a word from one memory location to another and then performs a relative branch.
- `write_constant_and_branch`: Stores a constant word into a memory location and then performs a relative branch.
- `decrement_and_store_word_2`: Decrements a word retrieved from a global location by one and stores the result into another global location before performing a short branch.
- `process_nested_bitfield_data`: Iterates through a nested structure defined by a pointer, extracting bitfield values to control loop counts and conditional branching.
- `update_global_counter_2`: Modifies a global variable by incrementing, decrementing, or subtracting a value based on bitwise flags in another global variable.
- `clear_global_variable`: Sets a global variable to zero.
- `copy_planar_buffer_blocks`: Performs a block memory transfer of bitplane data by copying words from a source buffer to a destination buffer using nested loops.
- `resolve_indirect_byte`: Determines a byte value by evaluating bit flags in a byte to decide whether to fetch from a direct offset or calculate an address using a table and a pointer.
- `evaluate_conditional_flags`: Performs multiple comparisons between two 16-bit values based on a bitmask and returns based on the results.
- `branch_on_byte_test`: Calls two routines, moves a value from a register to another, and performs a bit test on a byte from a pointer to determine a branch destination.
- `resolve_indirect_byte_2`: Determines a byte value by evaluating bit flags and stores the result at a pointer location.
- `get_unaligned_word_2`: Reads a 16-bit value from a pointer offset by two bytes if a specific condition is met.
- `process_button_input`: Checks certain bits in a register and updates a global variable based on the state of those bits.
- `update_mouse_sprite_position`: Calculates new horizontal and vertical coordinates based on joystick or mouse input and updates a sprite's position registers.
- `increment_and_dereference_global`: Increments a global variable and then replaces its value with the value stored at that new offset in a table.
- `increment_and_load_pointer`: Increments a global variable and returns the value stored at the address or offset pointed to by that incremented value.
- `modify_global_values`: Calls another routine, writes a word to a memory location, calls several other routines, and then writes zero to that same memory location.
- `modify_memory_block`: Checks a condition based on a global variable and a difference between two values, then iterates through a table to clear a specific bit in bytes at six-byte intervals.
- `copy_calculated_words`: Calculates a count based on a multiplication and copies that many words from a source location to a destination location.
- `increment_a6_register`: Adds one to the value in the a6 register.
- `load_byte_from_address`: Clears the d0 register and loads a byte from a specific memory address into it.
- `copy_multiple_words`: Calculates a count based on a multiplication, then uses a loop to copy words from one memory location to another.
- `load_a1_address`: Loads a specific memory address into the a1 register.
- `call_load_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls LoadRGB4 and returns its result to the caller.
- `synchronize_and_adjust_data_table`: Copies a block of data from a global variable to a destination table, then enters a loop that iteratively modifies nibble values within a specific memory range to increment them toward a target state.
- `sync_and_modify_table`: Copies a sequence of words from a calculated offset in a table to another location, then enters a loop to compare and bitwise modify values in a memory block until a match is found.
- `clear_buffer_with_check`: Zeros out a buffer of sixteen words and then iterates through a table to call another routine if a specific bit is set in each entry.
- `rotate_and_store_buffer`: Performs a bit-rotation on a value and writes it to a memory location.
- `compare_byte_range`: Loads a byte from a memory location and branches based on whether its value falls within a specific range.
- `validate_byte_range`: Loads a byte from a global variable and compares it against two constant values.
- `increment_base_pointer`: Increments the value held in the sixth address register by one.
- `execute_recursive_loop`: Executes a routine fifteen times in a loop, calling itself recursively during each iteration.
- `check_pointer_and_recurse`: Checks if the value pointed to by a register is non-zero and calls itself if so.
- `increment_frame_pointer`: Increments the value stored in the sixth address register by one.
- `traverse_and_process_buffer`: Iterates through a fixed-size sequence of memory-aligned words and calls another routine if a non-zero value is encountered.
- `update_bitmask_table`: Iterates through a table to check for a bitmask match, then updates a byte in that table and a byte in another memory area.
- `search_and_update_buffer_2`: Iterates through a table to check for a bitmask match, then updates a byte in the table and a byte in another memory area.
- `clear_address_register`: Sets an address register to zero and branches to another location.
- `search_and_update_buffer`: Iterates through a table to check if a byte matches a masked value, then increments a byte at a calculated destination address and updates a global variable.
- `compare_long_values`: Loads two long values from memory and performs a conditional branch based on their comparison.
- `scan_buffer_for_pattern`: Iterates through a buffer and checks if certain bits in a byte match a specific pattern, then recursively calls another routine and updates a pointer.
- `transfer_stack_bytes`: Clears a register, moves a byte from a stack offset into it, calls another routine, and then moves two more bytes from the stack into other registers.
- `reset_global_variable_2`: Sets a global variable to zero and subsequently overwrites it with values from registers before checking a bitmask in a loop.
- `execute_recursive_calls_4`: Performs two consecutive calls to another routine.
- `initialize_data_registers`: Loads three longwords into specific memory locations and calls another routine before performing two byte writes and a branch.
- `execute_recursive_call`: Performs a recursive call to itself while incrementing a register value.
- `traverse_memory_recursively`: Executes several routine calls, including a recursive call that increments a register and other routines that process bitmasking or memory offsets.
- `modify_byte_and_recursively_call`: Performs a recursive call, then applies a bitmask to a byte located at an offset from a register, followed by several other recursive calls.
- `modify_byte_and_recurse`: Copies a byte from one location to another and clears a bit in a byte at an offset from a pointer.
- `compare_and_modify_memory`: Compares a byte at a pointer to a specific value and, if it does not match, writes a byte to an offset from a constant address.
- `modify_memory_with_offsets`: Reads a byte from a pointer and writes modified or offset values to adjacent memory locations relative to a register-based address.
- `load_address_and_branch_3`: Loads a constant value into a register and performs a short branch.
- `load_address_and_jump`: Loads a specific memory address into a pointer register and performs a short branch to another location.
- `load_pointer_and_branch`: Loads a specific memory address into a pointer register and performs a short unconditional branch to another location.
- `parse_and_execute_script_command`: Parses a command string from a global buffer by normalizing specific characters and looking up values in a table.
- `loop_with_decrementing_counter`: Sets a global variable, calls another routine, assigns values to memory, and enters a nested loop structure that calls various routines and performs empty loops.
- `save_registers_call_and_branch`: Saves longwords of data registers and address registers to the stack, executes a subroutine, and then performs a relative branch to another location.
- `execute_nested_loop`: Executes a nested loop structure that decrements registers, effectively performing a fixed number of empty or secondary routine calls.
- `update_masked_buffer`: Performs a bitwise or-not and and-with operation while copying or updating values in a large buffer using a nested loop structure.
- `copy_buffer_segments`: Copies data from a source buffer to a destination buffer using a nested loop structure.
- `process_bitmask_table`: Iterates through a table to perform bitwise operations on words and updates specific memory offsets using a combination of bitwise AND and OR operations.
- `conditional_loop_on_byte`: Checks if a byte at a pointer is non-zero and then executes a loop a certain number of times, potentially calling another routine once more if the byte was non-zero.
- `loop_fixed_count`: Executes a loop that decrements a register from a fixed starting value down to a certain limit, calling another routine within the loop.
- `decrement_two_counters`: Executes a loop that decrements two different registers simultaneously, calling another routine with a changing or fixed value.
- `clear_global_in_loop`: Executes a loop that decrements a register and sets a global variable to zero during each iteration.
- `execute_infinite_loop_2`: This routine is empty and executes an infinite loop.
- `execute_recursive_loop_2`: Recursively calls another routine while decrementing a local variable until it reaches a certain value.
- `execute_descending_loop_sequence`: Executes another routine a fixed number of times while decrementing a local variable in a loop.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_global_pointer_offset`: Calls another routine and sets a global variable to a value derived from a bit-shifted byte.
- `parse_audio_parameter_buffer`: Iterates through a buffer of three-byte entries to update a global audio parameter table.
- `validate_file_data_sequence`: Performs a series of checks by calling other routines and comparing returned values against specific constants.
- `reset_global_variable`: Sets a global variable to zero and then to a maximum byte value.
- `apply_relative_offset`: Iterates through a memory range and adds a value stored at a global pointer to the routine's own address if that value is non-zero.
- `validate_sequence_of_calls`: Sets a global variable, iterates through a memory range to add a constant to non-zero values, and performs a series of nested checks by calling other routines and comparing their results to specific constants.
- `reset_global_range`: Sets a global variable to a zero value and then to a maximum byte value.
- `search_table_and_set_flags`: Clears a bit in a global variable and iterates through a table to find a matching byte, then sets specific values in a table or clears several bytes in a structure based on the result.
- `set_register_bounds`: Sets a register to a specific constant value and sets another register to an offset relative to a global variable.
- `search_and_modify_bitmask`: Checks a global variable, iterates through a table to find a matching byte, and if found, modifies a byte at a calculated pointer using a bitmask.
- `modify_bitmask_and_pointer`: Sets a register to a constant, copies a byte from a memory location, and modifies a byte at a calculated pointer address using a bitmask.
- `clear_memory_locations`: Sets two specific memory-mapped or global addresses to zero.
- `scan_table_for_match`: Iterates through a table to find a specific marker, then searches a sequence of bytes for a match to set specific values in a table or clear a global variable.
- `set_memory_registers`: Assigns a constant to a global variable and updates another global variable with an offset relative to a third global variable.
- `search_and_update_state`: Iterates through a table of two-byte entries to find a match for a global variable, then updates that variable with the second byte of the matching entry.
- `update_pointer_and_bitmask`: Sets a global variable to a constant, copies a byte between memory locations, and modifies a byte at a calculated offset using a bitwise mask.
- `clear_two_variables`: Sets a global variable and another specific memory location to zero.
- `update_state_variables`: Checks a global variable against several constants and updates multiple memory locations or a global variable based on the result and other conditional checks.
- `dispatch_instruction_opcode`: Interprets a command word by checking its high nibble to determine the execution mode.
- `check_and_clear_byte`: Compares a byte in a global variable to a constant, and if it does not match, writes a word to a global variable and clears the byte.
- `clear_and_mask_buffer`: Clears a byte at a specific memory location and a global variable, then conditionally clears the high bit of a byte at a specific offset.
- `check_and_call_routine`: Tests a byte from a pointer and, if it is not zero, loads a value into a register, calls another routine, and branches.
- `reset_global_and_branch`: Sets a global variable to zero and executes a sequence of other routines based on certain value checks.
- `tiny_move_2`: Small routine of 1 instruction that copies 63 to d3.
- `search_for_byte_in_list`: Iterates through a list of bytes, comparing a byte from a register against a value at an offset from a pointer, and returns if a match is found.
- `update_global_pointer_from_offset`: Calculates an address by adding a masked value to a base address, stores this address in a global variable, and then modifies a byte from that address before calling another routine.
- `update_global_pointer`: Stores a pointer in a global variable and sets another global variable to a calculated address based on a byte and a global variable.
- `process_conditional_calls`: Checks a byte at an offset, potentially executes a routine within a loop, and updates a global pointer if a global variable is zero.
- `parse_bitstream_instruction`: Reads a value from a pointer, checks a bitmask, and conditionally updates several global variables or buffers based on certain memory contents.
- `execute_bytecode_interpreter_3`: Processes a custom instruction stream by interpreting opcodes to perform recursive calls, table lookups, or direct function execution.
- `calculate_clamped_offsets`: Sets several global variables using values from registers and performs arithmetic to ensure certain calculated values remain within specific bounds.
- `apply_bitmask_transformations`: Performs bitwise logical operations and circular shifts on data blocks to transform values in a destination table.
- `update_handler_pointer`: Checks a bitmask in a memory location and, if a certain condition is met, modifies a byte and updates a global pointer or increments a global variable.
- `process_conditional_command_logic`: Evaluates a command byte and associated parameters to select a state or branch of execution.
- `select_value_by_conditions`: Evaluates multiple conditions involving a byte and another value to select one of several possible byte values, which is then stored in a global variable.
- `update_global_based_on_check`: Calls another routine and updates a global variable based on the returned value.
- `modify_memory_buffer`: Sets a global variable, modifies a byte in a table, and updates a memory location via a pointer.
- `process_bytecode_commands`: Iterates through a data structure, performing various checks on bytes and conditionally calling other routines or updating global variables.
- `select_register_constant`: Compares a byte at an offset from a pointer against two constants and loads a specific byte into a register based on the match.
- `compare_bytecode_value`: Compares a byte from a pointer against a value in a table and branches based on the result.
- `validate_and_execute_bytecode`: Tests a register, and if non-zero, stores it at an offset from a pointer, checks if that byte matches specific constants, and conditionally calls another routine.
- `compare_and_branch_bytecode`: Compares a byte at an offset from a pointer to a constant and branches if it does not match, otherwise loads a value into a register, calls another routine, and branches.
- `parse_bytecode_sequence`: Reads a sequence of bytes to perform conditional jumps or calls other routines and updates certain memory locations.
- `dispatch_bytecode_commands`: Moves two longwords from memory, sets an address register, loads a byte from a pointer, checks a global variable, and conditionally calls two different routines or performs a branch.
- `load_and_branch_data`: Loads an address into an address register, moves a byte from a pointer to a memory location, and performs a short branch.
- `parse_bitstream_instruction_2`: Reads a sequence of bits from a memory buffer to identify and execute specific instruction patterns.
- `modify_memory_with_offset`: Clears a bit in a byte at a pointer offset and calculates a new address stored in a register based on a global variable.
- `update_bitmask_table_2`: Iterates through a table to check for a bitmask match against a global variable, then updates a byte in that table and a byte in another memory area.
- `modify_memory_and_registers`: Loads a pointer from a global variable, clears a byte at a specific address, writes a constant byte to an offset of that pointer, and clears a word at another address.
- `traverse_and_compare_data`: Iterates through a list of values in memory, comparing them to a global variable, and performs conditional bitwise operations on a memory byte.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_data_streams`: Initializes a series of memory values and enters a loop that processes data streams using bitwise operations and conditional logic.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_block_with_loop`: Iterates through a memory block using a pointer, setting the contents of each longword to zero until a certain condition is met.
- `store_values_to_offset_addresses`: Calls another routine multiple times and stores the returned values into a sequence of memory locations using an offset.
- `execute_repeated_routine`: Executes another routine one hundred times in a loop.
- `load_address_register`: Loads a constant value into an address register and branches to another location.
- `apply_bitmask_to_planar_buffer`: Iterates through a table of pointers to apply bitwise AND and OR operations to specific memory locations.
- `calculate_and_apply_bitmask`: Calculates a bitmask using division and bitwise shifts, then applies it to specific memory locations via bitwise OR and AND operations.
- `clear_and_branch`: Clears a word at an offset from a register and then performs a short branch.
- `initialize_and_clear_memory`: Sets a global variable, calls another routine, performs three separate loops to fill memory blocks with specific values, and finally calls another routine through a pointer in a loop.
- `load_constants_and_branch`: Loads a byte and a word into two data registers and performs a short branch.
- `write_bytes_to_memory`: Calls another routine, then writes a constant byte and a global variable value to a memory location pointed to by a register.
- `calculate_offset_pointer`: Calculates an address based on a byte from a pointer and an offset, then stores it in a global variable.
- `update_global_counter`: Checks if a byte stored at an offset from a pointer matches specific characters and increments or decrements a global variable based on a secondary byte value.
- `increment_register`: Increments a long value in a register by one.
- `search_and_mask_table`: Searches a table for a specific byte pattern and then increments a global byte if certain characters are found while iterating through a pointer table.
- `process_buffer_values`: Calculates new values for three memory locations based on data extracted from a buffer and then calls another routine multiple times.
- `set_memory_with_offset`: Writes three specific byte values to memory locations at offsets from a pointer and then calls another routine multiple times.
- `search_for_char_in_list`: Iterates through a list using a pointer, checking if a byte at a certain offset matches a byte found in the list.
- `execute_recursive_sequence_2`: Performs a series of recursive calls to itself and assigns a value to a global variable between calls.
- `execute_self_recursive_calls`: This routine performs a sequence of empty recursive calls to itself without modifying registers or memory.
- `update_global_state`: Checks a global variable against multiple thresholds to set a series of memory locations and a specific word value.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a nested loop structure and offsets.
- `load_address_and_branch_2`: Loads a memory address into a register and performs a short branch to another location.
- `swap_bytes_in_buffer`: Writes a value to a memory location and then performs a byte swap on a word at a specific pointer location.
- `parse_bitstream_data`: Iterates through a bitstream by checking byte values against a range and performing recursive calls based on a global variable.
- `process_recursive_data`: Performs recursive calls based on the state of a global variable and updates memory accordingly.
- `update_string_buffer`: Writes a character or a derived value based on a global variable to a memory location and performs a conditional loop based on a bit in a global variable.
- `calculate_offset_value`: Performs bitwise operations on a value to derive a new number and executes a complex sequence of register-based logic and global variable updates.
- `loop_until_specific_value`: Repeatedly executes a routine and checks a modified value against a constant, continuing the loop until the returned value equals a specific negative constant.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer located at an offset from a global pointer.
- `parse_amigados_hunks_2`: Iterates through a memory structure to identify and process AmigaDOS hunk types, such as executable loading or relocation code.
- `parse_amigados_hunks`: Iterates through a memory structure to identify and process AmigaDOS hunk types, such as executable or relocation code.
- `dispatch_subroutines`: Loads an address into a register, calls two subroutines, updates a global word, and then calls two more subroutines before branching.
- `execute_infinite_loop_4`: Performs several subroutine calls and then enters an infinite loop.
- `clear_list_elements`: Iterates through a list and sets any byte equal to a specific value to zero, then resets a global variable to zero.
- `execute_sequential_list_replacements`: Increments a value in a register, then performs three consecutive calls to a routine that iterates through memory lists to replace specific bytes.
- `replace_list_bytes`: Iterates through a list, and if a certain byte is found, replaces it with another byte and stores a global variable in the following byte.
- `modify_table_indices`: Performs a sequence of calls to other routines and decrements bytes at specific offsets relative to a global pointer.
- `reverse_list_segment`: Reverses a segment of a list by swapping bytes from a start pointer and an end pointer moving towards each other.
- `process_byte_and_logic`: Reads a byte from an offset and executes a sequence of conditional checks and global variable assignments.
- `update_global_flag`: Checks a global variable and, if it is zero, writes two different values to it sequentially.
- `write_single_byte`: Writes a constant byte value to a specific memory location.
- `tiny_clr`: Small routine of 1 instruction that clears d1.
- `update_audio_dma_registers`: Updates audio registers and enables a DMA channel.
- `set_audio_volume_and_dma`: Checks a byte at a pointer and, if not zero, writes specific values to the audio volume and DMA control registers.
- `update_audio_state`: Increments a global variable and then replaces its value with the value stored at that new offset in a table, followed by updating audio DMA registers.
- `increment_a6_register_2`: Adds a constant value to the a6 register.
- `process_recursive_buffer`: Checks a global pointer, performs calculations involving division and modulo, and calls itself recursively if the pointer is non-zero.
- `call_audio_parameter_update`: This routine acts as a wrapper that calls a procedure to update a table of audio-related parameters, such as volume and DMA settings, based on a conditional check.
- `update_audio_parameter_table`: Updates a table of audio-related parameters, including volume and DMA settings, based on a conditional check of a value.
- `set_audio_parameters_2`: Writes values to several global variables and hardware registers, including audio period, length, and volume registers.
- `set_audio_parameters`: Writes constant values to several global variables and hardware registers.
- `initialize_audio_state`: Opens a resource, clears a table of words, and populates a list with a calculated value before executing several indirect calls.
- `clear_audio_state`: Executes an indirect call to a routine that clears certain audio and DMA related global variables.
- `find_maximum_byte`: Iterates through a table of bytes and returns a value based on the maximum byte found in that table.
- `set_cia_register`: Writes a constant value to a global variable and copies a value from a global variable back to itself.
- `clear_four_word_table`: Clears a table of four-word entries by writing zero to each entry.
- `reset_audio_and_custom_state`: The routine writes zero to four audio volume registers and a custom register, and clears several global variables used for audio state.
- `clear_audio_and_dma_state`: Clears several audio and DMA related global variables and sets a global variable to a constant value.
- `execute_indirect_routine`: Calls another routine via a pointer retrieved from a data structure.
- `increment_and_call_routine`: Increments a global word value and calls another routine if the value reaches a specific threshold, otherwise clears the value and calls a routine.
- `loop_and_increment_pointer`: Executes a loop that calls another routine while incrementing a pointer and a counter.
- `clamp_value_in_range`: Adds a word to a value at an offset from a pointer, compares it against a limit, and clamps the result if it exceeds that limit.
- `update_and_clamp_value`: Adds a word from a register to a value stored at an offset from a pointer, then compares that updated value with another word at a different offset from the same pointer.
- `process_command_data`: Interprets a byte from a pointer to execute different logic branches, including calling a function via a table, modifying a word, or setting a global bit.
- `set_global_bit`: Sets the second bit of a global variable.
- `clear_global_bit`: Clears the second bit of a global variable.
- `search_and_copy_word`: Searches for a word in a list and stores a word from a calculated offset into a destination table.
- `copy_word_with_conditional_offset`: The routine calculates a signed word offset by multiplying an input value by two and applying a sign change based on a flag.
- `store_word_at_offset`: Stores a word from a register into a memory location at a fixed offset from a pointer.
- `store_byte_and_branch`: Stores a byte from a register into a memory location and then performs a relative branch.
- `execute_indirect_call`: Uses a table and an index to call a function via a pointer stored in memory.
- `fetch_offset_word`: Calculates an index using a bit shift, fetches a word from a table at that index, and compares it to a value from another memory location.
- `fetch_indexed_word`: Searches for a matching word in a table and then stores a word from that table into a destination using a bit-shifted index.
- `fetch_masked_word`: Searches for a matching word in a table and then stores a word from that table into a destination using a masked index.
- `copy_word_from_offset`: Copies a word from a memory location at a fixed offset from one pointer to another location at a different fixed offset.
- `decrement_word_by_nibble`: Extracts a high nibble from a byte, and if it is non-zero, adds that value to a word at a pointer and stores the result in another location.
- `decrement_and_store_word`: If a specific nibble from a byte at an offset is non-zero, it subtracts that value from a word and stores the result in another location.
- `update_audio_registers`: Executes a loop that calls another routine, updates a global pointer, and writes certain values from a table into audio hardware registers using DMA enabled writes.
- `update_entity_attribute_state`: Updates an entity structure by retrieving data from a table based on a type index.
- `update_entity_state`: Calculates values based on offsets and byte patterns, updates a short value, and conditionally clears or copies data to a destination structure.
- `validate_and_clear_field`: Adds an offset to a value, compares it to a threshold, and conditionally stores the result or clears a byte based on a comparison.
- `update_channel_record`: Updates a destination block with data from a source record and updates a global mask using a bitwise OR operation.
- `copy_channel_sample_fields`: Copies a longword and two words from a channel record into the output block, and ORs the record's flag word into a running mask.
- `loop_with_subroutine_call`: Loads a long value into a register, calls a subroutine, and then enters a loop that decrements a counter based on a comparison.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies (a7)+ to d6, then loads 0 into d0, then returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `delay_and_modify_hardware`: Executes several software delay loops and then modifies bits in a value derived from a register.
- `modify_hardware_and_scan`: Modifies a bit in a value based on a calculated bit position and iterates through a string until a specific byte is found.
- `modify_bits_and_scan_string`: Performs bitwise operations on a global variable and then iterates through a string until a specific byte is encountered.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d1.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:RUNBACK

- `call_initialization_routine`: Performs a single subroutine call.
- `run_background_task`: The routine parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d0.
- `pop_and_modify_stack`: Pops several longwords from a stack into a register, sets a bit in a byte at the current stack pointer, and then pops more longwords.
- `initialize_data_structure`: Sets a bit in a byte, clears a block of memory using a loop, and stores a pointer and a global variable into a structure.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler_and_recurse`: Checks if a value at a specific offset is non-zero to call a function pointer, then calls itself recursively.
- `process_dialogue_text`: Parses a string to extract individual words and updates global state fields.
- `copy_register_to_self`: This routine performs a redundant move of the contents of a data register into itself.
- `run_back_loop_handler`: Executes a loop ten times calling another routine and then conditionally calls various other routines based on the contents of a table.
- `set_d0_and_branch`: Loads the value negative thirty-two thousand seven hundred sixty-eight into a data register and performs a short branch.
- `append_string_with_count`: Walks to the end of a destination string and appends bytes from a source, stopping at the source terminator or when a count runs out, then null-terminates the destination.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from a source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `lookup_and_call_function`: Validates an index against a table, checks a bit in a table entry, and calls another routine if the bit is not set before clearing the entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using an offset derived from a global pointer.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested structure within a table and executes it.
- `execute_indirect_function_2`: Retrieves a pointer from a memory location calculated using an offset from a global variable and executes the function at that address.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `execute_indirect_library_function`: Retrieves a pointer from a global variable and uses it to call a function at a specific offset within a library structure.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_offset_negative_90`: Calls a function from a library using a pointer retrieved from a global variable and an offset.
- `invoke_indirect_function`: Fetches a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Fetches a function pointer from a location calculated by adding an offset to a value stored in a global variable, then executes that function.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a pointer from a table located via an offset from a global variable and executes the function at that address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:Black

- `update_buffer_and_copy_data`: Iterates through a linked list to find a maximum value, then expands a buffer by writing a header and filling it with incrementing values.
- `move_and_branch_2`: Moves a long value from one register to another and then performs a short branch.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:ENDCLI

- `traverse_list_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize and fill a data buffer.
- `transfer_register_and_branch`: Moves a longword from one data register to another and performs a short relative branch to another location.

### Kult - The Temple of Flying Saucers (1989)(Exxos)(M3)[cr BM].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reallocate_and_copy_data_structure`: Traverses a linked list to find the maximum value, then reallocates a memory block and performs a series of block memory copies between a source and a destination structure.
- `transfer_register_and_branch_2`: Copies the contents of one data register to another and then performs an unconditional short branch to another routine.
- `dispatch_unnamed_callback`: The routine is a 22-instruction 68000 subroutine whose entry is taken by address through a PC-relative load, and it is called by exactly one unnamed routine while making one call to another unnamed routine.
